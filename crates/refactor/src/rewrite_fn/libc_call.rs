use analysis::ssa::consume::RichLocation;
use common::rewrite::Rewrite;
use either::Either::{Left, Right};
use rustc_hir::ForeignItem;
use rustc_middle::mir::{Location, Operand, Place, StatementKind, TerminatorKind};
use rustc_span::Span;
use rustc_type_ir::TyKind::FnDef;

use super::{FnRewriteCtxt, PlaceCtxt};
use crate::{PointerKind, RawMeta};

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
        match callee.ident.as_str() {
            "malloc" => {}
            "free" => self.rewrite_free(args, fn_span, location, rewriter),
            "printf" => self.rewrite_printf(args, location, rewriter),
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

    fn rewrite_free(
        &self,
        args: &Vec<Operand<'tcx>>,
        fn_span: Span,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            local_decision,
            body,
            def_use_chain,
            ..
        } = *self;
        let arg = &args[0];
        let Some(arg) = arg.place().and_then(|place| place.as_local()) else { unreachable!() };

        if matches!(local_decision[arg.index()].first(), Some(ptr_kind) if ptr_kind.is_move()) {
            rewriter.replace(self.tcx, fn_span, "()".to_owned())
        } else {
            let def_loc = def_use_chain.def_loc(arg, location);
            let RichLocation::Mir(def_loc) = def_loc else { panic!() };
            let Left(stmt) = body.stmt_at(def_loc) else {
                    unimplemented!()
                };
            let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
            self.rewrite_rvalue_at(
                rvalue,
                def_loc,
                stmt.source_info.span,
                PlaceCtxt::Ptr(&[PointerKind::Raw(RawMeta::Mut)]),
                rewriter,
            );
        }
    }

    fn rewrite_printf(
        &self,
        args: &Vec<Operand<'tcx>>,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        for arg in args.iter().skip(1) {
            if let Some(place) = arg.place() {
                let Some(local) = place.as_local() else { panic!() };
                let ty = self.body.local_decls[local].ty;
                let required = if ty.is_unsafe_ptr() {
                    PlaceCtxt::Ptr(&[PointerKind::Raw(RawMeta::Const)])
                } else {
                    PlaceCtxt::Irrelavent
                };
                self.rewrite_temporary(local, location, required, rewriter);
            }
        }
    }

    pub fn try_rewrite_alloc_from_dest(
        &self,
        operand: &Operand<'tcx>,
        location: Location,
        needs_erase: bool,
        rewriter: &mut impl Rewrite,
    ) -> Result<Option<Span>, ()> {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            tcx,
            ..
        } = *self;

        let local = operand
            .place()
            .and_then(|place| place.as_local())
            .ok_or(())?;
        let def_loc = def_use_chain.def_loc(local, location);
        let RichLocation::Mir(def_loc) = def_loc else { return Err(()) };
        let Right(terminator) = body.stmt_at(def_loc) else { return Err(()) };
        let TerminatorKind::Call { func, fn_span, args, destination, .. } = &terminator.kind else { return Err(()) };
        let Some(func) = func.constant() else { return Err(()) };
        let ty = func.ty();
        let &FnDef(callee, _) = ty.kind() else { unreachable!() };
        let Some(local_did) = callee.as_local() else { return Err(()) };
        let rustc_hir::Node::ForeignItem(foreign_item) = tcx.hir().find_by_def_id(local_did).unwrap() else { return Err(()) };
        if matches!(foreign_item.ident.as_str(), "malloc" | "calloc") {
            if needs_erase {
                Ok(Some(*fn_span))
            } else {
                self.rewrite_call_default(callee, args, *destination, *fn_span, def_loc, rewriter);
                Ok(None)
            }
        } else {
            Err(())
        }
    }
}
