//! Construct use-def chain via ssa

use common::data_structure::{
    assoc::AssocExt,
    vec_vec::{VecVec, VecVecConstruction},
};
use rustc_data_structures::sso::SsoHashSet;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, BasicBlockData, Body, Local, Location, Place, Rvalue, TerminatorKind,
    },
    ty::TyCtxt,
};
use smallvec::SmallVec;

use crate::ssa::{
    constraint::infer::{Pure, Renamer},
    consume::{Consume, ConsumeChain, Definitions, RichLocation},
    dom::compute_dominance_frontier,
    join_points::{JoinPoints, PhiNode},
    state::{SSAIdx, SSAState},
};

pub struct DefUseChain(ConsumeChain, JoinPoints<PhiNode>);

impl DefUseChain {
    pub fn uses(&self, location: Location) -> impl Iterator<Item = Local> + '_ {
        self.0
            .of_location(location)
            .iter()
            .filter_map(|(local, consume)| consume.is_use().then_some(*local))
    }

    pub fn def_loc(&self, local: Local, location: Location) -> RichLocation {
        let Some(consume) = self.0.of_location(location).get_by_key(&local) else { panic!("{:?} @ {:?}", local, location) };
        assert!(consume.is_use());

        let ssa_idx = consume.r#use;
        self.0.locs[local][ssa_idx]
    }

    pub fn phi_def_locs(
        &self,
        local: Local,
        block: BasicBlock,
    ) -> impl Iterator<Item = RichLocation> + '_ {
        let phi_node = &self.1[block].data.get_by_key(&local).unwrap();

        phi_node
            .rhs
            .iter()
            .copied()
            .filter(|&ssa_idx| ssa_idx != phi_node.lhs)
            .map(move |ssa_idx| self.0.locs[local][ssa_idx])
    }
}

pub fn def_use_chain<'tcx>(body: &Body<'tcx>, tcx: TyCtxt<'tcx>) -> DefUseChain {
    let definitions = definitions(body, tcx);
    let dominance_frontier = compute_dominance_frontier(body);
    let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
    let mut rn = Renamer::new(body, ssa_state, tcx);

    rn.go::<Pure>(());

    DefUseChain(rn.state.consume_chain, rn.state.join_points)
}

