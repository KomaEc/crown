use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    
    
    
    
    
    
    
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor5 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor6 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec4 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
pub type KM_POINT_CLASSIFICATION = libc::c_int;
pub const POINT_INFRONT_OF_PLANE: KM_POINT_CLASSIFICATION = 1;
pub const POINT_ON_PLANE: KM_POINT_CLASSIFICATION = 0;
pub const POINT_BEHIND_PLANE: KM_POINT_CLASSIFICATION = -1;
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDot(
    mut pP: *const crate::src::kazmath::mat4::kmPlane,
    mut pV: *const kmVec4,
) -> libc::c_float {
    return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z + (*pP).d * (*pV).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDotCoord(
    mut pP: *const crate::src::kazmath::mat4::kmPlane,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
) -> libc::c_float {
    return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z + (*pP).d;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDotNormal(
    mut pP: *const crate::src::kazmath::mat4::kmPlane,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
) -> libc::c_float {
    return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromNormalAndDistance(
    mut plane: Option<&mut crate::src::kazmath::mat4::kmPlane>,
    mut normal: *const crate::src::kazmath::aabb3::kmVec3,
    dist: libc::c_float,
) -> *mut crate::src::kazmath::mat4::kmPlane {
    (*plane.as_deref_mut().unwrap()).a= (*normal).x;
    (*plane.as_deref_mut().unwrap()).b= (*normal).y;
    (*plane.as_deref_mut().unwrap()).c= (*normal).z;
    (*plane.as_deref_mut().unwrap()).d= dist;
    return plane.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromPointAndNormal(
    mut pOut: Option<&mut crate::src::kazmath::mat4::kmPlane>,
    mut pPoint: *const crate::src::kazmath::aabb3::kmVec3,
    mut pNormal: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::mat4::kmPlane {
    (*pOut.as_deref_mut().unwrap()).a= (*pNormal).x;
    (*pOut.as_deref_mut().unwrap()).b= (*pNormal).y;
    (*pOut.as_deref_mut().unwrap()).c= (*pNormal).z;
    (*pOut.as_deref_mut().unwrap()).d= -crate::src::kazmath::vec3::kmVec3Dot(pNormal, pPoint);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromPoints(
    mut pOut: Option<&mut crate::src::kazmath::mat4::kmPlane>,
    mut p1: *const crate::src::kazmath::aabb3::kmVec3,
    mut p2: *const crate::src::kazmath::aabb3::kmVec3,
    mut p3: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::mat4::kmPlane {
    let mut n = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut v1 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut v2 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    crate::src::kazmath::vec3::kmVec3Subtract(core::ptr::addr_of_mut!(v1), p2, p1);
    crate::src::kazmath::vec3::kmVec3Subtract(core::ptr::addr_of_mut!(v2), p3, p1);
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(n), core::ptr::addr_of!(v1), core::ptr::addr_of!(v2));
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(n), core::ptr::addr_of!(n));
    (*pOut.as_deref_mut().unwrap()).a= n.x;
    (*pOut.as_deref_mut().unwrap()).b= n.y;
    (*pOut.as_deref_mut().unwrap()).c= n.z;
    (*pOut.as_deref_mut().unwrap()).d= crate::src::kazmath::vec3::kmVec3Dot(crate::src::kazmath::vec3::kmVec3Scale(core::ptr::addr_of_mut!(n), core::ptr::addr_of!(n), -1.0f64 as libc::c_float), p1);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneIntersectLine(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pP: *const crate::src::kazmath::mat4::kmPlane,
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut d = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    crate::src::kazmath::vec3::kmVec3Subtract(core::ptr::addr_of_mut!(d), pV2, pV1);
    let mut n = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    n.x= (*pP).a;
    n.y= (*pP).b;
    n.z= (*pP).c;
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(n), core::ptr::addr_of!(n));
    let mut nt = -(n.x * (*pV1).x + n.y * (*pV1).y + n.z * (*pV1).z + (*pP).d);
    let mut dt = n.x * d.x + n.y * d.y + n.z * d.z;
    if fabs(dt as libc::c_double) < 0.0001f64 {
        pOut= 0 as *mut crate::src::kazmath::aabb3::kmVec3;
        return pOut;
    }
    let mut t = nt / dt;
    (*pOut).x= (*pV1).x + d.x * t;
    (*pOut).y= (*pV1).y + d.y * t;
    (*pOut).z= (*pV1).z + d.z * t;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneNormalize(
    mut pOut: *mut crate::src::kazmath::mat4::kmPlane,
    mut pP: *const crate::src::kazmath::mat4::kmPlane,
) -> *mut crate::src::kazmath::mat4::kmPlane {
    let mut n = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut l = 0 as libc::c_int as libc::c_float;
    if (*pP).a == 0. && (*pP).b == 0. && (*pP).c == 0. {
        (*pOut).a= (*pP).a;
        (*pOut).b= (*pP).b;
        (*pOut).c= (*pP).c;
        (*pOut).d= (*pP).d;
        return pOut;
    }
    n.x= (*pP).a;
    n.y= (*pP).b;
    n.z= (*pP).c;
    l= 1.0f32 / crate::src::kazmath::vec3::kmVec3Length(core::ptr::addr_of!(n));
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(n), core::ptr::addr_of!(n));
    (*pOut).a= n.x;
    (*pOut).b= n.y;
    (*pOut).c= n.z;
    (*pOut).d= (*pP).d * l;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneScale(
    mut pOut: *mut crate::src::kazmath::mat4::kmPlane,
    mut pP: *const crate::src::kazmath::mat4::kmPlane,
    mut s: libc::c_float,
) -> *mut crate::src::kazmath::mat4::kmPlane {
    if 0 as libc::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const libc::c_char,
            b"../kazmath/plane.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int as libc::c_uint,
            b"kmPlane *kmPlaneScale(kmPlane *, const kmPlane *, float)\0" as *const u8 as *const i8,
        );
    }
    return 0 as *mut crate::src::kazmath::mat4::kmPlane;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneClassifyPoint(
    mut pIn: *const crate::src::kazmath::mat4::kmPlane,
    mut pP: *const crate::src::kazmath::aabb3::kmVec3,
) -> KM_POINT_CLASSIFICATION {
    let mut distance = (*pIn).a * (*pP).x + (*pIn).b * (*pP).y + (*pIn).c * (*pP).z
        + (*pIn).d;
    if distance as libc::c_double > 0.0001f64 {
        return POINT_INFRONT_OF_PLANE;
    }
    if (distance as libc::c_double) < -0.0001f64 {
        return POINT_BEHIND_PLANE;
    }
    return POINT_ON_PLANE;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneExtractFromMat4(
    mut pOut: Option<&mut crate::src::kazmath::mat4::kmPlane>,
    mut pIn: *const crate::src::kazmath::mat3::kmMat4,
    mut row: libc::c_int,
) -> *mut crate::src::kazmath::mat4::kmPlane {
    let mut scale = if row < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    row= abs(row) - 1 as libc::c_int;
    (*pOut.as_deref_mut().unwrap()).a= (*pIn).mat[3 as libc::c_int as usize]
        + scale as libc::c_float * (*pIn).mat[row as usize];
    (*pOut.as_deref_mut().unwrap()).b= (*pIn).mat[7 as libc::c_int as usize]
        + scale as libc::c_float * (*pIn).mat[(row + 4 as libc::c_int) as usize];
    (*pOut.as_deref_mut().unwrap()).c= (*pIn).mat[11 as libc::c_int as usize]
        + scale as libc::c_float * (*pIn).mat[(row + 8 as libc::c_int) as usize];
    (*pOut.as_deref_mut().unwrap()).d= (*pIn).mat[15 as libc::c_int as usize]
        + scale as libc::c_float * (*pIn).mat[(row + 12 as libc::c_int) as usize];
    return kmPlaneNormalize(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pOut.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneGetIntersection(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut p1: *const crate::src::kazmath::mat4::kmPlane,
    mut p2: *const crate::src::kazmath::mat4::kmPlane,
    mut p3: *const crate::src::kazmath::mat4::kmPlane,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut n1 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut n2 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut n3 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut cross = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut r1 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut r2 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut r3 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut denom = 0 as libc::c_int as libc::c_double;
    crate::src::kazmath::vec3::kmVec3Fill(Some(&mut n1), (*p1).a, (*p1).b, (*p1).c);
    crate::src::kazmath::vec3::kmVec3Fill(Some(&mut n2), (*p2).a, (*p2).b, (*p2).c);
    crate::src::kazmath::vec3::kmVec3Fill(Some(&mut n3), (*p3).a, (*p3).b, (*p3).c);
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(cross), core::ptr::addr_of!(n2), core::ptr::addr_of!(n3));
    denom= crate::src::kazmath::vec3::kmVec3Dot(core::ptr::addr_of!(n1), core::ptr::addr_of!(cross)) as libc::c_double;
    if crate::src::kazmath::utility::kmAlmostEqual(denom as libc::c_float, 0.0f64 as libc::c_float) != 0 {
        return 0 as *mut crate::src::kazmath::aabb3::kmVec3;
    }
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(r1), core::ptr::addr_of!(n2), core::ptr::addr_of!(n3));
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(r2), core::ptr::addr_of!(n3), core::ptr::addr_of!(n1));
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(r3), core::ptr::addr_of!(n1), core::ptr::addr_of!(n2));
    crate::src::kazmath::vec3::kmVec3Scale(core::ptr::addr_of_mut!(r1), core::ptr::addr_of!(r1), -(*p1).d);
    crate::src::kazmath::vec3::kmVec3Scale(core::ptr::addr_of_mut!(r2), core::ptr::addr_of!(r2), (*p2).d);
    crate::src::kazmath::vec3::kmVec3Scale(core::ptr::addr_of_mut!(r3), core::ptr::addr_of!(r3), (*p3).d);
    crate::src::kazmath::vec3::kmVec3Subtract(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of!(r1), core::ptr::addr_of!(r2));
    crate::src::kazmath::vec3::kmVec3Subtract(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pOut.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()), core::ptr::addr_of!(r3));
    crate::src::kazmath::vec3::kmVec3Scale(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pOut.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()), (1.0f64 / denom) as libc::c_float);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFill(
    mut plane: Option<&mut crate::src::kazmath::mat4::kmPlane>,
    mut a: libc::c_float,
    mut b: libc::c_float,
    mut c: libc::c_float,
    mut d: libc::c_float,
) -> *mut crate::src::kazmath::mat4::kmPlane {
    (*plane.as_deref_mut().unwrap()).a= a;
    (*plane.as_deref_mut().unwrap()).b= b;
    (*plane.as_deref_mut().unwrap()).c= c;
    (*plane.as_deref_mut().unwrap()).d= d;
    return plane.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
