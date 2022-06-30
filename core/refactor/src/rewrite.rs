mod rewrite_body;

use analysis::{
    api::AnalysisResults, call_graph::Func, fat_thin_analysis, mutability_analysis, null_analysis,
    ownership_analysis,
};
use rewriter::{RewriteMode, Rewriter};
use rustc_hir::{def_id::LocalDefId, FnRetTy, FnSig, ItemKind, Ty, TyKind};
use rustc_middle::{
    mir::{Field, Local},
    ty::TyCtxt,
};

use rewrite_body::{rewrite_body, BodyRewriteCtxt};

fn ty_nested_depth(ty: &Ty) -> usize {
    match &ty.kind {
        TyKind::Ptr(mut_ty) => 1 + ty_nested_depth(mut_ty.ty),
        _ => 0,
    }
}

pub fn rewrite<'tcx>(
    tcx: TyCtxt<'tcx>,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    null_analysis: &null_analysis::CrateResults<'tcx, '_>,
    struct_defs: &[LocalDefId],
    rewrite_mode: RewriteMode,
) {
    let mut rewriter = rewriter::Rewriter::default();
    rewrite_structs(
        tcx,
        &mut rewriter,
        ownership_analysis,
        fatness_analysis,
        null_analysis,
        struct_defs,
    );
    rewrite_functions(
        tcx,
        &mut rewriter,
        ownership_analysis,
        mutability_analysis,
        fatness_analysis,
        null_analysis,
    );

    rewriter.write(rewrite_mode)
}

fn rewrite_functions<'tcx, 'a>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    null_analysis: &null_analysis::CrateResults<'tcx, 'a>,
) {
    for (func, did) in ownership_analysis.call_graph.functions.iter_enumerated() {
        let did = did.expect_local();
        let item = tcx.hir().expect_item(did);
        let ItemKind::Fn(sig, _generics, _body_id) = &item.kind else { panic!() };
        rewrite_fn_sig(
            tcx,
            rewriter,
            ownership_analysis,
            mutability_analysis,
            fatness_analysis,
            null_analysis,
            func,
            did,
            sig,
        );
        let mut body_rewrite_cx = BodyRewriteCtxt {
            tcx,
            rewriter,
            ownership: ownership_analysis,
            mutability: mutability_analysis,
            fatness: fatness_analysis,
            null: null_analysis,
            func,
            def_id: did,
            body: tcx.optimized_mir(did),
        };
        rewrite_body(&mut body_rewrite_cx);
    }
}

fn rewrite_fn_sig(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership: &ownership_analysis::InterSummary,
    mutability: &mutability_analysis::InterSummary,
    fatness: &fat_thin_analysis::CrateSummary,
    null: &null_analysis::CrateResults,
    func: Func,
    def_id: LocalDefId,
    sig: &FnSig,
) {
    let results_for_local = |i, ty| {
        let local = Local::from_usize(i);
        let nested_depth = ty_nested_depth(ty);
        (0..nested_depth)
            .map(|nested_level| PtrResults {
                owning: ownership.func_sigs[func].sig[i][nested_level],
                fat: fatness.lambda_ctxt.assumptions[fatness.func_summaries[func].func_sig[i]
                    .clone()
                    .nth(nested_level)
                    .unwrap()]
                .unwrap_or(false),
                // i thought this unwrap_or should be true, but using false causes fewer errors in
                // bst-good :)
                mutable: mutability.func_sigs[func].sig[i][0 /* TODO */].unwrap_or(false),
                nullable: null.local_result(def_id, local, nested_level).unwrap(),
            })
            .collect::<Vec<_>>()
    };

    if let FnRetTy::Return(ty) = sig.decl.output {
        rewrite_raw_ptr_ty(tcx, rewriter, ty, &results_for_local(0, ty));
    }

    for (i, ty) in sig.decl.inputs.iter().enumerate() {
        rewrite_raw_ptr_ty(tcx, rewriter, ty, &results_for_local(i + 1, ty));
    }
}

fn rewrite_structs<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    null_analysis: &null_analysis::CrateResults<'tcx, '_>,
    dids: &[LocalDefId],
) {
    for &did in dids {
        let item = tcx.hir().expect_item(did);
        let ItemKind::Struct(variant_data, _generics) = &item.kind else { panic!() };
        rewrite_struct(
            tcx,
            rewriter,
            ownership_analysis,
            fatness_analysis,
            null_analysis,
            variant_data,
            did,
        );
    }
}

fn rewrite_struct<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    ownership: &ownership_analysis::InterSummary,
    fatness: &fat_thin_analysis::CrateSummary,
    null: &null_analysis::CrateResults<'tcx, '_>,
    variant_data: &rustc_hir::VariantData,
    did: LocalDefId,
) {
    for (idx, field) in variant_data.fields().iter().enumerate() {
        let field_idx = Field::from_usize(idx);
        let nested_depth = ty_nested_depth(field.ty);
        let results = (0..nested_depth)
            .map(|nested_level| {
                PtrResults {
                    owning: ownership.field_result(did, field_idx, nested_level),
                    fat: fatness.field_result(did, field_idx, nested_level).unwrap(),
                    mutable: true, // TODO
                    nullable: null.field_result(did, field_idx, nested_level).unwrap(),
                }
            })
            .collect::<Vec<_>>();
        rewrite_raw_ptr_ty(tcx, rewriter, field.ty, &results);
    }
}

#[derive(Clone, Copy)]
struct PtrResults {
    owning: Option<bool>,
    fat: bool,
    mutable: bool,
    nullable: bool,
}

fn rewrite_raw_ptr_ty(tcx: TyCtxt<'_>, rewriter: &mut Rewriter, ty: &Ty, results: &[PtrResults]) {
    // we want both recursion and local variable capture, so we need both a fn and a closure
    fn visit_nested_pointer(
        ty: &Ty,
        results: &[PtrResults],
        f: &mut impl FnMut(&Ty, Option<&PtrResults>),
    ) {
        if let TyKind::Ptr(mut_ty) = &ty.kind {
            visit_nested_pointer(mut_ty.ty, &results[1..], f);
        }
        f(ty, results.get(0));
    }

    let mut new_ty = String::new();
    visit_nested_pointer(ty, results, &mut |ty, result| {
        if !matches!(ty.kind, TyKind::Ptr(_)) {
            new_ty = tcx.sess.source_map().span_to_snippet(ty.span).unwrap();
            return;
        }
        let result = result.unwrap();

        if result.fat {
            new_ty = format!("[{new_ty}]");
        }

        let ptr = match (result.owning, result.mutable) {
            (Some(true), _) => {
                new_ty.push('>');
                "Box<"
            }
            (Some(false), true) => "&mut ",
            (Some(false), false) => "&",
            (None, true) => "*mut ",
            (None, false) => "*const ",
        };
        new_ty.insert_str(0, ptr);

        if result.nullable {
            new_ty.insert_str(0, "Option<");
            new_ty.push('>');
        }
    });
    rewriter.make_suggestion(tcx, ty.span, String::new(), new_ty);
}
