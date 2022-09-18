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
    pub type archive_entry;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_read_close(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_data_block(
        a: *mut archive,
        buff: *mut *const libc::c_void,
        size: *mut size_t,
        offset: *mut la_int64_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
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
    /*
     * Symlink types
     */
    /*
     * Basic object manipulation
     */
    #[no_mangle]
    fn archive_entry_clear(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    /*
     * This form of archive_entry_new2() will pull character-set
     * conversion information from the specified archive handle.  The
     * older archive_entry_new(void) form is equivalent to calling
     * archive_entry_new2(NULL) and will result in the use of an internal
     * default character-set conversion.
     */
    #[no_mangle]
    fn archive_entry_new2(_: *mut archive) -> *mut archive_entry;
    /* Initialize an archive_string object on the stack or elsewhere. */
    /* Append a C char to an archive_string, resizing as necessary. */
    /* Ditto for a wchar_t and an archive_wstring. */
    /* Append a raw array to an archive_string, resizing as necessary */
    /* Convert a Unicode string to current locale and append the result. */
    /* Returns -1 if conversion fails. */
    /* Create a string conversion object.
     * Return NULL and set a error message if the conversion is not supported
     * on the platform. */
    /* Create the default string conversion object for reading/writing an archive.
     * Return NULL if the conversion is unneeded.
     * Note: On non Windows platform this always returns NULL.
     */
    /* Dispose of a string conversion object. */
    /* Copy one archive_string to another in locale conversion.
     * Return -1 if conversion fails. */
    /* Copy one archive_string to another in locale conversion.
     * Return -1 if conversion fails. */
    /* Copy one archive_string to another */
    /* Concatenate one archive_string to another */
    /* Ensure that the underlying buffer is at least as large as the request. */
    /* Append C string, which may lack trailing \0. */
    /* The source is declared void * here because this gets used with
     * "signed char *", "unsigned char *" and "char *" arguments.
     * Declaring it "char *" as with some of the other functions just
     * leads to a lot of extra casts. */
    /* Append a C string to an archive_string, resizing as necessary. */
    /* Copy a C string to an archive_string, resizing as necessary. */
    /* Copy a C string to an archive_string with limit, resizing as necessary. */
    /* Return length of string. */
    /* Set string length to zero. */
    /* Release any allocated storage resources. */
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_errx(retvalue: libc::c_int, msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn __archive_clean(_: *mut archive) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __intmax_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type intmax_t = __intmax_t;
pub type size_t = libc::c_ulong;
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
pub const ARCHIVE_ERRNO_PROGRAMMER: libc::c_int = EINVAL;
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const EIO: libc::c_int = 5 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_RETRY: libc::c_int = -(10 as libc::c_int);
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_READ_FORMAT_CAPS_NONE: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA: libc::c_int =
    (1 as libc::c_int) << 0 as libc::c_int;
pub const ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA: libc::c_int =
    (1 as libc::c_int) << 1 as libc::c_int;
pub const ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED: libc::c_int = -(2 as libc::c_int);
pub const ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW: libc::c_int = -(1 as libc::c_int);
pub const ARCHIVE_FILTER_NONE: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_STATE_DATA: libc::c_uint = 4 as libc::c_uint;
pub const ARCHIVE_STATE_EOF: libc::c_uint = 0x10 as libc::c_uint;
pub const ARCHIVE_STATE_HEADER: libc::c_uint = 2 as libc::c_uint;
pub const ARCHIVE_STATE_CLOSED: libc::c_uint = 0x20 as libc::c_uint;
pub const ARCHIVE_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
pub const ARCHIVE_STATE_NEW: libc::c_uint = 1 as libc::c_uint;
pub const ARCHIVE_READ_MAGIC: libc::c_uint = 0xdeb0c5 as libc::c_uint;
pub const ARCHIVE_STATE_ANY: libc::c_uint = 0xffff as libc::c_uint & !ARCHIVE_STATE_FATAL;
unsafe extern "C" fn archive_read_vtable() -> *mut archive_vtable {
    static mut av: archive_vtable = archive_vtable {
        archive_close: None,
        archive_free: None,
        archive_write_header: None,
        archive_write_finish_entry: None,
        archive_write_data: None,
        archive_write_data_block: None,
        archive_read_next_header: None,
        archive_read_next_header2: None,
        archive_read_data_block: None,
        archive_filter_count: None,
        archive_filter_bytes: None,
        archive_filter_code: None,
        archive_filter_name: None,
    };
    static mut inited: libc::c_int = 0 as libc::c_int;
    if inited == 0 {
        av.archive_filter_bytes = Some(
            _archive_filter_bytes
                as unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t,
        );
        av.archive_filter_code = Some(
            _archive_filter_code
                as unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int,
        );
        av.archive_filter_name = Some(
            _archive_filter_name
                as unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char,
        );
        av.archive_filter_count =
            Some(_archive_filter_count as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_read_data_block = Some(
            _archive_read_data_block
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        );
        av.archive_read_next_header = Some(
            _archive_read_next_header
                as unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int,
        );
        av.archive_read_next_header2 = Some(
            _archive_read_next_header2
                as unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int,
        );
        av.archive_free =
            Some(_archive_read_free as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        av.archive_close =
            Some(_archive_read_close as unsafe extern "C" fn(_: *mut archive) -> libc::c_int);
        inited = 1 as libc::c_int
    }
    return &mut av;
}
/*
 * Allocate, initialize and return a struct archive object.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_new() -> *mut archive {
    let mut a: *mut archive_read = 0 as *mut archive_read;
    a = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_read>() as libc::c_ulong,
    ) as *mut archive_read;
    if a.is_null() {
        return 0 as *mut archive;
    }
    (*a).archive.magic = ARCHIVE_READ_MAGIC;
    (*a).archive.state = ARCHIVE_STATE_NEW;
    (*a).entry = archive_entry_new2(&mut (*a).archive);
    (*a).archive.vtable = archive_read_vtable();
    (*a).passphrases.last = &mut (*a).passphrases.first;
    return &mut (*a).archive;
}
/*
 * Record the do-not-extract-to file. This belongs in archive_read_extract.c.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_extract_set_skip_file(
    mut _a: *mut archive,
    mut d: la_int64_t,
    mut i: la_int64_t,
) {
    let mut a: *mut archive_read = _a as *mut archive_read;
    if ARCHIVE_OK
        != __archive_check_magic(
            _a,
            ARCHIVE_READ_MAGIC,
            ARCHIVE_STATE_ANY,
            b"archive_read_extract_set_skip_file\x00" as *const u8 as *const libc::c_char,
        )
    {
        return;
    }
    (*a).skip_file_set = 1 as libc::c_int;
    (*a).skip_file_dev = d;
    (*a).skip_file_ino = i;
}
/*
 * Open the archive
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_open(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut client_opener: Option<archive_open_callback>,
    mut client_reader: Option<archive_read_callback>,
    mut client_closer: Option<archive_close_callback>,
) -> libc::c_int {
    /* Old archive_read_open() is just a thin shell around
     * archive_read_open1. */
    archive_read_set_open_callback(a, client_opener);
    archive_read_set_read_callback(a, client_reader);
    archive_read_set_close_callback(a, client_closer);
    archive_read_set_callback_data(a, client_data);
    return archive_read_open1(a);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_open2(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut client_opener: Option<archive_open_callback>,
    mut client_reader: Option<archive_read_callback>,
    mut client_skipper: Option<archive_skip_callback>,
    mut client_closer: Option<archive_close_callback>,
) -> libc::c_int {
    /* Old archive_read_open2() is just a thin shell around
     * archive_read_open1. */
    archive_read_set_callback_data(a, client_data);
    archive_read_set_open_callback(a, client_opener);
    archive_read_set_read_callback(a, client_reader);
    archive_read_set_skip_callback(a, client_skipper);
    archive_read_set_close_callback(a, client_closer);
    return archive_read_open1(a);
}
unsafe extern "C" fn client_read_proxy(
    mut self_0: *mut archive_read_filter,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut r: ssize_t = 0;
    r = (*(*self_0).archive)
        .client
        .reader
        .expect("non-null function pointer")(
        &mut (*(*self_0).archive).archive,
        (*self_0).data,
        buff,
    );
    return r;
}
unsafe extern "C" fn client_skip_proxy(
    mut self_0: *mut archive_read_filter,
    mut request: int64_t,
) -> int64_t {
    if request < 0 as libc::c_int as libc::c_long {
        __archive_errx(
            1 as libc::c_int,
            b"Negative skip requested.\x00" as *const u8 as *const libc::c_char,
        );
    }
    if request == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as int64_t;
    }
    if (*(*self_0).archive).client.skipper.is_some() {
        /* Seek requests over 1GiB are broken down into
         * multiple seeks.  This avoids overflows when the
         * requests get passed through 32-bit arguments. */
        let mut skip_limit: int64_t = (1 as libc::c_int as int64_t) << 30 as libc::c_int;
        let mut total: int64_t = 0 as libc::c_int as int64_t;
        loop {
            let mut get: int64_t = 0;
            let mut ask: int64_t = request;
            if ask > skip_limit {
                ask = skip_limit
            }
            get = (*(*self_0).archive)
                .client
                .skipper
                .expect("non-null function pointer")(
                &mut (*(*self_0).archive).archive,
                (*self_0).data,
                ask,
            );
            total += get;
            if get == 0 as libc::c_int as libc::c_long || get == request {
                return total;
            }
            if get > request {
                return ARCHIVE_FATAL as int64_t;
            }
            request -= get
        }
    } else {
        if (*(*self_0).archive).client.seeker.is_some()
            && request > (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
        {
            /* If the client provided a seeker but not a skipper,
             * we can use the seeker to skip forward.
             *
             * Note: This isn't always a good idea.  The client
             * skipper is allowed to skip by less than requested
             * if it needs to maintain block alignment.  The
             * seeker is not allowed to play such games, so using
             * the seeker here may be a performance loss compared
             * to just reading and discarding.  That's why we
             * only do this for skips of over 64k.
             */
            let mut before: int64_t = (*self_0).position;
            let mut after: int64_t = (*(*self_0).archive)
                .client
                .seeker
                .expect("non-null function pointer")(
                &mut (*(*self_0).archive).archive,
                (*self_0).data,
                request,
                SEEK_CUR,
            );
            if after != before + request {
                return ARCHIVE_FATAL as int64_t;
            }
            return after - before;
        }
    }
    return 0 as libc::c_int as int64_t;
}
unsafe extern "C" fn client_seek_proxy(
    mut self_0: *mut archive_read_filter,
    mut offset: int64_t,
    mut whence: libc::c_int,
) -> int64_t {
    /* DO NOT use the skipper here!  If we transparently handled
     * forward seek here by using the skipper, that will break
     * other libarchive code that assumes a successful forward
     * seek means it can also seek backwards.
     */
    if (*(*self_0).archive).client.seeker.is_none() {
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Current client reader does not support seeking a device\x00" as *const u8
                as *const libc::c_char,
        );
        return -(25 as libc::c_int) as int64_t;
    }
    return (*(*self_0).archive)
        .client
        .seeker
        .expect("non-null function pointer")(
        &mut (*(*self_0).archive).archive,
        (*self_0).data,
        offset,
        whence,
    );
}
unsafe extern "C" fn client_close_proxy(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r2: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    if (*(*self_0).archive).client.closer.is_none() {
        return r;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*(*self_0).archive).client.nodes {
        r2 = (*(*self_0).archive)
            .client
            .closer
            .expect("non-null function pointer")(
            (*self_0).archive as *mut archive,
            (*(*(*self_0).archive).client.dataset.offset(i as isize)).data,
        );
        if r > r2 {
            r = r2
        }
        i = i.wrapping_add(1)
    }
    return r;
}
unsafe extern "C" fn client_open_proxy(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut r: libc::c_int = ARCHIVE_OK;
    if (*(*self_0).archive).client.opener.is_some() {
        r = (*(*self_0).archive)
            .client
            .opener
            .expect("non-null function pointer")(
            (*self_0).archive as *mut archive, (*self_0).data
        )
    }
    return r;
}
unsafe extern "C" fn client_switch_proxy(
    mut self_0: *mut archive_read_filter,
    mut iindex: libc::c_uint,
) -> libc::c_int {
    let mut r1: libc::c_int = ARCHIVE_OK;
    let mut r2: libc::c_int = ARCHIVE_OK;
    let mut data2: *mut libc::c_void = NULL as *mut libc::c_void;
    /* Don't do anything if already in the specified data node */
    if (*(*self_0).archive).client.cursor == iindex {
        return 0 as libc::c_int;
    }
    (*(*self_0).archive).client.cursor = iindex;
    data2 = (*(*(*self_0).archive)
        .client
        .dataset
        .offset((*(*self_0).archive).client.cursor as isize))
    .data;
    if (*(*self_0).archive).client.switcher.is_some() {
        r2 = (*(*self_0).archive)
            .client
            .switcher
            .expect("non-null function pointer")(
            (*self_0).archive as *mut archive,
            (*self_0).data,
            data2,
        );
        r1 = r2;
        (*self_0).data = data2
    } else {
        /* Attempt to call close and open instead */
        if (*(*self_0).archive).client.closer.is_some() {
            r1 = (*(*self_0).archive)
                .client
                .closer
                .expect("non-null function pointer")(
                (*self_0).archive as *mut archive,
                (*self_0).data,
            )
        }
        (*self_0).data = data2;
        if (*(*self_0).archive).client.opener.is_some() {
            r2 = (*(*self_0).archive)
                .client
                .opener
                .expect("non-null function pointer")(
                (*self_0).archive as *mut archive,
                (*self_0).data,
            )
        }
    }
    return if r1 < r2 { r1 } else { r2 };
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_open_callback(
    mut _a: *mut archive,
    mut client_opener: Option<archive_open_callback>,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_set_open_callback\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).client.opener = client_opener;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_read_callback(
    mut _a: *mut archive,
    mut client_reader: Option<archive_read_callback>,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_set_read_callback\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).client.reader = client_reader;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_skip_callback(
    mut _a: *mut archive,
    mut client_skipper: Option<archive_skip_callback>,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_set_skip_callback\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).client.skipper = client_skipper;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_seek_callback(
    mut _a: *mut archive,
    mut client_seeker: Option<archive_seek_callback>,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_set_seek_callback\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).client.seeker = client_seeker;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_close_callback(
    mut _a: *mut archive,
    mut client_closer: Option<archive_close_callback>,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_set_close_callback\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).client.closer = client_closer;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_switch_callback(
    mut _a: *mut archive,
    mut client_switcher: Option<archive_switch_callback>,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_set_switch_callback\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*a).client.switcher = client_switcher;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_callback_data(
    mut _a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    return archive_read_set_callback_data2(_a, client_data, 0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_set_callback_data2(
    mut _a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut iindex: libc::c_uint,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_set_callback_data2\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).client.nodes == 0 as libc::c_int as libc::c_uint {
        (*a).client.dataset = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<archive_read_data_node>() as libc::c_ulong,
        ) as *mut archive_read_data_node;
        if (*a).client.dataset.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"No memory.\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL;
        }
        (*a).client.nodes = 1 as libc::c_int as libc::c_uint
    }
    if iindex
        > (*a)
            .client
            .nodes
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Invalid index specified.\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    let ref mut fresh0 = (*(*a).client.dataset.offset(iindex as isize)).data;
    *fresh0 = client_data;
    (*(*a).client.dataset.offset(iindex as isize)).begin_position = -(1 as libc::c_int) as int64_t;
    (*(*a).client.dataset.offset(iindex as isize)).total_size = -(1 as libc::c_int) as int64_t;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_add_callback_data(
    mut _a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut iindex: libc::c_uint,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_uint = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_add_callback_data\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if iindex > (*a).client.nodes {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Invalid index specified.\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    (*a).client.nodes = (*a).client.nodes.wrapping_add(1);
    p = realloc(
        (*a).client.dataset as *mut libc::c_void,
        (::std::mem::size_of::<archive_read_data_node>() as libc::c_ulong)
            .wrapping_mul((*a).client.nodes as libc::c_ulong),
    );
    if p.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"No memory.\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    (*a).client.dataset = p as *mut archive_read_data_node;
    i = (*a)
        .client
        .nodes
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i > iindex {
        let ref mut fresh1 = (*(*a).client.dataset.offset(i as isize)).data;
        *fresh1 = (*(*a)
            .client
            .dataset
            .offset(i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
        .data;
        (*(*a).client.dataset.offset(i as isize)).begin_position = -(1 as libc::c_int) as int64_t;
        (*(*a).client.dataset.offset(i as isize)).total_size = -(1 as libc::c_int) as int64_t;
        i = i.wrapping_sub(1)
    }
    let ref mut fresh2 = (*(*a).client.dataset.offset(iindex as isize)).data;
    *fresh2 = client_data;
    (*(*a).client.dataset.offset(iindex as isize)).begin_position = -(1 as libc::c_int) as int64_t;
    (*(*a).client.dataset.offset(iindex as isize)).total_size = -(1 as libc::c_int) as int64_t;
    return ARCHIVE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_append_callback_data(
    mut _a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    return archive_read_add_callback_data(_a, client_data, (*a).client.nodes);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_prepend_callback_data(
    mut _a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    return archive_read_add_callback_data(_a, client_data, 0 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_open1(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut filter: *mut archive_read_filter = 0 as *mut archive_read_filter;
    let mut tmp: *mut archive_read_filter = 0 as *mut archive_read_filter;
    let mut slot: libc::c_int = 0;
    let mut e: libc::c_int = ARCHIVE_OK;
    let mut i: libc::c_uint = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_open\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_clear_error(&mut (*a).archive);
    if (*a).client.reader.is_none() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"No reader function provided to archive_read_open\x00" as *const u8
                as *const libc::c_char,
        );
        (*a).archive.state = ARCHIVE_STATE_FATAL;
        return -(30 as libc::c_int);
    }
    /* Open data source. */
    if (*a).client.opener.is_some() {
        e = (*a).client.opener.expect("non-null function pointer")(
            &mut (*a).archive,
            (*(*a).client.dataset.offset(0 as libc::c_int as isize)).data,
        );
        if e != 0 as libc::c_int {
            /* If the open failed, call the closer to clean up. */
            if (*a).client.closer.is_some() {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*a).client.nodes {
                    (*a).client.closer.expect("non-null function pointer")(
                        &mut (*a).archive,
                        (*(*a).client.dataset.offset(i as isize)).data,
                    );
                    i = i.wrapping_add(1)
                }
            }
            return e;
        }
    }
    filter = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_read_filter>() as libc::c_ulong,
    ) as *mut archive_read_filter;
    if filter.is_null() {
        return -(30 as libc::c_int);
    }
    (*filter).bidder = NULL as *mut archive_read_filter_bidder;
    (*filter).upstream = NULL as *mut archive_read_filter;
    (*filter).archive = a;
    (*filter).data = (*(*a).client.dataset.offset(0 as libc::c_int as isize)).data;
    (*filter).open =
        Some(client_open_proxy as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int);
    (*filter).read = Some(
        client_read_proxy
            as unsafe extern "C" fn(
                _: *mut archive_read_filter,
                _: *mut *const libc::c_void,
            ) -> ssize_t,
    );
    (*filter).skip = Some(
        client_skip_proxy
            as unsafe extern "C" fn(_: *mut archive_read_filter, _: int64_t) -> int64_t,
    );
    (*filter).seek = Some(
        client_seek_proxy
            as unsafe extern "C" fn(
                _: *mut archive_read_filter,
                _: int64_t,
                _: libc::c_int,
            ) -> int64_t,
    );
    (*filter).close = Some(
        client_close_proxy as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int,
    );
    (*filter).sswitch = Some(
        client_switch_proxy
            as unsafe extern "C" fn(_: *mut archive_read_filter, _: libc::c_uint) -> libc::c_int,
    );
    (*filter).name = b"none\x00" as *const u8 as *const libc::c_char;
    (*filter).code = ARCHIVE_FILTER_NONE;
    (*(*a).client.dataset.offset(0 as libc::c_int as isize)).begin_position =
        0 as libc::c_int as int64_t;
    if (*a).filter.is_null() || (*a).bypass_filter_bidding == 0 {
        (*a).filter = filter;
        /* Build out the input pipeline. */
        e = choose_filters(a);
        if e < ARCHIVE_WARN {
            (*a).archive.state = ARCHIVE_STATE_FATAL;
            return -(30 as libc::c_int);
        }
    } else {
        /* Need to add "NONE" type filter at the end of the filter chain */
        tmp = (*a).filter;
        while !(*tmp).upstream.is_null() {
            tmp = (*tmp).upstream
        }
        (*tmp).upstream = filter
    }
    if (*a).format.is_null() {
        slot = choose_format(a);
        if slot < 0 as libc::c_int {
            close_filters(a);
            (*a).archive.state = ARCHIVE_STATE_FATAL;
            return -(30 as libc::c_int);
        }
        (*a).format =
            &mut *(*a).formats.as_mut_ptr().offset(slot as isize) as *mut archive_format_descriptor
    }
    (*a).archive.state = ARCHIVE_STATE_HEADER;
    /* Ensure libarchive starts from the first node in a multivolume set */
    client_switch_proxy((*a).filter, 0 as libc::c_int as libc::c_uint);
    return e;
}
/*
 * Allow each registered stream transform to bid on whether
 * it wants to handle this stream.  Repeat until we've finished
 * building the pipeline.
 */
/* We won't build a filter pipeline with more stages than this. */
pub const MAX_NUMBER_FILTERS: libc::c_int = 25 as libc::c_int;
unsafe extern "C" fn choose_filters(mut a: *mut archive_read) -> libc::c_int {
    let mut number_bidders: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bid: libc::c_int = 0;
    let mut best_bid: libc::c_int = 0;
    let mut number_filters: libc::c_int = 0;
    let mut bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut best_bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut filter: *mut archive_read_filter = 0 as *mut archive_read_filter;
    let mut avail: ssize_t = 0;
    let mut r: libc::c_int = 0;
    number_filters = 0 as libc::c_int;
    while number_filters < MAX_NUMBER_FILTERS {
        number_bidders = (::std::mem::size_of::<[archive_read_filter_bidder; 16]>()
            as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<archive_read_filter_bidder>() as libc::c_ulong)
            as libc::c_int;
        best_bid = 0 as libc::c_int;
        best_bidder = NULL as *mut archive_read_filter_bidder;
        bidder = (*a).bidders.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < number_bidders {
            if (*bidder).bid.is_some() {
                bid = (*bidder).bid.expect("non-null function pointer")(bidder, (*a).filter);
                if bid > best_bid {
                    best_bid = bid;
                    best_bidder = bidder
                }
            }
            i += 1;
            bidder = bidder.offset(1)
        }
        /* If no bidder, we're done. */
        if best_bidder.is_null() {
            /* Verify the filter by asking it for some data. */
            __archive_read_filter_ahead((*a).filter, 1 as libc::c_int as size_t, &mut avail);
            if avail < 0 as libc::c_int as libc::c_long {
                __archive_read_free_filters(a);
                return -(30 as libc::c_int);
            }
            (*a).archive.compression_name = (*(*a).filter).name;
            (*a).archive.compression_code = (*(*a).filter).code;
            return 0 as libc::c_int;
        }
        filter = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<archive_read_filter>() as libc::c_ulong,
        ) as *mut archive_read_filter;
        if filter.is_null() {
            return -(30 as libc::c_int);
        }
        (*filter).bidder = best_bidder;
        (*filter).archive = a;
        (*filter).upstream = (*a).filter;
        (*a).filter = filter;
        r = (*best_bidder).init.expect("non-null function pointer")((*a).filter);
        if r != ARCHIVE_OK {
            __archive_read_free_filters(a);
            return -(30 as libc::c_int);
        }
        number_filters += 1
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_FILE_FORMAT,
        b"Input requires too many filters for decoding\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn __archive_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    if (*(*a).filter).read_header.is_some() {
        return (*(*a).filter)
            .read_header
            .expect("non-null function pointer")((*a).filter, entry);
    } else {
        return 0 as libc::c_int;
    };
}
/*
 * Read header of next entry.
 */
unsafe extern "C" fn _archive_read_next_header2(
    mut _a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut r1: libc::c_int = ARCHIVE_OK;
    let mut r2: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_read_next_header\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    archive_entry_clear(entry);
    archive_clear_error(&mut (*a).archive);
    /*
     * If client didn't consume entire data, skip any remainder
     * (This is especially important for GNU incremental directories.)
     */
    if (*a).archive.state == ARCHIVE_STATE_DATA {
        r1 = archive_read_data_skip(&mut (*a).archive);
        if r1 == ARCHIVE_EOF {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EIO,
                b"Premature end-of-file.\x00" as *const u8 as *const libc::c_char,
            );
        }
        if r1 == ARCHIVE_EOF || r1 == ARCHIVE_FATAL {
            (*a).archive.state = ARCHIVE_STATE_FATAL;
            return -(30 as libc::c_int);
        }
    }
    /* Record start-of-header offset in uncompressed stream. */
    (*a).header_position = (*(*a).filter).position;
    (*_a).file_count += 1;
    r2 = (*(*a).format)
        .read_header
        .expect("non-null function pointer")(a, entry);
    /*
     * EOF and FATAL are persistent at this layer.  By
     * modifying the state, we guarantee that future calls to
     * read a header or read data will fail.
     */
    match r2 {
        ARCHIVE_EOF => {
            (*a).archive.state = ARCHIVE_STATE_EOF; /* Revert a file counter. */
            (*_a).file_count -= 1
        }
        ARCHIVE_OK => (*a).archive.state = ARCHIVE_STATE_DATA,
        -20 => (*a).archive.state = ARCHIVE_STATE_DATA,
        -30 => (*a).archive.state = ARCHIVE_STATE_FATAL,
        -10 | _ => {}
    }
    __archive_reset_read_data(&mut (*a).archive);
    (*a).data_start_node = (*a).client.cursor;
    /* EOF always wins; otherwise return the worst error. */
    return if r2 < r1 || r2 == ARCHIVE_EOF { r2 } else { r1 };
}
unsafe extern "C" fn _archive_read_next_header(
    mut _a: *mut archive,
    mut entryp: *mut *mut archive_entry,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut a: *mut archive_read = _a as *mut archive_read;
    *entryp = NULL as *mut archive_entry;
    ret = _archive_read_next_header2(_a, (*a).entry);
    *entryp = (*a).entry;
    return ret;
}
/*
 * Allow each registered format to bid on whether it wants to handle
 * the next entry.  Return index of winning bidder.
 */
unsafe extern "C" fn choose_format(mut a: *mut archive_read) -> libc::c_int {
    let mut slots: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bid: libc::c_int = 0;
    let mut best_bid: libc::c_int = 0;
    let mut best_bid_slot: libc::c_int = 0;
    slots = (::std::mem::size_of::<[archive_format_descriptor; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<archive_format_descriptor>() as libc::c_ulong)
        as libc::c_int;
    best_bid = -(1 as libc::c_int);
    best_bid_slot = -(1 as libc::c_int);
    /* Set up a->format for convenience of bidders. */
    (*a).format = &mut *(*a).formats.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut archive_format_descriptor;
    i = 0 as libc::c_int;
    while i < slots {
        if (*(*a).format).bid.is_some() {
            bid = (*(*a).format).bid.expect("non-null function pointer")(a, best_bid);
            if bid == ARCHIVE_FATAL {
                return -(30 as libc::c_int);
            }
            if (*(*a).filter).position != 0 as libc::c_int as libc::c_long {
                __archive_read_seek(a, 0 as libc::c_int as int64_t, SEEK_SET);
            }
            if bid > best_bid || best_bid_slot < 0 as libc::c_int {
                best_bid = bid;
                best_bid_slot = i
            }
        }
        i += 1;
        (*a).format = (*a).format.offset(1)
    }
    /*
     * There were no bidders; this is a serious programmer error
     * and demands a quick and definitive abort.
     */
    if best_bid_slot < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"No formats registered\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * There were bidders, but no non-zero bids; this means we
     * can't support this stream.
     */
    if best_bid < 1 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Unrecognized archive format\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return best_bid_slot;
}
/*
 * Return the file offset (within the uncompressed data stream) where
 * the last header started.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_header_position(mut _a: *mut archive) -> la_int64_t {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint),
        b"archive_read_header_position\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL as la_int64_t;
    }
    return (*a).header_position;
}
/*
 * Returns 1 if the archive contains at least one encrypted entry.
 * If the archive format not support encryption at all
 * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned.
 * If for any other reason (e.g. not enough data read so far)
 * we cannot say whether there are encrypted entries, then
 * ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW is returned.
 * In general, this function will return values below zero when the
 * reader is uncertain or totally incapable of encryption support.
 * When this function returns 0 you can be sure that the reader
 * supports encryption detection but no encrypted entries have
 * been found yet.
 *
 * NOTE: If the metadata/header of an archive is also encrypted, you
 * cannot rely on the number of encrypted entries. That is why this
 * function does not return the number of encrypted entries but#
 * just shows that there are some.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_has_encrypted_entries(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut format_supports_encryption: libc::c_int = archive_read_format_capabilities(_a)
        & (ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_DATA | ARCHIVE_READ_FORMAT_CAPS_ENCRYPT_METADATA);
    if _a.is_null() || format_supports_encryption == 0 {
        /* Format in general doesn't support encryption */
        return ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED;
    }
    /* A reader potentially has read enough data now. */
    if !(*a).format.is_null() && (*(*a).format).has_encrypted_entries.is_some() {
        return (*(*a).format)
            .has_encrypted_entries
            .expect("non-null function pointer")(a);
    }
    /* For any other reason we cannot say how many entries are there. */
    return ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW;
}
/*
 * Returns a bitmask of capabilities that are supported by the archive format reader.
 * If the reader has no special capabilities, ARCHIVE_READ_FORMAT_CAPS_NONE is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_format_capabilities(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    if !a.is_null() && !(*a).format.is_null() && (*(*a).format).format_capabilties.is_some() {
        return (*(*a).format)
            .format_capabilties
            .expect("non-null function pointer")(a);
    }
    return ARCHIVE_READ_FORMAT_CAPS_NONE;
}
/*
 * Read data from an archive entry, using a read(2)-style interface.
 * This is a convenience routine that just calls
 * archive_read_data_block and copies the results into the client
 * buffer, filling any gaps with zero bytes.  Clients using this
 * API can be completely ignorant of sparse-file issues; sparse files
 * will simply be padded with nulls.
 *
 * DO NOT intermingle calls to this function and archive_read_data_block
 * to read a single entry body.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_data(
    mut _a: *mut archive,
    mut buff: *mut libc::c_void,
    mut s: size_t,
) -> la_ssize_t {
    let mut a: *mut archive = _a;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut read_buf: *const libc::c_void = 0 as *const libc::c_void;
    let mut bytes_read: size_t = 0;
    let mut len: size_t = 0;
    let mut r: libc::c_int = 0;
    bytes_read = 0 as libc::c_int as size_t;
    dest = buff as *mut libc::c_char;
    while s > 0 as libc::c_int as libc::c_ulong {
        if (*a).read_data_offset == (*a).read_data_output_offset
            && (*a).read_data_remaining == 0 as libc::c_int as libc::c_ulong
        {
            read_buf = (*a).read_data_block as *const libc::c_void;
            (*a).read_data_is_posix_read = 1 as libc::c_int as libc::c_char;
            (*a).read_data_requested = s;
            r = archive_read_data_block(
                a,
                &mut read_buf,
                &mut (*a).read_data_remaining,
                &mut (*a).read_data_offset,
            );
            (*a).read_data_block = read_buf as *const libc::c_char;
            if r == ARCHIVE_EOF {
                return bytes_read as la_ssize_t;
            }
            /*
             * Error codes are all negative, so the status
             * return here cannot be confused with a valid
             * byte count.  (ARCHIVE_OK is zero.)
             */
            if r < ARCHIVE_OK {
                return r as la_ssize_t;
            }
        }
        if (*a).read_data_offset < (*a).read_data_output_offset {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Encountered out-of-order sparse blocks\x00" as *const u8 as *const libc::c_char,
            );
            return -(10 as libc::c_int) as la_ssize_t;
        }
        /* Compute the amount of zero padding needed. */
        if ((*a).read_data_output_offset + s as int64_t) < (*a).read_data_offset {
            len = s
        } else if (*a).read_data_output_offset < (*a).read_data_offset {
            len = ((*a).read_data_offset - (*a).read_data_output_offset) as size_t
        } else {
            len = 0 as libc::c_int as size_t
        }
        /* Add zeroes. */
        memset(dest as *mut libc::c_void, 0 as libc::c_int, len);
        s = (s as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        (*a).read_data_output_offset =
            ((*a).read_data_output_offset as libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
        dest = dest.offset(len as isize);
        bytes_read = (bytes_read as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        /* Copy data if there is any space left. */
        if s > 0 as libc::c_int as libc::c_ulong {
            len = (*a).read_data_remaining;
            if len > s {
                len = s
            }
            if len != 0 {
                memcpy(
                    dest as *mut libc::c_void,
                    (*a).read_data_block as *const libc::c_void,
                    len,
                );
                s = (s as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
                (*a).read_data_block = (*a).read_data_block.offset(len as isize);
                (*a).read_data_remaining = ((*a).read_data_remaining as libc::c_ulong)
                    .wrapping_sub(len) as size_t
                    as size_t;
                (*a).read_data_output_offset = ((*a).read_data_output_offset as libc::c_ulong)
                    .wrapping_add(len) as int64_t
                    as int64_t;
                (*a).read_data_offset = ((*a).read_data_offset as libc::c_ulong).wrapping_add(len)
                    as int64_t as int64_t;
                dest = dest.offset(len as isize);
                bytes_read = (bytes_read as libc::c_ulong).wrapping_add(len) as size_t as size_t
            }
        }
    }
    (*a).read_data_is_posix_read = 0 as libc::c_int as libc::c_char;
    (*a).read_data_requested = 0 as libc::c_int as size_t;
    return bytes_read as la_ssize_t;
}
/*
 * Reset the read_data_* variables, used for starting a new entry.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_reset_read_data(mut a: *mut archive) {
    (*a).read_data_output_offset = 0 as libc::c_int as int64_t;
    (*a).read_data_remaining = 0 as libc::c_int as size_t;
    (*a).read_data_is_posix_read = 0 as libc::c_int as libc::c_char;
    (*a).read_data_requested = 0 as libc::c_int as size_t;
    /* extra resets, from rar.c */
    (*a).read_data_block = NULL as *const libc::c_char;
    (*a).read_data_offset = 0 as libc::c_int as int64_t;
}
/*
 * Skip over all remaining data in this entry.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_data_skip(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut r: libc::c_int = 0;
    let mut buff: *const libc::c_void = 0 as *const libc::c_void;
    let mut size: size_t = 0;
    let mut offset: int64_t = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_read_data_skip\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*(*a).format).read_data_skip.is_some() {
        r = (*(*a).format)
            .read_data_skip
            .expect("non-null function pointer")(a)
    } else {
        loop {
            r = archive_read_data_block(&mut (*a).archive, &mut buff, &mut size, &mut offset);
            if !(r == ARCHIVE_OK) {
                break;
            }
        }
    }
    if r == ARCHIVE_EOF {
        r = ARCHIVE_OK
    }
    (*a).archive.state = ARCHIVE_STATE_HEADER;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn archive_seek_data(
    mut _a: *mut archive,
    mut offset: int64_t,
    mut whence: libc::c_int,
) -> la_int64_t {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_seek_data_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL as la_int64_t;
    }
    if (*(*a).format).seek_data.is_none() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Internal error: No format_seek_data_block function registered\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int) as la_int64_t;
    }
    return (*(*a).format).seek_data.expect("non-null function pointer")(a, offset, whence);
}
/*
 * Read the next block of entry data from the archive.
 * This is a zero-copy interface; the client receives a pointer,
 * size, and file offset of the next available block of data.
 *
 * Returns ARCHIVE_OK if the operation is successful, ARCHIVE_EOF if
 * the end of entry is encountered.
 */
unsafe extern "C" fn _archive_read_data_block(
    mut _a: *mut archive,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        4 as libc::c_uint,
        b"archive_read_data_block\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*(*a).format).read_data.is_none() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_PROGRAMMER,
            b"Internal error: No format->read_data function registered\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return (*(*a).format).read_data.expect("non-null function pointer")(a, buff, size, offset);
}
unsafe extern "C" fn close_filters(mut a: *mut archive_read) -> libc::c_int {
    let mut f: *mut archive_read_filter = (*a).filter;
    let mut r: libc::c_int = ARCHIVE_OK;
    /* Close each filter in the pipeline. */
    while !f.is_null() {
        let mut t: *mut archive_read_filter = (*f).upstream;
        if (*f).closed == 0 && (*f).close.is_some() {
            let mut r1: libc::c_int = (*f).close.expect("non-null function pointer")(f);
            (*f).closed = 1 as libc::c_int as libc::c_char;
            if r1 < r {
                r = r1
            }
        }
        free((*f).buffer as *mut libc::c_void);
        (*f).buffer = NULL as *mut libc::c_char;
        f = t
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_read_free_filters(mut a: *mut archive_read) {
    /* Make sure filters are closed and their buffers are freed */
    close_filters(a);
    while !(*a).filter.is_null() {
        let mut t: *mut archive_read_filter = (*(*a).filter).upstream;
        free((*a).filter as *mut libc::c_void);
        (*a).filter = t
    }
}
/*
 * return the count of # of filters in use
 */
unsafe extern "C" fn _archive_filter_count(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut p: *mut archive_read_filter = (*a).filter;
    let mut count: libc::c_int = 0 as libc::c_int;
    while !p.is_null() {
        count += 1;
        p = (*p).upstream
    }
    return count;
}
/*
 * Close the file and all I/O.
 */
unsafe extern "C" fn _archive_read_close(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut r: libc::c_int = ARCHIVE_OK;
    let mut r1: libc::c_int = ARCHIVE_OK;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xdeb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_read_close\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state == ARCHIVE_STATE_CLOSED {
        return 0 as libc::c_int;
    }
    archive_clear_error(&mut (*a).archive);
    (*a).archive.state = ARCHIVE_STATE_CLOSED;
    /* TODO: Clean up the formatters. */
    /* Release the filter objects. */
    r1 = close_filters(a);
    if r1 < r {
        r = r1
    }
    return r;
}
/*
 * Release memory and other resources.
 */
unsafe extern "C" fn _archive_read_free(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut p: *mut archive_read_passphrase = 0 as *mut archive_read_passphrase;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut slots: libc::c_int = 0;
    let mut r: libc::c_int = ARCHIVE_OK;
    if _a.is_null() {
        return 0 as libc::c_int;
    }
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        0xffff as libc::c_uint & !(0x8000 as libc::c_uint) | 0x8000 as libc::c_uint,
        b"archive_read_free\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.state != ARCHIVE_STATE_CLOSED && (*a).archive.state != ARCHIVE_STATE_FATAL {
        r = archive_read_close(&mut (*a).archive)
    }
    /* Call cleanup functions registered by optional components. */
    if (*a).cleanup_archive_extract.is_some() {
        r = (*a)
            .cleanup_archive_extract
            .expect("non-null function pointer")(a)
    }
    /* Cleanup format-specific data. */
    slots = (::std::mem::size_of::<[archive_format_descriptor; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<archive_format_descriptor>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < slots {
        (*a).format =
            &mut *(*a).formats.as_mut_ptr().offset(i as isize) as *mut archive_format_descriptor;
        if (*a).formats[i as usize].cleanup.is_some() {
            (*a).formats[i as usize]
                .cleanup
                .expect("non-null function pointer")(a);
        }
        i += 1
    }
    /* Free the filters */
    __archive_read_free_filters(a);
    /* Release the bidder objects. */
    n = (::std::mem::size_of::<[archive_read_filter_bidder; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<archive_read_filter_bidder>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if (*a).bidders[i as usize].free.is_some() {
            let mut r1: libc::c_int = (*a).bidders[i as usize]
                .free
                .expect("non-null function pointer")(
                &mut *(*a).bidders.as_mut_ptr().offset(i as isize),
            );
            if r1 < r {
                r = r1
            }
        }
        i += 1
    }
    /* Release passphrase list. */
    p = (*a).passphrases.first;
    while !p.is_null() {
        let mut np: *mut archive_read_passphrase = (*p).next;
        /* A passphrase should be cleaned. */
        memset(
            (*p).passphrase as *mut libc::c_void,
            0 as libc::c_int,
            strlen((*p).passphrase),
        );
        free((*p).passphrase as *mut libc::c_void);
        free(p as *mut libc::c_void);
        p = np
    }
    archive_string_free(&mut (*a).archive.error_string);
    archive_entry_free((*a).entry);
    (*a).archive.magic = 0 as libc::c_int as libc::c_uint;
    __archive_clean(&mut (*a).archive);
    free((*a).client.dataset as *mut libc::c_void);
    free(a as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn get_filter(
    mut _a: *mut archive,
    mut n: libc::c_int,
) -> *mut archive_read_filter {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut f: *mut archive_read_filter = (*a).filter;
    /* We use n == -1 for 'the last filter', which is always the
     * client proxy. */
    if n == -(1 as libc::c_int) && !f.is_null() {
        let mut last: *mut archive_read_filter = f;
        f = (*f).upstream;
        while !f.is_null() {
            last = f;
            f = (*f).upstream
        }
        return last;
    }
    if n < 0 as libc::c_int {
        return NULL as *mut archive_read_filter;
    }
    while n > 0 as libc::c_int && !f.is_null() {
        f = (*f).upstream;
        n -= 1
    }
    return f;
}
unsafe extern "C" fn _archive_filter_code(mut _a: *mut archive, mut n: libc::c_int) -> libc::c_int {
    let mut f: *mut archive_read_filter = get_filter(_a, n);
    return if f.is_null() {
        -(1 as libc::c_int)
    } else {
        (*f).code
    };
}
unsafe extern "C" fn _archive_filter_name(
    mut _a: *mut archive,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut f: *mut archive_read_filter = get_filter(_a, n);
    return if !f.is_null() {
        (*f).name
    } else {
        NULL as *const libc::c_char
    };
}
unsafe extern "C" fn _archive_filter_bytes(mut _a: *mut archive, mut n: libc::c_int) -> int64_t {
    let mut f: *mut archive_read_filter = get_filter(_a, n);
    return if f.is_null() {
        -(1 as libc::c_int) as libc::c_long
    } else {
        (*f).position
    };
}
/*
 * Used internally by read format handlers to register their bid and
 * initialization functions.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_read_register_format(
    mut a: *mut archive_read,
    mut format_data: *mut libc::c_void,
    mut name: *const libc::c_char,
    mut bid: Option<unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int>,
    mut options: Option<
        unsafe extern "C" fn(
            _: *mut archive_read,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    mut read_header: Option<
        unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
    >,
    mut read_data: Option<
        unsafe extern "C" fn(
            _: *mut archive_read,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    mut read_data_skip: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    mut seek_data: Option<
        unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
    >,
    mut cleanup: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    mut format_capabilities: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
    mut has_encrypted_entries: Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
) -> libc::c_int {
    let mut i: libc::c_int = 0; /* We've already installed */
    let mut number_slots: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"__archive_read_register_format\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    number_slots = (::std::mem::size_of::<[archive_format_descriptor; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<archive_format_descriptor>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_slots {
        if (*a).formats[i as usize].bid == bid {
            return -(20 as libc::c_int);
        }
        if (*a).formats[i as usize].bid.is_none() {
            (*a).formats[i as usize].bid = bid;
            (*a).formats[i as usize].options = options;
            (*a).formats[i as usize].read_header = read_header;
            (*a).formats[i as usize].read_data = read_data;
            (*a).formats[i as usize].read_data_skip = read_data_skip;
            (*a).formats[i as usize].seek_data = seek_data;
            (*a).formats[i as usize].cleanup = cleanup;
            (*a).formats[i as usize].data = format_data;
            (*a).formats[i as usize].name = name;
            (*a).formats[i as usize].format_capabilties = format_capabilities;
            (*a).formats[i as usize].has_encrypted_entries = has_encrypted_entries;
            return 0 as libc::c_int;
        }
        i += 1
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ENOMEM,
        b"Not enough slots for format registration\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
/*
 * Used internally by decompression routines to register their bid and
 * initialization functions.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_read_get_bidder(
    mut a: *mut archive_read,
    mut bidder: *mut *mut archive_read_filter_bidder,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut number_slots: libc::c_int = 0;
    number_slots = (::std::mem::size_of::<[archive_read_filter_bidder; 16]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<archive_read_filter_bidder>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_slots {
        if (*a).bidders[i as usize].bid.is_none() {
            memset(
                (*a).bidders.as_mut_ptr().offset(i as isize) as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<archive_read_filter_bidder>() as libc::c_ulong,
            );
            *bidder = (*a).bidders.as_mut_ptr().offset(i as isize);
            return 0 as libc::c_int;
        }
        i += 1
    }
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ENOMEM,
        b"Not enough slots for filter registration\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
/*
 * The next section implements the peek/consume internal I/O
 * system used by archive readers.  This system allows simple
 * read-ahead for consumers while preserving zero-copy operation
 * most of the time.
 *
 * The two key operations:
 *  * The read-ahead function returns a pointer to a block of data
 *    that satisfies a minimum request.
 *  * The consume function advances the file pointer.
 *
 * In the ideal case, filters generate blocks of data
 * and __archive_read_ahead() just returns pointers directly into
 * those blocks.  Then __archive_read_consume() just bumps those
 * pointers.  Only if your request would span blocks does the I/O
 * layer use a copy buffer to provide you with a contiguous block of
 * data.
 *
 * A couple of useful idioms:
 *  * "I just want some data."  Ask for 1 byte and pay attention to
 *    the "number of bytes available" from __archive_read_ahead().
 *    Consume whatever you actually use.
 *  * "I want to output a large block of data."  As above, ask for 1 byte,
 *    emit all that's available (up to whatever limit you have), consume
 *    it all, then repeat until you're done.  This effectively means that
 *    you're passing along the blocks that came from your provider.
 *  * "I want to peek ahead by a large amount."  Ask for 4k or so, then
 *    double and repeat until you get an error or have enough.  Note
 *    that the I/O layer will likely end up expanding its copy buffer
 *    to fit your request, so use this technique cautiously.  This
 *    technique is used, for example, by some of the format tasting
 *    code that has uncertain look-ahead needs.
 */
/*
 * Looks ahead in the input stream:
 *  * If 'avail' pointer is provided, that returns number of bytes available
 *    in the current buffer, which may be much larger than requested.
 *  * If end-of-file, *avail gets set to zero.
 *  * If error, *avail gets error code.
 *  * If request can be met, returns pointer to data.
 *  * If minimum request cannot be met, returns NULL.
 *
 * Note: If you just want "some data", ask for 1 byte and pay attention
 * to *avail, which will have the actual amount available.  If you
 * know exactly how many bytes you need, just ask for that and treat
 * a NULL return as an error.
 *
 * Important:  This does NOT move the file pointer.  See
 * __archive_read_consume() below.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_read_ahead(
    mut a: *mut archive_read,
    mut min: size_t,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    return __archive_read_filter_ahead((*a).filter, min, avail);
}
#[no_mangle]
pub unsafe extern "C" fn __archive_read_filter_ahead(
    mut filter: *mut archive_read_filter,
    mut min: size_t,
    mut avail: *mut ssize_t,
) -> *const libc::c_void {
    let mut bytes_read: ssize_t = 0;
    let mut tocopy: size_t = 0;
    if (*filter).fatal != 0 {
        if !avail.is_null() {
            *avail = ARCHIVE_FATAL as ssize_t
        }
        return 0 as *const libc::c_void;
    }
    loop
    /*
     * Keep pulling more data until we can satisfy the request.
     */
    /*
     * If we can satisfy from the copy buffer (and the
     * copy buffer isn't empty), we're done.  In particular,
     * note that min == 0 is a perfectly well-defined
     * request.
     */
    {
        if (*filter).avail >= min && (*filter).avail > 0 as libc::c_int as libc::c_ulong {
            if !avail.is_null() {
                *avail = (*filter).avail as ssize_t
            }
            return (*filter).next as *const libc::c_void;
        }
        /*
         * We can satisfy directly from client buffer if everything
         * currently in the copy buffer is still in the client buffer.
         */
        if (*filter).client_total >= (*filter).client_avail.wrapping_add((*filter).avail)
            && (*filter).client_avail.wrapping_add((*filter).avail) >= min
        {
            /* "Roll back" to client buffer. */
            (*filter).client_avail = ((*filter).client_avail as libc::c_ulong)
                .wrapping_add((*filter).avail) as size_t
                as size_t;
            (*filter).client_next = (*filter).client_next.offset(-((*filter).avail as isize));
            /* Copy buffer is now empty. */
            (*filter).avail = 0 as libc::c_int as size_t;
            (*filter).next = (*filter).buffer;
            /* Return data from client buffer. */
            if !avail.is_null() {
                *avail = (*filter).client_avail as ssize_t
            }
            return (*filter).client_next as *const libc::c_void;
        }
        /* Move data forward in copy buffer if necessary. */
        if (*filter).next > (*filter).buffer
            && (*filter).next.offset(min as isize)
                > (*filter).buffer.offset((*filter).buffer_size as isize)
        {
            if (*filter).avail > 0 as libc::c_int as libc::c_ulong {
                memmove(
                    (*filter).buffer as *mut libc::c_void,
                    (*filter).next as *const libc::c_void,
                    (*filter).avail,
                );
            }
            (*filter).next = (*filter).buffer
        }
        /* If we've used up the client data, get more. */
        if (*filter).client_avail <= 0 as libc::c_int as libc::c_ulong {
            if (*filter).end_of_file != 0 {
                if !avail.is_null() {
                    *avail = 0 as libc::c_int as ssize_t
                }
                return 0 as *const libc::c_void;
            }
            bytes_read = (*filter).read.expect("non-null function pointer")(
                filter,
                &mut (*filter).client_buff,
            );
            if bytes_read < 0 as libc::c_int as libc::c_long {
                /* Read error. */
                (*filter).client_avail = 0 as libc::c_int as size_t;
                (*filter).client_total = (*filter).client_avail;
                (*filter).client_buff = NULL as *const libc::c_void;
                (*filter).client_next = (*filter).client_buff as *const libc::c_char;
                (*filter).fatal = 1 as libc::c_int as libc::c_char;
                if !avail.is_null() {
                    *avail = ARCHIVE_FATAL as ssize_t
                }
                return 0 as *const libc::c_void;
            }
            if bytes_read == 0 as libc::c_int as libc::c_long {
                /* Check for another client object first */
                if (*(*filter).archive).client.cursor
                    != (*(*filter).archive)
                        .client
                        .nodes
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    if client_switch_proxy(
                        filter,
                        (*(*filter).archive)
                            .client
                            .cursor
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    ) == ARCHIVE_OK
                    {
                        continue;
                    }
                }
                /* Premature end-of-file. */
                (*filter).client_avail = 0 as libc::c_int as size_t;
                (*filter).client_total = (*filter).client_avail;
                (*filter).client_buff = NULL as *const libc::c_void;
                (*filter).client_next = (*filter).client_buff as *const libc::c_char;
                (*filter).end_of_file = 1 as libc::c_int as libc::c_char;
                /* Return whatever we do have. */
                if !avail.is_null() {
                    *avail = (*filter).avail as ssize_t
                }
                return 0 as *const libc::c_void;
            } else {
                (*filter).client_total = bytes_read as size_t;
                (*filter).client_avail = (*filter).client_total;
                (*filter).client_next = (*filter).client_buff as *const libc::c_char
            }
        } else {
            /*
             * We can't satisfy the request from the copy
             * buffer or the existing client data, so we
             * need to copy more client data over to the
             * copy buffer.
             */
            /* Ensure the buffer is big enough. */
            if min > (*filter).buffer_size {
                let mut s: size_t = 0;
                let mut t: size_t = 0;
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                /* Double the buffer; watch for overflow. */
                t = (*filter).buffer_size;
                s = t;
                if s == 0 as libc::c_int as libc::c_ulong {
                    s = min
                }
                while s < min {
                    t = (t as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        as size_t as size_t;
                    if t <= s {
                        /* Integer overflow! */
                        archive_set_error(
                            &mut (*(*filter).archive).archive as *mut archive,
                            ENOMEM,
                            b"Unable to allocate copy buffer\x00" as *const u8
                                as *const libc::c_char,
                        );
                        (*filter).fatal = 1 as libc::c_int as libc::c_char;
                        if !avail.is_null() {
                            *avail = ARCHIVE_FATAL as ssize_t
                        }
                        return 0 as *const libc::c_void;
                    }
                    s = t
                }
                /* Now s >= min, so allocate a new buffer. */
                p = malloc(s) as *mut libc::c_char;
                if p.is_null() {
                    archive_set_error(
                        &mut (*(*filter).archive).archive as *mut archive,
                        ENOMEM,
                        b"Unable to allocate copy buffer\x00" as *const u8 as *const libc::c_char,
                    );
                    (*filter).fatal = 1 as libc::c_int as libc::c_char;
                    if !avail.is_null() {
                        *avail = ARCHIVE_FATAL as ssize_t
                    }
                    return 0 as *const libc::c_void;
                }
                /* Move data into newly-enlarged buffer. */
                if (*filter).avail > 0 as libc::c_int as libc::c_ulong {
                    memmove(
                        p as *mut libc::c_void,
                        (*filter).next as *const libc::c_void,
                        (*filter).avail,
                    );
                }
                free((*filter).buffer as *mut libc::c_void);
                (*filter).buffer = p;
                (*filter).next = (*filter).buffer;
                (*filter).buffer_size = s
            }
            /* We can add client data to copy buffer. */
            /* First estimate: copy to fill rest of buffer. */
            tocopy = (*filter)
                .buffer
                .offset((*filter).buffer_size as isize)
                .offset_from((*filter).next.offset((*filter).avail as isize))
                as libc::c_long as size_t;
            /* Don't waste time buffering more than we need to. */
            if tocopy.wrapping_add((*filter).avail) > min {
                tocopy = min.wrapping_sub((*filter).avail)
            }
            /* Don't copy more than is available. */
            if tocopy > (*filter).client_avail {
                tocopy = (*filter).client_avail
            }
            memcpy(
                (*filter).next.offset((*filter).avail as isize) as *mut libc::c_void,
                (*filter).client_next as *const libc::c_void,
                tocopy,
            );
            /* Remove this data from client buffer. */
            (*filter).client_next = (*filter).client_next.offset(tocopy as isize);
            (*filter).client_avail =
                ((*filter).client_avail as libc::c_ulong).wrapping_sub(tocopy) as size_t as size_t;
            /* add it to copy buffer. */
            (*filter).avail =
                ((*filter).avail as libc::c_ulong).wrapping_add(tocopy) as size_t as size_t
        }
    }
}
/*
 * Move the file pointer forward.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_read_consume(
    mut a: *mut archive_read,
    mut request: int64_t,
) -> int64_t {
    return __archive_read_filter_consume((*a).filter, request);
}
#[no_mangle]
pub unsafe extern "C" fn __archive_read_filter_consume(
    mut filter: *mut archive_read_filter,
    mut request: int64_t,
) -> int64_t {
    let mut skipped: int64_t = 0;
    if request < 0 as libc::c_int as libc::c_long {
        return ARCHIVE_FATAL as int64_t;
    }
    if request == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as int64_t;
    }
    skipped = advance_file_pointer(filter, request);
    if skipped == request {
        return skipped;
    }
    /* We hit EOF before we satisfied the skip request. */
    if skipped < 0 as libc::c_int as libc::c_long {
        /* Map error code to 0 for error message below. */
        skipped = 0 as libc::c_int as int64_t
    }
    archive_set_error(
        &mut (*(*filter).archive).archive as *mut archive,
        ARCHIVE_ERRNO_MISC,
        b"Truncated input file (needed %jd bytes, only %jd available)\x00" as *const u8
            as *const libc::c_char,
        request,
        skipped,
    );
    return -(30 as libc::c_int) as int64_t;
}
/*
 * Advance the file pointer by the amount requested.
 * Returns the amount actually advanced, which may be less than the
 * request if EOF is encountered first.
 * Returns a negative value if there's an I/O error.
 */
unsafe extern "C" fn advance_file_pointer(
    mut filter: *mut archive_read_filter,
    mut request: int64_t,
) -> int64_t {
    let mut bytes_skipped: int64_t = 0;
    let mut total_bytes_skipped: int64_t = 0 as libc::c_int as int64_t;
    let mut bytes_read: ssize_t = 0;
    let mut min: size_t = 0;
    if (*filter).fatal != 0 {
        return -(1 as libc::c_int) as int64_t;
    }
    /* Use up the copy buffer first. */
    if (*filter).avail > 0 as libc::c_int as libc::c_ulong {
        min = if request < (*filter).avail as int64_t {
            request
        } else {
            (*filter).avail as int64_t
        } as size_t;
        (*filter).next = (*filter).next.offset(min as isize);
        (*filter).avail = ((*filter).avail as libc::c_ulong).wrapping_sub(min) as size_t as size_t;
        request = (request as libc::c_ulong).wrapping_sub(min) as int64_t as int64_t;
        (*filter).position =
            ((*filter).position as libc::c_ulong).wrapping_add(min) as int64_t as int64_t;
        total_bytes_skipped =
            (total_bytes_skipped as libc::c_ulong).wrapping_add(min) as int64_t as int64_t
    }
    /* Then use up the client buffer. */
    if (*filter).client_avail > 0 as libc::c_int as libc::c_ulong {
        min = if request < (*filter).client_avail as int64_t {
            request
        } else {
            (*filter).client_avail as int64_t
        } as size_t;
        (*filter).client_next = (*filter).client_next.offset(min as isize);
        (*filter).client_avail =
            ((*filter).client_avail as libc::c_ulong).wrapping_sub(min) as size_t as size_t;
        request = (request as libc::c_ulong).wrapping_sub(min) as int64_t as int64_t;
        (*filter).position =
            ((*filter).position as libc::c_ulong).wrapping_add(min) as int64_t as int64_t;
        total_bytes_skipped =
            (total_bytes_skipped as libc::c_ulong).wrapping_add(min) as int64_t as int64_t
    }
    if request == 0 as libc::c_int as libc::c_long {
        return total_bytes_skipped;
    }
    /* If there's an optimized skip function, use it. */
    if (*filter).skip.is_some() {
        bytes_skipped = (*filter).skip.expect("non-null function pointer")(filter, request);
        if bytes_skipped < 0 as libc::c_int as libc::c_long {
            /* error */
            (*filter).fatal = 1 as libc::c_int as libc::c_char;
            return bytes_skipped;
        }
        (*filter).position += bytes_skipped;
        total_bytes_skipped += bytes_skipped;
        request -= bytes_skipped;
        if request == 0 as libc::c_int as libc::c_long {
            return total_bytes_skipped;
        }
    }
    loop
    /* Use ordinary reads as necessary to complete the request. */
    {
        bytes_read =
            (*filter).read.expect("non-null function pointer")(filter, &mut (*filter).client_buff);
        if bytes_read < 0 as libc::c_int as libc::c_long {
            (*filter).client_buff = NULL as *const libc::c_void;
            (*filter).fatal = 1 as libc::c_int as libc::c_char;
            return bytes_read;
        }
        if bytes_read == 0 as libc::c_int as libc::c_long {
            if (*(*filter).archive).client.cursor
                != (*(*filter).archive)
                    .client
                    .nodes
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                if client_switch_proxy(
                    filter,
                    (*(*filter).archive)
                        .client
                        .cursor
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                ) == ARCHIVE_OK
                {
                    continue;
                }
            }
            (*filter).client_buff = NULL as *const libc::c_void;
            (*filter).end_of_file = 1 as libc::c_int as libc::c_char;
            return total_bytes_skipped;
        } else {
            if bytes_read >= request {
                (*filter).client_next =
                    ((*filter).client_buff as *const libc::c_char).offset(request as isize);
                (*filter).client_avail = (bytes_read - request) as size_t;
                (*filter).client_total = bytes_read as size_t;
                total_bytes_skipped += request;
                (*filter).position += request;
                return total_bytes_skipped;
            }
            (*filter).position += bytes_read;
            total_bytes_skipped += bytes_read;
            request -= bytes_read
        }
    }
}
/* *
 * Returns ARCHIVE_FAILED if seeking isn't supported.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_read_seek(
    mut a: *mut archive_read,
    mut offset: int64_t,
    mut whence: libc::c_int,
) -> int64_t {
    return __archive_read_filter_seek((*a).filter, offset, whence);
}
#[no_mangle]
pub unsafe extern "C" fn __archive_read_filter_seek(
    mut filter: *mut archive_read_filter,
    mut offset: int64_t,
    mut whence: libc::c_int,
) -> int64_t {
    let mut client: *mut archive_read_client = 0 as *mut archive_read_client;
    let mut r: int64_t = 0;
    let mut cursor: libc::c_uint = 0;
    if (*filter).closed as libc::c_int != 0 || (*filter).fatal as libc::c_int != 0 {
        return -(30 as libc::c_int) as int64_t;
    }
    if (*filter).seek.is_none() {
        return -(25 as libc::c_int) as int64_t;
    }
    client = &mut (*(*filter).archive).client;
    let mut current_block_48: u64;
    match whence {
        SEEK_CUR => {
            /* Adjust the offset and use SEEK_SET instead */
            offset += (*filter).position;
            current_block_48 = 9231253800968817275;
        }
        SEEK_SET => {
            current_block_48 = 9231253800968817275;
        }
        SEEK_END => {
            cursor = 0 as libc::c_int as libc::c_uint;
            while !((*(*client).dataset.offset(cursor as isize)).begin_position
                < 0 as libc::c_int as libc::c_long
                || (*(*client).dataset.offset(cursor as isize)).total_size
                    < 0 as libc::c_int as libc::c_long
                || cursor.wrapping_add(1 as libc::c_int as libc::c_uint) >= (*client).nodes)
            {
                r = (*(*client).dataset.offset(cursor as isize)).begin_position
                    + (*(*client).dataset.offset(cursor as isize)).total_size;
                cursor = cursor.wrapping_add(1);
                (*(*client).dataset.offset(cursor as isize)).begin_position = r
            }
            loop {
                r = client_switch_proxy(filter, cursor) as int64_t;
                if r != ARCHIVE_OK as libc::c_long {
                    return r;
                }
                r = client_seek_proxy(filter, 0 as libc::c_int as int64_t, SEEK_END);
                if r < 0 as libc::c_int as libc::c_long {
                    return r;
                }
                (*(*client).dataset.offset(cursor as isize)).total_size = r;
                r = (*(*client).dataset.offset(cursor as isize)).begin_position
                    + (*(*client).dataset.offset(cursor as isize)).total_size;
                if cursor.wrapping_add(1 as libc::c_int as libc::c_uint) >= (*client).nodes {
                    break;
                }
                cursor = cursor.wrapping_add(1);
                (*(*client).dataset.offset(cursor as isize)).begin_position = r
            }
            while !(r + offset >= (*(*client).dataset.offset(cursor as isize)).begin_position) {
                offset += (*(*client).dataset.offset(cursor as isize)).total_size;
                if cursor == 0 as libc::c_int as libc::c_uint {
                    break;
                }
                cursor = cursor.wrapping_sub(1);
                r = (*(*client).dataset.offset(cursor as isize)).begin_position
                    + (*(*client).dataset.offset(cursor as isize)).total_size
            }
            offset = r + offset - (*(*client).dataset.offset(cursor as isize)).begin_position;
            r = client_switch_proxy(filter, cursor) as int64_t;
            if r != ARCHIVE_OK as libc::c_long {
                return r;
            }
            r = client_seek_proxy(filter, offset, SEEK_SET);
            if r < ARCHIVE_OK as libc::c_long {
                return r;
            }
            current_block_48 = 6072622540298447352;
        }
        _ => return -(30 as libc::c_int) as int64_t,
    }
    match current_block_48 {
        9231253800968817275 => {
            cursor = 0 as libc::c_int as libc::c_uint;
            while !((*(*client).dataset.offset(cursor as isize)).begin_position
                < 0 as libc::c_int as libc::c_long
                || (*(*client).dataset.offset(cursor as isize)).total_size
                    < 0 as libc::c_int as libc::c_long
                || (*(*client).dataset.offset(cursor as isize)).begin_position
                    + (*(*client).dataset.offset(cursor as isize)).total_size
                    - 1 as libc::c_int as libc::c_long
                    > offset
                || cursor.wrapping_add(1 as libc::c_int as libc::c_uint) >= (*client).nodes)
            {
                r = (*(*client).dataset.offset(cursor as isize)).begin_position
                    + (*(*client).dataset.offset(cursor as isize)).total_size;
                cursor = cursor.wrapping_add(1);
                (*(*client).dataset.offset(cursor as isize)).begin_position = r
            }
            loop {
                r = client_switch_proxy(filter, cursor) as int64_t;
                if r != ARCHIVE_OK as libc::c_long {
                    return r;
                }
                r = client_seek_proxy(filter, 0 as libc::c_int as int64_t, SEEK_END);
                if r < 0 as libc::c_int as libc::c_long {
                    return r;
                }
                (*(*client).dataset.offset(cursor as isize)).total_size = r;
                if (*(*client).dataset.offset(cursor as isize)).begin_position
                    + (*(*client).dataset.offset(cursor as isize)).total_size
                    - 1 as libc::c_int as libc::c_long
                    > offset
                    || cursor.wrapping_add(1 as libc::c_int as libc::c_uint) >= (*client).nodes
                {
                    break;
                }
                r = (*(*client).dataset.offset(cursor as isize)).begin_position
                    + (*(*client).dataset.offset(cursor as isize)).total_size;
                cursor = cursor.wrapping_add(1);
                (*(*client).dataset.offset(cursor as isize)).begin_position = r
            }
            offset -= (*(*client).dataset.offset(cursor as isize)).begin_position;
            if offset < 0 as libc::c_int as libc::c_long
                || offset > (*(*client).dataset.offset(cursor as isize)).total_size
            {
                return ARCHIVE_FATAL as int64_t;
            }
            r = client_seek_proxy(filter, offset, SEEK_SET);
            if r < 0 as libc::c_int as libc::c_long {
                return r;
            }
        }
        _ => {}
    }
    r += (*(*client).dataset.offset(cursor as isize)).begin_position;
    if r >= 0 as libc::c_int as libc::c_long {
        /*
         * Ouch.  Clearing the buffer like this hurts, especially
         * at bid time.  A lot of our efficiency at bid time comes
         * from having bidders reuse the data we've already read.
         *
         * TODO: If the seek request is in data we already
         * have, then don't call the seek callback.
         *
         * TODO: Zip seeks to end-of-file at bid time.  If
         * other formats also start doing this, we may need to
         * find a way for clients to fudge the seek offset to
         * a block boundary.
         *
         * Hmmm... If whence was SEEK_END, we know the file
         * size is (r - offset).  Can we use that to simplify
         * the TODO items above?
         */
        (*filter).client_avail = 0 as libc::c_int as size_t;
        (*filter).avail = (*filter).client_avail;
        (*filter).next = (*filter).buffer;
        (*filter).position = r;
        (*filter).end_of_file = 0 as libc::c_int as libc::c_char
    }
    return r;
}
