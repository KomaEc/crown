#![feature(rustc_private)]
#![feature(bool_to_option)]
#![feature(array_windows)]
#![feature(box_patterns)]

pub mod rewrite;

extern crate either;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

use std::process::exit;

use analysis::{
    call_graph::CallGraph,
    fat_thin_analysis::{self, CrateSummary, Lambda},
    ownership_analysis,
    mutability_analysis,
    ssa::rename::handler::LogSSAName,
};
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

pub fn print_fat_thin_analysis_results<'tcx>(
    tcx: TyCtxt<'tcx>,
    structs: Vec<LocalDefId>,
    funcs: Vec<LocalDefId>,
) {
    let call_graph = CallGraph::new(tcx, funcs.into_iter().map(|did| did.to_def_id()));
    let mut crate_summary = CrateSummary::new::<_>(tcx, &structs, call_graph, LogSSAName);

    match crate_summary.iterate_to_fixpoint() {
        Ok(()) => {
            let solutions = crate_summary.lambda_ctxt.assumptions.clone();

            // log::debug!("All constraints:");
            println!("All constraints:");
            for constraint in crate_summary.constraints.iter() {
                // log::debug!("{}", constraint)
                println!("{}", constraint)
            }

            for (lambda, solution) in solutions.into_iter_enumerated() {
                // log::debug!(
                println!(
                    "{: <7} = {: <2} at {}",
                    &format!("{:?}", lambda),
                    solution
                        .map(|fat| { fat.then_some("1").unwrap_or("0") })
                        .unwrap_or("?"),
                    // crate_summary.lambda_ctxt.lambda_map.data_map[lambda]
                    crate_summary.source_data_to_str(
                        tcx,
                        crate_summary.lambda_ctxt.source_map[lambda].clone()
                    )
                )
            }
        }
        Err(_) => {
            // log::debug!("Solve failed!");
            println!("Solve failed!");
            // log::debug!("Global context:");
            println!("Global context:");
            let solutions = crate_summary.lambda_ctxt.assumptions;
            for (lambda, solution) in solutions.raw[crate_summary.globals.clone()]
                .iter()
                .enumerate()
            {
                let lambda = Lambda::from(lambda);
                // log::debug!(
                println!(
                    "{: <7} = {: <2} at {}",
                    &format!("{:?}", lambda),
                    solution
                        .map(|fat| { fat.then_some("1").unwrap_or("0") })
                        .unwrap_or("?"),
                    crate_summary.lambda_ctxt.source_map[lambda]
                )
            }
        }
    }
}

pub fn show_mir<'tcx>(tcx: TyCtxt<'tcx>, funcs: Vec<LocalDefId>) {
    funcs.iter().for_each(|&fn_did| {
        let body = tcx.optimized_mir(fn_did);
        rustc_middle::mir::pretty::write_mir_fn(
            tcx,
            body,
            &mut |_, _| Ok(()),
            &mut std::io::stdout(),
        )
        .unwrap();
    });
}

pub fn show_ownership_analysis_results<'tcx>(
    tcx: TyCtxt<'tcx>,
    structs: Vec<LocalDefId>,
    funcs: Vec<LocalDefId>,
) {
    let call_graph = CallGraph::new(tcx, funcs.into_iter().map(|did| did.to_def_id()));
    let mut ownership_analysis =
        ownership_analysis::InterSummary::new(tcx, &structs, call_graph, LogSSAName);

    match ownership_analysis.resolve() {
        Ok(()) => {
            ownership_analysis.show_result();
        }
        Err(reason) => {
            log::error!("Cannot solve ownership constraints!");

            ownership_analysis::explain_error(reason)
        }
    }
}

pub fn show_mutability_analysis_results<'tcx>(
    tcx: TyCtxt<'tcx>,
    structs: Vec<LocalDefId>,
    funcs: Vec<LocalDefId>,
) {
    let call_graph = CallGraph::new(tcx, funcs.into_iter().map(|did| did.to_def_id()));
    let mut mutability_analysis =
        mutability_analysis::InterSummary::new(tcx, &structs, call_graph, LogSSAName);

    match mutability_analysis.resolve() {
        Ok(()) => {
            mutability_analysis.show_result();
        }
        Err(reason) => {
            log::error!("Cannot solve ownership constraints!");

            mutability_analysis::explain_error(reason)
        }
    }
}

pub fn fatness_analysis(
    tcx: TyCtxt<'_>,
    structs: &[LocalDefId],
    funcs: &[LocalDefId],
) -> fat_thin_analysis::CrateSummary {
    let call_graph = CallGraph::new(tcx, funcs.into_iter().map(|did| did.to_def_id()));
    let mut fatness_analysis =
        fat_thin_analysis::CrateSummary::new::<_>(tcx, &structs, call_graph, LogSSAName);
    match fatness_analysis.iterate_to_fixpoint() {
        Ok(()) => fatness_analysis,
        Err(_) => {
            log::error!("Cannot solve fatness constraints!");
            exit(0)
        }
    }
}

pub fn ownership_analysis(
    tcx: TyCtxt<'_>,
    structs: &[LocalDefId],
    funcs: &[LocalDefId],
) -> ownership_analysis::InterSummary {
    let call_graph = CallGraph::new(tcx, funcs.into_iter().map(|did| did.to_def_id()));
    let mut ownership_analysis =
        ownership_analysis::InterSummary::new(tcx, &structs, call_graph, LogSSAName);
    match ownership_analysis.resolve() {
        Ok(()) => ownership_analysis,
        Err(reason) => {
            log::error!("Cannot solve ownership constraints!");

            ownership_analysis::explain_error(reason);

            exit(0)
        }
    }
}

pub fn mutability_analysis(
    tcx: TyCtxt<'_>,
    structs: &[LocalDefId],
    funcs: &[LocalDefId],
) -> mutability_analysis::InterSummary {
    let call_graph = CallGraph::new(tcx, funcs.into_iter().map(|did| did.to_def_id()));
    let mut mutability_analysis =
    mutability_analysis::InterSummary::new(tcx, &structs, call_graph, LogSSAName);
    match mutability_analysis.resolve() {
        Ok(()) => mutability_analysis,
        Err(reason) => {
            log::error!("Cannot solve ownership constraints!");

            mutability_analysis::explain_error(reason);

            exit(0)
        }
    }
}