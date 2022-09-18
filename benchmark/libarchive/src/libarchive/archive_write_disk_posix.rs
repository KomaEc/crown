use ::libc;
extern "C" {
    pub type internal_state;
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn lsetxattr(
        __path: *const libc::c_char,
        __name: *const libc::c_char,
        __value: *const libc::c_void,
        __size: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn fsetxattr(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __value: *const libc::c_void,
        __size: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn mknod(__path: *const libc::c_char, __mode: __mode_t, __dev: __dev_t) -> libc::c_int;
    #[no_mangle]
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    #[no_mangle]
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    #[no_mangle]
    fn lchown(__file: *const libc::c_char, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn geteuid() -> __uid_t;
    #[no_mangle]
    fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn symlink(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn unlinkat(__fd: libc::c_int, __name: *const libc::c_char, __flag: libc::c_int)
        -> libc::c_int;
    #[no_mangle]
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
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
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
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
     * $FreeBSD$
     */
    /* E.g., access or default */
    /* E.g., user/group/other/mask */
    /* r/w/x bits */
    /* uid/gid for user/group */
    /* uname/gname */
    /* See acl_next for details. */
    #[no_mangle]
    fn archive_acl_clear(_: *mut archive_acl);
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_acl_copy(_: *mut archive_acl, _: *mut archive_acl);
    /* The 'clone' function does a deep copy; all of the strings are copied too. */
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
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
    fn archive_entry_atime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_birthtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_birthtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_birthtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_ctime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_fflags(_: *mut archive_entry, _: *mut libc::c_ulong, _: *mut libc::c_ulong);
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_gname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_mtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_rdev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_size_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_unset_size(_: *mut archive_entry);
    /*
     * Storage for Mac OS-specific AppleDouble metadata information.
     * Apple-format tar files store a separate binary blob containing
     * encoded metadata with ACL, extended attributes, etc.
     * This provides a place to store that blob.
     */
    #[no_mangle]
    fn archive_entry_mac_metadata(_: *mut archive_entry, _: *mut size_t) -> *const libc::c_void;
    #[no_mangle]
    fn archive_entry_acl(_: *mut archive_entry) -> *mut archive_acl;
    #[no_mangle]
    fn archive_entry_xattr_reset(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_xattr_next(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut *const libc::c_void,
        _: *mut size_t,
    ) -> libc::c_int;
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
    fn __archive_ensure_cloexec_flag(fd: libc::c_int);
    #[no_mangle]
    fn __archive_mkstemp(template: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __archive_clean(_: *mut archive) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type wchar_t = libc::c_int;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
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
pub struct archive_write_disk {
    pub archive: archive,
    pub user_umask: mode_t,
    pub fixup_list: *mut fixup_entry,
    pub current_fixup: *mut fixup_entry,
    pub user_uid: int64_t,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub start_time: time_t,
    pub lookup_gid: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, _: int64_t) -> int64_t,
    >,
    pub cleanup_gid: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub lookup_gid_data: *mut libc::c_void,
    pub lookup_uid: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, _: int64_t) -> int64_t,
    >,
    pub cleanup_uid: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub lookup_uid_data: *mut libc::c_void,
    pub path_safe: archive_string,
    pub st: stat,
    pub pst: *mut stat,
    pub entry: *mut archive_entry,
    pub name: *mut libc::c_char,
    pub _name_data: archive_string,
    pub tmpname: *mut libc::c_char,
    pub _tmpname_data: archive_string,
    pub todo: libc::c_int,
    pub deferred: libc::c_int,
    pub flags: libc::c_int,
    pub fd: libc::c_int,
    pub offset: int64_t,
    pub fd_offset: int64_t,
    pub total_bytes_written: int64_t,
    pub filesize: int64_t,
    pub restore_pwd: libc::c_int,
    pub mode: mode_t,
    pub uid: int64_t,
    pub gid: int64_t,
    pub decmpfs_attr_size: uint32_t,
    pub decmpfs_header_p: *mut libc::c_uchar,
    pub rsrc_xattr_options: libc::c_int,
    pub resource_fork: *mut libc::c_uchar,
    pub resource_fork_allocated_size: size_t,
    pub decmpfs_block_count: libc::c_uint,
    pub decmpfs_block_info: *mut uint32_t,
    pub compressed_buffer: *mut libc::c_uchar,
    pub compressed_buffer_size: size_t,
    pub compressed_buffer_remaining: size_t,
    pub compressed_rsrc_position: uint32_t,
    pub compressed_rsrc_position_v: uint32_t,
    pub uncompressed_buffer: *mut libc::c_char,
    pub block_remaining_bytes: size_t,
    pub file_remaining_bytes: size_t,
    pub stream: z_stream,
    pub stream_valid: libc::c_int,
    pub decmpfs_compression_level: libc::c_int,
}
/* Ignore non-int O_NOFOLLOW constant. */
/* gnulib's fcntl.h does this on AIX, but it seems practical everywhere */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fixup_entry {
    pub next: *mut fixup_entry,
    pub acl: archive_acl,
    pub mode: mode_t,
    pub atime: int64_t,
    pub birthtime: int64_t,
    pub mtime: int64_t,
    pub ctime: int64_t,
    pub atime_nanos: libc::c_ulong,
    pub birthtime_nanos: libc::c_ulong,
    pub mtime_nanos: libc::c_ulong,
    pub ctime_nanos: libc::c_ulong,
    pub fflags_set: libc::c_ulong,
    pub mac_metadata_size: size_t,
    pub mac_metadata: *mut libc::c_void,
    pub fixup: libc::c_int,
    pub name: *mut libc::c_char,
}
/* Return an opaque ACL object. */
/* There's not yet anything clients can actually do with this... */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_acl {
    pub mode: mode_t,
    pub acl_head: *mut archive_acl_entry,
    pub acl_p: *mut archive_acl_entry,
    pub acl_state: libc::c_int,
    pub acl_text_w: *mut wchar_t,
    pub acl_text: *mut libc::c_char,
    pub acl_types: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_acl_entry {
    pub next: *mut archive_acl_entry,
    pub type_0: libc::c_int,
    pub tag: libc::c_int,
    pub permset: libc::c_int,
    pub id: libc::c_int,
    pub name: archive_mstring,
}
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
pub const INT64_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
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
pub const _IOC_SIZESHIFT: libc::c_int = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: libc::c_int = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const _IOC_TYPESHIFT: libc::c_int = _IOC_NRSHIFT + _IOC_NRBITS;
pub const __S_ISVTX: libc::c_int = 0o1000 as libc::c_int;
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const __S_ISUID: libc::c_int = 0o4000 as libc::c_int;
pub const __S_ISGID: libc::c_int = 0o2000 as libc::c_int;
pub const __S_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const __S_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const EPERM: libc::c_int = 1 as libc::c_int;
pub const ENOENT: libc::c_int = 2 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOTSUP: libc::c_int = EOPNOTSUPP;
pub const ENOSYS: libc::c_int = 38 as libc::c_int;
pub const EOPNOTSUPP: libc::c_int = 95 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EEXIST: libc::c_int = 17 as libc::c_int;
pub const ENOTDIR: libc::c_int = 20 as libc::c_int;
pub const EISDIR: libc::c_int = 21 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const O_WRONLY: libc::c_int = 0o1 as libc::c_int;
pub const O_CREAT: libc::c_int = 0o100 as libc::c_int;
pub const O_EXCL: libc::c_int = 0o200 as libc::c_int;
pub const O_TRUNC: libc::c_int = 0o1000 as libc::c_int;
pub const O_NONBLOCK: libc::c_int = 0o4000 as libc::c_int;
pub const __O_DIRECTORY: libc::c_int = 0o200000 as libc::c_int;
pub const __O_NOFOLLOW: libc::c_int = 0o400000 as libc::c_int;
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const __O_PATH: libc::c_int = 0o10000000 as libc::c_int;
pub const O_DIRECTORY: libc::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: libc::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const O_PATH: libc::c_int = __O_PATH;
pub const AT_FDCWD: libc::c_int = -(100 as libc::c_int);
pub const AT_SYMLINK_NOFOLLOW: libc::c_int = 0x100 as libc::c_int;
pub const S_ISVTX: libc::c_int = __S_ISVTX;
pub const S_ISUID: libc::c_int = 0o4000 as libc::c_int;
pub const S_ISGID: libc::c_int = __S_ISGID;
pub const S_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const S_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const FS_IOC_GETFLAGS: libc::c_ulong = ((2 as libc::c_uint) << _IOC_DIRSHIFT
    | (('f' as i32) << _IOC_TYPESHIFT) as libc::c_uint
    | ((1 as libc::c_int) << _IOC_NRSHIFT) as libc::c_uint)
    as libc::c_ulong
    | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << _IOC_SIZESHIFT;
pub const FS_IOC_SETFLAGS: libc::c_ulong = ((1 as libc::c_uint) << _IOC_DIRSHIFT
    | (('f' as i32) << _IOC_TYPESHIFT) as libc::c_uint
    | ((2 as libc::c_int) << _IOC_NRSHIFT) as libc::c_uint)
    as libc::c_ulong
    | (::std::mem::size_of::<libc::c_long>() as libc::c_ulong) << _IOC_SIZESHIFT;
pub const FS_JOURNAL_DATA_FL: libc::c_int = 0x4000 as libc::c_int;
pub const FS_APPEND_FL: libc::c_int = 0x20 as libc::c_int;
pub const PATH_MAX: libc::c_int = 4096 as libc::c_int;
pub const FS_IMMUTABLE_FL: libc::c_int = 0x10 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
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
pub const ARCHIVE_EXTRACT_OWNER: libc::c_int = 0x1 as libc::c_int;
/* Default: Do obey umask, do not restore SUID/SGID/SVTX bits. */
pub const ARCHIVE_EXTRACT_PERM: libc::c_int = 0x2 as libc::c_int;
/* Default: Do not restore mtime/atime. */
pub const ARCHIVE_EXTRACT_TIME: libc::c_int = 0x4 as libc::c_int;
/* Default: Replace existing files. */
pub const ARCHIVE_EXTRACT_NO_OVERWRITE: libc::c_int = 0x8 as libc::c_int;
/* Default: Try create first, unlink only if create fails with EEXIST. */
pub const ARCHIVE_EXTRACT_UNLINK: libc::c_int = 0x10 as libc::c_int;
/* Default: Do not restore ACLs. */
pub const ARCHIVE_EXTRACT_ACL: libc::c_int = 0x20 as libc::c_int;
/* Default: Do not restore fflags. */
pub const ARCHIVE_EXTRACT_FFLAGS: libc::c_int = 0x40 as libc::c_int;
/* Default: Do not restore xattrs. */
pub const ARCHIVE_EXTRACT_XATTR: libc::c_int = 0x80 as libc::c_int;
/* Default: Do not try to guard against extracts redirected by symlinks. */
/* Note: With ARCHIVE_EXTRACT_UNLINK, will remove any intermediate symlink. */
pub const ARCHIVE_EXTRACT_SECURE_SYMLINKS: libc::c_int = 0x100 as libc::c_int;
/* Default: Do not reject entries with '..' as path elements. */
pub const ARCHIVE_EXTRACT_SECURE_NODOTDOT: libc::c_int = 0x200 as libc::c_int;
/* Default: Create parent directories as needed. */
pub const ARCHIVE_EXTRACT_NO_AUTODIR: libc::c_int = 0x400 as libc::c_int;
/* Default: Overwrite files, even if one on disk is newer. */
pub const ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER: libc::c_int = 0x800 as libc::c_int;
/* Detect blocks of 0 and write holes instead. */
pub const ARCHIVE_EXTRACT_SPARSE: libc::c_int = 0x1000 as libc::c_int;
/* Default: Do not restore Mac extended metadata. */
/* This has no effect except on Mac OS. */
pub const ARCHIVE_EXTRACT_MAC_METADATA: libc::c_int = 0x2000 as libc::c_int;
/* Default: Use HFS+ compression if it was compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not use HFS+ compression if it was not compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
pub const ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED: libc::c_int = 0x8000 as libc::c_int;
/* Default: Do not reject entries with absolute paths */
pub const ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS: libc::c_int = 0x10000 as libc::c_int;
/* Default: Do not clear no-change flags when unlinking object */
pub const ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS: libc::c_int = 0x20000 as libc::c_int;
/* Default: Do not extract atomically (using rename) */
pub const ARCHIVE_EXTRACT_SAFE_WRITES: libc::c_int = 0x40000 as libc::c_int;
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
pub const AE_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
pub const ARCHIVE_STATE_DATA: libc::c_uint = 4 as libc::c_uint;
pub const ARCHIVE_WRITE_DISK_MAGIC: libc::c_uint = 0xc001b0c5 as libc::c_uint;
pub const ARCHIVE_STATE_HEADER: libc::c_uint = 2 as libc::c_uint;
/* TODO: Support Mac OS 'quarantine' feature.  This is really just a
 * standard tag to mark files that have been downloaded as "tainted".
 * On Mac OS, we should mark the extracted files as tainted if the
 * archive being read was tainted.  Windows has a similar feature; we
 * should investigate ways to support this generically. */
pub const O_BINARY: libc::c_int = 0 as libc::c_int;
/*
 * We use a bitmask to track which operations remain to be done for
 * this file.  In particular, this helps us avoid unnecessary
 * operations when it's possible to take care of one step as a
 * side-effect of another.  For example, mkdir() can specify the mode
 * for the newly-created object but symlink() cannot.  This means we
 * can skip chmod() if mkdir() succeeded, but we must explicitly
 * chmod() if we're trying to create a directory that already exists
 * (mkdir() failed) or if we're restoring a symlink.  Similarly, we
 * need to verify UID/GID before trying to restore SUID/SGID bits;
 * that verification can occur explicitly through a stat() call or
 * implicitly because of a successful chown() call.
 */
pub const TODO_MODE_FORCE: libc::c_int = 0x40000000 as libc::c_int;
pub const TODO_MODE_BASE: libc::c_int = 0x20000000 as libc::c_int;
pub const TODO_SUID: libc::c_int = 0x10000000 as libc::c_int;
pub const TODO_SUID_CHECK: libc::c_int = 0x8000000 as libc::c_int;
pub const TODO_SGID: libc::c_int = 0x4000000 as libc::c_int;
pub const TODO_SGID_CHECK: libc::c_int = 0x2000000 as libc::c_int;
pub const TODO_APPLEDOUBLE: libc::c_int = 0x1000000 as libc::c_int;
pub const TODO_MODE: libc::c_int = TODO_MODE_BASE | TODO_SUID | TODO_SGID;
pub const TODO_TIMES: libc::c_int = ARCHIVE_EXTRACT_TIME;
pub const TODO_OWNER: libc::c_int = ARCHIVE_EXTRACT_OWNER;
pub const TODO_FFLAGS: libc::c_int = ARCHIVE_EXTRACT_FFLAGS;
pub const TODO_ACLS: libc::c_int = ARCHIVE_EXTRACT_ACL;
pub const TODO_XATTR: libc::c_int = ARCHIVE_EXTRACT_XATTR;
pub const TODO_MAC_METADATA: libc::c_int = ARCHIVE_EXTRACT_MAC_METADATA;
pub const TODO_HFS_COMPRESSION: libc::c_int = ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED;
/*
 * Default mode for dirs created automatically (will be modified by umask).
 * Note that POSIX specifies 0777 for implicitly-created dirs, "modified
 * by the process' file creation mask."
 */
pub const DEFAULT_DIR_MODE: libc::c_int = 0o777 as libc::c_int;
/*
 * Dir modes are restored in two steps:  During the extraction, the permissions
 * in the archive are modified to match the following limits.  During
 * the post-extract fixup pass, the permissions from the archive are
 * applied.
 */
pub const MINIMUM_DIR_MODE: libc::c_int = 0o700 as libc::c_int;
pub const MAXIMUM_DIR_MODE: libc::c_int = 0o775 as libc::c_int;
unsafe extern "C" fn la_mktemp(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut oerrno: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut mode: mode_t = 0;
    (*a)._tmpname_data.length = 0 as libc::c_int as size_t;
    archive_string_sprintf(
        &mut (*a)._tmpname_data as *mut archive_string,
        b"%s.XXXXXX\x00" as *const u8 as *const libc::c_char,
        (*a).name,
    );
    (*a).tmpname = (*a)._tmpname_data.s;
    fd = __archive_mkstemp((*a).tmpname);
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    mode = (*a).mode & 0o777 as libc::c_int as libc::c_uint & !(*a).user_umask;
    if fchmod(fd, mode) == -(1 as libc::c_int) {
        oerrno = errno;
        close(fd);
        errno = oerrno;
        return -(1 as libc::c_int);
    }
    return fd;
}
/*
 * Maximum uncompressed size of a decmpfs block.
 */
/*
 * HFS+ compression type.
 */
/* Compressed data in xattr. */
/* Compressed data in resource fork. */
/*
 * HFS+ compression resource fork.
 */
/* Base size of Resource fork header. */
/* Size of Resource fork footer. */
/* Size to write compressed data to resource fork. */
/* decmpfs definitions. */
unsafe extern "C" fn la_opendirat(
    mut fd: libc::c_int,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let flags: libc::c_int = O_CLOEXEC | O_BINARY | O_DIRECTORY | O_PATH;
    return openat(fd, path, flags);
}
unsafe extern "C" fn lazy_stat(mut a: *mut archive_write_disk) -> libc::c_int {
    if !(*a).pst.is_null() {
        /* Already have stat() data available. */
        return 0 as libc::c_int;
    }
    if (*a).fd >= 0 as libc::c_int && fstat((*a).fd, &mut (*a).st) == 0 as libc::c_int {
        (*a).pst = &mut (*a).st;
        return 0 as libc::c_int;
    }
    /*
     * XXX At this point, symlinks should not be hit, otherwise
     * XXX a race occurred.  Do we want to check explicitly for that?
     */
    if lstat((*a).name, &mut (*a).st) == 0 as libc::c_int {
        (*a).pst = &mut (*a).st;
        return 0 as libc::c_int;
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        errno,
        b"Couldn\'t stat file\x00" as *const u8 as *const libc::c_char,
    );
    return -(20 as libc::c_int);
}
unsafe extern "C" fn archive_write_disk_vtable() -> *mut archive_vtable {
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
        av.archive_close =
            Some(_archive_write_disk_close as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_filter_bytes = Some(
            _archive_write_disk_filter_bytes
                as unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t,
        );
        av.archive_free =
            Some(_archive_write_disk_free as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_write_header = Some(
            _archive_write_disk_header
                as unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int,
        );
        av.archive_write_finish_entry = Some(
            _archive_write_disk_finish_entry
                as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
        );
        av.archive_write_data = Some(
            _archive_write_disk_data
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> ssize_t,
        );
        av.archive_write_data_block = Some(
            _archive_write_disk_data_block
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_void,
                    _: size_t,
                    _: int64_t,
                ) -> ssize_t,
        );
        inited = 1 as libc::c_int
    }
    return &mut av;
}
unsafe extern "C" fn _archive_write_disk_filter_bytes(
    mut _a: *mut archive,
    mut n: libc::c_int,
) -> int64_t {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    /* UNUSED */
    if n == -(1 as libc::c_int) || n == 0 as libc::c_int {
        return (*a).total_bytes_written;
    }
    return -(1 as libc::c_int) as int64_t;
}
/* Set flags to control how the next item gets created.
 * This accepts a bitmask of ARCHIVE_EXTRACT_XXX flags defined above. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_set_options(
    mut _a: *mut archive,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    (*a).flags = flags;
    return 0 as libc::c_int;
}
/*
 * Extract this entry to disk.
 *
 * TODO: Validate hardlinks.  According to the standards, we're
 * supposed to check each extracted hardlink and squawk if it refers
 * to a file that we didn't restore.  I'm not entirely convinced this
 * is a good idea, but more importantly: Is there any way to validate
 * hardlinks without keeping a complete list of filenames from the
 * entire archive?? Ugh.
 *
 */
unsafe extern "C" fn _archive_write_disk_header(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut fe: *mut fixup_entry = 0 as *mut fixup_entry;
    let mut ret: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_write_disk_header\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_clear_error(&mut (*a).archive);
    if (*a).archive.state & ARCHIVE_STATE_DATA != 0 {
        r = _archive_write_disk_finish_entry(&mut (*a).archive);
        if r == ARCHIVE_FATAL {
            return r;
        }
    }
    /* Set up for this particular entry. */
    (*a).pst = NULL as *mut stat;
    (*a).current_fixup = NULL as *mut fixup_entry;
    (*a).deferred = 0 as libc::c_int;
    if !(*a).entry.is_null() {
        archive_entry_free((*a).entry);
        (*a).entry = NULL as *mut archive_entry
    }
    (*a).entry = archive_entry_clone(entry);
    (*a).fd = -(1 as libc::c_int);
    (*a).fd_offset = 0 as libc::c_int as int64_t;
    (*a).offset = 0 as libc::c_int as int64_t;
    (*a).restore_pwd = -(1 as libc::c_int);
    (*a).uid = (*a).user_uid;
    (*a).mode = archive_entry_mode((*a).entry);
    if archive_entry_size_is_set((*a).entry) != 0 {
        (*a).filesize = archive_entry_size((*a).entry)
    } else {
        (*a).filesize = -(1 as libc::c_int) as int64_t
    }
    (*a)._name_data.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*a)._name_data,
        archive_entry_pathname((*a).entry) as *const libc::c_void,
        (if archive_entry_pathname((*a).entry).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(archive_entry_pathname((*a).entry))
        }),
    );
    (*a).name = (*a)._name_data.s;
    archive_clear_error(&mut (*a).archive);
    /*
     * Clean up the requested path.  This is necessary for correct
     * dir restores; the dir restore logic otherwise gets messed
     * up by nonsense like "dir/.".
     */
    ret = cleanup_pathname(a);
    if ret != ARCHIVE_OK {
        return ret;
    }
    /*
     * Query the umask so we get predictable mode settings.
     * This gets done on every call to _write_header in case the
     * user edits their umask during the extraction for some
     * reason.
     */
    (*a).user_umask = umask(0 as libc::c_int as __mode_t);
    umask((*a).user_umask);
    /* Figure out what we need to do for this entry. */
    (*a).todo = TODO_MODE_BASE; /* Be pushy about permissions. */
    if (*a).flags & ARCHIVE_EXTRACT_PERM != 0 {
        (*a).todo |= TODO_MODE_FORCE;
        /*
         * SGID requires an extra "check" step because we
         * cannot easily predict the GID that the system will
         * assign.  (Different systems assign GIDs to files
         * based on a variety of criteria, including process
         * credentials and the gid of the enclosing
         * directory.)  We can only restore the SGID bit if
         * the file has the right GID, and we only know the
         * GID if we either set it (see set_ownership) or if
         * we've actually called stat() on the file after it
         * was restored.  Since there are several places at
         * which we might verify the GID, we need a TODO bit
         * to keep track.
         */
        if (*a).mode & S_ISGID as libc::c_uint != 0 {
            (*a).todo |= TODO_SGID | TODO_SGID_CHECK
        }
        /*
         * Verifying the SUID is simpler, but can still be
         * done in multiple ways, hence the separate "check" bit.
         */
        if (*a).mode & S_ISUID as libc::c_uint != 0 {
            (*a).todo |= TODO_SUID | TODO_SUID_CHECK
        }
    } else {
        /*
         * User didn't request full permissions, so don't
         * restore SUID, SGID bits and obey umask.
         */
        (*a).mode &= !S_ISUID as libc::c_uint;
        (*a).mode &= !S_ISGID as libc::c_uint;
        (*a).mode &= !S_ISVTX as libc::c_uint;
        (*a).mode &= !(*a).user_umask
    }
    if (*a).flags & ARCHIVE_EXTRACT_OWNER != 0 {
        (*a).todo |= TODO_OWNER
    }
    if (*a).flags & ARCHIVE_EXTRACT_TIME != 0 {
        (*a).todo |= TODO_TIMES
    }
    if (*a).flags & ARCHIVE_EXTRACT_ACL != 0 {
        if archive_entry_filetype((*a).entry) == AE_IFDIR as mode_t {
            (*a).deferred |= TODO_ACLS
        } else {
            (*a).todo |= TODO_ACLS
        }
    }
    if (*a).flags & ARCHIVE_EXTRACT_MAC_METADATA != 0 {
        if archive_entry_filetype((*a).entry) == AE_IFDIR as mode_t {
            (*a).deferred |= TODO_MAC_METADATA
        } else {
            (*a).todo |= TODO_MAC_METADATA
        }
    }
    if (*a).flags & ARCHIVE_EXTRACT_XATTR != 0 {
        (*a).todo |= TODO_XATTR
    }
    if (*a).flags & ARCHIVE_EXTRACT_FFLAGS != 0 {
        (*a).todo |= TODO_FFLAGS
    }
    if (*a).flags & ARCHIVE_EXTRACT_SECURE_SYMLINKS != 0 {
        ret = check_symlinks(a);
        if ret != ARCHIVE_OK {
            return ret;
        }
    }
    /* If path exceeds PATH_MAX, shorten the path. */
    edit_deep_directories(a);
    ret = restore_entry(a);
    /*
     * TODO: There are rumours that some extended attributes must
     * be restored before file data is written.  If this is true,
     * then we either need to write all extended attributes both
     * before and after restoring the data, or find some rule for
     * determining which must go first and which last.  Due to the
     * many ways people are using xattrs, this may prove to be an
     * intractable problem.
     */
    /* If we changed directory above, restore it here. */
    if (*a).restore_pwd >= 0 as libc::c_int {
        r = fchdir((*a).restore_pwd);
        if r != 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"chdir() failure\x00" as *const u8 as *const libc::c_char,
            );
            ret = ARCHIVE_FATAL
        }
        close((*a).restore_pwd);
        (*a).restore_pwd = -(1 as libc::c_int)
    }
    /*
     * Fixup uses the unedited pathname from archive_entry_pathname(),
     * because it is relative to the base dir and the edited path
     * might be relative to some intermediate dir as a result of the
     * deep restore logic.
     */
    if (*a).deferred & TODO_MODE != 0 {
        fe = current_fixup(a, archive_entry_pathname(entry));
        if fe.is_null() {
            return -(30 as libc::c_int);
        }
        (*fe).fixup |= TODO_MODE_BASE;
        (*fe).mode = (*a).mode
    }
    if (*a).deferred & TODO_TIMES != 0
        && (archive_entry_mtime_is_set(entry) != 0 || archive_entry_atime_is_set(entry) != 0)
    {
        fe = current_fixup(a, archive_entry_pathname(entry));
        if fe.is_null() {
            return -(30 as libc::c_int);
        }
        (*fe).mode = (*a).mode;
        (*fe).fixup |= TODO_TIMES;
        if archive_entry_atime_is_set(entry) != 0 {
            (*fe).atime = archive_entry_atime(entry);
            (*fe).atime_nanos = archive_entry_atime_nsec(entry) as libc::c_ulong
        } else {
            /* If atime is unset, use start time. */
            (*fe).atime = (*a).start_time;
            (*fe).atime_nanos = 0 as libc::c_int as libc::c_ulong
        }
        if archive_entry_mtime_is_set(entry) != 0 {
            (*fe).mtime = archive_entry_mtime(entry);
            (*fe).mtime_nanos = archive_entry_mtime_nsec(entry) as libc::c_ulong
        } else {
            /* If mtime is unset, use start time. */
            (*fe).mtime = (*a).start_time;
            (*fe).mtime_nanos = 0 as libc::c_int as libc::c_ulong
        }
        if archive_entry_birthtime_is_set(entry) != 0 {
            (*fe).birthtime = archive_entry_birthtime(entry);
            (*fe).birthtime_nanos = archive_entry_birthtime_nsec(entry) as libc::c_ulong
        } else {
            /* If birthtime is unset, use mtime. */
            (*fe).birthtime = (*fe).mtime;
            (*fe).birthtime_nanos = (*fe).mtime_nanos
        }
    }
    if (*a).deferred & TODO_ACLS != 0 {
        fe = current_fixup(a, archive_entry_pathname(entry));
        if fe.is_null() {
            return -(30 as libc::c_int);
        }
        (*fe).fixup |= TODO_ACLS;
        archive_acl_copy(&mut (*fe).acl, archive_entry_acl(entry));
    }
    if (*a).deferred & TODO_MAC_METADATA != 0 {
        let mut metadata: *const libc::c_void = 0 as *const libc::c_void;
        let mut metadata_size: size_t = 0;
        metadata = archive_entry_mac_metadata((*a).entry, &mut metadata_size);
        if metadata != NULL as *const libc::c_void
            && metadata_size > 0 as libc::c_int as libc::c_ulong
        {
            fe = current_fixup(a, archive_entry_pathname(entry));
            if fe.is_null() {
                return -(30 as libc::c_int);
            }
            (*fe).mac_metadata = malloc(metadata_size);
            if !(*fe).mac_metadata.is_null() {
                memcpy((*fe).mac_metadata, metadata, metadata_size);
                (*fe).mac_metadata_size = metadata_size;
                (*fe).fixup |= TODO_MAC_METADATA
            }
        }
    }
    if (*a).deferred & TODO_FFLAGS != 0 {
        fe = current_fixup(a, archive_entry_pathname(entry));
        if fe.is_null() {
            return -(30 as libc::c_int);
        }
        (*fe).fixup |= TODO_FFLAGS
        /* TODO: Complete this.. defer fflags from below. */
    }
    /* We've created the object and are ready to pour data into it. */
    if ret >= ARCHIVE_WARN {
        (*a).archive.state = ARCHIVE_STATE_DATA
    }
    /*
     * If it's not open, tell our client not to try writing.
     * In particular, dirs, links, etc, don't get written to.
     */
    if (*a).fd < 0 as libc::c_int {
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
        (*a).filesize = 0 as libc::c_int as int64_t
    }
    return ret;
}
/* This file will not be overwritten. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_set_skip_file(
    mut _a: *mut archive,
    mut d: la_int64_t,
    mut i: la_int64_t,
) -> libc::c_int {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_disk_set_skip_file\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).skip_file_set = 1 as libc::c_int;
    (*a).skip_file_dev = d;
    (*a).skip_file_ino = i;
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_data_block(
    mut a: *mut archive_write_disk,
    mut buff: *const libc::c_char,
    mut size: size_t,
) -> ssize_t {
    let mut start_size: uint64_t = size;
    let mut bytes_written: ssize_t = 0 as libc::c_int as ssize_t;
    let mut block_size: ssize_t = 0 as libc::c_int as ssize_t;
    let mut bytes_to_write: ssize_t = 0;
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    if (*a).filesize == 0 as libc::c_int as libc::c_long || (*a).fd < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            0 as libc::c_int,
            b"Attempt to write to an empty file\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int) as ssize_t;
    }
    if (*a).flags & ARCHIVE_EXTRACT_SPARSE != 0 {
        let mut r: libc::c_int = 0;
        r = lazy_stat(a);
        if r != ARCHIVE_OK {
            return r as ssize_t;
        }
        block_size = (*(*a).pst).st_blksize
    }
    /* If this write would run beyond the file size, truncate it. */
    if (*a).filesize >= 0 as libc::c_int as libc::c_long
        && ((*a).offset as libc::c_ulong).wrapping_add(size) as int64_t > (*a).filesize
    {
        size = ((*a).filesize - (*a).offset) as size_t;
        start_size = size
    }
    /* Write the data. */
    while size > 0 as libc::c_int as libc::c_ulong {
        if block_size == 0 as libc::c_int as libc::c_long {
            bytes_to_write = size as ssize_t
        } else {
            /* We're sparsifying the file. */
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            let mut end: *const libc::c_char = 0 as *const libc::c_char;
            let mut block_end: int64_t = 0;
            /* Skip leading zero bytes. */
            p = buff;
            end = buff.offset(size as isize);
            while p < end {
                if *p as libc::c_int != '\u{0}' as i32 {
                    break;
                }
                p = p.offset(1)
            }
            (*a).offset += p.offset_from(buff) as libc::c_long;
            size = (size as libc::c_ulong)
                .wrapping_sub(p.offset_from(buff) as libc::c_long as libc::c_ulong)
                as size_t as size_t;
            buff = p;
            if size == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            /* Calculate next block boundary after offset. */
            block_end = ((*a).offset / block_size + 1 as libc::c_int as libc::c_long) * block_size;
            /* If the adjusted write would cross block boundary,
             * truncate it to the block boundary. */
            bytes_to_write = size as ssize_t;
            if (*a).offset + bytes_to_write > block_end {
                bytes_to_write = block_end - (*a).offset
            }
        }
        /* Seek if necessary to the specified offset. */
        if (*a).offset != (*a).fd_offset {
            if lseek((*a).fd, (*a).offset, SEEK_SET) < 0 as libc::c_int as libc::c_long {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Seek failed\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int) as ssize_t;
            }
            (*a).fd_offset = (*a).offset
        }
        bytes_written = write(
            (*a).fd,
            buff as *const libc::c_void,
            bytes_to_write as size_t,
        );
        if bytes_written < 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Write failed\x00" as *const u8 as *const libc::c_char,
            );
            return -(20 as libc::c_int) as ssize_t;
        }
        buff = buff.offset(bytes_written as isize);
        size = (size as libc::c_ulong).wrapping_sub(bytes_written as libc::c_ulong) as size_t
            as size_t;
        (*a).total_bytes_written += bytes_written;
        (*a).offset += bytes_written;
        (*a).fd_offset = (*a).offset
    }
    return start_size.wrapping_sub(size) as ssize_t;
}
unsafe extern "C" fn hfs_write_data_block(
    mut a: *mut archive_write_disk,
    mut buff: *const libc::c_char,
    mut size: size_t,
) -> ssize_t {
    return write_data_block(a, buff, size);
}
unsafe extern "C" fn _archive_write_disk_data_block(
    mut _a: *mut archive,
    mut buff: *const libc::c_void,
    mut size: size_t,
    mut offset: int64_t,
) -> ssize_t {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut r: ssize_t = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_write_data_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL as ssize_t;
    }
    (*a).offset = offset;
    if (*a).todo & TODO_HFS_COMPRESSION != 0 {
        r = hfs_write_data_block(a, buff as *const libc::c_char, size)
    } else {
        r = write_data_block(a, buff as *const libc::c_char, size)
    }
    if r < ARCHIVE_OK as libc::c_long {
        return r;
    }
    if (r as size_t) < size {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            0 as libc::c_int,
            b"Too much data: Truncating file at %ju bytes\x00" as *const u8 as *const libc::c_char,
            (*a).filesize as uintmax_t,
        );
        return -(20 as libc::c_int) as ssize_t;
    }
    return 0 as libc::c_int as ssize_t;
}
unsafe extern "C" fn _archive_write_disk_data(
    mut _a: *mut archive,
    mut buff: *const libc::c_void,
    mut size: size_t,
) -> ssize_t {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_write_data\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL as ssize_t;
    }
    if (*a).todo & TODO_HFS_COMPRESSION != 0 {
        return hfs_write_data_block(a, buff as *const libc::c_char, size);
    }
    return write_data_block(a, buff as *const libc::c_char, size);
}
unsafe extern "C" fn _archive_write_disk_finish_entry(mut _a: *mut archive) -> libc::c_int {
    let mut current_block: u64;
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut ret: libc::c_int = ARCHIVE_OK;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_write_finish_entry\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state & ARCHIVE_STATE_HEADER != 0 {
        return 0 as libc::c_int;
    }
    archive_clear_error(&mut (*a).archive);
    /* Pad or truncate file to the right size. */
    if !((*a).fd < 0 as libc::c_int) {
        if !((*a).filesize < 0 as libc::c_int as libc::c_long) {
            if !((*a).fd_offset == (*a).filesize) {
                if ftruncate((*a).fd, (*a).filesize) == -(1 as libc::c_int)
                    && (*a).filesize == 0 as libc::c_int as libc::c_long
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        errno,
                        b"File size could not be restored\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
                /*
                 * Not all platforms implement the XSI option to
                 * extend files via ftruncate.  Stat() the file again
                 * to see what happened.
                 */
                (*a).pst = NULL as *mut stat;
                ret = lazy_stat(a);
                if ret != ARCHIVE_OK {
                    return ret;
                }
                /* We can use lseek()/write() to extend the file if
                 * ftruncate didn't work or isn't available. */
                if (*a).st.st_size < (*a).filesize {
                    let nul: libc::c_char = '\u{0}' as i32 as libc::c_char;
                    if lseek(
                        (*a).fd,
                        (*a).filesize - 1 as libc::c_int as libc::c_long,
                        SEEK_SET,
                    ) < 0 as libc::c_int as libc::c_long
                    {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            errno,
                            b"Seek failed\x00" as *const u8 as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    if write(
                        (*a).fd,
                        &nul as *const libc::c_char as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    ) < 0 as libc::c_int as libc::c_long
                    {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            errno,
                            b"Write to restore size failed\x00" as *const u8 as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    (*a).pst = NULL as *mut stat
                }
            }
        }
    }
    /* Restore metadata. */
    /*
     * This is specific to Mac OS X.
     * If the current file is an AppleDouble file, it should be
     * linked with the data fork file and remove it.
     */
    if (*a).todo & TODO_APPLEDOUBLE != 0 {
        let mut r2: libc::c_int = fixup_appledouble(a, (*a).name);
        if r2 == ARCHIVE_EOF {
            current_block = 16353819696627653807;
        } else {
            if r2 < ret {
                ret = r2
            }
            current_block = 2604890879466389055;
        }
    } else {
        current_block = 2604890879466389055;
    }
    match current_block {
        2604890879466389055 => {
            /*
             * Look up the "real" UID only if we're going to need it.
             * TODO: the TODO_SGID condition can be dropped here, can't it?
             */
            if (*a).todo & (TODO_OWNER | TODO_SUID | TODO_SGID) != 0 {
                (*a).uid = archive_write_disk_uid(
                    &mut (*a).archive,
                    archive_entry_uname((*a).entry),
                    archive_entry_uid((*a).entry),
                )
            }
            /* Look up the "real" GID only if we're going to need it. */
            /* TODO: the TODO_SUID condition can be dropped here, can't it? */
            if (*a).todo & (TODO_OWNER | TODO_SGID | TODO_SUID) != 0 {
                (*a).gid = archive_write_disk_gid(
                    &mut (*a).archive,
                    archive_entry_gname((*a).entry),
                    archive_entry_gid((*a).entry),
                )
            }
            /*
             * Restore ownership before set_mode tries to restore suid/sgid
             * bits.  If we set the owner, we know what it is and can skip
             * a stat() call to examine the ownership of the file on disk.
             */
            if (*a).todo & TODO_OWNER != 0 {
                let mut r2_0: libc::c_int = set_ownership(a);
                if r2_0 < ret {
                    ret = r2_0
                }
            }
            /*
             * HYPOTHESIS:
             * If we're not root, we won't be setting any security
             * attributes that may be wiped by the set_mode() routine
             * below.  We also can't set xattr on non-owner-writable files,
             * which may be the state after set_mode(). Perform
             * set_xattrs() first based on these constraints.
             */
            if (*a).user_uid != 0 as libc::c_int as libc::c_long && (*a).todo & TODO_XATTR != 0 {
                let mut r2_1: libc::c_int = set_xattrs(a);
                if r2_1 < ret {
                    ret = r2_1
                }
            }
            /*
             * set_mode must precede ACLs on systems such as Solaris and
             * FreeBSD where setting the mode implicitly clears extended ACLs
             */
            if (*a).todo & TODO_MODE != 0 {
                let mut r2_2: libc::c_int = set_mode(a, (*a).mode as libc::c_int);
                if r2_2 < ret {
                    ret = r2_2
                }
            }
            /*
             * Security-related extended attributes (such as
             * security.capability on Linux) have to be restored last,
             * since they're implicitly removed by other file changes.
             * We do this last only when root.
             */
            if (*a).user_uid == 0 as libc::c_int as libc::c_long && (*a).todo & TODO_XATTR != 0 {
                let mut r2_3: libc::c_int = set_xattrs(a);
                if r2_3 < ret {
                    ret = r2_3
                }
            }
            /*
             * Some flags prevent file modification; they must be restored after
             * file contents are written.
             */
            if (*a).todo & TODO_FFLAGS != 0 {
                let mut r2_4: libc::c_int = set_fflags(a);
                if r2_4 < ret {
                    ret = r2_4
                }
            }
            /*
             * Time must follow most other metadata;
             * otherwise atime will get changed.
             */
            if (*a).todo & TODO_TIMES != 0 {
                let mut r2_5: libc::c_int = set_times_from_entry(a);
                if r2_5 < ret {
                    ret = r2_5
                }
            }
            /*
             * Mac extended metadata includes ACLs.
             */
            if (*a).todo & TODO_MAC_METADATA != 0 {
                let mut metadata: *const libc::c_void = 0 as *const libc::c_void;
                let mut metadata_size: size_t = 0;
                metadata = archive_entry_mac_metadata((*a).entry, &mut metadata_size);
                if metadata != NULL as *const libc::c_void
                    && metadata_size > 0 as libc::c_int as libc::c_ulong
                {
                    let mut r2_6: libc::c_int = set_mac_metadata(
                        a,
                        archive_entry_pathname((*a).entry),
                        metadata,
                        metadata_size,
                    );
                    if r2_6 < ret {
                        ret = r2_6
                    }
                }
            }
            /*
             * ACLs must be restored after timestamps because there are
             * ACLs that prevent attribute changes (including time).
             */
            if (*a).todo & TODO_ACLS != 0 {
                let mut r2_7: libc::c_int = 0;
                r2_7 = archive_write_disk_set_acls(
                    &mut (*a).archive,
                    (*a).fd,
                    archive_entry_pathname((*a).entry),
                    archive_entry_acl((*a).entry),
                    archive_entry_mode((*a).entry),
                );
                if r2_7 < ret {
                    ret = r2_7
                }
            }
        }
        _ => {}
    }
    /* The current file has been successfully linked
     * with the data fork file and removed. So there
     * is nothing to do on the current file.  */
    /* If there's an fd, we can close it now. */
    if (*a).fd >= 0 as libc::c_int {
        close((*a).fd);
        (*a).fd = -(1 as libc::c_int);
        if !(*a).tmpname.is_null() {
            if rename((*a).tmpname, (*a).name) == -(1 as libc::c_int) {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Failed to rename temporary file\x00" as *const u8 as *const libc::c_char,
                );
                ret = ARCHIVE_FAILED;
                unlink((*a).tmpname);
            }
            (*a).tmpname = NULL as *mut libc::c_char
        }
    }
    /* If there's an entry, we can release it now. */
    archive_entry_free((*a).entry);
    (*a).entry = NULL as *mut archive_entry;
    (*a).archive.state = ARCHIVE_STATE_HEADER;
    return ret;
}
/*
 * If neither the default (naive) nor the standard (big) functions suit
 * your needs, you can write your own and register them.  Be sure to
 * include a cleanup function if you have allocated private data.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_set_group_lookup(
    mut _a: *mut archive,
    mut private_data: *mut libc::c_void,
    mut lookup_gid: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const libc::c_char,
            _: la_int64_t,
        ) -> la_int64_t,
    >,
    mut cleanup_gid: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
) -> libc::c_int {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_disk_set_group_lookup\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).cleanup_gid.is_some() && !(*a).lookup_gid_data.is_null() {
        (*a).cleanup_gid.expect("non-null function pointer")((*a).lookup_gid_data);
    }
    (*a).lookup_gid = lookup_gid;
    (*a).cleanup_gid = cleanup_gid;
    (*a).lookup_gid_data = private_data;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_set_user_lookup(
    mut _a: *mut archive,
    mut private_data: *mut libc::c_void,
    mut lookup_uid: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const libc::c_char, _: int64_t) -> int64_t,
    >,
    mut cleanup_uid: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
) -> libc::c_int {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_disk_set_user_lookup\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).cleanup_uid.is_some() && !(*a).lookup_uid_data.is_null() {
        (*a).cleanup_uid.expect("non-null function pointer")((*a).lookup_uid_data);
    }
    (*a).lookup_uid = lookup_uid;
    (*a).cleanup_uid = cleanup_uid;
    (*a).lookup_uid_data = private_data;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_gid(
    mut _a: *mut archive,
    mut name: *const libc::c_char,
    mut id: la_int64_t,
) -> la_int64_t {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_disk_gid\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL as la_int64_t;
    }
    if (*a).lookup_gid.is_some() {
        return (*a).lookup_gid.expect("non-null function pointer")((*a).lookup_gid_data, name, id);
    }
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_uid(
    mut _a: *mut archive,
    mut name: *const libc::c_char,
    mut id: la_int64_t,
) -> la_int64_t {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_write_disk_uid\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL as la_int64_t;
    }
    if (*a).lookup_uid.is_some() {
        return (*a).lookup_uid.expect("non-null function pointer")((*a).lookup_uid_data, name, id);
    }
    return id;
}
/*-
 * ARCHIVE_WRITE_DISK API
 *
 * To create objects on disk:
 *   1) Ask archive_write_disk_new for a new archive_write_disk object.
 *   2) Set any global properties.  In particular, you probably
 *      want to set the options.
 *   3) For each entry:
 *      - construct an appropriate struct archive_entry structure
 *      - archive_write_header to create the file/dir/etc on disk
 *      - archive_write_data to write the entry data
 *   4) archive_write_free to cleanup the writer and release resources
 *
 * In particular, you can use this in conjunction with archive_read()
 * to pull entries out of an archive and create them on disk.
 */
/*
 * Create a new archive_write_disk object and initialize it with global state.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_new() -> *mut archive {
    let mut a: *mut archive_write_disk = 0 as *mut archive_write_disk;
    a = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_write_disk>() as libc::c_ulong,
    ) as *mut archive_write_disk;
    if a.is_null() {
        return 0 as *mut archive;
    }
    (*a).archive.magic = ARCHIVE_WRITE_DISK_MAGIC;
    /* We're ready to write a header immediately. */
    (*a).archive.state = ARCHIVE_STATE_HEADER;
    (*a).archive.vtable = archive_write_disk_vtable();
    (*a).start_time = time(NULL as *mut time_t);
    /* Query and restore the umask. */
    (*a).user_umask = umask(0 as libc::c_int as __mode_t);
    umask((*a).user_umask);
    (*a).user_uid = geteuid() as int64_t;
    /* HAVE_GETEUID */
    if archive_string_ensure(&mut (*a).path_safe, 512 as libc::c_int as size_t).is_null() {
        free(a as *mut libc::c_void);
        return 0 as *mut archive;
    }
    (*a).decmpfs_compression_level = 5 as libc::c_int;
    return &mut (*a).archive;
}
/*
 * If pathname is longer than PATH_MAX, chdir to a suitable
 * intermediate dir and edit the path down to a shorter suffix.  Note
 * that this routine never returns an error; if the chdir() attempt
 * fails for any reason, we just go ahead with the long pathname.  The
 * object creation is likely to fail, but any error will get handled
 * at that time.
 */
