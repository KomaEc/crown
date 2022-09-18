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
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn _archive_entry_hardlink_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_pathname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_symlink_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_default_conversion_for_write(_: *mut archive) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_conversion_to_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
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
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_nulls(_: *mut archive_write, _: size_t) -> libc::c_int;
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
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct v7tar {
    pub entry_bytes_remaining: uint64_t,
    pub entry_padding: uint64_t,
    pub opt_sconv: *mut archive_string_conv,
    pub sconv_default: *mut archive_string_conv,
    pub init_default_conversion: libc::c_int,
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
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const ENAMETOOLONG: libc::c_int = 36 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const ERANGE: libc::c_int = 34 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR: libc::c_int = 0x30000 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
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
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const archive_entry_hardlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_hardlink_l;
pub const archive_entry_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_pathname_l;
pub const archive_entry_symlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_symlink_l;
/*
 * Define structure of POSIX 'v7tar' tar header.
 */
pub const V7TAR_name_offset: libc::c_int = 0 as libc::c_int;
pub const V7TAR_name_size: libc::c_int = 100 as libc::c_int;
pub const V7TAR_mode_offset: libc::c_int = 100 as libc::c_int;
pub const V7TAR_mode_size: libc::c_int = 6 as libc::c_int;
pub const V7TAR_mode_max_size: libc::c_int = 8 as libc::c_int;
pub const V7TAR_uid_offset: libc::c_int = 108 as libc::c_int;
pub const V7TAR_uid_size: libc::c_int = 6 as libc::c_int;
pub const V7TAR_uid_max_size: libc::c_int = 8 as libc::c_int;
pub const V7TAR_gid_offset: libc::c_int = 116 as libc::c_int;
pub const V7TAR_gid_size: libc::c_int = 6 as libc::c_int;
pub const V7TAR_gid_max_size: libc::c_int = 8 as libc::c_int;
pub const V7TAR_size_offset: libc::c_int = 124 as libc::c_int;
pub const V7TAR_size_size: libc::c_int = 11 as libc::c_int;
pub const V7TAR_size_max_size: libc::c_int = 12 as libc::c_int;
pub const V7TAR_mtime_offset: libc::c_int = 136 as libc::c_int;
pub const V7TAR_mtime_size: libc::c_int = 11 as libc::c_int;
pub const V7TAR_mtime_max_size: libc::c_int = 12 as libc::c_int;
pub const V7TAR_checksum_offset: libc::c_int = 148 as libc::c_int;
pub const V7TAR_typeflag_offset: libc::c_int = 156 as libc::c_int;
pub const V7TAR_linkname_offset: libc::c_int = 157 as libc::c_int;
pub const V7TAR_linkname_size: libc::c_int = 100 as libc::c_int;
/*
 * A filled-in copy of the header for initialization.
 */
static mut template_header: [libc::c_char; 512] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
/*
 * Set output format to 'v7tar' format.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_v7tar(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut v7tar: *mut v7tar = 0 as *mut v7tar;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_v7tar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If someone else was already registered, unregister them. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    /* Basic internal sanity test. */
    if ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
        != 512 as libc::c_int as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Internal: template_header wrong size: %zu should be 512\x00" as *const u8
                as *const libc::c_char,
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        return -(30 as libc::c_int);
    }
    v7tar = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<v7tar>() as libc::c_ulong,
    ) as *mut v7tar;
    if v7tar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate v7tar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*a).format_data = v7tar as *mut libc::c_void;
    (*a).format_name = b"tar (non-POSIX)\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        archive_write_v7tar_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        archive_write_v7tar_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        archive_write_v7tar_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_close = Some(
        archive_write_v7tar_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_free = Some(
        archive_write_v7tar_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_finish_entry = Some(
        archive_write_v7tar_finish_entry
            as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).archive.archive_format = ARCHIVE_FORMAT_TAR;
    (*a).archive.archive_format_name = b"tar (non-POSIX)\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_v7tar_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut v7tar: *mut v7tar = (*a).format_data as *mut v7tar;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if val.is_null()
            || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"%s: hdrcharset option needs a character-set name\x00" as *const u8
                    as *const libc::c_char,
                (*a).format_name,
            );
        } else {
            (*v7tar).opt_sconv =
                archive_string_conversion_to_charset(&mut (*a).archive, val, 0 as libc::c_int);
            if !(*v7tar).opt_sconv.is_null() {
                ret = ARCHIVE_OK
            } else {
                ret = ARCHIVE_FATAL
            }
        }
        return ret;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn archive_write_v7tar_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut buff: [libc::c_char; 512] = [0; 512];
    let mut ret: libc::c_int = 0;
    let mut ret2: libc::c_int = 0;
    let mut v7tar: *mut v7tar = 0 as *mut v7tar;
    let mut entry_main: *mut archive_entry = 0 as *mut archive_entry;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    v7tar = (*a).format_data as *mut v7tar;
    /* Setup default string conversion. */
    if (*v7tar).opt_sconv.is_null() {
        if (*v7tar).init_default_conversion == 0 {
            (*v7tar).sconv_default = archive_string_default_conversion_for_write(&mut (*a).archive);
            (*v7tar).init_default_conversion = 1 as libc::c_int
        }
        sconv = (*v7tar).sconv_default
    } else {
        sconv = (*v7tar).opt_sconv
    }
    /* Sanity check. */
    if archive_entry_pathname(entry).is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Can\'t record entry in tar file without pathname\x00" as *const u8
                as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* Only regular files (not hardlinks) have data. */
    if !archive_entry_hardlink(entry).is_null()
        || !archive_entry_symlink(entry).is_null()
        || !(archive_entry_filetype(entry) == AE_IFREG as mode_t)
    {
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
    }
    if AE_IFDIR as mode_t == archive_entry_filetype(entry) {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut path_length: size_t = 0;
        /*
         * Ensure a trailing '/'.  Modify the entry so
         * the client sees the change.
         */
        p = archive_entry_pathname(entry);
        /*
         * On Windows, this is a backup operation just in
         * case getting WCS failed. On POSIX, this is a
         * normal operation.
         */
        if !p.is_null()
            && *p.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            && *p.offset(strlen(p).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != '/' as i32
        {
            let mut as_0: archive_string = archive_string {
                s: 0 as *mut libc::c_char,
                length: 0,
                buffer_length: 0,
            };
            as_0.s = NULL as *mut libc::c_char;
            as_0.length = 0 as libc::c_int as size_t;
            as_0.buffer_length = 0 as libc::c_int as size_t;
            path_length = strlen(p);
            if archive_string_ensure(
                &mut as_0,
                path_length.wrapping_add(2 as libc::c_int as libc::c_ulong),
            )
            .is_null()
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate v7tar data\x00" as *const u8 as *const libc::c_char,
                );
                archive_string_free(&mut as_0);
                return -(30 as libc::c_int);
            }
            as_0.length = 0 as libc::c_int as size_t;
            archive_strncat(&mut as_0, p as *const libc::c_void, path_length);
            archive_strappend_char(&mut as_0, '/' as i32 as libc::c_char);
            archive_entry_copy_pathname(entry, as_0.s);
            archive_string_free(&mut as_0);
        }
    }
    entry_main = NULL as *mut archive_entry;
    ret = format_header_v7tar(a, buff.as_mut_ptr(), entry, 1 as libc::c_int, sconv);
    if ret < ARCHIVE_WARN {
        archive_entry_free(entry_main);
        return ret;
    }
    ret2 = __archive_write_output(
        a,
        buff.as_mut_ptr() as *const libc::c_void,
        512 as libc::c_int as size_t,
    );
    if ret2 < ARCHIVE_WARN {
        archive_entry_free(entry_main);
        return ret2;
    }
    if ret2 < ret {
        ret = ret2
    }
    (*v7tar).entry_bytes_remaining = archive_entry_size(entry) as uint64_t;
    (*v7tar).entry_padding = (0x1ff as libc::c_int as libc::c_long
        & -((*v7tar).entry_bytes_remaining as int64_t)) as uint64_t;
    archive_entry_free(entry_main);
    return ret;
}
/*
 * Format a basic 512-byte "v7tar" header.
 *
 * Returns -1 if format failed (due to field overflow).
 * Note that this always formats as much of the header as possible.
 * If "strict" is set to zero, it will extend numeric fields as
 * necessary (overwriting terminators or using base-256 extensions).
 *
 */
