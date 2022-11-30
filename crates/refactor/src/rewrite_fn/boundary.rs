use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::Left;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Location, Operand, Place, StatementKind};
use rustc_span::Span;

use super::FnRewriteCtxt;
use crate::FnLocals;

impl<'tcx, 'me> FnRewriteCtxt<'tcx, 'me> {
    pub fn rewrite_boundary(
        &self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        fn_span: Span,
        location: Location,
        fn_decision: &FnLocals,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            ..
        } = *self;

        let callee_decision = fn_decision.local_data(&callee);
        for (ctxt, operand) in itertools::izip!(&callee_decision[1..], args) {
            if let Some(place) = operand.place() {
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
