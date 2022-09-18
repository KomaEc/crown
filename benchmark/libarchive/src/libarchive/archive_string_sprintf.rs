use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_append_from_wcs(
        _: *mut archive_string,
        _: *const wchar_t,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    /*
     * The magic/state values are used to sanity-check the
     * client's usage.  If an API function is called at a
     * ridiculous time, or the client passes us an invalid
     * pointer, these values allow me to catch that.
     */
    /*
     * Some public API functions depend on the "real" type of the
     * archive object.
     */
    /* Currently active compression. */
    /* Number of file entries processed. */
    /* Current ACP(ANSI CodePage). */
    /* Current OEMCP(OEM CodePage). */
    /*
     * Used by archive_read_data() to track blocks and copy
     * data to client buffers, filling gaps with zero bytes.
     */
    /*
     * Used by formats/filters to determine the amount of data
     * requested from a call to archive_read_data(). This is only
     * useful when the format/filter has seek support.
     */
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_errx(retvalue: libc::c_int, msg: *const libc::c_char) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
pub const INTMAX_MIN: libc::c_long =
    -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * The use of printf()-family functions can be troublesome
 * for space-constrained applications.  In addition, correctly
 * implementing this function in terms of vsnprintf() requires
 * two calls (one to determine the size, another to format the
 * result), which in turn requires duplicating the argument list
 * using va_copy, which isn't yet universally available. <sigh>
 *
 * So, I've implemented a bare minimum of printf()-like capability
 * here.  This is only used to format error messages, so doesn't
 * require any floating-point support or field-width handling.
 */
/*
 * Utility functions to format signed/unsigned integers and append
 * them to an archive_string.
 */
unsafe extern "C" fn append_uint(
    mut as_0: *mut archive_string,
    mut d: uintmax_t,
    mut base: libc::c_uint,
) {
    static mut digits: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\x00")
    };
    if d >= base as libc::c_ulong {
        append_uint(as_0, d.wrapping_div(base as libc::c_ulong), base);
    }
    archive_strappend_char(as_0, digits[d.wrapping_rem(base as libc::c_ulong) as usize]);
}
unsafe extern "C" fn append_int(
    mut as_0: *mut archive_string,
    mut d: intmax_t,
    mut base: libc::c_uint,
) {
    let mut ud: uintmax_t = 0;
    if d < 0 as libc::c_int as libc::c_long {
        archive_strappend_char(as_0, '-' as i32 as libc::c_char);
        ud = if d == INTMAX_MIN {
            (9223372036854775807 as libc::c_long as uintmax_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            -d as uintmax_t
        }
    } else {
        ud = d as uintmax_t
    }
    append_uint(as_0, ud, base);
}
#[no_mangle]
pub unsafe extern "C" fn archive_string_sprintf(
    mut as_0: *mut archive_string,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    archive_string_vsprintf(as_0, fmt, ap.as_va_list());
}
/*-
 * Copyright (c) 2003-2010 Tim Kientzle
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * $FreeBSD: head/lib/libarchive/archive_string.h 201092 2009-12-28 02:26:06Z kientzle $
 *
 */
/* required for wchar_t on some systems */
/*
 * Basic resizable/reusable string support similar to Java's "StringBuffer."
 *
 * Unlike sbuf(9), the buffers here are fully reusable and track the
 * length throughout.
 */
/* Pointer to the storage */
/* Length of 's' in characters */
/* Length of malloc-ed storage in bytes. */
/* Pointer to the storage */
/* Length of 's' in characters */
/* Length of malloc-ed storage in bytes. */
/* Initialize an archive_string object on the stack or elsewhere. */
/* Append a C char to an archive_string, resizing as necessary. */
/* Ditto for a wchar_t and an archive_wstring. */
/* Append a raw array to an archive_string, resizing as necessary */
/* Convert a Unicode string to current locale and append the result. */
/* Returns -1 if conversion fails. */
/* Create a string conversion object.
 * Return NULL and set a error message if the conversion is not supported
 * on the platform. */
/* Create the default string conversion object for reading/writing an archive.
 * Return NULL if the conversion is unneeded.
 * Note: On non Windows platform this always returns NULL.
 */
/* Dispose of a string conversion object. */
/* Copy one archive_string to another in locale conversion.
 * Return -1 if conversion fails. */
/* Copy one archive_string to another in locale conversion.
 * Return -1 if conversion fails. */
/* Copy one archive_string to another */
/* Concatenate one archive_string to another */
/* Ensure that the underlying buffer is at least as large as the request. */
/* Append C string, which may lack trailing \0. */
/* The source is declared void * here because this gets used with
 * "signed char *", "unsigned char *" and "char *" arguments.
 * Declaring it "char *" as with some of the other functions just
 * leads to a lot of extra casts. */
/* Append a C string to an archive_string, resizing as necessary. */
/* Copy a C string to an archive_string, resizing as necessary. */
/* Copy a C string to an archive_string with limit, resizing as necessary. */
/* Return length of string. */
/* Set string length to zero. */
/* Release any allocated storage resources. */
/* Like 'vsprintf', but resizes the underlying string as necessary. */
/* Note: This only implements a small subset of standard printf functionality. */
/*
 * Like 'vsprintf', but ensures the target is big enough, resizing if
 * necessary.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_string_vsprintf(
    mut as_0: *mut archive_string,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut long_flag: libc::c_char = 0; /* Signed integer temp. */
    let mut s: intmax_t = 0; /* Unsigned integer temp. */
    let mut u: uintmax_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    let mut pw: *const wchar_t = 0 as *const wchar_t;
    if archive_string_ensure(as_0, 64 as libc::c_int as size_t).is_null() {
        __archive_errx(
            1 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    if fmt.is_null() {
        *(*as_0).s.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        return;
    }
    p = fmt;
    while *p as libc::c_int != '\u{0}' as i32 {
        let mut saved_p: *const libc::c_char = p;
        if *p as libc::c_int != '%' as i32 {
            archive_strappend_char(as_0, *p);
        } else {
            p = p.offset(1);
            long_flag = '\u{0}' as i32 as libc::c_char;
            match *p as libc::c_int {
                106 | 108 | 122 => {
                    long_flag = *p;
                    p = p.offset(1)
                }
                _ => {}
            }
            match *p as libc::c_int {
                37 => {
                    archive_strappend_char(as_0, '%' as i32 as libc::c_char);
                }
                99 => {
                    s = ap.arg::<libc::c_int>() as intmax_t;
                    archive_strappend_char(as_0, s as libc::c_char);
                }
                100 => {
                    match long_flag as libc::c_int {
                        106 => s = ap.arg::<intmax_t>(),
                        108 => s = ap.arg::<libc::c_long>(),
                        122 => s = ap.arg::<ssize_t>(),
                        _ => s = ap.arg::<libc::c_int>() as intmax_t,
                    }
                    append_int(as_0, s, 10 as libc::c_int as libc::c_uint);
                }
                115 => match long_flag as libc::c_int {
                    108 => {
                        pw = ap.arg::<*mut wchar_t>();
                        if pw.is_null() {
                            pw =
                                    (*::std::mem::transmute::<&[u8; 28],
                                                              &[libc::c_int; 7]>(b"(\x00\x00\x00n\x00\x00\x00u\x00\x00\x00l\x00\x00\x00l\x00\x00\x00)\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                        }
                        if archive_string_append_from_wcs(as_0, pw, wcslen(pw)) != 0 as libc::c_int
                            && errno == ENOMEM
                        {
                            __archive_errx(
                                1 as libc::c_int,
                                b"Out of memory\x00" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    _ => {
                        p2 = ap.arg::<*mut libc::c_char>();
                        if p2.is_null() {
                            p2 = b"(null)\x00" as *const u8 as *const libc::c_char
                        }
                        archive_strcat(as_0, p2 as *const libc::c_void);
                    }
                },
                83 => {
                    pw = ap.arg::<*mut wchar_t>();
                    if pw.is_null() {
                        pw =
                            (*::std::mem::transmute::<&[u8; 28],
                                                      &[libc::c_int; 7]>(b"(\x00\x00\x00n\x00\x00\x00u\x00\x00\x00l\x00\x00\x00l\x00\x00\x00)\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                    }
                    if archive_string_append_from_wcs(as_0, pw, wcslen(pw)) != 0 as libc::c_int
                        && errno == ENOMEM
                    {
                        __archive_errx(
                            1 as libc::c_int,
                            b"Out of memory\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                111 | 117 | 120 | 88 => {
                    /* Common handling for unsigned integer formats. */
                    match long_flag as libc::c_int {
                        106 => u = ap.arg::<uintmax_t>(),
                        108 => u = ap.arg::<libc::c_ulong>(),
                        122 => u = ap.arg::<size_t>(),
                        _ => u = ap.arg::<libc::c_uint>() as uintmax_t,
                    }
                    /* Format it in the correct base. */
                    match *p as libc::c_int {
                        111 => {
                            append_uint(as_0, u, 8 as libc::c_int as libc::c_uint);
                        }
                        117 => {
                            append_uint(as_0, u, 10 as libc::c_int as libc::c_uint);
                        }
                        _ => {
                            append_uint(as_0, u, 16 as libc::c_int as libc::c_uint);
                        }
                    }
                }
                _ => {
                    /* Rewind and print the initial '%' literally. */
                    p = saved_p;
                    archive_strappend_char(as_0, *p);
                }
            }
        }
        p = p.offset(1)
    }
}
