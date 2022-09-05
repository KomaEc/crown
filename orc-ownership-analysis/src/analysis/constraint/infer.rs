use std::{borrow::BorrowMut, ops::Range};

use itertools::izip;
// use orc_common::pointer::BorrowMut;
use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, Body, CastKind, Local, Location, Operand, Place, ProjectionElem,
    Rvalue, Statement, StatementKind, Terminator, TerminatorKind, RETURN_PLACE,
};
use rustc_type_ir::TyKind;

use crate::{
    analysis::{
        body_ext::DominanceFrontier,
        constraint::{CadicalDatabase, Database, OwnershipSig},
        def::{Consume, Definitions, RichLocation},
        state::{SSAIdx, SSAState},
    },
    CrateCtxt,
};

pub(crate) struct InferCtxt<'infercx, 'tcx, DB = CadicalDatabase> {
    database: DB,
    crate_ctxt: &'infercx CrateCtxt<'tcx>,
    local_sig: IndexVec<Local, IndexVec<SSAIdx, Range<OwnershipSig>>>,
    next: OwnershipSig,
}

impl<'infercx, 'tcx, DB> InferCtxt<'infercx, 'tcx, DB>
where
    'tcx: 'infercx,
    DB: Database,
{
    pub(crate) fn new(
        crate_ctxt: &'infercx CrateCtxt<'tcx>,
        body: &Body<'tcx>,
        definitions: &Definitions,
        db: DB,
    ) -> Self {
        let mut local_sig = IndexVec::with_capacity(definitions.def_sites.len());
        let mut next = OwnershipSig::MIN;

        for local in definitions.def_sites.indices() {
            if definitions.to_finalise.contains(local) {
                let ty = body.local_decls[local].ty;
                let count = crate_ctxt.ty_ptr_count(ty);
                let sigs = vec![next..next + count];
                next += count;
                local_sig.push(IndexVec::from_raw(sigs));
            } else {
                local_sig.push(IndexVec::default());
            }
        }

        InferCtxt {
            database: db,
            crate_ctxt,
            local_sig,
            next,
        }
    }

    pub(crate) fn new_sigs(&mut self, size: u32) -> Range<OwnershipSig> {
        let start = self.next;
        let end = start + size;
        self.next = end;
        start..end
    }
}

/// FIXME: is it the right way?
pub(crate) trait Mode {
    type Ctxt<'infercx, 'tcx, DB>
    where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx;

    type ConsumeResult;

    fn interpret_consume<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume<SSAIdx>,
    ) -> Self::ConsumeResult
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn transfer_ownership<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        lhs_result: Self::ConsumeResult,
        rhs_result: Self::ConsumeResult,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn finalise<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        local: Local,
        r#use: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;
}
#[derive(Debug)]
pub(crate) struct Pure;
#[derive(Debug)]
pub(crate) struct WithCtxt;
impl Mode for Pure {
    type Ctxt<'infercx, 'tcx, DB> = ()
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    type ConsumeResult = ();

