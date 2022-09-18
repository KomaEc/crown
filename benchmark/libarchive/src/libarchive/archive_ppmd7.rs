use ::libc;
extern "C" {
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
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    pub type archive_write;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
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
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
/* Operation was successful. */
/* Retry might succeed. */
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_read_data_node {
    pub begin_position: int64_t,
    pub total_size: int64_t,
    pub data: *mut libc::c_void,
}
pub type Byte = libc::c_uchar;
pub type UInt16 = libc::c_ushort;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type UInt64 = libc::c_ulonglong;
pub type Bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteIn {
    pub a: *mut archive_read,
    pub Read: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> Byte>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IByteOut {
    pub a: *mut archive_write,
    pub Write: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_See {
    pub Summ: UInt16,
    pub Shift: Byte,
    pub Count: Byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_State {
    pub Symbol: Byte,
    pub Freq: Byte,
    pub SuccessorLow: UInt16,
    pub SuccessorHigh: UInt16,
}
pub type CPpmd_State_Ref = UInt32;
pub type CPpmd_Void_Ref = UInt32;
/* SEE-contexts for PPM-contexts with masked symbols */
/* Freq */
/* Speed of Freq change; low Shift is for fast change */
/* Count to next change of Shift */
pub type CPpmd_Byte_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7_Context_ {
    pub NumStats: UInt16,
    pub SummFreq: UInt16,
    pub Stats: CPpmd_State_Ref,
    pub Suffix: CPpmd7_Context_Ref,
}
pub type CPpmd7_Context_Ref = UInt32;
pub type CPpmd7_Context = CPpmd7_Context_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7 {
    pub MinContext: *mut CPpmd7_Context,
    pub MaxContext: *mut CPpmd7_Context,
    pub FoundState: *mut CPpmd_State,
    pub OrderFall: libc::c_uint,
    pub InitEsc: libc::c_uint,
    pub PrevSuccess: libc::c_uint,
    pub MaxOrder: libc::c_uint,
    pub HiBitsFlag: libc::c_uint,
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
    pub Indx2Units: [Byte; 38],
    pub Units2Indx: [Byte; 128],
    pub FreeList: [CPpmd_Void_Ref; 38],
    pub NS2Indx: [Byte; 256],
    pub NS2BSIndx: [Byte; 256],
    pub HB2Flag: [Byte; 256],
    pub DummySee: CPpmd_See,
    pub See: [[CPpmd_See; 16]; 25],
    pub BinSumm: [[UInt16; 64]; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPpmd7_RangeDec {
    pub GetThreshold: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32>,
    pub Decode: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32, _: UInt32) -> ()>,
    pub DecodeBit: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7z_RangeDec {
    pub p: IPpmd7_RangeDec,
    pub Range: UInt32,
    pub Code: UInt32,
    pub Low: UInt32,
    pub Bottom: UInt32,
    pub Stream: *mut IByteIn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7z_RangeEnc {
    pub Low: UInt64,
    pub Range: UInt32,
    pub Cache: Byte,
    pub CacheSize: UInt64,
    pub Stream: *mut IByteOut,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPpmd7 {
    pub Ppmd7_Construct: Option<unsafe extern "C" fn(_: *mut CPpmd7) -> ()>,
    pub Ppmd7_Alloc: Option<unsafe extern "C" fn(_: *mut CPpmd7, _: UInt32) -> Bool>,
    pub Ppmd7_Free: Option<unsafe extern "C" fn(_: *mut CPpmd7) -> ()>,
    pub Ppmd7_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7, _: libc::c_uint) -> ()>,
    pub Ppmd7z_RangeDec_CreateVTable: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> ()>,
    pub PpmdRAR_RangeDec_CreateVTable: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> ()>,
    pub Ppmd7z_RangeDec_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> Bool>,
    pub PpmdRAR_RangeDec_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> Bool>,
    pub Ppmd7_DecodeSymbol:
        Option<unsafe extern "C" fn(_: *mut CPpmd7, _: *mut IPpmd7_RangeDec) -> libc::c_int>,
    pub Ppmd7z_RangeEnc_Init: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeEnc) -> ()>,
    pub Ppmd7z_RangeEnc_FlushData: Option<unsafe extern "C" fn(_: *mut CPpmd7z_RangeEnc) -> ()>,
    pub Ppmd7_EncodeSymbol: Option<
        unsafe extern "C" fn(_: *mut CPpmd7, _: *mut CPpmd7z_RangeEnc, _: libc::c_int) -> (),
    >,
}
pub type CTX_PTR = *mut CPpmd7_Context;
pub type CPpmd7_Node_Ref = UInt32;
pub type CPpmd7_Node = CPpmd7_Node_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7_Node_ {
    pub Stamp: UInt16,
    pub NU: UInt16,
    pub Next: CPpmd7_Node_Ref,
    pub Prev: CPpmd7_Node_Ref,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const True: libc::c_int = 1 as libc::c_int;
pub const False: libc::c_int = 0 as libc::c_int;
pub const PPMD_BIN_SCALE: libc::c_int = (1 as libc::c_int) << PPMD_INT_BITS + PPMD_PERIOD_BITS;
pub const PPMD_INT_BITS: libc::c_int = 7 as libc::c_int;
pub const PPMD_PERIOD_BITS: libc::c_int = 7 as libc::c_int;
pub const PPMD_NUM_INDEXES: libc::c_int = PPMD_N1 + PPMD_N2 + PPMD_N3 + PPMD_N4;
pub const PPMD_N4: libc::c_int = (128 as libc::c_int + 3 as libc::c_int
    - 1 as libc::c_int * PPMD_N1
    - 2 as libc::c_int * PPMD_N2
    - 3 as libc::c_int * PPMD_N3)
    / 4 as libc::c_int;
pub const PPMD_N1: libc::c_int = 4 as libc::c_int;
pub const PPMD_N2: libc::c_int = 4 as libc::c_int;
pub const PPMD_N3: libc::c_int = 4 as libc::c_int;
pub const kTopValue_0: libc::c_int = (1 as libc::c_int) << 24 as libc::c_int;
pub const MAX_FREQ: libc::c_int = 124 as libc::c_int;
pub const UNIT_SIZE: libc::c_int = 12 as libc::c_int;
static mut kInitBinEsc: [UInt16; 8] = [
    0x3cdd as libc::c_int as UInt16,
    0x1f3f as libc::c_int as UInt16,
    0x59bf as libc::c_int as UInt16,
    0x48f3 as libc::c_int as UInt16,
    0x64a1 as libc::c_int as UInt16,
    0x5abc as libc::c_int as UInt16,
    0x6632 as libc::c_int as UInt16,
    0x6051 as libc::c_int as UInt16,
];
static mut PPMD7_kExpEscape: [Byte; 16] = [
    25 as libc::c_int as Byte,
    14 as libc::c_int as Byte,
    9 as libc::c_int as Byte,
    7 as libc::c_int as Byte,
    5 as libc::c_int as Byte,
    5 as libc::c_int as Byte,
    4 as libc::c_int as Byte,
    4 as libc::c_int as Byte,
    4 as libc::c_int as Byte,
    3 as libc::c_int as Byte,
    3 as libc::c_int as Byte,
    3 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
    2 as libc::c_int as Byte,
];
/* must be at offset 0 as CPpmd7_Context::NumStats. Stamp=0 means free */
/* must be at offset >= 4 */
/* ----------- Base ----------- */
unsafe extern "C" fn Ppmd7_Construct(mut p: *mut CPpmd7) {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    (*p).Base = 0 as *mut Byte;
    i = 0 as libc::c_int as libc::c_uint;
    k = 0 as libc::c_int as libc::c_uint;
    while i < PPMD_NUM_INDEXES as libc::c_uint {
        let mut step: libc::c_uint = if i >= 12 as libc::c_int as libc::c_uint {
            4 as libc::c_int as libc::c_uint
        } else {
            (i >> 2 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_uint)
        };
        loop {
            let fresh0 = k;
            k = k.wrapping_add(1);
            (*p).Units2Indx[fresh0 as usize] = i as Byte;
            step = step.wrapping_sub(1);
            if !(step != 0) {
                break;
            }
        }
        (*p).Indx2Units[i as usize] = k as Byte;
        i = i.wrapping_add(1)
    }
    (*p).NS2BSIndx[0 as libc::c_int as usize] = ((0 as libc::c_int) << 1 as libc::c_int) as Byte;
    (*p).NS2BSIndx[1 as libc::c_int as usize] = ((1 as libc::c_int) << 1 as libc::c_int) as Byte;
    memset(
        (*p).NS2BSIndx
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut libc::c_void,
        (2 as libc::c_int) << 1 as libc::c_int,
        9 as libc::c_int as libc::c_ulong,
    );
    memset(
        (*p).NS2BSIndx
            .as_mut_ptr()
            .offset(11 as libc::c_int as isize) as *mut libc::c_void,
        (3 as libc::c_int) << 1 as libc::c_int,
        (256 as libc::c_int - 11 as libc::c_int) as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        (*p).NS2Indx[i as usize] = i as Byte;
        i = i.wrapping_add(1)
    }
    m = i;
    k = 1 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        (*p).NS2Indx[i as usize] = m as Byte;
        k = k.wrapping_sub(1);
        if k == 0 as libc::c_int as libc::c_uint {
            m = m.wrapping_add(1);
            k = m.wrapping_sub(2 as libc::c_int as libc::c_uint)
        }
        i = i.wrapping_add(1)
    }
    memset(
        (*p).HB2Flag.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        0x40 as libc::c_int as libc::c_ulong,
    );
    memset(
        (*p).HB2Flag
            .as_mut_ptr()
            .offset(0x40 as libc::c_int as isize) as *mut libc::c_void,
        8 as libc::c_int,
        (0x100 as libc::c_int - 0x40 as libc::c_int) as libc::c_ulong,
    );
}
unsafe extern "C" fn Ppmd7_Free(mut p: *mut CPpmd7) {
    free((*p).Base as *mut libc::c_void);
    (*p).Size = 0 as libc::c_int as UInt32;
    (*p).Base = 0 as *mut Byte;
}
unsafe extern "C" fn Ppmd7_Alloc(mut p: *mut CPpmd7, mut size: UInt32) -> Bool {
    if (*p).Base.is_null() || (*p).Size != size {
        /* RestartModel() below assumes that p->Size >= UNIT_SIZE
        (see the calculation of m->MinContext). */
        if size < UNIT_SIZE as libc::c_uint {
            return False;
        }
        Ppmd7_Free(p);
        (*p).AlignOffset = (4 as libc::c_int as libc::c_uint)
            .wrapping_sub(size & 3 as libc::c_int as libc::c_uint);
        (*p).Base = malloc(
            (*p).AlignOffset
                .wrapping_add(size)
                .wrapping_add(UNIT_SIZE as libc::c_uint) as libc::c_ulong,
        ) as *mut Byte;
        if (*p).Base.is_null() {
            return False;
        }
        (*p).Size = size
    }
    return True;
}
unsafe extern "C" fn InsertNode(
    mut p: *mut CPpmd7,
    mut node: *mut libc::c_void,
    mut indx: libc::c_uint,
) {
    *(node as *mut CPpmd_Void_Ref) = (*p).FreeList[indx as usize];
    (*p).FreeList[indx as usize] =
        (node as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
}
unsafe extern "C" fn RemoveNode(mut p: *mut CPpmd7, mut indx: libc::c_uint) -> *mut libc::c_void {
    let mut node: *mut CPpmd_Void_Ref = (*p).Base.offset((*p).FreeList[indx as usize] as isize)
        as *mut libc::c_void as *mut CPpmd_Void_Ref;
    (*p).FreeList[indx as usize] = *node;
    return node as *mut libc::c_void;
}
unsafe extern "C" fn SplitBlock(
    mut p: *mut CPpmd7,
    mut ptr: *mut libc::c_void,
    mut oldIndx: libc::c_uint,
    mut newIndx: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut nu: libc::c_uint = ((*p).Indx2Units[oldIndx as usize] as libc::c_int
        - (*p).Indx2Units[newIndx as usize] as libc::c_int)
        as libc::c_uint;
    ptr = (ptr as *mut Byte).offset(
        ((*p).Indx2Units[newIndx as usize] as UInt32).wrapping_mul(UNIT_SIZE as libc::c_uint)
            as isize,
    ) as *mut libc::c_void;
    i = (*p).Units2Indx[nu.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize] as libc::c_uint;
    if (*p).Indx2Units[i as usize] as libc::c_uint != nu {
        i = i.wrapping_sub(1);
        let mut k: libc::c_uint = (*p).Indx2Units[i as usize] as libc::c_uint;
        InsertNode(
            p,
            (ptr as *mut Byte).offset(k.wrapping_mul(UNIT_SIZE as libc::c_uint) as isize)
                as *mut libc::c_void,
            nu.wrapping_sub(k)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    InsertNode(p, ptr, i);
}
unsafe extern "C" fn GlueFreeBlocks(mut p: *mut CPpmd7) {
    let mut head: CPpmd7_Node_Ref = (*p).AlignOffset.wrapping_add((*p).Size);
    let mut n: CPpmd7_Node_Ref = head;
    let mut i: libc::c_uint = 0;
    (*p).GlueCount = 255 as libc::c_int as UInt32;
    /* create doubly-linked list of free blocks */
    i = 0 as libc::c_int as libc::c_uint;
    while i < PPMD_NUM_INDEXES as libc::c_uint {
        let mut nu: UInt16 = (*p).Indx2Units[i as usize] as UInt16;
        let mut next: CPpmd7_Node_Ref = (*p).FreeList[i as usize];
        (*p).FreeList[i as usize] = 0 as libc::c_int as CPpmd_Void_Ref;
        while next != 0 as libc::c_int as libc::c_uint {
            let mut node: *mut CPpmd7_Node = (*p).Base.offset(next as isize) as *mut CPpmd7_Node;
            (*node).Next = n;
            let ref mut fresh1 = (*((*p).Base.offset(n as isize) as *mut CPpmd7_Node)).Prev;
            *fresh1 = next;
            n = *fresh1;
            next = *(node as *const CPpmd7_Node_Ref);
            (*node).Stamp = 0 as libc::c_int as UInt16;
            (*node).NU = nu
        }
        i = i.wrapping_add(1)
    }
    (*((*p).Base.offset(head as isize) as *mut CPpmd7_Node)).Stamp = 1 as libc::c_int as UInt16;
    (*((*p).Base.offset(head as isize) as *mut CPpmd7_Node)).Next = n;
    (*((*p).Base.offset(n as isize) as *mut CPpmd7_Node)).Prev = head;
    if (*p).LoUnit != (*p).HiUnit {
        (*((*p).LoUnit as *mut CPpmd7_Node)).Stamp = 1 as libc::c_int as UInt16
    }
    /* Glue free blocks */
    while n != head {
        let mut node_0: *mut CPpmd7_Node = (*p).Base.offset(n as isize) as *mut CPpmd7_Node;
        let mut nu_0: UInt32 = (*node_0).NU as UInt32;
        loop {
            let mut node2: *mut CPpmd7_Node =
                ((*p).Base.offset(n as isize) as *mut CPpmd7_Node).offset(nu_0 as isize);
            nu_0 = (nu_0 as libc::c_uint).wrapping_add((*node2).NU as libc::c_uint) as UInt32
                as UInt32;
            if (*node2).Stamp as libc::c_int != 0 as libc::c_int
                || nu_0 >= 0x10000 as libc::c_int as libc::c_uint
            {
                break;
            }
            (*((*p).Base.offset((*node2).Prev as isize) as *mut CPpmd7_Node)).Next = (*node2).Next;
            (*((*p).Base.offset((*node2).Next as isize) as *mut CPpmd7_Node)).Prev = (*node2).Prev;
            (*node_0).NU = nu_0 as UInt16
        }
        n = (*node_0).Next
    }
    /* Fill lists of free blocks */
    n = (*((*p).Base.offset(head as isize) as *mut CPpmd7_Node)).Next; /* AllocContext(p); */
    while n != head {
        let mut node_1: *mut CPpmd7_Node = (*p).Base.offset(n as isize) as *mut CPpmd7_Node; /* AllocUnits(p, PPMD_NUM_INDEXES - 1); */
        let mut nu_1: libc::c_uint = 0; /* unused */
        let mut next_0: CPpmd7_Node_Ref = (*node_1).Next;
        nu_1 = (*node_1).NU as libc::c_uint;
        while nu_1 > 128 as libc::c_int as libc::c_uint {
            InsertNode(
                p,
                node_1 as *mut libc::c_void,
                (PPMD_NUM_INDEXES - 1 as libc::c_int) as libc::c_uint,
            );
            nu_1 = nu_1.wrapping_sub(128 as libc::c_int as libc::c_uint);
            node_1 = node_1.offset(128 as libc::c_int as isize)
        }
        i = (*p).Units2Indx[nu_1.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
            as libc::c_uint;
        if (*p).Indx2Units[i as usize] as libc::c_uint != nu_1 {
            i = i.wrapping_sub(1);
            let mut k: libc::c_uint = (*p).Indx2Units[i as usize] as libc::c_uint;
            InsertNode(
                p,
                node_1.offset(k as isize) as *mut libc::c_void,
                nu_1.wrapping_sub(k)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
        }
        InsertNode(p, node_1 as *mut libc::c_void, i);
        n = next_0
    }
}
unsafe extern "C" fn AllocUnitsRare(
    mut p: *mut CPpmd7,
    mut indx: libc::c_uint,
) -> *mut libc::c_void {
    let mut i: libc::c_uint = 0;
    let mut retVal: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*p).GlueCount == 0 as libc::c_int as libc::c_uint {
        GlueFreeBlocks(p);
        if (*p).FreeList[indx as usize] != 0 as libc::c_int as libc::c_uint {
            return RemoveNode(p, indx);
        }
    }
    i = indx;
    loop {
        i = i.wrapping_add(1);
        if i == PPMD_NUM_INDEXES as libc::c_uint {
            let mut numBytes: UInt32 =
                ((*p).Indx2Units[indx as usize] as UInt32).wrapping_mul(UNIT_SIZE as libc::c_uint);
            (*p).GlueCount = (*p).GlueCount.wrapping_sub(1);
            return if (*p).UnitsStart.offset_from((*p).Text) as libc::c_long as UInt32
                > numBytes
            {
                (*p).UnitsStart = (*p).UnitsStart.offset(-(numBytes as isize));
                (*p).UnitsStart
            } else {
                0 as *mut Byte
            } as *mut libc::c_void;
        }
        if !((*p).FreeList[i as usize] == 0 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    retVal = RemoveNode(p, i);
    SplitBlock(p, retVal, i, indx);
    return retVal;
}
unsafe extern "C" fn AllocUnits(mut p: *mut CPpmd7, mut indx: libc::c_uint) -> *mut libc::c_void {
    let mut numBytes: UInt32 = 0;
    if (*p).FreeList[indx as usize] != 0 as libc::c_int as libc::c_uint {
        return RemoveNode(p, indx);
    }
    numBytes = ((*p).Indx2Units[indx as usize] as UInt32).wrapping_mul(UNIT_SIZE as libc::c_uint);
    if numBytes <= (*p).HiUnit.offset_from((*p).LoUnit) as libc::c_long as UInt32 {
        let mut retVal: *mut libc::c_void = (*p).LoUnit as *mut libc::c_void;
        (*p).LoUnit = (*p).LoUnit.offset(numBytes as isize);
        return retVal;
    }
    return AllocUnitsRare(p, indx);
}
unsafe extern "C" fn ShrinkUnits(
    mut p: *mut CPpmd7,
    mut oldPtr: *mut libc::c_void,
    mut oldNU: libc::c_uint,
    mut newNU: libc::c_uint,
) -> *mut libc::c_void {
    let mut i0: libc::c_uint = (*p).Units2Indx
        [oldNU.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
        as libc::c_uint;
    let mut i1: libc::c_uint = (*p).Units2Indx
        [newNU.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
        as libc::c_uint;
    if i0 == i1 {
        return oldPtr;
    }
    if (*p).FreeList[i1 as usize] != 0 as libc::c_int as libc::c_uint {
        let mut ptr: *mut libc::c_void = RemoveNode(p, i1);
        let mut d: *mut UInt32 = ptr as *mut UInt32;
        let mut s: *const UInt32 = oldPtr as *const UInt32;
        let mut n: UInt32 = newNU;
        loop {
            *d.offset(0 as libc::c_int as isize) = *s.offset(0 as libc::c_int as isize);
            *d.offset(1 as libc::c_int as isize) = *s.offset(1 as libc::c_int as isize);
            *d.offset(2 as libc::c_int as isize) = *s.offset(2 as libc::c_int as isize);
            s = s.offset(3 as libc::c_int as isize);
            d = d.offset(3 as libc::c_int as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        InsertNode(p, oldPtr, i0);
        return ptr;
    }
    SplitBlock(p, oldPtr, i0, i1);
    return oldPtr;
}
unsafe extern "C" fn SetSuccessor(mut p: *mut CPpmd_State, mut v: CPpmd_Void_Ref) {
    (*p).SuccessorLow = (v & 0xffff as libc::c_int as libc::c_uint) as UInt16;
    (*p).SuccessorHigh = (v >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_uint) as UInt16;
}
unsafe extern "C" fn RestartModel(mut p: *mut CPpmd7) {
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    memset(
        (*p).FreeList.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[CPpmd_Void_Ref; 38]>() as libc::c_ulong,
    );
    (*p).Text = (*p).Base.offset((*p).AlignOffset as isize);
    (*p).HiUnit = (*p).Text.offset((*p).Size as isize);
    (*p).UnitsStart = (*p).HiUnit.offset(
        -((*p)
            .Size
            .wrapping_div(8 as libc::c_int as libc::c_uint)
            .wrapping_div(UNIT_SIZE as libc::c_uint)
            .wrapping_mul(7 as libc::c_int as libc::c_uint)
            .wrapping_mul(UNIT_SIZE as libc::c_uint) as isize),
    );
    (*p).LoUnit = (*p).UnitsStart;
    (*p).GlueCount = 0 as libc::c_int as UInt32;
    (*p).OrderFall = (*p).MaxOrder;
    (*p).InitRL = -((if (*p).MaxOrder < 12 as libc::c_int as libc::c_uint {
        (*p).MaxOrder
    } else {
        12 as libc::c_int as libc::c_uint
    }) as Int32)
        - 1 as libc::c_int;
    (*p).RunLength = (*p).InitRL;
    (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint;
    (*p).HiUnit = (*p).HiUnit.offset(-(UNIT_SIZE as isize));
    (*p).MaxContext = (*p).HiUnit as CTX_PTR;
    (*p).MinContext = (*p).MaxContext;
    (*(*p).MinContext).Suffix = 0 as libc::c_int as CPpmd7_Context_Ref;
    (*(*p).MinContext).NumStats = 256 as libc::c_int as UInt16;
    (*(*p).MinContext).SummFreq = (256 as libc::c_int + 1 as libc::c_int) as UInt16;
    (*p).FoundState = (*p).LoUnit as *mut CPpmd_State;
    (*p).LoUnit = (*p).LoUnit.offset(
        ((256 as libc::c_int / 2 as libc::c_int) as UInt32).wrapping_mul(UNIT_SIZE as libc::c_uint)
            as isize,
    );
    (*(*p).MinContext).Stats =
        ((*p).FoundState as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 256 as libc::c_int as libc::c_uint {
        let mut s: *mut CPpmd_State = &mut *(*p).FoundState.offset(i as isize) as *mut CPpmd_State;
        (*s).Symbol = i as Byte;
        (*s).Freq = 1 as libc::c_int as Byte;
        SetSuccessor(s, 0 as libc::c_int as CPpmd_Void_Ref);
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 128 as libc::c_int as libc::c_uint {
        k = 0 as libc::c_int as libc::c_uint;
        while k < 8 as libc::c_int as libc::c_uint {
            let mut dest: *mut UInt16 = (*p).BinSumm[i as usize].as_mut_ptr().offset(k as isize);
            let mut val: UInt16 = (PPMD_BIN_SCALE as libc::c_uint).wrapping_sub(
                (kInitBinEsc[k as usize] as libc::c_uint)
                    .wrapping_div(i.wrapping_add(2 as libc::c_int as libc::c_uint)),
            ) as UInt16;
            m = 0 as libc::c_int as libc::c_uint;
            while m < 64 as libc::c_int as libc::c_uint {
                *dest.offset(m as isize) = val;
                m = m.wrapping_add(8 as libc::c_int as libc::c_uint)
            }
            k = k.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 25 as libc::c_int as libc::c_uint {
        k = 0 as libc::c_int as libc::c_uint;
        while k < 16 as libc::c_int as libc::c_uint {
            let mut s_0: *mut CPpmd_See = &mut *(*(*p).See.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(k as isize) as *mut CPpmd_See;
            (*s_0).Shift = (PPMD_PERIOD_BITS - 4 as libc::c_int) as Byte;
            (*s_0).Summ = ((5 as libc::c_int as libc::c_uint)
                .wrapping_mul(i)
                .wrapping_add(10 as libc::c_int as libc::c_uint)
                << (*s_0).Shift as libc::c_int) as UInt16;
            (*s_0).Count = 4 as libc::c_int as Byte;
            k = k.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn Ppmd7_Init(mut p: *mut CPpmd7, mut maxOrder: libc::c_uint) {
    (*p).MaxOrder = maxOrder;
    RestartModel(p);
    (*p).DummySee.Shift = PPMD_PERIOD_BITS as Byte;
    (*p).DummySee.Summ = 0 as libc::c_int as UInt16;
    (*p).DummySee.Count = 64 as libc::c_int as Byte;
    /* unused */
}
unsafe extern "C" fn CreateSuccessors(mut p: *mut CPpmd7, mut skip: Bool) -> CTX_PTR {
    let mut upState: CPpmd_State = CPpmd_State {
        Symbol: 0,
        Freq: 0,
        SuccessorLow: 0,
        SuccessorHigh: 0,
    };
    let mut c: CTX_PTR = (*p).MinContext;
    let mut upBranch: CPpmd_Byte_Ref = (*(*p).FoundState).SuccessorLow as libc::c_uint
        | ((*(*p).FoundState).SuccessorHigh as UInt32) << 16 as libc::c_int;
    let mut ps: [*mut CPpmd_State; 64] = [0 as *mut CPpmd_State; 64];
    let mut numPs: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if skip == 0 {
        let fresh2 = numPs;
        numPs = numPs.wrapping_add(1);
        ps[fresh2 as usize] = (*p).FoundState
    }
    while (*c).Suffix != 0 {
        let mut successor: CPpmd_Void_Ref = 0;
        let mut s: *mut CPpmd_State = 0 as *mut CPpmd_State;
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd7_Context;
        if (*c).NumStats as libc::c_int != 1 as libc::c_int {
            s = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            while (*s).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int {
                s = s.offset(1)
            }
        } else {
            s = &mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State
        }
        successor =
            (*s).SuccessorLow as libc::c_uint | ((*s).SuccessorHigh as UInt32) << 16 as libc::c_int;
        if successor != upBranch {
            c = (*p).Base.offset(successor as isize) as *mut libc::c_void as *mut CPpmd7_Context;
            if numPs == 0 as libc::c_int as libc::c_uint {
                return c;
            }
            break;
        } else {
            let fresh3 = numPs;
            numPs = numPs.wrapping_add(1);
            ps[fresh3 as usize] = s
        }
    }
    upState.Symbol = *((*p).Base.offset(upBranch as isize) as *mut libc::c_void as *const Byte);
    SetSuccessor(
        &mut upState,
        upBranch.wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    if (*c).NumStats as libc::c_int == 1 as libc::c_int {
        upState.Freq = (*(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq
    } else {
        let mut cf: UInt32 = 0;
        let mut s0: UInt32 = 0;
        let mut s_0: *mut CPpmd_State = 0 as *mut CPpmd_State;
        s_0 = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
        while (*s_0).Symbol as libc::c_int != upState.Symbol as libc::c_int {
            s_0 = s_0.offset(1)
        }
        cf = ((*s_0).Freq as libc::c_int - 1 as libc::c_int) as UInt32;
        s0 = (((*c).SummFreq as libc::c_int - (*c).NumStats as libc::c_int) as libc::c_uint)
            .wrapping_sub(cf);
        upState.Freq = (1 as libc::c_int as libc::c_uint).wrapping_add(
            (if (2 as libc::c_int as libc::c_uint).wrapping_mul(cf) <= s0 {
                ((5 as libc::c_int as libc::c_uint).wrapping_mul(cf) > s0) as libc::c_int
                    as libc::c_uint
            } else {
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul(cf)
                    .wrapping_add((3 as libc::c_int as libc::c_uint).wrapping_mul(s0))
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_div((2 as libc::c_int as libc::c_uint).wrapping_mul(s0))
            }),
        ) as Byte
    }
    while numPs != 0 as libc::c_int as libc::c_uint {
        /* Create Child */
        let mut c1: CTX_PTR = 0 as *mut CPpmd7_Context; /* = AllocContext(p); */
        if (*p).HiUnit != (*p).LoUnit {
            (*p).HiUnit = (*p).HiUnit.offset(-(UNIT_SIZE as isize));
            c1 = (*p).HiUnit as CTX_PTR
        } else if (*p).FreeList[0 as libc::c_int as usize] != 0 as libc::c_int as libc::c_uint {
            c1 = RemoveNode(p, 0 as libc::c_int as libc::c_uint) as CTX_PTR
        } else {
            c1 = AllocUnitsRare(p, 0 as libc::c_int as libc::c_uint) as CTX_PTR;
            if c1.is_null() {
                return NULL as CTX_PTR;
            }
        }
        (*c1).NumStats = 1 as libc::c_int as UInt16;
        *(&mut (*c1).SummFreq as *mut UInt16 as *mut CPpmd_State) = upState;
        (*c1).Suffix = (c as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
        numPs = numPs.wrapping_sub(1);
        SetSuccessor(
            ps[numPs as usize],
            (c1 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32,
        );
        c = c1
    }
    return c;
}
unsafe extern "C" fn SwapStates(mut t1: *mut CPpmd_State, mut t2: *mut CPpmd_State) {
    let mut tmp: CPpmd_State = *t1;
    *t1 = *t2;
    *t2 = tmp;
}
unsafe extern "C" fn UpdateModel(mut p: *mut CPpmd7) {
    let mut successor: CPpmd_Void_Ref = 0;
    let mut fSuccessor: CPpmd_Void_Ref = (*(*p).FoundState).SuccessorLow as libc::c_uint
        | ((*(*p).FoundState).SuccessorHigh as UInt32) << 16 as libc::c_int;
    let mut c: CTX_PTR = 0 as *mut CPpmd7_Context;
    let mut s0: libc::c_uint = 0;
    let mut ns: libc::c_uint = 0;
    if ((*(*p).FoundState).Freq as libc::c_int) < MAX_FREQ / 4 as libc::c_int
        && (*(*p).MinContext).Suffix != 0 as libc::c_int as libc::c_uint
    {
        c = (*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
            as *mut CPpmd7_Context;
        if (*c).NumStats as libc::c_int == 1 as libc::c_int {
            let mut s: *mut CPpmd_State = &mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State;
            if ((*s).Freq as libc::c_int) < 32 as libc::c_int {
                (*s).Freq = (*s).Freq.wrapping_add(1)
            }
        } else {
            let mut s_0: *mut CPpmd_State =
                (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void as *mut CPpmd_State;
            if (*s_0).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int {
                loop {
                    s_0 = s_0.offset(1);
                    if !((*s_0).Symbol as libc::c_int != (*(*p).FoundState).Symbol as libc::c_int) {
                        break;
                    }
                }
                if (*s_0.offset(0 as libc::c_int as isize)).Freq as libc::c_int
                    >= (*s_0.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int
                {
                    SwapStates(
                        &mut *s_0.offset(0 as libc::c_int as isize),
                        &mut *s_0.offset(-(1 as libc::c_int) as isize),
                    );
                    s_0 = s_0.offset(-1)
                }
            }
            if ((*s_0).Freq as libc::c_int) < MAX_FREQ - 9 as libc::c_int {
                (*s_0).Freq = ((*s_0).Freq as libc::c_int + 2 as libc::c_int) as Byte;
                (*c).SummFreq = ((*c).SummFreq as libc::c_int + 2 as libc::c_int) as UInt16
            }
        }
    }
    if (*p).OrderFall == 0 as libc::c_int as libc::c_uint {
        (*p).MaxContext = CreateSuccessors(p, True);
        (*p).MinContext = (*p).MaxContext;
        if (*p).MinContext.is_null() {
            RestartModel(p);
            return;
        }
        SetSuccessor(
            (*p).FoundState,
            ((*p).MinContext as *mut Byte).offset_from((*p).Base) as libc::c_long
                as UInt32,
        );
        return;
    }
    let fresh4 = (*p).Text;
    (*p).Text = (*p).Text.offset(1);
    *fresh4 = (*(*p).FoundState).Symbol;
    successor = (*p).Text.offset_from((*p).Base) as libc::c_long as UInt32;
    if (*p).Text >= (*p).UnitsStart {
        RestartModel(p);
        return;
    }
    if fSuccessor != 0 {
        if fSuccessor <= successor {
            let mut cs: CTX_PTR = CreateSuccessors(p, False);
            if cs.is_null() {
                RestartModel(p);
                return;
            }
            fSuccessor = (cs as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
        }
        (*p).OrderFall = (*p).OrderFall.wrapping_sub(1);
        if (*p).OrderFall == 0 as libc::c_int as libc::c_uint {
            successor = fSuccessor;
            (*p).Text = (*p)
                .Text
                .offset(-(((*p).MaxContext != (*p).MinContext) as libc::c_int as isize))
        }
    } else {
        SetSuccessor((*p).FoundState, successor);
        fSuccessor =
            ((*p).MinContext as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
    }
    ns = (*(*p).MinContext).NumStats as libc::c_uint;
    s0 = ((*(*p).MinContext).SummFreq as libc::c_uint)
        .wrapping_sub(ns)
        .wrapping_sub(((*(*p).FoundState).Freq as libc::c_int - 1 as libc::c_int) as libc::c_uint);
    c = (*p).MaxContext;
    while c != (*p).MinContext {
        let mut ns1: libc::c_uint = 0;
        let mut cf: UInt32 = 0;
        let mut sf: UInt32 = 0;
        ns1 = (*c).NumStats as libc::c_uint;
        if ns1 != 1 as libc::c_int as libc::c_uint {
            if ns1 & 1 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                /* Expand for one UNIT */
                let mut oldNU: libc::c_uint = ns1 >> 1 as libc::c_int;
                let mut i: libc::c_uint = (*p).Units2Indx
                    [oldNU.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_uint;
                if i != (*p).Units2Indx[oldNU
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as usize] as libc::c_uint
                {
                    let mut ptr: *mut libc::c_void =
                        AllocUnits(p, i.wrapping_add(1 as libc::c_int as libc::c_uint));
                    let mut oldPtr: *mut libc::c_void = 0 as *mut libc::c_void;
                    if ptr.is_null() {
                        RestartModel(p);
                        return;
                    }
                    oldPtr = (*p).Base.offset((*c).Stats as isize) as *mut libc::c_void
                        as *mut CPpmd_State as *mut libc::c_void;
                    let mut d: *mut UInt32 = ptr as *mut UInt32;
                    let mut s_1: *const UInt32 = oldPtr as *const UInt32;
                    let mut n: UInt32 = oldNU;
                    loop {
                        *d.offset(0 as libc::c_int as isize) =
                            *s_1.offset(0 as libc::c_int as isize);
                        *d.offset(1 as libc::c_int as isize) =
                            *s_1.offset(1 as libc::c_int as isize);
                        *d.offset(2 as libc::c_int as isize) =
                            *s_1.offset(2 as libc::c_int as isize);
                        s_1 = s_1.offset(3 as libc::c_int as isize);
                        d = d.offset(3 as libc::c_int as isize);
                        n = n.wrapping_sub(1);
                        if !(n != 0) {
                            break;
                        }
                    }
                    InsertNode(p, oldPtr, i);
                    (*c).Stats =
                        (ptr as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32
                }
            }
            (*c).SummFreq = ((*c).SummFreq as libc::c_int
                + ((2 as libc::c_int as libc::c_uint).wrapping_mul(ns1) < ns) as libc::c_int
                + 2 as libc::c_int
                    * (((4 as libc::c_int as libc::c_uint).wrapping_mul(ns1) <= ns) as libc::c_int
                        & ((*c).SummFreq as libc::c_uint
                            <= (8 as libc::c_int as libc::c_uint).wrapping_mul(ns1))
                            as libc::c_int)) as UInt16
        } else {
            let mut s_2: *mut CPpmd_State =
                AllocUnits(p, 0 as libc::c_int as libc::c_uint) as *mut CPpmd_State;
            if s_2.is_null() {
                RestartModel(p);
                return;
            }
            *s_2 = *(&mut (*c).SummFreq as *mut UInt16 as *mut CPpmd_State);
            (*c).Stats =
                (s_2 as *mut Byte).offset_from((*p).Base) as libc::c_long as UInt32;
            if ((*s_2).Freq as libc::c_int) < MAX_FREQ / 4 as libc::c_int - 1 as libc::c_int {
                (*s_2).Freq = (((*s_2).Freq as libc::c_int) << 1 as libc::c_int) as Byte
            } else {
                (*s_2).Freq = (MAX_FREQ - 4 as libc::c_int) as Byte
            }
            (*c).SummFreq = ((*s_2).Freq as libc::c_uint)
                .wrapping_add((*p).InitEsc)
                .wrapping_add(
                    (ns > 3 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint,
                ) as UInt16
        }
        cf = (2 as libc::c_int as libc::c_uint)
            .wrapping_mul((*(*p).FoundState).Freq as UInt32)
            .wrapping_mul(((*c).SummFreq as libc::c_int + 6 as libc::c_int) as libc::c_uint);
        sf = s0.wrapping_add((*c).SummFreq as libc::c_uint);
        if cf < (6 as libc::c_int as libc::c_uint).wrapping_mul(sf) {
            cf = (1 as libc::c_int
                + (cf > sf) as libc::c_int
                + (cf >= (4 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int)
                as UInt32;
            (*c).SummFreq = ((*c).SummFreq as libc::c_int + 3 as libc::c_int) as UInt16
        } else {
            cf = (4 as libc::c_int
                + (cf >= (9 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                + (cf >= (12 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int
                + (cf >= (15 as libc::c_int as libc::c_uint).wrapping_mul(sf)) as libc::c_int)
                as UInt32;
            (*c).SummFreq = ((*c).SummFreq as libc::c_uint).wrapping_add(cf) as UInt16
        }
        let mut s_3: *mut CPpmd_State = ((*p).Base.offset((*c).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State)
            .offset(ns1 as isize);
        SetSuccessor(s_3, successor);
        (*s_3).Symbol = (*(*p).FoundState).Symbol;
        (*s_3).Freq = cf as Byte;
        (*c).NumStats = ns1.wrapping_add(1 as libc::c_int as libc::c_uint) as UInt16;
        c = (*p).Base.offset((*c).Suffix as isize) as *mut libc::c_void as *mut CPpmd7_Context
    }
    (*p).MinContext =
        (*p).Base.offset(fSuccessor as isize) as *mut libc::c_void as *mut CPpmd7_Context;
    (*p).MaxContext = (*p).MinContext;
}
unsafe extern "C" fn Rescale(mut p: *mut CPpmd7) {
    let mut i: libc::c_uint = 0;
    let mut adder: libc::c_uint = 0;
    let mut sumFreq: libc::c_uint = 0;
    let mut escFreq: libc::c_uint = 0;
    let mut stats: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
        as *mut libc::c_void as *mut CPpmd_State;
    let mut s: *mut CPpmd_State = (*p).FoundState;
    let mut tmp: CPpmd_State = *s;
    while s != stats {
        *s.offset(0 as libc::c_int as isize) = *s.offset(-(1 as libc::c_int) as isize);
        s = s.offset(-1)
    }
    *s = tmp;
    escFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int - (*s).Freq as libc::c_int) as libc::c_uint;
    (*s).Freq = ((*s).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    adder = ((*p).OrderFall != 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint;
    (*s).Freq = (((*s).Freq as libc::c_uint).wrapping_add(adder) >> 1 as libc::c_int) as Byte;
    sumFreq = (*s).Freq as libc::c_uint;
    i = ((*(*p).MinContext).NumStats as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    loop {
        s = s.offset(1);
        escFreq = escFreq.wrapping_sub((*s).Freq as libc::c_uint);
        (*s).Freq = (((*s).Freq as libc::c_uint).wrapping_add(adder) >> 1 as libc::c_int) as Byte;
        sumFreq = sumFreq.wrapping_add((*s).Freq as libc::c_uint);
        if (*s.offset(0 as libc::c_int as isize)).Freq as libc::c_int
            > (*s.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int
        {
            let mut s1: *mut CPpmd_State = s;
            let mut tmp_0: CPpmd_State = *s1;
            loop {
                *s1.offset(0 as libc::c_int as isize) = *s1.offset(-(1 as libc::c_int) as isize);
                s1 = s1.offset(-1);
                if !(s1 != stats
                    && tmp_0.Freq as libc::c_int
                        > (*s1.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int)
                {
                    break;
                }
            }
            *s1 = tmp_0
        }
        i = i.wrapping_sub(1);
        if !(i != 0) {
            break;
        }
    }
    if (*s).Freq as libc::c_int == 0 as libc::c_int {
        let mut numStats: libc::c_uint = (*(*p).MinContext).NumStats as libc::c_uint;
        let mut n0: libc::c_uint = 0;
        let mut n1: libc::c_uint = 0;
        loop {
            i = i.wrapping_add(1);
            s = s.offset(-1);
            if !((*s).Freq as libc::c_int == 0 as libc::c_int) {
                break;
            }
        }
        escFreq = escFreq.wrapping_add(i);
        (*(*p).MinContext).NumStats =
            ((*(*p).MinContext).NumStats as libc::c_uint).wrapping_sub(i) as UInt16;
        if (*(*p).MinContext).NumStats as libc::c_int == 1 as libc::c_int {
            let mut tmp_1: CPpmd_State = *stats;
            loop {
                tmp_1.Freq = (tmp_1.Freq as libc::c_int
                    - (tmp_1.Freq as libc::c_int >> 1 as libc::c_int))
                    as Byte;
                escFreq >>= 1 as libc::c_int;
                if !(escFreq > 1 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
            InsertNode(
                p,
                stats as *mut libc::c_void,
                (*p).Units2Indx[(numStats.wrapping_add(1 as libc::c_int as libc::c_uint)
                    >> 1 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as usize] as libc::c_uint,
            );
            (*p).FoundState = &mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State;
            *(*p).FoundState = tmp_1;
            return;
        }
        n0 = numStats.wrapping_add(1 as libc::c_int as libc::c_uint) >> 1 as libc::c_int;
        n1 = ((*(*p).MinContext).NumStats as libc::c_int + 1 as libc::c_int >> 1 as libc::c_int)
            as libc::c_uint;
        if n0 != n1 {
            (*(*p).MinContext).Stats =
                (ShrinkUnits(p, stats as *mut libc::c_void, n0, n1) as *mut Byte)
                    .offset_from((*p).Base) as libc::c_long as UInt32
        }
    }
    (*(*p).MinContext).SummFreq = sumFreq
        .wrapping_add(escFreq)
        .wrapping_sub(escFreq >> 1 as libc::c_int) as UInt16;
    (*p).FoundState = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
        as *mut CPpmd_State;
}
unsafe extern "C" fn Ppmd7_MakeEscFreq(
    mut p: *mut CPpmd7,
    mut numMasked: libc::c_uint,
    mut escFreq: *mut UInt32,
) -> *mut CPpmd_See {
    let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
    let mut nonMasked: libc::c_uint =
        ((*(*p).MinContext).NumStats as libc::c_uint).wrapping_sub(numMasked);
    if (*(*p).MinContext).NumStats as libc::c_int != 256 as libc::c_int {
        see = (*p).See[(*p).NS2Indx
            [nonMasked.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
            as usize]
            .as_mut_ptr()
            .offset(
                (nonMasked
                    < ((*((*p).Base.offset((*(*p).MinContext).Suffix as isize) as *mut libc::c_void
                        as *mut CPpmd7_Context))
                        .NumStats as libc::c_uint)
                        .wrapping_sub((*(*p).MinContext).NumStats as libc::c_uint))
                    as libc::c_int as isize,
            )
            .offset(
                (2 as libc::c_int
                    * (((*(*p).MinContext).SummFreq as libc::c_int)
                        < 11 as libc::c_int * (*(*p).MinContext).NumStats as libc::c_int)
                        as libc::c_int) as isize,
            )
            .offset((4 as libc::c_int * (numMasked > nonMasked) as libc::c_int) as isize)
            .offset((*p).HiBitsFlag as isize);
        let mut r: libc::c_uint =
            ((*see).Summ as libc::c_int >> (*see).Shift as libc::c_int) as libc::c_uint;
        (*see).Summ = ((*see).Summ as libc::c_uint).wrapping_sub(r) as UInt16;
        *escFreq =
            r.wrapping_add((r == 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_uint)
    } else {
        see = &mut (*p).DummySee;
        *escFreq = 1 as libc::c_int as UInt32
    }
    return see;
}
unsafe extern "C" fn NextContext(mut p: *mut CPpmd7) {
    let mut c: CTX_PTR = (*p).Base.offset(
        ((*(*p).FoundState).SuccessorLow as libc::c_uint
            | ((*(*p).FoundState).SuccessorHigh as UInt32) << 16 as libc::c_int) as isize,
    ) as *mut libc::c_void as *mut CPpmd7_Context;
    if (*p).OrderFall == 0 as libc::c_int as libc::c_uint && c as *mut Byte > (*p).Text {
        (*p).MaxContext = c;
        (*p).MinContext = (*p).MaxContext
    } else {
        UpdateModel(p);
    };
}
unsafe extern "C" fn Ppmd7_Update1(mut p: *mut CPpmd7) {
    let mut s: *mut CPpmd_State = (*p).FoundState;
    (*s).Freq = ((*s).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    (*(*p).MinContext).SummFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16;
    if (*s.offset(0 as libc::c_int as isize)).Freq as libc::c_int
        > (*s.offset(-(1 as libc::c_int) as isize)).Freq as libc::c_int
    {
        SwapStates(
            &mut *s.offset(0 as libc::c_int as isize),
            &mut *s.offset(-(1 as libc::c_int) as isize),
        );
        s = s.offset(-1);
        (*p).FoundState = s;
        if (*s).Freq as libc::c_int > MAX_FREQ {
            Rescale(p);
        }
    }
    NextContext(p);
}
unsafe extern "C" fn Ppmd7_Update1_0(mut p: *mut CPpmd7) {
    (*p).PrevSuccess = (2 as libc::c_int * (*(*p).FoundState).Freq as libc::c_int
        > (*(*p).MinContext).SummFreq as libc::c_int) as libc::c_int
        as libc::c_uint;
    (*p).RunLength =
        ((*p).RunLength as libc::c_uint).wrapping_add((*p).PrevSuccess) as Int32 as Int32;
    (*(*p).MinContext).SummFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16;
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    if (*(*p).FoundState).Freq as libc::c_int > MAX_FREQ {
        Rescale(p);
    }
    NextContext(p);
}
unsafe extern "C" fn Ppmd7_UpdateBin(mut p: *mut CPpmd7) {
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as libc::c_int
        + (if ((*(*p).FoundState).Freq as libc::c_int) < 128 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        })) as Byte;
    (*p).PrevSuccess = 1 as libc::c_int as libc::c_uint;
    (*p).RunLength += 1;
    NextContext(p);
}
unsafe extern "C" fn Ppmd7_Update2(mut p: *mut CPpmd7) {
    (*(*p).MinContext).SummFreq =
        ((*(*p).MinContext).SummFreq as libc::c_int + 4 as libc::c_int) as UInt16;
    (*(*p).FoundState).Freq = ((*(*p).FoundState).Freq as libc::c_int + 4 as libc::c_int) as Byte;
    if (*(*p).FoundState).Freq as libc::c_int > MAX_FREQ {
        Rescale(p);
    }
    (*p).RunLength = (*p).InitRL;
    UpdateModel(p);
}
/* ---------- Decode ---------- */
unsafe extern "C" fn Ppmd_RangeDec_Init(mut p: *mut CPpmd7z_RangeDec) -> Bool {
    let mut i: libc::c_uint = 0;
    (*p).Bottom = 0 as libc::c_int as UInt32;
    (*p).Low = (*p).Bottom;
    (*p).Range = 0xffffffff as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        (*p).Code = (*p).Code << 8 as libc::c_int
            | (*(*p).Stream).Read.expect("non-null function pointer")(
                (*p).Stream as *mut libc::c_void,
            ) as libc::c_uint;
        i = i.wrapping_add(1)
    }
    return ((*p).Code < 0xffffffff as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn Ppmd7z_RangeDec_Init(mut p: *mut CPpmd7z_RangeDec) -> Bool {
    if (*(*p).Stream).Read.expect("non-null function pointer")((*p).Stream as *mut libc::c_void)
        as libc::c_int
        != 0 as libc::c_int
    {
        return False;
    }
    return Ppmd_RangeDec_Init(p);
}
unsafe extern "C" fn PpmdRAR_RangeDec_Init(mut p: *mut CPpmd7z_RangeDec) -> Bool {
    if Ppmd_RangeDec_Init(p) == 0 {
        return False;
    }
    (*p).Bottom = 0x8000 as libc::c_int as UInt32;
    return True;
}
unsafe extern "C" fn Range_GetThreshold(mut pp: *mut libc::c_void, mut total: UInt32) -> UInt32 {
    let mut p: *mut CPpmd7z_RangeDec = pp as *mut CPpmd7z_RangeDec;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_div(total) as UInt32 as UInt32;
    return (*p).Code.wrapping_sub((*p).Low).wrapping_div((*p).Range);
}
unsafe extern "C" fn Range_Normalize(mut p: *mut CPpmd7z_RangeDec) {
    loop {
        if (*p).Low ^ (*p).Low.wrapping_add((*p).Range) >= kTopValue_0 as libc::c_uint {
            if (*p).Range >= (*p).Bottom {
                break;
            }
            (*p).Range = -((*p).Low as int32_t) as uint32_t
                & (*p).Bottom.wrapping_sub(1 as libc::c_int as libc::c_uint)
        }
        (*p).Code = (*p).Code << 8 as libc::c_int
            | (*(*p).Stream).Read.expect("non-null function pointer")(
                (*p).Stream as *mut libc::c_void,
            ) as libc::c_uint;
        (*p).Range <<= 8 as libc::c_int;
        (*p).Low <<= 8 as libc::c_int
    }
}
unsafe extern "C" fn Range_Decode_7z(
    mut pp: *mut libc::c_void,
    mut start: UInt32,
    mut size: UInt32,
) {
    let mut p: *mut CPpmd7z_RangeDec = pp as *mut CPpmd7z_RangeDec;
    (*p).Code = ((*p).Code as libc::c_uint).wrapping_sub(start.wrapping_mul((*p).Range)) as UInt32
        as UInt32;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
    Range_Normalize(p);
}
unsafe extern "C" fn Range_Decode_RAR(
    mut pp: *mut libc::c_void,
    mut start: UInt32,
    mut size: UInt32,
) {
    let mut p: *mut CPpmd7z_RangeDec = pp as *mut CPpmd7z_RangeDec;
    (*p).Low =
        ((*p).Low as libc::c_uint).wrapping_add(start.wrapping_mul((*p).Range)) as UInt32 as UInt32;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
    Range_Normalize(p);
}
unsafe extern "C" fn Range_DecodeBit_7z(mut pp: *mut libc::c_void, mut size0: UInt32) -> UInt32 {
    let mut p: *mut CPpmd7z_RangeDec = pp as *mut CPpmd7z_RangeDec;
    let mut newBound: UInt32 = ((*p).Range >> 14 as libc::c_int).wrapping_mul(size0);
    let mut symbol: UInt32 = 0;
    if (*p).Code < newBound {
        symbol = 0 as libc::c_int as UInt32;
        (*p).Range = newBound
    } else {
        symbol = 1 as libc::c_int as UInt32;
        (*p).Code = ((*p).Code as libc::c_uint).wrapping_sub(newBound) as UInt32 as UInt32;
        (*p).Range = ((*p).Range as libc::c_uint).wrapping_sub(newBound) as UInt32 as UInt32
    }
    Range_Normalize(p);
    return symbol;
}
unsafe extern "C" fn Range_DecodeBit_RAR(mut pp: *mut libc::c_void, mut size0: UInt32) -> UInt32 {
    let mut p: *mut CPpmd7z_RangeDec = pp as *mut CPpmd7z_RangeDec;
    let mut bit: UInt32 = 0;
    let mut value: UInt32 = (*p).p.GetThreshold.expect("non-null function pointer")(
        p as *mut libc::c_void,
        PPMD_BIN_SCALE as UInt32,
    );
    if value < size0 {
        bit = 0 as libc::c_int as UInt32;
        (*p).p.Decode.expect("non-null function pointer")(
            p as *mut libc::c_void,
            0 as libc::c_int as UInt32,
            size0,
        );
    } else {
        bit = 1 as libc::c_int as UInt32;
        (*p).p.Decode.expect("non-null function pointer")(
            p as *mut libc::c_void,
            size0,
            (PPMD_BIN_SCALE as libc::c_uint).wrapping_sub(size0),
        );
    }
    return bit;
}
unsafe extern "C" fn Ppmd7z_RangeDec_CreateVTable(mut p: *mut CPpmd7z_RangeDec) {
    (*p).p.GetThreshold =
        Some(Range_GetThreshold as unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32);
    (*p).p.Decode = Some(
        Range_Decode_7z as unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32, _: UInt32) -> (),
    );
    (*p).p.DecodeBit =
        Some(Range_DecodeBit_7z as unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32);
}
unsafe extern "C" fn PpmdRAR_RangeDec_CreateVTable(mut p: *mut CPpmd7z_RangeDec) {
    (*p).p.GetThreshold =
        Some(Range_GetThreshold as unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32);
    (*p).p.Decode = Some(
        Range_Decode_RAR as unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32, _: UInt32) -> (),
    );
    (*p).p.DecodeBit = Some(
        Range_DecodeBit_RAR as unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32,
    );
}
unsafe extern "C" fn Ppmd7_DecodeSymbol(
    mut p: *mut CPpmd7,
    mut rc: *mut IPpmd7_RangeDec,
) -> libc::c_int {
    let mut charMask: [size_t; 32] = [0; 32];
    if (*(*p).MinContext).NumStats as libc::c_int != 1 as libc::c_int {
        let mut s: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut i: libc::c_uint = 0;
        let mut count: UInt32 = 0;
        let mut hiCnt: UInt32 = 0;
        count = (*rc).GetThreshold.expect("non-null function pointer")(
            rc as *mut libc::c_void,
            (*(*p).MinContext).SummFreq as UInt32,
        );
        hiCnt = (*s).Freq as UInt32;
        if count < hiCnt {
            let mut symbol: Byte = 0;
            (*rc).Decode.expect("non-null function pointer")(
                rc as *mut libc::c_void,
                0 as libc::c_int as UInt32,
                (*s).Freq as UInt32,
            );
            (*p).FoundState = s;
            symbol = (*s).Symbol;
            Ppmd7_Update1_0(p);
            return symbol as libc::c_int;
        }
        (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint;
        i = ((*(*p).MinContext).NumStats as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        loop {
            s = s.offset(1);
            hiCnt =
                (hiCnt as libc::c_uint).wrapping_add((*s).Freq as libc::c_uint) as UInt32 as UInt32;
            if hiCnt > count {
                let mut symbol_0: Byte = 0;
                (*rc).Decode.expect("non-null function pointer")(
                    rc as *mut libc::c_void,
                    hiCnt.wrapping_sub((*s).Freq as libc::c_uint),
                    (*s).Freq as UInt32,
                );
                (*p).FoundState = s;
                symbol_0 = (*s).Symbol;
                Ppmd7_Update1(p);
                return symbol_0 as libc::c_int;
            }
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        if count >= (*(*p).MinContext).SummFreq as libc::c_uint {
            return -(2 as libc::c_int);
        }
        (*p).HiBitsFlag = (*p).HB2Flag[(*(*p).FoundState).Symbol as usize] as libc::c_uint;
        (*rc).Decode.expect("non-null function pointer")(
            rc as *mut libc::c_void,
            hiCnt,
            ((*(*p).MinContext).SummFreq as libc::c_uint).wrapping_sub(hiCnt),
        );
        let mut j: libc::c_uint = 0;
        j = 0 as libc::c_int as libc::c_uint;
        while (j as libc::c_ulong)
            < (256 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[j.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
                !(0 as libc::c_int as size_t);
            charMask[j.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(0 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(2 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(1 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(2 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(4 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(3 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(5 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(4 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(6 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(5 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(7 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(6 as libc::c_int as libc::c_uint) as usize];
            j = j.wrapping_add(8 as libc::c_int as libc::c_uint)
        }
        *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
            0 as libc::c_int as libc::c_schar;
        i = ((*(*p).MinContext).NumStats as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        loop {
            s = s.offset(-1);
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
                0 as libc::c_int as libc::c_schar;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
    } else {
        (*p).HiBitsFlag = *(*p)
            .HB2Flag
            .as_mut_ptr()
            .offset((*(*p).FoundState).Symbol as isize) as libc::c_uint;
        let mut prob: *mut UInt16 = &mut *(*(*p).BinSumm.as_mut_ptr().offset(
            ((*(&mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq
                as libc::c_int
                - 1 as libc::c_int) as isize,
        ))
        .as_mut_ptr()
        .offset(
            (*p).PrevSuccess
                .wrapping_add(
                    *(*p).NS2BSIndx.as_mut_ptr().offset(
                        ((*((*p).Base.offset((*(*p).MinContext).Suffix as isize)
                            as *mut libc::c_void
                            as *mut CPpmd7_Context))
                            .NumStats as libc::c_int
                            - 1 as libc::c_int) as isize,
                    ) as libc::c_uint,
                )
                .wrapping_add((*p).HiBitsFlag)
                .wrapping_add(
                    (2 as libc::c_int
                        * *(*p).HB2Flag.as_mut_ptr().offset(
                            (*(&mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State))
                                .Symbol as isize,
                        ) as libc::c_int) as libc::c_uint,
                )
                .wrapping_add(
                    ((*p).RunLength >> 26 as libc::c_int & 0x20 as libc::c_int) as libc::c_uint,
                ) as isize,
        ) as *mut UInt16;
        if (*rc).DecodeBit.expect("non-null function pointer")(
            rc as *mut libc::c_void,
            *prob as UInt32,
        ) == 0 as libc::c_int as libc::c_uint
        {
            let mut symbol_1: Byte = 0;
            *prob = (*prob as libc::c_int + ((1 as libc::c_int) << PPMD_INT_BITS)
                - (*prob as libc::c_int
                    + ((1 as libc::c_int) << 7 as libc::c_int - 2 as libc::c_int)
                    >> 7 as libc::c_int)) as UInt16;
            (*p).FoundState = &mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State;
            symbol_1 = (*(*p).FoundState).Symbol;
            Ppmd7_UpdateBin(p);
            return symbol_1 as libc::c_int;
        }
        *prob = (*prob as libc::c_int
            - (*prob as libc::c_int + ((1 as libc::c_int) << 7 as libc::c_int - 2 as libc::c_int)
                >> 7 as libc::c_int)) as UInt16;
        (*p).InitEsc =
            PPMD7_kExpEscape[(*prob as libc::c_int >> 10 as libc::c_int) as usize] as libc::c_uint;
        let mut j_0: libc::c_uint = 0;
        j_0 = 0 as libc::c_int as libc::c_uint;
        while (j_0 as libc::c_ulong)
            < (256 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[j_0.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
                !(0 as libc::c_int as size_t);
            charMask[j_0.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
                charMask[j_0.wrapping_add(0 as libc::c_int as libc::c_uint) as usize];
            charMask[j_0.wrapping_add(2 as libc::c_int as libc::c_uint) as usize] =
                charMask[j_0.wrapping_add(1 as libc::c_int as libc::c_uint) as usize];
            charMask[j_0.wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
                charMask[j_0.wrapping_add(2 as libc::c_int as libc::c_uint) as usize];
            charMask[j_0.wrapping_add(4 as libc::c_int as libc::c_uint) as usize] =
                charMask[j_0.wrapping_add(3 as libc::c_int as libc::c_uint) as usize];
            charMask[j_0.wrapping_add(5 as libc::c_int as libc::c_uint) as usize] =
                charMask[j_0.wrapping_add(4 as libc::c_int as libc::c_uint) as usize];
            charMask[j_0.wrapping_add(6 as libc::c_int as libc::c_uint) as usize] =
                charMask[j_0.wrapping_add(5 as libc::c_int as libc::c_uint) as usize];
            charMask[j_0.wrapping_add(7 as libc::c_int as libc::c_uint) as usize] =
                charMask[j_0.wrapping_add(6 as libc::c_int as libc::c_uint) as usize];
            j_0 = j_0.wrapping_add(8 as libc::c_int as libc::c_uint)
        }
        *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(
            (*(&mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State)).Symbol
                as isize,
        ) = 0 as libc::c_int as libc::c_schar;
        (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint
    }
    loop {
        let mut ps: [*mut CPpmd_State; 256] = [0 as *mut CPpmd_State; 256];
        let mut s_0: *mut CPpmd_State = 0 as *mut CPpmd_State;
        let mut freqSum: UInt32 = 0;
        let mut count_0: UInt32 = 0;
        let mut hiCnt_0: UInt32 = 0;
        let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
        let mut i_0: libc::c_uint = 0;
        let mut num: libc::c_uint = 0;
        let mut numMasked: libc::c_uint = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
            if (*(*p).MinContext).Suffix == 0 {
                return -(1 as libc::c_int);
            }
            (*p).MinContext = (*p).Base.offset((*(*p).MinContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd7_Context;
            if !((*(*p).MinContext).NumStats as libc::c_uint == numMasked) {
                break;
            }
        }
        hiCnt_0 = 0 as libc::c_int as UInt32;
        s_0 = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        i_0 = 0 as libc::c_int as libc::c_uint;
        num = ((*(*p).MinContext).NumStats as libc::c_uint).wrapping_sub(numMasked);
        loop {
            let mut k: libc::c_int = *(charMask.as_mut_ptr() as *mut libc::c_schar)
                .offset((*s_0).Symbol as isize) as libc::c_int;
            hiCnt_0 = (hiCnt_0 as libc::c_uint)
                .wrapping_add(((*s_0).Freq as libc::c_int & k) as libc::c_uint)
                as UInt32 as UInt32;
            let fresh5 = s_0;
            s_0 = s_0.offset(1);
            ps[i_0 as usize] = fresh5;
            i_0 = i_0.wrapping_sub(k as libc::c_uint);
            if !(i_0 != num) {
                break;
            }
        }
        see = Ppmd7_MakeEscFreq(p, numMasked, &mut freqSum);
        freqSum = (freqSum as libc::c_uint).wrapping_add(hiCnt_0) as UInt32 as UInt32;
        count_0 = (*rc).GetThreshold.expect("non-null function pointer")(
            rc as *mut libc::c_void,
            freqSum,
        );
        if count_0 < hiCnt_0 {
            let mut symbol_2: Byte = 0;
            let mut pps: *mut *mut CPpmd_State = ps.as_mut_ptr();
            hiCnt_0 = 0 as libc::c_int as UInt32;
            loop {
                hiCnt_0 = (hiCnt_0 as libc::c_uint).wrapping_add((**pps).Freq as libc::c_uint)
                    as UInt32 as UInt32;
                if !(hiCnt_0 <= count_0) {
                    break;
                }
                pps = pps.offset(1)
            }
            s_0 = *pps;
            (*rc).Decode.expect("non-null function pointer")(
                rc as *mut libc::c_void,
                hiCnt_0.wrapping_sub((*s_0).Freq as libc::c_uint),
                (*s_0).Freq as UInt32,
            );
            if ((*see).Shift as libc::c_int) < PPMD_PERIOD_BITS && {
                (*see).Count = (*see).Count.wrapping_sub(1);
                ((*see).Count as libc::c_int) == 0 as libc::c_int
            } {
                (*see).Summ = (((*see).Summ as libc::c_int) << 1 as libc::c_int) as UInt16;
                let fresh6 = (*see).Shift;
                (*see).Shift = (*see).Shift.wrapping_add(1);
                (*see).Count = ((3 as libc::c_int) << fresh6 as libc::c_int) as Byte
            }
            (*p).FoundState = s_0;
            symbol_2 = (*s_0).Symbol;
            Ppmd7_Update2(p);
            return symbol_2 as libc::c_int;
        }
        if count_0 >= freqSum {
            return -(2 as libc::c_int);
        }
        (*rc).Decode.expect("non-null function pointer")(
            rc as *mut libc::c_void,
            hiCnt_0,
            freqSum.wrapping_sub(hiCnt_0),
        );
        (*see).Summ = ((*see).Summ as libc::c_uint).wrapping_add(freqSum) as UInt16;
        loop {
            i_0 = i_0.wrapping_sub(1);
            *(charMask.as_mut_ptr() as *mut libc::c_schar)
                .offset((*ps[i_0 as usize]).Symbol as isize) = 0 as libc::c_int as libc::c_schar;
            if !(i_0 != 0 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
}
/* ---------- Encode ---------- Ppmd7Enc.c */
pub const kTopValue: libc::c_int = (1 as libc::c_int) << 24 as libc::c_int;
unsafe extern "C" fn Ppmd7z_RangeEnc_Init(mut p: *mut CPpmd7z_RangeEnc) {
    (*p).Low = 0 as libc::c_int as UInt64; /* EndMarker (symbol = -1) */
    (*p).Range = 0xffffffff as libc::c_uint;
    (*p).Cache = 0 as libc::c_int as Byte;
    (*p).CacheSize = 1 as libc::c_int as UInt64;
}
unsafe extern "C" fn RangeEnc_ShiftLow(mut p: *mut CPpmd7z_RangeEnc) {
    if ((*p).Low as UInt32) < 0xff000000 as libc::c_uint
        || ((*p).Low >> 32 as libc::c_int) as libc::c_uint != 0 as libc::c_int as libc::c_uint
    {
        let mut temp: Byte = (*p).Cache;
        loop {
            (*(*p).Stream).Write.expect("non-null function pointer")(
                (*p).Stream as *mut libc::c_void,
                (temp as libc::c_int + ((*p).Low >> 32 as libc::c_int) as Byte as libc::c_int)
                    as Byte,
            );
            temp = 0xff as libc::c_int as Byte;
            (*p).CacheSize = (*p).CacheSize.wrapping_sub(1);
            if !((*p).CacheSize != 0 as libc::c_int as libc::c_ulonglong) {
                break;
            }
        }
        (*p).Cache = ((*p).Low as UInt32 >> 24 as libc::c_int) as Byte
    }
    (*p).CacheSize = (*p).CacheSize.wrapping_add(1);
    (*p).Low = (((*p).Low as UInt32) << 8 as libc::c_int & 0xffffffff as libc::c_uint) as UInt64;
}
unsafe extern "C" fn RangeEnc_Encode(
    mut p: *mut CPpmd7z_RangeEnc,
    mut start: UInt32,
    mut size: UInt32,
    mut total: UInt32,
) {
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_div(total) as UInt32 as UInt32;
    (*p).Low = ((*p).Low as libc::c_ulonglong)
        .wrapping_add((start as UInt64).wrapping_mul((*p).Range as UInt64)) as UInt64
        as UInt64;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_mul(size) as UInt32 as UInt32;
    while (*p).Range < kTopValue as libc::c_uint {
        (*p).Range <<= 8 as libc::c_int;
        RangeEnc_ShiftLow(p);
    }
}
unsafe extern "C" fn RangeEnc_EncodeBit_0(mut p: *mut CPpmd7z_RangeEnc, mut size0: UInt32) {
    (*p).Range = ((*p).Range >> 14 as libc::c_int).wrapping_mul(size0);
    while (*p).Range < kTopValue as libc::c_uint {
        (*p).Range <<= 8 as libc::c_int;
        RangeEnc_ShiftLow(p);
    }
}
unsafe extern "C" fn RangeEnc_EncodeBit_1(mut p: *mut CPpmd7z_RangeEnc, mut size0: UInt32) {
    let mut newBound: UInt32 = ((*p).Range >> 14 as libc::c_int).wrapping_mul(size0);
    (*p).Low = ((*p).Low as libc::c_ulonglong).wrapping_add(newBound as libc::c_ulonglong) as UInt64
        as UInt64;
    (*p).Range = ((*p).Range as libc::c_uint).wrapping_sub(newBound) as UInt32 as UInt32;
    while (*p).Range < kTopValue as libc::c_uint {
        (*p).Range <<= 8 as libc::c_int;
        RangeEnc_ShiftLow(p);
    }
}
unsafe extern "C" fn Ppmd7z_RangeEnc_FlushData(mut p: *mut CPpmd7z_RangeEnc) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 5 as libc::c_int as libc::c_uint {
        RangeEnc_ShiftLow(p);
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn Ppmd7_EncodeSymbol(
    mut p: *mut CPpmd7,
    mut rc: *mut CPpmd7z_RangeEnc,
    mut symbol: libc::c_int,
) {
    let mut charMask: [size_t; 32] = [0; 32];
    if (*(*p).MinContext).NumStats as libc::c_int != 1 as libc::c_int {
        let mut s: *mut CPpmd_State = (*p).Base.offset((*(*p).MinContext).Stats as isize)
            as *mut libc::c_void as *mut CPpmd_State;
        let mut sum: UInt32 = 0;
        let mut i: libc::c_uint = 0;
        if (*s).Symbol as libc::c_int == symbol {
            RangeEnc_Encode(
                rc,
                0 as libc::c_int as UInt32,
                (*s).Freq as UInt32,
                (*(*p).MinContext).SummFreq as UInt32,
            );
            (*p).FoundState = s;
            Ppmd7_Update1_0(p);
            return;
        }
        (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint;
        sum = (*s).Freq as UInt32;
        i = ((*(*p).MinContext).NumStats as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        loop {
            s = s.offset(1);
            if (*s).Symbol as libc::c_int == symbol {
                RangeEnc_Encode(
                    rc,
                    sum,
                    (*s).Freq as UInt32,
                    (*(*p).MinContext).SummFreq as UInt32,
                );
                (*p).FoundState = s;
                Ppmd7_Update1(p);
                return;
            }
            sum = (sum as libc::c_uint).wrapping_add((*s).Freq as libc::c_uint) as UInt32 as UInt32;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        (*p).HiBitsFlag = (*p).HB2Flag[(*(*p).FoundState).Symbol as usize] as libc::c_uint;
        let mut j: libc::c_uint = 0;
        j = 0 as libc::c_int as libc::c_uint;
        while (j as libc::c_ulong)
            < (256 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
        {
            charMask[j.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
                !(0 as libc::c_int as size_t);
            charMask[j.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(0 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(2 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(1 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(2 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(4 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(3 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(5 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(4 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(6 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(5 as libc::c_int as libc::c_uint) as usize];
            charMask[j.wrapping_add(7 as libc::c_int as libc::c_uint) as usize] =
                charMask[j.wrapping_add(6 as libc::c_int as libc::c_uint) as usize];
            j = j.wrapping_add(8 as libc::c_int as libc::c_uint)
        }
        *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
            0 as libc::c_int as libc::c_schar;
        i = ((*(*p).MinContext).NumStats as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        loop {
            s = s.offset(-1);
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s).Symbol as isize) =
                0 as libc::c_int as libc::c_schar;
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        RangeEnc_Encode(
            rc,
            sum,
            ((*(*p).MinContext).SummFreq as libc::c_uint).wrapping_sub(sum),
            (*(*p).MinContext).SummFreq as UInt32,
        );
    } else {
        (*p).HiBitsFlag = *(*p)
            .HB2Flag
            .as_mut_ptr()
            .offset((*(*p).FoundState).Symbol as isize) as libc::c_uint;
        let mut prob: *mut UInt16 = &mut *(*(*p).BinSumm.as_mut_ptr().offset(
            ((*(&mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State)).Freq
                as libc::c_int
                - 1 as libc::c_int) as isize,
        ))
        .as_mut_ptr()
        .offset(
            (*p).PrevSuccess
                .wrapping_add(
                    *(*p).NS2BSIndx.as_mut_ptr().offset(
                        ((*((*p).Base.offset((*(*p).MinContext).Suffix as isize)
                            as *mut libc::c_void
                            as *mut CPpmd7_Context))
                            .NumStats as libc::c_int
                            - 1 as libc::c_int) as isize,
                    ) as libc::c_uint,
                )
                .wrapping_add((*p).HiBitsFlag)
                .wrapping_add(
                    (2 as libc::c_int
                        * *(*p).HB2Flag.as_mut_ptr().offset(
                            (*(&mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State))
                                .Symbol as isize,
                        ) as libc::c_int) as libc::c_uint,
                )
                .wrapping_add(
                    ((*p).RunLength >> 26 as libc::c_int & 0x20 as libc::c_int) as libc::c_uint,
                ) as isize,
        ) as *mut UInt16;
        let mut s_0: *mut CPpmd_State =
            &mut (*(*p).MinContext).SummFreq as *mut UInt16 as *mut CPpmd_State;
        if (*s_0).Symbol as libc::c_int == symbol {
            RangeEnc_EncodeBit_0(rc, *prob as UInt32);
            *prob = (*prob as libc::c_int + ((1 as libc::c_int) << PPMD_INT_BITS)
                - (*prob as libc::c_int
                    + ((1 as libc::c_int) << 7 as libc::c_int - 2 as libc::c_int)
                    >> 7 as libc::c_int)) as UInt16;
            (*p).FoundState = s_0;
            Ppmd7_UpdateBin(p);
            return;
        } else {
            RangeEnc_EncodeBit_1(rc, *prob as UInt32);
            *prob = (*prob as libc::c_int
                - (*prob as libc::c_int
                    + ((1 as libc::c_int) << 7 as libc::c_int - 2 as libc::c_int)
                    >> 7 as libc::c_int)) as UInt16;
            (*p).InitEsc = PPMD7_kExpEscape[(*prob as libc::c_int >> 10 as libc::c_int) as usize]
                as libc::c_uint;
            let mut j_0: libc::c_uint = 0;
            j_0 = 0 as libc::c_int as libc::c_uint;
            while (j_0 as libc::c_ulong)
                < (256 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<size_t>() as libc::c_ulong)
            {
                charMask[j_0.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
                    !(0 as libc::c_int as size_t);
                charMask[j_0.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
                    charMask[j_0.wrapping_add(0 as libc::c_int as libc::c_uint) as usize];
                charMask[j_0.wrapping_add(2 as libc::c_int as libc::c_uint) as usize] =
                    charMask[j_0.wrapping_add(1 as libc::c_int as libc::c_uint) as usize];
                charMask[j_0.wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
                    charMask[j_0.wrapping_add(2 as libc::c_int as libc::c_uint) as usize];
                charMask[j_0.wrapping_add(4 as libc::c_int as libc::c_uint) as usize] =
                    charMask[j_0.wrapping_add(3 as libc::c_int as libc::c_uint) as usize];
                charMask[j_0.wrapping_add(5 as libc::c_int as libc::c_uint) as usize] =
                    charMask[j_0.wrapping_add(4 as libc::c_int as libc::c_uint) as usize];
                charMask[j_0.wrapping_add(6 as libc::c_int as libc::c_uint) as usize] =
                    charMask[j_0.wrapping_add(5 as libc::c_int as libc::c_uint) as usize];
                charMask[j_0.wrapping_add(7 as libc::c_int as libc::c_uint) as usize] =
                    charMask[j_0.wrapping_add(6 as libc::c_int as libc::c_uint) as usize];
                j_0 = j_0.wrapping_add(8 as libc::c_int as libc::c_uint)
            }
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset((*s_0).Symbol as isize) =
                0 as libc::c_int as libc::c_schar;
            (*p).PrevSuccess = 0 as libc::c_int as libc::c_uint
        }
    }
    loop {
        let mut escFreq: UInt32 = 0;
        let mut see: *mut CPpmd_See = 0 as *mut CPpmd_See;
        let mut s_1: *mut CPpmd_State = 0 as *mut CPpmd_State;
        let mut sum_0: UInt32 = 0;
        let mut i_0: libc::c_uint = 0;
        let mut numMasked: libc::c_uint = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            (*p).OrderFall = (*p).OrderFall.wrapping_add(1);
            if (*(*p).MinContext).Suffix == 0 {
                return;
            }
            (*p).MinContext = (*p).Base.offset((*(*p).MinContext).Suffix as isize)
                as *mut libc::c_void as *mut CPpmd7_Context;
            if !((*(*p).MinContext).NumStats as libc::c_uint == numMasked) {
                break;
            }
        }
        see = Ppmd7_MakeEscFreq(p, numMasked, &mut escFreq);
        s_1 = (*p).Base.offset((*(*p).MinContext).Stats as isize) as *mut libc::c_void
            as *mut CPpmd_State;
        sum_0 = 0 as libc::c_int as UInt32;
        i_0 = (*(*p).MinContext).NumStats as libc::c_uint;
        loop {
            let mut cur: libc::c_int = (*s_1).Symbol as libc::c_int;
            if cur == symbol {
                let mut low: UInt32 = sum_0;
                let mut s1: *mut CPpmd_State = s_1;
                loop {
                    sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                        ((*s_1).Freq as libc::c_int
                            & *(charMask.as_mut_ptr() as *mut libc::c_schar)
                                .offset((*s_1).Symbol as isize)
                                as libc::c_int) as libc::c_uint,
                    ) as UInt32 as UInt32;
                    s_1 = s_1.offset(1);
                    i_0 = i_0.wrapping_sub(1);
                    if !(i_0 != 0) {
                        break;
                    }
                }
                RangeEnc_Encode(rc, low, (*s1).Freq as UInt32, sum_0.wrapping_add(escFreq));
                if ((*see).Shift as libc::c_int) < PPMD_PERIOD_BITS && {
                    (*see).Count = (*see).Count.wrapping_sub(1);
                    ((*see).Count as libc::c_int) == 0 as libc::c_int
                } {
                    (*see).Summ = (((*see).Summ as libc::c_int) << 1 as libc::c_int) as UInt16;
                    let fresh7 = (*see).Shift;
                    (*see).Shift = (*see).Shift.wrapping_add(1);
                    (*see).Count = ((3 as libc::c_int) << fresh7 as libc::c_int) as Byte
                }
                (*p).FoundState = s1;
                Ppmd7_Update2(p);
                return;
            }
            sum_0 = (sum_0 as libc::c_uint).wrapping_add(
                ((*s_1).Freq as libc::c_int
                    & *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize)
                        as libc::c_int) as libc::c_uint,
            ) as UInt32 as UInt32;
            *(charMask.as_mut_ptr() as *mut libc::c_schar).offset(cur as isize) =
                0 as libc::c_int as libc::c_schar;
            s_1 = s_1.offset(1);
            i_0 = i_0.wrapping_sub(1);
            if !(i_0 != 0) {
                break;
            }
        }
        RangeEnc_Encode(rc, sum_0, escFreq, sum_0.wrapping_add(escFreq));
        (*see).Summ = ((*see).Summ as libc::c_uint)
            .wrapping_add(sum_0)
            .wrapping_add(escFreq) as UInt16
    }
}
#[no_mangle]
pub static mut __archive_ppmd7_functions: IPpmd7 = {
    let mut init = IPpmd7 {
        Ppmd7_Construct: Some(Ppmd7_Construct as unsafe extern "C" fn(_: *mut CPpmd7) -> ()),
        Ppmd7_Alloc: Some(Ppmd7_Alloc as unsafe extern "C" fn(_: *mut CPpmd7, _: UInt32) -> Bool),
        Ppmd7_Free: Some(Ppmd7_Free as unsafe extern "C" fn(_: *mut CPpmd7) -> ()),
        Ppmd7_Init: Some(Ppmd7_Init as unsafe extern "C" fn(_: *mut CPpmd7, _: libc::c_uint) -> ()),
        Ppmd7z_RangeDec_CreateVTable: Some(
            Ppmd7z_RangeDec_CreateVTable as unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> (),
        ),
        PpmdRAR_RangeDec_CreateVTable: Some(
            PpmdRAR_RangeDec_CreateVTable as unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> (),
        ),
        Ppmd7z_RangeDec_Init: Some(
            Ppmd7z_RangeDec_Init as unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> Bool,
        ),
        PpmdRAR_RangeDec_Init: Some(
            PpmdRAR_RangeDec_Init as unsafe extern "C" fn(_: *mut CPpmd7z_RangeDec) -> Bool,
        ),
        Ppmd7_DecodeSymbol: Some(
            Ppmd7_DecodeSymbol
                as unsafe extern "C" fn(_: *mut CPpmd7, _: *mut IPpmd7_RangeDec) -> libc::c_int,
        ),
        Ppmd7z_RangeEnc_Init: Some(
            Ppmd7z_RangeEnc_Init as unsafe extern "C" fn(_: *mut CPpmd7z_RangeEnc) -> (),
        ),
        Ppmd7z_RangeEnc_FlushData: Some(
            Ppmd7z_RangeEnc_FlushData as unsafe extern "C" fn(_: *mut CPpmd7z_RangeEnc) -> (),
        ),
        Ppmd7_EncodeSymbol: Some(
            Ppmd7_EncodeSymbol
                as unsafe extern "C" fn(
                    _: *mut CPpmd7,
                    _: *mut CPpmd7z_RangeEnc,
                    _: libc::c_int,
                ) -> (),
        ),
    };
    init
};
