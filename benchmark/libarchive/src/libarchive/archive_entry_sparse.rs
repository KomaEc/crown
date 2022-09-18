use ::libc;
extern "C" {
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
    pub type archive_string_conv;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
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
/* uname/gname */
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
pub const INT64_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * sparse handling
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_sparse_clear(mut entry: *mut archive_entry) {
    let mut sp: *mut ae_sparse = 0 as *mut ae_sparse;
    while !(*entry).sparse_head.is_null() {
        sp = (*(*entry).sparse_head).next;
        free((*entry).sparse_head as *mut libc::c_void);
        (*entry).sparse_head = sp
    }
    (*entry).sparse_tail = NULL as *mut ae_sparse;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_sparse_add_entry(
    mut entry: *mut archive_entry,
    mut offset: la_int64_t,
    mut length: la_int64_t,
) {
    let mut sp: *mut ae_sparse = 0 as *mut ae_sparse;
    if offset < 0 as libc::c_int as libc::c_long || length < 0 as libc::c_int as libc::c_long {
        /* Invalid value */
        return;
    }
    if offset > INT64_MAX - length || offset + length > archive_entry_size(entry) {
        /* A value of "length" parameter is too large. */
        return;
    }
    sp = (*entry).sparse_tail;
    if !sp.is_null() {
        if (*sp).offset + (*sp).length > offset {
            /* Invalid value. */
            return;
        }
        if (*sp).offset + (*sp).length == offset {
            if (*sp).offset + (*sp).length + length < 0 as libc::c_int as libc::c_long {
                /* A value of "length" parameter is
                 * too large. */
                return;
            }
            /* Expand existing sparse block size. */
            (*sp).length += length;
            return;
        }
    }
    sp = malloc(::std::mem::size_of::<ae_sparse>() as libc::c_ulong) as *mut ae_sparse;
    if sp.is_null() {
        /* XXX Error XXX */
        return;
    }
    (*sp).offset = offset;
    (*sp).length = length;
    (*sp).next = NULL as *mut ae_sparse;
    if (*entry).sparse_head.is_null() {
        (*entry).sparse_tail = sp;
        (*entry).sparse_head = (*entry).sparse_tail
    } else {
        /* Add a new sparse block to the tail of list. */
        if !(*entry).sparse_tail.is_null() {
            (*(*entry).sparse_tail).next = sp
        }
        (*entry).sparse_tail = sp
    };
}
/*
 * returns number of the sparse entries
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_sparse_count(mut entry: *mut archive_entry) -> libc::c_int {
    let mut sp: *mut ae_sparse = 0 as *mut ae_sparse;
    let mut count: libc::c_int = 0 as libc::c_int;
    sp = (*entry).sparse_head;
    while !sp.is_null() {
        count += 1;
        sp = (*sp).next
    }
    /*
     * Sanity check if this entry is exactly sparse.
     * If amount of sparse blocks is just one and it indicates the whole
     * file data, we should remove it and return zero.
     */
    if count == 1 as libc::c_int {
        sp = (*entry).sparse_head;
        if (*sp).offset == 0 as libc::c_int as libc::c_long
            && (*sp).length >= archive_entry_size(entry)
        {
            count = 0 as libc::c_int;
            archive_entry_sparse_clear(entry);
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_sparse_reset(mut entry: *mut archive_entry) -> libc::c_int {
    (*entry).sparse_p = (*entry).sparse_head;
    return archive_entry_sparse_count(entry);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_sparse_next(
    mut entry: *mut archive_entry,
    mut offset: *mut la_int64_t,
    mut length: *mut la_int64_t,
) -> libc::c_int {
    if !(*entry).sparse_p.is_null() {
        *offset = (*(*entry).sparse_p).offset;
        *length = (*(*entry).sparse_p).length;
        (*entry).sparse_p = (*(*entry).sparse_p).next;
        return 0 as libc::c_int;
    } else {
        *offset = 0 as libc::c_int as la_int64_t;
        *length = 0 as libc::c_int as la_int64_t;
        return -(20 as libc::c_int);
    };
}
/*
 * end of sparse handling
 */