unsafe extern "C" fn edit_deep_directories(mut a: *mut archive_write_disk) {
    let mut ret: libc::c_int = 0;
    let mut tail: *mut libc::c_char = (*a).name;
    /* If path is short, avoid the open() below. */
    if strlen(tail) < PATH_MAX as libc::c_ulong {
        return;
    }
    /* Try to record our starting dir. */
    (*a).restore_pwd = la_opendirat(AT_FDCWD, b".\x00" as *const u8 as *const libc::c_char);
    __archive_ensure_cloexec_flag((*a).restore_pwd);
    if (*a).restore_pwd < 0 as libc::c_int {
        return;
    }
    /* As long as the path is too long... */
    while strlen(tail) >= PATH_MAX as libc::c_ulong {
        /* Locate a dir prefix shorter than PATH_MAX. */
        tail = tail.offset((PATH_MAX - 8 as libc::c_int) as isize);
        while tail > (*a).name && *tail as libc::c_int != '/' as i32 {
            tail = tail.offset(-1)
        }
        /* Exit if we find a too-long path component. */
        if tail <= (*a).name {
            return;
        }
        /* Create the intermediate dir and chdir to it. */
        *tail = '\u{0}' as i32 as libc::c_char; /* Terminate dir portion */
        ret = create_dir(a, (*a).name); /* Restore the / we removed. */
        if ret == ARCHIVE_OK && chdir((*a).name) != 0 as libc::c_int {
            ret = ARCHIVE_FAILED
        }
        *tail = '/' as i32 as libc::c_char;
        if ret != ARCHIVE_OK {
            return;
        }
        tail = tail.offset(1);
        /* The chdir() succeeded; we've now shortened the path. */
        (*a).name = tail
    }
}
/*
 * The main restore function.
 */
