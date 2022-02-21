use std::env;

use compiler_interface::Input;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

use crate::def_use::BorrowckDefUse;

use super::CrateSummary;

#[test]
fn test_print_mir() {
    let working_dir_path = env::current_dir().expect("current working directory value is invalid");
    compiler_interface::run_compiler_with_single_func(
        Input::File(working_dir_path.join("src/array_analysis/test/resource/simple/lib.rs")),
        |tcx, fn_did| {
            let body = tcx.optimized_mir(fn_did);

            let mut w = String::new();
            rustc_middle::mir::pretty::write_mir_fn(tcx, body, &mut |_, _| Ok(()), unsafe {
                &mut w.as_mut_vec()
            })
            .unwrap();
            println!("{}", w);

            // let mut intra = IntraContext::new(tcx, body);
            // intra.visit_body(body);
        },
    )
}

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
];

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
    crate_summary.infer::<BorrowckDefUse>();
    assert_eq!(crate_summary.lambda_ctxt.body_ctxt.len(), bodies.len())
}
