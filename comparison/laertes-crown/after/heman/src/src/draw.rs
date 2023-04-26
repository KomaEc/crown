
extern "C" {
    
    
    
    
    
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn free(_: * mut core::ffi::c_void);
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    
}
pub use crate::src::src::gaussian::generate_gaussian_splat;
pub use crate::src::src::generate::heman_internal_draw_seeds;
pub use crate::src::src::image::heman_image_clear;
pub use crate::src::src::image::heman_image_create;
pub use crate::src::src::image::heman_image_texel;
pub use crate::src::src::points::heman_points_destroy;
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
pub type heman_points = crate::src::src::color::heman_image_s;
pub type heman_color = std::os::raw::c_uint;
#[no_mangle]
pub unsafe extern "C" fn heman_draw_points<'a1>(
    mut target: * mut crate::src::src::color::heman_image_s,
    mut pts: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut val: std::os::raw::c_float,
) {
    let mut src = (*(borrow_mut(&mut pts)).unwrap()).data;
    let mut k = 0 as std::os::raw::c_int;
    while k < (*(borrow(& pts)).unwrap()).width {
        let mut x = *src.offset(0 as std::os::raw::c_int as isize);
        let mut y = *src.offset(1 as std::os::raw::c_int as isize);
        src = src.offset((*(borrow(& pts)).unwrap()).nbands as isize);
        let mut i = (x * (*target).width as std::os::raw::c_float) as std::os::raw::c_int;
        let mut j = (y * (*target).height as std::os::raw::c_float) as std::os::raw::c_int;
        if !(i < 0 as std::os::raw::c_int || i >= (*target).width || j < 0 as std::os::raw::c_int
            || j >= (*target).height)
        {
            let mut texel = heman_image_texel(target, i, j);
            let mut c = 0 as std::os::raw::c_int;
            while c < (*target).nbands {
                let mut fresh0 = texel;
                texel = texel.offset(1);
                *fresh0 = val;
                c += 1;
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_colored_points(
    mut target: * mut crate::src::src::color::heman_image_s,
    mut pts: * mut crate::src::src::color::heman_image_s,
    mut colors: * const std::os::raw::c_uint,
) {
    if (*target).nbands == 3 as std::os::raw::c_int || (*target).nbands == 4 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                as *const std::os::raw::c_char,
            b"../src/draw.c\0" as *const u8 as *const std::os::raw::c_char,
            27 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 83], &'_ [i8; 83]>(
                b"void heman_draw_colored_points(heman_image *, heman_points *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut src = (*pts).data;
    let mut inv = 1.0f32 / 255.0f32;
    let mut k = 0 as std::os::raw::c_int;
    while k < (*pts).width {
        let mut x = *src.offset(0 as std::os::raw::c_int as isize);
        let mut y = *src.offset(1 as std::os::raw::c_int as isize);
        src = src.offset((*pts).nbands as isize);
        let mut i = (x * (*target).width as std::os::raw::c_float) as std::os::raw::c_int;
        let mut j = (y * (*target).height as std::os::raw::c_float) as std::os::raw::c_int;
        if !(i < 0 as std::os::raw::c_int || i >= (*target).width || j < 0 as std::os::raw::c_int
            || j >= (*target).height)
        {
            let mut texel = heman_image_texel(target, i, j);
            let mut rgb = *colors.offset(k as isize);
            let mut fresh1 = texel;
            texel = texel.offset(1);
            *fresh1 = (rgb >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_float * inv;
            let mut fresh2 = texel;
            texel = texel.offset(1);
            *fresh2 = (rgb >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                as std::os::raw::c_float * inv;
            let mut fresh3 = texel;
            texel = texel.offset(1);
            *fresh3 = (rgb & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float * inv;
            if (*target).nbands == 4 as std::os::raw::c_int {
                *texel = (rgb >> 24 as std::os::raw::c_int) as std::os::raw::c_float * inv;
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_colored_circles<'a1>(
    mut target: * mut crate::src::src::color::heman_image_s,
    mut pts: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut radius: std::os::raw::c_int,
    mut colors: * const std::os::raw::c_uint,
) {
    let mut fwidth = radius * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int;
    let mut radius2 = radius * radius;
    let mut src = (*(borrow_mut(&mut pts)).unwrap()).data;
    let mut inv = 1.0f32 / 255.0f32;
    let mut w = (*target).width;
    let mut h = (*target).height;
    let mut k = 0 as std::os::raw::c_int;
    while k < (*(borrow(& pts)).unwrap()).width {
        let mut x = *src.offset(0 as std::os::raw::c_int as isize);
        let mut y = *src.offset(1 as std::os::raw::c_int as isize);
        src = src.offset((*(borrow(& pts)).unwrap()).nbands as isize);
        let mut ii = (x * w as std::os::raw::c_float - radius as std::os::raw::c_float) as std::os::raw::c_int;
        let mut jj = (y * h as std::os::raw::c_float - radius as std::os::raw::c_float) as std::os::raw::c_int;
        let mut kj = 0 as std::os::raw::c_int;
        while kj < fwidth {
            let mut ki = 0 as std::os::raw::c_int;
            while ki < fwidth {
                let mut i = ii + ki;
                let mut j = jj + kj;
                let mut r2 = ((i as std::os::raw::c_float - x * w as std::os::raw::c_float)
                    * (i as std::os::raw::c_float - x * w as std::os::raw::c_float)
                    + (j as std::os::raw::c_float - y * h as std::os::raw::c_float)
                        * (j as std::os::raw::c_float - y * h as std::os::raw::c_float)) as std::os::raw::c_int;
                if !(r2 > radius2) {
                    let mut texel = heman_image_texel(target, i, j);
                    let mut rgb = *colors.offset(k as isize);
                    let mut fresh4 = texel;
                    texel = texel.offset(1);
                    *fresh4 = (rgb >> 16 as std::os::raw::c_int) as std::os::raw::c_float * inv;
                    let mut fresh5 = texel;
                    texel = texel.offset(1);
                    *fresh5 = (rgb >> 8 as std::os::raw::c_int
                        & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float * inv;
                    *texel = (rgb & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float
                        * inv;
                }
                ki += 1;
            }
            kj += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_splats<'a1>(
    mut target: * mut crate::src::src::color::heman_image_s,
    mut pts: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut radius: std::os::raw::c_int,
    mut blend_mode: std::os::raw::c_int,
) {
    let mut fwidth = radius * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int;
    let mut gaussian_splat = malloc(
        ((fwidth * fwidth) as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_float;
    generate_gaussian_splat(gaussian_splat, fwidth);
    let mut src = (*(borrow_mut(&mut pts)).unwrap()).data;
    let mut w = (*target).width;
    let mut h = (*target).height;
    let mut i = 0 as std::os::raw::c_int;
    while i < (*(borrow(& pts)).unwrap()).width {
        let mut fresh6 = src;
        src = src.offset(1);
        let mut x = *fresh6;
        let mut fresh7 = src;
        src = src.offset(1);
        let mut y = *fresh7;
        let mut ii = (x * w as std::os::raw::c_float - radius as std::os::raw::c_float) as std::os::raw::c_int;
        let mut jj = (y * h as std::os::raw::c_float - radius as std::os::raw::c_float) as std::os::raw::c_int;
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
                    let mut c = 0 as std::os::raw::c_int;
                    while c < (*target).nbands {
                        let mut fresh8 = texel;
                        texel = texel.offset(1);
                        *fresh8 += *gaussian_splat.offset((kj * fwidth + ki) as isize);
                        c += 1;
                    }
                }
                ki += 1;
            }
            kj += 1;
        }
        i += 1;
    }
    free(gaussian_splat as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_contour_from_points(
    mut target: * mut crate::src::src::color::heman_image_s,
    mut coords: * mut crate::src::src::color::heman_image_s,
    mut rgb: std::os::raw::c_uint,
    mut mind: std::os::raw::c_float,
    mut maxd: std::os::raw::c_float,
    mut filterd: std::os::raw::c_int,
) {
    if (*target).nbands == 3 as std::os::raw::c_int || (*target).nbands == 4 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                as *const std::os::raw::c_char,
            b"../src/draw.c\0" as *const u8 as *const std::os::raw::c_char,
            119 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 99], &'_ [i8; 99]>(
                b"void heman_draw_contour_from_points(heman_image *, heman_points *, heman_color, float, float, int)\0",
            ))
                .as_ptr(),
        );
    }
    let mut width = (*target).width;
    let mut height = (*target).height;
    let mut seed = heman_image_create(width, height, 1 as std::os::raw::c_int);
    heman_image_clear(seed, 0 as std::os::raw::c_int as std::os::raw::c_float);
    heman_internal_draw_seeds(seed, coords, filterd);
    let mut inv = 1.0f32 / 255.0f32;
    let mut r = (rgb >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_float * inv;
    let mut g = (rgb >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_float * inv;
    let mut b = (rgb & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float * inv;
    let mut a = 1 as std::os::raw::c_int as std::os::raw::c_float;
    if (*target).nbands == 4 as std::os::raw::c_int {
        a = (rgb >> 24 as std::os::raw::c_int) as std::os::raw::c_float * inv;
    }
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = ((*target).data).offset((y * width * (*target).nbands) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut dist = *heman_image_texel(seed, x, y);
            if dist > mind && dist < maxd {
                *dst.offset(0 as std::os::raw::c_int as isize) = r;
                *dst.offset(1 as std::os::raw::c_int as isize) = g;
                *dst.offset(2 as std::os::raw::c_int as isize) = b;
                if (*target).nbands == 4 as std::os::raw::c_int {
                    *dst.offset(3 as std::os::raw::c_int as isize) = a;
                }
            }
            dst = dst.offset((*target).nbands as isize);
            x += 1;
        }
        y += 1;
    }
    heman_points_destroy(seed);
}
use crate::laertes_rt::*;
