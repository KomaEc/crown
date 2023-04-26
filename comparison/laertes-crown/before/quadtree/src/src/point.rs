
extern "C" {
    fn free(_: *mut std::os::raw::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: std::os::raw::c_double,
    pub y: std::os::raw::c_double,
}
pub type quadtree_point_t = quadtree_point;
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_new(
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
) -> *mut quadtree_point_t {
    let mut point = 0 as *mut quadtree_point_t;
    point = malloc(::std::mem::size_of::<quadtree_point_t>() as std::os::raw::c_ulong)
        as *mut quadtree_point_t;
    if point.is_null() {
        return 0 as *mut quadtree_point_t;
    }
    (*point).x = x;
    (*point).y = y;
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_free(mut point: *mut quadtree_point_t) {
    free(point as *mut std::os::raw::c_void);
}
