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
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateSetDictionary(
        strm: z_streamp,
        dictionary: *const Bytef,
        dictLength: uInt,
    ) -> libc::c_int;
    #[no_mangle]
    fn inflateReset(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_entry_pathname_w(_: *mut archive_entry) -> *const wchar_t;
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_copy_pathname_w(_: *mut archive_entry, _: *const wchar_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn _archive_entry_copy_pathname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_default_conversion_for_read(_: *mut archive) -> *mut archive_string_conv;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
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
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn archive_wstrncat(
        _: *mut archive_wstring,
        _: *const wchar_t,
        _: size_t,
    ) -> *mut archive_wstring;
    #[no_mangle]
    fn archive_wstring_ensure(_: *mut archive_wstring, _: size_t) -> *mut archive_wstring;
    #[no_mangle]
    fn archive_wstring_free(_: *mut archive_wstring);
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
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
pub struct cab {
    pub entry_offset: int64_t,
    pub entry_bytes_remaining: int64_t,
    pub entry_unconsumed: int64_t,
    pub entry_compressed_bytes_read: int64_t,
    pub entry_uncompressed_bytes_read: int64_t,
    pub entry_cffolder: *mut cffolder,
    pub entry_cffile: *mut cffile,
    pub entry_cfdata: *mut cfdata,
    pub cab_offset: int64_t,
    pub cfheader: cfheader,
    pub ws: archive_wstring,
    pub found_header: libc::c_char,
    pub end_of_archive: libc::c_char,
    pub end_of_entry: libc::c_char,
    pub end_of_entry_cleanup: libc::c_char,
    pub read_data_invoked: libc::c_char,
    pub bytes_skipped: int64_t,
    pub uncompressed_buffer: *mut libc::c_uchar,
    pub uncompressed_buffer_size: size_t,
    pub init_default_conversion: libc::c_int,
    pub sconv: *mut archive_string_conv,
    pub sconv_default: *mut archive_string_conv,
    pub sconv_utf8: *mut archive_string_conv,
    pub format_name: [libc::c_char; 64],
    pub stream: z_stream,
    pub stream_valid: libc::c_char,
    pub xstrm: lzx_stream,
}
/*->25*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_stream {
    pub next_in: *const libc::c_uchar,
    pub avail_in: int64_t,
    pub total_in: int64_t,
    pub next_out: *mut libc::c_uchar,
    pub avail_out: int64_t,
    pub total_out: int64_t,
    pub ds: *mut lzx_dec,
}
/*-
 * Copyright (c) 2010-2012 Michihiro NAKAJIMA
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_dec {
    pub state: libc::c_int,
    pub w_size: libc::c_int,
    pub w_mask: libc::c_int,
    pub w_buff: *mut libc::c_uchar,
    pub w_pos: libc::c_int,
    pub copy_pos: libc::c_int,
    pub copy_len: libc::c_int,
    pub translation_size: uint32_t,
    pub translation: libc::c_char,
    pub block_type: libc::c_char,
    pub block_size: size_t,
    pub block_bytes_avail: size_t,
    pub r0: libc::c_int,
    pub r1: libc::c_int,
    pub r2: libc::c_int,
    pub rbytes: [libc::c_uchar; 4],
    pub rbytes_avail: libc::c_int,
    pub length_header: libc::c_int,
    pub position_slot: libc::c_int,
    pub offset_bits: libc::c_int,
    pub pos_tbl: *mut lzx_pos_tbl,
    pub br: lzx_br,
    pub at: huffman,
    pub lt: huffman,
    pub mt: huffman,
    pub pt: huffman,
    pub loop_0: libc::c_int,
    pub error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffman {
    pub len_size: libc::c_int,
    pub freq: [libc::c_int; 17],
    pub bitlen: *mut libc::c_uchar,
    pub max_bits: libc::c_int,
    pub tbl_bits: libc::c_int,
    pub tree_used: libc::c_int,
    pub tbl: *mut uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_br {
    pub cache_buffer: uint64_t,
    pub cache_avail: libc::c_int,
    pub odd: libc::c_uchar,
    pub have_odd: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzx_pos_tbl {
    pub base: libc::c_int,
    pub footer_bits: libc::c_int,
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
pub struct cfheader {
    pub total_bytes: uint32_t,
    pub files_offset: uint32_t,
    pub folder_count: uint16_t,
    pub file_count: uint16_t,
    pub flags: uint16_t,
    pub setid: uint16_t,
    pub cabinet: uint16_t,
    pub major: libc::c_uchar,
    pub minor: libc::c_uchar,
    pub cffolder: libc::c_uchar,
    pub cfdata: libc::c_uchar,
    pub folder_array: *mut cffolder,
    pub file_array: *mut cffile,
    pub file_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cffile {
    pub uncompressed_size: uint32_t,
    pub offset: uint32_t,
    pub mtime: time_t,
    pub folder: uint16_t,
    pub attr: libc::c_uchar,
    pub pathname: archive_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cffolder {
    pub cfdata_offset_in_cab: uint32_t,
    pub cfdata_count: uint16_t,
    pub comptype: uint16_t,
    pub compdata: uint16_t,
    pub compname: *const libc::c_char,
    pub cfdata: cfdata,
    pub cfdata_index: libc::c_int,
    pub decompress_init: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfdata {
    pub sum: uint32_t,
    pub compressed_size: uint16_t,
    pub compressed_bytes_remaining: uint16_t,
    pub uncompressed_size: uint16_t,
    pub uncompressed_bytes_remaining: uint16_t,
    pub uncompressed_avail: uint16_t,
    pub read_offset: uint16_t,
    pub unconsumed: int64_t,
    pub memimage_size: size_t,
    pub memimage: *mut libc::c_uchar,
    pub sum_calculated: uint32_t,
    pub sum_extra: [libc::c_uchar; 4],
    pub sum_extra_avail: libc::c_int,
    pub sum_ptr: *const libc::c_void,
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
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const UCHAR_MAX: libc::c_int = __SCHAR_MAX__ * 2 as libc::c_int + 1 as libc::c_int;
pub const __SCHAR_MAX__: libc::c_int = 127 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_FORMAT_CAB: libc::c_int = 0xc0000 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
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
pub const archive_entry_copy_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_pathname_l;
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
unsafe extern "C" fn archive_le16enc(mut pp: *mut libc::c_void, mut u: uint16_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
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
pub const VERBATIM_BLOCK: libc::c_int = 1 as libc::c_int;
pub const ALIGNED_OFFSET_BLOCK: libc::c_int = 2 as libc::c_int;
pub const UNCOMPRESSED_BLOCK: libc::c_int = 3 as libc::c_int;
static mut slots: [libc::c_int; 11] = [
    30 as libc::c_int,
    32 as libc::c_int,
    34 as libc::c_int,
    36 as libc::c_int,
    38 as libc::c_int,
    42 as libc::c_int,
    50 as libc::c_int,
    66 as libc::c_int,
    98 as libc::c_int,
    162 as libc::c_int,
    290 as libc::c_int,
];
pub const SLOT_BASE: libc::c_int = 15 as libc::c_int;
pub const SLOT_MAX: libc::c_int = 21 as libc::c_int;
/*
 * Cabinet file definitions.
 */
/* CFHEADER offset */
pub const CFHEADER_signature: libc::c_int = 0 as libc::c_int;
pub const CFHEADER_cbCabinet: libc::c_int = 8 as libc::c_int;
pub const CFHEADER_coffFiles: libc::c_int = 16 as libc::c_int;
pub const CFHEADER_versionMinor: libc::c_int = 24 as libc::c_int;
pub const CFHEADER_versionMajor: libc::c_int = 25 as libc::c_int;
pub const CFHEADER_cFolders: libc::c_int = 26 as libc::c_int;
pub const CFHEADER_cFiles: libc::c_int = 28 as libc::c_int;
pub const CFHEADER_flags: libc::c_int = 30 as libc::c_int;
pub const CFHEADER_setID: libc::c_int = 32 as libc::c_int;
pub const CFHEADER_iCabinet: libc::c_int = 34 as libc::c_int;
pub const CFHEADER_cbCFHeader: libc::c_int = 36 as libc::c_int;
pub const CFHEADER_cbCFFolder: libc::c_int = 38 as libc::c_int;
pub const CFHEADER_cbCFData: libc::c_int = 39 as libc::c_int;
/* CFFOLDER offset */
pub const CFFOLDER_coffCabStart: libc::c_int = 0 as libc::c_int;
pub const CFFOLDER_cCFData: libc::c_int = 4 as libc::c_int;
pub const CFFOLDER_typeCompress: libc::c_int = 6 as libc::c_int;
/* CFFILE offset */
pub const CFFILE_cbFile: libc::c_int = 0 as libc::c_int;
pub const CFFILE_uoffFolderStart: libc::c_int = 4 as libc::c_int;
pub const CFFILE_iFolder: libc::c_int = 8 as libc::c_int;
pub const CFFILE_date_time: libc::c_int = 10 as libc::c_int;
pub const CFFILE_attribs: libc::c_int = 14 as libc::c_int;
/* CFDATA offset */
pub const CFDATA_csum: libc::c_int = 0 as libc::c_int;
pub const CFDATA_cbData: libc::c_int = 4 as libc::c_int;
pub const CFDATA_cbUncomp: libc::c_int = 6 as libc::c_int;
static mut compression_name: [*const libc::c_char; 4] = [
    b"NONE\x00" as *const u8 as *const libc::c_char,
    b"MSZIP\x00" as *const u8 as *const libc::c_char,
    b"Quantum\x00" as *const u8 as *const libc::c_char,
    b"LZX\x00" as *const u8 as *const libc::c_char,
];
pub const COMPTYPE_NONE: libc::c_int = 0 as libc::c_int;
pub const COMPTYPE_MSZIP: libc::c_int = 0x1 as libc::c_int;
pub const COMPTYPE_LZX: libc::c_int = 0x3 as libc::c_int;
pub const iFoldCONTINUED_FROM_PREV: libc::c_int = 0xfffd as libc::c_int;
pub const iFoldCONTINUED_TO_NEXT: libc::c_int = 0xfffe as libc::c_int;
pub const iFoldCONTINUED_PREV_AND_NEXT: libc::c_int = 0xffff as libc::c_int;
pub const ATTR_RDONLY: libc::c_int = 0x1 as libc::c_int;
pub const ATTR_NAME_IS_UTF: libc::c_int = 0x80 as libc::c_int;
pub const PREV_CABINET: libc::c_int = 0x1 as libc::c_int;
pub const NEXT_CABINET: libc::c_int = 0x2 as libc::c_int;
pub const RESERVE_PRESENT: libc::c_int = 0x4 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_cab(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cab: *mut cab = 0 as *mut cab;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_cab\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    cab = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cab>() as libc::c_ulong,
    ) as *mut cab;
    if cab.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate CAB data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*cab).ws.s = NULL as *mut wchar_t;
    (*cab).ws.length = 0 as libc::c_int as size_t;
    (*cab).ws.buffer_length = 0 as libc::c_int as size_t;
    archive_wstring_ensure(&mut (*cab).ws, 256 as libc::c_int as size_t);
    r = __archive_read_register_format(
        a,
        cab as *mut libc::c_void,
        b"cab\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_cab_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_cab_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_cab_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_cab_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_cab_read_data_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_cab_cleanup
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
        free(cab as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_cab_magic(mut p: *const libc::c_char) -> libc::c_int {
    match *p.offset(4 as libc::c_int as isize) as libc::c_int {
        0 => {
            /*
             * Note: Self-Extraction program has 'MSCF' string in their
             * program. If we were finding 'MSCF' string only, we got
             * wrong place for Cabinet header, thus, we have to check
             * following four bytes which are reserved and must be set
             * to zero.
             */
            if memcmp(
                p as *const libc::c_void,
                b"MSCF\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            return 5 as libc::c_int;
        }
        70 => return 1 as libc::c_int,
        67 => return 2 as libc::c_int,
        83 => return 3 as libc::c_int,
        77 => return 4 as libc::c_int,
        _ => return 5 as libc::c_int,
    };
}
unsafe extern "C" fn archive_read_format_cab_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut bytes_avail: ssize_t = 0;
    let mut offset: ssize_t = 0;
    let mut window: ssize_t = 0;
    /* If there's already a better bid than we can ever
    make, don't bother testing. */
    if best_bid > 64 as libc::c_int {
        return -(1 as libc::c_int);
    }
    p = __archive_read_ahead(a, 8 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if memcmp(
        p as *const libc::c_void,
        b"MSCF\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 64 as libc::c_int;
    }
    /*
     * Attempt to handle self-extracting archives
     * by noting a PE header and searching forward
     * up to 128k for a 'MSCF' marker.
     */
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
    {
        offset = 0 as libc::c_int as ssize_t;
        window = 4096 as libc::c_int as ssize_t;
        while offset < (1024 as libc::c_int * 128 as libc::c_int) as libc::c_long {
            let mut h: *const libc::c_char =
                __archive_read_ahead(a, (offset + window) as size_t, &mut bytes_avail)
                    as *const libc::c_char;
            if h.is_null() {
                /* Remaining bytes are less than window. */
                window >>= 1 as libc::c_int;
                if window < 128 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int;
                }
            } else {
                p = h.offset(offset as isize);
                while p.offset(8 as libc::c_int as isize) < h.offset(bytes_avail as isize) {
                    let mut next: libc::c_int = 0;
                    next = find_cab_magic(p);
                    if next == 0 as libc::c_int {
                        return 64 as libc::c_int;
                    }
                    p = p.offset(next as isize)
                }
                offset = p.offset_from(h) as libc::c_long
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_cab_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut cab: *mut cab = 0 as *mut cab;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    cab = (*(*a).format).data as *mut cab;
    if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if val.is_null()
            || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"cab: hdrcharset option needs a character-set name\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            (*cab).sconv =
                archive_string_conversion_from_charset(&mut (*a).archive, val, 0 as libc::c_int);
            if !(*cab).sconv.is_null() {
                ret = ARCHIVE_OK
            } else {
                ret = ARCHIVE_FATAL
            }
        }
        return ret;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn cab_skip_sfx(mut a: *mut archive_read) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut skip: size_t = 0;
    let mut bytes: ssize_t = 0;
    let mut window: ssize_t = 0;
    window = 4096 as libc::c_int as ssize_t;
    loop {
        let mut h: *const libc::c_char =
            __archive_read_ahead(a, window as size_t, &mut bytes) as *const libc::c_char;
        if h.is_null() {
            /* Remaining size are less than window. */
            window >>= 1 as libc::c_int;
            if window < 128 as libc::c_int as libc::c_long {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Couldn\'t find out CAB header\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        } else {
            p = h;
            q = p.offset(bytes as isize);
            /*
             * Scan ahead until we find something that looks
             * like the cab header.
             */
            while p.offset(8 as libc::c_int as isize) < q {
                let mut next: libc::c_int = 0; /* invalid */
                next = find_cab_magic(p);
                if next == 0 as libc::c_int {
                    skip = p.offset_from(h) as libc::c_long as size_t;
                    __archive_read_consume(a, skip as int64_t);
                    return 0 as libc::c_int;
                }
                p = p.offset(next as isize)
            }
            skip = p.offset_from(h) as libc::c_long as size_t;
            __archive_read_consume(a, skip as int64_t);
        }
    }
}
unsafe extern "C" fn truncated_error(mut a: *mut archive_read) -> libc::c_int {
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Truncated CAB header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn cab_strnlen(mut p: *const libc::c_uchar, mut maxlen: size_t) -> ssize_t {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i <= maxlen {
        if *p.offset(i as isize) as libc::c_int == 0 as libc::c_int {
            break;
        }
        i = i.wrapping_add(1)
    }
    if i > maxlen {
        return -(1 as libc::c_int) as ssize_t;
    }
    return i as ssize_t;
}
/* Read bytes as much as remaining. */
unsafe extern "C" fn cab_read_ahead_remaining(
    mut a: *mut archive_read,
    mut min: size_t,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    let mut p: *const libc::c_void = 0 as *const libc::c_void;
    while min > 0 as libc::c_int as libc::c_ulong {
        p = __archive_read_ahead(a, min, avail);
        if p != NULL as *const libc::c_void {
            return p;
        }
        min = min.wrapping_sub(1)
    }
    return 0 as *const libc::c_void;
}
/* Convert a path separator '\' -> '/' */
unsafe extern "C" fn cab_convert_path_separator_1(
    mut fn_0: *mut archive_string,
    mut attr: libc::c_uchar,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut mb: libc::c_int = 0;
    /* Easy check if we have '\' in multi-byte string. */
    mb = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*fn_0).length {
        if *(*fn_0).s.offset(i as isize) as libc::c_int == '\\' as i32 {
            if mb != 0 {
                break;
            }
            *(*fn_0).s.offset(i as isize) = '/' as i32 as libc::c_char;
            mb = 0 as libc::c_int
        } else if *(*fn_0).s.offset(i as isize) as libc::c_int & 0x80 as libc::c_int != 0
            && attr as libc::c_int & ATTR_NAME_IS_UTF == 0
        {
            mb = 1 as libc::c_int
        } else {
            mb = 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    if i == (*fn_0).length {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
/*
 * Replace a character '\' with '/' in wide character.
 */
unsafe extern "C" fn cab_convert_path_separator_2(
    mut cab: *mut cab,
    mut entry: *mut archive_entry,
) {
    let mut wp: *const wchar_t = 0 as *const wchar_t;
    let mut i: size_t = 0;
    /* If a conversion to wide character failed, force the replacement. */
    wp = archive_entry_pathname_w(entry);
    if !wp.is_null() {
        (*cab).ws.length = 0 as libc::c_int as size_t;
        archive_wstrncat(
            &mut (*cab).ws,
            wp,
            (if wp.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                wcslen(wp)
            }),
        );
        i = 0 as libc::c_int as size_t;
        while i < (*cab).ws.length {
            if *(*cab).ws.s.offset(i as isize) == '\\' as i32 {
                *(*cab).ws.s.offset(i as isize) = '/' as i32
            }
            i = i.wrapping_add(1)
        }
        archive_entry_copy_pathname_w(entry, (*cab).ws.s);
    };
}
/*
 * Read CFHEADER, CFFOLDER and CFFILE.
 */
unsafe extern "C" fn cab_read_header(mut a: *mut archive_read) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut cab: *mut cab = 0 as *mut cab;
    let mut hd: *mut cfheader = 0 as *mut cfheader;
    let mut bytes: size_t = 0;
    let mut used: size_t = 0;
    let mut len: ssize_t = 0;
    let mut skip: int64_t = 0;
    let mut err: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cur_folder: libc::c_int = 0;
    let mut prev_folder: libc::c_int = 0;
    let mut offset32: uint32_t = 0;
    (*a).archive.archive_format = ARCHIVE_FORMAT_CAB;
    if (*a).archive.archive_format_name.is_null() {
        (*a).archive.archive_format_name = b"CAB\x00" as *const u8 as *const libc::c_char
    }
    p = __archive_read_ahead(a, 42 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_uchar;
    if p.is_null() {
        return truncated_error(a);
    }
    cab = (*(*a).format).data as *mut cab;
    if (*cab).found_header as libc::c_int == 0 as libc::c_int
        && *p.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
    {
        /* This is an executable?  Must be self-extracting... */
        err = cab_skip_sfx(a);
        if err < ARCHIVE_WARN {
            return err;
        }
        /* Re-read header after processing the SFX. */
        p = __archive_read_ahead(a, 42 as libc::c_int as size_t, NULL as *mut ssize_t)
            as *const libc::c_uchar;
        if p.is_null() {
            return truncated_error(a);
        }
    }
    (*cab).cab_offset = 0 as libc::c_int as int64_t;
    /*
     * Read CFHEADER.
     */
    hd = &mut (*cab).cfheader; /* Avoid compiling warning. */
    if *p.offset((CFHEADER_signature + 0 as libc::c_int) as isize) as libc::c_int != 'M' as i32
        || *p.offset((CFHEADER_signature + 1 as libc::c_int) as isize) as libc::c_int != 'S' as i32
        || *p.offset((CFHEADER_signature + 2 as libc::c_int) as isize) as libc::c_int != 'C' as i32
        || *p.offset((CFHEADER_signature + 3 as libc::c_int) as isize) as libc::c_int != 'F' as i32
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Couldn\'t find out CAB header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*hd).total_bytes =
        archive_le32dec(p.offset(CFHEADER_cbCabinet as isize) as *const libc::c_void);
    (*hd).files_offset =
        archive_le32dec(p.offset(CFHEADER_coffFiles as isize) as *const libc::c_void);
    (*hd).minor = *p.offset(CFHEADER_versionMinor as isize);
    (*hd).major = *p.offset(CFHEADER_versionMajor as isize);
    (*hd).folder_count =
        archive_le16dec(p.offset(CFHEADER_cFolders as isize) as *const libc::c_void);
    if !((*hd).folder_count as libc::c_int == 0 as libc::c_int) {
        (*hd).file_count =
            archive_le16dec(p.offset(CFHEADER_cFiles as isize) as *const libc::c_void);
        if !((*hd).file_count as libc::c_int == 0 as libc::c_int) {
            (*hd).flags = archive_le16dec(p.offset(CFHEADER_flags as isize) as *const libc::c_void);
            (*hd).setid = archive_le16dec(p.offset(CFHEADER_setID as isize) as *const libc::c_void);
            (*hd).cabinet =
                archive_le16dec(p.offset(CFHEADER_iCabinet as isize) as *const libc::c_void);
            used = (CFHEADER_iCabinet + 2 as libc::c_int) as size_t;
            if (*hd).flags as libc::c_int & RESERVE_PRESENT != 0 {
                let mut cfheader: uint16_t = 0;
                cfheader =
                    archive_le16dec(p.offset(CFHEADER_cbCFHeader as isize) as *const libc::c_void);
                if cfheader as libc::c_uint > 60000 as libc::c_uint {
                    current_block = 3913466651192509181;
                } else {
                    (*hd).cffolder = *p.offset(CFHEADER_cbCFFolder as isize);
                    (*hd).cfdata = *p.offset(CFHEADER_cbCFData as isize);
                    /* abReserve */
                    used = (used as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                        as size_t as size_t; /* cbCFHeader, cbCFFolder and cbCFData */
                    used = (used as libc::c_ulong).wrapping_add(cfheader as libc::c_ulong) as size_t
                        as size_t;
                    current_block = 9007357115414505193;
                }
            } else {
                (*hd).cffolder = 0 as libc::c_int as libc::c_uchar;
                current_block = 9007357115414505193;
            }
            match current_block {
                3913466651192509181 => {}
                _ => {
                    if (*hd).flags as libc::c_int & PREV_CABINET != 0 {
                        /* How many bytes are used for szCabinetPrev. */
                        p = __archive_read_ahead(
                            a,
                            used.wrapping_add(256 as libc::c_int as libc::c_ulong),
                            NULL as *mut ssize_t,
                        ) as *const libc::c_uchar;
                        if p.is_null() {
                            return truncated_error(a);
                        }
                        len = cab_strnlen(p.offset(used as isize), 255 as libc::c_int as size_t);
                        if len <= 0 as libc::c_int as libc::c_long {
                            current_block = 3913466651192509181;
                        } else {
                            used = (used as libc::c_ulong).wrapping_add(
                                (len + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                            ) as size_t as size_t;
                            /* How many bytes are used for szDiskPrev. */
                            p = __archive_read_ahead(
                                a,
                                used.wrapping_add(256 as libc::c_int as libc::c_ulong),
                                NULL as *mut ssize_t,
                            ) as *const libc::c_uchar;
                            if p.is_null() {
                                return truncated_error(a);
                            }
                            len =
                                cab_strnlen(p.offset(used as isize), 255 as libc::c_int as size_t);
                            if len <= 0 as libc::c_int as libc::c_long {
                                current_block = 3913466651192509181;
                            } else {
                                used = (used as libc::c_ulong).wrapping_add(
                                    (len + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                ) as size_t as size_t;
                                current_block = 2989495919056355252;
                            }
                        }
                    } else {
                        current_block = 2989495919056355252;
                    }
                    match current_block {
                        3913466651192509181 => {}
                        _ => {
                            if (*hd).flags as libc::c_int & NEXT_CABINET != 0 {
                                /* How many bytes are used for szCabinetNext. */
                                p = __archive_read_ahead(
                                    a,
                                    used.wrapping_add(256 as libc::c_int as libc::c_ulong),
                                    NULL as *mut ssize_t,
                                ) as *const libc::c_uchar;
                                if p.is_null() {
                                    return truncated_error(a);
                                }
                                len = cab_strnlen(
                                    p.offset(used as isize),
                                    255 as libc::c_int as size_t,
                                );
                                if len <= 0 as libc::c_int as libc::c_long {
                                    current_block = 3913466651192509181;
                                } else {
                                    used = (used as libc::c_ulong).wrapping_add(
                                        (len + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                    ) as size_t
                                        as size_t;
                                    /* How many bytes are used for szDiskNext. */
                                    p = __archive_read_ahead(
                                        a,
                                        used.wrapping_add(256 as libc::c_int as libc::c_ulong),
                                        NULL as *mut ssize_t,
                                    )
                                        as *const libc::c_uchar;
                                    if p.is_null() {
                                        return truncated_error(a);
                                    }
                                    len = cab_strnlen(
                                        p.offset(used as isize),
                                        255 as libc::c_int as size_t,
                                    );
                                    if len <= 0 as libc::c_int as libc::c_long {
                                        current_block = 3913466651192509181;
                                    } else {
                                        used = (used as libc::c_ulong).wrapping_add(
                                            (len + 1 as libc::c_int as libc::c_long)
                                                as libc::c_ulong,
                                        ) as size_t
                                            as size_t;
                                        current_block = 6072622540298447352;
                                    }
                                }
                            } else {
                                current_block = 6072622540298447352;
                            }
                            match current_block {
                                3913466651192509181 => {}
                                _ => {
                                    __archive_read_consume(a, used as int64_t);
                                    (*cab).cab_offset = ((*cab).cab_offset as libc::c_ulong)
                                        .wrapping_add(used)
                                        as int64_t
                                        as int64_t;
                                    used = 0 as libc::c_int as size_t;
                                    /*
                                     * Read CFFOLDER.
                                     */
                                    (*hd).folder_array = calloc(
                                        (*hd).folder_count as libc::c_ulong,
                                        ::std::mem::size_of::<cffolder>() as libc::c_ulong,
                                    )
                                        as *mut cffolder;
                                    if (*hd).folder_array.is_null() {
                                        current_block = 2643310281128592672;
                                    } else {
                                        bytes = 8 as libc::c_int as size_t;
                                        if (*hd).flags as libc::c_int & RESERVE_PRESENT != 0 {
                                            bytes = (bytes as libc::c_ulong)
                                                .wrapping_add((*hd).cffolder as libc::c_ulong)
                                                as size_t
                                                as size_t
                                        }
                                        bytes = (bytes as libc::c_ulong)
                                            .wrapping_mul((*hd).folder_count as libc::c_ulong)
                                            as size_t
                                            as size_t;
                                        p = __archive_read_ahead(a, bytes, NULL as *mut ssize_t)
                                            as *const libc::c_uchar;
                                        if p.is_null() {
                                            return truncated_error(a);
                                        }
                                        offset32 = 0 as libc::c_int as uint32_t;
                                        i = 0 as libc::c_int;
                                        loop {
                                            if !(i < (*hd).folder_count as libc::c_int) {
                                                current_block = 12027283704867122503;
                                                break;
                                            }
                                            let mut folder: *mut cffolder =
                                                &mut *(*hd).folder_array.offset(i as isize)
                                                    as *mut cffolder;
                                            (*folder).cfdata_offset_in_cab = archive_le32dec(
                                                p.offset(CFFOLDER_coffCabStart as isize)
                                                    as *const libc::c_void,
                                            );
                                            (*folder).cfdata_count = archive_le16dec(
                                                p.offset(CFFOLDER_cCFData as isize)
                                                    as *const libc::c_void,
                                            );
                                            (*folder).comptype = (archive_le16dec(
                                                p.offset(CFFOLDER_typeCompress as isize)
                                                    as *const libc::c_void,
                                            )
                                                as libc::c_int
                                                & 0xf as libc::c_int)
                                                as uint16_t;
                                            (*folder).compdata = (archive_le16dec(
                                                p.offset(CFFOLDER_typeCompress as isize)
                                                    as *const libc::c_void,
                                            )
                                                as libc::c_int
                                                >> 8 as libc::c_int)
                                                as uint16_t;
                                            /* Get a compression name. */
                                            if ((*folder).comptype as libc::c_ulong)
                                                < (::std::mem::size_of::<[*const libc::c_char; 4]>()
                                                    as libc::c_ulong)
                                                    .wrapping_div(::std::mem::size_of::<
                                                        *const libc::c_char,
                                                    >(
                                                    )
                                                        as libc::c_ulong)
                                            {
                                                (*folder).compname =
                                                    compression_name[(*folder).comptype as usize]
                                            } else {
                                                (*folder).compname = b"UNKNOWN\x00" as *const u8
                                                    as *const libc::c_char
                                            } /* abReserve */
                                            p = p.offset(8 as libc::c_int as isize);
                                            used = (used as libc::c_ulong)
                                                .wrapping_add(8 as libc::c_int as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                            if (*hd).flags as libc::c_int & RESERVE_PRESENT != 0 {
                                                p = p
                                                    .offset((*hd).cffolder as libc::c_int as isize);
                                                used = (used as libc::c_ulong)
                                                    .wrapping_add((*hd).cffolder as libc::c_ulong)
                                                    as size_t
                                                    as size_t
                                            }
                                            /*
                                             * Sanity check if each data is acceptable.
                                             */
                                            if offset32 >= (*folder).cfdata_offset_in_cab {
                                                current_block = 3913466651192509181;
                                                break;
                                            }
                                            offset32 = (*folder).cfdata_offset_in_cab;
                                            /* Set a request to initialize zlib for the CFDATA of
                                             * this folder. */
                                            (*folder).decompress_init =
                                                0 as libc::c_int as libc::c_char;
                                            i += 1
                                        }
                                        match current_block {
                                            3913466651192509181 => {}
                                            _ => {
                                                __archive_read_consume(a, used as int64_t);
                                                (*cab).cab_offset = ((*cab).cab_offset
                                                    as libc::c_ulong)
                                                    .wrapping_add(used)
                                                    as int64_t
                                                    as int64_t;
                                                /*
                                                 * Read CFFILE.
                                                 */
                                                /* Seek read pointer to the offset of CFFILE if needed. */
                                                skip = (*hd).files_offset as int64_t
                                                    - (*cab).cab_offset;
                                                if skip < 0 as libc::c_int as libc::c_long {
                                                    archive_set_error(
                                                        &mut (*a).archive as *mut archive,
                                                        ARCHIVE_ERRNO_MISC,
                                                        b"Invalid offset of CFFILE %jd < %jd\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        (*hd).files_offset as intmax_t,
                                                        (*cab).cab_offset,
                                                    );
                                                    return -(30 as libc::c_int);
                                                }
                                                if skip != 0 {
                                                    __archive_read_consume(a, skip);
                                                    (*cab).cab_offset += skip
                                                }
                                                /* Allocate memory for CFDATA */
                                                (*hd).file_array = calloc(
                                                    (*hd).file_count as libc::c_ulong,
                                                    ::std::mem::size_of::<cffile>()
                                                        as libc::c_ulong,
                                                )
                                                    as *mut cffile;
                                                if (*hd).file_array.is_null() {
                                                    current_block = 2643310281128592672;
                                                } else {
                                                    prev_folder = -(1 as libc::c_int);
                                                    i = 0 as libc::c_int;
                                                    loop {
                                                        if !(i < (*hd).file_count as libc::c_int) {
                                                            current_block = 9343041660989783267;
                                                            break;
                                                        }
                                                        let mut file: *mut cffile = &mut *(*hd)
                                                            .file_array
                                                            .offset(i as isize)
                                                            as *mut cffile;
                                                        let mut avail: ssize_t = 0;
                                                        p = __archive_read_ahead(
                                                            a,
                                                            16 as libc::c_int as size_t,
                                                            NULL as *mut ssize_t,
                                                        )
                                                            as *const libc::c_uchar;
                                                        if p.is_null() {
                                                            return truncated_error(a);
                                                        }
                                                        (*file).uncompressed_size = archive_le32dec(
                                                            p.offset(CFFILE_cbFile as isize)
                                                                as *const libc::c_void,
                                                        );
                                                        (*file).offset =
                                                            archive_le32dec(p.offset(
                                                                CFFILE_uoffFolderStart as isize,
                                                            )
                                                                as *const libc::c_void);
                                                        (*file).folder = archive_le16dec(
                                                            p.offset(CFFILE_iFolder as isize)
                                                                as *const libc::c_void,
                                                        );
                                                        (*file).mtime = cab_dos_time(
                                                            p.offset(CFFILE_date_time as isize),
                                                        );
                                                        (*file).attr = archive_le16dec(
                                                            p.offset(CFFILE_attribs as isize)
                                                                as *const libc::c_void,
                                                        )
                                                            as uint8_t;
                                                        __archive_read_consume(
                                                            a,
                                                            16 as libc::c_int as int64_t,
                                                        );
                                                        (*cab).cab_offset +=
                                                            16 as libc::c_int as libc::c_long;
                                                        p = cab_read_ahead_remaining(
                                                            a,
                                                            256 as libc::c_int as size_t,
                                                            &mut avail,
                                                        )
                                                            as *const libc::c_uchar;
                                                        if p.is_null() {
                                                            return truncated_error(a);
                                                        }
                                                        len = cab_strnlen(
                                                            p,
                                                            (avail
                                                                - 1 as libc::c_int as libc::c_long)
                                                                as size_t,
                                                        );
                                                        if len <= 0 as libc::c_int as libc::c_long {
                                                            current_block = 3913466651192509181;
                                                            break;
                                                        }
                                                        /* Copy a pathname.  */
                                                        (*file).pathname.s =
                                                            NULL as *mut libc::c_char;
                                                        (*file).pathname.length =
                                                            0 as libc::c_int as size_t;
                                                        (*file).pathname.buffer_length =
                                                            0 as libc::c_int as size_t;
                                                        (*file).pathname.length =
                                                            0 as libc::c_int as size_t;
                                                        archive_strncat(
                                                            &mut (*file).pathname,
                                                            p as *const libc::c_void,
                                                            len as size_t,
                                                        );
                                                        __archive_read_consume(
                                                            a,
                                                            len + 1 as libc::c_int as libc::c_long,
                                                        );
                                                        (*cab).cab_offset +=
                                                            len + 1 as libc::c_int as libc::c_long;
                                                        /*
                                                         * Sanity check if each data is acceptable.
                                                         */
                                                        if (*file).uncompressed_size
                                                            > 0x7fff8000 as libc::c_int
                                                                as libc::c_uint
                                                        {
                                                            current_block = 3913466651192509181; /* Too large */
                                                            break; /* Too large */
                                                        }
                                                        if ((*file).offset as int64_t
                                                            + (*file).uncompressed_size as int64_t)
                                                            as libc::c_longlong
                                                            > 0x7fff8000 as libc::c_longlong
                                                        {
                                                            current_block = 3913466651192509181;
                                                            break;
                                                        }
                                                        match (*file).folder as libc::c_int {
                                                            iFoldCONTINUED_TO_NEXT => {
                                                                /* This must be last file in a folder. */
                                                                if i != (*hd).file_count
                                                                    as libc::c_int
                                                                    - 1 as libc::c_int
                                                                {
                                                                    current_block =
                                                                        3913466651192509181;
                                                                    break;
                                                                }
                                                                cur_folder = (*hd).folder_count
                                                                    as libc::c_int
                                                                    - 1 as libc::c_int;
                                                                current_block =
                                                                    17392506108461345148;
                                                            }
                                                            iFoldCONTINUED_PREV_AND_NEXT => {
                                                                /* This must be only one file in a folder. */
                                                                if (*hd).file_count as libc::c_int
                                                                    != 1 as libc::c_int
                                                                {
                                                                    current_block =
                                                                        3913466651192509181;
                                                                    break;
                                                                }
                                                                /* FALL THROUGH */
                                                                current_block = 7219988385716139285;
                                                            }
                                                            iFoldCONTINUED_FROM_PREV => {
                                                                current_block = 7219988385716139285;
                                                            }
                                                            _ => {
                                                                if (*file).folder as libc::c_int
                                                                    >= (*hd).folder_count
                                                                        as libc::c_int
                                                                {
                                                                    current_block =
                                                                        3913466651192509181;
                                                                    break;
                                                                }
                                                                cur_folder =
                                                                    (*file).folder as libc::c_int;
                                                                current_block =
                                                                    17392506108461345148;
                                                            }
                                                        }
                                                        match current_block {
                                                            7219988385716139285 =>
                                                            /* This must be first file in a folder. */
                                                            {
                                                                if i != 0 as libc::c_int {
                                                                    current_block =
                                                                        3913466651192509181;
                                                                    break;
                                                                }
                                                                cur_folder = 0 as libc::c_int;
                                                                prev_folder = cur_folder;
                                                                offset32 = (*file).offset
                                                            }
                                                            _ => {}
                                                        }
                                                        /* Dot not back track. */
                                                        if cur_folder < prev_folder {
                                                            current_block = 3913466651192509181;
                                                            break;
                                                        }
                                                        if cur_folder != prev_folder {
                                                            offset32 = 0 as libc::c_int as uint32_t
                                                        }
                                                        prev_folder = cur_folder;
                                                        /* Make sure there are not any blanks from last file
                                                         * contents. */
                                                        if offset32 != (*file).offset {
                                                            current_block = 3913466651192509181;
                                                            break;
                                                        }
                                                        offset32 = (offset32 as libc::c_uint)
                                                            .wrapping_add((*file).uncompressed_size)
                                                            as uint32_t
                                                            as uint32_t;
                                                        /* CFDATA is available for file contents. */
                                                        if (*file).uncompressed_size
                                                            > 0 as libc::c_int as libc::c_uint
                                                            && (*(*hd)
                                                                .folder_array
                                                                .offset(cur_folder as isize))
                                                            .cfdata_count
                                                                as libc::c_int
                                                                == 0 as libc::c_int
                                                        {
                                                            current_block = 3913466651192509181;
                                                            break;
                                                        }
                                                        i += 1
                                                    }
                                                    match current_block {
                                                        3913466651192509181 => {}
                                                        _ => {
                                                            if (*hd).cabinet as libc::c_int
                                                                != 0 as libc::c_int
                                                                || (*hd).flags as libc::c_int
                                                                    & (PREV_CABINET | NEXT_CABINET)
                                                                    != 0
                                                            {
                                                                archive_set_error(&mut (*a).archive
                                                                                      as
                                                                                      *mut archive,
                                                                                  ARCHIVE_ERRNO_FILE_FORMAT,
                                                                                  b"Multivolume cabinet file is unsupported\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char);
                                                                return -(20 as libc::c_int);
                                                            }
                                                            return 0 as libc::c_int;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    match current_block {
                                        3913466651192509181 => {}
                                        _ => {
                                            archive_set_error(
                                                &mut (*a).archive as *mut archive,
                                                ENOMEM,
                                                b"Can\'t allocate memory for CAB data\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                            );
                                            return -(30 as libc::c_int);
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
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Invalid CAB header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn archive_read_format_cab_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut cab: *mut cab = 0 as *mut cab;
    let mut hd: *mut cfheader = 0 as *mut cfheader;
    let mut prev_folder: *mut cffolder = 0 as *mut cffolder;
    let mut file: *mut cffile = 0 as *mut cffile;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut err: libc::c_int = ARCHIVE_OK;
    let mut r: libc::c_int = 0;
    cab = (*(*a).format).data as *mut cab;
    if (*cab).found_header as libc::c_int == 0 as libc::c_int {
        err = cab_read_header(a);
        if err < ARCHIVE_WARN {
            return err;
        }
        /* We've found the header. */
        (*cab).found_header = 1 as libc::c_int as libc::c_char
    }
    hd = &mut (*cab).cfheader;
    if (*hd).file_index >= (*hd).file_count as libc::c_int {
        (*cab).end_of_archive = 1 as libc::c_int as libc::c_char;
        return 1 as libc::c_int;
    }
    let fresh0 = (*hd).file_index;
    (*hd).file_index = (*hd).file_index + 1;
    file = &mut *(*hd).file_array.offset(fresh0 as isize) as *mut cffile;
    (*cab).end_of_entry = 0 as libc::c_int as libc::c_char;
    (*cab).end_of_entry_cleanup = 0 as libc::c_int as libc::c_char;
    (*cab).entry_compressed_bytes_read = 0 as libc::c_int as int64_t;
    (*cab).entry_uncompressed_bytes_read = 0 as libc::c_int as int64_t;
    (*cab).entry_unconsumed = 0 as libc::c_int as int64_t;
    (*cab).entry_cffile = file;
    /*
     * Choose a proper folder.
     */
    prev_folder = (*cab).entry_cffolder;
    match (*file).folder as libc::c_int {
        iFoldCONTINUED_FROM_PREV | iFoldCONTINUED_PREV_AND_NEXT => {
            (*cab).entry_cffolder =
                &mut *(*hd).folder_array.offset(0 as libc::c_int as isize) as *mut cffolder
        }
        iFoldCONTINUED_TO_NEXT => {
            (*cab).entry_cffolder = &mut *(*hd)
                .folder_array
                .offset(((*hd).folder_count as libc::c_int - 1 as libc::c_int) as isize)
                as *mut cffolder
        }
        _ => {
            (*cab).entry_cffolder =
                &mut *(*hd).folder_array.offset((*file).folder as isize) as *mut cffolder
        }
    }
    /* If a cffolder of this file is changed, reset a cfdata to read
     * file contents from next cfdata. */
    if prev_folder != (*cab).entry_cffolder {
        (*cab).entry_cfdata = NULL as *mut cfdata
    }
    /* If a pathname is UTF-8, prepare a string conversion object
     * for UTF-8 and use it. */
    if (*file).attr as libc::c_int & ATTR_NAME_IS_UTF != 0 {
        if (*cab).sconv_utf8.is_null() {
            (*cab).sconv_utf8 = archive_string_conversion_from_charset(
                &mut (*a).archive,
                b"UTF-8\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*cab).sconv_utf8.is_null() {
                return -(30 as libc::c_int);
            }
        }
        sconv = (*cab).sconv_utf8
    } else if !(*cab).sconv.is_null() {
        /* Choose the conversion specified by the option. */
        sconv = (*cab).sconv
    } else {
        /* Choose the default conversion. */
        if (*cab).init_default_conversion == 0 {
            (*cab).sconv_default = archive_string_default_conversion_for_read(&mut (*a).archive);
            (*cab).init_default_conversion = 1 as libc::c_int
        }
        sconv = (*cab).sconv_default
    }
    /*
     * Set a default value and common data
     */
    r = cab_convert_path_separator_1(&mut (*file).pathname, (*file).attr);
    if _archive_entry_copy_pathname_l(entry, (*file).pathname.s, (*file).pathname.length, sconv)
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
        err = ARCHIVE_WARN
    }
    if r < 0 as libc::c_int {
        /* Convert a path separator '\' -> '/' */
        cab_convert_path_separator_2(cab, entry);
    }
    archive_entry_set_size(entry, (*file).uncompressed_size as la_int64_t);
    if (*file).attr as libc::c_int & ATTR_RDONLY != 0 {
        archive_entry_set_mode(
            entry,
            AE_IFREG as mode_t | 0o555 as libc::c_int as libc::c_uint,
        );
    } else {
        archive_entry_set_mode(
            entry,
            AE_IFREG as mode_t | 0o666 as libc::c_int as libc::c_uint,
        );
    }
    archive_entry_set_mtime(entry, (*file).mtime, 0 as libc::c_int as libc::c_long);
    (*cab).entry_bytes_remaining = (*file).uncompressed_size as int64_t;
    (*cab).entry_offset = 0 as libc::c_int as int64_t;
    /* We don't need compress data. */
    if (*file).uncompressed_size == 0 as libc::c_int as libc::c_uint {
        (*cab).end_of_entry = 1 as libc::c_int as libc::c_char;
        (*cab).end_of_entry_cleanup = (*cab).end_of_entry
    }
    /* Set up a more descriptive format name. */
    sprintf(
        (*cab).format_name.as_mut_ptr(),
        b"CAB %d.%d (%s)\x00" as *const u8 as *const libc::c_char,
        (*hd).major as libc::c_int,
        (*hd).minor as libc::c_int,
        (*(*cab).entry_cffolder).compname,
    );
    (*a).archive.archive_format_name = (*cab).format_name.as_mut_ptr();
    return err;
}
unsafe extern "C" fn archive_read_format_cab_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut r: libc::c_int = 0;
    match (*(*cab).entry_cffile).folder as libc::c_int {
        iFoldCONTINUED_FROM_PREV | iFoldCONTINUED_TO_NEXT | iFoldCONTINUED_PREV_AND_NEXT => {
            *buff = NULL as *const libc::c_void;
            *size = 0 as libc::c_int as size_t;
            *offset = 0 as libc::c_int as int64_t;
            archive_clear_error(&mut (*a).archive);
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Cannot restore this file split in multivolume.\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        _ => {}
    }
    if (*cab).read_data_invoked as libc::c_int == 0 as libc::c_int {
        if (*cab).bytes_skipped != 0 {
            if (*cab).entry_cfdata.is_null() {
                r = cab_next_cfdata(a);
                if r < 0 as libc::c_int {
                    return r;
                }
            }
            if cab_consume_cfdata(a, (*cab).bytes_skipped) < 0 as libc::c_int as libc::c_long {
                return -(30 as libc::c_int);
            }
            (*cab).bytes_skipped = 0 as libc::c_int as int64_t
        }
        (*cab).read_data_invoked = 1 as libc::c_int as libc::c_char
    }
    if (*cab).entry_unconsumed != 0 {
        /* Consume as much as the compressor actually used. */
        r = cab_consume_cfdata(a, (*cab).entry_unconsumed) as libc::c_int;
        (*cab).entry_unconsumed = 0 as libc::c_int as int64_t;
        if r < 0 as libc::c_int {
            return r;
        }
    }
    if (*cab).end_of_archive as libc::c_int != 0 || (*cab).end_of_entry as libc::c_int != 0 {
        if (*cab).end_of_entry_cleanup == 0 {
            /* End-of-entry cleanup done. */
            (*cab).end_of_entry_cleanup = 1 as libc::c_int as libc::c_char
        }
        *offset = (*cab).entry_offset;
        *size = 0 as libc::c_int as size_t;
        *buff = NULL as *const libc::c_void;
        return 1 as libc::c_int;
    }
    return cab_read_data(a, buff, size, offset);
}
unsafe extern "C" fn cab_checksum_cfdata_4(
    mut p: *const libc::c_void,
    mut bytes: size_t,
    mut seed: uint32_t,
) -> uint32_t {
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut u32num: libc::c_uint = 0;
    let mut sum: uint32_t = 0;
    u32num = (bytes as libc::c_uint).wrapping_div(4 as libc::c_int as libc::c_uint);
    sum = seed;
    b = p as *const libc::c_uchar;
    while u32num > 0 as libc::c_int as libc::c_uint {
        sum ^= archive_le32dec(b as *const libc::c_void);
        b = b.offset(4 as libc::c_int as isize);
        u32num = u32num.wrapping_sub(1)
    }
    return sum;
}
unsafe extern "C" fn cab_checksum_cfdata(
    mut p: *const libc::c_void,
    mut bytes: size_t,
    mut seed: uint32_t,
) -> uint32_t {
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut sum: uint32_t = 0;
    let mut t: uint32_t = 0;
    sum = cab_checksum_cfdata_4(p, bytes, seed);
    b = p as *const libc::c_uchar;
    b = b.offset((bytes & !(3 as libc::c_int) as libc::c_ulong) as isize);
    t = 0 as libc::c_int as uint32_t;
    let mut current_block_6: u64;
    match bytes & 3 as libc::c_int as libc::c_ulong {
        3 => {
            let fresh1 = b;
            b = b.offset(1);
            t |= (*fresh1 as uint32_t) << 16 as libc::c_int;
            current_block_6 = 9198812510414462111;
        }
        2 => {
            current_block_6 = 9198812510414462111;
        }
        1 => {
            current_block_6 = 1982130065057554431;
        }
        _ => {
            current_block_6 = 1917311967535052937;
        }
    }
    match current_block_6 {
        9198812510414462111 =>
        /* FALL THROUGH */
        {
            let fresh2 = b;
            b = b.offset(1);
            t |= (*fresh2 as uint32_t) << 8 as libc::c_int;
            current_block_6 = 1982130065057554431;
        }
        _ => {}
    }
    match current_block_6 {
        1982130065057554431 =>
        /* FALL THROUGH */
        {
            t |= *b as libc::c_uint
        }
        _ => {}
    }
    sum ^= t;
    return sum;
}
unsafe extern "C" fn cab_checksum_update(mut a: *mut archive_read, mut bytes: size_t) {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = (*cab).entry_cfdata;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut sumbytes: size_t = 0;
    if (*cfdata).sum == 0 as libc::c_int as libc::c_uint
        || (*cfdata).sum_ptr == NULL as *const libc::c_void
    {
        return;
    }
    /*
     * Calculate the sum of this CFDATA.
     * Make sure CFDATA must be calculated in four bytes.
     */
    p = (*cfdata).sum_ptr as *const libc::c_uchar;
    sumbytes = bytes;
    if (*cfdata).sum_extra_avail != 0 {
        while (*cfdata).sum_extra_avail < 4 as libc::c_int
            && sumbytes > 0 as libc::c_int as libc::c_ulong
        {
            let fresh3 = p;
            p = p.offset(1);
            let fresh4 = (*cfdata).sum_extra_avail;
            (*cfdata).sum_extra_avail = (*cfdata).sum_extra_avail + 1;
            (*cfdata).sum_extra[fresh4 as usize] = *fresh3;
            sumbytes = sumbytes.wrapping_sub(1)
        }
        if (*cfdata).sum_extra_avail == 4 as libc::c_int {
            (*cfdata).sum_calculated = cab_checksum_cfdata_4(
                (*cfdata).sum_extra.as_mut_ptr() as *const libc::c_void,
                4 as libc::c_int as size_t,
                (*cfdata).sum_calculated,
            );
            (*cfdata).sum_extra_avail = 0 as libc::c_int
        }
    }
    if sumbytes != 0 {
        let mut odd: libc::c_int = (sumbytes & 3 as libc::c_int as libc::c_ulong) as libc::c_int;
        if sumbytes.wrapping_sub(odd as libc::c_ulong) > 0 as libc::c_int as libc::c_ulong {
            (*cfdata).sum_calculated = cab_checksum_cfdata_4(
                p as *const libc::c_void,
                sumbytes.wrapping_sub(odd as libc::c_ulong),
                (*cfdata).sum_calculated,
            )
        }
        if odd != 0 {
            memcpy(
                (*cfdata).sum_extra.as_mut_ptr() as *mut libc::c_void,
                p.offset(sumbytes as isize).offset(-(odd as isize)) as *const libc::c_void,
                odd as libc::c_ulong,
            );
        }
        (*cfdata).sum_extra_avail = odd
    }
    (*cfdata).sum_ptr = NULL as *const libc::c_void;
}
unsafe extern "C" fn cab_checksum_finish(mut a: *mut archive_read) -> libc::c_int {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = (*cab).entry_cfdata;
    let mut l: libc::c_int = 0;
    /* Do not need to compute a sum. */
    if (*cfdata).sum == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    /*
     * Calculate the sum of remaining CFDATA.
     */
    if (*cfdata).sum_extra_avail != 0 {
        (*cfdata).sum_calculated = cab_checksum_cfdata(
            (*cfdata).sum_extra.as_mut_ptr() as *const libc::c_void,
            (*cfdata).sum_extra_avail as size_t,
            (*cfdata).sum_calculated,
        );
        (*cfdata).sum_extra_avail = 0 as libc::c_int
    }
    l = 4 as libc::c_int;
    if (*cab).cfheader.flags as libc::c_int & RESERVE_PRESENT != 0 {
        l += (*cab).cfheader.cfdata as libc::c_int
    }
    (*cfdata).sum_calculated = cab_checksum_cfdata(
        (*cfdata).memimage.offset(CFDATA_cbData as isize) as *const libc::c_void,
        l as size_t,
        (*cfdata).sum_calculated,
    );
    if (*cfdata).sum_calculated != (*cfdata).sum {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Checksum error CFDATA[%d] %x:%x in %d bytes\x00" as *const u8 as *const libc::c_char,
            (*(*cab).entry_cffolder).cfdata_index - 1 as libc::c_int,
            (*cfdata).sum,
            (*cfdata).sum_calculated,
            (*cfdata).compressed_size as libc::c_int,
        );
        return -(25 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/*
 * Read CFDATA if needed.
 */
unsafe extern "C" fn cab_next_cfdata(mut a: *mut archive_read) -> libc::c_int {
    let mut current_block: u64;
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = (*cab).entry_cfdata;
    /* There are remaining bytes in current CFDATA, use it first. */
    if !cfdata.is_null() && (*cfdata).uncompressed_bytes_remaining as libc::c_int > 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if cfdata.is_null() {
        let mut skip: int64_t = 0;
        (*(*cab).entry_cffolder).cfdata_index = 0 as libc::c_int;
        /* Seek read pointer to the offset of CFDATA if needed. */
        skip = (*(*cab).entry_cffolder).cfdata_offset_in_cab as libc::c_long - (*cab).cab_offset;
        if skip < 0 as libc::c_int as libc::c_long {
            let mut folder_index: libc::c_int = 0;
            match (*(*cab).entry_cffile).folder as libc::c_int {
                iFoldCONTINUED_FROM_PREV | iFoldCONTINUED_PREV_AND_NEXT => {
                    folder_index = 0 as libc::c_int
                }
                iFoldCONTINUED_TO_NEXT => {
                    folder_index = (*cab).cfheader.folder_count as libc::c_int - 1 as libc::c_int
                }
                _ => folder_index = (*(*cab).entry_cffile).folder as libc::c_int,
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid offset of CFDATA in folder(%d) %jd < %jd\x00" as *const u8
                    as *const libc::c_char,
                folder_index,
                (*(*cab).entry_cffolder).cfdata_offset_in_cab as intmax_t,
                (*cab).cab_offset,
            );
            return -(30 as libc::c_int);
        }
        if skip > 0 as libc::c_int as libc::c_long {
            if __archive_read_consume(a, skip) < 0 as libc::c_int as libc::c_long {
                return -(30 as libc::c_int);
            }
            (*cab).cab_offset = (*(*cab).entry_cffolder).cfdata_offset_in_cab as int64_t
        }
    }
    /*
     * Read a CFDATA.
     */
    if (*(*cab).entry_cffolder).cfdata_index < (*(*cab).entry_cffolder).cfdata_count as libc::c_int
    {
        let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
        let mut l: libc::c_int = 0;
        cfdata = &mut (*(*cab).entry_cffolder).cfdata;
        (*(*cab).entry_cffolder).cfdata_index += 1;
        (*cab).entry_cfdata = cfdata;
        (*cfdata).sum_calculated = 0 as libc::c_int as uint32_t;
        (*cfdata).sum_extra_avail = 0 as libc::c_int;
        (*cfdata).sum_ptr = NULL as *const libc::c_void;
        l = 8 as libc::c_int;
        if (*cab).cfheader.flags as libc::c_int & RESERVE_PRESENT != 0 {
            l += (*cab).cfheader.cfdata as libc::c_int
        }
        p = __archive_read_ahead(a, l as size_t, NULL as *mut ssize_t) as *const libc::c_uchar;
        if p.is_null() {
            return truncated_error(a);
        }
        (*cfdata).sum = archive_le32dec(p.offset(CFDATA_csum as isize) as *const libc::c_void);
        (*cfdata).compressed_size =
            archive_le16dec(p.offset(CFDATA_cbData as isize) as *const libc::c_void);
        (*cfdata).compressed_bytes_remaining = (*cfdata).compressed_size;
        (*cfdata).uncompressed_size =
            archive_le16dec(p.offset(CFDATA_cbUncomp as isize) as *const libc::c_void);
        (*cfdata).uncompressed_bytes_remaining = (*cfdata).uncompressed_size;
        (*cfdata).uncompressed_avail = 0 as libc::c_int as uint16_t;
        (*cfdata).read_offset = 0 as libc::c_int as uint16_t;
        (*cfdata).unconsumed = 0 as libc::c_int as int64_t;
        /*
         * Sanity check if data size is acceptable.
         */
        if (*cfdata).compressed_size as libc::c_int == 0 as libc::c_int
            || (*cfdata).compressed_size as libc::c_int
                > 0x8000 as libc::c_int + 6144 as libc::c_int
        {
            current_block = 6116957410927263949;
        } else if (*cfdata).uncompressed_size as libc::c_int > 0x8000 as libc::c_int {
            current_block = 6116957410927263949;
        } else {
            if (*cfdata).uncompressed_size as libc::c_int == 0 as libc::c_int {
                match (*(*cab).entry_cffile).folder as libc::c_int {
                    iFoldCONTINUED_PREV_AND_NEXT | iFoldCONTINUED_TO_NEXT => {
                        current_block = 1434579379687443766;
                    }
                    iFoldCONTINUED_FROM_PREV | _ => {
                        current_block = 6116957410927263949;
                    }
                }
            } else {
                current_block = 1434579379687443766;
            }
            match current_block {
                6116957410927263949 => {}
                _ =>
                /* If CFDATA is not last in a folder, an uncompressed
                 * size must be 0x8000(32KBi) */
                {
                    if (*(*cab).entry_cffolder).cfdata_index
                        < (*(*cab).entry_cffolder).cfdata_count as libc::c_int
                        && (*cfdata).uncompressed_size as libc::c_int != 0x8000 as libc::c_int
                    {
                        current_block = 6116957410927263949;
                    } else if (*(*cab).entry_cffolder).comptype as libc::c_int == COMPTYPE_NONE
                        && (*cfdata).compressed_size as libc::c_int
                            != (*cfdata).uncompressed_size as libc::c_int
                    {
                        current_block = 6116957410927263949;
                    } else {
                        /* A compressed data size and an uncompressed data size must
                         * be the same in no compression mode. */
                        /*
                         * Save CFDATA image for sum check.
                         */
                        if (*cfdata).memimage_size < l as size_t {
                            free((*cfdata).memimage as *mut libc::c_void);
                            (*cfdata).memimage = malloc(l as libc::c_ulong) as *mut libc::c_uchar;
                            if (*cfdata).memimage.is_null() {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ENOMEM,
                                    b"Can\'t allocate memory for CAB data\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                return -(30 as libc::c_int);
                            }
                            (*cfdata).memimage_size = l as size_t
                        }
                        memcpy(
                            (*cfdata).memimage as *mut libc::c_void,
                            p as *const libc::c_void,
                            l as libc::c_ulong,
                        );
                        /* Consume bytes as much as we used. */
                        __archive_read_consume(a, l as int64_t);
                        (*cab).cab_offset += l as libc::c_long;
                        current_block = 16779030619667747692;
                    }
                }
            }
        }
        match current_block {
            16779030619667747692 => {}
            _ => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Invalid CFDATA\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
    } else if (*(*cab).entry_cffolder).cfdata_count as libc::c_int > 0 as libc::c_int {
        /* Run out of all CFDATA in a folder. */
        (*cfdata).compressed_size = 0 as libc::c_int as uint16_t;
        (*cfdata).uncompressed_size = 0 as libc::c_int as uint16_t;
        (*cfdata).compressed_bytes_remaining = 0 as libc::c_int as uint16_t;
        (*cfdata).uncompressed_bytes_remaining = 0 as libc::c_int as uint16_t
    } else {
        /* Current folder does not have any CFDATA. */
        cfdata = &mut (*(*cab).entry_cffolder).cfdata;
        (*cab).entry_cfdata = cfdata;
        memset(
            cfdata as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<cfdata>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
/*
 * Read ahead CFDATA.
 */
unsafe extern "C" fn cab_read_ahead_cfdata(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut err: libc::c_int = 0;
    err = cab_next_cfdata(a);
    if err < ARCHIVE_OK {
        *avail = err as ssize_t;
        return 0 as *const libc::c_void;
    }
    match (*(*cab).entry_cffolder).comptype as libc::c_int {
        COMPTYPE_NONE => return cab_read_ahead_cfdata_none(a, avail),
        COMPTYPE_MSZIP => return cab_read_ahead_cfdata_deflate(a, avail),
        COMPTYPE_LZX => return cab_read_ahead_cfdata_lzx(a, avail),
        _ => {
            /* Unsupported compression. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unsupported CAB compression : %s\x00" as *const u8 as *const libc::c_char,
                (*(*cab).entry_cffolder).compname,
            );
            *avail = ARCHIVE_FAILED as ssize_t;
            return 0 as *const libc::c_void;
        }
    };
}
/*
 * Read ahead CFDATA as uncompressed data.
 */
unsafe extern "C" fn cab_read_ahead_cfdata_none(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut d: *const libc::c_void = 0 as *const libc::c_void;
    cfdata = (*cab).entry_cfdata;
    /*
     * Note: '1' here is a performance optimization.
     * Recall that the decompression layer returns a count of
     * available bytes; asking for more than that forces the
     * decompressor to combine reads by copying data.
     */
    d = __archive_read_ahead(a, 1 as libc::c_int as size_t, avail);
    if *avail <= 0 as libc::c_int as libc::c_long {
        *avail = truncated_error(a) as ssize_t;
        return 0 as *const libc::c_void;
    }
    if *avail > (*cfdata).uncompressed_bytes_remaining as libc::c_long {
        *avail = (*cfdata).uncompressed_bytes_remaining as ssize_t
    }
    (*cfdata).uncompressed_avail = (*cfdata).uncompressed_size;
    (*cfdata).unconsumed = *avail;
    (*cfdata).sum_ptr = d;
    return d;
}
/*
 * Read ahead CFDATA as deflate data.
 */
unsafe extern "C" fn cab_read_ahead_cfdata_deflate(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    let mut current_block: u64;
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut d: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    let mut mszip: libc::c_int = 0;
    let mut uavail: uint16_t = 0;
    let mut eod: libc::c_char = 0 as libc::c_int as libc::c_char;
    cfdata = (*cab).entry_cfdata;
    /* If the buffer hasn't been allocated, allocate it now. */
    if (*cab).uncompressed_buffer.is_null() {
        (*cab).uncompressed_buffer_size = 0x8000 as libc::c_int as size_t;
        (*cab).uncompressed_buffer = malloc((*cab).uncompressed_buffer_size) as *mut libc::c_uchar;
        if (*cab).uncompressed_buffer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory for CAB reader\x00" as *const u8 as *const libc::c_char,
            );
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
    }
    uavail = (*cfdata).uncompressed_avail;
    if uavail as libc::c_int == (*cfdata).uncompressed_size as libc::c_int {
        d = (*cab)
            .uncompressed_buffer
            .offset((*cfdata).read_offset as libc::c_int as isize)
            as *const libc::c_void;
        *avail = (uavail as libc::c_int - (*cfdata).read_offset as libc::c_int) as ssize_t;
        return d;
    }
    if (*(*cab).entry_cffolder).decompress_init == 0 {
        (*cab).stream.next_in = NULL as *mut Bytef;
        (*cab).stream.avail_in = 0 as libc::c_int as uInt;
        (*cab).stream.total_in = 0 as libc::c_int as uLong;
        (*cab).stream.next_out = NULL as *mut Bytef;
        (*cab).stream.avail_out = 0 as libc::c_int as uInt;
        (*cab).stream.total_out = 0 as libc::c_int as uLong;
        if (*cab).stream_valid != 0 {
            r = inflateReset(&mut (*cab).stream)
        } else {
            r = inflateInit2_(
                &mut (*cab).stream,
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
                b"Can\'t initialize deflate decompression.\x00" as *const u8 as *const libc::c_char,
            );
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
        /* Stream structure has been set up. */
        (*cab).stream_valid = 1 as libc::c_int as libc::c_char;
        /* We've initialized decompression for this stream. */
        (*(*cab).entry_cffolder).decompress_init = 1 as libc::c_int as libc::c_char
    }
    if (*cfdata).compressed_bytes_remaining as libc::c_int
        == (*cfdata).compressed_size as libc::c_int
    {
        mszip = 2 as libc::c_int
    } else {
        mszip = 0 as libc::c_int
    }
    eod = 0 as libc::c_int as libc::c_char;
    (*cab).stream.total_out = uavail as uLong;
    loop
    /*
     * We always uncompress all data in current CFDATA.
     */
    {
        if !(eod == 0 && (*cab).stream.total_out < (*cfdata).uncompressed_size as libc::c_ulong) {
            current_block = 10778260831612459202;
            break;
        }
        let mut bytes_avail: ssize_t = 0;
        (*cab).stream.next_out = (*cab)
            .uncompressed_buffer
            .offset((*cab).stream.total_out as isize);
        (*cab).stream.avail_out = ((*cfdata).uncompressed_size as libc::c_ulong)
            .wrapping_sub((*cab).stream.total_out) as uInt;
        d = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail);
        if bytes_avail <= 0 as libc::c_int as libc::c_long {
            *avail = truncated_error(a) as ssize_t;
            return 0 as *const libc::c_void;
        }
        if bytes_avail > (*cfdata).compressed_bytes_remaining as libc::c_long {
            bytes_avail = (*cfdata).compressed_bytes_remaining as ssize_t
        }
        /*
         * A bug in zlib.h: stream.next_in should be marked 'const'
         * but isn't (the library never alters data through the
         * next_in pointer, only reads it).  The result: this ugly
         * cast to remove 'const'.
         */
        (*cab).stream.next_in = d as uintptr_t as *mut Bytef;
        (*cab).stream.avail_in = bytes_avail as uInt;
        (*cab).stream.total_in = 0 as libc::c_int as uLong;
        /* Cut out a tow-byte MSZIP signature(0x43, 0x4b). */
        if mszip > 0 as libc::c_int {
            if bytes_avail <= 0 as libc::c_int as libc::c_long {
                current_block = 15417607113809240846;
                break;
            }
            if bytes_avail <= mszip as libc::c_long {
                if mszip == 2 as libc::c_int {
                    if *(*cab).stream.next_in.offset(0 as libc::c_int as isize) as libc::c_int
                        != 0x43 as libc::c_int
                    {
                        current_block = 15417607113809240846;
                        break;
                    }
                    if bytes_avail > 1 as libc::c_int as libc::c_long
                        && *(*cab).stream.next_in.offset(1 as libc::c_int as isize) as libc::c_int
                            != 0x4b as libc::c_int
                    {
                        current_block = 15417607113809240846;
                        break;
                    }
                } else if *(*cab).stream.next_in.offset(0 as libc::c_int as isize) as libc::c_int
                    != 0x4b as libc::c_int
                {
                    current_block = 15417607113809240846;
                    break;
                }
                (*cfdata).unconsumed = bytes_avail;
                (*cfdata).sum_ptr = d;
                if cab_minimum_consume_cfdata(a, (*cfdata).unconsumed)
                    < 0 as libc::c_int as libc::c_long
                {
                    *avail = ARCHIVE_FATAL as ssize_t;
                    return 0 as *const libc::c_void;
                }
                mszip -= bytes_avail as libc::c_int;
                continue;
            } else {
                if mszip == 1 as libc::c_int
                    && *(*cab).stream.next_in.offset(0 as libc::c_int as isize) as libc::c_int
                        != 0x4b as libc::c_int
                {
                    current_block = 15417607113809240846;
                    break;
                }
                if mszip == 2 as libc::c_int
                    && (*(*cab).stream.next_in.offset(0 as libc::c_int as isize) as libc::c_int
                        != 0x43 as libc::c_int
                        || *(*cab).stream.next_in.offset(1 as libc::c_int as isize) as libc::c_int
                            != 0x4b as libc::c_int)
                {
                    current_block = 15417607113809240846;
                    break;
                }
                (*cab).stream.next_in = (*cab).stream.next_in.offset(mszip as isize);
                (*cab).stream.avail_in = ((*cab).stream.avail_in as libc::c_uint)
                    .wrapping_sub(mszip as libc::c_uint)
                    as uInt as uInt;
                (*cab).stream.total_in = ((*cab).stream.total_in as libc::c_ulong)
                    .wrapping_add(mszip as libc::c_ulong)
                    as uLong as uLong;
                mszip = 0 as libc::c_int
            }
        }
        r = inflate(&mut (*cab).stream, 0 as libc::c_int);
        match r {
            Z_OK => {}
            Z_STREAM_END => eod = 1 as libc::c_int as libc::c_char,
            _ => {
                current_block = 8657001954145895731;
                break;
            }
        }
        (*cfdata).unconsumed = (*cab).stream.total_in as int64_t;
        (*cfdata).sum_ptr = d;
        if cab_minimum_consume_cfdata(a, (*cfdata).unconsumed) < 0 as libc::c_int as libc::c_long {
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
    }
    match current_block {
        10778260831612459202 => {
            uavail = (*cab).stream.total_out as uint16_t;
            if (uavail as libc::c_int) < (*cfdata).uncompressed_size as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Invalid uncompressed size (%d < %d)\x00" as *const u8 as *const libc::c_char,
                    uavail as libc::c_int,
                    (*cfdata).uncompressed_size as libc::c_int,
                );
                *avail = ARCHIVE_FATAL as ssize_t;
                return 0 as *const libc::c_void;
            }
            /*
             * Note: I suspect there is a bug in makecab.exe because, in rare
             * case, compressed bytes are still remaining regardless we have
             * gotten all uncompressed bytes, which size is recorded in CFDATA,
             * as much as we need, and we have to use the garbage so as to
             * correctly compute the sum of CFDATA accordingly.
             */
            if (*cfdata).compressed_bytes_remaining as libc::c_int > 0 as libc::c_int {
                let mut bytes_avail_0: ssize_t = 0;
                d = __archive_read_ahead(
                    a,
                    (*cfdata).compressed_bytes_remaining as size_t,
                    &mut bytes_avail_0,
                );
                if bytes_avail_0 <= 0 as libc::c_int as libc::c_long {
                    *avail = truncated_error(a) as ssize_t;
                    return 0 as *const libc::c_void;
                }
                (*cfdata).unconsumed = (*cfdata).compressed_bytes_remaining as int64_t;
                (*cfdata).sum_ptr = d;
                if cab_minimum_consume_cfdata(a, (*cfdata).unconsumed)
                    < 0 as libc::c_int as libc::c_long
                {
                    *avail = ARCHIVE_FATAL as ssize_t;
                    return 0 as *const libc::c_void;
                }
            }
            /*
             * Set dictionary data for decompressing of next CFDATA, which
             * in the same folder. This is why we always do decompress CFDATA
             * even if beginning CFDATA or some of CFDATA are not used in
             * skipping file data.
             */
            if (*(*cab).entry_cffolder).cfdata_index
                < (*(*cab).entry_cffolder).cfdata_count as libc::c_int
            {
                r = inflateReset(&mut (*cab).stream);
                if r != Z_OK {
                    current_block = 8657001954145895731;
                } else {
                    r = inflateSetDictionary(
                        &mut (*cab).stream,
                        (*cab).uncompressed_buffer,
                        (*cfdata).uncompressed_size as uInt,
                    );
                    if r != Z_OK {
                        current_block = 8657001954145895731;
                    } else {
                        current_block = 796174441944384681;
                    }
                }
            } else {
                current_block = 796174441944384681;
            }
            match current_block {
                8657001954145895731 => {}
                _ => {
                    d = (*cab)
                        .uncompressed_buffer
                        .offset((*cfdata).read_offset as libc::c_int as isize)
                        as *const libc::c_void;
                    *avail =
                        (uavail as libc::c_int - (*cfdata).read_offset as libc::c_int) as ssize_t;
                    (*cfdata).uncompressed_avail = uavail;
                    return d;
                }
            }
        }
        15417607113809240846 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"CFDATA incorrect(no MSZIP signature)\x00" as *const u8 as *const libc::c_char,
            );
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
        _ => {}
    }
    match r {
        -4 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory for deflate decompression\x00" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Deflate decompression failed (%d)\x00" as *const u8 as *const libc::c_char,
                r,
            );
        }
    }
    *avail = ARCHIVE_FATAL as ssize_t;
    return 0 as *const libc::c_void;
}
/* HAVE_ZLIB_H */
/* HAVE_ZLIB_H */
unsafe extern "C" fn cab_read_ahead_cfdata_lzx(
    mut a: *mut archive_read,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut d: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    let mut uavail: uint16_t = 0;
    cfdata = (*cab).entry_cfdata;
    /* If the buffer hasn't been allocated, allocate it now. */
    if (*cab).uncompressed_buffer.is_null() {
        (*cab).uncompressed_buffer_size = 0x8000 as libc::c_int as size_t;
        (*cab).uncompressed_buffer = malloc((*cab).uncompressed_buffer_size) as *mut libc::c_uchar;
        if (*cab).uncompressed_buffer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory for CAB reader\x00" as *const u8 as *const libc::c_char,
            );
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
    }
    uavail = (*cfdata).uncompressed_avail;
    if uavail as libc::c_int == (*cfdata).uncompressed_size as libc::c_int {
        d = (*cab)
            .uncompressed_buffer
            .offset((*cfdata).read_offset as libc::c_int as isize)
            as *const libc::c_void;
        *avail = (uavail as libc::c_int - (*cfdata).read_offset as libc::c_int) as ssize_t;
        return d;
    }
    if (*(*cab).entry_cffolder).decompress_init == 0 {
        r = lzx_decode_init(
            &mut (*cab).xstrm,
            (*(*cab).entry_cffolder).compdata as libc::c_int,
        );
        if r != ARCHIVE_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Can\'t initialize LZX decompression.\x00" as *const u8 as *const libc::c_char,
            );
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
        /* We've initialized decompression for this stream. */
        (*(*cab).entry_cffolder).decompress_init = 1 as libc::c_int as libc::c_char
    }
    /* Clean up remaining bits of previous CFDATA. */
    lzx_cleanup_bitstream(&mut (*cab).xstrm);
    (*cab).xstrm.total_out = uavail as int64_t;
    while (*cab).xstrm.total_out < (*cfdata).uncompressed_size as libc::c_long {
        let mut bytes_avail: ssize_t = 0;
        (*cab).xstrm.next_out = (*cab)
            .uncompressed_buffer
            .offset((*cab).xstrm.total_out as isize);
        (*cab).xstrm.avail_out =
            (*cfdata).uncompressed_size as libc::c_long - (*cab).xstrm.total_out;
        d = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail);
        if bytes_avail <= 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated CAB file data\x00" as *const u8 as *const libc::c_char,
            );
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
        if bytes_avail > (*cfdata).compressed_bytes_remaining as libc::c_long {
            bytes_avail = (*cfdata).compressed_bytes_remaining as ssize_t
        }
        (*cab).xstrm.next_in = d as *const libc::c_uchar;
        (*cab).xstrm.avail_in = bytes_avail;
        (*cab).xstrm.total_in = 0 as libc::c_int as int64_t;
        r = lzx_decode(
            &mut (*cab).xstrm,
            ((*cfdata).compressed_bytes_remaining as libc::c_long == bytes_avail) as libc::c_int,
        );
        match r {
            ARCHIVE_OK | ARCHIVE_EOF => {}
            _ => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"LZX decompression failed (%d)\x00" as *const u8 as *const libc::c_char,
                    r,
                );
                *avail = ARCHIVE_FATAL as ssize_t;
                return 0 as *const libc::c_void;
            }
        }
        (*cfdata).unconsumed = (*cab).xstrm.total_in;
        (*cfdata).sum_ptr = d;
        if cab_minimum_consume_cfdata(a, (*cfdata).unconsumed) < 0 as libc::c_int as libc::c_long {
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
    }
    uavail = (*cab).xstrm.total_out as uint16_t;
    /*
     * Make sure a read pointer advances to next CFDATA.
     */
    if (*cfdata).compressed_bytes_remaining as libc::c_int > 0 as libc::c_int {
        let mut bytes_avail_0: ssize_t = 0;
        d = __archive_read_ahead(
            a,
            (*cfdata).compressed_bytes_remaining as size_t,
            &mut bytes_avail_0,
        );
        if bytes_avail_0 <= 0 as libc::c_int as libc::c_long {
            *avail = truncated_error(a) as ssize_t;
            return 0 as *const libc::c_void;
        }
        (*cfdata).unconsumed = (*cfdata).compressed_bytes_remaining as int64_t;
        (*cfdata).sum_ptr = d;
        if cab_minimum_consume_cfdata(a, (*cfdata).unconsumed) < 0 as libc::c_int as libc::c_long {
            *avail = ARCHIVE_FATAL as ssize_t;
            return 0 as *const libc::c_void;
        }
    }
    /*
     * Translation reversal of x86 processor CALL byte sequence(E8).
     */
    lzx_translation(
        &mut (*cab).xstrm,
        (*cab).uncompressed_buffer as *mut libc::c_void,
        (*cfdata).uncompressed_size as size_t,
        (((*(*cab).entry_cffolder).cfdata_index - 1 as libc::c_int) * 0x8000 as libc::c_int)
            as uint32_t,
    );
    d = (*cab)
        .uncompressed_buffer
        .offset((*cfdata).read_offset as libc::c_int as isize) as *const libc::c_void;
    *avail = (uavail as libc::c_int - (*cfdata).read_offset as libc::c_int) as ssize_t;
    (*cfdata).uncompressed_avail = uavail;
    return d;
}
/*
 * Consume CFDATA.
 * We always decompress CFDATA to consume CFDATA as much as we need
 * in uncompressed bytes because all CFDATA in a folder are related
 * so we do not skip any CFDATA without decompressing.
 * Note: If the folder of a CFFILE is iFoldCONTINUED_PREV_AND_NEXT or
 * iFoldCONTINUED_FROM_PREV, we won't decompress because a CFDATA for
 * the CFFILE is remaining bytes of previous Multivolume CAB file.
 */
unsafe extern "C" fn cab_consume_cfdata(
    mut a: *mut archive_read,
    mut consumed_bytes: int64_t,
) -> int64_t {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut cbytes: int64_t = 0;
    let mut rbytes: int64_t = 0;
    let mut err: libc::c_int = 0;
    rbytes = cab_minimum_consume_cfdata(a, consumed_bytes);
    if rbytes < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int) as int64_t;
    }
    cfdata = (*cab).entry_cfdata;
    while rbytes > 0 as libc::c_int as libc::c_long {
        let mut avail: ssize_t = 0;
        if (*cfdata).compressed_size as libc::c_int == 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Invalid CFDATA\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as int64_t;
        }
        cbytes = (*cfdata).uncompressed_bytes_remaining as int64_t;
        if cbytes > rbytes {
            cbytes = rbytes
        }
        rbytes -= cbytes;
        if (*cfdata).uncompressed_avail as libc::c_int == 0 as libc::c_int
            && ((*(*cab).entry_cffile).folder as libc::c_int == iFoldCONTINUED_PREV_AND_NEXT
                || (*(*cab).entry_cffile).folder as libc::c_int == iFoldCONTINUED_FROM_PREV)
        {
            /* We have not read any data yet. */
            if cbytes == (*cfdata).uncompressed_bytes_remaining as libc::c_long {
                /* Skip whole current CFDATA. */
                __archive_read_consume(a, (*cfdata).compressed_size as int64_t);
                (*cab).cab_offset += (*cfdata).compressed_size as libc::c_long;
                (*cfdata).compressed_bytes_remaining = 0 as libc::c_int as uint16_t;
                (*cfdata).uncompressed_bytes_remaining = 0 as libc::c_int as uint16_t;
                err = cab_next_cfdata(a);
                if err < 0 as libc::c_int {
                    return err as int64_t;
                }
                cfdata = (*cab).entry_cfdata;
                if (*cfdata).uncompressed_size as libc::c_int == 0 as libc::c_int {
                    match (*(*cab).entry_cffile).folder as libc::c_int {
                        iFoldCONTINUED_PREV_AND_NEXT
                        | iFoldCONTINUED_TO_NEXT
                        | iFoldCONTINUED_FROM_PREV => rbytes = 0 as libc::c_int as int64_t,
                        _ => {}
                    }
                }
            } else {
                (*cfdata).read_offset = ((*cfdata).read_offset as libc::c_int
                    + cbytes as uint16_t as libc::c_int)
                    as uint16_t;
                (*cfdata).uncompressed_bytes_remaining =
                    ((*cfdata).uncompressed_bytes_remaining as libc::c_int
                        - cbytes as uint16_t as libc::c_int) as uint16_t;
                break;
            }
        } else if cbytes == 0 as libc::c_int as libc::c_long {
            err = cab_next_cfdata(a);
            if err < 0 as libc::c_int {
                return err as int64_t;
            }
            cfdata = (*cab).entry_cfdata;
            if (*cfdata).uncompressed_size as libc::c_int == 0 as libc::c_int {
                match (*(*cab).entry_cffile).folder as libc::c_int {
                    iFoldCONTINUED_PREV_AND_NEXT
                    | iFoldCONTINUED_TO_NEXT
                    | iFoldCONTINUED_FROM_PREV => return -(30 as libc::c_int) as int64_t,
                    _ => {}
                }
            }
        } else {
            while cbytes > 0 as libc::c_int as libc::c_long {
                cab_read_ahead_cfdata(a, &mut avail);
                if avail <= 0 as libc::c_int as libc::c_long {
                    return -(30 as libc::c_int) as int64_t;
                }
                if avail > cbytes {
                    avail = cbytes
                }
                if cab_minimum_consume_cfdata(a, avail) < 0 as libc::c_int as libc::c_long {
                    return -(30 as libc::c_int) as int64_t;
                }
                cbytes -= avail
            }
        }
    }
    return consumed_bytes;
}
/*
 * Consume CFDATA as much as we have already gotten and
 * compute the sum of CFDATA.
 */
unsafe extern "C" fn cab_minimum_consume_cfdata(
    mut a: *mut archive_read,
    mut consumed_bytes: int64_t,
) -> int64_t {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut cfdata: *mut cfdata = 0 as *mut cfdata;
    let mut cbytes: int64_t = 0;
    let mut rbytes: int64_t = 0;
    let mut err: libc::c_int = 0;
    cfdata = (*cab).entry_cfdata;
    rbytes = consumed_bytes;
    if (*(*cab).entry_cffolder).comptype as libc::c_int == COMPTYPE_NONE {
        if consumed_bytes < (*cfdata).unconsumed {
            cbytes = consumed_bytes
        } else {
            cbytes = (*cfdata).unconsumed
        }
        rbytes -= cbytes;
        (*cfdata).read_offset =
            ((*cfdata).read_offset as libc::c_int + cbytes as uint16_t as libc::c_int) as uint16_t;
        (*cfdata).uncompressed_bytes_remaining =
            ((*cfdata).uncompressed_bytes_remaining as libc::c_int
                - cbytes as uint16_t as libc::c_int) as uint16_t;
        (*cfdata).unconsumed -= cbytes
    } else {
        cbytes = ((*cfdata).uncompressed_avail as libc::c_int
            - (*cfdata).read_offset as libc::c_int) as int64_t;
        if cbytes > 0 as libc::c_int as libc::c_long {
            if consumed_bytes < cbytes {
                cbytes = consumed_bytes
            }
            rbytes -= cbytes;
            (*cfdata).read_offset = ((*cfdata).read_offset as libc::c_int
                + cbytes as uint16_t as libc::c_int)
                as uint16_t;
            (*cfdata).uncompressed_bytes_remaining =
                ((*cfdata).uncompressed_bytes_remaining as libc::c_int
                    - cbytes as uint16_t as libc::c_int) as uint16_t
        }
        if (*cfdata).unconsumed != 0 {
            cbytes = (*cfdata).unconsumed;
            (*cfdata).unconsumed = 0 as libc::c_int as int64_t
        } else {
            cbytes = 0 as libc::c_int as int64_t
        }
    }
    if cbytes != 0 {
        /* Compute the sum. */
        cab_checksum_update(a, cbytes as size_t);
        /* Consume as much as the compressor actually used. */
        __archive_read_consume(a, cbytes);
        (*cab).cab_offset += cbytes;
        (*cfdata).compressed_bytes_remaining = ((*cfdata).compressed_bytes_remaining as libc::c_int
            - cbytes as uint16_t as libc::c_int)
            as uint16_t;
        if (*cfdata).compressed_bytes_remaining as libc::c_int == 0 as libc::c_int {
            err = cab_checksum_finish(a);
            if err < 0 as libc::c_int {
                return err as int64_t;
            }
        }
    }
    return rbytes;
}
/*
 * Returns ARCHIVE_OK if successful, ARCHIVE_FATAL otherwise, sets
 * cab->end_of_entry if it consumes all of the data.
 */
unsafe extern "C" fn cab_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut bytes_avail: ssize_t = 0;
    if (*cab).entry_bytes_remaining == 0 as libc::c_int as libc::c_long {
        *buff = NULL as *const libc::c_void;
        *size = 0 as libc::c_int as size_t;
        *offset = (*cab).entry_offset;
        (*cab).end_of_entry = 1 as libc::c_int as libc::c_char;
        return 0 as libc::c_int;
    }
    *buff = cab_read_ahead_cfdata(a, &mut bytes_avail);
    if bytes_avail <= 0 as libc::c_int as libc::c_long {
        *buff = NULL as *const libc::c_void;
        *size = 0 as libc::c_int as size_t;
        *offset = 0 as libc::c_int as int64_t;
        if bytes_avail == 0 as libc::c_int as libc::c_long
            && (*(*cab).entry_cfdata).uncompressed_size as libc::c_int == 0 as libc::c_int
        {
            /* All of CFDATA in a folder has been handled. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Invalid CFDATA\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        } else {
            return bytes_avail as libc::c_int;
        }
    }
    if bytes_avail > (*cab).entry_bytes_remaining {
        bytes_avail = (*cab).entry_bytes_remaining
    }
    *size = bytes_avail as size_t;
    *offset = (*cab).entry_offset;
    (*cab).entry_offset += bytes_avail;
    (*cab).entry_bytes_remaining -= bytes_avail;
    if (*cab).entry_bytes_remaining == 0 as libc::c_int as libc::c_long {
        (*cab).end_of_entry = 1 as libc::c_int as libc::c_char
    }
    (*cab).entry_unconsumed = bytes_avail;
    if (*(*cab).entry_cffolder).comptype as libc::c_int == COMPTYPE_NONE {
        /* Don't consume more than current entry used. */
        if (*(*cab).entry_cfdata).unconsumed > (*cab).entry_unconsumed {
            (*(*cab).entry_cfdata).unconsumed = (*cab).entry_unconsumed
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_cab_read_data_skip(
    mut a: *mut archive_read,
) -> libc::c_int {
    let mut cab: *mut cab = 0 as *mut cab;
    let mut bytes_skipped: int64_t = 0;
    let mut r: libc::c_int = 0;
    cab = (*(*a).format).data as *mut cab;
    if (*cab).end_of_archive != 0 {
        return 1 as libc::c_int;
    }
    if (*cab).read_data_invoked == 0 {
        (*cab).bytes_skipped += (*cab).entry_bytes_remaining;
        (*cab).entry_bytes_remaining = 0 as libc::c_int as int64_t;
        /* This entry is finished and done. */
        (*cab).end_of_entry = 1 as libc::c_int as libc::c_char;
        (*cab).end_of_entry_cleanup = (*cab).end_of_entry;
        return 0 as libc::c_int;
    }
    if (*cab).entry_unconsumed != 0 {
        /* Consume as much as the compressor actually used. */
        r = cab_consume_cfdata(a, (*cab).entry_unconsumed) as libc::c_int;
        (*cab).entry_unconsumed = 0 as libc::c_int as int64_t;
        if r < 0 as libc::c_int {
            return r;
        }
    } else if (*cab).entry_cfdata.is_null() {
        r = cab_next_cfdata(a);
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /* if we've already read to end of data, we're done. */
    if (*cab).end_of_entry_cleanup != 0 {
        return 0 as libc::c_int;
    }
    /*
     * If the length is at the beginning, we can skip the
     * compressed data much more quickly.
     */
    bytes_skipped = cab_consume_cfdata(a, (*cab).entry_bytes_remaining);
    if bytes_skipped < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    /* If the compression type is none(uncompressed), we've already
     * consumed data as much as the current entry size. */
    if (*(*cab).entry_cffolder).comptype as libc::c_int == COMPTYPE_NONE
        && !(*cab).entry_cfdata.is_null()
    {
        (*(*cab).entry_cfdata).unconsumed = 0 as libc::c_int as int64_t
    }
    /* This entry is finished and done. */
    (*cab).end_of_entry = 1 as libc::c_int as libc::c_char;
    (*cab).end_of_entry_cleanup = (*cab).end_of_entry;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_cab_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut cab: *mut cab = (*(*a).format).data as *mut cab;
    let mut hd: *mut cfheader = &mut (*cab).cfheader;
    let mut i: libc::c_int = 0;
    if !(*hd).folder_array.is_null() {
        i = 0 as libc::c_int;
        while i < (*hd).folder_count as libc::c_int {
            free((*(*hd).folder_array.offset(i as isize)).cfdata.memimage as *mut libc::c_void);
            i += 1
        }
        free((*hd).folder_array as *mut libc::c_void);
    }
    if !(*hd).file_array.is_null() {
        i = 0 as libc::c_int;
        while i < (*cab).cfheader.file_count as libc::c_int {
            archive_string_free(&mut (*(*hd).file_array.offset(i as isize)).pathname);
            i += 1
        }
        free((*hd).file_array as *mut libc::c_void);
    }
    if (*cab).stream_valid != 0 {
        inflateEnd(&mut (*cab).stream);
    }
    lzx_decode_free(&mut (*cab).xstrm);
    archive_wstring_free(&mut (*cab).ws);
    free((*cab).uncompressed_buffer as *mut libc::c_void);
    free(cab as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/* Convert an MSDOS-style date/time into Unix-style time. */
unsafe extern "C" fn cab_dos_time(mut p: *const libc::c_uchar) -> time_t {
    let mut msTime: libc::c_int = 0; /* Years since 1900. */
    let mut msDate: libc::c_int = 0; /* Month number.     */
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
    }; /* Day of month.     */
    msDate = archive_le16dec(p as *const libc::c_void) as libc::c_int;
    msTime =
        archive_le16dec(p.offset(2 as libc::c_int as isize) as *const libc::c_void) as libc::c_int;
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
/* ****************************************************************
 *
 * LZX decompression code.
 *
 *****************************************************************/
/*
 * Initialize LZX decoder.
 *
 * Returns ARCHIVE_OK if initialization was successful.
 * Returns ARCHIVE_FAILED if w_bits has unsupported value.
 * Returns ARCHIVE_FATAL if initialization failed; memory allocation
 * error occurred.
 */
unsafe extern "C" fn lzx_decode_init(
    mut strm: *mut lzx_stream,
    mut w_bits: libc::c_int,
) -> libc::c_int {
    let mut ds: *mut lzx_dec = 0 as *mut lzx_dec;
    let mut slot: libc::c_int = 0;
    let mut w_size: libc::c_int = 0;
    let mut w_slot: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut footer: libc::c_int = 0;
    let mut base_inc: [libc::c_int; 18] = [0; 18];
    if (*strm).ds.is_null() {
        (*strm).ds = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<lzx_dec>() as libc::c_ulong,
        ) as *mut lzx_dec;
        if (*strm).ds.is_null() {
            return -(30 as libc::c_int);
        }
    }
    ds = (*strm).ds;
    (*ds).error = ARCHIVE_FAILED;
    /* Allow bits from 15(32KBi) up to 21(2MBi) */
    if w_bits < SLOT_BASE || w_bits > SLOT_MAX {
        return -(25 as libc::c_int);
    }
    (*ds).error = ARCHIVE_FATAL;
    /*
     * Alloc window
     */
    w_size = (*ds).w_size;
    w_slot = slots[(w_bits - SLOT_BASE) as usize];
    (*ds).w_size = ((1 as libc::c_uint) << w_bits) as libc::c_int;
    (*ds).w_mask = (*ds).w_size - 1 as libc::c_int;
    if (*ds).w_buff.is_null() || w_size != (*ds).w_size {
        free((*ds).w_buff as *mut libc::c_void);
        (*ds).w_buff = malloc((*ds).w_size as libc::c_ulong) as *mut libc::c_uchar;
        if (*ds).w_buff.is_null() {
            return -(30 as libc::c_int);
        }
        free((*ds).pos_tbl as *mut libc::c_void);
        (*ds).pos_tbl = malloc(
            (::std::mem::size_of::<lzx_pos_tbl>() as libc::c_ulong)
                .wrapping_mul(w_slot as libc::c_ulong),
        ) as *mut lzx_pos_tbl;
        if (*ds).pos_tbl.is_null() {
            return -(30 as libc::c_int);
        }
        lzx_huffman_free(&mut (*ds).mt);
    }
    footer = 0 as libc::c_int;
    while footer < 18 as libc::c_int {
        base_inc[footer as usize] = (1 as libc::c_int) << footer;
        footer += 1
    }
    footer = 0 as libc::c_int;
    base = footer;
    slot = 0 as libc::c_int;
    while slot < w_slot {
        let mut n: libc::c_int = 0;
        if footer == 0 as libc::c_int {
            base = slot
        } else {
            base += base_inc[footer as usize]
        }
        if footer < 17 as libc::c_int {
            footer = -(2 as libc::c_int);
            n = base;
            while n != 0 {
                footer += 1;
                n >>= 1 as libc::c_int
            }
            if footer <= 0 as libc::c_int {
                footer = 0 as libc::c_int
            }
        }
        (*(*ds).pos_tbl.offset(slot as isize)).base = base;
        (*(*ds).pos_tbl.offset(slot as isize)).footer_bits = footer;
        slot += 1
    }
    (*ds).w_pos = 0 as libc::c_int;
    (*ds).state = 0 as libc::c_int;
    (*ds).br.cache_buffer = 0 as libc::c_int as uint64_t;
    (*ds).br.cache_avail = 0 as libc::c_int;
    (*ds).r2 = 1 as libc::c_int;
    (*ds).r1 = (*ds).r2;
    (*ds).r0 = (*ds).r1;
    /* Initialize aligned offset tree. */
    if lzx_huffman_init(&mut (*ds).at, 8 as libc::c_int as size_t, 8 as libc::c_int) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /* Initialize pre-tree. */
    if lzx_huffman_init(
        &mut (*ds).pt,
        20 as libc::c_int as size_t,
        10 as libc::c_int,
    ) != ARCHIVE_OK
    {
        return -(30 as libc::c_int);
    }
    /* Initialize Main tree. */
    if lzx_huffman_init(
        &mut (*ds).mt,
        (256 as libc::c_int + (w_slot << 3 as libc::c_int)) as size_t,
        16 as libc::c_int,
    ) != ARCHIVE_OK
    {
        return -(30 as libc::c_int);
    }
    /* Initialize Length tree. */
    if lzx_huffman_init(
        &mut (*ds).lt,
        249 as libc::c_int as size_t,
        16 as libc::c_int,
    ) != ARCHIVE_OK
    {
        return -(30 as libc::c_int);
    }
    (*ds).error = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/*
 * Release LZX decoder.
 */
unsafe extern "C" fn lzx_decode_free(mut strm: *mut lzx_stream) {
    if (*strm).ds.is_null() {
        return;
    }
    free((*(*strm).ds).w_buff as *mut libc::c_void);
    free((*(*strm).ds).pos_tbl as *mut libc::c_void);
    lzx_huffman_free(&mut (*(*strm).ds).at);
    lzx_huffman_free(&mut (*(*strm).ds).pt);
    lzx_huffman_free(&mut (*(*strm).ds).mt);
    lzx_huffman_free(&mut (*(*strm).ds).lt);
    free((*strm).ds as *mut libc::c_void);
    (*strm).ds = NULL as *mut lzx_dec;
}
/*
 * E8 Call Translation reversal.
 */
unsafe extern "C" fn lzx_translation(
    mut strm: *mut lzx_stream,
    mut p: *mut libc::c_void,
    mut size: size_t,
    mut offset: uint32_t,
) {
    let mut ds: *mut lzx_dec = (*strm).ds;
    let mut b: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*ds).translation == 0 || size <= 10 as libc::c_int as libc::c_ulong {
        return;
    }
    b = p as *mut libc::c_uchar;
    end = b
        .offset(size as isize)
        .offset(-(10 as libc::c_int as isize));
    while b < end && {
        b = memchr(
            b as *const libc::c_void,
            0xe8 as libc::c_int,
            end.offset_from(b) as libc::c_long as libc::c_ulong,
        ) as *mut libc::c_uchar;
        !b.is_null()
    } {
        let mut i: size_t =
            b.offset_from(p as *mut libc::c_uchar) as libc::c_long as size_t;
        let mut cp: int32_t = 0;
        let mut displacement: int32_t = 0;
        let mut value: int32_t = 0;
        cp = offset.wrapping_add(i as uint32_t) as int32_t;
        value = archive_le32dec(
            &mut *b.offset(1 as libc::c_int as isize) as *mut libc::c_uchar as *const libc::c_void,
        ) as int32_t;
        if value >= -cp && value < (*ds).translation_size as int32_t {
            if value >= 0 as libc::c_int {
                displacement = value - cp
            } else {
                displacement =
                    (value as libc::c_uint).wrapping_add((*ds).translation_size) as int32_t
            }
            archive_le32enc(
                &mut *b.offset(1 as libc::c_int as isize) as *mut libc::c_uchar
                    as *mut libc::c_void,
                displacement as uint32_t,
            );
        }
        b = b.offset(5 as libc::c_int as isize)
    }
}
static mut cache_masks: [uint32_t; 36] = [
    0 as libc::c_int as uint32_t,
    0x1 as libc::c_int as uint32_t,
    0x3 as libc::c_int as uint32_t,
    0x7 as libc::c_int as uint32_t,
    0xf as libc::c_int as uint32_t,
    0x1f as libc::c_int as uint32_t,
    0x3f as libc::c_int as uint32_t,
    0x7f as libc::c_int as uint32_t,
    0xff as libc::c_int as uint32_t,
    0x1ff as libc::c_int as uint32_t,
    0x3ff as libc::c_int as uint32_t,
    0x7ff as libc::c_int as uint32_t,
    0xfff as libc::c_int as uint32_t,
    0x1fff as libc::c_int as uint32_t,
    0x3fff as libc::c_int as uint32_t,
    0x7fff as libc::c_int as uint32_t,
    0xffff as libc::c_int as uint32_t,
    0x1ffff as libc::c_int as uint32_t,
    0x3ffff as libc::c_int as uint32_t,
    0x7ffff as libc::c_int as uint32_t,
    0xfffff as libc::c_int as uint32_t,
    0x1fffff as libc::c_int as uint32_t,
    0x3fffff as libc::c_int as uint32_t,
    0x7fffff as libc::c_int as uint32_t,
    0xffffff as libc::c_int as uint32_t,
    0x1ffffff as libc::c_int as uint32_t,
    0x3ffffff as libc::c_int as uint32_t,
    0x7ffffff as libc::c_int as uint32_t,
    0xfffffff as libc::c_int as uint32_t,
    0x1fffffff as libc::c_int as uint32_t,
    0x3fffffff as libc::c_int as uint32_t,
    0x7fffffff as libc::c_int as uint32_t,
    0xffffffff as libc::c_uint,
    0xffffffff as libc::c_uint,
    0xffffffff as libc::c_uint,
    0xffffffff as libc::c_uint,
];
/*
 * Shift away used bits in the cache data and fill it up with following bits.
 * Call this when cache buffer does not have enough bits you need.
 *
 * Returns 1 if the cache buffer is full.
 * Returns 0 if the cache buffer is not full; input buffer is empty.
 */
unsafe extern "C" fn lzx_br_fillup(mut strm: *mut lzx_stream, mut br: *mut lzx_br) -> libc::c_int {
    /*
     * x86 processor family can read misaligned data without an access error.
     */
    let mut n: libc::c_int = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong)
        .wrapping_sub((*br).cache_avail as libc::c_ulong)
        as libc::c_int;
    loop {
        match n >> 4 as libc::c_int {
            4 => {
                if (*strm).avail_in >= 8 as libc::c_int as libc::c_long {
                    (*br).cache_buffer = (*(*strm).next_in.offset(1 as libc::c_int as isize)
                        as uint64_t)
                        << 56 as libc::c_int
                        | (*(*strm).next_in.offset(0 as libc::c_int as isize) as uint64_t)
                            << 48 as libc::c_int
                        | (*(*strm).next_in.offset(3 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*strm).next_in.offset(2 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*strm).next_in.offset(5 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(4 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(7 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*strm).next_in.offset(6 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*strm).next_in = (*strm).next_in.offset(8 as libc::c_int as isize);
                    (*strm).avail_in -= 8 as libc::c_int as libc::c_long;
                    (*br).cache_avail += 8 as libc::c_int * 8 as libc::c_int;
                    return 1 as libc::c_int;
                }
            }
            3 => {
                if (*strm).avail_in >= 6 as libc::c_int as libc::c_long {
                    (*br).cache_buffer = (*br).cache_buffer << 48 as libc::c_int
                        | (*(*strm).next_in.offset(1 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*strm).next_in.offset(0 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*strm).next_in.offset(3 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(2 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(5 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*strm).next_in.offset(4 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*strm).next_in = (*strm).next_in.offset(6 as libc::c_int as isize);
                    (*strm).avail_in -= 6 as libc::c_int as libc::c_long;
                    (*br).cache_avail += 6 as libc::c_int * 8 as libc::c_int;
                    return 1 as libc::c_int;
                }
            }
            0 => {
                /* We have enough compressed data in
                 * the cache buffer.*/
                return 1 as libc::c_int;
            }
            _ => {}
        }
        if (*strm).avail_in < 2 as libc::c_int as libc::c_long {
            /* There is not enough compressed data to
             * fill up the cache buffer. */
            if (*strm).avail_in == 1 as libc::c_int as libc::c_long {
                let fresh5 = (*strm).next_in;
                (*strm).next_in = (*strm).next_in.offset(1);
                (*br).odd = *fresh5;
                (*strm).avail_in -= 1;
                (*br).have_odd = 1 as libc::c_int as libc::c_char
            }
            return 0 as libc::c_int;
        }
        (*br).cache_buffer = (*br).cache_buffer << 16 as libc::c_int
            | archive_le16dec((*strm).next_in as *const libc::c_void) as libc::c_ulong;
        (*strm).next_in = (*strm).next_in.offset(2 as libc::c_int as isize);
        (*strm).avail_in -= 2 as libc::c_int as libc::c_long;
        (*br).cache_avail += 16 as libc::c_int;
        n -= 16 as libc::c_int
    }
}
unsafe extern "C" fn lzx_br_fixup(mut strm: *mut lzx_stream, mut br: *mut lzx_br) {
    let mut n: libc::c_int = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong)
        .wrapping_sub((*br).cache_avail as libc::c_ulong)
        as libc::c_int;
    if (*br).have_odd as libc::c_int != 0
        && n >= 16 as libc::c_int
        && (*strm).avail_in > 0 as libc::c_int as libc::c_long
    {
        (*br).cache_buffer = (*br).cache_buffer << 16 as libc::c_int
            | ((*(*strm).next_in as uint16_t as libc::c_int) << 8 as libc::c_int) as libc::c_ulong
            | (*br).odd as libc::c_ulong;
        (*strm).next_in = (*strm).next_in.offset(1);
        (*strm).avail_in -= 1;
        (*br).cache_avail += 16 as libc::c_int;
        (*br).have_odd = 0 as libc::c_int as libc::c_char
    };
}
unsafe extern "C" fn lzx_cleanup_bitstream(mut strm: *mut lzx_stream) {
    (*(*strm).ds).br.cache_avail = 0 as libc::c_int;
    (*(*strm).ds).br.have_odd = 0 as libc::c_int as libc::c_char;
}
/*
 * Decode LZX.
 *
 * 1. Returns ARCHIVE_OK if output buffer or input buffer are empty.
 *    Please set available buffer and call this function again.
 * 2. Returns ARCHIVE_EOF if decompression has been completed.
 * 3. Returns ARCHIVE_FAILED if an error occurred; compressed data
 *    is broken or you do not set 'last' flag properly.
 */
pub const ST_RD_TRANSLATION: libc::c_int = 0 as libc::c_int;
pub const ST_RD_TRANSLATION_SIZE: libc::c_int = 1 as libc::c_int;
pub const ST_RD_BLOCK_TYPE: libc::c_int = 2 as libc::c_int;
pub const ST_RD_BLOCK_SIZE: libc::c_int = 3 as libc::c_int;
pub const ST_RD_ALIGNMENT: libc::c_int = 4 as libc::c_int;
pub const ST_RD_R0: libc::c_int = 5 as libc::c_int;
pub const ST_RD_R1: libc::c_int = 6 as libc::c_int;
pub const ST_RD_R2: libc::c_int = 7 as libc::c_int;
pub const ST_COPY_UNCOMP1: libc::c_int = 8 as libc::c_int;
pub const ST_COPY_UNCOMP2: libc::c_int = 9 as libc::c_int;
pub const ST_RD_ALIGNED_OFFSET: libc::c_int = 10 as libc::c_int;
pub const ST_RD_VERBATIM: libc::c_int = 11 as libc::c_int;
pub const ST_RD_PRE_MAIN_TREE_256: libc::c_int = 12 as libc::c_int;
pub const ST_MAIN_TREE_256: libc::c_int = 13 as libc::c_int;
pub const ST_RD_PRE_MAIN_TREE_REM: libc::c_int = 14 as libc::c_int;
pub const ST_MAIN_TREE_REM: libc::c_int = 15 as libc::c_int;
pub const ST_RD_PRE_LENGTH_TREE: libc::c_int = 16 as libc::c_int;
pub const ST_LENGTH_TREE: libc::c_int = 17 as libc::c_int;
pub const ST_MAIN: libc::c_int = 18 as libc::c_int;
pub const ST_LENGTH: libc::c_int = 19 as libc::c_int;
pub const ST_OFFSET: libc::c_int = 20 as libc::c_int;
pub const ST_REAL_POS: libc::c_int = 21 as libc::c_int;
pub const ST_COPY: libc::c_int = 22 as libc::c_int;
unsafe extern "C" fn lzx_decode(mut strm: *mut lzx_stream, mut last: libc::c_int) -> libc::c_int {
    let mut ds: *mut lzx_dec = (*strm).ds;
    let mut avail_in: int64_t = 0;
    let mut r: libc::c_int = 0;
    if (*ds).error != 0 {
        return (*ds).error;
    }
    avail_in = (*strm).avail_in;
    lzx_br_fixup(strm, &mut (*ds).br);
    loop {
        if (*ds).state < ST_MAIN {
            r = lzx_read_blocks(strm, last)
        } else {
            let mut bytes_written: int64_t = (*strm).avail_out;
            r = lzx_decode_blocks(strm, last);
            bytes_written -= (*strm).avail_out;
            (*strm).next_out = (*strm).next_out.offset(bytes_written as isize);
            (*strm).total_out += bytes_written
        }
        if !(r == 100 as libc::c_int) {
            break;
        }
    }
    (*strm).total_in += avail_in - (*strm).avail_in;
    return r;
}
unsafe extern "C" fn lzx_read_blocks(
    mut strm: *mut lzx_stream,
    mut last: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ds: *mut lzx_dec = (*strm).ds;
    let mut br: *mut lzx_br = &mut (*ds).br;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    's_16: loop {
        match (*ds).state {
            ST_RD_TRANSLATION => {
                if !((*br).cache_avail >= 1 as libc::c_int
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= 1 as libc::c_int)
                {
                    (*ds).state = ST_RD_TRANSLATION;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    (*ds).translation =
                        (((*br).cache_buffer >> (*br).cache_avail - 1 as libc::c_int) as uint32_t
                            & cache_masks[1 as libc::c_int as usize])
                            as libc::c_char;
                    (*br).cache_avail -= 1 as libc::c_int
                }
                current_block = 12734998125717865982;
            }
            ST_RD_TRANSLATION_SIZE => {
                current_block = 12734998125717865982;
            }
            ST_RD_BLOCK_TYPE => {
                current_block = 15042310719884093888;
            }
            ST_RD_BLOCK_SIZE => {
                current_block = 860601949763470011;
            }
            ST_RD_ALIGNMENT => {
                current_block = 4801541944421719473;
            }
            ST_RD_R0 | ST_RD_R1 | ST_RD_R2 => {
                current_block = 1724319918354933278;
            }
            ST_COPY_UNCOMP1 => {
                current_block = 5023038348526654800;
            }
            ST_COPY_UNCOMP2 => {
                current_block = 8450291639822095200;
            }
            ST_RD_ALIGNED_OFFSET => {
                /*
                 * Read Aligned offset tree.
                 */
                if !((*br).cache_avail >= 3 as libc::c_int * (*ds).at.len_size
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= 3 as libc::c_int * (*ds).at.len_size)
                {
                    (*ds).state = ST_RD_ALIGNED_OFFSET;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    memset(
                        (*ds).at.freq.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[libc::c_int; 17]>() as libc::c_ulong,
                    );
                    i = 0 as libc::c_int;
                    while i < (*ds).at.len_size {
                        *(*ds).at.bitlen.offset(i as isize) = (((*br).cache_buffer
                            >> (*br).cache_avail - 3 as libc::c_int)
                            as uint32_t
                            & cache_masks[3 as libc::c_int as usize])
                            as libc::c_uchar;
                        (*ds).at.freq[*(*ds).at.bitlen.offset(i as isize) as usize] += 1;
                        (*br).cache_avail -= 3 as libc::c_int;
                        i += 1
                    }
                    if lzx_make_huffman_table(&mut (*ds).at) == 0 {
                        break;
                    }
                }
                current_block = 17354132686035956570;
            }
            ST_RD_VERBATIM => {
                current_block = 17354132686035956570;
            }
            ST_RD_PRE_MAIN_TREE_256 => {
                current_block = 7577173092109912252;
            }
            ST_MAIN_TREE_256 => {
                current_block = 10131319541265114523;
            }
            ST_RD_PRE_MAIN_TREE_REM => {
                current_block = 3554158773753863382;
            }
            ST_MAIN_TREE_REM => {
                current_block = 6435707263684746297;
            }
            ST_RD_PRE_LENGTH_TREE => {
                current_block = 18073281983946017701;
            }
            ST_LENGTH_TREE => {
                current_block = 15301158066411706028;
            }
            _ => {
                continue;
            }
        }
        match current_block {
            12734998125717865982 =>
            /* FALL THROUGH */
            {
                if (*ds).translation != 0 {
                    if !((*br).cache_avail >= 32 as libc::c_int
                        || lzx_br_fillup(strm, br) != 0
                        || (*br).cache_avail >= 32 as libc::c_int)
                    {
                        (*ds).state = ST_RD_TRANSLATION_SIZE;
                        if last != 0 {
                            break;
                        }
                        return 0 as libc::c_int;
                    } else {
                        (*ds).translation_size = ((*br).cache_buffer
                            >> (*br).cache_avail - 16 as libc::c_int)
                            as uint32_t
                            & cache_masks[16 as libc::c_int as usize];
                        (*br).cache_avail -= 16 as libc::c_int;
                        (*ds).translation_size <<= 16 as libc::c_int;
                        (*ds).translation_size |= ((*br).cache_buffer
                            >> (*br).cache_avail - 16 as libc::c_int)
                            as uint32_t
                            & cache_masks[16 as libc::c_int as usize];
                        (*br).cache_avail -= 16 as libc::c_int
                    }
                    current_block = 15042310719884093888;
                } else {
                    current_block = 15042310719884093888;
                }
            }
            17354132686035956570 =>
            /* FALL THROUGH */
            {
                (*ds).loop_0 = 0 as libc::c_int;
                /* FALL THROUGH */
                current_block = 7577173092109912252;
            }
            _ => {}
        }
        match current_block {
            7577173092109912252 =>
            /*
             * Read Pre-tree for first 256 elements of main tree.
             */
            {
                if lzx_read_pre_tree(strm) == 0 {
                    (*ds).state = ST_RD_PRE_MAIN_TREE_256;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    if lzx_make_huffman_table(&mut (*ds).pt) == 0 {
                        break;
                    }
                    (*ds).loop_0 = 0 as libc::c_int
                }
                current_block = 10131319541265114523;
            }
            15042310719884093888 =>
            /* FALL THROUGH */
            {
                if !((*br).cache_avail >= 3 as libc::c_int
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= 3 as libc::c_int)
                {
                    (*ds).state = ST_RD_BLOCK_TYPE;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    (*ds).block_type = (((*br).cache_buffer >> (*br).cache_avail - 3 as libc::c_int)
                        as uint32_t
                        & cache_masks[3 as libc::c_int as usize])
                        as libc::c_char;
                    (*br).cache_avail -= 3 as libc::c_int;
                    /* Check a block type. */
                    match (*ds).block_type as libc::c_int {
                        VERBATIM_BLOCK | ALIGNED_OFFSET_BLOCK | UNCOMPRESSED_BLOCK => {}
                        _ => {
                            break;
                        }
                    }
                }
                current_block = 860601949763470011;
            }
            _ => {}
        }
        match current_block {
            10131319541265114523 =>
            /* FALL THROUGH */
            /*
             * Get path lengths of first 256 elements of main tree.
             */
            {
                r = lzx_read_bitlen(strm, &mut (*ds).mt, 256 as libc::c_int);
                if r < 0 as libc::c_int {
                    break;
                }
                if r == 0 {
                    (*ds).state = ST_MAIN_TREE_256;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    (*ds).loop_0 = 0 as libc::c_int
                }
                current_block = 3554158773753863382;
            }
            860601949763470011 =>
            /* FALL THROUGH */
            {
                if !((*br).cache_avail >= 24 as libc::c_int
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= 24 as libc::c_int)
                {
                    (*ds).state = ST_RD_BLOCK_SIZE;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    (*ds).block_size = (((*br).cache_buffer >> (*br).cache_avail - 8 as libc::c_int)
                        as uint32_t
                        & cache_masks[8 as libc::c_int as usize])
                        as size_t;
                    (*br).cache_avail -= 8 as libc::c_int;
                    (*ds).block_size <<= 16 as libc::c_int;
                    (*ds).block_size |=
                        (((*br).cache_buffer >> (*br).cache_avail - 16 as libc::c_int) as uint32_t
                            & cache_masks[16 as libc::c_int as usize])
                            as libc::c_ulong;
                    (*br).cache_avail -= 16 as libc::c_int;
                    if (*ds).block_size == 0 as libc::c_int as libc::c_ulong {
                        break;
                    }
                    (*ds).block_bytes_avail = (*ds).block_size;
                    if (*ds).block_type as libc::c_int != UNCOMPRESSED_BLOCK {
                        if (*ds).block_type as libc::c_int == VERBATIM_BLOCK {
                            (*ds).state = ST_RD_VERBATIM
                        } else {
                            (*ds).state = ST_RD_ALIGNED_OFFSET
                        }
                        continue;
                    }
                }
                current_block = 4801541944421719473;
            }
            _ => {}
        }
        match current_block {
            4801541944421719473 =>
            /*
             * Handle an Uncompressed Block.
             */
            /* Skip padding to align following field on
             * 16-bit boundary. */
            {
                if (*br).cache_avail & 0xf as libc::c_int != 0 {
                    (*br).cache_avail &= !(0xf as libc::c_int)
                } else if (*br).cache_avail >= 16 as libc::c_int
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= 16 as libc::c_int
                {
                    (*br).cache_avail -= 16 as libc::c_int
                } else {
                    (*ds).state = ST_RD_ALIGNMENT;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                }
                /* Preparation to read repeated offsets R0,R1 and R2. */
                (*ds).rbytes_avail = 0 as libc::c_int;
                (*ds).state = ST_RD_R0;
                current_block = 1724319918354933278;
            }
            3554158773753863382 =>
            /*
             * Read Pre-tree for remaining elements of main tree.
             */
            {
                if lzx_read_pre_tree(strm) == 0 {
                    (*ds).state = ST_RD_PRE_MAIN_TREE_REM;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    if lzx_make_huffman_table(&mut (*ds).pt) == 0 {
                        break;
                    }
                    (*ds).loop_0 = 256 as libc::c_int
                }
                current_block = 6435707263684746297;
            }
            _ => {}
        }
        match current_block {
            1724319918354933278 => {
                loop
                /* FALL THROUGH */
                {
                    let mut u16: uint16_t = 0;
                    /* Drain bits in the cache buffer of
                     * bit-stream. */
                    if (*br).cache_avail >= 32 as libc::c_int {
                        u16 = (((*br).cache_buffer >> (*br).cache_avail - 16 as libc::c_int)
                            as uint32_t
                            & cache_masks[16 as libc::c_int as usize])
                            as uint16_t;
                        (*br).cache_avail -= 16 as libc::c_int;
                        archive_le16enc((*ds).rbytes.as_mut_ptr() as *mut libc::c_void, u16);
                        u16 = (((*br).cache_buffer >> (*br).cache_avail - 16 as libc::c_int)
                            as uint32_t
                            & cache_masks[16 as libc::c_int as usize])
                            as uint16_t;
                        (*br).cache_avail -= 16 as libc::c_int;
                        archive_le16enc(
                            (*ds).rbytes.as_mut_ptr().offset(2 as libc::c_int as isize)
                                as *mut libc::c_void,
                            u16,
                        );
                        (*ds).rbytes_avail = 4 as libc::c_int
                    } else if (*br).cache_avail >= 16 as libc::c_int {
                        u16 = (((*br).cache_buffer >> (*br).cache_avail - 16 as libc::c_int)
                            as uint32_t
                            & cache_masks[16 as libc::c_int as usize])
                            as uint16_t;
                        (*br).cache_avail -= 16 as libc::c_int;
                        archive_le16enc((*ds).rbytes.as_mut_ptr() as *mut libc::c_void, u16);
                        (*ds).rbytes_avail = 2 as libc::c_int
                    }
                    if (*ds).rbytes_avail < 4 as libc::c_int
                        && (*ds).br.have_odd as libc::c_int != 0
                    {
                        let fresh6 = (*ds).rbytes_avail;
                        (*ds).rbytes_avail = (*ds).rbytes_avail + 1;
                        (*ds).rbytes[fresh6 as usize] = (*ds).br.odd;
                        (*ds).br.have_odd = 0 as libc::c_int as libc::c_char
                    }
                    while (*ds).rbytes_avail < 4 as libc::c_int {
                        if (*strm).avail_in <= 0 as libc::c_int as libc::c_long {
                            if last != 0 {
                                break 's_16;
                            }
                            return 0 as libc::c_int;
                        } else {
                            let fresh7 = (*strm).next_in;
                            (*strm).next_in = (*strm).next_in.offset(1);
                            let fresh8 = (*ds).rbytes_avail;
                            (*ds).rbytes_avail = (*ds).rbytes_avail + 1;
                            (*ds).rbytes[fresh8 as usize] = *fresh7;
                            (*strm).avail_in -= 1
                        }
                    }
                    (*ds).rbytes_avail = 0 as libc::c_int;
                    if (*ds).state == ST_RD_R0 {
                        (*ds).r0 = archive_le32dec((*ds).rbytes.as_mut_ptr() as *const libc::c_void)
                            as libc::c_int;
                        if (*ds).r0 < 0 as libc::c_int {
                            break 's_16;
                        }
                        (*ds).state = ST_RD_R1
                    } else if (*ds).state == ST_RD_R1 {
                        (*ds).r1 = archive_le32dec((*ds).rbytes.as_mut_ptr() as *const libc::c_void)
                            as libc::c_int;
                        if (*ds).r1 < 0 as libc::c_int {
                            break 's_16;
                        }
                        (*ds).state = ST_RD_R2
                    } else if (*ds).state == ST_RD_R2 {
                        (*ds).r2 = archive_le32dec((*ds).rbytes.as_mut_ptr() as *const libc::c_void)
                            as libc::c_int;
                        if (*ds).r2 < 0 as libc::c_int {
                            break 's_16;
                        }
                        /* We've gotten all repeated offsets. */
                        (*ds).state = ST_COPY_UNCOMP1
                    }
                    if !((*ds).state != ST_COPY_UNCOMP1) {
                        break;
                    }
                }
                /* FALL THROUGH */
                current_block = 5023038348526654800;
            }
            6435707263684746297 =>
            /* FALL THROUGH */
            /*
             * Get path lengths of remaining elements of main tree.
             */
            {
                r = lzx_read_bitlen(strm, &mut (*ds).mt, -(1 as libc::c_int));
                if r < 0 as libc::c_int {
                    break;
                }
                if r == 0 {
                    (*ds).state = ST_MAIN_TREE_REM;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    if lzx_make_huffman_table(&mut (*ds).mt) == 0 {
                        break;
                    }
                    (*ds).loop_0 = 0 as libc::c_int
                }
                current_block = 18073281983946017701;
            }
            _ => {}
        }
        match current_block {
            5023038348526654800 => {
                /*
                 * Copy bytes form next_in to next_out directly.
                 */
                while (*ds).block_bytes_avail != 0 {
                    let mut l: libc::c_int = 0;
                    if (*strm).avail_out <= 0 as libc::c_int as libc::c_long {
                        /* Output buffer is empty. */
                        return 0 as libc::c_int;
                    }
                    if (*strm).avail_in <= 0 as libc::c_int as libc::c_long {
                        /* Input buffer is empty. */
                        if last != 0 {
                            break 's_16;
                        }
                        return 0 as libc::c_int;
                    } else {
                        l = (*ds).block_bytes_avail as libc::c_int;
                        if l > (*ds).w_size - (*ds).w_pos {
                            l = (*ds).w_size - (*ds).w_pos
                        }
                        if l as libc::c_long > (*strm).avail_out {
                            l = (*strm).avail_out as libc::c_int
                        }
                        if l as libc::c_long > (*strm).avail_in {
                            l = (*strm).avail_in as libc::c_int
                        }
                        memcpy(
                            (*strm).next_out as *mut libc::c_void,
                            (*strm).next_in as *const libc::c_void,
                            l as libc::c_ulong,
                        );
                        memcpy(
                            &mut *(*ds).w_buff.offset((*ds).w_pos as isize) as *mut libc::c_uchar
                                as *mut libc::c_void,
                            (*strm).next_in as *const libc::c_void,
                            l as libc::c_ulong,
                        );
                        (*strm).next_in = (*strm).next_in.offset(l as isize);
                        (*strm).avail_in -= l as libc::c_long;
                        (*strm).next_out = (*strm).next_out.offset(l as isize);
                        (*strm).avail_out -= l as libc::c_long;
                        (*strm).total_out += l as libc::c_long;
                        (*ds).w_pos = (*ds).w_pos + l & (*ds).w_mask;
                        (*ds).block_bytes_avail = ((*ds).block_bytes_avail as libc::c_ulong)
                            .wrapping_sub(l as libc::c_ulong)
                            as size_t as size_t
                    }
                }
                /* FALL THROUGH */
                current_block = 8450291639822095200;
            }
            18073281983946017701 =>
            /*
             * Read Pre-tree for remaining elements of main tree.
             */
            {
                if lzx_read_pre_tree(strm) == 0 {
                    (*ds).state = ST_RD_PRE_LENGTH_TREE;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    if lzx_make_huffman_table(&mut (*ds).pt) == 0 {
                        break;
                    }
                    (*ds).loop_0 = 0 as libc::c_int
                }
                current_block = 15301158066411706028;
            }
            _ => {}
        }
        match current_block {
            8450291639822095200 =>
            /* Re-align; skip padding byte. */
            {
                if (*ds).block_size & 1 as libc::c_int as libc::c_ulong != 0 {
                    if (*strm).avail_in <= 0 as libc::c_int as libc::c_long {
                        /* Input buffer is empty. */
                        (*ds).state = ST_COPY_UNCOMP2;
                        if last != 0 {
                            break;
                        }
                        return 0 as libc::c_int;
                    } else {
                        (*strm).next_in = (*strm).next_in.offset(1);
                        (*strm).avail_in -= 1
                    }
                }
                /* This block ended. */
                (*ds).state = ST_RD_BLOCK_TYPE;
                return 1 as libc::c_int;
            }
            _ =>
            /* FALL THROUGH */
            /*
             * Get path lengths of remaining elements of main tree.
             */
            {
                r = lzx_read_bitlen(strm, &mut (*ds).lt, -(1 as libc::c_int));
                if r < 0 as libc::c_int {
                    break;
                }
                if r == 0 {
                    (*ds).state = ST_LENGTH_TREE;
                    if last != 0 {
                        break;
                    }
                    return 0 as libc::c_int;
                } else {
                    if lzx_make_huffman_table(&mut (*ds).lt) == 0 {
                        break;
                    }
                    (*ds).state = ST_MAIN;
                    return 100 as libc::c_int;
                }
            }
        }
    }
    (*ds).error = ARCHIVE_FAILED;
    return (*ds).error;
}
unsafe extern "C" fn lzx_decode_blocks(
    mut strm: *mut lzx_stream,
    mut last: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ds: *mut lzx_dec = (*strm).ds;
    let mut bre: lzx_br = (*ds).br;
    let mut at: *mut huffman = &mut (*ds).at;
    let mut lt: *mut huffman = &mut (*ds).lt;
    let mut mt: *mut huffman = &mut (*ds).mt;
    let mut pos_tbl: *const lzx_pos_tbl = (*ds).pos_tbl;
    let mut noutp: *mut libc::c_uchar = (*strm).next_out;
    let mut endp: *mut libc::c_uchar = noutp.offset((*strm).avail_out as isize);
    let mut w_buff: *mut libc::c_uchar = (*ds).w_buff;
    let mut at_bitlen: *mut libc::c_uchar = (*at).bitlen;
    let mut lt_bitlen: *mut libc::c_uchar = (*lt).bitlen;
    let mut mt_bitlen: *mut libc::c_uchar = (*mt).bitlen;
    let mut block_bytes_avail: size_t = (*ds).block_bytes_avail;
    let mut at_max_bits: libc::c_int = (*at).max_bits;
    let mut lt_max_bits: libc::c_int = (*lt).max_bits;
    let mut mt_max_bits: libc::c_int = (*mt).max_bits;
    let mut c: libc::c_int = 0;
    let mut copy_len: libc::c_int = (*ds).copy_len;
    let mut copy_pos: libc::c_int = (*ds).copy_pos;
    let mut w_pos: libc::c_int = (*ds).w_pos;
    let mut w_mask: libc::c_int = (*ds).w_mask;
    let mut w_size: libc::c_int = (*ds).w_size;
    let mut length_header: libc::c_int = (*ds).length_header;
    let mut offset_bits: libc::c_int = (*ds).offset_bits;
    let mut position_slot: libc::c_int = (*ds).position_slot;
    let mut r0: libc::c_int = (*ds).r0;
    let mut r1: libc::c_int = (*ds).r1;
    let mut r2: libc::c_int = (*ds).r2;
    let mut state: libc::c_int = (*ds).state;
    let mut block_type: libc::c_char = (*ds).block_type;
    's_73: loop {
        match state {
            ST_MAIN => {
                current_block = 7149356873433890176;
            }
            ST_LENGTH => {
                current_block = 2062342269357045135;
            }
            ST_OFFSET => {
                current_block = 3336416671349548293;
            }
            ST_REAL_POS => {
                current_block = 9324558484539972570;
            }
            ST_COPY => {
                current_block = 9521147444787763968;
            }
            _ => {
                continue;
            }
        }
        loop {
            match current_block {
                7149356873433890176 => {
                    if block_bytes_avail == 0 as libc::c_int as libc::c_ulong {
                        /* This block ended. */
                        (*ds).state = ST_RD_BLOCK_TYPE;
                        (*ds).br = bre;
                        (*ds).block_bytes_avail = block_bytes_avail;
                        (*ds).copy_len = copy_len;
                        (*ds).copy_pos = copy_pos;
                        (*ds).length_header = length_header;
                        (*ds).position_slot = position_slot;
                        (*ds).r0 = r0;
                        (*ds).r1 = r1;
                        (*ds).r2 = r2;
                        (*ds).w_pos = w_pos;
                        (*strm).avail_out = endp.offset_from(noutp) as libc::c_long;
                        return 1 as libc::c_int;
                    }
                    if noutp >= endp {
                        current_block = 14029810558126289694;
                        break 's_73;
                    }
                    if !(bre.cache_avail >= mt_max_bits
                        || lzx_br_fillup(strm, &mut bre) != 0
                        || bre.cache_avail >= mt_max_bits)
                    {
                        if last == 0 {
                            current_block = 14029810558126289694;
                            break 's_73;
                            /* Over read. */
                        }
                        /* Remaining bits are less than
                         * maximum bits(mt.max_bits) but maybe
                         * it still remains as much as we need,
                         * so we should try to use it with
                         * dummy bits. */
                        c = lzx_decode_huffman(
                            mt,
                            (bre.cache_buffer << mt_max_bits - bre.cache_avail) as uint32_t
                                & cache_masks[mt_max_bits as usize],
                        );
                        bre.cache_avail -= *mt_bitlen.offset(c as isize) as libc::c_int;
                        if !(bre.cache_avail >= 0 as libc::c_int) {
                            current_block = 7651564465025286514;
                            break 's_73;
                        }
                    } else {
                        c = lzx_decode_huffman(
                            mt,
                            (bre.cache_buffer >> bre.cache_avail - mt_max_bits) as uint32_t
                                & cache_masks[mt_max_bits as usize],
                        );
                        bre.cache_avail -= *mt_bitlen.offset(c as isize) as libc::c_int
                    }
                    if c > UCHAR_MAX {
                        /*
                         * Get a match code, its length and offset.
                         */
                        c -= UCHAR_MAX + 1 as libc::c_int;
                        length_header = c & 7 as libc::c_int;
                        position_slot = c >> 3 as libc::c_int;
                        /* FALL THROUGH */
                        current_block = 2062342269357045135;
                    } else {
                        /*
                         * 'c' is exactly literal code.
                         */
                        /* Save a decoded code to reference it
                         * afterward. */
                        *w_buff.offset(w_pos as isize) = c as libc::c_uchar;
                        w_pos = w_pos + 1 as libc::c_int & w_mask;
                        /* Store the decoded code to output buffer. */
                        let fresh9 = noutp;
                        noutp = noutp.offset(1);
                        *fresh9 = c as libc::c_uchar;
                        block_bytes_avail = block_bytes_avail.wrapping_sub(1);
                        current_block = 7149356873433890176;
                    }
                }
                2062342269357045135 =>
                /*
                 * Get a length.
                 */
                {
                    if length_header == 7 as libc::c_int {
                        if !(bre.cache_avail >= lt_max_bits
                            || lzx_br_fillup(strm, &mut bre) != 0
                            || bre.cache_avail >= lt_max_bits)
                        {
                            if last == 0 {
                                state = ST_LENGTH;
                                current_block = 14029810558126289694;
                                break 's_73;
                            } else {
                                c = lzx_decode_huffman(
                                    lt,
                                    (bre.cache_buffer << lt_max_bits - bre.cache_avail) as uint32_t
                                        & cache_masks[lt_max_bits as usize],
                                );
                                bre.cache_avail -= *lt_bitlen.offset(c as isize) as libc::c_int;
                                if !(bre.cache_avail >= 0 as libc::c_int) {
                                    current_block = 7651564465025286514;
                                    break 's_73;
                                }
                            }
                        /* Over read. */
                        } else {
                            c = lzx_decode_huffman(
                                lt,
                                (bre.cache_buffer >> bre.cache_avail - lt_max_bits) as uint32_t
                                    & cache_masks[lt_max_bits as usize],
                            );
                            bre.cache_avail -= *lt_bitlen.offset(c as isize) as libc::c_int
                        }
                        copy_len = c + 7 as libc::c_int + 2 as libc::c_int
                    } else {
                        copy_len = length_header + 2 as libc::c_int
                    }
                    if copy_len as size_t > block_bytes_avail {
                        current_block = 7651564465025286514;
                        break 's_73;
                    }
                    /*
                     * Get an offset.
                     */
                    match position_slot {
                        0 => {
                            /* Use repeated offset 0. */
                            copy_pos = r0;
                            state = ST_REAL_POS;
                            break;
                        }
                        1 => {
                            /* Use repeated offset 1. */
                            copy_pos = r1;
                            /* Swap repeated offset. */
                            r1 = r0;
                            r0 = copy_pos;
                            state = ST_REAL_POS;
                            break;
                        }
                        2 => {
                            /* Use repeated offset 2. */
                            copy_pos = r2;
                            /* Swap repeated offset. */
                            r2 = r0;
                            r0 = copy_pos;
                            state = ST_REAL_POS;
                            break;
                        }
                        _ => {
                            offset_bits = (*pos_tbl.offset(position_slot as isize)).footer_bits;
                            /* FALL THROUGH */
                            current_block = 3336416671349548293;
                        }
                    }
                }
                9521147444787763968 =>
                /*
                 * Copy several bytes as extracted data from the window
                 * into the output buffer.
                 */
                {
                    let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
                    let mut l: libc::c_int = 0;
                    l = copy_len;
                    if copy_pos > w_pos {
                        if l > w_size - copy_pos {
                            l = w_size - copy_pos
                        }
                    } else if l > w_size - w_pos {
                        l = w_size - w_pos
                    }
                    if noutp.offset(l as isize) >= endp {
                        l = endp.offset_from(noutp) as libc::c_long as libc::c_int
                    }
                    s = w_buff.offset(copy_pos as isize);
                    if l >= 8 as libc::c_int && (copy_pos + l < w_pos || w_pos + l < copy_pos) {
                        memcpy(
                            w_buff.offset(w_pos as isize) as *mut libc::c_void,
                            s as *const libc::c_void,
                            l as libc::c_ulong,
                        );
                        memcpy(
                            noutp as *mut libc::c_void,
                            s as *const libc::c_void,
                            l as libc::c_ulong,
                        );
                    } else {
                        let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                        let mut li: libc::c_int = 0;
                        d = w_buff.offset(w_pos as isize);
                        li = 0 as libc::c_int;
                        while li < l {
                            let ref mut fresh10 = *d.offset(li as isize);
                            *fresh10 = *s.offset(li as isize);
                            *noutp.offset(li as isize) = *fresh10;
                            li += 1
                        }
                    }
                    noutp = noutp.offset(l as isize);
                    copy_pos = copy_pos + l & w_mask;
                    w_pos = w_pos + l & w_mask;
                    block_bytes_avail = (block_bytes_avail as libc::c_ulong)
                        .wrapping_sub(l as libc::c_ulong)
                        as size_t as size_t;
                    if copy_len <= l {
                        /* A copy of current pattern ended. */
                        state = ST_MAIN;
                        break;
                    } else {
                        copy_len -= l;
                        if !(noutp >= endp) {
                            current_block = 9521147444787763968;
                            continue;
                        }
                        /* Output buffer is empty. */
                        state = ST_COPY;
                        current_block = 14029810558126289694;
                        break 's_73;
                    }
                }
                9324558484539972570 =>
                /* FALL THROUGH */
                /*
                 * Compute a real position in window.
                 */
                {
                    copy_pos = w_pos - copy_pos & w_mask;
                    /* FALL THROUGH */
                    current_block = 9521147444787763968;
                }
                _ =>
                /*
                 * Get the offset, which is a distance from
                 * current window position.
                 */
                {
                    if block_type as libc::c_int == ALIGNED_OFFSET_BLOCK
                        && offset_bits >= 3 as libc::c_int
                    {
                        let mut offbits: libc::c_int = offset_bits - 3 as libc::c_int;
                        if !(bre.cache_avail >= offbits
                            || lzx_br_fillup(strm, &mut bre) != 0
                            || bre.cache_avail >= offbits)
                        {
                            state = ST_OFFSET;
                            if last != 0 {
                                current_block = 7651564465025286514;
                                break 's_73;
                            } else {
                                current_block = 14029810558126289694;
                                break 's_73;
                            }
                        } else {
                            copy_pos = (((bre.cache_buffer >> bre.cache_avail - offbits)
                                as uint32_t
                                & cache_masks[offbits as usize])
                                << 3 as libc::c_int)
                                as libc::c_int;
                            /* Get an aligned number. */
                            if !(bre.cache_avail >= offbits + at_max_bits
                                || lzx_br_fillup(strm, &mut bre) != 0
                                || bre.cache_avail >= offbits + at_max_bits)
                            {
                                if last == 0 {
                                    state = ST_OFFSET;
                                    current_block = 14029810558126289694;
                                    break 's_73;
                                } else {
                                    bre.cache_avail -= offbits;
                                    c = lzx_decode_huffman(
                                        at,
                                        (bre.cache_buffer << at_max_bits - bre.cache_avail)
                                            as uint32_t
                                            & cache_masks[at_max_bits as usize],
                                    );
                                    bre.cache_avail -= *at_bitlen.offset(c as isize) as libc::c_int;
                                    if !(bre.cache_avail >= 0 as libc::c_int) {
                                        current_block = 7651564465025286514;
                                        break 's_73;
                                    }
                                }
                            /* Over read. */
                            } else {
                                bre.cache_avail -= offbits;
                                c = lzx_decode_huffman(
                                    at,
                                    (bre.cache_buffer >> bre.cache_avail - at_max_bits) as uint32_t
                                        & cache_masks[at_max_bits as usize],
                                );
                                bre.cache_avail -= *at_bitlen.offset(c as isize) as libc::c_int
                            }
                            /* Add an aligned number. */
                            copy_pos += c
                        }
                    } else if !(bre.cache_avail >= offset_bits
                        || lzx_br_fillup(strm, &mut bre) != 0
                        || bre.cache_avail >= offset_bits)
                    {
                        state = ST_OFFSET;
                        if last != 0 {
                            current_block = 7651564465025286514;
                            break 's_73;
                        } else {
                            current_block = 14029810558126289694;
                            break 's_73;
                        }
                    } else {
                        copy_pos = ((bre.cache_buffer >> bre.cache_avail - offset_bits) as uint32_t
                            & cache_masks[offset_bits as usize])
                            as libc::c_int;
                        bre.cache_avail -= offset_bits
                    }
                    copy_pos += (*pos_tbl.offset(position_slot as isize)).base - 2 as libc::c_int;
                    /* Update repeated offset LRU queue. */
                    r2 = r1;
                    r1 = r0;
                    r0 = copy_pos;
                    current_block = 9324558484539972570;
                }
            }
        }
    }
    match current_block {
        7651564465025286514 => {
            (*ds).error = ARCHIVE_FAILED;
            return (*ds).error;
        }
        _ =>
        /* Output buffer is empty. */
        {
            (*ds).br = bre;
            (*ds).block_bytes_avail = block_bytes_avail;
            (*ds).copy_len = copy_len;
            (*ds).copy_pos = copy_pos;
            (*ds).length_header = length_header;
            (*ds).offset_bits = offset_bits;
            (*ds).position_slot = position_slot;
            (*ds).r0 = r0;
            (*ds).r1 = r1;
            (*ds).r2 = r2;
            (*ds).state = state;
            (*ds).w_pos = w_pos;
            (*strm).avail_out = endp.offset_from(noutp) as libc::c_long;
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn lzx_read_pre_tree(mut strm: *mut lzx_stream) -> libc::c_int {
    let mut ds: *mut lzx_dec = (*strm).ds;
    let mut br: *mut lzx_br = &mut (*ds).br;
    let mut i: libc::c_int = 0;
    if (*ds).loop_0 == 0 as libc::c_int {
        memset(
            (*ds).pt.freq.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_int; 17]>() as libc::c_ulong,
        );
    }
    i = (*ds).loop_0;
    while i < (*ds).pt.len_size {
        if !((*br).cache_avail >= 4 as libc::c_int
            || lzx_br_fillup(strm, br) != 0
            || (*br).cache_avail >= 4 as libc::c_int)
        {
            (*ds).loop_0 = i;
            return 0 as libc::c_int;
        }
        *(*ds).pt.bitlen.offset(i as isize) =
            (((*br).cache_buffer >> (*br).cache_avail - 4 as libc::c_int) as uint32_t
                & cache_masks[4 as libc::c_int as usize]) as libc::c_uchar;
        (*ds).pt.freq[*(*ds).pt.bitlen.offset(i as isize) as usize] += 1;
        (*br).cache_avail -= 4 as libc::c_int;
        i += 1
    }
    (*ds).loop_0 = i;
    return 1 as libc::c_int;
}
/*
 * Read a bunch of bit-lengths from pre-tree.
 */
unsafe extern "C" fn lzx_read_bitlen(
    mut strm: *mut lzx_stream,
    mut d: *mut huffman,
    mut end: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ds: *mut lzx_dec = (*strm).ds;
    let mut br: *mut lzx_br = &mut (*ds).br;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut same: libc::c_int = 0;
    let mut rbits: libc::c_uint = 0;
    i = (*ds).loop_0;
    if i == 0 as libc::c_int {
        memset(
            (*d).freq.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_int; 17]>() as libc::c_ulong,
        );
    }
    ret = 0 as libc::c_int;
    if end < 0 as libc::c_int {
        end = (*d).len_size
    }
    loop {
        if !(i < end) {
            current_block = 5141539773904409130;
            break;
        }
        (*ds).loop_0 = i;
        if !((*br).cache_avail >= (*ds).pt.max_bits
            || lzx_br_fillup(strm, br) != 0
            || (*br).cache_avail >= (*ds).pt.max_bits)
        {
            current_block = 10506943081148444573;
            break;
        }
        rbits = ((*br).cache_buffer >> (*br).cache_avail - (*ds).pt.max_bits) as uint32_t
            & cache_masks[(*ds).pt.max_bits as usize];
        c = lzx_decode_huffman(&mut (*ds).pt, rbits);
        match c {
            17 => {
                /* several zero lengths, from 4 to 19. */
                if !((*br).cache_avail
                    >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int + 4 as libc::c_int
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail
                        >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int + 4 as libc::c_int)
                {
                    current_block = 10506943081148444573; /* Invalid */
                    break;
                }
                (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                same = (((*br).cache_buffer >> (*br).cache_avail - 4 as libc::c_int) as uint32_t
                    & cache_masks[4 as libc::c_int as usize])
                    .wrapping_add(4 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if i + same > end {
                    return -(1 as libc::c_int);
                }
                (*br).cache_avail -= 4 as libc::c_int;
                j = 0 as libc::c_int;
                while j < same {
                    let fresh11 = i;
                    i = i + 1;
                    *(*d).bitlen.offset(fresh11 as isize) = 0 as libc::c_int as libc::c_uchar;
                    j += 1
                }
            }
            18 => {
                /* many zero lengths, from 20 to 51. */
                if !((*br).cache_avail
                    >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int + 5 as libc::c_int
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail
                        >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int + 5 as libc::c_int)
                {
                    current_block = 10506943081148444573; /* Invalid */
                    break;
                }
                (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                same = (((*br).cache_buffer >> (*br).cache_avail - 5 as libc::c_int) as uint32_t
                    & cache_masks[5 as libc::c_int as usize])
                    .wrapping_add(20 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if i + same > end {
                    return -(1 as libc::c_int);
                }
                (*br).cache_avail -= 5 as libc::c_int;
                memset(
                    (*d).bitlen.offset(i as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    same as libc::c_ulong,
                );
                i += same
            }
            19 => {
                /* a few same lengths. */
                if !((*br).cache_avail
                    >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int
                        + 1 as libc::c_int
                        + (*ds).pt.max_bits
                    || lzx_br_fillup(strm, br) != 0
                    || (*br).cache_avail
                        >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int
                            + 1 as libc::c_int
                            + (*ds).pt.max_bits)
                {
                    current_block = 10506943081148444573; /* Invalid */
                    break; /* Invalid */
                }
                (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                same = (((*br).cache_buffer >> (*br).cache_avail - 1 as libc::c_int) as uint32_t
                    & cache_masks[1 as libc::c_int as usize])
                    .wrapping_add(4 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if i + same > end {
                    return -(1 as libc::c_int);
                }
                (*br).cache_avail -= 1 as libc::c_int;
                rbits = ((*br).cache_buffer >> (*br).cache_avail - (*ds).pt.max_bits) as uint32_t
                    & cache_masks[(*ds).pt.max_bits as usize];
                c = lzx_decode_huffman(&mut (*ds).pt, rbits);
                (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                c = (*(*d).bitlen.offset(i as isize) as libc::c_int - c + 17 as libc::c_int)
                    % 17 as libc::c_int;
                if c < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                j = 0 as libc::c_int;
                while j < same {
                    let fresh12 = i;
                    i = i + 1;
                    *(*d).bitlen.offset(fresh12 as isize) = c as libc::c_uchar;
                    j += 1
                }
                (*d).freq[c as usize] += same
            }
            _ => {
                (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                c = (*(*d).bitlen.offset(i as isize) as libc::c_int - c + 17 as libc::c_int)
                    % 17 as libc::c_int;
                if c < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                (*d).freq[c as usize] += 1;
                let fresh13 = i;
                i = i + 1;
                *(*d).bitlen.offset(fresh13 as isize) = c as libc::c_uchar
            }
        }
    }
    match current_block {
        5141539773904409130 => ret = 1 as libc::c_int,
        _ => {}
    }
    (*ds).loop_0 = i;
    return ret;
}
unsafe extern "C" fn lzx_huffman_init(
    mut hf: *mut huffman,
    mut len_size: size_t,
    mut tbl_bits: libc::c_int,
) -> libc::c_int {
    if (*hf).bitlen.is_null() || (*hf).len_size != len_size as libc::c_int {
        free((*hf).bitlen as *mut libc::c_void);
        (*hf).bitlen = calloc(
            len_size,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
        ) as *mut libc::c_uchar;
        if (*hf).bitlen.is_null() {
            return -(30 as libc::c_int);
        }
        (*hf).len_size = len_size as libc::c_int
    } else {
        memset(
            (*hf).bitlen as *mut libc::c_void,
            0 as libc::c_int,
            len_size.wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        );
    }
    if (*hf).tbl.is_null() {
        (*hf).tbl = malloc(
            ((1 as libc::c_int as size_t) << tbl_bits)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t;
        if (*hf).tbl.is_null() {
            return -(30 as libc::c_int);
        }
        (*hf).tbl_bits = tbl_bits
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lzx_huffman_free(mut hf: *mut huffman) {
    free((*hf).bitlen as *mut libc::c_void);
    free((*hf).tbl as *mut libc::c_void);
}
/*
 * Make a huffman coding table.
 */
unsafe extern "C" fn lzx_make_huffman_table(mut hf: *mut huffman) -> libc::c_int {
    let mut tbl: *mut uint16_t = 0 as *mut uint16_t;
    let mut bitlen: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut bitptn: [libc::c_int; 17] = [0; 17];
    let mut weight: [libc::c_int; 17] = [0; 17];
    let mut i: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0 as libc::c_int;
    let mut ptn: libc::c_int = 0;
    let mut tbl_size: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut len_avail: libc::c_int = 0;
    /*
     * Initialize bit patterns.
     */
    ptn = 0 as libc::c_int; /* Invalid */
    i = 1 as libc::c_int;
    w = (1 as libc::c_int) << 15 as libc::c_int;
    while i <= 16 as libc::c_int {
        bitptn[i as usize] = ptn;
        weight[i as usize] = w;
        if (*hf).freq[i as usize] != 0 {
            ptn += (*hf).freq[i as usize] * w;
            maxbits = i
        }
        i += 1;
        w >>= 1 as libc::c_int
    }
    if ptn & 0xffff as libc::c_int != 0 as libc::c_int || maxbits > (*hf).tbl_bits {
        return 0 as libc::c_int;
    }
    (*hf).max_bits = maxbits;
    /*
     * Cut out extra bits which we won't house in the table.
     * This preparation reduces the same calculation in the for-loop
     * making the table.
     */
    if maxbits < 16 as libc::c_int {
        let mut ebits: libc::c_int = 16 as libc::c_int - maxbits;
        i = 1 as libc::c_int;
        while i <= maxbits {
            bitptn[i as usize] >>= ebits;
            weight[i as usize] >>= ebits;
            i += 1
        }
    }
    /*
     * Make the table.
     */
    tbl_size = (1 as libc::c_int) << (*hf).tbl_bits;
    tbl = (*hf).tbl;
    bitlen = (*hf).bitlen;
    len_avail = (*hf).len_size;
    (*hf).tree_used = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len_avail {
        let mut p: *mut uint16_t = 0 as *mut uint16_t;
        let mut len: libc::c_int = 0;
        let mut cnt: libc::c_int = 0;
        if !(*bitlen.offset(i as isize) as libc::c_int == 0 as libc::c_int) {
            /* Get a bit pattern */
            len = *bitlen.offset(i as isize) as libc::c_int;
            if len > tbl_size {
                return 0 as libc::c_int;
            }
            ptn = bitptn[len as usize];
            cnt = weight[len as usize];
            /* Calculate next bit pattern */
            bitptn[len as usize] = ptn + cnt; /* Invalid */
            if bitptn[len as usize] > tbl_size {
                return 0 as libc::c_int;
            }
            /* Update the table */
            p = &mut *tbl.offset(ptn as isize) as *mut uint16_t;
            loop {
                cnt -= 1;
                if !(cnt >= 0 as libc::c_int) {
                    break;
                }
                *p.offset(cnt as isize) = i as uint16_t
            }
        }
        i += 1
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn lzx_decode_huffman(
    mut hf: *mut huffman,
    mut rbits: libc::c_uint,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = *(*hf).tbl.offset(rbits as isize) as libc::c_int;
    if c < (*hf).len_size {
        return c;
    }
    return 0 as libc::c_int;
}
