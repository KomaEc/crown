use std::{borrow::BorrowMut, ops::Range};

use itertools::izip;
use TyKind::FnDef;
// use orc_common::pointer::BorrowMut;
use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        AggregateKind, BasicBlock, BasicBlockData, Body, CastKind, Local, Location, Operand, Place,
        ProjectionElem, Rvalue, Statement, StatementKind, Terminator, TerminatorKind,
    },
    ty::Ty,
};
use rustc_type_ir::TyKind;

use crate::{
    analysis::{
        body_ext::DominanceFrontier,
        constraint::{CadicalDatabase, Database, OwnershipSig},
        def::{Consume, Definitions, RichLocation},
        join_points::PhiNode,
        state::{SSAIdx, SSAState},
        FnSig,
    },
    CrateCtxt,
};

pub(crate) mod model_call;

pub(crate) struct InferCtxt<'infercx, 'tcx, DB = CadicalDatabase> {
    pub(crate) database: DB,
    crate_ctxt: &'infercx CrateCtxt<'tcx>,
    pub(crate) local_sig: IndexVec<Local, IndexVec<SSAIdx, Range<OwnershipSig>>>,
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

    pub(crate) fn all_sigs(&self) -> Range<OwnershipSig> {
        OwnershipSig::MIN..self.next
    }
}

pub(crate) trait Mode {
    type Ctxt<'infercx, 'tcx, DB>
    where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx;

    type Interpretation: Clone + std::fmt::Debug;

    type FnSig<T>;

    fn transform_fn_sig(
        func_sig: impl Iterator<Item = Option<Consume<Self::Interpretation>>>,
    ) -> Self::FnSig<Option<Consume<Self::Interpretation>>>;

    // type ConsumeInterpretation;

