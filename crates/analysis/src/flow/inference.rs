use std::borrow::BorrowMut;

use rustc_abi::FieldIdx;
use rustc_ast::Mutability;
use rustc_data_structures::graph::WithSuccessors;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        AggregateKind, AssertKind, BasicBlock, BasicBlockData, BinOp, Body, BorrowKind, CastKind,
        Local, Location, NullOp, Operand, Place, ProjectionElem, Rvalue, Statement, StatementKind,
        Terminator, TerminatorKind, UnOp,
    },
    ty::{Const, TyCtxt},
};
use utils::data_structure::assoc::AssocExt;

use super::{def_use::DefUseChain, join_points::PhiNode, state::SSAState, SSAIdx};
use crate::flow::{def_use::UseKind, RichLocation};

/// The set of inference operations
pub trait Inference: InferAssign + InferCall + InferReturn + InferIrrelevant + InferJoin {}

pub trait InferAssign {
    fn infer_use(&mut self, engine: &mut Engine, lhs: &Place, rhs: &Operand, location: Location);
    fn infer_mut_borrow(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    );
    fn infer_shr_borrow(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    );
    fn infer_mut_addr(&mut self, engine: &mut Engine, lhs: &Place, rhs: &Place, location: Location);
    fn infer_const_addr(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    );
    fn infer_cast(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Operand,
        cast_kind: CastKind,
        location: Location,
    );
    fn infer_binop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        left: &Operand,
        right: &Operand,
        binop: BinOp,
        location: Location,
    );
    fn infer_unop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        operand: &Operand,
        unop: UnOp,
        location: Location,
    );
    fn infer_nullop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        nullop: NullOp,
        location: Location,
    );
    fn infer_discriminant(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    );
    fn infer_deref_copy(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    );
    fn infer_repeat(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        operand: &Operand,
        len: &Const,
        location: Location,
    );
    fn infer_aggregate_array(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        values: &IndexVec<FieldIdx, Operand>,
        location: Location,
    );
    fn infer_aggregate_adt(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        values: &IndexVec<FieldIdx, Operand>,
        location: Location,
    );
}

pub trait InferCall {
    fn infer_call(
        &mut self,
        engine: &mut Engine,
        func: &Operand,
        args: &Vec<Operand>,
        destination: &Place,
        location: Location,
    );
}

pub trait InferReturn {
    fn infer_return(&mut self, engine: &mut Engine, location: Location);
}

/// Guaranteed irrelevant parts of the program. Uses of locals are guaranteed to be
/// pure reads.
pub trait InferIrrelevant {
    fn infer_goto(&mut self, engine: &Engine, target: BasicBlock, location: Location);
    fn irrelevant_operand(&mut self, engine: &mut Engine, operand: &Operand, location: Location);
}

pub trait InferJoin {
    fn phi_node_output(&mut self, local: Local, ssa_idx: SSAIdx, block: BasicBlock);
    fn phi_node_input(&mut self, local: Local, ssa_idx: SSAIdx, block: BasicBlock);
    fn infer_join(&mut self, engine: &Engine, local: Local, phi_node: &PhiNode, block: BasicBlock);
}

/// The inference engine that walks over the procedure
pub struct Engine<'engine, 'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'engine Body<'tcx>,
    pub(super) def_use_chain: DefUseChain,
    ssa_state: SSAState,
}

