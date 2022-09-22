use std::{borrow::BorrowMut, ops::Range};

use itertools::izip;
use orc_common::data_structure::assoc::AssocExt;
use rustc_data_structures::graph::WithSuccessors;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        AggregateKind, BasicBlock, BasicBlockData, Body, CastKind, Local, Location,
        NonDivergingIntrinsic, Operand, Place, PlaceElem, ProjectionElem, Rvalue, Statement,
        StatementKind, Terminator, TerminatorKind, RETURN_PLACE,
    },
    ty::Ty,
};
use rustc_type_ir::TyKind;
use TyKind::FnDef;

use crate::{
    analysis::{
        constraint::{
            generate_signatures_for_local, infer::handle_call::HandleCall, Database, Gen,
            OwnershipSig,
        },
        consume::{Consume, RichLocation},
        join_points::PhiNode,
        state::{SSAIdx, SSAState},
        AnalysisKind, FnResult, FnSig,
    },
    ptr::Measurable,
    struct_topology::HasStructTopology,
    CrateCtxt,
};

pub mod handle_call;

pub type LocalSig = Range<OwnershipSig>;
pub type FnBodySig<LocalSig> = IndexVec<Local, IndexVec<SSAIdx, LocalSig>>;

pub struct FnSummary {
    pub fn_body_sig: FnBodySig<LocalSig>,
    pub ssa_state: SSAState,
}

impl FnSummary {
    pub fn new<'analysis, DB: Database, Kind: AnalysisKind<'analysis>>(
        rn: Renamer,
        infer_cx: InferCtxt<'analysis, '_, DB, Kind>,
    ) -> Self {
        FnSummary {
            fn_body_sig: infer_cx.fn_body_sig,
            ssa_state: rn.state,
        }
    }
}

impl<'a> FnResult<'a> for FnSummary {
    type LocalResult = LocalSig;

    type LocationResults = impl Iterator<Item = (Local, Consume<LocalSig>)> + 'a;

    #[inline]
    fn local_result(&self, local: Local, location: Location) -> Option<Consume<LocalSig>> {
        let consume_chain = &self.ssa_state.consume_chain;
        let consumes = consume_chain.of_location(location);
        let consume = consumes.get_by_key(&local)?;
        Some(consume.map(|ssa_idx| self.fn_body_sig[local][ssa_idx].clone()))
    }

    #[inline]
    fn location_result(&'a self, location: Location) -> Self::LocationResults {
        let consume_chain = &self.ssa_state.consume_chain;
        let consumes = consume_chain.of_location(location);
        consumes.iter().map(|(local, consume)| {
            (
                *local,
                consume.map(|ssa_idx| self.fn_body_sig[*local][ssa_idx].clone()),
            )
        })
    }
}

pub struct InferCtxt<'infercx, 'tcx, DB, Analysis>
where
    'tcx: 'infercx,
    DB: Database,
    Analysis: AnalysisKind<'infercx>,
{
    inter_ctxt: Analysis::InterCtxt,
    database: &'infercx mut DB,
    gen: &'infercx mut Gen,
    crate_ctxt: &'infercx CrateCtxt<'tcx>,
    pub fn_body_sig: FnBodySig<LocalSig>,
    // deref_copys: Consume<<Kind as InferMode<'infercx, 'tcx, DB>>::Interpretation>,
    // threshold: Threshold,
}

