use super::*;

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
    common::test::run_compiler_with(PROGRAM.into(), |tcx, fns, structs| {
        let input = common::CrateData::new(tcx, fns, structs);
        let steensgaard = Steensgaard::new(&input);
        steensgaard.print_results();
        let s = input.structs[0];
        let pts = &steensgaard.pts;
        let f = steensgaard.struct_fields.get_index(s, 0);
        let g = steensgaard.struct_fields.get_index(s, 1);
        assert!(steensgaard.pts_targets.equiv(pts[f], pts[g]))
    })
}

#[test]
fn test2() {
    const PROGRAM: &str = "
    fn f() {
        let local = 0;
        let mut p = &local;
    }
    ";
    common::test::run_compiler_with(PROGRAM.into(), |tcx, fns, structs| {
        let input = common::CrateData::new(tcx, fns, structs);
        let steensgaard = Steensgaard::new(&input);
        steensgaard.print_results();
        let pts = &steensgaard.pts;
        let f = input.fns[0];
        let local = steensgaard.function_locals.get_index(f, 1);
        let p = steensgaard.function_locals.get_index(f, 2);
        assert_ne!(local, p);
        assert_eq!(pts[p], local);
    })
}
