
extern "C" {
    fn free(_: * mut core::ffi::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
}
// #[derive(Copy, Clone)]

pub type quadtree_point = crate::src::src::bounds::quadtree_point;
pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_new(
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
) -> * mut crate::src::src::bounds::quadtree_point {
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
pub unsafe extern "C" fn quadtree_point_free(mut point: * mut crate::src::src::bounds::quadtree_point) {
    free(point as *mut std::os::raw::c_void);
}
use crate::laertes_rt::*;
