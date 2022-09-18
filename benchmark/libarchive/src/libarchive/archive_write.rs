use ::libc;
extern "C" {
    /* Length of malloc-ed storage in bytes. */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_write_finish_entry(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_close(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_entry_dev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_dev_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_ino64(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_ino_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn __archive_clean(_: *mut archive) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type dev_t = __dev_t;
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
 * $FreeBSD: src/lib/libarchive/archive.h.in,v 1.50 2008/05/26 17:00:22 kientzle Exp $
 */
/*
 * The version number is expressed as a single integer that makes it
 * easy to compare versions at build time: for version a.b.c, the
 * version number is printf("%d%03d%03d",a,b,c).  For example, if you
 * know your application requires version 2.12.108 or later, you can
 * assert that ARCHIVE_VERSION_NUMBER >= 2012108.
 */
/* Note: Compiler will complain if this does not match archive_entry.h! */
/* for wchar_t */
/* For FILE * */
/* For time_t */
/*
 * Note: archive.h is for use outside of libarchive; the configuration
 * headers (config.h, archive_platform.h, etc.) are purely internal.
 * Do NOT use HAVE_XXX configuration macros to control the behavior of
 * this header!  If you must conditionalize, use predefined compiler and/or
 * platform macros.
 */
/* Get appropriate definitions of 64-bit integer */
/* Older code relied on the __LA_INT64_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
pub type la_int64_t = int64_t;
/* The la_ssize_t should match the type used in 'struct stat' */
/* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
pub type la_ssize_t = ssize_t;
/*-
 * Copyright (c) 2003-2008 Tim Kientzle
 * Copyright (c) 2016 Martin Matuska
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
 * $FreeBSD: head/lib/libarchive/archive_entry.h 201096 2009-12-28 02:41:27Z kientzle $
 */
/* Note: Compiler will complain if this does not match archive.h! */
/*
 * Note: archive_entry.h is for use outside of libarchive; the
 * configuration headers (config.h, archive_platform.h, etc.) are
 * purely internal.  Do NOT use HAVE_XXX configuration macros to
 * control the behavior of this header!  If you must conditionalize,
 * use predefined compiler and/or platform macros.
 */
/* for wchar_t */
/* Get a suitable 64-bit integer type. */
/* The la_ssize_t should match the type used in 'struct stat' */
/* Get a suitable definition for mode_t */
/* Large file support for Android */
/*
 * On Windows, define LIBARCHIVE_STATIC if you're building or using a
 * .lib.  The default here assumes you're building a DLL.  Only
 * libarchive source should ever define __LIBARCHIVE_BUILD.
 */
/* Static libraries on all platforms and shared libraries on non-Windows. */
/*
 * Description of an archive entry.
 *
 * You can think of this as "struct stat" with some text fields added in.
 *
 * TODO: Add "comment", "charset", and possibly other entries that are
 * supported by "pax interchange" format.  However, GNU, ustar, cpio,
 * and other variants don't support these features, so they're not an
 * excruciatingly high priority right now.
 *
 * TODO: "pax interchange" format allows essentially arbitrary
 * key/value attributes to be attached to any entry.  Supporting
 * such extensions may make this library useful for special
 * applications (e.g., a package manager could attach special
 * package-management attributes to each entry).
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
/* Returns size actually written, zero on EOF, -1 on error. */
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
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
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
pub struct archive_none {
    pub buffer_size: size_t,
    pub avail: size_t,
    pub buffer: *mut libc::c_char,
    pub next: *mut libc::c_char,
}
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const INT_MAX: libc::c_int = 2147483647 as libc::c_int;
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
pub const ARCHIVE_STATE_CLOSED: libc::c_uint = 0x20 as libc::c_uint;
pub const ARCHIVE_STATE_HEADER: libc::c_uint = 2 as libc::c_uint;
pub const ARCHIVE_STATE_NEW: libc::c_uint = 1 as libc::c_uint;
pub const ARCHIVE_STATE_DATA: libc::c_uint = 4 as libc::c_uint;
pub const ARCHIVE_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
pub const ARCHIVE_WRITE_MAGIC: libc::c_uint = 0xb0c5c0de as libc::c_uint;
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
pub const ARCHIVE_WRITE_FILTER_STATE_NEW: libc::c_uint = 1 as libc::c_uint;
pub const ARCHIVE_WRITE_FILTER_STATE_OPEN: libc::c_uint = 2 as libc::c_uint;
pub const ARCHIVE_WRITE_FILTER_STATE_CLOSED: libc::c_uint = 4 as libc::c_uint;
pub const ARCHIVE_WRITE_FILTER_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
/*
 * This file contains the "essential" portions of the write API, that
 * is, stuff that will essentially always be used by any client that
 * actually needs to write an archive.  Optional pieces have been, as
 * far as possible, separated out into separate files to reduce
 * needlessly bloating statically-linked clients.
 */
unsafe extern "C" fn archive_write_vtable() -> *mut archive_vtable {
    static mut av: archive_vtable = archive_vtable {
        archive_close: None,
        archive_free: None,
        archive_write_header: None,
        archive_write_finish_entry: None,
        archive_write_data: None,
        archive_write_data_block: None,
        archive_read_next_header: None,
        archive_read_next_header2: None,
        archive_read_data_block: None,
        archive_filter_count: None,
        archive_filter_bytes: None,
        archive_filter_code: None,
        archive_filter_name: None,
    };
    static mut inited: libc::c_int = 0 as libc::c_int;
    if inited == 0 {
        av.archive_close =
            Some(_archive_write_close as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_filter_bytes = Some(
            _archive_filter_bytes
                as unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t,
        );
        av.archive_filter_code = Some(
            _archive_filter_code
                as unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int,
        );
        av.archive_filter_name = Some(
            _archive_filter_name
                as unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char,
        );
        av.archive_filter_count = Some(
            _archive_write_filter_count as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
        );
        av.archive_free =
            Some(_archive_write_free as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_write_header = Some(
            _archive_write_header
                as unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int,
        );
        av.archive_write_finish_entry = Some(
            _archive_write_finish_entry as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
        );
        av.archive_write_data = Some(
            _archive_write_data
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> ssize_t,
        );
        inited = 1 as libc::c_int
    }
    return &mut av;
}
/*-
 * To create an archive:
 *   1) Ask archive_write_new for an archive writer object.
 *   2) Set any global properties.  In particular, you should set
 *      the compression and format to use.
 *   3) Call archive_write_open to open the file (most people
 *       will use archive_write_open_file or archive_write_open_fd,
 *       which provide convenient canned I/O callbacks for you).
 *   4) For each entry:
 *      - construct an appropriate struct archive_entry structure
 *      - archive_write_header to write the header
 *      - archive_write_data to write the entry data
 *   5) archive_write_close to close the output
 *   6) archive_write_free to cleanup the writer and release resources
 */
/*
 * Allocate, initialize and return an archive object.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_new() -> *mut archive {
    let mut a: *mut archive_write = 0 as *mut archive_write;
    let mut nulls: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    a = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_write>() as libc::c_ulong,
    ) as *mut archive_write;
    if a.is_null() {
        return 0 as *mut archive;
    }
    (*a).archive.magic = ARCHIVE_WRITE_MAGIC;
    (*a).archive.state = ARCHIVE_STATE_NEW;
    (*a).archive.vtable = archive_write_vtable();
    /*
     * The value 10240 here matches the traditional tar default,
     * but is otherwise arbitrary.
     * TODO: Set the default block size from the format selected.
     */
    (*a).bytes_per_block = 10240 as libc::c_int; /* Default */
    (*a).bytes_in_last_block = -(1 as libc::c_int);
    /* Initialize a block of nulls for padding purposes. */
    (*a).null_length = 1024 as libc::c_int as size_t;
    nulls = calloc(1 as libc::c_int as libc::c_ulong, (*a).null_length) as *mut libc::c_uchar;
    if nulls.is_null() {
        free(a as *mut libc::c_void);
        return 0 as *mut archive;
    }
    (*a).nulls = nulls;
    return &mut (*a).archive;
}
/*
 * Set the block size.  Returns 0 if successful.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_bytes_per_block(
    mut _a: *mut archive,
    mut bytes_per_block: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_bytes_per_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).bytes_per_block = bytes_per_block;
    return 0 as libc::c_int;
}
/*
 * Get the current block size.  -1 if it has never been set.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_get_bytes_per_block(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_get_bytes_per_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return (*a).bytes_per_block;
}
/* XXX This is badly misnamed; suggestions appreciated. XXX */
/*
 * Set the size for the last block.
 * Returns 0 if successful.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_bytes_in_last_block(
    mut _a: *mut archive,
    mut bytes: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_set_bytes_in_last_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).bytes_in_last_block = bytes;
    return 0 as libc::c_int;
}
/*
 * Return the value set above.  -1 indicates it has not been set.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_get_bytes_in_last_block(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_get_bytes_in_last_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return (*a).bytes_in_last_block;
}
/* The dev/ino of a file that won't be archived.  This is used
 * to avoid recursively adding an archive to itself. */
/*
 * dev/ino of a file to be rejected.  Used to prevent adding
 * an archive to itself recursively.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_skip_file(
    mut _a: *mut archive,
    mut d: la_int64_t,
    mut i: la_int64_t,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_set_skip_file\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).skip_file_set = 1 as libc::c_int;
    (*a).skip_file_dev = d;
    (*a).skip_file_ino = i;
    return 0 as libc::c_int;
}
/*
 * Allocate and return the next filter structure.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_write_allocate_filter(
    mut _a: *mut archive,
) -> *mut archive_write_filter {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut f: *mut archive_write_filter = 0 as *mut archive_write_filter;
    f = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_write_filter>() as libc::c_ulong,
    ) as *mut archive_write_filter;
    (*f).archive = _a;
    (*f).state = ARCHIVE_WRITE_FILTER_STATE_NEW as libc::c_int;
    if (*a).filter_first.is_null() {
        (*a).filter_first = f
    } else {
        (*(*a).filter_last).next_filter = f
    }
    (*a).filter_last = f;
    return f;
}
/*
 * Write data to a particular filter.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_write_filter(
    mut f: *mut archive_write_filter,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    /* Never write to non-open filters */
    if (*f).state as libc::c_uint != ARCHIVE_WRITE_FILTER_STATE_OPEN {
        return -(30 as libc::c_int);
    }
    if length == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if (*f).write.is_none() {
        /* If unset, a fatal error has already occurred, so this filter
         * didn't open. We cannot write anything. */
        return -(30 as libc::c_int);
    }
    r = (*f).write.expect("non-null function pointer")(f, buff, length);
    (*f).bytes_written =
        ((*f).bytes_written as libc::c_ulong).wrapping_add(length) as int64_t as int64_t;
    return r;
}
/*
 * Recursive function for opening the filter chain
 * Last filter is opened first
 */
unsafe extern "C" fn __archive_write_open_filter(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = ARCHIVE_OK;
    if !(*f).next_filter.is_null() {
        ret = __archive_write_open_filter((*f).next_filter)
    }
    if ret != ARCHIVE_OK {
        return ret;
    }
    if (*f).state as libc::c_uint != ARCHIVE_WRITE_FILTER_STATE_NEW {
        return -(30 as libc::c_int);
    }
    if (*f).open.is_none() {
        (*f).state = ARCHIVE_WRITE_FILTER_STATE_OPEN as libc::c_int;
        return 0 as libc::c_int;
    }
    ret = (*f).open.expect("non-null function pointer")(f);
    if ret == ARCHIVE_OK {
        (*f).state = ARCHIVE_WRITE_FILTER_STATE_OPEN as libc::c_int
    } else {
        (*f).state = ARCHIVE_WRITE_FILTER_STATE_FATAL as libc::c_int
    }
    return ret;
}
/*
 * Open all filters
 */
unsafe extern "C" fn __archive_write_filters_open(mut a: *mut archive_write) -> libc::c_int {
    return __archive_write_open_filter((*a).filter_first);
}
/*
 * Close all filtes
 */
unsafe extern "C" fn __archive_write_filters_close(mut a: *mut archive_write) -> libc::c_int {
    let mut f: *mut archive_write_filter = 0 as *mut archive_write_filter;
    let mut ret: libc::c_int = 0;
    let mut ret1: libc::c_int = 0;
    ret = ARCHIVE_OK;
    f = (*a).filter_first;
    while !f.is_null() {
        /* Do not close filters that are not open */
        if (*f).state as libc::c_uint == ARCHIVE_WRITE_FILTER_STATE_OPEN {
            if (*f).close.is_some() {
                ret1 = (*f).close.expect("non-null function pointer")(f);
                if ret1 < ret {
                    ret = ret1
                }
                if ret1 == ARCHIVE_OK {
                    (*f).state = ARCHIVE_WRITE_FILTER_STATE_CLOSED as libc::c_int
                } else {
                    (*f).state = ARCHIVE_WRITE_FILTER_STATE_FATAL as libc::c_int
                }
            } else {
                (*f).state = ARCHIVE_WRITE_FILTER_STATE_CLOSED as libc::c_int
            }
        }
        f = (*f).next_filter
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_write_output(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    return __archive_write_filter((*a).filter_first, buff, length);
}
#[no_mangle]
pub unsafe extern "C" fn __archive_write_nulls(
    mut a: *mut archive_write,
    mut length: size_t,
) -> libc::c_int {
    if length == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while length > 0 as libc::c_int as libc::c_ulong {
        let mut to_write: size_t = if length < (*a).null_length {
            length
        } else {
            (*a).null_length
        };
        let mut r: libc::c_int =
            __archive_write_output(a, (*a).nulls as *const libc::c_void, to_write);
        if r < ARCHIVE_OK {
            return r;
        }
        length = (length as libc::c_ulong).wrapping_sub(to_write) as size_t as size_t
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_client_open(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut a: *mut archive_write = (*f).archive as *mut archive_write;
    let mut state: *mut archive_none = 0 as *mut archive_none;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut buffer_size: size_t = 0;
    let mut ret: libc::c_int = 0;
    (*f).bytes_per_block = archive_write_get_bytes_per_block((*f).archive);
    (*f).bytes_in_last_block = archive_write_get_bytes_in_last_block((*f).archive);
    buffer_size = (*f).bytes_per_block as size_t;
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_none>() as libc::c_ulong,
    ) as *mut archive_none;
    buffer = malloc(buffer_size) as *mut libc::c_char as *mut libc::c_void;
    if state.is_null() || buffer.is_null() {
        free(state as *mut libc::c_void);
        free(buffer);
        archive_set_error(
            (*f).archive,
            ENOMEM,
            b"Can\'t allocate data for output buffering\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*state).buffer_size = buffer_size;
    (*state).buffer = buffer as *mut libc::c_char;
    (*state).next = (*state).buffer;
    (*state).avail = (*state).buffer_size;
    (*f).data = state as *mut libc::c_void;
    if (*a).client_opener.is_none() {
        return 0 as libc::c_int;
    }
    ret = (*a).client_opener.expect("non-null function pointer")((*f).archive, (*a).client_data);
    if ret != ARCHIVE_OK {
        free((*state).buffer as *mut libc::c_void);
        free(state as *mut libc::c_void);
        (*f).data = NULL as *mut libc::c_void
    }
    return ret;
}
unsafe extern "C" fn archive_write_client_write(
    mut f: *mut archive_write_filter,
    mut _buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut a: *mut archive_write = (*f).archive as *mut archive_write;
    let mut state: *mut archive_none = (*f).data as *mut archive_none;
    let mut buff: *const libc::c_char = _buff as *const libc::c_char;
    let mut remaining: ssize_t = 0;
    let mut to_copy: ssize_t = 0;
    let mut bytes_written: ssize_t = 0;
    remaining = length as ssize_t;
    /*
     * If there is no buffer for blocking, just pass the data
     * straight through to the client write callback.  In
     * particular, this supports "no write delay" operation for
     * special applications.  Just set the block size to zero.
     */
    if (*state).buffer_size == 0 as libc::c_int as libc::c_ulong {
        while remaining > 0 as libc::c_int as libc::c_long {
            bytes_written = (*a).client_writer.expect("non-null function pointer")(
                &mut (*a).archive,
                (*a).client_data,
                buff as *const libc::c_void,
                remaining as size_t,
            );
            if bytes_written <= 0 as libc::c_int as libc::c_long {
                return -(30 as libc::c_int);
            }
            remaining -= bytes_written;
            buff = buff.offset(bytes_written as isize)
        }
        return 0 as libc::c_int;
    }
    /* If the copy buffer isn't empty, try to fill it. */
    if (*state).avail < (*state).buffer_size {
        /* If buffer is not empty... */
        /* ... copy data into buffer ... */
        to_copy = if remaining as size_t > (*state).avail {
            (*state).avail
        } else {
            remaining as size_t
        } as ssize_t;
        memcpy(
            (*state).next as *mut libc::c_void,
            buff as *const libc::c_void,
            to_copy as libc::c_ulong,
        );
        (*state).next = (*state).next.offset(to_copy as isize);
        (*state).avail = ((*state).avail as libc::c_ulong).wrapping_sub(to_copy as libc::c_ulong)
            as size_t as size_t;
        buff = buff.offset(to_copy as isize);
        remaining -= to_copy;
        /* ... if it's full, write it out. */
        if (*state).avail == 0 as libc::c_int as libc::c_ulong {
            let mut p: *mut libc::c_char = (*state).buffer;
            let mut to_write: size_t = (*state).buffer_size;
            while to_write > 0 as libc::c_int as libc::c_ulong {
                bytes_written = (*a).client_writer.expect("non-null function pointer")(
                    &mut (*a).archive,
                    (*a).client_data,
                    p as *const libc::c_void,
                    to_write,
                );
                if bytes_written <= 0 as libc::c_int as libc::c_long {
                    return -(30 as libc::c_int);
                }
                if bytes_written as size_t > to_write {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        -(1 as libc::c_int),
                        b"write overrun\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                p = p.offset(bytes_written as isize);
                to_write = (to_write as libc::c_ulong).wrapping_sub(bytes_written as libc::c_ulong)
                    as size_t as size_t
            }
            (*state).next = (*state).buffer;
            (*state).avail = (*state).buffer_size
        }
    }
    while remaining as size_t >= (*state).buffer_size {
        /* Write out full blocks directly to client. */
        bytes_written = (*a).client_writer.expect("non-null function pointer")(
            &mut (*a).archive,
            (*a).client_data,
            buff as *const libc::c_void,
            (*state).buffer_size,
        );
        if bytes_written <= 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int);
        }
        buff = buff.offset(bytes_written as isize);
        remaining -= bytes_written
    }
    if remaining > 0 as libc::c_int as libc::c_long {
        /* Copy last bit into copy buffer. */
        memcpy(
            (*state).next as *mut libc::c_void,
            buff as *const libc::c_void,
            remaining as libc::c_ulong,
        );
        (*state).next = (*state).next.offset(remaining as isize);
        (*state).avail = ((*state).avail as libc::c_ulong).wrapping_sub(remaining as libc::c_ulong)
            as size_t as size_t
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_client_close(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut a: *mut archive_write = (*f).archive as *mut archive_write;
    let mut state: *mut archive_none = (*f).data as *mut archive_none;
    let mut block_length: ssize_t = 0;
    let mut target_block_length: ssize_t = 0;
    let mut bytes_written: ssize_t = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    /* If there's pending data, pad and write the last block */
    if (*state).next != (*state).buffer {
        block_length = (*state).buffer_size.wrapping_sub((*state).avail) as ssize_t;
        /* Tricky calculation to determine size of last block */
        if (*a).bytes_in_last_block <= 0 as libc::c_int {
            /* Default or Zero: pad to full block */
            target_block_length = (*a).bytes_per_block as ssize_t
        } else {
            /* Round to next multiple of bytes_in_last_block. */
            target_block_length = (*a).bytes_in_last_block as libc::c_long
                * ((block_length + (*a).bytes_in_last_block as libc::c_long
                    - 1 as libc::c_int as libc::c_long)
                    / (*a).bytes_in_last_block as libc::c_long)
        }
        if target_block_length > (*a).bytes_per_block as libc::c_long {
            target_block_length = (*a).bytes_per_block as ssize_t
        }
        if block_length < target_block_length {
            memset(
                (*state).next as *mut libc::c_void,
                0 as libc::c_int,
                (target_block_length - block_length) as libc::c_ulong,
            );
            block_length = target_block_length
        }
        bytes_written = (*a).client_writer.expect("non-null function pointer")(
            &mut (*a).archive,
            (*a).client_data,
            (*state).buffer as *const libc::c_void,
            block_length as size_t,
        );
        ret = if bytes_written <= 0 as libc::c_int as libc::c_long {
            ARCHIVE_FATAL
        } else {
            ARCHIVE_OK
        }
    }
    if (*a).client_closer.is_some() {
        Some((*a).client_closer.expect("non-null function pointer"))
            .expect("non-null function pointer")(&mut (*a).archive, (*a).client_data);
    }
    free((*state).buffer as *mut libc::c_void);
    free(state as *mut libc::c_void);
    (*a).client_data = NULL as *mut libc::c_void;
    /* Clear passphrase. */
    if !(*a).passphrase.is_null() {
        memset(
            (*a).passphrase as *mut libc::c_void,
            0 as libc::c_int,
            strlen((*a).passphrase),
        );
        free((*a).passphrase as *mut libc::c_void);
        (*a).passphrase = NULL as *mut libc::c_char
    }
    /* Clear the close handler myself not to be called again. */
    (*f).state = ARCHIVE_WRITE_FILTER_STATE_CLOSED as libc::c_int;
    return ret;
}
/*
 * Open the archive using the current settings.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_open(
    mut _a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut opener: Option<archive_open_callback>,
    mut writer: Option<archive_write_callback>,
    mut closer: Option<archive_close_callback>,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut client_filter: *mut archive_write_filter = 0 as *mut archive_write_filter;
    let mut ret: libc::c_int = 0;
    let mut r1: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_open\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_clear_error(&mut (*a).archive);
    (*a).client_writer = writer;
    (*a).client_opener = opener;
    (*a).client_closer = closer;
    (*a).client_data = client_data;
    client_filter = __archive_write_allocate_filter(_a);
    (*client_filter).open = Some(
        archive_write_client_open
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*client_filter).write = Some(
        archive_write_client_write
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_int,
    );
    (*client_filter).close = Some(
        archive_write_client_close
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    ret = __archive_write_filters_open(a);
    if ret < ARCHIVE_WARN {
        r1 = __archive_write_filters_close(a);
        __archive_write_filters_free(_a);
        return if r1 < ret { r1 } else { ret };
    }
    (*a).archive.state = ARCHIVE_STATE_HEADER;
    if (*a).format_init.is_some() {
        ret = (*a).format_init.expect("non-null function pointer")(a)
    }
    return ret;
}
/*
 * Close out the archive.
 */
unsafe extern "C" fn _archive_write_close(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write; /* Okay to close() when not open. */
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r1: libc::c_int = ARCHIVE_OK;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_write_close\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state == ARCHIVE_STATE_NEW || (*a).archive.state == ARCHIVE_STATE_CLOSED {
        return 0 as libc::c_int;
    }
    archive_clear_error(&mut (*a).archive);
    /* Finish the last entry if a finish callback is specified */
    if (*a).archive.state == ARCHIVE_STATE_DATA && (*a).format_finish_entry.is_some() {
        r = (*a).format_finish_entry.expect("non-null function pointer")(a)
    }
    /* Finish off the archive. */
    /* TODO: have format closers invoke compression close. */
    if (*a).format_close.is_some() {
        r1 = (*a).format_close.expect("non-null function pointer")(a);
        if r1 < r {
            r = r1
        }
    }
    /* Finish the compression and close the stream. */
    r1 = __archive_write_filters_close(a);
    if r1 < r {
        r = r1
    }
    if (*a).archive.state != ARCHIVE_STATE_FATAL {
        (*a).archive.state = ARCHIVE_STATE_CLOSED
    }
    return r;
}
unsafe extern "C" fn _archive_write_filter_count(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut p: *mut archive_write_filter = (*a).filter_first;
    let mut count: libc::c_int = 0 as libc::c_int;
    while !p.is_null() {
        count += 1;
        p = (*p).next_filter
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_write_filters_free(mut _a: *mut archive) {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r1: libc::c_int = 0;
    while !(*a).filter_first.is_null() {
        let mut next: *mut archive_write_filter = (*(*a).filter_first).next_filter;
        if (*(*a).filter_first).free.is_some() {
            r1 = Some(
                (*(*a).filter_first)
                    .free
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")((*a).filter_first);
            if r > r1 {
                r = r1
            }
        }
        free((*a).filter_first as *mut libc::c_void);
        (*a).filter_first = next
    }
    (*a).filter_last = NULL as *mut archive_write_filter;
}
/*
 * Destroy the archive structure.
 *
 * Be careful: user might just call write_new and then write_free.
 * Don't assume we actually wrote anything or performed any non-trivial
 * initialization.
 */
unsafe extern "C" fn _archive_write_free(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r1: libc::c_int = 0;
    if _a.is_null() {
        return 0 as libc::c_int;
    }
    /* It is okay to call free() in state FATAL. */
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_write_free\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state != ARCHIVE_STATE_FATAL {
        r = archive_write_close(&mut (*a).archive)
    }
    /* Release format resources. */
    if (*a).format_free.is_some() {
        r1 = (*a).format_free.expect("non-null function pointer")(a);
        if r1 < r {
            r = r1
        }
    }
    __archive_write_filters_free(_a);
    /* Release various dynamic buffers. */
    free((*a).nulls as *const libc::c_void as uintptr_t as *mut libc::c_void);
    archive_string_free(&mut (*a).archive.error_string);
    if !(*a).passphrase.is_null() {
        /* A passphrase should be cleaned. */
        memset(
            (*a).passphrase as *mut libc::c_void,
            0 as libc::c_int,
            strlen((*a).passphrase),
        );
        free((*a).passphrase as *mut libc::c_void);
    }
    (*a).archive.magic = 0 as libc::c_int as libc::c_uint;
    __archive_clean(&mut (*a).archive);
    free(a as *mut libc::c_void);
    return r;
}
/*
 * Write the appropriate header.
 */
unsafe extern "C" fn _archive_write_header(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut ret: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        4 as libc::c_uint | 2 as libc::c_uint,
        b"archive_write_header\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_clear_error(&mut (*a).archive);
    if (*a).format_write_header.is_none() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Format must be set before you can write to an archive.\x00" as *const u8
                as *const libc::c_char,
        );
        (*a).archive.state = ARCHIVE_STATE_FATAL;
        return -(30 as libc::c_int);
    }
    /* In particular, "retry" and "fatal" get returned immediately. */
    ret = archive_write_finish_entry(&mut (*a).archive);
    if ret == ARCHIVE_FATAL {
        (*a).archive.state = ARCHIVE_STATE_FATAL;
        return -(30 as libc::c_int);
    }
    if ret < ARCHIVE_OK && ret != ARCHIVE_WARN {
        return ret;
    }
    if (*a).skip_file_set != 0
        && archive_entry_dev_is_set(entry) != 0
        && archive_entry_ino_is_set(entry) != 0
        && archive_entry_dev(entry) == (*a).skip_file_dev as dev_t
        && archive_entry_ino64(entry) == (*a).skip_file_ino
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            0 as libc::c_int,
            b"Can\'t add archive to itself\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* Format and write header. */
    r2 = (*a).format_write_header.expect("non-null function pointer")(a, entry);
    if r2 == ARCHIVE_FAILED {
        return -(25 as libc::c_int);
    }
    if r2 == ARCHIVE_FATAL {
        (*a).archive.state = ARCHIVE_STATE_FATAL;
        return -(30 as libc::c_int);
    }
    if r2 < ret {
        ret = r2
    }
    (*a).archive.state = ARCHIVE_STATE_DATA;
    return ret;
}
unsafe extern "C" fn _archive_write_finish_entry(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut ret: libc::c_int = ARCHIVE_OK;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_write_finish_entry\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state & ARCHIVE_STATE_DATA != 0 && (*a).format_finish_entry.is_some() {
        ret = (*a).format_finish_entry.expect("non-null function pointer")(a)
    }
    (*a).archive.state = ARCHIVE_STATE_HEADER;
    return ret;
}
/*
 * Note that the compressor is responsible for blocking.
 */
unsafe extern "C" fn _archive_write_data(
    mut _a: *mut archive,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let max_write: size_t = INT_MAX as size_t;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        4 as libc::c_uint,
        b"archive_write_data\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL as ssize_t;
    }
    /* In particular, this catches attempts to pass negative values. */
    if s > max_write {
        s = max_write
    }
    archive_clear_error(&mut (*a).archive);
    return (*a).format_write_data.expect("non-null function pointer")(a, buff, s);
}
unsafe extern "C" fn filter_lookup(
    mut _a: *mut archive,
    mut n: libc::c_int,
) -> *mut archive_write_filter {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut f: *mut archive_write_filter = (*a).filter_first;
    if n == -(1 as libc::c_int) {
        return (*a).filter_last;
    }
    if n < 0 as libc::c_int {
        return NULL as *mut archive_write_filter;
    }
    while n > 0 as libc::c_int && !f.is_null() {
        f = (*f).next_filter;
        n -= 1
    }
    return f;
}
unsafe extern "C" fn _archive_filter_code(mut _a: *mut archive, mut n: libc::c_int) -> libc::c_int {
    let mut f: *mut archive_write_filter = filter_lookup(_a, n);
    return if f.is_null() {
        -(1 as libc::c_int)
    } else {
        (*f).code
    };
}
unsafe extern "C" fn _archive_filter_name(
    mut _a: *mut archive,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut f: *mut archive_write_filter = filter_lookup(_a, n);
    return if !f.is_null() {
        (*f).name
    } else {
        NULL as *const libc::c_char
    };
}
unsafe extern "C" fn _archive_filter_bytes(mut _a: *mut archive, mut n: libc::c_int) -> int64_t {
    let mut f: *mut archive_write_filter = filter_lookup(_a, n);
    return if f.is_null() {
        -(1 as libc::c_int) as libc::c_long
    } else {
        (*f).bytes_written
    };
}
