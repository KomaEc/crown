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
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_write_set_bytes_in_last_block(
        _: *mut archive,
        bytes_in_last_block: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    /* The 'clone' function does a deep copy; all of the strings are copied too. */
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_fflags_text(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_gname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_rdevmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_rdevminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
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
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uintmax_t = __uintmax_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ptrdiff_t = libc::c_long;
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
pub struct shar {
    pub dump: libc::c_int,
    pub end_of_line: libc::c_int,
    pub entry: *mut archive_entry,
    pub has_data: libc::c_int,
    pub last_dir: *mut libc::c_char,
    pub outbuff: [libc::c_char; 45],
    pub outpos: size_t,
    pub wrote_header: libc::c_int,
    pub work: archive_string,
    pub quoted_name: archive_string,
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
pub const ARCHIVE_FORMAT_SHAR: libc::c_int = 0x20000 as libc::c_int;
pub const ARCHIVE_FORMAT_SHAR_BASE: libc::c_int = ARCHIVE_FORMAT_SHAR | 1 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FORMAT_SHAR_DUMP: libc::c_int = ARCHIVE_FORMAT_SHAR | 2 as libc::c_int;
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
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
/*
 * Copy the given string to the buffer, quoting all shell meta characters
 * found.
 */
unsafe extern "C" fn shar_quote(
    mut buf: *mut archive_string,
    mut str: *const libc::c_char,
    mut in_shell: libc::c_int,
) {
    static mut meta: [libc::c_char; 26] = unsafe {
        *::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"\n \t\'`\";&<>()|*?{}[]\\$!#^~\x00",
        )
    };
    let mut len: size_t = 0;
    while *str as libc::c_int != '\u{0}' as i32 {
        len = strcspn(str, meta.as_ptr());
        if len != 0 as libc::c_int as libc::c_ulong {
            archive_strncat(buf, str as *const libc::c_void, len);
            str = str.offset(len as isize)
        } else if *str as libc::c_int == '\n' as i32 {
            if in_shell != 0 {
                archive_strcat(
                    buf,
                    b"\"\n\"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            } else {
                archive_strcat(
                    buf,
                    b"\\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            str = str.offset(1)
        } else {
            archive_strappend_char(buf, '\\' as i32 as libc::c_char);
            archive_strappend_char(buf, *str);
            str = str.offset(1)
        }
    }
}
/*
 * Set output format to 'shar' format.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_shar(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut shar: *mut shar = 0 as *mut shar;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_shar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If someone else was already registered, unregister them. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    shar = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<shar>() as libc::c_ulong,
    ) as *mut shar;
    if shar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate shar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*shar).work.s = NULL as *mut libc::c_char;
    (*shar).work.length = 0 as libc::c_int as size_t;
    (*shar).work.buffer_length = 0 as libc::c_int as size_t;
    (*shar).quoted_name.s = NULL as *mut libc::c_char;
    (*shar).quoted_name.length = 0 as libc::c_int as size_t;
    (*shar).quoted_name.buffer_length = 0 as libc::c_int as size_t;
    (*a).format_data = shar as *mut libc::c_void;
    (*a).format_name = b"shar\x00" as *const u8 as *const libc::c_char;
    (*a).format_write_header = Some(
        archive_write_shar_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_close = Some(
        archive_write_shar_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_free =
        Some(archive_write_shar_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_write_data = Some(
        archive_write_shar_data_sed
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry = Some(
        archive_write_shar_finish_entry
            as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).archive.archive_format = ARCHIVE_FORMAT_SHAR_BASE;
    (*a).archive.archive_format_name = b"shar\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
/*
 * An alternate 'shar' that uses uudecode instead of 'sed' to encode
 * file contents and can therefore be used to archive binary files.
 * In addition, this variant also attempts to restore ownership, file modes,
 * and other extended file information.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_shar_dump(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut shar: *mut shar = 0 as *mut shar;
    archive_write_set_format_shar(&mut (*a).archive);
    shar = (*a).format_data as *mut shar;
    (*shar).dump = 1 as libc::c_int;
    (*a).format_write_data = Some(
        archive_write_shar_data_uuencode
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).archive.archive_format = ARCHIVE_FORMAT_SHAR_DUMP;
    (*a).archive.archive_format_name = b"shar dump\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_shar_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut linkname: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shar: *mut shar = 0 as *mut shar;
    shar = (*a).format_data as *mut shar;
    if (*shar).wrote_header == 0 {
        archive_strcat(
            &mut (*shar).work,
            b"#!/bin/sh\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        archive_strcat(
            &mut (*shar).work,
            b"# This is a shell archive\n\x00" as *const u8 as *const libc::c_char
                as *const libc::c_void,
        );
        (*shar).wrote_header = 1 as libc::c_int
    }
    /* Save the entry for the closing. */
    archive_entry_free((*shar).entry);
    (*shar).entry = archive_entry_clone(entry);
    name = archive_entry_pathname(entry);
    /* Handle some preparatory issues. */
    match archive_entry_filetype(entry) {
        32768 => {}
        16384 => {
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            /* Don't bother trying to recreate '.' */
            if strcmp(name, b".\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(name, b"./\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        4096 | 8192 | 24576 => {
            /* All other file types have zero size in the archive. */
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
        }
        _ => {
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            if archive_entry_hardlink(entry).is_null() && archive_entry_symlink(entry).is_null() {
                __archive_write_entry_filetype_unsupported(
                    &mut (*a).archive,
                    entry,
                    b"shar\x00" as *const u8 as *const libc::c_char,
                );
                return -(20 as libc::c_int);
            }
        }
    }
    (*shar).quoted_name.length = 0 as libc::c_int as size_t;
    shar_quote(&mut (*shar).quoted_name, name, 1 as libc::c_int);
    /* Stock preparation for all file types. */
    archive_string_sprintf(
        &mut (*shar).work as *mut archive_string,
        b"echo x %s\n\x00" as *const u8 as *const libc::c_char,
        (*shar).quoted_name.s,
    );
    if archive_entry_filetype(entry) != AE_IFDIR as mode_t {
        /* Try to create the dir. */
        p = strdup(name);
        pp = strrchr(p, '/' as i32);
        /* If there is a / character, try to create the dir. */
        if !pp.is_null() {
            *pp = '\u{0}' as i32 as libc::c_char;
            /* Try to avoid a lot of redundant mkdir commands. */
            if strcmp(p, b".\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                /* Don't try to "mkdir ." */
                free(p as *mut libc::c_void);
            } else if (*shar).last_dir.is_null() {
                archive_strcat(
                    &mut (*shar).work,
                    b"mkdir -p \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
                shar_quote(&mut (*shar).work, p, 1 as libc::c_int);
                archive_strcat(
                    &mut (*shar).work,
                    b" > /dev/null 2>&1\n\x00" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                );
                (*shar).last_dir = p
            } else if strcmp(p, (*shar).last_dir) == 0 as libc::c_int {
                /* We've already created this exact dir. */
                free(p as *mut libc::c_void);
            } else if strlen(p) < strlen((*shar).last_dir)
                && strncmp(p, (*shar).last_dir, strlen(p)) == 0 as libc::c_int
            {
                /* We've already created a subdir. */
                free(p as *mut libc::c_void);
            } else {
                archive_strcat(
                    &mut (*shar).work,
                    b"mkdir -p \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
                shar_quote(&mut (*shar).work, p, 1 as libc::c_int);
                archive_strcat(
                    &mut (*shar).work,
                    b" > /dev/null 2>&1\n\x00" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                );
                (*shar).last_dir = p
            }
        } else {
            free(p as *mut libc::c_void);
        }
    }
    /* Handle file-type specific issues. */
    (*shar).has_data = 0 as libc::c_int;
    linkname = archive_entry_hardlink(entry);
    if !linkname.is_null() {
        archive_strcat(
            &mut (*shar).work,
            b"ln -f \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        shar_quote(&mut (*shar).work, linkname, 1 as libc::c_int);
        archive_string_sprintf(
            &mut (*shar).work as *mut archive_string,
            b" %s\n\x00" as *const u8 as *const libc::c_char,
            (*shar).quoted_name.s,
        );
    } else {
        linkname = archive_entry_symlink(entry);
        if !linkname.is_null() {
            archive_strcat(
                &mut (*shar).work,
                b"ln -fs \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            shar_quote(&mut (*shar).work, linkname, 1 as libc::c_int);
            archive_string_sprintf(
                &mut (*shar).work as *mut archive_string,
                b" %s\n\x00" as *const u8 as *const libc::c_char,
                (*shar).quoted_name.s,
            );
        } else {
            match archive_entry_filetype(entry) {
                32768 => {
                    if archive_entry_size(entry) == 0 as libc::c_int as libc::c_long {
                        /* More portable than "touch." */
                        archive_string_sprintf(
                            &mut (*shar).work as *mut archive_string,
                            b"test -e \"%s\" || :> \"%s\"\n\x00" as *const u8
                                as *const libc::c_char,
                            (*shar).quoted_name.s,
                            (*shar).quoted_name.s,
                        );
                    } else {
                        if (*shar).dump != 0 {
                            let mut mode: libc::c_uint =
                                archive_entry_mode(entry) & 0o777 as libc::c_int as libc::c_uint;
                            archive_string_sprintf(
                                &mut (*shar).work as *mut archive_string,
                                b"uudecode -p > %s << \'SHAR_END\'\n\x00" as *const u8
                                    as *const libc::c_char,
                                (*shar).quoted_name.s,
                            );
                            archive_string_sprintf(
                                &mut (*shar).work as *mut archive_string,
                                b"begin %o \x00" as *const u8 as *const libc::c_char,
                                mode,
                            );
                            shar_quote(&mut (*shar).work, name, 0 as libc::c_int);
                            archive_strcat(
                                &mut (*shar).work,
                                b"\n\x00" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                            );
                        } else {
                            archive_string_sprintf(
                                &mut (*shar).work as *mut archive_string,
                                b"sed \'s/^X//\' > %s << \'SHAR_END\'\n\x00" as *const u8
                                    as *const libc::c_char,
                                (*shar).quoted_name.s,
                            );
                        }
                        (*shar).has_data = 1 as libc::c_int;
                        (*shar).end_of_line = 1 as libc::c_int;
                        (*shar).outpos = 0 as libc::c_int as size_t
                    }
                }
                16384 => {
                    archive_string_sprintf(
                        &mut (*shar).work as *mut archive_string,
                        b"mkdir -p %s > /dev/null 2>&1\n\x00" as *const u8 as *const libc::c_char,
                        (*shar).quoted_name.s,
                    );
                    /* Record that we just created this directory. */
                    free((*shar).last_dir as *mut libc::c_void);
                    (*shar).last_dir = strdup(name);
                    /* Trim a trailing '/'. */
                    pp = strrchr((*shar).last_dir, '/' as i32);
                    if !pp.is_null()
                        && *pp.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                    {
                        *pp = '\u{0}' as i32 as libc::c_char
                    }
                }
                4096 => {
                    archive_string_sprintf(
                        &mut (*shar).work as *mut archive_string,
                        b"mkfifo %s\n\x00" as *const u8 as *const libc::c_char,
                        (*shar).quoted_name.s,
                    );
                }
                8192 => {
                    archive_string_sprintf(
                        &mut (*shar).work as *mut archive_string,
                        b"mknod %s c %ju %ju\n\x00" as *const u8 as *const libc::c_char,
                        (*shar).quoted_name.s,
                        archive_entry_rdevmajor(entry),
                        archive_entry_rdevminor(entry),
                    );
                }
                24576 => {
                    archive_string_sprintf(
                        &mut (*shar).work as *mut archive_string,
                        b"mknod %s b %ju %ju\n\x00" as *const u8 as *const libc::c_char,
                        (*shar).quoted_name.s,
                        archive_entry_rdevmajor(entry),
                        archive_entry_rdevminor(entry),
                    );
                }
                _ => return -(20 as libc::c_int),
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_shar_data_sed(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut n: size_t,
) -> ssize_t {
    static mut ensured: size_t = 65533 as libc::c_int as size_t;
    let mut shar: *mut shar = 0 as *mut shar;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut written: size_t = n;
    shar = (*a).format_data as *mut shar;
    if (*shar).has_data == 0 || n == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    src = buff as *const libc::c_char;
    /*
     * ensure is the number of bytes in buffer before expanding the
     * current character.  Each operation writes the current character
     * and optionally the start-of-new-line marker.  This can happen
     * twice before entering the loop, so make sure three additional
     * bytes can be written.
     */
    if archive_string_ensure(
        &mut (*shar).work,
        ensured.wrapping_add(3 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int) as ssize_t;
    }
    if (*shar).work.length > ensured {
        ret = __archive_write_output(
            a,
            (*shar).work.s as *const libc::c_void,
            (*shar).work.length,
        );
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        (*shar).work.length = 0 as libc::c_int as size_t
    }
    buf = (*shar).work.s.offset((*shar).work.length as isize);
    buf_end = (*shar).work.s.offset(ensured as isize);
    if (*shar).end_of_line != 0 {
        let fresh0 = buf;
        buf = buf.offset(1);
        *fresh0 = 'X' as i32 as libc::c_char;
        (*shar).end_of_line = 0 as libc::c_int
    }
    loop {
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if !(fresh1 != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let fresh2 = src;
        src = src.offset(1);
        let fresh3 = buf;
        buf = buf.offset(1);
        *fresh3 = *fresh2;
        if *fresh3 as libc::c_int == '\n' as i32 {
            if n == 0 as libc::c_int as libc::c_ulong {
                (*shar).end_of_line = 1 as libc::c_int
            } else {
                let fresh4 = buf;
                buf = buf.offset(1);
                *fresh4 = 'X' as i32 as libc::c_char
            }
        }
        if buf >= buf_end {
            (*shar).work.length =
                buf.offset_from((*shar).work.s) as libc::c_long as size_t;
            ret = __archive_write_output(
                a,
                (*shar).work.s as *const libc::c_void,
                (*shar).work.length,
            );
            if ret != ARCHIVE_OK {
                return -(30 as libc::c_int) as ssize_t;
            }
            (*shar).work.length = 0 as libc::c_int as size_t;
            buf = (*shar).work.s
        }
    }
    (*shar).work.length = buf.offset_from((*shar).work.s) as libc::c_long as size_t;
    return written as ssize_t;
}
unsafe extern "C" fn uuencode_group(mut _in: *const libc::c_char, mut out: *mut libc::c_char) {
    let mut in_0: *const libc::c_uchar = _in as *const libc::c_uchar;
    let mut t: libc::c_int = 0;
    t = (*in_0.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
        | (*in_0.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *in_0.offset(2 as libc::c_int as isize) as libc::c_int;
    *out.offset(0 as libc::c_int as isize) =
        if 0x3f as libc::c_int & t >> 18 as libc::c_int != 0 as libc::c_int {
            (0x3f as libc::c_int & t >> 18 as libc::c_int & 0o77 as libc::c_int) + ' ' as i32
        } else {
            '`' as i32
        } as libc::c_char;
    *out.offset(1 as libc::c_int as isize) =
        if 0x3f as libc::c_int & t >> 12 as libc::c_int != 0 as libc::c_int {
            (0x3f as libc::c_int & t >> 12 as libc::c_int & 0o77 as libc::c_int) + ' ' as i32
        } else {
            '`' as i32
        } as libc::c_char;
    *out.offset(2 as libc::c_int as isize) =
        if 0x3f as libc::c_int & t >> 6 as libc::c_int != 0 as libc::c_int {
            (0x3f as libc::c_int & t >> 6 as libc::c_int & 0o77 as libc::c_int) + ' ' as i32
        } else {
            '`' as i32
        } as libc::c_char;
    *out.offset(3 as libc::c_int as isize) = if 0x3f as libc::c_int & t != 0 as libc::c_int {
        (0x3f as libc::c_int & t & 0o77 as libc::c_int) + ' ' as i32
    } else {
        '`' as i32
    } as libc::c_char;
}
unsafe extern "C" fn _uuencode_line(
    mut a: *mut archive_write,
    mut shar: *mut shar,
    mut inbuf: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alloc_len: size_t = 0;
    /* len <= 45 -> expanded to 60 + len byte + new line */
    alloc_len = (*shar)
        .work
        .length
        .wrapping_add(62 as libc::c_int as libc::c_ulong);
    if archive_string_ensure(&mut (*shar).work, alloc_len).is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    buf = (*shar).work.s.offset((*shar).work.length as isize);
    let fresh5 = buf;
    buf = buf.offset(1);
    *fresh5 = if len != 0 as libc::c_int as libc::c_ulong {
        (len & 0o77 as libc::c_int as libc::c_ulong).wrapping_add(' ' as i32 as libc::c_ulong)
    } else {
        '`' as i32 as libc::c_ulong
    } as libc::c_char;
    while len >= 3 as libc::c_int as libc::c_ulong {
        uuencode_group(inbuf, buf);
        len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        inbuf = inbuf.offset(3 as libc::c_int as isize);
        buf = buf.offset(4 as libc::c_int as isize)
    }
    if len != 0 as libc::c_int as libc::c_ulong {
        let mut tmp_buf: [libc::c_char; 3] = [0; 3];
        tmp_buf[0 as libc::c_int as usize] = *inbuf.offset(0 as libc::c_int as isize);
        if len == 1 as libc::c_int as libc::c_ulong {
            tmp_buf[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
        } else {
            tmp_buf[1 as libc::c_int as usize] = *inbuf.offset(1 as libc::c_int as isize)
        }
        tmp_buf[2 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        uuencode_group(tmp_buf.as_mut_ptr() as *const libc::c_char, buf);
        buf = buf.offset(4 as libc::c_int as isize)
    }
    let fresh6 = buf;
    buf = buf.offset(1);
    *fresh6 = '\n' as i32 as libc::c_char;
    if buf.offset_from((*shar).work.s) as libc::c_long
        > (*shar)
            .work
            .length
            .wrapping_add(62 as libc::c_int as libc::c_ulong) as ptrdiff_t
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Buffer overflow\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*shar).work.length = buf.offset_from((*shar).work.s) as libc::c_long as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_shar_data_uuencode(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> ssize_t {
    let mut shar: *mut shar = 0 as *mut shar;
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0;
    let mut ret: libc::c_int = 0;
    shar = (*a).format_data as *mut shar;
    if (*shar).has_data == 0 {
        return 0 as libc::c_int as ssize_t;
    }
    src = buff as *const libc::c_char;
    if (*shar).outpos != 0 as libc::c_int as libc::c_ulong {
        n = (45 as libc::c_int as libc::c_ulong).wrapping_sub((*shar).outpos);
        if n > length {
            n = length
        }
        memcpy(
            (*shar).outbuff.as_mut_ptr().offset((*shar).outpos as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            n,
        );
        if (*shar).outpos.wrapping_add(n) < 45 as libc::c_int as libc::c_ulong {
            (*shar).outpos = ((*shar).outpos as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            return length as ssize_t;
        }
        let mut r: libc::c_int = _uuencode_line(
            a,
            shar,
            (*shar).outbuff.as_mut_ptr(),
            45 as libc::c_int as size_t,
        );
        if r != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        src = src.offset(n as isize);
        n = length.wrapping_sub(n)
    } else {
        n = length
    }
    while n >= 45 as libc::c_int as libc::c_ulong {
        let mut r_0: libc::c_int = _uuencode_line(a, shar, src, 45 as libc::c_int as size_t);
        if r_0 != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        src = src.offset(45 as libc::c_int as isize);
        n = (n as libc::c_ulong).wrapping_sub(45 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        if (*shar).work.length < 65536 as libc::c_int as libc::c_ulong {
            continue;
        }
        ret = __archive_write_output(
            a,
            (*shar).work.s as *const libc::c_void,
            (*shar).work.length,
        );
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        (*shar).work.length = 0 as libc::c_int as size_t
    }
    if n != 0 as libc::c_int as libc::c_ulong {
        memcpy(
            (*shar).outbuff.as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            n,
        );
        (*shar).outpos = n
    }
    return length as ssize_t;
}
unsafe extern "C" fn archive_write_shar_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut g: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut u: *const libc::c_char = 0 as *const libc::c_char;
    let mut shar: *mut shar = 0 as *mut shar;
    let mut ret: libc::c_int = 0;
    shar = (*a).format_data as *mut shar;
    if (*shar).entry.is_null() {
        return 0 as libc::c_int;
    }
    if (*shar).dump != 0 {
        /* Finish uuencoded data. */
        if (*shar).has_data != 0 {
            if (*shar).outpos > 0 as libc::c_int as libc::c_ulong {
                let mut r: libc::c_int =
                    _uuencode_line(a, shar, (*shar).outbuff.as_mut_ptr(), (*shar).outpos);
                if r != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
            }
            archive_strcat(
                &mut (*shar).work,
                b"`\nend\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            archive_strcat(
                &mut (*shar).work,
                b"SHAR_END\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
        }
        /* TODO: restore ACLs */
        archive_string_sprintf(
            &mut (*shar).work as *mut archive_string,
            b"chmod %o \x00" as *const u8 as *const libc::c_char,
            archive_entry_mode((*shar).entry) & 0o7777 as libc::c_int as libc::c_uint,
        );
        shar_quote(
            &mut (*shar).work,
            archive_entry_pathname((*shar).entry),
            1 as libc::c_int,
        );
        archive_strcat(
            &mut (*shar).work,
            b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        u = archive_entry_uname((*shar).entry);
        g = archive_entry_gname((*shar).entry);
        if !u.is_null() || !g.is_null() {
            archive_strcat(
                &mut (*shar).work,
                b"chown \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            if !u.is_null() {
                shar_quote(&mut (*shar).work, u, 1 as libc::c_int);
            }
            if !g.is_null() {
                archive_strcat(
                    &mut (*shar).work,
                    b":\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
                shar_quote(&mut (*shar).work, g, 1 as libc::c_int);
            }
            archive_strcat(
                &mut (*shar).work,
                b" \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            shar_quote(
                &mut (*shar).work,
                archive_entry_pathname((*shar).entry),
                1 as libc::c_int,
            );
            archive_strcat(
                &mut (*shar).work,
                b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
        }
        p = archive_entry_fflags_text((*shar).entry);
        if !p.is_null() {
            archive_string_sprintf(
                &mut (*shar).work as *mut archive_string,
                b"chflags %s \x00" as *const u8 as *const libc::c_char,
                p,
            );
            shar_quote(
                &mut (*shar).work,
                archive_entry_pathname((*shar).entry),
                1 as libc::c_int,
            );
            archive_strcat(
                &mut (*shar).work,
                b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
        }
    } else if (*shar).has_data != 0 {
        /* Restore file mode, owner, flags. */
        /*
         * TODO: Don't immediately restore mode for
         * directories; defer that to end of script.
         */
        /* Finish sed-encoded data:  ensure last line ends. */
        if (*shar).end_of_line == 0 {
            archive_strappend_char(&mut (*shar).work, '\n' as i32 as libc::c_char);
        }
        archive_strcat(
            &mut (*shar).work,
            b"SHAR_END\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
    }
    archive_entry_free((*shar).entry);
    (*shar).entry = NULL as *mut archive_entry;
    if (*shar).work.length < 65536 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    ret = __archive_write_output(
        a,
        (*shar).work.s as *const libc::c_void,
        (*shar).work.length,
    );
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*shar).work.length = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_shar_close(mut a: *mut archive_write) -> libc::c_int {
    let mut shar: *mut shar = 0 as *mut shar;
    let mut ret: libc::c_int = 0;
    /*
     * TODO: Accumulate list of directory names/modes and
     * fix them all up at end-of-archive.
     */
    shar = (*a).format_data as *mut shar;
    /*
     * Only write the end-of-archive markers if the archive was
     * actually started.  This avoids problems if someone sets
     * shar format, then sets another format (which would invoke
     * shar_finish to free the format-specific data).
     */
    if (*shar).wrote_header == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    archive_strcat(
        &mut (*shar).work,
        b"exit\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    );
    ret = __archive_write_output(
        a,
        (*shar).work.s as *const libc::c_void,
        (*shar).work.length,
    );
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Shar output is never padded. */
    archive_write_set_bytes_in_last_block(&mut (*a).archive, 1 as libc::c_int);
    /*
     * TODO: shar should also suppress padding of
     * uncompressed data within gzip/bzip2 streams.
     */
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_shar_free(mut a: *mut archive_write) -> libc::c_int {
    let mut shar: *mut shar = 0 as *mut shar;
    shar = (*a).format_data as *mut shar;
    if shar.is_null() {
        return 0 as libc::c_int;
    }
    archive_entry_free((*shar).entry);
    free((*shar).last_dir as *mut libc::c_void);
    archive_string_free(&mut (*shar).work);
    archive_string_free(&mut (*shar).quoted_name);
    free(shar as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
