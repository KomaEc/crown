#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, extern_types, register_tool)]
extern "C" {
    pub type json_object;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn _json_c_strerror(errno_in: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn printbuf_new() -> *mut printbuf;
    #[no_mangle]
    fn printbuf_memappend(p: *mut printbuf, buf: *const std::os::raw::c_char,
                          size: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn printbuf_free(p: *mut printbuf);
    #[no_mangle]
    fn json_object_to_json_string_ext(obj: *mut json_object,
                                      flags: std::os::raw::c_int)
     -> *const std::os::raw::c_char;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn strtod(_: *const std::os::raw::c_char, _: *mut *mut std::os::raw::c_char)
     -> std::os::raw::c_double;
    #[no_mangle]
    fn strtoll(_: *const std::os::raw::c_char, _: *mut *mut std::os::raw::c_char,
               _: std::os::raw::c_int) -> std::os::raw::c_longlong;
    #[no_mangle]
    fn open(_: *const std::os::raw::c_char, _: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn close(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn read(_: std::os::raw::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const libc::c_void, __nbyte: size_t)
     -> ssize_t;
    #[no_mangle]
    fn json_tokener_parse(str: *const std::os::raw::c_char) -> *mut json_object;
    #[no_mangle]
    fn _json_c_set_last_err(err_fmt: *const std::os::raw::c_char, _: ...);
}
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_ssize_t = std::os::raw::c_long;
pub type int64_t = std::os::raw::c_longlong;
pub type size_t = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut std::os::raw::c_char,
    pub bpos: std::os::raw::c_int,
    pub size: std::os::raw::c_int,
}
pub type json_type = std::os::raw::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub type ssize_t = __darwin_ssize_t;
#[no_mangle]
pub unsafe extern "C" fn json_object_from_fd(mut fd: std::os::raw::c_int)
 -> *mut json_object {
    let mut pb: *mut printbuf = 0 as *mut printbuf;
    let mut obj: *mut json_object = 0 as *mut json_object;
    let mut buf: [std::os::raw::c_char; 4096] = [0; 4096];
    let mut ret: std::os::raw::c_int = 0;
    pb = printbuf_new();
    if pb.is_null() {
        _json_c_set_last_err(b"json_object_from_file: printbuf_new failed\n\x00"
                                 as *const u8 as *const std::os::raw::c_char);
        return 0 as *mut json_object
    }
    loop  {
        ret =
            read(fd, buf.as_mut_ptr() as *mut libc::c_void,
                 4096 as std::os::raw::c_int as size_t) as std::os::raw::c_int;
        if !(ret > 0 as std::os::raw::c_int) { break ; }
        printbuf_memappend(pb, buf.as_mut_ptr(), ret);
    }
    if ret < 0 as std::os::raw::c_int {
        _json_c_set_last_err(b"json_object_from_fd: error reading fd %d: %s\n\x00"
                                 as *const u8 as *const std::os::raw::c_char, fd,
                             _json_c_strerror(*__error()));
        printbuf_free(pb);
        return 0 as *mut json_object
    }
    obj = json_tokener_parse((*pb).buf);
    printbuf_free(pb);
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_from_file(mut filename:
                                                   *const std::os::raw::c_char)
 -> *mut json_object {
    let mut obj: *mut json_object = 0 as *mut json_object;
    let mut fd: std::os::raw::c_int = 0;
    fd = open(filename, 0 as std::os::raw::c_int);
    if fd < 0 as std::os::raw::c_int {
        _json_c_set_last_err(b"json_object_from_file: error opening file %s: %s\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             filename, _json_c_strerror(*__error()));
        return 0 as *mut json_object
    }
    obj = json_object_from_fd(fd);
    close(fd);
    return obj;
}
/* extended "format and write to file" function */
#[no_mangle]
pub unsafe extern "C" fn json_object_to_file_ext(mut filename:
                                                     *const std::os::raw::c_char,
                                                 mut obj: *mut json_object,
                                                 mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut fd: std::os::raw::c_int = 0;
    let mut ret: std::os::raw::c_int = 0;
    let mut saved_errno: std::os::raw::c_int = 0;
    if obj.is_null() {
        _json_c_set_last_err(b"json_object_to_file: object is null\n\x00" as
                                 *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    fd =
        open(filename,
             0x1 as std::os::raw::c_int | 0x400 as std::os::raw::c_int | 0x200 as std::os::raw::c_int,
             0o644 as std::os::raw::c_int);
    if fd < 0 as std::os::raw::c_int {
        _json_c_set_last_err(b"json_object_to_file: error opening file %s: %s\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             filename, _json_c_strerror(*__error()));
        return -(1 as std::os::raw::c_int)
    }
    ret = _json_object_to_fd(fd, obj, flags, filename);
    saved_errno = *__error();
    close(fd);
    *__error() = saved_errno;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_fd(mut fd: std::os::raw::c_int,
                                           mut obj: *mut json_object,
                                           mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if obj.is_null() {
        _json_c_set_last_err(b"json_object_to_fd: object is null\n\x00" as
                                 *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    }
    return _json_object_to_fd(fd, obj, flags, 0 as *const std::os::raw::c_char);
}
/*
 * $Id: json_util.c,v 1.4 2006/01/30 23:07:57 mclark Exp $
 *
 * Copyright (c) 2004, 2005 Metaparadigm Pte. Ltd.
 * Michael Clark <michael@metaparadigm.com>
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 */
/* HAVE_SYS_TYPES_H */
/* HAVE_SYS_STAT_H */
/* HAVE_FCNTL_H */
/* HAVE_UNISTD_H */
/* defined(WIN32) */
unsafe extern "C" fn _json_object_to_fd(mut fd: std::os::raw::c_int,
                                        mut obj: *mut json_object,
                                        mut flags: std::os::raw::c_int,
                                        mut filename: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int =
        0; /* CAW: probably unnecessary, but the most 64bit safe */
    let mut json_str: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut wpos: std::os::raw::c_uint = 0;
    let mut wsize: std::os::raw::c_uint = 0;
    filename =
        if !filename.is_null() {
            filename
        } else { b"(fd)\x00" as *const u8 as *const std::os::raw::c_char };
    json_str = json_object_to_json_string_ext(obj, flags);
    if json_str.is_null() { return -(1 as std::os::raw::c_int) }
    wsize =
        (strlen(json_str) & 0xffffffff as std::os::raw::c_uint as std::os::raw::c_ulong) as
            std::os::raw::c_uint;
    wpos = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while wpos < wsize {
        ret =
            write(fd, json_str.offset(wpos as isize) as *const libc::c_void,
                  wsize.wrapping_sub(wpos) as size_t) as std::os::raw::c_int;
        if ret < 0 as std::os::raw::c_int {
            _json_c_set_last_err(b"json_object_to_file: error writing file %s: %s\n\x00"
                                     as *const u8 as *const std::os::raw::c_char,
                                 filename, _json_c_strerror(*__error()));
            return -(1 as std::os::raw::c_int)
        }
        /* because of the above check for ret < 0, we can safely cast and add */
        wpos = wpos.wrapping_add(ret as std::os::raw::c_uint)
    }
    return 0 as std::os::raw::c_int;
}
// backwards compatible "format and write to file" function
#[no_mangle]
pub unsafe extern "C" fn json_object_to_file(mut filename:
                                                 *const std::os::raw::c_char,
                                             mut obj: *mut json_object)
 -> std::os::raw::c_int {
    return json_object_to_file_ext(filename, obj, 0 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_double(mut buf: *const std::os::raw::c_char,
                                           mut retval: *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    *retval = strtod(buf, &mut end);
    return if end == buf as *mut std::os::raw::c_char {
               1 as std::os::raw::c_int
           } else { 0 as std::os::raw::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_int64(mut buf: *const std::os::raw::c_char,
                                          mut retval: *mut int64_t)
 -> std::os::raw::c_int {
    let mut end: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut val: int64_t = 0;
    *__error() = 0 as std::os::raw::c_int;
    val = strtoll(buf, &mut end, 10 as std::os::raw::c_int);
    if end != buf as *mut std::os::raw::c_char { *retval = val }
    return if val == 0 as std::os::raw::c_int as std::os::raw::c_longlong &&
                  *__error() != 0 as std::os::raw::c_int ||
                  end == buf as *mut std::os::raw::c_char {
               1 as std::os::raw::c_int
           } else { 0 as std::os::raw::c_int };
}
static mut json_type_name: [*const std::os::raw::c_char; 7] =
    [b"null\x00" as *const u8 as *const std::os::raw::c_char,
     b"boolean\x00" as *const u8 as *const std::os::raw::c_char,
     b"double\x00" as *const u8 as *const std::os::raw::c_char,
     b"int\x00" as *const u8 as *const std::os::raw::c_char,
     b"object\x00" as *const u8 as *const std::os::raw::c_char,
     b"array\x00" as *const u8 as *const std::os::raw::c_char,
     b"string\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn json_type_to_name(mut o_type: json_type)
 -> *const std::os::raw::c_char {
    let mut o_type_int: std::os::raw::c_int = o_type as std::os::raw::c_int;
    if o_type_int < 0 as std::os::raw::c_int ||
           o_type_int >=
               (::std::mem::size_of::<[*const std::os::raw::c_char; 7]>() as
                    std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*const std::os::raw::c_char>()
                                                    as std::os::raw::c_ulong) as
                   std::os::raw::c_int {
        _json_c_set_last_err(b"json_type_to_name: type %d is out of range [0,%d]\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             o_type as std::os::raw::c_uint,
                             (::std::mem::size_of::<[*const std::os::raw::c_char; 7]>()
                                  as
                                  std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*const std::os::raw::c_char>()
                                                                  as
                                                                  std::os::raw::c_ulong));
        return 0 as *const std::os::raw::c_char
    }
    return json_type_name[o_type as usize];
}
