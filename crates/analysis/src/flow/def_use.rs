use rustc_index::{bit_set::BitSet, IndexVec};
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        BasicBlock, BasicBlockData, Body, Local, Location, Place, Rvalue,
    },
    ty::TyCtxt,
};
use smallvec::SmallVec;
use utils::data_structure::{
    assoc::AssocExt,
    vec_vec::{VecVec, VecVecBuilder},
};

use super::{
    dom::compute_dominance_frontier,
    infer::Engine,
    join_points::{JoinPoints, PhiNode},
    state::SSAState,
    vanilla::Vanilla,
    LocationMap, RichLocation, SSAIdx,
};

#[derive(Clone, Debug)]
pub struct Update<T> {
    pub r#use: T,
    pub def: T,
}

impl Update<SSAIdx> {
    pub fn new() -> Self {
        Update {
            r#use: SSAIdx::MAX,
            def: SSAIdx::MAX,
        }
    }
}

#[derive(Clone, Debug)]
pub enum UseKind<T> {
    Use(T),
    Def(Update<T>),
    /// Peek the definition of `T`, and the definition is guaranteed to be
    /// located within the same basic block
    LocalPeek(T),
}
pub use UseKind::*;

#[cfg(not(debug_assertions))]
const _: () = assert!(8 == std::mem::size_of::<UseKind<SSAIdx>>());

pub struct DefUseChain {
    /// `Location -> Local -> UseKind<SSAIdx>`
    pub uses: LocationMap<SmallVec<[(Local, UseKind<SSAIdx>); 2]>>,

    /// `Local -> SSAIdx -> RichLocation`
    pub def_locs: IndexVec<Local, IndexVec<SSAIdx, RichLocation>>,

    pub join_points: JoinPoints<PhiNode>,
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
            let uses = def_use_chain.uses[location]
                .iter()
                .filter(|(_, use_kind)| matches!(use_kind, Use(..)))
                .map(|&(local, _)| (local, def_use_chain.def_loc(local, location)))
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
            let uses = def_use_chain.uses[location]
                .iter()
                .filter(|(_, use_kind)| matches!(use_kind, Use(..)))
                .map(|&(local, _)| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");
        }
    }
}

impl DefUseChain {
    /// Construct a normal def use chain (Definition of def and use is similar to
    /// a liveness analysis)
    pub fn new<'tcx>(body: &Body<'tcx>, tcx: TyCtxt<'tcx>) -> Self {
        let vanilla_builder = VanillaBuilder::default();
        let mut def_use_chain = DefUseChain::initialise(body, vanilla_builder);
        let ssa_state = SSAState::new(body.local_decls.len());
        let mut engine = Engine::new(tcx, body, &mut def_use_chain, ssa_state);
        engine.run(Vanilla);

        def_use_chain
    }

    pub fn initialise<'tcx, L: LocationBuilder<'tcx>>(
        body: &Body<'tcx>,
        location_builder: L,
    ) -> Self {
        let uses = DefUseBuilder::build(body, location_builder);
        let def_locs =
            IndexVec::from_elem(IndexVec::from([RichLocation::Entry]), &body.local_decls);
        let dominance_frontier = compute_dominance_frontier(body);
        let mut def_sites = IndexVec::from_elem(
            BitSet::new_empty(body.basic_blocks.len()),
            &body.local_decls,
        );
        for (location, uses) in uses.iter_enumerated() {
            for &(local, ref use_kind) in uses.iter() {
                if let Def(..) = use_kind {
                    def_sites[local].insert(location.block);
                }
            }
        }
        let join_points = JoinPoints::new(body, &dominance_frontier, &def_sites);

        DefUseChain {
            uses,
            def_locs,
            join_points,
        }
    }

    pub fn pure_uses(&self, location: Location) -> impl Iterator<Item = Local> + '_ {
        self.uses[location].iter().filter_map(|(local, use_kind)| {
            if let Use(..) = use_kind {
                Some(*local)
            } else {
                None
            }
        })
    }

    pub fn peeked_loc(&self, local: Local, location: Location) -> Location {
        let Some(use_kind) = self.uses[location].get_by_key(&local) else {
            panic!("nonexisting peek {:?} @ {:?}", local, location)
        };
        let LocalPeek(ssa_idx) = *use_kind else {
            panic!("nonexisting peek {:?} @ {:?}", local, location)
        };
        match self.def_locs[local][ssa_idx] {
            RichLocation::Mir(loc) => loc,
            _ => unreachable!(),
        }
    }

    pub fn def_loc(&self, local: Local, location: Location) -> RichLocation {
        let Some(use_kind) = self.uses[location].get_by_key(&local) else {
            panic!("nonexisting use {:?} @ {:?}", local, location)
        };
        let Use(ssa_idx) = *use_kind else {
            panic!("nonexisting use {:?} @ {:?}", local, location)
        };
        self.def_locs[local][ssa_idx]
    }

    pub fn phi_node_def_locs(
        &self,
        local: Local,
        block: BasicBlock,
    ) -> impl Iterator<Item = RichLocation> + '_ {
        let phi_node = self.join_points[block]
            .data
            .get_by_key(&local)
            .unwrap_or_else(|| panic!("phi node {:?} does not exist @ {:?}", local, block));

        phi_node
            .rhs
            .iter()
            .copied()
            .filter(|&ssa_idx| ssa_idx != phi_node.lhs)
            .map(move |ssa_idx| self.def_locs[local][ssa_idx])
    }
}

