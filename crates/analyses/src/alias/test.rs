use crate::alias::{AliasResult, TaintResult};

#[test]
fn test_simple_program() {
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
    fn main() {}
    ";
    utils::rustc::run_compiler(PROGRAM, |program| {
        let input = program;
        let steensgaard = TaintResult::field_based(&input);
        // steensgaard.print_results();
        let s = input.structs[0];
        let pts = &steensgaard.pts;
        let f = steensgaard.struct_fields.memory_location(&s, 0);
        let g = steensgaard.struct_fields.memory_location(&s, 1);
        assert!(steensgaard.pts_targets.equiv(pts[f], pts[g]))
    })
}

/// TODO add some property based testing for libtree
#[test]
fn smoke_test_libtree() {
    utils::rustc::run_compiler(utils::rustc::SourceCode::Libtree, |program| {
        let _steensgaard = AliasResult::field_insensitive(&program);
        // println!("{}", steensgaard.pretty(program.tcx));
    })
}
