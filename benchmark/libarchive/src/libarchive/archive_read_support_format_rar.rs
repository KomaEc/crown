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
    pub type archive_write;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
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
    fn _archive_entry_copy_symlink_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
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
        read_header_0: Option<
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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_string_default_conversion_for_read(_: *mut archive) -> *mut archive_string_conv;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __archive_read_ahead(
        _: *mut archive_read,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
    #[no_mangle]
    fn __archive_read_seek(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t;
    #[no_mangle]
    fn __archive_reset_read_data(_: *mut archive);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    /* must be 32-bit at least */
    /* ---------- Decode ---------- */
    /* ---------- Encode ---------- */
    /* Base Functions */
    /* Decode Functions */
    /* Encode Functions */
    #[no_mangle]
    static __archive_ppmd7_functions: IPpmd7;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
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
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
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
pub struct rar {
    pub main_flags: libc::c_uint,
    pub file_crc: libc::c_ulong,
    pub reserved1: [libc::c_char; 2],
    pub reserved2: [libc::c_char; 4],
    pub encryptver: libc::c_char,
    pub compression_method: libc::c_char,
    pub file_flags: libc::c_uint,
    pub packed_size: int64_t,
    pub unp_size: int64_t,
    pub mtime: time_t,
    pub mnsec: libc::c_long,
    pub mode: mode_t,
    pub filename: *mut libc::c_char,
    pub filename_save: *mut libc::c_char,
    pub filename_save_size: size_t,
    pub filename_allocated: size_t,
    pub salt: [libc::c_char; 8],
    pub atime: time_t,
    pub ansec: libc::c_long,
    pub ctime: time_t,
    pub cnsec: libc::c_long,
    pub arctime: time_t,
    pub arcnsec: libc::c_long,
    pub bytes_unconsumed: int64_t,
    pub bytes_remaining: int64_t,
    pub bytes_uncopied: int64_t,
    pub offset: int64_t,
    pub offset_outgoing: int64_t,
    pub offset_seek: int64_t,
    pub valid: libc::c_char,
    pub unp_offset: libc::c_uint,
    pub unp_buffer_size: libc::c_uint,
    pub unp_buffer: *mut libc::c_uchar,
    pub dictionary_size: libc::c_uint,
    pub start_new_block: libc::c_char,
    pub entry_eof: libc::c_char,
    pub crc_calculated: libc::c_ulong,
    pub found_first_header: libc::c_int,
    pub has_endarc_header: libc::c_char,
    pub dbo: *mut data_block_offsets,
    pub cursor: libc::c_uint,
    pub nodes: libc::c_uint,
    pub filename_must_match: libc::c_char,
    pub maincode: huffman_code,
    pub offsetcode: huffman_code,
    pub lowoffsetcode: huffman_code,
    pub lengthcode: huffman_code,
    pub lengthtable: [libc::c_uchar; 404],
    pub lzss: lzss,
    pub output_last_match: libc::c_char,
    pub lastlength: libc::c_uint,
    pub lastoffset: libc::c_uint,
    pub oldoffset: [libc::c_uint; 4],
    pub lastlowoffset: libc::c_uint,
    pub numlowoffsetrepeats: libc::c_uint,
    pub filterstart: int64_t,
    pub start_new_table: libc::c_char,
    pub ppmd_valid: libc::c_char,
    pub ppmd_eod: libc::c_char,
    pub is_ppmd_block: libc::c_char,
    pub ppmd_escape: libc::c_int,
    pub ppmd7_context: CPpmd7,
    pub range_dec: CPpmd7z_RangeDec,
    pub bytein: IByteIn,
    pub init_default_conversion: libc::c_int,
    pub sconv_default: *mut archive_string_conv,
    pub opt_sconv: *mut archive_string_conv,
    pub sconv_utf8: *mut archive_string_conv,
    pub sconv_utf16be: *mut archive_string_conv,
    pub br: rar_br,
    pub has_encrypted_entries: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rar_br {
    pub cache_buffer: uint64_t,
    pub cache_avail: libc::c_int,
    pub avail_in: ssize_t,
    pub next_in: *const libc::c_uchar,
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
pub struct lzss {
    pub window: *mut libc::c_uchar,
    pub mask: libc::c_int,
    pub position: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffman_code {
    pub tree: *mut huffman_tree_node,
    pub numentries: libc::c_int,
    pub numallocatedentries: libc::c_int,
    pub minlength: libc::c_int,
    pub maxlength: libc::c_int,
    pub tablesize: libc::c_int,
    pub table: *mut huffman_table_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffman_table_entry {
    pub length: libc::c_uint,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffman_tree_node {
    pub branches: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_block_offsets {
    pub header_size: int64_t,
    pub start_offset: int64_t,
    pub end_offset: int64_t,
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
/* Fields common to all file headers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rar_file_header {
    pub pack_size: [libc::c_char; 4],
    pub unp_size: [libc::c_char; 4],
    pub host_os: libc::c_char,
    pub file_crc: [libc::c_char; 4],
    pub file_time: [libc::c_char; 4],
    pub unp_ver: libc::c_char,
    pub method: libc::c_char,
    pub name_size: [libc::c_char; 2],
    pub file_attr: [libc::c_char; 4],
}
/* Fields common to all headers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rar_header {
    pub crc: [libc::c_char; 2],
    pub type_0: libc::c_char,
    pub flags: [libc::c_char; 2],
    pub size: [libc::c_char; 2],
}
pub const INT64_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
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
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const INT_MIN: libc::c_int = -__INT_MAX__ - 1 as libc::c_int;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const S_IRUSR: libc::c_int = __S_IREAD;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW: libc::c_int = -(1 as libc::c_int);
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA: libc::c_int =
    (1 as libc::c_int) << 1 as libc::c_int;
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA: libc::c_int =
    (1 as libc::c_int) << 0 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const __S_IREAD: libc::c_int = 0o400 as libc::c_int;
pub const __S_IWRITE: libc::c_int = 0o200 as libc::c_int;
pub const __S_IEXEC: libc::c_int = 0o100 as libc::c_int;
pub const ARCHIVE_FORMAT_RAR: libc::c_int = 0xd0000 as libc::c_int;
pub const S_IXOTH: libc::c_int = S_IXGRP >> 3 as libc::c_int;
pub const S_IXGRP: libc::c_int = S_IXUSR >> 3 as libc::c_int;
pub const S_IXUSR: libc::c_int = 0o100 as libc::c_int;
pub const S_IWUSR: libc::c_int = __S_IWRITE;
pub const S_IROTH: libc::c_int = S_IRGRP >> 3 as libc::c_int;
pub const S_IRGRP: libc::c_int = S_IRUSR >> 3 as libc::c_int;
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
pub const archive_entry_copy_symlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_symlink_l;
/*-
* Copyright (c) 2003-2007 Tim Kientzle
* Copyright (c) 2011 Andres Mejia
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
/* crc32 */
/* RAR signature, also known as the mark header */
pub const RAR_SIGNATURE: [libc::c_char; 8] =
    unsafe { *::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"Rar!\x1a\x07\x00\x00") };
/* Header types */
pub const MARK_HEAD: libc::c_int = 0x72 as libc::c_int;
pub const MAIN_HEAD: libc::c_int = 0x73 as libc::c_int;
pub const FILE_HEAD: libc::c_int = 0x74 as libc::c_int;
pub const COMM_HEAD: libc::c_int = 0x75 as libc::c_int;
pub const AV_HEAD: libc::c_int = 0x76 as libc::c_int;
pub const SUB_HEAD: libc::c_int = 0x77 as libc::c_int;
pub const PROTECT_HEAD: libc::c_int = 0x78 as libc::c_int;
pub const SIGN_HEAD: libc::c_int = 0x79 as libc::c_int;
pub const NEWSUB_HEAD: libc::c_int = 0x7a as libc::c_int;
pub const ENDARC_HEAD: libc::c_int = 0x7b as libc::c_int;
/* Main Header Flags */
pub const MHD_VOLUME: libc::c_int = 0x1 as libc::c_int;
pub const MHD_PASSWORD: libc::c_int = 0x80 as libc::c_int;
pub const MHD_ENCRYPTVER: libc::c_int = 0x200 as libc::c_int;
/* Flags common to all headers */
pub const HD_ADD_SIZE_PRESENT: libc::c_int = 0x8000 as libc::c_int;
/* File Header Flags */
pub const FHD_SPLIT_BEFORE: libc::c_int = 0x1 as libc::c_int;
pub const FHD_SPLIT_AFTER: libc::c_int = 0x2 as libc::c_int;
pub const FHD_PASSWORD: libc::c_int = 0x4 as libc::c_int;
pub const FHD_SOLID: libc::c_int = 0x10 as libc::c_int;
pub const FHD_LARGE: libc::c_int = 0x100 as libc::c_int;
pub const FHD_UNICODE: libc::c_int = 0x200 as libc::c_int;
pub const FHD_SALT: libc::c_int = 0x400 as libc::c_int;
pub const FHD_EXTTIME: libc::c_int = 0x1000 as libc::c_int;
/* File dictionary sizes */
/* OS Flags */
pub const OS_MSDOS: libc::c_int = 0 as libc::c_int;
pub const OS_OS2: libc::c_int = 1 as libc::c_int;
pub const OS_WIN32: libc::c_int = 2 as libc::c_int;
pub const OS_UNIX: libc::c_int = 3 as libc::c_int;
pub const OS_MAC_OS: libc::c_int = 4 as libc::c_int;
pub const OS_BEOS: libc::c_int = 5 as libc::c_int;
/* Compression Methods */
pub const COMPRESS_METHOD_STORE: libc::c_int = 0x30 as libc::c_int;
/* LZSS */
pub const COMPRESS_METHOD_FASTEST: libc::c_int = 0x31 as libc::c_int;
pub const COMPRESS_METHOD_FAST: libc::c_int = 0x32 as libc::c_int;
pub const COMPRESS_METHOD_NORMAL: libc::c_int = 0x33 as libc::c_int;
/* PPMd Variant H */
pub const COMPRESS_METHOD_GOOD: libc::c_int = 0x34 as libc::c_int;
pub const COMPRESS_METHOD_BEST: libc::c_int = 0x35 as libc::c_int;
pub const NS_UNIT: libc::c_int = 10000000 as libc::c_int;
pub const DICTIONARY_MAX_SIZE: libc::c_int = 0x400000 as libc::c_int;
pub const MAINCODE_SIZE: libc::c_int = 299 as libc::c_int;
pub const OFFSETCODE_SIZE: libc::c_int = 60 as libc::c_int;
pub const LOWOFFSETCODE_SIZE: libc::c_int = 17 as libc::c_int;
pub const LENGTHCODE_SIZE: libc::c_int = 28 as libc::c_int;
pub const MAX_SYMBOL_LENGTH: libc::c_int = 0xf as libc::c_int;
pub const MAX_SYMBOLS: libc::c_int = 20 as libc::c_int;
/*
 * Considering L1,L2 cache miss and a calling of write system-call,
 * the best size of the output buffer(uncompressed buffer) is 128K.
 * If the structure of extracting process is changed, this value
 * might be researched again.
 */
pub const UNP_BUFFER_SIZE: libc::c_int = 128 as libc::c_int * 1024 as libc::c_int;
/* Define this here for non-Windows platforms */
pub const FILE_ATTRIBUTE_DIRECTORY: libc::c_int = 0x10 as libc::c_int;
/* Notify how many bits we consumed. */
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
unsafe extern "C" fn rar_br_fillup(mut a: *mut archive_read, mut br: *mut rar_br) -> libc::c_int {
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    let mut n: libc::c_int = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong)
        .wrapping_sub((*br).cache_avail as libc::c_ulong)
        as libc::c_int;
    loop {
        match n >> 3 as libc::c_int {
            8 => {
                if (*br).avail_in >= 8 as libc::c_int as libc::c_long {
                    (*br).cache_buffer = (*(*br).next_in.offset(0 as libc::c_int as isize)
                        as uint64_t)
                        << 56 as libc::c_int
                        | (*(*br).next_in.offset(1 as libc::c_int as isize) as uint64_t)
                            << 48 as libc::c_int
                        | (*(*br).next_in.offset(2 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*br).next_in.offset(3 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*br).next_in.offset(4 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*br).next_in.offset(5 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*br).next_in.offset(6 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*br).next_in.offset(7 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*br).next_in = (*br).next_in.offset(8 as libc::c_int as isize);
                    (*br).avail_in -= 8 as libc::c_int as libc::c_long;
                    (*br).cache_avail += 8 as libc::c_int * 8 as libc::c_int;
                    (*rar).bytes_unconsumed += 8 as libc::c_int as libc::c_long;
                    (*rar).bytes_remaining -= 8 as libc::c_int as libc::c_long;
                    return 1 as libc::c_int;
                }
            }
            7 => {
                if (*br).avail_in >= 7 as libc::c_int as libc::c_long {
                    (*br).cache_buffer = (*br).cache_buffer << 56 as libc::c_int
                        | (*(*br).next_in.offset(0 as libc::c_int as isize) as uint64_t)
                            << 48 as libc::c_int
                        | (*(*br).next_in.offset(1 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*br).next_in.offset(2 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*br).next_in.offset(3 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*br).next_in.offset(4 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*br).next_in.offset(5 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*br).next_in.offset(6 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*br).next_in = (*br).next_in.offset(7 as libc::c_int as isize);
                    (*br).avail_in -= 7 as libc::c_int as libc::c_long;
                    (*br).cache_avail += 7 as libc::c_int * 8 as libc::c_int;
                    (*rar).bytes_unconsumed += 7 as libc::c_int as libc::c_long;
                    (*rar).bytes_remaining -= 7 as libc::c_int as libc::c_long;
                    return 1 as libc::c_int;
                }
            }
            6 => {
                if (*br).avail_in >= 6 as libc::c_int as libc::c_long {
                    (*br).cache_buffer = (*br).cache_buffer << 48 as libc::c_int
                        | (*(*br).next_in.offset(0 as libc::c_int as isize) as uint64_t)
                            << 40 as libc::c_int
                        | (*(*br).next_in.offset(1 as libc::c_int as isize) as uint64_t)
                            << 32 as libc::c_int
                        | ((*(*br).next_in.offset(2 as libc::c_int as isize) as uint32_t)
                            << 24 as libc::c_int) as libc::c_ulong
                        | ((*(*br).next_in.offset(3 as libc::c_int as isize) as uint32_t)
                            << 16 as libc::c_int) as libc::c_ulong
                        | ((*(*br).next_in.offset(4 as libc::c_int as isize) as uint32_t)
                            << 8 as libc::c_int) as libc::c_ulong
                        | *(*br).next_in.offset(5 as libc::c_int as isize) as uint32_t
                            as libc::c_ulong;
                    (*br).next_in = (*br).next_in.offset(6 as libc::c_int as isize);
                    (*br).avail_in -= 6 as libc::c_int as libc::c_long;
                    (*br).cache_avail += 6 as libc::c_int * 8 as libc::c_int;
                    (*rar).bytes_unconsumed += 6 as libc::c_int as libc::c_long;
                    (*rar).bytes_remaining -= 6 as libc::c_int as libc::c_long;
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
        if (*br).avail_in <= 0 as libc::c_int as libc::c_long {
            if (*rar).bytes_unconsumed > 0 as libc::c_int as libc::c_long {
                /* Consume as much as the decompressor
                 * actually used. */
                __archive_read_consume(a, (*rar).bytes_unconsumed);
                (*rar).bytes_unconsumed = 0 as libc::c_int as int64_t
            }
            (*br).next_in = rar_read_ahead(a, 1 as libc::c_int as size_t, &mut (*br).avail_in)
                as *const libc::c_uchar;
            if (*br).next_in.is_null() {
                return 0 as libc::c_int;
            }
            if (*br).avail_in == 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
        }
        let fresh0 = (*br).next_in;
        (*br).next_in = (*br).next_in.offset(1);
        (*br).cache_buffer = (*br).cache_buffer << 8 as libc::c_int | *fresh0 as libc::c_ulong;
        (*br).avail_in -= 1;
        (*br).cache_avail += 8 as libc::c_int;
        n -= 8 as libc::c_int;
        (*rar).bytes_unconsumed += 1;
        (*rar).bytes_remaining -= 1
    }
}
unsafe extern "C" fn rar_br_preparation(
    mut a: *mut archive_read,
    mut br: *mut rar_br,
) -> libc::c_int {
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    if (*rar).bytes_remaining > 0 as libc::c_int as libc::c_long {
        (*br).next_in = rar_read_ahead(a, 1 as libc::c_int as size_t, &mut (*br).avail_in)
            as *const libc::c_uchar;
        if (*br).next_in.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if (*br).cache_avail == 0 as libc::c_int {
            rar_br_fillup(a, br);
        }
    }
    return 0 as libc::c_int;
}
/* Find last bit set */
#[inline]
unsafe extern "C" fn rar_fls(mut word: libc::c_uint) -> libc::c_int {
    word |= word >> 1 as libc::c_int;
    word |= word >> 2 as libc::c_int;
    word |= word >> 4 as libc::c_int;
    word |= word >> 8 as libc::c_int;
    word |= word >> 16 as libc::c_int;
    return word.wrapping_sub(word >> 1 as libc::c_int) as libc::c_int;
}
/* LZSS functions */
#[inline]
unsafe extern "C" fn lzss_position(mut lzss: *mut lzss) -> int64_t {
    return (*lzss).position;
}
#[inline]
unsafe extern "C" fn lzss_mask(mut lzss: *mut lzss) -> libc::c_int {
    return (*lzss).mask;
}
#[inline]
unsafe extern "C" fn lzss_size(mut lzss: *mut lzss) -> libc::c_int {
    return (*lzss).mask + 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn lzss_offset_for_position(
    mut lzss: *mut lzss,
    mut pos: int64_t,
) -> libc::c_int {
    return (pos & (*lzss).mask as libc::c_long) as libc::c_int;
}
#[inline]
unsafe extern "C" fn lzss_pointer_for_position(
    mut lzss: *mut lzss,
    mut pos: int64_t,
) -> *mut libc::c_uchar {
    return &mut *(*lzss).window.offset((lzss_offset_for_position
        as unsafe extern "C" fn(_: *mut lzss, _: int64_t) -> libc::c_int)(
        lzss, pos
    ) as isize) as *mut libc::c_uchar;
}
#[inline]
unsafe extern "C" fn lzss_current_offset(mut lzss: *mut lzss) -> libc::c_int {
    return lzss_offset_for_position(lzss, (*lzss).position);
}
#[inline]
unsafe extern "C" fn lzss_current_pointer(mut lzss: *mut lzss) -> *mut uint8_t {
    return lzss_pointer_for_position(lzss, (*lzss).position);
}
#[inline]
unsafe extern "C" fn lzss_emit_literal(mut rar: *mut rar, mut literal: uint8_t) {
    *lzss_current_pointer(&mut (*rar).lzss) = literal;
    (*rar).lzss.position += 1;
}
#[inline]
unsafe extern "C" fn lzss_emit_match(
    mut rar: *mut rar,
    mut offset: libc::c_int,
    mut length: libc::c_int,
) {
    let mut dstoffs: libc::c_int = lzss_current_offset(&mut (*rar).lzss);
    let mut srcoffs: libc::c_int = dstoffs - offset & lzss_mask(&mut (*rar).lzss);
    let mut l: libc::c_int = 0;
    let mut li: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    remaining = length;
    while remaining > 0 as libc::c_int {
        l = remaining;
        if dstoffs > srcoffs {
            if l > lzss_size(&mut (*rar).lzss) - dstoffs {
                l = lzss_size(&mut (*rar).lzss) - dstoffs
            }
        } else if l > lzss_size(&mut (*rar).lzss) - srcoffs {
            l = lzss_size(&mut (*rar).lzss) - srcoffs
        }
        d = &mut *(*rar).lzss.window.offset(dstoffs as isize) as *mut libc::c_uchar;
        s = &mut *(*rar).lzss.window.offset(srcoffs as isize) as *mut libc::c_uchar;
        if dstoffs + l < srcoffs || srcoffs + l < dstoffs {
            memcpy(
                d as *mut libc::c_void,
                s as *const libc::c_void,
                l as libc::c_ulong,
            );
        } else {
            li = 0 as libc::c_int;
            while li < l {
                *d.offset(li as isize) = *s.offset(li as isize);
                li += 1
            }
        }
        remaining -= l;
        dstoffs = dstoffs + l & lzss_mask(&mut (*rar).lzss);
        srcoffs = srcoffs + l & lzss_mask(&mut (*rar).lzss)
    }
    (*rar).lzss.position += length as libc::c_long;
}
unsafe extern "C" fn ppmd_read(mut p: *mut libc::c_void) -> Byte {
    let mut a: *mut archive_read = (*(p as *mut IByteIn)).a;
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    let mut br: *mut rar_br = &mut (*rar).br;
    let mut b: Byte = 0;
    if !((*br).cache_avail >= 8 as libc::c_int
        || rar_br_fillup(a, br) != 0
        || (*br).cache_avail >= 8 as libc::c_int)
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
        );
        (*rar).valid = 0 as libc::c_int as libc::c_char;
        return 0 as libc::c_int as Byte;
    }
    b = (((*br).cache_buffer >> (*br).cache_avail - 8 as libc::c_int) as uint32_t
        & cache_masks[8 as libc::c_int as usize]) as Byte;
    (*br).cache_avail -= 8 as libc::c_int;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_rar(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut rar: *mut rar = 0 as *mut rar;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_rar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    rar = calloc(
        ::std::mem::size_of::<rar>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut rar;
    if rar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate rar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * Until enough data has been read, we cannot tell about
     * any encrypted entries yet.
     */
    (*rar).has_encrypted_entries = ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
    r = __archive_read_register_format(
        a,
        rar as *mut libc::c_void,
        b"rar\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_rar_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_rar_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_rar_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_rar_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_rar_read_data_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_format_rar_seek_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: int64_t,
                    _: libc::c_int,
                ) -> int64_t,
        ),
        Some(
            archive_read_format_rar_cleanup
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_support_format_rar_capabilities
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        Some(
            archive_read_format_rar_has_encrypted_entries
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
    );
    if r != ARCHIVE_OK {
        free(rar as *mut libc::c_void);
    }
    return r;
}
unsafe extern "C" fn archive_read_support_format_rar_capabilities(
    mut a: *mut archive_read,
) -> libc::c_int {
    /* UNUSED */
    return ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA | ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA;
}
unsafe extern "C" fn archive_read_format_rar_has_encrypted_entries(
    mut _a: *mut archive_read,
) -> libc::c_int {
    if !_a.is_null() && !(*_a).format.is_null() {
        let mut rar: *mut rar = (*(*_a).format).data as *mut rar;
        if !rar.is_null() {
            return (*rar).has_encrypted_entries;
        }
    }
    return ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
}
unsafe extern "C" fn archive_read_format_rar_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* If there's already a bid > 30, we'll never win. */
    if best_bid > 30 as libc::c_int {
        return -(1 as libc::c_int);
    }
    p = __archive_read_ahead(a, 7 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_char;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    if memcmp(
        p as *const libc::c_void,
        RAR_SIGNATURE.as_ptr(),
        7 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 30 as libc::c_int;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
        || memcmp(
            p as *const libc::c_void,
            b"\x7fELF\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        /* This is a PE file */
        let mut offset: ssize_t = 0x10000 as libc::c_int as ssize_t;
        let mut window: ssize_t = 4096 as libc::c_int as ssize_t;
        let mut bytes_avail: ssize_t = 0;
        while offset + window <= (1024 as libc::c_int * 128 as libc::c_int) as libc::c_long {
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
                while p.offset(7 as libc::c_int as isize) < buff.offset(bytes_avail as isize) {
                    if memcmp(
                        p as *const libc::c_void,
                        RAR_SIGNATURE.as_ptr(),
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        return 30 as libc::c_int;
                    }
                    p = p.offset(0x10 as libc::c_int as isize)
                }
                offset = p.offset_from(buff) as libc::c_long
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn skip_sfx(mut a: *mut archive_read) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut skip: size_t = 0;
    let mut total: size_t = 0;
    let mut bytes: ssize_t = 0;
    let mut window: ssize_t = 0;
    total = 0 as libc::c_int as size_t;
    window = 4096 as libc::c_int as ssize_t;
    while total.wrapping_add(window as libc::c_ulong)
        <= (1024 as libc::c_int * 128 as libc::c_int) as libc::c_ulong
    {
        h = __archive_read_ahead(a, window as size_t, &mut bytes);
        if h == NULL as *const libc::c_void {
            /* Remaining bytes are less than window. */
            window >>= 1 as libc::c_int;
            if window < 0x40 as libc::c_int as libc::c_long {
                break;
            }
        } else {
            if bytes < 0x40 as libc::c_int as libc::c_long {
                break;
            }
            p = h as *const libc::c_char;
            q = p.offset(bytes as isize);
            /*
             * Scan ahead until we find something that looks
             * like the RAR header.
             */
            while p.offset(7 as libc::c_int as isize) < q {
                if memcmp(
                    p as *const libc::c_void,
                    RAR_SIGNATURE.as_ptr(),
                    7 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    skip =
                        p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
                    __archive_read_consume(a, skip as int64_t);
                    return 0 as libc::c_int;
                }
                p = p.offset(0x10 as libc::c_int as isize)
            }
            skip = p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
            __archive_read_consume(a, skip as int64_t);
            total = (total as libc::c_ulong).wrapping_add(skip) as size_t as size_t
        }
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Couldn\'t find out RAR header\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn archive_read_format_rar_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut rar: *mut rar = 0 as *mut rar;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    rar = (*(*a).format).data as *mut rar;
    if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if val.is_null()
            || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"rar: hdrcharset option needs a character-set name\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            (*rar).opt_sconv =
                archive_string_conversion_from_charset(&mut (*a).archive, val, 0 as libc::c_int);
            if !(*rar).opt_sconv.is_null() {
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
unsafe extern "C" fn archive_read_format_rar_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut rar: *mut rar = 0 as *mut rar;
    let mut skip: size_t = 0;
    let mut head_type: libc::c_char = 0;
    let mut ret: libc::c_int = 0;
    let mut flags: libc::c_uint = 0;
    let mut crc32_expected: libc::c_ulong = 0;
    (*a).archive.archive_format = ARCHIVE_FORMAT_RAR;
    if (*a).archive.archive_format_name.is_null() {
        (*a).archive.archive_format_name = b"RAR\x00" as *const u8 as *const libc::c_char
    }
    rar = (*(*a).format).data as *mut rar;
    /*
     * It should be sufficient to call archive_read_next_header() for
     * a reader to determine if an entry is encrypted or not. If the
     * encryption of an entry is only detectable when calling
     * archive_read_data(), so be it. We'll do the same check there
     * as well.
     */
    if (*rar).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*rar).has_encrypted_entries = 0 as libc::c_int
    }
    /* RAR files can be generated without EOF headers, so return ARCHIVE_EOF if
     * this fails.
     */
    h = __archive_read_ahead(a, 7 as libc::c_int as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        return 1 as libc::c_int;
    }
    p = h as *const libc::c_char;
    if (*rar).found_first_header == 0 as libc::c_int
        && (*p.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
            || memcmp(
                p as *const libc::c_void,
                b"\x7fELF\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
    {
        /* This is an executable ? Must be self-extracting... */
        ret = skip_sfx(a);
        if ret < ARCHIVE_WARN {
            return ret;
        }
    }
    (*rar).found_first_header = 1 as libc::c_int;
    loop {
        let mut crc32_val: libc::c_ulong = 0;
        h = __archive_read_ahead(a, 7 as libc::c_int as size_t, NULL as *mut ssize_t);
        if h == NULL as *const libc::c_void {
            return -(30 as libc::c_int);
        }
        p = h as *const libc::c_char;
        head_type = *p.offset(2 as libc::c_int as isize);
        match head_type as libc::c_int {
            MARK_HEAD => {
                if memcmp(
                    p as *const libc::c_void,
                    RAR_SIGNATURE.as_ptr(),
                    7 as libc::c_int as libc::c_ulong,
                ) != 0 as libc::c_int
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Invalid marker header\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                __archive_read_consume(a, 7 as libc::c_int as int64_t);
            }
            MAIN_HEAD => {
                (*rar).main_flags =
                    archive_le16dec(p.offset(3 as libc::c_int as isize) as *const libc::c_void)
                        as libc::c_uint;
                skip = archive_le16dec(p.offset(5 as libc::c_int as isize) as *const libc::c_void)
                    as size_t;
                if skip
                    < (7 as libc::c_int as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Invalid header size\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                h = __archive_read_ahead(a, skip, NULL as *mut ssize_t);
                if h == NULL as *const libc::c_void {
                    return -(30 as libc::c_int);
                }
                p = h as *const libc::c_char;
                memcpy(
                    (*rar).reserved1.as_mut_ptr() as *mut libc::c_void,
                    p.offset(7 as libc::c_int as isize) as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                );
                memcpy((*rar).reserved2.as_mut_ptr() as *mut libc::c_void,
                       p.offset(7 as libc::c_int as
                                    isize).offset(::std::mem::size_of::<[libc::c_char; 2]>()
                                                      as libc::c_ulong as
                                                      isize) as
                           *const libc::c_void,
                       ::std::mem::size_of::<[libc::c_char; 4]>() as
                           libc::c_ulong);
                if (*rar).main_flags & MHD_ENCRYPTVER as libc::c_uint != 0 {
                    if skip
                        < (7 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                            )
                            .wrapping_add(
                                ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Invalid header size\x00" as *const u8 as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    (*rar).encryptver =
                        *p.offset(7 as libc::c_int as isize)
                            .offset(::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                                as isize)
                            .offset(::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                                as isize)
                }
                /* Main header is password encrypted, so we cannot read any
                file names or any other info about files from the header. */
                if (*rar).main_flags & MHD_PASSWORD as libc::c_uint != 0 {
                    archive_entry_set_is_metadata_encrypted(
                        entry,
                        1 as libc::c_int as libc::c_char,
                    );
                    archive_entry_set_is_data_encrypted(entry, 1 as libc::c_int as libc::c_char);
                    (*rar).has_encrypted_entries = 1 as libc::c_int;
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"RAR encryption support unavailable.\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                crc32_val = crc32(
                    0 as libc::c_int as uLong,
                    (p as *const libc::c_uchar).offset(2 as libc::c_int as isize),
                    (skip as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint),
                );
                if crc32_val & 0xffff as libc::c_int as libc::c_ulong
                    != archive_le16dec(p as *const libc::c_void) as libc::c_ulong
                {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Header CRC error\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                __archive_read_consume(a, skip as int64_t);
            }
            FILE_HEAD => return read_header(a, entry, head_type),
            COMM_HEAD | AV_HEAD | SUB_HEAD | PROTECT_HEAD | SIGN_HEAD | ENDARC_HEAD => {
                flags = archive_le16dec(p.offset(3 as libc::c_int as isize) as *const libc::c_void)
                    as libc::c_uint;
                skip = archive_le16dec(p.offset(5 as libc::c_int as isize) as *const libc::c_void)
                    as size_t;
                if skip < 7 as libc::c_int as libc::c_ulong {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Invalid header size too small\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                if flags & HD_ADD_SIZE_PRESENT as libc::c_uint != 0 {
                    if skip < (7 as libc::c_int + 4 as libc::c_int) as libc::c_ulong {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Invalid header size too small\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    h = __archive_read_ahead(a, skip, NULL as *mut ssize_t);
                    if h == NULL as *const libc::c_void {
                        return -(30 as libc::c_int);
                    }
                    p = h as *const libc::c_char;
                    skip = (skip as libc::c_ulong)
                        .wrapping_add(archive_le32dec(
                            p.offset(7 as libc::c_int as isize) as *const libc::c_void
                        ) as libc::c_ulong) as size_t as size_t
                }
                /* Skip over the 2-byte CRC at the beginning of the header. */
                crc32_expected = archive_le16dec(p as *const libc::c_void) as libc::c_ulong;
                __archive_read_consume(a, 2 as libc::c_int as int64_t);
                skip = (skip as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                /* Skim the entire header and compute the CRC. */
                crc32_val = 0 as libc::c_int as libc::c_ulong;
                while skip > 0 as libc::c_int as libc::c_ulong {
                    let mut to_read: size_t = skip;
                    let mut did_read: ssize_t = 0;
                    if to_read > (32 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
                        to_read = (32 as libc::c_int * 1024 as libc::c_int) as size_t
                    }
                    h = __archive_read_ahead(a, to_read, &mut did_read);
                    if h == NULL as *const libc::c_void {
                        return -(30 as libc::c_int);
                    }
                    p = h as *const libc::c_char;
                    crc32_val = crc32(
                        crc32_val,
                        p as *const libc::c_uchar,
                        did_read as libc::c_uint,
                    );
                    __archive_read_consume(a, did_read);
                    skip = (skip as libc::c_ulong).wrapping_sub(did_read as libc::c_ulong) as size_t
                        as size_t
                }
                if crc32_val & 0xffff as libc::c_int as libc::c_ulong != crc32_expected {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Header CRC error\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                if head_type as libc::c_int == ENDARC_HEAD {
                    return 1 as libc::c_int;
                }
            }
            NEWSUB_HEAD => {
                ret = read_header(a, entry, head_type);
                if ret < ARCHIVE_WARN {
                    return ret;
                }
            }
            _ => {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Bad RAR file\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
    }
}
unsafe extern "C" fn archive_read_format_rar_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    let mut ret: libc::c_int = 0;
    if (*rar).has_encrypted_entries == ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW {
        (*rar).has_encrypted_entries = 0 as libc::c_int
    }
    if (*rar).bytes_unconsumed > 0 as libc::c_int as libc::c_long {
        /* Consume as much as the decompressor actually used. */
        __archive_read_consume(a, (*rar).bytes_unconsumed);
        (*rar).bytes_unconsumed = 0 as libc::c_int as int64_t
    }
    *buff = NULL as *const libc::c_void;
    if (*rar).entry_eof as libc::c_int != 0 || (*rar).offset_seek >= (*rar).unp_size {
        *size = 0 as libc::c_int as size_t;
        *offset = (*rar).offset;
        if *offset < (*rar).unp_size {
            *offset = (*rar).unp_size
        }
        return 1 as libc::c_int;
    }
    match (*rar).compression_method as libc::c_int {
        COMPRESS_METHOD_STORE => ret = read_data_stored(a, buff, size, offset),
        COMPRESS_METHOD_FASTEST
        | COMPRESS_METHOD_FAST
        | COMPRESS_METHOD_NORMAL
        | COMPRESS_METHOD_GOOD
        | COMPRESS_METHOD_BEST => {
            ret = read_data_compressed(a, buff, size, offset);
            if ret != ARCHIVE_OK && ret != ARCHIVE_WARN {
                __archive_ppmd7_functions
                    .Ppmd7_Free
                    .expect("non-null function pointer")(&mut (*rar).ppmd7_context);
                (*rar).start_new_table = 1 as libc::c_int as libc::c_char;
                (*rar).ppmd_valid = 0 as libc::c_int as libc::c_char
            }
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unsupported compression method for RAR file.\x00" as *const u8
                    as *const libc::c_char,
            );
            ret = ARCHIVE_FATAL
        }
    }
    return ret;
}
unsafe extern "C" fn archive_read_format_rar_read_data_skip(
    mut a: *mut archive_read,
) -> libc::c_int {
    let mut rar: *mut rar = 0 as *mut rar;
    let mut bytes_skipped: int64_t = 0;
    let mut ret: libc::c_int = 0;
    rar = (*(*a).format).data as *mut rar;
    if (*rar).bytes_unconsumed > 0 as libc::c_int as libc::c_long {
        /* Consume as much as the decompressor actually used. */
        __archive_read_consume(a, (*rar).bytes_unconsumed);
        (*rar).bytes_unconsumed = 0 as libc::c_int as int64_t
    }
    if (*rar).bytes_remaining > 0 as libc::c_int as libc::c_long {
        bytes_skipped = __archive_read_consume(a, (*rar).bytes_remaining);
        if bytes_skipped < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int);
        }
    }
    /* Compressed data to skip must be read from each header in a multivolume
     * archive.
     */
    if (*rar).main_flags & MHD_VOLUME as libc::c_uint != 0
        && (*rar).file_flags & FHD_SPLIT_AFTER as libc::c_uint != 0
    {
        ret = archive_read_format_rar_read_header(a, (*a).entry);
        if ret == 1 as libc::c_int {
            ret = archive_read_format_rar_read_header(a, (*a).entry)
        }
        if ret != 0 as libc::c_int {
            return ret;
        }
        return archive_read_format_rar_read_data_skip(a);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_rar_seek_data(
    mut a: *mut archive_read,
    mut offset: int64_t,
    mut whence: libc::c_int,
) -> int64_t {
    let mut client_offset: int64_t = 0;
    let mut ret: int64_t = 0;
    let mut i: libc::c_uint = 0;
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    if (*rar).compression_method as libc::c_int == COMPRESS_METHOD_STORE {
        /* Modify the offset for use with SEEK_SET */
        match whence {
            SEEK_CUR => client_offset = (*rar).offset_seek,
            SEEK_END => client_offset = (*rar).unp_size,
            SEEK_SET | _ => client_offset = 0 as libc::c_int as int64_t,
        }
        client_offset += offset;
        if client_offset < 0 as libc::c_int as libc::c_long {
            /* Can't seek past beginning of data block */
            return -(1 as libc::c_int) as int64_t;
        } else {
            if client_offset > (*rar).unp_size {
                /*
                 * Set the returned offset but only seek to the end of
                 * the data block.
                 */
                (*rar).offset_seek = client_offset;
                client_offset = (*rar).unp_size
            }
        }
        client_offset += (*(*rar).dbo.offset(0 as libc::c_int as isize)).start_offset;
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*rar).cursor {
            i = i.wrapping_add(1);
            client_offset += (*(*rar).dbo.offset(i as isize)).start_offset
                - (*(*rar)
                    .dbo
                    .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
                .end_offset
        }
        if (*rar).main_flags & MHD_VOLUME as libc::c_uint != 0 {
            loop
            /* Find the appropriate offset among the multivolume archive */
            {
                if client_offset < (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset
                    && (*rar).file_flags & FHD_SPLIT_BEFORE as libc::c_uint != 0
                {
                    /* Search backwards for the correct data block */
                    if (*rar).cursor == 0 as libc::c_int as libc::c_uint {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"Attempt to seek past beginning of RAR data block\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return -(25 as libc::c_int) as int64_t;
                    }
                    (*rar).cursor = (*rar).cursor.wrapping_sub(1);
                    client_offset -=
                        (*(*rar)
                            .dbo
                            .offset((*rar).cursor.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as isize))
                        .start_offset
                            - (*(*rar).dbo.offset((*rar).cursor as isize)).end_offset;
                    if client_offset < (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset {
                        continue;
                    }
                    ret = __archive_read_seek(
                        a,
                        (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset
                            - (*(*rar).dbo.offset((*rar).cursor as isize)).header_size,
                        SEEK_SET,
                    );
                    if ret < 0 as libc::c_int as libc::c_long {
                        return ret;
                    }
                    ret = archive_read_format_rar_read_header(a, (*a).entry) as int64_t;
                    if ret != 0 as libc::c_int as libc::c_long {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"Error during seek of RAR file\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return -(25 as libc::c_int) as int64_t;
                    }
                    (*rar).cursor = (*rar).cursor.wrapping_sub(1);
                    break;
                } else {
                    if !(client_offset > (*(*rar).dbo.offset((*rar).cursor as isize)).end_offset
                        && (*rar).file_flags & FHD_SPLIT_AFTER as libc::c_uint != 0)
                    {
                        break;
                    }
                    /* Search forward for the correct data block */
                    (*rar).cursor = (*rar).cursor.wrapping_add(1);
                    if (*rar).cursor < (*rar).nodes
                        && client_offset > (*(*rar).dbo.offset((*rar).cursor as isize)).end_offset
                    {
                        client_offset += (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset
                            - (*(*rar).dbo.offset(
                                (*rar).cursor.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ))
                            .end_offset
                    } else {
                        (*rar).cursor = (*rar).cursor.wrapping_sub(1);
                        ret = __archive_read_seek(
                            a,
                            (*(*rar).dbo.offset((*rar).cursor as isize)).end_offset,
                            SEEK_SET,
                        );
                        if ret < 0 as libc::c_int as libc::c_long {
                            return ret;
                        }
                        ret = archive_read_format_rar_read_header(a, (*a).entry) as int64_t;
                        if ret == 1 as libc::c_int as libc::c_long {
                            (*rar).has_endarc_header = 1 as libc::c_int as libc::c_char;
                            ret = archive_read_format_rar_read_header(a, (*a).entry) as int64_t
                        }
                        if ret != 0 as libc::c_int as libc::c_long {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"Error during seek of RAR file\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            return -(25 as libc::c_int) as int64_t;
                        }
                        client_offset += (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset
                            - (*(*rar).dbo.offset(
                                (*rar).cursor.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ))
                            .end_offset
                    }
                }
            }
        }
        ret = __archive_read_seek(a, client_offset, SEEK_SET);
        if ret < 0 as libc::c_int as libc::c_long {
            return ret;
        }
        (*rar).bytes_remaining = (*(*rar).dbo.offset((*rar).cursor as isize)).end_offset - ret;
        i = (*rar).cursor;
        while i > 0 as libc::c_int as libc::c_uint {
            i = i.wrapping_sub(1);
            ret -= (*(*rar)
                .dbo
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
            .start_offset
                - (*(*rar).dbo.offset(i as isize)).end_offset
        }
        ret -= (*(*rar).dbo.offset(0 as libc::c_int as isize)).start_offset;
        /* Always restart reading the file after a seek */
        __archive_reset_read_data(&mut (*a).archive);
        (*rar).bytes_unconsumed = 0 as libc::c_int as int64_t;
        (*rar).offset = 0 as libc::c_int as int64_t;
        /*
         * If a seek past the end of file was requested, return the requested
         * offset.
         */
        if ret == (*rar).unp_size && (*rar).offset_seek > (*rar).unp_size {
            return (*rar).offset_seek;
        }
        /* Return the new offset */
        (*rar).offset_seek = ret;
        return (*rar).offset_seek;
    } else {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Seeking of compressed RAR files is unsupported\x00" as *const u8
                as *const libc::c_char,
        );
    }
    return -(25 as libc::c_int) as int64_t;
}
unsafe extern "C" fn archive_read_format_rar_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut rar: *mut rar = 0 as *mut rar;
    rar = (*(*a).format).data as *mut rar;
    free_codes(a);
    free((*rar).filename as *mut libc::c_void);
    free((*rar).filename_save as *mut libc::c_void);
    free((*rar).dbo as *mut libc::c_void);
    free((*rar).unp_buffer as *mut libc::c_void);
    free((*rar).lzss.window as *mut libc::c_void);
    __archive_ppmd7_functions
        .Ppmd7_Free
        .expect("non-null function pointer")(&mut (*rar).ppmd7_context);
    free(rar as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/* Support functions */
unsafe extern "C" fn read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut head_type: libc::c_char,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endp: *const libc::c_char = 0 as *const libc::c_char;
    let mut rar: *mut rar = 0 as *mut rar;
    let mut rar_header: rar_header = rar_header {
        crc: [0; 2],
        type_0: 0,
        flags: [0; 2],
        size: [0; 2],
    };
    let mut file_header: rar_file_header = rar_file_header {
        pack_size: [0; 4],
        unp_size: [0; 4],
        host_os: 0,
        file_crc: [0; 4],
        file_time: [0; 4],
        unp_ver: 0,
        method: 0,
        name_size: [0; 2],
        file_attr: [0; 4],
    };
    let mut header_size: int64_t = 0;
    let mut filename_size: libc::c_uint = 0;
    let mut end: libc::c_uint = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut strp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut packed_size: [libc::c_char; 8] = [0; 8];
    let mut unp_size: [libc::c_char; 8] = [0; 8];
    let mut ttime: libc::c_int = 0;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut fn_sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut crc32_val: libc::c_ulong = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ret2: libc::c_int = 0;
    rar = (*(*a).format).data as *mut rar;
    /* Setup a string conversion object for non-rar-unicode filenames. */
    sconv = (*rar).opt_sconv;
    if sconv.is_null() {
        if (*rar).init_default_conversion == 0 {
            (*rar).sconv_default = archive_string_default_conversion_for_read(&mut (*a).archive);
            (*rar).init_default_conversion = 1 as libc::c_int
        }
        sconv = (*rar).sconv_default
    }
    h = __archive_read_ahead(a, 7 as libc::c_int as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    p = h as *const libc::c_char;
    memcpy(
        &mut rar_header as *mut rar_header as *mut libc::c_void,
        p as *const libc::c_void,
        ::std::mem::size_of::<rar_header>() as libc::c_ulong,
    );
    (*rar).file_flags =
        archive_le16dec(rar_header.flags.as_mut_ptr() as *const libc::c_void) as libc::c_uint;
    header_size = archive_le16dec(rar_header.size.as_mut_ptr() as *const libc::c_void) as int64_t;
    if header_size
        < ::std::mem::size_of::<rar_file_header>() as libc::c_ulong as int64_t
            + 7 as libc::c_int as libc::c_long
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid header size\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    crc32_val = crc32(
        0 as libc::c_int as uLong,
        (p as *const libc::c_uchar).offset(2 as libc::c_int as isize),
        (7 as libc::c_int - 2 as libc::c_int) as uInt,
    );
    __archive_read_consume(a, 7 as libc::c_int as int64_t);
    if (*rar).file_flags & FHD_SOLID as libc::c_uint == 0 {
        (*rar).compression_method = 0 as libc::c_int as libc::c_char;
        (*rar).packed_size = 0 as libc::c_int as int64_t;
        (*rar).unp_size = 0 as libc::c_int as int64_t;
        (*rar).mtime = 0 as libc::c_int as time_t;
        (*rar).ctime = 0 as libc::c_int as time_t;
        (*rar).atime = 0 as libc::c_int as time_t;
        (*rar).arctime = 0 as libc::c_int as time_t;
        (*rar).mode = 0 as libc::c_int as mode_t;
        memset(
            &mut (*rar).salt as *mut [libc::c_char; 8] as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
        (*rar).atime = 0 as libc::c_int as time_t;
        (*rar).ansec = 0 as libc::c_int as libc::c_long;
        (*rar).ctime = 0 as libc::c_int as time_t;
        (*rar).cnsec = 0 as libc::c_int as libc::c_long;
        (*rar).mtime = 0 as libc::c_int as time_t;
        (*rar).mnsec = 0 as libc::c_int as libc::c_long;
        (*rar).arctime = 0 as libc::c_int as time_t;
        (*rar).arcnsec = 0 as libc::c_int as libc::c_long
    } else {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"RAR solid archive support unavailable.\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    h = __archive_read_ahead(
        a,
        (header_size as size_t).wrapping_sub(7 as libc::c_int as libc::c_ulong),
        NULL as *mut ssize_t,
    );
    if h == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    /* File Header CRC check. */
    crc32_val = crc32(
        crc32_val,
        h as *const Bytef,
        (header_size - 7 as libc::c_int as libc::c_long) as libc::c_uint,
    );
    if crc32_val & 0xffff as libc::c_int as libc::c_ulong
        != archive_le16dec(rar_header.crc.as_mut_ptr() as *const libc::c_void) as libc::c_ulong
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Header CRC error\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* If no CRC error, Go on parsing File Header. */
    p = h as *const libc::c_char;
    endp = p
        .offset(header_size as isize)
        .offset(-(7 as libc::c_int as isize));
    memcpy(
        &mut file_header as *mut rar_file_header as *mut libc::c_void,
        p as *const libc::c_void,
        ::std::mem::size_of::<rar_file_header>() as libc::c_ulong,
    );
    p = p.offset(::std::mem::size_of::<rar_file_header>() as libc::c_ulong as isize);
    (*rar).compression_method = file_header.method;
    ttime =
        archive_le32dec(file_header.file_time.as_mut_ptr() as *const libc::c_void) as libc::c_int;
    (*rar).mtime = get_time(ttime);
    (*rar).file_crc =
        archive_le32dec(file_header.file_crc.as_mut_ptr() as *const libc::c_void) as libc::c_ulong;
    if (*rar).file_flags & FHD_PASSWORD as libc::c_uint != 0 {
        archive_entry_set_is_data_encrypted(entry, 1 as libc::c_int as libc::c_char);
        (*rar).has_encrypted_entries = 1 as libc::c_int;
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"RAR encryption support unavailable.\x00" as *const u8 as *const libc::c_char,
        );
        /* Since it is only the data part itself that is encrypted we can at least
        extract information about the currently processed entry and don't need
        to return ARCHIVE_FATAL here. */
        /*return (ARCHIVE_FATAL);*/
    } /* High pack size */
    if (*rar).file_flags & FHD_LARGE as libc::c_uint != 0 {
        memcpy(
            packed_size.as_mut_ptr() as *mut libc::c_void,
            file_header.pack_size.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ); /* High unpack size */
        memcpy(
            packed_size.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        p = p.offset(4 as libc::c_int as isize);
        memcpy(
            unp_size.as_mut_ptr() as *mut libc::c_void,
            file_header.unp_size.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            unp_size.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        p = p.offset(4 as libc::c_int as isize);
        (*rar).packed_size =
            archive_le64dec(&mut packed_size as *mut [libc::c_char; 8] as *const libc::c_void)
                as int64_t;
        (*rar).unp_size =
            archive_le64dec(&mut unp_size as *mut [libc::c_char; 8] as *const libc::c_void)
                as int64_t
    } else {
        (*rar).packed_size =
            archive_le32dec(file_header.pack_size.as_mut_ptr() as *const libc::c_void) as int64_t;
        (*rar).unp_size =
            archive_le32dec(file_header.unp_size.as_mut_ptr() as *const libc::c_void) as int64_t
    }
    if (*rar).packed_size < 0 as libc::c_int as libc::c_long
        || (*rar).unp_size < 0 as libc::c_int as libc::c_long
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid sizes specified.\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*rar).bytes_remaining = (*rar).packed_size;
    /* TODO: RARv3 subblocks contain comments. For now the complete block is
     * consumed at the end.
     */
    if head_type as libc::c_int == NEWSUB_HEAD {
        let mut distance: size_t =
            p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
        header_size += (*rar).packed_size;
        /* Make sure we have the extended data. */
        h = __archive_read_ahead(
            a,
            (header_size as size_t).wrapping_sub(7 as libc::c_int as libc::c_ulong),
            NULL as *mut ssize_t,
        );
        if h == NULL as *const libc::c_void {
            return -(30 as libc::c_int);
        }
        p = h as *const libc::c_char;
        endp = p
            .offset(header_size as isize)
            .offset(-(7 as libc::c_int as isize));
        p = p.offset(distance as isize)
    }
    filename_size =
        archive_le16dec(file_header.name_size.as_mut_ptr() as *const libc::c_void) as libc::c_uint;
    if p.offset(filename_size as isize) > endp {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid filename size\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*rar).filename_allocated
        < filename_size
            .wrapping_mul(2 as libc::c_int as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong
    {
        let mut newptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut newsize: size_t = filename_size
            .wrapping_mul(2 as libc::c_int as libc::c_uint)
            .wrapping_add(2 as libc::c_int as libc::c_uint)
            as size_t;
        newptr = realloc((*rar).filename as *mut libc::c_void, newsize) as *mut libc::c_char;
        if newptr.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Couldn\'t allocate memory.\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*rar).filename = newptr;
        (*rar).filename_allocated = newsize
    }
    filename = (*rar).filename;
    memcpy(
        filename as *mut libc::c_void,
        p as *const libc::c_void,
        filename_size as libc::c_ulong,
    );
    *filename.offset(filename_size as isize) = '\u{0}' as i32 as libc::c_char;
    if (*rar).file_flags & FHD_UNICODE as libc::c_uint != 0 {
        if filename_size as libc::c_ulong != strlen(filename) {
            let mut highbyte: libc::c_uchar = 0;
            let mut flagbits: libc::c_uchar = 0;
            let mut flagbyte: libc::c_uchar = 0;
            let mut fn_end: libc::c_uint = 0;
            let mut offset: libc::c_uint = 0;
            end = filename_size;
            fn_end = filename_size.wrapping_mul(2 as libc::c_int as libc::c_uint);
            filename_size = 0 as libc::c_int as libc::c_uint;
            offset =
                (strlen(filename) as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint);
            let fresh1 = offset;
            offset = offset.wrapping_add(1);
            highbyte = *p.offset(fresh1 as isize) as libc::c_uchar;
            flagbits = 0 as libc::c_int as libc::c_uchar;
            flagbyte = 0 as libc::c_int as libc::c_uchar;
            while offset < end && filename_size < fn_end {
                if flagbits == 0 {
                    let fresh2 = offset;
                    offset = offset.wrapping_add(1);
                    flagbyte = *p.offset(fresh2 as isize) as libc::c_uchar;
                    flagbits = 8 as libc::c_int as libc::c_uchar
                }
                flagbits = (flagbits as libc::c_int - 2 as libc::c_int) as libc::c_uchar;
                match flagbyte as libc::c_int >> flagbits as libc::c_int & 3 as libc::c_int {
                    0 => {
                        let fresh3 = filename_size;
                        filename_size = filename_size.wrapping_add(1);
                        *filename.offset(fresh3 as isize) = '\u{0}' as i32 as libc::c_char;
                        let fresh4 = offset;
                        offset = offset.wrapping_add(1);
                        let fresh5 = filename_size;
                        filename_size = filename_size.wrapping_add(1);
                        *filename.offset(fresh5 as isize) = *p.offset(fresh4 as isize)
                    }
                    1 => {
                        let fresh6 = filename_size;
                        filename_size = filename_size.wrapping_add(1);
                        *filename.offset(fresh6 as isize) = highbyte as libc::c_char;
                        let fresh7 = offset;
                        offset = offset.wrapping_add(1);
                        let fresh8 = filename_size;
                        filename_size = filename_size.wrapping_add(1);
                        *filename.offset(fresh8 as isize) = *p.offset(fresh7 as isize)
                    }
                    2 => {
                        let fresh9 = filename_size;
                        filename_size = filename_size.wrapping_add(1);
                        *filename.offset(fresh9 as isize) =
                            *p.offset(offset as isize).offset(1 as libc::c_int as isize);
                        let fresh10 = filename_size;
                        filename_size = filename_size.wrapping_add(1);
                        *filename.offset(fresh10 as isize) = *p.offset(offset as isize);
                        offset = offset.wrapping_add(2 as libc::c_int as libc::c_uint)
                    }
                    3 => {
                        let mut extra: libc::c_char = 0;
                        let mut high: libc::c_char = 0;
                        let fresh11 = offset;
                        offset = offset.wrapping_add(1);
                        let mut length: uint8_t = *p.offset(fresh11 as isize) as uint8_t;
                        if length as libc::c_int & 0x80 as libc::c_int != 0 {
                            let fresh12 = offset;
                            offset = offset.wrapping_add(1);
                            extra = *p.offset(fresh12 as isize);
                            high = highbyte as libc::c_char
                        } else {
                            high = 0 as libc::c_int as libc::c_char;
                            extra = high
                        }
                        length = ((length as libc::c_int & 0x7f as libc::c_int) + 2 as libc::c_int)
                            as uint8_t;
                        while length as libc::c_int != 0 && filename_size < fn_end {
                            let mut cp: libc::c_uint = filename_size >> 1 as libc::c_int;
                            let fresh13 = filename_size;
                            filename_size = filename_size.wrapping_add(1);
                            *filename.offset(fresh13 as isize) = high;
                            let fresh14 = filename_size;
                            filename_size = filename_size.wrapping_add(1);
                            *filename.offset(fresh14 as isize) =
                                (*p.offset(cp as isize) as libc::c_int + extra as libc::c_int)
                                    as libc::c_char;
                            length = length.wrapping_sub(1)
                        }
                    }
                    _ => {}
                }
            }
            if filename_size > fn_end {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Invalid filename\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            let fresh15 = filename_size;
            filename_size = filename_size.wrapping_add(1);
            *filename.offset(fresh15 as isize) = '\u{0}' as i32 as libc::c_char;
            /*
             * Do not increment filename_size here as the computations below
             * add the space for the terminating NUL explicitly.
             */
            *filename.offset(filename_size as isize) = '\u{0}' as i32 as libc::c_char;
            /* Decoded unicode form is UTF-16BE, so we have to update a string
             * conversion object for it. */
            if (*rar).sconv_utf16be.is_null() {
                (*rar).sconv_utf16be = archive_string_conversion_from_charset(
                    &mut (*a).archive,
                    b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
                if (*rar).sconv_utf16be.is_null() {
                    return -(30 as libc::c_int);
                }
            }
            fn_sconv = (*rar).sconv_utf16be;
            strp = filename;
            while memcmp(
                strp as *const libc::c_void,
                b"\x00\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) != 0
            {
                if memcmp(
                    strp as *const libc::c_void,
                    b"\x00\\\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    *strp.offset(1 as libc::c_int as isize) = '/' as i32 as libc::c_char
                }
                strp = strp.offset(2 as libc::c_int as isize)
            }
            p = p.offset(offset as isize)
        } else {
            /*
             * If FHD_UNICODE is set but no unicode data, this file name form
             * is UTF-8, so we have to update a string conversion object for
             * it accordingly.
             */
            if (*rar).sconv_utf8.is_null() {
                (*rar).sconv_utf8 = archive_string_conversion_from_charset(
                    &mut (*a).archive,
                    b"UTF-8\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
                if (*rar).sconv_utf8.is_null() {
                    return -(30 as libc::c_int);
                }
            }
            fn_sconv = (*rar).sconv_utf8;
            loop {
                strp = strchr(filename, '\\' as i32);
                if strp.is_null() {
                    break;
                }
                *strp = '/' as i32 as libc::c_char
            }
            p = p.offset(filename_size as isize)
        }
    } else {
        fn_sconv = sconv;
        loop {
            strp = strchr(filename, '\\' as i32);
            if strp.is_null() {
                break;
            }
            *strp = '/' as i32 as libc::c_char
        }
        p = p.offset(filename_size as isize)
    }
    /* Split file in multivolume RAR. No more need to process header. */
    if !(*rar).filename_save.is_null()
        && filename_size as libc::c_ulong == (*rar).filename_save_size
        && memcmp(
            (*rar).filename as *const libc::c_void,
            (*rar).filename_save as *const libc::c_void,
            filename_size.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        ) == 0
    {
        __archive_read_consume(a, header_size - 7 as libc::c_int as libc::c_long);
        (*rar).cursor = (*rar).cursor.wrapping_add(1);
        if (*rar).cursor >= (*rar).nodes {
            (*rar).nodes = (*rar).nodes.wrapping_add(1);
            (*rar).dbo = realloc(
                (*rar).dbo as *mut libc::c_void,
                (::std::mem::size_of::<data_block_offsets>() as libc::c_ulong)
                    .wrapping_mul((*rar).nodes as libc::c_ulong),
            ) as *mut data_block_offsets;
            if (*rar).dbo.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Couldn\'t allocate memory.\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            (*(*rar).dbo.offset((*rar).cursor as isize)).header_size = header_size;
            (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset =
                -(1 as libc::c_int) as int64_t;
            (*(*rar).dbo.offset((*rar).cursor as isize)).end_offset = -(1 as libc::c_int) as int64_t
        }
        if (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset
            < 0 as libc::c_int as libc::c_long
        {
            (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset = (*(*a).filter).position;
            (*(*rar).dbo.offset((*rar).cursor as isize)).end_offset =
                (*(*rar).dbo.offset((*rar).cursor as isize)).start_offset + (*rar).packed_size
        }
        return ret;
    } else {
        if (*rar).filename_must_match != 0 {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Mismatch of file parts split across multi-volume archive\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    (*rar).filename_save = realloc(
        (*rar).filename_save as *mut libc::c_void,
        filename_size.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut libc::c_char;
    memcpy(
        (*rar).filename_save as *mut libc::c_void,
        (*rar).filename as *const libc::c_void,
        filename_size.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    (*rar).filename_save_size = filename_size as size_t;
    /* Set info for seeking */
    free((*rar).dbo as *mut libc::c_void);
    (*rar).dbo = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<data_block_offsets>() as libc::c_ulong,
    ) as *mut data_block_offsets;
    if (*rar).dbo.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Couldn\'t allocate memory.\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*(*rar).dbo.offset(0 as libc::c_int as isize)).header_size = header_size;
    (*(*rar).dbo.offset(0 as libc::c_int as isize)).start_offset = -(1 as libc::c_int) as int64_t;
    (*(*rar).dbo.offset(0 as libc::c_int as isize)).end_offset = -(1 as libc::c_int) as int64_t;
    (*rar).cursor = 0 as libc::c_int as libc::c_uint;
    (*rar).nodes = 1 as libc::c_int as libc::c_uint;
    if (*rar).file_flags & FHD_SALT as libc::c_uint != 0 {
        if p.offset(8 as libc::c_int as isize) > endp {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Invalid header size\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        memcpy(
            (*rar).salt.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        p = p.offset(8 as libc::c_int as isize)
    }
    if (*rar).file_flags & FHD_EXTTIME as libc::c_uint != 0 {
        if read_exttime(p, rar, endp) < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Invalid header size\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    __archive_read_consume(a, header_size - 7 as libc::c_int as libc::c_long);
    (*(*rar).dbo.offset(0 as libc::c_int as isize)).start_offset = (*(*a).filter).position;
    (*(*rar).dbo.offset(0 as libc::c_int as isize)).end_offset =
        (*(*rar).dbo.offset(0 as libc::c_int as isize)).start_offset + (*rar).packed_size;
    match file_header.host_os as libc::c_int {
        OS_MSDOS | OS_OS2 | OS_WIN32 => {
            (*rar).mode =
                archive_le32dec(file_header.file_attr.as_mut_ptr() as *const libc::c_void);
            if (*rar).mode & FILE_ATTRIBUTE_DIRECTORY as libc::c_uint != 0 {
                (*rar).mode = AE_IFDIR as mode_t
                    | S_IXUSR as libc::c_uint
                    | S_IXGRP as libc::c_uint
                    | S_IXOTH as libc::c_uint
            } else {
                (*rar).mode = AE_IFREG as mode_t
            }
            (*rar).mode |= (S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) as libc::c_uint
        }
        OS_UNIX | OS_MAC_OS | OS_BEOS => {
            (*rar).mode = archive_le32dec(file_header.file_attr.as_mut_ptr() as *const libc::c_void)
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Unknown file attributes from RAR file\'s host OS\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    (*rar).bytes_unconsumed = 0 as libc::c_int as int64_t;
    (*rar).bytes_uncopied = (*rar).bytes_unconsumed;
    (*rar).offset = 0 as libc::c_int as int64_t;
    (*rar).lzss.position = (*rar).offset;
    (*rar).offset_seek = 0 as libc::c_int as int64_t;
    (*rar).dictionary_size = 0 as libc::c_int as libc::c_uint;
    (*rar).offset_outgoing = 0 as libc::c_int as int64_t;
    (*rar).br.cache_avail = 0 as libc::c_int;
    (*rar).br.avail_in = 0 as libc::c_int as ssize_t;
    (*rar).crc_calculated = 0 as libc::c_int as libc::c_ulong;
    (*rar).entry_eof = 0 as libc::c_int as libc::c_char;
    (*rar).valid = 1 as libc::c_int as libc::c_char;
    (*rar).is_ppmd_block = 0 as libc::c_int as libc::c_char;
    (*rar).start_new_table = 1 as libc::c_int as libc::c_char;
    free((*rar).unp_buffer as *mut libc::c_void);
    (*rar).unp_buffer = NULL as *mut libc::c_uchar;
    (*rar).unp_offset = 0 as libc::c_int as libc::c_uint;
    (*rar).unp_buffer_size = UNP_BUFFER_SIZE as libc::c_uint;
    memset(
        (*rar).lengthtable.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 404]>() as libc::c_ulong,
    );
    __archive_ppmd7_functions
        .Ppmd7_Free
        .expect("non-null function pointer")(&mut (*rar).ppmd7_context);
    (*rar).ppmd_eod = 0 as libc::c_int as libc::c_char;
    (*rar).ppmd_valid = (*rar).ppmd_eod;
    /* Don't set any archive entries for non-file header types */
    if head_type as libc::c_int == NEWSUB_HEAD {
        return ret;
    }
    archive_entry_set_mtime(entry, (*rar).mtime, (*rar).mnsec);
    archive_entry_set_ctime(entry, (*rar).ctime, (*rar).cnsec);
    archive_entry_set_atime(entry, (*rar).atime, (*rar).ansec);
    archive_entry_set_size(entry, (*rar).unp_size);
    archive_entry_set_mode(entry, (*rar).mode);
    if _archive_entry_copy_pathname_l(entry, filename, filename_size as size_t, fn_sconv) != 0 {
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
            archive_string_conversion_charset_name(fn_sconv),
        );
        ret = -(20 as libc::c_int)
    }
    if (*rar).mode & AE_IFMT as mode_t == AE_IFLNK as mode_t {
        /* Make sure a symbolic-link file does not have its body. */
        (*rar).bytes_remaining = 0 as libc::c_int as int64_t;
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
        /* Read a symbolic-link name. */
        ret2 = read_symlink_stored(a, entry, sconv);
        if ret2 < -(20 as libc::c_int) {
            return ret2;
        }
        if ret > ret2 {
            ret = ret2
        }
    }
    if (*rar).bytes_remaining == 0 as libc::c_int as libc::c_long {
        (*rar).entry_eof = 1 as libc::c_int as libc::c_char
    }
    return ret;
}
unsafe extern "C" fn get_time(mut ttime: libc::c_int) -> time_t {
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
    tm.tm_sec = 2 as libc::c_int * (ttime & 0x1f as libc::c_int);
    tm.tm_min = ttime >> 5 as libc::c_int & 0x3f as libc::c_int;
    tm.tm_hour = ttime >> 11 as libc::c_int & 0x1f as libc::c_int;
    tm.tm_mday = ttime >> 16 as libc::c_int & 0x1f as libc::c_int;
    tm.tm_mon = (ttime >> 21 as libc::c_int & 0xf as libc::c_int) - 1 as libc::c_int;
    tm.tm_year = (ttime >> 25 as libc::c_int & 0x7f as libc::c_int) + 80 as libc::c_int;
    tm.tm_isdst = -(1 as libc::c_int);
    return mktime(&mut tm);
}
unsafe extern "C" fn read_exttime(
    mut p: *const libc::c_char,
    mut rar: *mut rar,
    mut endp: *const libc::c_char,
) -> libc::c_int {
    let mut rmode: libc::c_uint = 0;
    let mut flags: libc::c_uint = 0;
    let mut rem: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    let mut ttime: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut t: time_t = 0;
    let mut nsec: libc::c_long = 0;
    let mut tmbuf: tm = tm {
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
    if p.offset(2 as libc::c_int as isize) > endp {
        return -(1 as libc::c_int);
    }
    flags = archive_le16dec(p as *const libc::c_void) as libc::c_uint;
    p = p.offset(2 as libc::c_int as isize);
    i = 3 as libc::c_int;
    while i >= 0 as libc::c_int {
        t = 0 as libc::c_int as time_t;
        if i == 3 as libc::c_int {
            t = (*rar).mtime
        }
        rmode = flags >> i * 4 as libc::c_int;
        if rmode & 8 as libc::c_int as libc::c_uint != 0 {
            if t == 0 {
                if p.offset(4 as libc::c_int as isize) > endp {
                    return -(1 as libc::c_int);
                }
                ttime = archive_le32dec(p as *const libc::c_void) as libc::c_int;
                t = get_time(ttime);
                p = p.offset(4 as libc::c_int as isize)
            }
            rem = 0 as libc::c_int as libc::c_uint;
            count = rmode & 3 as libc::c_int as libc::c_uint;
            if p.offset(count as isize) > endp {
                return -(1 as libc::c_int);
            }
            j = 0 as libc::c_int as libc::c_uint;
            while j < count {
                rem = (*p as libc::c_uchar as libc::c_uint) << 16 as libc::c_int
                    | rem >> 8 as libc::c_int;
                p = p.offset(1);
                j = j.wrapping_add(1)
            }
            tm = localtime_r(&mut t, &mut tmbuf);
            nsec = ((*tm).tm_sec as libc::c_uint)
                .wrapping_add(rem.wrapping_div(NS_UNIT as libc::c_uint))
                as libc::c_long;
            if rmode & 4 as libc::c_int as libc::c_uint != 0 {
                (*tm).tm_sec += 1;
                t = mktime(tm)
            }
            if i == 3 as libc::c_int {
                (*rar).mtime = t;
                (*rar).mnsec = nsec
            } else if i == 2 as libc::c_int {
                (*rar).ctime = t;
                (*rar).cnsec = nsec
            } else if i == 1 as libc::c_int {
                (*rar).atime = t;
                (*rar).ansec = nsec
            } else {
                (*rar).arctime = t;
                (*rar).arcnsec = nsec
            }
        }
        i -= 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_symlink_stored(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut sconv: *mut archive_string_conv,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut rar: *mut rar = 0 as *mut rar;
    let mut ret: libc::c_int = 0 as libc::c_int;
    rar = (*(*a).format).data as *mut rar;
    h = rar_read_ahead(a, (*rar).packed_size as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    p = h as *const libc::c_char;
    if _archive_entry_copy_symlink_l(entry, p, (*rar).packed_size as size_t, sconv) != 0 {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for link\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"link cannot be converted from %s to current locale.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name(sconv),
        );
        ret = -(20 as libc::c_int)
    }
    __archive_read_consume(a, (*rar).packed_size);
    return ret;
}
unsafe extern "C" fn read_data_stored(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut rar: *mut rar = 0 as *mut rar;
    let mut bytes_avail: ssize_t = 0;
    rar = (*(*a).format).data as *mut rar;
    if (*rar).bytes_remaining == 0 as libc::c_int as libc::c_long
        && !((*rar).main_flags & MHD_VOLUME as libc::c_uint != 0
            && (*rar).file_flags & FHD_SPLIT_AFTER as libc::c_uint != 0)
    {
        *buff = NULL as *const libc::c_void;
        *size = 0 as libc::c_int as size_t;
        *offset = (*rar).offset;
        if (*rar).file_crc != (*rar).crc_calculated {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"File CRC error\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*rar).entry_eof = 1 as libc::c_int as libc::c_char;
        return 1 as libc::c_int;
    }
    *buff = rar_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_avail);
    if bytes_avail <= 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    *size = bytes_avail as size_t;
    *offset = (*rar).offset;
    (*rar).offset += bytes_avail;
    (*rar).offset_seek += bytes_avail;
    (*rar).bytes_remaining -= bytes_avail;
    (*rar).bytes_unconsumed = bytes_avail;
    /* Calculate File CRC. */
    (*rar).crc_calculated = crc32(
        (*rar).crc_calculated,
        *buff as *const Bytef,
        bytes_avail as libc::c_uint,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_data_compressed(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut rar: *mut rar = 0 as *mut rar;
    let mut start: int64_t = 0;
    let mut end: int64_t = 0;
    let mut actualend: int64_t = 0;
    let mut bs: size_t = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut sym: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut lzss_offset: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    rar = (*(*a).format).data as *mut rar;
    let mut current_block_100: u64;
    loop {
        if (*rar).valid == 0 {
            return -(30 as libc::c_int);
        }
        if (*rar).ppmd_eod as libc::c_int != 0
            || (*rar).dictionary_size != 0 && (*rar).offset >= (*rar).unp_size
        {
            if (*rar).unp_offset > 0 as libc::c_int as libc::c_uint {
                /*
                 * If *buff is NULL, it means unp_buffer is not full.
                 * So we have to continue extracting a RAR file.
                 */
                /*
                 * We have unprocessed extracted data. write it out.
                 */
                *buff = (*rar).unp_buffer as *const libc::c_void;
                *size = (*rar).unp_offset as size_t;
                *offset = (*rar).offset_outgoing;
                (*rar).offset_outgoing = ((*rar).offset_outgoing as libc::c_ulong)
                    .wrapping_add(*size) as int64_t
                    as int64_t;
                /* Calculate File CRC. */
                (*rar).crc_calculated = crc32(
                    (*rar).crc_calculated,
                    *buff as *const Bytef,
                    *size as libc::c_uint,
                );
                (*rar).unp_offset = 0 as libc::c_int as libc::c_uint;
                return 0 as libc::c_int;
            }
            *buff = NULL as *const libc::c_void;
            *size = 0 as libc::c_int as size_t;
            *offset = (*rar).offset;
            if (*rar).file_crc != (*rar).crc_calculated {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"File CRC error\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            (*rar).entry_eof = 1 as libc::c_int as libc::c_char;
            return 1 as libc::c_int;
        }
        if (*rar).is_ppmd_block == 0
            && (*rar).dictionary_size != 0
            && (*rar).bytes_uncopied > 0 as libc::c_int as libc::c_long
        {
            if (*rar).bytes_uncopied
                > (*rar).unp_buffer_size.wrapping_sub((*rar).unp_offset) as libc::c_long
            {
                bs = (*rar).unp_buffer_size.wrapping_sub((*rar).unp_offset) as size_t
            } else {
                bs = (*rar).bytes_uncopied as size_t
            }
            ret = copy_from_lzss_window(a, buff, (*rar).offset, bs as libc::c_int);
            if ret != ARCHIVE_OK {
                return ret;
            }
            (*rar).offset = ((*rar).offset as libc::c_ulong).wrapping_add(bs) as int64_t as int64_t;
            (*rar).bytes_uncopied =
                ((*rar).bytes_uncopied as libc::c_ulong).wrapping_sub(bs) as int64_t as int64_t;
            if *buff != NULL as *const libc::c_void {
                (*rar).unp_offset = 0 as libc::c_int as libc::c_uint;
                *size = (*rar).unp_buffer_size as size_t;
                *offset = (*rar).offset_outgoing;
                (*rar).offset_outgoing = ((*rar).offset_outgoing as libc::c_ulong)
                    .wrapping_add(*size) as int64_t
                    as int64_t;
                /* Calculate File CRC. */
                (*rar).crc_calculated = crc32(
                    (*rar).crc_calculated,
                    *buff as *const Bytef,
                    *size as libc::c_uint,
                ); /* End Of ppmd Data. */
                return ret;
            }
        } else {
            if (*rar).br.next_in.is_null() && {
                ret = rar_br_preparation(a, &mut (*rar).br);
                (ret) < ARCHIVE_WARN
            } {
                return ret;
            }
            if (*rar).start_new_table as libc::c_int != 0 && {
                ret = parse_codes(a);
                (ret) < -(20 as libc::c_int)
            } {
                return ret;
            }
            if (*rar).is_ppmd_block != 0 {
                sym = __archive_ppmd7_functions
                    .Ppmd7_DecodeSymbol
                    .expect("non-null function pointer")(
                    &mut (*rar).ppmd7_context,
                    &mut (*rar).range_dec.p,
                );
                if sym < 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Invalid symbol\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                if sym != (*rar).ppmd_escape {
                    lzss_emit_literal(rar, sym as uint8_t);
                    (*rar).bytes_uncopied += 1;
                    current_block_100 = 1852451392920375136;
                } else {
                    code = __archive_ppmd7_functions
                        .Ppmd7_DecodeSymbol
                        .expect("non-null function pointer")(
                        &mut (*rar).ppmd7_context,
                        &mut (*rar).range_dec.p,
                    );
                    if code < 0 as libc::c_int {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Invalid symbol\x00" as *const u8 as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    match code {
                        0 => {
                            (*rar).start_new_table = 1 as libc::c_int as libc::c_char;
                            return read_data_compressed(a, buff, size, offset);
                        }
                        2 => {
                            (*rar).ppmd_eod = 1 as libc::c_int as libc::c_char;
                            current_block_100 = 10680521327981672866;
                        }
                        3 => {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"Parsing filters is unsupported.\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            return -(25 as libc::c_int);
                        }
                        4 => {
                            lzss_offset = 0 as libc::c_int;
                            i = 2 as libc::c_int;
                            while i >= 0 as libc::c_int {
                                code = __archive_ppmd7_functions
                                    .Ppmd7_DecodeSymbol
                                    .expect("non-null function pointer")(
                                    &mut (*rar).ppmd7_context,
                                    &mut (*rar).range_dec.p,
                                );
                                if code < 0 as libc::c_int {
                                    archive_set_error(
                                        &mut (*a).archive as *mut archive,
                                        ARCHIVE_ERRNO_FILE_FORMAT,
                                        b"Invalid symbol\x00" as *const u8 as *const libc::c_char,
                                    );
                                    return -(30 as libc::c_int);
                                }
                                lzss_offset |= code << i * 8 as libc::c_int;
                                i -= 1
                            }
                            length = __archive_ppmd7_functions
                                .Ppmd7_DecodeSymbol
                                .expect("non-null function pointer")(
                                &mut (*rar).ppmd7_context,
                                &mut (*rar).range_dec.p,
                            );
                            if length < 0 as libc::c_int {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_FILE_FORMAT,
                                    b"Invalid symbol\x00" as *const u8 as *const libc::c_char,
                                );
                                return -(30 as libc::c_int);
                            }
                            lzss_emit_match(
                                rar,
                                lzss_offset + 2 as libc::c_int,
                                length + 32 as libc::c_int,
                            );
                            (*rar).bytes_uncopied += (length + 32 as libc::c_int) as libc::c_long;
                            current_block_100 = 1852451392920375136;
                        }
                        5 => {
                            length = __archive_ppmd7_functions
                                .Ppmd7_DecodeSymbol
                                .expect("non-null function pointer")(
                                &mut (*rar).ppmd7_context,
                                &mut (*rar).range_dec.p,
                            );
                            if length < 0 as libc::c_int {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_FILE_FORMAT,
                                    b"Invalid symbol\x00" as *const u8 as *const libc::c_char,
                                );
                                return -(30 as libc::c_int);
                            }
                            lzss_emit_match(rar, 1 as libc::c_int, length + 4 as libc::c_int);
                            (*rar).bytes_uncopied += (length + 4 as libc::c_int) as libc::c_long;
                            current_block_100 = 1852451392920375136;
                        }
                        _ => {
                            lzss_emit_literal(rar, sym as uint8_t);
                            (*rar).bytes_uncopied += 1;
                            current_block_100 = 1852451392920375136;
                        }
                    }
                }
            } else {
                start = (*rar).offset;
                end = start + (*rar).dictionary_size as libc::c_long;
                (*rar).filterstart = INT64_MAX;
                actualend = expand(a, end);
                if actualend < 0 as libc::c_int as libc::c_long {
                    return actualend as libc::c_int;
                }
                (*rar).bytes_uncopied = actualend - start;
                if (*rar).bytes_uncopied == 0 as libc::c_int as libc::c_long {
                    /* Broken RAR files cause this case.
                     * NOTE: If this case were possible on a normal RAR file
                     * we would find out where it was actually bad and
                     * what we would do to solve it. */
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_FILE_FORMAT,
                        b"Internal error extracting RAR file\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
                current_block_100 = 1852451392920375136;
            }
            match current_block_100 {
                10680521327981672866 => {}
                _ => {
                    if (*rar).bytes_uncopied
                        > (*rar).unp_buffer_size.wrapping_sub((*rar).unp_offset) as libc::c_long
                    {
                        bs = (*rar).unp_buffer_size.wrapping_sub((*rar).unp_offset) as size_t
                    } else {
                        bs = (*rar).bytes_uncopied as size_t
                    }
                    ret = copy_from_lzss_window(a, buff, (*rar).offset, bs as libc::c_int);
                    if ret != ARCHIVE_OK {
                        return ret;
                    }
                    (*rar).offset =
                        ((*rar).offset as libc::c_ulong).wrapping_add(bs) as int64_t as int64_t;
                    (*rar).bytes_uncopied = ((*rar).bytes_uncopied as libc::c_ulong)
                        .wrapping_sub(bs) as int64_t
                        as int64_t
                }
            }
        }
        if !(*buff == NULL as *const libc::c_void) {
            break;
        }
    }
    (*rar).unp_offset = 0 as libc::c_int as libc::c_uint;
    *size = (*rar).unp_buffer_size as size_t;
    *offset = (*rar).offset_outgoing;
    (*rar).offset_outgoing =
        ((*rar).offset_outgoing as libc::c_ulong).wrapping_add(*size) as int64_t as int64_t;
    /* Calculate File CRC. */
    (*rar).crc_calculated = crc32(
        (*rar).crc_calculated,
        *buff as *const Bytef,
        *size as libc::c_uint,
    );
    return ret;
}
unsafe extern "C" fn parse_codes(mut a: *mut archive_read) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut bitlengths: [libc::c_uchar; 20] = [0; 20];
    let mut zerocount: libc::c_uchar = 0;
    let mut ppmd_flags: libc::c_uchar = 0;
    let mut maxorder: libc::c_uint = 0;
    let mut precode: huffman_code = huffman_code {
        tree: 0 as *mut huffman_tree_node,
        numentries: 0,
        numallocatedentries: 0,
        minlength: 0,
        maxlength: 0,
        tablesize: 0,
        table: 0 as *mut huffman_table_entry,
    };
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    let mut br: *mut rar_br = &mut (*rar).br;
    free_codes(a);
    /* Skip to the next byte */
    (*br).cache_avail &= !(7 as libc::c_int);
    /* PPMd block flag */
    if (*br).cache_avail >= 1 as libc::c_int
        || rar_br_fillup(a, br) != 0
        || (*br).cache_avail >= 1 as libc::c_int
    {
        (*rar).is_ppmd_block =
            (((*br).cache_buffer >> (*br).cache_avail - 1 as libc::c_int) as uint32_t
                & cache_masks[1 as libc::c_int as usize]) as libc::c_char;
        if (*rar).is_ppmd_block as libc::c_int != 0 as libc::c_int {
            (*br).cache_avail -= 1 as libc::c_int;
            if !((*br).cache_avail >= 7 as libc::c_int
                || rar_br_fillup(a, br) != 0
                || (*br).cache_avail >= 7 as libc::c_int)
            {
                current_block = 10545432766654176713;
            } else {
                ppmd_flags = (((*br).cache_buffer >> (*br).cache_avail - 7 as libc::c_int)
                    as uint32_t
                    & cache_masks[7 as libc::c_int as usize])
                    as libc::c_uchar;
                (*br).cache_avail -= 7 as libc::c_int;
                /* Memory is allocated in MB */
                if ppmd_flags as libc::c_int & 0x20 as libc::c_int != 0 {
                    if !((*br).cache_avail >= 8 as libc::c_int
                        || rar_br_fillup(a, br) != 0
                        || (*br).cache_avail >= 8 as libc::c_int)
                    {
                        current_block = 10545432766654176713;
                    } else {
                        (*rar).dictionary_size = (((*br).cache_buffer
                            >> (*br).cache_avail - 8 as libc::c_int)
                            as uint32_t
                            & cache_masks[8 as libc::c_int as usize])
                            .wrapping_add(1 as libc::c_int as libc::c_uint)
                            << 20 as libc::c_int;
                        (*br).cache_avail -= 8 as libc::c_int;
                        current_block = 7976072742316086414;
                    }
                } else {
                    current_block = 7976072742316086414;
                }
                match current_block {
                    10545432766654176713 => {}
                    _ => {
                        if ppmd_flags as libc::c_int & 0x40 as libc::c_int != 0 {
                            if !((*br).cache_avail >= 8 as libc::c_int
                                || rar_br_fillup(a, br) != 0
                                || (*br).cache_avail >= 8 as libc::c_int)
                            {
                                current_block = 10545432766654176713;
                            } else {
                                (*rar).ppmd7_context.InitEsc = ((*br).cache_buffer
                                    >> (*br).cache_avail - 8 as libc::c_int)
                                    as uint32_t
                                    & cache_masks[8 as libc::c_int as usize];
                                (*rar).ppmd_escape = (*rar).ppmd7_context.InitEsc as libc::c_int;
                                (*br).cache_avail -= 8 as libc::c_int;
                                current_block = 26972500619410423;
                            }
                        } else {
                            (*rar).ppmd_escape = 2 as libc::c_int;
                            current_block = 26972500619410423;
                        }
                        match current_block {
                            10545432766654176713 => {}
                            _ => {
                                if ppmd_flags as libc::c_int & 0x20 as libc::c_int != 0 {
                                    maxorder = ((ppmd_flags as libc::c_int & 0x1f as libc::c_int)
                                        + 1 as libc::c_int)
                                        as libc::c_uint;
                                    if maxorder > 16 as libc::c_int as libc::c_uint {
                                        maxorder = (16 as libc::c_int as libc::c_uint).wrapping_add(
                                            maxorder
                                                .wrapping_sub(16 as libc::c_int as libc::c_uint)
                                                .wrapping_mul(3 as libc::c_int as libc::c_uint),
                                        )
                                    }
                                    if maxorder == 1 as libc::c_int as libc::c_uint {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_FILE_FORMAT,
                                            b"Truncated RAR file data\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int);
                                    }
                                    /* Make sure ppmd7_contest is freed before Ppmd7_Construct
                                     * because reading a broken file cause this abnormal sequence. */
                                    __archive_ppmd7_functions
                                        .Ppmd7_Free
                                        .expect("non-null function pointer")(
                                        &mut (*rar).ppmd7_context,
                                    );
                                    (*rar).bytein.a = a;
                                    (*rar).bytein.Read = Some(
                                        ppmd_read
                                            as unsafe extern "C" fn(_: *mut libc::c_void) -> Byte,
                                    );
                                    __archive_ppmd7_functions
                                        .PpmdRAR_RangeDec_CreateVTable
                                        .expect("non-null function pointer")(
                                        &mut (*rar).range_dec
                                    );
                                    (*rar).range_dec.Stream = &mut (*rar).bytein;
                                    __archive_ppmd7_functions
                                        .Ppmd7_Construct
                                        .expect("non-null function pointer")(
                                        &mut (*rar).ppmd7_context,
                                    );
                                    if (*rar).dictionary_size == 0 as libc::c_int as libc::c_uint {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_FILE_FORMAT,
                                            b"Invalid zero dictionary size\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int);
                                    }
                                    if __archive_ppmd7_functions
                                        .Ppmd7_Alloc
                                        .expect("non-null function pointer")(
                                        &mut (*rar).ppmd7_context,
                                        (*rar).dictionary_size,
                                    ) == 0
                                    {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ENOMEM,
                                            b"Out of memory\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int);
                                    }
                                    if __archive_ppmd7_functions
                                        .PpmdRAR_RangeDec_Init
                                        .expect("non-null function pointer")(
                                        &mut (*rar).range_dec
                                    ) == 0
                                    {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_FILE_FORMAT,
                                            b"Unable to initialize PPMd range decoder\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int);
                                    }
                                    __archive_ppmd7_functions
                                        .Ppmd7_Init
                                        .expect("non-null function pointer")(
                                        &mut (*rar).ppmd7_context,
                                        maxorder,
                                    );
                                    (*rar).ppmd_valid = 1 as libc::c_int as libc::c_char
                                } else {
                                    if (*rar).ppmd_valid == 0 {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_FILE_FORMAT,
                                            b"Invalid PPMd sequence\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int);
                                    }
                                    if __archive_ppmd7_functions
                                        .PpmdRAR_RangeDec_Init
                                        .expect("non-null function pointer")(
                                        &mut (*rar).range_dec
                                    ) == 0
                                    {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_FILE_FORMAT,
                                            b"Unable to initialize PPMd range decoder\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                        return -(30 as libc::c_int);
                                    }
                                }
                                current_block = 13174377073168946860;
                            }
                        }
                    }
                }
            }
        } else {
            (*br).cache_avail -= 1 as libc::c_int;
            /* Keep existing table flag */
            if !((*br).cache_avail >= 1 as libc::c_int
                || rar_br_fillup(a, br) != 0
                || (*br).cache_avail >= 1 as libc::c_int)
            {
                current_block = 10545432766654176713;
            } else {
                if ((*br).cache_buffer >> (*br).cache_avail - 1 as libc::c_int) as uint32_t
                    & cache_masks[1 as libc::c_int as usize]
                    == 0
                {
                    memset(
                        (*rar).lengthtable.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[libc::c_uchar; 404]>() as libc::c_ulong,
                    );
                }
                (*br).cache_avail -= 1 as libc::c_int;
                memset(
                    &mut bitlengths as *mut [libc::c_uchar; 20] as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
                );
                i = 0 as libc::c_int;
                loop {
                    if !(i < MAX_SYMBOLS) {
                        current_block = 2723324002591448311;
                        break;
                    }
                    if !((*br).cache_avail >= 4 as libc::c_int
                        || rar_br_fillup(a, br) != 0
                        || (*br).cache_avail >= 4 as libc::c_int)
                    {
                        current_block = 10545432766654176713;
                        break;
                    }
                    let fresh16 = i;
                    i = i + 1;
                    bitlengths[fresh16 as usize] =
                        (((*br).cache_buffer >> (*br).cache_avail - 4 as libc::c_int) as uint32_t
                            & cache_masks[4 as libc::c_int as usize])
                            as libc::c_uchar;
                    (*br).cache_avail -= 4 as libc::c_int;
                    if !(bitlengths[(i - 1 as libc::c_int) as usize] as libc::c_int
                        == 0xf as libc::c_int)
                    {
                        continue;
                    }
                    if !((*br).cache_avail >= 4 as libc::c_int
                        || rar_br_fillup(a, br) != 0
                        || (*br).cache_avail >= 4 as libc::c_int)
                    {
                        current_block = 10545432766654176713;
                        break;
                    }
                    zerocount = (((*br).cache_buffer >> (*br).cache_avail - 4 as libc::c_int)
                        as uint32_t
                        & cache_masks[4 as libc::c_int as usize])
                        as libc::c_uchar;
                    (*br).cache_avail -= 4 as libc::c_int;
                    if zerocount != 0 {
                        i -= 1;
                        j = 0 as libc::c_int;
                        while j < zerocount as libc::c_int + 2 as libc::c_int && i < MAX_SYMBOLS {
                            let fresh17 = i;
                            i = i + 1;
                            bitlengths[fresh17 as usize] = 0 as libc::c_int as libc::c_uchar;
                            j += 1
                        }
                    }
                }
                match current_block {
                    10545432766654176713 => {}
                    _ => {
                        memset(
                            &mut precode as *mut huffman_code as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<huffman_code>() as libc::c_ulong,
                        );
                        r = create_code(
                            a,
                            &mut precode,
                            bitlengths.as_mut_ptr(),
                            MAX_SYMBOLS,
                            MAX_SYMBOL_LENGTH as libc::c_char,
                        );
                        if r != ARCHIVE_OK {
                            free(precode.tree as *mut libc::c_void);
                            free(precode.table as *mut libc::c_void);
                            return r;
                        }
                        i = 0 as libc::c_int;
                        loop {
                            if !(i < MAINCODE_SIZE
                                + OFFSETCODE_SIZE
                                + LOWOFFSETCODE_SIZE
                                + LENGTHCODE_SIZE)
                            {
                                current_block = 13256895345714485905;
                                break;
                            }
                            val = read_next_symbol(a, &mut precode);
                            if val < 0 as libc::c_int {
                                free(precode.tree as *mut libc::c_void);
                                free(precode.table as *mut libc::c_void);
                                return -(30 as libc::c_int);
                            }
                            if val < 16 as libc::c_int {
                                (*rar).lengthtable[i as usize] =
                                    ((*rar).lengthtable[i as usize] as libc::c_int + val
                                        & 0xf as libc::c_int)
                                        as libc::c_uchar;
                                i += 1
                            } else if val < 18 as libc::c_int {
                                if i == 0 as libc::c_int {
                                    free(precode.tree as *mut libc::c_void);
                                    free(precode.table as *mut libc::c_void);
                                    archive_set_error(
                                        &mut (*a).archive as *mut archive,
                                        ARCHIVE_ERRNO_FILE_FORMAT,
                                        b"Internal error extracting RAR file.\x00" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return -(30 as libc::c_int);
                                }
                                if val == 16 as libc::c_int {
                                    if !((*br).cache_avail >= 3 as libc::c_int
                                        || rar_br_fillup(a, br) != 0
                                        || (*br).cache_avail >= 3 as libc::c_int)
                                    {
                                        free(precode.tree as *mut libc::c_void);
                                        free(precode.table as *mut libc::c_void);
                                        current_block = 10545432766654176713;
                                        break;
                                    } else {
                                        n = (((*br).cache_buffer
                                            >> (*br).cache_avail - 3 as libc::c_int)
                                            as uint32_t
                                            & cache_masks[3 as libc::c_int as usize])
                                            .wrapping_add(3 as libc::c_int as libc::c_uint)
                                            as libc::c_int;
                                        (*br).cache_avail -= 3 as libc::c_int
                                    }
                                } else if !((*br).cache_avail >= 7 as libc::c_int
                                    || rar_br_fillup(a, br) != 0
                                    || (*br).cache_avail >= 7 as libc::c_int)
                                {
                                    free(precode.tree as *mut libc::c_void);
                                    free(precode.table as *mut libc::c_void);
                                    current_block = 10545432766654176713;
                                    break;
                                } else {
                                    n = (((*br).cache_buffer
                                        >> (*br).cache_avail - 7 as libc::c_int)
                                        as uint32_t
                                        & cache_masks[7 as libc::c_int as usize])
                                        .wrapping_add(11 as libc::c_int as libc::c_uint)
                                        as libc::c_int;
                                    (*br).cache_avail -= 7 as libc::c_int
                                }
                                j = 0 as libc::c_int;
                                while j < n
                                    && i < MAINCODE_SIZE
                                        + OFFSETCODE_SIZE
                                        + LOWOFFSETCODE_SIZE
                                        + LENGTHCODE_SIZE
                                {
                                    (*rar).lengthtable[i as usize] =
                                        (*rar).lengthtable[(i - 1 as libc::c_int) as usize];
                                    i += 1;
                                    j += 1
                                }
                            } else {
                                if val == 18 as libc::c_int {
                                    if !((*br).cache_avail >= 3 as libc::c_int
                                        || rar_br_fillup(a, br) != 0
                                        || (*br).cache_avail >= 3 as libc::c_int)
                                    {
                                        free(precode.tree as *mut libc::c_void);
                                        free(precode.table as *mut libc::c_void);
                                        current_block = 10545432766654176713;
                                        break;
                                    } else {
                                        n = (((*br).cache_buffer
                                            >> (*br).cache_avail - 3 as libc::c_int)
                                            as uint32_t
                                            & cache_masks[3 as libc::c_int as usize])
                                            .wrapping_add(3 as libc::c_int as libc::c_uint)
                                            as libc::c_int;
                                        (*br).cache_avail -= 3 as libc::c_int
                                    }
                                } else if !((*br).cache_avail >= 7 as libc::c_int
                                    || rar_br_fillup(a, br) != 0
                                    || (*br).cache_avail >= 7 as libc::c_int)
                                {
                                    free(precode.tree as *mut libc::c_void);
                                    free(precode.table as *mut libc::c_void);
                                    current_block = 10545432766654176713;
                                    break;
                                } else {
                                    n = (((*br).cache_buffer
                                        >> (*br).cache_avail - 7 as libc::c_int)
                                        as uint32_t
                                        & cache_masks[7 as libc::c_int as usize])
                                        .wrapping_add(11 as libc::c_int as libc::c_uint)
                                        as libc::c_int;
                                    (*br).cache_avail -= 7 as libc::c_int
                                }
                                j = 0 as libc::c_int;
                                while j < n
                                    && i < MAINCODE_SIZE
                                        + OFFSETCODE_SIZE
                                        + LOWOFFSETCODE_SIZE
                                        + LENGTHCODE_SIZE
                                {
                                    let fresh18 = i;
                                    i = i + 1;
                                    (*rar).lengthtable[fresh18 as usize] =
                                        0 as libc::c_int as libc::c_uchar;
                                    j += 1
                                }
                            }
                        }
                        match current_block {
                            10545432766654176713 => {}
                            _ => {
                                free(precode.tree as *mut libc::c_void);
                                free(precode.table as *mut libc::c_void);
                                r = create_code(
                                    a,
                                    &mut (*rar).maincode,
                                    &mut *(*rar)
                                        .lengthtable
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize),
                                    MAINCODE_SIZE,
                                    MAX_SYMBOL_LENGTH as libc::c_char,
                                );
                                if r != ARCHIVE_OK {
                                    return r;
                                }
                                r = create_code(
                                    a,
                                    &mut (*rar).offsetcode,
                                    &mut *(*rar)
                                        .lengthtable
                                        .as_mut_ptr()
                                        .offset(MAINCODE_SIZE as isize),
                                    OFFSETCODE_SIZE,
                                    MAX_SYMBOL_LENGTH as libc::c_char,
                                );
                                if r != ARCHIVE_OK {
                                    return r;
                                }
                                r = create_code(
                                    a,
                                    &mut (*rar).lowoffsetcode,
                                    &mut *(*rar)
                                        .lengthtable
                                        .as_mut_ptr()
                                        .offset((MAINCODE_SIZE + OFFSETCODE_SIZE) as isize),
                                    LOWOFFSETCODE_SIZE,
                                    MAX_SYMBOL_LENGTH as libc::c_char,
                                );
                                if r != ARCHIVE_OK {
                                    return r;
                                }
                                r = create_code(
                                    a,
                                    &mut (*rar).lengthcode,
                                    &mut *(*rar).lengthtable.as_mut_ptr().offset(
                                        (MAINCODE_SIZE + OFFSETCODE_SIZE + LOWOFFSETCODE_SIZE)
                                            as isize,
                                    ),
                                    LENGTHCODE_SIZE,
                                    MAX_SYMBOL_LENGTH as libc::c_char,
                                );
                                if r != ARCHIVE_OK {
                                    return r;
                                }
                                current_block = 13174377073168946860;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            10545432766654176713 => {}
            _ => {
                if (*rar).dictionary_size == 0 || (*rar).lzss.window.is_null() {
                    /* Seems as though dictionary sizes are not used. Even so, minimize
                     * memory usage as much as possible.
                     */
                    let mut new_window: *mut libc::c_void = 0 as *mut libc::c_void;
                    let mut new_size: libc::c_uint = 0;
                    if (*rar).unp_size >= DICTIONARY_MAX_SIZE as libc::c_long {
                        new_size = DICTIONARY_MAX_SIZE as libc::c_uint
                    } else {
                        new_size = (rar_fls((*rar).unp_size as libc::c_uint) << 1 as libc::c_int)
                            as libc::c_uint
                    }
                    if new_size == 0 as libc::c_int as libc::c_uint {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_FILE_FORMAT,
                            b"Zero window size is invalid.\x00" as *const u8 as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    new_window = realloc(
                        (*rar).lzss.window as *mut libc::c_void,
                        new_size as libc::c_ulong,
                    );
                    if new_window.is_null() {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ENOMEM,
                            b"Unable to allocate memory for uncompressed data.\x00" as *const u8
                                as *const libc::c_char,
                        );
                        return -(30 as libc::c_int);
                    }
                    (*rar).lzss.window = new_window as *mut libc::c_uchar;
                    (*rar).dictionary_size = new_size;
                    memset(
                        (*rar).lzss.window as *mut libc::c_void,
                        0 as libc::c_int,
                        (*rar).dictionary_size as libc::c_ulong,
                    );
                    (*rar).lzss.mask = (*rar)
                        .dictionary_size
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as libc::c_int
                }
                (*rar).start_new_table = 0 as libc::c_int as libc::c_char;
                return 0 as libc::c_int;
            }
        }
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
    );
    (*rar).valid = 0 as libc::c_int as libc::c_char;
    return -(30 as libc::c_int);
}
unsafe extern "C" fn free_codes(mut a: *mut archive_read) {
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    free((*rar).maincode.tree as *mut libc::c_void);
    free((*rar).offsetcode.tree as *mut libc::c_void);
    free((*rar).lowoffsetcode.tree as *mut libc::c_void);
    free((*rar).lengthcode.tree as *mut libc::c_void);
    free((*rar).maincode.table as *mut libc::c_void);
    free((*rar).offsetcode.table as *mut libc::c_void);
    free((*rar).lowoffsetcode.table as *mut libc::c_void);
    free((*rar).lengthcode.table as *mut libc::c_void);
    memset(
        &mut (*rar).maincode as *mut huffman_code as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<huffman_code>() as libc::c_ulong,
    );
    memset(
        &mut (*rar).offsetcode as *mut huffman_code as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<huffman_code>() as libc::c_ulong,
    );
    memset(
        &mut (*rar).lowoffsetcode as *mut huffman_code as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<huffman_code>() as libc::c_ulong,
    );
    memset(
        &mut (*rar).lengthcode as *mut huffman_code as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<huffman_code>() as libc::c_ulong,
    );
}
unsafe extern "C" fn read_next_symbol(
    mut a: *mut archive_read,
    mut code: *mut huffman_code,
) -> libc::c_int {
    let mut bit: libc::c_uchar = 0;
    let mut bits: libc::c_uint = 0;
    let mut length: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut rar: *mut rar = 0 as *mut rar;
    let mut br: *mut rar_br = 0 as *mut rar_br;
    if (*code).table.is_null() {
        if make_table(a, code) != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    rar = (*(*a).format).data as *mut rar;
    br = &mut (*rar).br;
    /* Look ahead (peek) at bits */
    if !((*br).cache_avail >= (*code).tablesize
        || rar_br_fillup(a, br) != 0
        || (*br).cache_avail >= (*code).tablesize)
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
        );
        (*rar).valid = 0 as libc::c_int as libc::c_char;
        return -(1 as libc::c_int);
    }
    bits = ((*br).cache_buffer >> (*br).cache_avail - (*code).tablesize) as uint32_t
        & cache_masks[(*code).tablesize as usize];
    length = (*(*code).table.offset(bits as isize)).length as libc::c_int;
    value = (*(*code).table.offset(bits as isize)).value;
    if length < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid prefix code in bitstream\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if length <= (*code).tablesize {
        /* Skip length bits */
        (*br).cache_avail -= length;
        return value;
    }
    /* Skip tablesize bits */
    (*br).cache_avail -= (*code).tablesize;
    node = value;
    while !((*(*code).tree.offset(node as isize)).branches[0 as libc::c_int as usize]
        == (*(*code).tree.offset(node as isize)).branches[1 as libc::c_int as usize])
    {
        if !((*br).cache_avail >= 1 as libc::c_int
            || rar_br_fillup(a, br) != 0
            || (*br).cache_avail >= 1 as libc::c_int)
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
            );
            (*rar).valid = 0 as libc::c_int as libc::c_char;
            return -(1 as libc::c_int);
        }
        bit = (((*br).cache_buffer >> (*br).cache_avail - 1 as libc::c_int) as uint32_t
            & cache_masks[1 as libc::c_int as usize]) as libc::c_uchar;
        (*br).cache_avail -= 1 as libc::c_int;
        if (*(*code).tree.offset(node as isize)).branches[bit as usize] < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Invalid prefix code in bitstream\x00" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        node = (*(*code).tree.offset(node as isize)).branches[bit as usize]
    }
    return (*(*code).tree.offset(node as isize)).branches[0 as libc::c_int as usize];
}
unsafe extern "C" fn create_code(
    mut a: *mut archive_read,
    mut code: *mut huffman_code,
    mut lengths: *mut libc::c_uchar,
    mut numsymbols: libc::c_int,
    mut maxlength: libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut codebits: libc::c_int = 0 as libc::c_int;
    let mut symbolsleft: libc::c_int = numsymbols;
    (*code).numentries = 0 as libc::c_int;
    (*code).numallocatedentries = 0 as libc::c_int;
    if new_node(code) < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Unable to allocate memory for node data.\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*code).numentries = 1 as libc::c_int;
    (*code).minlength = INT_MAX;
    (*code).maxlength = INT_MIN;
    codebits = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i <= maxlength as libc::c_int {
        j = 0 as libc::c_int;
        while j < numsymbols {
            if !(*lengths.offset(j as isize) as libc::c_int != i) {
                if add_value(a, code, j, codebits, i) != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
                codebits += 1;
                symbolsleft -= 1;
                if symbolsleft <= 0 as libc::c_int {
                    break;
                }
            }
            j += 1
        }
        if symbolsleft <= 0 as libc::c_int {
            break;
        }
        codebits <<= 1 as libc::c_int;
        i += 1
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_value(
    mut a: *mut archive_read,
    mut code: *mut huffman_code,
    mut value: libc::c_int,
    mut codebits: libc::c_int,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut lastnode: libc::c_int = 0;
    let mut bitpos: libc::c_int = 0;
    let mut bit: libc::c_int = 0;
    /* int repeatpos, repeatnode, nextnode; */
    free((*code).table as *mut libc::c_void);
    (*code).table = NULL as *mut huffman_table_entry;
    if length > (*code).maxlength {
        (*code).maxlength = length
    }
    if length < (*code).minlength {
        (*code).minlength = length
    }
    /*
     * Dead code, repeatpos was is -1
     *
    repeatpos = -1;
    if (repeatpos == 0 || (repeatpos >= 0
      && (((codebits >> (repeatpos - 1)) & 3) == 0
      || ((codebits >> (repeatpos - 1)) & 3) == 3)))
    {
      archive_set_error(&a->archive, ARCHIVE_ERRNO_FILE_FORMAT,
                        "Invalid repeat position");
      return (ARCHIVE_FATAL);
    }
    */
    lastnode = 0 as libc::c_int;
    bitpos = length - 1 as libc::c_int;
    while bitpos >= 0 as libc::c_int {
        bit = codebits >> bitpos & 1 as libc::c_int;
        /* } */
        if (*(*code).tree.offset(lastnode as isize)).branches[0 as libc::c_int as usize]
            == (*(*code).tree.offset(lastnode as isize)).branches[1 as libc::c_int as usize]
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Prefix found\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if (*(*code).tree.offset(lastnode as isize)).branches[bit as usize] < 0 as libc::c_int {
            if new_node(code) < 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Unable to allocate memory for node data.\x00" as *const u8
                        as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            let fresh19 = (*code).numentries;
            (*code).numentries = (*code).numentries + 1;
            (*(*code).tree.offset(lastnode as isize)).branches[bit as usize] = fresh19
        }
        lastnode = (*(*code).tree.offset(lastnode as isize)).branches[bit as usize];
        bitpos -= 1
    }
    if !((*(*code).tree.offset(lastnode as isize)).branches[0 as libc::c_int as usize]
        == -(1 as libc::c_int)
        && (*(*code).tree.offset(lastnode as isize)).branches[1 as libc::c_int as usize]
            == -(2 as libc::c_int))
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Prefix found\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Leaf node check */
    /*
     * Dead code, repeatpos was -1, bitpos >=0
     *
    if (bitpos == repeatpos)
    {
      * Open branch check *
      if (!(code->tree[lastnode].branches[bit] < 0))
      {
        archive_set_error(&a->archive, ARCHIVE_ERRNO_FILE_FORMAT,
                          "Invalid repeating code");
        return (ARCHIVE_FATAL);
      }

      if ((repeatnode = new_node(code)) < 0) {
        archive_set_error(&a->archive, ENOMEM,
                          "Unable to allocate memory for node data.");
        return (ARCHIVE_FATAL);
      }
      if ((nextnode = new_node(code)) < 0) {
        archive_set_error(&a->archive, ENOMEM,
                          "Unable to allocate memory for node data.");
        return (ARCHIVE_FATAL);
      }

      * Set branches *
      code->tree[lastnode].branches[bit] = repeatnode;
      code->tree[repeatnode].branches[bit] = repeatnode;
      code->tree[repeatnode].branches[bit^1] = nextnode;
      lastnode = nextnode;

      bitpos++; * terminating bit already handled, skip it *
    }
    else
    {
    */
    /* Open branch check */
    /* set to branch */
    /* Set leaf value */
    (*(*code).tree.offset(lastnode as isize)).branches[0 as libc::c_int as usize] = value;
    (*(*code).tree.offset(lastnode as isize)).branches[1 as libc::c_int as usize] = value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn new_node(mut code: *mut huffman_code) -> libc::c_int {
    let mut new_tree: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*code).numallocatedentries == (*code).numentries {
        let mut new_num_entries: libc::c_int = 256 as libc::c_int;
        if (*code).numentries > 0 as libc::c_int {
            new_num_entries = (*code).numentries * 2 as libc::c_int
        }
        new_tree = realloc(
            (*code).tree as *mut libc::c_void,
            (new_num_entries as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<huffman_tree_node>() as libc::c_ulong),
        );
        if new_tree.is_null() {
            return -(1 as libc::c_int);
        }
        (*code).tree = new_tree as *mut huffman_tree_node;
        (*code).numallocatedentries = new_num_entries
    }
    (*(*code).tree.offset((*code).numentries as isize)).branches[0 as libc::c_int as usize] =
        -(1 as libc::c_int);
    (*(*code).tree.offset((*code).numentries as isize)).branches[1 as libc::c_int as usize] =
        -(2 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn make_table(
    mut a: *mut archive_read,
    mut code: *mut huffman_code,
) -> libc::c_int {
    if (*code).maxlength < (*code).minlength || (*code).maxlength > 10 as libc::c_int {
        (*code).tablesize = 10 as libc::c_int
    } else {
        (*code).tablesize = (*code).maxlength
    }
    (*code).table = calloc(
        1 as libc::c_int as libc::c_ulong,
        (::std::mem::size_of::<huffman_table_entry>() as libc::c_ulong)
            .wrapping_mul((1 as libc::c_int as size_t) << (*code).tablesize),
    ) as *mut huffman_table_entry;
    return make_table_recurse(
        a,
        code,
        0 as libc::c_int,
        (*code).table,
        0 as libc::c_int,
        (*code).tablesize,
    );
}
unsafe extern "C" fn make_table_recurse(
    mut a: *mut archive_read,
    mut code: *mut huffman_code,
    mut node: libc::c_int,
    mut table: *mut huffman_table_entry,
    mut depth: libc::c_int,
    mut maxdepth: libc::c_int,
) -> libc::c_int {
    let mut currtablesize: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*code).tree.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Huffman tree was not created.\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if node < 0 as libc::c_int || node >= (*code).numentries {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid location to Huffman tree specified.\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    currtablesize = (1 as libc::c_int) << maxdepth - depth;
    if (*(*code).tree.offset(node as isize)).branches[0 as libc::c_int as usize]
        == (*(*code).tree.offset(node as isize)).branches[1 as libc::c_int as usize]
    {
        i = 0 as libc::c_int;
        while i < currtablesize {
            (*table.offset(i as isize)).length = depth as libc::c_uint;
            (*table.offset(i as isize)).value =
                (*(*code).tree.offset(node as isize)).branches[0 as libc::c_int as usize];
            i += 1
        }
    } else if depth == maxdepth {
        (*table.offset(0 as libc::c_int as isize)).length =
            (maxdepth + 1 as libc::c_int) as libc::c_uint;
        (*table.offset(0 as libc::c_int as isize)).value = node
    } else {
        ret |= make_table_recurse(
            a,
            code,
            (*(*code).tree.offset(node as isize)).branches[0 as libc::c_int as usize],
            table,
            depth + 1 as libc::c_int,
            maxdepth,
        );
        ret |= make_table_recurse(
            a,
            code,
            (*(*code).tree.offset(node as isize)).branches[1 as libc::c_int as usize],
            table.offset((currtablesize / 2 as libc::c_int) as isize),
            depth + 1 as libc::c_int,
            maxdepth,
        )
    }
    return ret;
}
/*
 * Dead code, node >= 0
 *
else if (node < 0)
{
  for(i = 0; i < currtablesize; i++)
    table[i].length = -1;
}
 */
// Initialized in run_static_initializers
static mut lengthb_min: libc::c_int = 0;
// Initialized in run_static_initializers
static mut offsetb_min: libc::c_int = 0;
unsafe extern "C" fn expand(mut a: *mut archive_read, mut end: int64_t) -> int64_t {
    let mut current_block: u64;
    static mut lengthbases: [libc::c_uchar; 28] = [
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
        56 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
    ];
    static mut lengthbits: [libc::c_uchar; 28] = [
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
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
    ];
    static mut offsetbases: [libc::c_uint; 60] = [
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        12 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
        24 as libc::c_int as libc::c_uint,
        32 as libc::c_int as libc::c_uint,
        48 as libc::c_int as libc::c_uint,
        64 as libc::c_int as libc::c_uint,
        96 as libc::c_int as libc::c_uint,
        128 as libc::c_int as libc::c_uint,
        192 as libc::c_int as libc::c_uint,
        256 as libc::c_int as libc::c_uint,
        384 as libc::c_int as libc::c_uint,
        512 as libc::c_int as libc::c_uint,
        768 as libc::c_int as libc::c_uint,
        1024 as libc::c_int as libc::c_uint,
        1536 as libc::c_int as libc::c_uint,
        2048 as libc::c_int as libc::c_uint,
        3072 as libc::c_int as libc::c_uint,
        4096 as libc::c_int as libc::c_uint,
        6144 as libc::c_int as libc::c_uint,
        8192 as libc::c_int as libc::c_uint,
        12288 as libc::c_int as libc::c_uint,
        16384 as libc::c_int as libc::c_uint,
        24576 as libc::c_int as libc::c_uint,
        32768 as libc::c_int as libc::c_uint,
        49152 as libc::c_int as libc::c_uint,
        65536 as libc::c_int as libc::c_uint,
        98304 as libc::c_int as libc::c_uint,
        131072 as libc::c_int as libc::c_uint,
        196608 as libc::c_int as libc::c_uint,
        262144 as libc::c_int as libc::c_uint,
        327680 as libc::c_int as libc::c_uint,
        393216 as libc::c_int as libc::c_uint,
        458752 as libc::c_int as libc::c_uint,
        524288 as libc::c_int as libc::c_uint,
        589824 as libc::c_int as libc::c_uint,
        655360 as libc::c_int as libc::c_uint,
        720896 as libc::c_int as libc::c_uint,
        786432 as libc::c_int as libc::c_uint,
        851968 as libc::c_int as libc::c_uint,
        917504 as libc::c_int as libc::c_uint,
        983040 as libc::c_int as libc::c_uint,
        1048576 as libc::c_int as libc::c_uint,
        1310720 as libc::c_int as libc::c_uint,
        1572864 as libc::c_int as libc::c_uint,
        1835008 as libc::c_int as libc::c_uint,
        2097152 as libc::c_int as libc::c_uint,
        2359296 as libc::c_int as libc::c_uint,
        2621440 as libc::c_int as libc::c_uint,
        2883584 as libc::c_int as libc::c_uint,
        3145728 as libc::c_int as libc::c_uint,
        3407872 as libc::c_int as libc::c_uint,
        3670016 as libc::c_int as libc::c_uint,
        3932160 as libc::c_int as libc::c_uint,
    ];
    static mut offsetbits: [libc::c_uchar; 60] = [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
    ];
    static mut shortbases: [libc::c_uchar; 8] = [
        0 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar,
    ];
    static mut shortbits: [libc::c_uchar; 8] = [
        2 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
    ];
    let mut symbol: libc::c_int = 0;
    let mut offs: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut offsindex: libc::c_int = 0;
    let mut lensymbol: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut offssymbol: libc::c_int = 0;
    let mut lowoffsetsymbol: libc::c_int = 0;
    let mut newfile: libc::c_uchar = 0;
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    let mut br: *mut rar_br = &mut (*rar).br;
    if (*rar).filterstart < end {
        end = (*rar).filterstart
    }
    loop {
        if (*rar).output_last_match as libc::c_int != 0
            && lzss_position(&mut (*rar).lzss) + (*rar).lastlength as libc::c_long <= end
        {
            lzss_emit_match(
                rar,
                (*rar).lastoffset as libc::c_int,
                (*rar).lastlength as libc::c_int,
            );
            (*rar).output_last_match = 0 as libc::c_int as libc::c_char
        }
        if (*rar).is_ppmd_block as libc::c_int != 0
            || (*rar).output_last_match as libc::c_int != 0
            || lzss_position(&mut (*rar).lzss) >= end
        {
            return lzss_position(&mut (*rar).lzss);
        }
        symbol = read_next_symbol(a, &mut (*rar).maincode);
        if symbol < 0 as libc::c_int {
            return -(30 as libc::c_int) as int64_t;
        }
        (*rar).output_last_match = 0 as libc::c_int as libc::c_char;
        if symbol < 256 as libc::c_int {
            lzss_emit_literal(rar, symbol as uint8_t);
        } else if symbol == 256 as libc::c_int {
            if !((*br).cache_avail >= 1 as libc::c_int
                || rar_br_fillup(a, br) != 0
                || (*br).cache_avail >= 1 as libc::c_int)
            {
                current_block = 5127745012624082490;
                break;
            }
            newfile = (((*br).cache_buffer >> (*br).cache_avail - 1 as libc::c_int) as uint32_t
                & cache_masks[1 as libc::c_int as usize]
                == 0) as libc::c_int as libc::c_uchar;
            (*br).cache_avail -= 1 as libc::c_int;
            if newfile != 0 {
                (*rar).start_new_block = 1 as libc::c_int as libc::c_char;
                if !((*br).cache_avail >= 1 as libc::c_int
                    || rar_br_fillup(a, br) != 0
                    || (*br).cache_avail >= 1 as libc::c_int)
                {
                    current_block = 5127745012624082490;
                    break;
                }
                (*rar).start_new_table =
                    (((*br).cache_buffer >> (*br).cache_avail - 1 as libc::c_int) as uint32_t
                        & cache_masks[1 as libc::c_int as usize])
                        as libc::c_char;
                (*br).cache_avail -= 1 as libc::c_int;
                return lzss_position(&mut (*rar).lzss);
            } else if parse_codes(a) != ARCHIVE_OK {
                return -(30 as libc::c_int) as int64_t;
            }
        } else if symbol == 257 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Parsing filters is unsupported.\x00" as *const u8 as *const libc::c_char,
            );
            return -(25 as libc::c_int) as int64_t;
        } else {
            if symbol == 258 as libc::c_int {
                if (*rar).lastlength == 0 as libc::c_int as libc::c_uint {
                    continue;
                }
                offs = (*rar).lastoffset as libc::c_int;
                len = (*rar).lastlength as libc::c_int
            } else if symbol <= 262 as libc::c_int {
                offsindex = symbol - 259 as libc::c_int;
                offs = (*rar).oldoffset[offsindex as usize] as libc::c_int;
                lensymbol = read_next_symbol(a, &mut (*rar).lengthcode);
                if lensymbol < 0 as libc::c_int {
                    current_block = 12054168401192418602;
                    break;
                }
                if lensymbol > lengthb_min {
                    current_block = 12054168401192418602;
                    break;
                }
                len = lengthbases[lensymbol as usize] as libc::c_int + 2 as libc::c_int;
                if lengthbits[lensymbol as usize] as libc::c_int > 0 as libc::c_int {
                    if !((*br).cache_avail >= lengthbits[lensymbol as usize] as libc::c_int
                        || rar_br_fillup(a, br) != 0
                        || (*br).cache_avail >= lengthbits[lensymbol as usize] as libc::c_int)
                    {
                        current_block = 5127745012624082490;
                        break;
                    }
                    len = (len as libc::c_uint).wrapping_add(
                        ((*br).cache_buffer
                            >> (*br).cache_avail - lengthbits[lensymbol as usize] as libc::c_int)
                            as uint32_t
                            & cache_masks[lengthbits[lensymbol as usize] as usize],
                    ) as libc::c_int as libc::c_int;
                    (*br).cache_avail -= lengthbits[lensymbol as usize] as libc::c_int
                }
                i = offsindex;
                while i > 0 as libc::c_int {
                    (*rar).oldoffset[i as usize] =
                        (*rar).oldoffset[(i - 1 as libc::c_int) as usize];
                    i -= 1
                }
                (*rar).oldoffset[0 as libc::c_int as usize] = offs as libc::c_uint
            } else if symbol <= 270 as libc::c_int {
                offs = shortbases[(symbol - 263 as libc::c_int) as usize] as libc::c_int
                    + 1 as libc::c_int;
                if shortbits[(symbol - 263 as libc::c_int) as usize] as libc::c_int
                    > 0 as libc::c_int
                {
                    if !((*br).cache_avail
                        >= shortbits[(symbol - 263 as libc::c_int) as usize] as libc::c_int
                        || rar_br_fillup(a, br) != 0
                        || (*br).cache_avail
                            >= shortbits[(symbol - 263 as libc::c_int) as usize] as libc::c_int)
                    {
                        current_block = 5127745012624082490;
                        break;
                    }
                    offs = (offs as libc::c_uint).wrapping_add(
                        ((*br).cache_buffer
                            >> (*br).cache_avail
                                - shortbits[(symbol - 263 as libc::c_int) as usize] as libc::c_int)
                            as uint32_t
                            & cache_masks
                                [shortbits[(symbol - 263 as libc::c_int) as usize] as usize],
                    ) as libc::c_int as libc::c_int;
                    (*br).cache_avail -=
                        shortbits[(symbol - 263 as libc::c_int) as usize] as libc::c_int
                }
                len = 2 as libc::c_int;
                i = 3 as libc::c_int;
                while i > 0 as libc::c_int {
                    (*rar).oldoffset[i as usize] =
                        (*rar).oldoffset[(i - 1 as libc::c_int) as usize];
                    i -= 1
                }
                (*rar).oldoffset[0 as libc::c_int as usize] = offs as libc::c_uint
            } else {
                if symbol - 271 as libc::c_int > lengthb_min {
                    current_block = 12054168401192418602;
                    break;
                }
                len = lengthbases[(symbol - 271 as libc::c_int) as usize] as libc::c_int
                    + 3 as libc::c_int;
                if lengthbits[(symbol - 271 as libc::c_int) as usize] as libc::c_int
                    > 0 as libc::c_int
                {
                    if !((*br).cache_avail
                        >= lengthbits[(symbol - 271 as libc::c_int) as usize] as libc::c_int
                        || rar_br_fillup(a, br) != 0
                        || (*br).cache_avail
                            >= lengthbits[(symbol - 271 as libc::c_int) as usize] as libc::c_int)
                    {
                        current_block = 5127745012624082490;
                        break;
                    }
                    len = (len as libc::c_uint).wrapping_add(
                        ((*br).cache_buffer
                            >> (*br).cache_avail
                                - lengthbits[(symbol - 271 as libc::c_int) as usize] as libc::c_int)
                            as uint32_t
                            & cache_masks
                                [lengthbits[(symbol - 271 as libc::c_int) as usize] as usize],
                    ) as libc::c_int as libc::c_int;
                    (*br).cache_avail -=
                        lengthbits[(symbol - 271 as libc::c_int) as usize] as libc::c_int
                }
                offssymbol = read_next_symbol(a, &mut (*rar).offsetcode);
                if offssymbol < 0 as libc::c_int {
                    current_block = 12054168401192418602;
                    break;
                }
                if offssymbol > offsetb_min {
                    current_block = 12054168401192418602;
                    break;
                }
                offs = offsetbases[offssymbol as usize]
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                if offsetbits[offssymbol as usize] as libc::c_int > 0 as libc::c_int {
                    if offssymbol > 9 as libc::c_int {
                        if offsetbits[offssymbol as usize] as libc::c_int > 4 as libc::c_int {
                            if !((*br).cache_avail
                                >= offsetbits[offssymbol as usize] as libc::c_int
                                    - 4 as libc::c_int
                                || rar_br_fillup(a, br) != 0
                                || (*br).cache_avail
                                    >= offsetbits[offssymbol as usize] as libc::c_int
                                        - 4 as libc::c_int)
                            {
                                current_block = 5127745012624082490;
                                break;
                            }
                            offs = (offs as libc::c_uint).wrapping_add(
                                (((*br).cache_buffer
                                    >> (*br).cache_avail
                                        - (offsetbits[offssymbol as usize] as libc::c_int
                                            - 4 as libc::c_int))
                                    as uint32_t
                                    & cache_masks[(offsetbits[offssymbol as usize] as libc::c_int
                                        - 4 as libc::c_int)
                                        as usize])
                                    << 4 as libc::c_int,
                            ) as libc::c_int as libc::c_int;
                            (*br).cache_avail -=
                                offsetbits[offssymbol as usize] as libc::c_int - 4 as libc::c_int
                        }
                        if (*rar).numlowoffsetrepeats > 0 as libc::c_int as libc::c_uint {
                            (*rar).numlowoffsetrepeats = (*rar).numlowoffsetrepeats.wrapping_sub(1);
                            offs = (offs as libc::c_uint).wrapping_add((*rar).lastlowoffset)
                                as libc::c_int as libc::c_int
                        } else {
                            lowoffsetsymbol = read_next_symbol(a, &mut (*rar).lowoffsetcode);
                            if lowoffsetsymbol < 0 as libc::c_int {
                                return -(30 as libc::c_int) as int64_t;
                            }
                            if lowoffsetsymbol == 16 as libc::c_int {
                                (*rar).numlowoffsetrepeats = 15 as libc::c_int as libc::c_uint;
                                offs = (offs as libc::c_uint).wrapping_add((*rar).lastlowoffset)
                                    as libc::c_int
                                    as libc::c_int
                            } else {
                                offs += lowoffsetsymbol;
                                (*rar).lastlowoffset = lowoffsetsymbol as libc::c_uint
                            }
                        }
                    } else {
                        if !((*br).cache_avail >= offsetbits[offssymbol as usize] as libc::c_int
                            || rar_br_fillup(a, br) != 0
                            || (*br).cache_avail >= offsetbits[offssymbol as usize] as libc::c_int)
                        {
                            current_block = 5127745012624082490;
                            break;
                        }
                        offs = (offs as libc::c_uint).wrapping_add(
                            ((*br).cache_buffer
                                >> (*br).cache_avail
                                    - offsetbits[offssymbol as usize] as libc::c_int)
                                as uint32_t
                                & cache_masks[offsetbits[offssymbol as usize] as usize],
                        ) as libc::c_int as libc::c_int;
                        (*br).cache_avail -= offsetbits[offssymbol as usize] as libc::c_int
                    }
                }
                if offs >= 0x40000 as libc::c_int {
                    len += 1
                }
                if offs >= 0x2000 as libc::c_int {
                    len += 1
                }
                i = 3 as libc::c_int;
                while i > 0 as libc::c_int {
                    (*rar).oldoffset[i as usize] =
                        (*rar).oldoffset[(i - 1 as libc::c_int) as usize];
                    i -= 1
                }
                (*rar).oldoffset[0 as libc::c_int as usize] = offs as libc::c_uint
            }
            (*rar).lastoffset = offs as libc::c_uint;
            (*rar).lastlength = len as libc::c_uint;
            (*rar).output_last_match = 1 as libc::c_int as libc::c_char
        }
    }
    match current_block {
        12054168401192418602 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Bad RAR file data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int) as int64_t;
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated RAR file data\x00" as *const u8 as *const libc::c_char,
            );
            (*rar).valid = 0 as libc::c_int as libc::c_char;
            return -(30 as libc::c_int) as int64_t;
        }
    };
}
unsafe extern "C" fn copy_from_lzss_window(
    mut a: *mut archive_read,
    mut buffer: *mut *const libc::c_void,
    mut startpos: int64_t,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut windowoffs: libc::c_int = 0;
    let mut firstpart: libc::c_int = 0;
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    if (*rar).unp_buffer.is_null() {
        (*rar).unp_buffer = malloc((*rar).unp_buffer_size as libc::c_ulong) as *mut libc::c_uchar;
        if (*rar).unp_buffer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Unable to allocate memory for uncompressed data.\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    windowoffs = lzss_offset_for_position(&mut (*rar).lzss, startpos);
    if windowoffs + length <= lzss_size(&mut (*rar).lzss) {
        memcpy(
            &mut *(*rar).unp_buffer.offset((*rar).unp_offset as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            &mut *(*rar).lzss.window.offset(windowoffs as isize) as *mut libc::c_uchar
                as *const libc::c_void,
            length as libc::c_ulong,
        );
    } else if length <= lzss_size(&mut (*rar).lzss) {
        firstpart = lzss_size(&mut (*rar).lzss) - windowoffs;
        if firstpart < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Bad RAR file data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if firstpart < length {
            memcpy(
                &mut *(*rar).unp_buffer.offset((*rar).unp_offset as isize) as *mut libc::c_uchar
                    as *mut libc::c_void,
                &mut *(*rar).lzss.window.offset(windowoffs as isize) as *mut libc::c_uchar
                    as *const libc::c_void,
                firstpart as libc::c_ulong,
            );
            memcpy(
                &mut *(*rar)
                    .unp_buffer
                    .offset((*rar).unp_offset.wrapping_add(firstpart as libc::c_uint) as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                &mut *(*rar).lzss.window.offset(0 as libc::c_int as isize) as *mut libc::c_uchar
                    as *const libc::c_void,
                (length - firstpart) as libc::c_ulong,
            );
        } else {
            memcpy(
                &mut *(*rar).unp_buffer.offset((*rar).unp_offset as isize) as *mut libc::c_uchar
                    as *mut libc::c_void,
                &mut *(*rar).lzss.window.offset(windowoffs as isize) as *mut libc::c_uchar
                    as *const libc::c_void,
                length as libc::c_ulong,
            );
        }
    } else {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Bad RAR file data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*rar).unp_offset = (*rar).unp_offset.wrapping_add(length as libc::c_uint);
    if (*rar).unp_offset >= (*rar).unp_buffer_size {
        *buffer = (*rar).unp_buffer as *const libc::c_void
    } else {
        *buffer = NULL as *const libc::c_void
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn rar_read_ahead(
    mut a: *mut archive_read,
    mut min: size_t,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    let mut rar: *mut rar = (*(*a).format).data as *mut rar;
    let mut h: *const libc::c_void = __archive_read_ahead(a, min, avail);
    let mut ret: libc::c_int = 0;
    if !avail.is_null() {
        if (*a).archive.read_data_is_posix_read as libc::c_int != 0
            && *avail > (*a).archive.read_data_requested as ssize_t
        {
            *avail = (*a).archive.read_data_requested as ssize_t
        }
        if *avail > (*rar).bytes_remaining {
            *avail = (*rar).bytes_remaining
        }
        if *avail < 0 as libc::c_int as libc::c_long {
            return NULL as *const libc::c_void;
        } else {
            if *avail == 0 as libc::c_int as libc::c_long
                && (*rar).main_flags & MHD_VOLUME as libc::c_uint != 0
                && (*rar).file_flags & FHD_SPLIT_AFTER as libc::c_uint != 0
            {
                (*rar).filename_must_match = 1 as libc::c_int as libc::c_char;
                ret = archive_read_format_rar_read_header(a, (*a).entry);
                if ret == 1 as libc::c_int {
                    (*rar).has_endarc_header = 1 as libc::c_int as libc::c_char;
                    ret = archive_read_format_rar_read_header(a, (*a).entry)
                }
                (*rar).filename_must_match = 0 as libc::c_int as libc::c_char;
                if ret != 0 as libc::c_int {
                    return NULL as *const libc::c_void;
                }
                return rar_read_ahead(a, min, avail);
            }
        }
    }
    return h;
}
unsafe extern "C" fn run_static_initializers() {
    lengthb_min = if ((::std::mem::size_of::<[libc::c_uchar; 28]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as libc::c_int)
        < (::std::mem::size_of::<[libc::c_uchar; 28]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as libc::c_int
    {
        (::std::mem::size_of::<[libc::c_uchar; 28]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as libc::c_int
    } else {
        (::std::mem::size_of::<[libc::c_uchar; 28]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as libc::c_int
    };
    offsetb_min = if ((::std::mem::size_of::<[libc::c_uint; 60]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
        as libc::c_int)
        < (::std::mem::size_of::<[libc::c_uchar; 60]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as libc::c_int
    {
        (::std::mem::size_of::<[libc::c_uint; 60]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            as libc::c_int
    } else {
        (::std::mem::size_of::<[libc::c_uchar; 60]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as libc::c_int
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
