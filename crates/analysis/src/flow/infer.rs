use std::borrow::BorrowMut;

use rustc_ast::Mutability;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        AggregateKind, BasicBlock, BasicBlockData, BinOp, Body, BorrowKind, CastKind, Location,
        NullOp, Operand, Place, Rvalue, Statement, StatementKind, Terminator, TerminatorKind, UnOp,
    },
    ty::TyCtxt,
};

/// The set of inference operations
pub trait Inference:
    HasState + InferAssign + InferCall + InferReturn + InferOperandMention
{
}

// leave it to `Engine` maybe?
pub trait HasState {}

pub trait InferAssign {
    fn infer_use(&mut self, engine: &Engine, lhs: &Place, rhs: &Operand, location: Location);
    fn infer_mut_borrow(
        &mut self,
        engine: &Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    );
    fn infer_shr_borrow(
        &mut self,
        engine: &Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    );
    fn infer_mut_addr(&mut self, engine: &Engine, lhs: &Place, rhs: &Place, location: Location);
    fn infer_const_addr(&mut self, engine: &Engine, lhs: &Place, rhs: &Place, location: Location);
    fn infer_cast(
        &mut self,
        engine: &Engine,
        lhs: &Place,
        rhs: &Operand,
        cast_kind: CastKind,
        location: Location,
    );
    fn infer_binop(
        &mut self,
        engine: &Engine,
        lhs: &Place,
        left: &Operand,
        right: &Operand,
        binop: BinOp,
        location: Location,
    );
    fn infer_unop(
        &mut self,
        engine: &Engine,
        lhs: &Place,
        operand: &Operand,
        unop: UnOp,
        location: Location,
    );
    fn infer_discriminant(&mut self, engine: &Engine, lhs: &Place, rhs: &Place, location: Location);
    fn infer_deref_copy(&mut self, engine: &Engine, lhs: &Place, rhs: &Place, location: Location);
}

pub trait InferCall {
    fn infer_call(
        &mut self,
        engine: &Engine,
        func: &Operand,
        args: &Vec<Operand>,
        destination: &Place,
        location: Location,
    );
}

pub trait InferReturn {
    fn infer_return(&mut self, engine: &Engine, location: Location);
}

pub trait InferOperandMention {
    fn infer_operand_mention(&mut self, engine: &Engine, operand: &Operand, location: Location);
}

/// The inference engine that walks over the procedure
pub struct Engine<'engine, 'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'engine Body<'tcx>,
}

impl<'tcx> Engine<'_, 'tcx> {
    pub fn run<Infer: Inference>(&self, mut inference: impl BorrowMut<Infer>) {
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
                    // update numbering
                    todo!()
                }
            }
        }
    }

    fn walk_basic_block<Infer: Inference>(&self, inference: &mut Infer, bb: BasicBlock) {
        tracing::debug!("Walking {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = &self.body.basic_blocks[bb];

        // process phi-nodes

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

        // process phi-nodes
    }

    fn walk_terminator<Infer: Inference>(
        &self,
        inference: &mut Infer,
        terminator: &Terminator<'tcx>,
        location: Location,
    ) {
        match &terminator.kind {
            TerminatorKind::Goto { .. } => {
                tracing::debug!("Ignoring goto @ {:?}", location);
            }
            TerminatorKind::SwitchInt { discr, .. } => {
                inference.infer_operand_mention(self, discr, location)
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
            TerminatorKind::Assert { cond, .. } => {
                inference.infer_operand_mention(self, cond, location)
            }
            // TerminatorKind::Assert {
            //     cond,
            //     expected,
            //     msg,
            //     target,
            //     unwind,
            // } => todo!(),
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
        &self,
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
            } => todo!(),
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
        &self,
        inference: &mut Infer,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        let lhs = place;

        match rvalue {
            Rvalue::Use(operand) => inference.infer_use(self, lhs, operand, location),
            Rvalue::Repeat(_, _) => unimplemented!("Rvalue::Repeat @ {:?}", location),
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
            Rvalue::Aggregate(box AggregateKind::Array(_), _) => {
                unimplemented!("Rvalue::Aggregate @ {:?}", location)
            }
            Rvalue::CopyForDeref(rhs) => inference.infer_deref_copy(self, lhs, rhs, location),
            Rvalue::NullaryOp(NullOp::AlignOf, _) => tracing::debug!("ignoring AlignOf"),
            Rvalue::Ref(_, BorrowKind::Shallow, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Len(_)
            | Rvalue::Aggregate(..)
            | Rvalue::NullaryOp(_, _)
            | Rvalue::ShallowInitBox(_, _) => unreachable!("Rvalue type {:?}", rvalue),
        }
    }
}
