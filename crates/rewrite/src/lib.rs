#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(if_let_guard)]

mod collect;
mod decision;
mod transform;

use analyses::borrow::PromotedMutRefs as PromotedMutRefResult;
use analyses::output_params::OutputParams as OutputParamResult;
use clap::Args;
use rustc_ast::mut_visit::MutVisitor;
use utils::ast_util::{TransformationResult, transform_ast};
use utils::ir_util::{HirToThir, IrMappings};
use utils::rustc::RustProgram;

use crate::transform::TransformVisitor;

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_ast_ir;
extern crate rustc_ast_pretty;
extern crate rustc_const_eval;
extern crate rustc_data_structures;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_parse;
extern crate rustc_passes;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_type_ir;
extern crate smallvec;
extern crate thin_vec;

#[derive(Args, Clone, Debug)]
pub struct RewriteOptions {}

pub struct Analysis {
    output_param_result: OutputParamResult,
    promoted_mut_ref_result: PromotedMutRefResult,
}

impl Analysis {
    pub fn new(
        output_param_result: OutputParamResult,
        promoted_mut_ref_result: PromotedMutRefResult,
    ) -> Self {
        Analysis {
            output_param_result,
            promoted_mut_ref_result,
        }
    }
}

pub fn rewrite<'tcx>(
    dir: &std::path::Path,
    rust_program: &RustProgram<'tcx>,
    hir_to_thir: HirToThir,
    analysis: &Analysis,
) -> TransformationResult {
    transform_ast(
        move |krate, ast_to_hir| {
            let ir_mappings = IrMappings {
                ast_to_hir: &ast_to_hir,
                hir_to_thir: &hir_to_thir,
            };
            let mut transform_visitor = TransformVisitor::new(rust_program, analysis, ir_mappings);
            transform_visitor.visit_crate(krate);
            let mut post_transform_visitor = transform::post::UnnecessaryDerefRemover;
            post_transform_visitor.visit_crate(krate);
            transform_visitor.updated
        },
        dir,
        rust_program.tcx,
    )
}
