use ::libc;
extern "C" {
    
    
    
    
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmAABB2 {
    pub min: kmVec2,
    pub max: kmVec2,
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Initialize(
    mut pBox: Option<&mut kmAABB2>,
    mut centre: *const kmVec2,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut depth: libc::c_float,
) -> *mut kmAABB2 {
    if pBox.as_deref().is_none() {();
        return 0 as *mut kmAABB2;
    }
    let mut origin = kmVec2 { x: 0., y: 0. };
    let mut point = if !centre.is_null() { centre as *mut kmVec2 } else {(); core::ptr::addr_of_mut!(origin) };
    crate::src::kazmath::vec2::kmVec2Fill(Some(&mut origin), 0.0f32, 0.0f32);
    (*pBox.as_deref_mut().unwrap()).min.x= (*point).x - width / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).min.y= (*point).y - height / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).max.x= (*point).x + width / 2 as libc::c_int as libc::c_float;
    (*pBox.as_deref_mut().unwrap()).max.y= (*point).y + height / 2 as libc::c_int as libc::c_float;
    return pBox.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Sanitize(
    mut pOut: Option<&mut kmAABB2>,
    mut pIn: *const kmAABB2,
) -> *mut kmAABB2 {
    if (*pIn).min.x <= (*pIn).max.x {
        (*pOut.as_deref_mut().unwrap()).min.x= (*pIn).min.x;
        (*pOut.as_deref_mut().unwrap()).max.x= (*pIn).max.x;
    } else {
        (*pOut.as_deref_mut().unwrap()).min.x= (*pIn).max.x;
        (*pOut.as_deref_mut().unwrap()).max.x= (*pIn).min.x;
    }
    if (*pIn).min.y <= (*pIn).max.y {
        (*pOut.as_deref_mut().unwrap()).min.y= (*pIn).min.y;
        (*pOut.as_deref_mut().unwrap()).max.y= (*pIn).max.y;
    } else {
        (*pOut.as_deref_mut().unwrap()).min.y= (*pIn).max.y;
        (*pOut.as_deref_mut().unwrap()).max.y= (*pIn).min.y;
    }
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ContainsPoint(
    mut pBox: *const kmAABB2,
    mut pPoint: *const kmVec2,
) -> libc::c_int {
    if (*pPoint).x >= (*pBox).min.x && (*pPoint).x <= (*pBox).max.x
        && (*pPoint).y >= (*pBox).min.y && (*pPoint).y <= (*pBox).max.y
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Assign(
    mut pOut: *mut kmAABB2,
    mut pIn: *const kmAABB2,
) -> *mut kmAABB2 {
    crate::src::kazmath::vec2::kmVec2Assign(core::ptr::addr_of_mut!((*pOut).min), &(*pIn).min);
    crate::src::kazmath::vec2::kmVec2Assign(core::ptr::addr_of_mut!((*pOut).max), &(*pIn).max);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Translate(
    mut pOut: *mut kmAABB2,
    mut pIn: *const kmAABB2,
    mut translation: *const kmVec2,
) -> *mut kmAABB2 {
    crate::src::kazmath::vec2::kmVec2Add(core::ptr::addr_of_mut!((*pOut).min), &(*pIn).min, translation);
    crate::src::kazmath::vec2::kmVec2Add(core::ptr::addr_of_mut!((*pOut).max), &(*pIn).max, translation);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Scale(
    mut pOut: *mut kmAABB2,
    mut pIn: *const kmAABB2,
    mut s: libc::c_float,
) -> *mut kmAABB2 {
    crate::src::kazmath::vec2::kmVec2Scale(core::ptr::addr_of_mut!((*pOut).max), &(*pIn).max, s);
    crate::src::kazmath::vec2::kmVec2Scale(core::ptr::addr_of_mut!((*pOut).min), &(*pIn).min, s);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ScaleWithPivot(
    mut pOut: *mut kmAABB2,
    mut pIn: *const kmAABB2,
    mut pivot: *const kmVec2,
    mut s: libc::c_float,
) -> *mut kmAABB2 {
    let mut translate = kmVec2 { x: 0., y: 0. };
    translate.x= -(*pivot).x;
    translate.y= -(*pivot).y;
    kmAABB2Translate(pOut, pIn, core::ptr::addr_of!(translate));
    kmAABB2Scale(pOut, pIn, s);
    kmAABB2Translate(pOut, pIn, pivot);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ContainsAABB(
    mut container: *const kmAABB2,
    mut to_check: *const kmAABB2,
) -> libc::c_uint {
    let mut corners: [kmVec2; 4] = [kmVec2 { x: 0., y: 0. }; 4];
    crate::src::kazmath::vec2::kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(0 as libc::c_int as isize)),
        (*to_check).min.x,
        (*to_check).min.y,
    );
    crate::src::kazmath::vec2::kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(1 as libc::c_int as isize)),
        (*to_check).max.x,
        (*to_check).min.y,
    );
    crate::src::kazmath::vec2::kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(2 as libc::c_int as isize)),
        (*to_check).max.x,
        (*to_check).max.y,
    );
    crate::src::kazmath::vec2::kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(3 as libc::c_int as isize)),
        (*to_check).min.x,
        (*to_check).max.y,
    );
    let mut nContains = kmAABB2ContainsPoint(
        container,
        core::ptr::addr_of_mut!(*corners.as_mut_ptr().offset(0 as libc::c_int as isize)),
    )
        + kmAABB2ContainsPoint(
            container,
            core::ptr::addr_of_mut!(*corners.as_mut_ptr().offset(1 as libc::c_int as isize)),
        )
        + kmAABB2ContainsPoint(
            container,
            core::ptr::addr_of_mut!(*corners.as_mut_ptr().offset(2 as libc::c_int as isize)),
        )
        + kmAABB2ContainsPoint(
            container,
            core::ptr::addr_of_mut!(*corners.as_mut_ptr().offset(3 as libc::c_int as isize)),
        );
    if nContains == 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint
    } else if nContains < 4 as libc::c_int {
        return 1 as libc::c_int as libc::c_uint
    } else {
        return 2 as libc::c_int as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2DiameterX(mut aabb: *const kmAABB2) -> libc::c_float {
    return (*aabb).max.x - (*aabb).min.x;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2DiameterY(mut aabb: *const kmAABB2) -> libc::c_float {
    return (*aabb).max.y - (*aabb).min.y;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Centre(
    mut aabb: *const kmAABB2,
    mut pOut: *mut kmVec2,
) -> *mut kmVec2 {
    crate::src::kazmath::vec2::kmVec2Add(pOut, &(*aabb).min, &(*aabb).max);
    crate::src::kazmath::vec2::kmVec2Scale(pOut, pOut, 0.5f64 as libc::c_float);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ExpandToContain(
    mut pOut: Option<&mut kmAABB2>,
    mut pIn: *const kmAABB2,
    mut other: *const kmAABB2,
) -> *mut kmAABB2 {
    let mut result = kmAABB2 {
        min: kmVec2 { x: 0., y: 0. },
        max: kmVec2 { x: 0., y: 0. },
    };
    result.min.x= if (*pIn).min.x < (*other).min.x { (*pIn).min.x } else { (*other).min.x };
    result.max.x= if (*pIn).max.x > (*other).max.x { (*pIn).max.x } else { (*other).max.x };
    result.min.y= if (*pIn).min.y < (*other).min.y { (*pIn).min.y } else { (*other).min.y };
    result.max.y= if (*pIn).max.y > (*other).max.y { (*pIn).max.y } else { (*other).max.y };
    kmAABB2Assign(pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of!(result));
    return pOut.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
}
