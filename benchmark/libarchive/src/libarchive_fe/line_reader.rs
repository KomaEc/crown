use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn lafe_errc(eval: libc::c_int, code: libc::c_int, fmt: *const libc::c_char, _: ...) -> !;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
/*
 * Read lines from file and do something with each one.  If option_null
 * is set, lines are terminated with zero bytes; otherwise, they're
 * terminated with newlines.
 *
 * This uses a self-sizing buffer to handle arbitrarily-long lines.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lafe_line_reader {
    pub f: *mut FILE,
    pub buff: *mut libc::c_char,
    pub buff_end: *mut libc::c_char,
    pub line_start: *mut libc::c_char,
    pub line_end: *mut libc::c_char,
    pub pathname: *mut libc::c_char,
    pub buff_length: size_t,
    pub nullSeparator: libc::c_int,
}
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
/*-
 * Copyright (c) 2009 Joerg Sonnenberger
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
/* Lines separated by null, not CR/CRLF/etc. */
#[no_mangle]
pub unsafe extern "C" fn lafe_line_reader(
    mut pathname: *const libc::c_char,
    mut nullSeparator: libc::c_int,
) -> *mut lafe_line_reader {
    let mut lr: *mut lafe_line_reader = 0 as *mut lafe_line_reader;
    lr = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<lafe_line_reader>() as libc::c_ulong,
    ) as *mut lafe_line_reader;
    if lr.is_null() {
        lafe_errc(
            1 as libc::c_int,
            ENOMEM,
            b"Can\'t open %s\x00" as *const u8 as *const libc::c_char,
            pathname,
        );
    }
    (*lr).nullSeparator = nullSeparator;
    (*lr).pathname = strdup(pathname);
    if strcmp(pathname, b"-\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        (*lr).f = stdin
    } else {
        (*lr).f = fopen(pathname, b"r\x00" as *const u8 as *const libc::c_char)
    }
    if (*lr).f.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Couldn\'t open %s\x00" as *const u8 as *const libc::c_char,
            pathname,
        );
    }
    (*lr).buff_length = 8192 as libc::c_int as size_t;
    (*lr).buff = NULL as *mut libc::c_char;
    (*lr).buff_end = (*lr).buff;
    (*lr).line_end = (*lr).buff_end;
    (*lr).line_start = (*lr).line_end;
    return lr;
}
unsafe extern "C" fn lafe_line_reader_find_eol(mut lr: *mut lafe_line_reader) {
    (*lr).line_end = (*lr).line_end.offset(strcspn(
        (*lr).line_end,
        if (*lr).nullSeparator != 0 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"\r\n\x00" as *const u8 as *const libc::c_char
        },
    ) as isize);
    *(*lr).line_end = '\u{0}' as i32 as libc::c_char;
    /* Noop if line_end == buff_end */
}
#[no_mangle]
pub unsafe extern "C" fn lafe_line_reader_next(
    mut lr: *mut lafe_line_reader,
) -> *const libc::c_char {
    let mut bytes_wanted: size_t = 0;
    let mut bytes_read: size_t = 0;
    let mut new_buff_size: size_t = 0;
    let mut line_start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        /* If there's a line in the buffer, return it immediately. */
        while (*lr).line_end < (*lr).buff_end {
            line_start = (*lr).line_start;
            (*lr).line_end = (*lr).line_end.offset(1);
            (*lr).line_start = (*lr).line_end;
            lafe_line_reader_find_eol(lr);
            if (*lr).nullSeparator != 0
                || *line_start.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            {
                return line_start;
            }
        }
        /* If we're at end-of-file, process the final data. */
        if (*lr).f.is_null() {
            if (*lr).line_start == (*lr).buff_end {
                return 0 as *const libc::c_char;
            } /* No more text */
            line_start = (*lr).line_start;
            (*lr).line_start = (*lr).buff_end;
            return line_start;
        }
        /* Buffer only has part of a line. */
        if (*lr).line_start > (*lr).buff {
            /* Move a leftover fractional line to the beginning. */
            memmove(
                (*lr).buff as *mut libc::c_void,
                (*lr).line_start as *const libc::c_void,
                (*lr).buff_end.offset_from((*lr).line_start) as libc::c_long
                    as libc::c_ulong,
            );
            (*lr).buff_end = (*lr).buff_end.offset(
                -((*lr).line_start.offset_from((*lr).buff) as libc::c_long as isize),
            );
            (*lr).line_end = (*lr).line_end.offset(
                -((*lr).line_start.offset_from((*lr).buff) as libc::c_long as isize),
            );
            (*lr).line_start = (*lr).buff
        } else {
            /* Line is too big; enlarge the buffer. */
            new_buff_size = (*lr)
                .buff_length
                .wrapping_mul(2 as libc::c_int as libc::c_ulong);
            if new_buff_size <= (*lr).buff_length {
                lafe_errc(
                    1 as libc::c_int,
                    ENOMEM,
                    b"Line too long in %s\x00" as *const u8 as *const libc::c_char,
                    (*lr).pathname,
                );
            }
            (*lr).buff_length = new_buff_size;
            /*
             * Allocate one extra byte to allow terminating
             * the buffer.
             */
            p = realloc(
                (*lr).buff as *mut libc::c_void,
                new_buff_size.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            if p.is_null() {
                lafe_errc(
                    1 as libc::c_int,
                    ENOMEM,
                    b"Line too long in %s\x00" as *const u8 as *const libc::c_char,
                    (*lr).pathname,
                );
            }
            (*lr).buff_end =
                p.offset((*lr).buff_end.offset_from((*lr).buff) as libc::c_long as isize);
            (*lr).line_end =
                p.offset((*lr).line_end.offset_from((*lr).buff) as libc::c_long as isize);
            (*lr).buff = p;
            (*lr).line_start = (*lr).buff
        }
        /* Get some more data into the buffer. */
        bytes_wanted = (*lr)
            .buff
            .offset((*lr).buff_length as isize)
            .offset_from((*lr).buff_end) as libc::c_long as size_t; /* Always terminate buffer */
        bytes_read = fread(
            (*lr).buff_end as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            bytes_wanted,
            (*lr).f,
        );
        (*lr).buff_end = (*lr).buff_end.offset(bytes_read as isize);
        *(*lr).buff_end = '\u{0}' as i32 as libc::c_char;
        lafe_line_reader_find_eol(lr);
        if ferror((*lr).f) != 0 {
            lafe_errc(
                1 as libc::c_int,
                errno,
                b"Can\'t read %s\x00" as *const u8 as *const libc::c_char,
                (*lr).pathname,
            );
        }
        if feof((*lr).f) != 0 {
            if (*lr).f != stdin {
                fclose((*lr).f);
            }
            (*lr).f = NULL as *mut FILE
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn lafe_line_reader_free(mut lr: *mut lafe_line_reader) {
    free((*lr).buff as *mut libc::c_void);
    free((*lr).pathname as *mut libc::c_void);
    free(lr as *mut libc::c_void);
}
