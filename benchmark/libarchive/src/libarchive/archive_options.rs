use ::libc;
extern "C" {
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
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
/*-
 * Copyright (c) 2003-2007 Tim Kientzle
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
 * $FreeBSD: head/lib/libarchive/archive_private.h 201098 2009-12-28 02:58:14Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive {
    pub magic: libc::c_uint,
    pub state: libc::c_uint,
    pub vtable: *mut archive_vtable,
    pub archive_format: libc::c_int,
    pub archive_format_name: *const libc::c_char,
    pub compression_code: libc::c_int,
    pub compression_name: *const libc::c_char,
    pub file_count: libc::c_int,
    pub archive_error_number: libc::c_int,
    pub error: *const libc::c_char,
    pub error_string: archive_string,
    pub current_code: *mut libc::c_char,
    pub current_codepage: libc::c_uint,
    pub current_oemcp: libc::c_uint,
    pub sconv: *mut archive_string_conv,
    pub read_data_block: *const libc::c_char,
    pub read_data_offset: int64_t,
    pub read_data_output_offset: int64_t,
    pub read_data_remaining: size_t,
    pub read_data_is_posix_read: libc::c_char,
    pub read_data_requested: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_vtable {
    pub archive_close: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_free: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_write_finish_entry: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_data:
        Option<unsafe extern "C" fn(_: *mut archive, _: *const libc::c_void, _: size_t) -> ssize_t>,
    pub archive_write_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *const libc::c_void,
            _: size_t,
            _: int64_t,
        ) -> ssize_t,
    >,
    pub archive_read_next_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int>,
    pub archive_read_next_header2:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_read_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub archive_filter_count: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_filter_bytes:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t>,
    pub archive_filter_code:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int>,
    pub archive_filter_name:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char>,
}
pub type option_handler = Option<
    unsafe extern "C" fn(
        _: *mut archive,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int,
>;
/*-
 * Copyright (c) 2003-2007 Tim Kientzle
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
 * $FreeBSD: head/lib/libarchive/archive_platform.h 201090 2009-12-28 02:22:04Z kientzle $
 */
/* !!ONLY FOR USE INTERNALLY TO LIBARCHIVE!! */
/*
 * This header is the first thing included in any of the libarchive
 * source files.  As far as possible, platform-specific issues should
 * be dealt with here and not within individual source files.  I'm
 * actively trying to minimize #if blocks within the main source,
 * since they obfuscate the code.
 */
/* archive.h and archive_entry.h require this. */
/* Most POSIX platforms use the 'configure' script to build config.h */
/* On macOS check for some symbols based on the deployment target version.  */
/* It should be possible to get rid of this by extending the feature-test
 * macros to cover Windows API functions, probably along with non-trivial
 * refactoring of code to find structures that sit more cleanly on top of
 * either Windows or Posix APIs. */
/*
 * The config files define a lot of feature macros.  The following
 * uses those macros to select/define replacements and include key
 * headers as required.
 */
/* Get a real definition for __FBSDID or __RCSID if we can */
/* If not, define them so as to avoid dangling semicolons. */
/* Try to get standard C99-style integer type definitions. */
/* Borland warns about its own constants!  */
/* Some platforms lack the standard *_MAX definitions. */
/*
 * If we can't restore metadata using a file descriptor, then
 * for compatibility's sake, close files before trying to restore metadata.
 */
/*
 * glibc 2.24 deprecates readdir_r
 */
/* Set up defaults for internal error codes. */
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn _archive_set_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
    mut magic: libc::c_int,
    mut fn_0: *const libc::c_char,
    mut use_option: option_handler,
) -> libc::c_int {
    let mut mp: *const libc::c_char = 0 as *const libc::c_char;
    let mut op: *const libc::c_char = 0 as *const libc::c_char;
    let mut vp: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int =
        __archive_check_magic(a, magic as libc::c_uint, 1 as libc::c_uint, fn_0);
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    mp = if !m.is_null() && *m.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
        m
    } else {
        NULL as *const libc::c_char
    };
    op = if !o.is_null() && *o.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
        o
    } else {
        NULL as *const libc::c_char
    };
    vp = if !v.is_null() && *v.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
        v
    } else {
        NULL as *const libc::c_char
    };
    if op.is_null() && vp.is_null() {
        return 0 as libc::c_int;
    }
    if op.is_null() {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Empty option\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = use_option.expect("non-null function pointer")(a, mp, op, vp);
    if r == ARCHIVE_WARN - 1 as libc::c_int {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Unknown module name: `%s\'\x00" as *const u8 as *const libc::c_char,
            mp,
        );
        return -(25 as libc::c_int);
    }
    if r == ARCHIVE_WARN {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Undefined option: `%s%s%s%s%s%s\'\x00" as *const u8 as *const libc::c_char,
            if !vp.is_null() {
                b"\x00" as *const u8 as *const libc::c_char
            } else {
                b"!\x00" as *const u8 as *const libc::c_char
            },
            if !mp.is_null() {
                mp
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !mp.is_null() {
                b":\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            op,
            if !vp.is_null() {
                b"=\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if !vp.is_null() {
                vp
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        );
        return -(25 as libc::c_int);
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_set_either_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
    mut use_format_option: option_handler,
    mut use_filter_option: option_handler,
) -> libc::c_int {
    let mut r1: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    if o.is_null() && v.is_null() {
        return 0 as libc::c_int;
    }
    if o.is_null() {
        return -(25 as libc::c_int);
    }
    r1 = use_format_option.expect("non-null function pointer")(a, m, o, v);
    if r1 == ARCHIVE_FATAL {
        return -(30 as libc::c_int);
    }
    r2 = use_filter_option.expect("non-null function pointer")(a, m, o, v);
    if r2 == ARCHIVE_FATAL {
        return -(30 as libc::c_int);
    }
    if r2 == ARCHIVE_WARN - 1 as libc::c_int {
        return r1;
    }
    return if r1 > r2 { r1 } else { r2 };
}
/*-
 * Copyright (c) 2011 Tim Kientzle
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
 */
#[no_mangle]
pub unsafe extern "C" fn _archive_set_options(
    mut a: *mut archive,
    mut options: *const libc::c_char,
    mut magic: libc::c_int,
    mut fn_0: *const libc::c_char,
    mut use_option: option_handler,
) -> libc::c_int {
    let mut allok: libc::c_int = 1 as libc::c_int;
    let mut anyok: libc::c_int = 0 as libc::c_int;
    let mut ignore_mod_err: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut mod_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut magic_test: libc::c_int =
        __archive_check_magic(a, magic as libc::c_uint, 1 as libc::c_uint, fn_0);
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if options.is_null()
        || *options.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        return ARCHIVE_OK;
    }
    data = strdup(options);
    if data.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"Out of memory adding file to list\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    s = data as *const libc::c_char;
    loop {
        val = NULL as *const libc::c_char;
        opt = val;
        mod_0 = opt;
        parse_option(&mut s, &mut mod_0, &mut opt, &mut val);
        if mod_0.is_null()
            && !opt.is_null()
            && strcmp(
                b"__ignore_wrong_module_name__\x00" as *const u8 as *const libc::c_char,
                opt,
            ) == 0 as libc::c_int
        {
            /* Ignore module name error */
            if !val.is_null() {
                ignore_mod_err = 1 as libc::c_int;
                anyok = 1 as libc::c_int
            }
        } else {
            r = use_option.expect("non-null function pointer")(a, mod_0, opt, val);
            if r == ARCHIVE_FATAL {
                free(data as *mut libc::c_void);
                return -(30 as libc::c_int);
            }
            if r == ARCHIVE_FAILED && !mod_0.is_null() {
                free(data as *mut libc::c_void);
                return -(25 as libc::c_int);
            }
            if r == ARCHIVE_WARN - 1 as libc::c_int {
                if !(ignore_mod_err != 0) {
                    /* The module name is wrong. */
                    archive_set_error(
                        a,
                        ARCHIVE_ERRNO_MISC,
                        b"Unknown module name: `%s\'\x00" as *const u8 as *const libc::c_char,
                        mod_0,
                    );
                    free(data as *mut libc::c_void);
                    return -(25 as libc::c_int);
                }
            } else {
                if r == ARCHIVE_WARN {
                    /* The option name is wrong. No-one used this. */
                    archive_set_error(
                        a,
                        ARCHIVE_ERRNO_MISC,
                        b"Undefined option: `%s%s%s\'\x00" as *const u8 as *const libc::c_char,
                        if !mod_0.is_null() {
                            mod_0
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                        if !mod_0.is_null() {
                            b":\x00" as *const u8 as *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        },
                        opt,
                    );
                    free(data as *mut libc::c_void);
                    return -(25 as libc::c_int);
                }
                if r == ARCHIVE_OK {
                    anyok = 1 as libc::c_int
                } else {
                    allok = 0 as libc::c_int
                }
            }
        }
        if s.is_null() {
            break;
        }
    }
    free(data as *mut libc::c_void);
    return if allok != 0 {
        ARCHIVE_OK
    } else if anyok != 0 {
        ARCHIVE_WARN
    } else {
        ARCHIVE_FAILED
    };
}
unsafe extern "C" fn parse_option(
    mut s: *mut *const libc::c_char,
    mut m: *mut *const libc::c_char,
    mut o: *mut *const libc::c_char,
    mut v: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut mod_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: *const libc::c_char = 0 as *const libc::c_char;
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    end = NULL as *const libc::c_char;
    mod_0 = NULL as *const libc::c_char;
    opt = *s;
    val = b"1\x00" as *const u8 as *const libc::c_char;
    p = strchr(opt, ',' as i32);
    if !p.is_null() {
        *p = '\u{0}' as i32 as libc::c_char;
        end = (p as *const libc::c_char).offset(1 as libc::c_int as isize)
    }
    if 0 as libc::c_int as libc::c_ulong == strlen(opt) {
        *s = end;
        *m = NULL as *const libc::c_char;
        *o = NULL as *const libc::c_char;
        *v = NULL as *const libc::c_char;
        return end;
    }
    p = strchr(opt, ':' as i32);
    if !p.is_null() {
        *p = '\u{0}' as i32 as libc::c_char;
        mod_0 = opt;
        p = p.offset(1);
        opt = p
    }
    p = strchr(opt, '=' as i32);
    if !p.is_null() {
        *p = '\u{0}' as i32 as libc::c_char;
        p = p.offset(1);
        val = p
    } else if *opt.offset(0 as libc::c_int as isize) as libc::c_int == '!' as i32 {
        opt = opt.offset(1);
        val = NULL as *const libc::c_char
    }
    *s = end;
    *m = mod_0;
    *o = opt;
    *v = val;
    return end;
}
