use ::libc;
extern "C" {
    pub type __dirstream;
    pub type archive_string_conv;
    /*-
     * Copyright (c) 2003-2009 Tim Kientzle
     * All rights reserved.
     *
     * Redistribution and use in source and binary forms, with or without
     * modification, are permitted provided that the following conditions
     * are met:
     * 1. Redistributions of source code must retain the above copyright
     *    notice, this list of conditions and the following disclaimer
     *    in this position and unchanged.
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
     * $FreeBSD: head/lib/libarchive/archive_read_disk_private.h 201105 2009-12-28 03:20:54Z kientzle $
     */
    pub type archive_entry;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    #[no_mangle]
    fn fstatfs(__fildes: libc::c_int, __buf: *mut statfs) -> libc::c_int;
    #[no_mangle]
    fn fstatvfs(__fildes: libc::c_int, __buf: *mut statvfs) -> libc::c_int;
    #[no_mangle]
    fn lutimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn futimesat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __tvp: *const timeval,
    ) -> libc::c_int;
    #[no_mangle]
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn dup(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn fpathconf(__fd: libc::c_int, __name: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn archive_read_disk_entry_from_file(
        _: *mut archive,
        _: *mut archive_entry,
        _: libc::c_int,
        _: *const stat,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_match_time_excluded(_: *mut archive, _: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_match_path_excluded(_: *mut archive, _: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_error_string(_: *mut archive) -> *const libc::c_char;
    /*
     * Test if a file is excluded by its uid ,gid, uname or gname.
     * The conditions are set by following functions.
     */
    #[no_mangle]
    fn archive_match_owner_excluded(_: *mut archive, _: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_string_append_from_wcs(
        _: *mut archive_string,
        _: *const wchar_t,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    /*
     * Symlink types
     */
    /*
     * Basic object manipulation
     */
    #[no_mangle]
    fn archive_entry_clear(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    /*
     * This form of archive_entry_new2() will pull character-set
     * conversion information from the specified archive handle.  The
     * older archive_entry_new(void) form is equivalent to calling
     * archive_entry_new2(NULL) and will result in the use of an internal
     * default character-set conversion.
     */
    #[no_mangle]
    fn archive_entry_new2(_: *mut archive) -> *mut archive_entry;
    /*
     * Retrieve fields from an archive_entry.
     *
     * There are a number of implicit conversions among these fields.  For
     * example, if a regular string field is set and you read the _w wide
     * character field, the entry will implicitly convert narrow-to-wide
     * using the current locale.  Similarly, dev values are automatically
     * updated when you write devmajor or devminor and vice versa.
     *
     * In addition, fields can be "set" or "unset."  Unset string fields
     * return NULL, non-string fields have _is_set() functions to test
     * whether they've been set.  You can "unset" a string field by
     * assigning NULL; non-string fields have _unset() functions to
     * unset them.
     *
     * Note: There is one ambiguity in the above; string fields will
     * also return NULL when implicit character set conversions fail.
     * This is usually what you want.
     */
    #[no_mangle]
    fn archive_entry_atime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_atime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_copy_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_sourcepath(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_uname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_stat(_: *mut archive_entry, _: *const stat);
    #[no_mangle]
    fn archive_entry_sparse_reset(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_sparse_next(
        _: *mut archive_entry,
        _: *mut la_int64_t,
        _: *mut la_int64_t,
    ) -> libc::c_int;
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_clean(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn __archive_ensure_cloexec_flag(fd: libc::c_int);
    #[no_mangle]
    fn __archive_reset_read_data(_: *mut archive);
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uintptr_t = libc::c_ulong;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: libc::c_ulong,
    pub f_frsize: libc::c_ulong,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: libc::c_ulong,
    pub f_flag: libc::c_ulong,
    pub f_namemax: libc::c_ulong,
    pub __f_spare: [libc::c_int; 6],
}
pub type C2RustUnnamed = libc::c_uint;
pub const ST_RELATIME: C2RustUnnamed = 4096;
pub const ST_NODIRATIME: C2RustUnnamed = 2048;
pub const ST_NOATIME: C2RustUnnamed = 1024;
pub const ST_IMMUTABLE: C2RustUnnamed = 512;
pub const ST_APPEND: C2RustUnnamed = 256;
pub const ST_WRITE: C2RustUnnamed = 128;
pub const ST_MANDLOCK: C2RustUnnamed = 64;
pub const ST_SYNCHRONOUS: C2RustUnnamed = 16;
pub const ST_NOEXEC: C2RustUnnamed = 8;
pub const ST_NODEV: C2RustUnnamed = 4;
pub const ST_NOSUID: C2RustUnnamed = 2;
pub const ST_RDONLY: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type wchar_t = libc::c_int;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _PC_2_SYMLINKS: C2RustUnnamed_0 = 20;
pub const _PC_SYMLINK_MAX: C2RustUnnamed_0 = 19;
pub const _PC_ALLOC_SIZE_MIN: C2RustUnnamed_0 = 18;
pub const _PC_REC_XFER_ALIGN: C2RustUnnamed_0 = 17;
pub const _PC_REC_MIN_XFER_SIZE: C2RustUnnamed_0 = 16;
pub const _PC_REC_MAX_XFER_SIZE: C2RustUnnamed_0 = 15;
pub const _PC_REC_INCR_XFER_SIZE: C2RustUnnamed_0 = 14;
pub const _PC_FILESIZEBITS: C2RustUnnamed_0 = 13;
pub const _PC_SOCK_MAXBUF: C2RustUnnamed_0 = 12;
pub const _PC_PRIO_IO: C2RustUnnamed_0 = 11;
pub const _PC_ASYNC_IO: C2RustUnnamed_0 = 10;
pub const _PC_SYNC_IO: C2RustUnnamed_0 = 9;
pub const _PC_VDISABLE: C2RustUnnamed_0 = 8;
pub const _PC_NO_TRUNC: C2RustUnnamed_0 = 7;
pub const _PC_CHOWN_RESTRICTED: C2RustUnnamed_0 = 6;
pub const _PC_PIPE_BUF: C2RustUnnamed_0 = 5;
pub const _PC_PATH_MAX: C2RustUnnamed_0 = 4;
pub const _PC_NAME_MAX: C2RustUnnamed_0 = 3;
pub const _PC_MAX_INPUT: C2RustUnnamed_0 = 2;
pub const _PC_MAX_CANON: C2RustUnnamed_0 = 1;
pub const _PC_LINK_MAX: C2RustUnnamed_0 = 0;
pub type la_int64_t = int64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_disk {
    pub archive: archive,
    pub entry: *mut archive_entry,
    pub symlink_mode: libc::c_char,
    pub follow_symlinks: libc::c_char,
    pub tree: *mut tree,
    pub open_on_current_dir: Option<
        unsafe extern "C" fn(_: *mut tree, _: *const libc::c_char, _: libc::c_int) -> libc::c_int,
    >,
    pub tree_current_dir_fd: Option<unsafe extern "C" fn(_: *mut tree) -> libc::c_int>,
    pub tree_enter_working_dir: Option<unsafe extern "C" fn(_: *mut tree) -> libc::c_int>,
    pub flags: libc::c_int,
    pub lookup_gname:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: int64_t) -> *const libc::c_char>,
    pub cleanup_gname: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub lookup_gname_data: *mut libc::c_void,
    pub lookup_uname:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: int64_t) -> *const libc::c_char>,
    pub cleanup_uname: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub lookup_uname_data: *mut libc::c_void,
    pub metadata_filter_func: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut libc::c_void,
            _: *mut archive_entry,
        ) -> libc::c_int,
    >,
    pub metadata_filter_data: *mut libc::c_void,
    pub matching: *mut archive,
    pub excluded_cb_func: Option<
        unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void, _: *mut archive_entry) -> (),
    >,
    pub excluded_cb_data: *mut libc::c_void,
}
/* This entry needs to be postvisited. */
/*
 * Local data for this package.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree {
    pub stack: *mut tree_entry,
    pub current: *mut tree_entry,
    pub d: *mut DIR,
    pub de: *mut dirent,
    pub flags: libc::c_int,
    pub visit_type: libc::c_int,
    pub tree_errno: libc::c_int,
    pub path: archive_string,
    pub basename: *const libc::c_char,
    pub dirname_length: size_t,
    pub depth: libc::c_int,
    pub openCount: libc::c_int,
    pub maxOpenCount: libc::c_int,
    pub initial_dir_fd: libc::c_int,
    pub working_dir_fd: libc::c_int,
    pub lst: stat,
    pub st: stat,
    pub descend: libc::c_int,
    pub nlink: libc::c_int,
    pub restore_time: restore_time,
    pub sparse_list: *mut entry_sparse,
    pub current_sparse: *mut entry_sparse,
    pub sparse_count: libc::c_int,
    pub sparse_list_size: libc::c_int,
    pub initial_symlink_mode: libc::c_char,
    pub symlink_mode: libc::c_char,
    pub current_filesystem: *mut filesystem,
    pub filesystem_table: *mut filesystem,
    pub initial_filesystem_id: libc::c_int,
    pub current_filesystem_id: libc::c_int,
    pub max_filesystem_id: libc::c_int,
    pub allocated_filesystem: libc::c_int,
    pub entry_fd: libc::c_int,
    pub entry_eof: libc::c_int,
    pub entry_remaining_bytes: int64_t,
    pub entry_total: int64_t,
    pub entry_buff: *mut libc::c_uchar,
    pub entry_buff_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filesystem {
    pub dev: int64_t,
    pub synthetic: libc::c_int,
    pub remote: libc::c_int,
    pub noatime: libc::c_int,
    pub incr_xfer_size: libc::c_long,
    pub max_xfer_size: libc::c_long,
    pub min_xfer_size: libc::c_long,
    pub xfer_align: libc::c_long,
    pub allocation_ptr: *mut libc::c_uchar,
    pub buff: *mut libc::c_uchar,
    pub buff_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry_sparse {
    pub length: int64_t,
    pub offset: int64_t,
}
/*-
 * This is a new directory-walking system that addresses a number
 * of problems I've had with fts(3).  In particular, it has no
 * pathname-length limits (other than the size of 'int'), handles
 * deep logical traversals, uses considerably less memory, and has
 * an opaque interface (easier to modify in the future).
 *
 * Internally, it keeps a single list of "tree_entry" items that
 * represent filesystem objects that require further attention.
 * Non-directories are not kept in memory: they are pulled from
 * readdir(), returned to the client, then freed as soon as possible.
 * Any directory entry to be traversed gets pushed onto the stack.
 *
 * There is surprisingly little information that needs to be kept for
 * each item on the stack.  Just the name, depth (represented here as the
 * string length of the parent directory's pathname), and some markers
 * indicating how to get back to the parent (via chdir("..") for a
 * regular dir or via fchdir(2) for a symlink).
 */
/*
 * TODO:
 *    1) Loop checking.
 *    3) Arbitrary logical traversals by closing/reopening intermediate fds.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct restore_time {
    pub name: *const libc::c_char,
    pub mtime: time_t,
    pub mtime_nsec: libc::c_long,
    pub atime: time_t,
    pub atime_nsec: libc::c_long,
    pub filetype: mode_t,
    pub noatime: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_entry {
    pub depth: libc::c_int,
    pub next: *mut tree_entry,
    pub parent: *mut tree_entry,
    pub name: archive_string,
    pub dirname_length: size_t,
    pub dev: int64_t,
    pub ino: int64_t,
    pub flags: libc::c_int,
    pub filesystem_id: libc::c_int,
    pub symlink_parent_fd: libc::c_int,
    pub restore_time: restore_time,
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const _IOC_DIRSHIFT: libc::c_int = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const _IOC_SIZEBITS: libc::c_int = 14 as libc::c_int;
pub const _IOC_SIZESHIFT: libc::c_int = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_TYPESHIFT: libc::c_int = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_NRSHIFT: libc::c_int = 0 as libc::c_int;
pub const _IOC_NRBITS: libc::c_int = 8 as libc::c_int;
pub const _IOC_TYPEBITS: libc::c_int = 8 as libc::c_int;
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const ST_NOATIME_0: libc::c_int = ST_NOATIME as libc::c_int;
pub const AFS_SUPER_MAGIC: libc::c_int = 0x5346414f as libc::c_int;
pub const CODA_SUPER_MAGIC: libc::c_int = 0x73757245 as libc::c_int;
pub const NCP_SUPER_MAGIC: libc::c_int = 0x564c as libc::c_int;
pub const NFS_SUPER_MAGIC: libc::c_int = 0x6969 as libc::c_int;
pub const SMB_SUPER_MAGIC: libc::c_int = 0x517b as libc::c_int;
pub const PROC_SUPER_MAGIC: libc::c_int = 0x9fa0 as libc::c_int;
pub const USBDEVICE_SUPER_MAGIC: libc::c_int = 0x9fa2 as libc::c_int;
pub const FS_NODUMP_FL: libc::c_int = 0x40 as libc::c_int;
pub const FS_IOC_GETFLAGS: libc::c_ulong = ((2 as libc::c_uint) << _IOC_DIRSHIFT
    | (('f' as i32) << _IOC_TYPESHIFT) as libc::c_uint
    | ((1 as libc::c_int) << _IOC_NRSHIFT) as libc::c_uint)
    as libc::c_ulong
    | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << _IOC_SIZESHIFT;
pub const errno: libc::c_int = *__errno_location();
pub const EPERM: libc::c_int = 1 as libc::c_int;
pub const ENOENT: libc::c_int = 2 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const O_NONBLOCK: libc::c_int = 0o4000 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const AT_SYMLINK_NOFOLLOW: libc::c_int = 0x100 as libc::c_int;
pub const O_PATH: libc::c_int = __O_PATH;
pub const F_DUPFD_CLOEXEC: libc::c_int = 1030 as libc::c_int;
pub const __O_DIRECTORY: libc::c_int = 0o200000 as libc::c_int;
pub const O_DIRECTORY: libc::c_int = __O_DIRECTORY;
pub const __O_NOATIME: libc::c_int = 0o1000000 as libc::c_int;
pub const O_NOATIME: libc::c_int = __O_NOATIME;
pub const __O_PATH: libc::c_int = 0o10000000 as libc::c_int;
pub const _PC_REC_XFER_ALIGN_0: libc::c_int = _PC_REC_XFER_ALIGN as libc::c_int;
pub const _PC_REC_MIN_XFER_SIZE_0: libc::c_int = _PC_REC_MIN_XFER_SIZE as libc::c_int;
pub const _PC_REC_MAX_XFER_SIZE_0: libc::c_int = _PC_REC_MAX_XFER_SIZE as libc::c_int;
pub const _PC_REC_INCR_XFER_SIZE_0: libc::c_int = _PC_REC_INCR_XFER_SIZE as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_RETRY: libc::c_int = -(10 as libc::c_int);
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_READDISK_RESTORE_ATIME: libc::c_int = 0x1 as libc::c_int;
pub const ARCHIVE_READDISK_NO_TRAVERSE_MOUNTS: libc::c_int = 0x8 as libc::c_int;
pub const ARCHIVE_READDISK_HONOR_NODUMP: libc::c_int = 0x2 as libc::c_int;
pub const ARCHIVE_READDISK_MAC_COPYFILE: libc::c_int = 0x4 as libc::c_int;
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
pub const ARCHIVE_READ_DISK_MAGIC: libc::c_uint = 0xbadb0c5 as libc::c_uint;
pub const ARCHIVE_STATE_NEW: libc::c_uint = 1 as libc::c_uint;
pub const ARCHIVE_STATE_HEADER: libc::c_uint = 2 as libc::c_uint;
pub const ARCHIVE_STATE_DATA: libc::c_uint = 4 as libc::c_uint;
pub const ARCHIVE_STATE_EOF: libc::c_uint = 0x10 as libc::c_uint;
pub const ARCHIVE_STATE_CLOSED: libc::c_uint = 0x20 as libc::c_uint;
pub const ARCHIVE_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
pub const ARCHIVE_STATE_ANY: libc::c_uint = 0xffff as libc::c_uint & !ARCHIVE_STATE_FATAL;
/*
 * Some Linux distributions have both linux/ext2_fs.h and ext2fs/ext2_fs.h.
 * As the include guards don't agree, the order of include is important.
 */
pub const O_BINARY: libc::c_int = 0 as libc::c_int;
/* Definitions for tree_entry.flags bitmap. */
pub const isDir: libc::c_int = 1 as libc::c_int;
/* This entry is a regular directory. */
pub const isDirLink: libc::c_int = 2 as libc::c_int;
/* This entry is a symbolic link to a directory. */
pub const needsFirstVisit: libc::c_int = 4 as libc::c_int;
/* This is an initial entry. */
pub const needsDescent: libc::c_int = 8 as libc::c_int;
/* This entry needs to be previsited. */
pub const needsOpen: libc::c_int = 16 as libc::c_int;
/* This is a directory that needs to be opened. */
pub const needsAscent: libc::c_int = 32 as libc::c_int;
pub const INVALID_DIR_HANDLE: libc::c_int = 0 as libc::c_int;
/* Definitions for tree.flags bitmap. */
pub const hasStat: libc::c_int = 16 as libc::c_int;
/* The st entry is valid. */
pub const hasLstat: libc::c_int = 32 as libc::c_int;
/* The lst entry is valid. */
pub const onWorkingDir: libc::c_int = 64 as libc::c_int;
/* We are on the working dir where we are
 * reading directory entry at this time. */
pub const needsRestoreTimes: libc::c_int = 128 as libc::c_int;
pub const onInitialDir: libc::c_int = 256 as libc::c_int;
/*
 * tree_next() returns Zero if there is no next entry, non-zero if
 * there is.  Note that directories are visited three times.
 * Directories are always visited first as part of enumerating their
 * parent; that is a "regular" visit.  If tree_descend() is invoked at
 * that time, the directory is added to a work list and will
 * subsequently be visited two more times: once just after descending
 * into the directory ("postdescent") and again just after ascending
 * back to the parent ("postascent").
 *
 * TREE_ERROR_DIR is returned if the descent failed (because the
 * directory couldn't be opened, for instance).  This is returned
 * instead of TREE_POSTDESCENT/TREE_POSTASCENT.  TREE_ERROR_DIR is not a
 * fatal error, but it does imply that the relevant subtree won't be
 * visited.  TREE_ERROR_FATAL is returned for an error that left the
 * traversal completely hosed.  Right now, this is only returned for
 * chdir() failures during ascent.
 */
pub const TREE_REGULAR: libc::c_int = 1 as libc::c_int;
pub const TREE_POSTDESCENT: libc::c_int = 2 as libc::c_int;
pub const TREE_POSTASCENT: libc::c_int = 3 as libc::c_int;
pub const TREE_ERROR_DIR: libc::c_int = -(1 as libc::c_int);
pub const TREE_ERROR_FATAL: libc::c_int = -(2 as libc::c_int);
unsafe extern "C" fn archive_read_disk_vtable() -> *mut archive_vtable {
    static mut av: archive_vtable = archive_vtable {
        archive_close: None,
        archive_free: None,
        archive_write_header: None,
        archive_write_finish_entry: None,
        archive_write_data: None,
        archive_write_data_block: None,
        archive_read_next_header: None,
        archive_read_next_header2: None,
        archive_read_data_block: None,
        archive_filter_count: None,
        archive_filter_bytes: None,
        archive_filter_code: None,
        archive_filter_name: None,
    };
    static mut inited: libc::c_int = 0 as libc::c_int;
    if inited == 0 {
        av.archive_free =
            Some(_archive_read_free as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_close =
            Some(_archive_read_close as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_read_data_block = Some(
            _archive_read_data_block
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        );
        av.archive_read_next_header = Some(
            _archive_read_next_header
                as unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int,
        );
        av.archive_read_next_header2 = Some(
            _archive_read_next_header2
                as unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int,
        );
        inited = 1 as libc::c_int
    }
    return &mut av;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_gname(
    mut _a: *mut archive,
    mut gid: la_int64_t,
) -> *const libc::c_char {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    if ARCHIVE_OK
        != __archive_check_magic(
            _a,
            ARCHIVE_READ_DISK_MAGIC,
            ARCHIVE_STATE_ANY,
            b"archive_read_disk_gname\x00" as *const u8 as *const libc::c_char,
        )
    {
        return 0 as *const libc::c_char;
    }
    if (*a).lookup_gname.is_none() {
        return 0 as *const libc::c_char;
    }
    return Some((*a).lookup_gname.expect("non-null function pointer"))
        .expect("non-null function pointer")((*a).lookup_gname_data, gid);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_uname(
    mut _a: *mut archive,
    mut uid: la_int64_t,
) -> *const libc::c_char {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    if ARCHIVE_OK
        != __archive_check_magic(
            _a,
            ARCHIVE_READ_DISK_MAGIC,
            ARCHIVE_STATE_ANY,
            b"archive_read_disk_uname\x00" as *const u8 as *const libc::c_char,
        )
    {
        return 0 as *const libc::c_char;
    }
    if (*a).lookup_uname.is_none() {
        return 0 as *const libc::c_char;
    }
    return Some((*a).lookup_uname.expect("non-null function pointer"))
        .expect("non-null function pointer")((*a).lookup_uname_data, uid);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_gname_lookup(
    mut _a: *mut archive,
    mut private_data: *mut libc::c_void,
    mut lookup_gname: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: la_int64_t) -> *const libc::c_char,
    >,
    mut cleanup_gname: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_set_gname_lookup\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).cleanup_gname.is_some() && !(*a).lookup_gname_data.is_null() {
        (*a).cleanup_gname.expect("non-null function pointer")((*a).lookup_gname_data);
    }
    (*a).lookup_gname = lookup_gname;
    (*a).cleanup_gname = cleanup_gname;
    (*a).lookup_gname_data = private_data;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_uname_lookup(
    mut _a: *mut archive,
    mut private_data: *mut libc::c_void,
    mut lookup_uname: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: la_int64_t) -> *const libc::c_char,
    >,
    mut cleanup_uname: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_set_uname_lookup\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).cleanup_uname.is_some() && !(*a).lookup_uname_data.is_null() {
        (*a).cleanup_uname.expect("non-null function pointer")((*a).lookup_uname_data);
    }
    (*a).lookup_uname = lookup_uname;
    (*a).cleanup_uname = cleanup_uname;
    (*a).lookup_uname_data = private_data;
    return 0 as libc::c_int;
}
/*
 * Create a new archive_read_disk object and initialize it with global state.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_new() -> *mut archive {
    let mut a: *mut archive_read_disk = 0 as *mut archive_read_disk; /* Follow symlinks initially. */
    a = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_read_disk>() as libc::c_ulong,
    ) as *mut archive_read_disk;
    if a.is_null() {
        return 0 as *mut archive;
    }
    (*a).archive.magic = ARCHIVE_READ_DISK_MAGIC;
    (*a).archive.state = ARCHIVE_STATE_NEW;
    (*a).archive.vtable = archive_read_disk_vtable();
    (*a).entry = archive_entry_new2(&mut (*a).archive);
    (*a).lookup_uname = Some(
        trivial_lookup_uname
            as unsafe extern "C" fn(_: *mut libc::c_void, _: int64_t) -> *const libc::c_char,
    );
    (*a).lookup_gname = Some(
        trivial_lookup_gname
            as unsafe extern "C" fn(_: *mut libc::c_void, _: int64_t) -> *const libc::c_char,
    );
    (*a).flags = ARCHIVE_READDISK_MAC_COPYFILE;
    (*a).open_on_current_dir = Some(
        open_on_current_dir
            as unsafe extern "C" fn(
                _: *mut tree,
                _: *const libc::c_char,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    (*a).tree_current_dir_fd =
        Some(tree_current_dir_fd as unsafe extern "C" fn(_: *mut tree) -> libc::c_int);
    (*a).tree_enter_working_dir =
        Some(tree_enter_working_dir as unsafe extern "C" fn(_: *mut tree) -> libc::c_int);
    return &mut (*a).archive;
}
unsafe extern "C" fn _archive_read_free(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut r: libc::c_int = 0;
    if _a.is_null() {
        return 0 as libc::c_int;
    }
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_read_free\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state != ARCHIVE_STATE_CLOSED {
        r = _archive_read_close(&mut (*a).archive)
    } else {
        r = ARCHIVE_OK
    }
    tree_free((*a).tree);
    if (*a).cleanup_gname.is_some() && !(*a).lookup_gname_data.is_null() {
        (*a).cleanup_gname.expect("non-null function pointer")((*a).lookup_gname_data);
    }
    if (*a).cleanup_uname.is_some() && !(*a).lookup_uname_data.is_null() {
        (*a).cleanup_uname.expect("non-null function pointer")((*a).lookup_uname_data);
    }
    archive_string_free(&mut (*a).archive.error_string);
    archive_entry_free((*a).entry);
    (*a).archive.magic = 0 as libc::c_int as libc::c_uint;
    __archive_clean(&mut (*a).archive);
    free(a as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn _archive_read_close(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_read_close\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state != ARCHIVE_STATE_FATAL {
        (*a).archive.state = ARCHIVE_STATE_CLOSED
    }
    tree_close((*a).tree);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setup_symlink_mode(
    mut a: *mut archive_read_disk,
    mut symlink_mode: libc::c_char,
    mut follow_symlinks: libc::c_int,
) {
    (*a).symlink_mode = symlink_mode;
    (*a).follow_symlinks = follow_symlinks as libc::c_char;
    if !(*a).tree.is_null() {
        (*(*a).tree).initial_symlink_mode = (*a).symlink_mode;
        (*(*a).tree).symlink_mode = (*a).symlink_mode
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_symlink_logical(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_set_symlink_logical\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    setup_symlink_mode(a, 'L' as i32 as libc::c_char, 1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_symlink_physical(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_set_symlink_physical\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    setup_symlink_mode(a, 'P' as i32 as libc::c_char, 0 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_symlink_hybrid(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_set_symlink_hybrid\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    setup_symlink_mode(a, 'H' as i32 as libc::c_char, 1 as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_atime_restored(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_restore_atime\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).flags |= ARCHIVE_READDISK_RESTORE_ATIME;
    if !(*a).tree.is_null() {
        (*(*a).tree).flags |= needsRestoreTimes
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_behavior(
    mut _a: *mut archive,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_honor_nodump\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).flags = flags;
    if flags & ARCHIVE_READDISK_RESTORE_ATIME != 0 {
        r = archive_read_disk_set_atime_restored(_a)
    } else if !(*a).tree.is_null() {
        (*(*a).tree).flags &= !needsRestoreTimes
    }
    return r;
}
/*
 * Trivial implementations of gname/uname lookup functions.
 * These are normally overridden by the client, but these stub
 * versions ensure that we always have something that works.
 */
unsafe extern "C" fn trivial_lookup_gname(
    mut private_data: *mut libc::c_void,
    mut gid: int64_t,
) -> *const libc::c_char {
    /* UNUSED */
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn trivial_lookup_uname(
    mut private_data: *mut libc::c_void,
    mut uid: int64_t,
) -> *const libc::c_char {
    /* UNUSED */
    return 0 as *const libc::c_char;
}
/*
 * Allocate memory for the reading buffer adjusted to the filesystem
 * alignment.
 */
unsafe extern "C" fn setup_suitable_read_buffer(mut a: *mut archive_read_disk) -> libc::c_int {
    let mut t: *mut tree = (*a).tree;
    let mut cf: *mut filesystem = (*t).current_filesystem;
    let mut asize: size_t = 0;
    let mut s: size_t = 0;
    if (*cf).allocation_ptr.is_null() {
        /* If we couldn't get a filesystem alignment,
         * we use 4096 as default value but we won't use
         * O_DIRECT to open() and openat() operations. */
        let mut xfer_align: libc::c_long =
            if (*cf).xfer_align == -(1 as libc::c_int) as libc::c_long {
                4096 as libc::c_int as libc::c_long
            } else {
                (*cf).xfer_align
            };
        if (*cf).max_xfer_size != -(1 as libc::c_int) as libc::c_long {
            asize = ((*cf).max_xfer_size + xfer_align) as size_t
        } else {
            let mut incr: libc::c_long = (*cf).incr_xfer_size;
            /* Some platform does not set a proper value to
             * incr_xfer_size.*/
            if incr < 0 as libc::c_int as libc::c_long {
                incr = (*cf).min_xfer_size
            }
            if (*cf).min_xfer_size < 0 as libc::c_int as libc::c_long {
                incr = xfer_align;
                asize = xfer_align as size_t
            } else {
                asize = (*cf).min_xfer_size as size_t
            }
            /* Increase a buffer size up to 64K bytes in
             * a proper increment size. */
            while asize < (1024 as libc::c_int * 64 as libc::c_int) as libc::c_ulong {
                asize =
                    (asize as libc::c_ulong).wrapping_add(incr as libc::c_ulong) as size_t as size_t
            }
            /* Take a margin to adjust to the filesystem
             * alignment. */
            asize = (asize as libc::c_ulong).wrapping_add(xfer_align as libc::c_ulong) as size_t
                as size_t
        }
        (*cf).allocation_ptr = malloc(asize) as *mut libc::c_uchar;
        if (*cf).allocation_ptr.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Couldn\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            (*a).archive.state = ARCHIVE_STATE_FATAL;
            return -(30 as libc::c_int);
        }
        /*
         * Calculate proper address for the filesystem.
         */
        s = (*cf).allocation_ptr as uintptr_t;
        s = (s as libc::c_ulong).wrapping_rem(xfer_align as libc::c_ulong) as size_t as size_t;
        if s > 0 as libc::c_int as libc::c_ulong {
            s = (xfer_align as libc::c_ulong).wrapping_sub(s)
        }
        /*
         * Set a read buffer pointer in the proper alignment of
         * the current filesystem.
         */
        (*cf).buff = (*cf).allocation_ptr.offset(s as isize);
        (*cf).buff_size = asize.wrapping_sub(xfer_align as libc::c_ulong)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _archive_read_data_block(
    mut _a: *mut archive,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut t: *mut tree = (*a).tree;
    let mut r: libc::c_int = 0;
    let mut bytes: ssize_t = 0;
    let mut sparse_bytes: int64_t = 0;
    let mut buffbytes: size_t = 0;
    let mut empty_sparse_region: libc::c_int = 0 as libc::c_int;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_read_data_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*t).entry_eof != 0 || (*t).entry_remaining_bytes <= 0 as libc::c_int as libc::c_long {
        r = ARCHIVE_EOF
    } else {
        /*
         * Open the current file.
         */
        if (*t).entry_fd < 0 as libc::c_int {
            let mut flags: libc::c_int = O_RDONLY | O_BINARY | O_CLOEXEC;
            /*
             * Eliminate or reduce cache effects if we can.
             *
             * Carefully consider this to be enabled.
             */
            /* Disabled for now */
            /*
             * Linux has O_NOATIME flag; use it if we need.
             */
            if (*t).flags & needsRestoreTimes != 0 as libc::c_int
                && (*t).restore_time.noatime == 0 as libc::c_int
            {
                flags |= O_NOATIME
            }
            (*t).entry_fd = open_on_current_dir(t, tree_current_access_path(t), flags);
            __archive_ensure_cloexec_flag((*t).entry_fd);
            /*
             * When we did open the file with O_NOATIME flag,
             * if successful, set 1 to t->restore_time.noatime
             * not to restore an atime of the file later.
             * if failed by EPERM, retry it without O_NOATIME flag.
             */
            if flags & O_NOATIME != 0 {
                if (*t).entry_fd >= 0 as libc::c_int {
                    (*t).restore_time.noatime = 1 as libc::c_int
                } else if errno == EPERM {
                    flags &= !O_NOATIME
                }
            }
            if (*t).entry_fd < 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Couldn\'t open %s\x00" as *const u8 as *const libc::c_char,
                    tree_current_path(t),
                );
                r = ARCHIVE_FAILED;
                tree_enter_initial_dir(t);
                current_block = 341217363119494045;
            } else {
                tree_enter_initial_dir(t);
                current_block = 14818589718467733107;
            }
        } else {
            current_block = 14818589718467733107;
        }
        match current_block {
            341217363119494045 => {}
            _ =>
            /*
             * Allocate read buffer if not allocated.
             */
            {
                if (*(*t).current_filesystem).allocation_ptr.is_null() {
                    r = setup_suitable_read_buffer(a);
                    if r != ARCHIVE_OK {
                        (*a).archive.state = ARCHIVE_STATE_FATAL;
                        current_block = 341217363119494045;
                    } else {
                        current_block = 652864300344834934;
                    }
                } else {
                    current_block = 652864300344834934;
                }
                match current_block {
                    341217363119494045 => {}
                    _ => {
                        (*t).entry_buff = (*(*t).current_filesystem).buff;
                        (*t).entry_buff_size = (*(*t).current_filesystem).buff_size;
                        buffbytes = (*t).entry_buff_size;
                        if buffbytes as int64_t > (*(*t).current_sparse).length {
                            buffbytes = (*(*t).current_sparse).length as size_t
                        }
                        if (*(*t).current_sparse).length == 0 as libc::c_int as libc::c_long {
                            empty_sparse_region = 1 as libc::c_int
                        }
                        /*
                         * Skip hole.
                         * TODO: Should we consider t->current_filesystem->xfer_align?
                         */
                        if (*(*t).current_sparse).offset > (*t).entry_total {
                            if lseek((*t).entry_fd, (*(*t).current_sparse).offset, SEEK_SET)
                                < 0 as libc::c_int as libc::c_long
                            {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    errno,
                                    b"Seek error\x00" as *const u8 as *const libc::c_char,
                                );
                                r = ARCHIVE_FATAL;
                                (*a).archive.state = ARCHIVE_STATE_FATAL;
                                current_block = 341217363119494045;
                            } else {
                                sparse_bytes = (*(*t).current_sparse).offset - (*t).entry_total;
                                (*t).entry_remaining_bytes -= sparse_bytes;
                                (*t).entry_total += sparse_bytes;
                                current_block = 1434579379687443766;
                            }
                        } else {
                            current_block = 1434579379687443766;
                        }
                        match current_block {
                            341217363119494045 => {}
                            _ =>
                            /*
                             * Read file contents.
                             */
                            {
                                if buffbytes > 0 as libc::c_int as libc::c_ulong {
                                    bytes = read(
                                        (*t).entry_fd,
                                        (*t).entry_buff as *mut libc::c_void,
                                        buffbytes,
                                    );
                                    if bytes < 0 as libc::c_int as libc::c_long {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            errno,
                                            b"Read error\x00" as *const u8 as *const libc::c_char,
                                        );
                                        r = ARCHIVE_FATAL;
                                        (*a).archive.state = ARCHIVE_STATE_FATAL;
                                        current_block = 341217363119494045;
                                    } else {
                                        current_block = 1724319918354933278;
                                    }
                                } else {
                                    bytes = 0 as libc::c_int as ssize_t;
                                    current_block = 1724319918354933278;
                                }
                                match current_block {
                                    341217363119494045 => {}
                                    _ =>
                                    /*
                                     * Return an EOF unless we've read a leading empty sparse region, which
                                     * is used to represent fully-sparse files.
                                    	*/
                                    {
                                        if bytes == 0 as libc::c_int as libc::c_long
                                            && empty_sparse_region == 0
                                        {
                                            /* Get EOF */
                                            (*t).entry_eof = 1 as libc::c_int;
                                            r = ARCHIVE_EOF
                                        } else {
                                            *buff = (*t).entry_buff as *const libc::c_void;
                                            *size = bytes as size_t;
                                            *offset = (*t).entry_total;
                                            (*t).entry_total += bytes;
                                            (*t).entry_remaining_bytes -= bytes;
                                            if (*t).entry_remaining_bytes
                                                == 0 as libc::c_int as libc::c_long
                                            {
                                                /* Close the current file descriptor */
                                                close_and_restore_time(
                                                    (*t).entry_fd,
                                                    t,
                                                    &mut (*t).restore_time,
                                                );
                                                (*t).entry_fd = -(1 as libc::c_int);
                                                (*t).entry_eof = 1 as libc::c_int
                                            }
                                            (*(*t).current_sparse).offset += bytes;
                                            (*(*t).current_sparse).length -= bytes;
                                            if (*(*t).current_sparse).length
                                                == 0 as libc::c_int as libc::c_long
                                                && (*t).entry_eof == 0
                                            {
                                                (*t).current_sparse = (*t).current_sparse.offset(1)
                                            }
                                            return 0 as libc::c_int;
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
    *buff = NULL as *const libc::c_void;
    *size = 0 as libc::c_int as size_t;
    *offset = (*t).entry_total;
    if (*t).entry_fd >= 0 as libc::c_int {
        /* Close the current file descriptor */
        close_and_restore_time((*t).entry_fd, t, &mut (*t).restore_time); /* info to use for this entry */
        (*t).entry_fd = -(1 as libc::c_int)
    } /* lstat() information */
    return r;
}
unsafe extern "C" fn next_entry(
    mut a: *mut archive_read_disk,
    mut t: *mut tree,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut st: *const stat = 0 as *const stat;
    let mut lst: *const stat = 0 as *const stat;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut delayed: libc::c_int = 0;
    let mut delayed_errno: libc::c_int = 0;
    let mut descend: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut delayed_str: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    delayed = ARCHIVE_OK;
    delayed_errno = 0 as libc::c_int;
    delayed_str.s = NULL as *mut libc::c_char;
    delayed_str.length = 0 as libc::c_int as size_t;
    delayed_str.buffer_length = 0 as libc::c_int as size_t;
    st = NULL as *const stat;
    lst = NULL as *const stat;
    (*t).descend = 0 as libc::c_int;
    loop {
        match tree_next(t) {
            -2 => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    (*t).tree_errno,
                    b"%s: Unable to continue traversing directory tree\x00" as *const u8
                        as *const libc::c_char,
                    tree_current_path(t),
                );
                (*a).archive.state = ARCHIVE_STATE_FATAL;
                tree_enter_initial_dir(t);
                return -(30 as libc::c_int);
            }
            -1 => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"%s: Couldn\'t visit directory\x00" as *const u8 as *const libc::c_char,
                    tree_current_path(t),
                );
                tree_enter_initial_dir(t);
                return -(25 as libc::c_int);
            }
            0 => {
                tree_enter_initial_dir(t);
                return 1 as libc::c_int;
            }
            TREE_REGULAR => {
                lst = tree_current_lstat(t);
                if lst.is_null() {
                    if errno == ENOENT && (*t).depth > 0 as libc::c_int {
                        delayed = ARCHIVE_WARN;
                        delayed_errno = errno;
                        if delayed_str.length == 0 as libc::c_int as libc::c_ulong {
                            archive_string_sprintf(
                                &mut delayed_str as *mut archive_string,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                tree_current_path(t),
                            );
                        } else {
                            archive_string_sprintf(
                                &mut delayed_str as *mut archive_string,
                                b" %s\x00" as *const u8 as *const libc::c_char,
                                tree_current_path(t),
                            );
                        }
                    } else {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            errno,
                            b"%s: Cannot stat\x00" as *const u8 as *const libc::c_char,
                            tree_current_path(t),
                        );
                        tree_enter_initial_dir(t);
                        return -(25 as libc::c_int);
                    }
                }
            }
            TREE_POSTDESCENT | TREE_POSTASCENT | _ => {}
        }
        if !lst.is_null() {
            break;
        }
    }
    archive_entry_copy_pathname(entry, tree_current_path(t));
    /*
     * Perform path matching.
     */
    if !(*a).matching.is_null() {
        r = archive_match_path_excluded((*a).matching, entry);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Failed : %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string((*a).matching),
            );
            return r;
        }
        if r != 0 {
            if (*a).excluded_cb_func.is_some() {
                (*a).excluded_cb_func.expect("non-null function pointer")(
                    &mut (*a).archive,
                    (*a).excluded_cb_data,
                    entry,
                );
            }
            return -(10 as libc::c_int);
        }
    }
    let mut current_block_59: u64;
    /*
     * Distinguish 'L'/'P'/'H' symlink following.
     */
    match (*t).symlink_mode as libc::c_int {
        72 => {
            /* 'H': After the first item, rest like 'P'. */
            (*t).symlink_mode = 'P' as i32 as libc::c_char;
            current_block_59 = 14681321039408416399;
        }
        76 => {
            current_block_59 = 14681321039408416399;
        }
        _ => {
            current_block_59 = 6861722712724833375;
        }
    }
    match current_block_59 {
        14681321039408416399 =>
        /* 'H': First item (from command line) like 'L'. */
        /* FALLTHROUGH */
        /* 'L': Do descend through a symlink to dir. */
        {
            descend = tree_current_is_dir(t);
            /* 'L': Follow symlinks to files. */
            (*a).symlink_mode = 'L' as i32 as libc::c_char;
            (*a).follow_symlinks = 1 as libc::c_int as libc::c_char;
            /* 'L': Archive symlinks as targets, if we can. */
            st = tree_current_stat(t);
            if !st.is_null() && tree_target_is_same_as_parent(t, st) == 0 {
                current_block_59 = 8835654301469918283;
            } else {
                current_block_59 = 6861722712724833375;
            }
        }
        _ => {}
    }
    match current_block_59 {
        6861722712724833375 =>
        /* If stat fails, we have a broken symlink;
         * in that case, don't follow the link. */
        /* FALLTHROUGH */
        /* 'P': Don't descend through a symlink to dir. */
        {
            descend = tree_current_is_physical_dir(t);
            /* 'P': Don't follow symlinks to files. */
            (*a).symlink_mode = 'P' as i32 as libc::c_char;
            (*a).follow_symlinks = 0 as libc::c_int as libc::c_char;
            /* 'P': Archive symlinks as symlinks. */
            st = lst
        }
        _ => {}
    }
    if update_current_filesystem(a, (*st).st_dev as int64_t) != ARCHIVE_OK {
        (*a).archive.state = ARCHIVE_STATE_FATAL;
        tree_enter_initial_dir(t);
        return -(30 as libc::c_int);
    }
    if (*t).initial_filesystem_id == -(1 as libc::c_int) {
        (*t).initial_filesystem_id = (*t).current_filesystem_id
    }
    if (*a).flags & ARCHIVE_READDISK_NO_TRAVERSE_MOUNTS != 0 {
        if (*t).initial_filesystem_id != (*t).current_filesystem_id {
            descend = 0 as libc::c_int
        }
    }
    (*t).descend = descend;
    /*
     * Honor nodump flag.
     * If the file is marked with nodump flag, do not return this entry.
     */
    if (*a).flags & ARCHIVE_READDISK_HONOR_NODUMP != 0 {
        if (*st).st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint
            || (*st).st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint
        {
            let mut stflags: libc::c_int = 0;
            (*t).entry_fd = open_on_current_dir(
                t,
                tree_current_access_path(t),
                O_RDONLY | O_NONBLOCK | O_CLOEXEC,
            );
            __archive_ensure_cloexec_flag((*t).entry_fd);
            if (*t).entry_fd >= 0 as libc::c_int {
                r = ioctl(
                    (*t).entry_fd,
                    FS_IOC_GETFLAGS,
                    &mut stflags as *mut libc::c_int,
                );
                if r == 0 as libc::c_int && stflags & FS_NODUMP_FL != 0 as libc::c_int {
                    return -(10 as libc::c_int);
                }
            }
        }
    }
    archive_entry_copy_stat(entry, st);
    /* Save the times to be restored. This must be in before
     * calling archive_read_disk_descend() or any chance of it,
     * especially, invoking a callback. */
    (*t).restore_time.mtime = archive_entry_mtime(entry);
    (*t).restore_time.mtime_nsec = archive_entry_mtime_nsec(entry);
    (*t).restore_time.atime = archive_entry_atime(entry);
    (*t).restore_time.atime_nsec = archive_entry_atime_nsec(entry);
    (*t).restore_time.filetype = archive_entry_filetype(entry);
    (*t).restore_time.noatime = (*(*t).current_filesystem).noatime;
    /*
     * Perform time matching.
     */
    if !(*a).matching.is_null() {
        r = archive_match_time_excluded((*a).matching, entry);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Failed : %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string((*a).matching),
            );
            return r;
        }
        if r != 0 {
            if (*a).excluded_cb_func.is_some() {
                (*a).excluded_cb_func.expect("non-null function pointer")(
                    &mut (*a).archive,
                    (*a).excluded_cb_data,
                    entry,
                );
            }
            return -(10 as libc::c_int);
        }
    }
    /* Lookup uname/gname */
    name = archive_read_disk_uname(&mut (*a).archive, archive_entry_uid(entry));
    if !name.is_null() {
        archive_entry_copy_uname(entry, name);
    }
    name = archive_read_disk_gname(&mut (*a).archive, archive_entry_gid(entry));
    if !name.is_null() {
        archive_entry_copy_gname(entry, name);
    }
    /*
     * Perform owner matching.
     */
    if !(*a).matching.is_null() {
        r = archive_match_owner_excluded((*a).matching, entry);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Failed : %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string((*a).matching),
            );
            return r;
        }
        if r != 0 {
            if (*a).excluded_cb_func.is_some() {
                (*a).excluded_cb_func.expect("non-null function pointer")(
                    &mut (*a).archive,
                    (*a).excluded_cb_data,
                    entry,
                );
            }
            return -(10 as libc::c_int);
        }
    }
    /*
     * Invoke a meta data filter callback.
     */
    if (*a).metadata_filter_func.is_some() {
        if (*a)
            .metadata_filter_func
            .expect("non-null function pointer")(
            &mut (*a).archive,
            (*a).metadata_filter_data,
            entry,
        ) == 0
        {
            return -(10 as libc::c_int);
        }
    }
    /*
     * Populate the archive_entry with metadata from the disk.
     */
    archive_entry_copy_sourcepath(entry, tree_current_access_path(t));
    r = archive_read_disk_entry_from_file(&mut (*a).archive, entry, (*t).entry_fd, st);
    if r == ARCHIVE_OK {
        r = delayed;
        if r != ARCHIVE_OK {
            archive_string_sprintf(
                &mut delayed_str as *mut archive_string,
                b": %s\x00" as *const u8 as *const libc::c_char,
                b"File removed before we read it\x00" as *const u8 as *const libc::c_char,
            );
            archive_set_error(
                &mut (*a).archive as *mut archive,
                delayed_errno,
                b"%s\x00" as *const u8 as *const libc::c_char,
                delayed_str.s,
            );
        }
    }
    archive_string_free(&mut delayed_str);
    return r;
}
unsafe extern "C" fn _archive_read_next_header(
    mut _a: *mut archive,
    mut entryp: *mut *mut archive_entry,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    *entryp = NULL as *mut archive_entry;
    ret = _archive_read_next_header2(_a, (*a).entry);
    *entryp = (*a).entry;
    return ret;
}
unsafe extern "C" fn _archive_read_next_header2(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut t: *mut tree = 0 as *mut tree;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_read_next_header2\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    t = (*a).tree;
    if (*t).entry_fd >= 0 as libc::c_int {
        close_and_restore_time((*t).entry_fd, t, &mut (*t).restore_time);
        (*t).entry_fd = -(1 as libc::c_int)
    }
    archive_entry_clear(entry);
    loop {
        r = next_entry(a, t, entry);
        if (*t).entry_fd >= 0 as libc::c_int {
            close((*t).entry_fd);
            (*t).entry_fd = -(1 as libc::c_int)
        }
        if !(r == ARCHIVE_RETRY) {
            break;
        }
        archive_entry_clear(entry);
    }
    /* Return to the initial directory. */
    tree_enter_initial_dir(t);
    /*
     * EOF and FATAL are persistent at this layer.  By
     * modifying the state, we guarantee that future calls to
     * read a header or read data will fail.
     */
    match r {
        ARCHIVE_EOF => (*a).archive.state = ARCHIVE_STATE_EOF,
        ARCHIVE_OK | -20 => {
            /* Overwrite the sourcepath based on the initial directory. */
            archive_entry_copy_sourcepath(entry, tree_current_path(t));
            (*t).entry_total = 0 as libc::c_int as int64_t;
            if archive_entry_filetype(entry) == AE_IFREG as mode_t {
                (*t).nlink = archive_entry_nlink(entry) as libc::c_int;
                (*t).entry_remaining_bytes = archive_entry_size(entry);
                (*t).entry_eof = if (*t).entry_remaining_bytes == 0 as libc::c_int as libc::c_long {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                if (*t).entry_eof == 0 && setup_sparse(a, entry) != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
            } else {
                (*t).entry_remaining_bytes = 0 as libc::c_int as int64_t;
                (*t).entry_eof = 1 as libc::c_int
            }
            (*a).archive.state = ARCHIVE_STATE_DATA
        }
        -30 => (*a).archive.state = ARCHIVE_STATE_FATAL,
        -10 | _ => {}
    }
    __archive_reset_read_data(&mut (*a).archive);
    return r;
}
unsafe extern "C" fn setup_sparse(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut t: *mut tree = (*a).tree;
    let mut length: int64_t = 0;
    let mut offset: int64_t = 0;
    let mut i: libc::c_int = 0;
    (*t).sparse_count = archive_entry_sparse_reset(entry);
    if (*t).sparse_count + 1 as libc::c_int > (*t).sparse_list_size {
        free((*t).sparse_list as *mut libc::c_void);
        (*t).sparse_list_size = (*t).sparse_count + 1 as libc::c_int;
        (*t).sparse_list = malloc(
            (::std::mem::size_of::<entry_sparse>() as libc::c_ulong)
                .wrapping_mul((*t).sparse_list_size as libc::c_ulong),
        ) as *mut entry_sparse;
        if (*t).sparse_list.is_null() {
            (*t).sparse_list_size = 0 as libc::c_int;
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate data\x00" as *const u8 as *const libc::c_char,
            );
            (*a).archive.state = ARCHIVE_STATE_FATAL;
            return -(30 as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    while i < (*t).sparse_count {
        archive_entry_sparse_next(entry, &mut offset, &mut length);
        (*(*t).sparse_list.offset(i as isize)).offset = offset;
        (*(*t).sparse_list.offset(i as isize)).length = length;
        i += 1
    }
    if i == 0 as libc::c_int {
        (*(*t).sparse_list.offset(i as isize)).offset = 0 as libc::c_int as int64_t;
        (*(*t).sparse_list.offset(i as isize)).length = archive_entry_size(entry)
    } else {
        (*(*t).sparse_list.offset(i as isize)).offset = archive_entry_size(entry);
        (*(*t).sparse_list.offset(i as isize)).length = 0 as libc::c_int as int64_t
    }
    (*t).current_sparse = (*t).sparse_list;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_matching(
    mut _a: *mut archive,
    mut _ma: *mut archive,
    mut _excluded_func: Option<
        unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void, _: *mut archive_entry) -> (),
    >,
    mut _client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_set_matching\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).matching = _ma;
    (*a).excluded_cb_func = _excluded_func;
    (*a).excluded_cb_data = _client_data;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_metadata_filter_callback(
    mut _a: *mut archive,
    mut _metadata_filter_func: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut libc::c_void,
            _: *mut archive_entry,
        ) -> libc::c_int,
    >,
    mut _client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_set_metadata_filter_callback\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).metadata_filter_func = _metadata_filter_func;
    (*a).metadata_filter_data = _client_data;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_can_descend(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut t: *mut tree = (*a).tree;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_read_disk_can_descend\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return ((*t).visit_type == TREE_REGULAR && (*t).descend != 0) as libc::c_int;
}
/*
 * Called by the client to mark the directory just returned from
 * tree_next() as needing to be visited.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_descend(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut t: *mut tree = (*a).tree;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_read_disk_descend\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*t).visit_type != TREE_REGULAR || (*t).descend == 0 {
        return 0 as libc::c_int;
    }
    /*
     * We must not treat the initial specified path as a physical dir,
     * because if we do then we will try and ascend out of it by opening
     * ".." which is (a) wrong and (b) causes spurious permissions errors
     * if ".." is not readable by us. Instead, treat it as if it were a
     * symlink. (This uses an extra fd, but it can only happen once at the
     * top level of a traverse.) But we can't necessarily assume t->st is
     * valid here (though t->lst is), which complicates the logic a
     * little.
     */
    if tree_current_is_physical_dir(t) != 0 {
        tree_push(
            t,
            (*t).basename,
            (*t).current_filesystem_id,
            (*t).lst.st_dev as int64_t,
            (*t).lst.st_ino as int64_t,
            &mut (*t).restore_time,
        );
        if !(*(*(*t).stack).parent).parent.is_null() {
            (*(*t).stack).flags |= isDir
        } else {
            (*(*t).stack).flags |= isDirLink
        }
    } else if tree_current_is_dir(t) != 0 {
        tree_push(
            t,
            (*t).basename,
            (*t).current_filesystem_id,
            (*t).st.st_dev as int64_t,
            (*t).st.st_ino as int64_t,
            &mut (*t).restore_time,
        );
        (*(*t).stack).flags |= isDirLink
    }
    (*t).descend = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_open(
    mut _a: *mut archive,
    mut pathname: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        1 as libc::c_uint | 0x20 as libc::c_uint,
        b"archive_read_disk_open\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_clear_error(&mut (*a).archive);
    return _archive_read_disk_open(_a, pathname);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_open_w(
    mut _a: *mut archive,
    mut pathname: *const wchar_t,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut path: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut ret: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        1 as libc::c_uint | 0x20 as libc::c_uint,
        b"archive_read_disk_open_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_clear_error(&mut (*a).archive);
    /* Make a char string from a wchar_t string. */
    path.s = NULL as *mut libc::c_char;
    path.length = 0 as libc::c_int as size_t;
    path.buffer_length = 0 as libc::c_int as size_t;
    if archive_string_append_from_wcs(&mut path, pathname, wcslen(pathname)) != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Can\'t convert a path to a char string\x00" as *const u8 as *const libc::c_char,
            );
        }
        (*a).archive.state = ARCHIVE_STATE_FATAL;
        ret = ARCHIVE_FATAL
    } else {
        ret = _archive_read_disk_open(_a, path.s)
    }
    archive_string_free(&mut path);
    return ret;
}
unsafe extern "C" fn _archive_read_disk_open(
    mut _a: *mut archive,
    mut pathname: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    if !(*a).tree.is_null() {
        (*a).tree = tree_reopen(
            (*a).tree,
            pathname,
            (*a).flags & ARCHIVE_READDISK_RESTORE_ATIME,
        )
    } else {
        (*a).tree = tree_open(
            pathname,
            (*a).symlink_mode as libc::c_int,
            (*a).flags & ARCHIVE_READDISK_RESTORE_ATIME,
        )
    }
    if (*a).tree.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate tar data\x00" as *const u8 as *const libc::c_char,
        );
        (*a).archive.state = ARCHIVE_STATE_FATAL;
        return -(30 as libc::c_int);
    }
    (*a).archive.state = ARCHIVE_STATE_HEADER;
    return 0 as libc::c_int;
}
/*
 * Return a current filesystem ID which is index of the filesystem entry
 * you've visited through archive_read_disk.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_current_filesystem(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_read_disk_current_filesystem\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return (*(*a).tree).current_filesystem_id;
}
unsafe extern "C" fn update_current_filesystem(
    mut a: *mut archive_read_disk,
    mut dev: int64_t,
) -> libc::c_int {
    let mut t: *mut tree = (*a).tree;
    let mut i: libc::c_int = 0;
    let mut fid: libc::c_int = 0;
    if !(*t).current_filesystem.is_null() && (*(*t).current_filesystem).dev == dev {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*t).max_filesystem_id {
        if (*(*t).filesystem_table.offset(i as isize)).dev == dev {
            /* There is the filesystem ID we've already generated. */
            (*t).current_filesystem_id = i;
            (*t).current_filesystem =
                &mut *(*t).filesystem_table.offset(i as isize) as *mut filesystem;
            return 0 as libc::c_int;
        }
        i += 1
    }
    /*
     * This is the new filesystem which we have to generate a new ID for.
     */
    let fresh0 = (*t).max_filesystem_id;
    (*t).max_filesystem_id = (*t).max_filesystem_id + 1;
    fid = fresh0;
    if (*t).max_filesystem_id > (*t).allocated_filesystem {
        let mut s: size_t = 0;
        let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
        s = ((*t).max_filesystem_id * 2 as libc::c_int) as size_t;
        p = realloc(
            (*t).filesystem_table as *mut libc::c_void,
            s.wrapping_mul(::std::mem::size_of::<filesystem>() as libc::c_ulong),
        );
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate tar data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*t).filesystem_table = p as *mut filesystem;
        (*t).allocated_filesystem = s as libc::c_int
    }
    (*t).current_filesystem_id = fid;
    (*t).current_filesystem = &mut *(*t).filesystem_table.offset(fid as isize) as *mut filesystem;
    (*(*t).current_filesystem).dev = dev;
    (*(*t).current_filesystem).allocation_ptr = NULL as *mut libc::c_uchar;
    (*(*t).current_filesystem).buff = NULL as *mut libc::c_uchar;
    /* Setup the current filesystem properties which depend on
     * platform specific. */
    return setup_current_filesystem(a);
}
/*
 * Returns 1 if current filesystem is generated filesystem, 0 if it is not
 * or -1 if it is unknown.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_current_filesystem_is_synthetic(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_read_disk_current_filesystem\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return (*(*(*a).tree).current_filesystem).synthetic;
}
/*
 * Returns 1 if current filesystem is remote filesystem, 0 if it is not
 * or -1 if it is unknown.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_current_filesystem_is_remote(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_read_disk_current_filesystem\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return (*(*(*a).tree).current_filesystem).remote;
}
unsafe extern "C" fn get_xfer_size(
    mut t: *mut tree,
    mut fd: libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    (*(*t).current_filesystem).xfer_align = -(1 as libc::c_int) as libc::c_long;
    errno = 0 as libc::c_int;
    if fd >= 0 as libc::c_int {
        (*(*t).current_filesystem).incr_xfer_size = fpathconf(fd, _PC_REC_INCR_XFER_SIZE_0);
        (*(*t).current_filesystem).max_xfer_size = fpathconf(fd, _PC_REC_MAX_XFER_SIZE_0);
        (*(*t).current_filesystem).min_xfer_size = fpathconf(fd, _PC_REC_MIN_XFER_SIZE_0);
        (*(*t).current_filesystem).xfer_align = fpathconf(fd, _PC_REC_XFER_ALIGN_0)
    } else if !path.is_null() {
        (*(*t).current_filesystem).incr_xfer_size = pathconf(path, _PC_REC_INCR_XFER_SIZE_0);
        (*(*t).current_filesystem).max_xfer_size = pathconf(path, _PC_REC_MAX_XFER_SIZE_0);
        (*(*t).current_filesystem).min_xfer_size = pathconf(path, _PC_REC_MIN_XFER_SIZE_0);
        (*(*t).current_filesystem).xfer_align = pathconf(path, _PC_REC_XFER_ALIGN_0)
    }
    /* At least we need an alignment size. */
    if (*(*t).current_filesystem).xfer_align == -(1 as libc::c_int) as libc::c_long {
        return if errno == EINVAL {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    } else {
        return 0 as libc::c_int;
    };
}
/*
 * Note: statfs is deprecated since LSB 3.2
 */
pub const CIFS_SUPER_MAGIC: libc::c_uint = 0xff534d42 as libc::c_uint;
pub const DEVFS_SUPER_MAGIC: libc::c_int = 0x1373 as libc::c_int;
/*
 * Gather current filesystem properties on Linux
 */
unsafe extern "C" fn setup_current_filesystem(mut a: *mut archive_read_disk) -> libc::c_int {
    let mut t: *mut tree = (*a).tree;
    let mut sfs: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    let mut svfs: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        __f_spare: [0; 6],
    };
    let mut r: libc::c_int = 0;
    let mut vr: libc::c_int = 0 as libc::c_int;
    let mut xr: libc::c_int = 0 as libc::c_int;
    if tree_current_is_symblic_link_target(t) != 0 {
        /*
         * Get file system statistics on any directory
         * where current is.
         */
        let mut fd: libc::c_int = openat(
            tree_current_dir_fd(t),
            tree_current_access_path(t),
            O_RDONLY | O_CLOEXEC,
        ); /* for f_flag, mount flags */
        __archive_ensure_cloexec_flag(fd);
        if fd < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"openat failed\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        vr = fstatvfs(fd, &mut svfs);
        r = fstatfs(fd, &mut sfs);
        if r == 0 as libc::c_int {
            xr = get_xfer_size(t, fd, NULL as *const libc::c_char)
        }
        close(fd);
    } else {
        vr = fstatvfs(tree_current_dir_fd(t), &mut svfs);
        r = fstatfs(tree_current_dir_fd(t), &mut sfs);
        if r == 0 as libc::c_int {
            xr = get_xfer_size(t, tree_current_dir_fd(t), NULL as *const libc::c_char)
        }
    }
    if r == -(1 as libc::c_int) || xr == -(1 as libc::c_int) || vr == -(1 as libc::c_int) {
        (*(*t).current_filesystem).synthetic = -(1 as libc::c_int);
        (*(*t).current_filesystem).remote = -(1 as libc::c_int);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"statfs failed\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    } else {
        if xr == 1 as libc::c_int {
            /* pathconf(_PC_REX_*) operations are not supported. */
            (*(*t).current_filesystem).xfer_align = svfs.f_frsize as libc::c_long;
            (*(*t).current_filesystem).max_xfer_size = -(1 as libc::c_int) as libc::c_long;
            (*(*t).current_filesystem).min_xfer_size = svfs.f_bsize as libc::c_long;
            (*(*t).current_filesystem).incr_xfer_size = svfs.f_bsize as libc::c_long
        }
    }
    match sfs.f_type {
        1397113167 | 4283649346 | 1937076805 | 22092 | 26985 | 20859 => {
            /* NetWare */
            (*(*t).current_filesystem).remote = 1 as libc::c_int;
            (*(*t).current_filesystem).synthetic = 0 as libc::c_int
        }
        4979 | 40864 | 40866 => {
            (*(*t).current_filesystem).remote = 0 as libc::c_int;
            (*(*t).current_filesystem).synthetic = 1 as libc::c_int
        }
        _ => {
            (*(*t).current_filesystem).remote = 0 as libc::c_int;
            (*(*t).current_filesystem).synthetic = 0 as libc::c_int
        }
    }
    if svfs.f_flag & ST_NOATIME_0 as libc::c_ulong != 0 {
        (*(*t).current_filesystem).noatime = 1 as libc::c_int
    } else {
        (*(*t).current_filesystem).noatime = 0 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn close_and_restore_time(
    mut fd: libc::c_int,
    mut t: *mut tree,
    mut rt: *mut restore_time,
) -> libc::c_int {
    let mut timespecs: [timespec; 2] = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 2];
    let mut times: [timeval; 2] = [timeval {
        tv_sec: 0,
        tv_usec: 0,
    }; 2];
    if (*t).flags & needsRestoreTimes == 0 as libc::c_int || (*rt).noatime != 0 {
        if fd >= 0 as libc::c_int {
            return close(fd);
        } else {
            return 0 as libc::c_int;
        }
    }
    timespecs[1 as libc::c_int as usize].tv_sec = (*rt).mtime;
    timespecs[1 as libc::c_int as usize].tv_nsec = (*rt).mtime_nsec;
    timespecs[0 as libc::c_int as usize].tv_sec = (*rt).atime;
    timespecs[0 as libc::c_int as usize].tv_nsec = (*rt).atime_nsec;
    /* futimens() is defined in POSIX.1-2008. */
    if futimens(fd, timespecs.as_mut_ptr() as *const timespec) == 0 as libc::c_int {
        return close(fd);
    }
    times[1 as libc::c_int as usize].tv_sec = (*rt).mtime;
    times[1 as libc::c_int as usize].tv_usec =
        (*rt).mtime_nsec / 1000 as libc::c_int as libc::c_long;
    times[0 as libc::c_int as usize].tv_sec = (*rt).atime;
    times[0 as libc::c_int as usize].tv_usec =
        (*rt).atime_nsec / 1000 as libc::c_int as libc::c_long;
    close(fd);
    if futimesat(
        tree_current_dir_fd(t),
        (*rt).name,
        times.as_mut_ptr() as *const timeval,
    ) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if lutimes((*rt).name, times.as_mut_ptr() as *const timeval) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn open_on_current_dir(
    mut t: *mut tree,
    mut path: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    return openat(tree_current_dir_fd(t), path, flags);
}
unsafe extern "C" fn tree_dup(mut fd: libc::c_int) -> libc::c_int {
    let mut new_fd: libc::c_int = 0;
    static mut can_dupfd_cloexec: libc::c_int = 1 as libc::c_int;
    if can_dupfd_cloexec != 0 {
        new_fd = fcntl(fd, F_DUPFD_CLOEXEC, 0 as libc::c_int);
        if new_fd != -(1 as libc::c_int) {
            return new_fd;
        }
        /* Linux 2.6.18 - 2.6.23 declare F_DUPFD_CLOEXEC,
         * but it cannot be used. So we have to try dup(). */
        /* We won't try F_DUPFD_CLOEXEC. */
        ::std::ptr::write_volatile(&mut can_dupfd_cloexec as *mut libc::c_int, 0 as libc::c_int)
    }
    /* F_DUPFD_CLOEXEC */
    new_fd = dup(fd);
    __archive_ensure_cloexec_flag(new_fd);
    return new_fd;
}
/*
 * Add a directory path to the current stack.
 */
unsafe extern "C" fn tree_push(
    mut t: *mut tree,
    mut path: *const libc::c_char,
    mut filesystem_id: libc::c_int,
    mut dev: int64_t,
    mut ino: int64_t,
    mut rt: *mut restore_time,
) {
    let mut te: *mut tree_entry = 0 as *mut tree_entry;
    te = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<tree_entry>() as libc::c_ulong,
    ) as *mut tree_entry;
    (*te).next = (*t).stack;
    (*te).parent = (*t).current;
    if !(*te).parent.is_null() {
        (*te).depth = (*(*te).parent).depth + 1 as libc::c_int
    }
    (*t).stack = te;
    (*te).name.s = NULL as *mut libc::c_char;
    (*te).name.length = 0 as libc::c_int as size_t;
    (*te).name.buffer_length = 0 as libc::c_int as size_t;
    (*te).symlink_parent_fd = -(1 as libc::c_int);
    (*te).name.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*te).name,
        path as *const libc::c_void,
        (if path.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(path)
        }),
    );
    (*te).flags = needsDescent | needsOpen | needsAscent;
    (*te).filesystem_id = filesystem_id;
    (*te).dev = dev;
    (*te).ino = ino;
    (*te).dirname_length = (*t).dirname_length;
    (*te).restore_time.name = (*te).name.s;
    if !rt.is_null() {
        (*te).restore_time.mtime = (*rt).mtime;
        (*te).restore_time.mtime_nsec = (*rt).mtime_nsec;
        (*te).restore_time.atime = (*rt).atime;
        (*te).restore_time.atime_nsec = (*rt).atime_nsec;
        (*te).restore_time.filetype = (*rt).filetype;
        (*te).restore_time.noatime = (*rt).noatime
    };
}
/*
 * Append a name to the current dir path.
 */
unsafe extern "C" fn tree_append(
    mut t: *mut tree,
    mut name: *const libc::c_char,
    mut name_length: size_t,
) {
    let mut size_needed: size_t = 0;
    *(*t).path.s.offset((*t).dirname_length as isize) = '\u{0}' as i32 as libc::c_char;
    (*t).path.length = (*t).dirname_length;
    /* Strip trailing '/' from name, unless entire name is "/". */
    while name_length > 1 as libc::c_int as libc::c_ulong
        && *name.offset(name_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '/' as i32
    {
        name_length = name_length.wrapping_sub(1)
    }
    /* Resize pathname buffer as needed. */
    size_needed = name_length
        .wrapping_add((*t).dirname_length)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    archive_string_ensure(&mut (*t).path, size_needed);
    /* Add a separating '/' if it's needed. */
    if (*t).dirname_length > 0 as libc::c_int as libc::c_ulong
        && *(*t).path.s.offset(
            (*t).path
                .length
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int
            != '/' as i32
    {
        archive_strappend_char(&mut (*t).path, '/' as i32 as libc::c_char);
    }
    (*t).basename = (*t).path.s.offset((*t).path.length as isize);
    archive_strncat(&mut (*t).path, name as *const libc::c_void, name_length);
    (*t).restore_time.name = (*t).basename;
}
/* Initiate/terminate a tree traversal. */
/*
 * Open a directory tree for traversal.
 */
unsafe extern "C" fn tree_open(
    mut path: *const libc::c_char,
    mut symlink_mode: libc::c_int,
    mut restore_time: libc::c_int,
) -> *mut tree {
    let mut t: *mut tree = 0 as *mut tree;
    t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<tree>() as libc::c_ulong,
    ) as *mut tree;
    if t.is_null() {
        return 0 as *mut tree;
    }
    (*t).path.s = NULL as *mut libc::c_char;
    (*t).path.length = 0 as libc::c_int as size_t;
    (*t).path.buffer_length = 0 as libc::c_int as size_t;
    archive_string_ensure(&mut (*t).path, 31 as libc::c_int as size_t);
    (*t).initial_symlink_mode = symlink_mode as libc::c_char;
    return tree_reopen(t, path, restore_time);
}
unsafe extern "C" fn tree_reopen(
    mut t: *mut tree,
    mut path: *const libc::c_char,
    mut restore_time: libc::c_int,
) -> *mut tree {
    /* Linux */
    let o_flag: libc::c_int = O_PATH;
    (*t).flags = if restore_time != 0 as libc::c_int {
        needsRestoreTimes
    } else {
        0 as libc::c_int
    };
    (*t).flags |= onInitialDir;
    (*t).visit_type = 0 as libc::c_int;
    (*t).tree_errno = 0 as libc::c_int;
    (*t).dirname_length = 0 as libc::c_int as size_t;
    (*t).depth = 0 as libc::c_int;
    (*t).descend = 0 as libc::c_int;
    (*t).current = NULL as *mut tree_entry;
    (*t).d = INVALID_DIR_HANDLE as *mut DIR;
    (*t).symlink_mode = (*t).initial_symlink_mode;
    (*t).path.length = 0 as libc::c_int as size_t;
    (*t).entry_fd = -(1 as libc::c_int);
    (*t).entry_eof = 0 as libc::c_int;
    (*t).entry_remaining_bytes = 0 as libc::c_int as int64_t;
    (*t).initial_filesystem_id = -(1 as libc::c_int);
    /* First item is set up a lot like a symlink traversal. */
    tree_push(
        t,
        path,
        0 as libc::c_int,
        0 as libc::c_int as int64_t,
        0 as libc::c_int as int64_t,
        NULL as *mut restore_time,
    );
    (*(*t).stack).flags = needsFirstVisit;
    (*t).openCount = 1 as libc::c_int;
    (*t).maxOpenCount = (*t).openCount;
    (*t).initial_dir_fd = open(
        b".\x00" as *const u8 as *const libc::c_char,
        O_RDONLY | O_CLOEXEC,
    );
    /*
     * Most likely reason to fail opening "." is that it's not readable,
     * so try again for execute. The consequences of not opening this are
     * unhelpful and unnecessary errors later.
     */
    if (*t).initial_dir_fd < 0 as libc::c_int {
        (*t).initial_dir_fd = open(
            b".\x00" as *const u8 as *const libc::c_char,
            o_flag | O_CLOEXEC,
        )
    }
    __archive_ensure_cloexec_flag((*t).initial_dir_fd);
    (*t).working_dir_fd = tree_dup((*t).initial_dir_fd);
    return t;
}
unsafe extern "C" fn tree_descent(mut t: *mut tree) -> libc::c_int {
    let mut flag: libc::c_int = 0;
    let mut new_fd: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    (*t).dirname_length = (*t).path.length;
    flag = O_RDONLY | O_CLOEXEC;
    flag |= O_DIRECTORY;
    new_fd = open_on_current_dir(t, (*(*t).stack).name.s, flag);
    __archive_ensure_cloexec_flag(new_fd);
    if new_fd < 0 as libc::c_int {
        (*t).tree_errno = errno;
        r = TREE_ERROR_DIR
    } else {
        (*t).depth += 1;
        /* If it is a link, set up fd for the ascent. */
        if (*(*t).stack).flags & isDirLink != 0 {
            (*(*t).stack).symlink_parent_fd = (*t).working_dir_fd;
            (*t).openCount += 1;
            if (*t).openCount > (*t).maxOpenCount {
                (*t).maxOpenCount = (*t).openCount
            }
        } else {
            close((*t).working_dir_fd);
        }
        /* Renew the current working directory. */
        (*t).working_dir_fd = new_fd;
        (*t).flags &= !onWorkingDir
    }
    return r;
}
/*
 * We've finished a directory; ascend back to the parent.
 */
unsafe extern "C" fn tree_ascend(mut t: *mut tree) -> libc::c_int {
    let mut te: *mut tree_entry = 0 as *mut tree_entry;
    let mut new_fd: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut prev_dir_fd: libc::c_int = 0;
    te = (*t).stack;
    prev_dir_fd = (*t).working_dir_fd;
    if (*te).flags & isDirLink != 0 {
        new_fd = (*te).symlink_parent_fd
    } else {
        new_fd = open_on_current_dir(
            t,
            b"..\x00" as *const u8 as *const libc::c_char,
            O_RDONLY | O_CLOEXEC,
        );
        __archive_ensure_cloexec_flag(new_fd);
    }
    if new_fd < 0 as libc::c_int {
        (*t).tree_errno = errno;
        r = TREE_ERROR_FATAL
    } else {
        /* Renew the current working directory. */
        (*t).working_dir_fd = new_fd;
        (*t).flags &= !onWorkingDir;
        /* Current directory has been changed, we should
         * close an fd of previous working directory. */
        close_and_restore_time(prev_dir_fd, t, &mut (*te).restore_time);
        if (*te).flags & isDirLink != 0 {
            (*t).openCount -= 1;
            (*te).symlink_parent_fd = -(1 as libc::c_int)
        }
        (*t).depth -= 1
    }
    return r;
}
/*
 * Return to the initial directory where tree_open() was performed.
 */
unsafe extern "C" fn tree_enter_initial_dir(mut t: *mut tree) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    if (*t).flags & onInitialDir == 0 as libc::c_int {
        r = fchdir((*t).initial_dir_fd);
        if r == 0 as libc::c_int {
            (*t).flags &= !onWorkingDir;
            (*t).flags |= onInitialDir
        }
    }
    return r;
}
/*
 * Restore working directory of directory traversals.
 */
unsafe extern "C" fn tree_enter_working_dir(mut t: *mut tree) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    /*
     * Change the current directory if really needed.
     * Sometimes this is unneeded when we did not do
     * descent.
     */
    if (*t).depth > 0 as libc::c_int && (*t).flags & onWorkingDir == 0 as libc::c_int {
        r = fchdir((*t).working_dir_fd);
        if r == 0 as libc::c_int {
            (*t).flags &= !onInitialDir;
            (*t).flags |= onWorkingDir
        }
    }
    return r;
}
unsafe extern "C" fn tree_current_dir_fd(mut t: *mut tree) -> libc::c_int {
    return (*t).working_dir_fd;
}
/*
 * Pop the working stack.
 */
unsafe extern "C" fn tree_pop(mut t: *mut tree) {
    let mut te: *mut tree_entry = 0 as *mut tree_entry;
    *(*t).path.s.offset((*t).dirname_length as isize) = '\u{0}' as i32 as libc::c_char;
    (*t).path.length = (*t).dirname_length;
    if (*t).stack == (*t).current && !(*t).current.is_null() {
        (*t).current = (*(*t).current).parent
    }
    te = (*t).stack;
    (*t).stack = (*te).next;
    (*t).dirname_length = (*te).dirname_length;
    (*t).basename = (*t).path.s.offset((*t).dirname_length as isize);
    while *(*t).basename.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        (*t).basename = (*t).basename.offset(1)
    }
    archive_string_free(&mut (*te).name);
    free(te as *mut libc::c_void);
}
/*
 * Get the next item in the tree traversal.
 */
unsafe extern "C" fn tree_next(mut t: *mut tree) -> libc::c_int {
    let mut r: libc::c_int = 0;
    while !(*t).stack.is_null() {
        /* If there's an open dir, get the next entry from there. */
        if !(*t).d.is_null() {
            r = tree_dir_next_posix(t);
            if r == 0 as libc::c_int {
                continue;
            }
            return r;
        } else if (*(*t).stack).flags & needsFirstVisit != 0 {
            /* Top stack item needs a regular visit. */
            (*t).current = (*t).stack;
            tree_append(t, (*(*t).stack).name.s, (*(*t).stack).name.length);
            /* t->dirname_length = t->path_length; */
            /* tree_pop(t); */
            (*(*t).stack).flags &= !needsFirstVisit;
            (*t).visit_type = TREE_REGULAR;
            return (*t).visit_type;
        } else if (*(*t).stack).flags & needsDescent != 0 {
            /* Top stack item is dir to descend into. */
            (*t).current = (*t).stack;
            tree_append(t, (*(*t).stack).name.s, (*(*t).stack).name.length);
            (*(*t).stack).flags &= !needsDescent;
            r = tree_descent(t);
            if r != 0 as libc::c_int {
                tree_pop(t);
                (*t).visit_type = r
            } else {
                (*t).visit_type = TREE_POSTDESCENT
            }
            return (*t).visit_type;
        } else if (*(*t).stack).flags & needsOpen != 0 {
            (*(*t).stack).flags &= !needsOpen;
            r = tree_dir_next_posix(t);
            if r == 0 as libc::c_int {
                continue;
            }
            return r;
        } else if (*(*t).stack).flags & needsAscent != 0 {
            /* Top stack item is dir and we're done with it. */
            r = tree_ascend(t);
            tree_pop(t);
            (*t).visit_type = if r != 0 as libc::c_int {
                r
            } else {
                TREE_POSTASCENT
            };
            return (*t).visit_type;
        } else {
            /* Top item on stack is dead. */
            tree_pop(t);
            (*t).flags &= !hasLstat;
            (*t).flags &= !hasStat
        }
    }
    (*t).visit_type = 0 as libc::c_int;
    return (*t).visit_type;
}
/* We are on the initial dir. */
unsafe extern "C" fn tree_dir_next_posix(mut t: *mut tree) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut namelen: size_t = 0;
    if (*t).d.is_null() {
        (*t).d = fdopendir(tree_dup((*t).working_dir_fd));
        /* USE_READDIR_R */
        if (*t).d.is_null() {
            /* HAVE_FDOPENDIR */
            /* HAVE_FDOPENDIR */
            r = tree_ascend(t); /* Undo "chdir" */
            tree_pop(t);
            (*t).tree_errno = errno;
            (*t).visit_type = if r != 0 as libc::c_int {
                r
            } else {
                TREE_ERROR_DIR
            };
            return (*t).visit_type;
        }
    }
    loop {
        errno = 0 as libc::c_int;
        (*t).de = readdir((*t).d);
        if (*t).de.is_null() {
            r = errno;
            closedir((*t).d);
            (*t).d = INVALID_DIR_HANDLE as *mut DIR;
            if r != 0 as libc::c_int {
                (*t).tree_errno = r;
                (*t).visit_type = TREE_ERROR_DIR;
                return (*t).visit_type;
            } else {
                return 0 as libc::c_int;
            }
        }
        name = (*(*t).de).d_name.as_mut_ptr();
        namelen = strlen((*(*t).de).d_name.as_mut_ptr());
        (*t).flags &= !hasLstat;
        (*t).flags &= !hasStat;
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            continue;
        }
        if *name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *name.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            continue;
        }
        tree_append(t, name, namelen);
        (*t).visit_type = TREE_REGULAR;
        return (*t).visit_type;
    }
}
/*
 * Request the lstat() or stat() data for the current path.  Since the
 * tree package needs to do some of this anyway, and caches the
 * results, you should take advantage of it here if you need it rather
 * than make a redundant stat() or lstat() call of your own.
 */
/*
 * Get the stat() data for the entry just returned from tree_next().
 */
unsafe extern "C" fn tree_current_stat(mut t: *mut tree) -> *const stat {
    if (*t).flags & hasStat == 0 {
        if fstatat(
            tree_current_dir_fd(t),
            tree_current_access_path(t),
            &mut (*t).st,
            0 as libc::c_int,
        ) != 0 as libc::c_int
        {
            return NULL as *const stat;
        }
        (*t).flags |= hasStat
    }
    return &mut (*t).st;
}
/*
 * Get the lstat() data for the entry just returned from tree_next().
 */
unsafe extern "C" fn tree_current_lstat(mut t: *mut tree) -> *const stat {
    if (*t).flags & hasLstat == 0 {
        if fstatat(
            tree_current_dir_fd(t),
            tree_current_access_path(t),
            &mut (*t).lst,
            AT_SYMLINK_NOFOLLOW,
        ) != 0 as libc::c_int
        {
            return NULL as *const stat;
        }
        (*t).flags |= hasLstat
    }
    return &mut (*t).lst;
}
/* "is_dir" is equivalent to S_ISDIR(tree_current_stat()->st_mode) */
/*
 * Test whether current entry is a dir or link to a dir.
 */
unsafe extern "C" fn tree_current_is_dir(mut t: *mut tree) -> libc::c_int {
    let mut st: *const stat = 0 as *const stat;
    /*
     * If we already have lstat() info, then try some
     * cheap tests to determine if this is a dir.
     */
    if (*t).flags & hasLstat != 0 {
        /* If lstat() says it's a dir, it must be a dir. */
        st = tree_current_lstat(t);
        if st.is_null() {
            return 0 as libc::c_int;
        }
        if (*st).st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        }
        /*
         * It's a link, but we don't know what it's a link to,
         * so we'll have to use stat().
         */
        if !((*st).st_mode & __S_IFMT as libc::c_uint == 0o120000 as libc::c_int as libc::c_uint) {
            return 0 as libc::c_int;
        }
    }
    st = tree_current_stat(t);
    /* Not a dir; might be a link to a dir. */
    /* If it's not a link, then it's not a link to a dir. */
    /* If we can't stat it, it's not a dir. */
    if st.is_null() {
        return 0 as libc::c_int;
    }
    /* Use the definitive test.  Hopefully this is cached. */
    return ((*st).st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
/* The following functions use tricks to avoid a certain number of
 * stat()/lstat() calls. */
/* "is_physical_dir" is equivalent to S_ISDIR(tree_current_lstat()->st_mode) */
/*
 * Test whether current entry is a physical directory.  Usually, we
 * already have at least one of stat() or lstat() in memory, so we
 * use tricks to try to avoid an extra trip to the disk.
 */
unsafe extern "C" fn tree_current_is_physical_dir(mut t: *mut tree) -> libc::c_int {
    let mut st: *const stat = 0 as *const stat;
    /*
     * If stat() says it isn't a dir, then it's not a dir.
     * If stat() data is cached, this check is free, so do it first.
     */
    if (*t).flags & hasStat != 0 {
        st = tree_current_stat(t);
        if st.is_null() {
            return 0 as libc::c_int;
        }
        if !((*st).st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint) {
            return 0 as libc::c_int;
        }
    }
    /*
     * Either stat() said it was a dir (in which case, we have
     * to determine whether it's really a link to a dir) or
     * stat() info wasn't available.  So we use lstat(), which
     * hopefully is already cached.
     */
    st = tree_current_lstat(t);
    /* If we can't stat it, it's not a dir. */
    if st.is_null() {
        return 0 as libc::c_int;
    }
    /* Use the definitive test.  Hopefully this is cached. */
    return ((*st).st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
/*
 * Test whether the same file has been in the tree as its parent.
 */
unsafe extern "C" fn tree_target_is_same_as_parent(
    mut t: *mut tree,
    mut st: *const stat,
) -> libc::c_int {
    let mut te: *mut tree_entry = 0 as *mut tree_entry;
    te = (*(*t).current).parent;
    while !te.is_null() {
        if (*te).dev == (*st).st_dev as int64_t && (*te).ino == (*st).st_ino as int64_t {
            return 1 as libc::c_int;
        }
        te = (*te).parent
    }
    return 0 as libc::c_int;
}
/*
 * Test whether the current file is symbolic link target and
 * on the other filesystem.
 */
unsafe extern "C" fn tree_current_is_symblic_link_target(mut t: *mut tree) -> libc::c_int {
    static mut lst: *const stat = 0 as *const stat;
    static mut st: *const stat = 0 as *const stat;
    lst = tree_current_lstat(t);
    st = tree_current_stat(t);
    return (!st.is_null()
        && !lst.is_null()
        && (*st).st_dev as int64_t == (*(*t).current_filesystem).dev
        && (*st).st_dev != (*lst).st_dev) as libc::c_int;
}
/*
 * Return the access path for the entry just returned from tree_next().
 */
unsafe extern "C" fn tree_current_access_path(mut t: *mut tree) -> *const libc::c_char {
    return (*t).basename;
}
/*
 * Return information about the current entry.
 */
/*
 * The current full pathname, length of the full pathname, and a name
 * that can be used to access the file.  Because tree does use chdir
 * extensively, the access path is almost never the same as the full
 * current path.
 *
 * TODO: On platforms that support it, use openat()-style operations
 * to eliminate the chdir() operations entirely while still supporting
 * arbitrarily deep traversals.  This makes access_path troublesome to
 * support, of course, which means we'll need a rich enough interface
 * that clients can function without it.  (In particular, we'll need
 * tree_current_open() that returns an open file descriptor.)
 *
 */
/*
 * Return the full path for the entry just returned from tree_next().
 */
unsafe extern "C" fn tree_current_path(mut t: *mut tree) -> *const libc::c_char {
    return (*t).path.s;
}
/*
 * Terminate the traversal.
 */
unsafe extern "C" fn tree_close(mut t: *mut tree) {
    if t.is_null() {
        return;
    }
    if (*t).entry_fd >= 0 as libc::c_int {
        close_and_restore_time((*t).entry_fd, t, &mut (*t).restore_time);
        (*t).entry_fd = -(1 as libc::c_int)
    }
    /* Close the handle of readdir(). */
    if !(*t).d.is_null() {
        closedir((*t).d);
        (*t).d = INVALID_DIR_HANDLE as *mut DIR
    }
    /* Release anything remaining in the stack. */
    while !(*t).stack.is_null() {
        if (*(*t).stack).flags & isDirLink != 0 {
            close((*(*t).stack).symlink_parent_fd);
        }
        tree_pop(t);
    }
    if (*t).working_dir_fd >= 0 as libc::c_int {
        close((*t).working_dir_fd);
        (*t).working_dir_fd = -(1 as libc::c_int)
    }
    if (*t).initial_dir_fd >= 0 as libc::c_int {
        close((*t).initial_dir_fd);
        (*t).initial_dir_fd = -(1 as libc::c_int)
    };
}
/*
 * Release any resources.
 */
unsafe extern "C" fn tree_free(mut t: *mut tree) {
    let mut i: libc::c_int = 0;
    if t.is_null() {
        return;
    }
    archive_string_free(&mut (*t).path);
    free((*t).sparse_list as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < (*t).max_filesystem_id {
        free((*(*t).filesystem_table.offset(i as isize)).allocation_ptr as *mut libc::c_void);
        i += 1
    }
    free((*t).filesystem_table as *mut libc::c_void);
    free(t as *mut libc::c_void);
}
