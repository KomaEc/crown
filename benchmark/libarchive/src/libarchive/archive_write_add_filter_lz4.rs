use ::c2rust_bitfields;
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
    pub type archive_write_program_data;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /*
     * Set behavior. The "flags" argument selects optional behavior.
     */
    /* Request that the access time of the entry visited by traversal be restored.
     * This is the same as archive_read_disk_set_atime_restored. */
    /* Default: Do not skip an entry which has nodump flags. */
    /* Default: Skip a mac resource fork file whose prefix is "._" because of
     * using copyfile. */
    /* Default: Traverse mount points. */
    /* Default: Xattrs are read from disk. */
    /* Default: ACLs are read from disk. */
    /* Default: File flags are read from disk. */
    /*
     * Set archive_match object that will be used in archive_read_disk to
     * know whether an entry should be skipped. The callback function
     * _excluded_func will be invoked when an entry is skipped by the result
     * of archive_match.
     */
    /* Simplified cleanup interface;
     * This calls archive_read_free() or archive_write_free() as needed. */
    /*
     * Accessor functions to read/set various information in
     * the struct archive object:
     */
    /* Number of filters in the current filter pipeline. */
    /* Filter #0 is the one closest to the format, -1 is a synonym for the
     * last filter, which is always the pseudo-filter that wraps the
     * client callbacks. */
    /* These don't properly handle multiple filters, so are deprecated and
     * will eventually be removed. */
    /* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, -1); */
    /* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, 0); */
    /* As of libarchive 3.0, this is an alias for archive_filter_name(a, 0); */
    /* As of libarchive 3.0, this is an alias for archive_filter_code(a, 0); */
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
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    /* Append a C string to an archive_string, resizing as necessary. */
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    /* Copy a C string to an archive_string, resizing as necessary. */
    /* Copy a C string to an archive_string with limit, resizing as necessary. */
    /* Return length of string. */
    /* Set string length to zero. */
    /* Release any allocated storage resources. */
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn __archive_write_allocate_filter(_: *mut archive) -> *mut archive_write_filter;
    #[no_mangle]
    fn __archive_write_program_allocate(
        program_name: *const libc::c_char,
    ) -> *mut archive_write_program_data;
    #[no_mangle]
    fn __archive_write_program_free(_: *mut archive_write_program_data) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_program_open(
        _: *mut archive_write_filter,
        _: *mut archive_write_program_data,
        _: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_program_close(
        _: *mut archive_write_filter,
        _: *mut archive_write_program_data,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_program_write(
        _: *mut archive_write_filter,
        _: *mut archive_write_program_data,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
pub type archive_write_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *const libc::c_void,
    _: size_t,
) -> la_ssize_t;
pub type archive_open_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write {
    pub archive: archive,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub nulls: *const libc::c_uchar,
    pub null_length: size_t,
    pub client_opener: Option<archive_open_callback>,
    pub client_writer: Option<archive_write_callback>,
    pub client_closer: Option<archive_close_callback>,
    pub client_data: *mut libc::c_void,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub filter_first: *mut archive_write_filter,
    pub filter_last: *mut archive_write_filter,
    pub format_data: *mut libc::c_void,
    pub format_name: *const libc::c_char,
    pub format_init: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub format_finish_entry: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_write_header:
        Option<unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int>,
    pub format_write_data: Option<
        unsafe extern "C" fn(_: *mut archive_write, _: *const libc::c_void, _: size_t) -> ssize_t,
    >,
    pub format_close: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_free: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub passphrase: *mut libc::c_char,
    pub passphrase_callback: Option<archive_passphrase_callback>,
    pub passphrase_client_data: *mut libc::c_void,
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
 * $FreeBSD: head/lib/libarchive/archive_write_private.h 201155 2009-12-29 05:20:12Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write_filter {
    pub bytes_written: int64_t,
    pub archive: *mut archive,
    pub next_filter: *mut archive_write_filter,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub write: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct private_data {
    pub compression_level: libc::c_int,
    #[bitfield(name = "header_written", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "version_number", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "block_independence", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "block_checksum", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "stream_size", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "stream_checksum", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "preset_dictionary", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "block_maximum_size", ty = "libc::c_uint", bits = "7..=9")]
    pub header_written_version_number_block_independence_block_checksum_stream_size_stream_checksum_preset_dictionary_block_maximum_size:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub pdata: *mut archive_write_program_data,
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FILTER_LZ4: libc::c_int = 13 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * Add a lz4 compression filter to this write handle.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_add_filter_lz4(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut f: *mut archive_write_filter = __archive_write_allocate_filter(_a);
    let mut data: *mut private_data = 0 as *mut private_data;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_add_filter_lz4\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<private_data>() as libc::c_ulong,
    ) as *mut private_data;
    if data.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * Setup default settings.
     */
    (*data).compression_level = 1 as libc::c_int;
    (*data).set_version_number(0x1 as libc::c_int as libc::c_uint);
    (*data).set_block_independence(1 as libc::c_int as libc::c_uint);
    (*data).set_block_checksum(0 as libc::c_int as libc::c_uint);
    (*data).set_stream_size(0 as libc::c_int as libc::c_uint);
    (*data).set_stream_checksum(1 as libc::c_int as libc::c_uint);
    (*data).set_preset_dictionary(0 as libc::c_int as libc::c_uint);
    (*data).set_block_maximum_size(7 as libc::c_int as libc::c_uint);
    /*
     * Setup a filter setting.
     */
    (*f).data = data as *mut libc::c_void;
    (*f).options = Some(
        archive_filter_lz4_options
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*f).close = Some(
        archive_filter_lz4_close
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).free = Some(
        archive_filter_lz4_free
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).open = Some(
        archive_filter_lz4_open
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).code = ARCHIVE_FILTER_LZ4;
    (*f).name = b"lz4\x00" as *const u8 as *const libc::c_char;
    /*
     * We don't have lz4 library, and execute external lz4 program
     * instead.
     */
    (*data).pdata =
        __archive_write_program_allocate(b"lz4\x00" as *const u8 as *const libc::c_char);
    if (*data).pdata.is_null() {
        free(data as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*data).compression_level = 0 as libc::c_int;
    archive_set_error(
        &mut (*a).archive as *mut archive,
        ARCHIVE_ERRNO_MISC,
        b"Using external lz4 program\x00" as *const u8 as *const libc::c_char,
    );
    return -(20 as libc::c_int);
}
/*
 * Set write options.
 */
unsafe extern "C" fn archive_filter_lz4_options(
    mut f: *mut archive_write_filter,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    if strcmp(
        key,
        b"compression-level\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        let mut val: libc::c_int = 0;
        if value.is_null()
            || {
                val = *value.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
                !(val >= 1 as libc::c_int && val <= 9 as libc::c_int)
            }
            || *value.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        {
            return -(20 as libc::c_int);
        }
        if val >= 3 as libc::c_int {
            archive_set_error(
                (*f).archive,
                ARCHIVE_ERRNO_PROGRAMMER,
                b"High compression not included in this build\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*data).compression_level = val;
        return 0 as libc::c_int;
    }
    if strcmp(
        key,
        b"stream-checksum\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*data).set_stream_checksum(
            (value != NULL as *const libc::c_char) as libc::c_int as libc::c_uint,
        );
        return 0 as libc::c_int;
    }
    if strcmp(
        key,
        b"block-checksum\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*data).set_block_checksum(
            (value != NULL as *const libc::c_char) as libc::c_int as libc::c_uint,
        );
        return 0 as libc::c_int;
    }
    if strcmp(key, b"block-size\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if value.is_null()
            || !(*value.offset(0 as libc::c_int as isize) as libc::c_int >= '4' as i32
                && *value.offset(0 as libc::c_int as isize) as libc::c_int <= '7' as i32)
            || *value.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        {
            return -(20 as libc::c_int);
        }
        (*data).set_block_maximum_size(
            (*value.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32) as libc::c_uint,
        );
        return 0 as libc::c_int;
    }
    if strcmp(
        key,
        b"block-dependence\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*data).set_block_independence(
            (value == NULL as *const libc::c_char) as libc::c_int as libc::c_uint,
        );
        return 0 as libc::c_int;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
/* HAVE_LIBLZ4 */
unsafe extern "C" fn archive_filter_lz4_open(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    let mut as_0: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut r: libc::c_int = 0;
    as_0.s = NULL as *mut libc::c_char;
    as_0.length = 0 as libc::c_int as size_t;
    as_0.buffer_length = 0 as libc::c_int as size_t;
    as_0.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut as_0,
        b"lz4 -z -q -q\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        (if (b"lz4 -z -q -q\x00" as *const u8 as *const libc::c_char).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(b"lz4 -z -q -q\x00" as *const u8 as *const libc::c_char)
        }),
    );
    /* Specify a compression level. */
    if (*data).compression_level > 0 as libc::c_int {
        archive_strcat(
            &mut as_0,
            b" -\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
        archive_strappend_char(
            &mut as_0,
            ('0' as i32 + (*data).compression_level) as libc::c_char,
        );
    }
    /* Specify a block size. */
    archive_strcat(
        &mut as_0,
        b" -B\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
    );
    archive_strappend_char(
        &mut as_0,
        ('0' as i32 + (*data).block_maximum_size() as libc::c_int) as libc::c_char,
    );
    if (*data).block_checksum() != 0 {
        archive_strcat(
            &mut as_0,
            b" -BX\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
    }
    if (*data).stream_checksum() as libc::c_int == 0 as libc::c_int {
        archive_strcat(
            &mut as_0,
            b" --no-frame-crc\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
    }
    if (*data).block_independence() as libc::c_int == 0 as libc::c_int {
        archive_strcat(
            &mut as_0,
            b" -BD\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        );
    }
    (*f).write = Some(
        archive_filter_lz4_write
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_int,
    );
    r = __archive_write_program_open(f, (*data).pdata, as_0.s);
    archive_string_free(&mut as_0);
    return r;
}
unsafe extern "C" fn archive_filter_lz4_write(
    mut f: *mut archive_write_filter,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    return __archive_write_program_write(f, (*data).pdata, buff, length);
}
unsafe extern "C" fn archive_filter_lz4_close(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    return __archive_write_program_close(f, (*data).pdata);
}
unsafe extern "C" fn archive_filter_lz4_free(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    __archive_write_program_free((*data).pdata);
    free(data as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* HAVE_LIBLZ4 */