unsafe extern "C" fn restore_entry(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut ret: libc::c_int = ARCHIVE_OK;
    let mut en: libc::c_int = 0;
    if (*a).flags & ARCHIVE_EXTRACT_UNLINK != 0
        && !((*a).mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint)
    {
        /*
         * TODO: Fix this.  Apparently, there are platforms
         * that still allow root to hose the entire filesystem
         * by unlinking a dir.  The S_ISDIR() test above
         * prevents us from using unlink() here if the new
         * object is a dir, but that doesn't mean the old
         * object isn't a dir.
         */
        if (*a).flags & ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS != 0 {
            clear_nochange_fflags(a);
        }
        if unlink((*a).name) == 0 as libc::c_int {
            /* We removed it, reset cached stat. */
            (*a).pst = NULL as *mut stat
        } else if !(errno == ENOENT) {
            if rmdir((*a).name) == 0 as libc::c_int {
                /* It was a dir, but now it's gone. */
                (*a).pst = NULL as *mut stat
            } else {
                /* We tried, but couldn't get rid of it. */
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Could not unlink\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
        }
    }
    /* Try creating it first; if this fails, we'll try to recover. */
    en = create_filesystem_object(a);
    if (en == ENOTDIR || en == ENOENT) && (*a).flags & ARCHIVE_EXTRACT_NO_AUTODIR == 0 {
        /* If the parent dir doesn't exist, try creating it. */
        create_parent_dir(a, (*a).name);
        /* Now try to create the object again. */
        en = create_filesystem_object(a)
    }
    if en == ENOENT && !archive_entry_hardlink((*a).entry).is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            en,
            b"Hard-link target \'%s\' does not exist.\x00" as *const u8 as *const libc::c_char,
            archive_entry_hardlink((*a).entry),
        );
        return -(25 as libc::c_int);
    }
    if (en == EISDIR || en == EEXIST) && (*a).flags & ARCHIVE_EXTRACT_NO_OVERWRITE != 0 {
        /* If we're not overwriting, we're done. */
        if (*a).mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint {
            /* Don't overwrite any settings on existing directories. */
            (*a).todo = 0 as libc::c_int
        }
        archive_entry_unset_size((*a).entry);
        return 0 as libc::c_int;
    }
    /*
     * Some platforms return EISDIR if you call
     * open(O_WRONLY | O_EXCL | O_CREAT) on a directory, some
     * return EEXIST.  POSIX is ambiguous, requiring EISDIR
     * for open(O_WRONLY) on a dir and EEXIST for open(O_EXCL | O_CREAT)
     * on an existing item.
     */
    if en == EISDIR {
        /* A dir is in the way of a non-dir, rmdir it. */
        if rmdir((*a).name) != 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t remove already-existing dir\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        (*a).pst = NULL as *mut stat;
        /* Try again. */
        en = create_filesystem_object(a)
    } else if en == EEXIST {
        /*
         * We know something is in the way, but we don't know what;
         * we need to find out before we go any further.
         */
        let mut r: libc::c_int = 0 as libc::c_int;
        /*
         * The SECURE_SYMLINKS logic has already removed a
         * symlink to a dir if the client wants that.  So
         * follow the symlink if we're creating a dir.
         */
        if (*a).mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint {
            r = stat((*a).name, &mut (*a).st)
        }
        /*
         * If it's not a dir (or it's a broken symlink),
         * then don't follow it.
         */
        if r != 0 as libc::c_int
            || !((*a).mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint)
        {
            r = lstat((*a).name, &mut (*a).st)
        }
        if r != 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t stat existing object\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        /*
         * NO_OVERWRITE_NEWER doesn't apply to directories.
         */
        if (*a).flags & ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER != 0
            && !((*a).st.st_mode & __S_IFMT as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
        {
            if older(&mut (*a).st, (*a).entry) == 0 {
                archive_entry_unset_size((*a).entry);
                return 0 as libc::c_int;
            }
        }
        /* If it's our archive, we're done. */
        if (*a).skip_file_set != 0
            && (*a).st.st_dev == (*a).skip_file_dev as dev_t
            && (*a).st.st_ino == (*a).skip_file_ino as ino_t
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                0 as libc::c_int,
                b"Refusing to overwrite archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        if !((*a).st.st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint) {
            if (*a).flags & ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS != 0 {
                clear_nochange_fflags(a);
            }
            if (*a).flags & ARCHIVE_EXTRACT_SAFE_WRITES != 0
                && (*a).st.st_mode & __S_IFMT as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
            {
                /* Use a temporary file to extract */
                (*a).fd = la_mktemp(a);
                if (*a).fd == -(1 as libc::c_int) {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        errno,
                        b"Can\'t create temporary file\x00" as *const u8 as *const libc::c_char,
                    );
                    return ARCHIVE_FAILED;
                }
                (*a).pst = NULL as *mut stat;
                en = 0 as libc::c_int
            } else {
                /* A non-dir is in the way, unlink it. */
                if unlink((*a).name) != 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        errno,
                        b"Can\'t unlink already-existing object\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
                (*a).pst = NULL as *mut stat;
                /* Try again. */
                en = create_filesystem_object(a)
            }
        } else if !((*a).mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint)
        {
            /* A dir is in the way of a non-dir, rmdir it. */
            if (*a).flags & ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS != 0 {
                clear_nochange_fflags(a);
            }
            if rmdir((*a).name) != 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Can\'t replace existing directory with non-directory\x00" as *const u8
                        as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            /* Try again. */
            en = create_filesystem_object(a)
        } else {
            /*
             * There's a dir in the way of a dir.  Don't
             * waste time with rmdir()/mkdir(), just fix
             * up the permissions on the existing dir.
             * Note that we don't change perms on existing
             * dirs unless _EXTRACT_PERM is specified.
             */
            if (*a).mode != (*a).st.st_mode && (*a).todo & TODO_MODE_FORCE != 0 {
                (*a).deferred |= (*a).todo & TODO_MODE
            }
            /* Forget the EEXIST. */
            en = 0 as libc::c_int
        }
    }
    if en != 0 {
        /* Ownership doesn't need deferred fixup. */
        /* Everything failed; give up here. */
        if (*a).archive.error.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                en,
                b"Can\'t create \'%s\'\x00" as *const u8 as *const libc::c_char,
                (*a).name,
            ); /* Cached stat data no longer valid. */
        }
        return -(25 as libc::c_int);
    }
    (*a).pst = NULL as *mut stat;
    return ret;
}
/*
 * Returns 0 if creation succeeds, or else returns errno value from
 * the failed system call.   Note:  This function should only ever perform
 * a single system call.
 */
