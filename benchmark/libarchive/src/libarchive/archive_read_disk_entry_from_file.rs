use ::libc;
extern "C" {
    /* Length of malloc-ed storage in bytes. */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
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
    pub type tree;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn getxattr(
        __path: *const libc::c_char,
        __name: *const libc::c_char,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn lgetxattr(
        __path: *const libc::c_char,
        __name: *const libc::c_char,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn fgetxattr(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn listxattr(__path: *const libc::c_char, __list: *mut libc::c_char, __size: size_t)
        -> ssize_t;
    #[no_mangle]
    fn llistxattr(
        __path: *const libc::c_char,
        __list: *mut libc::c_char,
        __size: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn flistxattr(__fd: libc::c_int, __list: *mut libc::c_char, __size: size_t) -> ssize_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char, __len: size_t) -> ssize_t;
    #[no_mangle]
    fn readlinkat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_read_disk_gname(_: *mut archive, _: la_int64_t) -> *const libc::c_char;
    #[no_mangle]
    fn archive_read_disk_uname(_: *mut archive, _: la_int64_t) -> *const libc::c_char;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_sourcepath(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_set_fflags(_: *mut archive_entry, _: libc::c_ulong, _: libc::c_ulong);
    #[no_mangle]
    fn archive_entry_copy_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_uname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_stat(_: *mut archive_entry, _: *const stat);
    #[no_mangle]
    fn archive_entry_xattr_add_entry(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: *const libc::c_void,
        _: size_t,
    );
    #[no_mangle]
    fn archive_entry_sparse_add_entry(_: *mut archive_entry, _: la_int64_t, _: la_int64_t);
    /*
     * To retrieve the xattr list, first "reset", then repeatedly ask for the
     * "next" entry.
     */
    #[no_mangle]
    fn archive_entry_sparse_count(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __archive_ensure_cloexec_flag(fd: libc::c_int);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
pub type uint64_t = __uint64_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub type __u32 = libc::c_uint;
pub type __u64 = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fiemap_extent {
    pub fe_logical: __u64,
    pub fe_physical: __u64,
    pub fe_length: __u64,
    pub fe_reserved64: [__u64; 2],
    pub fe_flags: __u32,
    pub fe_reserved: [__u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fiemap {
    pub fm_start: __u64,
    pub fm_length: __u64,
    pub fm_flags: __u32,
    pub fm_mapped_extents: __u32,
    pub fm_extent_count: __u32,
    pub fm_reserved: __u32,
    pub fm_extents: [fiemap_extent; 0],
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
pub const _IOC_NRBITS: libc::c_int = 8 as libc::c_int;
pub const _IOC_TYPEBITS: libc::c_int = 8 as libc::c_int;
pub const _IOC_SIZEBITS: libc::c_int = 14 as libc::c_int;
pub const _IOC_NRSHIFT: libc::c_int = 0 as libc::c_int;
pub const _IOC_TYPESHIFT: libc::c_int = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: libc::c_int = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: libc::c_int = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const EOPNOTSUPP: libc::c_int = 95 as libc::c_int;
pub const ENOTSUP: libc::c_int = EOPNOTSUPP;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const ENOSYS: libc::c_int = 38 as libc::c_int;
pub const ENXIO: libc::c_int = 6 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const O_NONBLOCK: libc::c_int = 0o4000 as libc::c_int;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const FIEMAP_FLAG_SYNC: libc::c_int = 0x1 as libc::c_int;
pub const FIEMAP_EXTENT_LAST: libc::c_int = 0x1 as libc::c_int;
pub const FIEMAP_EXTENT_UNWRITTEN: libc::c_int = 0x800 as libc::c_int;
pub const FS_IOC_GETFLAGS: libc::c_ulong = ((2 as libc::c_uint) << _IOC_DIRSHIFT
    | (('f' as i32) << _IOC_TYPESHIFT) as libc::c_uint
    | ((1 as libc::c_int) << _IOC_NRSHIFT) as libc::c_uint)
    as libc::c_ulong
    | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << _IOC_SIZESHIFT;
pub const FS_IOC_FIEMAP: libc::c_ulong = ((2 as libc::c_uint | 1 as libc::c_uint) << _IOC_DIRSHIFT
    | (('f' as i32) << _IOC_TYPESHIFT) as libc::c_uint
    | ((11 as libc::c_int) << _IOC_NRSHIFT) as libc::c_uint)
    as libc::c_ulong
    | (::std::mem::size_of::<fiemap>() as libc::c_ulong) << _IOC_SIZESHIFT;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const SEEK_HOLE: libc::c_int = 4 as libc::c_int;
pub const SEEK_DATA: libc::c_int = 3 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_READDISK_MAC_COPYFILE: libc::c_int = 0x4 as libc::c_int;
pub const ARCHIVE_READDISK_NO_XATTR: libc::c_int = 0x10 as libc::c_int;
pub const ARCHIVE_READDISK_NO_ACL: libc::c_int = 0x20 as libc::c_int;
pub const ARCHIVE_READDISK_NO_FFLAGS: libc::c_int = 0x40 as libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_entry_setup_acls(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
    mut fd: *mut libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
/*
 * Enter working directory and return working pathname of archive_entry.
 * If a pointer to an integer is provided and its value is below zero
 * open a file descriptor on this pathname.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_entry_setup_path(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
    mut fd: *mut libc::c_int,
) -> *const libc::c_char {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    path = archive_entry_sourcepath(entry);
    if path.is_null()
        || !(*a).tree.is_null()
            && (*a)
                .tree_enter_working_dir
                .expect("non-null function pointer")((*a).tree)
                != 0 as libc::c_int
    {
        path = archive_entry_pathname(entry)
    }
    if path.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Couldn\'t determine path\x00" as *const u8 as *const libc::c_char,
        );
    } else if !fd.is_null()
        && *fd < 0 as libc::c_int
        && !(*a).tree.is_null()
        && ((*a).follow_symlinks as libc::c_int != 0
            || archive_entry_filetype(entry) != AE_IFLNK as mode_t)
    {
        *fd = (*a).open_on_current_dir.expect("non-null function pointer")(
            (*a).tree,
            path,
            O_RDONLY | O_NONBLOCK,
        )
    }
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_entry_from_file(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
    mut fd: libc::c_int,
    mut st: *const stat,
) -> libc::c_int {
    let mut a: *mut archive_read_disk = _a as *mut archive_read_disk;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: stat = stat {
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
    let mut initial_fd: libc::c_int = fd;
    let mut r: libc::c_int = 0;
    let mut r1: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xbadb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_disk_entry_from_file\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_clear_error(_a);
    path = archive_entry_sourcepath(entry);
    if path.is_null() {
        path = archive_entry_pathname(entry)
    }
    if (*a).tree.is_null() {
        if st.is_null() {
            if fd >= 0 as libc::c_int {
                if fstat(fd, &mut s) != 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        errno,
                        b"Can\'t fstat\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
            } else if (*a).follow_symlinks == 0 {
                if lstat(path, &mut s) != 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        errno,
                        b"Can\'t lstat %s\x00" as *const u8 as *const libc::c_char,
                        path,
                    );
                    return -(25 as libc::c_int);
                }
            } else if stat(path, &mut s) != 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Can\'t stat %s\x00" as *const u8 as *const libc::c_char,
                    path,
                );
                return -(25 as libc::c_int);
            }
            st = &mut s
        }
        archive_entry_copy_stat(entry, st);
    }
    /* Lookup uname/gname */
    name = archive_read_disk_uname(_a, archive_entry_uid(entry));
    if !name.is_null() {
        archive_entry_copy_uname(entry, name);
    }
    name = archive_read_disk_gname(_a, archive_entry_gid(entry));
    if !name.is_null() {
        archive_entry_copy_gname(entry, name);
    }
    /* Linux requires an extra ioctl to pull the flags.  Although
     * this is an extra step, it has a nice side-effect: We get an
     * open file descriptor which we can use in the subsequent lookups. */
    if (*a).flags & ARCHIVE_READDISK_NO_FFLAGS == 0 as libc::c_int
        && ((*st).st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint
            || (*st).st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint)
    {
        if fd < 0 as libc::c_int {
            if !(*a).tree.is_null() {
                fd = (*a).open_on_current_dir.expect("non-null function pointer")(
                    (*a).tree,
                    path,
                    O_RDONLY | O_NONBLOCK | O_CLOEXEC,
                )
            } else {
                fd = open(path, O_RDONLY | O_NONBLOCK | O_CLOEXEC)
            }
            __archive_ensure_cloexec_flag(fd);
        }
        if fd >= 0 as libc::c_int {
            let mut stflags: libc::c_int = 0;
            r = ioctl(fd, FS_IOC_GETFLAGS, &mut stflags as *mut libc::c_int);
            if r == 0 as libc::c_int && stflags != 0 as libc::c_int {
                archive_entry_set_fflags(
                    entry,
                    stflags as libc::c_ulong,
                    0 as libc::c_int as libc::c_ulong,
                );
            }
        }
    }
    if (*st).st_mode & __S_IFMT as libc::c_uint == 0o120000 as libc::c_int as libc::c_uint {
        let mut linkbuffer_len: size_t = (*st).st_size as size_t;
        let mut linkbuffer: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut lnklen: libc::c_int = 0;
        linkbuffer = malloc(linkbuffer_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if linkbuffer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Couldn\'t read link data\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        if !(*a).tree.is_null() {
            lnklen = readlinkat(
                (*a).tree_current_dir_fd.expect("non-null function pointer")((*a).tree),
                path,
                linkbuffer,
                linkbuffer_len,
            ) as libc::c_int
        /* HAVE_READLINKAT */
        } else {
            lnklen = readlink(path, linkbuffer, linkbuffer_len) as libc::c_int
        }
        if lnklen < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Couldn\'t read link data\x00" as *const u8 as *const libc::c_char,
            );
            free(linkbuffer as *mut libc::c_void);
            return -(25 as libc::c_int);
        }
        *linkbuffer.offset(lnklen as isize) = '\u{0}' as i32 as libc::c_char;
        archive_entry_set_symlink(entry, linkbuffer);
        free(linkbuffer as *mut libc::c_void);
    }
    /* HAVE_READLINK || HAVE_READLINKAT */
    r = 0 as libc::c_int;
    if (*a).flags & ARCHIVE_READDISK_NO_ACL == 0 as libc::c_int {
        r = archive_read_disk_entry_setup_acls(a, entry, &mut fd)
    }
    if (*a).flags & ARCHIVE_READDISK_NO_XATTR == 0 as libc::c_int {
        r1 = setup_xattrs(a, entry, &mut fd);
        if r1 < r {
            r = r1
        }
    }
    if (*a).flags & ARCHIVE_READDISK_MAC_COPYFILE != 0 {
        r1 = setup_mac_metadata(a, entry, &mut fd);
        if r1 < r {
            r = r1
        }
    }
    r1 = setup_sparse(a, entry, &mut fd);
    if r1 < r {
        r = r1
    }
    /* If we opened the file earlier in this function, close it. */
    if initial_fd != fd {
        close(fd);
    }
    return r;
}
/* This is the tree-walking code for POSIX systems. */
/*
 * Some Linux distributions have both linux/ext2_fs.h and ext2fs/ext2_fs.h.
 * As the include guards don't agree, the order of include is important.
 */
/*
 * Stub implementation for non-Mac systems.
 */
unsafe extern "C" fn setup_mac_metadata(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
    mut fd: *mut libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
/*
 * Linux, Darwin and AIX extended attribute support.
 *
 * TODO:  By using a stack-allocated buffer for the first
 * call to getxattr(), we might be able to avoid the second
 * call entirely.  We only need the second call if the
 * stack-allocated buffer is too small.  But a modest buffer
 * of 1024 bytes or so will often be big enough.  Same applies
 * to listxattr().
 */
unsafe extern "C" fn setup_xattr(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
    mut fd: libc::c_int,
    mut accpath: *const libc::c_char,
) -> libc::c_int {
    let mut size: ssize_t = 0;
    let mut value: *mut libc::c_void = NULL as *mut libc::c_void;
    if fd >= 0 as libc::c_int {
        size = fgetxattr(
            fd,
            name,
            NULL as *mut libc::c_void,
            0 as libc::c_int as size_t,
        )
    } else if (*a).follow_symlinks == 0 {
        size = lgetxattr(
            accpath,
            name,
            NULL as *mut libc::c_void,
            0 as libc::c_int as size_t,
        )
    } else {
        size = getxattr(
            accpath,
            name,
            NULL as *mut libc::c_void,
            0 as libc::c_int as size_t,
        )
    }
    if size == -(1 as libc::c_int) as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Couldn\'t query extended attribute\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    if size > 0 as libc::c_int as libc::c_long && {
        value = malloc(size as libc::c_ulong);
        value.is_null()
    } {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if fd >= 0 as libc::c_int {
        size = fgetxattr(fd, name, value, size as size_t)
    } else if (*a).follow_symlinks == 0 {
        size = lgetxattr(accpath, name, value, size as size_t)
    } else {
        size = getxattr(accpath, name, value, size as size_t)
    }
    if size == -(1 as libc::c_int) as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Couldn\'t read extended attribute\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    archive_entry_xattr_add_entry(entry, name, value, size as size_t);
    free(value);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setup_xattrs(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
    mut fd: *mut libc::c_int,
) -> libc::c_int {
    let mut list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut list_size: ssize_t = 0;
    path = NULL as *const libc::c_char;
    if *fd < 0 as libc::c_int {
        path = archive_read_disk_entry_setup_path(a, entry, fd);
        if path.is_null() {
            return -(20 as libc::c_int);
        }
    }
    if *fd >= 0 as libc::c_int {
        list_size = flistxattr(*fd, NULL as *mut libc::c_char, 0 as libc::c_int as size_t)
    } else if (*a).follow_symlinks == 0 {
        list_size = llistxattr(path, NULL as *mut libc::c_char, 0 as libc::c_int as size_t)
    } else {
        list_size = listxattr(path, NULL as *mut libc::c_char, 0 as libc::c_int as size_t)
    }
    if list_size == -(1 as libc::c_int) as libc::c_long {
        if errno == ENOTSUP || errno == ENOSYS {
            return 0 as libc::c_int;
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Couldn\'t list extended attributes\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    if list_size == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    list = malloc(list_size as libc::c_ulong) as *mut libc::c_char;
    if list.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if *fd >= 0 as libc::c_int {
        list_size = flistxattr(*fd, list, list_size as size_t)
    } else if (*a).follow_symlinks == 0 {
        list_size = llistxattr(path, list, list_size as size_t)
    } else {
        list_size = listxattr(path, list, list_size as size_t)
    }
    if list_size == -(1 as libc::c_int) as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Couldn\'t retrieve extended attributes\x00" as *const u8 as *const libc::c_char,
        );
        free(list as *mut libc::c_void);
        return -(20 as libc::c_int);
    }
    p = list;
    while (p.offset_from(list) as libc::c_long) < list_size {
        /* Linux: skip POSIX.1e ACL extended attributes */
        if !(strncmp(
            p,
            b"system.\x00" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && (strcmp(
                p.offset(7 as libc::c_int as isize),
                b"posix_acl_access\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    p.offset(7 as libc::c_int as isize),
                    b"posix_acl_default\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int))
        {
            if !(strncmp(
                p,
                b"trusted.SGI_\x00" as *const u8 as *const libc::c_char,
                12 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
                && (strcmp(
                    p.offset(12 as libc::c_int as isize),
                    b"ACL_DEFAULT\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        p.offset(12 as libc::c_int as isize),
                        b"ACL_FILE\x00" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int))
            {
                /* Linux: xfsroot namespace is obsolete and unsupported */
                if !(strncmp(
                    p,
                    b"xfsroot.\x00" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int)
                {
                    setup_xattr(a, entry, p, *fd, path);
                }
            }
        }
        p = p.offset(strlen(p).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
    }
    free(list as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*
 * Linux FIEMAP sparse interface.
 *
 * The FIEMAP ioctl returns an "extent" for each physical allocation
 * on disk.  We need to process those to generate a more compact list
 * of logical file blocks.  We also need to be very careful to use
 * FIEMAP_FLAG_SYNC here, since there are reports that Linux sometimes
 * does not report allocations for newly-written data that hasn't
 * been synced to disk.
 *
 * It's important to return a minimal sparse file list because we want
 * to not trigger sparse file extensions if we don't have to, since
 * not all readers support them.
 */
unsafe extern "C" fn setup_sparse_fiemap(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
    mut fd: *mut libc::c_int,
) -> libc::c_int {
    let mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut fm: *mut fiemap = 0 as *mut fiemap;
    let mut fe: *mut fiemap_extent = 0 as *mut fiemap_extent;
    let mut size: int64_t = 0;
    let mut count: libc::c_int = 0;
    let mut do_fiemap: libc::c_int = 0;
    let mut iters: libc::c_int = 0;
    let mut exit_sts: libc::c_int = ARCHIVE_OK;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if archive_entry_filetype(entry) != AE_IFREG as mode_t
        || archive_entry_size(entry) <= 0 as libc::c_int as libc::c_long
        || !archive_entry_hardlink(entry).is_null()
    {
        return 0 as libc::c_int;
    }
    if *fd < 0 as libc::c_int {
        path = archive_read_disk_entry_setup_path(a, entry, NULL as *mut libc::c_int);
        if path.is_null() {
            return -(25 as libc::c_int);
        }
        if !(*a).tree.is_null() {
            *fd = (*a).open_on_current_dir.expect("non-null function pointer")(
                (*a).tree,
                path,
                O_RDONLY | O_NONBLOCK | O_CLOEXEC,
            )
        } else {
            *fd = open(path, O_RDONLY | O_NONBLOCK | O_CLOEXEC)
        }
        if *fd < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t open `%s\'\x00" as *const u8 as *const libc::c_char,
                path,
            );
            return -(25 as libc::c_int);
        }
        __archive_ensure_cloexec_flag(*fd);
    }
    /* Initialize buffer to avoid the error valgrind complains about. */
    memset(
        buff.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    count = (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<fiemap>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<fiemap_extent>() as libc::c_ulong)
        as libc::c_int;
    fm = buff.as_mut_ptr() as *mut fiemap;
    (*fm).fm_start = 0 as libc::c_int as __u64;
    (*fm).fm_length = !(0 as libc::c_ulonglong);
    (*fm).fm_flags = FIEMAP_FLAG_SYNC as __u32;
    (*fm).fm_extent_count = count as __u32;
    do_fiemap = 1 as libc::c_int;
    size = archive_entry_size(entry);
    iters = 0 as libc::c_int;
    loop {
        let mut i: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        r = ioctl(*fd, FS_IOC_FIEMAP, fm);
        if r < 0 as libc::c_int {
            break;
        }
        if (*fm).fm_mapped_extents == 0 as libc::c_int as libc::c_uint {
            if iters == 0 as libc::c_int {
                /* Fully sparse file; insert a zero-length "data" entry */
                archive_entry_sparse_add_entry(
                    entry,
                    0 as libc::c_int as la_int64_t,
                    0 as libc::c_int as la_int64_t,
                );
            }
            break;
        } else {
            fe = (*fm).fm_extents.as_mut_ptr();
            i = 0 as libc::c_int;
            while i < (*fm).fm_mapped_extents as libc::c_int {
                if (*fe).fe_flags & FIEMAP_EXTENT_UNWRITTEN as libc::c_uint == 0 {
                    /* The fe_length of the last block does not
                     * adjust itself to its size files. */
                    let mut length: int64_t = (*fe).fe_length as int64_t;
                    if (*fe).fe_logical.wrapping_add(length as libc::c_ulonglong)
                        > size as uint64_t as libc::c_ulonglong
                    {
                        length = (length as libc::c_ulonglong).wrapping_sub(
                            (*fe)
                                .fe_logical
                                .wrapping_add(length as libc::c_ulonglong)
                                .wrapping_sub(size as libc::c_ulonglong),
                        ) as int64_t as int64_t
                    }
                    if (*fe).fe_logical == 0 as libc::c_int as libc::c_ulonglong && length == size {
                        /* This is not sparse. */
                        do_fiemap = 0 as libc::c_int;
                        break;
                    } else if length > 0 as libc::c_int as libc::c_long {
                        archive_entry_sparse_add_entry(
                            entry,
                            (*fe).fe_logical as la_int64_t,
                            length,
                        );
                    }
                }
                if (*fe).fe_flags & FIEMAP_EXTENT_LAST as libc::c_uint != 0 {
                    do_fiemap = 0 as libc::c_int
                }
                i += 1;
                fe = fe.offset(1)
            }
            if !(do_fiemap != 0) {
                break;
            }
            fe = (*fm)
                .fm_extents
                .as_mut_ptr()
                .offset((*fm).fm_mapped_extents as isize)
                .offset(-(1 as libc::c_int as isize));
            (*fm).fm_start = (*fe).fe_logical.wrapping_add((*fe).fe_length);
            iters += 1
        }
    }
    /* When something error happens, it is better we
     * should return ARCHIVE_OK because an earlier
     * version(<2.6.28) cannot perform FS_IOC_FIEMAP. */
    return exit_sts;
}
/* defined(HAVE_LINUX_FIEMAP_H) */
/*
 * SEEK_HOLE sparse interface (FreeBSD, Linux, Solaris)
 */
unsafe extern "C" fn setup_sparse(
    mut a: *mut archive_read_disk,
    mut entry: *mut archive_entry,
    mut fd: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut size: int64_t = 0;
    let mut initial_off: off_t = 0;
    let mut off_s: off_t = 0;
    let mut off_e: off_t = 0;
    let mut exit_sts: libc::c_int = ARCHIVE_OK;
    let mut check_fully_sparse: libc::c_int = 0 as libc::c_int;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if archive_entry_filetype(entry) != AE_IFREG as mode_t
        || archive_entry_size(entry) <= 0 as libc::c_int as libc::c_long
        || !archive_entry_hardlink(entry).is_null()
    {
        return 0 as libc::c_int;
    }
    /* Does filesystem support the reporting of hole ? */
    if *fd < 0 as libc::c_int {
        path = archive_read_disk_entry_setup_path(a, entry, fd)
    } else {
        path = NULL as *const libc::c_char
    }
    if *fd >= 0 as libc::c_int {
        initial_off = lseek(*fd, 0 as libc::c_int as __off_t, SEEK_CUR);
        if initial_off != 0 as libc::c_int as libc::c_long {
            lseek(*fd, 0 as libc::c_int as __off_t, SEEK_SET);
        }
    } else {
        if path.is_null() {
            return -(25 as libc::c_int);
        }
        *fd = open(path, O_RDONLY | O_NONBLOCK | O_CLOEXEC);
        if *fd < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t open `%s\'\x00" as *const u8 as *const libc::c_char,
                path,
            );
            return -(25 as libc::c_int);
        }
        __archive_ensure_cloexec_flag(*fd);
        initial_off = 0 as libc::c_int as off_t
    }
    /* Check if the underlying filesystem supports seek hole */
    off_s = lseek(*fd, 0 as libc::c_int as __off_t, SEEK_HOLE);
    if off_s < 0 as libc::c_int as libc::c_long {
        return setup_sparse_fiemap(a, entry, fd);
    } else {
        if off_s > 0 as libc::c_int as libc::c_long {
            lseek(*fd, 0 as libc::c_int as __off_t, SEEK_SET);
        }
    }
    off_s = 0 as libc::c_int as off_t;
    size = archive_entry_size(entry);
    loop {
        if !(off_s < size) {
            current_block = 14775119014532381840;
            break;
        }
        off_s = lseek(*fd, off_s, SEEK_DATA);
        if off_s == -(1 as libc::c_int) as off_t {
            if errno == ENXIO {
                /* no more hole */
                if archive_entry_sparse_count(entry) == 0 as libc::c_int {
                    /* Potentially a fully-sparse file. */
                    check_fully_sparse = 1 as libc::c_int
                }
                current_block = 14775119014532381840;
                break;
            } else {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"lseek(SEEK_HOLE) failed\x00" as *const u8 as *const libc::c_char,
                );
                exit_sts = ARCHIVE_FAILED;
                current_block = 10744229829428874160;
                break;
            }
        } else {
            off_e = lseek(*fd, off_s, SEEK_HOLE);
            if off_e == -(1 as libc::c_int) as off_t {
                if errno == ENXIO {
                    off_e = lseek(*fd, 0 as libc::c_int as __off_t, SEEK_END);
                    if off_e != -(1 as libc::c_int) as off_t {
                        current_block = 14775119014532381840;
                        break;
                    }
                    /* no more data */
                } /* This is not sparse. */
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"lseek(SEEK_DATA) failed\x00" as *const u8 as *const libc::c_char,
                );
                exit_sts = ARCHIVE_FAILED;
                current_block = 10744229829428874160;
                break;
            } else {
                if off_s == 0 as libc::c_int as libc::c_long && off_e == size {
                    current_block = 14775119014532381840;
                    break;
                }
                archive_entry_sparse_add_entry(entry, off_s, off_e - off_s);
                off_s = off_e
            }
        }
    }
    match current_block {
        14775119014532381840 => {
            if check_fully_sparse != 0 {
                if lseek(*fd, 0 as libc::c_int as __off_t, SEEK_HOLE)
                    == 0 as libc::c_int as libc::c_long
                    && lseek(*fd, 0 as libc::c_int as __off_t, SEEK_END) == size
                {
                    /* Fully sparse file; insert a zero-length "data" entry */
                    archive_entry_sparse_add_entry(
                        entry,
                        0 as libc::c_int as la_int64_t,
                        0 as libc::c_int as la_int64_t,
                    );
                }
            }
        }
        _ => {}
    }
    lseek(*fd, initial_off, SEEK_SET);
    return exit_sts;
}
/* !defined(_WIN32) || defined(__CYGWIN__) */
