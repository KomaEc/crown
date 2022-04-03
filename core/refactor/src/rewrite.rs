mod rewrite_body;

use analysis::{
    call_graph::Func, fat_thin_analysis, ownership_analysis,
    required_mutability::required_mutability,
};
use rewriter::{RewriteMode, Rewriter};
use rustc_hir::{def_id::LocalDefId, FnRetTy, FnSig, ItemKind};
use rustc_index::bit_set::BitSet;
use rustc_middle::{mir::Local, ty::TyCtxt};

use self::rewrite_body::rewrite_fn_body;

pub fn rewrite(
    tcx: TyCtxt<'_>,
    ownership_analysis: &ownership_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    fn_defs: &[LocalDefId],
    struct_defs: &[LocalDefId],
    rewrite_mode: RewriteMode,
) {
    let mut rewriter = rewriter::Rewriter::default();
    rewrite_structs(tcx, &mut rewriter, ownership_analysis, fatness_analysis, struct_defs);
    rewrite_functions(tcx, &mut rewriter, ownership_analysis, fatness_analysis, fn_defs);

    rewriter.write(rewrite_mode)
}

fn rewrite_functions(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    dids: &[LocalDefId],
) {
    for &did in dids {
        let func = ownership_analysis
            .call_graph
            .lookup_function(&did.to_def_id())
            .unwrap();
        let item = tcx.hir().expect_item(did);
        if let ItemKind::Fn(sig, _generics, body_id) = &item.kind {
            let required_mutability = required_mutability(tcx, did);
            rewrite_fn_sig(
                tcx,
                rewriter,
                ownership_analysis,
                func,
                &required_mutability,
                sig,
            );
            rewrite_fn_body(
                tcx,
                rewriter,
                ownership_analysis,
                func,
                &required_mutability,
                did,
            );
        } else {
            unreachable!()
        }
    }
}

fn rewrite_fn_sig(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    func: Func,
    required_mutability: &BitSet<Local>,
    sig: &FnSig,
) {
    let mut ownership_sig_iter = ownership_analysis.func_sigs[func].sig.iter();
    let mut idx = Local::from_u32(0);
    let ret_values = ownership_sig_iter.next().unwrap();
    if let FnRetTy::Return(ret_ty) = sig.decl.output {
        for (ty, &value) in (HirPtrTypeWalker { ty: ret_ty }).zip(ret_values.iter()) {
            if let Some(known) = value {
                let ownership =
                    known
                        .then(|| Ownership::Owning)
                        .unwrap_or_else(|| Ownership::Transient {
                            mutbl: required_mutability.contains(idx),
                        });
                rewrite_raw_ptr_ty(tcx, rewriter, ty, ownership, false);
            }
        }
    }

    idx = idx + 1;
    for (arg, values) in sig.decl.inputs.iter().zip(ownership_sig_iter) {
        for (ty, &value) in (HirPtrTypeWalker { ty: arg }).zip(values.iter()) {
            if let Some(known) = value {
                let ownership =
                    known
                        .then(|| Ownership::Owning)
                        .unwrap_or_else(|| Ownership::Transient {
                            mutbl: required_mutability.contains(idx),
                        });
                rewrite_raw_ptr_ty(tcx, rewriter, ty, ownership, false);
            }
        }
        idx = idx + 1;
    }
}

