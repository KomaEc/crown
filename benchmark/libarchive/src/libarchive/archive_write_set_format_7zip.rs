use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type internal_state;
    pub type archive_string_conv;
    /* Declare our basic types. */
    /*-
     * Copyright (c) 2011 Michihiro NAKAJIMA
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
     * $FreeBSD$
     */
    pub type archive_entry;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn deflateInit2_(
        strm: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    /*
     * Retrieve fields from an archive_entry.
     *
     * There are a number of implicit conversions among these fields.  For
     * example, if a regular string field is set and you read the _w wide
     * character field, the entry will implicitly convert narrow-to-wide
     * using the current locale.  Similarly, dev values are automatically
     * updated when you write devmajor or devminor and vice versa.
     *
     * In addition, fields can be "set" or "unset."  Unset string fields
     * return NULL, non-string fields have _is_set() functions to test
     * whether they've been set.  You can "unset" a string field by
     * assigning NULL; non-string fields have _unset() functions to
     * unset them.
     *
     * Note: There is one ambiguity in the above; string fields will
     * also return NULL when implicit character set conversions fail.
     * This is usually what you want.
     */
    #[no_mangle]
    fn archive_entry_atime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_atime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_atime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_ctime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_mtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn _archive_entry_pathname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
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
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_string_conversion_to_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* must be 32-bit at least */
    /* ---------- Decode ---------- */
    /* ---------- Encode ---------- */
    /* Base Functions */
    /* Decode Functions */
    /* Encode Functions */
    #[no_mangle]
    static __archive_ppmd7_functions: IPpmd7;
    #[no_mangle]
    fn __archive_mktemp(tmpdir: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __archive_rb_tree_init(_: *mut archive_rb_tree, _: *const archive_rb_tree_ops);
    #[no_mangle]
    fn __archive_rb_tree_insert_node(
        _: *mut archive_rb_tree,
        _: *mut archive_rb_node,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_rb_tree_iterate(
        _: *mut archive_rb_tree,
        _: *mut archive_rb_node,
        _: libc::c_uint,
    ) -> *mut archive_rb_node;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type size_t = libc::c_ulong;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
/* No more operations are possible. */
/*
 * As far as possible, archive_errno returns standard platform errno codes.
 * Of course, the details vary by platform, so the actual definitions
 * here are stored in "archive_platform.h".  The symbols are listed here
 * for reference; as a rule, clients should not need to know the exact
 * platform-dependent error code.
 */
/* Unrecognized or invalid file format. */
/* #define	ARCHIVE_ERRNO_FILE_FORMAT */
/* Illegal usage of the library. */
/* #define	ARCHIVE_ERRNO_PROGRAMMER_ERROR */
/* Unknown or unclassified error. */
/* #define	ARCHIVE_ERRNO_MISC */
/*
 * Callbacks are invoked to automatically read/skip/write/open/close the
 * archive. You can provide your own for complex tasks (like breaking
 * archives across multiple tapes) or use standard ones built into the
 * library.
 */
/* Returns pointer and size of next block of data from archive. */
pub type archive_read_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut *const libc::c_void,
) -> la_ssize_t;
/* Skips at most request bytes from archive and returns the skipped amount.
 * This may skip fewer bytes than requested; it may even skip zero bytes.
 * If you do skip fewer bytes than requested, libarchive will invoke your
 * read callback and discard data as necessary to make up the full skip.
 */
pub type archive_skip_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void, _: la_int64_t) -> la_int64_t;
/* Seeks to specified location in the file and returns the position.
 * Whence values are SEEK_SET, SEEK_CUR, SEEK_END from stdio.h.
 * Return ARCHIVE_FATAL if the seek fails for any reason.
 */
pub type archive_seek_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: la_int64_t,
    _: libc::c_int,
) -> la_int64_t;
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
/* Switches from one client data object to the next/prev client data object.
 * This is useful for reading from different data blocks such as a set of files
 * that make up one large file.
 */
