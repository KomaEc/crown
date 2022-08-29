use std::ops::Range;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, BasicBlockData, Body, Local, Location, Operand, Place, Rvalue,
        Statement, StatementKind,
    },
    ty::TyCtxt,
};

use crate::{
    analysis::{def_sites::Definitions, state::{SSAState, SSAIdx}, ty_ext::TyExt},
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

pub(crate) struct Renamer<'me, 'tcx: 'me> {
    body: &'me Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    struct_topology: &'me StructTopology,
    state: SSAState,
    definitions: Definitions,
}

impl<'me, 'tcx: 'me> Renamer<'me, 'tcx> {
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
                    for &local in self.definitions.of_block(bb).iter().flatten() {
                        self.state.name_state.pop(local);
                    }
                }
            }
        }
    }

    pub(crate) fn go_basic_block<M: Mode>(&mut self, bb: BasicBlock, data: &BasicBlockData) {
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
            // go statement
            todo!();
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            // go terminator
            todo!()
        }

        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.state.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.state.name_state.get_name(local);
                phi_node.rhs.push(ssa_idx)
            }
        }

        todo!()
    }

    fn go_statement<M: Mode>(&mut self, statement: &Statement<'tcx>, location: Location) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.go_assign::<M>(place, rvalue, location)
            }
            StatementKind::SetDiscriminant { .. } => todo!(),
            StatementKind::AscribeUserType(_, _) => todo!(),
            StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_)
            | StatementKind::Deinit(_)
            | StatementKind::Retag(_, _)
            | StatementKind::FakeRead(_)
            | StatementKind::Coverage(_)
            | StatementKind::CopyNonOverlapping(_)
            | StatementKind::Nop => {
                unreachable!("statement {:?} is not assumed to appear", statement)
            }
        }
    }

    fn go_assign<M: Mode>(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);

        let ty = place.ty(self.body, self.tcx).ty;
        if !ty.contains_ptr(self.struct_topology) {
            return;
        }

        if self
            .definitions
            .of_location(location)
            .contains(&place.local)
        {
            let ssa_idx = self.state.name_state.generate_fresh_name(place.local);
            tracing::debug!("fresh name for {:?}: {:?}", place.local, ssa_idx);
        }

        match rvalue {
            Rvalue::Use(Operand::Constant(_)) | Rvalue::Cast(_, Operand::Constant(_), _) => todo!(),
            Rvalue::Use(Operand::Copy(rplace) | Operand::Move(rplace))
            | Rvalue::Cast(_, Operand::Copy(rplace) | Operand::Move(rplace), _) => todo!(),
            Rvalue::CopyForDeref(rplace) => todo!(),
            Rvalue::Ref(_, _, rplace) | Rvalue::AddressOf(_, rplace) => todo!(),
            Rvalue::BinaryOp(_, _) | Rvalue::CheckedBinaryOp(_, _) | Rvalue::UnaryOp(_, _) => {
                unreachable!("{:?}: {ty} cannot contain ptr", rvalue)
            }
            Rvalue::NullaryOp(_, _)
            | Rvalue::Aggregate(_, _)
            | Rvalue::Discriminant(_)
            | Rvalue::Len(_)
            | Rvalue::ShallowInitBox(_, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Repeat(_, _) => unreachable!("rvalue {:?} is not assumed to appear", rvalue),
        }
    }

    fn go_place<M: Mode>(&mut self, place: &Place, location: Location) {
        if self
            .definitions
            .of_location(location)
            .contains(&place.local)
        {
            let ssa_idx = self.state.name_state.generate_fresh_name(place.local);
            tracing::debug!("fresh name for {:?}: {:?}", place.local, ssa_idx);
        }
    }
}
