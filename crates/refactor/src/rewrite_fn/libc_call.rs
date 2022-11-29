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
            _ => self.default_rewrite(
                callee.owner_id.to_def_id(),
                args,
                destination,
                fn_span,
                location,
                rewriter,
            ),
        }
    }

    fn default_rewrite(
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

        let fn_sig = tcx.fn_sig(callee).skip_binder();
        for (arg, ty) in itertools::izip!(args, fn_sig.inputs()) {
            let ctxt: &[_] = if ty.is_unsafe_ptr() {
                &[PointerKind::Raw]
            } else {
                &[]
            };
            if let Some(place) = arg.place() {
                let Some(local) = place.as_local() else { panic!() };
                let def_loc = def_use_chain.def_loc(local, location);
                let RichLocation::Mir(def_loc) = def_loc else { panic!() };
                let Left(stmt) = body.stmt_at(def_loc) else {
                    // TODO correctness?
                    return
                };
                let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
                self.rewrite_rvalue_at(rvalue, def_loc, stmt.source_info.span, ctxt, rewriter);
            }
        }
    }
}
