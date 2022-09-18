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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_pathname_w(_: *mut archive_entry) -> *const wchar_t;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_symlink_w(_: *mut archive_entry) -> *const wchar_t;
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
    fn archive_entry_unset_atime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_birthtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_unset_birthtime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_unset_ctime(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_copy_pathname_w(_: *mut archive_entry, _: *const wchar_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_unset_size(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_set_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_symlink_w(_: *mut archive_entry, _: *const wchar_t);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn archive_array_append(
        _: *mut archive_string,
        _: *const libc::c_char,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_wstring_concat(dest: *mut archive_wstring, src: *mut archive_wstring);
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_wstring_free(_: *mut archive_wstring);
    #[no_mangle]
    fn archive_mstring_clean(_: *mut archive_mstring);
    #[no_mangle]
    fn archive_wstrncat(
        _: *mut archive_wstring,
        _: *const wchar_t,
        _: size_t,
    ) -> *mut archive_wstring;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_mstring_get_wcs(
        _: *mut archive,
        _: *mut archive_mstring,
        _: *mut *const wchar_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_mstring_copy_mbs_len_l(
        _: *mut archive_mstring,
        mbs: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub struct lha {
    pub entry_offset: int64_t,
    pub entry_bytes_remaining: int64_t,
    pub entry_unconsumed: int64_t,
    pub entry_crc_calculated: uint16_t,
    pub header_size: size_t,
    pub level: libc::c_uchar,
    pub method: [libc::c_char; 3],
    pub compsize: int64_t,
    pub origsize: int64_t,
    pub setflag: libc::c_int,
    pub birthtime: time_t,
    pub birthtime_tv_nsec: libc::c_long,
    pub mtime: time_t,
    pub mtime_tv_nsec: libc::c_long,
    pub atime: time_t,
    pub atime_tv_nsec: libc::c_long,
    pub mode: mode_t,
    pub uid: int64_t,
    pub gid: int64_t,
    pub uname: archive_string,
    pub gname: archive_string,
    pub header_crc: uint16_t,
    pub crc: uint16_t,
    pub sconv_dir: *mut archive_string_conv,
    pub sconv_fname: *mut archive_string_conv,
    pub opt_sconv: *mut archive_string_conv,
    pub dirname: archive_string,
    pub filename: archive_string,
    pub ws: archive_wstring,
    pub dos_attr: libc::c_uchar,
    pub found_first_header: libc::c_char,
    pub directory: libc::c_char,
    pub decompress_init: libc::c_char,
    pub end_of_entry: libc::c_char,
    pub end_of_entry_cleanup: libc::c_char,
    pub entry_is_compressed: libc::c_char,
    pub format_name: [libc::c_char; 64],
    pub strm: lzh_stream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzh_stream {
    pub next_in: *const libc::c_uchar,
    pub avail_in: libc::c_int,
    pub total_in: int64_t,
    pub ref_ptr: *const libc::c_uchar,
    pub avail_out: libc::c_int,
    pub total_out: int64_t,
    pub ds: *mut lzh_dec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzh_dec {
    pub state: libc::c_int,
    pub w_size: libc::c_int,
    pub w_mask: libc::c_int,
    pub w_buff: *mut libc::c_uchar,
    pub w_pos: libc::c_int,
    pub copy_pos: libc::c_int,
    pub copy_len: libc::c_int,
    pub br: lzh_br,
    pub lt: huffman,
    pub pt: huffman,
    pub blocks_avail: libc::c_int,
    pub pos_pt_len_size: libc::c_int,
    pub pos_pt_len_bits: libc::c_int,
    pub literal_pt_len_size: libc::c_int,
    pub literal_pt_len_bits: libc::c_int,
    pub reading_position: libc::c_int,
    pub loop_0: libc::c_int,
    pub error: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffman {
    pub len_size: libc::c_int,
    pub len_avail: libc::c_int,
    pub len_bits: libc::c_int,
    pub freq: [libc::c_int; 17],
    pub bitlen: *mut libc::c_uchar,
    pub max_bits: libc::c_int,
    pub shift_bits: libc::c_int,
    pub tbl_bits: libc::c_int,
    pub tree_used: libc::c_int,
    pub tree_avail: libc::c_int,
    pub tbl: *mut uint16_t,
    pub tree: *mut htree_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htree_t {
    pub left: uint16_t,
    pub right: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzh_br {
    pub cache_buffer: uint64_t,
    pub cache_avail: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub i: uint32_t,
    pub c: [libc::c_char; 4],
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const UCHAR_MAX: libc::c_int = __SCHAR_MAX__ * 2 as libc::c_int + 1 as libc::c_int;
pub const __SCHAR_MAX__: libc::c_int = 127 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_LHA: libc::c_int = 0xb0000 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
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
/*-
 * Copyright (c) 2002 Thomas Moestl <tmm@FreeBSD.org>
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
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * $FreeBSD: head/lib/libarchive/archive_endian.h 201085 2009-12-28 02:17:15Z kientzle $
 *
 * Borrowed from FreeBSD's <sys/endian.h>
 */
/* Note:  This is a purely internal header! */
/* Do not use this outside of libarchive internal code! */
/*
 * Disabling inline keyword for compilers known to choke on it:
 * - Watcom C++ in C code.  (For any version?)
 * - SGI MIPSpro
 * - Microsoft Visual C++ 6.0 (supposedly newer versions too)
 * - IBM VisualAge 6 (XL v6)
 * - Sun WorkShop C (SunPro) before 5.9
 */
/* Alignment-agnostic encode/decode bytestream to/from little/big endian. */
#[inline]
unsafe extern "C" fn archive_be16dec(mut pp: *const libc::c_void) -> uint16_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return (p0 << 8 as libc::c_int | p1) as uint16_t;
}
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
/*-
 * Copyright (c) 2008-2014 Michihiro NAKAJIMA
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
pub const MAXMATCH: libc::c_int = 256 as libc::c_int;
/* Maximum match length. */
pub const MINMATCH: libc::c_int = 3 as libc::c_int;
/* Minimum match length. */
/*
 * Literal table format:
 * +0              +256                      +510
 * +---------------+-------------------------+
 * | literal code  |       match length      |
 * |   0 ... 255   |  MINMATCH ... MAXMATCH  |
 * +---------------+-------------------------+
 *  <---          LT_BITLEN_SIZE         --->
 */
/* Literal table size. */
pub const LT_BITLEN_SIZE: libc::c_int =
    UCHAR_MAX + 1 as libc::c_int + MAXMATCH - MINMATCH + 1 as libc::c_int;
/* Position table size.
 * Note: this used for both position table and pre literal table.*/
pub const PT_BITLEN_SIZE: libc::c_int = 3 as libc::c_int + 16 as libc::c_int;
pub const HTBL_BITS: libc::c_int = 10 as libc::c_int;
pub const BIRTHTIME_IS_SET: libc::c_int = 1 as libc::c_int;
pub const ATIME_IS_SET: libc::c_int = 2 as libc::c_int;
pub const UNIX_MODE_IS_SET: libc::c_int = 4 as libc::c_int;
pub const CRC_IS_SET: libc::c_int = 8 as libc::c_int;
/*
 * LHA header common member offset.
 */
pub const H_METHOD_OFFSET: libc::c_int = 2 as libc::c_int;
/* Compress type. */
pub const H_ATTR_OFFSET: libc::c_int = 19 as libc::c_int;
/* DOS attribute. */
pub const H_LEVEL_OFFSET: libc::c_int = 20 as libc::c_int;
/* Header Level.  */
pub const H_SIZE: libc::c_int = 22 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_lha(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut lha: *mut lha = 0 as *mut lha;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_lha\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    lha = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<lha>() as libc::c_ulong,
    ) as *mut lha;
    if lha.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate lha data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*lha).ws.s = NULL as *mut wchar_t;
    (*lha).ws.length = 0 as libc::c_int as size_t;
    (*lha).ws.buffer_length = 0 as libc::c_int as size_t;
    r = __archive_read_register_format(
        a,
        lha as *mut libc::c_void,
        b"lha\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_lha_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_lha_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_lha_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_lha_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_lha_read_data_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_lha_cleanup
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
        free(lha as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lha_check_header_format(mut h: *const libc::c_void) -> size_t {
    let mut p: *const libc::c_uchar = h as *const libc::c_uchar;
    let mut next_skip_bytes: size_t = 0;
    let mut current_block_11: u64;
    match *p.offset((H_METHOD_OFFSET + 3 as libc::c_int) as isize) as libc::c_int {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 100 | 115 => {
            /*
             * "-lh0-" ... "-lh7-" "-lhd-"
             * "-lzs-" "-lz5-"
             */
            next_skip_bytes = 4 as libc::c_int as size_t;
            /* b0 == 0 means the end of an LHa archive file.	*/
            if !(*p.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int) {
                if !(*p.offset(H_METHOD_OFFSET as isize) as libc::c_int != '-' as i32
                    || *p.offset((H_METHOD_OFFSET + 1 as libc::c_int) as isize) as libc::c_int
                        != 'l' as i32
                    || *p.offset((H_METHOD_OFFSET + 4 as libc::c_int) as isize) as libc::c_int
                        != '-' as i32)
                {
                    if *p.offset((H_METHOD_OFFSET + 2 as libc::c_int) as isize) as libc::c_int
                        == 'h' as i32
                    {
                        /* "-lh?-" */
                        if *p.offset((H_METHOD_OFFSET + 3 as libc::c_int) as isize) as libc::c_int
                            == 's' as i32
                        {
                            current_block_11 = 14648156034262866959;
                        } else {
                            if *p.offset(H_LEVEL_OFFSET as isize) as libc::c_int == 0 as libc::c_int
                            {
                                return 0 as libc::c_int as size_t;
                            }
                            if *p.offset(H_LEVEL_OFFSET as isize) as libc::c_int <= 3 as libc::c_int
                                && *p.offset(H_ATTR_OFFSET as isize) as libc::c_int
                                    == 0x20 as libc::c_int
                            {
                                return 0 as libc::c_int as size_t;
                            }
                            current_block_11 = 17860125682698302841;
                        }
                    } else {
                        current_block_11 = 17860125682698302841;
                    }
                    match current_block_11 {
                        14648156034262866959 => {}
                        _ => {
                            if *p.offset((H_METHOD_OFFSET + 2 as libc::c_int) as isize)
                                as libc::c_int
                                == 'z' as i32
                            {
                                /* LArc extensions: -lzs-,-lz4- and -lz5- */
                                if !(*p.offset(H_LEVEL_OFFSET as isize) as libc::c_int
                                    != 0 as libc::c_int)
                                {
                                    if *p.offset((H_METHOD_OFFSET + 3 as libc::c_int) as isize)
                                        as libc::c_int
                                        == 's' as i32
                                        || *p.offset((H_METHOD_OFFSET + 3 as libc::c_int) as isize)
                                            as libc::c_int
                                            == '4' as i32
                                        || *p.offset((H_METHOD_OFFSET + 3 as libc::c_int) as isize)
                                            as libc::c_int
                                            == '5' as i32
                                    {
                                        return 0 as libc::c_int as size_t;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        104 => next_skip_bytes = 1 as libc::c_int as size_t,
        122 => next_skip_bytes = 1 as libc::c_int as size_t,
        108 => next_skip_bytes = 2 as libc::c_int as size_t,
        45 => next_skip_bytes = 3 as libc::c_int as size_t,
        _ => next_skip_bytes = 4 as libc::c_int as size_t,
    }
    return next_skip_bytes;
}
/* Minimum header size. */
unsafe extern "C" fn archive_read_format_lha_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut buff: *const libc::c_void = 0 as *const libc::c_void;
    let mut bytes_avail: ssize_t = 0;
    let mut offset: ssize_t = 0;
    let mut window: ssize_t = 0;
    let mut next: size_t = 0;
    /* If there's already a better bid than we can ever
    make, don't bother testing. */
    if best_bid > 30 as libc::c_int {
        return -(1 as libc::c_int);
    }
    p = __archive_read_ahead(a, H_SIZE as size_t, NULL as *mut ssize_t) as *const libc::c_char;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if lha_check_header_format(p as *const libc::c_void) == 0 as libc::c_int as libc::c_ulong {
        return 30 as libc::c_int;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
    {
        /* PE file */
        offset = 0 as libc::c_int as ssize_t;
        window = 4096 as libc::c_int as ssize_t;
        while offset < (1024 as libc::c_int * 20 as libc::c_int) as libc::c_long {
            buff = __archive_read_ahead(a, (offset + window) as size_t, &mut bytes_avail);
            if buff == NULL as *const libc::c_void {
                /* Remaining bytes are less than window. */
                window >>= 1 as libc::c_int;
                if window < (H_SIZE + 3 as libc::c_int) as libc::c_long {
                    return 0 as libc::c_int;
                }
            } else {
                p = (buff as *const libc::c_char).offset(offset as isize);
                while p.offset(H_SIZE as isize)
                    < (buff as *const libc::c_char).offset(bytes_avail as isize)
                {
                    next = lha_check_header_format(p as *const libc::c_void);
                    if next == 0 as libc::c_int as libc::c_ulong {
                        return 30 as libc::c_int;
                    }
                    p = p.offset(next as isize)
                }
                offset = p.offset_from(buff as *const libc::c_char) as libc::c_long
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_lha_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut lha: *mut lha = 0 as *mut lha;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    lha = (*(*a).format).data as *mut lha;
    if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if val.is_null()
            || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"lha: hdrcharset option needs a character-set name\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            (*lha).opt_sconv =
                archive_string_conversion_from_charset(&mut (*a).archive, val, 0 as libc::c_int);
            if !(*lha).opt_sconv.is_null() {
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
unsafe extern "C" fn lha_skip_sfx(mut a: *mut archive_read) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut next: size_t = 0;
    let mut skip: size_t = 0;
    let mut bytes: ssize_t = 0;
    let mut window: ssize_t = 0;
    window = 4096 as libc::c_int as ssize_t;
    loop {
        h = __archive_read_ahead(a, window as size_t, &mut bytes);
        if h == NULL as *const libc::c_void {
            /* Remaining bytes are less than window. */
            window >>= 1 as libc::c_int;
            if window < (H_SIZE + 3 as libc::c_int) as libc::c_long {
                break;
            }
        } else {
            if bytes < H_SIZE as libc::c_long {
                break;
            }
            p = h as *const libc::c_char;
            q = p.offset(bytes as isize);
            /*
             * Scan ahead until we find something that looks
             * like the lha header.
             */
            while p.offset(H_SIZE as isize) < q {
                next = lha_check_header_format(p as *const libc::c_void);
                if next == 0 as libc::c_int as libc::c_ulong {
                    skip =
                        p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
                    __archive_read_consume(a, skip as int64_t);
                    return 0 as libc::c_int;
                }
                p = p.offset(next as isize)
            }
            skip = p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
            __archive_read_consume(a, skip as int64_t);
        }
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Couldn\'t find out LHa header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn truncated_error(mut a: *mut archive_read) -> libc::c_int {
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Truncated LHa header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn archive_read_format_lha_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut linkname: archive_wstring = archive_wstring {
        s: 0 as *mut wchar_t,
        length: 0,
        buffer_length: 0,
    };
    let mut pathname: archive_wstring = archive_wstring {
        s: 0 as *mut wchar_t,
        length: 0,
        buffer_length: 0,
    };
    let mut lha: *mut lha = 0 as *mut lha;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut signature: *const libc::c_char = 0 as *const libc::c_char;
    let mut err: libc::c_int = 0;
    let mut conv_buffer: archive_mstring = archive_mstring {
        aes_mbs: archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        },
        aes_utf8: archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        },
        aes_wcs: archive_wstring {
            s: 0 as *mut wchar_t,
            length: 0,
            buffer_length: 0,
        },
        aes_mbs_in_locale: archive_string {
            s: 0 as *mut libc::c_char,
            length: 0,
            buffer_length: 0,
        },
        aes_set: 0,
    };
    let mut conv_buffer_p: *const wchar_t = 0 as *const wchar_t;
    lha_crc16_init();
    (*a).archive.archive_format = ARCHIVE_FORMAT_LHA;
    if (*a).archive.archive_format_name.is_null() {
        (*a).archive.archive_format_name = b"lha\x00" as *const u8 as *const libc::c_char
    }
    lha = (*(*a).format).data as *mut lha;
    (*lha).decompress_init = 0 as libc::c_int as libc::c_char;
    (*lha).end_of_entry = 0 as libc::c_int as libc::c_char;
    (*lha).end_of_entry_cleanup = 0 as libc::c_int as libc::c_char;
    (*lha).entry_unconsumed = 0 as libc::c_int as int64_t;
    p = __archive_read_ahead(a, H_SIZE as size_t, NULL as *mut ssize_t) as *const libc::c_uchar;
    if p.is_null() {
        /*
         * LHa archiver added 0 to the tail of its archive file as
         * the mark of the end of the archive.
         */
        signature = __archive_read_ahead(
            a,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            NULL as *mut ssize_t,
        ) as *const libc::c_char;
        if signature.is_null()
            || *signature.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        return truncated_error(a);
    }
    signature = p as *const libc::c_char;
    if (*lha).found_first_header as libc::c_int == 0 as libc::c_int
        && *signature.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *signature.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
    {
        /* This is an executable?  Must be self-extracting... 	*/
        err = lha_skip_sfx(a);
        if err < ARCHIVE_WARN {
            return err;
        }
        p = __archive_read_ahead(
            a,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
            NULL as *mut ssize_t,
        ) as *const libc::c_uchar;
        if p.is_null() {
            return truncated_error(a);
        }
        signature = p as *const libc::c_char
    }
    /* signature[0] == 0 means the end of an LHa archive file. */
    if *signature.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    /*
     * Check the header format and method type.
     */
    if lha_check_header_format(p as *const libc::c_void) != 0 as libc::c_int as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Bad LHa file\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* We've found the first header. */
    (*lha).found_first_header = 1 as libc::c_int as libc::c_char;
    /* Set a default value and common data */
    (*lha).header_size = 0 as libc::c_int as size_t;
    (*lha).level = *p.offset(H_LEVEL_OFFSET as isize);
    (*lha).method[0 as libc::c_int as usize] =
        *p.offset((H_METHOD_OFFSET + 1 as libc::c_int) as isize) as libc::c_char;
    (*lha).method[1 as libc::c_int as usize] =
        *p.offset((H_METHOD_OFFSET + 2 as libc::c_int) as isize) as libc::c_char;
    (*lha).method[2 as libc::c_int as usize] =
        *p.offset((H_METHOD_OFFSET + 3 as libc::c_int) as isize) as libc::c_char;
    if memcmp(
        (*lha).method.as_mut_ptr() as *const libc::c_void,
        b"lhd\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        (*lha).directory = 1 as libc::c_int as libc::c_char
    } else {
        (*lha).directory = 0 as libc::c_int as libc::c_char
    }
    if memcmp(
        (*lha).method.as_mut_ptr() as *const libc::c_void,
        b"lh0\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || memcmp(
            (*lha).method.as_mut_ptr() as *const libc::c_void,
            b"lz4\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (*lha).entry_is_compressed = 0 as libc::c_int as libc::c_char
    } else {
        (*lha).entry_is_compressed = 1 as libc::c_int as libc::c_char
    }
    (*lha).compsize = 0 as libc::c_int as int64_t;
    (*lha).origsize = 0 as libc::c_int as int64_t;
    (*lha).setflag = 0 as libc::c_int;
    (*lha).birthtime = 0 as libc::c_int as time_t;
    (*lha).birthtime_tv_nsec = 0 as libc::c_int as libc::c_long;
    (*lha).mtime = 0 as libc::c_int as time_t;
    (*lha).mtime_tv_nsec = 0 as libc::c_int as libc::c_long;
    (*lha).atime = 0 as libc::c_int as time_t;
    (*lha).atime_tv_nsec = 0 as libc::c_int as libc::c_long;
    (*lha).mode = if (*lha).directory as libc::c_int != 0 {
        0o777 as libc::c_int
    } else {
        0o666 as libc::c_int
    } as mode_t;
    (*lha).uid = 0 as libc::c_int as int64_t;
    (*lha).gid = 0 as libc::c_int as int64_t;
    (*lha).dirname.length = 0 as libc::c_int as size_t;
    (*lha).filename.length = 0 as libc::c_int as size_t;
    (*lha).dos_attr = 0 as libc::c_int as libc::c_uchar;
    if !(*lha).opt_sconv.is_null() {
        (*lha).sconv_dir = (*lha).opt_sconv;
        (*lha).sconv_fname = (*lha).opt_sconv
    } else {
        (*lha).sconv_dir = NULL as *mut archive_string_conv;
        (*lha).sconv_fname = NULL as *mut archive_string_conv
    }
    match *p.offset(H_LEVEL_OFFSET as isize) as libc::c_int {
        0 => err = lha_read_file_header_0(a, lha),
        1 => err = lha_read_file_header_1(a, lha),
        2 => err = lha_read_file_header_2(a, lha),
        3 => err = lha_read_file_header_3(a, lha),
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unsupported LHa header level %d\x00" as *const u8 as *const libc::c_char,
                *p.offset(H_LEVEL_OFFSET as isize) as libc::c_int,
            );
            err = ARCHIVE_FATAL
        }
    }
    if err < ARCHIVE_WARN {
        return err;
    }
    if (*lha).directory == 0 && (*lha).filename.length == 0 as libc::c_int as libc::c_ulong {
        /* The filename has not been set */
        return truncated_error(a);
    }
    /*
     * Make a pathname from a dirname and a filename, after converting to Unicode.
     * This is because codepages might differ between dirname and filename.
    	*/
    pathname.s = NULL as *mut wchar_t;
    pathname.length = 0 as libc::c_int as size_t;
    pathname.buffer_length = 0 as libc::c_int as size_t;
    linkname.s = NULL as *mut wchar_t;
    linkname.length = 0 as libc::c_int as size_t;
    linkname.buffer_length = 0 as libc::c_int as size_t;
    conv_buffer.aes_mbs.s = NULL as *mut libc::c_char;
    conv_buffer.aes_mbs.length = 0 as libc::c_int as size_t;
    conv_buffer.aes_mbs.buffer_length = 0 as libc::c_int as size_t;
    conv_buffer.aes_mbs_in_locale.s = NULL as *mut libc::c_char;
    conv_buffer.aes_mbs_in_locale.length = 0 as libc::c_int as size_t;
    conv_buffer.aes_mbs_in_locale.buffer_length = 0 as libc::c_int as size_t;
    conv_buffer.aes_utf8.s = NULL as *mut libc::c_char;
    conv_buffer.aes_utf8.length = 0 as libc::c_int as size_t;
    conv_buffer.aes_utf8.buffer_length = 0 as libc::c_int as size_t;
    conv_buffer.aes_wcs.s = NULL as *mut wchar_t;
    conv_buffer.aes_wcs.length = 0 as libc::c_int as size_t;
    conv_buffer.aes_wcs.buffer_length = 0 as libc::c_int as size_t;
    if 0 as libc::c_int
        != archive_mstring_copy_mbs_len_l(
            &mut conv_buffer,
            (*lha).dirname.s,
            (*lha).dirname.length,
            (*lha).sconv_dir,
        )
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Pathname cannot be converted from %s to Unicode.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name((*lha).sconv_dir),
        );
        err = ARCHIVE_FATAL
    } else if 0 as libc::c_int
        != archive_mstring_get_wcs(&mut (*a).archive, &mut conv_buffer, &mut conv_buffer_p)
    {
        err = ARCHIVE_FATAL
    }
    if err == ARCHIVE_FATAL {
        archive_mstring_clean(&mut conv_buffer);
        archive_wstring_free(&mut pathname);
        archive_wstring_free(&mut linkname);
        return err;
    }
    pathname.length = 0 as libc::c_int as size_t;
    archive_wstring_concat(&mut pathname, &mut conv_buffer.aes_wcs);
    conv_buffer.aes_mbs.length = 0 as libc::c_int as size_t;
    conv_buffer.aes_mbs_in_locale.length = 0 as libc::c_int as size_t;
    conv_buffer.aes_utf8.length = 0 as libc::c_int as size_t;
    conv_buffer.aes_wcs.length = 0 as libc::c_int as size_t;
    if 0 as libc::c_int
        != archive_mstring_copy_mbs_len_l(
            &mut conv_buffer,
            (*lha).filename.s,
            (*lha).filename.length,
            (*lha).sconv_fname,
        )
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Pathname cannot be converted from %s to Unicode.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name((*lha).sconv_fname),
        );
        err = ARCHIVE_FATAL
    } else if 0 as libc::c_int
        != archive_mstring_get_wcs(&mut (*a).archive, &mut conv_buffer, &mut conv_buffer_p)
    {
        err = ARCHIVE_FATAL
    }
    if err == ARCHIVE_FATAL {
        archive_mstring_clean(&mut conv_buffer);
        archive_wstring_free(&mut pathname);
        archive_wstring_free(&mut linkname);
        return err;
    }
    archive_wstring_concat(&mut pathname, &mut conv_buffer.aes_wcs);
    archive_mstring_clean(&mut conv_buffer);
    if (*lha).mode & AE_IFMT as mode_t == AE_IFLNK as mode_t {
        /*
         * Extract the symlink-name if it's included in the pathname.
         */
        if lha_parse_linkname(&mut linkname, &mut pathname) == 0 {
            /* We couldn't get the symlink-name. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unknown symlink-name\x00" as *const u8 as *const libc::c_char,
            );
            archive_wstring_free(&mut pathname);
            archive_wstring_free(&mut linkname);
            return -(25 as libc::c_int);
        }
    } else {
        /*
         * Make sure a file-type is set.
         * The mode has been overridden if it is in the extended data.
         */
        (*lha).mode = (*lha).mode & !(AE_IFMT as mode_t)
            | (if (*lha).directory as libc::c_int != 0 {
                AE_IFDIR as mode_t
            } else {
                AE_IFREG as mode_t
            })
    } /* read only. */
    if (*lha).setflag & UNIX_MODE_IS_SET == 0 as libc::c_int
        && (*lha).dos_attr as libc::c_int & 1 as libc::c_int != 0 as libc::c_int
    {
        (*lha).mode &= !(0o222 as libc::c_int) as libc::c_uint
    }
    /*
     * Set basic file parameters.
     */
    archive_entry_copy_pathname_w(entry, pathname.s);
    archive_wstring_free(&mut pathname);
    if linkname.length > 0 as libc::c_int as libc::c_ulong {
        archive_entry_copy_symlink_w(entry, linkname.s);
    } else {
        archive_entry_set_symlink(entry, NULL as *const libc::c_char);
    }
    archive_wstring_free(&mut linkname);
    /*
     * When a header level is 0, there is a possibility that
     * a pathname and a symlink has '\' character, a directory
     * separator in DOS/Windows. So we should convert it to '/'.
     */
    if *p.offset(H_LEVEL_OFFSET as isize) as libc::c_int == 0 as libc::c_int {
        lha_replace_path_separator(lha, entry);
    }
    archive_entry_set_mode(entry, (*lha).mode);
    archive_entry_set_uid(entry, (*lha).uid);
    archive_entry_set_gid(entry, (*lha).gid);
    if (*lha).uname.length > 0 as libc::c_int as libc::c_ulong {
        archive_entry_set_uname(entry, (*lha).uname.s);
    }
    if (*lha).gname.length > 0 as libc::c_int as libc::c_ulong {
        archive_entry_set_gname(entry, (*lha).gname.s);
    }
    if (*lha).setflag & BIRTHTIME_IS_SET != 0 {
        archive_entry_set_birthtime(entry, (*lha).birthtime, (*lha).birthtime_tv_nsec);
        archive_entry_set_ctime(entry, (*lha).birthtime, (*lha).birthtime_tv_nsec);
    } else {
        archive_entry_unset_birthtime(entry);
        archive_entry_unset_ctime(entry);
    }
    archive_entry_set_mtime(entry, (*lha).mtime, (*lha).mtime_tv_nsec);
    if (*lha).setflag & ATIME_IS_SET != 0 {
        archive_entry_set_atime(entry, (*lha).atime, (*lha).atime_tv_nsec);
    } else {
        archive_entry_unset_atime(entry);
    }
    if (*lha).directory as libc::c_int != 0 || !archive_entry_symlink(entry).is_null() {
        archive_entry_unset_size(entry);
    } else {
        archive_entry_set_size(entry, (*lha).origsize);
    }
    /*
     * Prepare variables used to read a file content.
     */
    (*lha).entry_bytes_remaining = (*lha).compsize;
    if (*lha).entry_bytes_remaining < 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid LHa entry size\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*lha).entry_offset = 0 as libc::c_int as int64_t;
    (*lha).entry_crc_calculated = 0 as libc::c_int as uint16_t;
    /*
     * This file does not have a content.
     */
    if (*lha).directory as libc::c_int != 0 || (*lha).compsize == 0 as libc::c_int as libc::c_long {
        (*lha).end_of_entry = 1 as libc::c_int as libc::c_char
    }
    sprintf(
        (*lha).format_name.as_mut_ptr(),
        b"lha -%c%c%c-\x00" as *const u8 as *const libc::c_char,
        (*lha).method[0 as libc::c_int as usize] as libc::c_int,
        (*lha).method[1 as libc::c_int as usize] as libc::c_int,
        (*lha).method[2 as libc::c_int as usize] as libc::c_int,
    );
    (*a).archive.archive_format_name = (*lha).format_name.as_mut_ptr();
    return err;
}
/*
 * Replace a DOS path separator '\' by a character '/'.
 * Some multi-byte character set have  a character '\' in its second byte.
 */
unsafe extern "C" fn lha_replace_path_separator(mut lha: *mut lha, mut entry: *mut archive_entry) {
    let mut wp: *const wchar_t = 0 as *const wchar_t;
    let mut i: size_t = 0;
    wp = archive_entry_pathname_w(entry);
    if !wp.is_null() {
        (*lha).ws.length = 0 as libc::c_int as size_t;
        archive_wstrncat(
            &mut (*lha).ws,
            wp,
            (if wp.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                wcslen(wp)
            }),
        );
        i = 0 as libc::c_int as size_t;
        while i < (*lha).ws.length {
            if *(*lha).ws.s.offset(i as isize) == '\\' as i32 {
                *(*lha).ws.s.offset(i as isize) = '/' as i32
            }
            i = i.wrapping_add(1)
        }
        archive_entry_copy_pathname_w(entry, (*lha).ws.s);
    }
    wp = archive_entry_symlink_w(entry);
    if !wp.is_null() {
        (*lha).ws.length = 0 as libc::c_int as size_t;
        archive_wstrncat(
            &mut (*lha).ws,
            wp,
            (if wp.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                wcslen(wp)
            }),
        );
        i = 0 as libc::c_int as size_t;
        while i < (*lha).ws.length {
            if *(*lha).ws.s.offset(i as isize) == '\\' as i32 {
                *(*lha).ws.s.offset(i as isize) = '/' as i32
            }
            i = i.wrapping_add(1)
        }
        archive_entry_copy_symlink_w(entry, (*lha).ws.s);
    };
}
/*
 * Header 0 format
 *
 * +0              +1         +2               +7                  +11
 * +---------------+----------+----------------+-------------------+
 * |header size(*1)|header sum|compression type|compressed size(*2)|
 * +---------------+----------+----------------+-------------------+
 *                             <---------------------(*1)----------*
 *
 * +11               +15       +17       +19            +20              +21
 * +-----------------+---------+---------+--------------+----------------+
 * |uncompressed size|time(DOS)|date(DOS)|attribute(DOS)|header level(=0)|
 * +-----------------+---------+---------+--------------+----------------+
 * *--------------------------------(*1)---------------------------------*
 *
 * +21             +22       +22+(*3)   +22+(*3)+2       +22+(*3)+2+(*4)
 * +---------------+---------+----------+----------------+------------------+
 * |name length(*3)|file name|file CRC16|extra header(*4)|  compressed data |
 * +---------------+---------+----------+----------------+------------------+
 *                  <--(*3)->                             <------(*2)------>
 * *----------------------(*1)-------------------------->
 *
 */
pub const H0_HEADER_SIZE_OFFSET: libc::c_int = 0 as libc::c_int;
pub const H0_HEADER_SUM_OFFSET: libc::c_int = 1 as libc::c_int;
pub const H0_COMP_SIZE_OFFSET: libc::c_int = 7 as libc::c_int;
pub const H0_ORIG_SIZE_OFFSET: libc::c_int = 11 as libc::c_int;
pub const H0_DOS_TIME_OFFSET: libc::c_int = 15 as libc::c_int;
pub const H0_NAME_LEN_OFFSET: libc::c_int = 21 as libc::c_int;
pub const H0_FILE_NAME_OFFSET: libc::c_int = 22 as libc::c_int;
pub const H0_FIXED_SIZE: libc::c_int = 24 as libc::c_int;
unsafe extern "C" fn lha_read_file_header_0(
    mut a: *mut archive_read,
    mut lha: *mut lha,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut extdsize: libc::c_int = 0;
    let mut namelen: libc::c_int = 0;
    let mut headersum: libc::c_uchar = 0;
    let mut sum_calculated: libc::c_uchar = 0;
    p = __archive_read_ahead(a, H0_FIXED_SIZE as size_t, NULL as *mut ssize_t)
        as *const libc::c_uchar;
    if p.is_null() {
        return truncated_error(a);
    }
    (*lha).header_size =
        (*p.offset(H0_HEADER_SIZE_OFFSET as isize) as libc::c_int + 2 as libc::c_int) as size_t;
    headersum = *p.offset(H0_HEADER_SUM_OFFSET as isize);
    (*lha).compsize =
        archive_le32dec(p.offset(H0_COMP_SIZE_OFFSET as isize) as *const libc::c_void) as int64_t;
    (*lha).origsize =
        archive_le32dec(p.offset(H0_ORIG_SIZE_OFFSET as isize) as *const libc::c_void) as int64_t;
    (*lha).mtime = lha_dos_time(p.offset(H0_DOS_TIME_OFFSET as isize));
    namelen = *p.offset(H0_NAME_LEN_OFFSET as isize) as libc::c_int;
    extdsize = (*lha).header_size as libc::c_int - H0_FIXED_SIZE - namelen;
    if (namelen > 221 as libc::c_int || extdsize < 0 as libc::c_int)
        && extdsize != -(2 as libc::c_int)
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid LHa header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    p = __archive_read_ahead(a, (*lha).header_size, NULL as *mut ssize_t) as *const libc::c_uchar;
    if p.is_null() {
        return truncated_error(a);
    }
    (*lha).filename.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*lha).filename,
        p.offset(22 as libc::c_int as isize) as *const libc::c_void,
        namelen as size_t,
    );
    /* When extdsize == -2, A CRC16 value is not present in the header. */
    if extdsize >= 0 as libc::c_int {
        (*lha).crc = archive_le16dec(
            p.offset(H0_FILE_NAME_OFFSET as isize)
                .offset(namelen as isize) as *const libc::c_void,
        );
        (*lha).setflag |= CRC_IS_SET
    }
    sum_calculated = lha_calcsum(
        0 as libc::c_int as libc::c_uchar,
        p as *const libc::c_void,
        2 as libc::c_int,
        (*lha)
            .header_size
            .wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    /* Read an extended header */
    if extdsize > 0 as libc::c_int {
        /* This extended data is set by 'LHa for UNIX' only.
         * Maybe fixed size.
         */
        p = p.offset((H0_FILE_NAME_OFFSET + namelen + 2 as libc::c_int) as isize);
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'U' as i32
            && extdsize == 12 as libc::c_int
        {
            /* p[1] is a minor version. */
            (*lha).mtime = archive_le32dec(&*p.offset(2 as libc::c_int as isize)
                as *const libc::c_uchar
                as *const libc::c_void) as time_t;
            (*lha).mode = archive_le16dec(&*p.offset(6 as libc::c_int as isize)
                as *const libc::c_uchar
                as *const libc::c_void) as mode_t;
            (*lha).uid = archive_le16dec(&*p.offset(8 as libc::c_int as isize)
                as *const libc::c_uchar
                as *const libc::c_void) as int64_t;
            (*lha).gid = archive_le16dec(&*p.offset(10 as libc::c_int as isize)
                as *const libc::c_uchar
                as *const libc::c_void) as int64_t;
            (*lha).setflag |= UNIX_MODE_IS_SET
        }
    }
    __archive_read_consume(a, (*lha).header_size as int64_t);
    if sum_calculated as libc::c_int != headersum as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"LHa header sum error\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/*
 * Header 1 format
 *
 * +0              +1         +2               +7            +11
 * +---------------+----------+----------------+-------------+
 * |header size(*1)|header sum|compression type|skip size(*2)|
 * +---------------+----------+----------------+-------------+
 *                             <---------------(*1)----------*
 *
 * +11               +15       +17       +19            +20              +21
 * +-----------------+---------+---------+--------------+----------------+
 * |uncompressed size|time(DOS)|date(DOS)|attribute(DOS)|header level(=1)|
 * +-----------------+---------+---------+--------------+----------------+
 * *-------------------------------(*1)----------------------------------*
 *
 * +21             +22       +22+(*3)   +22+(*3)+2  +22+(*3)+3  +22+(*3)+3+(*4)
 * +---------------+---------+----------+-----------+-----------+
 * |name length(*3)|file name|file CRC16|  creator  |padding(*4)|
 * +---------------+---------+----------+-----------+-----------+
 *                  <--(*3)->
 * *----------------------------(*1)----------------------------*
 *
 * +22+(*3)+3+(*4)  +22+(*3)+3+(*4)+2     +22+(*3)+3+(*4)+2+(*5)
 * +----------------+---------------------+------------------------+
 * |next header size| extended header(*5) |     compressed data    |
 * +----------------+---------------------+------------------------+
 * *------(*1)-----> <--------------------(*2)-------------------->
 */
pub const H1_HEADER_SIZE_OFFSET: libc::c_int = 0 as libc::c_int;
pub const H1_HEADER_SUM_OFFSET: libc::c_int = 1 as libc::c_int;
pub const H1_COMP_SIZE_OFFSET: libc::c_int = 7 as libc::c_int;
pub const H1_ORIG_SIZE_OFFSET: libc::c_int = 11 as libc::c_int;
pub const H1_DOS_TIME_OFFSET: libc::c_int = 15 as libc::c_int;
pub const H1_NAME_LEN_OFFSET: libc::c_int = 21 as libc::c_int;
pub const H1_FILE_NAME_OFFSET: libc::c_int = 22 as libc::c_int;
pub const H1_FIXED_SIZE: libc::c_int = 27 as libc::c_int;
unsafe extern "C" fn lha_read_file_header_1(
    mut a: *mut archive_read,
    mut lha: *mut lha,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut extdsize: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut err2: libc::c_int = 0;
    let mut namelen: libc::c_int = 0;
    let mut padding: libc::c_int = 0;
    let mut headersum: libc::c_uchar = 0;
    let mut sum_calculated: libc::c_uchar = 0;
    err = ARCHIVE_OK;
    p = __archive_read_ahead(a, H1_FIXED_SIZE as size_t, NULL as *mut ssize_t)
        as *const libc::c_uchar;
    if p.is_null() {
        return truncated_error(a);
    }
    (*lha).header_size =
        (*p.offset(H1_HEADER_SIZE_OFFSET as isize) as libc::c_int + 2 as libc::c_int) as size_t;
    headersum = *p.offset(H1_HEADER_SUM_OFFSET as isize);
    /* Note: An extended header size is included in a compsize. */
    (*lha).compsize =
        archive_le32dec(p.offset(H1_COMP_SIZE_OFFSET as isize) as *const libc::c_void) as int64_t;
    (*lha).origsize =
        archive_le32dec(p.offset(H1_ORIG_SIZE_OFFSET as isize) as *const libc::c_void) as int64_t;
    (*lha).mtime = lha_dos_time(p.offset(H1_DOS_TIME_OFFSET as isize));
    namelen = *p.offset(H1_NAME_LEN_OFFSET as isize) as libc::c_int;
    /* Calculate a padding size. The result will be normally 0 only(?) */
    padding = (*lha).header_size as libc::c_int - H1_FIXED_SIZE - namelen;
    if !(namelen > 230 as libc::c_int || padding < 0 as libc::c_int) {
        p = __archive_read_ahead(a, (*lha).header_size, NULL as *mut ssize_t)
            as *const libc::c_uchar;
        if p.is_null() {
            return truncated_error(a);
        }
        i = 0 as libc::c_int;
        loop {
            if !(i < namelen) {
                current_block = 17833034027772472439;
                break;
            }
            if *p.offset((i + H1_FILE_NAME_OFFSET) as isize) as libc::c_int == 0xff as libc::c_int {
                current_block = 2240781809193226912;
                break;
                /* Invalid filename. */
            }
            i += 1
        }
        match current_block {
            2240781809193226912 => {}
            _ => {
                (*lha).filename.length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut (*lha).filename,
                    p.offset(22 as libc::c_int as isize) as *const libc::c_void,
                    namelen as size_t,
                );
                (*lha).crc = archive_le16dec(
                    p.offset(H1_FILE_NAME_OFFSET as isize)
                        .offset(namelen as isize) as *const libc::c_void,
                );
                (*lha).setflag |= CRC_IS_SET;
                sum_calculated = lha_calcsum(
                    0 as libc::c_int as libc::c_uchar,
                    p as *const libc::c_void,
                    2 as libc::c_int,
                    (*lha)
                        .header_size
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong),
                );
                /* Consume used bytes but not include `next header size' data
                 * since it will be consumed in lha_read_file_extended_header(). */
                __archive_read_consume(
                    a,
                    (*lha)
                        .header_size
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                        as int64_t,
                );
                /* Read extended headers */
                err2 = lha_read_file_extended_header(
                    a,
                    lha,
                    NULL as *mut uint16_t,
                    2 as libc::c_int,
                    ((*lha).compsize + 2 as libc::c_int as libc::c_long) as size_t,
                    &mut extdsize,
                );
                if err2 < ARCHIVE_WARN {
                    return err2;
                }
                if err2 < err {
                    err = err2
                }
                /* Get a real compressed file size. */
                (*lha).compsize = ((*lha).compsize as libc::c_ulong)
                    .wrapping_sub(extdsize.wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    as int64_t as int64_t; /* Invalid compressed file size */
                if !((*lha).compsize < 0 as libc::c_int as libc::c_long) {
                    if sum_calculated as libc::c_int != headersum as libc::c_int {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"LHa header sum error\x00" as *const u8 as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    return err;
                }
            }
        }
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Invalid LHa header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
/*
 * Header 2 format
 *
 * +0              +2               +7                  +11               +15
 * +---------------+----------------+-------------------+-----------------+
 * |header size(*1)|compression type|compressed size(*2)|uncompressed size|
 * +---------------+----------------+-------------------+-----------------+
 *  <--------------------------------(*1)---------------------------------*
 *
 * +15               +19          +20              +21        +23         +24
 * +-----------------+------------+----------------+----------+-----------+
 * |data/time(time_t)| 0x20 fixed |header level(=2)|file CRC16|  creator  |
 * +-----------------+------------+----------------+----------+-----------+
 * *---------------------------------(*1)---------------------------------*
 *
 * +24              +26                 +26+(*3)      +26+(*3)+(*4)
 * +----------------+-------------------+-------------+-------------------+
 * |next header size|extended header(*3)| padding(*4) |  compressed data  |
 * +----------------+-------------------+-------------+-------------------+
 * *--------------------------(*1)-------------------> <------(*2)------->
 *
 */
pub const H2_HEADER_SIZE_OFFSET: libc::c_int = 0 as libc::c_int;
pub const H2_COMP_SIZE_OFFSET: libc::c_int = 7 as libc::c_int;
pub const H2_ORIG_SIZE_OFFSET: libc::c_int = 11 as libc::c_int;
pub const H2_TIME_OFFSET: libc::c_int = 15 as libc::c_int;
pub const H2_CRC_OFFSET: libc::c_int = 21 as libc::c_int;
pub const H2_FIXED_SIZE: libc::c_int = 24 as libc::c_int;
unsafe extern "C" fn lha_read_file_header_2(
    mut a: *mut archive_read,
    mut lha: *mut lha,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut extdsize: size_t = 0;
    let mut err: libc::c_int = 0;
    let mut padding: libc::c_int = 0;
    let mut header_crc: uint16_t = 0;
    p = __archive_read_ahead(a, H2_FIXED_SIZE as size_t, NULL as *mut ssize_t)
        as *const libc::c_uchar;
    if p.is_null() {
        return truncated_error(a);
    }
    (*lha).header_size =
        archive_le16dec(p.offset(H2_HEADER_SIZE_OFFSET as isize) as *const libc::c_void) as size_t;
    (*lha).compsize =
        archive_le32dec(p.offset(H2_COMP_SIZE_OFFSET as isize) as *const libc::c_void) as int64_t;
    (*lha).origsize =
        archive_le32dec(p.offset(H2_ORIG_SIZE_OFFSET as isize) as *const libc::c_void) as int64_t;
    (*lha).mtime =
        archive_le32dec(p.offset(H2_TIME_OFFSET as isize) as *const libc::c_void) as time_t;
    (*lha).crc = archive_le16dec(p.offset(H2_CRC_OFFSET as isize) as *const libc::c_void);
    (*lha).setflag |= CRC_IS_SET;
    if (*lha).header_size < H2_FIXED_SIZE as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid LHa header size\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    header_crc = lha_crc16(
        0 as libc::c_int as uint16_t,
        p as *const libc::c_void,
        H2_FIXED_SIZE as size_t,
    );
    __archive_read_consume(a, H2_FIXED_SIZE as int64_t);
    /* Read extended headers */
    err = lha_read_file_extended_header(
        a,
        lha,
        &mut header_crc,
        2 as libc::c_int,
        (*lha)
            .header_size
            .wrapping_sub(H2_FIXED_SIZE as libc::c_ulong),
        &mut extdsize,
    );
    if err < ARCHIVE_WARN {
        return err;
    }
    /* Calculate a padding size. The result will be normally 0 or 1. */
    padding = (*lha).header_size as libc::c_int
        - (H2_FIXED_SIZE as libc::c_ulong).wrapping_add(extdsize) as libc::c_int;
    if padding > 0 as libc::c_int {
        p = __archive_read_ahead(a, padding as size_t, NULL as *mut ssize_t)
            as *const libc::c_uchar;
        if p.is_null() {
            return truncated_error(a);
        }
        header_crc = lha_crc16(header_crc, p as *const libc::c_void, padding as size_t);
        __archive_read_consume(a, padding as int64_t);
    }
    if header_crc as libc::c_int != (*lha).header_crc as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"LHa header CRC error\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return err;
}
/*
 * Header 3 format
 *
 * +0           +2               +7                  +11               +15
 * +------------+----------------+-------------------+-----------------+
 * | 0x04 fixed |compression type|compressed size(*2)|uncompressed size|
 * +------------+----------------+-------------------+-----------------+
 *  <-------------------------------(*1)-------------------------------*
 *
 * +15               +19          +20              +21        +23         +24
 * +-----------------+------------+----------------+----------+-----------+
 * |date/time(time_t)| 0x20 fixed |header level(=3)|file CRC16|  creator  |
 * +-----------------+------------+----------------+----------+-----------+
 * *--------------------------------(*1)----------------------------------*
 *
 * +24             +28              +32                 +32+(*3)
 * +---------------+----------------+-------------------+-----------------+
 * |header size(*1)|next header size|extended header(*3)| compressed data |
 * +---------------+----------------+-------------------+-----------------+
 * *------------------------(*1)-----------------------> <------(*2)----->
 *
 */
pub const H3_FIELD_LEN_OFFSET: libc::c_int = 0 as libc::c_int;
pub const H3_COMP_SIZE_OFFSET: libc::c_int = 7 as libc::c_int;
pub const H3_ORIG_SIZE_OFFSET: libc::c_int = 11 as libc::c_int;
pub const H3_TIME_OFFSET: libc::c_int = 15 as libc::c_int;
pub const H3_CRC_OFFSET: libc::c_int = 21 as libc::c_int;
pub const H3_HEADER_SIZE_OFFSET: libc::c_int = 24 as libc::c_int;
pub const H3_FIXED_SIZE: libc::c_int = 28 as libc::c_int;
unsafe extern "C" fn lha_read_file_header_3(
    mut a: *mut archive_read,
    mut lha: *mut lha,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut extdsize: size_t = 0;
    let mut err: libc::c_int = 0;
    let mut header_crc: uint16_t = 0;
    p = __archive_read_ahead(a, H3_FIXED_SIZE as size_t, NULL as *mut ssize_t)
        as *const libc::c_uchar;
    if p.is_null() {
        return truncated_error(a);
    }
    if !(archive_le16dec(p.offset(H3_FIELD_LEN_OFFSET as isize) as *const libc::c_void)
        as libc::c_int
        != 4 as libc::c_int)
    {
        (*lha).header_size =
            archive_le32dec(p.offset(H3_HEADER_SIZE_OFFSET as isize) as *const libc::c_void)
                as size_t;
        (*lha).compsize =
            archive_le32dec(p.offset(H3_COMP_SIZE_OFFSET as isize) as *const libc::c_void)
                as int64_t;
        (*lha).origsize =
            archive_le32dec(p.offset(H3_ORIG_SIZE_OFFSET as isize) as *const libc::c_void)
                as int64_t;
        (*lha).mtime =
            archive_le32dec(p.offset(H3_TIME_OFFSET as isize) as *const libc::c_void) as time_t;
        (*lha).crc = archive_le16dec(p.offset(H3_CRC_OFFSET as isize) as *const libc::c_void);
        (*lha).setflag |= CRC_IS_SET;
        if !((*lha).header_size < (H3_FIXED_SIZE + 4 as libc::c_int) as libc::c_ulong) {
            header_crc = lha_crc16(
                0 as libc::c_int as uint16_t,
                p as *const libc::c_void,
                H3_FIXED_SIZE as size_t,
            );
            __archive_read_consume(a, H3_FIXED_SIZE as int64_t);
            /* Read extended headers */
            err = lha_read_file_extended_header(
                a,
                lha,
                &mut header_crc,
                4 as libc::c_int,
                (*lha)
                    .header_size
                    .wrapping_sub(H3_FIXED_SIZE as libc::c_ulong),
                &mut extdsize,
            );
            if err < ARCHIVE_WARN {
                return err;
            }
            if header_crc as libc::c_int != (*lha).header_crc as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"LHa header CRC error\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            return err;
        }
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Invalid LHa header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
/*
 * Extended header format
 *
 * +0             +2        +3  -- used in header 1 and 2
 * +0             +4        +5  -- used in header 3
 * +--------------+---------+-------------------+--------------+--
 * |ex-header size|header id|        data       |ex-header size| .......
 * +--------------+---------+-------------------+--------------+--
 *  <-------------( ex-header size)------------> <-- next extended header --*
 *
 * If the ex-header size is zero, it is the make of the end of extended
 * headers.
 *
 */
unsafe extern "C" fn lha_read_file_extended_header(
    mut a: *mut archive_read,
    mut lha: *mut lha,
    mut crc: *mut uint16_t,
    mut sizefield_length: libc::c_int,
    mut limitsize: size_t,
    mut total_size: *mut size_t,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut extdheader: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut extdsize: size_t = 0;
    let mut datasize: size_t = 0;
    let mut i: libc::c_uint = 0;
    let mut extdtype: libc::c_uchar = 0;
    /* Header CRC and information*/
    /* Filename 		    */
    /* Directory name	    */
    /* MS-DOS attribute	    */
    /* Windows time stamp	    */
    /* Large file size	    */
    /* Time zone		    */
    /* UTF-16 filename 	    */
    /* UTF-16 directory name    */
    /* Codepage		    */
    /* File permission	    */
    /* gid,uid		    */
    /* Group name		    */
    /* User name		    */
    /* Modified time	    */
    /* new attribute(OS/2 only) */
    /* new attribute	    */
    *total_size = sizefield_length as size_t;
    loop {
        /* Read an extended header size. */
        h = __archive_read_ahead(a, sizefield_length as size_t, NULL as *mut ssize_t);
        if h == NULL as *const libc::c_void {
            return truncated_error(a);
        }
        /* Check if the size is the zero indicates the end of the
         * extended header. */
        if sizefield_length as libc::c_ulong == ::std::mem::size_of::<uint16_t>() as libc::c_ulong {
            extdsize = archive_le16dec(h) as size_t
        } else {
            extdsize = archive_le32dec(h) as size_t
        }
        if extdsize == 0 as libc::c_int as libc::c_ulong {
            /* End of extended header */
            if !crc.is_null() {
                *crc = lha_crc16(*crc, h, sizefield_length as size_t)
            }
            __archive_read_consume(a, sizefield_length as int64_t);
            return 0 as libc::c_int;
        }
        /* Sanity check to the extended header size. */
        if (*total_size).wrapping_add(extdsize) > limitsize
            || extdsize <= sizefield_length as size_t
        {
            break;
        }
        /* Read the extended header. */
        h = __archive_read_ahead(a, extdsize, NULL as *mut ssize_t);
        if h == NULL as *const libc::c_void {
            return truncated_error(a);
        }
        *total_size = (*total_size as libc::c_ulong).wrapping_add(extdsize) as size_t as size_t;
        extdheader = h as *const libc::c_uchar;
        /* Get the extended header type. */
        extdtype = *extdheader.offset(sizefield_length as isize);
        /* Calculate an extended data size. */
        datasize = extdsize.wrapping_sub((1 as libc::c_int + sizefield_length) as libc::c_ulong);
        /* Skip an extended header size field and type field. */
        extdheader = extdheader.offset((sizefield_length + 1 as libc::c_int) as isize);
        if !crc.is_null() && extdtype as libc::c_int != EXT_HEADER_CRC {
            *crc = lha_crc16(*crc, h, extdsize)
        }
        match extdtype as libc::c_int {
            EXT_HEADER_CRC => {
                /* We only use a header CRC. Following data will not
                 * be used. */
                if datasize >= 2 as libc::c_int as libc::c_ulong {
                    (*lha).header_crc = archive_le16dec(extdheader as *const libc::c_void);
                    if !crc.is_null() {
                        static mut zeros: [libc::c_char; 2] = [
                            0 as libc::c_int as libc::c_char,
                            0 as libc::c_int as libc::c_char,
                        ];
                        *crc = lha_crc16(*crc, h, extdsize.wrapping_sub(datasize));
                        /* CRC value itself as zero */
                        *crc = lha_crc16(
                            *crc,
                            zeros.as_ptr() as *const libc::c_void,
                            2 as libc::c_int as size_t,
                        );
                        *crc = lha_crc16(
                            *crc,
                            extdheader.offset(2 as libc::c_int as isize) as *const libc::c_void,
                            datasize.wrapping_sub(2 as libc::c_int as libc::c_ulong),
                        )
                    }
                }
            }
            EXT_FILENAME => {
                if datasize == 0 as libc::c_int as libc::c_ulong {
                    /* maybe directory header */
                    (*lha).filename.length = 0 as libc::c_int as size_t
                } else {
                    if *extdheader.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32
                    {
                        break;
                    }
                    (*lha).filename.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut (*lha).filename,
                        extdheader as *const libc::c_char as *const libc::c_void,
                        datasize,
                    );
                }
            }
            EXT_UTF16_FILENAME => {
                if datasize == 0 as libc::c_int as libc::c_ulong {
                    /* maybe directory header */
                    (*lha).filename.length = 0 as libc::c_int as size_t
                } else if datasize & 1 as libc::c_int as libc::c_ulong != 0 {
                    /* UTF-16 characters take always 2 or 4 bytes */
                    break;
                } else {
                    if *extdheader.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32
                    {
                        break;
                    }
                    (*lha).filename.length = 0 as libc::c_int as size_t;
                    archive_array_append(
                        &mut (*lha).filename,
                        extdheader as *const libc::c_char,
                        datasize,
                    );
                    /* Setup a string conversion for a filename. */
                    (*lha).sconv_fname = archive_string_conversion_from_charset(
                        &mut (*a).archive,
                        b"UTF-16LE\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    );
                    if (*lha).sconv_fname.is_null() {
                        return -(30 as libc::c_int);
                    }
                }
            }
            EXT_DIRECTORY => {
                if datasize == 0 as libc::c_int as libc::c_ulong
                    || *extdheader.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32
                {
                    /* no directory name data. exit this case. */
                    break;
                } else {
                    (*lha).dirname.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut (*lha).dirname,
                        extdheader as *const libc::c_char as *const libc::c_void,
                        datasize,
                    );
                    /*
                     * Convert directory delimiter from 0xFF
                     * to '/' for local system.
                     */
                    i = 0 as libc::c_int as libc::c_uint;
                    while (i as libc::c_ulong) < (*lha).dirname.length {
                        if *(*lha).dirname.s.offset(i as isize) as libc::c_uchar as libc::c_int
                            == 0xff as libc::c_int
                        {
                            *(*lha).dirname.s.offset(i as isize) = '/' as i32 as libc::c_char
                        }
                        i = i.wrapping_add(1)
                    }
                    /* Is last character directory separator? */
                    if *(*lha).dirname.s.offset(
                        (*lha)
                            .dirname
                            .length
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int
                        != '/' as i32
                    {
                        /* invalid directory data */
                        break;
                    }
                }
            }
            EXT_UTF16_DIRECTORY => {
                /* UTF-16 characters take always 2 or 4 bytes */
                if datasize == 0 as libc::c_int as libc::c_ulong
                    || datasize & 1 as libc::c_int as libc::c_ulong != 0
                    || *extdheader.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\u{0}' as i32
                {
                    /* no directory name data. exit this case. */
                    break;
                } else {
                    (*lha).dirname.length = 0 as libc::c_int as size_t;
                    archive_array_append(
                        &mut (*lha).dirname,
                        extdheader as *const libc::c_char,
                        datasize,
                    );
                    (*lha).sconv_dir = archive_string_conversion_from_charset(
                        &mut (*a).archive,
                        b"UTF-16LE\x00" as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    );
                    if (*lha).sconv_dir.is_null() {
                        return -(30 as libc::c_int);
                    } else {
                        /*
                         * Convert directory delimiter from 0xFFFF
                         * to '/' for local system.
                         */
                        let mut dirSep: uint16_t = 0;
                        let mut d: uint16_t = 1 as libc::c_int as uint16_t;
                        if archive_be16dec(&mut d as *mut uint16_t as *const libc::c_void)
                            as libc::c_int
                            == 1 as libc::c_int
                        {
                            dirSep = 0x2f00 as libc::c_int as uint16_t
                        } else {
                            dirSep = 0x2f as libc::c_int as uint16_t
                        }
                        /* UTF-16LE character */
                        let mut utf16name: *mut uint16_t = (*lha).dirname.s as *mut uint16_t;
                        i = 0 as libc::c_int as libc::c_uint;
                        while (i as libc::c_ulong)
                            < (*lha)
                                .dirname
                                .length
                                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                        {
                            if *utf16name.offset(i as isize) as libc::c_int == 0xffff as libc::c_int
                            {
                                *utf16name.offset(i as isize) = dirSep
                            }
                            i = i.wrapping_add(1)
                        }
                        /* Is last character directory separator? */
                        if *utf16name.offset(
                            (*lha)
                                .dirname
                                .length
                                .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int
                            != dirSep as libc::c_int
                        {
                            break;
                        }
                    }
                }
            }
            EXT_DOS_ATTR => {
                if datasize == 2 as libc::c_int as libc::c_ulong {
                    (*lha).dos_attr =
                        (archive_le16dec(extdheader as *const libc::c_void) as libc::c_int
                            & 0xff as libc::c_int) as libc::c_uchar
                }
            }
            EXT_TIMESTAMP => {
                if datasize
                    == (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                {
                    (*lha).birthtime = lha_win_time(
                        archive_le64dec(extdheader as *const libc::c_void),
                        &mut (*lha).birthtime_tv_nsec,
                    );
                    extdheader = extdheader
                        .offset(::std::mem::size_of::<uint64_t>() as libc::c_ulong as isize);
                    (*lha).mtime = lha_win_time(
                        archive_le64dec(extdheader as *const libc::c_void),
                        &mut (*lha).mtime_tv_nsec,
                    );
                    extdheader = extdheader
                        .offset(::std::mem::size_of::<uint64_t>() as libc::c_ulong as isize);
                    (*lha).atime = lha_win_time(
                        archive_le64dec(extdheader as *const libc::c_void),
                        &mut (*lha).atime_tv_nsec,
                    );
                    (*lha).setflag |= BIRTHTIME_IS_SET | ATIME_IS_SET
                }
            }
            EXT_FILESIZE => {
                if datasize
                    == (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                {
                    (*lha).compsize = archive_le64dec(extdheader as *const libc::c_void) as int64_t;
                    extdheader = extdheader
                        .offset(::std::mem::size_of::<uint64_t>() as libc::c_ulong as isize);
                    (*lha).origsize = archive_le64dec(extdheader as *const libc::c_void) as int64_t
                }
            }
            EXT_CODEPAGE => {
                /* Get an archived filename charset from codepage.
                 * This overwrites the charset specified by
                 * hdrcharset option. */
                if datasize == ::std::mem::size_of::<uint32_t>() as libc::c_ulong {
                    let mut cp: archive_string = archive_string {
                        s: 0 as *mut libc::c_char,
                        length: 0,
                        buffer_length: 0,
                    };
                    let mut charset: *const libc::c_char = 0 as *const libc::c_char;
                    cp.s = NULL as *mut libc::c_char;
                    cp.length = 0 as libc::c_int as size_t;
                    cp.buffer_length = 0 as libc::c_int as size_t;
                    match archive_le32dec(extdheader as *const libc::c_void) {
                        65001 => {
                            /* UTF-8 */
                            charset = b"UTF-8\x00" as *const u8 as *const libc::c_char
                        }
                        _ => {
                            archive_string_sprintf(
                                &mut cp as *mut archive_string,
                                b"CP%d\x00" as *const u8 as *const libc::c_char,
                                archive_le32dec(extdheader as *const libc::c_void) as libc::c_int,
                            );
                            charset = cp.s
                        }
                    }
                    (*lha).sconv_dir = archive_string_conversion_from_charset(
                        &mut (*a).archive,
                        charset,
                        1 as libc::c_int,
                    );
                    (*lha).sconv_fname = archive_string_conversion_from_charset(
                        &mut (*a).archive,
                        charset,
                        1 as libc::c_int,
                    );
                    archive_string_free(&mut cp);
                    if (*lha).sconv_dir.is_null() {
                        return -(30 as libc::c_int);
                    }
                    if (*lha).sconv_fname.is_null() {
                        return -(30 as libc::c_int);
                    }
                }
            }
            EXT_UNIX_MODE => {
                if datasize == ::std::mem::size_of::<uint16_t>() as libc::c_ulong {
                    (*lha).mode = archive_le16dec(extdheader as *const libc::c_void) as mode_t;
                    (*lha).setflag |= UNIX_MODE_IS_SET
                }
            }
            EXT_UNIX_GID_UID => {
                if datasize
                    == (::std::mem::size_of::<uint16_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                {
                    (*lha).gid = archive_le16dec(extdheader as *const libc::c_void) as int64_t;
                    (*lha).uid = archive_le16dec(
                        extdheader.offset(2 as libc::c_int as isize) as *const libc::c_void
                    ) as int64_t
                }
            }
            EXT_UNIX_GNAME => {
                if datasize > 0 as libc::c_int as libc::c_ulong {
                    (*lha).gname.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut (*lha).gname,
                        extdheader as *const libc::c_char as *const libc::c_void,
                        datasize,
                    );
                }
            }
            EXT_UNIX_UNAME => {
                if datasize > 0 as libc::c_int as libc::c_ulong {
                    (*lha).uname.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut (*lha).uname,
                        extdheader as *const libc::c_char as *const libc::c_void,
                        datasize,
                    );
                }
            }
            EXT_UNIX_MTIME => {
                if datasize == ::std::mem::size_of::<uint32_t>() as libc::c_ulong {
                    (*lha).mtime = archive_le32dec(extdheader as *const libc::c_void) as time_t
                }
            }
            EXT_OS2_NEW_ATTR => {
                /* This extended header is OS/2 depend. */
                if datasize == 16 as libc::c_int as libc::c_ulong {
                    (*lha).dos_attr =
                        (archive_le16dec(extdheader as *const libc::c_void) as libc::c_int
                            & 0xff as libc::c_int) as libc::c_uchar;
                    (*lha).mode = archive_le16dec(
                        extdheader.offset(2 as libc::c_int as isize) as *const libc::c_void
                    ) as mode_t;
                    (*lha).gid = archive_le16dec(
                        extdheader.offset(4 as libc::c_int as isize) as *const libc::c_void
                    ) as int64_t;
                    (*lha).uid = archive_le16dec(
                        extdheader.offset(6 as libc::c_int as isize) as *const libc::c_void
                    ) as int64_t;
                    (*lha).birthtime = archive_le32dec(
                        extdheader.offset(8 as libc::c_int as isize) as *const libc::c_void
                    ) as time_t;
                    (*lha).atime = archive_le32dec(
                        extdheader.offset(12 as libc::c_int as isize) as *const libc::c_void
                    ) as time_t;
                    (*lha).setflag |= UNIX_MODE_IS_SET | BIRTHTIME_IS_SET | ATIME_IS_SET
                }
            }
            EXT_NEW_ATTR => {
                if datasize == 20 as libc::c_int as libc::c_ulong {
                    (*lha).mode = archive_le32dec(extdheader as *const libc::c_void);
                    (*lha).gid = archive_le32dec(
                        extdheader.offset(4 as libc::c_int as isize) as *const libc::c_void
                    ) as int64_t;
                    (*lha).uid = archive_le32dec(
                        extdheader.offset(8 as libc::c_int as isize) as *const libc::c_void
                    ) as int64_t;
                    (*lha).birthtime = archive_le32dec(
                        extdheader.offset(12 as libc::c_int as isize) as *const libc::c_void,
                    ) as time_t;
                    (*lha).atime = archive_le32dec(
                        extdheader.offset(16 as libc::c_int as isize) as *const libc::c_void
                    ) as time_t;
                    (*lha).setflag |= UNIX_MODE_IS_SET | BIRTHTIME_IS_SET | ATIME_IS_SET
                }
            }
            EXT_TIMEZONE | _ => {}
        }
        /* Not supported */
        __archive_read_consume(a, extdsize as int64_t);
    }
    /* invalid directory data */
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Invalid extended LHa header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
pub const EXT_HEADER_CRC: libc::c_int = 0 as libc::c_int;
pub const EXT_FILENAME: libc::c_int = 0x1 as libc::c_int;
pub const EXT_DIRECTORY: libc::c_int = 0x2 as libc::c_int;
pub const EXT_DOS_ATTR: libc::c_int = 0x40 as libc::c_int;
pub const EXT_TIMESTAMP: libc::c_int = 0x41 as libc::c_int;
pub const EXT_FILESIZE: libc::c_int = 0x42 as libc::c_int;
pub const EXT_TIMEZONE: libc::c_int = 0x43 as libc::c_int;
pub const EXT_UTF16_FILENAME: libc::c_int = 0x44 as libc::c_int;
pub const EXT_UTF16_DIRECTORY: libc::c_int = 0x45 as libc::c_int;
pub const EXT_CODEPAGE: libc::c_int = 0x46 as libc::c_int;
pub const EXT_UNIX_MODE: libc::c_int = 0x50 as libc::c_int;
pub const EXT_UNIX_GID_UID: libc::c_int = 0x51 as libc::c_int;
pub const EXT_UNIX_GNAME: libc::c_int = 0x52 as libc::c_int;
pub const EXT_UNIX_UNAME: libc::c_int = 0x53 as libc::c_int;
pub const EXT_UNIX_MTIME: libc::c_int = 0x54 as libc::c_int;
pub const EXT_OS2_NEW_ATTR: libc::c_int = 0x7f as libc::c_int;
pub const EXT_NEW_ATTR: libc::c_int = 0xff as libc::c_int;
unsafe extern "C" fn lha_end_of_entry(mut a: *mut archive_read) -> libc::c_int {
    let mut lha: *mut lha = (*(*a).format).data as *mut lha;
    let mut r: libc::c_int = ARCHIVE_EOF;
    if (*lha).end_of_entry_cleanup == 0 {
        if (*lha).setflag & CRC_IS_SET != 0
            && (*lha).crc as libc::c_int != (*lha).entry_crc_calculated as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"LHa data CRC error\x00" as *const u8 as *const libc::c_char,
            );
            r = ARCHIVE_WARN
        }
        /* End-of-entry cleanup done. */
        (*lha).end_of_entry_cleanup = 1 as libc::c_int as libc::c_char
    }
    return r;
}
unsafe extern "C" fn archive_read_format_lha_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut lha: *mut lha = (*(*a).format).data as *mut lha;
    let mut r: libc::c_int = 0;
    if (*lha).entry_unconsumed != 0 {
        /* Consume as much as the decompressor actually used. */
        __archive_read_consume(a, (*lha).entry_unconsumed);
        (*lha).entry_unconsumed = 0 as libc::c_int as int64_t
    }
    if (*lha).end_of_entry != 0 {
        *offset = (*lha).entry_offset;
        *size = 0 as libc::c_int as size_t;
        *buff = NULL as *const libc::c_void;
        return lha_end_of_entry(a);
    }
    if (*lha).entry_is_compressed != 0 {
        r = lha_read_data_lzh(a, buff, size, offset)
    } else {
        /* No compression. */
        r = lha_read_data_none(a, buff, size, offset)
    }
    return r;
}
/*
 * Read a file content in no compression.
 *
 * Returns ARCHIVE_OK if successful, ARCHIVE_FATAL otherwise, sets
 * lha->end_of_entry if it consumes all of the data.
 */
unsafe extern "C" fn lha_read_data_none(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut lha: *mut lha = (*(*a).format).data as *mut lha;
    let mut bytes_avail: ssize_t = 0;
    if (*lha).entry_bytes_remaining == 0 as libc::c_int as libc::c_long {
        *buff = NULL as *const libc::c_void;
        *size = 0 as libc::c_int as size_t;
        *offset = (*lha).entry_offset;
        (*lha).end_of_entry = 1 as libc::c_int as libc::c_char;
        return 0 as libc::c_int;
    }
    /*
     * Note: '1' here is a performance optimization.
     * Recall that the decompression layer returns a count of
     * available bytes; asking for more than that forces the
     * decompressor to combine reads by copying data.
     */
    *buff = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail);
    if bytes_avail <= 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated LHa file data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if bytes_avail > (*lha).entry_bytes_remaining {
        bytes_avail = (*lha).entry_bytes_remaining
    }
    (*lha).entry_crc_calculated =
        lha_crc16((*lha).entry_crc_calculated, *buff, bytes_avail as size_t);
    *size = bytes_avail as size_t;
    *offset = (*lha).entry_offset;
    (*lha).entry_offset += bytes_avail;
    (*lha).entry_bytes_remaining -= bytes_avail;
    if (*lha).entry_bytes_remaining == 0 as libc::c_int as libc::c_long {
        (*lha).end_of_entry = 1 as libc::c_int as libc::c_char
    }
    (*lha).entry_unconsumed = bytes_avail;
    return 0 as libc::c_int;
}
/*
 * Read a file content in LZHUFF encoding.
 *
 * Returns ARCHIVE_OK if successful, returns ARCHIVE_WARN if compression is
 * unsupported, ARCHIVE_FATAL otherwise, sets lha->end_of_entry if it consumes
 * all of the data.
 */
unsafe extern "C" fn lha_read_data_lzh(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut lha: *mut lha = (*(*a).format).data as *mut lha;
    let mut bytes_avail: ssize_t = 0;
    let mut r: libc::c_int = 0;
    /* If we haven't yet read any data, initialize the decompressor. */
    if (*lha).decompress_init == 0 {
        r = lzh_decode_init(&mut (*lha).strm, (*lha).method.as_mut_ptr());
        match r {
            ARCHIVE_OK => {}
            -25 => {
                /* Unsupported compression. */
                *buff = NULL as *const libc::c_void;
                *size = 0 as libc::c_int as size_t;
                *offset = 0 as libc::c_int as int64_t;
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Unsupported lzh compression method -%c%c%c-\x00" as *const u8
                        as *const libc::c_char,
                    (*lha).method[0 as libc::c_int as usize] as libc::c_int,
                    (*lha).method[1 as libc::c_int as usize] as libc::c_int,
                    (*lha).method[2 as libc::c_int as usize] as libc::c_int,
                );
                /* We know compressed size; just skip it. */
                archive_read_format_lha_read_data_skip(a);
                return -(20 as libc::c_int);
            }
            _ => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Couldn\'t allocate memory for lzh decompression\x00" as *const u8
                        as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        /* We've initialized decompression for this stream. */
        (*lha).decompress_init = 1 as libc::c_int as libc::c_char;
        (*lha).strm.avail_out = 0 as libc::c_int;
        (*lha).strm.total_out = 0 as libc::c_int as int64_t
    }
    /*
     * Note: '1' here is a performance optimization.
     * Recall that the decompression layer returns a count of
     * available bytes; asking for more than that forces the
     * decompressor to combine reads by copying data.
     */
    (*lha).strm.next_in = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail)
        as *const libc::c_uchar;
    if bytes_avail <= 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated LHa file body\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if bytes_avail > (*lha).entry_bytes_remaining {
        bytes_avail = (*lha).entry_bytes_remaining
    }
    (*lha).strm.avail_in = bytes_avail as libc::c_int;
    (*lha).strm.total_in = 0 as libc::c_int as int64_t;
    (*lha).strm.avail_out = 0 as libc::c_int;
    r = lzh_decode(
        &mut (*lha).strm,
        (bytes_avail == (*lha).entry_bytes_remaining) as libc::c_int,
    );
    match r {
        ARCHIVE_OK => {}
        ARCHIVE_EOF => (*lha).end_of_entry = 1 as libc::c_int as libc::c_char,
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Bad lzh data\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
    }
    (*lha).entry_unconsumed = (*lha).strm.total_in;
    (*lha).entry_bytes_remaining -= (*lha).strm.total_in;
    if (*lha).strm.avail_out != 0 {
        *offset = (*lha).entry_offset;
        *size = (*lha).strm.avail_out as size_t;
        *buff = (*lha).strm.ref_ptr as *const libc::c_void;
        (*lha).entry_crc_calculated = lha_crc16((*lha).entry_crc_calculated, *buff, *size);
        (*lha).entry_offset =
            ((*lha).entry_offset as libc::c_ulong).wrapping_add(*size) as int64_t as int64_t
    } else {
        *offset = (*lha).entry_offset;
        *size = 0 as libc::c_int as size_t;
        *buff = NULL as *const libc::c_void;
        if (*lha).end_of_entry != 0 {
            return lha_end_of_entry(a);
        }
    }
    return 0 as libc::c_int;
}
/*
 * Skip a file content.
 */
unsafe extern "C" fn archive_read_format_lha_read_data_skip(
    mut a: *mut archive_read,
) -> libc::c_int {
    let mut lha: *mut lha = 0 as *mut lha;
    let mut bytes_skipped: int64_t = 0;
    lha = (*(*a).format).data as *mut lha;
    if (*lha).entry_unconsumed != 0 {
        /* Consume as much as the decompressor actually used. */
        __archive_read_consume(a, (*lha).entry_unconsumed);
        (*lha).entry_unconsumed = 0 as libc::c_int as int64_t
    }
    /* if we've already read to end of data, we're done. */
    if (*lha).end_of_entry_cleanup != 0 {
        return 0 as libc::c_int;
    }
    /*
     * If the length is at the beginning, we can skip the
     * compressed data much more quickly.
     */
    bytes_skipped = __archive_read_consume(a, (*lha).entry_bytes_remaining);
    if bytes_skipped < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    /* This entry is finished and done. */
    (*lha).end_of_entry = 1 as libc::c_int as libc::c_char;
    (*lha).end_of_entry_cleanup = (*lha).end_of_entry;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_lha_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut lha: *mut lha = (*(*a).format).data as *mut lha;
    lzh_decode_free(&mut (*lha).strm);
    archive_string_free(&mut (*lha).dirname);
    archive_string_free(&mut (*lha).filename);
    archive_string_free(&mut (*lha).uname);
    archive_string_free(&mut (*lha).gname);
    archive_wstring_free(&mut (*lha).ws);
    free(lha as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
 * 'LHa for UNIX' utility has archived a symbolic-link name after
 * a pathname with '|' character.
 * This function extracts the symbolic-link name from the pathname.
 *
 * example.
 *   1. a symbolic-name is 'aaa/bb/cc'
 *   2. a filename is 'xxx/bbb'
 *  then a archived pathname is 'xxx/bbb|aaa/bb/cc'
 */
unsafe extern "C" fn lha_parse_linkname(
    mut linkname: *mut archive_wstring,
    mut pathname: *mut archive_wstring,
) -> libc::c_int {
    let mut linkptr: *mut wchar_t = 0 as *mut wchar_t;
    let mut symlen: size_t = 0;
    linkptr = wcschr((*pathname).s, '|' as i32);
    if !linkptr.is_null() {
        symlen = wcslen(linkptr.offset(1 as libc::c_int as isize));
        (*linkname).length = 0 as libc::c_int as size_t;
        archive_wstrncat(linkname, linkptr.offset(1 as libc::c_int as isize), symlen);
        *linkptr = 0 as libc::c_int;
        (*pathname).length = wcslen((*pathname).s);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* Convert an MSDOS-style date/time into Unix-style time. */
unsafe extern "C" fn lha_dos_time(mut p: *const libc::c_uchar) -> time_t {
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
    msTime = archive_le16dec(p as *const libc::c_void) as libc::c_int;
    msDate =
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
/* Convert an MS-Windows-style date/time into Unix-style time. */
unsafe extern "C" fn lha_win_time(mut wintime: uint64_t, mut ns: *mut libc::c_long) -> time_t {
    if wintime as libc::c_ulonglong >= 116444736000000000 as libc::c_ulonglong {
        wintime = (wintime as libc::c_ulonglong)
            .wrapping_sub(116444736000000000 as libc::c_ulonglong) as uint64_t
            as uint64_t; /* 1970-01-01 00:00:00 (UTC) */
        if !ns.is_null() {
            *ns = wintime.wrapping_rem(10000000 as libc::c_int as libc::c_ulong) as libc::c_long
                * 100 as libc::c_int as libc::c_long
        }
        return wintime.wrapping_div(10000000 as libc::c_int as libc::c_ulong) as time_t;
    } else {
        if !ns.is_null() {
            *ns = 0 as libc::c_int as libc::c_long
        }
        return 0 as libc::c_int as time_t;
    };
}
unsafe extern "C" fn lha_calcsum(
    mut sum: libc::c_uchar,
    mut pp: *const libc::c_void,
    mut offset: libc::c_int,
    mut size: size_t,
) -> libc::c_uchar {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    p = p.offset(offset as isize);
    while size > 0 as libc::c_int as libc::c_ulong {
        let fresh0 = p;
        p = p.offset(1);
        sum = (sum as libc::c_int + *fresh0 as libc::c_int) as libc::c_uchar;
        size = size.wrapping_sub(1)
    }
    return sum;
}
static mut crc16tbl: [[uint16_t; 256]; 2] = [[0; 256]; 2];
unsafe extern "C" fn lha_crc16_init() {
    let mut i: libc::c_uint = 0;
    static mut crc16init: libc::c_int = 0 as libc::c_int;
    if crc16init != 0 {
        return;
    }
    crc16init = 1 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        let mut j: libc::c_uint = 0;
        let mut crc: uint16_t = i as uint16_t;
        j = 8 as libc::c_int as libc::c_uint;
        while j != 0 {
            crc = (crc as libc::c_int >> 1 as libc::c_int
                ^ (crc as libc::c_int & 1 as libc::c_int) * 0xa001 as libc::c_int)
                as uint16_t;
            j = j.wrapping_sub(1)
        }
        crc16tbl[0 as libc::c_int as usize][i as usize] = crc;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        crc16tbl[1 as libc::c_int as usize][i as usize] = (crc16tbl[0 as libc::c_int as usize]
            [i as usize] as libc::c_int
            >> 8 as libc::c_int
            ^ crc16tbl[0 as libc::c_int as usize][(crc16tbl[0 as libc::c_int as usize][i as usize]
                as libc::c_int
                & 0xff as libc::c_int) as usize] as libc::c_int)
            as uint16_t;
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn lha_crc16(
    mut crc: uint16_t,
    mut pp: *const libc::c_void,
    mut len: size_t,
) -> uint16_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    let mut buff: *const uint16_t = 0 as *const uint16_t;
    let u: C2RustUnnamed_0 = C2RustUnnamed_0 {
        i: 0x1020304 as libc::c_int as uint32_t,
    };
    if len == 0 as libc::c_int as libc::c_ulong {
        return crc;
    }
    /* Process unaligned address. */
    if p as uintptr_t & 0x1 as libc::c_int as uintptr_t != 0 {
        let fresh1 = p;
        p = p.offset(1);
        crc = (crc as libc::c_int >> 8 as libc::c_int
            ^ crc16tbl[0 as libc::c_int as usize]
                [((crc as libc::c_int ^ *fresh1 as libc::c_int) & 0xff as libc::c_int) as usize]
                as libc::c_int) as uint16_t;
        len = len.wrapping_sub(1)
    }
    buff = p as *const uint16_t;
    /*
     * Modern C compiler such as GCC does not unroll automatically yet
     * without unrolling pragma, and Clang is so. So we should
     * unroll this loop for its performance.
     */
    while len >= 8 as libc::c_int as libc::c_ulong {
        /* This if statement expects compiler optimization will
         * remove the statement which will not be executed. */
        /* Visual Studio */
        /* All clang versions have __builtin_bswap16() */
        /* Big endian */
        if u.c[0 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int {
            crc = (crc as libc::c_int ^ (*buff).swap_bytes() as libc::c_int) as uint16_t;
            buff = buff.offset(1)
        } else {
            let fresh2 = buff;
            buff = buff.offset(1);
            crc = (crc as libc::c_int ^ *fresh2 as libc::c_int) as uint16_t
        }
        crc = (crc16tbl[1 as libc::c_int as usize]
            [(crc as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
            ^ crc16tbl[0 as libc::c_int as usize][(crc as libc::c_int >> 8 as libc::c_int) as usize]
                as libc::c_int) as uint16_t;
        if u.c[0 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int {
            crc = (crc as libc::c_int ^ (*buff).swap_bytes() as libc::c_int) as uint16_t;
            buff = buff.offset(1)
        } else {
            let fresh3 = buff;
            buff = buff.offset(1);
            crc = (crc as libc::c_int ^ *fresh3 as libc::c_int) as uint16_t
        }
        crc = (crc16tbl[1 as libc::c_int as usize]
            [(crc as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
            ^ crc16tbl[0 as libc::c_int as usize][(crc as libc::c_int >> 8 as libc::c_int) as usize]
                as libc::c_int) as uint16_t;
        if u.c[0 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int {
            crc = (crc as libc::c_int ^ (*buff).swap_bytes() as libc::c_int) as uint16_t;
            buff = buff.offset(1)
        } else {
            let fresh4 = buff;
            buff = buff.offset(1);
            crc = (crc as libc::c_int ^ *fresh4 as libc::c_int) as uint16_t
        }
        crc = (crc16tbl[1 as libc::c_int as usize]
            [(crc as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
            ^ crc16tbl[0 as libc::c_int as usize][(crc as libc::c_int >> 8 as libc::c_int) as usize]
                as libc::c_int) as uint16_t;
        if u.c[0 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int {
            crc = (crc as libc::c_int ^ (*buff).swap_bytes() as libc::c_int) as uint16_t;
            buff = buff.offset(1)
        } else {
            let fresh5 = buff;
            buff = buff.offset(1);
            crc = (crc as libc::c_int ^ *fresh5 as libc::c_int) as uint16_t
        }
        crc = (crc16tbl[1 as libc::c_int as usize]
            [(crc as libc::c_int & 0xff as libc::c_int) as usize] as libc::c_int
            ^ crc16tbl[0 as libc::c_int as usize][(crc as libc::c_int >> 8 as libc::c_int) as usize]
                as libc::c_int) as uint16_t;
        len = (len as libc::c_ulong).wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    p = buff as *const libc::c_uchar;
    while len != 0 {
        let fresh6 = p;
        p = p.offset(1);
        crc = (crc as libc::c_int >> 8 as libc::c_int
            ^ crc16tbl[0 as libc::c_int as usize]
                [((crc as libc::c_int ^ *fresh6 as libc::c_int) & 0xff as libc::c_int) as usize]
                as libc::c_int) as uint16_t;
        len = len.wrapping_sub(1)
    }
    return crc;
}
/*
 * Initialize LZHUF decoder.
 *
 * Returns ARCHIVE_OK if initialization was successful.
 * Returns ARCHIVE_FAILED if method is unsupported.
 * Returns ARCHIVE_FATAL if initialization failed; memory allocation
 * error occurred.
 */
unsafe extern "C" fn lzh_decode_init(
    mut strm: *mut lzh_stream,
    mut method: *const libc::c_char,
) -> libc::c_int {
    let mut ds: *mut lzh_dec = 0 as *mut lzh_dec;
    let mut w_bits: libc::c_int = 0;
    let mut w_size: libc::c_int = 0;
    if (*strm).ds.is_null() {
        (*strm).ds = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<lzh_dec>() as libc::c_ulong,
        ) as *mut lzh_dec;
        if (*strm).ds.is_null() {
            return -(30 as libc::c_int);
        }
    }
    ds = (*strm).ds;
    (*ds).error = ARCHIVE_FAILED;
    if method.is_null()
        || *method.offset(0 as libc::c_int as isize) as libc::c_int != 'l' as i32
        || *method.offset(1 as libc::c_int as isize) as libc::c_int != 'h' as i32
    {
        return -(25 as libc::c_int);
    }
    match *method.offset(2 as libc::c_int as isize) as libc::c_int {
        53 => {
            w_bits = 13 as libc::c_int
            /* Not supported. */
        }
        54 => w_bits = 15 as libc::c_int,
        55 => {
            /* 32KiB for window */
            w_bits = 16 as libc::c_int
        }
        _ => return -(25 as libc::c_int),
    } /* 64KiB for window */
    (*ds).error = ARCHIVE_FATAL;
    /* Expand a window size up to 128 KiB for decompressing process
     * performance whatever its original window size is. */
    (*ds).w_size = ((1 as libc::c_uint) << 17 as libc::c_int) as libc::c_int;
    (*ds).w_mask = (*ds).w_size - 1 as libc::c_int;
    if (*ds).w_buff.is_null() {
        (*ds).w_buff = malloc((*ds).w_size as libc::c_ulong) as *mut libc::c_uchar;
        if (*ds).w_buff.is_null() {
            return -(30 as libc::c_int);
        }
    }
    w_size = ((1 as libc::c_uint) << w_bits) as libc::c_int;
    memset(
        (*ds)
            .w_buff
            .offset((*ds).w_size as isize)
            .offset(-(w_size as isize)) as *mut libc::c_void,
        0x20 as libc::c_int,
        w_size as libc::c_ulong,
    );
    (*ds).w_pos = 0 as libc::c_int;
    (*ds).state = 0 as libc::c_int;
    (*ds).pos_pt_len_size = w_bits + 1 as libc::c_int;
    (*ds).pos_pt_len_bits = if w_bits == 15 as libc::c_int || w_bits == 16 as libc::c_int {
        5 as libc::c_int
    } else {
        4 as libc::c_int
    };
    (*ds).literal_pt_len_size = PT_BITLEN_SIZE;
    (*ds).literal_pt_len_bits = 5 as libc::c_int;
    (*ds).br.cache_buffer = 0 as libc::c_int as uint64_t;
    (*ds).br.cache_avail = 0 as libc::c_int;
    if lzh_huffman_init(&mut (*ds).lt, LT_BITLEN_SIZE as size_t, 16 as libc::c_int) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*ds).lt.len_bits = 9 as libc::c_int;
    if lzh_huffman_init(&mut (*ds).pt, PT_BITLEN_SIZE as size_t, 16 as libc::c_int) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*ds).error = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/*
 * Release LZHUF decoder.
 */
unsafe extern "C" fn lzh_decode_free(mut strm: *mut lzh_stream) {
    if (*strm).ds.is_null() {
        return;
    }
    free((*(*strm).ds).w_buff as *mut libc::c_void);
    lzh_huffman_free(&mut (*(*strm).ds).lt);
    lzh_huffman_free(&mut (*(*strm).ds).pt);
    free((*strm).ds as *mut libc::c_void);
    (*strm).ds = NULL as *mut lzh_dec;
}
/* Notify how many bits we consumed. */
static mut cache_masks: [uint16_t; 20] = [
    0 as libc::c_int as uint16_t,
    0x1 as libc::c_int as uint16_t,
    0x3 as libc::c_int as uint16_t,
    0x7 as libc::c_int as uint16_t,
    0xf as libc::c_int as uint16_t,
    0x1f as libc::c_int as uint16_t,
    0x3f as libc::c_int as uint16_t,
    0x7f as libc::c_int as uint16_t,
    0xff as libc::c_int as uint16_t,
    0x1ff as libc::c_int as uint16_t,
    0x3ff as libc::c_int as uint16_t,
    0x7ff as libc::c_int as uint16_t,
    0xfff as libc::c_int as uint16_t,
    0x1fff as libc::c_int as uint16_t,
    0x3fff as libc::c_int as uint16_t,
    0x7fff as libc::c_int as uint16_t,
    0xffff as libc::c_int as uint16_t,
    0xffff as libc::c_int as uint16_t,
    0xffff as libc::c_int as uint16_t,
    0xffff as libc::c_int as uint16_t,
];
/*
 * Shift away used bits in the cache data and fill it up with following bits.
 * Call this when cache buffer does not have enough bits you need.
 *
 * Returns 1 if the cache buffer is full.
 * Returns 0 if the cache buffer is not full; input buffer is empty.
 */
unsafe extern "C" fn lzh_br_fillup(mut strm: *mut lzh_stream, mut br: *mut lzh_br) -> libc::c_int {
    let mut n: libc::c_int = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong)
        .wrapping_sub((*br).cache_avail as libc::c_ulong)
        as libc::c_int;
    loop {
        let x: libc::c_int = n >> 3 as libc::c_int;
        if (*strm).avail_in >= x {
            match x {
                8 => {
                    (*br).cache_buffer = (*(*strm).next_in.offset(0 as libc::c_int as isize)
                        as uint64_t)
                        << 56 as libc::c_int
                        | (*(*strm).next_in.offset(1 as libc::c_int as isize) as uint64_t)
                            << 48 as libc::c_int
                        | (*(*strm).next_in.offset(2 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*strm).next_in.offset(3 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*strm).next_in.offset(4 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(5 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(6 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*strm).next_in.offset(7 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*strm).next_in = (*strm).next_in.offset(8 as libc::c_int as isize);
                    (*strm).avail_in -= 8 as libc::c_int;
                    (*br).cache_avail += 8 as libc::c_int * 8 as libc::c_int;
                    return 1 as libc::c_int;
                }
                7 => {
                    (*br).cache_buffer = (*br).cache_buffer << 56 as libc::c_int
                        | (*(*strm).next_in.offset(0 as libc::c_int as isize) as uint64_t)
                            << 48 as libc::c_int
                        | (*(*strm).next_in.offset(1 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*strm).next_in.offset(2 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*strm).next_in.offset(3 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(4 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(5 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*strm).next_in.offset(6 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*strm).next_in = (*strm).next_in.offset(7 as libc::c_int as isize);
                    (*strm).avail_in -= 7 as libc::c_int;
                    (*br).cache_avail += 7 as libc::c_int * 8 as libc::c_int;
                    return 1 as libc::c_int;
                }
                6 => {
                    (*br).cache_buffer = (*br).cache_buffer << 48 as libc::c_int
                        | (*(*strm).next_in.offset(0 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*strm).next_in.offset(1 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*strm).next_in.offset(2 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(3 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*strm).next_in.offset(4 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*strm).next_in.offset(5 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*strm).next_in = (*strm).next_in.offset(6 as libc::c_int as isize);
                    (*strm).avail_in -= 6 as libc::c_int;
                    (*br).cache_avail += 6 as libc::c_int * 8 as libc::c_int;
                    return 1 as libc::c_int;
                }
                0 => {
                    /* We have enough compressed data in
                     * the cache buffer.*/
                    return 1 as libc::c_int;
                }
                _ => {}
            }
        }
        if (*strm).avail_in == 0 as libc::c_int {
            /* There is not enough compressed data to fill up the
             * cache buffer. */
            return 0 as libc::c_int;
        }
        let fresh7 = (*strm).next_in;
        (*strm).next_in = (*strm).next_in.offset(1);
        (*br).cache_buffer = (*br).cache_buffer << 8 as libc::c_int | *fresh7 as libc::c_ulong;
        (*strm).avail_in -= 1;
        (*br).cache_avail += 8 as libc::c_int;
        n -= 8 as libc::c_int
    }
}
pub const ST_RD_BLOCK: libc::c_int = 0 as libc::c_int;
pub const ST_RD_PT_1: libc::c_int = 1 as libc::c_int;
pub const ST_RD_PT_2: libc::c_int = 2 as libc::c_int;
pub const ST_RD_PT_3: libc::c_int = 3 as libc::c_int;
pub const ST_RD_PT_4: libc::c_int = 4 as libc::c_int;
pub const ST_RD_LITERAL_1: libc::c_int = 5 as libc::c_int;
pub const ST_RD_LITERAL_2: libc::c_int = 6 as libc::c_int;
pub const ST_RD_LITERAL_3: libc::c_int = 7 as libc::c_int;
pub const ST_RD_POS_DATA_1: libc::c_int = 8 as libc::c_int;
pub const ST_GET_LITERAL: libc::c_int = 9 as libc::c_int;
pub const ST_GET_POS_1: libc::c_int = 10 as libc::c_int;
pub const ST_GET_POS_2: libc::c_int = 11 as libc::c_int;
pub const ST_COPY_DATA: libc::c_int = 12 as libc::c_int;
unsafe extern "C" fn lzh_decode(mut strm: *mut lzh_stream, mut last: libc::c_int) -> libc::c_int {
    let mut ds: *mut lzh_dec = (*strm).ds;
    let mut avail_in: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if (*ds).error != 0 {
        return (*ds).error;
    }
    avail_in = (*strm).avail_in;
    loop {
        if (*ds).state < ST_GET_LITERAL {
            r = lzh_read_blocks(strm, last)
        } else {
            r = lzh_decode_blocks(strm, last)
        }
        if !(r == 100 as libc::c_int) {
            break;
        }
    }
    (*strm).total_in += (avail_in - (*strm).avail_in) as libc::c_long;
    return r;
}
unsafe extern "C" fn lzh_emit_window(mut strm: *mut lzh_stream, mut s: size_t) {
    (*strm).ref_ptr = (*(*strm).ds).w_buff;
    (*strm).avail_out = s as libc::c_int;
    (*strm).total_out = ((*strm).total_out as libc::c_ulong).wrapping_add(s) as int64_t as int64_t;
}
/*
 * Decode LZHUF.
 *
 * 1. Returns ARCHIVE_OK if output buffer or input buffer are empty.
 *    Please set available buffer and call this function again.
 * 2. Returns ARCHIVE_EOF if decompression has been completed.
 * 3. Returns ARCHIVE_FAILED if an error occurred; compressed data
 *    is broken or you do not set 'last' flag properly.
 * 4. 'last' flag is very important, you must set 1 to the flag if there
 *    is no input data. The lha compressed data format does not provide how
 *    to know the compressed data is really finished.
 *    Note: lha command utility check if the total size of output bytes is
 *    reached the uncompressed size recorded in its header. it does not mind
 *    that the decoding process is properly finished.
 *    GNU ZIP can decompress another compressed file made by SCO LZH compress.
 *    it handles EOF as null to fill read buffer with zero until the decoding
 *    process meet 2 bytes of zeros at reading a size of a next chunk, so the
 *    zeros are treated as the mark of the end of the data although the zeros
 *    is dummy, not the file data.
 */
unsafe extern "C" fn lzh_read_blocks(
    mut strm: *mut lzh_stream,
    mut last: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ds: *mut lzh_dec = (*strm).ds;
    let mut br: *mut lzh_br = &mut (*ds).br;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut rbits: libc::c_uint = 0;
    's_19: loop {
        match (*ds).state {
            ST_RD_BLOCK => {
                /*
                 * Read a block number indicates how many blocks
                 * we will handle. The block is composed of a
                 * literal and a match, sometimes a literal only
                 * in particular, there are no reference data at
                 * the beginning of the decompression.
                 */
                if !((*br).cache_avail >= 16 as libc::c_int || lzh_br_fillup(strm, br) != 0) {
                    if last == 0 {
                        /* We need following data. */
                        return 0 as libc::c_int;
                    }
                    if (*br).cache_avail >= 8 as libc::c_int {
                        /*
                         * It seems there are extra bits.
                         *  1. Compressed data is broken.
                         *  2. `last' flag does not properly
                         *     set.
                         */
                        break;
                    } else {
                        if (*ds).w_pos > 0 as libc::c_int {
                            lzh_emit_window(strm, (*ds).w_pos as size_t);
                            (*ds).w_pos = 0 as libc::c_int;
                            return 0 as libc::c_int;
                        }
                        /* End of compressed data; we have completely
                         * handled all compressed data. */
                        return 1 as libc::c_int;
                    }
                } else {
                    (*ds).blocks_avail = ((*br).cache_buffer
                        >> (*br).cache_avail - 16 as libc::c_int)
                        as uint16_t as libc::c_int
                        & cache_masks[16 as libc::c_int as usize] as libc::c_int;
                    if (*ds).blocks_avail == 0 as libc::c_int {
                        break;
                    }
                    (*br).cache_avail -= 16 as libc::c_int;
                    /*
                     * Read a literal table compressed in huffman
                     * coding.
                     */
                    (*ds).pt.len_size = (*ds).literal_pt_len_size;
                    (*ds).pt.len_bits = (*ds).literal_pt_len_bits;
                    (*ds).reading_position = 0 as libc::c_int
                }
                current_block = 8438980875191059710;
            }
            ST_RD_PT_1 => {
                current_block = 8438980875191059710;
            }
            ST_RD_PT_2 => {
                current_block = 15926396324098648065;
            }
            ST_RD_PT_3 => {
                current_block = 4260190332476513462;
            }
            ST_RD_PT_4 => {
                current_block = 10797866301349591617;
            }
            ST_RD_LITERAL_1 => {
                current_block = 7069697584811266517;
            }
            ST_RD_LITERAL_2 => {
                current_block = 5005401933950163426;
            }
            ST_RD_LITERAL_3 => {
                current_block = 3912448730491330959;
            }
            ST_RD_POS_DATA_1 => {
                current_block = 14724290549042048788;
            }
            ST_GET_LITERAL => return 100 as libc::c_int,
            _ => {
                continue;
            }
        }
        match current_block {
            8438980875191059710 =>
            /* Note: ST_RD_PT_1, ST_RD_PT_2 and ST_RD_PT_4 are
             * used in reading both a literal table and a
             * position table. */
            {
                if !((*br).cache_avail >= (*ds).pt.len_bits
                    || lzh_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= (*ds).pt.len_bits)
                {
                    if last != 0 {
                        break; /* Truncated data. */
                    }
                    (*ds).state = ST_RD_PT_1;
                    return 0 as libc::c_int;
                } else {
                    (*ds).pt.len_avail = ((*br).cache_buffer
                        >> (*br).cache_avail - (*ds).pt.len_bits)
                        as uint16_t as libc::c_int
                        & cache_masks[(*ds).pt.len_bits as usize] as libc::c_int;
                    (*br).cache_avail -= (*ds).pt.len_bits
                }
                current_block = 15926396324098648065;
            }
            _ => {}
        }
        match current_block {
            15926396324098648065 =>
            /* FALL THROUGH */
            {
                if (*ds).pt.len_avail == 0 as libc::c_int {
                    /* Invalid data. */
                    /* There is no bitlen. */
                    if !((*br).cache_avail >= (*ds).pt.len_bits
                        || lzh_br_fillup(strm, br) != 0
                        || (*br).cache_avail >= (*ds).pt.len_bits)
                    {
                        if last != 0 {
                            break; /* Truncated data.*/
                        } /* Invalid data. */
                        (*ds).state = ST_RD_PT_2;
                        return 0 as libc::c_int;
                    } else {
                        if lzh_make_fake_table(
                            &mut (*ds).pt,
                            (((*br).cache_buffer >> (*br).cache_avail - (*ds).pt.len_bits)
                                as uint16_t as libc::c_int
                                & cache_masks[(*ds).pt.len_bits as usize] as libc::c_int)
                                as uint16_t,
                        ) == 0
                        {
                            break;
                        }
                        (*br).cache_avail -= (*ds).pt.len_bits;
                        if (*ds).reading_position != 0 {
                            (*ds).state = ST_GET_LITERAL
                        } else {
                            (*ds).state = ST_RD_LITERAL_1
                        }
                        continue;
                    }
                } else {
                    if (*ds).pt.len_avail > (*ds).pt.len_size {
                        break;
                    }
                    (*ds).loop_0 = 0 as libc::c_int;
                    memset(
                        (*ds).pt.freq.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[libc::c_int; 17]>() as libc::c_ulong,
                    );
                    if (*ds).pt.len_avail < 3 as libc::c_int
                        || (*ds).pt.len_size == (*ds).pos_pt_len_size
                    {
                        (*ds).state = ST_RD_PT_4;
                        continue;
                    }
                }
                current_block = 4260190332476513462;
            }
            _ => {}
        }
        match current_block {
            4260190332476513462 =>
            /* FALL THROUGH */
            {
                (*ds).loop_0 = lzh_read_pt_bitlen(strm, (*ds).loop_0, 3 as libc::c_int); /* Invalid data. */
                if (*ds).loop_0 < 3 as libc::c_int {
                    if (*ds).loop_0 < 0 as libc::c_int || last != 0 {
                        break;
                    }
                    /* Not completed, get following data. */
                    (*ds).state = ST_RD_PT_3;
                    return 0 as libc::c_int;
                } else if !((*br).cache_avail >= 2 as libc::c_int
                    || lzh_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= 2 as libc::c_int)
                {
                    /* There are some null in bitlen of the literal. */
                    if last != 0 {
                        break; /* Truncated data. */
                    } /* Invalid data. */
                    (*ds).state = ST_RD_PT_3;
                    return 0 as libc::c_int;
                } else {
                    c = ((*br).cache_buffer >> (*br).cache_avail - 2 as libc::c_int) as uint16_t
                        as libc::c_int
                        & cache_masks[2 as libc::c_int as usize] as libc::c_int;
                    (*br).cache_avail -= 2 as libc::c_int;
                    if c > (*ds).pt.len_avail - 3 as libc::c_int {
                        break;
                    }
                    i = 3 as libc::c_int;
                    loop {
                        let fresh8 = c;
                        c = c - 1;
                        if !(fresh8 > 0 as libc::c_int) {
                            break;
                        }
                        let fresh9 = i;
                        i = i + 1;
                        *(*ds).pt.bitlen.offset(fresh9 as isize) = 0 as libc::c_int as libc::c_uchar
                    }
                    (*ds).loop_0 = i
                }
                current_block = 10797866301349591617;
            }
            _ => {}
        }
        match current_block {
            10797866301349591617 =>
            /* FALL THROUGH */
            {
                (*ds).loop_0 = lzh_read_pt_bitlen(strm, (*ds).loop_0, (*ds).pt.len_avail); /* Invalid data. */
                if (*ds).loop_0 < (*ds).pt.len_avail {
                    if (*ds).loop_0 < 0 as libc::c_int || last != 0 {
                        break;
                    }
                    /* Not completed, get following data. */
                    (*ds).state = ST_RD_PT_4; /* Invalid data */
                    return 0 as libc::c_int;
                } else {
                    if lzh_make_huffman_table(&mut (*ds).pt) == 0 {
                        break;
                    }
                    if (*ds).reading_position != 0 {
                        (*ds).state = ST_GET_LITERAL;
                        continue;
                    }
                }
                current_block = 7069697584811266517;
            }
            _ => {}
        }
        match current_block {
            7069697584811266517 =>
            /* FALL THROUGH */
            {
                if !((*br).cache_avail >= (*ds).lt.len_bits
                    || lzh_br_fillup(strm, br) != 0
                    || (*br).cache_avail >= (*ds).lt.len_bits)
                {
                    if last != 0 {
                        break; /* Truncated data. */
                    }
                    (*ds).state = ST_RD_LITERAL_1;
                    return 0 as libc::c_int;
                } else {
                    (*ds).lt.len_avail = ((*br).cache_buffer
                        >> (*br).cache_avail - (*ds).lt.len_bits)
                        as uint16_t as libc::c_int
                        & cache_masks[(*ds).lt.len_bits as usize] as libc::c_int;
                    (*br).cache_avail -= (*ds).lt.len_bits
                }
                current_block = 5005401933950163426;
            }
            _ => {}
        }
        match current_block {
            5005401933950163426 =>
            /* FALL THROUGH */
            {
                if (*ds).lt.len_avail == 0 as libc::c_int {
                    /* Invalid data */
                    /* There is no bitlen. */
                    if !((*br).cache_avail >= (*ds).lt.len_bits
                        || lzh_br_fillup(strm, br) != 0
                        || (*br).cache_avail >= (*ds).lt.len_bits)
                    {
                        if last != 0 {
                            break; /* Truncated data.*/
                        } /* Invalid data */
                        (*ds).state = ST_RD_LITERAL_2;
                        return 0 as libc::c_int;
                    } else {
                        if lzh_make_fake_table(
                            &mut (*ds).lt,
                            (((*br).cache_buffer >> (*br).cache_avail - (*ds).lt.len_bits)
                                as uint16_t as libc::c_int
                                & cache_masks[(*ds).lt.len_bits as usize] as libc::c_int)
                                as uint16_t,
                        ) == 0
                        {
                            break;
                        }
                        (*br).cache_avail -= (*ds).lt.len_bits;
                        (*ds).state = ST_RD_POS_DATA_1;
                        continue;
                    }
                } else {
                    if (*ds).lt.len_avail > (*ds).lt.len_size {
                        break;
                    }
                    (*ds).loop_0 = 0 as libc::c_int;
                    memset(
                        (*ds).lt.freq.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[libc::c_int; 17]>() as libc::c_ulong,
                    );
                }
                current_block = 3912448730491330959;
            }
            _ => {}
        }
        match current_block {
            3912448730491330959 =>
            /* FALL THROUGH */
            {
                i = (*ds).loop_0; /* Truncated data.*/
                while i < (*ds).lt.len_avail {
                    if !((*br).cache_avail >= (*ds).pt.max_bits
                        || lzh_br_fillup(strm, br) != 0
                        || (*br).cache_avail >= (*ds).pt.max_bits)
                    {
                        if last != 0 {
                            break 's_19;
                        }
                        (*ds).loop_0 = i;
                        (*ds).state = ST_RD_LITERAL_3;
                        return 0 as libc::c_int;
                    } else {
                        rbits = (((*br).cache_buffer >> (*br).cache_avail - (*ds).pt.max_bits)
                            as uint16_t as libc::c_int
                            & cache_masks[(*ds).pt.max_bits as usize] as libc::c_int)
                            as libc::c_uint;
                        c = lzh_decode_huffman(&mut (*ds).pt, rbits);
                        if c > 2 as libc::c_int {
                            /* Note: 'c' will never be more than
                             * eighteen since it's limited by
                             * PT_BITLEN_SIZE, which is being set
                             * to ds->pt.len_size through
                             * ds->literal_pt_len_size. */
                            (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                            c -= 2 as libc::c_int;
                            (*ds).lt.freq[c as usize] += 1;
                            let fresh10 = i;
                            i = i + 1;
                            *(*ds).lt.bitlen.offset(fresh10 as isize) = c as libc::c_uchar
                        } else if c == 0 as libc::c_int {
                            (*br).cache_avail -= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                            let fresh11 = i;
                            i = i + 1;
                            *(*ds).lt.bitlen.offset(fresh11 as isize) =
                                0 as libc::c_int as libc::c_uchar
                        } else {
                            /* c == 1 or c == 2 */
                            let mut n: libc::c_int = if c == 1 as libc::c_int {
                                4 as libc::c_int
                            } else {
                                9 as libc::c_int
                            }; /* Invalid data */
                            if !((*br).cache_avail
                                >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int + n
                                || lzh_br_fillup(strm, br) != 0
                                || (*br).cache_avail
                                    >= *(*ds).pt.bitlen.offset(c as isize) as libc::c_int + n)
                            {
                                if last != 0 {
                                    break 's_19; /* Invalid data */
                                }
                                (*ds).loop_0 = i;
                                (*ds).state = ST_RD_LITERAL_3;
                                return 0 as libc::c_int;
                            } else {
                                (*br).cache_avail -=
                                    *(*ds).pt.bitlen.offset(c as isize) as libc::c_int;
                                c = ((*br).cache_buffer >> (*br).cache_avail - n) as uint16_t
                                    as libc::c_int
                                    & cache_masks[n as usize] as libc::c_int;
                                (*br).cache_avail -= n;
                                c += if n == 4 as libc::c_int {
                                    3 as libc::c_int
                                } else {
                                    20 as libc::c_int
                                };
                                if i + c > (*ds).lt.len_avail {
                                    break 's_19;
                                }
                                memset(
                                    &mut *(*ds).lt.bitlen.offset(i as isize) as *mut libc::c_uchar
                                        as *mut libc::c_void,
                                    0 as libc::c_int,
                                    c as libc::c_ulong,
                                );
                                i += c
                            }
                        }
                    }
                }
                if i > (*ds).lt.len_avail || lzh_make_huffman_table(&mut (*ds).lt) == 0 {
                    break;
                }
            }
            _ => {}
        }
        /* FALL THROUGH */
        /*
         * Read a position table compressed in huffman
         * coding.
         */
        (*ds).pt.len_size = (*ds).pos_pt_len_size;
        (*ds).pt.len_bits = (*ds).pos_pt_len_bits;
        (*ds).reading_position = 1 as libc::c_int;
        (*ds).state = ST_RD_PT_1
    }
    /* Truncated data. */
    (*ds).error = ARCHIVE_FAILED;
    return (*ds).error;
}
unsafe extern "C" fn lzh_decode_blocks(
    mut strm: *mut lzh_stream,
    mut last: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ds: *mut lzh_dec = (*strm).ds;
    let mut bre: lzh_br = (*ds).br;
    let mut lt: *mut huffman = &mut (*ds).lt;
    let mut pt: *mut huffman = &mut (*ds).pt;
    let mut w_buff: *mut libc::c_uchar = (*ds).w_buff;
    let mut lt_bitlen: *mut libc::c_uchar = (*lt).bitlen;
    let mut pt_bitlen: *mut libc::c_uchar = (*pt).bitlen;
    let mut blocks_avail: libc::c_int = (*ds).blocks_avail;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut copy_len: libc::c_int = (*ds).copy_len;
    let mut copy_pos: libc::c_int = (*ds).copy_pos;
    let mut w_pos: libc::c_int = (*ds).w_pos;
    let mut w_mask: libc::c_int = (*ds).w_mask;
    let mut w_size: libc::c_int = (*ds).w_size;
    let mut lt_max_bits: libc::c_int = (*lt).max_bits;
    let mut pt_max_bits: libc::c_int = (*pt).max_bits;
    let mut state: libc::c_int = (*ds).state;
    's_43: loop {
        match state {
            ST_GET_LITERAL => {
                current_block = 2868539653012386629;
            }
            ST_GET_POS_1 => {
                current_block = 2353883182535819539;
            }
            ST_GET_POS_2 => {
                current_block = 5102923774869242951;
            }
            ST_COPY_DATA => {
                current_block = 7343950298149844727;
            }
            _ => {
                continue;
            }
        }
        loop {
            match current_block {
                2868539653012386629 => {
                    if blocks_avail == 0 as libc::c_int {
                        /* We have decoded all blocks.
                         * Let's handle next blocks. */
                        (*ds).state = ST_RD_BLOCK;
                        (*ds).br = bre;
                        (*ds).blocks_avail = 0 as libc::c_int;
                        (*ds).w_pos = w_pos;
                        (*ds).copy_pos = 0 as libc::c_int;
                        return 100 as libc::c_int;
                    }
                    /* lzh_br_read_ahead() always try to fill the
                     * cache buffer up. In specific situation we
                     * are close to the end of the data, the cache
                     * buffer will not be full and thus we have to
                     * determine if the cache buffer has some bits
                     * as much as we need after lzh_br_read_ahead()
                     * failed. */
                    if !(bre.cache_avail >= lt_max_bits
                        || lzh_br_fillup(strm, &mut bre) != 0
                        || bre.cache_avail >= lt_max_bits)
                    {
                        if last == 0 {
                            current_block = 6272238089492121364;
                            break 's_43;
                            /* Over read. */
                        }
                        /* Remaining bits are less than
                         * maximum bits(lt.max_bits) but maybe
                         * it still remains as much as we need,
                         * so we should try to use it with
                         * dummy bits. */
                        c = lzh_decode_huffman(
                            lt,
                            ((bre.cache_buffer << lt_max_bits - bre.cache_avail) as uint16_t
                                as libc::c_int
                                & cache_masks[lt_max_bits as usize] as libc::c_int)
                                as libc::c_uint,
                        );
                        bre.cache_avail -= *lt_bitlen.offset(c as isize) as libc::c_int;
                        if !(bre.cache_avail >= 0 as libc::c_int) {
                            current_block = 5081962102481586684;
                            break 's_43;
                        }
                    } else {
                        c = lzh_decode_huffman(
                            lt,
                            ((bre.cache_buffer >> bre.cache_avail - lt_max_bits) as uint16_t
                                as libc::c_int
                                & cache_masks[lt_max_bits as usize] as libc::c_int)
                                as libc::c_uint,
                        );
                        bre.cache_avail -= *lt_bitlen.offset(c as isize) as libc::c_int
                    }
                    blocks_avail -= 1;
                    if c > UCHAR_MAX {
                        /* Current block is a match data. */
                        /* 'c' is the length of a match pattern we have
                         * already extracted, which has be stored in
                         * window(ds->w_buff). */
                        copy_len = c - (UCHAR_MAX + 1 as libc::c_int) + MINMATCH;
                        /* FALL THROUGH */
                        current_block = 2353883182535819539;
                    } else {
                        /*
                         * 'c' is exactly a literal code.
                         */
                        /* Save a decoded code to reference it
                         * afterward. */
                        *w_buff.offset(w_pos as isize) = c as libc::c_uchar;
                        w_pos += 1;
                        if !(w_pos >= w_size) {
                            current_block = 2868539653012386629;
                            continue;
                        }
                        w_pos = 0 as libc::c_int;
                        lzh_emit_window(strm, w_size as size_t);
                        current_block = 6272238089492121364;
                        break 's_43;
                    }
                }
                5102923774869242951 =>
                /* FALL THROUGH */
                {
                    if copy_pos > 1 as libc::c_int {
                        /* We need an additional adjustment number to
                         * the position. */
                        let mut p: libc::c_int = copy_pos - 1 as libc::c_int; /* Truncated data.*/
                        if !(bre.cache_avail >= p
                            || lzh_br_fillup(strm, &mut bre) != 0
                            || bre.cache_avail >= p)
                        {
                            if last != 0 {
                                current_block = 5081962102481586684;
                                break 's_43;
                            }
                            state = ST_GET_POS_2;
                            (*ds).copy_len = copy_len;
                            (*ds).copy_pos = copy_pos;
                            current_block = 6272238089492121364;
                            break 's_43;
                        } else {
                            copy_pos = ((1 as libc::c_int) << p)
                                + ((bre.cache_buffer >> bre.cache_avail - p) as uint16_t
                                    as libc::c_int
                                    & cache_masks[p as usize] as libc::c_int);
                            bre.cache_avail -= p
                        }
                    }
                    /* The position is actually a distance from the last
                     * code we had extracted and thus we have to convert
                     * it to a position of the window. */
                    copy_pos = w_pos - copy_pos - 1 as libc::c_int & w_mask;
                    /* FALL THROUGH */
                    current_block = 7343950298149844727;
                }
                2353883182535819539 =>
                /*
                 * Get a reference position.
                 */
                {
                    if !(bre.cache_avail >= pt_max_bits
                        || lzh_br_fillup(strm, &mut bre) != 0
                        || bre.cache_avail >= pt_max_bits)
                    {
                        if last == 0 {
                            state = ST_GET_POS_1;
                            (*ds).copy_len = copy_len;
                            current_block = 6272238089492121364;
                            break 's_43;
                        } else {
                            copy_pos = lzh_decode_huffman(
                                pt,
                                ((bre.cache_buffer << pt_max_bits - bre.cache_avail) as uint16_t
                                    as libc::c_int
                                    & cache_masks[pt_max_bits as usize] as libc::c_int)
                                    as libc::c_uint,
                            );
                            bre.cache_avail -= *pt_bitlen.offset(copy_pos as isize) as libc::c_int;
                            if !(bre.cache_avail >= 0 as libc::c_int) {
                                current_block = 5081962102481586684;
                                break 's_43;
                            } else {
                                current_block = 5102923774869242951;
                            }
                        }
                    /* Over read. */
                    } else {
                        copy_pos = lzh_decode_huffman(
                            pt,
                            ((bre.cache_buffer >> bre.cache_avail - pt_max_bits) as uint16_t
                                as libc::c_int
                                & cache_masks[pt_max_bits as usize] as libc::c_int)
                                as libc::c_uint,
                        );
                        bre.cache_avail -= *pt_bitlen.offset(copy_pos as isize) as libc::c_int;
                        current_block = 5102923774869242951;
                    }
                }
                _ =>
                /*
                 * Copy `copy_len' bytes as extracted data from
                 * the window into the output buffer.
                 */
                {
                    let mut l: libc::c_int = 0;
                    l = copy_len;
                    if copy_pos > w_pos {
                        if l > w_size - copy_pos {
                            l = w_size - copy_pos
                        }
                    } else if l > w_size - w_pos {
                        l = w_size - w_pos
                    }
                    if copy_pos + l < w_pos || w_pos + l < copy_pos {
                        /* No overlap. */
                        memcpy(
                            w_buff.offset(w_pos as isize) as *mut libc::c_void,
                            w_buff.offset(copy_pos as isize) as *const libc::c_void,
                            l as libc::c_ulong,
                        );
                    } else {
                        let mut s: *const libc::c_uchar = 0 as *const libc::c_uchar;
                        let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                        let mut li: libc::c_int = 0;
                        d = w_buff.offset(w_pos as isize);
                        s = w_buff.offset(copy_pos as isize);
                        li = 0 as libc::c_int;
                        while li < l - 1 as libc::c_int {
                            *d.offset(li as isize) = *s.offset(li as isize);
                            li += 1;
                            *d.offset(li as isize) = *s.offset(li as isize);
                            li += 1
                        }
                        if li < l {
                            *d.offset(li as isize) = *s.offset(li as isize)
                        }
                    }
                    w_pos += l;
                    if w_pos == w_size {
                        w_pos = 0 as libc::c_int;
                        lzh_emit_window(strm, w_size as size_t);
                        if copy_len <= l {
                            state = ST_GET_LITERAL
                        } else {
                            state = ST_COPY_DATA;
                            (*ds).copy_len = copy_len - l;
                            (*ds).copy_pos = copy_pos + l & w_mask
                        }
                        current_block = 6272238089492121364;
                        break 's_43;
                    } else if copy_len <= l {
                        /* A copy of current pattern ended. */
                        state = ST_GET_LITERAL;
                        break;
                    } else {
                        copy_len -= l;
                        copy_pos = copy_pos + l & w_mask;
                        current_block = 7343950298149844727;
                    }
                }
            }
        }
    }
    match current_block {
        5081962102481586684 => {
            (*ds).error = ARCHIVE_FAILED;
            return (*ds).error;
        }
        _ => {
            (*ds).br = bre;
            (*ds).blocks_avail = blocks_avail;
            (*ds).state = state;
            (*ds).w_pos = w_pos;
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn lzh_huffman_init(
    mut hf: *mut huffman,
    mut len_size: size_t,
    mut tbl_bits: libc::c_int,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    if (*hf).bitlen.is_null() {
        (*hf).bitlen =
            malloc(len_size.wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong))
                as *mut libc::c_uchar;
        if (*hf).bitlen.is_null() {
            return -(30 as libc::c_int);
        }
    }
    if (*hf).tbl.is_null() {
        if tbl_bits < HTBL_BITS {
            bits = tbl_bits
        } else {
            bits = HTBL_BITS
        }
        (*hf).tbl = malloc(
            ((1 as libc::c_int as size_t) << bits)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        ) as *mut uint16_t;
        if (*hf).tbl.is_null() {
            return -(30 as libc::c_int);
        }
    }
    if (*hf).tree.is_null() && tbl_bits > HTBL_BITS {
        (*hf).tree_avail = (1 as libc::c_int) << tbl_bits - HTBL_BITS + 4 as libc::c_int;
        (*hf).tree = malloc(
            ((*hf).tree_avail as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<htree_t>() as libc::c_ulong),
        ) as *mut htree_t;
        if (*hf).tree.is_null() {
            return -(30 as libc::c_int);
        }
    }
    (*hf).len_size = len_size as libc::c_int;
    (*hf).tbl_bits = tbl_bits;
    return 0 as libc::c_int;
}
unsafe extern "C" fn lzh_huffman_free(mut hf: *mut huffman) {
    free((*hf).bitlen as *mut libc::c_void);
    free((*hf).tbl as *mut libc::c_void);
    free((*hf).tree as *mut libc::c_void);
}
static mut bitlen_tbl: [libc::c_char; 1024] = [
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    16 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn lzh_read_pt_bitlen(
    mut strm: *mut lzh_stream,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> libc::c_int {
    let mut ds: *mut lzh_dec = (*strm).ds;
    let mut br: *mut lzh_br = &mut (*ds).br;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = start;
    while i < end {
        /*
         *  bit pattern     the number we need
         *     000           ->  0
         *     001           ->  1
         *     010           ->  2
         *     ...
         *     110           ->  6
         *     1110          ->  7
         *     11110         ->  8
         *     ...
         *     1111111111110 ->  16
         */
        if !((*br).cache_avail >= 3 as libc::c_int
            || lzh_br_fillup(strm, br) != 0
            || (*br).cache_avail >= 3 as libc::c_int)
        {
            return i;
        }
        c = ((*br).cache_buffer >> (*br).cache_avail - 3 as libc::c_int) as uint16_t as libc::c_int
            & cache_masks[3 as libc::c_int as usize] as libc::c_int;
        if c == 7 as libc::c_int {
            if !((*br).cache_avail >= 13 as libc::c_int
                || lzh_br_fillup(strm, br) != 0
                || (*br).cache_avail >= 13 as libc::c_int)
            {
                return i;
            }
            c = bitlen_tbl[(((*br).cache_buffer >> (*br).cache_avail - 13 as libc::c_int)
                as uint16_t as libc::c_int
                & cache_masks[13 as libc::c_int as usize] as libc::c_int
                & 0x3ff as libc::c_int) as usize] as libc::c_int;
            if c != 0 {
                (*br).cache_avail -= c - 3 as libc::c_int
            } else {
                return -(1 as libc::c_int);
            }
        /* Invalid data. */
        } else {
            (*br).cache_avail -= 3 as libc::c_int
        }
        let fresh12 = i;
        i = i + 1;
        *(*ds).pt.bitlen.offset(fresh12 as isize) = c as libc::c_uchar;
        (*ds).pt.freq[c as usize] += 1
    }
    return i;
}
unsafe extern "C" fn lzh_make_fake_table(mut hf: *mut huffman, mut c: uint16_t) -> libc::c_int {
    if c as libc::c_int >= (*hf).len_size {
        return 0 as libc::c_int;
    }
    *(*hf).tbl.offset(0 as libc::c_int as isize) = c;
    (*hf).max_bits = 0 as libc::c_int;
    (*hf).shift_bits = 0 as libc::c_int;
    *(*hf)
        .bitlen
        .offset(*(*hf).tbl.offset(0 as libc::c_int as isize) as isize) =
        0 as libc::c_int as libc::c_uchar;
    return 1 as libc::c_int;
}
/*
 * Make a huffman coding table.
 */
unsafe extern "C" fn lzh_make_huffman_table(mut hf: *mut huffman) -> libc::c_int {
    let mut tbl: *mut uint16_t = 0 as *mut uint16_t;
    let mut bitlen: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut bitptn: [libc::c_int; 17] = [0; 17];
    let mut weight: [libc::c_int; 17] = [0; 17];
    let mut i: libc::c_int = 0;
    let mut maxbits: libc::c_int = 0 as libc::c_int;
    let mut ptn: libc::c_int = 0;
    let mut tbl_size: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut diffbits: libc::c_int = 0;
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
    if ptn != 0x10000 as libc::c_int || maxbits > (*hf).tbl_bits {
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
    if maxbits > HTBL_BITS {
        let mut htbl_max: libc::c_uint = 0;
        let mut p: *mut uint16_t = 0 as *mut uint16_t;
        diffbits = maxbits - HTBL_BITS;
        i = 1 as libc::c_int;
        while i <= HTBL_BITS {
            bitptn[i as usize] >>= diffbits;
            weight[i as usize] >>= diffbits;
            i += 1
        }
        htbl_max = (bitptn[HTBL_BITS as usize]
            + weight[HTBL_BITS as usize] * (*hf).freq[HTBL_BITS as usize])
            as libc::c_uint;
        p = &mut *(*hf).tbl.offset(htbl_max as isize) as *mut uint16_t;
        while p < &mut *(*hf)
            .tbl
            .offset(((1 as libc::c_uint) << HTBL_BITS) as isize) as *mut uint16_t
        {
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = 0 as libc::c_int as uint16_t
        }
    } else {
        diffbits = 0 as libc::c_int
    }
    (*hf).shift_bits = diffbits;
    /*
     * Make the table.
     */
    tbl_size = (1 as libc::c_int) << HTBL_BITS;
    tbl = (*hf).tbl;
    bitlen = (*hf).bitlen;
    len_avail = (*hf).len_avail;
    (*hf).tree_used = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len_avail {
        let mut p_0: *mut uint16_t = 0 as *mut uint16_t;
        let mut len: libc::c_int = 0;
        let mut cnt: libc::c_int = 0;
        let mut bit: uint16_t = 0;
        let mut extlen: libc::c_int = 0;
        let mut ht: *mut htree_t = 0 as *mut htree_t;
        if !(*bitlen.offset(i as isize) as libc::c_int == 0 as libc::c_int) {
            /* Get a bit pattern */
            len = *bitlen.offset(i as isize) as libc::c_int;
            ptn = bitptn[len as usize];
            cnt = weight[len as usize];
            if len <= HTBL_BITS {
                /* Calculate next bit pattern */
                bitptn[len as usize] = ptn + cnt; /* Invalid */
                if bitptn[len as usize] > tbl_size {
                    return 0 as libc::c_int;
                }
                /* Update the table */
                p_0 = &mut *tbl.offset(ptn as isize) as *mut uint16_t;
                if cnt > 7 as libc::c_int {
                    let mut pc: *mut uint16_t = 0 as *mut uint16_t;
                    cnt -= 8 as libc::c_int;
                    pc = &mut *p_0.offset(cnt as isize) as *mut uint16_t;
                    *pc.offset(0 as libc::c_int as isize) = i as uint16_t;
                    *pc.offset(1 as libc::c_int as isize) = i as uint16_t;
                    *pc.offset(2 as libc::c_int as isize) = i as uint16_t;
                    *pc.offset(3 as libc::c_int as isize) = i as uint16_t;
                    *pc.offset(4 as libc::c_int as isize) = i as uint16_t;
                    *pc.offset(5 as libc::c_int as isize) = i as uint16_t;
                    *pc.offset(6 as libc::c_int as isize) = i as uint16_t;
                    *pc.offset(7 as libc::c_int as isize) = i as uint16_t;
                    if cnt > 7 as libc::c_int {
                        cnt -= 8 as libc::c_int;
                        memcpy(
                            &mut *p_0.offset(cnt as isize) as *mut uint16_t as *mut libc::c_void,
                            pc as *const libc::c_void,
                            (8 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
                        );
                        pc = &mut *p_0.offset(cnt as isize) as *mut uint16_t;
                        while cnt > 15 as libc::c_int {
                            cnt -= 16 as libc::c_int;
                            memcpy(
                                &mut *p_0.offset(cnt as isize) as *mut uint16_t
                                    as *mut libc::c_void,
                                pc as *const libc::c_void,
                                (16 as libc::c_int as libc::c_ulong).wrapping_mul(
                                    ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                                ),
                            );
                        }
                    }
                    if cnt != 0 {
                        memcpy(
                            p_0 as *mut libc::c_void,
                            pc as *const libc::c_void,
                            (cnt as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
                        );
                    }
                } else {
                    while cnt > 1 as libc::c_int {
                        cnt -= 1;
                        *p_0.offset(cnt as isize) = i as uint16_t;
                        cnt -= 1;
                        *p_0.offset(cnt as isize) = i as uint16_t
                    }
                    if cnt != 0 {
                        cnt -= 1;
                        *p_0.offset(cnt as isize) = i as uint16_t
                    }
                }
            } else {
                /*
                 * A bit length is too big to be housed to a direct table,
                 * so we use a tree model for its extra bits.
                 */
                bitptn[len as usize] = ptn + cnt; /* Invalid */
                bit = ((1 as libc::c_uint) << diffbits - 1 as libc::c_int) as uint16_t; /* Invalid */
                extlen = len - HTBL_BITS; /* Invalid */
                p_0 = &mut *tbl.offset((ptn >> diffbits) as isize) as *mut uint16_t; /* Invalid */
                if *p_0 as libc::c_int == 0 as libc::c_int {
                    *p_0 = (len_avail + (*hf).tree_used) as uint16_t; /* Invalid */
                    let fresh14 = (*hf).tree_used; /* Invalid */
                    (*hf).tree_used = (*hf).tree_used + 1;
                    ht = &mut *(*hf).tree.offset(fresh14 as isize) as *mut htree_t;
                    if (*hf).tree_used > (*hf).tree_avail {
                        return 0 as libc::c_int;
                    }
                    (*ht).left = 0 as libc::c_int as uint16_t;
                    (*ht).right = 0 as libc::c_int as uint16_t
                } else {
                    if (*p_0 as libc::c_int) < len_avail
                        || *p_0 as libc::c_int >= len_avail + (*hf).tree_used
                    {
                        return 0 as libc::c_int;
                    }
                    ht = &mut *(*hf)
                        .tree
                        .offset((*p_0 as libc::c_int - len_avail) as isize)
                        as *mut htree_t
                }
                loop {
                    extlen -= 1;
                    if !(extlen > 0 as libc::c_int) {
                        break;
                    }
                    if ptn & bit as libc::c_int != 0 {
                        if ((*ht).left as libc::c_int) < len_avail {
                            (*ht).left = (len_avail + (*hf).tree_used) as uint16_t;
                            let fresh15 = (*hf).tree_used;
                            (*hf).tree_used = (*hf).tree_used + 1;
                            ht = &mut *(*hf).tree.offset(fresh15 as isize) as *mut htree_t;
                            if (*hf).tree_used > (*hf).tree_avail {
                                return 0 as libc::c_int;
                            }
                            (*ht).left = 0 as libc::c_int as uint16_t;
                            (*ht).right = 0 as libc::c_int as uint16_t
                        } else {
                            ht = &mut *(*hf)
                                .tree
                                .offset(((*ht).left as libc::c_int - len_avail) as isize)
                                as *mut htree_t
                        }
                    } else if ((*ht).right as libc::c_int) < len_avail {
                        (*ht).right = (len_avail + (*hf).tree_used) as uint16_t;
                        let fresh16 = (*hf).tree_used;
                        (*hf).tree_used = (*hf).tree_used + 1;
                        ht = &mut *(*hf).tree.offset(fresh16 as isize) as *mut htree_t;
                        if (*hf).tree_used > (*hf).tree_avail {
                            return 0 as libc::c_int;
                        }
                        (*ht).left = 0 as libc::c_int as uint16_t;
                        (*ht).right = 0 as libc::c_int as uint16_t
                    } else {
                        ht = &mut *(*hf)
                            .tree
                            .offset(((*ht).right as libc::c_int - len_avail) as isize)
                            as *mut htree_t
                    }
                    bit = (bit as libc::c_int >> 1 as libc::c_int) as uint16_t
                }
                if ptn & bit as libc::c_int != 0 {
                    if (*ht).left as libc::c_int != 0 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    (*ht).left = i as uint16_t
                } else {
                    if (*ht).right as libc::c_int != 0 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                    (*ht).right = i as uint16_t
                }
            }
        }
        i += 1
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn lzh_decode_huffman_tree(
    mut hf: *mut huffman,
    mut rbits: libc::c_uint,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut ht: *mut htree_t = 0 as *mut htree_t;
    let mut extlen: libc::c_int = 0;
    ht = (*hf).tree;
    extlen = (*hf).shift_bits;
    while c >= (*hf).len_avail {
        c -= (*hf).len_avail;
        let fresh17 = extlen;
        extlen = extlen - 1;
        if fresh17 <= 0 as libc::c_int || c >= (*hf).tree_used {
            return 0 as libc::c_int;
        }
        if rbits & (1 as libc::c_uint) << extlen != 0 {
            c = (*ht.offset(c as isize)).left as libc::c_int
        } else {
            c = (*ht.offset(c as isize)).right as libc::c_int
        }
    }
    return c;
}
#[inline]
unsafe extern "C" fn lzh_decode_huffman(
    mut hf: *mut huffman,
    mut rbits: libc::c_uint,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    /*
     * At first search an index table for a bit pattern.
     * If it fails, search a huffman tree for.
     */
    c = *(*hf).tbl.offset((rbits >> (*hf).shift_bits) as isize) as libc::c_int;
    if c < (*hf).len_avail || (*hf).len_avail == 0 as libc::c_int {
        return c;
    }
    /* This bit pattern needs to be found out at a huffman tree. */
    return lzh_decode_huffman_tree(hf, rbits, c);
}
