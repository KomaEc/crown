use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn cosf(_: libc::c_float) -> libc::c_float;
    fn sinf(_: libc::c_float) -> libc::c_float;
    
    
    
    
    
    
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [libc::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [libc::c_float; 9],
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmQuaternion {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Fill(
    mut pOut: *mut kmMat3,
    mut pMat: *const libc::c_float,
) -> *mut kmMat3 {
    memcpy(
        (*pOut).mat.as_mut_ptr() as *mut libc::c_void,
        pMat as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(9 as libc::c_int as libc::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Identity(mut pOut: *mut kmMat3) -> *mut kmMat3 {
    memset(
        (*pOut).mat.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(9 as libc::c_int as libc::c_ulong),
    );
    (*pOut).mat[8 as libc::c_int as usize]= 1.0f32; (*pOut).mat[4 as libc::c_int as usize]= (*pOut).mat[8 as libc::c_int as usize]; (*pOut).mat[0 as libc::c_int as usize]= (*pOut).mat[4 as libc::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Determinant(mut pIn: *const kmMat3) -> libc::c_float {
    let mut output: libc::c_float = 0.;
    output= (*pIn).mat[0 as libc::c_int as usize]
        * (*pIn).mat[4 as libc::c_int as usize] * (*pIn).mat[8 as libc::c_int as usize]
        + (*pIn).mat[1 as libc::c_int as usize] * (*pIn).mat[5 as libc::c_int as usize]
            * (*pIn).mat[6 as libc::c_int as usize]
        + (*pIn).mat[2 as libc::c_int as usize] * (*pIn).mat[3 as libc::c_int as usize]
            * (*pIn).mat[7 as libc::c_int as usize];
    output-= (*pIn).mat[2 as libc::c_int as usize] * (*pIn).mat[4 as libc::c_int as usize]
            * (*pIn).mat[6 as libc::c_int as usize]
            + (*pIn).mat[0 as libc::c_int as usize]
                * (*pIn).mat[5 as libc::c_int as usize]
                * (*pIn).mat[7 as libc::c_int as usize]
            + (*pIn).mat[1 as libc::c_int as usize]
                * (*pIn).mat[3 as libc::c_int as usize]
                * (*pIn).mat[8 as libc::c_int as usize];
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Adjugate(
    mut pOut: Option<&mut kmMat3>,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int
        as usize]= (*pIn).mat[4 as libc::c_int as usize]
        * (*pIn).mat[8 as libc::c_int as usize]
        - (*pIn).mat[5 as libc::c_int as usize] * (*pIn).mat[7 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int
        as usize]= (*pIn).mat[2 as libc::c_int as usize]
        * (*pIn).mat[7 as libc::c_int as usize]
        - (*pIn).mat[1 as libc::c_int as usize] * (*pIn).mat[8 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int
        as usize]= (*pIn).mat[1 as libc::c_int as usize]
        * (*pIn).mat[5 as libc::c_int as usize]
        - (*pIn).mat[2 as libc::c_int as usize] * (*pIn).mat[4 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int
        as usize]= (*pIn).mat[5 as libc::c_int as usize]
        * (*pIn).mat[6 as libc::c_int as usize]
        - (*pIn).mat[3 as libc::c_int as usize] * (*pIn).mat[8 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int
        as usize]= (*pIn).mat[0 as libc::c_int as usize]
        * (*pIn).mat[8 as libc::c_int as usize]
        - (*pIn).mat[2 as libc::c_int as usize] * (*pIn).mat[6 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int
        as usize]= (*pIn).mat[2 as libc::c_int as usize]
        * (*pIn).mat[3 as libc::c_int as usize]
        - (*pIn).mat[0 as libc::c_int as usize] * (*pIn).mat[5 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int
        as usize]= (*pIn).mat[3 as libc::c_int as usize]
        * (*pIn).mat[7 as libc::c_int as usize]
        - (*pIn).mat[4 as libc::c_int as usize] * (*pIn).mat[6 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int
        as usize]= (*pIn).mat[1 as libc::c_int as usize]
        * (*pIn).mat[6 as libc::c_int as usize]
        - (*pIn).mat[0 as libc::c_int as usize] * (*pIn).mat[7 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int
        as usize]= (*pIn).mat[0 as libc::c_int as usize]
        * (*pIn).mat[4 as libc::c_int as usize]
        - (*pIn).mat[1 as libc::c_int as usize] * (*pIn).mat[3 as libc::c_int as usize];
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Inverse(
    mut pOut: Option<&mut kmMat3>,
    mut pM: *const kmMat3,
) -> *mut kmMat3 {
    let mut determinate = kmMat3Determinant(pM);
    let mut detInv: libc::c_float = 0.;
    let mut adjugate = kmMat3 { mat: [0.; 9] };
    if determinate as libc::c_double == 0.0f64 {
        return 0 as *mut kmMat3;
    }
    detInv= (1.0f64 / determinate as libc::c_double) as libc::c_float;
    kmMat3Adjugate(Some(&mut adjugate), pM);
    kmMat3ScalarMultiply(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of!(adjugate), detInv);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3IsIdentity(mut pIn: *const kmMat3) -> libc::c_int {
    static mut identity: [libc::c_float; 9] = [
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
        identity.as_mut_ptr() as *const libc::c_void,
        ((*pIn).mat).as_ptr() as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(9 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Transpose(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmMat3,
) -> *mut kmMat3 {
    let mut temp: [libc::c_float; 9] = [0.; 9];
    temp[0 as libc::c_int as usize]= (*pIn).mat[0 as libc::c_int as usize];
    temp[1 as libc::c_int as usize]= (*pIn).mat[3 as libc::c_int as usize];
    temp[2 as libc::c_int as usize]= (*pIn).mat[6 as libc::c_int as usize];
    temp[3 as libc::c_int as usize]= (*pIn).mat[1 as libc::c_int as usize];
    temp[4 as libc::c_int as usize]= (*pIn).mat[4 as libc::c_int as usize];
    temp[5 as libc::c_int as usize]= (*pIn).mat[7 as libc::c_int as usize];
    temp[6 as libc::c_int as usize]= (*pIn).mat[2 as libc::c_int as usize];
    temp[7 as libc::c_int as usize]= (*pIn).mat[5 as libc::c_int as usize];
    temp[8 as libc::c_int as usize]= (*pIn).mat[8 as libc::c_int as usize];
    memcpy(
        core::ptr::addr_of_mut!((*pOut).mat) as *mut [libc::c_float; 9] as *mut libc::c_void,
        temp.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(9 as libc::c_int as libc::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Multiply(
    mut pOut: *mut kmMat3,
    mut pM1: *const kmMat3,
    mut pM2: *const kmMat3,
) -> *mut kmMat3 {
    let mut mat: [libc::c_float; 9] = [0.; 9];
    let mut m1 = ((*pM1).mat).as_ptr();
    let mut m2 = ((*pM2).mat).as_ptr();
    mat[0 as libc::c_int
        as usize]= *m1.offset(0 as libc::c_int as isize)
        * *m2.offset(0 as libc::c_int as isize)
        + *m1.offset(3 as libc::c_int as isize) * *m2.offset(1 as libc::c_int as isize)
        + *m1.offset(6 as libc::c_int as isize) * *m2.offset(2 as libc::c_int as isize);
    mat[1 as libc::c_int
        as usize]= *m1.offset(1 as libc::c_int as isize)
        * *m2.offset(0 as libc::c_int as isize)
        + *m1.offset(4 as libc::c_int as isize) * *m2.offset(1 as libc::c_int as isize)
        + *m1.offset(7 as libc::c_int as isize) * *m2.offset(2 as libc::c_int as isize);
    mat[2 as libc::c_int
        as usize]= *m1.offset(2 as libc::c_int as isize)
        * *m2.offset(0 as libc::c_int as isize)
        + *m1.offset(5 as libc::c_int as isize) * *m2.offset(1 as libc::c_int as isize)
        + *m1.offset(8 as libc::c_int as isize) * *m2.offset(2 as libc::c_int as isize);
    mat[3 as libc::c_int
        as usize]= *m1.offset(0 as libc::c_int as isize)
        * *m2.offset(3 as libc::c_int as isize)
        + *m1.offset(3 as libc::c_int as isize) * *m2.offset(4 as libc::c_int as isize)
        + *m1.offset(6 as libc::c_int as isize) * *m2.offset(5 as libc::c_int as isize);
    mat[4 as libc::c_int
        as usize]= *m1.offset(1 as libc::c_int as isize)
        * *m2.offset(3 as libc::c_int as isize)
        + *m1.offset(4 as libc::c_int as isize) * *m2.offset(4 as libc::c_int as isize)
        + *m1.offset(7 as libc::c_int as isize) * *m2.offset(5 as libc::c_int as isize);
    mat[5 as libc::c_int
        as usize]= *m1.offset(2 as libc::c_int as isize)
        * *m2.offset(3 as libc::c_int as isize)
        + *m1.offset(5 as libc::c_int as isize) * *m2.offset(4 as libc::c_int as isize)
        + *m1.offset(8 as libc::c_int as isize) * *m2.offset(5 as libc::c_int as isize);
    mat[6 as libc::c_int
        as usize]= *m1.offset(0 as libc::c_int as isize)
        * *m2.offset(6 as libc::c_int as isize)
        + *m1.offset(3 as libc::c_int as isize) * *m2.offset(7 as libc::c_int as isize)
        + *m1.offset(6 as libc::c_int as isize) * *m2.offset(8 as libc::c_int as isize);
    mat[7 as libc::c_int
        as usize]= *m1.offset(1 as libc::c_int as isize)
        * *m2.offset(6 as libc::c_int as isize)
        + *m1.offset(4 as libc::c_int as isize) * *m2.offset(7 as libc::c_int as isize)
        + *m1.offset(7 as libc::c_int as isize) * *m2.offset(8 as libc::c_int as isize);
    mat[8 as libc::c_int
        as usize]= *m1.offset(2 as libc::c_int as isize)
        * *m2.offset(6 as libc::c_int as isize)
        + *m1.offset(5 as libc::c_int as isize) * *m2.offset(7 as libc::c_int as isize)
        + *m1.offset(8 as libc::c_int as isize) * *m2.offset(8 as libc::c_int as isize);
    memcpy(
        (*pOut).mat.as_mut_ptr() as *mut libc::c_void,
        mat.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(9 as libc::c_int as libc::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3ScalarMultiply(
    mut pOut: *mut kmMat3,
    mut pM: *const kmMat3,
    mut pFactor: libc::c_float,
) -> *mut kmMat3 {
    let mut mat: [libc::c_float; 9] = [0.; 9];
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 9 as libc::c_int {
        mat[i as usize]= (*pM).mat[i as usize] * pFactor;
        i+= 1;
    }
    memcpy(
        (*pOut).mat.as_mut_ptr() as *mut libc::c_void,
        mat.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(9 as libc::c_int as libc::c_ulong),
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
            b"pOut != pIn\0" as *const u8 as *const libc::c_char,
            b"../kazmath/mat3.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int as libc::c_uint,
            b"kmMat3 *kmMat3Assign(kmMat3 *, const kmMat3 *)\0" as *const u8 as *const i8,
        );
    }
    memcpy(
        (*pOut).mat.as_mut_ptr() as *mut libc::c_void,
        ((*pIn).mat).as_ptr() as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(9 as libc::c_int as libc::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3AssignMat4(
    mut pOut: Option<&mut kmMat3>,
    mut pIn: *const kmMat4,
) -> *mut kmMat3 {
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int as usize]= (*pIn).mat[0 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int as usize]= (*pIn).mat[1 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int as usize]= (*pIn).mat[2 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int as usize]= (*pIn).mat[4 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int as usize]= (*pIn).mat[5 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int as usize]= (*pIn).mat[6 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int as usize]= (*pIn).mat[8 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int as usize]= (*pIn).mat[9 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int as usize]= (*pIn).mat[10 as libc::c_int as usize];
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3AreEqual(
    mut pMat1: *const kmMat3,
    mut pMat2: *const kmMat3,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if pMat1 == pMat2 {
        return 1 as libc::c_int;
    }
    i= 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if !((*pMat1).mat[i as usize] as libc::c_double + 0.0001f64
            > (*pMat2).mat[i as usize] as libc::c_double
            && (*pMat1).mat[i as usize] as libc::c_double - 0.0001f64
                < (*pMat2).mat[i as usize] as libc::c_double)
        {
            return 0 as libc::c_int;
        }
        i+= 1;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Rotation(
    mut pOut: Option<&mut kmMat3>,
    mut radians: libc::c_float,
) -> *mut kmMat3 {
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int as usize]= cosf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int as usize]= sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int as usize]= -sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int as usize]= cosf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int as usize]= 1.0f32;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Scaling(
    mut pOut: Option<&mut kmMat3>,
    mut x: libc::c_float,
    mut y: libc::c_float,
) -> *mut kmMat3 {
    kmMat3Identity(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int as usize]= x;
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int as usize]= y;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3Translation(
    mut pOut: Option<&mut kmMat3>,
    mut x: libc::c_float,
    mut y: libc::c_float,
) -> *mut kmMat3 {
    kmMat3Identity(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int as usize]= x;
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int as usize]= y;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationQuaternion(
    mut pOut: Option<&mut kmMat3>,
    mut pIn: *const kmQuaternion,
) -> *mut kmMat3 {
    if pIn.is_null() || pOut.as_deref().is_none() {
        return 0 as *mut kmMat3;
    }
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int
        as usize]= 1.0f32 - 2.0f32 * ((*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z);
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int
        as usize]= 2.0f32 * ((*pIn).x * (*pIn).y - (*pIn).w * (*pIn).z);
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int
        as usize]= 2.0f32 * ((*pIn).x * (*pIn).z + (*pIn).w * (*pIn).y);
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int
        as usize]= 2.0f32 * ((*pIn).x * (*pIn).y + (*pIn).w * (*pIn).z);
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int
        as usize]= 1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).z * (*pIn).z);
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int
        as usize]= 2.0f32 * ((*pIn).y * (*pIn).z - (*pIn).w * (*pIn).x);
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int
        as usize]= 2.0f32 * ((*pIn).x * (*pIn).z - (*pIn).w * (*pIn).y);
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int
        as usize]= 2.0f32 * ((*pIn).y * (*pIn).z + (*pIn).w * (*pIn).x);
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int
        as usize]= 1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationAxisAngle(
    mut pOut: Option<&mut kmMat3>,
    mut axis: *const crate::src::kazmath::aabb3::kmVec3,
    mut radians: libc::c_float,
) -> *mut kmMat3 {
    let mut rcos = cosf(radians);
    let mut rsin = sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int
        as usize]= rcos
        + (*axis).x * (*axis).x * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int
        as usize]= (*axis).z * rsin
        + (*axis).y * (*axis).x * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int
        as usize]= -(*axis).y * rsin
        + (*axis).z * (*axis).x * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int
        as usize]= -(*axis).z * rsin
        + (*axis).x * (*axis).y * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int
        as usize]= rcos
        + (*axis).y * (*axis).y * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int
        as usize]= (*axis).x * rsin
        + (*axis).z * (*axis).y * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int
        as usize]= (*axis).y * rsin
        + (*axis).x * (*axis).z * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int
        as usize]= -(*axis).x * rsin
        + (*axis).y * (*axis).z * (1 as libc::c_int as libc::c_float - rcos);
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int
        as usize]= rcos
        + (*axis).z * (*axis).z * (1 as libc::c_int as libc::c_float - rcos);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationToAxisAngle(
    mut pAxis: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut radians: Option<&mut libc::c_float>,
    mut pIn: *const kmMat3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut temp = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    crate::src::kazmath::quaternion::kmQuaternionRotationMatrix(Some(&mut temp), pIn);
    crate::src::kazmath::quaternion::kmQuaternionToAxisAngle(core::ptr::addr_of!(temp), pAxis.as_deref_mut(), radians.as_deref_mut());
    return pAxis.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationX(
    mut pOut: Option<&mut kmMat3>,
    mut radians: libc::c_float,
) -> *mut kmMat3 {
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int as usize]= 1.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int as usize]= cosf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int as usize]= sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int as usize]= -sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int as usize]= cosf(radians);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationY(
    mut pOut: Option<&mut kmMat3>,
    mut radians: libc::c_float,
) -> *mut kmMat3 {
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int as usize]= cosf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int as usize]= -sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int as usize]= 1.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int as usize]= sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int as usize]= cosf(radians);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3RotationZ(
    mut pOut: Option<&mut kmMat3>,
    mut radians: libc::c_float,
) -> *mut kmMat3 {
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int as usize]= cosf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int as usize]= -sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int as usize]= sinf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int as usize]= cosf(radians);
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int as usize]= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int as usize]= 1.0f32;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetUpVec3(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const kmMat3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut.as_deref_mut().unwrap()).x= (*pIn).mat[3 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).y= (*pIn).mat[4 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).z= (*pIn).mat[5 as libc::c_int as usize];
    crate::src::kazmath::vec3::kmVec3Normalize(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pOut.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetRightVec3(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const kmMat3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut.as_deref_mut().unwrap()).x= (*pIn).mat[0 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).y= (*pIn).mat[1 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).z= (*pIn).mat[2 as libc::c_int as usize];
    crate::src::kazmath::vec3::kmVec3Normalize(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pOut.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3GetForwardVec3(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const kmMat3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut.as_deref_mut().unwrap()).x= (*pIn).mat[6 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).y= (*pIn).mat[7 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).z= (*pIn).mat[8 as libc::c_int as usize];
    crate::src::kazmath::vec3::kmVec3Normalize(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pOut.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmMat3LookAt(
    mut pOut: Option<&mut kmMat3>,
    mut pEye: *const crate::src::kazmath::aabb3::kmVec3,
    mut pCenter: *const crate::src::kazmath::aabb3::kmVec3,
    mut pUp: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut kmMat3 {
    let mut f = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut up = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut s = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut u = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    crate::src::kazmath::vec3::kmVec3Subtract(core::ptr::addr_of_mut!(f), pCenter, pEye);
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(f), core::ptr::addr_of!(f));
    crate::src::kazmath::vec3::kmVec3Assign(core::ptr::addr_of_mut!(up), pUp);
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(up), core::ptr::addr_of!(up));
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(s), core::ptr::addr_of!(f), core::ptr::addr_of!(up));
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(s), core::ptr::addr_of!(s));
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(u), core::ptr::addr_of!(s), core::ptr::addr_of!(f));
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(s), core::ptr::addr_of!(s));
    (*pOut.as_deref_mut().unwrap()).mat[0 as libc::c_int as usize]= s.x;
    (*pOut.as_deref_mut().unwrap()).mat[3 as libc::c_int as usize]= s.y;
    (*pOut.as_deref_mut().unwrap()).mat[6 as libc::c_int as usize]= s.z;
    (*pOut.as_deref_mut().unwrap()).mat[1 as libc::c_int as usize]= u.x;
    (*pOut.as_deref_mut().unwrap()).mat[4 as libc::c_int as usize]= u.y;
    (*pOut.as_deref_mut().unwrap()).mat[7 as libc::c_int as usize]= u.z;
    (*pOut.as_deref_mut().unwrap()).mat[2 as libc::c_int as usize]= -f.x;
    (*pOut.as_deref_mut().unwrap()).mat[5 as libc::c_int as usize]= -f.y;
    (*pOut.as_deref_mut().unwrap()).mat[8 as libc::c_int as usize]= -f.z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
