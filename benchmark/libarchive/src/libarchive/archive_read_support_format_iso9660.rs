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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn timegm(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateReset(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateInit_(
        strm: z_streamp,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    /* Request that the access time of the entry visited by traversal be restored. */
    /*
     * Set behavior. The "flags" argument selects optional behavior.
     */
    /* Request that the access time of the entry visited by traversal be restored.
     * This is the same as archive_read_disk_set_atime_restored. */
    /* Default: Do not skip an entry which has nodump flags. */
    /* Default: Skip a mac resource fork file whose prefix is "._" because of
     * using copyfile. */
    /* Default: Traverse mount points. */
    /* Default: Xattrs are read from disk. */
    /* Default: ACLs are read from disk. */
    /* Default: File flags are read from disk. */
    /*
     * Set archive_match object that will be used in archive_read_disk to
     * know whether an entry should be skipped. The callback function
     * _excluded_func will be invoked when an entry is skipped by the result
     * of archive_match.
     */
    /* Simplified cleanup interface;
     * This calls archive_read_free() or archive_write_free() as needed. */
    /*
     * Accessor functions to read/set various information in
     * the struct archive object:
     */
    /* Number of filters in the current filter pipeline. */
    /* Filter #0 is the one closest to the format, -1 is a synonym for the
     * last filter, which is always the pseudo-filter that wraps the
     * client callbacks. */
    /* These don't properly handle multiple filters, so are deprecated and
     * will eventually be removed. */
    /* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, -1); */
    /* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, 0); */
    /* As of libarchive 3.0, this is an alias for archive_filter_name(a, 0); */
    /* As of libarchive 3.0, this is an alias for archive_filter_code(a, 0); */
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    /*
     * Set fields in an archive_entry.
     *
     * Note: Before libarchive 2.4, there were 'set' and 'copy' versions
     * of the string setters.  'copy' copied the actual string, 'set' just
     * stored the pointer.  In libarchive 2.4 and later, strings are
     * always copied.
     */
    #[no_mangle]
    fn archive_entry_set_atime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_birthtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_unset_birthtime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_hardlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_rdev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_unset_size(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_copy_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn _archive_entry_copy_hardlink_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_copy_pathname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_concat(dest: *mut archive_string, src: *mut archive_string);
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_read_register_format(
        a: *mut archive_read,
        format_data: *mut libc::c_void,
        name: *const libc::c_char,
        bid: Option<unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int>,
        options: Option<
            unsafe extern "C" fn(
                _: *mut archive_read,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
        >,
        read_header: Option<
            unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        >,
        read_data: Option<
            unsafe extern "C" fn(
                _: *mut archive_read,
                _: *mut *const libc::c_void,
                _: *mut size_t,
                _: *mut int64_t,
            ) -> libc::c_int,
        >,
        read_data_skip: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        seek_data: Option<
            unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
        >,
        cleanup: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        format_capabilities: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        has_encrypted_entries: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_read_ahead(
        _: *mut archive_read,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn __archive_read_seek(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iso9660 {
    pub magic: libc::c_int,
    pub opt_support_joliet: libc::c_int,
    pub opt_support_rockridge: libc::c_int,
    pub pathname: archive_string,
    pub seenRockridge: libc::c_char,
    pub seenSUSP: libc::c_char,
    pub seenJoliet: libc::c_char,
    pub suspOffset: libc::c_uchar,
    pub rr_moved: *mut file_info,
    pub read_ce_req: read_ce_queue,
    pub previous_number: int64_t,
    pub previous_pathname: archive_string,
    pub use_files: *mut file_info,
    pub pending_files: heap_queue,
    pub cache_files: C2RustUnnamed_2,
    pub re_files: C2RustUnnamed,
    pub current_position: uint64_t,
    pub logical_block_size: ssize_t,
    pub volume_size: uint64_t,
    pub volume_block: int32_t,
    pub primary: vd,
    pub joliet: vd,
    pub entry_sparse_offset: int64_t,
    pub entry_bytes_remaining: int64_t,
    pub entry_bytes_unconsumed: size_t,
    pub entry_zisofs: zisofs,
    pub entry_content: *mut content,
    pub sconv_utf16be: *mut archive_string_conv,
    pub utf16be_path: *mut libc::c_uchar,
    pub utf16be_path_len: size_t,
    pub utf16be_previous_path: *mut libc::c_uchar,
    pub utf16be_previous_path_len: size_t,
    pub null: [libc::c_uchar; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct content {
    pub offset: uint64_t,
    pub size: uint64_t,
    pub next: *mut content,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zisofs {
    pub pz: libc::c_int,
    pub pz_log2_bs: libc::c_int,
    pub pz_uncompressed_size: uint64_t,
    pub initialized: libc::c_int,
    pub uncompressed_buffer: *mut libc::c_uchar,
    pub uncompressed_buffer_size: size_t,
    pub pz_offset: uint32_t,
    pub header: [libc::c_uchar; 16],
    pub header_avail: size_t,
    pub header_passed: libc::c_int,
    pub block_pointers: *mut libc::c_uchar,
    pub block_pointers_alloc: size_t,
    pub block_pointers_size: size_t,
    pub block_pointers_avail: size_t,
    pub block_off: size_t,
    pub block_avail: uint32_t,
    pub stream: z_stream,
    pub stream_valid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vd {
    pub location: libc::c_int,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub first: *mut file_info,
    pub last: *mut *mut file_info,
}
/* In-memory storage for a directory record. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_info {
    pub use_next: *mut file_info,
    pub parent: *mut file_info,
    pub next: *mut file_info,
    pub re_next: *mut file_info,
    pub subdirs: libc::c_int,
    pub key: uint64_t,
    pub offset: uint64_t,
    pub size: uint64_t,
    pub ce_offset: uint32_t,
    pub ce_size: uint32_t,
    pub rr_moved: libc::c_char,
    pub rr_moved_has_re_only: libc::c_char,
    pub re: libc::c_char,
    pub re_descendant: libc::c_char,
    pub cl_offset: uint64_t,
    pub birthtime_is_set: libc::c_int,
    pub birthtime: time_t,
    pub mtime: time_t,
    pub atime: time_t,
    pub ctime: time_t,
    pub rdev: uint64_t,
    pub mode: mode_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub number: int64_t,
    pub nlinks: libc::c_int,
    pub name: archive_string,
    pub utf16be_name: *mut libc::c_uchar,
    pub utf16be_bytes: size_t,
    pub name_continues: libc::c_char,
    pub symlink: archive_string,
    pub symlink_continues: libc::c_char,
    pub pz: libc::c_int,
    pub pz_log2_bs: libc::c_int,
    pub pz_uncompressed_size: uint64_t,
    pub multi_extent: libc::c_int,
    pub contents: C2RustUnnamed_1,
    pub rede_files: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub first: *mut file_info,
    pub last: *mut *mut file_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub first: *mut content,
    pub last: *mut *mut content,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub first: *mut file_info,
    pub last: *mut *mut file_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_queue {
    pub files: *mut *mut file_info,
    pub allocated: libc::c_int,
    pub used: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_ce_queue {
    pub reqs: *mut read_ce_req,
    pub cnt: libc::c_int,
    pub allocated: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_ce_req {
    pub offset: uint64_t,
    pub file: *mut file_info,
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
    pub passphrases: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
/*
 * This structure is allocated within the archive_read core
 * and initialized by archive_read and the init() method of the
 * corresponding bidder above.
 */
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
 * $FreeBSD: head/lib/libarchive/archive_read_private.h 201088 2009-12-28 02:18:55Z kientzle $
 */
/*
 * How bidding works for filters:
 *   * The bid manager initializes the client-provided reader as the
 *     first filter.
 *   * It invokes the bidder for each registered filter with the
 *     current head filter.
 *   * The bidders can use archive_read_filter_ahead() to peek ahead
 *     at the incoming data to compose their bids.
 *   * The bid manager creates a new filter structure for the winning
 *     bidder and gives the winning bidder a chance to initialize it.
 *   * The new filter becomes the new top filter and we repeat the
 *     process.
 * This ends only when no bidder provides a non-zero bid.  Then
 * we perform a similar dance with the registered format handlers.
 */
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
/*
 * The client looks a lot like a filter, so we just wrap it here.
 *
 * TODO: Make archive_read_filter and archive_read_client identical so
 * that users of the library can easily register their own
 * transformation filters.  This will probably break the API/ABI and
 * so should be deferred at least until libarchive 3.0.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_data_node {
    pub begin_position: int64_t,
    pub total_size: int64_t,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub first: *mut file_info,
    pub last: *mut *mut file_info,
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
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
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
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
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
pub const ARCHIVE_FORMAT_ISO9660: libc::c_int = 0x40000 as libc::c_int;
pub const ARCHIVE_FORMAT_ISO9660_ROCKRIDGE: libc::c_int = ARCHIVE_FORMAT_ISO9660 | 1 as libc::c_int;
#[inline]
unsafe extern "C" fn archive_be32dec(mut pp: *const libc::c_void) -> uint32_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p3: libc::c_uint = *p.offset(3 as libc::c_int as isize) as libc::c_uint;
    let mut p2: libc::c_uint = *p.offset(2 as libc::c_int as isize) as libc::c_uint;
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return p0 << 24 as libc::c_int | p1 << 16 as libc::c_int | p2 << 8 as libc::c_int | p3;
}
#[inline]
unsafe extern "C" fn archive_le16dec(mut pp: *const libc::c_void) -> uint16_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return (p1 << 8 as libc::c_int | p0) as uint16_t;
}
#[inline]
unsafe extern "C" fn archive_le32dec(mut pp: *const libc::c_void) -> uint32_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p3: libc::c_uint = *p.offset(3 as libc::c_int as isize) as libc::c_uint;
    let mut p2: libc::c_uint = *p.offset(2 as libc::c_int as isize) as libc::c_uint;
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return p3 << 24 as libc::c_int | p2 << 16 as libc::c_int | p1 << 8 as libc::c_int | p0;
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
pub const AE_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const archive_entry_copy_hardlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_hardlink_l;
pub const archive_entry_copy_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_pathname_l;
/* #include <stdint.h> */
/* See archive_platform.h */
/*
 * An overview of ISO 9660 format:
 *
 * Each disk is laid out as follows:
 *   * 32k reserved for private use
 *   * Volume descriptor table.  Each volume descriptor
 *     is 2k and specifies basic format information.
 *     The "Primary Volume Descriptor" (PVD) is defined by the
 *     standard and should always be present; other volume
 *     descriptors include various vendor-specific extensions.
 *   * Files and directories.  Each file/dir is specified by
 *     an "extent" (starting sector and length in bytes).
 *     Dirs are just files with directory records packed one
 *     after another.  The PVD contains a single dir entry
 *     specifying the location of the root directory.  Everything
 *     else follows from there.
 *
 * This module works by first reading the volume descriptors, then
 * building a list of directory entries, sorted by starting
 * sector.  At each step, I look for the earliest dir entry that
 * hasn't yet been read, seek forward to that location and read
 * that entry.  If it's a dir, I slurp in the new dir entries and
 * add them to the heap; if it's a regular file, I return the
 * corresponding archive_entry and wait for the client to request
 * the file body.  This strategy allows us to read most compliant
 * CDs with a single pass through the data, as required by libarchive.
 */
pub const LOGICAL_BLOCK_SIZE: libc::c_int = 2048 as libc::c_int;
pub const SYSTEM_AREA_BLOCK: libc::c_int = 16 as libc::c_int;
/* Structure of on-disk primary volume descriptor. */
pub const PVD_type_offset: libc::c_int = 0 as libc::c_int;
pub const PVD_type_size: libc::c_int = 1 as libc::c_int;
pub const PVD_id_offset: libc::c_int = PVD_type_offset + PVD_type_size;
pub const PVD_id_size: libc::c_int = 5 as libc::c_int;
pub const PVD_version_offset: libc::c_int = PVD_id_offset + PVD_id_size;
pub const PVD_version_size: libc::c_int = 1 as libc::c_int;
pub const PVD_reserved1_offset: libc::c_int = PVD_version_offset + PVD_version_size;
pub const PVD_reserved1_size: libc::c_int = 1 as libc::c_int;
pub const PVD_system_id_offset: libc::c_int = PVD_reserved1_offset + PVD_reserved1_size;
pub const PVD_system_id_size: libc::c_int = 32 as libc::c_int;
pub const PVD_volume_id_offset: libc::c_int = PVD_system_id_offset + PVD_system_id_size;
pub const PVD_volume_id_size: libc::c_int = 32 as libc::c_int;
pub const PVD_reserved2_offset: libc::c_int = PVD_volume_id_offset + PVD_volume_id_size;
pub const PVD_reserved2_size: libc::c_int = 8 as libc::c_int;
pub const PVD_volume_space_size_offset: libc::c_int = PVD_reserved2_offset + PVD_reserved2_size;
pub const PVD_volume_space_size_size: libc::c_int = 8 as libc::c_int;
pub const PVD_reserved3_offset: libc::c_int =
    PVD_volume_space_size_offset + PVD_volume_space_size_size;
pub const PVD_reserved3_size: libc::c_int = 32 as libc::c_int;
pub const PVD_volume_set_size_offset: libc::c_int = PVD_reserved3_offset + PVD_reserved3_size;
pub const PVD_volume_set_size_size: libc::c_int = 4 as libc::c_int;
pub const PVD_volume_sequence_number_offset: libc::c_int =
    PVD_volume_set_size_offset + PVD_volume_set_size_size;
pub const PVD_volume_sequence_number_size: libc::c_int = 4 as libc::c_int;
pub const PVD_logical_block_size_offset: libc::c_int =
    PVD_volume_sequence_number_offset + PVD_volume_sequence_number_size;
pub const PVD_logical_block_size_size: libc::c_int = 4 as libc::c_int;
pub const PVD_path_table_size_offset: libc::c_int =
    PVD_logical_block_size_offset + PVD_logical_block_size_size;
pub const PVD_path_table_size_size: libc::c_int = 8 as libc::c_int;
pub const PVD_type_1_path_table_offset: libc::c_int =
    PVD_path_table_size_offset + PVD_path_table_size_size;
pub const PVD_type_1_path_table_size: libc::c_int = 4 as libc::c_int;
pub const PVD_opt_type_1_path_table_offset: libc::c_int =
    PVD_type_1_path_table_offset + PVD_type_1_path_table_size;
pub const PVD_opt_type_1_path_table_size: libc::c_int = 4 as libc::c_int;
pub const PVD_type_m_path_table_offset: libc::c_int =
    PVD_opt_type_1_path_table_offset + PVD_opt_type_1_path_table_size;
pub const PVD_type_m_path_table_size: libc::c_int = 4 as libc::c_int;
pub const PVD_opt_type_m_path_table_offset: libc::c_int =
    PVD_type_m_path_table_offset + PVD_type_m_path_table_size;
pub const PVD_opt_type_m_path_table_size: libc::c_int = 4 as libc::c_int;
pub const PVD_root_directory_record_offset: libc::c_int =
    PVD_opt_type_m_path_table_offset + PVD_opt_type_m_path_table_size;
pub const PVD_root_directory_record_size: libc::c_int = 34 as libc::c_int;
pub const PVD_volume_set_id_offset: libc::c_int =
    PVD_root_directory_record_offset + PVD_root_directory_record_size;
pub const PVD_volume_set_id_size: libc::c_int = 128 as libc::c_int;
pub const PVD_publisher_id_offset: libc::c_int = PVD_volume_set_id_offset + PVD_volume_set_id_size;
pub const PVD_publisher_id_size: libc::c_int = 128 as libc::c_int;
pub const PVD_preparer_id_offset: libc::c_int = PVD_publisher_id_offset + PVD_publisher_id_size;
pub const PVD_preparer_id_size: libc::c_int = 128 as libc::c_int;
pub const PVD_application_id_offset: libc::c_int = PVD_preparer_id_offset + PVD_preparer_id_size;
pub const PVD_application_id_size: libc::c_int = 128 as libc::c_int;
pub const PVD_copyright_file_id_offset: libc::c_int =
    PVD_application_id_offset + PVD_application_id_size;
pub const PVD_copyright_file_id_size: libc::c_int = 37 as libc::c_int;
pub const PVD_abstract_file_id_offset: libc::c_int =
    PVD_copyright_file_id_offset + PVD_copyright_file_id_size;
pub const PVD_abstract_file_id_size: libc::c_int = 37 as libc::c_int;
pub const PVD_bibliographic_file_id_offset: libc::c_int =
    PVD_abstract_file_id_offset + PVD_abstract_file_id_size;
pub const PVD_bibliographic_file_id_size: libc::c_int = 37 as libc::c_int;
pub const PVD_creation_date_offset: libc::c_int =
    PVD_bibliographic_file_id_offset + PVD_bibliographic_file_id_size;
pub const PVD_creation_date_size: libc::c_int = 17 as libc::c_int;
pub const PVD_modification_date_offset: libc::c_int =
    PVD_creation_date_offset + PVD_creation_date_size;
pub const PVD_modification_date_size: libc::c_int = 17 as libc::c_int;
pub const PVD_expiration_date_offset: libc::c_int =
    PVD_modification_date_offset + PVD_modification_date_size;
pub const PVD_expiration_date_size: libc::c_int = 17 as libc::c_int;
pub const PVD_effective_date_offset: libc::c_int =
    PVD_expiration_date_offset + PVD_expiration_date_size;
pub const PVD_effective_date_size: libc::c_int = 17 as libc::c_int;
pub const PVD_file_structure_version_offset: libc::c_int =
    PVD_effective_date_offset + PVD_effective_date_size;
pub const PVD_file_structure_version_size: libc::c_int = 1 as libc::c_int;
pub const PVD_reserved4_offset: libc::c_int =
    PVD_file_structure_version_offset + PVD_file_structure_version_size;
pub const PVD_reserved4_size: libc::c_int = 1 as libc::c_int;
pub const PVD_application_data_offset: libc::c_int = PVD_reserved4_offset + PVD_reserved4_size;
pub const PVD_application_data_size: libc::c_int = 512 as libc::c_int;
pub const PVD_reserved5_offset: libc::c_int =
    PVD_application_data_offset + PVD_application_data_size;
pub const PVD_reserved5_size: libc::c_int = 2048 as libc::c_int - PVD_reserved5_offset;
/* TODO: It would make future maintenance easier to just hardcode the
 * above values.  In particular, ECMA119 states the offsets as part of
 * the standard.  That would eliminate the need for the following check.*/
/* Structure of optional on-disk supplementary volume descriptor. */
pub const SVD_type_offset: libc::c_int = 0 as libc::c_int;
/* ... */
pub const SVD_reserved1_offset: libc::c_int = 72 as libc::c_int;
pub const SVD_reserved1_size: libc::c_int = 8 as libc::c_int;
pub const SVD_volume_space_size_offset: libc::c_int = 80 as libc::c_int;
pub const SVD_volume_space_size_size: libc::c_int = 8 as libc::c_int;
pub const SVD_escape_sequences_offset: libc::c_int =
    SVD_volume_space_size_offset + SVD_volume_space_size_size;
/* ... */
pub const SVD_logical_block_size_offset: libc::c_int = 128 as libc::c_int;
pub const SVD_type_L_path_table_offset: libc::c_int = 140 as libc::c_int;
pub const SVD_type_M_path_table_offset: libc::c_int = 148 as libc::c_int;
/* ... */
pub const SVD_root_directory_record_offset: libc::c_int = 156 as libc::c_int;
pub const SVD_file_structure_version_offset: libc::c_int = 881 as libc::c_int;
pub const SVD_reserved2_offset: libc::c_int = 882 as libc::c_int;
pub const SVD_reserved2_size: libc::c_int = 1 as libc::c_int;
pub const SVD_reserved3_offset: libc::c_int = 1395 as libc::c_int;
pub const SVD_reserved3_size: libc::c_int = 653 as libc::c_int;
/* ... */
/* FIXME: validate correctness of last SVD entry offset. */
/* Structure of an on-disk directory record. */
/* Note:  ISO9660 stores each multi-byte integer twice, once in
 * each byte order.  The sizes here are the size of just one
 * of the two integers.  (This is why the offset of a field isn't
 * the same as the offset+size of the previous field.) */
pub const DR_length_offset: libc::c_int = 0 as libc::c_int;
pub const DR_extent_offset: libc::c_int = 2 as libc::c_int;
pub const DR_size_offset: libc::c_int = 10 as libc::c_int;
pub const DR_size_size: libc::c_int = 4 as libc::c_int;
pub const DR_date_offset: libc::c_int = 18 as libc::c_int;
pub const DR_flags_offset: libc::c_int = 25 as libc::c_int;
pub const DR_name_len_offset: libc::c_int = 32 as libc::c_int;
pub const DR_name_offset: libc::c_int = 33 as libc::c_int;
static mut zisofs_magic: [libc::c_uchar; 8] = [
    0x37 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
];
pub const ISO9660_MAGIC: libc::c_uint = 0x96609660 as libc::c_uint;
pub const UTF16_NAME_MAX: libc::c_int = 1024 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_iso9660(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_iso9660\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    iso9660 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<iso9660>() as libc::c_ulong,
    ) as *mut iso9660;
    if iso9660.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate iso9660 data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*iso9660).magic = ISO9660_MAGIC as libc::c_int;
    (*iso9660).cache_files.first = NULL as *mut file_info;
    (*iso9660).cache_files.last = &mut (*iso9660).cache_files.first;
    (*iso9660).re_files.first = NULL as *mut file_info;
    (*iso9660).re_files.last = &mut (*iso9660).re_files.first;
    /* Enable to support Joliet extensions by default.	*/
    (*iso9660).opt_support_joliet = 1 as libc::c_int;
    /* Enable to support Rock Ridge extensions by default.	*/
    (*iso9660).opt_support_rockridge = 1 as libc::c_int;
    r = __archive_read_register_format(
        a,
        iso9660 as *mut libc::c_void,
        b"iso9660\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_iso9660_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_iso9660_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_iso9660_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_iso9660_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_iso9660_read_data_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_iso9660_cleanup
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        >(NULL as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        >(NULL as libc::intptr_t),
    );
    if r != ARCHIVE_OK {
        free(iso9660 as *mut libc::c_void);
        return r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_iso9660_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut bytes_read: ssize_t = 0;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut seenTerminator: libc::c_int = 0;
    /* If there's already a better bid than we can ever
    make, don't bother testing. */
    if best_bid > 48 as libc::c_int {
        return -(1 as libc::c_int);
    }
    iso9660 = (*(*a).format).data as *mut iso9660;
    /*
     * Skip the first 32k (reserved area) and get the first
     * 8 sectors of the volume descriptor table.  Of course,
     * if the I/O layer gives us more, we'll take it.
     */
    p = __archive_read_ahead(
        a,
        (RESERVED_AREA + 8 as libc::c_int * LOGICAL_BLOCK_SIZE) as size_t,
        &mut bytes_read,
    ) as *const libc::c_uchar;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    /* Skip the reserved area. */
    bytes_read -= RESERVED_AREA as libc::c_long;
    p = p.offset(RESERVED_AREA as isize);
    /* Check each volume descriptor. */
    seenTerminator = 0 as libc::c_int;
    let mut current_block_14: u64;
    while bytes_read > LOGICAL_BLOCK_SIZE as libc::c_long {
        /* Do not handle undefined Volume Descriptor Type. */
        if *p.offset(0 as libc::c_int as isize) as libc::c_int >= 4 as libc::c_int
            && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 254 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        /* Standard Identifier must be "CD001" */
        if memcmp(
            p.offset(1 as libc::c_int as isize) as *const libc::c_void,
            b"CD001\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if !(isPVD(iso9660, p) != 0) {
            if (*iso9660).joliet.location == 0 {
                if isJolietSVD(iso9660, p) != 0 {
                    current_block_14 = 13109137661213826276;
                } else {
                    current_block_14 = 6009453772311597924;
                }
            } else {
                current_block_14 = 6009453772311597924;
            }
            match current_block_14 {
                13109137661213826276 => {}
                _ => {
                    if !(isBootRecord(iso9660, p) != 0) {
                        if !(isEVD(iso9660, p) != 0) {
                            if !(isSVD(iso9660, p) != 0) {
                                if !(isVolumePartition(iso9660, p) != 0) {
                                    if isVDSetTerminator(iso9660, p) != 0 {
                                        seenTerminator = 1 as libc::c_int;
                                        break;
                                    } else {
                                        return 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        bytes_read -= LOGICAL_BLOCK_SIZE as libc::c_long;
        p = p.offset(LOGICAL_BLOCK_SIZE as isize)
    }
    /*
     * ISO 9660 format must have Primary Volume Descriptor and
     * Volume Descriptor Set Terminator.
     */
    if seenTerminator != 0 && (*iso9660).primary.location > 16 as libc::c_int {
        return 48 as libc::c_int;
    }
    /* We didn't find a valid PVD; return a bid of zero. */
    return 0 as libc::c_int;
}
pub const RESERVED_AREA: libc::c_int = SYSTEM_AREA_BLOCK * LOGICAL_BLOCK_SIZE;
unsafe extern "C" fn archive_read_format_iso9660_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    iso9660 = (*(*a).format).data as *mut iso9660;
    if strcmp(key, b"joliet\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if val.is_null()
            || strcmp(val, b"off\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(val, b"ignore\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(val, b"disable\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(val, b"0\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*iso9660).opt_support_joliet = 0 as libc::c_int
        } else {
            (*iso9660).opt_support_joliet = 1 as libc::c_int
        }
        return 0 as libc::c_int;
    }
    if strcmp(key, b"rockridge\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(key, b"Rockridge\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*iso9660).opt_support_rockridge = (val != NULL as *const libc::c_char) as libc::c_int;
        return 0 as libc::c_int;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn isNull(
    mut iso9660: *mut iso9660,
    mut h: *const libc::c_uchar,
    mut offset: libc::c_uint,
    mut bytes: libc::c_uint,
) -> libc::c_int {
    while bytes as libc::c_ulong >= ::std::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong
    {
        if memcmp(
            (*iso9660).null.as_mut_ptr() as *const libc::c_void,
            h.offset(offset as isize) as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        offset = (offset as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong)
            as libc::c_uint as libc::c_uint;
        bytes = (bytes as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong)
            as libc::c_uint as libc::c_uint
    }
    if bytes != 0 {
        return (memcmp(
            (*iso9660).null.as_mut_ptr() as *const libc::c_void,
            h.offset(offset as isize) as *const libc::c_void,
            bytes as libc::c_ulong,
        ) == 0 as libc::c_int) as libc::c_int;
    } else {
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn isBootRecord(
    mut iso9660: *mut iso9660,
    mut h: *const libc::c_uchar,
) -> libc::c_int {
    /* UNUSED */
    /* Type of the Volume Descriptor Boot Record must be 0. */
    if *h.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Volume Descriptor Version must be 1. */
    if *h.offset(6 as libc::c_int as isize) as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isVolumePartition(
    mut iso9660: *mut iso9660,
    mut h: *const libc::c_uchar,
) -> libc::c_int {
    let mut location: int32_t = 0;
    /* Type of the Volume Partition Descriptor must be 3. */
    if *h.offset(0 as libc::c_int as isize) as libc::c_int != 3 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Volume Descriptor Version must be 1. */
    if *h.offset(6 as libc::c_int as isize) as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Unused Field */
    if *h.offset(7 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    location =
        archive_le32dec(h.offset(72 as libc::c_int as isize) as *const libc::c_void) as int32_t;
    if location <= SYSTEM_AREA_BLOCK || location >= (*iso9660).volume_block {
        return 0 as libc::c_int;
    }
    if location as uint32_t
        != archive_be32dec(h.offset(76 as libc::c_int as isize) as *const libc::c_void)
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isVDSetTerminator(
    mut iso9660: *mut iso9660,
    mut h: *const libc::c_uchar,
) -> libc::c_int {
    /* UNUSED */
    /* Type of the Volume Descriptor Set Terminator must be 255. */
    if *h.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Volume Descriptor Version must be 1. */
    if *h.offset(6 as libc::c_int as isize) as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        7 as libc::c_int as libc::c_uint,
        (2048 as libc::c_int - 7 as libc::c_int) as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isJolietSVD(
    mut iso9660: *mut iso9660,
    mut h: *const libc::c_uchar,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut logical_block_size: ssize_t = 0;
    let mut volume_block: int32_t = 0;
    /* Check if current sector is a kind of Supplementary Volume
     * Descriptor. */
    if isSVD(iso9660, h) == 0 {
        return 0 as libc::c_int;
    }
    /* FIXME: do more validations according to joliet spec. */
    /* check if this SVD contains joliet extension! */
    p = h.offset(SVD_escape_sequences_offset as isize);
    /* N.B. Joliet spec says p[1] == '\\', but.... */
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        let mut level: libc::c_int = 0 as libc::c_int; /* not joliet */
        if *p.offset(2 as libc::c_int as isize) as libc::c_int == '@' as i32 {
            level = 1 as libc::c_int
        } else if *p.offset(2 as libc::c_int as isize) as libc::c_int == 'C' as i32 {
            level = 2 as libc::c_int
        } else if *p.offset(2 as libc::c_int as isize) as libc::c_int == 'E' as i32 {
            level = 3 as libc::c_int
        } else {
            /* not joliet */
            return 0 as libc::c_int;
        }
        (*iso9660).seenJoliet = level as libc::c_char
    } else {
        return 0 as libc::c_int;
    }
    logical_block_size =
        archive_le16dec(h.offset(SVD_logical_block_size_offset as isize) as *const libc::c_void)
            as ssize_t;
    volume_block =
        archive_le32dec(h.offset(SVD_volume_space_size_offset as isize) as *const libc::c_void)
            as int32_t;
    (*iso9660).logical_block_size = logical_block_size;
    (*iso9660).volume_block = volume_block;
    (*iso9660).volume_size =
        (logical_block_size as libc::c_ulong).wrapping_mul(volume_block as uint64_t);
    /* Read Root Directory Record in Volume Descriptor. */
    p = h.offset(SVD_root_directory_record_offset as isize);
    (*iso9660).joliet.location =
        archive_le32dec(p.offset(DR_extent_offset as isize) as *const libc::c_void) as libc::c_int;
    (*iso9660).joliet.size =
        archive_le32dec(p.offset(DR_size_offset as isize) as *const libc::c_void);
    return 48 as libc::c_int;
}
unsafe extern "C" fn isSVD(mut iso9660: *mut iso9660, mut h: *const libc::c_uchar) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut logical_block_size: ssize_t = 0;
    let mut volume_block: int32_t = 0;
    let mut location: int32_t = 0;
    /* UNUSED */
    /* Type 2 means it's a SVD. */
    if *h.offset(SVD_type_offset as isize) as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        SVD_reserved1_offset as libc::c_uint,
        SVD_reserved1_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if isNull(
        iso9660,
        h,
        SVD_reserved2_offset as libc::c_uint,
        SVD_reserved2_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if isNull(
        iso9660,
        h,
        SVD_reserved3_offset as libc::c_uint,
        SVD_reserved3_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* File structure version must be 1 for ISO9660/ECMA119. */
    if *h.offset(SVD_file_structure_version_offset as isize) as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    logical_block_size =
        archive_le16dec(h.offset(SVD_logical_block_size_offset as isize) as *const libc::c_void)
            as ssize_t;
    if logical_block_size <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    volume_block =
        archive_le32dec(h.offset(SVD_volume_space_size_offset as isize) as *const libc::c_void)
            as int32_t;
    if volume_block <= SYSTEM_AREA_BLOCK + 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Location of Occurrence of Type L Path Table must be
     * available location,
     * >= SYSTEM_AREA_BLOCK(16) + 2 and < Volume Space Size. */
    location =
        archive_le32dec(h.offset(SVD_type_L_path_table_offset as isize) as *const libc::c_void)
            as int32_t;
    if location < SYSTEM_AREA_BLOCK + 2 as libc::c_int || location >= volume_block {
        return 0 as libc::c_int;
    }
    /* The Type M Path Table must be at a valid location (WinISO
     * and probably other programs omit this, so we allow zero)
     *
     * >= SYSTEM_AREA_BLOCK(16) + 2 and < Volume Space Size. */
    location =
        archive_be32dec(h.offset(SVD_type_M_path_table_offset as isize) as *const libc::c_void)
            as int32_t;
    if location > 0 as libc::c_int && location < SYSTEM_AREA_BLOCK + 2 as libc::c_int
        || location >= volume_block
    {
        return 0 as libc::c_int;
    }
    /* Read Root Directory Record in Volume Descriptor. */
    p = h.offset(SVD_root_directory_record_offset as isize);
    if *p.offset(DR_length_offset as isize) as libc::c_int != 34 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 48 as libc::c_int;
}
unsafe extern "C" fn isEVD(mut iso9660: *mut iso9660, mut h: *const libc::c_uchar) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut logical_block_size: ssize_t = 0;
    let mut volume_block: int32_t = 0;
    let mut location: int32_t = 0;
    /* UNUSED */
    /* Type of the Enhanced Volume Descriptor must be 2. */
    if *h.offset(PVD_type_offset as isize) as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* EVD version must be 2. */
    if *h.offset(PVD_version_offset as isize) as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if *h.offset(PVD_reserved1_offset as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        PVD_reserved2_offset as libc::c_uint,
        PVD_reserved2_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        PVD_reserved3_offset as libc::c_uint,
        PVD_reserved3_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Logical block size must be > 0. */
    /* I've looked at Ecma 119 and can't find any stronger
     * restriction on this field. */
    logical_block_size =
        archive_le16dec(h.offset(PVD_logical_block_size_offset as isize) as *const libc::c_void)
            as ssize_t;
    if logical_block_size <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    volume_block =
        archive_le32dec(h.offset(PVD_volume_space_size_offset as isize) as *const libc::c_void)
            as int32_t;
    if volume_block <= SYSTEM_AREA_BLOCK + 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* File structure version must be 2 for ISO9660:1999. */
    if *h.offset(PVD_file_structure_version_offset as isize) as libc::c_int != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Location of Occurrence of Type L Path Table must be
     * available location,
     * >= SYSTEM_AREA_BLOCK(16) + 2 and < Volume Space Size. */
    location =
        archive_le32dec(h.offset(PVD_type_1_path_table_offset as isize) as *const libc::c_void)
            as int32_t;
    if location < SYSTEM_AREA_BLOCK + 2 as libc::c_int || location >= volume_block {
        return 0 as libc::c_int;
    }
    /* Location of Occurrence of Type M Path Table must be
     * available location,
     * >= SYSTEM_AREA_BLOCK(16) + 2 and < Volume Space Size. */
    location =
        archive_be32dec(h.offset(PVD_type_m_path_table_offset as isize) as *const libc::c_void)
            as int32_t;
    if location > 0 as libc::c_int && location < SYSTEM_AREA_BLOCK + 2 as libc::c_int
        || location >= volume_block
    {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        PVD_reserved4_offset as libc::c_uint,
        PVD_reserved4_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        PVD_reserved5_offset as libc::c_uint,
        PVD_reserved5_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Read Root Directory Record in Volume Descriptor. */
    p = h.offset(PVD_root_directory_record_offset as isize);
    if *p.offset(DR_length_offset as isize) as libc::c_int != 34 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 48 as libc::c_int;
}
unsafe extern "C" fn isPVD(mut iso9660: *mut iso9660, mut h: *const libc::c_uchar) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut logical_block_size: ssize_t = 0;
    let mut volume_block: int32_t = 0;
    let mut location: int32_t = 0;
    let mut i: libc::c_int = 0;
    /* Type of the Primary Volume Descriptor must be 1. */
    if *h.offset(PVD_type_offset as isize) as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* PVD version must be 1. */
    if *h.offset(PVD_version_offset as isize) as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if *h.offset(PVD_reserved1_offset as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        PVD_reserved2_offset as libc::c_uint,
        PVD_reserved2_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        PVD_reserved3_offset as libc::c_uint,
        PVD_reserved3_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Logical block size must be > 0. */
    /* I've looked at Ecma 119 and can't find any stronger
     * restriction on this field. */
    logical_block_size =
        archive_le16dec(h.offset(PVD_logical_block_size_offset as isize) as *const libc::c_void)
            as ssize_t;
    if logical_block_size <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    volume_block =
        archive_le32dec(h.offset(PVD_volume_space_size_offset as isize) as *const libc::c_void)
            as int32_t;
    if volume_block <= SYSTEM_AREA_BLOCK + 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* File structure version must be 1 for ISO9660/ECMA119. */
    if *h.offset(PVD_file_structure_version_offset as isize) as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    /* Location of Occurrence of Type L Path Table must be
     * available location,
     * > SYSTEM_AREA_BLOCK(16) + 2 and < Volume Space Size. */
    location =
        archive_le32dec(h.offset(PVD_type_1_path_table_offset as isize) as *const libc::c_void)
            as int32_t;
    if location < SYSTEM_AREA_BLOCK + 2 as libc::c_int || location >= volume_block {
        return 0 as libc::c_int;
    }
    /* The Type M Path Table must also be at a valid location
     * (although ECMA 119 requires a Type M Path Table, WinISO and
     * probably other programs omit it, so we permit a zero here)
     *
     * >= SYSTEM_AREA_BLOCK(16) + 2 and < Volume Space Size. */
    location =
        archive_be32dec(h.offset(PVD_type_m_path_table_offset as isize) as *const libc::c_void)
            as int32_t;
    if location > 0 as libc::c_int && location < SYSTEM_AREA_BLOCK + 2 as libc::c_int
        || location >= volume_block
    {
        return 0 as libc::c_int;
    }
    /* Reserved field must be 0. */
    /* But accept NetBSD/FreeBSD "makefs" images with 0x20 here. */
    i = 0 as libc::c_int;
    while i < PVD_reserved4_size {
        if *h.offset((PVD_reserved4_offset + i) as isize) as libc::c_int != 0 as libc::c_int
            && *h.offset((PVD_reserved4_offset + i) as isize) as libc::c_int != 0x20 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1
    }
    /* Reserved field must be 0. */
    if isNull(
        iso9660,
        h,
        PVD_reserved5_offset as libc::c_uint,
        PVD_reserved5_size as libc::c_uint,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    /* XXX TODO: Check other values for sanity; reject more
     * malformed PVDs. XXX */
    /* Read Root Directory Record in Volume Descriptor. */
    p = h.offset(PVD_root_directory_record_offset as isize);
    if *p.offset(DR_length_offset as isize) as libc::c_int != 34 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*iso9660).primary.location == 0 {
        (*iso9660).logical_block_size = logical_block_size;
        (*iso9660).volume_block = volume_block;
        (*iso9660).volume_size =
            (logical_block_size as libc::c_ulong).wrapping_mul(volume_block as uint64_t);
        (*iso9660).primary.location =
            archive_le32dec(p.offset(DR_extent_offset as isize) as *const libc::c_void)
                as libc::c_int;
        (*iso9660).primary.size =
            archive_le32dec(p.offset(DR_size_offset as isize) as *const libc::c_void)
    }
    return 48 as libc::c_int;
}
unsafe extern "C" fn read_children(
    mut a: *mut archive_read,
    mut parent: *mut file_info,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut multi: *mut file_info = 0 as *mut file_info;
    let mut step: size_t = 0;
    let mut skip_size: size_t = 0;
    iso9660 = (*(*a).format).data as *mut iso9660;
    /* flush any remaining bytes from the last round to ensure
     * we're positioned */
    if (*iso9660).entry_bytes_unconsumed != 0 {
        __archive_read_consume(a, (*iso9660).entry_bytes_unconsumed as int64_t);
        (*iso9660).entry_bytes_unconsumed = 0 as libc::c_int as size_t
    }
    if (*iso9660).current_position > (*parent).offset {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Ignoring out-of-order directory (%s) %jd > %jd\x00" as *const u8
                as *const libc::c_char,
            (*parent).name.s,
            (*iso9660).current_position as intmax_t,
            (*parent).offset as intmax_t,
        );
        return -(20 as libc::c_int);
    }
    if (*parent).offset.wrapping_add((*parent).size) > (*iso9660).volume_size {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Directory is beyond end-of-media: %s\x00" as *const u8 as *const libc::c_char,
            (*parent).name.s,
        );
        return -(20 as libc::c_int);
    }
    if (*iso9660).current_position < (*parent).offset {
        let mut skipsize: int64_t = 0;
        skipsize = (*parent).offset.wrapping_sub((*iso9660).current_position) as int64_t;
        skipsize = __archive_read_consume(a, skipsize);
        if skipsize < 0 as libc::c_int as libc::c_long {
            return skipsize as libc::c_int;
        }
        (*iso9660).current_position = (*parent).offset
    }
    step = (*parent)
        .size
        .wrapping_add((*iso9660).logical_block_size as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div((*iso9660).logical_block_size as libc::c_ulong)
        .wrapping_mul((*iso9660).logical_block_size as libc::c_ulong);
    b = __archive_read_ahead(a, step, NULL as *mut ssize_t) as *const libc::c_uchar;
    if b.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Failed to read full block when scanning ISO9660 directory list\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*iso9660).current_position =
        ((*iso9660).current_position as libc::c_ulong).wrapping_add(step) as uint64_t as uint64_t;
    multi = NULL as *mut file_info;
    skip_size = step;
    while step != 0 {
        p = b;
        b = b.offset((*iso9660).logical_block_size as isize);
        step = (step as libc::c_ulong).wrapping_sub((*iso9660).logical_block_size as libc::c_ulong)
            as size_t as size_t;
        while *p as libc::c_int != 0 as libc::c_int
            && p < b
            && p.offset(*p as libc::c_int as isize) <= b
        {
            let mut child: *mut file_info = 0 as *mut file_info;
            /* N.B.: these special directory identifiers
             * are 8 bit "values" even on a
             * Joliet CD with UCS-2 (16bit) encoding.
             */
            /* Skip '.' entry. */
            if !(*p.offset(DR_name_len_offset as isize) as libc::c_int == 1 as libc::c_int
                && *p.offset(DR_name_offset as isize) as libc::c_int == '\u{0}' as i32)
            {
                /* Skip '..' entry. */
                if !(*p.offset(DR_name_len_offset as isize) as libc::c_int == 1 as libc::c_int
                    && *p.offset(DR_name_offset as isize) as libc::c_int == '\u{1}' as i32)
                {
                    child = parse_file_info(
                        a,
                        parent,
                        p,
                        b.offset_from(p) as libc::c_long as size_t,
                    );
                    if child.is_null() {
                        __archive_read_consume(a, skip_size as int64_t);
                        return -(30 as libc::c_int);
                    }
                    if (*child).cl_offset == 0 as libc::c_int as libc::c_ulong
                        && ((*child).multi_extent != 0 || !multi.is_null())
                    {
                        let mut con: *mut content = 0 as *mut content;
                        if multi.is_null() {
                            multi = child;
                            (*multi).contents.first = NULL as *mut content;
                            (*multi).contents.last = &mut (*multi).contents.first
                        }
                        con = malloc(::std::mem::size_of::<content>() as libc::c_ulong)
                            as *mut content;
                        if con.is_null() {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ENOMEM,
                                b"No memory for multi extent\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            __archive_read_consume(a, skip_size as int64_t);
                            return -(30 as libc::c_int);
                        }
                        (*con).offset = (*child).offset;
                        (*con).size = (*child).size;
                        (*con).next = NULL as *mut content;
                        *(*multi).contents.last = con;
                        (*multi).contents.last = &mut (*con).next;
                        if multi == child {
                            if heap_add_entry(
                                a,
                                &mut (*iso9660).pending_files,
                                child,
                                (*child).offset,
                            ) != ARCHIVE_OK
                            {
                                return -(30 as libc::c_int);
                            }
                        } else {
                            (*multi).size = ((*multi).size as libc::c_ulong)
                                .wrapping_add((*child).size)
                                as uint64_t as uint64_t;
                            if (*child).multi_extent == 0 {
                                multi = NULL as *mut file_info
                            }
                        }
                    } else if heap_add_entry(
                        a,
                        &mut (*iso9660).pending_files,
                        child,
                        (*child).offset,
                    ) != ARCHIVE_OK
                    {
                        return -(30 as libc::c_int);
                    }
                }
            }
            p = p.offset(*p as libc::c_int as isize)
        }
    }
    __archive_read_consume(a, skip_size as int64_t);
    /* Read data which recorded by RRIP "CE" extension. */
    if read_CE(a, iso9660) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn choose_volume(
    mut a: *mut archive_read,
    mut iso9660: *mut iso9660,
) -> libc::c_int {
    let mut file: *mut file_info = 0 as *mut file_info;
    let mut skipsize: int64_t = 0;
    let mut vd: *mut vd = 0 as *mut vd;
    let mut block: *const libc::c_void = 0 as *const libc::c_void;
    let mut seenJoliet: libc::c_char = 0;
    vd = &mut (*iso9660).primary;
    if (*iso9660).opt_support_joliet == 0 {
        (*iso9660).seenJoliet = 0 as libc::c_int as libc::c_char
    }
    if (*iso9660).seenJoliet as libc::c_int != 0 && (*vd).location > (*iso9660).joliet.location {
        /* This condition is unlikely; by way of caution. */
        vd = &mut (*iso9660).joliet
    }
    skipsize = LOGICAL_BLOCK_SIZE as libc::c_long * (*vd).location as int64_t;
    skipsize = __archive_read_consume(a, skipsize);
    if skipsize < 0 as libc::c_int as libc::c_long {
        return skipsize as libc::c_int;
    }
    (*iso9660).current_position = skipsize as uint64_t;
    block = __archive_read_ahead(a, (*vd).size as size_t, NULL as *mut ssize_t);
    if block == NULL as *const libc::c_void {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Failed to read full block when scanning ISO9660 directory list\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * While reading Root Directory, flag seenJoliet must be zero to
     * avoid converting special name 0x00(Current Directory) and
     * next byte to UCS2.
     */
    seenJoliet = (*iso9660).seenJoliet; /* Save flag. */
    (*iso9660).seenJoliet = 0 as libc::c_int as libc::c_char;
    file = parse_file_info(
        a,
        NULL as *mut file_info,
        block as *const libc::c_uchar,
        (*vd).size as size_t,
    );
    if file.is_null() {
        return -(30 as libc::c_int);
    }
    (*iso9660).seenJoliet = seenJoliet;
    /*
     * If the iso image has both RockRidge and Joliet, we preferentially
     * use RockRidge Extensions rather than Joliet ones.
     */
    if vd == &mut (*iso9660).primary as *mut vd
        && (*iso9660).seenRockridge as libc::c_int != 0
        && (*iso9660).seenJoliet as libc::c_int != 0
    {
        (*iso9660).seenJoliet = 0 as libc::c_int as libc::c_char
    }
    if vd == &mut (*iso9660).primary as *mut vd
        && (*iso9660).seenRockridge == 0
        && (*iso9660).seenJoliet as libc::c_int != 0
    {
        /* Switch reading data from primary to joliet. */
        vd = &mut (*iso9660).joliet;
        skipsize = LOGICAL_BLOCK_SIZE as libc::c_long * (*vd).location as int64_t;
        skipsize = (skipsize as libc::c_ulong).wrapping_sub((*iso9660).current_position) as int64_t
            as int64_t;
        skipsize = __archive_read_consume(a, skipsize);
        if skipsize < 0 as libc::c_int as libc::c_long {
            return skipsize as libc::c_int;
        }
        (*iso9660).current_position = ((*iso9660).current_position as libc::c_ulong)
            .wrapping_add(skipsize as libc::c_ulong)
            as uint64_t as uint64_t;
        block = __archive_read_ahead(a, (*vd).size as size_t, NULL as *mut ssize_t);
        if block == NULL as *const libc::c_void {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Failed to read full block when scanning ISO9660 directory list\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*iso9660).seenJoliet = 0 as libc::c_int as libc::c_char;
        file = parse_file_info(
            a,
            NULL as *mut file_info,
            block as *const libc::c_uchar,
            (*vd).size as size_t,
        );
        if file.is_null() {
            return -(30 as libc::c_int);
        }
        (*iso9660).seenJoliet = seenJoliet
    }
    /* Store the root directory in the pending list. */
    if heap_add_entry(a, &mut (*iso9660).pending_files, file, (*file).offset) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    } /* Eliminate a warning. */
    if (*iso9660).seenRockridge != 0 {
        (*a).archive.archive_format = ARCHIVE_FORMAT_ISO9660_ROCKRIDGE;
        (*a).archive.archive_format_name =
            b"ISO9660 with Rockridge extensions\x00" as *const u8 as *const libc::c_char
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_iso9660_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut file: *mut file_info = 0 as *mut file_info;
    let mut r: libc::c_int = 0;
    let mut rd_r: libc::c_int = ARCHIVE_OK;
    iso9660 = (*(*a).format).data as *mut iso9660;
    if (*a).archive.archive_format == 0 {
        (*a).archive.archive_format = ARCHIVE_FORMAT_ISO9660;
        (*a).archive.archive_format_name = b"ISO9660\x00" as *const u8 as *const libc::c_char
    }
    if (*iso9660).current_position == 0 as libc::c_int as libc::c_ulong {
        r = choose_volume(a, iso9660);
        if r != ARCHIVE_OK {
            return r;
        }
    }
    file = NULL as *mut file_info;
    /* Get the next entry that appears after the current offset. */
    r = next_entry_seek(a, iso9660, &mut file);
    if r != ARCHIVE_OK {
        return r;
    }
    if (*iso9660).seenJoliet != 0 {
        /*
         * Convert UTF-16BE of a filename to local locale MBS
         * and store the result into a filename field.
         */
        if (*iso9660).sconv_utf16be.is_null() {
            (*iso9660).sconv_utf16be = archive_string_conversion_from_charset(
                &mut (*a).archive,
                b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*iso9660).sconv_utf16be.is_null() {
                /* Couldn't allocate memory */
                return -(30 as libc::c_int);
            }
        }
        if (*iso9660).utf16be_path.is_null() {
            (*iso9660).utf16be_path = malloc(UTF16_NAME_MAX as libc::c_ulong) as *mut libc::c_uchar;
            if (*iso9660).utf16be_path.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        if (*iso9660).utf16be_previous_path.is_null() {
            (*iso9660).utf16be_previous_path =
                malloc(UTF16_NAME_MAX as libc::c_ulong) as *mut libc::c_uchar;
            if (*iso9660).utf16be_previous_path.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        (*iso9660).utf16be_path_len = 0 as libc::c_int as size_t;
        if build_pathname_utf16be(
            (*iso9660).utf16be_path,
            UTF16_NAME_MAX as size_t,
            &mut (*iso9660).utf16be_path_len,
            file,
        ) != 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Pathname is too long\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        r = _archive_entry_copy_pathname_l(
            entry,
            (*iso9660).utf16be_path as *const libc::c_char,
            (*iso9660).utf16be_path_len,
            (*iso9660).sconv_utf16be,
        );
        if r != 0 as libc::c_int {
            if errno == ENOMEM {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory for Pathname\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Pathname cannot be converted from %s to current locale.\x00" as *const u8
                    as *const libc::c_char,
                archive_string_conversion_charset_name((*iso9660).sconv_utf16be),
            );
            rd_r = ARCHIVE_WARN
        }
    } else {
        let mut path: *const libc::c_char =
            build_pathname(&mut (*iso9660).pathname, file, 0 as libc::c_int);
        if path.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Pathname is too long\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        } else {
            (*iso9660).pathname.length = 0 as libc::c_int as size_t;
            archive_entry_set_pathname(entry, path);
        }
    }
    (*iso9660).entry_bytes_remaining = (*file).size as int64_t;
    /* Offset for sparse-file-aware clients. */
    (*iso9660).entry_sparse_offset = 0 as libc::c_int as int64_t;
    if (*file).offset.wrapping_add((*file).size) > (*iso9660).volume_size {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"File is beyond end-of-media: %s\x00" as *const u8 as *const libc::c_char,
            archive_entry_pathname(entry),
        );
        (*iso9660).entry_bytes_remaining = 0 as libc::c_int as int64_t;
        return -(20 as libc::c_int);
    }
    /* Set up the entry structure with information about this entry. */
    archive_entry_set_mode(entry, (*file).mode);
    archive_entry_set_uid(entry, (*file).uid as la_int64_t);
    archive_entry_set_gid(entry, (*file).gid as la_int64_t);
    archive_entry_set_nlink(entry, (*file).nlinks as libc::c_uint);
    if (*file).birthtime_is_set != 0 {
        archive_entry_set_birthtime(entry, (*file).birthtime, 0 as libc::c_int as libc::c_long);
    } else {
        archive_entry_unset_birthtime(entry);
    }
    archive_entry_set_mtime(entry, (*file).mtime, 0 as libc::c_int as libc::c_long);
    archive_entry_set_ctime(entry, (*file).ctime, 0 as libc::c_int as libc::c_long);
    archive_entry_set_atime(entry, (*file).atime, 0 as libc::c_int as libc::c_long);
    /* N.B.: Rock Ridge supports 64-bit device numbers. */
    archive_entry_set_rdev(entry, (*file).rdev);
    archive_entry_set_size(entry, (*iso9660).entry_bytes_remaining);
    if !(*file).symlink.s.is_null() {
        archive_entry_copy_symlink(entry, (*file).symlink.s);
    }
    /* Note: If the input isn't seekable, we can't rewind to
     * return the same body again, so if the next entry refers to
     * the same data, we have to return it as a hardlink to the
     * original entry. */
    if (*file).number != -(1 as libc::c_int) as libc::c_long
        && (*file).number == (*iso9660).previous_number
    {
        if (*iso9660).seenJoliet != 0 {
            r = _archive_entry_copy_hardlink_l(
                entry,
                (*iso9660).utf16be_previous_path as *const libc::c_char,
                (*iso9660).utf16be_previous_path_len,
                (*iso9660).sconv_utf16be,
            );
            if r != 0 as libc::c_int {
                if errno == ENOMEM {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"No memory for Linkname\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Linkname cannot be converted from %s to current locale.\x00" as *const u8
                        as *const libc::c_char,
                    archive_string_conversion_charset_name((*iso9660).sconv_utf16be),
                );
                rd_r = ARCHIVE_WARN
            }
        } else {
            archive_entry_set_hardlink(entry, (*iso9660).previous_pathname.s);
        }
        archive_entry_unset_size(entry);
        (*iso9660).entry_bytes_remaining = 0 as libc::c_int as int64_t;
        return rd_r;
    }
    if (*file).mode & AE_IFMT as mode_t != AE_IFDIR as mode_t
        && (*file).offset < (*iso9660).current_position
    {
        let mut r64: int64_t = 0;
        r64 = __archive_read_seek(a, (*file).offset as int64_t, SEEK_SET);
        if r64 != (*file).offset as int64_t {
            /* We can't seek backwards to extract it, so issue
             * a warning.  Note that this can only happen if
             * this entry was added to the heap after we passed
             * this offset, that is, only if the directory
             * mentioning this entry is later than the body of
             * the entry. Such layouts are very unusual; most
             * ISO9660 writers lay out and record all directory
             * information first, then store all file bodies. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Ignoring out-of-order file @%jx (%s) %jd < %jd\x00" as *const u8
                    as *const libc::c_char,
                (*file).number,
                (*iso9660).pathname.s,
                (*file).offset as intmax_t,
                (*iso9660).current_position as intmax_t,
            );
            (*iso9660).entry_bytes_remaining = 0 as libc::c_int as int64_t;
            return -(20 as libc::c_int);
        }
        (*iso9660).current_position = r64 as uint64_t
    }
    /* Initialize zisofs variables. */
    (*iso9660).entry_zisofs.pz = (*file).pz;
    if (*file).pz != 0 {
        let mut zisofs: *mut zisofs = 0 as *mut zisofs;
        zisofs = &mut (*iso9660).entry_zisofs;
        (*zisofs).initialized = 0 as libc::c_int;
        (*zisofs).pz_log2_bs = (*file).pz_log2_bs;
        (*zisofs).pz_uncompressed_size = (*file).pz_uncompressed_size;
        (*zisofs).pz_offset = 0 as libc::c_int as uint32_t;
        (*zisofs).header_avail = 0 as libc::c_int as size_t;
        (*zisofs).header_passed = 0 as libc::c_int;
        (*zisofs).block_pointers_avail = 0 as libc::c_int as size_t;
        archive_entry_set_size(entry, (*file).pz_uncompressed_size as la_int64_t);
    }
    (*iso9660).previous_number = (*file).number;
    if (*iso9660).seenJoliet != 0 {
        memcpy(
            (*iso9660).utf16be_previous_path as *mut libc::c_void,
            (*iso9660).utf16be_path as *const libc::c_void,
            (*iso9660).utf16be_path_len,
        );
        (*iso9660).utf16be_previous_path_len = (*iso9660).utf16be_path_len
    } else {
        (*iso9660).previous_pathname.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*iso9660).previous_pathname,
            (*iso9660).pathname.s as *const libc::c_void,
            (if (*iso9660).pathname.s.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen((*iso9660).pathname.s)
            }),
        );
    }
    /* Reset entry_bytes_remaining if the file is multi extent. */
    (*iso9660).entry_content = (*file).contents.first;
    if !(*iso9660).entry_content.is_null() {
        (*iso9660).entry_bytes_remaining = (*(*iso9660).entry_content).size as int64_t
    }
    if archive_entry_filetype(entry) == AE_IFDIR as mode_t {
        /* Overwrite nlinks by proper link number which is
         * calculated from number of sub directories. */
        archive_entry_set_nlink(entry, (2 as libc::c_int + (*file).subdirs) as libc::c_uint);
        /* Directory data has been read completely. */
        (*iso9660).entry_bytes_remaining = 0 as libc::c_int as int64_t
    }
    if rd_r != ARCHIVE_OK {
        return rd_r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_iso9660_read_data_skip(
    mut a: *mut archive_read,
) -> libc::c_int {
    /* Because read_next_header always does an explicit skip
     * to the next entry, we don't need to do anything here. */
    /* UNUSED */
    return 0 as libc::c_int;
}
unsafe extern "C" fn zisofs_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut zisofs: *mut zisofs = 0 as *mut zisofs;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: size_t = 0;
    let mut bytes_read: ssize_t = 0;
    let mut uncompressed_size: size_t = 0;
    let mut r: libc::c_int = 0;
    iso9660 = (*(*a).format).data as *mut iso9660;
    zisofs = &mut (*iso9660).entry_zisofs;
    p = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read)
        as *const libc::c_uchar;
    if bytes_read <= 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated zisofs file body\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if bytes_read > (*iso9660).entry_bytes_remaining {
        bytes_read = (*iso9660).entry_bytes_remaining
    }
    avail = bytes_read as size_t;
    uncompressed_size = 0 as libc::c_int as size_t;
    if (*zisofs).initialized == 0 {
        let mut ceil: size_t = 0;
        let mut xsize: size_t = 0;
        /* We need more data. */
        /* Allocate block pointers buffer. */
        ceil = (*zisofs)
            .pz_uncompressed_size
            .wrapping_add(((1 as libc::c_int as int64_t) << (*zisofs).pz_log2_bs) as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            >> (*zisofs).pz_log2_bs;
        xsize = ceil
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong);
        if (*zisofs).block_pointers_alloc < xsize {
            let mut alloc: size_t = 0;
            if !(*zisofs).block_pointers.is_null() {
                free((*zisofs).block_pointers as *mut libc::c_void);
            }
            alloc = (xsize >> 10 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong)
                << 10 as libc::c_int;
            (*zisofs).block_pointers = malloc(alloc) as *mut libc::c_uchar;
            if (*zisofs).block_pointers.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory for zisofs decompression\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            (*zisofs).block_pointers_alloc = alloc
        }
        (*zisofs).block_pointers_size = xsize;
        /* Allocate uncompressed data buffer. */
        xsize = (1 as libc::c_ulong) << (*zisofs).pz_log2_bs;
        if (*zisofs).uncompressed_buffer_size < xsize {
            if !(*zisofs).uncompressed_buffer.is_null() {
                free((*zisofs).uncompressed_buffer as *mut libc::c_void);
            }
            (*zisofs).uncompressed_buffer = malloc(xsize) as *mut libc::c_uchar;
            if (*zisofs).uncompressed_buffer.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory for zisofs decompression\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        (*zisofs).uncompressed_buffer_size = xsize;
        /*
         * Read the file header, and check the magic code of zisofs.
         */
        if (*zisofs).header_avail < ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong {
            xsize = (::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong)
                .wrapping_sub((*zisofs).header_avail);
            if avail < xsize {
                xsize = avail
            }
            memcpy(
                (*zisofs)
                    .header
                    .as_mut_ptr()
                    .offset((*zisofs).header_avail as isize) as *mut libc::c_void,
                p as *const libc::c_void,
                xsize,
            );
            (*zisofs).header_avail =
                ((*zisofs).header_avail as libc::c_ulong).wrapping_add(xsize) as size_t as size_t;
            avail = (avail as libc::c_ulong).wrapping_sub(xsize) as size_t as size_t;
            p = p.offset(xsize as isize)
        }
        if (*zisofs).header_passed == 0
            && (*zisofs).header_avail
                == ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong
        {
            let mut err: libc::c_int = 0 as libc::c_int;
            if memcmp(
                (*zisofs).header.as_mut_ptr() as *const libc::c_void,
                zisofs_magic.as_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                err = 1 as libc::c_int
            }
            if archive_le32dec(
                (*zisofs)
                    .header
                    .as_mut_ptr()
                    .offset(8 as libc::c_int as isize) as *const libc::c_void,
            ) as libc::c_ulong
                != (*zisofs).pz_uncompressed_size
            {
                err = 1 as libc::c_int
            }
            if (*zisofs).header[12 as libc::c_int as usize] as libc::c_int != 4 as libc::c_int {
                err = 1 as libc::c_int
            }
            if (*zisofs).header[13 as libc::c_int as usize] as libc::c_int != (*zisofs).pz_log2_bs {
                err = 1 as libc::c_int
            }
            if err != 0 {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Illegal zisofs file body\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            (*zisofs).header_passed = 1 as libc::c_int
        }
        /*
         * Read block pointers.
         */
        if (*zisofs).header_passed != 0
            && (*zisofs).block_pointers_avail < (*zisofs).block_pointers_size
        {
            xsize = (*zisofs)
                .block_pointers_size
                .wrapping_sub((*zisofs).block_pointers_avail);
            if avail < xsize {
                xsize = avail
            }
            memcpy(
                (*zisofs)
                    .block_pointers
                    .offset((*zisofs).block_pointers_avail as isize)
                    as *mut libc::c_void,
                p as *const libc::c_void,
                xsize,
            );
            (*zisofs).block_pointers_avail = ((*zisofs).block_pointers_avail as libc::c_ulong)
                .wrapping_add(xsize) as size_t
                as size_t;
            avail = (avail as libc::c_ulong).wrapping_sub(xsize) as size_t as size_t;
            p = p.offset(xsize as isize);
            if (*zisofs).block_pointers_avail == (*zisofs).block_pointers_size {
                /* We've got all block pointers and initialize
                 * related variables.	*/
                (*zisofs).block_off = 0 as libc::c_int as size_t;
                (*zisofs).block_avail = 0 as libc::c_int as uint32_t;
                /* Complete a initialization */
                (*zisofs).initialized = 1 as libc::c_int
            }
        }
        if (*zisofs).initialized == 0 {
            current_block = 4190662929543724231;
        } else {
            current_block = 14298507163138330979;
        }
    } else {
        current_block = 14298507163138330979;
    }
    match current_block {
        14298507163138330979 => {
            /*
             * Get block offsets from block pointers.
             */
            if (*zisofs).block_avail == 0 as libc::c_int as libc::c_uint {
                let mut bst: uint32_t = 0;
                let mut bed: uint32_t = 0;
                if (*zisofs)
                    .block_off
                    .wrapping_add(4 as libc::c_int as libc::c_ulong)
                    >= (*zisofs).block_pointers_size
                {
                    /* There isn't a pair of offsets. */
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Illegal zisofs block pointers\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                bst = archive_le32dec(
                    (*zisofs)
                        .block_pointers
                        .offset((*zisofs).block_off as isize)
                        as *const libc::c_void,
                );
                if bst as libc::c_ulong
                    != ((*zisofs).pz_offset as libc::c_ulong)
                        .wrapping_add((bytes_read as libc::c_ulong).wrapping_sub(avail))
                {
                    /* TODO: Should we seek offset of current file
                     * by bst ? */
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Illegal zisofs block pointers(cannot seek)\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                bed = archive_le32dec(
                    (*zisofs)
                        .block_pointers
                        .offset((*zisofs).block_off as isize)
                        .offset(4 as libc::c_int as isize)
                        as *const libc::c_void,
                );
                if bed < bst {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Illegal zisofs block pointers\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                (*zisofs).block_avail = bed.wrapping_sub(bst);
                (*zisofs).block_off = ((*zisofs).block_off as libc::c_ulong)
                    .wrapping_add(4 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                /* Initialize compression library for new block. */
                if (*zisofs).stream_valid != 0 {
                    r = inflateReset(&mut (*zisofs).stream)
                } else {
                    r = inflateInit_(
                        &mut (*zisofs).stream,
                        ZLIB_VERSION.as_ptr(),
                        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
                    )
                }
                if r != Z_OK {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Can\'t initialize zisofs decompression.\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                (*zisofs).stream_valid = 1 as libc::c_int;
                (*zisofs).stream.total_in = 0 as libc::c_int as uLong;
                (*zisofs).stream.total_out = 0 as libc::c_int as uLong
            }
            /*
             * Make uncompressed data.
             */
            if (*zisofs).block_avail == 0 as libc::c_int as libc::c_uint {
                memset(
                    (*zisofs).uncompressed_buffer as *mut libc::c_void,
                    0 as libc::c_int,
                    (*zisofs).uncompressed_buffer_size,
                );
                uncompressed_size = (*zisofs).uncompressed_buffer_size
            } else {
                (*zisofs).stream.next_in = p as *const libc::c_void as uintptr_t as *mut Bytef;
                if avail > (*zisofs).block_avail as libc::c_ulong {
                    (*zisofs).stream.avail_in = (*zisofs).block_avail
                } else {
                    (*zisofs).stream.avail_in = avail as uInt
                }
                (*zisofs).stream.next_out = (*zisofs).uncompressed_buffer;
                (*zisofs).stream.avail_out = (*zisofs).uncompressed_buffer_size as uInt;
                r = inflate(&mut (*zisofs).stream, 0 as libc::c_int);
                match r {
                    Z_OK => {}
                    Z_STREAM_END => {}
                    _ => {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"zisofs decompression failed (%d)\x00" as *const u8
                                as *const libc::c_char,
                            r,
                        );
                        return -(30 as libc::c_int);
                    }
                }
                uncompressed_size = (*zisofs)
                    .uncompressed_buffer_size
                    .wrapping_sub((*zisofs).stream.avail_out as libc::c_ulong);
                avail =
                    (avail as libc::c_ulong)
                        .wrapping_sub((*zisofs).stream.next_in.offset_from(p)
                            as libc::c_long as libc::c_ulong) as size_t
                        as size_t;
                (*zisofs).block_avail =
                    ((*zisofs).block_avail as libc::c_uint)
                        .wrapping_sub((*zisofs).stream.next_in.offset_from(p)
                            as libc::c_long as uint32_t) as uint32_t as uint32_t
            }
        }
        _ => {}
    }
    bytes_read = (bytes_read as libc::c_ulong).wrapping_sub(avail) as ssize_t as ssize_t;
    *buff = (*zisofs).uncompressed_buffer as *const libc::c_void;
    *size = uncompressed_size;
    *offset = (*iso9660).entry_sparse_offset;
    (*iso9660).entry_sparse_offset = ((*iso9660).entry_sparse_offset as libc::c_ulong)
        .wrapping_add(uncompressed_size) as int64_t as int64_t;
    (*iso9660).entry_bytes_remaining -= bytes_read;
    (*iso9660).current_position = ((*iso9660).current_position as libc::c_ulong)
        .wrapping_add(bytes_read as libc::c_ulong) as uint64_t
        as uint64_t;
    (*zisofs).pz_offset = ((*zisofs).pz_offset as libc::c_uint).wrapping_add(bytes_read as uint32_t)
        as uint32_t as uint32_t;
    (*iso9660).entry_bytes_unconsumed = ((*iso9660).entry_bytes_unconsumed as libc::c_ulong)
        .wrapping_add(bytes_read as libc::c_ulong) as size_t
        as size_t;
    return 0 as libc::c_int;
}
/* HAVE_ZLIB_H */
/* HAVE_ZLIB_H */
unsafe extern "C" fn archive_read_format_iso9660_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut bytes_read: ssize_t = 0;
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    iso9660 = (*(*a).format).data as *mut iso9660;
    if (*iso9660).entry_bytes_unconsumed != 0 {
        __archive_read_consume(a, (*iso9660).entry_bytes_unconsumed as int64_t);
        (*iso9660).entry_bytes_unconsumed = 0 as libc::c_int as size_t
    }
    if (*iso9660).entry_bytes_remaining <= 0 as libc::c_int as libc::c_long {
        if !(*iso9660).entry_content.is_null() {
            (*iso9660).entry_content = (*(*iso9660).entry_content).next
        }
        if (*iso9660).entry_content.is_null() {
            *buff = NULL as *const libc::c_void;
            *size = 0 as libc::c_int as size_t;
            *offset = (*iso9660).entry_sparse_offset;
            return 1 as libc::c_int;
        }
        /* Seek forward to the start of the entry. */
        if (*iso9660).current_position < (*(*iso9660).entry_content).offset {
            let mut step: int64_t = 0;
            step = (*(*iso9660).entry_content)
                .offset
                .wrapping_sub((*iso9660).current_position) as int64_t;
            step = __archive_read_consume(a, step);
            if step < 0 as libc::c_int as libc::c_long {
                return step as libc::c_int;
            }
            (*iso9660).current_position = (*(*iso9660).entry_content).offset
        }
        if (*(*iso9660).entry_content).offset < (*iso9660).current_position {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Ignoring out-of-order file (%s) %jd < %jd\x00" as *const u8
                    as *const libc::c_char,
                (*iso9660).pathname.s,
                (*(*iso9660).entry_content).offset as intmax_t,
                (*iso9660).current_position as intmax_t,
            );
            *buff = NULL as *const libc::c_void;
            *size = 0 as libc::c_int as size_t;
            *offset = (*iso9660).entry_sparse_offset;
            return -(20 as libc::c_int);
        }
        (*iso9660).entry_bytes_remaining = (*(*iso9660).entry_content).size as int64_t
    }
    if (*iso9660).entry_zisofs.pz != 0 {
        return zisofs_read_data(a, buff, size, offset);
    }
    *buff = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read);
    if bytes_read == 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Truncated input file\x00" as *const u8 as *const libc::c_char,
        );
    }
    if *buff == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    if bytes_read > (*iso9660).entry_bytes_remaining {
        bytes_read = (*iso9660).entry_bytes_remaining
    }
    *size = bytes_read as size_t;
    *offset = (*iso9660).entry_sparse_offset;
    (*iso9660).entry_sparse_offset += bytes_read;
    (*iso9660).entry_bytes_remaining -= bytes_read;
    (*iso9660).entry_bytes_unconsumed = bytes_read as size_t;
    (*iso9660).current_position = ((*iso9660).current_position as libc::c_ulong)
        .wrapping_add(bytes_read as libc::c_ulong) as uint64_t
        as uint64_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_iso9660_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut r: libc::c_int = ARCHIVE_OK;
    iso9660 = (*(*a).format).data as *mut iso9660;
    release_files(iso9660);
    free((*iso9660).read_ce_req.reqs as *mut libc::c_void);
    archive_string_free(&mut (*iso9660).pathname);
    archive_string_free(&mut (*iso9660).previous_pathname);
    free((*iso9660).pending_files.files as *mut libc::c_void);
    free((*iso9660).entry_zisofs.uncompressed_buffer as *mut libc::c_void);
    free((*iso9660).entry_zisofs.block_pointers as *mut libc::c_void);
    if (*iso9660).entry_zisofs.stream_valid != 0 {
        if inflateEnd(&mut (*iso9660).entry_zisofs.stream) != Z_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Failed to clean up zlib decompressor\x00" as *const u8 as *const libc::c_char,
            );
            r = ARCHIVE_FATAL
        }
    }
    free((*iso9660).utf16be_path as *mut libc::c_void);
    free((*iso9660).utf16be_previous_path as *mut libc::c_void);
    free(iso9660 as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return r;
}
/*
 * This routine parses a single ISO directory record, makes sense
 * of any extensions, and stores the result in memory.
 */
unsafe extern "C" fn parse_file_info(
    mut a: *mut archive_read,
    mut parent: *mut file_info,
    mut isodirrec: *const libc::c_uchar,
    mut reclen: size_t,
) -> *mut file_info {
    let mut current_block: u64;
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut file: *mut file_info = 0 as *mut file_info;
    let mut filep: *mut file_info = 0 as *mut file_info;
    let mut name_len: size_t = 0;
    let mut rr_start: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut rr_end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut dr_len: size_t = 0;
    let mut fsize: uint64_t = 0;
    let mut offset: uint64_t = 0;
    let mut location: int32_t = 0;
    let mut flags: libc::c_int = 0;
    iso9660 = (*(*a).format).data as *mut iso9660;
    if reclen != 0 as libc::c_int as libc::c_ulong {
        dr_len = *isodirrec.offset(DR_length_offset as isize) as size_t
    }
    /*
     * Sanity check that reclen is not zero and dr_len is greater than
     * reclen but at least 34
     */
    if reclen == 0 as libc::c_int as libc::c_ulong
        || reclen < dr_len
        || dr_len < 34 as libc::c_int as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Invalid length of directory record\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut file_info;
    }
    name_len = *isodirrec.offset(DR_name_len_offset as isize) as size_t;
    location = archive_le32dec(isodirrec.offset(DR_extent_offset as isize) as *const libc::c_void)
        as int32_t;
    fsize = toi(
        isodirrec.offset(DR_size_offset as isize) as *const libc::c_void,
        DR_size_size,
    ) as uint64_t;
    /* Sanity check that name_len doesn't exceed dr_len. */
    if dr_len.wrapping_sub(33 as libc::c_int as libc::c_ulong) < name_len
        || name_len == 0 as libc::c_int as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Invalid length of file identifier\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut file_info;
    }
    /* Sanity check that location doesn't exceed volume block.
     * Don't check lower limit of location; it's possibility
     * the location has negative value when file type is symbolic
     * link or file size is zero. As far as I know latest mkisofs
     * do that.
     */
    if location > 0 as libc::c_int
        && (location as libc::c_ulong).wrapping_add(
            fsize
                .wrapping_add((*iso9660).logical_block_size as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div((*iso9660).logical_block_size as libc::c_ulong),
        ) > (*iso9660).volume_block as uint32_t as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Invalid location of extent of file\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut file_info;
    }
    /* Sanity check that location doesn't have a negative value
     * when the file is not empty. it's too large. */
    if fsize != 0 as libc::c_int as libc::c_ulong && location < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Invalid location of extent of file\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut file_info;
    }
    /* Sanity check that this entry does not create a cycle. */
    offset = ((*iso9660).logical_block_size as libc::c_ulong).wrapping_mul(location as uint64_t);
    filep = parent;
    while !filep.is_null() {
        if (*filep).offset == offset {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Directory structure contains loop\x00" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut file_info;
        }
        filep = (*filep).parent
    }
    /* Create a new file entry and copy data from the ISO dir record. */
    file = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<file_info>() as libc::c_ulong,
    ) as *mut file_info;
    if file.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"No memory for file entry\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut file_info;
    }
    (*file).parent = parent;
    (*file).offset = offset;
    (*file).size = fsize;
    (*file).mtime = isodate7(isodirrec.offset(DR_date_offset as isize));
    (*file).atime = (*file).mtime;
    (*file).ctime = (*file).atime;
    (*file).rede_files.first = NULL as *mut file_info;
    (*file).rede_files.last = &mut (*file).rede_files.first;
    p = isodirrec.offset(DR_name_offset as isize);
    /* Rockridge extensions (if any) follow name.  Compute this
     * before fidgeting the name_len below. */
    rr_start = p.offset(name_len as isize).offset(
        (if name_len & 1 as libc::c_int as libc::c_ulong != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as isize,
    );
    rr_end = isodirrec.offset(dr_len as isize);
    if (*iso9660).seenJoliet != 0 {
        /* Joliet names are max 64 chars (128 bytes) according to spec,
         * but genisoimage/mkisofs allows recording longer Joliet
         * names which are 103 UCS2 characters(206 bytes) by their
         * option '-joliet-long'.
         */
        if name_len > 206 as libc::c_int as libc::c_ulong {
            name_len = 206 as libc::c_int as size_t
        }
        name_len &= !(1 as libc::c_int) as libc::c_ulong;
        /* trim trailing first version and dot from filename.
         *
         * Remember we were in UTF-16BE land!
         * SEPARATOR 1 (.) and SEPARATOR 2 (;) are both
         * 16 bits big endian characters on Joliet.
         *
         * TODO: sanitize filename?
         *       Joliet allows any UCS-2 char except:
         *       *, /, :, ;, ? and \.
         */
        /* Chop off trailing ';1' from files. */
        if name_len > 4 as libc::c_int as libc::c_ulong
            && *p.offset(name_len.wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == 0 as libc::c_int
            && *p.offset(name_len.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == ';' as i32
            && *p.offset(name_len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == 0 as libc::c_int
            && *p.offset(name_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '1' as i32
        {
            name_len = (name_len as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t
        }
        /* XXX: this somehow manages to strip of single-character file extensions, like '.c'. */
        (*file).utf16be_name = malloc(name_len) as *mut libc::c_uchar;
        if (*file).utf16be_name.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory for file name\x00" as *const u8 as *const libc::c_char,
            );
            current_block = 14180276789801570031;
        } else {
            memcpy(
                (*file).utf16be_name as *mut libc::c_void,
                p as *const libc::c_void,
                name_len,
            );
            (*file).utf16be_bytes = name_len;
            current_block = 6528285054092551010;
        }
    } else {
        /* Chop off trailing ';1' from files. */
        if name_len > 2 as libc::c_int as libc::c_ulong
            && *p.offset(name_len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == ';' as i32
            && *p.offset(name_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '1' as i32
        {
            name_len = (name_len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t
        }
        /* Chop off trailing '.' from filenames. */
        if name_len > 1 as libc::c_int as libc::c_ulong
            && *p.offset(name_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '.' as i32
        {
            name_len = name_len.wrapping_sub(1)
        }
        (*file).name.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*file).name,
            p as *const libc::c_char as *const libc::c_void,
            name_len,
        );
        current_block = 6528285054092551010;
    }
    match current_block {
        6528285054092551010 => {
            flags = *isodirrec.offset(DR_flags_offset as isize) as libc::c_int;
            if flags & 0x2 as libc::c_int != 0 {
                (*file).mode = AE_IFDIR as mode_t | 0o700 as libc::c_int as libc::c_uint
            } else {
                (*file).mode = AE_IFREG as mode_t | 0o400 as libc::c_int as libc::c_uint
            }
            if flags & 0x80 as libc::c_int != 0 {
                (*file).multi_extent = 1 as libc::c_int
            } else {
                (*file).multi_extent = 0 as libc::c_int
            }
            /*
             * Use a location for the file number, which is treated as an inode
             * number to find out hardlink target. If Rockridge extensions is
             * being used, the file number will be overwritten by FILE SERIAL
             * NUMBER of RRIP "PX" extension.
             * Note: Old mkisofs did not record that FILE SERIAL NUMBER
             * in ISO images.
             * Note2: xorriso set 0 to the location of a symlink file.
             */
            if (*file).size == 0 as libc::c_int as libc::c_ulong && location >= 0 as libc::c_int {
                /* If file->size is zero, its location points wrong place,
                 * and so we should not use it for the file number.
                 * When the location has negative value, it can be used
                 * for the file number.
                 */
                (*file).number = -(1 as libc::c_int) as int64_t;
                /* Do not appear before any directory entries. */
                (*file).offset = -(1 as libc::c_int) as uint64_t
            } else {
                (*file).number = location as uint32_t as int64_t
            }
            /* Rockridge extensions overwrite information from above. */
            if (*iso9660).opt_support_rockridge != 0 {
                if parent.is_null()
                    && rr_end.offset_from(rr_start) as libc::c_long
                        >= 7 as libc::c_int as libc::c_long
                {
                    p = rr_start;
                    if memcmp(
                        p as *const libc::c_void,
                        b"SP\x07\x01\xbe\xef\x00" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        /*
                         * SP extension stores the suspOffset
                         * (Number of bytes to skip between
                         * filename and SUSP records.)
                         * It is mandatory by the SUSP standard
                         * (IEEE 1281).
                         *
                         * It allows SUSP to coexist with
                         * non-SUSP uses of the System
                         * Use Area by placing non-SUSP data
                         * before SUSP data.
                         *
                         * SP extension must be in the root
                         * directory entry, disable all SUSP
                         * processing if not found.
                         */
                        (*iso9660).suspOffset = *p.offset(6 as libc::c_int as isize);
                        (*iso9660).seenSUSP = 1 as libc::c_int as libc::c_char;
                        rr_start = rr_start.offset(7 as libc::c_int as isize)
                    }
                }
                if (*iso9660).seenSUSP != 0 {
                    let mut r: libc::c_int = 0;
                    (*file).name_continues = 0 as libc::c_int as libc::c_char;
                    (*file).symlink_continues = 0 as libc::c_int as libc::c_char;
                    rr_start = rr_start.offset((*iso9660).suspOffset as libc::c_int as isize);
                    r = parse_rockridge(a, file, rr_start, rr_end);
                    if r != ARCHIVE_OK {
                        current_block = 14180276789801570031;
                    } else {
                        /*
                         * A file size of symbolic link files in ISO images
                         * made by makefs is not zero and its location is
                         * the same as those of next regular file. That is
                         * the same as hard like file and it causes unexpected
                         * error.
                         */
                        if (*file).size > 0 as libc::c_int as libc::c_ulong
                            && (*file).mode & AE_IFMT as mode_t == AE_IFLNK as mode_t
                        {
                            (*file).size = 0 as libc::c_int as uint64_t;
                            (*file).number = -(1 as libc::c_int) as int64_t;
                            (*file).offset = -(1 as libc::c_int) as uint64_t
                        }
                        current_block = 1852451392920375136;
                    }
                } else {
                    /* If there isn't SUSP, disable parsing
                     * rock ridge extensions. */
                    (*iso9660).opt_support_rockridge = 0 as libc::c_int; /* Reset nlink. we'll calculate it later. */
                    current_block = 1852451392920375136;
                }
            } else {
                current_block = 1852451392920375136;
            }
            match current_block {
                14180276789801570031 => {}
                _ => {
                    (*file).nlinks = 1 as libc::c_int;
                    /* Tell file's parent how many children that parent has. */
                    if !parent.is_null() && flags & 0x2 as libc::c_int != 0 {
                        (*parent).subdirs += 1
                    }
                    if (*iso9660).seenRockridge != 0 {
                        if !parent.is_null()
                            && (*parent).parent.is_null()
                            && flags & 0x2 as libc::c_int != 0
                            && (*iso9660).rr_moved.is_null()
                            && !(*file).name.s.is_null()
                            && (strcmp(
                                (*file).name.s,
                                b"rr_moved\x00" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int
                                || strcmp(
                                    (*file).name.s,
                                    b".rr_moved\x00" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int)
                        {
                            (*iso9660).rr_moved = file;
                            (*file).rr_moved = 1 as libc::c_int as libc::c_char;
                            (*file).rr_moved_has_re_only = 1 as libc::c_int as libc::c_char;
                            (*file).re = 0 as libc::c_int as libc::c_char;
                            (*parent).subdirs -= 1;
                            current_block = 6733407218104445560;
                        } else if (*file).re != 0 {
                            /*
                             * Sanity check: file's parent is rr_moved.
                             */
                            if parent.is_null()
                                || (*parent).rr_moved as libc::c_int == 0 as libc::c_int
                            {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_MISC,
                                    b"Invalid Rockridge RE\x00" as *const u8 as *const libc::c_char,
                                );
                                current_block = 14180276789801570031;
                            } else if (*file).cl_offset != 0 {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_MISC,
                                    b"Invalid Rockridge RE and CL\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                current_block = 14180276789801570031;
                            } else if flags & 0x2 as libc::c_int == 0 as libc::c_int {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_MISC,
                                    b"Invalid Rockridge RE\x00" as *const u8 as *const libc::c_char,
                                );
                                current_block = 14180276789801570031;
                            } else {
                                current_block = 6733407218104445560;
                            }
                        } else {
                            if !parent.is_null() && (*parent).rr_moved as libc::c_int != 0 {
                                (*file).rr_moved_has_re_only = 0 as libc::c_int as libc::c_char
                            } else if !parent.is_null()
                                && flags & 0x2 as libc::c_int != 0
                                && ((*parent).re as libc::c_int != 0
                                    || (*parent).re_descendant as libc::c_int != 0)
                            {
                                (*file).re_descendant = 1 as libc::c_int as libc::c_char
                            }
                            current_block = 6733407218104445560;
                        }
                        match current_block {
                            14180276789801570031 => {}
                            _ => {
                                if (*file).cl_offset != 0 {
                                    let mut r_0: *mut file_info = 0 as *mut file_info;
                                    if parent.is_null() || (*parent).parent.is_null() {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_MISC,
                                            b"Invalid Rockridge CL\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 14180276789801570031;
                                    } else if flags & 0x2 as libc::c_int != 0 as libc::c_int {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_MISC,
                                            b"Invalid Rockridge CL\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block = 14180276789801570031;
                                    } else {
                                        (*parent).subdirs += 1;
                                        /*
                                         * Sanity check: file does not have "CL" extension.
                                         */
                                        /*
                                         * Sanity check: The file type must be a directory.
                                         */
                                        /*
                                         * Sanity check: The file type must be a regular file.
                                         */
                                        /* Overwrite an offset and a number of this "CL" entry
                                         * to appear before other dirs. "+1" to those is to
                                         * make sure to appear after "RE" entry which this
                                         * "CL" entry should be connected with. */
                                        (*file).number = (*file)
                                            .cl_offset
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as int64_t;
                                        (*file).offset = (*file).number as uint64_t;
                                        /*
                                         * Sanity check: cl_offset does not point at its
                                         * the parents or itself.
                                         */
                                        r_0 = parent;
                                        loop {
                                            if r_0.is_null() {
                                                current_block = 16696653877814833746;
                                                break;
                                            }
                                            if (*r_0).offset == (*file).cl_offset {
                                                archive_set_error(
                                                    &mut (*a).archive as *mut archive,
                                                    ARCHIVE_ERRNO_MISC,
                                                    b"Invalid Rockridge CL\x00" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                current_block = 14180276789801570031;
                                                break;
                                            } else {
                                                r_0 = (*r_0).parent
                                            }
                                        }
                                        match current_block {
                                            14180276789801570031 => {}
                                            _ => {
                                                if (*file).cl_offset == (*file).offset
                                                    || (*parent).rr_moved as libc::c_int != 0
                                                {
                                                    archive_set_error(
                                                        &mut (*a).archive as *mut archive,
                                                        ARCHIVE_ERRNO_MISC,
                                                        b"Invalid Rockridge CL\x00" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    current_block = 14180276789801570031;
                                                } else {
                                                    current_block = 248631179418912492;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    current_block = 248631179418912492;
                                }
                            }
                        }
                    } else {
                        current_block = 248631179418912492;
                    }
                    match current_block {
                        14180276789801570031 => {}
                        _ => {
                            register_file(iso9660, file);
                            return file;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    archive_string_free(&mut (*file).name);
    free(file as *mut libc::c_void);
    return 0 as *mut file_info;
}
unsafe extern "C" fn parse_rockridge(
    mut a: *mut archive_read,
    mut file: *mut file_info,
    mut p: *const libc::c_uchar,
    mut end: *const libc::c_uchar,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut entry_seen: libc::c_int = 0 as libc::c_int;
    iso9660 = (*(*a).format).data as *mut iso9660;
    while p.offset(4 as libc::c_int as isize) <= end
        && *p.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
        && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int >= 'A' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
        && *p.offset(2 as libc::c_int as isize) as libc::c_int >= 4 as libc::c_int
        && p.offset(*p.offset(2 as libc::c_int as isize) as libc::c_int as isize) <= end
    {
        /* Sanity-check length. */
        let mut data: *const libc::c_uchar = p.offset(4 as libc::c_int as isize);
        let mut data_length: libc::c_int =
            *p.offset(2 as libc::c_int as isize) as libc::c_int - 4 as libc::c_int;
        let mut version: libc::c_int = *p.offset(3 as libc::c_int as isize) as libc::c_int;
        match *p.offset(0 as libc::c_int as isize) as libc::c_int {
            67 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'E' as i32 {
                    if version == 1 as libc::c_int && data_length == 24 as libc::c_int {
                        /*
                         * CE extension comprises:
                         *   8 byte sector containing extension
                         *   8 byte offset w/in above sector
                         *   8 byte length of continuation
                         */
                        let mut location: int32_t =
                            archive_le32dec(data as *const libc::c_void) as int32_t;
                        (*file).ce_offset = archive_le32dec(
                            data.offset(8 as libc::c_int as isize) as *const libc::c_void
                        );
                        (*file).ce_size = archive_le32dec(
                            data.offset(16 as libc::c_int as isize) as *const libc::c_void
                        );
                        if register_CE(a, location, file) != ARCHIVE_OK {
                            return -(30 as libc::c_int);
                        }
                    }
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'L' as i32 {
                    if version == 1 as libc::c_int && data_length == 8 as libc::c_int {
                        (*file).cl_offset = ((*iso9660).logical_block_size as uint64_t)
                            .wrapping_mul(archive_le32dec(data as *const libc::c_void) as uint64_t);
                        (*iso9660).seenRockridge = 1 as libc::c_int as libc::c_char
                    }
                }
            }
            78 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'M' as i32 {
                    if version == 1 as libc::c_int {
                        parse_rockridge_NM1(file, data, data_length);
                        (*iso9660).seenRockridge = 1 as libc::c_int as libc::c_char
                    }
                }
            }
            80 => {
                /*
                 * PD extension is padding;
                 * contents are always ignored.
                 *
                 * PL extension won't appear;
                 * contents are always ignored.
                 */
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'N' as i32 {
                    if version == 1 as libc::c_int && data_length == 16 as libc::c_int {
                        (*file).rdev =
                            toi(data as *const libc::c_void, 4 as libc::c_int) as uint64_t;
                        (*file).rdev <<= 32 as libc::c_int;
                        (*file).rdev |= toi(
                            data.offset(8 as libc::c_int as isize) as *const libc::c_void,
                            4 as libc::c_int,
                        ) as libc::c_ulong;
                        (*iso9660).seenRockridge = 1 as libc::c_int as libc::c_char
                    }
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32 {
                    /*
                     * PX extension comprises:
                     *   8 bytes for mode,
                     *   8 bytes for nlinks,
                     *   8 bytes for uid,
                     *   8 bytes for gid,
                     *   8 bytes for inode.
                     */
                    if version == 1 as libc::c_int {
                        if data_length >= 8 as libc::c_int {
                            (*file).mode = toi(data as *const libc::c_void, 4 as libc::c_int)
                        }
                        if data_length >= 16 as libc::c_int {
                            (*file).nlinks = toi(
                                data.offset(8 as libc::c_int as isize) as *const libc::c_void,
                                4 as libc::c_int,
                            ) as libc::c_int
                        }
                        if data_length >= 24 as libc::c_int {
                            (*file).uid = toi(
                                data.offset(16 as libc::c_int as isize) as *const libc::c_void,
                                4 as libc::c_int,
                            )
                        }
                        if data_length >= 32 as libc::c_int {
                            (*file).gid = toi(
                                data.offset(24 as libc::c_int as isize) as *const libc::c_void,
                                4 as libc::c_int,
                            )
                        }
                        if data_length >= 40 as libc::c_int {
                            (*file).number = toi(
                                data.offset(32 as libc::c_int as isize) as *const libc::c_void,
                                4 as libc::c_int,
                            ) as int64_t
                        }
                        (*iso9660).seenRockridge = 1 as libc::c_int as libc::c_char
                    }
                }
            }
            82 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'E' as i32
                    && version == 1 as libc::c_int
                {
                    (*file).re = 1 as libc::c_int as libc::c_char;
                    (*iso9660).seenRockridge = 1 as libc::c_int as libc::c_char
                } else {
                    (*p.offset(1 as libc::c_int as isize) as libc::c_int == 'R' as i32)
                        && version == 1 as libc::c_int;
                }
            }
            83 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'L' as i32 {
                    if version == 1 as libc::c_int {
                        parse_rockridge_SL1(file, data, data_length);
                        (*iso9660).seenRockridge = 1 as libc::c_int as libc::c_char
                    }
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'T' as i32
                    && data_length == 0 as libc::c_int
                    && version == 1 as libc::c_int
                {
                    /*
                     * ST extension marks end of this
                     * block of SUSP entries.
                     *
                     * It allows SUSP to coexist with
                     * non-SUSP uses of the System
                     * Use Area by placing non-SUSP data
                     * after SUSP data.
                     */
                    (*iso9660).seenSUSP = 0 as libc::c_int as libc::c_char;
                    (*iso9660).seenRockridge = 0 as libc::c_int as libc::c_char;
                    return 0 as libc::c_int;
                }
            }
            84 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'F' as i32 {
                    if version == 1 as libc::c_int {
                        parse_rockridge_TF1(file, data, data_length);
                        (*iso9660).seenRockridge = 1 as libc::c_int as libc::c_char
                    }
                }
            }
            90 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'F' as i32 {
                    if version == 1 as libc::c_int {
                        parse_rockridge_ZF1(file, data, data_length);
                    }
                }
            }
            _ => {}
        }
        p = p.offset(*p.offset(2 as libc::c_int as isize) as libc::c_int as isize);
        entry_seen = 1 as libc::c_int
    }
    if entry_seen != 0 {
        return 0 as libc::c_int;
    } else {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Tried to parse Rockridge extensions, but none found\x00" as *const u8
                as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    };
}
unsafe extern "C" fn register_CE(
    mut a: *mut archive_read,
    mut location: int32_t,
    mut file: *mut file_info,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut heap: *mut read_ce_queue = 0 as *mut read_ce_queue;
    let mut p: *mut read_ce_req = 0 as *mut read_ce_req;
    let mut offset: uint64_t = 0;
    let mut parent_offset: uint64_t = 0;
    let mut hole: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    iso9660 = (*(*a).format).data as *mut iso9660;
    offset = (location as uint64_t).wrapping_mul((*iso9660).logical_block_size as uint64_t);
    if (*file).mode & AE_IFMT as mode_t == AE_IFREG as mode_t && offset >= (*file).offset
        || offset < (*iso9660).current_position
        || ((*file).ce_offset as uint64_t).wrapping_add((*file).ce_size as libc::c_ulong)
            > (*iso9660).logical_block_size as uint64_t
        || offset
            .wrapping_add((*file).ce_offset as libc::c_ulong)
            .wrapping_add((*file).ce_size as libc::c_ulong)
            > (*iso9660).volume_size
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Invalid parameter in SUSP \"CE\" extension\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Expand our CE list as necessary. */
    heap = &mut (*iso9660).read_ce_req;
    if (*heap).cnt >= (*heap).allocated {
        let mut new_size: libc::c_int = 0;
        if (*heap).allocated < 16 as libc::c_int {
            new_size = 16 as libc::c_int
        } else {
            new_size = (*heap).allocated * 2 as libc::c_int
        }
        /* Overflow might keep us from growing the list. */
        if new_size <= (*heap).allocated {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        p = calloc(
            new_size as libc::c_ulong,
            ::std::mem::size_of::<read_ce_req>() as libc::c_ulong,
        ) as *mut read_ce_req;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if !(*heap).reqs.is_null() {
            memcpy(
                p as *mut libc::c_void,
                (*heap).reqs as *const libc::c_void,
                ((*heap).cnt as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<read_ce_req>() as libc::c_ulong),
            );
            free((*heap).reqs as *mut libc::c_void);
        }
        (*heap).reqs = p;
        (*heap).allocated = new_size
    }
    /*
     * Start with hole at end, walk it up tree to find insertion point.
     */
    let fresh0 = (*heap).cnt;
    (*heap).cnt = (*heap).cnt + 1;
    hole = fresh0;
    while hole > 0 as libc::c_int {
        parent = (hole - 1 as libc::c_int) / 2 as libc::c_int;
        parent_offset = (*(*heap).reqs.offset(parent as isize)).offset;
        if offset >= parent_offset {
            (*(*heap).reqs.offset(hole as isize)).offset = offset;
            let ref mut fresh1 = (*(*heap).reqs.offset(hole as isize)).file;
            *fresh1 = file;
            return 0 as libc::c_int;
        }
        /* Move parent into hole <==> move hole up tree. */
        *(*heap).reqs.offset(hole as isize) = *(*heap).reqs.offset(parent as isize);
        hole = parent
    }
    (*(*heap).reqs.offset(0 as libc::c_int as isize)).offset = offset;
    let ref mut fresh2 = (*(*heap).reqs.offset(0 as libc::c_int as isize)).file;
    *fresh2 = file;
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_CE(mut heap: *mut read_ce_queue) {
    let mut a_offset: uint64_t = 0;
    let mut b_offset: uint64_t = 0;
    let mut c_offset: uint64_t = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tmp: read_ce_req = read_ce_req {
        offset: 0,
        file: 0 as *mut file_info,
    };
    if (*heap).cnt < 1 as libc::c_int {
        return;
    }
    /*
     * Move the last item in the heap to the root of the tree
     */
    (*heap).cnt -= 1;
    *(*heap).reqs.offset(0 as libc::c_int as isize) = *(*heap).reqs.offset((*heap).cnt as isize);
    /*
     * Rebalance the heap.
     */
    a = 0 as libc::c_int; /* Starting element and its offset */
    a_offset = (*(*heap).reqs.offset(a as isize)).offset; /* First child */
    loop {
        b = a + a + 1 as libc::c_int; /* Use second child if it is smaller. */
        if b >= (*heap).cnt {
            return;
        }
        b_offset = (*(*heap).reqs.offset(b as isize)).offset;
        c = b + 1 as libc::c_int;
        if c < (*heap).cnt {
            c_offset = (*(*heap).reqs.offset(c as isize)).offset;
            if c_offset < b_offset {
                b = c;
                b_offset = c_offset
            }
        }
        if a_offset <= b_offset {
            return;
        }
        tmp = *(*heap).reqs.offset(a as isize);
        *(*heap).reqs.offset(a as isize) = *(*heap).reqs.offset(b as isize);
        *(*heap).reqs.offset(b as isize) = tmp;
        a = b
    }
}
unsafe extern "C" fn read_CE(mut a: *mut archive_read, mut iso9660: *mut iso9660) -> libc::c_int {
    let mut heap: *mut read_ce_queue = 0 as *mut read_ce_queue;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut file: *mut file_info = 0 as *mut file_info;
    let mut step: size_t = 0;
    let mut r: libc::c_int = 0;
    /* Read data which RRIP "CE" extension points. */
    heap = &mut (*iso9660).read_ce_req;
    step = (*iso9660).logical_block_size as size_t;
    while (*heap).cnt != 0
        && (*(*heap).reqs.offset(0 as libc::c_int as isize)).offset == (*iso9660).current_position
    {
        b = __archive_read_ahead(a, step, NULL as *mut ssize_t) as *const libc::c_uchar;
        if b.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Failed to read full block when scanning ISO9660 directory list\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        loop {
            file = (*(*heap).reqs.offset(0 as libc::c_int as isize)).file;
            if (*file).ce_offset.wrapping_add((*file).ce_size) as libc::c_ulong > step {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Malformed CE information\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            p = b.offset((*file).ce_offset as isize);
            end = p.offset((*file).ce_size as isize);
            next_CE(heap);
            r = parse_rockridge(a, file, p, end);
            if r != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
            if !((*heap).cnt != 0
                && (*(*heap).reqs.offset(0 as libc::c_int as isize)).offset
                    == (*iso9660).current_position)
            {
                break;
            }
        }
        /* NOTE: Do not move this consume's code to front of
         * do-while loop. Registration of nested CE extension
         * might cause error because of current position. */
        __archive_read_consume(a, step as int64_t);
        (*iso9660).current_position = ((*iso9660).current_position as libc::c_ulong)
            .wrapping_add(step) as uint64_t as uint64_t
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_rockridge_NM1(
    mut file: *mut file_info,
    mut data: *const libc::c_uchar,
    mut data_length: libc::c_int,
) {
    if (*file).name_continues == 0 {
        (*file).name.length = 0 as libc::c_int as size_t
    }
    (*file).name_continues = 0 as libc::c_int as libc::c_char;
    if data_length < 1 as libc::c_int {
        return;
    }
    /*
     * NM version 1 extension comprises:
     *   1 byte flag, value is one of:
     *     = 0: remainder is name
     *     = 1: remainder is name, next NM entry continues name
     *     = 2: "."
     *     = 4: ".."
     *     = 32: Implementation specific
     *     All other values are reserved.
     */
    match *data.offset(0 as libc::c_int as isize) as libc::c_int {
        0 => {
            if data_length < 2 as libc::c_int {
                return;
            }
            archive_strncat(
                &mut (*file).name,
                (data as *const libc::c_char).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (data_length - 1 as libc::c_int) as size_t,
            );
        }
        1 => {
            if data_length < 2 as libc::c_int {
                return;
            }
            archive_strncat(
                &mut (*file).name,
                (data as *const libc::c_char).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                (data_length - 1 as libc::c_int) as size_t,
            );
            (*file).name_continues = 1 as libc::c_int as libc::c_char
        }
        2 => {
            archive_strcat(
                &mut (*file).name,
                b".\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
        }
        4 => {
            archive_strcat(
                &mut (*file).name,
                b"..\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
        }
        _ => return,
    };
}
unsafe extern "C" fn parse_rockridge_TF1(
    mut file: *mut file_info,
    mut data: *const libc::c_uchar,
    mut data_length: libc::c_int,
) {
    let mut flag: libc::c_char = 0;
    /*
     * TF extension comprises:
     *   one byte flag
     *   create time (optional)
     *   modify time (optional)
     *   access time (optional)
     *   attribute time (optional)
     *  Time format and presence of fields
     *  is controlled by flag bits.
     */
    if data_length < 1 as libc::c_int {
        return;
    }
    flag = *data.offset(0 as libc::c_int as isize) as libc::c_char;
    data = data.offset(1);
    data_length -= 1;
    if flag as libc::c_int & 0x80 as libc::c_int != 0 {
        /* Use 17-byte time format. */
        if flag as libc::c_int & 1 as libc::c_int != 0 && data_length >= 17 as libc::c_int {
            /* Create time. */
            (*file).birthtime_is_set = 1 as libc::c_int;
            (*file).birthtime = isodate17(data);
            data = data.offset(17 as libc::c_int as isize);
            data_length -= 17 as libc::c_int
        }
        if flag as libc::c_int & 2 as libc::c_int != 0 && data_length >= 17 as libc::c_int {
            /* Modify time. */
            (*file).mtime = isodate17(data);
            data = data.offset(17 as libc::c_int as isize);
            data_length -= 17 as libc::c_int
        }
        if flag as libc::c_int & 4 as libc::c_int != 0 && data_length >= 17 as libc::c_int {
            /* Access time. */
            (*file).atime = isodate17(data);
            data = data.offset(17 as libc::c_int as isize);
            data_length -= 17 as libc::c_int
        }
        if flag as libc::c_int & 8 as libc::c_int != 0 && data_length >= 17 as libc::c_int {
            /* Attribute change time. */
            (*file).ctime = isodate17(data)
        }
    } else {
        /* Use 7-byte time format. */
        if flag as libc::c_int & 1 as libc::c_int != 0 && data_length >= 7 as libc::c_int {
            /* Create time. */
            (*file).birthtime_is_set = 1 as libc::c_int;
            (*file).birthtime = isodate7(data);
            data = data.offset(7 as libc::c_int as isize);
            data_length -= 7 as libc::c_int
        }
        if flag as libc::c_int & 2 as libc::c_int != 0 && data_length >= 7 as libc::c_int {
            /* Modify time. */
            (*file).mtime = isodate7(data);
            data = data.offset(7 as libc::c_int as isize);
            data_length -= 7 as libc::c_int
        }
        if flag as libc::c_int & 4 as libc::c_int != 0 && data_length >= 7 as libc::c_int {
            /* Access time. */
            (*file).atime = isodate7(data);
            data = data.offset(7 as libc::c_int as isize);
            data_length -= 7 as libc::c_int
        }
        if flag as libc::c_int & 8 as libc::c_int != 0 && data_length >= 7 as libc::c_int {
            /* Attribute change time. */
            (*file).ctime = isodate7(data)
        }
    };
}
unsafe extern "C" fn parse_rockridge_SL1(
    mut file: *mut file_info,
    mut data: *const libc::c_uchar,
    mut data_length: libc::c_int,
) {
    let mut separator: *const libc::c_char = b"\x00" as *const u8 as *const libc::c_char;
    if (*file).symlink_continues == 0 || (*file).symlink.length < 1 as libc::c_int as libc::c_ulong
    {
        (*file).symlink.length = 0 as libc::c_int as size_t
    }
    (*file).symlink_continues = 0 as libc::c_int as libc::c_char;
    /*
     * Defined flag values:
     *  0: This is the last SL record for this symbolic link
     *  1: this symbolic link field continues in next SL entry
     *  All other values are reserved.
     */
    if data_length < 1 as libc::c_int {
        return;
    } /* Skip flag byte. */
    match *data as libc::c_int {
        0 => {}
        1 => (*file).symlink_continues = 1 as libc::c_int as libc::c_char,
        _ => return,
    }
    data = data.offset(1);
    data_length -= 1;
    /*
     * SL extension body stores "components".
     * Basically, this is a complicated way of storing
     * a POSIX path.  It also interferes with using
     * symlinks for storing non-path data. <sigh>
     *
     * Each component is 2 bytes (flag and length)
     * possibly followed by name data.
     */
    while data_length >= 2 as libc::c_int {
        let fresh3 = data;
        data = data.offset(1);
        let mut flag: libc::c_uchar = *fresh3;
        let fresh4 = data;
        data = data.offset(1);
        let mut nlen: libc::c_uchar = *fresh4;
        data_length -= 2 as libc::c_int;
        archive_strcat(&mut (*file).symlink, separator as *const libc::c_void);
        separator = b"/\x00" as *const u8 as *const libc::c_char;
        match flag as libc::c_int {
            0 => {
                /* Usual case, this is text. */
                if data_length < nlen as libc::c_int {
                    return;
                }
                archive_strncat(
                    &mut (*file).symlink,
                    data as *const libc::c_char as *const libc::c_void,
                    nlen as size_t,
                );
            }
            1 => {
                /* Text continues in next component. */
                if data_length < nlen as libc::c_int {
                    return;
                }
                archive_strncat(
                    &mut (*file).symlink,
                    data as *const libc::c_char as *const libc::c_void,
                    nlen as size_t,
                );
                separator = b"\x00" as *const u8 as *const libc::c_char
            }
            2 => {
                /* Current dir. */
                archive_strcat(
                    &mut (*file).symlink,
                    b".\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            4 => {
                /* Parent dir. */
                archive_strcat(
                    &mut (*file).symlink,
                    b"..\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            8 => {
                /* Root of filesystem. */
                archive_strcat(
                    &mut (*file).symlink,
                    b"/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
                separator = b"\x00" as *const u8 as *const libc::c_char
            }
            16 => {
                /* Undefined (historically "volume root" */
                (*file).symlink.length = 0 as libc::c_int as size_t;
                archive_strcat(
                    &mut (*file).symlink,
                    b"ROOT\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            32 => {
                /* Undefined (historically "hostname") */
                archive_strcat(
                    &mut (*file).symlink,
                    b"hostname\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            _ => {
                /* TODO: issue a warning ? */
                return;
            }
        }
        data = data.offset(nlen as libc::c_int as isize);
        data_length -= nlen as libc::c_int
    }
}
unsafe extern "C" fn parse_rockridge_ZF1(
    mut file: *mut file_info,
    mut data: *const libc::c_uchar,
    mut data_length: libc::c_int,
) {
    if *data.offset(0 as libc::c_int as isize) as libc::c_int == 0x70 as libc::c_int
        && *data.offset(1 as libc::c_int as isize) as libc::c_int == 0x7a as libc::c_int
        && data_length == 12 as libc::c_int
    {
        /* paged zlib */
        (*file).pz = 1 as libc::c_int;
        (*file).pz_log2_bs = *data.offset(3 as libc::c_int as isize) as libc::c_int;
        (*file).pz_uncompressed_size = archive_le32dec(&*data.offset(4 as libc::c_int as isize)
            as *const libc::c_uchar
            as *const libc::c_void) as uint64_t
    };
}
unsafe extern "C" fn register_file(mut iso9660: *mut iso9660, mut file: *mut file_info) {
    (*file).use_next = (*iso9660).use_files;
    (*iso9660).use_files = file;
}
unsafe extern "C" fn release_files(mut iso9660: *mut iso9660) {
    let mut con: *mut content = 0 as *mut content;
    let mut connext: *mut content = 0 as *mut content;
    let mut file: *mut file_info = 0 as *mut file_info;
    file = (*iso9660).use_files;
    while !file.is_null() {
        let mut next: *mut file_info = (*file).use_next;
        archive_string_free(&mut (*file).name);
        archive_string_free(&mut (*file).symlink);
        free((*file).utf16be_name as *mut libc::c_void);
        con = (*file).contents.first;
        while !con.is_null() {
            connext = (*con).next;
            free(con as *mut libc::c_void);
            con = connext
        }
        free(file as *mut libc::c_void);
        file = next
    }
}
unsafe extern "C" fn next_entry_seek(
    mut a: *mut archive_read,
    mut iso9660: *mut iso9660,
    mut pfile: *mut *mut file_info,
) -> libc::c_int {
    let mut file: *mut file_info = 0 as *mut file_info;
    let mut r: libc::c_int = 0;
    r = next_cache_entry(a, iso9660, pfile);
    if r != ARCHIVE_OK {
        return r;
    }
    file = *pfile;
    /* Don't waste time seeking for zero-length bodies. */
    if (*file).size == 0 as libc::c_int as libc::c_ulong {
        (*file).offset = (*iso9660).current_position
    }
    /* flush any remaining bytes from the last round to ensure
     * we're positioned */
    if (*iso9660).entry_bytes_unconsumed != 0 {
        __archive_read_consume(a, (*iso9660).entry_bytes_unconsumed as int64_t);
        (*iso9660).entry_bytes_unconsumed = 0 as libc::c_int as size_t
    }
    /* Seek forward to the start of the entry. */
    if (*iso9660).current_position < (*file).offset {
        let mut step: int64_t = 0;
        step = (*file).offset.wrapping_sub((*iso9660).current_position) as int64_t;
        step = __archive_read_consume(a, step);
        if step < 0 as libc::c_int as libc::c_long {
            return step as libc::c_int;
        }
        (*iso9660).current_position = (*file).offset
    }
    /* We found body of file; handle it now. */
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_cache_entry(
    mut a: *mut archive_read,
    mut iso9660: *mut iso9660,
    mut pfile: *mut *mut file_info,
) -> libc::c_int {
    let mut current_block: u64;
    let mut file: *mut file_info = 0 as *mut file_info;
    let mut empty_files: C2RustUnnamed_4 = C2RustUnnamed_4 {
        first: 0 as *mut file_info,
        last: 0 as *mut *mut file_info,
    };
    let mut number: int64_t = 0;
    let mut count: libc::c_int = 0;
    file = cache_get_entry(iso9660);
    if !file.is_null() {
        *pfile = file;
        return 0 as libc::c_int;
    }
    's_39: loop
    /*
     * Do not expose this at this time
     * because we have not gotten its full-path
     * name yet.
     */
    {
        let mut re: *mut file_info = 0 as *mut file_info;
        let mut d: *mut file_info = 0 as *mut file_info;
        file = heap_get_entry(&mut (*iso9660).pending_files);
        *pfile = file;
        if file.is_null() {
            /*
             * If directory entries all which are descendant of
             * rr_moved are still remaining, expose their.
             */
            if !(*iso9660).re_files.first.is_null()
                && !(*iso9660).rr_moved.is_null()
                && (*(*iso9660).rr_moved).rr_moved_has_re_only as libc::c_int != 0
            {
                /* Expose "rr_moved" entry. */
                cache_add_entry(iso9660, (*iso9660).rr_moved);
            }
            loop {
                re = re_get_entry(iso9660);
                if re.is_null() {
                    break;
                }
                loop
                /* Expose its descendant dirs. */
                {
                    d = rede_get_entry(re);
                    if d.is_null() {
                        break;
                    }
                    cache_add_entry(iso9660, d);
                }
            }
            if !(*iso9660).cache_files.first.is_null() {
                return next_cache_entry(a, iso9660, pfile);
            }
            return 1 as libc::c_int;
        }
        if (*file).cl_offset != 0 {
            let mut first_re: *mut file_info = NULL as *mut file_info;
            let mut nexted_re: libc::c_int = 0 as libc::c_int;
            's_109: loop
            /*
             * Find "RE" dir for the current file, which
             * has "CL" flag.
             */
            {
                re = re_get_entry(iso9660);
                if !(re != first_re) {
                    break;
                }
                if first_re.is_null() {
                    first_re = re
                }
                if (*re).offset == (*file).cl_offset {
                    (*(*re).parent).subdirs -= 1;
                    (*re).parent = (*file).parent;
                    (*re).re = 0 as libc::c_int as libc::c_char;
                    if (*(*re).parent).re_descendant != 0 {
                        nexted_re = 1 as libc::c_int;
                        (*re).re_descendant = 1 as libc::c_int as libc::c_char;
                        if rede_add_entry(re) < 0 as libc::c_int {
                            current_block = 15670586582839803159;
                            break 's_39;
                        }
                        loop
                        /* Move a list of descendants
                         * to a new ancestor. */
                        {
                            d = rede_get_entry(re);
                            if d.is_null() {
                                break 's_109;
                            }
                            if rede_add_entry(d) < 0 as libc::c_int {
                                current_block = 15670586582839803159;
                                break 's_39;
                            }
                        }
                    } else {
                        /* Replace the current file
                         * with "RE" dir */
                        file = re;
                        *pfile = file;
                        loop
                        /* Expose its descendant */
                        {
                            d = rede_get_entry(file);
                            if d.is_null() {
                                break;
                            }
                            cache_add_entry(iso9660, d);
                        }
                        break;
                    }
                } else {
                    re_add_entry(iso9660, re);
                }
            }
            if !(nexted_re != 0) {
                current_block = 12829669402821218572;
                break;
            }
        } else {
            if !((*file).mode & AE_IFMT as mode_t == AE_IFDIR as mode_t) {
                current_block = 12829669402821218572;
                break;
            }
            let mut r: libc::c_int = 0;
            /* Read file entries in this dir. */
            r = read_children(a, file);
            if r != ARCHIVE_OK {
                return r;
            }
            /*
             * Handle a special dir of Rockridge extensions,
             * "rr_moved".
             */
            if (*file).rr_moved != 0 {
                /*
                 * If this has only the subdirectories which
                 * have "RE" flags, do not expose at this time.
                 */
                if !((*file).rr_moved_has_re_only != 0) {
                    current_block = 12829669402821218572;
                    break;
                }
            /* Otherwise expose "rr_moved" entry. */
            } else if (*file).re != 0 {
                /*
                 * Do not expose this at this time
                 * because we have not gotten its full-path
                 * name yet.
                 */
                re_add_entry(iso9660, file);
            } else {
                if !((*file).re_descendant != 0) {
                    current_block = 12829669402821218572;
                    break;
                }
                /*
                 * If the top level "RE" entry of this entry
                 * is not exposed, we, accordingly, should not
                 * expose this entry at this time because
                 * we cannot make its proper full-path name.
                 */
                if !(rede_add_entry(file) == 0 as libc::c_int) {
                    current_block = 12829669402821218572;
                    break;
                }
                /* Otherwise we can expose this entry because
                 * it seems its top level "RE" has already been
                 * exposed. */
            }
        }
    }
    match current_block {
        15670586582839803159 => {
            archive_set_error(&mut (*a).archive as *mut archive,
                              ARCHIVE_ERRNO_MISC,
                              b"Failed to connect \'CL\' pointer to \'RE\' rr_moved pointer of Rockridge extensions: current position = %jd, CL offset = %jd\x00"
                                  as *const u8 as *const libc::c_char,
                              (*iso9660).current_position as intmax_t,
                              (*file).cl_offset as intmax_t);
            return -(30 as libc::c_int);
        }
        _ => {
            if (*file).mode & AE_IFMT as mode_t != AE_IFREG as mode_t
                || (*file).number == -(1 as libc::c_int) as libc::c_long
            {
                return 0 as libc::c_int;
            }
            count = 0 as libc::c_int;
            number = (*file).number;
            (*iso9660).cache_files.first = NULL as *mut file_info;
            (*iso9660).cache_files.last = &mut (*iso9660).cache_files.first;
            empty_files.first = NULL as *mut file_info;
            empty_files.last = &mut empty_files.first;
            /* Collect files which has the same file serial number.
             * Peek pending_files so that file which number is different
             * is not put back. */
            while (*iso9660).pending_files.used > 0 as libc::c_int
                && ((**(*iso9660)
                    .pending_files
                    .files
                    .offset(0 as libc::c_int as isize))
                .number
                    == -(1 as libc::c_int) as libc::c_long
                    || (**(*iso9660)
                        .pending_files
                        .files
                        .offset(0 as libc::c_int as isize))
                    .number
                        == number)
            {
                if (*file).number == -(1 as libc::c_int) as libc::c_long {
                    /* This file has the same offset
                     * but it's wrong offset which empty files
                     * and symlink files have.
                     * NOTE: This wrong offset was recorded by
                     * old mkisofs utility. If ISO images is
                     * created by latest mkisofs, this does not
                     * happen.
                     */
                    (*file).next = NULL as *mut file_info;
                    *empty_files.last = file;
                    empty_files.last = &mut (*file).next
                } else {
                    count += 1;
                    cache_add_entry(iso9660, file);
                }
                file = heap_get_entry(&mut (*iso9660).pending_files)
            }
            if count == 0 as libc::c_int {
                *pfile = file;
                return if file.is_null() {
                    ARCHIVE_EOF
                } else {
                    ARCHIVE_OK
                };
            }
            if (*file).number == -(1 as libc::c_int) as libc::c_long {
                (*file).next = NULL as *mut file_info;
                *empty_files.last = file;
                empty_files.last = &mut (*file).next
            } else {
                count += 1;
                cache_add_entry(iso9660, file);
            }
            if count > 1 as libc::c_int {
                /* The count is the same as number of hardlink,
                 * so much so that each nlinks of files in cache_file
                 * is overwritten by value of the count.
                 */
                file = (*iso9660).cache_files.first;
                while !file.is_null() {
                    (*file).nlinks = count;
                    file = (*file).next
                }
            }
            /* If there are empty files, that files are added
             * to the tail of the cache_files. */
            if !empty_files.first.is_null() {
                *(*iso9660).cache_files.last = empty_files.first;
                (*iso9660).cache_files.last = empty_files.last
            }
            *pfile = cache_get_entry(iso9660);
            return if (*pfile).is_null() {
                ARCHIVE_EOF
            } else {
                ARCHIVE_OK
            };
        }
    };
}
#[inline]
unsafe extern "C" fn re_add_entry(mut iso9660: *mut iso9660, mut file: *mut file_info) {
    (*file).re_next = NULL as *mut file_info;
    *(*iso9660).re_files.last = file;
    (*iso9660).re_files.last = &mut (*file).re_next;
}
#[inline]
unsafe extern "C" fn re_get_entry(mut iso9660: *mut iso9660) -> *mut file_info {
    let mut file: *mut file_info = 0 as *mut file_info;
    file = (*iso9660).re_files.first;
    if !file.is_null() {
        (*iso9660).re_files.first = (*file).re_next;
        if (*iso9660).re_files.first.is_null() {
            (*iso9660).re_files.last = &mut (*iso9660).re_files.first
        }
    }
    return file;
}
#[inline]
unsafe extern "C" fn rede_add_entry(mut file: *mut file_info) -> libc::c_int {
    let mut re: *mut file_info = 0 as *mut file_info;
    /*
     * Find "RE" entry.
     */
    re = (*file).parent;
    while !re.is_null() && (*re).re == 0 {
        re = (*re).parent
    }
    if re.is_null() {
        return -(1 as libc::c_int);
    }
    (*file).re_next = NULL as *mut file_info;
    *(*re).rede_files.last = file;
    (*re).rede_files.last = &mut (*file).re_next;
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn rede_get_entry(mut re: *mut file_info) -> *mut file_info {
    let mut file: *mut file_info = 0 as *mut file_info;
    file = (*re).rede_files.first;
    if !file.is_null() {
        (*re).rede_files.first = (*file).re_next;
        if (*re).rede_files.first.is_null() {
            (*re).rede_files.last = &mut (*re).rede_files.first
        }
    }
    return file;
}
#[inline]
unsafe extern "C" fn cache_add_entry(mut iso9660: *mut iso9660, mut file: *mut file_info) {
    (*file).next = NULL as *mut file_info;
    *(*iso9660).cache_files.last = file;
    (*iso9660).cache_files.last = &mut (*file).next;
}
#[inline]
unsafe extern "C" fn cache_get_entry(mut iso9660: *mut iso9660) -> *mut file_info {
    let mut file: *mut file_info = 0 as *mut file_info;
    file = (*iso9660).cache_files.first;
    if !file.is_null() {
        (*iso9660).cache_files.first = (*file).next;
        if (*iso9660).cache_files.first.is_null() {
            (*iso9660).cache_files.last = &mut (*iso9660).cache_files.first
        }
    }
    return file;
}
unsafe extern "C" fn heap_add_entry(
    mut a: *mut archive_read,
    mut heap: *mut heap_queue,
    mut file: *mut file_info,
    mut key: uint64_t,
) -> libc::c_int {
    let mut file_key: uint64_t = 0;
    let mut parent_key: uint64_t = 0;
    let mut hole: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    /* Expand our pending files list as necessary. */
    if (*heap).used >= (*heap).allocated {
        let mut new_pending_files: *mut *mut file_info = 0 as *mut *mut file_info;
        let mut new_size: libc::c_int = (*heap).allocated * 2 as libc::c_int;
        if (*heap).allocated < 1024 as libc::c_int {
            new_size = 1024 as libc::c_int
        }
        /* Overflow might keep us from growing the list. */
        if new_size <= (*heap).allocated {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        new_pending_files = malloc(
            (new_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut file_info>() as libc::c_ulong),
        ) as *mut *mut file_info;
        if new_pending_files.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if (*heap).allocated != 0 {
            memcpy(
                new_pending_files as *mut libc::c_void,
                (*heap).files as *const libc::c_void,
                ((*heap).allocated as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut file_info>() as libc::c_ulong),
            );
        }
        free((*heap).files as *mut libc::c_void);
        (*heap).files = new_pending_files;
        (*heap).allocated = new_size
    }
    (*file).key = key;
    file_key = (*file).key;
    /*
     * Start with hole at end, walk it up tree to find insertion point.
     */
    let fresh5 = (*heap).used;
    (*heap).used = (*heap).used + 1;
    hole = fresh5;
    while hole > 0 as libc::c_int {
        parent = (hole - 1 as libc::c_int) / 2 as libc::c_int;
        parent_key = (**(*heap).files.offset(parent as isize)).key;
        if file_key >= parent_key {
            let ref mut fresh6 = *(*heap).files.offset(hole as isize);
            *fresh6 = file;
            return 0 as libc::c_int;
        }
        /* Move parent into hole <==> move hole up tree. */
        let ref mut fresh7 = *(*heap).files.offset(hole as isize);
        *fresh7 = *(*heap).files.offset(parent as isize);
        hole = parent
    }
    let ref mut fresh8 = *(*heap).files.offset(0 as libc::c_int as isize);
    *fresh8 = file;
    return 0 as libc::c_int;
}
unsafe extern "C" fn heap_get_entry(mut heap: *mut heap_queue) -> *mut file_info {
    let mut a_key: uint64_t = 0;
    let mut b_key: uint64_t = 0;
    let mut c_key: uint64_t = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut r: *mut file_info = 0 as *mut file_info;
    let mut tmp: *mut file_info = 0 as *mut file_info;
    if (*heap).used < 1 as libc::c_int {
        return 0 as *mut file_info;
    }
    /*
     * The first file in the list is the earliest; we'll return this.
     */
    r = *(*heap).files.offset(0 as libc::c_int as isize);
    /*
     * Move the last item in the heap to the root of the tree
     */
    (*heap).used -= 1;
    let ref mut fresh9 = *(*heap).files.offset(0 as libc::c_int as isize);
    *fresh9 = *(*heap).files.offset((*heap).used as isize);
    /*
     * Rebalance the heap.
     */
    a = 0 as libc::c_int; /* Starting element and its heap key */
    a_key = (**(*heap).files.offset(a as isize)).key; /* First child */
    loop {
        b = a + a + 1 as libc::c_int; /* Use second child if it is smaller. */
        if b >= (*heap).used {
            return r;
        }
        b_key = (**(*heap).files.offset(b as isize)).key;
        c = b + 1 as libc::c_int;
        if c < (*heap).used {
            c_key = (**(*heap).files.offset(c as isize)).key;
            if c_key < b_key {
                b = c;
                b_key = c_key
            }
        }
        if a_key <= b_key {
            return r;
        }
        tmp = *(*heap).files.offset(a as isize);
        let ref mut fresh10 = *(*heap).files.offset(a as isize);
        *fresh10 = *(*heap).files.offset(b as isize);
        let ref mut fresh11 = *(*heap).files.offset(b as isize);
        *fresh11 = tmp;
        a = b
    }
}
unsafe extern "C" fn toi(mut p: *const libc::c_void, mut n: libc::c_int) -> libc::c_uint {
    let mut v: *const libc::c_uchar = p as *const libc::c_uchar;
    if n > 1 as libc::c_int {
        return (*v.offset(0 as libc::c_int as isize) as libc::c_uint).wrapping_add(
            (256 as libc::c_int as libc::c_uint).wrapping_mul(toi(
                v.offset(1 as libc::c_int as isize) as *const libc::c_void,
                n - 1 as libc::c_int,
            )),
        );
    }
    if n == 1 as libc::c_int {
        return *v.offset(0 as libc::c_int as isize) as libc::c_uint;
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn isodate7(mut v: *const libc::c_uchar) -> time_t {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut offset: libc::c_int = 0;
    let mut t: time_t = 0;
    memset(
        &mut tm as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tm>() as libc::c_ulong,
    );
    tm.tm_year = *v.offset(0 as libc::c_int as isize) as libc::c_int;
    tm.tm_mon = *v.offset(1 as libc::c_int as isize) as libc::c_int - 1 as libc::c_int;
    tm.tm_mday = *v.offset(2 as libc::c_int as isize) as libc::c_int;
    tm.tm_hour = *v.offset(3 as libc::c_int as isize) as libc::c_int;
    tm.tm_min = *v.offset(4 as libc::c_int as isize) as libc::c_int;
    tm.tm_sec = *v.offset(5 as libc::c_int as isize) as libc::c_int;
    /* v[6] is the signed timezone offset, in 1/4-hour increments. */
    offset = *(v as *const libc::c_schar).offset(6 as libc::c_int as isize) as libc::c_int;
    if offset > -(48 as libc::c_int) && offset < 52 as libc::c_int {
        tm.tm_hour -= offset / 4 as libc::c_int;
        tm.tm_min -= offset % 4 as libc::c_int * 15 as libc::c_int
    }
    t = time_from_tm(&mut tm);
    if t == -(1 as libc::c_int) as time_t {
        return 0 as libc::c_int as time_t;
    }
    return t;
}
unsafe extern "C" fn isodate17(mut v: *const libc::c_uchar) -> time_t {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut offset: libc::c_int = 0;
    let mut t: time_t = 0;
    memset(
        &mut tm as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tm>() as libc::c_ulong,
    );
    tm.tm_year = (*v.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
        * 1000 as libc::c_int
        + (*v.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32) * 100 as libc::c_int
        + (*v.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32) * 10 as libc::c_int
        + (*v.offset(3 as libc::c_int as isize) as libc::c_int - '0' as i32)
        - 1900 as libc::c_int;
    tm.tm_mon = (*v.offset(4 as libc::c_int as isize) as libc::c_int - '0' as i32)
        * 10 as libc::c_int
        + (*v.offset(5 as libc::c_int as isize) as libc::c_int - '0' as i32);
    tm.tm_mday = (*v.offset(6 as libc::c_int as isize) as libc::c_int - '0' as i32)
        * 10 as libc::c_int
        + (*v.offset(7 as libc::c_int as isize) as libc::c_int - '0' as i32);
    tm.tm_hour = (*v.offset(8 as libc::c_int as isize) as libc::c_int - '0' as i32)
        * 10 as libc::c_int
        + (*v.offset(9 as libc::c_int as isize) as libc::c_int - '0' as i32);
    tm.tm_min = (*v.offset(10 as libc::c_int as isize) as libc::c_int - '0' as i32)
        * 10 as libc::c_int
        + (*v.offset(11 as libc::c_int as isize) as libc::c_int - '0' as i32);
    tm.tm_sec = (*v.offset(12 as libc::c_int as isize) as libc::c_int - '0' as i32)
        * 10 as libc::c_int
        + (*v.offset(13 as libc::c_int as isize) as libc::c_int - '0' as i32);
    /* v[16] is the signed timezone offset, in 1/4-hour increments. */
    offset = *(v as *const libc::c_schar).offset(16 as libc::c_int as isize) as libc::c_int;
    if offset > -(48 as libc::c_int) && offset < 52 as libc::c_int {
        tm.tm_hour -= offset / 4 as libc::c_int;
        tm.tm_min -= offset % 4 as libc::c_int * 15 as libc::c_int
    }
    t = time_from_tm(&mut tm);
    if t == -(1 as libc::c_int) as time_t {
        return 0 as libc::c_int as time_t;
    }
    return t;
}
unsafe extern "C" fn time_from_tm(mut t: *mut tm) -> time_t {
    /* Use platform timegm() if available. */
    return timegm(t);
}
unsafe extern "C" fn build_pathname(
    mut as_0: *mut archive_string,
    mut file: *mut file_info,
    mut depth: libc::c_int,
) -> *const libc::c_char {
    // Plain ISO9660 only allows 8 dir levels; if we get
    // to 1000, then something is very, very wrong.
    if depth > 1000 as libc::c_int {
        return NULL as *const libc::c_char;
    } /* Path is too long! */
    if !(*file).parent.is_null()
        && (*(*file).parent).name.length > 0 as libc::c_int as libc::c_ulong
    {
        if build_pathname(as_0, (*file).parent, depth + 1 as libc::c_int).is_null() {
            return NULL as *const libc::c_char;
        } /* Path is too long! */
        archive_strcat(
            as_0,
            b"/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
    }
    if (*file).name.length == 0 as libc::c_int as libc::c_ulong {
        archive_strcat(
            as_0,
            b".\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
    } else {
        archive_string_concat(as_0, &mut (*file).name);
    }
    return (*as_0).s;
}
unsafe extern "C" fn build_pathname_utf16be(
    mut p: *mut libc::c_uchar,
    mut max: size_t,
    mut len: *mut size_t,
    mut file: *mut file_info,
) -> libc::c_int {
    if !(*file).parent.is_null()
        && (*(*file).parent).utf16be_bytes > 0 as libc::c_int as libc::c_ulong
    {
        if build_pathname_utf16be(p, max, len, (*file).parent) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        *p.offset(*len as isize) = 0 as libc::c_int as libc::c_uchar;
        *p.offset((*len).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) =
            '/' as i32 as libc::c_uchar;
        *len = (*len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    if (*file).utf16be_bytes == 0 as libc::c_int as libc::c_ulong {
        if (*len).wrapping_add(2 as libc::c_int as libc::c_ulong) > max {
            return -(1 as libc::c_int);
        }
        *p.offset(*len as isize) = 0 as libc::c_int as libc::c_uchar;
        *p.offset((*len).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) =
            '.' as i32 as libc::c_uchar;
        *len = (*len as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    } else {
        if (*len).wrapping_add((*file).utf16be_bytes) > max {
            return -(1 as libc::c_int);
        }
        memcpy(
            p.offset(*len as isize) as *mut libc::c_void,
            (*file).utf16be_name as *const libc::c_void,
            (*file).utf16be_bytes,
        );
        *len = (*len as libc::c_ulong).wrapping_add((*file).utf16be_bytes) as size_t as size_t
    }
    return 0 as libc::c_int;
}
