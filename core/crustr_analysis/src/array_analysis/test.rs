use std::env;

use rustc_data_structures::graph::WithNumNodes;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

use crate::{
    array_analysis::CrateSummary, call_graph::CallGraph, def_use::BorrowckDefUse,
    ssa::rename::handler::LogSSAName, test::init_logger,
};

#[test]
fn test_infer_not_crash() {
    init_logger();
    for &prog in TESTS {
        compiler_interface::run_compiler_with_struct_defs_and_funcs(prog.into(), run_infer)
    }
}

#[test]
fn test_solve_not_crash_with_input_file() {
    init_logger();
    let file = env::current_dir()
        .expect("current working directory value is invalid")
        .join("src/array_analysis/test/resource/0/lib.rs");
    compiler_interface::run_compiler_with_struct_defs_and_funcs(file.into(), run_solve)
}

/// Testing `src/array_analysis/test/resource/2`
/// The call graph of this program:
/// i
/// ^
/// |
/// g ----> 
///   <---- h
/// ^
/// |
/// f
#[test]
fn test_solve() {
    init_logger();
    let file = env::current_dir()
        .expect("current working directory value is invalid")
        .join("src/array_analysis/test/resource/2/lib.rs");
    compiler_interface::run_compiler_with_struct_defs_and_funcs(file.into(), run_solve)
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

fn run_solve<'tcx>(tcx: TyCtxt<'tcx>, struct_defs: Vec<LocalDefId>, fn_dids: Vec<LocalDefId>) {
    let (bodies, adt_defs) = collect_bodies_and_adt_defs(tcx, struct_defs, fn_dids);

    let call_graph = CallGraph::new(tcx, bodies.into_iter());
    let mut crate_summary = CrateSummary::new(tcx, &adt_defs, call_graph);
    crate_summary.debug();
    crate_summary.infer::<BorrowckDefUse, LogSSAName>(LogSSAName);
    // assert_eq!(crate_summary.call_graph.num_nodes(), 1);

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
    let (bodies, adt_defs) = collect_bodies_and_adt_defs(tcx, struct_defs, fn_dids);

    let num_funcs = bodies.len();
    let call_graph = CallGraph::new(tcx, bodies.into_iter());
    let mut crate_summary = CrateSummary::new(tcx, &adt_defs, call_graph);
    crate_summary.debug();
    crate_summary.infer::<BorrowckDefUse, LogSSAName>(LogSSAName);
    assert_eq!(crate_summary.lambda_ctxt.func_ctxt.len(), num_funcs)
}

const TESTS: &'static [&'static str] = &[
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
];
