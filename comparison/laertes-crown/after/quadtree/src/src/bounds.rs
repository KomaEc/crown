
extern "C" {
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    fn free(_: * mut core::ffi::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn fmax(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fmin(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
}
pub use crate::src::src::point::quadtree_point_free;
pub use crate::src::src::point::quadtree_point_new;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: std::os::raw::c_double,
    pub y: std::os::raw::c_double,
}
impl quadtree_point {
    pub const fn new() -> Self {
        quadtree_point {
        x: 0.0,
        y: 0.0
        }
    }
}

impl std::default::Default for quadtree_point {
    fn default() -> Self { quadtree_point::new() }
}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: * mut crate::src::src::bounds::quadtree_point,
    pub se: * mut crate::src::src::bounds::quadtree_point,
    pub width: std::os::raw::c_double,
    pub height: std::os::raw::c_double,
}
impl quadtree_bounds {
    pub const fn new() -> Self {
        quadtree_bounds {
        nw: (0 as * mut crate::src::src::bounds::quadtree_point),
        se: (0 as * mut crate::src::src::bounds::quadtree_point),
        width: 0.0,
        height: 0.0
        }
    }
}

impl std::default::Default for quadtree_bounds {
    fn default() -> Self { quadtree_bounds::new() }
}

pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_extend(
    mut bounds: * mut crate::src::src::bounds::quadtree_bounds,
    mut x: std::os::raw::c_double,
    mut y: std::os::raw::c_double,
) {
    (*(*bounds).nw).x = fmin(x, (*(*bounds).nw).x);
    (*(*bounds).nw).y = fmax(y, (*(*bounds).nw).y);
    (*(*bounds).se).x = fmax(x, (*(*bounds).se).x);
    (*(*bounds).se).y = fmin(y, (*(*bounds).se).y);
    (*bounds).width = fabs((*(*bounds).nw).x - (*(*bounds).se).x);
    (*bounds).height = fabs((*(*bounds).nw).y - (*(*bounds).se).y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_free(mut bounds: * mut crate::src::src::bounds::quadtree_bounds) {
    quadtree_point_free((*bounds).nw);
    quadtree_point_free((*bounds).se);
    free(bounds as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_new() -> * mut crate::src::src::bounds::quadtree_bounds {
    let mut bounds = 0 as *mut quadtree_bounds_t;
    bounds = malloc(::std::mem::size_of::<quadtree_bounds_t>() as std::os::raw::c_ulong)
        as *mut quadtree_bounds_t;
    if bounds.is_null() {
        return 0 as *mut quadtree_bounds_t;
    }
    let ref mut fresh0 = (*bounds).nw;
    *fresh0 = quadtree_point_new(
        ::std::f32::INFINITY as std::os::raw::c_double,
        -::std::f32::INFINITY as std::os::raw::c_double,
    );
    let ref mut fresh1 = (*bounds).se;
    *fresh1 = quadtree_point_new(
        -::std::f32::INFINITY as std::os::raw::c_double,
        ::std::f32::INFINITY as std::os::raw::c_double,
    );
    (*bounds).width = 0 as std::os::raw::c_int as std::os::raw::c_double;
    (*bounds).height = 0 as std::os::raw::c_int as std::os::raw::c_double;
    return bounds;
}
use crate::laertes_rt::*;
