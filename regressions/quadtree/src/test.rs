use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor6;
pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7;
pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor8;
pub type quadtree_node_t = crate::src::src::node::quadtree_node;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor9;
pub type quadtree_t = crate::src::src::quadtree::quadtree;
#[no_mangle]
pub unsafe extern "C" fn descent(mut node: *mut quadtree_node_t) {
    if !(*node).bounds.is_null() {
        printf(
            b"{ nw.x:%f, nw.y:%f, se.x:%f, se.y:%f }: \0" as *const u8
                as *const libc::c_char,
            (*(*(*node).bounds).nw).x,
            (*(*(*node).bounds).nw).y,
            (*(*(*node).bounds).se).x,
            (*(*(*node).bounds).se).y,
        );
    }else { (); }
}
#[no_mangle]
pub unsafe extern "C" fn ascent(mut node: *mut quadtree_node_t) {
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn test_node() {
    let mut node = crate::src::src::node::quadtree_node_new();
    if crate::src::src::node::quadtree_node_isleaf(node) == 0 {} else {
        __assert_fail(
            b"!quadtree_node_isleaf(node)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int as libc::c_uint,
            b"void test_node()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::node::quadtree_node_isempty(node) != 0 {} else {
        __assert_fail(
            b"quadtree_node_isempty(node)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            b"void test_node()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::node::quadtree_node_ispointer(node) == 0 {} else {
        __assert_fail(
            b"!quadtree_node_ispointer(node)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            b"void test_node()\0" as *const u8 as *const i8,
        );
    };
}
unsafe extern "C" fn test_bounds() {
    let mut bounds = crate::src::src::bounds::quadtree_bounds_new();
    if !bounds.is_null() {} else {();
        __assert_fail(
            b"bounds\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*(*bounds).nw).x == ::std::f32::INFINITY as libc::c_double {} else {
        __assert_fail(
            b"bounds->nw->x == INFINITY\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*(*bounds).se).x == -::std::f32::INFINITY as libc::c_double {} else {
        __assert_fail(
            b"bounds->se->x == -INFINITY\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    crate::src::src::bounds::quadtree_bounds_extend(bounds, 5.0f64, 5.0f64);
    if (*(*bounds).nw).x == 5.0f64 {} else {
        __assert_fail(
            b"bounds->nw->x == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*(*bounds).se).x == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->x == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    crate::src::src::bounds::quadtree_bounds_extend(bounds, 10.0f64, 10.0f64);
    if (*(*bounds).nw).y == 10.0f64 {} else {
        __assert_fail(
            b"bounds->nw->y == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*(*bounds).nw).y == 10.0f64 {} else {
        __assert_fail(
            b"bounds->nw->y == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*(*bounds).se).y == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->y == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*(*bounds).se).y == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->y == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*bounds).width == 5.0f64 {} else {
        __assert_fail(
            b"bounds->width == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    if (*bounds).height == 5.0f64 {} else {
        __assert_fail(
            b"bounds->height == 5.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            b"void test_bounds()\0" as *const u8 as *const i8,
        );
    };
    crate::src::src::bounds::quadtree_bounds_free(bounds);
}
unsafe extern "C" fn test_tree() {
    let mut val = 10 as libc::c_int;
    let mut tree = crate::src::src::quadtree::quadtree_new(
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
    );
    if (*(*(*(*tree).root).bounds).nw).x == 1 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"tree->root->bounds->nw->x == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*(*(*(*tree).root).bounds).nw).y == 10.0f64 {} else {
        __assert_fail(
            b"tree->root->bounds->nw->y == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*(*(*(*tree).root).bounds).se).x == 10.0f64 {} else {
        __assert_fail(
            b"tree->root->bounds->se->x == 10.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*(*(*(*tree).root).bounds).se).y == 1 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"tree->root->bounds->se->y == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 0, 0, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree,
        10 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 10, 10, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree,
        110.0f64,
        110.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 110.0, 110.0, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree,
        8.0f64,
        2.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 8.0, 2.0, &val) != 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*tree).length == 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tree->length == 1\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*(*(*tree).root).point).x == 8.0f64 {} else {
        __assert_fail(
            b"tree->root->point->x == 8.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*(*(*tree).root).point).y == 2.0f64 {} else {
        __assert_fail(
            b"tree->root->point->y == 2.0\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree,
        2.0f64,
        3.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 2.0, 3.0, &val) != 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree,
        2.0f64,
        3.0f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 2.0, 3.0, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*tree).length == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tree->length == 2\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*(*tree).root).point.is_null() {();} else {
        __assert_fail(
            b"tree->root->point == NULL\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if crate::src::src::quadtree::quadtree_insert(
        tree,
        3.0f64,
        1.1f64,
        core::ptr::addr_of_mut!(val) as *mut libc::c_int as *mut libc::c_void,
    ) == 1 as libc::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 3.0, 1.1, &val) == 1\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*tree).length == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"tree->length == 3\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    if (*crate::src::src::quadtree::quadtree_search(tree.as_mut(), 3.0f64, 1.1f64)).x == 3.0f64 {} else {
        __assert_fail(
            b"quadtree_search(tree, 3.0, 1.1)->x == 3.0\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            b"void test_tree()\0" as *const u8 as *const i8,
        );
    };
    crate::src::src::quadtree::quadtree_walk(
        (*tree).root,
        Some(ascent as unsafe extern "C" fn(*mut quadtree_node_t) -> ()),
        Some(descent as unsafe extern "C" fn(*mut quadtree_node_t) -> ()),
    );
    crate::src::src::quadtree::quadtree_free(tree);
}
unsafe extern "C" fn test_points() {
    let mut point = crate::src::src::point::quadtree_point_new(
        5 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
    );
    if (*point).x == 5 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"point->x == 5\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            b"void test_points()\0" as *const u8 as *const i8,
        );
    };
    if (*point).y == 6 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"point->y == 6\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            b"void test_points()\0" as *const u8 as *const i8,
        );
    };
    crate::src::src::point::quadtree_point_free(point);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(
        b"\nquadtree_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_t>() as libc::c_ulong,
    );
    printf(
        b"quadtree_node_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_node_t>() as libc::c_ulong,
    );
    printf(
        b"quadtree_bounds_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_bounds_t>() as libc::c_ulong,
    );
    printf(
        b"quadtree_point_t: %ld\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<quadtree_point_t>() as libc::c_ulong,
    );
    printf(b"\x1B[33mtree\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_tree();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[33mnode\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_node();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[33mbounds\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_bounds();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[33mpoints\x1B[0m \0" as *const u8 as *const libc::c_char);
    test_points();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const libc::c_char);
    return 0;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *const libc::c_char,
//             ) as i32,
//         )
//     }
// }
