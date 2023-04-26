
extern "C" {
    
    
    
    
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
    fn abs(_: std::os::raw::c_int) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn memset(
        _: * mut core::ffi::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn atan(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::kazmath::utility::kmClamp;
pub use crate::src::kazmath::utility::kmMax;
pub use crate::src::kazmath::vec3::kmVec3Cross;
pub use crate::src::kazmath::vec3::kmVec3Dot;
pub use crate::src::kazmath::vec3::kmVec3Length;
pub use crate::src::kazmath::vec3::kmVec3Lerp;
pub use crate::src::kazmath::vec3::kmVec3Normalize;
pub use crate::src::kazmath::vec3::kmVec3Scale;
pub use crate::src::kazmath::vec3::kmVec3Subtract;
pub use crate::src::src::image::heman_image_create;
pub use crate::src::src::image::heman_image_destroy;
pub use crate::src::src::image::heman_image_texel;
pub use crate::src::kazmath::vec3::KM_VEC3_POS_Z;
pub use crate::src::src::color::_gamma;
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
static mut _occlusion_scale: std::os::raw::c_float = 0.0; unsafe fn laertes_init__occlusion_scale() {
_occlusion_scale = 1.0f32;}//;
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_set_occlusion_scale(mut s: std::os::raw::c_float) {
    _occlusion_scale = s;
}
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_compute_normals(
    mut heightmap: * mut crate::src::src::color::heman_image_s,
) -> * mut crate::src::src::color::heman_image_s {
    if (*heightmap).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"heightmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
            16 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 59], &'_ [i8; 59]>(b"heman_image *heman_lighting_compute_normals(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut width = (*heightmap).width;
    let mut height = (*heightmap).height;
    let mut result = heman_image_create(width, height, 3 as std::os::raw::c_int);
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut maxx = width - 1 as std::os::raw::c_int;
    let mut maxy = height - 1 as std::os::raw::c_int;
    let mut normals = (*result).data as *mut kmVec3;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut v = y as std::os::raw::c_float * invh;
        let mut y1 = if y + 1 as std::os::raw::c_int > maxy {
            maxy
        } else {
            y + 1 as std::os::raw::c_int
        };
        let mut p = kmVec3 { x: 0., y: 0., z: 0. };
        let mut px = kmVec3 { x: 0., y: 0., z: 0. };
        let mut py = kmVec3 { x: 0., y: 0., z: 0. };
        let mut n = normals.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut u = x as std::os::raw::c_float * invw;
            let mut x1 = if x + 1 as std::os::raw::c_int > maxx {
                maxx
            } else {
                x + 1 as std::os::raw::c_int
            };
            p.x = u;
            p.y = v;
            p.z = *heman_image_texel(heightmap, x, y);
            px.x = u + invw;
            px.y = v;
            px.z = *heman_image_texel(heightmap, x1, y);
            py.x = u;
            py.y = v + invh;
            py.z = *heman_image_texel(heightmap, x, y1);
            kmVec3Subtract(&mut px, &mut px, &mut p);
            kmVec3Subtract(&mut py, &mut py, &mut p);
            kmVec3Cross(n, Some(&mut px), &mut py);
            kmVec3Normalize(n, n);
            (*n).y *= -(1 as std::os::raw::c_int) as std::os::raw::c_float;
            x += 1;
            n = n.offset(1);
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_apply(
    mut heightmap: * mut crate::src::src::color::heman_image_s,
    mut albedo: * mut crate::src::src::color::heman_image_s,
    mut occlusion: std::os::raw::c_float,
    mut diffuse: std::os::raw::c_float,
    mut diffuse_softening: std::os::raw::c_float,
    mut light_position: * const std::os::raw::c_float,
) -> * mut crate::src::src::color::heman_image_s {
    if (*heightmap).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"heightmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
            61 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 100], &'_ [i8; 100]>(
                b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut width = (*heightmap).width;
    let mut height = (*heightmap).height;
    let mut final_0 = heman_image_create(width, height, 3 as std::os::raw::c_int);
    let mut normals = heman_lighting_compute_normals(heightmap);
    let mut occ = heman_lighting_compute_occlusion(heightmap);
    if !albedo.is_null() {
        if (*albedo).nbands == 3 as std::os::raw::c_int {} else {
            __assert_fail(
                b"albedo->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
                69 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 100], &'_ [i8; 100]>(
                    b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*albedo).width == width {} else {
            __assert_fail(
                b"albedo->width == width\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
                70 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 100], &'_ [i8; 100]>(
                    b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
        if (*albedo).height == height {} else {
            __assert_fail(
                b"albedo->height == height\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
                71 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 100], &'_ [i8; 100]>(
                    b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",
                ))
                    .as_ptr(),
            );
        }
    }
    static mut default_pos: [std::os::raw::c_float; 3] = [0.0,0.0,0.0,]; unsafe fn laertes_init_default_pos() {
default_pos = [-0.5f32, 0.5f32, 1.0f32];}//;
    if light_position.is_null() {
        light_position = default_pos.as_mut_ptr();
    }
    let mut colors = (*final_0).data as *mut kmVec3;
    let mut invgamma = 1.0f32 / _gamma;
    let mut L = kmVec3 { x: 0., y: 0., z: 0. };
    L.x = *light_position.offset(0 as std::os::raw::c_int as isize);
    L.y = *light_position.offset(1 as std::os::raw::c_int as isize);
    L.z = *light_position.offset(2 as std::os::raw::c_int as isize);
    kmVec3Normalize(&mut L, &mut L);
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut color = colors.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut N = heman_image_texel(normals, x, y) as *mut kmVec3;
            kmVec3Lerp(N, N, (Some(&KM_VEC3_POS_Z)).clone(), diffuse_softening);
            let mut df = 1 as std::os::raw::c_int as std::os::raw::c_float
                - diffuse
                    * (1 as std::os::raw::c_int as std::os::raw::c_float
                        - kmClamp(
                            kmVec3Dot(N, &mut L),
                            0 as std::os::raw::c_int as std::os::raw::c_float,
                            1 as std::os::raw::c_int as std::os::raw::c_float,
                        ));
            let mut of = 1 as std::os::raw::c_int as std::os::raw::c_float
                - occlusion
                    * (1 as std::os::raw::c_int as std::os::raw::c_float
                        - *heman_image_texel(occ, x, y));
            if !albedo.is_null() {
                *color = *(heman_image_texel(albedo, x, y) as *mut kmVec3);
            } else {
                let ref mut fresh0 = (*color).z;
                *fresh0 = 1 as std::os::raw::c_int as std::os::raw::c_float;
                let ref mut fresh1 = (*color).y;
                *fresh1 = *fresh0;
                (*color).x = *fresh1;
            }
            (*color)
                .x = pow((*color).x as std::os::raw::c_double, _gamma as std::os::raw::c_double)
                as std::os::raw::c_float;
            (*color)
                .y = pow((*color).y as std::os::raw::c_double, _gamma as std::os::raw::c_double)
                as std::os::raw::c_float;
            (*color)
                .z = pow((*color).z as std::os::raw::c_double, _gamma as std::os::raw::c_double)
                as std::os::raw::c_float;
            kmVec3Scale(color, color, df * of);
            (*color)
                .x = pow((*color).x as std::os::raw::c_double, invgamma as std::os::raw::c_double)
                as std::os::raw::c_float;
            (*color)
                .y = pow((*color).y as std::os::raw::c_double, invgamma as std::os::raw::c_double)
                as std::os::raw::c_float;
            (*color)
                .z = pow((*color).z as std::os::raw::c_double, invgamma as std::os::raw::c_double)
                as std::os::raw::c_float;
            x += 1;
            color = color.offset(1);
        }
        y += 1;
    }
    heman_image_destroy(normals);
    heman_image_destroy(occ);
    return final_0;
}
unsafe extern "C" fn azimuth_slope(mut a: crate::src::kazmath::aabb3::kmVec3, mut b: crate::src::kazmath::aabb3::kmVec3) -> std::os::raw::c_float {
    let mut d = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Subtract(&mut d, &mut a, &mut b);
    let mut x = kmVec3Length(&mut d);
    let mut y = b.z - a.z;
    return y / x;
}
unsafe extern "C" fn compute_occlusion(
    mut thispt: crate::src::kazmath::aabb3::kmVec3,
    mut horizonpt: crate::src::kazmath::aabb3::kmVec3,
) -> std::os::raw::c_float {
    let mut direction = kmVec3 { x: 0., y: 0., z: 0. };
    kmVec3Subtract(&mut direction, &mut horizonpt, &mut thispt);
    kmVec3Normalize(&mut direction, &mut direction);
    let mut dot = kmVec3Dot(&mut direction, &KM_VEC3_POS_Z);
    return (atan((if dot > 0.0f32 { dot } else { 0.0f32 }) as std::os::raw::c_double)
        * 0.63661977236f64) as std::os::raw::c_float;
}
unsafe extern "C" fn horizon_scan(
    mut heightmap: * mut crate::src::src::color::heman_image_s,
    mut result: * mut crate::src::src::color::heman_image_s,
    mut startpts: * mut std::os::raw::c_int,
    mut dx: std::os::raw::c_int,
    mut dy: std::os::raw::c_int,
) {
    let mut w = (*heightmap).width;
    let mut h = (*heightmap).height;
    let mut sx = (dx > 0 as std::os::raw::c_int) as std::os::raw::c_int
        - (dx < 0 as std::os::raw::c_int) as std::os::raw::c_int;
    let mut sy = (dy > 0 as std::os::raw::c_int) as std::os::raw::c_int
        - (dy < 0 as std::os::raw::c_int) as std::os::raw::c_int;
    let mut ax = abs(dx);
    let mut ay = abs(dy);
    let mut nsweeps = ay * w + ax * h - (ax + ay - 1 as std::os::raw::c_int);
    let mut p = startpts;
    let mut x = -ax;
    while x < w - ax {
        let mut y = -ay;
        while y < h - ay {
            if !(x >= 0 as std::os::raw::c_int && x < w && y >= 0 as std::os::raw::c_int && y < h) {
                let mut fresh2 = p;
                p = p.offset(1);
                *fresh2 = if sx < 0 as std::os::raw::c_int {
                    w - x - 1 as std::os::raw::c_int
                } else {
                    x
                };
                let mut fresh3 = p;
                p = p.offset(1);
                *fresh3 = if sy < 0 as std::os::raw::c_int {
                    h - y - 1 as std::os::raw::c_int
                } else {
                    y
                };
            }
            y += 1;
        }
        x += 1;
    }
    if nsweeps as std::os::raw::c_long
        == p.offset_from(startpts) as std::os::raw::c_long / 2 as std::os::raw::c_int as std::os::raw::c_long
    {} else {
        __assert_fail(
            b"nsweeps == (p - startpts) / 2\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
            160 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 65], &'_ [i8; 65]>(b"void horizon_scan(heman_image *, heman_image *, int *, int, int)\0"))
                .as_ptr(),
        );
    }
    let mut pathlen = 0 as std::os::raw::c_int;
    let mut i = *startpts.offset(0 as std::os::raw::c_int as isize);
    let mut j = *startpts.offset(1 as std::os::raw::c_int as isize);
    loop {
        i += dx;
        j += dy;
        pathlen += 1;
        if !(i >= 0 as std::os::raw::c_int && i < w && j >= 0 as std::os::raw::c_int && j < h) {
            break;
        }
    }
    let mut cellw = _occlusion_scale / (if w > h { w } else { h }) as std::os::raw::c_float;
    let mut cellh = _occlusion_scale / (if w > h { w } else { h }) as std::os::raw::c_float;
    let mut hull_buffer = malloc(
        (::std::mem::size_of::<kmVec3>() as std::os::raw::c_ulong)
            .wrapping_mul(pathlen as std::os::raw::c_ulong)
            .wrapping_mul(nsweeps as std::os::raw::c_ulong),
    ) as *mut kmVec3;
    let mut sweep: i32 = 0;
    sweep = 0 as std::os::raw::c_int;
    while sweep < nsweeps {
        let mut convex_hull = hull_buffer.offset((sweep * pathlen) as isize);
        let mut p_0 = startpts.offset((sweep * 2 as std::os::raw::c_int) as isize);
        let mut i_0 = *p_0.offset(0 as std::os::raw::c_int as isize);
        let mut j_0 = *p_0.offset(1 as std::os::raw::c_int as isize);
        let mut thispt = kmVec3 { x: 0., y: 0., z: 0. };
        let mut horizonpt = kmVec3 { x: 0., y: 0., z: 0. };
        thispt.x = i_0 as std::os::raw::c_float * cellw;
        thispt.y = j_0 as std::os::raw::c_float * cellh;
        thispt
            .z = *heman_image_texel(
            heightmap,
            if 0 as std::os::raw::c_int
                > (if w - 1 as std::os::raw::c_int > i_0 { i_0 } else { w - 1 as std::os::raw::c_int })
            {
                0 as std::os::raw::c_int
            } else if w - 1 as std::os::raw::c_int > i_0 {
                i_0
            } else {
                w - 1 as std::os::raw::c_int
            },
            if 0 as std::os::raw::c_int
                > (if h - 1 as std::os::raw::c_int > j_0 { j_0 } else { h - 1 as std::os::raw::c_int })
            {
                0 as std::os::raw::c_int
            } else if h - 1 as std::os::raw::c_int > j_0 {
                j_0
            } else {
                h - 1 as std::os::raw::c_int
            },
        );
        let mut stack_top = 0 as std::os::raw::c_int;
        *convex_hull.offset(0 as std::os::raw::c_int as isize) = thispt;
        i_0 += dx;
        j_0 += dy;
        while i_0 >= 0 as std::os::raw::c_int && i_0 < w && j_0 >= 0 as std::os::raw::c_int && j_0 < h {
            thispt.x = i_0 as std::os::raw::c_float * cellw;
            thispt.y = j_0 as std::os::raw::c_float * cellh;
            thispt.z = *heman_image_texel(heightmap, i_0, j_0);
            while stack_top > 0 as std::os::raw::c_int {
                let mut s1 = azimuth_slope(
                    thispt,
                    *convex_hull.offset(stack_top as isize),
                );
                let mut s2 = azimuth_slope(
                    thispt,
                    *convex_hull.offset((stack_top - 1 as std::os::raw::c_int) as isize),
                );
                if s1 >= s2 {
                    break;
                }
                stack_top -= 1;
            }
            let mut fresh4 = stack_top;
            stack_top = stack_top + 1;
            horizonpt = *convex_hull.offset(fresh4 as isize);
            if stack_top < pathlen {} else {
                __assert_fail(
                    b"stack_top < pathlen\0" as *const u8 as *const std::os::raw::c_char,
                    b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
                    213 as std::os::raw::c_int as std::os::raw::c_uint,
                    (*core::intrinsics::transmute::<&'_ [u8; 65], &'_ [i8; 65]>(
                        b"void horizon_scan(heman_image *, heman_image *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            *convex_hull.offset(stack_top as isize) = thispt;
            let mut occlusion = compute_occlusion(thispt, horizonpt);
            *heman_image_texel(result, i_0, j_0) += 1.0f32 / 16.0f32 * occlusion;
            i_0 += dx;
            j_0 += dy;
        }
        sweep += 1;
    }
    free(hull_buffer as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_lighting_compute_occlusion(
    mut heightmap: * mut crate::src::src::color::heman_image_s,
) -> * mut crate::src::src::color::heman_image_s {
    if (*heightmap).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"heightmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
            227 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 61], &'_ [i8; 61]>(b"heman_image *heman_lighting_compute_occlusion(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut width = (*heightmap).width;
    let mut height = (*heightmap).height;
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    memset(
        (*result).data as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(width as std::os::raw::c_ulong)
            .wrapping_mul(height as std::os::raw::c_ulong),
    );
    let scans: [i32; 32] = [
        1 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        -(1 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        -(1 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        -(2 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
        -(2 as std::os::raw::c_int),
        -(1 as std::os::raw::c_int),
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        -(2 as std::os::raw::c_int),
        -(1 as std::os::raw::c_int),
        2 as std::os::raw::c_int,
        -(1 as std::os::raw::c_int),
        -(2 as std::os::raw::c_int),
    ];
    let mut startpts = malloc(
        ((::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_float
            * kmMax(width as std::os::raw::c_float, height as std::os::raw::c_float)) as std::os::raw::c_ulong,
    ) as *mut std::os::raw::c_int;
    let mut i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        let mut dx = scans[(i * 2 as std::os::raw::c_int) as usize];
        let mut dy = scans[(i * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) as usize];
        horizon_scan(heightmap, result, startpts, dx, dy);
        i += 1;
    }
    let mut i_0 = 0 as std::os::raw::c_int;
    while i_0 < width * height {
        *((*result).data)
            .offset(i_0 as isize) = 1.0f32 - *((*result).data).offset(i_0 as isize);
        if *((*result).data).offset(i_0 as isize) as std::os::raw::c_double >= 0.0f64
            && *((*result).data).offset(i_0 as isize) <= 1.0f32
        {} else {
            __assert_fail(
                b"result->data[i] >= 0.0 && result->data[i] <= 1.0f\0" as *const u8
                    as *const std::os::raw::c_char,
                b"../src/lighting.c\0" as *const u8 as *const std::os::raw::c_char,
                253 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 61], &'_ [i8; 61]>(b"heman_image *heman_lighting_compute_occlusion(heman_image *)\0"))
                    .as_ptr(),
            );
        }
        i_0 += 1;
    }
    free(startpts as *mut std::os::raw::c_void);
    return result;
}
use crate::laertes_rt::*;
