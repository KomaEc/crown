use ::libc;
extern "C" {
    pub type _xmlTextReader;
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
    pub type evp_md_ctx_st;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn xmlTextReaderSetErrorHandler(
        reader: xmlTextReaderPtr,
        f: xmlTextReaderErrorFunc,
        arg: *mut libc::c_void,
    );
    #[no_mangle]
    fn xmlReaderForIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut libc::c_void,
        URL: *const libc::c_char,
        encoding: *const libc::c_char,
        options: libc::c_int,
    ) -> xmlTextReaderPtr;
    #[no_mangle]
    fn xmlTextReaderMoveToNextAttribute(reader: xmlTextReaderPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlTextReaderMoveToFirstAttribute(reader: xmlTextReaderPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlCleanupParser();
    #[no_mangle]
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    #[no_mangle]
    fn xmlTextReaderRead(reader: xmlTextReaderPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlTextReaderIsEmptyElement(reader: xmlTextReaderPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlTextReaderNodeType(reader: xmlTextReaderPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlTextReaderConstLocalName(reader: xmlTextReaderPtr) -> *const xmlChar;
    #[no_mangle]
    fn xmlTextReaderConstValue(reader: xmlTextReaderPtr) -> *const xmlChar;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateReset(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn inflateInit_(
        strm: z_streamp,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn timegm(__tp: *mut tm) -> time_t;
    /* defines */
    /* Minimal interface to digest functionality for internal use in libarchive */
    /* Message Digest */
    #[no_mangle]
    static __archive_digest: archive_digest;
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
    fn archive_entry_set_dev(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_devmajor(_: *mut archive_entry, _: dev_t);
    #[no_mangle]
    fn archive_entry_set_devminor(_: *mut archive_entry, _: dev_t);
    /* Returns pointer to start of first invalid token, or NULL if none. */
    /* Note that all recognized tokens are processed, regardless. */
    #[no_mangle]
    fn archive_entry_copy_fflags_text(
        _: *mut archive_entry,
        _: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_hardlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_ino64(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_xattr_add_entry(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: *const libc::c_void,
        _: size_t,
    );
    #[no_mangle]
    fn _archive_entry_copy_gname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
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
    fn _archive_entry_copy_uname_l(
        _: *mut archive_entry,
        _: *const libc::c_char,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn archive_string_concat(dest: *mut archive_string, src: *mut archive_string);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_conversion_from_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
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
    fn __archive_read_seek(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t;
    #[no_mangle]
    fn __archive_read_consume(_: *mut archive_read, _: int64_t) -> int64_t;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
/*
 * Summary: set of routines to process strings
 * Description: type and interfaces needed for the internal string handling
 *              of the library, especially UTF8 processing.
 *
 * Copy: See Copyright for the status of this software.
 *
 * Author: Daniel Veillard
 */
/* *
 * xmlChar:
 *
 * This is a basic byte in an UTF-8 encoded string.
 * It's unsigned allowing to pinpoint case where char * are assigned
 * to xmlChar * (possibly making serialization back impossible).
 */
pub type xmlChar = libc::c_uchar;
pub type xmlInputCloseCallback = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
pub type xmlInputReadCallback = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int,
>;
pub type xmlParserSeverities = libc::c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed = 0;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
pub type xmlTextReaderLocatorPtr = *mut libc::c_void;
pub type xmlTextReaderErrorFunc = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: xmlParserSeverities,
        _: xmlTextReaderLocatorPtr,
    ) -> (),
>;
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
pub struct xar {
    pub offset: uint64_t,
    pub total: int64_t,
    pub h_base: uint64_t,
    pub end_of_file: libc::c_int,
    pub outbuff: *mut libc::c_uchar,
    pub xmlsts: xmlstatus,
    pub xmlsts_unknown: xmlstatus,
    pub unknowntags: *mut unknown_tag,
    pub base64text: libc::c_int,
    pub toc_remaining: uint64_t,
    pub toc_total: uint64_t,
    pub toc_chksum_offset: uint64_t,
    pub toc_chksum_size: uint64_t,
    pub rd_encoding: enctype,
    pub stream: z_stream,
    pub stream_valid: libc::c_int,
    pub a_sumwrk: chksumwork,
    pub e_sumwrk: chksumwork,
    pub file: *mut xar_file,
    pub xattr: *mut xattr,
    pub file_queue: heap_queue,
    pub hdlink_orgs: *mut xar_file,
    pub hdlink_list: *mut hdlink,
    pub entry_init: libc::c_int,
    pub entry_total: uint64_t,
    pub entry_remaining: uint64_t,
    pub entry_unconsumed: size_t,
    pub entry_size: uint64_t,
    pub entry_encoding: enctype,
    pub entry_a_sum: chksumval,
    pub entry_e_sum: chksumval,
    pub sconv: *mut archive_string_conv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chksumval {
    pub alg: libc::c_int,
    pub len: size_t,
    pub val: [libc::c_uchar; 20],
}
pub type enctype = libc::c_uint;
pub const XZ: enctype = 4;
pub const LZMA: enctype = 3;
pub const BZIP2: enctype = 2;
pub const GZIP: enctype = 1;
pub const NONE: enctype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdlink {
    pub next: *mut hdlink,
    pub id: libc::c_uint,
    pub cnt: libc::c_int,
    pub files: *mut xar_file,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xar_file {
    pub next: *mut xar_file,
    pub hdnext: *mut xar_file,
    pub parent: *mut xar_file,
    pub subdirs: libc::c_int,
    pub has: libc::c_uint,
    pub id: uint64_t,
    pub length: uint64_t,
    pub offset: uint64_t,
    pub size: uint64_t,
    pub encoding: enctype,
    pub a_sum: chksumval,
    pub e_sum: chksumval,
    pub pathname: archive_string,
    pub symlink: archive_string,
    pub ctime: time_t,
    pub mtime: time_t,
    pub atime: time_t,
    pub uname: archive_string,
    pub uid: int64_t,
    pub gname: archive_string,
    pub gid: int64_t,
    pub mode: mode_t,
    pub dev: dev_t,
    pub devmajor: dev_t,
    pub devminor: dev_t,
    pub ino64: int64_t,
    pub fflags_text: archive_string,
    pub link: libc::c_uint,
    pub nlink: libc::c_uint,
    pub hardlink: archive_string,
    pub xattr_list: *mut xattr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr {
    pub next: *mut xattr,
    pub name: archive_string,
    pub id: uint64_t,
    pub length: uint64_t,
    pub offset: uint64_t,
    pub size: uint64_t,
    pub encoding: enctype,
    pub a_sum: chksumval,
    pub e_sum: chksumval,
    pub fstype: archive_string,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_queue {
    pub files: *mut *mut xar_file,
    pub allocated: libc::c_int,
    pub used: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chksumwork {
    pub alg: libc::c_int,
    pub md5ctx: archive_md5_ctx,
    pub sha1ctx: archive_sha1_ctx,
}
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
/*
 * Crypto support in various Operating Systems:
 *
 * NetBSD:
 * - MD5 and SHA1 in libc: without _ after algorithm name
 * - SHA2 in libc: with _ after algorithm name
 *
 * OpenBSD:
 * - MD5, SHA1 and SHA2 in libc: without _ after algorithm name
 * - OpenBSD 4.4 and earlier have SHA2 in libc with _ after algorithm name
 *
 * DragonFly and FreeBSD:
 * - MD5 libmd: without _ after algorithm name
 * - SHA1, SHA256 and SHA512 in libmd: with _ after algorithm name
 *
 * Mac OS X (10.4 and later):
 * - MD5, SHA1 and SHA2 in libSystem: with CC_ prefix and _ after algorithm name
 *
 * OpenSSL:
 * - MD5, SHA1 and SHA2 in libcrypto: with _ after algorithm name
 *
 * Windows:
 * - MD5, SHA1 and SHA2 in archive_crypto.c using Windows crypto API
 */
/* libc crypto headers */
/* libmd crypto headers */
/* libSystem crypto headers */
/* mbed TLS crypto headers */
/* Nettle crypto headers */
/* OpenSSL crypto headers */
/* Windows crypto headers */
/* typedefs */
pub type archive_sha1_ctx = *mut EVP_MD_CTX;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type archive_md5_ctx = *mut EVP_MD_CTX;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unknown_tag {
    pub next: *mut unknown_tag,
    pub name: archive_string,
}
pub type xmlstatus = libc::c_uint;
pub const UNKNOWN: xmlstatus = 77;
pub const FILE_EXT2_Reserved: xmlstatus = 76;
pub const FILE_EXT2_TopDir: xmlstatus = 75;
pub const FILE_EXT2_DirSync: xmlstatus = 74;
pub const FILE_EXT2_NoTail: xmlstatus = 73;
pub const FILE_EXT2_Journaled: xmlstatus = 72;
pub const FILE_EXT2_iMagic: xmlstatus = 71;
pub const FILE_EXT2_HashIndexed: xmlstatus = 70;
pub const FILE_EXT2_BTree: xmlstatus = 69;
pub const FILE_EXT2_CompError: xmlstatus = 68;
pub const FILE_EXT2_NoCompBlock: xmlstatus = 67;
pub const FILE_EXT2_CompBlock: xmlstatus = 66;
pub const FILE_EXT2_CompDirty: xmlstatus = 65;
pub const FILE_EXT2_NoAtime: xmlstatus = 64;
pub const FILE_EXT2_NoDump: xmlstatus = 63;
pub const FILE_EXT2_AppendOnly: xmlstatus = 62;
pub const FILE_EXT2_Immutable: xmlstatus = 61;
pub const FILE_EXT2_Synchronous: xmlstatus = 60;
pub const FILE_EXT2_Compress: xmlstatus = 59;
pub const FILE_EXT2_Undelete: xmlstatus = 58;
pub const FILE_EXT2_SecureDeletion: xmlstatus = 57;
/* Linux file flags. */
pub const FILE_EXT2: xmlstatus = 56;
pub const FILE_FLAGS_SYS_SNAPSHOT: xmlstatus = 55;
pub const FILE_FLAGS_SYS_NOUNLINK: xmlstatus = 54;
pub const FILE_FLAGS_SYS_APPEND: xmlstatus = 53;
pub const FILE_FLAGS_SYS_IMMUTABLE: xmlstatus = 52;
pub const FILE_FLAGS_SYS_ARCHIVED: xmlstatus = 51;
pub const FILE_FLAGS_USER_NOUNLINK: xmlstatus = 50;
pub const FILE_FLAGS_USER_OPAQUE: xmlstatus = 49;
pub const FILE_FLAGS_USER_APPEND: xmlstatus = 48;
pub const FILE_FLAGS_USER_IMMUTABLE: xmlstatus = 47;
pub const FILE_FLAGS_USER_NODUMP: xmlstatus = 46;
/* BSD file flags. */
pub const FILE_FLAGS: xmlstatus = 45;
pub const FILE_ACL_APPLEEXTENDED: xmlstatus = 44;
pub const FILE_ACL_ACCESS: xmlstatus = 43;
pub const FILE_ACL_DEFAULT: xmlstatus = 42;
pub const FILE_ACL: xmlstatus = 41;
pub const FILE_NAME: xmlstatus = 40;
pub const FILE_TYPE: xmlstatus = 39;
pub const FILE_LINK: xmlstatus = 38;
pub const FILE_INODE: xmlstatus = 37;
pub const FILE_DEVICENO: xmlstatus = 36;
pub const FILE_DEVICE_MINOR: xmlstatus = 35;
pub const FILE_DEVICE_MAJOR: xmlstatus = 34;
pub const FILE_DEVICE: xmlstatus = 33;
pub const FILE_MODE: xmlstatus = 32;
pub const FILE_UID: xmlstatus = 31;
pub const FILE_USER: xmlstatus = 30;
pub const FILE_GID: xmlstatus = 29;
pub const FILE_GROUP: xmlstatus = 28;
pub const FILE_ATIME: xmlstatus = 27;
pub const FILE_MTIME: xmlstatus = 26;
pub const FILE_CTIME: xmlstatus = 25;
pub const FILE_EA_FSTYPE: xmlstatus = 24;
pub const FILE_EA_NAME: xmlstatus = 23;
pub const FILE_EA_E_CHECKSUM: xmlstatus = 22;
pub const FILE_EA_A_CHECKSUM: xmlstatus = 21;
pub const FILE_EA_ENCODING: xmlstatus = 20;
pub const FILE_EA_SIZE: xmlstatus = 19;
pub const FILE_EA_OFFSET: xmlstatus = 18;
pub const FILE_EA_LENGTH: xmlstatus = 17;
pub const FILE_EA: xmlstatus = 16;
pub const FILE_DATA_CONTENT: xmlstatus = 15;
pub const FILE_DATA_E_CHECKSUM: xmlstatus = 14;
pub const FILE_DATA_A_CHECKSUM: xmlstatus = 13;
pub const FILE_DATA_ENCODING: xmlstatus = 12;
pub const FILE_DATA_SIZE: xmlstatus = 11;
pub const FILE_DATA_OFFSET: xmlstatus = 10;
pub const FILE_DATA_LENGTH: xmlstatus = 9;
pub const FILE_DATA: xmlstatus = 8;
pub const TOC_FILE: xmlstatus = 7;
pub const TOC_CHECKSUM_SIZE: xmlstatus = 6;
pub const TOC_CHECKSUM_OFFSET: xmlstatus = 5;
pub const TOC_CHECKSUM: xmlstatus = 4;
pub const TOC_CREATION_TIME: xmlstatus = 3;
pub const TOC: xmlstatus = 2;
pub const XAR: xmlstatus = 1;
pub const INIT: xmlstatus = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_digest {
    pub md5init: Option<unsafe extern "C" fn(_: *mut archive_md5_ctx) -> libc::c_int>,
    pub md5update: Option<
        unsafe extern "C" fn(
            _: *mut archive_md5_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub md5final:
        Option<unsafe extern "C" fn(_: *mut archive_md5_ctx, _: *mut libc::c_void) -> libc::c_int>,
    pub rmd160init: Option<unsafe extern "C" fn(_: *mut archive_rmd160_ctx) -> libc::c_int>,
    pub rmd160update: Option<
        unsafe extern "C" fn(
            _: *mut archive_rmd160_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub rmd160final: Option<
        unsafe extern "C" fn(_: *mut archive_rmd160_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha1init: Option<unsafe extern "C" fn(_: *mut archive_sha1_ctx) -> libc::c_int>,
    pub sha1update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha1_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha1final:
        Option<unsafe extern "C" fn(_: *mut archive_sha1_ctx, _: *mut libc::c_void) -> libc::c_int>,
    pub sha256init: Option<unsafe extern "C" fn(_: *mut archive_sha256_ctx) -> libc::c_int>,
    pub sha256update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha256_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha256final: Option<
        unsafe extern "C" fn(_: *mut archive_sha256_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha384init: Option<unsafe extern "C" fn(_: *mut archive_sha384_ctx) -> libc::c_int>,
    pub sha384update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha384_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha384final: Option<
        unsafe extern "C" fn(_: *mut archive_sha384_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha512init: Option<unsafe extern "C" fn(_: *mut archive_sha512_ctx) -> libc::c_int>,
    pub sha512update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha512_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha512final: Option<
        unsafe extern "C" fn(_: *mut archive_sha512_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
}
pub type archive_sha512_ctx = *mut EVP_MD_CTX;
pub type archive_sha384_ctx = *mut EVP_MD_CTX;
pub type archive_sha256_ctx = *mut EVP_MD_CTX;
pub type archive_rmd160_ctx = *mut EVP_MD_CTX;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlattr_list {
    pub first: *mut xmlattr,
    pub last: *mut *mut xmlattr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlattr {
    pub next: *mut xmlattr,
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
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
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
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
pub const ARCHIVE_FORMAT_XAR: libc::c_int = 0xa0000 as libc::c_int;
/*-
 * Copyright (c) 2002 Thomas Moestl <tmm@FreeBSD.org>
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
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * $FreeBSD: head/lib/libarchive/archive_endian.h 201085 2009-12-28 02:17:15Z kientzle $
 *
 * Borrowed from FreeBSD's <sys/endian.h>
 */
/* Note:  This is a purely internal header! */
/* Do not use this outside of libarchive internal code! */
/*
 * Disabling inline keyword for compilers known to choke on it:
 * - Watcom C++ in C code.  (For any version?)
 * - SGI MIPSpro
 * - Microsoft Visual C++ 6.0 (supposedly newer versions too)
 * - IBM VisualAge 6 (XL v6)
 * - Sun WorkShop C (SunPro) before 5.9
 */
/* Alignment-agnostic encode/decode bytestream to/from little/big endian. */
#[inline]
unsafe extern "C" fn archive_be16dec(mut pp: *const libc::c_void) -> uint16_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return (p0 << 8 as libc::c_int | p1) as uint16_t;
}
#[inline]
unsafe extern "C" fn archive_be32dec(mut pp: *const libc::c_void) -> uint32_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p3: libc::c_uint = *p.offset(3 as libc::c_int as isize) as libc::c_uint;
    let mut p2: libc::c_uint = *p.offset(2 as libc::c_int as isize) as libc::c_uint;
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return p0 << 24 as libc::c_int | p1 << 16 as libc::c_int | p2 << 8 as libc::c_int | p3;
}
#[inline]
unsafe extern "C" fn archive_be64dec(mut pp: *const libc::c_void) -> uint64_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    return (archive_be32dec(p as *const libc::c_void) as uint64_t) << 32 as libc::c_int
        | archive_be32dec(p.offset(4 as libc::c_int as isize) as *const libc::c_void)
            as libc::c_ulong;
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
pub const AE_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
pub const archive_entry_copy_gname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_gname_l;
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
pub const archive_entry_copy_uname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *const libc::c_char,
    _: size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_copy_uname_l;
/* Support xar format */
/* #define DEBUG 1 */
/* #define DEBUG_PRINT_TOC 1 */
pub const HEADER_MAGIC: libc::c_int = 0x78617221 as libc::c_int;
pub const HEADER_SIZE: libc::c_int = 28 as libc::c_int;
pub const HEADER_VERSION: libc::c_int = 1 as libc::c_int;
pub const CKSUM_NONE: libc::c_int = 0 as libc::c_int;
pub const CKSUM_SHA1: libc::c_int = 1 as libc::c_int;
pub const CKSUM_MD5: libc::c_int = 2 as libc::c_int;
pub const MD5_SIZE: libc::c_int = 16 as libc::c_int;
pub const SHA1_SIZE: libc::c_int = 20 as libc::c_int;
pub const HAS_DATA: libc::c_int = 0x1 as libc::c_int;
pub const HAS_PATHNAME: libc::c_int = 0x2 as libc::c_int;
pub const HAS_SYMLINK: libc::c_int = 0x4 as libc::c_int;
pub const HAS_TIME: libc::c_int = 0x8 as libc::c_int;
pub const HAS_UID: libc::c_int = 0x10 as libc::c_int;
pub const HAS_GID: libc::c_int = 0x20 as libc::c_int;
pub const HAS_MODE: libc::c_int = 0x40 as libc::c_int;
pub const HAS_TYPE: libc::c_int = 0x80 as libc::c_int;
pub const HAS_DEV: libc::c_int = 0x100 as libc::c_int;
pub const HAS_DEVMAJOR: libc::c_int = 0x200 as libc::c_int;
pub const HAS_DEVMINOR: libc::c_int = 0x400 as libc::c_int;
pub const HAS_INO: libc::c_int = 0x800 as libc::c_int;
pub const HAS_FFLAGS: libc::c_int = 0x1000 as libc::c_int;
pub const HAS_XATTR: libc::c_int = 0x2000 as libc::c_int;
pub const HAS_ACL: libc::c_int = 0x4000 as libc::c_int;
pub const HAS_CTIME: libc::c_int = 0x8000 as libc::c_int;
pub const HAS_MTIME: libc::c_int = 0x10000 as libc::c_int;
pub const HAS_ATIME: libc::c_int = 0x20000 as libc::c_int;
pub const OUTBUFF_SIZE: libc::c_int = 1024 as libc::c_int * 64 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_format_xar(mut _a: *mut archive) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut a: *mut archive_read = _a as *mut archive_read;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_format_xar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    xar = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<xar>() as libc::c_ulong,
    ) as *mut xar;
    if xar.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate xar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* initialize xar->file_queue */
    (*xar).file_queue.allocated = 0 as libc::c_int;
    (*xar).file_queue.used = 0 as libc::c_int;
    (*xar).file_queue.files = NULL as *mut *mut xar_file;
    r = __archive_read_register_format(
        a,
        xar as *mut libc::c_void,
        b"xar\x00" as *const u8 as *const libc::c_char,
        Some(xar_bid as unsafe extern "C" fn(_: *mut archive_read, _: libc::c_int) -> libc::c_int),
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
            xar_read_header
                as unsafe extern "C" fn(_: *mut archive_read, _: *mut archive_entry) -> libc::c_int,
        ),
        Some(
            xar_read_data
                as unsafe extern "C" fn(
                    _: *mut archive_read,
                    _: *mut *const libc::c_void,
                    _: *mut size_t,
                    _: *mut int64_t,
                ) -> libc::c_int,
        ),
        Some(xar_read_data_skip as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
        ::std::mem::transmute::<
            libc::intptr_t,
            Option<
                unsafe extern "C" fn(_: *mut archive_read, _: int64_t, _: libc::c_int) -> int64_t,
            >,
        >(NULL as libc::intptr_t),
        Some(xar_cleanup as unsafe extern "C" fn(_: *mut archive_read) -> libc::c_int),
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
        free(xar as *mut libc::c_void);
    }
    return r;
}
unsafe extern "C" fn xar_bid(mut a: *mut archive_read, mut best_bid: libc::c_int) -> libc::c_int {
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut bid: libc::c_int = 0;
    /* UNUSED */
    b = __archive_read_ahead(a, HEADER_SIZE as size_t, NULL as *mut ssize_t)
        as *const libc::c_uchar;
    if b.is_null() {
        return -(1 as libc::c_int);
    }
    bid = 0 as libc::c_int;
    /*
     * Verify magic code
     */
    if archive_be32dec(b as *const libc::c_void) != HEADER_MAGIC as libc::c_uint {
        return 0 as libc::c_int;
    }
    bid += 32 as libc::c_int;
    /*
     * Verify header size
     */
    if archive_be16dec(b.offset(4 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        != HEADER_SIZE
    {
        return 0 as libc::c_int;
    }
    bid += 16 as libc::c_int;
    /*
     * Verify header version
     */
    if archive_be16dec(b.offset(6 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        != HEADER_VERSION
    {
        return 0 as libc::c_int;
    }
    bid += 16 as libc::c_int;
    /*
     * Verify type of checksum
     */
    match archive_be32dec(b.offset(24 as libc::c_int as isize) as *const libc::c_void) {
        0 | 1 | 2 => bid += 32 as libc::c_int,
        _ => return 0 as libc::c_int,
    }
    return bid;
}
unsafe extern "C" fn read_toc(mut a: *mut archive_read) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut file: *mut xar_file = 0 as *mut xar_file;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut toc_compressed_size: uint64_t = 0;
    let mut toc_uncompressed_size: uint64_t = 0;
    let mut toc_chksum_alg: uint32_t = 0;
    let mut bytes: ssize_t = 0;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    /*
     * Read xar header.
     */
    b = __archive_read_ahead(a, HEADER_SIZE as size_t, &mut bytes) as *const libc::c_uchar;
    if bytes < 0 as libc::c_int as libc::c_long {
        return bytes as libc::c_int;
    }
    if bytes < HEADER_SIZE as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Truncated archive header\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if archive_be32dec(b as *const libc::c_void) != HEADER_MAGIC as libc::c_uint {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Invalid header magic\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if archive_be16dec(b.offset(6 as libc::c_int as isize) as *const libc::c_void) as libc::c_int
        != HEADER_VERSION
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Unsupported header version(%d)\x00" as *const u8 as *const libc::c_char,
            archive_be16dec(b.offset(6 as libc::c_int as isize) as *const libc::c_void)
                as libc::c_int,
        );
        return -(30 as libc::c_int);
    }
    toc_compressed_size =
        archive_be64dec(b.offset(8 as libc::c_int as isize) as *const libc::c_void);
    (*xar).toc_remaining = toc_compressed_size;
    toc_uncompressed_size =
        archive_be64dec(b.offset(16 as libc::c_int as isize) as *const libc::c_void);
    toc_chksum_alg = archive_be32dec(b.offset(24 as libc::c_int as isize) as *const libc::c_void);
    __archive_read_consume(a, HEADER_SIZE as int64_t);
    (*xar).offset = ((*xar).offset as libc::c_ulong).wrapping_add(HEADER_SIZE as libc::c_ulong)
        as uint64_t as uint64_t;
    (*xar).toc_total = 0 as libc::c_int as uint64_t;
    /*
     * Read TOC(Table of Contents).
     */
    /* Initialize reading contents. */
    r = move_reading_point(a, HEADER_SIZE as uint64_t);
    if r != ARCHIVE_OK {
        return r;
    }
    r = rd_contents_init(a, GZIP, toc_chksum_alg as libc::c_int, CKSUM_NONE);
    if r != ARCHIVE_OK {
        return r;
    }
    r = xml2_read_toc(a);
    if r != ARCHIVE_OK {
        return r;
    }
    /* Set 'The HEAP' base. */
    (*xar).h_base = (*xar).offset;
    if (*xar).toc_total != toc_uncompressed_size {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"TOC uncompressed size error\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /*
     * Checksum TOC
     */
    if toc_chksum_alg != CKSUM_NONE as libc::c_uint {
        r = move_reading_point(a, (*xar).toc_chksum_offset);
        if r != ARCHIVE_OK {
            return r;
        }
        b = __archive_read_ahead(a, (*xar).toc_chksum_size, &mut bytes) as *const libc::c_uchar;
        if bytes < 0 as libc::c_int as libc::c_long {
            return bytes as libc::c_int;
        }
        if (bytes as uint64_t) < (*xar).toc_chksum_size {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Truncated archive file\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        r = checksum_final(
            a,
            b as *const libc::c_void,
            (*xar).toc_chksum_size,
            NULL as *const libc::c_void,
            0 as libc::c_int as size_t,
        );
        __archive_read_consume(a, (*xar).toc_chksum_size as int64_t);
        (*xar).offset = ((*xar).offset as libc::c_ulong).wrapping_add((*xar).toc_chksum_size)
            as uint64_t as uint64_t;
        if r != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Connect hardlinked files.
     */
    file = (*xar).hdlink_orgs;
    while !file.is_null() {
        let mut hdlink: *mut *mut hdlink = 0 as *mut *mut hdlink;
        hdlink = &mut (*xar).hdlink_list;
        while !(*hdlink).is_null() {
            if (**hdlink).id as libc::c_ulong == (*file).id {
                let mut hltmp: *mut hdlink = 0 as *mut hdlink;
                let mut f2: *mut xar_file = 0 as *mut xar_file;
                let mut nlink: libc::c_int = (**hdlink).cnt + 1 as libc::c_int;
                (*file).nlink = nlink as libc::c_uint;
                f2 = (**hdlink).files;
                while !f2.is_null() {
                    (*f2).nlink = nlink as libc::c_uint;
                    (*f2).hardlink.length = 0 as libc::c_int as size_t;
                    archive_string_concat(&mut (*f2).hardlink, &mut (*file).pathname);
                    f2 = (*f2).hdnext
                }
                /* Remove resolved files from hdlist_list. */
                hltmp = *hdlink;
                *hdlink = (*hltmp).next;
                free(hltmp as *mut libc::c_void);
                break;
            } else {
                hdlink = &mut (**hdlink).next
            }
        }
        file = (*file).hdnext
    }
    (*a).archive.archive_format = ARCHIVE_FORMAT_XAR;
    (*a).archive.archive_format_name = b"xar\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xar_read_header(
    mut a: *mut archive_read,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut file: *mut xar_file = 0 as *mut xar_file;
    let mut xattr: *mut xattr = 0 as *mut xattr;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    r = ARCHIVE_OK;
    if (*xar).offset == 0 as libc::c_int as libc::c_ulong {
        /* Create a character conversion object. */
        if (*xar).sconv.is_null() {
            (*xar).sconv = archive_string_conversion_from_charset(
                &mut (*a).archive,
                b"UTF-8\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*xar).sconv.is_null() {
                return -(30 as libc::c_int);
            }
        }
        /* Read TOC. */
        r = read_toc(a);
        if r != ARCHIVE_OK {
            return r;
        }
    }
    loop {
        (*xar).file = heap_get_entry(&mut (*xar).file_queue);
        file = (*xar).file;
        if file.is_null() {
            (*xar).end_of_file = 1 as libc::c_int;
            return 1 as libc::c_int;
        }
        if (*file).mode & AE_IFMT as mode_t != AE_IFDIR as mode_t {
            break;
        }
        if (*file).has != (HAS_PATHNAME | HAS_TYPE) as libc::c_uint {
            break;
        }
        /*
         * If a file type is a directory and it does not have
         * any metadata, do not export.
         */
        file_free(file);
    }
    if (*file).has & HAS_ATIME as libc::c_uint != 0 {
        archive_entry_set_atime(entry, (*file).atime, 0 as libc::c_int as libc::c_long);
    }
    if (*file).has & HAS_CTIME as libc::c_uint != 0 {
        archive_entry_set_ctime(entry, (*file).ctime, 0 as libc::c_int as libc::c_long);
    }
    if (*file).has & HAS_MTIME as libc::c_uint != 0 {
        archive_entry_set_mtime(entry, (*file).mtime, 0 as libc::c_int as libc::c_long);
    }
    archive_entry_set_gid(entry, (*file).gid);
    if (*file).gname.length > 0 as libc::c_int as libc::c_ulong
        && _archive_entry_copy_gname_l(entry, (*file).gname.s, (*file).gname.length, (*xar).sconv)
            != 0 as libc::c_int
    {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Gname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Gname cannot be converted from %s to current locale.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name((*xar).sconv),
        );
        r = ARCHIVE_WARN
    }
    archive_entry_set_uid(entry, (*file).uid);
    if (*file).uname.length > 0 as libc::c_int as libc::c_ulong
        && _archive_entry_copy_uname_l(entry, (*file).uname.s, (*file).uname.length, (*xar).sconv)
            != 0 as libc::c_int
    {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Uname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"Uname cannot be converted from %s to current locale.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name((*xar).sconv),
        );
        r = ARCHIVE_WARN
    }
    archive_entry_set_mode(entry, (*file).mode);
    if _archive_entry_copy_pathname_l(
        entry,
        (*file).pathname.s,
        (*file).pathname.length,
        (*xar).sconv,
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
            archive_string_conversion_charset_name((*xar).sconv),
        );
        r = ARCHIVE_WARN
    }
    if (*file).symlink.length > 0 as libc::c_int as libc::c_ulong
        && _archive_entry_copy_symlink_l(
            entry,
            (*file).symlink.s,
            (*file).symlink.length,
            (*xar).sconv,
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
            b"Linkname cannot be converted from %s to current locale.\x00" as *const u8
                as *const libc::c_char,
            archive_string_conversion_charset_name((*xar).sconv),
        );
        r = ARCHIVE_WARN
    }
    /* Set proper nlink. */
    if (*file).mode & AE_IFMT as mode_t == AE_IFDIR as mode_t {
        archive_entry_set_nlink(entry, ((*file).subdirs + 2 as libc::c_int) as libc::c_uint);
    } else {
        archive_entry_set_nlink(entry, (*file).nlink);
    }
    archive_entry_set_size(entry, (*file).size as la_int64_t);
    if (*file).hardlink.length > 0 as libc::c_int as libc::c_ulong {
        archive_entry_set_hardlink(entry, (*file).hardlink.s);
    }
    archive_entry_set_ino64(entry, (*file).ino64);
    if (*file).has & HAS_DEV as libc::c_uint != 0 {
        archive_entry_set_dev(entry, (*file).dev);
    }
    if (*file).has & HAS_DEVMAJOR as libc::c_uint != 0 {
        archive_entry_set_devmajor(entry, (*file).devmajor);
    }
    if (*file).has & HAS_DEVMINOR as libc::c_uint != 0 {
        archive_entry_set_devminor(entry, (*file).devminor);
    }
    if (*file).fflags_text.length > 0 as libc::c_int as libc::c_ulong {
        archive_entry_copy_fflags_text(entry, (*file).fflags_text.s);
    }
    (*xar).entry_init = 1 as libc::c_int;
    (*xar).entry_total = 0 as libc::c_int as uint64_t;
    (*xar).entry_remaining = (*file).length;
    (*xar).entry_size = (*file).size;
    (*xar).entry_encoding = (*file).encoding;
    (*xar).entry_a_sum = (*file).a_sum;
    (*xar).entry_e_sum = (*file).e_sum;
    /*
     * Read extended attributes.
     */
    xattr = (*file).xattr_list;
    while !xattr.is_null() {
        let mut d: *const libc::c_void = 0 as *const libc::c_void;
        let mut outbytes: size_t = 0 as libc::c_int as size_t;
        let mut used: size_t = 0 as libc::c_int as size_t;
        r = move_reading_point(a, (*xattr).offset);
        if r != ARCHIVE_OK {
            break;
        }
        r = rd_contents_init(a, (*xattr).encoding, (*xattr).a_sum.alg, (*xattr).e_sum.alg);
        if r != ARCHIVE_OK {
            break;
        }
        d = NULL as *const libc::c_void;
        r = rd_contents(a, &mut d, &mut outbytes, &mut used, (*xattr).length);
        if r != ARCHIVE_OK {
            break;
        }
        if outbytes != (*xattr).size {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Decompressed size error\x00" as *const u8 as *const libc::c_char,
            );
            r = ARCHIVE_FATAL;
            break;
        } else {
            r = checksum_final(
                a,
                (*xattr).a_sum.val.as_mut_ptr() as *const libc::c_void,
                (*xattr).a_sum.len,
                (*xattr).e_sum.val.as_mut_ptr() as *const libc::c_void,
                (*xattr).e_sum.len,
            );
            if r != ARCHIVE_OK {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Xattr checksum error\x00" as *const u8 as *const libc::c_char,
                );
                r = ARCHIVE_WARN;
                break;
            } else if (*xattr).name.s.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Xattr name error\x00" as *const u8 as *const libc::c_char,
                );
                r = ARCHIVE_WARN;
                break;
            } else {
                archive_entry_xattr_add_entry(entry, (*xattr).name.s, d, outbytes);
                xattr = (*xattr).next
            }
        }
    }
    if r != ARCHIVE_OK {
        file_free(file);
        return r;
    }
    if (*xar).entry_remaining > 0 as libc::c_int as libc::c_ulong {
        /* Move reading point to the beginning of current
         * file contents. */
        r = move_reading_point(a, (*file).offset)
    } else {
        r = ARCHIVE_OK
    }
    file_free(file);
    return r;
}
unsafe extern "C" fn xar_read_data(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut offset: *mut int64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut xar: *mut xar = 0 as *mut xar;
    let mut used: size_t = 0 as libc::c_int as size_t;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    if (*xar).entry_unconsumed != 0 {
        __archive_read_consume(a, (*xar).entry_unconsumed as int64_t);
        (*xar).entry_unconsumed = 0 as libc::c_int as size_t
    }
    if (*xar).end_of_file != 0 || (*xar).entry_remaining <= 0 as libc::c_int as libc::c_ulong {
        r = ARCHIVE_EOF
    } else {
        if (*xar).entry_init != 0 {
            r = rd_contents_init(
                a,
                (*xar).entry_encoding,
                (*xar).entry_a_sum.alg,
                (*xar).entry_e_sum.alg,
            );
            if r != ARCHIVE_OK {
                (*xar).entry_remaining = 0 as libc::c_int as uint64_t;
                return r;
            }
            (*xar).entry_init = 0 as libc::c_int
        }
        *buff = NULL as *const libc::c_void;
        r = rd_contents(a, buff, size, &mut used, (*xar).entry_remaining);
        if !(r != ARCHIVE_OK) {
            *offset = (*xar).entry_total as int64_t;
            (*xar).entry_total =
                ((*xar).entry_total as libc::c_ulong).wrapping_add(*size) as uint64_t as uint64_t;
            (*xar).total =
                ((*xar).total as libc::c_ulong).wrapping_add(*size) as int64_t as int64_t;
            (*xar).offset =
                ((*xar).offset as libc::c_ulong).wrapping_add(used) as uint64_t as uint64_t;
            (*xar).entry_remaining = ((*xar).entry_remaining as libc::c_ulong).wrapping_sub(used)
                as uint64_t as uint64_t;
            (*xar).entry_unconsumed = used;
            if (*xar).entry_remaining == 0 as libc::c_int as libc::c_ulong {
                if (*xar).entry_total != (*xar).entry_size {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Decompressed size error\x00" as *const u8 as *const libc::c_char,
                    );
                    r = ARCHIVE_FATAL;
                    current_block = 12115052188632565762;
                } else {
                    r = checksum_final(
                        a,
                        (*xar).entry_a_sum.val.as_mut_ptr() as *const libc::c_void,
                        (*xar).entry_a_sum.len,
                        (*xar).entry_e_sum.val.as_mut_ptr() as *const libc::c_void,
                        (*xar).entry_e_sum.len,
                    );
                    if r != ARCHIVE_OK {
                        current_block = 12115052188632565762;
                    } else {
                        current_block = 7205609094909031804;
                    }
                }
            } else {
                current_block = 7205609094909031804;
            }
            match current_block {
                12115052188632565762 => {}
                _ => return 0 as libc::c_int,
            }
        }
    }
    *buff = NULL as *const libc::c_void;
    *size = 0 as libc::c_int as size_t;
    *offset = (*xar).total;
    return r;
}
unsafe extern "C" fn xar_read_data_skip(mut a: *mut archive_read) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut bytes_skipped: int64_t = 0;
    xar = (*(*a).format).data as *mut xar;
    if (*xar).end_of_file != 0 {
        return 1 as libc::c_int;
    }
    bytes_skipped = __archive_read_consume(
        a,
        (*xar).entry_remaining.wrapping_add((*xar).entry_unconsumed) as int64_t,
    );
    if bytes_skipped < 0 as libc::c_int as libc::c_long {
        return -(30 as libc::c_int);
    }
    (*xar).offset = ((*xar).offset as libc::c_ulong).wrapping_add(bytes_skipped as libc::c_ulong)
        as uint64_t as uint64_t;
    (*xar).entry_unconsumed = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xar_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut hdlink: *mut hdlink = 0 as *mut hdlink;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    checksum_cleanup(a);
    r = decompression_cleanup(a);
    hdlink = (*xar).hdlink_list;
    while !hdlink.is_null() {
        let mut next: *mut hdlink = (*hdlink).next;
        free(hdlink as *mut libc::c_void);
        hdlink = next
    }
    i = 0 as libc::c_int;
    while i < (*xar).file_queue.used {
        file_free(*(*xar).file_queue.files.offset(i as isize));
        i += 1
    }
    free((*xar).file_queue.files as *mut libc::c_void);
    while !(*xar).unknowntags.is_null() {
        let mut tag: *mut unknown_tag = 0 as *mut unknown_tag;
        tag = (*xar).unknowntags;
        (*xar).unknowntags = (*tag).next;
        archive_string_free(&mut (*tag).name);
        free(tag as *mut libc::c_void);
    }
    free((*xar).outbuff as *mut libc::c_void);
    free(xar as *mut libc::c_void);
    (*(*a).format).data = NULL as *mut libc::c_void;
    return r;
}
unsafe extern "C" fn move_reading_point(
    mut a: *mut archive_read,
    mut offset: uint64_t,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    xar = (*(*a).format).data as *mut xar;
    if (*xar).offset.wrapping_sub((*xar).h_base) != offset {
        /* Seek forward to the start of file contents. */
        let mut step: int64_t = 0;
        step = offset.wrapping_sub((*xar).offset.wrapping_sub((*xar).h_base)) as int64_t;
        if step > 0 as libc::c_int as libc::c_long {
            step = __archive_read_consume(a, step);
            if step < 0 as libc::c_int as libc::c_long {
                return step as libc::c_int;
            }
            (*xar).offset = ((*xar).offset as libc::c_ulong).wrapping_add(step as libc::c_ulong)
                as uint64_t as uint64_t
        } else {
            let mut pos: int64_t =
                __archive_read_seek(a, (*xar).h_base.wrapping_add(offset) as int64_t, SEEK_SET);
            if pos == ARCHIVE_FAILED as libc::c_long {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Cannot seek.\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
            (*xar).offset = pos as uint64_t
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn rd_contents_init(
    mut a: *mut archive_read,
    mut encoding: enctype,
    mut a_sum_alg: libc::c_int,
    mut e_sum_alg: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    /* Init decompress library. */
    r = decompression_init(a, encoding);
    if r != ARCHIVE_OK {
        return r;
    }
    /* Init checksum library. */
    checksum_init(a, a_sum_alg, e_sum_alg);
    return 0 as libc::c_int;
}
unsafe extern "C" fn rd_contents(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut size: *mut size_t,
    mut used: *mut size_t,
    mut remaining: uint64_t,
) -> libc::c_int {
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut bytes: ssize_t = 0;
    /* Get whatever bytes are immediately available. */
    b = __archive_read_ahead(a, 1 as libc::c_int as size_t, &mut bytes) as *const libc::c_uchar;
    if bytes < 0 as libc::c_int as libc::c_long {
        return bytes as libc::c_int;
    }
    if bytes == 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Truncated archive file\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if bytes as uint64_t > remaining {
        bytes = remaining as ssize_t
    }
    /*
     * Decompress contents of file.
     */
    *used = bytes as size_t;
    if decompress(a, buff, size, b as *const libc::c_void, used) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    /*
     * Update checksum of a compressed data and a extracted data.
     */
    checksum_update(a, b as *const libc::c_void, *used, *buff, *size);
    return 0 as libc::c_int;
}
/*
 * Note that this implementation does not (and should not!) obey
 * locale settings; you cannot simply substitute strtol here, since
 * it does obey locale.
 */
unsafe extern "C" fn atol10(mut p: *const libc::c_char, mut char_cnt: size_t) -> uint64_t {
    let mut l: uint64_t = 0;
    let mut digit: libc::c_int = 0;
    if char_cnt == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as uint64_t;
    }
    l = 0 as libc::c_int as uint64_t;
    digit = *p as libc::c_int - '0' as i32;
    while digit >= 0 as libc::c_int && digit < 10 as libc::c_int && {
        let fresh0 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        (fresh0) > 0 as libc::c_int as libc::c_ulong
    } {
        l = l
            .wrapping_mul(10 as libc::c_int as libc::c_ulong)
            .wrapping_add(digit as libc::c_ulong);
        p = p.offset(1);
        digit = *p as libc::c_int - '0' as i32
    }
    return l;
}
unsafe extern "C" fn atol8(mut p: *const libc::c_char, mut char_cnt: size_t) -> int64_t {
    let mut l: int64_t = 0;
    let mut digit: libc::c_int = 0;
    if char_cnt == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as int64_t;
    }
    l = 0 as libc::c_int as int64_t;
    loop {
        let fresh1 = char_cnt;
        char_cnt = char_cnt.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if !(*p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '7' as i32) {
            break;
        }
        digit = *p as libc::c_int - '0' as i32;
        p = p.offset(1);
        l <<= 3 as libc::c_int;
        l |= digit as libc::c_long
    }
    return l;
}
unsafe extern "C" fn atohex(
    mut b: *mut libc::c_uchar,
    mut bsize: size_t,
    mut p: *const libc::c_char,
    mut psize: size_t,
) -> size_t {
    let mut fbsize: size_t = bsize;
    while bsize != 0 && psize > 1 as libc::c_int as libc::c_ulong {
        let mut x: libc::c_uchar = 0;
        if *p.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
            && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
        {
            x = ((*p.offset(0 as libc::c_int as isize) as libc::c_int - 'a' as i32
                + 0xa as libc::c_int)
                << 4 as libc::c_int) as libc::c_uchar
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
            && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
        {
            x = ((*p.offset(0 as libc::c_int as isize) as libc::c_int - 'A' as i32
                + 0xa as libc::c_int)
                << 4 as libc::c_int) as libc::c_uchar
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
            && *p.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
        {
            x = ((*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                << 4 as libc::c_int) as libc::c_uchar
        } else {
            return -(1 as libc::c_int) as size_t;
        }
        if *p.offset(1 as libc::c_int as isize) as libc::c_int >= 'a' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int <= 'z' as i32
        {
            x = (x as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int - 'a' as i32
                    + 0xa as libc::c_int) as libc::c_uchar
        } else if *p.offset(1 as libc::c_int as isize) as libc::c_int >= 'A' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int <= 'Z' as i32
        {
            x = (x as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int - 'A' as i32
                    + 0xa as libc::c_int) as libc::c_uchar
        } else if *p.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
        {
            x = (x as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32)
                as libc::c_uchar
        } else {
            return -(1 as libc::c_int) as size_t;
        }
        let fresh2 = b;
        b = b.offset(1);
        *fresh2 = x;
        bsize = bsize.wrapping_sub(1);
        p = p.offset(2 as libc::c_int as isize);
        psize = (psize as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    return fbsize.wrapping_sub(bsize);
}
unsafe extern "C" fn time_from_tm(mut t: *mut tm) -> time_t {
    /* Use platform timegm() if available. */
    return timegm(t);
}
unsafe extern "C" fn parse_time(mut p: *const libc::c_char, mut n: size_t) -> time_t {
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
    let mut t: time_t = 0 as libc::c_int as time_t;
    let mut data: int64_t = 0;
    memset(
        &mut tm as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tm>() as libc::c_ulong,
    );
    if n != 20 as libc::c_int as libc::c_ulong {
        return t;
    }
    data = atol10(p, 4 as libc::c_int as size_t) as int64_t;
    if data < 1900 as libc::c_int as libc::c_long {
        return t;
    }
    tm.tm_year = data as libc::c_int - 1900 as libc::c_int;
    p = p.offset(4 as libc::c_int as isize);
    let fresh3 = p;
    p = p.offset(1);
    if *fresh3 as libc::c_int != '-' as i32 {
        return t;
    }
    data = atol10(p, 2 as libc::c_int as size_t) as int64_t;
    if data < 1 as libc::c_int as libc::c_long || data > 12 as libc::c_int as libc::c_long {
        return t;
    }
    tm.tm_mon = data as libc::c_int - 1 as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let fresh4 = p;
    p = p.offset(1);
    if *fresh4 as libc::c_int != '-' as i32 {
        return t;
    }
    data = atol10(p, 2 as libc::c_int as size_t) as int64_t;
    if data < 1 as libc::c_int as libc::c_long || data > 31 as libc::c_int as libc::c_long {
        return t;
    }
    tm.tm_mday = data as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let fresh5 = p;
    p = p.offset(1);
    if *fresh5 as libc::c_int != 'T' as i32 {
        return t;
    }
    data = atol10(p, 2 as libc::c_int as size_t) as int64_t;
    if data < 0 as libc::c_int as libc::c_long || data > 23 as libc::c_int as libc::c_long {
        return t;
    }
    tm.tm_hour = data as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let fresh6 = p;
    p = p.offset(1);
    if *fresh6 as libc::c_int != ':' as i32 {
        return t;
    }
    data = atol10(p, 2 as libc::c_int as size_t) as int64_t;
    if data < 0 as libc::c_int as libc::c_long || data > 59 as libc::c_int as libc::c_long {
        return t;
    }
    tm.tm_min = data as libc::c_int;
    p = p.offset(2 as libc::c_int as isize);
    let fresh7 = p;
    p = p.offset(1);
    if *fresh7 as libc::c_int != ':' as i32 {
        return t;
    }
    data = atol10(p, 2 as libc::c_int as size_t) as int64_t;
    if data < 0 as libc::c_int as libc::c_long || data > 60 as libc::c_int as libc::c_long {
        return t;
    }
    tm.tm_sec = data as libc::c_int;
    t = time_from_tm(&mut tm);
    return t;
}
unsafe extern "C" fn heap_add_entry(
    mut a: *mut archive_read,
    mut heap: *mut heap_queue,
    mut file: *mut xar_file,
) -> libc::c_int {
    let mut file_id: uint64_t = 0;
    let mut parent_id: uint64_t = 0;
    let mut hole: libc::c_int = 0;
    let mut parent: libc::c_int = 0;
    /* Expand our pending files list as necessary. */
    if (*heap).used >= (*heap).allocated {
        let mut new_pending_files: *mut *mut xar_file = 0 as *mut *mut xar_file;
        let mut new_size: libc::c_int = 0;
        if (*heap).allocated < 1024 as libc::c_int {
            new_size = 1024 as libc::c_int
        } else {
            new_size = (*heap).allocated * 2 as libc::c_int
        }
        /* Overflow might keep us from growing the list. */
        if new_size <= (*heap).allocated {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        new_pending_files = malloc(
            (new_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut xar_file>() as libc::c_ulong),
        ) as *mut *mut xar_file;
        if new_pending_files.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        if (*heap).allocated != 0 {
            memcpy(
                new_pending_files as *mut libc::c_void,
                (*heap).files as *const libc::c_void,
                ((*heap).allocated as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut xar_file>() as libc::c_ulong),
            );
            free((*heap).files as *mut libc::c_void);
        }
        (*heap).files = new_pending_files;
        (*heap).allocated = new_size
    }
    file_id = (*file).id;
    /*
     * Start with hole at end, walk it up tree to find insertion point.
     */
    let fresh8 = (*heap).used;
    (*heap).used = (*heap).used + 1;
    hole = fresh8;
    while hole > 0 as libc::c_int {
        parent = (hole - 1 as libc::c_int) / 2 as libc::c_int;
        parent_id = (**(*heap).files.offset(parent as isize)).id;
        if file_id >= parent_id {
            let ref mut fresh9 = *(*heap).files.offset(hole as isize);
            *fresh9 = file;
            return 0 as libc::c_int;
        }
        /* Move parent into hole <==> move hole up tree. */
        let ref mut fresh10 = *(*heap).files.offset(hole as isize);
        *fresh10 = *(*heap).files.offset(parent as isize);
        hole = parent
    }
    let ref mut fresh11 = *(*heap).files.offset(0 as libc::c_int as isize);
    *fresh11 = file;
    return 0 as libc::c_int;
}
unsafe extern "C" fn heap_get_entry(mut heap: *mut heap_queue) -> *mut xar_file {
    let mut a_id: uint64_t = 0;
    let mut b_id: uint64_t = 0;
    let mut c_id: uint64_t = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut r: *mut xar_file = 0 as *mut xar_file;
    let mut tmp: *mut xar_file = 0 as *mut xar_file;
    if (*heap).used < 1 as libc::c_int {
        return 0 as *mut xar_file;
    }
    /*
     * The first file in the list is the earliest; we'll return this.
     */
    r = *(*heap).files.offset(0 as libc::c_int as isize);
    /*
     * Move the last item in the heap to the root of the tree
     */
    (*heap).used -= 1;
    let ref mut fresh12 = *(*heap).files.offset(0 as libc::c_int as isize);
    *fresh12 = *(*heap).files.offset((*heap).used as isize);
    /*
     * Rebalance the heap.
     */
    a = 0 as libc::c_int; /* Starting element and its heap key */
    a_id = (**(*heap).files.offset(a as isize)).id; /* First child */
    loop {
        b = a + a + 1 as libc::c_int; /* Use second child if it is smaller. */
        if b >= (*heap).used {
            return r;
        }
        b_id = (**(*heap).files.offset(b as isize)).id;
        c = b + 1 as libc::c_int;
        if c < (*heap).used {
            c_id = (**(*heap).files.offset(c as isize)).id;
            if c_id < b_id {
                b = c;
                b_id = c_id
            }
        }
        if a_id <= b_id {
            return r;
        }
        tmp = *(*heap).files.offset(a as isize);
        let ref mut fresh13 = *(*heap).files.offset(a as isize);
        *fresh13 = *(*heap).files.offset(b as isize);
        let ref mut fresh14 = *(*heap).files.offset(b as isize);
        *fresh14 = tmp;
        a = b
    }
}
unsafe extern "C" fn add_link(
    mut a: *mut archive_read,
    mut xar: *mut xar,
    mut file: *mut xar_file,
) -> libc::c_int {
    let mut hdlink: *mut hdlink = 0 as *mut hdlink;
    hdlink = (*xar).hdlink_list;
    while !hdlink.is_null() {
        if (*hdlink).id == (*file).link {
            (*file).hdnext = (*hdlink).files;
            (*hdlink).cnt += 1;
            (*hdlink).files = file;
            return 0 as libc::c_int;
        }
        hdlink = (*hdlink).next
    }
    hdlink = malloc(::std::mem::size_of::<hdlink>() as libc::c_ulong) as *mut hdlink;
    if hdlink.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*file).hdnext = NULL as *mut xar_file;
    (*hdlink).id = (*file).link;
    (*hdlink).cnt = 1 as libc::c_int;
    (*hdlink).files = file;
    (*hdlink).next = (*xar).hdlink_list;
    (*xar).hdlink_list = hdlink;
    return 0 as libc::c_int;
}
unsafe extern "C" fn _checksum_init(mut sumwrk: *mut chksumwork, mut sum_alg: libc::c_int) {
    (*sumwrk).alg = sum_alg;
    match sum_alg {
        CKSUM_SHA1 => {
            __archive_digest
                .sha1init
                .expect("non-null function pointer")(&mut (*sumwrk).sha1ctx);
        }
        CKSUM_MD5 => {
            __archive_digest.md5init.expect("non-null function pointer")(&mut (*sumwrk).md5ctx);
        }
        CKSUM_NONE | _ => {}
    };
}
unsafe extern "C" fn _checksum_update(
    mut sumwrk: *mut chksumwork,
    mut buff: *const libc::c_void,
    mut size: size_t,
) {
    match (*sumwrk).alg {
        CKSUM_SHA1 => {
            __archive_digest
                .sha1update
                .expect("non-null function pointer")(&mut (*sumwrk).sha1ctx, buff, size);
        }
        CKSUM_MD5 => {
            __archive_digest
                .md5update
                .expect("non-null function pointer")(&mut (*sumwrk).md5ctx, buff, size);
        }
        CKSUM_NONE | _ => {}
    };
}
unsafe extern "C" fn _checksum_final(
    mut sumwrk: *mut chksumwork,
    mut val: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut sum: [libc::c_uchar; 20] = [0; 20];
    let mut r: libc::c_int = ARCHIVE_OK;
    match (*sumwrk).alg {
        CKSUM_SHA1 => {
            __archive_digest
                .sha1final
                .expect("non-null function pointer")(
                &mut (*sumwrk).sha1ctx,
                sum.as_mut_ptr() as *mut libc::c_void,
            );
            if len != SHA1_SIZE as libc::c_ulong
                || memcmp(
                    val,
                    sum.as_mut_ptr() as *const libc::c_void,
                    SHA1_SIZE as libc::c_ulong,
                ) != 0 as libc::c_int
            {
                r = ARCHIVE_FAILED
            }
        }
        CKSUM_MD5 => {
            __archive_digest
                .md5final
                .expect("non-null function pointer")(
                &mut (*sumwrk).md5ctx,
                sum.as_mut_ptr() as *mut libc::c_void,
            );
            if len != MD5_SIZE as libc::c_ulong
                || memcmp(
                    val,
                    sum.as_mut_ptr() as *const libc::c_void,
                    MD5_SIZE as libc::c_ulong,
                ) != 0 as libc::c_int
            {
                r = ARCHIVE_FAILED
            }
        }
        CKSUM_NONE | _ => {}
    }
    return r;
}
unsafe extern "C" fn checksum_init(
    mut a: *mut archive_read,
    mut a_sum_alg: libc::c_int,
    mut e_sum_alg: libc::c_int,
) {
    let mut xar: *mut xar = 0 as *mut xar;
    xar = (*(*a).format).data as *mut xar;
    _checksum_init(&mut (*xar).a_sumwrk, a_sum_alg);
    _checksum_init(&mut (*xar).e_sumwrk, e_sum_alg);
}
unsafe extern "C" fn checksum_update(
    mut a: *mut archive_read,
    mut abuff: *const libc::c_void,
    mut asize: size_t,
    mut ebuff: *const libc::c_void,
    mut esize: size_t,
) {
    let mut xar: *mut xar = 0 as *mut xar;
    xar = (*(*a).format).data as *mut xar;
    _checksum_update(&mut (*xar).a_sumwrk, abuff, asize);
    _checksum_update(&mut (*xar).e_sumwrk, ebuff, esize);
}
unsafe extern "C" fn checksum_final(
    mut a: *mut archive_read,
    mut a_sum_val: *const libc::c_void,
    mut a_sum_len: size_t,
    mut e_sum_val: *const libc::c_void,
    mut e_sum_len: size_t,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    r = _checksum_final(&mut (*xar).a_sumwrk, a_sum_val, a_sum_len);
    if r == ARCHIVE_OK {
        r = _checksum_final(&mut (*xar).e_sumwrk, e_sum_val, e_sum_len)
    }
    if r != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Sumcheck error\x00" as *const u8 as *const libc::c_char,
        );
    }
    return r;
}
unsafe extern "C" fn decompression_init(
    mut a: *mut archive_read,
    mut encoding: enctype,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut detail: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    (*xar).rd_encoding = encoding;
    match encoding as libc::c_uint {
        0 => {}
        1 => {
            if (*xar).stream_valid != 0 {
                r = inflateReset(&mut (*xar).stream)
            } else {
                r = inflateInit_(
                    &mut (*xar).stream,
                    ZLIB_VERSION.as_ptr(),
                    ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
                )
            }
            if r != Z_OK {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"Couldn\'t initialize zlib stream.\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            (*xar).stream_valid = 1 as libc::c_int;
            (*xar).stream.total_in = 0 as libc::c_int as uLong;
            (*xar).stream.total_out = 0 as libc::c_int as uLong
        }
        2 | 3 | 4 | _ => {
            /*
             * Unsupported compression.
             */
            match (*xar).entry_encoding as libc::c_uint {
                2 => detail = b"bzip2\x00" as *const u8 as *const libc::c_char,
                3 => detail = b"lzma\x00" as *const u8 as *const libc::c_char,
                4 => detail = b"xz\x00" as *const u8 as *const libc::c_char,
                _ => detail = b"??\x00" as *const u8 as *const libc::c_char,
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"%s compression not supported on this platform\x00" as *const u8
                    as *const libc::c_char,
                detail,
            );
            return -(25 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn decompress(
    mut a: *mut archive_read,
    mut buff: *mut *const libc::c_void,
    mut outbytes: *mut size_t,
    mut b: *const libc::c_void,
    mut used: *mut size_t,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut outbuff: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut avail_in: size_t = 0;
    let mut avail_out: size_t = 0;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    avail_in = *used;
    outbuff = *buff as uintptr_t as *mut libc::c_void;
    if outbuff.is_null() {
        if (*xar).outbuff.is_null() {
            (*xar).outbuff = malloc(OUTBUFF_SIZE as libc::c_ulong) as *mut libc::c_uchar;
            if (*xar).outbuff.is_null() {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Couldn\'t allocate memory for out buffer\x00" as *const u8
                        as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        outbuff = (*xar).outbuff as *mut libc::c_void;
        *buff = outbuff;
        avail_out = OUTBUFF_SIZE as size_t
    } else {
        avail_out = *outbytes
    }
    match (*xar).rd_encoding as libc::c_uint {
        1 => {
            (*xar).stream.next_in = b as uintptr_t as *mut Bytef;
            (*xar).stream.avail_in = avail_in as uInt;
            (*xar).stream.next_out = outbuff as *mut libc::c_uchar;
            (*xar).stream.avail_out = avail_out as uInt;
            r = inflate(&mut (*xar).stream, 0 as libc::c_int);
            match r {
                Z_OK => {}
                Z_STREAM_END => {}
                _ => {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"File decompression failed (%d)\x00" as *const u8 as *const libc::c_char,
                        r,
                    );
                    return -(30 as libc::c_int);
                }
            }
            *used = avail_in.wrapping_sub((*xar).stream.avail_in as libc::c_ulong);
            *outbytes = avail_out.wrapping_sub((*xar).stream.avail_out as libc::c_ulong)
        }
        2 | 3 | 4 | 0 | _ => {
            if outbuff == (*xar).outbuff as *mut libc::c_void {
                *buff = b;
                *used = avail_in;
                *outbytes = avail_in
            } else {
                if avail_out > avail_in {
                    avail_out = avail_in
                }
                memcpy(outbuff, b, avail_out);
                *used = avail_out;
                *outbytes = avail_out
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn decompression_cleanup(mut a: *mut archive_read) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut r: libc::c_int = 0;
    xar = (*(*a).format).data as *mut xar;
    r = ARCHIVE_OK;
    if (*xar).stream_valid != 0 {
        if inflateEnd(&mut (*xar).stream) != Z_OK {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Failed to clean up zlib decompressor\x00" as *const u8 as *const libc::c_char,
            );
            r = ARCHIVE_FATAL
        }
    }
    return r;
}
unsafe extern "C" fn checksum_cleanup(mut a: *mut archive_read) {
    let mut xar: *mut xar = 0 as *mut xar;
    xar = (*(*a).format).data as *mut xar;
    _checksum_final(
        &mut (*xar).a_sumwrk,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
    _checksum_final(
        &mut (*xar).e_sumwrk,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn xmlattr_cleanup(mut list: *mut xmlattr_list) {
    let mut attr: *mut xmlattr = 0 as *mut xmlattr;
    let mut next: *mut xmlattr = 0 as *mut xmlattr;
    attr = (*list).first;
    while !attr.is_null() {
        next = (*attr).next;
        free((*attr).name as *mut libc::c_void);
        free((*attr).value as *mut libc::c_void);
        free(attr as *mut libc::c_void);
        attr = next
    }
    (*list).first = NULL as *mut xmlattr;
    (*list).last = &mut (*list).first;
}
unsafe extern "C" fn file_new(
    mut a: *mut archive_read,
    mut xar: *mut xar,
    mut list: *mut xmlattr_list,
) -> libc::c_int {
    let mut file: *mut xar_file = 0 as *mut xar_file;
    let mut attr: *mut xmlattr = 0 as *mut xmlattr;
    file = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<xar_file>() as libc::c_ulong,
    ) as *mut xar_file;
    if file.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*file).parent = (*xar).file;
    (*file).mode = 0o777 as libc::c_int as libc::c_uint | AE_IFREG as mode_t;
    (*file).atime = 0 as libc::c_int as time_t;
    (*file).mtime = 0 as libc::c_int as time_t;
    (*xar).file = file;
    (*xar).xattr = NULL as *mut xattr;
    attr = (*list).first;
    while !attr.is_null() {
        if strcmp((*attr).name, b"id\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*file).id = atol10((*attr).value, strlen((*attr).value))
        }
        attr = (*attr).next
    }
    (*file).nlink = 1 as libc::c_int as libc::c_uint;
    if heap_add_entry(a, &mut (*xar).file_queue, file) != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_free(mut file: *mut xar_file) {
    let mut xattr: *mut xattr = 0 as *mut xattr;
    archive_string_free(&mut (*file).pathname);
    archive_string_free(&mut (*file).symlink);
    archive_string_free(&mut (*file).uname);
    archive_string_free(&mut (*file).gname);
    archive_string_free(&mut (*file).hardlink);
    xattr = (*file).xattr_list;
    while !xattr.is_null() {
        let mut next: *mut xattr = 0 as *mut xattr;
        next = (*xattr).next;
        xattr_free(xattr);
        xattr = next
    }
    free(file as *mut libc::c_void);
}
unsafe extern "C" fn xattr_new(
    mut a: *mut archive_read,
    mut xar: *mut xar,
    mut list: *mut xmlattr_list,
) -> libc::c_int {
    let mut xattr: *mut xattr = 0 as *mut xattr;
    let mut nx: *mut *mut xattr = 0 as *mut *mut xattr;
    let mut attr: *mut xmlattr = 0 as *mut xmlattr;
    xattr = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<xattr>() as libc::c_ulong,
    ) as *mut xattr;
    if xattr.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*xar).xattr = xattr;
    attr = (*list).first;
    while !attr.is_null() {
        if strcmp((*attr).name, b"id\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            (*xattr).id = atol10((*attr).value, strlen((*attr).value))
        }
        attr = (*attr).next
    }
    /* Chain to xattr list. */
    nx = &mut (*(*xar).file).xattr_list;
    while !(*nx).is_null() {
        if (*xattr).id < (**nx).id {
            break;
        }
        nx = &mut (**nx).next
    }
    (*xattr).next = *nx;
    *nx = xattr;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xattr_free(mut xattr: *mut xattr) {
    archive_string_free(&mut (*xattr).name);
    free(xattr as *mut libc::c_void);
}
unsafe extern "C" fn getencoding(mut list: *mut xmlattr_list) -> libc::c_int {
    let mut attr: *mut xmlattr = 0 as *mut xmlattr;
    let mut encoding: enctype = NONE;
    attr = (*list).first;
    while !attr.is_null() {
        if strcmp(
            (*attr).name,
            b"style\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if strcmp(
                (*attr).value,
                b"application/octet-stream\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                encoding = NONE
            } else if strcmp(
                (*attr).value,
                b"application/x-gzip\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                encoding = GZIP
            } else if strcmp(
                (*attr).value,
                b"application/x-bzip2\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                encoding = BZIP2
            } else if strcmp(
                (*attr).value,
                b"application/x-lzma\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                encoding = LZMA
            } else if strcmp(
                (*attr).value,
                b"application/x-xz\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                encoding = XZ
            }
        }
        attr = (*attr).next
    }
    return encoding as libc::c_int;
}
unsafe extern "C" fn getsumalgorithm(mut list: *mut xmlattr_list) -> libc::c_int {
    let mut attr: *mut xmlattr = 0 as *mut xmlattr;
    let mut alg: libc::c_int = CKSUM_NONE;
    attr = (*list).first;
    while !attr.is_null() {
        if strcmp(
            (*attr).name,
            b"style\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let mut v: *const libc::c_char = (*attr).value;
            if (*v.offset(0 as libc::c_int as isize) as libc::c_int == 'S' as i32
                || *v.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32)
                && (*v.offset(1 as libc::c_int as isize) as libc::c_int == 'H' as i32
                    || *v.offset(1 as libc::c_int as isize) as libc::c_int == 'h' as i32)
                && (*v.offset(2 as libc::c_int as isize) as libc::c_int == 'A' as i32
                    || *v.offset(2 as libc::c_int as isize) as libc::c_int == 'a' as i32)
                && *v.offset(3 as libc::c_int as isize) as libc::c_int == '1' as i32
                && *v.offset(4 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                alg = CKSUM_SHA1
            }
            if (*v.offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
                || *v.offset(0 as libc::c_int as isize) as libc::c_int == 'm' as i32)
                && (*v.offset(1 as libc::c_int as isize) as libc::c_int == 'D' as i32
                    || *v.offset(1 as libc::c_int as isize) as libc::c_int == 'd' as i32)
                && *v.offset(2 as libc::c_int as isize) as libc::c_int == '5' as i32
                && *v.offset(3 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                alg = CKSUM_MD5
            }
        }
        attr = (*attr).next
    }
    return alg;
}
unsafe extern "C" fn unknowntag_start(
    mut a: *mut archive_read,
    mut xar: *mut xar,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut tag: *mut unknown_tag = 0 as *mut unknown_tag;
    tag = malloc(::std::mem::size_of::<unknown_tag>() as libc::c_ulong) as *mut unknown_tag;
    if tag.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*tag).next = (*xar).unknowntags;
    (*tag).name.s = NULL as *mut libc::c_char;
    (*tag).name.length = 0 as libc::c_int as size_t;
    (*tag).name.buffer_length = 0 as libc::c_int as size_t;
    (*tag).name.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*tag).name,
        name as *const libc::c_void,
        (if name.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(name)
        }),
    );
    if (*xar).unknowntags.is_null() {
        (*xar).xmlsts_unknown = (*xar).xmlsts;
        (*xar).xmlsts = UNKNOWN
    }
    (*xar).unknowntags = tag;
    return 0 as libc::c_int;
}
unsafe extern "C" fn unknowntag_end(mut xar: *mut xar, mut name: *const libc::c_char) {
    let mut tag: *mut unknown_tag = 0 as *mut unknown_tag;
    tag = (*xar).unknowntags;
    if tag.is_null() || name.is_null() {
        return;
    }
    if strcmp((*tag).name.s, name) == 0 as libc::c_int {
        (*xar).unknowntags = (*tag).next;
        archive_string_free(&mut (*tag).name);
        free(tag as *mut libc::c_void);
        if (*xar).unknowntags.is_null() {
            (*xar).xmlsts = (*xar).xmlsts_unknown
        }
    };
}
unsafe extern "C" fn xml_start(
    mut a: *mut archive_read,
    mut name: *const libc::c_char,
    mut list: *mut xmlattr_list,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut attr: *mut xmlattr = 0 as *mut xmlattr;
    xar = (*(*a).format).data as *mut xar;
    (*xar).base64text = 0 as libc::c_int;
    match (*xar).xmlsts as libc::c_uint {
        0 => {
            if strcmp(name, b"xar\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = XAR
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        1 => {
            if strcmp(name, b"toc\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        2 => {
            if strcmp(
                name,
                b"creation-time\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = TOC_CREATION_TIME
            } else if strcmp(name, b"checksum\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = TOC_CHECKSUM
            } else if strcmp(name, b"file\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if file_new(a, xar, list) != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
                (*xar).xmlsts = TOC_FILE
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        4 => {
            if strcmp(name, b"offset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_CHECKSUM_OFFSET
            } else if strcmp(name, b"size\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = TOC_CHECKSUM_SIZE
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        7 => {
            if strcmp(name, b"file\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                if file_new(a, xar, list) != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
            } else if strcmp(name, b"data\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA
            } else if strcmp(name, b"ea\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if xattr_new(a, xar, list) != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
                (*xar).xmlsts = FILE_EA
            } else if strcmp(name, b"ctime\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_CTIME
            } else if strcmp(name, b"mtime\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_MTIME
            } else if strcmp(name, b"atime\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_ATIME
            } else if strcmp(name, b"group\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_GROUP
            } else if strcmp(name, b"gid\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_GID
            } else if strcmp(name, b"user\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_USER
            } else if strcmp(name, b"uid\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_UID
            } else if strcmp(name, b"mode\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_MODE
            } else if strcmp(name, b"device\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DEVICE
            } else if strcmp(name, b"deviceno\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DEVICENO
            } else if strcmp(name, b"inode\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_INODE
            } else if strcmp(name, b"link\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_LINK
            } else if strcmp(name, b"type\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_TYPE;
                attr = (*list).first;
                while !attr.is_null() {
                    if !(strcmp(
                        (*attr).name,
                        b"link\x00" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int)
                    {
                        if strcmp(
                            (*attr).value,
                            b"original\x00" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            (*(*xar).file).hdnext = (*xar).hdlink_orgs;
                            (*xar).hdlink_orgs = (*xar).file
                        } else {
                            (*(*xar).file).link =
                                atol10((*attr).value, strlen((*attr).value)) as libc::c_uint;
                            if (*(*xar).file).link > 0 as libc::c_int as libc::c_uint {
                                if add_link(a, xar, (*xar).file) != ARCHIVE_OK {
                                    return -(30 as libc::c_int);
                                }
                            }
                        }
                    }
                    attr = (*attr).next
                }
            } else if strcmp(name, b"name\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_NAME;
                attr = (*list).first;
                while !attr.is_null() {
                    if strcmp(
                        (*attr).name,
                        b"enctype\x00" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                        && strcmp(
                            (*attr).value,
                            b"base64\x00" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                    {
                        (*xar).base64text = 1 as libc::c_int
                    }
                    attr = (*attr).next
                }
            } else if strcmp(name, b"acl\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_ACL
            } else if strcmp(name, b"flags\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            } else if strcmp(name, b"ext2\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        8 => {
            if strcmp(name, b"length\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_DATA_LENGTH
            } else if strcmp(name, b"offset\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA_OFFSET
            } else if strcmp(name, b"size\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA_SIZE
            } else if strcmp(name, b"encoding\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA_ENCODING;
                (*(*xar).file).encoding = getencoding(list) as enctype
            } else if strcmp(
                name,
                b"archived-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA_A_CHECKSUM;
                (*(*xar).file).a_sum.alg = getsumalgorithm(list)
            } else if strcmp(
                name,
                b"extracted-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA_E_CHECKSUM;
                (*(*xar).file).e_sum.alg = getsumalgorithm(list)
            } else if strcmp(name, b"content\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA_CONTENT
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        33 => {
            if strcmp(name, b"major\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_DEVICE_MAJOR
            } else if strcmp(name, b"minor\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DEVICE_MINOR
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        15 => {
            if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        16 => {
            if strcmp(name, b"length\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EA_LENGTH
            } else if strcmp(name, b"offset\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA_OFFSET
            } else if strcmp(name, b"size\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA_SIZE
            } else if strcmp(name, b"encoding\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA_ENCODING;
                (*(*xar).xattr).encoding = getencoding(list) as enctype
            } else if strcmp(
                name,
                b"archived-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA_A_CHECKSUM
            } else if strcmp(
                name,
                b"extracted-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA_E_CHECKSUM
            } else if strcmp(name, b"name\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA_NAME
            } else if strcmp(name, b"fstype\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA_FSTYPE
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        41 => {
            if strcmp(
                name,
                b"appleextended\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_ACL_APPLEEXTENDED
            } else if strcmp(name, b"default\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_ACL_DEFAULT
            } else if strcmp(name, b"access\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_ACL_ACCESS
            } else if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        45 => {
            if xml_parse_file_flags(xar, name) == 0 {
                if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
            }
        }
        56 => {
            if xml_parse_file_ext2(xar, name) == 0 {
                if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                    return -(30 as libc::c_int);
                }
            }
        }
        3 | 5 | 6 | 9 | 10 | 11 | 12 | 13 | 14 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25
        | 26 | 27 | 28 | 29 | 30 | 31 | 37 | 34 | 35 | 36 | 32 | 39 | 38 | 40 | 42 | 43 | 44
        | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 57 | 58 | 59 | 60 | 61 | 62 | 63
        | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 => {
            if unknowntag_start(a, xar, name) != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xml_end(mut userData: *mut libc::c_void, mut name: *const libc::c_char) {
    let mut a: *mut archive_read = 0 as *mut archive_read;
    let mut xar: *mut xar = 0 as *mut xar;
    a = userData as *mut archive_read;
    xar = (*(*a).format).data as *mut xar;
    match (*xar).xmlsts as libc::c_uint {
        1 => {
            if strcmp(name, b"xar\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = INIT
            }
        }
        2 => {
            if strcmp(name, b"toc\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = XAR
            }
        }
        3 => {
            if strcmp(
                name,
                b"creation-time\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = TOC
            }
        }
        4 => {
            if strcmp(name, b"checksum\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = TOC
            }
        }
        5 => {
            if strcmp(name, b"offset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_CHECKSUM
            }
        }
        6 => {
            if strcmp(name, b"size\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_CHECKSUM
            }
        }
        7 => {
            if strcmp(name, b"file\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                if !(*(*xar).file).parent.is_null()
                    && (*(*xar).file).mode & AE_IFMT as mode_t == AE_IFDIR as mode_t
                {
                    (*(*(*xar).file).parent).subdirs += 1
                }
                (*xar).file = (*(*xar).file).parent;
                if (*xar).file.is_null() {
                    (*xar).xmlsts = TOC
                }
            }
        }
        8 => {
            if strcmp(name, b"data\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        9 => {
            if strcmp(name, b"length\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_DATA
            }
        }
        10 => {
            if strcmp(name, b"offset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_DATA
            }
        }
        11 => {
            if strcmp(name, b"size\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_DATA
            }
        }
        12 => {
            if strcmp(name, b"encoding\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA
            }
        }
        13 => {
            if strcmp(
                name,
                b"archived-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA
            }
        }
        14 => {
            if strcmp(
                name,
                b"extracted-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA
            }
        }
        15 => {
            if strcmp(name, b"content\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_DATA
            }
        }
        16 => {
            if strcmp(name, b"ea\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE;
                (*xar).xattr = NULL as *mut xattr
            }
        }
        17 => {
            if strcmp(name, b"length\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EA
            }
        }
        18 => {
            if strcmp(name, b"offset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EA
            }
        }
        19 => {
            if strcmp(name, b"size\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EA
            }
        }
        20 => {
            if strcmp(name, b"encoding\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA
            }
        }
        21 => {
            if strcmp(
                name,
                b"archived-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA
            }
        }
        22 => {
            if strcmp(
                name,
                b"extracted-checksum\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EA
            }
        }
        23 => {
            if strcmp(name, b"name\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EA
            }
        }
        24 => {
            if strcmp(name, b"fstype\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EA
            }
        }
        25 => {
            if strcmp(name, b"ctime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        26 => {
            if strcmp(name, b"mtime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        27 => {
            if strcmp(name, b"atime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        28 => {
            if strcmp(name, b"group\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        29 => {
            if strcmp(name, b"gid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        30 => {
            if strcmp(name, b"user\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        31 => {
            if strcmp(name, b"uid\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        32 => {
            if strcmp(name, b"mode\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        33 => {
            if strcmp(name, b"device\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        34 => {
            if strcmp(name, b"major\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_DEVICE
            }
        }
        35 => {
            if strcmp(name, b"minor\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_DEVICE
            }
        }
        36 => {
            if strcmp(name, b"deviceno\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = TOC_FILE
            }
        }
        37 => {
            if strcmp(name, b"inode\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        38 => {
            if strcmp(name, b"link\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        39 => {
            if strcmp(name, b"type\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        40 => {
            if strcmp(name, b"name\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        41 => {
            if strcmp(name, b"acl\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        42 => {
            if strcmp(name, b"default\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_ACL
            }
        }
        43 => {
            if strcmp(name, b"access\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_ACL
            }
        }
        44 => {
            if strcmp(
                name,
                b"appleextended\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_ACL
            }
        }
        45 => {
            if strcmp(name, b"flags\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        46 => {
            if strcmp(name, b"UserNoDump\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        47 => {
            if strcmp(
                name,
                b"UserImmutable\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        48 => {
            if strcmp(name, b"UserAppend\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        49 => {
            if strcmp(name, b"UserOpaque\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        50 => {
            if strcmp(
                name,
                b"UserNoUnlink\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        51 => {
            if strcmp(
                name,
                b"SystemArchived\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        52 => {
            if strcmp(
                name,
                b"SystemImmutable\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        53 => {
            if strcmp(
                name,
                b"SystemAppend\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        54 => {
            if strcmp(
                name,
                b"SystemNoUnlink\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        55 => {
            if strcmp(
                name,
                b"SystemSnapshot\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_FLAGS
            }
        }
        56 => {
            if strcmp(name, b"ext2\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = TOC_FILE
            }
        }
        57 => {
            if strcmp(
                name,
                b"SecureDeletion\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        58 => {
            if strcmp(name, b"Undelete\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        59 => {
            if strcmp(name, b"Compress\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        60 => {
            if strcmp(name, b"Synchronous\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        61 => {
            if strcmp(name, b"Immutable\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        62 => {
            if strcmp(name, b"AppendOnly\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        63 => {
            if strcmp(name, b"NoDump\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        64 => {
            if strcmp(name, b"NoAtime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        65 => {
            if strcmp(name, b"CompDirty\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        66 => {
            if strcmp(name, b"CompBlock\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        67 => {
            if strcmp(name, b"NoCompBlock\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        68 => {
            if strcmp(name, b"CompError\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        69 => {
            if strcmp(name, b"BTree\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        70 => {
            if strcmp(name, b"HashIndexed\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        71 => {
            if strcmp(name, b"iMagic\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        72 => {
            if strcmp(name, b"Journaled\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        73 => {
            if strcmp(name, b"NoTail\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        74 => {
            if strcmp(name, b"DirSync\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        75 => {
            if strcmp(name, b"TopDir\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        76 => {
            if strcmp(name, b"Reserved\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*xar).xmlsts = FILE_EXT2
            }
        }
        77 => {
            unknowntag_end(xar, name);
        }
        0 | _ => {}
    };
}
static mut base64: [libc::c_int; 256] = [
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    62 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    63 as libc::c_int,
    52 as libc::c_int,
    53 as libc::c_int,
    54 as libc::c_int,
    55 as libc::c_int,
    56 as libc::c_int,
    57 as libc::c_int,
    58 as libc::c_int,
    59 as libc::c_int,
    60 as libc::c_int,
    61 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    15 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
    20 as libc::c_int,
    21 as libc::c_int,
    22 as libc::c_int,
    23 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    26 as libc::c_int,
    27 as libc::c_int,
    28 as libc::c_int,
    29 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    32 as libc::c_int,
    33 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    38 as libc::c_int,
    39 as libc::c_int,
    40 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    44 as libc::c_int,
    45 as libc::c_int,
    46 as libc::c_int,
    47 as libc::c_int,
    48 as libc::c_int,
    49 as libc::c_int,
    50 as libc::c_int,
    51 as libc::c_int,
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
];
unsafe extern "C" fn strappend_base64(
    mut xar: *mut xar,
    mut as_0: *mut archive_string,
    mut s: *const libc::c_char,
    mut l: size_t,
) {
    let mut buff: [libc::c_uchar; 256] = [0; 256];
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut b: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut len: size_t = 0;
    /* UNUSED */
    len = 0 as libc::c_int as size_t;
    out = buff.as_mut_ptr();
    b = s as *const libc::c_uchar;
    while l > 0 as libc::c_int as libc::c_ulong {
        let mut n: libc::c_int = 0 as libc::c_int;
        if base64[*b.offset(0 as libc::c_int as isize) as usize] < 0 as libc::c_int
            || base64[*b.offset(1 as libc::c_int as isize) as usize] < 0 as libc::c_int
        {
            break;
        }
        let fresh15 = b;
        b = b.offset(1);
        n = base64[*fresh15 as usize] << 18 as libc::c_int;
        let fresh16 = b;
        b = b.offset(1);
        n |= base64[*fresh16 as usize] << 12 as libc::c_int;
        let fresh17 = out;
        out = out.offset(1);
        *fresh17 = (n >> 16 as libc::c_int) as libc::c_uchar;
        len = len.wrapping_add(1);
        l = (l as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        if l > 0 as libc::c_int as libc::c_ulong {
            if base64[*b as usize] < 0 as libc::c_int {
                break;
            }
            let fresh18 = b;
            b = b.offset(1);
            n |= base64[*fresh18 as usize] << 6 as libc::c_int;
            let fresh19 = out;
            out = out.offset(1);
            *fresh19 = (n >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
            len = len.wrapping_add(1);
            l = l.wrapping_sub(1)
        }
        if l > 0 as libc::c_int as libc::c_ulong {
            if base64[*b as usize] < 0 as libc::c_int {
                break;
            }
            let fresh20 = b;
            b = b.offset(1);
            n |= base64[*fresh20 as usize];
            let fresh21 = out;
            out = out.offset(1);
            *fresh21 = (n & 0xff as libc::c_int) as libc::c_uchar;
            len = len.wrapping_add(1);
            l = l.wrapping_sub(1)
        }
        if len.wrapping_add(3 as libc::c_int as libc::c_ulong)
            >= ::std::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong
        {
            archive_strncat(
                as_0,
                buff.as_mut_ptr() as *const libc::c_char as *const libc::c_void,
                len,
            );
            len = 0 as libc::c_int as size_t;
            out = buff.as_mut_ptr()
        }
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        archive_strncat(
            as_0,
            buff.as_mut_ptr() as *const libc::c_char as *const libc::c_void,
            len,
        );
    };
}
unsafe extern "C" fn is_string(
    mut known: *const libc::c_char,
    mut data: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if strlen(known) != len {
        return -(1 as libc::c_int);
    }
    return memcmp(
        data as *const libc::c_void,
        known as *const libc::c_void,
        len,
    );
}
unsafe extern "C" fn xml_data(
    mut userData: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut len: libc::c_int,
) {
    let mut a: *mut archive_read = 0 as *mut archive_read;
    let mut xar: *mut xar = 0 as *mut xar;
    a = userData as *mut archive_read;
    xar = (*(*a).format).data as *mut xar;
    match (*xar).xmlsts as libc::c_uint {
        5 => (*xar).toc_chksum_offset = atol10(s, len as size_t),
        6 => (*xar).toc_chksum_size = atol10(s, len as size_t),
        _ => {}
    }
    if (*xar).file.is_null() {
        return;
    }
    match (*xar).xmlsts as libc::c_uint {
        40 => {
            if !(*(*xar).file).parent.is_null() {
                archive_string_concat(
                    &mut (*(*xar).file).pathname,
                    &mut (*(*(*xar).file).parent).pathname,
                );
                archive_strappend_char(&mut (*(*xar).file).pathname, '/' as i32 as libc::c_char);
            }
            (*(*xar).file).has |= HAS_PATHNAME as libc::c_uint;
            if (*xar).base64text != 0 {
                strappend_base64(xar, &mut (*(*xar).file).pathname, s, len as size_t);
            } else {
                archive_strncat(
                    &mut (*(*xar).file).pathname,
                    s as *const libc::c_void,
                    len as size_t,
                );
            }
        }
        38 => {
            (*(*xar).file).has |= HAS_SYMLINK as libc::c_uint;
            (*(*xar).file).symlink.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut (*(*xar).file).symlink,
                s as *const libc::c_void,
                len as size_t,
            );
        }
        39 => {
            if is_string(
                b"file\x00" as *const u8 as *const libc::c_char,
                s,
                len as size_t,
            ) == 0 as libc::c_int
                || is_string(
                    b"hardlink\x00" as *const u8 as *const libc::c_char,
                    s,
                    len as size_t,
                ) == 0 as libc::c_int
            {
                (*(*xar).file).mode =
                    (*(*xar).file).mode & !(AE_IFMT as mode_t) | AE_IFREG as mode_t
            }
            if is_string(
                b"directory\x00" as *const u8 as *const libc::c_char,
                s,
                len as size_t,
            ) == 0 as libc::c_int
            {
                (*(*xar).file).mode =
                    (*(*xar).file).mode & !(AE_IFMT as mode_t) | AE_IFDIR as mode_t
            }
            if is_string(
                b"symlink\x00" as *const u8 as *const libc::c_char,
                s,
                len as size_t,
            ) == 0 as libc::c_int
            {
                (*(*xar).file).mode =
                    (*(*xar).file).mode & !(AE_IFMT as mode_t) | AE_IFLNK as mode_t
            }
            if is_string(
                b"character special\x00" as *const u8 as *const libc::c_char,
                s,
                len as size_t,
            ) == 0 as libc::c_int
            {
                (*(*xar).file).mode =
                    (*(*xar).file).mode & !(AE_IFMT as mode_t) | AE_IFCHR as mode_t
            }
            if is_string(
                b"block special\x00" as *const u8 as *const libc::c_char,
                s,
                len as size_t,
            ) == 0 as libc::c_int
            {
                (*(*xar).file).mode =
                    (*(*xar).file).mode & !(AE_IFMT as mode_t) | AE_IFBLK as mode_t
            }
            if is_string(
                b"socket\x00" as *const u8 as *const libc::c_char,
                s,
                len as size_t,
            ) == 0 as libc::c_int
            {
                (*(*xar).file).mode =
                    (*(*xar).file).mode & !(AE_IFMT as mode_t) | AE_IFSOCK as mode_t
            }
            if is_string(
                b"fifo\x00" as *const u8 as *const libc::c_char,
                s,
                len as size_t,
            ) == 0 as libc::c_int
            {
                (*(*xar).file).mode =
                    (*(*xar).file).mode & !(AE_IFMT as mode_t) | AE_IFIFO as mode_t
            }
            (*(*xar).file).has |= HAS_TYPE as libc::c_uint
        }
        37 => {
            (*(*xar).file).has |= HAS_INO as libc::c_uint;
            (*(*xar).file).ino64 = atol10(s, len as size_t) as int64_t
        }
        34 => {
            (*(*xar).file).has |= HAS_DEVMAJOR as libc::c_uint;
            (*(*xar).file).devmajor = atol10(s, len as size_t)
        }
        35 => {
            (*(*xar).file).has |= HAS_DEVMINOR as libc::c_uint;
            (*(*xar).file).devminor = atol10(s, len as size_t)
        }
        36 => {
            (*(*xar).file).has |= HAS_DEV as libc::c_uint;
            (*(*xar).file).dev = atol10(s, len as size_t)
        }
        32 => {
            (*(*xar).file).has |= HAS_MODE as libc::c_uint;
            (*(*xar).file).mode = (*(*xar).file).mode & AE_IFMT as mode_t
                | atol8(s, len as size_t) as mode_t & !(AE_IFMT as mode_t)
        }
        28 => {
            (*(*xar).file).has |= HAS_GID as libc::c_uint;
            (*(*xar).file).gname.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut (*(*xar).file).gname,
                s as *const libc::c_void,
                len as size_t,
            );
        }
        29 => {
            (*(*xar).file).has |= HAS_GID as libc::c_uint;
            (*(*xar).file).gid = atol10(s, len as size_t) as int64_t
        }
        30 => {
            (*(*xar).file).has |= HAS_UID as libc::c_uint;
            (*(*xar).file).uname.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut (*(*xar).file).uname,
                s as *const libc::c_void,
                len as size_t,
            );
        }
        31 => {
            (*(*xar).file).has |= HAS_UID as libc::c_uint;
            (*(*xar).file).uid = atol10(s, len as size_t) as int64_t
        }
        25 => {
            (*(*xar).file).has |= (HAS_TIME | HAS_CTIME) as libc::c_uint;
            (*(*xar).file).ctime = parse_time(s, len as size_t)
        }
        26 => {
            (*(*xar).file).has |= (HAS_TIME | HAS_MTIME) as libc::c_uint;
            (*(*xar).file).mtime = parse_time(s, len as size_t)
        }
        27 => {
            (*(*xar).file).has |= (HAS_TIME | HAS_ATIME) as libc::c_uint;
            (*(*xar).file).atime = parse_time(s, len as size_t)
        }
        9 => {
            (*(*xar).file).has |= HAS_DATA as libc::c_uint;
            (*(*xar).file).length = atol10(s, len as size_t)
        }
        10 => {
            (*(*xar).file).has |= HAS_DATA as libc::c_uint;
            (*(*xar).file).offset = atol10(s, len as size_t)
        }
        11 => {
            (*(*xar).file).has |= HAS_DATA as libc::c_uint;
            (*(*xar).file).size = atol10(s, len as size_t)
        }
        13 => {
            (*(*xar).file).a_sum.len = atohex(
                (*(*xar).file).a_sum.val.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
                s,
                len as size_t,
            )
        }
        14 => {
            (*(*xar).file).e_sum.len = atohex(
                (*(*xar).file).e_sum.val.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
                s,
                len as size_t,
            )
        }
        17 => {
            (*(*xar).file).has |= HAS_XATTR as libc::c_uint;
            (*(*xar).xattr).length = atol10(s, len as size_t)
        }
        18 => {
            (*(*xar).file).has |= HAS_XATTR as libc::c_uint;
            (*(*xar).xattr).offset = atol10(s, len as size_t)
        }
        19 => {
            (*(*xar).file).has |= HAS_XATTR as libc::c_uint;
            (*(*xar).xattr).size = atol10(s, len as size_t)
        }
        21 => {
            (*(*xar).file).has |= HAS_XATTR as libc::c_uint;
            (*(*xar).xattr).a_sum.len = atohex(
                (*(*xar).xattr).a_sum.val.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
                s,
                len as size_t,
            )
        }
        22 => {
            (*(*xar).file).has |= HAS_XATTR as libc::c_uint;
            (*(*xar).xattr).e_sum.len = atohex(
                (*(*xar).xattr).e_sum.val.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
                s,
                len as size_t,
            )
        }
        23 => {
            (*(*xar).file).has |= HAS_XATTR as libc::c_uint;
            (*(*xar).xattr).name.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut (*(*xar).xattr).name,
                s as *const libc::c_void,
                len as size_t,
            );
        }
        24 => {
            (*(*xar).file).has |= HAS_XATTR as libc::c_uint;
            (*(*xar).xattr).fstype.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut (*(*xar).xattr).fstype,
                s as *const libc::c_void,
                len as size_t,
            );
        }
        42 | 43 | 44 => (*(*xar).file).has |= HAS_ACL as libc::c_uint,
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 12 | 15 | 33 | 16 | 20 | 41 | 45 | 46 | 47 | 48
        | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65
        | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | _ => {}
    };
}
/*
 * BSD file flags.
 */
unsafe extern "C" fn xml_parse_file_flags(
    mut xar: *mut xar,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut flag: *const libc::c_char = NULL as *const libc::c_char;
    if strcmp(name, b"UserNoDump\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_FLAGS_USER_NODUMP;
        flag = b"nodump\x00" as *const u8 as *const libc::c_char
    } else if strcmp(
        name,
        b"UserImmutable\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_USER_IMMUTABLE;
        flag = b"uimmutable\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"UserAppend\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_USER_APPEND;
        flag = b"uappend\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"UserOpaque\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_USER_OPAQUE;
        flag = b"opaque\x00" as *const u8 as *const libc::c_char
    } else if strcmp(
        name,
        b"UserNoUnlink\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_USER_NOUNLINK;
        flag = b"nouunlink\x00" as *const u8 as *const libc::c_char
    } else if strcmp(
        name,
        b"SystemArchived\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_SYS_ARCHIVED;
        flag = b"archived\x00" as *const u8 as *const libc::c_char
    } else if strcmp(
        name,
        b"SystemImmutable\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_SYS_IMMUTABLE;
        flag = b"simmutable\x00" as *const u8 as *const libc::c_char
    } else if strcmp(
        name,
        b"SystemAppend\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_SYS_APPEND;
        flag = b"sappend\x00" as *const u8 as *const libc::c_char
    } else if strcmp(
        name,
        b"SystemNoUnlink\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_SYS_NOUNLINK;
        flag = b"nosunlink\x00" as *const u8 as *const libc::c_char
    } else if strcmp(
        name,
        b"SystemSnapshot\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_FLAGS_SYS_SNAPSHOT;
        flag = b"snapshot\x00" as *const u8 as *const libc::c_char
    }
    if flag.is_null() {
        return 0 as libc::c_int;
    }
    (*(*xar).file).has |= HAS_FFLAGS as libc::c_uint;
    if (*(*xar).file).fflags_text.length > 0 as libc::c_int as libc::c_ulong {
        archive_strappend_char(&mut (*(*xar).file).fflags_text, ',' as i32 as libc::c_char);
    }
    archive_strcat(&mut (*(*xar).file).fflags_text, flag as *const libc::c_void);
    return 1 as libc::c_int;
}
/*
 * Linux file flags.
 */
unsafe extern "C" fn xml_parse_file_ext2(
    mut xar: *mut xar,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut flag: *const libc::c_char = NULL as *const libc::c_char;
    if strcmp(
        name,
        b"SecureDeletion\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_SecureDeletion;
        flag = b"securedeletion\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"Undelete\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_Undelete;
        flag = b"nouunlink\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"Compress\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_Compress;
        flag = b"compress\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"Synchronous\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_Synchronous;
        flag = b"sync\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"Immutable\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_Immutable;
        flag = b"simmutable\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"AppendOnly\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_AppendOnly;
        flag = b"sappend\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"NoDump\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_EXT2_NoDump;
        flag = b"nodump\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"NoAtime\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_EXT2_NoAtime;
        flag = b"noatime\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"CompDirty\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_CompDirty;
        flag = b"compdirty\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"CompBlock\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_CompBlock;
        flag = b"comprblk\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"NoCompBlock\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_NoCompBlock;
        flag = b"nocomprblk\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"CompError\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_CompError;
        flag = b"comperr\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"BTree\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_EXT2_BTree;
        flag = b"btree\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"HashIndexed\x00" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_HashIndexed;
        flag = b"hashidx\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"iMagic\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_EXT2_iMagic;
        flag = b"imagic\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"Journaled\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_Journaled;
        flag = b"journal\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"NoTail\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_EXT2_NoTail;
        flag = b"notail\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"DirSync\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_EXT2_DirSync;
        flag = b"dirsync\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"TopDir\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*xar).xmlsts = FILE_EXT2_TopDir;
        flag = b"topdir\x00" as *const u8 as *const libc::c_char
    } else if strcmp(name, b"Reserved\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        (*xar).xmlsts = FILE_EXT2_Reserved;
        flag = b"reserved\x00" as *const u8 as *const libc::c_char
    }
    if flag.is_null() {
        return 0 as libc::c_int;
    }
    if (*(*xar).file).fflags_text.length > 0 as libc::c_int as libc::c_ulong {
        archive_strappend_char(&mut (*(*xar).file).fflags_text, ',' as i32 as libc::c_char);
    }
    archive_strcat(&mut (*(*xar).file).fflags_text, flag as *const libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn xml2_xmlattr_setup(
    mut a: *mut archive_read,
    mut list: *mut xmlattr_list,
    mut reader: xmlTextReaderPtr,
) -> libc::c_int {
    let mut attr: *mut xmlattr = 0 as *mut xmlattr;
    let mut r: libc::c_int = 0;
    (*list).first = NULL as *mut xmlattr;
    (*list).last = &mut (*list).first;
    r = xmlTextReaderMoveToFirstAttribute(reader);
    while r == 1 as libc::c_int {
        attr = malloc(::std::mem::size_of::<xmlattr>() as libc::c_ulong) as *mut xmlattr;
        if attr.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*attr).name = strdup(xmlTextReaderConstLocalName(reader) as *const libc::c_char);
        if (*attr).name.is_null() {
            free(attr as *mut libc::c_void);
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*attr).value = strdup(xmlTextReaderConstValue(reader) as *const libc::c_char);
        if (*attr).value.is_null() {
            free((*attr).name as *mut libc::c_void);
            free(attr as *mut libc::c_void);
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*attr).next = NULL as *mut xmlattr;
        *(*list).last = attr;
        (*list).last = &mut (*attr).next;
        r = xmlTextReaderMoveToNextAttribute(reader)
    }
    return r;
}
unsafe extern "C" fn xml2_read_cb(
    mut context: *mut libc::c_void,
    mut buffer: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut a: *mut archive_read = 0 as *mut archive_read;
    let mut xar: *mut xar = 0 as *mut xar;
    let mut d: *const libc::c_void = 0 as *const libc::c_void;
    let mut outbytes: size_t = 0;
    let mut used: size_t = 0 as libc::c_int as size_t;
    let mut r: libc::c_int = 0;
    a = context as *mut archive_read;
    xar = (*(*a).format).data as *mut xar;
    if (*xar).toc_remaining <= 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    d = buffer as *const libc::c_void;
    outbytes = len as size_t;
    r = rd_contents(a, &mut d, &mut outbytes, &mut used, (*xar).toc_remaining);
    if r != ARCHIVE_OK {
        return r;
    }
    __archive_read_consume(a, used as int64_t);
    (*xar).toc_remaining =
        ((*xar).toc_remaining as libc::c_ulong).wrapping_sub(used) as uint64_t as uint64_t;
    (*xar).offset = ((*xar).offset as libc::c_ulong).wrapping_add(used) as uint64_t as uint64_t;
    (*xar).toc_total =
        ((*xar).toc_total as libc::c_ulong).wrapping_add(outbytes) as uint64_t as uint64_t;
    return outbytes as libc::c_int;
}
unsafe extern "C" fn xml2_close_cb(mut context: *mut libc::c_void) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
unsafe extern "C" fn xml2_error_hdr(
    mut arg: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut severity: xmlParserSeverities,
    mut locator: xmlTextReaderLocatorPtr,
) {
    let mut a: *mut archive_read = 0 as *mut archive_read;
    /* UNUSED */
    a = arg as *mut archive_read;
    match severity as libc::c_uint {
        1 | 3 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"XML Parsing error: %s\x00" as *const u8 as *const libc::c_char,
                msg,
            );
        }
        2 | 4 => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"XML Parsing error: %s\x00" as *const u8 as *const libc::c_char,
                msg,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn xml2_read_toc(mut a: *mut archive_read) -> libc::c_int {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut list: xmlattr_list = xmlattr_list {
        first: 0 as *mut xmlattr,
        last: 0 as *mut *mut xmlattr,
    };
    let mut r: libc::c_int = 0;
    reader = xmlReaderForIO(
        Some(
            xml2_read_cb
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_char,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
        Some(xml2_close_cb as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int),
        a as *mut libc::c_void,
        NULL as *const libc::c_char,
        NULL as *const libc::c_char,
        0 as libc::c_int,
    );
    if reader.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Couldn\'t allocate memory for xml parser\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    xmlTextReaderSetErrorHandler(
        reader,
        Some(
            xml2_error_hdr
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *const libc::c_char,
                    _: xmlParserSeverities,
                    _: xmlTextReaderLocatorPtr,
                ) -> (),
        ),
        a as *mut libc::c_void,
    );
    loop {
        r = xmlTextReaderRead(reader);
        if !(r == 1 as libc::c_int) {
            break;
        }
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *const libc::c_char = 0 as *const libc::c_char;
        let mut type_0: libc::c_int = 0;
        let mut empty: libc::c_int = 0;
        type_0 = xmlTextReaderNodeType(reader);
        name = xmlTextReaderConstLocalName(reader) as *const libc::c_char;
        match type_0 {
            1 => {
                empty = xmlTextReaderIsEmptyElement(reader);
                r = xml2_xmlattr_setup(a, &mut list, reader);
                if r == ARCHIVE_OK {
                    r = xml_start(a, name, &mut list)
                }
                xmlattr_cleanup(&mut list);
                if r != ARCHIVE_OK {
                    return r;
                }
                if empty != 0 {
                    xml_end(a as *mut libc::c_void, name);
                }
            }
            15 => {
                xml_end(a as *mut libc::c_void, name);
            }
            3 => {
                value = xmlTextReaderConstValue(reader) as *const libc::c_char;
                xml_data(a as *mut libc::c_void, value, strlen(value) as libc::c_int);
            }
            14 | _ => {}
        }
        if r < 0 as libc::c_int {
            break;
        }
    }
    xmlFreeTextReader(reader);
    xmlCleanupParser();
    return if r == 0 as libc::c_int {
        ARCHIVE_OK
    } else {
        ARCHIVE_FATAL
    };
}
/* Support xar format */
/* defined(HAVE_BSDXML_H) || defined(HAVE_EXPAT_H) */
