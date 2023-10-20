use utils::compiler_interface::run_compiler;

use crate::flow::{
    def_use::display_def_use_chain,
    ownership::{
        access_path::AccessPaths,
        constraint::{CadicalDatabase, Debug, Z3Database},
        flow_chain, Interprocedural,
    },
};

#[test]
/// Sanity check `build_engine`
fn sanity_test_0() {
    utils::tracing_setup::init_logger();
    const PROGRAM: &str = "extern \"C\" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut ();
    fn free(_: *mut ());
}
unsafe fn f() {
    let mut p = malloc(4u64) as *mut i32;
    let mut q = p;
    free(q as *mut _);
}";
    run_compiler(PROGRAM.into(), |program| {
        const K_LIMIT: usize = 3;
        let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);
        let tcx = program.tcx;
        for did in &program.fns {
            let body = tcx.optimized_mir(did);
            display_def_use_chain(body, &flow_chain(body, &access_paths, K_LIMIT))
        }
        let mut infer_ctxt: Interprocedural<Debug, _> =
            Interprocedural::new(&program, CadicalDatabase::new(), ());
        infer_ctxt.dry_run(tcx);
    })
}

#[test]
/// Sanity check `build_engine`
fn sanity_test_1() {
    utils::tracing_setup::init_logger();
    const PROGRAM: &str = "extern \"C\" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut ();
    fn free(_: *mut ());
}

unsafe fn g(r: *mut *mut *mut i32) {
    if !(**r).is_null() {
        free(**r as *mut ())
    } else {
        assert!((**r).is_null())
    }
}";
    run_compiler(PROGRAM.into(), |program| {
        const K_LIMIT: usize = 3;
        let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);
        let tcx = program.tcx;
        for did in &program.fns {
            let body = tcx.optimized_mir(did);
            display_def_use_chain(body, &flow_chain(body, &access_paths, K_LIMIT))
        }

        let mut infer_ctxt: Interprocedural<Debug, _> =
            Interprocedural::new(&program, CadicalDatabase::new(), ());
        infer_ctxt.dry_run(tcx);
    })
}

#[test]
/// Sanity check `build_engine`
fn sanity_test_2() {
    utils::tracing_setup::init_logger();
    const PROGRAM: &str = "extern \"C\" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut ();
    fn free(_: *mut ());
}

unsafe fn f(r: *mut *mut i32) {
    free(*r as *mut ());
}

unsafe fn g(mut p: *mut i32) {
    f(core::ptr::addr_of_mut!(p))
}

unsafe fn h(mut p: *mut *mut i32) {
    **p = 1;
    f(p);
    g(*p);
}
";
    run_compiler(PROGRAM.into(), |program| {
        const K_LIMIT: usize = 3;
        let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);
        let tcx = program.tcx;
        for did in &program.fns {
            let body = tcx.optimized_mir(did);
            display_def_use_chain(body, &flow_chain(body, &access_paths, K_LIMIT))
        }

        let mut infer_ctxt: Interprocedural<Debug, _> =
            Interprocedural::new(&program, CadicalDatabase::new(), ());
        infer_ctxt.dry_run(tcx);
    })
}

#[test]
/// Sanity check `build_engine`
fn sanity_test_3() {
    utils::tracing_setup::init_logger();
    const PROGRAM: &str = "extern \"C\" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut ();
    fn free(_: *mut ());
}

unsafe fn f(p: *mut i32) -> *mut i32 {
    p
}

unsafe fn g() {
    let p = malloc(4u64) as *mut _;
    let q = f(p);
    free(q as *mut ());
}";
    run_compiler(PROGRAM.into(), |program| {
        const K_LIMIT: usize = 3;
        let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);
        let tcx = program.tcx;
        for did in &program.fns {
            let body = tcx.optimized_mir(did);
            display_def_use_chain(body, &flow_chain(body, &access_paths, K_LIMIT))
        }

        let config = z3::Config::new();
        let ctx = z3::Context::new(&config);

        let infer_ctxt: Interprocedural<Debug, _> =
            Interprocedural::new(&program, Z3Database::new(&ctx), ());
        let result = infer_ctxt.run(tcx);
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.fn_sig_str(body));
        }
    })
}
