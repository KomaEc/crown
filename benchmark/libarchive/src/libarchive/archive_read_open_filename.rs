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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn archive_read_set_open_callback(
        _: *mut archive,
        _: Option<archive_open_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_read_callback(
        _: *mut archive,
        _: Option<archive_read_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_seek_callback(
        _: *mut archive,
        _: Option<archive_seek_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_skip_callback(
        _: *mut archive,
        _: Option<archive_skip_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_close_callback(
        _: *mut archive,
        _: Option<archive_close_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_switch_callback(
        _: *mut archive,
        _: Option<archive_switch_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_append_callback_data(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn archive_read_open1(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_read_extract_set_skip_file(_: *mut archive, _: la_int64_t, _: la_int64_t);
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn __archive_ensure_cloexec_flag(fd: libc::c_int);
    #[no_mangle]
    fn archive_string_append_from_wcs(
        _: *mut archive_string,
        _: *const wchar_t,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
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
pub type mode_t = __mode_t;
pub type off_t = __off_t;
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
pub type archive_read_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut *const libc::c_void,
) -> la_ssize_t;
pub type archive_skip_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void, _: la_int64_t) -> la_int64_t;
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
pub type archive_switch_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub m: [libc::c_char; 1],
    pub w: [wchar_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_file_data {
    pub fd: libc::c_int,
    pub block_size: size_t,
    pub buffer: *mut libc::c_void,
    pub st_mode: mode_t,
    pub use_lseek: libc::c_char,
    pub filename_type: fnt_e,
    pub filename: C2RustUnnamed,
}
pub type fnt_e = libc::c_uint;
pub const FNT_WCS: fnt_e = 2;
pub const FNT_MBS: fnt_e = 1;
pub const FNT_STDIN: fnt_e = 0;
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
pub const errno: libc::c_int = *__errno_location();
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const ESPIPE: libc::c_int = 29 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const O_BINARY: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_file(
    mut a: *mut archive,
    mut filename: *const libc::c_char,
    mut block_size: size_t,
) -> libc::c_int {
    return archive_read_open_filename(a, filename, block_size);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_filename(
    mut a: *mut archive,
    mut filename: *const libc::c_char,
    mut block_size: size_t,
) -> libc::c_int {
    let mut filenames: [*const libc::c_char; 2] = [0 as *const libc::c_char; 2];
    filenames[0 as libc::c_int as usize] = filename;
    filenames[1 as libc::c_int as usize] = NULL as *const libc::c_char;
    return archive_read_open_filenames(a, filenames.as_mut_ptr(), block_size);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_filenames(
    mut a: *mut archive,
    mut filenames: *mut *const libc::c_char,
    mut block_size: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut mine: *mut read_file_data = 0 as *mut read_file_data;
    let mut filename: *const libc::c_char = NULL as *const libc::c_char;
    if !filenames.is_null() {
        let fresh0 = filenames;
        filenames = filenames.offset(1);
        filename = *fresh0
    }
    archive_clear_error(a);
    loop {
        if filename.is_null() {
            filename = b"\x00" as *const u8 as *const libc::c_char
        }
        mine = calloc(
            1 as libc::c_int as libc::c_ulong,
            (::std::mem::size_of::<read_file_data>() as libc::c_ulong)
                .wrapping_add(strlen(filename)),
        ) as *mut read_file_data;
        if mine.is_null() {
            current_block = 3748563103244195371;
            break;
        }
        strcpy((*mine).filename.m.as_mut_ptr(), filename);
        (*mine).block_size = block_size;
        (*mine).fd = -(1 as libc::c_int);
        (*mine).buffer = NULL as *mut libc::c_void;
        (*mine).use_lseek = 0 as libc::c_int as libc::c_char;
        (*mine).st_mode = (*mine).use_lseek as mode_t;
        if filename.is_null()
            || *filename.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            (*mine).filename_type = FNT_STDIN
        } else {
            (*mine).filename_type = FNT_MBS
        }
        if archive_read_append_callback_data(a, mine as *mut libc::c_void) != 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
        if filenames.is_null() {
            current_block = 26972500619410423;
            break;
        }
        let fresh1 = filenames;
        filenames = filenames.offset(1);
        filename = *fresh1;
        if !(!filename.is_null()
            && *filename.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32)
        {
            current_block = 26972500619410423;
            break;
        }
    }
    match current_block {
        3748563103244195371 => {
            archive_set_error(
                a,
                ENOMEM,
                b"No memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        _ => {
            archive_read_set_open_callback(
                a,
                Some(
                    file_open
                        as unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            archive_read_set_read_callback(
                a,
                Some(
                    file_read
                        as unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                            _: *mut *const libc::c_void,
                        ) -> ssize_t,
                ),
            );
            archive_read_set_skip_callback(
                a,
                Some(
                    file_skip
                        as unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                            _: int64_t,
                        ) -> int64_t,
                ),
            );
            archive_read_set_close_callback(
                a,
                Some(
                    file_close
                        as unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            archive_read_set_switch_callback(
                a,
                Some(
                    file_switch
                        as unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            archive_read_set_seek_callback(
                a,
                Some(
                    file_seek
                        as unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                            _: int64_t,
                            _: libc::c_int,
                        ) -> int64_t,
                ),
            );
            return archive_read_open1(a);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_filename_w(
    mut a: *mut archive,
    mut wfilename: *const wchar_t,
    mut block_size: size_t,
) -> libc::c_int {
    let mut mine: *mut read_file_data = calloc(
        1 as libc::c_int as libc::c_ulong,
        (::std::mem::size_of::<read_file_data>() as libc::c_ulong).wrapping_add(
            wcslen(wfilename).wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
        ),
    ) as *mut read_file_data;
    if mine.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*mine).fd = -(1 as libc::c_int);
    (*mine).block_size = block_size;
    if wfilename.is_null() || *wfilename.offset(0 as libc::c_int as isize) == '\u{0}' as i32 {
        (*mine).filename_type = FNT_STDIN
    } else {
        /*
         * POSIX system does not support a wchar_t interface for
         * open() system call, so we have to translate a wchar_t
         * filename to multi-byte one and use it.
         */
        let mut fn_0: archive_string = archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        };
        fn_0.s = NULL as *mut libc::c_char;
        fn_0.length = 0 as libc::c_int as size_t;
        fn_0.buffer_length = 0 as libc::c_int as size_t;
        if archive_string_append_from_wcs(&mut fn_0, wfilename, wcslen(wfilename))
            != 0 as libc::c_int
        {
            if errno == ENOMEM {
                archive_set_error(
                    a,
                    errno,
                    b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                archive_set_error(
                    a,
                    EINVAL,
                    b"Failed to convert a wide-character filename to a multi-byte filename\x00"
                        as *const u8 as *const libc::c_char,
                );
            }
            archive_string_free(&mut fn_0);
            free(mine as *mut libc::c_void);
            return -(30 as libc::c_int);
        }
        (*mine).filename_type = FNT_MBS;
        strcpy((*mine).filename.m.as_mut_ptr(), fn_0.s);
        archive_string_free(&mut fn_0);
    }
    if archive_read_append_callback_data(a, mine as *mut libc::c_void) != 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    archive_read_set_open_callback(
        a,
        Some(
            file_open as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
    archive_read_set_read_callback(
        a,
        Some(
            file_read
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *mut *const libc::c_void,
                ) -> ssize_t,
        ),
    );
    archive_read_set_skip_callback(
        a,
        Some(
            file_skip
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: int64_t,
                ) -> int64_t,
        ),
    );
    archive_read_set_close_callback(
        a,
        Some(
            file_close
                as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
    archive_read_set_switch_callback(
        a,
        Some(
            file_switch
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    archive_read_set_seek_callback(
        a,
        Some(
            file_seek
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: int64_t,
                    _: libc::c_int,
                ) -> int64_t,
        ),
    );
    return archive_read_open1(a);
}
/* Must be last! */
unsafe extern "C" fn file_open(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
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
    let mut mine: *mut read_file_data = client_data as *mut read_file_data;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut filename: *const libc::c_char = NULL as *const libc::c_char;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut is_disk_like: libc::c_int = 0 as libc::c_int;
    archive_clear_error(a);
    if (*mine).filename_type as libc::c_uint == FNT_STDIN as libc::c_int as libc::c_uint {
        /* We used to delegate stdin support by
         * directly calling archive_read_open_fd(a,0,block_size)
         * here, but that doesn't (and shouldn't) handle the
         * end-of-file flush when reading stdout from a pipe.
         * Basically, read_open_fd() is intended for folks who
         * are willing to handle such details themselves.  This
         * API is intended to be a little smarter for folks who
         * want easy handling of the common case.
         */
        fd = 0 as libc::c_int;
        filename = b"\x00" as *const u8 as *const libc::c_char;
        current_block = 5143058163439228106;
    } else if (*mine).filename_type as libc::c_uint == FNT_MBS as libc::c_int as libc::c_uint {
        filename = (*mine).filename.m.as_mut_ptr();
        fd = open(filename, O_RDONLY | O_BINARY | O_CLOEXEC);
        __archive_ensure_cloexec_flag(fd);
        if fd < 0 as libc::c_int {
            archive_set_error(
                a,
                errno,
                b"Failed to open \'%s\'\x00" as *const u8 as *const libc::c_char,
                filename,
            );
            return -(30 as libc::c_int);
        }
        current_block = 5143058163439228106;
    } else {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Unexpedted operation in archive_read_open_filename\x00" as *const u8
                as *const libc::c_char,
        );
        current_block = 8596636336592935543;
    }
    match current_block {
        5143058163439228106 => {
            if fstat(fd, &mut st) != 0 as libc::c_int {
                archive_set_error(
                    a,
                    errno,
                    b"Can\'t stat \'%s\'\x00" as *const u8 as *const libc::c_char,
                    filename,
                );
            } else {
                /*
                 * Determine whether the input looks like a disk device or a
                 * tape device.  The results are used below to select an I/O
                 * strategy:
                 *  = "disk-like" devices support arbitrary lseek() and will
                 *    support I/O requests of any size.  So we get easy skipping
                 *    and can cheat on block sizes to get better performance.
                 *  = "tape-like" devices require strict blocking and use
                 *    specialized ioctls for seeking.
                 *  = "socket-like" devices cannot seek at all but can improve
                 *    performance by using nonblocking I/O to read "whatever is
                 *    available right now".
                 *
                 * Right now, we only specially recognize disk-like devices,
                 * but it should be straightforward to add probes and strategy
                 * here for tape-like and socket-like devices.
                 */
                if st.st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint
                {
                    /* Safety:  Tell the extractor not to overwrite the input. */
                    archive_read_extract_set_skip_file(
                        a,
                        st.st_dev as la_int64_t,
                        st.st_ino as la_int64_t,
                    );
                    /* Regular files act like disks. */
                    is_disk_like = 1 as libc::c_int
                } else if st.st_mode & __S_IFMT as libc::c_uint
                    == 0o60000 as libc::c_int as libc::c_uint
                    && lseek(fd, 0 as libc::c_int as __off_t, SEEK_CUR)
                        == 0 as libc::c_int as libc::c_long
                    && lseek(fd, 0 as libc::c_int as __off_t, SEEK_SET)
                        == 0 as libc::c_int as libc::c_long
                    && lseek(fd, 0 as libc::c_int as __off_t, SEEK_END)
                        > 0 as libc::c_int as libc::c_long
                    && lseek(fd, 0 as libc::c_int as __off_t, SEEK_SET)
                        == 0 as libc::c_int as libc::c_long
                {
                    is_disk_like = 1 as libc::c_int
                }
                /* Linux:  All block devices are disk-like. */
                /* TODO: Add an "is_tape_like" variable and appropriate tests. */
                /* Disk-like devices prefer power-of-two block sizes.  */
                /* Use provided block_size as a guide so users have some control. */
                if is_disk_like != 0 {
                    let mut new_block_size: size_t =
                        (64 as libc::c_int * 1024 as libc::c_int) as size_t;
                    while new_block_size < (*mine).block_size
                        && new_block_size
                            < (64 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                                as libc::c_ulong
                    {
                        new_block_size = (new_block_size as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            as size_t as size_t
                    }
                    (*mine).block_size = new_block_size
                }
                buffer = malloc((*mine).block_size);
                if buffer.is_null() {
                    archive_set_error(
                        a,
                        ENOMEM,
                        b"No memory\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    (*mine).buffer = buffer;
                    (*mine).fd = fd;
                    /* Remember mode so close can decide whether to flush. */
                    (*mine).st_mode = st.st_mode;
                    /* Disk-like inputs can use lseek(). */
                    if is_disk_like != 0 {
                        (*mine).use_lseek = 1 as libc::c_int as libc::c_char
                    }
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    /*
     * Don't close file descriptors not opened or ones pointing referring
     * to `FNT_STDIN`.
     */
    if fd != -(1 as libc::c_int) && fd != 0 as libc::c_int {
        close(fd);
    }
    return -(30 as libc::c_int);
}
unsafe extern "C" fn file_read(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut mine: *mut read_file_data = client_data as *mut read_file_data;
    let mut bytes_read: ssize_t = 0;
    /* TODO: If a recent lseek() operation has left us
     * mis-aligned, read and return a short block to try to get
     * us back in alignment. */
    /* TODO: Someday, try mmap() here; if that succeeds, give
     * the entire file to libarchive as a single block.  That
     * could be a lot faster than block-by-block manual I/O. */
    /* TODO: We might be able to improve performance on pipes and
     * sockets by setting non-blocking I/O and just accepting
     * whatever we get here instead of waiting for a full block
     * worth of data. */
    *buff = (*mine).buffer;
    loop {
        bytes_read = read((*mine).fd, (*mine).buffer, (*mine).block_size);
        if bytes_read < 0 as libc::c_int as libc::c_long {
            if errno == EINTR {
                continue;
            }
            if (*mine).filename_type as libc::c_uint == FNT_STDIN as libc::c_int as libc::c_uint {
                archive_set_error(
                    a,
                    errno,
                    b"Error reading stdin\x00" as *const u8 as *const libc::c_char,
                );
            } else if (*mine).filename_type as libc::c_uint
                == FNT_MBS as libc::c_int as libc::c_uint
            {
                archive_set_error(
                    a,
                    errno,
                    b"Error reading \'%s\'\x00" as *const u8 as *const libc::c_char,
                    (*mine).filename.m.as_mut_ptr(),
                );
            } else {
                archive_set_error(
                    a,
                    errno,
                    b"Error reading \'%S\'\x00" as *const u8 as *const libc::c_char,
                    (*mine).filename.w.as_mut_ptr(),
                );
            }
        }
        return bytes_read;
    }
}
/*
 * Regular files and disk-like block devices can use simple lseek
 * without needing to round the request to the block size.
 *
 * TODO: This can leave future reads mis-aligned.  Since we know the
 * offset here, we should store it and use it in file_read() above
 * to determine whether we should perform a short read to get back
 * into alignment.  Long series of mis-aligned reads can negatively
 * impact disk throughput.  (Of course, the performance impact should
 * be carefully tested; extra code complexity is only worthwhile if
 * it does provide measurable improvement.)
 *
 * TODO: Be lazy about the actual seek.  There are a few pathological
 * cases where libarchive makes a bunch of seek requests in a row
 * without any intervening reads.  This isn't a huge performance
 * problem, since the kernel handles seeks lazily already, but
 * it would be very slightly faster if we simply remembered the
 * seek request here and then actually performed the seek at the
 * top of the read callback above.
 */
unsafe extern "C" fn file_skip_lseek(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut request: int64_t,
) -> int64_t {
    let mut mine: *mut read_file_data = client_data as *mut read_file_data;
    let mut old_offset: off_t = 0;
    let mut new_offset: off_t = 0;
    /* We use off_t here because lseek() is declared that way. */
    /* TODO: Deal with case where off_t isn't 64 bits.
     * This shouldn't be a problem on Linux or other POSIX
     * systems, since the configuration logic for libarchive
     * tries to obtain a 64-bit off_t.
     */
    old_offset = lseek((*mine).fd, 0 as libc::c_int as __off_t, SEEK_CUR);
    if old_offset >= 0 as libc::c_int as libc::c_long && {
        new_offset = lseek((*mine).fd, request, SEEK_CUR);
        (new_offset) >= 0 as libc::c_int as libc::c_long
    } {
        return new_offset - old_offset;
    }
    /* If lseek() fails, don't bother trying again. */
    (*mine).use_lseek = 0 as libc::c_int as libc::c_char;
    /* Let libarchive recover with read+discard */
    if errno == ESPIPE {
        return 0 as libc::c_int as int64_t;
    }
    /* If the input is corrupted or truncated, fail. */
    if (*mine).filename_type as libc::c_uint == FNT_STDIN as libc::c_int as libc::c_uint {
        archive_set_error(
            a,
            errno,
            b"Error seeking in stdin\x00" as *const u8 as *const libc::c_char,
        );
    } else if (*mine).filename_type as libc::c_uint == FNT_MBS as libc::c_int as libc::c_uint {
        archive_set_error(
            a,
            errno,
            b"Error seeking in \'%s\'\x00" as *const u8 as *const libc::c_char,
            (*mine).filename.m.as_mut_ptr(),
        );
    } else {
        archive_set_error(
            a,
            errno,
            b"Error seeking in \'%S\'\x00" as *const u8 as *const libc::c_char,
            (*mine).filename.w.as_mut_ptr(),
        );
    }
    return -(1 as libc::c_int) as int64_t;
}
/*
 * TODO: Implement another file_skip_XXXX that uses MTIO ioctls to
 * accelerate operation on tape drives.
 */
unsafe extern "C" fn file_skip(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut request: int64_t,
) -> int64_t {
    let mut mine: *mut read_file_data = client_data as *mut read_file_data;
    /* Delegate skip requests. */
    if (*mine).use_lseek != 0 {
        return file_skip_lseek(a, client_data, request);
    }
    /* If we can't skip, return 0; libarchive will read+discard instead. */
    return 0 as libc::c_int as int64_t;
}
/*
 * TODO: Store the offset and use it in the read callback.
 */
unsafe extern "C" fn file_seek(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut request: int64_t,
    mut whence: libc::c_int,
) -> int64_t {
    let mut mine: *mut read_file_data = client_data as *mut read_file_data;
    let mut r: int64_t = 0;
    /* We use off_t here because lseek() is declared that way. */
    /* See above for notes about when off_t is less than 64 bits. */
    r = lseek((*mine).fd, request, whence);
    if r >= 0 as libc::c_int as libc::c_long {
        return r;
    }
    /* If the input is corrupted or truncated, fail. */
    if (*mine).filename_type as libc::c_uint == FNT_STDIN as libc::c_int as libc::c_uint {
        archive_set_error(
            a,
            errno,
            b"Error seeking in stdin\x00" as *const u8 as *const libc::c_char,
        );
    } else if (*mine).filename_type as libc::c_uint == FNT_MBS as libc::c_int as libc::c_uint {
        archive_set_error(
            a,
            errno,
            b"Error seeking in \'%s\'\x00" as *const u8 as *const libc::c_char,
            (*mine).filename.m.as_mut_ptr(),
        );
    } else {
        archive_set_error(
            a,
            errno,
            b"Error seeking in \'%S\'\x00" as *const u8 as *const libc::c_char,
            (*mine).filename.w.as_mut_ptr(),
        );
    }
    return -(30 as libc::c_int) as int64_t;
}
unsafe extern "C" fn file_close2(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut mine: *mut read_file_data = client_data as *mut read_file_data;
    /* UNUSED */
    /* Only flush and close if open succeeded. */
    if (*mine).fd >= 0 as libc::c_int {
        /*
         * Sometimes, we should flush the input before closing.
         *   Regular files: faster to just close without flush.
         *   Disk-like devices:  Ditto.
         *   Tapes: must not flush (user might need to
         *      read the "next" item on a non-rewind device).
         *   Pipes and sockets:  must flush (otherwise, the
         *      program feeding the pipe or socket may complain).
         * Here, I flush everything except for regular files and
         * device nodes.
         */
        if !((*mine).st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint)
            && !((*mine).st_mode & __S_IFMT as libc::c_uint
                == 0o20000 as libc::c_int as libc::c_uint)
            && !((*mine).st_mode & __S_IFMT as libc::c_uint
                == 0o60000 as libc::c_int as libc::c_uint)
        {
            let mut bytesRead: ssize_t = 0;
            loop {
                bytesRead = read((*mine).fd, (*mine).buffer, (*mine).block_size);
                if !(bytesRead > 0 as libc::c_int as libc::c_long) {
                    break;
                }
            }
        }
        /* If a named file was opened, then it needs to be closed. */
        if (*mine).filename_type as libc::c_uint != FNT_STDIN as libc::c_int as libc::c_uint {
            close((*mine).fd);
        }
    }
    free((*mine).buffer);
    (*mine).buffer = NULL as *mut libc::c_void;
    (*mine).fd = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_close(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut mine: *mut read_file_data = client_data as *mut read_file_data;
    file_close2(a, client_data);
    free(mine as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_switch(
    mut a: *mut archive,
    mut client_data1: *mut libc::c_void,
    mut client_data2: *mut libc::c_void,
) -> libc::c_int {
    file_close2(a, client_data1);
    return file_open(a, client_data2);
}
