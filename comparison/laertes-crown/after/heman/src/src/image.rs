
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
}
// #[derive(Copy, Clone)]

pub type heman_image_s = crate::src::src::color::heman_image_s;
pub type heman_image = crate::src::src::color::heman_image_s;
#[no_mangle]
pub unsafe extern "C" fn heman_image_data<'a1>(
    mut img: Option<&'a1 mut crate::src::src::color::heman_image_s>,
) -> * mut std::os::raw::c_float {
    return (*(borrow_mut(&mut img)).unwrap()).data;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_array<'a1, 'a2, 'a3>(
    mut img: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut data: Option<&'a2 mut * mut std::os::raw::c_float>,
    mut nfloats: Option<&'a3 mut std::os::raw::c_int>,
) {
    *(borrow_mut(&mut data)).unwrap() = (*(borrow_mut(&mut img)).unwrap()).data;
    *(borrow_mut(&mut nfloats)).unwrap() = (*(borrow(& img)).unwrap()).width * (*(borrow(& img)).unwrap()).height * (*(borrow(& img)).unwrap()).nbands;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_info<'a1, 'a2, 'a3, 'a4>(
    mut img: Option<&'a1 mut crate::src::src::color::heman_image_s>,
    mut width: Option<&'a2 mut std::os::raw::c_int>,
    mut height: Option<&'a3 mut std::os::raw::c_int>,
    mut nbands: Option<&'a4 mut std::os::raw::c_int>,
) {
    *(borrow_mut(&mut width)).unwrap() = (*(borrow_mut(&mut img)).unwrap()).width;
    *(borrow_mut(&mut height)).unwrap() = (*(borrow_mut(&mut img)).unwrap()).height;
    *(borrow_mut(&mut nbands)).unwrap() = (*(borrow_mut(&mut img)).unwrap()).nbands;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_texel(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut x: std::os::raw::c_int,
    mut y: std::os::raw::c_int,
) -> * mut std::os::raw::c_float {
    return ((*img).data)
        .offset((y * (*img).width * (*img).nbands) as isize)
        .offset((x * (*img).nbands) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_create(
    mut width: std::os::raw::c_int,
    mut height: std::os::raw::c_int,
    mut nbands: std::os::raw::c_int,
) -> * mut crate::src::src::color::heman_image_s {
    let mut img = malloc(::std::mem::size_of::<heman_image>() as std::os::raw::c_ulong)
        as *mut heman_image;
    (*img).width = width;
    (*img).height = height;
    (*img).nbands = nbands;
    let ref mut fresh0 = (*img).data;
    *fresh0 = malloc(
        (::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong)
            .wrapping_mul(width as std::os::raw::c_ulong)
            .wrapping_mul(height as std::os::raw::c_ulong)
            .wrapping_mul(nbands as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_float;
    return img;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_destroy(mut img: * mut crate::src::src::color::heman_image_s) {
    free((*img).data as *mut std::os::raw::c_void);
    free(img as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_sample(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut u: std::os::raw::c_float,
    mut v: std::os::raw::c_float,
    mut result: * mut std::os::raw::c_float,
) {
    let mut x = (if 0 as std::os::raw::c_int as std::os::raw::c_float
        > (if ((*img).width - 1 as std::os::raw::c_int) as std::os::raw::c_float
            > (*img).width as std::os::raw::c_float * u
        {
            (*img).width as std::os::raw::c_float * u
        } else {
            ((*img).width - 1 as std::os::raw::c_int) as std::os::raw::c_float
        })
    {
        0 as std::os::raw::c_int as std::os::raw::c_float
    } else if ((*img).width - 1 as std::os::raw::c_int) as std::os::raw::c_float
        > (*img).width as std::os::raw::c_float * u
    {
        (*img).width as std::os::raw::c_float * u
    } else {
        ((*img).width - 1 as std::os::raw::c_int) as std::os::raw::c_float
    }) as std::os::raw::c_int;
    let mut y = (if 0 as std::os::raw::c_int as std::os::raw::c_float
        > (if ((*img).height - 1 as std::os::raw::c_int) as std::os::raw::c_float
            > (*img).height as std::os::raw::c_float * v
        {
            (*img).height as std::os::raw::c_float * v
        } else {
            ((*img).height - 1 as std::os::raw::c_int) as std::os::raw::c_float
        })
    {
        0 as std::os::raw::c_int as std::os::raw::c_float
    } else if ((*img).height - 1 as std::os::raw::c_int) as std::os::raw::c_float
        > (*img).height as std::os::raw::c_float * v
    {
        (*img).height as std::os::raw::c_float * v
    } else {
        ((*img).height - 1 as std::os::raw::c_int) as std::os::raw::c_float
    }) as std::os::raw::c_int;
    let mut data = heman_image_texel(img, x, y);
    let mut b = 0 as std::os::raw::c_int;
    while b < (*img).nbands {
        let mut fresh1 = data;
        data = data.offset(1);
        let mut fresh2 = result;
        result = result.offset(1);
        *fresh2 = *fresh1;
        b += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_clear(
    mut img: * mut crate::src::src::color::heman_image_s,
    mut value: std::os::raw::c_float,
) {
    let mut size = (*img).width * (*img).height * (*img).nbands;
    let mut dst = (*img).data;
    loop {
        let mut fresh3 = size;
        size = size - 1;
        if !(fresh3 != 0) {
            break;
        }
        let mut fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 = value;
    };
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_extract_alpha<'a1>(
    mut img: Option<&'a1 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& img)).unwrap()).nbands == 4 as std::os::raw::c_int {} else {
        __assert_fail(
            b"img->nbands == 4\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/image.c\0" as *const u8 as *const std::os::raw::c_char,
            63 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 54], &'_ [i8; 54]>(b"heman_image *heman_image_extract_alpha(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut retval = heman_image_create((*(borrow_mut(&mut img)).unwrap()).width, (*(borrow_mut(&mut img)).unwrap()).height, 1 as std::os::raw::c_int);
    let mut size = (*(borrow(& img)).unwrap()).width * (*(borrow(& img)).unwrap()).height;
    let mut src = (*(borrow_mut(&mut img)).unwrap()).data;
    let mut dst = (*retval).data;
    loop {
        let mut fresh5 = size;
        size = size - 1;
        if !(fresh5 != 0) {
            break;
        }
        src = src.offset(3 as std::os::raw::c_int as isize);
        let mut fresh6 = src;
        src = src.offset(1);
        let mut fresh7 = dst;
        dst = dst.offset(1);
        *fresh7 = *fresh6;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn heman_image_extract_rgb<'a1>(
    mut img: Option<&'a1 mut crate::src::src::color::heman_image_s>,
) -> * mut crate::src::src::color::heman_image_s {
    if (*(borrow(& img)).unwrap()).nbands == 4 as std::os::raw::c_int {} else {
        __assert_fail(
            b"img->nbands == 4\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/image.c\0" as *const u8 as *const std::os::raw::c_char,
            77 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 52], &'_ [i8; 52]>(b"heman_image *heman_image_extract_rgb(heman_image *)\0"))
                .as_ptr(),
        );
    }
    let mut retval = heman_image_create((*(borrow_mut(&mut img)).unwrap()).width, (*(borrow_mut(&mut img)).unwrap()).height, 3 as std::os::raw::c_int);
    let mut size = (*(borrow(& img)).unwrap()).width * (*(borrow(& img)).unwrap()).height;
    let mut src = (*(borrow_mut(&mut img)).unwrap()).data;
    let mut dst = (*retval).data;
    loop {
        let mut fresh8 = size;
        size = size - 1;
        if !(fresh8 != 0) {
            break;
        }
        let mut fresh9 = src;
        src = src.offset(1);
        let mut fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = *fresh9;
        let mut fresh11 = src;
        src = src.offset(1);
        let mut fresh12 = dst;
        dst = dst.offset(1);
        *fresh12 = *fresh11;
        let mut fresh13 = src;
        src = src.offset(1);
        let mut fresh14 = dst;
        dst = dst.offset(1);
        *fresh14 = *fresh13;
        src = src.offset(1);
    }
    return retval;
}
use crate::laertes_rt::*;
