use common::rewrite::Rewrite;
use rustc_middle::ty::TyCtxt;

use crate::PointerKind;

pub fn rewrite_struct() {}

pub fn rewrite_raw_ptr(
    mut ty: &rustc_hir::Ty,
    decision: PointerKind,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
) {
    while let rustc_hir::TyKind::Array(inner_ty, _) | rustc_hir::TyKind::Slice(inner_ty) = &ty.kind
    {
        ty = inner_ty;
    }
    let rustc_hir::TyKind::Ptr(pointee) = &ty.kind else { unreachable!() };

    match decision {
        PointerKind::Move => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "Option<Box<".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">>".to_owned());
        }
        PointerKind::Mut => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "Option<&mut".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">".to_owned());
        }
        PointerKind::Shr => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "Option<&".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">".to_owned());
        }
        PointerKind::Raw => {}
    }
}
