#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, extern_types,
           register_tool)]
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ::std::ffi::VaList)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn vprintf(_: *const std::os::raw::c_char, _: ::std::ffi::VaList) -> std::os::raw::c_int;
    #[no_mangle]
    fn vsyslog(_: std::os::raw::c_int, _: *const std::os::raw::c_char, _: ::std::ffi::VaList);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
/*
 * $Id: debug.c,v 1.5 2006/01/26 02:16:28 mclark Exp $
 *
 * Copyright (c) 2004, 2005 Metaparadigm Pte. Ltd.
 * Michael Clark <michael@metaparadigm.com>
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 */
/* HAVE_SYSLOG_H */
/* HAVE_UNISTD_H */
/* HAVE_SYS_PARAM_H */
static mut _syslog: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut _debug: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn mc_set_debug(mut debug: std::os::raw::c_int) {
    _debug = debug;
}
#[no_mangle]
pub unsafe extern "C" fn mc_get_debug() -> std::os::raw::c_int { return _debug; }
#[no_mangle]
pub unsafe extern "C" fn mc_set_syslog(mut syslog: std::os::raw::c_int) {
    _syslog = syslog;
}
#[no_mangle]
pub unsafe extern "C" fn mc_debug(mut msg: *const std::os::raw::c_char,
                                  mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if _debug != 0 {
        ap = args.clone();
        if _syslog != 0 {
            vsyslog(7 as std::os::raw::c_int, msg, ap.as_va_list());
        } else { vprintf(msg, ap.as_va_list()); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mc_error(mut msg: *const std::os::raw::c_char,
                                  mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(3 as std::os::raw::c_int, msg, ap.as_va_list());
    } else { vfprintf(__stderrp, msg, ap.as_va_list()); };
}
#[no_mangle]
pub unsafe extern "C" fn mc_info(mut msg: *const std::os::raw::c_char,
                                 mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(6 as std::os::raw::c_int, msg, ap.as_va_list());
    } else { vfprintf(__stderrp, msg, ap.as_va_list()); };
}
