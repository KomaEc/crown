
extern "C" {
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn abs(_: std::os::raw::c_int) -> std::os::raw::c_int;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    
    
    
    
    
}
pub use crate::src::kazmath::utility::kmAlmostEqual;
pub use crate::src::kazmath::vec3::kmVec3Cross;
pub use crate::src::kazmath::vec3::kmVec3Dot;
pub use crate::src::kazmath::vec3::kmVec3Fill;
pub use crate::src::kazmath::vec3::kmVec3Length;
pub use crate::src::kazmath::vec3::kmVec3Normalize;
pub use crate::src::kazmath::vec3::kmVec3Scale;
pub use crate::src::kazmath::vec3::kmVec3Subtract;
// #[derive(Copy, Clone)]

pub type kmMat4 = crate::src::kazmath::mat3::kmMat4;
// #[derive(Copy, Clone)]

pub type kmPlane = crate::src::kazmath::mat4::kmPlane;
// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec4 {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
    pub w: std::os::raw::c_float,
}
impl kmVec4 {
    pub const fn new() -> Self {
        kmVec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0
        }
    }
}

impl std::default::Default for kmVec4 {
    fn default() -> Self { kmVec4::new() }
}

pub type KM_POINT_CLASSIFICATION = std::os::raw::c_int;
pub const POINT_INFRONT_OF_PLANE: KM_POINT_CLASSIFICATION = 1;
pub const POINT_ON_PLANE: KM_POINT_CLASSIFICATION = 0;
pub const POINT_BEHIND_PLANE: KM_POINT_CLASSIFICATION = -1;
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDot<'a1, 'a2>(
    mut pP: Option<&'a1 crate::src::kazmath::mat4::kmPlane>,
    mut pV: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
) -> std::os::raw::c_float {
    return (*((pP).clone()).unwrap()).a * (*((pV).clone()).unwrap()).x + (*((pP).clone()).unwrap()).b * (*((pV).clone()).unwrap()).y + (*((pP).clone()).unwrap()).c * (*((pV).clone()).unwrap()).z + (*((pP).clone()).unwrap()).d * (*((pV).clone()).unwrap()).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDotCoord<'a1, 'a2>(
    mut pP: Option<&'a1 crate::src::kazmath::mat4::kmPlane>,
    mut pV: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
) -> std::os::raw::c_float {
    return (*((pP).clone()).unwrap()).a * (*((pV).clone()).unwrap()).x + (*((pP).clone()).unwrap()).b * (*((pV).clone()).unwrap()).y + (*((pP).clone()).unwrap()).c * (*((pV).clone()).unwrap()).z + (*((pP).clone()).unwrap()).d;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneDotNormal<'a1, 'a2>(
    mut pP: Option<&'a1 crate::src::kazmath::mat4::kmPlane>,
    mut pV: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
) -> std::os::raw::c_float {
    return (*((pP).clone()).unwrap()).a * (*((pV).clone()).unwrap()).x + (*((pP).clone()).unwrap()).b * (*((pV).clone()).unwrap()).y + (*((pP).clone()).unwrap()).c * (*((pV).clone()).unwrap()).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromNormalAndDistance<'a1, 'a2, 'a3>(
    mut plane: Option<&'a1 mut crate::src::kazmath::mat4::kmPlane>,
    mut normal: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    dist: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::kazmath::mat4::kmPlane> where 'a1: 'a3 {
    (*(borrow_mut(&mut plane)).unwrap()).a = (*(normal).unwrap()).x;
    (*(borrow_mut(&mut plane)).unwrap()).b = (*(normal).unwrap()).y;
    (*(borrow_mut(&mut plane)).unwrap()).c = (*(normal).unwrap()).z;
    (*(borrow_mut(&mut plane)).unwrap()).d = dist;
    return plane;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromPointAndNormal<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat4::kmPlane>,
    mut pPoint: * const crate::src::kazmath::aabb3::kmVec3,
    mut pNormal: * const crate::src::kazmath::aabb3::kmVec3,
) -> Option<&'a2 mut crate::src::kazmath::mat4::kmPlane> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).a = (*pNormal).x;
    (*(borrow_mut(&mut pOut)).unwrap()).b = (*pNormal).y;
    (*(borrow_mut(&mut pOut)).unwrap()).c = (*pNormal).z;
    (*(borrow_mut(&mut pOut)).unwrap()).d = -kmVec3Dot(pNormal, pPoint);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFromPoints<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat4::kmPlane>,
    mut p1: * const crate::src::kazmath::aabb3::kmVec3,
    mut p2: * const crate::src::kazmath::aabb3::kmVec3,
    mut p3: * const crate::src::kazmath::aabb3::kmVec3,
) -> Option<&'a2 mut crate::src::kazmath::mat4::kmPlane> where 'a1: 'a2 {
    let mut n = kmVec3 { x: 0., y: 0., z: 0. };
    let mut v1 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut v2 = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Subtract(&mut v1, p2, p1);
    kmVec3Subtract(&mut v2, p3, p1);
    kmVec3Cross(&mut n, Some(&mut v1), &mut v2);
    kmVec3Normalize(&mut n, &mut n);
    (*(borrow_mut(&mut pOut)).unwrap()).a = n.x;
    (*(borrow_mut(&mut pOut)).unwrap()).b = n.y;
    (*(borrow_mut(&mut pOut)).unwrap()).c = n.z;
    (*(borrow_mut(&mut pOut)).unwrap()).d = kmVec3Dot(kmVec3Scale(&mut n, &mut n, -1.0f64 as std::os::raw::c_float), p1);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneIntersectLine<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pP: Option<&'a2 crate::src::kazmath::mat4::kmPlane>,
    mut pV1: * const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: * const crate::src::kazmath::aabb3::kmVec3,
) -> Option<&'a3 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a3 {
    let mut d = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Subtract(&mut d, pV2, pV1);
    let mut n = kmVec3 { x: 0., y: 0., z: 0. };
    n.x = (*(pP).unwrap()).a;
    n.y = (*(pP).unwrap()).b;
    n.z = (*(pP).unwrap()).c;
    kmVec3Normalize(&mut n, &mut n);
    let mut nt = -(n.x * (*pV1).x + n.y * (*pV1).y + n.z * (*pV1).z + (*((pP).clone()).unwrap()).d);
    let mut dt = n.x * d.x + n.y * d.y + n.z * d.z;
    if fabs(dt as std::os::raw::c_double) < 0.0001f64 {
        pOut = Option::<&'_ mut crate::src::kazmath::aabb3::kmVec3>::None;
        return pOut;
    }
    let mut t = nt / dt;
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*pV1).x + d.x * t;
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*pV1).y + d.y * t;
    (*(borrow_mut(&mut pOut)).unwrap()).z = (*pV1).z + d.z * t;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneNormalize(
    mut pOut: * mut crate::src::kazmath::mat4::kmPlane,
    mut pP: * const crate::src::kazmath::mat4::kmPlane,
) -> * mut crate::src::kazmath::mat4::kmPlane {
    let mut n = kmVec3 { x: 0., y: 0., z: 0. };
    let mut l = 0 as std::os::raw::c_int as std::os::raw::c_float;
    if (*pP).a == 0. && (*pP).b == 0. && (*pP).c == 0. {
        (*pOut).a = (*pP).a;
        (*pOut).b = (*pP).b;
        (*pOut).c = (*pP).c;
        (*pOut).d = (*pP).d;
        return pOut;
    }
    n.x = (*pP).a;
    n.y = (*pP).b;
    n.z = (*pP).c;
    l = 1.0f32 / kmVec3Length(&mut n);
    kmVec3Normalize(&mut n, &mut n);
    (*pOut).a = n.x;
    (*pOut).b = n.y;
    (*pOut).c = n.z;
    (*pOut).d = (*pP).d * l;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneScale<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat4::kmPlane>,
    mut pP: Option<&'a2 crate::src::kazmath::mat4::kmPlane>,
    mut s: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::kazmath::mat4::kmPlane> {
    if 0 as std::os::raw::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const std::os::raw::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/plane.c\0" as *const u8 as *const std::os::raw::c_char,
            179 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 57], &'_ [i8; 57]>(b"kmPlane *kmPlaneScale(kmPlane *, const kmPlane *, float)\0"))
                .as_ptr(),
        );
    }
    return Option::<&'_ mut crate::src::kazmath::mat4::kmPlane>::None;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneClassifyPoint<'a1, 'a2>(
    mut pIn: Option<&'a1 crate::src::kazmath::mat4::kmPlane>,
    mut pP: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
) -> std::os::raw::c_int {
    let mut distance = (*((pIn).clone()).unwrap()).a * (*((pP).clone()).unwrap()).x + (*((pIn).clone()).unwrap()).b * (*((pP).clone()).unwrap()).y + (*((pIn).clone()).unwrap()).c * (*((pP).clone()).unwrap()).z
        + (*((pIn).clone()).unwrap()).d;
    if distance as std::os::raw::c_double > 0.0001f64 {
        return POINT_INFRONT_OF_PLANE;
    }
    if (distance as std::os::raw::c_double) < -0.0001f64 {
        return POINT_BEHIND_PLANE;
    }
    return POINT_ON_PLANE;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneExtractFromMat4<'a1>(
    mut pOut: * mut crate::src::kazmath::mat4::kmPlane,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
    mut row: std::os::raw::c_int,
) -> * mut crate::src::kazmath::mat4::kmPlane {
    let mut scale = if row < 0 as std::os::raw::c_int {
        -(1 as std::os::raw::c_int)
    } else {
        1 as std::os::raw::c_int
    };
    row = abs(row) - 1 as std::os::raw::c_int;
    (*pOut)
        .a = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
        + scale as std::os::raw::c_float * (*((pIn).clone()).unwrap()).mat[row as usize];
    (*pOut)
        .b = (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        + scale as std::os::raw::c_float * (*((pIn).clone()).unwrap()).mat[(row + 4 as std::os::raw::c_int) as usize];
    (*pOut)
        .c = (*((pIn).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + scale as std::os::raw::c_float * (*((pIn).clone()).unwrap()).mat[(row + 8 as std::os::raw::c_int) as usize];
    (*pOut)
        .d = (*((pIn).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + scale as std::os::raw::c_float * (*((pIn).clone()).unwrap()).mat[(row + 12 as std::os::raw::c_int) as usize];
    return kmPlaneNormalize(pOut, pOut);
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneGetIntersection<'a1, 'a2, 'a3>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut p1: Option<&'a1 crate::src::kazmath::mat4::kmPlane>,
    mut p2: Option<&'a2 crate::src::kazmath::mat4::kmPlane>,
    mut p3: Option<&'a3 crate::src::kazmath::mat4::kmPlane>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut n1 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut n2 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut n3 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut cross = kmVec3 { x: 0., y: 0., z: 0. };
    let mut r1 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut r2 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut r3 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut denom = 0 as std::os::raw::c_int as std::os::raw::c_double;
    borrow(& kmVec3Fill(Some(&mut n1), (*(p1).unwrap()).a, (*(p1).unwrap()).b, (*(p1).unwrap()).c));
    borrow(& kmVec3Fill(Some(&mut n2), (*(p2).unwrap()).a, (*(p2).unwrap()).b, (*(p2).unwrap()).c));
    borrow(& kmVec3Fill(Some(&mut n3), (*(p3).unwrap()).a, (*(p3).unwrap()).b, (*(p3).unwrap()).c));
    kmVec3Cross(&mut cross, Some(&mut n2), &mut n3);
    denom = kmVec3Dot(&mut n1, &mut cross) as std::os::raw::c_double;
    if kmAlmostEqual(denom as std::os::raw::c_float, 0.0f64 as std::os::raw::c_float) != 0 {
        return 0 as *mut kmVec3;
    }
    kmVec3Cross(&mut r1, Some(&mut n2), &mut n3);
    kmVec3Cross(&mut r2, Some(&mut n3), &mut n1);
    kmVec3Cross(&mut r3, Some(&mut n1), &mut n2);
    kmVec3Scale(&mut r1, &mut r1, -(*((p1).clone()).unwrap()).d);
    kmVec3Scale(&mut r2, &mut r2, (*(p2).unwrap()).d);
    kmVec3Scale(&mut r3, &mut r3, (*(p3).unwrap()).d);
    kmVec3Subtract(pOut, &mut r1, &mut r2);
    kmVec3Subtract(pOut, pOut, &mut r3);
    kmVec3Scale(pOut, pOut, (1.0f64 / denom) as std::os::raw::c_float);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmPlaneFill<'a1, 'a2>(
    mut plane: Option<&'a1 mut crate::src::kazmath::mat4::kmPlane>,
    mut a: std::os::raw::c_float,
    mut b: std::os::raw::c_float,
    mut c: std::os::raw::c_float,
    mut d: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat4::kmPlane> where 'a1: 'a2 {
    (*(borrow_mut(&mut plane)).unwrap()).a = a;
    (*(borrow_mut(&mut plane)).unwrap()).b = b;
    (*(borrow_mut(&mut plane)).unwrap()).c = c;
    (*(borrow_mut(&mut plane)).unwrap()).d = d;
    return plane;
}
use crate::laertes_rt::*;
