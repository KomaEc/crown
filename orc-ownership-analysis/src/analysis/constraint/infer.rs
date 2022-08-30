use std::ops::Range;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        BasicBlock, BasicBlockData, Body, CastKind, Local, Location, Operand, Place, Rvalue,
        Statement, StatementKind, Terminator, TerminatorKind, RETURN_PLACE,
    },
    ty::TyCtxt,
};

use crate::{
    analysis::{
        def::Definitions,
        state::{SSAIdx, SSAState},
        ty_ext::TyExt,
    },
    struct_topology::StructTopology,
};

use super::{CadicalDatabase, Database, OwnershipSig};

/// Should it hold 'tcx at all?
pub(crate) struct InferCtxt<'me, DB = CadicalDatabase>
where
    DB: Database,
{
    database: &'me mut DB,
    local_sig: IndexVec<Local, IndexVec<SSAIdx, Range<OwnershipSig>>>,
    // store: VecArray<Range<OwnershipSig>>,
    // TODO: signatures for the function that is analysed (and perhaps
    // those in the same connected component)
}

/// FIXME: is it the right way?
pub(crate) trait Mode {
    type Ctxt<'me, DB>
    where
        Self: 'me,
        DB: Database + 'me;
}
#[derive(Debug)]
pub(crate) struct Pure;
#[derive(Debug)]
pub(crate) struct WithCtxt;
impl Mode for Pure {
    type Ctxt<'me, DB: Database + 'me> = ();
}
impl Mode for WithCtxt {
    type Ctxt<'me, DB> = InferCtxt<'me, DB> where Self: 'me, DB: Database + 'me;
}

pub(crate) struct Renamer<'rn, 'tcx: 'rn> {
    body: &'rn Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    struct_topology: &'rn StructTopology,
    state: SSAState,
    definitions: Definitions,
}

impl<'rn, 'tcx: 'rn> Renamer<'rn, 'tcx> {
    pub(crate) fn go<M: Mode>(&mut self) {
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
                    // self.state.name_state.enter_new_block();
                    self.go_basic_block::<M>(bb, &self.body.basic_blocks()[bb]);
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for local in self.definitions.of_block(bb).iter().flatten().copied() {
                        self.state.name_state.pop(local);
                    }
                }
            }
        }
    }

    pub(crate) fn go_basic_block<M: Mode>(&mut self, bb: BasicBlock, data: &BasicBlockData<'tcx>) {
        tracing::debug!("Renaming {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for (local, phi_node) in self.state.join_points[bb].iter_enumerated_mut() {
            let ssa_idx = self.state.name_state.generate_fresh_name(local);
            phi_node.lhs = ssa_idx;
            todo!()
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
                phi_node.rhs.push(ssa_idx)
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
                let destination = destination.as_local().unwrap();
                let dest_ty = self.body.local_decls[destination].ty;
                if dest_ty.contains_ptr(&self.struct_topology) {
                    let _ = self.state.consume_at(destination, location).unwrap();
                    tracing::error!("TODO")
                }
                for arg in args {
                    match arg {
                        Operand::Move(arg) | Operand::Copy(arg) => {
                            let arg_ty = arg.ty(self.body, self.tcx).ty;
                            if arg_ty.contains_ptr(&self.struct_topology) {
                                let _ = self.state.consume_at(arg.local, location).unwrap();
                            }
                        }
                        _ => continue,
                    }
                }
            }
            TerminatorKind::Return => {
                let return_place = RETURN_PLACE;
                let ret_ty = self.body.local_decls[return_place].ty;
                if ret_ty.contains_ptr(&self.struct_topology) {
                    let _ = self.state.consume_at(return_place, location).unwrap();
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

        // Is this condition even necessary? If it is to be defined, then
        // it must contain pointers!!!
        let ty = lhs.ty(self.body, self.tcx).ty;
        if !ty.contains_ptr(self.struct_topology) {
            if let Rvalue::Cast(
                CastKind::PointerExposeAddress,
                Operand::Move(rhs) | Operand::Copy(rhs),
                _,
            ) = rhs
            {
                let _ = self.state.consume_at(rhs.local, location);
            }
            return;
        }

        let lhs_consume = self.state.consume_at(lhs.local, location);

        match rhs {
            Rvalue::Use(Operand::Constant(..)) => {
                tracing::debug!("ignoring rvalue {:?}", rhs)
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, ..) => {
                tracing::debug!("ignoring rvalue {:?}", rhs)
            }

            Rvalue::Cast(CastKind::PointerExposeAddress, ..) => {
                unreachable!("lhs shoud be a pointer")
            }

            Rvalue::Cast(_, Operand::Constant(_constant), _) => {
                todo!("constant pointer {:?}", _constant)
            }

            Rvalue::Use(Operand::Move(rhs) | Operand::Copy(rhs))
            | Rvalue::Cast(_, Operand::Move(rhs) | Operand::Copy(rhs), _) => {
                // if def_of_loc.contains(&rhs.local) {
                //     let ssa_idx = self.state.name_state.generate_fresh_name(rhs.local);
                //     tracing::debug!("fresh name for {:?}: {:?}", rhs.local, ssa_idx);
                // }
                let rhs_consume = self.state.consume_at(rhs.local, location);
                let (Some(lhs_consume), Some(rhs_consume)) = (lhs_consume, rhs_consume) else { return };
                tracing::error!("TODO");
            }

            Rvalue::CopyForDeref(rhs) => {
                tracing::warn!("deref_copy is ignored")
            }

            Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                let lhs = lhs
                    .as_local()
                    .expect("we assume that rustc guarantees the lhs of `p = &q` being local");

                tracing::error!("TODO");
            }

            Rvalue::BinaryOp(_, _) | Rvalue::CheckedBinaryOp(_, _) | Rvalue::UnaryOp(_, _) => {
                unreachable!("{:?}: {ty} cannot contain ptr", rhs)
            }
            Rvalue::NullaryOp(_, _)
            | Rvalue::Aggregate(_, _)
            | Rvalue::Discriminant(_)
            | Rvalue::Len(_)
            | Rvalue::ShallowInitBox(_, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Repeat(_, _) => unreachable!("rvalue {:?} is not assumed to appear", rhs),
        }
    }
}
