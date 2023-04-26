
extern "C" {
    
    
    
    
    
    
    
    
    
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    fn fabsf(_: std::os::raw::c_float) -> std::os::raw::c_float;
    fn floor(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
    fn omp_get_max_threads() -> std::os::raw::c_int;
}
pub use crate::src::kazmath::vec3::kmVec3Lerp;
pub use crate::src::src::color::heman_color_to_grayscale;
pub use crate::src::src::distance::heman_distance_identity_cpcf;
pub use crate::src::src::image::heman_image_create;
pub use crate::src::src::image::heman_image_destroy;
pub use crate::src::src::image::heman_image_texel;
pub use crate::src::src::noise::open_simplex_noise;
pub use crate::src::src::noise::open_simplex_noise2;
pub use crate::src::src::noise::open_simplex_noise_free;
pub use crate::src::src::noise::osn_context;
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
pub type heman_points = crate::src::src::color::heman_image_s;
pub type heman_color = std::os::raw::c_uint;
// #[derive(Copy, Clone)]

pub type kmVec3 = crate::src::kazmath::aabb3::kmVec3;
pub type int64_t = std::os::raw::c_long;
pub type __int64_t = std::os::raw::c_long;
#[no_mangle]
pub unsafe extern "C" fn heman_get_num_threads() -> std::os::raw::c_int {
    return omp_get_max_threads();
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_step<'a1>(
    mut hmap: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut threshold: std::os::raw::c_float,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& hmap)).unwrap()).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            17 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 50], &'_ [i8; 50]>(b"heman_image *heman_ops_step(heman_image *, float)\0"))
                .as_ptr(),
        );
    }
    let mut result = heman_image_create((*(borrow_mut(&mut hmap)).unwrap()).width, (*(borrow_mut(&mut hmap)).unwrap()).height, 1 as std::os::raw::c_int);
    let mut size = (*(borrow(& hmap)).unwrap()).height * (*(borrow(& hmap)).unwrap()).width;
    let mut src = (*(borrow_mut(&mut hmap)).unwrap()).data;
    let mut dst = (*result).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut fresh0 = src;
        src = src.offset(1);
        let mut fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = (if *fresh0 >= threshold {
            1 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        }) as std::os::raw::c_float;
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_max<'a1, 'a2>(
    mut imga: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut imgb: Option<&'a2 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& imga)).unwrap()).width == (*(borrow(& imgb)).unwrap()).width {} else {
        __assert_fail(
            b"imga->width == imgb->width\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            30 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 57], &'_ [i8; 57]>(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    if (*(borrow(& imga)).unwrap()).height == (*(borrow(& imgb)).unwrap()).height {} else {
        __assert_fail(
            b"imga->height == imgb->height\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            31 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 57], &'_ [i8; 57]>(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    if (*(borrow(& imga)).unwrap()).nbands == (*(borrow(& imgb)).unwrap()).nbands {} else {
        __assert_fail(
            b"imga->nbands == imgb->nbands\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            32 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 57], &'_ [i8; 57]>(b"heman_image *heman_ops_max(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut result = heman_image_create((*(borrow_mut(&mut imga)).unwrap()).width, (*(borrow_mut(&mut imga)).unwrap()).height, (*(borrow_mut(&mut imga)).unwrap()).nbands);
    let mut size = (*(borrow(& imga)).unwrap()).height * (*(borrow(& imga)).unwrap()).width * (*(borrow(& imga)).unwrap()).nbands;
    let mut srca = (*(borrow_mut(&mut imga)).unwrap()).data;
    let mut srcb = (*(borrow_mut(&mut imgb)).unwrap()).data;
    let mut dst = (*result).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        *dst = if *srca > *srcb { *srca } else { *srcb };
        i += 1;
        dst = dst.offset(1);
        srca = srca.offset(1);
        srcb = srcb.offset(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_sweep<'a1>(
    mut hmap: Option<&'a1 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& hmap)).unwrap()).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            47 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 44], &'_ [i8; 44]>(b"heman_image *heman_ops_sweep(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut result = heman_image_create(
        (*(borrow_mut(&mut hmap)).unwrap()).height,
        1 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    );
    let mut dst = (*result).data;
    let mut src: * const f32 = (*(borrow(& hmap)).unwrap()).data;
    let mut invw = 1.0f32 / (*(borrow(& hmap)).unwrap()).width as std::os::raw::c_float;
    let mut y = 0 as std::os::raw::c_int;
    while y < (*(borrow(& hmap)).unwrap()).height {
        let mut acc = 0 as std::os::raw::c_int as std::os::raw::c_float;
        let mut x = 0 as std::os::raw::c_int;
        while x < (*(borrow(& hmap)).unwrap()).width {
            let mut fresh2 = src;
            src = src.offset(1);
            acc += *fresh2;
            x += 1;
        }
        let mut fresh3 = dst;
        dst = dst.offset(1);
        *fresh3 = acc * invw;
        y += 1;
    }
    return result;
}
unsafe extern "C" fn copy_row(
    mut src: * mut crate::src::src::color::heman_image_s,
    mut dst: * mut crate::src::src::color::heman_image_s,
    mut dstx: std::os::raw::c_int,
    mut y: std::os::raw::c_int,
) {
    let mut width = (*src).width;
    if (*src).nbands == 1 as std::os::raw::c_int {
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut srcp = heman_image_texel(src, x, y);
            let mut dstp = heman_image_texel(dst, dstx + x, y);
            *dstp = *srcp;
            x += 1;
        }
        return;
    }
    let mut x_0 = 0 as std::os::raw::c_int;
    while x_0 < width {
        let mut srcp_0 = heman_image_texel(src, x_0, y);
        let mut dstp_0 = heman_image_texel(dst, dstx + x_0, y);
        let mut nbands = (*src).nbands;
        loop {
            let mut fresh4 = nbands;
            nbands = nbands - 1;
            if !(fresh4 != 0) {
                break;
            }
            let mut fresh5 = srcp_0;
            srcp_0 = srcp_0.offset(1);
            let mut fresh6 = dstp_0;
            dstp_0 = dstp_0.offset(1);
            *fresh6 = *fresh5;
        }
        x_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_stitch_horizontal(
    mut images: * mut * mut crate::src::src::color::heman_image_s,
    mut count: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    if count > 0 as std::os::raw::c_int {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            85 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 62], &'_ [i8; 62]>(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                .as_ptr(),
        );
    }
    let mut width = (**images.offset(0 as std::os::raw::c_int as isize)).width;
    let mut height = (**images.offset(0 as std::os::raw::c_int as isize)).height;
    let mut nbands = (**images.offset(0 as std::os::raw::c_int as isize)).nbands;
    let mut i = 1 as std::os::raw::c_int;
    while i < count {
        if (**images.offset(i as isize)).width == width {} else {
            __assert_fail(
                b"images[i]->width == width\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
                90 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 62], &'_ [i8; 62]>(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        if (**images.offset(i as isize)).height == height {} else {
            __assert_fail(
                b"images[i]->height == height\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
                91 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 62], &'_ [i8; 62]>(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        if (**images.offset(i as isize)).nbands == nbands {} else {
            __assert_fail(
                b"images[i]->nbands == nbands\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
                92 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 62], &'_ [i8; 62]>(b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        i += 1;
    }
    let mut result = heman_image_create(width * count, height, nbands);
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut tile = 0 as std::os::raw::c_int;
        while tile < count {
            copy_row(*images.offset(tile as isize), result, tile * width, y);
            tile += 1;
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_stitch_vertical<'a1>(
    mut images: * mut Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut count: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    if count > 0 as std::os::raw::c_int {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            109 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 60], &'_ [i8; 60]>(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                .as_ptr(),
        );
    }
    let mut width = (*(borrow_mut(&mut *images.offset(0 as std::os::raw::c_int as isize))).unwrap()).width;
    let mut height = (*(borrow_mut(&mut *images.offset(0 as std::os::raw::c_int as isize))).unwrap()).height;
    let mut nbands = (*(borrow_mut(&mut *images.offset(0 as std::os::raw::c_int as isize))).unwrap()).nbands;
    let mut i = 1 as std::os::raw::c_int;
    while i < count {
        if (*(borrow(& *images.offset(i as isize))).unwrap()).width == width {} else {
            __assert_fail(
                b"images[i]->width == width\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
                114 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 60], &'_ [i8; 60]>(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        if (*(borrow(& *images.offset(i as isize))).unwrap()).height == height {} else {
            __assert_fail(
                b"images[i]->height == height\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
                115 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 60], &'_ [i8; 60]>(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        if (*(borrow(& *images.offset(i as isize))).unwrap()).nbands == nbands {} else {
            __assert_fail(
                b"images[i]->nbands == nbands\0" as *const u8 as *const std::os::raw::c_char,
                b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
                116 as std::os::raw::c_int as std::os::raw::c_uint,
                (*core::intrinsics::transmute::<&'_ [u8; 60], &'_ [i8; 60]>(b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0"))
                    .as_ptr(),
            );
        }
        i += 1;
    }
    let mut result = heman_image_create(width, height * count, nbands);
    let mut size = width * height * nbands;
    let mut dst = (*result).data;
    let mut tile = 0 as std::os::raw::c_int;
    while tile < count {
        memcpy(
            dst as *mut std::os::raw::c_void,
            (*(borrow(& *images.offset(tile as isize))).unwrap()).data as *const std::os::raw::c_void,
            (size as std::os::raw::c_ulong)
                .wrapping_mul(::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong),
        );
        dst = dst.offset(size as isize);
        tile += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_normalize_f32<'a1>(
    mut source: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut minv: std::os::raw::c_float,
    mut maxv: std::os::raw::c_float,
) -> * mut crate::src::src::color::heman_image_s {
    let mut result = heman_image_create(
        (*(borrow_mut(&mut source)).unwrap()).width,
        (*(borrow_mut(&mut source)).unwrap()).height,
        (*(borrow_mut(&mut source)).unwrap()).nbands,
    );
    let mut src = (*(borrow_mut(&mut source)).unwrap()).data;
    let mut dst = (*result).data;
    let mut scale = 1.0f32 / (maxv - minv);
    let mut size = (*(borrow(& source)).unwrap()).height * (*(borrow(& source)).unwrap()).width * (*(borrow(& source)).unwrap()).nbands;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut fresh7 = src;
        src = src.offset(1);
        let mut v = (*fresh7 - minv) * scale;
        let mut fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = if 0 as std::os::raw::c_int as std::os::raw::c_float
            > (if 1 as std::os::raw::c_int as std::os::raw::c_float > v {
                v
            } else {
                1 as std::os::raw::c_int as std::os::raw::c_float
            })
        {
            0 as std::os::raw::c_int as std::os::raw::c_float
        } else if 1 as std::os::raw::c_int as std::os::raw::c_float > v {
            v
        } else {
            1 as std::os::raw::c_int as std::os::raw::c_float
        };
        i += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_laplacian(
    mut heightmap: * mut crate::src::src::color::heman_image_s,
) -> * mut crate::src::src::color::heman_image_s {
    if (*heightmap).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"heightmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            146 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 48], &'_ [i8; 48]>(b"heman_image *heman_ops_laplacian(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut width = (*heightmap).width;
    let mut height = (*heightmap).height;
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut maxx = width - 1 as std::os::raw::c_int;
    let mut maxy = height - 1 as std::os::raw::c_int;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut y1 = if y + 1 as std::os::raw::c_int > maxy {
            maxy
        } else {
            y + 1 as std::os::raw::c_int
        };
        let mut dst = ((*result).data).offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut x1 = if x + 1 as std::os::raw::c_int > maxx {
                maxx
            } else {
                x + 1 as std::os::raw::c_int
            };
            let mut p = *heman_image_texel(heightmap, x, y);
            let mut px = *heman_image_texel(heightmap, x1, y);
            let mut py = *heman_image_texel(heightmap, x, y1);
            let mut fresh9 = dst;
            dst = dst.offset(1);
            *fresh9 = (p - px) * (p - px) + (p - py) * (p - py);
            x += 1;
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_accumulate<'a1, 'a2>(
    mut dst: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut src: Option<&'a2 mut crate::src::src::color::heman_image_s>,
) {
    if (*(borrow(& dst)).unwrap()).nbands == (*(borrow(& src)).unwrap()).nbands {} else {
        __assert_fail(
            b"dst->nbands == src->nbands\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            172 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 56], &'_ [i8; 56]>(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    if (*(borrow(& dst)).unwrap()).width == (*(borrow(& src)).unwrap()).width {} else {
        __assert_fail(
            b"dst->width == src->width\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            173 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 56], &'_ [i8; 56]>(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    if (*(borrow(& dst)).unwrap()).height == (*(borrow(& src)).unwrap()).height {} else {
        __assert_fail(
            b"dst->height == src->height\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            174 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 56], &'_ [i8; 56]>(b"void heman_ops_accumulate(heman_image *, heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut size = (*(borrow(& dst)).unwrap()).height * (*(borrow(& dst)).unwrap()).width;
    let mut sdata = (*(borrow_mut(&mut src)).unwrap()).data;
    let mut ddata = (*(borrow_mut(&mut dst)).unwrap()).data;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        let mut fresh10 = sdata;
        sdata = sdata.offset(1);
        let mut fresh11 = ddata;
        ddata = ddata.offset(1);
        *fresh11 += *fresh10;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_sobel<'a1>(
    mut img: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut rgb: std::os::raw::c_uint,
) -> * mut crate::src::src::color::heman_image_s {
    let mut width = (*(borrow_mut(&mut img)).unwrap()).width;
    let mut height = (*(borrow_mut(&mut img)).unwrap()).height;
    if (*(borrow(& img)).unwrap()).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"img->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            187 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 57], &'_ [i8; 57]>(b"heman_image *heman_ops_sobel(heman_image *, heman_color)\0"))
                .as_ptr(),
        );
    }
    let mut result = heman_image_create(width, height, 3 as std::os::raw::c_int);
    let mut gray = heman_color_to_grayscale(borrow_mut(&mut img));
    let mut inv = 1.0f32 / 255.0f32;
    let mut edge_rgb = kmVec3 { x: 0., y: 0., z: 0. };
    edge_rgb.x = (rgb >> 16 as std::os::raw::c_int) as std::os::raw::c_float * inv;
    edge_rgb
        .y = (rgb >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_float * inv;
    edge_rgb.z = (rgb & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float * inv;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = ((*result).data as *mut kmVec3).offset((y * width) as isize);
        let mut src: * const crate::src::kazmath::aabb3::kmVec3 = ((*(borrow_mut(&mut img)).unwrap()).data as *mut kmVec3)
            .offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut xm1 = if x - 1 as std::os::raw::c_int > 0 as std::os::raw::c_int {
                x - 1 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            };
            let mut xp1 = if x + 1 as std::os::raw::c_int > width - 1 as std::os::raw::c_int {
                width - 1 as std::os::raw::c_int
            } else {
                x + 1 as std::os::raw::c_int
            };
            let mut ym1 = if y - 1 as std::os::raw::c_int > 0 as std::os::raw::c_int {
                y - 1 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            };
            let mut yp1 = if y + 1 as std::os::raw::c_int > height - 1 as std::os::raw::c_int {
                height - 1 as std::os::raw::c_int
            } else {
                y + 1 as std::os::raw::c_int
            };
            let mut t00 = *heman_image_texel(gray, xm1, ym1);
            let mut t10 = *heman_image_texel(gray, x, ym1);
            let mut t20 = *heman_image_texel(gray, xp1, ym1);
            let mut t01 = *heman_image_texel(gray, xm1, 0 as std::os::raw::c_int);
            let mut t21 = *heman_image_texel(gray, xp1, 0 as std::os::raw::c_int);
            let mut t02 = *heman_image_texel(gray, xm1, yp1);
            let mut t12 = *heman_image_texel(gray, x, yp1);
            let mut t22 = *heman_image_texel(gray, xp1, yp1);
            let mut gx = (t00 as std::os::raw::c_double + 2.0f64 * t01 as std::os::raw::c_double
                + t02 as std::os::raw::c_double - t20 as std::os::raw::c_double
                - 2.0f64 * t21 as std::os::raw::c_double - t22 as std::os::raw::c_double)
                as std::os::raw::c_float;
            let mut gy = (t00 as std::os::raw::c_double + 2.0f64 * t10 as std::os::raw::c_double
                + t20 as std::os::raw::c_double - t02 as std::os::raw::c_double
                - 2.0f64 * t12 as std::os::raw::c_double - t22 as std::os::raw::c_double)
                as std::os::raw::c_float;
            let mut is_edge = ((gx * gx + gy * gy) as std::os::raw::c_double > 1e-5f64)
                as std::os::raw::c_int as std::os::raw::c_float;
            let mut fresh12 = dst;
            dst = dst.offset(1);
            let mut fresh13 = src;
            src = src.offset(1);
            kmVec3Lerp(fresh12, fresh13, Some(&mut edge_rgb), is_edge);
            x += 1;
        }
        y += 1;
    }
    heman_image_destroy(gray);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_warp_core(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut secondary: * mut crate::src::src::color::heman_image_s,
    mut seed: std::os::raw::c_int,
    mut octaves: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut width = (*img).width;
    let mut height = (*img).height;
    let mut nbands = (*img).nbands;
    let mut result = heman_image_create(width, height, nbands);
    let mut result2 = if !secondary.is_null() {
        heman_image_create(width, height, (*secondary).nbands)
    } else {
        0 as *mut heman_image
    };
    let mut invw = (1.0f64 / width as std::os::raw::c_double) as std::os::raw::c_float;
    let mut invh = (1.0f64 / height as std::os::raw::c_double) as std::os::raw::c_float;
    let mut inv = if invw > invh { invh } else { invw };
    let mut aspect = width as std::os::raw::c_float / height as std::os::raw::c_float;
    let mut gain = 0.6f64 as std::os::raw::c_float;
    let mut lacunarity = 2.0f64 as std::os::raw::c_float;
    let mut initial_amplitude = 0.05f64 as std::os::raw::c_float;
    let mut initial_frequency = 8.0f64 as std::os::raw::c_float;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = ((*result).data).offset((y * width * nbands) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut a = initial_amplitude;
            let mut f = initial_frequency;
            let mut src = 0 as *mut std::os::raw::c_float;
            if nbands == 4 as std::os::raw::c_int {
                src = heman_image_texel(img, x, y);
                let mut elev = 1 as std::os::raw::c_int as std::os::raw::c_float
                    - *src.offset(3 as std::os::raw::c_int as isize);
                a = (a as std::os::raw::c_double
                    * pow(elev as std::os::raw::c_double, 4 as std::os::raw::c_int as std::os::raw::c_double))
                    as std::os::raw::c_float;
            }
            let mut s = x as std::os::raw::c_float * inv;
            let mut t = y as std::os::raw::c_float * inv;
            let mut u = x as std::os::raw::c_float * invw;
            let mut v = y as std::os::raw::c_float * invh;
            let mut i = 0 as std::os::raw::c_int;
            while i < octaves {
                u = (u as std::os::raw::c_double
                    + a as std::os::raw::c_double
                        * open_simplex_noise2(
                            ctx,
                            (s * f) as std::os::raw::c_double,
                            (t * f) as std::os::raw::c_double,
                        )) as std::os::raw::c_float;
                v = (v as std::os::raw::c_double
                    + aspect as std::os::raw::c_double
                        * (a as std::os::raw::c_double
                            * open_simplex_noise2(
                                ctx,
                                (s * f) as std::os::raw::c_double + 0.5f64,
                                (t * f) as std::os::raw::c_double,
                            ))) as std::os::raw::c_float;
                a *= gain;
                f *= lacunarity;
                i += 1;
            }
            let mut i_0 = (if 0 as std::os::raw::c_int as std::os::raw::c_float
                > (if (width - 1 as std::os::raw::c_int) as std::os::raw::c_float
                    > u * width as std::os::raw::c_float
                {
                    u * width as std::os::raw::c_float
                } else {
                    (width - 1 as std::os::raw::c_int) as std::os::raw::c_float
                })
            {
                0 as std::os::raw::c_int as std::os::raw::c_float
            } else if (width - 1 as std::os::raw::c_int) as std::os::raw::c_float
                > u * width as std::os::raw::c_float
            {
                u * width as std::os::raw::c_float
            } else {
                (width - 1 as std::os::raw::c_int) as std::os::raw::c_float
            }) as std::os::raw::c_int;
            let mut j = (if 0 as std::os::raw::c_int as std::os::raw::c_float
                > (if (height - 1 as std::os::raw::c_int) as std::os::raw::c_float
                    > v * height as std::os::raw::c_float
                {
                    v * height as std::os::raw::c_float
                } else {
                    (height - 1 as std::os::raw::c_int) as std::os::raw::c_float
                })
            {
                0 as std::os::raw::c_int as std::os::raw::c_float
            } else if (height - 1 as std::os::raw::c_int) as std::os::raw::c_float
                > v * height as std::os::raw::c_float
            {
                v * height as std::os::raw::c_float
            } else {
                (height - 1 as std::os::raw::c_int) as std::os::raw::c_float
            }) as std::os::raw::c_int;
            src = heman_image_texel(img, i_0, j);
            let mut n = 0 as std::os::raw::c_int;
            while n < nbands {
                let mut fresh14 = src;
                src = src.offset(1);
                let mut fresh15 = dst;
                dst = dst.offset(1);
                *fresh15 = *fresh14;
                n += 1;
            }
            if !secondary.is_null() {
                src = heman_image_texel(secondary, x, y);
                let mut dst2 = heman_image_texel(result2, i_0, j);
                let mut n_0 = 0 as std::os::raw::c_int;
                while n_0 < (*secondary).nbands {
                    let mut fresh16 = src;
                    src = src.offset(1);
                    let mut fresh17 = dst2;
                    dst2 = dst2.offset(1);
                    *fresh17 = *fresh16;
                    n_0 += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    if !secondary.is_null() {
        free((*secondary).data as *mut std::os::raw::c_void);
        let ref mut fresh18 = (*secondary).data;
        *fresh18 = (*result2).data;
        free(result2 as *mut std::os::raw::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_warp_points<'a1>(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut seed: std::os::raw::c_int,
    mut octaves: std::os::raw::c_int,
    mut pts: Option<&'a1 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    let mut width = (*img).width;
    let mut height = (*img).height;
    let mut mapping = heman_distance_identity_cpcf(width, height);
    let mut retval = heman_ops_warp_core(img, mapping, seed, octaves);
    let mut src = (*(borrow_mut(&mut pts)).unwrap()).data;
    let mut k = 0 as std::os::raw::c_int;
    while k < (*(borrow(& pts)).unwrap()).width {
        let mut x = *src.offset(0 as std::os::raw::c_int as isize);
        let mut y = *src.offset(1 as std::os::raw::c_int as isize);
        let mut i = (x * (*mapping).width as std::os::raw::c_float) as std::os::raw::c_int;
        let mut j = (y * (*mapping).height as std::os::raw::c_float) as std::os::raw::c_int;
        if !(i < 0 as std::os::raw::c_int || i >= (*mapping).width || j < 0 as std::os::raw::c_int
            || j >= (*mapping).height)
        {
            let mut texel = heman_image_texel(mapping, i, j);
            *src
                .offset(
                    0 as std::os::raw::c_int as isize,
                ) = *texel.offset(0 as std::os::raw::c_int as isize)
                / (*mapping).width as std::os::raw::c_float;
            *src
                .offset(
                    1 as std::os::raw::c_int as isize,
                ) = *texel.offset(1 as std::os::raw::c_int as isize)
                / (*mapping).height as std::os::raw::c_float;
        }
        k += 1;
        src = src.offset((*(borrow(& pts)).unwrap()).nbands as isize);
    }
    heman_image_destroy(mapping);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_warp(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut seed: std::os::raw::c_int,
    mut octaves: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    return heman_ops_warp_core(img, 0 as *mut heman_image, seed, octaves);
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_extract_mask(
    mut source: * mut crate::src::src::color::heman_image_s,
    mut color: std::os::raw::c_uint,
    mut invert: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    if (*source).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"source->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            330 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 69], &'_ [i8; 69]>(b"heman_image *heman_ops_extract_mask(heman_image *, heman_color, int)\0"))
                .as_ptr(),
        );
    }
    let mut inv = 1.0f32 / 255.0f32;
    let mut r = (color >> 16 as std::os::raw::c_int) as std::os::raw::c_float * inv;
    let mut g = (color >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_float * inv;
    let mut b = (color & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float * inv;
    let mut height = (*source).height;
    let mut width = (*source).width;
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = ((*result).data).offset((y * width) as isize);
        let mut src = ((*source).data).offset((y * width * 3 as std::os::raw::c_int) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut val = (*src.offset(0 as std::os::raw::c_int as isize) == r
                && *src.offset(1 as std::os::raw::c_int as isize) == g
                && *src.offset(2 as std::os::raw::c_int as isize) == b) as std::os::raw::c_int
                as std::os::raw::c_float;
            if invert == 0 {
                val = 1 as std::os::raw::c_int as std::os::raw::c_float - val;
            }
            let mut fresh19 = dst;
            dst = dst.offset(1);
            *fresh19 = val;
            x += 1;
            src = src.offset(3 as std::os::raw::c_int as isize);
        }
        y += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_replace_color<'a1, 'a2>(
    mut source: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut color: std::os::raw::c_uint,
    mut texture: Option<&'a2 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& source)).unwrap()).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"source->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            359 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 80], &'_ [i8; 80]>(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*(borrow(& texture)).unwrap()).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"texture->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            360 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 80], &'_ [i8; 80]>(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut height = (*(borrow_mut(&mut source)).unwrap()).height;
    let mut width = (*(borrow_mut(&mut source)).unwrap()).width;
    if (*(borrow(& texture)).unwrap()).width == width {} else {
        __assert_fail(
            b"texture->width == width\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            363 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 80], &'_ [i8; 80]>(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    if (*(borrow(& texture)).unwrap()).height == height {} else {
        __assert_fail(
            b"texture->height == height\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            364 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 80], &'_ [i8; 80]>(
                b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",
            ))
                .as_ptr(),
        );
    }
    let mut inv = 1.0f32 / 255.0f32;
    let mut r = (color >> 16 as std::os::raw::c_int) as std::os::raw::c_float * inv;
    let mut g = (color >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_float * inv;
    let mut b = (color & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float * inv;
    let mut result = heman_image_create(width, height, 3 as std::os::raw::c_int);
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = ((*result).data).offset((y * width * 3 as std::os::raw::c_int) as isize);
        let mut src = ((*(borrow(& source)).unwrap()).data).offset((y * width * 3 as std::os::raw::c_int) as isize);
        let mut tex = ((*(borrow(& texture)).unwrap()).data).offset((y * width * 3 as std::os::raw::c_int) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            if *src.offset(0 as std::os::raw::c_int as isize) == r
                && *src.offset(1 as std::os::raw::c_int as isize) == g
                && *src.offset(2 as std::os::raw::c_int as isize) == b
            {
                *dst
                    .offset(
                        0 as std::os::raw::c_int as isize,
                    ) = *tex.offset(0 as std::os::raw::c_int as isize);
                *dst
                    .offset(
                        1 as std::os::raw::c_int as isize,
                    ) = *tex.offset(1 as std::os::raw::c_int as isize);
                *dst
                    .offset(
                        2 as std::os::raw::c_int as isize,
                    ) = *tex.offset(2 as std::os::raw::c_int as isize);
            } else {
                *dst
                    .offset(
                        0 as std::os::raw::c_int as isize,
                    ) = *src.offset(0 as std::os::raw::c_int as isize);
                *dst
                    .offset(
                        1 as std::os::raw::c_int as isize,
                    ) = *src.offset(1 as std::os::raw::c_int as isize);
                *dst
                    .offset(
                        2 as std::os::raw::c_int as isize,
                    ) = *src.offset(2 as std::os::raw::c_int as isize);
            }
            x += 1;
            src = src.offset(3 as std::os::raw::c_int as isize);
            dst = dst.offset(3 as std::os::raw::c_int as isize);
            tex = tex.offset(3 as std::os::raw::c_int as isize);
        }
        y += 1;
    }
    return result;
}
unsafe extern "C" fn _match<'a1>(
    mut mask: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut mask_color: std::os::raw::c_uint,
    mut invert_mask: std::os::raw::c_int,
    mut pixel_index: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut mcolor = ((*(borrow(& mask)).unwrap()).data).offset((pixel_index * 3 as std::os::raw::c_int) as isize);
    let mut r1 = (*mcolor.offset(0 as std::os::raw::c_int as isize)
        * 255 as std::os::raw::c_int as std::os::raw::c_float) as std::os::raw::c_uchar;
    let mut g1 = (*mcolor.offset(1 as std::os::raw::c_int as isize)
        * 255 as std::os::raw::c_int as std::os::raw::c_float) as std::os::raw::c_uchar;
    let mut b1 = (*mcolor.offset(2 as std::os::raw::c_int as isize)
        * 255 as std::os::raw::c_int as std::os::raw::c_float) as std::os::raw::c_uchar;
    let mut r2 = (mask_color >> 16 as std::os::raw::c_int) as std::os::raw::c_uchar;
    let mut g2 = (mask_color >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_uchar;
    let mut b2 = (mask_color & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_uchar;
    let mut retval = (r1 as std::os::raw::c_int == r2 as std::os::raw::c_int
        && g1 as std::os::raw::c_int == g2 as std::os::raw::c_int
        && b1 as std::os::raw::c_int == b2 as std::os::raw::c_int) as std::os::raw::c_int;
    return if invert_mask != 0 { 1 as std::os::raw::c_int - retval } else { retval };
}
unsafe extern "C" fn qselect(
    mut v: * mut std::os::raw::c_float,
    mut len: std::os::raw::c_int,
    mut k: std::os::raw::c_int,
) -> std::os::raw::c_float {
    let mut i: i32 = 0;
    let mut st: i32 = 0;
    i = 0 as std::os::raw::c_int;
    st = i;
    while i < len - 1 as std::os::raw::c_int {
        if !(*v.offset(i as isize) > *v.offset((len - 1 as std::os::raw::c_int) as isize)) {
            let mut f = *v.offset(i as isize);
            *v.offset(i as isize) = *v.offset(st as isize);
            *v.offset(st as isize) = f;
            st += 1;
        }
        i += 1;
    }
    let mut __0 = *v.offset((len - 1 as std::os::raw::c_int) as isize);
    *v.offset((len - 1 as std::os::raw::c_int) as isize) = *v.offset(st as isize);
    *v.offset(st as isize) = __0;
    return if k == st {
        *v.offset(st as isize)
    } else if st > k {
        qselect(v, st, k)
    } else {
        qselect(v.offset(st as isize), len - st, k - st)
    };
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_percentiles<'a1, 'a2, 'a3>(
    mut hmap: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut nsteps: std::os::raw::c_int,
    mut mask: Option<&'a2 mut crate::src::src::color::heman_image_s>,
    mut mask_color: std::os::raw::c_uint,
    mut invert_mask: std::os::raw::c_int,
    mut offset: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::src::color::heman_image_s> where 'a1: 'a3 {
    if (*(borrow(& hmap)).unwrap()).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            427 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 95], &'_ [i8; 95]>(
                b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    if borrow(& mask).is_none() || (*(borrow(& mask)).unwrap()).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"!mask || mask->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            428 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 95], &'_ [i8; 95]>(
                b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    let mut size = (*(borrow(& hmap)).unwrap()).height * (*(borrow(& hmap)).unwrap()).width;
    let mut src = (*(borrow_mut(&mut hmap)).unwrap()).data;
    let mut minv = 1000 as std::os::raw::c_int as std::os::raw::c_float;
    let mut maxv = -(1000 as std::os::raw::c_int) as std::os::raw::c_float;
    let mut npixels = 0 as std::os::raw::c_int;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        if borrow(& mask).is_none() || _match(borrow_mut(&mut mask), mask_color, invert_mask, i) != 0 {
            minv = if minv > *src.offset(i as isize) {
                *src.offset(i as isize)
            } else {
                minv
            };
            maxv = if maxv > *src.offset(i as isize) {
                maxv
            } else {
                *src.offset(i as isize)
            };
            npixels += 1;
        }
        i += 1;
    }
    let mut vals = malloc(
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(npixels as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_float;
    npixels = 0 as std::os::raw::c_int;
    let mut i_0 = 0 as std::os::raw::c_int;
    while i_0 < size {
        if borrow(& mask).is_none() || _match(borrow_mut(&mut mask), mask_color, invert_mask, i_0) != 0 {
            let mut fresh20 = npixels;
            npixels = npixels + 1;
            *vals.offset(fresh20 as isize) = *src.offset(i_0 as isize);
        }
        i_0 += 1;
    }
    let mut percentiles = malloc(
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(nsteps as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_float;
    let mut tier = 0 as std::os::raw::c_int;
    while tier < nsteps {
        let mut height = qselect(vals, npixels, tier * npixels / nsteps);
        *percentiles.offset(tier as isize) = height;
        tier += 1;
    }
    free(vals as *mut std::os::raw::c_void);
    let mut i_1 = 0 as std::os::raw::c_int;
    while i_1 < size {
        let mut e = *src;
        if borrow(& mask).is_none() || _match(borrow_mut(&mut mask), mask_color, invert_mask, i_1) != 0 {
            let mut tier_0 = nsteps - 1 as std::os::raw::c_int;
            while tier_0 >= 0 as std::os::raw::c_int {
                if e > *percentiles.offset(tier_0 as isize) {
                    e = *percentiles.offset(tier_0 as isize);
                    break;
                } else {
                    tier_0 -= 1;
                }
            }
        }
        let mut fresh21 = src;
        src = src.offset(1);
        *fresh21 = e + offset;
        i_1 += 1;
    }
    free(percentiles as *mut std::os::raw::c_void);
    return hmap;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_stairstep<'a1, 'a2, 'a3>(
    mut hmap: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut nsteps: std::os::raw::c_int,
    mut mask: Option<&'a2 mut crate::src::src::color::heman_image_s>,
    mut mask_color: std::os::raw::c_uint,
    mut invert_mask: std::os::raw::c_int,
    mut offset: std::os::raw::c_float,
) -> Option<&'a3 mut crate::src::src::color::heman_image_s> where 'a1: 'a3 {
    if (*(borrow(& hmap)).unwrap()).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            477 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 93], &'_ [i8; 93]>(
                b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    if borrow(& mask).is_none() || (*(borrow(& mask)).unwrap()).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"!mask || mask->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            478 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 93], &'_ [i8; 93]>(
                b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",
            ))
                .as_ptr(),
        );
    }
    let mut size = (*(borrow(& hmap)).unwrap()).height * (*(borrow(& hmap)).unwrap()).width;
    let mut src = (*(borrow_mut(&mut hmap)).unwrap()).data;
    let mut minv = 1000 as std::os::raw::c_int as std::os::raw::c_float;
    let mut maxv = -(1000 as std::os::raw::c_int) as std::os::raw::c_float;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        if borrow(& mask).is_none() || _match(borrow_mut(&mut mask), mask_color, invert_mask, i) != 0 {
            minv = if minv > *src.offset(i as isize) {
                *src.offset(i as isize)
            } else {
                minv
            };
            maxv = if maxv > *src.offset(i as isize) {
                maxv
            } else {
                *src.offset(i as isize)
            };
        }
        i += 1;
    }
    let mut range = maxv - minv;
    let mut i_0 = 0 as std::os::raw::c_int;
    while i_0 < size {
        let mut e = *src;
        if borrow(& mask).is_none() || _match(borrow_mut(&mut mask), mask_color, invert_mask, i_0) != 0 {
            e = e - minv;
            e /= range;
            e = (floor((e * nsteps as std::os::raw::c_float) as std::os::raw::c_double)
                / nsteps as std::os::raw::c_double) as std::os::raw::c_float;
            e = e * range + minv;
        }
        let mut fresh22 = src;
        src = src.offset(1);
        *fresh22 = e + offset;
        i_0 += 1;
    }
    return hmap;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_merge_political<'a1, 'a2>(
    mut hmap: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut cmap: Option<&'a2 mut crate::src::src::color::heman_image_s>,
    mut ocean: std::os::raw::c_uint,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& hmap)).unwrap()).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"hmap->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            506 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 82], &'_ [i8; 82]>(
                b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",
            ))
                .as_ptr(),
        );
    }
    if (*(borrow(& cmap)).unwrap()).nbands == 3 as std::os::raw::c_int {} else {
        __assert_fail(
            b"cmap->nbands == 3\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            507 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 82], &'_ [i8; 82]>(
                b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",
            ))
                .as_ptr(),
        );
    }
    let mut result = heman_image_create((*(borrow_mut(&mut hmap)).unwrap()).width, (*(borrow_mut(&mut hmap)).unwrap()).height, 4 as std::os::raw::c_int);
    let mut pheight = (*(borrow_mut(&mut hmap)).unwrap()).data;
    let mut pcolour = (*(borrow_mut(&mut cmap)).unwrap()).data;
    let mut pmerged = (*result).data;
    let mut inv = 1.0f32 / 255.0f32;
    let mut oceanr = (ocean >> 16 as std::os::raw::c_int) as std::os::raw::c_float * inv;
    let mut oceang = (ocean >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
        as std::os::raw::c_float * inv;
    let mut oceanb = (ocean & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_float
        * inv;
    let mut size = (*(borrow(& hmap)).unwrap()).height * (*(borrow(& hmap)).unwrap()).width;
    let mut minh = 1000 as std::os::raw::c_int as std::os::raw::c_float;
    let mut maxh = -(1000 as std::os::raw::c_int) as std::os::raw::c_float;
    let mut i = 0 as std::os::raw::c_int;
    while i < size {
        minh = if minh > *pheight.offset(i as isize) {
            *pheight.offset(i as isize)
        } else {
            minh
        };
        maxh = if maxh > *pheight.offset(i as isize) {
            *pheight.offset(i as isize)
        } else {
            maxh
        };
        i += 1;
    }
    let mut i_0 = 0 as std::os::raw::c_int;
    while i_0 < size {
        let mut fresh23 = pheight;
        pheight = pheight.offset(1);
        let mut h = *fresh23;
        if h < 0 as std::os::raw::c_int as std::os::raw::c_float {
            let mut fresh24 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh24 = oceanr;
            let mut fresh25 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh25 = oceang;
            let mut fresh26 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh26 = oceanb;
            pcolour = pcolour.offset(3 as std::os::raw::c_int as isize);
        } else {
            let mut fresh27 = pcolour;
            pcolour = pcolour.offset(1);
            let mut fresh28 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh28 = *fresh27;
            let mut fresh29 = pcolour;
            pcolour = pcolour.offset(1);
            let mut fresh30 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh30 = *fresh29;
            let mut fresh31 = pcolour;
            pcolour = pcolour.offset(1);
            let mut fresh32 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh32 = *fresh31;
        }
        let mut fresh33 = pmerged;
        pmerged = pmerged.offset(1);
        *fresh33 = (h - minh) / (maxh - minh);
        i_0 += 1;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn heman_ops_emboss(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut mode: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut seed = 1 as std::os::raw::c_int;
    let mut octaves = 4 as std::os::raw::c_int;
    let mut ctx = 0 as *mut osn_context;
    open_simplex_noise(seed as int64_t, Some(&mut ctx));
    let mut width = (*img).width;
    let mut height = (*img).height;
    if (*img).nbands == 1 as std::os::raw::c_int {} else {
        __assert_fail(
            b"img->nbands == 1\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/ops.c\0" as *const u8 as *const std::os::raw::c_char,
            549 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 50], &'_ [i8; 50]>(b"heman_image *heman_ops_emboss(heman_image *, int)\0"))
                .as_ptr(),
        );
    }
    let mut result = heman_image_create(width, height, 1 as std::os::raw::c_int);
    let mut invw = (1.0f64 / width as std::os::raw::c_double) as std::os::raw::c_float;
    let mut invh = (1.0f64 / height as std::os::raw::c_double) as std::os::raw::c_float;
    let mut inv = if invw > invh { invh } else { invw };
    let mut gain = 0.6f64 as std::os::raw::c_float;
    let mut lacunarity = 2.0f64 as std::os::raw::c_float;
    let mut land_amplitude = 0.0005f64 as std::os::raw::c_float;
    let mut land_frequency = 256.0f64 as std::os::raw::c_float;
    let mut ocean_amplitude = 0.5f64 as std::os::raw::c_float;
    let mut ocean_frequency = 1.0f64 as std::os::raw::c_float;
    let mut y: i32 = 0;
    y = 0 as std::os::raw::c_int;
    while y < height {
        let mut dst = ((*result).data).offset((y * width) as isize);
        let mut x = 0 as std::os::raw::c_int;
        while x < width {
            let mut z = *heman_image_texel(img, x, y);
            if z > 0 as std::os::raw::c_int as std::os::raw::c_float && mode == 1 as std::os::raw::c_int {
                let mut s = x as std::os::raw::c_float * inv;
                let mut t = y as std::os::raw::c_float * inv;
                let mut a = land_amplitude;
                let mut f = land_frequency;
                let mut i = 0 as std::os::raw::c_int;
                while i < octaves {
                    z = (z as std::os::raw::c_double
                        + a as std::os::raw::c_double
                            * open_simplex_noise2(
                                ctx,
                                (s * f) as std::os::raw::c_double,
                                (t * f) as std::os::raw::c_double,
                            )) as std::os::raw::c_float;
                    a *= gain;
                    f *= lacunarity;
                    i += 1;
                }
            } else if z <= 0 as std::os::raw::c_int as std::os::raw::c_float
                && mode == -(1 as std::os::raw::c_int)
            {
                z = (if z as std::os::raw::c_double > -0.1f64 {
                    z as std::os::raw::c_double
                } else {
                    -0.1f64
                }) as std::os::raw::c_float;
                let mut soften = fabsf(z);
                let mut s_0 = x as std::os::raw::c_float * inv;
                let mut t_0 = y as std::os::raw::c_float * inv;
                let mut a_0 = ocean_amplitude;
                let mut f_0 = ocean_frequency;
                let mut i_0 = 0 as std::os::raw::c_int;
                while i_0 < octaves {
                    z = (z as std::os::raw::c_double
                        + soften as std::os::raw::c_double
                            * (a_0 as std::os::raw::c_double
                                * open_simplex_noise2(
                                    ctx,
                                    (s_0 * f_0) as std::os::raw::c_double,
                                    (t_0 * f_0) as std::os::raw::c_double,
                                ))) as std::os::raw::c_float;
                    a_0 *= gain;
                    f_0 *= lacunarity;
                    i_0 += 1;
                }
            }
            let mut fresh34 = dst;
            dst = dst.offset(1);
            *fresh34 = z;
            x += 1;
        }
        y += 1;
    }
    open_simplex_noise_free(ctx);
    return result;
}
use crate::laertes_rt::*;
