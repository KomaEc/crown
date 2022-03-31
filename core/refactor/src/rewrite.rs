use analysis::{
    call_graph::CallGraph,
    fat_thin_analysis::{CrateSummary, Lambda},
    ownership_analysis,
    ssa::rename::handler::LogSSAName,
};
use rewriter::{RewriteMode, Rewriter};
use rustc_hir::{def_id::LocalDefId, ItemKind};
use rustc_middle::ty::TyCtxt;

pub fn rewrite(
    tcx: TyCtxt<'_>,
    ownership_analysis: &ownership_analysis::InterSummary,
    fn_defs: &[LocalDefId],
    struct_defs: &[LocalDefId],
    rewrite_mode: RewriteMode,
) {
    let mut rewriter = rewriter::Rewriter::default();
    rewrite_structs(tcx, &mut rewriter, ownership_analysis, struct_defs);

    rewriter.write(rewrite_mode)
}

fn rewrite_structs(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    dids: &[LocalDefId],
) {
    use std::fmt::Write;
    let owning_field_def = ownership_analysis.field_def_known_to_be_owning();
    for &did in dids {
        let item = tcx.hir().expect_item(did);
        if let ItemKind::Struct(variant_data, _generics) = &item.kind {
            let mut default_impl = format!(
                "

impl Default for {} {{
    fn default() -> Self {{
        Self {{
",
                item.ident
            );

            rewrite_struct(
                tcx,
                rewriter,
                ownership_analysis,
                &owning_field_def,
                variant_data,
                did,
                &mut default_impl
            );

            writeln!(default_impl, "        }}").unwrap();
            writeln!(default_impl, "    }}").unwrap();
            writeln!(default_impl, "}}").unwrap();

            let struct_span = item.span;
            rewriter.make_suggestion(
                tcx,
                struct_span.shrink_to_hi(),
                format!("adding default impl for {}", item.ident),
                default_impl,
            );
        } else {
            unreachable!()
        }
    }
}

fn rewrite_struct(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    owning_field_def: &[ownership_analysis::Rho],
    variant_data: &rustc_hir::VariantData,
    did: LocalDefId,
    default_impl_body: &mut String,
) {
    use std::fmt::Write;
    const PREFIX: &str = "            ";
    for (field, field_rhos) in variant_data
        .fields()
        .iter()
        .zip(&ownership_analysis.inter_ctxt.field_defs[&did][0usize.into()])
    {
        for (ty, rho) in (HirPtrTypeWalker { ty: field.ty }).zip(field_rhos.clone()) {
            if owning_field_def.contains(&rho) {
                rewrite_raw_ptr_ty(tcx, rewriter, ty, true, false)
            }
        }

        if !field_rhos.is_empty() {
            if owning_field_def.contains(&field_rhos.start) {
                writeln!(default_impl_body, "{PREFIX}{}: None,", field.ident).unwrap();
            } else {
                writeln!(default_impl_body, "{PREFIX}{}: std::ptr::null_mut(),", field.ident).unwrap();
            }
        } else {
            writeln!(default_impl_body, "{PREFIX}{}: Default::default(),", field.ident).unwrap();
        }
    }
}

fn rewrite_raw_ptr_ty(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ty: &rustc_hir::Ty,
    owning: bool,
    _fat: bool,
) {
    const OWNING_PTR_PREFIX: &str = "Option<Box<";
    const OWNING_PTR_SUFFIX: &str = ">>";

    if owning {
        if let rustc_hir::TyKind::Ptr(inner) = &ty.kind {
            let prefix_span = ty.span.until(inner.ty.span);
            rewriter.make_suggestion(
                tcx,
                prefix_span,
                "rewriting *mut into Option<Box<".to_owned(),
                OWNING_PTR_PREFIX.to_owned(),
            );

            rewriter.make_suggestion(
                tcx,
                ty.span.shrink_to_hi(),
                "adding suffix".to_owned(),
                OWNING_PTR_SUFFIX.to_owned(),
            );
        } else {
            unreachable!()
        }
    }
}

struct HirPtrTypeWalker<'hir> {
    ty: &'hir rustc_hir::Ty<'hir>,
}

impl<'hir> Iterator for HirPtrTypeWalker<'hir> {
    type Item = &'hir rustc_hir::Ty<'hir>;

    fn next(&mut self) -> Option<Self::Item> {
        if let rustc_hir::TyKind::Ptr(inner) = &self.ty.kind {
            let ptr_ty = self.ty;
            self.ty = inner.ty;
            Some(ptr_ty)
        } else {
            None
        }
    }
}
