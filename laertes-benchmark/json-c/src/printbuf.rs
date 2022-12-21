#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: std::os::raw::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut std::os::raw::c_char,
    pub bpos: std::os::raw::c_int,
    pub size: std::os::raw::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_new() -> *mut printbuf {
    let mut p: *mut printbuf = 0 as *mut printbuf;
    p =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<printbuf>() as std::os::raw::c_ulong) as
            *mut printbuf;
    if p.is_null() { return 0 as *mut printbuf }
    (*p).size = 32 as std::os::raw::c_int;
    (*p).bpos = 0 as std::os::raw::c_int;
    (*p).buf = malloc((*p).size as std::os::raw::c_ulong) as *mut std::os::raw::c_char;
    if (*p).buf.is_null() {
        free(p as *mut libc::c_void);
        return 0 as *mut printbuf
    }
    *(*p).buf.offset(0 as std::os::raw::c_int as isize) =
        '\u{0}' as i32 as std::os::raw::c_char;
    return p;
}
/*
 * $Id: printbuf.c,v 1.5 2006/01/26 02:16:28 mclark Exp $
 *
 * Copyright (c) 2004, 2005 Metaparadigm Pte. Ltd.
 * Michael Clark <michael@metaparadigm.com>
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 *
 * Copyright (c) 2008-2009 Yahoo! Inc.  All rights reserved.
 * The copyrights to the contents of this file are licensed under the MIT License
 * (http://www.opensource.org/licenses/mit-license.php)
 */
/* !HAVE_STDARG_H */
/* HAVE_STDARG_H */
/* *
 * Extend the buffer p so it has a size of at least min_size.
 *
 * If the current size is large enough, nothing is changed.
 *
 * Note: this does not check the available space!  The caller
 *  is responsible for performing those calculations.
 */
unsafe extern "C" fn printbuf_extend(mut p: *mut printbuf,
                                     mut min_size: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut t: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut new_size: std::os::raw::c_int = 0;
    if (*p).size >= min_size { return 0 as std::os::raw::c_int }
    new_size = (*p).size * 2 as std::os::raw::c_int;
    if new_size < min_size + 8 as std::os::raw::c_int {
        new_size = min_size + 8 as std::os::raw::c_int
    }
    /* PRINTBUF_DEBUG */
    t =
        realloc((*p).buf as *mut libc::c_void, new_size as std::os::raw::c_ulong) as
            *mut std::os::raw::c_char;
    if t.is_null() { return -(1 as std::os::raw::c_int) }
    (*p).size = new_size;
    (*p).buf = t;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_memappend(mut p: *mut printbuf,
                                            mut buf: *const std::os::raw::c_char,
                                            mut size: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if (*p).size <= (*p).bpos + size + 1 as std::os::raw::c_int {
        if printbuf_extend(p, (*p).bpos + size + 1 as std::os::raw::c_int) <
               0 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
    }
    memcpy((*p).buf.offset((*p).bpos as isize) as *mut libc::c_void,
           buf as *const libc::c_void, size as std::os::raw::c_ulong);
    (*p).bpos += size;
    *(*p).buf.offset((*p).bpos as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_memset(mut pb: *mut printbuf,
                                         mut offset: std::os::raw::c_int,
                                         mut charvalue: std::os::raw::c_int,
                                         mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut size_needed: std::os::raw::c_int = 0;
    if offset == -(1 as std::os::raw::c_int) { offset = (*pb).bpos }
    size_needed = offset + len;
    if (*pb).size < size_needed {
        if printbuf_extend(pb, size_needed) < 0 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
    }
    memset((*pb).buf.offset(offset as isize) as *mut libc::c_void, charvalue,
           len as std::os::raw::c_ulong);
    if (*pb).bpos < size_needed { (*pb).bpos = size_needed }
    return 0 as std::os::raw::c_int;
}
// In rust/src/sprintbuf.c
#[no_mangle]
pub unsafe extern "C" fn printbuf_reset(mut p: *mut printbuf) {
    *(*p).buf.offset(0 as std::os::raw::c_int as isize) =
        '\u{0}' as i32 as std::os::raw::c_char;
    (*p).bpos = 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn printbuf_free(mut p: *mut printbuf) {
    if !p.is_null() {
        free((*p).buf as *mut libc::c_void);
        free(p as *mut libc::c_void);
    };
}