impl<'engine, 'tcx> Engine<'engine, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        body: &'engine Body<'tcx>,
        def_use_chain: DefUseChain,
        ssa_state: SSAState,
    ) -> Self {
        Engine {
            tcx,
            body,
            def_use_chain,
            ssa_state,
        }
    }

    pub fn run<Infer: Inference>(&mut self, mut inference: impl BorrowMut<Infer>) {
        tracing::debug!("Running {:?}", self.body.source.def_id());
        let dominators = self.body.basic_blocks.dominators();
        let mut children = IndexVec::from_elem(vec![], &self.body.basic_blocks);
        let mut root = BasicBlock::from_u32(0);
        self.body.basic_blocks.indices().for_each(|bb| {
            let dom = dominators.immediate_dominator(bb);
            if let Some(dom) = dom {
                children[dom].push(bb);
            } else {
                root = bb;
            }
        });
        assert_eq!(root, BasicBlock::from_u32(0), "expect root");

        enum State {
            ToVisit,
            ToPopNames,
        }
        let mut recursion = vec![(root, State::ToVisit)];

        while let Some((bb, state)) = recursion.pop() {
            match state {
                State::ToVisit => {
                    self.walk_basic_block(inference.borrow_mut(), bb);
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for local in self.def_use_chain.uses[bb]
                        .iter()
                        .flatten()
                        .filter(|(_, use_kind)| matches!(use_kind, UseKind::Def(..)))
                        .map(|(local, _)| *local)
                        .chain(
                            self.def_use_chain.join_points[bb]
                                .iter()
                                .map(|(local, _)| *local),
                        )
                    {
                        let ssa_idx = self.ssa_state.pop(local);
                        tracing::debug!("popping at {:?}: {:?}~{:?}", bb, local, ssa_idx);
                    }
                }
            }
        }

        for (block, bb_data) in self.def_use_chain.join_points.iter_enumerated() {
            for &(local, ref phi_node) in bb_data.iter() {
                inference
                    .borrow_mut()
                    .infer_join(&self, local, phi_node, block);
            }
        }
    }

    fn walk_basic_block<Infer: Inference>(&mut self, inference: &mut Infer, bb: BasicBlock) {
        tracing::debug!("Walking {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = &self.body.basic_blocks[bb];

        // process phi-node definition
        for (local, phi_node) in self.def_use_chain.join_points[bb].iter_enumerated_mut() {
            let ssa_idx = self.ssa_state.fresh_name(local);
            phi_node.lhs = ssa_idx;
            assert_eq!(
                self.def_use_chain.def_locs[local].push(RichLocation::Phi(bb)),
                ssa_idx
            );
            tracing::debug!("defining {:?} at Phi({:?}), def: {:?}", local, bb, ssa_idx);
            inference.phi_node_output(local, ssa_idx, bb);
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            index += 1;

            self.walk_statement(inference, statement, location)
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };

            self.walk_terminator(inference, terminator, location)
        }

        // process phi-nodes uses
        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.def_use_chain.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.ssa_state.get_name(local);
                phi_node.rhs.push(ssa_idx);
                tracing::debug!("using {:?} at Phi({:?}), use: {:?}", local, succ, ssa_idx);
                inference.phi_node_input(local, ssa_idx, succ)
            }
        }
    }

    fn walk_terminator<Infer: Inference>(
        &mut self,
        inference: &mut Infer,
        terminator: &Terminator<'tcx>,
        location: Location,
    ) {
        match &terminator.kind {
            &TerminatorKind::Goto { target } => {
                inference.infer_goto(self, target, location);
            }
            TerminatorKind::SwitchInt { discr, .. } => {
                inference.irrelevant_operand(self, discr, location)
            }
            TerminatorKind::UnwindResume => {
                todo!("Ignoring unwind resume @ {:?}", location)
            }
            TerminatorKind::UnwindTerminate(_) => {
                todo!("Ignoring unwind terminate @ {:?}", location)
            }
            TerminatorKind::Return => inference.infer_return(self, location),
            TerminatorKind::Unreachable => unreachable!("Assume no unreachable in code"),
            TerminatorKind::Drop { .. } => {
                todo!("Ignoring drop @ {:?}", location)
            }
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => inference.infer_call(self, func, args, destination, location),
            TerminatorKind::Assert { cond, box msg, .. } => {
                inference.irrelevant_operand(self, cond, location);
                match msg {
                    AssertKind::BoundsCheck { len, index } => {
                        inference.irrelevant_operand(self, len, location);
                        inference.irrelevant_operand(self, index, location);
                    }
                    AssertKind::Overflow(_, left, right) => {
                        inference.irrelevant_operand(self, left, location);
                        inference.irrelevant_operand(self, right, location);
                    }
                    AssertKind::OverflowNeg(operand) => {
                        inference.irrelevant_operand(self, operand, location)
                    }
                    AssertKind::DivisionByZero(operand) => {
                        inference.irrelevant_operand(self, operand, location)
                    }
                    AssertKind::RemainderByZero(operand) => {
                        inference.irrelevant_operand(self, operand, location)
                    }
                    AssertKind::ResumedAfterReturn(_) => todo!(),
                    AssertKind::ResumedAfterPanic(_) => todo!(),
                    AssertKind::MisalignedPointerDereference { required, found } => {
                        inference.irrelevant_operand(self, required, location);
                        inference.irrelevant_operand(self, found, location);
                    }
                }
            }
            TerminatorKind::Yield { .. }
            | TerminatorKind::GeneratorDrop
            | TerminatorKind::FalseEdge { .. }
            | TerminatorKind::FalseUnwind { .. } => unreachable!(),
            TerminatorKind::InlineAsm { .. } => {
                tracing::debug!("Ignoring inline asm @ {:?}", location)
            }
        }
    }

    fn walk_statement<Infer: Inference>(
        &mut self,
        inference: &mut Infer,
        statement: &Statement<'tcx>,
        location: Location,
    ) {
        tracing::debug!("walking statement {:?}", statement);
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.walk_assign(inference, place, rvalue, location)
            }
            StatementKind::SetDiscriminant {
                place,
                variant_index,
            } => todo!("set {:?} @ {:?}", place, variant_index),
            StatementKind::Deinit(_) => todo!(),
            StatementKind::PlaceMention(_) => todo!(),
            StatementKind::Intrinsic(_) => todo!(),
            StatementKind::FakeRead(_)
            | StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_)
            | StatementKind::Retag(_, _)
            | StatementKind::AscribeUserType(_, _)
            | StatementKind::Coverage(_)
            | StatementKind::ConstEvalCounter
            | StatementKind::Nop => unreachable!("expect no such statements in optimised mir"),
        }
    }

    fn walk_assign<Infer: Inference>(
        &mut self,
        inference: &mut Infer,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        let lhs = place;

        match rvalue {
            Rvalue::Use(operand) => inference.infer_use(self, lhs, operand, location),
            Rvalue::Repeat(operand, len) => {
                inference.infer_repeat(self, lhs, operand, len, location)
            }
            Rvalue::Ref(_, BorrowKind::Mut { .. }, lender) => {
                inference.infer_mut_borrow(self, lhs, lender, location)
            }
            Rvalue::Ref(_, BorrowKind::Shared, lender) => {
                inference.infer_shr_borrow(self, lhs, lender, location)
            }
            Rvalue::AddressOf(Mutability::Not, place) => {
                inference.infer_const_addr(self, lhs, place, location)
            }
            Rvalue::AddressOf(Mutability::Mut, place) => {
                inference.infer_mut_addr(self, lhs, place, location)
            }
            Rvalue::Cast(cast_kind, rhs, ty) => {
                assert_eq!(lhs.ty(self.body, self.tcx).ty, *ty);
                inference.infer_cast(self, lhs, rhs, *cast_kind, location);
            }
            Rvalue::BinaryOp(binop, box (left, right))
            | Rvalue::CheckedBinaryOp(binop, box (left, right)) => {
                inference.infer_binop(self, lhs, left, right, *binop, location);
            }
            Rvalue::UnaryOp(unop, operand) => {
                inference.infer_unop(self, lhs, operand, *unop, location);
            }
            Rvalue::Discriminant(rhs) => inference.infer_discriminant(self, lhs, rhs, location),
            Rvalue::Aggregate(box AggregateKind::Array(_), values) => {
                inference.infer_aggregate_array(self, lhs, values, location)
            }
            Rvalue::Aggregate(box AggregateKind::Adt(..), values) => {
                inference.infer_aggregate_adt(self, lhs, values, location)
            }
            Rvalue::CopyForDeref(rhs) => inference.infer_deref_copy(self, lhs, rhs, location),
            Rvalue::NullaryOp(NullOp::AlignOf, _) => {
                inference.infer_nullop(self, lhs, NullOp::AlignOf, location)
            }
            Rvalue::Ref(_, BorrowKind::Shallow, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Len(_)
            | Rvalue::Aggregate(..)
            | Rvalue::NullaryOp(_, _)
            | Rvalue::ShallowInitBox(_, _) => unreachable!("Rvalue type {:?}", rvalue),
        }
    }

    pub fn try_use_local(&mut self, local: Local, location: Location) -> Option<UseKind<SSAIdx>> {
        let use_kind = self.def_use_chain.uses[location].get_by_key_mut(&local)?;
        let r#use = self.ssa_state.get_name(local);
        Some(match use_kind {
            UseKind::Inspect(ssa_idx) /* | UseKind::LocalPeek(ssa_idx) */ => {
                *ssa_idx = r#use;
                use_kind.clone()
            }
            UseKind::Def(update) => {
                let def = self.ssa_state.fresh_name(local);
                tracing::debug!(
                    "updating {:?} at {:?}, use: {:?}, def: {:?}",
                    local,
                    location,
                    r#use,
                    def
                );
                update.r#use = r#use;
                update.def = def;
                assert_eq!(
                    def,
                    self.def_use_chain.def_locs[local].push(location.into())
                );
                use_kind.clone()
            }
        })
    }

    /// indices are in-significant
    pub fn try_use_place(&mut self, place: &Place, location: Location) -> Option<UseKind<SSAIdx>> {
        let local_use = self.try_use_local(place.local, location);
        for elem in place.projection {
            let ProjectionElem::Index(idx) = elem else {
                continue;
            };
            let _ = self.try_use_local(idx, location);
        }
        local_use
    }
}
