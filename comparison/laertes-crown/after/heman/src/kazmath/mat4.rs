
extern "C" {
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn memset(
        _: * mut core::ffi::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn memcmp(
        _: * const core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn cosf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sinf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::kazmath::quaternion::kmQuaternionRotationAxisAngle;
pub use crate::src::kazmath::quaternion::kmQuaternionRotationMatrix;
pub use crate::src::kazmath::quaternion::kmQuaternionToAxisAngle;
pub use crate::src::kazmath::utility::kmDegreesToRadians;
pub use crate::src::kazmath::vec3::kmVec3Assign;
pub use crate::src::kazmath::vec3::kmVec3Cross;
pub use crate::src::kazmath::vec3::kmVec3MultiplyMat4;
pub use crate::src::kazmath::vec3::kmVec3Normalize;
pub use crate::src::kazmath::vec3::kmVec3Subtract;
pub use crate::src::kazmath::vec3::KM_VEC3_NEG_Z;
pub use crate::src::kazmath::vec3::KM_VEC3_POS_X;
pub use crate::src::kazmath::vec3::KM_VEC3_POS_Y;
pub use crate::src::kazmath::vec3::KM_VEC3_POS_Z;
// #[derive(Copy, Clone)]

pub type kmMat4 = crate::src::kazmath::mat3::kmMat4;
// #[derive(Copy, Clone)]

pub type kmMat3 = crate::src::kazmath::mat3::kmMat3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmPlane {
    pub a: std::os::raw::c_float,
    pub b: std::os::raw::c_float,
    pub c: std::os::raw::c_float,
    pub d: std::os::raw::c_float,
}
impl kmPlane {
    pub const fn new() -> Self {
        kmPlane {
        a: 0.0,
        b: 0.0,
        c: 0.0,
        d: 0.0
        }
    }
}

impl std::default::Default for kmPlane {
    fn default() -> Self { kmPlane::new() }
}

// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
// #[derive(Copy, Clone)]

pub type kmQuaternion = crate::src::kazmath::mat3::kmQuaternion;
#[no_mangle]
pub unsafe extern "C" fn kmMat4Fill<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    mut pMat: * const std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a2 {
    memcpy(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        pMat as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Identity(mut pOut: * mut crate::src::kazmath::mat3::kmMat4) -> * mut crate::src::kazmath::mat3::kmMat4 {
    memset(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    let ref mut fresh0 = (*pOut).mat[15 as std::os::raw::c_int as usize];
    *fresh0 = 1.0f32;
    let ref mut fresh1 = (*pOut).mat[10 as std::os::raw::c_int as usize];
    *fresh1 = *fresh0;
    let ref mut fresh2 = (*pOut).mat[5 as std::os::raw::c_int as usize];
    *fresh2 = *fresh1;
    (*pOut).mat[0 as std::os::raw::c_int as usize] = *fresh2;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Inverse<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    mut pM: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a3 {
    let mut tmp = kmMat4 { mat: [0.; 16] };
    let mut det: f64 = 0.;
    let mut i: i32 = 0;
    tmp
        .mat[0 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[4 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[8 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[12 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[1 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[5 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
    tmp
        .mat[9 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[13 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize];
    tmp
        .mat[2 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[6 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[10 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize];
    tmp
        .mat[14 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize];
    tmp
        .mat[3 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[7 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    tmp
        .mat[11 as std::os::raw::c_int
        as usize] = -(*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize];
    tmp
        .mat[15 as std::os::raw::c_int
        as usize] = (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        - (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize] * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
            * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize];
    det = ((*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * tmp.mat[0 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize] * tmp.mat[4 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize] * tmp.mat[8 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize] * tmp.mat[12 as std::os::raw::c_int as usize])
        as std::os::raw::c_double;
    if det == 0 as std::os::raw::c_int as std::os::raw::c_double {
        return Option::<&'_ mut crate::src::kazmath::mat3::kmMat4>::None;
    }
    det = 1.0f64 / det;
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        (*(borrow_mut(&mut pOut)).unwrap())
            .mat[i
            as usize] = (tmp.mat[i as usize] as std::os::raw::c_double * det) as std::os::raw::c_float;
        i += 1;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4IsIdentity<'a1>(mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat4>) -> std::os::raw::c_int {
    static mut identity: [std::os::raw::c_float; 16] = [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]; unsafe fn laertes_init_identity() {
identity = [
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
    ];}//;
    return (memcmp(
        identity.as_mut_ptr() as *const std::os::raw::c_void,
        ((*((pIn).clone()).unwrap()).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Transpose<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a3 {
    let mut x: i32 = 0;
    let mut z: i32 = 0;
    z = 0 as std::os::raw::c_int;
    while z < 4 as std::os::raw::c_int {
        x = 0 as std::os::raw::c_int;
        while x < 4 as std::os::raw::c_int {
            (*(borrow_mut(&mut pOut)).unwrap())
                .mat[(z * 4 as std::os::raw::c_int + x)
                as usize] = (*(pIn).unwrap()).mat[(x * 4 as std::os::raw::c_int + z) as usize];
            x += 1;
        }
        z += 1;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Multiply(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat4,
    mut pM1: * const crate::src::kazmath::mat3::kmMat4,
    mut pM2: * const crate::src::kazmath::mat3::kmMat4,
) -> * mut crate::src::kazmath::mat3::kmMat4 {
    let mut mat: [f32; 16] = [0.; 16];
    let mut m1 = ((*pM1).mat).as_ptr();
    let mut m2 = ((*pM2).mat).as_ptr();
    mat[0 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[1 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[2 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[3 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize) * *m2.offset(3 as std::os::raw::c_int as isize);
    mat[4 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[5 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[6 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[7 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize);
    mat[8 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[9 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[10 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[11 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(8 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(9 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(10 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize)
            * *m2.offset(11 as std::os::raw::c_int as isize);
    mat[12 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(12 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    mat[13 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(9 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(13 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    mat[14 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(10 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(14 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    mat[15 as std::os::raw::c_int
        as usize] = *m1.offset(3 as std::os::raw::c_int as isize)
        * *m2.offset(12 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(13 as std::os::raw::c_int as isize)
        + *m1.offset(11 as std::os::raw::c_int as isize) * *m2.offset(14 as std::os::raw::c_int as isize)
        + *m1.offset(15 as std::os::raw::c_int as isize)
            * *m2.offset(15 as std::os::raw::c_int as isize);
    memcpy(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        mat.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Assign<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a3 {
    if _ref_ne(borrow(& pOut) ,  (pIn).clone())
        && !(b"You have tried to self-assign!!\0" as *const u8 as *const std::os::raw::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pOut != pIn && \"You have tried to self-assign!!\"\0" as *const u8
                as *const std::os::raw::c_char,
            b"../kazmath/mat4.c\0" as *const u8 as *const std::os::raw::c_char,
            272 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 47], &'_ [i8; 47]>(b"kmMat4 *kmMat4Assign(kmMat4 *, const kmMat4 *)\0"))
                .as_ptr(),
        );
    }
    memcpy(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        ((*((pIn).clone()).unwrap()).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4AssignMat3<'a1>(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat4,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat3>,
) -> * mut crate::src::kazmath::mat3::kmMat4 {
    kmMat4Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[0 as std::os::raw::c_int as usize];
    (*pOut).mat[1 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[1 as std::os::raw::c_int as usize];
    (*pOut).mat[2 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[2 as std::os::raw::c_int as usize];
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[3 as std::os::raw::c_int as usize];
    (*pOut).mat[5 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[4 as std::os::raw::c_int as usize];
    (*pOut).mat[6 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[5 as std::os::raw::c_int as usize];
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[6 as std::os::raw::c_int as usize];
    (*pOut).mat[9 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[7 as std::os::raw::c_int as usize];
    (*pOut).mat[10 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[8 as std::os::raw::c_int as usize];
    (*pOut).mat[11 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4AreEqual<'a1, 'a2>(
    mut pMat1: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
    mut pMat2: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> std::os::raw::c_int {
    let mut i = 0 as std::os::raw::c_int;
    if _ref_ne((pMat1).clone() ,  (pMat2).clone())
        && !(b"You are comparing the same thing!\0" as *const u8 as *const std::os::raw::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"pMat1 != pMat2 && \"You are comparing the same thing!\"\0" as *const u8
                as *const std::os::raw::c_char,
            b"../kazmath/mat4.c\0" as *const u8 as *const std::os::raw::c_char,
            308 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 51], &'_ [i8; 51]>(b"int kmMat4AreEqual(const kmMat4 *, const kmMat4 *)\0"))
                .as_ptr(),
        );
    }
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        if !((*((pMat1).clone()).unwrap()).mat[i as usize] as std::os::raw::c_double + 0.0001f64
            > (*((pMat2).clone()).unwrap()).mat[i as usize] as std::os::raw::c_double
            && (*((pMat1).clone()).unwrap()).mat[i as usize] as std::os::raw::c_double - 0.0001f64
                < (*((pMat2).clone()).unwrap()).mat[i as usize] as std::os::raw::c_double)
        {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationAxisAngle<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    mut axis: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut radians: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a3 {
    let mut quat = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionRotationAxisAngle(&mut quat, (axis).clone(), radians);
    borrow(& kmMat4RotationQuaternion(borrow_mut(&mut pOut), Some(&mut quat)));
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationX<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    radians: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[9 as std::os::raw::c_int as usize] = -sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[10 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[12 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[13 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[14 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationY<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    radians: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = -sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[9 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[10 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[12 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[13 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[14 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationZ<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    radians: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = -sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[9 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[10 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[12 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[13 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[14 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationYawPitchRoll(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat4,
    pitch: std::os::raw::c_float,
    yaw: std::os::raw::c_float,
    roll: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmMat4 {
    let mut yaw_matrix = kmMat4 { mat: [0.; 16] };
    borrow(& kmMat4RotationY(Some(&mut yaw_matrix), yaw));
    let mut pitch_matrix = kmMat4 { mat: [0.; 16] };
    borrow(& kmMat4RotationX(Some(&mut pitch_matrix), pitch));
    let mut roll_matrix = kmMat4 { mat: [0.; 16] };
    borrow(& kmMat4RotationZ(Some(&mut roll_matrix), roll));
    kmMat4Multiply(pOut, &mut pitch_matrix, &mut roll_matrix);
    kmMat4Multiply(pOut, &mut yaw_matrix, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationQuaternion<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    mut pQ: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a3 {
    let mut xx = ((*((pQ).clone()).unwrap()).x * (*((pQ).clone()).unwrap()).x) as std::os::raw::c_double;
    let mut xy = ((*((pQ).clone()).unwrap()).x * (*((pQ).clone()).unwrap()).y) as std::os::raw::c_double;
    let mut xz = ((*((pQ).clone()).unwrap()).x * (*((pQ).clone()).unwrap()).z) as std::os::raw::c_double;
    let mut xw = ((*((pQ).clone()).unwrap()).x * (*((pQ).clone()).unwrap()).w) as std::os::raw::c_double;
    let mut yy = ((*((pQ).clone()).unwrap()).y * (*((pQ).clone()).unwrap()).y) as std::os::raw::c_double;
    let mut yz = ((*((pQ).clone()).unwrap()).y * (*((pQ).clone()).unwrap()).z) as std::os::raw::c_double;
    let mut yw = ((*((pQ).clone()).unwrap()).y * (*((pQ).clone()).unwrap()).w) as std::os::raw::c_double;
    let mut zz = ((*((pQ).clone()).unwrap()).z * (*((pQ).clone()).unwrap()).z) as std::os::raw::c_double;
    let mut zw = ((*((pQ).clone()).unwrap()).z * (*((pQ).clone()).unwrap()).w) as std::os::raw::c_double;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[0 as std::os::raw::c_int
        as usize] = (1 as std::os::raw::c_int as std::os::raw::c_double
        - 2 as std::os::raw::c_int as std::os::raw::c_double * (yy + zz)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[1 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xy + zw)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[2 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xz - yw)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[4 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xy - zw)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[5 as std::os::raw::c_int
        as usize] = (1 as std::os::raw::c_int as std::os::raw::c_double
        - 2 as std::os::raw::c_int as std::os::raw::c_double * (xx + zz)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[6 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (yz + xw)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[8 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (xz + yw)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[9 as std::os::raw::c_int
        as usize] = (2 as std::os::raw::c_int as std::os::raw::c_double * (yz - xw)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[10 as std::os::raw::c_int
        as usize] = (1 as std::os::raw::c_int as std::os::raw::c_double
        - 2 as std::os::raw::c_int as std::os::raw::c_double * (xx + yy)) as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[11 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[12 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[13 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[14 as std::os::raw::c_int as usize] = 0.0f64 as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[15 as std::os::raw::c_int as usize] = 1.0f64 as std::os::raw::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Scaling<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    x: std::os::raw::c_float,
    y: std::os::raw::c_float,
    mut z: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a2 {
    memset(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = x;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = y;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[10 as std::os::raw::c_int as usize] = z;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4Translation<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
    z: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a2 {
    memset(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(16 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[10 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[12 as std::os::raw::c_int as usize] = x;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[13 as std::os::raw::c_int as usize] = y;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[14 as std::os::raw::c_int as usize] = z;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetUpVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    kmVec3MultiplyMat4(pOut, (Some(&KM_VEC3_POS_Y)).clone(), (pIn).clone());
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetRightVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    kmVec3MultiplyMat4(pOut, (Some(&KM_VEC3_POS_X)).clone(), (pIn).clone());
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetForwardVec3RH<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    kmVec3MultiplyMat4(pOut, (Some(&KM_VEC3_NEG_Z)).clone(), (pIn).clone());
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4GetForwardVec3LH<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    kmVec3MultiplyMat4(pOut, (Some(&KM_VEC3_POS_Z)).clone(), (pIn).clone());
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4PerspectiveProjection(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat4,
    mut fovY: std::os::raw::c_float,
    mut aspect: std::os::raw::c_float,
    mut zNear: std::os::raw::c_float,
    mut zFar: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmMat4 {
    let mut r = kmDegreesToRadians(fovY / 2 as std::os::raw::c_int as std::os::raw::c_float);
    let mut deltaZ = zFar - zNear;
    let mut s = sin(r as std::os::raw::c_double) as std::os::raw::c_float;
    let mut cotangent = 0 as std::os::raw::c_int as std::os::raw::c_float;
    if deltaZ == 0 as std::os::raw::c_int as std::os::raw::c_float
        || s == 0 as std::os::raw::c_int as std::os::raw::c_float
        || aspect == 0 as std::os::raw::c_int as std::os::raw::c_float
    {
        return (0 as * mut crate::src::kazmath::mat3::kmMat4);
    }
    cotangent = (cos(r as std::os::raw::c_double) / s as std::os::raw::c_double) as std::os::raw::c_float;
    kmMat4Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = cotangent / aspect;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = cotangent;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = -(zFar + zNear) / deltaZ;
    (*pOut).mat[11 as std::os::raw::c_int as usize] = -(1 as std::os::raw::c_int) as std::os::raw::c_float;
    (*pOut)
        .mat[14 as std::os::raw::c_int
        as usize] = -(2 as std::os::raw::c_int) as std::os::raw::c_float * zNear * zFar / deltaZ;
    (*pOut).mat[15 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as std::os::raw::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4OrthographicProjection(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat4,
    mut left: std::os::raw::c_float,
    mut right: std::os::raw::c_float,
    mut bottom: std::os::raw::c_float,
    mut top: std::os::raw::c_float,
    mut nearVal: std::os::raw::c_float,
    mut farVal: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmMat4 {
    let mut tx = -((right + left) / (right - left));
    let mut ty = -((top + bottom) / (top - bottom));
    let mut tz = -((farVal + nearVal) / (farVal - nearVal));
    kmMat4Identity(pOut);
    (*pOut)
        .mat[0 as std::os::raw::c_int
        as usize] = 2 as std::os::raw::c_int as std::os::raw::c_float / (right - left);
    (*pOut)
        .mat[5 as std::os::raw::c_int
        as usize] = 2 as std::os::raw::c_int as std::os::raw::c_float / (top - bottom);
    (*pOut)
        .mat[10 as std::os::raw::c_int
        as usize] = -(2 as std::os::raw::c_int) as std::os::raw::c_float / (farVal - nearVal);
    (*pOut).mat[12 as std::os::raw::c_int as usize] = tx;
    (*pOut).mat[13 as std::os::raw::c_int as usize] = ty;
    (*pOut).mat[14 as std::os::raw::c_int as usize] = tz;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4LookAt(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat4,
    mut pEye: * const crate::src::kazmath::aabb3::kmVec3,
    mut pCenter: * const crate::src::kazmath::aabb3::kmVec3,
    mut pUp: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::mat3::kmMat4 {
    let mut f = kmVec3 { x: 0., y: 0., z: 0. };
    let mut up = kmVec3 { x: 0., y: 0., z: 0. };
    let mut s = kmVec3 { x: 0., y: 0., z: 0. };
    let mut u = kmVec3 { x: 0., y: 0., z: 0. };
    let mut translate = kmMat4 { mat: [0.; 16] };
    kmVec3Subtract(&mut f, pCenter, pEye);
    kmVec3Normalize(&mut f, &mut f);
    kmVec3Assign(&mut up, pUp);
    kmVec3Normalize(&mut up, &mut up);
    kmVec3Cross(&mut s, Some(&mut f), &mut up);
    kmVec3Normalize(&mut s, &mut s);
    kmVec3Cross(&mut u, Some(&mut s), &mut f);
    kmVec3Normalize(&mut s, &mut s);
    kmMat4Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = s.x;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = s.y;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = s.z;
    (*pOut).mat[1 as std::os::raw::c_int as usize] = u.x;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = u.y;
    (*pOut).mat[9 as std::os::raw::c_int as usize] = u.z;
    (*pOut).mat[2 as std::os::raw::c_int as usize] = -f.x;
    (*pOut).mat[6 as std::os::raw::c_int as usize] = -f.y;
    (*pOut).mat[10 as std::os::raw::c_int as usize] = -f.z;
    borrow(& kmMat4Translation(Some(&mut translate), -(*pEye).x, -(*pEye).y, -(*pEye).z));
    kmMat4Multiply(pOut, pOut, &mut translate);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4ExtractRotation<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[0 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[1 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[2 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[4 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[5 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[6 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[8 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[9 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[10 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationToAxisAngle<'a1, 'a2>(
    mut pAxis: * mut crate::src::kazmath::aabb3::kmVec3,
    mut radians: Option<&'a1 mut std::os::raw::c_float>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut temp = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut rotation = kmMat3 { mat: [0.; 9] };
    borrow(& kmMat4ExtractRotation(Some(&mut rotation), (pIn).clone()));
    kmQuaternionRotationMatrix(&mut temp, Some(&mut rotation));
    kmQuaternionToAxisAngle(Some(&mut temp), pAxis, borrow_mut(&mut radians));
    return pAxis;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4RotationTranslation<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat4>,
    mut rotation: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
    mut translation: Option<&'a3 crate::src::kazmath::aabb3::kmVec3>,
) -> Option<&'a4 mut crate::src::kazmath::mat3::kmMat4> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[0 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[1 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[2 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[3 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[4 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[5 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[6 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[9 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[7 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[10 as std::os::raw::c_int as usize] = (*(rotation).unwrap()).mat[8 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).mat[11 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[12 as std::os::raw::c_int as usize] = (*(translation).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[13 as std::os::raw::c_int as usize] = (*(translation).unwrap()).y;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[14 as std::os::raw::c_int as usize] = (*(translation).unwrap()).z;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[15 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat4ExtractPlane<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat4::kmPlane>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
    plane: std::os::raw::c_uint,
) -> Option<&'a3 mut crate::src::kazmath::mat4::kmPlane> where 'a1: 'a3 {
    let mut t = 1.0f32;
    match plane {
        1 => {
            (*(borrow_mut(&mut pOut)).unwrap())
                .a = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .b = (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .c = (*((pIn).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .d = (*((pIn).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize];
        }
        0 => {
            (*(borrow_mut(&mut pOut)).unwrap())
                .a = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .b = (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .c = (*((pIn).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .d = (*((pIn).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize];
        }
        2 => {
            (*(borrow_mut(&mut pOut)).unwrap())
                .a = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .b = (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .c = (*((pIn).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .d = (*((pIn).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize];
        }
        3 => {
            (*(borrow_mut(&mut pOut)).unwrap())
                .a = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .b = (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .c = (*((pIn).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .d = (*((pIn).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize];
        }
        5 => {
            (*(borrow_mut(&mut pOut)).unwrap())
                .a = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .b = (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .c = (*((pIn).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .d = (*((pIn).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
                - (*((pIn).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize];
        }
        4 => {
            (*(borrow_mut(&mut pOut)).unwrap())
                .a = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .b = (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .c = (*((pIn).clone()).unwrap()).mat[11 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
            (*(borrow_mut(&mut pOut)).unwrap())
                .d = (*((pIn).clone()).unwrap()).mat[15 as std::os::raw::c_int as usize]
                + (*((pIn).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize];
        }
        _ => {
            if 0 as std::os::raw::c_int != 0
                && !(b"Invalid plane index\0" as *const u8 as *const std::os::raw::c_char)
                    .is_null()
            {} else {
                __assert_fail(
                    b"0 && \"Invalid plane index\"\0" as *const u8
                        as *const std::os::raw::c_char,
                    b"../kazmath/mat4.c\0" as *const u8 as *const std::os::raw::c_char,
                    779 as std::os::raw::c_int as std::os::raw::c_uint,
                    (*core::intrinsics::transmute::<&'_ [u8; 82], &'_ [i8; 82]>(
                        b"struct kmPlane *kmMat4ExtractPlane(kmPlane *, const kmMat4 *, const unsigned int)\0",
                    ))
                        .as_ptr(),
                );
            }
        }
    }
    t = sqrtf((*(borrow(& pOut)).unwrap()).a * (*(borrow(& pOut)).unwrap()).a + (*(borrow(& pOut)).unwrap()).b * (*(borrow(& pOut)).unwrap()).b + (*(borrow(& pOut)).unwrap()).c * (*(borrow(& pOut)).unwrap()).c);
    (*(borrow_mut(&mut pOut)).unwrap()).a /= t;
    (*(borrow_mut(&mut pOut)).unwrap()).b /= t;
    (*(borrow_mut(&mut pOut)).unwrap()).c /= t;
    (*(borrow_mut(&mut pOut)).unwrap()).d /= t;
    return pOut;
}
use crate::laertes_rt::*;
