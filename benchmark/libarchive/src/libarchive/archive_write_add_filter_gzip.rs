use ::libc;
extern "C" {
    pub type internal_state;
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn deflateInit2_(
        strm: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_write_get_bytes_per_block(_: *mut archive) -> libc::c_int;
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
    #[no_mangle]
    fn __archive_write_filters_free(_: *mut archive);
    #[no_mangle]
    fn __archive_write_allocate_filter(_: *mut archive) -> *mut archive_write_filter;
    #[no_mangle]
    fn __archive_write_filter(
        _: *mut archive_write_filter,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
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
/* The la_ssize_t should match the type used in 'struct stat' */
/* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
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
/* Returns size actually written, zero on EOF, -1 on error. */
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
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
/* Don't compile this if we don't have zlib. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct private_data {
    pub compression_level: libc::c_int,
    pub timestamp: libc::c_int,
    pub stream: z_stream,
    pub total_in: int64_t,
    pub compressed: *mut libc::c_uchar,
    pub compressed_buffer_size: size_t,
    pub crc: libc::c_ulong,
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
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_STREAM_ERROR: libc::c_int = -(2 as libc::c_int);
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const Z_VERSION_ERROR: libc::c_int = -(6 as libc::c_int);
pub const Z_DEFAULT_COMPRESSION: libc::c_int = -(1 as libc::c_int);
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
pub const ARCHIVE_FILTER_GZIP: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_WRITE_MAGIC: libc::c_uint = 0xb0c5c0de as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_compression_gzip(mut a: *mut archive) -> libc::c_int {
    __archive_write_filters_free(a);
    return archive_write_add_filter_gzip(a);
}
/* This sets the first data object. */
/* This sets data object at specified index */
/* This adds a data object at the specified index. */
/* This appends a data object to the end of list */
/* This prepends a data object to the beginning of list */
/* Opening freezes the callbacks. */
/* Convenience wrappers around the above. */
/*
 * A variety of shortcuts that invoke archive_read_open() with
 * canned callbacks suitable for common situations.  The ones that
 * accept a block size handle tape blocking correctly.
 */
/* Use this if you know the filename.  Note: NULL indicates stdin. */
/* Use this for reading multivolume files by filenames.
 * NOTE: Must be NULL terminated. Sorting is NOT done. */
/* archive_read_open_file() is a deprecated synonym for ..._open_filename(). */
/* Read an archive that's stored in memory. */
/* A more involved version that is only used for internal testing. */
/* Read an archive that's already open, using the file descriptor. */
/* Read an archive that's already open, using a FILE *. */
/* Note: DO NOT use this with tape drives. */
/* Parses and returns next entry header. */
/* Parses and returns next entry header using the archive_entry passed in */
/*
 * Retrieve the byte offset in UNCOMPRESSED data where last-read
 * header started.
 */
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
/*
 * Returns a bitmask of capabilities that are supported by the archive format reader.
 * If the reader has no special capabilities, ARCHIVE_READ_FORMAT_CAPS_NONE is returned.
 */
/* Read data from the body of an entry.  Similar to read(2). */
/* Seek within the body of an entry.  Similar to lseek(2). */
/*
 * A zero-copy version of archive_read_data that also exposes the file offset
 * of each returned block.  Note that the client has no way to specify
 * the desired size of the block.  The API does guarantee that offsets will
 * be strictly increasing and that returned blocks will not overlap.
 */
/*-
 * Some convenience functions that are built on archive_read_data:
 *  'skip': skips entire entry
 *  'into_buffer': writes data into memory buffer that you provide
 *  'into_fd': writes data to specified filedes
 */
/*
 * Set read options.
 */
/* Apply option to the format only. */
/* Apply option to the filter only. */
/* Apply option to both the format and the filter. */
/* Apply option string to both the format and the filter. */
/*
 * Add a decryption passphrase.
 */
/*-
 * Convenience function to recreate the current entry (whose header
 * has just been read) on disk.
 *
 * This does quite a bit more than just copy data to disk. It also:
 *  - Creates intermediate directories as required.
 *  - Manages directory permissions:  non-writable directories will
 *    be initially created with write permission enabled; when the
 *    archive is closed, dir permissions are edited to the values specified
 *    in the archive.
 *  - Checks hardlinks:  hardlinks will not be extracted unless the
 *    linked-to file was also extracted within the same session. (TODO)
 */
/* The "flags" argument selects optional behavior, 'OR' the flags you want. */
/* Default: Do not try to set owner/group. */
/* Default: Do obey umask, do not restore SUID/SGID/SVTX bits. */
/* Default: Do not restore mtime/atime. */
/* Default: Replace existing files. */
/* Default: Try create first, unlink only if create fails with EEXIST. */
/* Default: Do not restore ACLs. */
/* Default: Do not restore fflags. */
/* Default: Do not restore xattrs. */
/* Default: Do not try to guard against extracts redirected by symlinks. */
/* Note: With ARCHIVE_EXTRACT_UNLINK, will remove any intermediate symlink. */
/* Default: Do not reject entries with '..' as path elements. */
/* Default: Create parent directories as needed. */
/* Default: Overwrite files, even if one on disk is newer. */
/* Detect blocks of 0 and write holes instead. */
/* Default: Do not restore Mac extended metadata. */
/* This has no effect except on Mac OS. */
/* Default: Use HFS+ compression if it was compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not use HFS+ compression if it was not compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not reject entries with absolute paths */
/* Default: Do not clear no-change flags when unlinking object */
/* Default: Do not extract atomically (using rename) */
/* dest */
/* Record the dev/ino of a file that will not be written.  This is
 * generally set to the dev/ino of the archive being read. */
/* Close the file and release most resources. */
/* Release all resources and destroy the object. */
/* Note that archive_read_free will call archive_read_close for you. */
/* Synonym for archive_read_free() for backwards compatibility. */
/*-
 * To create an archive:
 *   1) Ask archive_write_new for an archive writer object.
 *   2) Set any global properties.  In particular, you should set
 *      the compression and format to use.
 *   3) Call archive_write_open to open the file (most people
 *       will use archive_write_open_file or archive_write_open_fd,
 *       which provide convenient canned I/O callbacks for you).
 *   4) For each entry:
 *      - construct an appropriate struct archive_entry structure
 *      - archive_write_header to write the header
 *      - archive_write_data to write the entry data
 *   5) archive_write_close to close the output
 *   6) archive_write_free to cleanup the writer and release resources
 */
/* XXX This is badly misnamed; suggestions appreciated. XXX */
/* The dev/ino of a file that won't be archived.  This is used
 * to avoid recursively adding an archive to itself. */
/* A convenience function to set the filter based on the code. */
/*
 * Add a gzip compression filter to this write handle.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_add_filter_gzip(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut f: *mut archive_write_filter = __archive_write_allocate_filter(_a);
    let mut data: *mut private_data = 0 as *mut private_data;
    let mut magic_test: libc::c_int = __archive_check_magic(
        &mut (*a).archive,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_add_filter_gzip\x00" as *const u8 as *const libc::c_char,
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
    (*f).data = data as *mut libc::c_void;
    (*f).open = Some(
        archive_compressor_gzip_open
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).options = Some(
        archive_compressor_gzip_options
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*f).close = Some(
        archive_compressor_gzip_close
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).free = Some(
        archive_compressor_gzip_free
            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
    );
    (*f).code = ARCHIVE_FILTER_GZIP;
    (*f).name = b"gzip\x00" as *const u8 as *const libc::c_char;
    (*data).compression_level = Z_DEFAULT_COMPRESSION;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_compressor_gzip_free(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    free((*data).compressed as *mut libc::c_void);
    free(data as *mut libc::c_void);
    (*f).data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
 * Yuck.  zlib.h is not const-correct, so I need this one bit
 * of ugly hackery to convert a const * pointer to a non-const pointer.
 */
/*
 * Set write options.
 */
unsafe extern "C" fn archive_compressor_gzip_options(
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
        if value.is_null()
            || !(*value.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                && *value.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
            || *value.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
        {
            return -(20 as libc::c_int);
        }
        (*data).compression_level =
            *value.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
        return 0 as libc::c_int;
    }
    if strcmp(key, b"timestamp\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*data).timestamp = if value.is_null() {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        return 0 as libc::c_int;
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
/*
 * Setup callback.
 */
unsafe extern "C" fn archive_compressor_gzip_open(mut f: *mut archive_write_filter) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    let mut ret: libc::c_int = 0;
    if (*data).compressed.is_null() {
        let mut bs: size_t = 65536 as libc::c_int as size_t;
        let mut bpb: size_t = 0;
        if (*(*f).archive).magic == ARCHIVE_WRITE_MAGIC {
            /* Buffer size should be a multiple number of
             * the of bytes per block for performance. */
            bpb = archive_write_get_bytes_per_block((*f).archive) as size_t;
            if bpb > bs {
                bs = bpb
            } else if bpb != 0 as libc::c_int as libc::c_ulong {
                bs = (bs as libc::c_ulong).wrapping_sub(bs.wrapping_rem(bpb)) as size_t as size_t
            }
        }
        (*data).compressed_buffer_size = bs;
        (*data).compressed = malloc((*data).compressed_buffer_size) as *mut libc::c_uchar;
        if (*data).compressed.is_null() {
            archive_set_error(
                (*f).archive,
                ENOMEM,
                b"Can\'t allocate data for compression buffer\x00" as *const u8
                    as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    (*data).crc = crc32(
        0 as libc::c_long as uLong,
        NULL as *const Bytef,
        0 as libc::c_int as uInt,
    );
    (*data).stream.next_out = (*data).compressed;
    (*data).stream.avail_out = (*data).compressed_buffer_size as uInt;
    /* Prime output buffer with a gzip header. */
    *(*data).compressed.offset(0 as libc::c_int as isize) = 0x1f as libc::c_int as libc::c_uchar; /* GZip signature bytes */
    *(*data).compressed.offset(1 as libc::c_int as isize) = 0x8b as libc::c_int as libc::c_uchar; /* "Deflate" compression */
    *(*data).compressed.offset(2 as libc::c_int as isize) = 0x8 as libc::c_int as libc::c_uchar; /* No options */
    *(*data).compressed.offset(3 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar; /* Timestamp */
    if (*data).timestamp >= 0 as libc::c_int {
        let mut t: time_t = time(NULL as *mut time_t); /* OS=Unix */
        *(*data).compressed.offset(4 as libc::c_int as isize) =
            (t as uint8_t as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        *(*data).compressed.offset(5 as libc::c_int as isize) =
            ((t >> 8 as libc::c_int) as uint8_t as libc::c_int & 0xff as libc::c_int)
                as libc::c_uchar;
        *(*data).compressed.offset(6 as libc::c_int as isize) =
            ((t >> 16 as libc::c_int) as uint8_t as libc::c_int & 0xff as libc::c_int)
                as libc::c_uchar;
        *(*data).compressed.offset(7 as libc::c_int as isize) =
            ((t >> 24 as libc::c_int) as uint8_t as libc::c_int & 0xff as libc::c_int)
                as libc::c_uchar
    } else {
        memset(
            &mut *(*data).compressed.offset(4 as libc::c_int as isize) as *mut libc::c_uchar
                as *mut libc::c_void,
            0 as libc::c_int,
            4 as libc::c_int as libc::c_ulong,
        );
    }
    if (*data).compression_level == 9 as libc::c_int {
        *(*data).compressed.offset(8 as libc::c_int as isize) = 2 as libc::c_int as libc::c_uchar
    } else if (*data).compression_level == 1 as libc::c_int {
        *(*data).compressed.offset(8 as libc::c_int as isize) = 4 as libc::c_int as libc::c_uchar
    } else {
        *(*data).compressed.offset(8 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar
    }
    *(*data).compressed.offset(9 as libc::c_int as isize) = 3 as libc::c_int as libc::c_uchar;
    (*data).stream.next_out = (*data).stream.next_out.offset(10 as libc::c_int as isize);
    (*data).stream.avail_out = ((*data).stream.avail_out as libc::c_uint)
        .wrapping_sub(10 as libc::c_int as libc::c_uint) as uInt
        as uInt;
    (*f).write = Some(
        archive_compressor_gzip_write
            as unsafe extern "C" fn(
                _: *mut archive_write_filter,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_int,
    );
    /* Initialize compression library. */
    ret = deflateInit2_(
        &mut (*data).stream,
        (*data).compression_level,
        8 as libc::c_int,
        -(15 as libc::c_int),
        8 as libc::c_int,
        0 as libc::c_int,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if ret == Z_OK {
        (*f).data = data as *mut libc::c_void;
        return 0 as libc::c_int;
    }
    /* Library setup failed: clean up. */
    archive_set_error(
        (*f).archive,
        ARCHIVE_ERRNO_MISC,
        b"Internal error initializing compression library\x00" as *const u8 as *const libc::c_char,
    );
    /* Override the error message if we know what really went wrong. */
    match ret {
        -2 => {
            archive_set_error(
                (*f).archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal error initializing compression library: invalid setup parameter\x00"
                    as *const u8 as *const libc::c_char,
            );
        }
        -4 => {
            archive_set_error(
                (*f).archive,
                ENOMEM,
                b"Internal error initializing compression library\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        -6 => {
            archive_set_error(
                (*f).archive,
                ARCHIVE_ERRNO_MISC,
                b"Internal error initializing compression library: invalid library version\x00"
                    as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    return -(30 as libc::c_int);
}
/*
 * Write data to the compressed stream.
 */
unsafe extern "C" fn archive_compressor_gzip_write(
    mut f: *mut archive_write_filter,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    let mut ret: libc::c_int = 0;
    /* Update statistics */
    (*data).crc = crc32((*data).crc, buff as *const Bytef, length as uInt);
    (*data).total_in =
        ((*data).total_in as libc::c_ulong).wrapping_add(length) as int64_t as int64_t;
    /* Compress input data to output buffer */
    (*data).stream.next_in = buff as uintptr_t as *mut Bytef;
    (*data).stream.avail_in = length as uInt;
    ret = drive_compressor(f, data, 0 as libc::c_int);
    if ret != ARCHIVE_OK {
        return ret;
    }
    return 0 as libc::c_int;
}
/*
 * Finish the compression...
 */
unsafe extern "C" fn archive_compressor_gzip_close(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut trailer: [libc::c_uchar; 8] = [0; 8];
    let mut data: *mut private_data = (*f).data as *mut private_data;
    let mut ret: libc::c_int = 0;
    /* Finish compression cycle */
    ret = drive_compressor(f, data, 1 as libc::c_int);
    if ret == ARCHIVE_OK {
        /* Write the last compressed data. */
        ret = __archive_write_filter(
            (*f).next_filter,
            (*data).compressed as *const libc::c_void,
            (*data)
                .compressed_buffer_size
                .wrapping_sub((*data).stream.avail_out as libc::c_ulong),
        )
    }
    if ret == ARCHIVE_OK {
        /* Build and write out 8-byte trailer. */
        trailer[0 as libc::c_int as usize] =
            ((*data).crc as uint8_t as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        trailer[1 as libc::c_int as usize] = (((*data).crc >> 8 as libc::c_int) as uint8_t
            as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        trailer[2 as libc::c_int as usize] = (((*data).crc >> 16 as libc::c_int) as uint8_t
            as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        trailer[3 as libc::c_int as usize] = (((*data).crc >> 24 as libc::c_int) as uint8_t
            as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        trailer[4 as libc::c_int as usize] =
            ((*data).total_in as uint8_t as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
        trailer[5 as libc::c_int as usize] = (((*data).total_in >> 8 as libc::c_int) as uint8_t
            as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        trailer[6 as libc::c_int as usize] = (((*data).total_in >> 16 as libc::c_int) as uint8_t
            as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        trailer[7 as libc::c_int as usize] = (((*data).total_in >> 24 as libc::c_int) as uint8_t
            as libc::c_int
            & 0xff as libc::c_int) as libc::c_uchar;
        ret = __archive_write_filter(
            (*f).next_filter,
            trailer.as_mut_ptr() as *const libc::c_void,
            8 as libc::c_int as size_t,
        )
    }
    match deflateEnd(&mut (*data).stream) {
        Z_OK => {}
        _ => {
            archive_set_error(
                (*f).archive,
                ARCHIVE_ERRNO_MISC,
                b"Failed to clean up compressor\x00" as *const u8 as *const libc::c_char,
            );
            ret = ARCHIVE_FATAL
        }
    }
    return ret;
}
/*
 * Utility function to push input data through compressor,
 * writing full output blocks as necessary.
 *
 * Note that this handles both the regular write case (finishing ==
 * false) and the end-of-archive case (finishing == true).
 */
unsafe extern "C" fn drive_compressor(
    mut f: *mut archive_write_filter,
    mut data: *mut private_data,
    mut finishing: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        if (*data).stream.avail_out == 0 as libc::c_int as libc::c_uint {
            ret = __archive_write_filter(
                (*f).next_filter,
                (*data).compressed as *const libc::c_void,
                (*data).compressed_buffer_size,
            );
            if ret != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
            (*data).stream.next_out = (*data).compressed;
            (*data).stream.avail_out = (*data).compressed_buffer_size as uInt
        }
        /* If there's nothing to do, we're done. */
        if finishing == 0 && (*data).stream.avail_in == 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        ret = deflate(
            &mut (*data).stream,
            if finishing != 0 { Z_FINISH } else { Z_NO_FLUSH },
        );
        match ret {
            Z_OK => {
                /* In non-finishing case, check if compressor
                 * consumed everything */
                if finishing == 0 && (*data).stream.avail_in == 0 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int;
                }
            }
            Z_STREAM_END => {
                /* This return can only occur in finishing case. */
                return 0 as libc::c_int;
            }
            _ => {
                /* Any other return value indicates an error. */
                archive_set_error(
                    (*f).archive,
                    ARCHIVE_ERRNO_MISC,
                    b"GZip compression failed: deflate() call returned status %d\x00" as *const u8
                        as *const libc::c_char,
                    ret,
                );
                return -(30 as libc::c_int);
            }
        }
    }
}
/* HAVE_ZLIB_H */
/* HAVE_ZLIB_H */
