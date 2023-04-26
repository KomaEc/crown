
extern "C" {
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: * const std::os::raw::c_char) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::src::bounds::quadtree_bounds_extend;
pub use crate::src::src::bounds::quadtree_bounds_free;
pub use crate::src::src::bounds::quadtree_bounds_new;
pub use crate::src::src::node::quadtree_node_isempty;
pub use crate::src::src::node::quadtree_node_isleaf;
pub use crate::src::src::node::quadtree_node_ispointer;
pub use crate::src::src::node::quadtree_node_new;
pub use crate::src::src::point::quadtree_point_free;
pub use crate::src::src::point::quadtree_point_new;
pub use crate::src::src::quadtree::quadtree_free;
pub use crate::src::src::quadtree::quadtree_insert;
pub use crate::src::src::quadtree::quadtree_new;
pub use crate::src::src::quadtree::quadtree_search;
pub use crate::src::src::quadtree::quadtree_walk;
// #[derive(Copy, Clone)]

pub type quadtree_point = crate::src::src::bounds::quadtree_point;
pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
// #[derive(Copy, Clone)]

pub type quadtree_bounds = crate::src::src::bounds::quadtree_bounds;
pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
// #[derive(Copy, Clone)]

pub type quadtree_node = crate::src::src::node::quadtree_node;
pub type quadtree_node_t = crate::src::src::node::quadtree_node;
// #[derive(Copy, Clone)]

