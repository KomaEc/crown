
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
    fn cosf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sinf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    
    
    
    
    
    
}
pub use crate::src::kazmath::quaternion::kmQuaternionRotationMatrix;
pub use crate::src::kazmath::quaternion::kmQuaternionToAxisAngle;
pub use crate::src::kazmath::vec3::kmVec3Assign;
pub use crate::src::kazmath::vec3::kmVec3Cross;
pub use crate::src::kazmath::vec3::kmVec3Normalize;
pub use crate::src::kazmath::vec3::kmVec3Subtract;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [std::os::raw::c_float; 16],
}
impl kmMat4 {
    pub const fn new() -> Self {
        kmMat4 {
        mat: [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]
        }
    }
}

impl std::default::Default for kmMat4 {
    fn default() -> Self { kmMat4::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [std::os::raw::c_float; 9],
}
impl kmMat3 {
    pub const fn new() -> Self {
        kmMat3 {
        mat: [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]
        }
    }
}

impl std::default::Default for kmMat3 {
    fn default() -> Self { kmMat3::new() }
}

// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmQuaternion {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
    pub w: std::os::raw::c_float,
}
impl kmQuaternion {
    pub const fn new() -> Self {
        kmQuaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0
        }
    }
}

impl std::default::Default for kmQuaternion {
    fn default() -> Self { kmQuaternion::new() }
}

