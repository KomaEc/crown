use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn quadtree_point_free(point: *mut quadtree_point_t);
    fn quadtree_point_new(x: libc::c_double, y: libc::c_double) -> *mut quadtree_point_t;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_extend(
    mut bounds: *mut quadtree_bounds_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    (*(*bounds).nw).x = fmin(x, (*(*bounds).nw).x);
    (*(*bounds).nw).y = fmax(y, (*(*bounds).nw).y);
    (*(*bounds).se).x = fmax(x, (*(*bounds).se).x);
    (*(*bounds).se).y = fmin(y, (*(*bounds).se).y);
    (*bounds).width = fabs((*(*bounds).nw).x - (*(*bounds).se).x);
    (*bounds).height = fabs((*(*bounds).nw).y - (*(*bounds).se).y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_free(mut bounds: *mut quadtree_bounds_t) {
    quadtree_point_free((*bounds).nw);
    quadtree_point_free((*bounds).se);
    free(bounds as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_new() -> *mut quadtree_bounds_t {
    let mut bounds = 0 as *mut quadtree_bounds_t;
    bounds = malloc(::std::mem::size_of::<quadtree_bounds_t>() as libc::c_ulong)
        as *mut quadtree_bounds_t;
    if bounds.is_null() {
        return 0 as *mut quadtree_bounds_t;
    }
    let ref mut fresh0 = (*bounds).nw;
    *fresh0 = quadtree_point_new(
        ::std::f32::INFINITY as libc::c_double,
        -::std::f32::INFINITY as libc::c_double,
    );
    let ref mut fresh1 = (*bounds).se;
    *fresh1 = quadtree_point_new(
        -::std::f32::INFINITY as libc::c_double,
        ::std::f32::INFINITY as libc::c_double,
    );
    (*bounds).width = 0 as libc::c_int as libc::c_double;
    (*bounds).height = 0 as libc::c_int as libc::c_double;
    return bounds;
}
