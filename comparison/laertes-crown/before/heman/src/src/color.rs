
extern "C" {
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
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
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
pub type heman_color = std::os::raw::c_uint;
#[no_mangle]
pub static mut _gamma: std::os::raw::c_float = 2.2f32;
#[no_mangle]
pub unsafe extern "C" fn heman_color_set_gamma(mut g: std::os::raw::c_float) {
    _gamma = g;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_create_gradient(
    mut width: std::os::raw::c_int,
    mut num_colors: std::os::raw::c_int,
    mut cp_locations: *const std::os::raw::c_int,
    mut cp_values: *const heman_color,
) -> *mut heman_image {
    if width > 0 as std::os::raw::c_int && num_colors >= 2 as std::os::raw::c_int {} else {
        __assert_fail(
            b"width > 0 && num_colors >= 2\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            13 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[std::os::raw::c_char; 85],
            >(
                b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    if *cp_locations.offset(0 as std::os::raw::c_int as isize) == 0 as std::os::raw::c_int {} else {
        __assert_fail(
            b"cp_locations[0] == 0\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            14 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[std::os::raw::c_char; 85],
            >(
                b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    if *cp_locations.offset((num_colors - 1 as std::os::raw::c_int) as isize)
        == width - 1 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"cp_locations[num_colors - 1] == width - 1\0" as *const u8
                as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            15 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 85],
                &[std::os::raw::c_char; 85],
            >(
                b"heman_image *heman_color_create_gradient(int, int, const int *, const heman_color *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut f32colors = malloc(
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)
            .wrapping_mul(num_colors as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_float;
    let mut inv = 1.0f32 / 255.0f32;
    let mut f32color = f32colors;
    let mut u32color = cp_values;
    let mut index = 0 as std::os::raw::c_int;
    while index < num_colors {
        let fresh0 = u32color;
        u32color = u32color.offset(1);
        let mut rgb = *fresh0;
        let mut r = (rgb >> 16 as std::os::raw::c_int) as std::os::raw::c_float * inv;
        let mut g = (rgb >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_float * inv;
        let mut b = (rgb & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float * inv;
        let fresh1 = f32color;
        f32color = f32color.offset(1);
        *fresh1 = pow(r as std::os::raw::c_double, _gamma as std::os::raw::c_double) as std::os::raw::c_float;
        let fresh2 = f32color;
        f32color = f32color.offset(1);
        *fresh2 = pow(g as std::os::raw::c_double, _gamma as std::os::raw::c_double) as std::os::raw::c_float;
        let fresh3 = f32color;
        f32color = f32color.offset(1);
        *fresh3 = pow(b as std::os::raw::c_double, _gamma as std::os::raw::c_double) as std::os::raw::c_float;
        index += 1;
    }
    let mut result = heman_image_create(width, 1 as std::os::raw::c_int, 3 as std::os::raw::c_int);
    let mut index0 = 0 as std::os::raw::c_int;
    let mut index1 = 1 as std::os::raw::c_int;
    let mut dst = (*result).data;
    let mut t: std::os::raw::c_float = 0.;
    let mut invgamma = 1.0f32 / _gamma;
    let mut current_block_16: u64;
    let mut x = 0 as std::os::raw::c_int;
    while x < width {
        let mut loc0 = *cp_locations.offset(index0 as isize);
        let mut loc1 = *cp_locations.offset(index1 as isize);
        if loc0 == loc1 {
            t = 0 as std::os::raw::c_int as std::os::raw::c_float;
            current_block_16 = 11057878835866523405;
        } else {
            t = (x - loc0) as std::os::raw::c_float / (loc1 - loc0) as std::os::raw::c_float;
            if t >= 1 as std::os::raw::c_int as std::os::raw::c_float {
                x -= 1;
                index0 += 1;
                index1 = if index1 + 1 as std::os::raw::c_int > num_colors - 1 as std::os::raw::c_int {
                    num_colors - 1 as std::os::raw::c_int
                } else {
                    index1 + 1 as std::os::raw::c_int
                };
                current_block_16 = 12039483399334584727;
            } else {
                current_block_16 = 11057878835866523405;
            }
        }
        match current_block_16 {
            11057878835866523405 => {
                let mut r0 = *f32colors.offset((index0 * 3 as std::os::raw::c_int) as isize);
                let mut g0 = *f32colors
                    .offset((index0 * 3 as std::os::raw::c_int + 1 as std::os::raw::c_int) as isize);
                let mut b0 = *f32colors
                    .offset((index0 * 3 as std::os::raw::c_int + 2 as std::os::raw::c_int) as isize);
                let mut r1 = *f32colors.offset((index1 * 3 as std::os::raw::c_int) as isize);
                let mut g1 = *f32colors
                    .offset((index1 * 3 as std::os::raw::c_int + 1 as std::os::raw::c_int) as isize);
                let mut b1 = *f32colors
                    .offset((index1 * 3 as std::os::raw::c_int + 2 as std::os::raw::c_int) as isize);
                let mut invt = 1.0f32 - t;
                let mut r_0 = r0 * invt + r1 * t;
                let mut g_0 = g0 * invt + g1 * t;
                let mut b_0 = b0 * invt + b1 * t;
                let fresh4 = dst;
                dst = dst.offset(1);
                *fresh4 = pow(r_0 as std::os::raw::c_double, invgamma as std::os::raw::c_double)
                    as std::os::raw::c_float;
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = pow(g_0 as std::os::raw::c_double, invgamma as std::os::raw::c_double)
                    as std::os::raw::c_float;
                let fresh6 = dst;
                dst = dst.offset(1);
                *fresh6 = pow(b_0 as std::os::raw::c_double, invgamma as std::os::raw::c_double)
                    as std::os::raw::c_float;
            }
            _ => {}
        }
        x += 1;
    }
    free(f32colors as *mut std::os::raw::c_void);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_apply_gradient(
    mut heightmap: *mut heman_image,
    mut minheight: std::os::raw::c_float,
    mut maxheight: std::os::raw::c_float,
    mut gradient: *mut heman_image,
) -> *mut heman_image {
    if (*heightmap).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"heightmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            74 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[std::os::raw::c_char; 84],
            >(
                b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).height == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"gradient->height == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            75 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[std::os::raw::c_char; 84],
            >(
                b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*gradient).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"gradient->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            76 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[std::os::raw::c_char; 84],
            >(
                b"heman_image *heman_color_apply_gradient(heman_image *, float, float, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut w = (*heightmap).width;
    let mut h = (*heightmap).height;
    let mut result = heman_image_create(w, h, 3 as std::os::raw::c_int);
    let mut size = (*result).height * (*result).width;
    let mut dst = (*result).data;
    let mut src: *const std::os::raw::c_float = (*heightmap).data;
    let mut scale = 1.0f32 / (maxheight - minheight);
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut u = if 0.0f32
            > (if 1.0f32 > (*src - minheight) * scale {
                (*src - minheight) * scale
            } else {
                1.0f32
            })
        {
            0.0f32
        } else if 1.0f32 > (*src - minheight) * scale {
            (*src - minheight) * scale
        } else {
            1.0f32
        };
        heman_image_sample(gradient, u, 0.5f32, dst);
        i += 1;
        dst = dst.offset(3 as std::os::raw::c_int as isize);
        src = src.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_from_grayscale(
    mut grayscale: *mut heman_image,
) -> *mut heman_image {
    if (*grayscale).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"grayscale->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            93 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[std::os::raw::c_char; 55],
            >(b"heman_image *heman_color_from_grayscale(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut w = (*grayscale).width;
    let mut h = (*grayscale).height;
    let mut result = heman_image_create(w, h, 3 as std::os::raw::c_int);
    let mut size = w * h;
    let mut dst = (*result).data;
    let mut src: *const std::os::raw::c_float = (*grayscale).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let fresh7 = src;
        src = src.offset(1);
        let mut v = *fresh7;
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = v;
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = v;
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = v;
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_to_grayscale(
    mut colorimg: *mut heman_image,
) -> *mut heman_image {
    if (*colorimg).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"colorimg->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            111 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 53],
                &[std::os::raw::c_char; 53],
            >(b"heman_image *heman_color_to_grayscale(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut w = (*colorimg).width;
    let mut h = (*colorimg).height;
    let mut result = heman_image_create(w, h, 1 as std::os::raw::c_int);
    let mut size = w * h;
    let mut dst = (*result).data;
    let mut src: *const std::os::raw::c_float = (*colorimg).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let fresh11 = src;
        src = src.offset(1);
        let mut r = *fresh11;
        let fresh12 = src;
        src = src.offset(1);
        let mut g = *fresh12;
        let fresh13 = src;
        src = src.offset(1);
        let mut b = *fresh13;
        let fresh14 = dst;
        dst = dst.offset(1);
        *fresh14 = (0.299f64 * r as std::os::raw::c_double + 0.587f64 * g as std::os::raw::c_double
            + 0.114f64 * b as std::os::raw::c_double) as std::os::raw::c_float;
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_internal_rg(
    mut cfield: *mut heman_image,
) -> *mut heman_image {
    if (*cfield).nbands == 2 as std::os::raw::c_int {} else {
        __assert_fail(
            b"cfield->nbands == 2\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            129 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 46],
                &[std::os::raw::c_char; 46],
            >(b"heman_image *heman_internal_rg(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut w = (*cfield).width;
    let mut h = (*cfield).height;
    let mut target = heman_image_create(w, h, 3 as std::os::raw::c_int);
    let mut dst = (*target).data;
    let mut src = (*cfield).data;
    let mut size = w * h;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let fresh15 = src;
        src = src.offset(1);
        let mut u = *fresh15 / w as std::os::raw::c_float;
        let fresh16 = src;
        src = src.offset(1);
        let mut v = *fresh16 / h as std::os::raw::c_float;
        let fresh17 = dst;
        dst = dst.offset(1);
        *fresh17 = u;
        let fresh18 = dst;
        dst = dst.offset(1);
        *fresh18 = v;
        let fresh19 = dst;
        dst = dst.offset(1);
        *fresh19 = 0 as std::os::raw::c_int as std::os::raw::c_float;
        i += 1;
    }
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn heman_color_from_cpcf(
    mut cfield: *mut heman_image,
    mut texture: *mut heman_image,
) -> *mut heman_image {
    if texture.is_null() {
        return heman_internal_rg(cfield);
    }
    if (*cfield).nbands == 2 as std::os::raw::c_int {} else {
        __assert_fail(
            b"cfield->nbands == 2\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            151 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[std::os::raw::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    if (*texture).nbands == 3 as std::os::raw::c_int || (*texture).nbands == 4 as std::os::raw::c_int
    {} else {
        __assert_fail(
            b"texture->nbands == 3 || texture->nbands == 4\0" as *const u8
                as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            152 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[std::os::raw::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    if (*cfield).width == (*texture).width {} else {
        __assert_fail(
            b"cfield->width == texture->width\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            153 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[std::os::raw::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    if (*cfield).height == (*texture).height {} else {
        __assert_fail(
            b"cfield->height == texture->height\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/color.c\0" as *const u8 as *const std::os::raw::c_char,
            154 as std::os::raw::c_int as std::os::raw::c_uint,
            (*::std::mem::transmute::<
                &[u8; 65],
                &[std::os::raw::c_char; 65],
            >(b"heman_image *heman_color_from_cpcf(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut w = (*cfield).width;
    let mut h = (*cfield).height;
    let mut target = heman_image_create(w, h, (*texture).nbands);
    let mut dst = (*target).data;
    let mut src = (*cfield).data;
    let mut size = w * h;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let fresh20 = src;
        src = src.offset(1);
        let mut u = *fresh20;
        let fresh21 = src;
        src = src.offset(1);
        let mut v = *fresh21;
        let mut texel = heman_image_texel(texture, u as std::os::raw::c_int, v as std::os::raw::c_int);
        let mut c = 0 as std::os::raw::c_int;
        while c < (*texture).nbands {
            let fresh22 = texel;
            texel = texel.offset(1);
            let fresh23 = dst;
            dst = dst.offset(1);
            *fresh23 = *fresh22;
            c += 1;
        }
        i += 1;
    }
    return target;
}
