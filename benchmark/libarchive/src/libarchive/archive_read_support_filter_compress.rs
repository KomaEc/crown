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
pub type __uint16_t = libc::c_ushort;
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
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
/*
 * Because LZW decompression is pretty simple, I've just implemented
 * the whole decompressor here (cribbing from "compress" source code,
 * of course), rather than relying on an external library.  I have
 * made an effort to clarify and simplify the algorithm, so the
 * names and structure here don't exactly match those used by compress.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct private_data {
    pub next_in: *const libc::c_uchar,
    pub avail_in: size_t,
    pub consume_unnotified: size_t,
    pub bit_buffer: libc::c_int,
    pub bits_avail: libc::c_int,
    pub bytes_in_section: size_t,
    pub out_block_size: size_t,
    pub out_block: *mut libc::c_void,
    pub use_reset_code: libc::c_int,
    pub end_of_stream: libc::c_int,
    pub maxcode: libc::c_int,
    pub maxcode_bits: libc::c_int,
    pub section_end_code: libc::c_int,
    pub bits: libc::c_int,
    pub oldcode: libc::c_int,
    pub finbyte: libc::c_int,
    pub free_ent: libc::c_int,
    pub suffix: [libc::c_uchar; 65536],
    pub prefix: [uint16_t; 65536],
    pub stackp: *mut libc::c_uchar,
    pub stack: [libc::c_uchar; 65300],
}
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FILTER_COMPRESS: libc::c_int = 3 as libc::c_int;
/* Deprecated; remove in libarchive 4.0 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_compression_compress(
    mut a: *mut archive,
) -> libc::c_int {
    return archive_read_support_filter_compress(a);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_compress(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_filter_compress\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if __archive_read_get_bidder(a, &mut bidder) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*bidder).data = NULL as *mut libc::c_void;
    (*bidder).name = b"compress (.Z)\x00" as *const u8 as *const libc::c_char;
    (*bidder).bid = Some(
        compress_bidder_bid
            as unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *mut archive_read_filter,
            ) -> libc::c_int,
    );
    (*bidder).init = Some(
        compress_bidder_init as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int,
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
    (*bidder).free = Some(
        compress_bidder_free
            as unsafe extern "C" fn(_: *mut archive_read_filter_bidder) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
/*
 * Test whether we can handle this data.
 * This logic returns zero if any part of the signature fails.
 */
unsafe extern "C" fn compress_bidder_bid(
    mut self_0: *mut archive_read_filter_bidder,
    mut filter: *mut archive_read_filter,
) -> libc::c_int {
    let mut buffer: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: ssize_t = 0;
    let mut bits_checked: libc::c_int = 0;
    /* UNUSED */
    /* Shortest valid compress file is 3 bytes. */
    buffer = __archive_read_filter_ahead(filter, 3 as libc::c_int as size_t, &mut avail)
        as *const libc::c_uchar;
    if buffer.is_null() {
        return 0 as libc::c_int;
    }
    bits_checked = 0 as libc::c_int;
    /* First two bytes are the magic value */
    if *buffer.offset(0 as libc::c_int as isize) as libc::c_int != 0x1f as libc::c_int
        || *buffer.offset(1 as libc::c_int as isize) as libc::c_int != 0x9d as libc::c_int
    {
        return 0 as libc::c_int;
    }
    /* Third byte holds compression parameters. */
    if *buffer.offset(2 as libc::c_int as isize) as libc::c_int & 0x20 as libc::c_int != 0 {
        /* Reserved bit, must be zero. */
        return 0 as libc::c_int;
    }
    if *buffer.offset(2 as libc::c_int as isize) as libc::c_int & 0x40 as libc::c_int != 0 {
        /* Reserved bit, must be zero. */
        return 0 as libc::c_int;
    }
    bits_checked += 18 as libc::c_int;
    return bits_checked;
}
/*
 * Setup the callbacks.
 */
