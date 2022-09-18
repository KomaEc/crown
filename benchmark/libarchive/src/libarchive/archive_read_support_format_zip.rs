use ::libc;
extern "C" {
    pub type internal_state;
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
    pub type hmac_ctx_st;
    pub type evp_cipher_st;
    pub type evp_cipher_ctx_st;
    pub type archive_write;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateReset(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn archive_filter_bytes(_: *mut archive, _: libc::c_int) -> la_int64_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static __archive_cryptor: archive_cryptor;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_pathname_w(_: *mut archive_entry) -> *const wchar_t;
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
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_pathname_w(_: *mut archive_entry, _: *const wchar_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_is_data_encrypted(_: *mut archive_entry, is_encrypted: libc::c_char);
    #[no_mangle]
    fn archive_entry_set_is_metadata_encrypted(_: *mut archive_entry, is_encrypted: libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_mac_metadata(_: *mut archive_entry, _: *const libc::c_void, _: size_t);
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
    static __archive_hmac: archive_hmac;
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_default_conversion_for_read(_: *mut archive) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
    #[no_mangle]
    fn archive_wstrncat(
        _: *mut archive_wstring,
        _: *const wchar_t,
        _: size_t,
    ) -> *mut archive_wstring;
    #[no_mangle]
    fn archive_wstrcat(_: *mut archive_wstring, _: *const wchar_t) -> *mut archive_wstring;
    #[no_mangle]
    fn archive_wstrappend_wchar(_: *mut archive_wstring, _: wchar_t) -> *mut archive_wstring;
    #[no_mangle]
    fn archive_wstring_free(_: *mut archive_wstring);
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
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
    fn __archive_rb_tree_remove_node(_: *mut archive_rb_tree, _: *mut archive_rb_node);
    #[no_mangle]
    fn __archive_rb_tree_iterate(
        _: *mut archive_rb_tree,
        _: *mut archive_rb_node,
        _: libc::c_uint,
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
    fn __archive_read_seek(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
    /*
     * Get a decryption passphrase.
     */
    #[no_mangle]
    fn __archive_read_reset_passphrase(a: *mut archive_read);
    #[no_mangle]
    fn __archive_read_next_passphrase(a: *mut archive_read) -> *const libc::c_char;
    #[no_mangle]
    static __archive_ppmd8_functions: IPpmd8;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
/* */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip {
    pub format_name: archive_string,
    pub central_directory_offset: int64_t,
    pub central_directory_entries_total: size_t,
    pub central_directory_entries_on_this_disk: size_t,
    pub has_encrypted_entries: libc::c_int,
    pub zip_entries: *mut zip_entry,
    pub tree: archive_rb_tree,
    pub tree_rsrc: archive_rb_tree,
    pub unconsumed: size_t,
    pub entry: *mut zip_entry,
    pub entry_bytes_remaining: int64_t,
    pub entry_compressed_bytes_read: int64_t,
    pub entry_uncompressed_bytes_read: int64_t,
    pub entry_crc32: libc::c_ulong,
    pub crc32func: Option<
        unsafe extern "C" fn(_: libc::c_ulong, _: *const libc::c_void, _: size_t) -> libc::c_ulong,
    >,
    pub ignore_crc32: libc::c_char,
    pub decompress_init: libc::c_char,
    pub end_of_entry: libc::c_char,
    pub uncompressed_buffer: *mut libc::c_uchar,
    pub uncompressed_buffer_size: size_t,
    pub stream: z_stream,
    pub stream_valid: libc::c_char,
    pub zipx_ppmd_stream: IByteIn,
    pub zipx_ppmd_read_compressed: ssize_t,
    pub ppmd8: CPpmd8,
    pub ppmd8_valid: libc::c_char,
    pub ppmd8_stream_failed: libc::c_char,
    pub sconv: *mut archive_string_conv,
    pub sconv_default: *mut archive_string_conv,
    pub sconv_utf8: *mut archive_string_conv,
    pub init_default_conversion: libc::c_int,
    pub process_mac_extensions: libc::c_int,
    pub init_decryption: libc::c_char,
    pub decrypted_buffer: *mut libc::c_uchar,
    pub decrypted_ptr: *mut libc::c_uchar,
    pub decrypted_buffer_size: size_t,
    pub decrypted_bytes_remaining: size_t,
    pub decrypted_unconsumed_bytes: size_t,
    pub tctx: trad_enc_ctx,
    pub tctx_valid: libc::c_char,
    pub cctx: archive_crypto_ctx,
    pub cctx_valid: libc::c_char,
    pub hctx: archive_hmac_sha1_ctx,
    pub hctx_valid: libc::c_char,
    pub iv_size: libc::c_uint,
    pub alg_id: libc::c_uint,
    pub bit_len: libc::c_uint,
    pub flags: libc::c_uint,
    pub erd_size: libc::c_uint,
    pub v_size: libc::c_uint,
    pub v_crc32: libc::c_uint,
    pub iv: *mut uint8_t,
    pub erd: *mut uint8_t,
    pub v_data: *mut uint8_t,
}
/*-
* Copyright (c) 2014 Michihiro NAKAJIMA
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
*/
/*
 * On systems that do not support any recognized crypto libraries,
 * the archive_hmac.c file is expected to define no usable symbols.
 *
 * But some compilers and linkers choke on empty object files, so
 * define a public symbol that will always exist.  This could
 * be removed someday if this file gains another always-present
 * symbol definition.
 */
pub type archive_hmac_sha1_ctx = *mut HMAC_CTX;
pub type HMAC_CTX = hmac_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_crypto_ctx {
    pub ctx: *mut EVP_CIPHER_CTX,
    pub type_0: *const EVP_CIPHER,
    pub key: [uint8_t; 32],
    pub key_len: libc::c_uint,
    pub nonce: [uint8_t; 16],
    pub encr_buf: [uint8_t; 16],
    pub encr_pos: libc::c_uint,
}
pub type EVP_CIPHER = evp_cipher_st;
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trad_enc_ctx {
    pub keys: [uint32_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8 {
    pub MinContext: *mut CPpmd8_Context,
    pub MaxContext: *mut CPpmd8_Context,
    pub FoundState: *mut CPpmd_State,
    pub OrderFall: libc::c_uint,
    pub InitEsc: libc::c_uint,
    pub PrevSuccess: libc::c_uint,
    pub MaxOrder: libc::c_uint,
    pub RunLength: Int32,
    pub InitRL: Int32,
    pub Size: UInt32,
    pub GlueCount: UInt32,
    pub Base: *mut Byte,
    pub LoUnit: *mut Byte,
    pub HiUnit: *mut Byte,
    pub Text: *mut Byte,
    pub UnitsStart: *mut Byte,
    pub AlignOffset: UInt32,
    pub RestoreMethod: libc::c_uint,
    pub Range: UInt32,
    pub Code: UInt32,
    pub Low: UInt32,
    pub Stream: C2RustUnnamed,
    pub Indx2Units: [Byte; 38],
    pub Units2Indx: [Byte; 128],
    pub FreeList: [CPpmd_Void_Ref; 38],
    pub Stamps: [UInt32; 38],
    pub NS2BSIndx: [Byte; 256],
    pub NS2Indx: [Byte; 260],
    pub DummySee: CPpmd_See,
    pub See: [[CPpmd_See; 32]; 24],
    pub BinSumm: [[UInt16; 64]; 25],
}
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_See {
    pub Summ: UInt16,
    pub Shift: Byte,
    pub Count: Byte,
}
pub type UInt32 = libc::c_uint;
pub type CPpmd_Void_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub In: *mut IByteIn,
    pub Out: *mut IByteOut,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteOut {
    pub a: *mut archive_write,
    pub Write: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()>,
}
/* Ppmd.h -- PPMD codec common code
2010-03-12 : Igor Pavlov : Public domain
This code is based on PPMd var.H (2001): Dmitry Shkarin : Public domain */
/* ** Begin defined in Types.h ***/
/* The following interfaces use first parameter as pointer to structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteIn {
    pub a: *mut archive_read,
    pub Read: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> Byte>,
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
pub type Int32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_State {
    pub Symbol: Byte,
    pub Freq: Byte,
    pub SuccessorLow: UInt16,
    pub SuccessorHigh: UInt16,
}
pub type CPpmd8_Context = CPpmd8_Context_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd8_Context_ {
    pub NumStats: Byte,
    pub Flags: Byte,
    pub SummFreq: UInt16,
    pub Stats: CPpmd_State_Ref,
    pub Suffix: CPpmd8_Context_Ref,
}
/* Ppmd8.h -- PPMdI codec
2011-01-27 : Igor Pavlov : Public domain
This code is based on:
  PPMd var.I (2002): Dmitry Shkarin : Public domain
  Carryless rangecoder (1999): Dmitry Subbotin : Public domain */
pub type CPpmd8_Context_Ref = UInt32;
pub type CPpmd_State_Ref = UInt32;
/*
 * The definitive documentation of the Zip file format is:
 *   http://www.pkware.com/documents/casestudies/APPNOTE.TXT
 *
 * The Info-Zip project has pioneered various extensions to better
 * support Zip on Unix, including the 0x5455 "UT", 0x5855 "UX", 0x7855
 * "Ux", and 0x7875 "ux" extensions for time and ownership
 * information.
 *
 * History of this code: The streaming Zip reader was first added to
 * libarchive in January 2005.  Support for seekable input sources was
 * added in Nov 2011.  Zip64 support (including a significant code
 * refactoring) was added in 2014.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_entry {
    pub node: archive_rb_node,
    pub next: *mut zip_entry,
    pub local_header_offset: int64_t,
    pub compressed_size: int64_t,
    pub uncompressed_size: int64_t,
    pub gid: int64_t,
    pub uid: int64_t,
    pub rsrcname: archive_string,
    pub mtime: time_t,
    pub atime: time_t,
    pub ctime: time_t,
    pub crc32: uint32_t,
    pub mode: uint16_t,
    pub zip_flags: uint16_t,
    pub compression: libc::c_uchar,
    pub system: libc::c_uchar,
    pub flags: libc::c_uchar,
    pub decdat: libc::c_uchar,
    pub aes_extra: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub vendor: libc::c_uint,
    pub strength: libc::c_uint,
    pub compression: libc::c_uchar,
}
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
pub struct archive_hmac {
    pub __hmac_sha1_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_hmac_sha1_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub __hmac_sha1_update: Option<
        unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx, _: *const uint8_t, _: size_t) -> (),
    >,
    pub __hmac_sha1_final: Option<
        unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx, _: *mut uint8_t, _: *mut size_t) -> (),
    >,
    pub __hmac_sha1_cleanup: Option<unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx) -> ()>,
}
/* Minimal interface to cryptographic functionality for internal use in
 * libarchive */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_cryptor {
    pub pbkdf2sha1: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: size_t,
            _: *const uint8_t,
            _: size_t,
            _: libc::c_uint,
            _: *mut uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_update: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
            _: *mut uint8_t,
            _: *mut size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_release:
        Option<unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int>,
    pub encrypto_aes_ctr_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub encrypto_aes_ctr_update: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
            _: *mut uint8_t,
            _: *mut size_t,
        ) -> libc::c_int,
    >,
    pub encrypto_aes_ctr_release:
        Option<unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPpmd8 {
    pub Ppmd8_Construct: Option<unsafe extern "C" fn(_: *mut CPpmd8) -> ()>,
    pub Ppmd8_Alloc: Option<unsafe extern "C" fn(_: *mut CPpmd8, _: UInt32) -> Bool>,
    pub Ppmd8_Free: Option<unsafe extern "C" fn(_: *mut CPpmd8) -> ()>,
    pub Ppmd8_Init:
        Option<unsafe extern "C" fn(_: *mut CPpmd8, _: libc::c_uint, _: libc::c_uint) -> ()>,
    pub Ppmd8_RangeDec_Init: Option<unsafe extern "C" fn(_: *mut CPpmd8) -> libc::c_int>,
    pub Ppmd8_DecodeSymbol: Option<unsafe extern "C" fn(_: *mut CPpmd8) -> libc::c_int>,
}
pub type Bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub id: libc::c_int,
    pub name: *const libc::c_char,
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
pub const UINT32_MAX: libc::c_uint = 4294967295 as libc::c_uint;
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const INT64_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
pub const ARCHIVE_ERRNO_PROGRAMMER: libc::c_int = EINVAL;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW: libc::c_int = -(1 as libc::c_int);
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA: libc::c_int =
    (1 as libc::c_int) << 1 as libc::c_int;
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA: libc::c_int =
    (1 as libc::c_int) << 0 as libc::c_int;
pub const ARCHIVE_FORMAT_ZIP: libc::c_int = 0x50000 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
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
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
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
pub const ARCHIVE_RB_DIR_LEFT: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_RB_DIR_RIGHT: libc::c_int = 1 as libc::c_int;
pub const AES_VENDOR_AE_2: libc::c_int = 0x2 as libc::c_int;
/* Bits used in zip_flags. */
pub const ZIP_ENCRYPTED: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const ZIP_LENGTH_AT_END: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
pub const ZIP_STRONG_ENCRYPTED: libc::c_int = (1 as libc::c_int) << 6 as libc::c_int;
pub const ZIP_UTF8_NAME: libc::c_int = (1 as libc::c_int) << 11 as libc::c_int;
/* See "7.2 Single Password Symmetric Encryption Method"
in http://www.pkware.com/documents/casestudies/APPNOTE.TXT */
pub const ZIP_CENTRAL_DIRECTORY_ENCRYPTED: libc::c_int = (1 as libc::c_int) << 13 as libc::c_int;
/* Bits used in flags. */
pub const LA_USED_ZIP64: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const LA_FROM_CENTRAL_DIRECTORY: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
/*
 * See "WinZip - AES Encryption Information"
 *     http://www.winzip.com/aes_info.htm
 */
/* Value used in compression method. */
pub const WINZIP_AES_ENCRYPTION: libc::c_int = 99 as libc::c_int;
/* Authentication code size. */
pub const AUTH_CODE_SIZE: libc::c_int = 10 as libc::c_int;
/* This function is used by Ppmd8_DecodeSymbol during decompression of Ppmd8
 * streams inside ZIP files. It has 2 purposes: one is to fetch the next
 * compressed byte from the stream, second one is to increase the counter how
 * many compressed bytes were read. */
unsafe extern "C" fn ppmd_read(mut p: *mut libc::c_void) -> Byte {
    /* Get the handle to current decompression context. */
    let mut a: *mut archive_read = (*(p as *mut IByteIn)).a;
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut bytes_avail: ssize_t = 0 as libc::c_int as ssize_t;
    /* Fetch next byte. */
    let mut data: *const uint8_t =
        __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail) as *const uint8_t;
    if bytes_avail < 1 as libc::c_int as libc::c_long {
        (*zip).ppmd8_stream_failed = 1 as libc::c_int as libc::c_char;
        return 0 as libc::c_int as Byte;
    }
    __archive_read_consume(a, 1 as libc::c_int as int64_t);
    /* Increment the counter. */
    (*zip).zipx_ppmd_read_compressed += 1;
    /* Return the next compressed byte. */
    return *data.offset(0 as libc::c_int as isize);
}
/* ------------------------------------------------------------------------ */
/*
 Traditional PKWARE Decryption functions.
*/
unsafe extern "C" fn trad_enc_update_keys(mut ctx: *mut trad_enc_ctx, mut c: uint8_t) {
    let mut t: uint8_t = 0;
    (*ctx).keys[0 as libc::c_int as usize] = (crc32(
        (*ctx).keys[0 as libc::c_int as usize] as libc::c_ulong ^ 0xffffffff as libc::c_ulong,
        &mut c,
        1 as libc::c_int as uInt,
    ) ^ 0xffffffff as libc::c_ulong) as uint32_t;
    (*ctx).keys[1 as libc::c_int as usize] = ((*ctx).keys[1 as libc::c_int as usize]
        .wrapping_add((*ctx).keys[0 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
        as libc::c_long
        * 134775813 as libc::c_long
        + 1 as libc::c_int as libc::c_long)
        as uint32_t;
    t = ((*ctx).keys[1 as libc::c_int as usize] >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    (*ctx).keys[2 as libc::c_int as usize] = (crc32(
        (*ctx).keys[2 as libc::c_int as usize] as libc::c_ulong ^ 0xffffffff as libc::c_ulong,
        &mut t,
        1 as libc::c_int as uInt,
    ) ^ 0xffffffff as libc::c_ulong) as uint32_t;
}
unsafe extern "C" fn trad_enc_decrypt_byte(mut ctx: *mut trad_enc_ctx) -> uint8_t {
    let mut temp: libc::c_uint =
        (*ctx).keys[2 as libc::c_int as usize] | 2 as libc::c_int as libc::c_uint;
    return ((temp.wrapping_mul(temp ^ 1 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
        as uint8_t as libc::c_int
        & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn trad_enc_decrypt_update(
    mut ctx: *mut trad_enc_ctx,
    mut in_0: *const uint8_t,
    mut in_len: size_t,
    mut out: *mut uint8_t,
    mut out_len: size_t,
) {
    let mut i: libc::c_uint = 0;
    let mut max: libc::c_uint = 0;
    max = if in_len < out_len { in_len } else { out_len } as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < max {
        let mut t: uint8_t = (*in_0.offset(i as isize) as libc::c_int
            ^ trad_enc_decrypt_byte(ctx) as libc::c_int) as uint8_t;
        *out.offset(i as isize) = t;
        trad_enc_update_keys(ctx, t);
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn trad_enc_init(
    mut ctx: *mut trad_enc_ctx,
    mut pw: *const libc::c_char,
    mut pw_len: size_t,
    mut key: *const uint8_t,
    mut key_len: size_t,
    mut crcchk: *mut uint8_t,
) -> libc::c_int {
    let mut header: [uint8_t; 12] = [0; 12];
    if key_len < 12 as libc::c_int as libc::c_ulong {
        *crcchk = 0xff as libc::c_int as uint8_t;
        return -(1 as libc::c_int);
    }
    (*ctx).keys[0 as libc::c_int as usize] = 305419896 as libc::c_long as uint32_t;
    (*ctx).keys[1 as libc::c_int as usize] = 591751049 as libc::c_long as uint32_t;
    (*ctx).keys[2 as libc::c_int as usize] = 878082192 as libc::c_long as uint32_t;
    while pw_len != 0 {
        let fresh0 = pw;
        pw = pw.offset(1);
        trad_enc_update_keys(ctx, *fresh0 as uint8_t);
        pw_len = pw_len.wrapping_sub(1)
    }
    trad_enc_decrypt_update(
        ctx,
        key,
        12 as libc::c_int as size_t,
        header.as_mut_ptr(),
        12 as libc::c_int as size_t,
    );
    /* Return the last byte for CRC check. */
    *crcchk = header[11 as libc::c_int as usize];
    return 0 as libc::c_int;
}
/*
 * Common code for streaming or seeking modes.
 *
 * Includes code to read local file headers, decompress data
 * from entry bodies, and common API.
 */
unsafe extern "C" fn real_crc32(
    mut crc: libc::c_ulong,
    mut buff: *const libc::c_void,
    mut len: size_t,
) -> libc::c_ulong {
    return crc32(crc, buff as *const Bytef, len as libc::c_uint);
}
/* Used by "ignorecrc32" option to speed up tests. */
unsafe extern "C" fn fake_crc32(
    mut crc: libc::c_ulong,
    mut buff: *const libc::c_void,
    mut len: size_t,
) -> libc::c_ulong {
    /* UNUSED */
    return 0 as libc::c_int as libc::c_ulong;
}
static mut compression_methods: [C2RustUnnamed_2; 25] = [
    {
        let mut init = C2RustUnnamed_2 {
            id: 0 as libc::c_int,
            name: b"uncompressed\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 1 as libc::c_int,
            name: b"shrinking\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 2 as libc::c_int,
            name: b"reduced-1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 3 as libc::c_int,
            name: b"reduced-2\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 4 as libc::c_int,
            name: b"reduced-3\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 5 as libc::c_int,
            name: b"reduced-4\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 6 as libc::c_int,
            name: b"imploded\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 7 as libc::c_int,
            name: b"reserved\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 8 as libc::c_int,
            name: b"deflation\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 9 as libc::c_int,
            name: b"deflation-64-bit\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 10 as libc::c_int,
            name: b"ibm-terse\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 11 as libc::c_int,
            name: b"reserved\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 12 as libc::c_int,
            name: b"bzip\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 13 as libc::c_int,
            name: b"reserved\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 14 as libc::c_int,
            name: b"lzma\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 15 as libc::c_int,
            name: b"reserved\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 16 as libc::c_int,
            name: b"reserved\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 17 as libc::c_int,
            name: b"reserved\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 18 as libc::c_int,
            name: b"ibm-terse-new\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 19 as libc::c_int,
            name: b"ibm-lz777\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 95 as libc::c_int,
            name: b"xz\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 96 as libc::c_int,
            name: b"jpeg\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 97 as libc::c_int,
            name: b"wav-pack\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 98 as libc::c_int,
            name: b"ppmd-1\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_2 {
            id: 99 as libc::c_int,
            name: b"aes\x00" as *const u8 as *const libc::c_char,
        };
        init
    },
];
// Initialized in run_static_initializers
static mut num_compression_methods: libc::c_int = 0;
unsafe extern "C" fn compression_name(compression: libc::c_int) -> *const libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    while compression >= 0 as libc::c_int && i < num_compression_methods {
        if compression_methods[i as usize].id == compression {
            return compression_methods[i as usize].name;
        }
        i += 1
    }
    return b"??\x00" as *const u8 as *const libc::c_char;
}
/* Convert an MSDOS-style date/time into Unix-style time. */
unsafe extern "C" fn zip_time(mut p: *const libc::c_char) -> time_t {
    let mut msTime: libc::c_int = 0; /* Years since 1900. */
    let mut msDate: libc::c_int = 0; /* Month number. */
    let mut ts: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    }; /* Day of month. */
    msTime = (0xff as libc::c_int as libc::c_uint
        & *p.offset(0 as libc::c_int as isize) as libc::c_uint)
        .wrapping_add((256 as libc::c_int as libc::c_uint).wrapping_mul(
            0xff as libc::c_int as libc::c_uint
                & *p.offset(1 as libc::c_int as isize) as libc::c_uint,
        )) as libc::c_int;
    msDate = (0xff as libc::c_int as libc::c_uint
        & *p.offset(2 as libc::c_int as isize) as libc::c_uint)
        .wrapping_add((256 as libc::c_int as libc::c_uint).wrapping_mul(
            0xff as libc::c_int as libc::c_uint
                & *p.offset(3 as libc::c_int as isize) as libc::c_uint,
        )) as libc::c_int;
    memset(
        &mut ts as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tm>() as libc::c_ulong,
    );
    ts.tm_year = (msDate >> 9 as libc::c_int & 0x7f as libc::c_int) + 80 as libc::c_int;
    ts.tm_mon = (msDate >> 5 as libc::c_int & 0xf as libc::c_int) - 1 as libc::c_int;
    ts.tm_mday = msDate & 0x1f as libc::c_int;
    ts.tm_hour = msTime >> 11 as libc::c_int & 0x1f as libc::c_int;
    ts.tm_min = msTime >> 5 as libc::c_int & 0x3f as libc::c_int;
    ts.tm_sec = msTime << 1 as libc::c_int & 0x3e as libc::c_int;
    ts.tm_isdst = -(1 as libc::c_int);
    return mktime(&mut ts);
}
/*
 * The extra data is stored as a list of
 *	id1+size1+data1 + id2+size2+data2 ...
 *  triplets.  id and size are 2 bytes each.
 */
unsafe extern "C" fn process_extra(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut p: *const libc::c_char,
    mut extra_length: size_t,
    mut zip_entry: *mut zip_entry,
) -> libc::c_int {
    let mut offset: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    if extra_length == 0 as libc::c_int as libc::c_ulong {
        return ARCHIVE_OK;
    }
    if extra_length < 4 as libc::c_int as libc::c_ulong {
        let mut i: size_t = 0 as libc::c_int as size_t;
        /* Some ZIP files may have trailing 0 bytes. Let's check they
         * are all 0 and ignore them instead of returning an error.
         *
         * This is not technically correct, but some ZIP files look
         * like this and other tools support those files - so let's
         * also  support them.
         */
        while i < extra_length {
            if *p.offset(i as isize) as libc::c_int != 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Too-small extra data: Need at least 4 bytes, but only found %d bytes\x00"
                        as *const u8 as *const libc::c_char,
                    extra_length as libc::c_int,
                );
                return ARCHIVE_FAILED;
            }
            i = i.wrapping_add(1)
        }
        return ARCHIVE_OK;
    }
    while offset as libc::c_ulong <= extra_length.wrapping_sub(4 as libc::c_int as libc::c_ulong) {
        let mut headerid: libc::c_ushort =
            archive_le16dec(p.offset(offset as isize) as *const libc::c_void);
        let mut datasize: libc::c_ushort = archive_le16dec(
            p.offset(offset as isize).offset(2 as libc::c_int as isize) as *const libc::c_void,
        );
        offset = offset.wrapping_add(4 as libc::c_int as libc::c_uint);
        if offset.wrapping_add(datasize as libc::c_uint) as libc::c_ulong > extra_length {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Extra data overflow: Need %d bytes but only found %d bytes\x00" as *const u8
                    as *const libc::c_char,
                datasize as libc::c_int,
                extra_length.wrapping_sub(offset as libc::c_ulong) as libc::c_int,
            );
            return ARCHIVE_FAILED;
        }
        let mut current_block_140: u64;
        match headerid as libc::c_int {
            1 => {
                /* Zip64 extended information extra field. */
                (*zip_entry).flags =
                    ((*zip_entry).flags as libc::c_int | LA_USED_ZIP64) as libc::c_uchar;
                if (*zip_entry).uncompressed_size == 0xffffffff as libc::c_uint as libc::c_long {
                    let mut t: uint64_t = 0 as libc::c_int as uint64_t;
                    if (datasize as libc::c_int) < 8 as libc::c_int || {
                        t = archive_le64dec(p.offset(offset as isize) as *const libc::c_void);
                        (t) > INT64_MAX as libc::c_ulong
                    } {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Malformed 64-bit uncompressed size\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return ARCHIVE_FAILED;
                    }
                    (*zip_entry).uncompressed_size = t as int64_t;
                    offset = offset.wrapping_add(8 as libc::c_int as libc::c_uint);
                    datasize = (datasize as libc::c_int - 8 as libc::c_int) as libc::c_ushort
                }
                if (*zip_entry).compressed_size == 0xffffffff as libc::c_uint as libc::c_long {
                    let mut t_0: uint64_t = 0 as libc::c_int as uint64_t;
                    if (datasize as libc::c_int) < 8 as libc::c_int || {
                        t_0 = archive_le64dec(p.offset(offset as isize) as *const libc::c_void);
                        (t_0) > INT64_MAX as libc::c_ulong
                    } {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Malformed 64-bit compressed size\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return ARCHIVE_FAILED;
                    }
                    (*zip_entry).compressed_size = t_0 as int64_t;
                    offset = offset.wrapping_add(8 as libc::c_int as libc::c_uint);
                    datasize = (datasize as libc::c_int - 8 as libc::c_int) as libc::c_ushort
                }
                if (*zip_entry).local_header_offset == 0xffffffff as libc::c_uint as libc::c_long {
                    let mut t_1: uint64_t = 0 as libc::c_int as uint64_t;
                    if (datasize as libc::c_int) < 8 as libc::c_int || {
                        t_1 = archive_le64dec(p.offset(offset as isize) as *const libc::c_void);
                        (t_1) > INT64_MAX as libc::c_ulong
                    } {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Malformed 64-bit local header offset\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return ARCHIVE_FAILED;
                    }
                    (*zip_entry).local_header_offset = t_1 as int64_t;
                    offset = offset.wrapping_add(8 as libc::c_int as libc::c_uint);
                    datasize = (datasize as libc::c_int - 8 as libc::c_int) as libc::c_ushort
                }
            }
            21589 => {
                /* Extended time field "UT". */
                let mut flags: libc::c_int = 0;
                if datasize as libc::c_int == 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Incomplete extended time field\x00" as *const u8 as *const libc::c_char,
                    );
                    return ARCHIVE_FAILED;
                }
                flags = *p.offset(offset as isize) as libc::c_int;
                offset = offset.wrapping_add(1);
                datasize = datasize.wrapping_sub(1);
                /* Flag bits indicate which dates are present. */
                if flags & 0x1 as libc::c_int != 0 {
                    if (datasize as libc::c_int) < 4 as libc::c_int {
                        current_block_140 = 6893286596494697181;
                    } else {
                        (*zip_entry).mtime =
                            archive_le32dec(p.offset(offset as isize) as *const libc::c_void)
                                as time_t;
                        offset = offset.wrapping_add(4 as libc::c_int as libc::c_uint);
                        datasize = (datasize as libc::c_int - 4 as libc::c_int) as libc::c_ushort;
                        current_block_140 = 6072622540298447352;
                    }
                } else {
                    current_block_140 = 6072622540298447352;
                }
                match current_block_140 {
                    6893286596494697181 => {}
                    _ => {
                        if flags & 0x2 as libc::c_int != 0 {
                            if (datasize as libc::c_int) < 4 as libc::c_int {
                                current_block_140 = 6893286596494697181;
                            } else {
                                (*zip_entry).atime = archive_le32dec(
                                    p.offset(offset as isize) as *const libc::c_void
                                ) as time_t;
                                offset = offset.wrapping_add(4 as libc::c_int as libc::c_uint);
                                datasize =
                                    (datasize as libc::c_int - 4 as libc::c_int) as libc::c_ushort;
                                current_block_140 = 17075014677070940716;
                            }
                        } else {
                            current_block_140 = 17075014677070940716;
                        }
                        match current_block_140 {
                            6893286596494697181 => {}
                            _ => {
                                if flags & 0x4 as libc::c_int != 0 {
                                    if !((datasize as libc::c_int) < 4 as libc::c_int) {
                                        (*zip_entry).ctime = archive_le32dec(
                                            p.offset(offset as isize) as *const libc::c_void,
                                        )
                                            as time_t;
                                        offset =
                                            offset.wrapping_add(4 as libc::c_int as libc::c_uint);
                                        datasize = (datasize as libc::c_int - 4 as libc::c_int)
                                            as libc::c_ushort
                                    }
                                }
                            }
                        }
                    }
                }
            }
            22613 => {
                /* Info-ZIP Unix Extra Field (old version) "UX". */
                if datasize as libc::c_int >= 8 as libc::c_int {
                    (*zip_entry).atime =
                        archive_le32dec(p.offset(offset as isize) as *const libc::c_void) as time_t;
                    (*zip_entry).mtime =
                        archive_le32dec(p.offset(offset as isize).offset(4 as libc::c_int as isize)
                            as *const libc::c_void) as time_t
                }
                if datasize as libc::c_int >= 12 as libc::c_int {
                    (*zip_entry).uid =
                        archive_le16dec(p.offset(offset as isize).offset(8 as libc::c_int as isize)
                            as *const libc::c_void) as int64_t;
                    (*zip_entry).gid = archive_le16dec(
                        p.offset(offset as isize).offset(10 as libc::c_int as isize)
                            as *const libc::c_void,
                    ) as int64_t
                }
            }
            27768 => {
                /* Experimental 'xl' field */
                /*
                 * Introduced Dec 2013 to provide a way to
                 * include external file attributes (and other
                 * fields that ordinarily appear only in
                 * central directory) in local file header.
                 * This provides file type and permission
                 * information necessary to support full
                 * streaming extraction.  Currently being
                 * discussed with other Zip developers
                 * ... subject to change.
                 *
                 * Format:
                 *  The field starts with a bitmap that specifies
                 *  which additional fields are included.  The
                 *  bitmap is variable length and can be extended in
                 *  the future.
                 *
                 *  n bytes - feature bitmap: first byte has low-order
                 *    7 bits.  If high-order bit is set, a subsequent
                 *    byte holds the next 7 bits, etc.
                 *
                 *  if bitmap & 1, 2 byte "version made by"
                 *  if bitmap & 2, 2 byte "internal file attributes"
                 *  if bitmap & 4, 4 byte "external file attributes"
                 *  if bitmap & 8, 2 byte comment length + n byte
                 *  comment
                 */
                let mut bitmap: libc::c_int = 0;
                let mut bitmap_last: libc::c_int = 0;
                if !((datasize as libc::c_int) < 1 as libc::c_int) {
                    bitmap = 0xff as libc::c_int & *p.offset(offset as isize) as libc::c_int;
                    bitmap_last = bitmap;
                    offset = offset.wrapping_add(1 as libc::c_int as libc::c_uint);
                    datasize = (datasize as libc::c_int - 1 as libc::c_int) as libc::c_ushort;
                    /* We only support first 7 bits of bitmap; skip rest. */
                    while bitmap_last & 0x80 as libc::c_int != 0 as libc::c_int
                        && datasize as libc::c_int >= 1 as libc::c_int
                    {
                        bitmap_last = *p.offset(offset as isize) as libc::c_int;
                        offset = offset.wrapping_add(1 as libc::c_int as libc::c_uint);
                        datasize = (datasize as libc::c_int - 1 as libc::c_int) as libc::c_ushort
                    }
                    if bitmap & 1 as libc::c_int != 0 {
                        /* 2 byte "version made by" */
                        if (datasize as libc::c_int) < 2 as libc::c_int {
                            current_block_140 = 6893286596494697181;
                        } else {
                            (*zip_entry).system =
                                (archive_le16dec(p.offset(offset as isize) as *const libc::c_void)
                                    as libc::c_int
                                    >> 8 as libc::c_int)
                                    as libc::c_uchar;
                            offset = offset.wrapping_add(2 as libc::c_int as libc::c_uint);
                            datasize =
                                (datasize as libc::c_int - 2 as libc::c_int) as libc::c_ushort;
                            current_block_140 = 6471821049853688503;
                        }
                    } else {
                        current_block_140 = 6471821049853688503;
                    }
                    match current_block_140 {
                        6893286596494697181 => {}
                        _ => {
                            if bitmap & 2 as libc::c_int != 0 {
                                /* 2 byte "internal file attributes" */
                                let mut internal_attributes: uint32_t = 0;
                                if (datasize as libc::c_int) < 2 as libc::c_int {
                                    current_block_140 = 6893286596494697181;
                                } else {
                                    internal_attributes = archive_le16dec(
                                        p.offset(offset as isize) as *const libc::c_void
                                    )
                                        as uint32_t;
                                    /* Not used by libarchive at present. */
                                    /* UNUSED */
                                    offset = offset.wrapping_add(2 as libc::c_int as libc::c_uint);
                                    datasize = (datasize as libc::c_int - 2 as libc::c_int)
                                        as libc::c_ushort;
                                    current_block_140 = 6712462580143783635;
                                }
                            } else {
                                current_block_140 = 6712462580143783635;
                            }
                            match current_block_140 {
                                6893286596494697181 => {}
                                _ => {
                                    if bitmap & 4 as libc::c_int != 0 {
                                        /* 4 byte "external file attributes" */
                                        let mut external_attributes: uint32_t = 0;
                                        if (datasize as libc::c_int) < 4 as libc::c_int {
                                            current_block_140 = 6893286596494697181;
                                        } else {
                                            external_attributes =
                                                archive_le32dec(p.offset(offset as isize)
                                                    as *const libc::c_void);
                                            if (*zip_entry).system as libc::c_int
                                                == 3 as libc::c_int
                                            {
                                                (*zip_entry).mode = (external_attributes
                                                    >> 16 as libc::c_int)
                                                    as uint16_t
                                            } else if (*zip_entry).system as libc::c_int
                                                == 0 as libc::c_int
                                            {
                                                // Interpret MSDOS directory bit
                                                if 0x10 as libc::c_int as libc::c_uint
                                                    == external_attributes
                                                        & 0x10 as libc::c_int as libc::c_uint
                                                {
                                                    (*zip_entry).mode = (AE_IFDIR as mode_t
                                                        | 0o775 as libc::c_int as libc::c_uint)
                                                        as uint16_t
                                                } else {
                                                    (*zip_entry).mode = (AE_IFREG as mode_t
                                                        | 0o664 as libc::c_int as libc::c_uint)
                                                        as uint16_t
                                                }
                                                if 0x1 as libc::c_int as libc::c_uint
                                                    == external_attributes
                                                        & 0x1 as libc::c_int as libc::c_uint
                                                {
                                                    /* Read-only bit;
                                                     * strip write permissions */
                                                    (*zip_entry).mode = ((*zip_entry).mode
                                                        as libc::c_int
                                                        & 0o555 as libc::c_int)
                                                        as uint16_t
                                                }
                                            } else {
                                                (*zip_entry).mode = 0 as libc::c_int as uint16_t
                                            }
                                            offset = offset
                                                .wrapping_add(4 as libc::c_int as libc::c_uint);
                                            datasize = (datasize as libc::c_int - 4 as libc::c_int)
                                                as libc::c_ushort;
                                            current_block_140 = 1013506999122146761;
                                        }
                                    } else {
                                        current_block_140 = 1013506999122146761;
                                    }
                                    match current_block_140 {
                                        6893286596494697181 => {}
                                        _ => {
                                            if bitmap & 8 as libc::c_int != 0 {
                                                /* 2 byte comment length + comment */
                                                let mut comment_length: uint32_t = 0;
                                                if !((datasize as libc::c_int) < 2 as libc::c_int) {
                                                    comment_length =
                                                        archive_le16dec(p.offset(offset as isize)
                                                            as *const libc::c_void)
                                                            as uint32_t;
                                                    offset = offset.wrapping_add(
                                                        2 as libc::c_int as libc::c_uint,
                                                    );
                                                    datasize = (datasize as libc::c_int
                                                        - 2 as libc::c_int)
                                                        as libc::c_ushort;
                                                    if !((datasize as libc::c_uint)
                                                        < comment_length)
                                                    {
                                                        /* Comment is not supported by libarchive */
                                                        offset =
                                                            offset.wrapping_add(comment_length);
                                                        datasize = (datasize as libc::c_uint)
                                                            .wrapping_sub(comment_length)
                                                            as libc::c_ushort
                                                            as libc::c_ushort
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            28789 => {
                /* Info-ZIP Unicode Path Extra Field. */
                if !((datasize as libc::c_int) < 5 as libc::c_int || entry.is_null()) {
                    offset = offset.wrapping_add(5 as libc::c_int as libc::c_uint);
                    datasize = (datasize as libc::c_int - 5 as libc::c_int) as libc::c_ushort;
                    /* The path name in this field is always encoded
                     * in UTF-8. */
                    if (*zip).sconv_utf8.is_null() {
                        (*zip).sconv_utf8 = archive_string_conversion_from_charset(
                            &mut (*a).archive,
                            b"UTF-8\x00" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                        );
                        /* If the converter from UTF-8 is not
                         * available, then the path name from the main
                         * field will more likely be correct. */
                        if (*zip).sconv_utf8.is_null() {
                            current_block_140 = 6893286596494697181;
                        } else {
                            current_block_140 = 914440069034635393;
                        }
                    } else {
                        current_block_140 = 914440069034635393;
                    }
                    match current_block_140 {
                        6893286596494697181 => {}
                        _ =>
                        /* Make sure the CRC32 of the filename matches. */
                        {
                            if (*zip).ignore_crc32 == 0 {
                                let mut cp: *const libc::c_char = archive_entry_pathname(entry);
                                if !cp.is_null() {
                                    let mut file_crc: libc::c_ulong =
                                        (*zip).crc32func.expect("non-null function pointer")(
                                            0 as libc::c_int as libc::c_ulong,
                                            cp as *const libc::c_void,
                                            strlen(cp),
                                        );
                                    let mut utf_crc: libc::c_ulong = archive_le32dec(
                                        p.offset(offset as isize)
                                            .offset(-(4 as libc::c_int as isize))
                                            as *const libc::c_void,
                                    )
                                        as libc::c_ulong;
                                    if file_crc != utf_crc {
                                        current_block_140 = 6893286596494697181;
                                    } else {
                                        current_block_140 = 4235089732467486934;
                                    }
                                } else {
                                    current_block_140 = 4235089732467486934;
                                }
                            } else {
                                current_block_140 = 4235089732467486934;
                            }
                            match current_block_140 {
                                6893286596494697181 => {}
                                _ => {
                                    (_archive_entry_copy_pathname_l(
                                        entry,
                                        p.offset(offset as isize),
                                        datasize as size_t,
                                        (*zip).sconv_utf8,
                                    )) != 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
            30805 => {
                /* Info-ZIP Unix Extra Field (type 2) "Ux". */
                if datasize as libc::c_int >= 2 as libc::c_int {
                    (*zip_entry).uid =
                        archive_le16dec(p.offset(offset as isize) as *const libc::c_void) as int64_t
                }
                if datasize as libc::c_int >= 4 as libc::c_int {
                    (*zip_entry).gid =
                        archive_le16dec(p.offset(offset as isize).offset(2 as libc::c_int as isize)
                            as *const libc::c_void) as int64_t
                }
            }
            30837 => {
                /* Info-Zip Unix Extra Field (type 3) "ux". */
                let mut uidsize: libc::c_int = 0 as libc::c_int;
                let mut gidsize: libc::c_int = 0 as libc::c_int;
                /* TODO: support arbitrary uidsize/gidsize. */
                if datasize as libc::c_int >= 1 as libc::c_int
                    && *p.offset(offset as isize) as libc::c_int == 1 as libc::c_int
                {
                    /* version=1 */
                    if datasize as libc::c_int >= 4 as libc::c_int {
                        /* get a uid size. */
                        uidsize = 0xff as libc::c_int
                            & *p.offset(
                                offset.wrapping_add(1 as libc::c_int as libc::c_uint) as isize
                            ) as libc::c_int;
                        if uidsize == 2 as libc::c_int {
                            (*zip_entry).uid = archive_le16dec(
                                p.offset(offset as isize).offset(2 as libc::c_int as isize)
                                    as *const libc::c_void,
                            ) as int64_t
                        } else if uidsize == 4 as libc::c_int
                            && datasize as libc::c_int >= 6 as libc::c_int
                        {
                            (*zip_entry).uid = archive_le32dec(
                                p.offset(offset as isize).offset(2 as libc::c_int as isize)
                                    as *const libc::c_void,
                            ) as int64_t
                        }
                    }
                    if datasize as libc::c_int >= 2 as libc::c_int + uidsize + 3 as libc::c_int {
                        /* get a gid size. */
                        gidsize = 0xff as libc::c_int
                            & *p.offset(
                                offset
                                    .wrapping_add(2 as libc::c_int as libc::c_uint)
                                    .wrapping_add(uidsize as libc::c_uint)
                                    as isize,
                            ) as libc::c_int;
                        if gidsize == 2 as libc::c_int {
                            (*zip_entry).gid = archive_le16dec(
                                p.offset(offset as isize)
                                    .offset(2 as libc::c_int as isize)
                                    .offset(uidsize as isize)
                                    .offset(1 as libc::c_int as isize)
                                    as *const libc::c_void,
                            ) as int64_t
                        } else if gidsize == 4 as libc::c_int
                            && datasize as libc::c_int
                                >= 2 as libc::c_int + uidsize + 5 as libc::c_int
                        {
                            (*zip_entry).gid = archive_le32dec(
                                p.offset(offset as isize)
                                    .offset(2 as libc::c_int as isize)
                                    .offset(uidsize as isize)
                                    .offset(1 as libc::c_int as isize)
                                    as *const libc::c_void,
                            ) as int64_t
                        }
                    }
                }
            }
            39169 => {
                /* WinZip AES extra data field. */
                if (datasize as libc::c_int) < 6 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Incomplete AES field\x00" as *const u8 as *const libc::c_char,
                    );
                    return ARCHIVE_FAILED;
                }
                if *p.offset(offset.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_int
                    == 'A' as i32
                    && *p.offset(offset.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
                        as libc::c_int
                        == 'E' as i32
                {
                    /* Vendor version. */
                    (*zip_entry).aes_extra.vendor =
                        archive_le16dec(p.offset(offset as isize) as *const libc::c_void)
                            as libc::c_uint;
                    /* AES encryption strength. */
                    (*zip_entry).aes_extra.strength = *p
                        .offset(offset.wrapping_add(4 as libc::c_int as libc::c_uint) as isize)
                        as libc::c_uint;
                    /* Actual compression method. */
                    (*zip_entry).aes_extra.compression = *p
                        .offset(offset.wrapping_add(5 as libc::c_int as libc::c_uint) as isize)
                        as libc::c_uchar
                }
            }
            _ => {}
        }
        offset = offset.wrapping_add(datasize as libc::c_uint)
    }
    return ARCHIVE_OK;
}
/*
 * Assumes file pointer is at beginning of local file header.
 */
unsafe extern "C" fn zip_read_local_file_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut zip: *mut zip,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut wp: *const wchar_t = 0 as *const wchar_t;
    let mut cp: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut filename_length: size_t = 0;
    let mut extra_length: size_t = 0;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut zip_entry: *mut zip_entry = (*zip).entry;
    let mut zip_entry_central_dir: zip_entry = zip_entry {
        node: archive_rb_node {
            rb_nodes: [0 as *mut archive_rb_node; 2],
            rb_info: 0,
        },
        next: 0 as *mut zip_entry,
        local_header_offset: 0,
        compressed_size: 0,
        uncompressed_size: 0,
        gid: 0,
        uid: 0,
        rsrcname: archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        },
        mtime: 0,
        atime: 0,
        ctime: 0,
        crc32: 0,
        mode: 0,
        zip_flags: 0,
        compression: 0,
        system: 0,
        flags: 0,
        decdat: 0,
        aes_extra: C2RustUnnamed_1 {
            vendor: 0,
            strength: 0,
            compression: 0,
        },
    };
    let mut ret: libc::c_int = ARCHIVE_OK;
    let mut version: libc::c_char = 0;
    /* Save a copy of the original for consistency checks. */
    zip_entry_central_dir = *zip_entry;
    (*zip).decompress_init = 0 as libc::c_int as libc::c_char;
    (*zip).end_of_entry = 0 as libc::c_int as libc::c_char;
    (*zip).entry_uncompressed_bytes_read = 0 as libc::c_int as int64_t;
    (*zip).entry_compressed_bytes_read = 0 as libc::c_int as int64_t;
    (*zip).entry_crc32 = (*zip).crc32func.expect("non-null function pointer")(
        0 as libc::c_int as libc::c_ulong,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    /* Setup default conversion. */
    if (*zip).sconv.is_null() && (*zip).init_default_conversion == 0 {
        (*zip).sconv_default = archive_string_default_conversion_for_read(&mut (*a).archive);
        (*zip).init_default_conversion = 1 as libc::c_int
    }
    p = __archive_read_ahead(a, 30 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if p.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated ZIP file header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if memcmp(
        p as *const libc::c_void,
        b"PK\x03\x04\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Damaged Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    version = *p.offset(4 as libc::c_int as isize);
    (*zip_entry).system = *p.offset(5 as libc::c_int as isize) as libc::c_uchar;
    (*zip_entry).zip_flags =
        archive_le16dec(p.offset(6 as libc::c_int as isize) as *const libc::c_void);
    if (*zip_entry).zip_flags as libc::c_int & (ZIP_ENCRYPTED | ZIP_STRONG_ENCRYPTED) != 0 {
        (*zip).has_encrypted_entries = 1 as libc::c_int;
        archive_entry_set_is_data_encrypted(entry, 1 as libc::c_int as libc::c_char);
        if (*zip_entry).zip_flags as libc::c_int & ZIP_CENTRAL_DIRECTORY_ENCRYPTED != 0
            && (*zip_entry).zip_flags as libc::c_int & ZIP_ENCRYPTED != 0
            && (*zip_entry).zip_flags as libc::c_int & ZIP_STRONG_ENCRYPTED != 0
        {
            archive_entry_set_is_metadata_encrypted(entry, 1 as libc::c_int as libc::c_char);
            return ARCHIVE_FATAL;
        }
    }
    (*zip).init_decryption =
        ((*zip_entry).zip_flags as libc::c_int & ZIP_ENCRYPTED) as libc::c_char;
    (*zip_entry).compression =
        archive_le16dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void) as libc::c_char
            as libc::c_uchar;
    (*zip_entry).mtime = zip_time(p.offset(10 as libc::c_int as isize));
    (*zip_entry).crc32 =
        archive_le32dec(p.offset(14 as libc::c_int as isize) as *const libc::c_void);
    if (*zip_entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END != 0 {
        (*zip_entry).decdat = *p.offset(11 as libc::c_int as isize) as libc::c_uchar
    } else {
        (*zip_entry).decdat = *p.offset(17 as libc::c_int as isize) as libc::c_uchar
    }
    (*zip_entry).compressed_size =
        archive_le32dec(p.offset(18 as libc::c_int as isize) as *const libc::c_void) as int64_t;
    (*zip_entry).uncompressed_size =
        archive_le32dec(p.offset(22 as libc::c_int as isize) as *const libc::c_void) as int64_t;
    filename_length =
        archive_le16dec(p.offset(26 as libc::c_int as isize) as *const libc::c_void) as size_t;
    extra_length =
        archive_le16dec(p.offset(28 as libc::c_int as isize) as *const libc::c_void) as size_t;
    __archive_read_consume(a, 30 as libc::c_int as int64_t);
    /* Read the filename. */
    h = __archive_read_ahead(a, filename_length, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated ZIP file header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*zip_entry).zip_flags as libc::c_int & ZIP_UTF8_NAME != 0 {
        /* The filename is stored to be UTF-8. */
        if (*zip).sconv_utf8.is_null() {
            (*zip).sconv_utf8 = archive_string_conversion_from_charset(
                &mut (*a).archive,
                b"UTF-8\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*zip).sconv_utf8.is_null() {
                return -(30 as libc::c_int);
            }
        }
        sconv = (*zip).sconv_utf8
    } else if !(*zip).sconv.is_null() {
        sconv = (*zip).sconv
    } else {
        sconv = (*zip).sconv_default
    }
    if _archive_entry_copy_pathname_l(entry, h as *const libc::c_char, filename_length, sconv)
        != 0 as libc::c_int
    {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Pathname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Pathname cannot be converted from %s to current locale.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name(sconv),
        );
        ret = ARCHIVE_WARN
    }
    __archive_read_consume(a, filename_length as int64_t);
    /* Read the extra data. */
    h = __archive_read_ahead(a, extra_length, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated ZIP file header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if ARCHIVE_OK != process_extra(a, entry, h as *const libc::c_char, extra_length, zip_entry) {
        return ARCHIVE_FATAL;
    }
    __archive_read_consume(a, extra_length as int64_t);
    /* Work around a bug in Info-Zip: When reading from a pipe, it
     * stats the pipe instead of synthesizing a file entry. */
    if (*zip_entry).mode as libc::c_uint & AE_IFMT as mode_t == AE_IFIFO as mode_t {
        (*zip_entry).mode = ((*zip_entry).mode as libc::c_uint & !(AE_IFMT as mode_t)) as uint16_t;
        (*zip_entry).mode = ((*zip_entry).mode as libc::c_uint | AE_IFREG as mode_t) as uint16_t
    }
    /* If the mode is totally empty, set some sane default. */
    if (*zip_entry).mode as libc::c_int == 0 as libc::c_int {
        (*zip_entry).mode = ((*zip_entry).mode as libc::c_int | 0o664 as libc::c_int) as uint16_t
    }
    /* Windows archivers sometimes use backslash as the directory
     * separator. Normalize to slash. */
    if (*zip_entry).system as libc::c_int == 0 as libc::c_int && {
        wp = archive_entry_pathname_w(entry);
        !wp.is_null()
    } {
        if wcschr(wp, '/' as i32).is_null() && !wcschr(wp, '\\' as i32).is_null() {
            let mut i: size_t = 0;
            let mut s: archive_wstring = archive_wstring {
                s: 0 as *mut wchar_t,
                length: 0,
                buffer_length: 0,
            };
            s.s = NULL as *mut wchar_t;
            s.length = 0 as libc::c_int as size_t;
            s.buffer_length = 0 as libc::c_int as size_t;
            s.length = 0 as libc::c_int as size_t;
            archive_wstrncat(
                &mut s,
                wp,
                (if wp.is_null() {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    wcslen(wp)
                }),
            );
            i = 0 as libc::c_int as size_t;
            while i < s.length {
                if *s.s.offset(i as isize) == '\\' as i32 {
                    *s.s.offset(i as isize) = '/' as i32
                }
                i = i.wrapping_add(1)
            }
            archive_entry_copy_pathname_w(entry, s.s);
            archive_wstring_free(&mut s);
        }
    }
    /* Make sure that entries with a trailing '/' are marked as directories
     * even if the External File Attributes contains bogus values.  If this
     * is not a directory and there is no type, assume a regular file. */
    if (*zip_entry).mode as libc::c_uint & AE_IFMT as mode_t != AE_IFDIR as mode_t {
        let mut has_slash: libc::c_int = 0;
        wp = archive_entry_pathname_w(entry);
        if !wp.is_null() {
            len = wcslen(wp);
            has_slash = (len > 0 as libc::c_int as libc::c_ulong
                && *wp.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    == '/' as i32) as libc::c_int
        } else {
            cp = archive_entry_pathname(entry);
            len = if !cp.is_null() {
                strlen(cp)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            has_slash = (len > 0 as libc::c_int as libc::c_ulong
                && *cp.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '/' as i32) as libc::c_int
        }
        /* Correct file type as needed. */
        if has_slash != 0 {
            (*zip_entry).mode =
                ((*zip_entry).mode as libc::c_uint & !(AE_IFMT as mode_t)) as uint16_t;
            (*zip_entry).mode =
                ((*zip_entry).mode as libc::c_uint | AE_IFDIR as mode_t) as uint16_t;
            (*zip_entry).mode =
                ((*zip_entry).mode as libc::c_int | 0o111 as libc::c_int) as uint16_t
        } else if (*zip_entry).mode as libc::c_uint & AE_IFMT as mode_t
            == 0 as libc::c_int as libc::c_uint
        {
            (*zip_entry).mode = ((*zip_entry).mode as libc::c_uint | AE_IFREG as mode_t) as uint16_t
        }
    }
    /* Make sure directories end in '/' */
    if (*zip_entry).mode as libc::c_uint & AE_IFMT as mode_t == AE_IFDIR as mode_t {
        wp = archive_entry_pathname_w(entry);
        if !wp.is_null() {
            len = wcslen(wp);
            if len > 0 as libc::c_int as libc::c_ulong
                && *wp.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    != '/' as i32
            {
                let mut s_0: archive_wstring = archive_wstring {
                    s: 0 as *mut wchar_t,
                    length: 0,
                    buffer_length: 0,
                };
                s_0.s = NULL as *mut wchar_t;
                s_0.length = 0 as libc::c_int as size_t;
                s_0.buffer_length = 0 as libc::c_int as size_t;
                archive_wstrcat(&mut s_0, wp);
                archive_wstrappend_wchar(&mut s_0, '/' as i32);
                archive_entry_copy_pathname_w(entry, s_0.s);
                archive_wstring_free(&mut s_0);
            }
        } else {
            cp = archive_entry_pathname(entry);
            len = if !cp.is_null() {
                strlen(cp)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            if len > 0 as libc::c_int as libc::c_ulong
                && *cp.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    != '/' as i32
            {
                let mut s_1: archive_string = archive_string {
                    s: 0 as *mut libc::c_char,
                    length: 0,
                    buffer_length: 0,
                };
                s_1.s = NULL as *mut libc::c_char;
                s_1.length = 0 as libc::c_int as size_t;
                s_1.buffer_length = 0 as libc::c_int as size_t;
                archive_strcat(&mut s_1, cp as *const libc::c_void);
                archive_strappend_char(&mut s_1, '/' as i32 as libc::c_char);
                archive_entry_set_pathname(entry, s_1.s);
                archive_string_free(&mut s_1);
            }
        }
    }
    if (*zip_entry).flags as libc::c_int & LA_FROM_CENTRAL_DIRECTORY != 0 {
        /* If this came from the central dir, its size info
         * is definitive, so ignore the length-at-end flag. */
        (*zip_entry).zip_flags =
            ((*zip_entry).zip_flags as libc::c_int & !ZIP_LENGTH_AT_END) as uint16_t;
        /* If local header is missing a value, use the one from
        the central directory.  If both have it, warn about
        mismatches. */
        if (*zip_entry).crc32 == 0 as libc::c_int as libc::c_uint {
            (*zip_entry).crc32 = zip_entry_central_dir.crc32
        } else if (*zip).ignore_crc32 == 0 && (*zip_entry).crc32 != zip_entry_central_dir.crc32 {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Inconsistent CRC32 values\x00" as *const u8 as *const libc::c_char,
            );
            ret = ARCHIVE_WARN
        }
        if (*zip_entry).compressed_size == 0 as libc::c_int as libc::c_long {
            (*zip_entry).compressed_size = zip_entry_central_dir.compressed_size
        } else if (*zip_entry).compressed_size != zip_entry_central_dir.compressed_size {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Inconsistent compressed size: %jd in central directory, %jd in local header\x00"
                    as *const u8 as *const libc::c_char,
                zip_entry_central_dir.compressed_size,
                (*zip_entry).compressed_size,
            );
            ret = ARCHIVE_WARN
        }
        if (*zip_entry).uncompressed_size == 0 as libc::c_int as libc::c_long {
            (*zip_entry).uncompressed_size = zip_entry_central_dir.uncompressed_size
        } else if (*zip_entry).uncompressed_size != zip_entry_central_dir.uncompressed_size {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Inconsistent uncompressed size: %jd in central directory, %jd in local header\x00"
                    as *const u8 as *const libc::c_char,
                zip_entry_central_dir.uncompressed_size,
                (*zip_entry).uncompressed_size,
            );
            ret = ARCHIVE_WARN
        }
    }
    /* Populate some additional entry fields: */
    archive_entry_set_mode(entry, (*zip_entry).mode as mode_t);
    archive_entry_set_uid(entry, (*zip_entry).uid);
    archive_entry_set_gid(entry, (*zip_entry).gid);
    archive_entry_set_mtime(entry, (*zip_entry).mtime, 0 as libc::c_int as libc::c_long);
    archive_entry_set_ctime(entry, (*zip_entry).ctime, 0 as libc::c_int as libc::c_long);
    archive_entry_set_atime(entry, (*zip_entry).atime, 0 as libc::c_int as libc::c_long);
    if (*(*zip).entry).mode as libc::c_uint & AE_IFMT as mode_t == AE_IFLNK as mode_t {
        let mut linkname_length: size_t = 0;
        if (*zip_entry).compressed_size > (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Zip file with oversized link entry\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        linkname_length = (*zip_entry).compressed_size as size_t;
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
        p = __archive_read_ahead(a, linkname_length, NULL as *mut ssize_t) as *const libc::c_char;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Truncated Zip file\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        sconv = (*zip).sconv;
        if sconv.is_null() && (*(*zip).entry).zip_flags as libc::c_int & ZIP_UTF8_NAME != 0 {
            sconv = (*zip).sconv_utf8
        }
        if sconv.is_null() {
            sconv = (*zip).sconv_default
        }
        if _archive_entry_copy_symlink_l(entry, p, linkname_length, sconv) != 0 as libc::c_int {
            if errno != ENOMEM
                && sconv == (*zip).sconv_utf8
                && (*(*zip).entry).zip_flags as libc::c_int & ZIP_UTF8_NAME != 0
            {
                _archive_entry_copy_symlink_l(
                    entry,
                    p,
                    linkname_length,
                    NULL as *mut archive_string_conv,
                );
            }
            if errno == ENOMEM {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate memory for Symlink\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            /*
             * Since there is no character-set regulation for
             * symlink name, do not report the conversion error
             * in an automatic conversion.
             */
            if sconv != (*zip).sconv_utf8
                || (*(*zip).entry).zip_flags as libc::c_int & ZIP_UTF8_NAME == 0 as libc::c_int
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Symlink cannot be converted from %s to current locale.\x00" as *const u8
                        as *const libc::c_char,
                    archive_string_conversion_charset_name(sconv),
                );
                ret = ARCHIVE_WARN
            }
        }
        (*zip_entry).compressed_size = 0 as libc::c_int as int64_t;
        (*zip_entry).uncompressed_size = (*zip_entry).compressed_size;
        if __archive_read_consume(a, linkname_length as int64_t) < 0 as libc::c_int as libc::c_long
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Read error skipping symlink target name\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
    } else if 0 as libc::c_int == (*zip_entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END
        || (*zip_entry).uncompressed_size > 0 as libc::c_int as libc::c_long
    {
        /* Set the size only if it's meaningful. */
        archive_entry_set_size(entry, (*zip_entry).uncompressed_size);
    }
    (*zip).entry_bytes_remaining = (*zip_entry).compressed_size;
    /* If there's no body, force read_data() to return EOF immediately. */
    if 0 as libc::c_int == (*zip_entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END
        && (*zip).entry_bytes_remaining < 1 as libc::c_int as libc::c_long
    {
        (*zip).end_of_entry = 1 as libc::c_int as libc::c_char
    }
    /* Set up a more descriptive format name. */
    (*zip).format_name.length = 0 as libc::c_int as size_t;
    archive_string_sprintf(
        &mut (*zip).format_name as *mut archive_string,
        b"ZIP %d.%d (%s)\x00" as *const u8 as *const libc::c_char,
        version as libc::c_int / 10 as libc::c_int,
        version as libc::c_int % 10 as libc::c_int,
        compression_name((*(*zip).entry).compression as libc::c_int),
    );
    (*a).archive.archive_format_name = (*zip).format_name.s;
    return ret;
}
unsafe extern "C" fn check_authentication_code(
    mut a: *mut archive_read,
    mut _p: *const libc::c_void,
) -> libc::c_int {
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    /* Check authentication code. */
    if (*zip).hctx_valid != 0 {
        let mut p: *const libc::c_void = 0 as *const libc::c_void;
        let mut hmac: [uint8_t; 20] = [0; 20];
        let mut hmac_len: size_t = 20 as libc::c_int as size_t;
        let mut cmp: libc::c_int = 0;
        __archive_hmac
            .__hmac_sha1_final
            .expect("non-null function pointer")(
            &mut (*zip).hctx,
            hmac.as_mut_ptr(),
            &mut hmac_len,
        );
        if _p == NULL as *const libc::c_void {
            /* Read authentication code. */
            p = __archive_read_ahead(a, AUTH_CODE_SIZE as size_t, NULL as *mut ssize_t);
            if p == NULL as *const libc::c_void {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Truncated ZIP file data\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        } else {
            p = _p
        }
        cmp = memcmp(
            hmac.as_mut_ptr() as *const libc::c_void,
            p,
            AUTH_CODE_SIZE as libc::c_ulong,
        );
        __archive_read_consume(a, AUTH_CODE_SIZE as int64_t);
        if cmp != 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"ZIP bad Authentication code\x00" as *const u8 as *const libc::c_char,
            );
            return -(20 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
/*
 * Read "uncompressed" data.  There are three cases:
 *  1) We know the size of the data.  This is always true for the
 * seeking reader (we've examined the Central Directory already).
 *  2) ZIP_LENGTH_AT_END was set, but only the CRC was deferred.
 * Info-ZIP seems to do this; we know the size but have to grab
 * the CRC from the data descriptor afterwards.
 *  3) We're streaming and ZIP_LENGTH_AT_END was specified and
 * we have no size information.  In this case, we can do pretty
 * well by watching for the data descriptor record.  The data
 * descriptor is 16 bytes and includes a computed CRC that should
 * provide a strong check.
 *
 * TODO: Technically, the PK\007\010 signature is optional.
 * In the original spec, the data descriptor contained CRC
 * and size fields but had no leading signature.  In practice,
 * newer writers seem to provide the signature pretty consistently.
 *
 * For uncompressed data, the PK\007\010 marker seems essential
 * to be sure we've actually seen the end of the entry.
 *
 * Returns ARCHIVE_OK if successful, ARCHIVE_FATAL otherwise, sets
 * zip->end_of_entry if it consumes all of the data.
 */
unsafe extern "C" fn zip_read_data_none(
    mut a: *mut archive_read,
    mut _buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    let mut buff: *const libc::c_char = 0 as *const libc::c_char;
    let mut bytes_avail: ssize_t = 0;
    let mut r: libc::c_int = 0;
    /* UNUSED */
    zip = (*(*a).format).data as *mut zip;
    if (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END != 0 {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut grabbing_bytes: ssize_t = 24 as libc::c_int as ssize_t;
        if (*zip).hctx_valid != 0 {
            grabbing_bytes += AUTH_CODE_SIZE as libc::c_long
        }
        /* Grab at least 24 bytes. */
        buff = __archive_read_ahead(a, grabbing_bytes as size_t, &mut bytes_avail)
            as *const libc::c_char;
        if bytes_avail < grabbing_bytes {
            /* Zip archives have end-of-archive markers
            that are longer than this, so a failure to get at
            least 24 bytes really does indicate a truncated
            file. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated ZIP file data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Check for a complete PK\007\010 signature, followed
         * by the correct 4-byte CRC. */
        p = buff;
        if (*zip).hctx_valid != 0 {
            p = p.offset(AUTH_CODE_SIZE as isize)
        }
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'K' as i32
            && *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{7}' as i32
            && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{8}' as i32
            && (archive_le32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void)
                as libc::c_ulong
                == (*zip).entry_crc32
                || (*zip).ignore_crc32 as libc::c_int != 0
                || (*zip).hctx_valid as libc::c_int != 0
                    && (*(*zip).entry).aes_extra.vendor == AES_VENDOR_AE_2 as libc::c_uint)
        {
            if (*(*zip).entry).flags as libc::c_int & LA_USED_ZIP64 != 0 {
                let mut compressed: uint64_t = 0;
                let mut uncompressed: uint64_t = 0;
                (*(*zip).entry).crc32 =
                    archive_le32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void);
                compressed =
                    archive_le64dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void);
                uncompressed =
                    archive_le64dec(p.offset(16 as libc::c_int as isize) as *const libc::c_void);
                if compressed > INT64_MAX as libc::c_ulong
                    || uncompressed > INT64_MAX as libc::c_ulong
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Overflow of 64-bit file sizes\x00" as *const u8 as *const libc::c_char,
                    );
                    return ARCHIVE_FAILED;
                }
                (*(*zip).entry).compressed_size = compressed as int64_t;
                (*(*zip).entry).uncompressed_size = uncompressed as int64_t;
                (*zip).unconsumed = 24 as libc::c_int as size_t
            } else {
                (*(*zip).entry).crc32 =
                    archive_le32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void);
                (*(*zip).entry).compressed_size =
                    archive_le32dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void)
                        as int64_t;
                (*(*zip).entry).uncompressed_size =
                    archive_le32dec(p.offset(12 as libc::c_int as isize) as *const libc::c_void)
                        as int64_t;
                (*zip).unconsumed = 16 as libc::c_int as size_t
            }
            if (*zip).hctx_valid != 0 {
                r = check_authentication_code(a, buff as *const libc::c_void);
                if r != ARCHIVE_OK {
                    return r;
                }
            }
            (*zip).end_of_entry = 1 as libc::c_int as libc::c_char;
            return 0 as libc::c_int;
        }
        /* If not at EOF, ensure we consume at least one byte. */
        p = p.offset(1);
        /* Scan forward until we see where a PK\007\010 signature
         * might be. */
        /* Return bytes up until that point.  On the next call,
         * the code above will verify the data descriptor. */
        while p < buff
            .offset(bytes_avail as isize)
            .offset(-(4 as libc::c_int as isize))
        {
            if *p.offset(3 as libc::c_int as isize) as libc::c_int == 'P' as i32 {
                p = p.offset(3 as libc::c_int as isize)
            } else if *p.offset(3 as libc::c_int as isize) as libc::c_int == 'K' as i32 {
                p = p.offset(2 as libc::c_int as isize)
            } else if *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{7}' as i32 {
                p = p.offset(1 as libc::c_int as isize)
            } else if *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{8}' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{7}' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'K' as i32
                && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
            {
                if (*zip).hctx_valid != 0 {
                    p = p.offset(-(AUTH_CODE_SIZE as isize))
                }
                break;
            } else {
                p = p.offset(4 as libc::c_int as isize)
            }
        }
        bytes_avail = p.offset_from(buff) as libc::c_long
    } else {
        if (*zip).entry_bytes_remaining == 0 as libc::c_int as libc::c_long {
            (*zip).end_of_entry = 1 as libc::c_int as libc::c_char;
            if (*zip).hctx_valid != 0 {
                r = check_authentication_code(a, NULL as *const libc::c_void);
                if r != ARCHIVE_OK {
                    return r;
                }
            }
            return 0 as libc::c_int;
        }
        /* Grab a bunch of bytes. */
        buff = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail)
            as *const libc::c_char;
        if bytes_avail <= 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated ZIP file data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if bytes_avail > (*zip).entry_bytes_remaining {
            bytes_avail = (*zip).entry_bytes_remaining
        }
    }
    if (*zip).tctx_valid as libc::c_int != 0 || (*zip).cctx_valid as libc::c_int != 0 {
        let mut dec_size: size_t = bytes_avail as size_t;
        if dec_size > (*zip).decrypted_buffer_size {
            dec_size = (*zip).decrypted_buffer_size
        }
        if (*zip).tctx_valid != 0 {
            trad_enc_decrypt_update(
                &mut (*zip).tctx,
                buff as *const uint8_t,
                dec_size,
                (*zip).decrypted_buffer,
                dec_size,
            );
        } else {
            let mut dsize: size_t = dec_size;
            __archive_hmac
                .__hmac_sha1_update
                .expect("non-null function pointer")(
                &mut (*zip).hctx,
                buff as *const uint8_t,
                dec_size,
            );
            __archive_cryptor
                .decrypto_aes_ctr_update
                .expect("non-null function pointer")(
                &mut (*zip).cctx,
                buff as *const uint8_t,
                dec_size,
                (*zip).decrypted_buffer,
                &mut dsize,
            );
        }
        bytes_avail = dec_size as ssize_t;
        buff = (*zip).decrypted_buffer as *const libc::c_char
    }
    *size = bytes_avail as size_t;
    (*zip).entry_bytes_remaining -= bytes_avail;
    (*zip).entry_uncompressed_bytes_read += bytes_avail;
    (*zip).entry_compressed_bytes_read += bytes_avail;
    (*zip).unconsumed = ((*zip).unconsumed as libc::c_ulong)
        .wrapping_add(bytes_avail as libc::c_ulong) as size_t as size_t;
    *_buff = buff as *const libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn consume_optional_marker(
    mut a: *mut archive_read,
    mut zip: *mut zip,
) -> libc::c_int {
    if (*zip).end_of_entry as libc::c_int != 0
        && (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END != 0
    {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        p = __archive_read_ahead(a, 24 as libc::c_int as size_t, NULL as *mut ssize_t)
            as *const libc::c_char;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated ZIP end-of-file record\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Consume the optional PK\007\010 marker. */
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'K' as i32
            && *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{7}' as i32
            && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{8}' as i32
        {
            p = p.offset(4 as libc::c_int as isize);
            (*zip).unconsumed = 4 as libc::c_int as size_t
        }
        if (*(*zip).entry).flags as libc::c_int & LA_USED_ZIP64 != 0 {
            let mut compressed: uint64_t = 0;
            let mut uncompressed: uint64_t = 0;
            (*(*zip).entry).crc32 = archive_le32dec(p as *const libc::c_void);
            compressed =
                archive_le64dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void);
            uncompressed =
                archive_le64dec(p.offset(12 as libc::c_int as isize) as *const libc::c_void);
            if compressed > INT64_MAX as libc::c_ulong || uncompressed > INT64_MAX as libc::c_ulong
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Overflow of 64-bit file sizes\x00" as *const u8 as *const libc::c_char,
                );
                return ARCHIVE_FAILED;
            }
            (*(*zip).entry).compressed_size = compressed as int64_t;
            (*(*zip).entry).uncompressed_size = uncompressed as int64_t;
            (*zip).unconsumed = ((*zip).unconsumed as libc::c_ulong)
                .wrapping_add(20 as libc::c_int as libc::c_ulong)
                as size_t as size_t
        } else {
            (*(*zip).entry).crc32 = archive_le32dec(p as *const libc::c_void);
            (*(*zip).entry).compressed_size =
                archive_le32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void)
                    as int64_t;
            (*(*zip).entry).uncompressed_size =
                archive_le32dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void)
                    as int64_t;
            (*zip).unconsumed = ((*zip).unconsumed as libc::c_ulong)
                .wrapping_add(12 as libc::c_int as libc::c_ulong)
                as size_t as size_t
        }
    }
    return 0 as libc::c_int;
}
/* HAVE_LZMA_H && HAVE_LIBLZMA */
unsafe extern "C" fn zipx_ppmd8_init(mut a: *mut archive_read, mut zip: *mut zip) -> libc::c_int {
    let mut p: *const libc::c_void = 0 as *const libc::c_void;
    let mut val: uint32_t = 0;
    let mut order: uint32_t = 0;
    let mut mem: uint32_t = 0;
    let mut restore_method: uint32_t = 0;
    /* Remove previous decompression context if it exists. */
    if (*zip).ppmd8_valid != 0 {
        __archive_ppmd8_functions
            .Ppmd8_Free
            .expect("non-null function pointer")(&mut (*zip).ppmd8);
        (*zip).ppmd8_valid = 0 as libc::c_int as libc::c_char
    }
    /* Create a new decompression context. */
    __archive_ppmd8_functions
        .Ppmd8_Construct
        .expect("non-null function pointer")(&mut (*zip).ppmd8);
    (*zip).ppmd8_stream_failed = 0 as libc::c_int as libc::c_char;
    /* Setup function pointers required by Ppmd8 decompressor. The
     * 'ppmd_read' function will feed new bytes to the decompressor,
     * and will increment the 'zip->zipx_ppmd_read_compressed' counter. */
    (*zip).ppmd8.Stream.In = &mut (*zip).zipx_ppmd_stream;
    (*zip).zipx_ppmd_stream.a = a;
    (*zip).zipx_ppmd_stream.Read =
        Some(ppmd_read as unsafe extern "C" fn(_: *mut libc::c_void) -> Byte);
    /* Reset number of read bytes to 0. */
    (*zip).zipx_ppmd_read_compressed = 0 as libc::c_int as ssize_t;
    /* Read Ppmd8 header (2 bytes). */
    p = __archive_read_ahead(a, 2 as libc::c_int as size_t, NULL as *mut ssize_t);
    if p.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated file data in PPMd8 stream\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    __archive_read_consume(a, 2 as libc::c_int as int64_t);
    /* Decode the stream's compression parameters. */
    val = archive_le16dec(p) as uint32_t;
    order =
        (val & 15 as libc::c_int as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint);
    mem = (val >> 4 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    restore_method = val >> 12 as libc::c_int;
    if order < 2 as libc::c_int as libc::c_uint || restore_method > 2 as libc::c_int as libc::c_uint
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid parameter set in PPMd8 stream (order=%d, restore=%d)\x00" as *const u8
                as *const libc::c_char,
            order,
            restore_method,
        );
        return -(25 as libc::c_int);
    }
    /* Allocate the memory needed to properly decompress the file. */
    if __archive_ppmd8_functions
        .Ppmd8_Alloc
        .expect("non-null function pointer")(&mut (*zip).ppmd8, mem << 20 as libc::c_int)
        == 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Unable to allocate memory for PPMd8 stream: %d bytes\x00" as *const u8
                as *const libc::c_char,
            mem << 20 as libc::c_int,
        );
        return -(30 as libc::c_int);
    }
    /* Signal the cleanup function to release Ppmd8 context in the
     * cleanup phase. */
    (*zip).ppmd8_valid = 1 as libc::c_int as libc::c_char;
    /* Perform further Ppmd8 initialization. */
    if __archive_ppmd8_functions
        .Ppmd8_RangeDec_Init
        .expect("non-null function pointer")(&mut (*zip).ppmd8)
        == 0
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"PPMd8 stream range decoder initialization error\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    __archive_ppmd8_functions
        .Ppmd8_Init
        .expect("non-null function pointer")(&mut (*zip).ppmd8, order, restore_method);
    /* Allocate the buffer that will hold uncompressed data. */
    free((*zip).uncompressed_buffer as *mut libc::c_void);
    (*zip).uncompressed_buffer_size = (256 as libc::c_int * 1024 as libc::c_int) as size_t;
    (*zip).uncompressed_buffer = malloc((*zip).uncompressed_buffer_size) as *mut uint8_t;
    if (*zip).uncompressed_buffer.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"No memory for PPMd8 decompression\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    /* Ppmd8 initialization is done. */
    (*zip).decompress_init = 1 as libc::c_int as libc::c_char;
    /* We've already read 2 bytes in the output stream. Additionally,
     * Ppmd8 initialization code could read some data as well. So we
     * are advancing the stream by 2 bytes plus whatever number of
     * bytes Ppmd8 init function used. */
    (*zip).entry_compressed_bytes_read +=
        2 as libc::c_int as libc::c_long + (*zip).zipx_ppmd_read_compressed;
    return ARCHIVE_OK;
}
unsafe extern "C" fn zip_read_data_zipx_ppmd(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut ret: libc::c_int = 0;
    let mut consumed_bytes: size_t = 0 as libc::c_int as size_t;
    let mut bytes_avail: ssize_t = 0 as libc::c_int as ssize_t;
    /* UNUSED */
    /* If we're here for the first time, initialize Ppmd8 decompression
     * context first. */
    if (*zip).decompress_init == 0 {
        ret = zipx_ppmd8_init(a, zip);
        if ret != ARCHIVE_OK {
            return ret;
        }
    }
    /* Fetch for more data. We're reading 1 byte here, but libarchive
     * should prefetch more bytes. */
    __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail);
    if bytes_avail < 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated PPMd8 file body\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* This counter will be updated inside ppmd_read(), which at one
     * point will be called by Ppmd8_DecodeSymbol. */
    (*zip).zipx_ppmd_read_compressed = 0 as libc::c_int as ssize_t;
    loop
    /* Decompression loop. */
    {
        let mut sym: libc::c_int =
            __archive_ppmd8_functions
                .Ppmd8_DecodeSymbol
                .expect("non-null function pointer")(&mut (*zip).ppmd8);
        if sym < 0 as libc::c_int {
            (*zip).end_of_entry = 1 as libc::c_int as libc::c_char;
            break;
        } else {
            /* This field is set by ppmd_read() when there was no more data
             * to be read. */
            if (*zip).ppmd8_stream_failed != 0 {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Truncated PPMd8 file body\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            *(*zip).uncompressed_buffer.offset(consumed_bytes as isize) = sym as uint8_t;
            consumed_bytes = consumed_bytes.wrapping_add(1);
            if !(consumed_bytes < (*zip).uncompressed_buffer_size) {
                break;
            }
        }
    }
    /* Update pointers for libarchive. */
    *buff = (*zip).uncompressed_buffer as *const libc::c_void;
    *size = consumed_bytes;
    /* Update pointers so we can continue decompression in another call. */
    (*zip).entry_bytes_remaining -= (*zip).zipx_ppmd_read_compressed;
    (*zip).entry_compressed_bytes_read += (*zip).zipx_ppmd_read_compressed;
    (*zip).entry_uncompressed_bytes_read = ((*zip).entry_uncompressed_bytes_read as libc::c_ulong)
        .wrapping_add(consumed_bytes) as int64_t
        as int64_t;
    /* If we're at the end of stream, deinitialize Ppmd8 context. */
    if (*zip).end_of_entry != 0 {
        __archive_ppmd8_functions
            .Ppmd8_Free
            .expect("non-null function pointer")(&mut (*zip).ppmd8);
        (*zip).ppmd8_valid = 0 as libc::c_int as libc::c_char
    }
    /* Seek for optional marker, same way as in each zip entry. */
    ret = consume_optional_marker(a, zip);
    if ret != ARCHIVE_OK {
        return ret;
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn zip_deflate_init(mut a: *mut archive_read, mut zip: *mut zip) -> libc::c_int {
    let mut r: libc::c_int = 0;
    /* If we haven't yet read any data, initialize the decompressor. */
    if (*zip).decompress_init == 0 {
        if (*zip).stream_valid != 0 {
            r = inflateReset(&mut (*zip).stream)
        } else {
            r = inflateInit2_(
                &mut (*zip).stream,
                -(15 as libc::c_int),
                ZLIB_VERSION.as_ptr(),
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            )
        }
        /* Don't check for zlib header */
        if r != Z_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Can\'t initialize ZIP decompression.\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Stream structure has been set up. */
        (*zip).stream_valid = 1 as libc::c_int as libc::c_char;
        /* We've initialized decompression for this stream. */
        (*zip).decompress_init = 1 as libc::c_int as libc::c_char
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zip_read_data_deflate(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    let mut bytes_avail: ssize_t = 0;
    let mut compressed_buff: *const libc::c_void = 0 as *const libc::c_void;
    let mut sp: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    /* UNUSED */
    zip = (*(*a).format).data as *mut zip;
    /* If the buffer hasn't been allocated, allocate it now. */
    if (*zip).uncompressed_buffer.is_null() {
        (*zip).uncompressed_buffer_size = (256 as libc::c_int * 1024 as libc::c_int) as size_t;
        (*zip).uncompressed_buffer = malloc((*zip).uncompressed_buffer_size) as *mut libc::c_uchar;
        if (*zip).uncompressed_buffer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory for ZIP decompression\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    r = zip_deflate_init(a, zip);
    if r != ARCHIVE_OK {
        return r;
    }
    /*
     * Note: '1' here is a performance optimization.
     * Recall that the decompression layer returns a count of
     * available bytes; asking for more than that forces the
     * decompressor to combine reads by copying data.
     */
    sp = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail);
    compressed_buff = sp;
    if 0 as libc::c_int == (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END
        && bytes_avail > (*zip).entry_bytes_remaining
    {
        bytes_avail = (*zip).entry_bytes_remaining
    }
    if bytes_avail < 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated ZIP file body\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*zip).tctx_valid as libc::c_int != 0 || (*zip).cctx_valid as libc::c_int != 0 {
        if (*zip).decrypted_bytes_remaining < bytes_avail as size_t {
            let mut buff_remaining: size_t = (*zip)
                .decrypted_buffer
                .offset((*zip).decrypted_buffer_size as isize)
                .offset_from(
                    (*zip)
                        .decrypted_ptr
                        .offset((*zip).decrypted_bytes_remaining as isize),
                ) as libc::c_long as size_t;
            if buff_remaining > bytes_avail as size_t {
                buff_remaining = bytes_avail as size_t
            }
            if 0 as libc::c_int == (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END
                && (*zip).entry_bytes_remaining > 0 as libc::c_int as libc::c_long
            {
                if (*zip)
                    .decrypted_bytes_remaining
                    .wrapping_add(buff_remaining) as int64_t
                    > (*zip).entry_bytes_remaining
                {
                    if (*zip).entry_bytes_remaining < (*zip).decrypted_bytes_remaining as int64_t {
                        buff_remaining = 0 as libc::c_int as size_t
                    } else {
                        buff_remaining = ((*zip).entry_bytes_remaining as size_t)
                            .wrapping_sub((*zip).decrypted_bytes_remaining)
                    }
                }
            }
            if buff_remaining > 0 as libc::c_int as libc::c_ulong {
                if (*zip).tctx_valid != 0 {
                    trad_enc_decrypt_update(
                        &mut (*zip).tctx,
                        compressed_buff as *const uint8_t,
                        buff_remaining,
                        (*zip)
                            .decrypted_ptr
                            .offset((*zip).decrypted_bytes_remaining as isize),
                        buff_remaining,
                    );
                } else {
                    let mut dsize: size_t = buff_remaining;
                    __archive_cryptor
                        .decrypto_aes_ctr_update
                        .expect("non-null function pointer")(
                        &mut (*zip).cctx,
                        compressed_buff as *const uint8_t,
                        buff_remaining,
                        (*zip)
                            .decrypted_ptr
                            .offset((*zip).decrypted_bytes_remaining as isize),
                        &mut dsize,
                    );
                }
                (*zip).decrypted_bytes_remaining =
                    ((*zip).decrypted_bytes_remaining as libc::c_ulong).wrapping_add(buff_remaining)
                        as size_t as size_t
            }
        }
        bytes_avail = (*zip).decrypted_bytes_remaining as ssize_t;
        compressed_buff = (*zip).decrypted_ptr as *const libc::c_char as *const libc::c_void
    }
    /*
     * A bug in zlib.h: stream.next_in should be marked 'const'
     * but isn't (the library never alters data through the
     * next_in pointer, only reads it).  The result: this ugly
     * cast to remove 'const'.
     */
    (*zip).stream.next_in = compressed_buff as uintptr_t as *mut Bytef;
    (*zip).stream.avail_in = bytes_avail as uInt;
    (*zip).stream.total_in = 0 as libc::c_int as uLong;
    (*zip).stream.next_out = (*zip).uncompressed_buffer;
    (*zip).stream.avail_out = (*zip).uncompressed_buffer_size as uInt;
    (*zip).stream.total_out = 0 as libc::c_int as uLong;
    r = inflate(&mut (*zip).stream, 0 as libc::c_int);
    match r {
        Z_OK => {}
        Z_STREAM_END => (*zip).end_of_entry = 1 as libc::c_int as libc::c_char,
        -4 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory for ZIP decompression\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"ZIP decompression failed (%d)\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    }
    /* Consume as much as the compressor actually used. */
    bytes_avail = (*zip).stream.total_in as ssize_t;
    if (*zip).tctx_valid as libc::c_int != 0 || (*zip).cctx_valid as libc::c_int != 0 {
        (*zip).decrypted_bytes_remaining = ((*zip).decrypted_bytes_remaining as libc::c_ulong)
            .wrapping_sub(bytes_avail as libc::c_ulong)
            as size_t as size_t;
        if (*zip).decrypted_bytes_remaining == 0 as libc::c_int as libc::c_ulong {
            (*zip).decrypted_ptr = (*zip).decrypted_buffer
        } else {
            (*zip).decrypted_ptr = (*zip).decrypted_ptr.offset(bytes_avail as isize)
        }
    }
    /* Calculate compressed data as much as we used.*/
    if (*zip).hctx_valid != 0 {
        __archive_hmac
            .__hmac_sha1_update
            .expect("non-null function pointer")(
            &mut (*zip).hctx,
            sp as *const uint8_t,
            bytes_avail as size_t,
        );
    }
    __archive_read_consume(a, bytes_avail);
    (*zip).entry_bytes_remaining -= bytes_avail;
    (*zip).entry_compressed_bytes_read += bytes_avail;
    *size = (*zip).stream.total_out;
    (*zip).entry_uncompressed_bytes_read = ((*zip).entry_uncompressed_bytes_read as libc::c_ulong)
        .wrapping_add((*zip).stream.total_out) as int64_t
        as int64_t;
    *buff = (*zip).uncompressed_buffer as *const libc::c_void;
    if (*zip).end_of_entry as libc::c_int != 0 && (*zip).hctx_valid as libc::c_int != 0 {
        r = check_authentication_code(a, NULL as *const libc::c_void);
        if r != ARCHIVE_OK {
            return r;
        }
    }
    r = consume_optional_marker(a, zip);
    if r != ARCHIVE_OK {
        return r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_decryption_header(mut a: *mut archive_read) -> libc::c_int {
    let mut current_block: u64;
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut remaining_size: libc::c_uint = 0;
    let mut ts: libc::c_uint = 0;
    /*
     * Read an initialization vector data field.
     */
    p = __archive_read_ahead(a, 2 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if !p.is_null() {
        ts = (*zip).iv_size;
        (*zip).iv_size = archive_le16dec(p as *const libc::c_void) as libc::c_uint;
        __archive_read_consume(a, 2 as libc::c_int as int64_t);
        if ts < (*zip).iv_size {
            free((*zip).iv as *mut libc::c_void);
            (*zip).iv = NULL as *mut uint8_t
        }
        p = __archive_read_ahead(a, (*zip).iv_size as size_t, NULL as *mut ssize_t)
            as *const libc::c_char;
        if !p.is_null() {
            if (*zip).iv.is_null() {
                (*zip).iv = malloc((*zip).iv_size as libc::c_ulong) as *mut uint8_t;
                if (*zip).iv.is_null() {
                    current_block = 13467048591557691629;
                } else {
                    current_block = 13056961889198038528;
                }
            } else {
                current_block = 13056961889198038528;
            }
            match current_block {
                13056961889198038528 => {
                    memcpy(
                        (*zip).iv as *mut libc::c_void,
                        p as *const libc::c_void,
                        (*zip).iv_size as libc::c_ulong,
                    );
                    __archive_read_consume(a, (*zip).iv_size as int64_t);
                    /*
                     * Read a size of remaining decryption header field.
                     */
                    p = __archive_read_ahead(a, 14 as libc::c_int as size_t, NULL as *mut ssize_t)
                        as *const libc::c_char;
                    if p.is_null() {
                        current_block = 9565569445570550704;
                    } else {
                        remaining_size = archive_le32dec(p as *const libc::c_void);
                        if remaining_size < 16 as libc::c_int as libc::c_uint
                            || remaining_size
                                > ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_uint
                        {
                            current_block = 4209676304665092873;
                        } else {
                            /* Check if format version is supported. */
                            if archive_le16dec(
                                p.offset(4 as libc::c_int as isize) as *const libc::c_void
                            ) as libc::c_int
                                != 3 as libc::c_int
                            {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_FILE_FORMAT,
                                    b"Unsupported encryption format version: %u\x00" as *const u8
                                        as *const libc::c_char,
                                    archive_le16dec(
                                        p.offset(4 as libc::c_int as isize) as *const libc::c_void
                                    ) as libc::c_int,
                                );
                                return -(25 as libc::c_int);
                            }
                            /*
                             * Read an encryption algorithm field.
                             */
                            (*zip).alg_id = archive_le16dec(
                                p.offset(6 as libc::c_int as isize) as *const libc::c_void
                            ) as libc::c_uint;
                            let mut current_block_20: u64;
                            match (*zip).alg_id {
                                26113 => {
                                    current_block_20 = 11636175345244025579;
                                }
                                26114 => {
                                    /* RC2 */
                                    current_block_20 = 15191369955531545467;
                                }
                                26115 => {
                                    current_block_20 = 15191369955531545467;
                                }
                                26121 => {
                                    current_block_20 = 15679700284600549218;
                                }
                                26126 => {
                                    current_block_20 = 1890217500068296839;
                                }
                                26127 => {
                                    current_block_20 = 1407665506148312647;
                                }
                                26128 => {
                                    current_block_20 = 7642571721358449150;
                                }
                                26370 => {
                                    current_block_20 = 2779046719703530699;
                                }
                                26400 => {
                                    current_block_20 = 8217249381186882617;
                                }
                                26401 => {
                                    current_block_20 = 506662057365320143;
                                }
                                26625 => {
                                    current_block_20 = 17785059701548998426;
                                }
                                _ => {
                                    archive_set_error(
                                        &mut (*a).archive as *mut archive,
                                        ARCHIVE_ERRNO_FILE_FORMAT,
                                        b"Unknown encryption algorithm: %u\x00" as *const u8
                                            as *const libc::c_char,
                                        (*zip).alg_id,
                                    );
                                    return -(25 as libc::c_int);
                                }
                            }
                            match current_block_20 {
                                15191369955531545467 =>
                                /* 3DES 168 */
                                {
                                    current_block_20 = 15679700284600549218;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                15679700284600549218 =>
                                /* 3DES 112 */
                                {
                                    current_block_20 = 1890217500068296839;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                1890217500068296839 =>
                                /* AES 128 */
                                {
                                    current_block_20 = 1407665506148312647;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                1407665506148312647 =>
                                /* AES 192 */
                                {
                                    current_block_20 = 7642571721358449150;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                7642571721358449150 =>
                                /* AES 256 */
                                {
                                    current_block_20 = 2779046719703530699;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                2779046719703530699 =>
                                /* RC2 (version >= 5.2) */
                                {
                                    current_block_20 = 8217249381186882617;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                8217249381186882617 =>
                                /* Blowfish */
                                {
                                    current_block_20 = 506662057365320143;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                506662057365320143 =>
                                /* Twofish */
                                {
                                    current_block_20 = 17785059701548998426;
                                }
                                _ => {}
                            }
                            match current_block_20 {
                                17785059701548998426 =>
                                    /* RC4 */
                                /* Supported encryption algorithm. */
                                    {}
                                _ => {}
                            }
                            /*
                             * Read a bit length field.
                             */
                            (*zip).bit_len = archive_le16dec(
                                p.offset(8 as libc::c_int as isize) as *const libc::c_void
                            ) as libc::c_uint;
                            /*
                             * Read a flags field.
                             */
                            (*zip).flags = archive_le16dec(
                                p.offset(10 as libc::c_int as isize) as *const libc::c_void
                            ) as libc::c_uint;
                            let mut current_block_25: u64;
                            match (*zip).flags & 0xf000 as libc::c_int as libc::c_uint {
                                1 => {
                                    current_block_25 = 8180496224585318153;
                                }
                                2 => {
                                    /* Certificates only. */
                                    current_block_25 = 47308297132330914;
                                }
                                3 => {
                                    current_block_25 = 47308297132330914;
                                }
                                _ => {
                                    archive_set_error(
                                        &mut (*a).archive as *mut archive,
                                        ARCHIVE_ERRNO_FILE_FORMAT,
                                        b"Unknown encryption flag: %u\x00" as *const u8
                                            as *const libc::c_char,
                                        (*zip).flags,
                                    );
                                    return -(25 as libc::c_int);
                                }
                            }
                            match current_block_25 {
                                47308297132330914 =>
                                    /* Password or certificate required to decrypt. */
                                    {}
                                _ => {}
                            }
                            if (*zip).flags & 0xf000 as libc::c_int as libc::c_uint
                                == 0 as libc::c_int as libc::c_uint
                                || (*zip).flags & 0xf000 as libc::c_int as libc::c_uint
                                    == 0x4000 as libc::c_int as libc::c_uint
                            {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_FILE_FORMAT,
                                    b"Unknown encryption flag: %u\x00" as *const u8
                                        as *const libc::c_char,
                                    (*zip).flags,
                                );
                                return -(25 as libc::c_int);
                            }
                            /*
                             * Read an encrypted random data field.
                             */
                            ts = (*zip).erd_size;
                            (*zip).erd_size = archive_le16dec(
                                p.offset(12 as libc::c_int as isize) as *const libc::c_void
                            ) as libc::c_uint;
                            __archive_read_consume(a, 14 as libc::c_int as int64_t);
                            if (*zip).erd_size & 0xf as libc::c_int as libc::c_uint
                                != 0 as libc::c_int as libc::c_uint
                                || (*zip)
                                    .erd_size
                                    .wrapping_add(16 as libc::c_int as libc::c_uint)
                                    > remaining_size
                                || (*zip)
                                    .erd_size
                                    .wrapping_add(16 as libc::c_int as libc::c_uint)
                                    < (*zip).erd_size
                            {
                                current_block = 4209676304665092873;
                            } else {
                                if ts < (*zip).erd_size {
                                    free((*zip).erd as *mut libc::c_void);
                                    (*zip).erd = NULL as *mut uint8_t
                                }
                                p = __archive_read_ahead(
                                    a,
                                    (*zip).erd_size as size_t,
                                    NULL as *mut ssize_t,
                                ) as *const libc::c_char;
                                if p.is_null() {
                                    current_block = 9565569445570550704;
                                } else {
                                    if (*zip).erd.is_null() {
                                        (*zip).erd = malloc((*zip).erd_size as libc::c_ulong)
                                            as *mut uint8_t;
                                        if (*zip).erd.is_null() {
                                            current_block = 13467048591557691629;
                                        } else {
                                            current_block = 8151474771948790331;
                                        }
                                    } else {
                                        current_block = 8151474771948790331;
                                    }
                                    match current_block {
                                        13467048591557691629 => {}
                                        _ => {
                                            memcpy(
                                                (*zip).erd as *mut libc::c_void,
                                                p as *const libc::c_void,
                                                (*zip).erd_size as libc::c_ulong,
                                            );
                                            __archive_read_consume(a, (*zip).erd_size as int64_t);
                                            /*
                                             * Read a reserved data field.
                                             */
                                            p = __archive_read_ahead(
                                                a,
                                                4 as libc::c_int as size_t,
                                                NULL as *mut ssize_t,
                                            )
                                                as *const libc::c_char;
                                            if p.is_null() {
                                                current_block = 9565569445570550704;
                                            } else if archive_le32dec(p as *const libc::c_void)
                                                != 0 as libc::c_int as libc::c_uint
                                            {
                                                current_block = 4209676304665092873;
                                            } else {
                                                __archive_read_consume(
                                                    a,
                                                    4 as libc::c_int as int64_t,
                                                );
                                                /* Reserved data size should be zero. */
                                                /*
                                                 * Read a password validation data field.
                                                 */
                                                p = __archive_read_ahead(
                                                    a,
                                                    2 as libc::c_int as size_t,
                                                    NULL as *mut ssize_t,
                                                )
                                                    as *const libc::c_char;
                                                if p.is_null() {
                                                    current_block = 9565569445570550704;
                                                } else {
                                                    ts = (*zip).v_size;
                                                    (*zip).v_size =
                                                        archive_le16dec(p as *const libc::c_void)
                                                            as libc::c_uint;
                                                    __archive_read_consume(
                                                        a,
                                                        2 as libc::c_int as int64_t,
                                                    );
                                                    if (*zip).v_size
                                                        & 0xf as libc::c_int as libc::c_uint
                                                        != 0 as libc::c_int as libc::c_uint
                                                        || (*zip)
                                                            .erd_size
                                                            .wrapping_add((*zip).v_size)
                                                            .wrapping_add(
                                                                16 as libc::c_int as libc::c_uint,
                                                            )
                                                            > remaining_size
                                                        || (*zip)
                                                            .erd_size
                                                            .wrapping_add((*zip).v_size)
                                                            .wrapping_add(
                                                                16 as libc::c_int as libc::c_uint,
                                                            )
                                                            < (*zip)
                                                                .erd_size
                                                                .wrapping_add((*zip).v_size)
                                                    {
                                                        current_block = 4209676304665092873;
                                                    } else {
                                                        if ts < (*zip).v_size {
                                                            free(
                                                                (*zip).v_data as *mut libc::c_void,
                                                            );
                                                            (*zip).v_data = NULL as *mut uint8_t
                                                        }
                                                        p = __archive_read_ahead(
                                                            a,
                                                            (*zip).v_size as size_t,
                                                            NULL as *mut ssize_t,
                                                        )
                                                            as *const libc::c_char;
                                                        if p.is_null() {
                                                            current_block = 9565569445570550704;
                                                        } else {
                                                            if (*zip).v_data.is_null() {
                                                                (*zip).v_data = malloc(
                                                                    (*zip).v_size as libc::c_ulong,
                                                                )
                                                                    as *mut uint8_t;
                                                                if (*zip).v_data.is_null() {
                                                                    current_block =
                                                                        13467048591557691629;
                                                                } else {
                                                                    current_block =
                                                                        9437375157805982253;
                                                                }
                                                            } else {
                                                                current_block = 9437375157805982253;
                                                            }
                                                            match current_block {
                                                                13467048591557691629 => {}
                                                                _ => {
                                                                    memcpy(
                                                                        (*zip).v_data
                                                                            as *mut libc::c_void,
                                                                        p as *const libc::c_void,
                                                                        (*zip).v_size
                                                                            as libc::c_ulong,
                                                                    );
                                                                    __archive_read_consume(
                                                                        a,
                                                                        (*zip).v_size as int64_t,
                                                                    );
                                                                    p = __archive_read_ahead(
                                                                        a,
                                                                        4 as libc::c_int as size_t,
                                                                        NULL as *mut ssize_t,
                                                                    )
                                                                        as *const libc::c_char;
                                                                    if p.is_null() {
                                                                        current_block =
                                                                            9565569445570550704;
                                                                    } else {
                                                                        (*zip).v_crc32
                                                                            =
                                                                            archive_le32dec(p
                                                                                                as
                                                                                                *const libc::c_void);
                                                                        __archive_read_consume(
                                                                            a,
                                                                            4 as libc::c_int
                                                                                as int64_t,
                                                                        );
                                                                        /*return (ARCHIVE_OK);
                                                                         * This is not fully implemented yet.*/
                                                                        archive_set_error(&mut (*a).archive
                                                                                              as
                                                                                              *mut archive,
                                                                                          ARCHIVE_ERRNO_FILE_FORMAT,
                                                                                          b"Encrypted file is unsupported\x00"
                                                                                              as
                                                                                              *const u8
                                                                                              as
                                                                                              *const libc::c_char);
                                                                        return -(25 as libc::c_int);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            9565569445570550704 => {}
                            13467048591557691629 => {}
                            _ => {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_FILE_FORMAT,
                                    b"Corrupted ZIP file data\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                return -(30 as libc::c_int);
                            }
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                9565569445570550704 => {}
                _ => {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"No memory for ZIP decryption\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
            }
        }
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Truncated ZIP file data\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn zip_alloc_decryption_buffer(mut a: *mut archive_read) -> libc::c_int {
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut bs: size_t = (256 as libc::c_int * 1024 as libc::c_int) as size_t;
    if (*zip).decrypted_buffer.is_null() {
        (*zip).decrypted_buffer_size = bs;
        (*zip).decrypted_buffer = malloc(bs) as *mut libc::c_uchar;
        if (*zip).decrypted_buffer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory for ZIP decryption\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    (*zip).decrypted_ptr = (*zip).decrypted_buffer;
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_traditional_PKWARE_decryption(mut a: *mut archive_read) -> libc::c_int {
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut p: *const libc::c_void = 0 as *const libc::c_void;
    let mut retry: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if (*zip).tctx_valid != 0 {
        return 0 as libc::c_int;
    }
    /*
      Read the 12 bytes encryption header stored at
      the start of the data area.
    */
    if 0 as libc::c_int == (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END
        && (*zip).entry_bytes_remaining < ENC_HEADER_SIZE as libc::c_long
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated Zip encrypted body: only %jd bytes available\x00" as *const u8
                as *const libc::c_char,
            (*zip).entry_bytes_remaining,
        );
        return -(30 as libc::c_int);
    }
    p = __archive_read_ahead(a, ENC_HEADER_SIZE as size_t, NULL as *mut ssize_t);
    if p == NULL as *const libc::c_void {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated ZIP file data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    retry = 0 as libc::c_int;
    loop {
        let mut passphrase: *const libc::c_char = 0 as *const libc::c_char;
        let mut crcchk: uint8_t = 0;
        passphrase = __archive_read_next_passphrase(a);
        if passphrase.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                if retry > 0 as libc::c_int {
                    b"Incorrect passphrase\x00" as *const u8 as *const libc::c_char
                } else {
                    b"Passphrase required for this entry\x00" as *const u8 as *const libc::c_char
                },
            );
            return -(25 as libc::c_int);
        }
        /*
         * Initialize ctx for Traditional PKWARE Decryption.
         */
        r = trad_enc_init(
            &mut (*zip).tctx,
            passphrase,
            strlen(passphrase),
            p as *const uint8_t,
            ENC_HEADER_SIZE as size_t,
            &mut crcchk,
        ); /* The passphrase is OK. */
        if r == 0 as libc::c_int && crcchk as libc::c_int == (*(*zip).entry).decdat as libc::c_int {
            break;
        }
        if retry > 10000 as libc::c_int {
            /* Avoid infinity loop. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Too many incorrect passphrases\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        retry += 1
    }
    __archive_read_consume(a, ENC_HEADER_SIZE as int64_t);
    (*zip).tctx_valid = 1 as libc::c_int as libc::c_char;
    if 0 as libc::c_int == (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END {
        (*zip).entry_bytes_remaining -= ENC_HEADER_SIZE as libc::c_long
    }
    /*zip->entry_uncompressed_bytes_read += ENC_HEADER_SIZE;*/
    (*zip).entry_compressed_bytes_read += ENC_HEADER_SIZE as libc::c_long;
    (*zip).decrypted_bytes_remaining = 0 as libc::c_int as size_t;
    return zip_alloc_decryption_buffer(a);
}
pub const ENC_HEADER_SIZE: libc::c_int = 12 as libc::c_int;
unsafe extern "C" fn init_WinZip_AES_decryption(mut a: *mut archive_read) -> libc::c_int {
    let mut current_block: u64;
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut p: *const libc::c_void = 0 as *const libc::c_void;
    let mut pv: *const uint8_t = 0 as *const uint8_t;
    let mut key_len: size_t = 0;
    let mut salt_len: size_t = 0;
    let mut derived_key: [uint8_t; 66] = [0; 66];
    let mut retry: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if (*zip).cctx_valid as libc::c_int != 0 || (*zip).hctx_valid as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    match (*(*zip).entry).aes_extra.strength {
        1 => {
            salt_len = 8 as libc::c_int as size_t;
            key_len = 16 as libc::c_int as size_t;
            current_block = 8236137900636309791;
        }
        2 => {
            salt_len = 12 as libc::c_int as size_t;
            key_len = 24 as libc::c_int as size_t;
            current_block = 8236137900636309791;
        }
        3 => {
            salt_len = 16 as libc::c_int as size_t;
            key_len = 32 as libc::c_int as size_t;
            current_block = 8236137900636309791;
        }
        _ => {
            current_block = 12307823087670350305;
        }
    }
    match current_block {
        8236137900636309791 => {
            p = __archive_read_ahead(
                a,
                salt_len.wrapping_add(2 as libc::c_int as libc::c_ulong),
                NULL as *mut ssize_t,
            );
            if p == NULL as *const libc::c_void {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Truncated ZIP file data\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            } else {
                retry = 0 as libc::c_int;
                loop {
                    let mut passphrase: *const libc::c_char = 0 as *const libc::c_char;
                    passphrase = __archive_read_next_passphrase(a);
                    if passphrase.is_null() {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            if retry > 0 as libc::c_int {
                                b"Incorrect passphrase\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"Passphrase required for this entry\x00" as *const u8
                                    as *const libc::c_char
                            },
                        );
                        return -(25 as libc::c_int);
                    }
                    memset(
                        derived_key.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[uint8_t; 66]>() as libc::c_ulong,
                    );
                    r = __archive_cryptor
                        .pbkdf2sha1
                        .expect("non-null function pointer")(
                        passphrase,
                        strlen(passphrase),
                        p as *const uint8_t,
                        salt_len,
                        1000 as libc::c_int as libc::c_uint,
                        derived_key.as_mut_ptr(),
                        key_len
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    );
                    if r != 0 as libc::c_int {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"Decryption is unsupported due to lack of crypto library\x00"
                                as *const u8 as *const libc::c_char,
                        );
                        return -(25 as libc::c_int);
                    }
                    /* Check password verification value. */
                    pv = (p as *const uint8_t).offset(salt_len as isize); /* The passphrase is OK. */
                    if derived_key[key_len.wrapping_mul(2 as libc::c_int as libc::c_ulong) as usize]
                        as libc::c_int
                        == *pv.offset(0 as libc::c_int as isize) as libc::c_int
                        && derived_key[key_len
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as usize] as libc::c_int
                            == *pv.offset(1 as libc::c_int as isize) as libc::c_int
                    {
                        break;
                    }
                    if retry > 10000 as libc::c_int {
                        /* Avoid infinity loop. */
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"Too many incorrect passphrases\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return -(25 as libc::c_int);
                    }
                    retry += 1
                }
                r = __archive_cryptor
                    .decrypto_aes_ctr_init
                    .expect("non-null function pointer")(
                    &mut (*zip).cctx,
                    derived_key.as_mut_ptr(),
                    key_len,
                );
                if r != 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Decryption is unsupported due to lack of crypto library\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
                r = __archive_hmac
                    .__hmac_sha1_init
                    .expect("non-null function pointer")(
                    &mut (*zip).hctx,
                    derived_key.as_mut_ptr().offset(key_len as isize),
                    key_len,
                );
                if r != 0 as libc::c_int {
                    __archive_cryptor
                        .decrypto_aes_ctr_release
                        .expect("non-null function pointer")(&mut (*zip).cctx);
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Failed to initialize HMAC-SHA1\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
                (*zip).hctx_valid = 1 as libc::c_int as libc::c_char;
                (*zip).cctx_valid = (*zip).hctx_valid;
                __archive_read_consume(
                    a,
                    salt_len.wrapping_add(2 as libc::c_int as libc::c_ulong) as int64_t,
                );
                (*zip).entry_bytes_remaining = ((*zip).entry_bytes_remaining as libc::c_ulong)
                    .wrapping_sub(
                        salt_len
                            .wrapping_add(2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(AUTH_CODE_SIZE as libc::c_ulong),
                    ) as int64_t as int64_t;
                if !(0 as libc::c_int
                    == (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END
                    && (*zip).entry_bytes_remaining < 0 as libc::c_int as libc::c_long)
                {
                    (*zip).entry_compressed_bytes_read =
                        ((*zip).entry_compressed_bytes_read as libc::c_ulong).wrapping_add(
                            salt_len
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(AUTH_CODE_SIZE as libc::c_ulong),
                        ) as int64_t as int64_t;
                    (*zip).decrypted_bytes_remaining = 0 as libc::c_int as size_t;
                    (*(*zip).entry).compression = (*(*zip).entry).aes_extra.compression;
                    return zip_alloc_decryption_buffer(a);
                }
            }
        }
        _ => {}
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Corrupted ZIP file data\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn archive_read_format_zip_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    if (*zip).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*zip).has_encrypted_entries = 0 as libc::c_int
    }
    *offset = (*zip).entry_uncompressed_bytes_read;
    *size = 0 as libc::c_int as size_t;
    *buff = NULL as *const libc::c_void;
    /* If we hit end-of-entry last time, return ARCHIVE_EOF. */
    if (*zip).end_of_entry != 0 {
        return 1 as libc::c_int;
    }
    /* Return EOF immediately if this is a non-regular file. */
    if AE_IFREG as mode_t != (*(*zip).entry).mode as libc::c_uint & AE_IFMT as mode_t {
        return 1 as libc::c_int;
    }
    __archive_read_consume(a, (*zip).unconsumed as int64_t);
    (*zip).unconsumed = 0 as libc::c_int as size_t;
    if (*zip).init_decryption != 0 {
        (*zip).has_encrypted_entries = 1 as libc::c_int;
        if (*(*zip).entry).zip_flags as libc::c_int & ZIP_STRONG_ENCRYPTED != 0 {
            r = read_decryption_header(a)
        } else if (*(*zip).entry).compression as libc::c_int == WINZIP_AES_ENCRYPTION {
            r = init_WinZip_AES_decryption(a)
        } else {
            r = init_traditional_PKWARE_decryption(a)
        }
        if r != ARCHIVE_OK {
            return r;
        }
        (*zip).init_decryption = 0 as libc::c_int as libc::c_char
    }
    match (*(*zip).entry).compression as libc::c_int {
        0 => {
            /* No compression. */
            r = zip_read_data_none(a, buff, size, offset)
        }
        98 => {
            /* PPMd support is built-in, so we don't need any #if guards. */
            /* ZIPx PPMd compression. */
            r = zip_read_data_zipx_ppmd(a, buff, size, offset)
        }
        8 => {
            /* Deflate compression. */
            r = zip_read_data_deflate(a, buff, size, offset)
        }
        _ => {
            /* Unsupported compression. */
            /* Return a warning. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unsupported ZIP compression method (%d: %s)\x00" as *const u8
                    as *const libc::c_char,
                (*(*zip).entry).compression as libc::c_int,
                compression_name((*(*zip).entry).compression as libc::c_int),
            );
            /* We can't decompress this entry, but we will
             * be able to skip() it and try the next entry. */
            return -(25 as libc::c_int);
        }
    }
    if r != ARCHIVE_OK {
        return r;
    }
    /* Update checksum */
    if *size != 0 {
        (*zip).entry_crc32 = (*zip).crc32func.expect("non-null function pointer")(
            (*zip).entry_crc32,
            *buff,
            *size as libc::c_uint as size_t,
        )
    }
    /* If we hit the end, swallow any end-of-data marker. */
    if (*zip).end_of_entry != 0 {
        /* Check file size, CRC against these values. */
        if (*(*zip).entry).compressed_size != (*zip).entry_compressed_bytes_read {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"ZIP compressed data is wrong size (read %jd, expected %jd)\x00" as *const u8
                    as *const libc::c_char,
                (*zip).entry_compressed_bytes_read,
                (*(*zip).entry).compressed_size,
            );
            return -(20 as libc::c_int);
        }
        /* Size field only stores the lower 32 bits of the actual
         * size. */
        if (*(*zip).entry).uncompressed_size & UINT32_MAX as libc::c_long
            != (*zip).entry_uncompressed_bytes_read & UINT32_MAX as libc::c_long
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"ZIP uncompressed data is wrong size (read %jd, expected %jd)\n\x00" as *const u8
                    as *const libc::c_char,
                (*zip).entry_uncompressed_bytes_read,
                (*(*zip).entry).uncompressed_size,
            );
            return -(20 as libc::c_int);
        }
        /* Check computed CRC against header */
        if ((*zip).hctx_valid == 0
            || (*(*zip).entry).aes_extra.vendor != AES_VENDOR_AE_2 as libc::c_uint)
            && (*(*zip).entry).crc32 as libc::c_ulong != (*zip).entry_crc32
            && (*zip).ignore_crc32 == 0
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"ZIP bad CRC: 0x%lx should be 0x%lx\x00" as *const u8 as *const libc::c_char,
                (*zip).entry_crc32,
                (*(*zip).entry).crc32 as libc::c_ulong,
            );
            return -(20 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_zip_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    let mut zip_entry: *mut zip_entry = 0 as *mut zip_entry;
    let mut next_zip_entry: *mut zip_entry = 0 as *mut zip_entry;
    zip = (*(*a).format).data as *mut zip;
    if (*zip).stream_valid != 0 {
        inflateEnd(&mut (*zip).stream);
    }
    free((*zip).uncompressed_buffer as *mut libc::c_void);
    if (*zip).ppmd8_valid != 0 {
        __archive_ppmd8_functions
            .Ppmd8_Free
            .expect("non-null function pointer")(&mut (*zip).ppmd8);
    }
    if !(*zip).zip_entries.is_null() {
        zip_entry = (*zip).zip_entries;
        while !zip_entry.is_null() {
            next_zip_entry = (*zip_entry).next;
            archive_string_free(&mut (*zip_entry).rsrcname);
            free(zip_entry as *mut libc::c_void);
            zip_entry = next_zip_entry
        }
    }
    free((*zip).decrypted_buffer as *mut libc::c_void);
    if (*zip).cctx_valid != 0 {
        __archive_cryptor
            .decrypto_aes_ctr_release
            .expect("non-null function pointer")(&mut (*zip).cctx);
    }
    if (*zip).hctx_valid != 0 {
        __archive_hmac
            .__hmac_sha1_cleanup
            .expect("non-null function pointer")(&mut (*zip).hctx);
    }
    free((*zip).iv as *mut libc::c_void);
    free((*zip).erd as *mut libc::c_void);
    free((*zip).v_data as *mut libc::c_void);
    archive_string_free(&mut (*zip).format_name);
    free(zip as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_zip_has_encrypted_entries(
    mut _a: *mut archive_read,
) -> libc::c_int {
    if !_a.is_null() && !(*_a).format.is_null() {
        let mut zip: *mut zip = (*(*_a).format).data as *mut zip;
        if !zip.is_null() {
            return (*zip).has_encrypted_entries;
        }
    }
    return ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
}
unsafe extern "C" fn archive_read_format_zip_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    zip = (*(*a).format).data as *mut zip;
    if strcmp(key, b"compat-2x\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /* Handle filenames as libarchive 2.x */
        (*zip).init_default_conversion = if !val.is_null() {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        return 0 as libc::c_int;
    } else {
        if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            if val.is_null()
                || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"zip: hdrcharset option needs a character-set name\x00" as *const u8
                        as *const libc::c_char,
                );
            } else {
                (*zip).sconv = archive_string_conversion_from_charset(
                    &mut (*a).archive,
                    val,
                    0 as libc::c_int,
                );
                if !(*zip).sconv.is_null() {
                    if strcmp(val, b"UTF-8\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        (*zip).sconv_utf8 = (*zip).sconv
                    }
                    ret = ARCHIVE_OK
                } else {
                    ret = ARCHIVE_FATAL
                }
            }
            return ret;
        } else {
            if strcmp(key, b"ignorecrc32\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                /* Mostly useful for testing. */
                if val.is_null()
                    || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                {
                    (*zip).crc32func = Some(
                        real_crc32
                            as unsafe extern "C" fn(
                                _: libc::c_ulong,
                                _: *const libc::c_void,
                                _: size_t,
                            ) -> libc::c_ulong,
                    );
                    (*zip).ignore_crc32 = 0 as libc::c_int as libc::c_char
                } else {
                    (*zip).crc32func = Some(
                        fake_crc32
                            as unsafe extern "C" fn(
                                _: libc::c_ulong,
                                _: *const libc::c_void,
                                _: size_t,
                            ) -> libc::c_ulong,
                    );
                    (*zip).ignore_crc32 = 1 as libc::c_int as libc::c_char
                }
                return 0 as libc::c_int;
            } else {
                if strcmp(key, b"mac-ext\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*zip).process_mac_extensions = (!val.is_null()
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
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_zip(mut a: *mut archive) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = archive_read_support_format_zip_streamable(a);
    if r != ARCHIVE_OK {
        return r;
    }
    return archive_read_support_format_zip_seekable(a);
}
/* ------------------------------------------------------------------------ */
/*
 * Streaming-mode support
 */
unsafe extern "C" fn archive_read_support_format_zip_capabilities_streamable(
    mut a: *mut archive_read,
) -> libc::c_int {
    /* UNUSED */
    return ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA | ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA;
}
unsafe extern "C" fn archive_read_format_zip_streamable_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* UNUSED */
    p = __archive_read_ahead(a, 4 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    /*
     * Bid of 29 here comes from:
     *  + 16 bits for "PK",
     *  + next 16-bit field has 6 options so contributes
     *    about 16 - log_2(6) ~= 16 - 2.6 ~= 13 bits
     *
     * So we've effectively verified ~29 total bits of check data.
     */
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'K' as i32
    {
        if *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{1}' as i32
            && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{2}' as i32
            || *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{3}' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{4}' as i32
            || *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{5}' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{6}' as i32
            || *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{6}' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{6}' as i32
            || *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{7}' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{8}' as i32
            || *p.offset(2 as libc::c_int as isize) as libc::c_int == '0' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '0' as i32
        {
            return 29 as libc::c_int;
        }
    }
    /* TODO: It's worth looking ahead a little bit for a valid
     * PK signature.  In particular, that would make it possible
     * to read some UUEncoded SFX files or SFX files coming from
     * a network socket. */
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_zip_streamable_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    (*a).archive.archive_format = ARCHIVE_FORMAT_ZIP;
    if (*a).archive.archive_format_name.is_null() {
        (*a).archive.archive_format_name = b"ZIP\x00" as *const u8 as *const libc::c_char
    }
    zip = (*(*a).format).data as *mut zip;
    /*
     * It should be sufficient to call archive_read_next_header() for
     * a reader to determine if an entry is encrypted or not. If the
     * encryption of an entry is only detectable when calling
     * archive_read_data(), so be it. We'll do the same check there
     * as well.
     */
    if (*zip).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*zip).has_encrypted_entries = 0 as libc::c_int
    }
    /* Make sure we have a zip_entry structure to use. */
    if (*zip).zip_entries.is_null() {
        (*zip).zip_entries =
            malloc(::std::mem::size_of::<zip_entry>() as libc::c_ulong) as *mut zip_entry;
        if (*zip).zip_entries.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out  of memory\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
    }
    (*zip).entry = (*zip).zip_entries;
    memset(
        (*zip).entry as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zip_entry>() as libc::c_ulong,
    );
    if (*zip).cctx_valid != 0 {
        __archive_cryptor
            .decrypto_aes_ctr_release
            .expect("non-null function pointer")(&mut (*zip).cctx);
    }
    if (*zip).hctx_valid != 0 {
        __archive_hmac
            .__hmac_sha1_cleanup
            .expect("non-null function pointer")(&mut (*zip).hctx);
    }
    (*zip).hctx_valid = 0 as libc::c_int as libc::c_char;
    (*zip).cctx_valid = (*zip).hctx_valid;
    (*zip).tctx_valid = (*zip).cctx_valid;
    __archive_read_reset_passphrase(a);
    /* Search ahead for the next local file header. */
    __archive_read_consume(a, (*zip).unconsumed as int64_t);
    (*zip).unconsumed = 0 as libc::c_int as size_t;
    loop {
        let mut skipped: int64_t = 0 as libc::c_int as int64_t;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut bytes: ssize_t = 0;
        p = __archive_read_ahead(a, 4 as libc::c_int as size_t, &mut bytes) as *const libc::c_char;
        if p.is_null() {
            return -(30 as libc::c_int);
        }
        end = p.offset(bytes as isize);
        while p.offset(4 as libc::c_int as isize) <= end {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'K' as i32
            {
                if *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{3}' as i32
                    && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{4}' as i32
                {
                    /* Regular file entry. */
                    __archive_read_consume(a, skipped);
                    return zip_read_local_file_header(a, entry, zip);
                }
                /*
                 * TODO: We cannot restore permissions
                 * based only on the local file headers.
                 * Consider scanning the central
                 * directory and returning additional
                 * entries for at least directories.
                 * This would allow us to properly set
                 * directory permissions.
                 *
                 * This won't help us fix symlinks
                 * and may not help with regular file
                 * permissions, either.  <sigh>
                 */
                if *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{1}' as i32
                    && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{2}' as i32
                {
                    return 1 as libc::c_int;
                }
                /* End of central directory?  Must be an
                 * empty archive. */
                if *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{5}' as i32
                    && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{6}' as i32
                    || *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{6}' as i32
                        && *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{6}' as i32
                {
                    return 1 as libc::c_int;
                }
            }
            p = p.offset(1);
            skipped += 1
        }
        __archive_read_consume(a, skipped);
    }
}
unsafe extern "C" fn archive_read_format_zip_read_data_skip_streamable(
    mut a: *mut archive_read,
) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    let mut bytes_skipped: int64_t = 0;
    zip = (*(*a).format).data as *mut zip;
    bytes_skipped = __archive_read_consume(a, (*zip).unconsumed as int64_t);
    (*zip).unconsumed = 0 as libc::c_int as size_t;
    if bytes_skipped < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    /* If we've already read to end of data, we're done. */
    if (*zip).end_of_entry != 0 {
        return 0 as libc::c_int;
    }
    /* So we know we're streaming... */
    if 0 as libc::c_int == (*(*zip).entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END
        || (*(*zip).entry).compressed_size > 0 as libc::c_int as libc::c_long
    {
        /* We know the compressed length, so we can just skip. */
        bytes_skipped = __archive_read_consume(a, (*zip).entry_bytes_remaining);
        if bytes_skipped < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if (*zip).init_decryption != 0 {
        let mut r: libc::c_int = 0;
        (*zip).has_encrypted_entries = 1 as libc::c_int;
        if (*(*zip).entry).zip_flags as libc::c_int & ZIP_STRONG_ENCRYPTED != 0 {
            r = read_decryption_header(a)
        } else if (*(*zip).entry).compression as libc::c_int == WINZIP_AES_ENCRYPTION {
            r = init_WinZip_AES_decryption(a)
        } else {
            r = init_traditional_PKWARE_decryption(a)
        }
        if r != ARCHIVE_OK {
            return r;
        }
        (*zip).init_decryption = 0 as libc::c_int as libc::c_char
    }
    /* We're streaming and we don't know the length. */
    /* If the body is compressed and we know the format, we can
     * find an exact end-of-entry by decompressing it. */
    match (*(*zip).entry).compression as libc::c_int {
        8 => {
            /* Deflate compression. */
            while (*zip).end_of_entry == 0 {
                let mut offset: int64_t = 0 as libc::c_int as int64_t;
                let mut buff: *const libc::c_void = NULL as *const libc::c_void;
                let mut size: size_t = 0 as libc::c_int as size_t;
                let mut r_0: libc::c_int = 0;
                r_0 = zip_read_data_deflate(a, &mut buff, &mut size, &mut offset);
                if r_0 != ARCHIVE_OK {
                    return r_0;
                }
            }
            return ARCHIVE_OK;
        }
        _ => {
            loop
            /* Uncompressed or unknown. */
            /* Scan for a PK\007\010 signature. */
            {
                let mut p: *const libc::c_char = 0 as *const libc::c_char;
                let mut buff_0: *const libc::c_char = 0 as *const libc::c_char;
                let mut bytes_avail: ssize_t = 0;
                buff_0 = __archive_read_ahead(a, 16 as libc::c_int as size_t, &mut bytes_avail)
                    as *const libc::c_char;
                if bytes_avail < 16 as libc::c_int as libc::c_long {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Truncated ZIP file data\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                p = buff_0;
                while p
                    <= buff_0
                        .offset(bytes_avail as isize)
                        .offset(-(16 as libc::c_int as isize))
                {
                    if *p.offset(3 as libc::c_int as isize) as libc::c_int == 'P' as i32 {
                        p = p.offset(3 as libc::c_int as isize)
                    } else if *p.offset(3 as libc::c_int as isize) as libc::c_int == 'K' as i32 {
                        p = p.offset(2 as libc::c_int as isize)
                    } else if *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{7}' as i32
                    {
                        p = p.offset(1 as libc::c_int as isize)
                    } else if *p.offset(3 as libc::c_int as isize) as libc::c_int == '\u{8}' as i32
                        && *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{7}' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'K' as i32
                        && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
                    {
                        if (*(*zip).entry).flags as libc::c_int & LA_USED_ZIP64 != 0 {
                            __archive_read_consume(
                                a,
                                p.offset_from(buff_0) as libc::c_long
                                    + 24 as libc::c_int as libc::c_long,
                            );
                        } else {
                            __archive_read_consume(
                                a,
                                p.offset_from(buff_0) as libc::c_long
                                    + 16 as libc::c_int as libc::c_long,
                            );
                        }
                        return ARCHIVE_OK;
                    } else {
                        p = p.offset(4 as libc::c_int as isize)
                    }
                }
                __archive_read_consume(a, p.offset_from(buff_0) as libc::c_long);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_zip_streamable(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut zip: *mut zip = 0 as *mut zip;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_zip\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    zip = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<zip>() as libc::c_ulong,
    ) as *mut zip;
    if zip.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate zip data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Streamable reader doesn't support mac extensions. */
    (*zip).process_mac_extensions = 0 as libc::c_int;
    /*
     * Until enough data has been read, we cannot tell about
     * any encrypted entries yet.
     */
    (*zip).has_encrypted_entries = ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
    (*zip).crc32func = Some(
        real_crc32
            as unsafe extern "C" fn(
                _: libc::c_ulong,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_ulong,
    );
    r = __archive_read_register_format(
        a,
        zip as *mut libc::c_void,
        b"zip\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_zip_streamable_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_streamable_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_read_data_skip_streamable
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_zip_cleanup
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_support_format_zip_capabilities_streamable
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_has_encrypted_entries
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
    );
    if r != ARCHIVE_OK {
        free(zip as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
/* ------------------------------------------------------------------------ */
/*
 * Seeking-mode support
 */
unsafe extern "C" fn archive_read_support_format_zip_capabilities_seekable(
    mut a: *mut archive_read,
) -> libc::c_int {
    /* UNUSED */
    return ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA | ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA;
}
/*
 * TODO: This is a performance sink because it forces the read core to
 * drop buffered data from the start of file, which will then have to
 * be re-read again if this bidder loses.
 *
 * We workaround this a little by passing in the best bid so far so
 * that later bidders can do nothing if they know they'll never
 * outbid.  But we can certainly do better...
 */
unsafe extern "C" fn read_eocd(
    mut zip: *mut zip,
    mut p: *const libc::c_char,
    mut current_offset: int64_t,
) -> libc::c_int {
    /* Sanity-check the EOCD we've found. */
    /* This must be the first volume. */
    if archive_le16dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    /* Central directory must be on this volume. */
    if archive_le16dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        != archive_le16dec(p.offset(6 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_int
    {
        return 0 as libc::c_int;
    }
    /* All central directory entries must be on this volume. */
    if archive_le16dec(p.offset(10 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        != archive_le16dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_int
    {
        return 0 as libc::c_int;
    }
    /* Central directory can't extend beyond start of EOCD record. */
    if archive_le32dec(p.offset(16 as libc::c_int as isize) as *const libc::c_void).wrapping_add(
        archive_le32dec(p.offset(12 as libc::c_int as isize) as *const libc::c_void),
    ) as libc::c_long
        > current_offset
    {
        return 0 as libc::c_int;
    }
    /* Save the central directory location for later use. */
    (*zip).central_directory_offset =
        archive_le32dec(p.offset(16 as libc::c_int as isize) as *const libc::c_void) as int64_t;
    /* This is just a tiny bit higher than the maximum
    returned by the streaming Zip bidder.  This ensures
    that the more accurate seeking Zip parser wins
    whenever seek is available. */
    return 32 as libc::c_int;
}
/*
 * Examine Zip64 EOCD locator:  If it's valid, store the information
 * from it.
 */
unsafe extern "C" fn read_zip64_eocd(
    mut a: *mut archive_read,
    mut zip: *mut zip,
    mut p: *const libc::c_char,
) -> libc::c_int {
    let mut eocd64_offset: int64_t = 0;
    let mut eocd64_size: int64_t = 0;
    /* Sanity-check the locator record. */
    /* Central dir must be on first volume. */
    if archive_le32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void)
        != 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    /* Must be only a single volume. */
    if archive_le32dec(p.offset(16 as libc::c_int as isize) as *const libc::c_void)
        != 1 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    /* Find the Zip64 EOCD record. */
    eocd64_offset =
        archive_le64dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void) as int64_t;
    if __archive_read_seek(a, eocd64_offset, SEEK_SET) < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    p = __archive_read_ahead(a, 56 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    /* Make sure we can read all of it. */
    eocd64_size = archive_le64dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void)
        .wrapping_add(12 as libc::c_int as libc::c_ulong) as int64_t;
    if eocd64_size < 56 as libc::c_int as libc::c_long
        || eocd64_size > 16384 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    p = __archive_read_ahead(a, eocd64_size as size_t, NULL as *mut ssize_t) as *const libc::c_char;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    /* Sanity-check the EOCD64 */
    if archive_le32dec(p.offset(16 as libc::c_int as isize) as *const libc::c_void)
        != 0 as libc::c_int as libc::c_uint
    {
        /* Must be disk #0 */
        return 0 as libc::c_int;
    }
    if archive_le32dec(p.offset(20 as libc::c_int as isize) as *const libc::c_void)
        != 0 as libc::c_int as libc::c_uint
    {
        /* CD must be on disk #0 */
        return 0 as libc::c_int;
    }
    /* CD can't be split. */
    if archive_le64dec(p.offset(24 as libc::c_int as isize) as *const libc::c_void)
        != archive_le64dec(p.offset(32 as libc::c_int as isize) as *const libc::c_void)
    {
        return 0 as libc::c_int;
    }
    /* Save the central directory offset for later use. */
    (*zip).central_directory_offset =
        archive_le64dec(p.offset(48 as libc::c_int as isize) as *const libc::c_void) as int64_t;
    return 32 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_zip_seekable_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut file_size: int64_t = 0;
    let mut current_offset: int64_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tail: libc::c_int = 0;
    /* If someone has already bid more than 32, then avoid
    trashing the look-ahead buffers with a seek. */
    if best_bid > 32 as libc::c_int {
        return -(1 as libc::c_int);
    }
    file_size = __archive_read_seek(a, 0 as libc::c_int as int64_t, SEEK_END);
    if file_size <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    /* Search last 16k of file for end-of-central-directory
     * record (which starts with PK\005\006) */
    tail = if ((1024 as libc::c_int * 16 as libc::c_int) as libc::c_long) < file_size {
        (1024 as libc::c_int * 16 as libc::c_int) as libc::c_long
    } else {
        file_size
    } as libc::c_int;
    current_offset = __archive_read_seek(a, -tail as int64_t, SEEK_END);
    if current_offset < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    p = __archive_read_ahead(a, tail as size_t, NULL as *mut ssize_t) as *const libc::c_char;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    /* Boyer-Moore search backwards from the end, since we want
     * to match the last EOCD in the file (there can be more than
     * one if there is an uncompressed Zip archive as a member
     * within this Zip archive). */
    i = tail - 22 as libc::c_int;
    while i > 0 as libc::c_int {
        match *p.offset(i as isize) as libc::c_int {
            80 => {
                if memcmp(
                    p.offset(i as isize) as *const libc::c_void,
                    b"PK\x05\x06\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    let mut ret: libc::c_int = read_eocd(
                        zip,
                        p.offset(i as isize),
                        current_offset + i as libc::c_long,
                    );
                    /* Zip64 EOCD locator precedes
                     * regular EOCD if present. */
                    if i >= 20 as libc::c_int
                        && memcmp(
                            p.offset(i as isize).offset(-(20 as libc::c_int as isize))
                                as *const libc::c_void,
                            b"PK\x06\x07\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            4 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                    {
                        let mut ret_zip64: libc::c_int = read_zip64_eocd(
                            a,
                            zip,
                            p.offset(i as isize).offset(-(20 as libc::c_int as isize)),
                        );
                        if ret_zip64 > ret {
                            ret = ret_zip64
                        }
                    }
                    return ret;
                }
                i -= 4 as libc::c_int
            }
            75 => i -= 1 as libc::c_int,
            5 => i -= 2 as libc::c_int,
            6 => i -= 3 as libc::c_int,
            _ => i -= 4 as libc::c_int,
        }
    }
    return 0 as libc::c_int;
}
/* The red-black trees are only used in seeking mode to manage
 * the in-memory copy of the central directory. */
unsafe extern "C" fn cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut e1: *const zip_entry = n1 as *const zip_entry;
    let mut e2: *const zip_entry = n2 as *const zip_entry;
    if (*e1).local_header_offset > (*e2).local_header_offset {
        return -(1 as libc::c_int);
    }
    if (*e1).local_header_offset < (*e2).local_header_offset {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    /* This function won't be called */
    /* UNUSED */
    return 1 as libc::c_int;
}
static mut rb_ops: archive_rb_tree_ops = {
    let mut init = archive_rb_tree_ops {
        rbto_compare_nodes: Some(
            cmp_node
                as unsafe extern "C" fn(
                    _: *const archive_rb_node,
                    _: *const archive_rb_node,
                ) -> libc::c_int,
        ),
        rbto_compare_key: Some(
            cmp_key
                as unsafe extern "C" fn(
                    _: *const archive_rb_node,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    };
    init
};
unsafe extern "C" fn rsrc_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut e1: *const zip_entry = n1 as *const zip_entry;
    let mut e2: *const zip_entry = n2 as *const zip_entry;
    return strcmp((*e2).rsrcname.s, (*e1).rsrcname.s);
}
unsafe extern "C" fn rsrc_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut e: *const zip_entry = n as *const zip_entry;
    return strcmp(key as *const libc::c_char, (*e).rsrcname.s);
}
static mut rb_rsrc_ops: archive_rb_tree_ops = {
    let mut init = archive_rb_tree_ops {
        rbto_compare_nodes: Some(
            rsrc_cmp_node
                as unsafe extern "C" fn(
                    _: *const archive_rb_node,
                    _: *const archive_rb_node,
                ) -> libc::c_int,
        ),
        rbto_compare_key: Some(
            rsrc_cmp_key
                as unsafe extern "C" fn(
                    _: *const archive_rb_node,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    };
    init
};
unsafe extern "C" fn rsrc_basename(
    mut name: *const libc::c_char,
    mut name_length: size_t,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    s = name;
    r = s;
    loop {
        s = memchr(
            s as *const libc::c_void,
            '/' as i32,
            name_length.wrapping_sub(s.offset_from(name) as libc::c_long as libc::c_ulong),
        ) as *const libc::c_char;
        if s.is_null() {
            break;
        }
        s = s.offset(1);
        r = s
    }
    return r;
}
unsafe extern "C" fn expose_parent_dirs(
    mut zip: *mut zip,
    mut name: *const libc::c_char,
    mut name_length: size_t,
) {
    let mut str: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut dir: *mut zip_entry = 0 as *mut zip_entry;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    str.s = NULL as *mut libc::c_char;
    str.length = 0 as libc::c_int as size_t;
    str.buffer_length = 0 as libc::c_int as size_t;
    str.length = 0 as libc::c_int as size_t;
    archive_strncat(&mut str, name as *const libc::c_void, name_length);
    loop {
        s = strrchr(str.s, '/' as i32);
        if s.is_null() {
            break;
        }
        *s = '\u{0}' as i32 as libc::c_char;
        /* Transfer the parent directory from zip->tree_rsrc RB
         * tree to zip->tree RB tree to expose. */
        dir = __archive_rb_tree_find_node(&mut (*zip).tree_rsrc, str.s as *const libc::c_void)
            as *mut zip_entry;
        if dir.is_null() {
            break;
        }
        __archive_rb_tree_remove_node(&mut (*zip).tree_rsrc, &mut (*dir).node);
        archive_string_free(&mut (*dir).rsrcname);
        __archive_rb_tree_insert_node(&mut (*zip).tree, &mut (*dir).node);
    }
    archive_string_free(&mut str);
}
unsafe extern "C" fn slurp_central_directory(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut zip: *mut zip,
) -> libc::c_int {
    let mut i: ssize_t = 0;
    let mut found: libc::c_uint = 0;
    let mut correction: int64_t = 0;
    let mut bytes_avail: ssize_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /*
     * Find the start of the central directory.  The end-of-CD
     * record has our starting point, but there are lots of
     * Zip archives which have had other data prepended to the
     * file, which makes the recorded offsets all too small.
     * So we search forward from the specified offset until we
     * find the real start of the central directory.  Then we
     * know the correction we need to apply to account for leading
     * padding.
     */
    if __archive_read_seek(a, (*zip).central_directory_offset, SEEK_SET)
        < 0 as libc::c_int as libc::c_long
    {
        return ARCHIVE_FATAL;
    }
    found = 0 as libc::c_int as libc::c_uint;
    while found == 0 {
        p = __archive_read_ahead(a, 20 as libc::c_int as size_t, &mut bytes_avail)
            as *const libc::c_char;
        if p.is_null() {
            return ARCHIVE_FATAL;
        }
        found = 0 as libc::c_int as libc::c_uint;
        i = 0 as libc::c_int as ssize_t;
        while found == 0 && i < bytes_avail - 4 as libc::c_int as libc::c_long {
            match *p.offset((i + 3 as libc::c_int as libc::c_long) as isize) as libc::c_int {
                80 => i += 3 as libc::c_int as libc::c_long,
                75 => i += 2 as libc::c_int as libc::c_long,
                1 => i += 1 as libc::c_int as libc::c_long,
                2 => {
                    if memcmp(
                        p.offset(i as isize) as *const libc::c_void,
                        b"PK\x01\x02\x00" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        p = p.offset(i as isize);
                        found = 1 as libc::c_int as libc::c_uint
                    } else {
                        i += 4 as libc::c_int as libc::c_long
                    }
                }
                5 => i += 1 as libc::c_int as libc::c_long,
                6 => {
                    if memcmp(
                        p.offset(i as isize) as *const libc::c_void,
                        b"PK\x05\x06\x00" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        p = p.offset(i as isize);
                        found = 1 as libc::c_int as libc::c_uint
                    } else if memcmp(
                        p.offset(i as isize) as *const libc::c_void,
                        b"PK\x06\x06\x00" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        p = p.offset(i as isize);
                        found = 1 as libc::c_int as libc::c_uint
                    } else {
                        i += 1 as libc::c_int as libc::c_long
                    }
                }
                _ => i += 4 as libc::c_int as libc::c_long,
            }
        }
        __archive_read_consume(a, i);
    }
    correction =
        archive_filter_bytes(&mut (*a).archive, 0 as libc::c_int) - (*zip).central_directory_offset;
    __archive_rb_tree_init(&mut (*zip).tree, &rb_ops);
    __archive_rb_tree_init(&mut (*zip).tree_rsrc, &rb_rsrc_ops);
    (*zip).central_directory_entries_total = 0 as libc::c_int as size_t;
    loop {
        let mut zip_entry: *mut zip_entry = 0 as *mut zip_entry;
        let mut filename_length: size_t = 0;
        let mut extra_length: size_t = 0;
        let mut comment_length: size_t = 0;
        let mut external_attributes: uint32_t = 0;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut r: *const libc::c_char = 0 as *const libc::c_char;
        p = __archive_read_ahead(a, 4 as libc::c_int as size_t, NULL as *mut ssize_t)
            as *const libc::c_char;
        if p.is_null() {
            return ARCHIVE_FATAL;
        }
        if memcmp(
            p as *const libc::c_void,
            b"PK\x06\x06\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || memcmp(
                p as *const libc::c_void,
                b"PK\x05\x06\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            break;
        }
        if memcmp(
            p as *const libc::c_void,
            b"PK\x01\x02\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                -(1 as libc::c_int),
                b"Invalid central directory signature\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        p = __archive_read_ahead(a, 46 as libc::c_int as size_t, NULL as *mut ssize_t)
            as *const libc::c_char;
        if p.is_null() {
            return ARCHIVE_FATAL;
        }
        zip_entry = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<zip_entry>() as libc::c_ulong,
        ) as *mut zip_entry;
        if zip_entry.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate zip entry\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        (*zip_entry).next = (*zip).zip_entries;
        (*zip_entry).flags =
            ((*zip_entry).flags as libc::c_int | LA_FROM_CENTRAL_DIRECTORY) as libc::c_uchar;
        (*zip).zip_entries = zip_entry;
        (*zip).central_directory_entries_total =
            (*zip).central_directory_entries_total.wrapping_add(1);
        /* version = p[4]; */
        (*zip_entry).system = *p.offset(5 as libc::c_int as isize) as libc::c_uchar;
        /* version_required = archive_le16dec(p + 6); */
        (*zip_entry).zip_flags =
            archive_le16dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void);
        if (*zip_entry).zip_flags as libc::c_int & (ZIP_ENCRYPTED | ZIP_STRONG_ENCRYPTED) != 0 {
            (*zip).has_encrypted_entries = 1 as libc::c_int
        }
        (*zip_entry).compression =
            archive_le16dec(p.offset(10 as libc::c_int as isize) as *const libc::c_void)
                as libc::c_char as libc::c_uchar;
        (*zip_entry).mtime = zip_time(p.offset(12 as libc::c_int as isize));
        (*zip_entry).crc32 =
            archive_le32dec(p.offset(16 as libc::c_int as isize) as *const libc::c_void);
        if (*zip_entry).zip_flags as libc::c_int & ZIP_LENGTH_AT_END != 0 {
            (*zip_entry).decdat = *p.offset(13 as libc::c_int as isize) as libc::c_uchar
        } else {
            (*zip_entry).decdat = *p.offset(19 as libc::c_int as isize) as libc::c_uchar
        }
        (*zip_entry).compressed_size =
            archive_le32dec(p.offset(20 as libc::c_int as isize) as *const libc::c_void) as int64_t;
        (*zip_entry).uncompressed_size =
            archive_le32dec(p.offset(24 as libc::c_int as isize) as *const libc::c_void) as int64_t;
        filename_length =
            archive_le16dec(p.offset(28 as libc::c_int as isize) as *const libc::c_void) as size_t;
        extra_length =
            archive_le16dec(p.offset(30 as libc::c_int as isize) as *const libc::c_void) as size_t;
        comment_length =
            archive_le16dec(p.offset(32 as libc::c_int as isize) as *const libc::c_void) as size_t;
        /* disk_start = archive_le16dec(p + 34);
         *   Better be zero.
         * internal_attributes = archive_le16dec(p + 36);
         *   text bit */
        external_attributes =
            archive_le32dec(p.offset(38 as libc::c_int as isize) as *const libc::c_void);
        (*zip_entry).local_header_offset =
            archive_le32dec(p.offset(42 as libc::c_int as isize) as *const libc::c_void)
                as libc::c_long
                + correction;
        /* If we can't guess the mode, leave it zero here;
        when we read the local file header we might get
        more information. */
        if (*zip_entry).system as libc::c_int == 3 as libc::c_int {
            (*zip_entry).mode = (external_attributes >> 16 as libc::c_int) as uint16_t
        } else if (*zip_entry).system as libc::c_int == 0 as libc::c_int {
            // Interpret MSDOS directory bit
            if 0x10 as libc::c_int as libc::c_uint
                == external_attributes & 0x10 as libc::c_int as libc::c_uint
            {
                (*zip_entry).mode =
                    (AE_IFDIR as mode_t | 0o775 as libc::c_int as libc::c_uint) as uint16_t
            } else {
                (*zip_entry).mode =
                    (AE_IFREG as mode_t | 0o664 as libc::c_int as libc::c_uint) as uint16_t
            }
            if 0x1 as libc::c_int as libc::c_uint
                == external_attributes & 0x1 as libc::c_int as libc::c_uint
            {
                // Read-only bit; strip write permissions
                (*zip_entry).mode =
                    ((*zip_entry).mode as libc::c_int & 0o555 as libc::c_int) as uint16_t
            }
        } else {
            (*zip_entry).mode = 0 as libc::c_int as uint16_t
        }
        /* We're done with the regular data; get the filename and
         * extra data. */
        __archive_read_consume(a, 46 as libc::c_int as int64_t);
        p = __archive_read_ahead(
            a,
            filename_length.wrapping_add(extra_length),
            NULL as *mut ssize_t,
        ) as *const libc::c_char;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated ZIP file header\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        if ARCHIVE_OK
            != process_extra(
                a,
                entry,
                p.offset(filename_length as isize),
                extra_length,
                zip_entry,
            )
        {
            return ARCHIVE_FATAL;
        }
        /*
         * Mac resource fork files are stored under the
         * "__MACOSX/" directory, so we should check if
         * it is.
         */
        if (*zip).process_mac_extensions == 0 {
            /* Treat every entry as a regular entry. */
            __archive_rb_tree_insert_node(&mut (*zip).tree, &mut (*zip_entry).node);
        } else {
            name = p;
            r = rsrc_basename(name, filename_length);
            if filename_length >= 9 as libc::c_int as libc::c_ulong
                && strncmp(
                    b"__MACOSX/\x00" as *const u8 as *const libc::c_char,
                    name,
                    9 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                /* If this file is not a resource fork nor
                 * a directory. We should treat it as a non
                 * resource fork file to expose it. */
                if *name.offset(
                    filename_length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int
                    != '/' as i32
                    && ((r.offset_from(name) as libc::c_long)
                        < 3 as libc::c_int as libc::c_long
                        || *r.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
                        || *r.offset(1 as libc::c_int as isize) as libc::c_int != '_' as i32)
                {
                    __archive_rb_tree_insert_node(&mut (*zip).tree, &mut (*zip_entry).node);
                    /* Expose its parent directories. */
                    expose_parent_dirs(zip, name, filename_length);
                } else {
                    /* This file is a resource fork file or
                     * a directory. */
                    (*zip_entry).rsrcname.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut (*zip_entry).rsrcname,
                        name as *const libc::c_void,
                        filename_length,
                    );
                    __archive_rb_tree_insert_node(&mut (*zip).tree_rsrc, &mut (*zip_entry).node);
                }
            } else {
                /* Generate resource fork name to find its
                 * resource file at zip->tree_rsrc. */
                (*zip_entry).rsrcname.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*zip_entry).rsrcname,
                    b"__MACOSX/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    (if (b"__MACOSX/\x00" as *const u8 as *const libc::c_char).is_null() {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(b"__MACOSX/\x00" as *const u8 as *const libc::c_char)
                    }),
                );
                archive_strncat(
                    &mut (*zip_entry).rsrcname,
                    name as *const libc::c_void,
                    r.offset_from(name) as libc::c_long as size_t,
                );
                archive_strcat(
                    &mut (*zip_entry).rsrcname,
                    b"._\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                );
                archive_strncat(&mut (*zip_entry).rsrcname,
                                name.offset(r.offset_from(name) as
                                                libc::c_long as isize) as
                                    *const libc::c_void,
                                filename_length.wrapping_sub(r.offset_from(name)
                                                                 as
                                                                 libc::c_long
                                                                 as
                                                                 libc::c_ulong));
                /* Register an entry to RB tree to sort it by
                 * file offset. */
                __archive_rb_tree_insert_node(&mut (*zip).tree, &mut (*zip_entry).node);
            }
        }
        /* Skip the comment too ... */
        __archive_read_consume(
            a,
            filename_length
                .wrapping_add(extra_length)
                .wrapping_add(comment_length) as int64_t,
        );
    }
    return ARCHIVE_OK;
}
unsafe extern "C" fn zip_get_local_file_header_size(
    mut a: *mut archive_read,
    mut extra: size_t,
) -> ssize_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut filename_length: ssize_t = 0;
    let mut extra_length: ssize_t = 0;
    p = __archive_read_ahead(
        a,
        extra.wrapping_add(30 as libc::c_int as libc::c_ulong),
        NULL as *mut ssize_t,
    ) as *const libc::c_char;
    if p.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated ZIP file header\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int) as ssize_t;
    }
    p = p.offset(extra as isize);
    if memcmp(
        p as *const libc::c_void,
        b"PK\x03\x04\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Damaged Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_WARN as ssize_t;
    }
    filename_length =
        archive_le16dec(p.offset(26 as libc::c_int as isize) as *const libc::c_void) as ssize_t;
    extra_length =
        archive_le16dec(p.offset(28 as libc::c_int as isize) as *const libc::c_void) as ssize_t;
    return 30 as libc::c_int as libc::c_long + filename_length + extra_length;
}
unsafe extern "C" fn zip_read_mac_metadata(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut rsrc: *mut zip_entry,
) -> libc::c_int {
    let mut current_block: u64;
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut metadata: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut mp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut offset: int64_t = archive_filter_bytes(&mut (*a).archive, 0 as libc::c_int);
    let mut remaining_bytes: size_t = 0;
    let mut metadata_bytes: size_t = 0;
    let mut hsize: ssize_t = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    let mut eof: libc::c_int = 0;
    match (*rsrc).compression as libc::c_int {
        0 => {
            /* No compression. */
            if (*rsrc).uncompressed_size != (*rsrc).compressed_size {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Malformed OS X metadata entry: inconsistent size\x00" as *const u8
                        as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        8 => {}
        _ => {
            /* Unsupported compression. */
            /* Return a warning. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unsupported ZIP compression method (%s)\x00" as *const u8 as *const libc::c_char,
                compression_name((*rsrc).compression as libc::c_int),
            );
            /* We can't decompress this entry, but we will
             * be able to skip() it and try the next entry. */
            return -(20 as libc::c_int);
        }
    }
    if (*rsrc).uncompressed_size
        > (4 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_long
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Mac metadata is too large: %jd > 4M bytes\x00" as *const u8 as *const libc::c_char,
            (*rsrc).uncompressed_size,
        );
        return -(20 as libc::c_int);
    }
    if (*rsrc).compressed_size
        > (4 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_long
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Mac metadata is too large: %jd > 4M bytes\x00" as *const u8 as *const libc::c_char,
            (*rsrc).compressed_size,
        );
        return -(20 as libc::c_int);
    }
    metadata = malloc((*rsrc).uncompressed_size as size_t) as *mut libc::c_uchar;
    if metadata.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory for Mac metadata\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if offset < (*rsrc).local_header_offset {
        __archive_read_consume(a, (*rsrc).local_header_offset - offset);
    } else if offset != (*rsrc).local_header_offset {
        __archive_read_seek(a, (*rsrc).local_header_offset, SEEK_SET);
    }
    hsize = zip_get_local_file_header_size(a, 0 as libc::c_int as size_t);
    __archive_read_consume(a, hsize);
    remaining_bytes = (*rsrc).compressed_size as size_t;
    metadata_bytes = (*rsrc).uncompressed_size as size_t;
    mp = metadata;
    eof = 0 as libc::c_int;
    loop {
        if !(eof == 0 && remaining_bytes != 0) {
            current_block = 16029476503615101993;
            break;
        }
        let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
        let mut bytes_avail: ssize_t = 0;
        let mut bytes_used: size_t = 0;
        p = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail)
            as *const libc::c_uchar;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated ZIP file header\x00" as *const u8 as *const libc::c_char,
            );
            ret = ARCHIVE_WARN;
            current_block = 14717272049051617046;
            break;
        } else {
            if bytes_avail as size_t > remaining_bytes {
                bytes_avail = remaining_bytes as ssize_t
            }
            match (*rsrc).compression as libc::c_int {
                0 => {
                    /* No compression. */
                    if bytes_avail as size_t > metadata_bytes {
                        bytes_avail = metadata_bytes as ssize_t
                    }
                    memcpy(
                        mp as *mut libc::c_void,
                        p as *const libc::c_void,
                        bytes_avail as libc::c_ulong,
                    );
                    bytes_used = bytes_avail as size_t;
                    metadata_bytes = (metadata_bytes as libc::c_ulong).wrapping_sub(bytes_used)
                        as size_t as size_t;
                    mp = mp.offset(bytes_used as isize);
                    if metadata_bytes == 0 as libc::c_int as libc::c_ulong {
                        eof = 1 as libc::c_int
                    }
                }
                8 => {
                    /* Deflate compression. */
                    let mut r: libc::c_int = 0;
                    ret = zip_deflate_init(a, zip);
                    if ret != ARCHIVE_OK {
                        current_block = 14717272049051617046;
                        break;
                    }
                    (*zip).stream.next_in = p as *const libc::c_void as uintptr_t as *mut Bytef;
                    (*zip).stream.avail_in = bytes_avail as uInt;
                    (*zip).stream.total_in = 0 as libc::c_int as uLong;
                    (*zip).stream.next_out = mp;
                    (*zip).stream.avail_out = metadata_bytes as uInt;
                    (*zip).stream.total_out = 0 as libc::c_int as uLong;
                    r = inflate(&mut (*zip).stream, 0 as libc::c_int);
                    match r {
                        Z_OK => {}
                        Z_STREAM_END => eof = 1 as libc::c_int,
                        -4 => {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ENOMEM,
                                b"Out of memory for ZIP decompression\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            ret = ARCHIVE_FATAL;
                            current_block = 14717272049051617046;
                            break;
                        }
                        _ => {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"ZIP decompression failed (%d)\x00" as *const u8
                                    as *const libc::c_char,
                                r,
                            );
                            ret = ARCHIVE_FATAL;
                            current_block = 14717272049051617046;
                            break;
                        }
                    }
                    bytes_used = (*zip).stream.total_in;
                    metadata_bytes = (metadata_bytes as libc::c_ulong)
                        .wrapping_sub((*zip).stream.total_out)
                        as size_t as size_t;
                    mp = mp.offset((*zip).stream.total_out as isize)
                }
                _ => bytes_used = 0 as libc::c_int as size_t,
            }
            __archive_read_consume(a, bytes_used as int64_t);
            remaining_bytes =
                (remaining_bytes as libc::c_ulong).wrapping_sub(bytes_used) as size_t as size_t
        }
    }
    match current_block {
        16029476503615101993 => {
            archive_entry_copy_mac_metadata(
                entry,
                metadata as *const libc::c_void,
                ((*rsrc).uncompressed_size as size_t).wrapping_sub(metadata_bytes),
            );
        }
        _ => {}
    }
    __archive_read_seek(a, offset, SEEK_SET);
    (*zip).decompress_init = 0 as libc::c_int as libc::c_char;
    free(metadata as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn archive_read_format_zip_seekable_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut zip: *mut zip = (*(*a).format).data as *mut zip;
    let mut rsrc: *mut zip_entry = 0 as *mut zip_entry;
    let mut offset: int64_t = 0;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    /*
     * It should be sufficient to call archive_read_next_header() for
     * a reader to determine if an entry is encrypted or not. If the
     * encryption of an entry is only detectable when calling
     * archive_read_data(), so be it. We'll do the same check there
     * as well.
     */
    if (*zip).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*zip).has_encrypted_entries = 0 as libc::c_int
    }
    (*a).archive.archive_format = ARCHIVE_FORMAT_ZIP;
    if (*a).archive.archive_format_name.is_null() {
        (*a).archive.archive_format_name = b"ZIP\x00" as *const u8 as *const libc::c_char
    }
    if (*zip).zip_entries.is_null() {
        r = slurp_central_directory(a, entry, zip);
        if r != ARCHIVE_OK {
            return r;
        }
        /* Get first entry whose local header offset is lower than
         * other entries in the archive file. */
        (*zip).entry = __archive_rb_tree_iterate(
            &mut (*zip).tree,
            NULL as *mut archive_rb_node,
            ARCHIVE_RB_DIR_LEFT as libc::c_uint,
        ) as *mut zip_entry
    } else if !(*zip).entry.is_null() {
        /* Get next entry in local header offset order. */
        (*zip).entry = __archive_rb_tree_iterate(
            &mut (*zip).tree,
            &mut (*(*zip).entry).node,
            ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
        ) as *mut zip_entry
    }
    if (*zip).entry.is_null() {
        return ARCHIVE_EOF;
    }
    if !(*(*zip).entry).rsrcname.s.is_null() {
        rsrc = __archive_rb_tree_find_node(
            &mut (*zip).tree_rsrc,
            (*(*zip).entry).rsrcname.s as *const libc::c_void,
        ) as *mut zip_entry
    } else {
        rsrc = NULL as *mut zip_entry
    }
    if (*zip).cctx_valid != 0 {
        __archive_cryptor
            .decrypto_aes_ctr_release
            .expect("non-null function pointer")(&mut (*zip).cctx);
    }
    if (*zip).hctx_valid != 0 {
        __archive_hmac
            .__hmac_sha1_cleanup
            .expect("non-null function pointer")(&mut (*zip).hctx);
    }
    (*zip).hctx_valid = 0 as libc::c_int as libc::c_char;
    (*zip).cctx_valid = (*zip).hctx_valid;
    (*zip).tctx_valid = (*zip).cctx_valid;
    __archive_read_reset_passphrase(a);
    /* File entries are sorted by the header offset, we should mostly
     * use __archive_read_consume to advance a read point to avoid
     * redundant data reading.  */
    offset = archive_filter_bytes(&mut (*a).archive, 0 as libc::c_int);
    if offset < (*(*zip).entry).local_header_offset {
        __archive_read_consume(a, (*(*zip).entry).local_header_offset - offset);
    } else if offset != (*(*zip).entry).local_header_offset {
        __archive_read_seek(a, (*(*zip).entry).local_header_offset, SEEK_SET);
    }
    (*zip).unconsumed = 0 as libc::c_int as size_t;
    r = zip_read_local_file_header(a, entry, zip);
    if r != ARCHIVE_OK {
        return r;
    }
    if !rsrc.is_null() {
        let mut ret2: libc::c_int = zip_read_mac_metadata(a, entry, rsrc);
        if ret2 < ret {
            ret = ret2
        }
    }
    return ret;
}
/*
 * We're going to seek for the next header anyway, so we don't
 * need to bother doing anything here.
 */
unsafe extern "C" fn archive_read_format_zip_read_data_skip_seekable(
    mut a: *mut archive_read,
) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    zip = (*(*a).format).data as *mut zip;
    (*zip).unconsumed = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_zip_seekable(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut zip: *mut zip = 0 as *mut zip;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_zip_seekable\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    zip = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<zip>() as libc::c_ulong,
    ) as *mut zip;
    if zip.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate zip data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * Until enough data has been read, we cannot tell about
     * any encrypted entries yet.
     */
    (*zip).has_encrypted_entries = ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
    (*zip).crc32func = Some(
        real_crc32
            as unsafe extern "C" fn(
                _: libc::c_ulong,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_ulong,
    );
    r = __archive_read_register_format(
        a,
        zip as *mut libc::c_void,
        b"zip\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_zip_seekable_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_seekable_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_read_data_skip_seekable
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_zip_cleanup
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_support_format_zip_capabilities_seekable
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_format_zip_has_encrypted_entries
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
    );
    if r != ARCHIVE_OK {
        free(zip as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    num_compression_methods = (::std::mem::size_of::<[C2RustUnnamed_2; 25]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
/*# vim:set noet:*/
