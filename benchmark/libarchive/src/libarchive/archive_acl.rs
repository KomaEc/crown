use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn wcscpy(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn wmemcmp(_: *const libc::c_int, _: *const libc::c_int, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_mstring_copy_mbs_len_l(
        _: *mut archive_mstring,
        mbs: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_wcs_len(
        _: *mut archive_mstring,
        wcs: *const wchar_t,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_mbs(_: *mut archive_mstring, mbs: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_mbs_l(
        _: *mut archive_mstring,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_wcs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const wchar_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_get_mbs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy(dest: *mut archive_mstring, src: *mut archive_mstring);
    #[no_mangle]
    fn archive_mstring_clean(_: *mut archive_mstring);
    #[no_mangle]
    fn __archive_errx(retvalue: libc::c_int, msg: *const libc::c_char) -> !;
}
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_wstring {
    pub s: *mut wchar_t,
    pub length: size_t,
    pub buffer_length: size_t,
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
pub struct archive_acl_entry {
    pub next: *mut archive_acl_entry,
    pub type_0: libc::c_int,
    pub tag: libc::c_int,
    pub permset: libc::c_int,
    pub id: libc::c_int,
    pub name: archive_mstring,
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
pub struct C2RustUnnamed {
    pub perm: libc::c_int,
    pub c: libc::c_char,
    pub wc: wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub perm: libc::c_int,
    pub c: libc::c_char,
    pub wc: wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub start: *const wchar_t,
    pub end: *const wchar_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub start: *const libc::c_char,
    pub end: *const libc::c_char,
}
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
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
/* The la_ssize_t should match the type used in 'struct stat' */
/* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
/* Large file support for Android */
/*
 * On Windows, define LIBARCHIVE_STATIC if you're building or using a
 * .lib.  The default here assumes you're building a DLL.  Only
 * libarchive source should ever define __LIBARCHIVE_BUILD.
 */
/* Static libraries or non-Windows needs no special declaration. */
/*
 * The version number is provided as both a macro and a function.
 * The macro identifies the installed header; the function identifies
 * the library version (which may not be the same if you're using a
 * dynamically-linked version of the library).  Of course, if the
 * header and library are very different, you should expect some
 * strangeness.  Don't do that.
 */
/*
 * Textual name/version of the library, useful for version displays.
 */
/*
 * Detailed textual name/version of the library and its dependencies.
 * This has the form:
 *    "libarchive x.y.z zlib/a.b.c liblzma/d.e.f ... etc ..."
 * the list of libraries described here will vary depending on how
 * libarchive was compiled.
 */
/*
 * Returns NULL if libarchive was compiled without the associated library.
 * Otherwise, returns the version number that libarchive was compiled
 * against.
 */
/* Declare our basic types. */
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
/* Operation was successful. */
/* Retry might succeed. */
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
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
pub const ARCHIVE_ENTRY_ACL_EXECUTE: libc::c_int = 0x1 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_WRITE: libc::c_int = 0x2 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_READ: libc::c_int = 0x4 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_READ_DATA: libc::c_int = 0x8 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_LIST_DIRECTORY: libc::c_int = 0x8 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_WRITE_DATA: libc::c_int = 0x10 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ADD_FILE: libc::c_int = 0x10 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_APPEND_DATA: libc::c_int = 0x20 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ADD_SUBDIRECTORY: libc::c_int = 0x20 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_READ_NAMED_ATTRS: libc::c_int = 0x40 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_WRITE_NAMED_ATTRS: libc::c_int = 0x80 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_DELETE_CHILD: libc::c_int = 0x100 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_READ_ATTRIBUTES: libc::c_int = 0x200 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_WRITE_ATTRIBUTES: libc::c_int = 0x400 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_DELETE: libc::c_int = 0x800 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_READ_ACL: libc::c_int = 0x1000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_WRITE_ACL: libc::c_int = 0x2000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_WRITE_OWNER: libc::c_int = 0x4000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_SYNCHRONIZE: libc::c_int = 0x8000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_PERMS_POSIX1E: libc::c_int =
    ARCHIVE_ENTRY_ACL_EXECUTE | ARCHIVE_ENTRY_ACL_WRITE | ARCHIVE_ENTRY_ACL_READ;
pub const ARCHIVE_ENTRY_ACL_PERMS_NFS4: libc::c_int = ARCHIVE_ENTRY_ACL_EXECUTE
    | ARCHIVE_ENTRY_ACL_READ_DATA
    | ARCHIVE_ENTRY_ACL_LIST_DIRECTORY
    | ARCHIVE_ENTRY_ACL_WRITE_DATA
    | ARCHIVE_ENTRY_ACL_ADD_FILE
    | ARCHIVE_ENTRY_ACL_APPEND_DATA
    | ARCHIVE_ENTRY_ACL_ADD_SUBDIRECTORY
    | ARCHIVE_ENTRY_ACL_READ_NAMED_ATTRS
    | ARCHIVE_ENTRY_ACL_WRITE_NAMED_ATTRS
    | ARCHIVE_ENTRY_ACL_DELETE_CHILD
    | ARCHIVE_ENTRY_ACL_READ_ATTRIBUTES
    | ARCHIVE_ENTRY_ACL_WRITE_ATTRIBUTES
    | ARCHIVE_ENTRY_ACL_DELETE
    | ARCHIVE_ENTRY_ACL_READ_ACL
    | ARCHIVE_ENTRY_ACL_WRITE_ACL
    | ARCHIVE_ENTRY_ACL_WRITE_OWNER
    | ARCHIVE_ENTRY_ACL_SYNCHRONIZE;
/*
 * Inheritance values (NFS4 ACLs only); included in permset.
 */
pub const ARCHIVE_ENTRY_ACL_ENTRY_INHERITED: libc::c_int = 0x1000000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ENTRY_FILE_INHERIT: libc::c_int = 0x2000000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ENTRY_DIRECTORY_INHERIT: libc::c_int = 0x4000000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ENTRY_NO_PROPAGATE_INHERIT: libc::c_int = 0x8000000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ENTRY_INHERIT_ONLY: libc::c_int = 0x10000000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ENTRY_SUCCESSFUL_ACCESS: libc::c_int = 0x20000000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_ENTRY_FAILED_ACCESS: libc::c_int = 0x40000000 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_INHERITANCE_NFS4: libc::c_int = ARCHIVE_ENTRY_ACL_ENTRY_FILE_INHERIT
    | ARCHIVE_ENTRY_ACL_ENTRY_DIRECTORY_INHERIT
    | ARCHIVE_ENTRY_ACL_ENTRY_NO_PROPAGATE_INHERIT
    | ARCHIVE_ENTRY_ACL_ENTRY_INHERIT_ONLY
    | ARCHIVE_ENTRY_ACL_ENTRY_SUCCESSFUL_ACCESS
    | ARCHIVE_ENTRY_ACL_ENTRY_FAILED_ACCESS
    | ARCHIVE_ENTRY_ACL_ENTRY_INHERITED;
/* We need to be able to specify combinations of these. */
pub const ARCHIVE_ENTRY_ACL_TYPE_ACCESS: libc::c_int = 0x100 as libc::c_int;
/* POSIX.1e only */
pub const ARCHIVE_ENTRY_ACL_TYPE_DEFAULT: libc::c_int = 0x200 as libc::c_int;
/* POSIX.1e only */
pub const ARCHIVE_ENTRY_ACL_TYPE_ALLOW: libc::c_int = 0x400 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_DENY: libc::c_int = 0x800 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_AUDIT: libc::c_int = 0x1000 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_ALARM: libc::c_int = 0x2000 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_POSIX1E: libc::c_int =
    ARCHIVE_ENTRY_ACL_TYPE_ACCESS | ARCHIVE_ENTRY_ACL_TYPE_DEFAULT;
pub const ARCHIVE_ENTRY_ACL_TYPE_NFS4: libc::c_int = ARCHIVE_ENTRY_ACL_TYPE_ALLOW
    | ARCHIVE_ENTRY_ACL_TYPE_DENY
    | ARCHIVE_ENTRY_ACL_TYPE_AUDIT
    | ARCHIVE_ENTRY_ACL_TYPE_ALARM;
/* Tag values mimic POSIX.1e */
pub const ARCHIVE_ENTRY_ACL_USER: libc::c_int = 10001 as libc::c_int;
/* Specified user. */
pub const ARCHIVE_ENTRY_ACL_USER_OBJ: libc::c_int = 10002 as libc::c_int;
/* User who owns the file. */
pub const ARCHIVE_ENTRY_ACL_GROUP: libc::c_int = 10003 as libc::c_int;
/* Specified group. */
pub const ARCHIVE_ENTRY_ACL_GROUP_OBJ: libc::c_int = 10004 as libc::c_int;
/* Group who owns the file. */
pub const ARCHIVE_ENTRY_ACL_MASK: libc::c_int = 10005 as libc::c_int;
/* Modify group access (POSIX.1e only) */
pub const ARCHIVE_ENTRY_ACL_OTHER: libc::c_int = 10006 as libc::c_int;
/* Public (POSIX.1e only) */
pub const ARCHIVE_ENTRY_ACL_EVERYONE: libc::c_int = 10107 as libc::c_int;
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
pub const ARCHIVE_ENTRY_ACL_STYLE_SOLARIS: libc::c_int = 0x4 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA: libc::c_int = 0x8 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_STYLE_COMPACT: libc::c_int = 0x10 as libc::c_int;
static mut nfsv4_acl_perm_map: [C2RustUnnamed_0; 14] = [
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_READ_DATA | ARCHIVE_ENTRY_ACL_LIST_DIRECTORY,
            c: 'r' as i32 as libc::c_char,
            wc: 'r' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_WRITE_DATA | ARCHIVE_ENTRY_ACL_ADD_FILE,
            c: 'w' as i32 as libc::c_char,
            wc: 'w' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_EXECUTE,
            c: 'x' as i32 as libc::c_char,
            wc: 'x' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_APPEND_DATA | ARCHIVE_ENTRY_ACL_ADD_SUBDIRECTORY,
            c: 'p' as i32 as libc::c_char,
            wc: 'p' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_DELETE,
            c: 'd' as i32 as libc::c_char,
            wc: 'd' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_DELETE_CHILD,
            c: 'D' as i32 as libc::c_char,
            wc: 'D' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_READ_ATTRIBUTES,
            c: 'a' as i32 as libc::c_char,
            wc: 'a' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_WRITE_ATTRIBUTES,
            c: 'A' as i32 as libc::c_char,
            wc: 'A' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_READ_NAMED_ATTRS,
            c: 'R' as i32 as libc::c_char,
            wc: 'R' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_WRITE_NAMED_ATTRS,
            c: 'W' as i32 as libc::c_char,
            wc: 'W' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_READ_ACL,
            c: 'c' as i32 as libc::c_char,
            wc: 'c' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_WRITE_ACL,
            c: 'C' as i32 as libc::c_char,
            wc: 'C' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_WRITE_OWNER,
            c: 'o' as i32 as libc::c_char,
            wc: 'o' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            perm: ARCHIVE_ENTRY_ACL_SYNCHRONIZE,
            c: 's' as i32 as libc::c_char,
            wc: 's' as i32,
        };
        init
    },
];
// Initialized in run_static_initializers
static mut nfsv4_acl_perm_map_size: libc::c_int = 0;
static mut nfsv4_acl_flag_map: [C2RustUnnamed; 7] = [
    {
        let mut init = C2RustUnnamed {
            perm: ARCHIVE_ENTRY_ACL_ENTRY_FILE_INHERIT,
            c: 'f' as i32 as libc::c_char,
            wc: 'f' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            perm: ARCHIVE_ENTRY_ACL_ENTRY_DIRECTORY_INHERIT,
            c: 'd' as i32 as libc::c_char,
            wc: 'd' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            perm: ARCHIVE_ENTRY_ACL_ENTRY_INHERIT_ONLY,
            c: 'i' as i32 as libc::c_char,
            wc: 'i' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            perm: ARCHIVE_ENTRY_ACL_ENTRY_NO_PROPAGATE_INHERIT,
            c: 'n' as i32 as libc::c_char,
            wc: 'n' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            perm: ARCHIVE_ENTRY_ACL_ENTRY_SUCCESSFUL_ACCESS,
            c: 'S' as i32 as libc::c_char,
            wc: 'S' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            perm: ARCHIVE_ENTRY_ACL_ENTRY_FAILED_ACCESS,
            c: 'F' as i32 as libc::c_char,
            wc: 'F' as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed {
            perm: ARCHIVE_ENTRY_ACL_ENTRY_INHERITED,
            c: 'I' as i32 as libc::c_char,
            wc: 'I' as i32,
        };
        init
    },
];
// Initialized in run_static_initializers
static mut nfsv4_acl_flag_map_size: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn archive_acl_clear(mut acl: *mut archive_acl) {
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    while !(*acl).acl_head.is_null() {
        ap = (*(*acl).acl_head).next;
        archive_mstring_clean(&mut (*(*acl).acl_head).name);
        free((*acl).acl_head as *mut libc::c_void);
        (*acl).acl_head = ap
    }
    free((*acl).acl_text_w as *mut libc::c_void);
    (*acl).acl_text_w = NULL as *mut wchar_t;
    free((*acl).acl_text as *mut libc::c_void);
    (*acl).acl_text = NULL as *mut libc::c_char;
    (*acl).acl_p = NULL as *mut archive_acl_entry;
    (*acl).acl_types = 0 as libc::c_int;
    (*acl).acl_state = 0 as libc::c_int;
    /* Not counting. */
}
#[no_mangle]
pub unsafe extern "C" fn archive_acl_copy(mut dest: *mut archive_acl, mut src: *mut archive_acl) {
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    let mut ap2: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    archive_acl_clear(dest);
    (*dest).mode = (*src).mode;
    ap = (*src).acl_head;
    while !ap.is_null() {
        ap2 = acl_new_entry(dest, (*ap).type_0, (*ap).permset, (*ap).tag, (*ap).id);
        if !ap2.is_null() {
            archive_mstring_copy(&mut (*ap2).name, &mut (*ap).name);
        }
        ap = (*ap).next
    }
}
#[no_mangle]
pub unsafe extern "C" fn archive_acl_add_entry(
    mut acl: *mut archive_acl,
    mut type_0: libc::c_int,
    mut permset: libc::c_int,
    mut tag: libc::c_int,
    mut id: libc::c_int,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    if acl_special(acl, type_0, permset, tag) == 0 as libc::c_int {
        return ARCHIVE_OK;
    }
    ap = acl_new_entry(acl, type_0, permset, tag, id);
    if ap.is_null() {
        /* XXX Error XXX */
        return ARCHIVE_FAILED;
    }
    if !name.is_null() && *name as libc::c_int != '\u{0}' as i32 {
        archive_mstring_copy_mbs(&mut (*ap).name, name);
    } else {
        archive_mstring_clean(&mut (*ap).name);
    }
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_acl_add_entry_w_len(
    mut acl: *mut archive_acl,
    mut type_0: libc::c_int,
    mut permset: libc::c_int,
    mut tag: libc::c_int,
    mut id: libc::c_int,
    mut name: *const wchar_t,
    mut len: size_t,
) -> libc::c_int {
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    if acl_special(acl, type_0, permset, tag) == 0 as libc::c_int {
        return ARCHIVE_OK;
    }
    ap = acl_new_entry(acl, type_0, permset, tag, id);
    if ap.is_null() {
        /* XXX Error XXX */
        return ARCHIVE_FAILED;
    }
    if !name.is_null() && *name != '\u{0}' as i32 && len > 0 as libc::c_int as libc::c_ulong {
        archive_mstring_copy_wcs_len(&mut (*ap).name, name, len);
    } else {
        archive_mstring_clean(&mut (*ap).name);
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn archive_acl_add_entry_len_l(
    mut acl: *mut archive_acl,
    mut type_0: libc::c_int,
    mut permset: libc::c_int,
    mut tag: libc::c_int,
    mut id: libc::c_int,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    let mut r: libc::c_int = 0;
    if acl_special(acl, type_0, permset, tag) == 0 as libc::c_int {
        return ARCHIVE_OK;
    }
    ap = acl_new_entry(acl, type_0, permset, tag, id);
    if ap.is_null() {
        /* XXX Error XXX */
        return ARCHIVE_FAILED;
    }
    if !name.is_null()
        && *name as libc::c_int != '\u{0}' as i32
        && len > 0 as libc::c_int as libc::c_ulong
    {
        r = archive_mstring_copy_mbs_len_l(&mut (*ap).name, name, len, sc)
    } else {
        r = 0 as libc::c_int;
        archive_mstring_clean(&mut (*ap).name);
    }
    if r == 0 as libc::c_int {
        return 0 as libc::c_int;
    } else if errno == ENOMEM {
        return -(30 as libc::c_int);
    } else {
        return -(20 as libc::c_int);
    };
}
/*
 * If this ACL entry is part of the standard POSIX permissions set,
 * store the permissions in the stat structure and return zero.
 */
unsafe extern "C" fn acl_special(
    mut acl: *mut archive_acl,
    mut type_0: libc::c_int,
    mut permset: libc::c_int,
    mut tag: libc::c_int,
) -> libc::c_int {
    if type_0 == ARCHIVE_ENTRY_ACL_TYPE_ACCESS
        && permset & !(0o7 as libc::c_int) == 0 as libc::c_int
    {
        match tag {
            ARCHIVE_ENTRY_ACL_USER_OBJ => {
                (*acl).mode &= !(0o700 as libc::c_int) as libc::c_uint;
                (*acl).mode |= ((permset & 7 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
                return 0 as libc::c_int;
            }
            ARCHIVE_ENTRY_ACL_GROUP_OBJ => {
                (*acl).mode &= !(0o70 as libc::c_int) as libc::c_uint;
                (*acl).mode |= ((permset & 7 as libc::c_int) << 3 as libc::c_int) as libc::c_uint;
                return 0 as libc::c_int;
            }
            ARCHIVE_ENTRY_ACL_OTHER => {
                (*acl).mode &= !(0o7 as libc::c_int) as libc::c_uint;
                (*acl).mode |= (permset & 7 as libc::c_int) as libc::c_uint;
                return 0 as libc::c_int;
            }
            _ => {}
        }
    }
    return 1 as libc::c_int;
}
/*
 * Allocate and populate a new ACL entry with everything but the
 * name.
 */
unsafe extern "C" fn acl_new_entry(
    mut acl: *mut archive_acl,
    mut type_0: libc::c_int,
    mut permset: libc::c_int,
    mut tag: libc::c_int,
    mut id: libc::c_int,
) -> *mut archive_acl_entry {
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    let mut aq: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    /* Type argument must be a valid NFS4 or POSIX.1e type.
     * The type must agree with anything already set and
     * the permset must be compatible. */
    if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 {
        if (*acl).acl_types & !ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 {
            return 0 as *mut archive_acl_entry;
        }
        if permset & !(ARCHIVE_ENTRY_ACL_PERMS_NFS4 | ARCHIVE_ENTRY_ACL_INHERITANCE_NFS4) != 0 {
            return 0 as *mut archive_acl_entry;
        }
    } else if type_0 & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 {
        if (*acl).acl_types & !ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 {
            return 0 as *mut archive_acl_entry;
        }
        if permset & !ARCHIVE_ENTRY_ACL_PERMS_POSIX1E != 0 {
            return 0 as *mut archive_acl_entry;
        }
    } else {
        return 0 as *mut archive_acl_entry;
    }
    /* Verify the tag is valid and compatible with NFS4 or POSIX.1e. */
    match tag {
        ARCHIVE_ENTRY_ACL_USER
        | ARCHIVE_ENTRY_ACL_USER_OBJ
        | ARCHIVE_ENTRY_ACL_GROUP
        | ARCHIVE_ENTRY_ACL_GROUP_OBJ => {}
        ARCHIVE_ENTRY_ACL_MASK | ARCHIVE_ENTRY_ACL_OTHER => {
            /* Tags valid only in POSIX.1e. */
            if type_0 & !ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 {
                return 0 as *mut archive_acl_entry;
            }
        }
        ARCHIVE_ENTRY_ACL_EVERYONE => {
            /* Tags valid only in NFS4. */
            if type_0 & !ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 {
                return 0 as *mut archive_acl_entry;
            }
        }
        _ => {
            /* No other values are valid. */
            return 0 as *mut archive_acl_entry;
        }
    }
    free((*acl).acl_text_w as *mut libc::c_void);
    (*acl).acl_text_w = NULL as *mut wchar_t;
    free((*acl).acl_text as *mut libc::c_void);
    (*acl).acl_text = NULL as *mut libc::c_char;
    /*
     * If there's a matching entry already in the list, overwrite it.
     * NFSv4 entries may be repeated and are not overwritten.
     *
     * TODO: compare names of no id is provided (needs more rework)
     */
    ap = (*acl).acl_head;
    aq = NULL as *mut archive_acl_entry;
    while !ap.is_null() {
        if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 == 0 as libc::c_int
            && (*ap).type_0 == type_0
            && (*ap).tag == tag
            && (*ap).id == id
        {
            if id != -(1 as libc::c_int)
                || tag != ARCHIVE_ENTRY_ACL_USER && tag != ARCHIVE_ENTRY_ACL_GROUP
            {
                (*ap).permset = permset;
                return ap;
            }
        }
        aq = ap;
        ap = (*ap).next
    }
    /* Add a new entry to the end of the list. */
    ap = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_acl_entry>() as libc::c_ulong,
    ) as *mut archive_acl_entry;
    if ap.is_null() {
        return 0 as *mut archive_acl_entry;
    }
    if aq.is_null() {
        (*acl).acl_head = ap
    } else {
        (*aq).next = ap
    }
    (*ap).type_0 = type_0;
    (*ap).tag = tag;
    (*ap).id = id;
    (*ap).permset = permset;
    (*acl).acl_types |= type_0;
    return ap;
}
/*
 * Return a count of entries matching "want_type".
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_count(
    mut acl: *mut archive_acl,
    mut want_type: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    count = 0 as libc::c_int;
    ap = (*acl).acl_head;
    while !ap.is_null() {
        if (*ap).type_0 & want_type != 0 as libc::c_int {
            count += 1
        }
        ap = (*ap).next
    }
    if count > 0 as libc::c_int && want_type & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        count += 3 as libc::c_int
    }
    return count;
}
/*
 * Return a bitmask of stored ACL types in an ACL list
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_types(mut acl: *mut archive_acl) -> libc::c_int {
    return (*acl).acl_types;
}
/*
 * Prepare for reading entries from the ACL data.  Returns a count
 * of entries matching "want_type", or zero if there are no
 * non-extended ACL entries of that type.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_reset(
    mut acl: *mut archive_acl,
    mut want_type: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut cutoff: libc::c_int = 0;
    count = archive_acl_count(acl, want_type);
    /*
     * If the only entries are the three standard ones,
     * then don't return any ACL data.  (In this case,
     * client can just use chmod(2) to set permissions.)
     */
    if want_type & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        cutoff = 3 as libc::c_int
    } else {
        cutoff = 0 as libc::c_int
    }
    if count > cutoff {
        (*acl).acl_state = ARCHIVE_ENTRY_ACL_USER_OBJ
    } else {
        (*acl).acl_state = 0 as libc::c_int
    }
    (*acl).acl_p = (*acl).acl_head;
    return count;
}
/*
 * Return the next ACL entry in the list.  Fake entries for the
 * standard permissions and include them in the returned list.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_next(
    mut a: *mut archive,
    mut acl: *mut archive_acl,
    mut want_type: libc::c_int,
    mut type_0: *mut libc::c_int,
    mut permset: *mut libc::c_int,
    mut tag: *mut libc::c_int,
    mut id: *mut libc::c_int,
    mut name: *mut *const libc::c_char,
) -> libc::c_int {
    *name = NULL as *const libc::c_char;
    *id = -(1 as libc::c_int);
    /*
     * The acl_state is either zero (no entries available), -1
     * (reading from list), or an entry type (retrieve that type
     * from ae_stat.aest_mode).
     */
    if (*acl).acl_state == 0 as libc::c_int {
        return -(20 as libc::c_int);
    }
    /* The first three access entries are special. */
    if want_type & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        match (*acl).acl_state {
            ARCHIVE_ENTRY_ACL_USER_OBJ => {
                *permset = ((*acl).mode >> 6 as libc::c_int & 7 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                *type_0 = ARCHIVE_ENTRY_ACL_TYPE_ACCESS;
                *tag = ARCHIVE_ENTRY_ACL_USER_OBJ;
                (*acl).acl_state = ARCHIVE_ENTRY_ACL_GROUP_OBJ;
                return 0 as libc::c_int;
            }
            ARCHIVE_ENTRY_ACL_GROUP_OBJ => {
                *permset = ((*acl).mode >> 3 as libc::c_int & 7 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                *type_0 = ARCHIVE_ENTRY_ACL_TYPE_ACCESS;
                *tag = ARCHIVE_ENTRY_ACL_GROUP_OBJ;
                (*acl).acl_state = ARCHIVE_ENTRY_ACL_OTHER;
                return 0 as libc::c_int;
            }
            ARCHIVE_ENTRY_ACL_OTHER => {
                *permset = ((*acl).mode & 7 as libc::c_int as libc::c_uint) as libc::c_int;
                *type_0 = ARCHIVE_ENTRY_ACL_TYPE_ACCESS;
                *tag = ARCHIVE_ENTRY_ACL_OTHER;
                (*acl).acl_state = -(1 as libc::c_int);
                (*acl).acl_p = (*acl).acl_head;
                return 0 as libc::c_int;
            }
            _ => {}
        }
    }
    while !(*acl).acl_p.is_null() && (*(*acl).acl_p).type_0 & want_type == 0 as libc::c_int {
        (*acl).acl_p = (*(*acl).acl_p).next
    }
    if (*acl).acl_p.is_null() {
        (*acl).acl_state = 0 as libc::c_int;
        *type_0 = 0 as libc::c_int;
        *permset = 0 as libc::c_int;
        *tag = 0 as libc::c_int;
        *id = -(1 as libc::c_int);
        *name = NULL as *const libc::c_char;
        return 1 as libc::c_int;
        /* End of ACL entries. */
    }
    *type_0 = (*(*acl).acl_p).type_0;
    *permset = (*(*acl).acl_p).permset;
    *tag = (*(*acl).acl_p).tag;
    *id = (*(*acl).acl_p).id;
    if archive_mstring_get_mbs(a, &mut (*(*acl).acl_p).name, name) != 0 as libc::c_int {
        if errno == ENOMEM {
            return -(30 as libc::c_int);
        }
        *name = NULL as *const libc::c_char
    }
    (*acl).acl_p = (*(*acl).acl_p).next;
    return 0 as libc::c_int;
}
/*
 * Determine what type of ACL do we want
 */
unsafe extern "C" fn archive_acl_text_want_type(
    mut acl: *mut archive_acl,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut want_type: libc::c_int = 0;
    /* Check if ACL is NFSv4 */
    if (*acl).acl_types & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 as libc::c_int {
        /* NFSv4 should never mix with POSIX.1e */
        if (*acl).acl_types & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 as libc::c_int {
            return 0 as libc::c_int;
        } else {
            return ARCHIVE_ENTRY_ACL_TYPE_ALLOW
                | ARCHIVE_ENTRY_ACL_TYPE_DENY
                | ARCHIVE_ENTRY_ACL_TYPE_AUDIT
                | ARCHIVE_ENTRY_ACL_TYPE_ALARM;
        }
    }
    /* Now deal with POSIX.1e ACLs */
    want_type = 0 as libc::c_int;
    if flags & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        want_type |= ARCHIVE_ENTRY_ACL_TYPE_ACCESS
    }
    if flags & ARCHIVE_ENTRY_ACL_TYPE_DEFAULT != 0 as libc::c_int {
        want_type |= ARCHIVE_ENTRY_ACL_TYPE_DEFAULT
    }
    /* By default we want both access and default ACLs */
    if want_type == 0 as libc::c_int {
        return ARCHIVE_ENTRY_ACL_TYPE_ACCESS | ARCHIVE_ENTRY_ACL_TYPE_DEFAULT;
    }
    return want_type;
}
/*
 * Calculate ACL text string length
 */
unsafe extern "C" fn archive_acl_text_len(
    mut acl: *mut archive_acl,
    mut want_type: libc::c_int,
    mut flags: libc::c_int,
    mut wide: libc::c_int,
    mut a: *mut archive,
    mut sc: *mut archive_string_conv,
) -> ssize_t {
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut wname: *const wchar_t = 0 as *const wchar_t;
    let mut count: libc::c_int = 0;
    let mut idlen: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut length: ssize_t = 0;
    let mut len: size_t = 0;
    count = 0 as libc::c_int;
    length = 0 as libc::c_int as ssize_t;
    ap = (*acl).acl_head;
    while !ap.is_null() {
        if !((*ap).type_0 & want_type == 0 as libc::c_int) {
            /*
             * Filemode-mapping ACL entries are stored exclusively in
             * ap->mode so they should not be in the list
             */
            if !((*ap).type_0 == ARCHIVE_ENTRY_ACL_TYPE_ACCESS
                && ((*ap).tag == ARCHIVE_ENTRY_ACL_USER_OBJ
                    || (*ap).tag == ARCHIVE_ENTRY_ACL_GROUP_OBJ
                    || (*ap).tag == ARCHIVE_ENTRY_ACL_OTHER))
            {
                count += 1; /* "default:" */
                if want_type & ARCHIVE_ENTRY_ACL_TYPE_DEFAULT != 0 as libc::c_int
                    && (*ap).type_0 & ARCHIVE_ENTRY_ACL_TYPE_DEFAULT != 0 as libc::c_int
                {
                    length += 8 as libc::c_int as libc::c_long
                } /* "owner@" */
                let mut current_block_10: u64; /* "group@" */
                match (*ap).tag {
                    ARCHIVE_ENTRY_ACL_USER_OBJ => {
                        if want_type == ARCHIVE_ENTRY_ACL_TYPE_NFS4 {
                            length += 6 as libc::c_int as libc::c_long; /* "everyone@" */
                            current_block_10 = 2719512138335094285;
                        } else {
                            current_block_10 = 12585582625550259790;
                        }
                    }
                    ARCHIVE_ENTRY_ACL_USER | ARCHIVE_ENTRY_ACL_MASK => {
                        current_block_10 = 12585582625550259790;
                    }
                    ARCHIVE_ENTRY_ACL_GROUP_OBJ => {
                        if want_type == ARCHIVE_ENTRY_ACL_TYPE_NFS4 {
                            length += 6 as libc::c_int as libc::c_long;
                            current_block_10 = 2719512138335094285;
                        } else {
                            current_block_10 = 12667278935024443006;
                        }
                    }
                    ARCHIVE_ENTRY_ACL_GROUP | ARCHIVE_ENTRY_ACL_OTHER => {
                        current_block_10 = 12667278935024443006;
                    }
                    ARCHIVE_ENTRY_ACL_EVERYONE => {
                        length += 9 as libc::c_int as libc::c_long;
                        current_block_10 = 2719512138335094285;
                    }
                    _ => {
                        current_block_10 = 2719512138335094285;
                    }
                }
                match current_block_10 {
                    12585582625550259790 =>
                    /* FALLTHROUGH */
                    {
                        length += 4 as libc::c_int as libc::c_long
                    }
                    12667278935024443006 =>
                    /* "user", "mask" */
                    /* FALLTHROUGH */
                    {
                        length += 5 as libc::c_int as libc::c_long
                    }
                    _ => {}
                } /* "group", "other" */
                length += 1 as libc::c_int as libc::c_long; /* colon after tag */
                if (*ap).tag == ARCHIVE_ENTRY_ACL_USER || (*ap).tag == ARCHIVE_ENTRY_ACL_GROUP {
                    if wide != 0 {
                        r = archive_mstring_get_wcs(a, &mut (*ap).name, &mut wname); /* 2nd colon empty user,group or other */
                        if r == 0 as libc::c_int && !wname.is_null() {
                            length = (length as libc::c_ulong).wrapping_add(wcslen(wname))
                                as ssize_t as ssize_t
                        } else if r < 0 as libc::c_int && errno == ENOMEM {
                            return 0 as libc::c_int as ssize_t;
                        } else {
                            length = (length as libc::c_ulong).wrapping_add(
                                (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as ssize_t as ssize_t
                        }
                    } else {
                        r = archive_mstring_get_mbs_l(&mut (*ap).name, &mut name, &mut len, sc);
                        if r != 0 as libc::c_int {
                            return 0 as libc::c_int as ssize_t;
                        }
                        if len > 0 as libc::c_int as libc::c_ulong && !name.is_null() {
                            length =
                                (length as libc::c_ulong).wrapping_add(len) as ssize_t as ssize_t
                        } else {
                            length = (length as libc::c_ulong).wrapping_add(
                                (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as ssize_t as ssize_t
                        }
                    }
                    length += 1 as libc::c_int as libc::c_long
                /* colon after user or group name */
                } else if want_type != ARCHIVE_ENTRY_ACL_TYPE_NFS4 {
                    length += 1 as libc::c_int as libc::c_long
                }
                if flags & ARCHIVE_ENTRY_ACL_STYLE_SOLARIS != 0 as libc::c_int
                    && want_type & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 as libc::c_int
                    && ((*ap).tag == ARCHIVE_ENTRY_ACL_OTHER || (*ap).tag == ARCHIVE_ENTRY_ACL_MASK)
                {
                    /* Solaris has no colon after other: and mask: */
                    length = length - 1 as libc::c_int as libc::c_long
                } /* rwx */
                if want_type == ARCHIVE_ENTRY_ACL_TYPE_NFS4 {
                    /* rwxpdDaARWcCos:fdinSFI:deny */
                    length += 27 as libc::c_int as libc::c_long;
                    if (*ap).type_0 & ARCHIVE_ENTRY_ACL_TYPE_DENY == 0 as libc::c_int {
                        length += 1 as libc::c_int as libc::c_long
                    }
                /* allow, alarm, audit */
                } else {
                    length += 3 as libc::c_int as libc::c_long
                } /* colon */
                if ((*ap).tag == ARCHIVE_ENTRY_ACL_USER || (*ap).tag == ARCHIVE_ENTRY_ACL_GROUP)
                    && flags & ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID != 0 as libc::c_int
                {
                    length += 1 as libc::c_int as libc::c_long;
                    /* ID digit count */
                    idlen = 1 as libc::c_int;
                    tmp = (*ap).id;
                    while tmp > 9 as libc::c_int {
                        tmp = tmp / 10 as libc::c_int;
                        idlen += 1
                    }
                    length += idlen as libc::c_long
                }
                length += 1
            }
        }
        ap = (*ap).next
        /* entry separator */
    }
    /* Add filemode-mapping access entries to the length */
    if want_type & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        if flags & ARCHIVE_ENTRY_ACL_STYLE_SOLARIS != 0 as libc::c_int {
            /* "user::rwx\ngroup::rwx\nother:rwx\n" */
            length += 31 as libc::c_int as libc::c_long
        } else {
            /* "user::rwx\ngroup::rwx\nother::rwx\n" */
            length += 32 as libc::c_int as libc::c_long
        }
    } else if count == 0 as libc::c_int {
        return 0 as libc::c_int as ssize_t;
    }
    /* The terminating character is included in count */
    return length;
}
/*
 * Generate a wide text version of the ACL. The flags parameter controls
 * the type and style of the generated ACL.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_to_text_w(
    mut acl: *mut archive_acl,
    mut text_len: *mut ssize_t,
    mut flags: libc::c_int,
    mut a: *mut archive,
) -> *mut wchar_t {
    let mut count: libc::c_int = 0;
    let mut length: ssize_t = 0;
    let mut len: size_t = 0;
    let mut wname: *const wchar_t = 0 as *const wchar_t;
    let mut prefix: *const wchar_t = 0 as *const wchar_t;
    let mut separator: wchar_t = 0;
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    let mut id: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut want_type: libc::c_int = 0;
    let mut wp: *mut wchar_t = 0 as *mut wchar_t;
    let mut ws: *mut wchar_t = 0 as *mut wchar_t;
    want_type = archive_acl_text_want_type(acl, flags);
    /* Both NFSv4 and POSIX.1 types found */
    if want_type == 0 as libc::c_int {
        return 0 as *mut wchar_t;
    }
    if want_type == ARCHIVE_ENTRY_ACL_TYPE_POSIX1E {
        flags |= ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT
    }
    length = archive_acl_text_len(
        acl,
        want_type,
        flags,
        1 as libc::c_int,
        a,
        NULL as *mut archive_string_conv,
    );
    if length == 0 as libc::c_int as libc::c_long {
        return 0 as *mut wchar_t;
    }
    if flags & ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA != 0 {
        separator = ',' as i32
    } else {
        separator = '\n' as i32
    }
    /* Now, allocate the string and actually populate it. */
    ws = malloc(
        (length as libc::c_ulong).wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
    ) as *mut wchar_t;
    wp = ws;
    if wp.is_null() {
        if errno == ENOMEM {
            __archive_errx(
                1 as libc::c_int,
                b"No memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as *mut wchar_t;
    }
    count = 0 as libc::c_int;
    if want_type & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        append_entry_w(
            &mut wp,
            NULL as *const wchar_t,
            ARCHIVE_ENTRY_ACL_TYPE_ACCESS,
            ARCHIVE_ENTRY_ACL_USER_OBJ,
            flags,
            NULL as *const wchar_t,
            ((*acl).mode & 0o700 as libc::c_int as libc::c_uint) as libc::c_int,
            -(1 as libc::c_int),
        );
        let fresh0 = wp;
        wp = wp.offset(1);
        *fresh0 = separator;
        append_entry_w(
            &mut wp,
            NULL as *const wchar_t,
            ARCHIVE_ENTRY_ACL_TYPE_ACCESS,
            ARCHIVE_ENTRY_ACL_GROUP_OBJ,
            flags,
            NULL as *const wchar_t,
            ((*acl).mode & 0o70 as libc::c_int as libc::c_uint) as libc::c_int,
            -(1 as libc::c_int),
        );
        let fresh1 = wp;
        wp = wp.offset(1);
        *fresh1 = separator;
        append_entry_w(
            &mut wp,
            NULL as *const wchar_t,
            ARCHIVE_ENTRY_ACL_TYPE_ACCESS,
            ARCHIVE_ENTRY_ACL_OTHER,
            flags,
            NULL as *const wchar_t,
            ((*acl).mode & 0o7 as libc::c_int as libc::c_uint) as libc::c_int,
            -(1 as libc::c_int),
        );
        count += 3 as libc::c_int
    }
    ap = (*acl).acl_head;
    while !ap.is_null() {
        if !((*ap).type_0 & want_type == 0 as libc::c_int) {
            /*
             * Filemode-mapping ACL entries are stored exclusively in
             * ap->mode so they should not be in the list
             */
            if !((*ap).type_0 == ARCHIVE_ENTRY_ACL_TYPE_ACCESS
                && ((*ap).tag == ARCHIVE_ENTRY_ACL_USER_OBJ
                    || (*ap).tag == ARCHIVE_ENTRY_ACL_GROUP_OBJ
                    || (*ap).tag == ARCHIVE_ENTRY_ACL_OTHER))
            {
                if (*ap).type_0 == ARCHIVE_ENTRY_ACL_TYPE_DEFAULT
                    && flags & ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT != 0 as libc::c_int
                {
                    prefix =
                        (*::std::mem::transmute::<&[u8; 36],
                                                  &[libc::c_int; 9]>(b"d\x00\x00\x00e\x00\x00\x00f\x00\x00\x00a\x00\x00\x00u\x00\x00\x00l\x00\x00\x00t\x00\x00\x00:\x00\x00\x00\x00\x00\x00\x00")).as_ptr()
                } else {
                    prefix = NULL as *const wchar_t
                }
                r = archive_mstring_get_wcs(a, &mut (*ap).name, &mut wname);
                if r == 0 as libc::c_int {
                    if count > 0 as libc::c_int {
                        let fresh2 = wp;
                        wp = wp.offset(1);
                        *fresh2 = separator
                    }
                    if flags & ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID != 0 {
                        id = (*ap).id
                    } else {
                        id = -(1 as libc::c_int)
                    }
                    append_entry_w(
                        &mut wp,
                        prefix,
                        (*ap).type_0,
                        (*ap).tag,
                        flags,
                        wname,
                        (*ap).permset,
                        id,
                    );
                    count += 1
                } else if r < 0 as libc::c_int && errno == ENOMEM {
                    free(ws as *mut libc::c_void);
                    return 0 as *mut wchar_t;
                }
            }
        }
        ap = (*ap).next
    }
    /* Add terminating character */
    let fresh3 = wp;
    wp = wp.offset(1);
    *fresh3 = '\u{0}' as i32;
    len = wcslen(ws);
    if len as ssize_t > length - 1 as libc::c_int as libc::c_long {
        __archive_errx(
            1 as libc::c_int,
            b"Buffer overrun\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !text_len.is_null() {
        *text_len = len as ssize_t
    }
    return ws;
}
unsafe extern "C" fn append_id_w(mut wp: *mut *mut wchar_t, mut id: libc::c_int) {
    if id < 0 as libc::c_int {
        id = 0 as libc::c_int
    }
    if id > 9 as libc::c_int {
        append_id_w(wp, id / 10 as libc::c_int);
    }
    let fresh4 = *wp;
    *wp = (*wp).offset(1);
    *fresh4 =
        (*::std::mem::transmute::<&[u8; 44],
                                  &[libc::c_int; 11]>(b"0\x00\x00\x001\x00\x00\x002\x00\x00\x003\x00\x00\x004\x00\x00\x005\x00\x00\x006\x00\x00\x007\x00\x00\x008\x00\x00\x009\x00\x00\x00\x00\x00\x00\x00"))[(id
                                                                                                                                                                                                                   %
                                                                                                                                                                                                                   10
                                                                                                                                                                                                                       as
                                                                                                                                                                                                                       libc::c_int)
                                                                                                                                                                                                                  as
                                                                                                                                                                                                                  usize];
}
unsafe extern "C" fn append_entry_w(
    mut wp: *mut *mut wchar_t,
    mut prefix: *const wchar_t,
    mut type_0: libc::c_int,
    mut tag: libc::c_int,
    mut flags: libc::c_int,
    mut wname: *const wchar_t,
    mut perm: libc::c_int,
    mut id: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if !prefix.is_null() {
        wcscpy(*wp, prefix);
        *wp = (*wp).offset(wcslen(*wp) as isize)
    }
    let mut current_block_20: u64;
    match tag {
        ARCHIVE_ENTRY_ACL_USER_OBJ => {
            wname = NULL as *const wchar_t;
            id = -(1 as libc::c_int);
            if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 as libc::c_int {
                wcscpy(*wp,
                       (*::std::mem::transmute::<&[u8; 28],
                                                 &[libc::c_int; 7]>(b"o\x00\x00\x00w\x00\x00\x00n\x00\x00\x00e\x00\x00\x00r\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
                current_block_20 = 14818589718467733107;
            } else {
                current_block_20 = 4005560482714005869;
            }
        }
        ARCHIVE_ENTRY_ACL_USER => {
            current_block_20 = 4005560482714005869;
        }
        ARCHIVE_ENTRY_ACL_GROUP_OBJ => {
            wname = NULL as *const wchar_t;
            id = -(1 as libc::c_int);
            if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 as libc::c_int {
                wcscpy(*wp,
                       (*::std::mem::transmute::<&[u8; 28],
                                                 &[libc::c_int; 7]>(b"g\x00\x00\x00r\x00\x00\x00o\x00\x00\x00u\x00\x00\x00p\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
                current_block_20 = 14818589718467733107;
            } else {
                current_block_20 = 5900036613242386053;
            }
        }
        ARCHIVE_ENTRY_ACL_GROUP => {
            current_block_20 = 5900036613242386053;
        }
        ARCHIVE_ENTRY_ACL_MASK => {
            wcscpy(
                *wp,
                (*::std::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"m\x00\x00\x00a\x00\x00\x00s\x00\x00\x00k\x00\x00\x00\x00\x00\x00\x00",
                ))
                .as_ptr(),
            );
            wname = NULL as *const wchar_t;
            id = -(1 as libc::c_int);
            current_block_20 = 14818589718467733107;
        }
        ARCHIVE_ENTRY_ACL_OTHER => {
            wcscpy(*wp,
                   (*::std::mem::transmute::<&[u8; 24],
                                             &[libc::c_int; 6]>(b"o\x00\x00\x00t\x00\x00\x00h\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            wname = NULL as *const wchar_t;
            id = -(1 as libc::c_int);
            current_block_20 = 14818589718467733107;
        }
        ARCHIVE_ENTRY_ACL_EVERYONE => {
            wcscpy(*wp,
                   (*::std::mem::transmute::<&[u8; 40],
                                             &[libc::c_int; 10]>(b"e\x00\x00\x00v\x00\x00\x00e\x00\x00\x00r\x00\x00\x00y\x00\x00\x00o\x00\x00\x00n\x00\x00\x00e\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            wname = NULL as *const wchar_t;
            id = -(1 as libc::c_int);
            current_block_20 = 14818589718467733107;
        }
        _ => {
            current_block_20 = 14818589718467733107;
        }
    }
    match current_block_20 {
        4005560482714005869 =>
        /* FALLTHROUGH */
        {
            wcscpy(
                *wp,
                (*::std::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                    b"u\x00\x00\x00s\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00",
                ))
                .as_ptr(),
            );
        }
        5900036613242386053 =>
        /* FALLTHROUGH */
        {
            wcscpy(*wp,
                   (*::std::mem::transmute::<&[u8; 24],
                                             &[libc::c_int; 6]>(b"g\x00\x00\x00r\x00\x00\x00o\x00\x00\x00u\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
        }
        _ => {}
    }
    *wp = (*wp).offset(wcslen(*wp) as isize);
    let fresh5 = *wp;
    *wp = (*wp).offset(1);
    *fresh5 = ':' as i32;
    if type_0 & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 as libc::c_int
        || tag == ARCHIVE_ENTRY_ACL_USER
        || tag == ARCHIVE_ENTRY_ACL_GROUP
    {
        if !wname.is_null() {
            wcscpy(*wp, wname);
            *wp = (*wp).offset(wcslen(*wp) as isize)
        } else if tag == ARCHIVE_ENTRY_ACL_USER || tag == ARCHIVE_ENTRY_ACL_GROUP {
            append_id_w(wp, id);
            if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 == 0 as libc::c_int {
                id = -(1 as libc::c_int)
            }
        }
        /* Solaris style has no second colon after other and mask */
        if flags & ARCHIVE_ENTRY_ACL_STYLE_SOLARIS == 0 as libc::c_int
            || tag != ARCHIVE_ENTRY_ACL_OTHER && tag != ARCHIVE_ENTRY_ACL_MASK
        {
            let fresh6 = *wp;
            *wp = (*wp).offset(1);
            *fresh6 = ':' as i32
        }
    }
    if type_0 & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 as libc::c_int {
        /* POSIX.1e ACL perms */
        let fresh7 = *wp;
        *wp = (*wp).offset(1);
        *fresh7 = if perm & 0o444 as libc::c_int != 0 {
            'r' as i32
        } else {
            '-' as i32
        };
        let fresh8 = *wp;
        *wp = (*wp).offset(1);
        *fresh8 = if perm & 0o222 as libc::c_int != 0 {
            'w' as i32
        } else {
            '-' as i32
        };
        let fresh9 = *wp;
        *wp = (*wp).offset(1);
        *fresh9 = if perm & 0o111 as libc::c_int != 0 {
            'x' as i32
        } else {
            '-' as i32
        }
    } else {
        /* NFSv4 ACL perms */
        i = 0 as libc::c_int;
        while i < nfsv4_acl_perm_map_size {
            if perm & nfsv4_acl_perm_map[i as usize].perm != 0 {
                let fresh10 = *wp;
                *wp = (*wp).offset(1);
                *fresh10 = nfsv4_acl_perm_map[i as usize].wc
            } else if flags & ARCHIVE_ENTRY_ACL_STYLE_COMPACT == 0 as libc::c_int {
                let fresh11 = *wp;
                *wp = (*wp).offset(1);
                *fresh11 = '-' as i32
            }
            i += 1
        }
        let fresh12 = *wp;
        *wp = (*wp).offset(1);
        *fresh12 = ':' as i32;
        i = 0 as libc::c_int;
        while i < nfsv4_acl_flag_map_size {
            if perm & nfsv4_acl_flag_map[i as usize].perm != 0 {
                let fresh13 = *wp;
                *wp = (*wp).offset(1);
                *fresh13 = nfsv4_acl_flag_map[i as usize].wc
            } else if flags & ARCHIVE_ENTRY_ACL_STYLE_COMPACT == 0 as libc::c_int {
                let fresh14 = *wp;
                *wp = (*wp).offset(1);
                *fresh14 = '-' as i32
            }
            i += 1
        }
        let fresh15 = *wp;
        *wp = (*wp).offset(1);
        *fresh15 = ':' as i32;
        match type_0 {
            ARCHIVE_ENTRY_ACL_TYPE_ALLOW => {
                wcscpy(*wp,
                       (*::std::mem::transmute::<&[u8; 24],
                                                 &[libc::c_int; 6]>(b"a\x00\x00\x00l\x00\x00\x00l\x00\x00\x00o\x00\x00\x00w\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            }
            ARCHIVE_ENTRY_ACL_TYPE_DENY => {
                wcscpy(
                    *wp,
                    (*::std::mem::transmute::<&[u8; 20], &[libc::c_int; 5]>(
                        b"d\x00\x00\x00e\x00\x00\x00n\x00\x00\x00y\x00\x00\x00\x00\x00\x00\x00",
                    ))
                    .as_ptr(),
                );
            }
            ARCHIVE_ENTRY_ACL_TYPE_AUDIT => {
                wcscpy(*wp,
                       (*::std::mem::transmute::<&[u8; 24],
                                                 &[libc::c_int; 6]>(b"a\x00\x00\x00u\x00\x00\x00d\x00\x00\x00i\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            }
            ARCHIVE_ENTRY_ACL_TYPE_ALARM => {
                wcscpy(*wp,
                       (*::std::mem::transmute::<&[u8; 24],
                                                 &[libc::c_int; 6]>(b"a\x00\x00\x00l\x00\x00\x00a\x00\x00\x00r\x00\x00\x00m\x00\x00\x00\x00\x00\x00\x00")).as_ptr());
            }
            _ => {}
        }
        *wp = (*wp).offset(wcslen(*wp) as isize)
    }
    if id != -(1 as libc::c_int) {
        let fresh16 = *wp;
        *wp = (*wp).offset(1);
        *fresh16 = ':' as i32;
        append_id_w(wp, id);
    };
}
/*
 * Generate a text version of the ACL. The flags parameter controls
 * the type and style of the generated ACL.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_to_text_l(
    mut acl: *mut archive_acl,
    mut text_len: *mut ssize_t,
    mut flags: libc::c_int,
    mut sc: *mut archive_string_conv,
) -> *mut libc::c_char {
    let mut count: libc::c_int = 0;
    let mut length: ssize_t = 0;
    let mut len: size_t = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut separator: libc::c_char = 0;
    let mut ap: *mut archive_acl_entry = 0 as *mut archive_acl_entry;
    let mut id: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut want_type: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    want_type = archive_acl_text_want_type(acl, flags);
    /* Both NFSv4 and POSIX.1 types found */
    if want_type == 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if want_type == ARCHIVE_ENTRY_ACL_TYPE_POSIX1E {
        flags |= ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT
    }
    length = archive_acl_text_len(
        acl,
        want_type,
        flags,
        0 as libc::c_int,
        NULL as *mut archive,
        sc,
    );
    if length == 0 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_char;
    }
    if flags & ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA != 0 {
        separator = ',' as i32 as libc::c_char
    } else {
        separator = '\n' as i32 as libc::c_char
    }
    /* Now, allocate the string and actually populate it. */
    s = malloc(
        (length as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    p = s;
    if p.is_null() {
        if errno == ENOMEM {
            __archive_errx(
                1 as libc::c_int,
                b"No memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as *mut libc::c_char;
    }
    count = 0 as libc::c_int;
    if want_type & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        append_entry(
            &mut p,
            NULL as *const libc::c_char,
            ARCHIVE_ENTRY_ACL_TYPE_ACCESS,
            ARCHIVE_ENTRY_ACL_USER_OBJ,
            flags,
            NULL as *const libc::c_char,
            ((*acl).mode & 0o700 as libc::c_int as libc::c_uint) as libc::c_int,
            -(1 as libc::c_int),
        );
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = separator;
        append_entry(
            &mut p,
            NULL as *const libc::c_char,
            ARCHIVE_ENTRY_ACL_TYPE_ACCESS,
            ARCHIVE_ENTRY_ACL_GROUP_OBJ,
            flags,
            NULL as *const libc::c_char,
            ((*acl).mode & 0o70 as libc::c_int as libc::c_uint) as libc::c_int,
            -(1 as libc::c_int),
        );
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = separator;
        append_entry(
            &mut p,
            NULL as *const libc::c_char,
            ARCHIVE_ENTRY_ACL_TYPE_ACCESS,
            ARCHIVE_ENTRY_ACL_OTHER,
            flags,
            NULL as *const libc::c_char,
            ((*acl).mode & 0o7 as libc::c_int as libc::c_uint) as libc::c_int,
            -(1 as libc::c_int),
        );
        count += 3 as libc::c_int
    }
    ap = (*acl).acl_head;
    while !ap.is_null() {
        if !((*ap).type_0 & want_type == 0 as libc::c_int) {
            /*
             * Filemode-mapping ACL entries are stored exclusively in
             * ap->mode so they should not be in the list
             */
            if !((*ap).type_0 == ARCHIVE_ENTRY_ACL_TYPE_ACCESS
                && ((*ap).tag == ARCHIVE_ENTRY_ACL_USER_OBJ
                    || (*ap).tag == ARCHIVE_ENTRY_ACL_GROUP_OBJ
                    || (*ap).tag == ARCHIVE_ENTRY_ACL_OTHER))
            {
                if (*ap).type_0 == ARCHIVE_ENTRY_ACL_TYPE_DEFAULT
                    && flags & ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT != 0 as libc::c_int
                {
                    prefix = b"default:\x00" as *const u8 as *const libc::c_char
                } else {
                    prefix = NULL as *const libc::c_char
                }
                r = archive_mstring_get_mbs_l(&mut (*ap).name, &mut name, &mut len, sc);
                if r != 0 as libc::c_int {
                    free(s as *mut libc::c_void);
                    return 0 as *mut libc::c_char;
                }
                if count > 0 as libc::c_int {
                    let fresh19 = p;
                    p = p.offset(1);
                    *fresh19 = separator
                }
                if name.is_null() || flags & ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID != 0 {
                    id = (*ap).id
                } else {
                    id = -(1 as libc::c_int)
                }
                append_entry(
                    &mut p,
                    prefix,
                    (*ap).type_0,
                    (*ap).tag,
                    flags,
                    name,
                    (*ap).permset,
                    id,
                );
                count += 1
            }
        }
        ap = (*ap).next
    }
    /* Add terminating character */
    let fresh20 = p;
    p = p.offset(1);
    *fresh20 = '\u{0}' as i32 as libc::c_char;
    len = strlen(s);
    if len as ssize_t > length - 1 as libc::c_int as libc::c_long {
        __archive_errx(
            1 as libc::c_int,
            b"Buffer overrun\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !text_len.is_null() {
        *text_len = len as ssize_t
    }
    return s;
}
unsafe extern "C" fn append_id(mut p: *mut *mut libc::c_char, mut id: libc::c_int) {
    if id < 0 as libc::c_int {
        id = 0 as libc::c_int
    }
    if id > 9 as libc::c_int {
        append_id(p, id / 10 as libc::c_int);
    }
    let fresh21 = *p;
    *p = (*p).offset(1);
    *fresh21 = (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"0123456789\x00"))
        [(id % 10 as libc::c_int) as usize];
}
unsafe extern "C" fn append_entry(
    mut p: *mut *mut libc::c_char,
    mut prefix: *const libc::c_char,
    mut type_0: libc::c_int,
    mut tag: libc::c_int,
    mut flags: libc::c_int,
    mut name: *const libc::c_char,
    mut perm: libc::c_int,
    mut id: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if !prefix.is_null() {
        strcpy(*p, prefix);
        *p = (*p).offset(strlen(*p) as isize)
    }
    let mut current_block_20: u64;
    match tag {
        ARCHIVE_ENTRY_ACL_USER_OBJ => {
            name = NULL as *const libc::c_char;
            id = -(1 as libc::c_int);
            if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 as libc::c_int {
                strcpy(*p, b"owner@\x00" as *const u8 as *const libc::c_char);
                current_block_20 = 14818589718467733107;
            } else {
                current_block_20 = 9687478736510190570;
            }
        }
        ARCHIVE_ENTRY_ACL_USER => {
            current_block_20 = 9687478736510190570;
        }
        ARCHIVE_ENTRY_ACL_GROUP_OBJ => {
            name = NULL as *const libc::c_char;
            id = -(1 as libc::c_int);
            if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 as libc::c_int {
                strcpy(*p, b"group@\x00" as *const u8 as *const libc::c_char);
                current_block_20 = 14818589718467733107;
            } else {
                current_block_20 = 16880068237889820816;
            }
        }
        ARCHIVE_ENTRY_ACL_GROUP => {
            current_block_20 = 16880068237889820816;
        }
        ARCHIVE_ENTRY_ACL_MASK => {
            strcpy(*p, b"mask\x00" as *const u8 as *const libc::c_char);
            name = NULL as *const libc::c_char;
            id = -(1 as libc::c_int);
            current_block_20 = 14818589718467733107;
        }
        ARCHIVE_ENTRY_ACL_OTHER => {
            strcpy(*p, b"other\x00" as *const u8 as *const libc::c_char);
            name = NULL as *const libc::c_char;
            id = -(1 as libc::c_int);
            current_block_20 = 14818589718467733107;
        }
        ARCHIVE_ENTRY_ACL_EVERYONE => {
            strcpy(*p, b"everyone@\x00" as *const u8 as *const libc::c_char);
            name = NULL as *const libc::c_char;
            id = -(1 as libc::c_int);
            current_block_20 = 14818589718467733107;
        }
        _ => {
            current_block_20 = 14818589718467733107;
        }
    }
    match current_block_20 {
        9687478736510190570 =>
        /* FALLTHROUGH */
        {
            strcpy(*p, b"user\x00" as *const u8 as *const libc::c_char);
        }
        16880068237889820816 =>
        /* FALLTHROUGH */
        {
            strcpy(*p, b"group\x00" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    *p = (*p).offset(strlen(*p) as isize);
    let fresh22 = *p;
    *p = (*p).offset(1);
    *fresh22 = ':' as i32 as libc::c_char;
    if type_0 & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 as libc::c_int
        || tag == ARCHIVE_ENTRY_ACL_USER
        || tag == ARCHIVE_ENTRY_ACL_GROUP
    {
        if !name.is_null() {
            strcpy(*p, name);
            *p = (*p).offset(strlen(*p) as isize)
        } else if tag == ARCHIVE_ENTRY_ACL_USER || tag == ARCHIVE_ENTRY_ACL_GROUP {
            append_id(p, id);
            if type_0 & ARCHIVE_ENTRY_ACL_TYPE_NFS4 == 0 as libc::c_int {
                id = -(1 as libc::c_int)
            }
        }
        /* Solaris style has no second colon after other and mask */
        if flags & ARCHIVE_ENTRY_ACL_STYLE_SOLARIS == 0 as libc::c_int
            || tag != ARCHIVE_ENTRY_ACL_OTHER && tag != ARCHIVE_ENTRY_ACL_MASK
        {
            let fresh23 = *p;
            *p = (*p).offset(1);
            *fresh23 = ':' as i32 as libc::c_char
        }
    }
    if type_0 & ARCHIVE_ENTRY_ACL_TYPE_POSIX1E != 0 as libc::c_int {
        /* POSIX.1e ACL perms */
        let fresh24 = *p;
        *p = (*p).offset(1);
        *fresh24 = if perm & 0o444 as libc::c_int != 0 {
            'r' as i32
        } else {
            '-' as i32
        } as libc::c_char;
        let fresh25 = *p;
        *p = (*p).offset(1);
        *fresh25 = if perm & 0o222 as libc::c_int != 0 {
            'w' as i32
        } else {
            '-' as i32
        } as libc::c_char;
        let fresh26 = *p;
        *p = (*p).offset(1);
        *fresh26 = if perm & 0o111 as libc::c_int != 0 {
            'x' as i32
        } else {
            '-' as i32
        } as libc::c_char
    } else {
        /* NFSv4 ACL perms */
        i = 0 as libc::c_int;
        while i < nfsv4_acl_perm_map_size {
            if perm & nfsv4_acl_perm_map[i as usize].perm != 0 {
                let fresh27 = *p;
                *p = (*p).offset(1);
                *fresh27 = nfsv4_acl_perm_map[i as usize].c
            } else if flags & ARCHIVE_ENTRY_ACL_STYLE_COMPACT == 0 as libc::c_int {
                let fresh28 = *p;
                *p = (*p).offset(1);
                *fresh28 = '-' as i32 as libc::c_char
            }
            i += 1
        }
        let fresh29 = *p;
        *p = (*p).offset(1);
        *fresh29 = ':' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        while i < nfsv4_acl_flag_map_size {
            if perm & nfsv4_acl_flag_map[i as usize].perm != 0 {
                let fresh30 = *p;
                *p = (*p).offset(1);
                *fresh30 = nfsv4_acl_flag_map[i as usize].c
            } else if flags & ARCHIVE_ENTRY_ACL_STYLE_COMPACT == 0 as libc::c_int {
                let fresh31 = *p;
                *p = (*p).offset(1);
                *fresh31 = '-' as i32 as libc::c_char
            }
            i += 1
        }
        let fresh32 = *p;
        *p = (*p).offset(1);
        *fresh32 = ':' as i32 as libc::c_char;
        match type_0 {
            ARCHIVE_ENTRY_ACL_TYPE_ALLOW => {
                strcpy(*p, b"allow\x00" as *const u8 as *const libc::c_char);
            }
            ARCHIVE_ENTRY_ACL_TYPE_DENY => {
                strcpy(*p, b"deny\x00" as *const u8 as *const libc::c_char);
            }
            ARCHIVE_ENTRY_ACL_TYPE_AUDIT => {
                strcpy(*p, b"audit\x00" as *const u8 as *const libc::c_char);
            }
            ARCHIVE_ENTRY_ACL_TYPE_ALARM => {
                strcpy(*p, b"alarm\x00" as *const u8 as *const libc::c_char);
            }
            _ => {}
        }
        *p = (*p).offset(strlen(*p) as isize)
    }
    if id != -(1 as libc::c_int) {
        let fresh33 = *p;
        *p = (*p).offset(1);
        *fresh33 = ':' as i32 as libc::c_char;
        append_id(p, id);
    };
}
/*
 * Parse a wide ACL text string.
 *
 * The want_type argument may be one of the following:
 * ARCHIVE_ENTRY_ACL_TYPE_ACCESS - text is a POSIX.1e ACL of type ACCESS
 * ARCHIVE_ENTRY_ACL_TYPE_DEFAULT - text is a POSIX.1e ACL of type DEFAULT
 * ARCHIVE_ENTRY_ACL_TYPE_NFS4 - text is as a NFSv4 ACL
 *
 * POSIX.1e ACL entries prefixed with "default:" are treated as
 * ARCHIVE_ENTRY_ACL_TYPE_DEFAULT unless type is ARCHIVE_ENTRY_ACL_TYPE_NFS4
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_from_text_w(
    mut acl: *mut archive_acl,
    mut text: *const wchar_t,
    mut want_type: libc::c_int,
) -> libc::c_int {
    let mut field: [C2RustUnnamed_1; 6] = [C2RustUnnamed_1 {
        start: 0 as *const wchar_t,
        end: 0 as *const wchar_t,
    }; 6];
    let mut name: C2RustUnnamed_1 = C2RustUnnamed_1 {
        start: 0 as *const wchar_t,
        end: 0 as *const wchar_t,
    };
    let mut s: *const wchar_t = 0 as *const wchar_t;
    let mut st: *const wchar_t = 0 as *const wchar_t;
    let mut numfields: libc::c_int = 0;
    let mut fields: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut sol: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut types: libc::c_int = 0;
    let mut tag: libc::c_int = 0;
    let mut permset: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut sep: wchar_t = 0;
    ret = ARCHIVE_OK;
    types = 0 as libc::c_int;
    let mut current_block_6: u64;
    match want_type {
        768 => {
            want_type = ARCHIVE_ENTRY_ACL_TYPE_ACCESS;
            current_block_6 = 3527484645425325286;
        }
        ARCHIVE_ENTRY_ACL_TYPE_ACCESS | ARCHIVE_ENTRY_ACL_TYPE_DEFAULT => {
            current_block_6 = 3527484645425325286;
        }
        15360 => {
            numfields = 6 as libc::c_int;
            current_block_6 = 1856101646708284338;
        }
        _ => return -(30 as libc::c_int),
    }
    match current_block_6 {
        3527484645425325286 => numfields = 5 as libc::c_int,
        _ => {}
    }
    /* Comment, skip entry */
    while !text.is_null() && *text != '\u{0}' as i32 {
        /*
         * Parse the fields out of the next entry,
         * advance 'text' to start of next entry.
         */
        fields = 0 as libc::c_int;
        loop {
            let mut start: *const wchar_t = 0 as *const wchar_t;
            let mut end: *const wchar_t = 0 as *const wchar_t;
            next_field_w(&mut text, &mut start, &mut end, &mut sep);
            if fields < numfields {
                field[fields as usize].start = start;
                field[fields as usize].end = end
            }
            fields += 1;
            if !(sep == ':' as i32) {
                break;
            }
        }
        /* Set remaining fields to blank. */
        n = fields;
        while n < numfields {
            field[n as usize].end = NULL as *const wchar_t;
            field[n as usize].start = field[n as usize].end;
            n += 1
        }
        if !field[0 as libc::c_int as usize].start.is_null()
            && *field[0 as libc::c_int as usize].start == '#' as i32
        {
            continue;
        }
        n = 0 as libc::c_int;
        sol = 0 as libc::c_int;
        id = -(1 as libc::c_int);
        permset = 0 as libc::c_int;
        name.end = NULL as *const wchar_t;
        name.start = name.end;
        if want_type != ARCHIVE_ENTRY_ACL_TYPE_NFS4 {
            /* POSIX.1e ACLs */
            /*
             * Default keyword "default:user::rwx"
             * if found, we have one more field
             *
             * We also support old Solaris extension:
             * "defaultuser::rwx" is the default ACL corresponding
             * to "user::rwx", etc. valid only for first field
             */
            s = field[0 as libc::c_int as usize].start;
            len = field[0 as libc::c_int as usize]
                .end
                .offset_from(field[0 as libc::c_int as usize].start)
                as libc::c_long as size_t;
            if *s == 'd' as i32 &&
                   (len == 1 as libc::c_int as libc::c_ulong ||
                        len >= 7 as libc::c_int as libc::c_ulong &&
                            wmemcmp(s.offset(1 as libc::c_int as isize),
                                    (*::std::mem::transmute::<&[u8; 28],
                                                              &[libc::c_int; 7]>(b"e\x00\x00\x00f\x00\x00\x00a\x00\x00\x00u\x00\x00\x00l\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                    6 as libc::c_int as libc::c_ulong) ==
                                0 as libc::c_int) {
                type_0 = ARCHIVE_ENTRY_ACL_TYPE_DEFAULT;
                if len > 7 as libc::c_int as libc::c_ulong {
                    field[0 as libc::c_int as usize].start =
                        field[0 as libc::c_int as
                                  usize].start.offset(7 as libc::c_int as
                                                          isize)
                } else { n = 1 as libc::c_int }
            } else { type_0 = want_type }
            /* Check for a numeric ID in field n+1 or n+3. */
            isint_w(
                field[(n + 1 as libc::c_int) as usize].start,
                field[(n + 1 as libc::c_int) as usize].end,
                &mut id,
            );
            /* Field n+3 is optional. */
            if id == -(1 as libc::c_int) && fields > n + 3 as libc::c_int {
                isint_w(
                    field[(n + 3 as libc::c_int) as usize].start,
                    field[(n + 3 as libc::c_int) as usize].end,
                    &mut id,
                );
            }
            tag = 0 as libc::c_int;
            s = field[n as usize].start;
            st = field[n as usize].start.offset(1 as libc::c_int as isize);
            len = field[n as usize]
                .end
                .offset_from(field[n as usize].start) as libc::c_long
                as size_t;
            match *s {
                117 => {
                    if len == 1 as libc::c_int as libc::c_ulong ||
                           len == 4 as libc::c_int as libc::c_ulong &&
                               wmemcmp(st,
                                       (*::std::mem::transmute::<&[u8; 16],
                                                                 &[libc::c_int; 4]>(b"s\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                       3 as libc::c_int as libc::c_ulong) ==
                                   0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_USER_OBJ
                    }
                }
                103 => {
                    if len == 1 as libc::c_int as libc::c_ulong ||
                           len == 5 as libc::c_int as libc::c_ulong &&
                               wmemcmp(st,
                                       (*::std::mem::transmute::<&[u8; 20],
                                                                 &[libc::c_int; 5]>(b"r\x00\x00\x00o\x00\x00\x00u\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                       4 as libc::c_int as libc::c_ulong) ==
                                   0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_GROUP_OBJ
                    }
                }
                111 => {
                    if len == 1 as libc::c_int as libc::c_ulong ||
                           len == 5 as libc::c_int as libc::c_ulong &&
                               wmemcmp(st,
                                       (*::std::mem::transmute::<&[u8; 20],
                                                                 &[libc::c_int; 5]>(b"t\x00\x00\x00h\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                       4 as libc::c_int as libc::c_ulong) ==
                                   0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_OTHER
                    }
                }
                109 => {
                    if len == 1 as libc::c_int as libc::c_ulong ||
                           len == 4 as libc::c_int as libc::c_ulong &&
                               wmemcmp(st,
                                       (*::std::mem::transmute::<&[u8; 16],
                                                                 &[libc::c_int; 4]>(b"a\x00\x00\x00s\x00\x00\x00k\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                       3 as libc::c_int as libc::c_ulong) ==
                                   0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_MASK
                    }
                }
                _ => { }
            }
            match tag {
                ARCHIVE_ENTRY_ACL_OTHER | ARCHIVE_ENTRY_ACL_MASK => {
                    if fields == n + 2 as libc::c_int
                        && field[(n + 1 as libc::c_int) as usize].start
                            < field[(n + 1 as libc::c_int) as usize].end
                        && ismode_w(
                            field[(n + 1 as libc::c_int) as usize].start,
                            field[(n + 1 as libc::c_int) as usize].end,
                            &mut permset,
                        ) != 0
                    {
                        /* This is Solaris-style "other:rwx" */
                        sol = 1 as libc::c_int
                    } else if fields == n + 3 as libc::c_int
                        && field[(n + 1 as libc::c_int) as usize].start
                            < field[(n + 1 as libc::c_int) as usize].end
                    {
                        /* Invalid mask or other field */
                        ret = ARCHIVE_WARN;
                        continue;
                    }
                }
                ARCHIVE_ENTRY_ACL_USER_OBJ | ARCHIVE_ENTRY_ACL_GROUP_OBJ => {
                    if id != -(1 as libc::c_int)
                        || field[(n + 1 as libc::c_int) as usize].start
                            < field[(n + 1 as libc::c_int) as usize].end
                    {
                        name = field[(n + 1 as libc::c_int) as usize];
                        if tag == ARCHIVE_ENTRY_ACL_USER_OBJ {
                            tag = ARCHIVE_ENTRY_ACL_USER
                        } else {
                            tag = ARCHIVE_ENTRY_ACL_GROUP
                        }
                    }
                }
                _ => {
                    /* Invalid tag, skip entry */
                    ret = ARCHIVE_WARN;
                    continue;
                }
            }
            /*
             * Without "default:" we expect mode in field 2
             * Exception: Solaris other and mask fields
             */
            if permset == 0 as libc::c_int
                && ismode_w(
                    field[(n + 2 as libc::c_int - sol) as usize].start,
                    field[(n + 2 as libc::c_int - sol) as usize].end,
                    &mut permset,
                ) == 0
            {
                /* Invalid mode, skip entry */
                ret = ARCHIVE_WARN;
                continue;
            }
        } else {
            /* NFS4 ACLs */
            s = field[0 as libc::c_int as usize].start;
            len = field[0 as libc::c_int as usize]
                .end
                .offset_from(field[0 as libc::c_int as usize].start)
                as libc::c_long as size_t;
            tag = 0 as libc::c_int;
            match len {
                4 => {
                    if wmemcmp(s,
                               (*::std::mem::transmute::<&[u8; 20],
                                                         &[libc::c_int; 5]>(b"u\x00\x00\x00s\x00\x00\x00e\x00\x00\x00r\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                               4 as libc::c_int as libc::c_ulong) ==
                           0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_USER
                    }
                }
                5 => {
                    if wmemcmp(s,
                               (*::std::mem::transmute::<&[u8; 24],
                                                         &[libc::c_int; 6]>(b"g\x00\x00\x00r\x00\x00\x00o\x00\x00\x00u\x00\x00\x00p\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                               5 as libc::c_int as libc::c_ulong) ==
                           0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_GROUP
                    }
                }
                6 => {
                    if wmemcmp(s,
                               (*::std::mem::transmute::<&[u8; 28],
                                                         &[libc::c_int; 7]>(b"o\x00\x00\x00w\x00\x00\x00n\x00\x00\x00e\x00\x00\x00r\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                               6 as libc::c_int as libc::c_ulong) ==
                           0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_USER_OBJ
                    } else if wmemcmp(s,
                                      (*::std::mem::transmute::<&[u8; 28],
                                                                &[libc::c_int; 7]>(b"g\x00\x00\x00r\x00\x00\x00o\x00\x00\x00u\x00\x00\x00p\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                      len) == 0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_GROUP_OBJ
                    }
                }
                9 => {
                    if wmemcmp(s,
                               (*::std::mem::transmute::<&[u8; 40],
                                                         &[libc::c_int; 10]>(b"e\x00\x00\x00v\x00\x00\x00e\x00\x00\x00r\x00\x00\x00y\x00\x00\x00o\x00\x00\x00n\x00\x00\x00e\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                               9 as libc::c_int as libc::c_ulong) ==
                           0 as libc::c_int {
                        tag = ARCHIVE_ENTRY_ACL_EVERYONE
                    }
                }
                _ => { }
            }
            if tag == 0 as libc::c_int {
                /* Invalid tag, skip entry */
                ret = ARCHIVE_WARN;
                continue;
            } else {
                if tag == ARCHIVE_ENTRY_ACL_USER || tag == ARCHIVE_ENTRY_ACL_GROUP {
                    n = 1 as libc::c_int;
                    name = field[1 as libc::c_int as usize];
                    isint_w(name.start, name.end, &mut id);
                } else {
                    n = 0 as libc::c_int
                }
                if is_nfs4_perms_w(
                    field[(1 as libc::c_int + n) as usize].start,
                    field[(1 as libc::c_int + n) as usize].end,
                    &mut permset,
                ) == 0
                {
                    /* Invalid NFSv4 perms, skip entry */
                    ret = ARCHIVE_WARN;
                    continue;
                } else if is_nfs4_flags_w(
                    field[(2 as libc::c_int + n) as usize].start,
                    field[(2 as libc::c_int + n) as usize].end,
                    &mut permset,
                ) == 0
                {
                    /* Invalid NFSv4 flags, skip entry */
                    ret = ARCHIVE_WARN;
                    continue;
                } else {
                    s = field[(3 as libc::c_int + n) as usize].start;
                    len = field[(3 as libc::c_int + n) as usize]
                        .end
                        .offset_from(field[(3 as libc::c_int + n) as usize].start)
                        as libc::c_long as size_t;
                    type_0 = 0 as libc::c_int;
                    if len == 4 as libc::c_int as libc::c_ulong {
                        if wmemcmp(s,
                                   (*::std::mem::transmute::<&[u8; 20],
                                                             &[libc::c_int; 5]>(b"d\x00\x00\x00e\x00\x00\x00n\x00\x00\x00y\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                   4 as libc::c_int as libc::c_ulong) ==
                               0 as libc::c_int {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_DENY
                        }
                    } else if len == 5 as libc::c_int as libc::c_ulong {
                        if wmemcmp(s,
                                   (*::std::mem::transmute::<&[u8; 24],
                                                             &[libc::c_int; 6]>(b"a\x00\x00\x00l\x00\x00\x00l\x00\x00\x00o\x00\x00\x00w\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                   5 as libc::c_int as libc::c_ulong) ==
                               0 as libc::c_int {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_ALLOW
                        } else if wmemcmp(s,
                                          (*::std::mem::transmute::<&[u8; 24],
                                                                    &[libc::c_int; 6]>(b"a\x00\x00\x00u\x00\x00\x00d\x00\x00\x00i\x00\x00\x00t\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                          5 as libc::c_int as libc::c_ulong)
                                      == 0 as libc::c_int {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_AUDIT
                        } else if wmemcmp(s,
                                          (*::std::mem::transmute::<&[u8; 24],
                                                                    &[libc::c_int; 6]>(b"a\x00\x00\x00l\x00\x00\x00a\x00\x00\x00r\x00\x00\x00m\x00\x00\x00\x00\x00\x00\x00")).as_ptr(),
                                          5 as libc::c_int as libc::c_ulong)
                                      == 0 as libc::c_int {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_ALARM
                        }
                    }
                    if type_0 == 0 as libc::c_int {
                        /* Invalid entry type, skip entry */
                        ret = ARCHIVE_WARN;
                        continue;
                    } else {
                        isint_w(
                            field[(4 as libc::c_int + n) as usize].start,
                            field[(4 as libc::c_int + n) as usize].end,
                            &mut id,
                        );
                    }
                }
            }
        }
        /* Add entry to the internal list. */
        r = archive_acl_add_entry_w_len(
            acl,
            type_0,
            permset,
            tag,
            id,
            name.start,
            name.end.offset_from(name.start) as libc::c_long as size_t,
        );
        if r < ARCHIVE_WARN {
            return r;
        }
        if r != ARCHIVE_OK {
            ret = ARCHIVE_WARN
        }
        types |= type_0
    }
    /* Reset ACL */
    archive_acl_reset(acl, types);
    return ret;
}
/*
 * Parse a string to a positive decimal integer.  Returns true if
 * the string is non-empty and consists only of decimal digits,
 * false otherwise.
 */
unsafe extern "C" fn isint_w(
    mut start: *const wchar_t,
    mut end: *const wchar_t,
    mut result: *mut libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    if start >= end {
        return 0 as libc::c_int;
    }
    while start < end {
        if *start < '0' as i32 || *start > '9' as i32 {
            return 0 as libc::c_int;
        }
        if n > INT_MAX / 10 as libc::c_int
            || n == INT_MAX / 10 as libc::c_int && *start - '0' as i32 > INT_MAX % 10 as libc::c_int
        {
            n = INT_MAX
        } else {
            n *= 10 as libc::c_int;
            n += *start - '0' as i32
        }
        start = start.offset(1)
    }
    *result = n;
    return 1 as libc::c_int;
}
/*
 * Parse a string as a mode field.  Returns true if
 * the string is non-empty and consists only of mode characters,
 * false otherwise.
 */
unsafe extern "C" fn ismode_w(
    mut start: *const wchar_t,
    mut end: *const wchar_t,
    mut permset: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *const wchar_t = 0 as *const wchar_t;
    if start >= end {
        return 0 as libc::c_int;
    }
    p = start;
    *permset = 0 as libc::c_int;
    while p < end {
        let fresh34 = p;
        p = p.offset(1);
        match *fresh34 {
            114 | 82 => *permset |= ARCHIVE_ENTRY_ACL_READ,
            119 | 87 => *permset |= ARCHIVE_ENTRY_ACL_WRITE,
            120 | 88 => *permset |= ARCHIVE_ENTRY_ACL_EXECUTE,
            45 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
/*
 * Parse a string as a NFS4 ACL permission field.
 * Returns true if the string is non-empty and consists only of NFS4 ACL
 * permission characters, false otherwise
 */
unsafe extern "C" fn is_nfs4_perms_w(
    mut start: *const wchar_t,
    mut end: *const wchar_t,
    mut permset: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *const wchar_t = start;
    while p < end {
        let fresh35 = p;
        p = p.offset(1);
        match *fresh35 {
            114 => *permset |= ARCHIVE_ENTRY_ACL_READ_DATA,
            119 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_DATA,
            120 => *permset |= ARCHIVE_ENTRY_ACL_EXECUTE,
            112 => *permset |= ARCHIVE_ENTRY_ACL_APPEND_DATA,
            68 => *permset |= ARCHIVE_ENTRY_ACL_DELETE_CHILD,
            100 => *permset |= ARCHIVE_ENTRY_ACL_DELETE,
            97 => *permset |= ARCHIVE_ENTRY_ACL_READ_ATTRIBUTES,
            65 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_ATTRIBUTES,
            82 => *permset |= ARCHIVE_ENTRY_ACL_READ_NAMED_ATTRS,
            87 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_NAMED_ATTRS,
            99 => *permset |= ARCHIVE_ENTRY_ACL_READ_ACL,
            67 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_ACL,
            111 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_OWNER,
            115 => *permset |= ARCHIVE_ENTRY_ACL_SYNCHRONIZE,
            45 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
/*
 * Parse a string as a NFS4 ACL flags field.
 * Returns true if the string is non-empty and consists only of NFS4 ACL
 * flag characters, false otherwise
 */
unsafe extern "C" fn is_nfs4_flags_w(
    mut start: *const wchar_t,
    mut end: *const wchar_t,
    mut permset: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *const wchar_t = start;
    while p < end {
        let fresh36 = p;
        p = p.offset(1);
        match *fresh36 {
            102 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_FILE_INHERIT,
            100 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_DIRECTORY_INHERIT,
            105 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_INHERIT_ONLY,
            110 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_NO_PROPAGATE_INHERIT,
            83 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_SUCCESSFUL_ACCESS,
            70 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_FAILED_ACCESS,
            73 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_INHERITED,
            45 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
/*
 * Match "[:whitespace:]*(.*)[:whitespace:]*[:,\n]".  *wp is updated
 * to point to just after the separator.  *start points to the first
 * character of the matched text and *end just after the last
 * character of the matched identifier.  In particular *end - *start
 * is the length of the field body, not including leading or trailing
 * whitespace.
 */
unsafe extern "C" fn next_field_w(
    mut wp: *mut *const wchar_t,
    mut start: *mut *const wchar_t,
    mut end: *mut *const wchar_t,
    mut sep: *mut wchar_t,
) {
    /* Skip leading whitespace to find start of field. */
    while **wp == ' ' as i32 || **wp == '\t' as i32 || **wp == '\n' as i32 {
        *wp = (*wp).offset(1)
    }
    *start = *wp;
    /* Scan for the separator. */
    while **wp != '\u{0}' as i32
        && **wp != ',' as i32
        && **wp != ':' as i32
        && **wp != '\n' as i32
        && **wp != '#' as i32
    {
        *wp = (*wp).offset(1)
    }
    *sep = **wp;
    /* Locate end of field, trim trailing whitespace if necessary */
    if *wp == *start {
        *end = *wp
    } else {
        *end = (*wp).offset(-(1 as libc::c_int as isize));
        while **end == ' ' as i32 || **end == '\t' as i32 || **end == '\n' as i32 {
            *end = (*end).offset(-1)
        }
        *end = (*end).offset(1)
    }
    /* Handle in-field comments */
    if *sep == '#' as i32 {
        while **wp != '\u{0}' as i32 && **wp != ',' as i32 && **wp != '\n' as i32 {
            *wp = (*wp).offset(1)
        }
        *sep = **wp
    }
    /* Adjust scanner location. */
    if **wp != '\u{0}' as i32 {
        *wp = (*wp).offset(1)
    };
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
 * $FreeBSD$
 */
/* E.g., access or default */
/* E.g., user/group/other/mask */
/* r/w/x bits */
/* uid/gid for user/group */
/* uname/gname */
/* See acl_next for details. */
/*
 * ACL text parser.
 */
/* wtext */
/* type */
/*
 * Parse an ACL text string.
 *
 * The want_type argument may be one of the following:
 * ARCHIVE_ENTRY_ACL_TYPE_ACCESS - text is a POSIX.1e ACL of type ACCESS
 * ARCHIVE_ENTRY_ACL_TYPE_DEFAULT - text is a POSIX.1e ACL of type DEFAULT
 * ARCHIVE_ENTRY_ACL_TYPE_NFS4 - text is as a NFSv4 ACL
 *
 * POSIX.1e ACL entries prefixed with "default:" are treated as
 * ARCHIVE_ENTRY_ACL_TYPE_DEFAULT unless type is ARCHIVE_ENTRY_ACL_TYPE_NFS4
 */
#[no_mangle]
pub unsafe extern "C" fn archive_acl_from_text_l(
    mut acl: *mut archive_acl,
    mut text: *const libc::c_char,
    mut want_type: libc::c_int,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut field: [C2RustUnnamed_2; 6] = [C2RustUnnamed_2 {
        start: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
    }; 6];
    let mut name: C2RustUnnamed_2 = C2RustUnnamed_2 {
        start: 0 as *const libc::c_char,
        end: 0 as *const libc::c_char,
    };
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut st: *const libc::c_char = 0 as *const libc::c_char;
    let mut numfields: libc::c_int = 0;
    let mut fields: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut sol: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut types: libc::c_int = 0;
    let mut tag: libc::c_int = 0;
    let mut permset: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut len: size_t = 0;
    let mut sep: libc::c_char = 0;
    let mut current_block_4: u64;
    match want_type {
        768 => {
            want_type = ARCHIVE_ENTRY_ACL_TYPE_ACCESS;
            current_block_4 = 3326496505056763466;
        }
        ARCHIVE_ENTRY_ACL_TYPE_ACCESS | ARCHIVE_ENTRY_ACL_TYPE_DEFAULT => {
            current_block_4 = 3326496505056763466;
        }
        15360 => {
            numfields = 6 as libc::c_int;
            current_block_4 = 13536709405535804910;
        }
        _ => return -(30 as libc::c_int),
    }
    match current_block_4 {
        3326496505056763466 => numfields = 5 as libc::c_int,
        _ => {}
    }
    ret = ARCHIVE_OK;
    types = 0 as libc::c_int;
    /* Comment, skip entry */
    while !text.is_null() && *text as libc::c_int != '\u{0}' as i32 {
        /*
         * Parse the fields out of the next entry,
         * advance 'text' to start of next entry.
         */
        fields = 0 as libc::c_int;
        loop {
            let mut start: *const libc::c_char = 0 as *const libc::c_char;
            let mut end: *const libc::c_char = 0 as *const libc::c_char;
            next_field(&mut text, &mut start, &mut end, &mut sep);
            if fields < numfields {
                field[fields as usize].start = start;
                field[fields as usize].end = end
            }
            fields += 1;
            if !(sep as libc::c_int == ':' as i32) {
                break;
            }
        }
        /* Set remaining fields to blank. */
        n = fields;
        while n < numfields {
            field[n as usize].end = NULL as *const libc::c_char;
            field[n as usize].start = field[n as usize].end;
            n += 1
        }
        if !field[0 as libc::c_int as usize].start.is_null()
            && *field[0 as libc::c_int as usize].start as libc::c_int == '#' as i32
        {
            continue;
        }
        n = 0 as libc::c_int;
        sol = 0 as libc::c_int;
        id = -(1 as libc::c_int);
        permset = 0 as libc::c_int;
        name.end = NULL as *const libc::c_char;
        name.start = name.end;
        if want_type != ARCHIVE_ENTRY_ACL_TYPE_NFS4 {
            /* POSIX.1e ACLs */
            /*
             * Default keyword "default:user::rwx"
             * if found, we have one more field
             *
             * We also support old Solaris extension:
             * "defaultuser::rwx" is the default ACL corresponding
             * to "user::rwx", etc. valid only for first field
             */
            s = field[0 as libc::c_int as usize].start;
            len = field[0 as libc::c_int as usize]
                .end
                .offset_from(field[0 as libc::c_int as usize].start)
                as libc::c_long as size_t;
            if *s as libc::c_int == 'd' as i32
                && (len == 1 as libc::c_int as libc::c_ulong
                    || len >= 7 as libc::c_int as libc::c_ulong
                        && memcmp(
                            s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            b"efault\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int)
            {
                type_0 = ARCHIVE_ENTRY_ACL_TYPE_DEFAULT;
                if len > 7 as libc::c_int as libc::c_ulong {
                    field[0 as libc::c_int as usize].start = field[0 as libc::c_int as usize]
                        .start
                        .offset(7 as libc::c_int as isize)
                } else {
                    n = 1 as libc::c_int
                }
            } else {
                type_0 = want_type
            }
            /* Check for a numeric ID in field n+1 or n+3. */
            isint(
                field[(n + 1 as libc::c_int) as usize].start,
                field[(n + 1 as libc::c_int) as usize].end,
                &mut id,
            );
            /* Field n+3 is optional. */
            if id == -(1 as libc::c_int) && fields > n + 3 as libc::c_int {
                isint(
                    field[(n + 3 as libc::c_int) as usize].start,
                    field[(n + 3 as libc::c_int) as usize].end,
                    &mut id,
                );
            }
            tag = 0 as libc::c_int;
            s = field[n as usize].start;
            st = field[n as usize].start.offset(1 as libc::c_int as isize);
            len = field[n as usize]
                .end
                .offset_from(field[n as usize].start) as libc::c_long
                as size_t;
            if len == 0 as libc::c_int as libc::c_ulong {
                ret = ARCHIVE_WARN;
                continue;
            } else {
                match *s as libc::c_int {
                    117 => {
                        if len == 1 as libc::c_int as libc::c_ulong
                            || len == 4 as libc::c_int as libc::c_ulong
                                && memcmp(
                                    st as *const libc::c_void,
                                    b"ser\x00" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    3 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                        {
                            tag = ARCHIVE_ENTRY_ACL_USER_OBJ
                        }
                    }
                    103 => {
                        if len == 1 as libc::c_int as libc::c_ulong
                            || len == 5 as libc::c_int as libc::c_ulong
                                && memcmp(
                                    st as *const libc::c_void,
                                    b"roup\x00" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    4 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                        {
                            tag = ARCHIVE_ENTRY_ACL_GROUP_OBJ
                        }
                    }
                    111 => {
                        if len == 1 as libc::c_int as libc::c_ulong
                            || len == 5 as libc::c_int as libc::c_ulong
                                && memcmp(
                                    st as *const libc::c_void,
                                    b"ther\x00" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    4 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                        {
                            tag = ARCHIVE_ENTRY_ACL_OTHER
                        }
                    }
                    109 => {
                        if len == 1 as libc::c_int as libc::c_ulong
                            || len == 4 as libc::c_int as libc::c_ulong
                                && memcmp(
                                    st as *const libc::c_void,
                                    b"ask\x00" as *const u8 as *const libc::c_char
                                        as *const libc::c_void,
                                    3 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                        {
                            tag = ARCHIVE_ENTRY_ACL_MASK
                        }
                    }
                    _ => {}
                }
                match tag {
                    ARCHIVE_ENTRY_ACL_OTHER | ARCHIVE_ENTRY_ACL_MASK => {
                        if fields == n + 2 as libc::c_int
                            && field[(n + 1 as libc::c_int) as usize].start
                                < field[(n + 1 as libc::c_int) as usize].end
                            && ismode(
                                field[(n + 1 as libc::c_int) as usize].start,
                                field[(n + 1 as libc::c_int) as usize].end,
                                &mut permset,
                            ) != 0
                        {
                            /* This is Solaris-style "other:rwx" */
                            sol = 1 as libc::c_int
                        } else if fields == n + 3 as libc::c_int
                            && field[(n + 1 as libc::c_int) as usize].start
                                < field[(n + 1 as libc::c_int) as usize].end
                        {
                            /* Invalid mask or other field */
                            ret = ARCHIVE_WARN;
                            continue;
                        }
                    }
                    ARCHIVE_ENTRY_ACL_USER_OBJ | ARCHIVE_ENTRY_ACL_GROUP_OBJ => {
                        if id != -(1 as libc::c_int)
                            || field[(n + 1 as libc::c_int) as usize].start
                                < field[(n + 1 as libc::c_int) as usize].end
                        {
                            name = field[(n + 1 as libc::c_int) as usize];
                            if tag == ARCHIVE_ENTRY_ACL_USER_OBJ {
                                tag = ARCHIVE_ENTRY_ACL_USER
                            } else {
                                tag = ARCHIVE_ENTRY_ACL_GROUP
                            }
                        }
                    }
                    _ => {
                        /* Invalid tag, skip entry */
                        ret = ARCHIVE_WARN;
                        continue;
                    }
                }
                /*
                 * Without "default:" we expect mode in field 3
                 * Exception: Solaris other and mask fields
                 */
                if permset == 0 as libc::c_int
                    && ismode(
                        field[(n + 2 as libc::c_int - sol) as usize].start,
                        field[(n + 2 as libc::c_int - sol) as usize].end,
                        &mut permset,
                    ) == 0
                {
                    /* Invalid mode, skip entry */
                    ret = ARCHIVE_WARN;
                    continue;
                }
            }
        } else {
            /* NFS4 ACLs */
            s = field[0 as libc::c_int as usize].start;
            len = field[0 as libc::c_int as usize]
                .end
                .offset_from(field[0 as libc::c_int as usize].start)
                as libc::c_long as size_t;
            tag = 0 as libc::c_int;
            match len {
                4 => {
                    if memcmp(
                        s as *const libc::c_void,
                        b"user\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        tag = ARCHIVE_ENTRY_ACL_USER
                    }
                }
                5 => {
                    if memcmp(
                        s as *const libc::c_void,
                        b"group\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        tag = ARCHIVE_ENTRY_ACL_GROUP
                    }
                }
                6 => {
                    if memcmp(
                        s as *const libc::c_void,
                        b"owner@\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        tag = ARCHIVE_ENTRY_ACL_USER_OBJ
                    } else if memcmp(
                        s as *const libc::c_void,
                        b"group@\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        tag = ARCHIVE_ENTRY_ACL_GROUP_OBJ
                    }
                }
                9 => {
                    if memcmp(
                        s as *const libc::c_void,
                        b"everyone@\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        9 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        tag = ARCHIVE_ENTRY_ACL_EVERYONE
                    }
                }
                _ => {}
            }
            if tag == 0 as libc::c_int {
                /* Invalid tag, skip entry */
                ret = ARCHIVE_WARN;
                continue;
            } else {
                if tag == ARCHIVE_ENTRY_ACL_USER || tag == ARCHIVE_ENTRY_ACL_GROUP {
                    n = 1 as libc::c_int;
                    name = field[1 as libc::c_int as usize];
                    isint(name.start, name.end, &mut id);
                } else {
                    n = 0 as libc::c_int
                }
                if is_nfs4_perms(
                    field[(1 as libc::c_int + n) as usize].start,
                    field[(1 as libc::c_int + n) as usize].end,
                    &mut permset,
                ) == 0
                {
                    /* Invalid NFSv4 perms, skip entry */
                    ret = ARCHIVE_WARN;
                    continue;
                } else if is_nfs4_flags(
                    field[(2 as libc::c_int + n) as usize].start,
                    field[(2 as libc::c_int + n) as usize].end,
                    &mut permset,
                ) == 0
                {
                    /* Invalid NFSv4 flags, skip entry */
                    ret = ARCHIVE_WARN;
                    continue;
                } else {
                    s = field[(3 as libc::c_int + n) as usize].start;
                    len = field[(3 as libc::c_int + n) as usize]
                        .end
                        .offset_from(field[(3 as libc::c_int + n) as usize].start)
                        as libc::c_long as size_t;
                    type_0 = 0 as libc::c_int;
                    if len == 4 as libc::c_int as libc::c_ulong {
                        if memcmp(
                            s as *const libc::c_void,
                            b"deny\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            4 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_DENY
                        }
                    } else if len == 5 as libc::c_int as libc::c_ulong {
                        if memcmp(
                            s as *const libc::c_void,
                            b"allow\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            5 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_ALLOW
                        } else if memcmp(
                            s as *const libc::c_void,
                            b"audit\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            5 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_AUDIT
                        } else if memcmp(
                            s as *const libc::c_void,
                            b"alarm\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            5 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            type_0 = ARCHIVE_ENTRY_ACL_TYPE_ALARM
                        }
                    }
                    if type_0 == 0 as libc::c_int {
                        /* Invalid entry type, skip entry */
                        ret = ARCHIVE_WARN;
                        continue;
                    } else {
                        isint(
                            field[(4 as libc::c_int + n) as usize].start,
                            field[(4 as libc::c_int + n) as usize].end,
                            &mut id,
                        );
                    }
                }
            }
        }
        /* Add entry to the internal list. */
        r = archive_acl_add_entry_len_l(
            acl,
            type_0,
            permset,
            tag,
            id,
            name.start,
            name.end.offset_from(name.start) as libc::c_long as size_t,
            sc,
        );
        if r < ARCHIVE_WARN {
            return r;
        }
        if r != ARCHIVE_OK {
            ret = ARCHIVE_WARN
        }
        types |= type_0
    }
    /* Reset ACL */
    archive_acl_reset(acl, types);
    return ret;
}
/*
 * Parse a string to a positive decimal integer.  Returns true if
 * the string is non-empty and consists only of decimal digits,
 * false otherwise.
 */
unsafe extern "C" fn isint(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut result: *mut libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    if start >= end {
        return 0 as libc::c_int;
    }
    while start < end {
        if (*start as libc::c_int) < '0' as i32 || *start as libc::c_int > '9' as i32 {
            return 0 as libc::c_int;
        }
        if n > INT_MAX / 10 as libc::c_int
            || n == INT_MAX / 10 as libc::c_int
                && *start as libc::c_int - '0' as i32 > INT_MAX % 10 as libc::c_int
        {
            n = INT_MAX
        } else {
            n *= 10 as libc::c_int;
            n += *start as libc::c_int - '0' as i32
        }
        start = start.offset(1)
    }
    *result = n;
    return 1 as libc::c_int;
}
/*
 * Parse a string as a mode field.  Returns true if
 * the string is non-empty and consists only of mode characters,
 * false otherwise.
 */
unsafe extern "C" fn ismode(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut permset: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if start >= end {
        return 0 as libc::c_int;
    }
    p = start;
    *permset = 0 as libc::c_int;
    while p < end {
        let fresh37 = p;
        p = p.offset(1);
        match *fresh37 as libc::c_int {
            114 | 82 => *permset |= ARCHIVE_ENTRY_ACL_READ,
            119 | 87 => *permset |= ARCHIVE_ENTRY_ACL_WRITE,
            120 | 88 => *permset |= ARCHIVE_ENTRY_ACL_EXECUTE,
            45 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
/*
 * Parse a string as a NFS4 ACL permission field.
 * Returns true if the string is non-empty and consists only of NFS4 ACL
 * permission characters, false otherwise
 */
unsafe extern "C" fn is_nfs4_perms(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut permset: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = start;
    while p < end {
        let fresh38 = p;
        p = p.offset(1);
        match *fresh38 as libc::c_int {
            114 => *permset |= ARCHIVE_ENTRY_ACL_READ_DATA,
            119 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_DATA,
            120 => *permset |= ARCHIVE_ENTRY_ACL_EXECUTE,
            112 => *permset |= ARCHIVE_ENTRY_ACL_APPEND_DATA,
            68 => *permset |= ARCHIVE_ENTRY_ACL_DELETE_CHILD,
            100 => *permset |= ARCHIVE_ENTRY_ACL_DELETE,
            97 => *permset |= ARCHIVE_ENTRY_ACL_READ_ATTRIBUTES,
            65 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_ATTRIBUTES,
            82 => *permset |= ARCHIVE_ENTRY_ACL_READ_NAMED_ATTRS,
            87 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_NAMED_ATTRS,
            99 => *permset |= ARCHIVE_ENTRY_ACL_READ_ACL,
            67 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_ACL,
            111 => *permset |= ARCHIVE_ENTRY_ACL_WRITE_OWNER,
            115 => *permset |= ARCHIVE_ENTRY_ACL_SYNCHRONIZE,
            45 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
/*
 * Parse a string as a NFS4 ACL flags field.
 * Returns true if the string is non-empty and consists only of NFS4 ACL
 * flag characters, false otherwise
 */
unsafe extern "C" fn is_nfs4_flags(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    mut permset: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = start;
    while p < end {
        let fresh39 = p;
        p = p.offset(1);
        match *fresh39 as libc::c_int {
            102 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_FILE_INHERIT,
            100 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_DIRECTORY_INHERIT,
            105 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_INHERIT_ONLY,
            110 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_NO_PROPAGATE_INHERIT,
            83 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_SUCCESSFUL_ACCESS,
            70 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_FAILED_ACCESS,
            73 => *permset |= ARCHIVE_ENTRY_ACL_ENTRY_INHERITED,
            45 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
/*
 * Match "[:whitespace:]*(.*)[:whitespace:]*[:,\n]".  *wp is updated
 * to point to just after the separator.  *start points to the first
 * character of the matched text and *end just after the last
 * character of the matched identifier.  In particular *end - *start
 * is the length of the field body, not including leading or trailing
 * whitespace.
 */
unsafe extern "C" fn next_field(
    mut p: *mut *const libc::c_char,
    mut start: *mut *const libc::c_char,
    mut end: *mut *const libc::c_char,
    mut sep: *mut libc::c_char,
) {
    /* Skip leading whitespace to find start of field. */
    while **p as libc::c_int == ' ' as i32
        || **p as libc::c_int == '\t' as i32
        || **p as libc::c_int == '\n' as i32
    {
        *p = (*p).offset(1)
    }
    *start = *p;
    /* Scan for the separator. */
    while **p as libc::c_int != '\u{0}' as i32
        && **p as libc::c_int != ',' as i32
        && **p as libc::c_int != ':' as i32
        && **p as libc::c_int != '\n' as i32
        && **p as libc::c_int != '#' as i32
    {
        *p = (*p).offset(1)
    }
    *sep = **p;
    /* Locate end of field, trim trailing whitespace if necessary */
    if *p == *start {
        *end = *p
    } else {
        *end = (*p).offset(-(1 as libc::c_int as isize));
        while **end as libc::c_int == ' ' as i32
            || **end as libc::c_int == '\t' as i32
            || **end as libc::c_int == '\n' as i32
        {
            *end = (*end).offset(-1)
        }
        *end = (*end).offset(1)
    }
    /* Handle in-field comments */
    if *sep as libc::c_int == '#' as i32 {
        while **p as libc::c_int != '\u{0}' as i32
            && **p as libc::c_int != ',' as i32
            && **p as libc::c_int != '\n' as i32
        {
            *p = (*p).offset(1)
        }
        *sep = **p
    }
    /* Adjust scanner location. */
    if **p as libc::c_int != '\u{0}' as i32 {
        *p = (*p).offset(1)
    };
}
unsafe extern "C" fn run_static_initializers() {
    nfsv4_acl_perm_map_size = (::std::mem::size_of::<[C2RustUnnamed_0; 14]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
        as libc::c_int;
    nfsv4_acl_flag_map_size = (::std::mem::size_of::<[C2RustUnnamed; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
