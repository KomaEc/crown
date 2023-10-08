use utils::compiler_interface::run_compiler;

use crate::flow::ownership::{access_path::AccessPaths, build_engine};


#[test]
/// Sanity check `build_engine`
fn sanity_test_0() {
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
        let body = tcx.optimized_mir(program.fns.first().unwrap());
        let _ = build_engine(body, tcx, &access_paths);
    })
}

