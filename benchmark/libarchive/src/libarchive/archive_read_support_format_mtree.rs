use ::libc;
extern "C" {
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    pub type archive_entry;
    /*
     * Utility to match up hardlinks.
     *
     * The 'struct archive_entry_linkresolver' is a cache of archive entries
     * for files with multiple links.  Here's how to use it:
     *   1. Create a lookup object with archive_entry_linkresolver_new()
     *   2. Tell it the archive format you're using.
     *   3. Hand each archive_entry to archive_entry_linkify().
     *      That function will return 0, 1, or 2 entries that should
     *      be written.
     *   4. Call archive_entry_linkify(resolver, NULL) until
     *      no more entries are returned.
     *   5. Call archive_entry_linkresolver_free(resolver) to free resources.
     *
     * The entries returned have their hardlink and size fields updated
     * appropriately.  If an entry is passed in that does not refer to
     * a file with multiple links, it is returned unchanged.  The intention
     * is that you should be able to simply filter all entries through
     * this machine.
     *
     * To make things more efficient, be sure that each entry has a valid
     * nlinks value.  The hardlink cache uses this to track when all links
     * have been found.  If the nlinks value is zero, it will keep every
     * name in the cache indefinitely, which can use a lot of memory.
     *
     * Note that archive_entry_size() is reset to zero if the file
     * body should not be written to the archive.  Pay attention!
     */
    pub type archive_entry_linkresolver;
    #[no_mangle]
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    #[no_mangle]
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_set_dev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_filetype(_: *mut archive_entry, _: libc::c_uint);
    /* Returns pointer to start of first invalid token, or NULL if none. */
    /* Note that all recognized tokens are processed, regardless. */
    #[no_mangle]
    fn archive_entry_copy_fflags_text(
        _: *mut archive_entry,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_copy_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_ino(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_perm(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_rdev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_copy_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_copy_uname(_: *mut archive_entry, _: *const libc::c_char);
    /*
     * There are three different strategies for marking hardlinks.
     * The descriptions below name them after the best-known
     * formats that rely on each strategy:
     *
     * "Old cpio" is the simplest, it always returns any entry unmodified.
     *    As far as I know, only cpio formats use this.  Old cpio archives
     *    store every link with the full body; the onus is on the dearchiver
     *    to detect and properly link the files as they are restored.
     * "tar" is also pretty simple; it caches a copy the first time it sees
     *    any link.  Subsequent appearances are modified to be hardlink
     *    references to the first one without any body.  Used by all tar
     *    formats, although the newest tar formats permit the "old cpio" strategy
     *    as well.  This strategy is very simple for the dearchiver,
     *    and reasonably straightforward for the archiver.
     * "new cpio" is trickier.  It stores the body only with the last
     *    occurrence.  The complication is that we might not
     *    see every link to a particular file in a single session, so
     *    there's no easy way to know when we've seen the last occurrence.
     *    The solution here is to queue one link until we see the next.
     *    At the end of the session, you can enumerate any remaining
     *    entries by calling archive_entry_linkify(NULL) and store those
     *    bodies.  If you have a file with three links l1, l2, and l3,
     *    you'll get the following behavior if you see all three links:
     *           linkify(l1) => NULL   (the resolver stores l1 internally)
     *           linkify(l2) => l1     (resolver stores l2, you write l1)
     *           linkify(l3) => l2, l3 (all links seen, you can write both).
     *    If you only see l1 and l2, you'll get this behavior:
     *           linkify(l1) => NULL
     *           linkify(l2) => l1
     *           linkify(NULL) => l2   (at end, you retrieve remaining links)
     *    As the name suggests, this strategy is used by newer cpio variants.
     *    It's noticeably more complex for the archiver, slightly more complex
     *    for the dearchiver than the tar strategy, but makes it straightforward
     *    to restore a file using any link by simply continuing to scan until
     *    you see a link that is stored with a body.  In contrast, the tar
     *    strategy requires you to rescan the archive from the beginning to
     *    correctly extract an arbitrary link.
     */
    #[no_mangle]
    fn archive_entry_linkresolver_new() -> *mut archive_entry_linkresolver;
    #[no_mangle]
    fn archive_entry_linkresolver_set_strategy(_: *mut archive_entry_linkresolver, _: libc::c_int);
    #[no_mangle]
    fn archive_entry_linkresolver_free(_: *mut archive_entry_linkresolver);
    #[no_mangle]
    fn archive_entry_linkify(
        _: *mut archive_entry_linkresolver,
        _: *mut *mut archive_entry,
        _: *mut *mut archive_entry,
    );
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_ensure_cloexec_flag(fd: libc::c_int);
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
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
    #[no_mangle]
    fn __archive_read_register_format(
        a: *mut archive_read,
        format_data: *mut libc::c_void,
        name: *const libc::c_char,
        bid: Option<unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int>,
        options: Option<
            unsafe extern "C" fn(
                _: *mut archive_read,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
        >,
        read_header_0: Option<
            unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        >,
        read_data_0: Option<
            unsafe extern "C" fn(
                _: *mut archive_read,
                _: *mut *const libc::c_void,
                _: *mut size_t,
                _: *mut int64_t,
            ) -> libc::c_int,
        >,
        read_data_skip: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        seek_data: Option<
            unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
        >,
        cleanup_0: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        format_capabilities: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        has_encrypted_entries: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_read_ahead(
        _: *mut archive_read,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
    #[no_mangle]
    fn pack_find(_: *const libc::c_char) -> Option<pack_t>;
}
pub type __int64_t = libc::c_long;
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
pub type uintptr_t = libc::c_ulong;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtree {
    pub line: archive_string,
    pub buffsize: size_t,
    pub buff: *mut libc::c_char,
    pub offset: int64_t,
    pub fd: libc::c_int,
    pub archive_format: libc::c_int,
    pub archive_format_name: *const libc::c_char,
    pub entries: *mut mtree_entry,
    pub this_entry: *mut mtree_entry,
    pub entry_rbtree: archive_rb_tree,
    pub current_dir: archive_string,
    pub contents_name: archive_string,
    pub resolver: *mut archive_entry_linkresolver,
    pub rbtree: archive_rb_tree,
    pub cur_size: int64_t,
    pub checkfs: libc::c_char,
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
pub struct mtree_entry {
    pub rbnode: archive_rb_node,
    pub next_dup: *mut mtree_entry,
    pub next: *mut mtree_entry,
    pub options: *mut mtree_option,
    pub name: *mut libc::c_char,
    pub full: libc::c_char,
    pub used: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtree_option {
    pub next: *mut mtree_option,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read {
    pub archive: archive,
    pub entry: *mut archive_entry,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub client: archive_read_client,
    pub bidders: [archive_read_filter_bidder; 16],
    pub filter: *mut archive_read_filter,
    pub bypass_filter_bidding: libc::c_int,
    pub header_position: int64_t,
    pub data_start_node: libc::c_uint,
    pub data_end_node: libc::c_uint,
    pub formats: [archive_format_descriptor; 16],
    pub format: *mut archive_format_descriptor,
    pub extract: *mut archive_read_extract,
    pub cleanup_archive_extract: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub passphrases: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub first: *mut archive_read_passphrase,
    pub last: *mut *mut archive_read_passphrase,
    pub candidate: libc::c_int,
    pub callback: Option<archive_passphrase_callback>,
    pub client_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_passphrase {
    pub passphrase: *mut libc::c_char,
    pub next: *mut archive_read_passphrase,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_extract {
    pub ad: *mut archive,
    pub extract_progress: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub extract_progress_user_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_format_descriptor {
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub bid: Option<unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int>,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_read,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub read_header:
        Option<unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int>,
    pub read_data: Option<
        unsafe extern "C" fn(
            _: *mut archive_read,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub read_data_skip: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub seek_data:
        Option<unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t>,
    pub cleanup: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub format_capabilties: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    pub has_encrypted_entries: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
}
/*
 * This structure is allocated within the archive_read core
 * and initialized by archive_read and the init() method of the
 * corresponding bidder above.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_filter {
    pub position: int64_t,
    pub bidder: *mut archive_read_filter_bidder,
    pub upstream: *mut archive_read_filter,
    pub archive: *mut archive_read,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int>,
    pub read: Option<
        unsafe extern "C" fn(_: *mut archive_read_filter, _: *mut *const libc::c_void) -> ssize_t,
    >,
    pub skip: Option<unsafe extern "C" fn(_: *mut archive_read_filter, _: int64_t) -> int64_t>,
    pub seek: Option<
        unsafe extern "C" fn(_: *mut archive_read_filter, _: int64_t, _: libc::c_int) -> int64_t,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int>,
    pub sswitch:
        Option<unsafe extern "C" fn(_: *mut archive_read_filter, _: libc::c_uint) -> libc::c_int>,
    pub read_header: Option<
        unsafe extern "C" fn(_: *mut archive_read_filter, _: *mut archive_entry) -> libc::c_int,
    >,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub buffer: *mut libc::c_char,
    pub buffer_size: size_t,
    pub next: *mut libc::c_char,
    pub avail: size_t,
    pub client_buff: *const libc::c_void,
    pub client_total: size_t,
    pub client_next: *const libc::c_char,
    pub client_avail: size_t,
    pub end_of_file: libc::c_char,
    pub closed: libc::c_char,
    pub fatal: libc::c_char,
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
 * $FreeBSD: head/lib/libarchive/archive_read_private.h 201088 2009-12-28 02:18:55Z kientzle $
 */
/*
 * How bidding works for filters:
 *   * The bid manager initializes the client-provided reader as the
 *     first filter.
 *   * It invokes the bidder for each registered filter with the
 *     current head filter.
 *   * The bidders can use archive_read_filter_ahead() to peek ahead
 *     at the incoming data to compose their bids.
 *   * The bid manager creates a new filter structure for the winning
 *     bidder and gives the winning bidder a chance to initialize it.
 *   * The new filter becomes the new top filter and we repeat the
 *     process.
 * This ends only when no bidder provides a non-zero bid.  Then
 * we perform a similar dance with the registered format handlers.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_filter_bidder {
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub bid: Option<
        unsafe extern "C" fn(
            _: *mut archive_read_filter_bidder,
            _: *mut archive_read_filter,
        ) -> libc::c_int,
    >,
    pub init: Option<unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int>,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_read_filter_bidder,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_read_filter_bidder) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_client {
    pub opener: Option<archive_open_callback>,
    pub reader: Option<archive_read_callback>,
    pub skipper: Option<archive_skip_callback>,
    pub seeker: Option<archive_seek_callback>,
    pub closer: Option<archive_close_callback>,
    pub switcher: Option<archive_switch_callback>,
    pub nodes: libc::c_uint,
    pub cursor: libc::c_uint,
    pub position: int64_t,
    pub dataset: *mut archive_read_data_node,
}
/*
 * The client looks a lot like a filter, so we just wrap it here.
 *
 * TODO: Make archive_read_filter and archive_read_client identical so
 * that users of the library can easily register their own
 * transformation filters.  This will probably break the API/ABI and
 * so should be deferred at least until libarchive 3.0.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_data_node {
    pub begin_position: int64_t,
    pub total_size: int64_t,
    pub data: *mut libc::c_void,
}
/*	$NetBSD: pack_dev.h,v 1.8 2013/06/14 16:28:20 tsutsui Exp $	*/
/*-
 * Copyright (c) 1998, 2001 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Charles M. Hannum.
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
 */
/* Originally from NetBSD's mknod(8) source. */
pub type pack_t = unsafe extern "C" fn(
    _: libc::c_int,
    _: *mut libc::c_ulong,
    _: *mut *const libc::c_char,
) -> dev_t;
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
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const INT64_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
pub const INT64_MIN: libc::c_long =
    -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long;
pub const INT32_MIN: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
pub const INT32_MAX: libc::c_int = 2147483647 as libc::c_int;
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const __S_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const __S_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const __S_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const __S_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const __S_IFIFO: libc::c_int = 0o10000 as libc::c_int;
pub const __S_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const __S_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const ENOENT: libc::c_int = 2 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const S_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const S_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const S_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const S_IFMT: libc::c_int = 0o170000 as libc::c_int;
pub const S_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const S_IFIFO: libc::c_int = 0o10000 as libc::c_int;
pub const S_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const S_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_MTREE: libc::c_int = 0x80000 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
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
pub const AE_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
/* #include <stdint.h> */
/* See archive_platform.h */
pub const O_BINARY: libc::c_int = 0 as libc::c_int;
pub const MTREE_HAS_DEVICE: libc::c_int = 0x1 as libc::c_int;
pub const MTREE_HAS_FFLAGS: libc::c_int = 0x2 as libc::c_int;
pub const MTREE_HAS_GID: libc::c_int = 0x4 as libc::c_int;
pub const MTREE_HAS_GNAME: libc::c_int = 0x8 as libc::c_int;
pub const MTREE_HAS_MTIME: libc::c_int = 0x10 as libc::c_int;
pub const MTREE_HAS_NLINK: libc::c_int = 0x20 as libc::c_int;
pub const MTREE_HAS_PERM: libc::c_int = 0x40 as libc::c_int;
pub const MTREE_HAS_SIZE: libc::c_int = 0x80 as libc::c_int;
pub const MTREE_HAS_TYPE: libc::c_int = 0x100 as libc::c_int;
pub const MTREE_HAS_UID: libc::c_int = 0x200 as libc::c_int;
pub const MTREE_HAS_UNAME: libc::c_int = 0x400 as libc::c_int;
pub const MTREE_HAS_OPTIONAL: libc::c_int = 0x800 as libc::c_int;
pub const MTREE_HAS_NOCHANGE: libc::c_int = 0x1000 as libc::c_int;
/* FreeBSD specific */
pub const MAX_LINE_LEN: libc::c_int = 1024 as libc::c_int * 1024 as libc::c_int;
/*
 * There's no standard for TIME_T_MAX/TIME_T_MIN.  So we compute them
 * here.  TODO: Move this to configure time, but be careful
 * about cross-compile environments.
 */
unsafe extern "C" fn get_time_t_max() -> int64_t {
    /* ISO C allows time_t to be a floating-point type,
    but POSIX requires an integer type.  The following
    should work on any system that follows the POSIX
    conventions. */
    if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        /* Time_t is unsigned */
        return !(0 as libc::c_int as time_t);
    } else if ::std::mem::size_of::<time_t>() as libc::c_ulong
        == ::std::mem::size_of::<int64_t>() as libc::c_ulong
    {
        return INT64_MAX;
    } else {
        return INT32_MAX as time_t;
    };
}
unsafe extern "C" fn get_time_t_min() -> int64_t {
    if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        /* Time_t is signed. */
        /* Assume it's the same as int64_t or int32_t */
        /* Time_t is unsigned */
        return 0 as libc::c_int as time_t;
    } else if ::std::mem::size_of::<time_t>() as libc::c_ulong
        == ::std::mem::size_of::<int64_t>() as libc::c_ulong
    {
        return INT64_MIN;
    } else {
        return INT32_MIN as time_t;
    };
}
unsafe extern "C" fn archive_read_format_mtree_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut mtree: *mut mtree = 0 as *mut mtree;
    mtree = (*(*a).format).data as *mut mtree;
    if strcmp(key, b"checkfs\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /* Time_t is signed. */
        /* Allows to read information missing from the mtree from the file system */
        if val.is_null()
            || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            (*mtree).checkfs = 0 as libc::c_int as libc::c_char
        } else {
            (*mtree).checkfs = 1 as libc::c_int as libc::c_char
        }
        return 0 as libc::c_int;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn free_options(mut head: *mut mtree_option) {
    let mut next: *mut mtree_option = 0 as *mut mtree_option;
    while !head.is_null() {
        next = (*head).next;
        free((*head).value as *mut libc::c_void);
        free(head as *mut libc::c_void);
        head = next
    }
}
unsafe extern "C" fn mtree_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut e1: *const mtree_entry = n1 as *const mtree_entry;
    let mut e2: *const mtree_entry = n2 as *const mtree_entry;
    return strcmp((*e1).name, (*e2).name);
}
unsafe extern "C" fn mtree_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut e: *const mtree_entry = n as *const mtree_entry;
    return strcmp((*e).name, key as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_mtree(mut _a: *mut archive) -> libc::c_int {
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    mtree_cmp_node
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    mtree_cmp_key
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut mtree: *mut mtree = 0 as *mut mtree;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_mtree\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    mtree = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<mtree>() as libc::c_ulong,
    ) as *mut mtree;
    if mtree.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate mtree data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*mtree).checkfs = 0 as libc::c_int as libc::c_char;
    (*mtree).fd = -(1 as libc::c_int);
    __archive_rb_tree_init(&mut (*mtree).rbtree, &rb_ops);
    r = __archive_read_register_format(
        a,
        mtree as *mut libc::c_void,
        b"mtree\x00" as *const u8 as *const libc::c_char,
        Some(
            mtree_bid as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_mtree_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(skip as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(cleanup as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        >(NULL as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        >(NULL as libc::intptr_t),
    );
    if r != ARCHIVE_OK {
        free(mtree as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut mtree: *mut mtree = 0 as *mut mtree;
    let mut p: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut q: *mut mtree_entry = 0 as *mut mtree_entry;
    mtree = (*(*a).format).data as *mut mtree;
    p = (*mtree).entries;
    while !p.is_null() {
        q = (*p).next;
        free((*p).name as *mut libc::c_void);
        free_options((*p).options);
        free(p as *mut libc::c_void);
        p = q
    }
    archive_string_free(&mut (*mtree).line);
    archive_string_free(&mut (*mtree).current_dir);
    archive_string_free(&mut (*mtree).contents_name);
    archive_entry_linkresolver_free((*mtree).resolver);
    free((*mtree).buff as *mut libc::c_void);
    free(mtree as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_line_size(
    mut b: *const libc::c_char,
    mut avail: ssize_t,
    mut nlsize: *mut ssize_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    len = 0 as libc::c_int as ssize_t;
    while len < avail {
        's_82: {
            match *b as libc::c_int {
                0 => {
                    /* Non-ascii character or control character. */
                    if !nlsize.is_null() {
                        *nlsize = 0 as libc::c_int as ssize_t
                    }
                    return -(1 as libc::c_int) as ssize_t;
                }
                13 => {
                    if avail - len > 1 as libc::c_int as libc::c_long
                        && *b.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
                    {
                        if !nlsize.is_null() {
                            *nlsize = 2 as libc::c_int as ssize_t
                        }
                        return len + 2 as libc::c_int as libc::c_long;
                    }
                }
                10 => {}
                _ => {
                    b = b.offset(1);
                    len += 1;
                    break 's_82;
                }
            }
            /* FALL THROUGH */
            if !nlsize.is_null() {
                *nlsize = 1 as libc::c_int as ssize_t
            }
            return len + 1 as libc::c_int as libc::c_long;
        }
    }
    if !nlsize.is_null() {
        *nlsize = 0 as libc::c_int as ssize_t
    }
    return avail;
}
/*
 *  <---------------- ravail --------------------->
 *  <-- diff ------> <---  avail ----------------->
 *                   <---- len ----------->
 * | Previous lines | line being parsed  nl extra |
 *                  ^
 *                  b
 *
 */
unsafe extern "C" fn next_line(
    mut a: *mut archive_read,
    mut b: *mut *const libc::c_char,
    mut avail: *mut ssize_t,
    mut ravail: *mut ssize_t,
    mut nl: *mut ssize_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    let mut quit: libc::c_int = 0;
    quit = 0 as libc::c_int;
    if *avail == 0 as libc::c_int as libc::c_long {
        *nl = 0 as libc::c_int as ssize_t;
        len = 0 as libc::c_int as ssize_t
    } else {
        len = get_line_size(*b, *avail, nl)
    }
    /*
     * Read bytes more while it does not reach the end of line.
     */
    while *nl == 0 as libc::c_int as libc::c_long && len == *avail && quit == 0 {
        let mut diff: ssize_t = *ravail - *avail;
        let mut nbytes_req: size_t = (*ravail + 1023 as libc::c_int as libc::c_long
            & !(1023 as libc::c_uint) as libc::c_long)
            as size_t;
        let mut tested: ssize_t = 0;
        /*
         * Place an arbitrary limit on the line length.
         * mtree is almost free-form input and without line length limits,
         * it can consume a lot of memory.
         */
        if len >= MAX_LINE_LEN as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        /* Increase reading bytes if it is not enough to at least
         * new two lines. */
        if nbytes_req < (*ravail as size_t).wrapping_add(160 as libc::c_int as libc::c_ulong) {
            nbytes_req <<= 1 as libc::c_int
        }
        *b = __archive_read_ahead(a, nbytes_req, avail) as *const libc::c_char;
        if (*b).is_null() {
            if *ravail >= *avail {
                return 0 as libc::c_int as ssize_t;
            }
            /* Reading bytes reaches the end of file. */
            *b = __archive_read_ahead(a, *avail as size_t, avail) as *const libc::c_char; /* Skip some bytes we already determinated. */
            quit = 1 as libc::c_int
        }
        *ravail = *avail;
        *b = (*b).offset(diff as isize);
        *avail -= diff;
        tested = len;
        len = get_line_size((*b).offset(len as isize), *avail - len, nl);
        if len >= 0 as libc::c_int as libc::c_long {
            len += tested
        }
    }
    return len;
}
/*
 * Compare characters with a mtree keyword.
 * Returns the length of a mtree keyword if matched.
 * Returns 0 if not matched.
 */
unsafe extern "C" fn bid_keycmp(
    mut p: *const libc::c_char,
    mut key: *const libc::c_char,
    mut len: ssize_t,
) -> libc::c_int {
    let mut match_len: libc::c_int = 0 as libc::c_int;
    while len > 0 as libc::c_int as libc::c_long
        && *p as libc::c_int != 0
        && *key as libc::c_int != 0
    {
        if *p as libc::c_int == *key as libc::c_int {
            len -= 1;
            p = p.offset(1);
            key = key.offset(1);
            match_len += 1
        } else {
            return 0 as libc::c_int;
        }
        /* Not match */
    } /* Not match */
    if *key as libc::c_int != '\u{0}' as i32 {
        return 0 as libc::c_int;
    }
    /* A following character should be specified characters */
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\t' as i32
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == '\r' as i32)
    {
        return match_len;
    }
    return 0 as libc::c_int;
    /* Not match */
}
/*
 * Test whether the characters 'p' has is mtree keyword.
 * Returns the length of a detected keyword.
 * Returns 0 if any keywords were not found.
 */
unsafe extern "C" fn bid_keyword(mut p: *const libc::c_char, mut len: ssize_t) -> libc::c_int {
    static mut keys_c: [*const libc::c_char; 4] = [
        b"content\x00" as *const u8 as *const libc::c_char,
        b"contents\x00" as *const u8 as *const libc::c_char,
        b"cksum\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_df: [*const libc::c_char; 3] = [
        b"device\x00" as *const u8 as *const libc::c_char,
        b"flags\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_g: [*const libc::c_char; 3] = [
        b"gid\x00" as *const u8 as *const libc::c_char,
        b"gname\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_il: [*const libc::c_char; 4] = [
        b"ignore\x00" as *const u8 as *const libc::c_char,
        b"inode\x00" as *const u8 as *const libc::c_char,
        b"link\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_m: [*const libc::c_char; 4] = [
        b"md5\x00" as *const u8 as *const libc::c_char,
        b"md5digest\x00" as *const u8 as *const libc::c_char,
        b"mode\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_no: [*const libc::c_char; 4] = [
        b"nlink\x00" as *const u8 as *const libc::c_char,
        b"nochange\x00" as *const u8 as *const libc::c_char,
        b"optional\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_r: [*const libc::c_char; 4] = [
        b"resdevice\x00" as *const u8 as *const libc::c_char,
        b"rmd160\x00" as *const u8 as *const libc::c_char,
        b"rmd160digest\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_s: [*const libc::c_char; 10] = [
        b"sha1\x00" as *const u8 as *const libc::c_char,
        b"sha1digest\x00" as *const u8 as *const libc::c_char,
        b"sha256\x00" as *const u8 as *const libc::c_char,
        b"sha256digest\x00" as *const u8 as *const libc::c_char,
        b"sha384\x00" as *const u8 as *const libc::c_char,
        b"sha384digest\x00" as *const u8 as *const libc::c_char,
        b"sha512\x00" as *const u8 as *const libc::c_char,
        b"sha512digest\x00" as *const u8 as *const libc::c_char,
        b"size\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_t: [*const libc::c_char; 4] = [
        b"tags\x00" as *const u8 as *const libc::c_char,
        b"time\x00" as *const u8 as *const libc::c_char,
        b"type\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    static mut keys_u: [*const libc::c_char; 3] = [
        b"uid\x00" as *const u8 as *const libc::c_char,
        b"uname\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
    ];
    let mut keys: *const *const libc::c_char = 0 as *const *const libc::c_char;
    let mut i: libc::c_int = 0;
    match *p as libc::c_int {
        99 => {
            keys = keys_c.as_ptr()
            /* Unknown key */
        }
        100 | 102 => keys = keys_df.as_ptr(),
        103 => keys = keys_g.as_ptr(),
        105 | 108 => keys = keys_il.as_ptr(),
        109 => keys = keys_m.as_ptr(),
        110 | 111 => keys = keys_no.as_ptr(),
        114 => keys = keys_r.as_ptr(),
        115 => keys = keys_s.as_ptr(),
        116 => keys = keys_t.as_ptr(),
        117 => keys = keys_u.as_ptr(),
        _ => return 0 as libc::c_int,
    }
    i = 0 as libc::c_int;
    while !(*keys.offset(i as isize)).is_null() {
        let mut l: libc::c_int = bid_keycmp(p, *keys.offset(i as isize), len);
        if l > 0 as libc::c_int {
            return l;
        }
        i += 1
    }
    return 0 as libc::c_int;
    /* Unknown key */
}
/*
 * Test whether there is a set of mtree keywords.
 * Returns the number of keyword.
 * Returns -1 if we got incorrect sequence.
 * This function expects a set of "<space characters>keyword=value".
 * When "unset" is specified, expects a set of "<space characters>keyword".
 */
unsafe extern "C" fn bid_keyword_list(
    mut p: *const libc::c_char,
    mut len: ssize_t,
    mut unset: libc::c_int,
    mut last_is_path: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut keycnt: libc::c_int = 0 as libc::c_int;
    while len > 0 as libc::c_int as libc::c_long && *p as libc::c_int != 0 {
        let mut blank: libc::c_int = 0 as libc::c_int;
        /* Test whether there are blank characters in the line. */
        while len > 0 as libc::c_int as libc::c_long
            && (*p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32)
        {
            p = p.offset(1);
            len -= 1;
            blank = 1 as libc::c_int
        }
        if *p as libc::c_int == '\n' as i32 || *p as libc::c_int == '\r' as i32 {
            break;
        }
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == '\r' as i32)
        {
            break;
        }
        if blank == 0 && last_is_path == 0 {
            /* No blank character. */
            return -(1 as libc::c_int);
        }
        if last_is_path != 0 && len == 0 as libc::c_int as libc::c_long {
            return keycnt;
        }
        if unset != 0 {
            l = bid_keycmp(p, b"all\x00" as *const u8 as *const libc::c_char, len);
            if l > 0 as libc::c_int {
                return 1 as libc::c_int;
            }
        }
        /* Test whether there is a correct key in the line. */
        l = bid_keyword(p, len); /* Unknown keyword was found. */
        if l == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        p = p.offset(l as isize);
        len -= l as libc::c_long;
        keycnt += 1;
        /* Skip value */
        if *p as libc::c_int == '=' as i32 {
            let mut value: libc::c_int = 0 as libc::c_int;
            p = p.offset(1);
            len -= 1;
            while len > 0 as libc::c_int as libc::c_long
                && *p as libc::c_int != ' ' as i32
                && *p as libc::c_int != '\t' as i32
            {
                p = p.offset(1);
                len -= 1;
                value = 1 as libc::c_int
            }
            /* A keyword should have a its value unless
             * "/unset" operation. */
            if unset == 0 && value == 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
    }
    return keycnt;
}
unsafe extern "C" fn bid_entry(
    mut p: *const libc::c_char,
    mut len: ssize_t,
    mut nl: ssize_t,
    mut last_is_path: *mut libc::c_int,
) -> libc::c_int {
    let mut f: libc::c_int = 0 as libc::c_int;
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
    let mut ll: ssize_t = 0;
    let mut pp: *const libc::c_char = p;
    let pp_end: *const libc::c_char = pp.offset(len as isize);
    *last_is_path = 0 as libc::c_int;
    /*
     * Skip the path-name which is quoted.
     */
    while pp < pp_end {
        if safe_char[*(pp as *const libc::c_uchar) as usize] == 0 {
            if *pp as libc::c_int != ' ' as i32
                && *pp as libc::c_int != '\t' as i32
                && *pp as libc::c_int != '\r' as i32
                && *pp as libc::c_int != '\n' as i32
            {
                f = 0 as libc::c_int
            }
            break;
        } else {
            f = 1 as libc::c_int;
            pp = pp.offset(1)
        }
    }
    ll = pp_end.offset_from(pp) as libc::c_long;
    /* If a path-name was not found at the first, try to check
     * a mtree format(a.k.a form D) ``NetBSD's mtree -D'' creates,
     * which places the path-name at the last. */
    if f == 0 as libc::c_int {
        let mut pb: *const libc::c_char = p.offset(len as isize).offset(-(nl as isize));
        let mut name_len: libc::c_int = 0 as libc::c_int;
        let mut slash: libc::c_int = 0;
        /* The form D accepts only a single line for an entry. */
        if pb.offset(-(2 as libc::c_int as isize)) >= p
            && *pb.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32
            && (*pb.offset(-(2 as libc::c_int) as isize) as libc::c_int == ' ' as i32
                || *pb.offset(-(2 as libc::c_int) as isize) as libc::c_int == '\t' as i32)
        {
            return -(1 as libc::c_int);
        }
        if pb.offset(-(1 as libc::c_int as isize)) >= p
            && *pb.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32
        {
            return -(1 as libc::c_int);
        }
        slash = 0 as libc::c_int;
        loop {
            pb = pb.offset(-1);
            if !(p <= pb && *pb as libc::c_int != ' ' as i32 && *pb as libc::c_int != '\t' as i32) {
                break;
            }
            if safe_char[*(pb as *const libc::c_uchar) as usize] == 0 {
                return -(1 as libc::c_int);
            }
            name_len += 1;
            /* The pathname should have a slash in this
             * format. */
            if *pb as libc::c_int == '/' as i32 {
                slash = 1 as libc::c_int
            }
        }
        if name_len == 0 as libc::c_int || slash == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        /* If '/' is placed at the first in this field, this is not
         * a valid filename. */
        if *pb.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            return -(1 as libc::c_int);
        }
        ll = len - nl - name_len as libc::c_long;
        pp = p;
        *last_is_path = 1 as libc::c_int
    }
    return bid_keyword_list(pp, ll, 0 as libc::c_int, *last_is_path);
}
pub const MAX_BID_ENTRY: libc::c_int = 3 as libc::c_int;
unsafe extern "C" fn mtree_bid(mut a: *mut archive_read, mut best_bid: libc::c_int) -> libc::c_int {
    let mut signature: *const libc::c_char = b"#mtree\x00" as *const u8 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* UNUSED */
    /* Now let's look at the actual header and see if it matches. */
    p = __archive_read_ahead(a, strlen(signature), NULL as *mut ssize_t) as *const libc::c_char;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if memcmp(
        p as *const libc::c_void,
        signature as *const libc::c_void,
        strlen(signature),
    ) == 0 as libc::c_int
    {
        return 8 as libc::c_int * strlen(signature) as libc::c_int;
    }
    /*
     * There is not a mtree signature. Let's try to detect mtree format.
     */
    return detect_form(a, NULL as *mut libc::c_int); /* The archive is generated by `NetBSD mtree -D'
                                                     	* (In this source we call it `form D') . */
}
unsafe extern "C" fn detect_form(
    mut a: *mut archive_read,
    mut is_form_d: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut avail: ssize_t = 0;
    let mut ravail: ssize_t = 0;
    let mut detected_bytes: ssize_t = 0 as libc::c_int as ssize_t;
    let mut len: ssize_t = 0;
    let mut nl: ssize_t = 0;
    let mut entry_cnt: libc::c_int = 0 as libc::c_int;
    let mut multiline: libc::c_int = 0 as libc::c_int;
    let mut form_D: libc::c_int = 0 as libc::c_int;
    if !is_form_d.is_null() {
        *is_form_d = 0 as libc::c_int
    }
    p = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut avail) as *const libc::c_char;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    ravail = avail;
    loop {
        len = next_line(a, &mut p, &mut avail, &mut ravail, &mut nl);
        /* The terminal character of the line should be
         * a new line character, '\r\n' or '\n'. */
        if len <= 0 as libc::c_int as libc::c_long || nl == 0 as libc::c_int as libc::c_long {
            break;
        }
        if multiline == 0 {
            /* Leading whitespace is never significant,
             * ignore it. */
            while len > 0 as libc::c_int as libc::c_long
                && (*p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32)
            {
                p = p.offset(1);
                avail -= 1;
                len -= 1
            }
            /* Skip comment or empty line. */
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
                || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
                || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
            {
                p = p.offset(len as isize);
                avail -= len
            } else {
                if *p.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
                    let mut last_is_path: libc::c_int = 0;
                    let mut keywords: libc::c_int = 0;
                    keywords = bid_entry(p, len, nl, &mut last_is_path);
                    if !(keywords >= 0 as libc::c_int) {
                        break;
                    }
                    detected_bytes += len;
                    if form_D == 0 as libc::c_int {
                        if last_is_path != 0 {
                            form_D = 1 as libc::c_int
                        } else if keywords > 0 as libc::c_int {
                            /* This line is not `form D'. */
                            form_D = -(1 as libc::c_int)
                        }
                    } else if form_D == 1 as libc::c_int {
                        if last_is_path == 0 && keywords > 0 as libc::c_int {
                            break;
                        }
                    }
                    if last_is_path == 0
                        && *p.offset((len - nl - 1 as libc::c_int as libc::c_long) as isize)
                            as libc::c_int
                            == '\\' as i32
                    {
                        /* This line continues. */
                        multiline = 1 as libc::c_int
                    } else {
                        /* We've got plenty of correct lines
                         * to assume that this file is a mtree
                         * format. */
                        entry_cnt += 1;
                        if entry_cnt >= MAX_BID_ENTRY {
                            break;
                        }
                    }
                } else if len > 4 as libc::c_int as libc::c_long
                    && strncmp(
                        p,
                        b"/set\x00" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if bid_keyword_list(
                        p.offset(4 as libc::c_int as isize),
                        len - 4 as libc::c_int as libc::c_long,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    ) <= 0 as libc::c_int
                    {
                        break;
                    }
                    /* This line continues. */
                    if *p.offset((len - nl - 1 as libc::c_int as libc::c_long) as isize)
                        as libc::c_int
                        == '\\' as i32
                    {
                        multiline = 2 as libc::c_int
                    }
                } else {
                    if !(len > 6 as libc::c_int as libc::c_long
                        && strncmp(
                            p,
                            b"/unset\x00" as *const u8 as *const libc::c_char,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int)
                    {
                        break;
                    }
                    if bid_keyword_list(
                        p.offset(6 as libc::c_int as isize),
                        len - 6 as libc::c_int as libc::c_long,
                        1 as libc::c_int,
                        0 as libc::c_int,
                    ) <= 0 as libc::c_int
                    {
                        break;
                    }
                    /* This line continues. */
                    if *p.offset((len - nl - 1 as libc::c_int as libc::c_long) as isize)
                        as libc::c_int
                        == '\\' as i32
                    {
                        multiline = 2 as libc::c_int
                    }
                }
                /* Test next line. */
                p = p.offset(len as isize);
                avail -= len
            }
        } else {
            /* A continuance line; the terminal
             * character of previous line was '\' character. */
            if bid_keyword_list(p, len, 0 as libc::c_int, 0 as libc::c_int) <= 0 as libc::c_int {
                break;
            }
            if multiline == 1 as libc::c_int {
                detected_bytes += len
            }
            if *p.offset((len - nl - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
                != '\\' as i32
            {
                if multiline == 1 as libc::c_int && {
                    entry_cnt += 1;
                    (entry_cnt) >= MAX_BID_ENTRY
                } {
                    break;
                }
                multiline = 0 as libc::c_int
            }
            p = p.offset(len as isize);
            avail -= len
        }
    }
    if entry_cnt >= MAX_BID_ENTRY
        || entry_cnt > 0 as libc::c_int && len == 0 as libc::c_int as libc::c_long
    {
        if !is_form_d.is_null() {
            if form_D == 1 as libc::c_int {
                *is_form_d = 1 as libc::c_int
            }
        }
        return 32 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*
 * The extended mtree format permits multiple lines specifying
 * attributes for each file.  For those entries, only the last line
 * is actually used.  Practically speaking, that means we have
 * to read the entire mtree file into memory up front.
 *
 * The parsing is done in two steps.  First, it is decided if a line
 * changes the global defaults and if it is, processed accordingly.
 * Otherwise, the options of the line are merged with the current
 * global options.
 */
unsafe extern "C" fn add_option(
    mut a: *mut archive_read,
    mut global: *mut *mut mtree_option,
    mut value: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut opt: *mut mtree_option = 0 as *mut mtree_option;
    opt = malloc(::std::mem::size_of::<mtree_option>() as libc::c_ulong) as *mut mtree_option;
    if opt.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*opt).value = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if (*opt).value.is_null() {
        free(opt as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    memcpy(
        (*opt).value as *mut libc::c_void,
        value as *const libc::c_void,
        len,
    );
    *(*opt).value.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    (*opt).next = *global;
    *global = opt;
    return 0 as libc::c_int;
}
unsafe extern "C" fn remove_option(
    mut global: *mut *mut mtree_option,
    mut value: *const libc::c_char,
    mut len: size_t,
) {
    let mut iter: *mut mtree_option = 0 as *mut mtree_option;
    let mut last: *mut mtree_option = 0 as *mut mtree_option;
    last = NULL as *mut mtree_option;
    iter = *global;
    while !iter.is_null() {
        if strncmp((*iter).value, value, len) == 0 as libc::c_int
            && (*(*iter).value.offset(len as isize) as libc::c_int == '\u{0}' as i32
                || *(*iter).value.offset(len as isize) as libc::c_int == '=' as i32)
        {
            break;
        }
        last = iter;
        iter = (*iter).next
    }
    if iter.is_null() {
        return;
    }
    if last.is_null() {
        *global = (*iter).next
    } else {
        (*last).next = (*iter).next
    }
    free((*iter).value as *mut libc::c_void);
    free(iter as *mut libc::c_void);
}
unsafe extern "C" fn process_global_set(
    mut a: *mut archive_read,
    mut global: *mut *mut mtree_option,
    mut line: *const libc::c_char,
) -> libc::c_int {
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut eq: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut r: libc::c_int = 0;
    line = line.offset(4 as libc::c_int as isize);
    loop {
        next =
            line.offset(strspn(line, b" \t\r\n\x00" as *const u8 as *const libc::c_char) as isize);
        if *next as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        line = next;
        next =
            line.offset(strcspn(line, b" \t\r\n\x00" as *const u8 as *const libc::c_char) as isize);
        eq = strchr(line, '=' as i32);
        if eq > next {
            len = next.offset_from(line) as libc::c_long as size_t
        } else {
            len = eq.offset_from(line) as libc::c_long as size_t
        }
        remove_option(global, line, len);
        r = add_option(
            a,
            global,
            line,
            next.offset_from(line) as libc::c_long as size_t,
        );
        if r != ARCHIVE_OK {
            return r;
        }
        line = next
    }
}
unsafe extern "C" fn process_global_unset(
    mut a: *mut archive_read,
    mut global: *mut *mut mtree_option,
    mut line: *const libc::c_char,
) -> libc::c_int {
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    line = line.offset(6 as libc::c_int as isize);
    if !strchr(line, '=' as i32).is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"/unset shall not contain `=\'\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    loop {
        next =
            line.offset(strspn(line, b" \t\r\n\x00" as *const u8 as *const libc::c_char) as isize);
        if *next as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        line = next;
        len = strcspn(line, b" \t\r\n\x00" as *const u8 as *const libc::c_char);
        if len == 3 as libc::c_int as libc::c_ulong
            && strncmp(
                line,
                b"all\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            free_options(*global);
            *global = NULL as *mut mtree_option
        } else {
            remove_option(global, line, len);
        }
        line = line.offset(len as isize)
    }
}
unsafe extern "C" fn process_add_entry(
    mut a: *mut archive_read,
    mut mtree: *mut mtree,
    mut global: *mut *mut mtree_option,
    mut line: *const libc::c_char,
    mut line_len: ssize_t,
    mut last_entry: *mut *mut mtree_entry,
    mut is_form_d: libc::c_int,
) -> libc::c_int {
    let mut entry: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut iter: *mut mtree_option = 0 as *mut mtree_option;
    let mut next: *const libc::c_char = 0 as *const libc::c_char;
    let mut eq: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut name_len: size_t = 0;
    let mut len: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    entry = malloc(::std::mem::size_of::<mtree_entry>() as libc::c_ulong) as *mut mtree_entry;
    if entry.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*entry).next = NULL as *mut mtree_entry;
    (*entry).options = NULL as *mut mtree_option;
    (*entry).name = NULL as *mut libc::c_char;
    (*entry).used = 0 as libc::c_int as libc::c_char;
    (*entry).full = 0 as libc::c_int as libc::c_char;
    /* Add this entry to list. */
    if (*last_entry).is_null() {
        (*mtree).entries = entry
    } else {
        (**last_entry).next = entry
    }
    *last_entry = entry;
    if is_form_d != 0 {
        /* Filename is last item on line. */
        /* Adjust line_len to trim trailing whitespace */
        while line_len > 0 as libc::c_int as libc::c_long {
            let mut last_character: libc::c_char =
                *line.offset((line_len - 1 as libc::c_int as libc::c_long) as isize);
            if !(last_character as libc::c_int == '\r' as i32
                || last_character as libc::c_int == '\n' as i32
                || last_character as libc::c_int == '\t' as i32
                || last_character as libc::c_int == ' ' as i32)
            {
                break;
            }
            line_len -= 1
        }
        /* Name starts after the last whitespace separator */
        name = line;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < line_len {
            if *line.offset(i as isize) as libc::c_int == '\r' as i32
                || *line.offset(i as isize) as libc::c_int == '\n' as i32
                || *line.offset(i as isize) as libc::c_int == '\t' as i32
                || *line.offset(i as isize) as libc::c_int == ' ' as i32
            {
                name = line.offset(i as isize).offset(1 as libc::c_int as isize)
            }
            i += 1
        }
        name_len =
            line.offset(line_len as isize).offset_from(name) as libc::c_long as size_t;
        end = name
    } else {
        /* Filename is first item on line */
        name_len = strcspn(line, b" \t\r\n\x00" as *const u8 as *const libc::c_char);
        name = line;
        line = line.offset(name_len as isize);
        end = line.offset(line_len as isize)
    }
    /* name/name_len is the name within the line. */
    /* line..end brackets the entire line except the name */
    (*entry).name =
        malloc(name_len.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if (*entry).name.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    memcpy(
        (*entry).name as *mut libc::c_void,
        name as *const libc::c_void,
        name_len,
    );
    *(*entry).name.offset(name_len as isize) = '\u{0}' as i32 as libc::c_char;
    parse_escapes((*entry).name, entry);
    (*entry).next_dup = NULL as *mut mtree_entry;
    if (*entry).full != 0 {
        if __archive_rb_tree_insert_node(&mut (*mtree).rbtree, &mut (*entry).rbnode) == 0 {
            let mut alt: *mut mtree_entry = 0 as *mut mtree_entry;
            alt = __archive_rb_tree_find_node(
                &mut (*mtree).rbtree,
                (*entry).name as *const libc::c_void,
            ) as *mut mtree_entry;
            while !(*alt).next_dup.is_null() {
                alt = (*alt).next_dup
            }
            (*alt).next_dup = entry
        }
    }
    iter = *global;
    while !iter.is_null() {
        r = add_option(
            a,
            &mut (*entry).options,
            (*iter).value,
            strlen((*iter).value),
        );
        if r != ARCHIVE_OK {
            return r;
        }
        iter = (*iter).next
    }
    loop {
        next =
            line.offset(strspn(line, b" \t\r\n\x00" as *const u8 as *const libc::c_char) as isize);
        if *next as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        if next >= end {
            return 0 as libc::c_int;
        }
        line = next;
        next =
            line.offset(strcspn(line, b" \t\r\n\x00" as *const u8 as *const libc::c_char) as isize);
        eq = strchr(line, '=' as i32);
        if eq.is_null() || eq > next {
            len = next.offset_from(line) as libc::c_long as size_t
        } else {
            len = eq.offset_from(line) as libc::c_long as size_t
        }
        remove_option(&mut (*entry).options, line, len);
        r = add_option(
            a,
            &mut (*entry).options,
            line,
            next.offset_from(line) as libc::c_long as size_t,
        );
        if r != ARCHIVE_OK {
            return r;
        }
        line = next
    }
}
unsafe extern "C" fn read_mtree(mut a: *mut archive_read, mut mtree: *mut mtree) -> libc::c_int {
    let mut len: ssize_t = 0;
    let mut counter: uintmax_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut global: *mut mtree_option = 0 as *mut mtree_option;
    let mut last_entry: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut r: libc::c_int = 0;
    let mut is_form_d: libc::c_int = 0;
    (*mtree).archive_format = ARCHIVE_FORMAT_MTREE;
    (*mtree).archive_format_name = b"mtree\x00" as *const u8 as *const libc::c_char;
    global = NULL as *mut mtree_option;
    last_entry = NULL as *mut mtree_entry;
    detect_form(a, &mut is_form_d);
    counter = 1 as libc::c_int as uintmax_t;
    loop {
        r = ARCHIVE_OK;
        len = readline(a, mtree, &mut p, 65536 as libc::c_int as ssize_t);
        if len == 0 as libc::c_int as libc::c_long {
            (*mtree).this_entry = (*mtree).entries;
            free_options(global);
            return 0 as libc::c_int;
        }
        if len < 0 as libc::c_int as libc::c_long {
            free_options(global);
            return len as libc::c_int;
        }
        /* Leading whitespace is never significant, ignore it. */
        while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
            p = p.offset(1);
            len -= 1
        }
        /* Skip content lines and blank lines. */
        if !(*p as libc::c_int == '#' as i32) {
            if !(*p as libc::c_int == '\r' as i32
                || *p as libc::c_int == '\n' as i32
                || *p as libc::c_int == '\u{0}' as i32)
            {
                /* Non-printable characters are not allowed */
                s = p;
                while s < p.offset(len as isize).offset(-(1 as libc::c_int as isize)) {
                    if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                        == 0
                    {
                        r = ARCHIVE_FATAL;
                        break;
                    } else {
                        s = s.offset(1)
                    }
                }
                if r != ARCHIVE_OK {
                    break;
                }
                if *p as libc::c_int != '/' as i32 {
                    r = process_add_entry(a, mtree, &mut global, p, len, &mut last_entry, is_form_d)
                } else if len > 4 as libc::c_int as libc::c_long
                    && strncmp(
                        p,
                        b"/set\x00" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    if *p.offset(4 as libc::c_int as isize) as libc::c_int != ' ' as i32
                        && *p.offset(4 as libc::c_int as isize) as libc::c_int != '\t' as i32
                    {
                        break;
                    }
                    r = process_global_set(a, &mut global, p)
                } else {
                    if !(len > 6 as libc::c_int as libc::c_long
                        && strncmp(
                            p,
                            b"/unset\x00" as *const u8 as *const libc::c_char,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int)
                    {
                        break;
                    }
                    if *p.offset(6 as libc::c_int as isize) as libc::c_int != ' ' as i32
                        && *p.offset(6 as libc::c_int as isize) as libc::c_int != '\t' as i32
                    {
                        break;
                    }
                    r = process_global_unset(a, &mut global, p)
                }
                if r != ARCHIVE_OK {
                    free_options(global);
                    return r;
                }
            }
        }
        counter = counter.wrapping_add(1)
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Can\'t parse line %ju\x00" as *const u8 as *const libc::c_char,
        counter,
    );
    free_options(global);
    return -(30 as libc::c_int);
}
/*
 * Read in the entire mtree file into memory on the first request.
 * Then use the next unused file to satisfy each header request.
 */
unsafe extern "C" fn read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut mtree: *mut mtree = 0 as *mut mtree;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut use_next: libc::c_int = 0;
    mtree = (*(*a).format).data as *mut mtree;
    if (*mtree).fd >= 0 as libc::c_int {
        close((*mtree).fd);
        (*mtree).fd = -(1 as libc::c_int)
    }
    if (*mtree).entries.is_null() {
        (*mtree).resolver = archive_entry_linkresolver_new();
        if (*mtree).resolver.is_null() {
            return ARCHIVE_FATAL;
        }
        archive_entry_linkresolver_set_strategy((*mtree).resolver, ARCHIVE_FORMAT_MTREE);
        r = read_mtree(a, mtree);
        if r != ARCHIVE_OK {
            return r;
        }
    }
    (*a).archive.archive_format = (*mtree).archive_format;
    (*a).archive.archive_format_name = (*mtree).archive_format_name;
    loop {
        if (*mtree).this_entry.is_null() {
            return 1 as libc::c_int;
        }
        if strcmp(
            (*(*mtree).this_entry).name,
            b"..\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*(*mtree).this_entry).used = 1 as libc::c_int as libc::c_char;
            if (*mtree).current_dir.length > 0 as libc::c_int as libc::c_ulong {
                /* Roll back current path. */
                p = (*mtree)
                    .current_dir
                    .s
                    .offset((*mtree).current_dir.length as isize)
                    .offset(-(1 as libc::c_int as isize));
                while p >= (*mtree).current_dir.s && *p as libc::c_int != '/' as i32 {
                    p = p.offset(-1)
                }
                if p >= (*mtree).current_dir.s {
                    p = p.offset(-1)
                }
                (*mtree).current_dir.length =
                    (p.offset_from((*mtree).current_dir.s) as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as size_t
            }
        }
        if (*(*mtree).this_entry).used == 0 {
            use_next = 0 as libc::c_int;
            r = parse_file(a, entry, mtree, (*mtree).this_entry, &mut use_next);
            if use_next == 0 as libc::c_int {
                return r;
            }
        }
        (*mtree).this_entry = (*(*mtree).this_entry).next
    }
}
/*
 * A single file can have multiple lines contribute specifications.
 * Parse as many lines as necessary, then pull additional information
 * from a backing file on disk as necessary.
 */
unsafe extern "C" fn parse_file(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut mtree: *mut mtree,
    mut mentry: *mut mtree_entry,
    mut use_next: *mut libc::c_int,
) -> libc::c_int {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut st_storage: stat = stat {
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
    let mut st: *mut stat = 0 as *mut stat;
    let mut mp: *mut mtree_entry = 0 as *mut mtree_entry;
    let mut sparse_entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r1: libc::c_int = 0;
    let mut parsed_kws: libc::c_int = 0;
    (*mentry).used = 1 as libc::c_int as libc::c_char;
    /* Initialize reasonable defaults. */
    archive_entry_set_filetype(entry, AE_IFREG as mode_t);
    archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
    (*mtree).contents_name.length = 0 as libc::c_int as size_t;
    /* Parse options from this line. */
    parsed_kws = 0 as libc::c_int;
    r = parse_line(a, entry, mtree, mentry, &mut parsed_kws);
    if (*mentry).full != 0 {
        archive_entry_copy_pathname(entry, (*mentry).name);
        /*
         * "Full" entries are allowed to have multiple lines
         * and those lines aren't required to be adjacent.  We
         * don't support multiple lines for "relative" entries
         * nor do we make any attempt to merge data from
         * separate "relative" and "full" entries.  (Merging
         * "relative" and "full" entries would require dealing
         * with pathname canonicalization, which is a very
         * tricky subject.)
         */
        mp = __archive_rb_tree_find_node(
            &mut (*mtree).rbtree,
            (*mentry).name as *const libc::c_void,
        ) as *mut mtree_entry;
        while !mp.is_null() {
            if (*mp).full as libc::c_int != 0 && (*mp).used == 0 {
                /* Later lines override earlier ones. */
                (*mp).used = 1 as libc::c_int as libc::c_char;
                r1 = parse_line(a, entry, mtree, mp, &mut parsed_kws);
                if r1 < r {
                    r = r1
                }
            }
            mp = (*mp).next_dup
        }
    } else {
        /*
         * Relative entries require us to construct
         * the full path and possibly update the
         * current directory.
         */
        let mut n: size_t = (*mtree).current_dir.length;
        if n > 0 as libc::c_int as libc::c_ulong {
            archive_strcat(
                &mut (*mtree).current_dir,
                b"/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
        }
        archive_strcat(
            &mut (*mtree).current_dir,
            (*mentry).name as *const libc::c_void,
        );
        archive_entry_copy_pathname(entry, (*mtree).current_dir.s);
        if archive_entry_filetype(entry) != AE_IFDIR as mode_t {
            (*mtree).current_dir.length = n
        }
    }
    if (*mtree).checkfs != 0 {
        /*
         * Try to open and stat the file to get the real size
         * and other file info.  It would be nice to avoid
         * this here so that getting a listing of an mtree
         * wouldn't require opening every referenced contents
         * file.  But then we wouldn't know the actual
         * contents size, so I don't see a really viable way
         * around this.  (Also, we may want to someday pull
         * other unspecified info from the contents file on
         * disk.)
         */
        (*mtree).fd = -(1 as libc::c_int);
        if (*mtree).contents_name.length > 0 as libc::c_int as libc::c_ulong {
            path = (*mtree).contents_name.s
        } else {
            path = archive_entry_pathname(entry)
        }
        if archive_entry_filetype(entry) == AE_IFREG as mode_t
            || archive_entry_filetype(entry) == AE_IFDIR as mode_t
        {
            (*mtree).fd = open(path, O_RDONLY | O_BINARY | O_CLOEXEC);
            __archive_ensure_cloexec_flag((*mtree).fd);
            if (*mtree).fd == -(1 as libc::c_int)
                && (errno != ENOENT
                    || (*mtree).contents_name.length > 0 as libc::c_int as libc::c_ulong)
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Can\'t open %s\x00" as *const u8 as *const libc::c_char,
                    path,
                );
                r = ARCHIVE_WARN
            }
        }
        st = &mut st_storage;
        if (*mtree).fd >= 0 as libc::c_int {
            if fstat((*mtree).fd, st) == -(1 as libc::c_int) {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"Could not fstat %s\x00" as *const u8 as *const libc::c_char,
                    path,
                );
                r = ARCHIVE_WARN;
                /* If we can't stat it, don't keep it open. */
                close((*mtree).fd);
                (*mtree).fd = -(1 as libc::c_int);
                st = NULL as *mut stat
            }
        } else if lstat(path, st) == -(1 as libc::c_int) {
            st = NULL as *mut stat
        }
        /*
         * Check for a mismatch between the type in the specification
         * and the type of the contents object on disk.
         */
        if !st.is_null() {
            if (*st).st_mode & S_IFMT as libc::c_uint == S_IFREG as libc::c_uint
                && archive_entry_filetype(entry) == AE_IFREG as mode_t
                || (*st).st_mode & S_IFMT as libc::c_uint == S_IFLNK as libc::c_uint
                    && archive_entry_filetype(entry) == AE_IFLNK as mode_t
                || (*st).st_mode & S_IFSOCK as libc::c_uint == S_IFSOCK as libc::c_uint
                    && archive_entry_filetype(entry) == AE_IFSOCK as mode_t
                || (*st).st_mode & S_IFMT as libc::c_uint == S_IFCHR as libc::c_uint
                    && archive_entry_filetype(entry) == AE_IFCHR as mode_t
                || (*st).st_mode & S_IFMT as libc::c_uint == S_IFBLK as libc::c_uint
                    && archive_entry_filetype(entry) == AE_IFBLK as mode_t
                || (*st).st_mode & S_IFMT as libc::c_uint == S_IFDIR as libc::c_uint
                    && archive_entry_filetype(entry) == AE_IFDIR as mode_t
                || (*st).st_mode & S_IFMT as libc::c_uint == S_IFIFO as libc::c_uint
                    && archive_entry_filetype(entry) == AE_IFIFO as mode_t
            {
            } else {
                /* Types don't match; bail out gracefully. */
                if (*mtree).fd >= 0 as libc::c_int {
                    close((*mtree).fd);
                }
                (*mtree).fd = -(1 as libc::c_int);
                if parsed_kws & MTREE_HAS_OPTIONAL != 0 {
                    /* It's not an error for an optional
                     * entry to not match disk. */
                    *use_next = 1 as libc::c_int
                } else if r == ARCHIVE_OK {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"mtree specification has different type for %s\x00" as *const u8
                            as *const libc::c_char,
                        archive_entry_pathname(entry),
                    );
                    r = ARCHIVE_WARN
                }
                return r;
            }
        }
        /*
         * If there is a contents file on disk, pick some of the
         * metadata from that file.  For most of these, we only
         * set it from the contents if it wasn't already parsed
         * from the specification.
         */
        if !st.is_null() {
            if (parsed_kws & MTREE_HAS_DEVICE == 0 as libc::c_int
                || parsed_kws & MTREE_HAS_NOCHANGE != 0 as libc::c_int)
                && (archive_entry_filetype(entry) == AE_IFCHR as mode_t
                    || archive_entry_filetype(entry) == AE_IFBLK as mode_t)
            {
                archive_entry_set_rdev(entry, (*st).st_rdev);
            }
            if parsed_kws & (MTREE_HAS_GID | MTREE_HAS_GNAME) == 0 as libc::c_int
                || parsed_kws & MTREE_HAS_NOCHANGE != 0 as libc::c_int
            {
                archive_entry_set_gid(entry, (*st).st_gid as la_int64_t);
            }
            if parsed_kws & (MTREE_HAS_UID | MTREE_HAS_UNAME) == 0 as libc::c_int
                || parsed_kws & MTREE_HAS_NOCHANGE != 0 as libc::c_int
            {
                archive_entry_set_uid(entry, (*st).st_uid as la_int64_t);
            }
            if parsed_kws & MTREE_HAS_MTIME == 0 as libc::c_int
                || parsed_kws & MTREE_HAS_NOCHANGE != 0 as libc::c_int
            {
                archive_entry_set_mtime(entry, (*st).st_mtim.tv_sec, (*st).st_mtim.tv_nsec);
            }
            if parsed_kws & MTREE_HAS_NLINK == 0 as libc::c_int
                || parsed_kws & MTREE_HAS_NOCHANGE != 0 as libc::c_int
            {
                archive_entry_set_nlink(entry, (*st).st_nlink as libc::c_uint);
            }
            if parsed_kws & MTREE_HAS_PERM == 0 as libc::c_int
                || parsed_kws & MTREE_HAS_NOCHANGE != 0 as libc::c_int
            {
                archive_entry_set_perm(entry, (*st).st_mode);
            }
            if parsed_kws & MTREE_HAS_SIZE == 0 as libc::c_int
                || parsed_kws & MTREE_HAS_NOCHANGE != 0 as libc::c_int
            {
                archive_entry_set_size(entry, (*st).st_size);
            }
            archive_entry_set_ino(entry, (*st).st_ino as la_int64_t);
            archive_entry_set_dev(entry, (*st).st_dev);
            archive_entry_linkify((*mtree).resolver, &mut entry, &mut sparse_entry);
        } else if parsed_kws & MTREE_HAS_OPTIONAL != 0 {
            /*
             * Couldn't open the entry, stat it or the on-disk type
             * didn't match.  If this entry is optional, just
             * ignore it and read the next header entry.
             */
            *use_next = 1 as libc::c_int;
            return ARCHIVE_OK;
        }
    }
    (*mtree).cur_size = archive_entry_size(entry);
    (*mtree).offset = 0 as libc::c_int as int64_t;
    return r;
}
/*
 * Each line contains a sequence of keywords.
 */
unsafe extern "C" fn parse_line(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut mtree: *mut mtree,
    mut mp: *mut mtree_entry,
    mut parsed_kws: *mut libc::c_int,
) -> libc::c_int {
    let mut iter: *mut mtree_option = 0 as *mut mtree_option;
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r1: libc::c_int = 0;
    iter = (*mp).options;
    while !iter.is_null() {
        r1 = parse_keyword(a, mtree, entry, iter, parsed_kws);
        if r1 < r {
            r = r1
        }
        iter = (*iter).next
    }
    if r == ARCHIVE_OK && *parsed_kws & MTREE_HAS_TYPE == 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Missing type keyword in mtree specification\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    return r;
}
/*
 * Device entries have one of the following forms:
 *  - raw dev_t
 *  - format,major,minor[,subdevice]
 * When parsing succeeded, `pdev' will contain the appropriate dev_t value.
 */
/* strsep() is not in C90, but strcspn() is. */
/* Taken from http://unixpapa.com/incnote/string.html */
unsafe extern "C" fn la_strsep(
    mut sp: *mut *mut libc::c_char,
    mut sep: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if sp.is_null() || (*sp).is_null() || **sp as libc::c_int == '\u{0}' as i32 {
        return 0 as *mut libc::c_char;
    }
    s = *sp;
    p = s.offset(strcspn(s, sep) as isize);
    if *p as libc::c_int != '\u{0}' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char
    }
    *sp = p;
    return s;
}
unsafe extern "C" fn parse_device(
    mut pdev: *mut dev_t,
    mut a: *mut archive,
    mut val: *mut libc::c_char,
) -> libc::c_int {
    let mut numbers: [libc::c_ulong; 3] = [0; 3];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dev: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut pack: Option<pack_t> = None;
    let mut result: dev_t = 0;
    let mut error: *const libc::c_char = NULL as *const libc::c_char;
    memset(
        pdev as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<dev_t>() as libc::c_ulong,
    );
    dev = strchr(val, ',' as i32);
    if !dev.is_null() {
        /*
         * Device's major/minor are given in a specified format.
         * Decode and pack it accordingly.
         */
        let fresh1 = dev;
        dev = dev.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char;
        pack = pack_find(val);
        if pack.is_none() {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unknown format `%s\'\x00" as *const u8 as *const libc::c_char,
                val,
            );
            return ARCHIVE_WARN;
        }
        argc = 0 as libc::c_int;
        loop {
            p = la_strsep(&mut dev, b",\x00" as *const u8 as *const libc::c_char);
            if p.is_null() {
                break;
            }
            if *p as libc::c_int == '\u{0}' as i32 {
                archive_set_error(
                    a,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Missing number\x00" as *const u8 as *const libc::c_char,
                );
                return ARCHIVE_WARN;
            }
            if argc >= MAX_PACK_ARGS {
                archive_set_error(
                    a,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Too many arguments\x00" as *const u8 as *const libc::c_char,
                );
                return ARCHIVE_WARN;
            }
            let fresh2 = argc;
            argc = argc + 1;
            numbers[fresh2 as usize] = mtree_atol(&mut p, 0 as libc::c_int) as libc::c_ulong
        }
        if argc < 2 as libc::c_int {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Not enough arguments\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_WARN;
        }
        result = Some(pack.expect("non-null function pointer")).expect("non-null function pointer")(
            argc,
            numbers.as_mut_ptr(),
            &mut error,
        );
        if !error.is_null() {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"%s\x00" as *const u8 as *const libc::c_char,
                error,
            );
            return ARCHIVE_WARN;
        }
    } else {
        /* file system raw value. */
        result = mtree_atol(&mut val, 0 as libc::c_int) as dev_t
    }
    *pdev = result;
    return ARCHIVE_OK;
}
pub const MAX_PACK_ARGS: libc::c_int = 3 as libc::c_int;
/*
 * Parse a single keyword and its value.
 */
unsafe extern "C" fn parse_keyword(
    mut a: *mut archive_read,
    mut mtree: *mut mtree,
    mut entry: *mut archive_entry,
    mut opt: *mut mtree_option,
    mut parsed_kws: *mut libc::c_int,
) -> libc::c_int {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    key = (*opt).value;
    if *key as libc::c_int == '\u{0}' as i32 {
        return 0 as libc::c_int;
    }
    if strcmp(key, b"nochange\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        *parsed_kws |= MTREE_HAS_NOCHANGE;
        return 0 as libc::c_int;
    }
    if strcmp(key, b"optional\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        *parsed_kws |= MTREE_HAS_OPTIONAL;
        return 0 as libc::c_int;
    }
    if strcmp(key, b"ignore\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /*
         * The mtree processing is not recursive, so
         * recursion will only happen for explicitly listed
         * entries.
         */
        return 0 as libc::c_int;
    }
    val = strchr(key, '=' as i32);
    if val.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Malformed attribute \"%s\" (%d)\x00" as *const u8 as *const libc::c_char,
            key,
            *key.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        return -(20 as libc::c_int);
    }
    *val = '\u{0}' as i32 as libc::c_char;
    val = val.offset(1);
    let mut current_block_102: u64;
    match *key.offset(0 as libc::c_int as isize) as libc::c_int {
        99 => {
            if strcmp(key, b"content\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"contents\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                parse_escapes(val, NULL as *mut mtree_entry);
                (*mtree).contents_name.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*mtree).contents_name,
                    val as *const libc::c_void,
                    (if val.is_null() {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(val)
                    }),
                );
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"cksum\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 16162470160332600520;
            }
        }
        100 => {
            current_block_102 = 16162470160332600520;
        }
        102 => {
            current_block_102 = 9943878368997775741;
        }
        103 => {
            current_block_102 = 711389903700157801;
        }
        105 => {
            current_block_102 = 7425727042248476624;
        }
        108 => {
            current_block_102 = 9736422020172913141;
        }
        109 => {
            current_block_102 = 5708585036340422782;
        }
        110 => {
            current_block_102 = 6722169380287640260;
        }
        114 => {
            current_block_102 = 3042168899176905925;
        }
        115 => {
            current_block_102 = 17892600293413678740;
        }
        116 => {
            current_block_102 = 6752956218007646265;
        }
        117 => {
            current_block_102 = 10981874684149351084;
        }
        _ => {
            current_block_102 = 5750090253865036684;
        }
    }
    match current_block_102 {
        16162470160332600520 => {
            if strcmp(key, b"device\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                /* stat(2) st_rdev field, e.g. the major/minor IDs
                 * of a char/block special file */
                let mut r: libc::c_int = 0;
                let mut dev: dev_t = 0;
                *parsed_kws |= MTREE_HAS_DEVICE;
                r = parse_device(&mut dev, &mut (*a).archive, val);
                if r == ARCHIVE_OK {
                    archive_entry_set_rdev(entry, dev);
                }
                return r;
            }
            current_block_102 = 9943878368997775741;
        }
        _ => {}
    }
    match current_block_102 {
        9943878368997775741 => {
            if strcmp(key, b"flags\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                *parsed_kws |= MTREE_HAS_FFLAGS;
                archive_entry_copy_fflags_text(entry, val);
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 711389903700157801;
            }
        }
        _ => {}
    }
    match current_block_102 {
        711389903700157801 => {
            if strcmp(key, b"gid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                *parsed_kws |= MTREE_HAS_GID;
                archive_entry_set_gid(entry, mtree_atol(&mut val, 10 as libc::c_int));
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"gname\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                *parsed_kws |= MTREE_HAS_GNAME;
                archive_entry_copy_gname(entry, val);
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 7425727042248476624;
            }
        }
        _ => {}
    }
    match current_block_102 {
        7425727042248476624 => {
            if strcmp(key, b"inode\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                archive_entry_set_ino(entry, mtree_atol(&mut val, 10 as libc::c_int));
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 9736422020172913141;
            }
        }
        _ => {}
    }
    match current_block_102 {
        9736422020172913141 => {
            if strcmp(key, b"link\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                archive_entry_copy_symlink(entry, val);
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 5708585036340422782;
            }
        }
        _ => {}
    }
    match current_block_102 {
        5708585036340422782 => {
            if strcmp(key, b"md5\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"md5digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"mode\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if *val.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *val.offset(0 as libc::c_int as isize) as libc::c_int <= '7' as i32
                {
                    *parsed_kws |= MTREE_HAS_PERM;
                    archive_entry_set_perm(entry, mtree_atol(&mut val, 8 as libc::c_int) as mode_t);
                } else {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Symbolic or non-octal mode \"%s\" unsupported\x00" as *const u8
                            as *const libc::c_char,
                        val,
                    );
                    return ARCHIVE_WARN;
                }
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 6722169380287640260;
            }
        }
        _ => {}
    }
    match current_block_102 {
        6722169380287640260 => {
            if strcmp(key, b"nlink\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                *parsed_kws |= MTREE_HAS_NLINK;
                archive_entry_set_nlink(
                    entry,
                    mtree_atol(&mut val, 10 as libc::c_int) as libc::c_uint,
                );
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 3042168899176905925;
            }
        }
        _ => {}
    }
    match current_block_102 {
        3042168899176905925 => {
            if strcmp(key, b"resdevice\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                /* stat(2) st_dev field, e.g. the device ID where the
                 * inode resides */
                let mut r_0: libc::c_int = 0;
                let mut dev_0: dev_t = 0;
                r_0 = parse_device(&mut dev_0, &mut (*a).archive, val);
                if r_0 == ARCHIVE_OK {
                    archive_entry_set_dev(entry, dev_0);
                }
                return r_0;
            }
            if strcmp(key, b"rmd160\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"rmd160digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 17892600293413678740;
            }
        }
        _ => {}
    }
    match current_block_102 {
        17892600293413678740 => {
            if strcmp(key, b"sha1\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
                || strcmp(key, b"sha1digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"sha256\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(key, b"sha256digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"sha384\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(key, b"sha384digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"sha512\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(key, b"sha512digest\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"size\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                archive_entry_set_size(entry, mtree_atol(&mut val, 10 as libc::c_int));
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 6752956218007646265;
            }
        }
        _ => {}
    }
    match current_block_102 {
        6752956218007646265 => {
            if strcmp(key, b"tags\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"time\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut m: int64_t = 0;
                let mut my_time_t_max: int64_t = get_time_t_max();
                let mut my_time_t_min: int64_t = get_time_t_min();
                let mut ns: libc::c_long = 0 as libc::c_int as libc::c_long;
                *parsed_kws |= MTREE_HAS_MTIME;
                m = mtree_atol(&mut val, 10 as libc::c_int);
                /* Replicate an old mtree bug:
                 * 123456789.1 represents 123456789
                 * seconds and 1 nanosecond. */
                if *val as libc::c_int == '.' as i32 {
                    val = val.offset(1);
                    ns = mtree_atol(&mut val, 10 as libc::c_int);
                    if ns < 0 as libc::c_int as libc::c_long {
                        ns = 0 as libc::c_int as libc::c_long
                    } else if ns > 999999999 as libc::c_int as libc::c_long {
                        ns = 999999999 as libc::c_int as libc::c_long
                    }
                }
                if m > my_time_t_max {
                    m = my_time_t_max
                } else if m < my_time_t_min {
                    m = my_time_t_min
                }
                archive_entry_set_mtime(entry, m, ns);
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"type\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut current_block_92: u64;
                match *val.offset(0 as libc::c_int as isize) as libc::c_int {
                    98 => {
                        if strcmp(val, b"block\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            archive_entry_set_filetype(entry, AE_IFBLK as mode_t);
                            current_block_92 = 3267183688118022636;
                        } else {
                            current_block_92 = 18113473374131038547;
                        }
                    }
                    99 => {
                        current_block_92 = 18113473374131038547;
                    }
                    100 => {
                        current_block_92 = 8716068849081884060;
                    }
                    102 => {
                        current_block_92 = 7093963388734721767;
                    }
                    108 => {
                        current_block_92 = 3017423739268653173;
                    }
                    _ => {
                        current_block_92 = 5668240277071890353;
                    }
                }
                match current_block_92 {
                    18113473374131038547 => {
                        if strcmp(val, b"char\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            archive_entry_set_filetype(entry, AE_IFCHR as mode_t);
                            current_block_92 = 3267183688118022636;
                        } else {
                            current_block_92 = 8716068849081884060;
                        }
                    }
                    _ => {}
                }
                match current_block_92 {
                    8716068849081884060 => {
                        if strcmp(val, b"dir\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            archive_entry_set_filetype(entry, AE_IFDIR as mode_t);
                            current_block_92 = 3267183688118022636;
                        } else {
                            current_block_92 = 7093963388734721767;
                        }
                    }
                    _ => {}
                }
                match current_block_92 {
                    7093963388734721767 => {
                        if strcmp(val, b"fifo\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            archive_entry_set_filetype(entry, AE_IFIFO as mode_t);
                            current_block_92 = 3267183688118022636;
                        } else if strcmp(val, b"file\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            archive_entry_set_filetype(entry, AE_IFREG as mode_t);
                            current_block_92 = 3267183688118022636;
                        } else {
                            current_block_92 = 3017423739268653173;
                        }
                    }
                    _ => {}
                }
                match current_block_92 {
                    3017423739268653173 => {
                        if strcmp(val, b"link\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            archive_entry_set_filetype(entry, AE_IFLNK as mode_t);
                            current_block_92 = 3267183688118022636;
                        } else {
                            current_block_92 = 5668240277071890353;
                        }
                    }
                    _ => {}
                }
                match current_block_92 {
                    3267183688118022636 => {}
                    _ => {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Unrecognized file type \"%s\"; assuming \"file\"\x00" as *const u8
                                as *const libc::c_char,
                            val,
                        );
                        archive_entry_set_filetype(entry, AE_IFREG as mode_t);
                        return -(20 as libc::c_int);
                    }
                }
                *parsed_kws |= MTREE_HAS_TYPE;
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 10981874684149351084;
            }
        }
        _ => {}
    }
    match current_block_102 {
        10981874684149351084 => {
            if strcmp(key, b"uid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                *parsed_kws |= MTREE_HAS_UID;
                archive_entry_set_uid(entry, mtree_atol(&mut val, 10 as libc::c_int));
                current_block_102 = 14883390358315838856;
            } else if strcmp(key, b"uname\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                *parsed_kws |= MTREE_HAS_UNAME;
                archive_entry_copy_uname(entry, val);
                current_block_102 = 14883390358315838856;
            } else {
                current_block_102 = 5750090253865036684;
            }
        }
        _ => {}
    }
    match current_block_102 {
        14883390358315838856 =>
            /*
         * Comma delimited list of tags.
         * Ignore the tags for now, but the interface
         * should be extended to allow inclusion/exclusion.
         */
            {}
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unrecognized key %s=%s\x00" as *const u8 as *const libc::c_char,
                key,
                val,
            );
            return -(20 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut bytes_to_read: size_t = 0;
    let mut bytes_read: ssize_t = 0;
    let mut mtree: *mut mtree = 0 as *mut mtree;
    mtree = (*(*a).format).data as *mut mtree;
    if (*mtree).fd < 0 as libc::c_int {
        *buff = NULL as *const libc::c_void;
        *offset = 0 as libc::c_int as int64_t;
        *size = 0 as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    if (*mtree).buff.is_null() {
        (*mtree).buffsize = (64 as libc::c_int * 1024 as libc::c_int) as size_t;
        (*mtree).buff = malloc((*mtree).buffsize) as *mut libc::c_char;
        if (*mtree).buff.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    *buff = (*mtree).buff as *const libc::c_void;
    *offset = (*mtree).offset;
    if (*mtree).buffsize as int64_t > (*mtree).cur_size - (*mtree).offset {
        bytes_to_read = ((*mtree).cur_size - (*mtree).offset) as size_t
    } else {
        bytes_to_read = (*mtree).buffsize
    }
    bytes_read = read(
        (*mtree).fd,
        (*mtree).buff as *mut libc::c_void,
        bytes_to_read,
    );
    if bytes_read < 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"Can\'t read\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    if bytes_read == 0 as libc::c_int as libc::c_long {
        *size = 0 as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    (*mtree).offset += bytes_read;
    *size = bytes_read as size_t;
    return 0 as libc::c_int;
}
/* Skip does nothing except possibly close the contents file. */
unsafe extern "C" fn skip(mut a: *mut archive_read) -> libc::c_int {
    let mut mtree: *mut mtree = 0 as *mut mtree;
    mtree = (*(*a).format).data as *mut mtree;
    if (*mtree).fd >= 0 as libc::c_int {
        close((*mtree).fd);
        (*mtree).fd = -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
/*
 * Since parsing backslash sequences always makes strings shorter,
 * we can always do this conversion in-place.
 */
unsafe extern "C" fn parse_escapes(mut src: *mut libc::c_char, mut mentry: *mut mtree_entry) {
    let mut dest: *mut libc::c_char = src;
    let mut c: libc::c_char = 0;
    if !mentry.is_null()
        && strcmp(src, b".\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*mentry).full = 1 as libc::c_int as libc::c_char
    }
    while *src as libc::c_int != '\u{0}' as i32 {
        let fresh3 = src;
        src = src.offset(1);
        c = *fresh3;
        if c as libc::c_int == '/' as i32 && !mentry.is_null() {
            (*mentry).full = 1 as libc::c_int as libc::c_char
        }
        if c as libc::c_int == '\\' as i32 {
            let mut current_block_30: u64;
            match *src.offset(0 as libc::c_int as isize) as libc::c_int {
                48 => {
                    if (*src.offset(1 as libc::c_int as isize) as libc::c_int) < '0' as i32
                        || *src.offset(1 as libc::c_int as isize) as libc::c_int > '7' as i32
                    {
                        c = 0 as libc::c_int as libc::c_char;
                        src = src.offset(1);
                        current_block_30 = 3934796541983872331;
                    } else {
                        current_block_30 = 17629765841327595216;
                    }
                }
                49 | 50 | 51 => {
                    current_block_30 = 17629765841327595216;
                }
                97 => {
                    c = '\u{7}' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                98 => {
                    c = '\u{8}' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                102 => {
                    c = '\u{c}' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                110 => {
                    c = '\n' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                114 => {
                    c = '\r' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                115 => {
                    c = ' ' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                116 => {
                    c = '\t' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                118 => {
                    c = '\u{b}' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                92 => {
                    c = '\\' as i32 as libc::c_char;
                    src = src.offset(1);
                    current_block_30 = 3934796541983872331;
                }
                _ => {
                    current_block_30 = 3934796541983872331;
                }
            }
            match current_block_30 {
                17629765841327595216 =>
                /* FALLTHROUGH */
                {
                    if *src.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
                        && *src.offset(1 as libc::c_int as isize) as libc::c_int <= '7' as i32
                        && *src.offset(2 as libc::c_int as isize) as libc::c_int >= '0' as i32
                        && *src.offset(2 as libc::c_int as isize) as libc::c_int <= '7' as i32
                    {
                        c = ((*src.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                            << 6 as libc::c_int) as libc::c_char;
                        c = (c as libc::c_int
                            | (*src.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32)
                                << 3 as libc::c_int) as libc::c_char;
                        c = (c as libc::c_int
                            | *src.offset(2 as libc::c_int as isize) as libc::c_int - '0' as i32)
                            as libc::c_char;
                        src = src.offset(3 as libc::c_int as isize)
                    }
                }
                _ => {}
            }
        }
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = c
    }
    *dest = '\u{0}' as i32 as libc::c_char;
}
/* Parse a hex digit. */
unsafe extern "C" fn parsedigit(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return c as libc::c_int - '0' as i32;
    } else if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return c as libc::c_int - 'a' as i32;
    } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return c as libc::c_int - 'A' as i32;
    } else {
        return -(1 as libc::c_int);
    };
}
/*
 * Note that this implementation does not (and should not!) obey
 * locale settings; you cannot simply substitute strtol here, since
 * it does obey locale.
 */
unsafe extern "C" fn mtree_atol(mut p: *mut *mut libc::c_char, mut base: libc::c_int) -> int64_t {
    let mut l: int64_t = 0;
    let mut limit: int64_t = 0;
    let mut digit: libc::c_int = 0;
    let mut last_digit_limit: libc::c_int = 0;
    if base == 0 as libc::c_int {
        if **p as libc::c_int != '0' as i32 {
            base = 10 as libc::c_int
        } else if *(*p).offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *(*p).offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32
        {
            *p = (*p).offset(2 as libc::c_int as isize);
            base = 16 as libc::c_int
        } else {
            base = 8 as libc::c_int
        }
    }
    if **p as libc::c_int == '-' as i32 {
        limit = INT64_MIN / base as libc::c_long;
        last_digit_limit = (INT64_MIN % base as libc::c_long) as libc::c_int;
        *p = (*p).offset(1);
        l = 0 as libc::c_int as int64_t;
        digit = parsedigit(**p);
        while digit >= 0 as libc::c_int && digit < base {
            if l < limit || l == limit && digit > last_digit_limit {
                return INT64_MIN;
            }
            l = l * base as libc::c_long - digit as libc::c_long;
            *p = (*p).offset(1);
            digit = parsedigit(**p)
        }
        return l;
    } else {
        limit = INT64_MAX / base as libc::c_long;
        last_digit_limit = (INT64_MAX % base as libc::c_long) as libc::c_int;
        l = 0 as libc::c_int as int64_t;
        digit = parsedigit(**p);
        while digit >= 0 as libc::c_int && digit < base {
            if l > limit || l == limit && digit > last_digit_limit {
                return INT64_MAX;
            }
            l = l * base as libc::c_long + digit as libc::c_long;
            *p = (*p).offset(1);
            digit = parsedigit(**p)
        }
        return l;
    };
}
/*
 * Returns length of line (including trailing newline)
 * or negative on error.  'start' argument is updated to
 * point to first character of line.
 */
unsafe extern "C" fn readline(
    mut a: *mut archive_read,
    mut mtree: *mut mtree,
    mut start: *mut *mut libc::c_char,
    mut limit: ssize_t,
) -> ssize_t {
    let mut bytes_read: ssize_t = 0;
    let mut total_size: ssize_t = 0 as libc::c_int as ssize_t;
    let mut find_off: ssize_t = 0 as libc::c_int as ssize_t;
    let mut t: *const libc::c_void = 0 as *const libc::c_void;
    let mut nl: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut u: *mut libc::c_char = 0 as *mut libc::c_char;
    loop
    /* Accumulate line in a line buffer. */
    /* Read some more. */
    {
        t = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read);
        if t == NULL as *const libc::c_void {
            return 0 as libc::c_int as ssize_t;
        }
        if bytes_read < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int) as ssize_t;
        }
        nl = memchr(t, '\n' as i32, bytes_read as libc::c_ulong);
        /* If we found '\n', trim the read to end exactly there. */
        if !nl.is_null() {
            bytes_read = (nl as *const libc::c_char).offset_from(t as *const libc::c_char)
                as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }
        if total_size + bytes_read + 1 as libc::c_int as libc::c_long > limit {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Line too long\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        if archive_string_ensure(
            &mut (*mtree).line,
            (total_size + bytes_read + 1 as libc::c_int as libc::c_long) as size_t,
        )
        .is_null()
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate working buffer\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        /* Append new bytes to string. */
        memcpy(
            (*mtree).line.s.offset(total_size as isize) as *mut libc::c_void,
            t,
            bytes_read as libc::c_ulong,
        );
        __archive_read_consume(a, bytes_read);
        total_size += bytes_read;
        *(*mtree).line.s.offset(total_size as isize) = '\u{0}' as i32 as libc::c_char;
        u = (*mtree).line.s.offset(find_off as isize);
        while *u != 0 {
            if *u.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
                /* Ends with unescaped newline. */
                *start = (*mtree).line.s;
                return total_size;
            } else {
                if *u.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
                    /* Ends with comment sequence #...\n */
                    if nl.is_null() {
                        break;
                    }
                } else if *u.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32 {
                    if *u.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
                        /* Trim escaped newline. */
                        total_size -= 2 as libc::c_int as libc::c_long;
                        *(*mtree).line.s.offset(total_size as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        break;
                    } else if *u.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
                    {
                        /* Skip the two-char escape sequence */
                        u = u.offset(1)
                    }
                }
                u = u.offset(1)
            }
        }
        find_off = u.offset_from((*mtree).line.s) as libc::c_long
    }
}
