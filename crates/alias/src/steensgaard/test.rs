use crate::TaintResult;

#[test]
fn test1() {
    const PROGRAM: &str = "
    struct S {
        f: *mut i32,
        g: *mut i32,
    }
    unsafe fn f(s: *mut S) {
        let p = (*s).f;
        let q = p;
        (*s).g = q;
    }
    ";
    utils::compiler_interface::run_compiler(PROGRAM.into(), |program| {
        let input = program;
        let steensgaard = TaintResult::field_based(&input);
        steensgaard.print_results();
        let s = input.structs[0];
        let pts = &steensgaard.pts;
        let f = steensgaard.struct_fields.locations[steensgaard.struct_fields.did_idx[&s]][0];
        //.get_index(s, 0);
        let g = steensgaard.struct_fields.locations[steensgaard.struct_fields.did_idx[&s]][1];
        //.get_index(s, 1);
        assert!(steensgaard.pts_targets.equiv(pts[f], pts[g]))
    })
}
