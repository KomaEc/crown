#![feature(rustc_private)]
#![feature(bool_to_option)]
#![feature(array_windows)]

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
    call_graph::CallGraph,
    fat_thin_analysis::{CrateSummary, Lambda},
    ownership_analysis,
    ssa::rename::handler::LogSSAName,
};
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::{TyCtxt, WithOptConstParam};

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
    let mut crate_summary = CrateSummary::new::<_>(tcx, &adt_defs, call_graph, LogSSAName);

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
                    crate_summary
                        .source_data_to_str(crate_summary.lambda_ctxt.source_map[lambda].clone())
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
        /*
        let fn_did = WithOptConstParam::unknown(fn_did);
        let fn_did = fn_did.clone().try_upgrade(tcx).unwrap_or(fn_did);
        let body = tcx.mir_drops_elaborated_and_const_checked(fn_did);
        rustc_middle::mir::pretty::write_mir_fn(
            tcx,
            &*body.borrow(),
            &mut |_, _| Ok(()),
            &mut std::io::stdout(),
        )
        .unwrap();
        */

        let body = tcx.optimized_mir(fn_did);
        rustc_middle::mir::pretty::write_mir_fn(
            tcx,
            body,
            &mut |_, _| Ok(()),
            &mut std::io::stdout(),
        )
        .unwrap();

        // fn_did.to_def_id()
    });
}

pub fn show_ownership_analysis_results<'tcx>(
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
    let mut ownership_analysis =
        ownership_analysis::InterSummary::new(tcx, &adt_defs, call_graph, LogSSAName);

    match ownership_analysis.resolve() {
        Ok(()) => {
            ownership_analysis.show_result();
        }
        Err(reason) => {
            log::error!("Cannot solve ownership constraints!");

            assert!(reason.len() >= 2);
            assert_eq!(
                reason[0],
                <ownership_analysis::Rho as analysis::UnitAnalysisCV>::ONE
            );
            assert_eq!(
                *reason.last().unwrap(),
                <ownership_analysis::Rho as analysis::UnitAnalysisCV>::ZERO
            );

            log::debug!("A chain of inequalities that leads to this conflict:");
            for &[x, y] in reason.array_windows() {
                log::debug!("{:?} ≤ {:?}", x, y)
            }
        }
    }
}
