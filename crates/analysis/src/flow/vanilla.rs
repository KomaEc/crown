use rustc_middle::mir::{
    BasicBlock, BinOp, CastKind, Local, Location, NullOp, Operand, Place, UnOp, RETURN_PLACE,
};

use super::{
    infer::{
        Engine, InferAssign, InferCall, InferJoin, EnsureIrrelevant, InferReturn, Inference,
    },
    join_points::PhiNode,
    SSAIdx,
};

/// Vanilla inference rules
pub struct Vanilla;

impl Vanilla {
    fn touch_local(engine: &mut Engine, local: Local, location: Location) {
        let _ = engine.try_use_local(local, location);
    }

    fn touch_place(engine: &mut Engine, place: &Place, location: Location) {
        let _ = engine.try_use_place(place, location);
    }

    fn touch_operand(engine: &mut Engine, operand: &Operand, location: Location) {
        match operand {
            Operand::Copy(place) | Operand::Move(place) => {
                let _ = engine.try_use_place(place, location);
            }
            Operand::Constant(box _) => {}
        }
    }
}

impl Inference for Vanilla {}

impl InferAssign for Vanilla {
    fn infer_use(&mut self, engine: &mut Engine, lhs: &Place, rhs: &Operand, location: Location) {
        Vanilla::touch_place(engine, lhs, location);
        Vanilla::touch_operand(engine, rhs, location);
    }

    fn infer_mut_borrow(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    ) {
        self.infer_shr_borrow(engine, lhs, lender, location)
    }

    fn infer_shr_borrow(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        lender: &Place,
        location: Location,
    ) {
        Vanilla::touch_place(engine, lhs, location);
        Vanilla::touch_place(engine, lender, location);
    }

    fn infer_mut_addr(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        self.infer_const_addr(engine, lhs, rhs, location)
    }

    fn infer_const_addr(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        Vanilla::touch_place(engine, lhs, location);
        Vanilla::touch_place(engine, rhs, location);
    }

    fn infer_cast(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Operand,
        _: CastKind,
        location: Location,
    ) {
        self.infer_use(engine, lhs, rhs, location);
    }

    fn infer_binop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        left: &Operand,
        right: &Operand,
        _: BinOp,
        location: Location,
    ) {
        Vanilla::touch_place(engine, lhs, location);
        Vanilla::touch_operand(engine, left, location);
        Vanilla::touch_operand(engine, right, location);
    }

    fn infer_unop(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        operand: &Operand,
        _: UnOp,
        location: Location,
    ) {
        Vanilla::touch_place(engine, lhs, location);
        Vanilla::touch_operand(engine, operand, location);
    }

    fn infer_discriminant(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        Vanilla::touch_place(engine, lhs, location);
        Vanilla::touch_place(engine, rhs, location);
    }

    fn infer_deref_copy(
        &mut self,
        engine: &mut Engine,
        lhs: &Place,
        rhs: &Place,
        location: Location,
    ) {
        Vanilla::touch_place(engine, lhs, location);
        Vanilla::touch_place(engine, rhs, location);
    }

    fn infer_nullop(&mut self, engine: &mut Engine, lhs: &Place, _: NullOp, location: Location) {
        Vanilla::touch_place(engine, lhs, location)
    }
}

impl InferCall for Vanilla {
    fn infer_call(
        &mut self,
        engine: &mut Engine,
        func: &Operand,
        args: &Vec<Operand>,
        destination: &Place,
        location: Location,
    ) {
        Vanilla::touch_operand(engine, func, location);
        for arg in args {
            Vanilla::touch_operand(engine, arg, location);
        }
        Vanilla::touch_place(engine, destination, location)
    }
}

impl InferReturn for Vanilla {
    fn infer_return(&mut self, engine: &mut Engine, location: Location) {
        Vanilla::touch_local(engine, RETURN_PLACE, location)
    }
}

impl EnsureIrrelevant for Vanilla {
    fn irrelevant_operand(
        &mut self,
        engine: &mut Engine,
        operand: &Operand,
        location: Location,
    ) {
        Vanilla::touch_operand(engine, operand, location)
    }
}

impl InferJoin for Vanilla {
    fn infer_join(&mut self, _: &Engine, _: Local, _: &PhiNode, _: BasicBlock) {}

    fn phi_node_output(&mut self, _: Local, _: SSAIdx, _: BasicBlock) {}

    fn phi_node_input(&mut self, _: Local, _: SSAIdx, _: BasicBlock) {}
}
