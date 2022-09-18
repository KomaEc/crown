use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type internal_state;
    pub type archive_string_conv;
    /* Declare our basic types. */
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
    fn uname(__name: *mut utsname) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn ctime_r(__timer: *const time_t, __buf: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn tzset();
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn getuid() -> __uid_t;
    #[no_mangle]
    fn getgid() -> __gid_t;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn deflateReset(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateReset(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn deflateInit_(
        strm: z_streamp,
        level: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn inflateInit_(
        strm: z_streamp,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    /*
     * Textual name/version of the library, useful for version displays.
     */
    #[no_mangle]
    fn archive_version_string() -> *const libc::c_char;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    /* The 'clone' function does a deep copy; all of the strings are copied too. */
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
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
    fn archive_entry_atime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_birthtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_birthtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_is_set(_: *mut archive_entry) -> libc::c_int;
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
    fn archive_entry_mtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_rdev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    /*
     * Set fields in an archive_entry.
     *
     * Note: Before libarchive 2.4, there were 'set' and 'copy' versions
     * of the string setters.  'copy' copied the actual string, 'set' just
     * stored the pointer.  In libarchive 2.4 and later, strings are
     * always copied.
     */
    #[no_mangle]
    fn archive_entry_set_atime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_unset_atime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_unset_ctime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_unset_mtime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_unset_size(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn _archive_entry_pathname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn __archive_mktemp(tmpdir: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_string_conversion_to_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_string_concat(dest: *mut archive_string, src: *mut archive_string);
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_strncpy_l(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn __archive_rb_tree_init(_: *mut archive_rb_tree, _: *const archive_rb_tree_ops);
    #[no_mangle]
    fn __archive_rb_tree_insert_node(
        _: *mut archive_rb_tree,
        _: *mut archive_rb_node,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_rb_tree_find_node(
        _: *mut archive_rb_tree,
        _: *const libc::c_void,
    ) -> *mut archive_rb_node;
    #[no_mangle]
    fn __archive_rb_tree_remove_node(_: *mut archive_rb_tree, _: *mut archive_rb_node);
    #[no_mangle]
    fn __archive_rb_tree_iterate(
        _: *mut archive_rb_tree,
        _: *mut archive_rb_node,
        _: libc::c_uint,
    ) -> *mut archive_rb_node;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
pub type va_list = __builtin_va_list;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
pub type z_streamp = *mut z_stream;
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
/* Returns size actually written, zero on EOF, -1 on error. */
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
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
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
pub struct iso9660 {
    pub birth_time: time_t,
    pub temp_fd: libc::c_int,
    pub cur_file: *mut isofile,
    pub cur_dirent: *mut isoent,
    pub cur_dirstr: archive_string,
    pub bytes_remaining: uint64_t,
    pub need_multi_extent: libc::c_int,
    pub utf16be: archive_string,
    pub mbs: archive_string,
    pub sconv_to_utf16be: *mut archive_string_conv,
    pub sconv_from_utf16be: *mut archive_string_conv,
    pub all_file_list: C2RustUnnamed_9,
    pub data_file_list: C2RustUnnamed_8,
    pub hardlink_rbtree: archive_rb_tree,
    pub primary: vdd,
    pub joliet: vdd,
    pub volume_space_size: libc::c_int,
    pub volume_sequence_number: libc::c_int,
    pub total_file_block: libc::c_int,
    pub volume_identifier: archive_string,
    pub publisher_identifier: archive_string,
    pub data_preparer_identifier: archive_string,
    pub application_identifier: archive_string,
    pub copyright_file_identifier: archive_string,
    pub abstract_file_identifier: archive_string,
    pub bibliographic_file_identifier: archive_string,
    pub location_rrip_er: libc::c_int,
    pub zisofs: C2RustUnnamed_7,
    pub directories_too_deep: *mut isoent,
    pub dircnt_max: libc::c_int,
    pub wbuff: [libc::c_uchar; 65536],
    pub wbuff_remaining: size_t,
    pub wbuff_type: C2RustUnnamed_6,
    pub wbuff_offset: int64_t,
    pub wbuff_written: int64_t,
    pub wbuff_tail: int64_t,
    pub el_torito: C2RustUnnamed,
    pub opt: iso_option,
}
/*
 * ISO writer options
 */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct iso_option {
    #[bitfield(name = "abstract_file", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "application_id", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "allow_vernum", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "biblio_file", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "boot", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "boot_catalog", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "boot_info_table", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "boot_load_seg", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "boot_load_size", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "boot_type", ty = "libc::c_uint", bits = "9..=10")]
    #[bitfield(name = "compression_level", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "copyright_file", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "gid", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "iso_level", ty = "libc::c_uint", bits = "14..=16")]
    #[bitfield(name = "joliet", ty = "libc::c_uint", bits = "17..=18")]
    #[bitfield(name = "limit_depth", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "limit_dirs", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pad", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "publisher", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "rr", ty = "libc::c_uint", bits = "23..=24")]
    #[bitfield(name = "volume_id", ty = "libc::c_uint", bits = "25..=25")]
    #[bitfield(name = "zisofs", ty = "libc::c_uint", bits = "26..=26")]
    pub abstract_file_application_id_allow_vernum_biblio_file_boot_boot_catalog_boot_info_table_boot_load_seg_boot_load_size_boot_type_compression_level_copyright_file_gid_iso_level_joliet_limit_depth_limit_dirs_pad_publisher_rr_volume_id_zisofs:
        [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub catalog_filename: archive_string,
    pub catalog: *mut isoent,
    pub boot_filename: archive_string,
    pub boot: *mut isoent,
    pub platform_id: libc::c_uchar,
    pub id: archive_string,
    pub media_type: libc::c_uchar,
    pub system_type: libc::c_uchar,
    pub boot_load_seg: uint16_t,
    pub boot_load_size: uint16_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct isoent {
    pub rbnode: archive_rb_node,
    pub file: *mut isofile,
    pub parent: *mut isoent,
    pub children: C2RustUnnamed_3,
    pub rbtree: archive_rb_tree,
    pub subdirs: C2RustUnnamed_2,
    pub children_sorted: *mut *mut isoent,
    pub chnext: *mut isoent,
    pub drnext: *mut isoent,
    pub ptnext: *mut isoent,
    pub dir_number: libc::c_int,
    pub dr_len: C2RustUnnamed_1,
    pub dir_location: uint32_t,
    pub dir_block: libc::c_int,
    pub identifier: *mut libc::c_char,
    pub ext_off: libc::c_int,
    pub ext_len: libc::c_int,
    pub id_len: libc::c_int,
    pub mb_len: libc::c_int,
    pub rr_parent: *mut isoent,
    pub rr_child: *mut isoent,
    pub extr_rec_list: C2RustUnnamed_0,
    #[bitfield(name = "virtual_0", ty = "libc::c_int", bits = "0..=0")]
    #[bitfield(name = "dir", ty = "libc::c_int", bits = "1..=1")]
    pub virtual_0_dir: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub first: *mut extr_rec,
    pub last: *mut *mut extr_rec,
    pub current: *mut extr_rec,
}
/*
 * Manage extra records.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extr_rec {
    pub location: libc::c_int,
    pub offset: libc::c_int,
    pub buf: [libc::c_uchar; 2048],
    pub next: *mut extr_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub vd: libc::c_int,
    pub self_0: libc::c_int,
    pub parent: libc::c_int,
    pub normal: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub first: *mut isoent,
    pub last: *mut *mut isoent,
    pub cnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree {
    pub rbt_root: *mut archive_rb_node,
    pub rbt_ops: *const archive_rb_tree_ops,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree_ops {
    pub rbto_compare_nodes: archive_rbto_compare_nodes_fn,
    pub rbto_compare_key: archive_rbto_compare_key_fn,
}
pub type archive_rbto_compare_key_fn =
    Option<unsafe extern "C" fn(_: *const archive_rb_node, _: *const libc::c_void) -> libc::c_int>;
/*-
 * Copyright (c) 2001 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Matt Thomas <matt@3am-software.com>.
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
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Based on NetBSD: rb.h,v 1.13 2009/08/16 10:57:01 yamt Exp
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_node {
    pub rb_nodes: [*mut archive_rb_node; 2],
    pub rb_info: uintptr_t,
}
/*
 * archive_rbto_compare_nodes_fn:
 *	return a positive value if the first node < the second node.
 *	return a negative value if the first node > the second node.
 *	return 0 if they are considered same.
 *
 * archive_rbto_compare_key_fn:
 *	return a positive value if the node < the key.
 *	return a negative value if the node > the key.
 *	return 0 if they are considered same.
 */
pub type archive_rbto_compare_nodes_fn = Option<
    unsafe extern "C" fn(_: *const archive_rb_node, _: *const archive_rb_node) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub first: *mut isoent,
    pub last: *mut *mut isoent,
    pub cnt: libc::c_int,
}
/*
 * The relation of struct isofile and isoent and archive_entry.
 *
 * Primary volume tree  --> struct isoent
 *                                |
 *                                v
 *                          struct isofile --> archive_entry
 *                                ^
 *                                |
 * Joliet volume tree   --> struct isoent
 *
 * struct isoent has specific information for volume.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct isofile {
    pub allnext: *mut isofile,
    pub datanext: *mut isofile,
    pub hlnext: *mut isofile,
    pub hardlink_target: *mut isofile,
    pub entry: *mut archive_entry,
    pub parentdir: archive_string,
    pub basename: archive_string,
    pub basename_utf16: archive_string,
    pub symlink: archive_string,
    pub dircnt: libc::c_int,
    pub content: content,
    pub cur_content: *mut content,
    pub write_content: libc::c_int,
    pub boot: C2RustUnnamed_5,
    pub zisofs: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub header_size: libc::c_uchar,
    pub log2_bs: libc::c_uchar,
    pub uncompressed_size: uint32_t,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const BOOT_IMAGE: C2RustUnnamed_5 = 2;
pub const BOOT_CATALOG: C2RustUnnamed_5 = 1;
pub const NO: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct content {
    pub offset_of_temp: int64_t,
    pub size: int64_t,
    pub blocks: libc::c_int,
    pub location: uint32_t,
    pub next: *mut content,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const WB_TO_TEMP: C2RustUnnamed_6 = 1;
pub const WB_TO_STREAM: C2RustUnnamed_6 = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    #[bitfield(name = "detect_magic", ty = "libc::c_int", bits = "0..=0")]
    #[bitfield(name = "making", ty = "libc::c_int", bits = "1..=1")]
    #[bitfield(name = "allzero", ty = "libc::c_int", bits = "2..=2")]
    pub detect_magic_making_allzero: [u8; 1],
    pub magic_buffer: [libc::c_uchar; 64],
    pub magic_cnt: libc::c_int,
    pub block_pointers: *mut uint32_t,
    pub block_pointers_allocated: size_t,
    pub block_pointers_cnt: libc::c_int,
    pub block_pointers_idx: libc::c_int,
    pub total_size: int64_t,
    pub block_offset: int64_t,
    pub stream: z_stream,
    pub stream_valid: libc::c_int,
    pub remaining: int64_t,
    pub compression_level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vdd {
    pub rootent: *mut isoent,
    pub vdd_type: vdd_type,
    pub pathtbl: *mut path_table,
    pub max_depth: libc::c_int,
    pub path_table_block: libc::c_int,
    pub path_table_size: libc::c_int,
    pub location_type_L_path_table: libc::c_int,
    pub location_type_M_path_table: libc::c_int,
    pub total_dir_block: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct path_table {
    pub first: *mut isoent,
    pub last: *mut *mut isoent,
    pub sorted: *mut *mut isoent,
    pub cnt: libc::c_int,
}
pub type vdd_type = libc::c_uint;
pub const VDD_ENHANCED: vdd_type = 2;
pub const VDD_JOLIET: vdd_type = 1;
pub const VDD_PRIMARY: vdd_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub first: *mut isofile,
    pub last: *mut *mut isofile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub first: *mut isofile,
    pub last: *mut *mut isofile,
}
/*
 * Types of Directory Record
 */
pub type dir_rec_type = libc::c_uint;
/* Stored as Child.		*/
/* Stored as Parent Directory.	*/
pub const DIR_REC_NORMAL: dir_rec_type = 3;
/* Stored as Current Directory.	*/
pub const DIR_REC_PARENT: dir_rec_type = 2;
/* Stored in Volume Descriptor.	*/
pub const DIR_REC_SELF: dir_rec_type = 1;
pub const DIR_REC_VD: dir_rec_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctl_extr_rec {
    pub use_extr: libc::c_int,
    pub bp: *mut libc::c_uchar,
    pub isoent: *mut isoent,
    pub ce_ptr: *mut libc::c_uchar,
    pub cur_len: libc::c_int,
    pub dr_len: libc::c_int,
    pub limit: libc::c_int,
    pub extr_off: libc::c_int,
    pub extr_loc: libc::c_int,
}
pub type keytype = libc::c_uint;
pub const KEY_HEX: keytype = 3;
pub const KEY_INT: keytype = 2;
pub const KEY_STR: keytype = 1;
pub const KEY_FLG: keytype = 0;
/*
 * Types of Volume Descriptor
 */
pub type VD_type = libc::c_uint;
/* Volume Descriptor Set Terminator	*/
/* Supplementary Volume Descriptor	*/
pub const VDT_TERMINATOR: VD_type = 255;
/* Primary Volume Descriptor		*/
pub const VDT_SUPPLEMENTARY: VD_type = 2;
/* Boot Record Volume Descriptor 	*/
pub const VDT_PRIMARY: VD_type = 1;
pub const VDT_BOOT_RECORD: VD_type = 0;
pub type char_type = libc::c_uint;
pub const D_CHAR: char_type = 1;
pub const A_CHAR: char_type = 0;
/*
 * Kinds of Volume Descriptor Character
 */
pub type vdc = libc::c_uint;
pub const VDC_UCS2_DIRECT: vdc = 3;
pub const VDC_UCS2: vdc = 2;
pub const VDC_LOWERCASE: vdc = 1;
pub const VDC_STD: vdc = 0;
/*
 * IDentifier Resolver.
 * Used for resolving duplicated filenames.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct idr {
    pub idrent_pool: *mut idrent,
    pub rbtree: archive_rb_tree,
    pub wait_list: C2RustUnnamed_10,
    pub pool_size: libc::c_int,
    pub pool_idx: libc::c_int,
    pub num_size: libc::c_int,
    pub null_size: libc::c_int,
    pub char_map: [libc::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub first: *mut idrent,
    pub last: *mut *mut idrent,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct idrent {
    pub rbnode: archive_rb_node,
    pub wnext: *mut idrent,
    pub avail: *mut idrent,
    pub isoent: *mut isoent,
    pub weight: libc::c_int,
    pub noff: libc::c_int,
    pub rename_num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hardlink {
    pub rbnode: archive_rb_node,
    pub nlink: libc::c_int,
    pub file_list: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub first: *mut isofile,
    pub last: *mut *mut isofile,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct zisofs_extract {
    pub pz_log2_bs: libc::c_int,
    pub pz_uncompressed_size: uint64_t,
    pub uncompressed_buffer_size: size_t,
    #[bitfield(name = "initialized", ty = "libc::c_int", bits = "0..=0")]
    #[bitfield(name = "header_passed", ty = "libc::c_int", bits = "1..=1")]
    pub initialized_header_passed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub pz_offset: uint32_t,
    pub block_pointers: *mut libc::c_uchar,
    pub block_pointers_size: size_t,
    pub block_pointers_avail: size_t,
    pub block_off: size_t,
    pub block_avail: uint32_t,
    pub stream: z_stream,
    pub stream_valid: libc::c_int,
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
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_STREAM_ERROR: libc::c_int = -(2 as libc::c_int);
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const Z_VERSION_ERROR: libc::c_int = -(6 as libc::c_int);
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * Codes to identify various stream filters.
 */
/*
 * Codes returned by archive_format.
 *
 * Top 16 bits identifies the format family (e.g., "tar"); lower
 * 16 bits indicate the variant.  This is updated by read_next_header.
 * Note that the lower 16 bits will often vary from entry to entry.
 * In some cases, this variation occurs as libarchive learns more about
 * the archive (for example, later entries might utilize extensions that
 * weren't necessary earlier in the archive; in this case, libarchive
 * will change the format code to indicate the extended format that
 * was used).  In other cases, it's because different tools have
 * modified the archive and so different parts of the archive
 * actually have slightly different formats.  (Both tar and cpio store
 * format codes in each entry, so it is quite possible for each
 * entry to be in a different format.)
 */
pub const ARCHIVE_FORMAT_ISO9660: libc::c_int = 0x40000 as libc::c_int;
#[inline]
unsafe extern "C" fn archive_le16dec(mut pp: *const libc::c_void) -> uint16_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return (p1 << 8 as libc::c_int | p0) as uint16_t;
}
#[inline]
unsafe extern "C" fn archive_le32dec(mut pp: *const libc::c_void) -> uint32_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p3: libc::c_uint = *p.offset(3 as libc::c_int as isize) as libc::c_uint;
    let mut p2: libc::c_uint = *p.offset(2 as libc::c_int as isize) as libc::c_uint;
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return p3 << 24 as libc::c_int | p2 << 16 as libc::c_int | p1 << 8 as libc::c_int | p0;
}
#[inline]
unsafe extern "C" fn archive_be16enc(mut pp: *mut libc::c_void, mut u: uint16_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_be32enc(mut pp: *mut libc::c_void, mut u: uint32_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) =
        (u >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(3 as libc::c_int as isize) =
        (u & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_le16enc(mut pp: *mut libc::c_void, mut u: uint16_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_le32enc(mut pp: *mut libc::c_void, mut u: uint32_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) =
        (u >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(3 as libc::c_int as isize) =
        (u >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
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
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const archive_entry_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_pathname_l;
pub const ARCHIVE_RB_DIR_LEFT: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_RB_DIR_RIGHT: libc::c_int = 1 as libc::c_int;
/*-
 * Copyright (c) 2009-2012 Michihiro NAKAJIMA
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
 */
/*#define DEBUG 1*/
pub const LOGICAL_BLOCK_BITS: libc::c_int = 11 as libc::c_int;
pub const LOGICAL_BLOCK_SIZE: libc::c_int = 2048 as libc::c_int;
pub const PATH_TABLE_BLOCK_SIZE: libc::c_int = 4096 as libc::c_int;
pub const SYSTEM_AREA_BLOCK: libc::c_int = 16 as libc::c_int;
pub const PRIMARY_VOLUME_DESCRIPTOR_BLOCK: libc::c_int = 1 as libc::c_int;
pub const SUPPLEMENTARY_VOLUME_DESCRIPTOR_BLOCK: libc::c_int = 1 as libc::c_int;
pub const BOOT_RECORD_DESCRIPTOR_BLOCK: libc::c_int = 1 as libc::c_int;
pub const VOLUME_DESCRIPTOR_SET_TERMINATOR_BLOCK: libc::c_int = 1 as libc::c_int;
pub const NON_ISO_FILE_SYSTEM_INFORMATION_BLOCK: libc::c_int = 1 as libc::c_int;
pub const RRIP_ER_BLOCK: libc::c_int = 1 as libc::c_int;
pub const PADDING_BLOCK: libc::c_int = 150 as libc::c_int;
pub const FD_1_2M_SIZE: libc::c_int = 1024 as libc::c_int * 1200 as libc::c_int;
pub const FD_1_44M_SIZE: libc::c_int = 1024 as libc::c_int * 1440 as libc::c_int;
pub const FD_2_88M_SIZE: libc::c_int = 1024 as libc::c_int * 2880 as libc::c_int;
pub const MULTI_EXTENT_SIZE: libc::c_longlong = (1 as libc::c_longlong) << 32 as libc::c_int;
/* 4Gi bytes. */
pub const MAX_DEPTH: libc::c_int = 8 as libc::c_int;
pub const RR_CE_SIZE: libc::c_int = 28 as libc::c_int;
/* SUSP "CE" extension size */
pub const FILE_FLAG_DIRECTORY: libc::c_int = 0x2 as libc::c_int;
pub const FILE_FLAG_MULTI_EXTENT: libc::c_int = 0x80 as libc::c_int;
static mut rrip_identifier: [libc::c_char; 11] =
    unsafe { *::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"RRIP_1991A\x00") };
static mut rrip_descriptor: [libc::c_char; 85] = unsafe {
    *::std::mem::transmute::<&[u8; 85], &[libc::c_char; 85]>(
        b"THE ROCK RIDGE INTERCHANGE PROTOCOL PROVIDES SUPPORT FOR POSIX FILE SYSTEM SEMANTICS\x00",
    )
};
static mut rrip_source: [libc::c_char; 136] = unsafe {
    *::std::mem::transmute::<&[u8; 136],
                                 &[libc::c_char; 136]>(b"PLEASE CONTACT DISC PUBLISHER FOR SPECIFICATION SOURCE.  SEE PUBLISHER IDENTIFIER IN PRIMARY VOLUME DESCRIPTOR FOR CONTACT INFORMATION.\x00")
};
static mut zisofs_magic: [libc::c_uchar; 8] = [
    0x37 as libc::c_int as libc::c_uchar,
    0xe4 as libc::c_int as libc::c_uchar,
    0x53 as libc::c_int as libc::c_uchar,
    0x96 as libc::c_int as libc::c_uchar,
    0xc9 as libc::c_int as libc::c_uchar,
    0xdb as libc::c_int as libc::c_uchar,
    0xd6 as libc::c_int as libc::c_uchar,
    0x7 as libc::c_int as libc::c_uchar,
];
pub const ZF_HEADER_SIZE: libc::c_int = 16 as libc::c_int;
/* zisofs header size. */
pub const ZF_LOG2_BS: libc::c_int = 15 as libc::c_int;
/* log2 block size; 32K bytes. */
pub const ZF_BLOCK_SIZE: libc::c_ulong = (1 as libc::c_ulong) << ZF_LOG2_BS;
pub const DR_SAFETY: libc::c_int = RR_CE_SIZE;
pub const DR_LIMIT: libc::c_int = 254 as libc::c_int - DR_SAFETY;
pub const OPT_ABSTRACT_FILE_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const ABSTRACT_FILE_SIZE: libc::c_int = 37 as libc::c_int;
pub const OPT_APPLICATION_ID_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const APPLICATION_IDENTIFIER_SIZE: libc::c_int = 128 as libc::c_int;
pub const OPT_ALLOW_VERNUM_DEFAULT: libc::c_int = 1 as libc::c_int;
pub const OPT_BIBLIO_FILE_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const BIBLIO_FILE_SIZE: libc::c_int = 37 as libc::c_int;
pub const OPT_BOOT_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const OPT_BOOT_CATALOG_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const OPT_BOOT_INFO_TABLE_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const OPT_BOOT_LOAD_SEG_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const OPT_BOOT_LOAD_SIZE_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const OPT_BOOT_TYPE_AUTO: libc::c_int = 0 as libc::c_int;
pub const OPT_BOOT_TYPE_NO_EMU: libc::c_int = 1 as libc::c_int;
pub const OPT_BOOT_TYPE_FD: libc::c_int = 2 as libc::c_int;
pub const OPT_BOOT_TYPE_HARD_DISK: libc::c_int = 3 as libc::c_int;
pub const OPT_BOOT_TYPE_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const OPT_COMPRESSION_LEVEL_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const OPT_COPYRIGHT_FILE_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const COPYRIGHT_FILE_SIZE: libc::c_int = 37 as libc::c_int;
pub const OPT_ISO_LEVEL_DEFAULT: libc::c_int = 1 as libc::c_int;
pub const OPT_JOLIET_DISABLE: libc::c_int = 0 as libc::c_int;
pub const OPT_JOLIET_ENABLE: libc::c_int = 1 as libc::c_int;
pub const OPT_JOLIET_LONGNAME: libc::c_int = 2 as libc::c_int;
pub const OPT_JOLIET_DEFAULT: libc::c_int = OPT_JOLIET_ENABLE;
pub const OPT_LIMIT_DEPTH_DEFAULT: libc::c_int = 1 as libc::c_int;
pub const OPT_LIMIT_DIRS_DEFAULT: libc::c_int = 1 as libc::c_int;
pub const OPT_PAD_DEFAULT: libc::c_int = 1 as libc::c_int;
pub const OPT_PUBLISHER_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const PUBLISHER_IDENTIFIER_SIZE: libc::c_int = 128 as libc::c_int;
pub const OPT_RR_DISABLED: libc::c_int = 0 as libc::c_int;
pub const OPT_RR_STRICT: libc::c_int = 1 as libc::c_int;
pub const OPT_RR_USEFUL: libc::c_int = 2 as libc::c_int;
pub const OPT_RR_DEFAULT: libc::c_int = 2 as libc::c_int;
pub const OPT_VOLUME_ID_DEFAULT: libc::c_int = 0 as libc::c_int;
pub const VOLUME_IDENTIFIER_SIZE: libc::c_int = 32 as libc::c_int;
pub const OPT_ZISOFS_DISABLED: libc::c_int = 0 as libc::c_int;
pub const OPT_ZISOFS_DIRECT: libc::c_int = 1 as libc::c_int;
pub const OPT_ZISOFS_DEFAULT: libc::c_int = OPT_ZISOFS_DISABLED;
pub const BOOT_PLATFORM_X86: libc::c_int = 0 as libc::c_int;
pub const BOOT_MEDIA_NO_EMULATION: libc::c_int = 0 as libc::c_int;
pub const BOOT_MEDIA_1_2M_DISKETTE: libc::c_int = 1 as libc::c_int;
pub const BOOT_MEDIA_1_44M_DISKETTE: libc::c_int = 2 as libc::c_int;
pub const BOOT_MEDIA_2_88M_DISKETTE: libc::c_int = 3 as libc::c_int;
pub const BOOT_MEDIA_HARD_DISK: libc::c_int = 4 as libc::c_int;
pub const BOOT_LOAD_SIZE: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_iso9660(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_iso9660\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If another format was already registered, unregister it. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    iso9660 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<iso9660>() as libc::c_ulong,
    ) as *mut iso9660;
    if iso9660.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate iso9660 data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*iso9660).birth_time = 0 as libc::c_int as time_t;
    (*iso9660).temp_fd = -(1 as libc::c_int);
    (*iso9660).cur_file = NULL as *mut isofile;
    (*iso9660).primary.max_depth = 0 as libc::c_int;
    (*iso9660).primary.vdd_type = VDD_PRIMARY;
    (*iso9660).primary.pathtbl = NULL as *mut path_table;
    (*iso9660).joliet.rootent = NULL as *mut isoent;
    (*iso9660).joliet.max_depth = 0 as libc::c_int;
    (*iso9660).joliet.vdd_type = VDD_JOLIET;
    (*iso9660).joliet.pathtbl = NULL as *mut path_table;
    isofile_init_entry_list(iso9660);
    isofile_init_entry_data_file_list(iso9660);
    isofile_init_hardlinks(iso9660);
    (*iso9660).directories_too_deep = NULL as *mut isoent;
    (*iso9660).dircnt_max = 1 as libc::c_int;
    (*iso9660).wbuff_remaining = (LOGICAL_BLOCK_SIZE * 32 as libc::c_int) as size_t;
    (*iso9660).wbuff_type = WB_TO_TEMP;
    (*iso9660).wbuff_offset = 0 as libc::c_int as int64_t;
    (*iso9660).wbuff_written = 0 as libc::c_int as int64_t;
    (*iso9660).wbuff_tail = 0 as libc::c_int as int64_t;
    (*iso9660).utf16be.s = NULL as *mut libc::c_char;
    (*iso9660).utf16be.length = 0 as libc::c_int as size_t;
    (*iso9660).utf16be.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).mbs.s = NULL as *mut libc::c_char;
    (*iso9660).mbs.length = 0 as libc::c_int as size_t;
    (*iso9660).mbs.buffer_length = 0 as libc::c_int as size_t;
    /*
     * Init Identifiers used for PVD and SVD.
     */
    (*iso9660).volume_identifier.s = NULL as *mut libc::c_char;
    (*iso9660).volume_identifier.length = 0 as libc::c_int as size_t;
    (*iso9660).volume_identifier.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).volume_identifier.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*iso9660).volume_identifier,
        b"CDROM\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        (if (b"CDROM\x00" as *const u8 as *const libc::c_char).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(b"CDROM\x00" as *const u8 as *const libc::c_char)
        }),
    );
    (*iso9660).publisher_identifier.s = NULL as *mut libc::c_char;
    (*iso9660).publisher_identifier.length = 0 as libc::c_int as size_t;
    (*iso9660).publisher_identifier.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).data_preparer_identifier.s = NULL as *mut libc::c_char;
    (*iso9660).data_preparer_identifier.length = 0 as libc::c_int as size_t;
    (*iso9660).data_preparer_identifier.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).application_identifier.s = NULL as *mut libc::c_char;
    (*iso9660).application_identifier.length = 0 as libc::c_int as size_t;
    (*iso9660).application_identifier.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).application_identifier.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*iso9660).application_identifier,
        archive_version_string() as *const libc::c_void,
        (if archive_version_string().is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(archive_version_string())
        }),
    );
    (*iso9660).copyright_file_identifier.s = NULL as *mut libc::c_char;
    (*iso9660).copyright_file_identifier.length = 0 as libc::c_int as size_t;
    (*iso9660).copyright_file_identifier.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).abstract_file_identifier.s = NULL as *mut libc::c_char;
    (*iso9660).abstract_file_identifier.length = 0 as libc::c_int as size_t;
    (*iso9660).abstract_file_identifier.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).bibliographic_file_identifier.s = NULL as *mut libc::c_char;
    (*iso9660).bibliographic_file_identifier.length = 0 as libc::c_int as size_t;
    (*iso9660).bibliographic_file_identifier.buffer_length = 0 as libc::c_int as size_t;
    /*
     * Init El Torito bootable CD variables.
     */
    (*iso9660).el_torito.catalog_filename.s = NULL as *mut libc::c_char;
    (*iso9660).el_torito.catalog_filename.length = 0 as libc::c_int as size_t;
    (*iso9660).el_torito.catalog_filename.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).el_torito.catalog = NULL as *mut isoent;
    /* Set default file name of boot catalog  */
    (*iso9660).el_torito.catalog_filename.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*iso9660).el_torito.catalog_filename,
        b"boot.catalog\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        (if (b"boot.catalog\x00" as *const u8 as *const libc::c_char).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(b"boot.catalog\x00" as *const u8 as *const libc::c_char)
        }),
    );
    (*iso9660).el_torito.boot_filename.s = NULL as *mut libc::c_char;
    (*iso9660).el_torito.boot_filename.length = 0 as libc::c_int as size_t;
    (*iso9660).el_torito.boot_filename.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).el_torito.boot = NULL as *mut isoent;
    (*iso9660).el_torito.platform_id = BOOT_PLATFORM_X86 as libc::c_uchar;
    (*iso9660).el_torito.id.s = NULL as *mut libc::c_char;
    (*iso9660).el_torito.id.length = 0 as libc::c_int as size_t;
    (*iso9660).el_torito.id.buffer_length = 0 as libc::c_int as size_t;
    (*iso9660).el_torito.boot_load_seg = 0 as libc::c_int as uint16_t;
    (*iso9660).el_torito.boot_load_size = BOOT_LOAD_SIZE as uint16_t;
    /*
     * Init zisofs variables.
     */
    (*iso9660).zisofs.block_pointers = NULL as *mut uint32_t;
    (*iso9660).zisofs.block_pointers_allocated = 0 as libc::c_int as size_t;
    (*iso9660).zisofs.stream_valid = 0 as libc::c_int;
    (*iso9660).zisofs.compression_level = 9 as libc::c_int;
    memset(
        &mut (*iso9660).zisofs.stream as *mut z_stream as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong,
    );
    /*
     * Set default value of iso9660 options.
     */
    (*iso9660)
        .opt
        .set_abstract_file(OPT_ABSTRACT_FILE_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_application_id(OPT_APPLICATION_ID_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_allow_vernum(OPT_ALLOW_VERNUM_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_biblio_file(OPT_BIBLIO_FILE_DEFAULT as libc::c_uint);
    (*iso9660).opt.set_boot(OPT_BOOT_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_boot_catalog(OPT_BOOT_CATALOG_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_boot_info_table(OPT_BOOT_INFO_TABLE_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_boot_load_seg(OPT_BOOT_LOAD_SEG_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_boot_load_size(OPT_BOOT_LOAD_SIZE_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_boot_type(OPT_BOOT_TYPE_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_compression_level(OPT_COMPRESSION_LEVEL_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_copyright_file(OPT_COPYRIGHT_FILE_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_iso_level(OPT_ISO_LEVEL_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_joliet(OPT_JOLIET_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_limit_depth(OPT_LIMIT_DEPTH_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_limit_dirs(OPT_LIMIT_DIRS_DEFAULT as libc::c_uint);
    (*iso9660).opt.set_pad(OPT_PAD_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_publisher(OPT_PUBLISHER_DEFAULT as libc::c_uint);
    (*iso9660).opt.set_rr(OPT_RR_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_volume_id(OPT_VOLUME_ID_DEFAULT as libc::c_uint);
    (*iso9660)
        .opt
        .set_zisofs(OPT_ZISOFS_DEFAULT as libc::c_uint);
    /* Create the root directory. */
    (*iso9660).primary.rootent =
        isoent_create_virtual_dir(a, iso9660, b"\x00" as *const u8 as *const libc::c_char);
    if (*iso9660).primary.rootent.is_null() {
        free(iso9660 as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*(*iso9660).primary.rootent).parent = (*iso9660).primary.rootent;
    (*iso9660).cur_dirent = (*iso9660).primary.rootent;
    (*iso9660).cur_dirstr.s = NULL as *mut libc::c_char;
    (*iso9660).cur_dirstr.length = 0 as libc::c_int as size_t;
    (*iso9660).cur_dirstr.buffer_length = 0 as libc::c_int as size_t;
    archive_string_ensure(&mut (*iso9660).cur_dirstr, 1 as libc::c_int as size_t);
    *(*iso9660).cur_dirstr.s.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*iso9660).sconv_to_utf16be = NULL as *mut archive_string_conv;
    (*iso9660).sconv_from_utf16be = NULL as *mut archive_string_conv;
    (*a).format_data = iso9660 as *mut libc::c_void;
    (*a).format_name = b"iso9660\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        iso9660_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        iso9660_write_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        iso9660_write_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry =
        Some(iso9660_finish_entry as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_close =
        Some(iso9660_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_free =
        Some(iso9660_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).archive.archive_format = ARCHIVE_FORMAT_ISO9660;
    (*a).archive.archive_format_name = b"ISO9660\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_str_opt(
    mut a: *mut archive_write,
    mut s: *mut archive_string,
    mut maxsize: size_t,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    if strlen(value) > maxsize {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Value is longer than %zu characters for option ``%s\'\'\x00" as *const u8
                as *const libc::c_char,
            maxsize,
            key,
        );
        return -(30 as libc::c_int);
    }
    (*s).length = 0 as libc::c_int as size_t;
    archive_strncat(
        s,
        value as *const libc::c_void,
        (if value.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(value)
        }),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_num_opt(
    mut a: *mut archive_write,
    mut num: *mut libc::c_int,
    mut high: libc::c_int,
    mut low: libc::c_int,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = value;
    let mut data: libc::c_int = 0 as libc::c_int;
    let mut neg: libc::c_int = 0 as libc::c_int;
    if p.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Invalid value(empty) for option ``%s\'\'\x00" as *const u8 as *const libc::c_char,
            key,
        );
        return -(30 as libc::c_int);
    }
    if *p as libc::c_int == '-' as i32 {
        neg = 1 as libc::c_int;
        p = p.offset(1)
    }
    while *p != 0 {
        if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            data = data * 10 as libc::c_int + *p as libc::c_int - '0' as i32
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid value for option ``%s\'\'\x00" as *const u8 as *const libc::c_char,
                key,
            );
            return -(30 as libc::c_int);
        }
        if data > high {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid value(over %d) for option ``%s\'\'\x00" as *const u8
                    as *const libc::c_char,
                high,
                key,
            );
            return -(30 as libc::c_int);
        }
        if data < low {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid value(under %d) for option ``%s\'\'\x00" as *const u8
                    as *const libc::c_char,
                low,
                key,
            );
            return -(30 as libc::c_int);
        }
        p = p.offset(1)
    }
    if neg != 0 {
        data *= -(1 as libc::c_int)
    }
    *num = data;
    return 0 as libc::c_int;
}
unsafe extern "C" fn iso9660_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    match *key.offset(0 as libc::c_int as isize) as libc::c_int {
        97 => {
            if strcmp(
                key,
                b"abstract-file\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                r = get_str_opt(
                    a,
                    &mut (*iso9660).abstract_file_identifier,
                    ABSTRACT_FILE_SIZE as size_t,
                    key,
                    value,
                );
                (*iso9660)
                    .opt
                    .set_abstract_file((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                return r;
            }
            if strcmp(
                key,
                b"application-id\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                r = get_str_opt(
                    a,
                    &mut (*iso9660).application_identifier,
                    APPLICATION_IDENTIFIER_SIZE as size_t,
                    key,
                    value,
                );
                (*iso9660)
                    .opt
                    .set_application_id((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                return r;
            }
            if strcmp(key, b"allow-vernum\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*iso9660).opt.set_allow_vernum(
                    (value != NULL as *const libc::c_char) as libc::c_int as libc::c_uint,
                );
                return 0 as libc::c_int;
            }
            current_block = 17392506108461345148;
        }
        98 => {
            if strcmp(key, b"biblio-file\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                r = get_str_opt(
                    a,
                    &mut (*iso9660).bibliographic_file_identifier,
                    BIBLIO_FILE_SIZE as size_t,
                    key,
                    value,
                );
                (*iso9660)
                    .opt
                    .set_biblio_file((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                return r;
            }
            if strcmp(key, b"boot\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                if value.is_null() {
                    (*iso9660).opt.set_boot(0 as libc::c_int as libc::c_uint)
                } else {
                    (*iso9660).opt.set_boot(1 as libc::c_int as libc::c_uint);
                    (*iso9660).el_torito.boot_filename.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut (*iso9660).el_torito.boot_filename,
                        value as *const libc::c_void,
                        (if value.is_null() {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            strlen(value)
                        }),
                    );
                }
                return 0 as libc::c_int;
            }
            if strcmp(key, b"boot-catalog\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                r = get_str_opt(
                    a,
                    &mut (*iso9660).el_torito.catalog_filename,
                    1024 as libc::c_int as size_t,
                    key,
                    value,
                );
                (*iso9660)
                    .opt
                    .set_boot_catalog((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                return r;
            }
            if strcmp(
                key,
                b"boot-info-table\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*iso9660).opt.set_boot_info_table(
                    (value != NULL as *const libc::c_char) as libc::c_int as libc::c_uint,
                );
                return 0 as libc::c_int;
            }
            if strcmp(
                key,
                b"boot-load-seg\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut seg: uint32_t = 0;
                (*iso9660)
                    .opt
                    .set_boot_load_seg(0 as libc::c_int as libc::c_uint);
                if value.is_null() {
                    current_block = 17691188858882045618;
                } else {
                    seg = 0 as libc::c_int as uint32_t;
                    p = value;
                    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
                        && (*p.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                            || *p.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
                    {
                        p = p.offset(2 as libc::c_int as isize)
                    }
                    loop {
                        if !(*p != 0) {
                            current_block = 2516253395664191498;
                            break;
                        }
                        if seg != 0 {
                            seg <<= 4 as libc::c_int
                        }
                        if *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'F' as i32 {
                            seg = (seg as libc::c_uint).wrapping_add(
                                (*p as libc::c_int - 'A' as i32 + 0xa as libc::c_int)
                                    as libc::c_uint,
                            ) as uint32_t as uint32_t
                        } else if *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'f' as i32
                        {
                            seg = (seg as libc::c_uint).wrapping_add(
                                (*p as libc::c_int - 'a' as i32 + 0xa as libc::c_int)
                                    as libc::c_uint,
                            ) as uint32_t as uint32_t
                        } else {
                            if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32)
                            {
                                current_block = 17691188858882045618;
                                break;
                            }
                            seg = (seg as libc::c_uint)
                                .wrapping_add((*p as libc::c_int - '0' as i32) as libc::c_uint)
                                as uint32_t as uint32_t
                        }
                        if seg > 0xffff as libc::c_int as libc::c_uint {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"Invalid value(over 0xffff) for option ``%s\'\'\x00" as *const u8
                                    as *const libc::c_char,
                                key,
                            );
                            return -(30 as libc::c_int);
                        }
                        p = p.offset(1)
                    }
                    match current_block {
                        17691188858882045618 => {}
                        _ => {
                            (*iso9660).el_torito.boot_load_seg = seg as uint16_t;
                            (*iso9660)
                                .opt
                                .set_boot_load_seg(1 as libc::c_int as libc::c_uint);
                            return 0 as libc::c_int;
                        }
                    }
                }
            } else {
                if strcmp(
                    key,
                    b"boot-load-size\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut num: libc::c_int = 0 as libc::c_int;
                    r = get_num_opt(
                        a,
                        &mut num,
                        0xffff as libc::c_int,
                        1 as libc::c_int,
                        key,
                        value,
                    );
                    (*iso9660)
                        .opt
                        .set_boot_load_size((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                    if r != ARCHIVE_OK {
                        return -(30 as libc::c_int);
                    }
                    (*iso9660).el_torito.boot_load_size = num as uint16_t;
                    return 0 as libc::c_int;
                }
                if strcmp(key, b"boot-type\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if value.is_null() {
                        current_block = 17691188858882045618;
                    } else {
                        if strcmp(
                            value,
                            b"no-emulation\x00" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            (*iso9660)
                                .opt
                                .set_boot_type(OPT_BOOT_TYPE_NO_EMU as libc::c_uint);
                            current_block = 7189308829251266000;
                        } else if strcmp(value, b"fd\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            (*iso9660)
                                .opt
                                .set_boot_type(OPT_BOOT_TYPE_FD as libc::c_uint);
                            current_block = 7189308829251266000;
                        } else if strcmp(
                            value,
                            b"hard-disk\x00" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            (*iso9660)
                                .opt
                                .set_boot_type(OPT_BOOT_TYPE_HARD_DISK as libc::c_uint);
                            current_block = 7189308829251266000;
                        } else {
                            current_block = 17691188858882045618;
                        }
                        match current_block {
                            17691188858882045618 => {}
                            _ => return 0 as libc::c_int,
                        }
                    }
                } else {
                    current_block = 17392506108461345148;
                }
            }
        }
        99 => {
            if strcmp(
                key,
                b"compression-level\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                if value.is_null()
                    || !(*value.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                        && *value.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
                    || *value.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
                {
                    current_block = 17691188858882045618;
                } else {
                    (*iso9660).zisofs.compression_level =
                        *value.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
                    (*iso9660)
                        .opt
                        .set_compression_level(1 as libc::c_int as libc::c_uint);
                    return 0 as libc::c_int;
                }
            } else {
                if strcmp(
                    key,
                    b"copyright-file\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    r = get_str_opt(
                        a,
                        &mut (*iso9660).copyright_file_identifier,
                        COPYRIGHT_FILE_SIZE as size_t,
                        key,
                        value,
                    );
                    (*iso9660)
                        .opt
                        .set_copyright_file((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                    return r;
                }
                current_block = 17392506108461345148;
            }
        }
        105 => {
            if strcmp(key, b"iso-level\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                if !value.is_null()
                    && *value.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                    && (*value.offset(0 as libc::c_int as isize) as libc::c_int >= '1' as i32
                        && *value.offset(0 as libc::c_int as isize) as libc::c_int <= '4' as i32)
                {
                    (*iso9660).opt.set_iso_level(
                        (*value.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                            as libc::c_uint,
                    );
                    return 0 as libc::c_int;
                }
                current_block = 17691188858882045618;
            } else {
                current_block = 17392506108461345148;
            }
        }
        106 => {
            if strcmp(key, b"joliet\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                if value.is_null() {
                    (*iso9660)
                        .opt
                        .set_joliet(OPT_JOLIET_DISABLE as libc::c_uint);
                    current_block = 178030534879405462;
                } else if strcmp(value, b"1\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*iso9660).opt.set_joliet(OPT_JOLIET_ENABLE as libc::c_uint);
                    current_block = 178030534879405462;
                } else if strcmp(value, b"long\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*iso9660)
                        .opt
                        .set_joliet(OPT_JOLIET_LONGNAME as libc::c_uint);
                    current_block = 178030534879405462;
                } else {
                    current_block = 17691188858882045618;
                }
                match current_block {
                    17691188858882045618 => {}
                    _ => return 0 as libc::c_int,
                }
            } else {
                current_block = 17392506108461345148;
            }
        }
        108 => {
            if strcmp(key, b"limit-depth\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*iso9660).opt.set_limit_depth(
                    (value != NULL as *const libc::c_char) as libc::c_int as libc::c_uint,
                );
                return 0 as libc::c_int;
            }
            if strcmp(key, b"limit-dirs\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*iso9660).opt.set_limit_dirs(
                    (value != NULL as *const libc::c_char) as libc::c_int as libc::c_uint,
                );
                return 0 as libc::c_int;
            }
            current_block = 17392506108461345148;
        }
        112 => {
            if strcmp(key, b"pad\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*iso9660)
                    .opt
                    .set_pad((value != NULL as *const libc::c_char) as libc::c_int as libc::c_uint);
                return 0 as libc::c_int;
            }
            if strcmp(key, b"publisher\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                r = get_str_opt(
                    a,
                    &mut (*iso9660).publisher_identifier,
                    PUBLISHER_IDENTIFIER_SIZE as size_t,
                    key,
                    value,
                );
                (*iso9660)
                    .opt
                    .set_publisher((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                return r;
            }
            current_block = 17392506108461345148;
        }
        114 => {
            if strcmp(key, b"rockridge\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"Rockridge\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                if value.is_null() {
                    (*iso9660).opt.set_rr(OPT_RR_DISABLED as libc::c_uint);
                    current_block = 18218798608644444571;
                } else if strcmp(value, b"1\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*iso9660).opt.set_rr(OPT_RR_USEFUL as libc::c_uint);
                    current_block = 18218798608644444571;
                } else if strcmp(value, b"strict\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*iso9660).opt.set_rr(OPT_RR_STRICT as libc::c_uint);
                    current_block = 18218798608644444571;
                } else if strcmp(value, b"useful\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*iso9660).opt.set_rr(OPT_RR_USEFUL as libc::c_uint);
                    current_block = 18218798608644444571;
                } else {
                    current_block = 17691188858882045618;
                }
                match current_block {
                    17691188858882045618 => {}
                    _ => return 0 as libc::c_int,
                }
            } else {
                current_block = 17392506108461345148;
            }
        }
        118 => {
            if strcmp(key, b"volume-id\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                r = get_str_opt(
                    a,
                    &mut (*iso9660).volume_identifier,
                    VOLUME_IDENTIFIER_SIZE as size_t,
                    key,
                    value,
                );
                (*iso9660)
                    .opt
                    .set_volume_id((r == ARCHIVE_OK) as libc::c_int as libc::c_uint);
                return r;
            }
            current_block = 17392506108461345148;
        }
        122 => {
            if strcmp(key, b"zisofs\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                if value.is_null() {
                    (*iso9660)
                        .opt
                        .set_zisofs(OPT_ZISOFS_DISABLED as libc::c_uint)
                } else {
                    (*iso9660).opt.set_zisofs(OPT_ZISOFS_DIRECT as libc::c_uint)
                }
                return 0 as libc::c_int;
            }
            current_block = 17392506108461345148;
        }
        _ => {
            current_block = 17392506108461345148;
        }
    }
    match current_block {
        17392506108461345148 => {
            /* Note: The "warn" return is just to inform the options
             * supervisor that we didn't handle it.  It will generate
             * a suitable error if no one used this option. */
            return -(20 as libc::c_int);
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid value for option ``%s\'\'\x00" as *const u8 as *const libc::c_char,
                key,
            );
            return -(25 as libc::c_int);
        }
    };
}
unsafe extern "C" fn iso9660_write_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut file: *mut isofile = 0 as *mut isofile;
    let mut isoent: *mut isoent = 0 as *mut isoent;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    iso9660 = (*a).format_data as *mut iso9660;
    (*iso9660).cur_file = NULL as *mut isofile;
    (*iso9660).bytes_remaining = 0 as libc::c_int as uint64_t;
    (*iso9660).need_multi_extent = 0 as libc::c_int;
    if archive_entry_filetype(entry) == AE_IFLNK as mode_t
        && (*iso9660).opt.rr() as libc::c_int == OPT_RR_DISABLED
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Ignore symlink file.\x00" as *const u8 as *const libc::c_char,
        );
        (*iso9660).cur_file = NULL as *mut isofile;
        return -(20 as libc::c_int);
    }
    if archive_entry_filetype(entry) == AE_IFREG as mode_t
        && archive_entry_size(entry) as libc::c_longlong >= MULTI_EXTENT_SIZE
    {
        if ((*iso9660).opt.iso_level() as libc::c_int) < 3 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Ignore over %lld bytes file. This file too large.\x00" as *const u8
                    as *const libc::c_char,
                MULTI_EXTENT_SIZE,
            );
            (*iso9660).cur_file = NULL as *mut isofile;
            return -(20 as libc::c_int);
        }
        (*iso9660).need_multi_extent = 1 as libc::c_int
    }
    file = isofile_new(a, entry);
    if file.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    r = isofile_gen_utility_names(a, file);
    if r < ARCHIVE_WARN {
        isofile_free(file);
        return r;
    } else {
        if r < ret {
            ret = r
        }
    }
    /*
     * Ignore a path which looks like the top of directory name
     * since we have already made the root directory of an ISO image.
     */
    if (*file).parentdir.length == 0 as libc::c_int as libc::c_ulong
        && (*file).basename.length == 0 as libc::c_int as libc::c_ulong
    {
        isofile_free(file);
        return r;
    }
    isofile_add_entry(iso9660, file);
    isoent = isoent_new(file);
    if isoent.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*(*isoent).file).dircnt > (*iso9660).dircnt_max {
        (*iso9660).dircnt_max = (*(*isoent).file).dircnt
    }
    /* Add the current file into tree */
    r = isoent_tree(a, &mut isoent);
    if r != ARCHIVE_OK {
        return r;
    }
    /* If there is the same file in tree and
     * the current file is older than the file in tree.
     * So we don't need the current file data anymore. */
    if (*isoent).file != file {
        return 0 as libc::c_int;
    }
    /* Non regular files contents are unneeded to be saved to
     * temporary files. */
    if archive_entry_filetype((*file).entry) != AE_IFREG as mode_t {
        return ret;
    }
    /*
     * Set the current file to cur_file to read its contents.
     */
    (*iso9660).cur_file = file;
    if archive_entry_nlink((*file).entry) > 1 as libc::c_int as libc::c_uint {
        r = isofile_register_hardlink(a, file);
        if r != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Prepare to save the contents of the file.
     */
    if (*iso9660).temp_fd < 0 as libc::c_int {
        (*iso9660).temp_fd = __archive_mktemp(NULL as *const libc::c_char);
        if (*iso9660).temp_fd < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Couldn\'t create temporary file\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    /* Save an offset of current file in temporary file. */
    (*file).content.offset_of_temp =
        (((*((*a).format_data as *mut iso9660)).wbuff_offset
            + (LOGICAL_BLOCK_SIZE * 32 as libc::c_int) as libc::c_long) as libc::c_ulong)
            .wrapping_sub((*((*a).format_data as *mut iso9660)).wbuff_remaining) as int64_t;
    (*file).cur_content = &mut (*file).content;
    r = zisofs_init(a, file);
    if r < ret {
        ret = r
    }
    (*iso9660).bytes_remaining = archive_entry_size((*file).entry) as uint64_t;
    return ret;
}
unsafe extern "C" fn write_to_temp(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut written: ssize_t = 0;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    b = buff as *const libc::c_uchar;
    while s != 0 {
        written = write((*iso9660).temp_fd, b as *const libc::c_void, s);
        if written < 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t write to temporary file\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        s = (s as libc::c_ulong).wrapping_sub(written as libc::c_ulong) as size_t as size_t;
        b = b.offset(written as isize)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wb_write_to_temp(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> libc::c_int {
    let mut xp: *const libc::c_char = buff as *const libc::c_char;
    let mut xs: size_t = s;
    /*
     * If a written data size is big enough to use system-call
     * and there is no waiting data, this calls write_to_temp() in
     * order to reduce a extra memory copy.
     */
    if (*((*a).format_data as *mut iso9660)).wbuff_remaining
        == (LOGICAL_BLOCK_SIZE * 32 as libc::c_int) as libc::c_ulong
        && s > (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
    {
        let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
        xs = s.wrapping_rem(LOGICAL_BLOCK_SIZE as libc::c_ulong);
        (*iso9660).wbuff_offset = ((*iso9660).wbuff_offset as libc::c_ulong)
            .wrapping_add(s.wrapping_sub(xs)) as int64_t
            as int64_t;
        if write_to_temp(a, buff, s.wrapping_sub(xs)) != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        if xs == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        xp = xp.offset(s.wrapping_sub(xs) as isize)
    }
    while xs != 0 {
        let mut size: size_t = xs;
        if size > (*((*a).format_data as *mut iso9660)).wbuff_remaining {
            size = (*((*a).format_data as *mut iso9660)).wbuff_remaining
        }
        memcpy(
            wb_buffptr(a) as *mut libc::c_void,
            xp as *const libc::c_void,
            size,
        );
        if wb_consume(a, size) != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        xs = (xs as libc::c_ulong).wrapping_sub(size) as size_t as size_t;
        xp = xp.offset(size as isize)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wb_write_padding_to_temp(
    mut a: *mut archive_write,
    mut csize: int64_t,
) -> libc::c_int {
    let mut ns: size_t = 0;
    let mut ret: libc::c_int = 0;
    ns = (csize % LOGICAL_BLOCK_SIZE as libc::c_long) as size_t;
    if ns != 0 as libc::c_int as libc::c_ulong {
        ret = write_null(a, (LOGICAL_BLOCK_SIZE as libc::c_ulong).wrapping_sub(ns))
    } else {
        ret = ARCHIVE_OK
    }
    return ret;
}
unsafe extern "C" fn write_iso9660_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut ws: size_t = 0;
    if (*iso9660).temp_fd < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Couldn\'t create temporary file\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int) as ssize_t;
    }
    ws = s;
    if (*iso9660).need_multi_extent != 0
        && ((*(*(*iso9660).cur_file).cur_content).size as libc::c_ulong).wrapping_add(ws)
            as libc::c_ulonglong
            >= (MULTI_EXTENT_SIZE - LOGICAL_BLOCK_SIZE as libc::c_longlong) as libc::c_ulonglong
    {
        let mut con: *mut content = 0 as *mut content;
        let mut ts: size_t = 0;
        ts = (MULTI_EXTENT_SIZE
            - LOGICAL_BLOCK_SIZE as libc::c_longlong
            - (*(*(*iso9660).cur_file).cur_content).size as libc::c_longlong)
            as size_t;
        if (*iso9660).zisofs.detect_magic() != 0 {
            zisofs_detect_magic(a, buff, ts);
        }
        if (*iso9660).zisofs.making() != 0 {
            if zisofs_write_to_temp(a, buff, ts) != ARCHIVE_OK {
                return -(30 as libc::c_int) as ssize_t;
            }
        } else {
            if wb_write_to_temp(a, buff, ts) != ARCHIVE_OK {
                return -(30 as libc::c_int) as ssize_t;
            }
            (*(*(*iso9660).cur_file).cur_content).size =
                ((*(*(*iso9660).cur_file).cur_content).size as libc::c_ulong).wrapping_add(ts)
                    as int64_t as int64_t
        }
        /* Write padding. */
        if wb_write_padding_to_temp(a, (*(*(*iso9660).cur_file).cur_content).size) != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        /* Compute the logical block number. */
        (*(*(*iso9660).cur_file).cur_content).blocks =
            ((*(*(*iso9660).cur_file).cur_content).size + LOGICAL_BLOCK_SIZE as libc::c_long
                - 1 as libc::c_int as libc::c_long
                >> LOGICAL_BLOCK_BITS) as libc::c_int;
        /*
         * Make next extent.
         */
        ws = (ws as libc::c_ulong).wrapping_sub(ts) as size_t as size_t;
        buff = (buff as *const libc::c_uchar).offset(ts as isize) as *const libc::c_void;
        /* Make a content for next extent. */
        con = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<content>() as libc::c_ulong,
        ) as *mut content;
        if con.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate content data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        (*con).offset_of_temp = (((*((*a).format_data as *mut iso9660)).wbuff_offset
            + (LOGICAL_BLOCK_SIZE * 32 as libc::c_int) as libc::c_long)
            as libc::c_ulong)
            .wrapping_sub((*((*a).format_data as *mut iso9660)).wbuff_remaining)
            as int64_t;
        (*(*(*iso9660).cur_file).cur_content).next = con;
        (*(*iso9660).cur_file).cur_content = con;
        (*iso9660).zisofs.block_offset = 0 as libc::c_int as int64_t
    }
    if (*iso9660).zisofs.detect_magic() != 0 {
        zisofs_detect_magic(a, buff, ws);
    }
    if (*iso9660).zisofs.making() != 0 {
        if zisofs_write_to_temp(a, buff, ws) != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
    } else {
        if wb_write_to_temp(a, buff, ws) != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        (*(*(*iso9660).cur_file).cur_content).size =
            ((*(*(*iso9660).cur_file).cur_content).size as libc::c_ulong).wrapping_add(ws)
                as int64_t as int64_t
    }
    return s as ssize_t;
}
unsafe extern "C" fn iso9660_write_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut r: ssize_t = 0;
    if (*iso9660).cur_file.is_null() {
        return 0 as libc::c_int as ssize_t;
    }
    if archive_entry_filetype((*(*iso9660).cur_file).entry) != AE_IFREG as mode_t {
        return 0 as libc::c_int as ssize_t;
    }
    if s > (*iso9660).bytes_remaining {
        s = (*iso9660).bytes_remaining
    }
    if s == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    r = write_iso9660_data(a, buff, s);
    if r > 0 as libc::c_int as libc::c_long {
        (*iso9660).bytes_remaining = ((*iso9660).bytes_remaining as libc::c_ulong)
            .wrapping_sub(r as libc::c_ulong) as uint64_t
            as uint64_t
    }
    return r;
}
unsafe extern "C" fn iso9660_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    if (*iso9660).cur_file.is_null() {
        return 0 as libc::c_int;
    }
    if archive_entry_filetype((*(*iso9660).cur_file).entry) != AE_IFREG as mode_t {
        return 0 as libc::c_int;
    }
    if (*(*iso9660).cur_file).content.size == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    /* If there are unwritten data, write null data instead. */
    while (*iso9660).bytes_remaining > 0 as libc::c_int as libc::c_ulong {
        let mut s: size_t = 0;
        s = if (*iso9660).bytes_remaining > (*a).null_length {
            (*a).null_length
        } else {
            (*iso9660).bytes_remaining
        };
        if write_iso9660_data(a, (*a).nulls as *const libc::c_void, s)
            < 0 as libc::c_int as libc::c_long
        {
            return -(30 as libc::c_int);
        }
        (*iso9660).bytes_remaining =
            ((*iso9660).bytes_remaining as libc::c_ulong).wrapping_sub(s) as uint64_t as uint64_t
    }
    if (*iso9660).zisofs.making() != 0 && zisofs_finish_entry(a) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Write padding. */
    if wb_write_padding_to_temp(a, (*(*(*iso9660).cur_file).cur_content).size) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Compute the logical block number. */
    (*(*(*iso9660).cur_file).cur_content).blocks = ((*(*(*iso9660).cur_file).cur_content).size
        + LOGICAL_BLOCK_SIZE as libc::c_long
        - 1 as libc::c_int as libc::c_long
        >> LOGICAL_BLOCK_BITS) as libc::c_int;
    /* Add the current file to data file list. */
    isofile_add_data_file(iso9660, (*iso9660).cur_file);
    return 0 as libc::c_int;
}
unsafe extern "C" fn iso9660_close(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut ret: libc::c_int = 0;
    let mut blocks: libc::c_int = 0;
    iso9660 = (*a).format_data as *mut iso9660;
    /*
     * Write remaining data out to the temporary file.
     */
    if (*((*a).format_data as *mut iso9660)).wbuff_remaining > 0 as libc::c_int as libc::c_ulong {
        ret = wb_write_out(a);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    /*
     * Preparations...
     */
    time(&mut (*iso9660).birth_time);
    /*
     * Prepare a bootable ISO image.
     */
    if (*iso9660).opt.boot() != 0 {
        /* Find out the boot file entry. */
        ret = isoent_find_out_boot_file(a, (*iso9660).primary.rootent);
        if ret < 0 as libc::c_int {
            return ret;
        }
        /* Reconvert the boot file from zisofs'ed form to
         * plain form. */
        ret = zisofs_rewind_boot_file(a);
        if ret < 0 as libc::c_int {
            return ret;
        }
        /* Write remaining data out to the temporary file. */
        if (*((*a).format_data as *mut iso9660)).wbuff_remaining > 0 as libc::c_int as libc::c_ulong
        {
            ret = wb_write_out(a);
            if ret < 0 as libc::c_int {
                return ret;
            }
        }
        /* Create the boot catalog. */
        ret = isoent_create_boot_catalog(a, (*iso9660).primary.rootent);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    /*
     * Prepare joliet extensions.
     */
    if (*iso9660).opt.joliet() != 0 {
        /* Make a new tree for joliet. */
        ret = isoent_clone_tree(
            a,
            &mut (*iso9660).joliet.rootent,
            (*iso9660).primary.rootent,
        );
        if ret < 0 as libc::c_int {
            return ret;
        }
        /* Make sure we have UTF-16BE converters.
         * if there is no file entry, converters are still
         * uninitialized. */
        if (*iso9660).sconv_to_utf16be.is_null() {
            (*iso9660).sconv_to_utf16be = archive_string_conversion_to_charset(
                &mut (*a).archive,
                b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*iso9660).sconv_to_utf16be.is_null() {
                /* Couldn't allocate memory */
                return -(30 as libc::c_int);
            }
            (*iso9660).sconv_from_utf16be = archive_string_conversion_from_charset(
                &mut (*a).archive,
                b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*iso9660).sconv_from_utf16be.is_null() {
                /* Couldn't allocate memory */
                return -(30 as libc::c_int);
            }
        }
    }
    /*
     * Make Path Tables.
     */
    ret = isoent_make_path_table(a);
    if ret < 0 as libc::c_int {
        return ret;
    }
    /*
     * Calculate a total volume size and setup all locations of
     * contents of an iso9660 image.
     */
    blocks = SYSTEM_AREA_BLOCK
        + PRIMARY_VOLUME_DESCRIPTOR_BLOCK
        + VOLUME_DESCRIPTOR_SET_TERMINATOR_BLOCK
        + NON_ISO_FILE_SYSTEM_INFORMATION_BLOCK;
    if (*iso9660).opt.boot() != 0 {
        blocks += BOOT_RECORD_DESCRIPTOR_BLOCK
    }
    if (*iso9660).opt.joliet() != 0 {
        blocks += SUPPLEMENTARY_VOLUME_DESCRIPTOR_BLOCK
    }
    if (*iso9660).opt.iso_level() as libc::c_int == 4 as libc::c_int {
        blocks += SUPPLEMENTARY_VOLUME_DESCRIPTOR_BLOCK
    }
    /* Setup the locations of Path Table. */
    (*iso9660).primary.location_type_L_path_table = blocks;
    blocks += (*iso9660).primary.path_table_block;
    (*iso9660).primary.location_type_M_path_table = blocks;
    blocks += (*iso9660).primary.path_table_block;
    if (*iso9660).opt.joliet() != 0 {
        (*iso9660).joliet.location_type_L_path_table = blocks;
        blocks += (*iso9660).joliet.path_table_block;
        (*iso9660).joliet.location_type_M_path_table = blocks;
        blocks += (*iso9660).joliet.path_table_block
    }
    /* Setup the locations of directories. */
    isoent_setup_directory_location(iso9660, blocks, &mut (*iso9660).primary);
    blocks += (*iso9660).primary.total_dir_block;
    if (*iso9660).opt.joliet() != 0 {
        isoent_setup_directory_location(iso9660, blocks, &mut (*iso9660).joliet);
        blocks += (*iso9660).joliet.total_dir_block
    }
    if (*iso9660).opt.rr() != 0 {
        (*iso9660).location_rrip_er = blocks;
        blocks += RRIP_ER_BLOCK
    }
    /* Setup the locations of all file contents. */
    isoent_setup_file_location(iso9660, blocks);
    blocks += (*iso9660).total_file_block;
    if (*iso9660).opt.boot() as libc::c_int != 0
        && (*iso9660).opt.boot_info_table() as libc::c_int != 0
    {
        ret = setup_boot_information(a);
        if ret < 0 as libc::c_int {
            return ret;
        }
    }
    /* Now we have a total volume size. */
    (*iso9660).volume_space_size = blocks;
    if (*iso9660).opt.pad() != 0 {
        (*iso9660).volume_space_size += PADDING_BLOCK
    }
    (*iso9660).volume_sequence_number = 1 as libc::c_int;
    /*
     * Write an ISO 9660 image.
     */
    /* Switch to start using wbuff as file buffer. */
    (*iso9660).wbuff_remaining = (LOGICAL_BLOCK_SIZE * 32 as libc::c_int) as size_t;
    (*iso9660).wbuff_type = WB_TO_STREAM;
    (*iso9660).wbuff_offset = 0 as libc::c_int as int64_t;
    (*iso9660).wbuff_written = 0 as libc::c_int as int64_t;
    (*iso9660).wbuff_tail = 0 as libc::c_int as int64_t;
    /* Write The System Area */
    ret = write_null(a, (SYSTEM_AREA_BLOCK * LOGICAL_BLOCK_SIZE) as size_t);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Write Primary Volume Descriptor */
    ret = write_VD(a, &mut (*iso9660).primary);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    if (*iso9660).opt.boot() != 0 {
        /* Write Boot Record Volume Descriptor */
        ret = write_VD_boot_record(a);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    if (*iso9660).opt.iso_level() as libc::c_int == 4 as libc::c_int {
        /* Write Enhanced Volume Descriptor */
        (*iso9660).primary.vdd_type = VDD_ENHANCED;
        ret = write_VD(a, &mut (*iso9660).primary);
        (*iso9660).primary.vdd_type = VDD_PRIMARY;
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    if (*iso9660).opt.joliet() != 0 {
        ret = write_VD(a, &mut (*iso9660).joliet);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    /* Write Volume Descriptor Set Terminator */
    ret = write_VD_terminator(a);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Write Non-ISO File System Information */
    ret = write_information_block(a);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Write Type L Path Table */
    ret = write_path_table(a, 0 as libc::c_int, &mut (*iso9660).primary);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Write Type M Path Table */
    ret = write_path_table(a, 1 as libc::c_int, &mut (*iso9660).primary);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    if (*iso9660).opt.joliet() != 0 {
        /* Write Type L Path Table */
        ret = write_path_table(a, 0 as libc::c_int, &mut (*iso9660).joliet);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        /* Write Type M Path Table */
        ret = write_path_table(a, 1 as libc::c_int, &mut (*iso9660).joliet);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    /* Write Directory Descriptors */
    ret = write_directory_descriptors(a, &mut (*iso9660).primary);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    if (*iso9660).opt.joliet() != 0 {
        ret = write_directory_descriptors(a, &mut (*iso9660).joliet);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    if (*iso9660).opt.rr() != 0 {
        /* Write Rockridge ER(Extensions Reference) */
        ret = write_rr_ER(a);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    /* Write File Descriptors */
    ret = write_file_descriptors(a);
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Write Padding  */
    if (*iso9660).opt.pad() != 0 {
        ret = write_null(a, (PADDING_BLOCK * LOGICAL_BLOCK_SIZE) as size_t);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    if !(*iso9660).directories_too_deep.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"%s: Directories too deep.\x00" as *const u8 as *const libc::c_char,
            archive_entry_pathname((*(*(*iso9660).directories_too_deep).file).entry),
        );
        return -(20 as libc::c_int);
    }
    /* Write remaining data out. */
    ret = wb_write_out(a);
    return ret;
}
unsafe extern "C" fn iso9660_free(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    iso9660 = (*a).format_data as *mut iso9660;
    /* Close the temporary file. */
    if (*iso9660).temp_fd >= 0 as libc::c_int {
        close((*iso9660).temp_fd);
    }
    /* Free some stuff for zisofs operations. */
    ret = zisofs_free(a);
    /* Remove directory entries in tree which includes file entries. */
    isoent_free_all((*iso9660).primary.rootent);
    i = 0 as libc::c_int;
    while i < (*iso9660).primary.max_depth {
        free((*(*iso9660).primary.pathtbl.offset(i as isize)).sorted as *mut libc::c_void);
        i += 1
    }
    free((*iso9660).primary.pathtbl as *mut libc::c_void);
    if (*iso9660).opt.joliet() != 0 {
        isoent_free_all((*iso9660).joliet.rootent);
        i = 0 as libc::c_int;
        while i < (*iso9660).joliet.max_depth {
            free((*(*iso9660).joliet.pathtbl.offset(i as isize)).sorted as *mut libc::c_void);
            i += 1
        }
        free((*iso9660).joliet.pathtbl as *mut libc::c_void);
    }
    /* Remove isofile entries. */
    isofile_free_all_entries(iso9660);
    isofile_free_hardlinks(iso9660);
    archive_string_free(&mut (*iso9660).cur_dirstr);
    archive_string_free(&mut (*iso9660).volume_identifier);
    archive_string_free(&mut (*iso9660).publisher_identifier);
    archive_string_free(&mut (*iso9660).data_preparer_identifier);
    archive_string_free(&mut (*iso9660).application_identifier);
    archive_string_free(&mut (*iso9660).copyright_file_identifier);
    archive_string_free(&mut (*iso9660).abstract_file_identifier);
    archive_string_free(&mut (*iso9660).bibliographic_file_identifier);
    archive_string_free(&mut (*iso9660).el_torito.catalog_filename);
    archive_string_free(&mut (*iso9660).el_torito.boot_filename);
    archive_string_free(&mut (*iso9660).el_torito.id);
    archive_string_free(&mut (*iso9660).utf16be);
    archive_string_free(&mut (*iso9660).mbs);
    free(iso9660 as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return ret;
}
/*
 * Get the System Identifier
 */
unsafe extern "C" fn get_system_identitier(mut system_id: *mut libc::c_char, mut size: size_t) {
    let mut u: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    uname(&mut u);
    strncpy(
        system_id,
        u.sysname.as_mut_ptr(),
        size.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    *system_id.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
}
unsafe extern "C" fn set_str(
    mut p: *mut libc::c_uchar,
    mut s: *const libc::c_char,
    mut l: size_t,
    mut f: libc::c_char,
    mut map: *const libc::c_char,
) {
    let mut c: libc::c_uchar = 0;
    if s.is_null() {
        s = b"\x00" as *const u8 as *const libc::c_char
    }
    loop {
        let fresh0 = s;
        s = s.offset(1);
        c = *fresh0 as libc::c_uchar;
        if !(c as libc::c_int != 0 as libc::c_int && l > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if c as libc::c_int >= 0x80 as libc::c_int
            || *map.offset(c as isize) as libc::c_int == 0 as libc::c_int
        {
            /* illegal character */
            if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
                /* convert c from a-z to A-Z */
                c = (c as libc::c_int - 0x20 as libc::c_int) as libc::c_uchar
            } else {
                c = 0x5f as libc::c_int as libc::c_uchar
            }
        }
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = c;
        l = l.wrapping_sub(1)
    }
    /* If l isn't zero, fill p buffer by the character
     * which indicated by f. */
    if l > 0 as libc::c_int as libc::c_ulong {
        memset(p as *mut libc::c_void, f as libc::c_int, l);
    };
}
#[inline]
unsafe extern "C" fn joliet_allowed_char(
    mut high: libc::c_uchar,
    mut low: libc::c_uchar,
) -> libc::c_int {
    let mut utf16: libc::c_int = (high as libc::c_int) << 8 as libc::c_int | low as libc::c_int;
    if utf16 <= 0x1f as libc::c_int {
        return 0 as libc::c_int;
    }
    's_27: {
        let mut current_block_2: u64;
        match utf16 {
            42 => {
                /* '*' */
                current_block_2 = 411142475779952492;
            }
            47 => {
                current_block_2 = 411142475779952492;
            }
            58 => {
                current_block_2 = 7382459031201955754;
            }
            59 => {
                current_block_2 = 8427182090417329361;
            }
            63 => {
                current_block_2 = 5914609616168174476;
            }
            92 => {
                current_block_2 = 2961611692917757673;
            }
            _ => {
                break 's_27;
            }
        }
        match current_block_2 {
            411142475779952492 =>
            /* '/' */
            {
                current_block_2 = 7382459031201955754;
            }
            _ => {}
        }
        match current_block_2 {
            7382459031201955754 =>
            /* ':' */
            {
                current_block_2 = 8427182090417329361;
            }
            _ => {}
        }
        match current_block_2 {
            8427182090417329361 =>
            /* ';' */
            {
                current_block_2 = 5914609616168174476;
            }
            _ => {}
        }
        match current_block_2 {
            5914609616168174476 =>
                /* '?' */
                {}
            _ => {}
        }
        /* '\' */
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn set_str_utf16be(
    mut a: *mut archive_write,
    mut p: *mut libc::c_uchar,
    mut s: *const libc::c_char,
    mut l: size_t,
    mut uf: uint16_t,
    mut vdc: vdc,
) -> libc::c_int {
    let mut size: size_t = 0;
    let mut i: size_t = 0;
    let mut onepad: libc::c_int = 0;
    if s.is_null() {
        s = b"\x00" as *const u8 as *const libc::c_char
    }
    if l & 0x1 as libc::c_int as libc::c_ulong != 0 {
        onepad = 1 as libc::c_int;
        l &= !(1 as libc::c_int) as libc::c_ulong
    } else {
        onepad = 0 as libc::c_int
    }
    if vdc as libc::c_uint == VDC_UCS2 as libc::c_int as libc::c_uint {
        let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
        if archive_strncpy_l(
            &mut (*iso9660).utf16be,
            s as *const libc::c_void,
            strlen(s),
            (*iso9660).sconv_to_utf16be,
        ) != 0 as libc::c_int
            && errno == ENOMEM
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for UTF-16BE\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        size = (*iso9660).utf16be.length;
        if size > l {
            size = l
        }
        memcpy(
            p as *mut libc::c_void,
            (*iso9660).utf16be.s as *const libc::c_void,
            size,
        );
    } else {
        let mut u16: *const uint16_t = s as *const uint16_t;
        size = 0 as libc::c_int as size_t;
        loop {
            let fresh2 = u16;
            u16 = u16.offset(1);
            if !(*fresh2 != 0) {
                break;
            }
            size = (size as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        if size > l {
            size = l
        }
        memcpy(p as *mut libc::c_void, s as *const libc::c_void, size);
    }
    i = 0 as libc::c_int as size_t;
    while i < size {
        if joliet_allowed_char(
            *p.offset(0 as libc::c_int as isize),
            *p.offset(1 as libc::c_int as isize),
        ) == 0
        {
            archive_be16enc(p as *mut libc::c_void, 0x5f as libc::c_int as uint16_t);
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        p = p.offset(2 as libc::c_int as isize)
        /* Not allowed. */
        /* '_' */
    }
    l = (l as libc::c_ulong).wrapping_sub(size) as size_t as size_t;
    while l > 0 as libc::c_int as libc::c_ulong {
        archive_be16enc(p as *mut libc::c_void, uf);
        p = p.offset(2 as libc::c_int as isize);
        l = (l as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t as size_t
    }
    if onepad != 0 {
        *p = 0 as libc::c_int as libc::c_uchar
    }
    return 0 as libc::c_int;
}
static mut a_characters_map: [libc::c_char; 128] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
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
static mut a1_characters_map: [libc::c_char; 128] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
static mut d_characters_map: [libc::c_char; 128] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
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
static mut d1_characters_map: [libc::c_char; 128] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn set_str_a_characters_bp(
    mut a: *mut archive_write,
    mut bp: *mut libc::c_uchar,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut s: *const libc::c_char,
    mut vdc: vdc,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    match vdc as libc::c_uint {
        0 => {
            set_str(
                bp.offset(from as isize),
                s,
                (to - from + 1 as libc::c_int) as size_t,
                0x20 as libc::c_int as libc::c_char,
                a_characters_map.as_ptr(),
            );
            r = ARCHIVE_OK
        }
        1 => {
            set_str(
                bp.offset(from as isize),
                s,
                (to - from + 1 as libc::c_int) as size_t,
                0x20 as libc::c_int as libc::c_char,
                a1_characters_map.as_ptr(),
            );
            r = ARCHIVE_OK
        }
        2 | 3 => {
            r = set_str_utf16be(
                a,
                bp.offset(from as isize),
                s,
                (to - from + 1 as libc::c_int) as size_t,
                0x20 as libc::c_int as uint16_t,
                vdc,
            )
        }
        _ => r = ARCHIVE_FATAL,
    }
    return r;
}
unsafe extern "C" fn set_str_d_characters_bp(
    mut a: *mut archive_write,
    mut bp: *mut libc::c_uchar,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut s: *const libc::c_char,
    mut vdc: vdc,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    match vdc as libc::c_uint {
        0 => {
            set_str(
                bp.offset(from as isize),
                s,
                (to - from + 1 as libc::c_int) as size_t,
                0x20 as libc::c_int as libc::c_char,
                d_characters_map.as_ptr(),
            );
            r = ARCHIVE_OK
        }
        1 => {
            set_str(
                bp.offset(from as isize),
                s,
                (to - from + 1 as libc::c_int) as size_t,
                0x20 as libc::c_int as libc::c_char,
                d1_characters_map.as_ptr(),
            );
            r = ARCHIVE_OK
        }
        2 | 3 => {
            r = set_str_utf16be(
                a,
                bp.offset(from as isize),
                s,
                (to - from + 1 as libc::c_int) as size_t,
                0x20 as libc::c_int as uint16_t,
                vdc,
            )
        }
        _ => r = ARCHIVE_FATAL,
    }
    return r;
}
unsafe extern "C" fn set_VD_bp(
    mut bp: *mut libc::c_uchar,
    mut type_0: VD_type,
    mut ver: libc::c_uchar,
) {
    /* Volume Descriptor Type */
    *bp.offset(1 as libc::c_int as isize) = type_0 as libc::c_uchar;
    /* Standard Identifier */
    memcpy(
        bp.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        b"CD001\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    );
    /* Volume Descriptor Version */
    *bp.offset(7 as libc::c_int as isize) = ver;
}
#[inline]
unsafe extern "C" fn set_unused_field_bp(
    mut bp: *mut libc::c_uchar,
    mut from: libc::c_int,
    mut to: libc::c_int,
) {
    memset(
        bp.offset(from as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (to - from + 1 as libc::c_int) as libc::c_ulong,
    );
}
/*
 * 8-bit unsigned numerical values.
 * ISO9660 Standard 7.1.1
 */
#[inline]
unsafe extern "C" fn set_num_711(mut p: *mut libc::c_uchar, mut value: libc::c_uchar) {
    *p = value;
}
/*
 * 8-bit signed numerical values.
 * ISO9660 Standard 7.1.2
 */
#[inline]
unsafe extern "C" fn set_num_712(mut p: *mut libc::c_uchar, mut value: libc::c_char) {
    *(p as *mut libc::c_char) = value;
}
/*
 * Least significant byte first.
 * ISO9660 Standard 7.2.1
 */
#[inline]
unsafe extern "C" fn set_num_721(mut p: *mut libc::c_uchar, mut value: uint16_t) {
    archive_le16enc(p as *mut libc::c_void, value);
}
/*
 * Most significant byte first.
 * ISO9660 Standard 7.2.2
 */
#[inline]
unsafe extern "C" fn set_num_722(mut p: *mut libc::c_uchar, mut value: uint16_t) {
    archive_be16enc(p as *mut libc::c_void, value);
}
/*
 * Both-byte orders.
 * ISO9660 Standard 7.2.3
 */
unsafe extern "C" fn set_num_723(mut p: *mut libc::c_uchar, mut value: uint16_t) {
    archive_le16enc(p as *mut libc::c_void, value);
    archive_be16enc(
        p.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        value,
    );
}
/*
 * Least significant byte first.
 * ISO9660 Standard 7.3.1
 */
#[inline]
unsafe extern "C" fn set_num_731(mut p: *mut libc::c_uchar, mut value: uint32_t) {
    archive_le32enc(p as *mut libc::c_void, value);
}
/*
 * Most significant byte first.
 * ISO9660 Standard 7.3.2
 */
#[inline]
unsafe extern "C" fn set_num_732(mut p: *mut libc::c_uchar, mut value: uint32_t) {
    archive_be32enc(p as *mut libc::c_void, value);
}
/*
 * Both-byte orders.
 * ISO9660 Standard 7.3.3
 */
#[inline]
unsafe extern "C" fn set_num_733(mut p: *mut libc::c_uchar, mut value: uint32_t) {
    archive_le32enc(p as *mut libc::c_void, value);
    archive_be32enc(
        p.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        value,
    );
}
unsafe extern "C" fn set_digit(mut p: *mut libc::c_uchar, mut s: size_t, mut value: libc::c_int) {
    loop {
        let fresh3 = s;
        s = s.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        *p.offset(s as isize) = ('0' as i32 + value % 10 as libc::c_int) as libc::c_uchar;
        value /= 10 as libc::c_int
    }
}
unsafe extern "C" fn get_tmfromtime(mut tm: *mut tm, mut t: *mut time_t) {
    tzset();
    localtime_r(t, tm);
}
/*
 * Date and Time Format.
 * ISO9660 Standard 8.4.26.1
 */
unsafe extern "C" fn set_date_time(mut p: *mut libc::c_uchar, mut t: time_t) {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    get_tmfromtime(&mut tm, &mut t);
    set_digit(
        p,
        4 as libc::c_int as size_t,
        tm.tm_year + 1900 as libc::c_int,
    );
    set_digit(
        p.offset(4 as libc::c_int as isize),
        2 as libc::c_int as size_t,
        tm.tm_mon + 1 as libc::c_int,
    );
    set_digit(
        p.offset(6 as libc::c_int as isize),
        2 as libc::c_int as size_t,
        tm.tm_mday,
    );
    set_digit(
        p.offset(8 as libc::c_int as isize),
        2 as libc::c_int as size_t,
        tm.tm_hour,
    );
    set_digit(
        p.offset(10 as libc::c_int as isize),
        2 as libc::c_int as size_t,
        tm.tm_min,
    );
    set_digit(
        p.offset(12 as libc::c_int as isize),
        2 as libc::c_int as size_t,
        tm.tm_sec,
    );
    set_digit(
        p.offset(14 as libc::c_int as isize),
        2 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    set_num_712(
        p.offset(16 as libc::c_int as isize),
        (tm.tm_gmtoff / (60 as libc::c_int * 15 as libc::c_int) as libc::c_long) as libc::c_char,
    );
}
unsafe extern "C" fn set_date_time_null(mut p: *mut libc::c_uchar) {
    memset(
        p as *mut libc::c_void,
        '0' as i32,
        16 as libc::c_int as libc::c_ulong,
    );
    *p.offset(16 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn set_time_915(mut p: *mut libc::c_uchar, mut t: time_t) {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    get_tmfromtime(&mut tm, &mut t);
    set_num_711(
        p.offset(0 as libc::c_int as isize),
        tm.tm_year as libc::c_uchar,
    );
    set_num_711(
        p.offset(1 as libc::c_int as isize),
        (tm.tm_mon + 1 as libc::c_int) as libc::c_uchar,
    );
    set_num_711(
        p.offset(2 as libc::c_int as isize),
        tm.tm_mday as libc::c_uchar,
    );
    set_num_711(
        p.offset(3 as libc::c_int as isize),
        tm.tm_hour as libc::c_uchar,
    );
    set_num_711(
        p.offset(4 as libc::c_int as isize),
        tm.tm_min as libc::c_uchar,
    );
    set_num_711(
        p.offset(5 as libc::c_int as isize),
        tm.tm_sec as libc::c_uchar,
    );
    set_num_712(
        p.offset(6 as libc::c_int as isize),
        (tm.tm_gmtoff / (60 as libc::c_int * 15 as libc::c_int) as libc::c_long) as libc::c_char,
    );
}
/*
 * Write SUSP "CE" System Use Entry.
 */
unsafe extern "C" fn set_SUSP_CE(
    mut p: *mut libc::c_uchar,
    mut location: libc::c_int,
    mut offset: libc::c_int,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut bp: *mut libc::c_uchar = p.offset(-(1 as libc::c_int as isize));
    /*  Extend the System Use Area
     *   "CE" Format:
     *               len  ver
     *    +----+----+----+----+-----------+-----------+
     *    | 'C'| 'E'| 1C | 01 | LOCATION1 | LOCATION2 |
     *    +----+----+----+----+-----------+-----------+
     *    0    1    2    3    4          12          20
     *    +-----------+
     *    | LOCATION3 |
     *    +-----------+
     *   20          28
     *   LOCATION1 : Location of Continuation of System Use Area.
     *   LOCATION2 : Offset to Start of Continuation.
     *   LOCATION3 : Length of the Continuation.
     */
    *bp.offset(1 as libc::c_int as isize) = 'C' as i32 as libc::c_uchar; /* length	*/
    *bp.offset(2 as libc::c_int as isize) = 'E' as i32 as libc::c_uchar; /* version	*/
    *bp.offset(3 as libc::c_int as isize) = RR_CE_SIZE as libc::c_uchar;
    *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    set_num_733(bp.offset(5 as libc::c_int as isize), location as uint32_t);
    set_num_733(bp.offset(13 as libc::c_int as isize), offset as uint32_t);
    set_num_733(bp.offset(21 as libc::c_int as isize), size as uint32_t);
    return 28 as libc::c_int;
}
/*
 * The functions, which names are beginning with extra_, are used to
 * control extra records.
 * The maximum size of a Directory Record is 254. When a filename is
 * very long, all of RRIP data of a file won't stored to the Directory
 * Record and so remaining RRIP data store to an extra record instead.
 */
unsafe extern "C" fn extra_open_record(
    mut bp: *mut libc::c_uchar,
    mut dr_len: libc::c_int,
    mut isoent: *mut isoent,
    mut ctl: *mut ctl_extr_rec,
) -> *mut libc::c_uchar {
    (*ctl).bp = bp;
    if !bp.is_null() {
        bp = bp.offset(dr_len as isize)
    }
    (*ctl).use_extr = 0 as libc::c_int;
    (*ctl).isoent = isoent;
    (*ctl).ce_ptr = NULL as *mut libc::c_uchar;
    (*ctl).dr_len = dr_len;
    (*ctl).cur_len = (*ctl).dr_len;
    (*ctl).limit = DR_LIMIT;
    return bp;
}
unsafe extern "C" fn extra_close_record(mut ctl: *mut ctl_extr_rec, mut ce_size: libc::c_int) {
    let mut padding: libc::c_int = 0 as libc::c_int;
    if ce_size > 0 as libc::c_int {
        extra_tell_used_size(ctl, ce_size);
    }
    /* Padding. */
    if (*ctl).cur_len & 0x1 as libc::c_int != 0 {
        (*ctl).cur_len += 1; /* save cur_len */
        if !(*ctl).bp.is_null() {
            *(*ctl).bp.offset((*ctl).cur_len as isize) = 0 as libc::c_int as libc::c_uchar
        }
        padding = 1 as libc::c_int
    }
    if (*ctl).use_extr != 0 {
        if !(*ctl).ce_ptr.is_null() {
            set_SUSP_CE(
                (*ctl).ce_ptr,
                (*ctl).extr_loc,
                (*ctl).extr_off,
                (*ctl).cur_len - padding,
            );
        }
    } else {
        (*ctl).dr_len = (*ctl).cur_len
    };
}
unsafe extern "C" fn extra_next_record(
    mut ctl: *mut ctl_extr_rec,
    mut length: libc::c_int,
) -> *mut libc::c_uchar {
    let mut cur_len: libc::c_int = (*ctl).cur_len;
    /* Close the current extra record or Directory Record. */
    extra_close_record(ctl, RR_CE_SIZE);
    /* Get a next extra record. */
    (*ctl).use_extr = 1 as libc::c_int;
    if !(*ctl).bp.is_null() {
        /* Storing data into an extra record. */
        let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        /* Save the pointer where a CE extension will be
         * stored to. */
        (*ctl).ce_ptr =
            &mut *(*ctl).bp.offset((cur_len + 1 as libc::c_int) as isize) as *mut libc::c_uchar;
        p = extra_get_record(
            (*ctl).isoent,
            &mut (*ctl).limit,
            &mut (*ctl).extr_off,
            &mut (*ctl).extr_loc,
        );
        (*ctl).bp = p.offset(-(1 as libc::c_int as isize))
    } else {
        /* Calculating the size of an extra record. */
        extra_get_record(
            (*ctl).isoent,
            &mut (*ctl).limit,
            NULL as *mut libc::c_int,
            NULL as *mut libc::c_int,
        );
    }
    (*ctl).cur_len = 0 as libc::c_int;
    /* Check if an extra record is almost full.
     * If so, get a next one. */
    if (*ctl).limit - (*ctl).cur_len < length {
        extra_next_record(ctl, length);
    }
    return (*ctl).bp;
}
#[inline]
unsafe extern "C" fn extra_last_record(mut isoent: *mut isoent) -> *mut extr_rec {
    if (*isoent).extr_rec_list.first.is_null() {
        return 0 as *mut extr_rec;
    }
    return ((*isoent).extr_rec_list.last as *mut libc::c_char)
        .offset(-(2056 as libc::c_ulong as isize)) as *mut libc::c_void
        as *mut extr_rec;
}
unsafe extern "C" fn extra_get_record(
    mut isoent: *mut isoent,
    mut space: *mut libc::c_int,
    mut off: *mut libc::c_int,
    mut loc: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut rec: *mut extr_rec = 0 as *mut extr_rec;
    isoent = (*isoent).parent;
    if !off.is_null() {
        /* Storing data into an extra record. */
        rec = (*isoent).extr_rec_list.current;
        if DR_SAFETY > LOGICAL_BLOCK_SIZE - (*rec).offset {
            rec = (*rec).next
        }
    } else {
        /* Calculating the size of an extra record. */
        rec = extra_last_record(isoent);
        if rec.is_null() || DR_SAFETY > LOGICAL_BLOCK_SIZE - (*rec).offset {
            rec = malloc(::std::mem::size_of::<extr_rec>() as libc::c_ulong) as *mut extr_rec;
            if rec.is_null() {
                return 0 as *mut libc::c_uchar;
            }
            (*rec).location = 0 as libc::c_int;
            (*rec).offset = 0 as libc::c_int;
            /* Insert `rec` into the tail of isoent->extr_rec_list */
            (*rec).next = NULL as *mut extr_rec;
            /*
             * Note: testing isoent->extr_rec_list.last == NULL
             * here is really unneeded since it has been already
             * initialized at isoent_new function but Clang Static
             * Analyzer claims that it is dereference of null
             * pointer.
             */
            if (*isoent).extr_rec_list.last.is_null() {
                (*isoent).extr_rec_list.last = &mut (*isoent).extr_rec_list.first
            } /* Keep padding space. */
            *(*isoent).extr_rec_list.last = rec;
            (*isoent).extr_rec_list.last = &mut (*rec).next
        }
    }
    *space = LOGICAL_BLOCK_SIZE - (*rec).offset - DR_SAFETY;
    if *space & 0x1 as libc::c_int != 0 {
        *space -= 1 as libc::c_int
    }
    if !off.is_null() {
        *off = (*rec).offset
    }
    if !loc.is_null() {
        *loc = (*rec).location
    }
    (*isoent).extr_rec_list.current = rec;
    return &mut *(*rec).buf.as_mut_ptr().offset((*rec).offset as isize) as *mut libc::c_uchar;
}
unsafe extern "C" fn extra_tell_used_size(mut ctl: *mut ctl_extr_rec, mut size: libc::c_int) {
    let mut isoent: *mut isoent = 0 as *mut isoent;
    let mut rec: *mut extr_rec = 0 as *mut extr_rec;
    if (*ctl).use_extr != 0 {
        isoent = (*(*ctl).isoent).parent;
        rec = (*isoent).extr_rec_list.current;
        if !rec.is_null() {
            (*rec).offset += size
        }
    }
    (*ctl).cur_len += size;
}
unsafe extern "C" fn extra_setup_location(
    mut isoent: *mut isoent,
    mut location: libc::c_int,
) -> libc::c_int {
    let mut rec: *mut extr_rec = 0 as *mut extr_rec;
    let mut cnt: libc::c_int = 0;
    cnt = 0 as libc::c_int;
    rec = (*isoent).extr_rec_list.first;
    (*isoent).extr_rec_list.current = rec;
    while !rec.is_null() {
        cnt += 1;
        let fresh4 = location;
        location = location + 1;
        (*rec).location = fresh4;
        (*rec).offset = 0 as libc::c_int;
        rec = (*rec).next
    }
    return cnt;
}
/*
 * Create the RRIP entries.
 */
unsafe extern "C" fn set_directory_record_rr(
    mut bp: *mut libc::c_uchar,
    mut dr_len: libc::c_int,
    mut isoent: *mut isoent,
    mut iso9660: *mut iso9660,
    mut t: dir_rec_type,
) -> libc::c_int {
    /* Flags(BP 5) of the Rockridge "RR" System Use Field */
    let mut rr_flag: libc::c_uchar = 0;
    let mut length: libc::c_int = 0;
    let mut ctl: ctl_extr_rec = ctl_extr_rec {
        use_extr: 0,
        bp: 0 as *mut libc::c_uchar,
        isoent: 0 as *mut isoent,
        ce_ptr: 0 as *mut libc::c_uchar,
        cur_len: 0,
        dr_len: 0,
        limit: 0,
        extr_off: 0,
        extr_loc: 0,
    };
    let mut rr_parent: *mut isoent = 0 as *mut isoent;
    let mut pxent: *mut isoent = 0 as *mut isoent;
    let mut file: *mut isofile = 0 as *mut isofile;
    bp = extra_open_record(bp, dr_len, isoent, &mut ctl);
    if t as libc::c_uint == DIR_REC_PARENT as libc::c_int as libc::c_uint {
        rr_parent = (*isoent).rr_parent;
        pxent = (*isoent).parent;
        if !rr_parent.is_null() {
            isoent = rr_parent
        } else {
            isoent = (*isoent).parent
        }
    } else {
        rr_parent = NULL as *mut isoent;
        pxent = isoent
    }
    file = (*isoent).file;
    if t as libc::c_uint != DIR_REC_NORMAL as libc::c_int as libc::c_uint {
        rr_flag = (RR_USE_PX | RR_USE_TF) as libc::c_uchar;
        if !rr_parent.is_null() {
            rr_flag = (rr_flag as libc::c_int | RR_USE_PL) as libc::c_uchar
        }
    } else {
        rr_flag = (RR_USE_PX | RR_USE_NM | RR_USE_TF) as libc::c_uchar;
        if archive_entry_filetype((*file).entry) == AE_IFLNK as mode_t {
            rr_flag = (rr_flag as libc::c_int | RR_USE_SL) as libc::c_uchar
        }
        if !(*isoent).rr_parent.is_null() {
            rr_flag = (rr_flag as libc::c_int | RR_USE_RE) as libc::c_uchar
        }
        if !(*isoent).rr_child.is_null() {
            rr_flag = (rr_flag as libc::c_int | RR_USE_CL) as libc::c_uchar
        }
        if archive_entry_filetype((*file).entry) == AE_IFCHR as mode_t
            || archive_entry_filetype((*file).entry) == AE_IFBLK as mode_t
        {
            rr_flag = (rr_flag as libc::c_int | RR_USE_PN) as libc::c_uchar
        }
    }
    /* Write "SP" System Use Entry. */
    if t as libc::c_uint == DIR_REC_SELF as libc::c_int as libc::c_uint
        && isoent == (*isoent).parent
    {
        length = 7 as libc::c_int; /* version	*/
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'S' as i32 as libc::c_uchar; /* Check Byte	*/
            *bp.offset(2 as libc::c_int as isize) = 'P' as i32 as libc::c_uchar; /* Check Byte	*/
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            *bp.offset(5 as libc::c_int as isize) = 0xbe as libc::c_int as libc::c_uchar;
            *bp.offset(6 as libc::c_int as isize) = 0xef as libc::c_int as libc::c_uchar;
            *bp.offset(7 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "RR" System Use Entry. */
    length = 5 as libc::c_int; /* version */
    if ctl.limit - ctl.cur_len < length {
        bp = extra_next_record(&mut ctl, length)
    }
    if !bp.is_null() {
        *bp.offset(1 as libc::c_int as isize) = 'R' as i32 as libc::c_uchar;
        *bp.offset(2 as libc::c_int as isize) = 'R' as i32 as libc::c_uchar;
        *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
        *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
        *bp.offset(5 as libc::c_int as isize) = rr_flag;
        bp = bp.offset(length as isize)
    }
    extra_tell_used_size(&mut ctl, length);
    /* Write "NM" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_NM != 0 {
        /*
         *   "NM" Format:
         *     e.g. a basename is 'foo'
         *               len  ver  flg
         *    +----+----+----+----+----+----+----+----+
         *    | 'N'| 'M'| 08 | 01 | 00 | 'f'| 'o'| 'o'|
         *    +----+----+----+----+----+----+----+----+
         *    <----------------- len ----------------->
         */
        let mut nmlen: size_t = (*file).basename.length;
        let mut nm: *const libc::c_char = (*file).basename.s;
        let mut nmmax: size_t = 0;
        if ctl.limit - ctl.cur_len < 6 as libc::c_int {
            bp = extra_next_record(&mut ctl, 6 as libc::c_int)
        }
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'N' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'M' as i32 as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar
            /* version	*/
        } /* Alternate Name continues
           * in next "NM" field */
        nmmax = (ctl.limit - ctl.cur_len) as size_t;
        if nmmax > 0xff as libc::c_int as libc::c_ulong {
            nmmax = 0xff as libc::c_int as size_t
        }
        while nmlen.wrapping_add(5 as libc::c_int as libc::c_ulong) > nmmax {
            length = nmmax as libc::c_int;
            if !bp.is_null() {
                *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
                *bp.offset(5 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
                memcpy(
                    bp.offset(6 as libc::c_int as isize) as *mut libc::c_void,
                    nm as *const libc::c_void,
                    (length - 5 as libc::c_int) as libc::c_ulong,
                );
                bp = bp.offset(length as isize)
            }
            nmlen = (nmlen as libc::c_ulong)
                .wrapping_sub((length - 5 as libc::c_int) as libc::c_ulong)
                as size_t as size_t;
            nm = nm.offset((length - 5 as libc::c_int) as isize);
            extra_tell_used_size(&mut ctl, length);
            if ctl.limit - ctl.cur_len < 6 as libc::c_int {
                bp = extra_next_record(&mut ctl, 6 as libc::c_int);
                nmmax = (ctl.limit - ctl.cur_len) as size_t;
                if nmmax > 0xff as libc::c_int as libc::c_ulong {
                    nmmax = 0xff as libc::c_int as size_t
                }
            }
            if !bp.is_null() {
                *bp.offset(1 as libc::c_int as isize) = 'N' as i32 as libc::c_uchar;
                *bp.offset(2 as libc::c_int as isize) = 'M' as i32 as libc::c_uchar;
                *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar
                /* version */
            }
        }
        length = 5 as libc::c_int + nmlen as libc::c_int;
        if !bp.is_null() {
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(5 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
            memcpy(
                bp.offset(6 as libc::c_int as isize) as *mut libc::c_void,
                nm as *const libc::c_void,
                nmlen,
            );
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "PX" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_PX != 0 {
        /*
         *   "PX" Format:
         *               len  ver
         *    +----+----+----+----+-----------+-----------+
         *    | 'P'| 'X'| 2C | 01 | FILE MODE |   LINKS   |
         *    +----+----+----+----+-----------+-----------+
         *    0    1    2    3    4          12          20
         *    +-----------+-----------+------------------+
         *    |  USER ID  | GROUP ID  |FILE SERIAL NUMBER|
         *    +-----------+-----------+------------------+
         *   20          28          36                 44
         */
        length = 44 as libc::c_int;
        if ctl.limit - ctl.cur_len < length {
            bp = extra_next_record(&mut ctl, length)
        }
        if !bp.is_null() {
            let mut mode: mode_t = 0;
            let mut uid: int64_t = 0;
            let mut gid: int64_t = 0;
            mode = archive_entry_mode((*file).entry);
            uid = archive_entry_uid((*file).entry);
            gid = archive_entry_gid((*file).entry);
            if (*iso9660).opt.rr() as libc::c_int == OPT_RR_USEFUL {
                /*
                 * This action is similar to mkisofs -r option
                 * but our rockridge=useful option does not
                 * set a zero to uid and gid.
                 */
                /* set all read bit ON */
                mode |= 0o444 as libc::c_int as libc::c_uint;
                if mode & 0o111 as libc::c_int as libc::c_uint != 0 {
                    /* set all exec bit ON */
                    mode |= 0o111 as libc::c_int as libc::c_uint
                }
                /* clear all write bits. */
                mode &= !(0o222 as libc::c_int) as libc::c_uint;
                /* clear setuid,setgid,sticky bits. */
                mode &= !(0o7000 as libc::c_int) as libc::c_uint
            } /* version	*/
            *bp.offset(1 as libc::c_int as isize) = 'P' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'X' as i32 as libc::c_uchar;
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            /* file mode */
            set_num_733(bp.offset(5 as libc::c_int as isize), mode);
            /* file links (stat.st_nlink) */
            set_num_733(
                bp.offset(13 as libc::c_int as isize),
                archive_entry_nlink((*file).entry),
            );
            set_num_733(bp.offset(21 as libc::c_int as isize), uid as uint32_t);
            set_num_733(bp.offset(29 as libc::c_int as isize), gid as uint32_t);
            /* File Serial Number */
            if (*pxent).dir() != 0 {
                set_num_733(bp.offset(37 as libc::c_int as isize), (*pxent).dir_location);
            } else if !(*file).hardlink_target.is_null() {
                set_num_733(
                    bp.offset(37 as libc::c_int as isize),
                    (*(*(*file).hardlink_target).cur_content).location,
                );
            } else {
                set_num_733(
                    bp.offset(37 as libc::c_int as isize),
                    (*(*file).cur_content).location,
                );
            }
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "SL" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_SL != 0 {
        /*
         *   "SL" Format:
         *     e.g. a symbolic name is 'foo/bar'
         *               len  ver  flg
         *    +----+----+----+----+----+------------+
         *    | 'S'| 'L'| 0F | 01 | 00 | components |
         *    +----+----+----+----+----+-----+------+
         *    0    1    2    3    4    5  ...|...  15
         *    <----------------- len --------+------>
         *    components :                   |
         *     cflg clen                     |
         *    +----+----+----+----+----+     |
         *    | 00 | 03 | 'f'| 'o'| 'o'| <---+
         *    +----+----+----+----+----+     |
         *    5    6    7    8    9   10     |
         *     cflg clen                     |
         *    +----+----+----+----+----+     |
         *    | 00 | 03 | 'b'| 'a'| 'r'| <---+
         *    +----+----+----+----+----+
         *   10   11   12   13   14   15
         *
         *    - cflg : flag of component
         *    - clen : length of component
         */
        let mut sl: *const libc::c_char = 0 as *const libc::c_char;
        let mut sl_last: libc::c_char = 0;
        if ctl.limit - ctl.cur_len < 7 as libc::c_int {
            bp = extra_next_record(&mut ctl, 7 as libc::c_int)
        }
        sl = (*file).symlink.s;
        sl_last = '\u{0}' as i32 as libc::c_char;
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'S' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'L' as i32 as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar
            /* version	*/
        }
        loop {
            let mut nc: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut cf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut cl: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut cldmy: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
            let mut sllen: libc::c_int = 0;
            let mut slmax: libc::c_int = 0;
            slmax = ctl.limit - ctl.cur_len;
            if slmax > 0xff as libc::c_int {
                slmax = 0xff as libc::c_int
            }
            if !bp.is_null() {
                nc = &mut *bp.offset(6 as libc::c_int as isize) as *mut libc::c_uchar
            } else {
                nc = NULL as *mut libc::c_uchar
            }
            cl = NULL as *mut libc::c_uchar;
            cf = cl;
            sllen = 0 as libc::c_int;
            while *sl as libc::c_int != 0 && (sllen + 11 as libc::c_int) < slmax {
                if sl_last as libc::c_int == '\u{0}' as i32
                    && *sl.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                {
                    /*
                     *     flg  len
                     *    +----+----+
                     *    | 08 | 00 | ROOT component.
                     *    +----+----+ ("/")
                     *
                     * Root component has to appear
                     * at the first component only.
                     */
                    if !nc.is_null() {
                        let fresh5 = nc; /* ROOT */
                        nc = nc.offset(1);
                        cf = fresh5;
                        *cf = 0x8 as libc::c_int as libc::c_uchar;
                        let fresh6 = nc;
                        nc = nc.offset(1);
                        *fresh6 = 0 as libc::c_int as libc::c_uchar
                    }
                    sllen += 2 as libc::c_int;
                    sl = sl.offset(1);
                    sl_last = '/' as i32 as libc::c_char;
                    cl = NULL as *mut libc::c_uchar
                } else if (sl_last as libc::c_int == '\u{0}' as i32
                    || sl_last as libc::c_int == '/' as i32)
                    && *sl.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && *sl.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && (*sl.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                        || *sl.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
                    || *sl.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                        && *sl.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                        && *sl.offset(2 as libc::c_int as isize) as libc::c_int == '.' as i32
                        && (*sl.offset(3 as libc::c_int as isize) as libc::c_int == '/' as i32
                            || *sl.offset(3 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32)
                {
                    /*
                     *     flg  len
                     *    +----+----+
                     *    | 04 | 00 | PARENT component.
                     *    +----+----+ ("..")
                     */
                    if !nc.is_null() {
                        let fresh7 = nc; /* PARENT */
                        nc = nc.offset(1); /* skip ".." */
                        cf = fresh7; /* skip "/.." */
                        *cf = 0x4 as libc::c_int as libc::c_uchar;
                        let fresh8 = nc;
                        nc = nc.offset(1);
                        *fresh8 = 0 as libc::c_int as libc::c_uchar
                    }
                    sllen += 2 as libc::c_int;
                    if *sl.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                        sl = sl.offset(3 as libc::c_int as isize)
                    } else {
                        sl = sl.offset(2 as libc::c_int as isize)
                    }
                    sl_last = '.' as i32 as libc::c_char;
                    cl = NULL as *mut libc::c_uchar
                } else if (sl_last as libc::c_int == '\u{0}' as i32
                    || sl_last as libc::c_int == '/' as i32)
                    && *sl.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && (*sl.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                        || *sl.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32)
                    || *sl.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                        && *sl.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                        && (*sl.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                            || *sl.offset(2 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32)
                {
                    /*
                     *     flg  len
                     *    +----+----+
                     *    | 02 | 00 | CURRENT component.
                     *    +----+----+ (".")
                     */
                    if !nc.is_null() {
                        let fresh9 = nc; /* CURRENT */
                        nc = nc.offset(1); /* skip "." */
                        cf = fresh9; /* skip "/." */
                        *cf = 0x2 as libc::c_int as libc::c_uchar;
                        let fresh10 = nc;
                        nc = nc.offset(1);
                        *fresh10 = 0 as libc::c_int as libc::c_uchar
                    }
                    sllen += 2 as libc::c_int;
                    if *sl.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                        sl = sl.offset(2 as libc::c_int as isize)
                    } else {
                        sl = sl.offset(1)
                    }
                    sl_last = '.' as i32 as libc::c_char;
                    cl = NULL as *mut libc::c_uchar
                } else {
                    if *sl.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                        || cl.is_null()
                    {
                        if !nc.is_null() {
                            let fresh11 = nc;
                            nc = nc.offset(1);
                            cf = fresh11;
                            *cf = 0 as libc::c_int as libc::c_uchar;
                            let fresh12 = nc;
                            nc = nc.offset(1);
                            cl = fresh12;
                            *cl = 0 as libc::c_int as libc::c_uchar
                        } else {
                            cl = &mut cldmy
                        }
                        sllen += 2 as libc::c_int;
                        if *sl.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                            let fresh13 = sl;
                            sl = sl.offset(1);
                            sl_last = *fresh13;
                            continue;
                        }
                    }
                    let fresh14 = sl;
                    sl = sl.offset(1);
                    sl_last = *fresh14;
                    if !nc.is_null() {
                        let fresh15 = nc;
                        nc = nc.offset(1);
                        *fresh15 = sl_last as libc::c_uchar;
                        *cl = (*cl).wrapping_add(1)
                    }
                    sllen += 1
                }
            }
            if *sl != 0 {
                length = 5 as libc::c_int + sllen;
                if !bp.is_null() {
                    /*
                     * Mark flg as CONTINUE component.
                     */
                    *cf = (*cf as libc::c_int | 0x1 as libc::c_int) as libc::c_uchar;
                    /*
                     *               len  ver  flg
                     *    +----+----+----+----+----+-
                     *    | 'S'| 'L'| XX | 01 | 01 |
                     *    +----+----+----+----+----+-
                     *                           ^
                     *           continues in next "SL"
                     */
                    *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar; /* This Symbolic Link
                                                                                      * continues in next
                                                                                      * "SL" field */
                    *bp.offset(5 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
                    bp = bp.offset(length as isize)
                }
                extra_tell_used_size(&mut ctl, length);
                if ctl.limit - ctl.cur_len < 11 as libc::c_int {
                    bp = extra_next_record(&mut ctl, 11 as libc::c_int)
                }
                if !bp.is_null() {
                    /* Next 'SL' */
                    *bp.offset(1 as libc::c_int as isize) = 'S' as i32 as libc::c_uchar;
                    *bp.offset(2 as libc::c_int as isize) = 'L' as i32 as libc::c_uchar;
                    *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar
                    /* version */
                }
            } else {
                length = 5 as libc::c_int + sllen;
                if !bp.is_null() {
                    *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
                    *bp.offset(5 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
                    bp = bp.offset(length as isize)
                }
                extra_tell_used_size(&mut ctl, length);
                break;
            }
        }
    }
    /* Write "TF" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_TF != 0 {
        /*
         *   "TF" Format:
         *               len  ver
         *    +----+----+----+----+-----+-------------+
         *    | 'T'| 'F'| XX | 01 |FLAGS| TIME STAMPS |
         *    +----+----+----+----+-----+-------------+
         *    0    1    2    3    4     5            XX
         *    TIME STAMPS : ISO 9660 Standard 9.1.5.
         *                  If TF_LONG_FORM FLAGS is set,
         *                  use ISO9660 Standard 8.4.26.1.
         */
        /* Creation time recorded		*/
        /* Modification time recorded		*/
        /* Last Access time recorded		*/
        /* Last Attribute Change time recorded  */
        /* Last Backup time recorded		*/
        /* Expiration time recorded		*/
        /* Effective time recorded		*/
        /* ISO 9660 17-byte time format used	*/
        let mut tf_flags: libc::c_uchar = 0; /* version	*/
        length = 5 as libc::c_int;
        tf_flags = 0 as libc::c_int as libc::c_uchar;
        if archive_entry_birthtime_is_set((*file).entry) != 0
            && archive_entry_birthtime((*file).entry) <= archive_entry_mtime((*file).entry)
        {
            length += 7 as libc::c_int;
            tf_flags = (tf_flags as libc::c_int | TF_CREATION) as libc::c_uchar
        }
        if archive_entry_mtime_is_set((*file).entry) != 0 {
            length += 7 as libc::c_int;
            tf_flags = (tf_flags as libc::c_int | TF_MODIFY) as libc::c_uchar
        }
        if archive_entry_atime_is_set((*file).entry) != 0 {
            length += 7 as libc::c_int;
            tf_flags = (tf_flags as libc::c_int | TF_ACCESS) as libc::c_uchar
        }
        if archive_entry_ctime_is_set((*file).entry) != 0 {
            length += 7 as libc::c_int;
            tf_flags = (tf_flags as libc::c_int | TF_ATTRIBUTES) as libc::c_uchar
        }
        if ctl.limit - ctl.cur_len < length {
            bp = extra_next_record(&mut ctl, length)
        }
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'T' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'F' as i32 as libc::c_uchar;
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            *bp.offset(5 as libc::c_int as isize) = tf_flags;
            bp = bp.offset(5 as libc::c_int as isize);
            /* Creation time */
            if tf_flags as libc::c_int & TF_CREATION != 0 {
                set_time_915(
                    bp.offset(1 as libc::c_int as isize),
                    archive_entry_birthtime((*file).entry),
                );
                bp = bp.offset(7 as libc::c_int as isize)
            }
            /* Modification time */
            if tf_flags as libc::c_int & TF_MODIFY != 0 {
                set_time_915(
                    bp.offset(1 as libc::c_int as isize),
                    archive_entry_mtime((*file).entry),
                );
                bp = bp.offset(7 as libc::c_int as isize)
            }
            /* Last Access time */
            if tf_flags as libc::c_int & TF_ACCESS != 0 {
                set_time_915(
                    bp.offset(1 as libc::c_int as isize),
                    archive_entry_atime((*file).entry),
                );
                bp = bp.offset(7 as libc::c_int as isize)
            }
            /* Last Attribute Change time */
            if tf_flags as libc::c_int & TF_ATTRIBUTES != 0 {
                set_time_915(
                    bp.offset(1 as libc::c_int as isize),
                    archive_entry_ctime((*file).entry),
                );
                bp = bp.offset(7 as libc::c_int as isize)
            }
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "RE" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_RE != 0 {
        /*
         *   "RE" Format:
         *               len  ver
         *    +----+----+----+----+
         *    | 'R'| 'E'| 04 | 01 |
         *    +----+----+----+----+
         *    0    1    2    3    4
         */
        length = 4 as libc::c_int; /* version	*/
        if ctl.limit - ctl.cur_len < length {
            bp = extra_next_record(&mut ctl, length)
        }
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'R' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'E' as i32 as libc::c_uchar;
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "PL" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_PL != 0 {
        /*
         *   "PL" Format:
         *               len  ver
         *    +----+----+----+----+------------+
         *    | 'P'| 'L'| 0C | 01 | *LOCATION  |
         *    +----+----+----+----+------------+
         *    0    1    2    3    4           12
         *    *LOCATION: location of parent directory
         */
        length = 12 as libc::c_int; /* version	*/
        if ctl.limit - ctl.cur_len < length {
            bp = extra_next_record(&mut ctl, length)
        }
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'P' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'L' as i32 as libc::c_uchar;
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            set_num_733(
                bp.offset(5 as libc::c_int as isize),
                (*rr_parent).dir_location,
            );
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "CL" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_CL != 0 {
        /*
         *   "CL" Format:
         *               len  ver
         *    +----+----+----+----+------------+
         *    | 'C'| 'L'| 0C | 01 | *LOCATION  |
         *    +----+----+----+----+------------+
         *    0    1    2    3    4           12
         *    *LOCATION: location of child directory
         */
        length = 12 as libc::c_int; /* version	*/
        if ctl.limit - ctl.cur_len < length {
            bp = extra_next_record(&mut ctl, length)
        }
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'C' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'L' as i32 as libc::c_uchar;
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            set_num_733(
                bp.offset(5 as libc::c_int as isize),
                (*(*isoent).rr_child).dir_location,
            );
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "PN" System Use Entry. */
    if rr_flag as libc::c_int & RR_USE_PN != 0 {
        /*
         *   "PN" Format:
         *               len  ver
         *    +----+----+----+----+------------+------------+
         *    | 'P'| 'N'| 14 | 01 | dev_t high | dev_t low  |
         *    +----+----+----+----+------------+------------+
         *    0    1    2    3    4           12           20
         */
        length = 20 as libc::c_int; /* version	*/
        if ctl.limit - ctl.cur_len < length {
            bp = extra_next_record(&mut ctl, length)
        }
        if !bp.is_null() {
            let mut dev: uint64_t = 0;
            *bp.offset(1 as libc::c_int as isize) = 'P' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'N' as i32 as libc::c_uchar;
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            dev = archive_entry_rdev((*file).entry);
            set_num_733(
                bp.offset(5 as libc::c_int as isize),
                (dev >> 32 as libc::c_int) as uint32_t,
            );
            set_num_733(
                bp.offset(13 as libc::c_int as isize),
                (dev & 0xffffffff as libc::c_uint as libc::c_ulong) as uint32_t,
            );
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "ZF" System Use Entry. */
    if (*file).zisofs.header_size != 0 {
        /*
         *   "ZF" Format:
         *               len  ver
         *    +----+----+----+----+----+----+-------------+
         *    | 'Z'| 'F'| 10 | 01 | 'p'| 'z'| Header Size |
         *    +----+----+----+----+----+----+-------------+
         *    0    1    2    3    4    5    6             7
         *    +--------------------+-------------------+
         *    | Log2 of block Size | Uncompressed Size |
         *    +--------------------+-------------------+
         *    7                    8                   16
         */
        length = 16 as libc::c_int; /* version	*/
        if ctl.limit - ctl.cur_len < length {
            bp = extra_next_record(&mut ctl, length)
        }
        if !bp.is_null() {
            *bp.offset(1 as libc::c_int as isize) = 'Z' as i32 as libc::c_uchar;
            *bp.offset(2 as libc::c_int as isize) = 'F' as i32 as libc::c_uchar;
            *bp.offset(3 as libc::c_int as isize) = length as libc::c_uchar;
            *bp.offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
            *bp.offset(5 as libc::c_int as isize) = 'p' as i32 as libc::c_uchar;
            *bp.offset(6 as libc::c_int as isize) = 'z' as i32 as libc::c_uchar;
            *bp.offset(7 as libc::c_int as isize) = (*file).zisofs.header_size;
            *bp.offset(8 as libc::c_int as isize) = (*file).zisofs.log2_bs;
            set_num_733(
                bp.offset(9 as libc::c_int as isize),
                (*file).zisofs.uncompressed_size,
            );
            bp = bp.offset(length as isize)
        }
        extra_tell_used_size(&mut ctl, length);
    }
    /* Write "CE" System Use Entry. */
    if t as libc::c_uint == DIR_REC_SELF as libc::c_int as libc::c_uint
        && isoent == (*isoent).parent
    {
        length = RR_CE_SIZE;
        if !bp.is_null() {
            set_SUSP_CE(
                bp.offset(1 as libc::c_int as isize),
                (*iso9660).location_rrip_er,
                0 as libc::c_int,
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<[libc::c_char; 85]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<[libc::c_char; 136]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) as libc::c_int,
            );
        }
        extra_tell_used_size(&mut ctl, length);
    }
    extra_close_record(&mut ctl, 0 as libc::c_int);
    return ctl.dr_len;
}
pub const RR_USE_PX: libc::c_int = 0x1 as libc::c_int;
pub const RR_USE_PN: libc::c_int = 0x2 as libc::c_int;
pub const RR_USE_SL: libc::c_int = 0x4 as libc::c_int;
pub const RR_USE_NM: libc::c_int = 0x8 as libc::c_int;
pub const RR_USE_CL: libc::c_int = 0x10 as libc::c_int;
pub const RR_USE_PL: libc::c_int = 0x20 as libc::c_int;
pub const RR_USE_RE: libc::c_int = 0x40 as libc::c_int;
pub const RR_USE_TF: libc::c_int = 0x80 as libc::c_int;
pub const TF_CREATION: libc::c_int = 0x1 as libc::c_int;
pub const TF_MODIFY: libc::c_int = 0x2 as libc::c_int;
pub const TF_ACCESS: libc::c_int = 0x4 as libc::c_int;
pub const TF_ATTRIBUTES: libc::c_int = 0x8 as libc::c_int;
/*
 * Write data of a Directory Record or calculate writing bytes itself.
 * If parameter `p' is NULL, calculates the size of writing data, which
 * a Directory Record needs to write, then it saved and return
 * the calculated size.
 * Parameter `n' is a remaining size of buffer. when parameter `p' is
 * not NULL, check whether that `n' is not less than the saved size.
 * if that `n' is small, return zero.
 *
 * This format of the Directory Record is according to
 * ISO9660 Standard 9.1
 */
unsafe extern "C" fn set_directory_record(
    mut p: *mut libc::c_uchar,
    mut n: size_t,
    mut isoent: *mut isoent,
    mut iso9660: *mut iso9660,
    mut t: dir_rec_type,
    mut vdd_type: vdd_type,
) -> libc::c_int {
    let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dr_len: size_t = 0;
    let mut fi_len: size_t = 0;
    if !p.is_null() {
        /*
         * Check whether a write buffer size is less than the
         * saved size which is needed to write this Directory
         * Record.
         */
        match t as libc::c_uint {
            0 => dr_len = (*isoent).dr_len.vd as size_t,
            1 => dr_len = (*isoent).dr_len.self_0 as size_t,
            2 => dr_len = (*isoent).dr_len.parent as size_t,
            3 | _ => dr_len = (*isoent).dr_len.normal as size_t,
        }
        if dr_len > n {
            return 0 as libc::c_int;
        }
        /* Needs more buffer size. */
    }
    if t as libc::c_uint == DIR_REC_NORMAL as libc::c_int as libc::c_uint
        && !(*isoent).identifier.is_null()
    {
        fi_len = (*isoent).id_len as size_t
    } else {
        fi_len = 1 as libc::c_int as size_t
    }
    if !p.is_null() {
        let mut xisoent: *mut isoent = 0 as *mut isoent;
        let mut file: *mut isofile = 0 as *mut isofile;
        let mut flag: libc::c_uchar = 0;
        if t as libc::c_uint == DIR_REC_PARENT as libc::c_int as libc::c_uint {
            xisoent = (*isoent).parent
        } else {
            xisoent = isoent
        }
        file = (*isoent).file;
        if !(*file).hardlink_target.is_null() {
            file = (*file).hardlink_target
        }
        /* Make a file flag. */
        if (*xisoent).dir() != 0 {
            flag = FILE_FLAG_DIRECTORY as libc::c_uchar
        } else if !(*(*file).cur_content).next.is_null() {
            flag = FILE_FLAG_MULTI_EXTENT as libc::c_uchar
        } else {
            flag = 0 as libc::c_int as libc::c_uchar
        }
        bp = p.offset(-(1 as libc::c_int as isize));
        /* Extended Attribute Record Length */
        set_num_711(
            bp.offset(2 as libc::c_int as isize),
            0 as libc::c_int as libc::c_uchar,
        );
        /* Location of Extent */
        if (*xisoent).dir() != 0 {
            set_num_733(
                bp.offset(3 as libc::c_int as isize),
                (*xisoent).dir_location,
            );
        } else {
            set_num_733(
                bp.offset(3 as libc::c_int as isize),
                (*(*file).cur_content).location,
            );
        }
        /* Data Length */
        if (*xisoent).dir() != 0 {
            set_num_733(
                bp.offset(11 as libc::c_int as isize),
                ((*xisoent).dir_block * LOGICAL_BLOCK_SIZE) as uint32_t,
            );
        } else {
            set_num_733(
                bp.offset(11 as libc::c_int as isize),
                (*(*file).cur_content).size as uint32_t,
            );
        }
        /* Recording Date and Time */
        /* NOTE:
         *  If a file type is symbolic link, you are seeing this
         *  field value is different from a value mkisofs makes.
         *  libarchive uses lstat to get this one, but it
         *  seems mkisofs uses stat to get.
         */
        set_time_915(
            bp.offset(19 as libc::c_int as isize),
            archive_entry_mtime((*(*xisoent).file).entry),
        );
        /* File Flags */
        *bp.offset(26 as libc::c_int as isize) = flag;
        /* File Unit Size */
        set_num_711(
            bp.offset(27 as libc::c_int as isize),
            0 as libc::c_int as libc::c_uchar,
        );
        /* Interleave Gap Size */
        set_num_711(
            bp.offset(28 as libc::c_int as isize),
            0 as libc::c_int as libc::c_uchar,
        );
        /* Volume Sequence Number */
        set_num_723(
            bp.offset(29 as libc::c_int as isize),
            (*iso9660).volume_sequence_number as uint16_t,
        );
        /* Length of File Identifier */
        set_num_711(
            bp.offset(33 as libc::c_int as isize),
            fi_len as libc::c_uchar,
        );
        /* File Identifier */
        match t as libc::c_uint {
            0 | 1 => {
                set_num_711(
                    bp.offset(34 as libc::c_int as isize),
                    0 as libc::c_int as libc::c_uchar,
                );
            }
            2 => {
                set_num_711(
                    bp.offset(34 as libc::c_int as isize),
                    1 as libc::c_int as libc::c_uchar,
                );
            }
            3 => {
                if !(*isoent).identifier.is_null() {
                    memcpy(
                        bp.offset(34 as libc::c_int as isize) as *mut libc::c_void,
                        (*isoent).identifier as *const libc::c_void,
                        fi_len,
                    );
                } else {
                    set_num_711(
                        bp.offset(34 as libc::c_int as isize),
                        0 as libc::c_int as libc::c_uchar,
                    );
                }
            }
            _ => {}
        }
    } else {
        bp = NULL as *mut libc::c_uchar
    }
    dr_len = (33 as libc::c_int as libc::c_ulong).wrapping_add(fi_len);
    /* Padding Field */
    if dr_len & 0x1 as libc::c_int as libc::c_ulong != 0 {
        dr_len = dr_len.wrapping_add(1);
        if !p.is_null() {
            *bp.offset(dr_len as isize) = 0 as libc::c_int as libc::c_uchar
        }
    }
    /* Volume Descriptor does not record extension. */
    if t as libc::c_uint == DIR_REC_VD as libc::c_int as libc::c_uint {
        if !p.is_null() {
            /* Length of Directory Record */
            set_num_711(p, dr_len as libc::c_uchar);
        } else {
            (*isoent).dr_len.vd = dr_len as libc::c_int
        }
        return dr_len as libc::c_int;
    }
    /* Rockridge */
    if (*iso9660).opt.rr() as libc::c_int != 0
        && vdd_type as libc::c_uint != VDD_JOLIET as libc::c_int as libc::c_uint
    {
        dr_len = set_directory_record_rr(bp, dr_len as libc::c_int, isoent, iso9660, t) as size_t
    }
    if !p.is_null() {
        /* Length of Directory Record */
        set_num_711(p, dr_len as libc::c_uchar);
    } else {
        /*
         * Save the size which is needed to write this
         * Directory Record.
         */
        match t as libc::c_uint {
            1 => (*isoent).dr_len.self_0 = dr_len as libc::c_int,
            2 => (*isoent).dr_len.parent = dr_len as libc::c_int,
            3 => (*isoent).dr_len.normal = dr_len as libc::c_int,
            0 | _ => {}
        }
    }
    return dr_len as libc::c_int;
}
/*
 * Calculate the size of a directory record.
 */
#[inline]
unsafe extern "C" fn get_dir_rec_size(
    mut iso9660: *mut iso9660,
    mut isoent: *mut isoent,
    mut t: dir_rec_type,
    mut vdd_type: vdd_type,
) -> libc::c_int {
    return set_directory_record(
        NULL as *mut libc::c_uchar,
        SIZE_MAX,
        isoent,
        iso9660,
        t,
        vdd_type,
    );
}
/*
 * Manage to write ISO-image data with wbuff to reduce calling
 * __archive_write_output() for performance.
 */
#[inline]
unsafe extern "C" fn wb_buffptr(mut a: *mut archive_write) -> *mut libc::c_uchar {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    return &mut *(*iso9660).wbuff.as_mut_ptr().offset(
        (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
            .wrapping_sub((*iso9660).wbuff_remaining) as isize,
    ) as *mut libc::c_uchar;
}
unsafe extern "C" fn wb_write_out(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut wsize: size_t = 0;
    let mut nw: size_t = 0;
    let mut r: libc::c_int = 0;
    wsize = (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
        .wrapping_sub((*iso9660).wbuff_remaining);
    nw = wsize.wrapping_rem(LOGICAL_BLOCK_SIZE as libc::c_ulong);
    if (*iso9660).wbuff_type as libc::c_uint == WB_TO_STREAM as libc::c_int as libc::c_uint {
        r = __archive_write_output(
            a,
            (*iso9660).wbuff.as_mut_ptr() as *const libc::c_void,
            wsize.wrapping_sub(nw),
        )
    } else {
        r = write_to_temp(
            a,
            (*iso9660).wbuff.as_mut_ptr() as *const libc::c_void,
            wsize.wrapping_sub(nw),
        )
    }
    /* Increase the offset. */
    (*iso9660).wbuff_offset = ((*iso9660).wbuff_offset as libc::c_ulong)
        .wrapping_add(wsize.wrapping_sub(nw)) as int64_t as int64_t;
    if (*iso9660).wbuff_offset > (*iso9660).wbuff_written {
        (*iso9660).wbuff_written = (*iso9660).wbuff_offset
    }
    (*iso9660).wbuff_remaining = ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong;
    if nw != 0 {
        (*iso9660).wbuff_remaining =
            ((*iso9660).wbuff_remaining as libc::c_ulong).wrapping_sub(nw) as size_t as size_t;
        memmove(
            (*iso9660).wbuff.as_mut_ptr() as *mut libc::c_void,
            (*iso9660)
                .wbuff
                .as_mut_ptr()
                .offset(wsize as isize)
                .offset(-(nw as isize)) as *const libc::c_void,
            nw,
        );
    }
    return r;
}
unsafe extern "C" fn wb_consume(mut a: *mut archive_write, mut size: size_t) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    if size > (*iso9660).wbuff_remaining
        || (*iso9660).wbuff_remaining == 0 as libc::c_int as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Internal Programming error: iso9660:wb_consume() size=%jd, wbuff_remaining=%jd\x00"
                as *const u8 as *const libc::c_char,
            size as intmax_t,
            (*iso9660).wbuff_remaining as intmax_t,
        );
        return -(30 as libc::c_int);
    }
    (*iso9660).wbuff_remaining =
        ((*iso9660).wbuff_remaining as libc::c_ulong).wrapping_sub(size) as size_t as size_t;
    if (*iso9660).wbuff_remaining < LOGICAL_BLOCK_SIZE as libc::c_ulong {
        return wb_write_out(a);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wb_set_offset(mut a: *mut archive_write, mut off: int64_t) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut used: int64_t = 0;
    let mut ext_bytes: int64_t = 0;
    if (*iso9660).wbuff_type as libc::c_uint != WB_TO_TEMP as libc::c_int as libc::c_uint {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Internal Programming error: iso9660:wb_set_offset()\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    used = (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
        .wrapping_sub((*iso9660).wbuff_remaining) as int64_t;
    if (*iso9660).wbuff_offset + used > (*iso9660).wbuff_tail {
        (*iso9660).wbuff_tail = (*iso9660).wbuff_offset + used
    }
    if (*iso9660).wbuff_offset < (*iso9660).wbuff_written {
        if used > 0 as libc::c_int as libc::c_long
            && write_to_temp(
                a,
                (*iso9660).wbuff.as_mut_ptr() as *const libc::c_void,
                used as size_t,
            ) != ARCHIVE_OK
        {
            return -(30 as libc::c_int);
        }
        (*iso9660).wbuff_offset = (*iso9660).wbuff_written;
        lseek((*iso9660).temp_fd, (*iso9660).wbuff_offset, SEEK_SET);
        (*iso9660).wbuff_remaining =
            ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong;
        used = 0 as libc::c_int as int64_t
    }
    if off < (*iso9660).wbuff_offset {
        /*
         * Write out waiting data.
         */
        if used > 0 as libc::c_int as libc::c_long {
            if wb_write_out(a) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        lseek((*iso9660).temp_fd, off, SEEK_SET);
        (*iso9660).wbuff_offset = off;
        (*iso9660).wbuff_remaining =
            ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong
    } else if off <= (*iso9660).wbuff_tail {
        (*iso9660).wbuff_remaining = (::std::mem::size_of::<[libc::c_uchar; 65536]>()
            as libc::c_ulong)
            .wrapping_sub((off - (*iso9660).wbuff_offset) as libc::c_ulong)
    } else {
        ext_bytes = off - (*iso9660).wbuff_tail;
        (*iso9660).wbuff_remaining = (::std::mem::size_of::<[libc::c_uchar; 65536]>()
            as libc::c_ulong)
            .wrapping_sub(((*iso9660).wbuff_tail - (*iso9660).wbuff_offset) as libc::c_ulong);
        while ext_bytes >= (*iso9660).wbuff_remaining as int64_t {
            if write_null(a, (*iso9660).wbuff_remaining) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
            ext_bytes = (ext_bytes as libc::c_ulong).wrapping_sub((*iso9660).wbuff_remaining)
                as int64_t as int64_t
        }
        if ext_bytes > 0 as libc::c_int as libc::c_long {
            if write_null(a, ext_bytes as size_t) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
/* HAVE_ZLIB_H */
unsafe extern "C" fn write_null(mut a: *mut archive_write, mut size: size_t) -> libc::c_int {
    let mut remaining: size_t = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut old: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: libc::c_int = 0;
    remaining = (*((*a).format_data as *mut iso9660)).wbuff_remaining;
    p = wb_buffptr(a);
    if size <= remaining {
        memset(p as *mut libc::c_void, 0 as libc::c_int, size);
        return wb_consume(a, size);
    }
    memset(p as *mut libc::c_void, 0 as libc::c_int, remaining);
    r = wb_consume(a, remaining);
    if r != ARCHIVE_OK {
        return r;
    }
    size = (size as libc::c_ulong).wrapping_sub(remaining) as size_t as size_t;
    old = p;
    p = wb_buffptr(a);
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        old.offset_from(p) as libc::c_long as libc::c_ulong,
    );
    remaining = (*((*a).format_data as *mut iso9660)).wbuff_remaining;
    while size != 0 {
        let mut wsize: size_t = size;
        if wsize > remaining {
            wsize = remaining
        }
        r = wb_consume(a, wsize);
        if r != ARCHIVE_OK {
            return r;
        }
        size = (size as libc::c_ulong).wrapping_sub(wsize) as size_t as size_t
    }
    return 0 as libc::c_int;
}
/*
 * Write Volume Descriptor Set Terminator
 */
unsafe extern "C" fn write_VD_terminator(mut a: *mut archive_write) -> libc::c_int {
    let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    bp = wb_buffptr(a).offset(-(1 as libc::c_int as isize));
    set_VD_bp(bp, VDT_TERMINATOR, 1 as libc::c_int as libc::c_uchar);
    set_unused_field_bp(bp, 8 as libc::c_int, LOGICAL_BLOCK_SIZE);
    return wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
}
unsafe extern "C" fn set_file_identifier(
    mut bp: *mut libc::c_uchar,
    mut from: libc::c_int,
    mut to: libc::c_int,
    mut vdc: vdc,
    mut a: *mut archive_write,
    mut vdd: *mut vdd,
    mut id: *mut archive_string,
    mut label: *const libc::c_char,
    mut leading_under: libc::c_int,
    mut char_type: char_type,
) -> libc::c_int {
    let mut identifier: [libc::c_char; 256] = [0; 256];
    let mut isoent: *mut isoent = 0 as *mut isoent;
    let mut ids: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut r: libc::c_int = 0;
    if (*id).length > 0 as libc::c_int as libc::c_ulong
        && leading_under != 0
        && *(*id).s.offset(0 as libc::c_int as isize) as libc::c_int != '_' as i32
    {
        if char_type as libc::c_uint == A_CHAR as libc::c_int as libc::c_uint {
            r = set_str_a_characters_bp(a, bp, from, to, (*id).s, vdc)
        } else {
            r = set_str_d_characters_bp(a, bp, from, to, (*id).s, vdc)
        }
    } else if (*id).length > 0 as libc::c_int as libc::c_ulong {
        ids = (*id).s;
        if leading_under != 0 {
            ids = ids.offset(1)
        }
        isoent = isoent_find_entry((*vdd).rootent, ids);
        if isoent.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Not Found %s `%s\'.\x00" as *const u8 as *const libc::c_char,
                label,
                ids,
            );
            return -(30 as libc::c_int);
        }
        len = ((*isoent).ext_off + (*isoent).ext_len) as size_t;
        if (*vdd).vdd_type as libc::c_uint == VDD_JOLIET as libc::c_int as libc::c_uint {
            if len
                > (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            {
                len = (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
            }
        } else if len
            > (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            len = (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        }
        memcpy(
            identifier.as_mut_ptr() as *mut libc::c_void,
            (*isoent).identifier as *const libc::c_void,
            len,
        );
        identifier[len as usize] = '\u{0}' as i32 as libc::c_char;
        if (*vdd).vdd_type as libc::c_uint == VDD_JOLIET as libc::c_int as libc::c_uint {
            identifier[len.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize] =
                0 as libc::c_int as libc::c_char;
            vdc = VDC_UCS2_DIRECT
        }
        if char_type as libc::c_uint == A_CHAR as libc::c_int as libc::c_uint {
            r = set_str_a_characters_bp(a, bp, from, to, identifier.as_mut_ptr(), vdc)
        } else {
            r = set_str_d_characters_bp(a, bp, from, to, identifier.as_mut_ptr(), vdc)
        }
    } else if char_type as libc::c_uint == A_CHAR as libc::c_int as libc::c_uint {
        r = set_str_a_characters_bp(a, bp, from, to, NULL as *const libc::c_char, vdc)
    } else {
        r = set_str_d_characters_bp(a, bp, from, to, NULL as *const libc::c_char, vdc)
    }
    return r;
}
/*
 * Write Primary/Supplementary Volume Descriptor
 */
unsafe extern "C" fn write_VD(mut a: *mut archive_write, mut vdd: *mut vdd) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut volume_set_size: uint16_t = 1 as libc::c_int as uint16_t;
    let mut identifier: [libc::c_char; 256] = [0; 256];
    let mut vdt: VD_type = VDT_BOOT_RECORD;
    let mut vdc: vdc = VDC_STD;
    let mut vd_ver: libc::c_uchar = 0;
    let mut fst_ver: libc::c_uchar = 0;
    let mut r: libc::c_int = 0;
    iso9660 = (*a).format_data as *mut iso9660;
    match (*vdd).vdd_type as libc::c_uint {
        1 => {
            vdt = VDT_SUPPLEMENTARY;
            fst_ver = 1 as libc::c_int as libc::c_uchar;
            vd_ver = fst_ver;
            vdc = VDC_UCS2
        }
        2 => {
            vdt = VDT_SUPPLEMENTARY;
            fst_ver = 2 as libc::c_int as libc::c_uchar;
            vd_ver = fst_ver;
            vdc = VDC_LOWERCASE
        }
        0 | _ => {
            vdt = VDT_PRIMARY;
            fst_ver = 1 as libc::c_int as libc::c_uchar;
            vd_ver = fst_ver;
            vdc = VDC_STD
        }
    }
    bp = wb_buffptr(a).offset(-(1 as libc::c_int as isize));
    /* Volume Descriptor Type */
    set_VD_bp(bp, vdt, vd_ver);
    /* Unused Field */
    set_unused_field_bp(bp, 8 as libc::c_int, 8 as libc::c_int);
    /* System Identifier */
    get_system_identitier(
        identifier.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    r = set_str_a_characters_bp(
        a,
        bp,
        9 as libc::c_int,
        40 as libc::c_int,
        identifier.as_mut_ptr(),
        vdc,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Volume Identifier */
    r = set_str_d_characters_bp(
        a,
        bp,
        41 as libc::c_int,
        72 as libc::c_int,
        (*iso9660).volume_identifier.s,
        vdc,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Unused Field */
    set_unused_field_bp(bp, 73 as libc::c_int, 80 as libc::c_int);
    /* Volume Space Size */
    set_num_733(
        bp.offset(81 as libc::c_int as isize),
        (*iso9660).volume_space_size as uint32_t,
    );
    if (*vdd).vdd_type as libc::c_uint == VDD_JOLIET as libc::c_int as libc::c_uint {
        /* Escape Sequences */
        *bp.offset(89 as libc::c_int as isize) = 0x25 as libc::c_int as libc::c_uchar; /* UCS-2 Level 3 */
        *bp.offset(90 as libc::c_int as isize) = 0x2f as libc::c_int as libc::c_uchar;
        *bp.offset(91 as libc::c_int as isize) = 0x45 as libc::c_int as libc::c_uchar;
        memset(
            bp.offset(92 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (120 as libc::c_int - 92 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        );
    } else {
        /* Unused Field */
        set_unused_field_bp(bp, 89 as libc::c_int, 120 as libc::c_int);
    }
    /* Volume Set Size */
    set_num_723(bp.offset(121 as libc::c_int as isize), volume_set_size);
    /* Volume Sequence Number */
    set_num_723(
        bp.offset(125 as libc::c_int as isize),
        (*iso9660).volume_sequence_number as uint16_t,
    );
    /* Logical Block Size */
    set_num_723(
        bp.offset(129 as libc::c_int as isize),
        LOGICAL_BLOCK_SIZE as uint16_t,
    );
    /* Path Table Size */
    set_num_733(
        bp.offset(133 as libc::c_int as isize),
        (*vdd).path_table_size as uint32_t,
    );
    /* Location of Occurrence of Type L Path Table */
    set_num_731(
        bp.offset(141 as libc::c_int as isize),
        (*vdd).location_type_L_path_table as uint32_t,
    );
    /* Location of Optional Occurrence of Type L Path Table */
    set_num_731(
        bp.offset(145 as libc::c_int as isize),
        0 as libc::c_int as uint32_t,
    );
    /* Location of Occurrence of Type M Path Table */
    set_num_732(
        bp.offset(149 as libc::c_int as isize),
        (*vdd).location_type_M_path_table as uint32_t,
    );
    /* Location of Optional Occurrence of Type M Path Table */
    set_num_732(
        bp.offset(153 as libc::c_int as isize),
        0 as libc::c_int as uint32_t,
    );
    /* Directory Record for Root Directory(BP 157 to 190) */
    set_directory_record(
        bp.offset(157 as libc::c_int as isize),
        (190 as libc::c_int - 157 as libc::c_int + 1 as libc::c_int) as size_t,
        (*vdd).rootent,
        iso9660,
        DIR_REC_VD,
        (*vdd).vdd_type,
    );
    /* Volume Set Identifier */
    r = set_str_d_characters_bp(
        a,
        bp,
        191 as libc::c_int,
        318 as libc::c_int,
        b"\x00" as *const u8 as *const libc::c_char,
        vdc,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Publisher Identifier */
    r = set_file_identifier(
        bp,
        319 as libc::c_int,
        446 as libc::c_int,
        vdc,
        a,
        vdd,
        &mut (*iso9660).publisher_identifier,
        b"Publisher File\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        A_CHAR,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Data Preparer Identifier */
    r = set_file_identifier(
        bp,
        447 as libc::c_int,
        574 as libc::c_int,
        vdc,
        a,
        vdd,
        &mut (*iso9660).data_preparer_identifier,
        b"Data Preparer File\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        A_CHAR,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Application Identifier */
    r = set_file_identifier(
        bp,
        575 as libc::c_int,
        702 as libc::c_int,
        vdc,
        a,
        vdd,
        &mut (*iso9660).application_identifier,
        b"Application File\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        A_CHAR,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Copyright File Identifier */
    r = set_file_identifier(
        bp,
        703 as libc::c_int,
        739 as libc::c_int,
        vdc,
        a,
        vdd,
        &mut (*iso9660).copyright_file_identifier,
        b"Copyright File\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        D_CHAR,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Abstract File Identifier */
    r = set_file_identifier(
        bp,
        740 as libc::c_int,
        776 as libc::c_int,
        vdc,
        a,
        vdd,
        &mut (*iso9660).abstract_file_identifier,
        b"Abstract File\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        D_CHAR,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Bibliographic File Identifier */
    r = set_file_identifier(
        bp,
        777 as libc::c_int,
        813 as libc::c_int,
        vdc,
        a,
        vdd,
        &mut (*iso9660).bibliographic_file_identifier,
        b"Bibliongraphic File\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        D_CHAR,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    /* Volume Creation Date and Time */
    set_date_time(
        bp.offset(814 as libc::c_int as isize),
        (*iso9660).birth_time,
    );
    /* Volume Modification Date and Time */
    set_date_time(
        bp.offset(831 as libc::c_int as isize),
        (*iso9660).birth_time,
    );
    /* Volume Expiration Date and Time(obsolete) */
    set_date_time_null(bp.offset(848 as libc::c_int as isize));
    /* Volume Effective Date and Time */
    set_date_time(
        bp.offset(865 as libc::c_int as isize),
        (*iso9660).birth_time,
    );
    /* File Structure Version */
    *bp.offset(882 as libc::c_int as isize) = fst_ver;
    /* Reserved */
    *bp.offset(883 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    /* Application Use */
    memset(
        bp.offset(884 as libc::c_int as isize) as *mut libc::c_void,
        0x20 as libc::c_int,
        (1395 as libc::c_int - 884 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
    );
    /* Reserved */
    set_unused_field_bp(bp, 1396 as libc::c_int, LOGICAL_BLOCK_SIZE);
    return wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
}
/*
 * Write Boot Record Volume Descriptor
 */
unsafe extern "C" fn write_VD_boot_record(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    iso9660 = (*a).format_data as *mut iso9660;
    bp = wb_buffptr(a).offset(-(1 as libc::c_int as isize));
    /* Volume Descriptor Type */
    set_VD_bp(bp, VDT_BOOT_RECORD, 1 as libc::c_int as libc::c_uchar);
    /* Boot System Identifier */
    memcpy(
        bp.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        b"EL TORITO SPECIFICATION\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        23 as libc::c_int as libc::c_ulong,
    );
    set_unused_field_bp(bp, 8 as libc::c_int + 23 as libc::c_int, 39 as libc::c_int);
    /* Unused */
    set_unused_field_bp(bp, 40 as libc::c_int, 71 as libc::c_int);
    /* Absolute pointer to first sector of Boot Catalog */
    set_num_731(
        bp.offset(72 as libc::c_int as isize),
        (*(*(*iso9660).el_torito.catalog).file).content.location,
    );
    /* Unused */
    set_unused_field_bp(bp, 76 as libc::c_int, LOGICAL_BLOCK_SIZE);
    return wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
}
unsafe extern "C" fn set_option_info(
    mut info: *mut archive_string,
    mut opt: *mut libc::c_int,
    mut key: *const libc::c_char,
    mut type_0: keytype,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut prefix: libc::c_char = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut d: libc::c_int = 0;
    prefix = if *opt == 0 as libc::c_int {
        ' ' as i32
    } else {
        ',' as i32
    } as libc::c_char;
    ap = args.clone();
    match type_0 as libc::c_uint {
        0 => {
            d = ap.arg::<libc::c_int>();
            archive_string_sprintf(
                info,
                b"%c%s%s\x00" as *const u8 as *const libc::c_char,
                prefix as libc::c_int,
                if d == 0 as libc::c_int {
                    b"!\x00" as *const u8 as *const libc::c_char
                } else {
                    b"\x00" as *const u8 as *const libc::c_char
                },
                key,
            );
        }
        1 => {
            s = ap.arg::<*const libc::c_char>();
            archive_string_sprintf(
                info,
                b"%c%s=%s\x00" as *const u8 as *const libc::c_char,
                prefix as libc::c_int,
                key,
                s,
            );
        }
        2 => {
            d = ap.arg::<libc::c_int>();
            archive_string_sprintf(
                info,
                b"%c%s=%d\x00" as *const u8 as *const libc::c_char,
                prefix as libc::c_int,
                key,
                d,
            );
        }
        3 => {
            d = ap.arg::<libc::c_int>();
            archive_string_sprintf(
                info,
                b"%c%s=%x\x00" as *const u8 as *const libc::c_char,
                prefix as libc::c_int,
                key,
                d,
            );
        }
        _ => {}
    }
    *opt = 1 as libc::c_int;
}
/*
 * Make Non-ISO File System Information
 */
unsafe extern "C" fn write_information_block(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660; /* root directory */
    let mut buf: [libc::c_char; 128] = [0; 128]; /* Padding Field */
    let mut v: *const libc::c_char = 0 as *const libc::c_char; /* root directory */
    let mut opt: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut info: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut info_size: size_t =
        (LOGICAL_BLOCK_SIZE * NON_ISO_FILE_SYSTEM_INFORMATION_BLOCK) as size_t;
    iso9660 = (*a).format_data as *mut iso9660;
    if info_size > (*((*a).format_data as *mut iso9660)).wbuff_remaining {
        r = wb_write_out(a);
        if r != ARCHIVE_OK {
            return r;
        }
    }
    info.s = NULL as *mut libc::c_char;
    info.length = 0 as libc::c_int as size_t;
    info.buffer_length = 0 as libc::c_int as size_t;
    if archive_string_ensure(&mut info, info_size).is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    memset(info.s as *mut libc::c_void, 0 as libc::c_int, info_size);
    opt = 0 as libc::c_int;
    ctime_r(&mut (*iso9660).birth_time, buf.as_mut_ptr());
    archive_string_sprintf(
        &mut info as *mut archive_string,
        b"INFO %s%s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        archive_version_string(),
    );
    if (*iso9660).opt.abstract_file() as libc::c_int != OPT_ABSTRACT_FILE_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"abstract-file\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).abstract_file_identifier.s,
        );
    }
    if (*iso9660).opt.application_id() as libc::c_int != OPT_APPLICATION_ID_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"application-id\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).application_identifier.s,
        );
    }
    if (*iso9660).opt.allow_vernum() as libc::c_int != OPT_ALLOW_VERNUM_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"allow-vernum\x00" as *const u8 as *const libc::c_char,
            KEY_FLG,
            (*iso9660).opt.allow_vernum() as libc::c_int,
        );
    }
    if (*iso9660).opt.biblio_file() as libc::c_int != OPT_BIBLIO_FILE_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"biblio-file\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).bibliographic_file_identifier.s,
        );
    }
    if (*iso9660).opt.boot() as libc::c_int != OPT_BOOT_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"boot\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).el_torito.boot_filename.s,
        );
    }
    if (*iso9660).opt.boot_catalog() as libc::c_int != OPT_BOOT_CATALOG_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"boot-catalog\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).el_torito.catalog_filename.s,
        );
    }
    if (*iso9660).opt.boot_info_table() as libc::c_int != OPT_BOOT_INFO_TABLE_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"boot-info-table\x00" as *const u8 as *const libc::c_char,
            KEY_FLG,
            (*iso9660).opt.boot_info_table() as libc::c_int,
        );
    }
    if (*iso9660).opt.boot_load_seg() as libc::c_int != OPT_BOOT_LOAD_SEG_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"boot-load-seg\x00" as *const u8 as *const libc::c_char,
            KEY_HEX,
            (*iso9660).el_torito.boot_load_seg as libc::c_int,
        );
    }
    if (*iso9660).opt.boot_load_size() as libc::c_int != OPT_BOOT_LOAD_SIZE_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"boot-load-size\x00" as *const u8 as *const libc::c_char,
            KEY_INT,
            (*iso9660).el_torito.boot_load_size as libc::c_int,
        );
    }
    if (*iso9660).opt.boot_type() as libc::c_int != OPT_BOOT_TYPE_DEFAULT {
        v = b"no-emulation\x00" as *const u8 as *const libc::c_char;
        if (*iso9660).opt.boot_type() as libc::c_int == OPT_BOOT_TYPE_FD {
            v = b"fd\x00" as *const u8 as *const libc::c_char
        }
        if (*iso9660).opt.boot_type() as libc::c_int == OPT_BOOT_TYPE_HARD_DISK {
            v = b"hard-disk\x00" as *const u8 as *const libc::c_char
        }
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"boot-type\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            v,
        );
    }
    if (*iso9660).opt.compression_level() as libc::c_int != OPT_COMPRESSION_LEVEL_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"compression-level\x00" as *const u8 as *const libc::c_char,
            KEY_INT,
            (*iso9660).zisofs.compression_level,
        );
    }
    if (*iso9660).opt.copyright_file() as libc::c_int != OPT_COPYRIGHT_FILE_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"copyright-file\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).copyright_file_identifier.s,
        );
    }
    if (*iso9660).opt.iso_level() as libc::c_int != OPT_ISO_LEVEL_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"iso-level\x00" as *const u8 as *const libc::c_char,
            KEY_INT,
            (*iso9660).opt.iso_level() as libc::c_int,
        );
    }
    if (*iso9660).opt.joliet() as libc::c_int != OPT_JOLIET_DEFAULT {
        if (*iso9660).opt.joliet() as libc::c_int == OPT_JOLIET_LONGNAME {
            set_option_info(
                &mut info as *mut archive_string,
                &mut opt as *mut libc::c_int,
                b"joliet\x00" as *const u8 as *const libc::c_char,
                KEY_STR,
                b"long\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            set_option_info(
                &mut info as *mut archive_string,
                &mut opt as *mut libc::c_int,
                b"joliet\x00" as *const u8 as *const libc::c_char,
                KEY_FLG,
                (*iso9660).opt.joliet() as libc::c_int,
            );
        }
    }
    if (*iso9660).opt.limit_depth() as libc::c_int != OPT_LIMIT_DEPTH_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"limit-depth\x00" as *const u8 as *const libc::c_char,
            KEY_FLG,
            (*iso9660).opt.limit_depth() as libc::c_int,
        );
    }
    if (*iso9660).opt.limit_dirs() as libc::c_int != OPT_LIMIT_DIRS_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"limit-dirs\x00" as *const u8 as *const libc::c_char,
            KEY_FLG,
            (*iso9660).opt.limit_dirs() as libc::c_int,
        );
    }
    if (*iso9660).opt.pad() as libc::c_int != OPT_PAD_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"pad\x00" as *const u8 as *const libc::c_char,
            KEY_FLG,
            (*iso9660).opt.pad() as libc::c_int,
        );
    }
    if (*iso9660).opt.publisher() as libc::c_int != OPT_PUBLISHER_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"publisher\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).publisher_identifier.s,
        );
    }
    if (*iso9660).opt.rr() as libc::c_int != OPT_RR_DEFAULT {
        if (*iso9660).opt.rr() as libc::c_int == OPT_RR_DISABLED {
            set_option_info(
                &mut info as *mut archive_string,
                &mut opt as *mut libc::c_int,
                b"rockridge\x00" as *const u8 as *const libc::c_char,
                KEY_FLG,
                (*iso9660).opt.rr() as libc::c_int,
            );
        } else if (*iso9660).opt.rr() as libc::c_int == OPT_RR_STRICT {
            set_option_info(
                &mut info as *mut archive_string,
                &mut opt as *mut libc::c_int,
                b"rockridge\x00" as *const u8 as *const libc::c_char,
                KEY_STR,
                b"strict\x00" as *const u8 as *const libc::c_char,
            );
        } else if (*iso9660).opt.rr() as libc::c_int == OPT_RR_USEFUL {
            set_option_info(
                &mut info as *mut archive_string,
                &mut opt as *mut libc::c_int,
                b"rockridge\x00" as *const u8 as *const libc::c_char,
                KEY_STR,
                b"useful\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*iso9660).opt.volume_id() as libc::c_int != OPT_VOLUME_ID_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"volume-id\x00" as *const u8 as *const libc::c_char,
            KEY_STR,
            (*iso9660).volume_identifier.s,
        );
    }
    if (*iso9660).opt.zisofs() as libc::c_int != OPT_ZISOFS_DEFAULT {
        set_option_info(
            &mut info as *mut archive_string,
            &mut opt as *mut libc::c_int,
            b"zisofs\x00" as *const u8 as *const libc::c_char,
            KEY_FLG,
            (*iso9660).opt.zisofs() as libc::c_int,
        );
    }
    memcpy(
        wb_buffptr(a) as *mut libc::c_void,
        info.s as *const libc::c_void,
        info_size,
    );
    archive_string_free(&mut info);
    return wb_consume(a, info_size);
}
unsafe extern "C" fn write_rr_ER(mut a: *mut archive_write) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = wb_buffptr(a);
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        LOGICAL_BLOCK_SIZE as libc::c_ulong,
    );
    *p.offset(0 as libc::c_int as isize) = 'E' as i32 as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) = 'R' as i32 as libc::c_uchar;
    *p.offset(3 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) = (8 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 85]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 136]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_uchar;
    *p.offset(4 as libc::c_int as isize) =
        (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *p.offset(5 as libc::c_int as isize) =
        (::std::mem::size_of::<[libc::c_char; 85]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *p.offset(6 as libc::c_int as isize) =
        (::std::mem::size_of::<[libc::c_char; 136]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *p.offset(7 as libc::c_int as isize) = 0x1 as libc::c_int as libc::c_uchar;
    memcpy(
        &mut *p.offset(8 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        rrip_identifier.as_ptr() as *const libc::c_void,
        *p.offset(4 as libc::c_int as isize) as libc::c_ulong,
    );
    memcpy(
        &mut *p.offset(
            (8 as libc::c_int + *p.offset(4 as libc::c_int as isize) as libc::c_int) as isize,
        ) as *mut libc::c_uchar as *mut libc::c_void,
        rrip_descriptor.as_ptr() as *const libc::c_void,
        *p.offset(5 as libc::c_int as isize) as libc::c_ulong,
    );
    memcpy(
        &mut *p.offset(
            (8 as libc::c_int
                + *p.offset(4 as libc::c_int as isize) as libc::c_int
                + *p.offset(5 as libc::c_int as isize) as libc::c_int) as isize,
        ) as *mut libc::c_uchar as *mut libc::c_void,
        rrip_source.as_ptr() as *const libc::c_void,
        *p.offset(6 as libc::c_int as isize) as libc::c_ulong,
    );
    return wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
}
unsafe extern "C" fn calculate_path_table_size(mut vdd: *mut vdd) {
    let mut depth: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut pt: *mut path_table = 0 as *mut path_table;
    pt = (*vdd).pathtbl;
    size = 0 as libc::c_int;
    depth = 0 as libc::c_int;
    while depth < (*vdd).max_depth {
        let mut ptbl: *mut *mut isoent = 0 as *mut *mut isoent;
        let mut i: libc::c_int = 0;
        let mut cnt: libc::c_int = 0;
        cnt = (*pt.offset(depth as isize)).cnt;
        if cnt == 0 as libc::c_int {
            break;
        }
        ptbl = (*pt.offset(depth as isize)).sorted;
        i = 0 as libc::c_int;
        while i < cnt {
            let mut len: libc::c_int = 0;
            if (**ptbl.offset(i as isize)).identifier.is_null() {
                len = 1 as libc::c_int
            } else {
                len = (**ptbl.offset(i as isize)).id_len
            }
            if len & 0x1 as libc::c_int != 0 {
                len += 1
            }
            size += 8 as libc::c_int + len;
            i += 1
        }
        depth += 1
    }
    (*vdd).path_table_size = size;
    (*vdd).path_table_block = (size + PATH_TABLE_BLOCK_SIZE - 1 as libc::c_int)
        / PATH_TABLE_BLOCK_SIZE
        * (PATH_TABLE_BLOCK_SIZE / LOGICAL_BLOCK_SIZE);
}
unsafe extern "C" fn _write_path_table(
    mut a: *mut archive_write,
    mut type_m: libc::c_int,
    mut depth: libc::c_int,
    mut vdd: *mut vdd,
) -> libc::c_int {
    let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ptbl: *mut *mut isoent = 0 as *mut *mut isoent;
    let mut wbremaining: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut wsize: libc::c_int = 0;
    if (*(*vdd).pathtbl.offset(depth as isize)).cnt == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    wsize = 0 as libc::c_int;
    wb = wb_buffptr(a);
    wbremaining = (*((*a).format_data as *mut iso9660)).wbuff_remaining;
    bp = wb.offset(-(1 as libc::c_int as isize));
    ptbl = (*(*vdd).pathtbl.offset(depth as isize)).sorted;
    i = 0 as libc::c_int;
    while i < (*(*vdd).pathtbl.offset(depth as isize)).cnt {
        let mut np: *mut isoent = 0 as *mut isoent;
        let mut len: size_t = 0;
        np = *ptbl.offset(i as isize);
        if (*np).identifier.is_null() {
            len = 1 as libc::c_int as size_t
        } else {
            len = (*np).id_len as size_t
        }
        if wbremaining.wrapping_sub(
            bp.offset(1 as libc::c_int as isize)
                .offset_from(wb) as libc::c_long as libc::c_ulong,
        ) < len
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(8 as libc::c_int as libc::c_ulong)
        {
            r = wb_consume(
                a,
                bp.offset(1 as libc::c_int as isize)
                    .offset_from(wb) as libc::c_long as size_t,
            );
            if r < 0 as libc::c_int {
                return r;
            }
            wb = wb_buffptr(a);
            wbremaining = (*((*a).format_data as *mut iso9660)).wbuff_remaining;
            bp = wb.offset(-(1 as libc::c_int as isize))
        }
        /* Length of Directory Identifier */
        set_num_711(bp.offset(1 as libc::c_int as isize), len as libc::c_uchar);
        /* Extended Attribute Record Length */
        set_num_711(
            bp.offset(2 as libc::c_int as isize),
            0 as libc::c_int as libc::c_uchar,
        );
        /* Location of Extent */
        if type_m != 0 {
            set_num_732(bp.offset(3 as libc::c_int as isize), (*np).dir_location);
        } else {
            set_num_731(bp.offset(3 as libc::c_int as isize), (*np).dir_location);
        }
        /* Parent Directory Number */
        if type_m != 0 {
            set_num_722(
                bp.offset(7 as libc::c_int as isize),
                (*(*np).parent).dir_number as uint16_t,
            );
        } else {
            set_num_721(
                bp.offset(7 as libc::c_int as isize),
                (*(*np).parent).dir_number as uint16_t,
            );
        }
        /* Directory Identifier */
        if (*np).identifier.is_null() {
            *bp.offset(9 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar
        } else {
            memcpy(
                &mut *bp.offset(9 as libc::c_int as isize) as *mut libc::c_uchar
                    as *mut libc::c_void,
                (*np).identifier as *const libc::c_void,
                len,
            );
        }
        if len & 0x1 as libc::c_int as libc::c_ulong != 0 {
            /* Padding Field */
            *bp.offset((9 as libc::c_int as libc::c_ulong).wrapping_add(len) as isize) =
                0 as libc::c_int as libc::c_uchar;
            len = len.wrapping_add(1)
        }
        wsize += 8 as libc::c_int + len as libc::c_int;
        bp = bp.offset((8 as libc::c_int as libc::c_ulong).wrapping_add(len) as isize);
        i += 1
    }
    if bp.offset(1 as libc::c_int as isize) > wb {
        r = wb_consume(
            a,
            bp.offset(1 as libc::c_int as isize)
                .offset_from(wb) as libc::c_long as size_t,
        );
        if r < 0 as libc::c_int {
            return r;
        }
    }
    return wsize;
}
unsafe extern "C" fn write_path_table(
    mut a: *mut archive_write,
    mut type_m: libc::c_int,
    mut vdd: *mut vdd,
) -> libc::c_int {
    let mut depth: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut path_table_size: size_t = 0;
    r = ARCHIVE_OK;
    path_table_size = 0 as libc::c_int as size_t;
    depth = 0 as libc::c_int;
    while depth < (*vdd).max_depth {
        r = _write_path_table(a, type_m, depth, vdd);
        if r < 0 as libc::c_int {
            return r;
        }
        path_table_size =
            (path_table_size as libc::c_ulong).wrapping_add(r as libc::c_ulong) as size_t as size_t;
        depth += 1
    }
    /* Write padding data. */
    path_table_size = path_table_size.wrapping_rem(PATH_TABLE_BLOCK_SIZE as libc::c_ulong);
    if path_table_size > 0 as libc::c_int as libc::c_ulong {
        r = write_null(
            a,
            (PATH_TABLE_BLOCK_SIZE as libc::c_ulong).wrapping_sub(path_table_size),
        )
    }
    return r;
}
unsafe extern "C" fn calculate_directory_descriptors(
    mut iso9660: *mut iso9660,
    mut vdd: *mut vdd,
    mut isoent: *mut isoent,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut enttbl: *mut *mut isoent = 0 as *mut *mut isoent;
    let mut bs: libc::c_int = 0;
    let mut block: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    block = 1 as libc::c_int;
    bs = get_dir_rec_size(iso9660, isoent, DIR_REC_SELF, (*vdd).vdd_type);
    bs += get_dir_rec_size(iso9660, isoent, DIR_REC_PARENT, (*vdd).vdd_type);
    if (*isoent).children.cnt <= 0 as libc::c_int
        || (*vdd).vdd_type as libc::c_uint != VDD_JOLIET as libc::c_int as libc::c_uint
            && (*iso9660).opt.rr() == 0
            && depth + 1 as libc::c_int >= (*vdd).max_depth
    {
        return block;
    }
    enttbl = (*isoent).children_sorted;
    i = 0 as libc::c_int;
    while i < (*isoent).children.cnt {
        let mut np: *mut isoent = *enttbl.offset(i as isize);
        let mut file: *mut isofile = 0 as *mut isofile;
        file = (*np).file;
        if !(*file).hardlink_target.is_null() {
            file = (*file).hardlink_target
        }
        (*file).cur_content = &mut (*file).content;
        loop {
            let mut dr_l: libc::c_int = 0;
            dr_l = get_dir_rec_size(iso9660, np, DIR_REC_NORMAL, (*vdd).vdd_type);
            if bs + dr_l > LOGICAL_BLOCK_SIZE {
                block += 1;
                bs = dr_l
            } else {
                bs += dr_l
            }
            (*file).cur_content = (*(*file).cur_content).next;
            if (*file).cur_content.is_null() {
                break;
            }
        }
        i += 1
    }
    return block;
}
unsafe extern "C" fn _write_directory_descriptors(
    mut a: *mut archive_write,
    mut vdd: *mut vdd,
    mut isoent: *mut isoent,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut enttbl: *mut *mut isoent = 0 as *mut *mut isoent;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut dr_l: libc::c_int = 0;
    wb = wb_buffptr(a);
    p = wb;
    p = p.offset(set_directory_record(
        p,
        (LOGICAL_BLOCK_SIZE as libc::c_long - p.offset_from(wb) as libc::c_long) as size_t,
        isoent,
        iso9660,
        DIR_REC_SELF,
        (*vdd).vdd_type,
    ) as isize);
    p = p.offset(set_directory_record(
        p,
        (LOGICAL_BLOCK_SIZE as libc::c_long - p.offset_from(wb) as libc::c_long) as size_t,
        isoent,
        iso9660,
        DIR_REC_PARENT,
        (*vdd).vdd_type,
    ) as isize);
    if (*isoent).children.cnt <= 0 as libc::c_int
        || (*vdd).vdd_type as libc::c_uint != VDD_JOLIET as libc::c_int as libc::c_uint
            && (*iso9660).opt.rr() == 0
            && depth + 1 as libc::c_int >= (*vdd).max_depth
    {
        memset(
            p as *mut libc::c_void,
            0 as libc::c_int,
            (LOGICAL_BLOCK_SIZE as libc::c_long - p.offset_from(wb) as libc::c_long)
                as libc::c_ulong,
        );
        return wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
    }
    enttbl = (*isoent).children_sorted;
    i = 0 as libc::c_int;
    while i < (*isoent).children.cnt {
        let mut np: *mut isoent = *enttbl.offset(i as isize);
        let mut file: *mut isofile = (*np).file;
        if !(*file).hardlink_target.is_null() {
            file = (*file).hardlink_target
        }
        (*file).cur_content = &mut (*file).content;
        loop {
            dr_l = set_directory_record(
                p,
                (LOGICAL_BLOCK_SIZE as libc::c_long - p.offset_from(wb) as libc::c_long)
                    as size_t,
                np,
                iso9660,
                DIR_REC_NORMAL,
                (*vdd).vdd_type,
            );
            if dr_l == 0 as libc::c_int {
                memset(
                    p as *mut libc::c_void,
                    0 as libc::c_int,
                    (LOGICAL_BLOCK_SIZE as libc::c_long
                        - p.offset_from(wb) as libc::c_long)
                        as libc::c_ulong,
                );
                r = wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
                if r < 0 as libc::c_int {
                    return r;
                }
                wb = wb_buffptr(a);
                p = wb;
                dr_l = set_directory_record(
                    p,
                    (LOGICAL_BLOCK_SIZE as libc::c_long
                        - p.offset_from(wb) as libc::c_long) as size_t,
                    np,
                    iso9660,
                    DIR_REC_NORMAL,
                    (*vdd).vdd_type,
                )
            }
            p = p.offset(dr_l as isize);
            (*file).cur_content = (*(*file).cur_content).next;
            if (*file).cur_content.is_null() {
                break;
            }
        }
        i += 1
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        (LOGICAL_BLOCK_SIZE as libc::c_long - p.offset_from(wb) as libc::c_long)
            as libc::c_ulong,
    );
    return wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
}
unsafe extern "C" fn write_directory_descriptors(
    mut a: *mut archive_write,
    mut vdd: *mut vdd,
) -> libc::c_int {
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut depth: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    depth = 0 as libc::c_int;
    np = (*vdd).rootent;
    loop {
        let mut extr: *mut extr_rec = 0 as *mut extr_rec;
        r = _write_directory_descriptors(a, vdd, np, depth);
        if r < 0 as libc::c_int {
            return r;
        }
        if (*vdd).vdd_type as libc::c_uint != VDD_JOLIET as libc::c_int as libc::c_uint {
            /*
             * This extract record is used by SUSP,RRIP.
             * Not for joliet.
             */
            extr = (*np).extr_rec_list.first;
            while !extr.is_null() {
                let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                wb = wb_buffptr(a);
                memcpy(
                    wb as *mut libc::c_void,
                    (*extr).buf.as_mut_ptr() as *const libc::c_void,
                    (*extr).offset as libc::c_ulong,
                );
                memset(
                    wb.offset((*extr).offset as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    (LOGICAL_BLOCK_SIZE - (*extr).offset) as libc::c_ulong,
                );
                r = wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
                if r < 0 as libc::c_int {
                    return r;
                }
                extr = (*extr).next
            }
        }
        if !(*np).subdirs.first.is_null() && (depth + 1 as libc::c_int) < (*vdd).max_depth {
            /* Enter to sub directories. */
            np = (*np).subdirs.first;
            depth += 1
        } else {
            while np != (*np).parent {
                if (*np).drnext.is_null() {
                    /* Return to the parent directory. */
                    np = (*np).parent;
                    depth -= 1
                } else {
                    np = (*np).drnext;
                    break;
                }
            }
        }
        if !(np != (*np).parent) {
            break;
        }
    }
    return 0 as libc::c_int;
}
/*
 * Read file contents from the temporary file, and write it.
 */
unsafe extern "C" fn write_file_contents(
    mut a: *mut archive_write,
    mut offset: int64_t,
    mut size: int64_t,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut r: libc::c_int = 0;
    lseek((*iso9660).temp_fd, offset, SEEK_SET);
    while size != 0 {
        let mut rsize: size_t = 0;
        let mut rs: ssize_t = 0;
        let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        wb = wb_buffptr(a);
        rsize = (*((*a).format_data as *mut iso9660)).wbuff_remaining;
        if rsize > size as size_t {
            rsize = size as size_t
        }
        rs = read((*iso9660).temp_fd, wb as *mut libc::c_void, rsize);
        if rs <= 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t read temporary file(%jd)\x00" as *const u8 as *const libc::c_char,
                rs,
            );
            return -(30 as libc::c_int);
        }
        size -= rs;
        r = wb_consume(a, rs as size_t);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_file_descriptors(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut file: *mut isofile = 0 as *mut isofile;
    let mut blocks: int64_t = 0;
    let mut offset: int64_t = 0;
    let mut r: libc::c_int = 0;
    blocks = 0 as libc::c_int as int64_t;
    offset = 0 as libc::c_int as int64_t;
    /* Make the boot catalog contents, and write it. */
    if !(*iso9660).el_torito.catalog.is_null() {
        r = make_boot_catalog(a);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /* Write the boot file contents. */
    if !(*iso9660).el_torito.boot.is_null() {
        file = (*(*iso9660).el_torito.boot).file;
        blocks = (*file).content.blocks as int64_t;
        offset = (*file).content.offset_of_temp;
        if offset != 0 as libc::c_int as libc::c_long {
            r = write_file_contents(a, offset, blocks << LOGICAL_BLOCK_BITS);
            if r < 0 as libc::c_int {
                return r;
            }
            blocks = 0 as libc::c_int as int64_t;
            offset = 0 as libc::c_int as int64_t
        }
    }
    /* Write out all file contents. */
    file = (*iso9660).data_file_list.first;
    while !file.is_null() {
        if !((*file).write_content == 0) {
            if offset + (blocks << LOGICAL_BLOCK_BITS) < (*file).content.offset_of_temp {
                if blocks > 0 as libc::c_int as libc::c_long {
                    r = write_file_contents(a, offset, blocks << LOGICAL_BLOCK_BITS);
                    if r < 0 as libc::c_int {
                        return r;
                    }
                }
                blocks = 0 as libc::c_int as int64_t;
                offset = (*file).content.offset_of_temp
            }
            (*file).cur_content = &mut (*file).content;
            loop {
                blocks += (*(*file).cur_content).blocks as libc::c_long;
                /* Next fragment */
                (*file).cur_content = (*(*file).cur_content).next;
                if (*file).cur_content.is_null() {
                    break;
                }
            }
        }
        file = (*file).datanext
    }
    /* Flush out remaining blocks. */
    if blocks > 0 as libc::c_int as libc::c_long {
        r = write_file_contents(a, offset, blocks << LOGICAL_BLOCK_BITS);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn isofile_init_entry_list(mut iso9660: *mut iso9660) {
    (*iso9660).all_file_list.first = NULL as *mut isofile;
    (*iso9660).all_file_list.last = &mut (*iso9660).all_file_list.first;
}
unsafe extern "C" fn isofile_add_entry(mut iso9660: *mut iso9660, mut file: *mut isofile) {
    (*file).allnext = NULL as *mut isofile;
    *(*iso9660).all_file_list.last = file;
    (*iso9660).all_file_list.last = &mut (*file).allnext;
}
unsafe extern "C" fn isofile_free_all_entries(mut iso9660: *mut iso9660) {
    let mut file: *mut isofile = 0 as *mut isofile;
    let mut file_next: *mut isofile = 0 as *mut isofile;
    file = (*iso9660).all_file_list.first;
    while !file.is_null() {
        file_next = (*file).allnext;
        isofile_free(file);
        file = file_next
    }
}
unsafe extern "C" fn isofile_init_entry_data_file_list(mut iso9660: *mut iso9660) {
    (*iso9660).data_file_list.first = NULL as *mut isofile;
    (*iso9660).data_file_list.last = &mut (*iso9660).data_file_list.first;
}
unsafe extern "C" fn isofile_add_data_file(mut iso9660: *mut iso9660, mut file: *mut isofile) {
    (*file).datanext = NULL as *mut isofile;
    *(*iso9660).data_file_list.last = file;
    (*iso9660).data_file_list.last = &mut (*file).datanext;
}
unsafe extern "C" fn isofile_new(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> *mut isofile {
    let mut file: *mut isofile = 0 as *mut isofile;
    file = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<isofile>() as libc::c_ulong,
    ) as *mut isofile;
    if file.is_null() {
        return 0 as *mut isofile;
    }
    if !entry.is_null() {
        (*file).entry = archive_entry_clone(entry)
    } else {
        (*file).entry = archive_entry_new2(&mut (*a).archive)
    }
    if (*file).entry.is_null() {
        free(file as *mut libc::c_void);
        return 0 as *mut isofile;
    }
    (*file).parentdir.s = NULL as *mut libc::c_char;
    (*file).parentdir.length = 0 as libc::c_int as size_t;
    (*file).parentdir.buffer_length = 0 as libc::c_int as size_t;
    (*file).basename.s = NULL as *mut libc::c_char;
    (*file).basename.length = 0 as libc::c_int as size_t;
    (*file).basename.buffer_length = 0 as libc::c_int as size_t;
    (*file).basename_utf16.s = NULL as *mut libc::c_char;
    (*file).basename_utf16.length = 0 as libc::c_int as size_t;
    (*file).basename_utf16.buffer_length = 0 as libc::c_int as size_t;
    (*file).symlink.s = NULL as *mut libc::c_char;
    (*file).symlink.length = 0 as libc::c_int as size_t;
    (*file).symlink.buffer_length = 0 as libc::c_int as size_t;
    (*file).cur_content = &mut (*file).content;
    return file;
}
unsafe extern "C" fn isofile_free(mut file: *mut isofile) {
    let mut con: *mut content = 0 as *mut content;
    let mut tmp: *mut content = 0 as *mut content;
    con = (*file).content.next;
    while !con.is_null() {
        tmp = con;
        con = (*con).next;
        free(tmp as *mut libc::c_void);
    }
    archive_entry_free((*file).entry);
    archive_string_free(&mut (*file).parentdir);
    archive_string_free(&mut (*file).basename);
    archive_string_free(&mut (*file).basename_utf16);
    archive_string_free(&mut (*file).symlink);
    free(file as *mut libc::c_void);
}
/*
 * Generate a parent directory name and a base name from a pathname.
 */
unsafe extern "C" fn isofile_gen_utility_names(
    mut a: *mut archive_write,
    mut file: *mut isofile,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660;
    let mut pathname: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    iso9660 = (*a).format_data as *mut iso9660;
    (*file).parentdir.length = 0 as libc::c_int as size_t;
    (*file).basename.length = 0 as libc::c_int as size_t;
    (*file).basename_utf16.length = 0 as libc::c_int as size_t;
    (*file).symlink.length = 0 as libc::c_int as size_t;
    pathname = archive_entry_pathname((*file).entry);
    if pathname.is_null()
        || *pathname.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        /* virtual root */
        (*file).dircnt = 0 as libc::c_int;
        return ret;
    }
    /*
     * Make a UTF-16BE basename if Joliet extension enabled.
     */
    if (*iso9660).opt.joliet() != 0 {
        let mut u16: *const libc::c_char = 0 as *const libc::c_char;
        let mut ulast: *const libc::c_char = 0 as *const libc::c_char;
        let mut u16len: size_t = 0;
        let mut ulen_last: size_t = 0;
        if (*iso9660).sconv_to_utf16be.is_null() {
            (*iso9660).sconv_to_utf16be = archive_string_conversion_to_charset(
                &mut (*a).archive,
                b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*iso9660).sconv_to_utf16be.is_null() {
                /* Couldn't allocate memory */
                return -(30 as libc::c_int);
            }
            (*iso9660).sconv_from_utf16be = archive_string_conversion_from_charset(
                &mut (*a).archive,
                b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*iso9660).sconv_from_utf16be.is_null() {
                /* Couldn't allocate memory */
                return -(30 as libc::c_int);
            }
        }
        /*
         * Convert a filename to UTF-16BE.
         */
        if 0 as libc::c_int
            > _archive_entry_pathname_l(
                (*file).entry,
                &mut u16,
                &mut u16len,
                (*iso9660).sconv_to_utf16be,
            )
        {
            if errno == ENOMEM {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate memory for UTF-16BE\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            archive_set_error(&mut (*a).archive as *mut archive,
                              ARCHIVE_ERRNO_MISC,
                              b"A filename cannot be converted to UTF-16BE;You should disable making Joliet extension\x00"
                                  as *const u8 as *const libc::c_char);
            ret = ARCHIVE_WARN
        }
        /*
         * Make sure a path separator is not in the last;
         * Remove trailing '/'.
         */
        while u16len >= 2 as libc::c_int as libc::c_ulong {
            if !(*u16.offset(u16len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == 0 as libc::c_int
                && *u16.offset(u16len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '/' as i32)
            {
                break;
            }
            u16len = (u16len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t
        }
        /*
         * Find a basename in UTF-16BE.
         */
        ulast = u16;
        u16len >>= 1 as libc::c_int;
        ulen_last = u16len;
        while u16len > 0 as libc::c_int as libc::c_ulong {
            if *u16.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                && *u16.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                ulast = u16.offset(2 as libc::c_int as isize);
                ulen_last = u16len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            }
            u16 = u16.offset(2 as libc::c_int as isize);
            u16len = u16len.wrapping_sub(1)
        }
        ulen_last <<= 1 as libc::c_int;
        if archive_string_ensure(&mut (*file).basename_utf16, ulen_last).is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for UTF-16BE\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /*
         * Set UTF-16BE basename.
         */
        memcpy(
            (*file).basename_utf16.s as *mut libc::c_void,
            ulast as *const libc::c_void,
            ulen_last,
        );
        (*file).basename_utf16.length = ulen_last
    }
    (*file).parentdir.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*file).parentdir,
        pathname as *const libc::c_void,
        (if pathname.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(pathname)
        }),
    );
    len = (*file).parentdir.length;
    dirname = (*file).parentdir.s;
    p = dirname;
    /*
     * Remove leading '/', '../' and './' elements
     */
    while *p != 0 {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            p = p.offset(1);
            len = len.wrapping_sub(1)
        } else {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 {
                break;
            }
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                p = p.offset(3 as libc::c_int as isize);
                len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                p = p.offset(2 as libc::c_int as isize);
                len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            } else {
                if !(*p.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32) {
                    break;
                }
                p = p.offset(1);
                len = len.wrapping_sub(1)
            }
        }
    }
    if p != dirname {
        memmove(
            dirname as *mut libc::c_void,
            p as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        p = dirname
    }
    /*
     * Remove "/","/." and "/.." elements from tail.
     */
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut ll: size_t = len;
        if len > 0 as libc::c_int as libc::c_ulong
            && *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '/' as i32
        {
            *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char;
            len = len.wrapping_sub(1)
        }
        if len > 1 as libc::c_int as libc::c_ulong
            && *p.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '/' as i32
            && *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '.' as i32
        {
            *p.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char;
            len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        if len > 2 as libc::c_int as libc::c_ulong
            && *p.offset(len.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '/' as i32
            && *p.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '.' as i32
            && *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '.' as i32
        {
            *p.offset(len.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char;
            len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        if ll == len {
            break;
        }
    }
    while *p != 0 {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                /* Convert '//' --> '/' */
                memmove(
                    p as *mut libc::c_void,
                    p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    strlen(p.offset(1 as libc::c_int as isize))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                /* Convert '/./' --> '/' */
                memmove(
                    p as *mut libc::c_void,
                    p.offset(2 as libc::c_int as isize) as *const libc::c_void,
                    strlen(p.offset(2 as libc::c_int as isize))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                /* Convert 'dir/dir1/../dir2/'
                 *     --> 'dir/dir2/'
                 */
                let mut rp: *mut libc::c_char = p.offset(-(1 as libc::c_int as isize));
                while rp >= dirname {
                    if *rp as libc::c_int == '/' as i32 {
                        break;
                    }
                    rp = rp.offset(-1)
                }
                if rp > dirname {
                    strcpy(rp, p.offset(3 as libc::c_int as isize));
                    p = rp
                } else {
                    strcpy(dirname, p.offset(4 as libc::c_int as isize));
                    p = dirname
                }
            } else {
                p = p.offset(1)
            }
        } else {
            p = p.offset(1)
        }
    }
    p = dirname;
    len = strlen(p);
    if archive_entry_filetype((*file).entry) == AE_IFLNK as mode_t {
        /* Convert symlink name too. */
        pathname = archive_entry_symlink((*file).entry);
        (*file).symlink.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*file).symlink,
            pathname as *const libc::c_void,
            (if pathname.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(pathname)
            }),
        );
    }
    /*
     * - Count up directory elements.
     * - Find out the position which points the last position of
     *   path separator('/').
     */
    slash = NULL as *mut libc::c_char;
    (*file).dircnt = 0 as libc::c_int;
    while *p as libc::c_int != '\u{0}' as i32 {
        if *p as libc::c_int == '/' as i32 {
            slash = p;
            (*file).dircnt += 1
        }
        p = p.offset(1)
    }
    if slash.is_null() {
        /* The pathname doesn't have a parent directory. */
        (*file).parentdir.length = len;
        (*file).basename.length = 0 as libc::c_int as size_t;
        archive_string_concat(&mut (*file).basename, &mut (*file).parentdir);
        (*file).parentdir.length = 0 as libc::c_int as size_t;
        *(*file).parentdir.s = '\u{0}' as i32 as libc::c_char;
        return ret;
    }
    /* Make a basename from dirname and slash */
    *slash = '\u{0}' as i32 as libc::c_char;
    (*file).parentdir.length = slash.offset_from(dirname) as libc::c_long as size_t;
    (*file).basename.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*file).basename,
        slash.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (if slash.offset(1 as libc::c_int as isize).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(slash.offset(1 as libc::c_int as isize))
        }),
    );
    if archive_entry_filetype((*file).entry) == AE_IFDIR as mode_t {
        (*file).dircnt += 1
    }
    return ret;
}
/*
 * Register a entry to get a hardlink target.
 */
unsafe extern "C" fn isofile_register_hardlink(
    mut a: *mut archive_write,
    mut file: *mut isofile,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut hl: *mut hardlink = 0 as *mut hardlink;
    let mut pathname: *const libc::c_char = 0 as *const libc::c_char;
    archive_entry_set_nlink((*file).entry, 1 as libc::c_int as libc::c_uint);
    pathname = archive_entry_hardlink((*file).entry);
    if pathname.is_null() {
        /* This `file` is a hardlink target. */
        hl = malloc(::std::mem::size_of::<hardlink>() as libc::c_ulong) as *mut hardlink;
        if hl.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*hl).nlink = 1 as libc::c_int;
        /* A hardlink target must be the first position. */
        (*file).hlnext = NULL as *mut isofile;
        (*hl).file_list.first = file;
        (*hl).file_list.last = &mut (*file).hlnext;
        __archive_rb_tree_insert_node(&mut (*iso9660).hardlink_rbtree, hl as *mut archive_rb_node);
    } else {
        hl = __archive_rb_tree_find_node(
            &mut (*iso9660).hardlink_rbtree,
            pathname as *const libc::c_void,
        ) as *mut hardlink;
        if !hl.is_null() {
            /* Insert `file` entry into the tail. */
            (*file).hlnext = NULL as *mut isofile;
            *(*hl).file_list.last = file;
            (*hl).file_list.last = &mut (*file).hlnext;
            (*hl).nlink += 1
        }
        archive_entry_unset_size((*file).entry);
    }
    return 0 as libc::c_int;
}
/*
 * Hardlinked files have to have the same location of extent.
 * We have to find out hardlink target entries for the entries
 * which have a hardlink target name.
 */
unsafe extern "C" fn isofile_connect_hardlink_files(mut iso9660: *mut iso9660) {
    let mut n: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut hl: *mut hardlink = 0 as *mut hardlink;
    let mut target: *mut isofile = 0 as *mut isofile;
    let mut nf: *mut isofile = 0 as *mut isofile;
    n = __archive_rb_tree_iterate(
        &mut (*iso9660).hardlink_rbtree,
        NULL as *mut archive_rb_node,
        ARCHIVE_RB_DIR_LEFT as libc::c_uint,
    );
    while !n.is_null() {
        hl = n as *mut hardlink;
        /* The first entry must be a hardlink target. */
        target = (*hl).file_list.first;
        archive_entry_set_nlink((*target).entry, (*hl).nlink as libc::c_uint);
        /* Set a hardlink target to reference entries. */
        nf = (*target).hlnext;
        while !nf.is_null() {
            (*nf).hardlink_target = target;
            archive_entry_set_nlink((*nf).entry, (*hl).nlink as libc::c_uint);
            nf = (*nf).hlnext
        }
        n = __archive_rb_tree_iterate(
            &mut (*iso9660).hardlink_rbtree,
            n,
            ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
        )
    }
}
unsafe extern "C" fn isofile_hd_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut h1: *const hardlink = n1 as *const hardlink;
    let mut h2: *const hardlink = n2 as *const hardlink;
    return strcmp(
        archive_entry_pathname((*(*h1).file_list.first).entry),
        archive_entry_pathname((*(*h2).file_list.first).entry),
    );
}
unsafe extern "C" fn isofile_hd_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut h: *const hardlink = n as *const hardlink;
    return strcmp(
        archive_entry_pathname((*(*h).file_list.first).entry),
        key as *const libc::c_char,
    );
}
unsafe extern "C" fn isofile_init_hardlinks(mut iso9660: *mut iso9660) {
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    isofile_hd_cmp_node
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    isofile_hd_cmp_key
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    __archive_rb_tree_init(&mut (*iso9660).hardlink_rbtree, &rb_ops);
}
unsafe extern "C" fn isofile_free_hardlinks(mut iso9660: *mut iso9660) {
    let mut n: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut tmp: *mut archive_rb_node = 0 as *mut archive_rb_node;
    n = __archive_rb_tree_iterate(
        &mut (*iso9660).hardlink_rbtree,
        NULL as *mut archive_rb_node,
        ARCHIVE_RB_DIR_LEFT as libc::c_uint,
    );
    while !n.is_null() && {
        tmp = __archive_rb_tree_iterate(
            &mut (*iso9660).hardlink_rbtree,
            n,
            ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
        );
        (1 as libc::c_int) != 0
    } {
        __archive_rb_tree_remove_node(&mut (*iso9660).hardlink_rbtree, n);
        free(n as *mut libc::c_void);
        n = tmp
    }
}
unsafe extern "C" fn isoent_new(mut file: *mut isofile) -> *mut isoent {
    let mut isoent: *mut isoent = 0 as *mut isoent;
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    isoent_cmp_node
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    isoent_cmp_key
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    isoent = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<isoent>() as libc::c_ulong,
    ) as *mut isoent;
    if isoent.is_null() {
        return 0 as *mut isoent;
    }
    (*isoent).file = file;
    (*isoent).children.first = NULL as *mut isoent;
    (*isoent).children.last = &mut (*isoent).children.first;
    __archive_rb_tree_init(&mut (*isoent).rbtree, &rb_ops);
    (*isoent).subdirs.first = NULL as *mut isoent;
    (*isoent).subdirs.last = &mut (*isoent).subdirs.first;
    (*isoent).extr_rec_list.first = NULL as *mut extr_rec;
    (*isoent).extr_rec_list.last = &mut (*isoent).extr_rec_list.first;
    (*isoent).extr_rec_list.current = NULL as *mut extr_rec;
    if archive_entry_filetype((*file).entry) == AE_IFDIR as mode_t {
        (*isoent).set_dir(1 as libc::c_int)
    }
    return isoent;
}
#[inline]
unsafe extern "C" fn isoent_clone(mut src: *mut isoent) -> *mut isoent {
    return isoent_new((*src).file);
}
unsafe extern "C" fn _isoent_free(mut isoent: *mut isoent) {
    let mut er: *mut extr_rec = 0 as *mut extr_rec;
    let mut er_next: *mut extr_rec = 0 as *mut extr_rec;
    free((*isoent).children_sorted as *mut libc::c_void);
    free((*isoent).identifier as *mut libc::c_void);
    er = (*isoent).extr_rec_list.first;
    while !er.is_null() {
        er_next = (*er).next;
        free(er as *mut libc::c_void);
        er = er_next
    }
    free(isoent as *mut libc::c_void);
}
unsafe extern "C" fn isoent_free_all(mut isoent: *mut isoent) {
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut np_temp: *mut isoent = 0 as *mut isoent;
    if isoent.is_null() {
        return;
    }
    np = isoent;
    loop {
        if (*np).dir() != 0 {
            if !(*np).children.first.is_null() {
                /* Enter to sub directories. */
                np = (*np).children.first;
                continue;
            }
        }
        loop {
            np_temp = np;
            if (*np).chnext.is_null() {
                /* Return to the parent directory. */
                np = (*np).parent;
                _isoent_free(np_temp);
                if np == np_temp {
                    return;
                }
            } else {
                np = (*np).chnext;
                _isoent_free(np_temp);
                break;
            }
        }
    }
}
unsafe extern "C" fn isoent_create_virtual_dir(
    mut a: *mut archive_write,
    mut iso9660: *mut iso9660,
    mut pathname: *const libc::c_char,
) -> *mut isoent {
    let mut file: *mut isofile = 0 as *mut isofile;
    let mut isoent: *mut isoent = 0 as *mut isoent;
    file = isofile_new(a, NULL as *mut archive_entry);
    if file.is_null() {
        return 0 as *mut isoent;
    }
    archive_entry_set_pathname((*file).entry, pathname);
    archive_entry_unset_mtime((*file).entry);
    archive_entry_unset_atime((*file).entry);
    archive_entry_unset_ctime((*file).entry);
    archive_entry_set_uid((*file).entry, getuid() as la_int64_t);
    archive_entry_set_gid((*file).entry, getgid() as la_int64_t);
    archive_entry_set_mode(
        (*file).entry,
        0o555 as libc::c_int as libc::c_uint | AE_IFDIR as mode_t,
    );
    archive_entry_set_nlink((*file).entry, 2 as libc::c_int as libc::c_uint);
    if isofile_gen_utility_names(a, file) < ARCHIVE_WARN {
        isofile_free(file);
        return 0 as *mut isoent;
    }
    isofile_add_entry(iso9660, file);
    isoent = isoent_new(file);
    if isoent.is_null() {
        return 0 as *mut isoent;
    }
    (*isoent).set_dir(1 as libc::c_int);
    (*isoent).set_virtual_0(1 as libc::c_int);
    return isoent;
}
unsafe extern "C" fn isoent_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut e1: *const isoent = n1 as *const isoent;
    let mut e2: *const isoent = n2 as *const isoent;
    return strcmp((*(*e1).file).basename.s, (*(*e2).file).basename.s);
}
unsafe extern "C" fn isoent_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut e: *const isoent = n as *const isoent;
    return strcmp((*(*e).file).basename.s, key as *const libc::c_char);
}
unsafe extern "C" fn isoent_add_child_head(
    mut parent: *mut isoent,
    mut child: *mut isoent,
) -> libc::c_int {
    if __archive_rb_tree_insert_node(&mut (*parent).rbtree, child as *mut archive_rb_node) == 0 {
        return 0 as libc::c_int;
    }
    (*child).chnext = (*parent).children.first;
    if (*child).chnext.is_null() {
        (*parent).children.last = &mut (*child).chnext
    }
    (*parent).children.first = child;
    (*parent).children.cnt += 1;
    (*child).parent = parent;
    /* Add a child to a sub-directory chain */
    if (*child).dir() != 0 {
        (*child).drnext = (*parent).subdirs.first;
        if (*child).drnext.is_null() {
            (*parent).subdirs.last = &mut (*child).drnext
        }
        (*parent).subdirs.first = child;
        (*parent).subdirs.cnt += 1;
        (*child).parent = parent
    } else {
        (*child).drnext = NULL as *mut isoent
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isoent_add_child_tail(
    mut parent: *mut isoent,
    mut child: *mut isoent,
) -> libc::c_int {
    if __archive_rb_tree_insert_node(&mut (*parent).rbtree, child as *mut archive_rb_node) == 0 {
        return 0 as libc::c_int;
    }
    (*child).chnext = NULL as *mut isoent;
    *(*parent).children.last = child;
    (*parent).children.last = &mut (*child).chnext;
    (*parent).children.cnt += 1;
    (*child).parent = parent;
    /* Add a child to a sub-directory chain */
    (*child).drnext = NULL as *mut isoent;
    if (*child).dir() != 0 {
        *(*parent).subdirs.last = child;
        (*parent).subdirs.last = &mut (*child).drnext;
        (*parent).subdirs.cnt += 1;
        (*child).parent = parent
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isoent_remove_child(mut parent: *mut isoent, mut child: *mut isoent) {
    let mut ent: *mut isoent = 0 as *mut isoent;
    /* Remove a child entry from children chain. */
    ent = (*parent).children.first;
    while (*ent).chnext != child {
        ent = (*ent).chnext
    }
    (*ent).chnext = (*(*ent).chnext).chnext;
    if (*ent).chnext.is_null() {
        (*parent).children.last = &mut (*ent).chnext
    }
    (*parent).children.cnt -= 1;
    if (*child).dir() != 0 {
        /* Remove a child entry from sub-directory chain. */
        ent = (*parent).subdirs.first;
        while (*ent).drnext != child {
            ent = (*ent).drnext
        }
        (*ent).drnext = (*(*ent).drnext).drnext;
        if (*ent).drnext.is_null() {
            (*parent).subdirs.last = &mut (*ent).drnext
        }
        (*parent).subdirs.cnt -= 1
    }
    __archive_rb_tree_remove_node(&mut (*parent).rbtree, child as *mut archive_rb_node);
}
unsafe extern "C" fn isoent_clone_tree(
    mut a: *mut archive_write,
    mut nroot: *mut *mut isoent,
    mut root: *mut isoent,
) -> libc::c_int {
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut xroot: *mut isoent = 0 as *mut isoent;
    let mut newent: *mut isoent = 0 as *mut isoent;
    np = root;
    xroot = NULL as *mut isoent;
    loop {
        newent = isoent_clone(np);
        if newent.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if xroot.is_null() {
            xroot = newent;
            *nroot = xroot;
            (*newent).parent = xroot
        } else {
            isoent_add_child_tail(xroot, newent);
        }
        if (*np).dir() != 0 && !(*np).children.first.is_null() {
            /* Enter to sub directories. */
            np = (*np).children.first;
            xroot = newent
        } else {
            while np != (*np).parent {
                if (*np).chnext.is_null() {
                    /* Return to the parent directory. */
                    np = (*np).parent;
                    xroot = (*xroot).parent
                } else {
                    np = (*np).chnext;
                    break;
                }
            }
        }
        if !(np != (*np).parent) {
            break;
        }
    }
    return 0 as libc::c_int;
}
/*
 * Setup directory locations.
 */
unsafe extern "C" fn isoent_setup_directory_location(
    mut iso9660: *mut iso9660,
    mut location: libc::c_int,
    mut vdd: *mut vdd,
) {
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut depth: libc::c_int = 0;
    (*vdd).total_dir_block = 0 as libc::c_int;
    depth = 0 as libc::c_int;
    np = (*vdd).rootent;
    loop {
        let mut block: libc::c_int = 0;
        (*np).dir_block = calculate_directory_descriptors(iso9660, vdd, np, depth);
        (*vdd).total_dir_block += (*np).dir_block;
        (*np).dir_location = location as uint32_t;
        location += (*np).dir_block;
        block = extra_setup_location(np, location);
        (*vdd).total_dir_block += block;
        location += block;
        if !(*np).subdirs.first.is_null() && (depth + 1 as libc::c_int) < (*vdd).max_depth {
            /* Enter to sub directories. */
            np = (*np).subdirs.first;
            depth += 1
        } else {
            while np != (*np).parent {
                if (*np).drnext.is_null() {
                    /* Return to the parent directory. */
                    np = (*np).parent;
                    depth -= 1
                } else {
                    np = (*np).drnext;
                    break;
                }
            }
        }
        if !(np != (*np).parent) {
            break;
        }
    }
}
unsafe extern "C" fn _isoent_file_location(
    mut iso9660: *mut iso9660,
    mut isoent: *mut isoent,
    mut symlocation: *mut libc::c_int,
) {
    let mut children: *mut *mut isoent = 0 as *mut *mut isoent;
    let mut n: libc::c_int = 0;
    if (*isoent).children.cnt == 0 as libc::c_int {
        return;
    }
    children = (*isoent).children_sorted;
    n = 0 as libc::c_int;
    while n < (*isoent).children.cnt {
        let mut np: *mut isoent = 0 as *mut isoent;
        let mut file: *mut isofile = 0 as *mut isofile;
        np = *children.offset(n as isize);
        if !((*np).dir() != 0) {
            if !(np == (*iso9660).el_torito.boot) {
                file = (*np).file;
                if !((*file).boot as libc::c_uint != 0 || !(*file).hardlink_target.is_null()) {
                    if archive_entry_filetype((*file).entry) == AE_IFLNK as mode_t
                        || (*file).content.size == 0 as libc::c_int as libc::c_long
                    {
                        /*
                         * Do not point a valid location.
                         * Make sure entry is not hardlink file.
                         */
                        let fresh16 = *symlocation;
                        *symlocation = *symlocation - 1;
                        (*file).content.location = fresh16 as uint32_t
                    } else {
                        (*file).write_content = 1 as libc::c_int
                    }
                }
            }
        }
        n += 1
    }
}
/*
 * Setup file locations.
 */
unsafe extern "C" fn isoent_setup_file_location(
    mut iso9660: *mut iso9660,
    mut location: libc::c_int,
) {
    let mut isoent: *mut isoent = 0 as *mut isoent;
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut file: *mut isofile = 0 as *mut isofile;
    let mut size: size_t = 0;
    let mut block: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut joliet: libc::c_int = 0;
    let mut symlocation: libc::c_int = 0;
    let mut total_block: libc::c_int = 0;
    (*iso9660).total_file_block = 0 as libc::c_int;
    isoent = (*iso9660).el_torito.catalog;
    if !isoent.is_null() {
        (*(*isoent).file).content.location = location as uint32_t;
        block = (archive_entry_size((*(*isoent).file).entry) + LOGICAL_BLOCK_SIZE as libc::c_long
            - 1 as libc::c_int as libc::c_long
            >> LOGICAL_BLOCK_BITS) as libc::c_int;
        location += block;
        (*iso9660).total_file_block += block
    }
    isoent = (*iso9660).el_torito.boot;
    if !isoent.is_null() {
        (*(*isoent).file).content.location = location as uint32_t;
        size = fd_boot_image_size((*iso9660).el_torito.media_type as libc::c_int);
        if size == 0 as libc::c_int as libc::c_ulong {
            size = archive_entry_size((*(*isoent).file).entry) as size_t
        }
        block = size as libc::c_int + LOGICAL_BLOCK_SIZE - 1 as libc::c_int >> LOGICAL_BLOCK_BITS;
        location += block;
        (*iso9660).total_file_block += block;
        (*(*isoent).file).content.blocks = block
    }
    depth = 0 as libc::c_int;
    symlocation = -(16 as libc::c_int);
    if (*iso9660).opt.rr() == 0 && (*iso9660).opt.joliet() as libc::c_int != 0 {
        joliet = 1 as libc::c_int;
        np = (*iso9660).joliet.rootent
    } else {
        joliet = 0 as libc::c_int;
        np = (*iso9660).primary.rootent
    }
    loop {
        _isoent_file_location(iso9660, np, &mut symlocation);
        if !(*np).subdirs.first.is_null()
            && (joliet != 0
                || ((*iso9660).opt.rr() as libc::c_int == OPT_RR_DISABLED
                    && (depth + 2 as libc::c_int) < (*iso9660).primary.max_depth
                    || (*iso9660).opt.rr() as libc::c_int != 0
                        && (depth + 1 as libc::c_int) < (*iso9660).primary.max_depth))
        {
            /* Enter to sub directories. */
            np = (*np).subdirs.first;
            depth += 1
        } else {
            while np != (*np).parent {
                if (*np).drnext.is_null() {
                    /* Return to the parent directory. */
                    np = (*np).parent;
                    depth -= 1
                } else {
                    np = (*np).drnext;
                    break;
                }
            }
        }
        if !(np != (*np).parent) {
            break;
        }
    }
    total_block = 0 as libc::c_int;
    file = (*iso9660).data_file_list.first;
    while !file.is_null() {
        if !((*file).write_content == 0) {
            (*file).cur_content = &mut (*file).content;
            loop {
                (*(*file).cur_content).location = location as uint32_t;
                location += (*(*file).cur_content).blocks;
                total_block += (*(*file).cur_content).blocks;
                /* Next fragment */
                (*file).cur_content = (*(*file).cur_content).next;
                if (*file).cur_content.is_null() {
                    break;
                }
            }
        }
        file = (*file).datanext
    }
    (*iso9660).total_file_block += total_block;
}
unsafe extern "C" fn get_path_component(
    mut name: *mut libc::c_char,
    mut n: size_t,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: size_t = 0;
    p = strchr(fn_0, '/' as i32);
    if p.is_null() {
        l = strlen(fn_0);
        if l == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
    } else {
        l = p.offset_from(fn_0) as libc::c_long as size_t
    }
    if l > n.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        return -(1 as libc::c_int);
    }
    memcpy(name as *mut libc::c_void, fn_0 as *const libc::c_void, l);
    *name.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
    return l as libc::c_int;
}
/*
 * Add a new entry into the tree.
 */
unsafe extern "C" fn isoent_tree(
    mut a: *mut archive_write,
    mut isoentpp: *mut *mut isoent,
) -> libc::c_int {
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut dent: *mut isoent = 0 as *mut isoent;
    let mut isoent: *mut isoent = 0 as *mut isoent;
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut f1: *mut isofile = 0 as *mut isofile;
    let mut f2: *mut isofile = 0 as *mut isofile;
    let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: libc::c_int = 0;
    isoent = *isoentpp;
    dent = (*iso9660).primary.rootent;
    if (*(*isoent).file).parentdir.length > 0 as libc::c_int as libc::c_ulong {
        p = (*(*isoent).file).parentdir.s;
        fn_0 = p
    } else {
        p = b"\x00" as *const u8 as *const libc::c_char;
        fn_0 = p
    }
    /*
     * If the path of the parent directory of `isoent' entry is
     * the same as the path of `cur_dirent', add isoent to
     * `cur_dirent'.
     */
    if (*iso9660).cur_dirstr.length == (*(*isoent).file).parentdir.length
        && strcmp((*iso9660).cur_dirstr.s, fn_0) == 0 as libc::c_int
    {
        if isoent_add_child_tail((*iso9660).cur_dirent, isoent) == 0 {
            np = __archive_rb_tree_find_node(
                &mut (*(*iso9660).cur_dirent).rbtree,
                (*(*isoent).file).basename.s as *const libc::c_void,
            ) as *mut isoent
        } else {
            return 0 as libc::c_int;
        }
    } else {
        loop {
            l = get_path_component(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                fn_0,
            );
            if l == 0 as libc::c_int {
                np = NULL as *mut isoent;
                break;
            } else {
                if l < 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"A name buffer is too small\x00" as *const u8 as *const libc::c_char,
                    );
                    _isoent_free(isoent);
                    return -(30 as libc::c_int);
                }
                np = isoent_find_child(dent, name.as_mut_ptr());
                if np.is_null()
                    || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                {
                    break;
                }
                /* Find next subdirectory. */
                if (*np).dir() == 0 {
                    /* NOT Directory! */
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"`%s\' is not directory, we cannot insert `%s\' \x00" as *const u8
                            as *const libc::c_char,
                        archive_entry_pathname((*(*np).file).entry),
                        archive_entry_pathname((*(*isoent).file).entry),
                    );
                    _isoent_free(isoent);
                    *isoentpp = NULL as *mut isoent;
                    return -(25 as libc::c_int);
                }
                fn_0 = fn_0.offset(l as isize);
                if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    fn_0 = fn_0.offset(1)
                }
                dent = np
            }
        }
        if np.is_null() {
            /*
             * Create virtual parent directories.
             */
            while *fn_0.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
                let mut vp: *mut isoent = 0 as *mut isoent;
                let mut as_0: archive_string = archive_string {
                    s: 0 as *mut libc::c_char,
                    length: 0,
                    buffer_length: 0,
                };
                as_0.s = NULL as *mut libc::c_char;
                as_0.length = 0 as libc::c_int as size_t;
                as_0.buffer_length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut as_0,
                    p as *const libc::c_void,
                    (fn_0.offset_from(p) as libc::c_long + l as libc::c_long) as size_t,
                );
                if *as_0
                    .s
                    .offset(as_0.length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '/' as i32
                {
                    *as_0.s.offset(
                        as_0.length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize
                    ) = '\u{0}' as i32 as libc::c_char;
                    as_0.length = as_0.length.wrapping_sub(1)
                }
                vp = isoent_create_virtual_dir(a, iso9660, as_0.s);
                if vp.is_null() {
                    archive_string_free(&mut as_0);
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
                    );
                    _isoent_free(isoent);
                    *isoentpp = NULL as *mut isoent;
                    return -(30 as libc::c_int);
                }
                archive_string_free(&mut as_0);
                if (*(*vp).file).dircnt > (*iso9660).dircnt_max {
                    (*iso9660).dircnt_max = (*(*vp).file).dircnt
                }
                isoent_add_child_tail(dent, vp);
                np = vp;
                fn_0 = fn_0.offset(l as isize);
                if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    fn_0 = fn_0.offset(1)
                }
                l = get_path_component(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    fn_0,
                );
                if l < 0 as libc::c_int {
                    archive_string_free(&mut as_0);
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"A name buffer is too small\x00" as *const u8 as *const libc::c_char,
                    );
                    _isoent_free(isoent);
                    *isoentpp = NULL as *mut isoent;
                    return -(30 as libc::c_int);
                }
                dent = np
            }
            /* Found out the parent directory where isoent can be
             * inserted. */
            (*iso9660).cur_dirent = dent;
            (*iso9660).cur_dirstr.length = 0 as libc::c_int as size_t;
            archive_string_ensure(
                &mut (*iso9660).cur_dirstr,
                (*(*dent).file)
                    .parentdir
                    .length
                    .wrapping_add((*(*dent).file).basename.length)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            );
            if (*(*dent).file)
                .parentdir
                .length
                .wrapping_add((*(*dent).file).basename.length)
                == 0 as libc::c_int as libc::c_ulong
            {
                *(*iso9660).cur_dirstr.s.offset(0 as libc::c_int as isize) =
                    0 as libc::c_int as libc::c_char
            } else {
                if (*(*dent).file).parentdir.length > 0 as libc::c_int as libc::c_ulong {
                    (*iso9660).cur_dirstr.length = 0 as libc::c_int as size_t;
                    archive_string_concat(
                        &mut (*iso9660).cur_dirstr,
                        &mut (*(*dent).file).parentdir,
                    );
                    archive_strappend_char(&mut (*iso9660).cur_dirstr, '/' as i32 as libc::c_char);
                }
                archive_string_concat(&mut (*iso9660).cur_dirstr, &mut (*(*dent).file).basename);
            }
            if isoent_add_child_tail(dent, isoent) == 0 {
                np = __archive_rb_tree_find_node(
                    &mut (*dent).rbtree,
                    (*(*isoent).file).basename.s as *const libc::c_void,
                ) as *mut isoent
            } else {
                return 0 as libc::c_int;
            }
        }
    }
    /*
     * We have already has the entry the filename of which is
     * the same.
     */
    f1 = (*np).file;
    f2 = (*isoent).file;
    /* If the file type of entries is different,
     * we cannot handle it. */
    if archive_entry_filetype((*f1).entry) != archive_entry_filetype((*f2).entry) {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Found duplicate entries `%s\' and its file type is different\x00" as *const u8
                as *const libc::c_char,
            archive_entry_pathname((*f1).entry),
        );
        _isoent_free(isoent);
        *isoentpp = NULL as *mut isoent;
        return -(25 as libc::c_int);
    }
    /* Swap file entries. */
    (*np).file = f2;
    (*isoent).file = f1;
    (*np).set_virtual_0(0 as libc::c_int);
    _isoent_free(isoent);
    *isoentpp = np;
    return 0 as libc::c_int;
}
/*
 * Find a entry from `isoent'
 */
unsafe extern "C" fn isoent_find_child(
    mut isoent: *mut isoent,
    mut child_name: *const libc::c_char,
) -> *mut isoent {
    let mut np: *mut isoent = 0 as *mut isoent;
    np = __archive_rb_tree_find_node(&mut (*isoent).rbtree, child_name as *const libc::c_void)
        as *mut isoent;
    return np;
}
/*
 * Find a entry full-path of which is specified by `fn' parameter,
 * in the tree.
 */
unsafe extern "C" fn isoent_find_entry(
    mut rootent: *mut isoent,
    mut fn_0: *const libc::c_char,
) -> *mut isoent {
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut isoent: *mut isoent = 0 as *mut isoent;
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut l: libc::c_int = 0;
    isoent = rootent;
    np = NULL as *mut isoent;
    loop {
        l = get_path_component(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            fn_0,
        );
        if l == 0 as libc::c_int {
            break;
        }
        fn_0 = fn_0.offset(l as isize);
        if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            fn_0 = fn_0.offset(1)
        }
        np = isoent_find_child(isoent, name.as_mut_ptr());
        if np.is_null() {
            break;
        }
        /* Not directory */
        if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
            break; /* We found out the entry */
        }
        /* Try sub directory. */
        isoent = np;
        np = NULL as *mut isoent;
        if (*isoent).dir() == 0 {
            break;
        }
    }
    return np;
}
/*
 * Following idr_* functions are used for resolving duplicated filenames
 * and unreceivable filenames to generate ISO9660/Joliet Identifiers.
 */
unsafe extern "C" fn idr_relaxed_filenames(mut map: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0x21 as libc::c_int;
    while i <= 0x2f as libc::c_int {
        *map.offset(i as isize) = 1 as libc::c_int as libc::c_char;
        i += 1
    }
    i = 0x3a as libc::c_int;
    while i <= 0x41 as libc::c_int {
        *map.offset(i as isize) = 1 as libc::c_int as libc::c_char;
        i += 1
    }
    i = 0x5b as libc::c_int;
    while i <= 0x5e as libc::c_int {
        *map.offset(i as isize) = 1 as libc::c_int as libc::c_char;
        i += 1
    }
    *map.offset(0x60 as libc::c_int as isize) = 1 as libc::c_int as libc::c_char;
    i = 0x7b as libc::c_int;
    while i <= 0x7e as libc::c_int {
        *map.offset(i as isize) = 1 as libc::c_int as libc::c_char;
        i += 1
    }
}
unsafe extern "C" fn idr_init(mut iso9660: *mut iso9660, mut vdd: *mut vdd, mut idr: *mut idr) {
    (*idr).idrent_pool = NULL as *mut idrent;
    (*idr).pool_size = 0 as libc::c_int;
    if (*vdd).vdd_type as libc::c_uint != VDD_JOLIET as libc::c_int as libc::c_uint {
        if (*iso9660).opt.iso_level() as libc::c_int <= 3 as libc::c_int {
            memcpy(
                (*idr).char_map.as_mut_ptr() as *mut libc::c_void,
                d_characters_map.as_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            );
        } else {
            memcpy(
                (*idr).char_map.as_mut_ptr() as *mut libc::c_void,
                d1_characters_map.as_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            );
            idr_relaxed_filenames((*idr).char_map.as_mut_ptr());
        }
    };
}
unsafe extern "C" fn idr_cleanup(mut idr: *mut idr) {
    free((*idr).idrent_pool as *mut libc::c_void);
}
unsafe extern "C" fn idr_ensure_poolsize(
    mut a: *mut archive_write,
    mut idr: *mut idr,
    mut cnt: libc::c_int,
) -> libc::c_int {
    if (*idr).pool_size < cnt {
        let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
        let bk: libc::c_int = ((1 as libc::c_int) << 7 as libc::c_int) - 1 as libc::c_int;
        let mut psize: libc::c_int = 0;
        psize = cnt + bk & !bk;
        p = realloc(
            (*idr).idrent_pool as *mut libc::c_void,
            (::std::mem::size_of::<idrent>() as libc::c_ulong).wrapping_mul(psize as libc::c_ulong),
        );
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*idr).idrent_pool = p as *mut idrent;
        (*idr).pool_size = psize
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn idr_start(
    mut a: *mut archive_write,
    mut idr: *mut idr,
    mut cnt: libc::c_int,
    mut ffmax: libc::c_int,
    mut num_size: libc::c_int,
    mut null_size: libc::c_int,
    mut rbt_ops: *const archive_rb_tree_ops,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    /* UNUSED */
    r = idr_ensure_poolsize(a, idr, cnt);
    if r != ARCHIVE_OK {
        return r;
    }
    __archive_rb_tree_init(&mut (*idr).rbtree, rbt_ops);
    (*idr).wait_list.first = NULL as *mut idrent;
    (*idr).wait_list.last = &mut (*idr).wait_list.first;
    (*idr).pool_idx = 0 as libc::c_int;
    (*idr).num_size = num_size;
    (*idr).null_size = null_size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn idr_register(
    mut idr: *mut idr,
    mut isoent: *mut isoent,
    mut weight: libc::c_int,
    mut noff: libc::c_int,
) {
    let mut idrent: *mut idrent = 0 as *mut idrent;
    let mut n: *mut idrent = 0 as *mut idrent;
    let fresh17 = (*idr).pool_idx;
    (*idr).pool_idx = (*idr).pool_idx + 1;
    idrent = &mut *(*idr).idrent_pool.offset(fresh17 as isize) as *mut idrent;
    (*idrent).avail = NULL as *mut idrent;
    (*idrent).wnext = (*idrent).avail;
    (*idrent).isoent = isoent;
    (*idrent).weight = weight;
    (*idrent).noff = noff;
    (*idrent).rename_num = 0 as libc::c_int;
    if __archive_rb_tree_insert_node(&mut (*idr).rbtree, &mut (*idrent).rbnode) == 0 {
        n = __archive_rb_tree_find_node(&mut (*idr).rbtree, (*idrent).isoent as *const libc::c_void)
            as *mut idrent;
        if !n.is_null() {
            /* this `idrent' needs to rename. */
            (*idrent).avail = n;
            *(*idr).wait_list.last = idrent;
            (*idr).wait_list.last = &mut (*idrent).wnext
        }
    };
}
unsafe extern "C" fn idr_extend_identifier(
    mut wnp: *mut idrent,
    mut numsize: libc::c_int,
    mut nullsize: libc::c_int,
) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wnp_ext_off: libc::c_int = 0;
    wnp_ext_off = (*(*wnp).isoent).ext_off;
    if (*wnp).noff + numsize != wnp_ext_off {
        p = (*(*wnp).isoent).identifier as *mut libc::c_uchar;
        /* Extend the filename; foo.c --> foo___.c */
        memmove(
            p.offset((*wnp).noff as isize).offset(numsize as isize) as *mut libc::c_void,
            p.offset(wnp_ext_off as isize) as *const libc::c_void,
            ((*(*wnp).isoent).ext_len + nullsize) as libc::c_ulong,
        );
        wnp_ext_off = (*wnp).noff + numsize;
        (*(*wnp).isoent).ext_off = wnp_ext_off;
        (*(*wnp).isoent).id_len = wnp_ext_off + (*(*wnp).isoent).ext_len
    };
}
unsafe extern "C" fn idr_resolve(
    mut idr: *mut idr,
    mut fsetnum: Option<unsafe extern "C" fn(_: *mut libc::c_uchar, _: libc::c_int) -> ()>,
) {
    let mut n: *mut idrent = 0 as *mut idrent;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    n = (*idr).wait_list.first;
    while !n.is_null() {
        idr_extend_identifier(n, (*idr).num_size, (*idr).null_size);
        p = ((*(*n).isoent).identifier as *mut libc::c_uchar).offset((*n).noff as isize);
        loop {
            let fresh18 = (*(*n).avail).rename_num;
            (*(*n).avail).rename_num = (*(*n).avail).rename_num + 1;
            fsetnum.expect("non-null function pointer")(p, fresh18);
            if !(__archive_rb_tree_insert_node(&mut (*idr).rbtree, &mut (*n).rbnode) == 0) {
                break;
            }
        }
        n = (*n).wnext
    }
}
unsafe extern "C" fn idr_set_num(mut p: *mut libc::c_uchar, mut num: libc::c_int) {
    static mut xdig: [libc::c_char; 36] = [
        '0' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        'A' as i32 as libc::c_char,
        'B' as i32 as libc::c_char,
        'C' as i32 as libc::c_char,
        'D' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'F' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'H' as i32 as libc::c_char,
        'I' as i32 as libc::c_char,
        'J' as i32 as libc::c_char,
        'K' as i32 as libc::c_char,
        'L' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'N' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        'Q' as i32 as libc::c_char,
        'R' as i32 as libc::c_char,
        'S' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'U' as i32 as libc::c_char,
        'V' as i32 as libc::c_char,
        'W' as i32 as libc::c_char,
        'X' as i32 as libc::c_char,
        'Y' as i32 as libc::c_char,
        'Z' as i32 as libc::c_char,
    ];
    num = (num as libc::c_ulong).wrapping_rem(
        (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong),
    ) as libc::c_int as libc::c_int;
    *p.offset(0 as libc::c_int as isize) = xdig[(num as libc::c_ulong).wrapping_div(
        (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong),
    ) as usize] as libc::c_uchar;
    num = (num as libc::c_ulong).wrapping_rem(
        (::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong),
    ) as libc::c_int as libc::c_int;
    *p.offset(1 as libc::c_int as isize) = xdig[(num as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
        as usize] as libc::c_uchar;
    num = (num as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    *p.offset(2 as libc::c_int as isize) = xdig[num as usize] as libc::c_uchar;
}
unsafe extern "C" fn idr_set_num_beutf16(mut p: *mut libc::c_uchar, mut num: libc::c_int) {
    static mut xdig: [uint16_t; 36] = [
        0x30 as libc::c_int as uint16_t,
        0x31 as libc::c_int as uint16_t,
        0x32 as libc::c_int as uint16_t,
        0x33 as libc::c_int as uint16_t,
        0x34 as libc::c_int as uint16_t,
        0x35 as libc::c_int as uint16_t,
        0x36 as libc::c_int as uint16_t,
        0x37 as libc::c_int as uint16_t,
        0x38 as libc::c_int as uint16_t,
        0x39 as libc::c_int as uint16_t,
        0x41 as libc::c_int as uint16_t,
        0x42 as libc::c_int as uint16_t,
        0x43 as libc::c_int as uint16_t,
        0x44 as libc::c_int as uint16_t,
        0x45 as libc::c_int as uint16_t,
        0x46 as libc::c_int as uint16_t,
        0x47 as libc::c_int as uint16_t,
        0x48 as libc::c_int as uint16_t,
        0x49 as libc::c_int as uint16_t,
        0x4a as libc::c_int as uint16_t,
        0x4b as libc::c_int as uint16_t,
        0x4c as libc::c_int as uint16_t,
        0x4d as libc::c_int as uint16_t,
        0x4e as libc::c_int as uint16_t,
        0x4f as libc::c_int as uint16_t,
        0x50 as libc::c_int as uint16_t,
        0x51 as libc::c_int as uint16_t,
        0x52 as libc::c_int as uint16_t,
        0x53 as libc::c_int as uint16_t,
        0x54 as libc::c_int as uint16_t,
        0x55 as libc::c_int as uint16_t,
        0x56 as libc::c_int as uint16_t,
        0x57 as libc::c_int as uint16_t,
        0x58 as libc::c_int as uint16_t,
        0x59 as libc::c_int as uint16_t,
        0x5a as libc::c_int as uint16_t,
    ];
    num = (num as libc::c_ulong).wrapping_rem(
        (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
            )
            .wrapping_mul(
                (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
            ),
    ) as libc::c_int as libc::c_int;
    archive_be16enc(
        p as *mut libc::c_void,
        xdig[(num as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
                ),
        ) as usize],
    );
    num = (num as libc::c_ulong).wrapping_rem(
        (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_mul(
                (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
            ),
    ) as libc::c_int as libc::c_int;
    archive_be16enc(
        p.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        xdig[(num as libc::c_ulong).wrapping_div(
            (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as usize],
    );
    num = (num as libc::c_ulong).wrapping_rem(
        (::std::mem::size_of::<[uint16_t; 36]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
    ) as libc::c_int as libc::c_int;
    archive_be16enc(
        p.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        xdig[num as usize],
    );
}
/*
 * Generate ISO9660 Identifier.
 */
unsafe extern "C" fn isoent_gen_iso9660_identifier(
    mut a: *mut archive_write,
    mut isoent: *mut isoent,
    mut idr: *mut idr,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660; /* fnmax + '.' + 3 */
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut char_map: *const libc::c_char = 0 as *const libc::c_char;
    let mut allow_ldots: libc::c_char = 0;
    let mut allow_multidot: libc::c_char = 0;
    let mut allow_period: libc::c_char = 0;
    let mut allow_vernum: libc::c_char = 0;
    let mut fnmax: libc::c_int = 0;
    let mut ffmax: libc::c_int = 0;
    let mut dnmax: libc::c_int = 0;
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    isoent_cmp_node_iso9660
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    isoent_cmp_key_iso9660
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    if (*isoent).children.cnt == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    iso9660 = (*a).format_data as *mut iso9660;
    char_map = (*idr).char_map.as_mut_ptr();
    if (*iso9660).opt.iso_level() as libc::c_int <= 3 as libc::c_int {
        allow_ldots = 0 as libc::c_int as libc::c_char;
        allow_multidot = 0 as libc::c_int as libc::c_char;
        allow_period = 1 as libc::c_int as libc::c_char;
        allow_vernum = (*iso9660).opt.allow_vernum() as libc::c_char;
        if (*iso9660).opt.iso_level() as libc::c_int == 1 as libc::c_int {
            fnmax = 8 as libc::c_int;
            ffmax = 12 as libc::c_int;
            dnmax = 8 as libc::c_int
        } else {
            fnmax = 30 as libc::c_int;
            ffmax = 31 as libc::c_int;
            dnmax = 31 as libc::c_int
        }
    } else {
        allow_multidot = 1 as libc::c_int as libc::c_char;
        allow_ldots = allow_multidot;
        allow_vernum = 0 as libc::c_int as libc::c_char;
        allow_period = allow_vernum;
        if (*iso9660).opt.rr() != 0 {
            /*
             * MDR : The maximum size of Directory Record(254).
             * DRL : A Directory Record Length(33).
             * CE  : A size of SUSP CE System Use Entry(28).
             * MDR - DRL - CE = 254 - 33 - 28 = 193.
             */
            dnmax = 193 as libc::c_int;
            ffmax = dnmax;
            fnmax = ffmax
        } else {
            /*
             * XA  : CD-ROM XA System Use Extension
             *       Information(14).
             * MDR - DRL - XA = 254 - 33 -14 = 207.
             */
            dnmax = 207 as libc::c_int;
            ffmax = dnmax;
            fnmax = ffmax
        }
    }
    r = idr_start(
        a,
        idr,
        (*isoent).children.cnt,
        ffmax,
        3 as libc::c_int,
        1 as libc::c_int,
        &rb_ops,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    np = (*isoent).children.first;
    while !np.is_null() {
        let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut xdot: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ext_off: libc::c_int = 0;
        let mut noff: libc::c_int = 0;
        let mut weight: libc::c_int = 0;
        l = (*(*np).file).basename.length as libc::c_int;
        p = malloc((l + 31 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        memcpy(
            p as *mut libc::c_void,
            (*(*np).file).basename.s as *const libc::c_void,
            l as libc::c_ulong,
        );
        *p.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
        (*np).identifier = p;
        xdot = NULL as *mut libc::c_char;
        dot = xdot;
        if allow_ldots == 0 {
            /*
             * If there is a '.' character at the first byte,
             * it has to be replaced by '_' character.
             */
            if *p as libc::c_int == '.' as i32 {
                let fresh19 = p;
                p = p.offset(1);
                *fresh19 = '_' as i32 as libc::c_char
            }
        }
        let mut current_block_51: u64;
        while *p != 0 {
            if *p as libc::c_int & 0x80 as libc::c_int != 0 {
                *p = '_' as i32 as libc::c_char
            } else if *char_map.offset(*p as libc::c_uchar as isize) != 0 {
                /* if iso-level is '4', a character '.' is
                 * allowed by char_map. */
                if *p as libc::c_int == '.' as i32 {
                    xdot = dot;
                    dot = p
                }
            } else if *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32 {
                *p = (*p as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char
            } else {
                if *p as libc::c_int == '.' as i32 {
                    xdot = dot;
                    dot = p;
                    if allow_multidot != 0 {
                        current_block_51 = 1356832168064818221;
                    } else {
                        current_block_51 = 11441799814184323368;
                    }
                } else {
                    current_block_51 = 11441799814184323368;
                }
                match current_block_51 {
                    1356832168064818221 => {}
                    _ => *p = '_' as i32 as libc::c_char,
                }
            }
            p = p.offset(1)
        }
        p = (*np).identifier;
        weight = -(1 as libc::c_int);
        if dot.is_null() {
            let mut nammax: libc::c_int = 0;
            if (*np).dir() != 0 {
                nammax = dnmax
            } else {
                nammax = fnmax
            }
            if l > nammax {
                *p.offset(nammax as isize) = '\u{0}' as i32 as libc::c_char;
                weight = nammax;
                ext_off = nammax
            } else {
                ext_off = l
            }
        } else {
            *dot = '.' as i32 as libc::c_char;
            ext_off = dot.offset_from(p) as libc::c_long as libc::c_int;
            if (*iso9660).opt.iso_level() as libc::c_int == 1 as libc::c_int {
                if dot.offset_from(p) as libc::c_long <= 8 as libc::c_int as libc::c_long {
                    if strlen(dot) > 4 as libc::c_int as libc::c_ulong {
                        /* A length of a file extension
                         * must be less than 4 */
                        *dot.offset(4 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
                        weight = 0 as libc::c_int
                    }
                } else {
                    *p.offset(8 as libc::c_int as isize) = *dot.offset(0 as libc::c_int as isize);
                    *p.offset(9 as libc::c_int as isize) = *dot.offset(1 as libc::c_int as isize);
                    *p.offset(10 as libc::c_int as isize) = *dot.offset(2 as libc::c_int as isize);
                    *p.offset(11 as libc::c_int as isize) = *dot.offset(3 as libc::c_int as isize);
                    *p.offset(12 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
                    weight = 8 as libc::c_int;
                    ext_off = 8 as libc::c_int
                }
            } else if (*np).dir() != 0 {
                if l > dnmax {
                    *p.offset(dnmax as isize) = '\u{0}' as i32 as libc::c_char;
                    weight = dnmax;
                    if ext_off > dnmax {
                        ext_off = dnmax
                    }
                }
            } else if l > ffmax {
                let mut extlen: libc::c_int = strlen(dot) as libc::c_int;
                let mut xdoff: libc::c_int = 0;
                if !xdot.is_null() {
                    xdoff = xdot.offset_from(p) as libc::c_long as libc::c_int
                } else {
                    xdoff = 0 as libc::c_int
                }
                if extlen > 1 as libc::c_int && xdoff < fnmax - 1 as libc::c_int {
                    let mut off: libc::c_int = 0;
                    if extlen > ffmax {
                        extlen = ffmax
                    }
                    off = ffmax - extlen;
                    if off == 0 as libc::c_int {
                        /* A dot('.')  character
                         * doesn't place to the first
                         * byte of identifier. */
                        off += 1;
                        extlen -= 1
                    }
                    memmove(
                        p.offset(off as isize) as *mut libc::c_void,
                        dot as *const libc::c_void,
                        extlen as libc::c_ulong,
                    );
                    *p.offset(ffmax as isize) = '\u{0}' as i32 as libc::c_char;
                    ext_off = off;
                    weight = off
                } else {
                    *p.offset(fnmax as isize) = '\u{0}' as i32 as libc::c_char;
                    ext_off = fnmax;
                    weight = fnmax
                }
            }
        }
        /* Save an offset of a file name extension to sort files. */
        (*np).ext_off = ext_off;
        (*np).ext_len = strlen(&mut *p.offset(ext_off as isize)) as libc::c_int;
        l = ext_off + (*np).ext_len;
        (*np).id_len = l;
        /* Make an offset of the number which is used to be set
         * hexadecimal number to avoid duplicate identifier. */
        if (*iso9660).opt.iso_level() as libc::c_int == 1 as libc::c_int {
            if ext_off >= 5 as libc::c_int {
                noff = 5 as libc::c_int
            } else {
                noff = ext_off
            }
        } else if l == ffmax {
            noff = ext_off - 3 as libc::c_int
        } else if l == ffmax - 1 as libc::c_int {
            noff = ext_off - 2 as libc::c_int
        } else if l == ffmax - 2 as libc::c_int {
            noff = ext_off - 1 as libc::c_int
        } else {
            noff = ext_off
        }
        /* Register entry to the identifier resolver. */
        idr_register(idr, np, weight, noff);
        np = (*np).chnext
    }
    /* Resolve duplicate identifier. */
    idr_resolve(
        idr,
        Some(idr_set_num as unsafe extern "C" fn(_: *mut libc::c_uchar, _: libc::c_int) -> ()),
    );
    /* Add a period and a version number to identifiers. */
    np = (*isoent).children.first;
    while !np.is_null() {
        if (*np).dir() == 0 && (*np).rr_child.is_null() {
            p = (*np)
                .identifier
                .offset((*np).ext_off as isize)
                .offset((*np).ext_len as isize);
            if (*np).ext_len == 0 as libc::c_int && allow_period as libc::c_int != 0 {
                let fresh20 = p;
                p = p.offset(1);
                *fresh20 = '.' as i32 as libc::c_char;
                (*np).ext_len = 1 as libc::c_int
            }
            if (*np).ext_len == 1 as libc::c_int && allow_period == 0 {
                p = p.offset(-1);
                *p = '\u{0}' as i32 as libc::c_char;
                (*np).ext_len = 0 as libc::c_int
            }
            (*np).id_len = (*np).ext_off + (*np).ext_len;
            if allow_vernum != 0 {
                let fresh21 = p;
                p = p.offset(1);
                *fresh21 = ';' as i32 as libc::c_char;
                let fresh22 = p;
                p = p.offset(1);
                *fresh22 = '1' as i32 as libc::c_char;
                (*np).id_len += 2 as libc::c_int
            }
            *p = '\u{0}' as i32 as libc::c_char
        } else {
            (*np).id_len = (*np).ext_off + (*np).ext_len
        }
        (*np).mb_len = (*np).id_len;
        np = (*np).chnext
    }
    return 0 as libc::c_int;
}
/*
 * Generate Joliet Identifier.
 */
unsafe extern "C" fn isoent_gen_joliet_identifier(
    mut a: *mut archive_write,
    mut isoent: *mut isoent,
    mut idr: *mut idr,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = 0 as *mut iso9660; /* '_' */
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut l: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut ffmax: size_t = 0;
    let mut parent_len: size_t = 0;
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    isoent_cmp_node_joliet
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    isoent_cmp_key_joliet
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    if (*isoent).children.cnt == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    iso9660 = (*a).format_data as *mut iso9660;
    if (*iso9660).opt.joliet() as libc::c_int == OPT_JOLIET_LONGNAME {
        ffmax = 206 as libc::c_int as size_t
    } else {
        ffmax = 128 as libc::c_int as size_t
    }
    r = idr_start(
        a,
        idr,
        (*isoent).children.cnt,
        ffmax as libc::c_int,
        6 as libc::c_int,
        2 as libc::c_int,
        &rb_ops,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    parent_len = 1 as libc::c_int as size_t;
    np = isoent;
    while (*np).parent != np {
        parent_len = (parent_len as libc::c_ulong)
            .wrapping_add(((*np).mb_len + 1 as libc::c_int) as libc::c_ulong)
            as size_t as size_t;
        np = (*np).parent
    }
    np = (*isoent).children.first;
    while !np.is_null() {
        let mut dot: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut ext_off: libc::c_int = 0;
        let mut noff: libc::c_int = 0;
        let mut weight: libc::c_int = 0;
        let mut lt: size_t = 0;
        l = (*(*np).file).basename_utf16.length;
        if l > ffmax {
            l = ffmax
        }
        p = malloc(
            l.wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_uchar;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        memcpy(
            p as *mut libc::c_void,
            (*(*np).file).basename_utf16.s as *const libc::c_void,
            l,
        );
        *p.offset(l as isize) = 0 as libc::c_int as libc::c_uchar;
        *p.offset(l.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) =
            0 as libc::c_int as libc::c_uchar;
        (*np).identifier = p as *mut libc::c_char;
        lt = l;
        dot = p.offset(l as isize);
        weight = 0 as libc::c_int;
        while lt > 0 as libc::c_int as libc::c_ulong {
            if joliet_allowed_char(
                *p.offset(0 as libc::c_int as isize),
                *p.offset(1 as libc::c_int as isize),
            ) == 0
            {
                archive_be16enc(p as *mut libc::c_void, 0x5f as libc::c_int as uint16_t);
            } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 0x2e as libc::c_int
            {
                /* '.' */
                dot = p
            }
            p = p.offset(2 as libc::c_int as isize);
            lt = (lt as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        ext_off = dot.offset_from((*np).identifier as *mut libc::c_uchar) as libc::c_long
            as libc::c_int;
        (*np).ext_off = ext_off;
        (*np).ext_len = l as libc::c_int - ext_off;
        (*np).id_len = l as libc::c_int;
        /*
         * Get a length of MBS of a full-pathname.
         */
        if (*(*np).file).basename_utf16.length > ffmax {
            if archive_strncpy_l(
                &mut (*iso9660).mbs,
                (*np).identifier as *const libc::c_char as *const libc::c_void,
                l,
                (*iso9660).sconv_from_utf16be,
            ) != 0 as libc::c_int
                && errno == ENOMEM
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"No memory\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            (*np).mb_len = (*iso9660).mbs.length as libc::c_int;
            if (*np).mb_len != (*(*np).file).basename.length as libc::c_int {
                weight = (*np).mb_len
            }
        } else {
            (*np).mb_len = (*(*np).file).basename.length as libc::c_int
        }
        /* If a length of full-pathname is longer than 240 bytes,
         * it violates Joliet extensions regulation. */
        if parent_len > 240 as libc::c_int as libc::c_ulong
            || (*np).mb_len > 240 as libc::c_int
            || parent_len.wrapping_add((*np).mb_len as libc::c_ulong)
                > 240 as libc::c_int as libc::c_ulong
        {
            archive_set_error(&mut (*a).archive as *mut archive,
                              ARCHIVE_ERRNO_MISC,
                              b"The regulation of Joliet extensions; A length of a full-pathname of `%s\' is longer than 240 bytes, (p=%d, b=%d)\x00"
                                  as *const u8 as *const libc::c_char,
                              archive_entry_pathname((*(*np).file).entry),
                              parent_len as libc::c_int, (*np).mb_len);
            return -(30 as libc::c_int);
        }
        /* Make an offset of the number which is used to be set
         * hexadecimal number to avoid duplicate identifier. */
        if l == ffmax {
            noff = ext_off - 6 as libc::c_int
        } else if l == ffmax.wrapping_sub(2 as libc::c_int as libc::c_ulong) {
            noff = ext_off - 4 as libc::c_int
        } else if l == ffmax.wrapping_sub(4 as libc::c_int as libc::c_ulong) {
            noff = ext_off - 2 as libc::c_int
        } else {
            noff = ext_off
        }
        /* Register entry to the identifier resolver. */
        idr_register(idr, np, weight, noff);
        np = (*np).chnext
    }
    /* Resolve duplicate identifier with Joliet Volume. */
    idr_resolve(
        idr,
        Some(
            idr_set_num_beutf16
                as unsafe extern "C" fn(_: *mut libc::c_uchar, _: libc::c_int) -> (),
        ),
    );
    return 0 as libc::c_int;
}
/*
 * This comparing rule is according to ISO9660 Standard 9.3
 */
unsafe extern "C" fn isoent_cmp_iso9660_identifier(
    mut p1: *const isoent,
    mut p2: *const isoent,
) -> libc::c_int {
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmp: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    s1 = (*p1).identifier;
    s2 = (*p2).identifier;
    /* Compare File Name */
    l = (*p1).ext_off;
    if l > (*p2).ext_off {
        l = (*p2).ext_off
    }
    cmp = memcmp(
        s1 as *const libc::c_void,
        s2 as *const libc::c_void,
        l as libc::c_ulong,
    );
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    if (*p1).ext_off < (*p2).ext_off {
        s2 = s2.offset(l as isize);
        l = (*p2).ext_off - (*p1).ext_off;
        loop {
            let fresh23 = l;
            l = l - 1;
            if !(fresh23 != 0) {
                break;
            }
            let fresh24 = s2;
            s2 = s2.offset(1);
            if 0x20 as libc::c_int != *fresh24 as libc::c_int {
                return 0x20 as libc::c_int
                    - *(s2.offset(-(1 as libc::c_int as isize)) as *const libc::c_uchar)
                        as libc::c_int;
            }
        }
    } else if (*p1).ext_off > (*p2).ext_off {
        s1 = s1.offset(l as isize);
        l = (*p1).ext_off - (*p2).ext_off;
        loop {
            let fresh25 = l;
            l = l - 1;
            if !(fresh25 != 0) {
                break;
            }
            let fresh26 = s1;
            s1 = s1.offset(1);
            if 0x20 as libc::c_int != *fresh26 as libc::c_int {
                return *(s1.offset(-(1 as libc::c_int as isize)) as *const libc::c_uchar)
                    as libc::c_int
                    - 0x20 as libc::c_int;
            }
        }
    }
    /* Compare File Name Extension */
    if (*p1).ext_len == 0 as libc::c_int && (*p2).ext_len == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*p1).ext_len == 1 as libc::c_int && (*p2).ext_len == 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*p1).ext_len <= 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*p2).ext_len <= 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    l = (*p1).ext_len;
    if l > (*p2).ext_len {
        l = (*p2).ext_len
    }
    s1 = (*p1).identifier.offset((*p1).ext_off as isize);
    s2 = (*p2).identifier.offset((*p2).ext_off as isize);
    if l > 1 as libc::c_int {
        cmp = memcmp(
            s1 as *const libc::c_void,
            s2 as *const libc::c_void,
            l as libc::c_ulong,
        );
        if cmp != 0 as libc::c_int {
            return cmp;
        }
    }
    if (*p1).ext_len < (*p2).ext_len {
        s2 = s2.offset(l as isize);
        l = (*p2).ext_len - (*p1).ext_len;
        loop {
            let fresh27 = l;
            l = l - 1;
            if !(fresh27 != 0) {
                break;
            }
            let fresh28 = s2;
            s2 = s2.offset(1);
            if 0x20 as libc::c_int != *fresh28 as libc::c_int {
                return 0x20 as libc::c_int
                    - *(s2.offset(-(1 as libc::c_int as isize)) as *const libc::c_uchar)
                        as libc::c_int;
            }
        }
    } else if (*p1).ext_len > (*p2).ext_len {
        s1 = s1.offset(l as isize);
        l = (*p1).ext_len - (*p2).ext_len;
        loop {
            let fresh29 = l;
            l = l - 1;
            if !(fresh29 != 0) {
                break;
            }
            let fresh30 = s1;
            s1 = s1.offset(1);
            if 0x20 as libc::c_int != *fresh30 as libc::c_int {
                return *(s1.offset(-(1 as libc::c_int as isize)) as *const libc::c_uchar)
                    as libc::c_int
                    - 0x20 as libc::c_int;
            }
        }
    }
    /* Compare File Version Number */
    /* No operation. The File Version Number is always one. */
    return cmp;
}
unsafe extern "C" fn isoent_cmp_node_iso9660(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut e1: *const idrent = n1 as *const idrent;
    let mut e2: *const idrent = n2 as *const idrent;
    return isoent_cmp_iso9660_identifier((*e2).isoent, (*e1).isoent);
}
unsafe extern "C" fn isoent_cmp_key_iso9660(
    mut node: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut isoent: *const isoent = key as *const isoent;
    let mut idrent: *const idrent = node as *const idrent;
    return isoent_cmp_iso9660_identifier(isoent, (*idrent).isoent);
}
unsafe extern "C" fn isoent_cmp_joliet_identifier(
    mut p1: *const isoent,
    mut p2: *const isoent,
) -> libc::c_int {
    let mut s1: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut s2: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut cmp: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    s1 = (*p1).identifier as *const libc::c_uchar;
    s2 = (*p2).identifier as *const libc::c_uchar;
    /* Compare File Name */
    l = (*p1).ext_off;
    if l > (*p2).ext_off {
        l = (*p2).ext_off
    }
    cmp = memcmp(
        s1 as *const libc::c_void,
        s2 as *const libc::c_void,
        l as libc::c_ulong,
    );
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    if (*p1).ext_off < (*p2).ext_off {
        s2 = s2.offset(l as isize);
        l = (*p2).ext_off - (*p1).ext_off;
        loop {
            let fresh31 = l;
            l = l - 1;
            if !(fresh31 != 0) {
                break;
            }
            let fresh32 = s2;
            s2 = s2.offset(1);
            if 0 as libc::c_int != *fresh32 as libc::c_int {
                return -(*s2.offset(-(1 as libc::c_int as isize)) as libc::c_int);
            }
        }
    } else if (*p1).ext_off > (*p2).ext_off {
        s1 = s1.offset(l as isize);
        l = (*p1).ext_off - (*p2).ext_off;
        loop {
            let fresh33 = l;
            l = l - 1;
            if !(fresh33 != 0) {
                break;
            }
            let fresh34 = s1;
            s1 = s1.offset(1);
            if 0 as libc::c_int != *fresh34 as libc::c_int {
                return *s1.offset(-(1 as libc::c_int as isize)) as libc::c_int;
            }
        }
    }
    /* Compare File Name Extension */
    if (*p1).ext_len == 0 as libc::c_int && (*p2).ext_len == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*p1).ext_len == 2 as libc::c_int && (*p2).ext_len == 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*p1).ext_len <= 2 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*p2).ext_len <= 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    l = (*p1).ext_len;
    if l > (*p2).ext_len {
        l = (*p2).ext_len
    }
    s1 = (*p1).identifier.offset((*p1).ext_off as isize) as *mut libc::c_uchar;
    s2 = (*p2).identifier.offset((*p2).ext_off as isize) as *mut libc::c_uchar;
    if l > 1 as libc::c_int {
        cmp = memcmp(
            s1 as *const libc::c_void,
            s2 as *const libc::c_void,
            l as libc::c_ulong,
        );
        if cmp != 0 as libc::c_int {
            return cmp;
        }
    }
    if (*p1).ext_len < (*p2).ext_len {
        s2 = s2.offset(l as isize);
        l = (*p2).ext_len - (*p1).ext_len;
        loop {
            let fresh35 = l;
            l = l - 1;
            if !(fresh35 != 0) {
                break;
            }
            let fresh36 = s2;
            s2 = s2.offset(1);
            if 0 as libc::c_int != *fresh36 as libc::c_int {
                return -(*s2.offset(-(1 as libc::c_int as isize)) as libc::c_int);
            }
        }
    } else if (*p1).ext_len > (*p2).ext_len {
        s1 = s1.offset(l as isize);
        l = (*p1).ext_len - (*p2).ext_len;
        loop {
            let fresh37 = l;
            l = l - 1;
            if !(fresh37 != 0) {
                break;
            }
            let fresh38 = s1;
            s1 = s1.offset(1);
            if 0 as libc::c_int != *fresh38 as libc::c_int {
                return *s1.offset(-(1 as libc::c_int as isize)) as libc::c_int;
            }
        }
    }
    /* Compare File Version Number */
    /* No operation. The File Version Number is always one. */
    return cmp;
}
unsafe extern "C" fn isoent_cmp_node_joliet(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut e1: *const idrent = n1 as *const idrent;
    let mut e2: *const idrent = n2 as *const idrent;
    return isoent_cmp_joliet_identifier((*e2).isoent, (*e1).isoent);
}
unsafe extern "C" fn isoent_cmp_key_joliet(
    mut node: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut isoent: *const isoent = key as *const isoent;
    let mut idrent: *const idrent = node as *const idrent;
    return isoent_cmp_joliet_identifier(isoent, (*idrent).isoent);
}
unsafe extern "C" fn isoent_make_sorted_files(
    mut a: *mut archive_write,
    mut isoent: *mut isoent,
    mut idr: *mut idr,
) -> libc::c_int {
    let mut rn: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut children: *mut *mut isoent = 0 as *mut *mut isoent;
    children = malloc(
        ((*isoent).children.cnt as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut isoent>() as libc::c_ulong),
    ) as *mut *mut isoent;
    if children.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*isoent).children_sorted = children;
    rn = __archive_rb_tree_iterate(
        &mut (*idr).rbtree,
        NULL as *mut archive_rb_node,
        ARCHIVE_RB_DIR_LEFT as libc::c_uint,
    );
    while !rn.is_null() {
        let mut idrent: *mut idrent = rn as *mut idrent;
        let fresh39 = children;
        children = children.offset(1);
        *fresh39 = (*idrent).isoent;
        rn = __archive_rb_tree_iterate(&mut (*idr).rbtree, rn, ARCHIVE_RB_DIR_RIGHT as libc::c_uint)
    }
    return 0 as libc::c_int;
}
/*
 * - Generate ISO9660 and Joliet identifiers from basenames.
 * - Sort files by each directory.
 */
unsafe extern "C" fn isoent_traverse_tree(
    mut a: *mut archive_write,
    mut vdd: *mut vdd,
) -> libc::c_int {
    let mut current_block: u64;
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut idr: idr = idr {
        idrent_pool: 0 as *mut idrent,
        rbtree: archive_rb_tree {
            rbt_root: 0 as *mut archive_rb_node,
            rbt_ops: 0 as *const archive_rb_tree_ops,
        },
        wait_list: C2RustUnnamed_10 {
            first: 0 as *mut idrent,
            last: 0 as *mut *mut idrent,
        },
        pool_size: 0,
        pool_idx: 0,
        num_size: 0,
        null_size: 0,
        char_map: [0; 128],
    };
    let mut depth: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut genid: Option<
        unsafe extern "C" fn(_: *mut archive_write, _: *mut isoent, _: *mut idr) -> libc::c_int,
    > = None;
    idr_init(iso9660, vdd, &mut idr);
    np = (*vdd).rootent;
    depth = 0 as libc::c_int;
    if (*vdd).vdd_type as libc::c_uint == VDD_JOLIET as libc::c_int as libc::c_uint {
        genid = Some(
            isoent_gen_joliet_identifier
                as unsafe extern "C" fn(
                    _: *mut archive_write,
                    _: *mut isoent,
                    _: *mut idr,
                ) -> libc::c_int,
        )
    } else {
        genid = Some(
            isoent_gen_iso9660_identifier
                as unsafe extern "C" fn(
                    _: *mut archive_write,
                    _: *mut isoent,
                    _: *mut idr,
                ) -> libc::c_int,
        )
    }
    loop {
        if (*np).virtual_0() != 0 && archive_entry_mtime_is_set((*(*np).file).entry) == 0 {
            /* Set properly times to virtual directory */
            archive_entry_set_mtime(
                (*(*np).file).entry,
                (*iso9660).birth_time,
                0 as libc::c_int as libc::c_long,
            );
            archive_entry_set_atime(
                (*(*np).file).entry,
                (*iso9660).birth_time,
                0 as libc::c_int as libc::c_long,
            );
            archive_entry_set_ctime(
                (*(*np).file).entry,
                (*iso9660).birth_time,
                0 as libc::c_int as libc::c_long,
            );
        }
        if !(*np).children.first.is_null() {
            if (*vdd).vdd_type as libc::c_uint != VDD_JOLIET as libc::c_int as libc::c_uint
                && (*iso9660).opt.rr() == 0
                && depth + 1 as libc::c_int >= (*vdd).max_depth
            {
                if (*np).children.cnt > 0 as libc::c_int {
                    (*iso9660).directories_too_deep = np
                }
                current_block = 14818589718467733107;
            } else {
                /* Generate Identifier */
                r = genid.expect("non-null function pointer")(a, np, &mut idr);
                if r < 0 as libc::c_int {
                    current_block = 222014967776118151;
                    break;
                }
                r = isoent_make_sorted_files(a, np, &mut idr);
                if r < 0 as libc::c_int {
                    current_block = 222014967776118151;
                    break;
                }
                if !(*np).subdirs.first.is_null() && (depth + 1 as libc::c_int) < (*vdd).max_depth {
                    /* Enter to sub directories. */
                    np = (*np).subdirs.first;
                    depth += 1;
                    current_block = 5720623009719927633;
                } else {
                    current_block = 14818589718467733107;
                }
            }
        } else {
            current_block = 14818589718467733107;
        }
        match current_block {
            14818589718467733107 => {
                while np != (*np).parent {
                    if (*np).drnext.is_null() {
                        /* Return to the parent directory. */
                        np = (*np).parent;
                        depth -= 1
                    } else {
                        np = (*np).drnext;
                        break;
                    }
                }
            }
            _ => {}
        }
        if !(np != (*np).parent) {
            current_block = 2873832966593178012;
            break;
        }
    }
    match current_block {
        2873832966593178012 => r = ARCHIVE_OK,
        _ => {}
    }
    idr_cleanup(&mut idr);
    return r;
}
/*
 * Collect directory entries into path_table by a directory depth.
 */
unsafe extern "C" fn isoent_collect_dirs(
    mut vdd: *mut vdd,
    mut rootent: *mut isoent,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut np: *mut isoent = 0 as *mut isoent;
    if rootent.is_null() {
        rootent = (*vdd).rootent
    }
    np = rootent;
    loop {
        /* Register current directory to pathtable. */
        path_table_add_entry(&mut *(*vdd).pathtbl.offset(depth as isize), np);
        if !(*np).subdirs.first.is_null() && (depth + 1 as libc::c_int) < (*vdd).max_depth {
            /* Enter to sub directories. */
            np = (*np).subdirs.first;
            depth += 1
        } else {
            while np != rootent {
                if (*np).drnext.is_null() {
                    /* Return to the parent directory. */
                    np = (*np).parent;
                    depth -= 1
                } else {
                    np = (*np).drnext;
                    break;
                }
            }
        }
        if !(np != rootent) {
            break;
        }
    }
    return 0 as libc::c_int;
}
/*
 * The entry whose number of levels in a directory hierarchy is
 * large than eight relocate to rr_move directory.
 */
unsafe extern "C" fn isoent_rr_move_dir(
    mut a: *mut archive_write,
    mut rr_moved: *mut *mut isoent,
    mut curent: *mut isoent,
    mut newent: *mut *mut isoent,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut rrmoved: *mut isoent = 0 as *mut isoent;
    let mut mvent: *mut isoent = 0 as *mut isoent;
    let mut np: *mut isoent = 0 as *mut isoent;
    rrmoved = *rr_moved;
    if rrmoved.is_null() {
        let mut rootent: *mut isoent = (*iso9660).primary.rootent;
        /* There isn't rr_move entry.
         * Create rr_move entry and insert it into the root entry.
         */
        rrmoved = isoent_create_virtual_dir(
            a,
            iso9660,
            b"rr_moved\x00" as *const u8 as *const libc::c_char,
        );
        if rrmoved.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Add "rr_moved" entry to the root entry. */
        isoent_add_child_head(rootent, rrmoved);
        archive_entry_set_nlink(
            (*(*rootent).file).entry,
            archive_entry_nlink((*(*rootent).file).entry)
                .wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        /* Register "rr_moved" entry to second level pathtable. */
        path_table_add_entry(
            &mut *(*iso9660).primary.pathtbl.offset(1 as libc::c_int as isize),
            rrmoved,
        );
        /* Save rr_moved. */
        *rr_moved = rrmoved
    }
    /*
     * Make a clone of curent which is going to be relocated
     * to rr_moved.
     */
    mvent = isoent_clone(curent);
    if mvent.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* linking..  and use for creating "CL", "PL" and "RE" */
    (*mvent).rr_parent = (*curent).parent;
    (*curent).rr_child = mvent;
    /*
     * Move subdirectories from the curent to mvent
     */
    if !(*curent).children.first.is_null() {
        *(*mvent).children.last = (*curent).children.first;
        (*mvent).children.last = (*curent).children.last
    }
    np = (*mvent).children.first;
    while !np.is_null() {
        (*np).parent = mvent;
        np = (*np).chnext
    }
    (*mvent).children.cnt = (*curent).children.cnt;
    (*curent).children.cnt = 0 as libc::c_int;
    (*curent).children.first = NULL as *mut isoent;
    (*curent).children.last = &mut (*curent).children.first;
    if !(*curent).subdirs.first.is_null() {
        *(*mvent).subdirs.last = (*curent).subdirs.first;
        (*mvent).subdirs.last = (*curent).subdirs.last
    }
    (*mvent).subdirs.cnt = (*curent).subdirs.cnt;
    (*curent).subdirs.cnt = 0 as libc::c_int;
    (*curent).subdirs.first = NULL as *mut isoent;
    (*curent).subdirs.last = &mut (*curent).subdirs.first;
    /*
     * The mvent becomes a child of the rr_moved entry.
     */
    isoent_add_child_tail(rrmoved, mvent);
    archive_entry_set_nlink(
        (*(*rrmoved).file).entry,
        archive_entry_nlink((*(*rrmoved).file).entry)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    /*
     * This entry which relocated to the rr_moved directory
     * has to set the flag as a file.
     * See also RRIP 4.1.5.1 Description of the "CL" System Use Entry.
     */
    (*curent).set_dir(0 as libc::c_int);
    *newent = mvent;
    return 0 as libc::c_int;
}
unsafe extern "C" fn isoent_rr_move(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut pt: *mut path_table = 0 as *mut path_table;
    let mut rootent: *mut isoent = 0 as *mut isoent;
    let mut rr_moved: *mut isoent = 0 as *mut isoent;
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut last: *mut isoent = 0 as *mut isoent;
    let mut r: libc::c_int = 0;
    pt = &mut *(*iso9660)
        .primary
        .pathtbl
        .offset((MAX_DEPTH - 1 as libc::c_int) as isize) as *mut path_table;
    /* There aren't level 8 directories reaching a deeper level. */
    if (*pt).cnt == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    rootent = (*iso9660).primary.rootent;
    /* If "rr_moved" directory is already existing,
     * we have to use it. */
    rr_moved = isoent_find_child(rootent, b"rr_moved\x00" as *const u8 as *const libc::c_char);
    if !rr_moved.is_null() && rr_moved != (*rootent).children.first {
        /*
         * It's necessary that rr_move is the first entry
         * of the root.
         */
        /* Remove "rr_moved" entry from children chain. */
        isoent_remove_child(rootent, rr_moved);
        /* Add "rr_moved" entry into the head of children chain. */
        isoent_add_child_head(rootent, rr_moved);
    }
    /*
     * Check level 8 path_table.
     * If find out sub directory entries, that entries move to rr_move.
     */
    np = (*pt).first;
    while !np.is_null() {
        last = path_table_last_entry(pt);
        while !np.is_null() {
            let mut mvent: *mut isoent = 0 as *mut isoent;
            let mut newent: *mut isoent = 0 as *mut isoent;
            if !((*np).dir() == 0) {
                mvent = (*np).subdirs.first;
                while !mvent.is_null() {
                    r = isoent_rr_move_dir(a, &mut rr_moved, mvent, &mut newent);
                    if r < 0 as libc::c_int {
                        return r;
                    }
                    isoent_collect_dirs(&mut (*iso9660).primary, newent, 2 as libc::c_int);
                    mvent = (*mvent).drnext
                }
            }
            np = (*np).ptnext
        }
        /* If new entries are added to level 8 path_talbe,
         * its sub directory entries move to rr_move too.
         */
        np = (*last).ptnext
    }
    return 0 as libc::c_int;
}
/*
 * This comparing rule is according to ISO9660 Standard 6.9.1
 */
unsafe extern "C" fn _compare_path_table(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> libc::c_int {
    let mut p1: *const isoent = 0 as *const isoent;
    let mut p2: *const isoent = 0 as *const isoent;
    let mut s1: *const libc::c_char = 0 as *const libc::c_char;
    let mut s2: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmp: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    p1 = *(v1 as uintptr_t as *mut *const isoent);
    p2 = *(v2 as uintptr_t as *mut *const isoent);
    /* Compare parent directory number */
    cmp = (*(*p1).parent).dir_number - (*(*p2).parent).dir_number;
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    /* Compare identifier */
    s1 = (*p1).identifier;
    s2 = (*p2).identifier;
    l = (*p1).ext_off;
    if l > (*p2).ext_off {
        l = (*p2).ext_off
    }
    cmp = strncmp(s1, s2, l as libc::c_ulong);
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    if (*p1).ext_off < (*p2).ext_off {
        s2 = s2.offset(l as isize);
        l = (*p2).ext_off - (*p1).ext_off;
        loop {
            let fresh40 = l;
            l = l - 1;
            if !(fresh40 != 0) {
                break;
            }
            let fresh41 = s2;
            s2 = s2.offset(1);
            if 0x20 as libc::c_int != *fresh41 as libc::c_int {
                return 0x20 as libc::c_int
                    - *(s2.offset(-(1 as libc::c_int as isize)) as *const libc::c_uchar)
                        as libc::c_int;
            }
        }
    } else if (*p1).ext_off > (*p2).ext_off {
        s1 = s1.offset(l as isize);
        l = (*p1).ext_off - (*p2).ext_off;
        loop {
            let fresh42 = l;
            l = l - 1;
            if !(fresh42 != 0) {
                break;
            }
            let fresh43 = s1;
            s1 = s1.offset(1);
            if 0x20 as libc::c_int != *fresh43 as libc::c_int {
                return *(s1.offset(-(1 as libc::c_int as isize)) as *const libc::c_uchar)
                    as libc::c_int
                    - 0x20 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _compare_path_table_joliet(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> libc::c_int {
    let mut p1: *const isoent = 0 as *const isoent;
    let mut p2: *const isoent = 0 as *const isoent;
    let mut s1: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut s2: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut cmp: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    p1 = *(v1 as uintptr_t as *mut *const isoent);
    p2 = *(v2 as uintptr_t as *mut *const isoent);
    /* Compare parent directory number */
    cmp = (*(*p1).parent).dir_number - (*(*p2).parent).dir_number;
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    /* Compare identifier */
    s1 = (*p1).identifier as *const libc::c_uchar;
    s2 = (*p2).identifier as *const libc::c_uchar;
    l = (*p1).ext_off;
    if l > (*p2).ext_off {
        l = (*p2).ext_off
    }
    cmp = memcmp(
        s1 as *const libc::c_void,
        s2 as *const libc::c_void,
        l as libc::c_ulong,
    );
    if cmp != 0 as libc::c_int {
        return cmp;
    }
    if (*p1).ext_off < (*p2).ext_off {
        s2 = s2.offset(l as isize);
        l = (*p2).ext_off - (*p1).ext_off;
        loop {
            let fresh44 = l;
            l = l - 1;
            if !(fresh44 != 0) {
                break;
            }
            let fresh45 = s2;
            s2 = s2.offset(1);
            if 0 as libc::c_int != *fresh45 as libc::c_int {
                return -(*s2.offset(-(1 as libc::c_int as isize)) as libc::c_int);
            }
        }
    } else if (*p1).ext_off > (*p2).ext_off {
        s1 = s1.offset(l as isize);
        l = (*p1).ext_off - (*p2).ext_off;
        loop {
            let fresh46 = l;
            l = l - 1;
            if !(fresh46 != 0) {
                break;
            }
            let fresh47 = s1;
            s1 = s1.offset(1);
            if 0 as libc::c_int != *fresh47 as libc::c_int {
                return *s1.offset(-(1 as libc::c_int as isize)) as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn path_table_add_entry(mut pathtbl: *mut path_table, mut ent: *mut isoent) {
    (*ent).ptnext = NULL as *mut isoent;
    *(*pathtbl).last = ent;
    (*pathtbl).last = &mut (*ent).ptnext;
    (*pathtbl).cnt += 1;
}
#[inline]
unsafe extern "C" fn path_table_last_entry(mut pathtbl: *mut path_table) -> *mut isoent {
    if (*pathtbl).first.is_null() {
        return 0 as *mut isoent;
    }
    return ((*pathtbl).last as *mut libc::c_char).offset(-(128 as libc::c_ulong as isize))
        as *mut libc::c_void as *mut isoent;
}
/*
 * Sort directory entries in path_table
 * and assign directory number to each entries.
 */
unsafe extern "C" fn isoent_make_path_table_2(
    mut a: *mut archive_write,
    mut vdd: *mut vdd,
    mut depth: libc::c_int,
    mut dir_number: *mut libc::c_int,
) -> libc::c_int {
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut enttbl: *mut *mut isoent = 0 as *mut *mut isoent;
    let mut pt: *mut path_table = 0 as *mut path_table;
    let mut i: libc::c_int = 0;
    pt = &mut *(*vdd).pathtbl.offset(depth as isize) as *mut path_table;
    if (*pt).cnt == 0 as libc::c_int {
        (*pt).sorted = NULL as *mut *mut isoent;
        return 0 as libc::c_int;
    }
    enttbl = malloc(
        ((*pt).cnt as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut isoent>() as libc::c_ulong),
    ) as *mut *mut isoent;
    if enttbl.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*pt).sorted = enttbl;
    np = (*pt).first;
    while !np.is_null() {
        let fresh48 = enttbl;
        enttbl = enttbl.offset(1);
        *fresh48 = np;
        np = (*np).ptnext
    }
    enttbl = (*pt).sorted;
    match (*vdd).vdd_type as libc::c_uint {
        0 | 2 => {
            qsort(
                enttbl as *mut libc::c_void,
                (*pt).cnt as size_t,
                ::std::mem::size_of::<*mut isoent>() as libc::c_ulong,
                ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    __compar_fn_t,
                >(Some(
                    _compare_path_table
                        as unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                )),
            );
        }
        1 => {
            qsort(
                enttbl as *mut libc::c_void,
                (*pt).cnt as size_t,
                ::std::mem::size_of::<*mut isoent>() as libc::c_ulong,
                ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                    __compar_fn_t,
                >(Some(
                    _compare_path_table_joliet
                        as unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                )),
            );
        }
        _ => {}
    }
    i = 0 as libc::c_int;
    while i < (*pt).cnt {
        let fresh49 = *dir_number;
        *dir_number = *dir_number + 1;
        (**enttbl.offset(i as isize)).dir_number = fresh49;
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn isoent_alloc_path_table(
    mut a: *mut archive_write,
    mut vdd: *mut vdd,
    mut max_depth: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    (*vdd).max_depth = max_depth;
    (*vdd).pathtbl = malloc(
        (::std::mem::size_of::<path_table>() as libc::c_ulong)
            .wrapping_mul((*vdd).max_depth as libc::c_ulong),
    ) as *mut path_table;
    if (*vdd).pathtbl.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*vdd).max_depth {
        let ref mut fresh50 = (*(*vdd).pathtbl.offset(i as isize)).first;
        *fresh50 = NULL as *mut isoent;
        let ref mut fresh51 = (*(*vdd).pathtbl.offset(i as isize)).last;
        *fresh51 = &mut (*(*vdd).pathtbl.offset(i as isize)).first;
        let ref mut fresh52 = (*(*vdd).pathtbl.offset(i as isize)).sorted;
        *fresh52 = NULL as *mut *mut isoent;
        (*(*vdd).pathtbl.offset(i as isize)).cnt = 0 as libc::c_int;
        i += 1
    }
    return 0 as libc::c_int;
}
/*
 * Make Path Tables
 */
unsafe extern "C" fn isoent_make_path_table(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut depth: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut dir_number: libc::c_int = 0;
    /*
     * Init Path Table.
     */
    if (*iso9660).dircnt_max >= MAX_DEPTH
        && ((*iso9660).opt.limit_depth() == 0
            || (*iso9660).opt.iso_level() as libc::c_int == 4 as libc::c_int)
    {
        r = isoent_alloc_path_table(
            a,
            &mut (*iso9660).primary,
            (*iso9660).dircnt_max + 1 as libc::c_int,
        )
    } else {
        /* The number of levels in the hierarchy cannot exceed
         * eight. */
        r = isoent_alloc_path_table(a, &mut (*iso9660).primary, MAX_DEPTH)
    }
    if r < 0 as libc::c_int {
        return r;
    }
    if (*iso9660).opt.joliet() != 0 {
        r = isoent_alloc_path_table(
            a,
            &mut (*iso9660).joliet,
            (*iso9660).dircnt_max + 1 as libc::c_int,
        );
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /* Step 0.
     * - Collect directories for primary and joliet.
     */
    isoent_collect_dirs(
        &mut (*iso9660).primary,
        NULL as *mut isoent,
        0 as libc::c_int,
    );
    if (*iso9660).opt.joliet() != 0 {
        isoent_collect_dirs(
            &mut (*iso9660).joliet,
            NULL as *mut isoent,
            0 as libc::c_int,
        );
    }
    /*
     * Rockridge; move deeper depth directories to rr_moved.
     */
    if (*iso9660).opt.rr() != 0 {
        r = isoent_rr_move(a);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /* Update nlink. */
    isofile_connect_hardlink_files(iso9660);
    /* Step 1.
     * - Renew a value of the depth of that directories.
     * - Resolve hardlinks.
     * - Convert pathnames to ISO9660 name or UCS2(joliet).
     * - Sort files by each directory.
     */
    r = isoent_traverse_tree(a, &mut (*iso9660).primary);
    if r < 0 as libc::c_int {
        return r;
    }
    if (*iso9660).opt.joliet() != 0 {
        r = isoent_traverse_tree(a, &mut (*iso9660).joliet);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /* Step 2.
     * - Sort directories.
     * - Assign all directory number.
     */
    dir_number = 1 as libc::c_int;
    depth = 0 as libc::c_int;
    while depth < (*iso9660).primary.max_depth {
        r = isoent_make_path_table_2(a, &mut (*iso9660).primary, depth, &mut dir_number);
        if r < 0 as libc::c_int {
            return r;
        }
        depth += 1
    }
    if (*iso9660).opt.joliet() != 0 {
        dir_number = 1 as libc::c_int;
        depth = 0 as libc::c_int;
        while depth < (*iso9660).joliet.max_depth {
            r = isoent_make_path_table_2(a, &mut (*iso9660).joliet, depth, &mut dir_number);
            if r < 0 as libc::c_int {
                return r;
            }
            depth += 1
        }
    }
    if (*iso9660).opt.limit_dirs() as libc::c_int != 0 && dir_number > 0xffff as libc::c_int {
        /*
         * Maximum number of directories is 65535(0xffff)
         * doe to size(16bit) of Parent Directory Number of
         * the Path Table.
         * See also ISO9660 Standard 9.4.
         */
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Too many directories(%d) over 65535.\x00" as *const u8 as *const libc::c_char,
            dir_number,
        );
        return -(30 as libc::c_int);
    }
    /* Get the size of the Path Table. */
    calculate_path_table_size(&mut (*iso9660).primary);
    if (*iso9660).opt.joliet() != 0 {
        calculate_path_table_size(&mut (*iso9660).joliet);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn isoent_find_out_boot_file(
    mut a: *mut archive_write,
    mut rootent: *mut isoent,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    /* Find a isoent of the boot file. */
    (*iso9660).el_torito.boot = isoent_find_entry(rootent, (*iso9660).el_torito.boot_filename.s);
    if (*iso9660).el_torito.boot.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Can\'t find the boot image file ``%s\'\'\x00" as *const u8 as *const libc::c_char,
            (*iso9660).el_torito.boot_filename.s,
        );
        return -(30 as libc::c_int);
    }
    (*(*(*iso9660).el_torito.boot).file).boot = BOOT_IMAGE;
    return 0 as libc::c_int;
}
unsafe extern "C" fn isoent_create_boot_catalog(
    mut a: *mut archive_write,
    mut rootent: *mut isoent,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut file: *mut isofile = 0 as *mut isofile;
    let mut isoent: *mut isoent = 0 as *mut isoent;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    /* UNUSED */
    /*
     * Create the entry which is the "boot.catalog" file.
     */
    file = isofile_new(a, NULL as *mut archive_entry);
    if file.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    archive_entry_set_pathname((*file).entry, (*iso9660).el_torito.catalog_filename.s);
    archive_entry_set_size((*file).entry, LOGICAL_BLOCK_SIZE as la_int64_t);
    archive_entry_set_mtime(
        (*file).entry,
        (*iso9660).birth_time,
        0 as libc::c_int as libc::c_long,
    );
    archive_entry_set_atime(
        (*file).entry,
        (*iso9660).birth_time,
        0 as libc::c_int as libc::c_long,
    );
    archive_entry_set_ctime(
        (*file).entry,
        (*iso9660).birth_time,
        0 as libc::c_int as libc::c_long,
    );
    archive_entry_set_uid((*file).entry, getuid() as la_int64_t);
    archive_entry_set_gid((*file).entry, getgid() as la_int64_t);
    archive_entry_set_mode(
        (*file).entry,
        AE_IFREG as mode_t | 0o444 as libc::c_int as libc::c_uint,
    );
    archive_entry_set_nlink((*file).entry, 1 as libc::c_int as libc::c_uint);
    if isofile_gen_utility_names(a, file) < ARCHIVE_WARN {
        isofile_free(file);
        return -(30 as libc::c_int);
    }
    (*file).boot = BOOT_CATALOG;
    (*file).content.size = LOGICAL_BLOCK_SIZE as int64_t;
    isofile_add_entry(iso9660, file);
    isoent = isoent_new(file);
    if isoent.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*isoent).set_virtual_0(1 as libc::c_int);
    /* Add the "boot.catalog" entry into tree */
    if isoent_tree(a, &mut isoent) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*iso9660).el_torito.catalog = isoent;
    /*
     * Get a boot media type.
     */
    match (*iso9660).opt.boot_type() as libc::c_int {
        OPT_BOOT_TYPE_NO_EMU => {
            (*iso9660).el_torito.media_type = BOOT_MEDIA_NO_EMULATION as libc::c_uchar
        }
        OPT_BOOT_TYPE_HARD_DISK => {
            (*iso9660).el_torito.media_type = BOOT_MEDIA_HARD_DISK as libc::c_uchar
        }
        OPT_BOOT_TYPE_FD => {
            entry = (*(*(*iso9660).el_torito.boot).file).entry;
            if archive_entry_size(entry) <= FD_1_2M_SIZE as libc::c_long {
                (*iso9660).el_torito.media_type = BOOT_MEDIA_1_2M_DISKETTE as libc::c_uchar
            } else if archive_entry_size(entry) <= FD_1_44M_SIZE as libc::c_long {
                (*iso9660).el_torito.media_type = BOOT_MEDIA_1_44M_DISKETTE as libc::c_uchar
            } else if archive_entry_size(entry) <= FD_2_88M_SIZE as libc::c_long {
                (*iso9660).el_torito.media_type = BOOT_MEDIA_2_88M_DISKETTE as libc::c_uchar
            } else {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Boot image file(``%s\'\') size is too big for fd type.\x00" as *const u8
                        as *const libc::c_char,
                    (*iso9660).el_torito.boot_filename.s,
                );
                return -(30 as libc::c_int);
            }
        }
        OPT_BOOT_TYPE_AUTO | _ => {
            /* Try detecting a media type of the boot image. */
            entry = (*(*(*iso9660).el_torito.boot).file).entry;
            if archive_entry_size(entry) == FD_1_2M_SIZE as libc::c_long {
                (*iso9660).el_torito.media_type = BOOT_MEDIA_1_2M_DISKETTE as libc::c_uchar
            } else if archive_entry_size(entry) == FD_1_44M_SIZE as libc::c_long {
                (*iso9660).el_torito.media_type = BOOT_MEDIA_1_44M_DISKETTE as libc::c_uchar
            } else if archive_entry_size(entry) == FD_2_88M_SIZE as libc::c_long {
                (*iso9660).el_torito.media_type = BOOT_MEDIA_2_88M_DISKETTE as libc::c_uchar
            } else {
                /* We cannot decide whether the boot image is
                 * hard-disk. */
                (*iso9660).el_torito.media_type = BOOT_MEDIA_NO_EMULATION as libc::c_uchar
            }
        }
    }
    /*
     * Get a system type.
     * TODO: `El Torito' specification says "A copy of byte 5 from the
     *       Partition Table found in the boot image".
     */
    (*iso9660).el_torito.system_type = 0 as libc::c_int as libc::c_uchar;
    /*
     * Get an ID.
     */
    if (*iso9660).opt.publisher() != 0 {
        (*iso9660).el_torito.id.length = 0 as libc::c_int as size_t;
        archive_string_concat(
            &mut (*iso9660).el_torito.id,
            &mut (*iso9660).publisher_identifier,
        );
    }
    return 0 as libc::c_int;
}
/*
 * If a media type is floppy, return its image size.
 * otherwise return 0.
 */
unsafe extern "C" fn fd_boot_image_size(mut media_type: libc::c_int) -> size_t {
    match media_type {
        BOOT_MEDIA_1_2M_DISKETTE => return (1024 as libc::c_int * 1200 as libc::c_int) as size_t,
        BOOT_MEDIA_1_44M_DISKETTE => return (1024 as libc::c_int * 1440 as libc::c_int) as size_t,
        BOOT_MEDIA_2_88M_DISKETTE => return (1024 as libc::c_int * 2880 as libc::c_int) as size_t,
        _ => return 0 as libc::c_int as size_t,
    };
}
/*
 * Make a boot catalog image data.
 */
unsafe extern "C" fn make_boot_catalog(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut block: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sum: uint16_t = 0;
    let mut wp: *mut uint16_t = 0 as *mut uint16_t;
    block = wb_buffptr(a);
    memset(
        block as *mut libc::c_void,
        0 as libc::c_int,
        LOGICAL_BLOCK_SIZE as libc::c_ulong,
    );
    p = block;
    /*
     * Validation Entry
     */
    /* Header ID */
    *p.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_uchar;
    /* Platform ID */
    *p.offset(1 as libc::c_int as isize) = (*iso9660).el_torito.platform_id;
    /* Reserved */
    let ref mut fresh53 = *p.offset(3 as libc::c_int as isize);
    *fresh53 = 0 as libc::c_int as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) = *fresh53;
    /* ID */
    if (*iso9660).el_torito.id.length > 0 as libc::c_int as libc::c_ulong {
        strncpy(
            (p as *mut libc::c_char).offset(4 as libc::c_int as isize),
            (*iso9660).el_torito.id.s,
            23 as libc::c_int as libc::c_ulong,
        );
    }
    *p.offset(27 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    /* Checksum */
    let ref mut fresh54 = *p.offset(29 as libc::c_int as isize);
    *fresh54 = 0 as libc::c_int as libc::c_uchar;
    *p.offset(28 as libc::c_int as isize) = *fresh54;
    /* Key */
    *p.offset(30 as libc::c_int as isize) = 0x55 as libc::c_int as libc::c_uchar;
    *p.offset(31 as libc::c_int as isize) = 0xaa as libc::c_int as libc::c_uchar;
    sum = 0 as libc::c_int as uint16_t;
    wp = block as *mut uint16_t;
    while wp < &mut *block.offset(32 as libc::c_int as isize) as *mut libc::c_uchar as *mut uint16_t
    {
        let fresh55 = wp;
        wp = wp.offset(1);
        sum = (sum as libc::c_int + archive_le16dec(fresh55 as *const libc::c_void) as libc::c_int)
            as uint16_t
    }
    set_num_721(
        &mut *block.offset(28 as libc::c_int as isize),
        (!(sum as libc::c_int) + 1 as libc::c_int) as uint16_t,
    );
    /*
     * Initial/Default Entry
     */
    p = &mut *block.offset(32 as libc::c_int as isize) as *mut libc::c_uchar;
    /* Boot Indicator */
    *p.offset(0 as libc::c_int as isize) = 0x88 as libc::c_int as libc::c_uchar;
    /* Boot media type */
    *p.offset(1 as libc::c_int as isize) = (*iso9660).el_torito.media_type;
    /* Load Segment */
    if (*iso9660).el_torito.media_type as libc::c_int == BOOT_MEDIA_NO_EMULATION {
        set_num_721(
            &mut *p.offset(2 as libc::c_int as isize),
            (*iso9660).el_torito.boot_load_seg,
        );
    } else {
        set_num_721(
            &mut *p.offset(2 as libc::c_int as isize),
            0 as libc::c_int as uint16_t,
        );
    }
    /* System Type */
    *p.offset(4 as libc::c_int as isize) = (*iso9660).el_torito.system_type;
    /* Unused */
    *p.offset(5 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    /* Sector Count */
    if (*iso9660).el_torito.media_type as libc::c_int == BOOT_MEDIA_NO_EMULATION {
        set_num_721(
            &mut *p.offset(6 as libc::c_int as isize),
            (*iso9660).el_torito.boot_load_size,
        );
    } else {
        set_num_721(
            &mut *p.offset(6 as libc::c_int as isize),
            1 as libc::c_int as uint16_t,
        );
    }
    /* Load RBA */
    set_num_731(
        &mut *p.offset(8 as libc::c_int as isize),
        (*(*(*iso9660).el_torito.boot).file).content.location,
    );
    /* Unused */
    memset(
        &mut *p.offset(12 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        0 as libc::c_int,
        20 as libc::c_int as libc::c_ulong,
    );
    return wb_consume(a, LOGICAL_BLOCK_SIZE as size_t);
}
unsafe extern "C" fn setup_boot_information(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut np: *mut isoent = 0 as *mut isoent;
    let mut size: int64_t = 0;
    let mut sum: uint32_t = 0;
    let mut buff: [libc::c_uchar; 4096] = [0; 4096];
    np = (*iso9660).el_torito.boot;
    lseek(
        (*iso9660).temp_fd,
        (*(*np).file).content.offset_of_temp + 64 as libc::c_int as libc::c_long,
        SEEK_SET,
    );
    size = archive_entry_size((*(*np).file).entry) - 64 as libc::c_int as libc::c_long;
    if size <= 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Boot file(%jd) is too small\x00" as *const u8 as *const libc::c_char,
            size + 64 as libc::c_int as libc::c_long,
        );
        return -(30 as libc::c_int);
    }
    sum = 0 as libc::c_int as uint32_t;
    while size > 0 as libc::c_int as libc::c_long {
        let mut rsize: size_t = 0;
        let mut i: ssize_t = 0;
        let mut rs: ssize_t = 0;
        if size > ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong as int64_t {
            rsize = ::std::mem::size_of::<[libc::c_uchar; 4096]>() as libc::c_ulong
        } else {
            rsize = size as size_t
        }
        rs = read(
            (*iso9660).temp_fd,
            buff.as_mut_ptr() as *mut libc::c_void,
            rsize,
        );
        if rs <= 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t read temporary file(%jd)\x00" as *const u8 as *const libc::c_char,
                rs,
            );
            return -(30 as libc::c_int);
        }
        i = 0 as libc::c_int as ssize_t;
        while i < rs {
            sum = (sum as libc::c_uint).wrapping_add(archive_le32dec(
                buff.as_mut_ptr().offset(i as isize) as *const libc::c_void,
            )) as uint32_t as uint32_t;
            i += 4 as libc::c_int as libc::c_long
        }
        size -= rs
    }
    /* Set the location of Primary Volume Descriptor. */
    set_num_731(buff.as_mut_ptr(), SYSTEM_AREA_BLOCK as uint32_t);
    /* Set the location of the boot file. */
    set_num_731(
        buff.as_mut_ptr().offset(4 as libc::c_int as isize),
        (*(*np).file).content.location,
    );
    /* Set the size of the boot file. */
    size = fd_boot_image_size((*iso9660).el_torito.media_type as libc::c_int) as int64_t;
    if size == 0 as libc::c_int as libc::c_long {
        size = archive_entry_size((*(*np).file).entry)
    }
    set_num_731(
        buff.as_mut_ptr().offset(8 as libc::c_int as isize),
        size as uint32_t,
    );
    /* Set the sum of the boot file. */
    set_num_731(buff.as_mut_ptr().offset(12 as libc::c_int as isize), sum);
    /* Clear reserved bytes. */
    memset(
        buff.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        40 as libc::c_int as libc::c_ulong,
    );
    /* Overwrite the boot file. */
    lseek(
        (*iso9660).temp_fd,
        (*(*np).file).content.offset_of_temp + 8 as libc::c_int as libc::c_long,
        SEEK_SET,
    );
    return write_to_temp(
        a,
        buff.as_mut_ptr() as *const libc::c_void,
        56 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn zisofs_init_zstream(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut r: libc::c_int = 0;
    (*iso9660).zisofs.stream.next_in = NULL as *mut Bytef;
    (*iso9660).zisofs.stream.avail_in = 0 as libc::c_int as uInt;
    (*iso9660).zisofs.stream.total_in = 0 as libc::c_int as uLong;
    (*iso9660).zisofs.stream.total_out = 0 as libc::c_int as uLong;
    if (*iso9660).zisofs.stream_valid != 0 {
        r = deflateReset(&mut (*iso9660).zisofs.stream)
    } else {
        r = deflateInit_(
            &mut (*iso9660).zisofs.stream,
            (*iso9660).zisofs.compression_level,
            ZLIB_VERSION.as_ptr(),
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        );
        (*iso9660).zisofs.stream_valid = 1 as libc::c_int
    }
    match r {
        Z_OK => {}
        -4 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Internal error initializing compression library\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        -6 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal error initializing compression library: invalid library version\x00"
                    as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        -2 | _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal error initializing compression library: invalid setup parameter\x00"
                    as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
/* HAVE_ZLIB_H */
unsafe extern "C" fn zisofs_init(mut a: *mut archive_write, mut file: *mut isofile) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut tsize: uint64_t = 0;
    let mut _ceil: size_t = 0;
    let mut bpsize: size_t = 0;
    let mut r: libc::c_int = 0;
    (*iso9660).zisofs.set_detect_magic(0 as libc::c_int);
    (*iso9660).zisofs.set_making(0 as libc::c_int);
    if (*iso9660).opt.rr() == 0 || (*iso9660).opt.zisofs() == 0 {
        return 0 as libc::c_int;
    }
    if archive_entry_size((*file).entry) >= 24 as libc::c_int as libc::c_long
        && (archive_entry_size((*file).entry) as libc::c_longlong) < MULTI_EXTENT_SIZE
    {
        /* Acceptable file size for zisofs. */
        (*iso9660).zisofs.set_detect_magic(1 as libc::c_int);
        (*iso9660).zisofs.magic_cnt = 0 as libc::c_int
    }
    if (*iso9660).zisofs.detect_magic() == 0 {
        return 0 as libc::c_int;
    }
    /* The number of Logical Blocks which uncompressed data
     * will use in iso-image file is the same as the number of
     * Logical Blocks which zisofs(compressed) data will use
     * in ISO-image file. It won't reduce iso-image file size. */
    if archive_entry_size((*file).entry) <= LOGICAL_BLOCK_SIZE as libc::c_long {
        return 0 as libc::c_int;
    }
    /* Initialize compression library */
    r = zisofs_init_zstream(a);
    if r != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Mark file->zisofs to create RRIP 'ZF' Use Entry. */
    (*file).zisofs.header_size = (ZF_HEADER_SIZE >> 2 as libc::c_int) as libc::c_uchar;
    (*file).zisofs.log2_bs = ZF_LOG2_BS as libc::c_uchar;
    (*file).zisofs.uncompressed_size = archive_entry_size((*file).entry) as uint32_t;
    /* Calculate a size of Block Pointers of zisofs. */
    _ceil = ((*file).zisofs.uncompressed_size as libc::c_ulong)
        .wrapping_add(ZF_BLOCK_SIZE)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        >> (*file).zisofs.log2_bs as libc::c_int;
    (*iso9660).zisofs.block_pointers_cnt = _ceil as libc::c_int + 1 as libc::c_int;
    (*iso9660).zisofs.block_pointers_idx = 0 as libc::c_int;
    /* Ensure a buffer size used for Block Pointers */
    bpsize = ((*iso9660).zisofs.block_pointers_cnt as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    if (*iso9660).zisofs.block_pointers_allocated < bpsize {
        free((*iso9660).zisofs.block_pointers as *mut libc::c_void);
        (*iso9660).zisofs.block_pointers = malloc(bpsize) as *mut uint32_t;
        if (*iso9660).zisofs.block_pointers.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*iso9660).zisofs.block_pointers_allocated = bpsize
    }
    /*
     * Skip zisofs header and Block Pointers, which we will write
     * after all compressed data of a file written to the temporary
     * file.
     */
    tsize = (ZF_HEADER_SIZE as libc::c_ulong).wrapping_add(bpsize);
    if write_null(a, tsize) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /*
     * Initialize some variables to make zisofs.
     */
    archive_le32enc(
        &mut *(*iso9660)
            .zisofs
            .block_pointers
            .offset(0 as libc::c_int as isize) as *mut uint32_t as *mut libc::c_void,
        tsize as uint32_t,
    );
    (*iso9660).zisofs.remaining = (*file).zisofs.uncompressed_size as int64_t;
    (*iso9660).zisofs.set_making(1 as libc::c_int);
    (*iso9660).zisofs.set_allzero(1 as libc::c_int);
    (*iso9660).zisofs.block_offset = tsize as int64_t;
    (*iso9660).zisofs.total_size = tsize as int64_t;
    (*(*(*iso9660).cur_file).cur_content).size = tsize as int64_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn zisofs_detect_magic(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut file: *mut isofile = (*iso9660).cur_file;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut endp: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut magic_buff: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut uncompressed_size: uint32_t = 0;
    let mut header_size: libc::c_uchar = 0;
    let mut log2_bs: libc::c_uchar = 0;
    let mut _ceil: size_t = 0;
    let mut doff: size_t = 0;
    let mut bst: uint32_t = 0;
    let mut bed: uint32_t = 0;
    let mut magic_max: libc::c_int = 0;
    let mut entry_size: int64_t = 0;
    entry_size = archive_entry_size((*file).entry);
    if ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong as int64_t > entry_size {
        magic_max = entry_size as libc::c_int
    } else {
        magic_max = ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong as libc::c_int
    }
    if (*iso9660).zisofs.magic_cnt == 0 as libc::c_int && s >= magic_max as size_t {
        /* It's unnecessary we copy buffer. */
        magic_buff = buff as *const libc::c_uchar
    } else {
        if (*iso9660).zisofs.magic_cnt < magic_max {
            let mut l: size_t = 0;
            l = (::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong)
                .wrapping_sub((*iso9660).zisofs.magic_cnt as libc::c_ulong);
            if l > s {
                l = s
            }
            memcpy(
                (*iso9660)
                    .zisofs
                    .magic_buffer
                    .as_mut_ptr()
                    .offset((*iso9660).zisofs.magic_cnt as isize)
                    as *mut libc::c_void,
                buff,
                l,
            );
            (*iso9660).zisofs.magic_cnt += l as libc::c_int;
            if (*iso9660).zisofs.magic_cnt < magic_max {
                return;
            }
        }
        magic_buff = (*iso9660).zisofs.magic_buffer.as_mut_ptr()
    }
    (*iso9660).zisofs.set_detect_magic(0 as libc::c_int);
    p = magic_buff;
    /* Check the magic code of zisofs. */
    if memcmp(
        p as *const libc::c_void,
        zisofs_magic.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        /* This is not zisofs file which made by mkzftree. */
        return;
    }
    p = p.offset(::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as isize);
    /* Read a zisofs header. */
    uncompressed_size = archive_le32dec(p as *const libc::c_void); /* Invalid or not supported header. */
    header_size = *p.offset(4 as libc::c_int as isize);
    log2_bs = *p.offset(5 as libc::c_int as isize);
    if uncompressed_size < 24 as libc::c_int as libc::c_uint
        || header_size as libc::c_int != 4 as libc::c_int
        || log2_bs as libc::c_int > 30 as libc::c_int
        || (log2_bs as libc::c_int) < 7 as libc::c_int
    {
        return;
    }
    /* Calculate a size of Block Pointers of zisofs. */
    _ceil = (uncompressed_size as libc::c_longlong
        + ((1 as libc::c_longlong) << log2_bs as libc::c_int)
        - 1 as libc::c_int as libc::c_longlong
        >> log2_bs as libc::c_int) as size_t; /* Invalid data. */
    doff = _ceil
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        .wrapping_add(16 as libc::c_int as libc::c_ulong);
    if entry_size < doff as int64_t {
        return;
    }
    /* Check every Block Pointer has valid value. */
    p = magic_buff.offset(16 as libc::c_int as isize); /* Invalid data. */
    endp = magic_buff.offset(magic_max as isize); /* Invalid data. */
    while _ceil != 0 && p.offset(8 as libc::c_int as isize) <= endp {
        bst = archive_le32dec(p as *const libc::c_void);
        if bst as libc::c_ulong != doff {
            return;
        }
        p = p.offset(4 as libc::c_int as isize);
        bed = archive_le32dec(p as *const libc::c_void);
        if bed < bst || bed as libc::c_long > entry_size {
            return;
        }
        doff = (doff as libc::c_ulong).wrapping_add(bed.wrapping_sub(bst) as libc::c_ulong)
            as size_t as size_t;
        _ceil = _ceil.wrapping_sub(1)
    }
    (*file).zisofs.uncompressed_size = uncompressed_size;
    (*file).zisofs.header_size = header_size;
    (*file).zisofs.log2_bs = log2_bs;
    /* Disable making a zisofs image. */
    (*iso9660).zisofs.set_making(0 as libc::c_int);
}
/*
 * Compress data and write it to a temporary file.
 */
unsafe extern "C" fn zisofs_write_to_temp(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut file: *mut isofile = (*iso9660).cur_file;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut zstrm: *mut z_stream = 0 as *mut z_stream;
    let mut avail: size_t = 0;
    let mut csize: size_t = 0;
    let mut flush: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    zstrm = &mut (*iso9660).zisofs.stream;
    (*zstrm).next_out = wb_buffptr(a);
    (*zstrm).avail_out = (*((*a).format_data as *mut iso9660)).wbuff_remaining as uInt;
    b = buff as *const libc::c_uchar;
    loop {
        avail = ZF_BLOCK_SIZE.wrapping_sub((*zstrm).total_in);
        if s < avail {
            avail = s;
            flush = Z_NO_FLUSH
        } else {
            flush = Z_FINISH
        }
        (*iso9660).zisofs.remaining = ((*iso9660).zisofs.remaining as libc::c_ulong)
            .wrapping_sub(avail) as int64_t as int64_t;
        if (*iso9660).zisofs.remaining <= 0 as libc::c_int as libc::c_long {
            flush = Z_FINISH
        }
        (*zstrm).next_in = b as *const libc::c_void as uintptr_t as *mut Bytef;
        (*zstrm).avail_in = avail as uInt;
        /*
         * Check if current data block are all zero.
         */
        if (*iso9660).zisofs.allzero() != 0 {
            let mut nonzero: *const libc::c_uchar = b;
            let mut nonzeroend: *const libc::c_uchar = b.offset(avail as isize);
            while nonzero < nonzeroend {
                let fresh56 = nonzero;
                nonzero = nonzero.offset(1);
                if !(*fresh56 != 0) {
                    continue;
                }
                (*iso9660).zisofs.set_allzero(0 as libc::c_int);
                break;
            }
        }
        b = b.offset(avail as isize);
        s = (s as libc::c_ulong).wrapping_sub(avail) as size_t as size_t;
        /*
         * If current data block are all zero, we do not use
         * compressed data.
         */
        if flush == Z_FINISH
            && (*iso9660).zisofs.allzero() != 0
            && avail.wrapping_add((*zstrm).total_in) == ZF_BLOCK_SIZE
        {
            if (*iso9660).zisofs.block_offset != (*(*file).cur_content).size {
                let mut diff: int64_t = 0;
                r = wb_set_offset(
                    a,
                    (*(*file).cur_content).offset_of_temp + (*iso9660).zisofs.block_offset,
                );
                if r != ARCHIVE_OK {
                    return r;
                }
                diff = (*(*file).cur_content).size - (*iso9660).zisofs.block_offset;
                (*(*file).cur_content).size -= diff;
                (*iso9660).zisofs.total_size -= diff
            }
            (*zstrm).avail_in = 0 as libc::c_int as uInt
        }
        /*
         * Compress file data.
         */
        while (*zstrm).avail_in > 0 as libc::c_int as libc::c_uint {
            csize = (*zstrm).total_out;
            r = deflate(zstrm, flush);
            match r {
                Z_OK | Z_STREAM_END => {
                    csize = (*zstrm).total_out.wrapping_sub(csize);
                    if wb_consume(a, csize) != ARCHIVE_OK {
                        return -(30 as libc::c_int);
                    }
                    (*iso9660).zisofs.total_size = ((*iso9660).zisofs.total_size as libc::c_ulong)
                        .wrapping_add(csize)
                        as int64_t as int64_t;
                    (*(*(*iso9660).cur_file).cur_content).size =
                        ((*(*(*iso9660).cur_file).cur_content).size as libc::c_ulong)
                            .wrapping_add(csize) as int64_t as int64_t;
                    (*zstrm).next_out = wb_buffptr(a);
                    (*zstrm).avail_out =
                        (*((*a).format_data as *mut iso9660)).wbuff_remaining as uInt
                }
                _ => {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Compression failed: deflate() call returned status %d\x00" as *const u8
                            as *const libc::c_char,
                        r,
                    );
                    return -(30 as libc::c_int);
                }
            }
        }
        if flush == Z_FINISH {
            /*
             * Save the information of one zisofs block.
             */
            (*iso9660).zisofs.block_pointers_idx += 1;
            archive_le32enc(
                &mut *(*iso9660)
                    .zisofs
                    .block_pointers
                    .offset((*iso9660).zisofs.block_pointers_idx as isize)
                    as *mut uint32_t as *mut libc::c_void,
                (*iso9660).zisofs.total_size as uint32_t,
            );
            r = zisofs_init_zstream(a);
            if r != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
            (*iso9660).zisofs.set_allzero(1 as libc::c_int);
            (*iso9660).zisofs.block_offset = (*(*file).cur_content).size
        }
        if !(s != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zisofs_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut file: *mut isofile = (*iso9660).cur_file;
    let mut buff: [libc::c_uchar; 16] = [0; 16];
    let mut s: size_t = 0;
    let mut tail: int64_t = 0;
    /* Direct temp file stream to zisofs temp file stream. */
    archive_entry_set_size((*file).entry, (*iso9660).zisofs.total_size);
    /*
     * Save a file pointer which points the end of current zisofs data.
     */
    tail = (((*((*a).format_data as *mut iso9660)).wbuff_offset
        + (LOGICAL_BLOCK_SIZE * 32 as libc::c_int) as libc::c_long) as libc::c_ulong)
        .wrapping_sub((*((*a).format_data as *mut iso9660)).wbuff_remaining) as int64_t;
    /*
     * Make a header.
     *
     * +-----------------+----------------+-----------------+
     * | Header 16 bytes | Block Pointers | Compressed data |
     * +-----------------+----------------+-----------------+
     * 0                16               +X
     * Block Pointers :
     *   4 * (((Uncompressed file size + block_size -1) / block_size) + 1)
     *
     * Write zisofs header.
     *    Magic number
     * +----+----+----+----+----+----+----+----+
     * | 37 | E4 | 53 | 96 | C9 | DB | D6 | 07 |
     * +----+----+----+----+----+----+----+----+
     * 0    1    2    3    4    5    6    7    8
     *
     * +------------------------+------------------+
     * | Uncompressed file size | header_size >> 2 |
     * +------------------------+------------------+
     * 8                       12                 13
     *
     * +-----------------+----------------+
     * | log2 block_size | Reserved(0000) |
     * +-----------------+----------------+
     * 13               14               16
     */
    memcpy(
        buff.as_mut_ptr() as *mut libc::c_void,
        zisofs_magic.as_ptr() as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ); /* Reserved */
    set_num_731(
        buff.as_mut_ptr().offset(8 as libc::c_int as isize),
        (*file).zisofs.uncompressed_size,
    );
    buff[12 as libc::c_int as usize] = (*file).zisofs.header_size;
    buff[13 as libc::c_int as usize] = (*file).zisofs.log2_bs;
    buff[15 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    buff[14 as libc::c_int as usize] = buff[15 as libc::c_int as usize];
    /* Move to the right position to write the header. */
    wb_set_offset(a, (*file).content.offset_of_temp);
    /* Write the header. */
    if wb_write_to_temp(
        a,
        buff.as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as size_t,
    ) != ARCHIVE_OK
    {
        return -(30 as libc::c_int);
    }
    /*
     * Write zisofs Block Pointers.
     */
    s = ((*iso9660).zisofs.block_pointers_cnt as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong);
    if wb_write_to_temp(
        a,
        (*iso9660).zisofs.block_pointers as *const libc::c_void,
        s,
    ) != ARCHIVE_OK
    {
        return -(30 as libc::c_int);
    }
    /* Set a file pointer back to the end of the temporary file. */
    wb_set_offset(a, tail);
    return 0 as libc::c_int;
}
unsafe extern "C" fn zisofs_free(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut ret: libc::c_int = ARCHIVE_OK;
    free((*iso9660).zisofs.block_pointers as *mut libc::c_void);
    if (*iso9660).zisofs.stream_valid != 0 && deflateEnd(&mut (*iso9660).zisofs.stream) != Z_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Failed to clean up compressor\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FATAL
    }
    (*iso9660).zisofs.block_pointers = NULL as *mut uint32_t;
    (*iso9660).zisofs.stream_valid = 0 as libc::c_int;
    return ret;
}
unsafe extern "C" fn zisofs_extract_init(
    mut a: *mut archive_write,
    mut zisofs: *mut zisofs_extract,
    mut p: *const libc::c_uchar,
    mut bytes: size_t,
) -> ssize_t {
    let mut avail: size_t = bytes;
    let mut _ceil: size_t = 0;
    let mut xsize: size_t = 0;
    /* Allocate block pointers buffer. */
    _ceil = (*zisofs)
        .pz_uncompressed_size
        .wrapping_add(((1 as libc::c_int as int64_t) << (*zisofs).pz_log2_bs) as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        >> (*zisofs).pz_log2_bs;
    xsize = _ceil
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong);
    if (*zisofs).block_pointers.is_null() {
        let mut alloc: size_t = (xsize >> 10 as libc::c_int)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            << 10 as libc::c_int;
        (*zisofs).block_pointers = malloc(alloc) as *mut libc::c_uchar;
        if (*zisofs).block_pointers.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory for zisofs decompression\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
    }
    (*zisofs).block_pointers_size = xsize;
    /* Allocate uncompressed data buffer. */
    (*zisofs).uncompressed_buffer_size = (1 as libc::c_ulong) << (*zisofs).pz_log2_bs;
    /*
     * Read the file header, and check the magic code of zisofs.
     */
    if (*zisofs).header_passed() == 0 {
        let mut err: libc::c_int = 0 as libc::c_int;
        if avail < 16 as libc::c_int as libc::c_ulong {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Illegal zisofs file body\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        if memcmp(
            p as *const libc::c_void,
            zisofs_magic.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            err = 1 as libc::c_int
        } else if archive_le32dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_ulong
            != (*zisofs).pz_uncompressed_size
        {
            err = 1 as libc::c_int
        } else if *p.offset(12 as libc::c_int as isize) as libc::c_int != 4 as libc::c_int
            || *p.offset(13 as libc::c_int as isize) as libc::c_int != (*zisofs).pz_log2_bs
        {
            err = 1 as libc::c_int
        }
        if err != 0 {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Illegal zisofs file body\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        avail = (avail as libc::c_ulong).wrapping_sub(16 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        p = p.offset(16 as libc::c_int as isize);
        (*zisofs).set_header_passed(1 as libc::c_int)
    }
    /*
     * Read block pointers.
     */
    if (*zisofs).header_passed() != 0
        && (*zisofs).block_pointers_avail < (*zisofs).block_pointers_size
    {
        xsize = (*zisofs)
            .block_pointers_size
            .wrapping_sub((*zisofs).block_pointers_avail);
        if avail < xsize {
            xsize = avail
        }
        memcpy(
            (*zisofs)
                .block_pointers
                .offset((*zisofs).block_pointers_avail as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            xsize,
        );
        (*zisofs).block_pointers_avail = ((*zisofs).block_pointers_avail as libc::c_ulong)
            .wrapping_add(xsize) as size_t as size_t;
        avail = (avail as libc::c_ulong).wrapping_sub(xsize) as size_t as size_t;
        if (*zisofs).block_pointers_avail == (*zisofs).block_pointers_size {
            /* We've got all block pointers and initialize
             * related variables.	*/
            (*zisofs).block_off = 0 as libc::c_int as size_t;
            (*zisofs).block_avail = 0 as libc::c_int as uint32_t;
            /* Complete a initialization */
            (*zisofs).set_initialized(1 as libc::c_int)
        }
    }
    return avail as ssize_t;
}
unsafe extern "C" fn zisofs_extract(
    mut a: *mut archive_write,
    mut zisofs: *mut zisofs_extract,
    mut p: *const libc::c_uchar,
    mut bytes: size_t,
) -> ssize_t {
    let mut avail: size_t = 0;
    let mut r: libc::c_int = 0;
    if (*zisofs).initialized() == 0 {
        let mut rs: ssize_t = zisofs_extract_init(a, zisofs, p, bytes);
        if rs < 0 as libc::c_int as libc::c_long {
            return rs;
        }
        if (*zisofs).initialized() == 0 {
            /* We need more data. */
            (*zisofs).pz_offset = ((*zisofs).pz_offset as libc::c_uint)
                .wrapping_add(bytes as uint32_t) as uint32_t
                as uint32_t;
            return bytes as ssize_t;
        }
        avail = rs as size_t;
        p = p.offset(bytes.wrapping_sub(avail) as isize)
    } else {
        avail = bytes
    }
    /*
     * Get block offsets from block pointers.
     */
    if (*zisofs).block_avail == 0 as libc::c_int as libc::c_uint {
        let mut bst: uint32_t = 0;
        let mut bed: uint32_t = 0;
        if (*zisofs)
            .block_off
            .wrapping_add(4 as libc::c_int as libc::c_ulong)
            >= (*zisofs).block_pointers_size
        {
            /* There isn't a pair of offsets. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Illegal zisofs block pointers\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        bst = archive_le32dec(
            (*zisofs)
                .block_pointers
                .offset((*zisofs).block_off as isize) as *const libc::c_void,
        );
        if bst as libc::c_ulong
            != ((*zisofs).pz_offset as libc::c_ulong).wrapping_add(bytes.wrapping_sub(avail))
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Illegal zisofs block pointers(cannot seek)\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        bed = archive_le32dec(
            (*zisofs)
                .block_pointers
                .offset((*zisofs).block_off as isize)
                .offset(4 as libc::c_int as isize) as *const libc::c_void,
        );
        if bed < bst {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Illegal zisofs block pointers\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        (*zisofs).block_avail = bed.wrapping_sub(bst);
        (*zisofs).block_off = ((*zisofs).block_off as libc::c_ulong)
            .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        /* Initialize compression library for new block. */
        if (*zisofs).stream_valid != 0 {
            r = inflateReset(&mut (*zisofs).stream)
        } else {
            r = inflateInit_(
                &mut (*zisofs).stream,
                ZLIB_VERSION.as_ptr(),
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            )
        }
        if r != Z_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Can\'t initialize zisofs decompression.\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        (*zisofs).stream_valid = 1 as libc::c_int;
        (*zisofs).stream.total_in = 0 as libc::c_int as uLong;
        (*zisofs).stream.total_out = 0 as libc::c_int as uLong
    }
    /*
     * Make uncompressed data.
     */
    if (*zisofs).block_avail == 0 as libc::c_int as libc::c_uint {
        /*
         * It's basically 32K bytes NUL data.
         */
        let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut size: size_t = 0;
        let mut wsize: size_t = 0;
        size = (*zisofs).uncompressed_buffer_size;
        while size != 0 {
            wb = wb_buffptr(a);
            if size > (*((*a).format_data as *mut iso9660)).wbuff_remaining {
                wsize = (*((*a).format_data as *mut iso9660)).wbuff_remaining
            } else {
                wsize = size
            }
            memset(wb as *mut libc::c_void, 0 as libc::c_int, wsize);
            r = wb_consume(a, wsize);
            if r < 0 as libc::c_int {
                return r as ssize_t;
            }
            size = (size as libc::c_ulong).wrapping_sub(wsize) as size_t as size_t
        }
    } else {
        (*zisofs).stream.next_in = p as *const libc::c_void as uintptr_t as *mut Bytef;
        if avail > (*zisofs).block_avail as libc::c_ulong {
            (*zisofs).stream.avail_in = (*zisofs).block_avail
        } else {
            (*zisofs).stream.avail_in = avail as uInt
        }
        (*zisofs).stream.next_out = wb_buffptr(a);
        (*zisofs).stream.avail_out = (*((*a).format_data as *mut iso9660)).wbuff_remaining as uInt;
        r = inflate(&mut (*zisofs).stream, 0 as libc::c_int);
        match r {
            Z_OK => {}
            Z_STREAM_END => {}
            _ => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"zisofs decompression failed (%d)\x00" as *const u8 as *const libc::c_char,
                    r,
                );
                return -(30 as libc::c_int) as ssize_t;
            }
        }
        avail = (avail as libc::c_ulong).wrapping_sub(
            (*zisofs).stream.next_in.offset_from(p) as libc::c_long as libc::c_ulong,
        ) as size_t as size_t;
        (*zisofs).block_avail = ((*zisofs).block_avail as libc::c_uint).wrapping_sub(
            (*zisofs).stream.next_in.offset_from(p) as libc::c_long as uint32_t,
        ) as uint32_t as uint32_t;
        r = wb_consume(
            a,
            (*((*a).format_data as *mut iso9660))
                .wbuff_remaining
                .wrapping_sub((*zisofs).stream.avail_out as libc::c_ulong),
        );
        if r < 0 as libc::c_int {
            return r as ssize_t;
        }
    }
    (*zisofs).pz_offset = ((*zisofs).pz_offset as libc::c_uint).wrapping_add(bytes as uint32_t)
        as uint32_t as uint32_t;
    return bytes.wrapping_sub(avail) as ssize_t;
}
unsafe extern "C" fn zisofs_rewind_boot_file(mut a: *mut archive_write) -> libc::c_int {
    let mut iso9660: *mut iso9660 = (*a).format_data as *mut iso9660;
    let mut file: *mut isofile = 0 as *mut isofile;
    let mut rbuff: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: ssize_t = 0;
    let mut remaining: size_t = 0;
    let mut rbuff_size: size_t = 0;
    let mut zext: zisofs_extract = zisofs_extract {
        pz_log2_bs: 0,
        pz_uncompressed_size: 0,
        uncompressed_buffer_size: 0,
        initialized_header_passed: [0; 1],
        c2rust_padding: [0; 3],
        pz_offset: 0,
        block_pointers: 0 as *mut libc::c_uchar,
        block_pointers_size: 0,
        block_pointers_avail: 0,
        block_off: 0,
        block_avail: 0,
        stream: z_stream {
            next_in: 0 as *mut Bytef,
            avail_in: 0,
            total_in: 0,
            next_out: 0 as *mut Bytef,
            avail_out: 0,
            total_out: 0,
            msg: 0 as *mut libc::c_char,
            state: 0 as *mut internal_state,
            zalloc: None,
            zfree: None,
            opaque: 0 as *mut libc::c_void,
            data_type: 0,
            adler: 0,
            reserved: 0,
        },
        stream_valid: 0,
    };
    let mut read_offset: int64_t = 0;
    let mut write_offset: int64_t = 0;
    let mut new_offset: int64_t = 0;
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    file = (*(*iso9660).el_torito.boot).file;
    /*
     * There is nothing to do if this boot file does not have
     * zisofs header.
     */
    if (*file).zisofs.header_size as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    /*
     * Uncompress the zisofs'ed file contents.
     */
    memset(
        &mut zext as *mut zisofs_extract as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zisofs_extract>() as libc::c_ulong,
    );
    zext.pz_uncompressed_size = (*file).zisofs.uncompressed_size as uint64_t;
    zext.pz_log2_bs = (*file).zisofs.log2_bs as libc::c_int;
    fd = (*iso9660).temp_fd;
    new_offset =
        (((*((*a).format_data as *mut iso9660)).wbuff_offset
            + (LOGICAL_BLOCK_SIZE * 32 as libc::c_int) as libc::c_long) as libc::c_ulong)
            .wrapping_sub((*((*a).format_data as *mut iso9660)).wbuff_remaining) as int64_t;
    read_offset = (*file).content.offset_of_temp;
    remaining = (*file).content.size as size_t;
    if remaining > (1024 as libc::c_int * 32 as libc::c_int) as libc::c_ulong {
        rbuff_size = (1024 as libc::c_int * 32 as libc::c_int) as size_t
    } else {
        rbuff_size = remaining
    }
    rbuff = malloc(rbuff_size) as *mut libc::c_uchar;
    if rbuff.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    while remaining != 0 {
        let mut rsize: size_t = 0;
        let mut rs: ssize_t = 0;
        /* Get the current file pointer. */
        write_offset = lseek(fd, 0 as libc::c_int as __off_t, SEEK_CUR);
        /* Change the file pointer to read. */
        lseek(fd, read_offset, SEEK_SET);
        rsize = rbuff_size;
        if rsize > remaining {
            rsize = remaining
        }
        rs = read((*iso9660).temp_fd, rbuff as *mut libc::c_void, rsize);
        if rs <= 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t read temporary file(%jd)\x00" as *const u8 as *const libc::c_char,
                rs,
            );
            ret = ARCHIVE_FATAL;
            break;
        } else {
            remaining =
                (remaining as libc::c_ulong).wrapping_sub(rs as libc::c_ulong) as size_t as size_t;
            read_offset += rs;
            /* Put the file pointer back to write. */
            lseek(fd, write_offset, SEEK_SET);
            r = zisofs_extract(a, &mut zext, rbuff, rs as size_t);
            if !(r < 0 as libc::c_int as libc::c_long) {
                continue;
            }
            ret = r as libc::c_int;
            break;
        }
    }
    if ret == ARCHIVE_OK {
        /*
         * Change the boot file content from zisofs'ed data
         * to plain data.
         */
        (*file).content.offset_of_temp = new_offset;
        (*file).content.size = (*file).zisofs.uncompressed_size as int64_t;
        archive_entry_set_size((*file).entry, (*file).content.size);
        /* Set to be no zisofs. */
        (*file).zisofs.header_size = 0 as libc::c_int as libc::c_uchar;
        (*file).zisofs.log2_bs = 0 as libc::c_int as libc::c_uchar;
        (*file).zisofs.uncompressed_size = 0 as libc::c_int as uint32_t;
        r = wb_write_padding_to_temp(a, (*file).content.size) as ssize_t;
        if r < 0 as libc::c_int as libc::c_long {
            ret = ARCHIVE_FATAL
        }
    }
    /*
     * Free the resource we used in this function only.
     */
    free(rbuff as *mut libc::c_void);
    free(zext.block_pointers as *mut libc::c_void);
    if zext.stream_valid != 0 && inflateEnd(&mut zext.stream) != Z_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Failed to clean up compressor\x00" as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FATAL
    }
    return ret;
}
/* HAVE_ZLIB_H */
