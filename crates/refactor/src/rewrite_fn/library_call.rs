use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::Left;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Location, Operand, Place, StatementKind};
use rustc_span::Span;

use super::FnRewriteCtxt;
use crate::PointerKind;

impl<'tcx, 'me> FnRewriteCtxt<'tcx, 'me> {
    pub fn rewrite_library_call(
        &self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        fn_span: Span,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            local_decision,
            struct_decision,
            body,
            def_use_chain,
            user_idents,
            tcx,
        } = *self;
    }
}
