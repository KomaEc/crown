use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn archive_write_set_bytes_in_last_block(
        _: *mut archive,
        bytes_in_last_block: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_write_get_bytes_in_last_block(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_skip_file(_: *mut archive, _: la_int64_t, _: la_int64_t) -> libc::c_int;
    #[no_mangle]
    fn archive_write_open(
        _: *mut archive,
        _: *mut libc::c_void,
        _: Option<archive_open_callback>,
        _: Option<archive_write_callback>,
        _: Option<archive_close_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_write_open_fd(_: *mut archive, _fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_mstring_clean(_: *mut archive_mstring);
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
    fn __archive_ensure_cloexec_flag(fd: libc::c_int);
    #[no_mangle]
    fn archive_mstring_get_wcs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const wchar_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_mbs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_wcs(_: *mut archive_mstring, wcs: *const wchar_t) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_mbs(_: *mut archive_mstring, mbs: *const libc::c_char) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type ssize_t = __ssize_t;
pub type la_int64_t = int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct write_file_data {
    pub fd: libc::c_int,
    pub filename: archive_mstring,
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
/* Append C string, which may lack trailing \0. */
/* The source is declared void * here because this gets used with
 * "signed char *", "unsigned char *" and "char *" arguments.
 * Declaring it "char *" as with some of the other functions just
 * leads to a lot of extra casts. */
/* Append a C string to an archive_string, resizing as necessary. */
/* Copy a C string to an archive_string, resizing as necessary. */
/* Copy a C string to an archive_string with limit, resizing as necessary. */
/* Return length of string. */
/* Set string length to zero. */
/* Release any allocated storage resources. */
/* Like 'vsprintf', but resizes the underlying string as necessary. */
/* Note: This only implements a small subset of standard printf functionality. */
/* Translates from MBS to Unicode. */
/* Returns non-zero if conversion failed in any way. */
/* A "multistring" can hold Unicode, UTF8, or MBS versions of
 * the string.  If you set and read the same version, no translation
 * is done.  If you set and read different versions, the library
 * will attempt to transparently convert.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_mstring {
    pub aes_mbs: archive_string,
    pub aes_utf8: archive_string,
    pub aes_wcs: archive_wstring,
    pub aes_mbs_in_locale: archive_string,
    pub aes_set: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_wstring {
    pub s: *mut wchar_t,
    pub length: size_t,
    pub buffer_length: size_t,
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
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const O_WRONLY: libc::c_int = 0o1 as libc::c_int;
pub const O_CREAT: libc::c_int = 0o100 as libc::c_int;
pub const O_TRUNC: libc::c_int = 0o1000 as libc::c_int;
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const O_BINARY: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_write_open_file(
    mut a: *mut archive,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    return archive_write_open_filename(a, filename);
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_open_filename(
    mut a: *mut archive,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    if filename.is_null()
        || *filename.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        return archive_write_open_fd(a, 1 as libc::c_int);
    }
    return open_filename(a, 1 as libc::c_int, filename as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_open_filename_w(
    mut a: *mut archive,
    mut filename: *const wchar_t,
) -> libc::c_int {
    if filename.is_null() || *filename.offset(0 as libc::c_int as isize) == '\u{0}' as i32 {
        return archive_write_open_fd(a, 1 as libc::c_int);
    }
    return open_filename(a, 0 as libc::c_int, filename as *const libc::c_void);
}
unsafe extern "C" fn open_filename(
    mut a: *mut archive,
    mut mbs_fn: libc::c_int,
    mut filename: *const libc::c_void,
) -> libc::c_int {
    let mut mine: *mut write_file_data = 0 as *mut write_file_data;
    let mut r: libc::c_int = 0;
    mine = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<write_file_data>() as libc::c_ulong,
    ) as *mut write_file_data;
    if mine.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if mbs_fn != 0 {
        r = archive_mstring_copy_mbs(&mut (*mine).filename, filename as *const libc::c_char)
    } else {
        r = archive_mstring_copy_wcs(&mut (*mine).filename, filename as *const wchar_t)
    }
    if r < 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                a,
                ENOMEM,
                b"No memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if mbs_fn != 0 {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_MISC,
                b"Can\'t convert \'%s\' to WCS\x00" as *const u8 as *const libc::c_char,
                filename as *const libc::c_char,
            );
        } else {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_MISC,
                b"Can\'t convert \'%S\' to MBS\x00" as *const u8 as *const libc::c_char,
                filename as *const wchar_t,
            );
        }
        return -(25 as libc::c_int);
    }
    (*mine).fd = -(1 as libc::c_int);
    return archive_write_open(
        a,
        mine as *mut libc::c_void,
        Some(
            file_open as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
        Some(
            file_write
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> ssize_t,
        ),
        Some(
            file_close
                as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn file_open(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    let mut mine: *mut write_file_data = 0 as *mut write_file_data;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut wcs: *const wchar_t = 0 as *const wchar_t;
    let mut mbs: *const libc::c_char = 0 as *const libc::c_char;
    mine = client_data as *mut write_file_data;
    flags = O_WRONLY | O_CREAT | O_TRUNC | O_BINARY | O_CLOEXEC;
    /*
     * Open the file.
     */
    mbs = NULL as *const libc::c_char;
    wcs = NULL as *const wchar_t;
    if archive_mstring_get_mbs(a, &mut (*mine).filename, &mut mbs) != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                a,
                errno,
                b"No memory\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            archive_mstring_get_wcs(a, &mut (*mine).filename, &mut wcs);
            archive_set_error(
                a,
                errno,
                b"Can\'t convert \'%S\' to MBS\x00" as *const u8 as *const libc::c_char,
                wcs,
            );
        }
        return -(30 as libc::c_int);
    }
    (*mine).fd = open(mbs, flags, 0o666 as libc::c_int);
    __archive_ensure_cloexec_flag((*mine).fd);
    if (*mine).fd < 0 as libc::c_int {
        if !mbs.is_null() {
            archive_set_error(
                a,
                errno,
                b"Failed to open \'%s\'\x00" as *const u8 as *const libc::c_char,
                mbs,
            );
        } else {
            archive_set_error(
                a,
                errno,
                b"Failed to open \'%S\'\x00" as *const u8 as *const libc::c_char,
                wcs,
            );
        }
        return -(30 as libc::c_int);
    }
    if fstat((*mine).fd, &mut st) != 0 as libc::c_int {
        if !mbs.is_null() {
            archive_set_error(
                a,
                errno,
                b"Couldn\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
                mbs,
            );
        } else {
            archive_set_error(
                a,
                errno,
                b"Couldn\'t stat \'%S\'\x00" as *const u8 as *const libc::c_char,
                wcs,
            );
        }
        return -(30 as libc::c_int);
    }
    /*
     * Set up default last block handling.
     */
    if archive_write_get_bytes_in_last_block(a) < 0 as libc::c_int {
        if st.st_mode & __S_IFMT as libc::c_uint == 0o20000 as libc::c_int as libc::c_uint
            || st.st_mode & __S_IFMT as libc::c_uint == 0o60000 as libc::c_int as libc::c_uint
            || st.st_mode & __S_IFMT as libc::c_uint == 0o10000 as libc::c_int as libc::c_uint
        {
            /* Pad last block when writing to device or FIFO. */
            archive_write_set_bytes_in_last_block(a, 0 as libc::c_int);
        } else {
            /* Don't pad last block otherwise. */
            archive_write_set_bytes_in_last_block(a, 1 as libc::c_int);
        }
    }
    /*
     * If the output file is a regular file, don't add it to
     * itself.  If it's a device file, it's okay to add the device
     * entry to the output archive.
     */
    if st.st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint {
        archive_write_set_skip_file(a, st.st_dev as la_int64_t, st.st_ino as la_int64_t);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_write(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> ssize_t {
    let mut mine: *mut write_file_data = 0 as *mut write_file_data;
    let mut bytesWritten: ssize_t = 0;
    mine = client_data as *mut write_file_data;
    loop {
        bytesWritten = write((*mine).fd, buff, length);
        if bytesWritten <= 0 as libc::c_int as libc::c_long {
            if errno == EINTR {
                continue;
            }
            archive_set_error(
                a,
                errno,
                b"Write error\x00" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int) as ssize_t;
        } else {
            return bytesWritten;
        }
    }
}
unsafe extern "C" fn file_close(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut mine: *mut write_file_data = client_data as *mut write_file_data;
    /* UNUSED */
    if (*mine).fd >= 0 as libc::c_int {
        close((*mine).fd);
    }
    archive_mstring_clean(&mut (*mine).filename);
    free(mine as *mut libc::c_void);
    return 0 as libc::c_int;
}
