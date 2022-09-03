use std::ops::Range;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, Body, CastKind, Local, Location, Operand, Place, Rvalue, Statement,
    StatementKind, Terminator, TerminatorKind, RETURN_PLACE,
};

use crate::{
    analysis::{
        body_ext::DominanceFrontier,
        def::{Consume, Definitions, RichLocation},
        state::{SSAIdx, SSAState},
    },
    struct_topology::StructTopology,
};

use super::{CadicalDatabase, Database, OwnershipSig};

/// Should it hold 'tcx at all?
pub(crate) struct InferCtxt<DB = CadicalDatabase>
where
    DB: Database,
{
    database: DB,
    local_sig: IndexVec<Local, IndexVec<SSAIdx, Range<OwnershipSig>>>,
    // store: VecArray<Range<OwnershipSig>>,
    // TODO: signatures for the function that is analysed (and perhaps
    // those in the same connected component)
}

impl<DB: Database> InferCtxt<DB> {
    pub(crate) fn consume_place<'tcx>(
        &mut self,
        place: &Place<'tcx>,
        consume: Consume,
        struct_topology: &StructTopology,
    ) {
    }
}

/// FIXME: is it the right way?
pub(crate) trait Mode {
    type Ctxt<DB>
    where
        DB: Database;
}
#[derive(Debug)]
pub(crate) struct Pure;
#[derive(Debug)]
pub(crate) struct WithCtxt;
impl Mode for Pure {
    type Ctxt<DB: Database> = ();
}
impl Mode for WithCtxt {
    type Ctxt<DB> = InferCtxt<DB> where DB: Database;
}

pub(crate) struct Renamer<'rn, 'tcx: 'rn> {
    body: &'rn Body<'tcx>,
    state: SSAState,
}

