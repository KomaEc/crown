use std::env;

use rustc_data_structures::graph::WithNumNodes;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

use crate::{
    array_analysis::CrateSummary,
    call_graph::CallGraph,
    def_use::BorrowckDefUse,
    ssa::rename::handler::LogSSAName,
    test::init_logger,
};

#[test]
fn test_all_programs() {
    init_logger();
    for (prog, spec) in TEST_PROGRAMS {
        compiler_interface::run_compiler_with_struct_defs_and_funcs(Into::into(*prog), spec)
    }
}

#[test]
fn test_all_files() {
    init_logger();
    let file = env::current_dir()
        .expect("current working directory value is invalid")
        .join("src/array_analysis/test/resource/simple_struct/lib.rs");
    compiler_interface::run_compiler_with_struct_defs_and_funcs(file.into(), run_solve)
}

fn run_solve<'tcx>(tcx: TyCtxt<'tcx>, struct_defs: Vec<LocalDefId>, fn_dids: Vec<LocalDefId>) {
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

    let call_graph = CallGraph::new(tcx, bodies.into_iter());
    let mut crate_summary = CrateSummary::new(tcx, &adt_defs, call_graph);
    crate_summary.debug();
    crate_summary.infer::<BorrowckDefUse, LogSSAName>(LogSSAName);
    assert_eq!(crate_summary.call_graph.num_nodes(), 1);
    
    crate_summary.iterate_to_fixpoint().unwrap();
    let solutions = crate_summary.lambda_ctxt.lambda_map.assumptions;

    log::debug!("All constraints:");
    for constraint in crate_summary.constraints {
        log::debug!("{}", constraint)
    }

    for (lambda, solution) in solutions.into_iter_enumerated() {
        log::debug!(
            "{: <7} = {: <2}, with source data {}",
            &format!("{:?}", lambda),
            solution
                .map(|fat| { fat.then_some("1").unwrap_or("0") })
                .unwrap_or("?"),
            crate_summary.lambda_ctxt.lambda_map.data_map[lambda]
        )
    }
}

fn run_infer<'tcx>(tcx: TyCtxt<'tcx>, struct_defs: Vec<LocalDefId>, fn_dids: Vec<LocalDefId>) {
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
    // let bodies = vec![fn_did.to_def_id()];

    let num_funcs = bodies.len();
    let call_graph = CallGraph::new(tcx, bodies.into_iter());
    let mut crate_summary = CrateSummary::new(tcx, &adt_defs, call_graph);
    crate_summary.debug();
    crate_summary.infer::<BorrowckDefUse, LogSSAName>(LogSSAName);
    assert_eq!(crate_summary.lambda_ctxt.func_ctxt.len(), num_funcs)
}

const TEST_PROGRAMS: &'static [(
    &'static str,
    for<'tcx> fn(TyCtxt<'tcx>, Vec<LocalDefId>, Vec<LocalDefId>),
)] = &[
    (
        "
    struct S {
        f: *mut *mut i32,
        g: *mut i32,
        h: usize
    }
    
    unsafe fn f(mut x: *mut *mut i32, y: *mut S) {
        *(*y).f = *x;
        let z = *x;
        let w = z;
        (*y).g = w;
    }",
        run_infer,
    ),
    (
        "
    unsafe fn f(x: *mut *mut i32, y: *mut i32) {
        *x = y;
        *y = 3;
    }

    unsafe fn g(mut x: *mut i32, y: *mut *mut i32) {
        *x = 4;
        f(&mut x, *y);
    }
    ",
        run_infer,
    ),
];