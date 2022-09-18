use ::libc;
extern "C" {
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
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
    /*
     * The magic/state values are used to sanity-check the
     * client's usage.  If an API function is called at a
     * ridiculous time, or the client passes us an invalid
     * pointer, these values allow me to catch that.
     */
    /*
     * Some public API functions depend on the "real" type of the
     * archive object.
     */
    /* Currently active compression. */
    /* Number of file entries processed. */
    /* Current ACP(ANSI CodePage). */
    /* Current OEMCP(OEM CodePage). */
    /*
     * Used by archive_read_data() to track blocks and copy
     * data to client buffers, filling gaps with zero bytes.
     */
    /*
     * Used by formats/filters to determine the amount of data
     * requested from a call to archive_read_data(). This is only
     * useful when the format/filter has seek support.
     */
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
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
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type la_int64_t = int64_t;
pub type la_ssize_t = ssize_t;
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
/* in bytes */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uudecode {
    pub total: int64_t,
    pub in_buff: *mut libc::c_uchar,
    pub in_cnt: libc::c_int,
    pub in_allocated: size_t,
    pub out_buff: *mut libc::c_uchar,
    pub state: libc::c_int,
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FILTER_UU: libc::c_int = 7 as libc::c_int;
/* Maximum lookahead during bid phase */
pub const UUENCODE_BID_MAX_READ: libc::c_int = 128 as libc::c_int * 1024 as libc::c_int;
pub const IN_BUFF_SIZE: libc::c_int = 1024 as libc::c_int;
pub const OUT_BUFF_SIZE: libc::c_int = 64 as libc::c_int * 1024 as libc::c_int;
pub const ST_FIND_HEAD: libc::c_int = 0 as libc::c_int;
pub const ST_READ_UU: libc::c_int = 1 as libc::c_int;
pub const ST_UUEND: libc::c_int = 2 as libc::c_int;
pub const ST_READ_BASE64: libc::c_int = 3 as libc::c_int;
pub const ST_IGNORE: libc::c_int = 4 as libc::c_int;
/* Deprecated; remove in libarchive 4.0 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_compression_uu(mut a: *mut archive) -> libc::c_int {
    return archive_read_support_filter_uu(a);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_uu(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_filter_uu\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if __archive_read_get_bidder(a, &mut bidder) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*bidder).data = NULL as *mut libc::c_void;
    (*bidder).name = b"uu\x00" as *const u8 as *const libc::c_char;
    (*bidder).bid = Some(
        uudecode_bidder_bid
            as unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *mut archive_read_filter,
            ) -> libc::c_int,
    );
    (*bidder).init = Some(
        uudecode_bidder_init as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int,
    );
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
static mut ascii: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    '\n' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    '\r' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut uuchar: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut base64: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
static mut base64num: [libc::c_int; 128] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    62 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    63 as libc::c_int,
    52 as libc::c_int,
    53 as libc::c_int,
    54 as libc::c_int,
    55 as libc::c_int,
    56 as libc::c_int,
    57 as libc::c_int,
    58 as libc::c_int,
    59 as libc::c_int,
    60 as libc::c_int,
    61 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    15 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
    20 as libc::c_int,
    21 as libc::c_int,
    22 as libc::c_int,
    23 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    26 as libc::c_int,
    27 as libc::c_int,
    28 as libc::c_int,
    29 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    32 as libc::c_int,
    33 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    38 as libc::c_int,
    39 as libc::c_int,
    40 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    44 as libc::c_int,
    45 as libc::c_int,
    46 as libc::c_int,
    47 as libc::c_int,
    48 as libc::c_int,
    49 as libc::c_int,
    50 as libc::c_int,
    51 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
unsafe extern "C" fn get_line(
    mut b: *const libc::c_uchar,
    mut avail: ssize_t,
    mut nlsize: *mut ssize_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    len = 0 as libc::c_int as ssize_t;
    while len < avail {
        let mut current_block_13: u64;
        match ascii[*b as usize] as libc::c_int {
            0 => {
                /* Non-ascii character or control character. */
                if !nlsize.is_null() {
                    *nlsize = 0 as libc::c_int as ssize_t
                }
                return -(1 as libc::c_int) as ssize_t;
            }
            13 => {
                if avail - len > 1 as libc::c_int as libc::c_long
                    && *b.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
                {
                    if !nlsize.is_null() {
                        *nlsize = 2 as libc::c_int as ssize_t
                    }
                    return len + 2 as libc::c_int as libc::c_long;
                }
                current_block_13 = 12103867617836757934;
            }
            10 => {
                current_block_13 = 12103867617836757934;
            }
            1 => {
                b = b.offset(1);
                len += 1;
                current_block_13 = 1054647088692577877;
            }
            _ => {
                current_block_13 = 1054647088692577877;
            }
        }
        match current_block_13 {
            1054647088692577877 => {}
            _ =>
            /* FALL THROUGH */
            {
                if !nlsize.is_null() {
                    *nlsize = 1 as libc::c_int as ssize_t
                }
                return len + 1 as libc::c_int as libc::c_long;
            }
        }
    }
    if !nlsize.is_null() {
        *nlsize = 0 as libc::c_int as ssize_t
    }
    return avail;
}
unsafe extern "C" fn bid_get_line(
    mut filter: *mut archive_read_filter,
    mut b: *mut *const libc::c_uchar,
    mut avail: *mut ssize_t,
    mut ravail: *mut ssize_t,
    mut nl: *mut ssize_t,
    mut nbytes_read: *mut size_t,
) -> ssize_t {
    let mut len: ssize_t = 0;
    let mut quit: libc::c_int = 0;
    quit = 0 as libc::c_int;
    if *avail == 0 as libc::c_int as libc::c_long {
        *nl = 0 as libc::c_int as ssize_t;
        len = 0 as libc::c_int as ssize_t
    } else {
        len = get_line(*b, *avail, nl)
    }
    /*
     * Read bytes more while it does not reach the end of line.
     */
    while *nl == 0 as libc::c_int as libc::c_long
        && len == *avail
        && quit == 0
        && *nbytes_read < UUENCODE_BID_MAX_READ as libc::c_ulong
    {
        let mut diff: ssize_t = *ravail - *avail;
        let mut nbytes_req: size_t = (*ravail + 1023 as libc::c_int as libc::c_long
            & !(1023 as libc::c_uint) as libc::c_long)
            as size_t;
        let mut tested: ssize_t = 0;
        /* Increase reading bytes if it is not enough to at least
         * new two lines. */
        if nbytes_req < (*ravail as size_t).wrapping_add(160 as libc::c_int as libc::c_ulong) {
            nbytes_req <<= 1 as libc::c_int
        }
        *b = __archive_read_filter_ahead(filter, nbytes_req, avail) as *const libc::c_uchar;
        if (*b).is_null() {
            if *ravail >= *avail {
                return 0 as libc::c_int as ssize_t;
            }
            /* Reading bytes reaches the end of a stream. */
            *b = __archive_read_filter_ahead(filter, *avail as size_t, avail)
                as *const libc::c_uchar; /* Skip some bytes we already determinated. */
            quit = 1 as libc::c_int
        }
        *nbytes_read = *avail as size_t;
        *ravail = *avail;
        *b = (*b).offset(diff as isize);
        *avail -= diff;
        tested = len;
        len = get_line((*b).offset(tested as isize), *avail - tested, nl);
        if len >= 0 as libc::c_int as libc::c_long {
            len += tested
        }
    }
    return len;
}
unsafe extern "C" fn uudecode_bidder_bid(
    mut self_0: *mut archive_read_filter_bidder,
    mut filter: *mut archive_read_filter,
) -> libc::c_int {
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: ssize_t = 0;
    let mut ravail: ssize_t = 0;
    let mut len: ssize_t = 0;
    let mut nl: ssize_t = 0;
    let mut l: libc::c_int = 0;
    let mut firstline: libc::c_int = 0;
    let mut nbytes_read: size_t = 0;
    /* UNUSED */
    b = __archive_read_filter_ahead(filter, 1 as libc::c_int as size_t, &mut avail)
        as *const libc::c_uchar; /* No match found. */
    if b.is_null() {
        return 0 as libc::c_int;
    }
    firstline = 20 as libc::c_int;
    ravail = avail;
    nbytes_read = avail as size_t;
    loop {
        len = bid_get_line(
            filter,
            &mut b,
            &mut avail,
            &mut ravail,
            &mut nl,
            &mut nbytes_read,
        );
        if len < 0 as libc::c_int as libc::c_long || nl == 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        if len - nl >= 11 as libc::c_int as libc::c_long
            && memcmp(
                b as *const libc::c_void,
                b"begin \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            l = 6 as libc::c_int
        } else if len - nl >= 18 as libc::c_int as libc::c_long
            && memcmp(
                b as *const libc::c_void,
                b"begin-base64 \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                13 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            l = 13 as libc::c_int
        } else {
            l = 0 as libc::c_int
        }
        if l > 0 as libc::c_int
            && ((*b.offset(l as isize) as libc::c_int) < '0' as i32
                || *b.offset(l as isize) as libc::c_int > '7' as i32
                || (*b.offset((l + 1 as libc::c_int) as isize) as libc::c_int) < '0' as i32
                || *b.offset((l + 1 as libc::c_int) as isize) as libc::c_int > '7' as i32
                || (*b.offset((l + 2 as libc::c_int) as isize) as libc::c_int) < '0' as i32
                || *b.offset((l + 2 as libc::c_int) as isize) as libc::c_int > '7' as i32
                || *b.offset((l + 3 as libc::c_int) as isize) as libc::c_int != ' ' as i32)
        {
            l = 0 as libc::c_int
        }
        b = b.offset(len as isize);
        avail -= len;
        if l != 0 {
            break;
        }
        firstline = 0 as libc::c_int;
        /* Do not read more than UUENCODE_BID_MAX_READ bytes */
        if nbytes_read >= UUENCODE_BID_MAX_READ as libc::c_ulong {
            return 0 as libc::c_int;
        }
    } /* There are non-ascii characters. */
    if avail == 0 {
        return 0 as libc::c_int;
    }
    len = bid_get_line(
        filter,
        &mut b,
        &mut avail,
        &mut ravail,
        &mut nl,
        &mut nbytes_read,
    );
    if len < 0 as libc::c_int as libc::c_long || nl == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    avail -= len;
    if l == 6 as libc::c_int {
        /* "begin " */
        if uuchar[*b as usize] == 0 {
            return 0 as libc::c_int;
        }
        /* Get a length of decoded bytes. */
        let fresh0 = b;
        b = b.offset(1);
        l = *fresh0 as libc::c_int - 0x20 as libc::c_int & 0x3f as libc::c_int;
        len -= 1;
        if l > 45 as libc::c_int {
            /* Normally, maximum length is 45(character 'M'). */
            return 0 as libc::c_int;
        } /* Line too short. */
        if l as libc::c_long > len - nl {
            return 0 as libc::c_int;
        }
        while l != 0 {
            let fresh1 = b;
            b = b.offset(1);
            if uuchar[*fresh1 as usize] == 0 {
                return 0 as libc::c_int;
            }
            len -= 1;
            l -= 1
        }
        if len - nl == 1 as libc::c_int as libc::c_long
            && (uuchar[*b as usize] as libc::c_int != 0
                || *b as libc::c_int >= 'a' as i32 && *b as libc::c_int <= 'z' as i32)
        {
            /* Padding data(MINIX). */
            b = b.offset(1);
            len -= 1
        }
        b = b.offset(nl as isize);
        if avail != 0 && uuchar[*b as usize] as libc::c_int != 0 {
            return firstline + 30 as libc::c_int;
        }
    } else if l == 13 as libc::c_int {
        /* "begin-base64 " */
        while len - nl > 0 as libc::c_int as libc::c_long {
            let fresh2 = b; /* not supported */
            b = b.offset(1);
            if base64[*fresh2 as usize] == 0 {
                return 0 as libc::c_int;
            }
            len -= 1
        }
        b = b.offset(nl as isize);
        if avail >= 5 as libc::c_int as libc::c_long
            && memcmp(
                b as *const libc::c_void,
                b"====\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            return firstline + 40 as libc::c_int;
        }
        if avail >= 6 as libc::c_int as libc::c_long
            && memcmp(
                b as *const libc::c_void,
                b"====\r\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            return firstline + 40 as libc::c_int;
        }
        if avail > 0 as libc::c_int as libc::c_long && base64[*b as usize] as libc::c_int != 0 {
            return firstline + 30 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn uudecode_bidder_init(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut uudecode: *mut uudecode = 0 as *mut uudecode;
    let mut out_buff: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut in_buff: *mut libc::c_void = 0 as *mut libc::c_void;
    (*self_0).code = ARCHIVE_FILTER_UU;
    (*self_0).name = b"uu\x00" as *const u8 as *const libc::c_char;
    (*self_0).read = Some(
        uudecode_filter_read
            as unsafe extern "C" fn(
                _: *mut archive_read_filter,
                _: *mut *const libc::c_void,
            ) -> ssize_t,
    );
    (*self_0).skip = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut archive_read_filter, _: int64_t) -> int64_t>,
    >(NULL as libc::intptr_t);
    (*self_0).close = Some(
        uudecode_filter_close as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int,
    );
    uudecode = calloc(
        ::std::mem::size_of::<uudecode>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut uudecode;
    out_buff = malloc(OUT_BUFF_SIZE as libc::c_ulong);
    in_buff = malloc(IN_BUFF_SIZE as libc::c_ulong);
    if uudecode.is_null() || out_buff.is_null() || in_buff.is_null() {
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate data for uudecode\x00" as *const u8 as *const libc::c_char,
        );
        free(uudecode as *mut libc::c_void);
        free(out_buff);
        free(in_buff);
        return -(30 as libc::c_int);
    }
    (*self_0).data = uudecode as *mut libc::c_void;
    (*uudecode).in_buff = in_buff as *mut libc::c_uchar;
    (*uudecode).in_cnt = 0 as libc::c_int;
    (*uudecode).in_allocated = IN_BUFF_SIZE as size_t;
    (*uudecode).out_buff = out_buff as *mut libc::c_uchar;
    (*uudecode).state = ST_FIND_HEAD;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ensure_in_buff_size(
    mut self_0: *mut archive_read_filter,
    mut uudecode: *mut uudecode,
    mut size: size_t,
) -> libc::c_int {
    if size > (*uudecode).in_allocated {
        let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut newsize: size_t = 0;
        /*
         * Calculate a new buffer size for in_buff.
         * Increase its value until it has enough size we need.
         */
        newsize = (*uudecode).in_allocated;
        loop {
            if newsize < (IN_BUFF_SIZE * 32 as libc::c_int) as libc::c_ulong {
                newsize <<= 1 as libc::c_int
            } else {
                newsize = (newsize as libc::c_ulong).wrapping_add(IN_BUFF_SIZE as libc::c_ulong)
                    as size_t as size_t
            }
            if !(size > newsize) {
                break;
            }
        }
        /* Allocate the new buffer. */
        ptr = malloc(newsize) as *mut libc::c_uchar;
        if ptr.is_null() {
            free(ptr as *mut libc::c_void);
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate data for uudecode\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Move the remaining data in in_buff into the new buffer. */
        if (*uudecode).in_cnt != 0 {
            memmove(
                ptr as *mut libc::c_void,
                (*uudecode).in_buff as *const libc::c_void,
                (*uudecode).in_cnt as libc::c_ulong,
            );
        }
        /* Replace in_buff with the new buffer. */
        free((*uudecode).in_buff as *mut libc::c_void);
        (*uudecode).in_buff = ptr;
        (*uudecode).in_allocated = newsize
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn uudecode_filter_read(
    mut self_0: *mut archive_read_filter,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut l: int64_t = 0;
    let mut body: int64_t = 0;
    let mut current_block: u64;
    let mut uudecode: *mut uudecode = 0 as *mut uudecode;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut d: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut avail_in: ssize_t = 0;
    let mut ravail: ssize_t = 0;
    let mut used: ssize_t = 0;
    let mut total: ssize_t = 0;
    let mut len: ssize_t = 0;
    let mut llen: ssize_t = 0;
    let mut nl: ssize_t = 0;
    uudecode = (*self_0).data as *mut uudecode;
    'c_3762: loop {
        d = __archive_read_filter_ahead(
            (*self_0).upstream,
            1 as libc::c_int as size_t,
            &mut avail_in,
        ) as *const libc::c_uchar;
        if d.is_null() && avail_in < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int) as ssize_t;
        }
        /* Quiet a code analyzer; make sure avail_in must be zero
         * when d is NULL. */
        if d.is_null() {
            avail_in = 0 as libc::c_int as ssize_t
        }
        used = 0 as libc::c_int as ssize_t;
        total = 0 as libc::c_int as ssize_t;
        out = (*uudecode).out_buff;
        ravail = avail_in;
        if (*uudecode).state == ST_IGNORE {
            used = avail_in;
            current_block = 4053532438579145105;
            break;
        } else {
            if (*uudecode).in_cnt != 0 {
                /*
                 * If there is remaining data which is saved by
                 * previous calling, use it first.
                 */
                if ensure_in_buff_size(
                    self_0,
                    uudecode,
                    (avail_in + (*uudecode).in_cnt as libc::c_long) as size_t,
                ) != ARCHIVE_OK
                {
                    return -(30 as libc::c_int) as ssize_t;
                }
                memcpy(
                    (*uudecode).in_buff.offset((*uudecode).in_cnt as isize) as *mut libc::c_void,
                    d as *const libc::c_void,
                    avail_in as libc::c_ulong,
                );
                d = (*uudecode).in_buff;
                avail_in += (*uudecode).in_cnt as libc::c_long;
                (*uudecode).in_cnt = 0 as libc::c_int
            }
            loop {
                if !(used < avail_in) {
                    current_block = 4053532438579145105;
                    break 'c_3762;
                }
                l = 0;
                body = 0;
                b = d;
                len = get_line(b, avail_in - used, &mut nl);
                if len < 0 as libc::c_int as libc::c_long {
                    /* Non-ascii character is found. */
                    if (*uudecode).state == ST_FIND_HEAD
                        && ((*uudecode).total > 0 as libc::c_int as libc::c_long
                            || total > 0 as libc::c_int as libc::c_long)
                    {
                        current_block = 14818589718467733107;
                        break 'c_3762;
                    } else {
                        current_block = 5689316957504528238;
                        break 'c_3762;
                    }
                } else {
                    llen = len;
                    if nl == 0 as libc::c_int as libc::c_long && (*uudecode).state != ST_UUEND {
                        if total == 0 as libc::c_int as libc::c_long
                            && ravail <= 0 as libc::c_int as libc::c_long
                        {
                            /* There is nothing more to read, fail */
                            archive_set_error(
                                &mut (*(*self_0).archive).archive as *mut archive,
                                ARCHIVE_ERRNO_FILE_FORMAT,
                                b"Missing format data\x00" as *const u8 as *const libc::c_char,
                            );
                            return -(30 as libc::c_int) as ssize_t;
                        }
                        /*
                         * Save remaining data which does not contain
                         * NL('\n','\r').
                         */
                        if ensure_in_buff_size(self_0, uudecode, len as size_t) != ARCHIVE_OK {
                            return -(30 as libc::c_int) as ssize_t;
                        }
                        if (*uudecode).in_buff != b as *mut libc::c_uchar {
                            memmove(
                                (*uudecode).in_buff as *mut libc::c_void,
                                b as *const libc::c_void,
                                len as libc::c_ulong,
                            );
                        }
                        (*uudecode).in_cnt = len as libc::c_int;
                        if total == 0 as libc::c_int as libc::c_long {
                            current_block = 18435049525520518667;
                            break;
                        } else {
                            current_block = 12997042908615822766;
                            break;
                        }
                    } else {
                        match (*uudecode).state {
                            ST_READ_UU => {
                                if total + len * 2 as libc::c_int as libc::c_long
                                    > OUT_BUFF_SIZE as libc::c_long
                                {
                                    current_block = 4053532438579145105;
                                    break 'c_3762;
                                }
                                body = len - nl;
                                if uuchar[*b as usize] == 0
                                    || body <= 0 as libc::c_int as libc::c_long
                                {
                                    archive_set_error(
                                        &mut (*(*self_0).archive).archive as *mut archive,
                                        ARCHIVE_ERRNO_MISC,
                                        b"Insufficient compressed data\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return -(30 as libc::c_int) as ssize_t;
                                }
                                /* Get length of undecoded bytes of current line. */
                                let fresh3 = b;
                                b = b.offset(1);
                                l = (*fresh3 as libc::c_int - 0x20 as libc::c_int
                                    & 0x3f as libc::c_int)
                                    as int64_t;
                                body -= 1;
                                if l > body {
                                    archive_set_error(
                                        &mut (*(*self_0).archive).archive as *mut archive,
                                        ARCHIVE_ERRNO_MISC,
                                        b"Insufficient compressed data\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return -(30 as libc::c_int) as ssize_t;
                                }
                                if l == 0 as libc::c_int as libc::c_long {
                                    (*uudecode).state = ST_UUEND
                                } else {
                                    while l > 0 as libc::c_int as libc::c_long {
                                        let mut n: libc::c_int = 0 as libc::c_int;
                                        if uuchar[*b.offset(0 as libc::c_int as isize) as usize]
                                            == 0
                                            || uuchar[*b.offset(1 as libc::c_int as isize) as usize]
                                                == 0
                                        {
                                            break;
                                        }
                                        let fresh4 = b;
                                        b = b.offset(1);
                                        n = (*fresh4 as libc::c_int - 0x20 as libc::c_int
                                            & 0x3f as libc::c_int)
                                            << 18 as libc::c_int;
                                        let fresh5 = b;
                                        b = b.offset(1);
                                        n |= (*fresh5 as libc::c_int - 0x20 as libc::c_int
                                            & 0x3f as libc::c_int)
                                            << 12 as libc::c_int;
                                        let fresh6 = out;
                                        out = out.offset(1);
                                        *fresh6 = (n >> 16 as libc::c_int) as libc::c_uchar;
                                        total += 1;
                                        l -= 1;
                                        if l > 0 as libc::c_int as libc::c_long {
                                            if uuchar[*b.offset(0 as libc::c_int as isize) as usize]
                                                == 0
                                            {
                                                break;
                                            }
                                            let fresh7 = b;
                                            b = b.offset(1);
                                            n |= (*fresh7 as libc::c_int - 0x20 as libc::c_int
                                                & 0x3f as libc::c_int)
                                                << 6 as libc::c_int;
                                            let fresh8 = out;
                                            out = out.offset(1);
                                            *fresh8 = (n >> 8 as libc::c_int & 0xff as libc::c_int)
                                                as libc::c_uchar;
                                            total += 1;
                                            l -= 1
                                        }
                                        if !(l > 0 as libc::c_int as libc::c_long) {
                                            continue;
                                        }
                                        if uuchar[*b.offset(0 as libc::c_int as isize) as usize]
                                            == 0
                                        {
                                            break;
                                        }
                                        let fresh9 = b;
                                        b = b.offset(1);
                                        n |= *fresh9 as libc::c_int - 0x20 as libc::c_int
                                            & 0x3f as libc::c_int;
                                        let fresh10 = out;
                                        out = out.offset(1);
                                        *fresh10 = (n & 0xff as libc::c_int) as libc::c_uchar;
                                        total += 1;
                                        l -= 1
                                    }
                                    if l != 0 {
                                        archive_set_error(
                                            &mut (*(*self_0).archive).archive as *mut archive,
                                            ARCHIVE_ERRNO_MISC,
                                            b"Insufficient compressed data\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int) as ssize_t;
                                    }
                                }
                            }
                            ST_UUEND => {
                                if len - nl == 3 as libc::c_int as libc::c_long
                                    && memcmp(
                                        b as *const libc::c_void,
                                        b"end \x00" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        3 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int
                                {
                                    (*uudecode).state = ST_FIND_HEAD
                                } else {
                                    archive_set_error(
                                        &mut (*(*self_0).archive).archive as *mut archive,
                                        ARCHIVE_ERRNO_MISC,
                                        b"Insufficient compressed data\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return -(30 as libc::c_int) as ssize_t;
                                }
                            }
                            ST_READ_BASE64 => {
                                if total + len * 2 as libc::c_int as libc::c_long
                                    > OUT_BUFF_SIZE as libc::c_long
                                {
                                    current_block = 4053532438579145105;
                                    break 'c_3762;
                                }
                                l = len - nl;
                                if l >= 3 as libc::c_int as libc::c_long
                                    && *b.offset(0 as libc::c_int as isize) as libc::c_int
                                        == '=' as i32
                                    && *b.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '=' as i32
                                    && *b.offset(2 as libc::c_int as isize) as libc::c_int
                                        == '=' as i32
                                {
                                    (*uudecode).state = ST_FIND_HEAD
                                } else {
                                    while l > 0 as libc::c_int as libc::c_long {
                                        let mut n_0: libc::c_int = 0 as libc::c_int;
                                        if base64[*b.offset(0 as libc::c_int as isize) as usize]
                                            == 0
                                            || base64[*b.offset(1 as libc::c_int as isize) as usize]
                                                == 0
                                        {
                                            break;
                                        }
                                        let fresh11 = b;
                                        b = b.offset(1);
                                        n_0 = base64num[*fresh11 as usize] << 18 as libc::c_int;
                                        let fresh12 = b;
                                        b = b.offset(1);
                                        n_0 |= base64num[*fresh12 as usize] << 12 as libc::c_int;
                                        let fresh13 = out;
                                        out = out.offset(1);
                                        *fresh13 = (n_0 >> 16 as libc::c_int) as libc::c_uchar;
                                        total += 1;
                                        l -= 2 as libc::c_int as libc::c_long;
                                        if l > 0 as libc::c_int as libc::c_long {
                                            if *b as libc::c_int == '=' as i32 {
                                                break;
                                            }
                                            if base64[*b as usize] == 0 {
                                                break;
                                            }
                                            let fresh14 = b;
                                            b = b.offset(1);
                                            n_0 |= base64num[*fresh14 as usize] << 6 as libc::c_int;
                                            let fresh15 = out;
                                            out = out.offset(1);
                                            *fresh15 = (n_0 >> 8 as libc::c_int
                                                & 0xff as libc::c_int)
                                                as libc::c_uchar;
                                            total += 1;
                                            l -= 1
                                        }
                                        if !(l > 0 as libc::c_int as libc::c_long) {
                                            continue;
                                        }
                                        if *b as libc::c_int == '=' as i32 {
                                            break;
                                        }
                                        if base64[*b as usize] == 0 {
                                            break;
                                        }
                                        let fresh16 = b;
                                        b = b.offset(1);
                                        n_0 |= base64num[*fresh16 as usize];
                                        let fresh17 = out;
                                        out = out.offset(1);
                                        *fresh17 = (n_0 & 0xff as libc::c_int) as libc::c_uchar;
                                        total += 1;
                                        l -= 1
                                    }
                                    if l != 0 && *b as libc::c_int != '=' as i32 {
                                        archive_set_error(
                                            &mut (*(*self_0).archive).archive as *mut archive,
                                            ARCHIVE_ERRNO_MISC,
                                            b"Insufficient compressed data\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int) as ssize_t;
                                    }
                                }
                            }
                            ST_FIND_HEAD | _ => {
                                /* Do not read more than UUENCODE_BID_MAX_READ bytes */
                                if total + len >= UUENCODE_BID_MAX_READ as libc::c_long {
                                    archive_set_error(
                                        &mut (*(*self_0).archive).archive as *mut archive,
                                        ARCHIVE_ERRNO_FILE_FORMAT,
                                        b"Invalid format data\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return -(30 as libc::c_int) as ssize_t;
                                }
                                if len - nl >= 11 as libc::c_int as libc::c_long
                                    && memcmp(
                                        b as *const libc::c_void,
                                        b"begin \x00" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        6 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int
                                {
                                    l = 6 as libc::c_int as int64_t
                                } else if len - nl >= 18 as libc::c_int as libc::c_long
                                    && memcmp(
                                        b as *const libc::c_void,
                                        b"begin-base64 \x00" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        13 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int
                                {
                                    l = 13 as libc::c_int as int64_t
                                } else {
                                    l = 0 as libc::c_int as int64_t
                                }
                                if l != 0 as libc::c_int as libc::c_long
                                    && *b.offset(l as isize) as libc::c_int >= '0' as i32
                                    && *b.offset(l as isize) as libc::c_int <= '7' as i32
                                    && *b.offset((l + 1 as libc::c_int as libc::c_long) as isize)
                                        as libc::c_int
                                        >= '0' as i32
                                    && *b.offset((l + 1 as libc::c_int as libc::c_long) as isize)
                                        as libc::c_int
                                        <= '7' as i32
                                    && *b.offset((l + 2 as libc::c_int as libc::c_long) as isize)
                                        as libc::c_int
                                        >= '0' as i32
                                    && *b.offset((l + 2 as libc::c_int as libc::c_long) as isize)
                                        as libc::c_int
                                        <= '7' as i32
                                    && *b.offset((l + 3 as libc::c_int as libc::c_long) as isize)
                                        as libc::c_int
                                        == ' ' as i32
                                {
                                    if l == 6 as libc::c_int as libc::c_long {
                                        (*uudecode).state = ST_READ_UU
                                    } else {
                                        (*uudecode).state = ST_READ_BASE64
                                    }
                                }
                            }
                        }
                        d = d.offset(llen as isize);
                        used += llen
                    }
                }
            }
            match current_block {
                12997042908615822766 => {
                    used += len;
                    current_block = 4053532438579145105;
                    break;
                }
                _ => {
                    /* Do not return 0; it means end-of-file.
                     * We should try to read bytes more. */
                    __archive_read_filter_consume((*self_0).upstream, ravail);
                }
            }
        }
    }
    match current_block {
        14818589718467733107 => {
            (*uudecode).state = ST_IGNORE;
            used = avail_in
        }
        5689316957504528238 => {
            archive_set_error(
                &mut (*(*self_0).archive).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Insufficient compressed data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        _ => {}
    }
    if ravail < avail_in {
        used -= avail_in - ravail
    }
    __archive_read_filter_consume((*self_0).upstream, used);
    *buff = (*uudecode).out_buff as *const libc::c_void;
    (*uudecode).total += total;
    return total;
}
unsafe extern "C" fn uudecode_filter_close(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut uudecode: *mut uudecode = 0 as *mut uudecode;
    uudecode = (*self_0).data as *mut uudecode;
    free((*uudecode).in_buff as *mut libc::c_void);
    free((*uudecode).out_buff as *mut libc::c_void);
    free(uudecode as *mut libc::c_void);
    return 0 as libc::c_int;
}
