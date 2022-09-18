use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    pub type evp_md_ctx_st;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn archive_write_set_bytes_in_last_block(
        _: *mut archive,
        bytes_in_last_block: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    /* Minimal interface to digest functionality for internal use in libarchive */
    /* Message Digest */
    #[no_mangle]
    static __archive_digest: archive_digest;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_new() -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_devmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_devminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_fflags(_: *mut archive_entry, _: *mut libc::c_ulong, _: *mut libc::c_ulong);
    #[no_mangle]
    fn archive_entry_fflags_text(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_gname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_ino(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
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
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
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
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_string_concat(dest: *mut archive_string, src: *mut archive_string);
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
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
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
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
/*
 * The Data only for a regular file.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reg_info {
    pub compute_sum: libc::c_int,
    pub crc: uint32_t,
    pub buf_md5: [libc::c_uchar; 16],
    pub buf_rmd160: [libc::c_uchar; 20],
    pub buf_sha1: [libc::c_uchar; 20],
    pub buf_sha256: [libc::c_uchar; 32],
    pub buf_sha384: [libc::c_uchar; 48],
    pub buf_sha512: [libc::c_uchar; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtree_entry {
    pub rbnode: archive_rb_node,
    pub next: *mut mtree_entry,
    pub parent: *mut mtree_entry,
    pub dir_info: *mut dir_info,
    pub reg_info: *mut reg_info,
    pub parentdir: archive_string,
    pub basename: archive_string,
    pub pathname: archive_string,
    pub symlink: archive_string,
    pub uname: archive_string,
    pub gname: archive_string,
    pub fflags_text: archive_string,
    pub nlink: libc::c_uint,
    pub filetype: mode_t,
    pub mode: mode_t,
    pub size: int64_t,
    pub uid: int64_t,
    pub gid: int64_t,
    pub mtime: time_t,
    pub mtime_nsec: libc::c_long,
    pub fflags_set: libc::c_ulong,
    pub fflags_clear: libc::c_ulong,
    pub rdevmajor: dev_t,
    pub rdevminor: dev_t,
    pub devmajor: dev_t,
    pub devminor: dev_t,
    pub ino: int64_t,
}
/*
 * The Data only for a directory file.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dir_info {
    pub rbtree: archive_rb_tree,
    pub children: mtree_chain,
    pub chnext: *mut mtree_entry,
    pub virtual_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtree_chain {
    pub first: *mut mtree_entry,
    pub last: *mut *mut mtree_entry,
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
pub struct mtree_writer {
    pub mtree_entry: *mut mtree_entry,
    pub root: *mut mtree_entry,
    pub cur_dirent: *mut mtree_entry,
    pub cur_dirstr: archive_string,
    pub file_list: mtree_chain,
    pub ebuf: archive_string,
    pub buf: archive_string,
    pub first: libc::c_int,
    pub entry_bytes_remaining: uint64_t,
    pub set: C2RustUnnamed,
    pub acs: att_counter_set,
    pub classic: libc::c_int,
    pub depth: libc::c_int,
    pub compute_sum: libc::c_int,
    pub crc: uint32_t,
    pub crc_len: uint64_t,
    pub md5ctx: archive_md5_ctx,
    pub rmd160ctx: archive_rmd160_ctx,
    pub sha1ctx: archive_sha1_ctx,
    pub sha256ctx: archive_sha256_ctx,
    pub sha384ctx: archive_sha384_ctx,
    pub sha512ctx: archive_sha512_ctx,
    pub keys: libc::c_int,
    pub dironly: libc::c_int,
    pub indent: libc::c_int,
    pub output_global_set: libc::c_int,
}
pub type archive_sha512_ctx = *mut EVP_MD_CTX;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type archive_sha384_ctx = *mut EVP_MD_CTX;
pub type archive_sha256_ctx = *mut EVP_MD_CTX;
pub type archive_sha1_ctx = *mut EVP_MD_CTX;
pub type archive_rmd160_ctx = *mut EVP_MD_CTX;
pub type archive_md5_ctx = *mut EVP_MD_CTX;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct att_counter_set {
    pub uid_list: *mut attr_counter,
    pub gid_list: *mut attr_counter,
    pub mode_list: *mut attr_counter,
    pub flags_list: *mut attr_counter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_counter {
    pub prev: *mut attr_counter,
    pub next: *mut attr_counter,
    pub m_entry: *mut mtree_entry,
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub processing: libc::c_int,
    pub type_0: mode_t,
    pub keys: libc::c_int,
    pub uid: int64_t,
    pub gid: int64_t,
    pub mode: mode_t,
    pub fflags_set: libc::c_ulong,
    pub fflags_clear: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_digest {
    pub md5init: Option<unsafe extern "C" fn(_: *mut archive_md5_ctx) -> libc::c_int>,
    pub md5update: Option<
        unsafe extern "C" fn(
            _: *mut archive_md5_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub md5final:
        Option<unsafe extern "C" fn(_: *mut archive_md5_ctx, _: *mut libc::c_void) -> libc::c_int>,
    pub rmd160init: Option<unsafe extern "C" fn(_: *mut archive_rmd160_ctx) -> libc::c_int>,
    pub rmd160update: Option<
        unsafe extern "C" fn(
            _: *mut archive_rmd160_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub rmd160final: Option<
        unsafe extern "C" fn(_: *mut archive_rmd160_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha1init: Option<unsafe extern "C" fn(_: *mut archive_sha1_ctx) -> libc::c_int>,
    pub sha1update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha1_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha1final:
        Option<unsafe extern "C" fn(_: *mut archive_sha1_ctx, _: *mut libc::c_void) -> libc::c_int>,
    pub sha256init: Option<unsafe extern "C" fn(_: *mut archive_sha256_ctx) -> libc::c_int>,
    pub sha256update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha256_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha256final: Option<
        unsafe extern "C" fn(_: *mut archive_sha256_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha384init: Option<unsafe extern "C" fn(_: *mut archive_sha384_ctx) -> libc::c_int>,
    pub sha384update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha384_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha384final: Option<
        unsafe extern "C" fn(_: *mut archive_sha384_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha512init: Option<unsafe extern "C" fn(_: *mut archive_sha512_ctx) -> libc::c_int>,
    pub sha512update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha512_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha512final: Option<
        unsafe extern "C" fn(_: *mut archive_sha512_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FORMAT_MTREE: libc::c_int = 0x80000 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
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
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
pub const ARCHIVE_RB_DIR_LEFT: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_RB_DIR_RIGHT: libc::c_int = 1 as libc::c_int;
pub const INDENTNAMELEN: libc::c_int = 15 as libc::c_int;
pub const MAXLINELEN: libc::c_int = 80 as libc::c_int;
pub const SET_KEYS: libc::c_int = F_FLAGS | F_GID | F_GNAME | F_MODE | F_TYPE | F_UID | F_UNAME;
pub const F_CKSUM: libc::c_int = 0x1 as libc::c_int;
pub const F_DEV: libc::c_int = 0x2 as libc::c_int;
pub const F_FLAGS: libc::c_int = 0x8 as libc::c_int;
pub const F_GID: libc::c_int = 0x10 as libc::c_int;
pub const F_GNAME: libc::c_int = 0x20 as libc::c_int;
pub const F_MD5: libc::c_int = 0x100 as libc::c_int;
pub const F_MODE: libc::c_int = 0x200 as libc::c_int;
pub const F_NLINK: libc::c_int = 0x400 as libc::c_int;
pub const F_RMD160: libc::c_int = 0x2000 as libc::c_int;
pub const F_SHA1: libc::c_int = 0x4000 as libc::c_int;
pub const F_SIZE: libc::c_int = 0x8000 as libc::c_int;
pub const F_SLINK: libc::c_int = 0x10000 as libc::c_int;
pub const F_TIME: libc::c_int = 0x40000 as libc::c_int;
pub const F_TYPE: libc::c_int = 0x80000 as libc::c_int;
pub const F_UID: libc::c_int = 0x100000 as libc::c_int;
pub const F_UNAME: libc::c_int = 0x200000 as libc::c_int;
pub const F_SHA256: libc::c_int = 0x800000 as libc::c_int;
pub const F_SHA384: libc::c_int = 0x1000000 as libc::c_int;
pub const F_SHA512: libc::c_int = 0x2000000 as libc::c_int;
pub const F_INO: libc::c_int = 0x4000000 as libc::c_int;
pub const F_RESDEV: libc::c_int = 0x8000000 as libc::c_int;
/* If it is set, use /set keyword to set
 * global values. When generating mtree
 * classic format, it is set by default. */
pub const DEFAULT_KEYS: libc::c_int = F_DEV
    | F_FLAGS
    | F_GID
    | F_GNAME
    | F_SLINK
    | F_MODE
    | F_NLINK
    | F_SIZE
    | F_TIME
    | F_TYPE
    | F_UID
    | F_UNAME;
pub const attr_counter_set_reset: unsafe extern "C" fn(_: *mut mtree_writer) -> () =
    attr_counter_set_free;
static mut crctab: [uint32_t; 256] = [
    0 as libc::c_int as uint32_t,
    0x4c11db7 as libc::c_int as uint32_t,
    0x9823b6e as libc::c_int as uint32_t,
    0xd4326d9 as libc::c_int as uint32_t,
    0x130476dc as libc::c_int as uint32_t,
    0x17c56b6b as libc::c_int as uint32_t,
    0x1a864db2 as libc::c_int as uint32_t,
    0x1e475005 as libc::c_int as uint32_t,
    0x2608edb8 as libc::c_int as uint32_t,
    0x22c9f00f as libc::c_int as uint32_t,
    0x2f8ad6d6 as libc::c_int as uint32_t,
    0x2b4bcb61 as libc::c_int as uint32_t,
    0x350c9b64 as libc::c_int as uint32_t,
    0x31cd86d3 as libc::c_int as uint32_t,
    0x3c8ea00a as libc::c_int as uint32_t,
    0x384fbdbd as libc::c_int as uint32_t,
    0x4c11db70 as libc::c_int as uint32_t,
    0x48d0c6c7 as libc::c_int as uint32_t,
    0x4593e01e as libc::c_int as uint32_t,
    0x4152fda9 as libc::c_int as uint32_t,
    0x5f15adac as libc::c_int as uint32_t,
    0x5bd4b01b as libc::c_int as uint32_t,
    0x569796c2 as libc::c_int as uint32_t,
    0x52568b75 as libc::c_int as uint32_t,
    0x6a1936c8 as libc::c_int as uint32_t,
    0x6ed82b7f as libc::c_int as uint32_t,
    0x639b0da6 as libc::c_int as uint32_t,
    0x675a1011 as libc::c_int as uint32_t,
    0x791d4014 as libc::c_int as uint32_t,
    0x7ddc5da3 as libc::c_int as uint32_t,
    0x709f7b7a as libc::c_int as uint32_t,
    0x745e66cd as libc::c_int as uint32_t,
    0x9823b6e0 as libc::c_uint,
    0x9ce2ab57 as libc::c_uint,
    0x91a18d8e as libc::c_uint,
    0x95609039 as libc::c_uint,
    0x8b27c03c as libc::c_uint,
    0x8fe6dd8b as libc::c_uint,
    0x82a5fb52 as libc::c_uint,
    0x8664e6e5 as libc::c_uint,
    0xbe2b5b58 as libc::c_uint,
    0xbaea46ef as libc::c_uint,
    0xb7a96036 as libc::c_uint,
    0xb3687d81 as libc::c_uint,
    0xad2f2d84 as libc::c_uint,
    0xa9ee3033 as libc::c_uint,
    0xa4ad16ea as libc::c_uint,
    0xa06c0b5d as libc::c_uint,
    0xd4326d90 as libc::c_uint,
    0xd0f37027 as libc::c_uint,
    0xddb056fe as libc::c_uint,
    0xd9714b49 as libc::c_uint,
    0xc7361b4c as libc::c_uint,
    0xc3f706fb as libc::c_uint,
    0xceb42022 as libc::c_uint,
    0xca753d95 as libc::c_uint,
    0xf23a8028 as libc::c_uint,
    0xf6fb9d9f as libc::c_uint,
    0xfbb8bb46 as libc::c_uint,
    0xff79a6f1 as libc::c_uint,
    0xe13ef6f4 as libc::c_uint,
    0xe5ffeb43 as libc::c_uint,
    0xe8bccd9a as libc::c_uint,
    0xec7dd02d as libc::c_uint,
    0x34867077 as libc::c_int as uint32_t,
    0x30476dc0 as libc::c_int as uint32_t,
    0x3d044b19 as libc::c_int as uint32_t,
    0x39c556ae as libc::c_int as uint32_t,
    0x278206ab as libc::c_int as uint32_t,
    0x23431b1c as libc::c_int as uint32_t,
    0x2e003dc5 as libc::c_int as uint32_t,
    0x2ac12072 as libc::c_int as uint32_t,
    0x128e9dcf as libc::c_int as uint32_t,
    0x164f8078 as libc::c_int as uint32_t,
    0x1b0ca6a1 as libc::c_int as uint32_t,
    0x1fcdbb16 as libc::c_int as uint32_t,
    0x18aeb13 as libc::c_int as uint32_t,
    0x54bf6a4 as libc::c_int as uint32_t,
    0x808d07d as libc::c_int as uint32_t,
    0xcc9cdca as libc::c_int as uint32_t,
    0x7897ab07 as libc::c_int as uint32_t,
    0x7c56b6b0 as libc::c_int as uint32_t,
    0x71159069 as libc::c_int as uint32_t,
    0x75d48dde as libc::c_int as uint32_t,
    0x6b93dddb as libc::c_int as uint32_t,
    0x6f52c06c as libc::c_int as uint32_t,
    0x6211e6b5 as libc::c_int as uint32_t,
    0x66d0fb02 as libc::c_int as uint32_t,
    0x5e9f46bf as libc::c_int as uint32_t,
    0x5a5e5b08 as libc::c_int as uint32_t,
    0x571d7dd1 as libc::c_int as uint32_t,
    0x53dc6066 as libc::c_int as uint32_t,
    0x4d9b3063 as libc::c_int as uint32_t,
    0x495a2dd4 as libc::c_int as uint32_t,
    0x44190b0d as libc::c_int as uint32_t,
    0x40d816ba as libc::c_int as uint32_t,
    0xaca5c697 as libc::c_uint,
    0xa864db20 as libc::c_uint,
    0xa527fdf9 as libc::c_uint,
    0xa1e6e04e as libc::c_uint,
    0xbfa1b04b as libc::c_uint,
    0xbb60adfc as libc::c_uint,
    0xb6238b25 as libc::c_uint,
    0xb2e29692 as libc::c_uint,
    0x8aad2b2f as libc::c_uint,
    0x8e6c3698 as libc::c_uint,
    0x832f1041 as libc::c_uint,
    0x87ee0df6 as libc::c_uint,
    0x99a95df3 as libc::c_uint,
    0x9d684044 as libc::c_uint,
    0x902b669d as libc::c_uint,
    0x94ea7b2a as libc::c_uint,
    0xe0b41de7 as libc::c_uint,
    0xe4750050 as libc::c_uint,
    0xe9362689 as libc::c_uint,
    0xedf73b3e as libc::c_uint,
    0xf3b06b3b as libc::c_uint,
    0xf771768c as libc::c_uint,
    0xfa325055 as libc::c_uint,
    0xfef34de2 as libc::c_uint,
    0xc6bcf05f as libc::c_uint,
    0xc27dede8 as libc::c_uint,
    0xcf3ecb31 as libc::c_uint,
    0xcbffd686 as libc::c_uint,
    0xd5b88683 as libc::c_uint,
    0xd1799b34 as libc::c_uint,
    0xdc3abded as libc::c_uint,
    0xd8fba05a as libc::c_uint,
    0x690ce0ee as libc::c_int as uint32_t,
    0x6dcdfd59 as libc::c_int as uint32_t,
    0x608edb80 as libc::c_int as uint32_t,
    0x644fc637 as libc::c_int as uint32_t,
    0x7a089632 as libc::c_int as uint32_t,
    0x7ec98b85 as libc::c_int as uint32_t,
    0x738aad5c as libc::c_int as uint32_t,
    0x774bb0eb as libc::c_int as uint32_t,
    0x4f040d56 as libc::c_int as uint32_t,
    0x4bc510e1 as libc::c_int as uint32_t,
    0x46863638 as libc::c_int as uint32_t,
    0x42472b8f as libc::c_int as uint32_t,
    0x5c007b8a as libc::c_int as uint32_t,
    0x58c1663d as libc::c_int as uint32_t,
    0x558240e4 as libc::c_int as uint32_t,
    0x51435d53 as libc::c_int as uint32_t,
    0x251d3b9e as libc::c_int as uint32_t,
    0x21dc2629 as libc::c_int as uint32_t,
    0x2c9f00f0 as libc::c_int as uint32_t,
    0x285e1d47 as libc::c_int as uint32_t,
    0x36194d42 as libc::c_int as uint32_t,
    0x32d850f5 as libc::c_int as uint32_t,
    0x3f9b762c as libc::c_int as uint32_t,
    0x3b5a6b9b as libc::c_int as uint32_t,
    0x315d626 as libc::c_int as uint32_t,
    0x7d4cb91 as libc::c_int as uint32_t,
    0xa97ed48 as libc::c_int as uint32_t,
    0xe56f0ff as libc::c_int as uint32_t,
    0x1011a0fa as libc::c_int as uint32_t,
    0x14d0bd4d as libc::c_int as uint32_t,
    0x19939b94 as libc::c_int as uint32_t,
    0x1d528623 as libc::c_int as uint32_t,
    0xf12f560e as libc::c_uint,
    0xf5ee4bb9 as libc::c_uint,
    0xf8ad6d60 as libc::c_uint,
    0xfc6c70d7 as libc::c_uint,
    0xe22b20d2 as libc::c_uint,
    0xe6ea3d65 as libc::c_uint,
    0xeba91bbc as libc::c_uint,
    0xef68060b as libc::c_uint,
    0xd727bbb6 as libc::c_uint,
    0xd3e6a601 as libc::c_uint,
    0xdea580d8 as libc::c_uint,
    0xda649d6f as libc::c_uint,
    0xc423cd6a as libc::c_uint,
    0xc0e2d0dd as libc::c_uint,
    0xcda1f604 as libc::c_uint,
    0xc960ebb3 as libc::c_uint,
    0xbd3e8d7e as libc::c_uint,
    0xb9ff90c9 as libc::c_uint,
    0xb4bcb610 as libc::c_uint,
    0xb07daba7 as libc::c_uint,
    0xae3afba2 as libc::c_uint,
    0xaafbe615 as libc::c_uint,
    0xa7b8c0cc as libc::c_uint,
    0xa379dd7b as libc::c_uint,
    0x9b3660c6 as libc::c_uint,
    0x9ff77d71 as libc::c_uint,
    0x92b45ba8 as libc::c_uint,
    0x9675461f as libc::c_uint,
    0x8832161a as libc::c_uint,
    0x8cf30bad as libc::c_uint,
    0x81b02d74 as libc::c_uint,
    0x857130c3 as libc::c_uint,
    0x5d8a9099 as libc::c_int as uint32_t,
    0x594b8d2e as libc::c_int as uint32_t,
    0x5408abf7 as libc::c_int as uint32_t,
    0x50c9b640 as libc::c_int as uint32_t,
    0x4e8ee645 as libc::c_int as uint32_t,
    0x4a4ffbf2 as libc::c_int as uint32_t,
    0x470cdd2b as libc::c_int as uint32_t,
    0x43cdc09c as libc::c_int as uint32_t,
    0x7b827d21 as libc::c_int as uint32_t,
    0x7f436096 as libc::c_int as uint32_t,
    0x7200464f as libc::c_int as uint32_t,
    0x76c15bf8 as libc::c_int as uint32_t,
    0x68860bfd as libc::c_int as uint32_t,
    0x6c47164a as libc::c_int as uint32_t,
    0x61043093 as libc::c_int as uint32_t,
    0x65c52d24 as libc::c_int as uint32_t,
    0x119b4be9 as libc::c_int as uint32_t,
    0x155a565e as libc::c_int as uint32_t,
    0x18197087 as libc::c_int as uint32_t,
    0x1cd86d30 as libc::c_int as uint32_t,
    0x29f3d35 as libc::c_int as uint32_t,
    0x65e2082 as libc::c_int as uint32_t,
    0xb1d065b as libc::c_int as uint32_t,
    0xfdc1bec as libc::c_int as uint32_t,
    0x3793a651 as libc::c_int as uint32_t,
    0x3352bbe6 as libc::c_int as uint32_t,
    0x3e119d3f as libc::c_int as uint32_t,
    0x3ad08088 as libc::c_int as uint32_t,
    0x2497d08d as libc::c_int as uint32_t,
    0x2056cd3a as libc::c_int as uint32_t,
    0x2d15ebe3 as libc::c_int as uint32_t,
    0x29d4f654 as libc::c_int as uint32_t,
    0xc5a92679 as libc::c_uint,
    0xc1683bce as libc::c_uint,
    0xcc2b1d17 as libc::c_uint,
    0xc8ea00a0 as libc::c_uint,
    0xd6ad50a5 as libc::c_uint,
    0xd26c4d12 as libc::c_uint,
    0xdf2f6bcb as libc::c_uint,
    0xdbee767c as libc::c_uint,
    0xe3a1cbc1 as libc::c_uint,
    0xe760d676 as libc::c_uint,
    0xea23f0af as libc::c_uint,
    0xeee2ed18 as libc::c_uint,
    0xf0a5bd1d as libc::c_uint,
    0xf464a0aa as libc::c_uint,
    0xf9278673 as libc::c_uint,
    0xfde69bc4 as libc::c_uint,
    0x89b8fd09 as libc::c_uint,
    0x8d79e0be as libc::c_uint,
    0x803ac667 as libc::c_uint,
    0x84fbdbd0 as libc::c_uint,
    0x9abc8bd5 as libc::c_uint,
    0x9e7d9662 as libc::c_uint,
    0x933eb0bb as libc::c_uint,
    0x97ffad0c as libc::c_uint,
    0xafb010b1 as libc::c_uint,
    0xab710d06 as libc::c_uint,
    0xa6322bdf as libc::c_uint,
    0xa2f33668 as libc::c_uint,
    0xbcb4666d as libc::c_uint,
    0xb8757bda as libc::c_uint,
    0xb5365d03 as libc::c_uint,
    0xb1f740b4 as libc::c_uint,
];
static mut safe_char: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn mtree_quote(mut s: *mut archive_string, mut str: *const libc::c_char) {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 4] = [0; 4];
    let mut c: libc::c_uchar = 0;
    start = str;
    while *str as libc::c_int != '\u{0}' as i32 {
        if !(safe_char[*(str as *const libc::c_uchar) as usize] != 0) {
            if start != str {
                archive_strncat(
                    s,
                    start as *const libc::c_void,
                    str.offset_from(start) as libc::c_long as size_t,
                );
            }
            c = *str as libc::c_uchar;
            buf[0 as libc::c_int as usize] = '\\' as i32 as libc::c_char;
            buf[1 as libc::c_int as usize] =
                (c as libc::c_int / 64 as libc::c_int + '0' as i32) as libc::c_char;
            buf[2 as libc::c_int as usize] = (c as libc::c_int / 8 as libc::c_int
                % 8 as libc::c_int
                + '0' as i32) as libc::c_char;
            buf[3 as libc::c_int as usize] =
                (c as libc::c_int % 8 as libc::c_int + '0' as i32) as libc::c_char;
            archive_strncat(
                s,
                buf.as_mut_ptr() as *const libc::c_void,
                4 as libc::c_int as size_t,
            );
            start = str.offset(1 as libc::c_int as isize)
        }
        str = str.offset(1)
    }
    if start != str {
        archive_strncat(
            s,
            start as *const libc::c_void,
            str.offset_from(start) as libc::c_long as size_t,
        );
    };
}
/*
 * Indent a line as the mtree utility does so it is readable for people.
 */
unsafe extern "C" fn mtree_indent(mut mtree: *mut mtree_writer) {
    let mut i: libc::c_int = 0;
    let mut fn_0: libc::c_int = 0;
    let mut nd: libc::c_int = 0;
    let mut pd: libc::c_int = 0;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut x: *const libc::c_char = 0 as *const libc::c_char;
    if (*mtree).classic != 0 {
        if (*mtree).indent != 0 {
            nd = 0 as libc::c_int;
            pd = (*mtree).depth * 4 as libc::c_int
        } else {
            nd = if (*mtree).depth != 0 {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            };
            pd = 0 as libc::c_int
        }
    } else {
        pd = 0 as libc::c_int;
        nd = pd
    }
    fn_0 = 1 as libc::c_int;
    r = (*mtree).ebuf.s;
    s = r;
    x = NULL as *const libc::c_char;
    while *r as libc::c_int == ' ' as i32 {
        r = r.offset(1)
    }
    loop {
        r = strchr(r, ' ' as i32);
        if r.is_null() {
            break;
        }
        if fn_0 != 0 {
            fn_0 = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < nd + pd {
                archive_strappend_char(&mut (*mtree).buf, ' ' as i32 as libc::c_char);
                i += 1
            }
            archive_strncat(
                &mut (*mtree).buf,
                s as *const libc::c_void,
                r.offset_from(s) as libc::c_long as size_t,
            );
            if nd as libc::c_long + r.offset_from(s) as libc::c_long
                > INDENTNAMELEN as libc::c_long
            {
                archive_strncat(
                    &mut (*mtree).buf,
                    b" \\\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    3 as libc::c_int as size_t,
                );
                i = 0 as libc::c_int;
                while i < INDENTNAMELEN + 1 as libc::c_int + pd {
                    archive_strappend_char(&mut (*mtree).buf, ' ' as i32 as libc::c_char);
                    i += 1
                }
            } else {
                i = (r.offset_from(s) as libc::c_long + nd as libc::c_long) as libc::c_int;
                while i < INDENTNAMELEN + 1 as libc::c_int {
                    archive_strappend_char(&mut (*mtree).buf, ' ' as i32 as libc::c_char);
                    i += 1
                }
            }
            r = r.offset(1);
            s = r;
            x = NULL as *const libc::c_char
        } else if pd as libc::c_long + r.offset_from(s) as libc::c_long
            <= (MAXLINELEN - 3 as libc::c_int - INDENTNAMELEN) as libc::c_long
        {
            let fresh0 = r;
            r = r.offset(1);
            x = fresh0
        } else {
            if x.is_null() {
                x = r
            }
            archive_strncat(
                &mut (*mtree).buf,
                s as *const libc::c_void,
                x.offset_from(s) as libc::c_long as size_t,
            );
            archive_strncat(
                &mut (*mtree).buf,
                b" \\\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as size_t,
            );
            i = 0 as libc::c_int;
            while i < INDENTNAMELEN + 1 as libc::c_int + pd {
                archive_strappend_char(&mut (*mtree).buf, ' ' as i32 as libc::c_char);
                i += 1
            }
            x = x.offset(1);
            r = x;
            s = r;
            x = NULL as *const libc::c_char
        }
    }
    if fn_0 != 0 {
        i = 0 as libc::c_int;
        while i < nd + pd {
            archive_strappend_char(&mut (*mtree).buf, ' ' as i32 as libc::c_char);
            i += 1
        }
        archive_strcat(&mut (*mtree).buf, s as *const libc::c_void);
        s = s.offset(strlen(s) as isize)
    }
    if !x.is_null()
        && (pd as libc::c_ulong).wrapping_add(strlen(s))
            > (MAXLINELEN - 3 as libc::c_int - INDENTNAMELEN) as libc::c_ulong
    {
        /* Last keyword is longer. */
        archive_strncat(
            &mut (*mtree).buf,
            s as *const libc::c_void,
            x.offset_from(s) as libc::c_long as size_t,
        );
        archive_strncat(
            &mut (*mtree).buf,
            b" \\\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
        i = 0 as libc::c_int;
        while i < INDENTNAMELEN + 1 as libc::c_int + pd {
            archive_strappend_char(&mut (*mtree).buf, ' ' as i32 as libc::c_char);
            i += 1
        }
        x = x.offset(1);
        s = x
    }
    archive_strcat(&mut (*mtree).buf, s as *const libc::c_void);
    (*mtree).ebuf.length = 0 as libc::c_int as size_t;
}
/*
 * Write /set keyword.
 * Set the most used value of uid, gid, mode and fflags, which are
 * collected by the attr_counter_set_collect() function.
 */
unsafe extern "C" fn write_global(mut mtree: *mut mtree_writer) {
    let mut setstr: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut unsetstr: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut acs: *mut att_counter_set = 0 as *mut att_counter_set;
    let mut keys: libc::c_int = 0;
    let mut oldkeys: libc::c_int = 0;
    let mut effkeys: libc::c_int = 0;
    setstr.s = NULL as *mut libc::c_char;
    setstr.length = 0 as libc::c_int as size_t;
    setstr.buffer_length = 0 as libc::c_int as size_t;
    unsetstr.s = NULL as *mut libc::c_char;
    unsetstr.length = 0 as libc::c_int as size_t;
    unsetstr.buffer_length = 0 as libc::c_int as size_t;
    keys = (*mtree).keys & SET_KEYS;
    oldkeys = (*mtree).set.keys;
    effkeys = keys;
    acs = &mut (*mtree).acs;
    if (*mtree).set.processing != 0 {
        /*
         * Check if the global data needs updating.
         */
        effkeys &= !F_TYPE;
        if (*acs).uid_list.is_null() {
            effkeys &= !(F_UNAME | F_UID)
        } else if oldkeys & (F_UNAME | F_UID) != 0 {
            if (*(*acs).uid_list).count < 2 as libc::c_int
                || (*mtree).set.uid == (*(*(*acs).uid_list).m_entry).uid
            {
                effkeys &= !(F_UNAME | F_UID)
            }
        }
        if (*acs).gid_list.is_null() {
            effkeys &= !(F_GNAME | F_GID)
        } else if oldkeys & (F_GNAME | F_GID) != 0 {
            if (*(*acs).gid_list).count < 2 as libc::c_int
                || (*mtree).set.gid == (*(*(*acs).gid_list).m_entry).gid
            {
                effkeys &= !(F_GNAME | F_GID)
            }
        }
        if (*acs).mode_list.is_null() {
            effkeys &= !F_MODE
        } else if oldkeys & F_MODE != 0 {
            if (*(*acs).mode_list).count < 2 as libc::c_int
                || (*mtree).set.mode == (*(*(*acs).mode_list).m_entry).mode
            {
                effkeys &= !F_MODE
            }
        }
        if (*acs).flags_list.is_null() {
            effkeys &= !F_FLAGS
        } else if oldkeys & F_FLAGS != 0 as libc::c_int {
            if (*(*acs).flags_list).count < 2 as libc::c_int
                || (*(*(*acs).flags_list).m_entry).fflags_set == (*mtree).set.fflags_set
                    && (*(*(*acs).flags_list).m_entry).fflags_clear == (*mtree).set.fflags_clear
            {
                effkeys &= !F_FLAGS
            }
        }
    } else {
        if (*acs).uid_list.is_null() {
            keys &= !(F_UNAME | F_UID)
        }
        if (*acs).gid_list.is_null() {
            keys &= !(F_GNAME | F_GID)
        }
        if (*acs).mode_list.is_null() {
            keys &= !F_MODE
        }
        if (*acs).flags_list.is_null() {
            keys &= !F_FLAGS
        }
    }
    if keys & effkeys & F_TYPE != 0 as libc::c_int {
        if (*mtree).dironly != 0 {
            archive_strcat(
                &mut setstr,
                b" type=dir\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            (*mtree).set.type_0 = AE_IFDIR as mode_t
        } else {
            archive_strcat(
                &mut setstr,
                b" type=file\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            (*mtree).set.type_0 = AE_IFREG as mode_t
        }
    }
    if keys & effkeys & F_UNAME != 0 as libc::c_int {
        if (*(*(*acs).uid_list).m_entry).uname.length > 0 as libc::c_int as libc::c_ulong {
            archive_strcat(
                &mut setstr,
                b" uname=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            mtree_quote(&mut setstr, (*(*(*acs).uid_list).m_entry).uname.s);
        } else {
            keys &= !F_UNAME;
            if oldkeys & F_UNAME != 0 as libc::c_int {
                archive_strcat(
                    &mut unsetstr,
                    b" uname\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
        }
    }
    if keys & effkeys & F_UID != 0 as libc::c_int {
        (*mtree).set.uid = (*(*(*acs).uid_list).m_entry).uid;
        archive_string_sprintf(
            &mut setstr as *mut archive_string,
            b" uid=%jd\x00" as *const u8 as *const libc::c_char,
            (*mtree).set.uid,
        );
    }
    if keys & effkeys & F_GNAME != 0 as libc::c_int {
        if (*(*(*acs).gid_list).m_entry).gname.length > 0 as libc::c_int as libc::c_ulong {
            archive_strcat(
                &mut setstr,
                b" gname=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            mtree_quote(&mut setstr, (*(*(*acs).gid_list).m_entry).gname.s);
        } else {
            keys &= !F_GNAME;
            if oldkeys & F_GNAME != 0 as libc::c_int {
                archive_strcat(
                    &mut unsetstr,
                    b" gname\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
        }
    }
    if keys & effkeys & F_GID != 0 as libc::c_int {
        (*mtree).set.gid = (*(*(*acs).gid_list).m_entry).gid;
        archive_string_sprintf(
            &mut setstr as *mut archive_string,
            b" gid=%jd\x00" as *const u8 as *const libc::c_char,
            (*mtree).set.gid,
        );
    }
    if keys & effkeys & F_MODE != 0 as libc::c_int {
        (*mtree).set.mode = (*(*(*acs).mode_list).m_entry).mode;
        archive_string_sprintf(
            &mut setstr as *mut archive_string,
            b" mode=%o\x00" as *const u8 as *const libc::c_char,
            (*mtree).set.mode,
        );
    }
    if keys & effkeys & F_FLAGS != 0 as libc::c_int {
        if (*(*(*acs).flags_list).m_entry).fflags_text.length > 0 as libc::c_int as libc::c_ulong {
            archive_strcat(
                &mut setstr,
                b" flags=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            mtree_quote(&mut setstr, (*(*(*acs).flags_list).m_entry).fflags_text.s);
            (*mtree).set.fflags_set = (*(*(*acs).flags_list).m_entry).fflags_set;
            (*mtree).set.fflags_clear = (*(*(*acs).flags_list).m_entry).fflags_clear
        } else {
            keys &= !F_FLAGS;
            if oldkeys & F_FLAGS != 0 as libc::c_int {
                archive_strcat(
                    &mut unsetstr,
                    b" flags\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
        }
    }
    if unsetstr.length > 0 as libc::c_int as libc::c_ulong {
        archive_string_sprintf(
            &mut (*mtree).buf as *mut archive_string,
            b"/unset%s\n\x00" as *const u8 as *const libc::c_char,
            unsetstr.s,
        );
    }
    archive_string_free(&mut unsetstr);
    if setstr.length > 0 as libc::c_int as libc::c_ulong {
        archive_string_sprintf(
            &mut (*mtree).buf as *mut archive_string,
            b"/set%s\n\x00" as *const u8 as *const libc::c_char,
            setstr.s,
        );
    }
    archive_string_free(&mut setstr);
    (*mtree).set.keys = keys;
    (*mtree).set.processing = 1 as libc::c_int;
}
unsafe extern "C" fn attr_counter_new(
    mut me: *mut mtree_entry,
    mut prev: *mut attr_counter,
) -> *mut attr_counter {
    let mut ac: *mut attr_counter = 0 as *mut attr_counter;
    ac = malloc(::std::mem::size_of::<attr_counter>() as libc::c_ulong) as *mut attr_counter;
    if !ac.is_null() {
        (*ac).prev = prev;
        (*ac).next = NULL as *mut attr_counter;
        (*ac).count = 1 as libc::c_int;
        (*ac).m_entry = me
    }
    return ac;
}
unsafe extern "C" fn attr_counter_free(mut top: *mut *mut attr_counter) {
    let mut ac: *mut attr_counter = 0 as *mut attr_counter;
    let mut tac: *mut attr_counter = 0 as *mut attr_counter;
    if (*top).is_null() {
        return;
    }
    ac = *top;
    while !ac.is_null() {
        tac = (*ac).next;
        free(ac as *mut libc::c_void);
        ac = tac
    }
    *top = NULL as *mut attr_counter;
}
unsafe extern "C" fn attr_counter_inc(
    mut top: *mut *mut attr_counter,
    mut ac: *mut attr_counter,
    mut last: *mut attr_counter,
    mut me: *mut mtree_entry,
) -> libc::c_int {
    let mut pac: *mut attr_counter = 0 as *mut attr_counter;
    if !ac.is_null() {
        (*ac).count += 1;
        if *top == ac || (*(*ac).prev).count >= (*ac).count {
            return 0 as libc::c_int;
        }
        pac = (*ac).prev;
        while !pac.is_null() {
            if (*pac).count >= (*ac).count {
                break;
            }
            pac = (*pac).prev
        }
        (*(*ac).prev).next = (*ac).next;
        if !(*ac).next.is_null() {
            (*(*ac).next).prev = (*ac).prev
        }
        if !pac.is_null() {
            (*ac).prev = pac;
            (*ac).next = (*pac).next;
            (*pac).next = ac;
            if !(*ac).next.is_null() {
                (*(*ac).next).prev = ac
            }
        } else {
            (*ac).prev = NULL as *mut attr_counter;
            (*ac).next = *top;
            *top = ac;
            (*(*ac).next).prev = ac
        }
    } else if !last.is_null() {
        ac = attr_counter_new(me, last);
        if ac.is_null() {
            return -(1 as libc::c_int);
        }
        (*last).next = ac
    }
    return 0 as libc::c_int;
}
/*
 * Tabulate uid, gid, mode and fflags of a entry in order to be used for /set.
 */
unsafe extern "C" fn attr_counter_set_collect(
    mut mtree: *mut mtree_writer,
    mut me: *mut mtree_entry,
) -> libc::c_int {
    let mut ac: *mut attr_counter = 0 as *mut attr_counter;
    let mut last: *mut attr_counter = 0 as *mut attr_counter;
    let mut acs: *mut att_counter_set = &mut (*mtree).acs;
    let mut keys: libc::c_int = (*mtree).keys;
    if keys & (F_UNAME | F_UID) != 0 {
        if (*acs).uid_list.is_null() {
            (*acs).uid_list = attr_counter_new(me, NULL as *mut attr_counter);
            if (*acs).uid_list.is_null() {
                return -(1 as libc::c_int);
            }
        } else {
            last = NULL as *mut attr_counter;
            ac = (*acs).uid_list;
            while !ac.is_null() {
                if (*(*ac).m_entry).uid == (*me).uid {
                    break;
                }
                last = ac;
                ac = (*ac).next
            }
            if attr_counter_inc(&mut (*acs).uid_list, ac, last, me) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
    }
    if keys & (F_GNAME | F_GID) != 0 {
        if (*acs).gid_list.is_null() {
            (*acs).gid_list = attr_counter_new(me, NULL as *mut attr_counter);
            if (*acs).gid_list.is_null() {
                return -(1 as libc::c_int);
            }
        } else {
            last = NULL as *mut attr_counter;
            ac = (*acs).gid_list;
            while !ac.is_null() {
                if (*(*ac).m_entry).gid == (*me).gid {
                    break;
                }
                last = ac;
                ac = (*ac).next
            }
            if attr_counter_inc(&mut (*acs).gid_list, ac, last, me) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
    }
    if keys & F_MODE != 0 {
        if (*acs).mode_list.is_null() {
            (*acs).mode_list = attr_counter_new(me, NULL as *mut attr_counter);
            if (*acs).mode_list.is_null() {
                return -(1 as libc::c_int);
            }
        } else {
            last = NULL as *mut attr_counter;
            ac = (*acs).mode_list;
            while !ac.is_null() {
                if (*(*ac).m_entry).mode == (*me).mode {
                    break;
                }
                last = ac;
                ac = (*ac).next
            }
            if attr_counter_inc(&mut (*acs).mode_list, ac, last, me) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
    }
    if keys & F_FLAGS != 0 {
        if (*acs).flags_list.is_null() {
            (*acs).flags_list = attr_counter_new(me, NULL as *mut attr_counter);
            if (*acs).flags_list.is_null() {
                return -(1 as libc::c_int);
            }
        } else {
            last = NULL as *mut attr_counter;
            ac = (*acs).flags_list;
            while !ac.is_null() {
                if (*(*ac).m_entry).fflags_set == (*me).fflags_set
                    && (*(*ac).m_entry).fflags_clear == (*me).fflags_clear
                {
                    break;
                }
                last = ac;
                ac = (*ac).next
            }
            if attr_counter_inc(&mut (*acs).flags_list, ac, last, me) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn attr_counter_set_free(mut mtree: *mut mtree_writer) {
    let mut acs: *mut att_counter_set = &mut (*mtree).acs;
    attr_counter_free(&mut (*acs).uid_list);
    attr_counter_free(&mut (*acs).gid_list);
    attr_counter_free(&mut (*acs).mode_list);
    attr_counter_free(&mut (*acs).flags_list);
}
unsafe extern "C" fn get_global_set_keys(
    mut mtree: *mut mtree_writer,
    mut me: *mut mtree_entry,
) -> libc::c_int {
    let mut keys: libc::c_int = 0;
    keys = (*mtree).keys;
    /*
     * If a keyword has been set by /set, we do not need to
     * output it.
     */
    if (*mtree).set.keys == 0 as libc::c_int {
        return keys;
    } /* /set is not used. */
    if (*mtree).set.keys & (F_GNAME | F_GID) != 0 as libc::c_int && (*mtree).set.gid == (*me).gid {
        keys &= !(F_GNAME | F_GID)
    }
    if (*mtree).set.keys & (F_UNAME | F_UID) != 0 as libc::c_int && (*mtree).set.uid == (*me).uid {
        keys &= !(F_UNAME | F_UID)
    }
    if (*mtree).set.keys & F_FLAGS != 0 {
        if (*mtree).set.fflags_set == (*me).fflags_set
            && (*mtree).set.fflags_clear == (*me).fflags_clear
        {
            keys &= !F_FLAGS
        }
    }
    if (*mtree).set.keys & F_MODE != 0 as libc::c_int && (*mtree).set.mode == (*me).mode {
        keys &= !F_MODE
    }
    match (*me).filetype {
        40960 | 49152 | 8192 | 24576 | 4096 => {}
        16384 => {
            if (*mtree).set.keys & F_TYPE != 0 as libc::c_int
                && (*mtree).set.type_0 == AE_IFDIR as mode_t
            {
                keys &= !F_TYPE
            }
        }
        32768 | _ => {
            /* Handle unknown file types as regular files. */
            if (*mtree).set.keys & F_TYPE != 0 as libc::c_int
                && (*mtree).set.type_0 == AE_IFREG as mode_t
            {
                keys &= !F_TYPE
            }
        }
    }
    return keys;
}
unsafe extern "C" fn mtree_entry_new(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut m_entry: *mut *mut mtree_entry,
) -> libc::c_int {
    let mut me: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    mtree_entry_cmp_node
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    mtree_entry_cmp_key
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    me = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mtree_entry>() as libc::c_ulong,
    ) as *mut mtree_entry;
    if me.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for a mtree entry\x00" as *const u8 as *const libc::c_char,
        );
        *m_entry = NULL as *mut mtree_entry;
        return -(30 as libc::c_int);
    }
    r = mtree_entry_setup_filenames(a, me, entry);
    if r < ARCHIVE_WARN {
        mtree_entry_free(me);
        *m_entry = NULL as *mut mtree_entry;
        return r;
    }
    s = archive_entry_symlink(entry);
    if !s.is_null() {
        (*me).symlink.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*me).symlink,
            s as *const libc::c_void,
            (if s.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(s)
            }),
        );
    }
    (*me).nlink = archive_entry_nlink(entry);
    (*me).filetype = archive_entry_filetype(entry);
    (*me).mode = archive_entry_mode(entry) & 0o7777 as libc::c_int as libc::c_uint;
    (*me).uid = archive_entry_uid(entry);
    (*me).gid = archive_entry_gid(entry);
    s = archive_entry_uname(entry);
    if !s.is_null() {
        (*me).uname.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*me).uname,
            s as *const libc::c_void,
            (if s.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(s)
            }),
        );
    }
    s = archive_entry_gname(entry);
    if !s.is_null() {
        (*me).gname.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*me).gname,
            s as *const libc::c_void,
            (if s.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(s)
            }),
        );
    }
    s = archive_entry_fflags_text(entry);
    if !s.is_null() {
        (*me).fflags_text.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*me).fflags_text,
            s as *const libc::c_void,
            (if s.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(s)
            }),
        );
    }
    archive_entry_fflags(entry, &mut (*me).fflags_set, &mut (*me).fflags_clear);
    (*me).mtime = archive_entry_mtime(entry);
    (*me).mtime_nsec = archive_entry_mtime_nsec(entry);
    (*me).rdevmajor = archive_entry_rdevmajor(entry);
    (*me).rdevminor = archive_entry_rdevminor(entry);
    (*me).devmajor = archive_entry_devmajor(entry);
    (*me).devminor = archive_entry_devminor(entry);
    (*me).ino = archive_entry_ino(entry);
    (*me).size = archive_entry_size(entry);
    if (*me).filetype == AE_IFDIR as mode_t {
        (*me).dir_info = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<dir_info>() as libc::c_ulong,
        ) as *mut dir_info;
        if (*me).dir_info.is_null() {
            mtree_entry_free(me);
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for a mtree entry\x00" as *const u8 as *const libc::c_char,
            );
            *m_entry = NULL as *mut mtree_entry;
            return -(30 as libc::c_int);
        }
        __archive_rb_tree_init(&mut (*(*me).dir_info).rbtree, &rb_ops);
        (*(*me).dir_info).children.first = NULL as *mut mtree_entry;
        (*(*me).dir_info).children.last = &mut (*(*me).dir_info).children.first;
        (*(*me).dir_info).chnext = NULL as *mut mtree_entry
    } else if (*me).filetype == AE_IFREG as mode_t {
        (*me).reg_info = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<reg_info>() as libc::c_ulong,
        ) as *mut reg_info;
        if (*me).reg_info.is_null() {
            mtree_entry_free(me);
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for a mtree entry\x00" as *const u8 as *const libc::c_char,
            );
            *m_entry = NULL as *mut mtree_entry;
            return -(30 as libc::c_int);
        }
        (*(*me).reg_info).compute_sum = 0 as libc::c_int
    }
    *m_entry = me;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mtree_entry_free(mut me: *mut mtree_entry) {
    archive_string_free(&mut (*me).parentdir);
    archive_string_free(&mut (*me).basename);
    archive_string_free(&mut (*me).pathname);
    archive_string_free(&mut (*me).symlink);
    archive_string_free(&mut (*me).uname);
    archive_string_free(&mut (*me).gname);
    archive_string_free(&mut (*me).fflags_text);
    free((*me).dir_info as *mut libc::c_void);
    free((*me).reg_info as *mut libc::c_void);
    free(me as *mut libc::c_void);
}
unsafe extern "C" fn archive_write_mtree_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut mtree_entry: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut r: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    if (*mtree).first != 0 {
        (*mtree).first = 0 as libc::c_int;
        archive_strcat(
            &mut (*mtree).buf,
            b"#mtree\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        if (*mtree).keys & SET_KEYS == 0 as libc::c_int {
            (*mtree).output_global_set = 0 as libc::c_int
        }
        /* Disabled. */
    }
    (*mtree).entry_bytes_remaining = archive_entry_size(entry) as uint64_t;
    /* While directory only mode, we do not handle non directory files. */
    if (*mtree).dironly != 0 && archive_entry_filetype(entry) != AE_IFDIR as mode_t {
        return 0 as libc::c_int;
    }
    r2 = mtree_entry_new(a, entry, &mut mtree_entry);
    if r2 < ARCHIVE_WARN {
        return r2;
    }
    r = mtree_entry_tree_add(a, &mut mtree_entry);
    if r < ARCHIVE_WARN {
        mtree_entry_free(mtree_entry);
        return r;
    }
    (*mtree).mtree_entry = mtree_entry;
    /* If the current file is a regular file, we have to
     * compute the sum of its content.
     * Initialize a bunch of checksum context. */
    if !(*mtree_entry).reg_info.is_null() {
        sum_init(mtree);
    }
    return r2;
}
unsafe extern "C" fn write_mtree_entry(
    mut a: *mut archive_write,
    mut me: *mut mtree_entry,
) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut str: *mut archive_string = 0 as *mut archive_string;
    let mut keys: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if !(*me).dir_info.is_null() {
        if (*mtree).classic != 0 {
            /*
             * Output a comment line to describe the full
             * pathname of the entry as mtree utility does
             * while generating classic format.
             */
            if (*mtree).dironly == 0 {
                archive_strappend_char(&mut (*mtree).buf, '\n' as i32 as libc::c_char);
            }
            if !(*me).parentdir.s.is_null() {
                archive_string_sprintf(
                    &mut (*mtree).buf as *mut archive_string,
                    b"# %s/%s\n\x00" as *const u8 as *const libc::c_char,
                    (*me).parentdir.s,
                    (*me).basename.s,
                );
            } else {
                archive_string_sprintf(
                    &mut (*mtree).buf as *mut archive_string,
                    b"# %s\n\x00" as *const u8 as *const libc::c_char,
                    (*me).basename.s,
                );
            }
        }
        if (*mtree).output_global_set != 0 {
            write_global(mtree);
        }
    }
    (*mtree).ebuf.length = 0 as libc::c_int as size_t;
    str = if (*mtree).indent != 0 || (*mtree).classic != 0 {
        &mut (*mtree).ebuf
    } else {
        &mut (*mtree).buf
    };
    if (*mtree).classic == 0 && !(*me).parentdir.s.is_null() {
        /*
         * If generating format is not classic one(v1), output
         * a full pathname.
         */
        mtree_quote(str, (*me).parentdir.s);
        archive_strappend_char(str, '/' as i32 as libc::c_char);
    }
    mtree_quote(str, (*me).basename.s);
    keys = get_global_set_keys(mtree, me);
    if keys & F_NLINK != 0 as libc::c_int
        && (*me).nlink != 1 as libc::c_int as libc::c_uint
        && (*me).filetype != AE_IFDIR as mode_t
    {
        archive_string_sprintf(
            str,
            b" nlink=%u\x00" as *const u8 as *const libc::c_char,
            (*me).nlink,
        );
    }
    if keys & F_GNAME != 0 as libc::c_int && (*me).gname.length > 0 as libc::c_int as libc::c_ulong
    {
        archive_strcat(
            str,
            b" gname=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        mtree_quote(str, (*me).gname.s);
    }
    if keys & F_UNAME != 0 as libc::c_int && (*me).uname.length > 0 as libc::c_int as libc::c_ulong
    {
        archive_strcat(
            str,
            b" uname=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        mtree_quote(str, (*me).uname.s);
    }
    if keys & F_FLAGS != 0 as libc::c_int {
        if (*me).fflags_text.length > 0 as libc::c_int as libc::c_ulong {
            archive_strcat(
                str,
                b" flags=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            mtree_quote(str, (*me).fflags_text.s);
        } else if (*mtree).set.processing != 0 && (*mtree).set.keys & F_FLAGS != 0 as libc::c_int {
            /* Overwrite the global parameter. */
            archive_strcat(
                str,
                b" flags=none\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
        }
    }
    if keys & F_TIME != 0 as libc::c_int {
        archive_string_sprintf(
            str,
            b" time=%jd.%jd\x00" as *const u8 as *const libc::c_char,
            (*me).mtime,
            (*me).mtime_nsec,
        );
    }
    if keys & F_MODE != 0 as libc::c_int {
        archive_string_sprintf(
            str,
            b" mode=%o\x00" as *const u8 as *const libc::c_char,
            (*me).mode,
        );
    }
    if keys & F_GID != 0 as libc::c_int {
        archive_string_sprintf(
            str,
            b" gid=%jd\x00" as *const u8 as *const libc::c_char,
            (*me).gid,
        );
    }
    if keys & F_UID != 0 as libc::c_int {
        archive_string_sprintf(
            str,
            b" uid=%jd\x00" as *const u8 as *const libc::c_char,
            (*me).uid,
        );
    }
    if keys & F_INO != 0 as libc::c_int {
        archive_string_sprintf(
            str,
            b" inode=%jd\x00" as *const u8 as *const libc::c_char,
            (*me).ino,
        );
    }
    if keys & F_RESDEV != 0 as libc::c_int {
        archive_string_sprintf(
            str,
            b" resdevice=native,%ju,%ju\x00" as *const u8 as *const libc::c_char,
            (*me).devmajor,
            (*me).devminor,
        );
    }
    match (*me).filetype {
        40960 => {
            if keys & F_TYPE != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" type=link\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            if keys & F_SLINK != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" link=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
                mtree_quote(str, (*me).symlink.s);
            }
        }
        49152 => {
            if keys & F_TYPE != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" type=socket\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
        }
        8192 => {
            if keys & F_TYPE != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" type=char\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            if keys & F_DEV != 0 as libc::c_int {
                archive_string_sprintf(
                    str,
                    b" device=native,%ju,%ju\x00" as *const u8 as *const libc::c_char,
                    (*me).rdevmajor,
                    (*me).rdevminor,
                );
            }
        }
        24576 => {
            if keys & F_TYPE != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" type=block\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            if keys & F_DEV != 0 as libc::c_int {
                archive_string_sprintf(
                    str,
                    b" device=native,%ju,%ju\x00" as *const u8 as *const libc::c_char,
                    (*me).rdevmajor,
                    (*me).rdevminor,
                );
            }
        }
        16384 => {
            if keys & F_TYPE != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" type=dir\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
        }
        4096 => {
            if keys & F_TYPE != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" type=fifo\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
        }
        32768 | _ => {
            /* Handle unknown file types as regular files. */
            if keys & F_TYPE != 0 as libc::c_int {
                archive_strcat(
                    str,
                    b" type=file\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
            }
            if keys & F_SIZE != 0 as libc::c_int {
                archive_string_sprintf(
                    str,
                    b" size=%jd\x00" as *const u8 as *const libc::c_char,
                    (*me).size,
                );
            }
        }
    }
    /* Write a bunch of sum. */
    if !(*me).reg_info.is_null() {
        sum_write(str, (*me).reg_info);
    }
    archive_strappend_char(str, '\n' as i32 as libc::c_char);
    if (*mtree).indent != 0 || (*mtree).classic != 0 {
        mtree_indent(mtree);
    }
    if (*mtree).buf.length > 32768 as libc::c_int as libc::c_ulong {
        ret = __archive_write_output(
            a,
            (*mtree).buf.s as *const libc::c_void,
            (*mtree).buf.length,
        );
        (*mtree).buf.length = 0 as libc::c_int as size_t
    } else {
        ret = ARCHIVE_OK
    }
    return ret;
}
unsafe extern "C" fn write_dot_dot_entry(
    mut a: *mut archive_write,
    mut n: *mut mtree_entry,
) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut ret: libc::c_int = 0;
    if !(*n).parentdir.s.is_null() {
        if (*mtree).indent != 0 {
            let mut i: libc::c_int = 0;
            let mut pd: libc::c_int = (*mtree).depth * 4 as libc::c_int;
            i = 0 as libc::c_int;
            while i < pd {
                archive_strappend_char(&mut (*mtree).buf, ' ' as i32 as libc::c_char);
                i += 1
            }
        }
        archive_string_sprintf(
            &mut (*mtree).buf as *mut archive_string,
            b"# %s/%s\n\x00" as *const u8 as *const libc::c_char,
            (*n).parentdir.s,
            (*n).basename.s,
        );
    }
    if (*mtree).indent != 0 {
        (*mtree).ebuf.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*mtree).ebuf,
            b"..\n\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            if (*mtree).dironly != 0 {
                3 as libc::c_int
            } else {
                4 as libc::c_int
            } as size_t,
        );
        mtree_indent(mtree);
    } else {
        archive_strncat(
            &mut (*mtree).buf,
            b"..\n\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            if (*mtree).dironly != 0 {
                3 as libc::c_int
            } else {
                4 as libc::c_int
            } as size_t,
        );
    }
    if (*mtree).buf.length > 32768 as libc::c_int as libc::c_ulong {
        ret = __archive_write_output(
            a,
            (*mtree).buf.s as *const libc::c_void,
            (*mtree).buf.length,
        );
        (*mtree).buf.length = 0 as libc::c_int as size_t
    } else {
        ret = ARCHIVE_OK
    }
    return ret;
}
/*
 * Write mtree entries saved at attr_counter_set_collect() function.
 */
