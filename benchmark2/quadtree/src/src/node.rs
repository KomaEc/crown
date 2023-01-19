use ::libc;
extern "C" {
    fn quadtree_bounds_free(bounds: *mut quadtree_bounds_t);
    fn quadtree_bounds_extend(
        bounds: *mut quadtree_bounds_t,
        x: libc::c_double,
        y: libc::c_double,
    );
    fn quadtree_bounds_new() -> *mut quadtree_bounds_t;
    fn quadtree_point_free(point: *mut quadtree_point_t);
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point_t,
    pub se: *mut quadtree_point_t,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
pub type quadtree_bounds_t = quadtree_bounds;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_node {
    pub ne: *mut quadtree_node,
    pub nw: *mut quadtree_node,
    pub se: *mut quadtree_node,
    pub sw: *mut quadtree_node,
    pub bounds: *mut quadtree_bounds_t,
    pub point: *mut quadtree_point_t,
    pub key: *mut libc::c_void,
}
pub type quadtree_node_t = quadtree_node;
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_ispointer(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return (!((*node).nw).is_null() && !((*node).ne).is_null() && !((*node).sw).is_null()
        && !((*node).se).is_null() && quadtree_node_isleaf(node) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isempty(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return (((*node).nw).is_null() && ((*node).ne).is_null() && ((*node).sw).is_null()
        && ((*node).se).is_null() && quadtree_node_isleaf(node) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isleaf(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return ((*node).point != 0 as *mut libc::c_void as *mut quadtree_point_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_reset(
    mut node: *mut quadtree_node_t,
    mut key_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    quadtree_point_free((*node).point);
    (Some(key_free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*node).key);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_new() -> *mut quadtree_node_t {
    let mut node = 0 as *mut quadtree_node_t;
    node = malloc(::std::mem::size_of::<quadtree_node_t>() as libc::c_ulong)
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
    *fresh6 = 0 as *mut libc::c_void;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_with_bounds(
    mut minx: libc::c_double,
    mut miny: libc::c_double,
    mut maxx: libc::c_double,
    mut maxy: libc::c_double,
) -> *mut quadtree_node_t {
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
    mut node: *mut quadtree_node_t,
    mut key_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
    free(node as *mut libc::c_void);
}
