use rustc_hir::{Ty, TyKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_lint_defs::Applicability;
use rustc_middle::ty::TyCtxt;
use rustc_session::{declare_tool_lint, impl_lint_pass};
use utils::compiler_interface::Program;

use crate::{
    flow::ownership::{analyse, constraint::OwnershipToken, AnalysisResult, Ownership},
    type_qualifier::{
        flow_insensitive::mutability::mutability_analysis, output_params::compute_output_params,
    },
};

declare_tool_lint! {
    pub lint::RAW_POINTER_PERMISSION,
    Warn,
    "suggest if pointer permissions are understood",
    report_in_external_macro: false
}
impl_lint_pass!(RawPointerPermission => [RAW_POINTER_PERMISSION]);

pub struct RawPointerPermission {
    analysis_result: AnalysisResult,
}

fn ownership_type_to_type_str(ty: &Ty, tcx: TyCtxt, ownership: Vec<Ownership>) -> String {
    let inner_span = {
        let mut ty = ty;
        loop {
            let TyKind::Ptr(mut_ty) = ty.kind else {
                break ty.span;
            };
            ty = mut_ty.ty;
        }
    };
    let inner_repr = tcx.sess.source_map().span_to_snippet(inner_span).unwrap();
    let mut type_str = inner_repr;
    for ownership in ownership.into_iter().rev() {
        type_str = match ownership {
            Ownership::Owning => format!("Option<Box<{}>>", type_str),
            Ownership::Transient => format!("*mut {}", type_str),
        }
    }
    type_str
}

impl RawPointerPermission {
    pub fn new(tcx: TyCtxt) -> Self {
        let program = Program::new(tcx);
        let mutability_result = mutability_analysis(&program);
        let output_params = compute_output_params(&program, &mutability_result);
        let analysis_result = analyse(&program, &output_params);
        Self { analysis_result }
    }

    fn ownership_type(&self, start_token: OwnershipToken, mut ty: &Ty) -> Vec<Ownership> {
        let mut ownership = vec![];
        let mut token = start_token;
        while let TyKind::Ptr(mut_ty) = ty.kind {
            ownership.push(self.analysis_result.model[token]);
            token += 1;
            ty = mut_ty.ty;
        }
        ownership
    }

    fn ownership_type_under_k_limit(
        &self,
        start_token: OwnershipToken,
        mut ty: &Ty,
        mut k_limit: usize,
    ) -> Vec<Ownership> {
        let mut ownership = vec![];
        let mut token = start_token;
        while let TyKind::Ptr(mut_ty) = ty.kind {
            if k_limit > 0 {
                ownership.push(self.analysis_result.model[token]);
                token += 1;
            } else {
                ownership.push(Ownership::Transient)
            }
            k_limit -= 1;
            ty = mut_ty.ty;
        }
        ownership
    }
}

impl LateLintPass<'_> for RawPointerPermission {
    // // We don't do this because `variant_data` does not provide a `DefId`
    fn check_struct_def(&mut self, _: &LateContext<'_>, _: &'_ rustc_hir::VariantData<'_>) {}

    fn check_body(&mut self, _: &LateContext<'_>, _: &'_ rustc_hir::Body<'_>) {}

    fn check_item(&mut self, cx: &LateContext<'_>, item: &'_ rustc_hir::Item<'_>) {
        if item.span.from_expansion() {
            return;
        }
        use rustc_hir::ItemKind::*;
        let def_id = item.owner_id.def_id.to_def_id();
        match item.kind {
            Fn(fn_sig, ..) => {
                let fn_summary = self.analysis_result.fn_sigs.get(&def_id).unwrap();
                let k_limit = fn_summary.k_limit;
                let mut suggestions = vec![];
                for (ty, param) in fn_sig
                    .decl
                    .inputs
                    .iter()
                    .zip(fn_summary.inputs.iter().copied())
                {
                    let start_token = param.input();
                    let ownership = self.ownership_type_under_k_limit(start_token, ty, k_limit);
                    if ownership
                        .iter()
                        .any(|&ownership| ownership == Ownership::Owning)
                    {
                        let type_str = ownership_type_to_type_str(ty, cx.tcx, ownership);
                        suggestions.push((ty.span, type_str));
                    }
                }

                use rustc_hir::FnRetTy;
                if let FnRetTy::Return(ty) = fn_sig.decl.output {
                    let start_token = fn_summary.output;
                    let ownership = self.ownership_type_under_k_limit(start_token, ty, k_limit);
                    if ownership
                        .iter()
                        .any(|&ownership| ownership == Ownership::Owning)
                    {
                        let type_str = ownership_type_to_type_str(ty, cx.tcx, ownership);
                        suggestions.push((ty.span, type_str));
                    }
                }
                if !suggestions.is_empty() {
                    cx.struct_span_lint(
                        RAW_POINTER_PERMISSION,
                        fn_sig.span,
                        "pointer permissions".to_owned(),
                        |diag| {
                            diag.multipart_suggestion(
                                "try using a better type",
                                suggestions,
                                Applicability::MaybeIncorrect,
                            )
                        },
                    );
                }
            }
            Struct(variant_data, _) => {
                let struct_summary = self.analysis_result.ty_summaries.get(&def_id).unwrap();
                let mut suggestions = vec![];
                for (field_def, field_summary) in variant_data
                    .fields()
                    .iter()
                    .zip(struct_summary.fields.iter().copied())
                {
                    let ownership = self.ownership_type(field_summary, field_def.ty);
                    if !ownership.is_empty()
                        && ownership
                            .iter()
                            .any(|&ownership| ownership == Ownership::Owning)
                    {
                        let type_str = ownership_type_to_type_str(field_def.ty, cx.tcx, ownership);

                        suggestions.push((field_def.ty.span, type_str));
                    }
                }
                if !suggestions.is_empty() {
                    cx.struct_span_lint(
                        RAW_POINTER_PERMISSION,
                        // field_def.span,
                        item.span,
                        "pointer permissions".to_owned(),
                        |diag| {
                            diag.multipart_suggestion(
                                "try using a better type",
                                suggestions,
                                Applicability::MaybeIncorrect,
                            )
                        },
                    );
                }
            }
            _ => {}
        }
    }
}
