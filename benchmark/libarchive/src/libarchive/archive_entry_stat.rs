use ::libc;
extern "C" {
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
    /* Declare our basic types. */
    pub type archive;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_dev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_ino64(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_rdev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
}
pub type __uint32_t = libc::c_uint;
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
pub type __syscall_slong_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type ino_t = __ino_t;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type uid_t = __uid_t;
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
pub type wchar_t = libc::c_int;
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
pub type la_int64_t = int64_t;
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
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
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
/* Pointer to the storage */
/* Length of 's' in characters */
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
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * Routines to bulk copy fields to/from a platform-native "struct
 * stat."  Libarchive used to just store a struct stat inside of each
 * archive_entry object, but this created issues when trying to
 * manipulate archives on systems different than the ones they were
 * created on.
 *
 * TODO: On Linux and other LFS systems, provide both stat32 and
 * stat64 versions of these functions and all of the macro glue so
 * that archive_entry_stat is magically defined to
 * archive_entry_stat32 or archive_entry_stat64 as appropriate.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_entry_stat(mut entry: *mut archive_entry) -> *const stat {
    let mut st: *mut stat = 0 as *mut stat;
    if (*entry).stat.is_null() {
        (*entry).stat = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<stat>() as libc::c_ulong,
        );
        if (*entry).stat.is_null() {
            return 0 as *const stat;
        }
        (*entry).stat_valid = 0 as libc::c_int
    }
    /*
     * If none of the underlying fields have been changed, we
     * don't need to regenerate.  In theory, we could use a bitmap
     * here to flag only those items that have changed, but the
     * extra complexity probably isn't worth it.  It will be very
     * rare for anyone to change just one field then request a new
     * stat structure.
     */
    if (*entry).stat_valid != 0 {
        return (*entry).stat as *const stat;
    }
    st = (*entry).stat as *mut stat;
    /*
     * Use the public interfaces to extract items, so that
     * the appropriate conversions get invoked.
     */
    (*st).st_atim.tv_sec = archive_entry_atime(entry);
    (*st).st_ctim.tv_sec = archive_entry_ctime(entry);
    (*st).st_mtim.tv_sec = archive_entry_mtime(entry);
    (*st).st_dev = archive_entry_dev(entry);
    (*st).st_gid = archive_entry_gid(entry) as gid_t;
    (*st).st_uid = archive_entry_uid(entry) as uid_t;
    (*st).st_ino = archive_entry_ino64(entry) as ino_t;
    (*st).st_nlink = archive_entry_nlink(entry) as __nlink_t;
    (*st).st_rdev = archive_entry_rdev(entry);
    (*st).st_size = archive_entry_size(entry);
    (*st).st_mode = archive_entry_mode(entry);
    /*
     * On systems that support high-res timestamps, copy that
     * information into struct stat.
     */
    (*st).st_atim.tv_nsec = archive_entry_atime_nsec(entry);
    (*st).st_ctim.tv_nsec = archive_entry_ctime_nsec(entry);
    (*st).st_mtim.tv_nsec = archive_entry_mtime_nsec(entry);
    /*
     * TODO: On Linux, store 32 or 64 here depending on whether
     * the cached stat structure is a stat32 or a stat64.  This
     * will allow us to support both variants interchangeably.
     */
    (*entry).stat_valid = 1 as libc::c_int;
    return st;
}