    #[inline]
    fn interpret_consume<'infercx, 'tcx, DB>(
        (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: &Body<'tcx>,
        _: &Place<'tcx>,
        _: Consume<SSAIdx>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    #[inline]
    fn transfer_ownership<'infercx, 'tcx, DB>(
        (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
        (): Self::ConsumeResult,
        (): Self::ConsumeResult,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn finalise<'infercx, 'tcx, DB>((): &mut Self::Ctxt<'infercx, 'tcx, DB>, _: Local, _: SSAIdx)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }
}
impl Mode for WithCtxt {
    type Ctxt<'infercx, 'tcx, DB> = InferCtxt<'infercx, 'tcx, DB>
    where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx;

    type ConsumeResult = Consume<Range<OwnershipSig>>;

    fn interpret_consume<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume<SSAIdx>,
    ) -> Self::ConsumeResult
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        let base = place.local;
        let mut base_ty = body.local_decls[base].ty;
        let base_offset = infer_cx.crate_ctxt.ty_ptr_count(base_ty) as u32;

        let Range {
            start: old_start,
            end: old_end,
        } = infer_cx.local_sig[base][consume.r#use].clone();

        assert_eq!(base_offset, old_end.as_u32() - old_start.as_u32());

        let mut proj_start_offset = 0;

        for projection_elem in place.projection {
            match projection_elem {
                // do not track pointers behind dereferences for now
                ProjectionElem::Deref => unreachable!("indirect place is mistakenly consumed"),
                ProjectionElem::Field(field, ty) => {
                    let TyKind::Adt(adt_def, _) = base_ty.kind() else { unreachable!() };
                    let Some(field_offsets) = infer_cx
                        .crate_ctxt
                        .struct_topology()
                        .field_offsets(&adt_def.did()) else { unreachable!() };
                    proj_start_offset += field_offsets[field.index()];
                    base_ty = ty;
                }
                // [ty] is equivalent to ty
                ProjectionElem::Index(_) => base_ty = base_ty.builtin_index().unwrap(),
                ProjectionElem::ConstantIndex { .. } => {
                    unreachable!("unexpected constant index {:?}", place)
                }
                ProjectionElem::Subslice { .. } => {
                    unreachable!("unexpected subslicing {:?}", place)
                }
                ProjectionElem::Downcast(..) => unreachable!("unexpected downcasting {:?}", place),
            }
        }

        let Range {
            start: new_start,
            end: new_end,
        } = infer_cx.new_sigs(base_offset);
        // let new_start = infer_cx.next;
        // let new_end = new_start + base_offset;
        assert_eq!(
            infer_cx.local_sig[base].push(new_start..new_end),
            consume.def
        );

        for (old, new) in
            (old_start..old_start + proj_start_offset).zip(new_start..new_start + proj_start_offset)
        {
            infer_cx.database.push_equal::<super::Debug>((), old, new);
        }

        let proj_end_offset = proj_start_offset + infer_cx.crate_ctxt.ty_ptr_count(base_ty);

        for (old, new) in
            (old_start + proj_end_offset..old_end).zip(new_start + proj_end_offset..new_end)
        {
            infer_cx.database.push_equal::<super::Debug>((), old, new);
        }

        Consume {
            r#use: old_start + proj_start_offset..old_start + proj_end_offset,
            def: new_start + proj_start_offset..new_start + proj_end_offset,
        }
    }

    fn transfer_ownership<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        lhs_result: Self::ConsumeResult,
        rhs_result: Self::ConsumeResult,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for (lhs_use, lhs_def, rhs_use, rhs_def) in izip!(
            lhs_result.r#use,
            lhs_result.def,
            rhs_result.r#use,
            rhs_result.def
        ) {
            infer_cx
                .database
                .push_assume::<super::Debug>((), lhs_use, false);
            infer_cx
                .database
                .push_linear::<super::Debug>((), lhs_def, rhs_def, rhs_use)
        }
    }

    fn finalise<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        local: Local,
        r#use: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for sig in infer_cx.local_sig[local][r#use].clone() {
            infer_cx
                .database
                .push_assume::<super::Debug>((), sig, false)
        }
    }
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

    pub(crate) fn go<M: Mode + 'rn, DB: Database + 'rn>(
        &mut self,
        mut infer_cx: impl BorrowMut<M::Ctxt<'rn, 'tcx, DB>>,
    ) {
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
                    self.go_basic_block::<M, _>(
                        infer_cx.borrow_mut(),
                        bb,
                        &self.body.basic_blocks()[bb],
                    );
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

    fn go_basic_block<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        bb: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) {
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
            self.go_statement::<M, _>(infer_cx.borrow_mut(), statement, location);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_terminator::<M, _>(infer_cx, terminator, location);
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

    fn go_statement<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        statement: &Statement<'tcx>,
        location: Location,
    ) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.go_assign::<M, _>(infer_cx, place, rvalue, location)
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
    fn go_terminator<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        terminator: &Terminator<'tcx>,
        location: Location,
    ) {
        match &terminator.kind {
            TerminatorKind::Call {
                args, destination, ..
            } => {
                tracing::debug!("processing terminator {:?}", terminator.kind);
                assert!(destination.as_local().is_some());
                // let destination = destination.as_local().unwrap();
                let _ = self
                    .state
                    .try_consume_at(destination.local, location)
                    .map(|consume| M::interpret_consume(infer_cx, self.body, destination, consume));
                for arg in args {
                    match arg {
                        Operand::Move(arg) | Operand::Copy(arg) => {
                            let _ = self
                                .state
                                .try_consume_at(arg.local, location)
                                .map(|consume| {
                                    M::interpret_consume(infer_cx, self.body, arg, consume)
                                });
                        }
                        _ => continue,
                    }
                }
            }
            TerminatorKind::Return => {
                tracing::debug!("processing terminator {:?}", terminator.kind);
                let return_place = RETURN_PLACE;
                let _ = self
                    .state
                    .try_consume_at(return_place, location)
                    .map(|consume| {
                        M::interpret_consume(infer_cx, self.body, &Place::return_place(), consume)
                    });
                // finalise! note that return place is consumed before, so it needs
                // to be finalised as well
                for local in self.state.consume_chain.to_finalise() {
                    let r#use = self.state.name_state.get_name(local);
                    M::finalise(infer_cx, local, r#use);
                }
            }
            _ => {}
        }
    }

    fn go_assign<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);

        let lhs = place;
        let rhs = rvalue;

        let lhs_consume = self.state.try_consume_at(lhs.local, location);
        //.map(|consume| M::consume_place(infer_cx, self.body, place, consume));

        match rhs {
            Rvalue::Use(Operand::Constant(_)) => {
                // assert!(lhs_consume.is_none(), "pointers cannot be mir constants: {:?}", constant)
                // if lhs_consume.is_some() {
                //     tracing::debug!("constant pointer rvalue {:?}", rhs)
                // }
                if let Some(lhs_consume) = lhs_consume {
                    let _ = M::interpret_consume(infer_cx, self.body, lhs, lhs_consume);
                    tracing::debug!("constant pointer rvalue {:?}", rhs)
                }
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
                // even tho lhs is a pointer, it does not necessarily have an entry!
                // this is because we limit the nested level of pointers
                if let Some(lhs_consume) = lhs_consume {
                    let _ = M::interpret_consume(infer_cx, self.body, lhs, lhs_consume);
                }
                // lhs_consume.map(|lhs_consume| M::consume_place(infer_cx, self.body, lhs, lhs_consume));
                tracing::debug!("untrusted pointer source: raw address {:?}", operand)
            }

            Rvalue::Cast(
                CastKind::PointerExposeAddress,
                Operand::Copy(rhs) | Operand::Move(rhs),
                _,
            ) => {
                let rhs_consume = self.state.consume_at(rhs.local, location);
                let _ = M::interpret_consume(infer_cx, self.body, rhs, rhs_consume);
                tracing::debug!("untrusted pointer sink: address {:?}", lhs);
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
                let lhs_consume = lhs_consume
                    .map(|consume| M::interpret_consume(infer_cx, self.body, lhs, consume));
                let rhs_consume = rhs_consume
                    .map(|consume| M::interpret_consume(infer_cx, self.body, rhs, consume));

                lhs_consume.and_then(|lhs_result| {
                    rhs_consume
                        .map(|rhs_result| M::transfer_ownership(infer_cx, lhs_result, rhs_result))
                });

                // if let (Some(lhs_consume), Some(rhs_consume)) = (lhs_consume, rhs_consume) {
                //     M::transfer_ownership(infer_cx, lhs_consume, rhs_consume)
                // }
            }

            Rvalue::CopyForDeref(_) => {
                tracing::debug!("deref_copy is ignored")
            }

            Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                let _ = lhs
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
