
extern "C" {
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn memset(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn memcmp(
        _: *const std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn cosf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sinf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
    ) -> *mut kmVec3;
    fn kmVec3Subtract(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
    ) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmQuaternionRotationMatrix(
        pOut: *mut kmQuaternion,
        pIn: *const kmMat3,
    ) -> *mut kmQuaternion;
    fn kmQuaternionToAxisAngle(
        pIn: *const kmQuaternion,
        pVector: *mut kmVec3,
        pAngle: *mut std::os::raw::c_float,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [std::os::raw::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [std::os::raw::c_float; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmQuaternion {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
    pub w: std::os::raw::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Fill(
    mut pOut: *mut kmMat3,
    mut pMat: *const std::os::raw::c_float,
) -> *mut kmMat3 {
    memcpy(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        pMat as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Identity(mut pOut: *mut kmMat3) -> *mut kmMat3 {
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
pub unsafe extern "C" fn kmMat3Determinant(mut pIn: *const kmMat3) -> std::os::raw::c_float {
    let mut output: std::os::raw::c_float = 0.;
    output = (*pIn).mat[0 as std::os::raw::c_int as usize]
        * (*pIn).mat[4 as std::os::raw::c_int as usize] * (*pIn).mat[8 as std::os::raw::c_int as usize]
        + (*pIn).mat[1 as std::os::raw::c_int as usize] * (*pIn).mat[5 as std::os::raw::c_int as usize]
            * (*pIn).mat[6 as std::os::raw::c_int as usize]
        + (*pIn).mat[2 as std::os::raw::c_int as usize] * (*pIn).mat[3 as std::os::raw::c_int as usize]
            * (*pIn).mat[7 as std::os::raw::c_int as usize];
    output
        -= (*pIn).mat[2 as std::os::raw::c_int as usize] * (*pIn).mat[4 as std::os::raw::c_int as usize]
            * (*pIn).mat[6 as std::os::raw::c_int as usize]
            + (*pIn).mat[0 as std::os::raw::c_int as usize]
                * (*pIn).mat[5 as std::os::raw::c_int as usize]
                * (*pIn).mat[7 as std::os::raw::c_int as usize]
            + (*pIn).mat[1 as std::os::raw::c_int as usize]
                * (*pIn).mat[3 as std::os::raw::c_int as usize]
                * (*pIn).mat[8 as std::os::raw::c_int as usize];
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Adjugate(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    (*pOut)
        .mat[0 as std::os::raw::c_int
        as usize] = (*pIn).mat[4 as std::os::raw::c_int as usize]
        * (*pIn).mat[8 as std::os::raw::c_int as usize]
        - (*pIn).mat[5 as std::os::raw::c_int as usize] * (*pIn).mat[7 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[1 as std::os::raw::c_int
        as usize] = (*pIn).mat[2 as std::os::raw::c_int as usize]
        * (*pIn).mat[7 as std::os::raw::c_int as usize]
        - (*pIn).mat[1 as std::os::raw::c_int as usize] * (*pIn).mat[8 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[2 as std::os::raw::c_int
        as usize] = (*pIn).mat[1 as std::os::raw::c_int as usize]
        * (*pIn).mat[5 as std::os::raw::c_int as usize]
        - (*pIn).mat[2 as std::os::raw::c_int as usize] * (*pIn).mat[4 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[3 as std::os::raw::c_int
        as usize] = (*pIn).mat[5 as std::os::raw::c_int as usize]
        * (*pIn).mat[6 as std::os::raw::c_int as usize]
        - (*pIn).mat[3 as std::os::raw::c_int as usize] * (*pIn).mat[8 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[4 as std::os::raw::c_int
        as usize] = (*pIn).mat[0 as std::os::raw::c_int as usize]
        * (*pIn).mat[8 as std::os::raw::c_int as usize]
        - (*pIn).mat[2 as std::os::raw::c_int as usize] * (*pIn).mat[6 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[5 as std::os::raw::c_int
        as usize] = (*pIn).mat[2 as std::os::raw::c_int as usize]
        * (*pIn).mat[3 as std::os::raw::c_int as usize]
        - (*pIn).mat[0 as std::os::raw::c_int as usize] * (*pIn).mat[5 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[6 as std::os::raw::c_int
        as usize] = (*pIn).mat[3 as std::os::raw::c_int as usize]
        * (*pIn).mat[7 as std::os::raw::c_int as usize]
        - (*pIn).mat[4 as std::os::raw::c_int as usize] * (*pIn).mat[6 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[7 as std::os::raw::c_int
        as usize] = (*pIn).mat[1 as std::os::raw::c_int as usize]
        * (*pIn).mat[6 as std::os::raw::c_int as usize]
        - (*pIn).mat[0 as std::os::raw::c_int as usize] * (*pIn).mat[7 as std::os::raw::c_int as usize];
    (*pOut)
        .mat[8 as std::os::raw::c_int
        as usize] = (*pIn).mat[0 as std::os::raw::c_int as usize]
        * (*pIn).mat[4 as std::os::raw::c_int as usize]
        - (*pIn).mat[1 as std::os::raw::c_int as usize] * (*pIn).mat[3 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Inverse(
    mut pOut: *mut kmMat3,
    mut pM: *const kmMat3,
) -> *mut kmMat3 {
    let mut determinate = kmMat3Determinant(pM);
    let mut detInv: std::os::raw::c_float = 0.;
    let mut adjugate = kmMat3 { mat: [0.; 9] };
    if determinate as std::os::raw::c_double == 0.0f64 {
        return 0 as *mut kmMat3;
    }
    detInv = (1.0f64 / determinate as std::os::raw::c_double) as std::os::raw::c_float;
    kmMat3Adjugate(&mut adjugate, pM);
    kmMat3ScalarMultiply(pOut, &mut adjugate, detInv);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3IsIdentity(mut pIn: *const kmMat3) -> std::os::raw::c_int {
    static mut identity: [std::os::raw::c_float; 9] = [
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
        0.0f32,
        0.0f32,
        0.0f32,
        1.0f32,
    ];
    return (memcmp(
        identity.as_mut_ptr() as *const std::os::raw::c_void,
        ((*pIn).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Transpose(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    let mut temp: [std::os::raw::c_float; 9] = [0.; 9];
    temp[0 as std::os::raw::c_int as usize] = (*pIn).mat[0 as std::os::raw::c_int as usize];
    temp[1 as std::os::raw::c_int as usize] = (*pIn).mat[3 as std::os::raw::c_int as usize];
    temp[2 as std::os::raw::c_int as usize] = (*pIn).mat[6 as std::os::raw::c_int as usize];
    temp[3 as std::os::raw::c_int as usize] = (*pIn).mat[1 as std::os::raw::c_int as usize];
    temp[4 as std::os::raw::c_int as usize] = (*pIn).mat[4 as std::os::raw::c_int as usize];
    temp[5 as std::os::raw::c_int as usize] = (*pIn).mat[7 as std::os::raw::c_int as usize];
    temp[6 as std::os::raw::c_int as usize] = (*pIn).mat[2 as std::os::raw::c_int as usize];
    temp[7 as std::os::raw::c_int as usize] = (*pIn).mat[5 as std::os::raw::c_int as usize];
    temp[8 as std::os::raw::c_int as usize] = (*pIn).mat[8 as std::os::raw::c_int as usize];
    memcpy(
        &mut (*pOut).mat as *mut [std::os::raw::c_float; 9] as *mut std::os::raw::c_void,
        temp.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Multiply(
    mut pOut: *mut kmMat3,
    mut pM1: *const kmMat3,
    mut pM2: *const kmMat3,
) -> *mut kmMat3 {
    let mut mat: [std::os::raw::c_float; 9] = [0.; 9];
    let mut m1 = ((*pM1).mat).as_ptr();
    let mut m2 = ((*pM2).mat).as_ptr();
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
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        mat.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3ScalarMultiply(
    mut pOut: *mut kmMat3,
    mut pM: *const kmMat3,
    pFactor: std::os::raw::c_float,
) -> *mut kmMat3 {
    let mut mat: [std::os::raw::c_float; 9] = [0.; 9];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 9 as std::os::raw::c_int {
        mat[i as usize] = (*pM).mat[i as usize] * pFactor;
        i += 1;
    }
    memcpy(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        mat.as_mut_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Assign(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    if pOut != pIn as *mut kmMat3 {} else {
        __assert_fail(
            b"pOut != pIn\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/mat3.c\0" as *const u8 as *const std::os::raw::c_char,
            177 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[std::os::raw::c_char; 47],
            >(b"kmMat3 *kmMat3Assign(kmMat3 *, const kmMat3 *)\0"))
                .as_ptr(),
        );
    }
    memcpy(
        ((*pOut).mat).as_mut_ptr() as *mut std::os::raw::c_void,
        ((*pIn).mat).as_ptr() as *const std::os::raw::c_void,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(9 as std::os::raw::c_int as std::os::raw::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3AssignMat4(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat4,
) -> *mut kmMat3 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = (*pIn).mat[0 as std::os::raw::c_int as usize];
    (*pOut).mat[1 as std::os::raw::c_int as usize] = (*pIn).mat[1 as std::os::raw::c_int as usize];
    (*pOut).mat[2 as std::os::raw::c_int as usize] = (*pIn).mat[2 as std::os::raw::c_int as usize];
    (*pOut).mat[3 as std::os::raw::c_int as usize] = (*pIn).mat[4 as std::os::raw::c_int as usize];
    (*pOut).mat[4 as std::os::raw::c_int as usize] = (*pIn).mat[5 as std::os::raw::c_int as usize];
    (*pOut).mat[5 as std::os::raw::c_int as usize] = (*pIn).mat[6 as std::os::raw::c_int as usize];
    (*pOut).mat[6 as std::os::raw::c_int as usize] = (*pIn).mat[8 as std::os::raw::c_int as usize];
    (*pOut).mat[7 as std::os::raw::c_int as usize] = (*pIn).mat[9 as std::os::raw::c_int as usize];
    (*pOut).mat[8 as std::os::raw::c_int as usize] = (*pIn).mat[10 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3AreEqual(
    mut pMat1: *const kmMat3,
    mut pMat2: *const kmMat3,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    if pMat1 == pMat2 {
        return 1 as std::os::raw::c_int;
    }
    i = 0 as std::os::raw::c_int;
    while i < 9 as std::os::raw::c_int {
        if !((*pMat1).mat[i as usize] as std::os::raw::c_double + 0.0001f64
            > (*pMat2).mat[i as usize] as std::os::raw::c_double
            && (*pMat1).mat[i as usize] as std::os::raw::c_double - 0.0001f64
                < (*pMat2).mat[i as usize] as std::os::raw::c_double)
        {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Rotation(
    mut pOut: *mut kmMat3,
    radians: std::os::raw::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as std::os::raw::c_int as usize] = sinf(radians);
    (*pOut).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as std::os::raw::c_int as usize] = -sinf(radians);
    (*pOut).mat[4 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[5 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Scaling(
    mut pOut: *mut kmMat3,
    x: std::os::raw::c_float,
    y: std::os::raw::c_float,
) -> *mut kmMat3 {
    kmMat3Identity(pOut);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = x;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Translation(
    mut pOut: *mut kmMat3,
    x: std::os::raw::c_float,
    y: std::os::raw::c_float,
) -> *mut kmMat3 {
    kmMat3Identity(pOut);
    (*pOut).mat[6 as std::os::raw::c_int as usize] = x;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationQuaternion(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmQuaternion,
) -> *mut kmMat3 {
    if pIn.is_null() || pOut.is_null() {
        return 0 as *mut kmMat3;
    }
    (*pOut)
        .mat[0 as std::os::raw::c_int
        as usize] = 1.0f32 - 2.0f32 * ((*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z);
    (*pOut)
        .mat[1 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*pIn).x * (*pIn).y - (*pIn).w * (*pIn).z);
    (*pOut)
        .mat[2 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*pIn).x * (*pIn).z + (*pIn).w * (*pIn).y);
    (*pOut)
        .mat[3 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*pIn).x * (*pIn).y + (*pIn).w * (*pIn).z);
    (*pOut)
        .mat[4 as std::os::raw::c_int
        as usize] = 1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).z * (*pIn).z);
    (*pOut)
        .mat[5 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*pIn).y * (*pIn).z - (*pIn).w * (*pIn).x);
    (*pOut)
        .mat[6 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*pIn).x * (*pIn).z - (*pIn).w * (*pIn).y);
    (*pOut)
        .mat[7 as std::os::raw::c_int
        as usize] = 2.0f32 * ((*pIn).y * (*pIn).z + (*pIn).w * (*pIn).x);
    (*pOut)
        .mat[8 as std::os::raw::c_int
        as usize] = 1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationAxisAngle(
    mut pOut: *mut kmMat3,
    mut axis: *const kmVec3,
    mut radians: std::os::raw::c_float,
) -> *mut kmMat3 {
    let mut rcos = cosf(radians);
    let mut rsin = sinf(radians);
    (*pOut)
        .mat[0 as std::os::raw::c_int
        as usize] = rcos
        + (*axis).x * (*axis).x * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[1 as std::os::raw::c_int
        as usize] = (*axis).z * rsin
        + (*axis).y * (*axis).x * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[2 as std::os::raw::c_int
        as usize] = -(*axis).y * rsin
        + (*axis).z * (*axis).x * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[3 as std::os::raw::c_int
        as usize] = -(*axis).z * rsin
        + (*axis).x * (*axis).y * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[4 as std::os::raw::c_int
        as usize] = rcos
        + (*axis).y * (*axis).y * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[5 as std::os::raw::c_int
        as usize] = (*axis).x * rsin
        + (*axis).z * (*axis).y * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[6 as std::os::raw::c_int
        as usize] = (*axis).y * rsin
        + (*axis).x * (*axis).z * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[7 as std::os::raw::c_int
        as usize] = -(*axis).x * rsin
        + (*axis).y * (*axis).z * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    (*pOut)
        .mat[8 as std::os::raw::c_int
        as usize] = rcos
        + (*axis).z * (*axis).z * (1 as std::os::raw::c_int as std::os::raw::c_float - rcos);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationToAxisAngle(
    mut pAxis: *mut kmVec3,
    mut radians: *mut std::os::raw::c_float,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    let mut temp = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionRotationMatrix(&mut temp, pIn);
    kmQuaternionToAxisAngle(&mut temp, pAxis, radians);
    return pAxis;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationX(
    mut pOut: *mut kmMat3,
    radians: std::os::raw::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[5 as std::os::raw::c_int as usize] = sinf(radians);
    (*pOut).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = -sinf(radians);
    (*pOut).mat[8 as std::os::raw::c_int as usize] = cosf(radians);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationY(
    mut pOut: *mut kmMat3,
    radians: std::os::raw::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[2 as std::os::raw::c_int as usize] = -sinf(radians);
    (*pOut).mat[3 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = 1.0f32;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[6 as std::os::raw::c_int as usize] = sinf(radians);
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = cosf(radians);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationZ(
    mut pOut: *mut kmMat3,
    radians: std::os::raw::c_float,
) -> *mut kmMat3 {
    (*pOut).mat[0 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[1 as std::os::raw::c_int as usize] = -sinf(radians);
    (*pOut).mat[2 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[3 as std::os::raw::c_int as usize] = sinf(radians);
    (*pOut).mat[4 as std::os::raw::c_int as usize] = cosf(radians);
    (*pOut).mat[5 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[6 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = 0.0f32;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = 1.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetUpVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    (*pOut).x = (*pIn).mat[3 as std::os::raw::c_int as usize];
    (*pOut).y = (*pIn).mat[4 as std::os::raw::c_int as usize];
    (*pOut).z = (*pIn).mat[5 as std::os::raw::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetRightVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    (*pOut).x = (*pIn).mat[0 as std::os::raw::c_int as usize];
    (*pOut).y = (*pIn).mat[1 as std::os::raw::c_int as usize];
    (*pOut).z = (*pIn).mat[2 as std::os::raw::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetForwardVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    (*pOut).x = (*pIn).mat[6 as std::os::raw::c_int as usize];
    (*pOut).y = (*pIn).mat[7 as std::os::raw::c_int as usize];
    (*pOut).z = (*pIn).mat[8 as std::os::raw::c_int as usize];
    kmVec3Normalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3LookAt(
    mut pOut: *mut kmMat3,
    mut pEye: *const kmVec3,
    mut pCenter: *const kmVec3,
    mut pUp: *const kmVec3,
) -> *mut kmMat3 {
    let mut f = kmVec3 { x: 0., y: 0., z: 0. };
    let mut up = kmVec3 { x: 0., y: 0., z: 0. };
    let mut s = kmVec3 { x: 0., y: 0., z: 0. };
    let mut u = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Subtract(&mut f, pCenter, pEye);
    kmVec3Normalize(&mut f, &mut f);
    kmVec3Assign(&mut up, pUp);
    kmVec3Normalize(&mut up, &mut up);
    kmVec3Cross(&mut s, &mut f, &mut up);
    kmVec3Normalize(&mut s, &mut s);
    kmVec3Cross(&mut u, &mut s, &mut f);
    kmVec3Normalize(&mut s, &mut s);
    (*pOut).mat[0 as std::os::raw::c_int as usize] = s.x;
    (*pOut).mat[3 as std::os::raw::c_int as usize] = s.y;
    (*pOut).mat[6 as std::os::raw::c_int as usize] = s.z;
    (*pOut).mat[1 as std::os::raw::c_int as usize] = u.x;
    (*pOut).mat[4 as std::os::raw::c_int as usize] = u.y;
    (*pOut).mat[7 as std::os::raw::c_int as usize] = u.z;
    (*pOut).mat[2 as std::os::raw::c_int as usize] = -f.x;
    (*pOut).mat[5 as std::os::raw::c_int as usize] = -f.y;
    (*pOut).mat[8 as std::os::raw::c_int as usize] = -f.z;
    return pOut;
}
