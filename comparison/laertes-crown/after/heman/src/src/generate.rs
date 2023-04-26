
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn memset(
        _: * mut core::ffi::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
}
pub use crate::src::src::color::heman_color_from_cpcf;
pub use crate::src::src::distance::heman_distance_create_cpcf;
pub use crate::src::src::distance::heman_distance_create_sdf;
pub use crate::src::src::draw::heman_draw_colored_points;
pub use crate::src::src::draw::heman_draw_contour_from_points;
pub use crate::src::src::image::heman_image_clear;
pub use crate::src::src::image::heman_image_create;
pub use crate::src::src::image::heman_image_destroy;
pub use crate::src::src::image::heman_image_sample;
pub use crate::src::src::image::heman_image_texel;
pub use crate::src::src::noise::open_simplex_noise;
pub use crate::src::src::noise::open_simplex_noise2;
pub use crate::src::src::noise::open_simplex_noise3;
pub use crate::src::src::noise::open_simplex_noise_free;
pub use crate::src::src::ops::heman_ops_extract_mask;
pub use crate::src::src::ops::heman_ops_warp;
pub use crate::src::src::noise::osn_context;
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
pub type heman_points = crate::src::src::color::heman_image_s;
pub type heman_color = std::os::raw::c_uint;
pub type int64_t = std::os::raw::c_long;
pub type __int64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
static mut SEALEVEL: std::os::raw::c_float = 0.0; unsafe fn laertes_init_SEALEVEL() {
SEALEVEL = 0.5f64 as std::os::raw::c_float;}//;
static mut DEFAULT_STRENGTH: std::os::raw::c_float = 0.0; unsafe fn laertes_init_DEFAULT_STRENGTH() {
DEFAULT_STRENGTH = 0.6f64 as std::os::raw::c_float;}//;
#[no_mangle]
pub unsafe extern "C" fn heman_internal_generate_island_noise(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut img = heman_image_create(width, height, 3 as std::os::raw::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut invw = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut freqs: [f32; 5] = [
        4.0f64 as std::os::raw::c_float,
        16.0f64 as std::os::raw::c_float,
        32.0f64 as std::os::raw::c_float,
        64.0f64 as std::os::raw::c_float,
        128.0f64 as std::os::raw::c_float,
    ];
    let mut ampls: [f32; 5] = [
        0.2f64 as std::os::raw::c_float,
        0.1f64 as std::os::raw::c_float,
        0.05f64 as std::os::raw::c_float,
        0.025f64 as std::os::raw::c_float,
        0.0125f64 as std::os::raw::c_float,
    ];
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut v = y as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width * 3 as std::os::raw::c_int) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut u = x as std::os::raw::c_float * invw;
            let mut fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = (ampls[0 as std::os::raw::c_int as usize] as std::os::raw::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[0 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    (v * freqs[0 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                )
                + ampls[1 as std::os::raw::c_int as usize] as std::os::raw::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[1 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                        (v * freqs[1 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    )
                + ampls[2 as std::os::raw::c_int as usize] as std::os::raw::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[2 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                        (v * freqs[2 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    )) as std::os::raw::c_float;
            let mut fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = (ampls[3 as std::os::raw::c_int as usize] as std::os::raw::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[3 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    (v * freqs[3 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                )
                + ampls[4 as std::os::raw::c_int as usize] as std::os::raw::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[4 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                        (v * freqs[4 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    )) as std::os::raw::c_float;
            u = (u as std::os::raw::c_double + 0.5f64) as std::os::raw::c_float;
            let mut fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = (ampls[3 as std::os::raw::c_int as usize] as std::os::raw::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[3 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    (v * freqs[3 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                )
                + ampls[4 as std::os::raw::c_int as usize] as std::os::raw::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[4 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                        (v * freqs[4 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    )) as std::os::raw::c_float;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_generate_rock_noise(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut img = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut invw = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut freqs: [f32; 3] = [
        2.0f64 as std::os::raw::c_float,
        4.0f64 as std::os::raw::c_float,
        16.0f64 as std::os::raw::c_float,
    ];
    let mut ampls: [f32; 3] = [
        0.2f64 as std::os::raw::c_float,
        0.05f64 as std::os::raw::c_float,
        0.01f64 as std::os::raw::c_float,
    ];
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut v = y as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut u = x as std::os::raw::c_float * invw;
            let mut fresh3 = dst;
            dst = dst.offset(1);
            *fresh3 = (ampls[0 as std::os::raw::c_int as usize] as std::os::raw::c_double
                * open_simplex_noise2(
                    ctx,
                    (u * freqs[0 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    (v * freqs[0 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                )
                + ampls[1 as std::os::raw::c_int as usize] as std::os::raw::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[1 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                        (v * freqs[1 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    )
                + ampls[2 as std::os::raw::c_int as usize] as std::os::raw::c_double
                    * open_simplex_noise2(
                        ctx,
                        (u * freqs[2 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                        (v * freqs[2 as std::os::raw::c_int as usize]) as std::os::raw::c_double,
                    )) as std::os::raw::c_float;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_island_heightmap(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut data = (*coastmask).data;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut hh = height / 2 as std::os::raw::c_int;
    let mut hw = width / 2 as std::os::raw::c_int;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut vv = (y - hh) as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut n: [f32; 3] = [0.; 3];
            let mut v = y as std::os::raw::c_float * invh;
            let mut u = x as std::os::raw::c_float * invw;
            heman_image_sample(noisetex, u, v, n.as_mut_ptr());
            u = (x - hw) as std::os::raw::c_float * invw;
            v = vv;
            u += n[1 as std::os::raw::c_int as usize];
            v += n[2 as std::os::raw::c_int as usize];
            let mut m = (0.707f64 - sqrt((u * u + v * v) as std::os::raw::c_double))
                as std::os::raw::c_float;
            m += n[0 as std::os::raw::c_int as usize];
            let mut fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = (if m < SEALEVEL { 0 as std::os::raw::c_int } else { 1 as std::os::raw::c_int })
                as std::os::raw::c_float;
            x += 1;
        }
        y += 1;
    }
    let mut heightmap = heman_distance_create_sdf(coastmask);
    heman_image_destroy(coastmask);
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    data = (*result).data;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst_0 = data.offset((y * width) as isize);
        let mut x_0 = 0 as std::os::raw::c_int;
        while x_0 < width {
            let mut n_0: [f32; 3] = [0.; 3];
            let mut u_0 = x_0 as std::os::raw::c_float * invw;
            let mut v_0 = y as std::os::raw::c_float * invh;
            heman_image_sample(noisetex, u_0, v_0, n_0.as_mut_ptr());
            let mut z: f32 = 0.;
            heman_image_sample(heightmap, u_0, v_0, &mut z);
            if z as std::os::raw::c_double > 0.0f64 {
                let mut influence = z;
                u_0 += influence * n_0[1 as std::os::raw::c_int as usize];
                v_0 += influence * n_0[2 as std::os::raw::c_int as usize];
                heman_image_sample(heightmap, u_0, v_0, &mut z);
                z
                    += 6 as std::os::raw::c_int as std::os::raw::c_float * influence
                        * n_0[0 as std::os::raw::c_int as usize];
            }
            let mut fresh5 = dst_0;
            dst_0 = dst_0.offset(1);
            *fresh5 = z;
            x_0 += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    heman_image_destroy(heightmap);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_rock_heightmap(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut noisetex = heman_internal_generate_rock_noise(width, height, seed);
    let mut heightmap = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut data = (*heightmap).data;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut hh = height / 2 as std::os::raw::c_int;
    let mut hw = width / 2 as std::os::raw::c_int;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut vv = (y - hh) as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut v = y as std::os::raw::c_float * invh;
            let mut u = x as std::os::raw::c_float * invw;
            let mut n: f32 = 0.;
            heman_image_sample(noisetex, u, v, &mut n);
            u = (x - hw) as std::os::raw::c_float * invw;
            v = vv;
            let mut r = (0.3f64 + n as std::os::raw::c_double) as std::os::raw::c_float;
            if u * u + v * v > r * r {
                let mut fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = 0 as std::os::raw::c_int as std::os::raw::c_float;
            } else {
                let mut z = sqrt((r * r - u * u - v * v) as std::os::raw::c_double)
                    as std::os::raw::c_float;
                let mut fresh7 = dst;
                dst = dst.offset(1);
                *fresh7 = z;
            }
            x += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    return heightmap;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_simplex_fbm(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut frequency: std::os::raw::c_float,
    mut amplitude: std::os::raw::c_float,
    mut octaves: std::os::raw::c_int,
    mut lacunarity: std::os::raw::c_float,
    mut gain: std::os::raw::c_float,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut img = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut ampl = amplitude;
    let mut freq = frequency;
    memset(
        data as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(width as std::os::raw::c_ulong)
            .wrapping_mul(height as std::os::raw::c_ulong),
    );
    loop {
        let mut fresh8 = octaves;
        octaves = octaves - 1;
        if !(fresh8 != 0) {
            break;
        }
        let mut y: i32 = 0;
        y = 0 as std::os::raw::c_int;
        while y < height {
            let mut v = y as std::os::raw::c_float * invh;
            let mut dst = data.offset((y * width) as isize);
            let mut x = 0 as std::os::raw::c_int;
            while x < width {
                let mut u = x as std::os::raw::c_float * invw;
                let mut fresh9 = dst;
                dst = dst.offset(1);
                *fresh9 = (*fresh9 as std::os::raw::c_double
                    + ampl as std::os::raw::c_double
                        * open_simplex_noise2(
                            ctx,
                            (u * freq) as std::os::raw::c_double,
                            (v * freq) as std::os::raw::c_double,
                        )) as std::os::raw::c_float;
                x += 1;
            }
            y += 1;
        }
        ampl *= gain;
        freq *= lacunarity;
    }
    open_simplex_noise_free(ctx);
    return img;
}
unsafe extern "C" fn sphere<'a1>(
    mut u: std::os::raw::c_float,
    mut v: std::os::raw::c_float,
    mut r: std::os::raw::c_float,
    mut dst: Option<&'a1 mut crate::src::kazmath::aabb3::kmVec3>,
) {
    (*(borrow_mut(&mut dst)).unwrap())
        .x = (r as std::os::raw::c_double * sin(v as std::os::raw::c_double) * cos(u as std::os::raw::c_double))
        as std::os::raw::c_float;
    (*(borrow_mut(&mut dst)).unwrap()).y = (r as std::os::raw::c_double * cos(v as std::os::raw::c_double)) as std::os::raw::c_float;
    (*(borrow_mut(&mut dst)).unwrap())
        .z = (r as std::os::raw::c_double * -sin(v as std::os::raw::c_double) * sin(u as std::os::raw::c_double))
        as std::os::raw::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_planet_heightmap(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut scalex = (2.0f32 as std::os::raw::c_double * 3.1415926535f64
        / width as std::os::raw::c_double) as std::os::raw::c_float;
    let mut scaley = (3.1415926535f64 / height as std::os::raw::c_double) as std::os::raw::c_float;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = ((*result).data).offset((y * width) as isize);
        let mut p = kmVec3 { x: 0., y: 0., z: 0. };
        let mut v = y as std::os::raw::c_float * invh;
        let mut s = 0.95f64 as std::os::raw::c_float;
        let mut antarctic_influence = (if (10 as std::os::raw::c_int as std::os::raw::c_float * (v - s)
            / s) as std::os::raw::c_double > -0.5f64
        {
            (10 as std::os::raw::c_int as std::os::raw::c_float * (v - s) / s) as std::os::raw::c_double
        } else {
            -0.5f64
        }) as std::os::raw::c_float;
        v = fabs(v as std::os::raw::c_double - 0.5f64) as std::os::raw::c_float;
        v = (1.5f64 * (0.5f64 - v as std::os::raw::c_double)) as std::os::raw::c_float;
        let mut equatorial_influence = v * v;
        v = y as std::os::raw::c_float * scaley;
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut u = x as std::os::raw::c_float * scalex;
            let mut freq = 1 as std::os::raw::c_int as std::os::raw::c_float;
            let mut amp = 1 as std::os::raw::c_int as std::os::raw::c_float;
            let mut h = antarctic_influence + equatorial_influence;
            let mut oct = 0 as std::os::raw::c_int;
            while oct < 6 as std::os::raw::c_int {
                sphere(u, v, freq, Some(&mut p));
                h = (h as std::os::raw::c_double
                    + amp as std::os::raw::c_double
                        * open_simplex_noise3(
                            ctx,
                            p.x as std::os::raw::c_double,
                            p.y as std::os::raw::c_double,
                            p.z as std::os::raw::c_double,
                        )) as std::os::raw::c_float;
                amp = (amp as std::os::raw::c_double * 0.5f64) as std::os::raw::c_float;
                freq *= 2 as std::os::raw::c_int as std::os::raw::c_float;
                oct += 1;
            }
            let mut fresh10 = dst;
            dst = dst.offset(1);
            *fresh10 = h;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_draw_seeds(
    mut target: * mut crate::src::src::color::heman_image_s,
    mut pts: * mut crate::src::src::color::heman_image_s,
    mut filterd: std::os::raw::c_int,
) {
    let mut radius = (*target).width / filterd;
    let mut fwidth = radius * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int;
    let mut src = (*pts).data;
    let mut w = (*target).width;
    let mut h = (*target).height;
    let mut i = 0 as std::os::raw::c_int;
    while i < (*pts).width {
        let mut fresh11 = src;
        src = src.offset(1);
        let mut x = *fresh11;
        let mut fresh12 = src;
        src = src.offset(1);
        let mut y = *fresh12;
        let mut strength = DEFAULT_STRENGTH;
        if (*pts).nbands == 3 as std::os::raw::c_int {
            let mut fresh13 = src;
            src = src.offset(1);
            strength = *fresh13;
        }
        strength = (SEALEVEL as std::os::raw::c_double + strength as std::os::raw::c_double * 0.1f64)
            as std::os::raw::c_float;
        let mut ix = (x * w as std::os::raw::c_float) as std::os::raw::c_int;
        let mut iy = (y * h as std::os::raw::c_float) as std::os::raw::c_int;
        let mut ii = ix - radius;
        let mut jj = iy - radius;
        let mut kj = 0 as std::os::raw::c_int;
        while kj < fwidth {
            let mut ki = 0 as std::os::raw::c_int;
            while ki < fwidth {
                let mut i_0 = ii + ki;
                let mut j = jj + kj;
                if !(i_0 < 0 as std::os::raw::c_int || i_0 >= w || j < 0 as std::os::raw::c_int
                    || j >= h)
                {
                    let mut texel = heman_image_texel(target, i_0, j);
                    let mut d2 = (i_0 - ix) * (i_0 - ix) + (j - iy) * (j - iy);
                    let mut dist = (1 as std::os::raw::c_int as std::os::raw::c_double
                        - sqrt(d2 as std::os::raw::c_double) / radius as std::os::raw::c_double)
                        as std::os::raw::c_float;
                    *texel = if *texel > strength * dist {
                        *texel
                    } else {
                        strength * dist
                    };
                }
                ki += 1;
            }
            kj += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_heightmap(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut points: * mut crate::src::src::color::heman_image_s,
    mut noiseamt: std::os::raw::c_float,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask = heman_image_create(width, height, 1 as std::os::raw::c_int);
    heman_image_clear(coastmask, 0 as std::os::raw::c_int as std::os::raw::c_float);
    heman_internal_draw_seeds(coastmask, points, 1 as std::os::raw::c_int);
    let mut data = (*coastmask).data;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut hh = height / 2 as std::os::raw::c_int;
    let mut hw = width / 2 as std::os::raw::c_int;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut vv = (y - hh) as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut n: [f32; 3] = [0 as std::os::raw::c_int as std::os::raw::c_float, 0., 0.];
            let mut v = y as std::os::raw::c_float * invh;
            let mut u = x as std::os::raw::c_float * invw;
            heman_image_sample(noisetex, u, v, n.as_mut_ptr());
            u = (x - hw) as std::os::raw::c_float * invw;
            v = vv;
            u += noiseamt * n[1 as std::os::raw::c_int as usize];
            v += noiseamt * n[2 as std::os::raw::c_int as usize];
            let mut m = *dst;
            m += noiseamt * n[0 as std::os::raw::c_int as usize];
            let mut fresh14 = dst;
            dst = dst.offset(1);
            *fresh14 = (if m < SEALEVEL { 0 as std::os::raw::c_int } else { 1 as std::os::raw::c_int })
                as std::os::raw::c_float;
            x += 1;
        }
        y += 1;
    }
    let mut heightmap = heman_distance_create_sdf(coastmask);
    heman_image_destroy(coastmask);
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    data = (*result).data;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst_0 = data.offset((y * width) as isize);
        let mut x_0 = 0 as std::os::raw::c_int;
        while x_0 < width {
            let mut n_0: [f32; 3] = [0.; 3];
            let mut u_0 = x_0 as std::os::raw::c_float * invw;
            let mut v_0 = y as std::os::raw::c_float * invh;
            heman_image_sample(noisetex, u_0, v_0, n_0.as_mut_ptr());
            let mut z: f32 = 0.;
            heman_image_sample(heightmap, u_0, v_0, &mut z);
            if z as std::os::raw::c_double > 0.0f64 {
                let mut influence = z;
                u_0 += influence * n_0[1 as std::os::raw::c_int as usize];
                v_0 += influence * n_0[2 as std::os::raw::c_int as usize];
                heman_image_sample(heightmap, u_0, v_0, &mut z);
                z
                    += 6 as std::os::raw::c_int as std::os::raw::c_float * influence
                        * n_0[0 as std::os::raw::c_int as usize];
            }
            let mut fresh15 = dst_0;
            dst_0 = dst_0.offset(1);
            *fresh15 = z;
            x_0 += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    heman_image_destroy(heightmap);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_1(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut points: * mut crate::src::src::color::heman_image_s,
    mut colors: * const std::os::raw::c_uint,
    mut ocean: std::os::raw::c_uint,
    mut seed: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut contour = heman_image_create(width, height, 3 as std::os::raw::c_int);
    heman_image_clear(contour, 0 as std::os::raw::c_int as std::os::raw::c_float);
    heman_draw_contour_from_points(
        contour,
        points,
        ocean,
        0.40f64 as std::os::raw::c_float,
        0.41f64 as std::os::raw::c_float,
        1 as std::os::raw::c_int,
    );
    heman_draw_colored_points(contour, points, colors);
    let mut cf = heman_distance_create_cpcf(contour);
    let mut warped_cpcf = heman_ops_warp(cf, seed, 4 as std::os::raw::c_int);
    let mut political = heman_color_from_cpcf(warped_cpcf, contour);
    heman_image_destroy(warped_cpcf);
    heman_image_destroy(cf);
    heman_image_destroy(contour);
    return political;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_2(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut ocean: std::os::raw::c_uint,
    mut seed: std::os::raw::c_int,
    mut political: * mut crate::src::src::color::heman_image_s,
    mut invert: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut coastmask = heman_ops_extract_mask(political, ocean, invert);
    let mut sdf = heman_distance_create_sdf(coastmask);
    heman_image_destroy(coastmask);
    let mut elevation = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut data = (*elevation).data;
    let mut invw = (1.0f64 / width as std::os::raw::c_double) as std::os::raw::c_float;
    let mut invh = (1.0f64 / height as std::os::raw::c_double) as std::os::raw::c_float;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut n: [f32; 3] = [0.; 3];
            let mut u = x as std::os::raw::c_float * invw;
            let mut v = y as std::os::raw::c_float * invh;
            heman_image_sample(noisetex, u, v, n.as_mut_ptr());
            let mut z: f32 = 0.;
            heman_image_sample(sdf, u, v, &mut z);
            if z as std::os::raw::c_double > 0.0f64 {
                let mut influence = z;
                u += influence * n[1 as std::os::raw::c_int as usize];
                v += influence * n[2 as std::os::raw::c_int as usize];
                heman_image_sample(sdf, u, v, &mut z);
                z
                    += 6 as std::os::raw::c_int as std::os::raw::c_float * influence
                        * n[0 as std::os::raw::c_int as usize];
            }
            let mut fresh16 = dst;
            dst = dst.offset(1);
            *fresh16 = z;
            x += 1;
        }
        y += 1;
    }
    heman_image_destroy(noisetex);
    heman_image_destroy(sdf);
    return elevation;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_3(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut colors: * const std::os::raw::c_uint,
    mut ncolors: std::os::raw::c_int,
    mut ocean: std::os::raw::c_uint,
    mut seed: std::os::raw::c_int,
    mut political: * mut crate::src::src::color::heman_image_s,
) -> * mut crate::src::src::color::heman_image_s {
    let mut elevations = malloc(
        (::std::mem::size_of::<*mut heman_image>() as std::os::raw::c_ulong)
            .wrapping_mul(ncolors as std::os::raw::c_ulong),
    ) as *mut *mut heman_image;
    let mut cindex = 0 as std::os::raw::c_int;
    while cindex < ncolors {
        let ref mut fresh17 = *elevations.offset(cindex as isize);
        *fresh17 = heman_generate_archipelago_political_2(
            width,
            height,
            *colors.offset(cindex as isize),
            seed,
            political,
            1 as std::os::raw::c_int,
        );
        cindex += 1;
    }
    let mut elevation = heman_image_create(width, height, 1 as std::os::raw::c_int);
    heman_image_clear(elevation, 0 as std::os::raw::c_int as std::os::raw::c_float);
    let mut cindex_0 = 0 as std::os::raw::c_int;
    while cindex_0 < ncolors {
        let mut y: i32 = 0;
        y = 0 as std::os::raw::c_int;
        while y < height {
            let mut dst = ((*elevation).data).offset((y * width) as isize);
            let mut src = ((**elevations.offset(cindex_0 as isize)).data)
                .offset((y * width) as isize);
            let mut x = 0 as std::os::raw::c_int;
            while x < width {
                *dst = if *src > *dst { *src } else { *dst };
                x += 1;
                dst = dst.offset(1);
                src = src.offset(1);
            }
            y += 1;
        }
        heman_image_destroy(*elevations.offset(cindex_0 as isize));
        cindex_0 += 1;
    }
    free(elevations as *mut std::os::raw::c_void);
    let mut ocean_elevation = heman_generate_archipelago_political_2(
        width,
        height,
        ocean,
        seed,
        political,
        0 as std::os::raw::c_int,
    );
    let mut y_0: i32 = 0;
    y_0 = 0 as std::os::raw::c_int;
    while y_0 < height {
        let mut dst_0 = ((*elevation).data).offset((y_0 * width) as isize);
        let mut src_0 = ((*ocean_elevation).data).offset((y_0 * width) as isize);
        let mut x_0 = 0 as std::os::raw::c_int;
        while x_0 < width {
            if *src_0 < 0 as std::os::raw::c_int as std::os::raw::c_float {
                *dst_0 = *src_0;
            }
            x_0 += 1;
            dst_0 = dst_0.offset(1);
            src_0 = src_0.offset(1);
        }
        y_0 += 1;
    }
    heman_image_destroy(ocean_elevation);
    return elevation;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political<'a1, 'a2>(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut points: * mut crate::src::src::color::heman_image_s,
    mut colors: * const std::os::raw::c_uint,
    mut ocean: std::os::raw::c_uint,
    mut seed: std::os::raw::c_int,
    mut elevation: Option<&'a1 mut * mut crate::src::src::color::heman_image_s>,
    mut political: Option<&'a2 mut * mut crate::src::src::color::heman_image_s>,
    mut elevation_mode: std::os::raw::c_int,
) {
    *(borrow_mut(&mut political)).unwrap() = heman_generate_archipelago_political_1(
        width,
        height,
        points,
        colors,
        ocean,
        seed,
    );
    if elevation_mode == 0 as std::os::raw::c_int {
        *(borrow_mut(&mut elevation)).unwrap() = heman_generate_archipelago_political_2(
            width,
            height,
            ocean,
            seed,
            *(borrow_mut(&mut political)).unwrap(),
            0 as std::os::raw::c_int,
        );
    } else {
        let mut ncolors = (*points).width;
        *(borrow_mut(&mut elevation)).unwrap() = heman_generate_archipelago_political_3(
            width,
            height,
            colors,
            ncolors,
            ocean,
            seed,
            *(borrow_mut(&mut political)).unwrap(),
        );
    };
}
use crate::laertes_rt::*;
