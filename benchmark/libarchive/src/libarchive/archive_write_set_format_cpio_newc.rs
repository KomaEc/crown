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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_new() -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_devmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_devminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_ino64(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_rdevmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_rdevminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_size_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
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
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_nulls(_: *mut archive_write, _: size_t) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type dev_t = __dev_t;
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
pub struct cpio {
    pub entry_bytes_remaining: uint64_t,
    pub padding: libc::c_int,
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
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const ERANGE: libc::c_int = 34 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO: libc::c_int = 0x10000 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_SVR4_NOCRC: libc::c_int = ARCHIVE_FORMAT_CPIO | 4 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
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
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
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
pub const c_magic_offset: libc::c_int = 0 as libc::c_int;
pub const c_magic_size: libc::c_int = 6 as libc::c_int;
pub const c_ino_offset: libc::c_int = 6 as libc::c_int;
pub const c_ino_size: libc::c_int = 8 as libc::c_int;
pub const c_mode_offset: libc::c_int = 14 as libc::c_int;
pub const c_mode_size: libc::c_int = 8 as libc::c_int;
pub const c_uid_offset: libc::c_int = 22 as libc::c_int;
pub const c_uid_size: libc::c_int = 8 as libc::c_int;
pub const c_gid_offset: libc::c_int = 30 as libc::c_int;
pub const c_gid_size: libc::c_int = 8 as libc::c_int;
pub const c_nlink_offset: libc::c_int = 38 as libc::c_int;
pub const c_nlink_size: libc::c_int = 8 as libc::c_int;
pub const c_mtime_offset: libc::c_int = 46 as libc::c_int;
pub const c_mtime_size: libc::c_int = 8 as libc::c_int;
pub const c_filesize_offset: libc::c_int = 54 as libc::c_int;
pub const c_filesize_size: libc::c_int = 8 as libc::c_int;
pub const c_devmajor_offset: libc::c_int = 62 as libc::c_int;
pub const c_devmajor_size: libc::c_int = 8 as libc::c_int;
pub const c_devminor_offset: libc::c_int = 70 as libc::c_int;
pub const c_devminor_size: libc::c_int = 8 as libc::c_int;
pub const c_rdevmajor_offset: libc::c_int = 78 as libc::c_int;
pub const c_rdevmajor_size: libc::c_int = 8 as libc::c_int;
pub const c_rdevminor_offset: libc::c_int = 86 as libc::c_int;
pub const c_rdevminor_size: libc::c_int = 8 as libc::c_int;
pub const c_namesize_offset: libc::c_int = 94 as libc::c_int;
pub const c_namesize_size: libc::c_int = 8 as libc::c_int;
pub const c_checksum_offset: libc::c_int = 102 as libc::c_int;
pub const c_checksum_size: libc::c_int = 8 as libc::c_int;
pub const c_header_size: libc::c_int = 110 as libc::c_int;
/*
 * Set output format to 'cpio' format.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_cpio_newc(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_cpio_newc\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If someone else was already registered, unregister them. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    cpio = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpio>() as libc::c_ulong,
    ) as *mut cpio;
    if cpio.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate cpio data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*a).format_data = cpio as *mut libc::c_void;
    (*a).format_name = b"cpio\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        archive_write_newc_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        archive_write_newc_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        archive_write_newc_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry = Some(
        archive_write_newc_finish_entry
            as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_close = Some(
        archive_write_newc_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_free =
        Some(archive_write_newc_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_SVR4_NOCRC;
    (*a).archive.archive_format_name = b"SVR4 cpio nocrc\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_newc_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut cpio: *mut cpio = (*a).format_data as *mut cpio;
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
            (*cpio).opt_sconv =
                archive_string_conversion_to_charset(&mut (*a).archive, val, 0 as libc::c_int);
            if !(*cpio).opt_sconv.is_null() {
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
    return -(20 as libc::c_int); /* Include trailing null. */
}
unsafe extern "C" fn get_sconv(mut a: *mut archive_write) -> *mut archive_string_conv {
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    cpio = (*a).format_data as *mut cpio;
    sconv = (*cpio).opt_sconv;
    if sconv.is_null() {
        if (*cpio).init_default_conversion == 0 {
            (*cpio).sconv_default = archive_string_default_conversion_for_write(&mut (*a).archive);
            (*cpio).init_default_conversion = 1 as libc::c_int
        }
        sconv = (*cpio).sconv_default
    }
    return sconv;
}
unsafe extern "C" fn archive_write_newc_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    if archive_entry_filetype(entry) == 0 as libc::c_int as libc::c_uint {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Filetype required\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    if _archive_entry_pathname_l(entry, &mut path, &mut len, get_sconv(a)) != 0 as libc::c_int
        && errno == ENOMEM
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for Pathname\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_ulong
        || path.is_null()
        || *path.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Pathname required\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    if archive_entry_hardlink(entry).is_null()
        && (archive_entry_size_is_set(entry) == 0
            || archive_entry_size(entry) < 0 as libc::c_int as libc::c_long)
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Size required\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    return write_header(a, entry);
}
unsafe extern "C" fn write_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ino: int64_t = 0;
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut pathlength: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ret_final: libc::c_int = 0;
    let mut h: [libc::c_char; 110] = [0; 110];
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut entry_main: *mut archive_entry = 0 as *mut archive_entry;
    let mut len: size_t = 0;
    let mut pad: libc::c_int = 0;
    cpio = (*a).format_data as *mut cpio;
    ret_final = ARCHIVE_OK;
    sconv = get_sconv(a);
    entry_main = NULL as *mut archive_entry;
    ret = _archive_entry_pathname_l(entry, &mut path, &mut len, sconv);
    if ret != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Pathname\x00" as *const u8 as *const libc::c_char,
            );
            ret_final = ARCHIVE_FATAL;
            current_block = 14048450686769486295;
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Can\'t translate pathname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(entry),
                archive_string_conversion_charset_name(sconv),
            );
            ret_final = ARCHIVE_WARN;
            current_block = 13056961889198038528;
        }
    } else {
        current_block = 13056961889198038528;
    }
    match current_block {
        13056961889198038528 => {
            pathlength = len as libc::c_int + 1 as libc::c_int;
            memset(
                h.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                c_header_size as libc::c_ulong,
            );
            format_hex(
                0x70701 as libc::c_int as int64_t,
                h.as_mut_ptr().offset(c_magic_offset as isize) as *mut libc::c_void,
                c_magic_size,
            );
            format_hex(
                archive_entry_devmajor(entry) as int64_t,
                h.as_mut_ptr().offset(c_devmajor_offset as isize) as *mut libc::c_void,
                c_devmajor_size,
            );
            format_hex(
                archive_entry_devminor(entry) as int64_t,
                h.as_mut_ptr().offset(c_devminor_offset as isize) as *mut libc::c_void,
                c_devminor_size,
            );
            ino = archive_entry_ino64(entry);
            if ino > 0xffffffff as libc::c_uint as libc::c_long {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ERANGE,
                    b"large inode number truncated\x00" as *const u8 as *const libc::c_char,
                );
                ret_final = ARCHIVE_WARN
            }
            /* TODO: Set ret_final to ARCHIVE_WARN if any of these overflow. */
            format_hex(
                ino & 0xffffffff as libc::c_uint as libc::c_long,
                h.as_mut_ptr().offset(c_ino_offset as isize) as *mut libc::c_void,
                c_ino_size,
            );
            format_hex(
                archive_entry_mode(entry) as int64_t,
                h.as_mut_ptr().offset(c_mode_offset as isize) as *mut libc::c_void,
                c_mode_size,
            );
            format_hex(
                archive_entry_uid(entry),
                h.as_mut_ptr().offset(c_uid_offset as isize) as *mut libc::c_void,
                c_uid_size,
            );
            format_hex(
                archive_entry_gid(entry),
                h.as_mut_ptr().offset(c_gid_offset as isize) as *mut libc::c_void,
                c_gid_size,
            );
            format_hex(
                archive_entry_nlink(entry) as int64_t,
                h.as_mut_ptr().offset(c_nlink_offset as isize) as *mut libc::c_void,
                c_nlink_size,
            );
            if archive_entry_filetype(entry) == AE_IFBLK as mode_t
                || archive_entry_filetype(entry) == AE_IFCHR as mode_t
            {
                format_hex(
                    archive_entry_rdevmajor(entry) as int64_t,
                    h.as_mut_ptr().offset(c_rdevmajor_offset as isize) as *mut libc::c_void,
                    c_rdevmajor_size,
                );
                format_hex(
                    archive_entry_rdevminor(entry) as int64_t,
                    h.as_mut_ptr().offset(c_rdevminor_offset as isize) as *mut libc::c_void,
                    c_rdevminor_size,
                );
            } else {
                format_hex(
                    0 as libc::c_int as int64_t,
                    h.as_mut_ptr().offset(c_rdevmajor_offset as isize) as *mut libc::c_void,
                    c_rdevmajor_size,
                );
                format_hex(
                    0 as libc::c_int as int64_t,
                    h.as_mut_ptr().offset(c_rdevminor_offset as isize) as *mut libc::c_void,
                    c_rdevminor_size,
                );
            }
            format_hex(
                archive_entry_mtime(entry),
                h.as_mut_ptr().offset(c_mtime_offset as isize) as *mut libc::c_void,
                c_mtime_size,
            );
            format_hex(
                pathlength as int64_t,
                h.as_mut_ptr().offset(c_namesize_offset as isize) as *mut libc::c_void,
                c_namesize_size,
            );
            format_hex(
                0 as libc::c_int as int64_t,
                h.as_mut_ptr().offset(c_checksum_offset as isize) as *mut libc::c_void,
                c_checksum_size,
            );
            /* Non-regular files don't store bodies. */
            if archive_entry_filetype(entry) != AE_IFREG as mode_t {
                archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            }
            /* Symlinks get the link written as the body of the entry. */
            ret = _archive_entry_symlink_l(entry, &mut p, &mut len, sconv);
            if ret != 0 as libc::c_int {
                if errno == ENOMEM {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"Can\'t allocate memory for Likname\x00" as *const u8
                            as *const libc::c_char,
                    );
                    ret_final = ARCHIVE_FATAL;
                    current_block = 14048450686769486295;
                } else {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Can\'t translate linkname \'%s\' to %s\x00" as *const u8
                            as *const libc::c_char,
                        archive_entry_symlink(entry),
                        archive_string_conversion_charset_name(sconv),
                    );
                    ret_final = ARCHIVE_WARN;
                    current_block = 12381812505308290051;
                }
            } else {
                current_block = 12381812505308290051;
            }
            match current_block {
                14048450686769486295 => {}
                _ => {
                    if len > 0 as libc::c_int as libc::c_ulong
                        && !p.is_null()
                        && *p as libc::c_int != '\u{0}' as i32
                    {
                        ret = format_hex(
                            strlen(p) as int64_t,
                            h.as_mut_ptr().offset(c_filesize_offset as isize) as *mut libc::c_void,
                            c_filesize_size,
                        )
                    } else {
                        ret = format_hex(
                            archive_entry_size(entry),
                            h.as_mut_ptr().offset(c_filesize_offset as isize) as *mut libc::c_void,
                            c_filesize_size,
                        )
                    }
                    if ret != 0 {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ERANGE,
                            b"File is too large for this format.\x00" as *const u8
                                as *const libc::c_char,
                        );
                        ret_final = ARCHIVE_FAILED
                    } else {
                        ret = __archive_write_output(
                            a,
                            h.as_mut_ptr() as *const libc::c_void,
                            c_header_size as size_t,
                        );
                        if ret != ARCHIVE_OK {
                            ret_final = ARCHIVE_FATAL
                        } else {
                            /* Pad pathname to even length. */
                            ret = __archive_write_output(
                                a,
                                path as *const libc::c_void,
                                pathlength as size_t,
                            );
                            if ret != ARCHIVE_OK {
                                ret_final = ARCHIVE_FATAL
                            } else {
                                pad = 3 as libc::c_int
                                    & 1 as libc::c_int + !(pathlength + 110 as libc::c_int);
                                if pad != 0 {
                                    ret = __archive_write_output(
                                        a,
                                        b"\x00\x00\x00\x00" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        pad as size_t,
                                    );
                                    if ret != ARCHIVE_OK {
                                        ret_final = ARCHIVE_FATAL;
                                        current_block = 14048450686769486295;
                                    } else {
                                        current_block = 15594839951440953787;
                                    }
                                } else {
                                    current_block = 15594839951440953787;
                                }
                                match current_block {
                                    14048450686769486295 => {}
                                    _ => {
                                        (*cpio).entry_bytes_remaining =
                                            archive_entry_size(entry) as uint64_t;
                                        (*cpio).padding = (3 as libc::c_int as libc::c_ulong
                                            & (1 as libc::c_int as libc::c_ulong)
                                                .wrapping_add(!(*cpio).entry_bytes_remaining))
                                            as libc::c_int;
                                        /* Write the symlink now. */
                                        if !p.is_null() && *p as libc::c_int != '\u{0}' as i32 {
                                            ret = __archive_write_output(
                                                a,
                                                p as *const libc::c_void,
                                                strlen(p),
                                            );
                                            if ret != ARCHIVE_OK {
                                                ret_final = ARCHIVE_FATAL
                                            } else {
                                                pad = (3 as libc::c_int as libc::c_ulong
                                                    & (1 as libc::c_int as libc::c_ulong)
                                                        .wrapping_add(!strlen(p)))
                                                    as libc::c_int;
                                                ret = __archive_write_output(
                                                    a,
                                                    b"\x00\x00\x00\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *const libc::c_void,
                                                    pad as size_t,
                                                );
                                                if ret != ARCHIVE_OK {
                                                    ret_final = ARCHIVE_FATAL
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    archive_entry_free(entry_main);
    return ret_final;
}
unsafe extern "C" fn archive_write_newc_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut ret: libc::c_int = 0;
    cpio = (*a).format_data as *mut cpio;
    if s > (*cpio).entry_bytes_remaining {
        s = (*cpio).entry_bytes_remaining
    }
    ret = __archive_write_output(a, buff, s);
    (*cpio).entry_bytes_remaining =
        ((*cpio).entry_bytes_remaining as libc::c_ulong).wrapping_sub(s) as uint64_t as uint64_t;
    if ret >= 0 as libc::c_int {
        return s as ssize_t;
    } else {
        return ret as ssize_t;
    };
}
/*
 * Format a number into the specified field.
 */
unsafe extern "C" fn format_hex(
    mut v: int64_t,
    mut p: *mut libc::c_void,
    mut digits: libc::c_int,
) -> libc::c_int {
    let mut max: int64_t = 0;
    let mut ret: libc::c_int = 0;
    max = ((1 as libc::c_int as int64_t) << digits * 4 as libc::c_int)
        - 1 as libc::c_int as libc::c_long;
    if v >= 0 as libc::c_int as libc::c_long && v <= max {
        format_hex_recursive(v, p as *mut libc::c_char, digits);
        ret = 0 as libc::c_int
    } else {
        format_hex_recursive(max, p as *mut libc::c_char, digits);
        ret = -(1 as libc::c_int)
    }
    return ret;
}
unsafe extern "C" fn format_hex_recursive(
    mut v: int64_t,
    mut p: *mut libc::c_char,
    mut s: libc::c_int,
) -> int64_t {
    if s == 0 as libc::c_int {
        return v;
    }
    v = format_hex_recursive(v, p.offset(1 as libc::c_int as isize), s - 1 as libc::c_int);
    *p = (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\x00"))
        [(v & 0xf as libc::c_int as libc::c_long) as usize];
    return v >> 4 as libc::c_int;
}
unsafe extern "C" fn archive_write_newc_close(mut a: *mut archive_write) -> libc::c_int {
    let mut er: libc::c_int = 0;
    let mut trailer: *mut archive_entry = 0 as *mut archive_entry;
    trailer = archive_entry_new();
    archive_entry_set_nlink(trailer, 1 as libc::c_int as libc::c_uint);
    archive_entry_set_size(trailer, 0 as libc::c_int as la_int64_t);
    archive_entry_set_pathname(
        trailer,
        b"TRAILER!!!\x00" as *const u8 as *const libc::c_char,
    );
    /* Bypass the required data checks. */
    er = write_header(a, trailer);
    archive_entry_free(trailer);
    return er;
}
unsafe extern "C" fn archive_write_newc_free(mut a: *mut archive_write) -> libc::c_int {
    let mut cpio: *mut cpio = 0 as *mut cpio;
    cpio = (*a).format_data as *mut cpio;
    free(cpio as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_newc_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut cpio: *mut cpio = 0 as *mut cpio;
    cpio = (*a).format_data as *mut cpio;
    return __archive_write_nulls(
        a,
        (*cpio)
            .entry_bytes_remaining
            .wrapping_add((*cpio).padding as libc::c_ulong),
    );
}
