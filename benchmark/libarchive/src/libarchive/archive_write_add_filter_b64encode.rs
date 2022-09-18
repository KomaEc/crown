use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_write_get_bytes_per_block(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_bytes_in_last_block(
        _: *mut archive,
        bytes_in_last_block: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
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
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    /* Append C string, which may lack trailing \0. */
    /* The source is declared void * here because this gets used with
     * "signed char *", "unsigned char *" and "char *" arguments.
     * Declaring it "char *" as with some of the other functions just
     * leads to a lot of extra casts. */
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn __archive_write_allocate_filter(_: *mut archive) -> *mut archive_write_filter;
    #[no_mangle]
    fn __archive_write_filter(
        _: *mut archive_write_filter,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct private_b64encode {
    pub mode: libc::c_int,
    pub name: archive_string,
    pub encoded_buff: archive_string,
    pub bs: size_t,
    pub hold_len: size_t,
    pub hold: [libc::c_uchar; 57],
}
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
pub const ARCHIVE_FILTER_UU: libc::c_int = 7 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_WRITE_MAGIC: libc::c_uint = 0xb0c5c0de as libc::c_uint;
pub const LBYTES: libc::c_int = 57 as libc::c_int;
static mut base64: [libc::c_char; 64] = [
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
];
/*
 * Add a compress filter to this write handle.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_add_filter_b64encode(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut f: *mut archive_write_filter = __archive_write_allocate_filter(_a);
    let mut state: *mut private_b64encode = 0 as *mut private_b64encode;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_add_filter_uu\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<private_b64encode>() as libc::c_ulong,
    ) as *mut private_b64encode;
    if state.is_null() {
        archive_set_error(
            (*f).archive,
            ENOMEM,
            b"Can\'t allocate data for b64encode filter\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*state).name.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*state).name,
        b"-\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        (if (b"-\x00" as *const u8 as *const libc::c_char).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(b"-\x00" as *const u8 as *const libc::c_char)
        }),
    );
    (*state).mode = 0o644 as libc::c_int;
    (*f).data = state as *mut libc::c_void;
    (*f).name = b"b64encode\x00" as *const u8 as *const libc::c_char;
    (*f).code = ARCHIVE_FILTER_UU;
    (*f).open = Some(
        archive_filter_b64encode_open
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).options = Some(
        archive_filter_b64encode_options
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*f).write = Some(
        archive_filter_b64encode_write
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_int,
    );
    (*f).close = Some(
        archive_filter_b64encode_close
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).free = Some(
        archive_filter_b64encode_free
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
/*
 * Set write options.
 */
unsafe extern "C" fn archive_filter_b64encode_options(
    mut f: *mut archive_write_filter,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut state: *mut private_b64encode = (*f).data as *mut private_b64encode;
    if strcmp(key, b"mode\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if value.is_null() {
            archive_set_error(
                (*f).archive,
                ARCHIVE_ERRNO_MISC,
                b"mode option requires octal digits\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        (*state).mode = atol8(value, strlen(value)) as libc::c_int & 0o777 as libc::c_int;
        return 0 as libc::c_int;
    } else {
        if strcmp(key, b"name\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            if value.is_null() {
                archive_set_error(
                    (*f).archive,
                    ARCHIVE_ERRNO_MISC,
                    b"name option requires a string\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            (*state).name.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut (*state).name,
                value as *const libc::c_void,
                (if value.is_null() {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    strlen(value)
                }),
            );
            return 0 as libc::c_int;
        }
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
/*
 * Setup callback.
 */
unsafe extern "C" fn archive_filter_b64encode_open(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut state: *mut private_b64encode = (*f).data as *mut private_b64encode;
    let mut bs: size_t = 65536 as libc::c_int as size_t;
    let mut bpb: size_t = 0;
    if (*(*f).archive).magic == ARCHIVE_WRITE_MAGIC {
        /* Buffer size should be a multiple number of the of bytes
         * per block for performance. */
        bpb = archive_write_get_bytes_per_block((*f).archive) as size_t;
        if bpb > bs {
            bs = bpb
        } else if bpb != 0 as libc::c_int as libc::c_ulong {
            bs = (bs as libc::c_ulong).wrapping_sub(bs.wrapping_rem(bpb)) as size_t as size_t
        }
    }
    (*state).bs = bs;
    if archive_string_ensure(
        &mut (*state).encoded_buff,
        bs.wrapping_add(512 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        archive_set_error(
            (*f).archive,
            ENOMEM,
            b"Can\'t allocate data for b64encode buffer\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    archive_string_sprintf(
        &mut (*state).encoded_buff as *mut archive_string,
        b"begin-base64 %o %s\n\x00" as *const u8 as *const libc::c_char,
        (*state).mode,
        (*state).name.s,
    );
    (*f).data = state as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn la_b64_encode(
    mut as_0: *mut archive_string,
    mut p: *const libc::c_uchar,
    mut len: size_t,
) {
    let mut c: libc::c_int = 0;
    while len >= 3 as libc::c_int as libc::c_ulong {
        c = *p.offset(0 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int;
        archive_strappend_char(as_0, base64[c as usize]);
        c = (*p.offset(0 as libc::c_int as isize) as libc::c_int & 0x3 as libc::c_int)
            << 4 as libc::c_int
            | (*p.offset(1 as libc::c_int as isize) as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int;
        archive_strappend_char(as_0, base64[c as usize]);
        c = (*p.offset(1 as libc::c_int as isize) as libc::c_int & 0xf as libc::c_int)
            << 2 as libc::c_int
            | (*p.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int)
                >> 6 as libc::c_int;
        archive_strappend_char(as_0, base64[c as usize]);
        c = *p.offset(2 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int;
        archive_strappend_char(as_0, base64[c as usize]);
        p = p.offset(3 as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        c = *p.offset(0 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int;
        archive_strappend_char(as_0, base64[c as usize]);
        c = (*p.offset(0 as libc::c_int as isize) as libc::c_int & 0x3 as libc::c_int)
            << 4 as libc::c_int;
        if len == 1 as libc::c_int as libc::c_ulong {
            archive_strappend_char(as_0, base64[c as usize]);
            archive_strappend_char(as_0, '=' as i32 as libc::c_char);
            archive_strappend_char(as_0, '=' as i32 as libc::c_char);
        } else {
            c |= (*p.offset(1 as libc::c_int as isize) as libc::c_int & 0xf0 as libc::c_int)
                >> 4 as libc::c_int;
            archive_strappend_char(as_0, base64[c as usize]);
            c = (*p.offset(1 as libc::c_int as isize) as libc::c_int & 0xf as libc::c_int)
                << 2 as libc::c_int;
            archive_strappend_char(as_0, base64[c as usize]);
            archive_strappend_char(as_0, '=' as i32 as libc::c_char);
        }
    }
    archive_strappend_char(as_0, '\n' as i32 as libc::c_char);
}
/*
 * Write data to the encoded stream.
 */
unsafe extern "C" fn archive_filter_b64encode_write(
    mut f: *mut archive_write_filter,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut state: *mut private_b64encode = (*f).data as *mut private_b64encode;
    let mut p: *const libc::c_uchar = buff as *const libc::c_uchar;
    let mut ret: libc::c_int = ARCHIVE_OK;
    if length == 0 as libc::c_int as libc::c_ulong {
        return ret;
    }
    if (*state).hold_len != 0 {
        while (*state).hold_len < LBYTES as libc::c_ulong
            && length > 0 as libc::c_int as libc::c_ulong
        {
            let fresh0 = p;
            p = p.offset(1);
            let fresh1 = (*state).hold_len;
            (*state).hold_len = (*state).hold_len.wrapping_add(1);
            (*state).hold[fresh1 as usize] = *fresh0;
            length = length.wrapping_sub(1)
        }
        if (*state).hold_len < LBYTES as libc::c_ulong {
            return ret;
        }
        la_b64_encode(
            &mut (*state).encoded_buff,
            (*state).hold.as_mut_ptr(),
            LBYTES as size_t,
        );
        (*state).hold_len = 0 as libc::c_int as size_t
    }
    while length >= LBYTES as libc::c_ulong {
        la_b64_encode(&mut (*state).encoded_buff, p, LBYTES as size_t);
        length =
            (length as libc::c_ulong).wrapping_sub(LBYTES as libc::c_ulong) as size_t as size_t;
        p = p.offset(LBYTES as isize)
    }
    /* Save remaining bytes. */
    if length > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            (*state).hold.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            length,
        );
        (*state).hold_len = length
    }
    while (*state).encoded_buff.length >= (*state).bs {
        ret = __archive_write_filter(
            (*f).next_filter,
            (*state).encoded_buff.s as *const libc::c_void,
            (*state).bs,
        );
        memmove(
            (*state).encoded_buff.s as *mut libc::c_void,
            (*state).encoded_buff.s.offset((*state).bs as isize) as *const libc::c_void,
            (*state).encoded_buff.length.wrapping_sub((*state).bs),
        );
        (*state).encoded_buff.length = ((*state).encoded_buff.length as libc::c_ulong)
            .wrapping_sub((*state).bs) as size_t as size_t
    }
    return ret;
}
/*
 * Finish the compression...
 */
unsafe extern "C" fn archive_filter_b64encode_close(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut state: *mut private_b64encode = (*f).data as *mut private_b64encode;
    /* Flush remaining bytes. */
    if (*state).hold_len != 0 as libc::c_int as libc::c_ulong {
        la_b64_encode(
            &mut (*state).encoded_buff,
            (*state).hold.as_mut_ptr(),
            (*state).hold_len,
        );
    }
    archive_string_sprintf(
        &mut (*state).encoded_buff as *mut archive_string,
        b"====\n\x00" as *const u8 as *const libc::c_char,
    );
    /* Write the last block */
    archive_write_set_bytes_in_last_block((*f).archive, 1 as libc::c_int);
    return __archive_write_filter(
        (*f).next_filter,
        (*state).encoded_buff.s as *const libc::c_void,
        (*state).encoded_buff.length,
    );
}
unsafe extern "C" fn archive_filter_b64encode_free(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut state: *mut private_b64encode = (*f).data as *mut private_b64encode;
    archive_string_free(&mut (*state).name);
    archive_string_free(&mut (*state).encoded_buff);
    free(state as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn atol8(mut p: *const libc::c_char, mut char_cnt: size_t) -> int64_t {
    let mut l: int64_t = 0;
    let mut digit: libc::c_int = 0;
    l = 0 as libc::c_int as int64_t;
    loop {
        let fresh2 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        if !(fresh2 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '7' as i32) {
            break;
        }
        digit = *p as libc::c_int - '0' as i32;
        p = p.offset(1);
        l <<= 3 as libc::c_int;
        l |= digit as libc::c_long
    }
    return l;
}
