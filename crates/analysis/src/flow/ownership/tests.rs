use utils::compiler_interface::run_compiler;

use crate::flow::{
    def_use::display_uses,
    ownership::{access_path::AccessPaths, flow_chain},
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

unsafe fn f(r: *mut i32) {
    let mut p = malloc(8u64) as *mut *mut *mut i32;
    *p = malloc(8u64) as *mut *mut i32;
    **p = r;
    ***p = 1;
    let mut q = p;
    free(*q as *mut _);
    free(*q as *mut _);
    free(q as *mut _);
}";
    run_compiler(PROGRAM.into(), |program| {
        const K_LIMIT: usize = 3;
        let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);
        let tcx = program.tcx;
        for did in &program.fns {
            let body = tcx.optimized_mir(did);
            display_uses(body, &flow_chain(body, &access_paths))
        }
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

unsafe fn f(r: *mut i32) {
    let mut p = malloc(8u64) as *mut *mut *mut i32;
    *p = malloc(8u64) as *mut *mut i32;
    **p = r;
    ***p = 1;
    let mut q = p;
    free(*q as *mut _);
    free(*q as *mut _);
    free(q as *mut _);
}

unsafe fn g() {
    let r = malloc(4u64) as *mut i32;
    f(r);
}

";
    run_compiler(PROGRAM.into(), |program| {
        const K_LIMIT: usize = 3;
        let access_paths: AccessPaths<K_LIMIT> = AccessPaths::new(&program);
        let tcx = program.tcx;
        for did in &program.fns {
            let body = tcx.optimized_mir(did);
            display_uses(body, &flow_chain(body, &access_paths))
        }
    })
}
