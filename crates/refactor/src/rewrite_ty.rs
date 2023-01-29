use common::rewrite::Rewrite;
use rustc_hir::{def_id::DefId, Item, ItemKind};
use rustc_middle::ty::TyCtxt;
use smallvec::SmallVec;

use crate::{HirTyExt, PointerKind, RawMeta, RefactorOptions, StructFields};

pub fn rewrite_structs(
    structs: &[DefId],
    struct_decision: &StructFields,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
    options: &RefactorOptions,
) -> anyhow::Result<()> {
    let RefactorOptions {
        type_reconstruction,
        ..
    } = *options;
    use std::fmt::Write;

    let mut erased_version = 0;

    for did in structs {
        let fields_data = struct_decision.field_data(did);
        let item = tcx.hir().expect_item(did.expect_local());

        let struct_span = item.span;

        let is_owning = struct_decision.is_owning(tcx, did);

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
        if !options.no_box {
            rewriter.replace(tcx, struct_span.shrink_to_hi(), default_impl_block);
        }
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
        } else if let rustc_hir::TyKind::Array(ty, array_len) = &field_ty.kind {
            fn array_ty_replacement(
                tcx: TyCtxt,
                ty: &rustc_hir::Ty,
                array_len: &rustc_hir::ArrayLen,
            ) -> String {
                let rustc_hir::ArrayLen::Body(constant) = array_len else { unreachable!()};

                if let rustc_hir::TyKind::Array(inner_ty, inner_array_len) = &ty.kind {
                    let inner = array_ty_replacement(tcx, inner_ty, inner_array_len);
                    format!(
                        "[{inner}; {}]",
                        rustc_hir_pretty::id_to_string(&tcx.hir(), constant.hir_id)
                    )
                } else {
                    format!(
                        "[Default::default(); {}]",
                        rustc_hir_pretty::id_to_string(&tcx.hir(), constant.hir_id)
                    )
                }
            }

            // writeln!(default_impl_body, "{}: [Default::default(); {}],", field.ident, rustc_hir_pretty::id_to_string(&tcx.hir(), constant.hir_id))?;
            writeln!(
                default_impl_body,
                "{}: {},",
                field.ident,
                array_ty_replacement(tcx, ty, array_len)
            )?;
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
        PointerKind::Raw(RawMeta::Move) => {
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "*mut /* owning */ ".to_owned());
        }
        PointerKind::Raw(..) => {
            // rewrite is still performed because it might be the case that
            // *const is actually mutable.
            let qualifier_span = ty.span.until(pointee.ty.span);
            rewriter.replace(tcx, qualifier_span, "*mut ".to_owned());
        }
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
            PointerKind::Raw(RawMeta::Move) => {
                format!("*mut /* owning */ {}", retype(mt.ty, &decision[1..], tcx))
            }
            PointerKind::Raw(RawMeta::Mut) => {
                format!("*mut {}", retype(mt.ty, &decision[1..], tcx))
            }
        },
        hir::TyKind::Path(ref qpath) => {
            let hir::QPath::Resolved(None, path) = qpath else { todo!("not supported") };
            assert!(!path.segments.is_empty());
            if "Option" == path.segments[0].ident.as_str() {
                // HACK
                // [`Option`] happens only at bare fn ptr
                let span = ty.span;
                let snippet = common::rewrite::get_snippet(tcx, span).text.1;
                return snippet;
            }
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
        hir::TyKind::Never => "!".to_owned(),
        hir::TyKind::Tup(tuples) => {
            // happens with struct ErasedByPreprocessor { dummy: () }
            assert!(tuples.is_empty());
            rustc_hir_pretty::id_to_string(&tcx.hir(), ty.hir_id)
        }
        _ => unreachable!("{:?}", ty),
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
