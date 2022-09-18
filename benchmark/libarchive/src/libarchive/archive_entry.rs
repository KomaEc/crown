use ::libc;
extern "C" {
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    #[no_mangle]
    fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn gnu_dev_makedev(__major: libc::c_uint, __minor: libc::c_uint) -> __dev_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn wmemcmp(_: *const libc::c_int, _: *const libc::c_int, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_clean(_: *mut archive_mstring);
    #[no_mangle]
    fn archive_mstring_copy(dest: *mut archive_mstring, src: *mut archive_mstring);
    #[no_mangle]
    fn archive_mstring_get_mbs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_utf8(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_wcs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const wchar_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_mbs_l(
        _: *mut archive_mstring,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_mbs(_: *mut archive_mstring, mbs: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_utf8(_: *mut archive_mstring, utf8: *const libc::c_char)
        -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_wcs(_: *mut archive_mstring, wcs: *const wchar_t) -> libc::c_int;
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
    fn archive_acl_types(_: *mut archive_acl) -> libc::c_int;
    #[no_mangle]
    fn archive_acl_reset(_: *mut archive_acl, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_acl_next(
        _: *mut archive,
        _: *mut archive_acl,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
        _: *mut *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_acl_add_entry(
        _: *mut archive_acl,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_acl_add_entry_w_len(
        _: *mut archive_acl,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *const wchar_t,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_acl_to_text_w(
        _: *mut archive_acl,
        _: *mut ssize_t,
        _: libc::c_int,
        _: *mut archive,
    ) -> *mut wchar_t;
    #[no_mangle]
    fn archive_acl_to_text_l(
        _: *mut archive_acl,
        _: *mut ssize_t,
        _: libc::c_int,
        _: *mut archive_string_conv,
    ) -> *mut libc::c_char;
    /*
     * ACL text parser.
     */
    #[no_mangle]
    fn archive_acl_from_text_w(
        _: *mut archive_acl,
        _: *const wchar_t,
        _: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_acl_from_text_l(
        _: *mut archive_acl,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_acl_clear(_: *mut archive_acl);
    #[no_mangle]
    fn archive_acl_copy(_: *mut archive_acl, _: *mut archive_acl);
    #[no_mangle]
    fn archive_acl_count(_: *mut archive_acl, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_update_utf8(
        _: *mut archive,
        aes: *mut archive_mstring,
        utf8: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_mbs_len_l(
        _: *mut archive_mstring,
        mbs: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    /*
     * extended attributes
     */
    #[no_mangle]
    fn archive_entry_xattr_clear(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_xattr_add_entry(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: *const libc::c_void,
        _: size_t,
    );
    /*
     * sparse
     */
    #[no_mangle]
    fn archive_entry_sparse_clear(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_sparse_add_entry(_: *mut archive_entry, _: la_int64_t, _: la_int64_t);
    #[no_mangle]
    fn __archive_errx(retvalue: libc::c_int, msg: *const libc::c_char) -> !;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type time_t = __time_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
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
/*
 * Description of an archive entry.
 *
 * Basically, this is a "struct stat" with a few text fields added in.
 *
 * TODO: Add "comment", "charset", and possibly other entries
 * that are supported by "pax interchange" format.  However, GNU, ustar,
 * cpio, and other variants don't support these features, so they're not an
 * excruciatingly high priority right now.
 *
 * TODO: "pax interchange" format allows essentially arbitrary
 * key/value attributes to be attached to any entry.  Supporting
 * such extensions may make this library useful for special
 * applications (e.g., a package manager could attach special
 * package-management attributes to each entry).  There are tricky
 * API issues involved, so this is not going to happen until
 * there's a real demand for it.
 *
 * TODO: Design a good API for handling sparse files.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_entry {
    pub archive: *mut archive,
    pub stat: *mut libc::c_void,
    pub stat_valid: libc::c_int,
    pub ae_stat: aest,
    pub ae_set: libc::c_int,
    pub ae_fflags_text: archive_mstring,
    pub ae_fflags_set: libc::c_ulong,
    pub ae_fflags_clear: libc::c_ulong,
    pub ae_gname: archive_mstring,
    pub ae_hardlink: archive_mstring,
    pub ae_pathname: archive_mstring,
    pub ae_symlink: archive_mstring,
    pub ae_uname: archive_mstring,
    pub ae_sourcepath: archive_mstring,
    pub encryption: libc::c_char,
    pub mac_metadata: *mut libc::c_void,
    pub mac_metadata_size: size_t,
    pub acl: archive_acl,
    pub xattr_head: *mut ae_xattr,
    pub xattr_p: *mut ae_xattr,
    pub sparse_head: *mut ae_sparse,
    pub sparse_tail: *mut ae_sparse,
    pub sparse_p: *mut ae_sparse,
    pub strmode: [libc::c_char; 12],
    pub ae_symlink_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ae_sparse {
    pub next: *mut ae_sparse,
    pub offset: int64_t,
    pub length: int64_t,
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
 * $FreeBSD: head/lib/libarchive/archive_entry_private.h 201096 2009-12-28 02:41:27Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ae_xattr {
    pub next: *mut ae_xattr,
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_void,
    pub size: size_t,
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
/* A "multistring" can hold Unicode, UTF8, or MBS versions of
 * the string.  If you set and read the same version, no translation
 * is done.  If you set and read different versions, the library
 * will attempt to transparently convert.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_mstring {
    pub aes_mbs: archive_string,
    pub aes_utf8: archive_string,
    pub aes_wcs: archive_wstring,
    pub aes_mbs_in_locale: archive_string,
    pub aes_set: libc::c_int,
}
/* Length of malloc-ed storage in bytes. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_wstring {
    pub s: *mut wchar_t,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aest {
    pub aest_atime: int64_t,
    pub aest_atime_nsec: uint32_t,
    pub aest_ctime: int64_t,
    pub aest_ctime_nsec: uint32_t,
    pub aest_mtime: int64_t,
    pub aest_mtime_nsec: uint32_t,
    pub aest_birthtime: int64_t,
    pub aest_birthtime_nsec: uint32_t,
    pub aest_gid: int64_t,
    pub aest_ino: int64_t,
    pub aest_nlink: uint32_t,
    pub aest_size: uint64_t,
    pub aest_uid: int64_t,
    pub aest_dev_is_broken_down: libc::c_int,
    pub aest_dev: dev_t,
    pub aest_devmajor: dev_t,
    pub aest_devminor: dev_t,
    pub aest_rdev_is_broken_down: libc::c_int,
    pub aest_rdev: dev_t,
    pub aest_rdevmajor: dev_t,
    pub aest_rdevminor: dev_t,
}
/*
 * Following code is modified from UC Berkeley sources, and
 * is subject to the following copyright notice.
 */
/*-
 * Copyright (c) 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * Supported file flags on FreeBSD and Mac OS:
 * sappnd,sappend		SF_APPEND
 * arch,archived		SF_ARCHIVED
 * schg,schange,simmutable	SF_IMMUTABLE
 * sunlnk,sunlink		SF_NOUNLINK	(FreeBSD only)
 * uappnd,uappend		UF_APPEND
 * compressed			UF_COMPRESSED	(Mac OS only)
 * hidden,uhidden		UF_HIDDEN
 * uchg,uchange,uimmutable	UF_IMMUTABLE
 * nodump			UF_NODUMP
 * uunlnk,uunlink		UF_NOUNLINK	(FreeBSD only)
 * offline,uoffline		UF_OFFLINE	(FreeBSD only)
 * opaque			UF_OPAQUE
 * rdonly,urdonly,readonly	UF_READONLY	(FreeBSD only)
 * reparse,ureparse		UF_REPARSE	(FreeBSD only)
 * sparse,usparse		UF_SPARSE	(FreeBSD only)
 * system,usystem		UF_SYSTEM	(FreeBSD only)
 *
 * See chflags(2) for more information
 *
 * Supported file attributes on Linux:
 * a	append only			FS_APPEND_FL		sappnd
 * A	no atime updates		FS_NOATIME_FL		atime
 * c	compress			FS_COMPR_FL		compress
 * C	no copy on write		FS_NOCOW_FL		cow
 * d	no dump				FS_NODUMP_FL		dump
 * D	synchronous directory updates	FS_DIRSYNC_FL		dirsync
 * i	immutable			FS_IMMUTABLE_FL		schg
 * j	data journalling		FS_JOURNAL_DATA_FL	journal
 * P	project hierarchy		FS_PROJINHERIT_FL	projinherit
 * s	secure deletion			FS_SECRM_FL		securedeletion
 * S	synchronous updates		FS_SYNC_FL		sync
 * t	no tail-merging			FS_NOTAIL_FL		tail
 * T	top of directory hierarchy	FS_TOPDIR_FL		topdir
 * u	undeletable			FS_UNRM_FL		undel
 *
 * See ioctl_iflags(2) for more information
 *
 * Equivalent file flags supported on FreeBSD / Mac OS and Linux:
 * SF_APPEND		FS_APPEND_FL		sappnd
 * SF_IMMUTABLE		FS_IMMUTABLE_FL		schg
 * UF_NODUMP		FS_NODUMP_FL		nodump
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag {
    pub name: *const libc::c_char,
    pub wname: *const wchar_t,
    pub set: libc::c_ulong,
    pub clear: libc::c_ulong,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const FS_APPEND_FL: libc::c_int = 0x20 as libc::c_int;
pub const FS_IMMUTABLE_FL: libc::c_int = 0x10 as libc::c_int;
pub const FS_NODUMP_FL: libc::c_int = 0x40 as libc::c_int;
pub const FS_UNRM_FL: libc::c_int = 0x2 as libc::c_int;
pub const FS_COMPR_FL: libc::c_int = 0x4 as libc::c_int;
pub const FS_PROJINHERIT_FL: libc::c_int = 0x20000000 as libc::c_int;
pub const FS_NOCOW_FL: libc::c_int = 0x800000 as libc::c_int;
pub const FS_TOPDIR_FL: libc::c_int = 0x20000 as libc::c_int;
pub const FS_NOTAIL_FL: libc::c_int = 0x8000 as libc::c_int;
pub const FS_SYNC_FL: libc::c_int = 0x8 as libc::c_int;
pub const FS_SECRM_FL: libc::c_int = 0x1 as libc::c_int;
pub const FS_JOURNAL_DATA_FL: libc::c_int = 0x4000 as libc::c_int;
pub const FS_DIRSYNC_FL: libc::c_int = 0x10000 as libc::c_int;
pub const FS_NOATIME_FL: libc::c_int = 0x80 as libc::c_int;
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
pub const AE_IFMT: libc::c_int = 0o170000 as libc::c_int;
/*
 * Symlink types
 */
pub const AE_SYMLINK_TYPE_UNDEFINED: libc::c_int = 0 as libc::c_int;
/*
 * ACL routines.  This used to simply store and return text-format ACL
 * strings, but that proved insufficient for a number of reasons:
 *   = clients need control over uname/uid and gname/gid mappings
 *   = there are many different ACL text formats
 *   = would like to be able to read/convert archives containing ACLs
 *     on platforms that lack ACL libraries
 *
 *  This last point, in particular, forces me to implement a reasonably
 *  complete set of ACL support routines.
 */
/*
 * Permission bits.
 */
/*
 * Inheritance values (NFS4 ACLs only); included in permset.
 */
/* We need to be able to specify combinations of these. */
pub const ARCHIVE_ENTRY_ACL_TYPE_ACCESS: libc::c_int = 0x100 as libc::c_int;
/* POSIX.1e only */
pub const ARCHIVE_ENTRY_ACL_TYPE_DEFAULT: libc::c_int = 0x200 as libc::c_int;
/* POSIX.1e only */
/* NFS4 only */
/* NFS4 only */
/* NFS4 only */
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_POSIX1E: libc::c_int =
    ARCHIVE_ENTRY_ACL_TYPE_ACCESS | ARCHIVE_ENTRY_ACL_TYPE_DEFAULT;
/*
 * Construct a text-format ACL.  The flags argument is a bitmask that
 * can include any of the following:
 *
 * Flags only for archive entries with POSIX.1e ACL:
 * ARCHIVE_ENTRY_ACL_TYPE_ACCESS - Include POSIX.1e "access" entries.
 * ARCHIVE_ENTRY_ACL_TYPE_DEFAULT - Include POSIX.1e "default" entries.
 * ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT - Include "default:" before each
 *    default ACL entry.
 * ARCHIVE_ENTRY_ACL_STYLE_SOLARIS - Output only one colon after "other" and
 *    "mask" entries.
 *
 * Flags only for archive entries with NFSv4 ACL:
 * ARCHIVE_ENTRY_ACL_STYLE_COMPACT - Do not output the minus character for
 *    unset permissions and flags in NFSv4 ACL permission and flag fields
 *
 * Flags for for archive entries with POSIX.1e ACL or NFSv4 ACL:
 * ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID - Include extra numeric ID field in
 *    each ACL entry.
 * ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA - Separate entries with comma
 *    instead of newline.
 */
pub const ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID: libc::c_int = 0x1 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT: libc::c_int = 0x2 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA: libc::c_int = 0x8 as libc::c_int;
/* Deprecated constants */
pub const OLD_ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID: libc::c_int = 1024 as libc::c_int;
pub const OLD_ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT: libc::c_int = 2048 as libc::c_int;
pub const AE_SET_HARDLINK: libc::c_int = 1 as libc::c_int;
pub const AE_SET_SYMLINK: libc::c_int = 2 as libc::c_int;
pub const AE_SET_ATIME: libc::c_int = 4 as libc::c_int;
pub const AE_SET_CTIME: libc::c_int = 8 as libc::c_int;
pub const AE_SET_MTIME: libc::c_int = 16 as libc::c_int;
pub const AE_SET_BIRTHTIME: libc::c_int = 32 as libc::c_int;
pub const AE_SET_SIZE: libc::c_int = 64 as libc::c_int;
pub const AE_SET_INO: libc::c_int = 128 as libc::c_int;
pub const AE_SET_DEV: libc::c_int = 256 as libc::c_int;
pub const AE_ENCRYPTION_DATA: libc::c_int = 1 as libc::c_int;
pub const AE_ENCRYPTION_METADATA: libc::c_int = 2 as libc::c_int;
/*
 * Basic object manipulation
 */
/* ***************************************************************************
 *
 * Public Interface
 *
 ****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn archive_entry_clear(mut entry: *mut archive_entry) -> *mut archive_entry {
    if entry.is_null() {
        return 0 as *mut archive_entry;
    }
    archive_mstring_clean(&mut (*entry).ae_fflags_text);
    archive_mstring_clean(&mut (*entry).ae_gname);
    archive_mstring_clean(&mut (*entry).ae_hardlink);
    archive_mstring_clean(&mut (*entry).ae_pathname);
    archive_mstring_clean(&mut (*entry).ae_sourcepath);
    archive_mstring_clean(&mut (*entry).ae_symlink);
    archive_mstring_clean(&mut (*entry).ae_uname);
    archive_entry_copy_mac_metadata(
        entry,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    archive_acl_clear(&mut (*entry).acl);
    archive_entry_xattr_clear(entry);
    archive_entry_sparse_clear(entry);
    free((*entry).stat);
    (*entry).ae_symlink_type = AE_SYMLINK_TYPE_UNDEFINED;
    memset(
        entry as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<archive_entry>() as libc::c_ulong,
    );
    return entry;
}
/* The 'clone' function does a deep copy; all of the strings are copied too. */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_clone(mut entry: *mut archive_entry) -> *mut archive_entry {
    let mut entry2: *mut archive_entry = 0 as *mut archive_entry;
    let mut xp: *mut ae_xattr = 0 as *mut ae_xattr;
    let mut sp: *mut ae_sparse = 0 as *mut ae_sparse;
    let mut s: size_t = 0;
    let mut p: *const libc::c_void = 0 as *const libc::c_void;
    /* Allocate new structure and copy over all of the fields. */
    /* TODO: Should we copy the archive over?  Or require a new archive
     * as an argument? */
    entry2 = archive_entry_new2((*entry).archive);
    if entry2.is_null() {
        return 0 as *mut archive_entry;
    }
    (*entry2).ae_stat = (*entry).ae_stat;
    (*entry2).ae_fflags_set = (*entry).ae_fflags_set;
    (*entry2).ae_fflags_clear = (*entry).ae_fflags_clear;
    /* TODO: XXX If clone can have a different archive, what do we do here if
     * character sets are different? XXX */
    archive_mstring_copy(&mut (*entry2).ae_fflags_text, &mut (*entry).ae_fflags_text);
    archive_mstring_copy(&mut (*entry2).ae_gname, &mut (*entry).ae_gname);
    archive_mstring_copy(&mut (*entry2).ae_hardlink, &mut (*entry).ae_hardlink);
    archive_mstring_copy(&mut (*entry2).ae_pathname, &mut (*entry).ae_pathname);
    archive_mstring_copy(&mut (*entry2).ae_sourcepath, &mut (*entry).ae_sourcepath);
    archive_mstring_copy(&mut (*entry2).ae_symlink, &mut (*entry).ae_symlink);
    (*entry2).ae_set = (*entry).ae_set;
    archive_mstring_copy(&mut (*entry2).ae_uname, &mut (*entry).ae_uname);
    /* Copy symlink type */
    (*entry2).ae_symlink_type = (*entry).ae_symlink_type;
    /* Copy encryption status */
    (*entry2).encryption = (*entry).encryption;
    /* Copy ACL data over. */
    archive_acl_copy(&mut (*entry2).acl, &mut (*entry).acl);
    /* Copy Mac OS metadata. */
    p = archive_entry_mac_metadata(entry, &mut s);
    archive_entry_copy_mac_metadata(entry2, p, s);
    /* Copy xattr data over. */
    xp = (*entry).xattr_head;
    while !xp.is_null() {
        archive_entry_xattr_add_entry(entry2, (*xp).name, (*xp).value, (*xp).size);
        xp = (*xp).next
    }
    /* Copy sparse data over. */
    sp = (*entry).sparse_head;
    while !sp.is_null() {
        archive_entry_sparse_add_entry(entry2, (*sp).offset, (*sp).length);
        sp = (*sp).next
    }
    return entry2;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_free(mut entry: *mut archive_entry) {
    archive_entry_clear(entry);
    free(entry as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_new() -> *mut archive_entry {
    return archive_entry_new2(NULL as *mut archive);
}
/*
 * This form of archive_entry_new2() will pull character-set
 * conversion information from the specified archive handle.  The
 * older archive_entry_new(void) form is equivalent to calling
 * archive_entry_new2(NULL) and will result in the use of an internal
 * default character-set conversion.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_new2(mut a: *mut archive) -> *mut archive_entry {
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    entry = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_entry>() as libc::c_ulong,
    ) as *mut archive_entry;
    if entry.is_null() {
        return 0 as *mut archive_entry;
    }
    (*entry).archive = a;
    (*entry).ae_symlink_type = AE_SYMLINK_TYPE_UNDEFINED;
    return entry;
}
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
/*
 * Functions for reading fields from an archive_entry.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_atime(mut entry: *mut archive_entry) -> time_t {
    return (*entry).ae_stat.aest_atime;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_atime_nsec(mut entry: *mut archive_entry) -> libc::c_long {
    return (*entry).ae_stat.aest_atime_nsec as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_atime_is_set(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).ae_set & AE_SET_ATIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_birthtime(mut entry: *mut archive_entry) -> time_t {
    return (*entry).ae_stat.aest_birthtime;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_birthtime_nsec(
    mut entry: *mut archive_entry,
) -> libc::c_long {
    return (*entry).ae_stat.aest_birthtime_nsec as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_birthtime_is_set(
    mut entry: *mut archive_entry,
) -> libc::c_int {
    return (*entry).ae_set & AE_SET_BIRTHTIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_ctime(mut entry: *mut archive_entry) -> time_t {
    return (*entry).ae_stat.aest_ctime;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_ctime_is_set(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).ae_set & AE_SET_CTIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_ctime_nsec(mut entry: *mut archive_entry) -> libc::c_long {
    return (*entry).ae_stat.aest_ctime_nsec as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_dev(mut entry: *mut archive_entry) -> dev_t {
    if (*entry).ae_stat.aest_dev_is_broken_down != 0 {
        return gnu_dev_makedev(
            (*entry).ae_stat.aest_devmajor as libc::c_uint,
            (*entry).ae_stat.aest_devminor as libc::c_uint,
        );
    } else {
        return (*entry).ae_stat.aest_dev;
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_dev_is_set(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).ae_set & AE_SET_DEV;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_devmajor(mut entry: *mut archive_entry) -> dev_t {
    if (*entry).ae_stat.aest_dev_is_broken_down != 0 {
        return (*entry).ae_stat.aest_devmajor;
    } else {
        return gnu_dev_major((*entry).ae_stat.aest_dev) as dev_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_devminor(mut entry: *mut archive_entry) -> dev_t {
    if (*entry).ae_stat.aest_dev_is_broken_down != 0 {
        return (*entry).ae_stat.aest_devminor;
    } else {
        return gnu_dev_minor((*entry).ae_stat.aest_dev) as dev_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_filetype(mut entry: *mut archive_entry) -> mode_t {
    return AE_IFMT as mode_t & (*entry).acl.mode;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_fflags(
    mut entry: *mut archive_entry,
    mut set: *mut libc::c_ulong,
    mut clear: *mut libc::c_ulong,
) {
    *set = (*entry).ae_fflags_set;
    *clear = (*entry).ae_fflags_clear;
}
/*
 * Note: if text was provided, this just returns that text.  If you
 * really need the text to be rebuilt in a canonical form, set the
 * text, ask for the bitmaps, then set the bitmaps.  (Setting the
 * bitmaps clears any stored text.)  This design is deliberate: if
 * we're editing archives, we don't want to discard flags just because
 * they aren't supported on the current system.  The bitmap<->text
 * conversions are platform-specific (see below).
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_fflags_text(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_fflags_text, &mut f)
        == 0 as libc::c_int
    {
        if !f.is_null() {
            return f;
        }
    } else if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*entry).ae_fflags_set == 0 as libc::c_int as libc::c_ulong
        && (*entry).ae_fflags_clear == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *const libc::c_char;
    }
    p = ae_fflagstostr((*entry).ae_fflags_set, (*entry).ae_fflags_clear);
    if p.is_null() {
        return 0 as *const libc::c_char;
    }
    archive_mstring_copy_mbs(&mut (*entry).ae_fflags_text, p);
    free(p as *mut libc::c_void);
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_fflags_text, &mut f)
        == 0 as libc::c_int
    {
        return f;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_gid(mut entry: *mut archive_entry) -> la_int64_t {
    return (*entry).ae_stat.aest_gid;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_gname(mut entry: *mut archive_entry) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_gname, &mut p) == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_gname_utf8(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if archive_mstring_get_utf8((*entry).archive, &mut (*entry).ae_gname, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_gname_w(mut entry: *mut archive_entry) -> *const wchar_t {
    let mut p: *const wchar_t = 0 as *const wchar_t;
    if archive_mstring_get_wcs((*entry).archive, &mut (*entry).ae_gname, &mut p) == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const wchar_t;
}
/*
 * Utility functions to set and get entry attributes by translating
 * character-set. These are designed for use in format readers and writers.
 *
 * The return code and interface of these are quite different from other
 * functions for archive_entry defined in archive_entry.h.
 * Common return code are:
 *   Return 0 if the string conversion succeeded.
 *   Return -1 if the string conversion failed.
 */
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_gname_l(
    mut entry: *mut archive_entry,
    mut p: *mut *const libc::c_char,
    mut len: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return archive_mstring_get_mbs_l(&mut (*entry).ae_gname, p, len, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_hardlink(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if (*entry).ae_set & AE_SET_HARDLINK == 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_hardlink, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_hardlink_utf8(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if (*entry).ae_set & AE_SET_HARDLINK == 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if archive_mstring_get_utf8((*entry).archive, &mut (*entry).ae_hardlink, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_hardlink_w(mut entry: *mut archive_entry) -> *const wchar_t {
    let mut p: *const wchar_t = 0 as *const wchar_t;
    if (*entry).ae_set & AE_SET_HARDLINK == 0 as libc::c_int {
        return 0 as *const wchar_t;
    }
    if archive_mstring_get_wcs((*entry).archive, &mut (*entry).ae_hardlink, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_hardlink_l(
    mut entry: *mut archive_entry,
    mut p: *mut *const libc::c_char,
    mut len: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    if (*entry).ae_set & AE_SET_HARDLINK == 0 as libc::c_int {
        *p = NULL as *const libc::c_char;
        *len = 0 as libc::c_int as size_t;
        return 0 as libc::c_int;
    }
    return archive_mstring_get_mbs_l(&mut (*entry).ae_hardlink, p, len, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_ino(mut entry: *mut archive_entry) -> la_int64_t {
    return (*entry).ae_stat.aest_ino;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_ino_is_set(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).ae_set & AE_SET_INO;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_ino64(mut entry: *mut archive_entry) -> la_int64_t {
    return (*entry).ae_stat.aest_ino;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_mode(mut entry: *mut archive_entry) -> mode_t {
    return (*entry).acl.mode;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_mtime(mut entry: *mut archive_entry) -> time_t {
    return (*entry).ae_stat.aest_mtime;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_mtime_nsec(mut entry: *mut archive_entry) -> libc::c_long {
    return (*entry).ae_stat.aest_mtime_nsec as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_mtime_is_set(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).ae_set & AE_SET_MTIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_nlink(mut entry: *mut archive_entry) -> libc::c_uint {
    return (*entry).ae_stat.aest_nlink;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_pathname(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_pathname, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_pathname_utf8(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if archive_mstring_get_utf8((*entry).archive, &mut (*entry).ae_pathname, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_pathname_w(mut entry: *mut archive_entry) -> *const wchar_t {
    let mut p: *const wchar_t = 0 as *const wchar_t;
    if archive_mstring_get_wcs((*entry).archive, &mut (*entry).ae_pathname, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_pathname_l(
    mut entry: *mut archive_entry,
    mut p: *mut *const libc::c_char,
    mut len: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return archive_mstring_get_mbs_l(&mut (*entry).ae_pathname, p, len, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_perm(mut entry: *mut archive_entry) -> mode_t {
    return !(AE_IFMT as mode_t) & (*entry).acl.mode;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_rdev(mut entry: *mut archive_entry) -> dev_t {
    if (*entry).ae_stat.aest_rdev_is_broken_down != 0 {
        return gnu_dev_makedev(
            (*entry).ae_stat.aest_rdevmajor as libc::c_uint,
            (*entry).ae_stat.aest_rdevminor as libc::c_uint,
        );
    } else {
        return (*entry).ae_stat.aest_rdev;
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_rdevmajor(mut entry: *mut archive_entry) -> dev_t {
    if (*entry).ae_stat.aest_rdev_is_broken_down != 0 {
        return (*entry).ae_stat.aest_rdevmajor;
    } else {
        return gnu_dev_major((*entry).ae_stat.aest_rdev) as dev_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_rdevminor(mut entry: *mut archive_entry) -> dev_t {
    if (*entry).ae_stat.aest_rdev_is_broken_down != 0 {
        return (*entry).ae_stat.aest_rdevminor;
    } else {
        return gnu_dev_minor((*entry).ae_stat.aest_rdev) as dev_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_size(mut entry: *mut archive_entry) -> la_int64_t {
    return (*entry).ae_stat.aest_size as la_int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_size_is_set(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).ae_set & AE_SET_SIZE;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_sourcepath(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_sourcepath, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_sourcepath_w(
    mut entry: *mut archive_entry,
) -> *const wchar_t {
    let mut p: *const wchar_t = 0 as *const wchar_t;
    if archive_mstring_get_wcs((*entry).archive, &mut (*entry).ae_sourcepath, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    return 0 as *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_symlink(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if (*entry).ae_set & AE_SET_SYMLINK == 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_symlink, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_symlink_type(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).ae_symlink_type;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_symlink_utf8(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if (*entry).ae_set & AE_SET_SYMLINK == 0 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if archive_mstring_get_utf8((*entry).archive, &mut (*entry).ae_symlink, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_symlink_w(mut entry: *mut archive_entry) -> *const wchar_t {
    let mut p: *const wchar_t = 0 as *const wchar_t;
    if (*entry).ae_set & AE_SET_SYMLINK == 0 as libc::c_int {
        return 0 as *const wchar_t;
    }
    if archive_mstring_get_wcs((*entry).archive, &mut (*entry).ae_symlink, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_symlink_l(
    mut entry: *mut archive_entry,
    mut p: *mut *const libc::c_char,
    mut len: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    if (*entry).ae_set & AE_SET_SYMLINK == 0 as libc::c_int {
        *p = NULL as *const libc::c_char;
        *len = 0 as libc::c_int as size_t;
        return 0 as libc::c_int;
    }
    return archive_mstring_get_mbs_l(&mut (*entry).ae_symlink, p, len, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_uid(mut entry: *mut archive_entry) -> la_int64_t {
    return (*entry).ae_stat.aest_uid;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_uname(mut entry: *mut archive_entry) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if archive_mstring_get_mbs((*entry).archive, &mut (*entry).ae_uname, &mut p) == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_uname_utf8(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if archive_mstring_get_utf8((*entry).archive, &mut (*entry).ae_uname, &mut p)
        == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_uname_w(mut entry: *mut archive_entry) -> *const wchar_t {
    let mut p: *const wchar_t = 0 as *const wchar_t;
    if archive_mstring_get_wcs((*entry).archive, &mut (*entry).ae_uname, &mut p) == 0 as libc::c_int
    {
        return p;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *const wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_uname_l(
    mut entry: *mut archive_entry,
    mut p: *mut *const libc::c_char,
    mut len: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return archive_mstring_get_mbs_l(&mut (*entry).ae_uname, p, len, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_is_data_encrypted(
    mut entry: *mut archive_entry,
) -> libc::c_int {
    return ((*entry).encryption as libc::c_int & AE_ENCRYPTION_DATA == AE_ENCRYPTION_DATA)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_is_metadata_encrypted(
    mut entry: *mut archive_entry,
) -> libc::c_int {
    return ((*entry).encryption as libc::c_int & AE_ENCRYPTION_METADATA == AE_ENCRYPTION_METADATA)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_is_encrypted(mut entry: *mut archive_entry) -> libc::c_int {
    return (*entry).encryption as libc::c_int & (AE_ENCRYPTION_DATA | AE_ENCRYPTION_METADATA);
}
/*
 * Functions to set archive_entry properties.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_filetype(
    mut entry: *mut archive_entry,
    mut type_0: libc::c_uint,
) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).acl.mode &= !(AE_IFMT as mode_t);
    (*entry).acl.mode |= AE_IFMT as mode_t & type_0;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_fflags(
    mut entry: *mut archive_entry,
    mut set: libc::c_ulong,
    mut clear: libc::c_ulong,
) {
    archive_mstring_clean(&mut (*entry).ae_fflags_text);
    (*entry).ae_fflags_set = set;
    (*entry).ae_fflags_clear = clear;
}
/* Returns pointer to start of first invalid token, or NULL if none. */
/* Note that all recognized tokens are processed, regardless. */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_fflags_text(
    mut entry: *mut archive_entry,
    mut flags: *const libc::c_char,
) -> *const libc::c_char {
    archive_mstring_copy_mbs(&mut (*entry).ae_fflags_text, flags);
    return ae_strtofflags(
        flags,
        &mut (*entry).ae_fflags_set,
        &mut (*entry).ae_fflags_clear,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_fflags_text_w(
    mut entry: *mut archive_entry,
    mut flags: *const wchar_t,
) -> *const wchar_t {
    archive_mstring_copy_wcs(&mut (*entry).ae_fflags_text, flags);
    return ae_wcstofflags(
        flags,
        &mut (*entry).ae_fflags_set,
        &mut (*entry).ae_fflags_clear,
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_gid(mut entry: *mut archive_entry, mut g: la_int64_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_stat.aest_gid = g;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_gname(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_gname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_gname_utf8(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_utf8(&mut (*entry).ae_gname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_gname(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_gname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_gname_w(
    mut entry: *mut archive_entry,
    mut name: *const wchar_t,
) {
    archive_mstring_copy_wcs(&mut (*entry).ae_gname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_update_gname_utf8(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if archive_mstring_update_utf8((*entry).archive, &mut (*entry).ae_gname, name)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_copy_gname_l(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return archive_mstring_copy_mbs_len_l(&mut (*entry).ae_gname, name, len, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_ino(mut entry: *mut archive_entry, mut ino: la_int64_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_INO;
    (*entry).ae_stat.aest_ino = ino;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_ino64(
    mut entry: *mut archive_entry,
    mut ino: la_int64_t,
) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_INO;
    (*entry).ae_stat.aest_ino = ino;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_hardlink(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_hardlink, target);
    if !target.is_null() {
        (*entry).ae_set |= AE_SET_HARDLINK
    } else {
        (*entry).ae_set &= !AE_SET_HARDLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_hardlink_utf8(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) {
    archive_mstring_copy_utf8(&mut (*entry).ae_hardlink, target);
    if !target.is_null() {
        (*entry).ae_set |= AE_SET_HARDLINK
    } else {
        (*entry).ae_set &= !AE_SET_HARDLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_hardlink(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_hardlink, target);
    if !target.is_null() {
        (*entry).ae_set |= AE_SET_HARDLINK
    } else {
        (*entry).ae_set &= !AE_SET_HARDLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_hardlink_w(
    mut entry: *mut archive_entry,
    mut target: *const wchar_t,
) {
    archive_mstring_copy_wcs(&mut (*entry).ae_hardlink, target);
    if !target.is_null() {
        (*entry).ae_set |= AE_SET_HARDLINK
    } else {
        (*entry).ae_set &= !AE_SET_HARDLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_update_hardlink_utf8(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) -> libc::c_int {
    if !target.is_null() {
        (*entry).ae_set |= AE_SET_HARDLINK
    } else {
        (*entry).ae_set &= !AE_SET_HARDLINK
    }
    if archive_mstring_update_utf8((*entry).archive, &mut (*entry).ae_hardlink, target)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_copy_hardlink_l(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = archive_mstring_copy_mbs_len_l(&mut (*entry).ae_hardlink, target, len, sc);
    if !target.is_null() && r == 0 as libc::c_int {
        (*entry).ae_set |= AE_SET_HARDLINK
    } else {
        (*entry).ae_set &= !AE_SET_HARDLINK
    }
    return r;
}
/*
 * Set fields in an archive_entry.
 *
 * Note: Before libarchive 2.4, there were 'set' and 'copy' versions
 * of the string setters.  'copy' copied the actual string, 'set' just
 * stored the pointer.  In libarchive 2.4 and later, strings are
 * always copied.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_atime(
    mut entry: *mut archive_entry,
    mut t: time_t,
    mut ns: libc::c_long,
) {
    t += ns / 1000000000 as libc::c_int as libc::c_long;
    ns %= 1000000000 as libc::c_int as libc::c_long;
    if ns < 0 as libc::c_int as libc::c_long {
        t -= 1;
        ns += 1000000000 as libc::c_int as libc::c_long
    }
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_ATIME;
    (*entry).ae_stat.aest_atime = t;
    (*entry).ae_stat.aest_atime_nsec = ns as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_unset_atime(mut entry: *mut archive_entry) {
    archive_entry_set_atime(
        entry,
        0 as libc::c_int as time_t,
        0 as libc::c_int as libc::c_long,
    );
    (*entry).ae_set &= !AE_SET_ATIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_birthtime(
    mut entry: *mut archive_entry,
    mut t: time_t,
    mut ns: libc::c_long,
) {
    t += ns / 1000000000 as libc::c_int as libc::c_long;
    ns %= 1000000000 as libc::c_int as libc::c_long;
    if ns < 0 as libc::c_int as libc::c_long {
        t -= 1;
        ns += 1000000000 as libc::c_int as libc::c_long
    }
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_BIRTHTIME;
    (*entry).ae_stat.aest_birthtime = t;
    (*entry).ae_stat.aest_birthtime_nsec = ns as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_unset_birthtime(mut entry: *mut archive_entry) {
    archive_entry_set_birthtime(
        entry,
        0 as libc::c_int as time_t,
        0 as libc::c_int as libc::c_long,
    );
    (*entry).ae_set &= !AE_SET_BIRTHTIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_ctime(
    mut entry: *mut archive_entry,
    mut t: time_t,
    mut ns: libc::c_long,
) {
    t += ns / 1000000000 as libc::c_int as libc::c_long;
    ns %= 1000000000 as libc::c_int as libc::c_long;
    if ns < 0 as libc::c_int as libc::c_long {
        t -= 1;
        ns += 1000000000 as libc::c_int as libc::c_long
    }
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_CTIME;
    (*entry).ae_stat.aest_ctime = t;
    (*entry).ae_stat.aest_ctime_nsec = ns as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_unset_ctime(mut entry: *mut archive_entry) {
    archive_entry_set_ctime(
        entry,
        0 as libc::c_int as time_t,
        0 as libc::c_int as libc::c_long,
    );
    (*entry).ae_set &= !AE_SET_CTIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_dev(mut entry: *mut archive_entry, mut d: dev_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_DEV;
    (*entry).ae_stat.aest_dev_is_broken_down = 0 as libc::c_int;
    (*entry).ae_stat.aest_dev = d;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_devmajor(mut entry: *mut archive_entry, mut m: dev_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_DEV;
    (*entry).ae_stat.aest_dev_is_broken_down = 1 as libc::c_int;
    (*entry).ae_stat.aest_devmajor = m;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_devminor(mut entry: *mut archive_entry, mut m: dev_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_DEV;
    (*entry).ae_stat.aest_dev_is_broken_down = 1 as libc::c_int;
    (*entry).ae_stat.aest_devminor = m;
}
/* Set symlink if symlink is already set, else set hardlink. */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_link(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) {
    if (*entry).ae_set & AE_SET_SYMLINK != 0 {
        archive_mstring_copy_mbs(&mut (*entry).ae_symlink, target);
    } else {
        archive_mstring_copy_mbs(&mut (*entry).ae_hardlink, target);
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_link_utf8(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) {
    if (*entry).ae_set & AE_SET_SYMLINK != 0 {
        archive_mstring_copy_utf8(&mut (*entry).ae_symlink, target);
    } else {
        archive_mstring_copy_utf8(&mut (*entry).ae_hardlink, target);
    };
}
/* Set symlink if symlink is already set, else set hardlink. */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_link(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) {
    if (*entry).ae_set & AE_SET_SYMLINK != 0 {
        archive_mstring_copy_mbs(&mut (*entry).ae_symlink, target);
    } else {
        archive_mstring_copy_mbs(&mut (*entry).ae_hardlink, target);
    };
}
/* Set symlink if symlink is already set, else set hardlink. */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_link_w(
    mut entry: *mut archive_entry,
    mut target: *const wchar_t,
) {
    if (*entry).ae_set & AE_SET_SYMLINK != 0 {
        archive_mstring_copy_wcs(&mut (*entry).ae_symlink, target);
    } else {
        archive_mstring_copy_wcs(&mut (*entry).ae_hardlink, target);
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_update_link_utf8(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*entry).ae_set & AE_SET_SYMLINK != 0 {
        r = archive_mstring_update_utf8((*entry).archive, &mut (*entry).ae_symlink, target)
    } else {
        r = archive_mstring_update_utf8((*entry).archive, &mut (*entry).ae_hardlink, target)
    }
    if r == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_copy_link_l(
    mut entry: *mut archive_entry,
    mut target: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*entry).ae_set & AE_SET_SYMLINK != 0 {
        r = archive_mstring_copy_mbs_len_l(&mut (*entry).ae_symlink, target, len, sc)
    } else {
        r = archive_mstring_copy_mbs_len_l(&mut (*entry).ae_hardlink, target, len, sc)
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_mode(mut entry: *mut archive_entry, mut m: mode_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).acl.mode = m;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_mtime(
    mut entry: *mut archive_entry,
    mut t: time_t,
    mut ns: libc::c_long,
) {
    t += ns / 1000000000 as libc::c_int as libc::c_long;
    ns %= 1000000000 as libc::c_int as libc::c_long;
    if ns < 0 as libc::c_int as libc::c_long {
        t -= 1;
        ns += 1000000000 as libc::c_int as libc::c_long
    }
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_set |= AE_SET_MTIME;
    (*entry).ae_stat.aest_mtime = t;
    (*entry).ae_stat.aest_mtime_nsec = ns as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_unset_mtime(mut entry: *mut archive_entry) {
    archive_entry_set_mtime(
        entry,
        0 as libc::c_int as time_t,
        0 as libc::c_int as libc::c_long,
    );
    (*entry).ae_set &= !AE_SET_MTIME;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_nlink(
    mut entry: *mut archive_entry,
    mut nlink: libc::c_uint,
) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_stat.aest_nlink = nlink;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_pathname(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_pathname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_pathname_utf8(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_utf8(&mut (*entry).ae_pathname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_pathname(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_pathname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_pathname_w(
    mut entry: *mut archive_entry,
    mut name: *const wchar_t,
) {
    archive_mstring_copy_wcs(&mut (*entry).ae_pathname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_update_pathname_utf8(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if archive_mstring_update_utf8((*entry).archive, &mut (*entry).ae_pathname, name)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_copy_pathname_l(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return archive_mstring_copy_mbs_len_l(&mut (*entry).ae_pathname, name, len, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_perm(mut entry: *mut archive_entry, mut p: mode_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).acl.mode &= AE_IFMT as mode_t;
    (*entry).acl.mode |= !(AE_IFMT as mode_t) & p;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_rdev(mut entry: *mut archive_entry, mut m: dev_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_stat.aest_rdev = m;
    (*entry).ae_stat.aest_rdev_is_broken_down = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_rdevmajor(mut entry: *mut archive_entry, mut m: dev_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_stat.aest_rdev_is_broken_down = 1 as libc::c_int;
    (*entry).ae_stat.aest_rdevmajor = m;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_rdevminor(mut entry: *mut archive_entry, mut m: dev_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_stat.aest_rdev_is_broken_down = 1 as libc::c_int;
    (*entry).ae_stat.aest_rdevminor = m;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_size(mut entry: *mut archive_entry, mut s: la_int64_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_stat.aest_size = s as uint64_t;
    (*entry).ae_set |= AE_SET_SIZE;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_unset_size(mut entry: *mut archive_entry) {
    archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
    (*entry).ae_set &= !AE_SET_SIZE;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_sourcepath(
    mut entry: *mut archive_entry,
    mut path: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_sourcepath, path);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_sourcepath_w(
    mut entry: *mut archive_entry,
    mut path: *const wchar_t,
) {
    archive_mstring_copy_wcs(&mut (*entry).ae_sourcepath, path);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_symlink(
    mut entry: *mut archive_entry,
    mut linkname: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_symlink, linkname);
    if !linkname.is_null() {
        (*entry).ae_set |= AE_SET_SYMLINK
    } else {
        (*entry).ae_set &= !AE_SET_SYMLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_symlink_type(
    mut entry: *mut archive_entry,
    mut type_0: libc::c_int,
) {
    (*entry).ae_symlink_type = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_symlink_utf8(
    mut entry: *mut archive_entry,
    mut linkname: *const libc::c_char,
) {
    archive_mstring_copy_utf8(&mut (*entry).ae_symlink, linkname);
    if !linkname.is_null() {
        (*entry).ae_set |= AE_SET_SYMLINK
    } else {
        (*entry).ae_set &= !AE_SET_SYMLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_symlink(
    mut entry: *mut archive_entry,
    mut linkname: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_symlink, linkname);
    if !linkname.is_null() {
        (*entry).ae_set |= AE_SET_SYMLINK
    } else {
        (*entry).ae_set &= !AE_SET_SYMLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_symlink_w(
    mut entry: *mut archive_entry,
    mut linkname: *const wchar_t,
) {
    archive_mstring_copy_wcs(&mut (*entry).ae_symlink, linkname);
    if !linkname.is_null() {
        (*entry).ae_set |= AE_SET_SYMLINK
    } else {
        (*entry).ae_set &= !AE_SET_SYMLINK
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_update_symlink_utf8(
    mut entry: *mut archive_entry,
    mut linkname: *const libc::c_char,
) -> libc::c_int {
    if !linkname.is_null() {
        (*entry).ae_set |= AE_SET_SYMLINK
    } else {
        (*entry).ae_set &= !AE_SET_SYMLINK
    }
    if archive_mstring_update_utf8((*entry).archive, &mut (*entry).ae_symlink, linkname)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_copy_symlink_l(
    mut entry: *mut archive_entry,
    mut linkname: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = archive_mstring_copy_mbs_len_l(&mut (*entry).ae_symlink, linkname, len, sc);
    if !linkname.is_null() && r == 0 as libc::c_int {
        (*entry).ae_set |= AE_SET_SYMLINK
    } else {
        (*entry).ae_set &= !AE_SET_SYMLINK
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_uid(mut entry: *mut archive_entry, mut u: la_int64_t) {
    (*entry).stat_valid = 0 as libc::c_int;
    (*entry).ae_stat.aest_uid = u;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_uname(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_uname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_uname_utf8(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_utf8(&mut (*entry).ae_uname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_uname(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) {
    archive_mstring_copy_mbs(&mut (*entry).ae_uname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_uname_w(
    mut entry: *mut archive_entry,
    mut name: *const wchar_t,
) {
    archive_mstring_copy_wcs(&mut (*entry).ae_uname, name);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_update_uname_utf8(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if archive_mstring_update_utf8((*entry).archive, &mut (*entry).ae_uname, name)
        == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_is_data_encrypted(
    mut entry: *mut archive_entry,
    mut is_encrypted: libc::c_char,
) {
    if is_encrypted != 0 {
        (*entry).encryption =
            ((*entry).encryption as libc::c_int | AE_ENCRYPTION_DATA) as libc::c_char
    } else {
        (*entry).encryption =
            ((*entry).encryption as libc::c_int & !AE_ENCRYPTION_DATA) as libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_set_is_metadata_encrypted(
    mut entry: *mut archive_entry,
    mut is_encrypted: libc::c_char,
) {
    if is_encrypted != 0 {
        (*entry).encryption =
            ((*entry).encryption as libc::c_int | AE_ENCRYPTION_METADATA) as libc::c_char
    } else {
        (*entry).encryption =
            ((*entry).encryption as libc::c_int & !AE_ENCRYPTION_METADATA) as libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_copy_uname_l(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return archive_mstring_copy_mbs_len_l(&mut (*entry).ae_uname, name, len, sc);
}
/*
 * Storage for Mac OS-specific AppleDouble metadata information.
 * Apple-format tar files store a separate binary blob containing
 * encoded metadata with ACL, extended attributes, etc.
 * This provides a place to store that blob.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_mac_metadata(
    mut entry: *mut archive_entry,
    mut s: *mut size_t,
) -> *const libc::c_void {
    *s = (*entry).mac_metadata_size;
    return (*entry).mac_metadata;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_copy_mac_metadata(
    mut entry: *mut archive_entry,
    mut p: *const libc::c_void,
    mut s: size_t,
) {
    free((*entry).mac_metadata);
    if p == NULL as *const libc::c_void || s == 0 as libc::c_int as libc::c_ulong {
        (*entry).mac_metadata = NULL as *mut libc::c_void;
        (*entry).mac_metadata_size = 0 as libc::c_int as size_t
    } else {
        (*entry).mac_metadata_size = s;
        (*entry).mac_metadata = malloc(s);
        if (*entry).mac_metadata.is_null() {
            abort();
        }
        memcpy((*entry).mac_metadata, p, s);
    };
}
/*
 * ACL management.  The following would, of course, be a lot simpler
 * if: 1) the last draft of POSIX.1e were a really thorough and
 * complete standard that addressed the needs of ACL archiving and 2)
 * everyone followed it faithfully.  Alas, neither is true, so the
 * following is a lot more complex than might seem necessary to the
 * uninitiated.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl(mut entry: *mut archive_entry) -> *mut archive_acl {
    return &mut (*entry).acl;
}
/* Tag values mimic POSIX.1e */
/* Specified user. */
/* User who owns the file. */
/* Specified group. */
/* Group who owns the file. */
/* Modify group access (POSIX.1e only) */
/* Public (POSIX.1e only) */
/* Everyone (NFS4 only) */
/*
 * Set the ACL by clearing it and adding entries one at a time.
 * Unlike the POSIX.1e ACL routines, you must specify the type
 * (access/default) for each entry.  Internally, the ACL data is just
 * a soup of entries.  API calls here allow you to retrieve just the
 * entries of interest.  This design (which goes against the spirit of
 * POSIX.1e) is useful for handling archive formats that combine
 * default and access information in a single ACL list.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_clear(mut entry: *mut archive_entry) {
    archive_acl_clear(&mut (*entry).acl);
}
/*
 * Add a single ACL entry to the internal list of ACL data.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_add_entry(
    mut entry: *mut archive_entry,
    mut type_0: libc::c_int,
    mut permset: libc::c_int,
    mut tag: libc::c_int,
    mut id: libc::c_int,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return archive_acl_add_entry(&mut (*entry).acl, type_0, permset, tag, id, name);
}
/*
 * As above, but with a wide-character name.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_add_entry_w(
    mut entry: *mut archive_entry,
    mut type_0: libc::c_int,
    mut permset: libc::c_int,
    mut tag: libc::c_int,
    mut id: libc::c_int,
    mut name: *const wchar_t,
) -> libc::c_int {
    return archive_acl_add_entry_w_len(
        &mut (*entry).acl,
        type_0,
        permset,
        tag,
        id,
        name,
        wcslen(name),
    );
}
/* Return bitmask of ACL types in an archive entry */
/*
 * Return a bitmask of ACL types in an archive entry ACL list
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_types(mut entry: *mut archive_entry) -> libc::c_int {
    return archive_acl_types(&mut (*entry).acl);
}
/* Return a count of entries matching 'want_type' */
/*
 * Return a count of entries matching "want_type".
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_count(
    mut entry: *mut archive_entry,
    mut want_type: libc::c_int,
) -> libc::c_int {
    return archive_acl_count(&mut (*entry).acl, want_type);
}
/*
 * To retrieve the ACL, first "reset", then repeatedly ask for the
 * "next" entry.  The want_type parameter allows you to request only
 * certain types of entries.
 */
/*
 * Prepare for reading entries from the ACL data.  Returns a count
 * of entries matching "want_type", or zero if there are no
 * non-extended ACL entries of that type.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_reset(
    mut entry: *mut archive_entry,
    mut want_type: libc::c_int,
) -> libc::c_int {
    return archive_acl_reset(&mut (*entry).acl, want_type);
}
/*
 * Return the next ACL entry in the list.  Fake entries for the
 * standard permissions and include them in the returned list.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_next(
    mut entry: *mut archive_entry,
    mut want_type: libc::c_int,
    mut type_0: *mut libc::c_int,
    mut permset: *mut libc::c_int,
    mut tag: *mut libc::c_int,
    mut id: *mut libc::c_int,
    mut name: *mut *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = archive_acl_next(
        (*entry).archive,
        &mut (*entry).acl,
        want_type,
        type_0,
        permset,
        tag,
        id,
        name,
    );
    if r == ARCHIVE_FATAL && errno == ENOMEM {
        __archive_errx(
            1 as libc::c_int,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return r;
}
/*
 * Generate a text version of the ACL. The flags parameter controls
 * the style of the generated ACL.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_to_text_w(
    mut entry: *mut archive_entry,
    mut len: *mut la_ssize_t,
    mut flags: libc::c_int,
) -> *mut wchar_t {
    return archive_acl_to_text_w(&mut (*entry).acl, len, flags, (*entry).archive);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_to_text(
    mut entry: *mut archive_entry,
    mut len: *mut la_ssize_t,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    return archive_acl_to_text_l(
        &mut (*entry).acl,
        len,
        flags,
        NULL as *mut archive_string_conv,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_acl_to_text_l(
    mut entry: *mut archive_entry,
    mut len: *mut ssize_t,
    mut flags: libc::c_int,
    mut sc: *mut archive_string_conv,
) -> *mut libc::c_char {
    return archive_acl_to_text_l(&mut (*entry).acl, len, flags, sc);
}
/*
 * ACL text parser.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_from_text_w(
    mut entry: *mut archive_entry,
    mut wtext: *const wchar_t,
    mut type_0: libc::c_int,
) -> libc::c_int {
    return archive_acl_from_text_w(&mut (*entry).acl, wtext, type_0);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_from_text(
    mut entry: *mut archive_entry,
    mut text: *const libc::c_char,
    mut type_0: libc::c_int,
) -> libc::c_int {
    return archive_acl_from_text_l(
        &mut (*entry).acl,
        text,
        type_0,
        NULL as *mut archive_string_conv,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_acl_from_text_l(
    mut entry: *mut archive_entry,
    mut text: *const libc::c_char,
    mut type_0: libc::c_int,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return archive_acl_from_text_l(&mut (*entry).acl, text, type_0, sc);
}
/* Deprecated */
unsafe extern "C" fn archive_entry_acl_text_compat(mut flags: *mut libc::c_int) -> libc::c_int {
    if *flags & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    /* ABI compat with old ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID */
    if *flags & OLD_ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID != 0 as libc::c_int {
        *flags |= ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID
    }
    /* ABI compat with old ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT */
    if *flags & OLD_ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT != 0 as libc::c_int {
        *flags |= ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT
    }
    *flags |= ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA;
    return 0 as libc::c_int;
}
/* Deprecated functions */
/* Deprecated */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_text_w(
    mut entry: *mut archive_entry,
    mut flags: libc::c_int,
) -> *const wchar_t {
    free((*entry).acl.acl_text_w as *mut libc::c_void);
    (*entry).acl.acl_text_w = NULL as *mut wchar_t;
    if archive_entry_acl_text_compat(&mut flags) == 0 as libc::c_int {
        (*entry).acl.acl_text_w = archive_acl_to_text_w(
            &mut (*entry).acl,
            NULL as *mut ssize_t,
            flags,
            (*entry).archive,
        )
    }
    return (*entry).acl.acl_text_w;
}
/* Deprecated */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_acl_text(
    mut entry: *mut archive_entry,
    mut flags: libc::c_int,
) -> *const libc::c_char {
    free((*entry).acl.acl_text as *mut libc::c_void);
    (*entry).acl.acl_text = NULL as *mut libc::c_char;
    if archive_entry_acl_text_compat(&mut flags) == 0 as libc::c_int {
        (*entry).acl.acl_text = archive_acl_to_text_l(
            &mut (*entry).acl,
            NULL as *mut ssize_t,
            flags,
            NULL as *mut archive_string_conv,
        )
    }
    return (*entry).acl.acl_text;
}
/* Deprecated */
#[no_mangle]
pub unsafe extern "C" fn _archive_entry_acl_text_l(
    mut entry: *mut archive_entry,
    mut flags: libc::c_int,
    mut acl_text: *mut *const libc::c_char,
    mut len: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    free((*entry).acl.acl_text as *mut libc::c_void);
    (*entry).acl.acl_text = NULL as *mut libc::c_char;
    if archive_entry_acl_text_compat(&mut flags) == 0 as libc::c_int {
        (*entry).acl.acl_text =
            archive_acl_to_text_l(&mut (*entry).acl, len as *mut ssize_t, flags, sc)
    }
    *acl_text = (*entry).acl.acl_text;
    return 0 as libc::c_int;
}
static mut fileflags: [flag; 20] = unsafe {
    [
        {
            let mut init =
                 flag{name:
                          b"nosappnd\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_int; 9]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00a\x00\x00\x00p\x00\x00\x00p\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_APPEND_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nosappend\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_int; 10]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00a\x00\x00\x00p\x00\x00\x00p\x00\x00\x00e\x00\x00\x00n\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_APPEND_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name: b"noschg\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_int; 7]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00c\x00\x00\x00h\x00\x00\x00g\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_IMMUTABLE_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"noschange\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_int; 10]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00c\x00\x00\x00h\x00\x00\x00a\x00\x00\x00n\x00\x00\x00g\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_IMMUTABLE_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nosimmutable\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 52],
                                                    &[libc::c_int; 13]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00i\x00\x00\x00m\x00\x00\x00m\x00\x00\x00u\x00\x00\x00t\x00\x00\x00a\x00\x00\x00b\x00\x00\x00l\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_IMMUTABLE_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name: b"nodump\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_int; 7]>(b"n\x00\x00\x00o\x00\x00\x00d\x00\x00\x00u\x00\x00\x00m\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: 0 as libc::c_int as libc::c_ulong,
                      clear: FS_NODUMP_FL as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"noundel\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 32],
                                                    &[libc::c_int; 8]>(b"n\x00\x00\x00o\x00\x00\x00u\x00\x00\x00n\x00\x00\x00d\x00\x00\x00e\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_UNRM_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nocompress\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 44],
                                                    &[libc::c_int; 11]>(b"n\x00\x00\x00o\x00\x00\x00c\x00\x00\x00o\x00\x00\x00m\x00\x00\x00p\x00\x00\x00r\x00\x00\x00e\x00\x00\x00s\x00\x00\x00s\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_COMPR_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"noatime\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 32],
                                                    &[libc::c_int; 8]>(b"n\x00\x00\x00o\x00\x00\x00a\x00\x00\x00t\x00\x00\x00i\x00\x00\x00m\x00\x00\x00e\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: 0 as libc::c_int as libc::c_ulong,
                      clear: FS_NOATIME_FL as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nodirsync\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_int; 10]>(b"n\x00\x00\x00o\x00\x00\x00d\x00\x00\x00i\x00\x00\x00r\x00\x00\x00s\x00\x00\x00y\x00\x00\x00n\x00\x00\x00c\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_DIRSYNC_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nojournal-data\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 60],
                                                    &[libc::c_int; 15]>(b"n\x00\x00\x00o\x00\x00\x00j\x00\x00\x00o\x00\x00\x00u\x00\x00\x00r\x00\x00\x00n\x00\x00\x00a\x00\x00\x00l\x00\x00\x00-\x00\x00\x00d\x00\x00\x00a\x00\x00\x00t\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_JOURNAL_DATA_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nojournal\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 40],
                                                    &[libc::c_int; 10]>(b"n\x00\x00\x00o\x00\x00\x00j\x00\x00\x00o\x00\x00\x00u\x00\x00\x00r\x00\x00\x00n\x00\x00\x00a\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_JOURNAL_DATA_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nosecdel\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_int; 9]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00e\x00\x00\x00c\x00\x00\x00d\x00\x00\x00e\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_SECRM_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"nosecuredeletion\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 68],
                                                    &[libc::c_int; 17]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00e\x00\x00\x00c\x00\x00\x00u\x00\x00\x00r\x00\x00\x00e\x00\x00\x00d\x00\x00\x00e\x00\x00\x00l\x00\x00\x00e\x00\x00\x00t\x00\x00\x00i\x00\x00\x00o\x00\x00\x00n\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_SECRM_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name: b"nosync\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_int; 7]>(b"n\x00\x00\x00o\x00\x00\x00s\x00\x00\x00y\x00\x00\x00n\x00\x00\x00c\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_SYNC_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name: b"notail\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 28],
                                                    &[libc::c_int; 7]>(b"n\x00\x00\x00o\x00\x00\x00t\x00\x00\x00a\x00\x00\x00i\x00\x00\x00l\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: 0 as libc::c_int as libc::c_ulong,
                      clear: FS_NOTAIL_FL as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"notopdir\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_int; 9]>(b"n\x00\x00\x00o\x00\x00\x00t\x00\x00\x00o\x00\x00\x00p\x00\x00\x00d\x00\x00\x00i\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_TOPDIR_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name: b"nocow\x00" as *const u8 as *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 24],
                                                    &[libc::c_int; 6]>(b"n\x00\x00\x00o\x00\x00\x00c\x00\x00\x00o\x00\x00\x00w\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: 0 as libc::c_int as libc::c_ulong,
                      clear: FS_NOCOW_FL as libc::c_ulong,};
            init
        },
        {
            let mut init =
                 flag{name:
                          b"noprojinherit\x00" as *const u8 as
                              *const libc::c_char,
                      wname:
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_int; 14]>(b"n\x00\x00\x00o\x00\x00\x00p\x00\x00\x00r\x00\x00\x00o\x00\x00\x00j\x00\x00\x00i\x00\x00\x00n\x00\x00\x00h\x00\x00\x00e\x00\x00\x00r\x00\x00\x00i\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                      set: FS_PROJINHERIT_FL as libc::c_ulong,
                      clear: 0 as libc::c_int as libc::c_ulong,};
            init
        },
        {
            let mut init = flag {
                name: NULL as *const libc::c_char,
                wname: NULL as *const wchar_t,
                set: 0 as libc::c_int as libc::c_ulong,
                clear: 0 as libc::c_int as libc::c_ulong,
            };
            init
        },
    ]
};
/* for Linux file flags */
/*
 * Some Linux distributions have both linux/ext2_fs.h and ext2fs/ext2_fs.h.
 * As the include guards don't agree, the order of include is important.
 */
/* Play games to come up with a suitable makedev() definition. */
/* There's a "makedev" macro. */
/*
 * This adjustment is needed to support the following idiom for adding
 * 1000ns to the stored time:
 * archive_entry_set_atime(archive_entry_atime(),
 *                         archive_entry_atime_nsec() + 1000)
 * The additional if() here compensates for ambiguity in the C standard,
 * which permits two possible interpretations of a % b when a is negative.
 */
/*
 * fflagstostr --
 *	Convert file flags to a comma-separated string.  If no flags
 *	are set, return the empty string.
 */
unsafe extern "C" fn ae_fflagstostr(
    mut bitset: libc::c_ulong,
    mut bitclear: libc::c_ulong,
) -> *mut libc::c_char {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    let mut bits: libc::c_ulong = 0;
    let mut flag: *const flag = 0 as *const flag;
    let mut length: size_t = 0;
    bits = bitset | bitclear;
    length = 0 as libc::c_int as size_t;
    flag = fileflags.as_ptr();
    while !(*flag).name.is_null() {
        if bits & ((*flag).set | (*flag).clear) != 0 {
            length = (length as libc::c_ulong)
                .wrapping_add(strlen((*flag).name).wrapping_add(1 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
            bits &= !((*flag).set | (*flag).clear)
        }
        flag = flag.offset(1)
    }
    if length == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_char;
    }
    string = malloc(length) as *mut libc::c_char;
    if string.is_null() {
        return 0 as *mut libc::c_char;
    }
    dp = string;
    let mut current_block_25: u64;
    flag = fileflags.as_ptr();
    while !(*flag).name.is_null() {
        if bitset & (*flag).set != 0 || bitclear & (*flag).clear != 0 {
            sp = (*flag).name.offset(2 as libc::c_int as isize);
            current_block_25 = 14576567515993809846;
        } else if bitset & (*flag).clear != 0 || bitclear & (*flag).set != 0 {
            sp = (*flag).name;
            current_block_25 = 14576567515993809846;
        } else {
            current_block_25 = 1054647088692577877;
        }
        match current_block_25 {
            14576567515993809846 => {
                bitset &= !((*flag).set | (*flag).clear);
                bitclear &= !((*flag).set | (*flag).clear);
                if dp > string {
                    let fresh0 = dp;
                    dp = dp.offset(1);
                    *fresh0 = ',' as i32 as libc::c_char
                }
                loop {
                    let fresh1 = sp;
                    sp = sp.offset(1);
                    let fresh2 = dp;
                    dp = dp.offset(1);
                    *fresh2 = *fresh1;
                    if !(*fresh2 as libc::c_int != '\u{0}' as i32) {
                        break;
                    }
                }
                dp = dp.offset(-1)
            }
            _ => {}
        }
        flag = flag.offset(1)
    }
    *dp = '\u{0}' as i32 as libc::c_char;
    return string;
}
/*
 * strtofflags --
 *	Take string of arguments and return file flags.  This
 *	version works a little differently than strtofflags(3).
 *	In particular, it always tests every token, skipping any
 *	unrecognized tokens.  It returns a pointer to the first
 *	unrecognized token, or NULL if every token was recognized.
 *	This version is also const-correct and does not modify the
 *	provided string.
 */
unsafe extern "C" fn ae_strtofflags(
    mut s: *const libc::c_char,
    mut setp: *mut libc::c_ulong,
    mut clrp: *mut libc::c_ulong,
) -> *const libc::c_char {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut flag: *const flag = 0 as *const flag;
    let mut set: libc::c_ulong = 0;
    let mut clear: libc::c_ulong = 0;
    let mut failed: *const libc::c_char = 0 as *const libc::c_char;
    clear = 0 as libc::c_int as libc::c_ulong;
    set = clear;
    start = s;
    failed = NULL as *const libc::c_char;
    /* Find start of first token. */
    while *start as libc::c_int == '\t' as i32
        || *start as libc::c_int == ' ' as i32
        || *start as libc::c_int == ',' as i32
    {
        start = start.offset(1)
    }
    while *start as libc::c_int != '\u{0}' as i32 {
        let mut length: size_t = 0;
        /* Locate end of token. */
        end = start;
        while *end as libc::c_int != '\u{0}' as i32
            && *end as libc::c_int != '\t' as i32
            && *end as libc::c_int != ' ' as i32
            && *end as libc::c_int != ',' as i32
        {
            end = end.offset(1)
        }
        length = end.offset_from(start) as libc::c_long as size_t;
        flag = fileflags.as_ptr();
        while !(*flag).name.is_null() {
            let mut flag_length: size_t = strlen((*flag).name);
            if length == flag_length
                && memcmp(
                    start as *const libc::c_void,
                    (*flag).name as *const libc::c_void,
                    length,
                ) == 0 as libc::c_int
            {
                /* Matched "noXXXX", so reverse the sense. */
                clear |= (*flag).set;
                set |= (*flag).clear;
                break;
            } else if length == flag_length.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                && memcmp(
                    start as *const libc::c_void,
                    (*flag).name.offset(2 as libc::c_int as isize) as *const libc::c_void,
                    length,
                ) == 0 as libc::c_int
            {
                /* Matched "XXXX", so don't reverse. */
                set |= (*flag).set;
                clear |= (*flag).clear;
                break;
            } else {
                flag = flag.offset(1)
            }
        }
        /* Ignore unknown flag names. */
        if (*flag).name.is_null() && failed.is_null() {
            failed = start
        }
        /* Find start of next token. */
        start = end;
        while *start as libc::c_int == '\t' as i32
            || *start as libc::c_int == ' ' as i32
            || *start as libc::c_int == ',' as i32
        {
            start = start.offset(1)
        }
    }
    if !setp.is_null() {
        *setp = set
    }
    if !clrp.is_null() {
        *clrp = clear
    }
    /* Return location of first failure. */
    return failed;
}
/*
 * wcstofflags --
 *	Take string of arguments and return file flags.  This
 *	version works a little differently than strtofflags(3).
 *	In particular, it always tests every token, skipping any
 *	unrecognized tokens.  It returns a pointer to the first
 *	unrecognized token, or NULL if every token was recognized.
 *	This version is also const-correct and does not modify the
 *	provided string.
 */
unsafe extern "C" fn ae_wcstofflags(
    mut s: *const wchar_t,
    mut setp: *mut libc::c_ulong,
    mut clrp: *mut libc::c_ulong,
) -> *const wchar_t {
    let mut start: *const wchar_t = 0 as *const wchar_t;
    let mut end: *const wchar_t = 0 as *const wchar_t;
    let mut flag: *const flag = 0 as *const flag;
    let mut set: libc::c_ulong = 0;
    let mut clear: libc::c_ulong = 0;
    let mut failed: *const wchar_t = 0 as *const wchar_t;
    clear = 0 as libc::c_int as libc::c_ulong;
    set = clear;
    start = s;
    failed = NULL as *const wchar_t;
    /* Find start of first token. */
    while *start == '\t' as i32 || *start == ' ' as i32 || *start == ',' as i32 {
        start = start.offset(1)
    }
    while *start != '\u{0}' as i32 {
        let mut length: size_t = 0;
        /* Locate end of token. */
        end = start;
        while *end != '\u{0}' as i32
            && *end != '\t' as i32
            && *end != ' ' as i32
            && *end != ',' as i32
        {
            end = end.offset(1)
        }
        length = end.offset_from(start) as libc::c_long as size_t;
        flag = fileflags.as_ptr();
        while !(*flag).wname.is_null() {
            let mut flag_length: size_t = wcslen((*flag).wname);
            if length == flag_length && wmemcmp(start, (*flag).wname, length) == 0 as libc::c_int {
                /* Matched "noXXXX", so reverse the sense. */
                clear |= (*flag).set;
                set |= (*flag).clear;
                break;
            } else if length == flag_length.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                && wmemcmp(
                    start,
                    (*flag).wname.offset(2 as libc::c_int as isize),
                    length,
                ) == 0 as libc::c_int
            {
                /* Matched "XXXX", so don't reverse. */
                set |= (*flag).set;
                clear |= (*flag).clear;
                break;
            } else {
                flag = flag.offset(1)
            }
        }
        /* Ignore unknown flag names. */
        if (*flag).wname.is_null() && failed.is_null() {
            failed = start
        }
        /* Find start of next token. */
        start = end;
        while *start == '\t' as i32 || *start == ' ' as i32 || *start == ',' as i32 {
            start = start.offset(1)
        }
    }
    if !setp.is_null() {
        *setp = set
    }
    if !clrp.is_null() {
        *clrp = clear
    }
    /* Return location of first failure. */
    return failed;
}
