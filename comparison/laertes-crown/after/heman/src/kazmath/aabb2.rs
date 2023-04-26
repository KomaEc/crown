
extern "C" {
    
    
    
    
}
pub use crate::src::kazmath::vec2::kmVec2Add;
pub use crate::src::kazmath::vec2::kmVec2Assign;
pub use crate::src::kazmath::vec2::kmVec2Fill;
pub use crate::src::kazmath::vec2::kmVec2Scale;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
}
impl kmVec2 {
    pub const fn new() -> Self {
        kmVec2 {
        x: 0.0,
        y: 0.0
        }
    }
}

impl std::default::Default for kmVec2 {
    fn default() -> Self { kmVec2::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmAABB2 {
    pub min: crate::src::kazmath::aabb2::kmVec2,
    pub max: crate::src::kazmath::aabb2::kmVec2,
}
impl kmAABB2 {
    pub const fn new() -> Self {
        kmAABB2 {
        min: crate::src::kazmath::aabb2::kmVec2::new(),
        max: crate::src::kazmath::aabb2::kmVec2::new()
        }
    }
}

impl std::default::Default for kmAABB2 {
    fn default() -> Self { kmAABB2::new() }
}

#[no_mangle]
pub unsafe extern "C" fn kmAABB2Initialize<'a1, 'a2>(
    mut pBox: Option<&'a1 mut crate::src::kazmath::aabb2::kmAABB2>,
    mut centre: * const crate::src::kazmath::aabb2::kmVec2,
    width: std::os::raw::c_float,
    height: std::os::raw::c_float,
    depth: std::os::raw::c_float,
) -> Option<&'a2 mut crate::src::kazmath::aabb2::kmAABB2> where 'a1: 'a2 {
    if borrow(& pBox).is_none() {
        return Option::<&'_ mut crate::src::kazmath::aabb2::kmAABB2>::None;
    }
    let mut origin = kmVec2 { x: 0., y: 0. };
    let mut point = if !centre.is_null() { centre as *mut kmVec2 } else { &mut origin as * mut kmVec2 };
    borrow(& kmVec2Fill(Some(&mut origin), 0.0f32, 0.0f32));
    (*(borrow_mut(&mut pBox)).unwrap()).min.x = (*point).x - width / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).min.y = (*point).y - height / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).max.x = (*point).x + width / 2 as std::os::raw::c_int as std::os::raw::c_float;
    (*(borrow_mut(&mut pBox)).unwrap()).max.y = (*point).y + height / 2 as std::os::raw::c_int as std::os::raw::c_float;
    return pBox;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Sanitize<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmAABB2>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb2::kmAABB2>,
) -> Option<&'a3 mut crate::src::kazmath::aabb2::kmAABB2> where 'a1: 'a3 {
    if (*((pIn).clone()).unwrap()).min.x <= (*((pIn).clone()).unwrap()).max.x {
        (*(borrow_mut(&mut pOut)).unwrap()).min.x = (*(pIn).unwrap()).min.x;
        (*(borrow_mut(&mut pOut)).unwrap()).max.x = (*(pIn).unwrap()).max.x;
    } else {
        (*(borrow_mut(&mut pOut)).unwrap()).min.x = (*(pIn).unwrap()).max.x;
        (*(borrow_mut(&mut pOut)).unwrap()).max.x = (*(pIn).unwrap()).min.x;
    }
    if (*((pIn).clone()).unwrap()).min.y <= (*((pIn).clone()).unwrap()).max.y {
        (*(borrow_mut(&mut pOut)).unwrap()).min.y = (*(pIn).unwrap()).min.y;
        (*(borrow_mut(&mut pOut)).unwrap()).max.y = (*(pIn).unwrap()).max.y;
    } else {
        (*(borrow_mut(&mut pOut)).unwrap()).min.y = (*(pIn).unwrap()).max.y;
        (*(borrow_mut(&mut pOut)).unwrap()).max.y = (*(pIn).unwrap()).min.y;
    }
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ContainsPoint<'a1, 'a2>(
    mut pBox: Option<&'a1 crate::src::kazmath::aabb2::kmAABB2>,
    mut pPoint: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_int {
    if (*((pPoint).clone()).unwrap()).x >= (*((pBox).clone()).unwrap()).min.x && (*((pPoint).clone()).unwrap()).x <= (*((pBox).clone()).unwrap()).max.x
        && (*((pPoint).clone()).unwrap()).y >= (*((pBox).clone()).unwrap()).min.y && (*((pPoint).clone()).unwrap()).y <= (*((pBox).clone()).unwrap()).max.y
    {
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Assign<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmAABB2>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb2::kmAABB2>,
) -> Option<&'a3 mut crate::src::kazmath::aabb2::kmAABB2> where 'a1: 'a3 {
    kmVec2Assign(&mut (*(borrow_mut(&mut pOut)).unwrap()).min, &(*((pIn).clone()).unwrap()).min);
    kmVec2Assign(&mut (*(borrow_mut(&mut pOut)).unwrap()).max, &(*((pIn).clone()).unwrap()).max);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Translate<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmAABB2>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb2::kmAABB2>,
    mut translation: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmAABB2> where 'a1: 'a4 {
    kmVec2Add(&mut (*(borrow_mut(&mut pOut)).unwrap()).min, (Some(&(*((pIn).clone()).unwrap()).min)).clone(), (translation).clone());
    kmVec2Add(&mut (*(borrow_mut(&mut pOut)).unwrap()).max, (Some(&(*((pIn).clone()).unwrap()).max)).clone(), (translation).clone());
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Scale<'a1, 'a2, 'a3>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmAABB2>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb2::kmAABB2>,
    mut s: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::kazmath::aabb2::kmAABB2> where 'a1: 'a3 {
    kmVec2Scale(&mut (*(borrow_mut(&mut pOut)).unwrap()).max, &(*((pIn).clone()).unwrap()).max, s);
    kmVec2Scale(&mut (*(borrow_mut(&mut pOut)).unwrap()).min, &(*((pIn).clone()).unwrap()).min, s);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ScaleWithPivot<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmAABB2>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb2::kmAABB2>,
    mut pivot: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
    mut s: std::os::raw::c_float,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmAABB2> where 'a1: 'a4 {
    let mut translate = kmVec2 { x: 0., y: 0. };
    translate.x = -(*((pivot).clone()).unwrap()).x;
    translate.y = -(*((pivot).clone()).unwrap()).y;
    borrow(& kmAABB2Translate(borrow_mut(&mut pOut), (pIn).clone(), Some(&mut translate)));
    borrow(& kmAABB2Scale(borrow_mut(&mut pOut), (pIn).clone(), s));
    borrow(& kmAABB2Translate(borrow_mut(&mut pOut), (pIn).clone(), (pivot).clone()));
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ContainsAABB<'a1, 'a2>(
    mut container: Option<&'a1 crate::src::kazmath::aabb2::kmAABB2>,
    mut to_check: Option<&'a2 crate::src::kazmath::aabb2::kmAABB2>,
) -> std::os::raw::c_uint {
    let mut corners: [crate::src::kazmath::aabb2::kmVec2; 4] = [kmVec2 { x: 0., y: 0. }; 4];
    borrow(& kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(0 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).min.x,
        (*(to_check).unwrap()).min.y,
    ));
    borrow(& kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(1 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).max.x,
        (*(to_check).unwrap()).min.y,
    ));
    borrow(& kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(2 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).max.x,
        (*(to_check).unwrap()).max.y,
    ));
    borrow(& kmVec2Fill(
        Some(&mut *corners.as_mut_ptr().offset(3 as std::os::raw::c_int as isize)),
        (*(to_check).unwrap()).min.x,
        (*(to_check).unwrap()).max.y,
    ));
    let mut nContains = kmAABB2ContainsPoint(
        (container).clone(),
        Some(&mut *corners.as_mut_ptr().offset(0 as std::os::raw::c_int as isize)),
    )
        + kmAABB2ContainsPoint(
            (container).clone(),
            Some(&mut *corners.as_mut_ptr().offset(1 as std::os::raw::c_int as isize)),
        )
        + kmAABB2ContainsPoint(
            (container).clone(),
            Some(&mut *corners.as_mut_ptr().offset(2 as std::os::raw::c_int as isize)),
        )
        + kmAABB2ContainsPoint(
            (container).clone(),
            Some(&mut *corners.as_mut_ptr().offset(3 as std::os::raw::c_int as isize)),
        );
    if nContains == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as std::os::raw::c_uint
    } else if nContains < 4 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int as std::os::raw::c_uint
    } else {
        return 2 as std::os::raw::c_int as std::os::raw::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2DiameterX<'a1>(mut aabb: Option<&'a1 crate::src::kazmath::aabb2::kmAABB2>) -> std::os::raw::c_float {
    return (*((aabb).clone()).unwrap()).max.x - (*((aabb).clone()).unwrap()).min.x;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2DiameterY<'a1>(mut aabb: Option<&'a1 crate::src::kazmath::aabb2::kmAABB2>) -> std::os::raw::c_float {
    return (*((aabb).clone()).unwrap()).max.y - (*((aabb).clone()).unwrap()).min.y;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2Centre<'a1>(
    mut aabb: Option<&'a1 crate::src::kazmath::aabb2::kmAABB2>,
    mut pOut: * mut crate::src::kazmath::aabb2::kmVec2,
) -> * mut crate::src::kazmath::aabb2::kmVec2 {
    kmVec2Add(pOut, (Some(&(*((aabb).clone()).unwrap()).min)).clone(), (Some(&(*((aabb).clone()).unwrap()).max)).clone());
    kmVec2Scale(pOut, pOut, 0.5f64 as std::os::raw::c_float);
    return pOut;
}
#[no_mangle]
pub unsafe extern "C" fn kmAABB2ExpandToContain<'a1, 'a2, 'a3, 'a4>(
    mut pOut: Option<&'a1 mut crate::src::kazmath::aabb2::kmAABB2>,
    mut pIn: Option<&'a2 crate::src::kazmath::aabb2::kmAABB2>,
    mut other: Option<&'a3 crate::src::kazmath::aabb2::kmAABB2>,
) -> Option<&'a4 mut crate::src::kazmath::aabb2::kmAABB2> where 'a1: 'a4 {
    let mut result = kmAABB2 {
        min: kmVec2 { x: 0., y: 0. },
        max: kmVec2 { x: 0., y: 0. },
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
    borrow(& kmAABB2Assign(borrow_mut(&mut pOut), Some(&mut result)));
    return pOut;
}
use crate::laertes_rt::*;
