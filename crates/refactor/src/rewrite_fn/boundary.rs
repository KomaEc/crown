use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::Right;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Local, Location, Operand, Place};
use rustc_span::Span;

use super::{FnRewriteCtxt, PlaceCtxt};
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
        let callee_decision = fn_decision.local_data(&callee);
        for (ctxt, operand) in itertools::izip!(&callee_decision[1..], args) {
            if let Some(place) = operand.place() {
                let Some(local) = place.as_local() else { panic!() };
                let ty = self.body.local_decls[local].ty;
                let required = PlaceCtxt::from_ptr_ctxt(ty, ctxt);
                // Hack! may not work is no-box is off
                if required.is_ptr()
                    && required.expect_ptr()[0].is_mut()
                    && self.arg_is_dest_of_call(local, location)
                {
                    let def_span = self.get_temporary_def_span(local, location);
                    rewriter.replace(self.tcx, def_span.shrink_to_hi(), ".as_mut()".to_owned())
                }
                self.rewrite_temporary(local, location, required, rewriter);
            }
        }

        let ret_ty = destination.ty(self.body, self.tcx).ty;
        let required = self.acquire_place_info(&destination);
        let produced = PlaceCtxt::from_ptr_ctxt(ret_ty, &callee_decision[0]);
        self.adapt_usage(
            fn_span,
            ret_ty,
            destination.is_indirect(),
            produced,
            required,
            rewriter,
        )
    }

    fn arg_is_dest_of_call(&self, arg: Local, location: Location) -> bool {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            ..
        } = *self;
        let def_loc = def_use_chain.def_loc(arg, location);
        let RichLocation::Mir(def_loc) = def_loc else { return false };
        matches!(body.stmt_at(def_loc), Right(..))
    }
}
