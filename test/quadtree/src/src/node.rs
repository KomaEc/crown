use ::libc;
extern "C" {
    
    
    
    
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
impl Default for ErasedByPreprocessor0 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1 { dummy: () }
impl Default for ErasedByPreprocessor1 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer1;
#[repr(C)]
pub struct quadtree_node {
    pub ne: *mut /* owning */ quadtree_node,
    pub nw: *mut /* owning */ quadtree_node,
    pub se: *mut /* owning */ quadtree_node,
    pub sw: *mut /* owning */ quadtree_node,
    pub bounds: *mut /* owning */ quadtree_bounds_t,
    pub point: *mut quadtree_point_t,
    pub key: *mut libc::c_void,
}
impl Default for quadtree_node {fn default() -> Self {Self {
ne: std::ptr::null_mut(),
nw: std::ptr::null_mut(),
se: std::ptr::null_mut(),
sw: std::ptr::null_mut(),
bounds: std::ptr::null_mut(),
point: std::ptr::null_mut(),
key: std::ptr::null_mut(),
}}}
impl quadtree_node {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type quadtree_node_t = quadtree_node;
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_ispointer(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return (!(*node).nw.is_null() && !(*node).ne.is_null() && !(*node).sw.is_null()
        && !(*node).se.is_null() && quadtree_node_isleaf(node) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_isempty(
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    return ((*node).nw.is_null() && (*node).ne.is_null() && (*node).sw.is_null()
        && (*node).se.is_null() && quadtree_node_isleaf(node) == 0) as libc::c_int;
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
    crate::src::src::point::quadtree_point_free(Some(Box::from_raw((*node).point)));
    (Some(key_free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*node).key);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_new() -> Option<Box<quadtree_node_t>> {
    let mut node = None;
    node= Some(Box::new(<crate::src::src::node::quadtree_node as Default>::default()));
    if node.as_deref().is_none() {();
        return None;
    }
    (*node.as_deref_mut().unwrap()).ne= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).nw= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).se= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).sw= 0 as *mut quadtree_node;
    (*node.as_deref_mut().unwrap()).point= 0 as *mut quadtree_point_t;
    (*node.as_deref_mut().unwrap()).bounds= 0 as *mut quadtree_bounds_t;
    (*node.as_deref_mut().unwrap()).key= 0 as *mut libc::c_void;
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
    node= quadtree_node_new().map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut());
    if node.is_null() {();
        return 0 as *mut quadtree_node_t;
    }
    (*node).bounds= crate::src::src::bounds::quadtree_bounds_new().map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut());
    if (*node).bounds.is_null() {();
        return 0 as *mut quadtree_node_t;
    }
    crate::src::src::bounds::quadtree_bounds_extend((*node).bounds, maxx, maxy);
    crate::src::src::bounds::quadtree_bounds_extend((*node).bounds, minx, miny);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_node_free(
    mut node: Option<Box<quadtree_node_t>>,
    mut key_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if !(*node.as_deref().unwrap()).nw.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).nw)), key_free);
    }else { (); }
    if !(*node.as_deref().unwrap()).ne.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).ne)), key_free);
    }else { (); }
    if !(*node.as_deref().unwrap()).sw.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).sw)), key_free);
    }else { (); }
    if !(*node.as_deref().unwrap()).se.is_null() {
        quadtree_node_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).se)), key_free);
    }else { (); }
    crate::src::src::bounds::quadtree_bounds_free(Some(Box::from_raw((*node.as_deref_mut().unwrap()).bounds)));
    quadtree_node_reset(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), key_free);
    ();
}
