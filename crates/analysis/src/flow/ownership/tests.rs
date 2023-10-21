use utils::compiler_interface::run_compiler;

use crate::flow::ownership::{
    analyse,
    constraint::{CadicalDatabase, Debug},
    Interprocedural,
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
unsafe fn f(r: *mut *mut *mut i32) {
    free(**r as *mut ());
    free(*r as *mut ());
    free(r as *mut ());
}
";
    run_compiler(PROGRAM.into(), |program| {
        let result = analyse(&program);
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.body_summary_str(body));
        }
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.fn_sig_str(body));
        }
        assert_eq!(result.fn_sigs.into_values().next().unwrap().k_limit, 3);
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

unsafe fn h(r: *mut *mut *mut i32) -> *mut *mut *mut i32 {
    if !(**r).is_null() {
        free(**r as *mut ());
    } else {
        assert!((**r).is_null());
    }
    r
}";
    run_compiler(PROGRAM.into(), |program| {
        let mut infer_ctxt: Interprocedural<Debug, _> =
            Interprocedural::new(&program, CadicalDatabase::new(), ());
        infer_ctxt.dry_run(program.tcx);
        assert!(matches!(infer_ctxt.database.solver.solve(), Some(true)));
        let result = analyse(&program);
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.body_summary_str(body));
        }
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.fn_sig_str(body));
        }
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
        let mut infer_ctxt: Interprocedural<Debug, _> =
            Interprocedural::new(&program, CadicalDatabase::new(), ());
        infer_ctxt.dry_run(program.tcx);
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
        let result = analyse(&program);
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.body_summary_str(body));
        }
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.fn_sig_str(body));
        }
    })
}

#[test]
/// Sanity check `build_engine`
fn sanity_test_4() {
    utils::tracing_setup::init_logger();
    const PROGRAM: &str = "extern \"C\" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut ();
    fn free(_: *mut ());
}

#[inline(never)]
unsafe fn f(p: *mut *mut i32) -> *mut *mut i32 {
    p
}

unsafe fn g() {
    let mut p = malloc(8u64) as *mut *mut _;
    *p = malloc(4u64) as *mut i32;
    let mut q = f(p);
    free(*q as *mut ());
    free(p as *mut ());
}";
    run_compiler(PROGRAM.into(), |program| {
        let result = analyse(&program);
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.body_summary_str(body));
        }
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.fn_sig_str(body));
        }
    })
}

#[test]
/// Sanity check `build_engine`
fn sanity_test_5() {
    utils::tracing_setup::init_logger();
    const PROGRAM: &str = "extern \"C\" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut ();
    fn free(_: *mut ());
}

#[inline(never)]
unsafe fn f() -> *mut i32 {
    let r = malloc(4u64) as *mut _;
    r
}

unsafe fn g(p: *mut i32) -> *mut i32 {
    if p.is_null() {
        assert!(p.is_null());
        return f();
    }
    p
}";
    run_compiler(PROGRAM.into(), |program| {
        // let mut infer_ctxt: Interprocedural<Debug, _> =
        //     Interprocedural::new(&program, CadicalDatabase::new(), ());
        // infer_ctxt.dry_run(program.tcx);
        // assert!(matches!(infer_ctxt.database.solver.solve(), Some(true)));
        let result = analyse(&program);
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.body_summary_str(body));
        }
        for body in program.bodies() {
            print!("{}: ", program.tcx.def_path_str(body.source.def_id()));
            println!("{}", result.fn_sig_str(body));
        }
    })
}
