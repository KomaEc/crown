
extern "C" {
    
    
    
    
    fn free(_: * mut core::ffi::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
}
pub use crate::src::src::bounds::quadtree_bounds_extend;
pub use crate::src::src::bounds::quadtree_bounds_free;
pub use crate::src::src::bounds::quadtree_bounds_new;
pub use crate::src::src::point::quadtree_point_free;
// #[derive(Copy, Clone)]

pub type quadtree_point = crate::src::src::bounds::quadtree_point;
pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
// #[derive(Copy, Clone)]

pub type quadtree_bounds = crate::src::src::bounds::quadtree_bounds;
pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_node {
    pub ne: * mut crate::src::src::node::quadtree_node,
    pub nw: * mut crate::src::src::node::quadtree_node,
    pub se: * mut crate::src::src::node::quadtree_node,
    pub sw: * mut crate::src::src::node::quadtree_node,
    pub bounds: * mut crate::src::src::bounds::quadtree_bounds,
    pub point: * mut crate::src::src::bounds::quadtree_point,
    pub key: * mut core::ffi::c_void,
}
impl quadtree_node {
    pub const fn new() -> Self {
        quadtree_node {
        ne: (0 as * mut crate::src::src::node::quadtree_node),
        nw: (0 as * mut crate::src::src::node::quadtree_node),
        se: (0 as * mut crate::src::src::node::quadtree_node),
        sw: (0 as * mut crate::src::src::node::quadtree_node),
        bounds: (0 as * mut crate::src::src::bounds::quadtree_bounds),
        point: (0 as * mut crate::src::src::bounds::quadtree_point),
        key: (0 as * mut core::ffi::c_void)
        }
    }
}

impl std::default::Default for quadtree_node {
    fn default() -> Self { quadtree_node::new() }
}

pub type quadtree_node_t = crate::src::src::node::quadtree_node;
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_ispointer(
    mut node: * mut crate::src::src::node::quadtree_node,
) -> std::os::raw::c_int {
    return (!((*node).nw).is_null() && !((*node).ne).is_null() && !((*node).sw).is_null()
        && !((*node).se).is_null() && quadtree_node_isleaf(node) == 0) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isempty(
    mut node: * mut crate::src::src::node::quadtree_node,
) -> std::os::raw::c_int {
    return (((*node).nw).is_null() && ((*node).ne).is_null() && ((*node).sw).is_null()
        && ((*node).se).is_null() && quadtree_node_isleaf(node) == 0) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isleaf(
    mut node: * mut crate::src::src::node::quadtree_node,
) -> std::os::raw::c_int {
    return ((*node).point != 0 as *mut std::os::raw::c_void as *mut quadtree_point_t)
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_reset(
    mut node: * mut crate::src::src::node::quadtree_node,
    mut key_free: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
) {
    quadtree_point_free((*node).point);
    (Some(key_free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*node).key);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_new() -> * mut crate::src::src::node::quadtree_node {
    let mut node = 0 as *mut quadtree_node_t;
    node = malloc(::std::mem::size_of::<quadtree_node_t>() as std::os::raw::c_ulong)
        as *mut quadtree_node_t;
    if node.is_null() {
        return 0 as *mut quadtree_node_t;
    }
    let ref mut fresh0 = (*node).ne;
    *fresh0 = 0 as *mut quadtree_node;
    let ref mut fresh1 = (*node).nw;
    *fresh1 = 0 as *mut quadtree_node;
    let ref mut fresh2 = (*node).se;
    *fresh2 = 0 as *mut quadtree_node;
    let ref mut fresh3 = (*node).sw;
    *fresh3 = 0 as *mut quadtree_node;
    let ref mut fresh4 = (*node).point;
    *fresh4 = 0 as *mut quadtree_point_t;
    let ref mut fresh5 = (*node).bounds;
    *fresh5 = 0 as *mut quadtree_bounds_t;
    let ref mut fresh6 = (*node).key;
    *fresh6 = 0 as *mut std::os::raw::c_void;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_with_bounds(
    mut minx: std::os::raw::c_double,
    mut miny: std::os::raw::c_double,
    mut maxx: std::os::raw::c_double,
    mut maxy: std::os::raw::c_double,
) -> * mut crate::src::src::node::quadtree_node {
    let mut node = 0 as *mut quadtree_node_t;
    node = quadtree_node_new();
    if node.is_null() {
        return 0 as *mut quadtree_node_t;
    }
    let ref mut fresh7 = (*node).bounds;
    *fresh7 = quadtree_bounds_new();
    if (*fresh7).is_null() {
        return 0 as *mut quadtree_node_t;
    }
    quadtree_bounds_extend((*node).bounds, maxx, maxy);
    quadtree_bounds_extend((*node).bounds, minx, miny);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_free(
    mut node: * mut crate::src::src::node::quadtree_node,
    mut key_free: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
) {
    if !((*node).nw).is_null() {
        quadtree_node_free((*node).nw, key_free);
    }
    if !((*node).ne).is_null() {
        quadtree_node_free((*node).ne, key_free);
    }
    if !((*node).sw).is_null() {
        quadtree_node_free((*node).sw, key_free);
    }
    if !((*node).se).is_null() {
        quadtree_node_free((*node).se, key_free);
    }
    quadtree_bounds_free((*node).bounds);
    quadtree_node_reset(node, key_free);
    free(node as *mut std::os::raw::c_void);
}
use crate::laertes_rt::*;
