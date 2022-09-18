use ::libc;
extern "C" {
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
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
    fn __archive_read_program(_: *mut archive_read_filter, _: *const libc::c_char) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
 * $FreeBSD: src/lib/libarchive/archive.h.in,v 1.50 2008/05/26 17:00:22 kientzle Exp $
 */
/*
 * The version number is expressed as a single integer that makes it
 * easy to compare versions at build time: for version a.b.c, the
 * version number is printf("%d%03d%03d",a,b,c).  For example, if you
 * know your application requires version 2.12.108 or later, you can
 * assert that ARCHIVE_VERSION_NUMBER >= 2012108.
 */
/* Note: Compiler will complain if this does not match archive_entry.h! */
/* for wchar_t */
/* For FILE * */
/* For time_t */
/*
 * Note: archive.h is for use outside of libarchive; the configuration
 * headers (config.h, archive_platform.h, etc.) are purely internal.
 * Do NOT use HAVE_XXX configuration macros to control the behavior of
 * this header!  If you must conditionalize, use predefined compiler and/or
 * platform macros.
 */
/* Get appropriate definitions of 64-bit integer */
/* Older code relied on the __LA_INT64_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
pub type la_int64_t = int64_t;
/* The la_ssize_t should match the type used in 'struct stat' */
/* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
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
/* No more operations are possible. */
/*
 * As far as possible, archive_errno returns standard platform errno codes.
 * Of course, the details vary by platform, so the actual definitions
 * here are stored in "archive_platform.h".  The symbols are listed here
 * for reference; as a rule, clients should not need to know the exact
 * platform-dependent error code.
 */
/* Unrecognized or invalid file format. */
/* #define	ARCHIVE_ERRNO_FILE_FORMAT */
/* Illegal usage of the library. */
/* #define	ARCHIVE_ERRNO_PROGRAMMER_ERROR */
/* Unknown or unclassified error. */
/* #define	ARCHIVE_ERRNO_MISC */
/*
 * Callbacks are invoked to automatically read/skip/write/open/close the
 * archive. You can provide your own for complex tasks (like breaking
 * archives across multiple tapes) or use standard ones built into the
 * library.
 */
/* Returns pointer and size of next block of data from archive. */
pub type archive_read_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut *const libc::c_void,
) -> la_ssize_t;
/* Skips at most request bytes from archive and returns the skipped amount.
 * This may skip fewer bytes than requested; it may even skip zero bytes.
 * If you do skip fewer bytes than requested, libarchive will invoke your
 * read callback and discard data as necessary to make up the full skip.
 */
pub type archive_skip_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void, _: la_int64_t) -> la_int64_t;
/* Seeks to specified location in the file and returns the position.
 * Whence values are SEEK_SET, SEEK_CUR, SEEK_END from stdio.h.
 * Return ARCHIVE_FATAL if the seek fails for any reason.
 */
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
/* Switches from one client data object to the next/prev client data object.
 * This is useful for reading from different data blocks such as a set of files
 * that make up one large file.
 */
pub type archive_switch_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut libc::c_void,
) -> libc::c_int;
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
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
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * Codes to identify various stream filters.
 */
pub const ARCHIVE_FILTER_LZ4: libc::c_int = 13 as libc::c_int;
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
pub const LZ4_MAGICNUMBER: libc::c_int = 0x184d2204 as libc::c_int;
pub const LZ4_LEGACY: libc::c_int = 0x184c2102 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_lz4(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut reader: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_filter_lz4\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if __archive_read_get_bidder(a, &mut reader) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*reader).data = NULL as *mut libc::c_void;
    (*reader).name = b"lz4\x00" as *const u8 as *const libc::c_char;
    (*reader).bid = Some(
        lz4_reader_bid
            as unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *mut archive_read_filter,
            ) -> libc::c_int,
    );
    (*reader).init =
        Some(lz4_reader_init as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int);
    (*reader).options = ::std::mem::transmute::<
        libc::intptr_t,
        Option<
            unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
        >,
    >(NULL as libc::intptr_t);
    (*reader).free = Some(
        lz4_reader_free as unsafe extern "C" fn(_: *mut archive_read_filter_bidder) -> libc::c_int,
    );
    archive_set_error(
        _a,
        ARCHIVE_ERRNO_MISC,
        b"Using external lz4 program\x00" as *const u8 as *const libc::c_char,
    );
    return -(20 as libc::c_int);
}
unsafe extern "C" fn lz4_reader_free(mut self_0: *mut archive_read_filter_bidder) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
/*
 * Note that we can detect lz4 archives even if we can't decompress
 * them.  (In fact, we like detecting them because we can give better
 * error messages.)  So the bid framework here gets compiled even
 * if liblz4 is unavailable.
 */
/*
 * Test whether we can handle this data.
 *
 * This logic returns zero if any part of the signature fails.  It
 * also tries to Do The Right Thing if a very short buffer prevents us
 * from verifying as much as we would like.
 */
unsafe extern "C" fn lz4_reader_bid(
    mut self_0: *mut archive_read_filter_bidder,
    mut filter: *mut archive_read_filter,
) -> libc::c_int {
    let mut buffer: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: ssize_t = 0;
    let mut bits_checked: libc::c_int = 0;
    let mut number: uint32_t = 0;
    /* UNUSED */
    /* Minimal lz4 archive is 11 bytes. */
    buffer = __archive_read_filter_ahead(filter, 11 as libc::c_int as size_t, &mut avail)
        as *const libc::c_uchar;
    if buffer.is_null() {
        return 0 as libc::c_int;
    }
    /* First four bytes must be LZ4 magic numbers. */
    bits_checked = 0 as libc::c_int;
    number = archive_le32dec(buffer as *const libc::c_void);
    if number == LZ4_MAGICNUMBER as libc::c_uint {
        let mut flag: libc::c_uchar = 0;
        let mut BD: libc::c_uchar = 0;
        bits_checked += 32 as libc::c_int;
        /* Next follows a stream descriptor. */
        /* Descriptor Flags. */
        flag = *buffer.offset(4 as libc::c_int as isize);
        /* A version number must be "01". */
        if (flag as libc::c_int & 0xc0 as libc::c_int) >> 6 as libc::c_int != 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        /* A reserved bit must be "0". */
        if flag as libc::c_int & 2 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
        bits_checked += 8 as libc::c_int;
        BD = *buffer.offset(5 as libc::c_int as isize);
        /* A block maximum size should be more than 3. */
        if ((BD as libc::c_int & 0x70 as libc::c_int) >> 4 as libc::c_int) < 4 as libc::c_int {
            return 0 as libc::c_int;
        }
        /* Reserved bits must be "0". */
        if BD as libc::c_int & !(0x70 as libc::c_int) != 0 {
            return 0 as libc::c_int;
        }
        bits_checked += 8 as libc::c_int
    } else if number == LZ4_LEGACY as libc::c_uint {
        bits_checked += 32 as libc::c_int
    }
    return bits_checked;
}
/*
 * If we don't have the library on this system, we can't actually do the
 * decompression.  We can, however, still detect compressed archives
 * and emit a useful message.
 */
unsafe extern "C" fn lz4_reader_init(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = __archive_read_program(self_0, b"lz4 -d -q\x00" as *const u8 as *const libc::c_char);
    /* Note: We set the format here even if __archive_read_program()
     * above fails.  We do, after all, know what the format is
     * even if we weren't able to read it. */
    (*self_0).code = ARCHIVE_FILTER_LZ4;
    (*self_0).name = b"lz4\x00" as *const u8 as *const libc::c_char;
    return r;
}
/* HAVE_LIBLZ4 */
