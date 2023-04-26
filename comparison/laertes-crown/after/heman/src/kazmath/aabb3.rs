
extern "C" {
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    
    
    
    
    
}
pub use crate::src::kazmath::vec3::kmVec3Add;
pub use crate::src::kazmath::vec3::kmVec3Assign;
pub use crate::src::kazmath::vec3::kmVec3Fill;
pub use crate::src::kazmath::vec3::kmVec3Scale;
pub use crate::src::kazmath::vec3::kmVec3Zero;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
}
impl kmVec3 {
    pub const fn new() -> Self {
        kmVec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0
        }
    }
}

impl std::default::Default for kmVec3 {
    fn default() -> Self { kmVec3::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmAABB3 {
    pub min: crate::src::kazmath::aabb3::kmVec3,
    pub max: crate::src::kazmath::aabb3::kmVec3,
}
impl kmAABB3 {
    pub const fn new() -> Self {
        kmAABB3 {
        min: crate::src::kazmath::aabb3::kmVec3::new(),
        max: crate::src::kazmath::aabb3::kmVec3::new()
        }
    }
}

impl std::default::Default for kmAABB3 {
    fn default() -> Self { kmAABB3::new() }
}

#[no_mangle]
pub unsafe extern "C" fn kmAABB3Initialize<'a1, 'a2>(
    mut pBox: Option<&'a1 mut crate::src::kazmath::aabb3::kmAABB3>,
    mut centre: * const crate::src::kazmath::aabb3::kmVec3,
    width: std::os::raw::c_float,
    height: std::os::raw::c_float,
    depth: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::aabb3::kmAABB3> where 'a1: 'a2 {
    if borrow(& pBox).is_none() {
        return Option::<&'_ mut crate::src::kazmath::aabb3::kmAABB3>::None;
    }
    let mut origin = kmVec3 { x: 0., y: 0., z: 0. };
    let mut point = if !centre.is_null() { centre as *mut kmVec3 } else { &mut origin as * mut kmVec3 };
    borrow(& kmVec3Zero(Some(&mut origin)));
    (*(borrow_mut(&mut pBox)).unwrap()).min.x = (*point).x - width / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).min.y = (*point).y - height / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).min.z = (*point).z - depth / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).max.x = (*point).x + width / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).max.y = (*point).y + height / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).max.z = (*point).z + depth / 2 as std::os::raw::c_int as std::os::raw::c_float;
    return pBox;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ContainsPoint<'a1, 'a2>(
    mut pBox: Option<&'a1 crate::src::kazmath::aabb3::kmAABB3>,
    mut pPoint: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
) -> std::os::raw::c_int {
    if (*((pPoint).clone()).unwrap()).x >= (*((pBox).clone()).unwrap()).min.x && (*((pPoint).clone()).unwrap()).x <= (*((pBox).clone()).unwrap()).max.x
        && (*((pPoint).clone()).unwrap()).y >= (*((pBox).clone()).unwrap()).min.y && (*((pPoint).clone()).unwrap()).y <= (*((pBox).clone()).unwrap()).max.y
        && (*((pPoint).clone()).unwrap()).z >= (*((pBox).clone()).unwrap()).min.z && (*((pPoint).clone()).unwrap()).z <= (*((pBox).clone()).unwrap()).max.z
    {
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Assign<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmAABB3>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb3::kmAABB3>,
) -> Option<&'a3 mut crate::src::kazmath::aabb3::kmAABB3> where 'a1: 'a3 {
    kmVec3Assign(&mut (*(borrow_mut(&mut pOut)).unwrap()).min, &(*((pIn).clone()).unwrap()).min);
    kmVec3Assign(&mut (*(borrow_mut(&mut pOut)).unwrap()).max, &(*((pIn).clone()).unwrap()).max);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Scale<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmAABB3>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb3::kmAABB3>,
    mut s: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::kazmath::aabb3::kmAABB3> where 'a1: 'a3 {
    if 0 as std::os::raw::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const std::os::raw::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/aabb3.c\0" as *const u8 as *const std::os::raw::c_char,
            81 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 57], &'_ [i8; 57]>(b"kmAABB3 *kmAABB3Scale(kmAABB3 *, const kmAABB3 *, float)\0"))
                .as_ptr(),
        );
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3IntersectsTriangle<'a1, 'a2, 'a3, 'a4>(
    mut box_0: Option<&'a1 mut crate::src::kazmath::aabb3::kmAABB3>,
    mut p1: Option<&'a2 crate::src::kazmath::aabb3::kmVec3>,
    mut p2: Option<&'a3 crate::src::kazmath::aabb3::kmVec3>,
    mut p3: Option<&'a4 crate::src::kazmath::aabb3::kmVec3>,
) -> std::os::raw::c_uchar {
    if 0 as std::os::raw::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const std::os::raw::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/aabb3.c\0" as *const u8 as *const std::os::raw::c_char,
            86 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 99], &'_ [i8; 99]>(
                b"unsigned char kmAABB3IntersectsTriangle(kmAABB3 *, const kmVec3 *, const kmVec3 *, const kmVec3 *)\0",
            ))
                .as_ptr(),
        );
    }
    return 1 as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3IntersectsAABB<'a1, 'a2>(
    mut box_0: Option<&'a1 crate::src::kazmath::aabb3::kmAABB3>,
    mut other: Option<&'a2 crate::src::kazmath::aabb3::kmAABB3>,
) -> std::os::raw::c_uchar {
    return (kmAABB3ContainsAABB((box_0).clone(), (other).clone()) != 0 as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ContainsAABB<'a1, 'a2>(
    mut container: Option<&'a1 crate::src::kazmath::aabb3::kmAABB3>,
    mut to_check: Option<&'a2 crate::src::kazmath::aabb3::kmAABB3>,
) -> std::os::raw::c_uint {
    let mut corners: [crate::src::kazmath::aabb3::kmVec3; 8] = [kmVec3 { x: 0., y: 0., z: 0. }; 8];
    let mut result = 2 as std::os::raw::c_int as std::os::raw::c_uint;
    let mut found = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(0 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).min.x,
        (*(to_check).unwrap()).min.y,
        (*(to_check).unwrap()).min.z,
    ));
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(1 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).max.x,
        (*(to_check).unwrap()).min.y,
        (*(to_check).unwrap()).min.z,
    ));
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(2 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).max.x,
        (*(to_check).unwrap()).max.y,
        (*(to_check).unwrap()).min.z,
    ));
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(3 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).min.x,
        (*(to_check).unwrap()).max.y,
        (*(to_check).unwrap()).min.z,
    ));
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(4 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).min.x,
        (*(to_check).unwrap()).min.y,
        (*(to_check).unwrap()).max.z,
    ));
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(5 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).max.x,
        (*(to_check).unwrap()).min.y,
        (*(to_check).unwrap()).max.z,
    ));
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(6 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).max.x,
        (*(to_check).unwrap()).max.y,
        (*(to_check).unwrap()).max.z,
    ));
    borrow(& kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(7 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).min.x,
        (*(to_check).unwrap()).max.y,
        (*(to_check).unwrap()).max.z,
    ));
    let mut i = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    while (i as std::os::raw::c_int) < 8 as std::os::raw::c_int {
        if kmAABB3ContainsPoint((container).clone(), Some(&mut *corners.as_mut_ptr().offset(i as isize)))
            == 0
        {
            result = 1 as std::os::raw::c_int as std::os::raw::c_uint;
            if found != 0 {
                return result;
            }
        } else {
            found = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
        }
        i = i.wrapping_add(1);
    }
    if found == 0 {
        result = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterX<'a1>(mut aabb: Option<&'a1 crate::src::kazmath::aabb3::kmAABB3>) -> std::os::raw::c_float {
    return fabs(((*((aabb).clone()).unwrap()).max.x - (*((aabb).clone()).unwrap()).min.x) as std::os::raw::c_double) as std::os::raw::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterY<'a1>(mut aabb: Option<&'a1 crate::src::kazmath::aabb3::kmAABB3>) -> std::os::raw::c_float {
    return fabs(((*((aabb).clone()).unwrap()).max.y - (*((aabb).clone()).unwrap()).min.y) as std::os::raw::c_double) as std::os::raw::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterZ<'a1>(mut aabb: Option<&'a1 crate::src::kazmath::aabb3::kmAABB3>) -> std::os::raw::c_float {
    return fabs(((*((aabb).clone()).unwrap()).max.z - (*((aabb).clone()).unwrap()).min.z) as std::os::raw::c_double) as std::os::raw::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Centre<'a1>(
    mut aabb: Option<&'a1 crate::src::kazmath::aabb3::kmAABB3>,
    mut pOut: * mut crate::src::kazmath::aabb3::kmVec3,
) -> * mut crate::src::kazmath::aabb3::kmVec3 {
    kmVec3Add(pOut, &(*((aabb).clone()).unwrap()).min, (Some(&(*((aabb).clone()).unwrap()).max)).clone());
    kmVec3Scale(pOut, pOut, 0.5f64 as std::os::raw::c_float);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ExpandToContain<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb3::kmAABB3>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb3::kmAABB3>,
    mut other: Option<&'a3 crate::src::kazmath::aabb3::kmAABB3>,
) -> Option<&'a4 mut crate::src::kazmath::aabb3::kmAABB3> where 'a1: 'a4 {
    let mut result = kmAABB3 {
        min: kmVec3 { x: 0., y: 0., z: 0. },
        max: kmVec3 { x: 0., y: 0., z: 0. },
    };
    result
        .min
        .x = if (*((pIn).clone()).unwrap()).min.x < (*((other).clone()).unwrap()).min.x { (*(pIn).unwrap()).min.x } else { (*(other).unwrap()).min.x };
    result
        .max
        .x = if (*((pIn).clone()).unwrap()).max.x > (*((other).clone()).unwrap()).max.x { (*(pIn).unwrap()).max.x } else { (*(other).unwrap()).max.x };
    result
        .min
        .y = if (*((pIn).clone()).unwrap()).min.y < (*((other).clone()).unwrap()).min.y { (*(pIn).unwrap()).min.y } else { (*(other).unwrap()).min.y };
    result
        .max
        .y = if (*((pIn).clone()).unwrap()).max.y > (*((other).clone()).unwrap()).max.y { (*(pIn).unwrap()).max.y } else { (*(other).unwrap()).max.y };
    result
        .min
        .z = if (*((pIn).clone()).unwrap()).min.z < (*((other).clone()).unwrap()).min.z { (*(pIn).unwrap()).min.z } else { (*(other).unwrap()).min.z };
    result
        .max
        .z = if (*((pIn).clone()).unwrap()).max.z > (*((other).clone()).unwrap()).max.z { (*(pIn).unwrap()).max.z } else { (*(other).unwrap()).max.z };
    borrow(& kmAABB3Assign(borrow_mut(&mut pOut), Some(&mut result)));
    return pOut;
}
use crate::laertes_rt::*;