unsafe extern "C" fn create_filesystem_object(mut a: *mut archive_write_disk) -> libc::c_int {
    /* Create the entry. */
    let mut linkname: *const libc::c_char = 0 as *const libc::c_char;
    let mut final_mode: mode_t = 0;
    let mut mode: mode_t = 0;
    let mut r: libc::c_int = 0;
    /* these for check_symlinks_fsobj */
    let mut linkname_copy: *mut libc::c_char = 0 as *mut libc::c_char; /* non-const copy of linkname */
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
    let mut error_string: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut error_number: libc::c_int = 0;
    /* We identify hard/symlinks according to the link names. */
    /* Since link(2) and symlink(2) don't handle modes, we're done here. */
    linkname = archive_entry_hardlink((*a).entry);
    if !linkname.is_null() {
        error_string.s = NULL as *mut libc::c_char;
        error_string.length = 0 as libc::c_int as size_t;
        error_string.buffer_length = 0 as libc::c_int as size_t;
        linkname_copy = strdup(linkname);
        if linkname_copy.is_null() {
            return 1 as libc::c_int;
        }
        /*
         * TODO: consider using the cleaned-up path as the link
         * target?
         */
        r = cleanup_pathname_fsobj(
            linkname_copy,
            &mut error_number,
            &mut error_string,
            (*a).flags,
        );
        if r != ARCHIVE_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                error_number,
                b"%s\x00" as *const u8 as *const libc::c_char,
                error_string.s,
            );
            free(linkname_copy as *mut libc::c_void);
            archive_string_free(&mut error_string);
            /*
             * EPERM is more appropriate than error_number for our
             * callers
             */
            return 1 as libc::c_int;
        }
        r = check_symlinks_fsobj(
            linkname_copy,
            &mut error_number,
            &mut error_string,
            (*a).flags,
        );
        if r != ARCHIVE_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                error_number,
                b"%s\x00" as *const u8 as *const libc::c_char,
                error_string.s,
            );
            free(linkname_copy as *mut libc::c_void);
            archive_string_free(&mut error_string);
            /*
             * EPERM is more appropriate than error_number for our
             * callers
             */
            return 1 as libc::c_int;
        }
        free(linkname_copy as *mut libc::c_void);
        archive_string_free(&mut error_string);
        /*
         * Unlinking and linking here is really not atomic,
         * but doing it right, would require us to construct
         * an mktemplink() function, and then use rename(2).
         */
        if (*a).flags & ARCHIVE_EXTRACT_SAFE_WRITES != 0 {
            unlink((*a).name);
        }
        r = if link(linkname, (*a).name) != 0 {
            errno
        } else {
            0 as libc::c_int
        };
        /*
         * New cpio and pax formats allow hardlink entries
         * to carry data, so we may have to open the file
         * for hardlink entries.
         *
         * If the hardlink was successfully created and
         * the archive doesn't have carry data for it,
         * consider it to be non-authoritative for meta data.
         * This is consistent with GNU tar and BSD pax.
         * If the hardlink does carry data, let the last
         * archive entry decide ownership.
         */
        if r == 0 as libc::c_int && (*a).filesize <= 0 as libc::c_int as libc::c_long {
            (*a).todo = 0 as libc::c_int;
            (*a).deferred = 0 as libc::c_int
        } else if r == 0 as libc::c_int && (*a).filesize > 0 as libc::c_int as libc::c_long {
            r = lstat((*a).name, &mut st);
            if r != 0 as libc::c_int {
                r = errno
            } else if st.st_mode & AE_IFMT as mode_t == AE_IFREG as mode_t {
                (*a).fd = open(
                    (*a).name,
                    O_WRONLY | O_TRUNC | O_BINARY | O_CLOEXEC | O_NOFOLLOW,
                );
                __archive_ensure_cloexec_flag((*a).fd);
                if (*a).fd < 0 as libc::c_int {
                    r = errno
                }
            }
        }
        return r;
    }
    linkname = archive_entry_symlink((*a).entry);
    if !linkname.is_null() {
        /*
         * Unlinking and linking here is really not atomic,
         * but doing it right, would require us to construct
         * an mktempsymlink() function, and then use rename(2).
         */
        if (*a).flags & ARCHIVE_EXTRACT_SAFE_WRITES != 0 {
            unlink((*a).name);
        }
        return if symlink(linkname, (*a).name) != 0 {
            errno
        } else {
            0 as libc::c_int
        };
    }
    /*
     * The remaining system calls all set permissions, so let's
     * try to take advantage of that to avoid an extra chmod()
     * call.  (Recall that umask is set to zero right now!)
     */
    /* Mode we want for the final restored object (w/o file type bits). */
    final_mode = (*a).mode & 0o7777 as libc::c_int as libc::c_uint;
    /*
     * The mode that will actually be restored in this step.  Note
     * that SUID, SGID, etc, require additional work to ensure
     * security, so we never restore them at this point.
     */
    mode = final_mode & 0o777 as libc::c_int as libc::c_uint & !(*a).user_umask;
    /*
     * Always create writable such that [f]setxattr() works if we're not
     * root.
     */
    if (*a).user_uid != 0 as libc::c_int as libc::c_long
        && (*a).todo & (TODO_HFS_COMPRESSION | TODO_XATTR) != 0
    {
        mode |= 0o200 as libc::c_int as libc::c_uint
    }
    let mut current_block_73: u64;
    match (*a).mode & AE_IFMT as mode_t {
        32768 => {
            current_block_73 = 16327464825768601698;
        }
        8192 => {
            /* Note: we use AE_IFCHR for the case label, and
             * S_IFCHR for the mknod() call.  This is correct.  */
            r = mknod(
                (*a).name,
                mode | S_IFCHR as libc::c_uint,
                archive_entry_rdev((*a).entry),
            );
            current_block_73 = 12961834331865314435;
        }
        24576 => {
            /* HAVE_MKNOD */
            r = mknod(
                (*a).name,
                mode | S_IFBLK as libc::c_uint,
                archive_entry_rdev((*a).entry),
            );
            current_block_73 = 12961834331865314435;
        }
        16384 => {
            /* HAVE_MKNOD */
            mode = (mode | MINIMUM_DIR_MODE as libc::c_uint) & MAXIMUM_DIR_MODE as libc::c_uint;
            r = mkdir((*a).name, mode);
            if r == 0 as libc::c_int {
                /* Defer setting dir times. */
                (*a).deferred |= (*a).todo & TODO_TIMES;
                (*a).todo &= !TODO_TIMES;
                /* Never use an immediate chmod(). */
                /* We can't avoid the chmod() entirely if EXTRACT_PERM
                 * because of SysV SGID inheritance. */
                if mode != final_mode || (*a).flags & ARCHIVE_EXTRACT_PERM != 0 {
                    (*a).deferred |= (*a).todo & TODO_MODE
                }
                (*a).todo &= !TODO_MODE
            }
            current_block_73 = 12961834331865314435;
        }
        4096 => {
            r = mkfifo((*a).name, mode);
            current_block_73 = 12961834331865314435;
        }
        _ => {
            /* POSIX requires that we fall through here. */
            /* FALLTHROUGH */
            current_block_73 = 16327464825768601698;
        }
    }
    match current_block_73 {
        16327464825768601698 => {
            (*a).tmpname = NULL as *mut libc::c_char;
            (*a).fd = open(
                (*a).name,
                O_WRONLY | O_CREAT | O_EXCL | O_BINARY | O_CLOEXEC,
                mode,
            );
            __archive_ensure_cloexec_flag((*a).fd);
            r = ((*a).fd < 0 as libc::c_int) as libc::c_int
            /* HAVE_MKFIFO */
        }
        _ => {}
    }
    /* All the system calls above set errno on failure. */
    if r != 0 {
        return *__errno_location();
    }
    /* If we managed to set the final mode, we've avoided a chmod(). */
    if mode == final_mode {
        (*a).todo &= !TODO_MODE
    }
    return 0 as libc::c_int;
}
/*
 * Cleanup function for archive_extract.  Mostly, this involves processing
 * the fixup list, which is used to address a number of problems:
 *   * Dir permissions might prevent us from restoring a file in that
 *     dir, so we restore the dir with minimum 0700 permissions first,
 *     then correct the mode at the end.
 *   * Similarly, the act of restoring a file touches the directory
 *     and changes the timestamp on the dir, so we have to touch-up dir
 *     timestamps at the end as well.
 *   * Some file flags can interfere with the restore by, for example,
 *     preventing the creation of hardlinks to those files.
 *   * Mac OS extended metadata includes ACLs, so must be deferred on dirs.
 *
 * Note that tar/cpio do not require that archives be in a particular
 * order; there is no way to know when the last file has been restored
 * within a directory, so there's no way to optimize the memory usage
 * here by fixing up the directory any earlier than the
 * end-of-archive.
 *
 * XXX TODO: Directory ACLs should be restored here, for the same
 * reason we set directory perms here. XXX
 */
