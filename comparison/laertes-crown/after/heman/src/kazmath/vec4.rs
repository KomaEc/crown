
extern "C" {
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    
}
pub use crate::src::kazmath::utility::kmSQR;
// #[derive(Copy, Clone)]

pub type kmMat4 = crate::src::kazmath::mat3::kmMat4;
// #[derive(Copy, Clone)]

pub type kmVec4 = crate::src::kazmath::plane::kmVec4;
#[no_mangle]
pub unsafe extern "C" fn kmVec4Fill<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::plane::kmVec4>,
    mut x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
    mut z: std::os::raw::c_float,
    mut w: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::plane::kmVec4> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = z;
    (*(borrow_mut(&mut pOut)).unwrap()).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Add<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::plane::kmVec4>,
    mut pV1: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
    mut pV2: Option<&'a3 crate::src::kazmath::plane::kmVec4>,
) -> Option<&'a4 mut crate::src::kazmath::plane::kmVec4> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x + (*((pV2).clone()).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y + (*((pV2).clone()).unwrap()).y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = (*((pV1).clone()).unwrap()).z + (*((pV2).clone()).unwrap()).z;
    (*(borrow_mut(&mut pOut)).unwrap()).w = (*((pV1).clone()).unwrap()).w + (*((pV2).clone()).unwrap()).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Dot<'a1, 'a2>(
    mut pV1: Option<&'a1 crate::src::kazmath::plane::kmVec4>,
    mut pV2: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
) -> std::os::raw::c_float {
    return (*((pV1).clone()).unwrap()).x * (*((pV2).clone()).unwrap()).x + (*((pV1).clone()).unwrap()).y * (*((pV2).clone()).unwrap()).y + (*((pV1).clone()).unwrap()).z * (*((pV2).clone()).unwrap()).z
        + (*((pV1).clone()).unwrap()).w * (*((pV2).clone()).unwrap()).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Length(mut pIn: * const crate::src::kazmath::plane::kmVec4) -> std::os::raw::c_float {
    return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z) + kmSQR((*pIn).w));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4LengthSq<'a1>(mut pIn: Option<&'a1 crate::src::kazmath::plane::kmVec4>) -> std::os::raw::c_float {
    return kmSQR((*(pIn).unwrap()).x) + kmSQR((*(pIn).unwrap()).y) + kmSQR((*(pIn).unwrap()).z) + kmSQR((*(pIn).unwrap()).w);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Lerp<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::plane::kmVec4>,
    mut pV1: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
    mut pV2: Option<&'a3 crate::src::kazmath::plane::kmVec4>,
    mut t: std::os::raw::c_float,
) -> Option<&'a4 mut crate::src::kazmath::plane::kmVec4> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x + t * ((*((pV2).clone()).unwrap()).x - (*((pV1).clone()).unwrap()).x);
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y + t * ((*((pV2).clone()).unwrap()).y - (*((pV1).clone()).unwrap()).y);
    (*(borrow_mut(&mut pOut)).unwrap()).z = (*((pV1).clone()).unwrap()).z + t * ((*((pV2).clone()).unwrap()).z - (*((pV1).clone()).unwrap()).z);
    (*(borrow_mut(&mut pOut)).unwrap()).w = (*((pV1).clone()).unwrap()).w + t * ((*((pV2).clone()).unwrap()).w - (*((pV1).clone()).unwrap()).w);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Normalize(
    mut pOut: * mut crate::src::kazmath::plane::kmVec4,
    mut pIn: * const crate::src::kazmath::plane::kmVec4,
) -> * mut crate::src::kazmath::plane::kmVec4 {
    if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. && (*pIn).w == 0. {
        return kmVec4Assign(pOut, pIn);
    }
    let mut l = 1.0f32 / kmVec4Length(pIn);
    (*pOut).x = (*pIn).x * l;
    (*pOut).y = (*pIn).y * l;
    (*pOut).z = (*pIn).z * l;
    (*pOut).w = (*pIn).w * l;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Scale(
    mut pOut: * mut crate::src::kazmath::plane::kmVec4,
    mut pIn: * const crate::src::kazmath::plane::kmVec4,
    s: std::os::raw::c_float,
) -> * mut crate::src::kazmath::plane::kmVec4 {
    kmVec4Normalize(pOut, pIn);
    (*pOut).x *= s;
    (*pOut).y *= s;
    (*pOut).z *= s;
    (*pOut).w *= s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Subtract<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::plane::kmVec4>,
    mut pV1: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
    mut pV2: Option<&'a3 crate::src::kazmath::plane::kmVec4>,
) -> Option<&'a4 mut crate::src::kazmath::plane::kmVec4> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x - (*((pV2).clone()).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y - (*((pV2).clone()).unwrap()).y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = (*((pV1).clone()).unwrap()).z - (*((pV2).clone()).unwrap()).z;
    (*(borrow_mut(&mut pOut)).unwrap()).w = (*((pV1).clone()).unwrap()).w - (*((pV2).clone()).unwrap()).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Mul<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::plane::kmVec4>,
    mut pV1: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
    mut pV2: Option<&'a3 crate::src::kazmath::plane::kmVec4>,
) -> Option<&'a4 mut crate::src::kazmath::plane::kmVec4> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x * (*((pV2).clone()).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y * (*((pV2).clone()).unwrap()).y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = (*((pV1).clone()).unwrap()).z * (*((pV2).clone()).unwrap()).z;
    (*(borrow_mut(&mut pOut)).unwrap()).w = (*((pV1).clone()).unwrap()).w * (*((pV2).clone()).unwrap()).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Div<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::plane::kmVec4>,
    mut pV1: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
    mut pV2: Option<&'a3 crate::src::kazmath::plane::kmVec4>,
) -> Option<&'a4 mut crate::src::kazmath::plane::kmVec4> where 'a1: 'a4 {
    if (*((pV2).clone()).unwrap()).x != 0. && (*((pV2).clone()).unwrap()).y != 0. && (*((pV2).clone()).unwrap()).z != 0. && (*((pV2).clone()).unwrap()).w != 0. {
        (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x / (*((pV2).clone()).unwrap()).x;
        (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y / (*((pV2).clone()).unwrap()).y;
        (*(borrow_mut(&mut pOut)).unwrap()).z = (*((pV1).clone()).unwrap()).z / (*((pV2).clone()).unwrap()).z;
        (*(borrow_mut(&mut pOut)).unwrap()).w = (*((pV1).clone()).unwrap()).w / (*((pV2).clone()).unwrap()).w;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4MultiplyMat4<'a1>(
    mut pOut: * mut crate::src::kazmath::plane::kmVec4,
    mut pV: * const crate::src::kazmath::plane::kmVec4,
    mut pM: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::plane::kmVec4 {
    (*pOut)
        .x = (*pV).x * (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        + (*pV).y * (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        + (*pV).z * (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize]
        + (*pV).w * (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize];
    (*pOut)
        .y = (*pV).x * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        + (*pV).y * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        + (*pV).z * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        + (*pV).w * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize];
    (*pOut)
        .z = (*pV).x * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
        + (*pV).y * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        + (*pV).z * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        + (*pV).w * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize];
    (*pOut)
        .w = (*pV).x * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
        + (*pV).y * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        + (*pV).z * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + (*pV).w * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Transform<'a1>(
    mut pOut: * mut crate::src::kazmath::plane::kmVec4,
    mut pV: * const crate::src::kazmath::plane::kmVec4,
    mut pM: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::plane::kmVec4 {
    return kmVec4MultiplyMat4(pOut, pV, (pM).clone());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4TransformArray<'a1>(
    mut pOut: * mut crate::src::kazmath::plane::kmVec4,
    mut outStride: std::os::raw::c_uint,
    mut pV: * const crate::src::kazmath::plane::kmVec4,
    mut vStride: std::os::raw::c_uint,
    mut pM: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
    mut count: std::os::raw::c_uint,
) -> * mut crate::src::kazmath::plane::kmVec4 {
    let mut i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < count {
        let mut in_0 = pV.offset(i.wrapping_mul(vStride) as isize);
        let mut out = pOut.offset(i.wrapping_mul(outStride) as isize);
        kmVec4Transform(out, in_0, (pM).clone());
        i = i.wrapping_add(1);
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4AreEqual<'a1, 'a2>(
    mut p1: Option<&'a1 crate::src::kazmath::plane::kmVec4>,
    mut p2: Option<&'a2 crate::src::kazmath::plane::kmVec4>,
) -> std::os::raw::c_int {
    return (((*((p1).clone()).unwrap()).x as std::os::raw::c_double) < (*((p2).clone()).unwrap()).x as std::os::raw::c_double + 0.0001f64
        && (*((p1).clone()).unwrap()).x as std::os::raw::c_double > (*((p2).clone()).unwrap()).x as std::os::raw::c_double - 0.0001f64
        && (((*((p1).clone()).unwrap()).y as std::os::raw::c_double) < (*((p2).clone()).unwrap()).y as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).y as std::os::raw::c_double > (*((p2).clone()).unwrap()).y as std::os::raw::c_double - 0.0001f64)
        && (((*((p1).clone()).unwrap()).z as std::os::raw::c_double) < (*((p2).clone()).unwrap()).z as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).z as std::os::raw::c_double > (*((p2).clone()).unwrap()).z as std::os::raw::c_double - 0.0001f64)
        && (((*((p1).clone()).unwrap()).w as std::os::raw::c_double) < (*((p2).clone()).unwrap()).w as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).w as std::os::raw::c_double > (*((p2).clone()).unwrap()).w as std::os::raw::c_double - 0.0001f64))
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec4Assign(
    mut pOut: * mut crate::src::kazmath::plane::kmVec4,
    mut pIn: * const crate::src::kazmath::plane::kmVec4,
) -> * mut crate::src::kazmath::plane::kmVec4 {
    if pOut != pIn as *mut kmVec4 {} else {
        __assert_fail(
            b"pOut != pIn\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/vec4.c\0" as *const u8 as *const std::os::raw::c_char,
            176 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 47], &'_ [i8; 47]>(b"kmVec4 *kmVec4Assign(kmVec4 *, const kmVec4 *)\0"))
                .as_ptr(),
        );
    }
    memcpy(
        pOut as *mut std::os::raw::c_void,
        pIn as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(4 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
use crate::laertes_rt::*;
