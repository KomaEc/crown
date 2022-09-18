use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type archive_string_conv;
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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    /*-
     * Copyright (c) 2014 Michihiro NAKAJIMA
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
    /* Random number generator. */
    #[no_mangle]
    fn archive_random(buf: *mut libc::c_void, nbytes: size_t) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    /*-
     * Copyright (c) 2020 Martin Matuska
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
    #[no_mangle]
    fn __archive_write_entry_filetype_unsupported(
        a: *mut archive,
        entry: *mut archive_entry,
        format: *const libc::c_char,
    );
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
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct warc_s {
    #[bitfield(name = "omit_warcinfo", ty = "libc::c_uint", bits = "0..=0")]
    pub omit_warcinfo: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub now: time_t,
    pub typ: mode_t,
    pub rng: libc::c_uint,
    pub populz: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warc_essential_hdr_t {
    pub type_0: warc_type_t,
    pub tgturi: *const libc::c_char,
    pub recid: *const libc::c_char,
    pub rtime: time_t,
    pub mtime: time_t,
    pub cnttyp: *const libc::c_char,
    pub cntlen: uint64_t,
}
pub type warc_type_t = libc::c_uint;
pub const LAST_WT: warc_type_t = 9;
pub const WT_CONT: warc_type_t = 8;
pub const WT_CONV: warc_type_t = 7;
pub const WT_RVIS: warc_type_t = 6;
pub const WT_RSP: warc_type_t = 5;
pub const WT_REQ: warc_type_t = 4;
pub const WT_RSRC: warc_type_t = 3;
pub const WT_META: warc_type_t = 2;
pub const WT_INFO: warc_type_t = 1;
pub const WT_NONE: warc_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warc_uuid_t {
    pub u: [libc::c_uint; 4],
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
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_WARC: libc::c_int = 0xf0000 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
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
static mut warcinfo: [libc::c_char; 60] = unsafe {
    *::std::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
        b"software: libarchive/3.4.3\r\nformat: WARC file version 1.0\r\n\x00",
    )
};
/*
 * Set output format to ISO 28500 (aka WARC) format.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_warc(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut w: *mut warc_s = 0 as *mut warc_s;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_warc\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If another format was already registered, unregister it. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    w = malloc(::std::mem::size_of::<warc_s>() as libc::c_ulong) as *mut warc_s;
    if w.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate warc data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* by default we're emitting a file wide header */
    (*w).set_omit_warcinfo(0 as libc::c_uint);
    /* obtain current time for date fields */
    (*w).now = time(NULL as *mut time_t);
    /* reset file type info */
    (*w).typ = 0 as libc::c_int as mode_t;
    /* also initialise our rng */
    (*w).rng = (*w).now as libc::c_uint;
    (*a).format_data = w as *mut libc::c_void;
    (*a).format_name = b"WARC/1.0\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        _warc_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        _warc_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        _warc_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_close =
        Some(_warc_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_free =
        Some(_warc_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_finish_entry =
        Some(_warc_finish_entry as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).archive.archive_format = ARCHIVE_FORMAT_WARC;
    (*a).archive.archive_format_name = b"WARC/1.0\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
/* archive methods */
unsafe extern "C" fn _warc_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut w: *mut warc_s = (*a).format_data as *mut warc_s;
    if strcmp(
        key,
        b"omit-warcinfo\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        if val.is_null()
            || strcmp(val, b"true\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            /* great */
            (*w).set_omit_warcinfo(1 as libc::c_uint);
            return 0 as libc::c_int;
        }
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn _warc_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut w: *mut warc_s = (*a).format_data as *mut warc_s;
    let mut hdr: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    /* check whether warcinfo record needs outputting */
    if (*w).omit_warcinfo() == 0 {
        let mut r: ssize_t = 0;
        let mut wi: warc_essential_hdr_t = {
            let mut init = warc_essential_hdr_t {
                type_0: WT_INFO,
                tgturi: NULL as *const libc::c_char,
                recid: NULL as *const libc::c_char,
                rtime: 0 as libc::c_int as time_t,
                mtime: 0 as libc::c_int as time_t,
                cnttyp: b"application/warc-fields\x00" as *const u8 as *const libc::c_char,
                cntlen: (::std::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
            };
            init
        };
        wi.rtime = (*w).now;
        wi.mtime = (*w).now;
        hdr.s = NULL as *mut libc::c_char;
        hdr.length = 0 as libc::c_int as size_t;
        hdr.buffer_length = 0 as libc::c_int as size_t;
        r = _popul_ehdr(&mut hdr, MAX_HDR_SIZE as size_t, wi);
        if r >= 0 as libc::c_int as libc::c_long {
            /* jackpot! */
            /* now also use HDR buffer for the actual warcinfo */
            archive_strncat(
                &mut hdr,
                warcinfo.as_ptr() as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 60]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            /* append end-of-record indicator */
            archive_strncat(
                &mut hdr,
                b"\r\n\r\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as size_t,
            );
            /* write to output stream */
            __archive_write_output(a, hdr.s as *const libc::c_void, hdr.length);
        }
        /* indicate we're done with file header writing */
        (*w).set_omit_warcinfo(1 as libc::c_uint);
        archive_string_free(&mut hdr);
    }
    if archive_entry_pathname(entry).is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Invalid filename\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    (*w).typ = archive_entry_filetype(entry);
    (*w).populz = 0 as libc::c_uint as uint64_t;
    if (*w).typ == AE_IFREG as mode_t {
        let mut rh: warc_essential_hdr_t = {
            let mut init = warc_essential_hdr_t {
                type_0: WT_RSRC,
                tgturi: NULL as *const libc::c_char,
                recid: NULL as *const libc::c_char,
                rtime: 0 as libc::c_int as time_t,
                mtime: 0 as libc::c_int as time_t,
                cnttyp: NULL as *const libc::c_char,
                cntlen: 0 as libc::c_int as uint64_t,
            };
            init
        };
        let mut r_0: ssize_t = 0;
        rh.tgturi = archive_entry_pathname(entry);
        rh.rtime = (*w).now;
        rh.mtime = archive_entry_mtime(entry);
        rh.cntlen = archive_entry_size(entry) as size_t;
        hdr.s = NULL as *mut libc::c_char;
        hdr.length = 0 as libc::c_int as size_t;
        hdr.buffer_length = 0 as libc::c_int as size_t;
        r_0 = _popul_ehdr(&mut hdr, MAX_HDR_SIZE as size_t, rh);
        if r_0 < 0 as libc::c_int as libc::c_long {
            /* don't bother */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"cannot archive file\x00" as *const u8 as *const libc::c_char,
            );
            return -(20 as libc::c_int);
        }
        /* otherwise append to output stream */
        __archive_write_output(a, hdr.s as *const libc::c_void, r_0 as size_t);
        /* and let subsequent calls to _data() know about the size */
        (*w).populz = rh.cntlen;
        archive_string_free(&mut hdr);
        return 0 as libc::c_int;
    }
    /* just resort to erroring as per Tim's advice */
    __archive_write_entry_filetype_unsupported(
        &mut (*a).archive,
        entry,
        b"WARC\x00" as *const u8 as *const libc::c_char,
    );
    return -(25 as libc::c_int);
}
pub const MAX_HDR_SIZE: libc::c_int = 512 as libc::c_int;
unsafe extern "C" fn _warc_data(
    mut a: *mut archive_write,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut w: *mut warc_s = (*a).format_data as *mut warc_s;
    if (*w).typ == AE_IFREG as mode_t {
        let mut rc: libc::c_int = 0;
        /* never write more bytes than announced */
        if len > (*w).populz {
            len = (*w).populz
        }
        /* now then, out we put the whole shebang */
        rc = __archive_write_output(a, buf, len);
        if rc != ARCHIVE_OK {
            return rc as ssize_t;
        }
    }
    return len as ssize_t;
}
unsafe extern "C" fn _warc_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    static mut _eor: [libc::c_char; 5] =
        unsafe { *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\r\n\r\n\x00") };
    let mut w: *mut warc_s = (*a).format_data as *mut warc_s;
    if (*w).typ == AE_IFREG as mode_t {
        let mut rc: libc::c_int = __archive_write_output(
            a,
            _eor.as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
        );
        if rc != ARCHIVE_OK {
            return rc;
        }
    }
    /* reset type info */
    (*w).typ = 0 as libc::c_int as mode_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _warc_close(mut a: *mut archive_write) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
unsafe extern "C" fn _warc_free(mut a: *mut archive_write) -> libc::c_int {
    let mut w: *mut warc_s = (*a).format_data as *mut warc_s;
    free(w as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/* private routines */
unsafe extern "C" fn xstrftime(
    mut as_0: *mut archive_string,
    mut fmt: *const libc::c_char,
    mut t: time_t,
) {
    /* * like strftime(3) but for time_t objects */
    let mut rt: *mut tm = 0 as *mut tm;
    let mut timeHere: tm = tm {
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
    let mut strtime: [libc::c_char; 100] = [0; 100];
    let mut len: size_t = 0;
    rt = gmtime_r(&mut t, &mut timeHere);
    if rt.is_null() {
        return;
    }
    /* leave the hard yacker to our role model strftime() */
    len = strftime(
        strtime.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        fmt,
        rt,
    );
    archive_strncat(as_0, strtime.as_mut_ptr() as *const libc::c_void, len);
}
/* private routines */
unsafe extern "C" fn _popul_ehdr(
    mut tgt: *mut archive_string,
    mut tsz: size_t,
    mut hdr: warc_essential_hdr_t,
) -> ssize_t {
    static mut _ver: [libc::c_char; 11] =
        unsafe { *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"WARC/1.0\r\n\x00") };
    static mut _typ: [*const libc::c_char; 9] = [
        NULL as *const libc::c_char,
        b"warcinfo\x00" as *const u8 as *const libc::c_char,
        b"metadata\x00" as *const u8 as *const libc::c_char,
        b"resource\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut std_uuid: [libc::c_char; 48] = [0; 48];
    if hdr.type_0 as libc::c_uint == WT_NONE as libc::c_int as libc::c_uint
        || hdr.type_0 as libc::c_uint > WT_RSRC as libc::c_int as libc::c_uint
    {
        /* brilliant, how exactly did we get here? */
        return -(1 as libc::c_int) as ssize_t;
    }
    (*tgt).length = 0 as libc::c_int as size_t;
    archive_strncat(
        tgt,
        _ver.as_ptr() as *const libc::c_void,
        (if _ver.as_ptr().is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(_ver.as_ptr())
        }),
    );
    archive_string_sprintf(
        tgt,
        b"WARC-Type: %s\r\n\x00" as *const u8 as *const libc::c_char,
        _typ[hdr.type_0 as usize],
    );
    if !hdr.tgturi.is_null() {
        /* check if there's a xyz:// */
        static mut _uri: [libc::c_char; 1] =
            unsafe { *::std::mem::transmute::<&[u8; 1], &[libc::c_char; 1]>(b"\x00") };
        static mut _fil: [libc::c_char; 8] =
            unsafe { *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"file://\x00") };
        let mut u: *const libc::c_char = 0 as *const libc::c_char;
        let mut chk: *mut libc::c_char = strchr(hdr.tgturi, ':' as i32);
        if !chk.is_null()
            && *chk.offset(1 as libc::c_uint as isize) as libc::c_int == '/' as i32
            && *chk.offset(2 as libc::c_uint as isize) as libc::c_int == '/' as i32
        {
            /* yep, it's definitely a URI */
            u = _uri.as_ptr()
        } else {
            /* hm, best to prepend file:// then */
            u = _fil.as_ptr()
        }
        archive_string_sprintf(
            tgt,
            b"WARC-Target-URI: %s%s\r\n\x00" as *const u8 as *const libc::c_char,
            u,
            hdr.tgturi,
        );
    }
    /* record time is usually when the http is sent off,
     * just treat the archive writing as such for a moment */
    xstrftime(
        tgt,
        b"WARC-Date: %Y-%m-%dT%H:%M:%SZ\r\n\x00" as *const u8 as *const libc::c_char,
        hdr.rtime,
    );
    /* while we're at it, record the mtime */
    xstrftime(
        tgt,
        b"Last-Modified: %Y-%m-%dT%H:%M:%SZ\r\n\x00" as *const u8 as *const libc::c_char,
        hdr.mtime,
    );
    if hdr.recid.is_null() {
        /* generate one, grrrr */
        let mut u_0: warc_uuid_t = warc_uuid_t { u: [0; 4] };
        _gen_uuid(&mut u_0);
        /* Unfortunately, archive_string_sprintf does not
         * handle the minimum number following '%'.
         * So we have to use snprintf function here instead
         * of archive_string_snprintf function. */
        snprintf(
            std_uuid.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 48]>() as libc::c_ulong,
            b"<urn:uuid:%08x-%04x-%04x-%04x-%04x%08x>\x00" as *const u8 as *const libc::c_char,
            u_0.u[0 as libc::c_uint as usize],
            u_0.u[1 as libc::c_uint as usize] >> 16 as libc::c_uint,
            u_0.u[1 as libc::c_uint as usize] & 0xffff as libc::c_uint,
            u_0.u[2 as libc::c_uint as usize] >> 16 as libc::c_uint,
            u_0.u[2 as libc::c_uint as usize] & 0xffff as libc::c_uint,
            u_0.u[3 as libc::c_uint as usize],
        );
        hdr.recid = std_uuid.as_mut_ptr()
    }
    /* record-id is mandatory, fingers crossed we won't fail */
    archive_string_sprintf(
        tgt,
        b"WARC-Record-ID: %s\r\n\x00" as *const u8 as *const libc::c_char,
        hdr.recid,
    );
    if !hdr.cnttyp.is_null() {
        archive_string_sprintf(
            tgt,
            b"Content-Type: %s\r\n\x00" as *const u8 as *const libc::c_char,
            hdr.cnttyp,
        );
    }
    /* next one is mandatory */
    archive_string_sprintf(
        tgt,
        b"Content-Length: %ju\r\n\x00" as *const u8 as *const libc::c_char,
        hdr.cntlen,
    );
    /* */
    archive_strncat(
        tgt,
        b"\r\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    return if (*tgt).length >= tsz {
        -(1 as libc::c_int) as libc::c_long
    } else {
        (*tgt).length as ssize_t
    };
}
unsafe extern "C" fn _gen_uuid(mut tgt: *mut warc_uuid_t) -> libc::c_int {
    archive_random(
        (*tgt).u.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uint; 4]>() as libc::c_ulong,
    );
    /* obey uuid version 4 rules */
    (*tgt).u[1 as libc::c_uint as usize] &= 0xffff0fff as libc::c_uint;
    (*tgt).u[1 as libc::c_uint as usize] |= 0x4000 as libc::c_uint;
    (*tgt).u[2 as libc::c_uint as usize] &= 0x3fffffff as libc::c_uint;
    (*tgt).u[2 as libc::c_uint as usize] |= 0x80000000 as libc::c_uint;
    return 0 as libc::c_int;
}
/* archive_write_set_format_warc.c ends here */
