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
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_entry_set_filetype(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
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
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type time_t = __time_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub struct ar {
    pub entry_bytes_remaining: int64_t,
    pub entry_bytes_unconsumed: size_t,
    pub entry_offset: int64_t,
    pub entry_padding: int64_t,
    pub strtab: *mut libc::c_char,
    pub strtab_size: size_t,
    pub read_global_header: libc::c_char,
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
pub const UINT64_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FORMAT_AR: libc::c_int = 0x70000 as libc::c_int;
pub const ARCHIVE_FORMAT_AR_BSD: libc::c_int = ARCHIVE_FORMAT_AR | 2 as libc::c_int;
pub const ARCHIVE_FORMAT_AR_GNU: libc::c_int = ARCHIVE_FORMAT_AR | 1 as libc::c_int;
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
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
/*
 * Define structure of the "ar" header.
 */
pub const AR_name_offset: libc::c_int = 0 as libc::c_int;
pub const AR_name_size: libc::c_int = 16 as libc::c_int;
pub const AR_date_offset: libc::c_int = 16 as libc::c_int;
pub const AR_date_size: libc::c_int = 12 as libc::c_int;
pub const AR_uid_offset: libc::c_int = 28 as libc::c_int;
pub const AR_uid_size: libc::c_int = 6 as libc::c_int;
pub const AR_gid_offset: libc::c_int = 34 as libc::c_int;
pub const AR_gid_size: libc::c_int = 6 as libc::c_int;
pub const AR_mode_offset: libc::c_int = 40 as libc::c_int;
pub const AR_mode_size: libc::c_int = 8 as libc::c_int;
pub const AR_size_offset: libc::c_int = 48 as libc::c_int;
pub const AR_size_size: libc::c_int = 10 as libc::c_int;
pub const AR_fmag_offset: libc::c_int = 58 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_ar(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut ar: *mut ar = 0 as *mut ar;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_ar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    ar = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<ar>() as libc::c_ulong,
    ) as *mut ar;
    if ar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate ar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*ar).strtab = NULL as *mut libc::c_char;
    r = __archive_read_register_format(
        a,
        ar as *mut libc::c_void,
        b"ar\x00" as *const u8 as *const libc::c_char,
        Some(
            archive_read_format_ar_bid
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
            archive_read_format_ar_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            archive_read_format_ar_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(
            archive_read_format_ar_skip
                as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int,
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(
            archive_read_format_ar_cleanup
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
        free(ar as *mut libc::c_void);
        return r;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_ar_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut ar: *mut ar = 0 as *mut ar;
    ar = (*(*a).format).data as *mut ar;
    free((*ar).strtab as *mut libc::c_void);
    free(ar as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_ar_bid(
    mut a: *mut archive_read,
    mut best_bid: libc::c_int,
) -> libc::c_int {
    let mut h: *const libc::c_void = 0 as *const libc::c_void;
    /* UNUSED */
    /*
     * Verify the 8-byte file signature.
     * TODO: Do we need to check more than this?
     */
    h = __archive_read_ahead(a, 8 as libc::c_int as size_t, NULL as *mut ssize_t); /* Used to hold parsed numbers before validation. */
    if h == NULL as *const libc::c_void {
        return -(1 as libc::c_int);
    }
    if memcmp(
        h,
        b"!<arch>\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 64 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn _ar_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
    mut ar: *mut ar,
    mut h: *const libc::c_char,
    mut unconsumed: *mut size_t,
) -> libc::c_int {
    let mut filename: [libc::c_char; 17] = [0; 17];
    let mut number: uint64_t = 0;
    let mut bsd_name_length: size_t = 0;
    let mut entry_size: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut st: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    /* Verify the magic signature on the file header. */
    if strncmp(
        h.offset(AR_fmag_offset as isize),
        b"`\n\x00" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            EINVAL,
            b"Incorrect file header signature\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* Copy filename into work buffer. */
    strncpy(
        filename.as_mut_ptr(),
        h.offset(AR_name_offset as isize),
        AR_name_size as libc::c_ulong,
    );
    filename[AR_name_size as usize] = '\u{0}' as i32 as libc::c_char;
    /*
     * Guess the format variant based on the filename.
     */
    if (*a).archive.archive_format == ARCHIVE_FORMAT_AR {
        /* We don't already know the variant, so let's guess. */
        /*
         * Biggest clue is presence of '/': GNU starts special
         * filenames with '/', appends '/' as terminator to
         * non-special names, so anything with '/' should be
         * GNU except for BSD long filenames.
         */
        if strncmp(
            filename.as_mut_ptr(),
            b"#1/\x00" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*a).archive.archive_format = ARCHIVE_FORMAT_AR_BSD
        } else if !strchr(filename.as_mut_ptr(), '/' as i32).is_null() {
            (*a).archive.archive_format = ARCHIVE_FORMAT_AR_GNU
        } else if strncmp(
            filename.as_mut_ptr(),
            b"__.SYMDEF\x00" as *const u8 as *const libc::c_char,
            9 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*a).archive.archive_format = ARCHIVE_FORMAT_AR_BSD
        }
        /*
         * XXX Do GNU/SVR4 'ar' programs ever omit trailing '/'
         * if name exactly fills 16-byte field?  If so, we
         * can't assume entries without '/' are BSD. XXX
         */
    }
    /* Update format name from the code. */
    if (*a).archive.archive_format == ARCHIVE_FORMAT_AR_GNU {
        (*a).archive.archive_format_name = b"ar (GNU/SVR4)\x00" as *const u8 as *const libc::c_char
    } else if (*a).archive.archive_format == ARCHIVE_FORMAT_AR_BSD {
        (*a).archive.archive_format_name = b"ar (BSD)\x00" as *const u8 as *const libc::c_char
    } else {
        (*a).archive.archive_format_name = b"ar\x00" as *const u8 as *const libc::c_char
    }
    /*
     * Remove trailing spaces from the filename.  GNU and BSD
     * variants both pad filename area out with spaces.
     * This will only be wrong if GNU/SVR4 'ar' implementations
     * omit trailing '/' for 16-char filenames and we have
     * a 16-char filename that ends in ' '.
     */
    p = filename
        .as_mut_ptr()
        .offset(AR_name_size as isize)
        .offset(-(1 as libc::c_int as isize));
    while p >= filename.as_mut_ptr() && *p as libc::c_int == ' ' as i32 {
        *p = '\u{0}' as i32 as libc::c_char;
        p = p.offset(-1)
    }
    /*
     * Remove trailing slash unless first character is '/'.
     * (BSD entries never end in '/', so this will only trim
     * GNU-format entries.  GNU special entries start with '/'
     * and are not terminated in '/', so we don't trim anything
     * that starts with '/'.)
     */
    if filename[0 as libc::c_int as usize] as libc::c_int != '/' as i32
        && p > filename.as_mut_ptr()
        && *p as libc::c_int == '/' as i32
    {
        *p = '\u{0}' as i32 as libc::c_char
    }
    if p < filename.as_mut_ptr() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Found entry with empty filename\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * '//' is the GNU filename table.
     * Later entries can refer to names in this table.
     */
    if strcmp(
        filename.as_mut_ptr(),
        b"//\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        /* This must come before any call to _read_ahead. */
        ar_parse_common_header(ar, entry, h);
        archive_entry_copy_pathname(entry, filename.as_mut_ptr());
        archive_entry_set_filetype(entry, AE_IFREG as mode_t);
        /* Get the size of the filename table. */
        number = ar_atol10(
            h.offset(AR_size_offset as isize),
            AR_size_size as libc::c_uint,
        );
        if number > SIZE_MAX
            || number
                > (1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Filename table too large\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        entry_size = number;
        if entry_size == 0 as libc::c_int as libc::c_ulong {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"Invalid string table\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if !(*ar).strtab.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"More than one string tables exist\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Read the filename table into memory. */
        st = malloc(entry_size) as *mut libc::c_char;
        if st.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate filename table buffer\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*ar).strtab = st;
        (*ar).strtab_size = entry_size;
        if *unconsumed != 0 {
            __archive_read_consume(a, *unconsumed as int64_t);
            *unconsumed = 0 as libc::c_int as size_t
        }
        b = __archive_read_ahead(a, entry_size, NULL as *mut ssize_t);
        if b == NULL as *const libc::c_void {
            return -(30 as libc::c_int);
        }
        memcpy(st as *mut libc::c_void, b, entry_size);
        __archive_read_consume(a, entry_size as int64_t);
        /* All contents are consumed. */
        (*ar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
        archive_entry_set_size(entry, (*ar).entry_bytes_remaining);
        /* Parse the filename table. */
        return ar_parse_gnu_filename_table(a);
    }
    /*
     * GNU variant handles long filenames by storing /<number>
     * to indicate a name stored in the filename table.
     * XXX TODO: Verify that it's all digits... Don't be fooled
     * by "/9xyz" XXX
     */
    if filename[0 as libc::c_int as usize] as libc::c_int == '/' as i32
        && filename[1 as libc::c_int as usize] as libc::c_int >= '0' as i32
        && filename[1 as libc::c_int as usize] as libc::c_int <= '9' as i32
    {
        number = ar_atol10(
            h.offset(AR_name_offset as isize)
                .offset(1 as libc::c_int as isize),
            (AR_name_size - 1 as libc::c_int) as libc::c_uint,
        );
        /*
         * If we can't look up the real name, warn and return
         * the entry with the wrong name.
         */
        if (*ar).strtab.is_null() || number >= (*ar).strtab_size {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                EINVAL,
                b"Can\'t find long filename for GNU/SVR4 archive entry\x00" as *const u8
                    as *const libc::c_char,
            );
            archive_entry_copy_pathname(entry, filename.as_mut_ptr());
            /* Parse the time, owner, mode, size fields. */
            ar_parse_common_header(ar, entry, h);
            return -(30 as libc::c_int);
        }
        archive_entry_copy_pathname(entry, &mut *(*ar).strtab.offset(number as isize));
        /* Parse the time, owner, mode, size fields. */
        return ar_parse_common_header(ar, entry, h);
    }
    /*
     * BSD handles long filenames by storing "#1/" followed by the
     * length of filename as a decimal number, then prepends the
     * the filename to the file contents.
     */
    if strncmp(
        filename.as_mut_ptr(),
        b"#1/\x00" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        /* Parse the time, owner, mode, size fields. */
        /* This must occur before _read_ahead is called again. */
        ar_parse_common_header(ar, entry, h);
        /* Parse the size of the name, adjust the file size. */
        number = ar_atol10(
            h.offset(AR_name_offset as isize)
                .offset(3 as libc::c_int as isize),
            (AR_name_size - 3 as libc::c_int) as libc::c_uint,
        );
        /* Sanity check the filename length:
         *   = Must be <= SIZE_MAX - 1
         *   = Must be <= 1MB
         *   = Cannot be bigger than the entire entry
         */
        if number > SIZE_MAX.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            || number > (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
            || number as int64_t > (*ar).entry_bytes_remaining
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Bad input file size\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        bsd_name_length = number;
        (*ar).entry_bytes_remaining = ((*ar).entry_bytes_remaining as libc::c_ulong)
            .wrapping_sub(bsd_name_length) as int64_t
            as int64_t;
        /* Adjust file size reported to client. */
        archive_entry_set_size(entry, (*ar).entry_bytes_remaining);
        if *unconsumed != 0 {
            __archive_read_consume(a, *unconsumed as int64_t);
            *unconsumed = 0 as libc::c_int as size_t
        }
        /* Read the long name into memory. */
        b = __archive_read_ahead(a, bsd_name_length, NULL as *mut ssize_t);
        if b == NULL as *const libc::c_void {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Truncated input file\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        /* Store it in the entry. */
        p = malloc(bsd_name_length.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if p.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate fname buffer\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        strncpy(p, b as *const libc::c_char, bsd_name_length);
        *p.offset(bsd_name_length as isize) = '\u{0}' as i32 as libc::c_char;
        __archive_read_consume(a, bsd_name_length as int64_t);
        archive_entry_copy_pathname(entry, p);
        free(p as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    /*
     * "/" is the SVR4/GNU archive symbol table.
     * "/SYM64/" is the SVR4/GNU 64-bit variant archive symbol table.
     */
    if strcmp(
        filename.as_mut_ptr(),
        b"/\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            filename.as_mut_ptr(),
            b"/SYM64/\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        archive_entry_copy_pathname(entry, filename.as_mut_ptr());
        /* Parse the time, owner, mode, size fields. */
        r = ar_parse_common_header(ar, entry, h);
        /* Force the file type to a regular file. */
        archive_entry_set_filetype(entry, AE_IFREG as mode_t);
        return r;
    }
    /*
     * "__.SYMDEF" is a BSD archive symbol table.
     */
    if strcmp(
        filename.as_mut_ptr(),
        b"__.SYMDEF\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        archive_entry_copy_pathname(entry, filename.as_mut_ptr());
        /* Parse the time, owner, mode, size fields. */
        return ar_parse_common_header(ar, entry, h);
    }
    /*
     * Otherwise, this is a standard entry.  The filename
     * has already been trimmed as much as possible, based
     * on our current knowledge of the format.
     */
    archive_entry_copy_pathname(entry, filename.as_mut_ptr());
    return ar_parse_common_header(ar, entry, h);
}
unsafe extern "C" fn archive_read_format_ar_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut ar: *mut ar = (*(*a).format).data as *mut ar;
    let mut unconsumed: size_t = 0;
    let mut header_data: *const libc::c_void = 0 as *const libc::c_void;
    let mut ret: libc::c_int = 0;
    if (*ar).read_global_header == 0 {
        /*
         * We are now at the beginning of the archive,
         * so we need first consume the ar global header.
         */
        __archive_read_consume(a, 8 as libc::c_int as int64_t);
        (*ar).read_global_header = 1 as libc::c_int as libc::c_char;
        /* Set a default format code for now. */
        (*a).archive.archive_format = ARCHIVE_FORMAT_AR
    }
    /* Read the header for the next file entry. */
    header_data = __archive_read_ahead(a, 60 as libc::c_int as size_t, NULL as *mut ssize_t);
    if header_data == NULL as *const libc::c_void {
        /* Broken header. */
        return 1 as libc::c_int;
    }
    unconsumed = 60 as libc::c_int as size_t;
    ret = _ar_read_header(
        a,
        entry,
        ar,
        header_data as *const libc::c_char,
        &mut unconsumed,
    );
    if unconsumed != 0 {
        __archive_read_consume(a, unconsumed as int64_t);
    }
    return ret;
}
unsafe extern "C" fn ar_parse_common_header(
    mut ar: *mut ar,
    mut entry: *mut archive_entry,
    mut h: *const libc::c_char,
) -> libc::c_int {
    let mut n: uint64_t = 0;
    /* Copy remaining header */
    archive_entry_set_filetype(entry, AE_IFREG as mode_t);
    archive_entry_set_mtime(
        entry,
        ar_atol10(
            h.offset(AR_date_offset as isize),
            AR_date_size as libc::c_uint,
        ) as time_t,
        0 as libc::c_long,
    );
    archive_entry_set_uid(
        entry,
        ar_atol10(
            h.offset(AR_uid_offset as isize),
            AR_uid_size as libc::c_uint,
        ) as uid_t as la_int64_t,
    );
    archive_entry_set_gid(
        entry,
        ar_atol10(
            h.offset(AR_gid_offset as isize),
            AR_gid_size as libc::c_uint,
        ) as gid_t as la_int64_t,
    );
    archive_entry_set_mode(
        entry,
        ar_atol8(
            h.offset(AR_mode_offset as isize),
            AR_mode_size as libc::c_uint,
        ) as mode_t,
    );
    n = ar_atol10(
        h.offset(AR_size_offset as isize),
        AR_size_size as libc::c_uint,
    );
    (*ar).entry_offset = 0 as libc::c_int as int64_t;
    (*ar).entry_padding = n.wrapping_rem(2 as libc::c_int as libc::c_ulong) as int64_t;
    archive_entry_set_size(entry, n as la_int64_t);
    (*ar).entry_bytes_remaining = n as int64_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_read_format_ar_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut bytes_read: ssize_t = 0;
    let mut ar: *mut ar = 0 as *mut ar;
    ar = (*(*a).format).data as *mut ar;
    if (*ar).entry_bytes_unconsumed != 0 {
        __archive_read_consume(a, (*ar).entry_bytes_unconsumed as int64_t);
        (*ar).entry_bytes_unconsumed = 0 as libc::c_int as size_t
    }
    if (*ar).entry_bytes_remaining > 0 as libc::c_int as libc::c_long {
        *buff = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes_read);
        if bytes_read == 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Truncated ar archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if bytes_read < 0 as libc::c_int as libc::c_long {
            return -(30 as libc::c_int);
        }
        if bytes_read > (*ar).entry_bytes_remaining {
            bytes_read = (*ar).entry_bytes_remaining
        }
        *size = bytes_read as size_t;
        (*ar).entry_bytes_unconsumed = bytes_read as size_t;
        *offset = (*ar).entry_offset;
        (*ar).entry_offset += bytes_read;
        (*ar).entry_bytes_remaining -= bytes_read;
        return 0 as libc::c_int;
    } else {
        let mut skipped: int64_t = __archive_read_consume(a, (*ar).entry_padding);
        if skipped >= 0 as libc::c_int as libc::c_long {
            (*ar).entry_padding -= skipped
        }
        if (*ar).entry_padding != 0 {
            if skipped >= 0 as libc::c_int as libc::c_long {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Truncated ar archive- failed consuming padding\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            return -(30 as libc::c_int);
        }
        *buff = NULL as *const libc::c_void;
        *size = 0 as libc::c_int as size_t;
        *offset = (*ar).entry_offset;
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn archive_read_format_ar_skip(mut a: *mut archive_read) -> libc::c_int {
    let mut bytes_skipped: int64_t = 0;
    let mut ar: *mut ar = 0 as *mut ar;
    ar = (*(*a).format).data as *mut ar;
    bytes_skipped = __archive_read_consume(
        a,
        (((*ar).entry_bytes_remaining + (*ar).entry_padding) as libc::c_ulong)
            .wrapping_add((*ar).entry_bytes_unconsumed) as int64_t,
    );
    if bytes_skipped < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    (*ar).entry_bytes_remaining = 0 as libc::c_int as int64_t;
    (*ar).entry_bytes_unconsumed = 0 as libc::c_int as size_t;
    (*ar).entry_padding = 0 as libc::c_int as int64_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn ar_parse_gnu_filename_table(mut a: *mut archive_read) -> libc::c_int {
    let mut current_block: u64;
    let mut ar: *mut ar = 0 as *mut ar;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    ar = (*(*a).format).data as *mut ar;
    size = (*ar).strtab_size;
    p = (*ar).strtab;
    loop {
        if !(p
            < (*ar)
                .strtab
                .offset(size as isize)
                .offset(-(1 as libc::c_int as isize)))
        {
            current_block = 13109137661213826276;
            break;
        }
        if *p as libc::c_int == '/' as i32 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '\u{0}' as i32 as libc::c_char;
            if *p as libc::c_int != '\n' as i32 {
                current_block = 9293091585095973364;
                break;
            }
            *p = '\u{0}' as i32 as libc::c_char
        }
        p = p.offset(1)
    }
    match current_block {
        13109137661213826276 =>
        /*
         * GNU ar always pads the table to an even size.
         * The pad character is either '\n' or '`'.
         */
        {
            if !(p != (*ar).strtab.offset(size as isize)
                && *p as libc::c_int != '\n' as i32
                && *p as libc::c_int != '`' as i32)
            {
                /* Enforce zero termination. */
                *(*ar)
                    .strtab
                    .offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
                    '\u{0}' as i32 as libc::c_char; /* Truncate on overflow. */
                return 0 as libc::c_int;
            }
        }
        _ => {}
    } /* Truncate on overflow. */
    archive_set_error(
        &mut (*a).archive as *mut archive,
        EINVAL,
        b"Invalid string table\x00" as *const u8 as *const libc::c_char,
    );
    free((*ar).strtab as *mut libc::c_void);
    (*ar).strtab = NULL as *mut libc::c_char;
    return -(30 as libc::c_int);
}
unsafe extern "C" fn ar_atol8(mut p: *const libc::c_char, mut char_cnt: libc::c_uint) -> uint64_t {
    let mut l: uint64_t = 0;
    let mut limit: uint64_t = 0;
    let mut last_digit_limit: uint64_t = 0;
    let mut digit: libc::c_uint = 0;
    let mut base: libc::c_uint = 0;
    base = 8 as libc::c_int as libc::c_uint;
    limit = UINT64_MAX.wrapping_div(base as libc::c_ulong);
    last_digit_limit = UINT64_MAX.wrapping_rem(base as libc::c_ulong);
    while (*p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32) && {
        let fresh1 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        (fresh1) > 0 as libc::c_int as libc::c_uint
    } {
        p = p.offset(1)
    }
    l = 0 as libc::c_int as uint64_t;
    digit = (*p as libc::c_int - '0' as i32) as libc::c_uint;
    while *p as libc::c_int >= '0' as i32 && digit < base && {
        let fresh2 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        (fresh2) > 0 as libc::c_int as libc::c_uint
    } {
        if l > limit || l == limit && digit as libc::c_ulong > last_digit_limit {
            l = UINT64_MAX;
            break;
        } else {
            l = l
                .wrapping_mul(base as libc::c_ulong)
                .wrapping_add(digit as libc::c_ulong);
            p = p.offset(1);
            digit = (*p as libc::c_int - '0' as i32) as libc::c_uint
        }
    }
    return l;
}
unsafe extern "C" fn ar_atol10(mut p: *const libc::c_char, mut char_cnt: libc::c_uint) -> uint64_t {
    let mut l: uint64_t = 0;
    let mut limit: uint64_t = 0;
    let mut last_digit_limit: uint64_t = 0;
    let mut base: libc::c_uint = 0;
    let mut digit: libc::c_uint = 0;
    base = 10 as libc::c_int as libc::c_uint;
    limit = UINT64_MAX.wrapping_div(base as libc::c_ulong);
    last_digit_limit = UINT64_MAX.wrapping_rem(base as libc::c_ulong);
    while (*p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32) && {
        let fresh3 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        (fresh3) > 0 as libc::c_int as libc::c_uint
    } {
        p = p.offset(1)
    }
    l = 0 as libc::c_int as uint64_t;
    digit = (*p as libc::c_int - '0' as i32) as libc::c_uint;
    while *p as libc::c_int >= '0' as i32 && digit < base && {
        let fresh4 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        (fresh4) > 0 as libc::c_int as libc::c_uint
    } {
        if l > limit || l == limit && digit as libc::c_ulong > last_digit_limit {
            l = UINT64_MAX;
            break;
        } else {
            l = l
                .wrapping_mul(base as libc::c_ulong)
                .wrapping_add(digit as libc::c_ulong);
            p = p.offset(1);
            digit = (*p as libc::c_int - '0' as i32) as libc::c_uint
        }
    }
    return l;
}
