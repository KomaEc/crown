use ::libc;
extern "C" {
    pub type internal_state;
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
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
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_read_get_bidder(
        a: *mut archive_read,
        bidder: *mut *mut archive_read_filter_bidder,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_read_filter_ahead(
        _: *mut archive_read_filter,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn __archive_read_filter_consume(_: *mut archive_read_filter, _: int64_t) -> int64_t;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
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
pub struct private_data {
    pub stream: z_stream,
    pub in_stream: libc::c_char,
    pub out_block: *mut libc::c_uchar,
    pub out_block_size: size_t,
    pub total_out: int64_t,
    pub crc: libc::c_ulong,
    pub mtime: uint32_t,
    pub name: *mut libc::c_char,
    pub eof: libc::c_char,
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
pub const __LONG_MAX__: libc::c_long = 9223372036854775807 as libc::c_long;
pub const LONG_MAX: libc::c_long = __LONG_MAX__;
pub const SSIZE_MAX: libc::c_long = LONG_MAX;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_STREAM_ERROR: libc::c_int = -(2 as libc::c_int);
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const Z_VERSION_ERROR: libc::c_int = -(6 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_FILTER_GZIP: libc::c_int = 1 as libc::c_int;
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
/* Deprecated; remove in libarchive 4.0 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_compression_gzip(mut a: *mut archive) -> libc::c_int {
    return archive_read_support_filter_gzip(a); /* No data, so no cleanup necessary. */
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_gzip(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_filter_gzip\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if __archive_read_get_bidder(a, &mut bidder) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*bidder).data = NULL as *mut libc::c_void;
    (*bidder).name = b"gzip\x00" as *const u8 as *const libc::c_char;
    (*bidder).bid = Some(
        gzip_bidder_bid
            as unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *mut archive_read_filter,
            ) -> libc::c_int,
    );
    (*bidder).init =
        Some(gzip_bidder_init as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int);
    (*bidder).options = ::std::mem::transmute::<
        libc::intptr_t,
        Option<
            unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
        >,
    >(NULL as libc::intptr_t);
    (*bidder).free = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut archive_read_filter_bidder) -> libc::c_int>,
    >(NULL as libc::intptr_t);
    /* Signal the extent of gzip support with the return value here. */
    return 0 as libc::c_int;
}
/*
 * Read and verify the header.
 *
 * Returns zero if the header couldn't be validated, else returns
 * number of bytes in header.  If pbits is non-NULL, it receives a
 * count of bits verified, suitable for use by bidder.
 */
unsafe extern "C" fn peek_at_header(
    mut filter: *mut archive_read_filter,
    mut pbits: *mut libc::c_int,
    mut state: *mut private_data,
) -> ssize_t {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: ssize_t = 0;
    let mut len: ssize_t = 0;
    let mut bits: libc::c_int = 0 as libc::c_int;
    let mut header_flags: libc::c_int = 0;
    /* Start by looking at the first ten bytes of the header, which
     * is all fixed layout. */
    len = 10 as libc::c_int as ssize_t;
    p = __archive_read_filter_ahead(filter, len as size_t, &mut avail) as *const libc::c_uchar;
    if p.is_null() || avail == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as ssize_t;
    }
    /* We only support deflation- third byte must be 0x08. */
    if memcmp(
        p as *const libc::c_void,
        b"\x1f\x8b\x08\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int as ssize_t;
    }
    bits += 24 as libc::c_int;
    if *p.offset(3 as libc::c_int as isize) as libc::c_int & 0xe0 as libc::c_int != 0 as libc::c_int
    {
        /* No reserved flags set. */
        return 0 as libc::c_int as ssize_t;
    }
    bits += 3 as libc::c_int;
    header_flags = *p.offset(3 as libc::c_int as isize) as libc::c_int;
    /* Bytes 4-7 are mod time in little endian. */
    if !state.is_null() {
        (*state).mtime = archive_le32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void)
    }
    /* Byte 8 is deflate flags. */
    /* XXXX TODO: return deflate flags back to consume_header for use
    in initializing the decompressor. */
    /* Byte 9 is OS. */
    /* Optional extra data:  2 byte length plus variable body. */
    if header_flags & 4 as libc::c_int != 0 {
        p = __archive_read_filter_ahead(
            filter,
            (len + 2 as libc::c_int as libc::c_long) as size_t,
            &mut avail,
        ) as *const libc::c_uchar;
        if p.is_null() {
            return 0 as libc::c_int as ssize_t;
        }
        len += ((*p.offset((len + 1 as libc::c_int as libc::c_long) as isize) as libc::c_int)
            << 8 as libc::c_int
            | *p.offset(len as isize) as libc::c_int) as libc::c_long;
        len += 2 as libc::c_int as libc::c_long
    }
    /* Null-terminated optional filename. */
    if header_flags & 8 as libc::c_int != 0 {
        let mut file_start: ssize_t = len;
        loop {
            len += 1;
            if avail < len {
                p = __archive_read_filter_ahead(filter, len as size_t, &mut avail)
                    as *const libc::c_uchar
            }
            if p.is_null() {
                return 0 as libc::c_int as ssize_t;
            }
            if !(*p.offset((len - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
                != 0 as libc::c_int)
            {
                break;
            }
        }
        if !state.is_null() {
            /* Reset the name in case of repeat header reads. */
            free((*state).name as *mut libc::c_void);
            (*state).name = strdup(
                &*p.offset(file_start as isize) as *const libc::c_uchar as *const libc::c_char
            )
        }
    }
    /* Null-terminated optional comment. */
    if header_flags & 16 as libc::c_int != 0 {
        loop {
            len += 1;
            if avail < len {
                p = __archive_read_filter_ahead(filter, len as size_t, &mut avail)
                    as *const libc::c_uchar
            }
            if p.is_null() {
                return 0 as libc::c_int as ssize_t;
            }
            if !(*p.offset((len - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
                != 0 as libc::c_int)
            {
                break;
            }
        }
    }
    /* Optional header CRC */
    if header_flags & 2 as libc::c_int != 0 {
        p = __archive_read_filter_ahead(
            filter,
            (len + 2 as libc::c_int as libc::c_long) as size_t,
            &mut avail,
        ) as *const libc::c_uchar;
        if p.is_null() {
            return 0 as libc::c_int as ssize_t;
        }
        len += 2 as libc::c_int as libc::c_long
    }
    if !pbits.is_null() {
        *pbits = bits
    }
    return len;
}
/*
 * Note that we can detect gzip archives even if we can't decompress
 * them.  (In fact, we like detecting them because we can give better
 * error messages.)  So the bid framework here gets compiled even
 * if zlib is unavailable.
 *
 * TODO: If zlib is unavailable, gzip_bidder_init() should
 * use the compress_program framework to try to fire up an external
 * gzip program.
 */
/*
 * Bidder just verifies the header and returns the number of verified bits.
 */
unsafe extern "C" fn gzip_bidder_bid(
    mut self_0: *mut archive_read_filter_bidder,
    mut filter: *mut archive_read_filter,
) -> libc::c_int {
    let mut bits_checked: libc::c_int = 0;
    /* UNUSED */
    if peek_at_header(filter, &mut bits_checked, NULL as *mut private_data) != 0 {
        return bits_checked;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gzip_read_header(
    mut self_0: *mut archive_read_filter,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut state: *mut private_data = 0 as *mut private_data;
    state = (*self_0).data as *mut private_data;
    /* A mtime of 0 is considered invalid/missing. */
    if (*state).mtime != 0 as libc::c_int as libc::c_uint {
        archive_entry_set_mtime(
            entry,
            (*state).mtime as time_t,
            0 as libc::c_int as libc::c_long,
        );
    }
    /* If the name is available, extract it. */
    if !(*state).name.is_null() {
        archive_entry_set_pathname(entry, (*state).name);
    }
    return 0 as libc::c_int;
}
/*
 * Initialize the filter object.
 */
unsafe extern "C" fn gzip_bidder_init(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut private_data = 0 as *mut private_data; /* not supported */
    static mut out_block_size: size_t = (64 as libc::c_int * 1024 as libc::c_int) as size_t; /* We're not actually within a stream yet. */
    let mut out_block: *mut libc::c_void = 0 as *mut libc::c_void;
    (*self_0).code = ARCHIVE_FILTER_GZIP;
    (*self_0).name = b"gzip\x00" as *const u8 as *const libc::c_char;
    state = calloc(
        ::std::mem::size_of::<private_data>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut private_data;
    out_block = malloc(out_block_size) as *mut libc::c_uchar as *mut libc::c_void;
    if state.is_null() || out_block.is_null() {
        free(out_block);
        free(state as *mut libc::c_void);
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate data for gzip decompression\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*self_0).data = state as *mut libc::c_void;
    (*state).out_block_size = out_block_size;
    (*state).out_block = out_block as *mut libc::c_uchar;
    (*self_0).read = Some(
        gzip_filter_read
            as unsafe extern "C" fn(
                _: *mut archive_read_filter,
                _: *mut *const libc::c_void,
            ) -> ssize_t,
    );
    (*self_0).skip = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut archive_read_filter, _: int64_t) -> int64_t>,
    >(NULL as libc::intptr_t);
    (*self_0).close =
        Some(gzip_filter_close as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int);
    (*self_0).read_header = Some(
        gzip_read_header
            as unsafe extern "C" fn(
                _: *mut archive_read_filter,
                _: *mut archive_entry,
            ) -> libc::c_int,
    );
    (*state).in_stream = 0 as libc::c_int as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn consume_header(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut private_data = 0 as *mut private_data;
    let mut avail: ssize_t = 0;
    let mut len: size_t = 0;
    let mut ret: libc::c_int = 0;
    state = (*self_0).data as *mut private_data;
    /* If this is a real header, consume it. */
    len = peek_at_header((*self_0).upstream, NULL as *mut libc::c_int, state) as size_t;
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    __archive_read_filter_consume((*self_0).upstream, len as int64_t);
    /* Initialize CRC accumulator. */
    (*state).crc = crc32(
        0 as libc::c_long as uLong,
        NULL as *const Bytef,
        0 as libc::c_int as uInt,
    );
    /* Initialize compression library. */
    (*state).stream.next_in =
        __archive_read_filter_ahead((*self_0).upstream, 1 as libc::c_int as size_t, &mut avail)
            as uintptr_t as *mut libc::c_uchar;
    (*state).stream.avail_in = avail as uInt;
    ret = inflateInit2_(
        &mut (*state).stream,
        -(15 as libc::c_int),
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    /* Don't check for zlib header */
    /* Decipher the error code. */
    match ret {
        Z_OK => {
            (*state).in_stream = 1 as libc::c_int as libc::c_char;
            return 0 as libc::c_int;
        }
        -2 => {
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal error initializing compression library: invalid setup parameter\x00"
                    as *const u8 as *const libc::c_char,
            );
        }
        -4 => {
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ENOMEM,
                b"Internal error initializing compression library: out of memory\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        -6 => {
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal error initializing compression library: invalid library version\x00"
                    as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal error initializing compression library:  Zlib error %d\x00" as *const u8
                    as *const libc::c_char,
                ret,
            );
        }
    }
    return -(30 as libc::c_int);
}
unsafe extern "C" fn consume_trailer(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut private_data = 0 as *mut private_data;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: ssize_t = 0;
    state = (*self_0).data as *mut private_data;
    (*state).in_stream = 0 as libc::c_int as libc::c_char;
    match inflateEnd(&mut (*state).stream) {
        Z_OK => {}
        _ => {
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Failed to clean up gzip decompressor\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    /* GZip trailer is a fixed 8 byte structure. */
    p = __archive_read_filter_ahead((*self_0).upstream, 8 as libc::c_int as size_t, &mut avail)
        as *const libc::c_uchar;
    if p.is_null() || avail == 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    /* XXX TODO: Verify the length and CRC. */
    /* We've verified the trailer, so consume it now. */
    __archive_read_filter_consume((*self_0).upstream, 8 as libc::c_int as int64_t);
    return 0 as libc::c_int;
}
/* True = found end of compressed data. */
/* Gzip Filter. */
unsafe extern "C" fn gzip_filter_read(
    mut self_0: *mut archive_read_filter,
    mut p: *mut *const libc::c_void,
) -> ssize_t {
    let mut state: *mut private_data = 0 as *mut private_data;
    let mut decompressed: size_t = 0;
    let mut avail_in: ssize_t = 0;
    let mut max_in: ssize_t = 0;
    let mut ret: libc::c_int = 0;
    state = (*self_0).data as *mut private_data;
    /* Empty our output buffer. */
    (*state).stream.next_out = (*state).out_block;
    (*state).stream.avail_out = (*state).out_block_size as uInt;
    /* Try to fill the output buffer. */
    while (*state).stream.avail_out > 0 as libc::c_int as libc::c_uint && (*state).eof == 0 {
        /* If we're not in a stream, read a header
         * and initialize the decompression library. */
        if (*state).in_stream == 0 {
            ret = consume_header(self_0);
            if ret == ARCHIVE_EOF {
                (*state).eof = 1 as libc::c_int as libc::c_char;
                break;
            } else if ret < ARCHIVE_OK {
                return ret as ssize_t;
            }
        }
        /* Peek at the next available data. */
        /* ZLib treats stream.next_in as const but doesn't declare
         * it so, hence this ugly cast. */
        (*state).stream.next_in = __archive_read_filter_ahead(
            (*self_0).upstream,
            1 as libc::c_int as size_t,
            &mut avail_in,
        ) as uintptr_t as *mut libc::c_uchar;
        if (*state).stream.next_in.is_null() {
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"truncated gzip input\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        if (__INT_MAX__ as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) as libc::c_long
            >= SSIZE_MAX
        {
            max_in = SSIZE_MAX
        } else {
            max_in = (__INT_MAX__ as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint) as ssize_t
        }
        if avail_in > max_in {
            avail_in = max_in
        }
        (*state).stream.avail_in = avail_in as uInt;
        /* Decompress and consume some of that data. */
        ret = inflate(&mut (*state).stream, 0 as libc::c_int);
        match ret {
            Z_OK => {
                /* Decompressor made some progress. */
                __archive_read_filter_consume(
                    (*self_0).upstream,
                    avail_in - (*state).stream.avail_in as libc::c_long,
                );
            }
            Z_STREAM_END => {
                /* Found end of stream. */
                __archive_read_filter_consume(
                    (*self_0).upstream,
                    avail_in - (*state).stream.avail_in as libc::c_long,
                );
                /* Consume the stream trailer; release the
                 * decompression library. */
                ret = consume_trailer(self_0);
                if ret < ARCHIVE_OK {
                    return ret as ssize_t;
                }
            }
            _ => {
                /* Return an error. */
                archive_set_error(
                    &mut (*(*self_0).archive).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"gzip decompression failed\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int) as ssize_t;
            }
        }
    }
    /* We've read as much as we can. */
    decompressed = (*state)
        .stream
        .next_out
        .offset_from((*state).out_block) as libc::c_long as size_t;
    (*state).total_out =
        ((*state).total_out as libc::c_ulong).wrapping_add(decompressed) as int64_t as int64_t;
    if decompressed == 0 as libc::c_int as libc::c_ulong {
        *p = NULL as *const libc::c_void
    } else {
        *p = (*state).out_block as *const libc::c_void
    }
    return decompressed as ssize_t;
}
/*
 * Clean up the decompressor.
 */
unsafe extern "C" fn gzip_filter_close(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut private_data = 0 as *mut private_data;
    let mut ret: libc::c_int = 0;
    state = (*self_0).data as *mut private_data;
    ret = ARCHIVE_OK;
    if (*state).in_stream != 0 {
        match inflateEnd(&mut (*state).stream) {
            Z_OK => {}
            _ => {
                archive_set_error(
                    &mut (*(*self_0).archive).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Failed to clean up gzip compressor\x00" as *const u8 as *const libc::c_char,
                );
                ret = ARCHIVE_FATAL
            }
        }
    }
    free((*state).name as *mut libc::c_void);
    free((*state).out_block as *mut libc::c_void);
    free(state as *mut libc::c_void);
    return ret;
}
/* HAVE_ZLIB_H */