pub type quadtree = crate::src::src::quadtree::quadtree;
pub type quadtree_t = crate::src::src::quadtree::quadtree;
#[no_mangle]
pub unsafe extern "C" fn descent(mut node: * mut crate::src::src::node::quadtree_node) {
    if !((*node).bounds).is_null() {
        printf(
            b"{ nw.x:%f, nw.y:%f, se.x:%f, se.y:%f }: \0" as *const u8
                as *const std::os::raw::c_char,
            (*(*(*node).bounds).nw).x,
            (*(*(*node).bounds).nw).y,
            (*(*(*node).bounds).se).x,
            (*(*(*node).bounds).se).y,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ascent(mut node: * mut crate::src::src::node::quadtree_node) {
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
}
unsafe extern "C" fn test_node() {
    let mut node = quadtree_node_new();
    if quadtree_node_isleaf(node) == 0 {} else {
        __assert_fail(
            b"!quadtree_node_isleaf(node)\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            26 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_node()\0"))
                .as_ptr(),
        );
    };
    if quadtree_node_isempty(node) != 0 {} else {
        __assert_fail(
            b"quadtree_node_isempty(node)\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            27 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_node()\0"))
                .as_ptr(),
        );
    };
    if quadtree_node_ispointer(node) == 0 {} else {
        __assert_fail(
            b"!quadtree_node_ispointer(node)\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            28 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_node()\0"))
                .as_ptr(),
        );
    };
}
unsafe extern "C" fn test_bounds() {
    let mut bounds = quadtree_bounds_new();
    if !bounds.is_null() {} else {
        __assert_fail(
            b"bounds\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            35 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*(*bounds).nw).x == ::std::f32::INFINITY as std::os::raw::c_double {} else {
        __assert_fail(
            b"bounds->nw->x == INFINITY\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            36 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*(*bounds).se).x == -::std::f32::INFINITY as std::os::raw::c_double {} else {
        __assert_fail(
            b"bounds->se->x == -INFINITY\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            37 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    quadtree_bounds_extend(bounds, 5.0f64, 5.0f64);
    if (*(*bounds).nw).x == 5.0f64 {} else {
        __assert_fail(
            b"bounds->nw->x == 5.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            40 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*(*bounds).se).x == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->x == 5.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            41 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    quadtree_bounds_extend(bounds, 10.0f64, 10.0f64);
    if (*(*bounds).nw).y == 10.0f64 {} else {
        __assert_fail(
            b"bounds->nw->y == 10.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            44 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*(*bounds).nw).y == 10.0f64 {} else {
        __assert_fail(
            b"bounds->nw->y == 10.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            45 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*(*bounds).se).y == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->y == 5.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            46 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*(*bounds).se).y == 5.0f64 {} else {
        __assert_fail(
            b"bounds->se->y == 5.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            47 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*bounds).width == 5.0f64 {} else {
        __assert_fail(
            b"bounds->width == 5.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            49 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    if (*bounds).height == 5.0f64 {} else {
        __assert_fail(
            b"bounds->height == 5.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            50 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_bounds()\0"))
                .as_ptr(),
        );
    };
    quadtree_bounds_free(bounds);
}
unsafe extern "C" fn test_tree() {
    let mut val = 10 as std::os::raw::c_int;
    let mut tree = quadtree_new(
        1 as std::os::raw::c_int as std::os::raw::c_double,
        1 as std::os::raw::c_int as std::os::raw::c_double,
        10 as std::os::raw::c_int as std::os::raw::c_double,
        10 as std::os::raw::c_int as std::os::raw::c_double,
    );
    if (*(*(*(*tree).root).bounds).nw).x == 1 as std::os::raw::c_int as std::os::raw::c_double {} else {
        __assert_fail(
            b"tree->root->bounds->nw->x == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            61 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*(*(*(*tree).root).bounds).nw).y == 10.0f64 {} else {
        __assert_fail(
            b"tree->root->bounds->nw->y == 10.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            62 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*(*(*(*tree).root).bounds).se).x == 10.0f64 {} else {
        __assert_fail(
            b"tree->root->bounds->se->x == 10.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            63 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*(*(*(*tree).root).bounds).se).y == 1 as std::os::raw::c_int as std::os::raw::c_double {} else {
        __assert_fail(
            b"tree->root->bounds->se->y == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            64 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if quadtree_insert(
        tree,
        0 as std::os::raw::c_int as std::os::raw::c_double,
        0 as std::os::raw::c_int as std::os::raw::c_double,
        &mut val as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
    ) == 0 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 0, 0, &val) == 0\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            67 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if quadtree_insert(
        tree,
        10 as std::os::raw::c_int as std::os::raw::c_double,
        10 as std::os::raw::c_int as std::os::raw::c_double,
        &mut val as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
    ) == 0 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 10, 10, &val) == 0\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            68 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if quadtree_insert(
        tree,
        110.0f64,
        110.0f64,
        &mut val as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
    ) == 0 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 110.0, 110.0, &val) == 0\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            69 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if quadtree_insert(
        tree,
        8.0f64,
        2.0f64,
        &mut val as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
    ) != 0 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 8.0, 2.0, &val) != 0\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            71 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*tree).length == 1 as std::os::raw::c_int as std::os::raw::c_uint {} else {
        __assert_fail(
            b"tree->length == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            72 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*(*(*tree).root).point).x == 8.0f64 {} else {
        __assert_fail(
            b"tree->root->point->x == 8.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            73 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*(*(*tree).root).point).y == 2.0f64 {} else {
        __assert_fail(
            b"tree->root->point->y == 2.0\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            74 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if quadtree_insert(
        tree,
        2.0f64,
        3.0f64,
        &mut val as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
    ) != 0 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 2.0, 3.0, &val) != 0\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            76 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if quadtree_insert(
        tree,
        2.0f64,
        3.0f64,
        &mut val as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
    ) == 0 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 2.0, 3.0, &val) == 0\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            77 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*tree).length == 2 as std::os::raw::c_int as std::os::raw::c_uint {} else {
        __assert_fail(
            b"tree->length == 2\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            78 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if ((*(*tree).root).point).is_null() {} else {
        __assert_fail(
            b"tree->root->point == NULL\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            79 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if quadtree_insert(
        tree,
        3.0f64,
        1.1f64,
        &mut val as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
    ) == 1 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"quadtree_insert(tree, 3.0, 1.1, &val) == 1\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            81 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*tree).length == 3 as std::os::raw::c_int as std::os::raw::c_uint {} else {
        __assert_fail(
            b"tree->length == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            82 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    if (*quadtree_search(tree, 3.0f64, 1.1f64)).x == 3.0f64 {} else {
        __assert_fail(
            b"quadtree_search(tree, 3.0, 1.1)->x == 3.0\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            83 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_tree()\0"))
                .as_ptr(),
        );
    };
    quadtree_walk(
        (*tree).root,
        Some(ascent),
        Some(descent),
    );
    quadtree_free(tree);
}
unsafe extern "C" fn test_points() {
    let mut point = quadtree_point_new(
        5 as std::os::raw::c_int as std::os::raw::c_double,
        6 as std::os::raw::c_int as std::os::raw::c_double,
    );
    if (*point).x == 5 as std::os::raw::c_int as std::os::raw::c_double {} else {
        __assert_fail(
            b"point->x == 5\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            91 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_points()\0"))
                .as_ptr(),
        );
    };
    if (*point).y == 6 as std::os::raw::c_int as std::os::raw::c_double {} else {
        __assert_fail(
            b"point->y == 6\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            92 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 19], &'_ [i8; 19]>(b"void test_points()\0"))
                .as_ptr(),
        );
    };
    quadtree_point_free(point);
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: * mut * const std::os::raw::c_char,
) -> std::os::raw::c_int {
    printf(
        b"\nquadtree_t: %ld\n\0" as *const u8 as *const std::os::raw::c_char,
        ::std::mem::size_of::<quadtree_t>() as std::os::raw::c_ulong,
    );
    printf(
        b"quadtree_node_t: %ld\n\0" as *const u8 as *const std::os::raw::c_char,
        ::std::mem::size_of::<quadtree_node_t>() as std::os::raw::c_ulong,
    );
    printf(
        b"quadtree_bounds_t: %ld\n\0" as *const u8 as *const std::os::raw::c_char,
        ::std::mem::size_of::<quadtree_bounds_t>() as std::os::raw::c_ulong,
    );
    printf(
        b"quadtree_point_t: %ld\n\0" as *const u8 as *const std::os::raw::c_char,
        ::std::mem::size_of::<quadtree_point_t>() as std::os::raw::c_ulong,
    );
    printf(b"\x1B[33mtree\x1B[0m \0" as *const u8 as *const std::os::raw::c_char);
    test_tree();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const std::os::raw::c_char);
    printf(b"\x1B[33mnode\x1B[0m \0" as *const u8 as *const std::os::raw::c_char);
    test_node();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const std::os::raw::c_char);
    printf(b"\x1B[33mbounds\x1B[0m \0" as *const u8 as *const std::os::raw::c_char);
    test_bounds();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const std::os::raw::c_char);
    printf(b"\x1B[33mpoints\x1B[0m \0" as *const u8 as *const std::os::raw::c_char);
    test_points();
    puts(b"\x1B[1;32m \xE2\x9C\x93 \x1B[0m\0" as *const u8 as *const std::os::raw::c_char);
    return 0;
}
use crate::laertes_rt::*;
// pub fn main() {
//     let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
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
//                 (args.len() - 1) as std::os::raw::c_int,
//                 args.as_mut_ptr() as *mut *const std::os::raw::c_char,
//             ) as i32,
//         )
//     }
// }