unsafe extern "C" fn _archive_write_disk_close(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write_disk = _a as *mut archive_write_disk;
    let mut next: *mut fixup_entry = 0 as *mut fixup_entry;
    let mut p: *mut fixup_entry = 0 as *mut fixup_entry;
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xc001b0c5 as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_write_disk_close\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    ret = _archive_write_disk_finish_entry(&mut (*a).archive);
    /* Sort dir list so directories are fixed up in depth-first order. */
    p = sort_dir_list((*a).fixup_list); /* Mark stat cache as out-of-date. */
    while !p.is_null() {
        fd = -(1 as libc::c_int);
        (*a).pst = NULL as *mut stat;
        if (*p).fixup & (TODO_TIMES | TODO_MODE_BASE | TODO_ACLS | TODO_FFLAGS) != 0 {
            fd = open((*p).name, O_WRONLY | O_BINARY | O_NOFOLLOW | O_CLOEXEC)
        }
        if (*p).fixup & TODO_TIMES != 0 {
            set_times(
                a,
                fd,
                (*p).mode as libc::c_int,
                (*p).name,
                (*p).atime,
                (*p).atime_nanos as libc::c_long,
                (*p).birthtime,
                (*p).birthtime_nanos as libc::c_long,
                (*p).mtime,
                (*p).mtime_nanos as libc::c_long,
                (*p).ctime,
                (*p).ctime_nanos as libc::c_long,
            );
        }
        if (*p).fixup & TODO_MODE_BASE != 0 {
            if fd >= 0 as libc::c_int {
                fchmod(fd, (*p).mode);
            } else {
                chmod((*p).name, (*p).mode);
            }
        }
        if (*p).fixup & TODO_ACLS != 0 {
            archive_write_disk_set_acls(&mut (*a).archive, fd, (*p).name, &mut (*p).acl, (*p).mode);
        }
        if (*p).fixup & TODO_FFLAGS != 0 {
            set_fflags_platform(
                a,
                fd,
                (*p).name,
                (*p).mode,
                (*p).fflags_set,
                0 as libc::c_int as libc::c_ulong,
            );
        }
        if (*p).fixup & TODO_MAC_METADATA != 0 {
            set_mac_metadata(a, (*p).name, (*p).mac_metadata, (*p).mac_metadata_size);
        }
        next = (*p).next;
        archive_acl_clear(&mut (*p).acl);
        free((*p).mac_metadata);
        free((*p).name as *mut libc::c_void);
        if fd >= 0 as libc::c_int {
            close(fd);
        }
        free(p as *mut libc::c_void);
        p = next
    }
    (*a).fixup_list = NULL as *mut fixup_entry;
    return ret;
}
unsafe extern "C" fn _archive_write_disk_free(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write_disk = 0 as *mut archive_write_disk;
    let mut ret: libc::c_int = 0;
    if _a.is_null() {
        return 0 as libc::c_int;
    }
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xc001b0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_write_disk_free\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_write_disk;
    ret = _archive_write_disk_close(&mut (*a).archive);
    archive_write_disk_set_group_lookup(
        &mut (*a).archive,
        NULL as *mut libc::c_void,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const libc::c_char,
                    _: la_int64_t,
                ) -> la_int64_t,
            >,
        >(NULL as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(NULL as libc::intptr_t),
    );
    archive_write_disk_set_user_lookup(
        &mut (*a).archive,
        NULL as *mut libc::c_void,
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const libc::c_char,
                    _: la_int64_t,
                ) -> la_int64_t,
            >,
        >(NULL as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        >(NULL as libc::intptr_t),
    );
    archive_entry_free((*a).entry);
    archive_string_free(&mut (*a)._name_data);
    archive_string_free(&mut (*a)._tmpname_data);
    archive_string_free(&mut (*a).archive.error_string);
    archive_string_free(&mut (*a).path_safe);
    (*a).archive.magic = 0 as libc::c_int as libc::c_uint;
    __archive_clean(&mut (*a).archive);
    free((*a).decmpfs_header_p as *mut libc::c_void);
    free((*a).resource_fork as *mut libc::c_void);
    free((*a).compressed_buffer as *mut libc::c_void);
    free((*a).uncompressed_buffer as *mut libc::c_void);
    free(a as *mut libc::c_void);
    return ret;
}
/*
 * Simple O(n log n) merge sort to order the fixup list.  In
 * particular, we want to restore dir timestamps depth-first.
 */
