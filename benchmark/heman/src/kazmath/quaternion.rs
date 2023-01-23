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
    fn kmClamp(
        x: libc::c_float,
        min: libc::c_float,
        max: libc::c_float,
    ) -> libc::c_float;
    fn kmSQR(s: libc::c_float) -> libc::c_float;
    fn kmMat3LookAt(
        pOut: *mut kmMat3,
        pEye: *const kmVec3,
        pCenter: *const kmVec3,
        pUp: *const kmVec3,
    ) -> *mut kmMat3;
    fn kmVec3LengthSq(pIn: *const kmVec3) -> libc::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
    ) -> *mut kmVec3;
    fn kmVec3Dot(pV1: *const kmVec3, pV2: *const kmVec3) -> libc::c_float;
    fn kmVec3Add(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
    ) -> *mut kmVec3;
    fn kmVec3Scale(
        pOut: *mut kmVec3,
        pIn: *const kmVec3,
        s: libc::c_float,
    ) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    static KM_VEC3_NEG_Z: kmVec3;
    static KM_VEC3_POS_Z: kmVec3;
    static KM_VEC3_POS_Y: kmVec3;
    static KM_VEC3_POS_X: kmVec3;
    static KM_VEC3_ZERO: kmVec3;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [libc::c_float; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmQuaternion {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAreEqual(
    mut p1: *const kmQuaternion,
    mut p2: *const kmQuaternion,
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
    mut pOut: *mut kmQuaternion,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut w: libc::c_float,
) -> *mut kmQuaternion {
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionDot(
    mut q1: *const kmQuaternion,
    mut q2: *const kmQuaternion,
) -> libc::c_float {
    return (*q1).w * (*q2).w + (*q1).x * (*q2).x + (*q1).y * (*q2).y + (*q1).z * (*q2).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionExp(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
        68 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 68],
            &[libc::c_char; 68],
        >(b"kmQuaternion *kmQuaternionExp(kmQuaternion *, const kmQuaternion *)\0"))
            .as_ptr(),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIdentity(
    mut pOut: *mut kmQuaternion,
) -> *mut kmQuaternion {
    (*pOut).x = 0.0f64 as libc::c_float;
    (*pOut).y = 0.0f64 as libc::c_float;
    (*pOut).z = 0.0f64 as libc::c_float;
    (*pOut).w = 1.0f64 as libc::c_float;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionInverse(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    let mut l = kmQuaternionLength(pIn);
    if fabs(l as libc::c_double) < 0.0001f64 {
        (*pOut).x = 0.0f64 as libc::c_float;
        (*pOut).y = 0.0f64 as libc::c_float;
        (*pOut).z = 0.0f64 as libc::c_float;
        (*pOut).w = 0.0f64 as libc::c_float;
        return pOut;
    }
    (*pOut).x = -(*pIn).x;
    (*pOut).y = -(*pIn).y;
    (*pOut).z = -(*pIn).z;
    (*pOut).w = (*pIn).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionIsIdentity(
    mut pIn: *const kmQuaternion,
) -> libc::c_int {
    return ((*pIn).x as libc::c_double == 0.0f64 && (*pIn).y as libc::c_double == 0.0f64
        && (*pIn).z as libc::c_double == 0.0f64 && (*pIn).w as libc::c_double == 1.0f64)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLength(
    mut pIn: *const kmQuaternion,
) -> libc::c_float {
    return sqrt(kmQuaternionLengthSq(pIn) as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLengthSq(
    mut pIn: *const kmQuaternion,
) -> libc::c_float {
    return (*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z
        + (*pIn).w * (*pIn).w;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionLn(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
        137 as libc::c_int as libc::c_uint,
        (*::std::mem::transmute::<
            &[u8; 67],
            &[libc::c_char; 67],
        >(b"kmQuaternion *kmQuaternionLn(kmQuaternion *, const kmQuaternion *)\0"))
            .as_ptr(),
    );
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiply(
    mut pOut: *mut kmQuaternion,
    mut qu1: *const kmQuaternion,
    mut qu2: *const kmQuaternion,
) -> *mut kmQuaternion {
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
    let mut q1: *mut kmQuaternion = &mut tmp1;
    let mut q2: *mut kmQuaternion = &mut tmp2;
    (*pOut)
        .x = (*q1).w * (*q2).x + (*q1).x * (*q2).w + (*q1).y * (*q2).z
        - (*q1).z * (*q2).y;
    (*pOut)
        .y = (*q1).w * (*q2).y + (*q1).y * (*q2).w + (*q1).z * (*q2).x
        - (*q1).x * (*q2).z;
    (*pOut)
        .z = (*q1).w * (*q2).z + (*q1).z * (*q2).w + (*q1).x * (*q2).y
        - (*q1).y * (*q2).x;
    (*pOut)
        .w = (*q1).w * (*q2).w - (*q1).x * (*q2).x - (*q1).y * (*q2).y
        - (*q1).z * (*q2).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionNormalize(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    let mut length = kmQuaternionLength(pIn);
    if fabs(length as libc::c_double) < 0.0001f64 {
        (*pOut).x = 0.0f64 as libc::c_float;
        (*pOut).y = 0.0f64 as libc::c_float;
        (*pOut).z = 0.0f64 as libc::c_float;
        (*pOut).w = 0.0f64 as libc::c_float;
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
pub unsafe extern "C" fn kmQuaternionRotationAxisAngle(
    mut pOut: *mut kmQuaternion,
    mut pV: *const kmVec3,
    mut angle: libc::c_float,
) -> *mut kmQuaternion {
    let mut rad = angle * 0.5f32;
    let mut scale = sinf(rad);
    (*pOut).x = (*pV).x * scale;
    (*pOut).y = (*pV).y * scale;
    (*pOut).z = (*pV).z * scale;
    (*pOut).w = cosf(rad);
    kmQuaternionNormalize(pOut, pOut);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationMatrix(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmMat3,
) -> *mut kmQuaternion {
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
    if pIn.is_null() {
        return 0 as *mut kmQuaternion;
    }
    m4x4[0 as libc::c_int as usize] = (*pIn).mat[0 as libc::c_int as usize];
    m4x4[1 as libc::c_int as usize] = (*pIn).mat[3 as libc::c_int as usize];
    m4x4[2 as libc::c_int as usize] = (*pIn).mat[6 as libc::c_int as usize];
    m4x4[4 as libc::c_int as usize] = (*pIn).mat[1 as libc::c_int as usize];
    m4x4[5 as libc::c_int as usize] = (*pIn).mat[4 as libc::c_int as usize];
    m4x4[6 as libc::c_int as usize] = (*pIn).mat[7 as libc::c_int as usize];
    m4x4[8 as libc::c_int as usize] = (*pIn).mat[2 as libc::c_int as usize];
    m4x4[9 as libc::c_int as usize] = (*pIn).mat[5 as libc::c_int as usize];
    m4x4[10 as libc::c_int as usize] = (*pIn).mat[8 as libc::c_int as usize];
    m4x4[15 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
    pMatrix = &mut *m4x4.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_float;
    diagonal = *pMatrix.offset(0 as libc::c_int as isize)
        + *pMatrix.offset(5 as libc::c_int as isize)
        + *pMatrix.offset(10 as libc::c_int as isize)
        + 1 as libc::c_int as libc::c_float;
    if diagonal as libc::c_double > 0.0001f64 {
        scale = sqrt(diagonal as libc::c_double) as libc::c_float
            * 2 as libc::c_int as libc::c_float;
        x = (*pMatrix.offset(9 as libc::c_int as isize)
            - *pMatrix.offset(6 as libc::c_int as isize)) / scale;
        y = (*pMatrix.offset(2 as libc::c_int as isize)
            - *pMatrix.offset(8 as libc::c_int as isize)) / scale;
        z = (*pMatrix.offset(4 as libc::c_int as isize)
            - *pMatrix.offset(1 as libc::c_int as isize)) / scale;
        w = 0.25f32 * scale;
    } else if *pMatrix.offset(0 as libc::c_int as isize)
        > *pMatrix.offset(5 as libc::c_int as isize)
        && *pMatrix.offset(0 as libc::c_int as isize)
            > *pMatrix.offset(10 as libc::c_int as isize)
    {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(0 as libc::c_int as isize)
                - *pMatrix.offset(5 as libc::c_int as isize)
                - *pMatrix.offset(10 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float * 2.0f32;
        x = 0.25f32 * scale;
        y = (*pMatrix.offset(4 as libc::c_int as isize)
            + *pMatrix.offset(1 as libc::c_int as isize)) / scale;
        z = (*pMatrix.offset(2 as libc::c_int as isize)
            + *pMatrix.offset(8 as libc::c_int as isize)) / scale;
        w = (*pMatrix.offset(9 as libc::c_int as isize)
            - *pMatrix.offset(6 as libc::c_int as isize)) / scale;
    } else if *pMatrix.offset(5 as libc::c_int as isize)
        > *pMatrix.offset(10 as libc::c_int as isize)
    {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(5 as libc::c_int as isize)
                - *pMatrix.offset(0 as libc::c_int as isize)
                - *pMatrix.offset(10 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float * 2.0f32;
        x = (*pMatrix.offset(4 as libc::c_int as isize)
            + *pMatrix.offset(1 as libc::c_int as isize)) / scale;
        y = 0.25f32 * scale;
        z = (*pMatrix.offset(9 as libc::c_int as isize)
            + *pMatrix.offset(6 as libc::c_int as isize)) / scale;
        w = (*pMatrix.offset(2 as libc::c_int as isize)
            - *pMatrix.offset(8 as libc::c_int as isize)) / scale;
    } else {
        scale = sqrt(
            (1.0f32 + *pMatrix.offset(10 as libc::c_int as isize)
                - *pMatrix.offset(0 as libc::c_int as isize)
                - *pMatrix.offset(5 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float * 2.0f32;
        x = (*pMatrix.offset(2 as libc::c_int as isize)
            + *pMatrix.offset(8 as libc::c_int as isize)) / scale;
        y = (*pMatrix.offset(9 as libc::c_int as isize)
            + *pMatrix.offset(6 as libc::c_int as isize)) / scale;
        z = 0.25f32 * scale;
        w = (*pMatrix.offset(4 as libc::c_int as isize)
            - *pMatrix.offset(1 as libc::c_int as isize)) / scale;
    }
    (*pOut).x = x;
    (*pOut).y = y;
    (*pOut).z = z;
    (*pOut).w = w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationPitchYawRoll(
    mut pOut: *mut kmQuaternion,
    mut pitch: libc::c_float,
    mut yaw: libc::c_float,
    mut roll: libc::c_float,
) -> *mut kmQuaternion {
    if pitch <= 2 as libc::c_int as libc::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"pitch <= 2*kmPI\0" as *const u8 as *const libc::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",
            ))
                .as_ptr(),
        );
    }
    if yaw <= 2 as libc::c_int as libc::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"yaw <= 2*kmPI\0" as *const u8 as *const libc::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",
            ))
                .as_ptr(),
        );
    }
    if roll <= 2 as libc::c_int as libc::c_float * 3.14159265358979323846f32 {} else {
        __assert_fail(
            b"roll <= 2*kmPI\0" as *const u8 as *const libc::c_char,
            b"../kazmath/quaternion.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",
            ))
                .as_ptr(),
        );
    }
    let mut sY = sinf((yaw as libc::c_double * 0.5f64) as libc::c_float);
    let mut cY = cosf((yaw as libc::c_double * 0.5f64) as libc::c_float);
    let mut sZ = sinf((roll as libc::c_double * 0.5f64) as libc::c_float);
    let mut cZ = cosf((roll as libc::c_double * 0.5f64) as libc::c_float);
    let mut sX = sinf((pitch as libc::c_double * 0.5f64) as libc::c_float);
    let mut cX = cosf((pitch as libc::c_double * 0.5f64) as libc::c_float);
    (*pOut).w = cY * cZ * cX - sY * sZ * sX;
    (*pOut).x = sY * sZ * cX + cY * cZ * sX;
    (*pOut).y = sY * cZ * cX + cY * sZ * sX;
    (*pOut).z = cY * sZ * cX - sY * cZ * sX;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionSlerp(
    mut pOut: *mut kmQuaternion,
    mut q1: *const kmQuaternion,
    mut q2: *const kmQuaternion,
    mut t: libc::c_float,
) -> *mut kmQuaternion {
    let mut dot = kmQuaternionDot(q1, q2);
    let DOT_THRESHOLD = 0.9995f64;
    if dot as libc::c_double > DOT_THRESHOLD {
        let mut diff = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionSubtract(&mut diff, q2, q1);
        kmQuaternionScale(&mut diff, &mut diff, t);
        kmQuaternionAdd(pOut, q1, &mut diff);
        kmQuaternionNormalize(pOut, pOut);
        return pOut;
    }
    dot = kmClamp(
        dot,
        -(1 as libc::c_int) as libc::c_float,
        1 as libc::c_int as libc::c_float,
    );
    let mut theta_0 = acos(dot as libc::c_double) as libc::c_float;
    let mut theta = theta_0 * t;
    let mut tmp = kmQuaternion {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    kmQuaternionScale(&mut tmp, q1, dot);
    kmQuaternionSubtract(&mut tmp, q2, &mut tmp);
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
    kmQuaternionScale(&mut t1, q1, cos(theta as libc::c_double) as libc::c_float);
    kmQuaternionScale(&mut t2, &mut tmp, sin(theta as libc::c_double) as libc::c_float);
    kmQuaternionAdd(pOut, &mut t1, &mut t2);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionToAxisAngle(
    mut pIn: *const kmQuaternion,
    mut pAxis: *mut kmVec3,
    mut pAngle: *mut libc::c_float,
) {
    let mut tempAngle: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    tempAngle = acosf((*pIn).w);
    scale = sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z));
    if scale as libc::c_double > -0.0001f64 && (scale as libc::c_double) < 0.0001f64
        || (scale as libc::c_double)
            < (2 as libc::c_int as libc::c_float * 3.14159265358979323846f32)
                as libc::c_double + 0.0001f64
            && scale as libc::c_double
                > (2 as libc::c_int as libc::c_float * 3.14159265358979323846f32)
                    as libc::c_double - 0.0001f64
    {
        *pAngle = 0.0f32;
        (*pAxis).x = 0.0f32;
        (*pAxis).y = 0.0f32;
        (*pAxis).z = 1.0f32;
    } else {
        *pAngle = tempAngle * 2.0f32;
        (*pAxis).x = (*pIn).x / scale;
        (*pAxis).y = (*pIn).y / scale;
        (*pAxis).z = (*pIn).z / scale;
        kmVec3Normalize(pAxis, pAxis);
    };
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionScale(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
    mut s: libc::c_float,
) -> *mut kmQuaternion {
    (*pOut).x = (*pIn).x * s;
    (*pOut).y = (*pIn).y * s;
    (*pOut).z = (*pIn).z * s;
    (*pOut).w = (*pIn).w * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAssign(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
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
    mut pOut: *mut kmQuaternion,
    mut pQ1: *const kmQuaternion,
    mut pQ2: *const kmQuaternion,
) -> *mut kmQuaternion {
    (*pOut).x = (*pQ1).x - (*pQ2).x;
    (*pOut).y = (*pQ1).y - (*pQ2).y;
    (*pOut).z = (*pQ1).z - (*pQ2).z;
    (*pOut).w = (*pQ1).w - (*pQ2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionAdd(
    mut pOut: *mut kmQuaternion,
    mut pQ1: *const kmQuaternion,
    mut pQ2: *const kmQuaternion,
) -> *mut kmQuaternion {
    (*pOut).x = (*pQ1).x + (*pQ2).x;
    (*pOut).y = (*pQ1).y + (*pQ2).y;
    (*pOut).z = (*pQ1).z + (*pQ2).z;
    (*pOut).w = (*pQ1).w + (*pQ2).w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionRotationBetweenVec3(
    mut pOut: *mut kmQuaternion,
    mut vec1: *const kmVec3,
    mut vec2: *const kmVec3,
    mut fallback: *const kmVec3,
) -> *mut kmQuaternion {
    let mut v1 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut v2 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut a: libc::c_float = 0.;
    kmVec3Assign(&mut v1, vec1);
    kmVec3Assign(&mut v2, vec2);
    kmVec3Normalize(&mut v1, &mut v1);
    kmVec3Normalize(&mut v2, &mut v2);
    a = kmVec3Dot(&mut v1, &mut v2);
    if a as libc::c_double >= 1.0f64 {
        kmQuaternionIdentity(pOut);
        return pOut;
    }
    if a < 1e-6f32 - 1.0f32 {
        if fabs(kmVec3LengthSq(fallback) as libc::c_double) < 0.0001f64 {
            kmQuaternionRotationAxisAngle(pOut, fallback, 3.14159265358979323846f32);
        } else {
            let mut axis = kmVec3 { x: 0., y: 0., z: 0. };
            let mut X = kmVec3 { x: 0., y: 0., z: 0. };
            X.x = 1.0f64 as libc::c_float;
            X.y = 0.0f64 as libc::c_float;
            X.z = 0.0f64 as libc::c_float;
            kmVec3Cross(&mut axis, &mut X, vec1);
            if fabs(kmVec3LengthSq(&mut axis) as libc::c_double) < 0.0001f64 {
                let mut Y = kmVec3 { x: 0., y: 0., z: 0. };
                Y.x = 0.0f64 as libc::c_float;
                Y.y = 1.0f64 as libc::c_float;
                Y.z = 0.0f64 as libc::c_float;
                kmVec3Cross(&mut axis, &mut Y, vec1);
            }
            kmVec3Normalize(&mut axis, &mut axis);
            kmQuaternionRotationAxisAngle(pOut, &mut axis, 3.14159265358979323846f32);
        }
    } else {
        let mut s = sqrtf(
            (1 as libc::c_int as libc::c_float + a) * 2 as libc::c_int as libc::c_float,
        );
        let mut invs = 1 as libc::c_int as libc::c_float / s;
        let mut c = kmVec3 { x: 0., y: 0., z: 0. };
        kmVec3Cross(&mut c, &mut v1, &mut v2);
        (*pOut).x = c.x * invs;
        (*pOut).y = c.y * invs;
        (*pOut).z = c.z * invs;
        (*pOut).w = s * 0.5f32;
        kmQuaternionNormalize(pOut, pOut);
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionMultiplyVec3(
    mut pOut: *mut kmVec3,
    mut q: *const kmQuaternion,
    mut v: *const kmVec3,
) -> *mut kmVec3 {
    let mut uv = kmVec3 { x: 0., y: 0., z: 0. };
    let mut uuv = kmVec3 { x: 0., y: 0., z: 0. };
    let mut qvec = kmVec3 { x: 0., y: 0., z: 0. };
    qvec.x = (*q).x;
    qvec.y = (*q).y;
    qvec.z = (*q).z;
    kmVec3Cross(&mut uv, &mut qvec, v);
    kmVec3Cross(&mut uuv, &mut qvec, &mut uv);
    kmVec3Scale(&mut uv, &mut uv, 2.0f32 * (*q).w);
    kmVec3Scale(&mut uuv, &mut uuv, 2.0f32);
    kmVec3Add(pOut, v, &mut uv);
    kmVec3Add(pOut, pOut, &mut uuv);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetUpVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_POS_Y);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRightVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_POS_X);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3RH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_NEG_Z);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetForwardVec3LH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_POS_Z);
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetPitch(
    mut q: *const kmQuaternion,
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
    mut q: *const kmQuaternion,
) -> libc::c_float {
    let mut result = asin(
        (-(2 as libc::c_int) as libc::c_float * ((*q).x * (*q).z - (*q).w * (*q).y))
            as libc::c_double,
    ) as libc::c_float;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmQuaternionGetRoll(
    mut q: *const kmQuaternion,
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
    mut pOut: *mut kmQuaternion,
    mut direction: *const kmVec3,
    mut up: *const kmVec3,
) -> *mut kmQuaternion {
    let mut tmp = kmMat3 { mat: [0.; 9] };
    kmMat3LookAt(&mut tmp, &KM_VEC3_ZERO, direction, up);
    return kmQuaternionNormalize(pOut, kmQuaternionRotationMatrix(pOut, &mut tmp));
}