unsafe extern "C" fn write_mtree_entry_tree(mut a: *mut archive_write) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut np: *mut mtree_entry = (*mtree).root;
    let mut n: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut ret: libc::c_int = 0;
    loop {
        if (*mtree).output_global_set != 0 {
            /*
             * Collect attribute information to know which value
             * is frequently used among the children.
             */
            attr_counter_set_free(mtree);
            n = __archive_rb_tree_iterate(
                &mut (*(*np).dir_info).rbtree,
                NULL as *mut archive_rb_node,
                ARCHIVE_RB_DIR_LEFT as libc::c_uint,
            );
            while !n.is_null() {
                let mut e: *mut mtree_entry = n as *mut mtree_entry;
                if attr_counter_set_collect(mtree, e) < 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                n = __archive_rb_tree_iterate(
                    &mut (*(*np).dir_info).rbtree,
                    n,
                    ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
                )
            }
        }
        if (*(*np).dir_info).virtual_0 == 0 || (*mtree).classic != 0 {
            ret = write_mtree_entry(a, np);
            if ret != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        } else if (*mtree).output_global_set != 0 {
            write_global(mtree);
        }
        /* Whenever output_global_set is enabled
         * output global value(/set keywords)
         * even if the directory entry is not allowed
         * to be written because the global values
         * can be used for the children. */
        /*
         * Output the attribute of all files except directory files.
         */
        (*mtree).depth += 1;
        n = __archive_rb_tree_iterate(
            &mut (*(*np).dir_info).rbtree,
            NULL as *mut archive_rb_node,
            ARCHIVE_RB_DIR_LEFT as libc::c_uint,
        );
        while !n.is_null() {
            let mut e_0: *mut mtree_entry = n as *mut mtree_entry;
            if !(*e_0).dir_info.is_null() {
                mtree_entry_add_child_tail(np, e_0);
            } else {
                ret = write_mtree_entry(a, e_0);
                if ret != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
            }
            n = __archive_rb_tree_iterate(
                &mut (*(*np).dir_info).rbtree,
                n,
                ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
            )
        }
        (*mtree).depth -= 1;
        if !(*(*np).dir_info).children.first.is_null() {
            /*
             * Descend the tree.
             */
            np = (*(*np).dir_info).children.first;
            if (*mtree).indent != 0 {
                (*mtree).depth += 1
            }
        } else {
            if (*mtree).classic != 0 {
                /*
                 * While printing mtree classic, if there are not
                 * any directory files(except "." and "..") in the
                 * directory, output two dots ".." as returning
                 * the parent directory.
                 */
                ret = write_dot_dot_entry(a, np);
                if ret != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
            }
            while np != (*np).parent {
                if (*(*np).dir_info).chnext.is_null() {
                    /*
                     * Ascend the tree; go back to the parent.
                     */
                    if (*mtree).indent != 0 {
                        (*mtree).depth -= 1
                    }
                    if (*mtree).classic != 0 {
                        ret = write_dot_dot_entry(a, (*np).parent);
                        if ret != ARCHIVE_OK {
                            return -(30 as libc::c_int);
                        }
                    }
                    np = (*np).parent
                } else {
                    /*
                     * Switch to next mtree entry in the directory.
                     */
                    np = (*(*np).dir_info).chnext;
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
unsafe extern "C" fn archive_write_mtree_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut me: *mut mtree_entry = 0 as *mut mtree_entry;
    me = (*mtree).mtree_entry;
    if me.is_null() {
        return 0 as libc::c_int;
    }
    (*mtree).mtree_entry = NULL as *mut mtree_entry;
    if !(*me).reg_info.is_null() {
        sum_final(mtree, (*me).reg_info);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_mtree_close(mut a: *mut archive_write) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut ret: libc::c_int = 0;
    if !(*mtree).root.is_null() {
        ret = write_mtree_entry_tree(a);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    archive_write_set_bytes_in_last_block(&mut (*a).archive, 1 as libc::c_int);
    return __archive_write_output(
        a,
        (*mtree).buf.s as *const libc::c_void,
        (*mtree).buf.length,
    );
}
unsafe extern "C" fn archive_write_mtree_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut n: size_t,
) -> ssize_t {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    if n > (*mtree).entry_bytes_remaining {
        n = (*mtree).entry_bytes_remaining
    }
    (*mtree).entry_bytes_remaining =
        ((*mtree).entry_bytes_remaining as libc::c_ulong).wrapping_sub(n) as uint64_t as uint64_t;
    /* We don't need to compute a regular file sum */
    if (*mtree).mtree_entry.is_null() {
        return n as ssize_t;
    }
    if (*(*mtree).mtree_entry).filetype == AE_IFREG as mode_t {
        sum_update(mtree, buff, n);
    }
    return n as ssize_t;
}
unsafe extern "C" fn archive_write_mtree_free(mut a: *mut archive_write) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    if mtree.is_null() {
        return 0 as libc::c_int;
    }
    /* Make sure we do not leave any entries. */
    mtree_entry_register_free(mtree);
    archive_string_free(&mut (*mtree).cur_dirstr);
    archive_string_free(&mut (*mtree).ebuf);
    archive_string_free(&mut (*mtree).buf);
    attr_counter_set_free(mtree);
    free(mtree as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_mtree_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut keybit: libc::c_int = 0 as libc::c_int;
    match *key.offset(0 as libc::c_int as isize) as libc::c_int {
        97 => {
            if strcmp(key, b"all\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = !(0 as libc::c_int)
            }
        }
        99 => {
            if strcmp(key, b"cksum\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_CKSUM
            }
        }
        100 => {
            if strcmp(key, b"device\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_DEV
            } else if strcmp(key, b"dironly\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*mtree).dironly = if !value.is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                return 0 as libc::c_int;
            }
        }
        102 => {
            if strcmp(key, b"flags\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_FLAGS
            }
        }
        103 => {
            if strcmp(key, b"gid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_GID
            } else if strcmp(key, b"gname\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                keybit = F_GNAME
            }
        }
        105 => {
            if strcmp(key, b"indent\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*mtree).indent = if !value.is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                return 0 as libc::c_int;
            } else {
                if strcmp(key, b"inode\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                {
                    keybit = F_INO
                }
            }
        }
        108 => {
            if strcmp(key, b"link\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_SLINK
            }
        }
        109 => {
            if strcmp(key, b"md5\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"md5digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                keybit = F_MD5
            }
            if strcmp(key, b"mode\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_MODE
            }
        }
        110 => {
            if strcmp(key, b"nlink\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_NLINK
            }
        }
        114 => {
            if strcmp(key, b"resdevice\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                keybit = F_RESDEV
            } else if strcmp(
                key,
                b"ripemd160digest\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
                || strcmp(key, b"rmd160\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                || strcmp(key, b"rmd160digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                keybit = F_RMD160
            }
        }
        115 => {
            if strcmp(key, b"sha1\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"sha1digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                keybit = F_SHA1
            }
            if strcmp(key, b"sha256\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"sha256digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                keybit = F_SHA256
            }
            if strcmp(key, b"sha384\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"sha384digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                keybit = F_SHA384
            }
            if strcmp(key, b"sha512\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"sha512digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                keybit = F_SHA512
            }
            if strcmp(key, b"size\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_SIZE
            }
        }
        116 => {
            if strcmp(key, b"time\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_TIME
            } else if strcmp(key, b"type\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                keybit = F_TYPE
            }
        }
        117 => {
            if strcmp(key, b"uid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                keybit = F_UID
            } else if strcmp(key, b"uname\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                keybit = F_UNAME
            } else if strcmp(key, b"use-set\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*mtree).output_global_set = if !value.is_null() {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    if keybit != 0 as libc::c_int {
        if !value.is_null() {
            (*mtree).keys |= keybit
        } else {
            (*mtree).keys &= !keybit
        }
        return 0 as libc::c_int;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn archive_write_set_format_mtree_default(
    mut _a: *mut archive,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut mtree: *mut mtree_writer = 0 as *mut mtree_writer;
    let mut magic_test: libc::c_int =
        __archive_check_magic(_a, 0xb0c5c0de as libc::c_uint, 1 as libc::c_uint, fn_0);
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    mtree = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mtree_writer>() as libc::c_ulong,
    ) as *mut mtree_writer;
    if mtree.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate mtree data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*mtree).mtree_entry = NULL as *mut mtree_entry;
    (*mtree).first = 1 as libc::c_int;
    memset(
        &mut (*mtree).set as *mut C2RustUnnamed as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong,
    );
    (*mtree).keys = DEFAULT_KEYS;
    (*mtree).dironly = 0 as libc::c_int;
    (*mtree).indent = 0 as libc::c_int;
    (*mtree).ebuf.s = NULL as *mut libc::c_char;
    (*mtree).ebuf.length = 0 as libc::c_int as size_t;
    (*mtree).ebuf.buffer_length = 0 as libc::c_int as size_t;
    (*mtree).buf.s = NULL as *mut libc::c_char;
    (*mtree).buf.length = 0 as libc::c_int as size_t;
    (*mtree).buf.buffer_length = 0 as libc::c_int as size_t;
    mtree_entry_register_init(mtree);
    (*a).format_data = mtree as *mut libc::c_void;
    (*a).format_free = Some(
        archive_write_mtree_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_name = b"mtree\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        archive_write_mtree_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        archive_write_mtree_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_close = Some(
        archive_write_mtree_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        archive_write_mtree_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry = Some(
        archive_write_mtree_finish_entry
            as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).archive.archive_format = ARCHIVE_FORMAT_MTREE;
    (*a).archive.archive_format_name = b"mtree\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_mtree(mut _a: *mut archive) -> libc::c_int {
    return archive_write_set_format_mtree_default(
        _a,
        b"archive_write_set_format_mtree\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_mtree_classic(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = archive_write_set_format_mtree_default(
        _a,
        b"archive_write_set_format_mtree_classic\x00" as *const u8 as *const libc::c_char,
    );
    if r == ARCHIVE_OK {
        let mut a: *mut archive_write = _a as *mut archive_write;
        let mut mtree: *mut mtree_writer = 0 as *mut mtree_writer;
        mtree = (*a).format_data as *mut mtree_writer;
        /* Set to output a mtree archive in classic format. */
        (*mtree).classic = 1 as libc::c_int;
        /* Basically, mtree classic format uses '/set' global
         * value. */
        (*mtree).output_global_set = 1 as libc::c_int
    }
    return r;
}
unsafe extern "C" fn sum_init(mut mtree: *mut mtree_writer) {
    (*mtree).compute_sum = 0 as libc::c_int;
    if (*mtree).keys & F_CKSUM != 0 {
        (*mtree).compute_sum |= F_CKSUM;
        (*mtree).crc = 0 as libc::c_int as uint32_t;
        (*mtree).crc_len = 0 as libc::c_int as uint64_t
    }
    if (*mtree).keys & F_MD5 != 0 {
        if __archive_digest.md5init.expect("non-null function pointer")(&mut (*mtree).md5ctx)
            == ARCHIVE_OK
        {
            (*mtree).compute_sum |= F_MD5
        } else {
            (*mtree).keys &= !F_MD5
        }
        /* Not supported. */
    }
    if (*mtree).keys & F_RMD160 != 0 {
        if __archive_digest
            .rmd160init
            .expect("non-null function pointer")(&mut (*mtree).rmd160ctx)
            == ARCHIVE_OK
        {
            (*mtree).compute_sum |= F_RMD160
        } else {
            (*mtree).keys &= !F_RMD160
        }
        /* Not supported. */
    }
    if (*mtree).keys & F_SHA1 != 0 {
        if __archive_digest
            .sha1init
            .expect("non-null function pointer")(&mut (*mtree).sha1ctx)
            == ARCHIVE_OK
        {
            (*mtree).compute_sum |= F_SHA1
        } else {
            (*mtree).keys &= !F_SHA1
        }
        /* Not supported. */
    }
    if (*mtree).keys & F_SHA256 != 0 {
        if __archive_digest
            .sha256init
            .expect("non-null function pointer")(&mut (*mtree).sha256ctx)
            == ARCHIVE_OK
        {
            (*mtree).compute_sum |= F_SHA256
        } else {
            (*mtree).keys &= !F_SHA256
        }
        /* Not supported. */
    }
    if (*mtree).keys & F_SHA384 != 0 {
        if __archive_digest
            .sha384init
            .expect("non-null function pointer")(&mut (*mtree).sha384ctx)
            == ARCHIVE_OK
        {
            (*mtree).compute_sum |= F_SHA384
        } else {
            (*mtree).keys &= !F_SHA384
        }
        /* Not supported. */
    }
    if (*mtree).keys & F_SHA512 != 0 {
        if __archive_digest
            .sha512init
            .expect("non-null function pointer")(&mut (*mtree).sha512ctx)
            == ARCHIVE_OK
        {
            (*mtree).compute_sum |= F_SHA512
        } else {
            (*mtree).keys &= !F_SHA512
        }
        /* Not supported. */
    };
}
unsafe extern "C" fn sum_update(
    mut mtree: *mut mtree_writer,
    mut buff: *const libc::c_void,
    mut n: size_t,
) {
    if (*mtree).compute_sum & F_CKSUM != 0 {
        /*
         * Compute a POSIX 1003.2 checksum
         */
        let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
        let mut nn: size_t = 0;
        nn = n;
        p = buff as *const libc::c_uchar;
        loop {
            let fresh1 = nn;
            nn = nn.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            (*mtree).crc = (*mtree).crc << 8 as libc::c_int
                ^ crctab[((*mtree).crc >> 24 as libc::c_int ^ *p as libc::c_uint) as usize];
            p = p.offset(1)
        }
        (*mtree).crc_len =
            ((*mtree).crc_len as libc::c_ulong).wrapping_add(n) as uint64_t as uint64_t
    }
    if (*mtree).compute_sum & F_MD5 != 0 {
        __archive_digest
            .md5update
            .expect("non-null function pointer")(&mut (*mtree).md5ctx, buff, n);
    }
    if (*mtree).compute_sum & F_RMD160 != 0 {
        __archive_digest
            .rmd160update
            .expect("non-null function pointer")(&mut (*mtree).rmd160ctx, buff, n);
    }
    if (*mtree).compute_sum & F_SHA1 != 0 {
        __archive_digest
            .sha1update
            .expect("non-null function pointer")(&mut (*mtree).sha1ctx, buff, n);
    }
    if (*mtree).compute_sum & F_SHA256 != 0 {
        __archive_digest
            .sha256update
            .expect("non-null function pointer")(&mut (*mtree).sha256ctx, buff, n);
    }
    if (*mtree).compute_sum & F_SHA384 != 0 {
        __archive_digest
            .sha384update
            .expect("non-null function pointer")(&mut (*mtree).sha384ctx, buff, n);
    }
    if (*mtree).compute_sum & F_SHA512 != 0 {
        __archive_digest
            .sha512update
            .expect("non-null function pointer")(&mut (*mtree).sha512ctx, buff, n);
    };
}
unsafe extern "C" fn sum_final(mut mtree: *mut mtree_writer, mut reg: *mut reg_info) {
    if (*mtree).compute_sum & F_CKSUM != 0 {
        let mut len: uint64_t = 0;
        /* Include the length of the file. */
        len = (*mtree).crc_len;
        while len != 0 as libc::c_int as libc::c_ulong {
            (*mtree).crc = (*mtree).crc << 8 as libc::c_int
                ^ crctab[(((*mtree).crc >> 24 as libc::c_int) as libc::c_ulong
                    ^ len & 0xff as libc::c_int as libc::c_ulong)
                    as usize];
            len >>= 8 as libc::c_int
        }
        (*reg).crc = !(*mtree).crc
    }
    if (*mtree).compute_sum & F_MD5 != 0 {
        __archive_digest
            .md5final
            .expect("non-null function pointer")(
            &mut (*mtree).md5ctx,
            (*reg).buf_md5.as_mut_ptr() as *mut libc::c_void,
        );
    }
    if (*mtree).compute_sum & F_RMD160 != 0 {
        __archive_digest
            .rmd160final
            .expect("non-null function pointer")(
            &mut (*mtree).rmd160ctx,
            (*reg).buf_rmd160.as_mut_ptr() as *mut libc::c_void,
        );
    }
    if (*mtree).compute_sum & F_SHA1 != 0 {
        __archive_digest
            .sha1final
            .expect("non-null function pointer")(
            &mut (*mtree).sha1ctx,
            (*reg).buf_sha1.as_mut_ptr() as *mut libc::c_void,
        );
    }
    if (*mtree).compute_sum & F_SHA256 != 0 {
        __archive_digest
            .sha256final
            .expect("non-null function pointer")(
            &mut (*mtree).sha256ctx,
            (*reg).buf_sha256.as_mut_ptr() as *mut libc::c_void,
        );
    }
    if (*mtree).compute_sum & F_SHA384 != 0 {
        __archive_digest
            .sha384final
            .expect("non-null function pointer")(
            &mut (*mtree).sha384ctx,
            (*reg).buf_sha384.as_mut_ptr() as *mut libc::c_void,
        );
    }
    if (*mtree).compute_sum & F_SHA512 != 0 {
        __archive_digest
            .sha512final
            .expect("non-null function pointer")(
            &mut (*mtree).sha512ctx,
            (*reg).buf_sha512.as_mut_ptr() as *mut libc::c_void,
        );
    }
    /* Save what types of sum are computed. */
    (*reg).compute_sum = (*mtree).compute_sum;
}
unsafe extern "C" fn strappend_bin(
    mut s: *mut archive_string,
    mut bin: *const libc::c_uchar,
    mut n: libc::c_int,
) {
    static mut hex: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789abcdef\x00")
    };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        archive_strappend_char(
            s,
            hex[(*bin.offset(i as isize) as libc::c_int >> 4 as libc::c_int) as usize],
        );
        archive_strappend_char(
            s,
            hex[(*bin.offset(i as isize) as libc::c_int & 0xf as libc::c_int) as usize],
        );
        i += 1
    }
}
unsafe extern "C" fn sum_write(mut str: *mut archive_string, mut reg: *mut reg_info) {
    if (*reg).compute_sum & F_CKSUM != 0 {
        archive_string_sprintf(
            str,
            b" cksum=%ju\x00" as *const u8 as *const libc::c_char,
            (*reg).crc as uintmax_t,
        );
    }
    if (*reg).compute_sum & F_MD5 != 0 {
        archive_strcat(
            str,
            b" md5digest=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        strappend_bin(
            str,
            (*reg).buf_md5.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong as libc::c_int,
        );
    }
    if (*reg).compute_sum & F_RMD160 != 0 {
        archive_strcat(
            str,
            b" rmd160digest=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        strappend_bin(
            str,
            (*reg).buf_rmd160.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong as libc::c_int,
        );
    }
    if (*reg).compute_sum & F_SHA1 != 0 {
        archive_strcat(
            str,
            b" sha1digest=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        strappend_bin(
            str,
            (*reg).buf_sha1.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong as libc::c_int,
        );
    }
    if (*reg).compute_sum & F_SHA256 != 0 {
        archive_strcat(
            str,
            b" sha256digest=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        strappend_bin(
            str,
            (*reg).buf_sha256.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong as libc::c_int,
        );
    }
    if (*reg).compute_sum & F_SHA384 != 0 {
        archive_strcat(
            str,
            b" sha384digest=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        strappend_bin(
            str,
            (*reg).buf_sha384.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 48]>() as libc::c_ulong as libc::c_int,
        );
    }
    if (*reg).compute_sum & F_SHA512 != 0 {
        archive_strcat(
            str,
            b" sha512digest=\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        strappend_bin(
            str,
            (*reg).buf_sha512.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong as libc::c_int,
        );
    };
}
unsafe extern "C" fn mtree_entry_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut e1: *const mtree_entry = n1 as *const mtree_entry;
    let mut e2: *const mtree_entry = n2 as *const mtree_entry;
    return strcmp((*e2).basename.s, (*e1).basename.s);
}
unsafe extern "C" fn mtree_entry_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut e: *const mtree_entry = n as *const mtree_entry;
    return strcmp(key as *const libc::c_char, (*e).basename.s);
}
/*
 * Generate a parent directory name and a base name from a pathname.
 */
unsafe extern "C" fn mtree_entry_setup_filenames(
    mut a: *mut archive_write,
    mut file: *mut mtree_entry,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut pathname: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    (*file).pathname.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*file).pathname,
        archive_entry_pathname(entry) as *const libc::c_void,
        (if archive_entry_pathname(entry).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(archive_entry_pathname(entry))
        }),
    );
    /* UNUSED */
    pathname = (*file).pathname.s;
    if strcmp(pathname, b".\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*file).basename.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*file).basename,
            b".\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            (if (b".\x00" as *const u8 as *const libc::c_char).is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(b".\x00" as *const u8 as *const libc::c_char)
            }),
        );
        return 0 as libc::c_int;
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
     * Remove leading '/' and '../' elements
     */
    while *p != 0 {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            p = p.offset(1);
            len = len.wrapping_sub(1)
        } else {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 {
                break;
            }
            if !(*p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32)
            {
                break;
            }
            p = p.offset(3 as libc::c_int as isize);
            len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
                as size_t
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
    /*
     * Add "./" prefix.
     * NOTE: If the pathname does not have a path separator, we have
     * to add "./" to the head of the pathname because mtree reader
     * will suppose that it is v1(a.k.a classic) mtree format and
     * change the directory unexpectedly and so it will make a wrong
     * path.
     */
    if strcmp(p, b".\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int
        && strncmp(
            p,
            b"./\x00" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        let mut as_0: archive_string = archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        };
        as_0.s = NULL as *mut libc::c_char;
        as_0.length = 0 as libc::c_int as size_t;
        as_0.buffer_length = 0 as libc::c_int as size_t;
        as_0.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut as_0,
            b"./\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            (if (b"./\x00" as *const u8 as *const libc::c_char).is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(b"./\x00" as *const u8 as *const libc::c_char)
            }),
        );
        archive_strncat(&mut as_0, p as *const libc::c_void, len);
        (*file).parentdir.length = 0 as libc::c_int as size_t;
        archive_string_concat(&mut (*file).parentdir, &mut as_0);
        archive_string_free(&mut as_0);
        p = (*file).parentdir.s;
        len = (*file).parentdir.length
    }
    /*
     * Find out the position which points the last position of
     * path separator('/').
     */
    slash = NULL as *mut libc::c_char;
    while *p as libc::c_int != '\u{0}' as i32 {
        if *p as libc::c_int == '/' as i32 {
            slash = p
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
    /* Make a basename from file->parentdir.s and slash */
    *slash = '\u{0}' as i32 as libc::c_char;
    (*file).parentdir.length =
        slash.offset_from((*file).parentdir.s) as libc::c_long as size_t;
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
    return ret;
}
unsafe extern "C" fn mtree_entry_create_virtual_dir(
    mut a: *mut archive_write,
    mut pathname: *const libc::c_char,
    mut m_entry: *mut *mut mtree_entry,
) -> libc::c_int {
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut file: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut r: libc::c_int = 0;
    entry = archive_entry_new();
    if entry.is_null() {
        *m_entry = NULL as *mut mtree_entry;
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    archive_entry_copy_pathname(entry, pathname);
    archive_entry_set_mode(
        entry,
        AE_IFDIR as mode_t | 0o755 as libc::c_int as libc::c_uint,
    );
    archive_entry_set_mtime(
        entry,
        time(NULL as *mut time_t),
        0 as libc::c_int as libc::c_long,
    );
    r = mtree_entry_new(a, entry, &mut file);
    archive_entry_free(entry);
    if r < ARCHIVE_WARN {
        *m_entry = NULL as *mut mtree_entry;
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*(*file).dir_info).virtual_0 = 1 as libc::c_int;
    *m_entry = file;
    return 0 as libc::c_int;
}
unsafe extern "C" fn mtree_entry_register_add(
    mut mtree: *mut mtree_writer,
    mut file: *mut mtree_entry,
) {
    (*file).next = NULL as *mut mtree_entry;
    *(*mtree).file_list.last = file;
    (*mtree).file_list.last = &mut (*file).next;
}
unsafe extern "C" fn mtree_entry_register_init(mut mtree: *mut mtree_writer) {
    (*mtree).file_list.first = NULL as *mut mtree_entry;
    (*mtree).file_list.last = &mut (*mtree).file_list.first;
}
unsafe extern "C" fn mtree_entry_register_free(mut mtree: *mut mtree_writer) {
    let mut file: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut file_next: *mut mtree_entry = 0 as *mut mtree_entry;
    file = (*mtree).file_list.first;
    while !file.is_null() {
        file_next = (*file).next;
        mtree_entry_free(file);
        file = file_next
    }
}
unsafe extern "C" fn mtree_entry_add_child_tail(
    mut parent: *mut mtree_entry,
    mut child: *mut mtree_entry,
) -> libc::c_int {
    (*(*child).dir_info).chnext = NULL as *mut mtree_entry;
    *(*(*parent).dir_info).children.last = child;
    (*(*parent).dir_info).children.last = &mut (*(*child).dir_info).chnext;
    return 1 as libc::c_int;
}
/*
 * Find a entry from a parent entry with the name.
 */
unsafe extern "C" fn mtree_entry_find_child(
    mut parent: *mut mtree_entry,
    mut child_name: *const libc::c_char,
) -> *mut mtree_entry {
    let mut np: *mut mtree_entry = 0 as *mut mtree_entry;
    if parent.is_null() {
        return 0 as *mut mtree_entry;
    }
    np = __archive_rb_tree_find_node(
        &mut (*(*parent).dir_info).rbtree,
        child_name as *const libc::c_void,
    ) as *mut mtree_entry;
    return np;
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
unsafe extern "C" fn mtree_entry_tree_add(
    mut a: *mut archive_write,
    mut filep: *mut *mut mtree_entry,
) -> libc::c_int {
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut mtree: *mut mtree_writer = (*a).format_data as *mut mtree_writer;
    let mut dent: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut file: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut np: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    file = *filep;
    if (*file).parentdir.length == 0 as libc::c_int as libc::c_ulong
        && (*file).basename.length == 1 as libc::c_int as libc::c_ulong
        && *(*file).basename.s.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        (*file).parent = file;
        if !(*mtree).root.is_null() {
            np = (*mtree).root
        } else {
            (*mtree).root = file;
            mtree_entry_register_add(mtree, file);
            return 0 as libc::c_int;
        }
    } else {
        if (*file).parentdir.length == 0 as libc::c_int as libc::c_ulong {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal programming error in generating canonical name for %s\x00" as *const u8
                    as *const libc::c_char,
                (*file).pathname.s,
            );
            return -(25 as libc::c_int);
        }
        p = (*file).parentdir.s;
        fn_0 = p;
        /*
         * If the path of the parent directory of `file' entry is
         * the same as the path of `cur_dirent', add `file' entry to
         * `cur_dirent'.
         */
        if (*mtree).cur_dirstr.length == (*file).parentdir.length
            && strcmp((*mtree).cur_dirstr.s, fn_0) == 0 as libc::c_int
        {
            if __archive_rb_tree_insert_node(
                &mut (*(*(*mtree).cur_dirent).dir_info).rbtree,
                file as *mut archive_rb_node,
            ) == 0
            {
                /* There is the same name in the tree. */
                np = __archive_rb_tree_find_node(
                    &mut (*(*(*mtree).cur_dirent).dir_info).rbtree,
                    (*file).basename.s as *const libc::c_void,
                ) as *mut mtree_entry
            } else {
                (*file).parent = (*mtree).cur_dirent;
                mtree_entry_register_add(mtree, file);
                return 0 as libc::c_int;
            }
        } else {
            dent = (*mtree).root;
            loop {
                l = get_path_component(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    fn_0,
                );
                if l == 0 as libc::c_int {
                    np = NULL as *mut mtree_entry;
                    break;
                } else {
                    if l < 0 as libc::c_int {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"A name buffer is too small\x00" as *const u8 as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    if l == 1 as libc::c_int
                        && name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
                        && !dent.is_null()
                        && dent == (*mtree).root
                    {
                        fn_0 = fn_0.offset(l as isize);
                        if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                            fn_0 = fn_0.offset(1)
                        }
                    } else {
                        np = mtree_entry_find_child(dent, name.as_mut_ptr());
                        if np.is_null()
                            || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int
                                == '\u{0}' as i32
                        {
                            break;
                        }
                        /* Find next sub directory. */
                        if (*np).dir_info.is_null() {
                            /* NOT Directory! */
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"`%s\' is not directory, we cannot insert `%s\' \x00" as *const u8
                                    as *const libc::c_char,
                                (*np).pathname.s,
                                (*file).pathname.s,
                            );
                            return -(25 as libc::c_int);
                        }
                        fn_0 = fn_0.offset(l as isize);
                        if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                            fn_0 = fn_0.offset(1)
                        }
                        dent = np
                    }
                }
            }
            if np.is_null() {
                /*
                 * Create virtual parent directories.
                 */
                while *fn_0.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
                    let mut vp: *mut mtree_entry = 0 as *mut mtree_entry;
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
                        (fn_0.offset_from(p) as libc::c_long + l as libc::c_long)
                            as size_t,
                    );
                    if *as_0.s.offset(as_0.length.wrapping_sub(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong)
                                          as isize) as libc::c_int ==
                           '/' as i32 {
                        *as_0.s.offset(as_0.length.wrapping_sub(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                                           as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        as_0.length = as_0.length.wrapping_sub(1)
                    }
                    r = mtree_entry_create_virtual_dir(a, as_0.s, &mut vp);
                    archive_string_free(&mut as_0);
                    if r < ARCHIVE_WARN {
                        return r;
                    }
                    if strcmp(
                        (*vp).pathname.s,
                        b".\x00" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        (*vp).parent = vp;
                        (*mtree).root = vp
                    } else {
                        __archive_rb_tree_insert_node(
                            &mut (*(*dent).dir_info).rbtree,
                            vp as *mut archive_rb_node,
                        );
                        (*vp).parent = dent
                    }
                    mtree_entry_register_add(mtree, vp);
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
                        return -(30 as libc::c_int);
                    }
                    dent = np
                }
                /* Found out the parent directory where `file' can be
                 * inserted. */
                (*mtree).cur_dirent = dent;
                (*mtree).cur_dirstr.length = 0 as libc::c_int as size_t;
                archive_string_ensure(
                    &mut (*mtree).cur_dirstr,
                    (*dent)
                        .parentdir
                        .length
                        .wrapping_add((*dent).basename.length)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong),
                );
                if (*dent)
                    .parentdir
                    .length
                    .wrapping_add((*dent).basename.length)
                    == 0 as libc::c_int as libc::c_ulong
                {
                    *(*mtree).cur_dirstr.s.offset(0 as libc::c_int as isize) =
                        0 as libc::c_int as libc::c_char
                } else {
                    if (*dent).parentdir.length > 0 as libc::c_int as libc::c_ulong {
                        (*mtree).cur_dirstr.length = 0 as libc::c_int as size_t;
                        archive_string_concat(&mut (*mtree).cur_dirstr, &mut (*dent).parentdir);
                        archive_strappend_char(
                            &mut (*mtree).cur_dirstr,
                            '/' as i32 as libc::c_char,
                        );
                    }
                    archive_string_concat(&mut (*mtree).cur_dirstr, &mut (*dent).basename);
                }
                if __archive_rb_tree_insert_node(
                    &mut (*(*dent).dir_info).rbtree,
                    file as *mut archive_rb_node,
                ) == 0
                {
                    np = __archive_rb_tree_find_node(
                        &mut (*(*dent).dir_info).rbtree,
                        (*file).basename.s as *const libc::c_void,
                    ) as *mut mtree_entry
                } else {
                    (*file).parent = dent;
                    mtree_entry_register_add(mtree, file);
                    return 0 as libc::c_int;
                }
            }
        }
    }
    /*
     * We have already has the entry the filename of which is
     * the same.
     */
    r = mtree_entry_exchange_same_entry(a, np, file);
    if r < ARCHIVE_WARN {
        return r;
    }
    if !(*np).dir_info.is_null() {
        (*(*np).dir_info).virtual_0 = 0 as libc::c_int
    }
    *filep = np;
    mtree_entry_free(file);
    return -(20 as libc::c_int);
}
unsafe extern "C" fn mtree_entry_exchange_same_entry(
    mut a: *mut archive_write,
    mut np: *mut mtree_entry,
    mut file: *mut mtree_entry,
) -> libc::c_int {
    if (*np).mode & AE_IFMT as mode_t != (*file).mode & AE_IFMT as mode_t {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Found duplicate entries `%s\' and its file type is different\x00" as *const u8
                as *const libc::c_char,
            (*np).pathname.s,
        );
        return -(25 as libc::c_int);
    }
    /* Update the existent mtree entry's attributes by the new one's. */
    (*np).symlink.length = 0 as libc::c_int as size_t;
    archive_string_concat(&mut (*np).symlink, &mut (*file).symlink);
    (*np).uname.length = 0 as libc::c_int as size_t;
    archive_string_concat(&mut (*np).uname, &mut (*file).uname);
    (*np).gname.length = 0 as libc::c_int as size_t;
    archive_string_concat(&mut (*np).gname, &mut (*file).gname);
    (*np).fflags_text.length = 0 as libc::c_int as size_t;
    archive_string_concat(&mut (*np).fflags_text, &mut (*file).fflags_text);
    (*np).nlink = (*file).nlink;
    (*np).filetype = (*file).filetype;
    (*np).mode = (*file).mode;
    (*np).size = (*file).size;
    (*np).uid = (*file).uid;
    (*np).gid = (*file).gid;
    (*np).fflags_set = (*file).fflags_set;
    (*np).fflags_clear = (*file).fflags_clear;
    (*np).mtime = (*file).mtime;
    (*np).mtime_nsec = (*file).mtime_nsec;
    (*np).rdevmajor = (*file).rdevmajor;
    (*np).rdevminor = (*file).rdevminor;
    (*np).devmajor = (*file).devmajor;
    (*np).devminor = (*file).devminor;
    (*np).ino = (*file).ino;
    return -(20 as libc::c_int);
}
