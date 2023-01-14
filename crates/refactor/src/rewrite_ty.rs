use common::rewrite::Rewrite;
use rustc_hir::{def_id::DefId, Item, ItemKind};
use rustc_middle::ty::TyCtxt;
use smallvec::SmallVec;

use crate::{HirTyExt, PointerKind, RawMeta, StructFields};

pub fn rewrite_structs(
    structs: &[DefId],
    struct_decision: &StructFields,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
    type_reconstruction: bool,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    let mut erased_version = 0;

    for did in structs {
        let fields_data = struct_decision.field_data(did);
        let item = tcx.hir().expect_item(did.expect_local());
        let mut default_impl_block = String::new();
        writeln!(
            default_impl_block,
            "\nimpl Default for {} {{fn default() -> Self {{Self {{",
            item.ident
        )?;
        rewrite_struct(
            item,
            fields_data,
            &mut default_impl_block,
            rewriter,
            tcx,
            type_reconstruction,
        )?;

        writeln!(default_impl_block, "}}}}}}").unwrap();

        let struct_span = item.span;

        let is_owning = fields_data.iter().any(|field| {
            field
                .iter()
                .any(|ptr_kind| ptr_kind.is_move() || ptr_kind.is_raw_move())
        });
        if is_owning {
            rewriter.replace(
                tcx,
                struct_span.shrink_to_lo(),
                format!("struct ErasedByRefactorer{erased_version};\n#[repr(C)]\n"),
            );
            erased_version += 1;

            writeln!(
                default_impl_block,
                "impl {} {{pub fn take(&mut self) -> Self {{core::mem::take(self)}}}}",
                item.ident
            )
            .unwrap();
        }

        rewriter.replace(tcx, struct_span.shrink_to_hi(), default_impl_block);
    }
    Ok(())
}

pub fn rewrite_struct<'hir>(
    r#struct: &Item<'hir>,
    decision: &[SmallVec<[PointerKind; 3]>],
    default_impl_body: &mut String,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'hir>,
    type_reconstruction: bool,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    let ItemKind::Struct(variant_data, _generics) = &r#struct.kind else { panic!() };
    for (field, decision) in itertools::izip!(variant_data.fields(), decision) {
        rewrite_hir_ty(field.ty, decision, rewriter, tcx, type_reconstruction);
        // let field_ty = try_dealias(field.ty, tcx).unwrap_or(field.ty);
        let mut field_ty = field.ty;
        while let Some(ty) = try_dealias(field_ty, tcx) {
            field_ty = ty;
        }

        if let rustc_hir::TyKind::Ptr(pointee) = &field_ty.kind {
            let decision = decision.first().unwrap();
            if decision.is_raw() {
                match pointee.mutbl {
                    rustc_ast::Mutability::Mut => {
                        writeln!(default_impl_body, "{}: std::ptr::null_mut(),", field.ident)?;
                    }
                    rustc_ast::Mutability::Not => {
                        writeln!(default_impl_body, "{}: std::ptr::null(),", field.ident)?;
                    }
                }
            } else {
                writeln!(default_impl_body, "{}: None,", field.ident)?;
            }
        } else {
            writeln!(default_impl_body, "{}: Default::default(),", field.ident)?;
        }
    }

    Ok(())
}

pub fn rewrite_outermost_ptr_ty(
    ty: &rustc_hir::Ty,
    decision: PointerKind,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
) {
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
            rewriter.replace(tcx, qualifier_span, "Option<&mut ".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">".to_owned());
        }
        PointerKind::Const => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "Option<& ".to_owned());
            let end_span = ty.span.shrink_to_hi();
            rewriter.replace(tcx, end_span, ">".to_owned());
        }
        PointerKind::Raw(RawMeta::Const) => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "*const ".to_owned());
        }
        PointerKind::Raw(..) => {}
    }
}

fn try_dealias<'hir>(
    ty: &rustc_hir::Ty<'hir>,
    tcx: TyCtxt<'hir>,
) -> Option<&'hir rustc_hir::Ty<'hir>> {
    if let rustc_hir::TyKind::Path(rustc_hir::QPath::Resolved(_, path)) = ty.kind {
        let res = path.res;
        if let rustc_hir::def::Res::Def(rustc_hir::def::DefKind::TyAlias, did) = res {
            if let Some(local_did) = did.as_local() {
                let item = tcx
                    .hir()
                    .get_by_def_id(local_did)
                    .as_owner()
                    .unwrap()
                    .expect_item();
                let rustc_hir::ItemKind::TyAlias(aliased_ty, _) = &item.kind else { unreachable!() };
                return Some(*aliased_ty);
            }
        }
    }
    None
}

fn retype<'hir>(ty: &rustc_hir::Ty<'hir>, decision: &[PointerKind], tcx: TyCtxt<'hir>) -> String {
    use rustc_hir as hir;
    match ty.kind {
        hir::TyKind::Slice(ty) => {
            format!("[{}]", retype(ty, decision, tcx))
        }
        hir::TyKind::Ptr(ref mt) => match decision[0] {
            PointerKind::Move => {
                format!("Option<Box<{}>>", retype(mt.ty, &decision[1..], tcx))
            }
            PointerKind::Mut => {
                format!("Option<&mut {}>", retype(mt.ty, &decision[1..], tcx))
            }
            PointerKind::Const => {
                format!("Option<& {}>", retype(mt.ty, &decision[1..], tcx))
            }
            PointerKind::Raw(RawMeta::Const) => {
                format!("*const {}", retype(mt.ty, &decision[1..], tcx))
            }
            PointerKind::Raw(..) => {
                format!("*mut {}", retype(mt.ty, &decision[1..], tcx))
            }
        },
        hir::TyKind::Path(ref qpath) => {
            if let Some(aliased_ty) = try_dealias(ty, tcx) {
                return retype(aliased_ty, decision, tcx);
            }
            rustc_hir_pretty::qpath_to_string(qpath)
        }
        hir::TyKind::Array(ty, ref length) => {
            let hir::ArrayLen::Body(constant) = length else { unreachable!()};
            format!(
                "[{}; {}]",
                retype(ty, decision, tcx),
                rustc_hir_pretty::id_to_string(&tcx.hir(), constant.hir_id)
            )
        }
        _ => unreachable!(),
    }
}

pub fn rewrite_hir_ty<'hir>(
    ty: &rustc_hir::Ty<'hir>,
    decision: &[PointerKind],
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'hir>,
    type_reconstruction: bool,
) {
    if type_reconstruction {
        rewriter.replace(tcx, ty.span, retype(ty, decision, tcx))
    } else {
        for (raw_ptr_ty, &decision) in itertools::izip!(ty.walk_ptr(), decision) {
            rewrite_outermost_ptr_ty(raw_ptr_ty, decision, rewriter, tcx);
        }
    }
}
