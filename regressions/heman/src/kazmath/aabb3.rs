use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    
    
    
    
    
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
pub struct kmAABB3 {
    pub min: kmVec3,
    pub max: kmVec3,
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Initialize(
    mut pBox: Option<&mut kmAABB3>,
    mut centre: *const kmVec3,
    width: libc::c_float,
    height: libc::c_float,
    depth: libc::c_float,
) -> *mut kmAABB3 {
    if pBox.as_deref().is_none() {();
        return 0 as *mut kmAABB3;
    }
    let mut origin = kmVec3 { x: 0., y: 0., z: 0. };
    let mut point = if !centre.is_null() { centre as *mut kmVec3 } else {(); core::ptr::addr_of_mut!(origin) };
    crate::src::kazmath::vec3::kmVec3Zero(Some(&mut origin));
    (*pBox.as_deref_mut().unwrap()).min.x= (*point).x - width / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).min.y= (*point).y - height / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).min.z= (*point).z - depth / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).max.x= (*point).x + width / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).max.y= (*point).y + height / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).max.z= (*point).z + depth / 2 as libc::c_int as libc::c_float;
    return pBox.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ContainsPoint(
    mut pBox: *const kmAABB3,
    mut pPoint: *const kmVec3,
) -> libc::c_int {
    if (*pPoint).x >= (*pBox).min.x && (*pPoint).x <= (*pBox).max.x
        && (*pPoint).y >= (*pBox).min.y && (*pPoint).y <= (*pBox).max.y
        && (*pPoint).z >= (*pBox).min.z && (*pPoint).z <= (*pBox).max.z
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Assign(
    mut pOut: *mut kmAABB3,
    mut pIn: *const kmAABB3,
) -> *mut kmAABB3 {
    crate::src::kazmath::vec3::kmVec3Assign(core::ptr::addr_of_mut!((*pOut).min), &(*pIn).min);
    crate::src::kazmath::vec3::kmVec3Assign(core::ptr::addr_of_mut!((*pOut).max), &(*pIn).max);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Scale(
    mut pOut: *mut kmAABB3,
    mut pIn: *const kmAABB3,
    mut s: libc::c_float,
) -> *mut kmAABB3 {
    if 0 as libc::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const libc::c_char,
            b"../kazmath/aabb3.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            b"kmAABB3 *kmAABB3Scale(kmAABB3 *, const kmAABB3 *, float)\0" as *const u8 as *const i8,
        );
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3IntersectsTriangle(
    mut box_0: *mut kmAABB3,
    mut p1: *const kmVec3,
    mut p2: *const kmVec3,
    mut p3: *const kmVec3,
) -> libc::c_uchar {
    if 0 as libc::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const libc::c_char,
            b"../kazmath/aabb3.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            b"unsigned char kmAABB3IntersectsTriangle(kmAABB3 *, const kmVec3 *, const kmVec3 *, const kmVec3 *)\0" as *const u8 as *const i8,
        );
    }
    return 1 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3IntersectsAABB(
    mut box_0: *const kmAABB3,
    mut other: *const kmAABB3,
) -> libc::c_uchar {
    return (kmAABB3ContainsAABB(box_0, other) != 0 as libc::c_int as libc::c_uint)
        as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ContainsAABB(
    mut container: *const kmAABB3,
    mut to_check: *const kmAABB3,
) -> libc::c_uint {
    let mut corners: [kmVec3; 8] = [kmVec3 { x: 0., y: 0., z: 0. }; 8];
    let mut result = 2 as libc::c_int as libc::c_uint;
    let mut found = 0 as libc::c_int as libc::c_uchar;
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(0 as libc::c_int as isize)),
        (*to_check).min.x,
        (*to_check).min.y,
        (*to_check).min.z,
    );
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(1 as libc::c_int as isize)),
        (*to_check).max.x,
        (*to_check).min.y,
        (*to_check).min.z,
    );
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(2 as libc::c_int as isize)),
        (*to_check).max.x,
        (*to_check).max.y,
        (*to_check).min.z,
    );
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(3 as libc::c_int as isize)),
        (*to_check).min.x,
        (*to_check).max.y,
        (*to_check).min.z,
    );
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(4 as libc::c_int as isize)),
        (*to_check).min.x,
        (*to_check).min.y,
        (*to_check).max.z,
    );
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(5 as libc::c_int as isize)),
        (*to_check).max.x,
        (*to_check).min.y,
        (*to_check).max.z,
    );
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(6 as libc::c_int as isize)),
        (*to_check).max.x,
        (*to_check).max.y,
        (*to_check).max.z,
    );
    crate::src::kazmath::vec3::kmVec3Fill(
        Some(&mut *corners.as_mut_ptr().offset(7 as libc::c_int as isize)),
        (*to_check).min.x,
        (*to_check).max.y,
        (*to_check).max.z,
    );
    let mut i = 0 as libc::c_int as libc::c_uchar;
    while (i as libc::c_int) < 8 as libc::c_int {
        if kmAABB3ContainsPoint(container, core::ptr::addr_of_mut!(*corners.as_mut_ptr().offset(i as isize)))
            == 0
        {
            result= 1 as libc::c_int as libc::c_uint;
            if found != 0 {
                return result;
            }
        } else {
            found= 1 as libc::c_int as libc::c_uchar;
        }
        i= i.wrapping_add(1);
    }
    if found == 0 {
        result= 0 as libc::c_int as libc::c_uint;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterX(mut aabb: *const kmAABB3) -> libc::c_float {
    return fabs(((*aabb).max.x - (*aabb).min.x) as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterY(mut aabb: *const kmAABB3) -> libc::c_float {
    return fabs(((*aabb).max.y - (*aabb).min.y) as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3DiameterZ(mut aabb: *const kmAABB3) -> libc::c_float {
    return fabs(((*aabb).max.z - (*aabb).min.z) as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3Centre(
    mut aabb: *const kmAABB3,
    mut pOut: *mut kmVec3,
) -> *mut kmVec3 {
    crate::src::kazmath::vec3::kmVec3Add(pOut, &(*aabb).min, &(*aabb).max);
    crate::src::kazmath::vec3::kmVec3Scale(pOut, pOut, 0.5f64 as libc::c_float);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB3ExpandToContain(
    mut pOut: Option<&mut kmAABB3>,
    mut pIn: *const kmAABB3,
    mut other: *const kmAABB3,
) -> *mut kmAABB3 {
    let mut result = kmAABB3 {
        min: kmVec3 { x: 0., y: 0., z: 0. },
        max: kmVec3 { x: 0., y: 0., z: 0. },
    };
    result.min.x= if (*pIn).min.x < (*other).min.x { (*pIn).min.x } else { (*other).min.x };
    result.max.x= if (*pIn).max.x > (*other).max.x { (*pIn).max.x } else { (*other).max.x };
    result.min.y= if (*pIn).min.y < (*other).min.y { (*pIn).min.y } else { (*other).min.y };
    result.max.y= if (*pIn).max.y > (*other).max.y { (*pIn).max.y } else { (*other).max.y };
    result.min.z= if (*pIn).min.z < (*other).min.z { (*pIn).min.z } else { (*other).min.z };
    result.max.z= if (*pIn).max.z > (*other).max.z { (*pIn).max.z } else { (*other).max.z };
    kmAABB3Assign(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of!(result));
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
