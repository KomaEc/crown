use std::ops::Range;

use common::data_structure::assoc::AssocExt;
use itertools::izip;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{Body, Local, Location, Operand, Place, PlaceElem, ProjectionElem, RETURN_PLACE},
    ty::{Ty, TyKind},
};
use rustc_type_ir::TyKind::FnDef;

use self::boundary::Boundary;
use super::AnalysisKind;
use crate::{
    call_graph::FnSig,
    ptr::Measurable,
    ssa::{
        constraint::{
            infer::{InferMode, Renamer},
            initialize_local, Database, Gen, Var,
        },
        consume::Consume,
        join_points::PhiNode,
        state::{SSAIdx, SSAState},
        FnResult,
    },
    struct_topology::HasStructTopology,
    CrateCtxt,
};

pub mod boundary;

pub type LocalSig = Range<Var>;
pub type FnBodySig<LocalSig> = IndexVec<Local, IndexVec<SSAIdx, LocalSig>>;

pub struct FnSummary {
    pub fn_body_sig: FnBodySig<LocalSig>,
    pub ssa_state: SSAState,
    // pub output_params: OutputParams,
}

impl FnSummary {
    pub fn new<'analysis, 'db, Kind: AnalysisKind<'analysis, 'db>>(
        rn: Renamer,
        infer_cx: InferCtxt<'analysis, 'db, '_, Kind>,
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
        Some(consume.map_valid(|ssa_idx| self.fn_body_sig[local][ssa_idx].clone()))
    }

    #[inline]
    fn location_result(&'a self, location: Location) -> Self::LocationResults {
        let consume_chain = &self.ssa_state.consume_chain;
        let consumes = consume_chain.of_location(location);
        consumes.iter().map(|(local, consume)| {
            (
                *local,
                consume.map_valid(|ssa_idx| self.fn_body_sig[*local][ssa_idx].clone()),
            )
        })
    }
}

pub struct InferCtxt<'infercx, 'db, 'tcx, Analysis>
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db>,
{
    inter_ctxt: Analysis::InterCtxt,
    database: &'infercx mut Analysis::DB,
    gen: &'infercx mut Gen,
    crate_ctxt: &'infercx CrateCtxt<'tcx>,
    fn_body_sig: FnBodySig<LocalSig>,
    deref_copy: Option<Consume<<Analysis as InferMode<'infercx, 'db, 'tcx>>::LocalSig>>,
}

