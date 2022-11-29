use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::Left;
use rustc_hir::{def_id::DefId, ForeignItem};
use rustc_middle::mir::{Location, Operand, Place, StatementKind};
use rustc_span::Span;

use super::FnRewriteCtxt;
use crate::PointerKind;

impl<'tcx, 'me> FnRewriteCtxt<'tcx, 'me> {
    pub fn rewrite_libc_call(
        &self,
        callee: &ForeignItem<'tcx>,
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

        match callee.ident.as_str() {
            "malloc" => {}
            "free" => {}
            "printf" => {}
            _ => self.rewrite_call_default(
                callee.owner_id.to_def_id(),
                args,
                destination,
                fn_span,
                location,
                rewriter,
            ),
        }
    }
}