unsafe extern "C" fn sort_dir_list(mut p: *mut fixup_entry) -> *mut fixup_entry {
    let mut a: *mut fixup_entry = 0 as *mut fixup_entry;
    let mut b: *mut fixup_entry = 0 as *mut fixup_entry;
    let mut t: *mut fixup_entry = 0 as *mut fixup_entry;
    if p.is_null() {
        return 0 as *mut fixup_entry;
    }
    /* A one-item list is already sorted. */
    if (*p).next.is_null() {
        return p;
    }
    /* Step 1: split the list. */
    t = p;
    a = (*(*p).next).next;
    while !a.is_null() {
        /* Step a twice, t once. */
        a = (*a).next;
        if !a.is_null() {
            a = (*a).next
        }
        t = (*t).next
    }
    /* Now, t is at the mid-point, so break the list here. */
    b = (*t).next;
    (*t).next = NULL as *mut fixup_entry;
    a = p;
    /* Step 2: Recursively sort the two sub-lists. */
    a = sort_dir_list(a);
    b = sort_dir_list(b);
    /* Step 3: Merge the returned lists. */
    /* Pick the first element for the merged list. */
    if strcmp((*a).name, (*b).name) > 0 as libc::c_int {
        p = a;
        t = p;
        a = (*a).next
    } else {
        p = b;
        t = p;
        b = (*b).next
    }
    /* Always put the later element on the list first. */
    while !a.is_null() && !b.is_null() {
        if strcmp((*a).name, (*b).name) > 0 as libc::c_int {
            (*t).next = a;
            a = (*a).next
        } else {
            (*t).next = b;
            b = (*b).next
        }
        t = (*t).next
    }
    /* Only one list is non-empty, so just splice it on. */
    if !a.is_null() {
        (*t).next = a
    }
    if !b.is_null() {
        (*t).next = b
    }
    return p;
}
/*
 * Returns a new, initialized fixup entry.
 *
 * TODO: Reduce the memory requirements for this list by using a tree
 * structure rather than a simple list of names.
 */
unsafe extern "C" fn new_fixup(
    mut a: *mut archive_write_disk,
    mut pathname: *const libc::c_char,
) -> *mut fixup_entry {
    let mut fe: *mut fixup_entry = 0 as *mut fixup_entry;
    fe = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<fixup_entry>() as libc::c_ulong,
    ) as *mut fixup_entry;
    if fe.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for a fixup\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut fixup_entry;
    }
    (*fe).next = (*a).fixup_list;
    (*a).fixup_list = fe;
    (*fe).fixup = 0 as libc::c_int;
    (*fe).name = strdup(pathname);
    return fe;
}
/*
 * Returns a fixup structure for the current entry.
 */
unsafe extern "C" fn current_fixup(
    mut a: *mut archive_write_disk,
    mut pathname: *const libc::c_char,
) -> *mut fixup_entry {
    if (*a).current_fixup.is_null() {
        (*a).current_fixup = new_fixup(a, pathname)
    }
    return (*a).current_fixup;
}
/* Error helper for new *_fsobj functions */
unsafe extern "C" fn fsobj_error(
    mut a_eno: *mut libc::c_int,
    mut a_estr: *mut archive_string,
    mut err: libc::c_int,
    mut errstr: *const libc::c_char,
    mut path: *const libc::c_char,
) {
    if !a_eno.is_null() {
        *a_eno = err
    }
    if !a_estr.is_null() {
        archive_string_sprintf(
            a_estr,
            b"%s%s\x00" as *const u8 as *const libc::c_char,
            errstr,
            path,
        );
    };
}
/*
 * TODO: Someday, integrate this with the deep dir support; they both
 * scan the path and both can be optimized by comparing against other
 * recent paths.
 */
/*
 * Checks the given path to see if any elements along it are symlinks.  Returns
 * ARCHIVE_OK if there are none, otherwise puts an error in errmsg.
 */
unsafe extern "C" fn check_symlinks_fsobj(
    mut path: *mut libc::c_char,
    mut a_eno: *mut libc::c_int,
    mut a_estr: *mut archive_string,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = ARCHIVE_OK;
    let mut tail: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut head: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut r: libc::c_int = 0;
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
    let mut chdir_fd: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    /* Nothing to do here if name is empty */
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
        return 0 as libc::c_int;
    }
    /*
     * Guard against symlink tricks.  Reject any archive entry whose
     * destination would be altered by a symlink.
     *
     * Walk the filename in chunks separated by '/'.  For each segment:
     *  - if it doesn't exist, continue
     *  - if it's symlink, abort or remove it
     *  - if it's a directory and it's not the last chunk, cd into it
     * As we go:
     *  head points to the current (relative) path
     *  tail points to the temporary \0 terminating the segment we're
     *      currently examining
     *  c holds what used to be in *tail
     *  last is 1 if this is the last tail
     */
    chdir_fd = la_opendirat(AT_FDCWD, b".\x00" as *const u8 as *const libc::c_char);
    __archive_ensure_cloexec_flag(chdir_fd);
    if chdir_fd < 0 as libc::c_int {
        fsobj_error(
            a_eno,
            a_estr,
            errno,
            b"Could not open \x00" as *const u8 as *const libc::c_char,
            path,
        );
        return -(30 as libc::c_int);
    }
    head = path;
    tail = path;
    last = 0 as libc::c_int;
    /* TODO: reintroduce a safe cache here? */
    /* Skip the root directory if the path is absolute. */
    if tail == path && *tail.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        tail = tail.offset(1)
    }
    /* Keep going until we've checked the entire name.
     * head, tail, path all alias the same string, which is
     * temporarily zeroed at tail, so be careful restoring the
     * stashed (c=tail[0]) for error messages.
     * Exiting the loop with break is okay; continue is not.
     */
    while last == 0 {
        /*
         * Skip the separator we just consumed, plus any adjacent ones
         */
        while *tail as libc::c_int == '/' as i32 {
            tail = tail.offset(1)
        }
        /* Advance to the next segment. */
        /* Skip the next path element. */
        while *tail as libc::c_int != '\u{0}' as i32 && *tail as libc::c_int != '/' as i32 {
            tail = tail.offset(1)
        }
        /* is this the last path component? */
        last = (*tail.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *tail.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                && *tail.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
            as libc::c_int;
        /* temporarily truncate the string here */
        c = *tail.offset(0 as libc::c_int as isize);
        *tail.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        /* Check that we haven't hit a symlink. */
        r = fstatat(chdir_fd, head, &mut st, AT_SYMLINK_NOFOLLOW);
        if r != 0 as libc::c_int {
            *tail.offset(0 as libc::c_int as isize) = c;
            /* We've hit a dir that doesn't exist; stop now. */
            if errno == ENOENT {
                break;
            }
            /*
             * Treat any other error as fatal - best to be
             * paranoid here.
             * Note: This effectively disables deep
             * directory support when security checks are
             * enabled. Otherwise, very long pathnames that
             * trigger an error here could evade the
             * sandbox.
             * TODO: We could do better, but it would
             * probably require merging the symlink checks
             * with the deep-directory editing.
             */
            fsobj_error(
                a_eno,
                a_estr,
                errno,
                b"Could not stat \x00" as *const u8 as *const libc::c_char,
                path,
            );
            res = ARCHIVE_FAILED;
            break;
        } else {
            if st.st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint {
                if last == 0 {
                    fd = la_opendirat(chdir_fd, head);
                    if fd < 0 as libc::c_int {
                        r = -(1 as libc::c_int)
                    } else {
                        r = 0 as libc::c_int;
                        close(chdir_fd);
                        chdir_fd = fd
                    }
                    if r != 0 as libc::c_int {
                        *tail.offset(0 as libc::c_int as isize) = c;
                        fsobj_error(
                            a_eno,
                            a_estr,
                            errno,
                            b"Could not chdir \x00" as *const u8 as *const libc::c_char,
                            path,
                        );
                        res = -(30 as libc::c_int);
                        break;
                    } else {
                        /* Our view is now from inside this dir: */
                        head = tail.offset(1 as libc::c_int as isize)
                    }
                }
            } else if st.st_mode & __S_IFMT as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
            {
                if last != 0 {
                    /*
                     * Last element is symlink; remove it
                     * so we can overwrite it with the
                     * item being extracted.
                     */
                    r = unlinkat(chdir_fd, head, 0 as libc::c_int);
                    if r != 0 as libc::c_int {
                        *tail.offset(0 as libc::c_int as isize) = c;
                        fsobj_error(
                            a_eno,
                            a_estr,
                            errno,
                            b"Could not remove symlink \x00" as *const u8 as *const libc::c_char,
                            path,
                        );
                        res = ARCHIVE_FAILED;
                        break;
                    } else {
                        /*
                         * Even if we did remove it, a warning
                         * is in order.  The warning is silly,
                         * though, if we're just replacing one
                         * symlink with another symlink.
                         */
                        *tail.offset(0 as libc::c_int as isize) = c;
                        /*
                         * FIXME:  not sure how important this is to
                         * restore
                         */
                        /*
                        if (!S_ISLNK(path)) {
                            fsobj_error(a_eno, a_estr, 0,
                                "Removing symlink ", path);
                        }
                        */
                        /* Symlink gone.  No more problem! */
                        res = ARCHIVE_OK;
                        break;
                    }
                } else if flags & ARCHIVE_EXTRACT_UNLINK != 0 {
                    /* User asked us to remove problems. */
                    r = unlinkat(chdir_fd, head, 0 as libc::c_int);
                    if r != 0 as libc::c_int {
                        *tail.offset(0 as libc::c_int as isize) = c;
                        fsobj_error(
                            a_eno,
                            a_estr,
                            0 as libc::c_int,
                            b"Cannot remove intervening symlink \x00" as *const u8
                                as *const libc::c_char,
                            path,
                        );
                        res = ARCHIVE_FAILED;
                        break;
                    } else {
                        *tail.offset(0 as libc::c_int as isize) = c
                    }
                } else if flags & ARCHIVE_EXTRACT_SECURE_SYMLINKS == 0 as libc::c_int {
                    /*
                     * We are not the last element and we want to
                     * follow symlinks if they are a directory.
                     *
                     * This is needed to extract hardlinks over
                     * symlinks.
                     */
                    r = fstatat(chdir_fd, head, &mut st, 0 as libc::c_int);
                    if r != 0 as libc::c_int {
                        *tail.offset(0 as libc::c_int as isize) = c;
                        if errno == ENOENT {
                            break;
                        }
                        fsobj_error(
                            a_eno,
                            a_estr,
                            errno,
                            b"Could not stat \x00" as *const u8 as *const libc::c_char,
                            path,
                        );
                        res = -(25 as libc::c_int);
                        break;
                    } else if st.st_mode & __S_IFMT as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint
                    {
                        fd = la_opendirat(chdir_fd, head);
                        if fd < 0 as libc::c_int {
                            r = -(1 as libc::c_int)
                        } else {
                            r = 0 as libc::c_int;
                            close(chdir_fd);
                            chdir_fd = fd
                        }
                        if r != 0 as libc::c_int {
                            *tail.offset(0 as libc::c_int as isize) = c;
                            fsobj_error(
                                a_eno,
                                a_estr,
                                errno,
                                b"Could not chdir \x00" as *const u8 as *const libc::c_char,
                                path,
                            );
                            res = -(30 as libc::c_int);
                            break;
                        } else {
                            /*
                             * Our view is now from inside
                             * this dir:
                             */
                            head = tail.offset(1 as libc::c_int as isize)
                        }
                    } else {
                        *tail.offset(0 as libc::c_int as isize) = c;
                        fsobj_error(
                            a_eno,
                            a_estr,
                            0 as libc::c_int,
                            b"Cannot extract through symlink \x00" as *const u8
                                as *const libc::c_char,
                            path,
                        );
                        res = ARCHIVE_FAILED;
                        break;
                    }
                } else {
                    *tail.offset(0 as libc::c_int as isize) = c;
                    fsobj_error(
                        a_eno,
                        a_estr,
                        0 as libc::c_int,
                        b"Cannot extract through symlink \x00" as *const u8 as *const libc::c_char,
                        path,
                    );
                    res = ARCHIVE_FAILED;
                    break;
                }
            }
            /* be sure to always maintain this */
            *tail.offset(0 as libc::c_int as isize) = c;
            if *tail.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
                tail = tail.offset(1)
            }
        }
    }
    /* Catches loop exits via break */
    *tail.offset(0 as libc::c_int as isize) = c;
    /* If we operate with openat(), fstatat() and unlinkat() there was
     * no chdir(), so just close the fd */
    if chdir_fd >= 0 as libc::c_int {
        close(chdir_fd);
    }
    /* TODO: reintroduce a safe cache here? */
    return res;
}
/*
 * Check a->name for symlinks, returning ARCHIVE_OK if its clean, otherwise
 * calls archive_set_error and returns ARCHIVE_{FATAL,FAILED}
 */
unsafe extern "C" fn check_symlinks(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut error_string: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    }; /* to be safe */
    let mut error_number: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    error_string.s = NULL as *mut libc::c_char;
    error_string.length = 0 as libc::c_int as size_t;
    error_string.buffer_length = 0 as libc::c_int as size_t;
    rc = check_symlinks_fsobj((*a).name, &mut error_number, &mut error_string, (*a).flags);
    if rc != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            error_number,
            b"%s\x00" as *const u8 as *const libc::c_char,
            error_string.s,
        );
    }
    archive_string_free(&mut error_string);
    (*a).pst = NULL as *mut stat;
    return rc;
}
/*
 * Canonicalize the pathname.  In particular, this strips duplicate
 * '/' characters, '.' elements, and trailing '/'.  It also raises an
 * error for an empty path, a trailing '..', (if _SECURE_NODOTDOT is
 * set) any '..' in the path or (if ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS
 * is set) if the path is absolute.
 */
