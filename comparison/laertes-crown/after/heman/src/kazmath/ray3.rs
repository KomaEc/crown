
extern "C" {
    
    
    
}
pub use crate::src::kazmath::vec3::kmVec3Add;
pub use crate::src::kazmath::vec3::kmVec3Assign;
pub use crate::src::kazmath::vec3::kmVec3Scale;
// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
// #[derive(Copy, Clone)]

pub type kmPlane = crate::src::kazmath::mat4::kmPlane;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay3 {
    pub start: crate::src::kazmath::aabb3::kmVec3,
    pub dir: crate::src::kazmath::aabb3::kmVec3,
}
impl kmRay3 {
    pub const fn new() -> Self {
        kmRay3 {
        start: crate::src::kazmath::aabb3::kmVec3::new(),
        dir: crate::src::kazmath::aabb3::kmVec3::new()
        }
    }
}

impl std::default::Default for kmRay3 {
    fn default() -> Self { kmRay3::new() }
}

#[no_mangle]
pub unsafe extern "C" fn kmRay3Fill<'a1, 'a2>(
    mut ray: Option<&'a1 mut crate::src::kazmath::ray3::kmRay3>,
    mut px: std::os::raw::c_float,
    mut py: std::os::raw::c_float,
    mut pz: std::os::raw::c_float,
    mut vx: std::os::raw::c_float,
    mut vy: std::os::raw::c_float,
    mut vz: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::ray3::kmRay3> where 'a1: 'a2 {
    (*(borrow_mut(&mut ray)).unwrap()).start.x = px;
    (*(borrow_mut(&mut ray)).unwrap()).start.y = py;
    (*(borrow_mut(&mut ray)).unwrap()).start.z = pz;
    (*(borrow_mut(&mut ray)).unwrap()).dir.x = vx;
    (*(borrow_mut(&mut ray)).unwrap()).dir.y = vy;
    (*(borrow_mut(&mut ray)).unwrap()).dir.z = vz;
    return ray;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3FromPointAndDirection<'a1, 'a2>(
    mut ray: Option<&'a1 mut crate::src::kazmath::ray3::kmRay3>,
    mut point: * const crate::src::kazmath::aabb3::kmVec3,
    mut direction: * const crate::src::kazmath::aabb3::kmVec3,
) -> Option<&'a2 mut crate::src::kazmath::ray3::kmRay3> where 'a1: 'a2 {
    kmVec3Assign(&mut (*(borrow_mut(&mut ray)).unwrap()).start, point);
    kmVec3Assign(&mut (*(borrow_mut(&mut ray)).unwrap()).dir, direction);
    return ray;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3IntersectPlane<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut ray: Option<&'a1 crate::src::kazmath::ray3::kmRay3>,
    mut plane: Option<&'a2 crate::src::kazmath::mat4::kmPlane>,
) -> std::os::raw::c_uchar {
    let mut d = (*((plane).clone()).unwrap()).a * (*((ray).clone()).unwrap()).dir.x + (*((plane).clone()).unwrap()).b * (*((ray).clone()).unwrap()).dir.y
        + (*((plane).clone()).unwrap()).c * (*((ray).clone()).unwrap()).dir.z;
    if d == 0 as std::os::raw::c_int as std::os::raw::c_float {
        return 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    }
    let mut t = -((*((plane).clone()).unwrap()).a * (*((ray).clone()).unwrap()).start.x + (*((plane).clone()).unwrap()).b * (*((ray).clone()).unwrap()).start.y
        + (*((plane).clone()).unwrap()).c * (*((ray).clone()).unwrap()).start.z + (*((plane).clone()).unwrap()).d) / d;
    if t < 0 as std::os::raw::c_int as std::os::raw::c_float {
        return 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    }
    let mut scaled_dir = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Scale(&mut scaled_dir, &(*((ray).clone()).unwrap()).dir, t);
    kmVec3Add(pOut, &(*((ray).clone()).unwrap()).start, Some(&mut scaled_dir));
    return 1 as std::os::raw::c_int as std::os::raw::c_uchar;
}
use crate::laertes_rt::*;
