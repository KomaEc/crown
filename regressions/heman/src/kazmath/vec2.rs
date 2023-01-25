use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    
    
    
    fn cosf(_: libc::c_float) -> libc::c_float;
    fn sinf(_: libc::c_float) -> libc::c_float;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor14 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor15 { dummy: () }
#[no_mangle]
pub static mut KM_VEC2_POS_Y: crate::src::kazmath::aabb2::kmVec2 = {
    let mut init = crate::src::kazmath::aabb2::kmVec2 {
        x: 0 as libc::c_int as libc::c_float,
        y: 1 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC2_NEG_Y: crate::src::kazmath::aabb2::kmVec2 = {
    let mut init = crate::src::kazmath::aabb2::kmVec2 {
        x: 0 as libc::c_int as libc::c_float,
        y: -(1 as libc::c_int) as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC2_NEG_X: crate::src::kazmath::aabb2::kmVec2 = {
    let mut init = crate::src::kazmath::aabb2::kmVec2 {
        x: -(1 as libc::c_int) as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC2_POS_X: crate::src::kazmath::aabb2::kmVec2 = {
    let mut init = crate::src::kazmath::aabb2::kmVec2 {
        x: 1 as libc::c_int as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC2_ZERO: crate::src::kazmath::aabb2::kmVec2 = {
    let mut init = crate::src::kazmath::aabb2::kmVec2 {
        x: 0 as libc::c_int as libc::c_float,
        y: 0 as libc::c_int as libc::c_float,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn kmVec2Fill(
    mut pOut: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut x: libc::c_float,
    mut y: libc::c_float,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut.as_deref_mut().unwrap()).x= x;
    (*pOut.as_deref_mut().unwrap()).y= y;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Length(mut pIn: *const crate::src::kazmath::aabb2::kmVec2) -> libc::c_float {
    return sqrtf(crate::src::kazmath::utility::kmSQR((*pIn).x) + crate::src::kazmath::utility::kmSQR((*pIn).y));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2LengthSq(mut pIn: *const crate::src::kazmath::aabb2::kmVec2) -> libc::c_float {
    return crate::src::kazmath::utility::kmSQR((*pIn).x) + crate::src::kazmath::utility::kmSQR((*pIn).y);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Lerp(
    mut pOut: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV1: *const crate::src::kazmath::aabb2::kmVec2,
    mut pV2: *const crate::src::kazmath::aabb2::kmVec2,
    mut t: libc::c_float,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut.as_deref_mut().unwrap()).x= (*pV1).x + t * ((*pV2).x - (*pV1).x);
    (*pOut.as_deref_mut().unwrap()).y= (*pV1).y + t * ((*pV2).y - (*pV1).y);
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Normalize(
    mut pOut: *mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    if (*pIn).x == 0. && (*pIn).y == 0. {
        return kmVec2Assign(pOut, pIn);
    }
    let mut l = 1.0f32 / kmVec2Length(pIn);
    let mut v = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    v.x= (*pIn).x * l;
    v.y= (*pIn).y * l;
    (*pOut).x= v.x;
    (*pOut).y= v.y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Add(
    mut pOut: *mut crate::src::kazmath::aabb2::kmVec2,
    mut pV1: *const crate::src::kazmath::aabb2::kmVec2,
    mut pV2: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut).x= (*pV1).x + (*pV2).x;
    (*pOut).y= (*pV1).y + (*pV2).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Dot(
    mut pV1: *const crate::src::kazmath::aabb2::kmVec2,
    mut pV2: *const crate::src::kazmath::aabb2::kmVec2,
) -> libc::c_float {
    return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Cross(
    mut pV1: *const crate::src::kazmath::aabb2::kmVec2,
    mut pV2: *const crate::src::kazmath::aabb2::kmVec2,
) -> libc::c_float {
    return (*pV1).x * (*pV2).y - (*pV1).y * (*pV2).x;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Subtract(
    mut pOut: *mut crate::src::kazmath::aabb2::kmVec2,
    mut pV1: *const crate::src::kazmath::aabb2::kmVec2,
    mut pV2: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut).x= (*pV1).x - (*pV2).x;
    (*pOut).y= (*pV1).y - (*pV2).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Mul(
    mut pOut: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV1: *const crate::src::kazmath::aabb2::kmVec2,
    mut pV2: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut.as_deref_mut().unwrap()).x= (*pV1).x * (*pV2).x;
    (*pOut.as_deref_mut().unwrap()).y= (*pV1).y * (*pV2).y;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Div(
    mut pOut: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV1: *const crate::src::kazmath::aabb2::kmVec2,
    mut pV2: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    if (*pV2).x != 0. && (*pV2).y != 0. {
        (*pOut.as_deref_mut().unwrap()).x= (*pV1).x / (*pV2).x;
        (*pOut.as_deref_mut().unwrap()).y= (*pV1).y / (*pV2).y;
    }
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Transform(
    mut pOut: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut pV: *const crate::src::kazmath::aabb2::kmVec2,
    mut pM: *const crate::src::kazmath::mat3::kmMat3,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    let mut v = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    v.x= (*pV).x * (*pM).mat[0 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[3 as libc::c_int as usize]
        + (*pM).mat[6 as libc::c_int as usize];
    v.y= (*pV).x * (*pM).mat[1 as libc::c_int as usize]
        + (*pV).y * (*pM).mat[4 as libc::c_int as usize]
        + (*pM).mat[7 as libc::c_int as usize];
    (*pOut.as_deref_mut().unwrap()).x= v.x;
    (*pOut.as_deref_mut().unwrap()).y= v.y;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2TransformCoord(
    mut pOut: *mut crate::src::kazmath::aabb2::kmVec2,
    mut pV: *const crate::src::kazmath::aabb2::kmVec2,
    mut pM: *const crate::src::kazmath::mat3::kmMat3,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    __assert_fail(
        b"0\0" as *const u8 as *const libc::c_char,
        b"../kazmath/vec2.c\0" as *const u8 as *const libc::c_char,
        134 as libc::c_int as libc::c_uint,
        b"kmVec2 *kmVec2TransformCoord(kmVec2 *, const kmVec2 *, const kmMat3 *)\0" as *const u8 as *const i8,
    );
    return 0 as *mut crate::src::kazmath::aabb2::kmVec2;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Scale(
    mut pOut: *mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: *const crate::src::kazmath::aabb2::kmVec2,
    mut s: libc::c_float,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    (*pOut).x= (*pIn).x * s;
    (*pOut).y= (*pIn).y * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2AreEqual(
    mut p1: *const crate::src::kazmath::aabb2::kmVec2,
    mut p2: *const crate::src::kazmath::aabb2::kmVec2,
) -> libc::c_int {
    return (((*p1).x as libc::c_double) < (*p2).x as libc::c_double + 0.0001f64
        && (*p1).x as libc::c_double > (*p2).x as libc::c_double - 0.0001f64
        && (((*p1).y as libc::c_double) < (*p2).y as libc::c_double + 0.0001f64
            && (*p1).y as libc::c_double > (*p2).y as libc::c_double - 0.0001f64))
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Assign(
    mut pOut: *mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    if pOut == pIn as *mut crate::src::kazmath::aabb2::kmVec2 {
        return pOut;
    }
    (*pOut).x= (*pIn).x;
    (*pOut).y= (*pIn).y;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2RotateBy(
    mut pOut: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut pIn: *const crate::src::kazmath::aabb2::kmVec2,
    mut degrees: libc::c_float,
    mut center: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let radians = crate::src::kazmath::utility::kmDegreesToRadians(degrees);
    let cs = cosf(radians);
    let sn = sinf(radians);
    (*pOut.as_deref_mut().unwrap()).x= (*pIn).x - (*center).x;
    (*pOut.as_deref_mut().unwrap()).y= (*pIn).y - (*center).y;
    x= (*pOut.as_deref().unwrap()).x * cs - (*pOut.as_deref().unwrap()).y * sn;
    y= (*pOut.as_deref().unwrap()).x * sn + (*pOut.as_deref().unwrap()).y * cs;
    (*pOut.as_deref_mut().unwrap()).x= x + (*center).x;
    (*pOut.as_deref_mut().unwrap()).y= y + (*center).y;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2DegreesBetween(
    mut v1: *const crate::src::kazmath::aabb2::kmVec2,
    mut v2: *const crate::src::kazmath::aabb2::kmVec2,
) -> libc::c_float {
    if kmVec2AreEqual(v1, v2) != 0 {
        return 0.0f64 as libc::c_float;
    }
    let mut t1 = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut t2 = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    kmVec2Normalize(core::ptr::addr_of_mut!(t1), v1);
    kmVec2Normalize(core::ptr::addr_of_mut!(t2), v2);
    let mut cross = kmVec2Cross(core::ptr::addr_of!(t1), core::ptr::addr_of!(t2));
    let mut dot = kmVec2Dot(core::ptr::addr_of!(t1), core::ptr::addr_of!(t2));
    if dot as libc::c_double > 1.0f64 {
        dot= 1.0f64 as libc::c_float;
    }
    if (dot as libc::c_double) < -1.0f64 {
        dot= -1.0f64 as libc::c_float;
    }
    return crate::src::kazmath::utility::kmRadiansToDegrees(
        atan2(cross as libc::c_double, dot as libc::c_double) as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2DistanceBetween(
    mut v1: *const crate::src::kazmath::aabb2::kmVec2,
    mut v2: *const crate::src::kazmath::aabb2::kmVec2,
) -> libc::c_float {
    let mut diff = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    kmVec2Subtract(core::ptr::addr_of_mut!(diff), v2, v1);
    return fabs(kmVec2Length(core::ptr::addr_of!(diff)) as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2MidPointBetween(
    mut pOut: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut v1: *const crate::src::kazmath::aabb2::kmVec2,
    mut v2: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    let mut sum = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    kmVec2Add(core::ptr::addr_of_mut!(sum), v1, v2);
    (*pOut.as_deref_mut().unwrap()).x= sum.x / 2.0f32;
    (*pOut.as_deref_mut().unwrap()).y= sum.y / 2.0f32;
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec2Reflect(
    mut pOut: *mut crate::src::kazmath::aabb2::kmVec2,
    mut pIn: *const crate::src::kazmath::aabb2::kmVec2,
    mut normal: *const crate::src::kazmath::aabb2::kmVec2,
) -> *mut crate::src::kazmath::aabb2::kmVec2 {
    let mut tmp = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    kmVec2Scale(core::ptr::addr_of_mut!(tmp), normal, 2.0f32 * kmVec2Dot(pIn, normal));
    kmVec2Subtract(pOut, pIn, core::ptr::addr_of!(tmp));
    return pOut;
}