pub struct DefUseBuilder<L> {
    uses: VecVecBuilder<SmallVec<[(Local, UseKind<SSAIdx>); 2]>>,
    location_builder: L,
}

impl<'tcx, L: LocationBuilder<'tcx>> DefUseBuilder<L> {
    pub fn build(
        body: &Body<'tcx>,
        location_builder: L,
    ) -> LocationMap<SmallVec<[(Local, UseKind<SSAIdx>); 2]>> {
        let mut builder = DefUseBuilder {
            uses: VecVec::with_indices_capacity(body.basic_blocks.len()),
            location_builder,
        };
        builder.visit_body(body);
        LocationMap {
            map: builder.uses.complete(),
        }
    }

    fn visit_body(&mut self, body: &Body<'tcx>) {
        for (bb, data) in body.basic_blocks.iter_enumerated() {
            self.visit_basic_block_data(bb, data);
        }
    }

    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
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
            self.location_builder.visit_statement(statement, location);
            let location_uses = self.location_builder.retrieve();
            self.uses.push_element(location_uses);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block,
                statement_index: index,
            };
            self.location_builder.visit_terminator(terminator, location);
            let location_uses = self.location_builder.retrieve();
            self.uses.push_element(location_uses);
        }
        self.uses.complete_cur_vec();
    }
}

pub trait LocationBuilder<'tcx>: Visitor<'tcx> {
    fn retrieve(&mut self) -> SmallVec<[(Local, UseKind<SSAIdx>); 2]>;
}

#[derive(Default)]
pub struct VanillaBuilder {
    location_data: SmallVec<[(Local, UseKind<SSAIdx>); 2]>,
}

impl LocationBuilder<'_> for VanillaBuilder {
    fn retrieve(&mut self) -> SmallVec<[(Local, UseKind<SSAIdx>); 2]> {
        std::mem::take(&mut self.location_data)
    }
}

impl<'tcx> Visitor<'tcx> for VanillaBuilder {
    // for return terminator
    fn visit_local(&mut self, local: Local, context: PlaceContext, _: Location) {
        match VanillaDefUse::for_place(Place::from(local), context) {
            Some(VanillaDefUse::Def) => {
                self.location_data.push((local, Def(Update::new())));
            }
            Some(VanillaDefUse::Use) => self.location_data.push((local, Use(SSAIdx::MAX))),
            None => {}
        }
    }

    fn visit_place(&mut self, place: &Place, context: PlaceContext, location: Location) {
        match VanillaDefUse::for_place(*place, context) {
            Some(VanillaDefUse::Def) => {
                self.location_data.push((place.local, Def(Update::new())));
            }
            Some(VanillaDefUse::Use) => self.location_data.push((place.local, Use(SSAIdx::MAX))),
            None => {}
        }

        // call super_projection so that index operators are visited
        self.super_projection(place.as_ref(), context, location);
    }

    // special casing statements like _1 = BitAnd(_1, _3)
    // in this case, we do not generate usage for the right _1
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Rvalue::BinaryOp(_, box (operand1, operand2)) = rvalue {
            if let Some((lhs, operand1)) = place
                .as_local()
                .zip(operand1.place().and_then(|place| place.as_local()))
            {
                if lhs == operand1 {
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

/// This is to be regularly synced with `rustc_mir_dataflow::impls::liveness`
enum VanillaDefUse {
    Def,
    Use,
}

impl VanillaDefUse {
    fn for_place(place: Place<'_>, context: PlaceContext) -> Option<VanillaDefUse> {
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
                    Some(VanillaDefUse::Use)
                } else if place.projection.is_empty() {
                    Some(VanillaDefUse::Def)
                } else {
                    None
                }
            }

            // Setting the discriminant is not a use because it does no reading, but it is also not
            // a def because it does not overwrite the whole place
            PlaceContext::MutatingUse(MutatingUseContext::SetDiscriminant) => {
                place.is_indirect().then_some(VanillaDefUse::Use)
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
                | NonMutatingUseContext::PlaceMention
                | NonMutatingUseContext::ShallowBorrow
                | NonMutatingUseContext::SharedBorrow,
            ) => Some(VanillaDefUse::Use),

            PlaceContext::MutatingUse(MutatingUseContext::Projection)
            | PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection) => {
                unreachable!("A projection could be a def or a use and must be handled separately")
            }
        }
    }
}