impl<'infercx, 'tcx, DB, Analysis> InferCtxt<'infercx, 'tcx, DB, Analysis>
where
    'tcx: 'infercx,
    DB: Database,
    Analysis: AnalysisKind<'infercx>,
{
    pub fn new(
        crate_ctxt: &'infercx CrateCtxt<'tcx>,
        body: &Body<'tcx>,
        database: &'infercx mut DB,
        gen: &'infercx mut Gen,
        inter_ctxt: Analysis::InterCtxt,
    ) -> Self {
        let mut fn_body_sig = IndexVec::with_capacity(body.local_decls.len());

        for local_decl in body.local_decls.iter() {
            if let Some(sigs) = generate_signatures_for_local(local_decl, gen, database, crate_ctxt)
            {
                fn_body_sig.push(IndexVec::from_raw(vec![sigs]));
            } else {
                fn_body_sig.push(IndexVec::default());
            }
        }

        <Analysis as HandleCall<_>>::handle_inputs(
            // crate_ctxt,
            &inter_ctxt,
            database,
            body.source.def_id(),
            fn_body_sig
                .iter()
                .skip(1)
                .take(body.arg_count)
                .map(|vec| vec.raw.first().cloned()),
        );

        InferCtxt {
            inter_ctxt,
            database,
            gen,
            crate_ctxt,
            fn_body_sig,
            // deref_copys: Consume {
            //     r#use: Range {
            //         start: OwnershipSig::INVALID,
            //         end: OwnershipSig::INVALID,
            //     },
            //     def: Range {
            //         start: OwnershipSig::INVALID,
            //         end: OwnershipSig::INVALID,
            //     },
            // },
        }
    }

    pub fn new_sigs(&mut self, size: u32) -> Range<OwnershipSig> {
        self.database.new_vars(self.gen.new_sigs(size))
    }

    fn project_deeper(
        base: Consume<<Analysis as InferMode<'infercx, 'tcx, DB>>::LocalSig>,
        mut base_ty: Ty<'tcx>,
        projection: &[PlaceElem<'tcx>],
        infer_cx: &mut Self,
    ) -> Consume<<Analysis as InferMode<'infercx, 'tcx, DB>>::LocalSig> {
        let mut proj_start_offset = 0;

        for projection_elem in projection {
            match projection_elem {
                // do not track pointers behind dereferences for now
                ProjectionElem::Deref => {
                    // No need to set up threshold. Consumption of indirect places are processed
                    // only if definitions contain them, which happen in phases where threshold.
                    // Furthermore, mir places contain only at most one indirection.
                    proj_start_offset += 1;
                    base_ty = base_ty.builtin_deref(true).unwrap().ty;
                }
                ProjectionElem::Field(field, ty) => {
                    let TyKind::Adt(adt_def, _) = base_ty.kind() else { unreachable!() };
                    let Some(field_offsets) = infer_cx
                        .crate_ctxt
                        .struct_topology()
                        .field_offsets(&adt_def.did()) else { unreachable!() };
                    proj_start_offset += field_offsets[field.index()];
                    base_ty = *ty;
                }
                // [ty] is equivalent to ty
                ProjectionElem::Index(_) => base_ty = base_ty.builtin_index().unwrap(),
                ProjectionElem::ConstantIndex { .. } => {
                    unreachable!("unexpected constant index");
                }
                ProjectionElem::Subslice { .. } => {
                    unreachable!("unexpected subslicing")
                }
                ProjectionElem::Downcast(..) => unreachable!("unexpected downcasting"),
            }
        }

        for (pre, post) in (base.r#use.start..base.r#use.start + proj_start_offset)
            .zip(base.def.start..base.def.start + proj_start_offset)
        {
            infer_cx.database.push_equal::<super::Debug>((), pre, post);
        }

        let proj_end_offset = proj_start_offset + infer_cx.crate_ctxt.measure(base_ty);

        assert!(base.r#use.start + proj_end_offset <= base.r#use.end);

        for (pre, post) in (base.r#use.start + proj_end_offset..base.r#use.end)
            .zip(base.def.start + proj_end_offset..base.def.end)
        {
            infer_cx.database.push_equal::<super::Debug>((), pre, post);
        }

        Consume {
            r#use: base.r#use.start + proj_start_offset..base.r#use.start + proj_end_offset,
            def: base.def.start + proj_start_offset..base.def.start + proj_end_offset,
        }
    }
}

pub trait InferMode<'infercx, 'tcx: 'infercx, DB: Database> {
    type Ctxt;

    type LocalSig: Clone + std::fmt::Debug;

    type FnSig<T>: std::fmt::Debug
    where
        T: std::fmt::Debug;

    fn transform_fn_sig(
        fn_sig: impl Iterator<Item = Option<Consume<Self::LocalSig>>>,
    ) -> Self::FnSig<Option<Consume<Self::LocalSig>>>;

    fn define_phi_node(infer_cx: &mut Self::Ctxt, local: Local, ty: Ty<'tcx>, def: SSAIdx)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn join_phi_nodes<'a>(
        infer_cx: &'a mut Self::Ctxt,
        phi_nodes: impl Iterator<Item = (Local, &'a mut PhiNode)>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn interpret_consume(
        infer_cx: &mut Self::Ctxt,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume<SSAIdx>,
    ) -> Consume<Self::LocalSig>
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn transfer(
        infer_cx: &mut Self::Ctxt,
        lhs_result: Consume<Self::LocalSig>,
        rhs_result: Consume<Self::LocalSig>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn borrow(infer_cx: &mut Self::Ctxt, consume: Consume<Self::LocalSig>)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, consume.r#use, false);
        Self::assume(infer_cx, consume.def, false);
    }

    // /// Lend by the owner, not re-borrow
    // fn lend(infer_cx: &mut Self::Ctxt, consume: Consume<Self::Interpretation>)
    // where
    //     'tcx: 'infercx,
    //     DB: Database + 'infercx,
    // {
    //     Self::assume(infer_cx, consume.r#use, true);
    //     Self::assume(infer_cx, consume.def, true);
    // }

    fn source(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, result.r#use, false);
        Self::assume(infer_cx, result.def, true)
    }

    fn sink(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, result.r#use, true);
        Self::assume(infer_cx, result.def, false)
    }

    /// Impose no constraint on a definition. Constraints are still emitted
    /// because the old value of lhs must be non-owning
    fn unknown_source(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        Self::assume(infer_cx, result.r#use, false)
    }

    fn unknown_sink(_: &mut Self::Ctxt, _: Consume<Self::LocalSig>)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn assume(infer_cx: &mut Self::Ctxt, result: Self::LocalSig, value: bool)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn finalise(infer_cx: &mut Self::Ctxt, local: Local, r#use: SSAIdx)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn handle_call(
        infer_cx: &mut Self::Ctxt,
        caller: Self::FnSig<Option<Consume<Self::LocalSig>>>,
        func: &Operand,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn handle_output(infer_cx: &mut Self::Ctxt, ssa_idx: Option<SSAIdx>, r#fn: DefId)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;
}
#[derive(Debug)]
pub enum Pure {}
impl<'infercx, 'tcx: 'infercx, DB: Database> InferMode<'infercx, 'tcx, DB> for Pure {
    type Ctxt = ();

    type LocalSig = ();

    type FnSig<T> = () where T: std::fmt::Debug;

    fn transform_fn_sig(fn_sig: impl Iterator<Item = Option<Consume<Self::LocalSig>>>) {
        for _ in fn_sig {}
    }

    fn define_phi_node((): &mut Self::Ctxt, _: Local, _: Ty, _: SSAIdx)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn join_phi_nodes<'a>((): &'a mut Self::Ctxt, _: impl Iterator<Item = (Local, &'a mut PhiNode)>)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    #[inline]
    fn interpret_consume(
        (): &mut Self::Ctxt,
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
    fn transfer(
        (): &mut Self::Ctxt,
        Consume { r#use: (), def: () }: Consume<Self::LocalSig>,
        Consume { r#use: (), def: () }: Consume<Self::LocalSig>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn unknown_sink((): &mut Self::Ctxt, _: Consume<Self::LocalSig>)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn assume((): &mut Self::Ctxt, (): Self::LocalSig, _: bool)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn finalise((): &mut Self::Ctxt, _: Local, _: SSAIdx)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn handle_call((): &mut Self::Ctxt, (): (), _: &Operand)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }

    fn handle_output((): &mut Self::Ctxt, _: Option<SSAIdx>, _: DefId)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }
}

// pub trait WithCtxt: AnalysisKind {}
// impl<K: AnalysisKind> WithCtxt for K {}

impl<'infercx, 'tcx, DB, Analysis> InferMode<'infercx, 'tcx, DB> for Analysis
where
    'tcx: 'infercx,
    DB: Database + 'infercx,
    Analysis: AnalysisKind<'infercx>,
{
    type Ctxt = InferCtxt<'infercx, 'tcx, DB, Analysis>;

    type LocalSig = LocalSig;

    type FnSig<T> = FnSig<T> where T: std::fmt::Debug;

    #[inline]
    fn transform_fn_sig(
        mut fn_sig: impl Iterator<Item = Option<Consume<Self::LocalSig>>>,
    ) -> Self::FnSig<Option<Consume<Self::LocalSig>>> {
        let destination = fn_sig.next().unwrap();
        let args = fn_sig.collect();
        FnSig::new(destination, args)
    }

    #[inline]
    fn define_phi_node(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>,
        local: Local,
        ty: Ty<'tcx>,
        def: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        let measure = infer_cx.crate_ctxt.measure(ty);
        let sigs = infer_cx.new_sigs(measure);
        assert_eq!(def, infer_cx.fn_body_sig[local].push(sigs));
    }

    fn join_phi_nodes<'a>(
        infer_cx: &'a mut InferCtxt<'infercx, 'tcx, DB, Analysis>,
        phi_nodes: impl Iterator<Item = (Local, &'a mut PhiNode)>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for (local, phi_node) in phi_nodes {
            // This is not necessary if phi nodes have been prune
            phi_node.rhs.sort();
            phi_node.rhs.dedup();
            let lhs = phi_node.lhs;
            for rhs in phi_node.rhs.iter().copied() {
                if lhs == rhs {
                    continue;
                }
                let lhs_sigs = infer_cx.fn_body_sig[local][lhs].clone();
                let rhs_sigs = infer_cx.fn_body_sig[local][rhs].clone();
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
    fn interpret_consume(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume<SSAIdx>,
    ) -> Consume<Self::LocalSig>
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        tracing::debug!("interpretting consume for {:?} with {:?}", place, consume);
        let base = place.local;
        let base_ty = body.local_decls[base].ty;
        let base_offset = infer_cx.crate_ctxt.measure(base_ty);

        let r#use = infer_cx.fn_body_sig[base][consume.r#use].clone();
        let def = infer_cx.new_sigs(base_offset);
        assert_eq!(base_offset, r#use.end.as_u32() - r#use.start.as_u32());
        assert_eq!(
            infer_cx.fn_body_sig[base].push(def.start..def.end),
            consume.def
        );

        let base = Consume { r#use, def };

        InferCtxt::project_deeper(base, base_ty, place.projection, infer_cx)
    }

    fn transfer(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>,
        lhs_result: Consume<Self::LocalSig>,
        rhs_result: Consume<Self::LocalSig>,
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

    #[inline]
    fn unknown_sink(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>,
        consume: Consume<Self::LocalSig>,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for (r#use, def) in consume.r#use.zip(consume.def) {
            infer_cx
                .database
                .push_less_equal::<super::Debug>((), def, r#use);
        }
    }

    #[inline]
    fn assume(infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>, result: Self::LocalSig, value: bool)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for sig in result {
            infer_cx
                .database
                .push_assume::<super::Debug>((), sig, value)
        }
    }

    #[inline]
    fn finalise(infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>, local: Local, r#use: SSAIdx)
    where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        for sig in infer_cx.fn_body_sig[local][r#use].clone() {
            infer_cx
                .database
                .push_assume::<super::Debug>((), sig, false)
        }
    }

    fn handle_call(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>,
        caller: Self::FnSig<Option<Consume<Self::LocalSig>>>,
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
                    rustc_hir::Node::Item(_) => {
                        <Analysis as HandleCall<_>>::handle_call(infer_cx, &caller, callee)
                    }
                    // extern
                    rustc_hir::Node::ForeignItem(foreign_item) => {
                        infer_cx.handle_libc_call(&caller, foreign_item.ident)
                    }
                    // in libxml2.rust/src/xmlschemastypes.rs/{} impl_xmlSchemaValDate/set_mon
                    rustc_hir::Node::ImplItem(_) => { /* TODO */ }
                    _ => unreachable!(),
                }
            } else {
                // library
                infer_cx.handle_library_call(&caller, callee)
            }
        } else {
            // closure or fn ptr
            /* TODO */
        }
    }

    fn handle_output(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Analysis>,
        ssa_idx: Option<SSAIdx>,
        r#fn: DefId,
    ) where
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        let output = ssa_idx.map(|ssa_idx| infer_cx.fn_body_sig[RETURN_PLACE][ssa_idx].clone());
        <Analysis as HandleCall<_>>::handle_output(infer_cx, r#fn, output)
    }
}

pub struct Renamer<'rn, 'tcx> {
    body: &'rn Body<'tcx>,
    pub state: SSAState,
}

impl<'rn, 'tcx: 'rn> Renamer<'rn, 'tcx> {
    pub fn new(body: &'rn Body<'tcx>, state: SSAState) -> Self {
        Renamer { body, state }
    }

    pub fn go<Infer: InferMode<'rn, 'tcx, DB> + 'rn, DB: Database + 'rn>(
        &mut self,
        mut infer_cx: impl BorrowMut<Infer::Ctxt>,
    ) {
        tracing::debug!("Renaming {:?}", self.body.source.def_id());

        let dominators = self.body.basic_blocks.dominators();
        let mut children = IndexVec::from_elem(vec![], &self.body.basic_blocks);
        let mut root = BasicBlock::from_u32(0);
        self.body.basic_blocks.indices().for_each(|bb| {
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
                    self.go_basic_block::<Infer, _>(
                        infer_cx.borrow_mut(),
                        bb,
                        &self.body.basic_blocks[bb],
                    );
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for &(local, _) in self
                        .state
                        .consume_chain
                        .of_block(bb)
                        .iter()
                        .flatten()
                        .filter(|(_, consume)| !consume.is_use())
                    {
                        let ssa_idx = self.state.name_state.pop(local);
                        tracing::debug!("popping at {:?}: {:?}~{:?}", bb, local, ssa_idx);
                    }
                }
            }
        }

        Infer::join_phi_nodes(
            infer_cx.borrow_mut(),
            self.state.join_points.phi_nodes_mut(),
        );
    }

    fn go_basic_block<Infer: InferMode<'rn, 'tcx, DB>, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
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
            Infer::define_phi_node(infer_cx, local, self.body.local_decls[local].ty, ssa_idx);
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_statement::<Infer, _>(infer_cx.borrow_mut(), statement, location);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_terminator::<Infer, _>(infer_cx, terminator, location);
        }

        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.state.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.state.name_state.get_name(local);
                phi_node.rhs.push(ssa_idx);
                tracing::debug!("using {:?} at Phi({:?}), use: {:?}", local, succ, ssa_idx)
            }
        }
    }

    fn go_statement<Infer: InferMode<'rn, 'tcx, DB>, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
        statement: &Statement<'tcx>,
        location: Location,
    ) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.go_assign::<Infer, _>(infer_cx, place, rvalue, location)
            }
            StatementKind::SetDiscriminant { .. } => {
                tracing::debug!("ignoring SetDiscriminant statement {:?}", statement)
            }
            StatementKind::Deinit(..) => {
                tracing::debug!("ignoring Deinit statement {:?}", statement)
            }
            StatementKind::Intrinsic(box intrinsic) => {
                assert!(matches!(intrinsic, NonDivergingIntrinsic::Assume(..)))
            }
            StatementKind::AscribeUserType(_, _)
            | StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_)
            | StatementKind::Retag(_, _)
            | StatementKind::FakeRead(_)
            | StatementKind::Coverage(_)
            // | StatementKind::CopyNonOverlapping(_)
            | StatementKind::Nop => {
                unreachable!("statement {:?} is not assumed to appear", statement)
            }
        }
    }

    fn go_terminator<Infer: InferMode<'rn, 'tcx, DB>, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
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

                let fn_sig = std::iter::once(Some(destination))
                    .chain(args.iter().map(|arg| match arg {
                        Operand::Move(arg) | Operand::Copy(arg) => Some(arg),
                        Operand::Constant(..) => None,
                    }))
                    .map(|place| {
                        place.and_then(|place| {
                            let consume = self.state.try_consume_at(place.local, location);
                            consume.map(|consume| {
                                Infer::interpret_consume(infer_cx, self.body, place, consume)
                            })
                        })
                    });

                let fn_sig = Infer::transform_fn_sig(fn_sig);

                // println!("{:?}", self.body.source_info(location).span);
                // println!("{:?}", fn_sig);
                Infer::handle_call(infer_cx, fn_sig, func);
            }
            TerminatorKind::Return => {
                tracing::debug!("processing terminator {:?}", terminator.kind);

                Infer::handle_output(
                    infer_cx,
                    self.state.name_state.try_get_name(RETURN_PLACE),
                    self.body.source.def_id(),
                );

                // finalise!
                // note that return place should not be finalised!!
                for local in self.state.consume_chain.to_finalise() {
                    let r#use = self.state.name_state.get_name(local);
                    tracing::debug!("finalising {:?}~{:?}", local, r#use);
                    Infer::finalise(infer_cx, local, r#use);
                }
            }
            _ => {}
        }
    }

    fn go_assign<Infer: InferMode<'rn, 'tcx, DB>, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);

        let lhs = place;
        let rhs = rvalue;

        match rhs {
            Rvalue::Use(Operand::Constant(_)) => {
                if let Some(lhs_consume) = self.state.try_consume_at(lhs.local, location) {
                    let lhs_consume =
                        Infer::interpret_consume(infer_cx, self.body, lhs, lhs_consume);
                    Infer::unknown_source(infer_cx, lhs_consume);
                    tracing::debug!("constant pointer rvalue {:?}", rhs)
                }
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
                // even tho lhs is a pointer, it does not necessarily have an entry!
                // this is because we limit the nested level of pointers
                if let Some(lhs_consume) = self.state.try_consume_at(lhs.local, location) {
                    let lhs_consume =
                        Infer::interpret_consume(infer_cx, self.body, lhs, lhs_consume);
                    Infer::unknown_source(infer_cx, lhs_consume);
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
                let rhs_consume = Infer::interpret_consume(infer_cx, self.body, rhs, rhs_consume);
                // correctness?
                Infer::unknown_sink(infer_cx, rhs_consume);
                tracing::debug!("untrusted pointer sink: address {:?}", lhs);
            }

            Rvalue::Cast(_, Operand::Constant(box constant), _) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(
                    lhs_consume.is_none(),
                    "TODO: constant pointer {:?}",
                    constant
                )
            }

            Rvalue::Use(Operand::Move(rhs) | Operand::Copy(rhs))
            | Rvalue::Cast(_, Operand::Move(rhs) | Operand::Copy(rhs), _) => {
                let lhs_consume = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| Infer::interpret_consume(infer_cx, self.body, lhs, consume));

                let rhs_consume = self
                    .state
                    .try_consume_at(rhs.local, location)
                    .map(|consume| Infer::interpret_consume(infer_cx, self.body, rhs, consume));

                match (lhs_consume, rhs_consume) {
                    (None, None) => {}
                    (None, Some(rhs_consume)) => Infer::unknown_sink(infer_cx, rhs_consume),
                    (Some(lhs_consume), None) => Infer::unknown_source(infer_cx, lhs_consume),
                    (Some(lhs_consume), Some(rhs_consume)) => {
                        Infer::transfer(infer_cx, lhs_consume, rhs_consume)
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
                let lhs_consume = Infer::interpret_consume(infer_cx, self.body, lhs, lhs_consume);

                /* TODO */

                // correctness???
                Infer::borrow(infer_cx, lhs_consume)
            }

            Rvalue::Repeat(Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                let _ = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| Infer::interpret_consume(infer_cx, self.body, lhs, consume));
                let _ = self
                    .state
                    .try_consume_at(rhs.local, location)
                    .map(|consume| Infer::interpret_consume(infer_cx, self.body, rhs, consume));

                /* TODO */
            }

            Rvalue::Aggregate(box AggregateKind::Array(_), rhs_vec) => {
                let _ = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| Infer::interpret_consume(infer_cx, self.body, lhs, consume));

                for rhs in rhs_vec {
                    let Some(rhs) = rhs.place() else { continue };
                    let _ = self
                        .state
                        .try_consume_at(rhs.local, location)
                        .map(|consume| {
                            Infer::interpret_consume(infer_cx, self.body, &rhs, consume)
                        });
                }

                /* TODO */
            }

            Rvalue::Repeat(..) | Rvalue::Aggregate(..) => {
                // handle cases like _1 = [0 as *mut _; 50] or _1 = [move _12, move _13]

                // TODO note that vars in those rvalues are not counted as
                // definitions nor pure uses. If these are to be taken care
                // of, logic in initial_definition needs to be taken care of
                // as well

                let lhs_consume = self
                    .state
                    .try_consume_at(lhs.local, location)
                    .map(|consume| Infer::interpret_consume(infer_cx, self.body, lhs, consume));

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
                assert!(lhs_consume.is_none());
            }
        }
    }
}