pub type archive_switch_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut libc::c_void,
) -> libc::c_int;
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
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
/* Associated archive. */
/* Who I write to. */
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
pub struct _7zip {
    pub temp_fd: libc::c_int,
    pub temp_offset: uint64_t,
    pub cur_file: *mut file,
    pub total_number_entry: size_t,
    pub total_number_nonempty_entry: size_t,
    pub total_number_empty_entry: size_t,
    pub total_number_dir_entry: size_t,
    pub total_bytes_entry_name: size_t,
    pub total_number_time_defined: [size_t; 3],
    pub total_bytes_compressed: uint64_t,
    pub total_bytes_uncompressed: uint64_t,
    pub entry_bytes_remaining: uint64_t,
    pub entry_crc32: uint32_t,
    pub precode_crc32: uint32_t,
    pub encoded_crc32: uint32_t,
    pub crc32flg: libc::c_int,
    pub opt_compression: libc::c_uint,
    pub opt_compression_level: libc::c_int,
    pub stream: la_zstream,
    pub coder: coder,
    pub sconv: *mut archive_string_conv,
    pub wbuff: [libc::c_uchar; 61440],
    pub wbuff_remaining: size_t,
    pub file_list: C2RustUnnamed,
    pub empty_list: C2RustUnnamed,
    pub rbtree: archive_rb_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree {
    pub rbt_root: *mut archive_rb_node,
    pub rbt_ops: *const archive_rb_tree_ops,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree_ops {
    pub rbto_compare_nodes: archive_rbto_compare_nodes_fn,
    pub rbto_compare_key: archive_rbto_compare_key_fn,
}
pub type archive_rbto_compare_key_fn =
    Option<unsafe extern "C" fn(_: *const archive_rb_node, _: *const libc::c_void) -> libc::c_int>;
/*-
 * Copyright (c) 2001 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Matt Thomas <matt@3am-software.com>.
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
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Based on NetBSD: rb.h,v 1.13 2009/08/16 10:57:01 yamt Exp
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_node {
    pub rb_nodes: [*mut archive_rb_node; 2],
    pub rb_info: uintptr_t,
}
/*
 * archive_rbto_compare_nodes_fn:
 *	return a positive value if the first node < the second node.
 *	return a negative value if the first node > the second node.
 *	return 0 if they are considered same.
 *
 * archive_rbto_compare_key_fn:
 *	return a positive value if the node < the key.
 *	return a negative value if the node > the key.
 *	return 0 if they are considered same.
 */
pub type archive_rbto_compare_nodes_fn = Option<
    unsafe extern "C" fn(_: *const archive_rb_node, _: *const archive_rb_node) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub first: *mut file,
    pub last: *mut *mut file,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub rbnode: archive_rb_node,
    pub next: *mut file,
    pub name_len: libc::c_uint,
    pub utf16name: *mut uint8_t,
    pub size: uint64_t,
    pub flg: libc::c_uint,
    pub times: [C2RustUnnamed_0; 3],
    pub mode: mode_t,
    pub crc32: uint32_t,
    #[bitfield(name = "dir", ty = "libc::c_int", bits = "0..=0")]
    pub dir: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub time: time_t,
    pub time_ns: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coder {
    pub codec: libc::c_uint,
    pub prop_size: size_t,
    pub props: *mut uint8_t,
}
/*
 * A stream object of universal compressor.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct la_zstream {
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
    pub total_in: uint64_t,
    pub next_out: *mut uint8_t,
    pub avail_out: size_t,
    pub total_out: uint64_t,
    pub prop_size: uint32_t,
    pub props: *mut uint8_t,
    pub valid: libc::c_int,
    pub real_stream: *mut libc::c_void,
    pub code: Option<
        unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream, _: la_zaction) -> libc::c_int,
    >,
    pub end: Option<unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream) -> libc::c_int>,
}
pub type la_zaction = libc::c_uint;
pub const ARCHIVE_Z_RUN: la_zaction = 1;
pub const ARCHIVE_Z_FINISH: la_zaction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ppmd_stream {
    pub stat: libc::c_int,
    pub ppmd7_context: CPpmd7,
    pub range_enc: CPpmd7z_RangeEnc,
    pub byteout: IByteOut,
    pub buff: *mut uint8_t,
    pub buff_ptr: *mut uint8_t,
    pub buff_end: *mut uint8_t,
    pub buff_bytes: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteOut {
    pub a: *mut archive_write,
    pub Write: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7z_RangeEnc {
    pub Low: UInt64,
    pub Range: UInt32,
    pub Cache: Byte,
    pub CacheSize: UInt64,
    pub Stream: *mut IByteOut,
}
pub type UInt64 = libc::c_ulonglong;
pub type UInt32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7 {
    pub MinContext: *mut CPpmd7_Context,
    pub MaxContext: *mut CPpmd7_Context,
    pub FoundState: *mut CPpmd_State,
    pub OrderFall: libc::c_uint,
    pub InitEsc: libc::c_uint,
    pub PrevSuccess: libc::c_uint,
    pub MaxOrder: libc::c_uint,
    pub HiBitsFlag: libc::c_uint,
    pub RunLength: Int32,
    pub InitRL: Int32,
    pub Size: UInt32,
    pub GlueCount: UInt32,
    pub Base: *mut Byte,
    pub LoUnit: *mut Byte,
    pub HiUnit: *mut Byte,
    pub Text: *mut Byte,
    pub UnitsStart: *mut Byte,
    pub AlignOffset: UInt32,
    pub Indx2Units: [Byte; 38],
    pub Units2Indx: [Byte; 128],
    pub FreeList: [CPpmd_Void_Ref; 38],
    pub NS2Indx: [Byte; 256],
    pub NS2BSIndx: [Byte; 256],
    pub HB2Flag: [Byte; 256],
    pub DummySee: CPpmd_See,
    pub See: [[CPpmd_See; 16]; 25],
    pub BinSumm: [[UInt16; 64]; 128],
}
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_See {
    pub Summ: UInt16,
    pub Shift: Byte,
    pub Count: Byte,
}
pub type CPpmd_Void_Ref = UInt32;
pub type Int32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_State {
    pub Symbol: Byte,
    pub Freq: Byte,
    pub SuccessorLow: UInt16,
    pub SuccessorHigh: UInt16,
}
pub type CPpmd7_Context = CPpmd7_Context_;
/* Ppmd7.h -- PPMdH compression codec
2010-03-12 : Igor Pavlov : Public domain
This code is based on PPMd var.H (2001): Dmitry Shkarin : Public domain */
/* This code supports virtual RangeDecoder and includes the implementation
of RangeCoder from 7z, instead of RangeCoder from original PPMd var.H.
If you need the compatibility with original PPMd var.H, you can use external RangeDecoder */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7_Context_ {
    pub NumStats: UInt16,
    pub SummFreq: UInt16,
    pub Stats: CPpmd_State_Ref,
    pub Suffix: CPpmd7_Context_Ref,
}
pub type CPpmd7_Context_Ref = UInt32;
pub type CPpmd_State_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPpmd7 {
    pub Ppmd7_Construct: Option<unsafe extern "C" fn(_: *mut CPpmd7) -> ()>,
    pub Ppmd7_Alloc: Option<unsafe extern "C" fn(_: *mut CPpmd7, _: UInt32) -> Bool>,
    pub Ppmd7_Free: Option<unsafe extern "C" fn(_: *mut CPpmd7) -> ()>,
    pub Ppmd7_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7, _: libc::c_uint) -> ()>,
    pub Ppmd7z_RangeDec_CreateVTable: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> ()>,
    pub PpmdRAR_RangeDec_CreateVTable: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> ()>,
    pub Ppmd7z_RangeDec_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> Bool>,
    pub PpmdRAR_RangeDec_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> Bool>,
    pub Ppmd7_DecodeSymbol:
        Option<unsafe extern "C" fn(_: *mut CPpmd7, _: *mut IPpmd7_RangeDec) -> libc::c_int>,
    pub Ppmd7z_RangeEnc_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeEnc) -> ()>,
    pub Ppmd7z_RangeEnc_FlushData: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeEnc) -> ()>,
    pub Ppmd7_EncodeSymbol: Option<
        unsafe extern "C" fn(_: *mut CPpmd7, _: *mut CPpmd7z_RangeEnc, _: libc::c_int) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPpmd7_RangeDec {
    pub GetThreshold: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32>,
    pub Decode: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32, _: UInt32) -> ()>,
    pub DecodeBit: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7z_RangeDec {
    pub p: IPpmd7_RangeDec,
    pub Range: UInt32,
    pub Code: UInt32,
    pub Low: UInt32,
    pub Bottom: UInt32,
    pub Stream: *mut IByteIn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteIn {
    pub a: *mut archive_read,
    pub Read: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> Byte>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read {
    pub archive: archive,
    pub entry: *mut archive_entry,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub client: archive_read_client,
    pub bidders: [archive_read_filter_bidder; 16],
    pub filter: *mut archive_read_filter,
    pub bypass_filter_bidding: libc::c_int,
    pub header_position: int64_t,
    pub data_start_node: libc::c_uint,
    pub data_end_node: libc::c_uint,
    pub formats: [archive_format_descriptor; 16],
    pub format: *mut archive_format_descriptor,
    pub extract: *mut archive_read_extract,
    pub cleanup_archive_extract: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub passphrases: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub first: *mut archive_read_passphrase,
    pub last: *mut *mut archive_read_passphrase,
    pub candidate: libc::c_int,
    pub callback: Option<archive_passphrase_callback>,
    pub client_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_passphrase {
    pub passphrase: *mut libc::c_char,
    pub next: *mut archive_read_passphrase,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_extract {
    pub ad: *mut archive,
    pub extract_progress: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub extract_progress_user_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_format_descriptor {
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub bid: Option<unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int>,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_read,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub read_header:
        Option<unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int>,
    pub read_data: Option<
        unsafe extern "C" fn(
            _: *mut archive_read,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub read_data_skip: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub seek_data:
        Option<unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub format_capabilties: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub has_encrypted_entries: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_filter {
    pub position: int64_t,
    pub bidder: *mut archive_read_filter_bidder,
    pub upstream: *mut archive_read_filter,
    pub archive: *mut archive_read,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int>,
    pub read: Option<
        unsafe extern "C" fn(_: *mut archive_read_filter, _: *mut *const libc::c_void) -> ssize_t,
    >,
    pub skip: Option<unsafe extern "C" fn(_: *mut archive_read_filter, _: int64_t) -> int64_t>,
    pub seek: Option<
        unsafe extern "C" fn(_: *mut archive_read_filter, _: int64_t, _: libc::c_int) -> int64_t,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int>,
    pub sswitch:
        Option<unsafe extern "C" fn(_: *mut archive_read_filter, _: libc::c_uint) -> libc::c_int>,
    pub read_header: Option<
        unsafe extern "C" fn(_: *mut archive_read_filter, _: *mut archive_entry) -> libc::c_int,
    >,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub buffer: *mut libc::c_char,
    pub buffer_size: size_t,
    pub next: *mut libc::c_char,
    pub avail: size_t,
    pub client_buff: *const libc::c_void,
    pub client_total: size_t,
    pub client_next: *const libc::c_char,
    pub client_avail: size_t,
    pub end_of_file: libc::c_char,
    pub closed: libc::c_char,
    pub fatal: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_filter_bidder {
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub bid: Option<
        unsafe extern "C" fn(
            _: *mut archive_read_filter_bidder,
            _: *mut archive_read_filter,
        ) -> libc::c_int,
    >,
    pub init: Option<unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int>,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_read_filter_bidder,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_read_filter_bidder) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_client {
    pub opener: Option<archive_open_callback>,
    pub reader: Option<archive_read_callback>,
    pub skipper: Option<archive_skip_callback>,
    pub seeker: Option<archive_seek_callback>,
    pub closer: Option<archive_close_callback>,
    pub switcher: Option<archive_switch_callback>,
    pub nodes: libc::c_uint,
    pub cursor: libc::c_uint,
    pub position: int64_t,
    pub dataset: *mut archive_read_data_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_data_node {
    pub begin_position: int64_t,
    pub total_size: int64_t,
    pub data: *mut libc::c_void,
}
pub type Bool = libc::c_int;
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
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * Codes to identify various stream filters.
 */
/*
 * Codes returned by archive_format.
 *
 * Top 16 bits identifies the format family (e.g., "tar"); lower
 * 16 bits indicate the variant.  This is updated by read_next_header.
 * Note that the lower 16 bits will often vary from entry to entry.
 * In some cases, this variation occurs as libarchive learns more about
 * the archive (for example, later entries might utilize extensions that
 * weren't necessary earlier in the archive; in this case, libarchive
 * will change the format code to indicate the extended format that
 * was used).  In other cases, it's because different tools have
 * modified the archive and so different parts of the archive
 * actually have slightly different formats.  (Both tar and cpio store
 * format codes in each entry, so it is quite possible for each
 * entry to be in a different format.)
 */
pub const ARCHIVE_FORMAT_7ZIP: libc::c_int = 0xe0000 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn archive_be32enc(mut pp: *mut libc::c_void, mut u: uint32_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) =
        (u >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(3 as libc::c_int as isize) =
        (u & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_be64enc(mut pp: *mut libc::c_void, mut u: uint64_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    archive_be32enc(p as *mut libc::c_void, (u >> 32 as libc::c_int) as uint32_t);
    archive_be32enc(
        p.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        (u & 0xffffffff as libc::c_uint as libc::c_ulong) as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn archive_le32enc(mut pp: *mut libc::c_void, mut u: uint32_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) =
        (u >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(3 as libc::c_int as isize) =
        (u >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_le64enc(mut pp: *mut libc::c_void, mut u: uint64_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    archive_le32enc(
        p as *mut libc::c_void,
        (u & 0xffffffff as libc::c_uint as libc::c_ulong) as uint32_t,
    );
    archive_le32enc(
        p.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        (u >> 32 as libc::c_int) as uint32_t,
    );
}
/*
 * File-type constants.  These are returned from archive_entry_filetype()
 * and passed to archive_entry_set_filetype().
 *
 * These values match S_XXX defines on every platform I've checked,
 * including Windows, AIX, Linux, Solaris, and BSD.  They're
 * (re)defined here because platforms generally don't define the ones
 * they don't support.  For example, Windows doesn't define S_IFLNK or
 * S_IFBLK.  Instead of having a mass of conditional logic and system
 * checks to define any S_XXX values that aren't supported locally,
 * I've just defined a new set of such constants so that
 * libarchive-based applications can manipulate and identify archive
 * entries properly even if the hosting platform can't store them on
 * disk.
 *
 * These values are also used directly within some portable formats,
 * such as cpio.  If you find a platform that varies from these, the
 * correct solution is to leave these alone and translate from these
 * portable values to platform-native values when entries are read from
 * or written to disk.
 */
/*
 * In libarchive 4.0, we can drop the casts here.
 * They're needed to work around Borland C's broken mode_t.
 */
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const archive_entry_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_pathname_l;
pub const ARCHIVE_RB_DIR_LEFT: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_RB_DIR_RIGHT: libc::c_int = 1 as libc::c_int;
/*
 * Codec ID
 */
pub const _7Z_COPY: libc::c_int = 0 as libc::c_int;
pub const _7Z_LZMA1: libc::c_int = 0x30101 as libc::c_int;
pub const _7Z_LZMA2: libc::c_int = 0x21 as libc::c_int;
pub const _7Z_DEFLATE: libc::c_int = 0x40108 as libc::c_int;
pub const _7Z_BZIP2: libc::c_int = 0x40202 as libc::c_int;
pub const _7Z_PPMD: libc::c_int = 0x30401 as libc::c_int;
/*
 * 7-Zip header property IDs.
 */
pub const kEnd: libc::c_int = 0 as libc::c_int;
pub const kHeader: libc::c_int = 0x1 as libc::c_int;
pub const kMainStreamsInfo: libc::c_int = 0x4 as libc::c_int;
pub const kFilesInfo: libc::c_int = 0x5 as libc::c_int;
pub const kPackInfo: libc::c_int = 0x6 as libc::c_int;
pub const kUnPackInfo: libc::c_int = 0x7 as libc::c_int;
pub const kSubStreamsInfo: libc::c_int = 0x8 as libc::c_int;
pub const kSize: libc::c_int = 0x9 as libc::c_int;
pub const kCRC: libc::c_int = 0xa as libc::c_int;
pub const kFolder: libc::c_int = 0xb as libc::c_int;
pub const kCodersUnPackSize: libc::c_int = 0xc as libc::c_int;
pub const kNumUnPackStream: libc::c_int = 0xd as libc::c_int;
pub const kEmptyStream: libc::c_int = 0xe as libc::c_int;
pub const kEmptyFile: libc::c_int = 0xf as libc::c_int;
pub const kName: libc::c_int = 0x11 as libc::c_int;
pub const kCTime: libc::c_int = 0x12 as libc::c_int;
pub const kATime: libc::c_int = 0x13 as libc::c_int;
pub const kMTime: libc::c_int = 0x14 as libc::c_int;
pub const kAttributes: libc::c_int = 0x15 as libc::c_int;
pub const kEncodedHeader: libc::c_int = 0x17 as libc::c_int;
pub const PPMD7_DEFAULT_ORDER: libc::c_int = 6 as libc::c_int;
pub const PPMD7_DEFAULT_MEM_SIZE: libc::c_int = (1 as libc::c_int) << 24 as libc::c_int;
pub const MTIME_IS_SET: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const ATIME_IS_SET: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
pub const CTIME_IS_SET: libc::c_int = (1 as libc::c_int) << 2 as libc::c_int;
pub const MTIME: libc::c_int = 0 as libc::c_int;
pub const ATIME: libc::c_int = 1 as libc::c_int;
pub const CTIME: libc::c_int = 2 as libc::c_int;
pub const PRECODE_CRC32: libc::c_int = 1 as libc::c_int;
pub const ENCODED_CRC32: libc::c_int = 2 as libc::c_int;
/* To minimize link pollution, use one or more of the following. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_7zip(mut _a: *mut archive) -> libc::c_int {
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    file_cmp_node
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    file_cmp_key
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_7zip\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If another format was already registered, unregister it. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    zip = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_7zip>() as libc::c_ulong,
    ) as *mut _7zip;
    if zip.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate 7-Zip data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*zip).temp_fd = -(1 as libc::c_int);
    __archive_rb_tree_init(&mut (*zip).rbtree, &rb_ops);
    file_init_register(zip);
    file_init_register_empty(zip);
    /* Set default compression type and its level. */
    (*zip).opt_compression = _7Z_DEFLATE as libc::c_uint;
    (*zip).opt_compression_level = 6 as libc::c_int;
    (*a).format_data = zip as *mut libc::c_void;
    (*a).format_name = b"7zip\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        _7z_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        _7z_write_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        _7z_write_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry =
        Some(_7z_finish_entry as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_close =
        Some(_7z_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_free = Some(_7z_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).archive.archive_format = ARCHIVE_FORMAT_7ZIP;
    (*a).archive.archive_format_name = b"7zip\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
/* for empty files */
unsafe extern "C" fn _7z_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    zip = (*a).format_data as *mut _7zip;
    if strcmp(key, b"compression\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        let mut name: *const libc::c_char = NULL as *const libc::c_char;
        if value.is_null()
            || strcmp(value, b"copy\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(value, b"COPY\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(value, b"store\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(value, b"STORE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*zip).opt_compression = _7Z_COPY as libc::c_uint
        } else if strcmp(value, b"deflate\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(value, b"DEFLATE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*zip).opt_compression = _7Z_DEFLATE as libc::c_uint
        } else if strcmp(value, b"bzip2\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(value, b"BZIP2\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            name = b"bzip2\x00" as *const u8 as *const libc::c_char
        } else if strcmp(value, b"lzma1\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(value, b"LZMA1\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            name = b"lzma1\x00" as *const u8 as *const libc::c_char
        } else if strcmp(value, b"lzma2\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(value, b"LZMA2\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            name = b"lzma2\x00" as *const u8 as *const libc::c_char
        } else if strcmp(value, b"ppmd\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(value, b"PPMD\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(value, b"PPMd\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*zip).opt_compression = _7Z_PPMD as libc::c_uint
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unknown compression name: `%s\'\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(25 as libc::c_int);
        }
        if !name.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"`%s\' compression not supported on this platform\x00" as *const u8
                    as *const libc::c_char,
                name,
            );
            return -(25 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if strcmp(
        key,
        b"compression-level\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        if value.is_null()
            || !(*value.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *value.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
            || *value.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Illegal value `%s\'\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(25 as libc::c_int);
        }
        (*zip).opt_compression_level =
            *value.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
        return 0 as libc::c_int;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn _7z_write_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut file: *mut file = 0 as *mut file;
    let mut r: libc::c_int = 0;
    zip = (*a).format_data as *mut _7zip;
    (*zip).cur_file = NULL as *mut file;
    (*zip).entry_bytes_remaining = 0 as libc::c_int as uint64_t;
    if (*zip).sconv.is_null() {
        (*zip).sconv = archive_string_conversion_to_charset(
            &mut (*a).archive,
            b"UTF-16LE\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if (*zip).sconv.is_null() {
            return -(30 as libc::c_int);
        }
    }
    r = file_new(a, entry, &mut file);
    if r < ARCHIVE_WARN {
        if !file.is_null() {
            file_free(file);
        }
        return r;
    }
    if (*file).size == 0 as libc::c_int as libc::c_ulong && (*file).dir() != 0 {
        if __archive_rb_tree_insert_node(&mut (*zip).rbtree, file as *mut archive_rb_node) == 0 {
            /* We have already had the same file. */
            file_free(file);
            return 0 as libc::c_int;
        }
    }
    if (*file).flg & MTIME_IS_SET as libc::c_uint != 0 {
        (*zip).total_number_time_defined[MTIME as usize] =
            (*zip).total_number_time_defined[MTIME as usize].wrapping_add(1)
    }
    if (*file).flg & CTIME_IS_SET as libc::c_uint != 0 {
        (*zip).total_number_time_defined[CTIME as usize] =
            (*zip).total_number_time_defined[CTIME as usize].wrapping_add(1)
    }
    if (*file).flg & ATIME_IS_SET as libc::c_uint != 0 {
        (*zip).total_number_time_defined[ATIME as usize] =
            (*zip).total_number_time_defined[ATIME as usize].wrapping_add(1)
    }
    (*zip).total_number_entry = (*zip).total_number_entry.wrapping_add(1);
    (*zip).total_bytes_entry_name = ((*zip).total_bytes_entry_name as libc::c_ulong).wrapping_add(
        (*file)
            .name_len
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as size_t as size_t;
    if (*file).size == 0 as libc::c_int as libc::c_ulong {
        /* Count up the number of empty files. */
        (*zip).total_number_empty_entry = (*zip).total_number_empty_entry.wrapping_add(1);
        if (*file).dir() != 0 {
            (*zip).total_number_dir_entry = (*zip).total_number_dir_entry.wrapping_add(1)
        } else {
            file_register_empty(zip, file);
        }
        return r;
    }
    /*
     * Init compression.
     */
    if (*zip)
        .total_number_entry
        .wrapping_sub((*zip).total_number_empty_entry)
        == 1 as libc::c_int as libc::c_ulong
    {
        r = _7z_compression_init_encoder(a, (*zip).opt_compression, (*zip).opt_compression_level);
        if r < 0 as libc::c_int {
            file_free(file);
            return -(30 as libc::c_int);
        }
    }
    /* Register a non-empty file. */
    file_register(zip, file);
    /*
     * Set the current file to cur_file to read its contents.
     */
    (*zip).cur_file = file;
    /* Save a offset of current file in temporary file. */
    (*zip).entry_bytes_remaining = (*file).size;
    (*zip).entry_crc32 = 0 as libc::c_int as uint32_t;
    /*
     * Store a symbolic link name as file contents.
     */
    if archive_entry_filetype(entry) == AE_IFLNK as mode_t {
        let mut bytes: ssize_t = 0;
        let mut p: *const libc::c_void = archive_entry_symlink(entry) as *const libc::c_void;
        bytes = compress_out(a, p, (*file).size, ARCHIVE_Z_RUN);
        if bytes < 0 as libc::c_int as libc::c_long {
            return bytes as libc::c_int;
        }
        (*zip).entry_crc32 = crc32(
            (*zip).entry_crc32 as uLong,
            p as *const Bytef,
            bytes as libc::c_uint,
        ) as uint32_t;
        (*zip).entry_bytes_remaining = ((*zip).entry_bytes_remaining as libc::c_ulong)
            .wrapping_sub(bytes as libc::c_ulong) as uint64_t
            as uint64_t
    }
    return r;
}
/*
 * Write data to a temporary file.
 */
unsafe extern "C" fn write_to_temp(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ws: ssize_t = 0;
    zip = (*a).format_data as *mut _7zip;
    /*
     * Open a temporary file.
     */
    if (*zip).temp_fd == -(1 as libc::c_int) {
        (*zip).temp_offset = 0 as libc::c_int as uint64_t;
        (*zip).temp_fd = __archive_mktemp(NULL as *const libc::c_char);
        if (*zip).temp_fd < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Couldn\'t create temporary file\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    p = buff as *const libc::c_uchar;
    while s != 0 {
        ws = write((*zip).temp_fd, p as *const libc::c_void, s);
        if ws < 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"fwrite function failed\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        s = (s as libc::c_ulong).wrapping_sub(ws as libc::c_ulong) as size_t as size_t;
        p = p.offset(ws as isize);
        (*zip).temp_offset = ((*zip).temp_offset as libc::c_ulong).wrapping_add(ws as libc::c_ulong)
            as uint64_t as uint64_t
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compress_out(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
    mut run: la_zaction,
) -> ssize_t {
    let mut zip: *mut _7zip = (*a).format_data as *mut _7zip;
    let mut r: libc::c_int = 0;
    if run as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint
        && (*zip).stream.total_in == 0 as libc::c_int as libc::c_ulong
        && s == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as ssize_t;
    }
    if (*zip).crc32flg & PRECODE_CRC32 != 0 && s != 0 {
        (*zip).precode_crc32 = crc32(
            (*zip).precode_crc32 as uLong,
            buff as *const Bytef,
            s as libc::c_uint,
        ) as uint32_t
    }
    (*zip).stream.next_in = buff as *const libc::c_uchar;
    (*zip).stream.avail_in = s;
    loop {
        /* Compress file data. */
        r = compression_code(&mut (*a).archive, &mut (*zip).stream, run);
        if r != ARCHIVE_OK && r != ARCHIVE_EOF {
            return -(30 as libc::c_int) as ssize_t;
        }
        if (*zip).stream.avail_out == 0 as libc::c_int as libc::c_ulong {
            if write_to_temp(
                a,
                (*zip).wbuff.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong,
            ) != ARCHIVE_OK
            {
                return -(30 as libc::c_int) as ssize_t;
            }
            (*zip).stream.next_out = (*zip).wbuff.as_mut_ptr();
            (*zip).stream.avail_out =
                ::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong;
            if (*zip).crc32flg & ENCODED_CRC32 != 0 {
                (*zip).encoded_crc32 = crc32(
                    (*zip).encoded_crc32 as uLong,
                    (*zip).wbuff.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong as uInt,
                ) as uint32_t
            }
            if run as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint
                && r != ARCHIVE_EOF
            {
                continue;
            }
        }
        if (*zip).stream.avail_in == 0 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    if run as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint {
        let mut bytes: uint64_t = (::std::mem::size_of::<[libc::c_uchar; 61440]>()
            as libc::c_ulong)
            .wrapping_sub((*zip).stream.avail_out);
        if write_to_temp(a, (*zip).wbuff.as_mut_ptr() as *const libc::c_void, bytes) != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        if (*zip).crc32flg & ENCODED_CRC32 != 0 && bytes != 0 {
            (*zip).encoded_crc32 = crc32(
                (*zip).encoded_crc32 as uLong,
                (*zip).wbuff.as_mut_ptr(),
                bytes as libc::c_uint,
            ) as uint32_t
        }
    }
    return s as ssize_t;
}
unsafe extern "C" fn _7z_write_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut bytes: ssize_t = 0;
    zip = (*a).format_data as *mut _7zip;
    if s > (*zip).entry_bytes_remaining {
        s = (*zip).entry_bytes_remaining
    }
    if s == 0 as libc::c_int as libc::c_ulong || (*zip).cur_file.is_null() {
        return 0 as libc::c_int as ssize_t;
    }
    bytes = compress_out(a, buff, s, ARCHIVE_Z_RUN);
    if bytes < 0 as libc::c_int as libc::c_long {
        return bytes;
    }
    (*zip).entry_crc32 = crc32(
        (*zip).entry_crc32 as uLong,
        buff as *const Bytef,
        bytes as libc::c_uint,
    ) as uint32_t;
    (*zip).entry_bytes_remaining = ((*zip).entry_bytes_remaining as libc::c_ulong)
        .wrapping_sub(bytes as libc::c_ulong) as uint64_t
        as uint64_t;
    return bytes;
}
unsafe extern "C" fn _7z_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut s: size_t = 0;
    let mut r: ssize_t = 0;
    zip = (*a).format_data as *mut _7zip;
    if (*zip).cur_file.is_null() {
        return 0 as libc::c_int;
    }
    while (*zip).entry_bytes_remaining > 0 as libc::c_int as libc::c_ulong {
        s = (*zip).entry_bytes_remaining;
        if s > (*a).null_length {
            s = (*a).null_length
        }
        r = _7z_write_data(a, (*a).nulls as *const libc::c_void, s);
        if r < 0 as libc::c_int as libc::c_long {
            return r as libc::c_int;
        }
    }
    (*zip).total_bytes_compressed = ((*zip).total_bytes_compressed as libc::c_ulong)
        .wrapping_add((*zip).stream.total_in) as uint64_t
        as uint64_t;
    (*zip).total_bytes_uncompressed = ((*zip).total_bytes_uncompressed as libc::c_ulong)
        .wrapping_add((*zip).stream.total_out) as uint64_t
        as uint64_t;
    (*(*zip).cur_file).crc32 = (*zip).entry_crc32;
    (*zip).cur_file = NULL as *mut file;
    return 0 as libc::c_int;
}
unsafe extern "C" fn flush_wbuff(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut r: libc::c_int = 0;
    let mut s: size_t = 0;
    zip = (*a).format_data as *mut _7zip;
    s = (::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong)
        .wrapping_sub((*zip).wbuff_remaining);
    r = __archive_write_output(a, (*zip).wbuff.as_mut_ptr() as *const libc::c_void, s);
    if r != ARCHIVE_OK {
        return r;
    }
    (*zip).wbuff_remaining = ::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong;
    return r;
}
unsafe extern "C" fn copy_out(
    mut a: *mut archive_write,
    mut offset: uint64_t,
    mut length: uint64_t,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut r: libc::c_int = 0;
    zip = (*a).format_data as *mut _7zip;
    if (*zip).temp_offset > 0 as libc::c_int as libc::c_ulong
        && lseek((*zip).temp_fd, offset as __off_t, SEEK_SET) < 0 as libc::c_int as libc::c_long
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"lseek failed\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    while length != 0 {
        let mut rsize: size_t = 0;
        let mut rs: ssize_t = 0;
        let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if length > (*zip).wbuff_remaining {
            rsize = (*zip).wbuff_remaining
        } else {
            rsize = length
        }
        wb = (*zip).wbuff.as_mut_ptr().offset(
            (::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong)
                .wrapping_sub((*zip).wbuff_remaining) as isize,
        );
        rs = read((*zip).temp_fd, wb as *mut libc::c_void, rsize);
        if rs < 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t read temporary file(%jd)\x00" as *const u8 as *const libc::c_char,
                rs,
            );
            return -(30 as libc::c_int);
        }
        if rs == 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                0 as libc::c_int,
                b"Truncated 7-Zip archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*zip).wbuff_remaining = ((*zip).wbuff_remaining as libc::c_ulong)
            .wrapping_sub(rs as libc::c_ulong) as size_t as size_t;
        length =
            (length as libc::c_ulong).wrapping_sub(rs as libc::c_ulong) as uint64_t as uint64_t;
        if (*zip).wbuff_remaining == 0 as libc::c_int as libc::c_ulong {
            r = flush_wbuff(a);
            if r != ARCHIVE_OK {
                return r;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _7z_close(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut header_offset: uint64_t = 0;
    let mut header_size: uint64_t = 0;
    let mut header_unpacksize: uint64_t = 0;
    let mut length: uint64_t = 0;
    let mut header_crc32: uint32_t = 0;
    let mut r: libc::c_int = 0;
    zip = (*a).format_data as *mut _7zip;
    if (*zip).total_number_entry > 0 as libc::c_int as libc::c_ulong {
        let mut n: *mut archive_rb_node = 0 as *mut archive_rb_node;
        let mut data_offset: uint64_t = 0;
        let mut data_size: uint64_t = 0;
        let mut data_unpacksize: uint64_t = 0;
        let mut header_compression: libc::c_uint = 0;
        r = compress_out(
            a,
            NULL as *const libc::c_void,
            0 as libc::c_int as size_t,
            ARCHIVE_Z_FINISH,
        ) as libc::c_int;
        if r < 0 as libc::c_int {
            return r;
        }
        data_offset = 0 as libc::c_int as uint64_t;
        data_size = (*zip).stream.total_out;
        data_unpacksize = (*zip).stream.total_in;
        (*zip).coder.codec = (*zip).opt_compression;
        (*zip).coder.prop_size = (*zip).stream.prop_size as size_t;
        (*zip).coder.props = (*zip).stream.props;
        (*zip).stream.prop_size = 0 as libc::c_int as uint32_t;
        (*zip).stream.props = NULL as *mut uint8_t;
        (*zip).total_number_nonempty_entry = (*zip)
            .total_number_entry
            .wrapping_sub((*zip).total_number_empty_entry);
        /* Connect an empty file list. */
        if !(*zip).empty_list.first.is_null() {
            *(*zip).file_list.last = (*zip).empty_list.first;
            (*zip).file_list.last = (*zip).empty_list.last
        }
        /* Connect a directory file list. */
        n = __archive_rb_tree_iterate(
            &mut (*zip).rbtree,
            NULL as *mut archive_rb_node,
            ARCHIVE_RB_DIR_LEFT as libc::c_uint,
        );
        while !n.is_null() {
            file_register(zip, n as *mut file);
            n = __archive_rb_tree_iterate(
                &mut (*zip).rbtree,
                n,
                ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
            )
        }
        /*
         * NOTE: 7z command supports just LZMA1, LZMA2 and COPY for
         * the compression type for encoding the header.
         */
        header_compression = _7Z_COPY as libc::c_uint;
        r = _7z_compression_init_encoder(a, header_compression, 6 as libc::c_int);
        if r < 0 as libc::c_int {
            return r;
        }
        (*zip).crc32flg = PRECODE_CRC32;
        (*zip).precode_crc32 = 0 as libc::c_int as uint32_t;
        r = make_header(
            a,
            data_offset,
            data_size,
            data_unpacksize,
            1 as libc::c_int,
            &mut (*zip).coder,
        );
        if r < 0 as libc::c_int {
            return r;
        }
        r = compress_out(
            a,
            NULL as *const libc::c_void,
            0 as libc::c_int as size_t,
            ARCHIVE_Z_FINISH,
        ) as libc::c_int;
        if r < 0 as libc::c_int {
            return r;
        }
        header_offset = data_offset.wrapping_add(data_size);
        header_size = (*zip).stream.total_out;
        header_crc32 = (*zip).precode_crc32;
        header_unpacksize = (*zip).stream.total_in;
        if header_compression != _7Z_COPY as libc::c_uint {
            /*
             * Encode the header in order to reduce the size
             * of the archive.
             */
            free((*zip).coder.props as *mut libc::c_void);
            (*zip).coder.codec = header_compression;
            (*zip).coder.prop_size = (*zip).stream.prop_size as size_t;
            (*zip).coder.props = (*zip).stream.props;
            (*zip).stream.prop_size = 0 as libc::c_int as uint32_t;
            (*zip).stream.props = NULL as *mut uint8_t;
            r = _7z_compression_init_encoder(a, _7Z_COPY as libc::c_uint, 0 as libc::c_int);
            if r < 0 as libc::c_int {
                return r;
            }
            (*zip).crc32flg = ENCODED_CRC32;
            (*zip).encoded_crc32 = 0 as libc::c_int as uint32_t;
            /*
             * Make EncodedHeader.
             */
            r = enc_uint64(a, kEncodedHeader as uint64_t);
            if r < 0 as libc::c_int {
                return r;
            }
            r = make_streamsInfo(
                a,
                header_offset,
                header_size,
                header_unpacksize,
                1 as libc::c_int,
                &mut (*zip).coder,
                0 as libc::c_int,
                header_crc32,
            );
            if r < 0 as libc::c_int {
                return r;
            }
            r = compress_out(
                a,
                NULL as *const libc::c_void,
                0 as libc::c_int as size_t,
                ARCHIVE_Z_FINISH,
            ) as libc::c_int;
            if r < 0 as libc::c_int {
                return r;
            }
            header_offset = header_offset.wrapping_add(header_size);
            header_size = (*zip).stream.total_out;
            header_crc32 = (*zip).encoded_crc32
        }
        (*zip).crc32flg = 0 as libc::c_int
    } else {
        header_size = 0 as libc::c_int as uint64_t;
        header_offset = header_size;
        header_crc32 = 0 as libc::c_int as uint32_t
    }
    length = (*zip).temp_offset;
    /*
     * Make the zip header on wbuff(write buffer).
     */
    wb = (*zip).wbuff.as_mut_ptr(); /* Major version. */
    (*zip).wbuff_remaining = ::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong; /* Minor version. */
    memcpy(
        &mut *wb.offset(0 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        b"7z\xbc\xaf\'\x1c\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ); /* Next Header Offset */
    *wb.offset(6 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar; /* Next Header Size */
    *wb.offset(7 as libc::c_int as isize) = 3 as libc::c_int as libc::c_uchar; /* Next Header CRC */
    archive_le64enc(
        &mut *wb.offset(12 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        header_offset,
    ); /* Start Header CRC */
    archive_le64enc(
        &mut *wb.offset(20 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        header_size,
    );
    archive_le32enc(
        &mut *wb.offset(28 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        header_crc32,
    );
    archive_le32enc(
        &mut *wb.offset(8 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        crc32(
            0 as libc::c_int as uLong,
            &mut *wb.offset(12 as libc::c_int as isize),
            20 as libc::c_int as uInt,
        ) as uint32_t,
    );
    (*zip).wbuff_remaining = ((*zip).wbuff_remaining as libc::c_ulong)
        .wrapping_sub(32 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    /*
     * Read all file contents and an encoded header from the temporary
     * file and write out it.
     */
    r = copy_out(a, 0 as libc::c_int as uint64_t, length);
    if r != ARCHIVE_OK {
        return r;
    }
    r = flush_wbuff(a);
    return r;
}
/*
 * Encode 64 bits value into 7-Zip's encoded UINT64 value.
 */
unsafe extern "C" fn enc_uint64(mut a: *mut archive_write, mut val: uint64_t) -> libc::c_int {
    let mut mask: libc::c_uint = 0x80 as libc::c_int as libc::c_uint;
    let mut numdata: [uint8_t; 9] = [0; 9];
    let mut i: libc::c_int = 0;
    numdata[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    i = 1 as libc::c_int;
    while i < ::std::mem::size_of::<[uint8_t; 9]>() as libc::c_ulong as libc::c_int {
        if val < mask as libc::c_ulong {
            numdata[0 as libc::c_int as usize] = (numdata[0 as libc::c_int as usize] as libc::c_int
                | val as uint8_t as libc::c_int)
                as uint8_t;
            break;
        } else {
            numdata[i as usize] = val as uint8_t;
            val >>= 8 as libc::c_int;
            numdata[0 as libc::c_int as usize] =
                (numdata[0 as libc::c_int as usize] as libc::c_uint | mask) as uint8_t;
            mask >>= 1 as libc::c_int;
            i += 1
        }
    }
    return compress_out(
        a,
        numdata.as_mut_ptr() as *const libc::c_void,
        i as size_t,
        ARCHIVE_Z_RUN,
    ) as libc::c_int;
}
unsafe extern "C" fn make_substreamsInfo(
    mut a: *mut archive_write,
    mut coders: *mut coder,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*a).format_data as *mut _7zip;
    let mut file: *mut file = 0 as *mut file;
    let mut r: libc::c_int = 0;
    /*
     * Make SubStreamsInfo.
     */
    r = enc_uint64(a, kSubStreamsInfo as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    if (*zip).total_number_nonempty_entry > 1 as libc::c_int as libc::c_ulong
        && (*coders).codec != _7Z_COPY as libc::c_uint
    {
        /*
         * Make NumUnPackStream.
         */
        r = enc_uint64(a, kNumUnPackStream as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        /* Write numUnpackStreams */
        r = enc_uint64(a, (*zip).total_number_nonempty_entry);
        if r < 0 as libc::c_int {
            return r;
        }
        /*
         * Make kSize.
         */
        r = enc_uint64(a, kSize as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        file = (*zip).file_list.first;
        while !file.is_null() {
            if (*file).next.is_null() || (*(*file).next).size == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            r = enc_uint64(a, (*file).size);
            if r < 0 as libc::c_int {
                return r;
            }
            file = (*file).next
        }
    }
    /*
     * Make CRC.
     */
    r = enc_uint64(a, kCRC as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* All are defined */
    r = enc_uint64(a, 1 as libc::c_int as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    file = (*zip).file_list.first;
    while !file.is_null() {
        let mut crc: [uint8_t; 4] = [0; 4];
        if (*file).size == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        archive_le32enc(crc.as_mut_ptr() as *mut libc::c_void, (*file).crc32);
        r = compress_out(
            a,
            crc.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as size_t,
            ARCHIVE_Z_RUN,
        ) as libc::c_int;
        if r < 0 as libc::c_int {
            return r;
        }
        file = (*file).next
    }
    /* Write End. */
    r = enc_uint64(a, kEnd as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn make_streamsInfo(
    mut a: *mut archive_write,
    mut offset: uint64_t,
    mut pack_size: uint64_t,
    mut unpack_size: uint64_t,
    mut num_coder: libc::c_int,
    mut coders: *mut coder,
    mut substrm: libc::c_int,
    mut header_crc: uint32_t,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*a).format_data as *mut _7zip;
    let mut codec_buff: [uint8_t; 8] = [0; 8];
    let mut numFolders: libc::c_int = 0;
    let mut fi: libc::c_int = 0;
    let mut codec_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if (*coders).codec == _7Z_COPY as libc::c_uint {
        numFolders = (*zip).total_number_nonempty_entry as libc::c_int
    } else {
        numFolders = 1 as libc::c_int
    }
    /*
     * Make PackInfo.
     */
    r = enc_uint64(a, kPackInfo as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write PackPos. */
    r = enc_uint64(a, offset);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write NumPackStreams. */
    r = enc_uint64(a, numFolders as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Make Size. */
    r = enc_uint64(a, kSize as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    if numFolders > 1 as libc::c_int {
        let mut file: *mut file = (*zip).file_list.first;
        while !file.is_null() {
            if (*file).size == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            r = enc_uint64(a, (*file).size);
            if r < 0 as libc::c_int {
                return r;
            }
            file = (*file).next
        }
    } else {
        /* Write size. */
        r = enc_uint64(a, pack_size);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    r = enc_uint64(a, kEnd as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /*
     * Make UnPackInfo.
     */
    r = enc_uint64(a, kUnPackInfo as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /*
     * Make Folder.
     */
    r = enc_uint64(a, kFolder as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write NumFolders. */
    r = enc_uint64(a, numFolders as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write External. */
    r = enc_uint64(a, 0 as libc::c_int as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    fi = 0 as libc::c_int;
    while fi < numFolders {
        /* Write NumCoders. */
        r = enc_uint64(a, num_coder as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        i = 0 as libc::c_int;
        while i < num_coder {
            let mut codec_id: libc::c_uint = (*coders.offset(i as isize)).codec;
            /* Write Codec flag. */
            archive_be64enc(
                codec_buff.as_mut_ptr() as *mut libc::c_void,
                codec_id as uint64_t,
            );
            codec_size = 8 as libc::c_int;
            while codec_size > 0 as libc::c_int {
                if codec_buff[(8 as libc::c_int - codec_size) as usize] != 0 {
                    break;
                }
                codec_size -= 1
            }
            if codec_size == 0 as libc::c_int {
                codec_size = 1 as libc::c_int
            }
            if (*coders.offset(i as isize)).prop_size != 0 {
                r = enc_uint64(a, (codec_size | 0x20 as libc::c_int) as uint64_t)
            } else {
                r = enc_uint64(a, codec_size as uint64_t)
            }
            if r < 0 as libc::c_int {
                return r;
            }
            /* Write Codec ID. */
            codec_size &= 0xf as libc::c_int;
            r = compress_out(
                a,
                &mut *codec_buff
                    .as_mut_ptr()
                    .offset((8 as libc::c_int - codec_size) as isize)
                    as *mut uint8_t as *const libc::c_void,
                codec_size as size_t,
                ARCHIVE_Z_RUN,
            ) as libc::c_int;
            if r < 0 as libc::c_int {
                return r;
            }
            if (*coders.offset(i as isize)).prop_size != 0 {
                /* Write Codec property size. */
                r = enc_uint64(a, (*coders.offset(i as isize)).prop_size);
                if r < 0 as libc::c_int {
                    return r;
                }
                /* Write Codec properties. */
                r = compress_out(
                    a,
                    (*coders.offset(i as isize)).props as *const libc::c_void,
                    (*coders.offset(i as isize)).prop_size,
                    ARCHIVE_Z_RUN,
                ) as libc::c_int;
                if r < 0 as libc::c_int {
                    return r;
                }
            }
            i += 1
        }
        fi += 1
    }
    /*
     * Make CodersUnPackSize.
     */
    r = enc_uint64(a, kCodersUnPackSize as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    if numFolders > 1 as libc::c_int {
        let mut file_0: *mut file = (*zip).file_list.first;
        while !file_0.is_null() {
            if (*file_0).size == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            r = enc_uint64(a, (*file_0).size);
            if r < 0 as libc::c_int {
                return r;
            }
            file_0 = (*file_0).next
        }
    } else {
        /* Write UnPackSize. */
        r = enc_uint64(a, unpack_size);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    if substrm == 0 {
        let mut crc: [uint8_t; 4] = [0; 4];
        /*
         * Make CRC.
         */
        r = enc_uint64(a, kCRC as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        /* All are defined */
        r = enc_uint64(a, 1 as libc::c_int as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        archive_le32enc(crc.as_mut_ptr() as *mut libc::c_void, header_crc);
        r = compress_out(
            a,
            crc.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as size_t,
            ARCHIVE_Z_RUN,
        ) as libc::c_int;
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /* Write End. */
    r = enc_uint64(a, kEnd as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    if substrm != 0 {
        /*
         * Make SubStreamsInfo.
         */
        r = make_substreamsInfo(a, coders);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /* Write End. */
    r = enc_uint64(a, kEnd as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn utcToFiletime(mut t: time_t, mut ns: libc::c_long) -> uint64_t {
    let mut fileTime: uint64_t = 0;
    fileTime = t as uint64_t;
    fileTime = (fileTime as libc::c_ulong).wrapping_mul(10000000 as libc::c_int as libc::c_ulong)
        as uint64_t as uint64_t;
    fileTime = (fileTime as libc::c_ulong)
        .wrapping_add((ns / 100 as libc::c_int as libc::c_long) as libc::c_ulong)
        as uint64_t as uint64_t;
    fileTime = (fileTime as libc::c_ulonglong).wrapping_add(116444736000000000 as libc::c_ulonglong)
        as uint64_t as uint64_t;
    return fileTime;
}
unsafe extern "C" fn make_time(
    mut a: *mut archive_write,
    mut type_0: uint8_t,
    mut flg: libc::c_uint,
    mut ti: libc::c_int,
) -> libc::c_int {
    let mut filetime: [uint8_t; 8] = [0; 8];
    let mut zip: *mut _7zip = (*a).format_data as *mut _7zip;
    let mut file: *mut file = 0 as *mut file;
    let mut r: libc::c_int = 0;
    let mut b: uint8_t = 0;
    let mut mask: uint8_t = 0;
    /*
     * Make Time Bools.
     */
    if (*zip).total_number_time_defined[ti as usize] == (*zip).total_number_entry {
        /* Write Time Type. */
        r = enc_uint64(a, type_0 as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        /* Write EmptyStream Size. */
        r = enc_uint64(
            a,
            (2 as libc::c_int as libc::c_ulong).wrapping_add(
                (*zip)
                    .total_number_entry
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong),
            ),
        );
        if r < 0 as libc::c_int {
            return r;
        }
        /* All are defined. */
        r = enc_uint64(a, 1 as libc::c_int as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
    } else {
        if (*zip).total_number_time_defined[ti as usize] == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        /* Write Time Type. */
        r = enc_uint64(a, type_0 as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        /* Write EmptyStream Size. */
        r = enc_uint64(
            a,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (*zip)
                        .total_number_entry
                        .wrapping_add(7 as libc::c_int as libc::c_ulong)
                        >> 3 as libc::c_int,
                )
                .wrapping_add(
                    (*zip).total_number_time_defined[ti as usize]
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                ),
        );
        if r < 0 as libc::c_int {
            return r;
        }
        /* All are not defined. */
        r = enc_uint64(a, 0 as libc::c_int as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        b = 0 as libc::c_int as uint8_t;
        mask = 0x80 as libc::c_int as uint8_t;
        file = (*zip).file_list.first;
        while !file.is_null() {
            if (*file).flg & flg != 0 {
                b = (b as libc::c_int | mask as libc::c_int) as uint8_t
            }
            mask = (mask as libc::c_int >> 1 as libc::c_int) as uint8_t;
            if mask as libc::c_int == 0 as libc::c_int {
                r = compress_out(
                    a,
                    &mut b as *mut uint8_t as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    ARCHIVE_Z_RUN,
                ) as libc::c_int;
                if r < 0 as libc::c_int {
                    return r;
                }
                mask = 0x80 as libc::c_int as uint8_t;
                b = 0 as libc::c_int as uint8_t
            }
            file = (*file).next
        }
        if mask as libc::c_int != 0x80 as libc::c_int {
            r = compress_out(
                a,
                &mut b as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as size_t,
                ARCHIVE_Z_RUN,
            ) as libc::c_int;
            if r < 0 as libc::c_int {
                return r;
            }
        }
    }
    /* External. */
    r = enc_uint64(a, 0 as libc::c_int as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /*
     * Make Times.
     */
    file = (*zip).file_list.first;
    while !file.is_null() {
        if !((*file).flg & flg == 0 as libc::c_int as libc::c_uint) {
            archive_le64enc(
                filetime.as_mut_ptr() as *mut libc::c_void,
                utcToFiletime(
                    (*file).times[ti as usize].time,
                    (*file).times[ti as usize].time_ns,
                ),
            );
            r = compress_out(
                a,
                filetime.as_mut_ptr() as *const libc::c_void,
                8 as libc::c_int as size_t,
                ARCHIVE_Z_RUN,
            ) as libc::c_int;
            if r < 0 as libc::c_int {
                return r;
            }
        }
        file = (*file).next
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn make_header(
    mut a: *mut archive_write,
    mut offset: uint64_t,
    mut pack_size: uint64_t,
    mut unpack_size: uint64_t,
    mut codernum: libc::c_int,
    mut coders: *mut coder,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*a).format_data as *mut _7zip;
    let mut file: *mut file = 0 as *mut file;
    let mut r: libc::c_int = 0;
    let mut b: uint8_t = 0;
    let mut mask: uint8_t = 0;
    /*
     * Make FilesInfo.
     */
    r = enc_uint64(a, kHeader as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /*
     * If there are empty files only, do not write MainStreamInfo.
     */
    if (*zip).total_number_nonempty_entry != 0 {
        /*
         * Make MainStreamInfo.
         */
        r = enc_uint64(a, kMainStreamsInfo as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        r = make_streamsInfo(
            a,
            offset,
            pack_size,
            unpack_size,
            codernum,
            coders,
            1 as libc::c_int,
            0 as libc::c_int as uint32_t,
        );
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /*
     * Make FilesInfo.
     */
    r = enc_uint64(a, kFilesInfo as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write numFiles. */
    r = enc_uint64(a, (*zip).total_number_entry);
    if r < 0 as libc::c_int {
        return r;
    }
    if (*zip).total_number_empty_entry > 0 as libc::c_int as libc::c_ulong {
        /* Make EmptyStream. */
        r = enc_uint64(a, kEmptyStream as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        /* Write EmptyStream Size. */
        r = enc_uint64(
            a,
            (*zip)
                .total_number_entry
                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                >> 3 as libc::c_int,
        );
        if r < 0 as libc::c_int {
            return r;
        }
        b = 0 as libc::c_int as uint8_t;
        mask = 0x80 as libc::c_int as uint8_t;
        file = (*zip).file_list.first;
        while !file.is_null() {
            if (*file).size == 0 as libc::c_int as libc::c_ulong {
                b = (b as libc::c_int | mask as libc::c_int) as uint8_t
            }
            mask = (mask as libc::c_int >> 1 as libc::c_int) as uint8_t;
            if mask as libc::c_int == 0 as libc::c_int {
                r = compress_out(
                    a,
                    &mut b as *mut uint8_t as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    ARCHIVE_Z_RUN,
                ) as libc::c_int;
                if r < 0 as libc::c_int {
                    return r;
                }
                mask = 0x80 as libc::c_int as uint8_t;
                b = 0 as libc::c_int as uint8_t
            }
            file = (*file).next
        }
        if mask as libc::c_int != 0x80 as libc::c_int {
            r = compress_out(
                a,
                &mut b as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as size_t,
                ARCHIVE_Z_RUN,
            ) as libc::c_int;
            if r < 0 as libc::c_int {
                return r;
            }
        }
    }
    if (*zip).total_number_empty_entry > (*zip).total_number_dir_entry {
        /* Make EmptyFile. */
        r = enc_uint64(a, kEmptyFile as uint64_t);
        if r < 0 as libc::c_int {
            return r;
        }
        /* Write EmptyFile Size. */
        r = enc_uint64(
            a,
            (*zip)
                .total_number_empty_entry
                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                >> 3 as libc::c_int,
        );
        if r < 0 as libc::c_int {
            return r;
        }
        b = 0 as libc::c_int as uint8_t;
        mask = 0x80 as libc::c_int as uint8_t;
        file = (*zip).file_list.first;
        while !file.is_null() {
            if !((*file).size != 0) {
                if (*file).dir() == 0 {
                    b = (b as libc::c_int | mask as libc::c_int) as uint8_t
                }
                mask = (mask as libc::c_int >> 1 as libc::c_int) as uint8_t;
                if mask as libc::c_int == 0 as libc::c_int {
                    r = compress_out(
                        a,
                        &mut b as *mut uint8_t as *const libc::c_void,
                        1 as libc::c_int as size_t,
                        ARCHIVE_Z_RUN,
                    ) as libc::c_int;
                    if r < 0 as libc::c_int {
                        return r;
                    }
                    mask = 0x80 as libc::c_int as uint8_t;
                    b = 0 as libc::c_int as uint8_t
                }
            }
            file = (*file).next
        }
        if mask as libc::c_int != 0x80 as libc::c_int {
            r = compress_out(
                a,
                &mut b as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as size_t,
                ARCHIVE_Z_RUN,
            ) as libc::c_int;
            if r < 0 as libc::c_int {
                return r;
            }
        }
    }
    /* Make Name. */
    r = enc_uint64(a, kName as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write Name size. */
    r = enc_uint64(
        a,
        (*zip)
            .total_bytes_entry_name
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write dmy byte. */
    r = enc_uint64(a, 0 as libc::c_int as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    file = (*zip).file_list.first;
    while !file.is_null() {
        r = compress_out(
            a,
            (*file).utf16name as *const libc::c_void,
            (*file)
                .name_len
                .wrapping_add(2 as libc::c_int as libc::c_uint) as size_t,
            ARCHIVE_Z_RUN,
        ) as libc::c_int;
        if r < 0 as libc::c_int {
            return r;
        }
        file = (*file).next
    }
    /* Make MTime. */
    r = make_time(a, kMTime as uint8_t, MTIME_IS_SET as libc::c_uint, MTIME);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Make CTime. */
    r = make_time(a, kCTime as uint8_t, CTIME_IS_SET as libc::c_uint, CTIME);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Make ATime. */
    r = make_time(a, kATime as uint8_t, ATIME_IS_SET as libc::c_uint, ATIME);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Make Attributes. */
    r = enc_uint64(a, kAttributes as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write Attributes size. */
    r = enc_uint64(
        a,
        (2 as libc::c_int as libc::c_ulong).wrapping_add(
            (*zip)
                .total_number_entry
                .wrapping_mul(4 as libc::c_int as libc::c_ulong),
        ),
    );
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write "All Are Defined". */
    r = enc_uint64(a, 1 as libc::c_int as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write dmy byte. */
    r = enc_uint64(a, 0 as libc::c_int as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    file = (*zip).file_list.first;
    while !file.is_null() {
        /*
         * High 16bits is unix mode.
         * Low 16bits is Windows attributes.
         */
        let mut encattr: uint32_t = 0; /* Read Only. */
        let mut attr: uint32_t = 0;
        if (*file).dir() != 0 {
            attr = 0x8010 as libc::c_int as uint32_t
        } else {
            attr = 0x8020 as libc::c_int as uint32_t
        }
        if (*file).mode & 0o222 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            attr |= 1 as libc::c_int as libc::c_uint
        }
        attr |= (*file).mode << 16 as libc::c_int;
        archive_le32enc(&mut encattr as *mut uint32_t as *mut libc::c_void, attr);
        r = compress_out(
            a,
            &mut encattr as *mut uint32_t as *const libc::c_void,
            4 as libc::c_int as size_t,
            ARCHIVE_Z_RUN,
        ) as libc::c_int;
        if r < 0 as libc::c_int {
            return r;
        }
        file = (*file).next
    }
    /* Write End. */
    r = enc_uint64(a, kEnd as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    /* Write End. */
    r = enc_uint64(a, kEnd as uint64_t);
    if r < 0 as libc::c_int {
        return r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _7z_free(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut _7zip = (*a).format_data as *mut _7zip;
    /* Close the temporary file. */
    if (*zip).temp_fd >= 0 as libc::c_int {
        close((*zip).temp_fd);
    }
    file_free_register(zip);
    compression_end(&mut (*a).archive, &mut (*zip).stream);
    free((*zip).coder.props as *mut libc::c_void);
    free(zip as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut f1: *const file = n1 as *const file;
    let mut f2: *const file = n2 as *const file;
    if (*f1).name_len == (*f2).name_len {
        return memcmp(
            (*f1).utf16name as *const libc::c_void,
            (*f2).utf16name as *const libc::c_void,
            (*f1).name_len as libc::c_ulong,
        );
    }
    return if (*f1).name_len > (*f2).name_len {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn file_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut f: *const file = n as *const file;
    return (*f)
        .name_len
        .wrapping_sub(*(key as *const libc::c_char) as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn file_new(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut newfile: *mut *mut file,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut file: *mut file = 0 as *mut file;
    let mut u16: *const libc::c_char = 0 as *const libc::c_char;
    let mut u16len: size_t = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    zip = (*a).format_data as *mut _7zip;
    *newfile = NULL as *mut file;
    file = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<file>() as libc::c_ulong,
    ) as *mut file;
    if file.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if 0 as libc::c_int > _archive_entry_pathname_l(entry, &mut u16, &mut u16len, (*zip).sconv) {
        if errno == ENOMEM {
            free(file as *mut libc::c_void);
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for UTF-16LE\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(&mut (*a).archive as *mut archive,
                          ARCHIVE_ERRNO_MISC,
                          b"A filename cannot be converted to UTF-16LE;You should disable making Joliet extension\x00"
                              as *const u8 as *const libc::c_char);
        ret = ARCHIVE_WARN
    }
    (*file).utf16name =
        malloc(u16len.wrapping_add(2 as libc::c_int as libc::c_ulong)) as *mut uint8_t;
    if (*file).utf16name.is_null() {
        free(file as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for Name\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    memcpy(
        (*file).utf16name as *mut libc::c_void,
        u16 as *const libc::c_void,
        u16len,
    );
    *(*file)
        .utf16name
        .offset(u16len.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize) =
        0 as libc::c_int as uint8_t;
    *(*file)
        .utf16name
        .offset(u16len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) =
        0 as libc::c_int as uint8_t;
    (*file).name_len = u16len as libc::c_uint;
    (*file).mode = archive_entry_mode(entry);
    if archive_entry_filetype(entry) == AE_IFREG as mode_t {
        (*file).size = archive_entry_size(entry) as uint64_t
    } else {
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
    }
    if archive_entry_filetype(entry) == AE_IFDIR as mode_t {
        (*file).set_dir(1 as libc::c_int)
    } else if archive_entry_filetype(entry) == AE_IFLNK as mode_t {
        (*file).size = strlen(archive_entry_symlink(entry))
    }
    if archive_entry_mtime_is_set(entry) != 0 {
        (*file).flg |= MTIME_IS_SET as libc::c_uint;
        (*file).times[MTIME as usize].time = archive_entry_mtime(entry);
        (*file).times[MTIME as usize].time_ns = archive_entry_mtime_nsec(entry)
    }
    if archive_entry_atime_is_set(entry) != 0 {
        (*file).flg |= ATIME_IS_SET as libc::c_uint;
        (*file).times[ATIME as usize].time = archive_entry_atime(entry);
        (*file).times[ATIME as usize].time_ns = archive_entry_atime_nsec(entry)
    }
    if archive_entry_ctime_is_set(entry) != 0 {
        (*file).flg |= CTIME_IS_SET as libc::c_uint;
        (*file).times[CTIME as usize].time = archive_entry_ctime(entry);
        (*file).times[CTIME as usize].time_ns = archive_entry_ctime_nsec(entry)
    }
    *newfile = file;
    return ret;
}
unsafe extern "C" fn file_free(mut file: *mut file) {
    free((*file).utf16name as *mut libc::c_void);
    free(file as *mut libc::c_void);
}
unsafe extern "C" fn file_register(mut zip: *mut _7zip, mut file: *mut file) {
    (*file).next = NULL as *mut file;
    *(*zip).file_list.last = file;
    (*zip).file_list.last = &mut (*file).next;
}
unsafe extern "C" fn file_init_register(mut zip: *mut _7zip) {
    (*zip).file_list.first = NULL as *mut file;
    (*zip).file_list.last = &mut (*zip).file_list.first;
}
unsafe extern "C" fn file_free_register(mut zip: *mut _7zip) {
    let mut file: *mut file = 0 as *mut file;
    let mut file_next: *mut file = 0 as *mut file;
    file = (*zip).file_list.first;
    while !file.is_null() {
        file_next = (*file).next;
        file_free(file);
        file = file_next
    }
}
unsafe extern "C" fn file_register_empty(mut zip: *mut _7zip, mut file: *mut file) {
    (*file).next = NULL as *mut file;
    *(*zip).empty_list.last = file;
    (*zip).empty_list.last = &mut (*file).next;
}
unsafe extern "C" fn file_init_register_empty(mut zip: *mut _7zip) {
    (*zip).empty_list.first = NULL as *mut file;
    (*zip).empty_list.last = &mut (*zip).empty_list.first;
}
unsafe extern "C" fn compression_unsupported_encoder(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut name: *const libc::c_char,
) -> libc::c_int {
    archive_set_error(
        a,
        ARCHIVE_ERRNO_MISC,
        b"%s compression not supported on this platform\x00" as *const u8 as *const libc::c_char,
        name,
    );
    (*lastrm).valid = 0 as libc::c_int;
    (*lastrm).real_stream = NULL as *mut libc::c_void;
    return -(25 as libc::c_int);
}
/*
 * _7_COPY compressor.
 */
unsafe extern "C" fn compression_init_encoder_copy(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
) -> libc::c_int {
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    (*lastrm).valid = 1 as libc::c_int;
    (*lastrm).code = Some(
        compression_code_copy
            as unsafe extern "C" fn(
                _: *mut archive,
                _: *mut la_zstream,
                _: la_zaction,
            ) -> libc::c_int,
    );
    (*lastrm).end = Some(
        compression_end_copy
            as unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_code_copy(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut action: la_zaction,
) -> libc::c_int {
    let mut bytes: size_t = 0;
    /* UNUSED */
    if (*lastrm).avail_out > (*lastrm).avail_in {
        bytes = (*lastrm).avail_in
    } else {
        bytes = (*lastrm).avail_out
    }
    if bytes != 0 {
        memcpy(
            (*lastrm).next_out as *mut libc::c_void,
            (*lastrm).next_in as *const libc::c_void,
            bytes,
        );
        (*lastrm).next_in = (*lastrm).next_in.offset(bytes as isize);
        (*lastrm).avail_in =
            ((*lastrm).avail_in as libc::c_ulong).wrapping_sub(bytes) as size_t as size_t;
        (*lastrm).total_in =
            ((*lastrm).total_in as libc::c_ulong).wrapping_add(bytes) as uint64_t as uint64_t;
        (*lastrm).next_out = (*lastrm).next_out.offset(bytes as isize);
        (*lastrm).avail_out =
            ((*lastrm).avail_out as libc::c_ulong).wrapping_sub(bytes) as size_t as size_t;
        (*lastrm).total_out =
            ((*lastrm).total_out as libc::c_ulong).wrapping_add(bytes) as uint64_t as uint64_t
    }
    if action as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint
        && (*lastrm).avail_in == 0 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_end_copy(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
) -> libc::c_int {
    /* UNUSED */
    (*lastrm).valid = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/*
 * _7_DEFLATE compressor.
 */
unsafe extern "C" fn compression_init_encoder_deflate(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
    mut withheader: libc::c_int,
) -> libc::c_int {
    let mut strm: *mut z_stream = 0 as *mut z_stream;
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    strm = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong,
    ) as *mut z_stream;
    if strm.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"Can\'t allocate memory for gzip stream\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* zlib.h is not const-correct, so we need this one bit
     * of ugly hackery to convert a const * pointer to
     * a non-const pointer. */
    (*strm).next_in = (*lastrm).next_in as *const libc::c_void as uintptr_t as *mut Bytef;
    (*strm).avail_in = (*lastrm).avail_in as uInt;
    (*strm).total_in = (*lastrm).total_in;
    (*strm).next_out = (*lastrm).next_out;
    (*strm).avail_out = (*lastrm).avail_out as uInt;
    (*strm).total_out = (*lastrm).total_out;
    if deflateInit2_(
        strm,
        level,
        8 as libc::c_int,
        (if withheader != 0 {
            15 as libc::c_int
        } else {
            -(15 as libc::c_int)
        }),
        8 as libc::c_int,
        0 as libc::c_int,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    ) != Z_OK
    {
        free(strm as *mut libc::c_void);
        (*lastrm).real_stream = NULL as *mut libc::c_void;
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Internal error initializing compression library\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*lastrm).real_stream = strm as *mut libc::c_void;
    (*lastrm).valid = 1 as libc::c_int;
    (*lastrm).code = Some(
        compression_code_deflate
            as unsafe extern "C" fn(
                _: *mut archive,
                _: *mut la_zstream,
                _: la_zaction,
            ) -> libc::c_int,
    );
    (*lastrm).end = Some(
        compression_end_deflate
            as unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_code_deflate(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut action: la_zaction,
) -> libc::c_int {
    let mut strm: *mut z_stream = 0 as *mut z_stream;
    let mut r: libc::c_int = 0;
    strm = (*lastrm).real_stream as *mut z_stream;
    /* zlib.h is not const-correct, so we need this one bit
     * of ugly hackery to convert a const * pointer to
     * a non-const pointer. */
    (*strm).next_in = (*lastrm).next_in as *const libc::c_void as uintptr_t as *mut Bytef;
    (*strm).avail_in = (*lastrm).avail_in as uInt;
    (*strm).total_in = (*lastrm).total_in;
    (*strm).next_out = (*lastrm).next_out;
    (*strm).avail_out = (*lastrm).avail_out as uInt;
    (*strm).total_out = (*lastrm).total_out;
    r = deflate(
        strm,
        if action as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint {
            Z_FINISH
        } else {
            Z_NO_FLUSH
        },
    );
    (*lastrm).next_in = (*strm).next_in;
    (*lastrm).avail_in = (*strm).avail_in as size_t;
    (*lastrm).total_in = (*strm).total_in;
    (*lastrm).next_out = (*strm).next_out;
    (*lastrm).avail_out = (*strm).avail_out as size_t;
    (*lastrm).total_out = (*strm).total_out;
    match r {
        Z_OK => return 0 as libc::c_int,
        Z_STREAM_END => return 1 as libc::c_int,
        _ => {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_MISC,
                b"GZip compression failed: deflate() call returned status %d\x00" as *const u8
                    as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    };
}
unsafe extern "C" fn compression_end_deflate(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
) -> libc::c_int {
    let mut strm: *mut z_stream = 0 as *mut z_stream;
    let mut r: libc::c_int = 0;
    strm = (*lastrm).real_stream as *mut z_stream;
    r = deflateEnd(strm);
    free(strm as *mut libc::c_void);
    (*lastrm).real_stream = NULL as *mut libc::c_void;
    (*lastrm).valid = 0 as libc::c_int;
    if r != Z_OK {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Failed to clean up compressor\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/*
 * _7_BZIP2 compressor.
 */
unsafe extern "C" fn compression_init_encoder_bzip2(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    return compression_unsupported_encoder(
        a,
        lastrm,
        b"bzip2\x00" as *const u8 as *const libc::c_char,
    );
}
/*
 * _7_LZMA1, _7_LZMA2 compressor.
 */
unsafe extern "C" fn compression_init_encoder_lzma1(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    return compression_unsupported_encoder(
        a,
        lastrm,
        b"lzma\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn compression_init_encoder_lzma2(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    return compression_unsupported_encoder(
        a,
        lastrm,
        b"lzma\x00" as *const u8 as *const libc::c_char,
    );
}
/*
 * _7_PPMD compressor.
 */
unsafe extern "C" fn ppmd_write(mut p: *mut libc::c_void, mut b: Byte) {
    let mut a: *mut archive_write = (*(p as *mut IByteOut)).a;
    let mut zip: *mut _7zip = (*a).format_data as *mut _7zip;
    let mut lastrm: *mut la_zstream = &mut (*zip).stream;
    let mut strm: *mut ppmd_stream = 0 as *mut ppmd_stream;
    if (*lastrm).avail_out != 0 {
        let fresh0 = (*lastrm).next_out;
        (*lastrm).next_out = (*lastrm).next_out.offset(1);
        *fresh0 = b;
        (*lastrm).avail_out = (*lastrm).avail_out.wrapping_sub(1);
        (*lastrm).total_out = (*lastrm).total_out.wrapping_add(1);
        return;
    }
    strm = (*lastrm).real_stream as *mut ppmd_stream;
    if (*strm).buff_ptr < (*strm).buff_end {
        let fresh1 = (*strm).buff_ptr;
        (*strm).buff_ptr = (*strm).buff_ptr.offset(1);
        *fresh1 = b;
        (*strm).buff_bytes = (*strm).buff_bytes.wrapping_add(1)
    };
}
unsafe extern "C" fn compression_init_encoder_ppmd(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut maxOrder: libc::c_uint,
    mut msize: uint32_t,
) -> libc::c_int {
    let mut strm: *mut ppmd_stream = 0 as *mut ppmd_stream;
    let mut props: *mut uint8_t = 0 as *mut uint8_t;
    let mut r: libc::c_int = 0;
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    strm = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ppmd_stream>() as libc::c_ulong,
    ) as *mut ppmd_stream;
    if strm.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"Can\'t allocate memory for PPMd\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*strm).buff = malloc(32 as libc::c_int as libc::c_ulong) as *mut uint8_t;
    if (*strm).buff.is_null() {
        free(strm as *mut libc::c_void);
        archive_set_error(
            a,
            ENOMEM,
            b"Can\'t allocate memory for PPMd\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*strm).buff_ptr = (*strm).buff;
    (*strm).buff_end = (*strm).buff.offset(32 as libc::c_int as isize);
    props = malloc((1 as libc::c_int + 4 as libc::c_int) as libc::c_ulong) as *mut uint8_t;
    if props.is_null() {
        free((*strm).buff as *mut libc::c_void);
        free(strm as *mut libc::c_void);
        archive_set_error(
            a,
            ENOMEM,
            b"Coludn\'t allocate memory for PPMd\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    *props.offset(0 as libc::c_int as isize) = maxOrder as uint8_t;
    archive_le32enc(
        props.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        msize,
    );
    __archive_ppmd7_functions
        .Ppmd7_Construct
        .expect("non-null function pointer")(&mut (*strm).ppmd7_context);
    r = __archive_ppmd7_functions
        .Ppmd7_Alloc
        .expect("non-null function pointer")(&mut (*strm).ppmd7_context, msize);
    if r == 0 as libc::c_int {
        free((*strm).buff as *mut libc::c_void);
        free(strm as *mut libc::c_void);
        free(props as *mut libc::c_void);
        archive_set_error(
            a,
            ENOMEM,
            b"Coludn\'t allocate memory for PPMd\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    __archive_ppmd7_functions
        .Ppmd7_Init
        .expect("non-null function pointer")(&mut (*strm).ppmd7_context, maxOrder);
    (*strm).byteout.a = a as *mut archive_write;
    (*strm).byteout.Write =
        Some(ppmd_write as unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ());
    (*strm).range_enc.Stream = &mut (*strm).byteout;
    __archive_ppmd7_functions
        .Ppmd7z_RangeEnc_Init
        .expect("non-null function pointer")(&mut (*strm).range_enc);
    (*strm).stat = 0 as libc::c_int;
    (*lastrm).real_stream = strm as *mut libc::c_void;
    (*lastrm).valid = 1 as libc::c_int;
    (*lastrm).code = Some(
        compression_code_ppmd
            as unsafe extern "C" fn(
                _: *mut archive,
                _: *mut la_zstream,
                _: la_zaction,
            ) -> libc::c_int,
    );
    (*lastrm).end = Some(
        compression_end_ppmd
            as unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream) -> libc::c_int,
    );
    (*lastrm).prop_size = 5 as libc::c_int as uint32_t;
    (*lastrm).props = props;
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_code_ppmd(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut action: la_zaction,
) -> libc::c_int {
    let mut strm: *mut ppmd_stream = 0 as *mut ppmd_stream;
    /* UNUSED */
    strm = (*lastrm).real_stream as *mut ppmd_stream;
    /* Copy encoded data if there are remaining bytes from previous call. */
    if (*strm).buff_bytes != 0 {
        let mut p: *mut uint8_t = (*strm).buff_ptr.offset(-((*strm).buff_bytes as isize));
        while (*lastrm).avail_out != 0 && (*strm).buff_bytes != 0 {
            let fresh2 = p;
            p = p.offset(1);
            let fresh3 = (*lastrm).next_out;
            (*lastrm).next_out = (*lastrm).next_out.offset(1);
            *fresh3 = *fresh2;
            (*lastrm).avail_out = (*lastrm).avail_out.wrapping_sub(1);
            (*lastrm).total_out = (*lastrm).total_out.wrapping_add(1);
            (*strm).buff_bytes = (*strm).buff_bytes.wrapping_sub(1)
        }
        if (*strm).buff_bytes != 0 {
            return 0 as libc::c_int;
        }
        if (*strm).stat == 1 as libc::c_int {
            return 1 as libc::c_int;
        }
        (*strm).buff_ptr = (*strm).buff
    }
    while (*lastrm).avail_in != 0 && (*lastrm).avail_out != 0 {
        let fresh4 = (*lastrm).next_in;
        (*lastrm).next_in = (*lastrm).next_in.offset(1);
        __archive_ppmd7_functions
            .Ppmd7_EncodeSymbol
            .expect("non-null function pointer")(
            &mut (*strm).ppmd7_context,
            &mut (*strm).range_enc,
            *fresh4 as libc::c_int,
        );
        (*lastrm).avail_in = (*lastrm).avail_in.wrapping_sub(1);
        (*lastrm).total_in = (*lastrm).total_in.wrapping_add(1)
    }
    if (*lastrm).avail_in == 0 as libc::c_int as libc::c_ulong
        && action as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint
    {
        __archive_ppmd7_functions
            .Ppmd7z_RangeEnc_FlushData
            .expect("non-null function pointer")(&mut (*strm).range_enc);
        (*strm).stat = 1 as libc::c_int;
        /* Return EOF if there are no remaining bytes. */
        if (*strm).buff_bytes == 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_end_ppmd(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
) -> libc::c_int {
    let mut strm: *mut ppmd_stream = 0 as *mut ppmd_stream;
    /* UNUSED */
    strm = (*lastrm).real_stream as *mut ppmd_stream;
    __archive_ppmd7_functions
        .Ppmd7_Free
        .expect("non-null function pointer")(&mut (*strm).ppmd7_context);
    free((*strm).buff as *mut libc::c_void);
    free(strm as *mut libc::c_void);
    (*lastrm).real_stream = NULL as *mut libc::c_void;
    (*lastrm).valid = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/*
 * Universal compressor initializer.
 */
unsafe extern "C" fn _7z_compression_init_encoder(
    mut a: *mut archive_write,
    mut compression: libc::c_uint,
    mut compression_level: libc::c_int,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut r: libc::c_int = 0;
    zip = (*a).format_data as *mut _7zip;
    match compression {
        262408 => {
            r = compression_init_encoder_deflate(
                &mut (*a).archive,
                &mut (*zip).stream,
                compression_level,
                0 as libc::c_int,
            )
        }
        262658 => {
            r = compression_init_encoder_bzip2(
                &mut (*a).archive,
                &mut (*zip).stream,
                compression_level,
            )
        }
        196865 => {
            r = compression_init_encoder_lzma1(
                &mut (*a).archive,
                &mut (*zip).stream,
                compression_level,
            )
        }
        33 => {
            r = compression_init_encoder_lzma2(
                &mut (*a).archive,
                &mut (*zip).stream,
                compression_level,
            )
        }
        197633 => {
            r = compression_init_encoder_ppmd(
                &mut (*a).archive,
                &mut (*zip).stream,
                PPMD7_DEFAULT_ORDER as libc::c_uint,
                PPMD7_DEFAULT_MEM_SIZE as uint32_t,
            )
        }
        0 | _ => r = compression_init_encoder_copy(&mut (*a).archive, &mut (*zip).stream),
    }
    if r == ARCHIVE_OK {
        (*zip).stream.total_in = 0 as libc::c_int as uint64_t;
        (*zip).stream.next_out = (*zip).wbuff.as_mut_ptr();
        (*zip).stream.avail_out = ::std::mem::size_of::<[libc::c_uchar; 61440]>() as libc::c_ulong;
        (*zip).stream.total_out = 0 as libc::c_int as uint64_t
    }
    return r;
}
unsafe extern "C" fn compression_code(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut action: la_zaction,
) -> libc::c_int {
    if (*lastrm).valid != 0 {
        return (*lastrm).code.expect("non-null function pointer")(a, lastrm, action);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_end(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
) -> libc::c_int {
    if (*lastrm).valid != 0 {
        (*lastrm).prop_size = 0 as libc::c_int as uint32_t;
        free((*lastrm).props as *mut libc::c_void);
        (*lastrm).props = NULL as *mut uint8_t;
        return (*lastrm).end.expect("non-null function pointer")(a, lastrm);
    }
    return 0 as libc::c_int;
}
