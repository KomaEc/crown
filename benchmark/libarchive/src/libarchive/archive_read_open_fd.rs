use ::libc;
extern "C" {
    pub type archive;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
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
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
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
    fn archive_read_set_callback_data(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn archive_read_open1(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_read_extract_set_skip_file(_: *mut archive, _: la_int64_t, _: la_int64_t);
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
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
pub type ssize_t = __ssize_t;
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
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_fd_data {
    pub fd: libc::c_int,
    pub block_size: size_t,
    pub use_lseek: libc::c_char,
    pub buffer: *mut libc::c_void,
}
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const ESPIPE: libc::c_int = 29 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
/* cmd */
/* match */
/* archive_read_support_format_zip() enables both streamable and seekable
 * zip readers. */
/* Reads Zip archives as stream from beginning to end.  Doesn't
 * correctly handle SFX ZIP files or ZIP archives that have been modified
 * in-place. */
/* Reads starting from central directory; requires seekable input. */
/* Functions to manually set the format and filters to be used. This is
 * useful to bypass the bidding process when the format and filters to use
 * is known in advance.
 */
/* match */
/* Set various callbacks. */
/* Callback used to switch between one data object to the next */
/* This sets the first data object. */
/* This sets data object at specified index */
/* This adds a data object at the specified index. */
/* This appends a data object to the end of list */
/* This prepends a data object to the beginning of list */
/* Opening freezes the callbacks. */
/* Convenience wrappers around the above. */
/*
 * A variety of shortcuts that invoke archive_read_open() with
 * canned callbacks suitable for common situations.  The ones that
 * accept a block size handle tape blocking correctly.
 */
/* Use this if you know the filename.  Note: NULL indicates stdin. */
/* Use this for reading multivolume files by filenames.
 * NOTE: Must be NULL terminated. Sorting is NOT done. */
/* archive_read_open_file() is a deprecated synonym for ..._open_filename(). */
/* Read an archive that's stored in memory. */
/* A more involved version that is only used for internal testing. */
/* Read an archive that's already open, using the file descriptor. */
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_fd(
    mut a: *mut archive,
    mut fd: libc::c_int,
    mut block_size: size_t,
) -> libc::c_int {
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
    let mut mine: *mut read_fd_data = 0 as *mut read_fd_data;
    let mut b: *mut libc::c_void = 0 as *mut libc::c_void;
    archive_clear_error(a);
    if fstat(fd, &mut st) != 0 as libc::c_int {
        archive_set_error(
            a,
            errno,
            b"Can\'t stat fd %d\x00" as *const u8 as *const libc::c_char,
            fd,
        );
        return -(30 as libc::c_int);
    }
    mine = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<read_fd_data>() as libc::c_ulong,
    ) as *mut read_fd_data;
    b = malloc(block_size);
    if mine.is_null() || b.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        free(mine as *mut libc::c_void);
        free(b);
        return -(30 as libc::c_int);
    }
    (*mine).block_size = block_size;
    (*mine).buffer = b;
    (*mine).fd = fd;
    /*
     * Skip support is a performance optimization for anything
     * that supports lseek().  On FreeBSD, only regular files and
     * raw disk devices support lseek() and there's no portable
     * way to determine if a device is a raw disk device, so we
     * only enable this optimization for regular files.
     */
    if st.st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint {
        archive_read_extract_set_skip_file(a, st.st_dev as la_int64_t, st.st_ino as la_int64_t); /* off_t is a signed type. */
        (*mine).use_lseek = 1 as libc::c_int as libc::c_char
    }
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
    archive_read_set_close_callback(
        a,
        Some(
            file_close
                as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
    archive_read_set_callback_data(a, mine as *mut libc::c_void);
    return archive_read_open1(a);
}
unsafe extern "C" fn file_read(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut mine: *mut read_fd_data = client_data as *mut read_fd_data;
    let mut bytes_read: ssize_t = 0;
    *buff = (*mine).buffer;
    loop {
        bytes_read = read((*mine).fd, (*mine).buffer, (*mine).block_size);
        if bytes_read < 0 as libc::c_int as libc::c_long {
            if errno == EINTR {
                continue;
            }
            archive_set_error(
                a,
                errno,
                b"Error reading fd %d\x00" as *const u8 as *const libc::c_char,
                (*mine).fd,
            );
        }
        return bytes_read;
    }
}
unsafe extern "C" fn file_skip(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut request: int64_t,
) -> int64_t {
    let mut mine: *mut read_fd_data = client_data as *mut read_fd_data;
    let mut skip: int64_t = request;
    let mut old_offset: int64_t = 0;
    let mut new_offset: int64_t = 0;
    let mut skip_bits: libc::c_int = (::std::mem::size_of::<int64_t>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    if (*mine).use_lseek == 0 {
        return 0 as libc::c_int as int64_t;
    }
    /* Reduce a request that would overflow the 'skip' variable. */
    if ::std::mem::size_of::<int64_t>() as libc::c_ulong
        > ::std::mem::size_of::<int64_t>() as libc::c_ulong
    {
        let mut max_skip: int64_t = (((1 as libc::c_int as int64_t)
            << skip_bits - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long)
            * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        if request > max_skip {
            skip = max_skip
        }
    }
    /* Reduce request to the next smallest multiple of block_size */
    request = (request as libc::c_ulong)
        .wrapping_div((*mine).block_size)
        .wrapping_mul((*mine).block_size) as int64_t;
    if request == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as int64_t;
    }
    old_offset = lseek((*mine).fd, 0 as libc::c_int as __off_t, SEEK_CUR);
    if old_offset >= 0 as libc::c_int as libc::c_long && {
        new_offset = lseek((*mine).fd, skip, SEEK_CUR);
        (new_offset) >= 0 as libc::c_int as libc::c_long
    } {
        return new_offset - old_offset;
    }
    /* If seek failed once, it will probably fail again. */
    (*mine).use_lseek = 0 as libc::c_int as libc::c_char;
    /* Let libarchive recover with read+discard. */
    if errno == ESPIPE {
        return 0 as libc::c_int as int64_t;
    }
    /*
     * There's been an error other than ESPIPE. This is most
     * likely caused by a programmer error (too large request)
     * or a corrupted archive file.
     */
    archive_set_error(
        a,
        errno,
        b"Error seeking\x00" as *const u8 as *const libc::c_char,
    );
    return -(1 as libc::c_int) as int64_t;
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
    let mut mine: *mut read_fd_data = client_data as *mut read_fd_data;
    let mut r: int64_t = 0;
    /* We use off_t here because lseek() is declared that way. */
    /* See above for notes about when off_t is less than 64 bits. */
    r = lseek((*mine).fd, request, whence);
    if r >= 0 as libc::c_int as libc::c_long {
        return r;
    }
    if errno == ESPIPE {
        archive_set_error(
            a,
            errno,
            b"A file descriptor(%d) is not seekable(PIPE)\x00" as *const u8 as *const libc::c_char,
            (*mine).fd,
        );
        return -(25 as libc::c_int) as int64_t;
    } else {
        /* If the input is corrupted or truncated, fail. */
        archive_set_error(
            a,
            errno,
            b"Error seeking in a file descriptor(%d)\x00" as *const u8 as *const libc::c_char,
            (*mine).fd,
        );
        return -(30 as libc::c_int) as int64_t;
    };
}
unsafe extern "C" fn file_close(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut mine: *mut read_fd_data = client_data as *mut read_fd_data;
    /* UNUSED */
    free((*mine).buffer);
    free(mine as *mut libc::c_void);
    return 0 as libc::c_int;
}