fn rewrite_structs(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
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
                fatness_analysis,
                &owning_field_def,
                variant_data,
                did,
                &mut default_impl,
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
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    owning_field_def: &[ownership_analysis::Rho],
    variant_data: &rustc_hir::VariantData,
    did: LocalDefId,
    default_impl_body: &mut String,
) {
    use std::fmt::Write;
    const PREFIX: &str = "            ";
    for (field, (field_rhos, field_lambdas)) in variant_data
        .fields()
        .iter()
        .zip(ownership_analysis.inter_ctxt.field_defs[&did][0usize.into()].iter().zip(fatness_analysis.lambda_ctxt.field_defs[&did][0usize.into()].iter()))
    {
        for (ty, (rho, lambda)) in (HirPtrTypeWalker { ty: field.ty }).zip(field_rhos.clone().zip(field_lambdas.clone())) {
            // for structs, we only rewrite owning pointers

            let ownership = owning_field_def.contains(&rho).then_some(Ownership::Owning).unwrap_or(Ownership::Raw);
            let fatness = fatness_analysis.lambda_ctxt.assumptions[lambda].map(|value| value).unwrap_or(false);
            rewrite_raw_ptr_ty(tcx, rewriter, ty, ownership, fatness)
        }

        if !field_rhos.is_empty() {
            if owning_field_def.contains(&field_rhos.start) {
                writeln!(default_impl_body, "{PREFIX}{}: None,", field.ident).unwrap();
            } else {
                let mutbl_suffix = match &field.ty.kind {
                    rustc_hir::TyKind::Ptr(inner) => match inner.mutbl {
                        rustc_ast::Mutability::Mut => "_mut",
                        _ => "",
                    },
                    _ => unreachable!(),
                };
                writeln!(
                    default_impl_body,
                    "{PREFIX}{}: std::ptr::null{mutbl_suffix}(),",
                    field.ident
                )
                .unwrap();
            }
        } else {
            writeln!(
                default_impl_body,
                "{PREFIX}{}: Default::default(),",
                field.ident
            )
            .unwrap();
        }
    }
}

fn rewrite_raw_ptr_ty(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ty: &rustc_hir::Ty,
    ownership: Ownership,
    fat: bool,
) {

    if let rustc_hir::TyKind::Ptr(inner) = &ty.kind {
        let prefix_span = ty.span.until(inner.ty.span);

        // the order of rewriting prefix matters. Own rewrite a range
        // that may cover Fatness. So this does not work
        /*
        if fat {
            rewriter.make_suggestion(
                tcx, 
                inner.ty.span.shrink_to_lo(), 
                "adding slice prefix".to_owned(), 
                "[".to_owned()
            );
            debug_span_rewrited(prefix_span.shrink_to_hi());
            rewriter.make_suggestion(
                tcx, 
                inner.ty.span.shrink_to_hi(), 
                "adding slice suffix".to_owned(), 
                "]".to_owned()
            );
            debug_span_rewrited(inner.ty.span.shrink_to_hi());
        }

        match ownership {
            Ownership::Owning => {
                rewriter.make_suggestion(
                    tcx,
                    prefix_span,
                    "rewriting raw into box".to_owned(),
                    OWNING_PTR_PREFIX.to_owned(),
                );

                rewriter.make_suggestion(
                    tcx,
                    ty.span.shrink_to_hi(),
                    "adding suffix".to_owned(),
                    OWNING_PTR_SUFFIX.to_owned(),
                );
            }
            Ownership::Transient { mutbl } => {
                let mutability_modifier_str = mutbl.then_some("mut ").unwrap_or("");
                rewriter.make_suggestion(
                    tcx,
                    prefix_span,
                    "rewriting raw into ref".to_owned(),
                    format!("Option<&{mutability_modifier_str}"),
                );
                rewriter.make_suggestion(
                    tcx,
                    ty.span.shrink_to_hi(),
                    "adding suffix".to_owned(),
                    ">".to_owned(),
                );
            }
            Ownership::Raw => {}
        }
        */

        if !fat && matches!(ownership, Ownership::Raw) {
            return
        }

        let mut prefix = "".to_owned();
        let mut suffix = "".to_owned();
        if fat {
            prefix.push('[');
            suffix.push(']');
        }

        match ownership {
            Ownership::Owning => {
                prefix = format!("Option<Box<{prefix}");
                suffix.push_str(">>");
            }
            Ownership::Transient { mutbl } => {
                let mutability_modifier_str = mutbl.then_some("mut ").unwrap_or("");
                prefix = format!("&{mutability_modifier_str}{prefix}");
            }
            Ownership::Raw => unreachable!()
        }

        rewriter.make_suggestion(
            tcx,
            prefix_span,
            "rewriting ptr prefix".to_owned(),
            prefix,
        );

        rewriter.make_suggestion(
            tcx,
            ty.span.shrink_to_hi(),
            "adding suffix".to_owned(),
            suffix,
        );

    } else {
        unreachable!()
    }
}

/// A simple modelling of ownership analysis result.
/// Owning pointers will be rewritten as `Option<Box<..>>`,
/// Transient pointers will be rewritten only in function signatures,
/// as `&` or `&mut` depending on the context
#[derive(Clone, Copy)]
enum Ownership {
    Owning,
    Transient { mutbl: bool },
    Raw
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
