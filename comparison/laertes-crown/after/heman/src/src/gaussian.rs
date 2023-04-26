
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
#[no_mangle]
pub unsafe extern "C" fn generate_gaussian_row(
    mut target: * mut std::os::raw::c_int,
    mut fwidth: std::os::raw::c_int,
) {
    if fwidth > 0 as std::os::raw::c_int {} else {
        __assert_fail(
            b"fwidth > 0\0" as *const u8 as *const std::os::raw::c_char,
            b"../src/gaussian.c\0" as *const u8 as *const std::os::raw::c_char,
            9 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 39], &'_ [i8; 39]>(b"void generate_gaussian_row(int *, int)\0"))
                .as_ptr(),
        );
    }
    let mut nbytes = (fwidth as std::os::raw::c_ulong)
        .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    let mut tmp = malloc(nbytes as std::os::raw::c_ulong) as *mut std::os::raw::c_int;
    let ref mut fresh0 = *tmp.offset(0 as std::os::raw::c_int as isize);
    *fresh0 = 1 as std::os::raw::c_int;
    *target.offset(0 as std::os::raw::c_int as isize) = *fresh0;
    let mut col = 1 as std::os::raw::c_int;
    while col < fwidth {
        *target.offset(col as isize) = 0 as std::os::raw::c_int;
        *tmp.offset(col as isize) = 0 as std::os::raw::c_int;
        col += 1;
    }
    let mut row = 1 as std::os::raw::c_int;
    while row < fwidth {
        let mut col_0 = 1 as std::os::raw::c_int;
        while col_0 <= row {
            *target
                .offset(
                    col_0 as isize,
                ) = *tmp.offset(col_0 as isize)
                + *tmp.offset((col_0 - 1 as std::os::raw::c_int) as isize);
            col_0 += 1;
        }
        let mut col_1 = 1 as std::os::raw::c_int;
        while col_1 <= row {
            *tmp.offset(col_1 as isize) = *target.offset(col_1 as isize);
            col_1 += 1;
        }
        row += 1;
    }
    free(tmp as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn generate_gaussian_splat(
    mut target: * mut std::os::raw::c_float,
    mut fwidth: std::os::raw::c_int,
) {
    let mut gaussian_row = malloc(
        (fwidth as std::os::raw::c_ulong)
            .wrapping_mul(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_int;
    generate_gaussian_row(gaussian_row, fwidth);
    let mut shift = (1 as std::os::raw::c_int) << fwidth - 1 as std::os::raw::c_int;
    let mut scale = (1.0f64 / (shift * shift) as std::os::raw::c_double) as std::os::raw::c_float;
    let mut gptr = target;
    let mut j = 0 as std::os::raw::c_int;
    while j < fwidth {
        let mut i = 0 as std::os::raw::c_int;
        while i < fwidth {
            let mut fresh1 = gptr;
            gptr = gptr.offset(1);
            *fresh1 = (*gaussian_row.offset(i as isize)
                * *gaussian_row.offset(j as isize)) as std::os::raw::c_float * scale;
            i += 1;
        }
        j += 1;
    }
    free(gaussian_row as *mut std::os::raw::c_void);
}
use crate::laertes_rt::*;
