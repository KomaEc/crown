use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor2 { dummy: () }
pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_new(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> *mut /* owning */ quadtree_point_t {
    let mut point = 0 as *mut quadtree_point_t;
    point= malloc(::std::mem::size_of::<quadtree_point_t>() as libc::c_ulong)
        as *mut quadtree_point_t;
    if point.is_null() {();
        return 0 as *mut quadtree_point_t;
    }
    (*point).x= x;
    (*point).y= y;
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_free(mut point: *mut /* owning */ quadtree_point_t) {
    free(point as *mut libc::c_void);
}