    fn define_phi_node<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        local: Local,
        ty: Ty<'tcx>,
        def: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn join_phi_nodes<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        phi_nodes: impl Iterator<Item = (Local, &'infercx mut PhiNode)>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn interpret_consume<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume<SSAIdx>,
    ) -> Consume<Self::Interpretation>
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn transfer<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        lhs_result: Consume<Self::Interpretation>,
        rhs_result: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn borrow<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        consume: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, consume.r#use, false);
        Self::assume(infer_cx, consume.def, false);
    }

    // fn join<'infercx, 'tcx, DB>(
    //     infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
    //     left: Self::Interpretation,
    //     right: Self::Interpretation,
    // ) where
    //     'tcx: 'infercx,
    //     DB: Database + 'infercx;

    fn source<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        result: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, result.r#use, false);
        Self::assume(infer_cx, result.def, true)
    }

    fn sink<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        result: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, result.r#use, true);
        Self::assume(infer_cx, result.def, false)
    }

    /// Special treatment to null assignment
    fn null_assignment<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    /// Impose no constraint on a definition. Constraints are still emitted
    /// because the old value of lhs must be non-owning
    fn unknown_source<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        result: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, result.r#use, false)
    }

    fn unknown_sink<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn assume<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        result: Self::Interpretation,
        value: bool,
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

    // Design: Use Vec<Consume<Self::Interpretation>>?
    // Note that Vec<Consume<()>> won't allocate memories
    fn model_call<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        func_sig: Self::FnSig<Option<Consume<Self::Interpretation>>>,
        // func_args: Vec<Consume<Self::Interpretation>>,
        // func_sig: impl Iterator<Item = Option<Consume<Self::Interpretation>>>,
        func: &Operand,
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

    type Interpretation = ();

    type FnSig<T> = ();

    fn transform_fn_sig(func_sig: impl Iterator<Item = Option<Consume<Self::Interpretation>>>) {
        for _ in func_sig {}
    }

    fn define_phi_node<'infercx, 'tcx, DB>(
        (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: Local,
        _: Ty,
        _: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn join_phi_nodes<'infercx, 'tcx, DB>(
        (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: impl Iterator<Item = (Local, &'infercx mut PhiNode)>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    #[inline]
    fn interpret_consume<'infercx, 'tcx, DB>(
        (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: &Body<'tcx>,
        _: &Place<'tcx>,
        _: Consume<SSAIdx>,
    ) -> Consume<()>
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Consume { r#use: (), def: () }
    }

    #[inline]
    fn transfer<'infercx, 'tcx, DB>(
        (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
        Consume { r#use: (), def: () }: Consume<Self::Interpretation>,
        Consume { r#use: (), def: () }: Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    // fn join<'infercx, 'tcx, DB>(
    //     (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
    //     (): Self::Interpretation,
    //     (): Self::Interpretation,
    // ) where
    //     'tcx: 'infercx,
    //     DB: Database + 'infercx,
    // {
    // }

    fn assume<'infercx, 'tcx, DB>(
        (): &mut Self::Ctxt<'infercx, 'tcx, DB>,
        (): Self::Interpretation,
        _: bool,
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

    fn model_call<'infercx, 'tcx, DB>((): &mut Self::Ctxt<'infercx, 'tcx, DB>, (): (), _: &Operand)
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

    type Interpretation = Range<OwnershipSig>;

    type FnSig<T> = FnSig<T>;

    fn transform_fn_sig(
        mut func_sig: impl Iterator<Item = Option<Consume<Self::Interpretation>>>,
    ) -> Self::FnSig<Option<Consume<Self::Interpretation>>> {
        let destination = func_sig.next().unwrap();
        let args = func_sig.collect();
        FnSig::new(destination, args)
    }

    fn define_phi_node<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        local: Local,
        ty: Ty<'tcx>,
        def: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        let offset = infer_cx.crate_ctxt.ty_ptr_count(ty);
        let sigs = infer_cx.new_sigs(offset);
        assert_eq!(def, infer_cx.local_sig[local].push(sigs));
    }

    fn join_phi_nodes<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        phi_nodes: impl Iterator<Item = (Local, &'infercx mut PhiNode)>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for (local, phi_node) in phi_nodes {
            // !!!!!!!!
            // This is not necessary if phi nodes have been prune!
            phi_node.rhs.sort();
            phi_node.rhs.dedup();
            let lhs = phi_node.lhs;
            for rhs in phi_node.rhs.iter().copied() {
                if lhs == rhs {
                    continue;
                }
                let lhs_sigs = infer_cx.local_sig[local][lhs].clone();
                let rhs_sigs = infer_cx.local_sig[local][rhs].clone();
                for (lhs_sig, rhs_sig) in lhs_sigs.zip(rhs_sigs) {
                    infer_cx
                        .database
                        .push_equal::<super::Debug>((), lhs_sig, rhs_sig)
                }
            }
        }
    }

    /// Note: we may be very careful on cast when dealing with multi-level pointers.
    /// note that pointer to complex structures may be cast to a pointer to unit in
    /// order to perform free
    fn interpret_consume<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume<SSAIdx>,
    ) -> Consume<Self::Interpretation>
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        tracing::debug!("interpretting consume for {:?} with {:?}", place, consume);
        let base = place.local;
        let mut base_ty = body.local_decls[base].ty;
        let base_offset = infer_cx.crate_ctxt.ty_ptr_count(base_ty);

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

    fn transfer<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        lhs_result: Consume<Self::Interpretation>,
        rhs_result: Consume<Self::Interpretation>,
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

    // fn join<'infercx, 'tcx, DB>(
    //     infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
    //     left: Self::Interpretation,
    //     right: Self::Interpretation,
    // ) where
    //     'tcx: 'infercx,
    //     DB: Database + 'infercx,
    // {
    //     for (l, r) in left.zip(right) {
    //         infer_cx.database.push_equal::<super::Debug>((), l, r)
    //     }
    // }

    fn assume<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        result: Self::Interpretation,
        value: bool,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for sig in result {
            infer_cx
                .database
                .push_assume::<super::Debug>((), sig, value)
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

    fn model_call<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        fn_sig: Self::FnSig<Option<Consume<Self::Interpretation>>>,
        func: &Operand,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        if let Some(func) = func.constant() {
            let ty = func.ty();
            let &FnDef(callee, _) = ty.kind() else { unreachable!() };
            if let Some(local_did) = callee.as_local() {
                match infer_cx
                    .crate_ctxt
                    .tcx
                    .hir()
                    .find_by_def_id(local_did)
                    .unwrap()
                {
                    // this crate
                    rustc_hir::Node::Item(_) => { /* TODO */ }
                    // extern
                    rustc_hir::Node::ForeignItem(foreign_item) => {
                        infer_cx.model_libc_call(&fn_sig, foreign_item.ident)
                    }
                    // in libxml2.rust/src/xmlschemastypes.rs/{} impl_xmlSchemaValDate/set_mon
                    rustc_hir::Node::ImplItem(_) => { /* TODO */ }
                    _ => unreachable!(),
                }
            } else {
                // library
                infer_cx.model_library_call(&fn_sig, callee)
            }
        } else {
            // closure or fn ptr
            /* TODO */
        }
    }
}

// /// The kind of temporary that is ensured local or Special, and is not
// /// renamed in the inference.
// pub(crate) enum IgnoredTemporaryKind {
//     DerefCopy,
//     ArgFree,
//     DestMalloc,
// }

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
        &'rn mut self,
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

        M::join_phi_nodes(
            infer_cx.borrow_mut(),
            self.state.join_points.phi_nodes_mut(),
        );
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
            tracing::debug!("defining {:?} at Phi({:?}), def: {:?}", local, bb, ssa_idx);
            M::define_phi_node(infer_cx, local, self.body.local_decls[local].ty, ssa_idx);
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
                func,
                args,
                destination,
                ..
            } => {
                tracing::debug!("processing terminator {:?}", terminator.kind);

                // hmmm we can't do this. args do not necessarily match sigs. args may contain constants!!

                let func_sig = std::iter::once(Some(destination))
                    .chain(args.iter().map(|arg| match arg {
                        Operand::Move(arg) | Operand::Copy(arg) => Some(arg),
                        Operand::Constant(..) => None,
                    }))
                    .map(|place| {
                        place.and_then(|place| {
                            let consume = self.state.try_consume_at(place.local, location);
                            consume.map(|consume| {
                                M::interpret_consume(infer_cx, self.body, place, consume)
                            })
                        })
                    });

                let func_sig = M::transform_fn_sig(func_sig);

                M::model_call(infer_cx, func_sig, func);
            }
            TerminatorKind::Return => {
                tracing::debug!("processing terminator {:?}", terminator.kind);

                /* TODO */
                //
                // maybe not consuming return place?????
                // comment out visit_terminator in def.rs then we're done

                /* TODO */

                // finalise!
                // note that return place should not be finalised!!
                for local in self.state.consume_chain.to_finalise() {
                    let r#use = self.state.name_state.get_name(local);
                    tracing::debug!("finalising {:?}~{:?}", local, r#use);
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

        //.map(|consume| M::consume_place(infer_cx, self.body, place, consume));

        match rhs {
            Rvalue::Use(Operand::Constant(_))
            | Rvalue::Cast(CastKind::PointerFromExposedAddress, Operand::Constant(_), _) => {
                if let Some(lhs_consume) = self.state.try_consume_at(lhs.local, location) {
                    let lhs_consume = M::interpret_consume(infer_cx, self.body, lhs, lhs_consume);
                    M::null_assignment(infer_cx, lhs_consume);
                    tracing::debug!("constant pointer rvalue {:?}", rhs)
                }
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
                // even tho lhs is a pointer, it does not necessarily have an entry!
                // this is because we limit the nested level of pointers
                if let Some(lhs_consume) = self.state.try_consume_at(lhs.local, location) {
                    let lhs_consume = M::interpret_consume(infer_cx, self.body, lhs, lhs_consume);
                    M::unknown_source(infer_cx, lhs_consume);
                    tracing::debug!("untrusted pointer source: raw address {:?}", operand)
                }
            }

            Rvalue::Cast(
                CastKind::PointerExposeAddress,
                Operand::Copy(rhs) | Operand::Move(rhs),
                _,
            ) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(lhs_consume.is_none());
                let rhs_consume = self.state.consume_at(rhs.local, location);
                let rhs_consume = M::interpret_consume(infer_cx, self.body, rhs, rhs_consume);
                // correctness?
                M::unknown_sink(infer_cx, rhs_consume);
                tracing::debug!("untrusted pointer sink: address {:?}", lhs);
            }

            Rvalue::Cast(_, Operand::Constant(box constant), _) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(
                    lhs_consume.is_none(),
                    "TODO: constant pointer {:?}",
                    constant
                )
                // todo!("constant pointer {:?}, cast_kind: {:?}", constant)
            }

            Rvalue::Use(Operand::Move(rhs) | Operand::Copy(rhs))
            | Rvalue::Cast(_, Operand::Move(rhs) | Operand::Copy(rhs), _) => {
                let lhs_consume = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| M::interpret_consume(infer_cx, self.body, lhs, consume));

                let rhs_consume = self
                    .state
                    .try_consume_at(rhs.local, location)
                    .map(|consume| M::interpret_consume(infer_cx, self.body, rhs, consume));

                match (lhs_consume, rhs_consume) {
                    (None, None) => {}
                    (None, Some(rhs_consume)) => M::unknown_sink(infer_cx, rhs_consume),
                    (Some(lhs_consume), None) => M::unknown_source(infer_cx, lhs_consume),
                    (Some(lhs_consume), Some(rhs_consume)) => {
                        M::transfer(infer_cx, lhs_consume, rhs_consume)
                    }
                }
            }

            Rvalue::CopyForDeref(_) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(lhs_consume.is_none());
                tracing::debug!("deref_copy is ignored")
            }

            Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                let lhs_consume = self.state.consume_at(lhs.local, location);
                let Consume {
                    r#use: lhs_use,
                    def: lhs_def,
                } = M::interpret_consume(infer_cx, self.body, lhs, lhs_consume);

                /* TODO */

                M::assume(infer_cx, lhs_use, false);
                // correctness???
                M::assume(infer_cx, lhs_def, false);
            }

            Rvalue::Repeat(Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                let _ = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| M::interpret_consume(infer_cx, self.body, lhs, consume));
                let _ = self
                    .state
                    .try_consume_at(rhs.local, location)
                    .map(|consume| M::interpret_consume(infer_cx, self.body, rhs, consume));

                /* TODO */
            }

            Rvalue::Aggregate(box AggregateKind::Array(_), rhs_vec) => {
                let _ = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| M::interpret_consume(infer_cx, self.body, lhs, consume));

                for rhs in rhs_vec {
                    let Some(rhs) = rhs.place() else { continue };
                    let _ = self
                        .state
                        .try_consume_at(rhs.local, location)
                        .map(|consume| M::interpret_consume(infer_cx, self.body, &rhs, consume));
                }

                /* TODO */
            }

            Rvalue::Repeat(..) | Rvalue::Aggregate(..) => {
                // handle cases like _1 = [0 as *mut _; 50] or _1 = [move _12, move _13]

                let lhs_consume = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| M::interpret_consume(infer_cx, self.body, lhs, consume));

                if let Some(_) = lhs_consume { /* TODO */ }
            }

            Rvalue::BinaryOp(_, _) | Rvalue::CheckedBinaryOp(_, _) | Rvalue::UnaryOp(_, _) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(lhs_consume.is_none());
            }
            Rvalue::NullaryOp(_, _)
            | Rvalue::Discriminant(_)
            | Rvalue::Len(_)
            | Rvalue::ShallowInitBox(_, _)
            | Rvalue::ThreadLocalRef(_) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                // let lhs_consume = self
                //     .state
                //     .try_consume_at(lhs.local, location)
                //     .map(|consume| M::interpret_consume(infer_cx, self.body, lhs, consume));
                assert!(lhs_consume.is_none());
            }
        }
    }
}
