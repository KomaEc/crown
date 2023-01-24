use std::ops::Range;

use common::data_structure::assoc::AssocExt;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{Body, Local, LocalInfo, Location, Operand, Place, PlaceElem, ProjectionElem},
    ty::{AdtDef, Ty, TyCtxt, TyKind},
};
use rustc_type_ir::TyKind::FnDef;
use smallvec::SmallVec;

use self::boundary::Boundary;
use super::{AnalysisKind, Precision};
use crate::{
    ptr::{decompose_ty, Measurable},
    ssa::{
        constraint::{
            infer::{InferMode, Renamer},
            initialize_local, Database, Gen, GlobalAssumptions, Var,
        },
        consume::Consume,
        join_points::PhiNode,
        state::{SSAIdx, SSAState},
        FnResults,
    },
    struct_ctxt::{RestrictedStructCtxt, StructCtxt},
    CrateCtxt,
};

pub mod boundary;

pub type LocalSig = Range<Var>;
pub type FnBodySig<LocalSig> = IndexVec<Local, IndexVec<SSAIdx, LocalSig>>;

pub struct FnSummary {
    pub fn_body_sig: FnBodySig<LocalSig>,
    pub ssa_state: SSAState,
}

impl FnSummary {
    pub fn new<'analysis, 'db, 'tcx, Kind: AnalysisKind<'analysis, 'db, 'tcx>>(
        rn: Renamer,
        infer_cx: InferCtxt<'analysis, 'db, 'tcx, Kind>,
    ) -> Self {
        FnSummary {
            fn_body_sig: infer_cx.fn_body_sig,
            ssa_state: rn.state,
        }
    }
}

