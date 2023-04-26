use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    
    
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
impl Default for quadtree_point {fn default() -> Self {Self {
x: Default::default(),
y: Default::default(),
}}}

pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: Option<Box<quadtree_point_t>>,
    pub se: Option<Box<quadtree_point_t>>,
    pub width: libc::c_double,
    pub height: libc::c_double,
}
impl Default for quadtree_bounds {fn default() -> Self {Self {
nw: None,
se: None,
width: Default::default(),
height: Default::default(),
}}}
impl quadtree_bounds {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type quadtree_bounds_t = quadtree_bounds;
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_extend(
    mut bounds: *mut quadtree_bounds_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
) {
    (*(*bounds).nw.as_deref_mut().unwrap()).x= fmin(x, (*(*bounds).nw.as_deref().unwrap()).x);
    (*(*bounds).nw.as_deref_mut().unwrap()).y= fmax(y, (*(*bounds).nw.as_deref().unwrap()).y);
    (*(*bounds).se.as_deref_mut().unwrap()).x= fmax(x, (*(*bounds).se.as_deref().unwrap()).x);
    (*(*bounds).se.as_deref_mut().unwrap()).y= fmin(y, (*(*bounds).se.as_deref().unwrap()).y);
    (*bounds).width= fabs((*(*bounds).nw.as_deref().unwrap()).x - (*(*bounds).se.as_deref().unwrap()).x);
    (*bounds).height= fabs((*(*bounds).nw.as_deref().unwrap()).y - (*(*bounds).se.as_deref().unwrap()).y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_free(mut bounds: Option<Box<quadtree_bounds_t>>) {
    crate::src::src::point::quadtree_point_free((*bounds.as_deref_mut().unwrap()).nw.take());
    crate::src::src::point::quadtree_point_free((*bounds.as_deref_mut().unwrap()).se.take());
    ();
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_bounds_new() -> Option<Box<quadtree_bounds_t>> {
    let mut bounds = None;
    bounds= Some(Box::new(<crate::src::src::bounds::quadtree_bounds as Default>::default()));
    if bounds.as_deref().is_none() {();
        return None;
    }
    (*bounds.as_deref_mut().unwrap()).nw= crate::src::src::point::quadtree_point_new(
        ::std::f32::INFINITY as libc::c_double,
        -::std::f32::INFINITY as libc::c_double,
    );
    (*bounds.as_deref_mut().unwrap()).se= crate::src::src::point::quadtree_point_new(
        -::std::f32::INFINITY as libc::c_double,
        ::std::f32::INFINITY as libc::c_double,
    );
    (*bounds.as_deref_mut().unwrap()).width= 0 as libc::c_int as libc::c_double;
    (*bounds.as_deref_mut().unwrap()).height= 0 as libc::c_int as libc::c_double;
    return bounds;
}