impl<'rn, 'tcx: 'rn> Renamer<'rn, 'tcx> {
    pub(crate) fn new(
        body: &'rn Body<'tcx>,
        dominance_frontier: &DominanceFrontier,
        definitions: Definitions,
    ) -> Self {
        Renamer {
            body,
            state: SSAState::new(body, dominance_frontier, definitions),
        }
    }

    pub(crate) fn go<M: Mode>(&mut self) {
        tracing::debug!("Renaming {:?}", self.body.source.def_id());
        let dominators = self.body.basic_blocks.dominators();
        let mut children = IndexVec::from_elem(vec![], self.body.basic_blocks());
        let mut root = BasicBlock::from_u32(0);
        self.body.basic_blocks().indices().for_each(|bb| {
            let dom = dominators.immediate_dominator(bb);
            if dom != bb {
                children[dom].push(bb)
            } else {
                root = bb;
            }
        });
        assert_eq!(root, BasicBlock::from_u32(0));

        enum State {
            ToVisit,
            ToPopNames,
        }

        let mut recursion = vec![(root, State::ToVisit)];

        while let Some((bb, state)) = recursion.pop() {
            match state {
                State::ToVisit => {
                    self.go_basic_block::<M>(bb, &self.body.basic_blocks()[bb]);
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for &(local, _) in self.state.consume_chain.of_block(bb).iter().flatten() {
                        let ssa_idx = self.state.name_state.pop(local);
                        tracing::debug!("popping at {:?}: {:?}~{:?}", bb, local, ssa_idx);
                    }
                }
            }
        }
    }

    fn go_basic_block<M: Mode>(&mut self, bb: BasicBlock, data: &BasicBlockData<'tcx>) {
        tracing::debug!("Renaming {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for (local, phi_node) in self.state.join_points[bb].iter_enumerated_mut() {
            let ssa_idx = self.state.name_state.generate_fresh_name(local);
            phi_node.lhs = ssa_idx;
            assert_eq!(
                self.state.consume_chain.locs[local].push(RichLocation::Phi(bb)),
                ssa_idx
            );
            tracing::debug!("defining {:?} at Phi({:?}), def: {:?}", local, bb, ssa_idx)
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_statement::<M>(statement, location);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_terminator::<M>(terminator, location);
        }

        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.state.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.state.name_state.get_name(local);
                // .unwrap_or_else(|| panic!("{:?}: {}", local, self.body.local_decls[local].ty));
                phi_node.rhs.push(ssa_idx);
                tracing::debug!("using {:?} at Phi({:?}), use: {:?}", local, succ, ssa_idx)
            }
        }
    }

    fn go_statement<M: Mode>(&mut self, statement: &Statement<'tcx>, location: Location) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.go_assign::<M>(place, rvalue, location)
            }
            StatementKind::SetDiscriminant { .. } => {
                tracing::debug!("ignoring SetDiscriminant statement {:?}", statement)
            }
            StatementKind::Deinit(..) => {
                tracing::debug!("ignoring Deinit statement {:?}", statement)
            }
            StatementKind::AscribeUserType(_, _)
            | StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_)
            | StatementKind::Retag(_, _)
            | StatementKind::FakeRead(_)
            | StatementKind::Coverage(_)
            | StatementKind::CopyNonOverlapping(_)
            | StatementKind::Nop => {
                unreachable!("statement {:?} is not assumed to appear", statement)
            }
        }
    }

    /// TODO: handle return?
    fn go_terminator<M: Mode>(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        match &terminator.kind {
            TerminatorKind::Call {
                args, destination, ..
            } => {
                tracing::debug!("processing terminator {:?}", terminator.kind);
                let destination = destination.as_local().unwrap();
                let _ = self.state.try_consume_at(destination, location);
                for arg in args {
                    match arg {
                        Operand::Move(arg) | Operand::Copy(arg) => {
                            let _ = self.state.try_consume_at(arg.local, location);
                        }
                        _ => continue,
                    }
                }
            }
            TerminatorKind::Return => {
                tracing::debug!("processing terminator {:?}", terminator.kind);
                let return_place = RETURN_PLACE;
                let _ = self.state.try_consume_at(return_place, location);
                // finalise!
                for local in self.state.consume_chain.to_finalise() {
                    let _ = self.state.name_state.get_name(local);
                }
            }
            _ => {}
        }
    }

    fn go_assign<M: Mode>(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);

        let lhs = place;
        let rhs = rvalue;

        let lhs_consume = self.state.try_consume_at(lhs.local, location);
        // let rhs_consume = self.state.try_consume_at(rhs.local, location);

        match rhs {
            Rvalue::Use(Operand::Constant(_)) => {
                // assert!(lhs_consume.is_none(), "pointers cannot be mir constants: {:?}", constant)
                if lhs_consume.is_some() {
                    tracing::debug!("constant pointer rvalue {:?}", rhs)
                }
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
                tracing::debug!("untrusted pointer source: raw address {:?}", operand)
            }

            Rvalue::Cast(
                CastKind::PointerExposeAddress,
                Operand::Copy(rhs) | Operand::Move(rhs),
                _,
            ) => {
                tracing::debug!("untrusted pointer sink: address {:?}", lhs);
                let _ = self.state.consume_at(rhs.local, location);
            }

            Rvalue::Cast(_, Operand::Constant(box constant), _) => {
                assert!(
                    lhs_consume.is_none(),
                    "TODO: constant pointer {:?}",
                    constant
                )
                // todo!("constant pointer {:?}, cast_kind: {:?}", constant)
            }

            Rvalue::Use(Operand::Move(rhs) | Operand::Copy(rhs))
            | Rvalue::Cast(_, Operand::Move(rhs) | Operand::Copy(rhs), _) => {
                let rhs_consume = self.state.try_consume_at(rhs.local, location);
                let (Some(lhs_consume), Some(rhs_consume)) = (lhs_consume, rhs_consume) else { return };
                // tracing::error!("TODO");
            }

            Rvalue::CopyForDeref(rhs) => {
                tracing::warn!("deref_copy is ignored")
            }

            Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                let lhs = lhs
                    .as_local()
                    .expect("we assume that rustc guarantees the lhs of `p = &q` being local");

                // tracing::error!("TODO");
            }

            Rvalue::BinaryOp(_, _) | Rvalue::CheckedBinaryOp(_, _) | Rvalue::UnaryOp(_, _) => {
                // unreachable!("{:?}: {ty} cannot contain ptr", rhs)
                return;
            }
            Rvalue::NullaryOp(_, _)
            | Rvalue::Aggregate(_, _)
            | Rvalue::Discriminant(_)
            | Rvalue::Len(_)
            | Rvalue::ShallowInitBox(_, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Repeat(_, _) => return, // unreachable!("rvalue {:?} is not assumed to appear", rhs),
        }
    }
}
