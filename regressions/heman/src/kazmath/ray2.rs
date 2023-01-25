use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    
    
    
    
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor11 { dummy: () }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay2 {
    pub start: crate::src::kazmath::aabb2::kmVec2,
    pub dir: crate::src::kazmath::aabb2::kmVec2,
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2Fill(
    mut ray: Option<&mut kmRay2>,
    mut px: libc::c_float,
    mut py: libc::c_float,
    mut vx: libc::c_float,
    mut vy: libc::c_float,
) {
    (*ray.as_deref_mut().unwrap()).start.x= px;
    (*ray.as_deref_mut().unwrap()).start.y= py;
    (*ray.as_deref_mut().unwrap()).dir.x= vx;
    (*ray.as_deref_mut().unwrap()).dir.y= vy;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2FillWithEndpoints(
    mut ray: Option<&mut kmRay2>,
    mut start: *const crate::src::kazmath::aabb2::kmVec2,
    mut end: *const crate::src::kazmath::aabb2::kmVec2,
) {
    (*ray.as_deref_mut().unwrap()).start.x= (*start).x;
    (*ray.as_deref_mut().unwrap()).start.y= (*start).y;
    (*ray.as_deref_mut().unwrap()).dir.x= (*end).x - (*start).x;
    (*ray.as_deref_mut().unwrap()).dir.y= (*end).y - (*start).y;
}
#[no_mangle]
pub unsafe extern "C" fn kmLine2WithLineIntersection(
    mut ptA: *const crate::src::kazmath::aabb2::kmVec2,
    mut vecA: *const crate::src::kazmath::aabb2::kmVec2,
    mut ptB: *const crate::src::kazmath::aabb2::kmVec2,
    mut vecB: *const crate::src::kazmath::aabb2::kmVec2,
    mut outTA: Option<&mut libc::c_float>,
    mut outTB: Option<&mut libc::c_float>,
    mut outIntersection: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
) -> libc::c_uchar {
    let mut x1 = (*ptA).x;
    let mut y1 = (*ptA).y;
    let mut x2 = x1 + (*vecA).x;
    let mut y2 = y1 + (*vecA).y;
    let mut x3 = (*ptB).x;
    let mut y3 = (*ptB).y;
    let mut x4 = x3 + (*vecB).x;
    let mut y4 = y3 + (*vecB).y;
    let mut denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    if denom as libc::c_double > -0.0001f64 && (denom as libc::c_double) < 0.0001f64 {
        return 0 as libc::c_int as libc::c_uchar;
    }
    let mut ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denom;
    let mut ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denom;
    let mut x = x1 + ua * (x2 - x1);
    let mut y = y1 + ua * (y2 - y1);
    if !outTA.as_deref().is_none() {
        *outTA.as_deref_mut().unwrap()= ua;
    }else { (); }
    if !outTB.as_deref().is_none() {
        *outTB.as_deref_mut().unwrap()= ub;
    }else { (); }
    if !outIntersection.as_deref().is_none() {
        (*outIntersection.as_deref_mut().unwrap()).x= x;
        (*outIntersection.as_deref_mut().unwrap()).y= y;
    }else { (); }
    return 1 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmSegment2WithSegmentIntersection(
    mut segmentA: *const kmRay2,
    mut segmentB: *const kmRay2,
    mut intersection: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
) -> libc::c_uchar {
    let mut ua: libc::c_float = 0.;
    let mut ub: libc::c_float = 0.;
    let mut pt = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    if kmLine2WithLineIntersection(
        &(*segmentA).start,
        &(*segmentA).dir,
        &(*segmentB).start,
        &(*segmentB).start,
        Some(&mut ua),
        Some(&mut ub),
        Some(&mut pt),
    ) as libc::c_int != 0 && 0.0f64 <= ua as libc::c_double
        && ua as libc::c_double <= 1.0f64 && 0.0f64 <= ub as libc::c_double
        && ub as libc::c_double <= 1.0f64
    {
        (*intersection.as_deref_mut().unwrap()).x= pt.x;
        (*intersection.as_deref_mut().unwrap()).y= pt.y;
        return 1 as libc::c_int as libc::c_uchar;
    }
    return 0 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectLineSegment(
    mut ray: *const kmRay2,
    mut p1: *const crate::src::kazmath::aabb2::kmVec2,
    mut p2: *const crate::src::kazmath::aabb2::kmVec2,
    mut intersection: *mut crate::src::kazmath::aabb2::kmVec2,
) -> libc::c_uchar {
    let mut ua: libc::c_float = 0.;
    let mut ub: libc::c_float = 0.;
    let mut pt = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut otherSegment = kmRay2 {
        start: crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. },
        dir: crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. },
    };
    kmRay2FillWithEndpoints(Some(&mut otherSegment), p1, p2);
    if kmLine2WithLineIntersection(
        &(*ray).start,
        &(*ray).dir,
        core::ptr::addr_of!(otherSegment.start),
        core::ptr::addr_of!(otherSegment.dir),
        Some(&mut ua),
        Some(&mut ub),
        Some(&mut pt),
    ) as libc::c_int != 0 && 0.0f64 <= ua as libc::c_double
        && 0.0f64 <= ub as libc::c_double && ub as libc::c_double <= 1.0f64
    {
        (*intersection).x= pt.x;
        (*intersection).y= pt.y;
        return 1 as libc::c_int as libc::c_uchar;
    }
    return 0 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn calculate_line_normal(
    mut p1: crate::src::kazmath::aabb2::kmVec2,
    mut p2: crate::src::kazmath::aabb2::kmVec2,
    mut other_point: crate::src::kazmath::aabb2::kmVec2,
    mut normal_out: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
) {
    let mut edge = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut other_edge = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    crate::src::kazmath::vec2::kmVec2Subtract(core::ptr::addr_of_mut!(edge), core::ptr::addr_of!(p2), core::ptr::addr_of!(p1));
    crate::src::kazmath::vec2::kmVec2Subtract(core::ptr::addr_of_mut!(other_edge), core::ptr::addr_of!(other_point), core::ptr::addr_of!(p1));
    crate::src::kazmath::vec2::kmVec2Normalize(core::ptr::addr_of_mut!(edge), core::ptr::addr_of!(edge));
    crate::src::kazmath::vec2::kmVec2Normalize(core::ptr::addr_of_mut!(other_edge), core::ptr::addr_of!(other_edge));
    let mut n = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    n.x= edge.y;
    n.y= -edge.x;
    let mut d = crate::src::kazmath::vec2::kmVec2Dot(core::ptr::addr_of!(n), core::ptr::addr_of!(other_edge));
    if d > 0.0f32 {
        n.x= -n.x;
        n.y= -n.y;
    }
    (*normal_out.as_deref_mut().unwrap()).x= n.x;
    (*normal_out.as_deref_mut().unwrap()).y= n.y;
    crate::src::kazmath::vec2::kmVec2Normalize(normal_out.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), normal_out.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectTriangle(
    mut ray: *const kmRay2,
    mut p1: *const crate::src::kazmath::aabb2::kmVec2,
    mut p2: *const crate::src::kazmath::aabb2::kmVec2,
    mut p3: *const crate::src::kazmath::aabb2::kmVec2,
    mut intersection: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut normal_out: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut distance_out: Option<&mut libc::c_float>,
) -> libc::c_uchar {
    let mut intersect = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut final_intersect = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut normal = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut distance = 10000.0f32;
    let mut intersected = 0 as libc::c_int as libc::c_uchar;
    if kmRay2IntersectLineSegment(ray, p1, p2, core::ptr::addr_of_mut!(intersect)) != 0 {
        let mut tmp = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
        let mut this_distance = crate::src::kazmath::vec2::kmVec2Length(
            crate::src::kazmath::vec2::kmVec2Subtract(core::ptr::addr_of_mut!(tmp), core::ptr::addr_of!(intersect), &(*ray).start),
        );
        let mut this_normal = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
        calculate_line_normal((*p1), (*p2), (*p3), Some(&mut this_normal));
        if this_distance < distance && crate::src::kazmath::vec2::kmVec2Dot(core::ptr::addr_of!(this_normal), &(*ray).dir) < 0.0f32
        {
            final_intersect.x= intersect.x;
            final_intersect.y= intersect.y;
            distance= this_distance;
            crate::src::kazmath::vec2::kmVec2Assign(core::ptr::addr_of_mut!(normal), core::ptr::addr_of!(this_normal));
            intersected= 1 as libc::c_int as libc::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment(ray, p2, p3, core::ptr::addr_of_mut!(intersect)) != 0 {
        let mut tmp_0 = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
        let mut this_distance_0 = crate::src::kazmath::vec2::kmVec2Length(
            crate::src::kazmath::vec2::kmVec2Subtract(core::ptr::addr_of_mut!(tmp_0), core::ptr::addr_of!(intersect), &(*ray).start),
        );
        let mut this_normal_0 = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
        calculate_line_normal((*p2), (*p3), (*p1), Some(&mut this_normal_0));
        if this_distance_0 < distance
            && crate::src::kazmath::vec2::kmVec2Dot(core::ptr::addr_of!(this_normal_0), &(*ray).dir) < 0.0f32
        {
            final_intersect.x= intersect.x;
            final_intersect.y= intersect.y;
            distance= this_distance_0;
            crate::src::kazmath::vec2::kmVec2Assign(core::ptr::addr_of_mut!(normal), core::ptr::addr_of!(this_normal_0));
            intersected= 1 as libc::c_int as libc::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment(ray, p3, p1, core::ptr::addr_of_mut!(intersect)) != 0 {
        let mut tmp_1 = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
        let mut this_distance_1 = crate::src::kazmath::vec2::kmVec2Length(
            crate::src::kazmath::vec2::kmVec2Subtract(core::ptr::addr_of_mut!(tmp_1), core::ptr::addr_of!(intersect), &(*ray).start),
        );
        let mut this_normal_1 = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
        calculate_line_normal((*p3), (*p1), (*p2), Some(&mut this_normal_1));
        if this_distance_1 < distance
            && crate::src::kazmath::vec2::kmVec2Dot(core::ptr::addr_of!(this_normal_1), &(*ray).dir) < 0.0f32
        {
            final_intersect.x= intersect.x;
            final_intersect.y= intersect.y;
            distance= this_distance_1;
            crate::src::kazmath::vec2::kmVec2Assign(core::ptr::addr_of_mut!(normal), core::ptr::addr_of!(this_normal_1));
            intersected= 1 as libc::c_int as libc::c_uchar;
        }
    }
    if intersected != 0 {
        (*intersection.as_deref_mut().unwrap()).x= final_intersect.x;
        (*intersection.as_deref_mut().unwrap()).y= final_intersect.y;
        if !normal_out.as_deref().is_none() {
            (*normal_out.as_deref_mut().unwrap()).x= normal.x;
            (*normal_out.as_deref_mut().unwrap()).y= normal.y;
        }else { (); }
        if distance != 0. {
            *distance_out.as_deref_mut().unwrap()= distance;
        }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectBox(
    mut ray: *const kmRay2,
    mut p1: *const crate::src::kazmath::aabb2::kmVec2,
    mut p2: *const crate::src::kazmath::aabb2::kmVec2,
    mut p3: *const crate::src::kazmath::aabb2::kmVec2,
    mut p4: *const crate::src::kazmath::aabb2::kmVec2,
    mut intersection: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
    mut normal_out: Option<&mut crate::src::kazmath::aabb2::kmVec2>,
) -> libc::c_uchar {
    let mut intersected = 0 as libc::c_int as libc::c_uchar;
    let mut intersect = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut final_intersect = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut normal = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
    let mut distance = 10000.0f32;
    let mut points: [*const crate::src::kazmath::aabb2::kmVec2; 4] = [0 as *const crate::src::kazmath::aabb2::kmVec2; 4];
    points[0 as libc::c_int as usize]= p1;
    points[1 as libc::c_int as usize]= p2;
    points[2 as libc::c_int as usize]= p3;
    points[3 as libc::c_int as usize]= p4;
    let mut i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        let mut this_point = points[i as usize];
        let mut next_point = if i == 3 as libc::c_int as libc::c_uint {
            points[0 as libc::c_int as usize]
        } else {
            points[i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
        };
        let mut other_point = if i == 3 as libc::c_int as libc::c_uint
            || i == 0 as libc::c_int as libc::c_uint
        {
            points[1 as libc::c_int as usize]
        } else {
            points[0 as libc::c_int as usize]
        };
        if kmRay2IntersectLineSegment(ray, this_point, next_point, core::ptr::addr_of_mut!(intersect)) != 0 {
            let mut tmp = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
            let mut this_distance = crate::src::kazmath::vec2::kmVec2Length(
                crate::src::kazmath::vec2::kmVec2Subtract(core::ptr::addr_of_mut!(tmp), core::ptr::addr_of!(intersect), &(*ray).start),
            );
            let mut this_normal = crate::src::kazmath::aabb2::kmVec2 { x: 0., y: 0. };
            calculate_line_normal(
                (*this_point),
                (*next_point),
                (*other_point),
                Some(&mut this_normal),
            );
            if this_distance < distance
                && crate::src::kazmath::vec2::kmVec2Dot(core::ptr::addr_of!(this_normal), &(*ray).dir) < 0.0f32
            {
                crate::src::kazmath::vec2::kmVec2Assign(core::ptr::addr_of_mut!(final_intersect), core::ptr::addr_of!(intersect));
                distance= this_distance;
                intersected= 1 as libc::c_int as libc::c_uchar;
                crate::src::kazmath::vec2::kmVec2Assign(core::ptr::addr_of_mut!(normal), core::ptr::addr_of!(this_normal));
            }
        }
        i= i.wrapping_add(1);
    }
    if intersected != 0 {
        (*intersection.as_deref_mut().unwrap()).x= final_intersect.x;
        (*intersection.as_deref_mut().unwrap()).y= final_intersect.y;
        if !normal_out.as_deref().is_none() {
            (*normal_out.as_deref_mut().unwrap()).x= normal.x;
            (*normal_out.as_deref_mut().unwrap()).y= normal.y;
        }else { (); }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectCircle(
    mut ray: *const kmRay2,
    mut centre: crate::src::kazmath::aabb2::kmVec2,
    mut radius: libc::c_float,
    mut intersection: *mut crate::src::kazmath::aabb2::kmVec2,
) -> libc::c_uchar {
    if 0 as libc::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const libc::c_char,
            b"../kazmath/ray2.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_uint,
            b"unsigned char kmRay2IntersectCircle(const kmRay2 *, const kmVec2, const float, kmVec2 *)\0" as *const u8 as *const i8,
        );
    }
    return 1 as libc::c_int as libc::c_uchar;
}
