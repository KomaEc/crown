
extern "C" {
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn atan2(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    fn cosf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sinf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
}
pub use crate::src::kazmath::utility::kmDegreesToRadians;
pub use crate::src::kazmath::utility::kmRadiansToDegrees;
pub use crate::src::kazmath::utility::kmSQR;
// #[derive(Copy, Clone)]

pub type kmMat3 = crate::src::kazmath::mat3::kmMat3;
// #[derive(Copy, Clone)]

pub type kmVec2 = crate::src::kazmath::aabb2::kmVec2;
#[no_mangle]
pub static mut KM_VEC2_POS_Y: crate::src::kazmath::aabb2::kmVec2 = crate::src::kazmath::aabb2::kmVec2::new(); unsafe fn laertes_init_KM_VEC2_POS_Y() {
KM_VEC2_POS_Y = {
    let mut init = kmVec2 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: 1 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC2_NEG_Y: crate::src::kazmath::aabb2::kmVec2 = crate::src::kazmath::aabb2::kmVec2::new(); unsafe fn laertes_init_KM_VEC2_NEG_Y() {
KM_VEC2_NEG_Y = {
    let mut init = kmVec2 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: -(1 as std::os::raw::c_int) as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC2_NEG_X: crate::src::kazmath::aabb2::kmVec2 = crate::src::kazmath::aabb2::kmVec2::new(); unsafe fn laertes_init_KM_VEC2_NEG_X() {
KM_VEC2_NEG_X = {
    let mut init = kmVec2 {
        x: -(1 as std::os::raw::c_int) as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC2_POS_X: crate::src::kazmath::aabb2::kmVec2 = crate::src::kazmath::aabb2::kmVec2::new(); unsafe fn laertes_init_KM_VEC2_POS_X() {
KM_VEC2_POS_X = {
    let mut init = kmVec2 {
        x: 1 as std::os::raw::c_int as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC2_ZERO: crate::src::kazmath::aabb2::kmVec2 = crate::src::kazmath::aabb2::kmVec2::new(); unsafe fn laertes_init_KM_VEC2_ZERO() {
KM_VEC2_ZERO = {
    let mut init = kmVec2 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub unsafe extern "C" fn kmVec2Fill<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::aabb2::kmVec2> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Length(mut pIn: * const crate::src::kazmath::aabb2::kmVec2) -> std::os::raw::c_float {
    return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2LengthSq<'a1>(mut pIn: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>) -> std::os::raw::c_float {
    return kmSQR((*(pIn).unwrap()).x) + kmSQR((*(pIn).unwrap()).y);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Lerp<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV1: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut pV2: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
    mut t: std::os::raw::c_float,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x + t * ((*((pV2).clone()).unwrap()).x - (*((pV1).clone()).unwrap()).x);
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y + t * ((*((pV2).clone()).unwrap()).y - (*((pV1).clone()).unwrap()).y);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Normalize(
    mut pOut: * mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: * const crate::src::kazmath::aabb2::kmVec2,
) -> * mut crate::src::kazmath::aabb2::kmVec2 {
    if (*pIn).x == 0. && (*pIn).y == 0. {
        return kmVec2Assign(pOut, pIn);
    }
    let mut l = 1.0f32 / kmVec2Length(pIn);
    let mut v = kmVec2 { x: 0., y: 0. };
    v.x = (*pIn).x * l;
    v.y = (*pIn).y * l;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Add<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::aabb2::kmVec2,
    mut pV1: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>,
    mut pV2: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
) -> * mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut).x = (*((pV1).clone()).unwrap()).x + (*((pV2).clone()).unwrap()).x;
    (*pOut).y = (*((pV1).clone()).unwrap()).y + (*((pV2).clone()).unwrap()).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Dot<'a1>(
    mut pV1: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>,
    mut pV2: * const crate::src::kazmath::aabb2::kmVec2,
) -> std::os::raw::c_float {
    return (*((pV1).clone()).unwrap()).x * (*pV2).x + (*((pV1).clone()).unwrap()).y * (*pV2).y;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Cross<'a1, 'a2>(
    mut pV1: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>,
    mut pV2: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_float {
    return (*((pV1).clone()).unwrap()).x * (*((pV2).clone()).unwrap()).y - (*((pV1).clone()).unwrap()).y * (*((pV2).clone()).unwrap()).x;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Subtract<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::aabb2::kmVec2,
    mut pV1: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>,
    mut pV2: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
) -> * mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut).x = (*((pV1).clone()).unwrap()).x - (*((pV2).clone()).unwrap()).x;
    (*pOut).y = (*((pV1).clone()).unwrap()).y - (*((pV2).clone()).unwrap()).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Mul<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV1: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut pV2: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x * (*((pV2).clone()).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y * (*((pV2).clone()).unwrap()).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Div<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV1: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut pV2: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2> where 'a1: 'a4 {
    if (*((pV2).clone()).unwrap()).x != 0. && (*((pV2).clone()).unwrap()).y != 0. {
        (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x / (*((pV2).clone()).unwrap()).x;
        (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y / (*((pV2).clone()).unwrap()).y;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Transform<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut pM: Option<&'a3 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2> where 'a1: 'a4 {
    let mut v = kmVec2 { x: 0., y: 0. };
    v
        .x = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    v
        .y = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).x = v.x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = v.y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2TransformCoord<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut pM: Option<&'a3 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2> {
    __assert_fail(
        b"0\0" as *const u8 as *const std::os::raw::c_char,
        b"../kazmath/vec2.c\0" as *const u8 as *const std::os::raw::c_char,
        134 as std::os::raw::c_int as std::os::raw::c_uint,
        (*core::intrinsics::transmute::<&'_ [u8; 71], &'_ [i8; 71]>(b"kmVec2 *kmVec2TransformCoord(kmVec2 *, const kmVec2 *, const kmMat3 *)\0"))
            .as_ptr(),
    );
    return Option::<&'_ mut crate::src::kazmath::aabb2::kmVec2>::None;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Scale(
    mut pOut: * mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: * const crate::src::kazmath::aabb2::kmVec2,
    s: std::os::raw::c_float,
) -> * mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut).x = (*pIn).x * s;
    (*pOut).y = (*pIn).y * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2AreEqual(
    mut p1: * const crate::src::kazmath::aabb2::kmVec2,
    mut p2: * const crate::src::kazmath::aabb2::kmVec2,
) -> std::os::raw::c_int {
    return (((*p1).x as std::os::raw::c_double) < (*p2).x as std::os::raw::c_double + 0.0001f64
        && (*p1).x as std::os::raw::c_double > (*p2).x as std::os::raw::c_double - 0.0001f64
        && (((*p1).y as std::os::raw::c_double) < (*p2).y as std::os::raw::c_double + 0.0001f64
            && (*p1).y as std::os::raw::c_double > (*p2).y as std::os::raw::c_double - 0.0001f64))
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Assign(
    mut pOut: * mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: * const crate::src::kazmath::aabb2::kmVec2,
) -> * mut crate::src::kazmath::aabb2::kmVec2 {
    if pOut == pIn as *mut kmVec2 {
        return pOut;
    }
    (*pOut).x = (*pIn).x;
    (*pOut).y = (*pIn).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2RotateBy<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    degrees: std::os::raw::c_float,
    mut center: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2> where 'a1: 'a4 {
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let radians = kmDegreesToRadians(degrees);
    let cs = cosf(radians);
    let sn = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pIn).clone()).unwrap()).x - (*((center).clone()).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pIn).clone()).unwrap()).y - (*((center).clone()).unwrap()).y;
    x = (*(borrow(& pOut)).unwrap()).x * cs - (*(borrow(& pOut)).unwrap()).y * sn;
    y = (*(borrow(& pOut)).unwrap()).x * sn + (*(borrow(& pOut)).unwrap()).y * cs;
    (*(borrow_mut(&mut pOut)).unwrap()).x = x + (*((center).clone()).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = y + (*((center).clone()).unwrap()).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2DegreesBetween(
    mut v1: * const crate::src::kazmath::aabb2::kmVec2,
    mut v2: * const crate::src::kazmath::aabb2::kmVec2,
) -> std::os::raw::c_float {
    if kmVec2AreEqual(v1, v2) != 0 {
        return 0.0f64 as std::os::raw::c_float;
    }
    let mut t1 = kmVec2 { x: 0., y: 0. };
    let mut t2 = kmVec2 { x: 0., y: 0. };
    kmVec2Normalize(&mut t1, v1);
    kmVec2Normalize(&mut t2, v2);
    let mut cross = kmVec2Cross(Some(&mut t1), Some(&mut t2));
    let mut dot = kmVec2Dot(Some(&mut t1), &mut t2);
    if dot as std::os::raw::c_double > 1.0f64 {
        dot = 1.0f64 as std::os::raw::c_float;
    }
    if (dot as std::os::raw::c_double) < -1.0f64 {
        dot = -1.0f64 as std::os::raw::c_float;
    }
    return kmRadiansToDegrees(
        atan2(cross as std::os::raw::c_double, dot as std::os::raw::c_double) as std::os::raw::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2DistanceBetween<'a1, 'a2>(
    mut v1: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>,
    mut v2: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_float {
    let mut diff = kmVec2 { x: 0., y: 0. };
    kmVec2Subtract(&mut diff, (v2).clone(), (v1).clone());
    return fabs(kmVec2Length(&mut diff) as std::os::raw::c_double) as std::os::raw::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2MidPointBetween<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmVec2>,
    mut v1: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut v2: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2> where 'a1: 'a4 {
    let mut sum = kmVec2 { x: 0., y: 0. };
    kmVec2Add(&mut sum, (v1).clone(), (v2).clone());
    (*(borrow_mut(&mut pOut)).unwrap()).x = sum.x / 2.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).y = sum.y / 2.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Reflect<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>,
    mut normal: * const crate::src::kazmath::aabb2::kmVec2,
) -> * mut crate::src::kazmath::aabb2::kmVec2 {
    let mut tmp = kmVec2 { x: 0., y: 0. };
    kmVec2Scale(&mut tmp, normal, 2.0f32 * kmVec2Dot((pIn).clone(), normal));
    kmVec2Subtract(pOut, (pIn).clone(), Some(&mut tmp));
    return pOut;
}
use crate::laertes_rt::*;
