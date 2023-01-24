use ::libc;
extern "C" {
    
    
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor12 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor13 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay3 {
    pub start: crate::src::kazmath::aabb3::kmVec3,
    pub dir: crate::src::kazmath::aabb3::kmVec3,
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3Fill(
    mut ray: Option<&mut kmRay3>,
    mut px: libc::c_float,
    mut py: libc::c_float,
    mut pz: libc::c_float,
    mut vx: libc::c_float,
    mut vy: libc::c_float,
    mut vz: libc::c_float,
) -> *mut kmRay3 {
    (*ray.as_deref_mut().unwrap()).start.x= px;
    (*ray.as_deref_mut().unwrap()).start.y= py;
    (*ray.as_deref_mut().unwrap()).start.z= pz;
    (*ray.as_deref_mut().unwrap()).dir.x= vx;
    (*ray.as_deref_mut().unwrap()).dir.y= vy;
    (*ray.as_deref_mut().unwrap()).dir.z= vz;
    return ray.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3FromPointAndDirection(
    mut ray: *mut kmRay3,
    mut point: *const crate::src::kazmath::aabb3::kmVec3,
    mut direction: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut kmRay3 {
    crate::src::kazmath::vec3::kmVec3Assign(core::ptr::addr_of_mut!((*ray).start), point);
    crate::src::kazmath::vec3::kmVec3Assign(core::ptr::addr_of_mut!((*ray).dir), direction);
    return ray;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3IntersectPlane(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut ray: *const kmRay3,
    mut plane: *const crate::src::kazmath::mat4::kmPlane,
) -> libc::c_uchar {
    let mut d = (*plane).a * (*ray).dir.x + (*plane).b * (*ray).dir.y
        + (*plane).c * (*ray).dir.z;
    if d == 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as libc::c_uchar;
    }
    let mut t = -((*plane).a * (*ray).start.x + (*plane).b * (*ray).start.y
        + (*plane).c * (*ray).start.z + (*plane).d) / d;
    if t < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as libc::c_uchar;
    }
    let mut scaled_dir = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    crate::src::kazmath::vec3::kmVec3Scale(core::ptr::addr_of_mut!(scaled_dir), &(*ray).dir, t);
    crate::src::kazmath::vec3::kmVec3Add(pOut, &(*ray).start, core::ptr::addr_of!(scaled_dir));
    return 1 as libc::c_int as libc::c_uchar;
}
