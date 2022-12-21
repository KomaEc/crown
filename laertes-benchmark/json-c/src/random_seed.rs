#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           register_tool)]
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn _json_c_strerror(errno_in: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn open(_: *const std::os::raw::c_char, _: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn close(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn read(_: std::os::raw::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn stat(_: *const std::os::raw::c_char, _: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn time(_: *mut time_t) -> time_t;
}
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_longlong;
pub type __uint64_t = std::os::raw::c_ulonglong;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_ssize_t = std::os::raw::c_long;
pub type __darwin_time_t = std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
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
pub type off_t = __darwin_off_t;
pub type time_t = __darwin_time_t;
pub type mode_t = __darwin_mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
pub type blksize_t = __darwin_blksize_t;
pub type blkcnt_t = __darwin_blkcnt_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: std::os::raw::c_long,
}
pub type dev_t = __darwin_dev_t;
pub type gid_t = __darwin_gid_t;
pub type uid_t = __darwin_uid_t;
pub type nlink_t = __uint16_t;
/*
 * random_seed.c
 *
 * Copyright (c) 2013 Metaparadigm Pte. Ltd.
 * Michael Clark <michael@metaparadigm.com>
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 */
/* defined ENABLE_RDRAND */
/* has_dev_urandom */
static mut dev_random_file: *const std::os::raw::c_char =
    b"/dev/urandom\x00" as *const u8 as *const std::os::raw::c_char;
unsafe extern "C" fn has_dev_urandom() -> std::os::raw::c_int {
    let mut buf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    if stat(dev_random_file, &mut buf) != 0 { return 0 as std::os::raw::c_int }
    return (buf.st_mode as std::os::raw::c_int & 0o20000 as std::os::raw::c_int !=
                0 as std::os::raw::c_int) as std::os::raw::c_int;
}
/* get_dev_random_seed */
unsafe extern "C" fn get_dev_random_seed() -> std::os::raw::c_int {
    let mut fd: std::os::raw::c_int = open(dev_random_file, 0 as std::os::raw::c_int);
    if fd < 0 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"error opening %s: %s\x00" as *const u8 as
                    *const std::os::raw::c_char, dev_random_file,
                _json_c_strerror(*__error()));
        exit(1 as std::os::raw::c_int);
    }
    let mut r: std::os::raw::c_int = 0;
    let mut nread: ssize_t =
        read(fd, &mut r as *mut std::os::raw::c_int as *mut libc::c_void,
             ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong);
    if nread as std::os::raw::c_ulong !=
           ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong {
        fprintf(__stderrp,
                b"error short read %s: %s\x00" as *const u8 as
                    *const std::os::raw::c_char, dev_random_file,
                _json_c_strerror(*__error()));
        exit(1 as std::os::raw::c_int);
    }
    close(fd);
    return r;
}
/* get_cryptgenrandom_seed */
/* get_time_seed */
unsafe extern "C" fn get_time_seed() -> std::os::raw::c_int {
    return time(0 as *mut time_t) as std::os::raw::c_int * 433494437 as std::os::raw::c_int;
}
/* json_c_get_random_seed */
#[no_mangle]
pub unsafe extern "C" fn json_c_get_random_seed() -> std::os::raw::c_int {
    if has_dev_urandom() != 0 { return get_dev_random_seed() }
    return get_time_seed();
}