unsafe extern "C" fn format_header_v7tar(
    mut a: *mut archive_write,
    mut h: *mut libc::c_char,
    mut entry: *mut archive_entry,
    mut strict: libc::c_int,
    mut sconv: *mut archive_string_conv,
) -> libc::c_int {
    let mut checksum: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut copy_length: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut pp: *const libc::c_char = 0 as *const libc::c_char;
    let mut mytartype: libc::c_int = 0;
    ret = 0 as libc::c_int;
    mytartype = -(1 as libc::c_int);
    /*
     * The "template header" already includes the "v7tar"
     * signature, various end-of-field markers and other required
     * elements.
     */
    memcpy(
        h as *mut libc::c_void,
        &template_header as *const [libc::c_char; 512] as *const libc::c_void,
        512 as libc::c_int as libc::c_ulong,
    );
    /*
     * Because the block is already null-filled, and strings
     * are allowed to exactly fill their destination (without null),
     * I use memcpy(dest, src, strlen()) here a lot to copy strings.
     */
    r = _archive_entry_pathname_l(entry, &mut pp, &mut copy_length, sconv);
    if r != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Pathname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Can\'t translate pathname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
            pp,
            archive_string_conversion_charset_name(sconv),
        );
        ret = ARCHIVE_WARN
    }
    if strict != 0 && copy_length < V7TAR_name_size as libc::c_ulong {
        memcpy(
            h.offset(V7TAR_name_offset as isize) as *mut libc::c_void,
            pp as *const libc::c_void,
            copy_length,
        );
    } else if strict == 0 && copy_length <= V7TAR_name_size as libc::c_ulong {
        memcpy(
            h.offset(V7TAR_name_offset as isize) as *mut libc::c_void,
            pp as *const libc::c_void,
            copy_length,
        );
    } else {
        /* Prefix is too long. */
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENAMETOOLONG,
            b"Pathname too long\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FAILED
    }
    r = _archive_entry_hardlink_l(entry, &mut p, &mut copy_length, sconv);
    if r != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Linkname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Can\'t translate linkname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
            p,
            archive_string_conversion_charset_name(sconv),
        );
        ret = ARCHIVE_WARN
    }
    if copy_length > 0 as libc::c_int as libc::c_ulong {
        mytartype = '1' as i32
    } else {
        r = _archive_entry_symlink_l(entry, &mut p, &mut copy_length, sconv);
        if r != 0 as libc::c_int {
            if errno == ENOMEM {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate memory for Linkname\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Can\'t translate linkname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
                p,
                archive_string_conversion_charset_name(sconv),
            );
            ret = ARCHIVE_WARN
        }
    }
    if copy_length > 0 as libc::c_int as libc::c_ulong {
        if copy_length >= V7TAR_linkname_size as libc::c_ulong {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENAMETOOLONG,
                b"Link contents too long\x00" as *const u8 as *const libc::c_char,
            );
            ret = ARCHIVE_FAILED;
            copy_length = V7TAR_linkname_size as size_t
        }
        memcpy(
            h.offset(V7TAR_linkname_offset as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            copy_length,
        );
    }
    if format_number(
        (archive_entry_mode(entry) & 0o7777 as libc::c_int as libc::c_uint) as int64_t,
        h.offset(V7TAR_mode_offset as isize),
        V7TAR_mode_size,
        V7TAR_mode_max_size,
        strict,
    ) != 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"Numeric mode too large\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FAILED
    }
    if format_number(
        archive_entry_uid(entry),
        h.offset(V7TAR_uid_offset as isize),
        V7TAR_uid_size,
        V7TAR_uid_max_size,
        strict,
    ) != 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"Numeric user ID too large\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FAILED
    }
    if format_number(
        archive_entry_gid(entry),
        h.offset(V7TAR_gid_offset as isize),
        V7TAR_gid_size,
        V7TAR_gid_max_size,
        strict,
    ) != 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"Numeric group ID too large\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FAILED
    }
    if format_number(
        archive_entry_size(entry),
        h.offset(V7TAR_size_offset as isize),
        V7TAR_size_size,
        V7TAR_size_max_size,
        strict,
    ) != 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"File size out of range\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FAILED
    }
    if format_number(
        archive_entry_mtime(entry),
        h.offset(V7TAR_mtime_offset as isize),
        V7TAR_mtime_size,
        V7TAR_mtime_max_size,
        strict,
    ) != 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"File modification time too large\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FAILED
    }
    if mytartype >= 0 as libc::c_int {
        *h.offset(V7TAR_typeflag_offset as isize) = mytartype as libc::c_char
    } else {
        match archive_entry_filetype(entry) {
            32768 | 16384 => {}
            40960 => *h.offset(V7TAR_typeflag_offset as isize) = '2' as i32 as libc::c_char,
            _ => {
                /* AE_IFBLK, AE_IFCHR, AE_IFIFO, AE_IFSOCK
                 * and unknown */
                __archive_write_entry_filetype_unsupported(
                    &mut (*a).archive,
                    entry,
                    b"v7tar\x00" as *const u8 as *const libc::c_char,
                );
                ret = ARCHIVE_FAILED
            }
        }
    }
    checksum = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        checksum = checksum.wrapping_add(
            255 as libc::c_int as libc::c_uint & *h.offset(i as isize) as libc::c_uint,
        );
        i += 1
    }
    format_octal(
        checksum as int64_t,
        h.offset(V7TAR_checksum_offset as isize),
        6 as libc::c_int,
    );
    /* Can't be pre-set in the template. */
    *h.offset((V7TAR_checksum_offset + 6 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
    return ret;
}
/*
 * Format a number into a field, with some intelligence.
 */
unsafe extern "C" fn format_number(
    mut v: int64_t,
    mut p: *mut libc::c_char,
    mut s: libc::c_int,
    mut maxsize: libc::c_int,
    mut strict: libc::c_int,
) -> libc::c_int {
    let mut limit: int64_t = 0;
    limit = (1 as libc::c_int as int64_t) << s * 3 as libc::c_int;
    /* "Strict" only permits octal values with proper termination. */
    if strict != 0 {
        return format_octal(v, p, s);
    }
    /*
     * In non-strict mode, we allow the number to overwrite one or
     * more bytes of the field termination.  Even old tar
     * implementations should be able to handle this with no
     * problem.
     */
    if v >= 0 as libc::c_int as libc::c_long {
        while s <= maxsize {
            if v < limit {
                return format_octal(v, p, s);
            }
            s += 1;
            limit <<= 3 as libc::c_int
        }
    }
    /* Base-256 can handle any number, positive or negative. */
    return format_256(v, p, maxsize);
}
/*
 * Format a number into the specified field using base-256.
 */
unsafe extern "C" fn format_256(
    mut v: int64_t,
    mut p: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    p = p.offset(s as isize); /* Set the base-256 marker bit. */
    loop {
        let fresh0 = s;
        s = s - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        p = p.offset(-1);
        *p = (v & 0xff as libc::c_int as libc::c_long) as libc::c_char;
        v >>= 8 as libc::c_int
    }
    *p = (*p as libc::c_int | 0x80 as libc::c_int) as libc::c_char;
    return 0 as libc::c_int;
}
/*
 * Format a number into the specified field.
 */
unsafe extern "C" fn format_octal(
    mut v: int64_t,
    mut p: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    len = s;
    /* Octal values can't be negative, so use 0. */
    if v < 0 as libc::c_int as libc::c_long {
        loop {
            let fresh1 = len; /* Start at the end and work backwards. */
            len = len - 1;
            if !(fresh1 > 0 as libc::c_int) {
                break;
            }
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = '0' as i32 as libc::c_char
        }
        return -(1 as libc::c_int);
    }
    p = p.offset(s as isize);
    loop {
        let fresh3 = s;
        s = s - 1;
        if !(fresh3 > 0 as libc::c_int) {
            break;
        }
        p = p.offset(-1);
        *p = ('0' as i32 as libc::c_long + (v & 7 as libc::c_int as libc::c_long)) as libc::c_char;
        v >>= 3 as libc::c_int
    }
    if v == 0 as libc::c_int as libc::c_long {
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
unsafe extern "C" fn archive_write_v7tar_close(mut a: *mut archive_write) -> libc::c_int {
    return __archive_write_nulls(a, (512 as libc::c_int * 2 as libc::c_int) as size_t);
}
unsafe extern "C" fn archive_write_v7tar_free(mut a: *mut archive_write) -> libc::c_int {
    let mut v7tar: *mut v7tar = 0 as *mut v7tar;
    v7tar = (*a).format_data as *mut v7tar;
    free(v7tar as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_v7tar_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut v7tar: *mut v7tar = 0 as *mut v7tar;
    let mut ret: libc::c_int = 0;
    v7tar = (*a).format_data as *mut v7tar;
    ret = __archive_write_nulls(
        a,
        (*v7tar)
            .entry_bytes_remaining
            .wrapping_add((*v7tar).entry_padding),
    );
    (*v7tar).entry_padding = 0 as libc::c_int as uint64_t;
    (*v7tar).entry_bytes_remaining = (*v7tar).entry_padding;
    return ret;
}
unsafe extern "C" fn archive_write_v7tar_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut v7tar: *mut v7tar = 0 as *mut v7tar;
    let mut ret: libc::c_int = 0;
    v7tar = (*a).format_data as *mut v7tar;
    if s > (*v7tar).entry_bytes_remaining {
        s = (*v7tar).entry_bytes_remaining
    }
    ret = __archive_write_output(a, buff, s);
    (*v7tar).entry_bytes_remaining =
        ((*v7tar).entry_bytes_remaining as libc::c_ulong).wrapping_sub(s) as uint64_t as uint64_t;
    if ret != ARCHIVE_OK {
        return ret as ssize_t;
    }
    return s as ssize_t;
}
