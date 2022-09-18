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
    fn archive_read_support_filter_bzip2(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_compress(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_gzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_grzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_lrzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_lz4(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_lzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_lzma(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_lzop(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_rpm(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_uu(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_xz(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_filter_zstd(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
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
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
/* Operation was successful. */
/* Retry might succeed. */
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * The archive_read_support_XXX calls enable auto-detect for this
 * archive handle.  They also link in the necessary support code.
 * For example, if you don't want bzlib linked in, don't invoke
 * support_compression_bzip2().  The "all" functions provide the
 * obvious shorthand.
 */
/* Deprecated; remove in libarchive 4.0 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_compression_all(mut a: *mut archive) -> libc::c_int {
    return archive_read_support_filter_all(a);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_support_filter_all(mut a: *mut archive) -> libc::c_int {
    let mut magic_test: libc::c_int = __archive_check_magic(
        a,
        0xdeb0c5 as libc::c_uint,
        1 as libc::c_uint,
        b"archive_read_support_filter_all\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* Bzip falls back to "bunzip2" command-line */
    archive_read_support_filter_bzip2(a);
    /* The decompress code doesn't use an outside library. */
    archive_read_support_filter_compress(a);
    /* Gzip decompress falls back to "gzip -d" command-line. */
    archive_read_support_filter_gzip(a);
    /* Lzip falls back to "unlzip" command-line program. */
    archive_read_support_filter_lzip(a);
    /* The LZMA file format has a very weak signature, so it
     * may not be feasible to keep this here, but we'll try.
     * This will come back out if there are problems. */
    /* Lzma falls back to "unlzma" command-line program. */
    archive_read_support_filter_lzma(a);
    /* Xz falls back to "unxz" command-line program. */
    archive_read_support_filter_xz(a);
    /* The decode code doesn't use an outside library. */
    archive_read_support_filter_uu(a);
    /* The decode code doesn't use an outside library. */
    archive_read_support_filter_rpm(a);
    /* The decode code always uses "lrzip -q -d" command-line. */
    archive_read_support_filter_lrzip(a);
    /* Lzop decompress falls back to "lzop -d" command-line. */
    archive_read_support_filter_lzop(a);
    /* The decode code always uses "grzip -d" command-line. */
    archive_read_support_filter_grzip(a);
    /* Lz4 falls back to "lz4 -d" command-line program. */
    archive_read_support_filter_lz4(a);
    /* Zstd falls back to "zstd -d" command-line program. */
    archive_read_support_filter_zstd(a);
    /* Note: We always return ARCHIVE_OK here, even if some of the
     * above return ARCHIVE_WARN.  The intent here is to enable
     * "as much as possible."  Clients who need specific
     * compression should enable those individually so they can
     * verify the level of support. */
    /* Clear any warning messages set by the above functions. */
    archive_clear_error(a);
    return 0 as libc::c_int;
}
