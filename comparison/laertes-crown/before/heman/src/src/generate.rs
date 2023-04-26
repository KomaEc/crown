
extern "C" {
    pub type osn_context;
    fn heman_image_create(
        width: std::os::raw::c_int,
        height: std::os::raw::c_int,
        nbands: std::os::raw::c_int,
    ) -> *mut heman_image;
    fn heman_image_texel(
        _: *mut heman_image,
        x: std::os::raw::c_int,
        y: std::os::raw::c_int,
    ) -> *mut std::os::raw::c_float;
    fn heman_image_sample(
        _: *mut heman_image,
        u: std::os::raw::c_float,
        v: std::os::raw::c_float,
        result: *mut std::os::raw::c_float,
    );
    fn heman_image_clear(_: *mut heman_image, value: std::os::raw::c_float);
    fn heman_image_destroy(_: *mut heman_image);
    fn heman_color_from_cpcf(
        cfield: *mut heman_image,
        texture: *mut heman_image,
    ) -> *mut heman_image;
    fn heman_distance_create_sdf(monochrome: *mut heman_image) -> *mut heman_image;
    fn heman_distance_create_cpcf(seed: *mut heman_image) -> *mut heman_image;
    fn heman_ops_warp(
        src: *mut heman_image,
        seed: std::os::raw::c_int,
        octaves: std::os::raw::c_int,
    ) -> *mut heman_image;
    fn heman_ops_extract_mask(
        src: *mut heman_image,
        color: heman_color,
        invert: std::os::raw::c_int,
    ) -> *mut heman_image;
    fn heman_draw_colored_points(
        target: *mut heman_image,
        coords: *mut heman_points,
        colors: *const heman_color,
    );
    fn heman_draw_contour_from_points(
        target: *mut heman_image,
        coords: *mut heman_points,
        color: heman_color,
        mind: std::os::raw::c_float,
        maxd: std::os::raw::c_float,
        filterd: std::os::raw::c_int,
    );
    fn open_simplex_noise_free(ctx: *mut osn_context);
    fn open_simplex_noise3(
        ctx: *mut osn_context,
        x: std::os::raw::c_double,
        y: std::os::raw::c_double,
        z: std::os::raw::c_double,
    ) -> std::os::raw::c_double;
    fn open_simplex_noise2(
        ctx: *mut osn_context,
        x: std::os::raw::c_double,
        y: std::os::raw::c_double,
    ) -> std::os::raw::c_double;
    fn open_simplex_noise(seed: int64_t, ctx: *mut *mut osn_context) -> std::os::raw::c_int;
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    fn memset(
        _: *mut std::os::raw::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub nbands: std::os::raw::c_int,
    pub data: *mut std::os::raw::c_float,
}
pub type heman_image = heman_image_s;
pub type heman_points = heman_image_s;
pub type heman_color = std::os::raw::c_uint;
pub type int64_t = __int64_t;
pub type __int64_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub z: std::os::raw::c_float,
}
static mut SEALEVEL: std::os::raw::c_float = 0.5f64 as std::os::raw::c_float;
static mut DEFAULT_STRENGTH: std::os::raw::c_float = 0.6f64 as std::os::raw::c_float;
#[no_mangle]
pub unsafe extern "C" fn heman_internal_generate_island_noise(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut seed: std::os::raw::c_int,
) -> *mut heman_image {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, &mut ctx);
    let mut img = heman_image_create(width, height, 3 as std::os::raw::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut invw = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut freqs: [std::os::raw::c_float; 5] = [
        4.0f64 as std::os::raw::c_float,
        16.0f64 as std::os::raw::c_float,
        32.0f64 as std::os::raw::c_float,
        64.0f64 as std::os::raw::c_float,
        128.0f64 as std::os::raw::c_float,
    ];
    let mut ampls: [std::os::raw::c_float; 5] = [
        0.2f64 as std::os::raw::c_float,
        0.1f64 as std::os::raw::c_float,
        0.05f64 as std::os::raw::c_float,
        0.025f64 as std::os::raw::c_float,
        0.0125f64 as std::os::raw::c_float,
    ];
    let mut y: std::os::raw::c_int = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut v = y as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width * 3 as std::os::raw::c_int) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut u = x as std::os::raw::c_float * invw;
            let fresh0 = dst;
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
            let fresh1 = dst;
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
            let fresh2 = dst;
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
) -> *mut heman_image {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, &mut ctx);
    let mut img = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut invw = 1.0f32
        / (if width > height { width } else { height }) as std::os::raw::c_float;
    let mut freqs: [std::os::raw::c_float; 3] = [
        2.0f64 as std::os::raw::c_float,
        4.0f64 as std::os::raw::c_float,
        16.0f64 as std::os::raw::c_float,
    ];
    let mut ampls: [std::os::raw::c_float; 3] = [
        0.2f64 as std::os::raw::c_float,
        0.05f64 as std::os::raw::c_float,
        0.01f64 as std::os::raw::c_float,
    ];
    let mut y: std::os::raw::c_int = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut v = y as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut u = x as std::os::raw::c_float * invw;
            let fresh3 = dst;
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
) -> *mut heman_image {
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut data = (*coastmask).data;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut hh = height / 2 as std::os::raw::c_int;
    let mut hw = width / 2 as std::os::raw::c_int;
    let mut y: std::os::raw::c_int = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut vv = (y - hh) as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut n: [std::os::raw::c_float; 3] = [0.; 3];
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
            let fresh4 = dst;
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
            let mut n_0: [std::os::raw::c_float; 3] = [0.; 3];
            let mut u_0 = x_0 as std::os::raw::c_float * invw;
            let mut v_0 = y as std::os::raw::c_float * invh;
            heman_image_sample(noisetex, u_0, v_0, n_0.as_mut_ptr());
            let mut z: std::os::raw::c_float = 0.;
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
            let fresh5 = dst_0;
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
) -> *mut heman_image {
    let mut noisetex = heman_internal_generate_rock_noise(width, height, seed);
    let mut heightmap = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut data = (*heightmap).data;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut hh = height / 2 as std::os::raw::c_int;
    let mut hw = width / 2 as std::os::raw::c_int;
    let mut y: std::os::raw::c_int = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut vv = (y - hh) as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut v = y as std::os::raw::c_float * invh;
            let mut u = x as std::os::raw::c_float * invw;
            let mut n: std::os::raw::c_float = 0.;
            heman_image_sample(noisetex, u, v, &mut n);
            u = (x - hw) as std::os::raw::c_float * invw;
            v = vv;
            let mut r = (0.3f64 + n as std::os::raw::c_double) as std::os::raw::c_float;
            if u * u + v * v > r * r {
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = 0 as std::os::raw::c_int as std::os::raw::c_float;
            } else {
                let mut z = sqrt((r * r - u * u - v * v) as std::os::raw::c_double)
                    as std::os::raw::c_float;
                let fresh7 = dst;
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
) -> *mut heman_image {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, &mut ctx);
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
        let fresh8 = octaves;
        octaves = octaves - 1;
        if !(fresh8 != 0) {
            break;
        }
        let mut y: std::os::raw::c_int = 0;
        y = 0 as std::os::raw::c_int;
        while y < height {
            let mut v = y as std::os::raw::c_float * invh;
            let mut dst = data.offset((y * width) as isize);
            let mut x = 0 as std::os::raw::c_int;
            while x < width {
                let mut u = x as std::os::raw::c_float * invw;
                let fresh9 = dst;
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
unsafe extern "C" fn sphere(
    mut u: std::os::raw::c_float,
    mut v: std::os::raw::c_float,
    mut r: std::os::raw::c_float,
    mut dst: *mut kmVec3,
) {
    (*dst)
        .x = (r as std::os::raw::c_double * sin(v as std::os::raw::c_double) * cos(u as std::os::raw::c_double))
        as std::os::raw::c_float;
    (*dst).y = (r as std::os::raw::c_double * cos(v as std::os::raw::c_double)) as std::os::raw::c_float;
    (*dst)
        .z = (r as std::os::raw::c_double * -sin(v as std::os::raw::c_double) * sin(u as std::os::raw::c_double))
        as std::os::raw::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_planet_heightmap(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut seed: std::os::raw::c_int,
) -> *mut heman_image {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, &mut ctx);
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut scalex = (2.0f32 as std::os::raw::c_double * 3.1415926535f64
        / width as std::os::raw::c_double) as std::os::raw::c_float;
    let mut scaley = (3.1415926535f64 / height as std::os::raw::c_double) as std::os::raw::c_float;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut y: std::os::raw::c_int = 0;
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
                sphere(u, v, freq, &mut p);
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
            let fresh10 = dst;
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
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut filterd: std::os::raw::c_int,
) {
    let mut radius = (*target).width / filterd;
    let mut fwidth = radius * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int;
    let mut src = (*pts).data;
    let mut w = (*target).width;
    let mut h = (*target).height;
    let mut i = 0 as std::os::raw::c_int;
    while i < (*pts).width {
        let fresh11 = src;
        src = src.offset(1);
        let mut x = *fresh11;
        let fresh12 = src;
        src = src.offset(1);
        let mut y = *fresh12;
        let mut strength = DEFAULT_STRENGTH;
        if (*pts).nbands == 3 as std::os::raw::c_int {
            let fresh13 = src;
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
    mut points: *mut heman_points,
    mut noiseamt: std::os::raw::c_float,
    mut seed: std::os::raw::c_int,
) -> *mut heman_image {
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask = heman_image_create(width, height, 1 as std::os::raw::c_int);
    heman_image_clear(coastmask, 0 as std::os::raw::c_int as std::os::raw::c_float);
    heman_internal_draw_seeds(coastmask, points, 1 as std::os::raw::c_int);
    let mut data = (*coastmask).data;
    let mut invh = 1.0f32 / height as std::os::raw::c_float;
    let mut invw = 1.0f32 / width as std::os::raw::c_float;
    let mut hh = height / 2 as std::os::raw::c_int;
    let mut hw = width / 2 as std::os::raw::c_int;
    let mut y: std::os::raw::c_int = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut vv = (y - hh) as std::os::raw::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut n: [std::os::raw::c_float; 3] = [0 as std::os::raw::c_int as std::os::raw::c_float, 0., 0.];
            let mut v = y as std::os::raw::c_float * invh;
            let mut u = x as std::os::raw::c_float * invw;
            heman_image_sample(noisetex, u, v, n.as_mut_ptr());
            u = (x - hw) as std::os::raw::c_float * invw;
            v = vv;
            u += noiseamt * n[1 as std::os::raw::c_int as usize];
            v += noiseamt * n[2 as std::os::raw::c_int as usize];
            let mut m = *dst;
            m += noiseamt * n[0 as std::os::raw::c_int as usize];
            let fresh14 = dst;
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
            let mut n_0: [std::os::raw::c_float; 3] = [0.; 3];
            let mut u_0 = x_0 as std::os::raw::c_float * invw;
            let mut v_0 = y as std::os::raw::c_float * invh;
            heman_image_sample(noisetex, u_0, v_0, n_0.as_mut_ptr());
            let mut z: std::os::raw::c_float = 0.;
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
            let fresh15 = dst_0;
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
    mut points: *mut heman_points,
    mut colors: *const heman_color,
    mut ocean: heman_color,
    mut seed: std::os::raw::c_int,
) -> *mut heman_image {
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
    mut ocean: heman_color,
    mut seed: std::os::raw::c_int,
    mut political: *mut heman_image,
    mut invert: std::os::raw::c_int,
) -> *mut heman_image {
    let mut coastmask = heman_ops_extract_mask(political, ocean, invert);
    let mut sdf = heman_distance_create_sdf(coastmask);
    heman_image_destroy(coastmask);
    let mut elevation = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut data = (*elevation).data;
    let mut invw = (1.0f64 / width as std::os::raw::c_double) as std::os::raw::c_float;
    let mut invh = (1.0f64 / height as std::os::raw::c_double) as std::os::raw::c_float;
    let mut y: std::os::raw::c_int = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut n: [std::os::raw::c_float; 3] = [0.; 3];
            let mut u = x as std::os::raw::c_float * invw;
            let mut v = y as std::os::raw::c_float * invh;
            heman_image_sample(noisetex, u, v, n.as_mut_ptr());
            let mut z: std::os::raw::c_float = 0.;
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
            let fresh16 = dst;
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
    mut colors: *const heman_color,
    mut ncolors: std::os::raw::c_int,
    mut ocean: heman_color,
    mut seed: std::os::raw::c_int,
    mut political: *mut heman_image,
) -> *mut heman_image {
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
        let mut y: std::os::raw::c_int = 0;
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
    let mut y_0: std::os::raw::c_int = 0;
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
pub unsafe extern "C" fn heman_generate_archipelago_political(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut points: *mut heman_points,
    mut colors: *const heman_color,
    mut ocean: heman_color,
    mut seed: std::os::raw::c_int,
    mut elevation: *mut *mut heman_image,
    mut political: *mut *mut heman_image,
    mut elevation_mode: std::os::raw::c_int,
) {
    *political = heman_generate_archipelago_political_1(
        width,
        height,
        points,
        colors,
        ocean,
        seed,
    );
    if elevation_mode == 0 as std::os::raw::c_int {
        *elevation = heman_generate_archipelago_political_2(
            width,
            height,
            ocean,
            seed,
            *political,
            0 as std::os::raw::c_int,
        );
    } else {
        let mut ncolors = (*points).width;
        *elevation = heman_generate_archipelago_political_3(
            width,
            height,
            colors,
            ncolors,
            ocean,
            seed,
            *political,
        );
    };
}
