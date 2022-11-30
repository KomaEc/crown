use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::Left;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Location, Operand, Place, Rvalue, StatementKind};
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
        let FnRewriteCtxt { tcx, .. } = *self;

        let def_path = tcx.def_path(callee);
        // if it is a library call in core::ptr
        if def_path
            .data
            .get(0)
            .map(|d| match d.data {
                rustc_hir::definitions::DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
                _ => false,
            })
            .is_some()
        {
            // if it is core::ptr::<..>::..
            if let Some(d) = def_path.data.get(3) {
                if let rustc_hir::definitions::DefPathData::ValueNs(s) = d.data {
                    match s.as_str() {
                        "is_null" => {
                            self.rewrite_is_null(args, destination, fn_span, location, rewriter);
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.rewrite_call_default(callee, args, destination, fn_span, location, rewriter);
    }

    fn rewrite_is_null(
        &self,
        args: &Vec<Operand<'tcx>>,
        _destination: Place<'tcx>,
        fn_span: Span,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            tcx,
            ..
        } = *self;

        assert_eq!(args.len(), 1);
        let arg = args[0].place().unwrap().as_local().unwrap();
        let def_loc = def_use_chain.def_loc(arg, location);
        let RichLocation::Mir(def_loc) = def_loc else { panic!() };
        let Left(stmt) = body.stmt_at(def_loc) else {
            // TODO correctness?
            return
        };
        let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
        if let Rvalue::Use(operand) = rvalue {
            if let Some(place) = operand.place() {
                let ctxt = self.acquire_place_info(&place);
                if matches!(ctxt.first(), Some(ptr_kind) if ptr_kind.is_safe()) {
                    rewriter.replace(tcx, fn_span, "is_none()".to_string());
                    self.rewrite_rvalue_at(
                        rvalue,
                        def_loc,
                        stmt.source_info.span,
                        &[PointerKind::Shr],
                        rewriter,
                    );
                    return;
                }
            }
        }

        self.rewrite_rvalue_at(
            rvalue,
            def_loc,
            stmt.source_info.span,
            &[PointerKind::Raw],
            rewriter,
        );
    }
}
