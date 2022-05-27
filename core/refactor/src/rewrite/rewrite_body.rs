use analysis::{call_graph::Func, fat_thin_analysis, mutability_analysis, ownership_analysis};
use rewriter::Rewriter;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

pub fn rewrite_fn_body(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    func: Func,
    fn_did: LocalDefId,
) {
    let body = tcx.optimized_mir(fn_did);
}
