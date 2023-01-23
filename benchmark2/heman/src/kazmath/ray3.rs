use ::libc;
extern "C" {
    fn kmVec3Add(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
    ) -> *mut kmVec3;
    fn kmVec3Scale(
        pOut: *mut kmVec3,
        pIn: *const kmVec3,
        s: libc::c_float,
    ) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmPlane {
    pub a: libc::c_float,
    pub b: libc::c_float,
    pub c: libc::c_float,
    pub d: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay3 {
    pub start: kmVec3,
    pub dir: kmVec3,
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3Fill(
    mut ray: *mut kmRay3,
    mut px: libc::c_float,
    mut py: libc::c_float,
    mut pz: libc::c_float,
    mut vx: libc::c_float,
    mut vy: libc::c_float,
    mut vz: libc::c_float,
) -> *mut kmRay3 {
    (*ray).start.x = px;
    (*ray).start.y = py;
    (*ray).start.z = pz;
    (*ray).dir.x = vx;
    (*ray).dir.y = vy;
    (*ray).dir.z = vz;
    return ray;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3FromPointAndDirection(
    mut ray: *mut kmRay3,
    mut point: *const kmVec3,
    mut direction: *const kmVec3,
) -> *mut kmRay3 {
    kmVec3Assign(&mut (*ray).start, point);
    kmVec3Assign(&mut (*ray).dir, direction);
    return ray;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay3IntersectPlane(
    mut pOut: *mut kmVec3,
    mut ray: *const kmRay3,
    mut plane: *const kmPlane,
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
    let mut scaled_dir = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Scale(&mut scaled_dir, &(*ray).dir, t);
    kmVec3Add(pOut, &(*ray).start, &mut scaled_dir);
    return 1 as libc::c_int as libc::c_uchar;
}
