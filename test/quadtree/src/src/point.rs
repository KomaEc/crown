use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor2 { dummy: () }
impl Default for ErasedByPreprocessor2 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_new(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> Option<Box<quadtree_point_t>> {
    let mut point = None;
    point= Some(Box::new(<crate::src::src::bounds::quadtree_point as Default>::default()));
    if point.as_deref().is_none() {();
        return None;
    }
    (*point.as_deref_mut().unwrap()).x= x;
    (*point.as_deref_mut().unwrap()).y= y;
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_free(mut point: Option<Box<quadtree_point_t>>) {
    ();
}
