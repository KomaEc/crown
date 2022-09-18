use ::libc;
extern "C" {
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
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
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
    fn __archive_write_filters_free(_: *mut archive);
    #[no_mangle]
    fn __archive_write_allocate_filter(_: *mut archive) -> *mut archive_write_filter;
    #[no_mangle]
    fn __archive_write_filter(
        _: *mut archive_write_filter,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
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
pub struct private_data {
    pub pdata: *mut archive_write_program_data,
    pub description: archive_string,
    pub cmd: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write_program_data {
    pub child: pid_t,
    pub child_stdin: libc::c_int,
    pub child_stdout: libc::c_int,
    pub child_buf: *mut libc::c_char,
    pub child_buf_len: size_t,
    pub child_buf_avail: size_t,
    pub program_name: *mut libc::c_char,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const EIO: libc::c_int = 5 as libc::c_int;
pub const EAGAIN: libc::c_int = 11 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const EPIPE: libc::c_int = 32 as libc::c_int;
pub const F_SETFL: libc::c_int = 4 as libc::c_int;
pub const ARCHIVE_FILTER_PROGRAM: libc::c_int = 4 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_compression_program(
    mut a: *mut archive,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    __archive_write_filters_free(a);
    return archive_write_add_filter_program(a, cmd);
}
/*
 * Add a filter to this write handle that passes all data through an
 * external program.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_add_filter_program(
    mut _a: *mut archive,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    let mut f: *mut archive_write_filter = __archive_write_allocate_filter(_a);
    let mut data: *mut private_data = 0 as *mut private_data;
    static mut prefix: [libc::c_char; 10] =
        unsafe { *::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"Program: \x00") };
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_add_filter_program\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    (*f).data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<private_data>() as libc::c_ulong,
    );
    if !(*f).data.is_null() {
        data = (*f).data as *mut private_data;
        (*data).cmd = strdup(cmd);
        if !(*data).cmd.is_null() {
            (*data).pdata = __archive_write_program_allocate(cmd);
            if !(*data).pdata.is_null() {
                /* Make up a description string. */
                if !archive_string_ensure(
                    &mut (*data).description,
                    strlen(prefix.as_ptr())
                        .wrapping_add(strlen(cmd))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
                .is_null()
                {
                    (*data).description.length = 0 as libc::c_int as size_t;
                    archive_strncat(
                        &mut (*data).description,
                        prefix.as_ptr() as *const libc::c_void,
                        (if prefix.as_ptr().is_null() {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            strlen(prefix.as_ptr())
                        }),
                    );
                    archive_strcat(&mut (*data).description, cmd as *const libc::c_void);
                    (*f).name = (*data).description.s;
                    (*f).code = ARCHIVE_FILTER_PROGRAM;
                    (*f).open = Some(
                        archive_compressor_program_open
                            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
                    );
                    (*f).write = Some(
                        archive_compressor_program_write
                            as unsafe extern "C" fn(
                                _: *mut archive_write_filter,
                                _: *const libc::c_void,
                                _: size_t,
                            ) -> libc::c_int,
                    );
                    (*f).close = Some(
                        archive_compressor_program_close
                            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
                    );
                    (*f).free = Some(
                        archive_compressor_program_free
                            as unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int,
                    );
                    return 0 as libc::c_int;
                }
            }
        }
    }
    archive_compressor_program_free(f);
    archive_set_error(
        _a,
        ENOMEM,
        b"Can\'t allocate memory for filter program\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
unsafe extern "C" fn archive_compressor_program_open(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    return __archive_write_program_open(f, (*data).pdata, (*data).cmd);
}
unsafe extern "C" fn archive_compressor_program_write(
    mut f: *mut archive_write_filter,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    return __archive_write_program_write(f, (*data).pdata, buff, length);
}
unsafe extern "C" fn archive_compressor_program_close(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    return __archive_write_program_close(f, (*data).pdata);
}
unsafe extern "C" fn archive_compressor_program_free(
    mut f: *mut archive_write_filter,
) -> libc::c_int {
    let mut data: *mut private_data = (*f).data as *mut private_data;
    if !data.is_null() {
        free((*data).cmd as *mut libc::c_void);
        archive_string_free(&mut (*data).description);
        __archive_write_program_free((*data).pdata);
        free(data as *mut libc::c_void);
        (*f).data = NULL as *mut libc::c_void
    }
    return 0 as libc::c_int;
}
/*
 * Allocate resources for executing an external program.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_write_program_allocate(
    mut program: *const libc::c_char,
) -> *mut archive_write_program_data {
    let mut data: *mut archive_write_program_data = 0 as *mut archive_write_program_data;
    data = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_write_program_data>() as libc::c_ulong,
    ) as *mut archive_write_program_data;
    if data.is_null() {
        return data;
    }
    (*data).child_stdin = -(1 as libc::c_int);
    (*data).child_stdout = -(1 as libc::c_int);
    (*data).program_name = strdup(program);
    return data;
}
/*
 * Release the resources.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_write_program_free(
    mut data: *mut archive_write_program_data,
) -> libc::c_int {
    if !data.is_null() {
        free((*data).program_name as *mut libc::c_void);
        free((*data).child_buf as *mut libc::c_void);
        free(data as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_write_program_open(
    mut f: *mut archive_write_filter,
    mut data: *mut archive_write_program_data,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if (*data).child_buf.is_null() {
        (*data).child_buf_len = 65536 as libc::c_int as size_t;
        (*data).child_buf_avail = 0 as libc::c_int as size_t;
        (*data).child_buf = malloc((*data).child_buf_len) as *mut libc::c_char;
        if (*data).child_buf.is_null() {
            archive_set_error(
                (*f).archive,
                ENOMEM,
                b"Can\'t allocate compression buffer\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    ret = __archive_create_child(
        cmd,
        &mut (*data).child_stdin,
        &mut (*data).child_stdout,
        &mut (*data).child,
    );
    if ret != ARCHIVE_OK {
        archive_set_error(
            (*f).archive,
            EINVAL,
            b"Can\'t launch external program: %s\x00" as *const u8 as *const libc::c_char,
            cmd,
        );
        return -(30 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn child_write(
    mut f: *mut archive_write_filter,
    mut data: *mut archive_write_program_data,
    mut buf: *const libc::c_char,
    mut buf_len: size_t,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    if (*data).child_stdin == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as ssize_t;
    }
    if buf_len == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as ssize_t;
    }
    loop {
        loop {
            ret = write((*data).child_stdin, buf as *const libc::c_void, buf_len);
            if !(ret == -(1 as libc::c_int) as libc::c_long && errno == EINTR) {
                break;
            }
        }
        if ret > 0 as libc::c_int as libc::c_long {
            return ret;
        }
        if ret == 0 as libc::c_int as libc::c_long {
            close((*data).child_stdin);
            (*data).child_stdin = -(1 as libc::c_int);
            fcntl((*data).child_stdout, F_SETFL, 0 as libc::c_int);
            return 0 as libc::c_int as ssize_t;
        }
        if ret == -(1 as libc::c_int) as libc::c_long && errno != EAGAIN {
            return -(1 as libc::c_int) as ssize_t;
        }
        if (*data).child_stdout == -(1 as libc::c_int) {
            fcntl((*data).child_stdin, F_SETFL, 0 as libc::c_int);
            __archive_check_child((*data).child_stdin, (*data).child_stdout);
        } else {
            loop {
                ret = read(
                    (*data).child_stdout,
                    (*data).child_buf.offset((*data).child_buf_avail as isize) as *mut libc::c_void,
                    (*data).child_buf_len.wrapping_sub((*data).child_buf_avail),
                );
                if !(ret == -(1 as libc::c_int) as libc::c_long && errno == EINTR) {
                    break;
                }
            }
            if ret == 0 as libc::c_int as libc::c_long
                || ret == -(1 as libc::c_int) as libc::c_long && errno == EPIPE
            {
                close((*data).child_stdout);
                (*data).child_stdout = -(1 as libc::c_int);
                fcntl((*data).child_stdin, F_SETFL, 0 as libc::c_int);
            } else if ret == -(1 as libc::c_int) as libc::c_long && errno == EAGAIN {
                __archive_check_child((*data).child_stdin, (*data).child_stdout);
            } else {
                if ret == -(1 as libc::c_int) as libc::c_long {
                    return -(1 as libc::c_int) as ssize_t;
                }
                (*data).child_buf_avail = ((*data).child_buf_avail as libc::c_ulong)
                    .wrapping_add(ret as libc::c_ulong)
                    as size_t as size_t;
                ret = __archive_write_filter(
                    (*f).next_filter,
                    (*data).child_buf as *const libc::c_void,
                    (*data).child_buf_avail,
                ) as ssize_t;
                if ret != ARCHIVE_OK as libc::c_long {
                    return -(1 as libc::c_int) as ssize_t;
                }
                (*data).child_buf_avail = 0 as libc::c_int as size_t
            }
        }
    }
}
/*
 * Write data to the filter stream.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_write_program_write(
    mut f: *mut archive_write_filter,
    mut data: *mut archive_write_program_data,
    mut buff: *const libc::c_void,
    mut length: size_t,
) -> libc::c_int {
    let mut ret: ssize_t = 0;
    let mut buf: *const libc::c_char = 0 as *const libc::c_char;
    if (*data).child == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    buf = buff as *const libc::c_char;
    while length > 0 as libc::c_int as libc::c_ulong {
        ret = child_write(f, data, buf, length);
        if ret == -(1 as libc::c_int) as libc::c_long || ret == 0 as libc::c_int as libc::c_long {
            archive_set_error(
                (*f).archive,
                EIO,
                b"Can\'t write to program: %s\x00" as *const u8 as *const libc::c_char,
                (*data).program_name,
            );
            return -(30 as libc::c_int);
        }
        length = (length as libc::c_ulong).wrapping_sub(ret as libc::c_ulong) as size_t as size_t;
        buf = buf.offset(ret as isize)
    }
    return 0 as libc::c_int;
}
/*
 * Finish the filtering...
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_write_program_close(
    mut f: *mut archive_write_filter,
    mut data: *mut archive_write_program_data,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut bytes_read: ssize_t = 0;
    if (*data).child == 0 as libc::c_int {
        return ARCHIVE_OK;
    }
    ret = 0 as libc::c_int;
    close((*data).child_stdin);
    (*data).child_stdin = -(1 as libc::c_int);
    fcntl((*data).child_stdout, F_SETFL, 0 as libc::c_int);
    loop {
        loop {
            bytes_read = read(
                (*data).child_stdout,
                (*data).child_buf.offset((*data).child_buf_avail as isize) as *mut libc::c_void,
                (*data).child_buf_len.wrapping_sub((*data).child_buf_avail),
            );
            if !(bytes_read == -(1 as libc::c_int) as libc::c_long && errno == EINTR) {
                break;
            }
        }
        if bytes_read == 0 as libc::c_int as libc::c_long
            || bytes_read == -(1 as libc::c_int) as libc::c_long && errno == EPIPE
        {
            break;
        }
        if bytes_read == -(1 as libc::c_int) as libc::c_long {
            archive_set_error(
                (*f).archive,
                errno,
                b"Error reading from program: %s\x00" as *const u8 as *const libc::c_char,
                (*data).program_name,
            );
            ret = ARCHIVE_FATAL;
            break;
        } else {
            (*data).child_buf_avail = ((*data).child_buf_avail as libc::c_ulong)
                .wrapping_add(bytes_read as libc::c_ulong)
                as size_t as size_t;
            ret = __archive_write_filter(
                (*f).next_filter,
                (*data).child_buf as *const libc::c_void,
                (*data).child_buf_avail,
            );
            if ret != ARCHIVE_OK {
                ret = ARCHIVE_FATAL;
                break;
            } else {
                (*data).child_buf_avail = 0 as libc::c_int as size_t
            }
        }
    }
    /* Shut down the child. */
    if (*data).child_stdin != -(1 as libc::c_int) {
        close((*data).child_stdin);
    }
    if (*data).child_stdout != -(1 as libc::c_int) {
        close((*data).child_stdout);
    }
    while waitpid((*data).child, &mut status, 0 as libc::c_int) == -(1 as libc::c_int)
        && errno == EINTR
    {}
    (*data).child = 0 as libc::c_int;
    if status != 0 as libc::c_int {
        archive_set_error(
            (*f).archive,
            EIO,
            b"Error closing program: %s\x00" as *const u8 as *const libc::c_char,
            (*data).program_name,
        );
        ret = ARCHIVE_FATAL
    }
    return ret;
}
