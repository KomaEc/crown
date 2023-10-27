use rustc_hir::TyKind;
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::ty::TyCtxt;
use rustc_session::{declare_tool_lint, impl_lint_pass};
use utils::compiler_interface::Program;

use crate::{
    flow::ownership::{analyse, AnalysisResult, Ownership},
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

impl RawPointerPermission {
    pub fn new(tcx: TyCtxt) -> Self {
        let program = Program::new(tcx);
        let mutability_result = mutability_analysis(&program);
        let output_params = compute_output_params(&program, &mutability_result);
        let analysis_result = analyse(&program, &output_params);
        Self { analysis_result }
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
            Fn(..) => {}
            Struct(variant_data, _) => {
                let field_summaries = self.analysis_result.ty_summaries.get(&def_id).unwrap();
                for (field_def, field_summary) in variant_data
                    .fields()
                    .iter()
                    .zip(field_summaries.fields.iter().copied())
                {
                    let mut token = field_summary;
                    let mut ownership = vec![];
                    let mut ty = field_def.ty;
                    while let TyKind::Ptr(mut_ty) = ty.kind {
                        ownership.push(self.analysis_result.model[token]);
                        token += 1;
                        ty = mut_ty.ty;
                    }

                    if !ownership.is_empty()
                        && ownership
                            .iter()
                            .any(|&ownership| ownership == Ownership::Owning)
                    {
                        cx.struct_span_lint(
                            RAW_POINTER_PERMISSION,
                            field_def.span,
                            format!("The permissions of this field are {:?}", ownership),
                            |diag| diag,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}
