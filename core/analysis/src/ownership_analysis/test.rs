use std::env;

use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

use crate::{
    call_graph::CallGraph,
    ownership_analysis::{AnalysisEngine, InterSummary},
    ssa::rename::handler::LogSSAName,
    test::init_logger,
};

const TEST_FOLDER_NAMES: &[&str] = &["0"];
const TEST_RESOURCES_PATH_STR: &str = "src/ownership_analysis/test/resource/";

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

fn run_infer<'tcx>(tcx: TyCtxt<'tcx>, struct_defs: Vec<LocalDefId>, fn_dids: Vec<LocalDefId>) {
    let (bodies, adt_defs) = collect_bodies_and_adt_defs(tcx, struct_defs, fn_dids);

    let num_funcs = bodies.len();
    let call_graph = CallGraph::new(tcx, bodies.into_iter());
    let crate_summary = InterSummary::new::<_>(tcx, &adt_defs, call_graph, LogSSAName);
    // assert_eq!(crate_summary.rho_ctxt.locals.len(), num_funcs)
}

fn collect_bodies_and_adt_defs<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_defs: Vec<LocalDefId>,
    fn_dids: Vec<LocalDefId>,
) -> (Vec<rustc_hir::def_id::DefId>, Vec<rustc_hir::def_id::DefId>) {
    let bodies = fn_dids
        .iter()
        .map(|&fn_did| {
            let body = tcx.optimized_mir(fn_did);
            rustc_middle::mir::pretty::write_mir_fn(
                tcx,
                body,
                &mut |_, _| Ok(()),
                &mut std::io::stdout(),
            )
            .unwrap();
            fn_did.to_def_id()
        })
        .collect::<Vec<_>>();

    let adt_defs = struct_defs
        .into_iter()
        .map(|did| did.to_def_id())
        .collect::<Vec<_>>();

    (bodies, adt_defs)
}
