
extern "C" {
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn acosf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn acos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn asin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn atan2(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn cosf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sinf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::kazmath::mat3::kmMat3LookAt;
pub use crate::src::kazmath::utility::kmClamp;
pub use crate::src::kazmath::utility::kmSQR;
pub use crate::src::kazmath::vec3::kmVec3Add;
pub use crate::src::kazmath::vec3::kmVec3Assign;
pub use crate::src::kazmath::vec3::kmVec3Cross;
pub use crate::src::kazmath::vec3::kmVec3Dot;
pub use crate::src::kazmath::vec3::kmVec3LengthSq;
pub use crate::src::kazmath::vec3::kmVec3Normalize;
pub use crate::src::kazmath::vec3::kmVec3Scale;
pub use crate::src::kazmath::vec3::KM_VEC3_NEG_Z;
pub use crate::src::kazmath::vec3::KM_VEC3_POS_X;
pub use crate::src::kazmath::vec3::KM_VEC3_POS_Y;
pub use crate::src::kazmath::vec3::KM_VEC3_POS_Z;
pub use crate::src::kazmath::vec3::KM_VEC3_ZERO;
// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
// #[derive(Copy, Clone)]

pub type kmMat3 = crate::src::kazmath::mat3::kmMat3;
// #[derive(Copy, Clone)]

pub type kmQuaternion = crate::src::kazmath::mat3::kmQuaternion;
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAreEqual<'a1, 'a2>(
    mut p1: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut p2: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> std::os::raw::c_int {
    if ((*((p1).clone()).unwrap()).x as std::os::raw::c_double) < (*((p2).clone()).unwrap()).x as std::os::raw::c_double + 0.0001f64
        && (*((p1).clone()).unwrap()).x as std::os::raw::c_double > (*((p2).clone()).unwrap()).x as std::os::raw::c_double - 0.0001f64
        && (((*((p1).clone()).unwrap()).y as std::os::raw::c_double) < (*((p2).clone()).unwrap()).y as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).y as std::os::raw::c_double > (*((p2).clone()).unwrap()).y as std::os::raw::c_double - 0.0001f64)
        && (((*((p1).clone()).unwrap()).z as std::os::raw::c_double) < (*((p2).clone()).unwrap()).z as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).z as std::os::raw::c_double > (*((p2).clone()).unwrap()).z as std::os::raw::c_double - 0.0001f64)
        && (((*((p1).clone()).unwrap()).w as std::os::raw::c_double) < (*((p2).clone()).unwrap()).w as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).w as std::os::raw::c_double > (*((p2).clone()).unwrap()).w as std::os::raw::c_double - 0.0001f64)
    {
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionFill(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
    mut z: std::os::raw::c_float,
    mut w: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionDot<'a1, 'a2>(
    mut q1: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut q2: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> std::os::raw::c_float {
    return (*((q1).clone()).unwrap()).w * (*((q2).clone()).unwrap()).w + (*((q1).clone()).unwrap()).x * (*((q2).clone()).unwrap()).x + (*((q1).clone()).unwrap()).y * (*((q2).clone()).unwrap()).y + (*((q1).clone()).unwrap()).z * (*((q2).clone()).unwrap()).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionExp<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmQuaternion>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmQuaternion> where 'a1: 'a3 {
    __assert_fail(
        b"0\0" as *const u8 as *const std::os::raw::c_char,
        b"../kazmath/quaternion.c\0" as *const u8 as *const std::os::raw::c_char,
        68 as std::os::raw::c_int as std::os::raw::c_uint,
        (*core::intrinsics::transmute::<&'_ [u8; 68], &'_ [i8; 68]>(b"kmQuaternion *kmQuaternionExp(kmQuaternion *, const kmQuaternion *)\0"))
            .as_ptr(),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIdentity(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x = 0.0f64 as std::os::raw::c_float;
    (*pOut).y = 0.0f64 as std::os::raw::c_float;
    (*pOut).z = 0.0f64 as std::os::raw::c_float;
    (*pOut).w = 1.0f64 as std::os::raw::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionInverse<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmQuaternion>,
    mut pIn: * const crate::src::kazmath::mat3::kmQuaternion,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmQuaternion> where 'a1: 'a2 {
    let mut l = kmQuaternionLength(pIn);
    if fabs(l as std::os::raw::c_double) < 0.0001f64 {
        (*(borrow_mut(&mut pOut)).unwrap()).x = 0.0f64 as std::os::raw::c_float;
        (*(borrow_mut(&mut pOut)).unwrap()).y = 0.0f64 as std::os::raw::c_float;
        (*(borrow_mut(&mut pOut)).unwrap()).z = 0.0f64 as std::os::raw::c_float;
        (*(borrow_mut(&mut pOut)).unwrap()).w = 0.0f64 as std::os::raw::c_float;
        return pOut;
    }
    (*(borrow_mut(&mut pOut)).unwrap()).x = -(*pIn).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = -(*pIn).y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = -(*pIn).z;
    (*(borrow_mut(&mut pOut)).unwrap()).w = (*pIn).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIsIdentity<'a1>(
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> std::os::raw::c_int {
    return ((*((pIn).clone()).unwrap()).x as std::os::raw::c_double == 0.0f64 && (*((pIn).clone()).unwrap()).y as std::os::raw::c_double == 0.0f64
        && (*((pIn).clone()).unwrap()).z as std::os::raw::c_double == 0.0f64 && (*((pIn).clone()).unwrap()).w as std::os::raw::c_double == 1.0f64)
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLength(
    mut pIn: * const crate::src::kazmath::mat3::kmQuaternion,
) -> std::os::raw::c_float {
    return sqrt(kmQuaternionLengthSq(pIn) as std::os::raw::c_double) as std::os::raw::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLengthSq(
    mut pIn: * const crate::src::kazmath::mat3::kmQuaternion,
) -> std::os::raw::c_float {
    return (*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z
        + (*pIn).w * (*pIn).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLn<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmQuaternion>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmQuaternion> where 'a1: 'a3 {
    __assert_fail(
        b"0\0" as *const u8 as *const std::os::raw::c_char,
        b"../kazmath/quaternion.c\0" as *const u8 as *const std::os::raw::c_char,
        137 as std::os::raw::c_int as std::os::raw::c_uint,
        (*core::intrinsics::transmute::<&'_ [u8; 67], &'_ [i8; 67]>(b"kmQuaternion *kmQuaternionLn(kmQuaternion *, const kmQuaternion *)\0"))
            .as_ptr(),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiply<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmQuaternion>,
    mut qu1: * const crate::src::kazmath::mat3::kmQuaternion,
    mut qu2: * const crate::src::kazmath::mat3::kmQuaternion,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmQuaternion> where 'a1: 'a2 {
    let mut tmp1 = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut tmp2 = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionAssign(&mut tmp1, qu1);
    kmQuaternionAssign(&mut tmp2, qu2);
    let mut q1: Option<&'_ mut crate::src::kazmath::mat3::kmQuaternion> = Some(&mut tmp1);
    let mut q2: Option<&'_ mut crate::src::kazmath::mat3::kmQuaternion> = Some(&mut tmp2);
    (*(borrow_mut(&mut pOut)).unwrap())
        .x = (*(borrow(& q1)).unwrap()).w * (*(borrow(& q2)).unwrap()).x + (*(borrow(& q1)).unwrap()).x * (*(borrow(& q2)).unwrap()).w + (*(borrow(& q1)).unwrap()).y * (*(borrow(& q2)).unwrap()).z
        - (*(borrow(& q1)).unwrap()).z * (*(borrow(& q2)).unwrap()).y;
    (*(borrow_mut(&mut pOut)).unwrap())
        .y = (*(borrow(& q1)).unwrap()).w * (*(borrow(& q2)).unwrap()).y + (*(borrow(& q1)).unwrap()).y * (*(borrow(& q2)).unwrap()).w + (*(borrow(& q1)).unwrap()).z * (*(borrow(& q2)).unwrap()).x
        - (*(borrow(& q1)).unwrap()).x * (*(borrow(& q2)).unwrap()).z;
    (*(borrow_mut(&mut pOut)).unwrap())
        .z = (*(borrow(& q1)).unwrap()).w * (*(borrow(& q2)).unwrap()).z + (*(borrow(& q1)).unwrap()).z * (*(borrow(& q2)).unwrap()).w + (*(borrow(& q1)).unwrap()).x * (*(borrow(& q2)).unwrap()).y
        - (*(borrow(& q1)).unwrap()).y * (*(borrow(& q2)).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap())
        .w = (*(borrow(& q1)).unwrap()).w * (*(borrow(& q2)).unwrap()).w - (*(borrow(& q1)).unwrap()).x * (*(borrow(& q2)).unwrap()).x - (*(borrow(& q1)).unwrap()).y * (*(borrow(& q2)).unwrap()).y
        - (*(borrow(& q1)).unwrap()).z * (*(borrow(& q2)).unwrap()).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionNormalize(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: * const crate::src::kazmath::mat3::kmQuaternion,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    let mut length = kmQuaternionLength(pIn);
    if fabs(length as std::os::raw::c_double) < 0.0001f64 {
        (*pOut).x = 0.0f64 as std::os::raw::c_float;
        (*pOut).y = 0.0f64 as std::os::raw::c_float;
        (*pOut).z = 0.0f64 as std::os::raw::c_float;
        (*pOut).w = 0.0f64 as std::os::raw::c_float;
        return pOut;
    }
    kmQuaternionFill(
        pOut,
        (*pOut).x / length,
        (*pOut).y / length,
        (*pOut).z / length,
        (*pOut).w / length,
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationAxisAngle<'a1>(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut pV: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
    mut angle: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    let mut rad = angle * 0.5f32;
    let mut scale = sinf(rad);
    (*pOut).x = (*((pV).clone()).unwrap()).x * scale;
    (*pOut).y = (*((pV).clone()).unwrap()).y * scale;
    (*pOut).z = (*((pV).clone()).unwrap()).z * scale;
    (*pOut).w = cosf(rad);
    kmQuaternionNormalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationMatrix<'a1>(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat3>,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut z: f32 = 0.;
    let mut w: f32 = 0.;
    let mut pMatrix = (0 as * mut f32);
    let mut m4x4: [f32; 16] = [
        0 as std::os::raw::c_int as std::os::raw::c_float,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut scale = 0.0f32;
    let mut diagonal = 0.0f32;
    if (pIn).clone().is_none() {
        return (0 as * mut crate::src::kazmath::mat3::kmQuaternion);
    }
    m4x4[0 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[0 as std::os::raw::c_int as usize];
    m4x4[1 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[3 as std::os::raw::c_int as usize];
    m4x4[2 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[6 as std::os::raw::c_int as usize];
    m4x4[4 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[1 as std::os::raw::c_int as usize];
    m4x4[5 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[4 as std::os::raw::c_int as usize];
    m4x4[6 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[7 as std::os::raw::c_int as usize];
    m4x4[8 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[2 as std::os::raw::c_int as usize];
    m4x4[9 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[5 as std::os::raw::c_int as usize];
    m4x4[10 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[8 as std::os::raw::c_int as usize];
    m4x4[15 as std::os::raw::c_int as usize] = 1 as std::os::raw::c_int as std::os::raw::c_float;
    pMatrix = &mut *m4x4.as_mut_ptr().offset(0 as std::os::raw::c_int as isize)
        as *mut std::os::raw::c_float;
    diagonal = *pMatrix.offset(0 as std::os::raw::c_int as isize)
        + *pMatrix.offset(5 as std::os::raw::c_int as isize)
        + *pMatrix.offset(10 as std::os::raw::c_int as isize)
        + 1 as std::os::raw::c_int as std::os::raw::c_float;
    if diagonal as std::os::raw::c_double > 0.0001f64 {
        scale = sqrt(diagonal as std::os::raw::c_double) as std::os::raw::c_float
            * 2 as std::os::raw::c_int as std::os::raw::c_float;
        x = (*pMatrix.offset(9 as std::os::raw::c_int as isize)
            - *pMatrix.offset(6 as std::os::raw::c_int as isize)) / scale;
        y = (*pMatrix.offset(2 as std::os::raw::c_int as isize)
            - *pMatrix.offset(8 as std::os::raw::c_int as isize)) / scale;
        z = (*pMatrix.offset(4 as std::os::raw::c_int as isize)
            - *pMatrix.offset(1 as std::os::raw::c_int as isize)) / scale;
        w = 0.25f32 * scale;
    } else if *pMatrix.offset(0 as std::os::raw::c_int as isize)
        > *pMatrix.offset(5 as std::os::raw::c_int as isize)
        && *pMatrix.offset(0 as std::os::raw::c_int as isize)
            > *pMatrix.offset(10 as std::os::raw::c_int as isize)
    {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(0 as std::os::raw::c_int as isize)
                - *pMatrix.offset(5 as std::os::raw::c_int as isize)
                - *pMatrix.offset(10 as std::os::raw::c_int as isize)) as std::os::raw::c_double,
        ) as std::os::raw::c_float * 2.0f32;
        x = 0.25f32 * scale;
        y = (*pMatrix.offset(4 as std::os::raw::c_int as isize)
            + *pMatrix.offset(1 as std::os::raw::c_int as isize)) / scale;
        z = (*pMatrix.offset(2 as std::os::raw::c_int as isize)
            + *pMatrix.offset(8 as std::os::raw::c_int as isize)) / scale;
        w = (*pMatrix.offset(9 as std::os::raw::c_int as isize)
            - *pMatrix.offset(6 as std::os::raw::c_int as isize)) / scale;
    } else if *pMatrix.offset(5 as std::os::raw::c_int as isize)
        > *pMatrix.offset(10 as std::os::raw::c_int as isize)
    {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(5 as std::os::raw::c_int as isize)
                - *pMatrix.offset(0 as std::os::raw::c_int as isize)
                - *pMatrix.offset(10 as std::os::raw::c_int as isize)) as std::os::raw::c_double,
        ) as std::os::raw::c_float * 2.0f32;
        x = (*pMatrix.offset(4 as std::os::raw::c_int as isize)
            + *pMatrix.offset(1 as std::os::raw::c_int as isize)) / scale;
        y = 0.25f32 * scale;
        z = (*pMatrix.offset(9 as std::os::raw::c_int as isize)
            + *pMatrix.offset(6 as std::os::raw::c_int as isize)) / scale;
        w = (*pMatrix.offset(2 as std::os::raw::c_int as isize)
            - *pMatrix.offset(8 as std::os::raw::c_int as isize)) / scale;
    } else {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(10 as std::os::raw::c_int as isize)
                - *pMatrix.offset(0 as std::os::raw::c_int as isize)
                - *pMatrix.offset(5 as std::os::raw::c_int as isize)) as std::os::raw::c_double,
        ) as std::os::raw::c_float * 2.0f32;
        x = (*pMatrix.offset(2 as std::os::raw::c_int as isize)
            + *pMatrix.offset(8 as std::os::raw::c_int as isize)) / scale;
        y = (*pMatrix.offset(9 as std::os::raw::c_int as isize)
            + *pMatrix.offset(6 as std::os::raw::c_int as isize)) / scale;
        z = 0.25f32 * scale;
        w = (*pMatrix.offset(4 as std::os::raw::c_int as isize)
            - *pMatrix.offset(1 as std::os::raw::c_int as isize)) / scale;
    }
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationPitchYawRoll<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmQuaternion>,
    mut pitch: std::os::raw::c_float,
    mut yaw: std::os::raw::c_float,
    mut roll: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmQuaternion> where 'a1: 'a2 {
    if pitch <= 2 as std::os::raw::c_int as std::os::raw::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"pitch <= 2*kmPI\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const std::os::raw::c_char,
            334 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 84], &'_ [i8; 84]>(
                b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",
            ))
                .as_ptr(),
        );
    }
    if yaw <= 2 as std::os::raw::c_int as std::os::raw::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"yaw <= 2*kmPI\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const std::os::raw::c_char,
            335 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 84], &'_ [i8; 84]>(
                b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",
            ))
                .as_ptr(),
        );
    }
    if roll <= 2 as std::os::raw::c_int as std::os::raw::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"roll <= 2*kmPI\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const std::os::raw::c_char,
            336 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 84], &'_ [i8; 84]>(
                b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",
            ))
                .as_ptr(),
        );
    }
    let mut sY = sinf((yaw as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float);
    let mut cY = cosf((yaw as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float);
    let mut sZ = sinf((roll as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float);
    let mut cZ = cosf((roll as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float);
    let mut sX = sinf((pitch as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float);
    let mut cX = cosf((pitch as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float);
    (*(borrow_mut(&mut pOut)).unwrap()).w = cY * cZ * cX - sY * sZ * sX;
    (*(borrow_mut(&mut pOut)).unwrap()).x = sY * sZ * cX + cY * cZ * sX;
    (*(borrow_mut(&mut pOut)).unwrap()).y = sY * cZ * cX + cY * sZ * sX;
    (*(borrow_mut(&mut pOut)).unwrap()).z = cY * sZ * cX - sY * cZ * sX;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionSlerp<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut q1: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut q2: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
    mut t: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    let mut dot = kmQuaternionDot((q1).clone(), (q2).clone());
    let DOT_THRESHOLD = 0.9995f64;
    if dot as std::os::raw::c_double > DOT_THRESHOLD {
        let mut diff = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionSubtract(&mut diff, (q2).clone(), (q1).clone());
        kmQuaternionScale(&mut diff, Some(&mut diff), t);
        kmQuaternionAdd(pOut, (q1).clone(), Some(&mut diff));
        kmQuaternionNormalize(pOut, pOut);
        return pOut;
    }
    dot = kmClamp(
        dot,
        -(1 as std::os::raw::c_int) as std::os::raw::c_float,
        1 as std::os::raw::c_int as std::os::raw::c_float,
    );
    let mut theta_0 = acos(dot as std::os::raw::c_double) as std::os::raw::c_float;
    let mut theta = theta_0 * t;
    let mut tmp = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionScale(&mut tmp, (q1).clone(), dot);
    kmQuaternionSubtract(&mut tmp, (q2).clone(), Some(&mut tmp));
    kmQuaternionNormalize(&mut tmp, &mut tmp);
    let mut t1 = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut t2 = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionScale(&mut t1, (q1).clone(), cos(theta as std::os::raw::c_double) as std::os::raw::c_float);
    kmQuaternionScale(&mut t2, Some(&mut tmp), sin(theta as std::os::raw::c_double) as std::os::raw::c_float);
    kmQuaternionAdd(pOut, Some(&mut t1), Some(&mut t2));
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionToAxisAngle<'a1, 'a2>(
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut pAxis: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pAngle: Option<&'a2 mut std::os::raw::c_float>,
) {
    let mut tempAngle: f32 = 0.;
    let mut scale: f32 = 0.;
    tempAngle = acosf((*(pIn).unwrap()).w);
    scale = sqrtf(kmSQR((*(pIn).unwrap()).x) + kmSQR((*(pIn).unwrap()).y) + kmSQR((*(pIn).unwrap()).z));
    if scale as std::os::raw::c_double > -0.0001f64 && (scale as std::os::raw::c_double) < 0.0001f64
        || (scale as std::os::raw::c_double)
            < (2 as std::os::raw::c_int as std::os::raw::c_float * 3.14159265358979323846f32)
                as std::os::raw::c_double + 0.0001f64
            && scale as std::os::raw::c_double
                > (2 as std::os::raw::c_int as std::os::raw::c_float * 3.14159265358979323846f32)
                    as std::os::raw::c_double - 0.0001f64
    {
        *(borrow_mut(&mut pAngle)).unwrap() = 0.0f32;
        (*pAxis).x = 0.0f32;
        (*pAxis).y = 0.0f32;
        (*pAxis).z = 1.0f32;
    } else {
        *(borrow_mut(&mut pAngle)).unwrap() = tempAngle * 2.0f32;
        (*pAxis).x = (*((pIn).clone()).unwrap()).x / scale;
        (*pAxis).y = (*((pIn).clone()).unwrap()).y / scale;
        (*pAxis).z = (*((pIn).clone()).unwrap()).z / scale;
        kmVec3Normalize(pAxis, pAxis);
    };
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionScale<'a1>(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut s: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x = (*((pIn).clone()).unwrap()).x * s;
    (*pOut).y = (*((pIn).clone()).unwrap()).y * s;
    (*pOut).z = (*((pIn).clone()).unwrap()).z * s;
    (*pOut).w = (*((pIn).clone()).unwrap()).w * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAssign(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: * const crate::src::kazmath::mat3::kmQuaternion,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    memcpy(
        pOut as *mut std::os::raw::c_void,
        pIn as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionSubtract<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut pQ1: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut pQ2: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x = (*((pQ1).clone()).unwrap()).x - (*((pQ2).clone()).unwrap()).x;
    (*pOut).y = (*((pQ1).clone()).unwrap()).y - (*((pQ2).clone()).unwrap()).y;
    (*pOut).z = (*((pQ1).clone()).unwrap()).z - (*((pQ2).clone()).unwrap()).z;
    (*pOut).w = (*((pQ1).clone()).unwrap()).w - (*((pQ2).clone()).unwrap()).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAdd<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut pQ1: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut pQ2: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x = (*((pQ1).clone()).unwrap()).x + (*((pQ2).clone()).unwrap()).x;
    (*pOut).y = (*((pQ1).clone()).unwrap()).y + (*((pQ2).clone()).unwrap()).y;
    (*pOut).z = (*((pQ1).clone()).unwrap()).z + (*((pQ2).clone()).unwrap()).z;
    (*pOut).w = (*((pQ1).clone()).unwrap()).w + (*((pQ2).clone()).unwrap()).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationBetweenVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut vec1: * const crate::src::kazmath::aabb3::kmVec3,
    mut vec2: * const crate::src::kazmath::aabb3::kmVec3,
    mut fallback: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    let mut v1 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut v2 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut a: f32 = 0.;
    kmVec3Assign(&mut v1, vec1);
    kmVec3Assign(&mut v2, vec2);
    kmVec3Normalize(&mut v1, &mut v1);
    kmVec3Normalize(&mut v2, &mut v2);
    a = kmVec3Dot(&mut v1, &mut v2);
    if a as std::os::raw::c_double >= 1.0f64 {
        kmQuaternionIdentity(pOut);
        return pOut;
    }
    if a < 1e-6f32 - 1.0f32 {
        if fabs(kmVec3LengthSq((fallback).clone()) as std::os::raw::c_double) < 0.0001f64 {
            kmQuaternionRotationAxisAngle(pOut, (fallback).clone(), 3.14159265358979323846f32);
        } else {
            let mut axis = kmVec3 { x: 0., y: 0., z: 0. };
            let mut X = kmVec3 { x: 0., y: 0., z: 0. };
            X.x = 1.0f64 as std::os::raw::c_float;
            X.y = 0.0f64 as std::os::raw::c_float;
            X.z = 0.0f64 as std::os::raw::c_float;
            kmVec3Cross(&mut axis, Some(&mut X), vec1);
            if fabs(kmVec3LengthSq(Some(&mut axis)) as std::os::raw::c_double) < 0.0001f64 {
                let mut Y = kmVec3 { x: 0., y: 0., z: 0. };
                Y.x = 0.0f64 as std::os::raw::c_float;
                Y.y = 1.0f64 as std::os::raw::c_float;
                Y.z = 0.0f64 as std::os::raw::c_float;
                kmVec3Cross(&mut axis, Some(&mut Y), vec1);
            }
            kmVec3Normalize(&mut axis, &mut axis);
            kmQuaternionRotationAxisAngle(pOut, Some(&mut axis), 3.14159265358979323846f32);
        }
    } else {
        let mut s = sqrtf(
            (1 as std::os::raw::c_int as std::os::raw::c_float + a) * 2 as std::os::raw::c_int as std::os::raw::c_float,
        );
        let mut invs = 1 as std::os::raw::c_int as std::os::raw::c_float / s;
        let mut c = kmVec3 { x: 0., y: 0., z: 0. };
        kmVec3Cross(&mut c, Some(&mut v1), &mut v2);
        (*pOut).x = c.x * invs;
        (*pOut).y = c.y * invs;
        (*pOut).z = c.z * invs;
        (*pOut).w = s * 0.5f32;
        kmQuaternionNormalize(pOut, pOut);
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiplyVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut q: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
    mut v: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut uv = kmVec3 { x: 0., y: 0., z: 0. };
    let mut uuv = kmVec3 { x: 0., y: 0., z: 0. };
    let mut qvec = kmVec3 { x: 0., y: 0., z: 0. };
    qvec.x = (*(q).unwrap()).x;
    qvec.y = (*(q).unwrap()).y;
    qvec.z = (*(q).unwrap()).z;
    kmVec3Cross(&mut uv, Some(&mut qvec), v);
    kmVec3Cross(&mut uuv, Some(&mut qvec), &mut uv);
    kmVec3Scale(&mut uv, &mut uv, 2.0f32 * (*((q).clone()).unwrap()).w);
    kmVec3Scale(&mut uuv, &mut uuv, 2.0f32);
    kmVec3Add(pOut, v, Some(&mut uv));
    kmVec3Add(pOut, pOut, Some(&mut uuv));
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetUpVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, (pIn).clone(), &KM_VEC3_POS_Y);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRightVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, (pIn).clone(), &KM_VEC3_POS_X);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3RH<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, (pIn).clone(), &KM_VEC3_NEG_Z);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3LH<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, (pIn).clone(), &KM_VEC3_POS_Z);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetPitch<'a1>(
    mut q: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> std::os::raw::c_float {
    let mut result = atan2(
        (2 as std::os::raw::c_int as std::os::raw::c_float * ((*((q).clone()).unwrap()).y * (*((q).clone()).unwrap()).z + (*((q).clone()).unwrap()).w * (*((q).clone()).unwrap()).x))
            as std::os::raw::c_double,
        ((*((q).clone()).unwrap()).w * (*((q).clone()).unwrap()).w - (*((q).clone()).unwrap()).x * (*((q).clone()).unwrap()).x - (*((q).clone()).unwrap()).y * (*((q).clone()).unwrap()).y + (*((q).clone()).unwrap()).z * (*((q).clone()).unwrap()).z)
            as std::os::raw::c_double,
    ) as std::os::raw::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetYaw<'a1>(
    mut q: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> std::os::raw::c_float {
    let mut result = asin(
        (-(2 as std::os::raw::c_int) as std::os::raw::c_float * ((*((q).clone()).unwrap()).x * (*((q).clone()).unwrap()).z - (*((q).clone()).unwrap()).w * (*((q).clone()).unwrap()).y))
            as std::os::raw::c_double,
    ) as std::os::raw::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRoll<'a1>(
    mut q: Option<&'a1 crate::src::kazmath::mat3::kmQuaternion>,
) -> std::os::raw::c_float {
    let mut result = atan2(
        (2 as std::os::raw::c_int as std::os::raw::c_float * ((*((q).clone()).unwrap()).x * (*((q).clone()).unwrap()).y + (*((q).clone()).unwrap()).w * (*((q).clone()).unwrap()).z))
            as std::os::raw::c_double,
        ((*((q).clone()).unwrap()).w * (*((q).clone()).unwrap()).w + (*((q).clone()).unwrap()).x * (*((q).clone()).unwrap()).x - (*((q).clone()).unwrap()).y * (*((q).clone()).unwrap()).y - (*((q).clone()).unwrap()).z * (*((q).clone()).unwrap()).z)
            as std::os::raw::c_double,
    ) as std::os::raw::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLookRotation(
    mut pOut: * mut crate::src::kazmath::mat3::kmQuaternion,
    mut direction: * const crate::src::kazmath::aabb3::kmVec3,
    mut up: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::mat3::kmQuaternion {
    let mut tmp = kmMat3 { mat: [0.; 9] };
    borrow(& kmMat3LookAt(Some(&mut tmp), &KM_VEC3_ZERO, direction, up));
    return kmQuaternionNormalize(pOut, kmQuaternionRotationMatrix(pOut, Some(&mut tmp)));
}
use crate::laertes_rt::*;
