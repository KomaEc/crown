use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rgba_to_string(rgba: rgba_t, buf: *mut libc::c_char, len: size_t);
    fn rgba_from_string(str: *const libc::c_char, ok: *mut libc::c_short) -> uint32_t;
    fn rgba_new(rgba: uint32_t) -> rgba_t;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rgba_t {
    pub r: libc::c_double,
    pub g: libc::c_double,
    pub b: libc::c_double,
    pub a: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn test_named() {
    let mut ok: libc::c_short = 0;
    let mut val = rgba_from_string(
        b"olive\0" as *const u8 as *const libc::c_char,
        &mut ok,
    ) as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"void test_named()\0"))
                .as_ptr(),
        );
    };
    if 0x808000ff as libc::c_uint == val as libc::c_uint {} else {
        __assert_fail(
            b"0x808000FF == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            16 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"void test_named()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_rgb() {
    let mut ok: libc::c_short = 0;
    let mut val = rgba_from_string(
        b"rgb(255, 30   , 0)\0" as *const u8 as *const libc::c_char,
        &mut ok,
    ) as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
    if 0xff1e00ff as libc::c_uint == val as libc::c_uint {} else {
        __assert_fail(
            b"0xff1e00ff == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"rgb(0,0,0)\0" as *const u8 as *const libc::c_char, &mut ok)
        as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
    if 0xff as libc::c_int == val {} else {
        __assert_fail(
            b"0x000000ff == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_rgb()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_rgba() {
    let mut ok: libc::c_short = 0;
    let mut val = rgba_from_string(
        b"rgba(255, 30   , 0, .5)\0" as *const u8 as *const libc::c_char,
        &mut ok,
    ) as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
    if 0xff1e007f as libc::c_uint == val as libc::c_uint {} else {
        __assert_fail(
            b"0xff1e007f == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(
        b"rgba(0,0,0, 1)\0" as *const u8 as *const libc::c_char,
        &mut ok,
    ) as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
    if 0xff as libc::c_int == val {} else {
        __assert_fail(
            b"0x000000ff == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"void test_rgba()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_hex() {
    let mut ok: libc::c_short = 0;
    let mut val = rgba_from_string(
        b"#ff1e00\0" as *const u8 as *const libc::c_char,
        &mut ok,
    ) as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xff1e00ff as libc::c_uint == val as libc::c_uint {} else {
        __assert_fail(
            b"0xff1e00ff == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"#ffffff\0" as *const u8 as *const libc::c_char, &mut ok)
        as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xffffffff as libc::c_uint == val as libc::c_uint {} else {
        __assert_fail(
            b"0xffffffff == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"#ffcc00\0" as *const u8 as *const libc::c_char, &mut ok)
        as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xffcc00ff as libc::c_uint == val as libc::c_uint {} else {
        __assert_fail(
            b"0xffcc00ff == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    val = rgba_from_string(b"#fco\0" as *const u8 as *const libc::c_char, &mut ok)
        as int32_t;
    if ok as libc::c_int != 0 {} else {
        __assert_fail(
            b"ok\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
    if 0xffcc00ff as libc::c_uint == val as libc::c_uint {} else {
        __assert_fail(
            b"0xffcc00ff == val\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"void test_hex()\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_to_string() {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut color = rgba_new(0xffcc00ff as libc::c_uint);
    rgba_to_string(color, buf.as_mut_ptr(), 256 as libc::c_int as size_t);
    if 0 as libc::c_int
        == strcmp(b"#ffcc00\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr())
    {} else {
        __assert_fail(
            b"0 == strcmp(\"#ffcc00\", buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_to_string()\0"))
                .as_ptr(),
        );
    };
    color = rgba_new(0xffcc0050 as libc::c_uint);
    rgba_to_string(color, buf.as_mut_ptr(), 256 as libc::c_int as size_t);
    if 0 as libc::c_int
        == strcmp(
            b"rgba(255, 204, 0, 0.31)\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        )
    {} else {
        __assert_fail(
            b"0 == strcmp(\"rgba(255, 204, 0, 0.31)\", buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void test_to_string()\0"))
                .as_ptr(),
        );
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    test_named();
    test_rgb();
    test_rgba();
    test_hex();
    test_to_string();
    printf(
        b"\n  \x1B[32m\xE2\x9C\x93 \x1B[90mok\x1B[0m\n\n\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