unsafe extern "C" fn cleanup_pathname_fsobj(
    mut path: *mut libc::c_char,
    mut a_eno: *mut libc::c_int,
    mut a_estr: *mut archive_string,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut separator: libc::c_char = '\u{0}' as i32 as libc::c_char;
    src = path;
    dest = src;
    if *src as libc::c_int == '\u{0}' as i32 {
        fsobj_error(
            a_eno,
            a_estr,
            ARCHIVE_ERRNO_MISC,
            b"Invalid empty \x00" as *const u8 as *const libc::c_char,
            b"pathname\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* Skip leading '/'. */
    if *src as libc::c_int == '/' as i32 {
        if flags & ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS != 0 {
            fsobj_error(
                a_eno,
                a_estr,
                ARCHIVE_ERRNO_MISC,
                b"Path is \x00" as *const u8 as *const libc::c_char,
                b"absolute\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        let fresh0 = src;
        src = src.offset(1);
        separator = *fresh0
    }
    /* Scan the pathname one element at a time. */
    /* src points to first char after '/' */
    while !(*src.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32) {
        if *src.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            /* Found '//', ignore second one. */
            src = src.offset(1)
        } else {
            if *src.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                if *src.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
                    break;
                }
                if *src.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    /* Skip './'. */
                    src = src.offset(2 as libc::c_int as isize);
                    continue;
                } else if *src.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                    if *src.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                        || *src.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                    {
                        /* Conditionally warn about '..' */
                        if flags & ARCHIVE_EXTRACT_SECURE_NODOTDOT != 0 {
                            fsobj_error(
                                a_eno,
                                a_estr,
                                ARCHIVE_ERRNO_MISC,
                                b"Path contains \x00" as *const u8 as *const libc::c_char,
                                b"\'..\'\x00" as *const u8 as *const libc::c_char,
                            );
                            return -(25 as libc::c_int);
                        }
                    }
                    /*
                     * Note: Under no circumstances do we
                     * remove '..' elements.  In
                     * particular, restoring
                     * '/foo/../bar/' should create the
                     * 'foo' dir as a side-effect.
                     */
                }
            }
            /* Copy current element, including leading '/'. */
            if separator != 0 {
                let fresh1 = dest;
                dest = dest.offset(1);
                *fresh1 = '/' as i32 as libc::c_char
            }
            while *src as libc::c_int != '\u{0}' as i32 && *src as libc::c_int != '/' as i32 {
                let fresh2 = src;
                src = src.offset(1);
                let fresh3 = dest;
                dest = dest.offset(1);
                *fresh3 = *fresh2
            }
            if *src as libc::c_int == '\u{0}' as i32 {
                break;
            }
            /* Skip '/' separator. */
            let fresh4 = src;
            src = src.offset(1);
            separator = *fresh4
        }
    }
    /*
     * We've just copied zero or more path elements, not including the
     * final '/'.
     */
    if dest == path {
        /*
         * Nothing got copied.  The path must have been something
         * like '.' or '/' or './' or '/././././/./'.
         */
        if separator != 0 {
            let fresh5 = dest;
            dest = dest.offset(1);
            *fresh5 = '/' as i32 as libc::c_char
        } else {
            let fresh6 = dest;
            dest = dest.offset(1);
            *fresh6 = '.' as i32 as libc::c_char
        }
    }
    /* Terminate the result. */
    *dest = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn cleanup_pathname(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut error_string: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut error_number: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    error_string.s = NULL as *mut libc::c_char;
    error_string.length = 0 as libc::c_int as size_t;
    error_string.buffer_length = 0 as libc::c_int as size_t;
    rc = cleanup_pathname_fsobj((*a).name, &mut error_number, &mut error_string, (*a).flags);
    if rc != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            error_number,
            b"%s\x00" as *const u8 as *const libc::c_char,
            error_string.s,
        );
    }
    archive_string_free(&mut error_string);
    return rc;
}
/*
 * Create the parent directory of the specified path, assuming path
 * is already in mutable storage.
 */
unsafe extern "C" fn create_parent_dir(
    mut a: *mut archive_write_disk,
    mut path: *mut libc::c_char,
) -> libc::c_int {
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    /* Remove tail element to obtain parent name. */
    slash = strrchr(path, '/' as i32);
    if slash.is_null() {
        return 0 as libc::c_int;
    }
    *slash = '\u{0}' as i32 as libc::c_char;
    r = create_dir(a, path);
    *slash = '/' as i32 as libc::c_char;
    return r;
}
/*
 * Create the specified dir, recursing to create parents as necessary.
 *
 * Returns ARCHIVE_OK if the path exists when we're done here.
 * Otherwise, returns ARCHIVE_FAILED.
 * Assumes path is in mutable storage; path is unchanged on exit.
 */
unsafe extern "C" fn create_dir(
    mut a: *mut archive_write_disk,
    mut path: *mut libc::c_char,
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
    let mut le: *mut fixup_entry = 0 as *mut fixup_entry;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mode_final: mode_t = 0;
    let mut mode: mode_t = 0;
    let mut r: libc::c_int = 0;
    /* Check for special names and just skip them. */
    slash = strrchr(path, '/' as i32);
    if slash.is_null() {
        base = path
    } else {
        base = slash.offset(1 as libc::c_int as isize)
    }
    if *base.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        || *base.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *base.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        || *base.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *base.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *base.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        /* Don't bother trying to create null path, '.', or '..'. */
        if !slash.is_null() {
            *slash = '\u{0}' as i32 as libc::c_char;
            r = create_dir(a, path);
            *slash = '/' as i32 as libc::c_char;
            return r;
        }
        return 0 as libc::c_int;
    }
    /*
     * Yes, this should be stat() and not lstat().  Using lstat()
     * here loses the ability to extract through symlinks.  Also note
     * that this should not use the a->st cache.
     */
    if stat(path, &mut st) == 0 as libc::c_int {
        if st.st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        if (*a).flags & ARCHIVE_EXTRACT_NO_OVERWRITE != 0 {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EEXIST,
                b"Can\'t create directory \'%s\'\x00" as *const u8 as *const libc::c_char,
                path,
            );
            return -(25 as libc::c_int);
        }
        if unlink(path) != 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t create directory \'%s\': Conflicting file cannot be removed\x00"
                    as *const u8 as *const libc::c_char,
                path,
            );
            return -(25 as libc::c_int);
        }
    } else if errno != ENOENT && errno != ENOTDIR {
        /* Stat failed? */
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Can\'t test directory \'%s\'\x00" as *const u8 as *const libc::c_char,
            path,
        );
        return -(25 as libc::c_int);
    } else {
        if !slash.is_null() {
            *slash = '\u{0}' as i32 as libc::c_char;
            r = create_dir(a, path);
            *slash = '/' as i32 as libc::c_char;
            if r != ARCHIVE_OK {
                return r;
            }
        }
    }
    /*
     * Mode we want for the final restored directory.  Per POSIX,
     * implicitly-created dirs must be created obeying the umask.
     * There's no mention whether this is different for privileged
     * restores (which the rest of this code handles by pretending
     * umask=0).  I've chosen here to always obey the user's umask for
     * implicit dirs, even if _EXTRACT_PERM was specified.
     */
    mode_final = DEFAULT_DIR_MODE as libc::c_uint & !(*a).user_umask;
    /* Mode we want on disk during the restore process. */
    mode = mode_final;
    mode |= MINIMUM_DIR_MODE as libc::c_uint;
    mode &= MAXIMUM_DIR_MODE as libc::c_uint;
    if mkdir(path, mode) == 0 as libc::c_int {
        if mode != mode_final {
            le = new_fixup(a, path);
            if le.is_null() {
                return -(30 as libc::c_int);
            }
            (*le).fixup |= TODO_MODE_BASE;
            (*le).mode = mode_final
        }
        return 0 as libc::c_int;
    }
    /*
     * Without the following check, a/b/../b/c/d fails at the
     * second visit to 'b', so 'd' can't be created.  Note that we
     * don't add it to the fixup list here, as it's already been
     * added.
     */
    if stat(path, &mut st) == 0 as libc::c_int
        && st.st_mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        errno,
        b"Failed to create dir \'%s\'\x00" as *const u8 as *const libc::c_char,
        path,
    );
    return -(25 as libc::c_int);
}
/*
 * Note: Although we can skip setting the user id if the desired user
 * id matches the current user, we cannot skip setting the group, as
 * many systems set the gid based on the containing directory.  So
 * we have to perform a chown syscall if we want to set the SGID
 * bit.  (The alternative is to stat() and then possibly chown(); it's
 * more efficient to skip the stat() and just always chown().)  Note
 * that a successful chown() here clears the TODO_SGID_CHECK bit, which
 * allows set_mode to skip the stat() check for the GID.
 */
unsafe extern "C" fn set_ownership(mut a: *mut archive_write_disk) -> libc::c_int {
    /* If we have an fd, we can avoid a race. */
    if (*a).fd >= 0 as libc::c_int
        && fchown((*a).fd, (*a).uid as __uid_t, (*a).gid as __gid_t) == 0 as libc::c_int
    {
        /* We've set owner and know uid/gid are correct. */
        (*a).todo &= !(TODO_OWNER | TODO_SGID_CHECK | TODO_SUID_CHECK);
        return 0 as libc::c_int;
    }
    /* We prefer lchown() but will use chown() if that's all we have. */
    /* Of course, if we have neither, this will always fail. */
    if lchown((*a).name, (*a).uid as __uid_t, (*a).gid as __gid_t) == 0 as libc::c_int {
        /* We've set owner and know uid/gid are correct. */
        (*a).todo &= !(TODO_OWNER | TODO_SGID_CHECK | TODO_SUID_CHECK);
        return 0 as libc::c_int;
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        errno,
        b"Can\'t set user=%jd/group=%jd for %s\x00" as *const u8 as *const libc::c_char,
        (*a).uid,
        (*a).gid,
        (*a).name,
    );
    return -(20 as libc::c_int);
}
/*
 * Note: Returns 0 on success, non-zero on failure.
 */
