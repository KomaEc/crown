#![feature(rustc_private)]
#![feature(bool_to_option)]

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

use analysis::{
    fat_thin_analysis::{CrateSummary, Lambda},
    call_graph::CallGraph,
    def_use::FatThinAnalysisDefUse,
    ssa::rename::handler::LogSSAName,
};
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

pub fn print_fat_thin_analysis_results<'tcx>(
    tcx: TyCtxt<'tcx>,
    structs: Vec<LocalDefId>,
    funcs: Vec<LocalDefId>,
) {
    let bodies = funcs
        .iter()
        .map(|&fn_did| {
            /*
            let body = tcx.optimized_mir(fn_did);
            rustc_middle::mir::pretty::write_mir_fn(
                tcx,
                body,
                &mut |_, _| Ok(()),
                &mut std::io::stdout(),
            )
            .unwrap();
            */
            fn_did.to_def_id()
        })
        .collect::<Vec<_>>();

    let adt_defs = structs
        .into_iter()
        .map(|did| did.to_def_id())
        .collect::<Vec<_>>();

    let call_graph = CallGraph::new(tcx, bodies.into_iter());
    let mut crate_summary =
        CrateSummary::<FatThinAnalysisDefUse>::new::<_>(tcx, &adt_defs, call_graph, LogSSAName);

    match crate_summary.iterate_to_fixpoint() {
        Ok(()) => {
            let solutions = crate_summary
                .lambda_ctxt
                .lambda_data_map
                .assumptions
                .clone();

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
                    crate_summary.lambda_source_data_to_str(
                        crate_summary.lambda_ctxt.lambda_data_map.source_map[lambda].clone()
                    )
                )
            }
        }
        Err(_) => {
            // log::debug!("Solve failed!");
            println!("Solve failed!");
            // log::debug!("Global context:");
            println!("Global context:");
            let solutions = crate_summary.lambda_ctxt.lambda_data_map.assumptions;
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
                    crate_summary.lambda_ctxt.lambda_data_map.source_map[lambda]
                )
            }
        }
    }
}
