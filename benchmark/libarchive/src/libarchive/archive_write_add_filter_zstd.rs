use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    pub type archive_write_program_data;
    #[no_mangle]
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_string_concat(dest: *mut archive_string, src: *mut archive_string);
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn __archive_write_allocate_filter(_: *mut archive) -> *mut archive_write_filter;
    #[no_mangle]
    fn __archive_write_program_allocate(
        program_name: *const libc::c_char,
    ) -> *mut archive_write_program_data;
    #[no_mangle]
    fn __archive_write_program_free(_: *mut archive_write_program_data) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_program_open(
        _: *mut archive_write_filter,
        _: *mut archive_write_program_data,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_program_close(
        _: *mut archive_write_filter,
        _: *mut archive_write_program_data,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_program_write(
        _: *mut archive_write_filter,
        _: *mut archive_write_program_data,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type la_ssize_t = ssize_t;
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
pub type archive_write_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *const libc::c_void,
    _: size_t,
) -> la_ssize_t;
pub type archive_open_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write {
    pub archive: archive,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub nulls: *const libc::c_uchar,
    pub null_length: size_t,
    pub client_opener: Option<archive_open_callback>,
    pub client_writer: Option<archive_write_callback>,
    pub client_closer: Option<archive_close_callback>,
    pub client_data: *mut libc::c_void,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub filter_first: *mut archive_write_filter,
    pub filter_last: *mut archive_write_filter,
    pub format_data: *mut libc::c_void,
    pub format_name: *const libc::c_char,
    pub format_init: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub format_finish_entry: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_write_header:
        Option<unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int>,
    pub format_write_data: Option<
        unsafe extern "C" fn(_: *mut archive_write, _: *const libc::c_void, _: size_t) -> ssize_t,
    >,
    pub format_close: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_free: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub passphrase: *mut libc::c_char,
    pub passphrase_callback: Option<archive_passphrase_callback>,
    pub passphrase_client_data: *mut libc::c_void,
}
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
 * $FreeBSD: head/lib/libarchive/archive_write_private.h 201155 2009-12-29 05:20:12Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write_filter {
    pub bytes_written: int64_t,
    pub archive: *mut archive,
    pub next_filter: *mut archive_write_filter,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub write: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub state: libc::c_int,
}
/* Don't compile this if we don't have zstd.h */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct private_data {
    pub compression_level: libc::c_int,
    pub pdata: *mut archive_write_program_data,
}
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FILTER_ZSTD: libc::c_int = 14 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/* If we don't have the library use default range values (zstdcli.c v1.4.0) */
pub const CLEVEL_MIN: libc::c_int = -(99 as libc::c_int);
pub const CLEVEL_STD_MIN: libc::c_int = 0 as libc::c_int;
/* prior to 1.3.4 and more recent without using --fast */
pub const CLEVEL_DEFAULT: libc::c_int = 3 as libc::c_int;
pub const CLEVEL_STD_MAX: libc::c_int = 19 as libc::c_int;
/* without using --ultra */
pub const CLEVEL_MAX: libc::c_int = 22 as libc::c_int;
/*
 * Add a zstd compression filter to this write handle.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_add_filter_zstd(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut f: *mut archive_write_filter = __archive_write_allocate_filter(_a);
    let mut data: *mut private_data = 0 as *mut private_data;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_add_filter_zstd\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<private_data>() as libc::c_ulong,
    ) as *mut private_data;
    if data.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*f).data = data as *mut libc::c_void;
    (*f).open = Some(
        archive_compressor_zstd_open
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).options = Some(
        archive_compressor_zstd_options
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*f).close = Some(
        archive_compressor_zstd_close
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).free = Some(
        archive_compressor_zstd_free
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).code = ARCHIVE_FILTER_ZSTD;
    (*f).name = b"zstd\x00" as *const u8 as *const libc::c_char;
    (*data).compression_level = CLEVEL_DEFAULT;
    (*data).pdata =
        __archive_write_program_allocate(b"zstd\x00" as *const u8 as *const libc::c_char);
    if (*data).pdata.is_null() {
        free(data as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_MISC,
        b"Using external zstd program\x00" as *const u8 as *const libc::c_char,
    );
    return -(20 as libc::c_int);
}
unsafe extern "C" fn archive_compressor_zstd_free(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    __archive_write_program_free((*data).pdata);
    free(data as *mut libc::c_void);
    (*f).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn string_is_numeric(mut value: *const libc::c_char) -> libc::c_int {
    let mut len: size_t = strlen(value);
    let mut i: size_t = 0;
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(20 as libc::c_int);
    } else {
        if len == 1 as libc::c_int as libc::c_ulong
            && !(*value.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *value.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
        {
            return -(20 as libc::c_int);
        } else {
            if !(*value.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *value.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
                && *value.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                && *value.offset(0 as libc::c_int as isize) as libc::c_int != '+' as i32
            {
                return -(20 as libc::c_int);
            }
        }
    }
    i = 1 as libc::c_int as size_t;
    while i < len {
        if !(*value.offset(i as isize) as libc::c_int >= '0' as i32
            && *value.offset(i as isize) as libc::c_int <= '9' as i32)
        {
            return -(20 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
/*
 * Set write options.
 */
unsafe extern "C" fn archive_compressor_zstd_options(
    mut f: *mut archive_write_filter,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    if strcmp(
        key,
        b"compression-level\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut level: libc::c_int = atoi(value);
        /* If we don't have the library, hard-code the max level */
        let mut minimum: libc::c_int = CLEVEL_MIN;
        let mut maximum: libc::c_int = CLEVEL_MAX;
        if string_is_numeric(value) != ARCHIVE_OK {
            return -(20 as libc::c_int);
        }
        if level < minimum || level > maximum {
            return -(20 as libc::c_int);
        }
        (*data).compression_level = level;
        return 0 as libc::c_int;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
/* HAVE_ZSTD_H && HAVE_LIBZSTD */
unsafe extern "C" fn archive_compressor_zstd_open(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    let mut as_0: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut r: libc::c_int = 0;
    as_0.s = NULL as *mut libc::c_char;
    as_0.length = 0 as libc::c_int as size_t;
    as_0.buffer_length = 0 as libc::c_int as size_t;
    /* --no-check matches library default */
    as_0.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut as_0,
        b"zstd --no-check\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        (if (b"zstd --no-check\x00" as *const u8 as *const libc::c_char).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(b"zstd --no-check\x00" as *const u8 as *const libc::c_char)
        }),
    );
    if (*data).compression_level < CLEVEL_STD_MIN {
        let mut as2: archive_string = archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        };
        as2.s = NULL as *mut libc::c_char;
        as2.length = 0 as libc::c_int as size_t;
        as2.buffer_length = 0 as libc::c_int as size_t;
        archive_string_sprintf(
            &mut as2 as *mut archive_string,
            b" --fast=%d\x00" as *const u8 as *const libc::c_char,
            -(*data).compression_level,
        );
        archive_string_concat(&mut as_0, &mut as2);
        archive_string_free(&mut as2);
    } else {
        let mut as2_0: archive_string = archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        };
        as2_0.s = NULL as *mut libc::c_char;
        as2_0.length = 0 as libc::c_int as size_t;
        as2_0.buffer_length = 0 as libc::c_int as size_t;
        archive_string_sprintf(
            &mut as2_0 as *mut archive_string,
            b" -%d\x00" as *const u8 as *const libc::c_char,
            (*data).compression_level,
        );
        archive_string_concat(&mut as_0, &mut as2_0);
        archive_string_free(&mut as2_0);
    }
    if (*data).compression_level > CLEVEL_STD_MAX {
        archive_strcat(
            &mut as_0,
            b" --ultra\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
    }
    (*f).write = Some(
        archive_compressor_zstd_write
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_int,
    );
    r = __archive_write_program_open(f, (*data).pdata, as_0.s);
    archive_string_free(&mut as_0);
    return r;
}
unsafe extern "C" fn archive_compressor_zstd_write(
    mut f: *mut archive_write_filter,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    return __archive_write_program_write(f, (*data).pdata, buff, length);
}
unsafe extern "C" fn archive_compressor_zstd_close(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    return __archive_write_program_close(f, (*data).pdata);
}
/* HAVE_ZSTD_H && HAVE_LIBZSTD */
