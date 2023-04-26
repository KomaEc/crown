
extern "C" {
    fn atan2(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrtf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    
    
    
}
pub use crate::src::kazmath::ray3::kmRay3IntersectPlane;
pub use crate::src::kazmath::utility::kmDegreesToRadians;
pub use crate::src::kazmath::utility::kmRadiansToDegrees;
pub use crate::src::kazmath::utility::kmSQR;
pub use crate::src::kazmath::vec4::kmVec4Fill;
pub use crate::src::kazmath::vec4::kmVec4Transform;
// #[derive(Copy, Clone)]

pub type kmMat4 = crate::src::kazmath::mat3::kmMat4;
// #[derive(Copy, Clone)]

pub type kmVec4 = crate::src::kazmath::plane::kmVec4;
// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
// #[derive(Copy, Clone)]

pub type kmMat3 = crate::src::kazmath::mat3::kmMat3;
// #[derive(Copy, Clone)]

pub type kmPlane = crate::src::kazmath::mat4::kmPlane;
// #[derive(Copy, Clone)]

pub type kmRay3 = crate::src::kazmath::ray3::kmRay3;
#[no_mangle]
pub static mut KM_VEC3_POS_Z: crate::src::kazmath::aabb3::kmVec3 = crate::src::kazmath::aabb3::kmVec3::new(); unsafe fn laertes_init_KM_VEC3_POS_Z() {
KM_VEC3_POS_Z = {
    let mut init = kmVec3 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
        z: 1 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC3_NEG_Z: crate::src::kazmath::aabb3::kmVec3 = crate::src::kazmath::aabb3::kmVec3::new(); unsafe fn laertes_init_KM_VEC3_NEG_Z() {
KM_VEC3_NEG_Z = {
    let mut init = kmVec3 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
        z: -(1 as std::os::raw::c_int) as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC3_POS_Y: crate::src::kazmath::aabb3::kmVec3 = crate::src::kazmath::aabb3::kmVec3::new(); unsafe fn laertes_init_KM_VEC3_POS_Y() {
KM_VEC3_POS_Y = {
    let mut init = kmVec3 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: 1 as std::os::raw::c_int as std::os::raw::c_float,
        z: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC3_NEG_Y: crate::src::kazmath::aabb3::kmVec3 = crate::src::kazmath::aabb3::kmVec3::new(); unsafe fn laertes_init_KM_VEC3_NEG_Y() {
KM_VEC3_NEG_Y = {
    let mut init = kmVec3 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: -(1 as std::os::raw::c_int) as std::os::raw::c_float,
        z: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC3_NEG_X: crate::src::kazmath::aabb3::kmVec3 = crate::src::kazmath::aabb3::kmVec3::new(); unsafe fn laertes_init_KM_VEC3_NEG_X() {
KM_VEC3_NEG_X = {
    let mut init = kmVec3 {
        x: -(1 as std::os::raw::c_int) as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
        z: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC3_POS_X: crate::src::kazmath::aabb3::kmVec3 = crate::src::kazmath::aabb3::kmVec3::new(); unsafe fn laertes_init_KM_VEC3_POS_X() {
KM_VEC3_POS_X = {
    let mut init = kmVec3 {
        x: 1 as std::os::raw::c_int as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
        z: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub static mut KM_VEC3_ZERO: crate::src::kazmath::aabb3::kmVec3 = crate::src::kazmath::aabb3::kmVec3::new(); unsafe fn laertes_init_KM_VEC3_ZERO() {
KM_VEC3_ZERO = {
    let mut init = kmVec3 {
        x: 0 as std::os::raw::c_int as std::os::raw::c_float,
        y: 0 as std::os::raw::c_int as std::os::raw::c_float,
        z: 0 as std::os::raw::c_int as std::os::raw::c_float,
    };
    init
};}//;
#[no_mangle]
pub unsafe extern "C" fn kmVec3Fill<'a1, 'a2>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut x: std::os::raw::c_float,
    mut y: std::os::raw::c_float,
    mut z: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Length(mut pIn: * const crate::src::kazmath::aabb3::kmVec3) -> std::os::raw::c_float {
    return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z));
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3LengthSq<'a1>(mut pIn: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>) -> std::os::raw::c_float {
    return kmSQR((*(pIn).unwrap()).x) + kmSQR((*(pIn).unwrap()).y) + kmSQR((*(pIn).unwrap()).z);
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Lerp<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: * const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
    mut t: std::os::raw::c_float,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut).x = (*pV1).x + t * ((*((pV2).clone()).unwrap()).x - (*pV1).x);
    (*pOut).y = (*pV1).y + t * ((*((pV2).clone()).unwrap()).y - (*pV1).y);
    (*pOut).z = (*pV1).z + t * ((*((pV2).clone()).unwrap()).z - (*pV1).z);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Normalize(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. {
        return kmVec3Assign(pOut, pIn);
    }
    let mut l = 1.0f32 / kmVec3Length(pIn);
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v.x = (*pIn).x * l;
    v.y = (*pIn).y * l;
    v.z = (*pIn).z * l;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Cross<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
    mut pV2: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v.x = (*((pV1).clone()).unwrap()).y * (*pV2).z - (*((pV1).clone()).unwrap()).z * (*pV2).y;
    v.y = (*((pV1).clone()).unwrap()).z * (*pV2).x - (*((pV1).clone()).unwrap()).x * (*pV2).z;
    v.z = (*((pV1).clone()).unwrap()).x * (*pV2).y - (*((pV1).clone()).unwrap()).y * (*pV2).x;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Dot(
    mut pV1: * const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: * const crate::src::kazmath::aabb3::kmVec3,
) -> std::os::raw::c_float {
    return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y + (*pV1).z * (*pV2).z;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Add<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: * const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v.x = (*pV1).x + (*((pV2).clone()).unwrap()).x;
    v.y = (*pV1).y + (*((pV2).clone()).unwrap()).y;
    v.z = (*pV1).z + (*((pV2).clone()).unwrap()).z;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Subtract(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pV1: * const crate::src::kazmath::aabb3::kmVec3,
    mut pV2: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v.x = (*pV1).x - (*pV2).x;
    v.y = (*pV1).y - (*pV2).y;
    v.z = (*pV1).z - (*pV2).z;
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Mul<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV1: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut pV2: Option<&'a3 crate::src::kazmath::aabb3::kmVec3>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x * (*((pV2).clone()).unwrap()).x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y * (*((pV2).clone()).unwrap()).y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = (*((pV1).clone()).unwrap()).z * (*((pV2).clone()).unwrap()).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Div<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV1: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut pV2: Option<&'a3 crate::src::kazmath::aabb3::kmVec3>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    if (*((pV2).clone()).unwrap()).x != 0. && (*((pV2).clone()).unwrap()).y != 0. && (*((pV2).clone()).unwrap()).z != 0. {
        (*(borrow_mut(&mut pOut)).unwrap()).x = (*((pV1).clone()).unwrap()).x / (*((pV2).clone()).unwrap()).x;
        (*(borrow_mut(&mut pOut)).unwrap()).y = (*((pV1).clone()).unwrap()).y / (*((pV2).clone()).unwrap()).y;
        (*(borrow_mut(&mut pOut)).unwrap()).z = (*((pV1).clone()).unwrap()).z / (*((pV2).clone()).unwrap()).z;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3MultiplyMat3<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut pM: Option<&'a3 crate::src::kazmath::mat3::kmMat3>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v
        .x = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[3 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    v
        .y = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[7 as std::os::raw::c_int as usize];
    v
        .z = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).x = v.x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = v.y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3MultiplyMat4<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pV: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
    mut pM: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v
        .x = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize];
    v
        .y = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize];
    v
        .z = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize]
        + (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize];
    (*pOut).x = v.x;
    (*pOut).y = v.y;
    (*pOut).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Transform<'a1, 'a2>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pV: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
    mut pM: Option<&'a2 crate::src::kazmath::mat3::kmMat4>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    return kmVec3MultiplyMat4(pOut, (pV).clone(), (pM).clone());
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3InverseTransform<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pVect: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut pM: Option<&'a3 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    let mut v1 = kmVec3 { x: 0., y: 0., z: 0. };
    let mut v2 = kmVec3 { x: 0., y: 0., z: 0. };
    v1.x = (*((pVect).clone()).unwrap()).x - (*((pM).clone()).unwrap()).mat[12 as std::os::raw::c_int as usize];
    v1.y = (*((pVect).clone()).unwrap()).y - (*((pM).clone()).unwrap()).mat[13 as std::os::raw::c_int as usize];
    v1.z = (*((pVect).clone()).unwrap()).z - (*((pM).clone()).unwrap()).mat[14 as std::os::raw::c_int as usize];
    v2
        .x = v1.x * (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        + v1.y * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        + v1.z * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize];
    v2
        .y = v1.x * (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        + v1.y * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        + v1.z * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    v2
        .z = v1.x * (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize]
        + v1.y * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        + v1.z * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).x = v2.x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = v2.y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = v2.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3InverseTransformNormal<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pVect: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut pM: Option<&'a3 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v
        .x = (*((pVect).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        + (*((pVect).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        + (*((pVect).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize];
    v
        .y = (*((pVect).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        + (*((pVect).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        + (*((pVect).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize];
    v
        .z = (*((pVect).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize]
        + (*((pVect).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize]
        + (*((pVect).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).x = v.x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = v.y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3TransformCoord<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut pM: Option<&'a3 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    let mut v = kmVec4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    let mut inV = kmVec4 {
        x: 0.,
        y: 0.,
        z: 0.,
        w: 0.,
    };
    borrow(& kmVec4Fill(Some(&mut inV), (*(pV).unwrap()).x, (*(pV).unwrap()).y, (*(pV).unwrap()).z, 1.0f64 as std::os::raw::c_float));
    kmVec4Transform(&mut v, &mut inV, (pM).clone());
    (*(borrow_mut(&mut pOut)).unwrap()).x = v.x / v.w;
    (*(borrow_mut(&mut pOut)).unwrap()).y = v.y / v.w;
    (*(borrow_mut(&mut pOut)).unwrap()).z = v.z / v.w;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3TransformNormal<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pV: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut pM: Option<&'a3 crate::src::kazmath::mat3::kmMat4>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    let mut v = kmVec3 { x: 0., y: 0., z: 0. };
    v
        .x = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[0 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[4 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[8 as std::os::raw::c_int as usize];
    v
        .y = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[1 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[5 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[9 as std::os::raw::c_int as usize];
    v
        .z = (*((pV).clone()).unwrap()).x * (*((pM).clone()).unwrap()).mat[2 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).y * (*((pM).clone()).unwrap()).mat[6 as std::os::raw::c_int as usize]
        + (*((pV).clone()).unwrap()).z * (*((pM).clone()).unwrap()).mat[10 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap()).x = v.x;
    (*(borrow_mut(&mut pOut)).unwrap()).y = v.y;
    (*(borrow_mut(&mut pOut)).unwrap()).z = v.z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Scale(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: * const crate::src::kazmath::aabb3::kmVec3,
    s: std::os::raw::c_float,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    (*pOut).x = (*pIn).x * s;
    (*pOut).y = (*pIn).y * s;
    (*pOut).z = (*pIn).z * s;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3AreEqual<'a1, 'a2>(
    mut p1: Option<&'a1 crate::src::kazmath::aabb3::kmVec3>,
    mut p2: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
) -> std::os::raw::c_int {
    if ((*((p1).clone()).unwrap()).x as std::os::raw::c_double) < (*((p2).clone()).unwrap()).x as std::os::raw::c_double + 0.0001f64
        && (*((p1).clone()).unwrap()).x as std::os::raw::c_double > (*((p2).clone()).unwrap()).x as std::os::raw::c_double - 0.0001f64
        && (((*((p1).clone()).unwrap()).y as std::os::raw::c_double) < (*((p2).clone()).unwrap()).y as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).y as std::os::raw::c_double > (*((p2).clone()).unwrap()).y as std::os::raw::c_double - 0.0001f64)
        && (((*((p1).clone()).unwrap()).z as std::os::raw::c_double) < (*((p2).clone()).unwrap()).z as std::os::raw::c_double + 0.0001f64
            && (*((p1).clone()).unwrap()).z as std::os::raw::c_double > (*((p2).clone()).unwrap()).z as std::os::raw::c_double - 0.0001f64)
    {
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Assign(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    if pOut == pIn as *mut kmVec3 {
        return pOut;
    }
    (*pOut).x = (*pIn).x;
    (*pOut).y = (*pIn).y;
    (*pOut).z = (*pIn).z;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Zero<'a1, 'a2>(mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>) -> Option<&'a2 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a2 {
    (*(borrow_mut(&mut pOut)).unwrap()).x = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).y = 0.0f32;
    (*(borrow_mut(&mut pOut)).unwrap()).z = 0.0f32;
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3GetHorizontalAngle<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
) -> Option<&'a3 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a3 {
    let z1 = sqrt(((*((pIn).clone()).unwrap()).x * (*((pIn).clone()).unwrap()).x + (*((pIn).clone()).unwrap()).z * (*((pIn).clone()).unwrap()).z) as std::os::raw::c_double)
        as std::os::raw::c_float;
    (*(borrow_mut(&mut pOut)).unwrap())
        .y = kmRadiansToDegrees(
        atan2((*(pIn).unwrap()).x as std::os::raw::c_double, (*(pIn).unwrap()).z as std::os::raw::c_double) as std::os::raw::c_float,
    );
    if (*(borrow(& pOut)).unwrap()).y < 0 as std::os::raw::c_int as std::os::raw::c_float {
        (*(borrow_mut(&mut pOut)).unwrap()).y += 360 as std::os::raw::c_int as std::os::raw::c_float;
    }
    if (*(borrow(& pOut)).unwrap()).y >= 360 as std::os::raw::c_int as std::os::raw::c_float {
        (*(borrow_mut(&mut pOut)).unwrap()).y -= 360 as std::os::raw::c_int as std::os::raw::c_float;
    }
    (*(borrow_mut(&mut pOut)).unwrap())
        .x = (kmRadiansToDegrees(
        atan2(z1 as std::os::raw::c_double, (*(pIn).unwrap()).y as std::os::raw::c_double) as std::os::raw::c_float,
    ) as std::os::raw::c_double - 90.0f64) as std::os::raw::c_float;
    if (*(borrow(& pOut)).unwrap()).x < 0 as std::os::raw::c_int as std::os::raw::c_float {
        (*(borrow_mut(&mut pOut)).unwrap()).x += 360 as std::os::raw::c_int as std::os::raw::c_float;
    }
    if (*(borrow(& pOut)).unwrap()).x >= 360 as std::os::raw::c_int as std::os::raw::c_float {
        (*(borrow_mut(&mut pOut)).unwrap()).x -= 360 as std::os::raw::c_int as std::os::raw::c_float;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3RotationToDirection<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut forwards: Option<&'a3 crate::src::kazmath::aabb3::kmVec3>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmVec3> where 'a1: 'a4 {
    let xr = kmDegreesToRadians((*(pIn).unwrap()).x);
    let yr = kmDegreesToRadians((*(pIn).unwrap()).y);
    let zr = kmDegreesToRadians((*(pIn).unwrap()).z);
    let cr = cos(xr as std::os::raw::c_double) as std::os::raw::c_float;
    let sr = sin(xr as std::os::raw::c_double) as std::os::raw::c_float;
    let cp = cos(yr as std::os::raw::c_double) as std::os::raw::c_float;
    let sp = sin(yr as std::os::raw::c_double) as std::os::raw::c_float;
    let cy = cos(zr as std::os::raw::c_double) as std::os::raw::c_float;
    let sy = sin(zr as std::os::raw::c_double) as std::os::raw::c_float;
    let srsp = sr * sp;
    let crsp = cr * sp;
    let pseudoMatrix: [f32; 9] = [
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
    (*(borrow_mut(&mut pOut)).unwrap())
        .x = (*((forwards).clone()).unwrap()).x * pseudoMatrix[0 as std::os::raw::c_int as usize]
        + (*((forwards).clone()).unwrap()).y * pseudoMatrix[3 as std::os::raw::c_int as usize]
        + (*((forwards).clone()).unwrap()).z * pseudoMatrix[6 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .y = (*((forwards).clone()).unwrap()).x * pseudoMatrix[1 as std::os::raw::c_int as usize]
        + (*((forwards).clone()).unwrap()).y * pseudoMatrix[4 as std::os::raw::c_int as usize]
        + (*((forwards).clone()).unwrap()).z * pseudoMatrix[7 as std::os::raw::c_int as usize];
    (*(borrow_mut(&mut pOut)).unwrap())
        .z = (*((forwards).clone()).unwrap()).x * pseudoMatrix[2 as std::os::raw::c_int as usize]
        + (*((forwards).clone()).unwrap()).y * pseudoMatrix[5 as std::os::raw::c_int as usize]
        + (*((forwards).clone()).unwrap()).z * pseudoMatrix[8 as std::os::raw::c_int as usize];
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3ProjectOnToPlane<'a1>(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut point: * const crate::src::kazmath::aabb3::kmVec3,
    mut plane: Option<&'a1 crate::src::kazmath::mat4::kmPlane>,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut ray = kmRay3 {
        start: kmVec3 { x: 0., y: 0., z: 0. },
        dir: kmVec3 { x: 0., y: 0., z: 0. },
    };
    kmVec3Assign(&mut ray.start, point);
    ray.dir.x = -(*((plane).clone()).unwrap()).a;
    ray.dir.y = -(*((plane).clone()).unwrap()).b;
    ray.dir.z = -(*((plane).clone()).unwrap()).c;
    kmRay3IntersectPlane(pOut, Some(&mut ray), (plane).clone());
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmVec3Reflect(
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
    mut pIn: * const crate::src::kazmath::aabb3::kmVec3,
    mut normal: * const crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    let mut tmp = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Scale(&mut tmp, normal, 2.0f32 * kmVec3Dot(pIn, normal));
    kmVec3Subtract(pOut, pIn, &mut tmp);
    return pOut;
}
use crate::laertes_rt::*;