unsafe extern "C" fn compress_bidder_init(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut private_data = 0 as *mut private_data; /* not supported */
    static mut out_block_size: size_t = (64 as libc::c_int * 1024 as libc::c_int) as size_t;
    let mut out_block: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut code: libc::c_int = 0;
    (*self_0).code = ARCHIVE_FILTER_COMPRESS;
    (*self_0).name = b"compress (.Z)\x00" as *const u8 as *const libc::c_char;
    state = calloc(
        ::std::mem::size_of::<private_data>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut private_data;
    out_block = malloc(out_block_size);
    if state.is_null() || out_block.is_null() {
        free(out_block);
        free(state as *mut libc::c_void);
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate data for %s decompression\x00" as *const u8 as *const libc::c_char,
            (*self_0).name,
        );
        return -(30 as libc::c_int);
    }
    (*self_0).data = state as *mut libc::c_void;
    (*state).out_block_size = out_block_size;
    (*state).out_block = out_block;
    (*self_0).read = Some(
        compress_filter_read
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
        compress_filter_close as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int,
    );
    /* XXX MOVE THE FOLLOWING OUT OF INIT() XXX */
    getbits(self_0, 8 as libc::c_int); /* Skip first signature byte. */
    getbits(self_0, 8 as libc::c_int); /* Skip second signature byte. */
    /* Get compression parameters. */
    code = getbits(self_0, 8 as libc::c_int);
    if code & 0x1f as libc::c_int > 16 as libc::c_int {
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            -(1 as libc::c_int),
            b"Invalid compressed data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*state).maxcode_bits = code & 0x1f as libc::c_int;
    (*state).maxcode = (1 as libc::c_int) << (*state).maxcode_bits;
    (*state).use_reset_code = code & 0x80 as libc::c_int;
    /* Initialize decompressor. */
    (*state).free_ent = 256 as libc::c_int;
    (*state).stackp = (*state).stack.as_mut_ptr();
    if (*state).use_reset_code != 0 {
        (*state).free_ent += 1
    }
    (*state).bits = 9 as libc::c_int;
    (*state).section_end_code = ((1 as libc::c_int) << (*state).bits) - 1 as libc::c_int;
    (*state).oldcode = -(1 as libc::c_int);
    code = 255 as libc::c_int;
    while code >= 0 as libc::c_int {
        (*state).prefix[code as usize] = 0 as libc::c_int as uint16_t;
        (*state).suffix[code as usize] = code as libc::c_uchar;
        code -= 1
    }
    next_code(self_0);
    return 0 as libc::c_int;
}
/*
 * Return a block of data from the decompression buffer.  Decompress more
 * as necessary.
 */
unsafe extern "C" fn compress_filter_read(
    mut self_0: *mut archive_read_filter,
    mut pblock: *mut *const libc::c_void,
) -> ssize_t {
    let mut state: *mut private_data = 0 as *mut private_data;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut start: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    state = (*self_0).data as *mut private_data;
    if (*state).end_of_stream != 0 {
        *pblock = NULL as *const libc::c_void;
        return 0 as libc::c_int as ssize_t;
    }
    start = (*state).out_block as *mut libc::c_uchar;
    p = start;
    end = start.offset((*state).out_block_size as isize);
    while p < end && (*state).end_of_stream == 0 {
        if (*state).stackp > (*state).stack.as_mut_ptr() {
            (*state).stackp = (*state).stackp.offset(-1);
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = *(*state).stackp
        } else {
            ret = next_code(self_0);
            if ret == -(1 as libc::c_int) {
                (*state).end_of_stream = ret
            } else if ret != ARCHIVE_OK {
                return ret as ssize_t;
            }
        }
    }
    *pblock = start as *const libc::c_void;
    return p.offset_from(start) as libc::c_long;
}
/*
 * Clean up the reader.
 */
unsafe extern "C" fn compress_bidder_free(
    mut self_0: *mut archive_read_filter_bidder,
) -> libc::c_int {
    (*self_0).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
 * Close and release the filter.
 */
unsafe extern "C" fn compress_filter_close(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut private_data = (*self_0).data as *mut private_data;
    free((*state).out_block);
    free(state as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*
 * Process the next code and fill the stack with the expansion
 * of the code.  Returns ARCHIVE_FATAL if there is a fatal I/O or
 * format error, ARCHIVE_EOF if we hit end of data, ARCHIVE_OK otherwise.
 */
unsafe extern "C" fn next_code(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut private_data = (*self_0).data as *mut private_data;
    let mut code: libc::c_int = 0;
    let mut newcode: libc::c_int = 0;
    static mut debug_buff: [libc::c_int; 1024] = [0; 1024];
    static mut debug_index: libc::c_uint = 0;
    newcode = getbits(self_0, (*state).bits);
    code = newcode;
    if code < 0 as libc::c_int {
        return code;
    }
    let fresh1 = debug_index;
    debug_index = debug_index.wrapping_add(1);
    debug_buff[fresh1 as usize] = code;
    if debug_index as libc::c_ulong
        >= (::std::mem::size_of::<[libc::c_int; 1024]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        debug_index = 0 as libc::c_int as libc::c_uint
    }
    /* If it's a reset code, reset the dictionary. */
    if code == 256 as libc::c_int && (*state).use_reset_code != 0 {
        /*
         * The original 'compress' implementation blocked its
         * I/O in a manner that resulted in junk bytes being
         * inserted after every reset.  The next section skips
         * this junk.  (Yes, the number of *bytes* to skip is
         * a function of the current *bit* length.)
         */
        let mut skip_bytes: libc::c_int = ((*state).bits as libc::c_ulong).wrapping_sub(
            (*state)
                .bytes_in_section
                .wrapping_rem((*state).bits as libc::c_ulong),
        ) as libc::c_int; /* Discard rest of this byte. */
        skip_bytes %= (*state).bits;
        (*state).bits_avail = 0 as libc::c_int;
        loop {
            let fresh2 = skip_bytes;
            skip_bytes = skip_bytes - 1;
            if !(fresh2 > 0 as libc::c_int) {
                break;
            }
            code = getbits(self_0, 8 as libc::c_int);
            if code < 0 as libc::c_int {
                return code;
            }
        }
        /* Now, actually do the reset. */
        (*state).bytes_in_section = 0 as libc::c_int as size_t;
        (*state).bits = 9 as libc::c_int;
        (*state).section_end_code = ((1 as libc::c_int) << (*state).bits) - 1 as libc::c_int;
        (*state).free_ent = 257 as libc::c_int;
        (*state).oldcode = -(1 as libc::c_int);
        return next_code(self_0);
    }
    if code > (*state).free_ent || code == (*state).free_ent && (*state).oldcode < 0 as libc::c_int
    {
        /* An invalid code is a fatal error. */
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            -(1 as libc::c_int),
            b"Invalid compressed data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Special case for KwKwK string. */
    if code >= (*state).free_ent {
        let fresh3 = (*state).stackp;
        (*state).stackp = (*state).stackp.offset(1);
        *fresh3 = (*state).finbyte as libc::c_uchar;
        code = (*state).oldcode
    }
    /* Generate output characters in reverse order. */
    while code >= 256 as libc::c_int {
        let fresh4 = (*state).stackp;
        (*state).stackp = (*state).stackp.offset(1);
        *fresh4 = (*state).suffix[code as usize];
        code = (*state).prefix[code as usize] as libc::c_int
    }
    (*state).finbyte = code;
    let fresh5 = (*state).stackp;
    (*state).stackp = (*state).stackp.offset(1);
    *fresh5 = (*state).finbyte as libc::c_uchar;
    /* Generate the new entry. */
    code = (*state).free_ent;
    if code < (*state).maxcode && (*state).oldcode >= 0 as libc::c_int {
        (*state).prefix[code as usize] = (*state).oldcode as uint16_t;
        (*state).suffix[code as usize] = (*state).finbyte as libc::c_uchar;
        (*state).free_ent += 1
    }
    if (*state).free_ent > (*state).section_end_code {
        (*state).bits += 1;
        (*state).bytes_in_section = 0 as libc::c_int as size_t;
        if (*state).bits == (*state).maxcode_bits {
            (*state).section_end_code = (*state).maxcode
        } else {
            (*state).section_end_code = ((1 as libc::c_int) << (*state).bits) - 1 as libc::c_int
        }
    }
    /* Remember previous code. */
    (*state).oldcode = newcode;
    return 0 as libc::c_int;
}
/*
 * Return next 'n' bits from stream.
 *
 * -1 indicates end of available data.
 */
unsafe extern "C" fn getbits(
    mut self_0: *mut archive_read_filter,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut state: *mut private_data = (*self_0).data as *mut private_data;
    let mut code: libc::c_int = 0;
    let mut ret: ssize_t = 0;
    static mut mask: [libc::c_int; 17] = [
        0 as libc::c_int,
        0x1 as libc::c_int,
        0x3 as libc::c_int,
        0x7 as libc::c_int,
        0xf as libc::c_int,
        0x1f as libc::c_int,
        0x3f as libc::c_int,
        0x7f as libc::c_int,
        0xff as libc::c_int,
        0x1ff as libc::c_int,
        0x3ff as libc::c_int,
        0x7ff as libc::c_int,
        0xfff as libc::c_int,
        0x1fff as libc::c_int,
        0x3fff as libc::c_int,
        0x7fff as libc::c_int,
        0xffff as libc::c_int,
    ];
    while (*state).bits_avail < n {
        if (*state).avail_in <= 0 as libc::c_int as libc::c_ulong {
            if (*state).consume_unnotified != 0 {
                __archive_read_filter_consume(
                    (*self_0).upstream,
                    (*state).consume_unnotified as int64_t,
                );
                (*state).consume_unnotified = 0 as libc::c_int as size_t
            }
            (*state).next_in = __archive_read_filter_ahead(
                (*self_0).upstream,
                1 as libc::c_int as size_t,
                &mut ret,
            ) as *const libc::c_uchar;
            if ret == 0 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int);
            }
            if ret < 0 as libc::c_int as libc::c_long || (*state).next_in.is_null() {
                return -(30 as libc::c_int);
            }
            (*state).avail_in = ret as size_t;
            (*state).consume_unnotified = (*state).avail_in
        }
        let fresh6 = (*state).next_in;
        (*state).next_in = (*state).next_in.offset(1);
        (*state).bit_buffer |= (*fresh6 as libc::c_int) << (*state).bits_avail;
        (*state).avail_in = (*state).avail_in.wrapping_sub(1);
        (*state).bits_avail += 8 as libc::c_int;
        (*state).bytes_in_section = (*state).bytes_in_section.wrapping_add(1)
    }
    code = (*state).bit_buffer;
    (*state).bit_buffer >>= n;
    (*state).bits_avail -= n;
    return code & mask[n as usize];
}
