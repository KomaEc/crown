use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::Left;
use rustc_hir::{def_id::DefId, definitions::DefPathData};
use rustc_middle::mir::{Location, Operand, Place, Rvalue, StatementKind};
use rustc_span::Span;

use super::FnRewriteCtxt;
use crate::{
    rewrite_fn::{PlaceLoadMode, PlaceCtxt},
    PointerKind, RawMeta,
};

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

        if let [DefPathData::TypeNs(slice), _, DefPathData::ValueNs(as_mut_ptr), ..] = &def_path
            .data
            .iter()
            .map(|data| data.data)
            .collect::<smallvec::SmallVec<[_; 4]>>()[..]
        {
            if slice.as_str() == "slice" && as_mut_ptr.as_str() == "as_mut_ptr" {
                self.rewrite_as_mut_ptr(args, destination, None, fn_span, location, rewriter);
                return;
            } else if slice.as_str() == "option" {
                match as_mut_ptr.as_str() {
                    "is_some" | "is_none" => {
                        self.rewrite_is_some(args, destination, fn_span, location, rewriter);
                        return;
                    }
                    _ => {} // fall
                }
            }
        }

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
                        "addr" => return,
                        _ => {}
                    }
                }
            }
        }

        self.rewrite_call_default(callee, args, destination, fn_span, location, rewriter);
    }

    /// NOTE dangerous
    fn rewrite_as_mut_ptr(
        &self,
        args: &Vec<Operand<'tcx>>,
        _destination: Place<'tcx>,
        required: Option<PlaceCtxt>,
        _fn_span: Span,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        if let Some(arg) = args[0].place().and_then(|place| place.as_local()) {
            let ty = self.body.local_decls[arg].ty;
            let required = required.unwrap_or_else(|| {
                PlaceCtxt::from_ptr_ctxt(ty, &[PointerKind::Raw(RawMeta::Mut)])
            });
            self.rewrite_temporary(arg, location, required, rewriter);
        }
    }

    fn rewrite_is_some(
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
        let Rvalue::Ref(_, _, place) = rvalue else { unreachable!() };
        let span = stmt.source_info.span.until(fn_span);
        self.rewrite_place_load_at::<{ PlaceLoadMode::ByValue as u8 }>(
            *place,
            def_loc,
            span,
            PlaceCtxt::Irrelavent,
            rewriter,
        );
        rewriter.replace(tcx, span.shrink_to_hi(), ".".to_owned())
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
                let produced = self.acquire_place_info(&place);
                assert!(produced.is_ptr());
                if !produced.is_raw_ptr() {
                    rewriter.replace(tcx, fn_span, "is_none()".to_string());
                    self.rewrite_rvalue_at(
                        rvalue,
                        def_loc,
                        stmt.source_info.span,
                        PlaceCtxt::Ptr(&[PointerKind::Const]),
                        rewriter,
                    );
                    return;
                }
            }
        }

        let ty = self.body.local_decls[arg].ty;
        let required = PlaceCtxt::from_ptr_ctxt(
            ty,
            if ty.is_mutable_ptr() {
                &[PointerKind::Raw(RawMeta::Mut)]
            } else {
                &[PointerKind::Raw(RawMeta::Const)]
            },
        );

        self.rewrite_rvalue_at(rvalue, def_loc, stmt.source_info.span, required, rewriter);
    }
}
