use ::libc;
extern "C" {
    pub type internal_state;
    pub type archive_string_conv;
    /* Declare our basic types. */
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
    pub type archive_write;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_copy_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_is_data_encrypted(_: *mut archive_entry, is_encrypted: libc::c_char);
    #[no_mangle]
    fn archive_entry_set_is_metadata_encrypted(_: *mut archive_entry, is_encrypted: libc::c_char);
    #[no_mangle]
    fn _archive_entry_copy_pathname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
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
    /* Configuration data for the bidder. */
    /* Name of the filter */
    /* Taste the upstream filter to see if we handle this. */
    /* Initialize a newly-created filter. */
    /* Set an option for the filter bidder. */
    /* Release the bidder's configuration data. */
    /*
     * This structure is allocated within the archive_read core
     * and initialized by archive_read and the init() method of the
     * corresponding bidder above.
     */
    /* Essentially all filters will need these values, so
     * just declare them here. */
    /* My bidder. */
    /* Who I read from. */
    /* Associated archive. */
    /* Open a block for reading */
    /* Return next block. */
    /* Skip forward this many bytes. */
    /* Seek to an absolute location. */
    /* Close (just this filter) and free(self). */
    /* Function that handles switching from reading one block to the next/prev */
    /* Read any header metadata if available. */
    /* My private data. */
    /* Used by reblocking logic. */
    /* Current read location. */
    /* Bytes in my buffer. */
    /* Client buffer information. */
    /*
     * The client looks a lot like a filter, so we just wrap it here.
     *
     * TODO: Make archive_read_filter and archive_read_client identical so
     * that users of the library can easily register their own
     * transformation filters.  This will probably break the API/ABI and
     * so should be deferred at least until libarchive 3.0.
     */
    /* archive_write_disk object */
    /* Progress function invoked during extract. */
    /* Dev/ino of the archive being read/written. */
    /* Callbacks to open/read/write/close client archive streams. */
    /* Registered filter bidders. */
    /* Last filter in chain */
    /* Whether to bypass filter bidding process */
    /* File offset of beginning of most recently-read header. */
    /* Nodes and offsets of compressed data block */
    /*
     * Format detection is mostly the same as compression
     * detection, with one significant difference: The bidders
     * use the read_ahead calls above to examine the stream rather
     * than having the supervisor hand them a block of data to
     * examine.
     */
    /* Active format. */
    /*
     * Various information needed by archive_extract.
     */
    /*
     * Decryption passphrase.
     */
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
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn __archive_read_seek(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t;
    #[no_mangle]
    fn __archive_read_ahead(
        _: *mut archive_read,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
    /* must be 32-bit at least */
    /* ---------- Decode ---------- */
    /* ---------- Encode ---------- */
    /* Base Functions */
    /* Decode Functions */
    /* Encode Functions */
    #[no_mangle]
    static __archive_ppmd7_functions: IPpmd7;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7zip {
    pub si: _7z_stream_info,
    pub header_is_being_read: libc::c_int,
    pub header_is_encoded: libc::c_int,
    pub header_bytes_remaining: uint64_t,
    pub header_crc32: libc::c_ulong,
    pub header_offset: uint64_t,
    pub seek_base: uint64_t,
    pub entries_remaining: size_t,
    pub numFiles: uint64_t,
    pub entries: *mut _7zip_entry,
    pub entry: *mut _7zip_entry,
    pub entry_names: *mut libc::c_uchar,
    pub entry_offset: int64_t,
    pub entry_bytes_remaining: uint64_t,
    pub entry_crc32: libc::c_ulong,
    pub end_of_entry: libc::c_char,
    pub uncompressed_buffer: *mut libc::c_uchar,
    pub uncompressed_buffer_pointer: *mut libc::c_uchar,
    pub uncompressed_buffer_size: size_t,
    pub uncompressed_buffer_bytes_remaining: size_t,
    pub stream_offset: int64_t,
    pub folder_index: libc::c_uint,
    pub folder_outbytes_remaining: uint64_t,
    pub pack_stream_index: libc::c_uint,
    pub pack_stream_remaining: libc::c_uint,
    pub pack_stream_inbytes_remaining: uint64_t,
    pub pack_stream_bytes_unconsumed: size_t,
    pub codec: libc::c_ulong,
    pub codec2: libc::c_ulong,
    pub stream: z_stream,
    pub stream_valid: libc::c_int,
    pub ppmd7_stat: libc::c_int,
    pub ppmd7_context: CPpmd7,
    pub range_dec: CPpmd7z_RangeDec,
    pub bytein: IByteIn,
    pub ppstream: C2RustUnnamed,
    pub ppmd7_valid: libc::c_int,
    pub bcj_state: uint32_t,
    pub odd_bcj_size: size_t,
    pub odd_bcj: [libc::c_uchar; 4],
    pub bcj_prevPosT: size_t,
    pub bcj_prevMask: uint32_t,
    pub bcj_ip: uint32_t,
    pub main_stream_bytes_remaining: size_t,
    pub sub_stream_buff: [*mut libc::c_uchar; 3],
    pub sub_stream_size: [size_t; 3],
    pub sub_stream_bytes_remaining: [size_t; 3],
    pub tmp_stream_buff: *mut libc::c_uchar,
    pub tmp_stream_buff_size: size_t,
    pub tmp_stream_bytes_avail: size_t,
    pub tmp_stream_bytes_remaining: size_t,
    pub bcj2_p: [uint16_t; 258],
    pub bcj2_prevByte: uint8_t,
    pub bcj2_range: uint32_t,
    pub bcj2_code: uint32_t,
    pub bcj2_outPos: uint64_t,
    pub sconv: *mut archive_string_conv,
    pub format_name: [libc::c_char; 64],
    pub has_encrypted_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub next_in: *const libc::c_uchar,
    pub avail_in: int64_t,
    pub total_in: int64_t,
    pub next_out: *mut libc::c_uchar,
    pub avail_out: int64_t,
    pub total_out: int64_t,
    pub overconsumed: libc::c_int,
}
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
pub type UInt32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IPpmd7_RangeDec {
    pub GetThreshold: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32>,
    pub Decode: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32, _: UInt32) -> ()>,
    pub DecodeBit: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: UInt32) -> UInt32>,
}
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
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_See {
    pub Summ: UInt16,
    pub Shift: Byte,
    pub Count: Byte,
}
pub type CPpmd_Void_Ref = UInt32;
pub type Int32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd_State {
    pub Symbol: Byte,
    pub Freq: Byte,
    pub SuccessorLow: UInt16,
    pub SuccessorHigh: UInt16,
}
pub type CPpmd7_Context = CPpmd7_Context_;
/* Ppmd7.h -- PPMdH compression codec
2010-03-12 : Igor Pavlov : Public domain
This code is based on PPMd var.H (2001): Dmitry Shkarin : Public domain */
/* This code supports virtual RangeDecoder and includes the implementation
of RangeCoder from 7z, instead of RangeCoder from original PPMd var.H.
If you need the compatibility with original PPMd var.H, you can use external RangeDecoder */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CPpmd7_Context_ {
    pub NumStats: UInt16,
    pub SummFreq: UInt16,
    pub Stats: CPpmd_State_Ref,
    pub Suffix: CPpmd7_Context_Ref,
}
pub type CPpmd7_Context_Ref = UInt32;
pub type CPpmd_State_Ref = UInt32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7zip_entry {
    pub name_len: size_t,
    pub utf16name: *mut libc::c_uchar,
    pub folderIndex: uint32_t,
    pub ssIndex: uint32_t,
    pub flg: libc::c_uint,
    pub mtime: time_t,
    pub atime: time_t,
    pub ctime: time_t,
    pub mtime_ns: libc::c_long,
    pub atime_ns: libc::c_long,
    pub ctime_ns: libc::c_long,
    pub mode: uint32_t,
    pub attr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_stream_info {
    pub pi: _7z_pack_info,
    pub ci: _7z_coders_info,
    pub ss: _7z_substream_info,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_substream_info {
    pub unpack_streams: size_t,
    pub unpackSizes: *mut uint64_t,
    pub digestsDefined: *mut libc::c_uchar,
    pub digests: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_coders_info {
    pub numFolders: uint64_t,
    pub folders: *mut _7z_folder,
    pub dataStreamIndex: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_folder {
    pub numCoders: uint64_t,
    pub coders: *mut _7z_coder,
    pub numBindPairs: uint64_t,
    pub bindPairs: *mut C2RustUnnamed_1,
    pub numPackedStreams: uint64_t,
    pub packedStreams: *mut uint64_t,
    pub numInStreams: uint64_t,
    pub numOutStreams: uint64_t,
    pub unPackSize: *mut uint64_t,
    pub digest_defined: libc::c_uchar,
    pub digest: uint32_t,
    pub numUnpackStreams: uint64_t,
    pub packIndex: uint32_t,
    pub skipped_bytes: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub inIndex: uint64_t,
    pub outIndex: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_coder {
    pub codec: libc::c_ulong,
    pub numInStreams: uint64_t,
    pub numOutStreams: uint64_t,
    pub propertiesSize: uint64_t,
    pub properties: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_pack_info {
    pub pos: uint64_t,
    pub numPackStreams: uint64_t,
    pub sizes: *mut uint64_t,
    pub digest: _7z_digests,
    pub positions: *mut uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_digests {
    pub defineds: *mut libc::c_uchar,
    pub digests: *mut uint32_t,
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
pub struct IByteOut {
    pub a: *mut archive_write,
    pub Write: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: Byte) -> ()>,
}
pub type UInt64 = libc::c_ulonglong;
pub type Bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _7z_header_info {
    pub dataIndex: uint64_t,
    pub emptyStreamBools: *mut libc::c_uchar,
    pub emptyFileBools: *mut libc::c_uchar,
    pub antiBools: *mut libc::c_uchar,
    pub attrBools: *mut libc::c_uchar,
}
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
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
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * Codes to identify various stream filters.
 */
/*
 * Codes returned by archive_format.
 *
 * Top 16 bits identifies the format family (e.g., "tar"); lower
 * 16 bits indicate the variant.  This is updated by read_next_header.
 * Note that the lower 16 bits will often vary from entry to entry.
 * In some cases, this variation occurs as libarchive learns more about
 * the archive (for example, later entries might utilize extensions that
 * weren't necessary earlier in the archive; in this case, libarchive
 * will change the format code to indicate the extended format that
 * was used).  In other cases, it's because different tools have
 * modified the archive and so different parts of the archive
 * actually have slightly different formats.  (Both tar and cpio store
 * format codes in each entry, so it is quite possible for each
 * entry to be in a different format.)
 */
pub const ARCHIVE_FORMAT_7ZIP: libc::c_int = 0xe0000 as libc::c_int;
/*
 * Codes returned by archive_read_format_capabilities().
 *
 * This list can be extended with values between 0 and 0xffff.
 * The original purpose of this list was to let different archive
 * format readers expose their general capabilities in terms of
 * encryption.
 */
/* no special capabilities */
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA: libc::c_int =
    (1 as libc::c_int) << 0 as libc::c_int;
/* reader can detect encrypted data */
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA: libc::c_int =
    (1 as libc::c_int) << 1 as libc::c_int;
/* reader can detect encryptable metadata (pathname, mtime, etc.) */
/*
 * Codes returned by archive_read_has_encrypted_entries().
 *
 * In case the archive does not support encryption detection at all
 * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned. If the reader
 * for some other reason (e.g. not enough bytes read) cannot say if
 * there are encrypted entries, ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW
 * is returned.
 */
pub const ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW: libc::c_int = -(1 as libc::c_int);
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
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
pub const archive_entry_copy_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_pathname_l;
pub const PPMD7_MIN_ORDER: libc::c_int = 2 as libc::c_int;
pub const PPMD7_MAX_ORDER: libc::c_int = 64 as libc::c_int;
pub const PPMD7_MIN_MEM_SIZE: libc::c_int = (1 as libc::c_int) << 11 as libc::c_int;
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
pub const _7ZIP_SIGNATURE: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"7z\xbc\xaf\'\x1c\x00") };
pub const SFX_MIN_ADDR: libc::c_int = 0x27000 as libc::c_int;
pub const SFX_MAX_ADDR: libc::c_int = 0x60000 as libc::c_int;
/*
 * Codec ID
 */
pub const _7Z_COPY: libc::c_int = 0 as libc::c_int;
pub const _7Z_LZMA: libc::c_int = 0x30101 as libc::c_int;
pub const _7Z_LZMA2: libc::c_int = 0x21 as libc::c_int;
pub const _7Z_DEFLATE: libc::c_int = 0x40108 as libc::c_int;
pub const _7Z_BZ2: libc::c_int = 0x40202 as libc::c_int;
pub const _7Z_PPMD: libc::c_int = 0x30401 as libc::c_int;
pub const _7Z_DELTA: libc::c_int = 0x3 as libc::c_int;
pub const _7Z_CRYPTO_MAIN_ZIP: libc::c_int = 0x6f10101 as libc::c_int;
/* Main Zip crypto algo */
pub const _7Z_CRYPTO_RAR_29: libc::c_int = 0x6f10303 as libc::c_int;
/* Rar29 AES-128 + (modified SHA-1) */
pub const _7Z_CRYPTO_AES_256_SHA_256: libc::c_int = 0x6f10701 as libc::c_int;
/* AES-256 + SHA-256 */
pub const _7Z_X86: libc::c_int = 0x3030103 as libc::c_int;
pub const _7Z_X86_BCJ2: libc::c_int = 0x303011b as libc::c_int;
pub const _7Z_POWERPC: libc::c_int = 0x3030205 as libc::c_int;
pub const _7Z_IA64: libc::c_int = 0x3030401 as libc::c_int;
pub const _7Z_ARM: libc::c_int = 0x3030501 as libc::c_int;
pub const _7Z_ARMTHUMB: libc::c_int = 0x3030701 as libc::c_int;
pub const _7Z_SPARC: libc::c_int = 0x3030805 as libc::c_int;
/*
 * 7-Zip header property IDs.
 */
pub const kEnd: libc::c_int = 0 as libc::c_int;
pub const kHeader: libc::c_int = 0x1 as libc::c_int;
pub const kArchiveProperties: libc::c_int = 0x2 as libc::c_int;
pub const kMainStreamsInfo: libc::c_int = 0x4 as libc::c_int;
pub const kFilesInfo: libc::c_int = 0x5 as libc::c_int;
pub const kPackInfo: libc::c_int = 0x6 as libc::c_int;
pub const kUnPackInfo: libc::c_int = 0x7 as libc::c_int;
pub const kSubStreamsInfo: libc::c_int = 0x8 as libc::c_int;
pub const kSize: libc::c_int = 0x9 as libc::c_int;
pub const kCRC: libc::c_int = 0xa as libc::c_int;
pub const kFolder: libc::c_int = 0xb as libc::c_int;
pub const kCodersUnPackSize: libc::c_int = 0xc as libc::c_int;
pub const kNumUnPackStream: libc::c_int = 0xd as libc::c_int;
pub const kEmptyStream: libc::c_int = 0xe as libc::c_int;
pub const kEmptyFile: libc::c_int = 0xf as libc::c_int;
pub const kAnti: libc::c_int = 0x10 as libc::c_int;
pub const kName: libc::c_int = 0x11 as libc::c_int;
pub const kCTime: libc::c_int = 0x12 as libc::c_int;
pub const kATime: libc::c_int = 0x13 as libc::c_int;
pub const kMTime: libc::c_int = 0x14 as libc::c_int;
pub const kAttributes: libc::c_int = 0x15 as libc::c_int;
pub const kEncodedHeader: libc::c_int = 0x17 as libc::c_int;
pub const kDummy: libc::c_int = 0x19 as libc::c_int;
pub const MTIME_IS_SET: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const ATIME_IS_SET: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
pub const CTIME_IS_SET: libc::c_int = (1 as libc::c_int) << 2 as libc::c_int;
pub const CRC32_IS_SET: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
pub const HAS_STREAM: libc::c_int = (1 as libc::c_int) << 4 as libc::c_int;
pub const UBUFF_SIZE: libc::c_int = 64 as libc::c_int * 1024 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_7zip(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_7zip\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    zip = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_7zip>() as libc::c_ulong,
    ) as *mut _7zip;
    if zip.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate 7zip data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * Until enough data has been read, we cannot tell about
     * any encrypted entries yet.
     */
    (*zip).has_encrypted_entries = ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
    r = __archive_read_register_format(
        a,
        zip as *mut libc::c_void,
        b"7zip\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_7zip_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
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
            archive_read_format_7zip_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_7zip_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_7zip_read_data_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_7zip_cleanup
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_support_format_7zip_capabilities
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_format_7zip_has_encrypted_entries
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
    );
    if r != ARCHIVE_OK {
        free(zip as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_support_format_7zip_capabilities(
    mut a: *mut archive_read,
) -> libc::c_int {
    /* UNUSED */
    return ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA | ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA;
}
/* Maximum entry size. This limitation prevents reading intentional
 * corrupted 7-zip files on assuming there are not so many entries in
 * the files. */
unsafe extern "C" fn archive_read_format_7zip_has_encrypted_entries(
    mut _a: *mut archive_read,
) -> libc::c_int {
    if !_a.is_null() && !(*_a).format.is_null() {
        let mut zip: *mut _7zip = (*(*_a).format).data as *mut _7zip;
        if !zip.is_null() {
            return (*zip).has_encrypted_entries;
        }
    }
    return ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
}
unsafe extern "C" fn archive_read_format_7zip_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* If someone has already bid more than 32, then avoid
    trashing the look-ahead buffers with a seek. */
    if best_bid > 32 as libc::c_int {
        return -(1 as libc::c_int);
    }
    p = __archive_read_ahead(a, 6 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    /* If first six bytes are the 7-Zip signature,
     * return the bid right now. */
    if memcmp(
        p as *const libc::c_void,
        _7ZIP_SIGNATURE.as_ptr(),
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 48 as libc::c_int;
    }
    /*
     * It may a 7-Zip SFX archive file. If first two bytes are
     * 'M' and 'Z' available on Windows or first four bytes are
     * "\x7F\x45LF" available on posix like system, seek the 7-Zip
     * signature. Although we will perform a seek when reading
     * a header, what we do not use __archive_read_seek() here is
     * due to a bidding performance.
     */
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
        || memcmp(
            p as *const libc::c_void,
            b"\x7fELF\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        let mut offset: ssize_t = SFX_MIN_ADDR as ssize_t;
        let mut window: ssize_t = 4096 as libc::c_int as ssize_t;
        let mut bytes_avail: ssize_t = 0;
        while offset + window <= 0x60000 as libc::c_int as libc::c_long {
            let mut buff: *const libc::c_char =
                __archive_read_ahead(a, (offset + window) as size_t, &mut bytes_avail)
                    as *const libc::c_char;
            if buff.is_null() {
                /* Remaining bytes are less than window. */
                window >>= 1 as libc::c_int;
                if window < 0x40 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int;
                }
            } else {
                p = buff.offset(offset as isize);
                while p.offset(32 as libc::c_int as isize) < buff.offset(bytes_avail as isize) {
                    let mut step: libc::c_int = check_7zip_header_in_sfx(p);
                    if step == 0 as libc::c_int {
                        return 48 as libc::c_int;
                    }
                    p = p.offset(step as isize)
                }
                offset = p.offset_from(buff) as libc::c_long
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_7zip_header_in_sfx(mut p: *const libc::c_char) -> libc::c_int {
    match *p.offset(5 as libc::c_int as isize) as libc::c_uchar as libc::c_int {
        28 => {
            if memcmp(
                p as *const libc::c_void,
                _7ZIP_SIGNATURE.as_ptr(),
                6 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                return 6 as libc::c_int;
            }
            /*
             * Test the CRC because its extraction code has 7-Zip
             * Magic Code, so we should do this in order not to
             * make a mis-detection.
             */
            if crc32(
                0 as libc::c_int as uLong,
                (p as *const libc::c_uchar).offset(12 as libc::c_int as isize),
                20 as libc::c_int as uInt,
            ) != archive_le32dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void)
                as libc::c_ulong
            {
                return 6 as libc::c_int;
            }
            /* Hit the header! */
            return 0 as libc::c_int;
        }
        55 => return 5 as libc::c_int,
        122 => return 4 as libc::c_int,
        188 => return 3 as libc::c_int,
        175 => return 2 as libc::c_int,
        39 => return 1 as libc::c_int,
        _ => return 6 as libc::c_int,
    };
}
unsafe extern "C" fn skip_sfx(mut a: *mut archive_read, mut bytes_avail: ssize_t) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut skip: size_t = 0;
    let mut offset: size_t = 0;
    let mut bytes: ssize_t = 0;
    let mut window: ssize_t = 0;
    /*
     * If bytes_avail > SFX_MIN_ADDR we do not have to call
     * __archive_read_seek() at this time since we have
     * already had enough data.
     */
    if bytes_avail > SFX_MIN_ADDR as libc::c_long {
        __archive_read_consume(a, SFX_MIN_ADDR as int64_t);
    } else if __archive_read_seek(a, SFX_MIN_ADDR as int64_t, SEEK_SET)
        < 0 as libc::c_int as libc::c_long
    {
        return -(30 as libc::c_int);
    }
    offset = 0 as libc::c_int as size_t;
    window = 1 as libc::c_int as ssize_t;
    while offset.wrapping_add(window as libc::c_ulong)
        <= (SFX_MAX_ADDR - SFX_MIN_ADDR) as libc::c_ulong
    {
        h = __archive_read_ahead(a, window as size_t, &mut bytes);
        if h == NULL as *const libc::c_void {
            /* Remaining bytes are less than window. */
            window >>= 1 as libc::c_int;
            if window < 0x40 as libc::c_int as libc::c_long {
                break;
            }
        } else if bytes < 6 as libc::c_int as libc::c_long {
            /* This case might happen when window == 1. */
            window = 4096 as libc::c_int as ssize_t
        } else {
            p = h as *const libc::c_char;
            q = p.offset(bytes as isize);
            /*
             * Scan ahead until we find something that looks
             * like the 7-Zip header.
             */
            while p.offset(32 as libc::c_int as isize) < q {
                let mut step: libc::c_int = check_7zip_header_in_sfx(p);
                if step == 0 as libc::c_int {
                    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
                    skip =
                        p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
                    __archive_read_consume(a, skip as int64_t);
                    (*zip).seek_base = (SFX_MIN_ADDR as libc::c_ulong)
                        .wrapping_add(offset)
                        .wrapping_add(skip);
                    return 0 as libc::c_int;
                }
                p = p.offset(step as isize)
            }
            skip = p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
            __archive_read_consume(a, skip as int64_t);
            offset = (offset as libc::c_ulong).wrapping_add(skip) as size_t as size_t;
            if window == 1 as libc::c_int as libc::c_long {
                window = 4096 as libc::c_int as ssize_t
            }
        }
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Couldn\'t find out 7-Zip header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn archive_read_format_7zip_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut zip_entry: *mut _7zip_entry = 0 as *mut _7zip_entry;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    let mut folder: *mut _7z_folder = 0 as *mut _7z_folder;
    let mut fidx: uint64_t = 0 as libc::c_int as uint64_t;
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
    (*a).archive.archive_format = ARCHIVE_FORMAT_7ZIP;
    if (*a).archive.archive_format_name.is_null() {
        (*a).archive.archive_format_name = b"7-Zip\x00" as *const u8 as *const libc::c_char
    }
    if (*zip).entries.is_null() {
        let mut header: _7z_header_info = _7z_header_info {
            dataIndex: 0,
            emptyStreamBools: 0 as *mut libc::c_uchar,
            emptyFileBools: 0 as *mut libc::c_uchar,
            antiBools: 0 as *mut libc::c_uchar,
            attrBools: 0 as *mut libc::c_uchar,
        };
        memset(
            &mut header as *mut _7z_header_info as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<_7z_header_info>() as libc::c_ulong,
        );
        r = slurp_central_directory(a, zip, &mut header);
        free_Header(&mut header);
        if r != ARCHIVE_OK {
            return r;
        }
        (*zip).entries_remaining = (*zip).numFiles;
        (*zip).entry = (*zip).entries
    } else {
        (*zip).entry = (*zip).entry.offset(1)
    }
    zip_entry = (*zip).entry;
    if (*zip).entries_remaining <= 0 as libc::c_int as libc::c_ulong || zip_entry.is_null() {
        return ARCHIVE_EOF;
    }
    (*zip).entries_remaining = (*zip).entries_remaining.wrapping_sub(1);
    (*zip).entry_offset = 0 as libc::c_int as int64_t;
    (*zip).end_of_entry = 0 as libc::c_int as libc::c_char;
    (*zip).entry_crc32 = crc32(
        0 as libc::c_int as uLong,
        NULL as *const Bytef,
        0 as libc::c_int as uInt,
    );
    /* Setup a string conversion for a filename. */
    if (*zip).sconv.is_null() {
        (*zip).sconv = archive_string_conversion_from_charset(
            &mut (*a).archive,
            b"UTF-16LE\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if (*zip).sconv.is_null() {
            return -(30 as libc::c_int);
        }
    }
    /* Figure out if the entry is encrypted by looking at the folder
    that is associated to the current 7zip entry. If the folder
    has a coder with a _7Z_CRYPTO codec then the folder is encrypted.
    Hence the entry must also be encrypted. */
    if !zip_entry.is_null() && ((*zip_entry).folderIndex as libc::c_ulong) < (*zip).si.ci.numFolders
    {
        folder = &mut *(*zip)
            .si
            .ci
            .folders
            .offset((*zip_entry).folderIndex as isize) as *mut _7z_folder;
        fidx = 0 as libc::c_int as uint64_t;
        while !folder.is_null() && fidx < (*folder).numCoders {
            match (*(*folder).coders.offset(fidx as isize)).codec {
                116457729 | 116458243 | 116459265 => {
                    archive_entry_set_is_data_encrypted(entry, 1 as libc::c_int as libc::c_char);
                    (*zip).has_encrypted_entries = 1 as libc::c_int
                }
                _ => {}
            }
            fidx = fidx.wrapping_add(1)
        }
    }
    /* Now that we've checked for encryption, if there were still no
     * encrypted entries found we can say for sure that there are none.
     */
    if (*zip).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*zip).has_encrypted_entries = 0 as libc::c_int
    }
    if _archive_entry_copy_pathname_l(
        entry,
        (*zip_entry).utf16name as *const libc::c_char,
        (*zip_entry).name_len,
        (*zip).sconv,
    ) != 0 as libc::c_int
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
            archive_string_conversion_charset_name((*zip).sconv),
        );
        ret = ARCHIVE_WARN
    }
    /* Populate some additional entry fields: */
    archive_entry_set_mode(entry, (*zip_entry).mode);
    if (*zip_entry).flg & MTIME_IS_SET as libc::c_uint != 0 {
        archive_entry_set_mtime(entry, (*zip_entry).mtime, (*zip_entry).mtime_ns);
    }
    if (*zip_entry).flg & CTIME_IS_SET as libc::c_uint != 0 {
        archive_entry_set_ctime(entry, (*zip_entry).ctime, (*zip_entry).ctime_ns);
    }
    if (*zip_entry).flg & ATIME_IS_SET as libc::c_uint != 0 {
        archive_entry_set_atime(entry, (*zip_entry).atime, (*zip_entry).atime_ns);
    }
    if (*zip_entry).ssIndex != -(1 as libc::c_int) as uint32_t {
        (*zip).entry_bytes_remaining = *(*zip)
            .si
            .ss
            .unpackSizes
            .offset((*zip_entry).ssIndex as isize);
        archive_entry_set_size(entry, (*zip).entry_bytes_remaining as la_int64_t);
    } else {
        (*zip).entry_bytes_remaining = 0 as libc::c_int as uint64_t;
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
    }
    /* If there's no body, force read_data() to return EOF immediately. */
    if (*zip).entry_bytes_remaining < 1 as libc::c_int as libc::c_ulong {
        (*zip).end_of_entry = 1 as libc::c_int as libc::c_char
    }
    if (*zip_entry).mode & AE_IFMT as mode_t == AE_IFLNK as mode_t {
        let mut symname: *mut libc::c_uchar = NULL as *mut libc::c_uchar;
        let mut symsize: size_t = 0 as libc::c_int as size_t;
        /*
         * Symbolic-name is recorded as its contents. We have to
         * read the contents at this time.
         */
        while (*zip).entry_bytes_remaining > 0 as libc::c_int as libc::c_ulong {
            let mut buff: *const libc::c_void = 0 as *const libc::c_void;
            let mut mem: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            let mut size: size_t = 0;
            let mut offset: int64_t = 0;
            r = archive_read_format_7zip_read_data(a, &mut buff, &mut size, &mut offset);
            if r < ARCHIVE_WARN {
                free(symname as *mut libc::c_void);
                return r;
            }
            mem = realloc(
                symname as *mut libc::c_void,
                symsize
                    .wrapping_add(size)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_uchar;
            if mem.is_null() {
                free(symname as *mut libc::c_void);
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate memory for Symname\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            symname = mem;
            memcpy(
                symname.offset(symsize as isize) as *mut libc::c_void,
                buff,
                size,
            );
            symsize = (symsize as libc::c_ulong).wrapping_add(size) as size_t as size_t
        }
        if symsize == 0 as libc::c_int as libc::c_ulong {
            /* If there is no symname, handle it as a regular
             * file. */
            (*zip_entry).mode &= !(AE_IFMT as mode_t);
            (*zip_entry).mode |= AE_IFREG as mode_t;
            archive_entry_set_mode(entry, (*zip_entry).mode);
        } else {
            *symname.offset(symsize as isize) = '\u{0}' as i32 as libc::c_uchar;
            archive_entry_copy_symlink(entry, symname as *const libc::c_char);
        }
        free(symname as *mut libc::c_void);
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
    }
    /* Set up a more descriptive format name. */
    sprintf(
        (*zip).format_name.as_mut_ptr(),
        b"7-Zip\x00" as *const u8 as *const libc::c_char,
    );
    (*a).archive.archive_format_name = (*zip).format_name.as_mut_ptr();
    return ret;
}
unsafe extern "C" fn archive_read_format_7zip_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut bytes: ssize_t = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    zip = (*(*a).format).data as *mut _7zip;
    if (*zip).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*zip).has_encrypted_entries = 0 as libc::c_int
    }
    if (*zip).pack_stream_bytes_unconsumed != 0 {
        read_consume(a);
    }
    *offset = (*zip).entry_offset;
    *size = 0 as libc::c_int as size_t;
    *buff = NULL as *const libc::c_void;
    /*
     * If we hit end-of-entry last time, clean up and return
     * ARCHIVE_EOF this time.
     */
    if (*zip).end_of_entry != 0 {
        return 1 as libc::c_int;
    }
    bytes = read_stream(
        a,
        buff,
        (*zip).entry_bytes_remaining,
        0 as libc::c_int as size_t,
    );
    if bytes < 0 as libc::c_int as libc::c_long {
        return bytes as libc::c_int;
    }
    if bytes == 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated 7-Zip file body\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*zip).entry_bytes_remaining = ((*zip).entry_bytes_remaining as libc::c_ulong)
        .wrapping_sub(bytes as libc::c_ulong) as uint64_t
        as uint64_t;
    if (*zip).entry_bytes_remaining == 0 as libc::c_int as libc::c_ulong {
        (*zip).end_of_entry = 1 as libc::c_int as libc::c_char
    }
    /* Update checksum */
    if (*(*zip).entry).flg & CRC32_IS_SET as libc::c_uint != 0 && bytes != 0 {
        (*zip).entry_crc32 = crc32(
            (*zip).entry_crc32,
            *buff as *const Bytef,
            bytes as libc::c_uint,
        )
    }
    /* If we hit the end, swallow any end-of-data marker. */
    if (*zip).end_of_entry != 0 {
        /* Check computed CRC against file contents. */
        if (*(*zip).entry).flg & CRC32_IS_SET as libc::c_uint != 0
            && *(*zip)
                .si
                .ss
                .digests
                .offset((*(*zip).entry).ssIndex as isize) as libc::c_ulong
                != (*zip).entry_crc32
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"7-Zip bad CRC: 0x%lx should be 0x%lx\x00" as *const u8 as *const libc::c_char,
                (*zip).entry_crc32,
                *(*zip)
                    .si
                    .ss
                    .digests
                    .offset((*(*zip).entry).ssIndex as isize) as libc::c_ulong,
            );
            ret = ARCHIVE_WARN
        }
    }
    *size = bytes as size_t;
    *offset = (*zip).entry_offset;
    (*zip).entry_offset += bytes;
    return ret;
}
unsafe extern "C" fn archive_read_format_7zip_read_data_skip(
    mut a: *mut archive_read,
) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    let mut bytes_skipped: int64_t = 0;
    zip = (*(*a).format).data as *mut _7zip;
    if (*zip).pack_stream_bytes_unconsumed != 0 {
        read_consume(a);
    }
    /* If we've already read to end of data, we're done. */
    if (*zip).end_of_entry != 0 {
        return 0 as libc::c_int;
    }
    /*
     * If the length is at the beginning, we can skip the
     * compressed data much more quickly.
     */
    bytes_skipped = skip_stream(a, (*zip).entry_bytes_remaining);
    if bytes_skipped < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    (*zip).entry_bytes_remaining = 0 as libc::c_int as uint64_t;
    /* This entry is finished and done. */
    (*zip).end_of_entry = 1 as libc::c_int as libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_7zip_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut zip: *mut _7zip = 0 as *mut _7zip;
    zip = (*(*a).format).data as *mut _7zip;
    free_StreamsInfo(&mut (*zip).si);
    free((*zip).entries as *mut libc::c_void);
    free((*zip).entry_names as *mut libc::c_void);
    free_decompression(a, zip);
    free((*zip).uncompressed_buffer as *mut libc::c_void);
    free((*zip).sub_stream_buff[0 as libc::c_int as usize] as *mut libc::c_void);
    free((*zip).sub_stream_buff[1 as libc::c_int as usize] as *mut libc::c_void);
    free((*zip).sub_stream_buff[2 as libc::c_int as usize] as *mut libc::c_void);
    free((*zip).tmp_stream_buff as *mut libc::c_void);
    free(zip as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_consume(mut a: *mut archive_read) {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    if (*zip).pack_stream_bytes_unconsumed != 0 {
        __archive_read_consume(a, (*zip).pack_stream_bytes_unconsumed as int64_t);
        (*zip).stream_offset = ((*zip).stream_offset as libc::c_ulong)
            .wrapping_add((*zip).pack_stream_bytes_unconsumed)
            as int64_t as int64_t;
        (*zip).pack_stream_bytes_unconsumed = 0 as libc::c_int as size_t
    };
}
unsafe extern "C" fn decode_codec_id(
    mut codecId: *const libc::c_uchar,
    mut id_size: size_t,
) -> libc::c_ulong {
    let mut i: libc::c_uint = 0;
    let mut id: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < id_size {
        id <<= 8 as libc::c_int;
        id = id.wrapping_add(*codecId.offset(i as isize) as libc::c_ulong);
        i = i.wrapping_add(1)
    }
    return id;
}
unsafe extern "C" fn ppmd_read(mut p: *mut libc::c_void) -> Byte {
    let mut a: *mut archive_read = (*(p as *mut IByteIn)).a;
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut b: Byte = 0;
    if (*zip).ppstream.avail_in == 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
        );
        (*zip).ppstream.overconsumed = 1 as libc::c_int;
        return 0 as libc::c_int as Byte;
    }
    let fresh0 = (*zip).ppstream.next_in;
    (*zip).ppstream.next_in = (*zip).ppstream.next_in.offset(1);
    b = *fresh0;
    (*zip).ppstream.avail_in -= 1;
    (*zip).ppstream.total_in += 1;
    return b;
}
unsafe extern "C" fn init_decompression(
    mut a: *mut archive_read,
    mut zip: *mut _7zip,
    mut coder1: *const _7z_coder,
    mut coder2: *const _7z_coder,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    (*zip).codec = (*coder1).codec;
    (*zip).codec2 = -(1 as libc::c_int) as libc::c_ulong;
    match (*zip).codec {
        0 | 262658 | 262408 | 197633 => {
            if !coder2.is_null() {
                if (*coder2).codec != _7Z_X86 as libc::c_ulong
                    && (*coder2).codec != _7Z_X86_BCJ2 as libc::c_ulong
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Unsupported filter %lx for %lx\x00" as *const u8 as *const libc::c_char,
                        (*coder2).codec,
                        (*coder1).codec,
                    );
                    return -(25 as libc::c_int);
                }
                (*zip).codec2 = (*coder2).codec;
                (*zip).bcj_state = 0 as libc::c_int as uint32_t;
                if (*coder2).codec == _7Z_X86 as libc::c_ulong {
                    x86_Init(zip);
                }
            }
        }
        _ => {}
    }
    match (*zip).codec {
        0 => {}
        196865 | 33 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"LZMA codec is unsupported\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        262658 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"BZ2 codec is unsupported\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        262408 => {
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
                    b"Couldn\'t initialize zlib stream.\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            (*zip).stream_valid = 1 as libc::c_int;
            (*zip).stream.total_in = 0 as libc::c_int as uLong;
            (*zip).stream.total_out = 0 as libc::c_int as uLong
        }
        197633 => {
            let mut order: libc::c_uint = 0;
            let mut msize: uint32_t = 0;
            if (*zip).ppmd7_valid != 0 {
                __archive_ppmd7_functions
                    .Ppmd7_Free
                    .expect("non-null function pointer")(&mut (*zip).ppmd7_context);
                (*zip).ppmd7_valid = 0 as libc::c_int
            }
            if (*coder1).propertiesSize < 5 as libc::c_int as libc::c_ulong {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Malformed PPMd parameter\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            order = *(*coder1).properties.offset(0 as libc::c_int as isize) as libc::c_uint;
            msize = archive_le32dec(&mut *(*coder1).properties.offset(1 as libc::c_int as isize)
                as *mut libc::c_uchar as *const libc::c_void);
            if order < PPMD7_MIN_ORDER as libc::c_uint
                || order > PPMD7_MAX_ORDER as libc::c_uint
                || msize < PPMD7_MIN_MEM_SIZE as libc::c_uint
                || msize
                    > (0xffffffff as libc::c_uint)
                        .wrapping_sub((12 as libc::c_int * 3 as libc::c_int) as libc::c_uint)
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Malformed PPMd parameter\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            __archive_ppmd7_functions
                .Ppmd7_Construct
                .expect("non-null function pointer")(&mut (*zip).ppmd7_context);
            r = __archive_ppmd7_functions
                .Ppmd7_Alloc
                .expect("non-null function pointer")(
                &mut (*zip).ppmd7_context, msize
            );
            if r == 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Coludn\'t allocate memory for PPMd\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            __archive_ppmd7_functions
                .Ppmd7_Init
                .expect("non-null function pointer")(&mut (*zip).ppmd7_context, order);
            __archive_ppmd7_functions
                .Ppmd7z_RangeDec_CreateVTable
                .expect("non-null function pointer")(&mut (*zip).range_dec);
            (*zip).ppmd7_valid = 1 as libc::c_int;
            (*zip).ppmd7_stat = 0 as libc::c_int;
            (*zip).ppstream.overconsumed = 0 as libc::c_int;
            (*zip).ppstream.total_in = 0 as libc::c_int as int64_t;
            (*zip).ppstream.total_out = 0 as libc::c_int as int64_t
        }
        50528515 | 50528539 | 50528773 | 50529281 | 50529537 | 50530049 | 50530309 | 3 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unexpected codec ID: %lX\x00" as *const u8 as *const libc::c_char,
                (*zip).codec,
            );
            return -(25 as libc::c_int);
        }
        116457729 | 116458243 | 116459265 => {
            if !(*a).entry.is_null() {
                archive_entry_set_is_metadata_encrypted(
                    (*a).entry,
                    1 as libc::c_int as libc::c_char,
                );
                archive_entry_set_is_data_encrypted((*a).entry, 1 as libc::c_int as libc::c_char);
                (*zip).has_encrypted_entries = 1 as libc::c_int
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Crypto codec not supported yet (ID: 0x%lX)\x00" as *const u8
                    as *const libc::c_char,
                (*zip).codec,
            );
            return -(25 as libc::c_int);
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unknown codec ID: %lX\x00" as *const u8 as *const libc::c_char,
                (*zip).codec,
            );
            return -(25 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn decompress(
    mut a: *mut archive_read,
    mut zip: *mut _7zip,
    mut buff: *mut libc::c_void,
    mut outbytes: *mut size_t,
    mut b: *const libc::c_void,
    mut used: *mut size_t,
) -> libc::c_int {
    let mut t_next_in: *const uint8_t = 0 as *const uint8_t;
    let mut t_next_out: *mut uint8_t = 0 as *mut uint8_t;
    let mut o_avail_in: size_t = 0;
    let mut o_avail_out: size_t = 0;
    let mut t_avail_in: size_t = 0;
    let mut t_avail_out: size_t = 0;
    let mut bcj2_next_out: *mut uint8_t = 0 as *mut uint8_t;
    let mut bcj2_avail_out: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = ARCHIVE_OK;
    o_avail_in = *used;
    t_avail_in = o_avail_in;
    o_avail_out = *outbytes;
    t_avail_out = o_avail_out;
    t_next_in = b as *const uint8_t;
    t_next_out = buff as *mut uint8_t;
    if (*zip).codec != _7Z_LZMA2 as libc::c_ulong && (*zip).codec2 == _7Z_X86 as libc::c_ulong {
        let mut i: libc::c_int = 0;
        /* Do not copy out the BCJ remaining bytes when the output
         * buffer size is less than five bytes. */
        if o_avail_in != 0 as libc::c_int as libc::c_ulong
            && t_avail_out < 5 as libc::c_int as libc::c_ulong
            && (*zip).odd_bcj_size != 0
        {
            *used = 0 as libc::c_int as size_t;
            *outbytes = 0 as libc::c_int as size_t;
            return ret;
        }
        i = 0 as libc::c_int;
        while (*zip).odd_bcj_size > 0 as libc::c_int as libc::c_ulong && t_avail_out != 0 {
            let fresh1 = t_next_out;
            t_next_out = t_next_out.offset(1);
            *fresh1 = (*zip).odd_bcj[i as usize];
            t_avail_out = t_avail_out.wrapping_sub(1);
            (*zip).odd_bcj_size = (*zip).odd_bcj_size.wrapping_sub(1);
            i += 1
        }
        if o_avail_in == 0 as libc::c_int as libc::c_ulong
            || t_avail_out == 0 as libc::c_int as libc::c_ulong
        {
            *used = o_avail_in.wrapping_sub(t_avail_in);
            *outbytes = o_avail_out.wrapping_sub(t_avail_out);
            if o_avail_in == 0 as libc::c_int as libc::c_ulong {
                ret = ARCHIVE_EOF
            }
            return ret;
        }
    }
    bcj2_next_out = t_next_out;
    bcj2_avail_out = t_avail_out;
    if (*zip).codec2 == _7Z_X86_BCJ2 as libc::c_ulong {
        /*
         * Decord a remaining decompressed main stream for BCJ2.
         */
        if (*zip).tmp_stream_bytes_remaining != 0 {
            let mut bytes: ssize_t = 0;
            let mut remaining: size_t = (*zip).tmp_stream_bytes_remaining;
            bytes = Bcj2_Decode(zip, t_next_out, t_avail_out);
            if bytes < 0 as libc::c_int as libc::c_long {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"BCJ2 conversion Failed\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            (*zip).main_stream_bytes_remaining = ((*zip).main_stream_bytes_remaining
                as libc::c_ulong)
                .wrapping_sub(remaining.wrapping_sub((*zip).tmp_stream_bytes_remaining))
                as size_t as size_t;
            t_avail_out = (t_avail_out as libc::c_ulong).wrapping_sub(bytes as libc::c_ulong)
                as size_t as size_t;
            if o_avail_in == 0 as libc::c_int as libc::c_ulong
                || t_avail_out == 0 as libc::c_int as libc::c_ulong
            {
                *used = 0 as libc::c_int as size_t;
                *outbytes = o_avail_out.wrapping_sub(t_avail_out);
                if o_avail_in == 0 as libc::c_int as libc::c_ulong
                    && (*zip).tmp_stream_bytes_remaining != 0
                {
                    ret = ARCHIVE_EOF
                }
                return ret;
            }
            t_next_out = t_next_out.offset(bytes as isize);
            bcj2_next_out = t_next_out;
            bcj2_avail_out = t_avail_out
        }
        t_next_out = (*zip).tmp_stream_buff;
        t_avail_out = (*zip).tmp_stream_buff_size
    }
    match (*zip).codec {
        0 => {
            let mut bytes_0: size_t = if t_avail_in > t_avail_out {
                t_avail_out
            } else {
                t_avail_in
            };
            memcpy(
                t_next_out as *mut libc::c_void,
                t_next_in as *const libc::c_void,
                bytes_0,
            );
            t_avail_in = (t_avail_in as libc::c_ulong).wrapping_sub(bytes_0) as size_t as size_t;
            t_avail_out = (t_avail_out as libc::c_ulong).wrapping_sub(bytes_0) as size_t as size_t;
            if o_avail_in == 0 as libc::c_int as libc::c_ulong {
                ret = ARCHIVE_EOF
            }
        }
        262408 => {
            (*zip).stream.next_in = t_next_in as uintptr_t as *mut Bytef;
            (*zip).stream.avail_in = t_avail_in as uInt;
            (*zip).stream.next_out = t_next_out;
            (*zip).stream.avail_out = t_avail_out as uInt;
            r = inflate(&mut (*zip).stream, 0 as libc::c_int);
            match r {
                Z_STREAM_END => {
                    /* Found end of stream. */
                    ret = ARCHIVE_EOF
                }
                Z_OK => {}
                _ => {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"File decompression failed (%d)\x00" as *const u8 as *const libc::c_char,
                        r,
                    );
                    return -(25 as libc::c_int);
                }
            }
            t_avail_in = (*zip).stream.avail_in as size_t;
            t_avail_out = (*zip).stream.avail_out as size_t
        }
        197633 => {
            let mut flush_bytes: uint64_t = 0;
            if (*zip).ppmd7_valid == 0
                || (*zip).ppmd7_stat < 0 as libc::c_int
                || t_avail_out <= 0 as libc::c_int as libc::c_ulong
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Decompression internal error\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            (*zip).ppstream.next_in = t_next_in;
            (*zip).ppstream.avail_in = t_avail_in as int64_t;
            (*zip).ppstream.next_out = t_next_out;
            (*zip).ppstream.avail_out = t_avail_out as int64_t;
            if (*zip).ppmd7_stat == 0 as libc::c_int {
                (*zip).bytein.a = a;
                (*zip).bytein.Read =
                    Some(ppmd_read as unsafe extern "C" fn(_: *mut libc::c_void) -> Byte);
                (*zip).range_dec.Stream = &mut (*zip).bytein;
                r = __archive_ppmd7_functions
                    .Ppmd7z_RangeDec_Init
                    .expect("non-null function pointer")(&mut (*zip).range_dec);
                if r == 0 as libc::c_int {
                    (*zip).ppmd7_stat = -(1 as libc::c_int);
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Failed to initialize PPMd range decorder\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
                if (*zip).ppstream.overconsumed != 0 {
                    (*zip).ppmd7_stat = -(1 as libc::c_int);
                    return -(25 as libc::c_int);
                }
                (*zip).ppmd7_stat = 1 as libc::c_int
            }
            if t_avail_in == 0 as libc::c_int as libc::c_ulong {
                /* XXX Flush out remaining decoded data XXX */
                flush_bytes = (*zip).folder_outbytes_remaining
            } else {
                flush_bytes = 0 as libc::c_int as uint64_t
            }
            loop {
                let mut sym: libc::c_int = 0;
                sym = __archive_ppmd7_functions
                    .Ppmd7_DecodeSymbol
                    .expect("non-null function pointer")(
                    &mut (*zip).ppmd7_context,
                    &mut (*zip).range_dec.p,
                );
                if sym < 0 as libc::c_int {
                    (*zip).ppmd7_stat = -(1 as libc::c_int);
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Failed to decode PPMd\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
                if (*zip).ppstream.overconsumed != 0 {
                    (*zip).ppmd7_stat = -(1 as libc::c_int);
                    return -(25 as libc::c_int);
                }
                let fresh2 = (*zip).ppstream.next_out;
                (*zip).ppstream.next_out = (*zip).ppstream.next_out.offset(1);
                *fresh2 = sym as libc::c_uchar;
                (*zip).ppstream.avail_out -= 1;
                (*zip).ppstream.total_out += 1;
                if flush_bytes != 0 {
                    flush_bytes = flush_bytes.wrapping_sub(1)
                }
                if !((*zip).ppstream.avail_out != 0
                    && ((*zip).ppstream.avail_in != 0 || flush_bytes != 0))
                {
                    break;
                }
            }
            t_avail_in = (*zip).ppstream.avail_in as size_t;
            t_avail_out = (*zip).ppstream.avail_out as size_t
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Decompression internal error\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
    }
    if ret != ARCHIVE_OK && ret != ARCHIVE_EOF {
        return ret;
    }
    *used = o_avail_in.wrapping_sub(t_avail_in);
    *outbytes = o_avail_out.wrapping_sub(t_avail_out);
    /*
     * Decord BCJ.
     */
    if (*zip).codec != _7Z_LZMA2 as libc::c_ulong && (*zip).codec2 == _7Z_X86 as libc::c_ulong {
        let mut l: size_t = x86_Convert(zip, buff as *mut uint8_t, *outbytes);
        (*zip).odd_bcj_size = (*outbytes).wrapping_sub(l);
        if (*zip).odd_bcj_size > 0 as libc::c_int as libc::c_ulong
            && (*zip).odd_bcj_size <= 4 as libc::c_int as libc::c_ulong
            && o_avail_in != 0
            && ret != ARCHIVE_EOF
        {
            memcpy(
                (*zip).odd_bcj.as_mut_ptr() as *mut libc::c_void,
                (buff as *mut libc::c_uchar).offset(l as isize) as *const libc::c_void,
                (*zip).odd_bcj_size,
            );
            *outbytes = l
        } else {
            (*zip).odd_bcj_size = 0 as libc::c_int as size_t
        }
    }
    /*
     * Decord BCJ2 with a decompressed main stream.
     */
    if (*zip).codec2 == _7Z_X86_BCJ2 as libc::c_ulong {
        let mut bytes_1: ssize_t = 0;
        (*zip).tmp_stream_bytes_avail = (*zip).tmp_stream_buff_size.wrapping_sub(t_avail_out);
        if (*zip).tmp_stream_bytes_avail > (*zip).main_stream_bytes_remaining {
            (*zip).tmp_stream_bytes_avail = (*zip).main_stream_bytes_remaining
        }
        (*zip).tmp_stream_bytes_remaining = (*zip).tmp_stream_bytes_avail;
        bytes_1 = Bcj2_Decode(zip, bcj2_next_out, bcj2_avail_out);
        if bytes_1 < 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"BCJ2 conversion Failed\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int);
        }
        (*zip).main_stream_bytes_remaining = ((*zip).main_stream_bytes_remaining as libc::c_ulong)
            .wrapping_sub(
                (*zip)
                    .tmp_stream_bytes_avail
                    .wrapping_sub((*zip).tmp_stream_bytes_remaining),
            ) as size_t as size_t;
        bcj2_avail_out = (bcj2_avail_out as libc::c_ulong).wrapping_sub(bytes_1 as libc::c_ulong)
            as size_t as size_t;
        *outbytes = o_avail_out.wrapping_sub(bcj2_avail_out)
    }
    return ret;
}
unsafe extern "C" fn free_decompression(
    mut a: *mut archive_read,
    mut zip: *mut _7zip,
) -> libc::c_int {
    let mut r: libc::c_int = ARCHIVE_OK;
    if (*zip).stream_valid != 0 {
        if inflateEnd(&mut (*zip).stream) != Z_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Failed to clean up zlib decompressor\x00" as *const u8 as *const libc::c_char,
            );
            r = ARCHIVE_FATAL
        }
        (*zip).stream_valid = 0 as libc::c_int
    }
    if (*zip).ppmd7_valid != 0 {
        __archive_ppmd7_functions
            .Ppmd7_Free
            .expect("non-null function pointer")(&mut (*zip).ppmd7_context);
        (*zip).ppmd7_valid = 0 as libc::c_int
    }
    return r;
}
unsafe extern "C" fn parse_7zip_uint64(
    mut a: *mut archive_read,
    mut val: *mut uint64_t,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut avail: libc::c_uchar = 0;
    let mut mask: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    avail = *p;
    mask = 0x80 as libc::c_int as libc::c_uchar;
    *val = 0 as libc::c_int as uint64_t;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if avail as libc::c_int & mask as libc::c_int != 0 {
            p = header_bytes(a, 1 as libc::c_int as size_t);
            if p.is_null() {
                return -(1 as libc::c_int);
            }
            *val |= (*p as uint64_t) << 8 as libc::c_int * i;
            mask = (mask as libc::c_int >> 1 as libc::c_int) as libc::c_uchar;
            i += 1
        } else {
            *val = (*val as libc::c_ulong).wrapping_add(
                ((avail as libc::c_int & mask as libc::c_int - 1 as libc::c_int) as uint64_t)
                    << 8 as libc::c_int * i,
            ) as uint64_t as uint64_t;
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_Bools(
    mut a: *mut archive_read,
    mut data: *mut libc::c_uchar,
    mut num: size_t,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut avail: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < num {
        if mask == 0 as libc::c_int as libc::c_uint {
            p = header_bytes(a, 1 as libc::c_int as size_t);
            if p.is_null() {
                return -(1 as libc::c_int);
            }
            avail = *p as libc::c_uint;
            mask = 0x80 as libc::c_int as libc::c_uint
        }
        *data.offset(i as isize) = if avail & mask != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } as libc::c_uchar;
        mask >>= 1 as libc::c_int;
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_Digest(mut d: *mut _7z_digests) {
    free((*d).defineds as *mut libc::c_void);
    free((*d).digests as *mut libc::c_void);
}
unsafe extern "C" fn read_Digests(
    mut a: *mut archive_read,
    mut d: *mut _7z_digests,
    mut num: size_t,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut i: libc::c_uint = 0;
    if num == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memset(
        d as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_7z_digests>() as libc::c_ulong,
    );
    (*d).defineds = malloc(num) as *mut libc::c_uchar;
    if (*d).defineds.is_null() {
        return -(1 as libc::c_int);
    }
    /*
     * Read Bools.
     */
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if *p as libc::c_int == 0 as libc::c_int {
        if read_Bools(a, (*d).defineds, num) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else {
        /* All are defined */
        memset((*d).defineds as *mut libc::c_void, 1 as libc::c_int, num);
    }
    (*d).digests = calloc(num, ::std::mem::size_of::<uint32_t>() as libc::c_ulong) as *mut uint32_t;
    if (*d).digests.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < num {
        if *(*d).defineds.offset(i as isize) != 0 {
            p = header_bytes(a, 4 as libc::c_int as size_t);
            if p.is_null() {
                return -(1 as libc::c_int);
            }
            *(*d).digests.offset(i as isize) = archive_le32dec(p as *const libc::c_void)
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_PackInfo(mut pi: *mut _7z_pack_info) {
    free((*pi).sizes as *mut libc::c_void);
    free((*pi).positions as *mut libc::c_void);
    free_Digest(&mut (*pi).digest);
}
unsafe extern "C" fn read_PackInfo(
    mut a: *mut archive_read,
    mut pi: *mut _7z_pack_info,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut i: libc::c_uint = 0;
    memset(
        pi as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_7z_pack_info>() as libc::c_ulong,
    );
    /*
     * Read PackPos.
     */
    if parse_7zip_uint64(a, &mut (*pi).pos) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /*
     * Read NumPackStreams.
     */
    if parse_7zip_uint64(a, &mut (*pi).numPackStreams) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*pi).numPackStreams == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if (100000000 as libc::c_ulonglong) < (*pi).numPackStreams as libc::c_ulonglong {
        return -(1 as libc::c_int);
    }
    /*
     * Read PackSizes[num]
     */
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if *p as libc::c_int == kEnd {
        /* PackSizes[num] are not present. */
        return 0 as libc::c_int;
    }
    if *p as libc::c_int != kSize {
        return -(1 as libc::c_int);
    }
    (*pi).sizes = calloc(
        (*pi).numPackStreams,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) as *mut uint64_t;
    (*pi).positions = calloc(
        (*pi).numPackStreams,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) as *mut uint64_t;
    if (*pi).sizes.is_null() || (*pi).positions.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*pi).numPackStreams {
        if parse_7zip_uint64(a, &mut *(*pi).sizes.offset(i as isize)) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    /*
     * Read PackStreamDigests[num]
     */
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if *p as libc::c_int == kEnd {
        /* PackStreamDigests[num] are not present. */
        (*pi).digest.defineds = calloc(
            (*pi).numPackStreams,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
        ) as *mut libc::c_uchar;
        (*pi).digest.digests = calloc(
            (*pi).numPackStreams,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        ) as *mut uint32_t;
        if (*pi).digest.defineds.is_null() || (*pi).digest.digests.is_null() {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if *p as libc::c_int != kCRC {
        return -(1 as libc::c_int);
    }
    if read_Digests(a, &mut (*pi).digest, (*pi).numPackStreams) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    /*
     *  Must be marked by kEnd.
     */
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if *p as libc::c_int != kEnd {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_Folder(mut f: *mut _7z_folder) {
    let mut i: libc::c_uint = 0;
    if !(*f).coders.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*f).numCoders {
            free((*(*f).coders.offset(i as isize)).properties as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free((*f).coders as *mut libc::c_void);
    }
    free((*f).bindPairs as *mut libc::c_void);
    free((*f).packedStreams as *mut libc::c_void);
    free((*f).unPackSize as *mut libc::c_void);
}
unsafe extern "C" fn read_Folder(mut a: *mut archive_read, mut f: *mut _7z_folder) -> libc::c_int {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut numInStreamsTotal: uint64_t = 0 as libc::c_int as uint64_t;
    let mut numOutStreamsTotal: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_uint = 0;
    memset(
        f as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_7z_folder>() as libc::c_ulong,
    );
    /*
     * Read NumCoders.
     */
    if parse_7zip_uint64(a, &mut (*f).numCoders) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*f).numCoders > 4 as libc::c_int as libc::c_ulong {
        /* Too many coders. */
        return -(1 as libc::c_int);
    }
    (*f).coders = calloc(
        (*f).numCoders,
        ::std::mem::size_of::<_7z_coder>() as libc::c_ulong,
    ) as *mut _7z_coder;
    if (*f).coders.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*f).numCoders {
        let mut codec_size: size_t = 0;
        let mut simple: libc::c_int = 0;
        let mut attr: libc::c_int = 0;
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        /*
         * 0:3 CodecIdSize
         * 4:  0 - IsSimple
         *     1 - Is not Simple
         * 5:  0 - No Attributes
         *     1 - There are Attributes;
         * 7:  Must be zero.
         */
        codec_size = (*p as libc::c_int & 0xf as libc::c_int) as size_t; /* Not supported. */
        simple = if *p as libc::c_int & 0x10 as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
        attr = *p as libc::c_int & 0x20 as libc::c_int;
        if *p as libc::c_int & 0x80 as libc::c_int != 0 {
            return -(1 as libc::c_int);
        }
        /*
         * Read Decompression Method IDs.
         */
        p = header_bytes(a, codec_size);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        (*(*f).coders.offset(i as isize)).codec = decode_codec_id(p, codec_size);
        if simple != 0 {
            (*(*f).coders.offset(i as isize)).numInStreams = 1 as libc::c_int as uint64_t;
            (*(*f).coders.offset(i as isize)).numOutStreams = 1 as libc::c_int as uint64_t
        } else {
            if parse_7zip_uint64(a, &mut (*(*f).coders.offset(i as isize)).numInStreams)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if (100000000 as libc::c_ulonglong)
                < (*(*f).coders.offset(i as isize)).numInStreams as libc::c_ulonglong
            {
                return -(1 as libc::c_int);
            }
            if parse_7zip_uint64(a, &mut (*(*f).coders.offset(i as isize)).numOutStreams)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if (100000000 as libc::c_ulonglong)
                < (*(*f).coders.offset(i as isize)).numOutStreams as libc::c_ulonglong
            {
                return -(1 as libc::c_int);
            }
        }
        if attr != 0 {
            if parse_7zip_uint64(a, &mut (*(*f).coders.offset(i as isize)).propertiesSize)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            p = header_bytes(a, (*(*f).coders.offset(i as isize)).propertiesSize);
            if p.is_null() {
                return -(1 as libc::c_int);
            }
            let ref mut fresh3 = (*(*f).coders.offset(i as isize)).properties;
            *fresh3 =
                malloc((*(*f).coders.offset(i as isize)).propertiesSize) as *mut libc::c_uchar;
            if (*(*f).coders.offset(i as isize)).properties.is_null() {
                return -(1 as libc::c_int);
            }
            memcpy(
                (*(*f).coders.offset(i as isize)).properties as *mut libc::c_void,
                p as *const libc::c_void,
                (*(*f).coders.offset(i as isize)).propertiesSize,
            );
        }
        numInStreamsTotal = (numInStreamsTotal as libc::c_ulong)
            .wrapping_add((*(*f).coders.offset(i as isize)).numInStreams)
            as uint64_t as uint64_t;
        numOutStreamsTotal = (numOutStreamsTotal as libc::c_ulong)
            .wrapping_add((*(*f).coders.offset(i as isize)).numOutStreams)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1)
    }
    if numOutStreamsTotal == 0 as libc::c_int as libc::c_ulong
        || numInStreamsTotal < numOutStreamsTotal.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    (*f).numBindPairs = numOutStreamsTotal.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if (*zip).header_bytes_remaining < (*f).numBindPairs {
        return -(1 as libc::c_int);
    }
    if (*f).numBindPairs > 0 as libc::c_int as libc::c_ulong {
        (*f).bindPairs = calloc(
            (*f).numBindPairs,
            ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong,
        ) as *mut C2RustUnnamed_1;
        if (*f).bindPairs.is_null() {
            return -(1 as libc::c_int);
        }
    } else {
        (*f).bindPairs = NULL as *mut C2RustUnnamed_1
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*f).numBindPairs {
        if parse_7zip_uint64(a, &mut (*(*f).bindPairs.offset(i as isize)).inIndex)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (100000000 as libc::c_ulonglong)
            < (*(*f).bindPairs.offset(i as isize)).inIndex as libc::c_ulonglong
        {
            return -(1 as libc::c_int);
        }
        if parse_7zip_uint64(a, &mut (*(*f).bindPairs.offset(i as isize)).outIndex)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (100000000 as libc::c_ulonglong)
            < (*(*f).bindPairs.offset(i as isize)).outIndex as libc::c_ulonglong
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    (*f).numPackedStreams = numInStreamsTotal.wrapping_sub((*f).numBindPairs);
    (*f).packedStreams = calloc(
        (*f).numPackedStreams,
        ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) as *mut uint64_t;
    if (*f).packedStreams.is_null() {
        return -(1 as libc::c_int);
    }
    if (*f).numPackedStreams == 1 as libc::c_int as libc::c_ulong {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < numInStreamsTotal {
            let mut j: libc::c_uint = 0;
            j = 0 as libc::c_int as libc::c_uint;
            while (j as libc::c_ulong) < (*f).numBindPairs {
                if (*(*f).bindPairs.offset(j as isize)).inIndex == i as libc::c_ulong {
                    break;
                }
                j = j.wrapping_add(1)
            }
            if j as libc::c_ulong == (*f).numBindPairs {
                break;
            }
            i = i.wrapping_add(1)
        }
        if i as libc::c_ulong == numInStreamsTotal {
            return -(1 as libc::c_int);
        }
        *(*f).packedStreams.offset(0 as libc::c_int as isize) = i as uint64_t
    } else {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*f).numPackedStreams {
            if parse_7zip_uint64(a, &mut *(*f).packedStreams.offset(i as isize)) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if (100000000 as libc::c_ulonglong)
                < *(*f).packedStreams.offset(i as isize) as libc::c_ulonglong
            {
                return -(1 as libc::c_int);
            }
            i = i.wrapping_add(1)
        }
    }
    (*f).numInStreams = numInStreamsTotal;
    (*f).numOutStreams = numOutStreamsTotal;
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_CodersInfo(mut ci: *mut _7z_coders_info) {
    let mut i: libc::c_uint = 0;
    if !(*ci).folders.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*ci).numFolders {
            free_Folder(&mut *(*ci).folders.offset(i as isize));
            i = i.wrapping_add(1)
        }
        free((*ci).folders as *mut libc::c_void);
    };
}
unsafe extern "C" fn read_CodersInfo(
    mut a: *mut archive_read,
    mut ci: *mut _7z_coders_info,
) -> libc::c_int {
    let mut current_block: u64;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut digest: _7z_digests = _7z_digests {
        defineds: 0 as *mut libc::c_uchar,
        digests: 0 as *mut uint32_t,
    };
    let mut i: libc::c_uint = 0;
    memset(
        ci as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_7z_coders_info>() as libc::c_ulong,
    );
    memset(
        &mut digest as *mut _7z_digests as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_7z_digests>() as libc::c_ulong,
    );
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if !p.is_null() {
        if !(*p as libc::c_int != kFolder) {
            /*
             * Read NumFolders.
             */
            if !(parse_7zip_uint64(a, &mut (*ci).numFolders) < 0 as libc::c_int) {
                if (100000000 as libc::c_ulonglong) < (*ci).numFolders as libc::c_ulonglong {
                    return -(1 as libc::c_int);
                }
                /*
                 * Read External.
                 */
                p = header_bytes(a, 1 as libc::c_int as size_t);
                if !p.is_null() {
                    match *p as libc::c_int {
                        0 => {
                            (*ci).folders = calloc(
                                (*ci).numFolders,
                                ::std::mem::size_of::<_7z_folder>() as libc::c_ulong,
                            ) as *mut _7z_folder;
                            if (*ci).folders.is_null() {
                                return -(1 as libc::c_int);
                            }
                            i = 0 as libc::c_int as libc::c_uint;
                            loop {
                                if !((i as libc::c_ulong) < (*ci).numFolders) {
                                    current_block = 4068382217303356765;
                                    break;
                                }
                                if read_Folder(a, &mut *(*ci).folders.offset(i as isize))
                                    < 0 as libc::c_int
                                {
                                    current_block = 1544091447943322374;
                                    break;
                                }
                                i = i.wrapping_add(1)
                            }
                        }
                        1 => {
                            if parse_7zip_uint64(a, &mut (*ci).dataStreamIndex) < 0 as libc::c_int {
                                return -(1 as libc::c_int);
                            }
                            if (100000000 as libc::c_ulonglong)
                                < (*ci).dataStreamIndex as libc::c_ulonglong
                            {
                                return -(1 as libc::c_int);
                            }
                            if (*ci).numFolders > 0 as libc::c_int as libc::c_ulong {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    -(1 as libc::c_int),
                                    b"Malformed 7-Zip archive\x00" as *const u8
                                        as *const libc::c_char,
                                );
                                current_block = 1544091447943322374;
                            } else {
                                current_block = 4068382217303356765;
                            }
                        }
                        _ => {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                -(1 as libc::c_int),
                                b"Malformed 7-Zip archive\x00" as *const u8 as *const libc::c_char,
                            );
                            current_block = 1544091447943322374;
                        }
                    }
                    match current_block {
                        1544091447943322374 => {}
                        _ => {
                            p = header_bytes(a, 1 as libc::c_int as size_t);
                            if !p.is_null() {
                                if !(*p as libc::c_int != kCodersUnPackSize) {
                                    i = 0 as libc::c_int as libc::c_uint;
                                    's_148: loop {
                                        if !((i as libc::c_ulong) < (*ci).numFolders) {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        let mut folder: *mut _7z_folder =
                                            &mut *(*ci).folders.offset(i as isize)
                                                as *mut _7z_folder;
                                        let mut j: libc::c_uint = 0;
                                        (*folder).unPackSize = calloc(
                                            (*folder).numOutStreams,
                                            ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
                                        )
                                            as *mut uint64_t;
                                        if (*folder).unPackSize.is_null() {
                                            current_block = 1544091447943322374;
                                            break;
                                        }
                                        j = 0 as libc::c_int as libc::c_uint;
                                        while (j as libc::c_ulong) < (*folder).numOutStreams {
                                            if parse_7zip_uint64(
                                                a,
                                                &mut *(*folder).unPackSize.offset(j as isize),
                                            ) < 0 as libc::c_int
                                            {
                                                current_block = 1544091447943322374;
                                                break 's_148;
                                            }
                                            j = j.wrapping_add(1)
                                        }
                                        i = i.wrapping_add(1)
                                    }
                                    match current_block {
                                        1544091447943322374 => {}
                                        _ =>
                                        /*
                                         * Read CRCs.
                                         */
                                        {
                                            p = header_bytes(a, 1 as libc::c_int as size_t);
                                            if !p.is_null() {
                                                if *p as libc::c_int == kEnd {
                                                    return 0 as libc::c_int;
                                                }
                                                if !(*p as libc::c_int != kCRC) {
                                                    if !(read_Digests(
                                                        a,
                                                        &mut digest,
                                                        (*ci).numFolders,
                                                    ) < 0 as libc::c_int)
                                                    {
                                                        i = 0 as libc::c_int as libc::c_uint;
                                                        while (i as libc::c_ulong)
                                                            < (*ci).numFolders
                                                        {
                                                            (*(*ci).folders.offset(i as isize))
                                                                .digest_defined =
                                                                *digest.defineds.offset(i as isize);
                                                            (*(*ci).folders.offset(i as isize))
                                                                .digest =
                                                                *digest.digests.offset(i as isize);
                                                            i = i.wrapping_add(1)
                                                        }
                                                        /*
                                                         *  Must be kEnd.
                                                         */
                                                        p = header_bytes(
                                                            a,
                                                            1 as libc::c_int as size_t,
                                                        );
                                                        if !p.is_null() {
                                                            if !(*p as libc::c_int != kEnd) {
                                                                free_Digest(&mut digest);
                                                                return 0 as libc::c_int;
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
            }
        }
    }
    free_Digest(&mut digest);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn folder_uncompressed_size(mut f: *mut _7z_folder) -> uint64_t {
    let mut n: libc::c_int = (*f).numOutStreams as libc::c_int;
    let mut pairs: libc::c_uint = (*f).numBindPairs as libc::c_uint;
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int) {
            break;
        }
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while i < pairs {
            if (*(*f).bindPairs.offset(i as isize)).outIndex == n as uint64_t {
                break;
            }
            i = i.wrapping_add(1)
        }
        if i >= pairs {
            return *(*f).unPackSize.offset(n as isize);
        }
    }
    return 0 as libc::c_int as uint64_t;
}
unsafe extern "C" fn free_SubStreamsInfo(mut ss: *mut _7z_substream_info) {
    free((*ss).unpackSizes as *mut libc::c_void);
    free((*ss).digestsDefined as *mut libc::c_void);
    free((*ss).digests as *mut libc::c_void);
}
unsafe extern "C" fn read_SubStreamsInfo(
    mut a: *mut archive_read,
    mut ss: *mut _7z_substream_info,
    mut f: *mut _7z_folder,
    mut numFolders: size_t,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut usizes: *mut uint64_t = 0 as *mut uint64_t;
    let mut unpack_streams: size_t = 0;
    let mut type_0: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut numDigests: uint32_t = 0;
    memset(
        ss as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_7z_substream_info>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < numFolders {
        (*f.offset(i as isize)).numUnpackStreams = 1 as libc::c_int as uint64_t;
        i = i.wrapping_add(1)
    }
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    type_0 = *p as libc::c_int;
    if type_0 == kNumUnPackStream {
        unpack_streams = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < numFolders {
            if parse_7zip_uint64(a, &mut (*f.offset(i as isize)).numUnpackStreams)
                < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if (100000000 as libc::c_ulonglong)
                < (*f.offset(i as isize)).numUnpackStreams as libc::c_ulonglong
            {
                return -(1 as libc::c_int);
            }
            if unpack_streams as libc::c_ulonglong
                > (SIZE_MAX as libc::c_ulonglong).wrapping_sub(100000000 as libc::c_ulonglong)
            {
                return -(1 as libc::c_int);
            }
            unpack_streams = (unpack_streams as libc::c_ulong)
                .wrapping_add((*f.offset(i as isize)).numUnpackStreams)
                as size_t as size_t;
            i = i.wrapping_add(1)
        }
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        type_0 = *p as libc::c_int
    } else {
        unpack_streams = numFolders
    }
    (*ss).unpack_streams = unpack_streams;
    if unpack_streams != 0 {
        (*ss).unpackSizes = calloc(
            unpack_streams,
            ::std::mem::size_of::<uint64_t>() as libc::c_ulong,
        ) as *mut uint64_t;
        (*ss).digestsDefined = calloc(
            unpack_streams,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
        ) as *mut libc::c_uchar;
        (*ss).digests = calloc(
            unpack_streams,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        ) as *mut uint32_t;
        if (*ss).unpackSizes.is_null() || (*ss).digestsDefined.is_null() || (*ss).digests.is_null()
        {
            return -(1 as libc::c_int);
        }
    }
    usizes = (*ss).unpackSizes;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < numFolders {
        let mut pack: libc::c_uint = 0;
        let mut sum: uint64_t = 0;
        if !((*f.offset(i as isize)).numUnpackStreams == 0 as libc::c_int as libc::c_ulong) {
            sum = 0 as libc::c_int as uint64_t;
            if type_0 == kSize {
                pack = 1 as libc::c_int as libc::c_uint;
                while (pack as libc::c_ulong) < (*f.offset(i as isize)).numUnpackStreams {
                    if parse_7zip_uint64(a, usizes) < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                    let fresh4 = usizes;
                    usizes = usizes.offset(1);
                    sum = (sum as libc::c_ulong).wrapping_add(*fresh4) as uint64_t as uint64_t;
                    pack = pack.wrapping_add(1)
                }
            }
            let fresh5 = usizes;
            usizes = usizes.offset(1);
            *fresh5 = folder_uncompressed_size(&mut *f.offset(i as isize)).wrapping_sub(sum)
        }
        i = i.wrapping_add(1)
    }
    if type_0 == kSize {
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        type_0 = *p as libc::c_int
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < unpack_streams {
        *(*ss).digestsDefined.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
        *(*ss).digests.offset(i as isize) = 0 as libc::c_int as uint32_t;
        i = i.wrapping_add(1)
    }
    numDigests = 0 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < numFolders {
        if (*f.offset(i as isize)).numUnpackStreams != 1 as libc::c_int as libc::c_ulong
            || (*f.offset(i as isize)).digest_defined == 0
        {
            numDigests = (numDigests as libc::c_uint)
                .wrapping_add((*f.offset(i as isize)).numUnpackStreams as uint32_t)
                as uint32_t as uint32_t
        }
        i = i.wrapping_add(1)
    }
    if type_0 == kCRC {
        let mut tmpDigests: _7z_digests = _7z_digests {
            defineds: 0 as *mut libc::c_uchar,
            digests: 0 as *mut uint32_t,
        };
        let mut digestsDefined: *mut libc::c_uchar = (*ss).digestsDefined;
        let mut digests: *mut uint32_t = (*ss).digests;
        let mut di: libc::c_int = 0 as libc::c_int;
        memset(
            &mut tmpDigests as *mut _7z_digests as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<_7z_digests>() as libc::c_ulong,
        );
        if read_Digests(a, &mut tmpDigests, numDigests as size_t) < 0 as libc::c_int {
            free_Digest(&mut tmpDigests);
            return -(1 as libc::c_int);
        }
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < numFolders {
            if (*f.offset(i as isize)).numUnpackStreams == 1 as libc::c_int as libc::c_ulong
                && (*f.offset(i as isize)).digest_defined as libc::c_int != 0
            {
                let fresh6 = digestsDefined;
                digestsDefined = digestsDefined.offset(1);
                *fresh6 = 1 as libc::c_int as libc::c_uchar;
                let fresh7 = digests;
                digests = digests.offset(1);
                *fresh7 = (*f.offset(i as isize)).digest
            } else {
                let mut j: libc::c_uint = 0;
                j = 0 as libc::c_int as libc::c_uint;
                while (j as libc::c_ulong) < (*f.offset(i as isize)).numUnpackStreams {
                    let fresh8 = digestsDefined;
                    digestsDefined = digestsDefined.offset(1);
                    *fresh8 = *tmpDigests.defineds.offset(di as isize);
                    let fresh9 = digests;
                    digests = digests.offset(1);
                    *fresh9 = *tmpDigests.digests.offset(di as isize);
                    j = j.wrapping_add(1);
                    di += 1
                }
            }
            i = i.wrapping_add(1)
        }
        free_Digest(&mut tmpDigests);
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        type_0 = *p as libc::c_int
    }
    /*
     *  Must be kEnd.
     */
    if type_0 != kEnd {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_StreamsInfo(mut si: *mut _7z_stream_info) {
    free_PackInfo(&mut (*si).pi);
    free_CodersInfo(&mut (*si).ci);
    free_SubStreamsInfo(&mut (*si).ss);
}
unsafe extern "C" fn read_StreamsInfo(
    mut a: *mut archive_read,
    mut si: *mut _7z_stream_info,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut i: libc::c_uint = 0;
    memset(
        si as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<_7z_stream_info>() as libc::c_ulong,
    );
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if *p as libc::c_int == kPackInfo {
        let mut packPos: uint64_t = 0;
        if read_PackInfo(a, &mut (*si).pi) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if (*si).pi.positions.is_null() || (*si).pi.sizes.is_null() {
            return -(1 as libc::c_int);
        }
        /*
         * Calculate packed stream positions.
         */
        packPos = (*si).pi.pos;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*si).pi.numPackStreams {
            *(*si).pi.positions.offset(i as isize) = packPos;
            packPos = (packPos as libc::c_ulong).wrapping_add(*(*si).pi.sizes.offset(i as isize))
                as uint64_t as uint64_t;
            if packPos > (*zip).header_offset {
                return -(1 as libc::c_int);
            }
            i = i.wrapping_add(1)
        }
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
    }
    if *p as libc::c_int == kUnPackInfo {
        let mut packIndex: uint32_t = 0;
        let mut f: *mut _7z_folder = 0 as *mut _7z_folder;
        if read_CodersInfo(a, &mut (*si).ci) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        /*
         * Calculate packed stream indexes.
         */
        packIndex = 0 as libc::c_int as uint32_t;
        f = (*si).ci.folders;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < (*si).ci.numFolders {
            (*f.offset(i as isize)).packIndex = packIndex;
            packIndex = (packIndex as libc::c_uint)
                .wrapping_add((*f.offset(i as isize)).numPackedStreams as uint32_t)
                as uint32_t as uint32_t;
            if packIndex as libc::c_ulong > (*si).pi.numPackStreams {
                return -(1 as libc::c_int);
            }
            i = i.wrapping_add(1)
        }
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
    }
    if *p as libc::c_int == kSubStreamsInfo {
        if read_SubStreamsInfo(a, &mut (*si).ss, (*si).ci.folders, (*si).ci.numFolders)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
    }
    /*
     *  Must be kEnd.
     */
    if *p as libc::c_int != kEnd {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_Header(mut h: *mut _7z_header_info) {
    free((*h).emptyStreamBools as *mut libc::c_void);
    free((*h).emptyFileBools as *mut libc::c_void);
    free((*h).antiBools as *mut libc::c_void);
    free((*h).attrBools as *mut libc::c_void);
}
unsafe extern "C" fn read_Header(
    mut a: *mut archive_read,
    mut h: *mut _7z_header_info,
    mut check_header_id: libc::c_int,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut folders: *mut _7z_folder = 0 as *mut _7z_folder;
    let mut si: *mut _7z_stream_info = &mut (*zip).si;
    let mut entries: *mut _7zip_entry = 0 as *mut _7zip_entry;
    let mut folderIndex: uint32_t = 0;
    let mut indexInFolder: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    let mut eindex: libc::c_int = 0;
    let mut empty_streams: libc::c_int = 0;
    let mut sindex: libc::c_int = 0;
    if check_header_id != 0 {
        /*
         * Read Header.
         */
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        if *p as libc::c_int != kHeader {
            return -(1 as libc::c_int);
        }
    }
    /*
     * Read ArchiveProperties.
     */
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if *p as libc::c_int == kArchiveProperties {
        loop {
            let mut size: uint64_t = 0;
            p = header_bytes(a, 1 as libc::c_int as size_t);
            if p.is_null() {
                return -(1 as libc::c_int);
            }
            if *p as libc::c_int == 0 as libc::c_int {
                break;
            }
            if parse_7zip_uint64(a, &mut size) < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
        }
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
    }
    /*
     * Read MainStreamsInfo.
     */
    if *p as libc::c_int == kMainStreamsInfo {
        if read_StreamsInfo(a, &mut (*zip).si) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
    }
    if *p as libc::c_int == kEnd {
        return 0 as libc::c_int;
    }
    /*
     * Read FilesInfo.
     */
    if *p as libc::c_int != kFilesInfo {
        return -(1 as libc::c_int);
    }
    if parse_7zip_uint64(a, &mut (*zip).numFiles) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (100000000 as libc::c_ulonglong) < (*zip).numFiles as libc::c_ulonglong {
        return -(1 as libc::c_int);
    }
    (*zip).entries = calloc(
        (*zip).numFiles,
        ::std::mem::size_of::<_7zip_entry>() as libc::c_ulong,
    ) as *mut _7zip_entry;
    if (*zip).entries.is_null() {
        return -(1 as libc::c_int);
    }
    entries = (*zip).entries;
    empty_streams = 0 as libc::c_int;
    loop {
        let mut type_0: libc::c_int = 0;
        let mut size_0: uint64_t = 0;
        let mut ll: size_t = 0;
        p = header_bytes(a, 1 as libc::c_int as size_t);
        if p.is_null() {
            return -(1 as libc::c_int);
        }
        type_0 = *p as libc::c_int;
        if type_0 == kEnd {
            break;
        }
        if parse_7zip_uint64(a, &mut size_0) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if (*zip).header_bytes_remaining < size_0 {
            return -(1 as libc::c_int);
        }
        ll = size_0;
        let mut current_block_137: u64;
        match type_0 {
            kEmptyStream => {
                if !(*h).emptyStreamBools.is_null() {
                    return -(1 as libc::c_int);
                }
                (*h).emptyStreamBools = calloc(
                    (*zip).numFiles,
                    ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                ) as *mut libc::c_uchar;
                if (*h).emptyStreamBools.is_null() {
                    return -(1 as libc::c_int);
                }
                if read_Bools(a, (*h).emptyStreamBools, (*zip).numFiles) < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                empty_streams = 0 as libc::c_int;
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong) < (*zip).numFiles {
                    if *(*h).emptyStreamBools.offset(i as isize) != 0 {
                        empty_streams += 1
                    }
                    i = i.wrapping_add(1)
                }
                current_block_137 = 7999014830792590863;
            }
            kEmptyFile => {
                if empty_streams <= 0 as libc::c_int {
                    /* Unexcepted sequence. Skip this. */
                    if header_bytes(a, ll).is_null() {
                        return -(1 as libc::c_int);
                    }
                } else {
                    if !(*h).emptyFileBools.is_null() {
                        return -(1 as libc::c_int);
                    }
                    (*h).emptyFileBools = calloc(
                        empty_streams as libc::c_ulong,
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ) as *mut libc::c_uchar;
                    if (*h).emptyFileBools.is_null() {
                        return -(1 as libc::c_int);
                    }
                    if read_Bools(a, (*h).emptyFileBools, empty_streams as size_t)
                        < 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                }
                current_block_137 = 7999014830792590863;
            }
            kAnti => {
                if empty_streams <= 0 as libc::c_int {
                    /* Unexcepted sequence. Skip this. */
                    if header_bytes(a, ll).is_null() {
                        return -(1 as libc::c_int);
                    }
                } else {
                    if !(*h).antiBools.is_null() {
                        return -(1 as libc::c_int);
                    }
                    (*h).antiBools = calloc(
                        empty_streams as libc::c_ulong,
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ) as *mut libc::c_uchar;
                    if (*h).antiBools.is_null() {
                        return -(1 as libc::c_int);
                    }
                    if read_Bools(a, (*h).antiBools, empty_streams as size_t) < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                }
                current_block_137 = 7999014830792590863;
            }
            kCTime | kATime | kMTime => {
                if read_Times(a, h, type_0) < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                current_block_137 = 7999014830792590863;
            }
            kName => {
                let mut np: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut nl: size_t = 0;
                let mut nb: size_t = 0;
                /* Skip one byte. */
                p = header_bytes(a, 1 as libc::c_int as size_t);
                if p.is_null() {
                    return -(1 as libc::c_int);
                }
                ll = ll.wrapping_sub(1);
                if ll & 1 as libc::c_int as libc::c_ulong != 0
                    || ll
                        < (*zip)
                            .numFiles
                            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
                {
                    return -(1 as libc::c_int);
                }
                if !(*zip).entry_names.is_null() {
                    return -(1 as libc::c_int);
                }
                (*zip).entry_names = malloc(ll) as *mut libc::c_uchar;
                if (*zip).entry_names.is_null() {
                    return -(1 as libc::c_int);
                }
                np = (*zip).entry_names;
                nb = ll;
                /*
                 * Copy whole file names.
                 * NOTE: This loop prevents from expanding
                 * the uncompressed buffer in order not to
                 * use extra memory resource.
                 */
                while nb != 0 {
                    let mut b: size_t = 0;
                    if nb > UBUFF_SIZE as libc::c_ulong {
                        b = UBUFF_SIZE as size_t
                    } else {
                        b = nb
                    }
                    p = header_bytes(a, b);
                    if p.is_null() {
                        return -(1 as libc::c_int);
                    }
                    memcpy(np as *mut libc::c_void, p as *const libc::c_void, b);
                    np = np.offset(b as isize);
                    nb = (nb as libc::c_ulong).wrapping_sub(b) as size_t as size_t
                }
                np = (*zip).entry_names;
                nl = ll;
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong) < (*zip).numFiles {
                    let ref mut fresh10 = (*entries.offset(i as isize)).utf16name;
                    *fresh10 = np;
                    /* Find a terminator. */
                    while nl >= 2 as libc::c_int as libc::c_ulong
                        && (*np.offset(0 as libc::c_int as isize) as libc::c_int != 0
                            || *np.offset(1 as libc::c_int as isize) as libc::c_int != 0)
                    {
                        np = np.offset(2 as libc::c_int as isize); /* Terminator not found */
                        nl = (nl as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                            as size_t as size_t
                    }
                    if nl < 2 as libc::c_int as libc::c_ulong {
                        return -(1 as libc::c_int);
                    }
                    (*entries.offset(i as isize)).name_len =
                        np.offset_from((*entries.offset(i as isize)).utf16name)
                            as libc::c_long as size_t;
                    np = np.offset(2 as libc::c_int as isize);
                    nl = (nl as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                        as size_t as size_t;
                    i = i.wrapping_add(1)
                }
                current_block_137 = 7999014830792590863;
            }
            kAttributes => {
                let mut allAreDefined: libc::c_int = 0;
                p = header_bytes(a, 2 as libc::c_int as size_t);
                if p.is_null() {
                    return -(1 as libc::c_int);
                }
                allAreDefined = *p as libc::c_int;
                if !(*h).attrBools.is_null() {
                    return -(1 as libc::c_int);
                }
                (*h).attrBools = calloc(
                    (*zip).numFiles,
                    ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                ) as *mut libc::c_uchar;
                if (*h).attrBools.is_null() {
                    return -(1 as libc::c_int);
                }
                if allAreDefined != 0 {
                    memset(
                        (*h).attrBools as *mut libc::c_void,
                        1 as libc::c_int,
                        (*zip).numFiles,
                    );
                } else if read_Bools(a, (*h).attrBools, (*zip).numFiles) < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong) < (*zip).numFiles {
                    if *(*h).attrBools.offset(i as isize) != 0 {
                        p = header_bytes(a, 4 as libc::c_int as size_t);
                        if p.is_null() {
                            return -(1 as libc::c_int);
                        }
                        (*entries.offset(i as isize)).attr =
                            archive_le32dec(p as *const libc::c_void)
                    }
                    i = i.wrapping_add(1)
                }
                current_block_137 = 7999014830792590863;
            }
            kDummy => {
                if ll == 0 as libc::c_int as libc::c_ulong {
                    current_block_137 = 7999014830792590863;
                } else {
                    current_block_137 = 9379646460352968115;
                }
            }
            _ => {
                current_block_137 = 9379646460352968115;
            }
        }
        match current_block_137 {
            9379646460352968115 => {
                if header_bytes(a, ll).is_null() {
                    return -(1 as libc::c_int);
                }
            }
            _ => {}
        }
    }
    /*
     * Set up entry's attributes.
     */
    folders = (*si).ci.folders;
    sindex = 0 as libc::c_int;
    eindex = sindex;
    indexInFolder = 0 as libc::c_int as uint32_t;
    folderIndex = indexInFolder;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*zip).numFiles {
        if (*h).emptyStreamBools.is_null()
            || *(*h).emptyStreamBools.offset(i as isize) as libc::c_int == 0 as libc::c_int
        {
            (*entries.offset(i as isize)).flg |= HAS_STREAM as libc::c_uint
        }
        /* The high 16 bits of attributes is a posix file mode. */
        (*entries.offset(i as isize)).mode =
            (*entries.offset(i as isize)).attr >> 16 as libc::c_int; /* Read only. */
        if (*entries.offset(i as isize)).flg & HAS_STREAM as libc::c_uint != 0 {
            if sindex as size_t >= (*si).ss.unpack_streams {
                return -(1 as libc::c_int);
            }
            if (*entries.offset(i as isize)).mode == 0 as libc::c_int as libc::c_uint {
                (*entries.offset(i as isize)).mode =
                    AE_IFREG as mode_t | 0o666 as libc::c_int as libc::c_uint
            }
            if *(*si).ss.digestsDefined.offset(sindex as isize) != 0 {
                (*entries.offset(i as isize)).flg |= CRC32_IS_SET as libc::c_uint
            }
            (*entries.offset(i as isize)).ssIndex = sindex as uint32_t;
            sindex += 1
        } else {
            let mut dir: libc::c_int = 0;
            if (*h).emptyFileBools.is_null() {
                dir = 1 as libc::c_int
            } else {
                if *(*h).emptyFileBools.offset(eindex as isize) != 0 {
                    dir = 0 as libc::c_int
                } else {
                    dir = 1 as libc::c_int
                }
                eindex += 1
            }
            if (*entries.offset(i as isize)).mode == 0 as libc::c_int as libc::c_uint {
                if dir != 0 {
                    (*entries.offset(i as isize)).mode =
                        AE_IFDIR as mode_t | 0o777 as libc::c_int as libc::c_uint
                } else {
                    (*entries.offset(i as isize)).mode =
                        AE_IFREG as mode_t | 0o666 as libc::c_int as libc::c_uint
                }
            } else if dir != 0
                && (*entries.offset(i as isize)).mode & AE_IFMT as mode_t != AE_IFDIR as mode_t
            {
                let ref mut fresh11 = (*entries.offset(i as isize)).mode;
                *fresh11 &= !(AE_IFMT as mode_t);
                let ref mut fresh12 = (*entries.offset(i as isize)).mode;
                *fresh12 |= AE_IFDIR as mode_t
            }
            if (*entries.offset(i as isize)).mode & AE_IFMT as mode_t == AE_IFDIR as mode_t
                && (*entries.offset(i as isize)).name_len >= 2 as libc::c_int as libc::c_ulong
                && (*(*entries.offset(i as isize)).utf16name.offset(
                    (*entries.offset(i as isize))
                        .name_len
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int
                    != '/' as i32
                    || *(*entries.offset(i as isize)).utf16name.offset(
                        (*entries.offset(i as isize))
                            .name_len
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int
                        != 0 as libc::c_int)
            {
                *(*entries.offset(i as isize))
                    .utf16name
                    .offset((*entries.offset(i as isize)).name_len as isize) =
                    '/' as i32 as libc::c_uchar;
                *(*entries.offset(i as isize)).utf16name.offset(
                    (*entries.offset(i as isize))
                        .name_len
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) = 0 as libc::c_int as libc::c_uchar;
                let ref mut fresh13 = (*entries.offset(i as isize)).name_len;
                *fresh13 = (*fresh13 as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            }
            (*entries.offset(i as isize)).ssIndex = -(1 as libc::c_int) as uint32_t
        }
        if (*entries.offset(i as isize)).attr & 0x1 as libc::c_int as libc::c_uint != 0 {
            let ref mut fresh14 = (*entries.offset(i as isize)).mode;
            *fresh14 &= !(0o222 as libc::c_int) as libc::c_uint
        }
        if (*entries.offset(i as isize)).flg & HAS_STREAM as libc::c_uint
            == 0 as libc::c_int as libc::c_uint
            && indexInFolder == 0 as libc::c_int as libc::c_uint
        {
            /*
             * The entry is an empty file or a directory file,
             * those both have no contents.
             */
            (*entries.offset(i as isize)).folderIndex = -(1 as libc::c_int) as uint32_t
        } else {
            if indexInFolder == 0 as libc::c_int as libc::c_uint {
                loop {
                    if folderIndex as libc::c_ulong >= (*si).ci.numFolders {
                        return -(1 as libc::c_int);
                    }
                    if (*folders.offset(folderIndex as isize)).numUnpackStreams != 0 {
                        break;
                    }
                    folderIndex = folderIndex.wrapping_add(1)
                }
            }
            (*entries.offset(i as isize)).folderIndex = folderIndex;
            if !((*entries.offset(i as isize)).flg & HAS_STREAM as libc::c_uint
                == 0 as libc::c_int as libc::c_uint)
            {
                indexInFolder = indexInFolder.wrapping_add(1);
                if indexInFolder as libc::c_ulong
                    >= (*folders.offset(folderIndex as isize)).numUnpackStreams
                {
                    folderIndex = folderIndex.wrapping_add(1);
                    indexInFolder = 0 as libc::c_int as uint32_t
                }
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fileTimeToUtc(
    mut fileTime: uint64_t,
    mut timep: *mut time_t,
    mut ns: *mut libc::c_long,
) {
    if fileTime as libc::c_ulonglong >= 116444736000000000 as libc::c_ulonglong {
        fileTime = (fileTime as libc::c_ulonglong)
            .wrapping_sub(116444736000000000 as libc::c_ulonglong) as uint64_t
            as uint64_t;
        /* milli seconds base */
        *timep = fileTime.wrapping_div(10000000 as libc::c_int as libc::c_ulong) as time_t;
        /* nano seconds base */
        *ns = fileTime.wrapping_rem(10000000 as libc::c_int as libc::c_ulong) as libc::c_long
            * 100 as libc::c_int as libc::c_long
    } else {
        *timep = 0 as libc::c_int as time_t;
        *ns = 0 as libc::c_int as libc::c_long
    };
}
unsafe extern "C" fn read_Times(
    mut a: *mut archive_read,
    mut h: *mut _7z_header_info,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut entries: *mut _7zip_entry = (*zip).entries;
    let mut timeBools: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut allAreDefined: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    timeBools = calloc(
        (*zip).numFiles,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if timeBools.is_null() {
        return -(1 as libc::c_int);
    }
    /* Read allAreDefined. */
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if !p.is_null() {
        allAreDefined = *p as libc::c_int;
        if allAreDefined != 0 {
            memset(
                timeBools as *mut libc::c_void,
                1 as libc::c_int,
                (*zip).numFiles,
            );
            current_block = 7746791466490516765;
        } else if read_Bools(a, timeBools, (*zip).numFiles) < 0 as libc::c_int {
            current_block = 1467979945751933765;
        } else {
            current_block = 7746791466490516765;
        }
        match current_block {
            1467979945751933765 => {}
            _ =>
            /* Read external. */
            {
                p = header_bytes(a, 1 as libc::c_int as size_t);
                if !p.is_null() {
                    if *p != 0 {
                        if parse_7zip_uint64(a, &mut (*h).dataIndex) < 0 as libc::c_int {
                            current_block = 1467979945751933765;
                        } else if (100000000 as libc::c_ulonglong)
                            < (*h).dataIndex as libc::c_ulonglong
                        {
                            current_block = 1467979945751933765;
                        } else {
                            current_block = 15976848397966268834;
                        }
                    } else {
                        current_block = 15976848397966268834;
                    }
                    match current_block {
                        1467979945751933765 => {}
                        _ => {
                            i = 0 as libc::c_int as libc::c_uint;
                            loop {
                                if !((i as libc::c_ulong) < (*zip).numFiles) {
                                    current_block = 8693738493027456495;
                                    break;
                                }
                                if !(*timeBools.offset(i as isize) == 0) {
                                    p = header_bytes(a, 8 as libc::c_int as size_t);
                                    if p.is_null() {
                                        current_block = 1467979945751933765;
                                        break;
                                    }
                                    match type_0 {
                                        kCTime => {
                                            fileTimeToUtc(
                                                archive_le64dec(p as *const libc::c_void),
                                                &mut (*entries.offset(i as isize)).ctime,
                                                &mut (*entries.offset(i as isize)).ctime_ns,
                                            );
                                            (*entries.offset(i as isize)).flg |=
                                                CTIME_IS_SET as libc::c_uint
                                        }
                                        kATime => {
                                            fileTimeToUtc(
                                                archive_le64dec(p as *const libc::c_void),
                                                &mut (*entries.offset(i as isize)).atime,
                                                &mut (*entries.offset(i as isize)).atime_ns,
                                            );
                                            (*entries.offset(i as isize)).flg |=
                                                ATIME_IS_SET as libc::c_uint
                                        }
                                        kMTime => {
                                            fileTimeToUtc(
                                                archive_le64dec(p as *const libc::c_void),
                                                &mut (*entries.offset(i as isize)).mtime,
                                                &mut (*entries.offset(i as isize)).mtime_ns,
                                            );
                                            (*entries.offset(i as isize)).flg |=
                                                MTIME_IS_SET as libc::c_uint
                                        }
                                        _ => {}
                                    }
                                }
                                i = i.wrapping_add(1)
                            }
                            match current_block {
                                1467979945751933765 => {}
                                _ => {
                                    free(timeBools as *mut libc::c_void);
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(timeBools as *mut libc::c_void);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn decode_encoded_header_info(
    mut a: *mut archive_read,
    mut si: *mut _7z_stream_info,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    errno = 0 as libc::c_int;
    if read_StreamsInfo(a, si) < 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                -(1 as libc::c_int),
                b"Couldn\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                -(1 as libc::c_int),
                b"Malformed 7-Zip archive\x00" as *const u8 as *const libc::c_char,
            );
        }
        return -(30 as libc::c_int);
    }
    if (*si).pi.numPackStreams == 0 as libc::c_int as libc::c_ulong
        || (*si).ci.numFolders == 0 as libc::c_int as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Malformed 7-Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*zip).header_offset
        < (*si)
            .pi
            .pos
            .wrapping_add(*(*si).pi.sizes.offset(0 as libc::c_int as isize))
        || ((*si)
            .pi
            .pos
            .wrapping_add(*(*si).pi.sizes.offset(0 as libc::c_int as isize)) as int64_t)
            < 0 as libc::c_int as libc::c_long
        || *(*si).pi.sizes.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_ulong
        || ((*si).pi.pos as int64_t) < 0 as libc::c_int as libc::c_long
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Malformed Header offset\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn header_bytes(
    mut a: *mut archive_read,
    mut rbytes: size_t,
) -> *const libc::c_uchar {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    if (*zip).header_bytes_remaining < rbytes {
        return 0 as *const libc::c_uchar;
    }
    if (*zip).pack_stream_bytes_unconsumed != 0 {
        read_consume(a);
    }
    if (*zip).header_is_encoded == 0 as libc::c_int {
        p = __archive_read_ahead(a, rbytes, NULL as *mut ssize_t) as *const libc::c_uchar;
        if p.is_null() {
            return 0 as *const libc::c_uchar;
        }
        (*zip).header_bytes_remaining = ((*zip).header_bytes_remaining as libc::c_ulong)
            .wrapping_sub(rbytes) as uint64_t as uint64_t;
        (*zip).pack_stream_bytes_unconsumed = rbytes
    } else {
        let mut buff: *const libc::c_void = 0 as *const libc::c_void;
        let mut bytes: ssize_t = 0;
        bytes = read_stream(a, &mut buff, rbytes, rbytes);
        if bytes <= 0 as libc::c_int as libc::c_long {
            return 0 as *const libc::c_uchar;
        }
        (*zip).header_bytes_remaining = ((*zip).header_bytes_remaining as libc::c_ulong)
            .wrapping_sub(bytes as libc::c_ulong)
            as uint64_t as uint64_t;
        p = buff as *const libc::c_uchar
    }
    /* Update checksum */
    (*zip).header_crc32 = crc32((*zip).header_crc32, p, rbytes as libc::c_uint);
    return p;
}
unsafe extern "C" fn slurp_central_directory(
    mut a: *mut archive_read,
    mut zip: *mut _7zip,
    mut header: *mut _7z_header_info,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut next_header_offset: uint64_t = 0;
    let mut next_header_size: uint64_t = 0;
    let mut next_header_crc: uint32_t = 0;
    let mut bytes_avail: ssize_t = 0;
    let mut check_header_crc: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    p = __archive_read_ahead(a, 32 as libc::c_int as size_t, &mut bytes_avail)
        as *const libc::c_uchar;
    if p.is_null() {
        return -(30 as libc::c_int);
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
        || memcmp(
            p as *const libc::c_void,
            b"\x7fELF\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        /* This is an executable ? Must be self-extracting... */
        r = skip_sfx(a, bytes_avail);
        if r < ARCHIVE_WARN {
            return r;
        }
        p = __archive_read_ahead(a, 32 as libc::c_int as size_t, &mut bytes_avail)
            as *const libc::c_uchar;
        if p.is_null() {
            return -(30 as libc::c_int);
        }
    }
    (*zip).seek_base = ((*zip).seek_base as libc::c_ulong)
        .wrapping_add(32 as libc::c_int as libc::c_ulong) as uint64_t
        as uint64_t;
    if memcmp(
        p as *const libc::c_void,
        _7ZIP_SIGNATURE.as_ptr(),
        6 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Not 7-Zip archive file\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* CRC check. */
    if crc32(
        0 as libc::c_int as uLong,
        p.offset(12 as libc::c_int as isize),
        20 as libc::c_int as uInt,
    ) != archive_le32dec(p.offset(8 as libc::c_int as isize) as *const libc::c_void)
        as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Header CRC error\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    next_header_offset =
        archive_le64dec(p.offset(12 as libc::c_int as isize) as *const libc::c_void);
    next_header_size = archive_le64dec(p.offset(20 as libc::c_int as isize) as *const libc::c_void);
    next_header_crc = archive_le32dec(p.offset(28 as libc::c_int as isize) as *const libc::c_void);
    if next_header_size == 0 as libc::c_int as libc::c_ulong {
        /* There is no entry in an archive file. */
        return 1 as libc::c_int;
    }
    if (next_header_offset as int64_t) < 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Malformed 7-Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    __archive_read_consume(a, 32 as libc::c_int as int64_t);
    if next_header_offset != 0 as libc::c_int as libc::c_ulong {
        if bytes_avail >= next_header_offset as ssize_t {
            __archive_read_consume(a, next_header_offset as int64_t);
        } else if __archive_read_seek(
            a,
            next_header_offset.wrapping_add((*zip).seek_base) as int64_t,
            SEEK_SET,
        ) < 0 as libc::c_int as libc::c_long
        {
            return -(30 as libc::c_int);
        }
    }
    (*zip).stream_offset = next_header_offset as int64_t;
    (*zip).header_offset = next_header_offset;
    (*zip).header_bytes_remaining = next_header_size;
    (*zip).header_crc32 = 0 as libc::c_int as libc::c_ulong;
    (*zip).header_is_encoded = 0 as libc::c_int;
    (*zip).header_is_being_read = 1 as libc::c_int;
    (*zip).has_encrypted_entries = 0 as libc::c_int;
    check_header_crc = 1 as libc::c_int;
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated 7-Zip file body\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Parse ArchiveProperties. */
    match *p.offset(0 as libc::c_int as isize) as libc::c_int {
        kEncodedHeader => {
            /*
             * The archive has an encoded header and we have to decode it
             * in order to parse the header correctly.
             */
            r = decode_encoded_header_info(a, &mut (*zip).si);
            /* Check the EncodedHeader CRC.*/
            if r == 0 as libc::c_int && (*zip).header_crc32 != next_header_crc as libc::c_ulong {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    -(1 as libc::c_int),
                    b"Damaged 7-Zip archive\x00" as *const u8 as *const libc::c_char,
                );
                r = -(1 as libc::c_int)
            }
            if r == 0 as libc::c_int {
                if (*(*zip).si.ci.folders.offset(0 as libc::c_int as isize)).digest_defined != 0 {
                    next_header_crc =
                        (*(*zip).si.ci.folders.offset(0 as libc::c_int as isize)).digest
                } else {
                    check_header_crc = 0 as libc::c_int
                }
                if (*zip).pack_stream_bytes_unconsumed != 0 {
                    read_consume(a);
                }
                r = setup_decode_folder(a, (*zip).si.ci.folders, 1 as libc::c_int);
                if r == 0 as libc::c_int {
                    (*zip).header_bytes_remaining = (*zip).folder_outbytes_remaining;
                    r = seek_pack(a)
                }
            }
            /* Clean up StreamsInfo. */
            free_StreamsInfo(&mut (*zip).si);
            memset(
                &mut (*zip).si as *mut _7z_stream_info as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<_7z_stream_info>() as libc::c_ulong,
            );
            if r < 0 as libc::c_int {
                return -(30 as libc::c_int);
            }
            (*zip).header_is_encoded = 1 as libc::c_int;
            (*zip).header_crc32 = 0 as libc::c_int as libc::c_ulong
        }
        kHeader => {}
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                -(1 as libc::c_int),
                b"Unexpected Property ID = %X\x00" as *const u8 as *const libc::c_char,
                *p.offset(0 as libc::c_int as isize) as libc::c_int,
            );
            return -(30 as libc::c_int);
        }
    }
    /* FALL THROUGH */
    /*
     * Parse the header.
     */
    errno = 0 as libc::c_int;
    r = read_Header(a, header, (*zip).header_is_encoded);
    if r < 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                -(1 as libc::c_int),
                b"Couldn\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                -(1 as libc::c_int),
                b"Damaged 7-Zip archive\x00" as *const u8 as *const libc::c_char,
            );
        }
        return -(30 as libc::c_int);
    }
    p = header_bytes(a, 1 as libc::c_int as size_t);
    if p.is_null() || *p as libc::c_int != kEnd {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Malformed 7-Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if check_header_crc != 0 && (*zip).header_crc32 != next_header_crc as libc::c_ulong {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            -(1 as libc::c_int),
            b"Malformed 7-Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     *  Must be kEnd.
     */
    /* Check the Header CRC.*/
    /* Clean up variables be used for decoding the archive header */
    (*zip).pack_stream_remaining = 0 as libc::c_int as libc::c_uint;
    (*zip).pack_stream_index = 0 as libc::c_int as libc::c_uint;
    (*zip).folder_outbytes_remaining = 0 as libc::c_int as uint64_t;
    (*zip).uncompressed_buffer_bytes_remaining = 0 as libc::c_int as size_t;
    (*zip).pack_stream_bytes_unconsumed = 0 as libc::c_int as size_t;
    (*zip).header_is_being_read = 0 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_uncompressed_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: size_t,
    mut minimum: size_t,
) -> ssize_t {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut bytes_avail: ssize_t = 0;
    if (*zip).codec == _7Z_COPY as libc::c_ulong
        && (*zip).codec2 == -(1 as libc::c_int) as libc::c_ulong
    {
        /* Copy mode. */
        *buff = __archive_read_ahead(a, minimum, &mut bytes_avail);
        if bytes_avail <= 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated 7-Zip file data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        if bytes_avail as size_t > (*zip).uncompressed_buffer_bytes_remaining {
            bytes_avail = (*zip).uncompressed_buffer_bytes_remaining as ssize_t
        }
        if bytes_avail as size_t > size {
            bytes_avail = size as ssize_t
        }
        (*zip).pack_stream_bytes_unconsumed = bytes_avail as size_t
    } else if (*zip).uncompressed_buffer_pointer.is_null() {
        /* Decompression has failed. */
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Damaged 7-Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int) as ssize_t;
    } else {
        /* Packed mode. */
        if minimum > (*zip).uncompressed_buffer_bytes_remaining {
            /*
             * If remaining uncompressed data size is less than
             * the minimum size, fill the buffer up to the
             * minimum size.
             */
            if extract_pack_stream(a, minimum) < 0 as libc::c_int as libc::c_long {
                return -(30 as libc::c_int) as ssize_t;
            }
        }
        if size > (*zip).uncompressed_buffer_bytes_remaining {
            bytes_avail = (*zip).uncompressed_buffer_bytes_remaining as ssize_t
        } else {
            bytes_avail = size as ssize_t
        }
        *buff = (*zip).uncompressed_buffer_pointer as *const libc::c_void;
        (*zip).uncompressed_buffer_pointer = (*zip)
            .uncompressed_buffer_pointer
            .offset(bytes_avail as isize)
    }
    (*zip).uncompressed_buffer_bytes_remaining =
        ((*zip).uncompressed_buffer_bytes_remaining as libc::c_ulong)
            .wrapping_sub(bytes_avail as libc::c_ulong) as size_t as size_t;
    return bytes_avail;
}
unsafe extern "C" fn extract_pack_stream(mut a: *mut archive_read, mut minimum: size_t) -> ssize_t {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut bytes_avail: ssize_t = 0;
    let mut r: libc::c_int = 0;
    if (*zip).codec == _7Z_COPY as libc::c_ulong
        && (*zip).codec2 == -(1 as libc::c_int) as libc::c_ulong
    {
        if minimum == 0 as libc::c_int as libc::c_ulong {
            minimum = 1 as libc::c_int as size_t
        }
        if __archive_read_ahead(a, minimum, &mut bytes_avail) == NULL as *const libc::c_void
            || bytes_avail <= 0 as libc::c_int as libc::c_long
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated 7-Zip file body\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        if bytes_avail > (*zip).pack_stream_inbytes_remaining as ssize_t {
            bytes_avail = (*zip).pack_stream_inbytes_remaining as ssize_t
        }
        (*zip).pack_stream_inbytes_remaining =
            ((*zip).pack_stream_inbytes_remaining as libc::c_ulong)
                .wrapping_sub(bytes_avail as libc::c_ulong) as uint64_t as uint64_t;
        if bytes_avail > (*zip).folder_outbytes_remaining as ssize_t {
            bytes_avail = (*zip).folder_outbytes_remaining as ssize_t
        }
        (*zip).folder_outbytes_remaining = ((*zip).folder_outbytes_remaining as libc::c_ulong)
            .wrapping_sub(bytes_avail as libc::c_ulong)
            as uint64_t as uint64_t;
        (*zip).uncompressed_buffer_bytes_remaining = bytes_avail as size_t;
        return 0 as libc::c_int as ssize_t;
    }
    /* If the buffer hasn't been allocated, allocate it now. */
    if (*zip).uncompressed_buffer.is_null() {
        (*zip).uncompressed_buffer_size = UBUFF_SIZE as size_t;
        if (*zip).uncompressed_buffer_size < minimum {
            (*zip).uncompressed_buffer_size =
                minimum.wrapping_add(1023 as libc::c_int as libc::c_ulong);
            (*zip).uncompressed_buffer_size &= !(0x3ff as libc::c_int) as libc::c_ulong
        }
        (*zip).uncompressed_buffer = malloc((*zip).uncompressed_buffer_size) as *mut libc::c_uchar;
        if (*zip).uncompressed_buffer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory for 7-Zip decompression\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        (*zip).uncompressed_buffer_bytes_remaining = 0 as libc::c_int as size_t
    } else if (*zip).uncompressed_buffer_size < minimum
        || (*zip).uncompressed_buffer_bytes_remaining < minimum
    {
        /*
         * Make sure the uncompressed buffer can have bytes
         * at least `minimum' bytes.
         * NOTE: This case happen when reading the header.
         */
        let mut used: size_t = 0;
        if !(*zip).uncompressed_buffer_pointer.is_null() {
            used = (*zip)
                .uncompressed_buffer_pointer
                .offset_from((*zip).uncompressed_buffer) as libc::c_long
                as size_t
        } else {
            used = 0 as libc::c_int as size_t
        }
        if (*zip).uncompressed_buffer_size < minimum {
            /*
             * Expand the uncompressed buffer up to
             * the minimum size.
             */
            let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut new_size: size_t = 0;
            new_size = minimum.wrapping_add(1023 as libc::c_int as libc::c_ulong);
            new_size &= !(0x3ff as libc::c_int) as libc::c_ulong;
            p = realloc((*zip).uncompressed_buffer as *mut libc::c_void, new_size);
            if p.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory for 7-Zip decompression\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int) as ssize_t;
            }
            (*zip).uncompressed_buffer = p as *mut libc::c_uchar;
            (*zip).uncompressed_buffer_size = new_size
        }
        /*
         * Move unconsumed bytes to the head.
         */
        if used != 0 {
            memmove(
                (*zip).uncompressed_buffer as *mut libc::c_void,
                (*zip).uncompressed_buffer.offset(used as isize) as *const libc::c_void,
                (*zip).uncompressed_buffer_bytes_remaining,
            );
        }
    } else {
        (*zip).uncompressed_buffer_bytes_remaining = 0 as libc::c_int as size_t
    }
    (*zip).uncompressed_buffer_pointer = NULL as *mut libc::c_uchar;
    loop {
        let mut bytes_in: size_t = 0;
        let mut bytes_out: size_t = 0;
        let mut buff_in: *const libc::c_void = 0 as *const libc::c_void;
        let mut buff_out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut end_of_data: libc::c_int = 0;
        /*
         * Note: '1' here is a performance optimization.
         * Recall that the decompression layer returns a count of
         * available bytes; asking for more than that forces the
         * decompressor to combine reads by copying data.
         */
        buff_in = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail);
        if bytes_avail <= 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated 7-Zip file body\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        buff_out = (*zip)
            .uncompressed_buffer
            .offset((*zip).uncompressed_buffer_bytes_remaining as isize);
        bytes_out = (*zip)
            .uncompressed_buffer_size
            .wrapping_sub((*zip).uncompressed_buffer_bytes_remaining);
        bytes_in = bytes_avail as size_t;
        if bytes_in > (*zip).pack_stream_inbytes_remaining {
            bytes_in = (*zip).pack_stream_inbytes_remaining
        }
        /* Drive decompression. */
        r = decompress(
            a,
            zip,
            buff_out as *mut libc::c_void,
            &mut bytes_out,
            buff_in,
            &mut bytes_in,
        );
        match r {
            ARCHIVE_OK => end_of_data = 0 as libc::c_int,
            ARCHIVE_EOF => end_of_data = 1 as libc::c_int,
            _ => return -(30 as libc::c_int) as ssize_t,
        }
        (*zip).pack_stream_inbytes_remaining =
            ((*zip).pack_stream_inbytes_remaining as libc::c_ulong).wrapping_sub(bytes_in)
                as uint64_t as uint64_t;
        if bytes_out > (*zip).folder_outbytes_remaining {
            bytes_out = (*zip).folder_outbytes_remaining
        }
        (*zip).folder_outbytes_remaining = ((*zip).folder_outbytes_remaining as libc::c_ulong)
            .wrapping_sub(bytes_out) as uint64_t
            as uint64_t;
        (*zip).uncompressed_buffer_bytes_remaining =
            ((*zip).uncompressed_buffer_bytes_remaining as libc::c_ulong).wrapping_add(bytes_out)
                as size_t as size_t;
        (*zip).pack_stream_bytes_unconsumed = bytes_in;
        /*
         * Continue decompression until uncompressed_buffer is full.
         */
        if (*zip).uncompressed_buffer_bytes_remaining == (*zip).uncompressed_buffer_size {
            break;
        }
        if (*zip).codec2 == _7Z_X86 as libc::c_ulong
            && (*zip).odd_bcj_size != 0
            && (*zip)
                .uncompressed_buffer_bytes_remaining
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                > (*zip).uncompressed_buffer_size
        {
            break;
        }
        if (*zip).pack_stream_inbytes_remaining == 0 as libc::c_int as libc::c_ulong
            && (*zip).folder_outbytes_remaining == 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        if end_of_data != 0
            || bytes_in == 0 as libc::c_int as libc::c_ulong
                && bytes_out == 0 as libc::c_int as libc::c_ulong
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Damaged 7-Zip archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        read_consume(a);
    }
    if (*zip).uncompressed_buffer_bytes_remaining < minimum {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Damaged 7-Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int) as ssize_t;
    }
    (*zip).uncompressed_buffer_pointer = (*zip).uncompressed_buffer;
    return 0 as libc::c_int as ssize_t;
}
unsafe extern "C" fn seek_pack(mut a: *mut archive_read) -> libc::c_int {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut pack_offset: int64_t = 0;
    if (*zip).pack_stream_remaining <= 0 as libc::c_int as libc::c_uint {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Damaged 7-Zip archive\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*zip).pack_stream_inbytes_remaining =
        *(*zip).si.pi.sizes.offset((*zip).pack_stream_index as isize);
    pack_offset = *(*zip)
        .si
        .pi
        .positions
        .offset((*zip).pack_stream_index as isize) as int64_t;
    if (*zip).stream_offset != pack_offset {
        if 0 as libc::c_int as libc::c_long
            > __archive_read_seek(
                a,
                (pack_offset as libc::c_ulong).wrapping_add((*zip).seek_base) as int64_t,
                SEEK_SET,
            )
        {
            return -(30 as libc::c_int);
        }
        (*zip).stream_offset = pack_offset
    }
    (*zip).pack_stream_index = (*zip).pack_stream_index.wrapping_add(1);
    (*zip).pack_stream_remaining = (*zip).pack_stream_remaining.wrapping_sub(1);
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_stream(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: size_t,
    mut minimum: size_t,
) -> ssize_t {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut skip_bytes: uint64_t = 0 as libc::c_int as uint64_t;
    let mut r: ssize_t = 0;
    if (*zip).uncompressed_buffer_bytes_remaining == 0 as libc::c_int as libc::c_ulong {
        if (*zip).pack_stream_inbytes_remaining > 0 as libc::c_int as libc::c_ulong {
            r = extract_pack_stream(a, 0 as libc::c_int as size_t);
            if r < 0 as libc::c_int as libc::c_long {
                return r;
            }
            return get_uncompressed_data(a, buff, size, minimum);
        } else {
            if (*zip).folder_outbytes_remaining > 0 as libc::c_int as libc::c_ulong {
                /* Extract a remaining pack stream. */
                r = extract_pack_stream(a, 0 as libc::c_int as size_t);
                if r < 0 as libc::c_int as libc::c_long {
                    return r;
                }
                return get_uncompressed_data(a, buff, size, minimum);
            }
        }
    } else {
        return get_uncompressed_data(a, buff, size, minimum);
    }
    /*
     * Current pack stream has been consumed.
     */
    if (*zip).pack_stream_remaining == 0 as libc::c_int as libc::c_uint {
        if (*zip).header_is_being_read != 0 {
            /* Invalid sequence. This might happen when
             * reading a malformed archive. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Malformed 7-Zip archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as ssize_t;
        }
        /*
         * All current folder's pack streams have been
         * consumed. Switch to next folder.
         */
        if (*zip).folder_index == 0 as libc::c_int as libc::c_uint
            && ((*(*zip)
                .si
                .ci
                .folders
                .offset((*(*zip).entry).folderIndex as isize))
            .skipped_bytes
                != 0
                || (*zip).folder_index != (*(*zip).entry).folderIndex)
        {
            (*zip).folder_index = (*(*zip).entry).folderIndex;
            skip_bytes = (*(*zip).si.ci.folders.offset((*zip).folder_index as isize)).skipped_bytes
        }
        if (*zip).folder_index as libc::c_ulong >= (*zip).si.ci.numFolders {
            /*
             * We have consumed all folders and its pack streams.
             */
            *buff = NULL as *const libc::c_void;
            return 0 as libc::c_int as ssize_t;
        }
        r = setup_decode_folder(
            a,
            &mut *(*zip).si.ci.folders.offset((*zip).folder_index as isize),
            0 as libc::c_int,
        ) as ssize_t;
        if r != ARCHIVE_OK as libc::c_long {
            return -(30 as libc::c_int) as ssize_t;
        }
        (*zip).folder_index = (*zip).folder_index.wrapping_add(1)
    }
    /*
     * Switch to next pack stream.
     */
    r = seek_pack(a) as ssize_t;
    if r < 0 as libc::c_int as libc::c_long {
        return r;
    }
    /* Extract a new pack stream. */
    r = extract_pack_stream(a, 0 as libc::c_int as size_t);
    if r < 0 as libc::c_int as libc::c_long {
        return r;
    }
    /*
     * Skip the bytes we already has skipped in skip_stream().
     */
    while skip_bytes != 0 {
        let mut skipped: ssize_t = 0;
        if (*zip).uncompressed_buffer_bytes_remaining == 0 as libc::c_int as libc::c_ulong {
            if (*zip).pack_stream_inbytes_remaining > 0 as libc::c_int as libc::c_ulong {
                r = extract_pack_stream(a, 0 as libc::c_int as size_t);
                if r < 0 as libc::c_int as libc::c_long {
                    return r;
                }
            } else if (*zip).folder_outbytes_remaining > 0 as libc::c_int as libc::c_ulong {
                /* Extract a remaining pack stream. */
                r = extract_pack_stream(a, 0 as libc::c_int as size_t);
                if r < 0 as libc::c_int as libc::c_long {
                    return r;
                }
            } else {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Truncated 7-Zip file body\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int) as ssize_t;
            }
        }
        skipped = get_uncompressed_data(a, buff, skip_bytes, 0 as libc::c_int as size_t);
        if skipped < 0 as libc::c_int as libc::c_long {
            return skipped;
        }
        skip_bytes = (skip_bytes as libc::c_ulong).wrapping_sub(skipped as libc::c_ulong)
            as uint64_t as uint64_t;
        if (*zip).pack_stream_bytes_unconsumed != 0 {
            read_consume(a);
        }
    }
    return get_uncompressed_data(a, buff, size, minimum);
}
unsafe extern "C" fn setup_decode_folder(
    mut a: *mut archive_read,
    mut folder: *mut _7z_folder,
    mut header: libc::c_int,
) -> libc::c_int {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut coder1: *const _7z_coder = 0 as *const _7z_coder;
    let mut coder2: *const _7z_coder = 0 as *const _7z_coder;
    let mut cname: *const libc::c_char = if header != 0 {
        b"archive header\x00" as *const u8 as *const libc::c_char
    } else {
        b"file content\x00" as *const u8 as *const libc::c_char
    };
    let mut i: libc::c_uint = 0;
    let mut r: libc::c_int = 0;
    let mut found_bcj2: libc::c_int = 0 as libc::c_int;
    /*
     * Release the memory which the previous folder used for BCJ2.
     */
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        free((*zip).sub_stream_buff[i as usize] as *mut libc::c_void);
        (*zip).sub_stream_buff[i as usize] = NULL as *mut libc::c_uchar;
        i = i.wrapping_add(1)
    }
    /*
     * Initialize a stream reader.
     */
    (*zip).pack_stream_remaining = (*folder).numPackedStreams as libc::c_uint;
    (*zip).pack_stream_index = (*folder).packIndex;
    (*zip).folder_outbytes_remaining = folder_uncompressed_size(folder);
    (*zip).uncompressed_buffer_bytes_remaining = 0 as libc::c_int as size_t;
    /*
     * Check coder types.
     */
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*folder).numCoders {
        match (*(*folder).coders.offset(i as isize)).codec {
            116457729 | 116458243 | 116459265 => {
                /* For entry that is associated with this folder, mark
                it as encrypted (data+metadata). */
                (*zip).has_encrypted_entries = 1 as libc::c_int;
                if !(*a).entry.is_null() {
                    archive_entry_set_is_data_encrypted(
                        (*a).entry,
                        1 as libc::c_int as libc::c_char,
                    );
                    archive_entry_set_is_metadata_encrypted(
                        (*a).entry,
                        1 as libc::c_int as libc::c_char,
                    );
                }
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"The %s is encrypted, but currently not supported\x00" as *const u8
                        as *const libc::c_char,
                    cname,
                );
                return -(30 as libc::c_int);
            }
            50528539 => found_bcj2 += 1,
            _ => {}
        }
        i = i.wrapping_add(1)
    }
    /* Now that we've checked for encryption, if there were still no
     * encrypted entries found we can say for sure that there are none.
     */
    if (*zip).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*zip).has_encrypted_entries = 0 as libc::c_int
    }
    if (*folder).numCoders > 2 as libc::c_int as libc::c_ulong && found_bcj2 == 0
        || found_bcj2 > 1 as libc::c_int
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"The %s is encoded with many filters, but currently not supported\x00" as *const u8
                as *const libc::c_char,
            cname,
        );
        return -(30 as libc::c_int);
    }
    coder1 = &mut *(*folder).coders.offset(0 as libc::c_int as isize) as *mut _7z_coder;
    if (*folder).numCoders == 2 as libc::c_int as libc::c_ulong {
        coder2 = &mut *(*folder).coders.offset(1 as libc::c_int as isize) as *mut _7z_coder
    } else {
        coder2 = NULL as *const _7z_coder
    }
    if found_bcj2 != 0 {
        /*
         * Preparation to decode BCJ2.
         * Decoding BCJ2 requires four sources. Those are at least,
         * as far as I know, two types of the storage form.
         */
        let mut fc: *const _7z_coder = (*folder).coders;
        static mut coder_copy: _7z_coder = {
            let mut init = _7z_coder {
                codec: 0 as libc::c_int as libc::c_ulong,
                numInStreams: 1 as libc::c_int as uint64_t,
                numOutStreams: 1 as libc::c_int as uint64_t,
                propertiesSize: 0 as libc::c_int as uint64_t,
                properties: NULL as *mut libc::c_uchar,
            };
            init
        };
        let mut scoder: [*const _7z_coder; 3] = [&coder_copy, &coder_copy, &coder_copy];
        let mut buff: *const libc::c_void = 0 as *const libc::c_void;
        let mut bytes: ssize_t = 0;
        let mut b: [*mut libc::c_uchar; 3] = [
            NULL as *mut libc::c_uchar,
            NULL as *mut libc::c_uchar,
            NULL as *mut libc::c_uchar,
        ];
        let mut sunpack: [uint64_t; 3] = [
            -(1 as libc::c_int) as uint64_t,
            -(1 as libc::c_int) as uint64_t,
            -(1 as libc::c_int) as uint64_t,
        ];
        let mut s: [size_t; 3] = [
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        ];
        let mut idx: [libc::c_int; 3] = [0 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int];
        if (*folder).numCoders == 4 as libc::c_int as libc::c_ulong
            && (*fc.offset(3 as libc::c_int as isize)).codec == _7Z_X86_BCJ2 as libc::c_ulong
            && (*folder).numInStreams == 7 as libc::c_int as libc::c_ulong
            && (*folder).numOutStreams == 4 as libc::c_int as libc::c_ulong
            && (*zip).pack_stream_remaining == 4 as libc::c_int as libc::c_uint
        {
            /* Source type 1 made by 7zr or 7z with -m options. */
            if (*(*folder).bindPairs.offset(0 as libc::c_int as isize)).inIndex
                == 5 as libc::c_int as libc::c_ulong
            {
                /* The form made by 7zr */
                idx[0 as libc::c_int as usize] = 1 as libc::c_int;
                idx[1 as libc::c_int as usize] = 2 as libc::c_int;
                idx[2 as libc::c_int as usize] = 0 as libc::c_int;
                scoder[1 as libc::c_int as usize] =
                    &*fc.offset(1 as libc::c_int as isize) as *const _7z_coder;
                scoder[2 as libc::c_int as usize] =
                    &*fc.offset(0 as libc::c_int as isize) as *const _7z_coder;
                sunpack[1 as libc::c_int as usize] =
                    *(*folder).unPackSize.offset(1 as libc::c_int as isize);
                sunpack[2 as libc::c_int as usize] =
                    *(*folder).unPackSize.offset(0 as libc::c_int as isize);
                coder1 = &*fc.offset(2 as libc::c_int as isize) as *const _7z_coder
            } else if (*fc.offset(0 as libc::c_int as isize)).codec == _7Z_COPY as libc::c_ulong
                && (*fc.offset(1 as libc::c_int as isize)).codec == _7Z_COPY as libc::c_ulong
            {
                coder1 = &mut *(*folder).coders.offset(2 as libc::c_int as isize) as *mut _7z_coder
            } else if (*fc.offset(0 as libc::c_int as isize)).codec == _7Z_COPY as libc::c_ulong
                && (*fc.offset(2 as libc::c_int as isize)).codec == _7Z_COPY as libc::c_ulong
            {
                coder1 = &mut *(*folder).coders.offset(1 as libc::c_int as isize) as *mut _7z_coder
            } else if (*fc.offset(1 as libc::c_int as isize)).codec == _7Z_COPY as libc::c_ulong
                && (*fc.offset(2 as libc::c_int as isize)).codec == _7Z_COPY as libc::c_ulong
            {
                coder1 = &mut *(*folder).coders.offset(0 as libc::c_int as isize) as *mut _7z_coder
            } else {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Unsupported form of BCJ2 streams\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            coder2 = &*fc.offset(3 as libc::c_int as isize) as *const _7z_coder;
            (*zip).main_stream_bytes_remaining =
                *(*folder).unPackSize.offset(2 as libc::c_int as isize)
        } else if !coder2.is_null()
            && (*coder2).codec == _7Z_X86_BCJ2 as libc::c_ulong
            && (*zip).pack_stream_remaining == 4 as libc::c_int as libc::c_uint
            && (*folder).numInStreams == 5 as libc::c_int as libc::c_ulong
            && (*folder).numOutStreams == 2 as libc::c_int as libc::c_ulong
        {
            /*
             * NOTE: Some patterns do not work.
             * work:
             *  7z a -m0=BCJ2 -m1=COPY -m2=COPY
             *       -m3=(any)
             *  7z a -m0=BCJ2 -m1=COPY -m2=(any)
             *       -m3=COPY
             *  7z a -m0=BCJ2 -m1=(any) -m2=COPY
             *       -m3=COPY
             * not work:
             *  other patterns.
             *
             * We have to handle this like `pipe' or
             * our libarchive7s filter frame work,
             * decoding the BCJ2 main stream sequentially,
             * m3 -> m2 -> m1 -> BCJ2.
             *
             */
            /* Source type 0 made by 7z */
            (*zip).main_stream_bytes_remaining =
                *(*folder).unPackSize.offset(0 as libc::c_int as isize)
        } else {
            /* We got an unexpected form. */
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unsupported form of BCJ2 streams\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Skip the main stream at this time. */
        r = seek_pack(a);
        if r < 0 as libc::c_int {
            return r;
        }
        (*zip).pack_stream_bytes_unconsumed = (*zip).pack_stream_inbytes_remaining;
        read_consume(a);
        /* Read following three sub streams. */
        i = 0 as libc::c_int as libc::c_uint;
        while i < 3 as libc::c_int as libc::c_uint {
            let mut coder: *const _7z_coder = scoder[i as usize];
            r = seek_pack(a);
            if r < 0 as libc::c_int {
                free(b[0 as libc::c_int as usize] as *mut libc::c_void);
                free(b[1 as libc::c_int as usize] as *mut libc::c_void);
                free(b[2 as libc::c_int as usize] as *mut libc::c_void);
                return r;
            }
            if sunpack[i as usize] == -(1 as libc::c_int) as uint64_t {
                (*zip).folder_outbytes_remaining = (*zip).pack_stream_inbytes_remaining
            } else {
                (*zip).folder_outbytes_remaining = sunpack[i as usize]
            }
            r = init_decompression(a, zip, coder, NULL as *const _7z_coder);
            if r != ARCHIVE_OK {
                free(b[0 as libc::c_int as usize] as *mut libc::c_void);
                free(b[1 as libc::c_int as usize] as *mut libc::c_void);
                free(b[2 as libc::c_int as usize] as *mut libc::c_void);
                return -(30 as libc::c_int);
            }
            /* Allocate memory for the decoded data of a sub
             * stream. */
            b[i as usize] = malloc((*zip).folder_outbytes_remaining) as *mut libc::c_uchar;
            if b[i as usize].is_null() {
                free(b[0 as libc::c_int as usize] as *mut libc::c_void);
                free(b[1 as libc::c_int as usize] as *mut libc::c_void);
                free(b[2 as libc::c_int as usize] as *mut libc::c_void);
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory for 7-Zip decompression\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            /* Extract a sub stream. */
            while (*zip).pack_stream_inbytes_remaining > 0 as libc::c_int as libc::c_ulong {
                r = extract_pack_stream(a, 0 as libc::c_int as size_t) as libc::c_int;
                if r < 0 as libc::c_int {
                    free(b[0 as libc::c_int as usize] as *mut libc::c_void);
                    free(b[1 as libc::c_int as usize] as *mut libc::c_void);
                    free(b[2 as libc::c_int as usize] as *mut libc::c_void);
                    return r;
                }
                bytes = get_uncompressed_data(
                    a,
                    &mut buff,
                    (*zip).uncompressed_buffer_bytes_remaining,
                    0 as libc::c_int as size_t,
                );
                if bytes < 0 as libc::c_int as libc::c_long {
                    free(b[0 as libc::c_int as usize] as *mut libc::c_void);
                    free(b[1 as libc::c_int as usize] as *mut libc::c_void);
                    free(b[2 as libc::c_int as usize] as *mut libc::c_void);
                    return bytes as libc::c_int;
                }
                memcpy(
                    b[i as usize].offset(s[i as usize] as isize) as *mut libc::c_void,
                    buff,
                    bytes as libc::c_ulong,
                );
                s[i as usize] = (s[i as usize] as libc::c_ulong)
                    .wrapping_add(bytes as libc::c_ulong) as size_t
                    as size_t;
                if (*zip).pack_stream_bytes_unconsumed != 0 {
                    read_consume(a);
                }
            }
            i = i.wrapping_add(1)
        }
        /* Set the sub streams to the right place. */
        i = 0 as libc::c_int as libc::c_uint;
        while i < 3 as libc::c_int as libc::c_uint {
            (*zip).sub_stream_buff[i as usize] = b[idx[i as usize] as usize];
            (*zip).sub_stream_size[i as usize] = s[idx[i as usize] as usize];
            (*zip).sub_stream_bytes_remaining[i as usize] = s[idx[i as usize] as usize];
            i = i.wrapping_add(1)
        }
        /* Allocate memory used for decoded main stream bytes. */
        if (*zip).tmp_stream_buff.is_null() {
            (*zip).tmp_stream_buff_size = (32 as libc::c_int * 1024 as libc::c_int) as size_t;
            (*zip).tmp_stream_buff = malloc((*zip).tmp_stream_buff_size) as *mut libc::c_uchar;
            if (*zip).tmp_stream_buff.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"No memory for 7-Zip decompression\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        (*zip).tmp_stream_bytes_avail = 0 as libc::c_int as size_t;
        (*zip).tmp_stream_bytes_remaining = 0 as libc::c_int as size_t;
        (*zip).odd_bcj_size = 0 as libc::c_int as size_t;
        (*zip).bcj2_outPos = 0 as libc::c_int as uint64_t;
        /*
         * Reset a stream reader in order to read the main stream
         * of BCJ2.
         */
        (*zip).pack_stream_remaining = 1 as libc::c_int as libc::c_uint;
        (*zip).pack_stream_index = (*folder).packIndex;
        (*zip).folder_outbytes_remaining = folder_uncompressed_size(folder);
        (*zip).uncompressed_buffer_bytes_remaining = 0 as libc::c_int as size_t
    }
    /*
     * Initialize the decompressor for the new folder's pack streams.
     */
    r = init_decompression(a, zip, coder1, coder2);
    if r != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn skip_stream(mut a: *mut archive_read, mut skip_bytes: size_t) -> int64_t {
    let mut zip: *mut _7zip = (*(*a).format).data as *mut _7zip;
    let mut p: *const libc::c_void = 0 as *const libc::c_void;
    let mut skipped_bytes: int64_t = 0;
    let mut bytes: size_t = skip_bytes;
    if (*zip).folder_index == 0 as libc::c_int as libc::c_uint {
        /*
         * Optimization for a list mode.
         * Avoid unnecessary decoding operations.
         */
        let ref mut fresh15 = (*(*zip)
            .si
            .ci
            .folders
            .offset((*(*zip).entry).folderIndex as isize))
        .skipped_bytes;
        *fresh15 = (*fresh15 as libc::c_ulong).wrapping_add(skip_bytes) as uint64_t as uint64_t;
        return skip_bytes as int64_t;
    }
    while bytes != 0 {
        skipped_bytes = read_stream(a, &mut p, bytes, 0 as libc::c_int as size_t);
        if skipped_bytes < 0 as libc::c_int as libc::c_long {
            return skipped_bytes;
        }
        if skipped_bytes == 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated 7-Zip file body\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as int64_t;
        }
        bytes = (bytes as libc::c_ulong).wrapping_sub(skipped_bytes as size_t) as size_t as size_t;
        if (*zip).pack_stream_bytes_unconsumed != 0 {
            read_consume(a);
        }
    }
    return skip_bytes as int64_t;
}
unsafe extern "C" fn x86_Init(mut zip: *mut _7zip) {
    (*zip).bcj_state = 0 as libc::c_int as uint32_t;
    (*zip).bcj_prevPosT =
        (0 as libc::c_int as size_t).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*zip).bcj_prevMask = 0 as libc::c_int as uint32_t;
    (*zip).bcj_ip = 5 as libc::c_int as uint32_t;
}
unsafe extern "C" fn x86_Convert(
    mut zip: *mut _7zip,
    mut data: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    static mut kMaskToAllowedStatus: [uint8_t; 8] = [
        1 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    static mut kMaskToBitNumber: [uint8_t; 8] = [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
    ];
    let mut bufferPos: size_t = 0;
    let mut prevPosT: size_t = 0;
    let mut ip: uint32_t = 0;
    let mut prevMask: uint32_t = 0;
    if size < 5 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    bufferPos = 0 as libc::c_int as size_t;
    prevPosT = (*zip).bcj_prevPosT;
    prevMask = (*zip).bcj_prevMask;
    ip = (*zip).bcj_ip;
    loop {
        let mut p: *mut uint8_t = data.offset(bufferPos as isize);
        let mut limit: *mut uint8_t = data
            .offset(size as isize)
            .offset(-(4 as libc::c_int as isize));
        while p < limit {
            if *p as libc::c_int & 0xfe as libc::c_int == 0xe8 as libc::c_int {
                break;
            }
            p = p.offset(1)
        }
        bufferPos = p.offset_from(data) as libc::c_long as size_t;
        if p >= limit {
            break;
        }
        prevPosT = bufferPos.wrapping_sub(prevPosT);
        if prevPosT > 3 as libc::c_int as libc::c_ulong {
            prevMask = 0 as libc::c_int as uint32_t
        } else {
            prevMask = prevMask << prevPosT as libc::c_int - 1 as libc::c_int
                & 0x7 as libc::c_int as libc::c_uint;
            if prevMask != 0 as libc::c_int as libc::c_uint {
                let mut b: libc::c_uchar = *p.offset(
                    (4 as libc::c_int - kMaskToBitNumber[prevMask as usize] as libc::c_int)
                        as isize,
                );
                if kMaskToAllowedStatus[prevMask as usize] == 0
                    || (b as libc::c_int == 0 as libc::c_int
                        || b as libc::c_int == 0xff as libc::c_int)
                {
                    prevPosT = bufferPos;
                    prevMask = prevMask << 1 as libc::c_int & 0x7 as libc::c_int as libc::c_uint
                        | 1 as libc::c_int as libc::c_uint;
                    bufferPos = bufferPos.wrapping_add(1);
                    continue;
                }
            }
        }
        prevPosT = bufferPos;
        if *p.offset(4 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            || *p.offset(4 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int
        {
            let mut src: uint32_t = (*p.offset(4 as libc::c_int as isize) as uint32_t)
                << 24 as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                | (*p.offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as uint32_t;
            let mut dest: uint32_t = 0;
            loop {
                let mut b_0: uint8_t = 0;
                let mut b_index: libc::c_int = 0;
                dest = src.wrapping_sub(ip.wrapping_add(bufferPos as uint32_t));
                if prevMask == 0 as libc::c_int as libc::c_uint {
                    break;
                }
                b_index = kMaskToBitNumber[prevMask as usize] as libc::c_int * 8 as libc::c_int;
                b_0 = (dest >> 24 as libc::c_int - b_index) as uint8_t;
                if !(b_0 as libc::c_int == 0 as libc::c_int
                    || b_0 as libc::c_int == 0xff as libc::c_int)
                {
                    break;
                }
                src = dest
                    ^ (((1 as libc::c_int) << 32 as libc::c_int - b_index) - 1 as libc::c_int)
                        as libc::c_uint
            }
            *p.offset(4 as libc::c_int as isize) =
                !(dest >> 24 as libc::c_int & 1 as libc::c_int as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t;
            *p.offset(3 as libc::c_int as isize) = (dest >> 16 as libc::c_int) as uint8_t;
            *p.offset(2 as libc::c_int as isize) = (dest >> 8 as libc::c_int) as uint8_t;
            *p.offset(1 as libc::c_int as isize) = dest as uint8_t;
            bufferPos = (bufferPos as libc::c_ulong).wrapping_add(5 as libc::c_int as libc::c_ulong)
                as size_t as size_t
        } else {
            prevMask = prevMask << 1 as libc::c_int & 0x7 as libc::c_int as libc::c_uint
                | 1 as libc::c_int as libc::c_uint;
            bufferPos = bufferPos.wrapping_add(1)
        }
    }
    (*zip).bcj_prevPosT = prevPosT;
    (*zip).bcj_prevMask = prevMask;
    (*zip).bcj_ip =
        ((*zip).bcj_ip as libc::c_uint).wrapping_add(bufferPos as uint32_t) as uint32_t as uint32_t;
    return bufferPos;
}
/*
 * Brought from LZMA SDK.
 *
 * Bcj2.c -- Converter for x86 code (BCJ2)
 * 2008-10-04 : Igor Pavlov : Public domain
 *
 */
pub const SZ_ERROR_DATA: libc::c_int = -(25 as libc::c_int);
pub const kNumTopBits: libc::c_int = 24 as libc::c_int;
pub const kTopValue: uint32_t = (1 as libc::c_int as uint32_t) << kNumTopBits;
pub const kNumBitModelTotalBits: libc::c_int = 11 as libc::c_int;
pub const kBitModelTotal: libc::c_int = (1 as libc::c_int) << kNumBitModelTotalBits;
pub const kNumMoveBits: libc::c_int = 5 as libc::c_int;
unsafe extern "C" fn Bcj2_Decode(
    mut zip: *mut _7zip,
    mut outBuf: *mut uint8_t,
    mut outSize: size_t,
) -> ssize_t {
    let mut inPos: size_t = 0 as libc::c_int as size_t;
    let mut outPos: size_t = 0 as libc::c_int as size_t;
    let mut buf0: *const uint8_t = 0 as *const uint8_t;
    let mut buf1: *const uint8_t = 0 as *const uint8_t;
    let mut buf2: *const uint8_t = 0 as *const uint8_t;
    let mut buf3: *const uint8_t = 0 as *const uint8_t;
    let mut size0: size_t = 0;
    let mut size1: size_t = 0;
    let mut size2: size_t = 0;
    let mut size3: size_t = 0;
    let mut buffer: *const uint8_t = 0 as *const uint8_t;
    let mut bufferLim: *const uint8_t = 0 as *const uint8_t;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    size0 = (*zip).tmp_stream_bytes_remaining;
    buf0 = (*zip)
        .tmp_stream_buff
        .offset((*zip).tmp_stream_bytes_avail as isize)
        .offset(-(size0 as isize));
    size1 = (*zip).sub_stream_bytes_remaining[0 as libc::c_int as usize];
    buf1 = (*zip).sub_stream_buff[0 as libc::c_int as usize]
        .offset((*zip).sub_stream_size[0 as libc::c_int as usize] as isize)
        .offset(-(size1 as isize));
    size2 = (*zip).sub_stream_bytes_remaining[1 as libc::c_int as usize];
    buf2 = (*zip).sub_stream_buff[1 as libc::c_int as usize]
        .offset((*zip).sub_stream_size[1 as libc::c_int as usize] as isize)
        .offset(-(size2 as isize));
    size3 = (*zip).sub_stream_bytes_remaining[2 as libc::c_int as usize];
    buf3 = (*zip).sub_stream_buff[2 as libc::c_int as usize]
        .offset((*zip).sub_stream_size[2 as libc::c_int as usize] as isize)
        .offset(-(size3 as isize));
    buffer = buf3;
    bufferLim = buffer.offset(size3 as isize);
    if (*zip).bcj_state == 0 as libc::c_int as libc::c_uint {
        /*
         * Initialize.
         */
        (*zip).bcj2_prevByte = 0 as libc::c_int as uint8_t;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[uint16_t; 258]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        {
            (*zip).bcj2_p[i as usize] = (kBitModelTotal >> 1 as libc::c_int) as uint16_t;
            i = i.wrapping_add(1)
        }
        (*zip).bcj2_code = 0 as libc::c_int as uint32_t;
        (*zip).bcj2_range = 0xffffffff as libc::c_uint;
        let mut ii: libc::c_int = 0;
        ii = 0 as libc::c_int;
        while ii < 5 as libc::c_int {
            if buffer == bufferLim {
                return SZ_ERROR_DATA as ssize_t;
            }
            let fresh16 = buffer;
            buffer = buffer.offset(1);
            (*zip).bcj2_code = (*zip).bcj2_code << 8 as libc::c_int | *fresh16 as libc::c_uint;
            ii += 1
        }
        (*zip).bcj_state = 1 as libc::c_int as uint32_t
    }
    /*
     * Gather the odd bytes of a previous call.
     */
    i = 0 as libc::c_int as libc::c_uint;
    while (*zip).odd_bcj_size > 0 as libc::c_int as libc::c_ulong && outPos < outSize {
        let fresh17 = outPos;
        outPos = outPos.wrapping_add(1);
        *outBuf.offset(fresh17 as isize) = (*zip).odd_bcj[i as usize];
        (*zip).odd_bcj_size = (*zip).odd_bcj_size.wrapping_sub(1);
        i = i.wrapping_add(1)
    }
    if outSize == 0 as libc::c_int as libc::c_ulong {
        (*zip).bcj2_outPos =
            ((*zip).bcj2_outPos as libc::c_ulong).wrapping_add(outPos) as uint64_t as uint64_t;
        return outPos as ssize_t;
    }
    loop {
        let mut b: uint8_t = 0;
        let mut prob: *mut uint16_t = 0 as *mut uint16_t;
        let mut bound: uint32_t = 0;
        let mut ttt: uint32_t = 0;
        let mut limit: size_t = size0.wrapping_sub(inPos);
        if outSize.wrapping_sub(outPos) < limit {
            limit = outSize.wrapping_sub(outPos)
        }
        if (*zip).bcj_state == 1 as libc::c_int as libc::c_uint {
            while limit != 0 as libc::c_int as libc::c_ulong {
                let mut bb: uint8_t = *buf0.offset(inPos as isize);
                let fresh18 = outPos;
                outPos = outPos.wrapping_add(1);
                *outBuf.offset(fresh18 as isize) = bb;
                if bb as libc::c_int & 0xfe as libc::c_int == 0xe8 as libc::c_int
                    || (*zip).bcj2_prevByte as libc::c_int == 0xf as libc::c_int
                        && bb as libc::c_int & 0xf0 as libc::c_int == 0x80 as libc::c_int
                {
                    (*zip).bcj_state = 2 as libc::c_int as uint32_t;
                    break;
                } else {
                    inPos = inPos.wrapping_add(1);
                    (*zip).bcj2_prevByte = bb;
                    limit = limit.wrapping_sub(1)
                }
            }
        }
        if limit == 0 as libc::c_int as libc::c_ulong || outPos == outSize {
            break;
        }
        (*zip).bcj_state = 1 as libc::c_int as uint32_t;
        let fresh19 = inPos;
        inPos = inPos.wrapping_add(1);
        b = *buf0.offset(fresh19 as isize);
        if b as libc::c_int == 0xe8 as libc::c_int {
            prob = (*zip)
                .bcj2_p
                .as_mut_ptr()
                .offset((*zip).bcj2_prevByte as libc::c_int as isize)
        } else if b as libc::c_int == 0xe9 as libc::c_int {
            prob = (*zip)
                .bcj2_p
                .as_mut_ptr()
                .offset(256 as libc::c_int as isize)
        } else {
            prob = (*zip)
                .bcj2_p
                .as_mut_ptr()
                .offset(257 as libc::c_int as isize)
        }
        ttt = *prob as uint32_t;
        bound = ((*zip).bcj2_range >> kNumBitModelTotalBits).wrapping_mul(ttt);
        if (*zip).bcj2_code < bound {
            (*zip).bcj2_range = bound;
            *prob = ttt
                .wrapping_add((kBitModelTotal as libc::c_uint).wrapping_sub(ttt) >> kNumMoveBits)
                as uint16_t;
            if (*zip).bcj2_range < kTopValue {
                if buffer == bufferLim {
                    return SZ_ERROR_DATA as ssize_t;
                }
                (*zip).bcj2_range <<= 8 as libc::c_int;
                let fresh20 = buffer;
                buffer = buffer.offset(1);
                (*zip).bcj2_code = (*zip).bcj2_code << 8 as libc::c_int | *fresh20 as libc::c_uint
            }
            (*zip).bcj2_prevByte = b
        } else {
            let mut dest: uint32_t = 0;
            let mut v: *const uint8_t = 0 as *const uint8_t;
            let mut out: [uint8_t; 4] = [0; 4];
            (*zip).bcj2_range =
                ((*zip).bcj2_range as libc::c_uint).wrapping_sub(bound) as uint32_t as uint32_t;
            (*zip).bcj2_code =
                ((*zip).bcj2_code as libc::c_uint).wrapping_sub(bound) as uint32_t as uint32_t;
            *prob = ttt.wrapping_sub(ttt >> kNumMoveBits) as uint16_t;
            if (*zip).bcj2_range < kTopValue {
                if buffer == bufferLim {
                    return SZ_ERROR_DATA as ssize_t;
                }
                (*zip).bcj2_range <<= 8 as libc::c_int;
                let fresh21 = buffer;
                buffer = buffer.offset(1);
                (*zip).bcj2_code = (*zip).bcj2_code << 8 as libc::c_int | *fresh21 as libc::c_uint
            }
            if b as libc::c_int == 0xe8 as libc::c_int {
                v = buf1;
                if size1 < 4 as libc::c_int as libc::c_ulong {
                    return SZ_ERROR_DATA as ssize_t;
                }
                buf1 = buf1.offset(4 as libc::c_int as isize);
                size1 = (size1 as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            } else {
                v = buf2;
                if size2 < 4 as libc::c_int as libc::c_ulong {
                    return SZ_ERROR_DATA as ssize_t;
                }
                buf2 = buf2.offset(4 as libc::c_int as isize);
                size2 = (size2 as libc::c_ulong).wrapping_sub(4 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            }
            dest = ((*v.offset(0 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
                | (*v.offset(1 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                | (*v.offset(2 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *v.offset(3 as libc::c_int as isize) as uint32_t)
                .wrapping_sub(
                    ((*zip).bcj2_outPos as uint32_t)
                        .wrapping_add(outPos as uint32_t)
                        .wrapping_add(4 as libc::c_int as libc::c_uint),
                );
            out[0 as libc::c_int as usize] = dest as uint8_t;
            out[1 as libc::c_int as usize] = (dest >> 8 as libc::c_int) as uint8_t;
            out[2 as libc::c_int as usize] = (dest >> 16 as libc::c_int) as uint8_t;
            (*zip).bcj2_prevByte = (dest >> 24 as libc::c_int) as uint8_t;
            out[3 as libc::c_int as usize] = (*zip).bcj2_prevByte;
            i = 0 as libc::c_int as libc::c_uint;
            while i < 4 as libc::c_int as libc::c_uint && outPos < outSize {
                let fresh22 = outPos;
                outPos = outPos.wrapping_add(1);
                *outBuf.offset(fresh22 as isize) = out[i as usize];
                i = i.wrapping_add(1)
            }
            if !(i < 4 as libc::c_int as libc::c_uint) {
                continue;
            }
            /*
             * Save odd bytes which we could not add into
             * the output buffer because of out of space.
             */
            (*zip).odd_bcj_size = (4 as libc::c_int as libc::c_uint).wrapping_sub(i) as size_t;
            while i < 4 as libc::c_int as libc::c_uint {
                j = i
                    .wrapping_sub(4 as libc::c_int as libc::c_uint)
                    .wrapping_add((*zip).odd_bcj_size as libc::c_uint);
                (*zip).odd_bcj[j as usize] = out[i as usize];
                i = i.wrapping_add(1)
            }
            break;
        }
    }
    (*zip).tmp_stream_bytes_remaining = ((*zip).tmp_stream_bytes_remaining as libc::c_ulong)
        .wrapping_sub(inPos) as size_t as size_t;
    (*zip).sub_stream_bytes_remaining[0 as libc::c_int as usize] = size1;
    (*zip).sub_stream_bytes_remaining[1 as libc::c_int as usize] = size2;
    (*zip).sub_stream_bytes_remaining[2 as libc::c_int as usize] =
        bufferLim.offset_from(buffer) as libc::c_long as size_t;
    (*zip).bcj2_outPos =
        ((*zip).bcj2_outPos as libc::c_ulong).wrapping_add(outPos) as uint64_t as uint64_t;
    return outPos as ssize_t;
}
