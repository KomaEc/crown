use rustc_abi::FieldIdx;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{BasicBlock, BinOp, Local, Location, NullOp, Operand, Place, UnOp},
    ty::Const,
};

use super::{constraint::Database, Ctxt};
use crate::flow::{
    def_use::{Def, Update, UseKind},
    inference::{
        Engine, InferAssign, InferCall, InferIrrelevant, InferJoin, InferReturn, Inference,
    },
    join_points::PhiNode,
    SSAIdx,
};

pub struct Analysis<'analysis, const K_LIMIT: usize, DB> {
    ctxt: &'analysis mut Ctxt<K_LIMIT, DB>,
}

impl<'analysis, const K_LIMIT: usize, DB: Database> Analysis<'analysis, K_LIMIT, DB> {
    // fn unconstrained<'tcx>(
    //     &mut self,
    //     engine: &Engine<'_, 'tcx>,
    //     place: &Place<'tcx>,
    //     update: Update<SSAIdx>,
    // ) {
    //         let path = self
    //             .ctxt
    //             .access_path
    //             .path(&Place::from(place.local), engine.body);
    //         let _ = self.ctxt.database.new_tokens(path.1);
    // }

    // fn r#move<'tcx>(
    //     &mut self,
    //     engine: &Engine<'_, 'tcx>,
    //     lhs: &Place<'tcx>,
    //     lhs_use: Option<UseKind<SSAIdx>>,
    //     rhs: &Place<'tcx>,
    //     rhs_use: Option<UseKind<SSAIdx>>,
    // ) {
    //     let lhs_use = lhs_use.and_then(UseKind::update);
    //     let rhs_use = rhs_use.and_then(UseKind::update);
    // }

    // fn copy<'tcx>(
    //     &mut self,
    //     engine: &Engine<'_, 'tcx>,
    //     lhs: &Place<'tcx>,
    //     lhs_use: Option<UseKind<SSAIdx>>,
    //     rhs: &Place<'tcx>,
    //     rhs_use: Option<UseKind<SSAIdx>>,
    // ) {
    // }
}

impl<'analysis, 'tcx, const K_LIMIT: usize, DB: Database> Inference<'tcx>
    for Analysis<'analysis, K_LIMIT, DB>
{
}

impl<'analysis, 'tcx, const K_LIMIT: usize, DB: Database> InferAssign<'tcx>
    for Analysis<'analysis, K_LIMIT, DB>
{
    fn infer_use(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        rhs: &Operand<'tcx>,
        location: Location,
    ) {
        match rhs {
            Operand::Copy(_) => todo!(),
            Operand::Move(_) => todo!(),
            Operand::Constant(_) => {
                let r#use = engine.use_place(lhs, location);
            }
        }
    }

    fn infer_mut_borrow(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    ) {
        todo!()
    }

    fn infer_shr_borrow(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    ) {
        todo!()
    }

    fn infer_mut_addr(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        todo!()
    }

    fn infer_const_addr(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        todo!()
    }

    fn infer_cast(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Operand,
        cast_kind: rustc_middle::mir::CastKind,
        location: Location,
    ) {
        todo!()
    }

    fn infer_binop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        left: &Operand,
        right: &Operand,
        binop: BinOp,
        location: Location,
    ) {
        todo!()
    }

    fn infer_unop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        operand: &Operand,
        unop: UnOp,
        location: Location,
    ) {
        todo!()
    }

    fn infer_nullop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        nullop: NullOp,
        location: Location,
    ) {
        todo!()
    }

    fn infer_discriminant(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        todo!()
    }

    fn infer_deref_copy(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        todo!()
    }

    fn infer_repeat(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        operand: &Operand,
        len: &Const,
        location: Location,
    ) {
        todo!()
    }

    fn infer_aggregate_array(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        values: &IndexVec<FieldIdx, Operand>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_aggregate_adt(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        values: &IndexVec<FieldIdx, Operand>,
        location: Location,
    ) {
        todo!()
    }
}

impl<'analysis, 'tcx, const K_LIMIT: usize, DB: Database> InferReturn<'tcx>
    for Analysis<'analysis, K_LIMIT, DB>
{
    fn infer_return(&mut self, engine: &mut Engine, location: Location) {
        todo!()
    }
}

impl<'analysis, 'tcx, const K_LIMIT: usize, DB: Database> InferCall<'tcx>
    for Analysis<'analysis, K_LIMIT, DB>
{
    fn infer_call(
        &mut self,
        engine: &mut Engine,
        func: &Operand,
        args: &Vec<Operand>,
        destination: &Place,
        location: Location,
    ) {
        todo!()
    }
}

impl<'analysis, 'tcx, const K_LIMIT: usize, DB: Database> InferIrrelevant<'tcx>
    for Analysis<'analysis, K_LIMIT, DB>
{
    fn infer_goto(&mut self, engine: &Engine, target: BasicBlock, location: Location) {
        todo!()
    }

    fn irrelevant_operand(&mut self, engine: &mut Engine, operand: &Operand, location: Location) {
        todo!()
    }
}

impl<'analysis, 'tcx, const K_LIMIT: usize, DB: Database> InferJoin<'tcx>
    for Analysis<'analysis, K_LIMIT, DB>
{
    fn phi_node_output(&mut self, local: Local, ssa_idx: SSAIdx, block: BasicBlock) {
        todo!()
    }

    fn phi_node_input(&mut self, local: Local, ssa_idx: SSAIdx, block: BasicBlock) {
        todo!()
    }

    fn infer_join(&mut self, engine: &Engine, local: Local, phi_node: &PhiNode, block: BasicBlock) {
        todo!()
    }
}
