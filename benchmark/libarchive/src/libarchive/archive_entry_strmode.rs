use ::libc;
extern "C" {
    /* Declare our basic types. */
    pub type archive;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_acl_types(_: *mut archive_entry) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
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
pub const S_ISUID: libc::c_int = 0o4000 as libc::c_int;
pub const S_ISGID: libc::c_int = 0o2000 as libc::c_int;
pub const S_ISVTX: libc::c_int = 0o1000 as libc::c_int;
pub const __S_ISUID: libc::c_int = 0o4000 as libc::c_int;
pub const __S_ISGID: libc::c_int = 0o2000 as libc::c_int;
pub const __S_ISVTX: libc::c_int = 0o1000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
pub const AE_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_entry_strmode(
    mut entry: *mut archive_entry,
) -> *const libc::c_char {
    static mut permbits: [mode_t; 9] = [
        0o400 as libc::c_int as mode_t,
        0o200 as libc::c_int as mode_t,
        0o100 as libc::c_int as mode_t,
        0o40 as libc::c_int as mode_t,
        0o20 as libc::c_int as mode_t,
        0o10 as libc::c_int as mode_t,
        0o4 as libc::c_int as mode_t,
        0o2 as libc::c_int as mode_t,
        0o1 as libc::c_int as mode_t,
    ];
    let mut bp: *mut libc::c_char = (*entry).strmode.as_mut_ptr();
    let mut mode: mode_t = 0;
    let mut i: libc::c_int = 0;
    /* Fill in a default string, then selectively override. */
    strcpy(bp, b"?rwxrwxrwx \x00" as *const u8 as *const libc::c_char);
    mode = archive_entry_mode(entry);
    match archive_entry_filetype(entry) {
        32768 => *bp.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char,
        24576 => *bp.offset(0 as libc::c_int as isize) = 'b' as i32 as libc::c_char,
        8192 => *bp.offset(0 as libc::c_int as isize) = 'c' as i32 as libc::c_char,
        16384 => *bp.offset(0 as libc::c_int as isize) = 'd' as i32 as libc::c_char,
        40960 => *bp.offset(0 as libc::c_int as isize) = 'l' as i32 as libc::c_char,
        49152 => *bp.offset(0 as libc::c_int as isize) = 's' as i32 as libc::c_char,
        4096 => *bp.offset(0 as libc::c_int as isize) = 'p' as i32 as libc::c_char,
        _ => {
            if !archive_entry_hardlink(entry).is_null() {
                *bp.offset(0 as libc::c_int as isize) = 'h' as i32 as libc::c_char
            }
        }
    }
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if mode & permbits[i as usize] == 0 {
            *bp.offset((i + 1 as libc::c_int) as isize) = '-' as i32 as libc::c_char
        }
        i += 1
    }
    if mode & S_ISUID as libc::c_uint != 0 {
        if mode & 0o100 as libc::c_int as libc::c_uint != 0 {
            *bp.offset(3 as libc::c_int as isize) = 's' as i32 as libc::c_char
        } else {
            *bp.offset(3 as libc::c_int as isize) = 'S' as i32 as libc::c_char
        }
    }
    if mode & S_ISGID as libc::c_uint != 0 {
        if mode & 0o10 as libc::c_int as libc::c_uint != 0 {
            *bp.offset(6 as libc::c_int as isize) = 's' as i32 as libc::c_char
        } else {
            *bp.offset(6 as libc::c_int as isize) = 'S' as i32 as libc::c_char
        }
    }
    if mode & S_ISVTX as libc::c_uint != 0 {
        if mode & 0o1 as libc::c_int as libc::c_uint != 0 {
            *bp.offset(9 as libc::c_int as isize) = 't' as i32 as libc::c_char
        } else {
            *bp.offset(9 as libc::c_int as isize) = 'T' as i32 as libc::c_char
        }
    }
    if archive_entry_acl_types(entry) != 0 as libc::c_int {
        *bp.offset(10 as libc::c_int as isize) = '+' as i32 as libc::c_char
    }
    return bp;
}
