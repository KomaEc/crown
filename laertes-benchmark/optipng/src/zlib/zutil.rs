
extern "C" {
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(__ptr: *mut std::os::raw::c_void);
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
}
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type voidpf = *mut std::os::raw::c_void;
pub type __off_t = std::os::raw::c_long;
pub type off_t = __off_t;
/* zutil.c -- target dependent utility functions for the compression library
 * Copyright (C) 1995-2017 Jean-loup Gailly
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
#[no_mangle]
pub static mut z_errmsg: [*mut std::os::raw::c_char; 10] =
    [b"need dictionary\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"stream end\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"file error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"stream error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"data error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"insufficient memory\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"buffer error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"incompatible version\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn zlibVersion() -> *const std::os::raw::c_char {
    return b"1.2.11-optipng\x00" as *const u8 as *const std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn zlibCompileFlags() -> uLong {
    let mut flags: uLong = 0;
    flags = 0 as std::os::raw::c_int as uLong;
    match ::std::mem::size_of::<uInt>() as std::os::raw::c_ulong as std::os::raw::c_int {
        2 => { }
        4 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        8 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        _ => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(3 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
    }
    match ::std::mem::size_of::<uLong>() as std::os::raw::c_ulong as std::os::raw::c_int {
        2 => { }
        4 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((1 as std::os::raw::c_int) <<
                                                      2 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        8 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((2 as std::os::raw::c_int) <<
                                                      2 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        _ => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((3 as std::os::raw::c_int) <<
                                                      2 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
    }
    match ::std::mem::size_of::<voidpf>() as std::os::raw::c_ulong as std::os::raw::c_int {
        2 => { }
        4 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((1 as std::os::raw::c_int) <<
                                                      4 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        8 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((2 as std::os::raw::c_int) <<
                                                      4 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        _ => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((3 as std::os::raw::c_int) <<
                                                      4 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
    }
    match ::std::mem::size_of::<off_t>() as std::os::raw::c_ulong as std::os::raw::c_int {
        2 => { }
        4 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((1 as std::os::raw::c_int) <<
                                                      6 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        8 => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((2 as std::os::raw::c_int) <<
                                                      6 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        _ => {
            flags =
                (flags as
                     std::os::raw::c_ulong).wrapping_add(((3 as std::os::raw::c_int) <<
                                                      6 as std::os::raw::c_int) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
    }
    flags =
        (flags as
             std::os::raw::c_ulong).wrapping_add(((1 as std::os::raw::c_long) <<
                                              16 as std::os::raw::c_int) as
                                             std::os::raw::c_ulong) as uLong as uLong;
    flags =
        (flags as
             std::os::raw::c_ulong).wrapping_add(((1 as std::os::raw::c_long) <<
                                              17 as std::os::raw::c_int) as
                                             std::os::raw::c_ulong) as uLong as uLong;
    return flags;
}
/* exported to allow conversion of error code to string for compress() and
 * uncompress()
 */
#[no_mangle]
pub unsafe extern "C" fn zError(mut err: std::os::raw::c_int) -> *const std::os::raw::c_char {
    return z_errmsg[(2 as std::os::raw::c_int - err) as usize];
}
/* SYS16BIT */
/* Any system without a special alloc function */
#[no_mangle]
pub unsafe extern "C" fn zcalloc(mut opaque: voidpf, mut items: std::os::raw::c_uint,
                                 mut size: std::os::raw::c_uint) -> voidpf {
    return if ::std::mem::size_of::<uInt>() as std::os::raw::c_ulong >
                  2 as std::os::raw::c_int as std::os::raw::c_ulong {
               malloc(items.wrapping_mul(size) as std::os::raw::c_ulong)
           } else { calloc(items as std::os::raw::c_ulong, size as std::os::raw::c_ulong) };
}
#[no_mangle]
pub unsafe extern "C" fn zcfree(mut opaque: voidpf, mut ptr: voidpf) {
    free(ptr);
}
/* !Z_SOLO */
/* MY_ZCALLOC */
