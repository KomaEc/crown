use rustc_lint::{LateContext, LateLintPass, LintPass};
use rustc_hir::{Mod, HirId};
use rustc_span::Span;

pub struct RewritePass;

impl LintPass for RewritePass {
    fn name(&self) -> &'static str {
        "InitialRewritePass"
    }
}

impl<'tcx> LateLintPass<'tcx> for RewritePass {
    fn check_mod(&mut self, _: &LateContext<'tcx>, _: &'tcx Mod< 'tcx>, _: Span, _: HirId) {
        
    }
}