use rustc_hir::intravisit;

use crate::print_struct::PrintStruct;

#[test]
fn test_print_all_struct() {
    compiler_interface::run_compiler_with_input_str_with_all_structs(
        TEST_PROGRAM,
        |tcx, struct_defs| {
            let mut vis = PrintStruct { tcx };
            for variant_data in struct_defs {
                intravisit::walk_struct_def(&mut vis, variant_data);
            }
        },
    )
}

const TEST_PROGRAM: &'static str = "
struct S {
    f: *mut *mut *mut i32,
    g: i32,
    h: *mut i32
}

struct T {
    f: *mut *const f32,
    g: *const (),
    h: f64
}

fn f() {
    assert!(true);
}";
