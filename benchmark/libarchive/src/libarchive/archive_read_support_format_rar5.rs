use ::c2rust_bitfields;
use ::libc;
extern "C" {
    pub type archive_string_conv;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    /*
     * Basic object manipulation
     */
    #[no_mangle]
    fn archive_entry_clear(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_new() -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_pathname_utf8(_: *mut archive_entry) -> *const libc::c_char;
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
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
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
    fn archive_entry_set_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_update_hardlink_utf8(
        _: *mut archive_entry,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_update_pathname_utf8(
        _: *mut archive_entry,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_symlink_type(_: *mut archive_entry, _: libc::c_int);
    #[no_mangle]
    fn archive_entry_update_symlink_utf8(
        _: *mut archive_entry,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
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
        read_header: Option<
            unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        >,
        read_data: Option<
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
        cleanup: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        format_capabilities: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        has_encrypted_entries: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    ) -> libc::c_int;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __archive_read_ahead(
        _: *mut archive_read,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn blake2sp_init(S: *mut blake2sp_state, outlen: size_t) -> libc::c_int;
    #[no_mangle]
    fn blake2sp_update(
        S: *mut blake2sp_state,
        in_0: *const libc::c_void,
        inlen: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn blake2sp_final(
        S: *mut blake2sp_state,
        out: *mut libc::c_void,
        outlen: size_t,
    ) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type wchar_t = libc::c_int;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
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
/* Configuration data for the bidder. */
/* Name of the filter */
/* Taste the upstream filter to see if we handle this. */
/* Initialize a newly-created filter. */
/* Set an option for the filter bidder. */
/* Release the bidder's configuration data. */
/*
 * This structure is allocated within the archive_read core
 * and initialized by archive_read and the init() method of the
 * corresponding bidder above.
 */
/* Essentially all filters will need these values, so
 * just declare them here. */
/* My bidder. */
/* Who I read from. */
/* Associated archive. */
/* Open a block for reading */
/* Return next block. */
/* Skip forward this many bytes. */
/* Seek to an absolute location. */
/* Close (just this filter) and free(self). */
/* Function that handles switching from reading one block to the next/prev */
/* Read any header metadata if available. */
/* My private data. */
/* Used by reblocking logic. */
/* Current read location. */
/* Bytes in my buffer. */
/* Client buffer information. */
/*
 * The client looks a lot like a filter, so we just wrap it here.
 *
 * TODO: Make archive_read_filter and archive_read_client identical so
 * that users of the library can easily register their own
 * transformation filters.  This will probably break the API/ABI and
 * so should be deferred at least until libarchive 3.0.
 */
/* archive_write_disk object */
/* Progress function invoked during extract. */
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
    pub passphrases: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_data_node {
    pub begin_position: int64_t,
    pub total_size: int64_t,
    pub data: *mut libc::c_void,
}
/* Main context structure. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rar5 {
    pub header_initialized: libc::c_int,
    pub skipped_magic: libc::c_int,
    pub skip_mode: libc::c_int,
    pub merge_mode: libc::c_int,
    pub qlist_offset: uint64_t,
    pub rr_offset: uint64_t,
    pub generic: generic_header,
    pub main: main_header,
    pub cstate: comp_state,
    pub file: file_header,
    pub bits: bit_reader,
    pub vol: multivolume,
    pub last_block_hdr: compressed_block_header,
}
/* Current byte pointer. */
/* RARv5 block header structure. Use bf_* functions to get values from
 * block_flags_u8 field. I.e. bf_byte_count, etc. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compressed_block_header {
    pub block_flags_u8: uint8_t,
    pub block_cksum: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multivolume {
    pub expected_vol_no: libc::c_uint,
    pub push_buf: *mut uint8_t,
}
/* Bit reader state. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_reader {
    pub bit_addr: int8_t,
    pub in_addr: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file_header {
    pub bytes_remaining: ssize_t,
    pub unpacked_size: ssize_t,
    pub last_offset: int64_t,
    pub last_size: int64_t,
    #[bitfield(name = "solid", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "service", ty = "uint8_t", bits = "1..=1")]
    #[bitfield(name = "eof", ty = "uint8_t", bits = "2..=2")]
    #[bitfield(name = "dir", ty = "uint8_t", bits = "3..=3")]
    pub solid_service_eof_dir: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub e_mtime: uint64_t,
    pub e_ctime: uint64_t,
    pub e_atime: uint64_t,
    pub e_unix_ns: uint32_t,
    pub stored_crc32: uint32_t,
    pub calculated_crc32: uint32_t,
    pub blake2sp: [uint8_t; 32],
    pub b2state: blake2sp_state,
    pub has_blake2: libc::c_char,
    pub redir_type: uint64_t,
    pub redir_flags: uint64_t,
    pub solid_window_size: ssize_t,
}
pub type blake2sp_state = blake2sp_state__;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blake2sp_state__ {
    pub S: [[blake2s_state; 1]; 8],
    pub R: [blake2s_state; 1],
    pub buf: [uint8_t; 512],
    pub buflen: size_t,
    pub outlen: size_t,
}
pub type blake2s_state = blake2s_state__;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blake2s_state__ {
    pub h: [uint32_t; 8],
    pub t: [uint32_t; 2],
    pub f: [uint32_t; 2],
    pub buf: [uint8_t; 64],
    pub buflen: size_t,
    pub outlen: size_t,
    pub last_node: uint8_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct comp_state {
    #[bitfield(name = "initialized", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "all_filters_applied", ty = "uint8_t", bits = "1..=1")]
    #[bitfield(name = "switch_multivolume", ty = "uint8_t", bits = "2..=2")]
    #[bitfield(name = "block_parsing_finished", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "notused", ty = "libc::c_int", bits = "4..=7")]
    pub initialized_all_filters_applied_switch_multivolume_block_parsing_finished_notused: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub flags: libc::c_int,
    pub method: libc::c_int,
    pub version: libc::c_int,
    pub window_size: ssize_t,
    pub window_buf: *mut uint8_t,
    pub filtered_buf: *mut uint8_t,
    pub block_buf: *const uint8_t,
    pub window_mask: size_t,
    pub write_ptr: int64_t,
    pub last_write_ptr: int64_t,
    pub last_unstore_ptr: int64_t,
    pub solid_offset: int64_t,
    pub cur_block_size: ssize_t,
    pub last_len: libc::c_int,
    pub bd: decode_table,
    pub ld: decode_table,
    pub dd: decode_table,
    pub ldd: decode_table,
    pub rd: decode_table,
    pub filters: cdeque,
    pub last_block_start: int64_t,
    pub last_block_length: ssize_t,
    pub dist_cache: [libc::c_int; 4],
    pub dready: [data_ready; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_ready {
    pub used: libc::c_char,
    pub buf: *const uint8_t,
    pub size: size_t,
    pub offset: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdeque {
    pub beg_pos: uint16_t,
    pub end_pos: uint16_t,
    pub cap_mask: uint16_t,
    pub size: uint16_t,
    pub arr: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decode_table {
    pub size: uint32_t,
    pub decode_len: [int32_t; 16],
    pub decode_pos: [uint32_t; 16],
    pub quick_bits: uint32_t,
    pub quick_len: [uint8_t; 1024],
    pub quick_num: [uint16_t; 1024],
    pub decode_num: [uint16_t; 306],
}
/* RARv5 main header structure. */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct main_header {
    #[bitfield(name = "solid", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "volume", ty = "uint8_t", bits = "1..=1")]
    #[bitfield(name = "endarc", ty = "uint8_t", bits = "2..=2")]
    #[bitfield(name = "notused", ty = "uint8_t", bits = "3..=7")]
    pub solid_volume_endarc_notused: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub vol_no: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct generic_header {
    #[bitfield(name = "split_after", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "split_before", ty = "uint8_t", bits = "1..=1")]
    #[bitfield(name = "padding", ty = "uint8_t", bits = "2..=7")]
    pub split_after_split_before_padding: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub size: libc::c_int,
    pub last_header_id: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filter_info {
    pub type_0: libc::c_int,
    pub channels: libc::c_int,
    pub pos_r: libc::c_int,
    pub block_start: int64_t,
    pub block_length: ssize_t,
    pub width: uint16_t,
}
pub const CDE_OK: CDE_RETURN_VALUES = 0;
pub const CDE_OUT_OF_BOUNDS: CDE_RETURN_VALUES = 3;
pub const CDE_PARAM: CDE_RETURN_VALUES = 2;
/* Intel x86 code. */
pub const FILTER_ARM: FILTER_TYPE = 3;
/* Intel x86 code. */
pub const FILTER_E8E9: FILTER_TYPE = 2;
/* Generic pattern. */
pub const FILTER_E8: FILTER_TYPE = 1;
pub const FILTER_DELTA: FILTER_TYPE = 0;
pub const ESCAPE: C2RustUnnamed_0 = 15;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HFL_SKIP_IF_UNKNOWN: HEADER_FLAGS = 4;
pub const HEAD_MARK: HEADER_TYPE = 0;
pub const HEAD_ENDARC: HEADER_TYPE = 5;
pub const HEAD_CRYPT: HEADER_TYPE = 4;
pub const CRC32: FILE_FLAGS = 4;
pub const UTIME: FILE_FLAGS = 2;
pub const REDIR_TYPE_NONE: REDIR_TYPE = 0;
pub const UNKNOWN_UNPACKED_SIZE: FILE_FLAGS = 8;
pub const EX_SUBDATA: EXTRA = 7;
pub const EX_CRYPT: EXTRA = 1;
pub const EX_VERSION: EXTRA = 4;
pub const EX_UOWNER: EXTRA = 6;
pub const REDIR_TYPE_HARDLINK: REDIR_TYPE = 4;
pub const REDIR_TYPE_WINSYMLINK: REDIR_TYPE = 2;
pub const REDIR_TYPE_UNIXSYMLINK: REDIR_TYPE = 1;
pub const EX_REDIR: EXTRA = 5;
pub const HAS_UNIX_NS: HTIME_FLAGS = 16;
pub const HAS_ATIME: HTIME_FLAGS = 8;
pub const HAS_CTIME: HTIME_FLAGS = 4;
pub const HAS_MTIME: HTIME_FLAGS = 2;
pub const IS_UNIX: HTIME_FLAGS = 1;
pub type HTIME_FLAGS = libc::c_uint;
pub const EX_HTIME: EXTRA = 3;
pub const BLAKE2sp: HASH_TYPE = 0;
pub type HASH_TYPE = libc::c_uint;
pub const EX_HASH: EXTRA = 2;
pub const HOST_UNIX: HOST_OS = 1;
pub const ATTR_SYSTEM: FILE_ATTRS = 4;
pub const ATTR_HIDDEN: FILE_ATTRS = 2;
pub const ATTR_READONLY: FILE_ATTRS = 1;
pub const ATTR_DIRECTORY: FILE_ATTRS = 16;
pub const HOST_WINDOWS: HOST_OS = 0;
pub const SOLID: COMP_INFO_FLAGS = 64;
pub const DIRECTORY: FILE_FLAGS = 1;
pub const HFL_DATA: HEADER_FLAGS = 2;
pub const HFL_EXTRA_DATA: HEADER_FLAGS = 1;
pub type HOST_OS = libc::c_uint;
pub type COMP_INFO_FLAGS = libc::c_uint;
pub type FILE_ATTRS = libc::c_uint;
pub type FILE_FLAGS = libc::c_uint;
pub const HEAD_FILE: HEADER_TYPE = 2;
pub const HEAD_SERVICE: HEADER_TYPE = 3;
pub const RECOVERY: LOCATOR_FLAGS = 2;
pub const QLIST: LOCATOR_FLAGS = 1;
pub type LOCATOR_FLAGS = libc::c_uint;
// Just one attribute here.
pub const LOCATOR: MAIN_EXTRA = 1;
/* multi-volume archive */
pub const VOLUME_NUMBER: MAIN_FLAGS = 2;
/* volume number, first vol doesn't
 * have it */
pub const SOLID_0: MAIN_FLAGS = 4;
pub const VOLUME: MAIN_FLAGS = 1;
pub type MAIN_EXTRA = libc::c_uint;
pub type MAIN_FLAGS = libc::c_uint;
/* readonly flag, not used */
/* contains Recovery info */
pub const LOCK: MAIN_FLAGS = 16;
/* solid archive */
pub const PROTECT: MAIN_FLAGS = 8;
pub const HEAD_MAIN: HEADER_TYPE = 1;
pub const HFL_SPLIT_BEFORE: HEADER_FLAGS = 8;
pub const HFL_SPLIT_AFTER: HEADER_FLAGS = 16;
pub type HEADER_TYPE = libc::c_uint;
pub const HEAD_UNKNOWN: HEADER_TYPE = 255;
pub const BEST: COMPRESSION_METHOD = 5;
pub const GOOD: COMPRESSION_METHOD = 4;
pub const NORMAL: COMPRESSION_METHOD = 3;
pub const FAST: COMPRESSION_METHOD = 2;
pub const FASTEST: COMPRESSION_METHOD = 1;
pub const STORE: COMPRESSION_METHOD = 0;
pub type COMPRESSION_METHOD = libc::c_uint;
pub const CDE_ALLOC: CDE_RETURN_VALUES = 1;
/* Used in file format check. */
pub type EXTRA = libc::c_uint;
pub type REDIR_TYPE = libc::c_uint;
pub const REDIR_TYPE_FILECOPY: REDIR_TYPE = 5;
pub const REDIR_TYPE_JUNCTION: REDIR_TYPE = 3;
pub type FILTER_TYPE = libc::c_uint;
/* Predictive pattern matching, not used in
RARv5. */
pub const FILTER_NONE: FILTER_TYPE = 8;
/* Intel's Itanium, not used in RARv5. */
pub const FILTER_PPM: FILTER_TYPE = 7;
/* Color palette, not used in RARv5. */
pub const FILTER_ITANIUM: FILTER_TYPE = 6;
/* Audio filter, not used in RARv5. */
pub const FILTER_RGB: FILTER_TYPE = 5;
/* ARM code. */
pub const FILTER_AUDIO: FILTER_TYPE = 4;
/* CDE_xxx = Circular Double Ended (Queue) return values. */
pub type CDE_RETURN_VALUES = libc::c_uint;
pub type HEADER_FLAGS = libc::c_uint;
pub const HFL_INHERITED: HEADER_FLAGS = 64;
pub const HFL_CHILD: HEADER_FLAGS = 32;
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
pub const ARCHIVE_ERRNO_PROGRAMMER: libc::c_int = EINVAL;
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
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
unsafe extern "C" fn archive_le64dec(mut pp: *const libc::c_void) -> uint64_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    return (archive_le32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void)
        as uint64_t)
        << 32 as libc::c_int
        | archive_le32dec(p as *const libc::c_void) as libc::c_ulong;
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FORMAT_RAR_V5: libc::c_int = 0x100000 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_RETRY: libc::c_int = -(10 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED: libc::c_int = -(2 as libc::c_int);
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
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
/*
 * Symlink types
 */
pub const AE_SYMLINK_TYPE_FILE: libc::c_int = 1 as libc::c_int;
pub const AE_SYMLINK_TYPE_DIRECTORY: libc::c_int = 2 as libc::c_int;
/* Real RAR5 magic number is:
 *
 * 0x52, 0x61, 0x72, 0x21, 0x1a, 0x07, 0x01, 0x00
 * "Rar!→•☺·\x00"
 *
 * Retrieved with `rar5_signature()` by XOR'ing it with 0xA1, because I don't
 * want to put this magic sequence in each binary that uses libarchive, so
 * applications that scan through the file for this marker won't trigger on
 * this "false" one.
 *
 * The array itself is decrypted in `rar5_init` function. */
static mut rar5_signature_xor: [libc::c_uchar; 8] = [
    243 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
];
static mut g_unpack_window_size: size_t = 0x20000 as libc::c_int as size_t;
/* These could have been static const's, but they aren't, because of
 * Visual Studio. */
pub const MAX_NAME_IN_CHARS: libc::c_int = 2048 as libc::c_int;
pub const REDIR_SYMLINK_IS_DIR: libc::c_int = 1 as libc::c_int;
pub const OWNER_USER_NAME: libc::c_int = 0x1 as libc::c_int;
pub const OWNER_GROUP_NAME: libc::c_int = 0x2 as libc::c_int;
pub const OWNER_USER_UID: libc::c_int = 0x4 as libc::c_int;
pub const OWNER_GROUP_GID: libc::c_int = 0x8 as libc::c_int;
pub const OWNER_MAXNAMELEN: libc::c_int = 256 as libc::c_int;
pub const HUFF_BC: libc::c_int = 20 as libc::c_int;
pub const HUFF_NC: libc::c_int = 306 as libc::c_int;
pub const HUFF_DC: libc::c_int = 64 as libc::c_int;
pub const HUFF_LDC: libc::c_int = 16 as libc::c_int;
pub const HUFF_RC: libc::c_int = 44 as libc::c_int;
pub const HUFF_TABLE_SIZE: libc::c_int = HUFF_NC + HUFF_DC + HUFF_RC + HUFF_LDC;
/* Clears the contents of this circular deque. */
unsafe extern "C" fn cdeque_clear(mut d: *mut cdeque) {
    (*d).size = 0 as libc::c_int as uint16_t;
    (*d).beg_pos = 0 as libc::c_int as uint16_t;
    (*d).end_pos = 0 as libc::c_int as uint16_t;
}
/* Creates a new circular deque object. Capacity must be power of 2: 8, 16, 32,
 * 64, 256, etc. When the user will add another item above current capacity,
 * the circular deque will overwrite the oldest entry. */
unsafe extern "C" fn cdeque_init(
    mut d: *mut cdeque,
    mut max_capacity_power_of_2: libc::c_int,
) -> libc::c_int {
    if d.is_null() || max_capacity_power_of_2 == 0 as libc::c_int {
        return CDE_PARAM as libc::c_int;
    }
    (*d).cap_mask = (max_capacity_power_of_2 - 1 as libc::c_int) as uint16_t;
    (*d).arr = NULL as *mut size_t;
    if max_capacity_power_of_2 & (*d).cap_mask as libc::c_int != 0 as libc::c_int {
        return CDE_PARAM as libc::c_int;
    }
    cdeque_clear(d);
    (*d).arr = malloc(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(max_capacity_power_of_2 as libc::c_ulong),
    ) as *mut size_t;
    return if !(*d).arr.is_null() {
        CDE_OK as libc::c_int
    } else {
        CDE_ALLOC as libc::c_int
    };
}
/* Return the current size (not capacity) of circular deque `d`. */
unsafe extern "C" fn cdeque_size(mut d: *mut cdeque) -> size_t {
    return (*d).size as size_t;
}
/* Returns the first element of current circular deque. Note that this function
 * doesn't perform any bounds checking. If you need bounds checking, use
 * `cdeque_front()` function instead. */
unsafe extern "C" fn cdeque_front_fast(mut d: *mut cdeque, mut value: *mut *mut libc::c_void) {
    *value = *(*d).arr.offset((*d).beg_pos as isize) as *mut libc::c_void;
}
/* Returns the first element of current circular deque. This function
 * performs bounds checking. */
unsafe extern "C" fn cdeque_front(
    mut d: *mut cdeque,
    mut value: *mut *mut libc::c_void,
) -> libc::c_int {
    if (*d).size as libc::c_int > 0 as libc::c_int {
        cdeque_front_fast(d, value);
        return CDE_OK as libc::c_int;
    } else {
        return CDE_OUT_OF_BOUNDS as libc::c_int;
    };
}
/* Pushes a new element into the end of this circular deque object. If current
 * size will exceed capacity, the oldest element will be overwritten. */
unsafe extern "C" fn cdeque_push_back(
    mut d: *mut cdeque,
    mut item: *mut libc::c_void,
) -> libc::c_int {
    if d.is_null() {
        return CDE_PARAM as libc::c_int;
    }
    if (*d).size as libc::c_int == (*d).cap_mask as libc::c_int + 1 as libc::c_int {
        return CDE_OUT_OF_BOUNDS as libc::c_int;
    }
    *(*d).arr.offset((*d).end_pos as isize) = item as size_t;
    (*d).end_pos =
        ((*d).end_pos as libc::c_int + 1 as libc::c_int & (*d).cap_mask as libc::c_int) as uint16_t;
    (*d).size = (*d).size.wrapping_add(1);
    return CDE_OK as libc::c_int;
}
/* Pops a front element of this circular deque object and returns its value.
 * This function doesn't perform any bounds checking. */
unsafe extern "C" fn cdeque_pop_front_fast(mut d: *mut cdeque, mut value: *mut *mut libc::c_void) {
    *value = *(*d).arr.offset((*d).beg_pos as isize) as *mut libc::c_void;
    (*d).beg_pos =
        ((*d).beg_pos as libc::c_int + 1 as libc::c_int & (*d).cap_mask as libc::c_int) as uint16_t;
    (*d).size = (*d).size.wrapping_sub(1);
}
/* Pops a front element of this circular deque object and returns its value.
 * This function performs bounds checking. */
unsafe extern "C" fn cdeque_pop_front(
    mut d: *mut cdeque,
    mut value: *mut *mut libc::c_void,
) -> libc::c_int {
    if d.is_null() || value.is_null() {
        return CDE_PARAM as libc::c_int;
    }
    if (*d).size as libc::c_int == 0 as libc::c_int {
        return CDE_OUT_OF_BOUNDS as libc::c_int;
    }
    cdeque_pop_front_fast(d, value);
    return CDE_OK as libc::c_int;
}
/* Convenience function to cast filter_info** to void **. */
unsafe extern "C" fn cdeque_filter_p(mut f: *mut *mut filter_info) -> *mut *mut libc::c_void {
    return f as size_t as *mut *mut libc::c_void;
}
/* Convenience function to cast filter_info* to void *. */
unsafe extern "C" fn cdeque_filter(mut f: *mut filter_info) -> *mut libc::c_void {
    return f as size_t as *mut *mut libc::c_void as *mut libc::c_void;
}
/* Destroys this circular deque object. Deallocates the memory of the
 * collection buffer, but doesn't deallocate the memory of any pointer passed
 * to this deque as a value. */
unsafe extern "C" fn cdeque_free(mut d: *mut cdeque) {
    if d.is_null() {
        return;
    }
    if (*d).arr.is_null() {
        return;
    }
    free((*d).arr as *mut libc::c_void);
    (*d).arr = NULL as *mut size_t;
    (*d).beg_pos = -(1 as libc::c_int) as uint16_t;
    (*d).end_pos = -(1 as libc::c_int) as uint16_t;
    (*d).cap_mask = 0 as libc::c_int as uint16_t;
}
#[inline]
unsafe extern "C" fn bf_bit_size(mut hdr: *const compressed_block_header) -> uint8_t {
    return ((*hdr).block_flags_u8 as libc::c_int & 7 as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn bf_byte_count(mut hdr: *const compressed_block_header) -> uint8_t {
    return ((*hdr).block_flags_u8 as libc::c_int >> 3 as libc::c_int & 7 as libc::c_int)
        as uint8_t;
}
#[inline]
unsafe extern "C" fn bf_is_table_present(mut hdr: *const compressed_block_header) -> uint8_t {
    return ((*hdr).block_flags_u8 as libc::c_int >> 7 as libc::c_int & 1 as libc::c_int)
        as uint8_t;
}
#[inline]
unsafe extern "C" fn get_context(mut a: *mut archive_read) -> *mut rar5 {
    return (*(*a).format).data as *mut rar5;
}
/* Convenience functions used by filter implementations. */
unsafe extern "C" fn circular_memcpy(
    mut dst: *mut uint8_t,
    mut window: *mut uint8_t,
    mask: uint64_t,
    mut start: int64_t,
    mut end: int64_t,
) {
    if start as libc::c_ulong & mask > end as libc::c_ulong & mask {
        let mut len1: ssize_t = mask
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(start as libc::c_ulong & mask) as ssize_t;
        let mut len2: ssize_t = (end as libc::c_ulong & mask) as ssize_t;
        memcpy(
            dst as *mut libc::c_void,
            &mut *window.offset((start as libc::c_ulong & mask) as isize) as *mut uint8_t
                as *const libc::c_void,
            len1 as libc::c_ulong,
        );
        memcpy(
            dst.offset(len1 as isize) as *mut libc::c_void,
            window as *const libc::c_void,
            len2 as libc::c_ulong,
        );
    } else {
        memcpy(
            dst as *mut libc::c_void,
            &mut *window.offset((start as libc::c_ulong & mask) as isize) as *mut uint8_t
                as *const libc::c_void,
            (end - start) as size_t,
        );
    };
}
unsafe extern "C" fn read_filter_data(mut rar: *mut rar5, mut offset: uint32_t) -> uint32_t {
    let mut linear_buf: [uint8_t; 4] = [0; 4];
    circular_memcpy(
        linear_buf.as_mut_ptr(),
        (*rar).cstate.window_buf,
        (*rar).cstate.window_mask,
        offset as int64_t,
        offset.wrapping_add(4 as libc::c_int as libc::c_uint) as int64_t,
    );
    return archive_le32dec(linear_buf.as_mut_ptr() as *const libc::c_void);
}
unsafe extern "C" fn write_filter_data(
    mut rar: *mut rar5,
    mut offset: uint32_t,
    mut value: uint32_t,
) {
    archive_le32enc(
        &mut *(*rar).cstate.filtered_buf.offset(offset as isize) as *mut uint8_t
            as *mut libc::c_void,
        value,
    );
}
/* Allocates a new filter descriptor and adds it to the filter array. */
unsafe extern "C" fn add_new_filter(mut rar: *mut rar5) -> *mut filter_info {
    let mut f: *mut filter_info = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<filter_info>() as libc::c_ulong,
    ) as *mut filter_info;
    if f.is_null() {
        return NULL as *mut filter_info;
    }
    cdeque_push_back(&mut (*rar).cstate.filters, cdeque_filter(f));
    return f;
}
unsafe extern "C" fn run_delta_filter(
    mut rar: *mut rar5,
    mut flt: *mut filter_info,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dest_pos: ssize_t = 0;
    let mut src_pos: ssize_t = 0 as libc::c_int as ssize_t;
    i = 0 as libc::c_int;
    while i < (*flt).channels {
        let mut prev_byte: uint8_t = 0 as libc::c_int as uint8_t;
        dest_pos = i as ssize_t;
        while dest_pos < (*flt).block_length {
            let mut byte: uint8_t = 0;
            byte = *(*rar).cstate.window_buf.offset(
                (((*rar).cstate.solid_offset + (*flt).block_start + src_pos) as libc::c_ulong
                    & (*rar).cstate.window_mask) as isize,
            );
            prev_byte = (prev_byte as libc::c_int - byte as libc::c_int) as uint8_t;
            *(*rar).cstate.filtered_buf.offset(dest_pos as isize) = prev_byte;
            src_pos += 1;
            dest_pos += (*flt).channels as libc::c_long
        }
        i += 1
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn run_e8e9_filter(
    mut rar: *mut rar5,
    mut flt: *mut filter_info,
    mut extended: libc::c_int,
) -> libc::c_int {
    let file_size: uint32_t = 0x1000000 as libc::c_int as uint32_t;
    let mut i: ssize_t = 0;
    circular_memcpy(
        (*rar).cstate.filtered_buf,
        (*rar).cstate.window_buf,
        (*rar).cstate.window_mask,
        (*rar).cstate.solid_offset + (*flt).block_start,
        (*rar).cstate.solid_offset + (*flt).block_start + (*flt).block_length,
    );
    i = 0 as libc::c_int as ssize_t;
    while i < (*flt).block_length - 4 as libc::c_int as libc::c_long {
        let fresh0 = i;
        i = i + 1;
        let mut b: uint8_t = *(*rar).cstate.window_buf.offset(
            (((*rar).cstate.solid_offset + (*flt).block_start + fresh0) as libc::c_ulong
                & (*rar).cstate.window_mask) as isize,
        );
        /*
         * 0xE8 = x86's call <relative_addr_uint32> (function call)
         * 0xE9 = x86's jmp <relative_addr_uint32> (unconditional jump)
         */
        if b as libc::c_int == 0xe8 as libc::c_int
            || extended != 0 && b as libc::c_int == 0xe9 as libc::c_int
        {
            let mut addr: uint32_t = 0;
            let mut offset: uint32_t =
                ((i + (*flt).block_start) % file_size as libc::c_long) as uint32_t;
            addr = read_filter_data(
                rar,
                (((*rar).cstate.solid_offset + (*flt).block_start + i) as uint32_t as libc::c_ulong
                    & (*rar).cstate.window_mask) as uint32_t,
            );
            if addr & 0x80000000 as libc::c_uint != 0 {
                if addr.wrapping_add(offset) & 0x80000000 as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    write_filter_data(rar, i as uint32_t, addr.wrapping_add(file_size));
                }
            } else if addr.wrapping_sub(file_size) & 0x80000000 as libc::c_uint != 0 {
                let mut naddr: uint32_t = addr.wrapping_sub(offset);
                write_filter_data(rar, i as uint32_t, naddr);
            }
            i += 4 as libc::c_int as libc::c_long
        }
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn run_arm_filter(mut rar: *mut rar5, mut flt: *mut filter_info) -> libc::c_int {
    let mut i: ssize_t = 0 as libc::c_int as ssize_t;
    let mut offset: uint32_t = 0;
    circular_memcpy(
        (*rar).cstate.filtered_buf,
        (*rar).cstate.window_buf,
        (*rar).cstate.window_mask,
        (*rar).cstate.solid_offset + (*flt).block_start,
        (*rar).cstate.solid_offset + (*flt).block_start + (*flt).block_length,
    );
    i = 0 as libc::c_int as ssize_t;
    while i < (*flt).block_length - 3 as libc::c_int as libc::c_long {
        let mut b: *mut uint8_t = &mut *(*rar).cstate.window_buf.offset(
            (((*rar).cstate.solid_offset
                + (*flt).block_start
                + i
                + 3 as libc::c_int as libc::c_long) as libc::c_ulong
                & (*rar).cstate.window_mask) as isize,
        ) as *mut uint8_t;
        if *b as libc::c_int == 0xeb as libc::c_int {
            /* 0xEB = ARM's BL (branch + link) instruction. */
            offset = read_filter_data(
                rar,
                (((*rar).cstate.solid_offset + (*flt).block_start + i) as libc::c_ulong
                    & (*rar).cstate.window_mask) as uint32_t,
            ) & 0xffffff as libc::c_int as libc::c_uint;
            offset = (offset as libc::c_uint).wrapping_sub(
                ((i + (*flt).block_start) / 4 as libc::c_int as libc::c_long) as uint32_t,
            ) as uint32_t as uint32_t;
            offset = offset & 0xffffff as libc::c_int as libc::c_uint | 0xeb000000 as libc::c_uint;
            write_filter_data(rar, i as uint32_t, offset);
        }
        i += 4 as libc::c_int as libc::c_long
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn run_filter(
    mut a: *mut archive_read,
    mut flt: *mut filter_info,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut rar: *mut rar5 = get_context(a);
    free((*rar).cstate.filtered_buf as *mut libc::c_void);
    (*rar).cstate.filtered_buf = malloc((*flt).block_length as libc::c_ulong) as *mut uint8_t;
    if (*rar).cstate.filtered_buf.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for filter data.\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    match (*flt).type_0 {
        0 => ret = run_delta_filter(rar, flt),
        1 | 2 => {
            /* fallthrough */
            ret = run_e8e9_filter(
                rar,
                flt,
                ((*flt).type_0 == FILTER_E8E9 as libc::c_int) as libc::c_int,
            )
        }
        3 => ret = run_arm_filter(rar, flt),
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unsupported filter type: 0x%x\x00" as *const u8 as *const libc::c_char,
                (*flt).type_0,
            );
            return ARCHIVE_FATAL;
        }
    }
    if ret != ARCHIVE_OK {
        /* Filter has failed. */
        return ret;
    }
    if ARCHIVE_OK
        != push_data_ready(
            a,
            rar,
            (*rar).cstate.filtered_buf,
            (*flt).block_length as size_t,
            (*rar).cstate.last_write_ptr,
        )
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Stack overflow when submitting unpacked data\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    (*rar).cstate.last_write_ptr += (*flt).block_length;
    return ARCHIVE_OK;
}
/* The `push_data` function submits the selected data range to the user.
 * Next call of `use_data` will use the pointer, size and offset arguments
 * that are specified here. These arguments are pushed to the FIFO stack here,
 * and popped from the stack by the `use_data` function. */
unsafe extern "C" fn push_data(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut buf: *const uint8_t,
    mut idx_begin: int64_t,
    mut idx_end: int64_t,
) {
    let wmask: uint64_t = (*rar).cstate.window_mask;
    let solid_write_ptr: ssize_t = (((*rar).cstate.solid_offset + (*rar).cstate.last_write_ptr)
        as libc::c_ulong
        & wmask) as ssize_t;
    idx_begin += (*rar).cstate.solid_offset;
    idx_end += (*rar).cstate.solid_offset;
    /* Check if our unpacked data is wrapped inside the window circular
     * buffer.  If it's not wrapped, it can be copied out by using
     * a single memcpy, but when it's wrapped, we need to copy the first
     * part with one memcpy, and the second part with another memcpy. */
    if idx_begin as libc::c_ulong & wmask > idx_end as libc::c_ulong & wmask {
        /* The data is wrapped (begin offset sis bigger than end
         * offset). */
        let frag1_size: ssize_t = ((*rar).cstate.window_size as libc::c_ulong)
            .wrapping_sub(idx_begin as libc::c_ulong & wmask)
            as ssize_t;
        let frag2_size: ssize_t = (idx_end as libc::c_ulong & wmask) as ssize_t;
        /* Copy the first part of the buffer first. */
        push_data_ready(
            a,
            rar,
            buf.offset(solid_write_ptr as isize),
            frag1_size as size_t,
            (*rar).cstate.last_write_ptr,
        );
        /* Copy the second part of the buffer. */
        push_data_ready(
            a,
            rar,
            buf,
            frag2_size as size_t,
            (*rar).cstate.last_write_ptr + frag1_size,
        );
        (*rar).cstate.last_write_ptr += frag1_size + frag2_size
    } else {
        /* Data is not wrapped, so we can just use one call to copy the
         * data. */
        push_data_ready(
            a,
            rar,
            buf.offset(solid_write_ptr as isize),
            (idx_end - idx_begin) as libc::c_ulong & wmask,
            (*rar).cstate.last_write_ptr,
        );
        (*rar).cstate.last_write_ptr += idx_end - idx_begin
    };
}
/* Convenience function that submits the data to the user. It uses the
 * unpack window buffer as a source location. */
unsafe extern "C" fn push_window_data(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut idx_begin: int64_t,
    mut idx_end: int64_t,
) {
    push_data(a, rar, (*rar).cstate.window_buf, idx_begin, idx_end);
}
unsafe extern "C" fn apply_filters(mut a: *mut archive_read) -> libc::c_int {
    let mut flt: *mut filter_info = 0 as *mut filter_info;
    let mut rar: *mut rar5 = get_context(a);
    let mut ret: libc::c_int = 0;
    (*rar)
        .cstate
        .set_all_filters_applied(0 as libc::c_int as uint8_t);
    /* Get the first filter that can be applied to our data. The data
     * needs to be fully unpacked before the filter can be run. */
    if CDE_OK as libc::c_int == cdeque_front(&mut (*rar).cstate.filters, cdeque_filter_p(&mut flt))
    {
        /* Check if our unpacked data fully covers this filter's
         * range. */
        if (*rar).cstate.write_ptr > (*flt).block_start
            && (*rar).cstate.write_ptr >= (*flt).block_start + (*flt).block_length
        {
            /* Check if we have some data pending to be written
             * right before the filter's start offset. */
            if (*rar).cstate.last_write_ptr == (*flt).block_start {
                /* Run the filter specified by descriptor
                 * `flt`. */
                ret = run_filter(a, flt);
                if ret != ARCHIVE_OK {
                    /* Filter failure, return error. */
                    return ret;
                }
                /* Filter descriptor won't be needed anymore
                 * after it's used, * so remove it from the
                 * filter list and free its memory. */
                cdeque_pop_front(&mut (*rar).cstate.filters, cdeque_filter_p(&mut flt));
                free(flt as *mut libc::c_void);
            } else {
                /* We can't run filters yet, dump the memory
                 * right before the filter. */
                push_window_data(a, rar, (*rar).cstate.last_write_ptr, (*flt).block_start);
            }
            /* Return 'filter applied or not needed' state to the
             * caller. */
            return ARCHIVE_RETRY;
        }
    }
    (*rar)
        .cstate
        .set_all_filters_applied(1 as libc::c_int as uint8_t);
    return ARCHIVE_OK;
}
unsafe extern "C" fn dist_cache_push(mut rar: *mut rar5, mut value: libc::c_int) {
    let mut q: *mut libc::c_int = (*rar).cstate.dist_cache.as_mut_ptr();
    *q.offset(3 as libc::c_int as isize) = *q.offset(2 as libc::c_int as isize);
    *q.offset(2 as libc::c_int as isize) = *q.offset(1 as libc::c_int as isize);
    *q.offset(1 as libc::c_int as isize) = *q.offset(0 as libc::c_int as isize);
    *q.offset(0 as libc::c_int as isize) = value;
}
unsafe extern "C" fn dist_cache_touch(mut rar: *mut rar5, mut idx: libc::c_int) -> libc::c_int {
    let mut q: *mut libc::c_int = (*rar).cstate.dist_cache.as_mut_ptr();
    let mut i: libc::c_int = 0;
    let mut dist: libc::c_int = *q.offset(idx as isize);
    i = idx;
    while i > 0 as libc::c_int {
        *q.offset(i as isize) = *q.offset((i - 1 as libc::c_int) as isize);
        i -= 1
    }
    *q.offset(0 as libc::c_int as isize) = dist;
    return dist;
}
unsafe extern "C" fn free_filters(mut rar: *mut rar5) {
    let mut d: *mut cdeque = &mut (*rar).cstate.filters;
    /* Free any remaining filters. All filters should be naturally
     * consumed by the unpacking function, so remaining filters after
     * unpacking normally mean that unpacking wasn't successful.
     * But still of course we shouldn't leak memory in such case. */
    /* cdeque_size() is a fast operation, so we can use it as a loop
     * expression. */
    while cdeque_size(d) > 0 as libc::c_int as libc::c_ulong {
        let mut f: *mut filter_info = NULL as *mut filter_info;
        /* Pop_front will also decrease the collection's size. */
        if CDE_OK as libc::c_int == cdeque_pop_front(d, cdeque_filter_p(&mut f)) {
            free(f as *mut libc::c_void);
        }
    }
    cdeque_clear(d);
    /* Also clear out the variables needed for sanity checking. */
    (*rar).cstate.last_block_start = 0 as libc::c_int as int64_t;
    (*rar).cstate.last_block_length = 0 as libc::c_int as ssize_t;
}
unsafe extern "C" fn reset_file_context(mut rar: *mut rar5) {
    memset(
        &mut (*rar).file as *mut file_header as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<file_header>() as libc::c_ulong,
    );
    blake2sp_init(&mut (*rar).file.b2state, 32 as libc::c_int as size_t);
    if (*rar).main.solid() != 0 {
        (*rar).cstate.solid_offset += (*rar).cstate.write_ptr
    } else {
        (*rar).cstate.solid_offset = 0 as libc::c_int as int64_t
    }
    (*rar).cstate.write_ptr = 0 as libc::c_int as int64_t;
    (*rar).cstate.last_write_ptr = 0 as libc::c_int as int64_t;
    (*rar).cstate.last_unstore_ptr = 0 as libc::c_int as int64_t;
    (*rar).file.redir_type = REDIR_TYPE_NONE as libc::c_int as uint64_t;
    (*rar).file.redir_flags = 0 as libc::c_int as uint64_t;
    free_filters(rar);
}
#[inline]
unsafe extern "C" fn get_archive_read(
    mut a: *mut archive,
    mut ar: *mut *mut archive_read,
) -> libc::c_int {
    *ar = a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_rar5\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn read_ahead(
    mut a: *mut archive_read,
    mut how_many: size_t,
    mut ptr: *mut *const uint8_t,
) -> libc::c_int {
    let mut avail: ssize_t = -(1 as libc::c_int) as ssize_t;
    if ptr.is_null() {
        return 0 as libc::c_int;
    }
    *ptr = __archive_read_ahead(a, how_many, &mut avail) as *const uint8_t;
    if (*ptr).is_null() {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn consume(mut a: *mut archive_read, mut how_many: int64_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = if how_many == __archive_read_consume(a, how_many) {
        ARCHIVE_OK
    } else {
        ARCHIVE_FATAL
    };
    return ret;
}
/* *
 * Read a RAR5 variable sized numeric value. This value will be stored in
 * `pvalue`. The `pvalue_len` argument points to a variable that will receive
 * the byte count that was consumed in order to decode the `pvalue` value, plus
 * one.
 *
 * pvalue_len is optional and can be NULL.
 *
 * NOTE: if `pvalue_len` is NOT NULL, the caller needs to manually consume
 * the number of bytes that `pvalue_len` value contains. If the `pvalue_len`
 * is NULL, this consuming operation is done automatically.
 *
 * Returns 1 if *pvalue was successfully read.
 * Returns 0 if there was an error. In this case, *pvalue contains an
 *           invalid value.
 */
unsafe extern "C" fn read_var(
    mut a: *mut archive_read,
    mut pvalue: *mut uint64_t,
    mut pvalue_len: *mut uint64_t,
) -> libc::c_int {
    let mut result: uint64_t = 0 as libc::c_int as uint64_t;
    let mut shift: size_t = 0;
    let mut i: size_t = 0;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut b: uint8_t = 0;
    /* We will read maximum of 8 bytes. We don't have to handle the
     * situation to read the RAR5 variable-sized value stored at the end of
     * the file, because such situation will never happen. */
    if read_ahead(a, 8 as libc::c_int as size_t, &mut p) == 0 {
        return 0 as libc::c_int;
    }
    shift = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        b = *p.offset(i as isize);
        /* Strip the MSB from the input byte and add the resulting
         * number to the `result`. */
        result = (result as libc::c_ulong)
            .wrapping_add((b as libc::c_ulong & 0x7f as libc::c_int as uint64_t) << shift)
            as uint64_t as uint64_t;
        /* MSB set to 1 means we need to continue decoding process.
         * MSB set to 0 means we're done.
         *
         * This conditional checks for the second case. */
        if b as libc::c_int & 0x80 as libc::c_int == 0 as libc::c_int {
            if !pvalue.is_null() {
                *pvalue = result
            }
            /* If the caller has passed the `pvalue_len` pointer,
             * store the number of consumed bytes in it and do NOT
             * consume those bytes, since the caller has all the
             * information it needs to perform */
            if !pvalue_len.is_null() {
                *pvalue_len = (1 as libc::c_int as libc::c_ulong).wrapping_add(i)
            } else if ARCHIVE_OK
                != consume(
                    a,
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(i) as int64_t,
                )
            {
                return 0 as libc::c_int;
            }
            /* If the caller did not provide the
             * `pvalue_len` pointer, it will not have the
             * possibility to advance the file pointer,
             * because it will not know how many bytes it
             * needs to consume. This is why we handle
             * such situation here automatically. */
            /* End of decoding process, return success. */
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
        shift = (shift as libc::c_ulong).wrapping_add(7 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    /* The decoded value takes the maximum number of 8 bytes.
     * It's a maximum number of bytes, so end decoding process here
     * even if the first bit of last byte is 1. */
    if !pvalue.is_null() {
        *pvalue = result
    }
    if !pvalue_len.is_null() {
        *pvalue_len = 9 as libc::c_int as uint64_t
    } else if ARCHIVE_OK != consume(a, 9 as libc::c_int as int64_t) {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_var_sized(
    mut a: *mut archive_read,
    mut pvalue: *mut size_t,
    mut pvalue_len: *mut size_t,
) -> libc::c_int {
    let mut v: uint64_t = 0;
    let mut v_size: uint64_t = 0 as libc::c_int as uint64_t;
    let ret: libc::c_int = if !pvalue_len.is_null() {
        read_var(a, &mut v, &mut v_size)
    } else {
        read_var(a, &mut v, NULL as *mut uint64_t)
    };
    if ret == 1 as libc::c_int && !pvalue.is_null() {
        *pvalue = v
    }
    if !pvalue_len.is_null() {
        /* Possible data truncation should be safe. */
        *pvalue_len = v_size
    }
    return ret;
}
unsafe extern "C" fn read_bits_32(
    mut rar: *mut rar5,
    mut p: *const uint8_t,
    mut value: *mut uint32_t,
) -> libc::c_int {
    let mut bits: uint32_t =
        (*p.offset((*rar).bits.in_addr as isize) as uint32_t) << 24 as libc::c_int;
    bits |= ((*p.offset(((*rar).bits.in_addr + 1 as libc::c_int) as isize) as libc::c_int)
        << 16 as libc::c_int) as libc::c_uint;
    bits |= ((*p.offset(((*rar).bits.in_addr + 2 as libc::c_int) as isize) as libc::c_int)
        << 8 as libc::c_int) as libc::c_uint;
    bits |= *p.offset(((*rar).bits.in_addr + 3 as libc::c_int) as isize) as libc::c_uint;
    bits <<= (*rar).bits.bit_addr as libc::c_int;
    bits |= (*p.offset(((*rar).bits.in_addr + 4 as libc::c_int) as isize) as libc::c_int
        >> 8 as libc::c_int - (*rar).bits.bit_addr as libc::c_int) as libc::c_uint;
    *value = bits;
    return ARCHIVE_OK;
}
unsafe extern "C" fn read_bits_16(
    mut rar: *mut rar5,
    mut p: *const uint8_t,
    mut value: *mut uint16_t,
) -> libc::c_int {
    let mut bits: libc::c_int =
        (*p.offset((*rar).bits.in_addr as isize) as uint32_t as libc::c_int) << 16 as libc::c_int;
    bits |= (*p.offset(((*rar).bits.in_addr + 1 as libc::c_int) as isize) as libc::c_int)
        << 8 as libc::c_int;
    bits |= *p.offset(((*rar).bits.in_addr + 2 as libc::c_int) as isize) as libc::c_int;
    bits >>= 8 as libc::c_int - (*rar).bits.bit_addr as libc::c_int;
    *value = (bits & 0xffff as libc::c_int) as uint16_t;
    return ARCHIVE_OK;
}
unsafe extern "C" fn skip_bits(mut rar: *mut rar5, mut bits: libc::c_int) {
    let new_bits: libc::c_int = (*rar).bits.bit_addr as libc::c_int + bits;
    (*rar).bits.in_addr += new_bits >> 3 as libc::c_int;
    (*rar).bits.bit_addr = (new_bits & 7 as libc::c_int) as int8_t;
}
/* n = up to 16 */
unsafe extern "C" fn read_consume_bits(
    mut rar: *mut rar5,
    mut p: *const uint8_t,
    mut n: libc::c_int,
    mut value: *mut libc::c_int,
) -> libc::c_int {
    let mut v: uint16_t = 0;
    let mut ret: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    if n == 0 as libc::c_int || n > 16 as libc::c_int {
        /* This is a programmer error and should never happen
         * in runtime. */
        return ARCHIVE_FATAL;
    }
    ret = read_bits_16(rar, p, &mut v);
    if ret != ARCHIVE_OK {
        return ret;
    }
    num = v as libc::c_int;
    num >>= 16 as libc::c_int - n;
    skip_bits(rar, n);
    if !value.is_null() {
        *value = num
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn read_u32(mut a: *mut archive_read, mut pvalue: *mut uint32_t) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    if read_ahead(a, 4 as libc::c_int as size_t, &mut p) == 0 {
        return 0 as libc::c_int;
    }
    *pvalue = archive_le32dec(p as *const libc::c_void);
    return if ARCHIVE_OK == consume(a, 4 as libc::c_int as int64_t) {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn read_u64(mut a: *mut archive_read, mut pvalue: *mut uint64_t) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    if read_ahead(a, 8 as libc::c_int as size_t, &mut p) == 0 {
        return 0 as libc::c_int;
    }
    *pvalue = archive_le64dec(p as *const libc::c_void);
    return if ARCHIVE_OK == consume(a, 8 as libc::c_int as int64_t) {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn bid_standard(mut a: *mut archive_read) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut signature: [libc::c_char; 8] = [0; 8];
    rar5_signature(signature.as_mut_ptr());
    if read_ahead(
        a,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
        &mut p,
    ) == 0
    {
        return -(1 as libc::c_int);
    }
    if memcmp(
        signature.as_mut_ptr() as *const libc::c_void,
        p as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
    ) == 0
    {
        return 30 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn rar5_bid(mut a: *mut archive_read, mut best_bid: libc::c_int) -> libc::c_int {
    let mut my_bid: libc::c_int = 0;
    if best_bid > 30 as libc::c_int {
        return -(1 as libc::c_int);
    }
    my_bid = bid_standard(a);
    if my_bid > -(1 as libc::c_int) {
        return my_bid;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn rar5_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    /* No options supported in this version. Return the ARCHIVE_WARN code
     * to signal the options supervisor that the unpacker didn't handle
     * setting this option. */
    return ARCHIVE_WARN;
}
unsafe extern "C" fn init_header(mut a: *mut archive_read) {
    (*a).archive.archive_format = ARCHIVE_FORMAT_RAR_V5;
    (*a).archive.archive_format_name = b"RAR5\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn init_window_mask(mut rar: *mut rar5) {
    if (*rar).cstate.window_size != 0 {
        (*rar).cstate.window_mask =
            ((*rar).cstate.window_size - 1 as libc::c_int as libc::c_long) as size_t
    } else {
        (*rar).cstate.window_mask = 0 as libc::c_int as size_t
    };
}
unsafe extern "C" fn process_main_locator_extra_block(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
) -> libc::c_int {
    let mut locator_flags: uint64_t = 0;
    if read_var(a, &mut locator_flags, NULL as *mut uint64_t) == 0 {
        return ARCHIVE_EOF;
    }
    if locator_flags & QLIST as libc::c_int as libc::c_ulong != 0 {
        if read_var(a, &mut (*rar).qlist_offset, NULL as *mut uint64_t) == 0 {
            return ARCHIVE_EOF;
        }
        /* qlist is not used */
    }
    if locator_flags & RECOVERY as libc::c_int as libc::c_ulong != 0 {
        if read_var(a, &mut (*rar).rr_offset, NULL as *mut uint64_t) == 0 {
            return ARCHIVE_EOF;
        }
        /* rr is not used */
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn parse_file_extra_hash(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut extra_data_size: *mut ssize_t,
) -> libc::c_int {
    let mut hash_type: size_t = 0 as libc::c_int as size_t;
    let mut value_len: size_t = 0;
    if read_var_sized(a, &mut hash_type, &mut value_len) == 0 {
        return ARCHIVE_EOF;
    }
    *extra_data_size =
        (*extra_data_size as libc::c_ulong).wrapping_sub(value_len) as ssize_t as ssize_t;
    if ARCHIVE_OK != consume(a, value_len as int64_t) {
        return ARCHIVE_EOF;
    }
    /* The file uses BLAKE2sp checksum algorithm instead of plain old
     * CRC32. */
    if hash_type == BLAKE2sp as libc::c_int as libc::c_ulong {
        let mut p: *const uint8_t = 0 as *const uint8_t;
        let hash_size: libc::c_int =
            ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong as libc::c_int;
        if read_ahead(a, hash_size as size_t, &mut p) == 0 {
            return ARCHIVE_EOF;
        }
        (*rar).file.has_blake2 = 1 as libc::c_int as libc::c_char;
        memcpy(
            &mut (*rar).file.blake2sp as *mut [uint8_t; 32] as *mut libc::c_void,
            p as *const libc::c_void,
            hash_size as libc::c_ulong,
        );
        if ARCHIVE_OK != consume(a, hash_size as int64_t) {
            return ARCHIVE_EOF;
        }
        *extra_data_size -= hash_size as libc::c_long
    } else {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Unsupported hash type (0x%x)\x00" as *const u8 as *const libc::c_char,
            hash_type as libc::c_int,
        );
        return ARCHIVE_FATAL;
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn time_win_to_unix(mut win_time: uint64_t) -> uint64_t {
    let ns_in_sec: size_t = 10000000 as libc::c_int as size_t;
    let sec_to_unix: uint64_t = 11644473600 as libc::c_longlong as uint64_t;
    return win_time.wrapping_div(ns_in_sec).wrapping_sub(sec_to_unix);
}
unsafe extern "C" fn parse_htime_item(
    mut a: *mut archive_read,
    mut unix_time: libc::c_char,
    mut where_0: *mut uint64_t,
    mut extra_data_size: *mut ssize_t,
) -> libc::c_int {
    if unix_time != 0 {
        let mut time_val: uint32_t = 0;
        if read_u32(a, &mut time_val) == 0 {
            return ARCHIVE_EOF;
        }
        *extra_data_size -= 4 as libc::c_int as libc::c_long;
        *where_0 = time_val as uint64_t
    } else {
        let mut windows_time: uint64_t = 0;
        if read_u64(a, &mut windows_time) == 0 {
            return ARCHIVE_EOF;
        }
        *where_0 = time_win_to_unix(windows_time);
        *extra_data_size -= 8 as libc::c_int as libc::c_long
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn parse_file_extra_version(
    mut a: *mut archive_read,
    mut e: *mut archive_entry,
    mut extra_data_size: *mut ssize_t,
) -> libc::c_int {
    let mut flags: size_t = 0 as libc::c_int as size_t;
    let mut version: size_t = 0 as libc::c_int as size_t;
    let mut value_len: size_t = 0 as libc::c_int as size_t;
    let mut version_string: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut name_utf8_string: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut cur_filename: *const libc::c_char = 0 as *const libc::c_char;
    /* Flags are ignored. */
    if read_var_sized(a, &mut flags, &mut value_len) == 0 {
        return ARCHIVE_EOF;
    }
    *extra_data_size =
        (*extra_data_size as libc::c_ulong).wrapping_sub(value_len) as ssize_t as ssize_t;
    if ARCHIVE_OK != consume(a, value_len as int64_t) {
        return ARCHIVE_EOF;
    }
    if read_var_sized(a, &mut version, &mut value_len) == 0 {
        return ARCHIVE_EOF;
    }
    *extra_data_size =
        (*extra_data_size as libc::c_ulong).wrapping_sub(value_len) as ssize_t as ssize_t;
    if ARCHIVE_OK != consume(a, value_len as int64_t) {
        return ARCHIVE_EOF;
    }
    /* extra_data_size should be zero here. */
    cur_filename = archive_entry_pathname_utf8(e);
    if cur_filename.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Version entry without file name\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    version_string.s = NULL as *mut libc::c_char;
    version_string.length = 0 as libc::c_int as size_t;
    version_string.buffer_length = 0 as libc::c_int as size_t;
    name_utf8_string.s = NULL as *mut libc::c_char;
    name_utf8_string.length = 0 as libc::c_int as size_t;
    name_utf8_string.buffer_length = 0 as libc::c_int as size_t;
    /* Prepare a ;123 suffix for the filename, where '123' is the version
     * value of this file. */
    archive_string_sprintf(
        &mut version_string as *mut archive_string,
        b";%zu\x00" as *const u8 as *const libc::c_char,
        version,
    );
    /* Build the new filename. */
    archive_strcat(&mut name_utf8_string, cur_filename as *const libc::c_void);
    archive_strcat(
        &mut name_utf8_string,
        version_string.s as *const libc::c_void,
    );
    /* Apply the new filename into this file's context. */
    archive_entry_update_pathname_utf8(e, name_utf8_string.s);
    /* Free buffers. */
    archive_string_free(&mut version_string);
    archive_string_free(&mut name_utf8_string);
    return ARCHIVE_OK;
}
unsafe extern "C" fn parse_file_extra_htime(
    mut a: *mut archive_read,
    mut e: *mut archive_entry,
    mut rar: *mut rar5,
    mut extra_data_size: *mut ssize_t,
) -> libc::c_int {
    let mut unix_time: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut flags: size_t = 0 as libc::c_int as size_t;
    let mut value_len: size_t = 0;
    if read_var_sized(a, &mut flags, &mut value_len) == 0 {
        return ARCHIVE_EOF;
    }
    *extra_data_size =
        (*extra_data_size as libc::c_ulong).wrapping_sub(value_len) as ssize_t as ssize_t;
    if ARCHIVE_OK != consume(a, value_len as int64_t) {
        return ARCHIVE_EOF;
    }
    unix_time = (flags & IS_UNIX as libc::c_int as libc::c_ulong) as libc::c_char;
    if flags & HAS_MTIME as libc::c_int as libc::c_ulong != 0 {
        parse_htime_item(a, unix_time, &mut (*rar).file.e_mtime, extra_data_size);
        archive_entry_set_mtime(
            e,
            (*rar).file.e_mtime as time_t,
            0 as libc::c_int as libc::c_long,
        );
    }
    if flags & HAS_CTIME as libc::c_int as libc::c_ulong != 0 {
        parse_htime_item(a, unix_time, &mut (*rar).file.e_ctime, extra_data_size);
        archive_entry_set_ctime(
            e,
            (*rar).file.e_ctime as time_t,
            0 as libc::c_int as libc::c_long,
        );
    }
    if flags & HAS_ATIME as libc::c_int as libc::c_ulong != 0 {
        parse_htime_item(a, unix_time, &mut (*rar).file.e_atime, extra_data_size);
        archive_entry_set_atime(
            e,
            (*rar).file.e_atime as time_t,
            0 as libc::c_int as libc::c_long,
        );
    }
    if flags & HAS_UNIX_NS as libc::c_int as libc::c_ulong != 0 {
        if read_u32(a, &mut (*rar).file.e_unix_ns) == 0 {
            return ARCHIVE_EOF;
        }
        *extra_data_size -= 4 as libc::c_int as libc::c_long
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn parse_file_extra_redir(
    mut a: *mut archive_read,
    mut e: *mut archive_entry,
    mut rar: *mut rar5,
    mut extra_data_size: *mut ssize_t,
) -> libc::c_int {
    let mut value_size: uint64_t = 0 as libc::c_int as uint64_t;
    let mut target_size: size_t = 0 as libc::c_int as size_t;
    let mut target_utf8_buf: [libc::c_char; 8192] = [0; 8192];
    let mut p: *const uint8_t = 0 as *const uint8_t;
    if read_var(a, &mut (*rar).file.redir_type, &mut value_size) == 0 {
        return ARCHIVE_EOF;
    }
    if ARCHIVE_OK != consume(a, value_size as int64_t) {
        return ARCHIVE_EOF;
    }
    *extra_data_size =
        (*extra_data_size as libc::c_ulong).wrapping_sub(value_size) as ssize_t as ssize_t;
    if read_var(a, &mut (*rar).file.redir_flags, &mut value_size) == 0 {
        return ARCHIVE_EOF;
    }
    if ARCHIVE_OK != consume(a, value_size as int64_t) {
        return ARCHIVE_EOF;
    }
    *extra_data_size =
        (*extra_data_size as libc::c_ulong).wrapping_sub(value_size) as ssize_t as ssize_t;
    if read_var_sized(a, &mut target_size, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    *extra_data_size = (*extra_data_size as libc::c_ulong)
        .wrapping_sub(target_size.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as ssize_t as ssize_t;
    if read_ahead(a, target_size, &mut p) == 0 {
        return ARCHIVE_EOF;
    }
    if target_size > (MAX_NAME_IN_CHARS - 1 as libc::c_int) as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Link target is too long\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    if target_size == 0 as libc::c_int as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"No link target specified\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    memcpy(
        target_utf8_buf.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        target_size,
    );
    target_utf8_buf[target_size as usize] = 0 as libc::c_int as libc::c_char;
    if ARCHIVE_OK != consume(a, target_size as int64_t) {
        return ARCHIVE_EOF;
    }
    match (*rar).file.redir_type {
        1 | 2 => {
            archive_entry_set_filetype(e, AE_IFLNK as mode_t);
            archive_entry_update_symlink_utf8(e, target_utf8_buf.as_mut_ptr());
            if (*rar).file.redir_flags & REDIR_SYMLINK_IS_DIR as libc::c_ulong != 0 {
                archive_entry_set_symlink_type(e, AE_SYMLINK_TYPE_DIRECTORY);
            } else {
                archive_entry_set_symlink_type(e, AE_SYMLINK_TYPE_FILE);
            }
        }
        4 => {
            archive_entry_set_filetype(e, AE_IFREG as mode_t);
            archive_entry_update_hardlink_utf8(e, target_utf8_buf.as_mut_ptr());
        }
        _ => {}
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn parse_file_extra_owner(
    mut a: *mut archive_read,
    mut e: *mut archive_entry,
    mut extra_data_size: *mut ssize_t,
) -> libc::c_int {
    let mut flags: uint64_t = 0 as libc::c_int as uint64_t;
    let mut value_size: uint64_t = 0 as libc::c_int as uint64_t;
    let mut id: uint64_t = 0 as libc::c_int as uint64_t;
    let mut name_len: size_t = 0 as libc::c_int as size_t;
    let mut name_size: size_t = 0 as libc::c_int as size_t;
    let mut namebuf: [libc::c_char; 256] = [0; 256];
    let mut p: *const uint8_t = 0 as *const uint8_t;
    if read_var(a, &mut flags, &mut value_size) == 0 {
        return ARCHIVE_EOF;
    }
    if ARCHIVE_OK != consume(a, value_size as int64_t) {
        return ARCHIVE_EOF;
    }
    *extra_data_size =
        (*extra_data_size as libc::c_ulong).wrapping_sub(value_size) as ssize_t as ssize_t;
    if flags & OWNER_USER_NAME as libc::c_ulong != 0 as libc::c_int as libc::c_ulong {
        if read_var_sized(a, &mut name_size, NULL as *mut size_t) == 0 {
            return ARCHIVE_EOF;
        }
        *extra_data_size = (*extra_data_size as libc::c_ulong)
            .wrapping_sub(name_size.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as ssize_t as ssize_t;
        if read_ahead(a, name_size, &mut p) == 0 {
            return ARCHIVE_EOF;
        }
        if name_size >= OWNER_MAXNAMELEN as libc::c_ulong {
            name_len = (OWNER_MAXNAMELEN - 1 as libc::c_int) as size_t
        } else {
            name_len = name_size
        }
        memcpy(
            namebuf.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            name_len,
        );
        namebuf[name_len as usize] = 0 as libc::c_int as libc::c_char;
        if ARCHIVE_OK != consume(a, name_size as int64_t) {
            return ARCHIVE_EOF;
        }
        archive_entry_set_uname(e, namebuf.as_mut_ptr());
    }
    if flags & OWNER_GROUP_NAME as libc::c_ulong != 0 as libc::c_int as libc::c_ulong {
        if read_var_sized(a, &mut name_size, NULL as *mut size_t) == 0 {
            return ARCHIVE_EOF;
        }
        *extra_data_size = (*extra_data_size as libc::c_ulong)
            .wrapping_sub(name_size.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as ssize_t as ssize_t;
        if read_ahead(a, name_size, &mut p) == 0 {
            return ARCHIVE_EOF;
        }
        if name_size >= OWNER_MAXNAMELEN as libc::c_ulong {
            name_len = (OWNER_MAXNAMELEN - 1 as libc::c_int) as size_t
        } else {
            name_len = name_size
        }
        memcpy(
            namebuf.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            name_len,
        );
        namebuf[name_len as usize] = 0 as libc::c_int as libc::c_char;
        if ARCHIVE_OK != consume(a, name_size as int64_t) {
            return ARCHIVE_EOF;
        }
        archive_entry_set_gname(e, namebuf.as_mut_ptr());
    }
    if flags & OWNER_USER_UID as libc::c_ulong != 0 as libc::c_int as libc::c_ulong {
        if read_var(a, &mut id, &mut value_size) == 0 {
            return ARCHIVE_EOF;
        }
        if ARCHIVE_OK != consume(a, value_size as int64_t) {
            return ARCHIVE_EOF;
        }
        *extra_data_size =
            (*extra_data_size as libc::c_ulong).wrapping_sub(value_size) as ssize_t as ssize_t;
        archive_entry_set_uid(e, id as la_int64_t);
    }
    if flags & OWNER_GROUP_GID as libc::c_ulong != 0 as libc::c_int as libc::c_ulong {
        if read_var(a, &mut id, &mut value_size) == 0 {
            return ARCHIVE_EOF;
        }
        if ARCHIVE_OK != consume(a, value_size as int64_t) {
            return ARCHIVE_EOF;
        }
        *extra_data_size =
            (*extra_data_size as libc::c_ulong).wrapping_sub(value_size) as ssize_t as ssize_t;
        archive_entry_set_gid(e, id as la_int64_t);
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn process_head_file_extra(
    mut a: *mut archive_read,
    mut e: *mut archive_entry,
    mut rar: *mut rar5,
    mut extra_data_size: ssize_t,
) -> libc::c_int {
    let mut extra_field_size: size_t = 0;
    let mut extra_field_id: size_t = 0 as libc::c_int as size_t;
    let mut ret: libc::c_int = ARCHIVE_FATAL;
    let mut var_size: size_t = 0;
    while extra_data_size > 0 as libc::c_int as libc::c_long {
        if read_var_sized(a, &mut extra_field_size, &mut var_size) == 0 {
            return ARCHIVE_EOF;
        }
        extra_data_size =
            (extra_data_size as libc::c_ulong).wrapping_sub(var_size) as ssize_t as ssize_t;
        if ARCHIVE_OK != consume(a, var_size as int64_t) {
            return ARCHIVE_EOF;
        }
        if read_var_sized(a, &mut extra_field_id, &mut var_size) == 0 {
            return ARCHIVE_EOF;
        }
        extra_data_size =
            (extra_data_size as libc::c_ulong).wrapping_sub(var_size) as ssize_t as ssize_t;
        if ARCHIVE_OK != consume(a, var_size as int64_t) {
            return ARCHIVE_EOF;
        }
        let mut current_block_17: u64;
        match extra_field_id {
            2 => {
                ret = parse_file_extra_hash(a, rar, &mut extra_data_size);
                current_block_17 = 5634871135123216486;
            }
            3 => {
                ret = parse_file_extra_htime(a, e, rar, &mut extra_data_size);
                current_block_17 = 5634871135123216486;
            }
            5 => {
                ret = parse_file_extra_redir(a, e, rar, &mut extra_data_size);
                current_block_17 = 5634871135123216486;
            }
            6 => {
                ret = parse_file_extra_owner(a, e, &mut extra_data_size);
                current_block_17 = 5634871135123216486;
            }
            4 => {
                ret = parse_file_extra_version(a, e, &mut extra_data_size);
                current_block_17 = 5634871135123216486;
            }
            7 => {
                /* fallthrough */
                current_block_17 = 9636827501879935959;
            }
            1 | _ => {
                current_block_17 = 9636827501879935959;
            }
        }
        match current_block_17 {
            5634871135123216486 => {}
            _ =>
            /* fallthrough */
            /* Skip unsupported entry. */
            {
                return consume(a, extra_data_size)
            }
        }
    }
    if ret != ARCHIVE_OK {
        /* Attribute not implemented. */
        return ret;
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn process_head_file(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut entry: *mut archive_entry,
    mut block_flags: size_t,
) -> libc::c_int {
    let mut extra_data_size: ssize_t = 0 as libc::c_int as ssize_t;
    let mut data_size: size_t = 0 as libc::c_int as size_t;
    let mut file_flags: size_t = 0 as libc::c_int as size_t;
    let mut file_attr: size_t = 0 as libc::c_int as size_t;
    let mut compression_info: size_t = 0 as libc::c_int as size_t;
    let mut host_os: size_t = 0 as libc::c_int as size_t;
    let mut name_size: size_t = 0 as libc::c_int as size_t;
    let mut unpacked_size: uint64_t = 0;
    let mut window_size: uint64_t = 0;
    let mut mtime: uint32_t = 0 as libc::c_int as uint32_t;
    let mut crc: uint32_t = 0 as libc::c_int as uint32_t;
    let mut c_method: libc::c_int = 0 as libc::c_int;
    let mut c_version: libc::c_int = 0 as libc::c_int;
    let mut name_utf8_buf: [libc::c_char; 8192] = [0; 8192];
    let mut p: *const uint8_t = 0 as *const uint8_t;
    archive_entry_clear(entry);
    /* Do not reset file context if we're switching archives. */
    if (*rar).cstate.switch_multivolume() == 0 {
        reset_file_context(rar);
    }
    if block_flags & HFL_EXTRA_DATA as libc::c_int as libc::c_ulong != 0 {
        let mut edata_size: size_t = 0 as libc::c_int as size_t;
        if read_var_sized(a, &mut edata_size, NULL as *mut size_t) == 0 {
            return ARCHIVE_EOF;
        }
        /* Intentional type cast from unsigned to signed. */
        extra_data_size = edata_size as ssize_t
    }
    if block_flags & HFL_DATA as libc::c_int as libc::c_ulong != 0 {
        if read_var_sized(a, &mut data_size, NULL as *mut size_t) == 0 {
            return ARCHIVE_EOF;
        }
        (*rar).file.bytes_remaining = data_size as ssize_t
    } else {
        (*rar).file.bytes_remaining = 0 as libc::c_int as ssize_t;
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"no data found in file/service block\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    if read_var_sized(a, &mut file_flags, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    if read_var(a, &mut unpacked_size, NULL as *mut uint64_t) == 0 {
        return ARCHIVE_EOF;
    }
    if file_flags & UNKNOWN_UNPACKED_SIZE as libc::c_int as libc::c_ulong != 0 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Files with unknown unpacked size are not supported\x00" as *const u8
                as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    (*rar).file.set_dir(
        (file_flags & DIRECTORY as libc::c_int as libc::c_ulong > 0 as libc::c_int as libc::c_ulong)
            as libc::c_int as uint8_t,
    );
    if read_var_sized(a, &mut file_attr, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    if file_flags & UTIME as libc::c_int as libc::c_ulong != 0 {
        if read_u32(a, &mut mtime) == 0 {
            return ARCHIVE_EOF;
        }
    }
    if file_flags & CRC32 as libc::c_int as libc::c_ulong != 0 {
        if read_u32(a, &mut crc) == 0 {
            return ARCHIVE_EOF;
        }
    }
    if read_var_sized(a, &mut compression_info, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    c_method = (compression_info >> 7 as libc::c_int) as libc::c_int & 0x7 as libc::c_int;
    c_version = (compression_info & 0x3f as libc::c_int as libc::c_ulong) as libc::c_int;
    /* RAR5 seems to limit the dictionary size to 64MB. */
    window_size = if (*rar).file.dir() as libc::c_int > 0 as libc::c_int {
        0 as libc::c_int as libc::c_ulong
    } else {
        (g_unpack_window_size)
            << (compression_info >> 10 as libc::c_int & 15 as libc::c_int as libc::c_ulong)
    };
    (*rar).cstate.method = c_method;
    (*rar).cstate.version = c_version + 50 as libc::c_int;
    (*rar).file.set_solid(
        (compression_info & SOLID as libc::c_int as libc::c_ulong
            > 0 as libc::c_int as libc::c_ulong) as libc::c_int as uint8_t,
    );
    /* Archives which declare solid files without initializing the window
     * buffer first are invalid. */
    if (*rar).file.solid() as libc::c_int > 0 as libc::c_int && (*rar).cstate.window_buf.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Declared solid file, but no window buffer initialized yet.\x00" as *const u8
                as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    /* Check if window_size is a sane value. Also, if the file is not
     * declared as a directory, disallow window_size == 0. */
    if window_size
        > (64 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        || (*rar).file.dir() as libc::c_int == 0 as libc::c_int
            && window_size == 0 as libc::c_int as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Declared dictionary size is not supported.\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    if (*rar).file.solid() as libc::c_int > 0 as libc::c_int {
        /* Re-check if current window size is the same as previous
         * window size (for solid files only). */
        if (*rar).file.solid_window_size > 0 as libc::c_int as libc::c_long
            && (*rar).file.solid_window_size != window_size as ssize_t
        {
            archive_set_error(&mut (*a).archive as *mut archive,
                              ARCHIVE_ERRNO_FILE_FORMAT,
                              b"Window size for this solid file doesn\'t match the window size used in previous solid file. \x00"
                                  as *const u8 as *const libc::c_char);
            return ARCHIVE_FATAL;
        }
    }
    /* If we're currently switching volumes, ignore the new definition of
     * window_size. */
    if (*rar).cstate.switch_multivolume() as libc::c_int == 0 as libc::c_int {
        /* Values up to 64M should fit into ssize_t on every
         * architecture. */
        (*rar).cstate.window_size = window_size as ssize_t
    }
    if (*rar).file.solid() as libc::c_int > 0 as libc::c_int
        && (*rar).file.solid_window_size == 0 as libc::c_int as libc::c_long
    {
        /* Solid files have to have the same window_size across
        whole archive. Remember the window_size parameter
        for first solid file found. */
        (*rar).file.solid_window_size = (*rar).cstate.window_size
    }
    init_window_mask(rar);
    (*rar).file.set_service(0 as libc::c_int as uint8_t);
    if read_var_sized(a, &mut host_os, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    if host_os == HOST_WINDOWS as libc::c_int as libc::c_ulong {
        /* Host OS is Windows */
        let mut mode: mode_t = 0;
        if file_attr & ATTR_DIRECTORY as libc::c_int as libc::c_ulong != 0 {
            if file_attr & ATTR_READONLY as libc::c_int as libc::c_ulong != 0 {
                mode = 0o555 as libc::c_int as libc::c_uint | AE_IFDIR as mode_t
            } else {
                mode = 0o755 as libc::c_int as libc::c_uint | AE_IFDIR as mode_t
            }
        } else if file_attr & ATTR_READONLY as libc::c_int as libc::c_ulong != 0 {
            mode = 0o444 as libc::c_int as libc::c_uint | AE_IFREG as mode_t
        } else {
            mode = 0o644 as libc::c_int as libc::c_uint | AE_IFREG as mode_t
        }
        archive_entry_set_mode(entry, mode);
        if file_attr
            & (ATTR_READONLY as libc::c_int
                | ATTR_HIDDEN as libc::c_int
                | ATTR_SYSTEM as libc::c_int) as libc::c_ulong
            != 0
        {
            let mut fflags_text: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
            /* allocate for "rdonly,hidden,system," */
            fflags_text = malloc(
                (22 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            if !fflags_text.is_null() {
                ptr = fflags_text;
                if file_attr & ATTR_READONLY as libc::c_int as libc::c_ulong != 0 {
                    strcpy(ptr, b"rdonly,\x00" as *const u8 as *const libc::c_char);
                    ptr = ptr.offset(7 as libc::c_int as isize)
                }
                if file_attr & ATTR_HIDDEN as libc::c_int as libc::c_ulong != 0 {
                    strcpy(ptr, b"hidden,\x00" as *const u8 as *const libc::c_char);
                    ptr = ptr.offset(7 as libc::c_int as isize)
                }
                if file_attr & ATTR_SYSTEM as libc::c_int as libc::c_ulong != 0 {
                    strcpy(ptr, b"system,\x00" as *const u8 as *const libc::c_char);
                    ptr = ptr.offset(7 as libc::c_int as isize)
                }
                if ptr > fflags_text {
                    /* Delete trailing comma */
                    *ptr.offset(-(1 as libc::c_int as isize)) = '\u{0}' as i32 as libc::c_char;
                    archive_entry_copy_fflags_text(entry, fflags_text);
                }
                free(fflags_text as *mut libc::c_void);
            }
        }
    } else if host_os == HOST_UNIX as libc::c_int as libc::c_ulong {
        /* Host OS is Unix */
        archive_entry_set_mode(entry, file_attr as mode_t);
    } else {
        /* Unknown host OS */
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Unsupported Host OS: 0x%x\x00" as *const u8 as *const libc::c_char,
            host_os as libc::c_int,
        );
        return ARCHIVE_FATAL;
    }
    if read_var_sized(a, &mut name_size, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    if read_ahead(a, name_size, &mut p) == 0 {
        return ARCHIVE_EOF;
    }
    if name_size > (MAX_NAME_IN_CHARS - 1 as libc::c_int) as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Filename is too long\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    if name_size == 0 as libc::c_int as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"No filename specified\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    memcpy(
        name_utf8_buf.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        name_size,
    );
    name_utf8_buf[name_size as usize] = 0 as libc::c_int as libc::c_char;
    if ARCHIVE_OK != consume(a, name_size as int64_t) {
        return ARCHIVE_EOF;
    }
    archive_entry_update_pathname_utf8(entry, name_utf8_buf.as_mut_ptr());
    if extra_data_size > 0 as libc::c_int as libc::c_long {
        let mut ret: libc::c_int = process_head_file_extra(a, entry, rar, extra_data_size);
        /*
         * TODO: rewrite or remove useless sanity check
         *       as extra_data_size is not passed as a pointer
         *
        if(extra_data_size < 0) {
            archive_set_error(&a->archive, ARCHIVE_ERRNO_PROGRAMMER,
                "File extra data size is not zero");
            return ARCHIVE_FATAL;
        }
         */
        if ret != ARCHIVE_OK {
            return ret;
        }
    }
    if file_flags & UNKNOWN_UNPACKED_SIZE as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        (*rar).file.unpacked_size = unpacked_size as ssize_t;
        if (*rar).file.redir_type == REDIR_TYPE_NONE as libc::c_int as libc::c_ulong {
            archive_entry_set_size(entry, unpacked_size as la_int64_t);
        }
    }
    if file_flags & UTIME as libc::c_int as libc::c_ulong != 0 {
        archive_entry_set_mtime(entry, mtime as time_t, 0 as libc::c_int as libc::c_long);
    }
    if file_flags & CRC32 as libc::c_int as libc::c_ulong != 0 {
        (*rar).file.stored_crc32 = crc
    }
    if (*rar).cstate.switch_multivolume() == 0 {
        /* Do not reinitialize unpacking state if we're switching
         * archives. */
        (*rar)
            .cstate
            .set_block_parsing_finished(1 as libc::c_int as uint8_t);
        (*rar)
            .cstate
            .set_all_filters_applied(1 as libc::c_int as uint8_t);
        (*rar).cstate.set_initialized(0 as libc::c_int as uint8_t)
    }
    if (*rar).generic.split_before() as libc::c_int > 0 as libc::c_int {
        /* If now we're standing on a header that has a 'split before'
         * mark, it means we're standing on a 'continuation' file
         * header. Signal the caller that if it wants to move to
         * another file, it must call rar5_read_header() function
         * again. */
        return ARCHIVE_RETRY;
    } else {
        return ARCHIVE_OK;
    };
}
unsafe extern "C" fn process_head_service(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut entry: *mut archive_entry,
    mut block_flags: size_t,
) -> libc::c_int {
    /* Process this SERVICE block the same way as FILE blocks. */
    let mut ret: libc::c_int = process_head_file(a, rar, entry, block_flags);
    if ret != ARCHIVE_OK {
        return ret;
    }
    (*rar).file.set_service(1 as libc::c_int as uint8_t);
    /* But skip the data part automatically. It's no use for the user
     * anyway.  It contains only service data, not even needed to
     * properly unpack the file. */
    ret = rar5_read_data_skip(a);
    if ret != ARCHIVE_OK {
        return ret;
    }
    /* After skipping, try parsing another block automatically. */
    return ARCHIVE_RETRY;
}
unsafe extern "C" fn process_head_main(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut entry: *mut archive_entry,
    mut block_flags: size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut extra_data_size: size_t = 0 as libc::c_int as size_t;
    let mut extra_field_size: size_t = 0 as libc::c_int as size_t;
    let mut extra_field_id: size_t = 0 as libc::c_int as size_t;
    let mut archive_flags: size_t = 0 as libc::c_int as size_t;
    if block_flags & HFL_EXTRA_DATA as libc::c_int as libc::c_ulong != 0 {
        if read_var_sized(a, &mut extra_data_size, NULL as *mut size_t) == 0 {
            return ARCHIVE_EOF;
        }
    } else {
        extra_data_size = 0 as libc::c_int as size_t
    }
    if read_var_sized(a, &mut archive_flags, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    (*rar).main.set_volume(
        (archive_flags & VOLUME as libc::c_int as libc::c_ulong > 0 as libc::c_int as libc::c_ulong)
            as libc::c_int as uint8_t,
    );
    (*rar).main.set_solid(
        (archive_flags & SOLID_0 as libc::c_int as libc::c_ulong
            > 0 as libc::c_int as libc::c_ulong) as libc::c_int as uint8_t,
    );
    if archive_flags & VOLUME_NUMBER as libc::c_int as libc::c_ulong != 0 {
        let mut v: size_t = 0 as libc::c_int as size_t;
        if read_var_sized(a, &mut v, NULL as *mut size_t) == 0 {
            return ARCHIVE_EOF;
        }
        if v > (__INT_MAX__ as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_ulong
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Invalid volume number\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        (*rar).main.vol_no = v as libc::c_uint
    } else {
        (*rar).main.vol_no = 0 as libc::c_int as libc::c_uint
    }
    if (*rar).vol.expected_vol_no > 0 as libc::c_int as libc::c_uint
        && (*rar).main.vol_no != (*rar).vol.expected_vol_no
    {
        /* Returning EOF instead of FATAL because of strange
         * libarchive behavior. When opening multiple files via
         * archive_read_open_filenames(), after reading up the whole
         * last file, the __archive_read_ahead function wraps up to
         * the first archive instead of returning EOF. */
        return ARCHIVE_EOF;
    }
    if extra_data_size == 0 as libc::c_int as libc::c_ulong {
        /* Early return. */
        return ARCHIVE_OK;
    }
    if read_var_sized(a, &mut extra_field_size, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    if read_var_sized(a, &mut extra_field_id, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    if extra_field_size == 0 as libc::c_int as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid extra field size\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    match extra_field_id {
        1 => {
            ret = process_main_locator_extra_block(a, rar);
            if ret != ARCHIVE_OK {
                /* Error while parsing main locator extra
                 * block. */
                return ret;
            }
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unsupported extra type (0x%x)\x00" as *const u8 as *const libc::c_char,
                extra_field_id as libc::c_int,
            );
            return ARCHIVE_FATAL;
        }
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn skip_unprocessed_bytes(mut a: *mut archive_read) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    let mut ret: libc::c_int = 0;
    if (*rar).file.bytes_remaining != 0 {
        /* Use different skipping method in block merging mode than in
         * normal mode. If merge mode is active, rar5_read_data_skip
         * can't be used, because it could allow recursive use of
         * merge_block() * function, and this function doesn't support
         * recursive use. */
        if (*rar).merge_mode != 0 {
            /* Discard whole merged block. This is valid in solid
             * mode as well, because the code will discard blocks
             * only if those blocks are safe to discard (i.e.
             * they're not FILE blocks).  */
            ret = consume(a, (*rar).file.bytes_remaining);
            if ret != ARCHIVE_OK {
                return ret;
            }
            (*rar).file.bytes_remaining = 0 as libc::c_int as ssize_t
        } else {
            /* If we're not in merge mode, use safe skipping code.
             * This will ensure we'll handle solid archives
             * properly. */
            ret = rar5_read_data_skip(a);
            if ret != ARCHIVE_OK {
                return ret;
            }
        }
    }
    return ARCHIVE_OK;
}
/* Base block processing function. A 'base block' is a RARv5 header block
 * that tells the reader what kind of data is stored inside the block.
 *
 * From the birds-eye view a RAR file looks file this:
 *
 * <magic><base_block_1><base_block_2>...<base_block_n>
 *
 * There are a few types of base blocks. Those types are specified inside
 * the 'switch' statement in this function. For example purposes, I'll write
 * how a standard RARv5 file could look like here:
 *
 * <magic><MAIN><FILE><FILE><FILE><SERVICE><ENDARC>
 *
 * The structure above could describe an archive file with 3 files in it,
 * one service "QuickOpen" block (that is ignored by this parser), and an
 * end of file base block marker.
 *
 * If the file is stored in multiple archive files ("multiarchive"), it might
 * look like this:
 *
 * .part01.rar: <magic><MAIN><FILE><ENDARC>
 * .part02.rar: <magic><MAIN><FILE><ENDARC>
 * .part03.rar: <magic><MAIN><FILE><ENDARC>
 *
 * This example could describe 3 RAR files that contain ONE archived file.
 * Or it could describe 3 RAR files that contain 3 different files. Or 3
 * RAR files than contain 2 files. It all depends what metadata is stored in
 * the headers of <FILE> blocks.
 *
 * Each <FILE> block contains info about its size, the name of the file it's
 * storing inside, and whether this FILE block is a continuation block of
 * previous archive ('split before'), and is this FILE block should be
 * continued in another archive ('split after'). By parsing the 'split before'
 * and 'split after' flags, we're able to tell if multiple <FILE> base blocks
 * are describing one file, or multiple files (with the same filename, for
 * example).
 *
 * One thing to note is that if we're parsing the first <FILE> block, and
 * we see 'split after' flag, then we need to jump over to another <FILE>
 * block to be able to decompress rest of the data. To do this, we need
 * to skip the <ENDARC> block, then switch to another file, then skip the
 * <magic> block, <MAIN> block, and then we're standing on the proper
 * <FILE> block.
 */
unsafe extern "C" fn process_base_block(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let SMALLEST_RAR5_BLOCK_SIZE: size_t = 3 as libc::c_int as size_t;
    let mut rar: *mut rar5 = get_context(a);
    let mut hdr_crc: uint32_t = 0;
    let mut computed_crc: uint32_t = 0;
    let mut raw_hdr_size: size_t = 0 as libc::c_int as size_t;
    let mut hdr_size_len: size_t = 0;
    let mut hdr_size: size_t = 0;
    let mut header_id: size_t = 0 as libc::c_int as size_t;
    let mut header_flags: size_t = 0 as libc::c_int as size_t;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut ret: libc::c_int = 0;
    /* Skip any unprocessed data for this file. */
    ret = skip_unprocessed_bytes(a);
    if ret != ARCHIVE_OK {
        return ret;
    }
    /* Read the expected CRC32 checksum. */
    if read_u32(a, &mut hdr_crc) == 0 {
        return ARCHIVE_EOF;
    }
    /* Read header size. */
    if read_var_sized(a, &mut raw_hdr_size, &mut hdr_size_len) == 0 {
        return ARCHIVE_EOF;
    }
    hdr_size = raw_hdr_size.wrapping_add(hdr_size_len);
    /* Sanity check, maximum header size for RAR5 is 2MB. */
    if hdr_size > (2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Base block header is too large\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    /* Additional sanity checks to weed out invalid files. */
    if raw_hdr_size == 0 as libc::c_int as libc::c_ulong
        || hdr_size_len == 0 as libc::c_int as libc::c_ulong
        || hdr_size < SMALLEST_RAR5_BLOCK_SIZE
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Too small block encountered (%zu bytes)\x00" as *const u8 as *const libc::c_char,
            raw_hdr_size,
        );
        return ARCHIVE_FATAL;
    }
    /* Read the whole header data into memory, maximum memory use here is
     * 2MB. */
    if read_ahead(a, hdr_size, &mut p) == 0 {
        return ARCHIVE_EOF;
    }
    /* Verify the CRC32 of the header data. */
    computed_crc = crc32(
        0 as libc::c_int as uLong,
        p,
        hdr_size as libc::c_int as uInt,
    ) as uint32_t;
    if computed_crc != hdr_crc {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Header CRC error\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    /* If the checksum is OK, we proceed with parsing. */
    if ARCHIVE_OK != consume(a, hdr_size_len as int64_t) {
        return ARCHIVE_EOF;
    }
    if read_var_sized(a, &mut header_id, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    if read_var_sized(a, &mut header_flags, NULL as *mut size_t) == 0 {
        return ARCHIVE_EOF;
    }
    (*rar).generic.set_split_after(
        (header_flags & HFL_SPLIT_AFTER as libc::c_int as libc::c_ulong
            > 0 as libc::c_int as libc::c_ulong) as libc::c_int as uint8_t,
    );
    (*rar).generic.set_split_before(
        (header_flags & HFL_SPLIT_BEFORE as libc::c_int as libc::c_ulong
            > 0 as libc::c_int as libc::c_ulong) as libc::c_int as uint8_t,
    );
    (*rar).generic.size = hdr_size as libc::c_int;
    (*rar).generic.last_header_id = header_id as libc::c_int;
    (*rar).main.set_endarc(0 as libc::c_int as uint8_t);
    /* Those are possible header ids in RARv5. */
    match header_id {
        1 => {
            ret = process_head_main(a, rar, entry, header_flags);
            /* Main header doesn't have any files in it, so it's
             * pointless to return to the caller. Retry to next
             * header, which should be HEAD_FILE/HEAD_SERVICE. */
            if ret == ARCHIVE_OK {
                return ARCHIVE_RETRY;
            }
            return ret;
        }
        3 => {
            ret = process_head_service(a, rar, entry, header_flags);
            return ret;
        }
        2 => {
            ret = process_head_file(a, rar, entry, header_flags);
            return ret;
        }
        4 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Encryption is not supported\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        5 => {
            (*rar).main.set_endarc(1 as libc::c_int as uint8_t);
            /* After encountering an end of file marker, we need
             * to take into consideration if this archive is
             * continued in another file (i.e. is it part01.rar:
             * is there a part02.rar?) */
            if (*rar).main.volume() != 0 {
                /* In case there is part02.rar, position the
                 * read pointer in a proper place, so we can
                 * resume parsing. */
                ret = scan_for_signature(a);
                if ret == ARCHIVE_FATAL {
                    return ARCHIVE_EOF;
                } else {
                    if (*rar).vol.expected_vol_no
                        == (__INT_MAX__ as libc::c_uint)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint)
                    {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Header error\x00" as *const u8 as *const libc::c_char,
                        );
                        return ARCHIVE_FATAL;
                    }
                    (*rar).vol.expected_vol_no = (*rar)
                        .main
                        .vol_no
                        .wrapping_add(1 as libc::c_int as libc::c_uint);
                    return ARCHIVE_OK;
                }
            } else {
                return ARCHIVE_EOF;
            }
        }
        0 => return ARCHIVE_EOF,
        _ => {
            if header_flags & HFL_SKIP_IF_UNKNOWN as libc::c_int as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Header type error\x00" as *const u8 as *const libc::c_char,
                );
                return ARCHIVE_FATAL;
            } else {
                /* If the block is marked as 'skip if unknown',
                 * do as the flag says: skip the block
                 * instead on failing on it. */
                return ARCHIVE_RETRY;
            }
        }
    };
}
unsafe extern "C" fn skip_base_block(mut a: *mut archive_read) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut rar: *mut rar5 = get_context(a);
    /* Create a new local archive_entry structure that will be operated on
     * by header reader; operations on this archive_entry will be discarded.
     */
    let mut entry: *mut archive_entry = archive_entry_new();
    ret = process_base_block(a, entry);
    /* Discard operations on this archive_entry structure. */
    archive_entry_free(entry);
    if ret == ARCHIVE_FATAL {
        return ret;
    }
    if (*rar).generic.last_header_id == 2 as libc::c_int
        && (*rar).generic.split_before() as libc::c_int > 0 as libc::c_int
    {
        return ARCHIVE_OK;
    }
    if ret == ARCHIVE_OK {
        return ARCHIVE_RETRY;
    } else {
        return ret;
    };
}
unsafe extern "C" fn rar5_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    let mut ret: libc::c_int = 0;
    if (*rar).header_initialized == 0 as libc::c_int {
        init_header(a);
        (*rar).header_initialized = 1 as libc::c_int
    }
    if (*rar).skipped_magic == 0 as libc::c_int {
        if ARCHIVE_OK
            != consume(
                a,
                ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as int64_t,
            )
        {
            return ARCHIVE_EOF;
        }
        (*rar).skipped_magic = 1 as libc::c_int
    }
    loop {
        ret = process_base_block(a, entry);
        if !(ret == ARCHIVE_RETRY
            || (*rar).main.endarc() as libc::c_int > 0 as libc::c_int && ret == ARCHIVE_OK)
        {
            break;
        }
    }
    return ret;
}
unsafe extern "C" fn init_unpack(mut rar: *mut rar5) {
    (*rar).file.calculated_crc32 = 0 as libc::c_int as uint32_t;
    init_window_mask(rar);
    free((*rar).cstate.window_buf as *mut libc::c_void);
    free((*rar).cstate.filtered_buf as *mut libc::c_void);
    if (*rar).cstate.window_size > 0 as libc::c_int as libc::c_long {
        (*rar).cstate.window_buf = calloc(
            1 as libc::c_int as libc::c_ulong,
            (*rar).cstate.window_size as libc::c_ulong,
        ) as *mut uint8_t;
        (*rar).cstate.filtered_buf = calloc(
            1 as libc::c_int as libc::c_ulong,
            (*rar).cstate.window_size as libc::c_ulong,
        ) as *mut uint8_t
    } else {
        (*rar).cstate.window_buf = NULL as *mut uint8_t;
        (*rar).cstate.filtered_buf = NULL as *mut uint8_t
    }
    (*rar).cstate.write_ptr = 0 as libc::c_int as int64_t;
    (*rar).cstate.last_write_ptr = 0 as libc::c_int as int64_t;
    memset(
        &mut (*rar).cstate.bd as *mut decode_table as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<decode_table>() as libc::c_ulong,
    );
    memset(
        &mut (*rar).cstate.ld as *mut decode_table as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<decode_table>() as libc::c_ulong,
    );
    memset(
        &mut (*rar).cstate.dd as *mut decode_table as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<decode_table>() as libc::c_ulong,
    );
    memset(
        &mut (*rar).cstate.ldd as *mut decode_table as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<decode_table>() as libc::c_ulong,
    );
    memset(
        &mut (*rar).cstate.rd as *mut decode_table as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<decode_table>() as libc::c_ulong,
    );
}
unsafe extern "C" fn update_crc(mut rar: *mut rar5, mut p: *const uint8_t, mut to_read: size_t) {
    let mut verify_crc: libc::c_int = 0;
    if (*rar).skip_mode != 0 {
        verify_crc = 0 as libc::c_int
    } else {
        verify_crc = 1 as libc::c_int
    }
    if verify_crc != 0 {
        /* Don't update CRC32 if the file doesn't have the
         * `stored_crc32` info filled in. */
        if (*rar).file.stored_crc32 > 0 as libc::c_int as libc::c_uint {
            (*rar).file.calculated_crc32 =
                crc32((*rar).file.calculated_crc32 as uLong, p, to_read as uInt) as uint32_t
        }
        /* Check if the file uses an optional BLAKE2sp checksum
         * algorithm. */
        if (*rar).file.has_blake2 as libc::c_int > 0 as libc::c_int {
            /* Return value of the `update` function is always 0,
             * so we can explicitly ignore it here. */
            blake2sp_update(&mut (*rar).file.b2state, p as *const libc::c_void, to_read);
        }
    };
}
unsafe extern "C" fn create_decode_tables(
    mut bit_length: *mut uint8_t,
    mut table: *mut decode_table,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut code: libc::c_int = 0;
    let mut upper_limit: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut lc: [libc::c_int; 16] = [0; 16];
    let mut decode_pos_clone: [uint32_t; 16] = [0; 16];
    let mut cur_len: ssize_t = 0;
    let mut quick_data_size: ssize_t = 0;
    memset(
        &mut lc as *mut [libc::c_int; 16] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    memset(
        (*table).decode_num.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint16_t; 306]>() as libc::c_ulong,
    );
    (*table).size = size as uint32_t;
    (*table).quick_bits = if size == HUFF_NC {
        10 as libc::c_int
    } else {
        7 as libc::c_int
    } as uint32_t;
    i = 0 as libc::c_int;
    while i < size {
        lc[(*bit_length.offset(i as isize) as libc::c_int & 15 as libc::c_int) as usize] += 1;
        i += 1
    }
    lc[0 as libc::c_int as usize] = 0 as libc::c_int;
    (*table).decode_pos[0 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    (*table).decode_len[0 as libc::c_int as usize] = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 16 as libc::c_int {
        upper_limit += lc[i as usize];
        (*table).decode_len[i as usize] = upper_limit << 16 as libc::c_int - i;
        (*table).decode_pos[i as usize] = (*table).decode_pos[(i - 1 as libc::c_int) as usize]
            .wrapping_add(lc[(i - 1 as libc::c_int) as usize] as libc::c_uint);
        upper_limit <<= 1 as libc::c_int;
        i += 1
    }
    memcpy(
        decode_pos_clone.as_mut_ptr() as *mut libc::c_void,
        (*table).decode_pos.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint32_t; 16]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < size {
        let mut clen: uint8_t =
            (*bit_length.offset(i as isize) as libc::c_int & 15 as libc::c_int) as uint8_t;
        if clen as libc::c_int > 0 as libc::c_int {
            let mut last_pos: libc::c_int = decode_pos_clone[clen as usize] as libc::c_int;
            (*table).decode_num[last_pos as usize] = i as uint16_t;
            decode_pos_clone[clen as usize] = decode_pos_clone[clen as usize].wrapping_add(1)
        }
        i += 1
    }
    quick_data_size = (1 as libc::c_int as int64_t) << (*table).quick_bits;
    cur_len = 1 as libc::c_int as ssize_t;
    code = 0 as libc::c_int;
    while (code as libc::c_long) < quick_data_size {
        let mut bit_field: libc::c_int =
            code << (16 as libc::c_int as libc::c_uint).wrapping_sub((*table).quick_bits);
        let mut dist: libc::c_int = 0;
        let mut pos: libc::c_int = 0;
        while cur_len
            < (::std::mem::size_of::<[int32_t; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<int32_t>() as libc::c_ulong)
                as ssize_t
            && bit_field >= (*table).decode_len[cur_len as usize]
        {
            cur_len += 1
        }
        (*table).quick_len[code as usize] = cur_len as uint8_t;
        dist =
            bit_field - (*table).decode_len[(cur_len - 1 as libc::c_int as libc::c_long) as usize];
        dist >>= 16 as libc::c_int as libc::c_long - cur_len;
        pos = (*table).decode_pos[(cur_len & 15 as libc::c_int as libc::c_long) as usize]
            .wrapping_add(dist as libc::c_uint) as libc::c_int;
        if cur_len
            < (::std::mem::size_of::<[uint32_t; 16]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
                as ssize_t
            && pos < size
        {
            (*table).quick_num[code as usize] = (*table).decode_num[pos as usize]
        } else {
            (*table).quick_num[code as usize] = 0 as libc::c_int as uint16_t
        }
        code += 1
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn decode_number(
    mut a: *mut archive_read,
    mut table: *mut decode_table,
    mut p: *const uint8_t,
    mut num: *mut uint16_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut dist: libc::c_int = 0;
    let mut bitfield: uint16_t = 0;
    let mut pos: uint32_t = 0;
    let mut rar: *mut rar5 = get_context(a);
    if ARCHIVE_OK != read_bits_16(rar, p, &mut bitfield) {
        return ARCHIVE_EOF;
    }
    bitfield = (bitfield as libc::c_int & 0xfffe as libc::c_int) as uint16_t;
    if (bitfield as libc::c_int) < (*table).decode_len[(*table).quick_bits as usize] {
        let mut code: libc::c_int = bitfield as libc::c_int
            >> (16 as libc::c_int as libc::c_uint).wrapping_sub((*table).quick_bits);
        skip_bits(rar, (*table).quick_len[code as usize] as libc::c_int);
        *num = (*table).quick_num[code as usize];
        return ARCHIVE_OK;
    }
    bits = 15 as libc::c_int;
    i = (*table)
        .quick_bits
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    while i < 15 as libc::c_int {
        if (bitfield as libc::c_int) < (*table).decode_len[i as usize] {
            bits = i;
            break;
        } else {
            i += 1
        }
    }
    skip_bits(rar, bits);
    dist = bitfield as libc::c_int - (*table).decode_len[(bits - 1 as libc::c_int) as usize];
    dist >>= 16 as libc::c_int - bits;
    pos = (*table).decode_pos[bits as usize].wrapping_add(dist as libc::c_uint);
    if pos >= (*table).size {
        pos = 0 as libc::c_int as uint32_t
    }
    *num = (*table).decode_num[pos as usize];
    return ARCHIVE_OK;
}
/* Reads and parses Huffman tables from the beginning of the block. */
unsafe extern "C" fn parse_tables(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut p: *const uint8_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut bit_length: [uint8_t; 20] = [0; 20];
    let mut table: [uint8_t; 430] = [0; 430];
    let mut nibble_mask: uint8_t = 0xf0 as libc::c_int as uint8_t;
    let mut nibble_shift: uint8_t = 4 as libc::c_int as uint8_t;
    /* The data for table generation is compressed using a simple RLE-like
     * algorithm when storing zeroes, so we need to unpack it first. */
    w = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while w < HUFF_BC {
        if i as libc::c_long >= (*rar).cstate.cur_block_size {
            /* Truncated data, can't continue. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated data in huffman tables\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        value = (*p.offset(i as isize) as libc::c_int & nibble_mask as libc::c_int)
            >> nibble_shift as libc::c_int;
        if nibble_mask as libc::c_int == 0xf as libc::c_int {
            i += 1
        }
        nibble_mask = (nibble_mask as libc::c_int ^ 0xff as libc::c_int) as uint8_t;
        nibble_shift = (nibble_shift as libc::c_int ^ 4 as libc::c_int) as uint8_t;
        /* Values smaller than 15 is data, so we write it directly.
         * Value 15 is a flag telling us that we need to unpack more
         * bytes. */
        if value == ESCAPE as libc::c_int {
            value = (*p.offset(i as isize) as libc::c_int & nibble_mask as libc::c_int)
                >> nibble_shift as libc::c_int;
            if nibble_mask as libc::c_int == 0xf as libc::c_int {
                i += 1
            }
            nibble_mask = (nibble_mask as libc::c_int ^ 0xff as libc::c_int) as uint8_t;
            nibble_shift = (nibble_shift as libc::c_int ^ 4 as libc::c_int) as uint8_t;
            if value == 0 as libc::c_int {
                /* We sometimes need to write the actual value
                 * of 15, so this case handles that. */
                let fresh1 = w;
                w = w + 1;
                bit_length[fresh1 as usize] = ESCAPE as libc::c_int as uint8_t
            } else {
                let mut k: libc::c_int = 0;
                /* Fill zeroes. */
                k = 0 as libc::c_int;
                while k < value + 2 as libc::c_int && w < HUFF_BC {
                    let fresh2 = w;
                    w = w + 1;
                    bit_length[fresh2 as usize] = 0 as libc::c_int as uint8_t;
                    k += 1
                }
            }
        } else {
            let fresh3 = w;
            w = w + 1;
            bit_length[fresh3 as usize] = value as uint8_t
        }
    }
    (*rar).bits.in_addr = i;
    (*rar).bits.bit_addr = (nibble_shift as libc::c_int ^ 4 as libc::c_int) as int8_t;
    ret = create_decode_tables(bit_length.as_mut_ptr(), &mut (*rar).cstate.bd, HUFF_BC);
    if ret != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Decoding huffman tables failed\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    i = 0 as libc::c_int;
    while i < HUFF_TABLE_SIZE {
        let mut num: uint16_t = 0;
        if ((*rar).bits.in_addr + 6 as libc::c_int) as libc::c_long >= (*rar).cstate.cur_block_size
        {
            /* Truncated data, can't continue. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated data in huffman tables (#2)\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        ret = decode_number(a, &mut (*rar).cstate.bd, p, &mut num);
        if ret != ARCHIVE_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Decoding huffman tables failed\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        if (num as libc::c_int) < 16 as libc::c_int {
            /* 0..15: store directly */
            table[i as usize] = num as uint8_t;
            i += 1
        } else if (num as libc::c_int) < 18 as libc::c_int {
            /* 16..17: repeat previous code */
            let mut n: uint16_t = 0;
            if ARCHIVE_OK != read_bits_16(rar, p, &mut n) {
                return ARCHIVE_EOF;
            }
            if num as libc::c_int == 16 as libc::c_int {
                n = (n as libc::c_int >> 13 as libc::c_int) as uint16_t;
                n = (n as libc::c_int + 3 as libc::c_int) as uint16_t;
                skip_bits(rar, 3 as libc::c_int);
            } else {
                n = (n as libc::c_int >> 9 as libc::c_int) as uint16_t;
                n = (n as libc::c_int + 11 as libc::c_int) as uint16_t;
                skip_bits(rar, 7 as libc::c_int);
            }
            if i > 0 as libc::c_int {
                loop {
                    let fresh4 = n;
                    n = n.wrapping_sub(1);
                    if !(fresh4 as libc::c_int > 0 as libc::c_int && i < HUFF_TABLE_SIZE) {
                        break;
                    }
                    table[i as usize] = table[(i - 1 as libc::c_int) as usize];
                    i += 1
                }
            } else {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Unexpected error when decoding huffman tables\x00" as *const u8
                        as *const libc::c_char,
                );
                return ARCHIVE_FATAL;
            }
        } else {
            /* other codes: fill with zeroes `n` times */
            let mut n_0: uint16_t = 0;
            if ARCHIVE_OK != read_bits_16(rar, p, &mut n_0) {
                return ARCHIVE_EOF;
            }
            if num as libc::c_int == 18 as libc::c_int {
                n_0 = (n_0 as libc::c_int >> 13 as libc::c_int) as uint16_t;
                n_0 = (n_0 as libc::c_int + 3 as libc::c_int) as uint16_t;
                skip_bits(rar, 3 as libc::c_int);
            } else {
                n_0 = (n_0 as libc::c_int >> 9 as libc::c_int) as uint16_t;
                n_0 = (n_0 as libc::c_int + 11 as libc::c_int) as uint16_t;
                skip_bits(rar, 7 as libc::c_int);
            }
            loop {
                let fresh5 = n_0;
                n_0 = n_0.wrapping_sub(1);
                if !(fresh5 as libc::c_int > 0 as libc::c_int && i < HUFF_TABLE_SIZE) {
                    break;
                }
                let fresh6 = i;
                i = i + 1;
                table[fresh6 as usize] = 0 as libc::c_int as uint8_t
            }
        }
    }
    ret = create_decode_tables(
        &mut *table.as_mut_ptr().offset(idx as isize),
        &mut (*rar).cstate.ld,
        HUFF_NC,
    );
    if ret != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Failed to create literal table\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    idx += HUFF_NC;
    ret = create_decode_tables(
        &mut *table.as_mut_ptr().offset(idx as isize),
        &mut (*rar).cstate.dd,
        HUFF_DC,
    );
    if ret != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Failed to create distance table\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    idx += HUFF_DC;
    ret = create_decode_tables(
        &mut *table.as_mut_ptr().offset(idx as isize),
        &mut (*rar).cstate.ldd,
        HUFF_LDC,
    );
    if ret != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Failed to create lower bits of distances table\x00" as *const u8
                as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    idx += HUFF_LDC;
    ret = create_decode_tables(
        &mut *table.as_mut_ptr().offset(idx as isize),
        &mut (*rar).cstate.rd,
        HUFF_RC,
    );
    if ret != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Failed to create repeating distances table\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    return ARCHIVE_OK;
}
/* Parses the block header, verifies its CRC byte, and saves the header
 * fields inside the `hdr` pointer. */
unsafe extern "C" fn parse_block_header(
    mut a: *mut archive_read,
    mut p: *const uint8_t,
    mut block_size: *mut ssize_t,
    mut hdr: *mut compressed_block_header,
) -> libc::c_int {
    let mut calculated_cksum: uint8_t = 0;
    memcpy(
        hdr as *mut libc::c_void,
        p as *const libc::c_void,
        ::std::mem::size_of::<compressed_block_header>() as libc::c_ulong,
    );
    if bf_byte_count(hdr) as libc::c_int > 2 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Unsupported block header size (was %d, max is 2)\x00" as *const u8
                as *const libc::c_char,
            bf_byte_count(hdr) as libc::c_int,
        );
        return ARCHIVE_FATAL;
    }
    /* This should probably use bit reader interface in order to be more
     * future-proof. */
    *block_size = 0 as libc::c_int as ssize_t;
    match bf_byte_count(hdr) as libc::c_int {
        0 => {
            /* 1-byte block size */
            *block_size = *(&*p.offset(2 as libc::c_int as isize) as *const uint8_t) as ssize_t
        }
        1 => {
            /* 2-byte block size */
            *block_size = archive_le16dec(
                &*p.offset(2 as libc::c_int as isize) as *const uint8_t as *const libc::c_void
            ) as ssize_t
        }
        2 => {
            /* 3-byte block size */
            *block_size = archive_le32dec(
                &*p.offset(2 as libc::c_int as isize) as *const uint8_t as *const libc::c_void
            ) as ssize_t;
            *block_size &= 0xffffff as libc::c_int as libc::c_long
        }
        _ => {
            /* Other block sizes are not supported. This case is not
             * reached, because we have an 'if' guard before the switch
             * that makes sure of it. */
            return ARCHIVE_FATAL;
        }
    }
    /* Verify the block header checksum. 0x5A is a magic value and is
     * always * constant. */
    calculated_cksum = (0x5a as libc::c_int
        ^ (*hdr).block_flags_u8 as libc::c_int
        ^ *block_size as uint8_t as libc::c_int
        ^ (*block_size >> 8 as libc::c_int) as uint8_t as libc::c_int
        ^ (*block_size >> 16 as libc::c_int) as uint8_t as libc::c_int)
        as uint8_t;
    if calculated_cksum as libc::c_int != (*hdr).block_cksum as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Block checksum error: got 0x%x, expected 0x%x\x00" as *const u8
                as *const libc::c_char,
            (*hdr).block_cksum as libc::c_int,
            calculated_cksum as libc::c_int,
        );
        return ARCHIVE_FATAL;
    }
    return ARCHIVE_OK;
}
/* Convenience function used during filter processing. */
unsafe extern "C" fn parse_filter_data(
    mut rar: *mut rar5,
    mut p: *const uint8_t,
    mut filter_data: *mut uint32_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    let mut data: uint32_t = 0 as libc::c_int as uint32_t;
    if ARCHIVE_OK != read_consume_bits(rar, p, 2 as libc::c_int, &mut bytes) {
        return ARCHIVE_EOF;
    }
    bytes += 1;
    i = 0 as libc::c_int;
    while i < bytes {
        let mut byte: uint16_t = 0;
        if ARCHIVE_OK != read_bits_16(rar, p, &mut byte) {
            return ARCHIVE_EOF;
        }
        /* Cast to uint32_t will ensure the shift operation will not
         * produce undefined result. */
        data = (data as libc::c_uint)
            .wrapping_add((byte as uint32_t >> 8 as libc::c_int) << i * 8 as libc::c_int)
            as uint32_t as uint32_t;
        skip_bits(rar, 8 as libc::c_int);
        i += 1
    }
    *filter_data = data;
    return ARCHIVE_OK;
}
/* Function is used during sanity checking. */
unsafe extern "C" fn is_valid_filter_block_start(
    mut rar: *mut rar5,
    mut start: uint32_t,
) -> libc::c_int {
    let block_start: int64_t = start as ssize_t + (*rar).cstate.write_ptr;
    let last_bs: int64_t = (*rar).cstate.last_block_start;
    let last_bl: ssize_t = (*rar).cstate.last_block_length;
    if last_bs == 0 as libc::c_int as libc::c_long || last_bl == 0 as libc::c_int as libc::c_long {
        /* We didn't have any filters yet, so accept this offset. */
        return 1 as libc::c_int;
    }
    if block_start >= last_bs + last_bl {
        /* Current offset is bigger than last block's end offset, so
         * accept current offset. */
        return 1 as libc::c_int;
    }
    /* Any other case is not a normal situation and we should fail. */
    return 0 as libc::c_int;
}
/* The function will create a new filter, read its parameters from the input
 * stream and add it to the filter collection. */
unsafe extern "C" fn parse_filter(mut ar: *mut archive_read, mut p: *const uint8_t) -> libc::c_int {
    let mut block_start: uint32_t = 0;
    let mut block_length: uint32_t = 0;
    let mut filter_type: uint16_t = 0;
    let mut filt: *mut filter_info = NULL as *mut filter_info;
    let mut rar: *mut rar5 = get_context(ar);
    /* Read the parameters from the input stream. */
    if ARCHIVE_OK != parse_filter_data(rar, p, &mut block_start) {
        return ARCHIVE_EOF;
    }
    if ARCHIVE_OK != parse_filter_data(rar, p, &mut block_length) {
        return ARCHIVE_EOF;
    }
    if ARCHIVE_OK != read_bits_16(rar, p, &mut filter_type) {
        return ARCHIVE_EOF;
    }
    filter_type = (filter_type as libc::c_int >> 13 as libc::c_int) as uint16_t;
    skip_bits(rar, 3 as libc::c_int);
    /* Perform some sanity checks on this filter parameters. Note that we
     * allow only DELTA, E8/E9 and ARM filters here, because rest of
     * filters are not used in RARv5. */
    if block_length < 4 as libc::c_int as libc::c_uint
        || block_length > 0x400000 as libc::c_int as libc::c_uint
        || filter_type as libc::c_int > FILTER_ARM as libc::c_int
        || is_valid_filter_block_start(rar, block_start) == 0
    {
        archive_set_error(
            &mut (*ar).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid filter encountered\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    /* Allocate a new filter. */
    filt = add_new_filter(rar);
    if filt.is_null() {
        archive_set_error(
            &mut (*ar).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for a filter descriptor.\x00" as *const u8
                as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    (*filt).type_0 = filter_type as libc::c_int;
    (*filt).block_start = (*rar).cstate.write_ptr + block_start as libc::c_long;
    (*filt).block_length = block_length as ssize_t;
    (*rar).cstate.last_block_start = (*filt).block_start;
    (*rar).cstate.last_block_length = (*filt).block_length;
    /* Read some more data in case this is a DELTA filter. Other filter
     * types don't require any additional data over what was already
     * read. */
    if filter_type as libc::c_int == FILTER_DELTA as libc::c_int {
        let mut channels: libc::c_int = 0;
        if ARCHIVE_OK != read_consume_bits(rar, p, 5 as libc::c_int, &mut channels) {
            return ARCHIVE_EOF;
        }
        (*filt).channels = channels + 1 as libc::c_int
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn decode_code_length(
    mut rar: *mut rar5,
    mut p: *const uint8_t,
    mut code: uint16_t,
) -> libc::c_int {
    let mut lbits: libc::c_int = 0;
    let mut length: libc::c_int = 2 as libc::c_int;
    if (code as libc::c_int) < 8 as libc::c_int {
        lbits = 0 as libc::c_int;
        length += code as libc::c_int
    } else {
        lbits = code as libc::c_int / 4 as libc::c_int - 1 as libc::c_int;
        length += (4 as libc::c_int | code as libc::c_int & 3 as libc::c_int) << lbits
    }
    if lbits > 0 as libc::c_int {
        let mut add: libc::c_int = 0;
        if ARCHIVE_OK != read_consume_bits(rar, p, lbits, &mut add) {
            return -(1 as libc::c_int);
        }
        length += add
    }
    return length;
}
unsafe extern "C" fn copy_string(
    mut a: *mut archive_read,
    mut len: libc::c_int,
    mut dist: libc::c_int,
) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    let cmask: uint64_t = (*rar).cstate.window_mask;
    let write_ptr: uint64_t = ((*rar).cstate.write_ptr + (*rar).cstate.solid_offset) as uint64_t;
    let mut i: libc::c_int = 0;
    if (*rar).cstate.window_buf.is_null() {
        return ARCHIVE_FATAL;
    }
    /* The unpacker spends most of the time in this function. It would be
     * a good idea to introduce some optimizations here.
     *
     * Just remember that this loop treats buffers that overlap differently
     * than buffers that do not overlap. This is why a simple memcpy(3)
     * call will not be enough. */
    i = 0 as libc::c_int;
    while i < len {
        let write_idx: ssize_t = (write_ptr.wrapping_add(i as libc::c_ulong) & cmask) as ssize_t;
        let read_idx: ssize_t = (write_ptr
            .wrapping_add(i as libc::c_ulong)
            .wrapping_sub(dist as libc::c_ulong)
            & cmask) as ssize_t;
        *(*rar).cstate.window_buf.offset(write_idx as isize) =
            *(*rar).cstate.window_buf.offset(read_idx as isize);
        i += 1
    }
    (*rar).cstate.write_ptr += len as libc::c_long;
    return ARCHIVE_OK;
}
unsafe extern "C" fn do_uncompress_block(
    mut a: *mut archive_read,
    mut p: *const uint8_t,
) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    let mut num: uint16_t = 0;
    let mut ret: libc::c_int = 0;
    let cmask: uint64_t = (*rar).cstate.window_mask;
    let mut hdr: *const compressed_block_header = &mut (*rar).last_block_hdr;
    let bit_size: uint8_t = (1 as libc::c_int + bf_bit_size(hdr) as libc::c_int) as uint8_t;
    while !((*rar).cstate.write_ptr - (*rar).cstate.last_write_ptr
        > (*rar).cstate.window_size >> 1 as libc::c_int)
    {
        if (*rar).bits.in_addr as libc::c_long
            > (*rar).cstate.cur_block_size - 1 as libc::c_int as libc::c_long
            || (*rar).bits.in_addr as libc::c_long
                == (*rar).cstate.cur_block_size - 1 as libc::c_int as libc::c_long
                && (*rar).bits.bit_addr as libc::c_int >= bit_size as libc::c_int
        {
            /* If the program counter is here, it means the
             * function has finished processing the block. */
            (*rar)
                .cstate
                .set_block_parsing_finished(1 as libc::c_int as uint8_t);
            break;
        } else {
            /* Decode the next literal. */
            if ARCHIVE_OK != decode_number(a, &mut (*rar).cstate.ld, p, &mut num) {
                return ARCHIVE_EOF;
            }
            /* Num holds a decompression literal, or 'command code'.
             *
             * - Values lower than 256 are just bytes. Those codes
             *   can be stored in the output buffer directly.
             *
             * - Code 256 defines a new filter, which is later used to
             *   ransform the data block accordingly to the filter type.
             *   The data block needs to be fully uncompressed first.
             *
             * - Code bigger than 257 and smaller than 262 define
             *   a repetition pattern that should be copied from
             *   an already uncompressed chunk of data.
             */
            if (num as libc::c_int) < 256 as libc::c_int {
                /* Directly store the byte. */
                let fresh7 = (*rar).cstate.write_ptr;
                (*rar).cstate.write_ptr = (*rar).cstate.write_ptr + 1;
                let mut write_idx: int64_t = (*rar).cstate.solid_offset + fresh7;
                *(*rar)
                    .cstate
                    .window_buf
                    .offset((write_idx as libc::c_ulong & cmask) as isize) = num as uint8_t
            } else if num as libc::c_int >= 262 as libc::c_int {
                let mut dist_slot: uint16_t = 0;
                let mut len: libc::c_int = decode_code_length(
                    rar,
                    p,
                    (num as libc::c_int - 262 as libc::c_int) as uint16_t,
                );
                let mut dbits: libc::c_int = 0;
                let mut dist: libc::c_int = 1 as libc::c_int;
                if len == -(1 as libc::c_int) {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_PROGRAMMER,
                        b"Failed to decode the code length\x00" as *const u8 as *const libc::c_char,
                    );
                    return ARCHIVE_FATAL;
                }
                if ARCHIVE_OK != decode_number(a, &mut (*rar).cstate.dd, p, &mut dist_slot) {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_PROGRAMMER,
                        b"Failed to decode the distance slot\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return ARCHIVE_FATAL;
                }
                if (dist_slot as libc::c_int) < 4 as libc::c_int {
                    dbits = 0 as libc::c_int;
                    dist += dist_slot as libc::c_int
                } else {
                    dbits = dist_slot as libc::c_int / 2 as libc::c_int - 1 as libc::c_int;
                    /* Cast to uint32_t will make sure the shift
                     * left operation won't produce undefined
                     * result. Then, the uint32_t type will
                     * be implicitly casted to int. */
                    dist = (dist as libc::c_uint).wrapping_add(
                        ((2 as libc::c_int | dist_slot as libc::c_int & 1 as libc::c_int)
                            as uint32_t)
                            << dbits,
                    ) as libc::c_int as libc::c_int
                }
                if dbits > 0 as libc::c_int {
                    if dbits >= 4 as libc::c_int {
                        let mut add: uint32_t = 0 as libc::c_int as uint32_t;
                        let mut low_dist: uint16_t = 0;
                        if dbits > 4 as libc::c_int {
                            if ARCHIVE_OK != read_bits_32(rar, p, &mut add) {
                                /* Return EOF if we
                                 * can't read more
                                 * data. */
                                return ARCHIVE_EOF;
                            }
                            skip_bits(rar, dbits - 4 as libc::c_int);
                            add = add >> 36 as libc::c_int - dbits << 4 as libc::c_int;
                            dist = (dist as libc::c_uint).wrapping_add(add) as libc::c_int
                                as libc::c_int
                        }
                        if ARCHIVE_OK != decode_number(a, &mut (*rar).cstate.ldd, p, &mut low_dist)
                        {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_PROGRAMMER,
                                b"Failed to decode the distance slot\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            return ARCHIVE_FATAL;
                        }
                        if dist >= INT_MAX - low_dist as libc::c_int - 1 as libc::c_int {
                            /* This only happens in
                             * invalid archives. */
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_FILE_FORMAT,
                                b"Distance pointer overflow\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            return ARCHIVE_FATAL;
                        }
                        dist += low_dist as libc::c_int
                    } else {
                        /* dbits is one of [0,1,2,3] */
                        let mut add_0: libc::c_int = 0;
                        if ARCHIVE_OK != read_consume_bits(rar, p, dbits, &mut add_0) {
                            /* Return EOF if we can't read
                             * more data. */
                            return ARCHIVE_EOF;
                        }
                        dist += add_0
                    }
                }
                if dist > 0x100 as libc::c_int {
                    len += 1;
                    if dist > 0x2000 as libc::c_int {
                        len += 1;
                        if dist > 0x40000 as libc::c_int {
                            len += 1
                        }
                    }
                }
                dist_cache_push(rar, dist);
                (*rar).cstate.last_len = len;
                if ARCHIVE_OK != copy_string(a, len, dist) {
                    return ARCHIVE_FATAL;
                }
            } else if num as libc::c_int == 256 as libc::c_int {
                /* Create a filter. */
                ret = parse_filter(a, p);
                if ret != ARCHIVE_OK {
                    return ret;
                }
            } else if num as libc::c_int == 257 as libc::c_int {
                if (*rar).cstate.last_len != 0 as libc::c_int {
                    if ARCHIVE_OK
                        != copy_string(
                            a,
                            (*rar).cstate.last_len,
                            (*rar).cstate.dist_cache[0 as libc::c_int as usize],
                        )
                    {
                        return ARCHIVE_FATAL;
                    }
                }
            } else {
                /* num < 262 */
                let idx: libc::c_int = num as libc::c_int - 258 as libc::c_int;
                let dist_0: libc::c_int = dist_cache_touch(rar, idx);
                let mut len_slot: uint16_t = 0;
                let mut len_0: libc::c_int = 0;
                if ARCHIVE_OK != decode_number(a, &mut (*rar).cstate.rd, p, &mut len_slot) {
                    return ARCHIVE_FATAL;
                }
                len_0 = decode_code_length(rar, p, len_slot);
                (*rar).cstate.last_len = len_0;
                if ARCHIVE_OK != copy_string(a, len_0, dist_0) {
                    return ARCHIVE_FATAL;
                }
            }
        }
    }
    return ARCHIVE_OK;
}
/* Binary search for the RARv5 signature. */
unsafe extern "C" fn scan_for_signature(mut a: *mut archive_read) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let chunk_size: libc::c_int = 512 as libc::c_int;
    let mut i: ssize_t = 0;
    let mut signature: [libc::c_char; 8] = [0; 8];
    /* If we're here, it means we're on an 'unknown territory' data.
     * There's no indication what kind of data we're reading here.
     * It could be some text comment, any kind of binary data,
     * digital sign, dragons, etc.
     *
     * We want to find a valid RARv5 magic header inside this unknown
     * data. */
    /* Is it possible in libarchive to just skip everything until the
     * end of the file? If so, it would be a better approach than the
     * current implementation of this function. */
    rar5_signature(signature.as_mut_ptr());
    loop {
        if read_ahead(a, chunk_size as size_t, &mut p) == 0 {
            return ARCHIVE_EOF;
        }
        i = 0 as libc::c_int as ssize_t;
        while i
            < (chunk_size
                - ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int)
                as libc::c_long
        {
            if memcmp(
                &*p.offset(i as isize) as *const uint8_t as *const libc::c_void,
                signature.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                /* Consume the number of bytes we've used to
                 * search for the signature, as well as the
                 * number of bytes used by the signature
                 * itself. After this we should be standing
                 * on a valid base block header. */
                consume(
                    a,
                    (i as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong)
                        as int64_t,
                );
                return ARCHIVE_OK;
            }
            i += 1
        }
        consume(a, chunk_size as int64_t);
    }
}
/* This function will switch the multivolume archive file to another file,
 * i.e. from part03 to part 04. */
unsafe extern "C" fn advance_multivolume(mut a: *mut archive_read) -> libc::c_int {
    let mut lret: libc::c_int = 0;
    let mut rar: *mut rar5 = get_context(a);
    loop
    /* A small state machine that will skip unnecessary data, needed to
     * switch from one multivolume to another. Such skipping is needed if
     * we want to be an stream-oriented (instead of file-oriented)
     * unpacker.
     *
     * The state machine starts with `rar->main.endarc` == 0. It also
     * assumes that current stream pointer points to some base block
     * header.
     *
     * The `endarc` field is being set when the base block parsing
     * function encounters the 'end of archive' marker.
     */
    {
        if (*rar).main.endarc() as libc::c_int == 1 as libc::c_int {
            let mut looping: libc::c_int = 1 as libc::c_int;
            (*rar).main.set_endarc(0 as libc::c_int as uint8_t);
            while looping != 0 {
                lret = skip_base_block(a);
                match lret {
                    -10 => {}
                    ARCHIVE_OK => {
                        /* Break loop. */
                        looping = 0 as libc::c_int
                    }
                    _ => {
                        /* Forward any errors to the
                         * caller. */
                        return lret;
                    }
                }
            }
            break;
        } else {
            /* Skip current base block. In order to properly skip
             * it, we really need to simply parse it and discard
             * the results. */
            lret = skip_base_block(a);
            if lret == ARCHIVE_FATAL || lret == ARCHIVE_FAILED {
                return lret;
            }
            /* The `skip_base_block` function tells us if we
             * should continue with skipping, or we should stop
             * skipping. We're trying to skip everything up to
             * a base FILE block. */
            if !(lret != ARCHIVE_RETRY) {
                continue;
            }
            /* If there was an error during skipping, or we
             * have just skipped a FILE base block... */
            if !((*rar).main.endarc() as libc::c_int == 0 as libc::c_int) {
                continue;
            }
            return lret;
        }
    }
    return ARCHIVE_OK;
}
/* Merges the partial block from the first multivolume archive file, and
 * partial block from the second multivolume archive file. The result is
 * a chunk of memory containing the whole block, and the stream pointer
 * is advanced to the next block in the second multivolume archive file. */
unsafe extern "C" fn merge_block(
    mut a: *mut archive_read,
    mut block_size: ssize_t,
    mut p: *mut *const uint8_t,
) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    let mut cur_block_size: ssize_t = 0;
    let mut partial_offset: ssize_t = 0 as libc::c_int as ssize_t;
    let mut lp: *const uint8_t = 0 as *const uint8_t;
    let mut ret: libc::c_int = 0;
    if (*rar).merge_mode != 0 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Recursive merge is not allowed\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    /* Set a flag that we're in the switching mode. */
    (*rar)
        .cstate
        .set_switch_multivolume(1 as libc::c_int as uint8_t);
    /* Reallocate the memory which will hold the whole block. */
    if !(*rar).vol.push_buf.is_null() {
        free((*rar).vol.push_buf as *mut libc::c_void);
    }
    /* Increasing the allocation block by 8 is due to bit reading functions,
     * which are using additional 2 or 4 bytes. Allocating the block size
     * by exact value would make bit reader perform reads from invalid
     * memory block when reading the last byte from the buffer. */
    (*rar).vol.push_buf =
        malloc((block_size + 8 as libc::c_int as libc::c_long) as libc::c_ulong) as *mut uint8_t;
    if (*rar).vol.push_buf.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for a merge block buffer.\x00" as *const u8
                as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    /* Valgrind complains if the extension block for bit reader is not
     * initialized, so initialize it. */
    memset(
        &mut *(*rar).vol.push_buf.offset(block_size as isize) as *mut uint8_t as *mut libc::c_void,
        0 as libc::c_int,
        8 as libc::c_int as libc::c_ulong,
    );
    loop
    /* A single block can span across multiple multivolume archive files,
     * so we use a loop here. This loop will consume enough multivolume
     * archive files until the whole block is read. */
    /* Get the size of current block chunk in this multivolume
     * archive file and read it. */
    {
        cur_block_size = if (*rar).file.bytes_remaining > block_size - partial_offset {
            (block_size) - partial_offset
        } else {
            (*rar).file.bytes_remaining
        };
        if cur_block_size == 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Encountered block size == 0 during block merge\x00" as *const u8
                    as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        if read_ahead(a, cur_block_size as size_t, &mut lp) == 0 {
            return ARCHIVE_EOF;
        }
        /* Sanity check; there should never be a situation where this
         * function reads more data than the block's size. */
        if partial_offset + cur_block_size > block_size {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_PROGRAMMER,
                b"Consumed too much data when merging blocks.\x00" as *const u8
                    as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        /* Merge previous block chunk with current block chunk,
         * or create first block chunk if this is our first
         * iteration. */
        memcpy(
            &mut *(*rar).vol.push_buf.offset(partial_offset as isize) as *mut uint8_t
                as *mut libc::c_void,
            lp as *const libc::c_void,
            cur_block_size as libc::c_ulong,
        );
        /* Advance the stream read pointer by this block chunk size. */
        if ARCHIVE_OK != consume(a, cur_block_size) {
            return ARCHIVE_EOF;
        }
        /* Update the pointers. `partial_offset` contains information
         * about the sum of merged block chunks. */
        partial_offset += cur_block_size;
        (*rar).file.bytes_remaining -= cur_block_size;
        /* If `partial_offset` is the same as `block_size`, this means
         * we've merged all block chunks and we have a valid full
         * block. */
        if partial_offset == block_size {
            break;
        }
        /* If we don't have any bytes to read, this means we should
         * switch to another multivolume archive file. */
        if (*rar).file.bytes_remaining == 0 as libc::c_int as libc::c_long {
            (*rar).merge_mode += 1;
            ret = advance_multivolume(a);
            (*rar).merge_mode -= 1;
            if ret != ARCHIVE_OK {
                return ret;
            }
        }
    }
    *p = (*rar).vol.push_buf;
    /* If we're here, we can resume unpacking by processing the block
     * pointed to by the `*p` memory pointer. */
    return ARCHIVE_OK;
}
unsafe extern "C" fn process_block(mut a: *mut archive_read) -> libc::c_int {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut rar: *mut rar5 = get_context(a);
    let mut ret: libc::c_int = 0;
    /* If we don't have any data to be processed, this most probably means
     * we need to switch to the next volume. */
    if (*rar).main.volume() as libc::c_int != 0
        && (*rar).file.bytes_remaining == 0 as libc::c_int as libc::c_long
    {
        ret = advance_multivolume(a);
        if ret != ARCHIVE_OK {
            return ret;
        }
    }
    if (*rar).cstate.block_parsing_finished() != 0 {
        let mut block_size: ssize_t = 0;
        let mut to_skip: ssize_t = 0;
        let mut cur_block_size: ssize_t = 0;
        /* The header size won't be bigger than 6 bytes. */
        if read_ahead(a, 6 as libc::c_int as size_t, &mut p) == 0 {
            /* Failed to prefetch data block header. */
            return ARCHIVE_EOF;
        }
        /*
         * Read block_size by parsing block header. Validate the header
         * by calculating CRC byte stored inside the header. Size of
         * the header is not constant (block size can be stored either
         * in 1 or 2 bytes), that's why block size is left out from the
         * `compressed_block_header` structure and returned by
         * `parse_block_header` as the second argument. */
        ret = parse_block_header(a, p, &mut block_size, &mut (*rar).last_block_hdr);
        if ret != ARCHIVE_OK {
            return ret;
        }
        /* Skip block header. Next data is huffman tables,
         * if present. */
        to_skip = (::std::mem::size_of::<compressed_block_header>() as libc::c_ulong)
            .wrapping_add(bf_byte_count(&mut (*rar).last_block_hdr) as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as ssize_t;
        if ARCHIVE_OK != consume(a, to_skip) {
            return ARCHIVE_EOF;
        }
        (*rar).file.bytes_remaining -= to_skip;
        /* The block size gives information about the whole block size,
         * but the block could be stored in split form when using
         * multi-volume archives. In this case, the block size will be
         * bigger than the actual data stored in this file. Remaining
         * part of the data will be in another file. */
        cur_block_size = if (*rar).file.bytes_remaining > block_size {
            block_size
        } else {
            (*rar).file.bytes_remaining
        };
        if block_size > (*rar).file.bytes_remaining {
            /* If current blocks' size is bigger than our data
             * size, this means we have a multivolume archive.
             * In this case, skip all base headers until the end
             * of the file, proceed to next "partXXX.rar" volume,
             * find its signature, skip all headers up to the first
             * FILE base header, and continue from there.
             *
             * Note that `merge_block` will update the `rar`
             * context structure quite extensively. */
            ret = merge_block(a, block_size, &mut p);
            if ret != ARCHIVE_OK {
                return ret;
            }
            cur_block_size = block_size
        /* Current stream pointer should be now directly
         * *after* the block that spanned through multiple
         * archive files. `p` pointer should have the data of
         * the *whole* block (merged from partial blocks
         * stored in multiple archives files). */
        } else {
            (*rar)
                .cstate
                .set_switch_multivolume(0 as libc::c_int as uint8_t);
            /* Read the whole block size into memory. This can take
             * up to  8 megabytes of memory in theoretical cases.
             * Might be worth to optimize this and use a standard
             * chunk of 4kb's. */
            if read_ahead(
                a,
                (4 as libc::c_int as libc::c_long + cur_block_size) as size_t,
                &mut p,
            ) == 0
            {
                /* Failed to prefetch block data. */
                return ARCHIVE_EOF;
            }
        }
        (*rar).cstate.block_buf = p;
        (*rar).cstate.cur_block_size = cur_block_size;
        (*rar)
            .cstate
            .set_block_parsing_finished(0 as libc::c_int as uint8_t);
        (*rar).bits.in_addr = 0 as libc::c_int;
        (*rar).bits.bit_addr = 0 as libc::c_int as int8_t;
        if bf_is_table_present(&mut (*rar).last_block_hdr) != 0 {
            /* Load Huffman tables. */
            ret = parse_tables(a, rar, p);
            if ret != ARCHIVE_OK {
                /* Error during decompression of Huffman
                 * tables. */
                return ret;
            }
        }
    } else {
        /* Block parsing not finished, reuse previous memory buffer. */
        p = (*rar).cstate.block_buf
    }
    /* Uncompress the block, or a part of it, depending on how many bytes
     * will be generated by uncompressing the block.
     *
     * In case too many bytes will be generated, calling this function
     * again will resume the uncompression operation. */
    ret = do_uncompress_block(a, p);
    if ret != ARCHIVE_OK {
        return ret;
    }
    if (*rar).cstate.block_parsing_finished() as libc::c_int != 0
        && (*rar).cstate.switch_multivolume() as libc::c_int == 0 as libc::c_int
        && (*rar).cstate.cur_block_size > 0 as libc::c_int as libc::c_long
    {
        /* If we're processing a normal block, consume the whole
         * block. We can do this because we've already read the whole
         * block to memory. */
        if ARCHIVE_OK != consume(a, (*rar).cstate.cur_block_size) {
            return ARCHIVE_FATAL;
        }
        (*rar).file.bytes_remaining -= (*rar).cstate.cur_block_size
    } else if (*rar).cstate.switch_multivolume() != 0 {
        /* Don't consume the block if we're doing multivolume
         * processing. The volume switching function will consume
         * the proper count of bytes instead. */
        (*rar)
            .cstate
            .set_switch_multivolume(0 as libc::c_int as uint8_t)
    }
    return ARCHIVE_OK;
}
/* Pops the `buf`, `size` and `offset` from the "data ready" stack.
 *
 * Returns ARCHIVE_OK when those arguments can be used, ARCHIVE_RETRY
 * when there is no data on the stack. */
unsafe extern "C" fn use_data(
    mut rar: *mut rar5,
    mut buf: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_long)
        < (::std::mem::size_of::<[data_ready; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<data_ready>() as libc::c_ulong) as ssize_t
    {
        let mut d: *mut data_ready =
            &mut *(*rar).cstate.dready.as_mut_ptr().offset(i as isize) as *mut data_ready;
        if (*d).used != 0 {
            if !buf.is_null() {
                *buf = (*d).buf as *const libc::c_void
            }
            if !size.is_null() {
                *size = (*d).size
            }
            if !offset.is_null() {
                *offset = (*d).offset
            }
            (*d).used = 0 as libc::c_int as libc::c_char;
            return ARCHIVE_OK;
        }
        i += 1
    }
    return ARCHIVE_RETRY;
}
/* Pushes the `buf`, `size` and `offset` arguments to the rar->cstate.dready
 * FIFO stack. Those values will be popped from this stack by the `use_data`
 * function. */
unsafe extern "C" fn push_data_ready(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut buf: *const uint8_t,
    mut size: size_t,
    mut offset: int64_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* Don't push if we're in skip mode. This is needed because solid
     * streams need full processing even if we're skipping data. After
     * fully processing the stream, we need to discard the generated bytes,
     * because we're interested only in the side effect: building up the
     * internal window circular buffer. This window buffer will be used
     * later during unpacking of requested data. */
    if (*rar).skip_mode != 0 {
        return ARCHIVE_OK;
    }
    /* Sanity check. */
    if offset != (*rar).file.last_offset + (*rar).file.last_size {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Sanity check error: output stream is not continuous\x00" as *const u8
                as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_long)
        < (::std::mem::size_of::<[data_ready; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<data_ready>() as libc::c_ulong) as ssize_t
    {
        let mut d: *mut data_ready =
            &mut *(*rar).cstate.dready.as_mut_ptr().offset(i as isize) as *mut data_ready;
        if (*d).used == 0 {
            (*d).used = 1 as libc::c_int as libc::c_char;
            (*d).buf = buf;
            (*d).size = size;
            (*d).offset = offset;
            /* These fields are used only in sanity checking. */
            (*rar).file.last_offset = offset;
            (*rar).file.last_size = size as int64_t;
            /* Calculate the checksum of this new block before
             * submitting data to libarchive's engine. */
            update_crc(rar, (*d).buf, (*d).size);
            return ARCHIVE_OK;
        }
        i += 1
    }
    /* Program counter will reach this code if the `rar->cstate.data_ready`
     * stack will be filled up so that no new entries will be allowed. The
     * code shouldn't allow such situation to occur. So we treat this case
     * as an internal error. */
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_PROGRAMMER,
        b"Error: premature end of data_ready stack\x00" as *const u8 as *const libc::c_char,
    );
    return ARCHIVE_FATAL;
}
/* This function uncompresses the data that is stored in the <FILE> base
 * block.
 *
 * The FILE base block looks like this:
 *
 * <header><huffman tables><block_1><block_2>...<block_n>
 *
 * The <header> is a block header, that is parsed in parse_block_header().
 * It's a "compressed_block_header" structure, containing metadata needed
 * to know when we should stop looking for more <block_n> blocks.
 *
 * <huffman tables> contain data needed to set up the huffman tables, needed
 * for the actual decompression.
 *
 * Each <block_n> consists of series of literals:
 *
 * <literal><literal><literal>...<literal>
 *
 * Those literals generate the uncompression data. They operate on a circular
 * buffer, sometimes writing raw data into it, sometimes referencing
 * some previous data inside this buffer, and sometimes declaring a filter
 * that will need to be executed on the data stored in the circular buffer.
 * It all depends on the literal that is used.
 *
 * Sometimes blocks produce output data, sometimes they don't. For example, for
 * some huge files that use lots of filters, sometimes a block is filled with
 * only filter declaration literals. Such blocks won't produce any data in the
 * circular buffer.
 *
 * Sometimes blocks will produce 4 bytes of data, and sometimes 1 megabyte,
 * because a literal can reference previously decompressed data. For example,
 * there can be a literal that says: 'append a byte 0xFE here', and after
 * it another literal can say 'append 1 megabyte of data from circular buffer
 * offset 0x12345'. This is how RAR format handles compressing repeated
 * patterns.
 *
 * The RAR compressor creates those literals and the actual efficiency of
 * compression depends on what those literals are. The literals can also
 * be seen as a kind of a non-turing-complete virtual machine that simply
 * tells the decompressor what it should do.
 * */
unsafe extern "C" fn do_uncompress_file(mut a: *mut archive_read) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    let mut ret: libc::c_int = 0;
    let mut max_end_pos: int64_t = 0;
    if (*rar).cstate.initialized() == 0 {
        /* Don't perform full context reinitialization if we're
         * processing a solid archive. */
        if (*rar).main.solid() == 0 || (*rar).cstate.window_buf.is_null() {
            init_unpack(rar);
        }
        (*rar).cstate.set_initialized(1 as libc::c_int as uint8_t)
    }
    if (*rar).cstate.all_filters_applied() as libc::c_int == 1 as libc::c_int {
        loop
        /* We use while(1) here, but standard case allows for just 1
         * iteration. The loop will iterate if process_block() didn't
         * generate any data at all. This can happen if the block
         * contains only filter definitions (this is common in big
         * files). */
        {
            ret = process_block(a);
            if ret == ARCHIVE_EOF || ret == ARCHIVE_FATAL {
                return ret;
            }
            if !((*rar).cstate.last_write_ptr == (*rar).cstate.write_ptr) {
                break;
            }
        }
    }
    /* Try to run filters. If filters won't be applied, it means that
     * insufficient data was generated. */
    ret = apply_filters(a);
    if ret == ARCHIVE_RETRY {
        return ARCHIVE_OK;
    } else {
        if ret == ARCHIVE_FATAL {
            return ARCHIVE_FATAL;
        }
    }
    /* If apply_filters() will return ARCHIVE_OK, we can continue here. */
    if cdeque_size(&mut (*rar).cstate.filters) > 0 as libc::c_int as libc::c_ulong {
        /* Check if we can write something before hitting first
         * filter. */
        let mut flt: *mut filter_info = 0 as *mut filter_info;
        /* Get the block_start offset from the first filter. */
        if CDE_OK as libc::c_int
            != cdeque_front(&mut (*rar).cstate.filters, cdeque_filter_p(&mut flt))
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_PROGRAMMER,
                b"Can\'t read first filter\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        max_end_pos = if (*flt).block_start > (*rar).cstate.write_ptr {
            (*rar).cstate.write_ptr
        } else {
            (*flt).block_start
        }
    } else {
        /* There are no filters defined, or all filters were applied.
         * This means we can just store the data without any
         * postprocessing. */
        max_end_pos = (*rar).cstate.write_ptr
    }
    if max_end_pos == (*rar).cstate.last_write_ptr {
        /* We can't write anything yet. The block uncompression
         * function did not generate enough data, and no filter can be
         * applied. At the same time we don't have any data that can be
         *  stored without filter postprocessing. This means we need to
         *  wait for more data to be generated, so we can apply the
         * filters.
         *
         * Signal the caller that we need more data to be able to do
         * anything.
         */
        return ARCHIVE_RETRY;
    } else {
        /* We can write the data before hitting the first filter.
         * So let's do it. The push_window_data() function will
         * effectively return the selected data block to the user
         * application. */
        push_window_data(a, rar, (*rar).cstate.last_write_ptr, max_end_pos);
        (*rar).cstate.last_write_ptr = max_end_pos
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn uncompress_file(mut a: *mut archive_read) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        /* Sometimes the uncompression function will return a
         * 'retry' signal. If this will happen, we have to retry
         * the function. */
        ret = do_uncompress_file(a);
        if ret != ARCHIVE_RETRY {
            return ret;
        }
    }
}
unsafe extern "C" fn do_unstore_file(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut buf: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut to_read: size_t = 0;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    if (*rar).file.bytes_remaining == 0 as libc::c_int as libc::c_long
        && (*rar).main.volume() as libc::c_int > 0 as libc::c_int
        && (*rar).generic.split_after() as libc::c_int > 0 as libc::c_int
    {
        let mut ret: libc::c_int = 0;
        (*rar)
            .cstate
            .set_switch_multivolume(1 as libc::c_int as uint8_t);
        ret = advance_multivolume(a);
        (*rar)
            .cstate
            .set_switch_multivolume(0 as libc::c_int as uint8_t);
        if ret != ARCHIVE_OK {
            /* Failed to advance to next multivolume archive
             * file. */
            return ret;
        }
    }
    to_read = if (*rar).file.bytes_remaining
        > (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
    {
        (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
    } else {
        (*rar).file.bytes_remaining
    } as size_t;
    if to_read == 0 as libc::c_int as libc::c_ulong {
        return ARCHIVE_EOF;
    }
    if read_ahead(a, to_read, &mut p) == 0 {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"I/O error when unstoring file\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    if ARCHIVE_OK != consume(a, to_read as int64_t) {
        return ARCHIVE_EOF;
    }
    if !buf.is_null() {
        *buf = p as *const libc::c_void
    }
    if !size.is_null() {
        *size = to_read
    }
    if !offset.is_null() {
        *offset = (*rar).cstate.last_unstore_ptr
    }
    (*rar).file.bytes_remaining =
        ((*rar).file.bytes_remaining as libc::c_ulong).wrapping_sub(to_read) as ssize_t as ssize_t;
    (*rar).cstate.last_unstore_ptr = ((*rar).cstate.last_unstore_ptr as libc::c_ulong)
        .wrapping_add(to_read) as int64_t as int64_t;
    update_crc(rar, p, to_read);
    return ARCHIVE_OK;
}
unsafe extern "C" fn do_unpack(
    mut a: *mut archive_read,
    mut rar: *mut rar5,
    mut buf: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    if (*rar).file.service() as libc::c_int > 0 as libc::c_int {
        return do_unstore_file(a, rar, buf, size, offset);
    } else {
        let mut current_block_5: u64;
        match (*rar).cstate.method {
            0 => return do_unstore_file(a, rar, buf, size, offset),
            1 => {
                /* fallthrough */
                current_block_5 = 6680022206450058600;
            }
            2 => {
                current_block_5 = 6680022206450058600;
            }
            4 => {
                /* fallthrough */
                current_block_5 = 183507752417177349;
            }
            3 | 5 => {
                current_block_5 = 183507752417177349;
            }
            _ => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Compression method not supported: 0x%x\x00" as *const u8
                        as *const libc::c_char,
                    (*rar).cstate.method,
                );
                return ARCHIVE_FATAL;
            }
        }
        match current_block_5 {
            6680022206450058600 =>
                /* fallthrough */
                {}
            _ => {}
        }
        /* fallthrough */
        return uncompress_file(a);
    };
}
unsafe extern "C" fn verify_checksums(mut a: *mut archive_read) -> libc::c_int {
    let mut verify_crc: libc::c_int = 0;
    let mut rar: *mut rar5 = get_context(a);
    /* Check checksums only when actually unpacking the data. There's no
     * need to calculate checksum when we're skipping data in solid archives
     * (skipping in solid archives is the same thing as unpacking compressed
     * data and discarding the result). */
    if (*rar).skip_mode == 0 {
        /* Always check checksums if we're not in skip mode */
        verify_crc = 1 as libc::c_int
    } else {
        /* We can override the logic above with a compile-time option
         * NO_CRC_ON_SOLID_SKIP. This option is used during debugging,
         * and it will check checksums of unpacked data even when
         * we're skipping it. */
        /* Normal case */
        verify_crc = 0 as libc::c_int
    }
    if verify_crc != 0 {
        /* During unpacking, on each unpacked block we're calling the
         * update_crc() function. Since we are here, the unpacking
         * process is already over and we can check if calculated
         * checksum (CRC32 or BLAKE2sp) is the same as what is stored
         * in the archive. */
        if (*rar).file.stored_crc32 > 0 as libc::c_int as libc::c_uint {
            /* Check CRC32 only when the file contains a CRC32
             * value for this file. */
            if (*rar).file.calculated_crc32 != (*rar).file.stored_crc32 {
                /* Checksums do not match; the unpacked file
                 * is corrupted. */
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Checksum error: CRC32\x00" as *const u8 as *const libc::c_char,
                );
                return ARCHIVE_FATAL;
            }
        }
        if (*rar).file.has_blake2 as libc::c_int > 0 as libc::c_int {
            /* BLAKE2sp is an optional checksum algorithm that is
             * added to RARv5 archives when using the `-htb` switch
             *  during creation of archive.
             *
             * We now finalize the hash calculation by calling the
             * `final` function. This will generate the final hash
             * value we can use to compare it with the BLAKE2sp
             * checksum that is stored in the archive.
             *
             * The return value of this `final` function is not
             * very helpful, as it guards only against improper use.
             * This is why we're explicitly ignoring it. */
            let mut b2_buf: [uint8_t; 32] = [0; 32];
            blake2sp_final(
                &mut (*rar).file.b2state,
                b2_buf.as_mut_ptr() as *mut libc::c_void,
                32 as libc::c_int as size_t,
            );
            if memcmp(
                &mut (*rar).file.blake2sp as *mut [uint8_t; 32] as *const libc::c_void,
                b2_buf.as_mut_ptr() as *const libc::c_void,
                32 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Checksum error: BLAKE2\x00" as *const u8 as *const libc::c_char,
                );
                return ARCHIVE_FATAL;
            }
        }
    }
    /* Finalization for this file has been successfully completed. */
    return ARCHIVE_OK;
}
unsafe extern "C" fn verify_global_checksums(mut a: *mut archive_read) -> libc::c_int {
    return verify_checksums(a);
}
/* Forward function declarations. */
/*
 * Decryption function for the magic signature pattern. Check the comment near
 * the `rar5_signature_xor` symbol to read the rationale behind this.
 */
unsafe extern "C" fn rar5_signature(mut buf: *mut libc::c_char) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < ::std::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong {
        *buf.offset(i as isize) =
            (rar5_signature_xor[i as usize] as libc::c_int ^ 0xa1 as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn rar5_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut rar: *mut rar5 = get_context(a);
    if (*rar).file.dir() as libc::c_int > 0 as libc::c_int {
        /* Don't process any data if this file entry was declared
         * as a directory. This is needed, because entries marked as
         * directory doesn't have any dictionary buffer allocated, so
         * it's impossible to perform any decompression. */
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Can\'t decompress an entry marked as a directory\x00" as *const u8
                as *const libc::c_char,
        );
        return ARCHIVE_FAILED;
    }
    if (*rar).skip_mode == 0 && (*rar).cstate.last_write_ptr > (*rar).file.unpacked_size {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Unpacker has written too many bytes\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    ret = use_data(rar, buff, size, offset);
    if ret == ARCHIVE_OK {
        return ret;
    }
    if (*rar).file.eof() as libc::c_int == 1 as libc::c_int {
        return ARCHIVE_EOF;
    }
    ret = do_unpack(a, rar, buff, size, offset);
    if ret != ARCHIVE_OK {
        return ret;
    }
    if (*rar).file.bytes_remaining == 0 as libc::c_int as libc::c_long
        && (*rar).cstate.last_write_ptr == (*rar).file.unpacked_size
    {
        /* If all bytes of current file were processed, run
         * finalization.
         *
         * Finalization will check checksum against proper values. If
         * some of the checksums will not match, we'll return an error
         * value in the last `archive_read_data` call to signal an error
         * to the user. */
        (*rar).file.set_eof(1 as libc::c_int as uint8_t);
        return verify_global_checksums(a);
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn rar5_read_data_skip(mut a: *mut archive_read) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    if (*rar).main.solid() != 0 {
        /* In solid archives, instead of skipping the data, we need to
         * extract it, and dispose the result. The side effect of this
         * operation will be setting up the initial window buffer state
         * needed to be able to extract the selected file. */
        let mut ret: libc::c_int = 0;
        /* Make sure to process all blocks in the compressed stream. */
        while (*rar).file.bytes_remaining > 0 as libc::c_int as libc::c_long {
            /* Setting the "skip mode" will allow us to skip
             * checksum checks during data skipping. Checking the
             * checksum of skipped data isn't really necessary and
             * it's only slowing things down.
             *
             * This is incremented instead of setting to 1 because
             * this data skipping function can be called
             * recursively. */
            (*rar).skip_mode += 1;
            /* We're disposing 1 block of data, so we use triple
             * NULLs in arguments. */
            ret = rar5_read_data(
                a,
                NULL as *mut *const libc::c_void,
                NULL as *mut size_t,
                NULL as *mut int64_t,
            );
            /* Turn off "skip mode". */
            (*rar).skip_mode -= 1;
            if ret < 0 as libc::c_int || ret == ARCHIVE_EOF {
                /* Propagate any potential error conditions
                 * to the caller. */
                return ret;
            }
        }
    } else {
        /* In standard archives, we can just jump over the compressed
         * stream. Each file in non-solid archives starts from an empty
         * window buffer. */
        if ARCHIVE_OK != consume(a, (*rar).file.bytes_remaining) {
            return ARCHIVE_FATAL;
        }
        (*rar).file.bytes_remaining = 0 as libc::c_int as ssize_t
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn rar5_seek_data(
    mut a: *mut archive_read,
    mut offset: int64_t,
    mut whence: libc::c_int,
) -> int64_t {
    /* We're a streaming unpacker, and we don't support seeking. */
    return ARCHIVE_FATAL as int64_t;
}
unsafe extern "C" fn rar5_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut rar: *mut rar5 = get_context(a);
    free((*rar).cstate.window_buf as *mut libc::c_void);
    free((*rar).cstate.filtered_buf as *mut libc::c_void);
    free((*rar).vol.push_buf as *mut libc::c_void);
    free_filters(rar);
    cdeque_free(&mut (*rar).cstate.filters);
    free(rar as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return ARCHIVE_OK;
}
unsafe extern "C" fn rar5_capabilities(mut a: *mut archive_read) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn rar5_has_encrypted_entries(mut _a: *mut archive_read) -> libc::c_int {
    /* Unsupported for now. */
    return ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED;
}
unsafe extern "C" fn rar5_init(mut rar: *mut rar5) -> libc::c_int {
    memset(
        rar as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rar5>() as libc::c_ulong,
    );
    if CDE_OK as libc::c_int != cdeque_init(&mut (*rar).cstate.filters, 8192 as libc::c_int) {
        return ARCHIVE_FATAL;
    }
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_rar5(mut _a: *mut archive) -> libc::c_int {
    let mut ar: *mut archive_read = 0 as *mut archive_read;
    let mut ret: libc::c_int = 0;
    let mut rar: *mut rar5 = 0 as *mut rar5;
    ret = get_archive_read(_a, &mut ar);
    if ARCHIVE_OK != ret {
        return ret;
    }
    rar = malloc(::std::mem::size_of::<rar5>() as libc::c_ulong) as *mut rar5;
    if rar.is_null() {
        archive_set_error(
            &mut (*ar).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate rar5 data\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    if ARCHIVE_OK != rar5_init(rar) {
        archive_set_error(
            &mut (*ar).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate rar5 filter buffer\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    ret = __archive_read_register_format(
        ar,
        rar as *mut libc::c_void,
        b"rar5\x00" as *const u8 as *const libc::c_char,
        Some(rar5_bid as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int),
        Some(
            rar5_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            rar5_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            rar5_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(rar5_read_data_skip as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
        Some(
            rar5_seek_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: int64_t,
                    _: libc::c_int,
                ) -> int64_t,
        ),
        Some(rar5_cleanup as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
        Some(rar5_capabilities as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
        Some(
            rar5_has_encrypted_entries as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
    );
    if ret != ARCHIVE_OK {
        rar5_cleanup(ar);
    }
    return ret;
}
