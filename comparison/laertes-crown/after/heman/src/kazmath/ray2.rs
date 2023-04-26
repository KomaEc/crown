
extern "C" {
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    
    
    
    
    
}
pub use crate::src::kazmath::vec2::kmVec2Assign;
pub use crate::src::kazmath::vec2::kmVec2Dot;
pub use crate::src::kazmath::vec2::kmVec2Length;
pub use crate::src::kazmath::vec2::kmVec2Normalize;
pub use crate::src::kazmath::vec2::kmVec2Subtract;
// #[derive(Copy, Clone)]

pub type kmVec2 = crate::src::kazmath::aabb2::kmVec2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay2 {
    pub start: crate::src::kazmath::aabb2::kmVec2,
    pub dir: crate::src::kazmath::aabb2::kmVec2,
}
impl kmRay2 {
    pub const fn new() -> Self {
        kmRay2 {
        start: crate::src::kazmath::aabb2::kmVec2::new(),
        dir: crate::src::kazmath::aabb2::kmVec2::new()
        }
    }
}

impl std::default::Default for kmRay2 {
    fn default() -> Self { kmRay2::new() }
}

#[no_mangle]
pub unsafe extern "C" fn kmRay2Fill<'a1>(
    mut ray: Option<&'a1 mut crate::src::kazmath::ray2::kmRay2>,
    mut px: std::os::raw::c_float,
    mut py: std::os::raw::c_float,
    mut vx: std::os::raw::c_float,
    mut vy: std::os::raw::c_float,
) {
    (*(borrow_mut(&mut ray)).unwrap()).start.x = px;
    (*(borrow_mut(&mut ray)).unwrap()).start.y = py;
    (*(borrow_mut(&mut ray)).unwrap()).dir.x = vx;
    (*(borrow_mut(&mut ray)).unwrap()).dir.y = vy;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2FillWithEndpoints<'a1, 'a2, 'a3>(
    mut ray: Option<&'a1 mut crate::src::kazmath::ray2::kmRay2>,
    mut start: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut end: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
) {
    (*(borrow_mut(&mut ray)).unwrap()).start.x = (*(start).unwrap()).x;
    (*(borrow_mut(&mut ray)).unwrap()).start.y = (*(start).unwrap()).y;
    (*(borrow_mut(&mut ray)).unwrap()).dir.x = (*((end).clone()).unwrap()).x - (*((start).clone()).unwrap()).x;
    (*(borrow_mut(&mut ray)).unwrap()).dir.y = (*((end).clone()).unwrap()).y - (*((start).clone()).unwrap()).y;
}
#[no_mangle]
pub unsafe extern "C" fn kmLine2WithLineIntersection<'a1, 'a2, 'a3, 'a4, 'a5, 'a6, 'a7>(
    mut ptA: Option<&'a1 crate::src::kazmath::aabb2::kmVec2>,
    mut vecA: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut ptB: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
    mut vecB: Option<&'a4 crate::src::kazmath::aabb2::kmVec2>,
    mut outTA: Option<&'a5 mut std::os::raw::c_float>,
    mut outTB: Option<&'a6 mut std::os::raw::c_float>,
    mut outIntersection: Option<&'a7 mut crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_uchar {
    let mut x1 = (*(ptA).unwrap()).x;
    let mut y1 = (*(ptA).unwrap()).y;
    let mut x2 = x1 + (*((vecA).clone()).unwrap()).x;
    let mut y2 = y1 + (*((vecA).clone()).unwrap()).y;
    let mut x3 = (*(ptB).unwrap()).x;
    let mut y3 = (*(ptB).unwrap()).y;
    let mut x4 = x3 + (*((vecB).clone()).unwrap()).x;
    let mut y4 = y3 + (*((vecB).clone()).unwrap()).y;
    let mut denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    if denom as std::os::raw::c_double > -0.0001f64 && (denom as std::os::raw::c_double) < 0.0001f64 {
        return 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    }
    let mut ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denom;
    let mut ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denom;
    let mut x = x1 + ua * (x2 - x1);
    let mut y = y1 + ua * (y2 - y1);
    if !borrow(& outTA).is_none() {
        *(borrow_mut(&mut outTA)).unwrap() = ua;
    }
    if !borrow(& outTB).is_none() {
        *(borrow_mut(&mut outTB)).unwrap() = ub;
    }
    if !borrow(& outIntersection).is_none() {
        (*(borrow_mut(&mut outIntersection)).unwrap()).x = x;
        (*(borrow_mut(&mut outIntersection)).unwrap()).y = y;
    }
    return 1 as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmSegment2WithSegmentIntersection<'a1, 'a2, 'a3>(
    mut segmentA: Option<&'a1 crate::src::kazmath::ray2::kmRay2>,
    mut segmentB: Option<&'a2 crate::src::kazmath::ray2::kmRay2>,
    mut intersection: Option<&'a3 mut crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_uchar {
    let mut ua: f32 = 0.;
    let mut ub: f32 = 0.;
    let mut pt = kmVec2 { x: 0., y: 0. };
    if kmLine2WithLineIntersection(
        (Some(&(*((segmentA).clone()).unwrap()).start)).clone(),
        (Some(&(*((segmentA).clone()).unwrap()).dir)).clone(),
        (Some(&(*((segmentB).clone()).unwrap()).start)).clone(),
        (Some(&(*((segmentB).clone()).unwrap()).start)).clone(),
        Some(&mut ua),
        Some(&mut ub),
        Some(&mut pt),
    ) as std::os::raw::c_int != 0 && 0.0f64 <= ua as std::os::raw::c_double
        && ua as std::os::raw::c_double <= 1.0f64 && 0.0f64 <= ub as std::os::raw::c_double
        && ub as std::os::raw::c_double <= 1.0f64
    {
        (*(borrow_mut(&mut intersection)).unwrap()).x = pt.x;
        (*(borrow_mut(&mut intersection)).unwrap()).y = pt.y;
        return 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectLineSegment<'a1, 'a2, 'a3, 'a4>(
    mut ray: Option<&'a1 crate::src::kazmath::ray2::kmRay2>,
    mut p1: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut p2: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
    mut intersection: Option<&'a4 mut crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_uchar {
    let mut ua: f32 = 0.;
    let mut ub: f32 = 0.;
    let mut pt = kmVec2 { x: 0., y: 0. };
    let mut otherSegment = kmRay2 {
        start: kmVec2 { x: 0., y: 0. },
        dir: kmVec2 { x: 0., y: 0. },
    };
    kmRay2FillWithEndpoints(Some(&mut otherSegment), (p1).clone(), (p2).clone());
    if kmLine2WithLineIntersection(
        (Some(&(*((ray).clone()).unwrap()).start)).clone(),
        (Some(&(*((ray).clone()).unwrap()).dir)).clone(),
        Some(&mut otherSegment.start),
        Some(&mut otherSegment.dir),
        Some(&mut ua),
        Some(&mut ub),
        Some(&mut pt),
    ) as std::os::raw::c_int != 0 && 0.0f64 <= ua as std::os::raw::c_double
        && 0.0f64 <= ub as std::os::raw::c_double && ub as std::os::raw::c_double <= 1.0f64
    {
        (*(borrow_mut(&mut intersection)).unwrap()).x = pt.x;
        (*(borrow_mut(&mut intersection)).unwrap()).y = pt.y;
        return 1 as std::os::raw::c_int as std::os::raw::c_uchar;
    }
    return 0 as std::os::raw::c_int as std::os::raw::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn calculate_line_normal(
    mut p1: crate::src::kazmath::aabb2::kmVec2,
    mut p2: crate::src::kazmath::aabb2::kmVec2,
    mut other_point: crate::src::kazmath::aabb2::kmVec2,
    mut normal_out: * mut crate::src::kazmath::aabb2::kmVec2,
) {
    let mut edge = kmVec2 { x: 0., y: 0. };
    let mut other_edge = kmVec2 { x: 0., y: 0. };
    kmVec2Subtract(&mut edge, Some(&mut p2), Some(&mut p1));
    kmVec2Subtract(&mut other_edge, Some(&mut other_point), Some(&mut p1));
    kmVec2Normalize(&mut edge, &mut edge);
    kmVec2Normalize(&mut other_edge, &mut other_edge);
    let mut n = kmVec2 { x: 0., y: 0. };
    n.x = edge.y;
    n.y = -edge.x;
    let mut d = kmVec2Dot(Some(&mut n), &mut other_edge);
    if d > 0.0f32 {
        n.x = -n.x;
        n.y = -n.y;
    }
    (*normal_out).x = n.x;
    (*normal_out).y = n.y;
    kmVec2Normalize(normal_out, normal_out);
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectTriangle<'a1, 'a2, 'a3, 'a4, 'a5, 'a6, 'a7>(
    mut ray: Option<&'a1 crate::src::kazmath::ray2::kmRay2>,
    mut p1: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut p2: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
    mut p3: Option<&'a4 crate::src::kazmath::aabb2::kmVec2>,
    mut intersection: Option<&'a5 mut crate::src::kazmath::aabb2::kmVec2>,
    mut normal_out: Option<&'a6 mut crate::src::kazmath::aabb2::kmVec2>,
    mut distance_out: Option<&'a7 mut std::os::raw::c_float>,
) -> std::os::raw::c_uchar {
    let mut intersect = kmVec2 { x: 0., y: 0. };
    let mut final_intersect = kmVec2 { x: 0., y: 0. };
    let mut normal = kmVec2 { x: 0., y: 0. };
    let mut distance = 10000.0f32;
    let mut intersected = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    if kmRay2IntersectLineSegment((ray).clone(), (p1).clone(), (p2).clone(), Some(&mut intersect)) != 0 {
        let mut tmp = kmVec2 { x: 0., y: 0. };
        let mut this_distance = kmVec2Length(
            kmVec2Subtract(&mut tmp, Some(&mut intersect), (Some(&(*((ray).clone()).unwrap()).start)).clone()),
        );
        let mut this_normal = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*(p1).unwrap(), *(p2).unwrap(), *(p3).unwrap(), &mut this_normal);
        if this_distance < distance && kmVec2Dot(Some(&mut this_normal), &(*((ray).clone()).unwrap()).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance;
            kmVec2Assign(&mut normal, &mut this_normal);
            intersected = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment((ray).clone(), (p2).clone(), (p3).clone(), Some(&mut intersect)) != 0 {
        let mut tmp_0 = kmVec2 { x: 0., y: 0. };
        let mut this_distance_0 = kmVec2Length(
            kmVec2Subtract(&mut tmp_0, Some(&mut intersect), (Some(&(*((ray).clone()).unwrap()).start)).clone()),
        );
        let mut this_normal_0 = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*(p2).unwrap(), *(p3).unwrap(), *(p1).unwrap(), &mut this_normal_0);
        if this_distance_0 < distance
            && kmVec2Dot(Some(&mut this_normal_0), &(*((ray).clone()).unwrap()).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance_0;
            kmVec2Assign(&mut normal, &mut this_normal_0);
            intersected = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment((ray).clone(), (p3).clone(), (p1).clone(), Some(&mut intersect)) != 0 {
        let mut tmp_1 = kmVec2 { x: 0., y: 0. };
        let mut this_distance_1 = kmVec2Length(
            kmVec2Subtract(&mut tmp_1, Some(&mut intersect), (Some(&(*((ray).clone()).unwrap()).start)).clone()),
        );
        let mut this_normal_1 = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*(p3).unwrap(), *(p1).unwrap(), *(p2).unwrap(), &mut this_normal_1);
        if this_distance_1 < distance
            && kmVec2Dot(Some(&mut this_normal_1), &(*((ray).clone()).unwrap()).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance_1;
            kmVec2Assign(&mut normal, &mut this_normal_1);
            intersected = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
        }
    }
    if intersected != 0 {
        (*(borrow_mut(&mut intersection)).unwrap()).x = final_intersect.x;
        (*(borrow_mut(&mut intersection)).unwrap()).y = final_intersect.y;
        if !borrow(& normal_out).is_none() {
            (*(borrow_mut(&mut normal_out)).unwrap()).x = normal.x;
            (*(borrow_mut(&mut normal_out)).unwrap()).y = normal.y;
        }
        if distance != 0. {
            *(borrow_mut(&mut distance_out)).unwrap() = distance;
        }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectBox<'a1, 'a2, 'a3, 'a4, 'a5, 'a6, 'a7>(
    mut ray: Option<&'a1 crate::src::kazmath::ray2::kmRay2>,
    mut p1: Option<&'a2 crate::src::kazmath::aabb2::kmVec2>,
    mut p2: Option<&'a3 crate::src::kazmath::aabb2::kmVec2>,
    mut p3: Option<&'a4 crate::src::kazmath::aabb2::kmVec2>,
    mut p4: Option<&'a5 crate::src::kazmath::aabb2::kmVec2>,
    mut intersection: Option<&'a6 mut crate::src::kazmath::aabb2::kmVec2>,
    mut normal_out: Option<&'a7 mut crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_uchar {
    let mut intersected = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    let mut intersect = kmVec2 { x: 0., y: 0. };
    let mut final_intersect = kmVec2 { x: 0., y: 0. };
    let mut normal = kmVec2 { x: 0., y: 0. };
    let mut distance = 10000.0f32;
    let mut points: [Option<&'_ crate::src::kazmath::aabb2::kmVec2>; 4] = [Option::<&'_ crate::src::kazmath::aabb2::kmVec2>::None,Option::<&'_ crate::src::kazmath::aabb2::kmVec2>::None,Option::<&'_ crate::src::kazmath::aabb2::kmVec2>::None,Option::<&'_ crate::src::kazmath::aabb2::kmVec2>::None,];
    points[0 as std::os::raw::c_int as usize] = (p1).clone();
    points[1 as std::os::raw::c_int as usize] = (p2).clone();
    points[2 as std::os::raw::c_int as usize] = (p3).clone();
    points[3 as std::os::raw::c_int as usize] = (p4).clone();
    let mut i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while i < 4 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut this_point = (points[i as usize]).clone();
        let mut next_point = if i == 3 as std::os::raw::c_int as std::os::raw::c_uint {
            (points[0 as std::os::raw::c_int as usize]).clone()
        } else {
            (points[i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_uint) as usize]).clone()
        };
        let mut other_point = if i == 3 as std::os::raw::c_int as std::os::raw::c_uint
            || i == 0 as std::os::raw::c_int as std::os::raw::c_uint
        {
            (points[1 as std::os::raw::c_int as usize]).clone()
        } else {
            (points[0 as std::os::raw::c_int as usize]).clone()
        };
        if kmRay2IntersectLineSegment((ray).clone(), (this_point).clone(), (next_point).clone(), Some(&mut intersect)) != 0 {
            let mut tmp = kmVec2 { x: 0., y: 0. };
            let mut this_distance = kmVec2Length(
                kmVec2Subtract(&mut tmp, Some(&mut intersect), (Some(&(*((ray).clone()).unwrap()).start)).clone()),
            );
            let mut this_normal = kmVec2 { x: 0., y: 0. };
            calculate_line_normal(
                *(this_point).unwrap(),
                *(next_point).unwrap(),
                *(other_point).unwrap(),
                &mut this_normal,
            );
            if this_distance < distance
                && kmVec2Dot(Some(&mut this_normal), &(*((ray).clone()).unwrap()).dir) < 0.0f32
            {
                kmVec2Assign(&mut final_intersect, &mut intersect);
                distance = this_distance;
                intersected = 1 as std::os::raw::c_int as std::os::raw::c_uchar;
                kmVec2Assign(&mut normal, &mut this_normal);
            }
        }
        i = i.wrapping_add(1);
    }
    if intersected != 0 {
        (*(borrow_mut(&mut intersection)).unwrap()).x = final_intersect.x;
        (*(borrow_mut(&mut intersection)).unwrap()).y = final_intersect.y;
        if !borrow(& normal_out).is_none() {
            (*(borrow_mut(&mut normal_out)).unwrap()).x = normal.x;
            (*(borrow_mut(&mut normal_out)).unwrap()).y = normal.y;
        }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectCircle<'a1, 'a2>(
    mut ray: Option<&'a1 crate::src::kazmath::ray2::kmRay2>,
    centre: crate::src::kazmath::aabb2::kmVec2,
    radius: std::os::raw::c_float,
    mut intersection: Option<&'a2 mut crate::src::kazmath::aabb2::kmVec2>,
) -> std::os::raw::c_uchar {
    if 0 as std::os::raw::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const std::os::raw::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const std::os::raw::c_char,
            b"../kazmath/ray2.c\0" as *const u8 as *const std::os::raw::c_char,
            256 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 89], &'_ [i8; 89]>(
                b"unsigned char kmRay2IntersectCircle(const kmRay2 *, const kmVec2, const float, kmVec2 *)\0",
            ))
                .as_ptr(),
        );
    }
    return 1 as std::os::raw::c_int as std::os::raw::c_uchar;
}
use crate::laertes_rt::*;