impl<'a> FnResults<'a> for FnSummary {
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
    fn location_results(&'a self, location: Location) -> Self::LocationResults {
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

type CallArgs = SmallVec<[Option<(Consume<LocalSig>, bool)>; 4]>;

pub struct InferCtxt<'infercx, 'db, 'tcx, Analysis>
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db, 'tcx>,
{
    tcx: TyCtxt<'tcx>,
    inter_ctxt: Analysis::InterCtxt,
    database: &'infercx mut Analysis::DB,
    gen: &'infercx mut Gen,
    struct_ctxt: RestrictedStructCtxt<'infercx, 'tcx>,
    fn_body_sig: FnBodySig<LocalSig>,
    deref_copy: Option<Consume<<Analysis as InferMode<'infercx, 'db, 'tcx>>::LocalSig>>,
    call_args: Vec<(
        Local,
        (
            Consume<<Analysis as InferMode<'infercx, 'db, 'tcx>>::LocalSig>,
            bool,
        ),
    )>,
    global_assumptions: &'infercx GlobalAssumptions,
}

impl<'infercx, 'db, 'tcx, Analysis> InferCtxt<'infercx, 'db, 'tcx, Analysis>
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db, 'tcx>,
{
    pub fn new(
        crate_ctxt: &'infercx CrateCtxt<'tcx>,
        max_precision: Precision,
        body: &Body<'tcx>,
        database: &'infercx mut Analysis::DB,
        gen: &'infercx mut Gen,
        inter_ctxt: Analysis::InterCtxt,
        global_assumptions: &'infercx GlobalAssumptions,
    ) -> Self {
        let struct_ctxt = crate_ctxt.struct_ctxt.with_max_precision(max_precision);
        let mut fn_body_sig = IndexVec::with_capacity(body.local_decls.len());

        for local_decl in body.local_decls.iter() {
            if let Some(sigs) = initialize_local(local_decl, gen, database, struct_ctxt) {
                fn_body_sig.push(IndexVec::from_raw(vec![sigs]));
            } else {
                fn_body_sig.push(IndexVec::default());
            }
        }

        <Analysis as Boundary>::entry(
            crate_ctxt,
            &inter_ctxt,
            global_assumptions,
            database,
            body,
            fn_body_sig
                .iter()
                .skip(1)
                .take(body.arg_count)
                .map(|vec| vec.raw.first().cloned()),
        );

        InferCtxt {
            tcx: crate_ctxt.tcx,
            inter_ctxt,
            database,
            gen,
            struct_ctxt,
            fn_body_sig,
            deref_copy: None,
            call_args: Vec::new(),
            global_assumptions,
        }
    }

    /// Dominance property
    fn new_vars(&mut self, ty: Ty<'tcx>) -> Range<Var> {
        let measure = self.struct_ctxt.measure(ty, 0);
        let vars = self.database.new_vars(self.gen, measure);
        let precision = self.struct_ctxt.max_ptr_chased();
        fn dominate<'tcx>(
            ty: Ty<'tcx>,
            mut dom: Option<Var>,
            vars: &mut Range<Var>,
            mut precision: u8,
            database: &mut impl Database,
            struct_ctxt: &StructCtxt,
            tcx: TyCtxt<'tcx>,
        ) {
            if precision == 0 {
                return;
            }

            let mut ty = ty;
            loop {
                if let Some(inner_ty) = ty.builtin_index() {
                    ty = inner_ty;
                    continue;
                }
                if let Some(ty_mut) = ty.builtin_deref(true) {
                    let var = vars.next().unwrap();
                    if let Some(dom) = dom {
                        database.push_less_equal::<crate::ssa::constraint::Debug>((), var, dom)
                    }

                    dom = Some(var);

                    precision -= 1;
                    if precision == 0 {
                        return;
                    }
                    ty = ty_mut.ty;
                    continue;
                }
                break;
            }

            if let TyKind::Adt(adt_def, subst) = ty.kind() {
                if struct_ctxt.is_struct_of_concerned(&adt_def.did()) {
                    for field_def in adt_def.all_fields() {
                        let field_ty = field_def.ty(tcx, subst);
                        dominate(field_ty, dom, vars, precision, database, struct_ctxt, tcx)
                    }
                }
            }
        }

        dominate(
            ty,
            None,
            vars.clone().by_ref(),
            precision,
            self.database,
            self.struct_ctxt.unrestricted,
            self.tcx,
        );

        vars
    }

    fn project_deeper(
        base: Consume<<Analysis as InferMode<'infercx, 'db, 'tcx>>::LocalSig>,
        ty: Ty<'tcx>,
        projection: &[PlaceElem<'tcx>],
        infer_cx: &mut Self,
    ) -> Option<Consume<<Analysis as InferMode<'infercx, 'db, 'tcx>>::LocalSig>> {
        let mut base_ty = ty;

        let base_measure = base.r#use.size_hint().1.unwrap() as u32;
        // if base_measure == 0 { return None; }
        let precision = infer_cx
            .struct_ctxt
            .absolute_precision(base_ty, base_measure) as u32;
        let max_ptr_chased = infer_cx.struct_ctxt.max_ptr_chased() as u32;

        // let mut ptr_chased = 0;
        let mut ptr_chased = max_ptr_chased - precision;

        let mut proj_start_offset = 0;

        // let mut deref_var = None;

        for projection_elem in projection {
            match projection_elem {
                // do not track pointers behind dereferences for now
                ProjectionElem::Deref => {
                    // No need to set up threshold. Consumption of indirect places are processed
                    // only if definitions contain them, which happen in phases where threshold.
                    // Furthermore, mir places contain only at most one indirection.

                    // let ptr = base.r#use.start + proj_start_offset;
                    // if ptr < base.r#use.end {
                    //     infer_cx
                    //         .database
                    //         .push_assume::<crate::ssa::constraint::Debug>((), ptr, true);
                    // } else {
                    //     break;
                    // }
                    // deref_var = Some(ptr);

                    proj_start_offset += 1;
                    base_ty = base_ty.builtin_deref(true).unwrap().ty;
                    ptr_chased += 1;
                }
                ProjectionElem::Field(field, ty) => {
                    let TyKind::Adt(adt_def, _) = base_ty.kind() else { unreachable!() };
                    proj_start_offset +=
                        infer_cx
                            .struct_ctxt
                            .field_offset(*adt_def, field.index(), ptr_chased);
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
                ProjectionElem::OpaqueCast(_) => unreachable!("unexpected opaque cast"),
                ProjectionElem::Downcast(..) => unreachable!("unexpected downcasting"),
            }
        }

        if base.r#use.start + proj_start_offset >= base.r#use.end {
            for (pre, post) in base.r#use.zip(base.def) {
                infer_cx
                    .database
                    .push_equal::<crate::ssa::constraint::Debug>((), pre, post);
            }
            return None;
        }

        // FIXME: this is currently buggy. What we really want to do is to enable consumption only
        // for pointers that are known to be owning. However, here we enforce a stricter constraint,
        // that once outtermost pointer is proven to be owning, not only does its consumption is
        // enabled, but also every further dereferences are enforced to be owning.
        // if let Some(ptr) = deref_var {
        //     infer_cx
        //         .database
        //         .push_assume::<crate::ssa::constraint::Debug>((), ptr, true);
        // }

        // TODO if proj to invalid, should the following constraints be emitted?

        for (pre, post) in (base.r#use.start..base.r#use.start + proj_start_offset)
            .zip(base.def.start..base.def.start + proj_start_offset)
        {
            infer_cx
                .database
                .push_equal::<crate::ssa::constraint::Debug>((), pre, post);
        }

        let proj_end_offset = proj_start_offset + infer_cx.struct_ctxt.measure(base_ty, ptr_chased);

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
    Analysis: AnalysisKind<'infercx, 'db, 'tcx>,
    <Analysis as AnalysisKind<'infercx, 'db, 'tcx>>::DB: 'infercx,
{
    type Ctxt = InferCtxt<'infercx, 'db, 'tcx, Analysis>;

    type LocalSig = LocalSig;

    #[inline]
    fn call_arg(
        infer_cx: &mut Self::Ctxt,
        temp: Local,
        arg: Consume<Self::LocalSig>,
        is_ref: bool,
    ) {
        infer_cx.call_args.push((temp, (arg, is_ref)))
    }

    #[inline]
    fn define_phi_node(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        local: Local,
        ty: Ty<'tcx>,
        def: SSAIdx,
    ) {
        // let measure = infer_cx.fn_ctxt.measure(ty, 0);
        // let sigs = infer_cx.new_vars(measure);
        let sigs = infer_cx.new_vars(ty);
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
            let base_offset = infer_cx.struct_ctxt.measure(base_ty, 0);

            tracing::debug!("interpretting consume for {:?} with {:?}", place, consume);

            let r#use = infer_cx.fn_body_sig[base][consume.r#use].clone();
            let def = infer_cx.new_vars(base_ty);
            assert_eq!(base_offset, r#use.end.as_u32() - r#use.start.as_u32());
            assert_eq!(
                infer_cx.fn_body_sig[base].push(def.start..def.end),
                consume.def
            );

            // let base = Consume { r#use, def };
            Consume { r#use, def }
        } else if matches!(
            body.local_decls[base].local_info,
            Some(box LocalInfo::DerefTemp)
        ) {
            if let Some(consume) = infer_cx.deref_copy.take() {
                consume
            } else {
                return None;
            }
        } else {
            return None;
        };

        InferCtxt::project_deeper(base, base_ty, place.projection, infer_cx)
    }

    fn copy_for_deref(infer_cx: &mut Self::Ctxt, consume: Option<Consume<Self::LocalSig>>) {
        assert!(infer_cx.deref_copy.is_none());
        infer_cx.deref_copy = consume
    }

    fn transfer<const ENSURE_MOVE: bool>(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        ty: Ty<'tcx>,
        lhs_result: Consume<Self::LocalSig>,
        rhs_result: Consume<Self::LocalSig>,
    ) {
        tracing::debug!("transfer relation: {:?} ~ {:?}", lhs_result, rhs_result);

        matcher(
            ty,
            lhs_result.transpose(),
            rhs_result.transpose(),
            &infer_cx.struct_ctxt,
            infer_cx.database,
            |lhs, rhs, database| {
                database.push_assume::<crate::ssa::constraint::Debug>((), lhs.r#use, false);
                if ENSURE_MOVE {
                    database.push_equal::<crate::ssa::constraint::Debug>((), lhs.def, rhs.r#use);
                    database.push_assume::<crate::ssa::constraint::Debug>((), rhs.def, false);
                } else {
                    database.push_linear::<crate::ssa::constraint::Debug>(
                        (),
                        lhs.def,
                        rhs.def,
                        rhs.r#use,
                    )
                }
            },
        )
    }

    fn cast<const ENSURE_MOVE: bool>(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        ty: Ty<'tcx>,
        lhs: Consume<Self::LocalSig>,
        rhs: Consume<Self::LocalSig>,
    ) {
        if ty.is_unsafe_ptr() || ty.is_box() || ty.is_region_ptr() {
            let lhs = lhs.repack(|sigs| sigs.start..sigs.start + 1u32);
            let rhs = rhs.repack(|sigs| sigs.start..sigs.start + 1u32);
            Self::transfer::<ENSURE_MOVE>(infer_cx, ty, lhs, rhs)
        } else {
            todo!("handling casts between structs are not supported")
        }
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
    fn lend(infer_cx: &mut Self::Ctxt, consume: Consume<Self::LocalSig>) {
        for (r#use, def) in consume.r#use.zip(consume.def) {
            infer_cx
                .database
                .push_equal::<crate::ssa::constraint::Debug>((), r#use, def)
        }
    }

    fn source(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>) {
        Self::assume(infer_cx, result.r#use, false);
        if let Some(sig) = result.def.clone().next() {
            infer_cx
                .database
                .push_assume::<crate::ssa::constraint::Debug>((), sig, true)
        }
    }

    fn sink(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>) {
        if let Some(sig) = result.r#use.clone().next() {
            infer_cx
                .database
                .push_assume::<crate::ssa::constraint::Debug>((), sig, true)
        }
        Self::assume(infer_cx, result.def, false);
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

    fn call(
        infer_cx: &mut InferCtxt<'infercx, 'db, 'tcx, Analysis>,
        destination: Option<Consume<Self::LocalSig>>,
        args: &[Operand<'tcx>],
        callee: &Operand<'tcx>,
    ) {
        let args = args
            .iter()
            .map(|operand| {
                operand
                    .place()
                    .and_then(|operand| operand.as_local())
                    .and_then(|operand| infer_cx.call_args.get_by_key(&operand))
                    .cloned()
            })
            .collect::<SmallVec<_>>();

        if let Some(func) = callee.constant() {
            let ty = func.ty();
            let &FnDef(callee, substs) = ty.kind() else { unreachable!() };
            if let Some(local_did) = callee.as_local() {
                match infer_cx.tcx.hir().find_by_def_id(local_did).unwrap() {
                    // this crate
                    rustc_hir::Node::Item(_) => {
                        <Analysis as Boundary>::call(infer_cx, destination, &args, callee)
                    }
                    // extern
                    rustc_hir::Node::ForeignItem(foreign_item) => {
                        infer_cx.libc_call(destination, &args, callee, foreign_item.ident)
                    }
                    // in libxml2.rust/src/xmlschemastypes.rs/{} impl_xmlSchemaValDate/set_mon
                    rustc_hir::Node::ImplItem(_) => { /* TODO */ }
                    _ => unreachable!(),
                }
            } else {
                // library
                infer_cx.library_call(destination, &args, callee, substs)
            }
        } else {
            // closure or fn ptr
            /* TODO */
        }
    }

    fn r#return<'a>(
        infer_cx: &mut Self::Ctxt,
        locals: impl Iterator<Item = (Local, Option<SSAIdx>)> + 'a,
        body: &'a Body<'tcx>,
    ) {
        let mut locals = locals.map(|(local, ssa_idx)| {
            ssa_idx.map(|ssa_idx| infer_cx.fn_body_sig[local][ssa_idx].clone())
        });

        <Analysis as Boundary>::exit(
            infer_cx.tcx,
            &infer_cx.inter_ctxt,
            infer_cx.global_assumptions,
            infer_cx.struct_ctxt.unrestricted,
            &mut infer_cx.database,
            body,
            locals.by_ref().take(body.arg_count + 1),
        );

        // finalize temporaries
        for vars in locals {
            let Some(vars) = vars else { continue; };
            for var in vars {
                infer_cx
                    .database
                    .push_assume::<crate::ssa::constraint::Debug>((), var, false)
            }
        }
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

/// [`measure`] could be either [`FnCtxt`] or [`StructTopology`]. This is because
/// ptr_chased is relative but precision is absolute.
fn fit<'tcx, T, U, DB>(
    adt_def: AdtDef,
    mut fitter: impl Iterator<Item = T>,
    fitter_precision: Precision,
    mut fittee: impl Iterator<Item = U>,
    delta: Precision,
    measurable: impl Measurable<'tcx>,
    database: &mut DB,
    mut on_matched: impl FnMut(T, U, &mut DB),
) {
    let fitter_ptr_chased = measurable.max_ptr_chased() - fitter_precision;

    let fitter_leaf_nodes = measurable.leaf_nodes(adt_def, fitter_ptr_chased as u32);

    let mut count = 0;
    for &(leaf_ext_ty, offset_to_be) in fitter_leaf_nodes {
        while count < offset_to_be {
            let (Some(fitter), Some(fittee)) = (fitter.next(), fittee.next()) else { panic!() };
            on_matched(fitter, fittee, database);
            count += 1;
        }

        let leaf_ext_measure =
            measurable.measure(leaf_ext_ty, (measurable.max_ptr_chased() - delta) as u32);

        for _ in 0..leaf_ext_measure {
            let _ = fittee.next().unwrap();
        }
    }

    assert!(fitter.next().is_none());
    assert!(fittee.next().is_none());
}

fn matcher<'tcx, T, U, DB>(
    ty: Ty<'tcx>,
    mut lhs_result: impl Iterator<Item = T>,
    mut rhs_result: impl Iterator<Item = U>,
    measurable: impl Measurable<'tcx>,
    database: &mut DB,
    mut on_matched: impl FnMut(T, U, &mut DB),
) {
    let lhs_measure = lhs_result.size_hint().1.unwrap() as u32;
    let rhs_measure = rhs_result.size_hint().1.unwrap() as u32;

    let lhs_precision = measurable.absolute_precision(ty, lhs_measure);
    let rhs_precision = measurable.absolute_precision(ty, rhs_measure);

    tracing::debug!("precision: lhs = {lhs_precision}, rhs = {rhs_precision}");

    let (ptr_depth, adt_def) = decompose_ty(ty);

    if ptr_depth as u8 > lhs_precision
        || ptr_depth as u8 > rhs_precision
        || lhs_precision == rhs_precision
    {
        for (lhs, rhs) in lhs_result.zip(rhs_result) {
            on_matched(lhs, rhs, database);
        }
    } else {
        for (lhs, rhs) in lhs_result
            .by_ref()
            .zip(rhs_result.by_ref())
            .take(ptr_depth as usize)
        {
            on_matched(lhs, rhs, database);
        }

        let lhs_precision = lhs_precision - ptr_depth as u8;
        let rhs_precision = rhs_precision - ptr_depth as u8;

        let delta = lhs_precision.abs_diff(rhs_precision);

        let adt_def = adt_def.unwrap();
        if lhs_precision < rhs_precision {
            fit(
                adt_def,
                lhs_result,
                lhs_precision,
                rhs_result,
                delta,
                measurable,
                database,
                on_matched,
            )
        } else {
            fit(
                adt_def,
                rhs_result,
                rhs_precision,
                lhs_result,
                delta,
                measurable,
                database,
                |rhs, lhs, database| on_matched(lhs, rhs, database),
            )
        }
    }
}
