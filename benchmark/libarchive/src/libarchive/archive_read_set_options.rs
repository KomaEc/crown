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
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn _archive_set_option(
        a: *mut archive,
        mod_0: *const libc::c_char,
        opt: *const libc::c_char,
        val: *const libc::c_char,
        magic: libc::c_int,
        fn_0: *const libc::c_char,
        use_option: option_handler,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_set_options(
        a: *mut archive,
        options: *const libc::c_char,
        magic: libc::c_int,
        fn_0: *const libc::c_char,
        use_option: option_handler,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_set_either_option(
        a: *mut archive,
        m: *const libc::c_char,
        o: *const libc::c_char,
        v: *const libc::c_char,
        use_format_option: option_handler,
        use_filter_option: option_handler,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
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
pub type option_handler = Option<
    unsafe extern "C" fn(
        _: *mut archive,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int,
>;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_READ_MAGIC: libc::c_uint = 0xdeb0c5 as libc::c_uint;
/*
 * Set read options.
 */
/* Apply option to the format only. */
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_format_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_option(
        a,
        m,
        o,
        v,
        ARCHIVE_READ_MAGIC as libc::c_int,
        b"archive_read_set_format_option\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_format_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
/* Apply option to the filter only. */
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_filter_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_option(
        a,
        m,
        o,
        v,
        ARCHIVE_READ_MAGIC as libc::c_int,
        b"archive_read_set_filter_option\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_filter_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
/* Apply option to both the format and the filter. */
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_option(
        a,
        m,
        o,
        v,
        ARCHIVE_READ_MAGIC as libc::c_int,
        b"archive_read_set_option\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
/* Apply option string to both the format and the filter. */
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_options(
    mut a: *mut archive,
    mut options: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_options(
        a,
        options,
        ARCHIVE_READ_MAGIC as libc::c_int,
        b"archive_read_set_options\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_set_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
unsafe extern "C" fn archive_set_format_option(
    mut _a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut i: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut rv: libc::c_int = ARCHIVE_WARN;
    let mut matched_modules: libc::c_int = 0 as libc::c_int;
    let mut current_block_8: u64;
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[archive_format_descriptor; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<archive_format_descriptor>() as libc::c_ulong)
    {
        let mut format: *mut archive_format_descriptor =
            &mut *(*a).formats.as_mut_ptr().offset(i as isize) as *mut archive_format_descriptor;
        if !((*format).options.is_none() || (*format).name.is_null()) {
            if !m.is_null() {
                if strcmp((*format).name, m) != 0 as libc::c_int {
                    current_block_8 = 14916268686031723178;
                } else {
                    matched_modules += 1;
                    current_block_8 = 5720623009719927633;
                }
            } else {
                current_block_8 = 5720623009719927633;
            }
            match current_block_8 {
                14916268686031723178 => {}
                _ => {
                    (*a).format = format;
                    r = (*format).options.expect("non-null function pointer")(a, o, v);
                    (*a).format = NULL as *mut archive_format_descriptor;
                    if r == ARCHIVE_FATAL {
                        return -(30 as libc::c_int);
                    }
                    if r == ARCHIVE_OK {
                        rv = ARCHIVE_OK
                    }
                }
            }
        }
        /* This format does not support option. */
        i = i.wrapping_add(1)
    }
    /* If the format name didn't match, return a special code for
     * _archive_set_option[s]. */
    if !m.is_null() && matched_modules == 0 as libc::c_int {
        return ARCHIVE_WARN - 1 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn archive_set_filter_option(
    mut _a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut filter: *mut archive_read_filter = 0 as *mut archive_read_filter;
    let mut bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut r: libc::c_int = 0;
    let mut rv: libc::c_int = ARCHIVE_WARN;
    let mut matched_modules: libc::c_int = 0 as libc::c_int;
    let mut current_block_7: u64;
    filter = (*a).filter;
    while !filter.is_null() {
        bidder = (*filter).bidder;
        if !bidder.is_null() {
            if !(*bidder).options.is_none() {
                if !m.is_null() {
                    if strcmp((*filter).name, m) != 0 as libc::c_int {
                        current_block_7 = 12675440807659640239;
                    } else {
                        matched_modules += 1;
                        current_block_7 = 13536709405535804910;
                    }
                } else {
                    current_block_7 = 13536709405535804910;
                }
                match current_block_7 {
                    12675440807659640239 => {}
                    _ => {
                        r = (*bidder).options.expect("non-null function pointer")(bidder, o, v);
                        if r == ARCHIVE_FATAL {
                            return -(30 as libc::c_int);
                        }
                        if r == ARCHIVE_OK {
                            rv = ARCHIVE_OK
                        }
                    }
                }
            }
        }
        /* This bidder does not support option */
        filter = (*filter).upstream
    }
    /* If the filter name didn't match, return a special code for
     * _archive_set_option[s]. */
    if !m.is_null() && matched_modules == 0 as libc::c_int {
        return ARCHIVE_WARN - 1 as libc::c_int;
    }
    return rv;
}
unsafe extern "C" fn archive_set_option(
    mut a: *mut archive,
    mut m: *const libc::c_char,
    mut o: *const libc::c_char,
    mut v: *const libc::c_char,
) -> libc::c_int {
    return _archive_set_either_option(
        a,
        m,
        o,
        v,
        Some(
            archive_set_format_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_set_filter_option
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
    );
}
