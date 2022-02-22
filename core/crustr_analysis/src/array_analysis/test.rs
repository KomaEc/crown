use std::env;

use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

use crate::{
    array_analysis::solve::solve, def_use::BorrowckDefUse, ssa::rename::handler::LogSSAName,
};

use super::CrateSummary;

#[test]
fn test_all() {
    env_logger::init();
    for (prog, spec) in TEST_PROGRAMS {
        compiler_interface::run_compiler_with_struct_defs_and_funcs(Into::into(*prog), spec)
    }
}

#[test]
fn test_file_with_extern_call() {
    env_logger::init();
    let file = env::current_dir()
        .expect("current working directory value is invalid")
        .join("src/array_analysis/test/resource/simple/lib.rs");
    compiler_interface::run_compiler_with_struct_defs_and_funcs(file.into(), solve_for_sinlge_func)
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
        print_mir_and_log_debug,
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
        print_mir_and_log_debug,
    ),
    /*
    (
        "
    unsafe fn f(q: *mut i32, r: *mut i32, cond: bool) -> *mut i32 {
        let mut local = 0;
        let mut p: *mut i32 = &mut local;
        if cond {
            p = q;
        } else {
            p = r;
        }
        return p;
    }
        ",
        print_mir_and_log_debug,
    ),
    (
        "
    fn f() -> *mut i32 {
        let i = 0 as *mut i32;
        let mut j = 1 as *mut i32;
        let mut k = 0 as *mut i32;
        while (k as usize) < 100 {
            if (j as usize) < 20 {
                j = i;
                k = ((*k)+1) as *mut i32;
            } else {
                j = k;
                k = ((*k)+2) as *mut i32;
            }
            assert!(true, \"Introduce a new block, this assertion is optimised away\")
        }
        return j
    }",
        print_mir_and_log_debug,
    ),
    */
];

fn solve_for_sinlge_func<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_defs: Vec<LocalDefId>,
    fn_dids: Vec<LocalDefId>,
) {
    let bodies = fn_dids
        .iter()
        .map(|&fn_did| {
            let body = tcx.optimized_mir(fn_did);

            let mut w = String::new();
            rustc_middle::mir::pretty::write_mir_fn(tcx, body, &mut |_, _| Ok(()), unsafe {
                &mut w.as_mut_vec()
            })
            .unwrap();
            println!("{}", w);
            fn_did.to_def_id()
        })
        .collect::<Vec<_>>();

    let adt_defs = struct_defs
        .into_iter()
        .map(|did| did.to_def_id())
        .collect::<Vec<_>>();

    let mut crate_summary = CrateSummary::new(tcx, &adt_defs, &bodies);
    crate_summary.debug();
    crate_summary.infer::<BorrowckDefUse, LogSSAName>(LogSSAName);

    assert_eq!(crate_summary.bodies.len(), 1);

    let solutions = solve(
        crate_summary.lambda_ctxt.lambda_map.assumptions,
        crate_summary.equalities,
        &crate_summary.constraints.raw,
    );

    for (lambda, solution) in solutions.into_iter_enumerated() {
        log::debug!(
            "{:?} = {}",
            lambda,
            solution
                .map(|fat| { fat.then_some("1").unwrap_or("0") })
                .unwrap_or("whatever")
        )
    }
}

fn print_mir_and_log_debug<'tcx>(
    tcx: TyCtxt<'tcx>,
    struct_defs: Vec<LocalDefId>,
    fn_dids: Vec<LocalDefId>,
) {
    let bodies = fn_dids
        .iter()
        .map(|&fn_did| {
            let body = tcx.optimized_mir(fn_did);

            let mut w = String::new();
            rustc_middle::mir::pretty::write_mir_fn(tcx, body, &mut |_, _| Ok(()), unsafe {
                &mut w.as_mut_vec()
            })
            .unwrap();
            println!("{}", w);
            fn_did.to_def_id()
        })
        .collect::<Vec<_>>();

    let adt_defs = struct_defs
        .into_iter()
        .map(|did| did.to_def_id())
        .collect::<Vec<_>>();
    // let bodies = vec![fn_did.to_def_id()];

    let mut crate_summary = CrateSummary::new(tcx, &adt_defs, &bodies);
    crate_summary.debug();
    crate_summary.infer::<BorrowckDefUse, LogSSAName>(LogSSAName);
    assert_eq!(crate_summary.lambda_ctxt.body_ctxt.len(), bodies.len())
}
