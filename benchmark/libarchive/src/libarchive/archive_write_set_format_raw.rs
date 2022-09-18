use ::libc;
extern "C" {
    /* Length of malloc-ed storage in bytes. */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    /* A deprecated synonym for archive_write_open_filename() */
    /* _buffSize is the size of the buffer, _used refers to a variable that
     * will be updated after each write into the buffer. */
    /*
     * Note that the library will truncate writes beyond the size provided
     * to archive_write_header or pad if the provided data is short.
     */
    /* This interface is currently only available for archive_write_disk handles.  */
    /* Marks the archive as FATAL so that a subsequent free() operation
     * won't try to close() cleanly.  Provides a fast abort capability
     * when the client discovers that things have gone wrong. */
    /* This can fail if the archive wasn't already closed, in which case
     * archive_write_free() will implicitly call archive_write_close(). */
    /* Synonym for archive_write_free() for backwards compatibility. */
    /*
     * Set write options.
     */
    /* Apply option to the format only. */
    /* Apply option to the filter only. */
    /* Apply option to both the format and the filter. */
    /* Apply option string to both the format and the filter. */
    /*
     * Set a encryption passphrase.
     */
    /*-
     * ARCHIVE_WRITE_DISK API
     *
     * To create objects on disk:
     *   1) Ask archive_write_disk_new for a new archive_write_disk object.
     *   2) Set any global properties.  In particular, you probably
     *      want to set the options.
     *   3) For each entry:
     *      - construct an appropriate struct archive_entry structure
     *      - archive_write_header to create the file/dir/etc on disk
     *      - archive_write_data to write the entry data
     *   4) archive_write_free to cleanup the writer and release resources
     *
     * In particular, you can use this in conjunction with archive_read()
     * to pull entries out of an archive and create them on disk.
     */
    /* This file will not be overwritten. */
    /* Set flags to control how the next item gets created.
     * This accepts a bitmask of ARCHIVE_EXTRACT_XXX flags defined above. */
    /*
     * The lookup functions are given uname/uid (or gname/gid) pairs and
     * return a uid (gid) suitable for this system.  These are used for
     * restoring ownership and for setting ACLs.  The default functions
     * are naive, they just return the uid/gid.  These are small, so reasonable
     * for applications that don't need to preserve ownership; they
     * are probably also appropriate for applications that are doing
     * same-system backup and restore.
     */
    /*
     * The "standard" lookup functions use common system calls to lookup
     * the uname/gname, falling back to the uid/gid if the names can't be
     * found.  They cache lookups and are reasonably fast, but can be very
     * large, so they are not used unless you ask for them.  In
     * particular, these match the specifications of POSIX "pax" and old
     * POSIX "tar".
     */
    /*
     * If neither the default (naive) nor the standard (big) functions suit
     * your needs, you can write your own and register them.  Be sure to
     * include a cleanup function if you have allocated private data.
     */
    /* private_data */
    /* cleanup */
    /* private_data */
    /* cleanup */
    /*
     * ARCHIVE_READ_DISK API
     *
     * This is still evolving and somewhat experimental.
     */
    /* The names for symlink modes here correspond to an old BSD
     * command-line argument convention: -L, -P, -H */
    /* Follow all symlinks. */
    /* Follow no symlinks. */
    /* Follow symlink initially, then not. */
    /* TODO: Handle Linux stat32/stat64 ugliness. <sigh> */
    /* fd */
    /* Look up gname for gid or uname for uid. */
    /* Default implementations are very, very stupid. */
    /* "Standard" implementation uses getpwuid_r, getgrgid_r and caches the
     * results for performance. */
    /* You can install your own lookups if you like. */
    /* private_data */
    /* lookup_fn */
    /* cleanup_fn */
    /* private_data */
    /* lookup_fn */
    /* cleanup_fn */
    /* Start traversal. */
    /*
     * Request that current entry be visited.  If you invoke it on every
     * directory, you'll get a physical traversal.  This is ignored if the
     * current entry isn't a directory or a link to a directory.  So, if
     * you invoke this on every returned path, you'll get a full logical
     * traversal.
     */
    /* Request that the access time of the entry visited by traversal be restored. */
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
    /* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type __int64_t = libc::c_long;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type la_ssize_t = ssize_t;
/* Declare our basic types. */
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
/* Associated archive. */
/* Who I write to. */
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
pub struct raw {
    pub entries_written: libc::c_int,
}
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const ERANGE: libc::c_int = 34 as libc::c_int;
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_RAW: libc::c_int = 0x90000 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * Set output format to 'raw' format.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_raw(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut raw: *mut raw = 0 as *mut raw;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_raw\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If someone else was already registered, unregister them. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    raw = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<raw>() as libc::c_ulong,
    ) as *mut raw;
    if raw.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate raw data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*raw).entries_written = 0 as libc::c_int;
    (*a).format_data = raw as *mut libc::c_void;
    (*a).format_name = b"raw\x00" as *const u8 as *const libc::c_char;
    /* no options exist for this format */
    (*a).format_options = ::std::mem::transmute::<
        libc::intptr_t,
        Option<
            unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
        >,
    >(NULL as libc::intptr_t);
    (*a).format_write_header = Some(
        archive_write_raw_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        archive_write_raw_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    >(NULL as libc::intptr_t);
    /* nothing needs to be done on closing */
    (*a).format_close = ::std::mem::transmute::<
        libc::intptr_t,
        Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    >(NULL as libc::intptr_t);
    (*a).format_free =
        Some(archive_write_raw_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).archive.archive_format = ARCHIVE_FORMAT_RAW;
    (*a).archive.archive_format_name = b"RAW\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_raw_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut raw: *mut raw = (*a).format_data as *mut raw;
    if archive_entry_filetype(entry) != AE_IFREG as mode_t {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"Raw format only supports filetype AE_IFREG\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if (*raw).entries_written > 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ERANGE,
            b"Raw format only supports one entry per archive\x00" as *const u8
                as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*raw).entries_written += 1;
    return 0 as libc::c_int;
}
/*-
 * Copyright (c) 2013 Marek Kubica
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
unsafe extern "C" fn archive_write_raw_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut ret: libc::c_int = 0;
    ret = __archive_write_output(a, buff, s);
    if ret >= 0 as libc::c_int {
        return s as ssize_t;
    } else {
        return ret as ssize_t;
    };
}
unsafe extern "C" fn archive_write_raw_free(mut a: *mut archive_write) -> libc::c_int {
    let mut raw: *mut raw = 0 as *mut raw;
    raw = (*a).format_data as *mut raw;
    free(raw as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
