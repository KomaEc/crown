use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn archive_read_new() -> *mut archive;
    #[no_mangle]
    fn archive_read_support_format_empty(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_format_raw(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_open_filename(
        _: *mut archive,
        _filename: *const libc::c_char,
        _block_size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_open_filename_w(
        _: *mut archive,
        _filename: *const wchar_t,
        _block_size: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_next_header(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_read_data_block(
        a: *mut archive,
        buff: *mut *const libc::c_void,
        size: *mut size_t,
        offset: *mut la_int64_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_free(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_copy_error(dest: *mut archive, src: *mut archive);
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
    /* Bitmap of which of the above are valid.  Because we're lazy
     * about malloc-ing and reusing the underlying storage, we
     * can't rely on NULL pointers to indicate whether a string
     * has been set. */
    #[no_mangle]
    fn archive_mstring_clean(_: *mut archive_mstring);
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
    fn wcscmp(_: *const libc::c_int, _: *const libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_wcs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const wchar_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_mbs_len(
        _: *mut archive_mstring,
        mbs: *const libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_wcs_len(
        _: *mut archive_mstring,
        wcs: *const wchar_t,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_mstring_get_mbs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_append_from_wcs(
        _: *mut archive_string,
        _: *const wchar_t,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_mbs(_: *mut archive_mstring, mbs: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_wcs(_: *mut archive_mstring, wcs: *const wchar_t) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_new() -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_ctime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_gname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_copy_stat(_: *mut archive_entry, _: *const stat);
    /*-
     * Copyright (c) 2003-2015 Tim Kientzle
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
    fn __archive_get_date(now: time_t, _: *const libc::c_char) -> time_t;
    /* Note that "^" and "$" are not special unless you set the corresponding
     * flag above. */
    #[no_mangle]
    fn __archive_pathmatch(
        p: *const libc::c_char,
        s: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_pathmatch_w(
        p: *const wchar_t,
        s: *const wchar_t,
        flags: libc::c_int,
    ) -> libc::c_int;
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
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub struct archive_match {
    pub archive: archive,
    pub setflag: libc::c_int,
    pub recursive_include: libc::c_int,
    pub exclusions: match_list,
    pub inclusions: match_list,
    pub now: time_t,
    pub newer_mtime_filter: libc::c_int,
    pub newer_mtime_sec: time_t,
    pub newer_mtime_nsec: libc::c_long,
    pub newer_ctime_filter: libc::c_int,
    pub newer_ctime_sec: time_t,
    pub newer_ctime_nsec: libc::c_long,
    pub older_mtime_filter: libc::c_int,
    pub older_mtime_sec: time_t,
    pub older_mtime_nsec: libc::c_long,
    pub older_ctime_filter: libc::c_int,
    pub older_ctime_sec: time_t,
    pub older_ctime_nsec: libc::c_long,
    pub exclusion_tree: archive_rb_tree,
    pub exclusion_entry_list: entry_list,
    pub inclusion_uids: id_array,
    pub inclusion_gids: id_array,
    pub inclusion_unames: match_list,
    pub inclusion_gnames: match_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_list {
    pub first: *mut match_0,
    pub last: *mut *mut match_0,
    pub count: libc::c_int,
    pub unmatched_count: libc::c_int,
    pub unmatched_next: *mut match_0,
    pub unmatched_eof: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_0 {
    pub next: *mut match_0,
    pub matches: libc::c_int,
    pub pattern: archive_mstring,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct id_array {
    pub size: size_t,
    pub count: size_t,
    pub ids: *mut int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry_list {
    pub first: *mut match_file,
    pub last: *mut *mut match_file,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_file {
    pub node: archive_rb_node,
    pub next: *mut match_file,
    pub pathname: archive_mstring,
    pub flag: libc::c_int,
    pub mtime_sec: time_t,
    pub mtime_nsec: libc::c_long,
    pub ctime_sec: time_t,
    pub ctime_nsec: libc::c_long,
}
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Control recursive inclusion of directory content when directory is included. Default on. */
/* Add exclusion pathname pattern. */
/* Add exclusion pathname pattern from file. */
/* Add inclusion pathname pattern. */
/* Add inclusion pathname pattern from file. */
/*
 * How to get statistic information for inclusion patterns.
 */
/* Return the amount number of unmatched inclusion patterns. */
/* Return the pattern of unmatched inclusion with ARCHIVE_OK.
 * Return ARCHIVE_EOF if there is no inclusion pattern. */
/*
 * Test if a file is excluded by its time stamp.
 * The conditions are set by following functions.
 */
/*
 * Flags to tell a matching type of time stamps. These are used for
 * following functions.
 */
/* Time flag: mtime to be tested. */
/* Time flag: ctime to be tested. */
pub const ARCHIVE_MATCH_CTIME: libc::c_int = 0x200 as libc::c_int;
pub const ARCHIVE_MATCH_MTIME: libc::c_int = 0x100 as libc::c_int;
/* Comparison flag: Match the time if it is newer than. */
pub const ARCHIVE_MATCH_NEWER: libc::c_int = 0x1 as libc::c_int;
/* Comparison flag: Match the time if it is older than. */
pub const ARCHIVE_MATCH_OLDER: libc::c_int = 0x2 as libc::c_int;
/* Comparison flag: Match the time if it is equal to. */
pub const ARCHIVE_MATCH_EQUAL: libc::c_int = 0x10 as libc::c_int;
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
pub const ARCHIVE_STATE_NEW: libc::c_uint = 1 as libc::c_uint;
pub const ARCHIVE_MATCH_MAGIC: libc::c_uint = 0xcad11c9 as libc::c_uint;
pub const ARCHIVE_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
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
 * $FreeBSD$
 */
/* Don't anchor at beginning unless the pattern starts with "^" */
pub const PATHMATCH_NO_ANCHOR_START: libc::c_int = 1 as libc::c_int;
/* Don't anchor at end unless the pattern ends with "$" */
pub const PATHMATCH_NO_ANCHOR_END: libc::c_int = 2 as libc::c_int;
pub const PATTERN_IS_SET: libc::c_int = 1 as libc::c_int;
pub const TIME_IS_SET: libc::c_int = 2 as libc::c_int;
pub const ID_IS_SET: libc::c_int = 4 as libc::c_int;
pub const get_date: unsafe extern "C" fn(_: time_t, _: *const libc::c_char) -> time_t =
    __archive_get_date;
static mut rb_ops_mbs: archive_rb_tree_ops = unsafe {
    {
        let mut init = archive_rb_tree_ops {
            rbto_compare_nodes: Some(
                cmp_node_mbs
                    as unsafe extern "C" fn(
                        _: *const archive_rb_node,
                        _: *const archive_rb_node,
                    ) -> libc::c_int,
            ),
            rbto_compare_key: Some(
                cmp_key_mbs
                    as unsafe extern "C" fn(
                        _: *const archive_rb_node,
                        _: *const libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
static mut rb_ops_wcs: archive_rb_tree_ops = unsafe {
    {
        let mut init = archive_rb_tree_ops {
            rbto_compare_nodes: Some(
                cmp_node_wcs
                    as unsafe extern "C" fn(
                        _: *const archive_rb_node,
                        _: *const archive_rb_node,
                    ) -> libc::c_int,
            ),
            rbto_compare_key: Some(
                cmp_key_wcs
                    as unsafe extern "C" fn(
                        _: *const archive_rb_node,
                        _: *const libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
/*
 * The matching logic here needs to be re-thought.  I started out to
 * try to mimic gtar's matching logic, but it's not entirely
 * consistent.  In particular 'tar -t' and 'tar -x' interpret patterns
 * on the command line as anchored, but --exclude doesn't.
 */
unsafe extern "C" fn error_nomem(mut a: *mut archive_match) -> libc::c_int {
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ENOMEM,
        b"No memory\x00" as *const u8 as *const libc::c_char,
    );
    (*a).archive.state = ARCHIVE_STATE_FATAL;
    return -(30 as libc::c_int);
}
/*
 * Create an ARCHIVE_MATCH object.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_new() -> *mut archive {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    a = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_match>() as libc::c_ulong,
    ) as *mut archive_match;
    if a.is_null() {
        return 0 as *mut archive;
    }
    (*a).archive.magic = ARCHIVE_MATCH_MAGIC;
    (*a).archive.state = ARCHIVE_STATE_NEW;
    (*a).recursive_include = 1 as libc::c_int;
    match_list_init(&mut (*a).inclusions);
    match_list_init(&mut (*a).exclusions);
    __archive_rb_tree_init(&mut (*a).exclusion_tree, &rb_ops_mbs);
    entry_list_init(&mut (*a).exclusion_entry_list);
    match_list_init(&mut (*a).inclusion_unames);
    match_list_init(&mut (*a).inclusion_gnames);
    time(&mut (*a).now);
    return &mut (*a).archive;
}
/*
 * Free an ARCHIVE_MATCH object.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_free(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    if _a.is_null() {
        return 0 as libc::c_int;
    }
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_match_free\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    match_list_free(&mut (*a).inclusions);
    match_list_free(&mut (*a).exclusions);
    entry_list_free(&mut (*a).exclusion_entry_list);
    free((*a).inclusion_uids.ids as *mut libc::c_void);
    free((*a).inclusion_gids.ids as *mut libc::c_void);
    match_list_free(&mut (*a).inclusion_unames);
    match_list_free(&mut (*a).inclusion_gnames);
    free(a as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*
 * Convenience function to perform all exclusion tests.
 *
 * Returns 1 if archive entry is excluded.
 * Returns 0 if archive entry is not excluded.
 * Returns <0 if something error happened.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_excluded(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_excluded_ae\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if entry.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"entry is NULL\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = 0 as libc::c_int;
    if (*a).setflag & PATTERN_IS_SET != 0 {
        r = path_excluded(
            a,
            1 as libc::c_int,
            archive_entry_pathname(entry) as *const libc::c_void,
        );
        if r != 0 as libc::c_int {
            return r;
        }
    }
    if (*a).setflag & TIME_IS_SET != 0 {
        r = time_excluded(a, entry);
        if r != 0 as libc::c_int {
            return r;
        }
    }
    if (*a).setflag & ID_IS_SET != 0 {
        r = owner_excluded(a, entry)
    }
    return r;
}
/*
 * Utility functions to manage exclusion/inclusion patterns
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_exclude_pattern(
    mut _a: *mut archive,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_exclude_pattern\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if pattern.is_null() || *pattern as libc::c_int == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"pattern is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = add_pattern_mbs(a, &mut (*a).exclusions, pattern);
    if r != ARCHIVE_OK {
        return r;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_exclude_pattern_w(
    mut _a: *mut archive,
    mut pattern: *const wchar_t,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_exclude_pattern_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if pattern.is_null() || *pattern == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"pattern is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = add_pattern_wcs(a, &mut (*a).exclusions, pattern);
    if r != ARCHIVE_OK {
        return r;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_exclude_pattern_from_file(
    mut _a: *mut archive,
    mut pathname: *const libc::c_char,
    mut nullSeparator: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_exclude_pattern_from_file\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_pattern_from_file(
        a,
        &mut (*a).exclusions,
        1 as libc::c_int,
        pathname as *const libc::c_void,
        nullSeparator,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_exclude_pattern_from_file_w(
    mut _a: *mut archive,
    mut pathname: *const wchar_t,
    mut nullSeparator: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_exclude_pattern_from_file_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_pattern_from_file(
        a,
        &mut (*a).exclusions,
        0 as libc::c_int,
        pathname as *const libc::c_void,
        nullSeparator,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_pattern(
    mut _a: *mut archive,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_pattern\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if pattern.is_null() || *pattern as libc::c_int == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"pattern is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = add_pattern_mbs(a, &mut (*a).inclusions, pattern);
    if r != ARCHIVE_OK {
        return r;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_pattern_w(
    mut _a: *mut archive,
    mut pattern: *const wchar_t,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_pattern_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if pattern.is_null() || *pattern == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"pattern is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = add_pattern_wcs(a, &mut (*a).inclusions, pattern);
    if r != ARCHIVE_OK {
        return r;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_pattern_from_file(
    mut _a: *mut archive,
    mut pathname: *const libc::c_char,
    mut nullSeparator: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_pattern_from_file\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_pattern_from_file(
        a,
        &mut (*a).inclusions,
        1 as libc::c_int,
        pathname as *const libc::c_void,
        nullSeparator,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_pattern_from_file_w(
    mut _a: *mut archive,
    mut pathname: *const wchar_t,
    mut nullSeparator: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_pattern_from_file_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_pattern_from_file(
        a,
        &mut (*a).inclusions,
        0 as libc::c_int,
        pathname as *const libc::c_void,
        nullSeparator,
    );
}
/* Look up gname for gid or uname for uid. */
/* Default implementations are very, very stupid. */
/* "Standard" implementation uses getpwuid_r, getgrgid_r and caches the
 * results for performance. */
/* You can install your own lookups if you like. */
/* private_data */
/* lookup_fn */
/* cleanup_fn */
/* private_data */
/* lookup_fn */
/* cleanup_fn */
/* Start traversal. */
/*
 * Request that current entry be visited.  If you invoke it on every
 * directory, you'll get a physical traversal.  This is ignored if the
 * current entry isn't a directory or a link to a directory.  So, if
 * you invoke this on every returned path, you'll get a full logical
 * traversal.
 */
/* Request that the access time of the entry visited by traversal be restored. */
/*
 * Set behavior. The "flags" argument selects optional behavior.
 */
/* Request that the access time of the entry visited by traversal be restored.
 * This is the same as archive_read_disk_set_atime_restored. */
/* Default: Do not skip an entry which has nodump flags. */
/* Default: Skip a mac resource fork file whose prefix is "._" because of
 * using copyfile. */
/* Default: Traverse mount points. */
/* Default: Xattrs are read from disk. */
/* Default: ACLs are read from disk. */
/* Default: File flags are read from disk. */
/*
 * Set archive_match object that will be used in archive_read_disk to
 * know whether an entry should be skipped. The callback function
 * _excluded_func will be invoked when an entry is skipped by the result
 * of archive_match.
 */
/* Simplified cleanup interface;
 * This calls archive_read_free() or archive_write_free() as needed. */
/*
 * Accessor functions to read/set various information in
 * the struct archive object:
 */
/* Number of filters in the current filter pipeline. */
/* Filter #0 is the one closest to the format, -1 is a synonym for the
 * last filter, which is always the pseudo-filter that wraps the
 * client callbacks. */
/* These don't properly handle multiple filters, so are deprecated and
 * will eventually be removed. */
/* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, -1); */
/* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, 0); */
/* As of libarchive 3.0, this is an alias for archive_filter_name(a, 0); */
/* As of libarchive 3.0, this is an alias for archive_filter_code(a, 0); */
/*
 * ARCHIVE_MATCH API
 */
/*
 * Test if archive_entry is excluded.
 * This is a convenience function. This is the same as calling all
 * archive_match_path_excluded, archive_match_time_excluded
 * and archive_match_owner_excluded.
 */
/*
 * Test if pathname is excluded. The conditions are set by following functions.
 */
/*
 * Test functions for pathname patterns.
 *
 * Returns 1 if archive entry is excluded.
 * Returns 0 if archive entry is not excluded.
 * Returns <0 if something error happened.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_path_excluded(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_path_excluded\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if entry.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"entry is NULL\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* If we don't have exclusion/inclusion pattern set at all,
     * the entry is always not excluded. */
    if (*a).setflag & PATTERN_IS_SET == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return path_excluded(
        a,
        1 as libc::c_int,
        archive_entry_pathname(entry) as *const libc::c_void,
    );
}
/*
 * When recursive inclusion of directory content is enabled,
 * an inclusion pattern that matches a directory will also
 * include everything beneath that directory. Enabled by default.
 *
 * For compatibility with GNU tar, exclusion patterns always
 * match if a subset of the full patch matches (i.e., they are
 * are not rooted at the beginning of the path) and thus there
 * is no corresponding non-recursive exclusion mode.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_set_inclusion_recursion(
    mut _a: *mut archive,
    mut enabled: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_set_inclusion_recursion\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    (*a).recursive_include = enabled;
    return 0 as libc::c_int;
}
/*
 * Utility functions to get statistic information for inclusion patterns.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_path_unmatched_inclusions(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_unmatched_inclusions\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return (*a).inclusions.unmatched_count;
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_path_unmatched_inclusions_next(
    mut _a: *mut archive,
    mut _p: *mut *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut v: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_unmatched_inclusions_next\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    r = match_list_unmatched_inclusions_next(a, &mut (*a).inclusions, 1 as libc::c_int, &mut v);
    *_p = v as *const libc::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_path_unmatched_inclusions_next_w(
    mut _a: *mut archive,
    mut _p: *mut *const wchar_t,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut v: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_unmatched_inclusions_next_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    r = match_list_unmatched_inclusions_next(a, &mut (*a).inclusions, 0 as libc::c_int, &mut v);
    *_p = v as *const wchar_t;
    return r;
}
/*
 * Add inclusion/exclusion patterns.
 */
unsafe extern "C" fn add_pattern_mbs(
    mut a: *mut archive_match,
    mut list: *mut match_list,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    let mut match_0: *mut match_0 = 0 as *mut match_0;
    let mut len: size_t = 0;
    match_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<match_0>() as libc::c_ulong,
    ) as *mut match_0;
    if match_0.is_null() {
        return error_nomem(a);
    }
    /* Both "foo/" and "foo" should match "foo/bar". */
    len = strlen(pattern);
    if len != 0
        && *pattern.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '/' as i32
    {
        len = len.wrapping_sub(1)
    }
    archive_mstring_copy_mbs_len(&mut (*match_0).pattern, pattern, len);
    match_list_add(list, match_0);
    (*a).setflag |= PATTERN_IS_SET;
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_pattern_wcs(
    mut a: *mut archive_match,
    mut list: *mut match_list,
    mut pattern: *const wchar_t,
) -> libc::c_int {
    let mut match_0: *mut match_0 = 0 as *mut match_0;
    let mut len: size_t = 0;
    match_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<match_0>() as libc::c_ulong,
    ) as *mut match_0;
    if match_0.is_null() {
        return error_nomem(a);
    }
    /* Both "foo/" and "foo" should match "foo/bar". */
    len = wcslen(pattern);
    if len != 0
        && *pattern.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == '/' as i32
    {
        len = len.wrapping_sub(1)
    }
    archive_mstring_copy_wcs_len(&mut (*match_0).pattern, pattern, len);
    match_list_add(list, match_0);
    (*a).setflag |= PATTERN_IS_SET;
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_pattern_from_file(
    mut a: *mut archive_match,
    mut mlist: *mut match_list,
    mut mbs: libc::c_int,
    mut pathname: *const libc::c_void,
    mut nullSeparator: libc::c_int,
) -> libc::c_int {
    let mut ar: *mut archive = 0 as *mut archive;
    let mut ae: *mut archive_entry = 0 as *mut archive_entry;
    let mut as_0: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut buff: *const libc::c_void = 0 as *const libc::c_void;
    let mut size: size_t = 0;
    let mut offset: int64_t = 0;
    let mut r: libc::c_int = 0;
    ar = archive_read_new();
    if ar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    r = archive_read_support_format_raw(ar);
    r = archive_read_support_format_empty(ar);
    if r != ARCHIVE_OK {
        archive_copy_error(&mut (*a).archive, ar);
        archive_read_free(ar);
        return r;
    }
    if mbs != 0 {
        r = archive_read_open_filename(
            ar,
            pathname as *const libc::c_char,
            (512 as libc::c_int * 20 as libc::c_int) as size_t,
        )
    } else {
        r = archive_read_open_filename_w(
            ar,
            pathname as *const wchar_t,
            (512 as libc::c_int * 20 as libc::c_int) as size_t,
        )
    }
    if r != ARCHIVE_OK {
        archive_copy_error(&mut (*a).archive, ar);
        archive_read_free(ar);
        return r;
    }
    r = archive_read_next_header(ar, &mut ae);
    if r != ARCHIVE_OK {
        archive_read_free(ar);
        if r == ARCHIVE_EOF {
            return 0 as libc::c_int;
        } else {
            archive_copy_error(&mut (*a).archive, ar);
            return r;
        }
    }
    as_0.s = NULL as *mut libc::c_char;
    as_0.length = 0 as libc::c_int as size_t;
    as_0.buffer_length = 0 as libc::c_int as size_t;
    loop {
        r = archive_read_data_block(ar, &mut buff, &mut size, &mut offset);
        if !(r == ARCHIVE_OK) {
            break;
        }
        let mut b: *const libc::c_char = buff as *const libc::c_char;
        while size != 0 {
            let mut s: *const libc::c_char = b;
            let mut length: size_t = 0 as libc::c_int as size_t;
            let mut found_separator: libc::c_int = 0 as libc::c_int;
            while length < size {
                if nullSeparator != 0 {
                    if *b as libc::c_int == '\u{0}' as i32 {
                        found_separator = 1 as libc::c_int;
                        break;
                    }
                } else if *b as libc::c_int == 0xd as libc::c_int
                    || *b as libc::c_int == 0xa as libc::c_int
                {
                    found_separator = 1 as libc::c_int;
                    break;
                }
                b = b.offset(1);
                length = length.wrapping_add(1)
            }
            if found_separator == 0 {
                archive_strncat(&mut as_0, s as *const libc::c_void, length);
                break;
            } else {
                b = b.offset(1);
                size = (size as libc::c_ulong)
                    .wrapping_sub(length.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                archive_strncat(&mut as_0, s as *const libc::c_void, length);
                /* If the line is not empty, add the pattern. */
                if as_0.length > 0 as libc::c_int as libc::c_ulong {
                    /* Add pattern. */
                    r = add_pattern_mbs(a, mlist, as_0.s);
                    if r != ARCHIVE_OK {
                        archive_read_free(ar);
                        archive_string_free(&mut as_0);
                        return r;
                    }
                    as_0.length = 0 as libc::c_int as size_t
                }
            }
        }
    }
    /* If an error occurred, report it immediately. */
    if r < ARCHIVE_OK {
        archive_copy_error(&mut (*a).archive, ar);
        archive_read_free(ar);
        archive_string_free(&mut as_0);
        return r;
    }
    /* If the line is not empty, add the pattern. */
    if r == ARCHIVE_EOF && as_0.length > 0 as libc::c_int as libc::c_ulong {
        /* Add pattern. */
        r = add_pattern_mbs(a, mlist, as_0.s);
        if r != ARCHIVE_OK {
            archive_read_free(ar);
            archive_string_free(&mut as_0);
            return r;
        }
    }
    archive_read_free(ar);
    archive_string_free(&mut as_0);
    return 0 as libc::c_int;
}
/*
 * Test if pathname is excluded by inclusion/exclusion patterns.
 */
unsafe extern "C" fn path_excluded(
    mut a: *mut archive_match,
    mut mbs: libc::c_int,
    mut pathname: *const libc::c_void,
) -> libc::c_int {
    let mut match_0: *mut match_0 = 0 as *mut match_0;
    let mut matched: *mut match_0 = 0 as *mut match_0;
    let mut r: libc::c_int = 0;
    if a.is_null() {
        return 0 as libc::c_int;
    }
    /* Mark off any unmatched inclusions. */
    /* In particular, if a filename does appear in the archive and
     * is explicitly included and excluded, then we don't report
     * it as missing even though we don't extract it.
     */
    matched = NULL as *mut match_0;
    match_0 = (*a).inclusions.first;
    while !match_0.is_null() {
        if (*match_0).matches == 0 as libc::c_int && {
            r = match_path_inclusion(a, match_0, mbs, pathname);
            (r) != 0 as libc::c_int
        } {
            if r < 0 as libc::c_int {
                return r;
            }
            (*a).inclusions.unmatched_count -= 1;
            (*match_0).matches += 1;
            matched = match_0
        }
        match_0 = (*match_0).next
    }
    /* Exclusions take priority */
    match_0 = (*a).exclusions.first;
    while !match_0.is_null() {
        r = match_path_exclusion(a, match_0, mbs, pathname);
        if r != 0 {
            return r;
        }
        match_0 = (*match_0).next
    }
    /* It's not excluded and we found an inclusion above, so it's
     * included. */
    if !matched.is_null() {
        return 0 as libc::c_int;
    }
    /* We didn't find an unmatched inclusion, check the remaining ones. */
    match_0 = (*a).inclusions.first;
    while !match_0.is_null() {
        /* We looked at previously-unmatched inclusions already. */
        if (*match_0).matches > 0 as libc::c_int && {
            r = match_path_inclusion(a, match_0, mbs, pathname);
            (r) != 0 as libc::c_int
        } {
            if r < 0 as libc::c_int {
                return r;
            }
            (*match_0).matches += 1;
            return 0 as libc::c_int;
        }
        match_0 = (*match_0).next
    }
    /* If there were inclusions, default is to exclude. */
    if !(*a).inclusions.first.is_null() {
        return 1 as libc::c_int;
    }
    /* No explicit inclusions, default is to match. */
    return 0 as libc::c_int;
}
/*
 * This is a little odd, but it matches the default behavior of
 * gtar.  In particular, 'a*b' will match 'foo/a1111/222b/bar'
 *
 */
unsafe extern "C" fn match_path_exclusion(
    mut a: *mut archive_match,
    mut m: *mut match_0,
    mut mbs: libc::c_int,
    mut pn: *const libc::c_void,
) -> libc::c_int {
    let mut flag: libc::c_int = PATHMATCH_NO_ANCHOR_START | PATHMATCH_NO_ANCHOR_END;
    let mut r: libc::c_int = 0;
    if mbs != 0 {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        r = archive_mstring_get_mbs(&mut (*a).archive, &mut (*m).pattern, &mut p);
        if r == 0 as libc::c_int {
            return __archive_pathmatch(p, pn as *const libc::c_char, flag);
        }
    } else {
        let mut p_0: *const wchar_t = 0 as *const wchar_t;
        r = archive_mstring_get_wcs(&mut (*a).archive, &mut (*m).pattern, &mut p_0);
        if r == 0 as libc::c_int {
            return __archive_pathmatch_w(p_0, pn as *const wchar_t, flag);
        }
    }
    if errno == ENOMEM {
        return error_nomem(a);
    }
    return 0 as libc::c_int;
}
/*
 * Again, mimic gtar:  inclusions are always anchored (have to match
 * the beginning of the path) even though exclusions are not anchored.
 */
unsafe extern "C" fn match_path_inclusion(
    mut a: *mut archive_match,
    mut m: *mut match_0,
    mut mbs: libc::c_int,
    mut pn: *const libc::c_void,
) -> libc::c_int {
    /* Recursive operation requires only a prefix match. */
    let mut flag: libc::c_int = if (*a).recursive_include != 0 {
        PATHMATCH_NO_ANCHOR_END
    } else {
        0 as libc::c_int
    };
    let mut r: libc::c_int = 0;
    if mbs != 0 {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        r = archive_mstring_get_mbs(&mut (*a).archive, &mut (*m).pattern, &mut p);
        if r == 0 as libc::c_int {
            return __archive_pathmatch(p, pn as *const libc::c_char, flag);
        }
    } else {
        let mut p_0: *const wchar_t = 0 as *const wchar_t;
        r = archive_mstring_get_wcs(&mut (*a).archive, &mut (*m).pattern, &mut p_0);
        if r == 0 as libc::c_int {
            return __archive_pathmatch_w(p_0, pn as *const wchar_t, flag);
        }
    }
    if errno == ENOMEM {
        return error_nomem(a);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn match_list_init(mut list: *mut match_list) {
    (*list).first = NULL as *mut match_0;
    (*list).last = &mut (*list).first;
    (*list).count = 0 as libc::c_int;
}
unsafe extern "C" fn match_list_free(mut list: *mut match_list) {
    let mut p: *mut match_0 = 0 as *mut match_0;
    let mut q: *mut match_0 = 0 as *mut match_0;
    p = (*list).first;
    while !p.is_null() {
        q = p;
        p = (*p).next;
        archive_mstring_clean(&mut (*q).pattern);
        free(q as *mut libc::c_void);
    }
}
unsafe extern "C" fn match_list_add(mut list: *mut match_list, mut m: *mut match_0) {
    *(*list).last = m;
    (*list).last = &mut (*m).next;
    (*list).count += 1;
    (*list).unmatched_count += 1;
}
unsafe extern "C" fn match_list_unmatched_inclusions_next(
    mut a: *mut archive_match,
    mut list: *mut match_list,
    mut mbs: libc::c_int,
    mut vp: *mut *const libc::c_void,
) -> libc::c_int {
    let mut m: *mut match_0 = 0 as *mut match_0;
    *vp = NULL as *const libc::c_void;
    if (*list).unmatched_eof != 0 {
        (*list).unmatched_eof = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    if (*list).unmatched_next.is_null() {
        if (*list).unmatched_count == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        (*list).unmatched_next = (*list).first
    }
    m = (*list).unmatched_next;
    while !m.is_null() {
        let mut r: libc::c_int = 0;
        if (*m).matches != 0 {
            m = (*m).next
        } else {
            if mbs != 0 {
                let mut p: *const libc::c_char = 0 as *const libc::c_char;
                r = archive_mstring_get_mbs(&mut (*a).archive, &mut (*m).pattern, &mut p);
                if r < 0 as libc::c_int && errno == ENOMEM {
                    return error_nomem(a);
                }
                if p.is_null() {
                    p = b"\x00" as *const u8 as *const libc::c_char
                }
                *vp = p as *const libc::c_void
            } else {
                let mut p_0: *const wchar_t = 0 as *const wchar_t;
                r = archive_mstring_get_wcs(&mut (*a).archive, &mut (*m).pattern, &mut p_0);
                if r < 0 as libc::c_int && errno == ENOMEM {
                    return error_nomem(a);
                }
                if p_0.is_null() {
                    p_0 =
                        (*::std::mem::transmute::<&[u8; 4], &[libc::c_int; 1]>(b"\x00\x00\x00\x00"))
                            .as_ptr()
                }
                *vp = p_0 as *const libc::c_void
            }
            (*list).unmatched_next = (*m).next;
            if (*list).unmatched_next.is_null() {
                /* To return EOF next time. */
                (*list).unmatched_eof = 1 as libc::c_int
            }
            return 0 as libc::c_int;
        }
    }
    (*list).unmatched_next = NULL as *mut match_0;
    return 1 as libc::c_int;
}
/* Set inclusion time. */
/*
 * Utility functions to manage inclusion timestamps.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_time(
    mut _a: *mut archive,
    mut flag: libc::c_int,
    mut sec: time_t,
    mut nsec: libc::c_long,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = validate_time_flag(
        _a,
        flag,
        b"archive_match_include_time\x00" as *const u8 as *const libc::c_char,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    return set_timefilter(_a as *mut archive_match, flag, sec, nsec, sec, nsec);
}
/* Set inclusion time by a date string. */
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_date(
    mut _a: *mut archive,
    mut flag: libc::c_int,
    mut datestr: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = validate_time_flag(
        _a,
        flag,
        b"archive_match_include_date\x00" as *const u8 as *const libc::c_char,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    return set_timefilter_date(_a as *mut archive_match, flag, datestr);
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_date_w(
    mut _a: *mut archive,
    mut flag: libc::c_int,
    mut datestr: *const wchar_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = validate_time_flag(
        _a,
        flag,
        b"archive_match_include_date_w\x00" as *const u8 as *const libc::c_char,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    return set_timefilter_date_w(_a as *mut archive_match, flag, datestr);
}
/* Set inclusion time by a particular file. */
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_file_time(
    mut _a: *mut archive,
    mut flag: libc::c_int,
    mut pathname: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = validate_time_flag(
        _a,
        flag,
        b"archive_match_include_file_time\x00" as *const u8 as *const libc::c_char,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    return set_timefilter_pathname_mbs(_a as *mut archive_match, flag, pathname);
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_file_time_w(
    mut _a: *mut archive,
    mut flag: libc::c_int,
    mut pathname: *const wchar_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = validate_time_flag(
        _a,
        flag,
        b"archive_match_include_file_time_w\x00" as *const u8 as *const libc::c_char,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    return set_timefilter_pathname_wcs(_a as *mut archive_match, flag, pathname);
}
/* Add exclusion entry. */
#[no_mangle]
pub unsafe extern "C" fn archive_match_exclude_entry(
    mut _a: *mut archive,
    mut flag: libc::c_int,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_time_include_entry\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if entry.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"entry is NULL\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = validate_time_flag(
        _a,
        flag,
        b"archive_match_exclude_entry\x00" as *const u8 as *const libc::c_char,
    );
    if r != ARCHIVE_OK {
        return r;
    }
    return add_entry(a, flag, entry);
}
/*
 * Test function for time stamps.
 *
 * Returns 1 if archive entry is excluded.
 * Returns 0 if archive entry is not excluded.
 * Returns <0 if something error happened.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_time_excluded(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_time_excluded_ae\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if entry.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"entry is NULL\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* If we don't have inclusion time set at all, the entry is always
     * not excluded. */
    if (*a).setflag & TIME_IS_SET == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return time_excluded(a, entry);
}
unsafe extern "C" fn validate_time_flag(
    mut _a: *mut archive,
    mut flag: libc::c_int,
    mut _fn: *const libc::c_char,
) -> libc::c_int {
    let mut magic_test: libc::c_int =
        __archive_check_magic(_a, 0xcad11c9 as libc::c_uint, 1 as libc::c_uint, _fn);
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* Check a type of time. */
    if flag & (!(ARCHIVE_MATCH_MTIME | ARCHIVE_MATCH_CTIME) & 0xff00 as libc::c_int) != 0 {
        archive_set_error(
            _a,
            EINVAL,
            b"Invalid time flag\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    if flag & (ARCHIVE_MATCH_MTIME | ARCHIVE_MATCH_CTIME) == 0 as libc::c_int {
        archive_set_error(
            _a,
            EINVAL,
            b"No time flag\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* Check a type of comparison. */
    if flag
        & (!(ARCHIVE_MATCH_NEWER | ARCHIVE_MATCH_OLDER | ARCHIVE_MATCH_EQUAL) & 0xff as libc::c_int)
        != 0
    {
        archive_set_error(
            _a,
            EINVAL,
            b"Invalid comparison flag\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    if flag & (ARCHIVE_MATCH_NEWER | ARCHIVE_MATCH_OLDER | ARCHIVE_MATCH_EQUAL) == 0 as libc::c_int
    {
        archive_set_error(
            _a,
            EINVAL,
            b"No comparison flag\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_timefilter(
    mut a: *mut archive_match,
    mut timetype: libc::c_int,
    mut mtime_sec: time_t,
    mut mtime_nsec: libc::c_long,
    mut ctime_sec: time_t,
    mut ctime_nsec: libc::c_long,
) -> libc::c_int {
    if timetype & ARCHIVE_MATCH_MTIME != 0 {
        if timetype & ARCHIVE_MATCH_NEWER != 0
            || timetype & (ARCHIVE_MATCH_EQUAL | ARCHIVE_MATCH_NEWER | ARCHIVE_MATCH_OLDER)
                == ARCHIVE_MATCH_EQUAL
        {
            (*a).newer_mtime_filter = timetype;
            (*a).newer_mtime_sec = mtime_sec;
            (*a).newer_mtime_nsec = mtime_nsec;
            (*a).setflag |= TIME_IS_SET
        }
        if timetype & ARCHIVE_MATCH_OLDER != 0
            || timetype & (ARCHIVE_MATCH_EQUAL | ARCHIVE_MATCH_NEWER | ARCHIVE_MATCH_OLDER)
                == ARCHIVE_MATCH_EQUAL
        {
            (*a).older_mtime_filter = timetype;
            (*a).older_mtime_sec = mtime_sec;
            (*a).older_mtime_nsec = mtime_nsec;
            (*a).setflag |= TIME_IS_SET
        }
    }
    if timetype & ARCHIVE_MATCH_CTIME != 0 {
        if timetype & ARCHIVE_MATCH_NEWER != 0
            || timetype & (ARCHIVE_MATCH_EQUAL | ARCHIVE_MATCH_NEWER | ARCHIVE_MATCH_OLDER)
                == ARCHIVE_MATCH_EQUAL
        {
            (*a).newer_ctime_filter = timetype;
            (*a).newer_ctime_sec = ctime_sec;
            (*a).newer_ctime_nsec = ctime_nsec;
            (*a).setflag |= TIME_IS_SET
        }
        if timetype & ARCHIVE_MATCH_OLDER != 0
            || timetype & (ARCHIVE_MATCH_EQUAL | ARCHIVE_MATCH_NEWER | ARCHIVE_MATCH_OLDER)
                == ARCHIVE_MATCH_EQUAL
        {
            (*a).older_ctime_filter = timetype;
            (*a).older_ctime_sec = ctime_sec;
            (*a).older_ctime_nsec = ctime_nsec;
            (*a).setflag |= TIME_IS_SET
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_timefilter_date(
    mut a: *mut archive_match,
    mut timetype: libc::c_int,
    mut datestr: *const libc::c_char,
) -> libc::c_int {
    let mut t: time_t = 0;
    if datestr.is_null() || *datestr as libc::c_int == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"date is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    t = __archive_get_date((*a).now, datestr);
    if t == -(1 as libc::c_int) as time_t {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"invalid date string\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    return set_timefilter(
        a,
        timetype,
        t,
        0 as libc::c_int as libc::c_long,
        t,
        0 as libc::c_int as libc::c_long,
    );
}
unsafe extern "C" fn set_timefilter_date_w(
    mut a: *mut archive_match,
    mut timetype: libc::c_int,
    mut datestr: *const wchar_t,
) -> libc::c_int {
    let mut as_0: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut t: time_t = 0;
    if datestr.is_null() || *datestr == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"date is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    as_0.s = NULL as *mut libc::c_char;
    as_0.length = 0 as libc::c_int as size_t;
    as_0.buffer_length = 0 as libc::c_int as size_t;
    if archive_string_append_from_wcs(&mut as_0, datestr, wcslen(datestr)) < 0 as libc::c_int {
        archive_string_free(&mut as_0);
        if errno == ENOMEM {
            return error_nomem(a);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Failed to convert WCS to MBS\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    t = __archive_get_date((*a).now, as_0.s);
    archive_string_free(&mut as_0);
    if t == -(1 as libc::c_int) as time_t {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"invalid date string\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    return set_timefilter(
        a,
        timetype,
        t,
        0 as libc::c_int as libc::c_long,
        t,
        0 as libc::c_int as libc::c_long,
    );
}
/* _WIN32 && !__CYGWIN__ */
unsafe extern "C" fn set_timefilter_stat(
    mut a: *mut archive_match,
    mut timetype: libc::c_int,
    mut st: *mut stat,
) -> libc::c_int {
    let mut ae: *mut archive_entry = 0 as *mut archive_entry;
    let mut ctime_sec: time_t = 0;
    let mut mtime_sec: time_t = 0;
    let mut ctime_ns: libc::c_long = 0;
    let mut mtime_ns: libc::c_long = 0;
    ae = archive_entry_new();
    if ae.is_null() {
        return error_nomem(a);
    }
    archive_entry_copy_stat(ae, st);
    ctime_sec = archive_entry_ctime(ae);
    ctime_ns = archive_entry_ctime_nsec(ae);
    mtime_sec = archive_entry_mtime(ae);
    mtime_ns = archive_entry_mtime_nsec(ae);
    archive_entry_free(ae);
    return set_timefilter(a, timetype, mtime_sec, mtime_ns, ctime_sec, ctime_ns);
}
unsafe extern "C" fn set_timefilter_pathname_mbs(
    mut a: *mut archive_match,
    mut timetype: libc::c_int,
    mut path: *const libc::c_char,
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
    if path.is_null() || *path as libc::c_int == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"pathname is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    if stat(path, &mut st) != 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Failed to stat()\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    return set_timefilter_stat(a, timetype, &mut st);
}
unsafe extern "C" fn set_timefilter_pathname_wcs(
    mut a: *mut archive_match,
    mut timetype: libc::c_int,
    mut path: *const wchar_t,
) -> libc::c_int {
    let mut as_0: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut r: libc::c_int = 0;
    if path.is_null() || *path == '\u{0}' as i32 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"pathname is empty\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* Convert WCS filename to MBS filename. */
    as_0.s = NULL as *mut libc::c_char;
    as_0.length = 0 as libc::c_int as size_t;
    as_0.buffer_length = 0 as libc::c_int as size_t;
    if archive_string_append_from_wcs(&mut as_0, path, wcslen(path)) < 0 as libc::c_int {
        archive_string_free(&mut as_0);
        if errno == ENOMEM {
            return error_nomem(a);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Failed to convert WCS to MBS\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    r = set_timefilter_pathname_mbs(a, timetype, as_0.s);
    archive_string_free(&mut as_0);
    return r;
}
/* _WIN32 && !__CYGWIN__ */
/*
 * Call back functions for archive_rb.
 */
unsafe extern "C" fn cmp_node_mbs(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut f1: *mut match_file = n1 as uintptr_t as *mut match_file;
    let mut f2: *mut match_file = n2 as uintptr_t as *mut match_file;
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    archive_mstring_get_mbs(NULL as *mut archive, &mut (*f1).pathname, &mut p1);
    archive_mstring_get_mbs(NULL as *mut archive, &mut (*f2).pathname, &mut p2);
    if p1.is_null() {
        return 1 as libc::c_int;
    }
    if p2.is_null() {
        return -(1 as libc::c_int);
    }
    return strcmp(p1, p2);
}
unsafe extern "C" fn cmp_key_mbs(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut f: *mut match_file = n as uintptr_t as *mut match_file;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    archive_mstring_get_mbs(NULL as *mut archive, &mut (*f).pathname, &mut p);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    return strcmp(p, key as *const libc::c_char);
}
unsafe extern "C" fn cmp_node_wcs(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut f1: *mut match_file = n1 as uintptr_t as *mut match_file;
    let mut f2: *mut match_file = n2 as uintptr_t as *mut match_file;
    let mut p1: *const wchar_t = 0 as *const wchar_t;
    let mut p2: *const wchar_t = 0 as *const wchar_t;
    archive_mstring_get_wcs(NULL as *mut archive, &mut (*f1).pathname, &mut p1);
    archive_mstring_get_wcs(NULL as *mut archive, &mut (*f2).pathname, &mut p2);
    if p1.is_null() {
        return 1 as libc::c_int;
    }
    if p2.is_null() {
        return -(1 as libc::c_int);
    }
    return wcscmp(p1, p2);
}
unsafe extern "C" fn cmp_key_wcs(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut f: *mut match_file = n as uintptr_t as *mut match_file;
    let mut p: *const wchar_t = 0 as *const wchar_t;
    archive_mstring_get_wcs(NULL as *mut archive, &mut (*f).pathname, &mut p);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    return wcscmp(p, key as *const wchar_t);
}
unsafe extern "C" fn entry_list_init(mut list: *mut entry_list) {
    (*list).first = NULL as *mut match_file;
    (*list).last = &mut (*list).first;
    (*list).count = 0 as libc::c_int;
}
unsafe extern "C" fn entry_list_free(mut list: *mut entry_list) {
    let mut p: *mut match_file = 0 as *mut match_file;
    let mut q: *mut match_file = 0 as *mut match_file;
    p = (*list).first;
    while !p.is_null() {
        q = p;
        p = (*p).next;
        archive_mstring_clean(&mut (*q).pathname);
        free(q as *mut libc::c_void);
    }
}
unsafe extern "C" fn entry_list_add(mut list: *mut entry_list, mut file: *mut match_file) {
    *(*list).last = file;
    (*list).last = &mut (*file).next;
    (*list).count += 1;
}
unsafe extern "C" fn add_entry(
    mut a: *mut archive_match,
    mut flag: libc::c_int,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut f: *mut match_file = 0 as *mut match_file;
    let mut pathname: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    f = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<match_file>() as libc::c_ulong,
    ) as *mut match_file;
    if f.is_null() {
        return error_nomem(a);
    }
    pathname = archive_entry_pathname(entry) as *const libc::c_void;
    if pathname == NULL as *const libc::c_void {
        free(f as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"pathname is NULL\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    archive_mstring_copy_mbs(&mut (*f).pathname, pathname as *const libc::c_char);
    (*a).exclusion_tree.rbt_ops = &rb_ops_mbs;
    (*f).flag = flag;
    (*f).mtime_sec = archive_entry_mtime(entry);
    (*f).mtime_nsec = archive_entry_mtime_nsec(entry);
    (*f).ctime_sec = archive_entry_ctime(entry);
    (*f).ctime_nsec = archive_entry_ctime_nsec(entry);
    r = __archive_rb_tree_insert_node(&mut (*a).exclusion_tree, &mut (*f).node);
    if r == 0 {
        let mut f2: *mut match_file = 0 as *mut match_file;
        /* Get the duplicated file. */
        f2 = __archive_rb_tree_find_node(&mut (*a).exclusion_tree, pathname) as *mut match_file;
        /*
         * We always overwrite comparison condition.
         * If you do not want to overwrite it, you should not
         * call archive_match_exclude_entry(). We cannot know
         * what behavior you really expect since overwriting
         * condition might be different with the flag.
         */
        if !f2.is_null() {
            (*f2).flag = (*f).flag;
            (*f2).mtime_sec = (*f).mtime_sec;
            (*f2).mtime_nsec = (*f).mtime_nsec;
            (*f2).ctime_sec = (*f).ctime_sec;
            (*f2).ctime_nsec = (*f).ctime_nsec
        }
        /* Release the duplicated file. */
        archive_mstring_clean(&mut (*f).pathname);
        free(f as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    entry_list_add(&mut (*a).exclusion_entry_list, f);
    (*a).setflag |= TIME_IS_SET;
    return 0 as libc::c_int;
}
/*
 * Test if entry is excluded by its timestamp.
 */
unsafe extern "C" fn time_excluded(
    mut a: *mut archive_match,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut f: *mut match_file = 0 as *mut match_file;
    let mut pathname: *const libc::c_void = 0 as *const libc::c_void;
    let mut sec: time_t = 0;
    let mut nsec: libc::c_long = 0;
    /*
     * If this file/dir is excluded by a time comparison, skip it.
     */
    if (*a).newer_ctime_filter != 0 {
        /* If ctime is not set, use mtime instead. */
        if archive_entry_ctime_is_set(entry) != 0 {
            sec = archive_entry_ctime(entry)
        } else {
            sec = archive_entry_mtime(entry)
        } /* Too old, skip it. */
        if sec < (*a).newer_ctime_sec {
            return 1 as libc::c_int;
        }
        if sec == (*a).newer_ctime_sec {
            if archive_entry_ctime_is_set(entry) != 0 {
                nsec = archive_entry_ctime_nsec(entry)
            } else {
                nsec = archive_entry_mtime_nsec(entry)
            }
            /* Equal, skip it. */
            if nsec < (*a).newer_ctime_nsec {
                return 1 as libc::c_int;
            } /* Too old, skip it. */
            if nsec == (*a).newer_ctime_nsec
                && (*a).newer_ctime_filter & ARCHIVE_MATCH_EQUAL == 0 as libc::c_int
            {
                return 1 as libc::c_int;
            }
        }
    }
    if (*a).older_ctime_filter != 0 {
        /* If ctime is not set, use mtime instead. */
        if archive_entry_ctime_is_set(entry) != 0 {
            sec = archive_entry_ctime(entry)
        } else {
            sec = archive_entry_mtime(entry)
        } /* Too new, skip it. */
        if sec > (*a).older_ctime_sec {
            return 1 as libc::c_int;
        }
        if sec == (*a).older_ctime_sec {
            if archive_entry_ctime_is_set(entry) != 0 {
                nsec = archive_entry_ctime_nsec(entry)
            } else {
                nsec = archive_entry_mtime_nsec(entry)
            }
            /* Equal, skip it. */
            if nsec > (*a).older_ctime_nsec {
                return 1 as libc::c_int;
            } /* Too new, skip it. */
            if nsec == (*a).older_ctime_nsec
                && (*a).older_ctime_filter & ARCHIVE_MATCH_EQUAL == 0 as libc::c_int
            {
                return 1 as libc::c_int;
            }
        }
    } /* Too old, skip it. */
    if (*a).newer_mtime_filter != 0 {
        sec = archive_entry_mtime(entry);
        if sec < (*a).newer_mtime_sec {
            return 1 as libc::c_int;
        }
        if sec == (*a).newer_mtime_sec {
            nsec = archive_entry_mtime_nsec(entry);
            /* Equal, skip it. */
            if nsec < (*a).newer_mtime_nsec {
                return 1 as libc::c_int;
            } /* Too old, skip it. */
            if nsec == (*a).newer_mtime_nsec
                && (*a).newer_mtime_filter & ARCHIVE_MATCH_EQUAL == 0 as libc::c_int
            {
                return 1 as libc::c_int;
            }
        }
    } /* Too new, skip it. */
    if (*a).older_mtime_filter != 0 {
        sec = archive_entry_mtime(entry); /* Too new, skip it. */
        if sec > (*a).older_mtime_sec {
            return 1 as libc::c_int;
        }
        nsec = archive_entry_mtime_nsec(entry);
        if sec == (*a).older_mtime_sec {
            if nsec > (*a).older_mtime_nsec {
                return 1 as libc::c_int;
            }
            if nsec == (*a).older_mtime_nsec
                && (*a).older_mtime_filter & ARCHIVE_MATCH_EQUAL == 0 as libc::c_int
            {
                return 1 as libc::c_int;
            }
            /* Equal, skip it. */
        }
    }
    /* If there is no exclusion list, include the file. */
    if (*a).exclusion_entry_list.count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    pathname = archive_entry_pathname(entry) as *const libc::c_void;
    (*a).exclusion_tree.rbt_ops = &rb_ops_mbs;
    if pathname == NULL as *const libc::c_void {
        return 0 as libc::c_int;
    }
    f = __archive_rb_tree_find_node(&mut (*a).exclusion_tree, pathname) as *mut match_file;
    /* If the file wasn't rejected, include it. */
    if f.is_null() {
        return 0 as libc::c_int;
    }
    if (*f).flag & ARCHIVE_MATCH_CTIME != 0 {
        sec = archive_entry_ctime(entry);
        if (*f).ctime_sec > sec {
            if (*f).flag & ARCHIVE_MATCH_OLDER != 0 {
                return 1 as libc::c_int;
            }
        } else if (*f).ctime_sec < sec {
            if (*f).flag & ARCHIVE_MATCH_NEWER != 0 {
                return 1 as libc::c_int;
            }
        } else {
            nsec = archive_entry_ctime_nsec(entry);
            if (*f).ctime_nsec > nsec {
                if (*f).flag & ARCHIVE_MATCH_OLDER != 0 {
                    return 1 as libc::c_int;
                }
            } else if (*f).ctime_nsec < nsec {
                if (*f).flag & ARCHIVE_MATCH_NEWER != 0 {
                    return 1 as libc::c_int;
                }
            } else if (*f).flag & ARCHIVE_MATCH_EQUAL != 0 {
                return 1 as libc::c_int;
            }
        }
    }
    if (*f).flag & ARCHIVE_MATCH_MTIME != 0 {
        sec = archive_entry_mtime(entry);
        if (*f).mtime_sec > sec {
            if (*f).flag & ARCHIVE_MATCH_OLDER != 0 {
                return 1 as libc::c_int;
            }
        } else if (*f).mtime_sec < sec {
            if (*f).flag & ARCHIVE_MATCH_NEWER != 0 {
                return 1 as libc::c_int;
            }
        } else {
            nsec = archive_entry_mtime_nsec(entry);
            if (*f).mtime_nsec > nsec {
                if (*f).flag & ARCHIVE_MATCH_OLDER != 0 {
                    return 1 as libc::c_int;
                }
            } else if (*f).mtime_nsec < nsec {
                if (*f).flag & ARCHIVE_MATCH_NEWER != 0 {
                    return 1 as libc::c_int;
                }
            } else if (*f).flag & ARCHIVE_MATCH_EQUAL != 0 {
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
/* Add inclusion uid, gid, uname and gname. */
/*
 * Utility functions to manage inclusion owners
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_uid(
    mut _a: *mut archive,
    mut uid: la_int64_t,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_uid\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_owner_id(a, &mut (*a).inclusion_uids, uid);
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_gid(
    mut _a: *mut archive,
    mut gid: la_int64_t,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_gid\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_owner_id(a, &mut (*a).inclusion_gids, gid);
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_uname(
    mut _a: *mut archive,
    mut uname: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_uname\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_owner_name(
        a,
        &mut (*a).inclusion_unames,
        1 as libc::c_int,
        uname as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_uname_w(
    mut _a: *mut archive,
    mut uname: *const wchar_t,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_uname_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_owner_name(
        a,
        &mut (*a).inclusion_unames,
        0 as libc::c_int,
        uname as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_gname(
    mut _a: *mut archive,
    mut gname: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_gname\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_owner_name(
        a,
        &mut (*a).inclusion_gnames,
        1 as libc::c_int,
        gname as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_match_include_gname_w(
    mut _a: *mut archive,
    mut gname: *const wchar_t,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_include_gname_w\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    return add_owner_name(
        a,
        &mut (*a).inclusion_gnames,
        0 as libc::c_int,
        gname as *const libc::c_void,
    );
}
/*
 * Test if a file is excluded by its uid ,gid, uname or gname.
 * The conditions are set by following functions.
 */
/*
 * Test function for owner(uid, gid, uname, gname).
 *
 * Returns 1 if archive entry is excluded.
 * Returns 0 if archive entry is not excluded.
 * Returns <0 if something error happened.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_match_owner_excluded(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_match = 0 as *mut archive_match;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xcad11c9 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_match_id_excluded_ae\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    a = _a as *mut archive_match;
    if entry.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"entry is NULL\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* If we don't have inclusion id set at all, the entry is always
     * not excluded. */
    if (*a).setflag & ID_IS_SET == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return owner_excluded(a, entry);
}
unsafe extern "C" fn add_owner_id(
    mut a: *mut archive_match,
    mut ids: *mut id_array,
    mut id: int64_t,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    if (*ids).count.wrapping_add(1 as libc::c_int as libc::c_ulong) >= (*ids).size {
        let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*ids).size == 0 as libc::c_int as libc::c_ulong {
            (*ids).size = 8 as libc::c_int as size_t
        } else {
            (*ids).size = ((*ids).size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        p = realloc(
            (*ids).ids as *mut libc::c_void,
            (::std::mem::size_of::<int64_t>() as libc::c_ulong).wrapping_mul((*ids).size),
        );
        if p.is_null() {
            return error_nomem(a);
        }
        (*ids).ids = p as *mut int64_t
    }
    /* Find an insert point. */
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*ids).count {
        if *(*ids).ids.offset(i as isize) >= id {
            break;
        }
        i = i.wrapping_add(1)
    }
    /* Add owner id. */
    if i as libc::c_ulong == (*ids).count {
        let fresh0 = (*ids).count;
        (*ids).count = (*ids).count.wrapping_add(1);
        *(*ids).ids.offset(fresh0 as isize) = id
    } else if *(*ids).ids.offset(i as isize) != id {
        memmove(
            &mut *(*ids)
                .ids
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                as *mut int64_t as *mut libc::c_void,
            &mut *(*ids).ids.offset(i as isize) as *mut int64_t as *const libc::c_void,
            (*ids)
                .count
                .wrapping_sub(i as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<int64_t>() as libc::c_ulong),
        );
        *(*ids).ids.offset(i as isize) = id;
        (*ids).count = (*ids).count.wrapping_add(1)
    }
    (*a).setflag |= ID_IS_SET;
    return 0 as libc::c_int;
}
unsafe extern "C" fn match_owner_id(mut ids: *mut id_array, mut id: int64_t) -> libc::c_int {
    let mut b: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    let mut t: libc::c_uint = 0;
    t = 0 as libc::c_int as libc::c_uint;
    b = (*ids).count as libc::c_uint;
    while t < b {
        m = t.wrapping_add(b) >> 1 as libc::c_int;
        if *(*ids).ids.offset(m as isize) == id {
            return 1 as libc::c_int;
        }
        if *(*ids).ids.offset(m as isize) < id {
            t = m.wrapping_add(1 as libc::c_int as libc::c_uint)
        } else {
            b = m
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_owner_name(
    mut a: *mut archive_match,
    mut list: *mut match_list,
    mut mbs: libc::c_int,
    mut name: *const libc::c_void,
) -> libc::c_int {
    let mut match_0: *mut match_0 = 0 as *mut match_0;
    match_0 = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<match_0>() as libc::c_ulong,
    ) as *mut match_0;
    if match_0.is_null() {
        return error_nomem(a);
    }
    if mbs != 0 {
        archive_mstring_copy_mbs(&mut (*match_0).pattern, name as *const libc::c_char);
    } else {
        archive_mstring_copy_wcs(&mut (*match_0).pattern, name as *const wchar_t);
    }
    match_list_add(list, match_0);
    (*a).setflag |= ID_IS_SET;
    return 0 as libc::c_int;
}
unsafe extern "C" fn match_owner_name_mbs(
    mut a: *mut archive_match,
    mut list: *mut match_list,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut m: *mut match_0 = 0 as *mut match_0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if name.is_null() || *name as libc::c_int == '\u{0}' as i32 {
        return 0 as libc::c_int;
    }
    m = (*list).first;
    while !m.is_null() {
        if archive_mstring_get_mbs(&mut (*a).archive, &mut (*m).pattern, &mut p) < 0 as libc::c_int
            && errno == ENOMEM
        {
            return error_nomem(a);
        }
        if !p.is_null() && strcmp(p, name) == 0 as libc::c_int {
            (*m).matches += 1;
            return 1 as libc::c_int;
        }
        m = (*m).next
    }
    return 0 as libc::c_int;
}
/*
 * Test if entry is excluded by uid, gid, uname or gname.
 */
unsafe extern "C" fn owner_excluded(
    mut a: *mut archive_match,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*a).inclusion_uids.count != 0 {
        if match_owner_id(&mut (*a).inclusion_uids, archive_entry_uid(entry)) == 0 {
            return 1 as libc::c_int;
        }
    }
    if (*a).inclusion_gids.count != 0 {
        if match_owner_id(&mut (*a).inclusion_gids, archive_entry_gid(entry)) == 0 {
            return 1 as libc::c_int;
        }
    }
    if (*a).inclusion_unames.count != 0 {
        r = match_owner_name_mbs(a, &mut (*a).inclusion_unames, archive_entry_uname(entry));
        if r == 0 {
            return 1 as libc::c_int;
        } else {
            if r < 0 as libc::c_int {
                return r;
            }
        }
    }
    if (*a).inclusion_gnames.count != 0 {
        r = match_owner_name_mbs(a, &mut (*a).inclusion_gnames, archive_entry_gname(entry));
        if r == 0 {
            return 1 as libc::c_int;
        } else {
            if r < 0 as libc::c_int {
                return r;
            }
        }
    }
    return 0 as libc::c_int;
}