#[no_mangle]
pub unsafe extern "C" fn kmMat3Fill<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pMat: * const std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a2 {
    memcpy(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        pMat as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Identity(mut pOut: * mut crate::src::kazmath::mat3::kmMat3) -> * mut crate::src::kazmath::mat3::kmMat3 {
    memset(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    let ref mut fresh0 = (*pOut).mat[8 as std::os::raw::c_int as usize];
    *fresh0 = 1.0f32;
    let ref mut fresh1 = (*pOut).mat[4 as std::os::raw::c_int as usize];
    *fresh1 = *fresh0;
    (*pOut).mat[0 as std::os::raw::c_int as usize] = *fresh1;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Determinant<'a1>(mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat3>) -> std::os::raw::c_float {
    let mut output: f32 = 0.;
    output = (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize]
        + (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
            * (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        + (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
            * (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize];
    output
        -= (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
            * (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
            + (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
                * (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
                * (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
            + (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
                * (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
                * (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize];
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Adjugate<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[0 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[1 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[2 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[3 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[4 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[5 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[6 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[7 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[8 as std::os::raw::c_int
        as usize] = (*((pIn).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        * (*((pIn).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        - (*((pIn).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize] * (*((pIn).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Inverse<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pM: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    let mut determinate = kmMat3Determinant((pM).clone());
    let mut detInv: f32 = 0.;
    let mut adjugate = kmMat3 { mat: [0.; 9] };
    if determinate as std::os::raw::c_double == 0.0f64 {
        return Option::<&'_ mut crate::src::kazmath::mat3::kmMat3>::None;
    }
    detInv = (1.0f64 / determinate as std::os::raw::c_double) as std::os::raw::c_float;
    borrow(& kmMat3Adjugate(Some(&mut adjugate), (pM).clone()));
    borrow(& kmMat3ScalarMultiply(borrow_mut(&mut pOut), Some(&mut adjugate), detInv));
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3IsIdentity<'a1>(mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat3>) -> std::os::raw::c_int {
    static mut identity: [std::os::raw::c_float; 9] = [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]; unsafe fn laertes_init_identity() {
identity = [
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
    ];}//;
    return (memcmp(
        identity.as_mut_ptr() as *const std::os::raw::c_void,
        ((*((pIn).clone()).unwrap()).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Transpose<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    let mut temp: [f32; 9] = [0.; 9];
    temp[0 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[0 as std::os::raw::c_int as usize];
    temp[1 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[3 as std::os::raw::c_int as usize];
    temp[2 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[6 as std::os::raw::c_int as usize];
    temp[3 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[1 as std::os::raw::c_int as usize];
    temp[4 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[4 as std::os::raw::c_int as usize];
    temp[5 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[7 as std::os::raw::c_int as usize];
    temp[6 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[2 as std::os::raw::c_int as usize];
    temp[7 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[5 as std::os::raw::c_int as usize];
    temp[8 as std::os::raw::c_int as usize] = (*(pIn).unwrap()).mat[8 as std::os::raw::c_int as usize];
    memcpy(
        &mut (*(borrow_mut(&mut pOut)).unwrap()).mat as *mut [std::os::raw::c_float; 9] as *mut std::os::raw::c_void,
        temp.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Multiply<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pM1: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
    mut pM2: Option<&'a3 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a4 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a4 {
    let mut mat: [f32; 9] = [0.; 9];
    let mut m1 = ((*((pM1).clone()).unwrap()).mat).as_ptr();
    let mut m2 = ((*((pM2).clone()).unwrap()).mat).as_ptr();
    mat[0 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(3 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize);
    mat[1 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize);
    mat[2 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(0 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(1 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(2 as std::os::raw::c_int as isize);
    mat[3 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(3 as std::os::raw::c_int as isize)
        + *m1.offset(3 as std::os::raw::c_int as isize) * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize);
    mat[4 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(3 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize);
    mat[5 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(3 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(4 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(5 as std::os::raw::c_int as isize);
    mat[6 as std::os::raw::c_int
        as usize] = *m1.offset(0 as std::os::raw::c_int as isize)
        * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(3 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize)
        + *m1.offset(6 as std::os::raw::c_int as isize) * *m2.offset(8 as std::os::raw::c_int as isize);
    mat[7 as std::os::raw::c_int
        as usize] = *m1.offset(1 as std::os::raw::c_int as isize)
        * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(4 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize)
        + *m1.offset(7 as std::os::raw::c_int as isize) * *m2.offset(8 as std::os::raw::c_int as isize);
    mat[8 as std::os::raw::c_int
        as usize] = *m1.offset(2 as std::os::raw::c_int as isize)
        * *m2.offset(6 as std::os::raw::c_int as isize)
        + *m1.offset(5 as std::os::raw::c_int as isize) * *m2.offset(7 as std::os::raw::c_int as isize)
        + *m1.offset(8 as std::os::raw::c_int as isize) * *m2.offset(8 as std::os::raw::c_int as isize);
    memcpy(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        mat.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3ScalarMultiply<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pM: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
    pFactor: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    let mut mat: [f32; 9] = [0.; 9];
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 9 as std::os::raw::c_int {
        mat[i as usize] = (*((pM).clone()).unwrap()).mat[i as usize] * pFactor;
        i += 1;
    }
    memcpy(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        mat.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Assign<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    if _ref_ne(borrow(& pOut) ,  (pIn).clone()) {} else {
        __assert_fail(
            b"pOut != pIn\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/mat3.c\0" as *const u8 as *const std::os::raw::c_char,
            177 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 47], &'_ [i8; 47]>(b"kmMat3 *kmMat3Assign(kmMat3 *, const kmMat3 *)\0"))
                .as_ptr(),
        );
    }
    memcpy(
        ((*(borrow_mut(&mut pOut)).unwrap()).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        ((*((pIn).clone()).unwrap()).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3AssignMat4<'a1, 'a2, 'a3>(
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
pub unsafe extern "C" fn kmMat3AreEqual<'a1, 'a2>(
    mut pMat1: Option<&'a1 crate::src::kazmath::mat3::kmMat3>,
    mut pMat2: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
) -> std::os::raw::c_int {
    let mut i: i32 = 0;
    if _ref_eq((pMat1).clone() ,  (pMat2).clone()) {
        return 1 as std::os::raw::c_int;
    }
    i = 0 as std::os::raw::c_int;
    while i < 9 as std::os::raw::c_int {
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
pub unsafe extern "C" fn kmMat3Rotation<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    radians: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = -sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Scaling(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat3,
    x: std::os::raw::c_float,
    y: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmMat3 {
    kmMat3Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = x;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Translation(
    mut pOut: * mut crate::src::kazmath::mat3::kmMat3,
    x: std::os::raw::c_float,
    y: std::os::raw::c_float,
) -> * mut crate::src::kazmath::mat3::kmMat3 {
    kmMat3Identity(pOut);
    (*pOut).mat[6 as std::os::raw::c_int as usize] = x;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationQuaternion<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmQuaternion>,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    if (pIn).clone().is_none() || borrow(& pOut).is_none() {
        return Option::<&'_ mut crate::src::kazmath::mat3::kmMat3>::None;
    }
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[0 as std::os::raw::c_int
        as usize] = 1.0f32 - 2.0f32 * ((*((pIn).clone()).unwrap()).y * (*((pIn).clone()).unwrap()).y + (*((pIn).clone()).unwrap()).z * (*((pIn).clone()).unwrap()).z);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[1 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*((pIn).clone()).unwrap()).x * (*((pIn).clone()).unwrap()).y - (*((pIn).clone()).unwrap()).w * (*((pIn).clone()).unwrap()).z);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[2 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*((pIn).clone()).unwrap()).x * (*((pIn).clone()).unwrap()).z + (*((pIn).clone()).unwrap()).w * (*((pIn).clone()).unwrap()).y);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[3 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*((pIn).clone()).unwrap()).x * (*((pIn).clone()).unwrap()).y + (*((pIn).clone()).unwrap()).w * (*((pIn).clone()).unwrap()).z);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[4 as std::os::raw::c_int
        as usize] = 1.0f32 - 2.0f32 * ((*((pIn).clone()).unwrap()).x * (*((pIn).clone()).unwrap()).x + (*((pIn).clone()).unwrap()).z * (*((pIn).clone()).unwrap()).z);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[5 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*((pIn).clone()).unwrap()).y * (*((pIn).clone()).unwrap()).z - (*((pIn).clone()).unwrap()).w * (*((pIn).clone()).unwrap()).x);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[6 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*((pIn).clone()).unwrap()).x * (*((pIn).clone()).unwrap()).z - (*((pIn).clone()).unwrap()).w * (*((pIn).clone()).unwrap()).y);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[7 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*((pIn).clone()).unwrap()).y * (*((pIn).clone()).unwrap()).z + (*((pIn).clone()).unwrap()).w * (*((pIn).clone()).unwrap()).x);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[8 as std::os::raw::c_int
        as usize] = 1.0f32 - 2.0f32 * ((*((pIn).clone()).unwrap()).x * (*((pIn).clone()).unwrap()).x + (*((pIn).clone()).unwrap()).y * (*((pIn).clone()).unwrap()).y);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationAxisAngle<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut axis: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut radians: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a3 {
    let mut rcos = cosf(radians);
    let mut rsin = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[0 as std::os::raw::c_int
        as usize] = rcos
        + (*((axis).clone()).unwrap()).x * (*((axis).clone()).unwrap()).x * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[1 as std::os::raw::c_int
        as usize] = (*((axis).clone()).unwrap()).z * rsin
        + (*((axis).clone()).unwrap()).y * (*((axis).clone()).unwrap()).x * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[2 as std::os::raw::c_int
        as usize] = -(*((axis).clone()).unwrap()).y * rsin
        + (*((axis).clone()).unwrap()).z * (*((axis).clone()).unwrap()).x * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[3 as std::os::raw::c_int
        as usize] = -(*((axis).clone()).unwrap()).z * rsin
        + (*((axis).clone()).unwrap()).x * (*((axis).clone()).unwrap()).y * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[4 as std::os::raw::c_int
        as usize] = rcos
        + (*((axis).clone()).unwrap()).y * (*((axis).clone()).unwrap()).y * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[5 as std::os::raw::c_int
        as usize] = (*((axis).clone()).unwrap()).x * rsin
        + (*((axis).clone()).unwrap()).z * (*((axis).clone()).unwrap()).y * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[6 as std::os::raw::c_int
        as usize] = (*((axis).clone()).unwrap()).y * rsin
        + (*((axis).clone()).unwrap()).x * (*((axis).clone()).unwrap()).z * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[7 as std::os::raw::c_int
        as usize] = -(*((axis).clone()).unwrap()).x * rsin
        + (*((axis).clone()).unwrap()).y * (*((axis).clone()).unwrap()).z * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*(borrow_mut(&mut pOut)).unwrap())
        .mat[8 as std::os::raw::c_int
        as usize] = rcos
        + (*((axis).clone()).unwrap()).z * (*((axis).clone()).unwrap()).z * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationToAxisAngle<'a1, 'a2>(
    mut pAxis: * mut crate::src::kazmath::aabb3::kmVec3,
    mut radians: Option<&'a1 mut std::os::raw::c_float>,
    mut pIn: Option<&'a2 crate::src::kazmath::mat3::kmMat3>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut temp = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionRotationMatrix(&mut temp, (pIn).clone());
    kmQuaternionToAxisAngle(Some(&mut temp), pAxis, borrow_mut(&mut radians));
    return pAxis;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationX<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    radians: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = -sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = cosf(radians);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationY<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    radians: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = -sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = 1.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = cosf(radians);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationZ<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    radians: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = -sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = sinf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = cosf(radians);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetUpVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat3>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut).x = (*(pIn).unwrap()).mat[3 as std::os::raw::c_int as usize];
    (*pOut).y = (*(pIn).unwrap()).mat[4 as std::os::raw::c_int as usize];
    (*pOut).z = (*(pIn).unwrap()).mat[5 as std::os::raw::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetRightVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat3>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut).x = (*(pIn).unwrap()).mat[0 as std::os::raw::c_int as usize];
    (*pOut).y = (*(pIn).unwrap()).mat[1 as std::os::raw::c_int as usize];
    (*pOut).z = (*(pIn).unwrap()).mat[2 as std::os::raw::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetForwardVec3<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: Option<&'a1 crate::src::kazmath::mat3::kmMat3>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut).x = (*(pIn).unwrap()).mat[6 as std::os::raw::c_int as usize];
    (*pOut).y = (*(pIn).unwrap()).mat[7 as std::os::raw::c_int as usize];
    (*pOut).z = (*(pIn).unwrap()).mat[8 as std::os::raw::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3LookAt<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::mat3::kmMat3>,
    mut pEye: * const crate::src::kazmath::aabb3::kmVec3,
    mut pCenter: * const crate::src::kazmath::aabb3::kmVec3,
    mut pUp: * const crate::src::kazmath::aabb3::kmVec3,
) -> Option<&'a2 mut crate::src::kazmath::mat3::kmMat3> where 'a1: 'a2 {
    let mut f = kmVec3 { x: 0., y: 0., z: 0. };
    let mut up = kmVec3 { x: 0., y: 0., z: 0. };
    let mut s = kmVec3 { x: 0., y: 0., z: 0. };
    let mut u = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Subtract(&mut f, pCenter, pEye);
    kmVec3Normalize(&mut f, &mut f);
    kmVec3Assign(&mut up, pUp);
    kmVec3Normalize(&mut up, &mut up);
    kmVec3Cross(&mut s, Some(&mut f), &mut up);
    kmVec3Normalize(&mut s, &mut s);
    kmVec3Cross(&mut u, Some(&mut s), &mut f);
    kmVec3Normalize(&mut s, &mut s);
    (*(borrow_mut(&mut pOut)).unwrap()).mat[0 as std::os::raw::c_int as usize] = s.x;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[3 as std::os::raw::c_int as usize] = s.y;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[6 as std::os::raw::c_int as usize] = s.z;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[1 as std::os::raw::c_int as usize] = u.x;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[4 as std::os::raw::c_int as usize] = u.y;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[7 as std::os::raw::c_int as usize] = u.z;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[2 as std::os::raw::c_int as usize] = -f.x;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[5 as std::os::raw::c_int as usize] = -f.y;
    (*(borrow_mut(&mut pOut)).unwrap()).mat[8 as std::os::raw::c_int as usize] = -f.z;
    return pOut;
}
use crate::laertes_rt::*;
