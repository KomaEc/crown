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

type Base = UseKind<OwnershipToken>;
#[cfg(not(debug_assertions))]
const _: () = assert!(std::mem::size_of::<Base>() == 8);

impl<'analysis, const K_LIMIT: usize, DB: Database> Analysis<'analysis, K_LIMIT, DB> {
    // Cached?
    fn base<'tcx>(
        &mut self,
        engine: &Engine<'_, 'tcx>,
        local: Local,
    ) -> std::ops::Range<OwnershipToken> {
        let size = self
            .ctxt
            .access_paths
            .path(&Place::from(local), engine.body)
            .num_pointers_reachable();
        let ownership_tokens = self.ctxt.database.new_tokens(size);

        // TODO monotonicity constraints

        ownership_tokens
    }

    /// If the path is a `Some`, then its size > 0
    fn path<'tcx>(
        &mut self,
        engine: &mut Engine<'_, 'tcx>,
        place: &Place<'tcx>,
        location: Location,
    ) -> Option<Path<Base>> {
        let path = self.ctxt.access_paths.path(place, engine.body);
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
        base.and_then(|base| {
            (path.num_pointers_reachable() > 0).then_some(Path::new(base, path.projections))
        })
    }

    fn r#move<'tcx>(&mut self, engine: &Engine<'_, 'tcx>, lhs: &Path<Base>, rhs: &Path<Base>) {
        todo!()
    }

    fn transfer<'tcx>(&mut self, engine: &Engine<'_, 'tcx>, lhs: &Path<Base>, rhs: &Path<Base>) {
        todo!()
    }

    fn unconstrained<'tcx>(&mut self, path: Option<&Path<Base>>) {
        if let Some(path) = path {
            let _ = path;
        }
    }
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
            Operand::Copy(rhs) => {
                let rhs = self.path(engine, rhs, location);
                match (lhs, rhs) {
                    (None, _) => self.unconstrained(rhs.as_ref()),
                    (_, None) => self.unconstrained(lhs.as_ref()),
                    (Some(lhs), Some(rhs)) => self.transfer(engine, &lhs, &rhs),
                }
            }
            Operand::Move(rhs) => {
                let rhs = self.path(engine, rhs, location);
                match (lhs, rhs) {
                    (None, _) => self.unconstrained(rhs.as_ref()),
                    (_, None) => self.unconstrained(lhs.as_ref()),
                    (Some(lhs), Some(rhs)) => self.r#move(engine, &lhs, &rhs),
                }
            }
            Operand::Constant(_) => self.unconstrained(lhs.as_ref()),
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
        _: rustc_middle::mir::CastKind,
        location: Location,
    ) {
        // FIXME might not be safe! Casting *mut S to *mut T?
        //
        // It is actually wrong. The path for `rhs` needs to be casted
        // self.infer_use(engine, lhs, rhs, location);
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
