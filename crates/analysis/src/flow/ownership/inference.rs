use either::Either::{Left, Right};
use rustc_abi::FieldIdx;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{BasicBlock, BinOp, Local, Location, NullOp, Operand, Place, UnOp},
    ty::Const,
};

use super::{
    access_path::Path,
    constraint::{Database, OwnershipToken},
    Ctxt,
};
use crate::flow::{
    def_use::{Def, Inspect, Update, UseKind},
    inference::{
        Engine, InferAssign, InferCall, InferIrrelevant, InferJoin, InferReturn, Inference,
    },
    join_points::PhiNode,
    SSAIdx,
};

pub struct Analysis<'analysis, const K_LIMIT: usize, DB> {
    ctxt: &'analysis mut Ctxt<K_LIMIT, DB>,
    /// `Local -> SSAIdx -> first token`
    tokens: &'analysis mut IndexVec<Local, IndexVec<SSAIdx, OwnershipToken>>,
}

type Base = Option<UseKind<OwnershipToken>>;
#[cfg(not(debug_assertions))]
const _: () = assert!(std::mem::size_of::<Base>() == 8);

// fn path_tokens(path: &Path<Base>, expected_depth: usize) -> impl Iterator<Item = OwnershipToken> {
//     if path.1.depth() == expected_depth {
//         let base = path.0.unwrap().update().unwrap().def;
//         Left(base + path.1.offset()..base + path.1.offset() + path.1.size())
//     } else {
//         Right(todo!())
//     }
// }

fn min_depth(path1: &Path<Base>, path2: &Path<Base>) -> usize {
    std::cmp::min(path1.1.depth(), path2.1.depth())
}

impl<'analysis, const K_LIMIT: usize, DB: Database> Analysis<'analysis, K_LIMIT, DB> {
    // Cached? 
    fn base<'tcx>(&mut self, engine: &Engine<'_, 'tcx>, local: Local) -> std::ops::Range<OwnershipToken> {

        let size = self.ctxt.access_paths.projections(&Place::from(local), engine.body).size();
        let ownership_tokens = self.ctxt.database.new_tokens(size);

        // TODO monotonicity constraints

        ownership_tokens
    }

    fn path<'tcx>(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        place: &Place<'tcx>,
        location: Location,
    ) -> Path<Base> {
        let projections = self.ctxt.access_paths.projections(place, engine.body);
        let base = engine.use_base_of(place, location).map(|base| match base {
            Inspect(ssa_idx) => Inspect(self.tokens[place.local][ssa_idx]),
            Def(Update { r#use, def }) => {
                let new_tokens = self.base(engine, place.local);
                assert_eq!(self.tokens[place.local].push(new_tokens.start), def);
                Def(Update {
                    r#use: self.tokens[place.local][r#use],
                    def: new_tokens.start,
                })
            }
        });
        (base, projections)
    }

    fn r#move<'tcx>(&mut self, engine: &Engine<'_, 'tcx>, lhs: &Path<Base>, rhs: &Path<Base>) {
        if lhs.1.size() > 0 && rhs.1.size() > 0 {
            let min_depth = min_depth(lhs, rhs);

        }
    }
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
        let lhs = self.path(engine, lhs, location);
        match rhs {
            Operand::Copy(_) => todo!(),
            Operand::Move(rhs) => {
                let rhs = self.path(engine, rhs, location);
                self.r#move(&engine, &lhs, &rhs);
            }
            Operand::Constant(_) => {
                // let r#use = engine.use_base_of(lhs, location);
            }
        }
    }

    fn infer_mut_borrow(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        lender: &Place<'tcx>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_shr_borrow(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        lender: &Place<'tcx>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_mut_addr(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        rhs: &Place<'tcx>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_const_addr(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        rhs: &Place<'tcx>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_cast(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        rhs: &Operand<'tcx>,
        cast_kind: rustc_middle::mir::CastKind,
        location: Location,
    ) {
        todo!()
    }

    fn infer_binop(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        left: &Operand<'tcx>,
        right: &Operand<'tcx>,
        binop: BinOp,
        location: Location,
    ) {
        todo!()
    }

    fn infer_unop(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        operand: &Operand<'tcx>,
        unop: UnOp,
        location: Location,
    ) {
        todo!()
    }

    fn infer_nullop(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        nullop: NullOp,
        location: Location,
    ) {
        todo!()
    }

    fn infer_discriminant(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        rhs: &Place<'tcx>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_deref_copy(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        rhs: &Place<'tcx>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_repeat(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        operand: &Operand<'tcx>,
        len: &Const<'tcx>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_aggregate_array(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        values: &IndexVec<FieldIdx, Operand<'tcx>>,
        location: Location,
    ) {
        todo!()
    }

    fn infer_aggregate_adt(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        lhs: &Place<'tcx>,
        values: &IndexVec<FieldIdx, Operand<'tcx>>,
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
