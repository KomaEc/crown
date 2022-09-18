use ::libc;
extern "C" {
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
    pub type archive_string_conv;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type uintmax_t = __uintmax_t;
pub type size_t = libc::c_ulong;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type la_int64_t = int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_w {
    pub entry_bytes_remaining: uint64_t,
    pub entry_padding: uint64_t,
    pub is_strtab: libc::c_int,
    pub has_strtab: libc::c_int,
    pub wrote_global_header: libc::c_char,
    pub strtab: *mut libc::c_char,
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
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const ERANGE: libc::c_int = 34 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_AR_GNU: libc::c_int = ARCHIVE_FORMAT_AR | 1 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_AR_BSD: libc::c_int = ARCHIVE_FORMAT_AR | 2 as libc::c_int;
pub const ARCHIVE_FORMAT_AR: libc::c_int = 0x70000 as libc::c_int;
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
/*
 * Define structure of the "ar" header.
 */
pub const AR_name_offset: libc::c_int = 0 as libc::c_int;
pub const AR_name_size: libc::c_int = 16 as libc::c_int;
pub const AR_date_offset: libc::c_int = 16 as libc::c_int;
pub const AR_date_size: libc::c_int = 12 as libc::c_int;
pub const AR_uid_offset: libc::c_int = 28 as libc::c_int;
pub const AR_uid_size: libc::c_int = 6 as libc::c_int;
pub const AR_gid_offset: libc::c_int = 34 as libc::c_int;
pub const AR_gid_size: libc::c_int = 6 as libc::c_int;
pub const AR_mode_offset: libc::c_int = 40 as libc::c_int;
pub const AR_mode_size: libc::c_int = 8 as libc::c_int;
pub const AR_size_offset: libc::c_int = 48 as libc::c_int;
pub const AR_size_size: libc::c_int = 10 as libc::c_int;
pub const AR_fmag_offset: libc::c_int = 58 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_ar_bsd(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_ar_bsd\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    r = archive_write_set_format_ar(a);
    if r == ARCHIVE_OK {
        (*a).archive.archive_format = ARCHIVE_FORMAT_AR_BSD;
        (*a).archive.archive_format_name = b"ar (BSD)\x00" as *const u8 as *const libc::c_char
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_ar_svr4(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_ar_svr4\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    r = archive_write_set_format_ar(a);
    if r == ARCHIVE_OK {
        (*a).archive.archive_format = ARCHIVE_FORMAT_AR_GNU;
        (*a).archive.archive_format_name = b"ar (GNU/SVR4)\x00" as *const u8 as *const libc::c_char
    }
    return r;
}
/*
 * Generic initialization.
 */
unsafe extern "C" fn archive_write_set_format_ar(mut a: *mut archive_write) -> libc::c_int {
    let mut ar: *mut ar_w = 0 as *mut ar_w;
    /* If someone else was already registered, unregister them. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    ar = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ar_w>() as libc::c_ulong,
    ) as *mut ar_w;
    if ar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate ar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*a).format_data = ar as *mut libc::c_void;
    (*a).format_name = b"ar\x00" as *const u8 as *const libc::c_char;
    (*a).format_write_header = Some(
        archive_write_ar_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        archive_write_ar_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_close =
        Some(archive_write_ar_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_free =
        Some(archive_write_ar_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_finish_entry = Some(
        archive_write_ar_finish_entry as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_ar_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut append_fn: libc::c_int = 0;
    let mut buff: [libc::c_char; 60] = [0; 60];
    let mut ss: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut se: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ar: *mut ar_w = 0 as *mut ar_w;
    let mut pathname: *const libc::c_char = 0 as *const libc::c_char;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: int64_t = 0;
    append_fn = 0 as libc::c_int;
    ar = (*a).format_data as *mut ar_w;
    (*ar).is_strtab = 0 as libc::c_int;
    filename = NULL as *const libc::c_char;
    size = archive_entry_size(entry);
    /*
     * Reject files with empty name.
     */
    pathname = archive_entry_pathname(entry);
    if pathname.is_null() || *pathname as libc::c_int == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Invalid filename\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    /*
     * If we are now at the beginning of the archive,
     * we need first write the ar global header.
     */
    if (*ar).wrote_global_header == 0 {
        __archive_write_output(
            a,
            b"!<arch>\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as size_t,
        );
        (*ar).wrote_global_header = 1 as libc::c_int as libc::c_char
    }
    memset(
        buff.as_mut_ptr() as *mut libc::c_void,
        ' ' as i32,
        60 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        &mut *buff.as_mut_ptr().offset(AR_fmag_offset as isize) as *mut libc::c_char
            as *mut libc::c_void,
        b"`\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    if strcmp(pathname, b"/\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /* Entry is archive symbol table in GNU format */
        buff[AR_name_offset as usize] = '/' as i32 as libc::c_char;
        current_block = 11042385209953506173;
    } else if strcmp(pathname, b"/SYM64/\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        /* Entry is archive symbol table in GNU 64-bit format */
        memcpy(
            buff.as_mut_ptr().offset(AR_name_offset as isize) as *mut libc::c_void,
            b"/SYM64/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            7 as libc::c_int as libc::c_ulong,
        );
        current_block = 11042385209953506173;
    } else if strcmp(
        pathname,
        b"__.SYMDEF\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        /* Entry is archive symbol table in BSD format */
        memcpy(
            buff.as_mut_ptr().offset(AR_name_offset as isize) as *mut libc::c_void,
            b"__.SYMDEF\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            9 as libc::c_int as libc::c_ulong,
        );
        current_block = 11042385209953506173;
    } else if strcmp(pathname, b"//\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /*
         * Entry is archive filename table, inform that we should
         * collect strtab in next _data call.
         */
        (*ar).is_strtab = 1 as libc::c_int;
        buff[(AR_name_offset + 1 as libc::c_int) as usize] = '/' as i32 as libc::c_char;
        buff[AR_name_offset as usize] = buff[(AR_name_offset + 1 as libc::c_int) as usize];
        current_block = 2233384814668020854;
    } else {
        /*
         * Otherwise, entry is a normal archive member.
         * Strip leading paths from filenames, if any.
         */
        filename = ar_basename(pathname);
        if filename.is_null() {
            /* Reject filenames with trailing "/" */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"Invalid filename\x00" as *const u8 as *const libc::c_char,
            );
            return -(20 as libc::c_int);
        }
        if (*a).archive.archive_format == ARCHIVE_FORMAT_AR_GNU {
            /*
             * SVR4/GNU variant use a "/" to mark then end of the filename,
             * make it possible to have embedded spaces in the filename.
             * So, the longest filename here (without extension) is
             * actually 15 bytes.
             */
            if strlen(filename) <= 15 as libc::c_int as libc::c_ulong {
                memcpy(
                    &mut *buff.as_mut_ptr().offset(AR_name_offset as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    filename as *const libc::c_void,
                    strlen(filename),
                );
                buff[(AR_name_offset as libc::c_ulong).wrapping_add(strlen(filename)) as usize] =
                    '/' as i32 as libc::c_char
            } else {
                /*
                 * For filename longer than 15 bytes, GNU variant
                 * makes use of a string table and instead stores the
                 * offset of the real filename to in the ar_name field.
                 * The string table should have been written before.
                 */
                if (*ar).has_strtab <= 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        EINVAL,
                        b"Can\'t find string table\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(20 as libc::c_int);
                }
                se = malloc(strlen(filename).wrapping_add(3 as libc::c_int as libc::c_ulong))
                    as *mut libc::c_char;
                if se.is_null() {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"Can\'t allocate filename buffer\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                memcpy(
                    se as *mut libc::c_void,
                    filename as *const libc::c_void,
                    strlen(filename),
                );
                strcpy(
                    se.offset(strlen(filename) as isize),
                    b"/\n\x00" as *const u8 as *const libc::c_char,
                );
                ss = strstr((*ar).strtab, se);
                free(se as *mut libc::c_void);
                if ss.is_null() {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        EINVAL,
                        b"Invalid string table\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(20 as libc::c_int);
                }
                /*
                 * GNU variant puts "/" followed by digits into
                 * ar_name field. These digits indicates the real
                 * filename string's offset to the string table.
                 */
                buff[AR_name_offset as usize] = '/' as i32 as libc::c_char;
                if format_decimal(
                    ss.offset_from((*ar).strtab) as libc::c_long,
                    buff.as_mut_ptr()
                        .offset(AR_name_offset as isize)
                        .offset(1 as libc::c_int as isize),
                    AR_name_size - 1 as libc::c_int,
                ) != 0
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ERANGE,
                        b"string table offset too large\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(20 as libc::c_int);
                }
            }
        } else if (*a).archive.archive_format == ARCHIVE_FORMAT_AR_BSD {
            /*
             * BSD variant: for any file name which is more than
             * 16 chars or contains one or more embedded space(s), the
             * string "#1/" followed by the ASCII length of the name is
             * put into the ar_name field. The file size (stored in the
             * ar_size field) is incremented by the length of the name.
             * The name is then written immediately following the
             * archive header.
             */
            if strlen(filename) <= 16 as libc::c_int as libc::c_ulong
                && strchr(filename, ' ' as i32).is_null()
            {
                memcpy(
                    &mut *buff.as_mut_ptr().offset(AR_name_offset as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    filename as *const libc::c_void,
                    strlen(filename),
                );
                buff[(AR_name_offset as libc::c_ulong).wrapping_add(strlen(filename)) as usize] =
                    ' ' as i32 as libc::c_char
            } else {
                memcpy(
                    buff.as_mut_ptr().offset(AR_name_offset as isize) as *mut libc::c_void,
                    b"#1/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    3 as libc::c_int as libc::c_ulong,
                );
                if format_decimal(
                    strlen(filename) as int64_t,
                    buff.as_mut_ptr()
                        .offset(AR_name_offset as isize)
                        .offset(3 as libc::c_int as isize),
                    AR_name_size - 3 as libc::c_int,
                ) != 0
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ERANGE,
                        b"File name too long\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(20 as libc::c_int);
                }
                append_fn = 1 as libc::c_int;
                size = (size as libc::c_ulong).wrapping_add(strlen(filename)) as int64_t as int64_t
            }
        }
        current_block = 11042385209953506173;
    }
    match current_block {
        11042385209953506173 => {
            if format_decimal(
                archive_entry_mtime(entry),
                buff.as_mut_ptr().offset(AR_date_offset as isize),
                AR_date_size,
            ) != 0
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ERANGE,
                    b"File modification time too large\x00" as *const u8 as *const libc::c_char,
                );
                return -(20 as libc::c_int);
            }
            if format_decimal(
                archive_entry_uid(entry),
                buff.as_mut_ptr().offset(AR_uid_offset as isize),
                AR_uid_size,
            ) != 0
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ERANGE,
                    b"Numeric user ID too large\x00" as *const u8 as *const libc::c_char,
                );
                return -(20 as libc::c_int);
            }
            if format_decimal(
                archive_entry_gid(entry),
                buff.as_mut_ptr().offset(AR_gid_offset as isize),
                AR_gid_size,
            ) != 0
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ERANGE,
                    b"Numeric group ID too large\x00" as *const u8 as *const libc::c_char,
                );
                return -(20 as libc::c_int);
            }
            if format_octal(
                archive_entry_mode(entry) as int64_t,
                buff.as_mut_ptr().offset(AR_mode_offset as isize),
                AR_mode_size,
            ) != 0
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ERANGE,
                    b"Numeric mode too large\x00" as *const u8 as *const libc::c_char,
                );
                return -(20 as libc::c_int);
            }
            /*
             * Sanity Check: A non-pseudo archive member should always be
             * a regular file.
             */
            if !filename.is_null() && archive_entry_filetype(entry) != AE_IFREG as mode_t {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    EINVAL,
                    b"Regular file required for non-pseudo member\x00" as *const u8
                        as *const libc::c_char,
                );
                return -(20 as libc::c_int);
            }
        }
        _ => {}
    }
    /*
     * For archive string table, only ar_size field should
     * be set.
     */
    if format_decimal(
        size,
        buff.as_mut_ptr().offset(AR_size_offset as isize),
        AR_size_size,
    ) != 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"File size out of range\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    ret = __archive_write_output(
        a,
        buff.as_mut_ptr() as *const libc::c_void,
        60 as libc::c_int as size_t,
    );
    if ret != ARCHIVE_OK {
        return ret;
    }
    (*ar).entry_bytes_remaining = size as uint64_t;
    (*ar).entry_padding = (*ar)
        .entry_bytes_remaining
        .wrapping_rem(2 as libc::c_int as libc::c_ulong);
    if append_fn > 0 as libc::c_int {
        ret = __archive_write_output(a, filename as *const libc::c_void, strlen(filename));
        if ret != ARCHIVE_OK {
            return ret;
        }
        (*ar).entry_bytes_remaining = ((*ar).entry_bytes_remaining as libc::c_ulong)
            .wrapping_sub(strlen(filename)) as uint64_t
            as uint64_t
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_ar_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut ar: *mut ar_w = 0 as *mut ar_w;
    let mut ret: libc::c_int = 0;
    ar = (*a).format_data as *mut ar_w;
    if s > (*ar).entry_bytes_remaining {
        s = (*ar).entry_bytes_remaining
    }
    if (*ar).is_strtab > 0 as libc::c_int {
        if (*ar).has_strtab > 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"More than one string tables exist\x00" as *const u8 as *const libc::c_char,
            );
            return -(20 as libc::c_int) as ssize_t;
        }
        (*ar).strtab =
            malloc(s.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
        if (*ar).strtab.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate strtab buffer\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        memcpy((*ar).strtab as *mut libc::c_void, buff, s);
        *(*ar).strtab.offset(s as isize) = '\u{0}' as i32 as libc::c_char;
        (*ar).has_strtab = 1 as libc::c_int
    }
    ret = __archive_write_output(a, buff, s);
    if ret != ARCHIVE_OK {
        return ret as ssize_t;
    }
    (*ar).entry_bytes_remaining =
        ((*ar).entry_bytes_remaining as libc::c_ulong).wrapping_sub(s) as uint64_t as uint64_t;
    return s as ssize_t;
}
unsafe extern "C" fn archive_write_ar_free(mut a: *mut archive_write) -> libc::c_int {
    let mut ar: *mut ar_w = 0 as *mut ar_w;
    ar = (*a).format_data as *mut ar_w;
    if ar.is_null() {
        return 0 as libc::c_int;
    }
    if (*ar).has_strtab > 0 as libc::c_int {
        free((*ar).strtab as *mut libc::c_void);
        (*ar).strtab = NULL as *mut libc::c_char
    }
    free(ar as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_ar_close(mut a: *mut archive_write) -> libc::c_int {
    let mut ar: *mut ar_w = 0 as *mut ar_w;
    let mut ret: libc::c_int = 0;
    /*
     * If we haven't written anything yet, we need to write
     * the ar global header now to make it a valid ar archive.
     */
    ar = (*a).format_data as *mut ar_w;
    if (*ar).wrote_global_header == 0 {
        (*ar).wrote_global_header = 1 as libc::c_int as libc::c_char;
        ret = __archive_write_output(
            a,
            b"!<arch>\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as size_t,
        );
        return ret;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_ar_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut ar: *mut ar_w = 0 as *mut ar_w;
    let mut ret: libc::c_int = 0;
    ar = (*a).format_data as *mut ar_w;
    if (*ar).entry_bytes_remaining != 0 as libc::c_int as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Entry remaining bytes larger than 0\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    if (*ar).entry_padding == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if (*ar).entry_padding != 1 as libc::c_int as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Padding wrong size: %ju should be 1 or 0\x00" as *const u8 as *const libc::c_char,
            (*ar).entry_padding,
        );
        return -(20 as libc::c_int);
    }
    ret = __archive_write_output(
        a,
        b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    return ret;
}
/*
 * Format a number into the specified field using base-8.
 * NB: This version is slightly different from the one in
 * _ustar.c
 */
unsafe extern "C" fn format_octal(
    mut v: int64_t,
    mut p: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut h: *mut libc::c_char = 0 as *mut libc::c_char;
    len = s;
    h = p;
    /* Octal values can't be negative, so use 0. */
    if v < 0 as libc::c_int as libc::c_long {
        loop {
            let fresh0 = len; /* Start at the end and work backwards. */
            len = len - 1;
            if !(fresh0 > 0 as libc::c_int) {
                break;
            }
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = '0' as i32 as libc::c_char
        }
        return -(1 as libc::c_int);
    }
    p = p.offset(s as isize);
    loop {
        p = p.offset(-1);
        *p = ('0' as i32 as libc::c_long + (v & 7 as libc::c_int as libc::c_long)) as libc::c_char;
        v >>= 3 as libc::c_int;
        s -= 1;
        if !(s > 0 as libc::c_int && v > 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    if v == 0 as libc::c_int as libc::c_long {
        memmove(
            h as *mut libc::c_void,
            p as *const libc::c_void,
            (len - s) as libc::c_ulong,
        );
        p = h.offset(len as isize).offset(-(s as isize));
        loop {
            let fresh2 = s;
            s = s - 1;
            if !(fresh2 > 0 as libc::c_int) {
                break;
            }
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = ' ' as i32 as libc::c_char
        }
        return 0 as libc::c_int;
    }
    loop
    /* If it overflowed, fill field with max value. */
    {
        let fresh4 = len;
        len = len - 1;
        if !(fresh4 > 0 as libc::c_int) {
            break;
        }
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '7' as i32 as libc::c_char
    }
    return -(1 as libc::c_int);
}
/*
 * Format a number into the specified field using base-10.
 */
unsafe extern "C" fn format_decimal(
    mut v: int64_t,
    mut p: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut h: *mut libc::c_char = 0 as *mut libc::c_char;
    len = s;
    h = p;
    /* Negative values in ar header are meaningless, so use 0. */
    if v < 0 as libc::c_int as libc::c_long {
        loop {
            let fresh6 = len;
            len = len - 1;
            if !(fresh6 > 0 as libc::c_int) {
                break;
            }
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = '0' as i32 as libc::c_char
        }
        return -(1 as libc::c_int);
    }
    p = p.offset(s as isize);
    loop {
        p = p.offset(-1);
        *p = ('0' as i32 as libc::c_long + v % 10 as libc::c_int as libc::c_long) as libc::c_char;
        v /= 10 as libc::c_int as libc::c_long;
        s -= 1;
        if !(s > 0 as libc::c_int && v > 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    if v == 0 as libc::c_int as libc::c_long {
        memmove(
            h as *mut libc::c_void,
            p as *const libc::c_void,
            (len - s) as libc::c_ulong,
        );
        p = h.offset(len as isize).offset(-(s as isize));
        loop {
            let fresh8 = s;
            s = s - 1;
            if !(fresh8 > 0 as libc::c_int) {
                break;
            }
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = ' ' as i32 as libc::c_char
        }
        return 0 as libc::c_int;
    }
    loop
    /* If it overflowed, fill field with max value. */
    {
        let fresh10 = len;
        len = len - 1;
        if !(fresh10 > 0 as libc::c_int) {
            break;
        }
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = '9' as i32 as libc::c_char
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn ar_basename(mut path: *const libc::c_char) -> *const libc::c_char {
    let mut endp: *const libc::c_char = 0 as *const libc::c_char;
    let mut startp: *const libc::c_char = 0 as *const libc::c_char;
    endp = path
        .offset(strlen(path) as isize)
        .offset(-(1 as libc::c_int as isize));
    /*
     * For filename with trailing slash(es), we return
     * NULL indicating an error.
     */
    if *endp as libc::c_int == '/' as i32 {
        return 0 as *const libc::c_char;
    }
    /* Find the start of the base */
    startp = endp;
    while startp > path && *startp.offset(-(1 as libc::c_int as isize)) as libc::c_int != '/' as i32
    {
        startp = startp.offset(-1)
    }
    return startp;
}
