use std::env;

use compiler_interface::run_compiler_with_file_with_single_func;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

use crate::def_use::BorrowckDefUse;

use super::CrateSummary;

#[test]
fn test_print_mir() {
    let working_dir_path = env::current_dir().expect("current working directory value is invalid");
    run_compiler_with_file_with_single_func(
        working_dir_path.join("src/array_analysis/test/resource/simple/lib.rs"),
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
        compiler_interface::run_compiler_with_input_str_with_struct_defs_and_single_func(prog, spec)
    }
}

const TEST_PROGRAMS: &'static [(
    &'static str,
    for<'tcx> fn(TyCtxt<'tcx>, Vec<LocalDefId>, LocalDefId),
)] = &[(
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
)];

fn print_mir_and_log_debug<'tcx>(tcx: TyCtxt<'tcx>, struct_defs: Vec<LocalDefId>, fn_did: LocalDefId) {
    let body = tcx.optimized_mir(fn_did);

    let mut w = String::new();
    rustc_middle::mir::pretty::write_mir_fn(tcx, body, &mut |_, _| Ok(()), unsafe {
        &mut w.as_mut_vec()
    })
    .unwrap();
    println!("{}", w);

    let adt_defs = struct_defs
        .into_iter()
        .map(|did| did.to_def_id())
        .collect::<Vec<_>>();
    let bodies = vec![fn_did.to_def_id()];

    let mut crate_summary = CrateSummary::new(tcx, &adt_defs, &bodies);
    crate_summary.debug();
    crate_summary.infer::<BorrowckDefUse>();
}
