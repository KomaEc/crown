use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn kmVec2Length(pIn: *const kmVec2) -> libc::c_float;
    fn kmVec2Normalize(pOut: *mut kmVec2, pIn: *const kmVec2) -> *mut kmVec2;
    fn kmVec2Dot(pV1: *const kmVec2, pV2: *const kmVec2) -> libc::c_float;
    fn kmVec2Subtract(
        pOut: *mut kmVec2,
        pV1: *const kmVec2,
        pV2: *const kmVec2,
    ) -> *mut kmVec2;
    fn kmVec2Assign(pOut: *mut kmVec2, pIn: *const kmVec2) -> *mut kmVec2;
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec2 {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay2 {
    pub start: kmVec2,
    pub dir: kmVec2,
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2Fill(
    mut ray: *mut kmRay2,
    mut px: libc::c_float,
    mut py: libc::c_float,
    mut vx: libc::c_float,
    mut vy: libc::c_float,
) {
    (*ray).start.x = px;
    (*ray).start.y = py;
    (*ray).dir.x = vx;
    (*ray).dir.y = vy;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2FillWithEndpoints(
    mut ray: *mut kmRay2,
    mut start: *const kmVec2,
    mut end: *const kmVec2,
) {
    (*ray).start.x = (*start).x;
    (*ray).start.y = (*start).y;
    (*ray).dir.x = (*end).x - (*start).x;
    (*ray).dir.y = (*end).y - (*start).y;
}
#[no_mangle]
pub unsafe extern "C" fn kmLine2WithLineIntersection(
    mut ptA: *const kmVec2,
    mut vecA: *const kmVec2,
    mut ptB: *const kmVec2,
    mut vecB: *const kmVec2,
    mut outTA: *mut libc::c_float,
    mut outTB: *mut libc::c_float,
    mut outIntersection: *mut kmVec2,
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
    if !outTA.is_null() {
        *outTA = ua;
    }
    if !outTB.is_null() {
        *outTB = ub;
    }
    if !outIntersection.is_null() {
        (*outIntersection).x = x;
        (*outIntersection).y = y;
    }
    return 1 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmSegment2WithSegmentIntersection(
    mut segmentA: *const kmRay2,
    mut segmentB: *const kmRay2,
    mut intersection: *mut kmVec2,
) -> libc::c_uchar {
    let mut ua: libc::c_float = 0.;
    let mut ub: libc::c_float = 0.;
    let mut pt = kmVec2 { x: 0., y: 0. };
    if kmLine2WithLineIntersection(
        &(*segmentA).start,
        &(*segmentA).dir,
        &(*segmentB).start,
        &(*segmentB).start,
        &mut ua,
        &mut ub,
        &mut pt,
    ) as libc::c_int != 0 && 0.0f64 <= ua as libc::c_double
        && ua as libc::c_double <= 1.0f64 && 0.0f64 <= ub as libc::c_double
        && ub as libc::c_double <= 1.0f64
    {
        (*intersection).x = pt.x;
        (*intersection).y = pt.y;
        return 1 as libc::c_int as libc::c_uchar;
    }
    return 0 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectLineSegment(
    mut ray: *const kmRay2,
    mut p1: *const kmVec2,
    mut p2: *const kmVec2,
    mut intersection: *mut kmVec2,
) -> libc::c_uchar {
    let mut ua: libc::c_float = 0.;
    let mut ub: libc::c_float = 0.;
    let mut pt = kmVec2 { x: 0., y: 0. };
    let mut otherSegment = kmRay2 {
        start: kmVec2 { x: 0., y: 0. },
        dir: kmVec2 { x: 0., y: 0. },
    };
    kmRay2FillWithEndpoints(&mut otherSegment, p1, p2);
    if kmLine2WithLineIntersection(
        &(*ray).start,
        &(*ray).dir,
        &mut otherSegment.start,
        &mut otherSegment.dir,
        &mut ua,
        &mut ub,
        &mut pt,
    ) as libc::c_int != 0 && 0.0f64 <= ua as libc::c_double
        && 0.0f64 <= ub as libc::c_double && ub as libc::c_double <= 1.0f64
    {
        (*intersection).x = pt.x;
        (*intersection).y = pt.y;
        return 1 as libc::c_int as libc::c_uchar;
    }
    return 0 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn calculate_line_normal(
    mut p1: kmVec2,
    mut p2: kmVec2,
    mut other_point: kmVec2,
    mut normal_out: *mut kmVec2,
) {
    let mut edge = kmVec2 { x: 0., y: 0. };
    let mut other_edge = kmVec2 { x: 0., y: 0. };
    kmVec2Subtract(&mut edge, &mut p2, &mut p1);
    kmVec2Subtract(&mut other_edge, &mut other_point, &mut p1);
    kmVec2Normalize(&mut edge, &mut edge);
    kmVec2Normalize(&mut other_edge, &mut other_edge);
    let mut n = kmVec2 { x: 0., y: 0. };
    n.x = edge.y;
    n.y = -edge.x;
    let mut d = kmVec2Dot(&mut n, &mut other_edge);
    if d > 0.0f32 {
        n.x = -n.x;
        n.y = -n.y;
    }
    (*normal_out).x = n.x;
    (*normal_out).y = n.y;
    kmVec2Normalize(normal_out, normal_out);
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectTriangle(
    mut ray: *const kmRay2,
    mut p1: *const kmVec2,
    mut p2: *const kmVec2,
    mut p3: *const kmVec2,
    mut intersection: *mut kmVec2,
    mut normal_out: *mut kmVec2,
    mut distance_out: *mut libc::c_float,
) -> libc::c_uchar {
    let mut intersect = kmVec2 { x: 0., y: 0. };
    let mut final_intersect = kmVec2 { x: 0., y: 0. };
    let mut normal = kmVec2 { x: 0., y: 0. };
    let mut distance = 10000.0f32;
    let mut intersected = 0 as libc::c_int as libc::c_uchar;
    if kmRay2IntersectLineSegment(ray, p1, p2, &mut intersect) != 0 {
        let mut tmp = kmVec2 { x: 0., y: 0. };
        let mut this_distance = kmVec2Length(
            kmVec2Subtract(&mut tmp, &mut intersect, &(*ray).start),
        );
        let mut this_normal = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*p1, *p2, *p3, &mut this_normal);
        if this_distance < distance && kmVec2Dot(&mut this_normal, &(*ray).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance;
            kmVec2Assign(&mut normal, &mut this_normal);
            intersected = 1 as libc::c_int as libc::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment(ray, p2, p3, &mut intersect) != 0 {
        let mut tmp_0 = kmVec2 { x: 0., y: 0. };
        let mut this_distance_0 = kmVec2Length(
            kmVec2Subtract(&mut tmp_0, &mut intersect, &(*ray).start),
        );
        let mut this_normal_0 = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*p2, *p3, *p1, &mut this_normal_0);
        if this_distance_0 < distance
            && kmVec2Dot(&mut this_normal_0, &(*ray).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance_0;
            kmVec2Assign(&mut normal, &mut this_normal_0);
            intersected = 1 as libc::c_int as libc::c_uchar;
        }
    }
    if kmRay2IntersectLineSegment(ray, p3, p1, &mut intersect) != 0 {
        let mut tmp_1 = kmVec2 { x: 0., y: 0. };
        let mut this_distance_1 = kmVec2Length(
            kmVec2Subtract(&mut tmp_1, &mut intersect, &(*ray).start),
        );
        let mut this_normal_1 = kmVec2 { x: 0., y: 0. };
        calculate_line_normal(*p3, *p1, *p2, &mut this_normal_1);
        if this_distance_1 < distance
            && kmVec2Dot(&mut this_normal_1, &(*ray).dir) < 0.0f32
        {
            final_intersect.x = intersect.x;
            final_intersect.y = intersect.y;
            distance = this_distance_1;
            kmVec2Assign(&mut normal, &mut this_normal_1);
            intersected = 1 as libc::c_int as libc::c_uchar;
        }
    }
    if intersected != 0 {
        (*intersection).x = final_intersect.x;
        (*intersection).y = final_intersect.y;
        if !normal_out.is_null() {
            (*normal_out).x = normal.x;
            (*normal_out).y = normal.y;
        }
        if distance != 0. {
            *distance_out = distance;
        }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectBox(
    mut ray: *const kmRay2,
    mut p1: *const kmVec2,
    mut p2: *const kmVec2,
    mut p3: *const kmVec2,
    mut p4: *const kmVec2,
    mut intersection: *mut kmVec2,
    mut normal_out: *mut kmVec2,
) -> libc::c_uchar {
    let mut intersected = 0 as libc::c_int as libc::c_uchar;
    let mut intersect = kmVec2 { x: 0., y: 0. };
    let mut final_intersect = kmVec2 { x: 0., y: 0. };
    let mut normal = kmVec2 { x: 0., y: 0. };
    let mut distance = 10000.0f32;
    let mut points: [*const kmVec2; 4] = [0 as *const kmVec2; 4];
    points[0 as libc::c_int as usize] = p1;
    points[1 as libc::c_int as usize] = p2;
    points[2 as libc::c_int as usize] = p3;
    points[3 as libc::c_int as usize] = p4;
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
        if kmRay2IntersectLineSegment(ray, this_point, next_point, &mut intersect) != 0 {
            let mut tmp = kmVec2 { x: 0., y: 0. };
            let mut this_distance = kmVec2Length(
                kmVec2Subtract(&mut tmp, &mut intersect, &(*ray).start),
            );
            let mut this_normal = kmVec2 { x: 0., y: 0. };
            calculate_line_normal(
                *this_point,
                *next_point,
                *other_point,
                &mut this_normal,
            );
            if this_distance < distance
                && kmVec2Dot(&mut this_normal, &(*ray).dir) < 0.0f32
            {
                kmVec2Assign(&mut final_intersect, &mut intersect);
                distance = this_distance;
                intersected = 1 as libc::c_int as libc::c_uchar;
                kmVec2Assign(&mut normal, &mut this_normal);
            }
        }
        i = i.wrapping_add(1);
    }
    if intersected != 0 {
        (*intersection).x = final_intersect.x;
        (*intersection).y = final_intersect.y;
        if !normal_out.is_null() {
            (*normal_out).x = normal.x;
            (*normal_out).y = normal.y;
        }
    }
    return intersected;
}
#[no_mangle]
pub unsafe extern "C" fn kmRay2IntersectCircle(
    mut ray: *const kmRay2,
    centre: kmVec2,
    radius: libc::c_float,
    mut intersection: *mut kmVec2,
) -> libc::c_uchar {
    if 0 as libc::c_int != 0
        && !(b"Not implemented\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"0 && \"Not implemented\"\0" as *const u8 as *const libc::c_char,
            b"../kazmath/ray2.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 89],
                &[libc::c_char; 89],
            >(
                b"unsigned char kmRay2IntersectCircle(const kmRay2 *, const kmVec2, const float, kmVec2 *)\0",
            ))
                .as_ptr(),
        );
    }
    return 1 as libc::c_int as libc::c_uchar;
}
