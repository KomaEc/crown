use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::Right;
use rustc_hir::{def_id::DefId, definitions::DefPathData};
use rustc_middle::mir::{Local, Location, Operand, Place, TerminatorKind};
use rustc_span::Span;
use rustc_type_ir::TyKind::FnDef;

use super::{FnRewriteCtxt, PlaceValueType};
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
                let required = PlaceValueType::from_ptr_ctxt(ty, ctxt);
                if let Some(as_mut_ptr_span) = self.is_call_of_as_mut_ptr(local, location) {
                    if required.is_ptr() && required.expect_ptr()[0].is_mut() {
                        rewriter.replace(
                            self.tcx,
                            as_mut_ptr_span.shrink_to_hi(),
                            ".as_mut()".to_owned(),
                        );
                        return;
                    }
                }
                self.rewrite_temporary(local, location, required, rewriter);
            }
        }

        let ret_ty = destination.ty(self.body, self.tcx).ty;
        let required = self.acquire_place_info(&destination);
        let produced = PlaceValueType::from_ptr_ctxt(ret_ty, &callee_decision[0]);
        println!("calling {}, adapting {:?} to {:?}", self.tcx.def_path_str(callee), produced, required);
        self.adapt_usage(fn_span, ret_ty, destination.is_indirect(), produced, required, rewriter)
    }

    /// Hack
    fn is_call_of_as_mut_ptr(&self, arg: Local, location: Location) -> Option<Span> {
        let FnRewriteCtxt {
            tcx,
            body,
            def_use_chain,
            ..
        } = *self;
        let def_loc = def_use_chain.def_loc(arg, location);
        let RichLocation::Mir(def_loc) = def_loc else { return None };
        let Right(term) = body.stmt_at(def_loc) else { return None };
        let TerminatorKind::Call { func, ..} = &term.kind else { return None };
        let Some(func) = func.constant() else { return None };
        let ty = func.ty();
        let &FnDef(callee, _) = ty.kind() else { return None };
        let false = callee.as_local().is_some() else { return None };
        let def_path = tcx.def_path(callee);
        if let [DefPathData::TypeNs(slice), _, DefPathData::ValueNs(as_mut_ptr), ..] = &def_path
            .data
            .iter()
            .map(|data| data.data)
            .collect::<smallvec::SmallVec<[_; 4]>>()[..]
        {
            if slice.as_str() == "slice" && as_mut_ptr.as_str() == "as_mut_ptr" {
                return Some(term.source_info.span);
            }
        }
        None
    }
}
