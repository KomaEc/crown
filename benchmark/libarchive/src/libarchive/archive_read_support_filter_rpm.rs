use ::libc;
extern "C" {
    /* Length of malloc-ed storage in bytes. */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
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
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type la_int64_t = int64_t;
pub type la_ssize_t = ssize_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const ST_ARCHIVE: C2RustUnnamed_0 = 4;
pub const ST_PADDING: C2RustUnnamed_0 = 3;
pub const ST_HEADER_DATA: C2RustUnnamed_0 = 2;
pub const ST_HEADER: C2RustUnnamed_0 = 1;
pub const ST_LEAD: C2RustUnnamed_0 = 0;
/*-
 * Copyright (c) 2009 Michihiro NAKAJIMA
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
pub struct rpm {
    pub total_in: int64_t,
    pub hpos: size_t,
    pub hlen: size_t,
    pub header: [libc::c_uchar; 16],
    pub state: C2RustUnnamed_0,
    pub first_header: libc::c_int,
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FILTER_RPM: libc::c_int = 8 as libc::c_int;
#[inline]
unsafe extern "C" fn archive_be32dec(mut pp: *const libc::c_void) -> uint32_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p3: libc::c_uint = *p.offset(3 as libc::c_int as isize) as libc::c_uint;
    let mut p2: libc::c_uint = *p.offset(2 as libc::c_int as isize) as libc::c_uint;
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return p0 << 24 as libc::c_int | p1 << 16 as libc::c_int | p2 << 8 as libc::c_int | p3;
}
pub const RPM_LEAD_SIZE: libc::c_int = 96 as libc::c_int;
/* Deprecated; remove in libarchive 4.0 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_compression_rpm(mut a: *mut archive) -> libc::c_int {
    return archive_read_support_filter_rpm(a);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_rpm(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_filter_rpm\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if __archive_read_get_bidder(a, &mut bidder) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*bidder).data = NULL as *mut libc::c_void;
    (*bidder).name = b"rpm\x00" as *const u8 as *const libc::c_char;
    (*bidder).bid = Some(
        rpm_bidder_bid
            as unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *mut archive_read_filter,
            ) -> libc::c_int,
    );
    (*bidder).init =
        Some(rpm_bidder_init as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int);
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
    return 0 as libc::c_int;
}
/* Size of 'Lead' section. */
unsafe extern "C" fn rpm_bidder_bid(
    mut self_0: *mut archive_read_filter_bidder,
    mut filter: *mut archive_read_filter,
) -> libc::c_int {
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: ssize_t = 0;
    let mut bits_checked: libc::c_int = 0;
    /* UNUSED */
    b = __archive_read_filter_ahead(filter, 8 as libc::c_int as size_t, &mut avail)
        as *const libc::c_uchar;
    if b.is_null() {
        return 0 as libc::c_int;
    }
    bits_checked = 0 as libc::c_int;
    /*
     * Verify Header Magic Bytes : 0XED 0XAB 0XEE 0XDB
     */
    if memcmp(
        b as *const libc::c_void,
        b"\xed\xab\xee\xdb\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    bits_checked += 32 as libc::c_int;
    /*
     * Check major version.
     */
    if *b.offset(4 as libc::c_int as isize) as libc::c_int != 3 as libc::c_int
        && *b.offset(4 as libc::c_int as isize) as libc::c_int != 4 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    bits_checked += 8 as libc::c_int;
    /*
     * Check package type; binary or source.
     */
    if *b.offset(6 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int;
    } /* not supported */
    bits_checked += 8 as libc::c_int;
    if *b.offset(7 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
        && *b.offset(7 as libc::c_int as isize) as libc::c_int != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    bits_checked += 8 as libc::c_int;
    return bits_checked;
}
unsafe extern "C" fn rpm_bidder_init(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut rpm: *mut rpm = 0 as *mut rpm;
    (*self_0).code = ARCHIVE_FILTER_RPM;
    (*self_0).name = b"rpm\x00" as *const u8 as *const libc::c_char;
    (*self_0).read = Some(
        rpm_filter_read
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
        Some(rpm_filter_close as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int);
    rpm = calloc(
        ::std::mem::size_of::<rpm>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut rpm;
    if rpm.is_null() {
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate data for rpm\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*self_0).data = rpm as *mut libc::c_void;
    (*rpm).state = ST_LEAD;
    return 0 as libc::c_int;
}
unsafe extern "C" fn rpm_filter_read(
    mut self_0: *mut archive_read_filter,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut rpm: *mut rpm = 0 as *mut rpm;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail_in: ssize_t = 0;
    let mut total: ssize_t = 0;
    let mut used: size_t = 0;
    let mut n: size_t = 0;
    let mut section: uint32_t = 0;
    let mut bytes: uint32_t = 0;
    rpm = (*self_0).data as *mut rpm;
    *buff = NULL as *const libc::c_void;
    avail_in = 0 as libc::c_int as ssize_t;
    total = avail_in;
    b = NULL as *const libc::c_uchar;
    used = 0 as libc::c_int as size_t;
    loop {
        if b.is_null() {
            b = __archive_read_filter_ahead(
                (*self_0).upstream,
                1 as libc::c_int as size_t,
                &mut avail_in,
            ) as *const libc::c_uchar;
            if b.is_null() {
                if !(avail_in < 0 as libc::c_int as libc::c_long) {
                    break;
                }
                return -(30 as libc::c_int) as ssize_t;
            }
        }
        match (*rpm).state as libc::c_uint {
            0 => {
                if (*rpm).total_in + avail_in < RPM_LEAD_SIZE as libc::c_long {
                    used = (used as libc::c_ulong).wrapping_add(avail_in as libc::c_ulong) as size_t
                        as size_t
                } else {
                    n = (RPM_LEAD_SIZE as libc::c_long - (*rpm).total_in) as size_t;
                    used = (used as libc::c_ulong).wrapping_add(n) as size_t as size_t;
                    b = b.offset(n as isize);
                    (*rpm).state = ST_HEADER;
                    (*rpm).hpos = 0 as libc::c_int as size_t;
                    (*rpm).hlen = 0 as libc::c_int as size_t;
                    (*rpm).first_header = 1 as libc::c_int
                }
            }
            1 => {
                n = (16 as libc::c_int as libc::c_ulong).wrapping_sub((*rpm).hpos);
                if n > (avail_in as libc::c_ulong).wrapping_sub(used) {
                    n = (avail_in as libc::c_ulong).wrapping_sub(used)
                }
                memcpy(
                    (*rpm).header.as_mut_ptr().offset((*rpm).hpos as isize) as *mut libc::c_void,
                    b as *const libc::c_void,
                    n,
                );
                b = b.offset(n as isize);
                used = (used as libc::c_ulong).wrapping_add(n) as size_t as size_t;
                (*rpm).hpos = ((*rpm).hpos as libc::c_ulong).wrapping_add(n) as size_t as size_t;
                if (*rpm).hpos == 16 as libc::c_int as libc::c_ulong {
                    if (*rpm).header[0 as libc::c_int as usize] as libc::c_int
                        != 0x8e as libc::c_int
                        || (*rpm).header[1 as libc::c_int as usize] as libc::c_int
                            != 0xad as libc::c_int
                        || (*rpm).header[2 as libc::c_int as usize] as libc::c_int
                            != 0xe8 as libc::c_int
                        || (*rpm).header[3 as libc::c_int as usize] as libc::c_int
                            != 0x1 as libc::c_int
                    {
                        if (*rpm).first_header != 0 {
                            archive_set_error(
                                &mut (*(*self_0).archive).archive as *mut archive,
                                ARCHIVE_ERRNO_FILE_FORMAT,
                                b"Unrecoginized rpm header\x00" as *const u8 as *const libc::c_char,
                            );
                            return -(30 as libc::c_int) as ssize_t;
                        }
                        (*rpm).state = ST_ARCHIVE;
                        *buff = (*rpm).header.as_mut_ptr() as *const libc::c_void;
                        total = (*rpm).hpos as ssize_t
                    } else {
                        /* Calculate 'Header' length. */
                        section = archive_be32dec(
                            (*rpm).header.as_mut_ptr().offset(8 as libc::c_int as isize)
                                as *const libc::c_void,
                        );
                        bytes = archive_be32dec(
                            (*rpm)
                                .header
                                .as_mut_ptr()
                                .offset(12 as libc::c_int as isize)
                                as *const libc::c_void,
                        );
                        (*rpm).hlen = (16 as libc::c_int as libc::c_uint)
                            .wrapping_add(section.wrapping_mul(16 as libc::c_int as libc::c_uint))
                            .wrapping_add(bytes) as size_t;
                        (*rpm).state = ST_HEADER_DATA;
                        (*rpm).first_header = 0 as libc::c_int
                    }
                }
            }
            2 => {
                n = (*rpm).hlen.wrapping_sub((*rpm).hpos);
                if n > (avail_in as libc::c_ulong).wrapping_sub(used) {
                    n = (avail_in as libc::c_ulong).wrapping_sub(used)
                }
                b = b.offset(n as isize);
                used = (used as libc::c_ulong).wrapping_add(n) as size_t as size_t;
                (*rpm).hpos = ((*rpm).hpos as libc::c_ulong).wrapping_add(n) as size_t as size_t;
                if (*rpm).hpos == (*rpm).hlen {
                    (*rpm).state = ST_PADDING
                }
            }
            3 => {
                while used < avail_in as size_t {
                    if *b as libc::c_int != 0 as libc::c_int {
                        /* Read next header. */
                        (*rpm).state = ST_HEADER;
                        (*rpm).hpos = 0 as libc::c_int as size_t;
                        (*rpm).hlen = 0 as libc::c_int as size_t;
                        break;
                    } else {
                        b = b.offset(1);
                        used = used.wrapping_add(1)
                    }
                }
            }
            4 => {
                *buff = b as *const libc::c_void;
                total = avail_in;
                used = avail_in as size_t
            }
            _ => {}
        }
        if used == avail_in as size_t {
            (*rpm).total_in =
                ((*rpm).total_in as libc::c_ulong).wrapping_add(used) as int64_t as int64_t;
            __archive_read_filter_consume((*self_0).upstream, used as int64_t);
            b = NULL as *const libc::c_uchar;
            used = 0 as libc::c_int as size_t
        }
        if !(total == 0 as libc::c_int as libc::c_long
            && avail_in > 0 as libc::c_int as libc::c_long)
        {
            break;
        }
    }
    if used > 0 as libc::c_int as libc::c_ulong && !b.is_null() {
        (*rpm).total_in =
            ((*rpm).total_in as libc::c_ulong).wrapping_add(used) as int64_t as int64_t;
        __archive_read_filter_consume((*self_0).upstream, used as int64_t);
    }
    return total;
}
unsafe extern "C" fn rpm_filter_close(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut rpm: *mut rpm = 0 as *mut rpm;
    rpm = (*self_0).data as *mut rpm;
    free(rpm as *mut libc::c_void);
    return 0 as libc::c_int;
}
