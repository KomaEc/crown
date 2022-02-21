use rustc_hir::{intravisit, ItemKind, OwnerNode};

use crate::print_struct::PrintStruct;

#[test]
fn test_print_all_struct() {
    compiler_interface::run_compiler_with_all_structs(TEST_PROGRAM.into(), |tcx, struct_defs| {
        let mut vis = PrintStruct { tcx };
        for did in struct_defs {
            let owner_info = tcx.hir().krate().owners[did].as_owner().unwrap();
            if let OwnerNode::Item(item) = owner_info.node() {
                if let ItemKind::Struct(variant_data, _generics) = &item.kind {
                    intravisit::walk_struct_def(&mut vis, variant_data);
                }
            }
        }
    })
}

#[test]
fn test_print_all_struct_mir() {
    /*
    compiler_interface::run_compiler_with_input_str_with_single_func(TEST_PROGRAM, |tcx, fn_did| {
        let body = tcx.optimized_mir(fn_did);
        let mut vis = PrintStructMir { tcx };
        vis.visit_body(body)
    })
    */
    compiler_interface::run_compiler_with_all_structs(TEST_PROGRAM.into(), |tcx, struct_dids| {
        for did in struct_dids {
            let adt_def = tcx.adt_def(did);
            let ty = tcx.type_of(did);
            println!("visiting adt def {:?} of type {}", adt_def, ty);
            let variant = &adt_def.variants.iter().next().unwrap();
            for field_def in &variant.fields {
                println!(
                    "with field {}: {}",
                    field_def.name,
                    tcx.type_of(field_def.did)
                );
                let walk = tcx
                    .type_of(field_def.did)
                    .walk()
                    .map(|ty| format!("{}", ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("... decomposed as {}", walk)
            }
        }
    })
}

const TEST_PROGRAM: &'static str = "
struct S {
    f: *mut *mut *mut i32,
    g: i32,
    h: *mut i32,
    s: *mut S,
}

struct T {
    f: *mut *const i32,
    g: *const (),
    h: f64
}

unsafe fn g(x: *mut S, y: *mut T, mut z: *mut S) {
    *(*y).f = **(*x).f;
    (*x).s = z;
}";