impl<'infercx, 'db, 'tcx, Analysis> InferCtxt<'infercx, 'db, 'tcx, Analysis>
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db>,
{
    pub fn new(
        crate_ctxt: &'infercx CrateCtxt<'tcx>,
        body: &Body<'tcx>,
        database: &'infercx mut Analysis::DB,
        gen: &'infercx mut Gen,
        inter_ctxt: Analysis::InterCtxt,
    ) -> Self {
        let mut fn_body_sig = IndexVec::with_capacity(body.local_decls.len());

        for local_decl in body.local_decls.iter() {
            if let Some(sigs) = initialize_local(local_decl, gen, database, crate_ctxt) {
                fn_body_sig.push(IndexVec::from_raw(vec![sigs]));
            } else {
                fn_body_sig.push(IndexVec::default());
            }
        }

        <Analysis as Boundary>::handle_params(
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
            deref_copy: None,
        }
    }

    pub fn new_sigs(&mut self, size: u32) -> Range<Var> {
        self.database.push_vars(self.gen.new_sigs(size))
    }

    fn project_deeper(
        base: Consume<<Analysis as InferMode<'infercx, 'db, 'tcx>>::LocalSig>,
        ty: Ty<'tcx>,
        projection: &[PlaceElem<'tcx>],
        infer_cx: &mut Self,
    ) -> Option<Consume<<Analysis as InferMode<'infercx, 'db, 'tcx>>::LocalSig>> {
        // assert!(!base.is_invalid());

        let mut base_ty = ty;

        let mut ptr_chased = 0;
        // let mut adt_gated = false;

        let mut proj_start_offset = 0;

        for projection_elem in projection {
            match projection_elem {
                // do not track pointers behind dereferences for now
                ProjectionElem::Deref => {
                    // No need to set up threshold. Consumption of indirect places are processed
                    // only if definitions contain them, which happen in phases where threshold.
                    // Furthermore, mir places contain only at most one indirection.

                    let ptr = base.r#use.start + proj_start_offset;
                    if ptr < base.r#use.end {
                        infer_cx
                            .database
                            .push_assume::<crate::ssa::constraint::Debug>((), ptr, true);
                    } else {
                        break;
                    }

                    proj_start_offset += 1;
                    base_ty = base_ty.builtin_deref(true).unwrap().ty;
                    ptr_chased += 1;
                }
                ProjectionElem::Field(field, ty) => {
                    let TyKind::Adt(adt_def, _) = base_ty.kind() else { unreachable!() };
                    let Some(field_offsets) = infer_cx
                        .crate_ctxt
                        .struct_topology()
                        .field_offsets(&adt_def.did(), ptr_chased) else { unreachable!() };
                    proj_start_offset += field_offsets[field.index()];
                    base_ty = *ty;
                    // adt_gated = true;
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

        if base.r#use.start + proj_start_offset >= base.r#use.end {
            return None;
        }

        // TODO if proj to invalid, should the following constraints be emitted?

        for (pre, post) in (base.r#use.start..base.r#use.start + proj_start_offset)
            .zip(base.def.start..base.def.start + proj_start_offset)
        {
            infer_cx
                .database
                .push_equal::<crate::ssa::constraint::Debug>((), pre, post);
        }

        let proj_end_offset = proj_start_offset + infer_cx.crate_ctxt.measure(base_ty, ptr_chased);

        #[cfg(debug_assertions)]
        assert!(
            base.r#use.start + proj_end_offset <= base.r#use.end,
            "{ty}: {} ~> {base_ty}: {}, with projection: {:?}, chased: {ptr_chased}",
            base.r#use.end.index() - base.r#use.start.index(),
            proj_end_offset - proj_start_offset,
            projection
        );
        #[cfg(not(debug_assertions))]
        assert!(base.r#use.start + proj_end_offset <= base.r#use.end);

        for (pre, post) in (base.r#use.start + proj_end_offset..base.r#use.end)
            .zip(base.def.start + proj_end_offset..base.def.end)
        {
            infer_cx
                .database
                .push_equal::<crate::ssa::constraint::Debug>((), pre, post);
        }

        Some(Consume {
            r#use: base.r#use.start + proj_start_offset..base.r#use.start + proj_end_offset,
            def: base.def.start + proj_start_offset..base.def.start + proj_end_offset,
        })
    }
}

impl<'infercx, 'db, 'tcx, Analysis> InferMode<'infercx, 'db, 'tcx> for Analysis
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db>,
    <Analysis as AnalysisKind<'infercx, 'db>>::DB: 'infercx,
{
    type Ctxt = InferCtxt<'infercx, 'db, 'tcx, Analysis>;

    type LocalSig = LocalSig;

    type CallArgs<T> = FnSig<T> where T: std::fmt::Debug;

    #[inline]
    fn collect_call_args(
        call_args: impl Iterator<Item = Option<Consume<Self::LocalSig>>>,
    ) -> Self::CallArgs<Option<Consume<Self::LocalSig>>> {
        call_args.collect()
    }

    #[inline]
    fn define_phi_node(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        local: Local,
        ty: Ty<'tcx>,
        def: SSAIdx,
    ) {
        let measure = infer_cx.crate_ctxt.measure(ty, 0);
        let sigs = infer_cx.new_sigs(measure);
        assert_eq!(def, infer_cx.fn_body_sig[local].push(sigs));
    }

    fn join_phi_nodes<'a>(
        infer_cx: &'a mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        phi_nodes: impl Iterator<Item = (Local, &'a mut PhiNode)>,
    ) {
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
                        .push_equal::<crate::ssa::constraint::Debug>((), lhs_sig, rhs_sig)
                }
            }
        }
    }

    fn interpret_consume(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Option<Consume<SSAIdx>>,
    ) -> Option<Consume<Self::LocalSig>> {
        let base = place.local;
        let base_ty = body.local_decls[base].ty;

        let base = if let Some(consume) = consume {
            let base_offset = infer_cx.crate_ctxt.measure(base_ty, 0);

            tracing::debug!("interpretting consume for {:?} with {:?}", place, consume);

            let r#use = infer_cx.fn_body_sig[base][consume.r#use].clone();
            let def = infer_cx.new_sigs(base_offset);
            assert_eq!(base_offset, r#use.end.as_u32() - r#use.start.as_u32());
            assert_eq!(
                infer_cx.fn_body_sig[base].push(def.start..def.end),
                consume.def
            );

            // let base = Consume { r#use, def };
            Consume { r#use, def }
        } else if let Some(consume) = infer_cx.deref_copy.take() {
            consume
        } else {
            return None;
        };

        InferCtxt::project_deeper(base, base_ty, place.projection, infer_cx)
    }

    fn copy_for_deref(infer_cx: &mut Self::Ctxt, consume: Option<Consume<Self::LocalSig>>) {
        assert!(infer_cx.deref_copy.is_none());
        infer_cx.deref_copy = consume
    }

    /// This is correct because of ownership dominance property!!! We are able to treat indirection
    /// as field selection!
    /// For *mut {f: *mut, g: *mut}, it is allowed that the outter is transfered but inners aren't.
    /// This is allowed in one sentence. But it will never be allowed in a body because of `finalise`!!!
    fn transfer(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        lhs_result: Consume<Self::LocalSig>,
        rhs_result: Consume<Self::LocalSig>,
    ) {
        for (lhs_use, lhs_def, rhs_use, rhs_def) in izip!(
            lhs_result.r#use,
            lhs_result.def,
            rhs_result.r#use,
            rhs_result.def
        ) {
            infer_cx
                .database
                .push_assume::<crate::ssa::constraint::Debug>((), lhs_use, false);
            infer_cx
                .database
                .push_linear::<crate::ssa::constraint::Debug>((), lhs_def, rhs_def, rhs_use)
        }
    }

    fn cast(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        lhs: Consume<Self::LocalSig>,
        rhs: Consume<Self::LocalSig>,
    ) {
        let lhs = lhs.repack(|sigs| sigs.start..sigs.start + 1u32);
        let rhs = rhs.repack(|sigs| sigs.start..sigs.start + 1u32);
        Self::transfer(infer_cx, lhs, rhs)
    }

    #[inline]
    fn unknown_sink(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        consume: Consume<Self::LocalSig>,
    ) {
        for (r#use, def) in consume.r#use.zip(consume.def) {
            infer_cx
                .database
                .push_less_equal::<crate::ssa::constraint::Debug>((), def, r#use);
        }
    }

    #[inline]
    fn assume(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        result: Self::LocalSig,
        value: bool,
    ) {
        for sig in result {
            infer_cx
                .database
                .push_assume::<crate::ssa::constraint::Debug>((), sig, value)
        }
    }

    #[inline]
    fn finalise(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        local: Local,
        r#use: SSAIdx,
    ) {
        for sig in infer_cx.fn_body_sig[local][r#use].clone() {
            infer_cx
                .database
                .push_assume::<crate::ssa::constraint::Debug>((), sig, false)
        }
    }

    fn call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        caller: Self::CallArgs<Option<Consume<Self::LocalSig>>>,
        func: &Operand,
    ) {
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
                        <Analysis as Boundary>::handle_call(infer_cx, &caller, callee)
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

    fn r#return(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        ssa_idx: Option<SSAIdx>,
        me: DefId,
    ) {
        let output = ssa_idx.map(|ssa_idx| infer_cx.fn_body_sig[RETURN_PLACE][ssa_idx].clone());
        <Analysis as Boundary>::handle_ret(infer_cx, me, output)
    }

    fn cast_to_c_void(
        infer_cx: &mut Self::Ctxt,
        consume: Consume<Self::LocalSig>,
    ) -> Consume<Self::LocalSig> {
        consume.repack(|sigs| {
            let (outter, inner) = (sigs.start..sigs.start + 1u32, sigs.start + 1u32..sigs.end);
            Self::assume(infer_cx, inner, false);
            outter
        })
    }
}