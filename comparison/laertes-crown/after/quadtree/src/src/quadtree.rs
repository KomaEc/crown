
extern "C" {
    
    
    
    
    
    
    
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
}
pub use crate::src::src::node::quadtree_node_free;
pub use crate::src::src::node::quadtree_node_isempty;
pub use crate::src::src::node::quadtree_node_isleaf;
pub use crate::src::src::node::quadtree_node_ispointer;
pub use crate::src::src::node::quadtree_node_reset;
pub use crate::src::src::node::quadtree_node_with_bounds;
pub use crate::src::src::point::quadtree_point_new;
// #[derive(Copy, Clone)]

pub type quadtree_point = crate::src::src::bounds::quadtree_point;
pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
// #[derive(Copy, Clone)]

pub type quadtree_bounds = crate::src::src::bounds::quadtree_bounds;
pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
// #[derive(Copy, Clone)]

pub type quadtree_node = crate::src::src::node::quadtree_node;
pub type quadtree_node_t = crate::src::src::node::quadtree_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree {
    pub root: * mut crate::src::src::node::quadtree_node,
    pub key_free: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
    pub length: std::os::raw::c_uint,
}
impl quadtree {
    pub const fn new() -> Self {
        quadtree {
        root: (0 as * mut crate::src::src::node::quadtree_node),
        key_free: None,
        length: 0
        }
    }
}

impl std::default::Default for quadtree {
    fn default() -> Self { quadtree::new() }
}

pub type quadtree_t = crate::src::src::quadtree::quadtree;
unsafe extern "C" fn node_contains_(
    mut outer: * mut crate::src::src::node::quadtree_node,
    mut it: * mut crate::src::src::bounds::quadtree_point,
) -> std::os::raw::c_int {
    return (!((*outer).bounds).is_null() && (*(*(*outer).bounds).nw).x < (*it).x
        && (*(*(*outer).bounds).nw).y > (*it).y && (*(*(*outer).bounds).se).x > (*it).x
        && (*(*(*outer).bounds).se).y < (*it).y) as std::os::raw::c_int;
}
 extern "C" fn elision_(mut key: * mut core::ffi::c_void) {}
