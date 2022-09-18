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
    fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int, __options: libc::c_int) -> __pid_t;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
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
    /*-
     * Copyright (c) 2007 Joerg Sonnenberger
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
     * $FreeBSD: head/lib/libarchive/filter_fork.h 201087 2009-12-28 02:18:26Z kientzle $
     */
    #[no_mangle]
    fn __archive_create_child(
        cmd: *const libc::c_char,
        child_stdin: *mut libc::c_int,
        child_stdout: *mut libc::c_int,
        out_child: *mut pid_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_check_child(in_0: libc::c_int, out: libc::c_int);
}
pub type __int64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type pid_t = __pid_t;
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
/*
 * The bidder object stores the command and the signature to watch for.
 * The 'inhibit' entry here is used to ensure that unchecked filters never
 * bid twice in the same pipeline.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_bidder {
    pub description: *mut libc::c_char,
    pub cmd: *mut libc::c_char,
    pub signature: *mut libc::c_void,
    pub signature_len: size_t,
    pub inhibit: libc::c_int,
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
 * The actual filter needs to track input and output data.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_filter {
    pub description: archive_string,
    pub child: pid_t,
    pub exit_status: libc::c_int,
    pub waitpid_return: libc::c_int,
    pub child_stdin: libc::c_int,
    pub child_stdout: libc::c_int,
    pub out_buf: *mut libc::c_char,
    pub out_buf_len: size_t,
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
pub const SIGPIPE: libc::c_int = 13 as libc::c_int;
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const EAGAIN: libc::c_int = 11 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const EPIPE: libc::c_int = 32 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const F_SETFL: libc::c_int = 4 as libc::c_int;
pub const __LONG_MAX__: libc::c_long = 9223372036854775807 as libc::c_long;
pub const SSIZE_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
pub const LONG_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FILTER_PROGRAM: libc::c_int = 4 as libc::c_int;
/* Deprecated; remove in libarchive 4.0 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_compression_program(
    mut a: *mut archive,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    return archive_read_support_filter_program(a, cmd);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_compression_program_signature(
    mut a: *mut archive,
    mut cmd: *const libc::c_char,
    mut signature: *const libc::c_void,
    mut signature_len: size_t,
) -> libc::c_int {
    return archive_read_support_filter_program_signature(a, cmd, signature, signature_len);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_program(
    mut a: *mut archive,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    return archive_read_support_filter_program_signature(
        a,
        cmd,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn set_bidder_signature(
    mut bidder: *mut archive_read_filter_bidder,
    mut state: *mut program_bidder,
    mut signature: *const libc::c_void,
    mut signature_len: size_t,
) -> libc::c_int {
    if signature != NULL as *const libc::c_void && signature_len > 0 as libc::c_int as libc::c_ulong
    {
        (*state).signature_len = signature_len;
        (*state).signature = malloc(signature_len);
        memcpy((*state).signature, signature, signature_len);
    }
    /*
     * Fill in the bidder object.
     */
    (*bidder).data = state as *mut libc::c_void;
    (*bidder).bid = Some(
        program_bidder_bid
            as unsafe extern "C" fn(
                _: *mut archive_read_filter_bidder,
                _: *mut archive_read_filter,
            ) -> libc::c_int,
    );
    (*bidder).init = Some(
        program_bidder_init as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int,
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
        program_bidder_free
            as unsafe extern "C" fn(_: *mut archive_read_filter_bidder) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_program_signature(
    mut _a: *mut archive,
    mut cmd: *const libc::c_char,
    mut signature: *const libc::c_void,
    mut signature_len: size_t,
) -> libc::c_int {
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut bidder: *mut archive_read_filter_bidder = 0 as *mut archive_read_filter_bidder;
    let mut state: *mut program_bidder = 0 as *mut program_bidder;
    /*
     * Get a bidder object from the read core.
     */
    if __archive_read_get_bidder(a, &mut bidder) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /*
     * Allocate our private state.
     */
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<program_bidder>() as libc::c_ulong,
    ) as *mut program_bidder;
    if !state.is_null() {
        (*state).cmd = strdup(cmd);
        if !(*state).cmd.is_null() {
            return set_bidder_signature(bidder, state, signature, signature_len);
        }
    }
    free_state(state);
    archive_set_error(
        _a,
        ENOMEM,
        b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn program_bidder_free(
    mut self_0: *mut archive_read_filter_bidder,
) -> libc::c_int {
    let mut state: *mut program_bidder = (*self_0).data as *mut program_bidder;
    free_state(state);
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_state(mut state: *mut program_bidder) {
    if !state.is_null() {
        free((*state).cmd as *mut libc::c_void);
        free((*state).signature);
        free(state as *mut libc::c_void);
    };
}
/*
 * If we do have a signature, bid only if that matches.
 *
 * If there's no signature, we bid INT_MAX the first time
 * we're called, then never bid again.
 */
unsafe extern "C" fn program_bidder_bid(
    mut self_0: *mut archive_read_filter_bidder,
    mut upstream: *mut archive_read_filter,
) -> libc::c_int {
    let mut state: *mut program_bidder = (*self_0).data as *mut program_bidder;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* If we have a signature, use that to match. */
    if (*state).signature_len > 0 as libc::c_int as libc::c_ulong {
        p = __archive_read_filter_ahead(upstream, (*state).signature_len, NULL as *mut ssize_t)
            as *const libc::c_char;
        if p.is_null() {
            return 0 as libc::c_int;
        }
        /* No match, so don't bid. */
        if memcmp(
            p as *const libc::c_void,
            (*state).signature,
            (*state).signature_len,
        ) != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        return (*state).signature_len as libc::c_int * 8 as libc::c_int;
    }
    /* Otherwise, bid once and then never bid again. */
    if (*state).inhibit != 0 {
        return 0 as libc::c_int;
    }
    (*state).inhibit = 1 as libc::c_int;
    return 2147483647 as libc::c_int;
}
/*
 * Shut down the child, return ARCHIVE_OK if it exited normally.
 *
 * Note that the return value is sticky; if we're called again,
 * we won't reap the child again, but we will return the same status
 * (including error message if the child came to a bad end).
 */
unsafe extern "C" fn child_stop(
    mut self_0: *mut archive_read_filter,
    mut state: *mut program_filter,
) -> libc::c_int {
    /* Close our side of the I/O with the child. */
    if (*state).child_stdin != -(1 as libc::c_int) {
        close((*state).child_stdin);
        (*state).child_stdin = -(1 as libc::c_int)
    }
    if (*state).child_stdout != -(1 as libc::c_int) {
        close((*state).child_stdout);
        (*state).child_stdout = -(1 as libc::c_int)
    }
    if (*state).child != 0 as libc::c_int {
        loop
        /* Reap the child. */
        {
            (*state).waitpid_return =
                waitpid((*state).child, &mut (*state).exit_status, 0 as libc::c_int);
            if !((*state).waitpid_return == -(1 as libc::c_int) && errno == EINTR) {
                break;
            }
        }
        (*state).child = 0 as libc::c_int
    }
    if (*state).waitpid_return < 0 as libc::c_int {
        /* waitpid() failed?  This is ugly. */
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Child process exited badly\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    if (((*state).exit_status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
        as libc::c_int
        >> 1 as libc::c_int
        > 0 as libc::c_int
    {
        /* If the child died because we stopped reading before
         * it was done, that's okay.  Some archive formats
         * have padding at the end that we routinely ignore. */
        /* The alternative to this would be to add a step
         * before close(child_stdout) above to read from the
         * child until the child has no more to write. */
        if (*state).exit_status & 0x7f as libc::c_int == SIGPIPE {
            return 0 as libc::c_int;
        }
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Child process exited with signal %d\x00" as *const u8 as *const libc::c_char,
            (*state).exit_status & 0x7f as libc::c_int,
        );
        return -(20 as libc::c_int);
    }
    /* !_WIN32 || __CYGWIN__ */
    if (*state).exit_status & 0x7f as libc::c_int == 0 as libc::c_int {
        if ((*state).exit_status & 0xff00 as libc::c_int) >> 8 as libc::c_int == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Child process exited with status %d\x00" as *const u8 as *const libc::c_char,
            ((*state).exit_status & 0xff00 as libc::c_int) >> 8 as libc::c_int,
        );
        return -(20 as libc::c_int);
    }
    return -(20 as libc::c_int);
}
/*
 * Use select() to decide whether the child is ready for read or write.
 */
unsafe extern "C" fn child_read(
    mut self_0: *mut archive_read_filter,
    mut buf: *mut libc::c_char,
    mut buf_len: size_t,
) -> ssize_t {
    let mut state: *mut program_filter = (*self_0).data as *mut program_filter;
    let mut ret: ssize_t = 0;
    let mut requested: ssize_t = 0;
    let mut avail: ssize_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    requested = if buf_len > SSIZE_MAX as libc::c_ulong {
        SSIZE_MAX as libc::c_ulong
    } else {
        buf_len
    } as ssize_t;
    loop {
        loop {
            ret = read(
                (*state).child_stdout,
                buf as *mut libc::c_void,
                requested as size_t,
            );
            if !(ret == -(1 as libc::c_int) as libc::c_long && errno == EINTR) {
                break;
            }
        }
        if ret > 0 as libc::c_int as libc::c_long {
            return ret;
        }
        if ret == 0 as libc::c_int as libc::c_long
            || ret == -(1 as libc::c_int) as libc::c_long && errno == EPIPE
        {
            /* Child has closed its output; reap the child
             * and return the status. */
            return child_stop(self_0, state) as ssize_t;
        }
        if ret == -(1 as libc::c_int) as libc::c_long && errno != EAGAIN {
            return -(1 as libc::c_int) as ssize_t;
        }
        if (*state).child_stdin == -(1 as libc::c_int) {
            /* Block until child has some I/O ready. */
            __archive_check_child((*state).child_stdin, (*state).child_stdout);
        } else {
            /* Get some more data from upstream. */
            p = __archive_read_filter_ahead(
                (*self_0).upstream,
                1 as libc::c_int as size_t,
                &mut avail,
            ) as *const libc::c_char;
            if p.is_null() {
                close((*state).child_stdin);
                (*state).child_stdin = -(1 as libc::c_int);
                fcntl((*state).child_stdout, F_SETFL, 0 as libc::c_int);
                if avail < 0 as libc::c_int as libc::c_long {
                    return avail;
                }
            } else {
                loop {
                    ret = write(
                        (*state).child_stdin,
                        p as *const libc::c_void,
                        avail as size_t,
                    );
                    if !(ret == -(1 as libc::c_int) as libc::c_long && errno == EINTR) {
                        break;
                    }
                }
                if ret > 0 as libc::c_int as libc::c_long {
                    /* Consume whatever we managed to write. */
                    __archive_read_filter_consume((*self_0).upstream, ret);
                } else if ret == -(1 as libc::c_int) as libc::c_long && errno == EAGAIN {
                    /* Block until child has some I/O ready. */
                    __archive_check_child((*state).child_stdin, (*state).child_stdout);
                } else {
                    /* Write failed. */
                    close((*state).child_stdin);
                    (*state).child_stdin = -(1 as libc::c_int);
                    fcntl((*state).child_stdout, F_SETFL, 0 as libc::c_int);
                    /* If it was a bad error, we're done; otherwise
                     * it was EPIPE or EOF, and we can still read
                     * from the child. */
                    if ret == -(1 as libc::c_int) as libc::c_long && errno != EPIPE {
                        return -(1 as libc::c_int) as ssize_t;
                    }
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __archive_read_program(
    mut self_0: *mut archive_read_filter,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    let mut state: *mut program_filter = 0 as *mut program_filter;
    static mut out_buf_len: size_t = 65536 as libc::c_int as size_t;
    let mut out_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *const libc::c_char = b"Program: \x00" as *const u8 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut l: size_t = 0;
    l = strlen(prefix)
        .wrapping_add(strlen(cmd))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    state = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<program_filter>() as libc::c_ulong,
    ) as *mut program_filter;
    out_buf = malloc(out_buf_len) as *mut libc::c_char;
    if state.is_null()
        || out_buf.is_null()
        || archive_string_ensure(&mut (*state).description, l).is_null()
    {
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate input data\x00" as *const u8 as *const libc::c_char,
        );
        if !state.is_null() {
            archive_string_free(&mut (*state).description);
            free(state as *mut libc::c_void);
        }
        free(out_buf as *mut libc::c_void);
        return -(30 as libc::c_int);
    }
    (*state).description.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*state).description,
        prefix as *const libc::c_void,
        (if prefix.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(prefix)
        }),
    );
    archive_strcat(&mut (*state).description, cmd as *const libc::c_void);
    (*self_0).code = ARCHIVE_FILTER_PROGRAM;
    (*self_0).name = (*state).description.s;
    (*state).out_buf = out_buf;
    (*state).out_buf_len = out_buf_len;
    ret = __archive_create_child(
        cmd,
        &mut (*state).child_stdin,
        &mut (*state).child_stdout,
        &mut (*state).child,
    );
    if ret != ARCHIVE_OK {
        free((*state).out_buf as *mut libc::c_void);
        archive_string_free(&mut (*state).description);
        free(state as *mut libc::c_void);
        archive_set_error(
            &mut (*(*self_0).archive).archive as *mut archive,
            EINVAL,
            b"Can\'t initialize filter; unable to run program \"%s\"\x00" as *const u8
                as *const libc::c_char,
            cmd,
        );
        return -(30 as libc::c_int);
    }
    (*self_0).data = state as *mut libc::c_void;
    (*self_0).read = Some(
        program_filter_read
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
        program_filter_close as unsafe extern "C" fn(_: *mut archive_read_filter) -> libc::c_int,
    );
    /* XXX Check that we can read at least one byte? */
    return 0 as libc::c_int;
}
unsafe extern "C" fn program_bidder_init(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut bidder_state: *mut program_bidder = 0 as *mut program_bidder;
    bidder_state = (*(*self_0).bidder).data as *mut program_bidder;
    return __archive_read_program(self_0, (*bidder_state).cmd);
}
unsafe extern "C" fn program_filter_read(
    mut self_0: *mut archive_read_filter,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut state: *mut program_filter = 0 as *mut program_filter;
    let mut bytes: ssize_t = 0;
    let mut total: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    state = (*self_0).data as *mut program_filter;
    total = 0 as libc::c_int as size_t;
    p = (*state).out_buf;
    while (*state).child_stdout != -(1 as libc::c_int) && total < (*state).out_buf_len {
        bytes = child_read(self_0, p, (*state).out_buf_len.wrapping_sub(total));
        if bytes < 0 as libc::c_int as libc::c_long {
            /* No recovery is possible if we can no longer
             * read from the child. */
            return -(30 as libc::c_int) as ssize_t;
        }
        if bytes == 0 as libc::c_int as libc::c_long {
            break;
        }
        total = (total as libc::c_ulong).wrapping_add(bytes as libc::c_ulong) as size_t as size_t;
        p = p.offset(bytes as isize)
    }
    *buff = (*state).out_buf as *const libc::c_void;
    return total as ssize_t;
}
unsafe extern "C" fn program_filter_close(mut self_0: *mut archive_read_filter) -> libc::c_int {
    let mut state: *mut program_filter = 0 as *mut program_filter;
    let mut e: libc::c_int = 0;
    state = (*self_0).data as *mut program_filter;
    e = child_stop(self_0, state);
    /* Release our private data. */
    free((*state).out_buf as *mut libc::c_void);
    archive_string_free(&mut (*state).description);
    free(state as *mut libc::c_void);
    return e;
}
