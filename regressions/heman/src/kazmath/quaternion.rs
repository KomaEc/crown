use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn acosf(_: libc::c_float) -> libc::c_float;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cosf(_: libc::c_float) -> libc::c_float;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sinf(_: libc::c_float) -> libc::c_float;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    
    
    
    
    
    
    
    
    
    
    static KM_VEC3_NEG_Z: crate::src::kazmath::aabb3::kmVec3;
    static KM_VEC3_POS_Z: crate::src::kazmath::aabb3::kmVec3;
    static KM_VEC3_POS_Y: crate::src::kazmath::aabb3::kmVec3;
    static KM_VEC3_POS_X: crate::src::kazmath::aabb3::kmVec3;
    static KM_VEC3_ZERO: crate::src::kazmath::aabb3::kmVec3;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor8 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor9 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor10 { dummy: () }
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAreEqual(
    mut p1: *const crate::src::kazmath::mat3::kmQuaternion,
    mut p2: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_int {
    if ((*p1).x as libc::c_double) < (*p2).x as libc::c_double + 0.0001f64
        && (*p1).x as libc::c_double > (*p2).x as libc::c_double - 0.0001f64
        && (((*p1).y as libc::c_double) < (*p2).y as libc::c_double + 0.0001f64
            && (*p1).y as libc::c_double > (*p2).y as libc::c_double - 0.0001f64)
        && (((*p1).z as libc::c_double) < (*p2).z as libc::c_double + 0.0001f64
            && (*p1).z as libc::c_double > (*p2).z as libc::c_double - 0.0001f64)
        && (((*p1).w as libc::c_double) < (*p2).w as libc::c_double + 0.0001f64
            && (*p1).w as libc::c_double > (*p2).w as libc::c_double - 0.0001f64)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionFill(
    mut pOut: Option<&mut crate::src::kazmath::mat3::kmQuaternion>,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut w: libc::c_float,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut.as_deref_mut().unwrap()).x= x;
    (*pOut.as_deref_mut().unwrap()).y= y;
    (*pOut.as_deref_mut().unwrap()).z= z;
    (*pOut.as_deref_mut().unwrap()).w= w;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionDot(
    mut q1: *const crate::src::kazmath::mat3::kmQuaternion,
    mut q2: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_float {
    return (*q1).w * (*q2).w + (*q1).x * (*q2).x + (*q1).y * (*q2).y + (*q1).z * (*q2).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionExp(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
        68 as libc::c_int as libc::c_uint,
        b"kmQuaternion *kmQuaternionExp(kmQuaternion *, const kmQuaternion *)\0" as *const u8 as *const i8,
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIdentity(
    mut pOut: Option<&mut crate::src::kazmath::mat3::kmQuaternion>,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut.as_deref_mut().unwrap()).x= 0.0f64 as libc::c_float;
    (*pOut.as_deref_mut().unwrap()).y= 0.0f64 as libc::c_float;
    (*pOut.as_deref_mut().unwrap()).z= 0.0f64 as libc::c_float;
    (*pOut.as_deref_mut().unwrap()).w= 1.0f64 as libc::c_float;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionInverse(
    mut pOut: Option<&mut crate::src::kazmath::mat3::kmQuaternion>,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    let mut l = kmQuaternionLength(pIn);
    if fabs(l as libc::c_double) < 0.0001f64 {
        (*pOut.as_deref_mut().unwrap()).x= 0.0f64 as libc::c_float;
        (*pOut.as_deref_mut().unwrap()).y= 0.0f64 as libc::c_float;
        (*pOut.as_deref_mut().unwrap()).z= 0.0f64 as libc::c_float;
        (*pOut.as_deref_mut().unwrap()).w= 0.0f64 as libc::c_float;
        return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
    }
    (*pOut.as_deref_mut().unwrap()).x= -(*pIn).x;
    (*pOut.as_deref_mut().unwrap()).y= -(*pIn).y;
    (*pOut.as_deref_mut().unwrap()).z= -(*pIn).z;
    (*pOut.as_deref_mut().unwrap()).w= (*pIn).w;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIsIdentity(
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_int {
    return ((*pIn).x as libc::c_double == 0.0f64 && (*pIn).y as libc::c_double == 0.0f64
        && (*pIn).z as libc::c_double == 0.0f64 && (*pIn).w as libc::c_double == 1.0f64)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLength(
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_float {
    return sqrt(kmQuaternionLengthSq(pIn) as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLengthSq(
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_float {
    return (*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z
        + (*pIn).w * (*pIn).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLn(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
        137 as libc::c_int as libc::c_uint,
        b"kmQuaternion *kmQuaternionLn(kmQuaternion *, const kmQuaternion *)\0" as *const u8 as *const i8,
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiply(
    mut pOut: Option<&mut crate::src::kazmath::mat3::kmQuaternion>,
    mut qu1: *const crate::src::kazmath::mat3::kmQuaternion,
    mut qu2: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    let mut tmp1 = crate::src::kazmath::mat3::kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut tmp2 = crate::src::kazmath::mat3::kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionAssign(core::ptr::addr_of_mut!(tmp1), qu1);
    kmQuaternionAssign(core::ptr::addr_of_mut!(tmp2), qu2);
    let mut q1: *mut crate::src::kazmath::mat3::kmQuaternion = core::ptr::addr_of_mut!(tmp1);
    let mut q2: *mut crate::src::kazmath::mat3::kmQuaternion = core::ptr::addr_of_mut!(tmp2);
    (*pOut.as_deref_mut().unwrap()).x= (*q1).w * (*q2).x + (*q1).x * (*q2).w + (*q1).y * (*q2).z
        - (*q1).z * (*q2).y;
    (*pOut.as_deref_mut().unwrap()).y= (*q1).w * (*q2).y + (*q1).y * (*q2).w + (*q1).z * (*q2).x
        - (*q1).x * (*q2).z;
    (*pOut.as_deref_mut().unwrap()).z= (*q1).w * (*q2).z + (*q1).z * (*q2).w + (*q1).x * (*q2).y
        - (*q1).y * (*q2).x;
    (*pOut.as_deref_mut().unwrap()).w= (*q1).w * (*q2).w - (*q1).x * (*q2).x - (*q1).y * (*q2).y
        - (*q1).z * (*q2).z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionNormalize(
    mut pOut: *mut /* owning */ crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut /* owning */ crate::src::kazmath::mat3::kmQuaternion {
    let mut length = kmQuaternionLength(pIn);
    if fabs(length as libc::c_double) < 0.0001f64 {
        (*pOut).x= 0.0f64 as libc::c_float;
        (*pOut).y= 0.0f64 as libc::c_float;
        (*pOut).z= 0.0f64 as libc::c_float;
        (*pOut).w= 0.0f64 as libc::c_float;
        return pOut;
    }
    kmQuaternionFill(
        pOut.as_mut(),
        (*pOut).x / length,
        (*pOut).y / length,
        (*pOut).z / length,
        (*pOut).w / length,
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationAxisAngle(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
    mut angle: libc::c_float,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    let mut rad = angle * 0.5f32;
    let mut scale = sinf(rad);
    (*pOut).x= (*pV).x * scale;
    (*pOut).y= (*pV).y * scale;
    (*pOut).z= (*pV).z * scale;
    (*pOut).w= cosf(rad);
    kmQuaternionNormalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationMatrix(
    mut pOut: Option<&mut crate::src::kazmath::mat3::kmQuaternion>,
    mut pIn: *const crate::src::kazmath::mat3::kmMat3,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut pMatrix = 0 as *mut libc::c_float;
    let mut m4x4: [libc::c_float; 16] = [
        0 as libc::c_int as libc::c_float,
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
    if pIn.is_null() {();
        return 0 as *mut crate::src::kazmath::mat3::kmQuaternion;
    }
    m4x4[0 as libc::c_int as usize]= (*pIn).mat[0 as libc::c_int as usize];
    m4x4[1 as libc::c_int as usize]= (*pIn).mat[3 as libc::c_int as usize];
    m4x4[2 as libc::c_int as usize]= (*pIn).mat[6 as libc::c_int as usize];
    m4x4[4 as libc::c_int as usize]= (*pIn).mat[1 as libc::c_int as usize];
    m4x4[5 as libc::c_int as usize]= (*pIn).mat[4 as libc::c_int as usize];
    m4x4[6 as libc::c_int as usize]= (*pIn).mat[7 as libc::c_int as usize];
    m4x4[8 as libc::c_int as usize]= (*pIn).mat[2 as libc::c_int as usize];
    m4x4[9 as libc::c_int as usize]= (*pIn).mat[5 as libc::c_int as usize];
    m4x4[10 as libc::c_int as usize]= (*pIn).mat[8 as libc::c_int as usize];
    m4x4[15 as libc::c_int as usize]= 1 as libc::c_int as libc::c_float;
    pMatrix= core::ptr::addr_of_mut!(*m4x4.as_mut_ptr().offset(0 as libc::c_int as isize))
        as *mut libc::c_float;
    diagonal= *pMatrix.offset(0 as libc::c_int as isize)
        + *pMatrix.offset(5 as libc::c_int as isize)
        + *pMatrix.offset(10 as libc::c_int as isize)
        + 1 as libc::c_int as libc::c_float;
    if diagonal as libc::c_double > 0.0001f64 {
        scale= sqrt(diagonal as libc::c_double) as libc::c_float
            * 2 as libc::c_int as libc::c_float;
        x= (*pMatrix.offset(9 as libc::c_int as isize)
            - *pMatrix.offset(6 as libc::c_int as isize)) / scale;
        y= (*pMatrix.offset(2 as libc::c_int as isize)
            - *pMatrix.offset(8 as libc::c_int as isize)) / scale;
        z= (*pMatrix.offset(4 as libc::c_int as isize)
            - *pMatrix.offset(1 as libc::c_int as isize)) / scale;
        w= 0.25f32 * scale;
    } else if *pMatrix.offset(0 as libc::c_int as isize)
        > *pMatrix.offset(5 as libc::c_int as isize)
        && *pMatrix.offset(0 as libc::c_int as isize)
            > *pMatrix.offset(10 as libc::c_int as isize)
    {
        scale= sqrt(
            (1.0f32 + *pMatrix.offset(0 as libc::c_int as isize)
                - *pMatrix.offset(5 as libc::c_int as isize)
                - *pMatrix.offset(10 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float * 2.0f32;
        x= 0.25f32 * scale;
        y= (*pMatrix.offset(4 as libc::c_int as isize)
            + *pMatrix.offset(1 as libc::c_int as isize)) / scale;
        z= (*pMatrix.offset(2 as libc::c_int as isize)
            + *pMatrix.offset(8 as libc::c_int as isize)) / scale;
        w= (*pMatrix.offset(9 as libc::c_int as isize)
            - *pMatrix.offset(6 as libc::c_int as isize)) / scale;
    } else if *pMatrix.offset(5 as libc::c_int as isize)
        > *pMatrix.offset(10 as libc::c_int as isize)
    {
        scale= sqrt(
            (1.0f32 + *pMatrix.offset(5 as libc::c_int as isize)
                - *pMatrix.offset(0 as libc::c_int as isize)
                - *pMatrix.offset(10 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float * 2.0f32;
        x= (*pMatrix.offset(4 as libc::c_int as isize)
            + *pMatrix.offset(1 as libc::c_int as isize)) / scale;
        y= 0.25f32 * scale;
        z= (*pMatrix.offset(9 as libc::c_int as isize)
            + *pMatrix.offset(6 as libc::c_int as isize)) / scale;
        w= (*pMatrix.offset(2 as libc::c_int as isize)
            - *pMatrix.offset(8 as libc::c_int as isize)) / scale;
    } else {
        scale= sqrt(
            (1.0f32 + *pMatrix.offset(10 as libc::c_int as isize)
                - *pMatrix.offset(0 as libc::c_int as isize)
                - *pMatrix.offset(5 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float * 2.0f32;
        x= (*pMatrix.offset(2 as libc::c_int as isize)
            + *pMatrix.offset(8 as libc::c_int as isize)) / scale;
        y= (*pMatrix.offset(9 as libc::c_int as isize)
            + *pMatrix.offset(6 as libc::c_int as isize)) / scale;
        z= 0.25f32 * scale;
        w= (*pMatrix.offset(4 as libc::c_int as isize)
            - *pMatrix.offset(1 as libc::c_int as isize)) / scale;
    }
    (*pOut.as_deref_mut().unwrap()).x= x;
    (*pOut.as_deref_mut().unwrap()).y= y;
    (*pOut.as_deref_mut().unwrap()).z= z;
    (*pOut.as_deref_mut().unwrap()).w= w;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationPitchYawRoll(
    mut pOut: Option<&mut crate::src::kazmath::mat3::kmQuaternion>,
    mut pitch: libc::c_float,
    mut yaw: libc::c_float,
    mut roll: libc::c_float,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    if pitch <= 2 as libc::c_int as libc::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"pitch <= 2*kmPI\0" as *const u8 as *const libc::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int as libc::c_uint,
            b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0" as *const u8 as *const i8,
        );
    }
    if yaw <= 2 as libc::c_int as libc::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"yaw <= 2*kmPI\0" as *const u8 as *const libc::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int as libc::c_uint,
            b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0" as *const u8 as *const i8,
        );
    }
    if roll <= 2 as libc::c_int as libc::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"roll <= 2*kmPI\0" as *const u8 as *const libc::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int as libc::c_uint,
            b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0" as *const u8 as *const i8,
        );
    }
    let mut sY = sinf((yaw as libc::c_double * 0.5f64) as libc::c_float);
    let mut cY = cosf((yaw as libc::c_double * 0.5f64) as libc::c_float);
    let mut sZ = sinf((roll as libc::c_double * 0.5f64) as libc::c_float);
    let mut cZ = cosf((roll as libc::c_double * 0.5f64) as libc::c_float);
    let mut sX = sinf((pitch as libc::c_double * 0.5f64) as libc::c_float);
    let mut cX = cosf((pitch as libc::c_double * 0.5f64) as libc::c_float);
    (*pOut.as_deref_mut().unwrap()).w= cY * cZ * cX - sY * sZ * sX;
    (*pOut.as_deref_mut().unwrap()).x= sY * sZ * cX + cY * cZ * sX;
    (*pOut.as_deref_mut().unwrap()).y= sY * cZ * cX + cY * sZ * sX;
    (*pOut.as_deref_mut().unwrap()).z= cY * sZ * cX - sY * cZ * sX;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionSlerp(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut q1: *const crate::src::kazmath::mat3::kmQuaternion,
    mut q2: *const crate::src::kazmath::mat3::kmQuaternion,
    mut t: libc::c_float,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    let mut dot = kmQuaternionDot(q1, q2);
    let DOT_THRESHOLD = 0.9995f64;
    if dot as libc::c_double > DOT_THRESHOLD {
        let mut diff = crate::src::kazmath::mat3::kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionSubtract(core::ptr::addr_of_mut!(diff), q2, q1);
        kmQuaternionScale(core::ptr::addr_of_mut!(diff), core::ptr::addr_of!(diff), t);
        kmQuaternionAdd(pOut, q1, core::ptr::addr_of!(diff));
        kmQuaternionNormalize(pOut, pOut);
        return pOut;
    }
    dot= crate::src::kazmath::utility::kmClamp(
        dot,
        -(1 as libc::c_int) as libc::c_float,
        1 as libc::c_int as libc::c_float,
    );
    let mut theta_0 = acos(dot as libc::c_double) as libc::c_float;
    let mut theta = theta_0 * t;
    let mut tmp = crate::src::kazmath::mat3::kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionScale(core::ptr::addr_of_mut!(tmp), q1, dot);
    kmQuaternionSubtract(core::ptr::addr_of_mut!(tmp), q2, core::ptr::addr_of!(tmp));
    kmQuaternionNormalize(core::ptr::addr_of_mut!(tmp), core::ptr::addr_of!(tmp));
    let mut t1 = crate::src::kazmath::mat3::kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut t2 = crate::src::kazmath::mat3::kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionScale(core::ptr::addr_of_mut!(t1), q1, cos(theta as libc::c_double) as libc::c_float);
    kmQuaternionScale(core::ptr::addr_of_mut!(t2), core::ptr::addr_of!(tmp), sin(theta as libc::c_double) as libc::c_float);
    kmQuaternionAdd(pOut, core::ptr::addr_of!(t1), core::ptr::addr_of!(t2));
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionToAxisAngle(
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
    mut pAxis: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pAngle: Option<&mut libc::c_float>,
) {
    let mut tempAngle: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    tempAngle= acosf((*pIn).w);
    scale= sqrtf(crate::src::kazmath::utility::kmSQR((*pIn).x) + crate::src::kazmath::utility::kmSQR((*pIn).y) + crate::src::kazmath::utility::kmSQR((*pIn).z));
    if scale as libc::c_double > -0.0001f64 && (scale as libc::c_double) < 0.0001f64
        || (scale as libc::c_double)
            < (2 as libc::c_int as libc::c_float * 3.14159265358979323846f32)
                as libc::c_double + 0.0001f64
            && scale as libc::c_double
                > (2 as libc::c_int as libc::c_float * 3.14159265358979323846f32)
                    as libc::c_double - 0.0001f64
    {
        *pAngle.as_deref_mut().unwrap()= 0.0f32;
        (*pAxis.as_deref_mut().unwrap()).x= 0.0f32;
        (*pAxis.as_deref_mut().unwrap()).y= 0.0f32;
        (*pAxis.as_deref_mut().unwrap()).z= 1.0f32;
    } else {
        *pAngle.as_deref_mut().unwrap()= tempAngle * 2.0f32;
        (*pAxis.as_deref_mut().unwrap()).x= (*pIn).x / scale;
        (*pAxis.as_deref_mut().unwrap()).y= (*pIn).y / scale;
        (*pAxis.as_deref_mut().unwrap()).z= (*pIn).z / scale;
        crate::src::kazmath::vec3::kmVec3Normalize(pAxis.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pAxis.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
    };
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionScale(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
    mut s: libc::c_float,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x= (*pIn).x * s;
    (*pOut).y= (*pIn).y * s;
    (*pOut).z= (*pIn).z * s;
    (*pOut).w= (*pIn).w * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAssign(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    memcpy(
        pOut as *mut libc::c_void,
        pIn as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionSubtract(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut pQ1: *const crate::src::kazmath::mat3::kmQuaternion,
    mut pQ2: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x= (*pQ1).x - (*pQ2).x;
    (*pOut).y= (*pQ1).y - (*pQ2).y;
    (*pOut).z= (*pQ1).z - (*pQ2).z;
    (*pOut).w= (*pQ1).w - (*pQ2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAdd(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut pQ1: *const crate::src::kazmath::mat3::kmQuaternion,
    mut pQ2: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    (*pOut).x= (*pQ1).x + (*pQ2).x;
    (*pOut).y= (*pQ1).y + (*pQ2).y;
    (*pOut).z= (*pQ1).z + (*pQ2).z;
    (*pOut).w= (*pQ1).w + (*pQ2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationBetweenVec3(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut vec1: *const crate::src::kazmath::aabb3::kmVec3,
    mut vec2: *const crate::src::kazmath::aabb3::kmVec3,
    mut fallback: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    let mut v1 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut v2 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut a: libc::c_float = 0.;
    crate::src::kazmath::vec3::kmVec3Assign(core::ptr::addr_of_mut!(v1), vec1);
    crate::src::kazmath::vec3::kmVec3Assign(core::ptr::addr_of_mut!(v2), vec2);
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(v1), core::ptr::addr_of!(v1));
    crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(v2), core::ptr::addr_of!(v2));
    a= crate::src::kazmath::vec3::kmVec3Dot(core::ptr::addr_of!(v1), core::ptr::addr_of!(v2));
    if a as libc::c_double >= 1.0f64 {
        kmQuaternionIdentity(pOut.as_mut());
        return pOut;
    }
    if a < 1e-6f32 - 1.0f32 {
        if fabs(crate::src::kazmath::vec3::kmVec3LengthSq(fallback) as libc::c_double) < 0.0001f64 {
            kmQuaternionRotationAxisAngle(pOut, fallback, 3.14159265358979323846f32);
        } else {
            let mut axis = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
            let mut X = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
            X.x= 1.0f64 as libc::c_float;
            X.y= 0.0f64 as libc::c_float;
            X.z= 0.0f64 as libc::c_float;
            crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(axis), core::ptr::addr_of!(X), vec1);
            if fabs(crate::src::kazmath::vec3::kmVec3LengthSq(core::ptr::addr_of!(axis)) as libc::c_double) < 0.0001f64 {
                let mut Y = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
                Y.x= 0.0f64 as libc::c_float;
                Y.y= 1.0f64 as libc::c_float;
                Y.z= 0.0f64 as libc::c_float;
                crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(axis), core::ptr::addr_of!(Y), vec1);
            }
            crate::src::kazmath::vec3::kmVec3Normalize(core::ptr::addr_of_mut!(axis), core::ptr::addr_of!(axis));
            kmQuaternionRotationAxisAngle(pOut, core::ptr::addr_of!(axis), 3.14159265358979323846f32);
        }
    } else {
        let mut s = sqrtf(
            (1 as libc::c_int as libc::c_float + a) * 2 as libc::c_int as libc::c_float,
        );
        let mut invs = 1 as libc::c_int as libc::c_float / s;
        let mut c = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
        crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(c), core::ptr::addr_of!(v1), core::ptr::addr_of!(v2));
        (*pOut).x= c.x * invs;
        (*pOut).y= c.y * invs;
        (*pOut).z= c.z * invs;
        (*pOut).w= s * 0.5f32;
        kmQuaternionNormalize(pOut, pOut);
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiplyVec3(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut q: *const crate::src::kazmath::mat3::kmQuaternion,
    mut v: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut uv = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut uuv = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut qvec = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    qvec.x= (*q).x;
    qvec.y= (*q).y;
    qvec.z= (*q).z;
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(uv), core::ptr::addr_of!(qvec), v);
    crate::src::kazmath::vec3::kmVec3Cross(core::ptr::addr_of_mut!(uuv), core::ptr::addr_of!(qvec), core::ptr::addr_of!(uv));
    crate::src::kazmath::vec3::kmVec3Scale(core::ptr::addr_of_mut!(uv), core::ptr::addr_of!(uv), 2.0f32 * (*q).w);
    crate::src::kazmath::vec3::kmVec3Scale(core::ptr::addr_of_mut!(uuv), core::ptr::addr_of!(uuv), 2.0f32);
    crate::src::kazmath::vec3::kmVec3Add(pOut, v, core::ptr::addr_of!(uv));
    crate::src::kazmath::vec3::kmVec3Add(pOut, pOut, core::ptr::addr_of!(uuv));
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetUpVec3(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pIn, &KM_VEC3_POS_Y);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRightVec3(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pIn, &KM_VEC3_POS_X);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3RH(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pIn, &KM_VEC3_NEG_Z);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3LH(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const crate::src::kazmath::mat3::kmQuaternion,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    return kmQuaternionMultiplyVec3(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), pIn, &KM_VEC3_POS_Z);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetPitch(
    mut q: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_float {
    let mut result = atan2(
        (2 as libc::c_int as libc::c_float * ((*q).y * (*q).z + (*q).w * (*q).x))
            as libc::c_double,
        ((*q).w * (*q).w - (*q).x * (*q).x - (*q).y * (*q).y + (*q).z * (*q).z)
            as libc::c_double,
    ) as libc::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetYaw(
    mut q: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_float {
    let mut result = asin(
        (-(2 as libc::c_int) as libc::c_float * ((*q).x * (*q).z - (*q).w * (*q).y))
            as libc::c_double,
    ) as libc::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRoll(
    mut q: *const crate::src::kazmath::mat3::kmQuaternion,
) -> libc::c_float {
    let mut result = atan2(
        (2 as libc::c_int as libc::c_float * ((*q).x * (*q).y + (*q).w * (*q).z))
            as libc::c_double,
        ((*q).w * (*q).w + (*q).x * (*q).x - (*q).y * (*q).y - (*q).z * (*q).z)
            as libc::c_double,
    ) as libc::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLookRotation(
    mut pOut: *mut crate::src::kazmath::mat3::kmQuaternion,
    mut direction: *const crate::src::kazmath::aabb3::kmVec3,
    mut up: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::mat3::kmQuaternion {
    let mut tmp = crate::src::kazmath::mat3::kmMat3 { mat: [0.; 9] };
    crate::src::kazmath::mat3::kmMat3LookAt(Some(&mut tmp), &KM_VEC3_ZERO, direction, up);
    return kmQuaternionNormalize(pOut, kmQuaternionRotationMatrix(pOut.as_mut(), core::ptr::addr_of!(tmp)));
}
