use ::libc;
extern "C" {
    pub type archive_string_conv;
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
    pub type archive_entry;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
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
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_conversion_set_opt(_: *mut archive_string_conv, _: libc::c_int);
    /*
     * ACL text parser.
     */
    /* wtext */
    /* type */
    #[no_mangle]
    fn archive_acl_from_text_l(
        _: *mut archive_acl,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_default_conversion_for_read(_: *mut archive) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_pathname_w(_: *mut archive_entry) -> *const wchar_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
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
    fn archive_entry_set_birthtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
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
    fn archive_entry_copy_link(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_rdev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_rdevmajor(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_rdevminor(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_symlink_type(_: *mut archive_entry, _: libc::c_int);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_copy_uname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_mac_metadata(_: *mut archive_entry, _: *const libc::c_void, _: size_t);
    #[no_mangle]
    fn archive_entry_acl(_: *mut archive_entry) -> *mut archive_acl;
    #[no_mangle]
    fn archive_entry_xattr_add_entry(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: *const libc::c_void,
        _: size_t,
    );
    #[no_mangle]
    fn archive_entry_sparse_add_entry(_: *mut archive_entry, _: la_int64_t, _: la_int64_t);
    #[no_mangle]
    fn _archive_entry_copy_gname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_copy_hardlink_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_copy_link_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_copy_pathname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_copy_symlink_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_copy_uname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
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
    fn __archive_read_ahead(
        _: *mut archive_read,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub struct tar {
    pub acl_text: archive_string,
    pub entry_pathname: archive_string,
    pub entry_pathname_override: archive_string,
    pub entry_linkpath: archive_string,
    pub entry_uname: archive_string,
    pub entry_gname: archive_string,
    pub longlink: archive_string,
    pub longname: archive_string,
    pub pax_header: archive_string,
    pub pax_global: archive_string,
    pub line: archive_string,
    pub pax_hdrcharset_binary: libc::c_int,
    pub header_recursion_depth: libc::c_int,
    pub entry_bytes_remaining: int64_t,
    pub entry_offset: int64_t,
    pub entry_padding: int64_t,
    pub entry_bytes_unconsumed: int64_t,
    pub realsize: int64_t,
    pub sparse_allowed: libc::c_int,
    pub sparse_list: *mut sparse_block,
    pub sparse_last: *mut sparse_block,
    pub sparse_offset: int64_t,
    pub sparse_numbytes: int64_t,
    pub sparse_gnu_major: libc::c_int,
    pub sparse_gnu_minor: libc::c_int,
    pub sparse_gnu_pending: libc::c_char,
    pub localname: archive_string,
    pub opt_sconv: *mut archive_string_conv,
    pub sconv: *mut archive_string_conv,
    pub sconv_acl: *mut archive_string_conv,
    pub sconv_default: *mut archive_string_conv,
    pub init_default_conversion: libc::c_int,
    pub compat_2x: libc::c_int,
    pub process_mac_extensions: libc::c_int,
    pub read_concatenated_archives: libc::c_int,
    pub realsize_override: libc::c_int,
}
/*
 * Old GNU format doesn't use POSIX 'prefix' field; they use
 * the 'L' (longname) entry instead.
 */
/*
 * Data specific to this format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_block {
    pub next: *mut sparse_block,
    pub offset: int64_t,
    pub remaining: int64_t,
    pub hole: libc::c_int,
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
/*
 * Layout of POSIX 'ustar' tar header.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_entry_header_ustar {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub checksum: [libc::c_char; 8],
    pub typeflag: [libc::c_char; 1],
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub rdevmajor: [libc::c_char; 8],
    pub rdevminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_entry_header_gnutar {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub checksum: [libc::c_char; 8],
    pub typeflag: [libc::c_char; 1],
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 8],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub rdevmajor: [libc::c_char; 8],
    pub rdevminor: [libc::c_char; 8],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
    pub offset: [libc::c_char; 12],
    pub longnames: [libc::c_char; 4],
    pub unused: [libc::c_char; 1],
    pub sparse: [gnu_sparse; 4],
    pub isextended: [libc::c_char; 1],
    pub realsize: [libc::c_char; 12],
}
/*
 * Structure of GNU tar header
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gnu_sparse {
    pub offset: [libc::c_char; 12],
    pub numbytes: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extended {
    pub sparse: [gnu_sparse; 21],
    pub isextended: [libc::c_char; 1],
    pub padding: [libc::c_char; 7],
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
pub const INT64_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
pub const INT64_MIN: libc::c_long =
    -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long;
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const errno: libc::c_int = *__errno_location();
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_USTAR: libc::c_int = ARCHIVE_FORMAT_TAR | 1 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE: libc::c_int = ARCHIVE_FORMAT_TAR | 2 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FORMAT_TAR_GNUTAR: libc::c_int = ARCHIVE_FORMAT_TAR | 4 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR: libc::c_int = 0x30000 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const SCONV_SET_OPT_UTF8_LIBARCHIVE2X: libc::c_int = 1 as libc::c_int;
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
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
/*
 * Symlink types
 */
pub const AE_SYMLINK_TYPE_FILE: libc::c_int = 1 as libc::c_int;
pub const AE_SYMLINK_TYPE_DIRECTORY: libc::c_int = 2 as libc::c_int;
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
pub const ARCHIVE_ENTRY_ACL_TYPE_ALLOW: libc::c_int = 0x400 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_DENY: libc::c_int = 0x800 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_AUDIT: libc::c_int = 0x1000 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_ALARM: libc::c_int = 0x2000 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_NFS4: libc::c_int = ARCHIVE_ENTRY_ACL_TYPE_ALLOW
    | ARCHIVE_ENTRY_ACL_TYPE_DENY
    | ARCHIVE_ENTRY_ACL_TYPE_AUDIT
    | ARCHIVE_ENTRY_ACL_TYPE_ALARM;
pub const archive_entry_copy_gname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_gname_l;
pub const archive_entry_copy_hardlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_hardlink_l;
pub const archive_entry_copy_link_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_link_l;
pub const archive_entry_copy_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_pathname_l;
pub const archive_entry_copy_symlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_symlink_l;
pub const archive_entry_copy_uname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_uname_l;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_gnutar(mut a: *mut archive) -> libc::c_int {
    let mut magic_test: libc::c_int = __archive_check_magic(
        a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_gnutar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    return archive_read_support_format_tar(a);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_tar(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut tar: *mut tar = 0 as *mut tar;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_tar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    tar = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<tar>() as libc::c_ulong,
    ) as *mut tar;
    if tar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate tar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    r = __archive_read_register_format(
        a,
        tar as *mut libc::c_void,
        b"tar\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_tar_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_tar_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_tar_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_tar_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_tar_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_tar_cleanup
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
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
        free(tar as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_tar_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut tar: *mut tar = 0 as *mut tar;
    tar = (*(*a).format).data as *mut tar;
    gnu_clear_sparse_list(tar);
    archive_string_free(&mut (*tar).acl_text);
    archive_string_free(&mut (*tar).entry_pathname);
    archive_string_free(&mut (*tar).entry_pathname_override);
    archive_string_free(&mut (*tar).entry_linkpath);
    archive_string_free(&mut (*tar).entry_uname);
    archive_string_free(&mut (*tar).entry_gname);
    archive_string_free(&mut (*tar).line);
    archive_string_free(&mut (*tar).pax_global);
    archive_string_free(&mut (*tar).pax_header);
    archive_string_free(&mut (*tar).longname);
    archive_string_free(&mut (*tar).longlink);
    archive_string_free(&mut (*tar).localname);
    free(tar as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
 * Validate number field
 *
 * This has to be pretty lenient in order to accommodate the enormous
 * variety of tar writers in the world:
 *  = POSIX (IEEE Std 1003.1-1988) ustar requires octal values with leading
 *    zeros and allows fields to be terminated with space or null characters
 *  = Many writers use different termination (in particular, libarchive
 *    omits terminator bytes to squeeze one or two more digits)
 *  = Many writers pad with space and omit leading zeros
 *  = GNU tar and star write base-256 values if numbers are too
 *    big to be represented in octal
 *
 *  Examples of specific tar headers that we should support:
 *  = Perl Archive::Tar terminates uid, gid, devminor and devmajor with two
 *    null bytes, pads size with spaces and other numeric fields with zeroes
 *  = plexus-archiver prior to 2.6.3 (before switching to commons-compress)
 *    may have uid and gid fields filled with spaces without any octal digits
 *    at all and pads all numeric fields with spaces
 *
 * This should tolerate all variants in use.  It will reject a field
 * where the writer just left garbage after a trailing NUL.
 */
unsafe extern "C" fn validate_number_field(
    mut p_field: *const libc::c_char,
    mut i_size: size_t,
) -> libc::c_int {
    let mut marker: libc::c_uchar = *p_field.offset(0 as libc::c_int as isize) as libc::c_uchar;
    if marker as libc::c_int == 128 as libc::c_int
        || marker as libc::c_int == 255 as libc::c_int
        || marker as libc::c_int == 0 as libc::c_int
    {
        /* Base-256 marker, there's nothing we can check. */
        return 1 as libc::c_int;
    } else {
        /* Must be octal */
        let mut i: size_t = 0 as libc::c_int as size_t;
        /* Skip any leading spaces */
        while i < i_size && *p_field.offset(i as isize) as libc::c_int == ' ' as i32 {
            i = i.wrapping_add(1)
        }
        /* Skip octal digits. */
        while i < i_size
            && *p_field.offset(i as isize) as libc::c_int >= '0' as i32
            && *p_field.offset(i as isize) as libc::c_int <= '7' as i32
        {
            i = i.wrapping_add(1)
        }
        /* Any remaining characters must be space or NUL padding. */
        while i < i_size {
            if *p_field.offset(i as isize) as libc::c_int != ' ' as i32
                && *p_field.offset(i as isize) as libc::c_int != 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            i = i.wrapping_add(1)
        }
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn archive_read_format_tar_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut bid: libc::c_int = 0;
    let mut h: *const libc::c_char = 0 as *const libc::c_char;
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    /* UNUSED */
    bid = 0 as libc::c_int;
    /* Now let's look at the actual header and see if it matches. */
    h = __archive_read_ahead(a, 512 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    /* If it's an end-of-archive mark, we can handle it. */
    if *h.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        && archive_block_is_null(h) != 0
    {
        /*
         * Usually, I bid the number of bits verified, but
         * in this case, 4096 seems excessive so I picked 10 as
         * an arbitrary but reasonable-seeming value.
         */
        return 10 as libc::c_int;
    }
    /* If it's not an end-of-archive mark, it must have a valid checksum.*/
    if checksum(a, h as *const libc::c_void) == 0 {
        return 0 as libc::c_int;
    } /* Checksum is usually 6 octal digits. */
    bid += 48 as libc::c_int;
    header = h as *const archive_entry_header_ustar;
    /* Recognize POSIX formats. */
    if memcmp(
        (*header).magic.as_ptr() as *const libc::c_void,
        b"ustar\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && memcmp(
            (*header).version.as_ptr() as *const libc::c_void,
            b"00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        bid += 56 as libc::c_int
    }
    /* Recognize GNU tar format. */
    if memcmp(
        (*header).magic.as_ptr() as *const libc::c_void,
        b"ustar \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        && memcmp(
            (*header).version.as_ptr() as *const libc::c_void,
            b" \x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        bid += 56 as libc::c_int
    }
    /* Type flag must be null, digit or A-Z, a-z. */
    if (*header).typeflag[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
        && !((*header).typeflag[0 as libc::c_int as usize] as libc::c_int >= '0' as i32
            && (*header).typeflag[0 as libc::c_int as usize] as libc::c_int <= '9' as i32)
        && !((*header).typeflag[0 as libc::c_int as usize] as libc::c_int >= 'A' as i32
            && (*header).typeflag[0 as libc::c_int as usize] as libc::c_int <= 'Z' as i32)
        && !((*header).typeflag[0 as libc::c_int as usize] as libc::c_int >= 'a' as i32
            && (*header).typeflag[0 as libc::c_int as usize] as libc::c_int <= 'z' as i32)
    {
        return 0 as libc::c_int;
    } /* 6 bits of variation in an 8-bit field leaves 2 bits. */
    bid += 2 as libc::c_int;
    /*
     * Check format of mode/uid/gid/mtime/size/rdevmajor/rdevminor fields.
     */
    if bid > 0 as libc::c_int
        && (validate_number_field(
            (*header).mode.as_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        ) == 0 as libc::c_int
            || validate_number_field(
                (*header).uid.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            || validate_number_field(
                (*header).gid.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            || validate_number_field(
                (*header).mtime.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            || validate_number_field(
                (*header).size.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            || validate_number_field(
                (*header).rdevmajor.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            || validate_number_field(
                (*header).rdevminor.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) == 0 as libc::c_int)
    {
        bid = 0 as libc::c_int
    }
    return bid;
}
unsafe extern "C" fn archive_read_format_tar_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut tar: *mut tar = 0 as *mut tar;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    tar = (*(*a).format).data as *mut tar;
    if strcmp(key, b"compat-2x\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /* Handle UTF-8 filenames as libarchive 2.x */
        (*tar).compat_2x = (!val.is_null()
            && *val.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
        (*tar).init_default_conversion = (*tar).compat_2x;
        return 0 as libc::c_int;
    } else {
        if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            if val.is_null()
                || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"tar: hdrcharset option needs a character-set name\x00" as *const u8
                        as *const libc::c_char,
                );
            } else {
                (*tar).opt_sconv = archive_string_conversion_from_charset(
                    &mut (*a).archive,
                    val,
                    0 as libc::c_int,
                );
                if !(*tar).opt_sconv.is_null() {
                    ret = ARCHIVE_OK
                } else {
                    ret = ARCHIVE_FATAL
                }
            }
            return ret;
        } else {
            if strcmp(key, b"mac-ext\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*tar).process_mac_extensions = (!val.is_null()
                    && *val.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int)
                    as libc::c_int;
                return 0 as libc::c_int;
            } else {
                if strcmp(
                    key,
                    b"read_concatenated_archives\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*tar).read_concatenated_archives = (!val.is_null()
                        && *val.offset(0 as libc::c_int as isize) as libc::c_int
                            != 0 as libc::c_int)
                        as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
        }
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
/* utility function- this exists to centralize the logic of tracking
 * how much unconsumed data we have floating around, and to consume
 * anything outstanding since we're going to do read_aheads
 */
unsafe extern "C" fn tar_flush_unconsumed(mut a: *mut archive_read, mut unconsumed: *mut size_t) {
    if *unconsumed != 0 {
        /*
                void *data = (void *)__archive_read_ahead(a, *unconsumed, NULL);
                 * this block of code is to poison claimed unconsumed space, ensuring
                 * things break if it is in use still.
                 * currently it WILL break things, so enable it only for debugging this issue
                if (data) {
                    memset(data, 0xff, *unconsumed);
                }
        */
        __archive_read_consume(a, *unconsumed as int64_t);
        *unconsumed = 0 as libc::c_int as size_t
    };
}
/*
 * The function invoked by archive_read_next_header().  This
 * just sets up a few things and then calls the internal
 * tar_read_header() function below.
 */
unsafe extern "C" fn archive_read_format_tar_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    /*
     * When converting tar archives to cpio archives, it is
     * essential that each distinct file have a distinct inode
     * number.  To simplify this, we keep a static count here to
     * assign fake dev/inode numbers to each tar entry.  Note that
     * pax format archives may overwrite this with something more
     * useful.
     *
     * Ideally, we would track every file read from the archive so
     * that we could assign the same dev/ino pair to hardlinks,
     * but the memory required to store a complete lookup table is
     * probably not worthwhile just to support the relatively
     * obscure tar->cpio conversion case.
     */
    static mut default_inode: libc::c_int = 0;
    static mut default_dev: libc::c_int = 0;
    let mut tar: *mut tar = 0 as *mut tar;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut wp: *const wchar_t = 0 as *const wchar_t;
    let mut r: libc::c_int = 0;
    let mut l: size_t = 0;
    let mut unconsumed: size_t = 0 as libc::c_int as size_t;
    /* Assign default device/inode values. */
    archive_entry_set_dev(entry, (1 as libc::c_int + default_dev) as dev_t); /* Don't use zero. */
    default_inode += 1; /* Don't use zero. */
    archive_entry_set_ino(entry, default_inode as la_int64_t);
    /* Limit generated st_ino number to 16 bits. */
    if default_inode >= 0xffff as libc::c_int {
        default_dev += 1; /* Mark this as "unset" */
        default_inode = 0 as libc::c_int
    }
    tar = (*(*a).format).data as *mut tar;
    (*tar).entry_offset = 0 as libc::c_int as int64_t;
    gnu_clear_sparse_list(tar);
    (*tar).realsize = -(1 as libc::c_int) as int64_t;
    (*tar).realsize_override = 0 as libc::c_int;
    /* Setup default string conversion. */
    (*tar).sconv = (*tar).opt_sconv;
    if (*tar).sconv.is_null() {
        if (*tar).init_default_conversion == 0 {
            (*tar).sconv_default = archive_string_default_conversion_for_read(&mut (*a).archive);
            (*tar).init_default_conversion = 1 as libc::c_int
        }
        (*tar).sconv = (*tar).sconv_default
    }
    r = tar_read_header(a, tar, entry, &mut unconsumed);
    tar_flush_unconsumed(a, &mut unconsumed);
    /*
     * "non-sparse" files are really just sparse files with
     * a single block.
     */
    if (*tar).sparse_list.is_null() {
        if gnu_add_sparse_entry(
            a,
            tar,
            0 as libc::c_int as int64_t,
            (*tar).entry_bytes_remaining,
        ) != ARCHIVE_OK
        {
            return -(30 as libc::c_int);
        }
    } else {
        let mut sb: *mut sparse_block = 0 as *mut sparse_block;
        sb = (*tar).sparse_list;
        while !sb.is_null() {
            if (*sb).hole == 0 {
                archive_entry_sparse_add_entry(entry, (*sb).offset, (*sb).remaining);
            }
            sb = (*sb).next
        }
    }
    if r == ARCHIVE_OK && archive_entry_filetype(entry) == AE_IFREG as mode_t {
        /*
         * "Regular" entry with trailing '/' is really
         * directory: This is needed for certain old tar
         * variants and even for some broken newer ones.
         */
        wp = archive_entry_pathname_w(entry);
        if !wp.is_null() {
            l = wcslen(wp);
            if l > 0 as libc::c_int as libc::c_ulong
                && *wp.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    == '/' as i32
            {
                archive_entry_set_filetype(entry, AE_IFDIR as mode_t);
            }
        } else {
            p = archive_entry_pathname(entry);
            if !p.is_null() {
                l = strlen(p);
                if l > 0 as libc::c_int as libc::c_ulong
                    && *p.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        == '/' as i32
                {
                    archive_entry_set_filetype(entry, AE_IFDIR as mode_t);
                }
            }
        }
    }
    return r;
}
unsafe extern "C" fn archive_read_format_tar_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut bytes_read: ssize_t = 0;
    let mut tar: *mut tar = 0 as *mut tar;
    let mut p: *mut sparse_block = 0 as *mut sparse_block;
    tar = (*(*a).format).data as *mut tar;
    loop {
        /* Remove exhausted entries from sparse list. */
        while !(*tar).sparse_list.is_null()
            && (*(*tar).sparse_list).remaining == 0 as libc::c_int as libc::c_long
        {
            p = (*tar).sparse_list;
            (*tar).sparse_list = (*p).next;
            free(p as *mut libc::c_void);
        }
        if (*tar).entry_bytes_unconsumed != 0 {
            __archive_read_consume(a, (*tar).entry_bytes_unconsumed);
            (*tar).entry_bytes_unconsumed = 0 as libc::c_int as int64_t
        }
        /* Current is hole data and skip this. */
        if (*tar).sparse_list.is_null()
            || (*tar).entry_bytes_remaining == 0 as libc::c_int as libc::c_long
        {
            if __archive_read_consume(a, (*tar).entry_padding) < 0 as libc::c_int as libc::c_long {
                return -(30 as libc::c_int);
            }
            (*tar).entry_padding = 0 as libc::c_int as int64_t;
            *buff = NULL as *const libc::c_void;
            *size = 0 as libc::c_int as size_t;
            *offset = (*tar).realsize;
            return 1 as libc::c_int;
        }
        *buff = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read);
        if bytes_read < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int);
        }
        if *buff == NULL as *const libc::c_void {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Truncated tar archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if bytes_read > (*tar).entry_bytes_remaining {
            bytes_read = (*tar).entry_bytes_remaining
        }
        if (*(*tar).sparse_list).remaining < bytes_read {
            bytes_read = (*(*tar).sparse_list).remaining
        }
        *size = bytes_read as size_t;
        *offset = (*(*tar).sparse_list).offset;
        (*(*tar).sparse_list).remaining -= bytes_read;
        (*(*tar).sparse_list).offset += bytes_read;
        (*tar).entry_bytes_remaining -= bytes_read;
        (*tar).entry_bytes_unconsumed = bytes_read;
        if (*(*tar).sparse_list).hole == 0 {
            return 0 as libc::c_int;
        }
    }
}
unsafe extern "C" fn archive_read_format_tar_skip(mut a: *mut archive_read) -> libc::c_int {
    let mut bytes_skipped: int64_t = 0;
    let mut request: int64_t = 0;
    let mut p: *mut sparse_block = 0 as *mut sparse_block;
    let mut tar: *mut tar = 0 as *mut tar;
    tar = (*(*a).format).data as *mut tar;
    /* If we're at end of file, return EOF. */
    /* Don't read more than is available in the
     * current sparse block. */
    /* Do not consume the hole of a sparse file. */
    request = 0 as libc::c_int as int64_t;
    p = (*tar).sparse_list;
    while !p.is_null() {
        if (*p).hole == 0 {
            if (*p).remaining >= INT64_MAX - request {
                return ARCHIVE_FATAL;
            }
            request += (*p).remaining
        }
        p = (*p).next
    }
    if request > (*tar).entry_bytes_remaining {
        request = (*tar).entry_bytes_remaining
    }
    request += (*tar).entry_padding + (*tar).entry_bytes_unconsumed;
    bytes_skipped = __archive_read_consume(a, request);
    if bytes_skipped < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
    (*tar).entry_bytes_unconsumed = 0 as libc::c_int as int64_t;
    (*tar).entry_padding = 0 as libc::c_int as int64_t;
    /* Free the sparse list. */
    gnu_clear_sparse_list(tar);
    return 0 as libc::c_int;
}
/*
 * This function recursively interprets all of the headers associated
 * with a single entry.
 */
unsafe extern "C" fn tar_read_header(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut bytes: ssize_t = 0;
    let mut err: libc::c_int = 0;
    let mut eof_vol_header: libc::c_int = 0;
    let mut h: *const libc::c_char = 0 as *const libc::c_char;
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    let mut gnuheader: *const archive_entry_header_gnutar = 0 as *const archive_entry_header_gnutar;
    eof_vol_header = 0 as libc::c_int;
    loop
    /* Loop until we find a workable header record. */
    {
        tar_flush_unconsumed(a, unconsumed);
        /* Read 512-byte header record */
        h = __archive_read_ahead(a, 512 as libc::c_int as size_t, &mut bytes)
            as *const libc::c_char;
        if bytes < 0 as libc::c_int as libc::c_long {
            return bytes as libc::c_int;
        }
        if bytes == 0 as libc::c_int as libc::c_long {
            /* EOF at a block boundary. */
            /* Some writers do omit the block of nulls. <sigh> */
            return 1 as libc::c_int;
        }
        if bytes < 512 as libc::c_int as libc::c_long {
            /* Short block at EOF; this is bad. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated tar archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        *unconsumed = 512 as libc::c_int as size_t;
        /* Header is workable if it's not an end-of-archive mark. */
        if *h.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
            || archive_block_is_null(h) == 0
        {
            break;
        }
        /* Ensure format is set for archives with only null blocks. */
        if (*a).archive.archive_format_name.is_null() {
            (*a).archive.archive_format = ARCHIVE_FORMAT_TAR;
            (*a).archive.archive_format_name = b"tar\x00" as *const u8 as *const libc::c_char
        }
        if (*tar).read_concatenated_archives == 0 {
            /* Try to consume a second all-null record, as well. */
            tar_flush_unconsumed(a, unconsumed);
            h = __archive_read_ahead(a, 512 as libc::c_int as size_t, NULL as *mut ssize_t)
                as *const libc::c_char;
            if !h.is_null()
                && *h.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                && archive_block_is_null(h) != 0
            {
                __archive_read_consume(a, 512 as libc::c_int as int64_t);
            }
            archive_clear_error(&mut (*a).archive);
            return 1 as libc::c_int;
        }
    }
    /*
     * Note: If the checksum fails and we return ARCHIVE_RETRY,
     * then the client is likely to just retry.  This is a very
     * crude way to search for the next valid header!
     *
     * TODO: Improve this by implementing a real header scan.
     */
    if checksum(a, h as *const libc::c_void) == 0 {
        tar_flush_unconsumed(a, unconsumed);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Damaged tar archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(10 as libc::c_int);
        /* Retryable: Invalid header */
    }
    (*tar).header_recursion_depth += 1;
    if (*tar).header_recursion_depth > 32 as libc::c_int {
        tar_flush_unconsumed(a, unconsumed);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Too many special headers\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    /* Determine the format variant. */
    header = h as *const archive_entry_header_ustar;
    match (*header).typeflag[0 as libc::c_int as usize] as libc::c_int {
        65 => {
            /* Solaris tar ACL */
            (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE;
            (*a).archive.archive_format_name =
                b"Solaris tar\x00" as *const u8 as *const libc::c_char;
            err = header_Solaris_ACL(a, tar, entry, h as *const libc::c_void, unconsumed)
        }
        103 => {
            /* POSIX-standard 'g' header. */
            (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE;
            (*a).archive.archive_format_name =
                b"POSIX pax interchange format\x00" as *const u8 as *const libc::c_char;
            err = header_pax_global(a, tar, entry, h as *const libc::c_void, unconsumed);
            if err == ARCHIVE_EOF {
                return err;
            }
        }
        75 => {
            /* Long link name (GNU tar, others) */
            err = header_longlink(a, tar, entry, h as *const libc::c_void, unconsumed)
        }
        76 => {
            /* Long filename (GNU tar, others) */
            err = header_longname(a, tar, entry, h as *const libc::c_void, unconsumed)
        }
        86 => {
            /* GNU volume header */
            err = header_volume(a, tar, entry, h as *const libc::c_void, unconsumed);
            if err == ARCHIVE_EOF {
                eof_vol_header = 1 as libc::c_int
            }
        }
        88 => {
            /* Used by SUN tar; same as 'x'. */
            (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE;
            (*a).archive.archive_format_name = b"POSIX pax interchange format (Sun variant)\x00"
                as *const u8 as *const libc::c_char;
            err = header_pax_extensions(a, tar, entry, h as *const libc::c_void, unconsumed)
        }
        120 => {
            /* POSIX-standard 'x' header. */
            (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE;
            (*a).archive.archive_format_name =
                b"POSIX pax interchange format\x00" as *const u8 as *const libc::c_char;
            err = header_pax_extensions(a, tar, entry, h as *const libc::c_void, unconsumed)
        }
        _ => {
            gnuheader = h as *const archive_entry_header_gnutar;
            if memcmp(
                (*gnuheader).magic.as_ptr() as *const libc::c_void,
                b"ustar  \x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_GNUTAR;
                (*a).archive.archive_format_name =
                    b"GNU tar format\x00" as *const u8 as *const libc::c_char;
                err = header_gnutar(a, tar, entry, h as *const libc::c_void, unconsumed)
            } else if memcmp(
                (*header).magic.as_ptr() as *const libc::c_void,
                b"ustar\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                if (*a).archive.archive_format != ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE {
                    (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_USTAR;
                    (*a).archive.archive_format_name =
                        b"POSIX ustar format\x00" as *const u8 as *const libc::c_char
                }
                err = header_ustar(a, tar, entry, h as *const libc::c_void)
            } else {
                (*a).archive.archive_format = ARCHIVE_FORMAT_TAR;
                (*a).archive.archive_format_name =
                    b"tar (non-POSIX)\x00" as *const u8 as *const libc::c_char;
                err = header_old_tar(a, tar, entry, h as *const libc::c_void)
            }
        }
    }
    if err == ARCHIVE_FATAL {
        return err;
    }
    tar_flush_unconsumed(a, unconsumed);
    h = NULL as *const libc::c_char;
    header = NULL as *const archive_entry_header_ustar;
    (*tar).header_recursion_depth -= 1;
    /* Yuck.  Apple's design here ends up storing long pathname
     * extensions for both the AppleDouble extension entry and the
     * regular entry.
     */
    if (err == ARCHIVE_WARN || err == ARCHIVE_OK)
        && (*tar).header_recursion_depth == 0 as libc::c_int
        && (*tar).process_mac_extensions != 0
    {
        let mut err2: libc::c_int =
            read_mac_metadata_blob(a, tar, entry, h as *const libc::c_void, unconsumed);
        if err2 < err {
            err = err2
        }
    }
    /* We return warnings or success as-is.  Anything else is fatal. */
    if err == ARCHIVE_WARN || err == ARCHIVE_OK {
        if (*tar).sparse_gnu_pending != 0 {
            if (*tar).sparse_gnu_major == 1 as libc::c_int
                && (*tar).sparse_gnu_minor == 0 as libc::c_int
            {
                let mut bytes_read: ssize_t = 0;
                (*tar).sparse_gnu_pending = 0 as libc::c_int as libc::c_char;
                /* Read initial sparse map. */
                bytes_read = gnu_sparse_10_read(a, tar, unconsumed);
                if bytes_read < 0 as libc::c_int as libc::c_long {
                    return bytes_read as libc::c_int;
                }
                (*tar).entry_bytes_remaining -= bytes_read
            } else {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Unrecognized GNU sparse file format\x00" as *const u8 as *const libc::c_char,
                );
                return -(20 as libc::c_int);
            }
            (*tar).sparse_gnu_pending = 0 as libc::c_int as libc::c_char
        }
        return err;
    }
    if err == ARCHIVE_EOF {
        if eof_vol_header == 0 {
            /* EOF when recursively reading a header is bad. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"Damaged tar archive\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            /* If we encounter just a GNU volume header treat
             * this situation as an empty archive */
            return 1 as libc::c_int;
        }
    }
    return -(30 as libc::c_int);
}
/*
 * Return true if block checksum is correct.
 */
unsafe extern "C" fn checksum(mut a: *mut archive_read, mut h: *const libc::c_void) -> libc::c_int {
    let mut bytes: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    let mut check: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut i: size_t = 0;
    /* UNUSED */
    bytes = h as *const libc::c_uchar;
    header = h as *const archive_entry_header_ustar;
    /* Checksum field must hold an octal number */
    i = 0 as libc::c_int as size_t;
    while i < ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong {
        let mut c: libc::c_char = (*header).checksum[i as usize];
        if c as libc::c_int != ' ' as i32
            && c as libc::c_int != '\u{0}' as i32
            && ((c as libc::c_int) < '0' as i32 || c as libc::c_int > '7' as i32)
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    /*
     * Test the checksum.  Note that POSIX specifies _unsigned_
     * bytes for this calculation.
     */
    sum = tar_atol(
        (*header).checksum.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    ) as libc::c_int;
    check = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 148 as libc::c_int as libc::c_ulong {
        check += *bytes.offset(i as isize) as libc::c_int;
        i = i.wrapping_add(1)
    }
    while i < 156 as libc::c_int as libc::c_ulong {
        check += 32 as libc::c_int;
        i = i.wrapping_add(1)
    }
    while i < 512 as libc::c_int as libc::c_ulong {
        check += *bytes.offset(i as isize) as libc::c_int;
        i = i.wrapping_add(1)
    }
    if sum == check {
        return 1 as libc::c_int;
    }
    /*
     * Repeat test with _signed_ bytes, just in case this archive
     * was created by an old BSD, Solaris, or HP-UX tar with a
     * broken checksum calculation.
     */
    check = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < 148 as libc::c_int as libc::c_ulong {
        check += *bytes.offset(i as isize) as libc::c_schar as libc::c_int;
        i = i.wrapping_add(1)
    }
    while i < 156 as libc::c_int as libc::c_ulong {
        check += 32 as libc::c_int;
        i = i.wrapping_add(1)
    }
    while i < 512 as libc::c_int as libc::c_ulong {
        check += *bytes.offset(i as isize) as libc::c_schar as libc::c_int;
        i = i.wrapping_add(1)
    }
    if sum == check {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*
 * Return true if this block contains only nulls.
 */
unsafe extern "C" fn archive_block_is_null(mut p: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 512 as libc::c_int as libc::c_uint {
        let fresh0 = p;
        p = p.offset(1);
        if *fresh0 != 0 {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/*
 * Interpret 'A' Solaris ACL header
 */
unsafe extern "C" fn header_Solaris_ACL(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    let mut size: size_t = 0;
    let mut err: libc::c_int = 0;
    let mut acl_type: libc::c_int = 0;
    let mut type_0: int64_t = 0;
    let mut acl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    /*
     * read_body_to_string adds a NUL terminator, but we need a little
     * more to make sure that we don't overrun acl_text later.
     */
    header = h as *const archive_entry_header_ustar;
    size = tar_atol(
        (*header).size.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    ) as size_t;
    err = read_body_to_string(a, tar, &mut (*tar).acl_text, h, unconsumed);
    if err != ARCHIVE_OK {
        return err;
    }
    /* Recursively read next header */
    err = tar_read_header(a, tar, entry, unconsumed);
    if err != ARCHIVE_OK && err != ARCHIVE_WARN {
        return err;
    }
    /* TODO: Examine the first characters to see if this
     * is an AIX ACL descriptor.  We'll likely never support
     * them, but it would be polite to recognize and warn when
     * we do see them. */
    /* Leading octal number indicates ACL type and number of entries. */
    acl = (*tar).acl_text.s;
    p = acl;
    type_0 = 0 as libc::c_int as int64_t;
    while *p as libc::c_int != '\u{0}' as i32 && p < acl.offset(size as isize) {
        if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '7' as i32 {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Malformed Solaris ACL attribute (invalid digit)\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(20 as libc::c_int);
        }
        type_0 <<= 3 as libc::c_int;
        type_0 += (*p as libc::c_int - '0' as i32) as libc::c_long;
        if type_0 > 0o77777777 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Malformed Solaris ACL attribute (count too large)\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(20 as libc::c_int);
        }
        p = p.offset(1)
    }
    match type_0 as libc::c_int & !(0o777777 as libc::c_int) {
        262144 => {
            /* POSIX.1e ACL */
            acl_type = ARCHIVE_ENTRY_ACL_TYPE_ACCESS
        }
        786432 => {
            /* NFSv4 ACL */
            acl_type = ARCHIVE_ENTRY_ACL_TYPE_NFS4
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Malformed Solaris ACL attribute (unsupported type %o)\x00" as *const u8
                    as *const libc::c_char,
                type_0 as libc::c_int,
            );
            return -(20 as libc::c_int);
        }
    }
    p = p.offset(1);
    if p >= acl.offset(size as isize) {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Malformed Solaris ACL attribute (body overflow)\x00" as *const u8
                as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    /* ACL text is null-terminated; find the end. */
    size = (size as libc::c_ulong)
        .wrapping_sub(p.offset_from(acl) as libc::c_long as libc::c_ulong)
        as size_t as size_t;
    acl = p;
    while *p as libc::c_int != '\u{0}' as i32 && p < acl.offset(size as isize) {
        p = p.offset(1)
    }
    if (*tar).sconv_acl.is_null() {
        (*tar).sconv_acl = archive_string_conversion_from_charset(
            &mut (*a).archive,
            b"UTF-8\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if (*tar).sconv_acl.is_null() {
            return -(30 as libc::c_int);
        }
    }
    (*tar).localname.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*tar).localname,
        acl as *const libc::c_void,
        p.offset_from(acl) as libc::c_long as size_t,
    );
    err = archive_acl_from_text_l(
        archive_entry_acl(entry),
        (*tar).localname.s,
        acl_type,
        (*tar).sconv_acl,
    );
    if err != ARCHIVE_OK {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for ACL\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Malformed Solaris ACL attribute (unparsable)\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return err;
}
/*
 * Interpret 'K' long linkname header.
 */
unsafe extern "C" fn header_longlink(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = read_body_to_string(a, tar, &mut (*tar).longlink, h, unconsumed);
    if err != ARCHIVE_OK {
        return err;
    }
    err = tar_read_header(a, tar, entry, unconsumed);
    if err != ARCHIVE_OK && err != ARCHIVE_WARN {
        return err;
    }
    /* Set symlink if symlink already set, else hardlink. */
    archive_entry_copy_link(entry, (*tar).longlink.s);
    return 0 as libc::c_int;
}
unsafe extern "C" fn set_conversion_failed_error(
    mut a: *mut archive_read,
    mut sconv: *mut archive_string_conv,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if errno == ENOMEM {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
        return -(30 as libc::c_int);
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"%s can\'t be converted from %s to current locale.\x00" as *const u8
            as *const libc::c_char,
        name,
        archive_string_conversion_charset_name(sconv),
    );
    return -(20 as libc::c_int);
}
/*
 * Interpret 'L' long filename header.
 */
unsafe extern "C" fn header_longname(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = read_body_to_string(a, tar, &mut (*tar).longname, h, unconsumed);
    if err != ARCHIVE_OK {
        return err;
    }
    /* Read and parse "real" header, then override name. */
    err = tar_read_header(a, tar, entry, unconsumed);
    if err != ARCHIVE_OK && err != ARCHIVE_WARN {
        return err;
    }
    if _archive_entry_copy_pathname_l(
        entry,
        (*tar).longname.s,
        (*tar).longname.length,
        (*tar).sconv,
    ) != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Pathname\x00" as *const u8 as *const libc::c_char,
        )
    }
    return err;
}
/*
 * Interpret 'V' GNU tar volume header.
 */
unsafe extern "C" fn header_volume(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    /* Just skip this and read the next header. */
    return tar_read_header(a, tar, entry, unconsumed);
}
/*
 * Read body of an archive entry into an archive_string object.
 */
unsafe extern "C" fn read_body_to_string(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut as_0: *mut archive_string,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut size: int64_t = 0;
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    let mut src: *const libc::c_void = 0 as *const libc::c_void;
    /* UNUSED */
    header = h as *const archive_entry_header_ustar;
    size = tar_atol(
        (*header).size.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    if size > 1048576 as libc::c_int as libc::c_long || size < 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Special header too large\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Fail if we can't make our buffer big enough. */
    if archive_string_ensure(
        as_0,
        (size as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    tar_flush_unconsumed(a, unconsumed);
    /* Read the body into the string. */
    *unconsumed = (size + 511 as libc::c_int as libc::c_long
        & !(511 as libc::c_int) as libc::c_long) as size_t;
    src = __archive_read_ahead(a, *unconsumed, NULL as *mut ssize_t);
    if src == NULL as *const libc::c_void {
        *unconsumed = 0 as libc::c_int as size_t;
        return -(30 as libc::c_int);
    }
    memcpy((*as_0).s as *mut libc::c_void, src, size as size_t);
    *(*as_0).s.offset(size as isize) = '\u{0}' as i32 as libc::c_char;
    (*as_0).length = size as size_t;
    return 0 as libc::c_int;
}
/*
 * Parse out common header elements.
 *
 * This would be the same as header_old_tar, except that the
 * filename is handled slightly differently for old and POSIX
 * entries  (POSIX entries support a 'prefix').  This factoring
 * allows header_old_tar and header_ustar
 * to handle filenames differently, while still putting most of the
 * common parsing into one place.
 */
unsafe extern "C" fn header_common(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
) -> libc::c_int {
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    let mut tartype: libc::c_char = 0;
    let mut err: libc::c_int = ARCHIVE_OK;
    header = h as *const archive_entry_header_ustar;
    if (*header).linkname[0 as libc::c_int as usize] != 0 {
        (*tar).entry_linkpath.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut (*tar).entry_linkpath,
            (*header).linkname.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        );
    } else {
        (*tar).entry_linkpath.length = 0 as libc::c_int as size_t
    }
    /* Parse out the numeric fields (all are octal) */
    archive_entry_set_mode(
        entry,
        tar_atol(
            (*header).mode.as_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        ) as mode_t,
    );
    archive_entry_set_uid(
        entry,
        tar_atol(
            (*header).uid.as_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        ),
    );
    archive_entry_set_gid(
        entry,
        tar_atol(
            (*header).gid.as_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        ),
    );
    (*tar).entry_bytes_remaining = tar_atol(
        (*header).size.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    if (*tar).entry_bytes_remaining < 0 as libc::c_int as libc::c_long {
        (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Tar entry has negative size\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*tar).entry_bytes_remaining == INT64_MAX {
        /* Note: tar_atol returns INT64_MAX on overflow */
        (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Tar entry size overflow\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*tar).realsize = (*tar).entry_bytes_remaining;
    archive_entry_set_size(entry, (*tar).entry_bytes_remaining);
    archive_entry_set_mtime(
        entry,
        tar_atol(
            (*header).mtime.as_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        ),
        0 as libc::c_int as libc::c_long,
    );
    /* Handle the tar type flag appropriately. */
    tartype = (*header).typeflag[0 as libc::c_int as usize];
    let mut current_block_64: u64;
    match tartype as libc::c_int {
        49 => {
            /* Hard link */
            if _archive_entry_copy_hardlink_l(
                entry,
                (*tar).entry_linkpath.s,
                (*tar).entry_linkpath.length,
                (*tar).sconv,
            ) != 0 as libc::c_int
            {
                err = set_conversion_failed_error(
                    a,
                    (*tar).sconv,
                    b"Linkname\x00" as *const u8 as *const libc::c_char,
                );
                if err == ARCHIVE_FATAL {
                    return err;
                }
            }
            /*
             * The following may seem odd, but: Technically, tar
             * does not store the file type for a "hard link"
             * entry, only the fact that it is a hard link.  So, I
             * leave the type zero normally.  But, pax interchange
             * format allows hard links to have data, which
             * implies that the underlying entry is a regular
             * file.
             */
            if archive_entry_size(entry) > 0 as libc::c_int as libc::c_long {
                archive_entry_set_filetype(entry, AE_IFREG as mode_t);
            }
            /*
             * A tricky point: Traditionally, tar readers have
             * ignored the size field when reading hardlink
             * entries, and some writers put non-zero sizes even
             * though the body is empty.  POSIX blessed this
             * convention in the 1988 standard, but broke with
             * this tradition in 2001 by permitting hardlink
             * entries to store valid bodies in pax interchange
             * format, but not in ustar format.  Since there is no
             * hard and fast way to distinguish pax interchange
             * from earlier archives (the 'x' and 'g' entries are
             * optional, after all), we need a heuristic.
             */
            if !(archive_entry_size(entry) == 0 as libc::c_int as libc::c_long) {
                if !((*a).archive.archive_format == ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE) {
                    if (*a).archive.archive_format == ARCHIVE_FORMAT_TAR
                        || (*a).archive.archive_format == ARCHIVE_FORMAT_TAR_GNUTAR
                    {
                        /* Old-style or GNU tar: we must ignore the size. */
                        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
                        (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t
                    } else if archive_read_format_tar_bid(a, 50 as libc::c_int) > 50 as libc::c_int
                    {
                        /*
                         * We don't know if it's pax: If the bid
                         * function sees a valid ustar header
                         * immediately following, then let's ignore
                         * the hardlink size.
                         */
                        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
                        (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t
                    }
                }
            }
            /*
             * TODO: There are still two cases I'd like to handle:
             *   = a ustar non-pax archive with a hardlink entry at
             *     end-of-archive.  (Look for block of nulls following?)
             *   = a pax archive that has not seen any pax headers
             *     and has an entry which is a hardlink entry storing
             *     a body containing an uncompressed tar archive.
             * The first is worth addressing; I don't see any reliable
             * way to deal with the second possibility.
             */
            current_block_64 = 5372832139739605200;
        }
        50 => {
            /* Symlink */
            archive_entry_set_filetype(entry, AE_IFLNK as mode_t);
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
            if _archive_entry_copy_symlink_l(
                entry,
                (*tar).entry_linkpath.s,
                (*tar).entry_linkpath.length,
                (*tar).sconv,
            ) != 0 as libc::c_int
            {
                err = set_conversion_failed_error(
                    a,
                    (*tar).sconv,
                    b"Linkname\x00" as *const u8 as *const libc::c_char,
                );
                if err == ARCHIVE_FATAL {
                    return err;
                }
            }
            current_block_64 = 5372832139739605200;
        }
        51 => {
            /* Character device */
            archive_entry_set_filetype(entry, AE_IFCHR as mode_t);
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
            current_block_64 = 5372832139739605200;
        }
        52 => {
            /* Block device */
            archive_entry_set_filetype(entry, AE_IFBLK as mode_t);
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
            current_block_64 = 5372832139739605200;
        }
        53 => {
            /* Dir */
            archive_entry_set_filetype(entry, AE_IFDIR as mode_t);
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
            current_block_64 = 5372832139739605200;
        }
        54 => {
            /* FIFO device */
            archive_entry_set_filetype(entry, AE_IFIFO as mode_t);
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
            (*tar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
            current_block_64 = 5372832139739605200;
        }
        68 => {
            /* GNU incremental directory type */
            /*
             * No special handling is actually required here.
             * It might be nice someday to preprocess the file list and
             * provide it to the client, though.
             */
            archive_entry_set_filetype(entry, AE_IFDIR as mode_t);
            current_block_64 = 5372832139739605200;
        }
        77 => {
            current_block_64 = 5372832139739605200;
        }
        78 => {
            /* Old GNU "long filename" entry. */
            /* The body of this entry is a script for renaming
             * previously-extracted entries.  Ugh.  It will never
             * be supported by libarchive. */
            archive_entry_set_filetype(entry, AE_IFREG as mode_t);
            current_block_64 = 5372832139739605200;
        }
        83 | 48 => {
            /* GNU sparse files */
            /*
             * Sparse files are really just regular files with
             * sparse information in the extended area.
             */
            /* FALLTHROUGH */
            /*
             * Enable sparse file "read" support only for regular
             * files and explicit GNU sparse files.  However, we
             * don't allow non-standard file types to be sparse.
             */
            (*tar).sparse_allowed = 1 as libc::c_int;
            current_block_64 = 380204216079784120;
        }
        _ => {
            current_block_64 = 380204216079784120;
        }
    }
    match current_block_64 {
        380204216079784120 =>
        /* FALLTHROUGH */
        /* Regular file  and non-standard types */
        /*
         * Per POSIX: non-recognized types should always be
         * treated as regular files.
         */
        {
            archive_entry_set_filetype(entry, AE_IFREG as mode_t);
        }
        _ => {}
    }
    return err;
}
/*
 * Parse out header elements for "old-style" tar archives.
 */
unsafe extern "C" fn header_old_tar(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
) -> libc::c_int {
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    let mut err: libc::c_int = ARCHIVE_OK;
    let mut err2: libc::c_int = 0;
    /* Copy filename over (to ensure null termination). */
    header = h as *const archive_entry_header_ustar;
    if _archive_entry_copy_pathname_l(
        entry,
        (*header).name.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        (*tar).sconv,
    ) != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Pathname\x00" as *const u8 as *const libc::c_char,
        );
        if err == ARCHIVE_FATAL {
            return err;
        }
    }
    /* Grab rest of common fields */
    err2 = header_common(a, tar, entry, h);
    if err > err2 {
        err = err2
    }
    (*tar).entry_padding = 0x1ff as libc::c_int as libc::c_long & -(*tar).entry_bytes_remaining;
    return err;
}
/*
 * Read a Mac AppleDouble-encoded blob of file metadata,
 * if there is one.
 */
unsafe extern "C" fn read_mac_metadata_blob(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut size: int64_t = 0;
    let mut data: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut wp: *const wchar_t = 0 as *const wchar_t;
    let mut wname: *const wchar_t = 0 as *const wchar_t;
    /* UNUSED */
    wp = archive_entry_pathname_w(entry);
    wname = wp;
    if !wp.is_null() {
        /* Find the last path element. */
        while *wp != '\u{0}' as i32 {
            if *wp.offset(0 as libc::c_int as isize) == '/' as i32
                && *wp.offset(1 as libc::c_int as isize) != '\u{0}' as i32
            {
                wname = wp.offset(1 as libc::c_int as isize)
            }
            wp = wp.offset(1)
        }
        /*
         * If last path element starts with "._", then
         * this is a Mac extension.
         */
        if *wname.offset(0 as libc::c_int as isize) != '.' as i32
            || *wname.offset(1 as libc::c_int as isize) != '_' as i32
            || *wname.offset(2 as libc::c_int as isize) == '\u{0}' as i32
        {
            return ARCHIVE_OK;
        }
    } else {
        /* Find the last path element. */
        p = archive_entry_pathname(entry);
        name = p;
        if p.is_null() {
            return -(25 as libc::c_int);
        }
        while *p as libc::c_int != '\u{0}' as i32 {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            {
                name = p.offset(1 as libc::c_int as isize)
            }
            p = p.offset(1)
        }
        /*
         * If last path element starts with "._", then
         * this is a Mac extension.
         */
        if *name.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
            || *name.offset(1 as libc::c_int as isize) as libc::c_int != '_' as i32
            || *name.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            return ARCHIVE_OK;
        }
    }
    /* Read the body as a Mac OS metadata blob. */
    size = archive_entry_size(entry);
    /*
     * TODO: Look beyond the body here to peek at the next header.
     * If it's a regular header (not an extension header)
     * that has the wrong name, just return the current
     * entry as-is, without consuming the body here.
     * That would reduce the risk of us mis-identifying
     * an ordinary file that just happened to have
     * a name starting with "._".
     *
     * Q: Is the above idea really possible?  Even
     * when there are GNU or pax extension entries?
     */
    data = __archive_read_ahead(a, size as size_t, NULL as *mut ssize_t);
    if data == NULL as *const libc::c_void {
        *unconsumed = 0 as libc::c_int as size_t;
        return -(30 as libc::c_int);
    }
    archive_entry_copy_mac_metadata(entry, data, size as size_t);
    *unconsumed = (size + 511 as libc::c_int as libc::c_long
        & !(511 as libc::c_int) as libc::c_long) as size_t;
    tar_flush_unconsumed(a, unconsumed);
    return tar_read_header(a, tar, entry, unconsumed);
}
/*
 * Parse a file header for a pax extended archive entry.
 */
unsafe extern "C" fn header_pax_global(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = read_body_to_string(a, tar, &mut (*tar).pax_global, h, unconsumed);
    if err != ARCHIVE_OK {
        return err;
    }
    err = tar_read_header(a, tar, entry, unconsumed);
    return err;
}
unsafe extern "C" fn header_pax_extensions(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut err2: libc::c_int = 0;
    err = read_body_to_string(a, tar, &mut (*tar).pax_header, h, unconsumed);
    if err != ARCHIVE_OK {
        return err;
    }
    /* Parse the next header. */
    err = tar_read_header(a, tar, entry, unconsumed);
    if err != ARCHIVE_OK && err != ARCHIVE_WARN {
        return err;
    }
    /*
     * TODO: Parse global/default options into 'entry' struct here
     * before handling file-specific options.
     *
     * This design (parse standard header, then overwrite with pax
     * extended attribute data) usually works well, but isn't ideal;
     * it would be better to parse the pax extended attributes first
     * and then skip any fields in the standard header that were
     * defined in the pax header.
     */
    err2 = pax_header(a, tar, entry, &mut (*tar).pax_header);
    err = if err < err2 { err } else { err2 };
    (*tar).entry_padding = 0x1ff as libc::c_int as libc::c_long & -(*tar).entry_bytes_remaining;
    return err;
}
/*
 * Parse a file header for a Posix "ustar" archive entry.  This also
 * handles "pax" or "extended ustar" entries.
 */
unsafe extern "C" fn header_ustar(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
) -> libc::c_int {
    let mut header: *const archive_entry_header_ustar = 0 as *const archive_entry_header_ustar;
    let mut as_0: *mut archive_string = 0 as *mut archive_string;
    let mut err: libc::c_int = ARCHIVE_OK;
    let mut r: libc::c_int = 0;
    header = h as *const archive_entry_header_ustar;
    /* Copy name into an internal buffer to ensure null-termination. */
    as_0 = &mut (*tar).entry_pathname;
    if (*header).prefix[0 as libc::c_int as usize] != 0 {
        (*as_0).length = 0 as libc::c_int as size_t;
        archive_strncat(
            as_0,
            (*header).prefix.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 155]>() as libc::c_ulong,
        );
        if *(*as_0).s.offset(
            (*as_0)
                .length
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int
            != '/' as i32
        {
            archive_strappend_char(as_0, '/' as i32 as libc::c_char);
        }
        archive_strncat(
            as_0,
            (*header).name.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        );
    } else {
        (*as_0).length = 0 as libc::c_int as size_t;
        archive_strncat(
            as_0,
            (*header).name.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        );
    }
    if _archive_entry_copy_pathname_l(entry, (*as_0).s, (*as_0).length, (*tar).sconv)
        != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Pathname\x00" as *const u8 as *const libc::c_char,
        );
        if err == ARCHIVE_FATAL {
            return err;
        }
    }
    /* Handle rest of common fields. */
    r = header_common(a, tar, entry, h);
    if r == ARCHIVE_FATAL {
        return r;
    }
    if r < err {
        err = r
    }
    /* Handle POSIX ustar fields. */
    if _archive_entry_copy_uname_l(
        entry,
        (*header).uname.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        (*tar).sconv,
    ) != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Uname\x00" as *const u8 as *const libc::c_char,
        );
        if err == ARCHIVE_FATAL {
            return err;
        }
    }
    if _archive_entry_copy_gname_l(
        entry,
        (*header).gname.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        (*tar).sconv,
    ) != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Gname\x00" as *const u8 as *const libc::c_char,
        );
        if err == ARCHIVE_FATAL {
            return err;
        }
    }
    /* Parse out device numbers only for char and block specials. */
    if (*header).typeflag[0 as libc::c_int as usize] as libc::c_int == '3' as i32
        || (*header).typeflag[0 as libc::c_int as usize] as libc::c_int == '4' as i32
    {
        archive_entry_set_rdevmajor(
            entry,
            tar_atol(
                (*header).rdevmajor.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) as dev_t,
        );
        archive_entry_set_rdevminor(
            entry,
            tar_atol(
                (*header).rdevminor.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) as dev_t,
        );
    }
    (*tar).entry_padding = 0x1ff as libc::c_int as libc::c_long & -(*tar).entry_bytes_remaining;
    return err;
}
/*
 * Parse the pax extended attributes record.
 *
 * Returns non-zero if there's an error in the data.
 */
unsafe extern "C" fn pax_header(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut in_as: *mut archive_string,
) -> libc::c_int {
    let mut attr_length: size_t = 0;
    let mut l: size_t = 0;
    let mut line_length: size_t = 0;
    let mut value_length: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut as_0: *mut archive_string = 0 as *mut archive_string;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut err: libc::c_int = 0;
    let mut err2: libc::c_int = 0;
    let mut attr: *mut libc::c_char = (*in_as).s;
    attr_length = (*in_as).length;
    (*tar).pax_hdrcharset_binary = 0 as libc::c_int;
    (*tar).entry_gname.length = 0 as libc::c_int as size_t;
    (*tar).entry_linkpath.length = 0 as libc::c_int as size_t;
    (*tar).entry_pathname.length = 0 as libc::c_int as size_t;
    (*tar).entry_pathname_override.length = 0 as libc::c_int as size_t;
    (*tar).entry_uname.length = 0 as libc::c_int as size_t;
    err = ARCHIVE_OK;
    while attr_length > 0 as libc::c_int as libc::c_ulong {
        /* Parse decimal length field at start of line. */
        line_length = 0 as libc::c_int as size_t; /* Record start of line. */
        l = attr_length;
        p = attr;
        while l > 0 as libc::c_int as libc::c_ulong {
            if *p as libc::c_int == ' ' as i32 {
                p = p.offset(1);
                l = l.wrapping_sub(1);
                break;
            } else {
                if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32 {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Ignoring malformed pax extended attributes\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(20 as libc::c_int);
                }
                line_length = (line_length as libc::c_ulong)
                    .wrapping_mul(10 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                line_length = (line_length as libc::c_ulong)
                    .wrapping_add((*p as libc::c_int - '0' as i32) as libc::c_ulong)
                    as size_t as size_t;
                if line_length > 999999 as libc::c_int as libc::c_ulong {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Rejecting pax extended attribute > 1MB\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(20 as libc::c_int);
                }
                p = p.offset(1);
                l = l.wrapping_sub(1)
            }
        }
        /*
         * Parsed length must be no bigger than available data,
         * at least 1, and the last character of the line must
         * be '\n'.
         */
        if line_length > attr_length
            || line_length < 1 as libc::c_int as libc::c_ulong
            || *attr.offset(line_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != '\n' as i32
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Ignoring malformed pax extended attribute\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(20 as libc::c_int);
        }
        /* Null-terminate the line. */
        *attr.offset(line_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
            '\u{0}' as i32 as libc::c_char;
        /* Find end of key and null terminate it. */
        key = p;
        if *key.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32 {
            return -(1 as libc::c_int);
        }
        while *p as libc::c_int != 0 && *p as libc::c_int != '=' as i32 {
            p = p.offset(1)
        }
        if *p as libc::c_int == '\u{0}' as i32 {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid pax extended attributes\x00" as *const u8 as *const libc::c_char,
            );
            return -(20 as libc::c_int);
        }
        *p = '\u{0}' as i32 as libc::c_char;
        value = p.offset(1 as libc::c_int as isize);
        /* Some values may be binary data */
        value_length = attr
            .offset(line_length as isize)
            .offset(-(1 as libc::c_int as isize))
            .offset_from(value) as libc::c_long as size_t;
        /* Identify this attribute and set it in the entry. */
        err2 = pax_attribute(a, tar, entry, key, value, value_length);
        if err2 == ARCHIVE_FATAL {
            return err2;
        }
        err = if err < err2 { err } else { err2 };
        /* Skip to next line */
        attr = attr.offset(line_length as isize);
        attr_length = (attr_length as libc::c_ulong).wrapping_sub(line_length) as size_t as size_t
    }
    /*
     * PAX format uses UTF-8 as default charset for its metadata
     * unless hdrcharset=BINARY is present in its header.
     * We apply the charset specified by the hdrcharset option only
     * when the hdrcharset attribute(in PAX header) is BINARY because
     * we respect the charset described in PAX header and BINARY also
     * means that metadata(filename,uname and gname) character-set
     * is unknown.
     */
    if (*tar).pax_hdrcharset_binary != 0 {
        sconv = (*tar).opt_sconv
    } else {
        sconv = archive_string_conversion_from_charset(
            &mut (*a).archive,
            b"UTF-8\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if sconv.is_null() {
            return -(30 as libc::c_int);
        }
        if (*tar).compat_2x != 0 {
            archive_string_conversion_set_opt(sconv, SCONV_SET_OPT_UTF8_LIBARCHIVE2X);
        }
    }
    if (*tar).entry_gname.length > 0 as libc::c_int as libc::c_ulong {
        if _archive_entry_copy_gname_l(
            entry,
            (*tar).entry_gname.s,
            (*tar).entry_gname.length,
            sconv,
        ) != 0 as libc::c_int
        {
            err = set_conversion_failed_error(
                a,
                sconv,
                b"Gname\x00" as *const u8 as *const libc::c_char,
            );
            if err == ARCHIVE_FATAL {
                return err;
            }
            /* Use a converted an original name. */
            archive_entry_copy_gname(entry, (*tar).entry_gname.s);
        }
    }
    if (*tar).entry_linkpath.length > 0 as libc::c_int as libc::c_ulong {
        if _archive_entry_copy_link_l(
            entry,
            (*tar).entry_linkpath.s,
            (*tar).entry_linkpath.length,
            sconv,
        ) != 0 as libc::c_int
        {
            err = set_conversion_failed_error(
                a,
                sconv,
                b"Linkname\x00" as *const u8 as *const libc::c_char,
            );
            if err == ARCHIVE_FATAL {
                return err;
            }
            /* Use a converted an original name. */
            archive_entry_copy_link(entry, (*tar).entry_linkpath.s);
        }
    }
    /*
     * Some extensions (such as the GNU sparse file extensions)
     * deliberately store a synthetic name under the regular 'path'
     * attribute and the real file name under a different attribute.
     * Since we're supposed to not care about the order, we
     * have no choice but to store all of the various filenames
     * we find and figure it all out afterwards.  This is the
     * figuring out part.
     */
    as_0 = NULL as *mut archive_string;
    if (*tar).entry_pathname_override.length > 0 as libc::c_int as libc::c_ulong {
        as_0 = &mut (*tar).entry_pathname_override
    } else if (*tar).entry_pathname.length > 0 as libc::c_int as libc::c_ulong {
        as_0 = &mut (*tar).entry_pathname
    }
    if !as_0.is_null() {
        if _archive_entry_copy_pathname_l(entry, (*as_0).s, (*as_0).length, sconv)
            != 0 as libc::c_int
        {
            err = set_conversion_failed_error(
                a,
                sconv,
                b"Pathname\x00" as *const u8 as *const libc::c_char,
            );
            if err == ARCHIVE_FATAL {
                return err;
            }
            /* Use a converted an original name. */
            archive_entry_copy_pathname(entry, (*as_0).s);
        }
    }
    if (*tar).entry_uname.length > 0 as libc::c_int as libc::c_ulong {
        if _archive_entry_copy_uname_l(
            entry,
            (*tar).entry_uname.s,
            (*tar).entry_uname.length,
            sconv,
        ) != 0 as libc::c_int
        {
            err = set_conversion_failed_error(
                a,
                sconv,
                b"Uname\x00" as *const u8 as *const libc::c_char,
            );
            if err == ARCHIVE_FATAL {
                return err;
            }
            /* Use a converted an original name. */
            archive_entry_copy_uname(entry, (*tar).entry_uname.s);
        }
    }
    return err;
}
unsafe extern "C" fn pax_attribute_xattr(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut name_decoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value_decoded: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value_len: size_t = 0;
    if strlen(name) < 18 as libc::c_int as libc::c_ulong
        || memcmp(
            name as *const libc::c_void,
            b"LIBARCHIVE.xattr.\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            17 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        return 3 as libc::c_int;
    }
    name = name.offset(17 as libc::c_int as isize);
    /* URL-decode name */
    name_decoded = url_decode(name);
    if name_decoded.is_null() {
        return 2 as libc::c_int;
    }
    /* Base-64 decode value */
    value_decoded = base64_decode(value, strlen(value), &mut value_len) as *mut libc::c_void;
    if value_decoded.is_null() {
        free(name_decoded as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    archive_entry_xattr_add_entry(entry, name_decoded, value_decoded, value_len);
    free(name_decoded as *mut libc::c_void);
    free(value_decoded);
    return 0 as libc::c_int;
}
unsafe extern "C" fn pax_attribute_schily_xattr(
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut value_length: size_t,
) -> libc::c_int {
    if strlen(name) < 14 as libc::c_int as libc::c_ulong
        || memcmp(
            name as *const libc::c_void,
            b"SCHILY.xattr.\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            13 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    name = name.offset(13 as libc::c_int as isize);
    archive_entry_xattr_add_entry(entry, name, value as *const libc::c_void, value_length);
    return 0 as libc::c_int;
}
unsafe extern "C" fn pax_attribute_rht_security_selinux(
    mut entry: *mut archive_entry,
    mut value: *const libc::c_char,
    mut value_length: size_t,
) -> libc::c_int {
    archive_entry_xattr_add_entry(
        entry,
        b"security.selinux\x00" as *const u8 as *const libc::c_char,
        value as *const libc::c_void,
        value_length,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn pax_attribute_acl(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut value: *const libc::c_char,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    match type_0 {
        ARCHIVE_ENTRY_ACL_TYPE_ACCESS => {
            errstr = b"SCHILY.acl.access\x00" as *const u8 as *const libc::c_char
        }
        ARCHIVE_ENTRY_ACL_TYPE_DEFAULT => {
            errstr = b"SCHILY.acl.default\x00" as *const u8 as *const libc::c_char
        }
        15360 => errstr = b"SCHILY.acl.ace\x00" as *const u8 as *const libc::c_char,
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unknown ACL type: %d\x00" as *const u8 as *const libc::c_char,
                type_0,
            );
            return -(30 as libc::c_int);
        }
    }
    if (*tar).sconv_acl.is_null() {
        (*tar).sconv_acl = archive_string_conversion_from_charset(
            &mut (*a).archive,
            b"UTF-8\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if (*tar).sconv_acl.is_null() {
            return -(30 as libc::c_int);
        }
    }
    r = archive_acl_from_text_l(archive_entry_acl(entry), value, type_0, (*tar).sconv_acl);
    if r != ARCHIVE_OK {
        if r == ARCHIVE_FATAL {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"%s %s\x00" as *const u8 as *const libc::c_char,
                b"Can\'t allocate memory for \x00" as *const u8 as *const libc::c_char,
                errstr,
            );
            return r;
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"%s %s\x00" as *const u8 as *const libc::c_char,
            b"Parse error: \x00" as *const u8 as *const libc::c_char,
            errstr,
        );
    }
    return r;
}
/*
 * Parse a single key=value attribute.  key/value pointers are
 * assumed to point into reasonably long-lived storage.
 *
 * Note that POSIX reserves all-lowercase keywords.  Vendor-specific
 * extensions should always have keywords of the form "VENDOR.attribute"
 * In particular, it's quite feasible to support many different
 * vendor extensions here.  I'm using "LIBARCHIVE" for extensions
 * unique to this library.
 *
 * Investigate other vendor-specific extensions and see if
 * any of them look useful.
 */
unsafe extern "C" fn pax_attribute(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
    mut value_length: size_t,
) -> libc::c_int {
    let mut s: int64_t = 0; /* Disable compiler warning; do not pass
                             * NULL pointer to strlen().  */
    let mut n: libc::c_long = 0;
    let mut err: libc::c_int = ARCHIVE_OK;
    let mut r: libc::c_int = 0;
    if value.is_null() {
        value = b"\x00" as *const u8 as *const libc::c_char
    }
    match *key.offset(0 as libc::c_int as isize) as libc::c_int {
        71 => {
            /* Reject GNU.sparse.* headers on non-regular files. */
            if strncmp(
                key,
                b"GNU.sparse\x00" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
                && (*tar).sparse_allowed == 0
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Non-regular file cannot be sparse\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            /* GNU "0.0" sparse pax format. */
            if strcmp(
                key,
                b"GNU.sparse.numblocks\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).sparse_offset = -(1 as libc::c_int) as int64_t;
                (*tar).sparse_numbytes = -(1 as libc::c_int) as int64_t;
                (*tar).sparse_gnu_major = 0 as libc::c_int;
                (*tar).sparse_gnu_minor = 0 as libc::c_int
            }
            if strcmp(
                key,
                b"GNU.sparse.offset\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).sparse_offset = tar_atol10(value, strlen(value));
                if (*tar).sparse_numbytes != -(1 as libc::c_int) as libc::c_long {
                    if gnu_add_sparse_entry(a, tar, (*tar).sparse_offset, (*tar).sparse_numbytes)
                        != ARCHIVE_OK
                    {
                        return -(30 as libc::c_int);
                    }
                    (*tar).sparse_offset = -(1 as libc::c_int) as int64_t;
                    (*tar).sparse_numbytes = -(1 as libc::c_int) as int64_t
                }
            }
            if strcmp(
                key,
                b"GNU.sparse.numbytes\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).sparse_numbytes = tar_atol10(value, strlen(value));
                if (*tar).sparse_numbytes != -(1 as libc::c_int) as libc::c_long {
                    if gnu_add_sparse_entry(a, tar, (*tar).sparse_offset, (*tar).sparse_numbytes)
                        != ARCHIVE_OK
                    {
                        return -(30 as libc::c_int);
                    }
                    (*tar).sparse_offset = -(1 as libc::c_int) as int64_t;
                    (*tar).sparse_numbytes = -(1 as libc::c_int) as int64_t
                }
            }
            if strcmp(
                key,
                b"GNU.sparse.size\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).realsize = tar_atol10(value, strlen(value));
                archive_entry_set_size(entry, (*tar).realsize);
                (*tar).realsize_override = 1 as libc::c_int
            }
            /* GNU "0.1" sparse pax format. */
            if strcmp(
                key,
                b"GNU.sparse.map\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).sparse_gnu_major = 0 as libc::c_int;
                (*tar).sparse_gnu_minor = 1 as libc::c_int;
                if gnu_sparse_01_parse(a, tar, value) != ARCHIVE_OK {
                    return -(20 as libc::c_int);
                }
            }
            /* GNU "1.0" sparse pax format */
            if strcmp(
                key,
                b"GNU.sparse.major\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).sparse_gnu_major = tar_atol10(value, strlen(value)) as libc::c_int;
                (*tar).sparse_gnu_pending = 1 as libc::c_int as libc::c_char
            }
            if strcmp(
                key,
                b"GNU.sparse.minor\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).sparse_gnu_minor = tar_atol10(value, strlen(value)) as libc::c_int;
                (*tar).sparse_gnu_pending = 1 as libc::c_int as libc::c_char
            }
            if strcmp(
                key,
                b"GNU.sparse.name\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                /*
                 * The real filename; when storing sparse
                 * files, GNU tar puts a synthesized name into
                 * the regular 'path' attribute in an attempt
                 * to limit confusion. ;-)
                 */
                (*tar).entry_pathname_override.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*tar).entry_pathname_override,
                    value as *const libc::c_void,
                    (if value.is_null() {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(value)
                    }),
                );
            }
            if strcmp(
                key,
                b"GNU.sparse.realsize\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).realsize = tar_atol10(value, strlen(value));
                archive_entry_set_size(entry, (*tar).realsize);
                (*tar).realsize_override = 1 as libc::c_int
            }
        }
        76 => {
            /* Our extensions */
            /* TODO: Handle arbitrary extended attributes... */
            /*
                    if (strcmp(key, "LIBARCHIVE.xxxxxxx") == 0)
                        archive_entry_set_xxxxxx(entry, value);
            */
            if strcmp(
                key,
                b"LIBARCHIVE.creationtime\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                pax_time(value, &mut s, &mut n);
                archive_entry_set_birthtime(entry, s, n);
            }
            if strcmp(
                key,
                b"LIBARCHIVE.symlinktype\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                if strcmp(value, b"file\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    archive_entry_set_symlink_type(entry, AE_SYMLINK_TYPE_FILE);
                } else if strcmp(value, b"dir\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    archive_entry_set_symlink_type(entry, AE_SYMLINK_TYPE_DIRECTORY);
                }
            }
            if memcmp(
                key as *const libc::c_void,
                b"LIBARCHIVE.xattr.\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                17 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                pax_attribute_xattr(entry, key, value);
            }
        }
        82 => {
            /* GNU tar uses RHT.security header to store SELinux xattrs
             * SCHILY.xattr.security.selinux == RHT.security.selinux */
            if strcmp(
                key,
                b"RHT.security.selinux\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                pax_attribute_rht_security_selinux(entry, value, value_length);
            }
        }
        83 => {
            /* We support some keys used by the "star" archiver */
            if strcmp(
                key,
                b"SCHILY.acl.access\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                r = pax_attribute_acl(a, tar, entry, value, ARCHIVE_ENTRY_ACL_TYPE_ACCESS);
                if r == ARCHIVE_FATAL {
                    return r;
                }
            } else if strcmp(
                key,
                b"SCHILY.acl.default\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                r = pax_attribute_acl(a, tar, entry, value, ARCHIVE_ENTRY_ACL_TYPE_DEFAULT);
                if r == ARCHIVE_FATAL {
                    return r;
                }
            } else if strcmp(
                key,
                b"SCHILY.acl.ace\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                r = pax_attribute_acl(a, tar, entry, value, ARCHIVE_ENTRY_ACL_TYPE_NFS4);
                if r == ARCHIVE_FATAL {
                    return r;
                }
            } else if strcmp(
                key,
                b"SCHILY.devmajor\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                archive_entry_set_rdevmajor(entry, tar_atol10(value, strlen(value)) as dev_t);
            } else if strcmp(
                key,
                b"SCHILY.devminor\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                archive_entry_set_rdevminor(entry, tar_atol10(value, strlen(value)) as dev_t);
            } else if strcmp(
                key,
                b"SCHILY.fflags\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                archive_entry_copy_fflags_text(entry, value);
            } else if strcmp(key, b"SCHILY.dev\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                archive_entry_set_dev(entry, tar_atol10(value, strlen(value)) as dev_t);
            } else if strcmp(key, b"SCHILY.ino\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                archive_entry_set_ino(entry, tar_atol10(value, strlen(value)));
            } else if strcmp(key, b"SCHILY.nlink\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                archive_entry_set_nlink(entry, tar_atol10(value, strlen(value)) as libc::c_uint);
            } else if strcmp(
                key,
                b"SCHILY.realsize\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*tar).realsize = tar_atol10(value, strlen(value));
                (*tar).realsize_override = 1 as libc::c_int;
                archive_entry_set_size(entry, (*tar).realsize);
            } else if strncmp(
                key,
                b"SCHILY.xattr.\x00" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                pax_attribute_schily_xattr(entry, key, value, value_length);
            } else if strcmp(
                key,
                b"SUN.holesdata\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                /* A Solaris extension for sparse. */
                r = solaris_sparse_parse(a, tar, entry, value);
                if r < err {
                    if r == ARCHIVE_FATAL {
                        return r;
                    }
                    err = r;
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Parse error: SUN.holesdata\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        97 => {
            if strcmp(key, b"atime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                pax_time(value, &mut s, &mut n);
                archive_entry_set_atime(entry, s, n);
            }
        }
        99 => {
            if strcmp(key, b"ctime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                pax_time(value, &mut s, &mut n);
                archive_entry_set_ctime(entry, s, n);
            } else if !(strcmp(key, b"charset\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
            {
                (strcmp(key, b"comment\x00" as *const u8 as *const libc::c_char))
                    == 0 as libc::c_int;
            }
        }
        103 => {
            if strcmp(key, b"gid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                archive_entry_set_gid(entry, tar_atol10(value, strlen(value)));
            } else if strcmp(key, b"gname\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*tar).entry_gname.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*tar).entry_gname,
                    value as *const libc::c_void,
                    (if value.is_null() {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(value)
                    }),
                );
            }
        }
        104 => {
            if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if strcmp(value, b"BINARY\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    /* Binary  mode. */
                    (*tar).pax_hdrcharset_binary = 1 as libc::c_int
                } else if strcmp(
                    value,
                    b"ISO-IR 10646 2000 UTF-8\x00" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*tar).pax_hdrcharset_binary = 0 as libc::c_int
                }
            }
        }
        108 => {
            /* pax interchange doesn't distinguish hardlink vs. symlink. */
            if strcmp(key, b"linkpath\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*tar).entry_linkpath.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*tar).entry_linkpath,
                    value as *const libc::c_void,
                    (if value.is_null() {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(value)
                    }),
                );
            }
        }
        109 => {
            if strcmp(key, b"mtime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                pax_time(value, &mut s, &mut n);
                archive_entry_set_mtime(entry, s, n);
            }
        }
        112 => {
            if strcmp(key, b"path\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*tar).entry_pathname.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*tar).entry_pathname,
                    value as *const libc::c_void,
                    (if value.is_null() {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(value)
                    }),
                );
            }
        }
        115 => {
            /* POSIX has reserved 'security.*' */
            /* Someday: if (strcmp(key, "security.acl") == 0) { ... } */
            if strcmp(key, b"size\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                /* "size" is the size of the data in the entry. */
                (*tar).entry_bytes_remaining = tar_atol10(value, strlen(value));
                /*
                 * The "size" pax header keyword always overrides the
                 * "size" field in the tar header.
                 * GNU.sparse.realsize, GNU.sparse.size and
                 * SCHILY.realsize override this value.
                 */
                if (*tar).realsize_override == 0 {
                    archive_entry_set_size(entry, (*tar).entry_bytes_remaining);
                    (*tar).realsize = (*tar).entry_bytes_remaining
                }
            }
        }
        117 => {
            if strcmp(key, b"uid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                archive_entry_set_uid(entry, tar_atol10(value, strlen(value)));
            } else if strcmp(key, b"uname\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*tar).entry_uname.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*tar).entry_uname,
                    value as *const libc::c_void,
                    (if value.is_null() {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(value)
                    }),
                );
            }
        }
        114 | _ => {}
    }
    return err;
}
/*
 * parse a decimal time value, which may include a fractional portion
 */
unsafe extern "C" fn pax_time(
    mut p: *const libc::c_char,
    mut ps: *mut int64_t,
    mut pn: *mut libc::c_long,
) {
    let mut digit: libc::c_char = 0;
    let mut s: int64_t = 0;
    let mut l: libc::c_ulong = 0;
    let mut sign: libc::c_int = 0;
    let mut limit: int64_t = 0;
    let mut last_digit_limit: int64_t = 0;
    limit = INT64_MAX / 10 as libc::c_int as libc::c_long;
    last_digit_limit = INT64_MAX % 10 as libc::c_int as libc::c_long;
    s = 0 as libc::c_int as int64_t;
    sign = 1 as libc::c_int;
    if *p as libc::c_int == '-' as i32 {
        sign = -(1 as libc::c_int);
        p = p.offset(1)
    }
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        digit = (*p as libc::c_int - '0' as i32) as libc::c_char;
        if s > limit || s == limit && digit as libc::c_long > last_digit_limit {
            s = INT64_MAX;
            break;
        } else {
            s = s * 10 as libc::c_int as libc::c_long + digit as libc::c_long;
            p = p.offset(1)
        }
    }
    *ps = s * sign as libc::c_long;
    /* Calculate nanoseconds. */
    *pn = 0 as libc::c_int as libc::c_long;
    if *p as libc::c_int != '.' as i32 {
        return;
    }
    l = 100000000 as libc::c_ulong;
    loop {
        p = p.offset(1);
        if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32) {
            break;
        }
        *pn = (*pn as libc::c_ulong)
            .wrapping_add(((*p as libc::c_int - '0' as i32) as libc::c_ulong).wrapping_mul(l))
            as libc::c_long as libc::c_long;
        l = l.wrapping_div(10 as libc::c_int as libc::c_ulong);
        if !(l != 0) {
            break;
        }
    }
}
/*
 * Parse GNU tar header
 */
unsafe extern "C" fn header_gnutar(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_void,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut header: *const archive_entry_header_gnutar = 0 as *const archive_entry_header_gnutar;
    let mut t: int64_t = 0;
    let mut err: libc::c_int = ARCHIVE_OK;
    /*
     * GNU header is like POSIX ustar, except 'prefix' is
     * replaced with some other fields. This also means the
     * filename is stored as in old-style archives.
     */
    /* Grab fields common to all tar variants. */
    err = header_common(a, tar, entry, h);
    if err == ARCHIVE_FATAL {
        return err;
    }
    /* Copy filename over (to ensure null termination). */
    header = h as *const archive_entry_header_gnutar;
    if _archive_entry_copy_pathname_l(
        entry,
        (*header).name.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        (*tar).sconv,
    ) != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Pathname\x00" as *const u8 as *const libc::c_char,
        );
        if err == ARCHIVE_FATAL {
            return err;
        }
    }
    /* Fields common to ustar and GNU */
    /* XXX Can the following be factored out since it's common
     * to ustar and gnu tar?  Is it okay to move it down into
     * header_common, perhaps?  */
    if _archive_entry_copy_uname_l(
        entry,
        (*header).uname.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        (*tar).sconv,
    ) != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Uname\x00" as *const u8 as *const libc::c_char,
        );
        if err == ARCHIVE_FATAL {
            return err;
        }
    }
    if _archive_entry_copy_gname_l(
        entry,
        (*header).gname.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        (*tar).sconv,
    ) != 0 as libc::c_int
    {
        err = set_conversion_failed_error(
            a,
            (*tar).sconv,
            b"Gname\x00" as *const u8 as *const libc::c_char,
        );
        if err == ARCHIVE_FATAL {
            return err;
        }
    }
    /* Parse out device numbers only for char and block specials */
    if (*header).typeflag[0 as libc::c_int as usize] as libc::c_int == '3' as i32
        || (*header).typeflag[0 as libc::c_int as usize] as libc::c_int == '4' as i32
    {
        archive_entry_set_rdevmajor(
            entry,
            tar_atol(
                (*header).rdevmajor.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) as dev_t,
        );
        archive_entry_set_rdevminor(
            entry,
            tar_atol(
                (*header).rdevminor.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            ) as dev_t,
        );
    } else {
        archive_entry_set_rdev(entry, 0 as libc::c_int as dev_t);
    }
    (*tar).entry_padding = 0x1ff as libc::c_int as libc::c_long & -(*tar).entry_bytes_remaining;
    /* Grab GNU-specific fields. */
    t = tar_atol(
        (*header).atime.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    if t > 0 as libc::c_int as libc::c_long {
        archive_entry_set_atime(entry, t, 0 as libc::c_int as libc::c_long);
    }
    t = tar_atol(
        (*header).ctime.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    if t > 0 as libc::c_int as libc::c_long {
        archive_entry_set_ctime(entry, t, 0 as libc::c_int as libc::c_long);
    }
    if (*header).realsize[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
        (*tar).realsize = tar_atol(
            (*header).realsize.as_ptr(),
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
        );
        archive_entry_set_size(entry, (*tar).realsize);
        (*tar).realsize_override = 1 as libc::c_int
    }
    if (*header).sparse[0 as libc::c_int as usize].offset[0 as libc::c_int as usize] as libc::c_int
        != 0 as libc::c_int
    {
        if gnu_sparse_old_read(a, tar, header, unconsumed) != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    } else {
        ((*header).isextended[0 as libc::c_int as usize] as libc::c_int) != 0 as libc::c_int;
    }
    return err;
}
unsafe extern "C" fn gnu_add_sparse_entry(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut offset: int64_t,
    mut remaining: int64_t,
) -> libc::c_int {
    let mut p: *mut sparse_block = 0 as *mut sparse_block;
    p = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<sparse_block>() as libc::c_ulong,
    ) as *mut sparse_block;
    if p.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if !(*tar).sparse_last.is_null() {
        (*(*tar).sparse_last).next = p
    } else {
        (*tar).sparse_list = p
    }
    (*tar).sparse_last = p;
    if remaining < 0 as libc::c_int as libc::c_long
        || offset < 0 as libc::c_int as libc::c_long
        || offset > INT64_MAX - remaining
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Malformed sparse map data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*p).offset = offset;
    (*p).remaining = remaining;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gnu_clear_sparse_list(mut tar: *mut tar) {
    let mut p: *mut sparse_block = 0 as *mut sparse_block;
    while !(*tar).sparse_list.is_null() {
        p = (*tar).sparse_list;
        (*tar).sparse_list = (*p).next;
        free(p as *mut libc::c_void);
    }
    (*tar).sparse_last = NULL as *mut sparse_block;
}
/*
 * GNU tar old-format sparse data.
 *
 * GNU old-format sparse data is stored in a fixed-field
 * format.  Offset/size values are 11-byte octal fields (same
 * format as 'size' field in ustart header).  These are
 * stored in the header, allocating subsequent header blocks
 * as needed.  Extending the header in this way is a pretty
 * severe POSIX violation; this design has earned GNU tar a
 * lot of criticism.
 */
unsafe extern "C" fn gnu_sparse_old_read(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut header: *const archive_entry_header_gnutar,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut bytes_read: ssize_t = 0;
    let mut data: *const libc::c_void = 0 as *const libc::c_void;
    let mut ext: *const extended = 0 as *const extended;
    if gnu_sparse_old_parse(a, tar, (*header).sparse.as_ptr(), 4 as libc::c_int) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    if (*header).isextended[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    loop {
        tar_flush_unconsumed(a, unconsumed);
        data = __archive_read_ahead(a, 512 as libc::c_int as size_t, &mut bytes_read);
        if bytes_read < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int);
        }
        if bytes_read < 512 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated tar archive detected while reading sparse file data\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        *unconsumed = 512 as libc::c_int as size_t;
        ext = data as *const extended;
        if gnu_sparse_old_parse(a, tar, (*ext).sparse.as_ptr(), 21 as libc::c_int) != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        if !((*ext).isextended[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int) {
            break;
        }
    }
    if !(*tar).sparse_list.is_null() {
        (*tar).entry_offset = (*(*tar).sparse_list).offset
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gnu_sparse_old_parse(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut sparse: *const gnu_sparse,
    mut length: libc::c_int,
) -> libc::c_int {
    while length > 0 as libc::c_int
        && (*sparse).offset[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int
    {
        if gnu_add_sparse_entry(
            a,
            tar,
            tar_atol(
                (*sparse).offset.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            ),
            tar_atol(
                (*sparse).numbytes.as_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
            ),
        ) != ARCHIVE_OK
        {
            return -(30 as libc::c_int);
        }
        sparse = sparse.offset(1);
        length -= 1
    }
    return 0 as libc::c_int;
}
/*
 * GNU tar sparse format 0.0
 *
 * Beginning with GNU tar 1.15, sparse files are stored using
 * information in the pax extended header.  The GNU tar maintainers
 * have gone through a number of variations in the process of working
 * out this scheme; fortunately, they're all numbered.
 *
 * Sparse format 0.0 uses attribute GNU.sparse.numblocks to store the
 * number of blocks, and GNU.sparse.offset/GNU.sparse.numbytes to
 * store offset/size for each block.  The repeated instances of these
 * latter fields violate the pax specification (which frowns on
 * duplicate keys), so this format was quickly replaced.
 */
/*
 * GNU tar sparse format 0.1
 *
 * This version replaced the offset/numbytes attributes with
 * a single "map" attribute that stored a list of integers.  This
 * format had two problems: First, the "map" attribute could be very
 * long, which caused problems for some implementations.  More
 * importantly, the sparse data was lost when extracted by archivers
 * that didn't recognize this extension.
 */
unsafe extern "C" fn gnu_sparse_01_parse(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut p: *const libc::c_char,
) -> libc::c_int {
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut offset: int64_t = -(1 as libc::c_int) as int64_t;
    let mut size: int64_t = -(1 as libc::c_int) as int64_t;
    loop {
        e = p;
        while *e as libc::c_int != '\u{0}' as i32 && *e as libc::c_int != ',' as i32 {
            if (*e as libc::c_int) < '0' as i32 || *e as libc::c_int > '9' as i32 {
                return -(20 as libc::c_int);
            }
            e = e.offset(1)
        }
        if offset < 0 as libc::c_int as libc::c_long {
            offset = tar_atol10(p, e.offset_from(p) as libc::c_long as size_t);
            if offset < 0 as libc::c_int as libc::c_long {
                return -(20 as libc::c_int);
            }
        } else {
            size = tar_atol10(p, e.offset_from(p) as libc::c_long as size_t);
            if size < 0 as libc::c_int as libc::c_long {
                return -(20 as libc::c_int);
            }
            if gnu_add_sparse_entry(a, tar, offset, size) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
            offset = -(1 as libc::c_int) as int64_t
        }
        if *e as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        p = e.offset(1 as libc::c_int as isize)
    }
}
/*
 * GNU tar sparse format 1.0
 *
 * The idea: The offset/size data is stored as a series of base-10
 * ASCII numbers prepended to the file data, so that dearchivers that
 * don't support this format will extract the block map along with the
 * data and a separate post-process can restore the sparseness.
 *
 * Unfortunately, GNU tar 1.16 had a bug that added unnecessary
 * padding to the body of the file when using this format.  GNU tar
 * 1.17 corrected this bug without bumping the version number, so
 * it's not possible to support both variants.  This code supports
 * the later variant at the expense of not supporting the former.
 *
 * This variant also replaced GNU.sparse.size with GNU.sparse.realsize
 * and introduced the GNU.sparse.major/GNU.sparse.minor attributes.
 */
/*
 * Read the next line from the input, and parse it as a decimal
 * integer followed by '\n'.  Returns positive integer value or
 * negative on error.
 */
unsafe extern "C" fn gnu_sparse_10_atol(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut remaining: *mut int64_t,
    mut unconsumed: *mut size_t,
) -> int64_t {
    let mut l: int64_t = 0;
    let mut limit: int64_t = 0;
    let mut last_digit_limit: int64_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut bytes_read: ssize_t = 0;
    let mut base: libc::c_int = 0;
    let mut digit: libc::c_int = 0;
    base = 10 as libc::c_int;
    limit = INT64_MAX / base as libc::c_long;
    last_digit_limit = INT64_MAX % base as libc::c_long;
    loop
    /*
     * Skip any lines starting with '#'; GNU tar specs
     * don't require this, but they should.
     */
    {
        bytes_read = readline(
            a,
            tar,
            &mut p,
            if *remaining < 100 as libc::c_int as libc::c_long {
                *remaining
            } else {
                100 as libc::c_int as libc::c_long
            },
            unconsumed,
        ); /* Truncate on overflow. */
        if bytes_read <= 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int) as int64_t;
        }
        *remaining -= bytes_read;
        if !(*p.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32) {
            break;
        }
    }
    l = 0 as libc::c_int as int64_t;
    while bytes_read > 0 as libc::c_int as libc::c_long {
        if *p as libc::c_int == '\n' as i32 {
            return l;
        }
        if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int >= '0' as i32 + base {
            return -(20 as libc::c_int) as int64_t;
        }
        digit = *p as libc::c_int - '0' as i32;
        if l > limit || l == limit && digit as libc::c_long > last_digit_limit {
            l = INT64_MAX
        } else {
            l = l * base as libc::c_long + digit as libc::c_long
        }
        p = p.offset(1);
        bytes_read -= 1
    }
    /* TODO: Error message. */
    return -(20 as libc::c_int) as int64_t;
}
/*
 * Returns length (in bytes) of the sparse data description
 * that was read.
 */
unsafe extern "C" fn gnu_sparse_10_read(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut unconsumed: *mut size_t,
) -> ssize_t {
    let mut bytes_read: ssize_t = 0;
    let mut entries: libc::c_int = 0;
    let mut offset: int64_t = 0;
    let mut size: int64_t = 0;
    let mut to_skip: int64_t = 0;
    let mut remaining: int64_t = 0;
    /* Clear out the existing sparse list. */
    gnu_clear_sparse_list(tar);
    remaining = (*tar).entry_bytes_remaining;
    /* Parse entries. */
    entries = gnu_sparse_10_atol(a, tar, &mut remaining, unconsumed) as libc::c_int;
    if entries < 0 as libc::c_int {
        return -(30 as libc::c_int) as ssize_t;
    }
    loop
    /* Parse the individual entries. */
    {
        let fresh1 = entries;
        entries = entries - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        /* Parse offset/size */
        offset = gnu_sparse_10_atol(a, tar, &mut remaining, unconsumed);
        if offset < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int) as ssize_t;
        }
        size = gnu_sparse_10_atol(a, tar, &mut remaining, unconsumed);
        if size < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int) as ssize_t;
        }
        /* Add a new sparse entry. */
        if gnu_add_sparse_entry(a, tar, offset, size) != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
    }
    /* Skip rest of block... */
    tar_flush_unconsumed(a, unconsumed);
    bytes_read = (*tar).entry_bytes_remaining - remaining;
    to_skip = 0x1ff as libc::c_int as libc::c_long & -bytes_read;
    /* Fail if tar->entry_bytes_remaing would get negative */
    if to_skip > remaining {
        return -(30 as libc::c_int) as ssize_t;
    }
    if to_skip != __archive_read_consume(a, to_skip) {
        return -(30 as libc::c_int) as ssize_t;
    }
    return bytes_read + to_skip;
}
/*
 * Solaris pax extension for a sparse file. This is recorded with the
 * data and hole pairs. The way recording sparse information by Solaris'
 * pax simply indicates where data and sparse are, so the stored contents
 * consist of both data and hole.
 */
unsafe extern "C" fn solaris_sparse_parse(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut entry: *mut archive_entry,
    mut p: *const libc::c_char,
) -> libc::c_int {
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut start: int64_t = 0;
    let mut end: int64_t = 0;
    let mut hole: libc::c_int = 1 as libc::c_int;
    /* UNUSED */
    end = 0 as libc::c_int as int64_t;
    if *p as libc::c_int == ' ' as i32 {
        p = p.offset(1)
    } else {
        return -(20 as libc::c_int);
    }
    loop {
        e = p;
        while *e as libc::c_int != '\u{0}' as i32 && *e as libc::c_int != ' ' as i32 {
            if (*e as libc::c_int) < '0' as i32 || *e as libc::c_int > '9' as i32 {
                return -(20 as libc::c_int);
            }
            e = e.offset(1)
        }
        start = end;
        end = tar_atol10(p, e.offset_from(p) as libc::c_long as size_t);
        if end < 0 as libc::c_int as libc::c_long {
            return -(20 as libc::c_int);
        }
        if start < end {
            if gnu_add_sparse_entry(a, tar, start, end - start) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
            (*(*tar).sparse_last).hole = hole
        }
        if *e as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        p = e.offset(1 as libc::c_int as isize);
        hole = (hole == 0 as libc::c_int) as libc::c_int
    }
}
/*-
 * Convert text->integer.
 *
 * Traditional tar formats (including POSIX) specify base-8 for
 * all of the standard numeric fields.  This is a significant limitation
 * in practice:
 *   = file size is limited to 8GB
 *   = rdevmajor and rdevminor are limited to 21 bits
 *   = uid/gid are limited to 21 bits
 *
 * There are two workarounds for this:
 *   = pax extended headers, which use variable-length string fields
 *   = GNU tar and STAR both allow either base-8 or base-256 in
 *      most fields.  The high bit is set to indicate base-256.
 *
 * On read, this implementation supports both extensions.
 */
unsafe extern "C" fn tar_atol(mut p: *const libc::c_char, mut char_cnt: size_t) -> int64_t {
    /*
     * Technically, GNU tar considers a field to be in base-256
     * only if the first byte is 0xff or 0x80.
     */
    if *p as libc::c_int & 0x80 as libc::c_int != 0 {
        return tar_atol256(p, char_cnt);
    }
    return tar_atol8(p, char_cnt);
}
/*
 * Note that this implementation does not (and should not!) obey
 * locale settings; you cannot simply substitute strtol here, since
 * it does obey locale.
 */
unsafe extern "C" fn tar_atol_base_n(
    mut p: *const libc::c_char,
    mut char_cnt: size_t,
    mut base: libc::c_int,
) -> int64_t {
    let mut l: int64_t = 0;
    let mut maxval: int64_t = 0;
    let mut limit: int64_t = 0;
    let mut last_digit_limit: int64_t = 0;
    let mut digit: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    maxval = INT64_MAX;
    limit = INT64_MAX / base as libc::c_long;
    last_digit_limit = INT64_MAX % base as libc::c_long;
    /* the pointer will not be dereferenced if char_cnt is zero
     * due to the way the && operator is evaluated.
     */
    while char_cnt != 0 as libc::c_int as libc::c_ulong
        && (*p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32)
    {
        p = p.offset(1);
        char_cnt = char_cnt.wrapping_sub(1)
    }
    sign = 1 as libc::c_int;
    if char_cnt != 0 as libc::c_int as libc::c_ulong && *p as libc::c_int == '-' as i32 {
        sign = -(1 as libc::c_int);
        p = p.offset(1);
        char_cnt = char_cnt.wrapping_sub(1);
        maxval = INT64_MIN;
        limit = -(INT64_MIN / base as libc::c_long);
        last_digit_limit = INT64_MIN % base as libc::c_long
    }
    l = 0 as libc::c_int as int64_t;
    if char_cnt != 0 as libc::c_int as libc::c_ulong {
        digit = *p as libc::c_int - '0' as i32;
        while digit >= 0 as libc::c_int
            && digit < base
            && char_cnt != 0 as libc::c_int as libc::c_ulong
        {
            if l > limit || l == limit && digit as libc::c_long > last_digit_limit {
                return maxval;
                /* Truncate on overflow. */
            }
            l = l * base as libc::c_long + digit as libc::c_long;
            p = p.offset(1);
            digit = *p as libc::c_int - '0' as i32;
            char_cnt = char_cnt.wrapping_sub(1)
        }
    }
    return if sign < 0 as libc::c_int { -l } else { l };
}
unsafe extern "C" fn tar_atol8(mut p: *const libc::c_char, mut char_cnt: size_t) -> int64_t {
    return tar_atol_base_n(p, char_cnt, 8 as libc::c_int);
}
unsafe extern "C" fn tar_atol10(mut p: *const libc::c_char, mut char_cnt: size_t) -> int64_t {
    return tar_atol_base_n(p, char_cnt, 10 as libc::c_int);
}
/*
 * Parse a base-256 integer.  This is just a variable-length
 * twos-complement signed binary value in big-endian order, except
 * that the high-order bit is ignored.  The values here can be up to
 * 12 bytes, so we need to be careful about overflowing 64-bit
 * (8-byte) integers.
 *
 * This code unashamedly assumes that the local machine uses 8-bit
 * bytes and twos-complement arithmetic.
 */
unsafe extern "C" fn tar_atol256(mut _p: *const libc::c_char, mut char_cnt: size_t) -> int64_t {
    let mut l: uint64_t = 0;
    let mut p: *const libc::c_uchar = _p as *const libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut neg: libc::c_uchar = 0;
    /* Extend 7-bit 2s-comp to 8-bit 2s-comp, decide sign. */
    c = *p;
    if c as libc::c_int & 0x40 as libc::c_int != 0 {
        neg = 0xff as libc::c_int as libc::c_uchar;
        c = (c as libc::c_int | 0x80 as libc::c_int) as libc::c_uchar;
        l = !(0 as libc::c_ulonglong) as uint64_t
    } else {
        neg = 0 as libc::c_int as libc::c_uchar;
        c = (c as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
        l = 0 as libc::c_int as uint64_t
    }
    /* If more than 8 bytes, check that we can ignore
     * high-order bits without overflow. */
    while char_cnt > ::std::mem::size_of::<int64_t>() as libc::c_ulong {
        char_cnt = char_cnt.wrapping_sub(1);
        if c as libc::c_int != neg as libc::c_int {
            return if neg as libc::c_int != 0 {
                INT64_MIN
            } else {
                INT64_MAX
            };
        }
        p = p.offset(1);
        c = *p
    }
    /* c is first byte that fits; if sign mismatch, return overflow */
    if (c as libc::c_int ^ neg as libc::c_int) & 0x80 as libc::c_int != 0 {
        return if neg as libc::c_int != 0 {
            INT64_MIN
        } else {
            INT64_MAX
        };
    }
    loop
    /* Accumulate remaining bytes. */
    {
        char_cnt = char_cnt.wrapping_sub(1);
        if !(char_cnt > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        l = l << 8 as libc::c_int | c as libc::c_ulong;
        p = p.offset(1);
        c = *p
    }
    l = l << 8 as libc::c_int | c as libc::c_ulong;
    /* Return signed twos-complement value. */
    return l as int64_t;
}
/*
 * Returns length of line (including trailing newline)
 * or negative on error.  'start' argument is updated to
 * point to first character of line.  This avoids copying
 * when possible.
 */
unsafe extern "C" fn readline(
    mut a: *mut archive_read,
    mut tar: *mut tar,
    mut start: *mut *const libc::c_char,
    mut limit: ssize_t,
    mut unconsumed: *mut size_t,
) -> ssize_t {
    let mut bytes_read: ssize_t = 0; /* Start of line? */
    let mut total_size: ssize_t = 0 as libc::c_int as ssize_t;
    let mut t: *const libc::c_void = 0 as *const libc::c_void;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    tar_flush_unconsumed(a, unconsumed);
    t = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read);
    if bytes_read <= 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int) as ssize_t;
    }
    s = t as *const libc::c_char;
    p = memchr(t, '\n' as i32, bytes_read as libc::c_ulong);
    /* If we found '\n' in the read buffer, return pointer to that. */
    if !p.is_null() {
        bytes_read = (p as *const libc::c_char)
            .offset(1 as libc::c_int as isize)
            .offset_from(s) as libc::c_long;
        if bytes_read > limit {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Line too long\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        *unconsumed = bytes_read as size_t;
        *start = s;
        return bytes_read;
    }
    *unconsumed = bytes_read as size_t;
    loop
    /* Otherwise, we need to accumulate in a line buffer. */
    {
        if total_size + bytes_read > limit {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Line too long\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        if archive_string_ensure(&mut (*tar).line, (total_size + bytes_read) as size_t).is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate working buffer\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        memcpy(
            (*tar).line.s.offset(total_size as isize) as *mut libc::c_void,
            t,
            bytes_read as libc::c_ulong,
        );
        tar_flush_unconsumed(a, unconsumed);
        total_size += bytes_read;
        /* If we found '\n', clean up and return. */
        if !p.is_null() {
            *start = (*tar).line.s;
            return total_size;
        }
        /* Read some more. */
        t = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read); /* Start of line? */
        if bytes_read <= 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int) as ssize_t;
        }
        s = t as *const libc::c_char;
        p = memchr(t, '\n' as i32, bytes_read as libc::c_ulong);
        /* If we found '\n', trim the read. */
        if !p.is_null() {
            bytes_read = (p as *const libc::c_char)
                .offset(1 as libc::c_int as isize)
                .offset_from(s) as libc::c_long
        }
        *unconsumed = bytes_read as size_t
    }
}
/*
 * base64_decode - Base64 decode
 *
 * This accepts most variations of base-64 encoding, including:
 *    * with or without line breaks
 *    * with or without the final group padded with '=' or '_' characters
 * (The most economical Base-64 variant does not pad the last group and
 * omits line breaks; RFC1341 used for MIME requires both.)
 */
unsafe extern "C" fn base64_decode(
    mut s: *const libc::c_char,
    mut len: size_t,
    mut out_len: *mut size_t,
) -> *mut libc::c_char {
    static mut digits: [libc::c_uchar; 64] = [
        'A' as i32 as libc::c_uchar,
        'B' as i32 as libc::c_uchar,
        'C' as i32 as libc::c_uchar,
        'D' as i32 as libc::c_uchar,
        'E' as i32 as libc::c_uchar,
        'F' as i32 as libc::c_uchar,
        'G' as i32 as libc::c_uchar,
        'H' as i32 as libc::c_uchar,
        'I' as i32 as libc::c_uchar,
        'J' as i32 as libc::c_uchar,
        'K' as i32 as libc::c_uchar,
        'L' as i32 as libc::c_uchar,
        'M' as i32 as libc::c_uchar,
        'N' as i32 as libc::c_uchar,
        'O' as i32 as libc::c_uchar,
        'P' as i32 as libc::c_uchar,
        'Q' as i32 as libc::c_uchar,
        'R' as i32 as libc::c_uchar,
        'S' as i32 as libc::c_uchar,
        'T' as i32 as libc::c_uchar,
        'U' as i32 as libc::c_uchar,
        'V' as i32 as libc::c_uchar,
        'W' as i32 as libc::c_uchar,
        'X' as i32 as libc::c_uchar,
        'Y' as i32 as libc::c_uchar,
        'Z' as i32 as libc::c_uchar,
        'a' as i32 as libc::c_uchar,
        'b' as i32 as libc::c_uchar,
        'c' as i32 as libc::c_uchar,
        'd' as i32 as libc::c_uchar,
        'e' as i32 as libc::c_uchar,
        'f' as i32 as libc::c_uchar,
        'g' as i32 as libc::c_uchar,
        'h' as i32 as libc::c_uchar,
        'i' as i32 as libc::c_uchar,
        'j' as i32 as libc::c_uchar,
        'k' as i32 as libc::c_uchar,
        'l' as i32 as libc::c_uchar,
        'm' as i32 as libc::c_uchar,
        'n' as i32 as libc::c_uchar,
        'o' as i32 as libc::c_uchar,
        'p' as i32 as libc::c_uchar,
        'q' as i32 as libc::c_uchar,
        'r' as i32 as libc::c_uchar,
        's' as i32 as libc::c_uchar,
        't' as i32 as libc::c_uchar,
        'u' as i32 as libc::c_uchar,
        'v' as i32 as libc::c_uchar,
        'w' as i32 as libc::c_uchar,
        'x' as i32 as libc::c_uchar,
        'y' as i32 as libc::c_uchar,
        'z' as i32 as libc::c_uchar,
        '0' as i32 as libc::c_uchar,
        '1' as i32 as libc::c_uchar,
        '2' as i32 as libc::c_uchar,
        '3' as i32 as libc::c_uchar,
        '4' as i32 as libc::c_uchar,
        '5' as i32 as libc::c_uchar,
        '6' as i32 as libc::c_uchar,
        '7' as i32 as libc::c_uchar,
        '8' as i32 as libc::c_uchar,
        '9' as i32 as libc::c_uchar,
        '+' as i32 as libc::c_uchar,
        '/' as i32 as libc::c_uchar,
    ];
    static mut decode_table: [libc::c_uchar; 128] = [0; 128];
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *const libc::c_uchar = s as *const libc::c_uchar;
    /* If the decode table is not yet initialized, prepare it. */
    if decode_table[digits[1 as libc::c_int as usize] as usize] as libc::c_int != 1 as libc::c_int {
        let mut i: libc::c_uint = 0;
        memset(
            decode_table.as_mut_ptr() as *mut libc::c_void,
            0xff as libc::c_int,
            ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong,
        );
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong {
            decode_table[digits[i as usize] as usize] = i as libc::c_uchar;
            i = i.wrapping_add(1)
        }
    }
    /* Allocate enough space to hold the entire output. */
    /* Note that we may not use all of this... */
    out = malloc(
        len.wrapping_sub(len.wrapping_div(4 as libc::c_int as libc::c_ulong))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if out.is_null() {
        *out_len = 0 as libc::c_int as size_t;
        return 0 as *mut libc::c_char;
    }
    d = out;
    while len > 0 as libc::c_int as libc::c_ulong {
        /* Collect the next group of (up to) four characters. */
        let mut v: libc::c_int = 0 as libc::c_int;
        let mut group_size: libc::c_int = 0 as libc::c_int;
        while group_size < 4 as libc::c_int && len > 0 as libc::c_int as libc::c_ulong {
            /* '=' or '_' padding indicates final group. */
            if *src as libc::c_int == '=' as i32 || *src as libc::c_int == '_' as i32 {
                len = 0 as libc::c_int as size_t;
                break;
            } else if *src as libc::c_int > 127 as libc::c_int
                || (*src as libc::c_int) < 32 as libc::c_int
                || decode_table[*src as usize] as libc::c_int == 0xff as libc::c_int
            {
                len = len.wrapping_sub(1);
                src = src.offset(1)
            } else {
                v <<= 6 as libc::c_int;
                let fresh2 = src;
                src = src.offset(1);
                v |= decode_table[*fresh2 as usize] as libc::c_int;
                len = len.wrapping_sub(1);
                group_size += 1
            }
        }
        /* Skip illegal characters (including line breaks) */
        /* Align a short group properly. */
        v <<= 6 as libc::c_int * (4 as libc::c_int - group_size);
        let mut current_block_23: u64;
        /* Unpack the group we just collected. */
        match group_size {
            4 => {
                *d.offset(2 as libc::c_int as isize) = (v & 0xff as libc::c_int) as libc::c_char;
                current_block_23 = 1715091323959057865;
            }
            3 => {
                current_block_23 = 1715091323959057865;
            }
            2 => {
                current_block_23 = 1844362019168185221;
            }
            1 | _ => {
                current_block_23 = 8704759739624374314;
            }
        }
        match current_block_23 {
            1715091323959057865 =>
            /* FALLTHROUGH */
            {
                *d.offset(1 as libc::c_int as isize) =
                    (v >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
                current_block_23 = 1844362019168185221;
            }
            _ => {}
        }
        match current_block_23 {
            1844362019168185221 =>
            /* FALLTHROUGH */
            {
                *d.offset(0 as libc::c_int as isize) =
                    (v >> 16 as libc::c_int & 0xff as libc::c_int) as libc::c_char
            }
            _ => {}
        }
        d = d.offset((group_size * 3 as libc::c_int / 4 as libc::c_int) as isize)
    }
    *out_len = d.offset_from(out) as libc::c_long as size_t;
    return out;
}
unsafe extern "C" fn url_decode(mut in_0: *const libc::c_char) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    out = malloc(strlen(in_0).wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    if out.is_null() {
        return 0 as *mut libc::c_char;
    }
    s = in_0;
    d = out;
    while *s as libc::c_int != '\u{0}' as i32 {
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            && *s.offset(2 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        {
            /* Try to convert % escape */
            let mut digit1: libc::c_int =
                tohex(*s.offset(1 as libc::c_int as isize) as libc::c_int);
            let mut digit2: libc::c_int =
                tohex(*s.offset(2 as libc::c_int as isize) as libc::c_int);
            if digit1 >= 0 as libc::c_int && digit2 >= 0 as libc::c_int {
                /* Looks good, consume three chars */
                s = s.offset(3 as libc::c_int as isize);
                /* Convert output */
                let fresh3 = d;
                d = d.offset(1);
                *fresh3 = (digit1 << 4 as libc::c_int | digit2) as libc::c_char;
                continue;
            }
            /* Else fall through and treat '%' as normal char */
        }
        let fresh4 = s;
        s = s.offset(1);
        let fresh5 = d;
        d = d.offset(1);
        *fresh5 = *fresh4
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return out;
}
unsafe extern "C" fn tohex(mut c: libc::c_int) -> libc::c_int {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    } else if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 10 as libc::c_int;
    } else if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 10 as libc::c_int;
    } else {
        return -(1 as libc::c_int);
    };
}
