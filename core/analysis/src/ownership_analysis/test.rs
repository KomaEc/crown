use std::env;

use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

use crate::{
    call_graph::CallGraph,
    ownership_analysis::{InterSummary, Rho},
    ssa::rename::handler::LogSSAName,
    test::init_logger,
    UnitAnalysisCV,
};

const TEST_FOLDER_NAMES: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7"];
const TEST_RESOURCES_PATH_STR: &str = "src/ownership_analysis/test/resource/";

#[test]
fn test_specific() {
    init_logger();
    let lib = env::current_dir()
        .expect("current working directory value is invalid")
        .join(TEST_RESOURCES_PATH_STR)
        .join("1/lib.rs");
    compiler_interface::run_compiler_with_struct_defs_and_funcs(
        lib.into(),
        |tcx, struct_defs, fn_dids| {
            let call_graph = CallGraph::new(tcx, fn_dids.into_iter().map(|did| did.to_def_id()));
            let mut crate_summary =
                InterSummary::new::<_>(tcx, &struct_defs, call_graph, LogSSAName);

            let func_we_care = crate::call_graph::Func::from_u32(0);

            crate_summary.func_summaries[func_we_care]
                .constraint_system
                .show();
            crate_summary.func_summaries[func_we_care]
                .constraint_system
                .le_constraints
                .show();

            match crate_summary.resolve() {
                Ok(()) => {
                    for summary in &crate_summary.func_summaries {
                        summary.constraint_system.show()
                    }

                    /*
                    let src = Rho::from_u32(34);
                    let tgt = Rho::from_u32(0);
                    log::debug!("Explaining {:?} ≤ {:?}", src, tgt);
                    let intra_summary = &crate_summary.func_summaries[func_we_care];
                    for &[x, y] in intra_summary
                        .constraint_system
                        .le_constraints
                        .explain(src, tgt)
                        .array_windows()
                    {
                        log::debug!("{:?} ≤ {:?}", x, y)
                    }
                    */
                }
                Err(reason) => {
                    log::error!("Cannot solve ownership constraints!");
                    assert!(reason.len() >= 2);
                    assert_eq!(reason[0], Rho::ONE);
                    assert_eq!(*reason.last().unwrap(), Rho::ZERO);

                    log::debug!("A chain of inequalities that leads to this conflict:");
                    for &[x, y] in reason.array_windows() {
                        log::debug!("{:?} ≤ {:?}", x, y)
                    }
                }
            }
        },
    )
}

#[test]
fn test_infer_not_crash() {
    init_logger();
    for &folder in TEST_FOLDER_NAMES {
        let file = env::current_dir()
            .expect("current working directory value is invalid")
            .join(TEST_RESOURCES_PATH_STR)
            .join(folder)
            .join("lib.rs");
        compiler_interface::run_compiler_with_struct_defs_and_funcs(file.into(), run_infer)
    }
}

#[test]
fn test_solve_not_crash() {
    init_logger();
    for &folder in TEST_FOLDER_NAMES {
        let file = env::current_dir()
            .expect("current working directory value is invalid")
            .join(TEST_RESOURCES_PATH_STR)
            .join(folder)
            .join("lib.rs");
        compiler_interface::run_compiler_with_struct_defs_and_funcs(file.into(), run_solve)
    }
}

fn run_infer<'tcx>(tcx: TyCtxt<'tcx>, struct_defs: Vec<LocalDefId>, fn_dids: Vec<LocalDefId>) {
    let call_graph = CallGraph::new(tcx, fn_dids.into_iter().map(|did| did.to_def_id()));
    InterSummary::new::<_>(tcx, &struct_defs, call_graph, LogSSAName);
}

fn run_solve<'tcx>(tcx: TyCtxt<'tcx>, struct_defs: Vec<LocalDefId>, fn_dids: Vec<LocalDefId>) {
    let call_graph = CallGraph::new(tcx, fn_dids.into_iter().map(|did| did.to_def_id()));
    let mut crate_summary = InterSummary::new::<_>(tcx, &struct_defs, call_graph, LogSSAName);

    match crate_summary.resolve() {
        Ok(()) => {
            for summary in &crate_summary.func_summaries {
                summary.constraint_system.show()
            }
        }
        Err(reason) => {
            log::error!("Cannot solve ownership constraints!");
            assert!(reason.len() >= 2);
            assert_eq!(reason[0], Rho::ONE);
            assert_eq!(*reason.last().unwrap(), Rho::ZERO);

            log::debug!("A chain of inequalities that leads to this conflict:");
            for &[x, y] in reason.array_windows() {
                log::debug!("{:?} ≤ {:?}", x, y)
            }
        }
    }
    // assert_eq!(crate_summary.rho_ctxt.locals.len(), num_funcs)
}
