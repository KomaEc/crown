use ::libc;
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    
    
    
    
    
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor16 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor17 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor18 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor19 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor20 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor21 { dummy: () }
#[no_mangle]
pub static mut KM_VEC3_POS_Z: crate::src::kazmath::aabb3::kmVec3 = {
    let mut init = crate::src::kazmath::aabb3::kmVec3 {
        x: 0 as libc::c_int as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
        z: 1 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_NEG_Z: crate::src::kazmath::aabb3::kmVec3 = {
    let mut init = crate::src::kazmath::aabb3::kmVec3 {
        x: 0 as libc::c_int as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
        z: -(1 as libc::c_int) as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_POS_Y: crate::src::kazmath::aabb3::kmVec3 = {
    let mut init = crate::src::kazmath::aabb3::kmVec3 {
        x: 0 as libc::c_int as libc::c_float,
        y: 1 as libc::c_int as libc::c_float,
        z: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_NEG_Y: crate::src::kazmath::aabb3::kmVec3 = {
    let mut init = crate::src::kazmath::aabb3::kmVec3 {
        x: 0 as libc::c_int as libc::c_float,
        y: -(1 as libc::c_int) as libc::c_float,
        z: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_NEG_X: crate::src::kazmath::aabb3::kmVec3 = {
    let mut init = crate::src::kazmath::aabb3::kmVec3 {
        x: -(1 as libc::c_int) as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
        z: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_POS_X: crate::src::kazmath::aabb3::kmVec3 = {
    let mut init = crate::src::kazmath::aabb3::kmVec3 {
        x: 1 as libc::c_int as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
        z: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_ZERO: crate::src::kazmath::aabb3::kmVec3 = {
    let mut init = crate::src::kazmath::aabb3::kmVec3 {
        x: 0 as libc::c_int as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
        z: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn kmVec3Fill(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut.as_deref_mut().unwrap()).x= x;
    (*pOut.as_deref_mut().unwrap()).y= y;
    (*pOut.as_deref_mut().unwrap()).z= z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Length(mut pIn: *const crate::src::kazmath::aabb3::kmVec3) -> libc::c_float {
    return sqrtf(crate::src::kazmath::utility::kmSQR((*pIn).x) + crate::src::kazmath::utility::kmSQR((*pIn).y) + crate::src::kazmath::utility::kmSQR((*pIn).z));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3LengthSq(mut pIn: *const crate::src::kazmath::aabb3::kmVec3) -> libc::c_float {
    return crate::src::kazmath::utility::kmSQR((*pIn).x) + crate::src::kazmath::utility::kmSQR((*pIn).y) + crate::src::kazmath::utility::kmSQR((*pIn).z);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Lerp(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
    mut t: libc::c_float,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut).x= (*pV1).x + t * ((*pV2).x - (*pV1).x);
    (*pOut).y= (*pV1).y + t * ((*pV2).y - (*pV1).y);
    (*pOut).z= (*pV1).z + t * ((*pV2).z - (*pV1).z);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Normalize(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. {
        return kmVec3Assign(pOut, pIn);
    }
    let mut l = 1.0f32 / kmVec3Length(pIn);
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pIn).x * l;
    v.y= (*pIn).y * l;
    v.z= (*pIn).z * l;
    (*pOut).x= v.x;
    (*pOut).y= v.y;
    (*pOut).z= v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Cross(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pV1).y * (*pV2).z - (*pV1).z * (*pV2).y;
    v.y= (*pV1).z * (*pV2).x - (*pV1).x * (*pV2).z;
    v.z= (*pV1).x * (*pV2).y - (*pV1).y * (*pV2).x;
    (*pOut).x= v.x;
    (*pOut).y= v.y;
    (*pOut).z= v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Dot(
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
) -> libc::c_float {
    return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y + (*pV1).z * (*pV2).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Add(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pV1).x + (*pV2).x;
    v.y= (*pV1).y + (*pV2).y;
    v.z= (*pV1).z + (*pV2).z;
    (*pOut).x= v.x;
    (*pOut).y= v.y;
    (*pOut).z= v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Subtract(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pV1).x - (*pV2).x;
    v.y= (*pV1).y - (*pV2).y;
    v.z= (*pV1).z - (*pV2).z;
    (*pOut).x= v.x;
    (*pOut).y= v.y;
    (*pOut).z= v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Mul(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut.as_deref_mut().unwrap()).x= (*pV1).x * (*pV2).x;
    (*pOut.as_deref_mut().unwrap()).y= (*pV1).y * (*pV2).y;
    (*pOut.as_deref_mut().unwrap()).z= (*pV1).z * (*pV2).z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Div(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV1: *const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    if (*pV2).x != 0. && (*pV2).y != 0. && (*pV2).z != 0. {
        (*pOut.as_deref_mut().unwrap()).x= (*pV1).x / (*pV2).x;
        (*pOut.as_deref_mut().unwrap()).y= (*pV1).y / (*pV2).y;
        (*pOut.as_deref_mut().unwrap()).z= (*pV1).z / (*pV2).z;
    }
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3MultiplyMat3(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
    mut pM: *const crate::src::kazmath::mat3::kmMat3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pV).x * (*pM).mat[0 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[3 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[6 as libc::c_int as usize];
    v.y= (*pV).x * (*pM).mat[1 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[4 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[7 as libc::c_int as usize];
    v.z= (*pV).x * (*pM).mat[2 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[5 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[8 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).x= v.x;
    (*pOut.as_deref_mut().unwrap()).y= v.y;
    (*pOut.as_deref_mut().unwrap()).z= v.z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3MultiplyMat4(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
    mut pM: *const crate::src::kazmath::mat3::kmMat4,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pV).x * (*pM).mat[0 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[4 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[8 as libc::c_int as usize]
        + (*pM).mat[12 as libc::c_int as usize];
    v.y= (*pV).x * (*pM).mat[1 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[5 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[9 as libc::c_int as usize]
        + (*pM).mat[13 as libc::c_int as usize];
    v.z= (*pV).x * (*pM).mat[2 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[6 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[10 as libc::c_int as usize]
        + (*pM).mat[14 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).x= v.x;
    (*pOut.as_deref_mut().unwrap()).y= v.y;
    (*pOut.as_deref_mut().unwrap()).z= v.z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Transform(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
    mut pM: *const crate::src::kazmath::mat3::kmMat4,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    return kmVec3MultiplyMat4(pOut.as_deref_mut(), pV, pM);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3InverseTransform(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pVect: *const crate::src::kazmath::aabb3::kmVec3,
    mut pM: *const crate::src::kazmath::mat3::kmMat4,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v1 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    let mut v2 = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v1.x= (*pVect).x - (*pM).mat[12 as libc::c_int as usize];
    v1.y= (*pVect).y - (*pM).mat[13 as libc::c_int as usize];
    v1.z= (*pVect).z - (*pM).mat[14 as libc::c_int as usize];
    v2.x= v1.x * (*pM).mat[0 as libc::c_int as usize]
        + v1.y * (*pM).mat[1 as libc::c_int as usize]
        + v1.z * (*pM).mat[2 as libc::c_int as usize];
    v2.y= v1.x * (*pM).mat[4 as libc::c_int as usize]
        + v1.y * (*pM).mat[5 as libc::c_int as usize]
        + v1.z * (*pM).mat[6 as libc::c_int as usize];
    v2.z= v1.x * (*pM).mat[8 as libc::c_int as usize]
        + v1.y * (*pM).mat[9 as libc::c_int as usize]
        + v1.z * (*pM).mat[10 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).x= v2.x;
    (*pOut.as_deref_mut().unwrap()).y= v2.y;
    (*pOut.as_deref_mut().unwrap()).z= v2.z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3InverseTransformNormal(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pVect: *const crate::src::kazmath::aabb3::kmVec3,
    mut pM: *const crate::src::kazmath::mat3::kmMat4,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pVect).x * (*pM).mat[0 as libc::c_int as usize]
        + (*pVect).y * (*pM).mat[1 as libc::c_int as usize]
        + (*pVect).z * (*pM).mat[2 as libc::c_int as usize];
    v.y= (*pVect).x * (*pM).mat[4 as libc::c_int as usize]
        + (*pVect).y * (*pM).mat[5 as libc::c_int as usize]
        + (*pVect).z * (*pM).mat[6 as libc::c_int as usize];
    v.z= (*pVect).x * (*pM).mat[8 as libc::c_int as usize]
        + (*pVect).y * (*pM).mat[9 as libc::c_int as usize]
        + (*pVect).z * (*pM).mat[10 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).x= v.x;
    (*pOut.as_deref_mut().unwrap()).y= v.y;
    (*pOut.as_deref_mut().unwrap()).z= v.z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3TransformCoord(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
    mut pM: *const crate::src::kazmath::mat3::kmMat4,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::plane::kmVec4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut inV = crate::src::kazmath::plane::kmVec4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    crate::src::kazmath::vec4::kmVec4Fill(Some(&mut inV), (*pV).x, (*pV).y, (*pV).z, 1.0f64 as libc::c_float);
    crate::src::kazmath::vec4::kmVec4Transform(Some(&mut v), core::ptr::addr_of!(inV), pM);
    (*pOut.as_deref_mut().unwrap()).x= v.x / v.w;
    (*pOut.as_deref_mut().unwrap()).y= v.y / v.w;
    (*pOut.as_deref_mut().unwrap()).z= v.z / v.w;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3TransformNormal(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: *const crate::src::kazmath::aabb3::kmVec3,
    mut pM: *const crate::src::kazmath::mat3::kmMat4,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    v.x= (*pV).x * (*pM).mat[0 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[4 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[8 as libc::c_int as usize];
    v.y= (*pV).x * (*pM).mat[1 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[5 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[9 as libc::c_int as usize];
    v.z= (*pV).x * (*pM).mat[2 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[6 as libc::c_int as usize]
        + (*pV).z * (*pM).mat[10 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).x= v.x;
    (*pOut.as_deref_mut().unwrap()).y= v.y;
    (*pOut.as_deref_mut().unwrap()).z= v.z;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Scale(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: *const crate::src::kazmath::aabb3::kmVec3,
    s: libc::c_float,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut).x= (*pIn).x * s;
    (*pOut).y= (*pIn).y * s;
    (*pOut).z= (*pIn).z * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3AreEqual(
    mut p1: *const crate::src::kazmath::aabb3::kmVec3,
    mut p2: *const crate::src::kazmath::aabb3::kmVec3,
) -> libc::c_int {
    if ((*p1).x as libc::c_double) < (*p2).x as libc::c_double + 0.0001f64
        && (*p1).x as libc::c_double > (*p2).x as libc::c_double - 0.0001f64
        && (((*p1).y as libc::c_double) < (*p2).y as libc::c_double + 0.0001f64
            && (*p1).y as libc::c_double > (*p2).y as libc::c_double - 0.0001f64)
        && (((*p1).z as libc::c_double) < (*p2).z as libc::c_double + 0.0001f64
            && (*p1).z as libc::c_double > (*p2).z as libc::c_double - 0.0001f64)
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Assign(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    if pOut == pIn as *mut crate::src::kazmath::aabb3::kmVec3 {
        return pOut;
    }
    (*pOut).x= (*pIn).x;
    (*pOut).y= (*pIn).y;
    (*pOut).z= (*pIn).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Zero(mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut.as_deref_mut().unwrap()).x= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).y= 0.0f32;
    (*pOut.as_deref_mut().unwrap()).z= 0.0f32;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3GetHorizontalAngle(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let z1 = sqrt(((*pIn).x * (*pIn).x + (*pIn).z * (*pIn).z) as libc::c_double)
        as libc::c_float;
    (*pOut.as_deref_mut().unwrap()).y= crate::src::kazmath::utility::kmRadiansToDegrees(
        atan2((*pIn).x as libc::c_double, (*pIn).z as libc::c_double) as libc::c_float,
    );
    if (*pOut.as_deref().unwrap()).y < 0 as libc::c_int as libc::c_float {
        (*pOut.as_deref_mut().unwrap()).y+= 360 as libc::c_int as libc::c_float;
    }
    if (*pOut.as_deref().unwrap()).y >= 360 as libc::c_int as libc::c_float {
        (*pOut.as_deref_mut().unwrap()).y-= 360 as libc::c_int as libc::c_float;
    }
    (*pOut.as_deref_mut().unwrap()).x= (crate::src::kazmath::utility::kmRadiansToDegrees(
        atan2(z1 as libc::c_double, (*pIn).y as libc::c_double) as libc::c_float,
    ) as libc::c_double - 90.0f64) as libc::c_float;
    if (*pOut.as_deref().unwrap()).x < 0 as libc::c_int as libc::c_float {
        (*pOut.as_deref_mut().unwrap()).x+= 360 as libc::c_int as libc::c_float;
    }
    if (*pOut.as_deref().unwrap()).x >= 360 as libc::c_int as libc::c_float {
        (*pOut.as_deref_mut().unwrap()).x-= 360 as libc::c_int as libc::c_float;
    }
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3RotationToDirection(
    mut pOut: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: *const crate::src::kazmath::aabb3::kmVec3,
    mut forwards: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let xr = crate::src::kazmath::utility::kmDegreesToRadians((*pIn).x);
    let yr = crate::src::kazmath::utility::kmDegreesToRadians((*pIn).y);
    let zr = crate::src::kazmath::utility::kmDegreesToRadians((*pIn).z);
    let cr = cos(xr as libc::c_double) as libc::c_float;
    let sr = sin(xr as libc::c_double) as libc::c_float;
    let cp = cos(yr as libc::c_double) as libc::c_float;
    let sp = sin(yr as libc::c_double) as libc::c_float;
    let cy = cos(zr as libc::c_double) as libc::c_float;
    let sy = sin(zr as libc::c_double) as libc::c_float;
    let srsp = sr * sp;
    let crsp = cr * sp;
    let pseudoMatrix: [libc::c_float; 9] = [
        cp * cy,
        cp * sy,
        -sp,
        srsp * cy - cr * sy,
        srsp * sy + cr * cy,
        sr * cp,
        crsp * cy + sr * sy,
        crsp * sy - sr * cy,
        cr * cp,
    ];
    (*pOut.as_deref_mut().unwrap()).x= (*forwards).x * pseudoMatrix[0 as libc::c_int as usize]
        + (*forwards).y * pseudoMatrix[3 as libc::c_int as usize]
        + (*forwards).z * pseudoMatrix[6 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).y= (*forwards).x * pseudoMatrix[1 as libc::c_int as usize]
        + (*forwards).y * pseudoMatrix[4 as libc::c_int as usize]
        + (*forwards).z * pseudoMatrix[7 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).z= (*forwards).x * pseudoMatrix[2 as libc::c_int as usize]
        + (*forwards).y * pseudoMatrix[5 as libc::c_int as usize]
        + (*forwards).z * pseudoMatrix[8 as libc::c_int as usize];
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3ProjectOnToPlane(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut point: *const crate::src::kazmath::aabb3::kmVec3,
    mut plane: *const crate::src::kazmath::mat4::kmPlane,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut ray = crate::src::kazmath::ray3::kmRay3 {
        start: crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. },
        dir: crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. },
    };
    kmVec3Assign(core::ptr::addr_of_mut!(ray.start), point);
    ray.dir.x= -(*plane).a;
    ray.dir.y= -(*plane).b;
    ray.dir.z= -(*plane).c;
    crate::src::kazmath::ray3::kmRay3IntersectPlane(pOut, core::ptr::addr_of!(ray), plane);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Reflect(
    mut pOut: *mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: *const crate::src::kazmath::aabb3::kmVec3,
    mut normal: *const crate::src::kazmath::aabb3::kmVec3,
) -> *mut crate::src::kazmath::aabb3::kmVec3 {
    let mut tmp = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Scale(core::ptr::addr_of_mut!(tmp), normal, 2.0f32 * kmVec3Dot(pIn, normal));
    kmVec3Subtract(pOut, pIn, core::ptr::addr_of!(tmp));
    return pOut;
}