fn definitions<'tcx>(body: &Body<'tcx>, tcx: TyCtxt<'tcx>) -> Definitions {
    let mut def_sites = IndexVec::from_elem(
        BitSet::new_empty(body.basic_blocks.len()),
        &body.local_decls,
    );

    let mut call_arg_temps: SsoHashSet<Local> = SsoHashSet::default();
    for bb_data in body.basic_blocks.iter() {
        let Some(terminator) = &bb_data.terminator else { continue; };
        if let TerminatorKind::Call { args, .. } = &terminator.kind {
            call_arg_temps.extend(
                args.iter()
                    .filter_map(|arg| arg.place().and_then(|arg| arg.as_local())),
            )
        }
    }

    let mut consumes = VecVec::with_indices_capacity(body.basic_blocks.len());

    #[allow(unused)]
    struct Vis<'me, 'tcx> {
        // call_arg_temps: &'me SsoHashSet<Local>,
        def_sites: &'me mut IndexVec<Local, BitSet<BasicBlock>>,
        consumes: &'me mut VecVecConstruction<SmallVec<[(Local, Consume<SSAIdx>); 2]>>,
        consumes_in_cur_stmt: SmallVec<[(Local, Consume<SSAIdx>); 2]>,
        body: &'me Body<'tcx>,
        tcx: TyCtxt<'tcx>,
    }

    impl<'me, 'tcx> Visitor<'tcx> for Vis<'me, 'tcx> {
        fn visit_basic_block_data(
            &mut self,
            block: BasicBlock,
            data: &rustc_middle::mir::BasicBlockData<'tcx>,
        ) {
            // println!("visiting {:?}", block);

            let BasicBlockData {
                statements,
                terminator,
                is_cleanup: _,
            } = data;

            let mut index = 0;
            for statement in statements {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_statement(statement, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.consumes_in_cur_stmt);
                self.consumes.push_inner(defs_in_cur_stmt);
                index += 1;
            }

            if let Some(terminator) = terminator {
                let location = Location {
                    block,
                    statement_index: index,
                };
                self.visit_terminator(terminator, location);
                let defs_in_cur_stmt = std::mem::take(&mut self.consumes_in_cur_stmt);
                self.consumes.push_inner(defs_in_cur_stmt);
            }
            self.consumes.push();
        }

        // for return terminator
        fn visit_local(&mut self, local: Local, context: PlaceContext, location: Location) {
            match DefUse::for_place(Place::from(local), context) {
                Some(DefUse::Def) => {
                    let consume = Consume::new();
                    self.def_sites[local].insert(location.block);
                    self.consumes_in_cur_stmt.push((local, consume));
                }
                Some(DefUse::Use) => {
                    let consume = Consume::pure_use();
                    self.def_sites[local].insert(location.block);
                    self.consumes_in_cur_stmt.push((local, consume));
                }
                None => {}
            }
        }

        fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
            match DefUse::for_place(*place, context) {
                Some(DefUse::Def) => {
                    let consume = Consume::new();
                    self.def_sites[place.local].insert(location.block);
                    self.consumes_in_cur_stmt.push((place.local, consume));
                }
                Some(DefUse::Use) => {
                    let consume = Consume::pure_use();
                    self.def_sites[place.local].insert(location.block);
                    self.consumes_in_cur_stmt.push((place.local, consume));
                }
                None => {}
            }

            // call super_projection so that index operators are visited
            self.super_projection(place.as_ref(), context, location);
        }

        fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
            if let Rvalue::BinaryOp(_, box (operand1, operand2)) = rvalue {
                if let Some((lhs, operand1)) = place
                    .as_local()
                    .zip(operand1.place().and_then(|place| place.as_local()))
                {
                    if lhs == operand1 {
                        // special casing statements like _1 = BitAnd(_1, _3)
                        // in this case, we do not generate usage for the right _1
                        self.visit_place(
                            place,
                            PlaceContext::MutatingUse(MutatingUseContext::Store),
                            location,
                        );
                        self.visit_operand(operand2, location);
                        return;
                    }
                }
            }
            self.super_assign(place, rvalue, location);
        }
    }

    Vis {
        def_sites: &mut def_sites,
        consumes: &mut consumes,
        consumes_in_cur_stmt: SmallVec::default(),
        tcx,
        body,
    }
    .visit_body(body);

    let locals_with_defs = BitSet::new_filled(body.local_decls.len());

    let consumes = consumes.done();

    Definitions {
        consumes,
        def_sites,
        locals_with_defs,
        call_arg_temps,
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum DefUse {
    Def,
    Use,
}

impl DefUse {
    fn for_place<'tcx>(place: Place<'tcx>, context: PlaceContext) -> Option<DefUse> {
        match context {
            PlaceContext::NonUse(_) => None,

            PlaceContext::MutatingUse(
                MutatingUseContext::Call
                | MutatingUseContext::Yield
                | MutatingUseContext::AsmOutput
                | MutatingUseContext::Store
                | MutatingUseContext::Deinit,
            ) => {
                if place.is_indirect() {
                    // Treat derefs as a use of the base local. `*p = 4` is not a def of `p` but a
                    // use.
                    Some(DefUse::Use)
                } else if place.projection.is_empty() {
                    Some(DefUse::Def)
                } else {
                    None
                }
            }

            // Setting the discriminant is not a use because it does no reading, but it is also not
            // a def because it does not overwrite the whole place
            PlaceContext::MutatingUse(MutatingUseContext::SetDiscriminant) => {
                place.is_indirect().then_some(DefUse::Use)
            }

            // All other contexts are uses...
            PlaceContext::MutatingUse(
                MutatingUseContext::AddressOf
                | MutatingUseContext::Borrow
                | MutatingUseContext::Drop
                | MutatingUseContext::Retag,
            )
            | PlaceContext::NonMutatingUse(
                NonMutatingUseContext::AddressOf
                | NonMutatingUseContext::Copy
                | NonMutatingUseContext::Inspect
                | NonMutatingUseContext::Move
                | NonMutatingUseContext::ShallowBorrow
                | NonMutatingUseContext::SharedBorrow
                | NonMutatingUseContext::UniqueBorrow,
            ) => Some(DefUse::Use),

            PlaceContext::MutatingUse(MutatingUseContext::Projection)
            | PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection) => {
                unreachable!("A projection could be a def or a use and must be handled separately")
            }
        }
    }
}

pub fn show_def_use_chain(body: &Body, def_use_chain: &DefUseChain) {
    println!("@{:?}", body.source.def_id());
    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        println!("{:?}:", bb);
        let mut statement_index = 0;
        for statement in bb_data.statements.iter() {
            println!("  {:?}", statement);

            let location = Location {
                block: bb,
                statement_index,
            };
            let uses = def_use_chain
                .uses(location)
                .map(|local| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");

            statement_index += 1;
        }
        if let Some(terminator) = &bb_data.terminator {
            println!("  {:?}", terminator.kind);
            let location = Location {
                block: bb,
                statement_index,
            };
            let uses = def_use_chain
                .uses(location)
                .map(|local| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");
        }
    }
}
