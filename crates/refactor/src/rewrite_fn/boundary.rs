use common::rewrite::Rewrite;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Location, Operand, Place};
use rustc_span::Span;

use super::{FnRewriteCtxt, PlaceValueType};
use crate::FnLocals;

impl<'tcx, 'me> FnRewriteCtxt<'tcx, 'me> {
    pub fn rewrite_boundary(
        &self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        _destination: Place<'tcx>,
        _fn_span: Span,
        location: Location,
        fn_decision: &FnLocals,
        rewriter: &mut impl Rewrite,
    ) {
        let callee_decision = fn_decision.local_data(&callee);
        for (ctxt, operand) in itertools::izip!(&callee_decision[1..], args) {
            if let Some(place) = operand.place() {
                let Some(local) = place.as_local() else { panic!() };
                let ty = self.body.local_decls[local].ty;
                let required = PlaceValueType::from_ptr_ctxt(ty, ctxt);
                self.rewrite_temporary(local, location, required, rewriter);
            }
        }
    }
}
