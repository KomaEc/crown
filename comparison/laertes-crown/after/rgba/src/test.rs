
extern "C" {
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    
    
    
}
pub use crate::src::src::rgba::rgba_from_string;
pub use crate::src::src::rgba::rgba_new;
pub use crate::src::src::rgba::rgba_to_string;
pub type size_t = std::os::raw::c_ulong;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type int32_t = std::os::raw::c_int;
pub type uint32_t = std::os::raw::c_uint;
// #[derive(Copy, Clone)]

pub type rgba_t = crate::src::src::rgba::rgba_t;
#[no_mangle]
pub unsafe extern "C" fn test_named() {
    let mut ok: i16 = 0;
    let mut val = rgba_from_string(
        b"olive\0" as *const u8 as *const std::os::raw::c_char,
        Some(&mut ok),
    ) as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            15 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 18], &'_ [i8; 18]>(b"void test_named()\0"))
                .as_ptr(),
        );
    };
    if 0x808000ff as std::os::raw::c_uint == val as std::os::raw::c_uint {} else {
        __assert_fail(
            b"0x808000FF == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            16 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 18], &'_ [i8; 18]>(b"void test_named()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_rgb() {
    let mut ok: i16 = 0;
    let mut val = rgba_from_string(
        b"rgb(255, 30   , 0)\0" as *const u8 as *const std::os::raw::c_char,
        Some(&mut ok),
    ) as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            27 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
    if 0xff1e00ff as std::os::raw::c_uint == val as std::os::raw::c_uint {} else {
        __assert_fail(
            b"0xff1e00ff == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            28 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"rgb(0,0,0)\0" as *const u8 as *const std::os::raw::c_char, Some(&mut ok))
        as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            31 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
    if 0xff as std::os::raw::c_int == val {} else {
        __assert_fail(
            b"0x000000ff == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            32 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_rgba() {
    let mut ok: i16 = 0;
    let mut val = rgba_from_string(
        b"rgba(255, 30   , 0, .5)\0" as *const u8 as *const std::os::raw::c_char,
        Some(&mut ok),
    ) as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            43 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
    if 0xff1e007f as std::os::raw::c_uint == val as std::os::raw::c_uint {} else {
        __assert_fail(
            b"0xff1e007f == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            44 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(
        b"rgba(0,0,0, 1)\0" as *const u8 as *const std::os::raw::c_char,
        Some(&mut ok),
    ) as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            47 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
    if 0xff as std::os::raw::c_int == val {} else {
        __assert_fail(
            b"0x000000ff == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            48 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_hex() {
    let mut ok: i16 = 0;
    let mut val = rgba_from_string(
        b"#ff1e00\0" as *const u8 as *const std::os::raw::c_char,
        Some(&mut ok),
    ) as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            59 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xff1e00ff as std::os::raw::c_uint == val as std::os::raw::c_uint {} else {
        __assert_fail(
            b"0xff1e00ff == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            60 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"#ffffff\0" as *const u8 as *const std::os::raw::c_char, Some(&mut ok))
        as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            63 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xffffffff as std::os::raw::c_uint == val as std::os::raw::c_uint {} else {
        __assert_fail(
            b"0xffffffff == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            64 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"#ffcc00\0" as *const u8 as *const std::os::raw::c_char, Some(&mut ok))
        as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            67 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xffcc00ff as std::os::raw::c_uint == val as std::os::raw::c_uint {} else {
        __assert_fail(
            b"0xffcc00ff == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            68 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"#fco\0" as *const u8 as *const std::os::raw::c_char, Some(&mut ok))
        as int32_t;
    if ok as std::os::raw::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            71 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xffcc00ff as std::os::raw::c_uint == val as std::os::raw::c_uint {} else {
        __assert_fail(
            b"0xffcc00ff == val\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            72 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 16], &'_ [i8; 16]>(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_to_string() {
    let mut buf: [i8; 256] = [0; 256];
    let mut color = rgba_new(0xffcc00ff as std::os::raw::c_uint);
    rgba_to_string(color, buf.as_mut_ptr(), 256 as std::os::raw::c_int as size_t);
    if 0 as std::os::raw::c_int
        == strcmp(b"#ffcc00\0" as *const u8 as *const std::os::raw::c_char, buf.as_mut_ptr())
    {} else {
        __assert_fail(
            b"0 == strcmp(\"#ffcc00\", buf)\0" as *const u8 as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            84 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 22], &'_ [i8; 22]>(b"void test_to_string()\0"))
                .as_ptr(),
        );
    };
    color = rgba_new(0xffcc0050 as std::os::raw::c_uint);
    rgba_to_string(color, buf.as_mut_ptr(), 256 as std::os::raw::c_int as size_t);
    if 0 as std::os::raw::c_int
        == strcmp(
            b"rgba(255, 204, 0, 0.31)\0" as *const u8 as *const std::os::raw::c_char,
            buf.as_mut_ptr(),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"rgba(255, 204, 0, 0.31)\", buf)\0" as *const u8
                as *const std::os::raw::c_char,
            b"test.c\0" as *const u8 as *const std::os::raw::c_char,
            88 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 22], &'_ [i8; 22]>(b"void test_to_string()\0"))
                .as_ptr(),
        );
    };
}
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: * mut * mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    test_named();
    test_rgb();
    test_rgba();
    test_hex();
    test_to_string();
    printf(
        b"\n  \x1B[32m\xE2\x9C\x93 \x1B[90mok\x1B[0m\n\n\0" as *const u8
            as *const std::os::raw::c_char,
    );
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
// pub fn main() {
//     let mut args: Vec::<*mut std::os::raw::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as std::os::raw::c_int,
//                 args.as_mut_ptr() as *mut *mut std::os::raw::c_char,
//             ) as i32,
//         )
//     }
// }
