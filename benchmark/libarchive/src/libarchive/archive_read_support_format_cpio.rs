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
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_dev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_ino64(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_dev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_devmajor(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_devminor(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_copy_hardlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_ino(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_rdev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_rdevmajor(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_rdevminor(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
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
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_default_conversion_for_read(_: *mut archive) -> *mut archive_string_conv;
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
    fn __archive_read_ahead(
        _: *mut archive_read,
        _: size_t,
        _: *mut ssize_t,
    ) -> *const libc::c_void;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub struct cpio {
    pub magic: libc::c_int,
    pub read_header: Option<
        unsafe extern "C" fn(
            _: *mut archive_read,
            _: *mut cpio,
            _: *mut archive_entry,
            _: *mut size_t,
            _: *mut size_t,
        ) -> libc::c_int,
    >,
    pub links_head: *mut links_entry,
    pub entry_bytes_remaining: int64_t,
    pub entry_bytes_unconsumed: int64_t,
    pub entry_offset: int64_t,
    pub entry_padding: int64_t,
    pub opt_sconv: *mut archive_string_conv,
    pub sconv_default: *mut archive_string_conv,
    pub init_default_conversion: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct links_entry {
    pub next: *mut links_entry,
    pub previous: *mut links_entry,
    pub links: libc::c_uint,
    pub dev: dev_t,
    pub ino: int64_t,
    pub name: *mut libc::c_char,
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
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FORMAT_CPIO_POSIX: libc::c_int = ARCHIVE_FORMAT_CPIO | 1 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_AFIO_LARGE: libc::c_int = ARCHIVE_FORMAT_CPIO | 6 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_SVR4_NOCRC: libc::c_int = ARCHIVE_FORMAT_CPIO | 4 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_SVR4_CRC: libc::c_int = ARCHIVE_FORMAT_CPIO | 5 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_BIN_BE: libc::c_int = ARCHIVE_FORMAT_CPIO | 3 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_BIN_LE: libc::c_int = ARCHIVE_FORMAT_CPIO | 2 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FORMAT_CPIO: libc::c_int = 0x10000 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
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
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
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
/* #include <stdint.h> */
/* See archive_platform.h */
pub const bin_dev_offset: libc::c_int = 2 as libc::c_int;
pub const bin_ino_offset: libc::c_int = 4 as libc::c_int;
pub const bin_mode_offset: libc::c_int = 6 as libc::c_int;
pub const bin_uid_offset: libc::c_int = 8 as libc::c_int;
pub const bin_gid_offset: libc::c_int = 10 as libc::c_int;
pub const bin_nlink_offset: libc::c_int = 12 as libc::c_int;
pub const bin_rdev_offset: libc::c_int = 14 as libc::c_int;
pub const bin_mtime_offset: libc::c_int = 16 as libc::c_int;
pub const bin_namesize_offset: libc::c_int = 20 as libc::c_int;
pub const bin_filesize_offset: libc::c_int = 22 as libc::c_int;
pub const bin_header_size: libc::c_int = 26 as libc::c_int;
pub const odc_dev_offset: libc::c_int = 6 as libc::c_int;
pub const odc_dev_size: libc::c_int = 6 as libc::c_int;
pub const odc_ino_offset: libc::c_int = 12 as libc::c_int;
pub const odc_ino_size: libc::c_int = 6 as libc::c_int;
pub const odc_mode_offset: libc::c_int = 18 as libc::c_int;
pub const odc_mode_size: libc::c_int = 6 as libc::c_int;
pub const odc_uid_offset: libc::c_int = 24 as libc::c_int;
pub const odc_uid_size: libc::c_int = 6 as libc::c_int;
pub const odc_gid_offset: libc::c_int = 30 as libc::c_int;
pub const odc_gid_size: libc::c_int = 6 as libc::c_int;
pub const odc_nlink_offset: libc::c_int = 36 as libc::c_int;
pub const odc_nlink_size: libc::c_int = 6 as libc::c_int;
pub const odc_rdev_offset: libc::c_int = 42 as libc::c_int;
pub const odc_rdev_size: libc::c_int = 6 as libc::c_int;
pub const odc_mtime_offset: libc::c_int = 48 as libc::c_int;
pub const odc_mtime_size: libc::c_int = 11 as libc::c_int;
pub const odc_namesize_offset: libc::c_int = 59 as libc::c_int;
pub const odc_namesize_size: libc::c_int = 6 as libc::c_int;
pub const odc_filesize_offset: libc::c_int = 65 as libc::c_int;
pub const odc_filesize_size: libc::c_int = 11 as libc::c_int;
pub const odc_header_size: libc::c_int = 76 as libc::c_int;
pub const newc_magic_offset: libc::c_int = 0 as libc::c_int;
pub const newc_ino_offset: libc::c_int = 6 as libc::c_int;
pub const newc_ino_size: libc::c_int = 8 as libc::c_int;
pub const newc_mode_offset: libc::c_int = 14 as libc::c_int;
pub const newc_mode_size: libc::c_int = 8 as libc::c_int;
pub const newc_uid_offset: libc::c_int = 22 as libc::c_int;
pub const newc_uid_size: libc::c_int = 8 as libc::c_int;
pub const newc_gid_offset: libc::c_int = 30 as libc::c_int;
pub const newc_gid_size: libc::c_int = 8 as libc::c_int;
pub const newc_nlink_offset: libc::c_int = 38 as libc::c_int;
pub const newc_nlink_size: libc::c_int = 8 as libc::c_int;
pub const newc_mtime_offset: libc::c_int = 46 as libc::c_int;
pub const newc_mtime_size: libc::c_int = 8 as libc::c_int;
pub const newc_filesize_offset: libc::c_int = 54 as libc::c_int;
pub const newc_filesize_size: libc::c_int = 8 as libc::c_int;
pub const newc_devmajor_offset: libc::c_int = 62 as libc::c_int;
pub const newc_devmajor_size: libc::c_int = 8 as libc::c_int;
pub const newc_devminor_offset: libc::c_int = 70 as libc::c_int;
pub const newc_devminor_size: libc::c_int = 8 as libc::c_int;
pub const newc_rdevmajor_offset: libc::c_int = 78 as libc::c_int;
pub const newc_rdevmajor_size: libc::c_int = 8 as libc::c_int;
pub const newc_rdevminor_offset: libc::c_int = 86 as libc::c_int;
pub const newc_rdevminor_size: libc::c_int = 8 as libc::c_int;
pub const newc_namesize_offset: libc::c_int = 94 as libc::c_int;
pub const newc_namesize_size: libc::c_int = 8 as libc::c_int;
pub const newc_header_size: libc::c_int = 110 as libc::c_int;
/*
 * An afio large ASCII header, which they named itself.
 * afio utility uses this header, if a file size is larger than 2G bytes
 * or inode/uid/gid is bigger than 65535(0xFFFF) or mtime is bigger than
 * 0x7fffffff, which we cannot record to odc header because of its limit.
 * If not, uses odc header.
 */
pub const afiol_dev_offset: libc::c_int = 6 as libc::c_int;
pub const afiol_dev_size: libc::c_int = 8 as libc::c_int;
/* hex */
pub const afiol_ino_offset: libc::c_int = 14 as libc::c_int;
pub const afiol_ino_size: libc::c_int = 16 as libc::c_int;
/* hex */
pub const afiol_ino_m_offset: libc::c_int = 30 as libc::c_int;
/* 'm' */
pub const afiol_mode_offset: libc::c_int = 31 as libc::c_int;
pub const afiol_mode_size: libc::c_int = 6 as libc::c_int;
/* oct */
pub const afiol_uid_offset: libc::c_int = 37 as libc::c_int;
pub const afiol_uid_size: libc::c_int = 8 as libc::c_int;
/* hex */
pub const afiol_gid_offset: libc::c_int = 45 as libc::c_int;
pub const afiol_gid_size: libc::c_int = 8 as libc::c_int;
/* hex */
pub const afiol_nlink_offset: libc::c_int = 53 as libc::c_int;
pub const afiol_nlink_size: libc::c_int = 8 as libc::c_int;
/* hex */
pub const afiol_rdev_offset: libc::c_int = 61 as libc::c_int;
pub const afiol_rdev_size: libc::c_int = 8 as libc::c_int;
/* hex */
pub const afiol_mtime_offset: libc::c_int = 69 as libc::c_int;
pub const afiol_mtime_size: libc::c_int = 16 as libc::c_int;
/* hex */
pub const afiol_mtime_n_offset: libc::c_int = 85 as libc::c_int;
/* 'n' */
pub const afiol_namesize_offset: libc::c_int = 86 as libc::c_int;
pub const afiol_namesize_size: libc::c_int = 4 as libc::c_int;
/* hex */
/* hex */
/* hex */
pub const afiol_xsize_s_offset: libc::c_int = 98 as libc::c_int;
/* 's' */
pub const afiol_filesize_offset: libc::c_int = 99 as libc::c_int;
pub const afiol_filesize_size: libc::c_int = 16 as libc::c_int;
/* hex */
pub const afiol_filesize_c_offset: libc::c_int = 115 as libc::c_int;
/* ':' */
pub const afiol_header_size: libc::c_int = 116 as libc::c_int;
pub const CPIO_MAGIC: libc::c_int = 0x13141516 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_cpio(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_cpio\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    cpio = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<cpio>() as libc::c_ulong,
    ) as *mut cpio;
    if cpio.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate cpio data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*cpio).magic = CPIO_MAGIC;
    r = __archive_read_register_format(
        a,
        cpio as *mut libc::c_void,
        b"cpio\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_cpio_bid
                as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int,
        ),
        Some(
            archive_read_format_cpio_options
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *const libc::c_char,
                    _: *const libc::c_char,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_cpio_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_cpio_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_cpio_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_cpio_cleanup
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        >(NULL as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int>,
        >(NULL as libc::intptr_t),
    );
    if r != ARCHIVE_OK {
        free(cpio as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_cpio_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut bid: libc::c_int = 0;
    /* UNUSED */
    cpio = (*(*a).format).data as *mut cpio;
    p = __archive_read_ahead(a, 6 as libc::c_int as size_t, NULL as *mut ssize_t)
        as *const libc::c_uchar;
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    bid = 0 as libc::c_int;
    if memcmp(
        p as *const libc::c_void,
        b"070707\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        /* ASCII cpio archive (odc, POSIX.1) */
        (*cpio).read_header = Some(
            header_odc
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut cpio,
                    _: *mut archive_entry,
                    _: *mut size_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        );
        bid += 48 as libc::c_int
    /*
     * XXX TODO:  More verification; Could check that only octal
     * digits appear in appropriate header locations. XXX
     */
    } else if memcmp(
        p as *const libc::c_void,
        b"070727\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        /* afio large ASCII cpio archive */
        (*cpio).read_header = Some(
            header_odc
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut cpio,
                    _: *mut archive_entry,
                    _: *mut size_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        );
        bid += 48 as libc::c_int
    /*
     * XXX TODO:  More verification; Could check that almost hex
     * digits appear in appropriate header locations. XXX
     */
    } else if memcmp(
        p as *const libc::c_void,
        b"070701\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        /* ASCII cpio archive (SVR4 without CRC) */
        (*cpio).read_header = Some(
            header_newc
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut cpio,
                    _: *mut archive_entry,
                    _: *mut size_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        );
        bid += 48 as libc::c_int
    /*
     * XXX TODO:  More verification; Could check that only hex
     * digits appear in appropriate header locations. XXX
     */
    } else if memcmp(
        p as *const libc::c_void,
        b"070702\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        /* ASCII cpio archive (SVR4 with CRC) */
        /* XXX TODO: Flag that we should check the CRC. XXX */
        (*cpio).read_header = Some(
            header_newc
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut cpio,
                    _: *mut archive_entry,
                    _: *mut size_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        );
        bid += 48 as libc::c_int
    /*
     * XXX TODO:  More verification; Could check that only hex
     * digits appear in appropriate header locations. XXX
     */
    } else if *p.offset(0 as libc::c_int as isize) as libc::c_int * 256 as libc::c_int
        + *p.offset(1 as libc::c_int as isize) as libc::c_int
        == 0o70707 as libc::c_int
    {
        /* big-endian binary cpio archives */
        (*cpio).read_header = Some(
            header_bin_be
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut cpio,
                    _: *mut archive_entry,
                    _: *mut size_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        );
        bid += 16 as libc::c_int
    /* Is more verification possible here? */
    } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
        + *p.offset(1 as libc::c_int as isize) as libc::c_int * 256 as libc::c_int
        == 0o70707 as libc::c_int
    {
        /* little-endian binary cpio archives */
        (*cpio).read_header = Some(
            header_bin_le
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut cpio,
                    _: *mut archive_entry,
                    _: *mut size_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        );
        bid += 16 as libc::c_int
    /* Is more verification possible here? */
    } else {
        return -(20 as libc::c_int);
    }
    return bid;
}
unsafe extern "C" fn archive_read_format_cpio_options(
    mut a: *mut archive_read,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    cpio = (*(*a).format).data as *mut cpio;
    if strcmp(key, b"compat-2x\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /* Handle filenames as libarchive 2.x */
        (*cpio).init_default_conversion = if !val.is_null() {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        return 0 as libc::c_int;
    } else {
        if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            if val.is_null()
                || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"cpio: hdrcharset option needs a character-set name\x00" as *const u8
                        as *const libc::c_char,
                );
            } else {
                (*cpio).opt_sconv = archive_string_conversion_from_charset(
                    &mut (*a).archive,
                    val,
                    0 as libc::c_int,
                );
                if !(*cpio).opt_sconv.is_null() {
                    ret = ARCHIVE_OK
                } else {
                    ret = ARCHIVE_FATAL
                }
            }
            return ret;
        }
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn archive_read_format_cpio_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut hl: *const libc::c_void = 0 as *const libc::c_void;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut namelength: size_t = 0;
    let mut name_pad: size_t = 0;
    let mut r: libc::c_int = 0;
    cpio = (*(*a).format).data as *mut cpio;
    sconv = (*cpio).opt_sconv;
    if sconv.is_null() {
        if (*cpio).init_default_conversion == 0 {
            (*cpio).sconv_default = archive_string_default_conversion_for_read(&mut (*a).archive);
            (*cpio).init_default_conversion = 1 as libc::c_int
        }
        sconv = (*cpio).sconv_default
    }
    r = (*cpio).read_header.expect("non-null function pointer")(
        a,
        cpio,
        entry,
        &mut namelength,
        &mut name_pad,
    );
    if r < ARCHIVE_WARN {
        return r;
    }
    /* Read name from buffer. */
    h = __archive_read_ahead(a, namelength.wrapping_add(name_pad), NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    if _archive_entry_copy_pathname_l(entry, h as *const libc::c_char, namelength, sconv)
        != 0 as libc::c_int
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
            b"Pathname can\'t be converted from %s to current locale.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name(sconv),
        );
        r = ARCHIVE_WARN
    }
    (*cpio).entry_offset = 0 as libc::c_int as int64_t;
    __archive_read_consume(a, namelength.wrapping_add(name_pad) as int64_t);
    /* If this is a symlink, read the link contents. */
    if archive_entry_filetype(entry) == AE_IFLNK as mode_t {
        if (*cpio).entry_bytes_remaining
            > (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_long
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Rejecting malformed cpio archive: symlink contents exceed 1 megabyte\x00"
                    as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        hl = __archive_read_ahead(
            a,
            (*cpio).entry_bytes_remaining as size_t,
            NULL as *mut ssize_t,
        );
        if hl == NULL as *const libc::c_void {
            return -(30 as libc::c_int);
        }
        if _archive_entry_copy_symlink_l(
            entry,
            hl as *const libc::c_char,
            (*cpio).entry_bytes_remaining as size_t,
            sconv,
        ) != 0 as libc::c_int
        {
            if errno == ENOMEM {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate memory for Linkname\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Linkname can\'t be converted from %s to current locale.\x00" as *const u8
                    as *const libc::c_char,
                archive_string_conversion_charset_name(sconv),
            );
            r = ARCHIVE_WARN
        }
        __archive_read_consume(a, (*cpio).entry_bytes_remaining);
        (*cpio).entry_bytes_remaining = 0 as libc::c_int as int64_t
    }
    /* XXX TODO: If the full mode is 0160200, then this is a Solaris
     * ACL description for the following entry.  Read this body
     * and parse it as a Solaris-style ACL, then read the next
     * header.  XXX */
    /* Compare name to "TRAILER!!!" to test for end-of-archive. */
    if namelength == 11 as libc::c_int as libc::c_ulong
        && strncmp(
            h as *const libc::c_char,
            b"TRAILER!!!\x00" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        /* TODO: Store file location of start of block. */
        archive_clear_error(&mut (*a).archive);
        return 1 as libc::c_int;
    }
    /* Detect and record hardlinks to previously-extracted entries. */
    if record_hardlink(a, cpio, entry) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    return r;
}
unsafe extern "C" fn archive_read_format_cpio_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut bytes_read: ssize_t = 0;
    let mut cpio: *mut cpio = 0 as *mut cpio;
    cpio = (*(*a).format).data as *mut cpio;
    if (*cpio).entry_bytes_unconsumed != 0 {
        __archive_read_consume(a, (*cpio).entry_bytes_unconsumed);
        (*cpio).entry_bytes_unconsumed = 0 as libc::c_int as int64_t
    }
    if (*cpio).entry_bytes_remaining > 0 as libc::c_int as libc::c_long {
        *buff = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read);
        if bytes_read <= 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int);
        }
        if bytes_read > (*cpio).entry_bytes_remaining {
            bytes_read = (*cpio).entry_bytes_remaining
        }
        *size = bytes_read as size_t;
        (*cpio).entry_bytes_unconsumed = bytes_read;
        *offset = (*cpio).entry_offset;
        (*cpio).entry_offset += bytes_read;
        (*cpio).entry_bytes_remaining -= bytes_read;
        return 0 as libc::c_int;
    } else {
        if (*cpio).entry_padding != __archive_read_consume(a, (*cpio).entry_padding) {
            return -(30 as libc::c_int);
        }
        (*cpio).entry_padding = 0 as libc::c_int as int64_t;
        *buff = NULL as *const libc::c_void;
        *size = 0 as libc::c_int as size_t;
        *offset = (*cpio).entry_offset;
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn archive_read_format_cpio_skip(mut a: *mut archive_read) -> libc::c_int {
    let mut cpio: *mut cpio = (*(*a).format).data as *mut cpio;
    let mut to_skip: int64_t =
        (*cpio).entry_bytes_remaining + (*cpio).entry_padding + (*cpio).entry_bytes_unconsumed;
    if to_skip != __archive_read_consume(a, to_skip) {
        return -(30 as libc::c_int);
    }
    (*cpio).entry_bytes_remaining = 0 as libc::c_int as int64_t;
    (*cpio).entry_padding = 0 as libc::c_int as int64_t;
    (*cpio).entry_bytes_unconsumed = 0 as libc::c_int as int64_t;
    return 0 as libc::c_int;
}
/*
 * Skip forward to the next cpio newc header by searching for the
 * 07070[12] string.  This should be generalized and merged with
 * find_odc_header below.
 */
unsafe extern "C" fn is_hex(mut p: *const libc::c_char, mut len: size_t) -> libc::c_int {
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32
            || *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'f' as i32
            || *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'F' as i32
        {
            p = p.offset(1)
        } else {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_newc_header(mut a: *mut archive_read) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut skip: size_t = 0;
    let mut skipped: size_t = 0 as libc::c_int as size_t;
    let mut bytes: ssize_t = 0;
    loop {
        h = __archive_read_ahead(a, newc_header_size as size_t, &mut bytes);
        if h == NULL as *const libc::c_void {
            return -(30 as libc::c_int);
        }
        p = h as *const libc::c_char;
        q = p.offset(bytes as isize);
        /* Try the typical case first, then go into the slow search.*/
        if memcmp(
            b"07070\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            p as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && (*p.offset(5 as libc::c_int as isize) as libc::c_int == '1' as i32
                || *p.offset(5 as libc::c_int as isize) as libc::c_int == '2' as i32)
            && is_hex(p, newc_header_size as size_t) != 0
        {
            return 0 as libc::c_int;
        }
        /*
         * Scan ahead until we find something that looks
         * like a newc header.
         */
        while p.offset(newc_header_size as isize) <= q {
            match *p.offset(5 as libc::c_int as isize) as libc::c_int {
                49 | 50 => {
                    if memcmp(
                        b"07070\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        p as *const libc::c_void,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                        && is_hex(p, newc_header_size as size_t) != 0
                    {
                        skip = p.offset_from(h as *const libc::c_char) as libc::c_long
                            as size_t;
                        __archive_read_consume(a, skip as int64_t);
                        skipped = (skipped as libc::c_ulong).wrapping_add(skip) as size_t as size_t;
                        if skipped > 0 as libc::c_int as libc::c_ulong {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                0 as libc::c_int,
                                b"Skipped %d bytes before finding valid header\x00" as *const u8
                                    as *const libc::c_char,
                                skipped as libc::c_int,
                            );
                            return -(20 as libc::c_int);
                        }
                        return 0 as libc::c_int;
                    }
                    p = p.offset(2 as libc::c_int as isize)
                }
                48 => p = p.offset(1),
                _ => p = p.offset(6 as libc::c_int as isize),
            }
        }
        skip = p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
        __archive_read_consume(a, skip as int64_t);
        skipped = (skipped as libc::c_ulong).wrapping_add(skip) as size_t as size_t
    }
}
unsafe extern "C" fn header_newc(
    mut a: *mut archive_read,
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
    mut namelength: *mut size_t,
    mut name_pad: *mut size_t,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut header: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    r = find_newc_header(a);
    if r < ARCHIVE_WARN {
        return r;
    }
    /* Read fixed-size portion of header. */
    h = __archive_read_ahead(a, newc_header_size as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    /* Parse out hex fields. */
    header = h as *const libc::c_char;
    if memcmp(
        header.offset(newc_magic_offset as isize) as *const libc::c_void,
        b"070701\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_SVR4_NOCRC;
        (*a).archive.archive_format_name =
            b"ASCII cpio (SVR4 with no CRC)\x00" as *const u8 as *const libc::c_char
    } else if memcmp(
        header.offset(newc_magic_offset as isize) as *const libc::c_void,
        b"070702\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_SVR4_CRC;
        (*a).archive.archive_format_name =
            b"ASCII cpio (SVR4 with CRC)\x00" as *const u8 as *const libc::c_char
    }
    archive_entry_set_devmajor(
        entry,
        atol16(
            header.offset(newc_devmajor_offset as isize),
            newc_devmajor_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_devminor(
        entry,
        atol16(
            header.offset(newc_devminor_offset as isize),
            newc_devminor_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_ino(
        entry,
        atol16(
            header.offset(newc_ino_offset as isize),
            newc_ino_size as libc::c_uint,
        ),
    );
    archive_entry_set_mode(
        entry,
        atol16(
            header.offset(newc_mode_offset as isize),
            newc_mode_size as libc::c_uint,
        ) as mode_t,
    );
    archive_entry_set_uid(
        entry,
        atol16(
            header.offset(newc_uid_offset as isize),
            newc_uid_size as libc::c_uint,
        ),
    );
    archive_entry_set_gid(
        entry,
        atol16(
            header.offset(newc_gid_offset as isize),
            newc_gid_size as libc::c_uint,
        ),
    );
    archive_entry_set_nlink(
        entry,
        atol16(
            header.offset(newc_nlink_offset as isize),
            newc_nlink_size as libc::c_uint,
        ) as libc::c_uint,
    );
    archive_entry_set_rdevmajor(
        entry,
        atol16(
            header.offset(newc_rdevmajor_offset as isize),
            newc_rdevmajor_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_rdevminor(
        entry,
        atol16(
            header.offset(newc_rdevminor_offset as isize),
            newc_rdevminor_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_mtime(
        entry,
        atol16(
            header.offset(newc_mtime_offset as isize),
            newc_mtime_size as libc::c_uint,
        ),
        0 as libc::c_int as libc::c_long,
    );
    *namelength = atol16(
        header.offset(newc_namesize_offset as isize),
        newc_namesize_size as libc::c_uint,
    ) as size_t;
    /* Pad name to 2 more than a multiple of 4. */
    *name_pad = (2 as libc::c_int as libc::c_ulong).wrapping_sub(*namelength)
        & 3 as libc::c_int as libc::c_ulong;
    /* Make sure that the padded name length fits into size_t. */
    if *name_pad > SIZE_MAX.wrapping_sub(*namelength) {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"cpio archive has invalid namelength\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * Note: entry_bytes_remaining is at least 64 bits and
     * therefore guaranteed to be big enough for a 33-bit file
     * size.
     */
    (*cpio).entry_bytes_remaining = atol16(
        header.offset(newc_filesize_offset as isize),
        newc_filesize_size as libc::c_uint,
    );
    archive_entry_set_size(entry, (*cpio).entry_bytes_remaining);
    /* Pad file contents to a multiple of 4. */
    (*cpio).entry_padding = 3 as libc::c_int as libc::c_long & -(*cpio).entry_bytes_remaining;
    __archive_read_consume(a, newc_header_size as int64_t);
    return r;
}
/*
 * Skip forward to the next cpio odc header by searching for the
 * 070707 string.  This is a hand-optimized search that could
 * probably be easily generalized to handle all character-based
 * cpio variants.
 */
unsafe extern "C" fn is_octal(mut p: *const libc::c_char, mut len: size_t) -> libc::c_int {
    loop {
        let fresh1 = len;
        len = len.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '7' as i32 {
            return 0 as libc::c_int;
        }
        p = p.offset(1)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn is_afio_large(mut h: *const libc::c_char, mut len: size_t) -> libc::c_int {
    if len < afiol_header_size as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if *h.offset(afiol_ino_m_offset as isize) as libc::c_int != 'm' as i32
        || *h.offset(afiol_mtime_n_offset as isize) as libc::c_int != 'n' as i32
        || *h.offset(afiol_xsize_s_offset as isize) as libc::c_int != 's' as i32
        || *h.offset(afiol_filesize_c_offset as isize) as libc::c_int != ':' as i32
    {
        return 0 as libc::c_int;
    }
    if is_hex(
        h.offset(afiol_dev_offset as isize),
        (afiol_ino_m_offset - afiol_dev_offset) as size_t,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if is_hex(
        h.offset(afiol_mode_offset as isize),
        (afiol_mtime_n_offset - afiol_mode_offset) as size_t,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if is_hex(
        h.offset(afiol_namesize_offset as isize),
        (afiol_xsize_s_offset - afiol_namesize_offset) as size_t,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if is_hex(
        h.offset(afiol_filesize_offset as isize),
        afiol_filesize_size as size_t,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_odc_header(mut a: *mut archive_read) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut skip: size_t = 0;
    let mut skipped: size_t = 0 as libc::c_int as size_t;
    let mut bytes: ssize_t = 0;
    loop {
        h = __archive_read_ahead(a, odc_header_size as size_t, &mut bytes);
        if h == NULL as *const libc::c_void {
            return -(30 as libc::c_int);
        }
        p = h as *const libc::c_char;
        q = p.offset(bytes as isize);
        /* Try the typical case first, then go into the slow search.*/
        if memcmp(
            b"070707\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            p as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && is_octal(p, odc_header_size as size_t) != 0
        {
            return 0 as libc::c_int;
        }
        if memcmp(
            b"070727\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            p as *const libc::c_void,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && is_afio_large(p, bytes as size_t) != 0
        {
            (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_AFIO_LARGE;
            return 0 as libc::c_int;
        }
        /*
         * Scan ahead until we find something that looks
         * like an odc header.
         */
        while p.offset(odc_header_size as isize) <= q {
            match *p.offset(5 as libc::c_int as isize) as libc::c_int {
                55 => {
                    if memcmp(
                        b"070707\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        p as *const libc::c_void,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                        && is_octal(p, odc_header_size as size_t) != 0
                        || memcmp(
                            b"070727\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            p as *const libc::c_void,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                            && is_afio_large(p, q.offset_from(p) as libc::c_long as size_t)
                                != 0
                    {
                        skip = p.offset_from(h as *const libc::c_char) as libc::c_long
                            as size_t;
                        __archive_read_consume(a, skip as int64_t);
                        skipped = (skipped as libc::c_ulong).wrapping_add(skip) as size_t as size_t;
                        if *p.offset(4 as libc::c_int as isize) as libc::c_int == '2' as i32 {
                            (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_AFIO_LARGE
                        }
                        if skipped > 0 as libc::c_int as libc::c_ulong {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                0 as libc::c_int,
                                b"Skipped %d bytes before finding valid header\x00" as *const u8
                                    as *const libc::c_char,
                                skipped as libc::c_int,
                            );
                            return -(20 as libc::c_int);
                        }
                        return 0 as libc::c_int;
                    }
                    p = p.offset(2 as libc::c_int as isize)
                }
                48 => p = p.offset(1),
                _ => p = p.offset(6 as libc::c_int as isize),
            }
        }
        skip = p.offset_from(h as *const libc::c_char) as libc::c_long as size_t;
        __archive_read_consume(a, skip as int64_t);
        skipped = (skipped as libc::c_ulong).wrapping_add(skip) as size_t as size_t
    }
}
unsafe extern "C" fn header_odc(
    mut a: *mut archive_read,
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
    mut namelength: *mut size_t,
    mut name_pad: *mut size_t,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    let mut header: *const libc::c_char = 0 as *const libc::c_char;
    (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_POSIX;
    (*a).archive.archive_format_name =
        b"POSIX octet-oriented cpio\x00" as *const u8 as *const libc::c_char;
    /* Find the start of the next header. */
    r = find_odc_header(a);
    if r < ARCHIVE_WARN {
        return r;
    }
    if (*a).archive.archive_format == ARCHIVE_FORMAT_CPIO_AFIO_LARGE {
        let mut r2: libc::c_int = header_afiol(a, cpio, entry, namelength, name_pad);
        if r2 == ARCHIVE_OK {
            return r;
        } else {
            return r2;
        }
    }
    /* Read fixed-size portion of header. */
    h = __archive_read_ahead(a, odc_header_size as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    /* Parse out octal fields. */
    header = h as *const libc::c_char; /* No padding of filename. */
    archive_entry_set_dev(
        entry,
        atol8(
            header.offset(odc_dev_offset as isize),
            odc_dev_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_ino(
        entry,
        atol8(
            header.offset(odc_ino_offset as isize),
            odc_ino_size as libc::c_uint,
        ),
    );
    archive_entry_set_mode(
        entry,
        atol8(
            header.offset(odc_mode_offset as isize),
            odc_mode_size as libc::c_uint,
        ) as mode_t,
    );
    archive_entry_set_uid(
        entry,
        atol8(
            header.offset(odc_uid_offset as isize),
            odc_uid_size as libc::c_uint,
        ),
    );
    archive_entry_set_gid(
        entry,
        atol8(
            header.offset(odc_gid_offset as isize),
            odc_gid_size as libc::c_uint,
        ),
    );
    archive_entry_set_nlink(
        entry,
        atol8(
            header.offset(odc_nlink_offset as isize),
            odc_nlink_size as libc::c_uint,
        ) as libc::c_uint,
    );
    archive_entry_set_rdev(
        entry,
        atol8(
            header.offset(odc_rdev_offset as isize),
            odc_rdev_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_mtime(
        entry,
        atol8(
            header.offset(odc_mtime_offset as isize),
            odc_mtime_size as libc::c_uint,
        ),
        0 as libc::c_int as libc::c_long,
    );
    *namelength = atol8(
        header.offset(odc_namesize_offset as isize),
        odc_namesize_size as libc::c_uint,
    ) as size_t;
    *name_pad = 0 as libc::c_int as size_t;
    /*
     * Note: entry_bytes_remaining is at least 64 bits and
     * therefore guaranteed to be big enough for a 33-bit file
     * size.
     */
    (*cpio).entry_bytes_remaining = atol8(
        header.offset(odc_filesize_offset as isize),
        odc_filesize_size as libc::c_uint,
    );
    archive_entry_set_size(entry, (*cpio).entry_bytes_remaining);
    (*cpio).entry_padding = 0 as libc::c_int as int64_t;
    __archive_read_consume(a, odc_header_size as int64_t);
    return r;
}
/*
 * NOTE: if a filename suffix is ".z", it is the file gziped by afio.
 * it would be nice that we can show uncompressed file size and we can
 * uncompressed file contents automatically, unfortunately we have nothing
 * to get a uncompressed file size while reading each header. It means
 * we also cannot uncompress file contents under our framework.
 */
unsafe extern "C" fn header_afiol(
    mut a: *mut archive_read,
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
    mut namelength: *mut size_t,
    mut name_pad: *mut size_t,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut header: *const libc::c_char = 0 as *const libc::c_char;
    (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_AFIO_LARGE;
    (*a).archive.archive_format_name = b"afio large ASCII\x00" as *const u8 as *const libc::c_char;
    /* Read fixed-size portion of header. */
    h = __archive_read_ahead(a, afiol_header_size as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        return -(30 as libc::c_int);
    }
    /* Parse out octal fields. */
    header = h as *const libc::c_char; /* No padding of filename. */
    archive_entry_set_dev(
        entry,
        atol16(
            header.offset(afiol_dev_offset as isize),
            afiol_dev_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_ino(
        entry,
        atol16(
            header.offset(afiol_ino_offset as isize),
            afiol_ino_size as libc::c_uint,
        ),
    );
    archive_entry_set_mode(
        entry,
        atol8(
            header.offset(afiol_mode_offset as isize),
            afiol_mode_size as libc::c_uint,
        ) as mode_t,
    );
    archive_entry_set_uid(
        entry,
        atol16(
            header.offset(afiol_uid_offset as isize),
            afiol_uid_size as libc::c_uint,
        ),
    );
    archive_entry_set_gid(
        entry,
        atol16(
            header.offset(afiol_gid_offset as isize),
            afiol_gid_size as libc::c_uint,
        ),
    );
    archive_entry_set_nlink(
        entry,
        atol16(
            header.offset(afiol_nlink_offset as isize),
            afiol_nlink_size as libc::c_uint,
        ) as libc::c_uint,
    );
    archive_entry_set_rdev(
        entry,
        atol16(
            header.offset(afiol_rdev_offset as isize),
            afiol_rdev_size as libc::c_uint,
        ) as dev_t,
    );
    archive_entry_set_mtime(
        entry,
        atol16(
            header.offset(afiol_mtime_offset as isize),
            afiol_mtime_size as libc::c_uint,
        ),
        0 as libc::c_int as libc::c_long,
    );
    *namelength = atol16(
        header.offset(afiol_namesize_offset as isize),
        afiol_namesize_size as libc::c_uint,
    ) as size_t;
    *name_pad = 0 as libc::c_int as size_t;
    (*cpio).entry_bytes_remaining = atol16(
        header.offset(afiol_filesize_offset as isize),
        afiol_filesize_size as libc::c_uint,
    );
    archive_entry_set_size(entry, (*cpio).entry_bytes_remaining);
    (*cpio).entry_padding = 0 as libc::c_int as int64_t;
    __archive_read_consume(a, afiol_header_size as int64_t);
    return 0 as libc::c_int;
}
unsafe extern "C" fn header_bin_le(
    mut a: *mut archive_read,
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
    mut namelength: *mut size_t,
    mut name_pad: *mut size_t,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut header: *const libc::c_uchar = 0 as *const libc::c_uchar;
    (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_BIN_LE;
    (*a).archive.archive_format_name =
        b"cpio (little-endian binary)\x00" as *const u8 as *const libc::c_char;
    /* Read fixed-size portion of header. */
    h = __archive_read_ahead(a, bin_header_size as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            0 as libc::c_int,
            b"End of file trying to read next cpio header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Parse out binary fields. */
    header = h as *const libc::c_uchar; /* Pad to even. */
    archive_entry_set_dev(
        entry,
        (*header.offset(bin_dev_offset as isize) as libc::c_int
            + *header.offset((bin_dev_offset + 1 as libc::c_int) as isize) as libc::c_int
                * 256 as libc::c_int) as dev_t,
    ); /* Pad to even. */
    archive_entry_set_ino(
        entry,
        (*header.offset(bin_ino_offset as isize) as libc::c_int
            + *header.offset((bin_ino_offset + 1 as libc::c_int) as isize) as libc::c_int
                * 256 as libc::c_int) as la_int64_t,
    );
    archive_entry_set_mode(
        entry,
        (*header.offset(bin_mode_offset as isize) as libc::c_int
            + *header.offset((bin_mode_offset + 1 as libc::c_int) as isize) as libc::c_int
                * 256 as libc::c_int) as mode_t,
    );
    archive_entry_set_uid(
        entry,
        (*header.offset(bin_uid_offset as isize) as libc::c_int
            + *header.offset((bin_uid_offset + 1 as libc::c_int) as isize) as libc::c_int
                * 256 as libc::c_int) as la_int64_t,
    );
    archive_entry_set_gid(
        entry,
        (*header.offset(bin_gid_offset as isize) as libc::c_int
            + *header.offset((bin_gid_offset + 1 as libc::c_int) as isize) as libc::c_int
                * 256 as libc::c_int) as la_int64_t,
    );
    archive_entry_set_nlink(
        entry,
        (*header.offset(bin_nlink_offset as isize) as libc::c_int
            + *header.offset((bin_nlink_offset + 1 as libc::c_int) as isize) as libc::c_int
                * 256 as libc::c_int) as libc::c_uint,
    );
    archive_entry_set_rdev(
        entry,
        (*header.offset(bin_rdev_offset as isize) as libc::c_int
            + *header.offset((bin_rdev_offset + 1 as libc::c_int) as isize) as libc::c_int
                * 256 as libc::c_int) as dev_t,
    );
    archive_entry_set_mtime(
        entry,
        le4(header.offset(bin_mtime_offset as isize)),
        0 as libc::c_int as libc::c_long,
    );
    *namelength = (*header.offset(bin_namesize_offset as isize) as libc::c_int
        + *header.offset((bin_namesize_offset + 1 as libc::c_int) as isize) as libc::c_int
            * 256 as libc::c_int) as size_t;
    *name_pad = *namelength & 1 as libc::c_int as libc::c_ulong;
    (*cpio).entry_bytes_remaining = le4(header.offset(bin_filesize_offset as isize));
    archive_entry_set_size(entry, (*cpio).entry_bytes_remaining);
    (*cpio).entry_padding = (*cpio).entry_bytes_remaining & 1 as libc::c_int as libc::c_long;
    __archive_read_consume(a, bin_header_size as int64_t);
    return 0 as libc::c_int;
}
unsafe extern "C" fn header_bin_be(
    mut a: *mut archive_read,
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
    mut namelength: *mut size_t,
    mut name_pad: *mut size_t,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    let mut header: *const libc::c_uchar = 0 as *const libc::c_uchar;
    (*a).archive.archive_format = ARCHIVE_FORMAT_CPIO_BIN_BE;
    (*a).archive.archive_format_name =
        b"cpio (big-endian binary)\x00" as *const u8 as *const libc::c_char;
    /* Read fixed-size portion of header. */
    h = __archive_read_ahead(a, bin_header_size as size_t, NULL as *mut ssize_t);
    if h == NULL as *const libc::c_void {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            0 as libc::c_int,
            b"End of file trying to read next cpio header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Parse out binary fields. */
    header = h as *const libc::c_uchar; /* Pad to even. */
    archive_entry_set_dev(
        entry,
        (*header.offset(bin_dev_offset as isize) as libc::c_int * 256 as libc::c_int
            + *header.offset((bin_dev_offset + 1 as libc::c_int) as isize) as libc::c_int)
            as dev_t,
    ); /* Pad to even. */
    archive_entry_set_ino(
        entry,
        (*header.offset(bin_ino_offset as isize) as libc::c_int * 256 as libc::c_int
            + *header.offset((bin_ino_offset + 1 as libc::c_int) as isize) as libc::c_int)
            as la_int64_t,
    );
    archive_entry_set_mode(
        entry,
        (*header.offset(bin_mode_offset as isize) as libc::c_int * 256 as libc::c_int
            + *header.offset((bin_mode_offset + 1 as libc::c_int) as isize) as libc::c_int)
            as mode_t,
    );
    archive_entry_set_uid(
        entry,
        (*header.offset(bin_uid_offset as isize) as libc::c_int * 256 as libc::c_int
            + *header.offset((bin_uid_offset + 1 as libc::c_int) as isize) as libc::c_int)
            as la_int64_t,
    );
    archive_entry_set_gid(
        entry,
        (*header.offset(bin_gid_offset as isize) as libc::c_int * 256 as libc::c_int
            + *header.offset((bin_gid_offset + 1 as libc::c_int) as isize) as libc::c_int)
            as la_int64_t,
    );
    archive_entry_set_nlink(
        entry,
        (*header.offset(bin_nlink_offset as isize) as libc::c_int * 256 as libc::c_int
            + *header.offset((bin_nlink_offset + 1 as libc::c_int) as isize) as libc::c_int)
            as libc::c_uint,
    );
    archive_entry_set_rdev(
        entry,
        (*header.offset(bin_rdev_offset as isize) as libc::c_int * 256 as libc::c_int
            + *header.offset((bin_rdev_offset + 1 as libc::c_int) as isize) as libc::c_int)
            as dev_t,
    );
    archive_entry_set_mtime(
        entry,
        be4(header.offset(bin_mtime_offset as isize)),
        0 as libc::c_int as libc::c_long,
    );
    *namelength = (*header.offset(bin_namesize_offset as isize) as libc::c_int * 256 as libc::c_int
        + *header.offset((bin_namesize_offset + 1 as libc::c_int) as isize) as libc::c_int)
        as size_t;
    *name_pad = *namelength & 1 as libc::c_int as libc::c_ulong;
    (*cpio).entry_bytes_remaining = be4(header.offset(bin_filesize_offset as isize));
    archive_entry_set_size(entry, (*cpio).entry_bytes_remaining);
    (*cpio).entry_padding = (*cpio).entry_bytes_remaining & 1 as libc::c_int as libc::c_long;
    __archive_read_consume(a, bin_header_size as int64_t);
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_cpio_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut cpio: *mut cpio = 0 as *mut cpio;
    cpio = (*(*a).format).data as *mut cpio;
    /* Free inode->name map */
    while !(*cpio).links_head.is_null() {
        let mut lp: *mut links_entry = (*(*cpio).links_head).next;
        free((*(*cpio).links_head).name as *mut libc::c_void);
        free((*cpio).links_head as *mut libc::c_void);
        (*cpio).links_head = lp
    }
    free(cpio as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn le4(mut p: *const libc::c_uchar) -> int64_t {
    return ((*p.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
        as libc::c_long
        + ((*p.offset(1 as libc::c_int as isize) as int64_t) << 24 as libc::c_int)
        + ((*p.offset(2 as libc::c_int as isize) as libc::c_int) << 0 as libc::c_int)
            as libc::c_long
        + ((*p.offset(3 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
            as libc::c_long;
}
unsafe extern "C" fn be4(mut p: *const libc::c_uchar) -> int64_t {
    return ((*p.offset(0 as libc::c_int as isize) as int64_t) << 24 as libc::c_int)
        + ((*p.offset(1 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int)
            as libc::c_long
        + ((*p.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int)
            as libc::c_long
        + *p.offset(3 as libc::c_int as isize) as libc::c_long;
}
/*
 * Note that this implementation does not (and should not!) obey
 * locale settings; you cannot simply substitute strtol here, since
 * it does obey locale.
 */
unsafe extern "C" fn atol8(mut p: *const libc::c_char, mut char_cnt: libc::c_uint) -> int64_t {
    let mut l: int64_t = 0;
    let mut digit: libc::c_int = 0;
    l = 0 as libc::c_int as int64_t;
    loop {
        let fresh2 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        if !(fresh2 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '7' as i32 {
            digit = *p as libc::c_int - '0' as i32
        } else {
            return l;
        }
        p = p.offset(1);
        l <<= 3 as libc::c_int;
        l |= digit as libc::c_long
    }
    return l;
}
unsafe extern "C" fn atol16(mut p: *const libc::c_char, mut char_cnt: libc::c_uint) -> int64_t {
    let mut l: int64_t = 0;
    let mut digit: libc::c_int = 0;
    l = 0 as libc::c_int as int64_t;
    loop {
        let fresh3 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        if !(fresh3 > 0 as libc::c_int as libc::c_uint) {
            break;
        }
        if *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'f' as i32 {
            digit = *p as libc::c_int - 'a' as i32 + 10 as libc::c_int
        } else if *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'F' as i32 {
            digit = *p as libc::c_int - 'A' as i32 + 10 as libc::c_int
        } else if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            digit = *p as libc::c_int - '0' as i32
        } else {
            return l;
        }
        p = p.offset(1);
        l <<= 4 as libc::c_int;
        l |= digit as libc::c_long
    }
    return l;
}
unsafe extern "C" fn record_hardlink(
    mut a: *mut archive_read,
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut le: *mut links_entry = 0 as *mut links_entry;
    let mut dev: dev_t = 0;
    let mut ino: int64_t = 0;
    if archive_entry_nlink(entry) <= 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    dev = archive_entry_dev(entry);
    ino = archive_entry_ino64(entry);
    /*
     * First look in the list of multiply-linked files.  If we've
     * already dumped it, convert this entry to a hard link entry.
     */
    le = (*cpio).links_head;
    while !le.is_null() {
        if (*le).dev == dev && (*le).ino == ino {
            archive_entry_copy_hardlink(entry, (*le).name);
            (*le).links = (*le).links.wrapping_sub(1);
            if (*le).links <= 0 as libc::c_int as libc::c_uint {
                if !(*le).previous.is_null() {
                    (*(*le).previous).next = (*le).next
                }
                if !(*le).next.is_null() {
                    (*(*le).next).previous = (*le).previous
                }
                if (*cpio).links_head == le {
                    (*cpio).links_head = (*le).next
                }
                free((*le).name as *mut libc::c_void);
                free(le as *mut libc::c_void);
            }
            return 0 as libc::c_int;
        }
        le = (*le).next
    }
    le = malloc(::std::mem::size_of::<links_entry>() as libc::c_ulong) as *mut links_entry;
    if le.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory adding file to list\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if !(*cpio).links_head.is_null() {
        (*(*cpio).links_head).previous = le
    }
    (*le).next = (*cpio).links_head;
    (*le).previous = NULL as *mut links_entry;
    (*cpio).links_head = le;
    (*le).dev = dev;
    (*le).ino = ino;
    (*le).links = archive_entry_nlink(entry).wrapping_sub(1 as libc::c_int as libc::c_uint);
    (*le).name = strdup(archive_entry_pathname(entry));
    if (*le).name.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory adding file to list\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