unsafe extern "C" fn set_time(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
    mut name: *const libc::c_char,
    mut atime: time_t,
    mut atime_nsec: libc::c_long,
    mut mtime: time_t,
    mut mtime_nsec: libc::c_long,
) -> libc::c_int {
    /* Select the best implementation for this platform. */
    /*
     * utimensat() and futimens() are defined in
     * POSIX.1-2008. They support ns resolution and setting times
     * on fds and symlinks.
     */
    let mut ts: [timespec; 2] = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 2];
    /* UNUSED */
    ts[0 as libc::c_int as usize].tv_sec = atime;
    ts[0 as libc::c_int as usize].tv_nsec = atime_nsec;
    ts[1 as libc::c_int as usize].tv_sec = mtime;
    ts[1 as libc::c_int as usize].tv_nsec = mtime_nsec;
    if fd >= 0 as libc::c_int {
        return futimens(fd, ts.as_mut_ptr() as *const timespec);
    }
    return utimensat(
        AT_FDCWD,
        name,
        ts.as_mut_ptr() as *const timespec,
        AT_SYMLINK_NOFOLLOW,
    );
}
/* F_SETTIMES */
unsafe extern "C" fn set_times(
    mut a: *mut archive_write_disk,
    mut fd: libc::c_int,
    mut mode: libc::c_int,
    mut name: *const libc::c_char,
    mut atime: time_t,
    mut atime_nanos: libc::c_long,
    mut birthtime: time_t,
    mut birthtime_nanos: libc::c_long,
    mut mtime: time_t,
    mut mtime_nanos: libc::c_long,
    mut cctime: time_t,
    mut ctime_nanos: libc::c_long,
) -> libc::c_int {
    /* Note: set_time doesn't use libarchive return conventions!
     * It uses syscall conventions.  So 0 here instead of ARCHIVE_OK. */
    let mut r1: libc::c_int = 0 as libc::c_int;
    let mut r2: libc::c_int = 0 as libc::c_int;
    /* Tru64 */
    /* UNUSED */
    r2 = set_time(fd, mode, name, atime, atime_nanos, mtime, mtime_nanos);
    if r1 != 0 as libc::c_int || r2 != 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Can\'t restore time\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_times_from_entry(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut atime: time_t = 0;
    let mut birthtime: time_t = 0;
    let mut mtime: time_t = 0;
    let mut cctime: time_t = 0;
    let mut atime_nsec: libc::c_long = 0;
    let mut birthtime_nsec: libc::c_long = 0;
    let mut mtime_nsec: libc::c_long = 0;
    let mut ctime_nsec: libc::c_long = 0;
    /* Suitable defaults. */
    cctime = (*a).start_time;
    mtime = cctime;
    birthtime = mtime;
    atime = birthtime;
    ctime_nsec = 0 as libc::c_int as libc::c_long;
    mtime_nsec = ctime_nsec;
    birthtime_nsec = mtime_nsec;
    atime_nsec = birthtime_nsec;
    /* If no time was provided, we're done. */
    if archive_entry_atime_is_set((*a).entry) == 0 && archive_entry_mtime_is_set((*a).entry) == 0 {
        return 0 as libc::c_int;
    } /* Strip off file type bits. */
    if archive_entry_atime_is_set((*a).entry) != 0 {
        atime = archive_entry_atime((*a).entry);
        atime_nsec = archive_entry_atime_nsec((*a).entry)
    }
    if archive_entry_birthtime_is_set((*a).entry) != 0 {
        birthtime = archive_entry_birthtime((*a).entry);
        birthtime_nsec = archive_entry_birthtime_nsec((*a).entry)
    }
    if archive_entry_mtime_is_set((*a).entry) != 0 {
        mtime = archive_entry_mtime((*a).entry);
        mtime_nsec = archive_entry_mtime_nsec((*a).entry)
    }
    if archive_entry_ctime_is_set((*a).entry) != 0 {
        cctime = archive_entry_ctime((*a).entry);
        ctime_nsec = archive_entry_ctime_nsec((*a).entry)
    }
    return set_times(
        a,
        (*a).fd,
        (*a).mode as libc::c_int,
        (*a).name,
        atime,
        atime_nsec,
        birthtime,
        birthtime_nsec,
        mtime,
        mtime_nsec,
        cctime,
        ctime_nsec,
    );
}
unsafe extern "C" fn set_mode(
    mut a: *mut archive_write_disk,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r2: libc::c_int = 0;
    mode &= 0o7777 as libc::c_int;
    if (*a).todo & TODO_SGID_CHECK != 0 {
        /*
         * If we don't know the GID is right, we must stat()
         * to verify it.  We can't just check the GID of this
         * process, since systems sometimes set GID from
         * the enclosing dir or based on ACLs.
         */
        r = lazy_stat(a);
        if r != ARCHIVE_OK {
            return r;
        }
        if (*(*a).pst).st_gid as libc::c_long != (*a).gid {
            mode &= !S_ISGID;
            if (*a).flags & ARCHIVE_EXTRACT_OWNER != 0 {
                /*
                 * This is only an error if you
                 * requested owner restore.  If you
                 * didn't, we'll try to restore
                 * sgid/suid, but won't consider it a
                 * problem if we can't.
                 */
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    -(1 as libc::c_int),
                    b"Can\'t restore SGID bit\x00" as *const u8 as *const libc::c_char,
                );
                r = ARCHIVE_WARN
            }
        }
        /* While we're here, double-check the UID. */
        if (*(*a).pst).st_uid as libc::c_long != (*a).uid && (*a).todo & TODO_SUID != 0 {
            mode &= !S_ISUID;
            if (*a).flags & ARCHIVE_EXTRACT_OWNER != 0 {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    -(1 as libc::c_int),
                    b"Can\'t restore SUID bit\x00" as *const u8 as *const libc::c_char,
                );
                r = ARCHIVE_WARN
            }
        }
        (*a).todo &= !TODO_SGID_CHECK;
        (*a).todo &= !TODO_SUID_CHECK
    } else if (*a).todo & TODO_SUID_CHECK != 0 {
        /*
         * If we don't know the UID is right, we can just check
         * the user, since all systems set the file UID from
         * the process UID.
         */
        if (*a).user_uid != (*a).uid {
            mode &= !S_ISUID;
            if (*a).flags & ARCHIVE_EXTRACT_OWNER != 0 {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    -(1 as libc::c_int),
                    b"Can\'t make file SUID\x00" as *const u8 as *const libc::c_char,
                );
                r = ARCHIVE_WARN
            }
        }
        (*a).todo &= !TODO_SUID_CHECK
    }
    if !((*a).mode & __S_IFMT as libc::c_uint == 0o120000 as libc::c_int as libc::c_uint) {
        if !((*a).mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint) {
            /*
             * If it's not a symlink and not a dir, then use
             * fchmod() or chmod(), depending on whether we have
             * an fd.  Dirs get their perms set during the
             * post-extract fixup, which is handled elsewhere.
             */
            if (*a).fd >= 0 as libc::c_int {
                r2 = fchmod((*a).fd, mode as __mode_t)
            } else {
                /* If this platform lacks fchmod(), then
                 * we'll just use chmod(). */
                r2 = chmod((*a).name, mode as __mode_t)
            }
            if r2 != 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Can\'t set permissions to 0%o\x00" as *const u8 as *const libc::c_char,
                    mode,
                );
                r = ARCHIVE_WARN
            }
        }
    }
    return r;
}
unsafe extern "C" fn set_fflags(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut le: *mut fixup_entry = 0 as *mut fixup_entry;
    let mut set: libc::c_ulong = 0;
    let mut clear: libc::c_ulong = 0;
    let mut r: libc::c_int = 0;
    let mut mode: mode_t = archive_entry_mode((*a).entry);
    /*
     * Make 'critical_flags' hold all file flags that can't be
     * immediately restored.  For example, on BSD systems,
     * SF_IMMUTABLE prevents hardlinks from being created, so
     * should not be set until after any hardlinks are created.  To
     * preserve some semblance of portability, this uses #ifdef
     * extensively.  Ugly, but it works.
     *
     * Yes, Virginia, this does create a security race.  It's mitigated
     * somewhat by the practice of creating dirs 0700 until the extract
     * is done, but it would be nice if we could do more than that.
     * People restoring critical file systems should be wary of
     * other programs that might try to muck with files as they're
     * being restored.
     */
    let critical_flags: libc::c_int =
        0 as libc::c_int | FS_APPEND_FL | FS_IMMUTABLE_FL | FS_JOURNAL_DATA_FL;
    if (*a).todo & TODO_FFLAGS != 0 {
        archive_entry_fflags((*a).entry, &mut set, &mut clear);
        /*
         * The first test encourages the compiler to eliminate
         * all of this if it's not necessary.
         */
        if critical_flags != 0 as libc::c_int && set & critical_flags as libc::c_ulong != 0 {
            le = current_fixup(a, (*a).name);
            if le.is_null() {
                return -(30 as libc::c_int);
            }
            (*le).fixup |= TODO_FFLAGS;
            (*le).fflags_set = set;
            /* Store the mode if it's not already there. */
            if (*le).fixup & TODO_MODE == 0 as libc::c_int {
                (*le).mode = mode
            }
        } else {
            r = set_fflags_platform(a, (*a).fd, (*a).name, mode, set, clear);
            if r != ARCHIVE_OK {
                return r;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn clear_nochange_fflags(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut mode: mode_t = archive_entry_mode((*a).entry);
    let nochange_flags: libc::c_int = 0 as libc::c_int;
    return set_fflags_platform(
        a,
        (*a).fd,
        (*a).name,
        mode,
        0 as libc::c_int as libc::c_ulong,
        nochange_flags as libc::c_ulong,
    );
}
/*
 * Linux uses ioctl() to read and write file flags.
 */
unsafe extern "C" fn set_fflags_platform(
    mut a: *mut archive_write_disk,
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut mode: mode_t,
    mut set: libc::c_ulong,
    mut clear: libc::c_ulong,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut myfd: libc::c_int = fd;
    let mut newflags: libc::c_int = 0;
    let mut oldflags: libc::c_int = 0;
    /*
     * Linux has no define for the flags that are only settable by
     * the root user.  This code may seem a little complex, but
     * there seem to be some Linux systems that lack these
     * defines. (?)  The code below degrades reasonably gracefully
     * if sf_mask is incomplete.
     */
    let sf_mask: libc::c_int =
        0 as libc::c_int | FS_IMMUTABLE_FL | FS_APPEND_FL | FS_JOURNAL_DATA_FL;
    if set == 0 as libc::c_int as libc::c_ulong && clear == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    /* Only regular files and dirs can have flags. */
    if !(mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint)
        && !(mode & __S_IFMT as libc::c_uint == 0o40000 as libc::c_int as libc::c_uint)
    {
        return 0 as libc::c_int;
    }
    /* If we weren't given an fd, open it ourselves. */
    if myfd < 0 as libc::c_int {
        myfd = open(name, O_RDONLY | O_NONBLOCK | O_BINARY | O_CLOEXEC);
        __archive_ensure_cloexec_flag(myfd);
    }
    if myfd < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /*
     * XXX As above, this would be way simpler if we didn't have
     * to read the current flags from disk. XXX
     */
    ret = ARCHIVE_OK;
    /* Read the current file flags. */
    if ioctl(myfd, FS_IOC_GETFLAGS, &mut oldflags as *mut libc::c_int) < 0 as libc::c_int {
        current_block = 2053931041760441864;
    } else {
        /* Try setting the flags as given. */
        newflags = (oldflags as libc::c_ulong & !clear | set) as libc::c_int;
        if ioctl(myfd, FS_IOC_SETFLAGS, &mut newflags as *mut libc::c_int) >= 0 as libc::c_int {
            current_block = 12994587391975696863;
        } else if errno != EPERM {
            current_block = 2053931041760441864;
        } else {
            /* If we couldn't set all the flags, try again with a subset. */
            newflags &= !sf_mask;
            oldflags &= sf_mask;
            newflags |= oldflags;
            if ioctl(myfd, FS_IOC_SETFLAGS, &mut newflags as *mut libc::c_int) >= 0 as libc::c_int {
                current_block = 12994587391975696863;
            } else {
                current_block = 2053931041760441864;
            }
        }
    }
    match current_block {
        2053931041760441864 =>
        /* We couldn't set the flags, so report the failure. */
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Failed to set file flags\x00" as *const u8 as *const libc::c_char,
            );
            ret = ARCHIVE_WARN
        }
        _ => {}
    }
    if fd < 0 as libc::c_int {
        close(myfd);
    }
    return ret;
}
/* __linux */
/* Default is to simply drop Mac extended metadata. */
unsafe extern "C" fn set_mac_metadata(
    mut a: *mut archive_write_disk,
    mut pathname: *const libc::c_char,
    mut metadata: *const libc::c_void,
    mut metadata_size: size_t,
) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
unsafe extern "C" fn fixup_appledouble(
    mut a: *mut archive_write_disk,
    mut pathname: *const libc::c_char,
) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
/*
 * Restore extended attributes -  Linux, Darwin and AIX implementations:
 * AIX' ea interface is syntaxwise identical to the Linux xattr interface.
 */
unsafe extern "C" fn set_xattrs(mut a: *mut archive_write_disk) -> libc::c_int {
    let mut entry: *mut archive_entry = (*a).entry;
    let mut errlist: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut ret: libc::c_int = ARCHIVE_OK;
    let mut i: libc::c_int = archive_entry_xattr_reset(entry);
    let mut fail: libc::c_short = 0 as libc::c_int as libc::c_short;
    errlist.s = NULL as *mut libc::c_char;
    errlist.length = 0 as libc::c_int as size_t;
    errlist.buffer_length = 0 as libc::c_int as size_t;
    loop {
        let fresh7 = i;
        i = i - 1;
        if !(fresh7 != 0) {
            break;
        }
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *const libc::c_void = 0 as *const libc::c_void;
        let mut size: size_t = 0;
        let mut e: libc::c_int = 0;
        archive_entry_xattr_next(entry, &mut name, &mut value, &mut size);
        if name.is_null() {
            continue;
        }
        /* Linux: quietly skip POSIX.1e ACL extended attributes */
        if strncmp(
            name,
            b"system.\x00" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && (strcmp(
                name.offset(7 as libc::c_int as isize),
                b"posix_acl_access\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    name.offset(7 as libc::c_int as isize),
                    b"posix_acl_default\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
        {
            continue;
        }
        if strncmp(
            name,
            b"trusted.SGI_\x00" as *const u8 as *const libc::c_char,
            12 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && (strcmp(
                name.offset(12 as libc::c_int as isize),
                b"ACL_DEFAULT\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(
                    name.offset(12 as libc::c_int as isize),
                    b"ACL_FILE\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int)
        {
            continue;
        }
        /* Linux: xfsroot namespace is obsolete and unsupported */
        if strncmp(
            name,
            b"xfsroot.\x00" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            fail = 1 as libc::c_int as libc::c_short;
            archive_strcat(&mut errlist, name as *const libc::c_void);
            archive_strappend_char(&mut errlist, ' ' as i32 as libc::c_char);
        } else {
            if (*a).fd >= 0 as libc::c_int {
                e = fsetxattr((*a).fd, name, value, size, 0 as libc::c_int)
            } else {
                e = lsetxattr(
                    archive_entry_pathname(entry),
                    name,
                    value,
                    size,
                    0 as libc::c_int,
                )
            }
            if e == -(1 as libc::c_int) {
                ret = ARCHIVE_WARN;
                archive_strcat(&mut errlist, name as *const libc::c_void);
                archive_strappend_char(&mut errlist, ' ' as i32 as libc::c_char);
                if errno != ENOTSUP && errno != ENOSYS {
                    fail = 1 as libc::c_int as libc::c_short
                }
            }
        }
    }
    if ret == ARCHIVE_WARN {
        if fail as libc::c_int != 0 && errlist.length > 0 as libc::c_int as libc::c_ulong {
            errlist.length = errlist.length.wrapping_sub(1);
            *errlist.s.offset(errlist.length as isize) = '\u{0}' as i32 as libc::c_char;
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Cannot restore extended attributes: %s\x00" as *const u8 as *const libc::c_char,
                errlist.s,
            );
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Cannot restore extended attributes on this file system.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    archive_string_free(&mut errlist);
    return ret;
}
/*
 * Test if file on disk is older than entry.
 */
unsafe extern "C" fn older(mut st: *mut stat, mut entry: *mut archive_entry) -> libc::c_int {
    /* First, test the seconds and return if we have a definite answer. */
    /* Definitely older. */
    if (if (*st).st_mtim.tv_sec < 0 as libc::c_int as libc::c_long {
        (*st).st_mtim.tv_sec
    } else {
        (if (*st).st_mtim.tv_sec as uint64_t > INT64_MAX as uint64_t {
            INT64_MAX
        } else {
            (*st).st_mtim.tv_sec
        })
    }) < (if archive_entry_mtime(entry) < 0 as libc::c_int as libc::c_long {
        archive_entry_mtime(entry)
    } else {
        (if archive_entry_mtime(entry) as uint64_t > INT64_MAX as uint64_t {
            INT64_MAX
        } else {
            archive_entry_mtime(entry)
        })
    }) {
        return 1 as libc::c_int;
    }
    /* Definitely younger. */
    if (if (*st).st_mtim.tv_sec < 0 as libc::c_int as libc::c_long {
        (*st).st_mtim.tv_sec
    } else {
        (if (*st).st_mtim.tv_sec as uint64_t > INT64_MAX as uint64_t {
            INT64_MAX
        } else {
            (*st).st_mtim.tv_sec
        })
    }) > (if archive_entry_mtime(entry) < 0 as libc::c_int as libc::c_long {
        archive_entry_mtime(entry)
    } else {
        (if archive_entry_mtime(entry) as uint64_t > INT64_MAX as uint64_t {
            INT64_MAX
        } else {
            archive_entry_mtime(entry)
        })
    }) {
        return 0 as libc::c_int;
    }
    /* If this platform supports fractional seconds, try those. */
    /* Definitely older. */
    if (*st).st_mtim.tv_nsec < archive_entry_mtime_nsec(entry) {
        return 1 as libc::c_int;
    }
    /* Same age or newer, so not older. */
    return 0 as libc::c_int;
}
/*-
 * Copyright (c) 2003-2007 Tim Kientzle
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
 * $FreeBSD: head/lib/libarchive/archive_write_disk_private.h 201086 2009-12-28 02:17:53Z kientzle $
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_set_acls(
    mut a: *mut archive,
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
    mut abstract_acl: *mut archive_acl,
    mut mode: mode_t,
) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
/* !_WIN32 || __CYGWIN__ */
