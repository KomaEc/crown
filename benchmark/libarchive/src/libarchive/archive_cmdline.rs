use ::libc;
extern "C" {
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
}
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
/*-
 * Copyright (c) 2012 Michihiro NAKAJIMA
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_cmdline {
    pub path: *mut libc::c_char,
    pub argv: *mut *mut libc::c_char,
    pub argc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn extract_quotation(
    mut as_0: *mut archive_string,
    mut p: *const libc::c_char,
) -> ssize_t {
    let mut s: *const libc::c_char = 0 as *const libc::c_char; /* Invalid sequence. */
    s = p.offset(1 as libc::c_int as isize);
    while *s != 0 {
        if *s as libc::c_int == '\\' as i32 {
            if *s.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
                archive_strappend_char(as_0, *s.offset(1 as libc::c_int as isize));
                s = s.offset(2 as libc::c_int as isize)
            } else {
                s = s.offset(1)
            }
        } else {
            if *s as libc::c_int == '\"' as i32 {
                break;
            }
            archive_strappend_char(as_0, *s.offset(0 as libc::c_int as isize));
            s = s.offset(1)
        }
    }
    if *s as libc::c_int != '\"' as i32 {
        return -(25 as libc::c_int) as ssize_t;
    }
    return s.offset(1 as libc::c_int as isize).offset_from(p) as libc::c_long;
}
unsafe extern "C" fn get_argument(
    mut as_0: *mut archive_string,
    mut p: *const libc::c_char,
) -> ssize_t {
    let mut s: *const libc::c_char = p;
    (*as_0).length = 0 as libc::c_int as size_t;
    /* Skip beginning space characters. */
    while *s as libc::c_int != '\u{0}' as i32 && *s as libc::c_int == ' ' as i32 {
        s = s.offset(1)
    }
    /* Copy non-space characters. */
    while *s as libc::c_int != '\u{0}' as i32 && *s as libc::c_int != ' ' as i32 {
        if *s as libc::c_int == '\\' as i32 {
            if *s.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
                archive_strappend_char(as_0, *s.offset(1 as libc::c_int as isize)); /* Ignore this character.*/
                s = s.offset(2 as libc::c_int as isize)
            } else {
                s = s.offset(1); /* Invalid sequence. */
                break;
            }
        } else if *s as libc::c_int == '\"' as i32 {
            let mut q: ssize_t = extract_quotation(as_0, s);
            if q < 0 as libc::c_int as libc::c_long {
                return -(25 as libc::c_int) as ssize_t;
            }
            s = s.offset(q as isize)
        } else {
            archive_strappend_char(as_0, *s.offset(0 as libc::c_int as isize));
            s = s.offset(1)
        }
    }
    return s.offset_from(p) as libc::c_long;
}
/*
 * Set up command line arguments.
 * Returns ARCHIVE_OK if everything okey.
 * Returns ARCHIVE_FAILED if there is a lack of the `"' terminator or an
 * empty command line.
 * Returns ARCHIVE_FATAL if no memory.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_cmdline_parse(
    mut data: *mut archive_cmdline,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut as_0: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut al: ssize_t = 0;
    let mut r: libc::c_int = 0;
    as_0.s = NULL as *mut libc::c_char;
    as_0.length = 0 as libc::c_int as size_t;
    as_0.buffer_length = 0 as libc::c_int as size_t;
    /* Get first argument as a command path. */
    al = get_argument(&mut as_0, cmd); /* Invalid sequence. */
    if al < 0 as libc::c_int as libc::c_long {
        r = ARCHIVE_FAILED
    } else if as_0.length == 0 as libc::c_int as libc::c_ulong {
        r = ARCHIVE_FAILED
    } else {
        r = cmdline_set_path(data, as_0.s); /* An empty command path. */
        if !(r != ARCHIVE_OK) {
            p = strrchr(as_0.s, '/' as i32); /* Invalid sequence. */
            if p.is_null() {
                p = as_0.s
            } else {
                p = p.offset(1)
            }
            r = cmdline_add_arg(data, p);
            if !(r != ARCHIVE_OK) {
                cmd = cmd.offset(al as isize);
                loop {
                    al = get_argument(&mut as_0, cmd);
                    if al < 0 as libc::c_int as libc::c_long {
                        r = ARCHIVE_FAILED;
                        current_block = 11461115498278250884;
                        break;
                    } else {
                        if al == 0 as libc::c_int as libc::c_long {
                            current_block = 5494826135382683477;
                            break;
                        }
                        cmd = cmd.offset(al as isize);
                        if as_0.length == 0 as libc::c_int as libc::c_ulong
                            && *cmd as libc::c_int == '\u{0}' as i32
                        {
                            current_block = 5494826135382683477;
                            break;
                        }
                        r = cmdline_add_arg(data, as_0.s);
                        if r != ARCHIVE_OK {
                            current_block = 11461115498278250884;
                            break;
                        }
                    }
                }
                match current_block {
                    11461115498278250884 => {}
                    _ => r = ARCHIVE_OK,
                }
            }
        }
    }
    archive_string_free(&mut as_0);
    return r;
}
/*
 * Set the program path.
 */
unsafe extern "C" fn cmdline_set_path(
    mut data: *mut archive_cmdline,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut newptr: *mut libc::c_char = 0 as *mut libc::c_char;
    newptr = realloc(
        (*data).path as *mut libc::c_void,
        strlen(path).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if newptr.is_null() {
        return -(30 as libc::c_int);
    }
    (*data).path = newptr;
    strcpy((*data).path, path);
    return 0 as libc::c_int;
}
/*
 * Add a argument for the program.
 */
unsafe extern "C" fn cmdline_add_arg(
    mut data: *mut archive_cmdline,
    mut arg: *const libc::c_char,
) -> libc::c_int {
    let mut newargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if (*data).path.is_null() {
        return -(25 as libc::c_int);
    }
    newargv = realloc(
        (*data).argv as *mut libc::c_void,
        (((*data).argc + 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if newargv.is_null() {
        return -(30 as libc::c_int);
    }
    (*data).argv = newargv;
    let ref mut fresh0 = *(*data).argv.offset((*data).argc as isize);
    *fresh0 = strdup(arg);
    if (*(*data).argv.offset((*data).argc as isize)).is_null() {
        return -(30 as libc::c_int);
    }
    /* Set the terminator of argv. */
    (*data).argc += 1;
    let ref mut fresh1 = *(*data).argv.offset((*data).argc as isize);
    *fresh1 = NULL as *mut libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_cmdline_allocate() -> *mut archive_cmdline {
    return calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_cmdline>() as libc::c_ulong,
    ) as *mut archive_cmdline;
}
/*
 * Release the resources.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_cmdline_free(mut data: *mut archive_cmdline) -> libc::c_int {
    if !data.is_null() {
        free((*data).path as *mut libc::c_void);
        if !(*data).argv.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while !(*(*data).argv.offset(i as isize)).is_null() {
                free(*(*data).argv.offset(i as isize) as *mut libc::c_void);
                i += 1
            }
            free((*data).argv as *mut libc::c_void);
        }
        free(data as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
