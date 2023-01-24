use ::libc;
extern "C" {
    
    
    
    
    
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor25 { dummy: () }
pub type heman_image = crate::src::src::color::heman_image_s;
pub type heman_points = crate::src::src::color::heman_image_s;
pub type heman_color = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn heman_draw_points(
    mut target: Option<&mut heman_image>,
    mut pts: *mut heman_points,
    mut val: libc::c_float,
) {
    let mut src = (*pts).data;
    let mut k = 0 as libc::c_int;
    while k < (*pts).width {
        let mut x = *src.offset(0 as libc::c_int as isize);
        let mut y = *src.offset(1 as libc::c_int as isize);
        src= src.offset((*pts).nbands as isize);
        let mut i = (x * (*target.as_deref().unwrap()).width as libc::c_float) as libc::c_int;
        let mut j = (y * (*target.as_deref().unwrap()).height as libc::c_float) as libc::c_int;
        if !(i < 0 as libc::c_int || i >= (*target.as_deref().unwrap()).width || j < 0 as libc::c_int
            || j >= (*target.as_deref().unwrap()).height)
        {
            let mut texel = crate::src::src::image::heman_image_texel(target.as_deref_mut(), i, j);
            let mut c = 0 as libc::c_int;
            while c < (*target.as_deref().unwrap()).nbands {
                let fresh0 = texel;
                texel= texel.offset(1);
                *fresh0= val;
                c+= 1;
            }
        }
        k+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_colored_points(
    mut target: Option<&mut heman_image>,
    mut pts: *mut heman_points,
    mut colors: *const heman_color,
) {
    if (*target.as_deref().unwrap()).nbands == 3 as libc::c_int || (*target.as_deref().unwrap()).nbands == 4 as libc::c_int
    {} else {
        __assert_fail(
            b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                as *const libc::c_char,
            b"../src/draw.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            b"void heman_draw_colored_points(heman_image *, heman_points *, const heman_color *)\0" as *const u8 as *const i8,
        );
    }
    let mut src = (*pts).data;
    let mut inv = 1.0f32 / 255.0f32;
    let mut k = 0 as libc::c_int;
    while k < (*pts).width {
        let mut x = *src.offset(0 as libc::c_int as isize);
        let mut y = *src.offset(1 as libc::c_int as isize);
        src= src.offset((*pts).nbands as isize);
        let mut i = (x * (*target.as_deref().unwrap()).width as libc::c_float) as libc::c_int;
        let mut j = (y * (*target.as_deref().unwrap()).height as libc::c_float) as libc::c_int;
        if !(i < 0 as libc::c_int || i >= (*target.as_deref().unwrap()).width || j < 0 as libc::c_int
            || j >= (*target.as_deref().unwrap()).height)
        {
            let mut texel = crate::src::src::image::heman_image_texel(target.as_deref_mut(), i, j);
            let mut rgb = *colors.offset(k as isize);
            let fresh1 = texel;
            texel= texel.offset(1);
            *fresh1= (rgb >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_float * inv;
            let fresh2 = texel;
            texel= texel.offset(1);
            *fresh2= (rgb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_float * inv;
            let fresh3 = texel;
            texel= texel.offset(1);
            *fresh3= (rgb & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
            if (*target.as_deref().unwrap()).nbands == 4 as libc::c_int {
                *texel= (rgb >> 24 as libc::c_int) as libc::c_float * inv;
            }
        }
        k+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_colored_circles(
    mut target: Option<&mut heman_image>,
    mut pts: *mut heman_points,
    mut radius: libc::c_int,
    mut colors: *const heman_color,
) {
    let mut fwidth = radius * 2 as libc::c_int + 1 as libc::c_int;
    let mut radius2 = radius * radius;
    let mut src = (*pts).data;
    let mut inv = 1.0f32 / 255.0f32;
    let mut w = (*target.as_deref().unwrap()).width;
    let mut h = (*target.as_deref().unwrap()).height;
    let mut k = 0 as libc::c_int;
    while k < (*pts).width {
        let mut x = *src.offset(0 as libc::c_int as isize);
        let mut y = *src.offset(1 as libc::c_int as isize);
        src= src.offset((*pts).nbands as isize);
        let mut ii = (x * w as libc::c_float - radius as libc::c_float) as libc::c_int;
        let mut jj = (y * h as libc::c_float - radius as libc::c_float) as libc::c_int;
        let mut kj = 0 as libc::c_int;
        while kj < fwidth {
            let mut ki = 0 as libc::c_int;
            while ki < fwidth {
                let mut i = ii + ki;
                let mut j = jj + kj;
                let mut r2 = ((i as libc::c_float - x * w as libc::c_float)
                    * (i as libc::c_float - x * w as libc::c_float)
                    + (j as libc::c_float - y * h as libc::c_float)
                        * (j as libc::c_float - y * h as libc::c_float)) as libc::c_int;
                if !(r2 > radius2) {
                    let mut texel = crate::src::src::image::heman_image_texel(target.as_deref_mut(), i, j);
                    let mut rgb = *colors.offset(k as isize);
                    let fresh4 = texel;
                    texel= texel.offset(1);
                    *fresh4= (rgb >> 16 as libc::c_int) as libc::c_float * inv;
                    let fresh5 = texel;
                    texel= texel.offset(1);
                    *fresh5= (rgb >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
                    *texel= (rgb & 0xff as libc::c_int as libc::c_uint) as libc::c_float
                        * inv;
                }
                ki+= 1;
            }
            kj+= 1;
        }
        k+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_splats(
    mut target: Option<&mut heman_image>,
    mut pts: *mut heman_points,
    mut radius: libc::c_int,
    mut blend_mode: libc::c_int,
) {
    let mut fwidth = radius * 2 as libc::c_int + 1 as libc::c_int;
    let mut gaussian_splat = malloc(
        ((fwidth * fwidth) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    crate::src::src::gaussian::generate_gaussian_splat(gaussian_splat, fwidth);
    let mut src = (*pts).data;
    let mut w = (*target.as_deref().unwrap()).width;
    let mut h = (*target.as_deref().unwrap()).height;
    let mut i = 0 as libc::c_int;
    while i < (*pts).width {
        let fresh6 = src;
        src= src.offset(1);
        let mut x = (*fresh6);
        let fresh7 = src;
        src= src.offset(1);
        let mut y = (*fresh7);
        let mut ii = (x * w as libc::c_float - radius as libc::c_float) as libc::c_int;
        let mut jj = (y * h as libc::c_float - radius as libc::c_float) as libc::c_int;
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
                    let mut c = 0 as libc::c_int;
                    while c < (*target.as_deref().unwrap()).nbands {
                        let fresh8 = texel;
                        texel= texel.offset(1);
                        *fresh8+= *gaussian_splat.offset((kj * fwidth + ki) as isize);
                        c+= 1;
                    }
                }
                ki+= 1;
            }
            kj+= 1;
        }
        i+= 1;
    }
    free(gaussian_splat as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_draw_contour_from_points(
    mut target: *mut heman_image,
    mut coords: *mut heman_points,
    mut rgb: heman_color,
    mut mind: libc::c_float,
    mut maxd: libc::c_float,
    mut filterd: libc::c_int,
) {
    if (*target).nbands == 3 as libc::c_int || (*target).nbands == 4 as libc::c_int
    {} else {
        __assert_fail(
            b"target->nbands == 3 || target->nbands == 4\0" as *const u8
                as *const libc::c_char,
            b"../src/draw.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            b"void heman_draw_contour_from_points(heman_image *, heman_points *, heman_color, float, float, int)\0" as *const u8 as *const i8,
        );
    }
    let mut width = (*target).width;
    let mut height = (*target).height;
    let mut seed = crate::src::src::image::heman_image_create(width, height, 1 as libc::c_int);
    crate::src::src::image::heman_image_clear(seed.as_mut(), 0 as libc::c_int as libc::c_float);
    crate::src::src::generate::heman_internal_draw_seeds(seed.as_mut(), coords, filterd);
    let mut inv = 1.0f32 / 255.0f32;
    let mut r = (rgb >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_float * inv;
    let mut g = (rgb >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_float * inv;
    let mut b = (rgb & 0xff as libc::c_int as libc::c_uint) as libc::c_float * inv;
    let mut a = 1 as libc::c_int as libc::c_float;
    if (*target).nbands == 4 as libc::c_int {
        a= (rgb >> 24 as libc::c_int) as libc::c_float * inv;
    }
    let mut y: libc::c_int = 0;
    y= 0 as libc::c_int;
    while y < height {
        let mut dst = (*target).data.offset((y * width * (*target).nbands) as isize);
        let mut x = 0 as libc::c_int;
        while x < width {
            let mut dist = *crate::src::src::image::heman_image_texel(seed.as_mut(), x, y);
            if dist > mind && dist < maxd {
                *dst.offset(0 as libc::c_int as isize) = r;
                *dst.offset(1 as libc::c_int as isize) = g;
                *dst.offset(2 as libc::c_int as isize) = b;
                if (*target).nbands == 4 as libc::c_int {
                    *dst.offset(3 as libc::c_int as isize) = a;
                }
            }
            dst= dst.offset((*target).nbands as isize);
            x+= 1;
        }
        y+= 1;
    }
    crate::src::src::points::heman_points_destroy(seed);
}
