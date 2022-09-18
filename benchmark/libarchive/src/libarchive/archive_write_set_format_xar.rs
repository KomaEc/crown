use ::c2rust_bitfields;
use ::libc;
extern "C" {
    /*
     * Summary: text writing API for XML
     * Description: text writing API for XML
     *
     * Copy: See Copyright for the status of this software.
     *
     * Author: Alfred Mickautsch <alfred@mickautsch.de>
     */
    pub type _xmlTextWriter;
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
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn xmlBufferCreate() -> xmlBufferPtr;
    #[no_mangle]
    fn xmlBufferFree(buf: xmlBufferPtr);
    #[no_mangle]
    fn xmlNewTextWriterMemory(buf: xmlBufferPtr, compression: libc::c_int) -> xmlTextWriterPtr;
    #[no_mangle]
    fn xmlFreeTextWriter(writer: xmlTextWriterPtr);
    /*
     * Functions
     */
    /*
     * Document
     */
    #[no_mangle]
    fn xmlTextWriterStartDocument(
        writer: xmlTextWriterPtr,
        version: *const libc::c_char,
        encoding: *const libc::c_char,
        standalone: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlTextWriterEndDocument(writer: xmlTextWriterPtr) -> libc::c_int;
    /*
     * Elements
     */
    #[no_mangle]
    fn xmlTextWriterStartElement(writer: xmlTextWriterPtr, name: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn xmlTextWriterEndElement(writer: xmlTextWriterPtr) -> libc::c_int;
    #[no_mangle]
    fn xmlTextWriterWriteString(writer: xmlTextWriterPtr, content: *const xmlChar) -> libc::c_int;
    #[no_mangle]
    fn xmlTextWriterWriteBase64(
        writer: xmlTextWriterPtr,
        data: *const libc::c_char,
        start: libc::c_int,
        len: libc::c_int,
    ) -> libc::c_int;
    /*
     * Attributes conveniency functions
     */
    #[no_mangle]
    fn xmlTextWriterWriteFormatAttribute(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn xmlTextWriterWriteAttribute(
        writer: xmlTextWriterPtr,
        name: *const xmlChar,
        content: *const xmlChar,
    ) -> libc::c_int;
    /*
     * Indentation
     */
    #[no_mangle]
    fn xmlTextWriterSetIndent(writer: xmlTextWriterPtr, indent: libc::c_int) -> libc::c_int;
    /*
     * Interfaces directly used by the parsers.
     */
    /*
     * Export a few useful functions
     */
    #[no_mangle]
    fn UTF8Toisolat1(
        out: *mut libc::c_uchar,
        outlen: *mut libc::c_int,
        in_0: *const libc::c_uchar,
        inlen: *mut libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
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
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
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
    /* defines */
    /* Minimal interface to digest functionality for internal use in libarchive */
    /* Message Digest */
    #[no_mangle]
    static __archive_digest: archive_digest;
    /* The 'clone' function does a deep copy; all of the strings are copied too. */
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
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
    /*
     * Retrieve fields from an archive_entry.
     *
     * There are a number of implicit conversions among these fields.  For
     * example, if a regular string field is set and you read the _w wide
     * character field, the entry will implicitly convert narrow-to-wide
     * using the current locale.  Similarly, dev values are automatically
     * updated when you write devmajor or devminor and vice versa.
     *
     * In addition, fields can be "set" or "unset."  Unset string fields
     * return NULL, non-string fields have _is_set() functions to test
     * whether they've been set.  You can "unset" a string field by
     * assigning NULL; non-string fields have _unset() functions to
     * unset them.
     *
     * Note: There is one ambiguity in the above; string fields will
     * also return NULL when implicit character set conversions fail.
     * This is usually what you want.
     */
    #[no_mangle]
    fn archive_entry_atime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_atime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_dev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_fflags_text(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_gname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_ino64(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_rdevmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_rdevminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_nlink(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_unset_size(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_xattr_reset(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_xattr_next(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut *const libc::c_void,
        _: *mut size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_gname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_pathname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_symlink_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_uname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_string_conversion_to_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_concat(dest: *mut archive_string, src: *mut archive_string);
    #[no_mangle]
    fn __archive_mktemp(tmpdir: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn archive_string_vsprintf(
        _: *mut archive_string,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    );
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn __archive_rb_tree_init(_: *mut archive_rb_tree, _: *const archive_rb_tree_ops);
    #[no_mangle]
    fn __archive_rb_tree_insert_node(
        _: *mut archive_rb_tree,
        _: *mut archive_rb_node,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_rb_tree_find_node(
        _: *mut archive_rb_tree,
        _: *const libc::c_void,
    ) -> *mut archive_rb_node;
    #[no_mangle]
    fn __archive_rb_tree_remove_node(_: *mut archive_rb_tree, _: *mut archive_rb_node);
    #[no_mangle]
    fn __archive_rb_tree_iterate(
        _: *mut archive_rb_tree,
        _: *mut archive_rb_node,
        _: libc::c_uint,
    ) -> *mut archive_rb_node;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type size_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type va_list = __builtin_va_list;
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
pub type xmlBufferAllocationScheme = libc::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: libc::c_uint,
    pub size: libc::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlTextWriter = _xmlTextWriter;
pub type xmlTextWriterPtr = *mut xmlTextWriter;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xar {
    pub temp_fd: libc::c_int,
    pub temp_offset: uint64_t,
    pub file_idx: libc::c_int,
    pub root: *mut file,
    pub cur_dirent: *mut file,
    pub cur_dirstr: archive_string,
    pub cur_file: *mut file,
    pub bytes_remaining: uint64_t,
    pub tstr: archive_string,
    pub vstr: archive_string,
    pub opt_toc_sumalg: sumalg,
    pub opt_sumalg: sumalg,
    pub opt_compression: enctype,
    pub opt_compression_level: libc::c_int,
    pub opt_threads: uint32_t,
    pub a_sumwrk: chksumwork,
    pub e_sumwrk: chksumwork,
    pub stream: la_zstream,
    pub sconv: *mut archive_string_conv,
    pub wbuff: [libc::c_uchar; 65536],
    pub wbuff_remaining: size_t,
    pub toc: heap_data,
    pub file_list: C2RustUnnamed,
    pub hardlink_rbtree: archive_rb_tree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree {
    pub rbt_root: *mut archive_rb_node,
    pub rbt_ops: *const archive_rb_tree_ops,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree_ops {
    pub rbto_compare_nodes: archive_rbto_compare_nodes_fn,
    pub rbto_compare_key: archive_rbto_compare_key_fn,
}
pub type archive_rbto_compare_key_fn =
    Option<unsafe extern "C" fn(_: *const archive_rb_node, _: *const libc::c_void) -> libc::c_int>;
/*-
 * Copyright (c) 2001 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Matt Thomas <matt@3am-software.com>.
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
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Based on NetBSD: rb.h,v 1.13 2009/08/16 10:57:01 yamt Exp
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_node {
    pub rb_nodes: [*mut archive_rb_node; 2],
    pub rb_info: uintptr_t,
}
/*
 * archive_rbto_compare_nodes_fn:
 *	return a positive value if the first node < the second node.
 *	return a negative value if the first node > the second node.
 *	return 0 if they are considered same.
 *
 * archive_rbto_compare_key_fn:
 *	return a positive value if the node < the key.
 *	return a negative value if the node > the key.
 *	return 0 if they are considered same.
 */
pub type archive_rbto_compare_nodes_fn = Option<
    unsafe extern "C" fn(_: *const archive_rb_node, _: *const archive_rb_node) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub first: *mut file,
    pub last: *mut *mut file,
}
/* extracted checksum.	*/
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub rbnode: archive_rb_node,
    pub id: libc::c_int,
    pub entry: *mut archive_entry,
    pub rbtree: archive_rb_tree,
    pub next: *mut file,
    pub chnext: *mut file,
    pub hlnext: *mut file,
    pub hardlink_target: *mut file,
    pub parent: *mut file,
    pub children: C2RustUnnamed_1,
    pub parentdir: archive_string,
    pub basename: archive_string,
    pub symlink: archive_string,
    pub ea_idx: libc::c_int,
    pub xattr: C2RustUnnamed_0,
    pub data: heap_data,
    pub script: archive_string,
    #[bitfield(name = "virtual_0", ty = "libc::c_int", bits = "0..=0")]
    #[bitfield(name = "dir", ty = "libc::c_int", bits = "1..=1")]
    pub virtual_0_dir: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_data {
    pub id: libc::c_int,
    pub next: *mut heap_data,
    pub temp_offset: uint64_t,
    pub length: uint64_t,
    pub size: uint64_t,
    pub compression: enctype,
    pub a_sum: chksumval,
    pub e_sum: chksumval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chksumval {
    pub alg: sumalg,
    pub len: size_t,
    pub val: [libc::c_uchar; 20],
}
pub type sumalg = libc::c_uint;
pub const CKSUM_MD5: sumalg = 2;
pub const CKSUM_SHA1: sumalg = 1;
pub const CKSUM_NONE: sumalg = 0;
pub type enctype = libc::c_uint;
pub const XZ: enctype = 4;
pub const LZMA: enctype = 3;
pub const BZIP2: enctype = 2;
pub const GZIP: enctype = 1;
pub const NONE: enctype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub first: *mut heap_data,
    pub last: *mut *mut heap_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub first: *mut file,
    pub last: *mut *mut file,
}
/*
 * Universal zstream.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct la_zstream {
    pub next_in: *const libc::c_uchar,
    pub avail_in: size_t,
    pub total_in: uint64_t,
    pub next_out: *mut libc::c_uchar,
    pub avail_out: size_t,
    pub total_out: uint64_t,
    pub valid: libc::c_int,
    pub real_stream: *mut libc::c_void,
    pub code: Option<
        unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream, _: la_zaction) -> libc::c_int,
    >,
    pub end: Option<unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream) -> libc::c_int>,
}
pub type la_zaction = libc::c_uint;
pub const ARCHIVE_Z_RUN: la_zaction = 1;
pub const ARCHIVE_Z_FINISH: la_zaction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chksumwork {
    pub alg: sumalg,
    pub md5ctx: archive_md5_ctx,
    pub sha1ctx: archive_sha1_ctx,
}
pub type archive_sha1_ctx = *mut EVP_MD_CTX;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type archive_md5_ctx = *mut EVP_MD_CTX;
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
pub struct flagentry {
    pub name: *const libc::c_char,
    pub xarname: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hardlink {
    pub rbnode: archive_rb_node,
    pub nlink: libc::c_int,
    pub file_list: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub first: *mut file,
    pub last: *mut *mut file,
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
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const PATH_MAX: libc::c_int = 4096 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
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
#[inline]
unsafe extern "C" fn archive_be16enc(mut pp: *mut libc::c_void, mut u: uint16_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_be32enc(mut pp: *mut libc::c_void, mut u: uint32_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) =
        (u >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(3 as libc::c_int as isize) =
        (u & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_be64enc(mut pp: *mut libc::c_void, mut u: uint64_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    archive_be32enc(p as *mut libc::c_void, (u >> 32 as libc::c_int) as uint32_t);
    archive_be32enc(
        p.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        (u & 0xffffffff as libc::c_uint as libc::c_ulong) as uint32_t,
    );
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
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
/*
 * Utility functions to set and get entry attributes by translating
 * character-set. These are designed for use in format readers and writers.
 *
 * The return code and interface of these are quite different from other
 * functions for archive_entry defined in archive_entry.h.
 * Common return code are:
 *   Return 0 if the string conversion succeeded.
 *   Return -1 if the string conversion failed.
 */
pub const archive_entry_gname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_gname_l;
pub const archive_entry_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_pathname_l;
pub const archive_entry_symlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_symlink_l;
pub const archive_entry_uname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_uname_l;
pub const ARCHIVE_RB_DIR_LEFT: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_RB_DIR_RIGHT: libc::c_int = 1 as libc::c_int;
/*
 * Differences to xar utility.
 * - Subdocument is not supported yet.
 * - ACL is not supported yet.
 * - When writing an XML element <link type="<file-type>">, <file-type>
 *   which is a file type a symbolic link is referencing is always marked
 *   as "broken". Xar utility uses stat(2) to get the file type, but, in
 *   libarchive format writer, we should not use it; if it is needed, we
 *   should get about it at archive_read_disk.c.
 * - It is possible to appear both <flags> and <ext2> elements.
 *   Xar utility generates <flags> on BSD platform and <ext2> on Linux
 *   platform.
 *
 */
/* Support xar format */
/*#define DEBUG_PRINT_TOC		1 */
pub const HEADER_MAGIC: libc::c_int = 0x78617221 as libc::c_int;
pub const HEADER_SIZE: libc::c_int = 28 as libc::c_int;
pub const HEADER_VERSION: libc::c_int = 1 as libc::c_int;
pub const MD5_SIZE: libc::c_int = 16 as libc::c_int;
pub const SHA1_SIZE: libc::c_int = 20 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_xar(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut xar: *mut xar = 0 as *mut xar;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_xar\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If another format was already registered, unregister it. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
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
    (*xar).temp_fd = -(1 as libc::c_int);
    file_init_register(xar);
    file_init_hardlinks(xar);
    (*xar).tstr.s = NULL as *mut libc::c_char;
    (*xar).tstr.length = 0 as libc::c_int as size_t;
    (*xar).tstr.buffer_length = 0 as libc::c_int as size_t;
    (*xar).vstr.s = NULL as *mut libc::c_char;
    (*xar).vstr.length = 0 as libc::c_int as size_t;
    (*xar).vstr.buffer_length = 0 as libc::c_int as size_t;
    /*
     * Create the root directory.
     */
    (*xar).root = file_create_virtual_dir(a, xar, b"\x00" as *const u8 as *const libc::c_char);
    if (*xar).root.is_null() {
        free(xar as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate xar data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*(*xar).root).parent = (*xar).root;
    file_register(xar, (*xar).root);
    (*xar).cur_dirent = (*xar).root;
    (*xar).cur_dirstr.s = NULL as *mut libc::c_char;
    (*xar).cur_dirstr.length = 0 as libc::c_int as size_t;
    (*xar).cur_dirstr.buffer_length = 0 as libc::c_int as size_t;
    archive_string_ensure(&mut (*xar).cur_dirstr, 1 as libc::c_int as size_t);
    *(*xar).cur_dirstr.s.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    /*
     * Initialize option.
     */
    /* Set default checksum type. */
    (*xar).opt_toc_sumalg = CKSUM_SHA1;
    (*xar).opt_sumalg = CKSUM_SHA1;
    /* Set default compression type, level, and number of threads. */
    (*xar).opt_compression = GZIP;
    (*xar).opt_compression_level = 6 as libc::c_int;
    (*xar).opt_threads = 1 as libc::c_int as uint32_t;
    (*a).format_data = xar as *mut libc::c_void;
    (*a).format_name = b"xar\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        xar_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        xar_write_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        xar_write_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry =
        Some(xar_finish_entry as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_close =
        Some(xar_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_free = Some(xar_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).archive.archive_format = ARCHIVE_FORMAT_XAR;
    (*a).archive.archive_format_name = b"xar\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xar_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    xar = (*a).format_data as *mut xar;
    if strcmp(key, b"checksum\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if value.is_null() {
            (*xar).opt_sumalg = CKSUM_NONE
        } else if strcmp(value, b"none\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_sumalg = CKSUM_NONE
        } else if strcmp(value, b"sha1\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_sumalg = CKSUM_SHA1
        } else if strcmp(value, b"md5\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_sumalg = CKSUM_MD5
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unknown checksum name: `%s\'\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(25 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if strcmp(key, b"compression\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        let mut name: *const libc::c_char = NULL as *const libc::c_char;
        if value.is_null() {
            (*xar).opt_compression = NONE
        } else if strcmp(value, b"none\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_compression = NONE
        } else if strcmp(value, b"gzip\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_compression = GZIP
        } else if strcmp(value, b"bzip2\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            name = b"bzip2\x00" as *const u8 as *const libc::c_char
        } else if strcmp(value, b"lzma\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            name = b"lzma\x00" as *const u8 as *const libc::c_char
        } else if strcmp(value, b"xz\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            name = b"xz\x00" as *const u8 as *const libc::c_char
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unknown compression name: `%s\'\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(25 as libc::c_int);
        }
        if !name.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"`%s\' compression not supported on this platform\x00" as *const u8
                    as *const libc::c_char,
                name,
            );
            return -(25 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
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
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Illegal value `%s\'\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(25 as libc::c_int);
        }
        (*xar).opt_compression_level =
            *value.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
        return 0 as libc::c_int;
    }
    if strcmp(key, b"toc-checksum\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if value.is_null() {
            (*xar).opt_toc_sumalg = CKSUM_NONE
        } else if strcmp(value, b"none\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_toc_sumalg = CKSUM_NONE
        } else if strcmp(value, b"sha1\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_toc_sumalg = CKSUM_SHA1
        } else if strcmp(value, b"md5\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*xar).opt_toc_sumalg = CKSUM_MD5
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Unknown checksum name: `%s\'\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(25 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if strcmp(key, b"threads\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        if value.is_null() {
            return -(25 as libc::c_int);
        }
        errno = 0 as libc::c_int;
        (*xar).opt_threads =
            strtoul(value, &mut endptr, 10 as libc::c_int) as libc::c_int as uint32_t;
        if errno != 0 as libc::c_int || *endptr as libc::c_int != '\u{0}' as i32 {
            (*xar).opt_threads = 1 as libc::c_int as uint32_t;
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Illegal value `%s\'\x00" as *const u8 as *const libc::c_char,
                value,
            );
            return -(25 as libc::c_int);
        }
        if (*xar).opt_threads == 0 as libc::c_int as libc::c_uint {
            (*xar).opt_threads = 1 as libc::c_int as uint32_t
        }
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
unsafe extern "C" fn xar_write_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut file: *mut file = 0 as *mut file;
    let mut file_entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut r: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    (*xar).cur_file = NULL as *mut file;
    (*xar).bytes_remaining = 0 as libc::c_int as uint64_t;
    if (*xar).sconv.is_null() {
        (*xar).sconv = archive_string_conversion_to_charset(
            &mut (*a).archive,
            b"UTF-8\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        if (*xar).sconv.is_null() {
            return -(30 as libc::c_int);
        }
    }
    file = file_new(a, entry);
    if file.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    r2 = file_gen_utility_names(a, file);
    if r2 < ARCHIVE_WARN {
        return r2;
    }
    /*
     * Ignore a path which looks like the top of directory name
     * since we have already made the root directory of an Xar archive.
     */
    if (*file).parentdir.length == 0 as libc::c_int as libc::c_ulong
        && (*file).basename.length == 0 as libc::c_int as libc::c_ulong
    {
        file_free(file);
        return r2;
    }
    /* Add entry into tree */
    file_entry = (*file).entry;
    r = file_tree(a, &mut file);
    if r != ARCHIVE_OK {
        return r;
    }
    /* There is the same file in tree and
     * the current file is older than the file in tree.
     * So we don't need the current file data anymore. */
    if (*file).entry != file_entry {
        return r2;
    }
    if (*file).id == 0 as libc::c_int {
        file_register(xar, file);
    }
    /* A virtual file, which is a directory, does not have
     * any contents and we won't store it into a archive
     * file other than its name. */
    if (*file).virtual_0() != 0 {
        return r2;
    }
    /*
     * Prepare to save the contents of the file.
     */
    if (*xar).temp_fd == -(1 as libc::c_int) {
        let mut algsize: libc::c_int = 0;
        (*xar).temp_offset = 0 as libc::c_int as uint64_t;
        (*xar).temp_fd = __archive_mktemp(NULL as *const libc::c_char);
        if (*xar).temp_fd < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Couldn\'t create temporary file\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        algsize = getalgsize((*xar).opt_toc_sumalg);
        if algsize > 0 as libc::c_int {
            if lseek((*xar).temp_fd, algsize as __off_t, SEEK_SET)
                < 0 as libc::c_int as libc::c_long
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    errno,
                    b"lseek failed\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            (*xar).temp_offset = algsize as uint64_t
        }
    }
    if archive_entry_hardlink((*file).entry).is_null() {
        r = save_xattrs(a, file);
        if r != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    /* Non regular files contents are unneeded to be saved to
     * a temporary file. */
    if archive_entry_filetype((*file).entry) != AE_IFREG as mode_t {
        return r2;
    }
    /*
     * Set the current file to cur_file to read its contents.
     */
    (*xar).cur_file = file;
    if archive_entry_nlink((*file).entry) > 1 as libc::c_int as libc::c_uint {
        r = file_register_hardlink(a, file);
        if r != ARCHIVE_OK {
            return r;
        }
        if !archive_entry_hardlink((*file).entry).is_null() {
            archive_entry_unset_size((*file).entry);
            return r2;
        }
    }
    /* Save a offset of current file in temporary file. */
    (*file).data.temp_offset = (*xar).temp_offset;
    (*file).data.size = archive_entry_size((*file).entry) as uint64_t;
    (*file).data.compression = (*xar).opt_compression;
    (*xar).bytes_remaining = archive_entry_size((*file).entry) as uint64_t;
    checksum_init(&mut (*xar).a_sumwrk, (*xar).opt_sumalg);
    checksum_init(&mut (*xar).e_sumwrk, (*xar).opt_sumalg);
    r = xar_compression_init_encoder(a);
    if r != ARCHIVE_OK {
        return r;
    } else {
        return r2;
    };
}
unsafe extern "C" fn write_to_temp(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ws: ssize_t = 0;
    xar = (*a).format_data as *mut xar;
    p = buff as *const libc::c_uchar;
    while s != 0 {
        ws = write((*xar).temp_fd, p as *const libc::c_void, s);
        if ws < 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"fwrite function failed\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        s = (s as libc::c_ulong).wrapping_sub(ws as libc::c_ulong) as size_t as size_t;
        p = p.offset(ws as isize);
        (*xar).temp_offset = ((*xar).temp_offset as libc::c_ulong).wrapping_add(ws as libc::c_ulong)
            as uint64_t as uint64_t
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xar_write_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut run: la_zaction = ARCHIVE_Z_FINISH;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut rsize: size_t = 0;
    let mut r: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    if s > (*xar).bytes_remaining {
        s = (*xar).bytes_remaining
    }
    if s == 0 as libc::c_int as libc::c_ulong || (*xar).cur_file.is_null() {
        return 0 as libc::c_int as ssize_t;
    }
    if (*(*xar).cur_file).data.compression as libc::c_uint == NONE as libc::c_int as libc::c_uint {
        checksum_update(&mut (*xar).e_sumwrk, buff, s);
        checksum_update(&mut (*xar).a_sumwrk, buff, s);
        rsize = s;
        size = rsize
    } else {
        (*xar).stream.next_in = buff as *const libc::c_uchar;
        (*xar).stream.avail_in = s;
        if (*xar).bytes_remaining > s {
            run = ARCHIVE_Z_RUN
        } else {
            run = ARCHIVE_Z_FINISH
        }
        loop
        /* Compress file data. */
        {
            r = compression_code(&mut (*a).archive, &mut (*xar).stream, run);
            if r != ARCHIVE_OK && r != ARCHIVE_EOF {
                return -(30 as libc::c_int) as ssize_t;
            }
            if (*xar).stream.avail_out == 0 as libc::c_int as libc::c_ulong
                || run as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint
            {
                size = (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
                    .wrapping_sub((*xar).stream.avail_out);
                checksum_update(
                    &mut (*xar).a_sumwrk,
                    (*xar).wbuff.as_mut_ptr() as *const libc::c_void,
                    size,
                );
                (*(*xar).cur_file).data.length = ((*(*xar).cur_file).data.length as libc::c_ulong)
                    .wrapping_add(size) as uint64_t
                    as uint64_t;
                if write_to_temp(a, (*xar).wbuff.as_mut_ptr() as *const libc::c_void, size)
                    != ARCHIVE_OK
                {
                    return -(30 as libc::c_int) as ssize_t;
                }
                if !(r == ARCHIVE_OK) {
                    break;
                }
                /* Output buffer was full */
                (*xar).stream.next_out = (*xar).wbuff.as_mut_ptr();
                (*xar).stream.avail_out =
                    ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong
            } else {
                /* Compressor wants more input */
                break;
            }
        }
        rsize = s.wrapping_sub((*xar).stream.avail_in);
        checksum_update(&mut (*xar).e_sumwrk, buff, rsize);
    }
    if (*xar).bytes_remaining == archive_entry_size((*(*xar).cur_file).entry) as uint64_t {
        /*
         * Get the path of a shell script if so.
         */
        let mut b: *const libc::c_uchar = buff as *const libc::c_uchar;
        (*(*xar).cur_file).script.length = 0 as libc::c_int as size_t;
        if rsize > 2 as libc::c_int as libc::c_ulong
            && *b.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            && *b.offset(1 as libc::c_int as isize) as libc::c_int == '!' as i32
        {
            let mut i: size_t = 0;
            let mut end: size_t = 0;
            let mut off: size_t = 0;
            off = 2 as libc::c_int as size_t;
            if *b.offset(off as isize) as libc::c_int == ' ' as i32 {
                off = off.wrapping_add(1)
            }
            if rsize.wrapping_sub(off) > PATH_MAX as libc::c_ulong {
                end = off.wrapping_add(PATH_MAX as libc::c_ulong)
            } else {
                end = rsize
            }
            /* Find the end of a script path. */
            i = off;
            while i < end
                && *b.offset(i as isize) as libc::c_int != '\u{0}' as i32
                && *b.offset(i as isize) as libc::c_int != '\n' as i32
                && *b.offset(i as isize) as libc::c_int != '\r' as i32
                && *b.offset(i as isize) as libc::c_int != ' ' as i32
                && *b.offset(i as isize) as libc::c_int != '\t' as i32
            {
                i = i.wrapping_add(1)
            }
            (*(*xar).cur_file).script.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut (*(*xar).cur_file).script,
                b.offset(off as isize) as *const libc::c_void,
                i.wrapping_sub(off),
            );
        }
    }
    if (*(*xar).cur_file).data.compression as libc::c_uint == NONE as libc::c_int as libc::c_uint {
        if write_to_temp(a, buff, size) != ARCHIVE_OK {
            return -(30 as libc::c_int) as ssize_t;
        }
        (*(*xar).cur_file).data.length = ((*(*xar).cur_file).data.length as libc::c_ulong)
            .wrapping_add(size) as uint64_t as uint64_t
    }
    (*xar).bytes_remaining =
        ((*xar).bytes_remaining as libc::c_ulong).wrapping_sub(rsize) as uint64_t as uint64_t;
    return rsize as ssize_t;
}
unsafe extern "C" fn xar_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut file: *mut file = 0 as *mut file;
    let mut s: size_t = 0;
    let mut w: ssize_t = 0;
    xar = (*a).format_data as *mut xar;
    if (*xar).cur_file.is_null() {
        return 0 as libc::c_int;
    }
    while (*xar).bytes_remaining > 0 as libc::c_int as libc::c_ulong {
        s = (*xar).bytes_remaining;
        if s > (*a).null_length {
            s = (*a).null_length
        }
        w = xar_write_data(a, (*a).nulls as *const libc::c_void, s);
        if w > 0 as libc::c_int as libc::c_long {
            (*xar).bytes_remaining = ((*xar).bytes_remaining as libc::c_ulong)
                .wrapping_sub(w as libc::c_ulong) as uint64_t
                as uint64_t
        } else {
            return w as libc::c_int;
        }
    }
    file = (*xar).cur_file;
    checksum_final(&mut (*xar).e_sumwrk, &mut (*file).data.e_sum);
    checksum_final(&mut (*xar).a_sumwrk, &mut (*file).data.a_sum);
    (*xar).cur_file = NULL as *mut file;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlwrite_string_attr(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
    mut attrkey: *const libc::c_char,
    mut attrvalue: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = xmlTextWriterStartElement(writer, key as *const xmlChar);
    if r < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"xmlTextWriterStartElement() failed: %d\x00" as *const u8 as *const libc::c_char,
            r,
        );
        return -(30 as libc::c_int);
    }
    if !attrkey.is_null() && !attrvalue.is_null() {
        r = xmlTextWriterWriteAttribute(
            writer,
            attrkey as *const xmlChar,
            attrvalue as *const xmlChar,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterWriteAttribute() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    }
    if !value.is_null() {
        r = xmlTextWriterWriteString(writer, value as *const xmlChar);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterWriteString() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    }
    r = xmlTextWriterEndElement(writer);
    if r < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
            r,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlwrite_string(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if value.is_null() {
        return 0 as libc::c_int;
    }
    r = xmlTextWriterStartElement(writer, key as *const xmlChar);
    if r < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"xmlTextWriterStartElement() failed: %d\x00" as *const u8 as *const libc::c_char,
            r,
        );
        return -(30 as libc::c_int);
    }
    if !value.is_null() {
        r = xmlTextWriterWriteString(writer, value as *const xmlChar);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterWriteString() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    }
    r = xmlTextWriterEndElement(writer);
    if r < 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
            r,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlwrite_fstring(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut key: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut ap: ::std::ffi::VaListImpl;
    xar = (*a).format_data as *mut xar;
    ap = args.clone();
    (*xar).vstr.length = 0 as libc::c_int as size_t;
    archive_string_vsprintf(&mut (*xar).vstr, fmt, ap.as_va_list());
    return xmlwrite_string(a, writer, key, (*xar).vstr.s);
}
unsafe extern "C" fn xmlwrite_time(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut key: *const libc::c_char,
    mut t: time_t,
    mut z: libc::c_int,
) -> libc::c_int {
    let mut timestr: [libc::c_char; 100] = [0; 100];
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
    gmtime_r(&mut t, &mut tm);
    memset(
        &mut timestr as *mut [libc::c_char; 100] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
    );
    /* Do not use %F and %T for portability. */
    strftime(
        timestr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        b"%Y-%m-%dT%H:%M:%S\x00" as *const u8 as *const libc::c_char,
        &mut tm,
    );
    if z != 0 {
        strcat(
            timestr.as_mut_ptr(),
            b"Z\x00" as *const u8 as *const libc::c_char,
        );
    }
    return xmlwrite_string(a, writer, key, timestr.as_mut_ptr());
}
unsafe extern "C" fn xmlwrite_mode(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut key: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    let mut ms: [libc::c_char; 5] = [0; 5];
    ms[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    ms[1 as libc::c_int as usize] = ('0' as i32 as libc::c_uint)
        .wrapping_add(mode >> 6 as libc::c_int & 0o7 as libc::c_int as libc::c_uint)
        as libc::c_char;
    ms[2 as libc::c_int as usize] = ('0' as i32 as libc::c_uint)
        .wrapping_add(mode >> 3 as libc::c_int & 0o7 as libc::c_int as libc::c_uint)
        as libc::c_char;
    ms[3 as libc::c_int as usize] = ('0' as i32 as libc::c_uint)
        .wrapping_add(mode & 0o7 as libc::c_int as libc::c_uint)
        as libc::c_char;
    ms[4 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    return xmlwrite_string(a, writer, key, ms.as_mut_ptr());
}
unsafe extern "C" fn xmlwrite_sum(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut key: *const libc::c_char,
    mut sum: *mut chksumval,
) -> libc::c_int {
    let mut algname: *const libc::c_char = 0 as *const libc::c_char;
    let mut algsize: libc::c_int = 0;
    let mut buff: [libc::c_char; 41] = [0; 41];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if (*sum).len > 0 as libc::c_int as libc::c_ulong {
        algname = getalgname((*sum).alg);
        algsize = getalgsize((*sum).alg);
        if !algname.is_null() {
            let mut hex: *const libc::c_char =
                b"0123456789abcdef\x00" as *const u8 as *const libc::c_char;
            p = buff.as_mut_ptr();
            s = (*sum).val.as_mut_ptr();
            i = 0 as libc::c_int;
            while i < algsize {
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = *hex.offset((*s as libc::c_int >> 4 as libc::c_int) as isize);
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = *hex.offset((*s as libc::c_int & 0xf as libc::c_int) as isize);
                s = s.offset(1);
                i += 1
            }
            *p = '\u{0}' as i32 as libc::c_char;
            r = xmlwrite_string_attr(
                a,
                writer,
                key,
                buff.as_mut_ptr(),
                b"style\x00" as *const u8 as *const libc::c_char,
                algname,
            );
            if r < 0 as libc::c_int {
                return -(30 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlwrite_heap(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut heap: *mut heap_data,
) -> libc::c_int {
    let mut encname: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    r = xmlwrite_fstring(
        a,
        writer,
        b"length\x00" as *const u8 as *const libc::c_char,
        b"%ju\x00" as *const u8 as *const libc::c_char,
        (*heap).length,
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    r = xmlwrite_fstring(
        a,
        writer,
        b"offset\x00" as *const u8 as *const libc::c_char,
        b"%ju\x00" as *const u8 as *const libc::c_char,
        (*heap).temp_offset,
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    r = xmlwrite_fstring(
        a,
        writer,
        b"size\x00" as *const u8 as *const libc::c_char,
        b"%ju\x00" as *const u8 as *const libc::c_char,
        (*heap).size,
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    match (*heap).compression as libc::c_uint {
        1 => encname = b"application/x-gzip\x00" as *const u8 as *const libc::c_char,
        2 => encname = b"application/x-bzip2\x00" as *const u8 as *const libc::c_char,
        3 => encname = b"application/x-lzma\x00" as *const u8 as *const libc::c_char,
        4 => encname = b"application/x-xz\x00" as *const u8 as *const libc::c_char,
        _ => encname = b"application/octet-stream\x00" as *const u8 as *const libc::c_char,
    }
    r = xmlwrite_string_attr(
        a,
        writer,
        b"encoding\x00" as *const u8 as *const libc::c_char,
        NULL as *const libc::c_char,
        b"style\x00" as *const u8 as *const libc::c_char,
        encname,
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    r = xmlwrite_sum(
        a,
        writer,
        b"archived-checksum\x00" as *const u8 as *const libc::c_char,
        &mut (*heap).a_sum,
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    r = xmlwrite_sum(
        a,
        writer,
        b"extracted-checksum\x00" as *const u8 as *const libc::c_char,
        &mut (*heap).e_sum,
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/*
 * xar utility records fflags as following xml elements:
 *   <flags>
 *     <UserNoDump/>
 *     .....
 *   </flags>
 * or
 *   <ext2>
 *     <NoDump/>
 *     .....
 *   </ext2>
 * If xar is running on BSD platform, records <flags>..</flags>;
 * if xar is running on linux platform, records <ext2>..</ext2>;
 * otherwise does not record.
 *
 * Our implements records both <flags> and <ext2> if it's necessary.
 */
unsafe extern "C" fn make_fflags_entry(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut element: *const libc::c_char,
    mut fflags_text: *const libc::c_char,
) -> libc::c_int {
    static mut flagbsd: [flagentry; 20] = [
        {
            let mut init = flagentry {
                name: b"sappnd\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemAppend\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"sappend\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemAppend\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"arch\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemArchived\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"archived\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemArchived\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"schg\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemImmutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"schange\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemImmutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"simmutable\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemImmutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nosunlnk\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemNoUnlink\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nosunlink\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemNoUnlink\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"snapshot\x00" as *const u8 as *const libc::c_char,
                xarname: b"SystemSnapshot\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"uappnd\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserAppend\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"uappend\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserAppend\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"uchg\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserImmutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"uchange\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserImmutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"uimmutable\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserImmutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nodump\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserNoDump\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"noopaque\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserOpaque\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nouunlnk\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserNoUnlink\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nouunlink\x00" as *const u8 as *const libc::c_char,
                xarname: b"UserNoUnlink\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: NULL as *const libc::c_char,
                xarname: NULL as *const libc::c_char,
            };
            init
        },
    ];
    static mut flagext2: [flagentry; 24] = [
        {
            let mut init = flagentry {
                name: b"sappnd\x00" as *const u8 as *const libc::c_char,
                xarname: b"AppendOnly\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"sappend\x00" as *const u8 as *const libc::c_char,
                xarname: b"AppendOnly\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"schg\x00" as *const u8 as *const libc::c_char,
                xarname: b"Immutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"schange\x00" as *const u8 as *const libc::c_char,
                xarname: b"Immutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"simmutable\x00" as *const u8 as *const libc::c_char,
                xarname: b"Immutable\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nodump\x00" as *const u8 as *const libc::c_char,
                xarname: b"NoDump\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nouunlnk\x00" as *const u8 as *const libc::c_char,
                xarname: b"Undelete\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"nouunlink\x00" as *const u8 as *const libc::c_char,
                xarname: b"Undelete\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"btree\x00" as *const u8 as *const libc::c_char,
                xarname: b"BTree\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"comperr\x00" as *const u8 as *const libc::c_char,
                xarname: b"CompError\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"compress\x00" as *const u8 as *const libc::c_char,
                xarname: b"Compress\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"noatime\x00" as *const u8 as *const libc::c_char,
                xarname: b"NoAtime\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"compdirty\x00" as *const u8 as *const libc::c_char,
                xarname: b"CompDirty\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"comprblk\x00" as *const u8 as *const libc::c_char,
                xarname: b"CompBlock\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"dirsync\x00" as *const u8 as *const libc::c_char,
                xarname: b"DirSync\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"hashidx\x00" as *const u8 as *const libc::c_char,
                xarname: b"HashIndexed\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"imagic\x00" as *const u8 as *const libc::c_char,
                xarname: b"iMagic\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"journal\x00" as *const u8 as *const libc::c_char,
                xarname: b"Journaled\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"securedeletion\x00" as *const u8 as *const libc::c_char,
                xarname: b"SecureDeletion\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"sync\x00" as *const u8 as *const libc::c_char,
                xarname: b"Synchronous\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"notail\x00" as *const u8 as *const libc::c_char,
                xarname: b"NoTail\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"topdir\x00" as *const u8 as *const libc::c_char,
                xarname: b"TopDir\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: b"reserved\x00" as *const u8 as *const libc::c_char,
                xarname: b"Reserved\x00" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = flagentry {
                name: NULL as *const libc::c_char,
                xarname: NULL as *const libc::c_char,
            };
            init
        },
    ];
    let mut fe: *const flagentry = 0 as *const flagentry;
    let mut flagentry_0: *const flagentry = 0 as *const flagentry;
    let mut avail: [*const flagentry; 2] = [0 as *const flagentry; 2];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    if strcmp(element, b"ext2\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        flagentry_0 = flagext2.as_ptr()
    } else {
        flagentry_0 = flagbsd.as_ptr()
    }
    n = 0 as libc::c_int;
    p = fflags_text;
    loop {
        let mut cp: *const libc::c_char = 0 as *const libc::c_char;
        cp = strchr(p, ',' as i32);
        if cp.is_null() {
            cp = p.offset(strlen(p) as isize)
        }
        fe = flagentry_0;
        while !(*fe).name.is_null() {
            if !(*(*fe)
                .name
                .offset(cp.offset_from(p) as libc::c_long as isize)
                as libc::c_int
                != '\u{0}' as i32
                || *p.offset(0 as libc::c_int as isize) as libc::c_int
                    != *(*fe).name.offset(0 as libc::c_int as isize) as libc::c_int)
            {
                if strncmp(
                    p,
                    (*fe).name,
                    cp.offset_from(p) as libc::c_long as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    let fresh2 = n;
                    n = n + 1;
                    avail[fresh2 as usize] = fe;
                    break;
                }
            }
            fe = fe.offset(1)
        }
        if *cp as libc::c_int == ',' as i32 {
            p = cp.offset(1 as libc::c_int as isize)
        } else {
            p = NULL as *const libc::c_char
        }
        if p.is_null() {
            break;
        }
    }
    if n > 0 as libc::c_int {
        r = xmlTextWriterStartElement(writer, element as *const xmlChar);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterStartElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < n {
            r = xmlwrite_string(
                a,
                writer,
                (*avail[i as usize]).xarname,
                NULL as *const libc::c_char,
            );
            if r != ARCHIVE_OK {
                return r;
            }
            i += 1
        }
        r = xmlTextWriterEndElement(writer);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn make_file_entry(
    mut a: *mut archive_write,
    mut writer: xmlTextWriterPtr,
    mut file: *mut file,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut filetype: *const libc::c_char = 0 as *const libc::c_char;
    let mut filelink: *const libc::c_char = 0 as *const libc::c_char;
    let mut fflags: *const libc::c_char = 0 as *const libc::c_char;
    let mut linkto: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut heap: *mut heap_data = 0 as *mut heap_data;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ll: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    r2 = ARCHIVE_OK;
    /*
     * Make a file name entry, "<name>".
     */
    ll = (*file).basename.length as libc::c_int;
    l = ll;
    tmp = malloc(l as libc::c_ulong) as *mut libc::c_uchar;
    if tmp.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    r = UTF8Toisolat1(tmp, &mut l, (*file).basename.s as *mut xmlChar, &mut ll);
    free(tmp as *mut libc::c_void);
    if r < 0 as libc::c_int {
        r = xmlTextWriterStartElement(
            writer,
            b"name\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterStartElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        r = xmlTextWriterWriteAttribute(
            writer,
            b"enctype\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
            b"base64\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterWriteAttribute() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        r = xmlTextWriterWriteBase64(
            writer,
            (*file).basename.s,
            0 as libc::c_int,
            (*file).basename.length as libc::c_int,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterWriteBase64() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        r = xmlTextWriterEndElement(writer);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    } else {
        r = xmlwrite_string(
            a,
            writer,
            b"name\x00" as *const u8 as *const libc::c_char,
            (*file).basename.s,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Make a file type entry, "<type>".
     */
    filelink = NULL as *const libc::c_char;
    linkto.s = NULL as *mut libc::c_char;
    linkto.length = 0 as libc::c_int as size_t;
    linkto.buffer_length = 0 as libc::c_int as size_t;
    match archive_entry_filetype((*file).entry) {
        16384 => filetype = b"directory\x00" as *const u8 as *const libc::c_char,
        40960 => filetype = b"symlink\x00" as *const u8 as *const libc::c_char,
        8192 => filetype = b"character special\x00" as *const u8 as *const libc::c_char,
        24576 => filetype = b"block special\x00" as *const u8 as *const libc::c_char,
        49152 => filetype = b"socket\x00" as *const u8 as *const libc::c_char,
        4096 => filetype = b"fifo\x00" as *const u8 as *const libc::c_char,
        32768 | _ => {
            if !(*file).hardlink_target.is_null() {
                filetype = b"hardlink\x00" as *const u8 as *const libc::c_char;
                filelink = b"link\x00" as *const u8 as *const libc::c_char;
                if (*file).hardlink_target == file {
                    linkto.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut linkto,
                        b"original\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                        (if (b"original\x00" as *const u8 as *const libc::c_char).is_null() {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            strlen(b"original\x00" as *const u8 as *const libc::c_char)
                        }),
                    );
                } else {
                    archive_string_sprintf(
                        &mut linkto as *mut archive_string,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*(*file).hardlink_target).id,
                    );
                }
            } else {
                filetype = b"file\x00" as *const u8 as *const libc::c_char
            }
        }
    }
    r = xmlwrite_string_attr(
        a,
        writer,
        b"type\x00" as *const u8 as *const libc::c_char,
        filetype,
        filelink,
        linkto.s,
    );
    archive_string_free(&mut linkto);
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    /*
     * On a virtual directory, we record "name" and "type" only.
     */
    if (*file).virtual_0() != 0 {
        return 0 as libc::c_int;
    }
    match archive_entry_filetype((*file).entry) {
        40960 => {
            /*
             * xar utility has checked a file type, which
             * a symbolic-link file has referenced.
             * For example:
             *   <link type="directory">../ref/</link>
             *   The symlink target file is "../ref/" and its
             *   file type is a directory.
             *
             *   <link type="file">../f</link>
             *   The symlink target file is "../f" and its
             *   file type is a regular file.
             *
             * But our implementation cannot do it, and then we
             * always record that a attribute "type" is "broken",
             * for example:
             *   <link type="broken">foo/bar</link>
             *   It means "foo/bar" is not reachable.
             */
            r = xmlwrite_string_attr(
                a,
                writer,
                b"link\x00" as *const u8 as *const libc::c_char,
                (*file).symlink.s,
                b"type\x00" as *const u8 as *const libc::c_char,
                b"broken\x00" as *const u8 as *const libc::c_char,
            );
            if r < 0 as libc::c_int {
                return -(30 as libc::c_int);
            }
        }
        8192 | 24576 => {
            r = xmlTextWriterStartElement(
                writer,
                b"device\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
            );
            if r < 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"xmlTextWriterStartElement() failed: %d\x00" as *const u8
                        as *const libc::c_char,
                    r,
                );
                return -(30 as libc::c_int);
            }
            r = xmlwrite_fstring(
                a,
                writer,
                b"major\x00" as *const u8 as *const libc::c_char,
                b"%d\x00" as *const u8 as *const libc::c_char,
                archive_entry_rdevmajor((*file).entry),
            );
            if r < 0 as libc::c_int {
                return -(30 as libc::c_int);
            }
            r = xmlwrite_fstring(
                a,
                writer,
                b"minor\x00" as *const u8 as *const libc::c_char,
                b"%d\x00" as *const u8 as *const libc::c_char,
                archive_entry_rdevminor((*file).entry),
            );
            if r < 0 as libc::c_int {
                return -(30 as libc::c_int);
            }
            r = xmlTextWriterEndElement(writer);
            if r < 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                    r,
                );
                return -(30 as libc::c_int);
            }
        }
        _ => {}
    }
    /*
     * Make a inode entry, "<inode>".
     */
    r = xmlwrite_fstring(
        a,
        writer,
        b"inode\x00" as *const u8 as *const libc::c_char,
        b"%jd\x00" as *const u8 as *const libc::c_char,
        archive_entry_ino64((*file).entry),
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    if archive_entry_dev((*file).entry) != 0 as libc::c_int as libc::c_ulong {
        r = xmlwrite_fstring(
            a,
            writer,
            b"deviceno\x00" as *const u8 as *const libc::c_char,
            b"%d\x00" as *const u8 as *const libc::c_char,
            archive_entry_dev((*file).entry),
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Make a file mode entry, "<mode>".
     */
    r = xmlwrite_mode(
        a,
        writer,
        b"mode\x00" as *const u8 as *const libc::c_char,
        archive_entry_mode((*file).entry),
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    /*
     * Make a user entry, "<uid>" and "<user>.
     */
    
    r = xmlwrite_fstring(
        a,
        writer,
        b"uid\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        archive_entry_uid((*file).entry),
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    r = _archive_entry_uname_l((*file).entry, &mut p, &mut len, (*xar).sconv);
    if r != 0 as libc::c_int {
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
            b"Can\'t translate uname \'%s\' to UTF-8\x00" as *const u8 as *const libc::c_char,
            archive_entry_uname((*file).entry),
        );
        r2 = ARCHIVE_WARN
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        r = xmlwrite_string(
            a,
            writer,
            b"user\x00" as *const u8 as *const libc::c_char,
            p,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Make a group entry, "<gid>" and "<group>.
     */
    
    r = xmlwrite_fstring(
        a,
        writer,
        b"gid\x00" as *const u8 as *const libc::c_char,
        b"%d\x00" as *const u8 as *const libc::c_char,
        archive_entry_gid((*file).entry),
    );
    if r < 0 as libc::c_int {
        return -(30 as libc::c_int);
    }
    r = _archive_entry_gname_l((*file).entry, &mut p, &mut len, (*xar).sconv);
    if r != 0 as libc::c_int {
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
            b"Can\'t translate gname \'%s\' to UTF-8\x00" as *const u8 as *const libc::c_char,
            archive_entry_gname((*file).entry),
        );
        r2 = ARCHIVE_WARN
    }
    if len > 0 as libc::c_int as libc::c_ulong {
        r = xmlwrite_string(
            a,
            writer,
            b"group\x00" as *const u8 as *const libc::c_char,
            p,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Make a ctime entry, "<ctime>".
     */
    if archive_entry_ctime_is_set((*file).entry) != 0 {
        r = xmlwrite_time(
            a,
            writer,
            b"ctime\x00" as *const u8 as *const libc::c_char,
            archive_entry_ctime((*file).entry),
            1 as libc::c_int,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Make a mtime entry, "<mtime>".
     */
    if archive_entry_mtime_is_set((*file).entry) != 0 {
        r = xmlwrite_time(
            a,
            writer,
            b"mtime\x00" as *const u8 as *const libc::c_char,
            archive_entry_mtime((*file).entry),
            1 as libc::c_int,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Make a atime entry, "<atime>".
     */
    if archive_entry_atime_is_set((*file).entry) != 0 {
        r = xmlwrite_time(
            a,
            writer,
            b"atime\x00" as *const u8 as *const libc::c_char,
            archive_entry_atime((*file).entry),
            1 as libc::c_int,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
    }
    /*
     * Make fflags entries, "<flags>" and "<ext2>".
     */
    fflags = archive_entry_fflags_text((*file).entry);
    if !fflags.is_null() {
        r = make_fflags_entry(
            a,
            writer,
            b"flags\x00" as *const u8 as *const libc::c_char,
            fflags,
        );
        if r < 0 as libc::c_int {
            return r;
        }
        r = make_fflags_entry(
            a,
            writer,
            b"ext2\x00" as *const u8 as *const libc::c_char,
            fflags,
        );
        if r < 0 as libc::c_int {
            return r;
        }
    }
    /*
     * Make extended attribute entries, "<ea>".
     */
    archive_entry_xattr_reset((*file).entry);
    heap = (*file).xattr.first;
    while !heap.is_null() {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *const libc::c_void = 0 as *const libc::c_void;
        let mut size: size_t = 0;
        archive_entry_xattr_next((*file).entry, &mut name, &mut value, &mut size);
        r = xmlTextWriterStartElement(
            writer,
            b"ea\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterStartElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        r = xmlTextWriterWriteFormatAttribute(
            writer,
            b"id\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*heap).id,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterWriteAttribute() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        r = xmlwrite_heap(a, writer, heap);
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
        r = xmlwrite_string(
            a,
            writer,
            b"name\x00" as *const u8 as *const libc::c_char,
            name,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
        r = xmlTextWriterEndElement(writer);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        heap = (*heap).next
    }
    /*
     * Make a file data entry, "<data>".
     */
    if (*file).data.length > 0 as libc::c_int as libc::c_ulong {
        r = xmlTextWriterStartElement(
            writer,
            b"data\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterStartElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        r = xmlwrite_heap(a, writer, &mut (*file).data);
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
        r = xmlTextWriterEndElement(writer);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    }
    if (*file).script.length > 0 as libc::c_int as libc::c_ulong {
        r = xmlTextWriterStartElement(
            writer,
            b"content\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
        );
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterStartElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
        r = xmlwrite_string(
            a,
            writer,
            b"interpreter\x00" as *const u8 as *const libc::c_char,
            (*file).script.s,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
        r = xmlwrite_string(
            a,
            writer,
            b"type\x00" as *const u8 as *const libc::c_char,
            b"script\x00" as *const u8 as *const libc::c_char,
        );
        if r < 0 as libc::c_int {
            return -(30 as libc::c_int);
        }
        r = xmlTextWriterEndElement(writer);
        if r < 0 as libc::c_int {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlTextWriterEndElement() failed: %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    }
    return r2;
}
/*
 * Make the TOC
 */
unsafe extern "C" fn make_toc(mut a: *mut archive_write) -> libc::c_int {
    let mut current_block: u64;
    let mut xar: *mut xar = 0 as *mut xar;
    let mut np: *mut file = 0 as *mut file;
    let mut bp: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut writer: xmlTextWriterPtr = 0 as *mut xmlTextWriter;
    let mut algsize: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    ret = ARCHIVE_FATAL;
    /*
     * Initialize xml writer.
     */
    writer = NULL as xmlTextWriterPtr;
    bp = xmlBufferCreate();
    if bp.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"xmlBufferCreate() couldn\'t create xml buffer\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
        writer = xmlNewTextWriterMemory(bp, 0 as libc::c_int);
        if writer.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"xmlNewTextWriterMemory() couldn\'t create xml writer\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            r = xmlTextWriterStartDocument(
                writer,
                b"1.0\x00" as *const u8 as *const libc::c_char,
                b"UTF-8\x00" as *const u8 as *const libc::c_char,
                NULL as *const libc::c_char,
            );
            if r < 0 as libc::c_int {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"xmlTextWriterStartDocument() failed: %d\x00" as *const u8
                        as *const libc::c_char,
                    r,
                );
            } else {
                r = xmlTextWriterSetIndent(writer, 4 as libc::c_int);
                if r < 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"xmlTextWriterSetIndent() failed: %d\x00" as *const u8
                            as *const libc::c_char,
                        r,
                    );
                } else {
                    /*
                     * Start recording TOC
                     */
                    r = xmlTextWriterStartElement(
                        writer,
                        b"xar\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                    );
                    if r < 0 as libc::c_int {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"xmlTextWriterStartElement() failed: %d\x00" as *const u8
                                as *const libc::c_char,
                            r,
                        );
                    } else {
                        r = xmlTextWriterStartElement(
                            writer,
                            b"toc\x00" as *const u8 as *const libc::c_char as *mut xmlChar,
                        );
                        if r < 0 as libc::c_int {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"xmlTextWriterStartDocument() failed: %d\x00" as *const u8
                                    as *const libc::c_char,
                                r,
                            );
                        } else {
                            /*
                             * Record the creation time of the archive file.
                             */
                            r = xmlwrite_time(
                                a,
                                writer,
                                b"creation-time\x00" as *const u8 as *const libc::c_char,
                                time(NULL as *mut time_t),
                                0 as libc::c_int,
                            );
                            if !(r < 0 as libc::c_int) {
                                /*
                                 * Record the checksum value of TOC
                                 */
                                algsize = getalgsize((*xar).opt_toc_sumalg);
                                if algsize != 0 {
                                    /*
                                     * Record TOC checksum
                                     */
                                    r = xmlTextWriterStartElement(
                                        writer,
                                        b"checksum\x00" as *const u8 as *const libc::c_char
                                            as *mut xmlChar,
                                    );
                                    if r < 0 as libc::c_int {
                                        archive_set_error(
                                            &mut (*a).archive as *mut archive,
                                            ARCHIVE_ERRNO_MISC,
                                            b"xmlTextWriterStartElement() failed: %d\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                            r,
                                        );
                                        current_block = 17775754040026677240;
                                    } else {
                                        r = xmlTextWriterWriteAttribute(
                                            writer,
                                            b"style\x00" as *const u8 as *const libc::c_char
                                                as *mut xmlChar,
                                            getalgname((*xar).opt_toc_sumalg) as *const xmlChar,
                                        );
                                        if r < 0 as libc::c_int {
                                            archive_set_error(
                                                &mut (*a).archive as *mut archive,
                                                ARCHIVE_ERRNO_MISC,
                                                b"xmlTextWriterWriteAttribute() failed: %d\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                r,
                                            );
                                            current_block = 17775754040026677240;
                                        } else {
                                            /*
                                             * Record the offset of the value of checksum of TOC
                                             */
                                            r = xmlwrite_string(
                                                a,
                                                writer,
                                                b"offset\x00" as *const u8 as *const libc::c_char,
                                                b"0\x00" as *const u8 as *const libc::c_char,
                                            );
                                            if r < 0 as libc::c_int {
                                                current_block = 17775754040026677240;
                                            } else {
                                                /*
                                                 * Record the size of the value of checksum of TOC
                                                 */
                                                r = xmlwrite_fstring(
                                                    a,
                                                    writer,
                                                    b"size\x00" as *const u8 as *const libc::c_char,
                                                    b"%d\x00" as *const u8 as *const libc::c_char,
                                                    algsize,
                                                );
                                                if r < 0 as libc::c_int {
                                                    current_block = 17775754040026677240;
                                                } else {
                                                    r = xmlTextWriterEndElement(writer);
                                                    if r < 0 as libc::c_int {
                                                        archive_set_error(&mut (*a).archive
                                                                              as
                                                                              *mut archive,
                                                                          ARCHIVE_ERRNO_MISC,
                                                                          b"xmlTextWriterEndElement() failed: %d\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          r);
                                                        current_block = 17775754040026677240;
                                                    } else {
                                                        current_block = 15004371738079956865;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    current_block = 15004371738079956865;
                                }
                                match current_block {
                                    17775754040026677240 => {}
                                    _ => {
                                        np = (*xar).root;
                                        's_234: loop {
                                            if np != (*np).parent {
                                                r = make_file_entry(a, writer, np);
                                                if r != ARCHIVE_OK {
                                                    current_block = 17775754040026677240;
                                                    break;
                                                }
                                            }
                                            if (*np).dir() != 0 && !(*np).children.first.is_null() {
                                                /* Enter to sub directories. */
                                                np = (*np).children.first;
                                                r = xmlTextWriterStartElement(
                                                    writer,
                                                    b"file\x00" as *const u8 as *const libc::c_char
                                                        as *mut xmlChar,
                                                );
                                                if r < 0 as libc::c_int {
                                                    archive_set_error(&mut (*a).archive
                                                                              as
                                                                              *mut archive,
                                                                          ARCHIVE_ERRNO_MISC,
                                                                          b"xmlTextWriterStartElement() failed: %d\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          r);
                                                    current_block = 17775754040026677240;
                                                    break;
                                                } else {
                                                    r = xmlTextWriterWriteFormatAttribute(
                                                        writer,
                                                        b"id\x00" as *const u8
                                                            as *const libc::c_char
                                                            as *mut xmlChar,
                                                        b"%d\x00" as *const u8
                                                            as *const libc::c_char,
                                                        (*np).id,
                                                    );
                                                    if r < 0 as libc::c_int {
                                                        archive_set_error(&mut (*a).archive
                                                                                  as
                                                                                  *mut archive,
                                                                              ARCHIVE_ERRNO_MISC,
                                                                              b"xmlTextWriterWriteAttribute() failed: %d\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              r);
                                                        current_block = 17775754040026677240;
                                                        break;
                                                    }
                                                }
                                            } else {
                                                while np != (*np).parent {
                                                    r = xmlTextWriterEndElement(writer);
                                                    if r < 0 as libc::c_int {
                                                        archive_set_error(&mut (*a).archive
                                                                                  as
                                                                                  *mut archive,
                                                                              ARCHIVE_ERRNO_MISC,
                                                                              b"xmlTextWriterEndElement() failed: %d\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              r);
                                                        current_block = 17775754040026677240;
                                                        break 's_234;
                                                    } else if (*np).chnext.is_null() {
                                                        /* Return to the parent directory. */
                                                        np = (*np).parent
                                                    } else {
                                                        np = (*np).chnext;
                                                        r = xmlTextWriterStartElement(
                                                            writer,
                                                            b"file\x00" as *const u8
                                                                as *const libc::c_char
                                                                as *mut xmlChar,
                                                        );
                                                        if r < 0 as libc::c_int {
                                                            archive_set_error(&mut (*a).archive
                                                                                      as
                                                                                      *mut archive,
                                                                                  ARCHIVE_ERRNO_MISC,
                                                                                  b"xmlTextWriterStartElement() failed: %d\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  r);
                                                            current_block = 17775754040026677240;
                                                            break 's_234;
                                                        } else {
                                                            r = xmlTextWriterWriteFormatAttribute(
                                                                writer,
                                                                b"id\x00" as *const u8
                                                                    as *const libc::c_char
                                                                    as *mut xmlChar,
                                                                b"%d\x00" as *const u8
                                                                    as *const libc::c_char,
                                                                (*np).id,
                                                            );
                                                            if !(r < 0 as libc::c_int) {
                                                                break;
                                                            }
                                                            archive_set_error(&mut (*a).archive
                                                                                      as
                                                                                      *mut archive,
                                                                                  ARCHIVE_ERRNO_MISC,
                                                                                  b"xmlTextWriterWriteAttribute() failed: %d\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  r);
                                                            current_block = 17775754040026677240;
                                                            break 's_234;
                                                        }
                                                    }
                                                }
                                            }
                                            if !(np != (*np).parent) {
                                                current_block = 10809827304263610514;
                                                break;
                                            }
                                        }
                                        match current_block {
                                            17775754040026677240 => {}
                                            _ => {
                                                r = xmlTextWriterEndDocument(writer);
                                                if r < 0 as libc::c_int {
                                                    archive_set_error(
                                                        &mut (*a).archive as *mut archive,
                                                        ARCHIVE_ERRNO_MISC,
                                                        b"xmlTextWriterEndDocument() failed: %d\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                        r,
                                                    );
                                                } else {
                                                    /*
                                                     * Compress the TOC and calculate the sum of the TOC.
                                                     */
                                                    (*xar).toc.temp_offset = (*xar).temp_offset;
                                                    (*xar).toc.size = (*bp).use_0 as uint64_t;
                                                    checksum_init(
                                                        &mut (*xar).a_sumwrk,
                                                        (*xar).opt_toc_sumalg,
                                                    );
                                                    r = compression_init_encoder_gzip(
                                                        &mut (*a).archive,
                                                        &mut (*xar).stream,
                                                        6 as libc::c_int,
                                                        1 as libc::c_int,
                                                    );
                                                    if !(r != ARCHIVE_OK) {
                                                        (*xar).stream.next_in = (*bp).content;
                                                        (*xar).stream.avail_in =
                                                            (*bp).use_0 as size_t;
                                                        (*xar).stream.total_in =
                                                            0 as libc::c_int as uint64_t;
                                                        (*xar).stream.next_out =
                                                            (*xar).wbuff.as_mut_ptr();
                                                        (*xar).stream.avail_out =
                                                            ::std::mem::size_of::<
                                                                [libc::c_uchar; 65536],
                                                            >(
                                                            )
                                                                as libc::c_ulong;
                                                        (*xar).stream.total_out =
                                                            0 as libc::c_int as uint64_t;
                                                        loop {
                                                            let mut size: size_t = 0;
                                                            r = compression_code(
                                                                &mut (*a).archive,
                                                                &mut (*xar).stream,
                                                                ARCHIVE_Z_FINISH,
                                                            );
                                                            if r != ARCHIVE_OK && r != ARCHIVE_EOF {
                                                                current_block =
                                                                    17775754040026677240;
                                                                break;
                                                            }
                                                            size = (::std::mem::size_of::<
                                                                [libc::c_uchar; 65536],
                                                            >(
                                                            )
                                                                as libc::c_ulong)
                                                                .wrapping_sub(
                                                                    (*xar).stream.avail_out,
                                                                );
                                                            checksum_update(
                                                                &mut (*xar).a_sumwrk,
                                                                (*xar).wbuff.as_mut_ptr()
                                                                    as *const libc::c_void,
                                                                size,
                                                            );
                                                            if write_to_temp(
                                                                a,
                                                                (*xar).wbuff.as_mut_ptr()
                                                                    as *const libc::c_void,
                                                                size,
                                                            ) != ARCHIVE_OK
                                                            {
                                                                current_block =
                                                                    17775754040026677240;
                                                                break;
                                                            }
                                                            if r == ARCHIVE_EOF {
                                                                current_block = 1417769144978639029;
                                                                break;
                                                            }
                                                            (*xar).stream.next_out =
                                                                (*xar).wbuff.as_mut_ptr();
                                                            (*xar).stream.avail_out =
                                                                ::std::mem::size_of::<
                                                                    [libc::c_uchar; 65536],
                                                                >(
                                                                )
                                                                    as libc::c_ulong
                                                        }
                                                        match current_block {
                                                            17775754040026677240 => {}
                                                            _ => {
                                                                r = compression_end(
                                                                    &mut (*a).archive,
                                                                    &mut (*xar).stream,
                                                                );
                                                                if !(r != ARCHIVE_OK) {
                                                                    (*xar).toc.length =
                                                                        (*xar).stream.total_out;
                                                                    (*xar).toc.compression = GZIP;
                                                                    checksum_final(
                                                                        &mut (*xar).a_sumwrk,
                                                                        &mut (*xar).toc.a_sum,
                                                                    );
                                                                    ret = ARCHIVE_OK
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
    }
    if !writer.is_null() {
        xmlFreeTextWriter(writer);
    }
    if !bp.is_null() {
        xmlBufferFree(bp);
    }
    return ret;
}
unsafe extern "C" fn flush_wbuff(mut a: *mut archive_write) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut r: libc::c_int = 0;
    let mut s: size_t = 0;
    xar = (*a).format_data as *mut xar;
    s = (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
        .wrapping_sub((*xar).wbuff_remaining);
    r = __archive_write_output(a, (*xar).wbuff.as_mut_ptr() as *const libc::c_void, s);
    if r != ARCHIVE_OK {
        return r;
    }
    (*xar).wbuff_remaining = ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong;
    return r;
}
unsafe extern "C" fn copy_out(
    mut a: *mut archive_write,
    mut offset: uint64_t,
    mut length: uint64_t,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut r: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    if lseek((*xar).temp_fd, offset as __off_t, SEEK_SET) < 0 as libc::c_int as libc::c_long {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            errno,
            b"lseek failed\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    while length != 0 {
        let mut rsize: size_t = 0;
        let mut rs: ssize_t = 0;
        let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if length > (*xar).wbuff_remaining {
            rsize = (*xar).wbuff_remaining
        } else {
            rsize = length
        }
        wb = (*xar).wbuff.as_mut_ptr().offset(
            (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
                .wrapping_sub((*xar).wbuff_remaining) as isize,
        );
        rs = read((*xar).temp_fd, wb as *mut libc::c_void, rsize);
        if rs < 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                errno,
                b"Can\'t read temporary file(%jd)\x00" as *const u8 as *const libc::c_char,
                rs,
            );
            return -(30 as libc::c_int);
        }
        if rs == 0 as libc::c_int as libc::c_long {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                0 as libc::c_int,
                b"Truncated xar archive\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*xar).wbuff_remaining = ((*xar).wbuff_remaining as libc::c_ulong)
            .wrapping_sub(rs as libc::c_ulong) as size_t as size_t;
        length =
            (length as libc::c_ulong).wrapping_sub(rs as libc::c_ulong) as uint64_t as uint64_t;
        if (*xar).wbuff_remaining == 0 as libc::c_int as libc::c_ulong {
            r = flush_wbuff(a);
            if r != ARCHIVE_OK {
                return r;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xar_close(mut a: *mut archive_write) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut wb: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut length: uint64_t = 0;
    let mut r: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    /* Empty! */
    if (*(*xar).root).children.first.is_null() {
        return 0 as libc::c_int;
    }
    /* Save the length of all file extended attributes and contents. */
    length = (*xar).temp_offset;
    /* Connect hardlinked files */
    file_connect_hardlink_files(xar);
    /* Make the TOC */
    r = make_toc(a);
    if r != ARCHIVE_OK {
        return r;
    }
    /*
     * Make the xar header on wbuff(write buffer).
     */
    wb = (*xar).wbuff.as_mut_ptr();
    (*xar).wbuff_remaining = ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong;
    archive_be32enc(
        &mut *wb.offset(0 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        HEADER_MAGIC as uint32_t,
    );
    archive_be16enc(
        &mut *wb.offset(4 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        HEADER_SIZE as uint16_t,
    );
    archive_be16enc(
        &mut *wb.offset(6 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        HEADER_VERSION as uint16_t,
    );
    archive_be64enc(
        &mut *wb.offset(8 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        (*xar).toc.length,
    );
    archive_be64enc(
        &mut *wb.offset(16 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        (*xar).toc.size,
    );
    archive_be32enc(
        &mut *wb.offset(24 as libc::c_int as isize) as *mut libc::c_uchar as *mut libc::c_void,
        (*xar).toc.a_sum.alg as uint32_t,
    );
    (*xar).wbuff_remaining = ((*xar).wbuff_remaining as libc::c_ulong)
        .wrapping_sub(HEADER_SIZE as libc::c_ulong) as size_t
        as size_t;
    /*
     * Write the TOC
     */
    r = copy_out(a, (*xar).toc.temp_offset, (*xar).toc.length);
    if r != ARCHIVE_OK {
        return r;
    }
    /* Write the checksum value of the TOC. */
    if (*xar).toc.a_sum.len != 0 {
        if (*xar).wbuff_remaining < (*xar).toc.a_sum.len {
            r = flush_wbuff(a);
            if r != ARCHIVE_OK {
                return r;
            }
        }
        wb = (*xar).wbuff.as_mut_ptr().offset(
            (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
                .wrapping_sub((*xar).wbuff_remaining) as isize,
        );
        memcpy(
            wb as *mut libc::c_void,
            (*xar).toc.a_sum.val.as_mut_ptr() as *const libc::c_void,
            (*xar).toc.a_sum.len,
        );
        (*xar).wbuff_remaining = ((*xar).wbuff_remaining as libc::c_ulong)
            .wrapping_sub((*xar).toc.a_sum.len) as size_t as size_t
    }
    /*
     * Write all file extended attributes and contents.
     */
    r = copy_out(a, (*xar).toc.a_sum.len, length);
    if r != ARCHIVE_OK {
        return r;
    }
    r = flush_wbuff(a);
    return r;
}
unsafe extern "C" fn xar_free(mut a: *mut archive_write) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    xar = (*a).format_data as *mut xar;
    /* Close the temporary file. */
    if (*xar).temp_fd >= 0 as libc::c_int {
        close((*xar).temp_fd);
    }
    archive_string_free(&mut (*xar).cur_dirstr);
    archive_string_free(&mut (*xar).tstr);
    archive_string_free(&mut (*xar).vstr);
    file_free_hardlinks(xar);
    file_free_register(xar);
    compression_end(&mut (*a).archive, &mut (*xar).stream);
    free(xar as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut f1: *const file = n1 as *const file;
    let mut f2: *const file = n2 as *const file;
    return strcmp((*f1).basename.s, (*f2).basename.s);
}
unsafe extern "C" fn file_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut f: *const file = n as *const file;
    return strcmp((*f).basename.s, key as *const libc::c_char);
}
unsafe extern "C" fn file_new(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> *mut file {
    let mut file: *mut file = 0 as *mut file;
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    file_cmp_node
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    file_cmp_key
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    file = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<file>() as libc::c_ulong,
    ) as *mut file;
    if file.is_null() {
        return 0 as *mut file;
    }
    if !entry.is_null() {
        (*file).entry = archive_entry_clone(entry)
    } else {
        (*file).entry = archive_entry_new2(&mut (*a).archive)
    }
    if (*file).entry.is_null() {
        free(file as *mut libc::c_void);
        return 0 as *mut file;
    }
    __archive_rb_tree_init(&mut (*file).rbtree, &rb_ops);
    (*file).children.first = NULL as *mut file;
    (*file).children.last = &mut (*file).children.first;
    (*file).xattr.first = NULL as *mut heap_data;
    (*file).xattr.last = &mut (*file).xattr.first;
    (*file).parentdir.s = NULL as *mut libc::c_char;
    (*file).parentdir.length = 0 as libc::c_int as size_t;
    (*file).parentdir.buffer_length = 0 as libc::c_int as size_t;
    (*file).basename.s = NULL as *mut libc::c_char;
    (*file).basename.length = 0 as libc::c_int as size_t;
    (*file).basename.buffer_length = 0 as libc::c_int as size_t;
    (*file).symlink.s = NULL as *mut libc::c_char;
    (*file).symlink.length = 0 as libc::c_int as size_t;
    (*file).symlink.buffer_length = 0 as libc::c_int as size_t;
    (*file).script.s = NULL as *mut libc::c_char;
    (*file).script.length = 0 as libc::c_int as size_t;
    (*file).script.buffer_length = 0 as libc::c_int as size_t;
    if !entry.is_null() && archive_entry_filetype(entry) == AE_IFDIR as mode_t {
        (*file).set_dir(1 as libc::c_int)
    }
    return file;
}
unsafe extern "C" fn file_free(mut file: *mut file) {
    let mut heap: *mut heap_data = 0 as *mut heap_data;
    let mut next_heap: *mut heap_data = 0 as *mut heap_data;
    heap = (*file).xattr.first;
    while !heap.is_null() {
        next_heap = (*heap).next;
        free(heap as *mut libc::c_void);
        heap = next_heap
    }
    archive_string_free(&mut (*file).parentdir);
    archive_string_free(&mut (*file).basename);
    archive_string_free(&mut (*file).symlink);
    archive_string_free(&mut (*file).script);
    archive_entry_free((*file).entry);
    free(file as *mut libc::c_void);
}
unsafe extern "C" fn file_create_virtual_dir(
    mut a: *mut archive_write,
    mut xar: *mut xar,
    mut pathname: *const libc::c_char,
) -> *mut file {
    let mut file: *mut file = 0 as *mut file;
    /* UNUSED */
    file = file_new(a, NULL as *mut archive_entry);
    if file.is_null() {
        return 0 as *mut file;
    }
    archive_entry_set_pathname((*file).entry, pathname);
    archive_entry_set_mode(
        (*file).entry,
        0o555 as libc::c_int as libc::c_uint | AE_IFDIR as mode_t,
    );
    (*file).set_dir(1 as libc::c_int);
    (*file).set_virtual_0(1 as libc::c_int);
    return file;
}
unsafe extern "C" fn file_add_child_tail(
    mut parent: *mut file,
    mut child: *mut file,
) -> libc::c_int {
    if __archive_rb_tree_insert_node(&mut (*parent).rbtree, child as *mut archive_rb_node) == 0 {
        return 0 as libc::c_int;
    }
    (*child).chnext = NULL as *mut file;
    *(*parent).children.last = child;
    (*parent).children.last = &mut (*child).chnext;
    (*child).parent = parent;
    return 1 as libc::c_int;
}
/*
 * Find a entry from `parent'
 */
unsafe extern "C" fn file_find_child(
    mut parent: *mut file,
    mut child_name: *const libc::c_char,
) -> *mut file {
    let mut np: *mut file = 0 as *mut file;
    np = __archive_rb_tree_find_node(&mut (*parent).rbtree, child_name as *const libc::c_void)
        as *mut file;
    return np;
}
/* nop */
/*
 * Generate a parent directory name and a base name from a pathname.
 */
unsafe extern "C" fn file_gen_utility_names(
    mut a: *mut archive_write,
    mut file: *mut file,
) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut pp: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut r: libc::c_int = ARCHIVE_OK;
    xar = (*a).format_data as *mut xar;
    (*file).parentdir.length = 0 as libc::c_int as size_t;
    (*file).basename.length = 0 as libc::c_int as size_t;
    (*file).symlink.length = 0 as libc::c_int as size_t;
    if (*file).parent == file {
        /* virtual root */
        return 0 as libc::c_int;
    }
    if _archive_entry_pathname_l((*file).entry, &mut pp, &mut len, (*xar).sconv) != 0 as libc::c_int
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
            b"Can\'t translate pathname \'%s\' to UTF-8\x00" as *const u8 as *const libc::c_char,
            archive_entry_pathname((*file).entry),
        );
        r = ARCHIVE_WARN
    }
    (*file).parentdir.length = 0 as libc::c_int as size_t;
    archive_strncat(&mut (*file).parentdir, pp as *const libc::c_void, len);
    len = (*file).parentdir.length;
    dirname = (*file).parentdir.s;
    p = dirname;
    /*
     * Convert a path-separator from '\' to  '/'
     */
    /*
     * Remove leading '/', '../' and './' elements
     */
    while *p != 0 {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            p = p.offset(1);
            len = len.wrapping_sub(1)
        } else {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 {
                break;
            }
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                p = p.offset(3 as libc::c_int as isize);
                len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                    && *p.offset(2 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                p = p.offset(2 as libc::c_int as isize);
                len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            } else {
                if !(*p.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32) {
                    break;
                }
                p = p.offset(1);
                len = len.wrapping_sub(1)
            }
        }
    }
    if p != dirname {
        memmove(
            dirname as *mut libc::c_void,
            p as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        p = dirname
    }
    /*
     * Remove "/","/." and "/.." elements from tail.
     */
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut ll: size_t = len;
        if *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
            == '/' as i32
        {
            *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char;
            len = len.wrapping_sub(1)
        }
        if len > 1 as libc::c_int as libc::c_ulong
            && *p.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '/' as i32
            && *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '.' as i32
        {
            *p.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char;
            len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        if len > 2 as libc::c_int as libc::c_ulong
            && *p.offset(len.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '/' as i32
            && *p.offset(len.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '.' as i32
            && *p.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                == '.' as i32
        {
            *p.offset(len.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char;
            len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
        if ll == len {
            break;
        }
    }
    while *p != 0 {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                /* Convert '//' --> '/' */
                memmove(
                    p as *mut libc::c_void,
                    p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    strlen(p.offset(1 as libc::c_int as isize))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                /* Convert '/./' --> '/' */
                memmove(
                    p as *mut libc::c_void,
                    p.offset(2 as libc::c_int as isize) as *const libc::c_void,
                    strlen(p.offset(2 as libc::c_int as isize))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(3 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                /* Convert 'dir/dir1/../dir2/'
                 *     --> 'dir/dir2/'
                 */
                let mut rp: *mut libc::c_char = p.offset(-(1 as libc::c_int as isize));
                while rp >= dirname {
                    if *rp as libc::c_int == '/' as i32 {
                        break;
                    }
                    rp = rp.offset(-1)
                }
                if rp > dirname {
                    strcpy(rp, p.offset(3 as libc::c_int as isize));
                    p = rp
                } else {
                    strcpy(dirname, p.offset(4 as libc::c_int as isize));
                    p = dirname
                }
            } else {
                p = p.offset(1)
            }
        } else {
            p = p.offset(1)
        }
    }
    p = dirname;
    len = strlen(p);
    if archive_entry_filetype((*file).entry) == AE_IFLNK as mode_t {
        let mut len2: size_t = 0;
        /* Convert symlink name too. */
        if _archive_entry_symlink_l((*file).entry, &mut pp, &mut len2, (*xar).sconv)
            != 0 as libc::c_int
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
                b"Can\'t translate symlink \'%s\' to UTF-8\x00" as *const u8 as *const libc::c_char,
                archive_entry_symlink((*file).entry),
            );
            r = ARCHIVE_WARN
        }
        (*file).symlink.length = 0 as libc::c_int as size_t;
        archive_strncat(&mut (*file).symlink, pp as *const libc::c_void, len2);
    }
    /*
     * - Count up directory elements.
     * - Find out the position which points the last position of
     *   path separator('/').
     */
    slash = NULL as *mut libc::c_char;
    while *p as libc::c_int != '\u{0}' as i32 {
        if *p as libc::c_int == '/' as i32 {
            slash = p
        }
        p = p.offset(1)
    }
    if slash.is_null() {
        /* The pathname doesn't have a parent directory. */
        (*file).parentdir.length = len;
        (*file).basename.length = 0 as libc::c_int as size_t;
        archive_string_concat(&mut (*file).basename, &mut (*file).parentdir);
        (*file).parentdir.length = 0 as libc::c_int as size_t;
        *(*file).parentdir.s = '\u{0}' as i32 as libc::c_char;
        return r;
    }
    /* Make a basename from dirname and slash */
    *slash = '\u{0}' as i32 as libc::c_char;
    (*file).parentdir.length = slash.offset_from(dirname) as libc::c_long as size_t;
    (*file).basename.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*file).basename,
        slash.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (if slash.offset(1 as libc::c_int as isize).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(slash.offset(1 as libc::c_int as isize))
        }),
    );
    return r;
}
unsafe extern "C" fn get_path_component(
    mut name: *mut libc::c_char,
    mut n: libc::c_int,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    p = strchr(fn_0, '/' as i32);
    if p.is_null() {
        l = strlen(fn_0) as libc::c_int;
        if l == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    } else {
        l = p.offset_from(fn_0) as libc::c_long as libc::c_int
    }
    if l > n - 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    memcpy(
        name as *mut libc::c_void,
        fn_0 as *const libc::c_void,
        l as libc::c_ulong,
    );
    *name.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
    return l;
}
/*
 * Add a new entry into the tree.
 */
unsafe extern "C" fn file_tree(
    mut a: *mut archive_write,
    mut filepp: *mut *mut file,
) -> libc::c_int {
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut xar: *mut xar = (*a).format_data as *mut xar;
    let mut dent: *mut file = 0 as *mut file;
    let mut file: *mut file = 0 as *mut file;
    let mut np: *mut file = 0 as *mut file;
    let mut ent: *mut archive_entry = 0 as *mut archive_entry;
    let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut l: libc::c_int = 0;
    file = *filepp;
    dent = (*xar).root;
    if (*file).parentdir.length > 0 as libc::c_int as libc::c_ulong {
        p = (*file).parentdir.s;
        fn_0 = p
    } else {
        p = b"\x00" as *const u8 as *const libc::c_char;
        fn_0 = p
    }
    /*
     * If the path of the parent directory of `file' entry is
     * the same as the path of `cur_dirent', add isoent to
     * `cur_dirent'.
     */
    if (*xar).cur_dirstr.length == (*file).parentdir.length
        && strcmp((*xar).cur_dirstr.s, fn_0) == 0 as libc::c_int
    {
        if file_add_child_tail((*xar).cur_dirent, file) == 0 {
            np = __archive_rb_tree_find_node(
                &mut (*(*xar).cur_dirent).rbtree,
                (*file).basename.s as *const libc::c_void,
            ) as *mut file
        } else {
            return 0 as libc::c_int;
        }
    } else {
        loop {
            l = get_path_component(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                fn_0,
            );
            if l == 0 as libc::c_int {
                np = NULL as *mut file;
                break;
            } else {
                if l < 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"A name buffer is too small\x00" as *const u8 as *const libc::c_char,
                    );
                    file_free(file);
                    *filepp = NULL as *mut file;
                    return -(30 as libc::c_int);
                }
                np = file_find_child(dent, name.as_mut_ptr());
                if np.is_null()
                    || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                {
                    break;
                }
                /* Find next subdirectory. */
                if (*np).dir() == 0 {
                    /* NOT Directory! */
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"`%s\' is not directory, we cannot insert `%s\' \x00" as *const u8
                            as *const libc::c_char,
                        archive_entry_pathname((*np).entry),
                        archive_entry_pathname((*file).entry),
                    );
                    file_free(file);
                    *filepp = NULL as *mut file;
                    return -(25 as libc::c_int);
                }
                fn_0 = fn_0.offset(l as isize);
                if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    fn_0 = fn_0.offset(1)
                }
                dent = np
            }
        }
        if np.is_null() {
            /*
             * Create virtual parent directories.
             */
            while *fn_0.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
                let mut vp: *mut file = 0 as *mut file;
                let mut as_0: archive_string = archive_string {
                    s: 0 as *mut libc::c_char,
                    length: 0,
                    buffer_length: 0,
                };
                as_0.s = NULL as *mut libc::c_char;
                as_0.length = 0 as libc::c_int as size_t;
                as_0.buffer_length = 0 as libc::c_int as size_t;
                archive_strncat(
                    &mut as_0,
                    p as *const libc::c_void,
                    (fn_0.offset_from(p) as libc::c_long + l as libc::c_long) as size_t,
                );
                if *as_0
                    .s
                    .offset(as_0.length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '/' as i32
                {
                    *as_0.s.offset(
                        as_0.length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize
                    ) = '\u{0}' as i32 as libc::c_char;
                    as_0.length = as_0.length.wrapping_sub(1)
                }
                vp = file_create_virtual_dir(a, xar, as_0.s);
                if vp.is_null() {
                    archive_string_free(&mut as_0);
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
                    );
                    file_free(file);
                    *filepp = NULL as *mut file;
                    return -(30 as libc::c_int);
                }
                archive_string_free(&mut as_0);
                if file_gen_utility_names(a, vp) <= ARCHIVE_FAILED {
                    return -(30 as libc::c_int);
                }
                file_add_child_tail(dent, vp);
                file_register(xar, vp);
                np = vp;
                fn_0 = fn_0.offset(l as isize);
                if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    fn_0 = fn_0.offset(1)
                }
                l = get_path_component(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                    fn_0,
                );
                if l < 0 as libc::c_int {
                    archive_string_free(&mut as_0);
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"A name buffer is too small\x00" as *const u8 as *const libc::c_char,
                    );
                    file_free(file);
                    *filepp = NULL as *mut file;
                    return -(30 as libc::c_int);
                }
                dent = np
            }
            /* Found out the parent directory where isoent can be
             * inserted. */
            (*xar).cur_dirent = dent;
            (*xar).cur_dirstr.length = 0 as libc::c_int as size_t;
            archive_string_ensure(
                &mut (*xar).cur_dirstr,
                (*dent)
                    .parentdir
                    .length
                    .wrapping_add((*dent).basename.length)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong),
            );
            if (*dent)
                .parentdir
                .length
                .wrapping_add((*dent).basename.length)
                == 0 as libc::c_int as libc::c_ulong
            {
                *(*xar).cur_dirstr.s.offset(0 as libc::c_int as isize) =
                    0 as libc::c_int as libc::c_char
            } else {
                if (*dent).parentdir.length > 0 as libc::c_int as libc::c_ulong {
                    (*xar).cur_dirstr.length = 0 as libc::c_int as size_t;
                    archive_string_concat(&mut (*xar).cur_dirstr, &mut (*dent).parentdir);
                    archive_strappend_char(&mut (*xar).cur_dirstr, '/' as i32 as libc::c_char);
                }
                archive_string_concat(&mut (*xar).cur_dirstr, &mut (*dent).basename);
            }
            if file_add_child_tail(dent, file) == 0 {
                np = __archive_rb_tree_find_node(
                    &mut (*dent).rbtree,
                    (*file).basename.s as *const libc::c_void,
                ) as *mut file
            } else {
                return 0 as libc::c_int;
            }
        }
    }
    /*
     * We have already has the entry the filename of which is
     * the same.
     */
    if archive_entry_filetype((*np).entry) != archive_entry_filetype((*file).entry) {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Found duplicate entries `%s\' and its file type is different\x00" as *const u8
                as *const libc::c_char,
            archive_entry_pathname((*np).entry),
        );
        file_free(file);
        *filepp = NULL as *mut file;
        return -(25 as libc::c_int);
    }
    /* Swap files. */
    ent = (*np).entry;
    (*np).entry = (*file).entry;
    (*file).entry = ent;
    (*np).set_virtual_0(0 as libc::c_int);
    file_free(file);
    *filepp = np;
    return 0 as libc::c_int;
}
unsafe extern "C" fn file_register(mut xar: *mut xar, mut file: *mut file) {
    let fresh3 = (*xar).file_idx;
    (*xar).file_idx = (*xar).file_idx + 1;
    (*file).id = fresh3;
    (*file).next = NULL as *mut file;
    *(*xar).file_list.last = file;
    (*xar).file_list.last = &mut (*file).next;
}
unsafe extern "C" fn file_init_register(mut xar: *mut xar) {
    (*xar).file_list.first = NULL as *mut file;
    (*xar).file_list.last = &mut (*xar).file_list.first;
}
unsafe extern "C" fn file_free_register(mut xar: *mut xar) {
    let mut file: *mut file = 0 as *mut file;
    let mut file_next: *mut file = 0 as *mut file;
    file = (*xar).file_list.first;
    while !file.is_null() {
        file_next = (*file).next;
        file_free(file);
        file = file_next
    }
}
/*
 * Register entry to get a hardlink target.
 */
unsafe extern "C" fn file_register_hardlink(
    mut a: *mut archive_write,
    mut file: *mut file,
) -> libc::c_int {
    let mut xar: *mut xar = (*a).format_data as *mut xar;
    let mut hl: *mut hardlink = 0 as *mut hardlink;
    let mut pathname: *const libc::c_char = 0 as *const libc::c_char;
    archive_entry_set_nlink((*file).entry, 1 as libc::c_int as libc::c_uint);
    pathname = archive_entry_hardlink((*file).entry);
    if pathname.is_null() {
        /* This `file` is a hardlink target. */
        hl = malloc(::std::mem::size_of::<hardlink>() as libc::c_ulong) as *mut hardlink;
        if hl.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        (*hl).nlink = 1 as libc::c_int;
        /* A hardlink target must be the first position. */
        (*file).hlnext = NULL as *mut file;
        (*hl).file_list.first = file;
        (*hl).file_list.last = &mut (*file).hlnext;
        __archive_rb_tree_insert_node(&mut (*xar).hardlink_rbtree, hl as *mut archive_rb_node);
    } else {
        hl = __archive_rb_tree_find_node(
            &mut (*xar).hardlink_rbtree,
            pathname as *const libc::c_void,
        ) as *mut hardlink;
        if !hl.is_null() {
            /* Insert `file` entry into the tail. */
            (*file).hlnext = NULL as *mut file;
            *(*hl).file_list.last = file;
            (*hl).file_list.last = &mut (*file).hlnext;
            (*hl).nlink += 1
        }
        archive_entry_unset_size((*file).entry);
    }
    return 0 as libc::c_int;
}
/*
 * Hardlinked files have to have the same location of extent.
 * We have to find out hardlink target entries for entries which
 * have a hardlink target name.
 */
unsafe extern "C" fn file_connect_hardlink_files(mut xar: *mut xar) {
    let mut n: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut hl: *mut hardlink = 0 as *mut hardlink;
    let mut target: *mut file = 0 as *mut file;
    let mut nf: *mut file = 0 as *mut file;
    n = __archive_rb_tree_iterate(
        &mut (*xar).hardlink_rbtree,
        NULL as *mut archive_rb_node,
        ARCHIVE_RB_DIR_LEFT as libc::c_uint,
    );
    while !n.is_null() {
        hl = n as *mut hardlink;
        /* The first entry must be a hardlink target. */
        target = (*hl).file_list.first;
        archive_entry_set_nlink((*target).entry, (*hl).nlink as libc::c_uint);
        if (*hl).nlink > 1 as libc::c_int {
            /* It means this file is a hardlink
             * target itself. */
            (*target).hardlink_target = target
        }
        nf = (*target).hlnext;
        while !nf.is_null() {
            (*nf).hardlink_target = target;
            archive_entry_set_nlink((*nf).entry, (*hl).nlink as libc::c_uint);
            nf = (*nf).hlnext
        }
        n = __archive_rb_tree_iterate(
            &mut (*xar).hardlink_rbtree,
            n,
            ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
        )
    }
}
unsafe extern "C" fn file_hd_cmp_node(
    mut n1: *const archive_rb_node,
    mut n2: *const archive_rb_node,
) -> libc::c_int {
    let mut h1: *const hardlink = n1 as *const hardlink;
    let mut h2: *const hardlink = n2 as *const hardlink;
    return strcmp(
        archive_entry_pathname((*(*h1).file_list.first).entry),
        archive_entry_pathname((*(*h2).file_list.first).entry),
    );
}
unsafe extern "C" fn file_hd_cmp_key(
    mut n: *const archive_rb_node,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut h: *const hardlink = n as *const hardlink;
    return strcmp(
        archive_entry_pathname((*(*h).file_list.first).entry),
        key as *const libc::c_char,
    );
}
unsafe extern "C" fn file_init_hardlinks(mut xar: *mut xar) {
    static mut rb_ops: archive_rb_tree_ops = unsafe {
        {
            let mut init = archive_rb_tree_ops {
                rbto_compare_nodes: Some(
                    file_hd_cmp_node
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const archive_rb_node,
                        ) -> libc::c_int,
                ),
                rbto_compare_key: Some(
                    file_hd_cmp_key
                        as unsafe extern "C" fn(
                            _: *const archive_rb_node,
                            _: *const libc::c_void,
                        ) -> libc::c_int,
                ),
            };
            init
        }
    };
    __archive_rb_tree_init(&mut (*xar).hardlink_rbtree, &rb_ops);
}
unsafe extern "C" fn file_free_hardlinks(mut xar: *mut xar) {
    let mut n: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut tmp: *mut archive_rb_node = 0 as *mut archive_rb_node;
    n = __archive_rb_tree_iterate(
        &mut (*xar).hardlink_rbtree,
        NULL as *mut archive_rb_node,
        ARCHIVE_RB_DIR_LEFT as libc::c_uint,
    );
    while !n.is_null() && {
        tmp = __archive_rb_tree_iterate(
            &mut (*xar).hardlink_rbtree,
            n,
            ARCHIVE_RB_DIR_RIGHT as libc::c_uint,
        );
        (1 as libc::c_int) != 0
    } {
        __archive_rb_tree_remove_node(&mut (*xar).hardlink_rbtree, n);
        free(n as *mut libc::c_void);
        n = tmp
    }
}
unsafe extern "C" fn checksum_init(mut sumwrk: *mut chksumwork, mut sum_alg: sumalg) {
    (*sumwrk).alg = sum_alg;
    match sum_alg as libc::c_uint {
        1 => {
            __archive_digest
                .sha1init
                .expect("non-null function pointer")(&mut (*sumwrk).sha1ctx);
        }
        2 => {
            __archive_digest.md5init.expect("non-null function pointer")(&mut (*sumwrk).md5ctx);
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn checksum_update(
    mut sumwrk: *mut chksumwork,
    mut buff: *const libc::c_void,
    mut size: size_t,
) {
    match (*sumwrk).alg as libc::c_uint {
        1 => {
            __archive_digest
                .sha1update
                .expect("non-null function pointer")(&mut (*sumwrk).sha1ctx, buff, size);
        }
        2 => {
            __archive_digest
                .md5update
                .expect("non-null function pointer")(&mut (*sumwrk).md5ctx, buff, size);
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn checksum_final(mut sumwrk: *mut chksumwork, mut sumval: *mut chksumval) {
    match (*sumwrk).alg as libc::c_uint {
        0 => (*sumval).len = 0 as libc::c_int as size_t,
        1 => {
            __archive_digest
                .sha1final
                .expect("non-null function pointer")(
                &mut (*sumwrk).sha1ctx,
                (*sumval).val.as_mut_ptr() as *mut libc::c_void,
            );
            (*sumval).len = SHA1_SIZE as size_t
        }
        2 => {
            __archive_digest
                .md5final
                .expect("non-null function pointer")(
                &mut (*sumwrk).md5ctx,
                (*sumval).val.as_mut_ptr() as *mut libc::c_void,
            );
            (*sumval).len = MD5_SIZE as size_t
        }
        _ => {}
    }
    (*sumval).alg = (*sumwrk).alg;
}
unsafe extern "C" fn compression_unsupported_encoder(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut name: *const libc::c_char,
) -> libc::c_int {
    archive_set_error(
        a,
        ARCHIVE_ERRNO_MISC,
        b"%s compression not supported on this platform\x00" as *const u8 as *const libc::c_char,
        name,
    );
    (*lastrm).valid = 0 as libc::c_int;
    (*lastrm).real_stream = NULL as *mut libc::c_void;
    return -(25 as libc::c_int);
}
unsafe extern "C" fn compression_init_encoder_gzip(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
    mut withheader: libc::c_int,
) -> libc::c_int {
    let mut strm: *mut z_stream = 0 as *mut z_stream;
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    strm = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong,
    ) as *mut z_stream;
    if strm.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"Can\'t allocate memory for gzip stream\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* zlib.h is not const-correct, so we need this one bit
     * of ugly hackery to convert a const * pointer to
     * a non-const pointer. */
    (*strm).next_in = (*lastrm).next_in as *const libc::c_void as uintptr_t as *mut Bytef;
    (*strm).avail_in = (*lastrm).avail_in as uInt;
    (*strm).total_in = (*lastrm).total_in;
    (*strm).next_out = (*lastrm).next_out;
    (*strm).avail_out = (*lastrm).avail_out as uInt;
    (*strm).total_out = (*lastrm).total_out;
    if deflateInit2_(
        strm,
        level,
        8 as libc::c_int,
        (if withheader != 0 {
            15 as libc::c_int
        } else {
            -(15 as libc::c_int)
        }),
        8 as libc::c_int,
        0 as libc::c_int,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    ) != Z_OK
    {
        free(strm as *mut libc::c_void);
        (*lastrm).real_stream = NULL as *mut libc::c_void;
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Internal error initializing compression library\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*lastrm).real_stream = strm as *mut libc::c_void;
    (*lastrm).valid = 1 as libc::c_int;
    (*lastrm).code = Some(
        compression_code_gzip
            as unsafe extern "C" fn(
                _: *mut archive,
                _: *mut la_zstream,
                _: la_zaction,
            ) -> libc::c_int,
    );
    (*lastrm).end = Some(
        compression_end_gzip
            as unsafe extern "C" fn(_: *mut archive, _: *mut la_zstream) -> libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_code_gzip(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut action: la_zaction,
) -> libc::c_int {
    let mut strm: *mut z_stream = 0 as *mut z_stream;
    let mut r: libc::c_int = 0;
    strm = (*lastrm).real_stream as *mut z_stream;
    /* zlib.h is not const-correct, so we need this one bit
     * of ugly hackery to convert a const * pointer to
     * a non-const pointer. */
    (*strm).next_in = (*lastrm).next_in as *const libc::c_void as uintptr_t as *mut Bytef;
    (*strm).avail_in = (*lastrm).avail_in as uInt;
    (*strm).total_in = (*lastrm).total_in;
    (*strm).next_out = (*lastrm).next_out;
    (*strm).avail_out = (*lastrm).avail_out as uInt;
    (*strm).total_out = (*lastrm).total_out;
    r = deflate(
        strm,
        if action as libc::c_uint == ARCHIVE_Z_FINISH as libc::c_int as libc::c_uint {
            Z_FINISH
        } else {
            Z_NO_FLUSH
        },
    );
    (*lastrm).next_in = (*strm).next_in;
    (*lastrm).avail_in = (*strm).avail_in as size_t;
    (*lastrm).total_in = (*strm).total_in;
    (*lastrm).next_out = (*strm).next_out;
    (*lastrm).avail_out = (*strm).avail_out as size_t;
    (*lastrm).total_out = (*strm).total_out;
    match r {
        Z_OK => return 0 as libc::c_int,
        Z_STREAM_END => return 1 as libc::c_int,
        _ => {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_MISC,
                b"GZip compression failed: deflate() call returned status %d\x00" as *const u8
                    as *const libc::c_char,
                r,
            );
            return -(30 as libc::c_int);
        }
    };
}
unsafe extern "C" fn compression_end_gzip(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
) -> libc::c_int {
    let mut strm: *mut z_stream = 0 as *mut z_stream;
    let mut r: libc::c_int = 0;
    strm = (*lastrm).real_stream as *mut z_stream;
    r = deflateEnd(strm);
    free(strm as *mut libc::c_void);
    (*lastrm).real_stream = NULL as *mut libc::c_void;
    (*lastrm).valid = 0 as libc::c_int;
    if r != Z_OK {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_MISC,
            b"Failed to clean up compressor\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_init_encoder_bzip2(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    return compression_unsupported_encoder(
        a,
        lastrm,
        b"bzip2\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn compression_init_encoder_lzma(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    return compression_unsupported_encoder(
        a,
        lastrm,
        b"lzma\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn compression_init_encoder_xz(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut level: libc::c_int,
    mut threads: libc::c_int,
) -> libc::c_int {
    /* UNUSED */
    if (*lastrm).valid != 0 {
        compression_end(a, lastrm);
    }
    return compression_unsupported_encoder(
        a,
        lastrm,
        b"xz\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn xar_compression_init_encoder(mut a: *mut archive_write) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut r: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    match (*xar).opt_compression as libc::c_uint {
        1 => {
            r = compression_init_encoder_gzip(
                &mut (*a).archive,
                &mut (*xar).stream,
                (*xar).opt_compression_level,
                1 as libc::c_int,
            )
        }
        2 => {
            r = compression_init_encoder_bzip2(
                &mut (*a).archive,
                &mut (*xar).stream,
                (*xar).opt_compression_level,
            )
        }
        3 => {
            r = compression_init_encoder_lzma(
                &mut (*a).archive,
                &mut (*xar).stream,
                (*xar).opt_compression_level,
            )
        }
        4 => {
            r = compression_init_encoder_xz(
                &mut (*a).archive,
                &mut (*xar).stream,
                (*xar).opt_compression_level,
                (*xar).opt_threads as libc::c_int,
            )
        }
        _ => r = ARCHIVE_OK,
    }
    if r == ARCHIVE_OK {
        (*xar).stream.total_in = 0 as libc::c_int as uint64_t;
        (*xar).stream.next_out = (*xar).wbuff.as_mut_ptr();
        (*xar).stream.avail_out = ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong;
        (*xar).stream.total_out = 0 as libc::c_int as uint64_t
    }
    return r;
}
unsafe extern "C" fn compression_code(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
    mut action: la_zaction,
) -> libc::c_int {
    if (*lastrm).valid != 0 {
        return (*lastrm).code.expect("non-null function pointer")(a, lastrm, action);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compression_end(
    mut a: *mut archive,
    mut lastrm: *mut la_zstream,
) -> libc::c_int {
    if (*lastrm).valid != 0 {
        return (*lastrm).end.expect("non-null function pointer")(a, lastrm);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn save_xattrs(mut a: *mut archive_write, mut file: *mut file) -> libc::c_int {
    let mut xar: *mut xar = 0 as *mut xar;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *const libc::c_void = 0 as *const libc::c_void;
    let mut heap: *mut heap_data = 0 as *mut heap_data;
    let mut size: size_t = 0;
    let mut count: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    xar = (*a).format_data as *mut xar;
    count = archive_entry_xattr_reset((*file).entry);
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    loop
    /* Next xattr */
    {
        let fresh4 = count; /* save a extracted size */
        count = count - 1;
        if !(fresh4 != 0) {
            break;
        }
        archive_entry_xattr_next((*file).entry, &mut name, &mut value, &mut size);
        checksum_init(&mut (*xar).a_sumwrk, (*xar).opt_sumalg);
        checksum_init(&mut (*xar).e_sumwrk, (*xar).opt_sumalg);
        heap = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<heap_data>() as libc::c_ulong,
        ) as *mut heap_data;
        if heap.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for xattr\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        let fresh5 = (*file).ea_idx;
        (*file).ea_idx = (*file).ea_idx + 1;
        (*heap).id = fresh5;
        (*heap).temp_offset = (*xar).temp_offset;
        (*heap).size = size;
        (*heap).compression = (*xar).opt_compression;
        /* Get a extracted sumcheck value. */
        checksum_update(&mut (*xar).e_sumwrk, value, size);
        checksum_final(&mut (*xar).e_sumwrk, &mut (*heap).e_sum);
        /*
         * Not compression to xattr is simple way.
         */
        if (*heap).compression as libc::c_uint == NONE as libc::c_int as libc::c_uint {
            checksum_update(&mut (*xar).a_sumwrk, value, size);
            checksum_final(&mut (*xar).a_sumwrk, &mut (*heap).a_sum);
            if write_to_temp(a, value, size) != ARCHIVE_OK {
                free(heap as *mut libc::c_void);
                return -(30 as libc::c_int);
            }
            (*heap).length = size;
            /* Add heap to the tail of file->xattr. */
            (*heap).next = NULL as *mut heap_data;
            *(*file).xattr.last = heap;
            (*file).xattr.last = &mut (*heap).next
        } else {
            /*
             * Init compression library.
             */
            r = xar_compression_init_encoder(a);
            if r != ARCHIVE_OK {
                free(heap as *mut libc::c_void);
                return -(30 as libc::c_int);
            }
            (*xar).stream.next_in = value as *const libc::c_uchar;
            (*xar).stream.avail_in = size;
            loop {
                r = compression_code(&mut (*a).archive, &mut (*xar).stream, ARCHIVE_Z_FINISH);
                if r != ARCHIVE_OK && r != ARCHIVE_EOF {
                    free(heap as *mut libc::c_void);
                    return -(30 as libc::c_int);
                }
                size = (::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong)
                    .wrapping_sub((*xar).stream.avail_out);
                checksum_update(
                    &mut (*xar).a_sumwrk,
                    (*xar).wbuff.as_mut_ptr() as *const libc::c_void,
                    size,
                );
                if write_to_temp(a, (*xar).wbuff.as_mut_ptr() as *const libc::c_void, size)
                    != ARCHIVE_OK
                {
                    free(heap as *mut libc::c_void);
                    return -(30 as libc::c_int);
                }
                if r == ARCHIVE_OK {
                    (*xar).stream.next_out = (*xar).wbuff.as_mut_ptr();
                    (*xar).stream.avail_out =
                        ::std::mem::size_of::<[libc::c_uchar; 65536]>() as libc::c_ulong
                } else {
                    checksum_final(&mut (*xar).a_sumwrk, &mut (*heap).a_sum);
                    (*heap).length = (*xar).stream.total_out;
                    /* Add heap to the tail of file->xattr. */
                    (*heap).next = NULL as *mut heap_data;
                    *(*file).xattr.last = heap;
                    (*file).xattr.last = &mut (*heap).next;
                    break;
                }
            }
            /* Clean up compression library. */
            r = compression_end(&mut (*a).archive, &mut (*xar).stream);
            if r != ARCHIVE_OK {
                return -(30 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn getalgsize(mut sumalg: sumalg) -> libc::c_int {
    match sumalg as libc::c_uint {
        1 => return 20 as libc::c_int,
        2 => return 16 as libc::c_int,
        0 | _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn getalgname(mut sumalg: sumalg) -> *const libc::c_char {
    match sumalg as libc::c_uint {
        1 => return b"sha1\x00" as *const u8 as *const libc::c_char,
        2 => return b"md5\x00" as *const u8 as *const libc::c_char,
        0 | _ => return 0 as *const libc::c_char,
    };
}
/* Support xar format */