unsafe extern "C" fn reset_node_(
    mut tree: * mut crate::src::src::quadtree::quadtree,
    mut node: * mut crate::src::src::node::quadtree_node,
) {
    if ((*tree).key_free).is_some() {
        quadtree_node_reset(node, (*tree).key_free);
    } else {
        quadtree_node_reset(
            node,
            Some(elision_),
        );
    };
}
unsafe extern "C" fn get_quadrant_(
    mut root: * mut crate::src::src::node::quadtree_node,
    mut point: * mut crate::src::src::bounds::quadtree_point,
) -> * mut crate::src::src::node::quadtree_node {
    if node_contains_((*root).nw, point) != 0 {
        return (*root).nw;
    }
    if node_contains_((*root).ne, point) != 0 {
        return (*root).ne;
    }
    if node_contains_((*root).sw, point) != 0 {
        return (*root).sw;
    }
    if node_contains_((*root).se, point) != 0 {
        return (*root).se;
    }
    return 0 as *mut quadtree_node_t;
}
unsafe extern "C" fn split_node_(
    mut tree: * mut crate::src::src::quadtree::quadtree,
    mut node: * mut crate::src::src::node::quadtree_node,
) -> std::os::raw::c_int {
    let mut nw = 0 as *mut quadtree_node_t;
    let mut ne = 0 as *mut quadtree_node_t;
    let mut sw = 0 as *mut quadtree_node_t;
    let mut se = 0 as *mut quadtree_node_t;
    let mut x = (*(*(*node).bounds).nw).x;
    let mut y = (*(*(*node).bounds).nw).y;
    let mut hw = (*(*node).bounds).width / 2 as std::os::raw::c_int as std::os::raw::c_double;
    let mut hh = (*(*node).bounds).height / 2 as std::os::raw::c_int as std::os::raw::c_double;
    nw = quadtree_node_with_bounds(x, y - hh, x + hw, y);
    if nw.is_null() {
        return 0 as std::os::raw::c_int;
    }
    ne = quadtree_node_with_bounds(
        x + hw,
        y - hh,
        x + hw * 2 as std::os::raw::c_int as std::os::raw::c_double,
        y,
    );
    if ne.is_null() {
        return 0 as std::os::raw::c_int;
    }
    sw = quadtree_node_with_bounds(
        x,
        y - hh * 2 as std::os::raw::c_int as std::os::raw::c_double,
        x + hw,
        y - hh,
    );
    if sw.is_null() {
        return 0 as std::os::raw::c_int;
    }
    se = quadtree_node_with_bounds(
        x + hw,
        y - hh * 2 as std::os::raw::c_int as std::os::raw::c_double,
        x + hw * 2 as std::os::raw::c_int as std::os::raw::c_double,
        y - hh,
    );
    if se.is_null() {
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh0 = (*node).nw;
    *fresh0 = nw;
    let ref mut fresh1 = (*node).ne;
    *fresh1 = ne;
    let ref mut fresh2 = (*node).sw;
    *fresh2 = sw;
    let ref mut fresh3 = (*node).se;
    *fresh3 = se;
    let mut old = (*node).point;
    let mut key = (*node).key;
    let ref mut fresh4 = (*node).point;
    *fresh4 = 0 as *mut quadtree_point_t;
    let ref mut fresh5 = (*node).key;
    *fresh5 = 0 as *mut std::os::raw::c_void;
    return insert_(tree, node, old, key);
}
unsafe extern "C" fn find_(
    mut node: * mut crate::src::src::node::quadtree_node,
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
) -> * mut crate::src::src::bounds::quadtree_point {
    if quadtree_node_isleaf(node) != 0 {
        if (*(*node).point).x == x && (*(*node).point).y == y {
            return (*node).point;
        }
    } else {
        let mut test = quadtree_point_t { x: 0., y: 0. };
        test.x = x;
        test.y = y;
        return find_(get_quadrant_(node, &mut test), x, y);
    }
    return 0 as *mut quadtree_point_t;
}
unsafe extern "C" fn insert_(
    mut tree: * mut crate::src::src::quadtree::quadtree,
    mut root: * mut crate::src::src::node::quadtree_node,
    mut point: * mut crate::src::src::bounds::quadtree_point,
    mut key: * mut core::ffi::c_void,
) -> std::os::raw::c_int {
    if quadtree_node_isempty(root) != 0 {
        let ref mut fresh6 = (*root).point;
        *fresh6 = point;
        let ref mut fresh7 = (*root).key;
        *fresh7 = key;
        return 1 as std::os::raw::c_int;
    } else {
        if quadtree_node_isleaf(root) != 0 {
            if (*(*root).point).x == (*point).x && (*(*root).point).y == (*point).y {
                reset_node_(tree, root);
                let ref mut fresh8 = (*root).point;
                *fresh8 = point;
                let ref mut fresh9 = (*root).key;
                *fresh9 = key;
                return 0 as std::os::raw::c_int;
            } else {
                if split_node_(tree, root) == 0 {
                    return 0 as std::os::raw::c_int;
                }
                return insert_(tree, root, point, key);
            }
        } else {
            if quadtree_node_ispointer(root) != 0 {
                let mut quadrant = get_quadrant_(root, point);
                return if quadrant.is_null() {
                    0 as std::os::raw::c_int
                } else {
                    insert_(tree, quadrant, point, key)
                };
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_new(
    mut minx: std::os::raw::c_double,
    mut miny: std::os::raw::c_double,
    mut maxx: std::os::raw::c_double,
    mut maxy: std::os::raw::c_double,
) -> * mut crate::src::src::quadtree::quadtree {
    let mut tree = 0 as *mut quadtree_t;
    tree = malloc(::std::mem::size_of::<quadtree_t>() as std::os::raw::c_ulong)
        as *mut quadtree_t;
    if tree.is_null() {
        return 0 as *mut quadtree_t;
    }
    let ref mut fresh10 = (*tree).root;
    *fresh10 = quadtree_node_with_bounds(minx, miny, maxx, maxy);
    if ((*tree).root).is_null() {
        free(tree as *mut std::os::raw::c_void);
        return 0 as *mut quadtree_t;
    }
    let ref mut fresh11 = (*tree).key_free;
    *fresh11 = None;
    (*tree).length = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_insert(
    mut tree: * mut crate::src::src::quadtree::quadtree,
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
    mut key: * mut core::ffi::c_void,
) -> std::os::raw::c_int {
    let mut point = 0 as *mut quadtree_point_t;
    point = quadtree_point_new(x, y);
    if point.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if node_contains_((*tree).root, point) == 0 {
        return 0 as std::os::raw::c_int;
    }
    if insert_(tree, (*tree).root, point, key) == 0 {
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh12 = (*tree).length;
    *fresh12 = (*fresh12).wrapping_add(1);
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_search(
    mut tree: * mut crate::src::src::quadtree::quadtree,
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
) -> * mut crate::src::src::bounds::quadtree_point {
    return find_((*tree).root, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_free(mut tree: * mut crate::src::src::quadtree::quadtree) {
    if ((*tree).key_free).is_some() {
        quadtree_node_free((*tree).root, (*tree).key_free);
    } else {
        quadtree_node_free(
            (*tree).root,
            Some(elision_),
        );
    }
    free(tree as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_walk(
    mut root: * mut crate::src::src::node::quadtree_node,
    mut descent: Option<unsafe extern "C"  fn(_: * mut crate::src::src::node::quadtree_node,) -> ()>,
    mut ascent: Option<unsafe extern "C"  fn(_: * mut crate::src::src::node::quadtree_node,) -> ()>,
) {
    (Some(descent.expect("non-null function pointer")))
        .expect("non-null function pointer")(root);
    if !((*root).nw).is_null() {
        quadtree_walk((*root).nw, descent, ascent);
    }
    if !((*root).ne).is_null() {
        quadtree_walk((*root).ne, descent, ascent);
    }
    if !((*root).sw).is_null() {
        quadtree_walk((*root).sw, descent, ascent);
    }
    if !((*root).se).is_null() {
        quadtree_walk((*root).se, descent, ascent);
    }
    (Some(ascent.expect("non-null function pointer")))
        .expect("non-null function pointer")(root);
}
use crate::laertes_rt::*;
