use ::libc;
extern "C" {
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub const NULL: libc::c_int = 0 as libc::c_int;
static mut lafe_progname: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn lafe_getprogname() -> *const libc::c_char {
    return lafe_progname;
}
#[no_mangle]
pub unsafe extern "C" fn lafe_setprogname(
    mut name: *const libc::c_char,
    mut defaultname: *const libc::c_char,
) {
    if name.is_null() {
        name = defaultname
    }
    lafe_progname = strrchr(name, '/' as i32);
    if !lafe_progname.is_null() {
        lafe_progname = lafe_progname.offset(1)
    } else {
        lafe_progname = name
    };
}
unsafe extern "C" fn lafe_vwarnc(
    mut code: libc::c_int,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    fprintf(
        stderr,
        b"%s: \x00" as *const u8 as *const libc::c_char,
        lafe_progname,
    );
    vfprintf(stderr, fmt, ap.as_va_list());
    if code != 0 as libc::c_int {
        fprintf(
            stderr,
            b": %s\x00" as *const u8 as *const libc::c_char,
            strerror(code),
        );
    }
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
}
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
#[no_mangle]
pub unsafe extern "C" fn lafe_warnc(
    mut code: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    lafe_vwarnc(code, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn lafe_errc(
    mut eval: libc::c_int,
    mut code: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    lafe_vwarnc(code, fmt, ap.as_va_list());
    exit(eval);
}
