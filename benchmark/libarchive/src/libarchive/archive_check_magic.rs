use ::libc;
extern "C" {
    /* Length of malloc-ed storage in bytes. */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_WRITE_MAGIC: libc::c_uint = 0xb0c5c0de as libc::c_uint;
pub const ARCHIVE_READ_MAGIC: libc::c_uint = 0xdeb0c5 as libc::c_uint;
pub const ARCHIVE_WRITE_DISK_MAGIC: libc::c_uint = 0xc001b0c5 as libc::c_uint;
pub const ARCHIVE_READ_DISK_MAGIC: libc::c_uint = 0xbadb0c5 as libc::c_uint;
pub const ARCHIVE_MATCH_MAGIC: libc::c_uint = 0xcad11c9 as libc::c_uint;
pub const ARCHIVE_STATE_NEW: libc::c_uint = 1 as libc::c_uint;
pub const ARCHIVE_STATE_HEADER: libc::c_uint = 2 as libc::c_uint;
pub const ARCHIVE_STATE_DATA: libc::c_uint = 4 as libc::c_uint;
pub const ARCHIVE_STATE_EOF: libc::c_uint = 0x10 as libc::c_uint;
pub const ARCHIVE_STATE_CLOSED: libc::c_uint = 0x20 as libc::c_uint;
pub const ARCHIVE_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
unsafe extern "C" fn errmsg(mut m: *const libc::c_char) {
    let mut s: size_t = strlen(m);
    let mut written: ssize_t = 0;
    while s > 0 as libc::c_int as libc::c_ulong {
        written = write(2 as libc::c_int, m as *const libc::c_void, strlen(m));
        if written <= 0 as libc::c_int as libc::c_long {
            return;
        }
        m = m.offset(written as isize);
        s = (s as libc::c_ulong).wrapping_sub(written as libc::c_ulong) as size_t as size_t
    }
}
unsafe extern "C" fn diediedie() -> ! {
    abort();
    /* Terminate the program abnormally. */
}
unsafe extern "C" fn state_name(mut s: libc::c_uint) -> *const libc::c_char {
    match s {
        ARCHIVE_STATE_NEW => return b"new\x00" as *const u8 as *const libc::c_char,
        ARCHIVE_STATE_HEADER => return b"header\x00" as *const u8 as *const libc::c_char,
        ARCHIVE_STATE_DATA => return b"data\x00" as *const u8 as *const libc::c_char,
        ARCHIVE_STATE_EOF => return b"eof\x00" as *const u8 as *const libc::c_char,
        ARCHIVE_STATE_CLOSED => return b"closed\x00" as *const u8 as *const libc::c_char,
        ARCHIVE_STATE_FATAL => return b"fatal\x00" as *const u8 as *const libc::c_char,
        _ => return b"??\x00" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn archive_handle_type_name(mut m: libc::c_uint) -> *const libc::c_char {
    match m {
        ARCHIVE_WRITE_MAGIC => return b"archive_write\x00" as *const u8 as *const libc::c_char,
        ARCHIVE_READ_MAGIC => return b"archive_read\x00" as *const u8 as *const libc::c_char,
        ARCHIVE_WRITE_DISK_MAGIC => {
            return b"archive_write_disk\x00" as *const u8 as *const libc::c_char
        }
        ARCHIVE_READ_DISK_MAGIC => {
            return b"archive_read_disk\x00" as *const u8 as *const libc::c_char
        }
        ARCHIVE_MATCH_MAGIC => return b"archive_match\x00" as *const u8 as *const libc::c_char,
        _ => return NULL as *const libc::c_char,
    };
}
unsafe extern "C" fn write_all_states(
    mut buff: *mut libc::c_char,
    mut states: libc::c_uint,
) -> *mut libc::c_char {
    let mut lowbit: libc::c_uint = 0;
    *buff.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    loop
    /* A trick for computing the lowest set bit. */
    {
        lowbit = states & (1 as libc::c_int as libc::c_uint).wrapping_add(!states); /* Clear the low bit. */
        if !(lowbit != 0 as libc::c_int as libc::c_uint) {
            break;
        }
        states &= !lowbit;
        strcat(buff, state_name(lowbit));
        if states != 0 as libc::c_int as libc::c_uint {
            strcat(buff, b"/\x00" as *const u8 as *const libc::c_char);
        }
    }
    return buff;
}
/* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
/*
 * Check magic value and current state.
 *   Magic value mismatches are fatal and result in calls to abort().
 *   State mismatches return ARCHIVE_FATAL.
 *   Otherwise, returns ARCHIVE_OK.
 *
 * This is designed to catch serious programming errors that violate
 * the libarchive API.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_check_magic(
    mut a: *mut archive,
    mut magic: libc::c_uint,
    mut state: libc::c_uint,
    mut function: *const libc::c_char,
) -> libc::c_int {
    let mut states1: [libc::c_char; 64] = [0; 64];
    let mut states2: [libc::c_char; 64] = [0; 64];
    let mut handle_type: *const libc::c_char = 0 as *const libc::c_char;
    /*
     * If this isn't some form of archive handle,
     * then the library user has screwed up so bad that
     * we don't even have a reliable way to report an error.
     */
    handle_type = archive_handle_type_name((*a).magic);
    if handle_type.is_null() {
        errmsg(b"PROGRAMMER ERROR: Function \x00" as *const u8 as *const libc::c_char);
        errmsg(function);
        errmsg(b" invoked with invalid archive handle.\n\x00" as *const u8 as *const libc::c_char);
        diediedie();
    }
    if (*a).magic != magic {
        archive_set_error(a, -(1 as libc::c_int),
                          b"PROGRAMMER ERROR: Function \'%s\' invoked on \'%s\' archive object, which is not supported.\x00"
                              as *const u8 as *const libc::c_char, function,
                          handle_type);
        (*a).state = ARCHIVE_STATE_FATAL;
        return -(30 as libc::c_int);
    }
    if (*a).state & state == 0 as libc::c_int as libc::c_uint {
        /* If we're already FATAL, don't overwrite the error. */
        if (*a).state != ARCHIVE_STATE_FATAL {
            archive_set_error(a, -(1 as libc::c_int),
                              b"INTERNAL ERROR: Function \'%s\' invoked with archive structure in state \'%s\', should be in state \'%s\'\x00"
                                  as *const u8 as *const libc::c_char,
                              function,
                              write_all_states(states1.as_mut_ptr(),
                                               (*a).state),
                              write_all_states(states2.as_mut_ptr(), state));
        }
        (*a).state = ARCHIVE_STATE_FATAL;
        return -(30 as libc::c_int);
    }
    return ARCHIVE_OK;
}
