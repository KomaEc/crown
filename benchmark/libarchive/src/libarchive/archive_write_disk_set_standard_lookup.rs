use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn getgrnam_r(
        __name: *const libc::c_char,
        __resultbuf: *mut group,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> libc::c_int;
    #[no_mangle]
    fn getpwnam_r(
        __name: *const libc::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_write_disk_set_user_lookup(
        _: *mut archive,
        _: *mut libc::c_void,
        _: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *const libc::c_char,
                _: la_int64_t,
            ) -> la_int64_t,
        >,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_write_disk_set_group_lookup(
        _: *mut archive,
        _: *mut libc::c_void,
        _: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: *const libc::c_char,
                _: la_int64_t,
            ) -> la_int64_t,
        >,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __id_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type id_t = __id_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
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
 * $FreeBSD: head/lib/libarchive/archive_private.h 201098 2009-12-28 02:58:14Z kientzle $
 */
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
pub struct bucket {
    pub name: *mut libc::c_char,
    pub hash: libc::c_int,
    pub id: id_t,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ERANGE: libc::c_int = 34 as libc::c_int;
static mut cache_size: size_t = 127 as libc::c_int as size_t;
/*
 * Installs functions that use getpwnam()/getgrnam()---along with
 * a simple cache to accelerate such lookups---into the archive_write_disk
 * object.  This is in a separate file because getpwnam()/getgrnam()
 * can pull in a LOT of library code (including NIS/LDAP functions, which
 * pull in DNS resolvers, etc).  This can easily top 500kB, which makes
 * it inappropriate for some space-constrained applications.
 *
 * Applications that are size-sensitive may want to just use the
 * real default functions (defined in archive_write_disk.c) that just
 * use the uid/gid without the lookup.  Or define your own custom functions
 * if you prefer.
 *
 * TODO: Replace these hash tables with simpler move-to-front LRU
 * lists with a bounded size (128 items?).  The hash is a bit faster,
 * but has a bad pathology in which it thrashes a single bucket.  Even
 * walking a list of 128 items is a lot faster than calling
 * getpwnam()!
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_disk_set_standard_lookup(
    mut a: *mut archive,
) -> libc::c_int {
    let mut ucache: *mut bucket =
        calloc(cache_size, ::std::mem::size_of::<bucket>() as libc::c_ulong) as *mut bucket;
    let mut gcache: *mut bucket =
        calloc(cache_size, ::std::mem::size_of::<bucket>() as libc::c_ulong) as *mut bucket;
    if ucache.is_null() || gcache.is_null() {
        free(ucache as *mut libc::c_void);
        free(gcache as *mut libc::c_void);
        return -(30 as libc::c_int);
    }
    archive_write_disk_set_group_lookup(
        a,
        gcache as *mut libc::c_void,
        Some(
            lookup_gid
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const libc::c_char,
                    _: int64_t,
                ) -> int64_t,
        ),
        Some(cleanup as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    archive_write_disk_set_user_lookup(
        a,
        ucache as *mut libc::c_void,
        Some(
            lookup_uid
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const libc::c_char,
                    _: int64_t,
                ) -> int64_t,
        ),
        Some(cleanup as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn lookup_gid(
    mut private_data: *mut libc::c_void,
    mut gname: *const libc::c_char,
    mut gid: int64_t,
) -> int64_t {
    let mut h: libc::c_int = 0;
    let mut b: *mut bucket = 0 as *mut bucket;
    let mut gcache: *mut bucket = private_data as *mut bucket;
    /* If no gname, just use the gid provided. */
    if gname.is_null() || *gname as libc::c_int == '\u{0}' as i32 {
        return gid;
    }
    /* Try to find gname in the cache. */
    h = hash(gname) as libc::c_int;
    b = &mut *gcache.offset((h as libc::c_ulong).wrapping_rem(cache_size) as isize) as *mut bucket;
    if !(*b).name.is_null() && (*b).hash == h && strcmp(gname, (*b).name) == 0 as libc::c_int {
        return (*b).id as int64_t;
    }
    /* Free the cache slot for a new entry. */
    free((*b).name as *mut libc::c_void);
    (*b).name = strdup(gname);
    /* Note: If strdup fails, that's okay; we just won't cache. */
    (*b).hash = h; /* Old getgrnam_r ignores last arg. */
    let mut _buffer: [libc::c_char; 128] = [0; 128];
    let mut bufsize: size_t = 128 as libc::c_int as size_t;
    let mut buffer: *mut libc::c_char = _buffer.as_mut_ptr();
    let mut allocated: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut grent: group = group {
        gr_name: 0 as *mut libc::c_char,
        gr_passwd: 0 as *mut libc::c_char,
        gr_gid: 0,
        gr_mem: 0 as *mut *mut libc::c_char,
    };
    let mut result: *mut group = 0 as *mut group;
    let mut r: libc::c_int = 0;
    loop {
        result = &mut grent;
        r = getgrnam_r(gname, &mut grent, buffer, bufsize, &mut result);
        if r == 0 as libc::c_int {
            break;
        }
        if r != ERANGE {
            break;
        }
        bufsize = (bufsize as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        free(allocated as *mut libc::c_void);
        allocated = malloc(bufsize) as *mut libc::c_char;
        if allocated.is_null() {
            break;
        }
        buffer = allocated
    }
    if !result.is_null() {
        gid = (*result).gr_gid as int64_t
    }
    free(allocated as *mut libc::c_void);
    /* HAVE_GETGRNAM_R */
    /* HAVE_GETGRNAM_R */
    (*b).id = gid as gid_t;
    return gid;
}
unsafe extern "C" fn lookup_uid(
    mut private_data: *mut libc::c_void,
    mut uname: *const libc::c_char,
    mut uid: int64_t,
) -> int64_t {
    let mut h: libc::c_int = 0;
    let mut b: *mut bucket = 0 as *mut bucket;
    let mut ucache: *mut bucket = private_data as *mut bucket;
    /* If no uname, just use the uid provided. */
    if uname.is_null() || *uname as libc::c_int == '\u{0}' as i32 {
        return uid;
    }
    /* Try to find uname in the cache. */
    h = hash(uname) as libc::c_int;
    b = &mut *ucache.offset((h as libc::c_ulong).wrapping_rem(cache_size) as isize) as *mut bucket;
    if !(*b).name.is_null() && (*b).hash == h && strcmp(uname, (*b).name) == 0 as libc::c_int {
        return (*b).id as int64_t;
    }
    /* Free the cache slot for a new entry. */
    free((*b).name as *mut libc::c_void);
    (*b).name = strdup(uname);
    /* Note: If strdup fails, that's okay; we just won't cache. */
    (*b).hash = h; /* Old getpwnam_r ignores last arg. */
    let mut _buffer: [libc::c_char; 128] = [0; 128];
    let mut bufsize: size_t = 128 as libc::c_int as size_t;
    let mut buffer: *mut libc::c_char = _buffer.as_mut_ptr();
    let mut allocated: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut pwent: passwd = passwd {
        pw_name: 0 as *mut libc::c_char,
        pw_passwd: 0 as *mut libc::c_char,
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: 0 as *mut libc::c_char,
        pw_dir: 0 as *mut libc::c_char,
        pw_shell: 0 as *mut libc::c_char,
    };
    let mut result: *mut passwd = 0 as *mut passwd;
    let mut r: libc::c_int = 0;
    loop {
        result = &mut pwent;
        r = getpwnam_r(uname, &mut pwent, buffer, bufsize, &mut result);
        if r == 0 as libc::c_int {
            break;
        }
        if r != ERANGE {
            break;
        }
        bufsize = (bufsize as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        free(allocated as *mut libc::c_void);
        allocated = malloc(bufsize) as *mut libc::c_char;
        if allocated.is_null() {
            break;
        }
        buffer = allocated
    }
    if !result.is_null() {
        uid = (*result).pw_uid as int64_t
    }
    free(allocated as *mut libc::c_void);
    /* HAVE_GETPWNAM_R */
    /* HAVE_GETPWNAM_R */
    (*b).id = uid as uid_t;
    return uid;
}
unsafe extern "C" fn cleanup(mut private: *mut libc::c_void) {
    let mut i: size_t = 0;
    let mut cache: *mut bucket = private as *mut bucket;
    i = 0 as libc::c_int as size_t;
    while i < cache_size {
        free((*cache.offset(i as isize)).name as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    free(cache as *mut libc::c_void);
}
unsafe extern "C" fn hash(mut p: *const libc::c_char) -> libc::c_uint {
    /* A 32-bit version of Peter Weinberger's (PJW) hash algorithm,
    as used by ELF for hashing function names. */
    let mut g: libc::c_uint = 0;
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *p as libc::c_int != '\u{0}' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        h = (h << 4 as libc::c_int).wrapping_add(*fresh0 as libc::c_uint);
        g = h & 0xf0000000 as libc::c_uint;
        if g != 0 as libc::c_int as libc::c_uint {
            h ^= g >> 24 as libc::c_int;
            h &= 0xfffffff as libc::c_int as libc::c_uint
        }
    }
    return h;
}
