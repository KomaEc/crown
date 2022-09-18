use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn timegm(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_filetype(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_perm(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
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
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
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
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type time_t = __time_t;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub struct warc_s {
    pub cntlen: size_t,
    pub cntoff: size_t,
    pub unconsumed: size_t,
    pub pool: warc_strbuf_t,
    pub pver: libc::c_uint,
    pub sver: archive_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warc_strbuf_t {
    pub len: size_t,
    pub str_0: *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warc_string_t {
    pub len: size_t,
    pub str_0: *const libc::c_char,
}
pub const WT_RSP: warc_type_t = 5;
pub const WT_RSRC: warc_type_t = 3;
pub type warc_type_t = libc::c_uint;
pub const LAST_WT: warc_type_t = 9;
pub const WT_CONT: warc_type_t = 8;
pub const WT_CONV: warc_type_t = 7;
pub const WT_RVIS: warc_type_t = 6;
pub const WT_REQ: warc_type_t = 4;
pub const WT_META: warc_type_t = 2;
pub const WT_INFO: warc_type_t = 1;
pub const WT_NONE: warc_type_t = 0;
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_WARC: libc::c_int = 0xf0000 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
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
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_warc(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut w: *mut warc_s = 0 as *mut warc_s;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_warc\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<warc_s>() as libc::c_ulong,
    ) as *mut warc_s;
    if w.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate warc data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    r = __archive_read_register_format(
        a,
        w as *mut libc::c_void,
        b"warc\x00" as *const u8 as *const libc::c_char,
        Some(
            _warc_bid as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
            >,
        >(NULL as libc::intptr_t),
        Some(
            _warc_rdhdr
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            _warc_read
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(_warc_skip as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(_warc_cleanup as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
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
        free(w as *mut libc::c_void);
        return r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _warc_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut w: *mut warc_s = (*(*a).format).data as *mut warc_s;
    if (*w).pool.len > 0 as libc::c_uint as libc::c_ulong {
        free((*w).pool.str_0 as *mut libc::c_void);
    }
    archive_string_free(&mut (*w).sver);
    free(w as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _warc_bid(mut a: *mut archive_read, mut best_bid: libc::c_int) -> libc::c_int {
    let mut hdr: *const libc::c_char = 0 as *const libc::c_char;
    let mut nrd: ssize_t = 0;
    let mut ver: libc::c_uint = 0;
    /* UNUSED */
    /* check first line of file, it should be a record already */
    hdr = __archive_read_ahead(a, 12 as libc::c_uint as size_t, &mut nrd) as *const libc::c_char;
    if hdr.is_null() {
        /* no idea what to do */
        return -(1 as libc::c_int);
    } else {
        if nrd < 12 as libc::c_int as libc::c_long {
            /* nah, not for us, our magic cookie is at least 12 bytes */
            return -(1 as libc::c_int);
        }
    }
    /* otherwise snarf the record's version number */
    ver = _warc_rdver(hdr, nrd as size_t);
    if ver < 1200 as libc::c_uint || ver > 10000 as libc::c_uint {
        /* we only support WARC 0.12 to 1.0 */
        return -(1 as libc::c_int);
    }
    /* otherwise be confident */
    return 64 as libc::c_int;
}
unsafe extern "C" fn _warc_rdhdr(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut w: *mut warc_s = (*(*a).format).data as *mut warc_s;
    let mut ver: libc::c_uint = 0;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    let mut nrd: ssize_t = 0;
    let mut eoh: *const libc::c_char = 0 as *const libc::c_char;
    /* for the file name, saves some strndup()'ing */
    let mut fnam: warc_string_t = warc_string_t {
        len: 0,
        str_0: 0 as *const libc::c_char,
    };
    /* warc record type, not that we really use it a lot */
    let mut ftyp: warc_type_t = WT_NONE;
    /* content-length+error monad */
    let mut cntlen: ssize_t = 0;
    /* record time is the WARC-Date time we reinterpret it as ctime */
    let mut rtime: time_t = 0;
    /* mtime is the Last-Modified time which will be the entry's mtime */
    let mut mtime: time_t = 0;
    loop {
        /* just use read_ahead() they keep track of unconsumed
         * bits and bobs for us; no need to put an extra shift in
         * and reproduce that functionality here */
        buf = __archive_read_ahead(a, HDR_PROBE_LEN as size_t, &mut nrd) as *const libc::c_char;
        if nrd < 0 as libc::c_int as libc::c_long {
            /* no good */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Bad record header\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        } else {
            if buf.is_null() {
                /* there should be room for at least WARC/bla\r\n
                 * must be EOF therefore */
                return 1 as libc::c_int;
            }
        }
        /* looks good so far, try and find the end of the header now */
        eoh = _warc_find_eoh(buf, nrd as size_t);
        if eoh.is_null() {
            /* still no good, the header end might be beyond the
             * probe we've requested, but then again who'd cram
             * so much stuff into the header *and* be 28500-compliant */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Bad record header\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        ver = _warc_rdver(buf, eoh.offset_from(buf) as libc::c_long as size_t);
        /* we currently support WARC 0.12 to 1.0 */
        if ver == 0 as libc::c_uint {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid record version\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        } else {
            if ver < 1200 as libc::c_uint || ver > 10000 as libc::c_uint {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Unsupported record version: %u.%u\x00" as *const u8 as *const libc::c_char,
                    ver.wrapping_div(10000 as libc::c_int as libc::c_uint),
                    ver.wrapping_rem(10000 as libc::c_int as libc::c_uint)
                        .wrapping_div(100 as libc::c_int as libc::c_uint),
                );
                return -(30 as libc::c_int);
            }
        }
        cntlen = _warc_rdlen(buf, eoh.offset_from(buf) as libc::c_long as size_t);
        if cntlen < 0 as libc::c_int as libc::c_long {
            /* nightmare!  the specs say content-length is mandatory
             * so I don't feel overly bad stopping the reader here */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"Bad content length\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        rtime = _warc_rdrtm(buf, eoh.offset_from(buf) as libc::c_long as size_t);
        if rtime == -(1 as libc::c_int) as time_t {
            /* record time is mandatory as per WARC/1.0,
             * so just barf here, fast and loud */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"Bad record time\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* let the world know we're a WARC archive */
        (*a).archive.archive_format = ARCHIVE_FORMAT_WARC;
        if ver != (*w).pver {
            /* stringify this entry's version */
            archive_string_sprintf(
                &mut (*w).sver as *mut archive_string,
                b"WARC/%u.%u\x00" as *const u8 as *const libc::c_char,
                ver.wrapping_div(10000 as libc::c_int as libc::c_uint),
                ver.wrapping_rem(10000 as libc::c_int as libc::c_uint)
                    .wrapping_div(100 as libc::c_int as libc::c_uint),
            );
            /* remember the version */
            (*w).pver = ver
        }
        /* start off with the type */
        ftyp = _warc_rdtyp(buf, eoh.offset_from(buf) as libc::c_long as size_t)
            as warc_type_t;
        /* and let future calls know about the content */
        (*w).cntlen = cntlen as size_t; /* Avoid compiling error on some platform. */
        (*w).cntoff = 0 as libc::c_uint as size_t;
        mtime = 0 as libc::c_int as time_t;
        match ftyp as libc::c_uint {
            3 | 5 => {
                /* only try and read the filename in the cases that are
                 * guaranteed to have one */
                fnam = _warc_rduri(buf, eoh.offset_from(buf) as libc::c_long as size_t);
                /* check the last character in the URI to avoid creating
                 * directory endpoints as files, see Todo above */
                if fnam.len == 0 as libc::c_int as libc::c_ulong
                    || *fnam
                        .str_0
                        .offset(fnam.len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        == '/' as i32
                {
                    /* break here for now */
                    fnam.len = 0 as libc::c_uint as size_t;
                    fnam.str_0 = NULL as *const libc::c_char
                } else {
                    /* bang to our string pool, so we save a
                     * malloc()+free() roundtrip */
                    if fnam.len.wrapping_add(1 as libc::c_uint as libc::c_ulong) > (*w).pool.len {
                        (*w).pool.len = fnam
                            .len
                            .wrapping_add(64 as libc::c_uint as libc::c_ulong)
                            .wrapping_div(64 as libc::c_uint as libc::c_ulong)
                            .wrapping_mul(64 as libc::c_uint as libc::c_ulong);
                        (*w).pool.str_0 =
                            realloc((*w).pool.str_0 as *mut libc::c_void, (*w).pool.len)
                                as *mut libc::c_char
                    }
                    memcpy(
                        (*w).pool.str_0 as *mut libc::c_void,
                        fnam.str_0 as *const libc::c_void,
                        fnam.len,
                    );
                    *(*w).pool.str_0.offset(fnam.len as isize) = '\u{0}' as i32 as libc::c_char;
                    /* let no one else know about the pool, it's a secret, shhh */
                    fnam.str_0 = (*w).pool.str_0;
                    /* snarf mtime or deduce from rtime
                     * this is a custom header added by our writer, it's quite
                     * hard to believe anyone else would go through with it
                     * (apart from being part of some http responses of course) */
                    mtime =
                        _warc_rdmtm(buf, eoh.offset_from(buf) as libc::c_long as size_t);
                    if mtime == -(1 as libc::c_int) as time_t {
                        mtime = rtime
                    }
                }
            }
            _ => {
                fnam.len = 0 as libc::c_uint as size_t;
                fnam.str_0 = NULL as *const libc::c_char
            }
        }
        /* now eat some of those delicious buffer bits */
        __archive_read_consume(a, eoh.offset_from(buf) as libc::c_long);
        match ftyp as libc::c_uint {
            3 | 5 => {
                if fnam.len > 0 as libc::c_uint as libc::c_ulong {
                    break;
                }
            }
            _ => {}
        }
        /* FALLTHROUGH */
        /* consume the content and start over */
        _warc_skip(a);
    }
    /* populate entry object */
    archive_entry_set_filetype(entry, AE_IFREG as mode_t);
    archive_entry_copy_pathname(entry, fnam.str_0);
    archive_entry_set_size(entry, cntlen);
    archive_entry_set_perm(entry, 0o644 as libc::c_int as mode_t);
    /* rtime is the new ctime, mtime stays mtime */
    archive_entry_set_ctime(entry, rtime, 0 as libc::c_long);
    archive_entry_set_mtime(entry, mtime, 0 as libc::c_long);
    return 0 as libc::c_int;
}
pub const HDR_PROBE_LEN: libc::c_uint = 12 as libc::c_uint;
unsafe extern "C" fn _warc_read(
    mut a: *mut archive_read,
    mut buf: *mut *const libc::c_void,
    mut bsz: *mut size_t,
    mut off: *mut int64_t,
) -> libc::c_int {
    let mut w: *mut warc_s = (*(*a).format).data as *mut warc_s;
    let mut rab: *const libc::c_char = 0 as *const libc::c_char;
    let mut nrd: ssize_t = 0;
    if !((*w).cntoff >= (*w).cntlen) {
        if (*w).unconsumed != 0 {
            __archive_read_consume(a, (*w).unconsumed as int64_t);
            (*w).unconsumed = 0 as libc::c_uint as size_t
        }
        rab = __archive_read_ahead(a, 1 as libc::c_uint as size_t, &mut nrd) as *const libc::c_char;
        if nrd < 0 as libc::c_int as libc::c_long {
            *bsz = 0 as libc::c_uint as size_t;
            /* big catastrophe */
            return nrd as libc::c_int;
        } else if !(nrd == 0 as libc::c_int as libc::c_long) {
            if nrd as size_t > (*w).cntlen.wrapping_sub((*w).cntoff) {
                /* clamp to content-length */
                nrd = (*w).cntlen.wrapping_sub((*w).cntoff) as ssize_t
            }
            *off = (*w).cntoff as int64_t;
            *bsz = nrd as size_t;
            *buf = rab as *const libc::c_void;
            (*w).cntoff = ((*w).cntoff as libc::c_ulong).wrapping_add(nrd as libc::c_ulong)
                as size_t as size_t;
            (*w).unconsumed = nrd as size_t;
            return 0 as libc::c_int;
        }
    }
    /* it's our lucky day, no work, we can leave early */
    *buf = NULL as *const libc::c_void; /*for \r\n\r\n separator*/
    *bsz = 0 as libc::c_uint as size_t;
    *off = (*w).cntoff.wrapping_add(4 as libc::c_uint as libc::c_ulong) as int64_t;
    (*w).unconsumed = 0 as libc::c_uint as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _warc_skip(mut a: *mut archive_read) -> libc::c_int {
    let mut w: *mut warc_s = (*(*a).format).data as *mut warc_s;
    __archive_read_consume(
        a,
        (*w).cntlen.wrapping_add(4 as libc::c_uint as libc::c_ulong) as int64_t,
    );
    (*w).cntlen = 0 as libc::c_uint as size_t;
    (*w).cntoff = 0 as libc::c_uint as size_t;
    return 0 as libc::c_int;
}
/* private routines */
unsafe extern "C" fn deconst(mut c: *const libc::c_void) -> *mut libc::c_void {
    return (0x1 as libc::c_int as *mut libc::c_char).offset(
        (c as *const libc::c_char).offset_from(0x1 as libc::c_int as *const libc::c_char)
            as libc::c_long as isize,
    ) as *mut libc::c_void;
}
unsafe extern "C" fn xmemmem(
    mut hay: *const libc::c_char,
    haysize: size_t,
    mut needle: *const libc::c_char,
    needlesize: size_t,
) -> *mut libc::c_char {
    let eoh: *const libc::c_char = hay.offset(haysize as isize);
    let eon: *const libc::c_char = needle.offset(needlesize as isize);
    let mut hp: *const libc::c_char = 0 as *const libc::c_char;
    let mut np: *const libc::c_char = 0 as *const libc::c_char;
    let mut cand: *const libc::c_char = 0 as *const libc::c_char;
    let mut hsum: libc::c_uint = 0;
    let mut nsum: libc::c_uint = 0;
    let mut eqp: libc::c_uint = 0;
    /* trivial checks first
     * a 0-sized needle is defined to be found anywhere in haystack
     * then run strchr() to find a candidate in HAYSTACK (i.e. a portion
     * that happens to begin with *NEEDLE) */
    if needlesize == 0 as libc::c_ulong {
        return deconst(hay as *const libc::c_void) as *mut libc::c_char;
    } else {
        hay = memchr(hay as *const libc::c_void, *needle as libc::c_int, haysize)
            as *const libc::c_char;
        if hay.is_null() {
            /* trivial */
            return NULL as *mut libc::c_char;
        }
    }
    /* First characters of haystack and needle are the same now. Both are
     * guaranteed to be at least one character long.  Now computes the sum
     * of characters values of needle together with the sum of the first
     * needle_len characters of haystack. */
    hp = hay.offset(1 as libc::c_uint as isize);
    np = needle.offset(1 as libc::c_uint as isize);
    hsum = *hay as libc::c_uint;
    nsum = *hay as libc::c_uint;
    eqp = 1 as libc::c_uint;
    while hp < eoh && np < eon {
        hsum ^= *hp as libc::c_uint;
        nsum ^= *np as libc::c_uint;
        eqp &= (*hp as libc::c_int == *np as libc::c_int) as libc::c_int as libc::c_uint;
        hp = hp.offset(1);
        np = np.offset(1)
    }
    /* HP now references the (NEEDLESIZE + 1)-th character. */
    if np < eon {
        /* haystack is smaller than needle, :O */
        return NULL as *mut libc::c_char;
    } else {
        if eqp != 0 {
            /* found a match */
            return deconst(hay as *const libc::c_void) as *mut libc::c_char;
        }
    }
    /* now loop through the rest of haystack,
     * updating the sum iteratively */
    cand = hay;
    while hp < eoh {
        let fresh0 = cand;
        cand = cand.offset(1);
        hsum ^= *fresh0 as libc::c_uint;
        hsum ^= *hp as libc::c_uint;
        /* Since the sum of the characters is already known to be
         * equal at that point, it is enough to check just NEEDLESIZE - 1
         * characters for equality,
         * also CAND is by design < HP, so no need for range checks */
        if hsum == nsum
            && memcmp(
                cand as *const libc::c_void,
                needle as *const libc::c_void,
                needlesize.wrapping_sub(1 as libc::c_uint as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            return deconst(cand as *const libc::c_void) as *mut libc::c_char;
        }
        hp = hp.offset(1)
    }
    return NULL as *mut libc::c_char;
}
unsafe extern "C" fn strtoi_lim(
    mut str: *const libc::c_char,
    mut ep: *mut *const libc::c_char,
    mut llim: libc::c_int,
    mut ulim: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut sp: *const libc::c_char = 0 as *const libc::c_char;
    /* we keep track of the number of digits via rulim */
    let mut rulim: libc::c_int = 0;
    sp = str;
    rulim = (if ulim > 10 as libc::c_int {
        ulim
    } else {
        10 as libc::c_int
    });
    while res * 10 as libc::c_int <= ulim
        && rulim != 0
        && *sp as libc::c_int >= '0' as i32
        && *sp as libc::c_int <= '9' as i32
    {
        res *= 10 as libc::c_int;
        res += *sp as libc::c_int - '0' as i32;
        sp = sp.offset(1);
        rulim /= 10 as libc::c_int
    }
    if sp == str {
        res = -(1 as libc::c_int)
    } else if res < llim || res > ulim {
        res = -(2 as libc::c_int)
    }
    *ep = sp;
    return res;
}
unsafe extern "C" fn time_from_tm(mut t: *mut tm) -> time_t {
    /* Use platform timegm() if available. */
    return timegm(t);
}
unsafe extern "C" fn xstrpisotime(
    mut s: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
) -> time_t {
    /* * like strptime() but strictly for ISO 8601 Zulu strings */
    let mut tm: tm = tm {
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
    };
    let mut res: time_t = -(1 as libc::c_int) as time_t;
    /* make sure tm is clean */
    memset(
        &mut tm as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tm>() as libc::c_ulong,
    );
    /* as a courtesy to our callers, and since this is a non-standard
     * routine, we skip leading whitespace */
    while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32 {
        s = s.offset(1)
    }
    /* read year */
    tm.tm_year = strtoi_lim(s, &mut s, 1583 as libc::c_int, 4095 as libc::c_int);
    if !(tm.tm_year < 0 as libc::c_int || {
        let fresh1 = s;
        s = s.offset(1);
        (*fresh1 as libc::c_int) != '-' as i32
    }) {
        /* read month */
        tm.tm_mon = strtoi_lim(s, &mut s, 1 as libc::c_int, 12 as libc::c_int);
        if !(tm.tm_mon < 0 as libc::c_int || {
            let fresh2 = s;
            s = s.offset(1);
            (*fresh2 as libc::c_int) != '-' as i32
        }) {
            /* read day-of-month */
            tm.tm_mday = strtoi_lim(s, &mut s, 1 as libc::c_int, 31 as libc::c_int);
            if !(tm.tm_mday < 0 as libc::c_int || {
                let fresh3 = s;
                s = s.offset(1);
                (*fresh3 as libc::c_int) != 'T' as i32
            }) {
                /* read hour */
                tm.tm_hour = strtoi_lim(s, &mut s, 0 as libc::c_int, 23 as libc::c_int);
                if !(tm.tm_hour < 0 as libc::c_int || {
                    let fresh4 = s;
                    s = s.offset(1);
                    (*fresh4 as libc::c_int) != ':' as i32
                }) {
                    /* read minute */
                    tm.tm_min = strtoi_lim(s, &mut s, 0 as libc::c_int, 59 as libc::c_int);
                    if !(tm.tm_min < 0 as libc::c_int || {
                        let fresh5 = s;
                        s = s.offset(1);
                        (*fresh5 as libc::c_int) != ':' as i32
                    }) {
                        /* read second */
                        tm.tm_sec = strtoi_lim(s, &mut s, 0 as libc::c_int, 60 as libc::c_int);
                        if !(tm.tm_sec < 0 as libc::c_int || {
                            let fresh6 = s;
                            s = s.offset(1);
                            (*fresh6 as libc::c_int) != 'Z' as i32
                        }) {
                            /* massage TM to fulfill some of POSIX' constraints */
                            tm.tm_year -= 1900 as libc::c_int;
                            tm.tm_mon -= 1;
                            /* now convert our custom tm struct to a unix stamp using UTC */
                            res = time_from_tm(&mut tm)
                        }
                    }
                }
            }
        }
    }
    if !endptr.is_null() {
        *endptr = deconst(s as *const libc::c_void) as *mut libc::c_char
    }
    return res;
}
/* private routines */
unsafe extern "C" fn _warc_rdver(mut buf: *const libc::c_char, mut bsz: size_t) -> libc::c_uint {
    static mut magic: [libc::c_char; 6] =
        unsafe { *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"WARC/\x00") };
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    let mut ver: libc::c_uint = 0 as libc::c_uint;
    let mut end: libc::c_uint = 0 as libc::c_uint;
    if bsz < 12 as libc::c_int as libc::c_ulong
        || memcmp(
            buf as *const libc::c_void,
            magic.as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
        ) != 0 as libc::c_int
    {
        /* buffer too small or invalid magic */
        return ver;
    }
    /* looks good so far, read the version number for a laugh */
    buf = buf.offset(
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as isize,
    );
    if *(*__ctype_b_loc())
        .offset(*buf.offset(0 as libc::c_uint as isize) as libc::c_uchar as libc::c_int as isize)
        as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
        && *buf.offset(1 as libc::c_uint as isize) as libc::c_int == '.' as i32
        && *(*__ctype_b_loc()).offset(
            *buf.offset(2 as libc::c_uint as isize) as libc::c_uchar as libc::c_int as isize
        ) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        /* we support a maximum of 2 digits in the minor version */
        if *(*__ctype_b_loc()).offset(
            *buf.offset(3 as libc::c_uint as isize) as libc::c_uchar as libc::c_int as isize
        ) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            end = 1 as libc::c_uint
        }
        /* set up major version */
        ver = ((*buf.offset(0 as libc::c_uint as isize) as libc::c_int - '0' as i32)
            as libc::c_uint)
            .wrapping_mul(10000 as libc::c_uint);
        /* set up minor version */
        if end == 1 as libc::c_uint {
            ver = ver.wrapping_add(
                ((*buf.offset(2 as libc::c_uint as isize) as libc::c_int - '0' as i32)
                    as libc::c_uint)
                    .wrapping_mul(1000 as libc::c_uint),
            );
            ver = ver.wrapping_add(
                ((*buf.offset(3 as libc::c_uint as isize) as libc::c_int - '0' as i32)
                    as libc::c_uint)
                    .wrapping_mul(100 as libc::c_uint),
            )
        } else {
            ver = ver.wrapping_add(
                ((*buf.offset(2 as libc::c_uint as isize) as libc::c_int - '0' as i32)
                    as libc::c_uint)
                    .wrapping_mul(100 as libc::c_uint),
            )
        }
        /*
         * WARC below version 0.12 has a space-separated header
         * WARC 0.12 and above terminates the version with a CRLF
         */
        c = buf.offset(3 as libc::c_uint as isize).offset(end as isize);
        if ver >= 1200 as libc::c_uint {
            if memcmp(
                c as *const libc::c_void,
                b"\r\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_uint as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                ver = 0 as libc::c_uint
            }
        } else if *c as libc::c_int != ' ' as i32 && *c as libc::c_int != '\t' as i32 {
            ver = 0 as libc::c_uint
        }
    }
    return ver;
}
unsafe extern "C" fn _warc_rdtyp(mut buf: *const libc::c_char, mut bsz: size_t) -> libc::c_uint {
    static mut _key: [libc::c_char; 13] =
        unsafe { *::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"\r\nWARC-Type:\x00") };
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut eol: *const libc::c_char = 0 as *const libc::c_char;
    val = xmemmem(
        buf,
        bsz,
        _key.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
    );
    if val.is_null() {
        /* ver < 1200U */
        /* no bother */
        return WT_NONE as libc::c_int as libc::c_uint;
    }
    val = val.offset(
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as isize,
    );
    eol = _warc_find_eol(
        val,
        buf.offset(bsz as isize).offset_from(val) as libc::c_long as size_t,
    );
    if eol.is_null() {
        /* no end of line */
        return WT_NONE as libc::c_int as libc::c_uint;
    }
    /* overread whitespace */
    while val < eol && (*val as libc::c_int == ' ' as i32 || *val as libc::c_int == '\t' as i32) {
        val = val.offset(1)
    }
    if val.offset(8 as libc::c_uint as isize) == eol {
        if memcmp(
            val as *const libc::c_void,
            b"resource\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_uint as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return WT_RSRC as libc::c_int as libc::c_uint;
        } else {
            if memcmp(
                val as *const libc::c_void,
                b"response\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                8 as libc::c_uint as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                return WT_RSP as libc::c_int as libc::c_uint;
            }
        }
    }
    return WT_NONE as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn _warc_rduri(mut buf: *const libc::c_char, mut bsz: size_t) -> warc_string_t {
    static mut _key: [libc::c_char; 19] = unsafe {
        *::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"\r\nWARC-Target-URI:\x00")
    };
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut uri: *const libc::c_char = 0 as *const libc::c_char;
    let mut eol: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut res: warc_string_t = {
        let mut init = warc_string_t {
            len: 0 as libc::c_uint as size_t,
            str_0: NULL as *const libc::c_char,
        };
        init
    };
    val = xmemmem(
        buf,
        bsz,
        _key.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
    );
    if val.is_null() {
        /* no bother */
        return res;
    }
    /* overread whitespace */
    val = val.offset(
        (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as isize,
    );
    eol = _warc_find_eol(
        val,
        buf.offset(bsz as isize).offset_from(val) as libc::c_long as size_t,
    );
    if eol.is_null() {
        /* no end of line */
        return res;
    }
    while val < eol && (*val as libc::c_int == ' ' as i32 || *val as libc::c_int == '\t' as i32) {
        val = val.offset(1)
    }
    /* overread URL designators */
    uri = xmemmem(
        val,
        eol.offset_from(val) as libc::c_long as size_t,
        b"://\x00" as *const u8 as *const libc::c_char,
        3 as libc::c_uint as size_t,
    );
    if uri.is_null() {
        /* not touching that! */
        return res;
    }
    /* spaces inside uri are not allowed, CRLF should follow */
    p = val;
    while p < eol {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            return res;
        }
        p = p.offset(1)
    }
    /* there must be at least space for ftp */
    if uri < val.offset(3 as libc::c_uint as isize) {
        return res;
    }
    /* move uri to point to after :// */
    uri = uri.offset(3 as libc::c_uint as isize);
    /* now then, inspect the URI */
    if !(memcmp(
        val as *const libc::c_void,
        b"file\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_uint as libc::c_ulong,
    ) == 0 as libc::c_int)
    {
        if memcmp(
            val as *const libc::c_void,
            b"http\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_uint as libc::c_ulong,
        ) == 0 as libc::c_int
            || memcmp(
                val as *const libc::c_void,
                b"ftp\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_uint as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            /* overread domain, and the first / */
            while uri < eol && {
                let fresh7 = uri;
                uri = uri.offset(1);
                (*fresh7 as libc::c_int) != '/' as i32
            } {}
        } else {
            /* not sure what to do? best to bugger off */
            return res;
        }
    }
    res.str_0 = uri;
    res.len = eol.offset_from(uri) as libc::c_long as size_t;
    return res;
}
unsafe extern "C" fn _warc_rdlen(mut buf: *const libc::c_char, mut bsz: size_t) -> ssize_t {
    static mut _key: [libc::c_char; 18] = unsafe {
        *::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"\r\nContent-Length:\x00")
    };
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut eol: *const libc::c_char = 0 as *const libc::c_char;
    let mut on: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut len: libc::c_long = 0;
    val = xmemmem(
        buf,
        bsz,
        _key.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
    );
    if val.is_null() {
        /* no bother */
        return -(1 as libc::c_int) as ssize_t;
    }
    val = val.offset(
        (::std::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as isize,
    );
    eol = _warc_find_eol(
        val,
        buf.offset(bsz as isize).offset_from(val) as libc::c_long as size_t,
    );
    if eol.is_null() {
        /* no end of line */
        return -(1 as libc::c_int) as ssize_t;
    }
    /* skip leading whitespace */
    while val < eol && (*val as libc::c_int == ' ' as i32 || *val as libc::c_int == '\t' as i32) {
        val = val.offset(1)
    }
    /* there must be at least one digit */
    if *(*__ctype_b_loc()).offset(*val as libc::c_uchar as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return -(1 as libc::c_int) as ssize_t;
    }
    errno = 0 as libc::c_int;
    len = strtol(val, &mut on, 10 as libc::c_int);
    if errno != 0 as libc::c_int || on != eol as *mut libc::c_char {
        /* line must end here */
        return -(1 as libc::c_int) as ssize_t;
    }
    return len as size_t as ssize_t;
}
unsafe extern "C" fn _warc_rdrtm(mut buf: *const libc::c_char, mut bsz: size_t) -> time_t {
    static mut _key: [libc::c_char; 13] =
        unsafe { *::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"\r\nWARC-Date:\x00") };
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut eol: *const libc::c_char = 0 as *const libc::c_char;
    let mut on: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut res: time_t = 0;
    val = xmemmem(
        buf,
        bsz,
        _key.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
    );
    if val.is_null() {
        /* no bother */
        return -(1 as libc::c_int) as time_t;
    }
    val = val.offset(
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as isize,
    );
    eol = _warc_find_eol(
        val,
        buf.offset(bsz as isize).offset_from(val) as libc::c_long as size_t,
    );
    if eol.is_null() {
        /* no end of line */
        return -(1 as libc::c_int) as time_t;
    }
    /* xstrpisotime() kindly overreads whitespace for us, so use that */
    res = xstrpisotime(val, &mut on);
    if on != eol as *mut libc::c_char {
        /* line must end here */
        return -(1 as libc::c_int) as time_t;
    }
    return res;
}
unsafe extern "C" fn _warc_rdmtm(mut buf: *const libc::c_char, mut bsz: size_t) -> time_t {
    static mut _key: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"\r\nLast-Modified:\x00")
    };
    let mut val: *const libc::c_char = 0 as *const libc::c_char;
    let mut eol: *const libc::c_char = 0 as *const libc::c_char;
    let mut on: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut res: time_t = 0;
    val = xmemmem(
        buf,
        bsz,
        _key.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
    );
    if val.is_null() {
        /* no bother */
        return -(1 as libc::c_int) as time_t;
    }
    val = val.offset(
        (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as isize,
    );
    eol = _warc_find_eol(
        val,
        buf.offset(bsz as isize).offset_from(val) as libc::c_long as size_t,
    );
    if eol.is_null() {
        /* no end of line */
        return -(1 as libc::c_int) as time_t;
    }
    /* xstrpisotime() kindly overreads whitespace for us, so use that */
    res = xstrpisotime(val, &mut on);
    if on != eol as *mut libc::c_char {
        /* line must end here */
        return -(1 as libc::c_int) as time_t;
    }
    return res;
}
unsafe extern "C" fn _warc_find_eoh(
    mut buf: *const libc::c_char,
    mut bsz: size_t,
) -> *const libc::c_char {
    static mut _marker: [libc::c_char; 5] =
        unsafe { *::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\r\n\r\n\x00") };
    let mut hit: *const libc::c_char = xmemmem(
        buf,
        bsz,
        _marker.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
    );
    if !hit.is_null() {
        hit = hit.offset(
            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_uint as libc::c_ulong) as isize,
        )
    }
    return hit;
}
unsafe extern "C" fn _warc_find_eol(
    mut buf: *const libc::c_char,
    mut bsz: size_t,
) -> *const libc::c_char {
    static mut _marker: [libc::c_char; 3] =
        unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"\r\n\x00") };
    let mut hit: *const libc::c_char = xmemmem(
        buf,
        bsz,
        _marker.as_ptr(),
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_uint as libc::c_ulong),
    );
    return hit;
}
/* archive_read_support_format_warc.c ends here */
