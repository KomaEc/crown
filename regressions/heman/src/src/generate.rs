use ::libc;
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor27 { dummy: () }
pub type heman_image = crate::src::src::color::heman_image_s;
pub type heman_points = crate::src::src::color::heman_image_s;
pub type heman_color = libc::c_uint;
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor28 { dummy: () }
static mut SEALEVEL: libc::c_float = 0.5f64 as libc::c_float;
static mut DEFAULT_STRENGTH: libc::c_float = 0.6f64 as libc::c_float;
#[no_mangle]
pub unsafe extern "C" fn heman_internal_generate_island_noise(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut seed: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut ctx = 0 as *mut crate::src::src::noise::osn_context;
    crate::src::src::noise::open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut img = crate::src::src::image::heman_image_create(width, height, 3 as libc::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32
        / (if width > height { width } else { height }) as libc::c_float;
    let mut invw = 1.0f32
        / (if width > height { width } else { height }) as libc::c_float;
    let mut freqs: [libc::c_float; 5] = [
        4.0f64 as libc::c_float,
        16.0f64 as libc::c_float,
        32.0f64 as libc::c_float,
        64.0f64 as libc::c_float,
        128.0f64 as libc::c_float,
    ];
    let mut ampls: [libc::c_float; 5] = [
        0.2f64 as libc::c_float,
        0.1f64 as libc::c_float,
        0.05f64 as libc::c_float,
        0.025f64 as libc::c_float,
        0.0125f64 as libc::c_float,
    ];
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut v = y as libc::c_float * invh;
        let mut dst = data.offset((y * width * 3 as libc::c_int) as isize);
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut u = x as libc::c_float * invw;
            let fresh0 = dst;
            dst= dst.offset(1);
            *fresh0= (ampls[0 as libc::c_int as usize] as libc::c_double
                * crate::src::src::noise::open_simplex_noise2(
                    ctx,
                    (u * freqs[0 as libc::c_int as usize]) as libc::c_double,
                    (v * freqs[0 as libc::c_int as usize]) as libc::c_double,
                )
                + ampls[1 as libc::c_int as usize] as libc::c_double
                    * crate::src::src::noise::open_simplex_noise2(
                        ctx,
                        (u * freqs[1 as libc::c_int as usize]) as libc::c_double,
                        (v * freqs[1 as libc::c_int as usize]) as libc::c_double,
                    )
                + ampls[2 as libc::c_int as usize] as libc::c_double
                    * crate::src::src::noise::open_simplex_noise2(
                        ctx,
                        (u * freqs[2 as libc::c_int as usize]) as libc::c_double,
                        (v * freqs[2 as libc::c_int as usize]) as libc::c_double,
                    )) as libc::c_float;
            let fresh1 = dst;
            dst= dst.offset(1);
            *fresh1= (ampls[3 as libc::c_int as usize] as libc::c_double
                * crate::src::src::noise::open_simplex_noise2(
                    ctx,
                    (u * freqs[3 as libc::c_int as usize]) as libc::c_double,
                    (v * freqs[3 as libc::c_int as usize]) as libc::c_double,
                )
                + ampls[4 as libc::c_int as usize] as libc::c_double
                    * crate::src::src::noise::open_simplex_noise2(
                        ctx,
                        (u * freqs[4 as libc::c_int as usize]) as libc::c_double,
                        (v * freqs[4 as libc::c_int as usize]) as libc::c_double,
                    )) as libc::c_float;
            u= (u as libc::c_double + 0.5f64) as libc::c_float;
            let fresh2 = dst;
            dst= dst.offset(1);
            *fresh2= (ampls[3 as libc::c_int as usize] as libc::c_double
                * crate::src::src::noise::open_simplex_noise2(
                    ctx,
                    (u * freqs[3 as libc::c_int as usize]) as libc::c_double,
                    (v * freqs[3 as libc::c_int as usize]) as libc::c_double,
                )
                + ampls[4 as libc::c_int as usize] as libc::c_double
                    * crate::src::src::noise::open_simplex_noise2(
                        ctx,
                        (u * freqs[4 as libc::c_int as usize]) as libc::c_double,
                        (v * freqs[4 as libc::c_int as usize]) as libc::c_double,
                    )) as libc::c_float;
            x+= 1;
        }
        y+= 1;
    }
    crate::src::src::noise::open_simplex_noise_free(ctx);
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_generate_rock_noise(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut seed: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut ctx = 0 as *mut crate::src::src::noise::osn_context;
    crate::src::src::noise::open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut img = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32
        / (if width > height { width } else { height }) as libc::c_float;
    let mut invw = 1.0f32
        / (if width > height { width } else { height }) as libc::c_float;
    let mut freqs: [libc::c_float; 3] = [
        2.0f64 as libc::c_float,
        4.0f64 as libc::c_float,
        16.0f64 as libc::c_float,
    ];
    let mut ampls: [libc::c_float; 3] = [
        0.2f64 as libc::c_float,
        0.05f64 as libc::c_float,
        0.01f64 as libc::c_float,
    ];
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut v = y as libc::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut u = x as libc::c_float * invw;
            let fresh3 = dst;
            dst= dst.offset(1);
            *fresh3= (ampls[0 as libc::c_int as usize] as libc::c_double
                * crate::src::src::noise::open_simplex_noise2(
                    ctx,
                    (u * freqs[0 as libc::c_int as usize]) as libc::c_double,
                    (v * freqs[0 as libc::c_int as usize]) as libc::c_double,
                )
                + ampls[1 as libc::c_int as usize] as libc::c_double
                    * crate::src::src::noise::open_simplex_noise2(
                        ctx,
                        (u * freqs[1 as libc::c_int as usize]) as libc::c_double,
                        (v * freqs[1 as libc::c_int as usize]) as libc::c_double,
                    )
                + ampls[2 as libc::c_int as usize] as libc::c_double
                    * crate::src::src::noise::open_simplex_noise2(
                        ctx,
                        (u * freqs[2 as libc::c_int as usize]) as libc::c_double,
                        (v * freqs[2 as libc::c_int as usize]) as libc::c_double,
                    )) as libc::c_float;
            x+= 1;
        }
        y+= 1;
    }
    crate::src::src::noise::open_simplex_noise_free(ctx);
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_island_heightmap(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut seed: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    let mut data = (*coastmask).data;
    let mut invh = 1.0f32 / height as libc::c_float;
    let mut invw = 1.0f32 / width as libc::c_float;
    let mut hh = height / 2 as libc::c_int;
    let mut hw = width / 2 as libc::c_int;
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut vv = (y - hh) as libc::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut n: [libc::c_float; 3] = [0.; 3];
            let mut v = y as libc::c_float * invh;
            let mut u = x as libc::c_float * invw;
            crate::src::src::image::heman_image_sample(noisetex.as_mut(), u, v, n.as_mut_ptr());
            u= (x - hw) as libc::c_float * invw;
            v= vv;
            u+= n[1 as libc::c_int as usize];
            v+= n[2 as libc::c_int as usize];
            let mut m = (0.707f64 - sqrt((u * u + v * v) as libc::c_double))
                as libc::c_float;
            m+= n[0 as libc::c_int as usize];
            let fresh4 = dst;
            dst= dst.offset(1);
            *fresh4= (if m < crate::src::src::generate::SEALEVEL { 0 as libc::c_int } else { 1 as libc::c_int })
                as libc::c_float;
            x+= 1;
        }
        y+= 1;
    }
    let mut heightmap = crate::src::src::distance::heman_distance_create_sdf(coastmask);
    crate::src::src::image::heman_image_destroy(coastmask);
    let mut result = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    data= (*result).data;
    y= 0 as libc::c_int;
    while y < height {
        let mut dst_0 = data.offset((y * width) as isize);
        let mut x_0 = 0 as libc::c_int;
        while x_0 < width {
            let mut n_0: [libc::c_float; 3] = [0.; 3];
            let mut u_0 = x_0 as libc::c_float * invw;
            let mut v_0 = y as libc::c_float * invh;
            crate::src::src::image::heman_image_sample(noisetex.as_mut(), u_0, v_0, n_0.as_mut_ptr());
            let mut z: libc::c_float = 0.;
            crate::src::src::image::heman_image_sample(heightmap.as_mut(), u_0, v_0, core::ptr::addr_of_mut!(z));
            if z as libc::c_double > 0.0f64 {
                let mut influence = z;
                u_0+= influence * n_0[1 as libc::c_int as usize];
                v_0+= influence * n_0[2 as libc::c_int as usize];
                crate::src::src::image::heman_image_sample(heightmap.as_mut(), u_0, v_0, core::ptr::addr_of_mut!(z));
                z+= 6 as libc::c_int as libc::c_float * influence
                        * n_0[0 as libc::c_int as usize];
            }
            let fresh5 = dst_0;
            dst_0= dst_0.offset(1);
            *fresh5= z;
            x_0+= 1;
        }
        y+= 1;
    }
    crate::src::src::image::heman_image_destroy(noisetex);
    crate::src::src::image::heman_image_destroy(heightmap);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_rock_heightmap(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut seed: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut noisetex = heman_internal_generate_rock_noise(width, height, seed);
    let mut heightmap = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    let mut data = (*heightmap).data;
    let mut invh = 1.0f32 / height as libc::c_float;
    let mut invw = 1.0f32 / width as libc::c_float;
    let mut hh = height / 2 as libc::c_int;
    let mut hw = width / 2 as libc::c_int;
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut vv = (y - hh) as libc::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut v = y as libc::c_float * invh;
            let mut u = x as libc::c_float * invw;
            let mut n: libc::c_float = 0.;
            crate::src::src::image::heman_image_sample(noisetex.as_mut(), u, v, core::ptr::addr_of_mut!(n));
            u= (x - hw) as libc::c_float * invw;
            v= vv;
            let mut r = (0.3f64 + n as libc::c_double) as libc::c_float;
            if u * u + v * v > r * r {
                let fresh6 = dst;
                dst= dst.offset(1);
                *fresh6= 0 as libc::c_int as libc::c_float;
            } else {
                let mut z = sqrt((r * r - u * u - v * v) as libc::c_double)
                    as libc::c_float;
                let fresh7 = dst;
                dst= dst.offset(1);
                *fresh7= z;
            }
            x+= 1;
        }
        y+= 1;
    }
    crate::src::src::image::heman_image_destroy(noisetex);
    return heightmap;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_simplex_fbm(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut frequency: libc::c_float,
    mut amplitude: libc::c_float,
    mut octaves: libc::c_int,
    mut lacunarity: libc::c_float,
    mut gain: libc::c_float,
    mut seed: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut ctx = 0 as *mut crate::src::src::noise::osn_context;
    crate::src::src::noise::open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut img = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    let mut data = (*img).data;
    let mut invh = 1.0f32 / height as libc::c_float;
    let mut invw = 1.0f32 / width as libc::c_float;
    let mut ampl = amplitude;
    let mut freq = frequency;
    memset(
        data as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(width as libc::c_ulong)
            .wrapping_mul(height as libc::c_ulong),
    );
    loop {
        let fresh8 = octaves;
        octaves= octaves - 1;
        if !(fresh8 != 0) {
            break;
        }
        let mut y: libc::c_int = 0;
        y= 0 as libc::c_int;
        while y < height {
            let mut v = y as libc::c_float * invh;
            let mut dst = data.offset((y * width) as isize);
            let mut x = 0 as libc::c_int;
            while x < width {
                let mut u = x as libc::c_float * invw;
                let fresh9 = dst;
                dst= dst.offset(1);
                *fresh9= ((*fresh9) as libc::c_double
                    + ampl as libc::c_double
                        * crate::src::src::noise::open_simplex_noise2(
                            ctx,
                            (u * freq) as libc::c_double,
                            (v * freq) as libc::c_double,
                        )) as libc::c_float;
                x+= 1;
            }
            y+= 1;
        }
        ampl*= gain;
        freq*= lacunarity;
    }
    crate::src::src::noise::open_simplex_noise_free(ctx);
    return img;
}
unsafe extern "C" fn sphere(
    mut u: libc::c_float,
    mut v: libc::c_float,
    mut r: libc::c_float,
    mut dst: Option<&mut crate::src::kazmath::aabb3::kmVec3>,
) {
    (*dst.as_deref_mut().unwrap()).x= (r as libc::c_double * sin(v as libc::c_double) * cos(u as libc::c_double))
        as libc::c_float;
    (*dst.as_deref_mut().unwrap()).y= (r as libc::c_double * cos(v as libc::c_double)) as libc::c_float;
    (*dst.as_deref_mut().unwrap()).z= (r as libc::c_double * -sin(v as libc::c_double) * sin(u as libc::c_double))
        as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_planet_heightmap(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut seed: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut ctx = 0 as *mut crate::src::src::noise::osn_context;
    crate::src::src::noise::open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut result = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    let mut scalex = (2.0f32 as libc::c_double * 3.1415926535f64
        / width as libc::c_double) as libc::c_float;
    let mut scaley = (3.1415926535f64 / height as libc::c_double) as libc::c_float;
    let mut invh = 1.0f32 / height as libc::c_float;
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut dst = (*result).data.offset((y * width) as isize);
        let mut p = crate::src::kazmath::aabb3::kmVec3 { x: 0., y: 0., z: 0. };
        let mut v = y as libc::c_float * invh;
        let mut s = 0.95f64 as libc::c_float;
        let mut antarctic_influence = (if (10 as libc::c_int as libc::c_float * (v - s)
            / s) as libc::c_double > -0.5f64
        {
            (10 as libc::c_int as libc::c_float * (v - s) / s) as libc::c_double
        } else {
            -0.5f64
        }) as libc::c_float;
        v= fabs(v as libc::c_double - 0.5f64) as libc::c_float;
        v= (1.5f64 * (0.5f64 - v as libc::c_double)) as libc::c_float;
        let mut equatorial_influence = v * v;
        v= y as libc::c_float * scaley;
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut u = x as libc::c_float * scalex;
            let mut freq = 1 as libc::c_int as libc::c_float;
            let mut amp = 1 as libc::c_int as libc::c_float;
            let mut h = antarctic_influence + equatorial_influence;
            let mut oct = 0 as libc::c_int;
            while oct < 6 as libc::c_int {
                sphere(u, v, freq, Some(&mut p));
                h= (h as libc::c_double
                    + amp as libc::c_double
                        * crate::src::src::noise::open_simplex_noise3(
                            ctx,
                            p.x as libc::c_double,
                            p.y as libc::c_double,
                            p.z as libc::c_double,
                        )) as libc::c_float;
                amp= (amp as libc::c_double * 0.5f64) as libc::c_float;
                freq*= 2 as libc::c_int as libc::c_float;
                oct+= 1;
            }
            let fresh10 = dst;
            dst= dst.offset(1);
            *fresh10= h;
            x+= 1;
        }
        y+= 1;
    }
    crate::src::src::noise::open_simplex_noise_free(ctx);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_draw_seeds(
    mut target: Option<&mut heman_image>,
    mut pts: *mut heman_points,
    mut filterd: libc::c_int,
) {
    let mut radius = (*target.as_deref().unwrap()).width / filterd;
    let mut fwidth = radius * 2 as libc::c_int + 1 as libc::c_int;
    let mut src = (*pts).data;
    let mut w = (*target.as_deref().unwrap()).width;
    let mut h = (*target.as_deref().unwrap()).height;
    let mut i = 0 as libc::c_int;
    while i < (*pts).width {
        let fresh11 = src;
        src= src.offset(1);
        let mut x = (*fresh11);
        let fresh12 = src;
        src= src.offset(1);
        let mut y = (*fresh12);
        let mut strength = crate::src::src::generate::DEFAULT_STRENGTH;
        if (*pts).nbands == 3 as libc::c_int {
            let fresh13 = src;
            src= src.offset(1);
            strength= (*fresh13);
        }
        strength= (crate::src::src::generate::SEALEVEL as libc::c_double + strength as libc::c_double * 0.1f64)
            as libc::c_float;
        let mut ix = (x * w as libc::c_float) as libc::c_int;
        let mut iy = (y * h as libc::c_float) as libc::c_int;
        let mut ii = ix - radius;
        let mut jj = iy - radius;
        let mut kj = 0 as libc::c_int;
        while kj < fwidth {
            let mut ki = 0 as libc::c_int;
            while ki < fwidth {
                let mut i_0 = ii + ki;
                let mut j = jj + kj;
                if !(i_0 < 0 as libc::c_int || i_0 >= w || j < 0 as libc::c_int
                    || j >= h)
                {
                    let mut texel = crate::src::src::image::heman_image_texel(target.as_deref_mut(), i_0, j);
                    let mut d2 = (i_0 - ix) * (i_0 - ix) + (j - iy) * (j - iy);
                    let mut dist = (1 as libc::c_int as libc::c_double
                        - sqrt(d2 as libc::c_double) / radius as libc::c_double)
                        as libc::c_float;
                    *texel= if (*texel) > strength * dist {
                        (*texel)
                    } else {
                        strength * dist
                    };
                }
                ki+= 1;
            }
            kj+= 1;
        }
        i+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_heightmap(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut points: *mut heman_points,
    mut noiseamt: libc::c_float,
    mut seed: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut coastmask = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    crate::src::src::image::heman_image_clear(coastmask.as_mut(), 0 as libc::c_int as libc::c_float);
    heman_internal_draw_seeds(coastmask.as_mut(), points, 1 as libc::c_int);
    let mut data = (*coastmask).data;
    let mut invh = 1.0f32 / height as libc::c_float;
    let mut invw = 1.0f32 / width as libc::c_float;
    let mut hh = height / 2 as libc::c_int;
    let mut hw = width / 2 as libc::c_int;
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut vv = (y - hh) as libc::c_float * invh;
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut n: [libc::c_float; 3] = [0 as libc::c_int as libc::c_float, 0., 0.];
            let mut v = y as libc::c_float * invh;
            let mut u = x as libc::c_float * invw;
            crate::src::src::image::heman_image_sample(noisetex.as_mut(), u, v, n.as_mut_ptr());
            u= (x - hw) as libc::c_float * invw;
            v= vv;
            u+= noiseamt * n[1 as libc::c_int as usize];
            v+= noiseamt * n[2 as libc::c_int as usize];
            let mut m = (*dst);
            m+= noiseamt * n[0 as libc::c_int as usize];
            let fresh14 = dst;
            dst= dst.offset(1);
            *fresh14= (if m < crate::src::src::generate::SEALEVEL { 0 as libc::c_int } else { 1 as libc::c_int })
                as libc::c_float;
            x+= 1;
        }
        y+= 1;
    }
    let mut heightmap = crate::src::src::distance::heman_distance_create_sdf(coastmask);
    crate::src::src::image::heman_image_destroy(coastmask);
    let mut result = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    data= (*result).data;
    y= 0 as libc::c_int;
    while y < height {
        let mut dst_0 = data.offset((y * width) as isize);
        let mut x_0 = 0 as libc::c_int;
        while x_0 < width {
            let mut n_0: [libc::c_float; 3] = [0.; 3];
            let mut u_0 = x_0 as libc::c_float * invw;
            let mut v_0 = y as libc::c_float * invh;
            crate::src::src::image::heman_image_sample(noisetex.as_mut(), u_0, v_0, n_0.as_mut_ptr());
            let mut z: libc::c_float = 0.;
            crate::src::src::image::heman_image_sample(heightmap.as_mut(), u_0, v_0, core::ptr::addr_of_mut!(z));
            if z as libc::c_double > 0.0f64 {
                let mut influence = z;
                u_0+= influence * n_0[1 as libc::c_int as usize];
                v_0+= influence * n_0[2 as libc::c_int as usize];
                crate::src::src::image::heman_image_sample(heightmap.as_mut(), u_0, v_0, core::ptr::addr_of_mut!(z));
                z+= 6 as libc::c_int as libc::c_float * influence
                        * n_0[0 as libc::c_int as usize];
            }
            let fresh15 = dst_0;
            dst_0= dst_0.offset(1);
            *fresh15= z;
            x_0+= 1;
        }
        y+= 1;
    }
    crate::src::src::image::heman_image_destroy(noisetex);
    crate::src::src::image::heman_image_destroy(heightmap);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_1(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut points: *mut heman_points,
    mut colors: *const heman_color,
    mut ocean: heman_color,
    mut seed: libc::c_int,
) -> *mut heman_image {
    let mut contour = crate::src::src::image::heman_image_create(width, height, 3 as libc::c_int);
    crate::src::src::image::heman_image_clear(contour.as_mut(), 0 as libc::c_int as libc::c_float);
    crate::src::src::draw::heman_draw_contour_from_points(
        contour,
        points,
        ocean,
        0.40f64 as libc::c_float,
        0.41f64 as libc::c_float,
        1 as libc::c_int,
    );
    crate::src::src::draw::heman_draw_colored_points(contour.as_mut(), points, colors);
    let mut cf = crate::src::src::distance::heman_distance_create_cpcf(contour);
    let mut warped_cpcf = crate::src::src::ops::heman_ops_warp(cf, seed, 4 as libc::c_int);
    let mut political = crate::src::src::color::heman_color_from_cpcf(warped_cpcf, contour);
    crate::src::src::image::heman_image_destroy(warped_cpcf);
    crate::src::src::image::heman_image_destroy(cf);
    crate::src::src::image::heman_image_destroy(contour);
    return political;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_2(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut ocean: heman_color,
    mut seed: libc::c_int,
    mut political: *mut heman_image,
    mut invert: libc::c_int,
) -> *mut /* owning */ heman_image {
    let mut coastmask = crate::src::src::ops::heman_ops_extract_mask(political, ocean, invert);
    let mut sdf = crate::src::src::distance::heman_distance_create_sdf(coastmask);
    crate::src::src::image::heman_image_destroy(coastmask);
    let mut elevation = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    let mut noisetex = heman_internal_generate_island_noise(width, height, seed);
    let mut data = (*elevation).data;
    let mut invw = (1.0f64 / width as libc::c_double) as libc::c_float;
    let mut invh = (1.0f64 / height as libc::c_double) as libc::c_float;
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut dst = data.offset((y * width) as isize);
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut n: [libc::c_float; 3] = [0.; 3];
            let mut u = x as libc::c_float * invw;
            let mut v = y as libc::c_float * invh;
            crate::src::src::image::heman_image_sample(noisetex.as_mut(), u, v, n.as_mut_ptr());
            let mut z: libc::c_float = 0.;
            crate::src::src::image::heman_image_sample(sdf.as_mut(), u, v, core::ptr::addr_of_mut!(z));
            if z as libc::c_double > 0.0f64 {
                let mut influence = z;
                u+= influence * n[1 as libc::c_int as usize];
                v+= influence * n[2 as libc::c_int as usize];
                crate::src::src::image::heman_image_sample(sdf.as_mut(), u, v, core::ptr::addr_of_mut!(z));
                z+= 6 as libc::c_int as libc::c_float * influence
                        * n[0 as libc::c_int as usize];
            }
            let fresh16 = dst;
            dst= dst.offset(1);
            *fresh16= z;
            x+= 1;
        }
        y+= 1;
    }
    crate::src::src::image::heman_image_destroy(noisetex);
    crate::src::src::image::heman_image_destroy(sdf);
    return elevation;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political_3(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut colors: *const heman_color,
    mut ncolors: libc::c_int,
    mut ocean: heman_color,
    mut seed: libc::c_int,
    mut political: *mut heman_image,
) -> *mut /* owning */ heman_image {
    let mut elevations = malloc(
        (::std::mem::size_of::<*mut heman_image>() as libc::c_ulong)
            .wrapping_mul(ncolors as libc::c_ulong),
    ) as *mut *mut heman_image;
    let mut cindex = 0 as libc::c_int;
    while cindex < ncolors {
        *elevations.offset(cindex as isize) = heman_generate_archipelago_political_2(
            width,
            height,
            *colors.offset(cindex as isize),
            seed,
            political,
            1 as libc::c_int,
        );
        cindex+= 1;
    }
    let mut elevation = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    crate::src::src::image::heman_image_clear(elevation.as_mut(), 0 as libc::c_int as libc::c_float);
    let mut cindex_0 = 0 as libc::c_int;
    while cindex_0 < ncolors {
        let mut y: libc::c_int = 0;
        y= 0 as libc::c_int;
        while y < height {
            let mut dst = (*elevation).data.offset((y * width) as isize);
            let mut src = ((**elevations.offset(cindex_0 as isize)).data)
                .offset((y * width) as isize);
            let mut x = 0 as libc::c_int;
            while x < width {
                *dst= if (*src) > (*dst) { (*src) } else { (*dst) };
                x+= 1;
                dst= dst.offset(1);
                src= src.offset(1);
            }
            y+= 1;
        }
        crate::src::src::image::heman_image_destroy(*elevations.offset(cindex_0 as isize));
        cindex_0+= 1;
    }
    free(elevations as *mut libc::c_void);
    let mut ocean_elevation = heman_generate_archipelago_political_2(
        width,
        height,
        ocean,
        seed,
        political,
        0 as libc::c_int,
    );
    let mut y_0: libc::c_int = 0;
    y_0= 0 as libc::c_int;
    while y_0 < height {
        let mut dst_0 = (*elevation).data.offset((y_0 * width) as isize);
        let mut src_0 = (*ocean_elevation).data.offset((y_0 * width) as isize);
        let mut x_0 = 0 as libc::c_int;
        while x_0 < width {
            if (*src_0) < 0 as libc::c_int as libc::c_float {
                *dst_0= (*src_0);
            }
            x_0+= 1;
            dst_0= dst_0.offset(1);
            src_0= src_0.offset(1);
        }
        y_0+= 1;
    }
    crate::src::src::image::heman_image_destroy(ocean_elevation);
    return elevation;
}
#[no_mangle]
pub unsafe extern "C" fn heman_generate_archipelago_political(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut points: *mut heman_points,
    mut colors: *const heman_color,
    mut ocean: heman_color,
    mut seed: libc::c_int,
    mut elevation: Option<&mut *mut /* owning */ heman_image>,
    mut political: Option<&mut *mut heman_image>,
    mut elevation_mode: libc::c_int,
) {
    *political.as_deref_mut().unwrap()= heman_generate_archipelago_political_1(
        width,
        height,
        points,
        colors,
        ocean,
        seed,
    );
    if elevation_mode == 0 as libc::c_int {
        *elevation.as_deref_mut().unwrap()= heman_generate_archipelago_political_2(
            width,
            height,
            ocean,
            seed,
            (*political.as_deref().unwrap()),
            0 as libc::c_int,
        );
    } else {
        let mut ncolors = (*points).width;
        *elevation.as_deref_mut().unwrap()= heman_generate_archipelago_political_3(
            width,
            height,
            colors,
            ncolors,
            ocean,
            seed,
            (*political.as_deref().unwrap()),
        );
    };
}
