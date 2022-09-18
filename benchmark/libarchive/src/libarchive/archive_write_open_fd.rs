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
    fn free(__ptr: *mut libc::c_void);
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
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
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
pub struct write_fd_data {
    pub fd: libc::c_int,
}
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const NULL: libc::c_int = 0 as libc::c_int;
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
/* Read an archive that's already open, using a FILE *. */
/* Note: DO NOT use this with tape drives. */
/* Parses and returns next entry header. */
/* Parses and returns next entry header using the archive_entry passed in */
/*
 * Retrieve the byte offset in UNCOMPRESSED data where last-read
 * header started.
 */
/*
 * Returns 1 if the archive contains at least one encrypted entry.
 * If the archive format not support encryption at all
 * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned.
 * If for any other reason (e.g. not enough data read so far)
 * we cannot say whether there are encrypted entries, then
 * ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW is returned.
 * In general, this function will return values below zero when the
 * reader is uncertain or totally incapable of encryption support.
 * When this function returns 0 you can be sure that the reader
 * supports encryption detection but no encrypted entries have
 * been found yet.
 *
 * NOTE: If the metadata/header of an archive is also encrypted, you
 * cannot rely on the number of encrypted entries. That is why this
 * function does not return the number of encrypted entries but#
 * just shows that there are some.
 */
/*
 * Returns a bitmask of capabilities that are supported by the archive format reader.
 * If the reader has no special capabilities, ARCHIVE_READ_FORMAT_CAPS_NONE is returned.
 */
/* Read data from the body of an entry.  Similar to read(2). */
/* Seek within the body of an entry.  Similar to lseek(2). */
/*
 * A zero-copy version of archive_read_data that also exposes the file offset
 * of each returned block.  Note that the client has no way to specify
 * the desired size of the block.  The API does guarantee that offsets will
 * be strictly increasing and that returned blocks will not overlap.
 */
/*-
 * Some convenience functions that are built on archive_read_data:
 *  'skip': skips entire entry
 *  'into_buffer': writes data into memory buffer that you provide
 *  'into_fd': writes data to specified filedes
 */
/*
 * Set read options.
 */
/* Apply option to the format only. */
/* Apply option to the filter only. */
/* Apply option to both the format and the filter. */
/* Apply option string to both the format and the filter. */
/*
 * Add a decryption passphrase.
 */
/*-
 * Convenience function to recreate the current entry (whose header
 * has just been read) on disk.
 *
 * This does quite a bit more than just copy data to disk. It also:
 *  - Creates intermediate directories as required.
 *  - Manages directory permissions:  non-writable directories will
 *    be initially created with write permission enabled; when the
 *    archive is closed, dir permissions are edited to the values specified
 *    in the archive.
 *  - Checks hardlinks:  hardlinks will not be extracted unless the
 *    linked-to file was also extracted within the same session. (TODO)
 */
/* The "flags" argument selects optional behavior, 'OR' the flags you want. */
/* Default: Do not try to set owner/group. */
/* Default: Do obey umask, do not restore SUID/SGID/SVTX bits. */
/* Default: Do not restore mtime/atime. */
/* Default: Replace existing files. */
/* Default: Try create first, unlink only if create fails with EEXIST. */
/* Default: Do not restore ACLs. */
/* Default: Do not restore fflags. */
/* Default: Do not restore xattrs. */
/* Default: Do not try to guard against extracts redirected by symlinks. */
/* Note: With ARCHIVE_EXTRACT_UNLINK, will remove any intermediate symlink. */
/* Default: Do not reject entries with '..' as path elements. */
/* Default: Create parent directories as needed. */
/* Default: Overwrite files, even if one on disk is newer. */
/* Detect blocks of 0 and write holes instead. */
/* Default: Do not restore Mac extended metadata. */
/* This has no effect except on Mac OS. */
/* Default: Use HFS+ compression if it was compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not use HFS+ compression if it was not compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not reject entries with absolute paths */
/* Default: Do not clear no-change flags when unlinking object */
/* Default: Do not extract atomically (using rename) */
/* dest */
/* Record the dev/ino of a file that will not be written.  This is
 * generally set to the dev/ino of the archive being read. */
/* Close the file and release most resources. */
/* Release all resources and destroy the object. */
/* Note that archive_read_free will call archive_read_close for you. */
/* Synonym for archive_read_free() for backwards compatibility. */
/*-
 * To create an archive:
 *   1) Ask archive_write_new for an archive writer object.
 *   2) Set any global properties.  In particular, you should set
 *      the compression and format to use.
 *   3) Call archive_write_open to open the file (most people
 *       will use archive_write_open_file or archive_write_open_fd,
 *       which provide convenient canned I/O callbacks for you).
 *   4) For each entry:
 *      - construct an appropriate struct archive_entry structure
 *      - archive_write_header to write the header
 *      - archive_write_data to write the entry data
 *   5) archive_write_close to close the output
 *   6) archive_write_free to cleanup the writer and release resources
 */
/* XXX This is badly misnamed; suggestions appreciated. XXX */
/* The dev/ino of a file that won't be archived.  This is used
 * to avoid recursively adding an archive to itself. */
/* A convenience function to set the filter based on the code. */
/* A convenience function to set the format based on the code or name. */
/* To minimize link pollution, use one or more of the following. */
/* TODO: int archive_write_set_format_old_tar(struct archive *); */
#[no_mangle]
pub unsafe extern "C" fn archive_write_open_fd(
    mut a: *mut archive,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut mine: *mut write_fd_data = 0 as *mut write_fd_data;
    mine = malloc(::std::mem::size_of::<write_fd_data>() as libc::c_ulong) as *mut write_fd_data;
    if mine.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*mine).fd = fd;
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
    let mut mine: *mut write_fd_data = 0 as *mut write_fd_data;
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
    mine = client_data as *mut write_fd_data;
    if fstat((*mine).fd, &mut st) != 0 as libc::c_int {
        archive_set_error(
            a,
            errno,
            b"Couldn\'t stat fd %d\x00" as *const u8 as *const libc::c_char,
            (*mine).fd,
        );
        return -(30 as libc::c_int);
    }
    /*
     * If this is a regular file, don't add it to itself.
     */
    if st.st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint {
        archive_write_set_skip_file(a, st.st_dev as la_int64_t, st.st_ino as la_int64_t);
    }
    /*
     * If client hasn't explicitly set the last block handling,
     * then set it here.
     */
    if archive_write_get_bytes_in_last_block(a) < 0 as libc::c_int {
        /* If the output is a block or character device, fifo,
         * or stdout, pad the last block, otherwise leave it
         * unpadded. */
        if st.st_mode & __S_IFMT as libc::c_uint == 0o20000 as libc::c_int as libc::c_uint
            || st.st_mode & __S_IFMT as libc::c_uint == 0o60000 as libc::c_int as libc::c_uint
            || st.st_mode & __S_IFMT as libc::c_uint == 0o10000 as libc::c_int as libc::c_uint
            || (*mine).fd == 1 as libc::c_int
        {
            /* Last block will be fully padded. */
            archive_write_set_bytes_in_last_block(a, 0 as libc::c_int);
        } else {
            archive_write_set_bytes_in_last_block(a, 1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_write(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> ssize_t {
    let mut mine: *mut write_fd_data = 0 as *mut write_fd_data;
    let mut bytesWritten: ssize_t = 0;
    mine = client_data as *mut write_fd_data;
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
    let mut mine: *mut write_fd_data = client_data as *mut write_fd_data;
    /* UNUSED */
    free(mine as *mut libc::c_void);
    return 0 as libc::c_int;
}
