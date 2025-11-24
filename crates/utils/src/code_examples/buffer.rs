//! Hard code the libtree benchmark for testing purposes

use std::{io, path::Path};

use rustc_driver::Callbacks;
use rustc_interface::Config;

pub struct Buffer;

impl rustc_span::source_map::FileLoader for Buffer {
    fn file_exists(&self, path: &Path) -> bool {
        path == Path::new("lib.rs")
            || path == Path::new("src/buffer.rs")
            || path == Path::new("src/test.rs")
    }

    fn read_file(&self, path: &Path) -> std::io::Result<String> {
        if path == Path::new("lib.rs") {
            Ok(LIB_RS.to_string())
        } else if path == Path::new("src/buffer.rs") {
            Ok(BUFFER_RS.to_string())
        } else if path == Path::new("src/test.rs") {
            Ok(TEST_RS.to_string())
        } else {
            Err(io::Error::other("oops"))
        }
    }

    fn read_binary_file(&self, _path: &Path) -> std::io::Result<std::sync::Arc<[u8]>> {
        Err(io::Error::other("oops"))
    }
}

pub struct BufferCompiler<'callbacks>(pub &'callbacks mut (dyn Callbacks + Send));

impl rustc_driver::Callbacks for BufferCompiler<'_> {
    fn config(&mut self, config: &mut Config) {
        config.file_loader = Some(Box::new(Buffer));
    }

    fn after_crate_root_parsing(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        krate: &mut rustc_ast::Crate,
    ) -> rustc_driver::Compilation {
        self.0.after_crate_root_parsing(compiler, krate)
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &rustc_interface::interface::Compiler,
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        self.0.after_analysis(compiler, tcx)
    }
}

const LIB_RS: &str = r#"#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(strict_provenance)]
#![feature(core_intrinsics)]
#![feature(raw_ref_op)]
#![feature(rustc_private)]
extern crate libc;
pub mod src {
    pub mod buffer;
    pub mod test;
} // mod src
"#;

const BUFFER_RS: &str = r#"use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong
    ) -> *mut libc::c_void;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(
        _: *const libc::c_char,
        _: *const libc::c_char
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: *mut libc::c_char,
    pub data: *mut libc::c_char
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new() -> *mut buffer_t {
    return buffer_new_with_size(64 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_size(mut n: size_t) -> *mut buffer_t {
    let mut self_0: *mut buffer_t =
        malloc(::std::mem::size_of::<buffer_t>() as libc::c_ulong)
            as *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    (*self_0).len = n;
    (*self_0).alloc = calloc(
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong
    ) as *mut libc::c_char;
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string(
    mut str: *mut libc::c_char
) -> *mut buffer_t {
    return buffer_new_with_string_length(str, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string_length(
    mut str: *mut libc::c_char,
    mut len: size_t
) -> *mut buffer_t {
    let mut self_0: *mut buffer_t =
        malloc(::std::mem::size_of::<buffer_t>() as libc::c_ulong)
            as *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    (*self_0).len = len;
    (*self_0).alloc = str;
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_copy(
    mut str: *mut libc::c_char
) -> *mut buffer_t {
    let mut len: size_t = strlen(str);
    let mut self_0: *mut buffer_t = buffer_new_with_size(len);
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    memcpy(
        (*self_0).alloc as *mut libc::c_void,
        str as *const libc::c_void,
        len
    );
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_compact(mut self_0: *mut buffer_t) -> ssize_t {
    let mut len: size_t = buffer_length(self_0);
    let mut rem: size_t = ((*self_0).len).wrapping_sub(len);
    let mut buf: *mut libc::c_char = calloc(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong
    ) as *mut libc::c_char;
    if buf.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    memcpy(
        buf as *mut libc::c_void,
        (*self_0).data as *const libc::c_void,
        len
    );
    free((*self_0).alloc as *mut libc::c_void);
    (*self_0).len = len;
    (*self_0).alloc = buf;
    (*self_0).data = (*self_0).alloc;
    return rem as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_free(mut self_0: *mut buffer_t) {
    free((*self_0).alloc as *mut libc::c_void);
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_size(mut self_0: *mut buffer_t) -> size_t {
    return (*self_0).len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_length(mut self_0: *mut buffer_t) -> size_t {
    return strlen((*self_0).data);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_resize(
    mut self_0: *mut buffer_t,
    mut n: size_t
) -> libc::c_int {
    n = n.wrapping_add(
        (1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
    ) & !(1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    (*self_0).len = n;
    (*self_0).data = realloc(
        (*self_0).alloc as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as libc::c_ulong)
    ) as *mut libc::c_char;
    (*self_0).alloc = (*self_0).data;
    if ((*self_0).alloc).is_null() {
        return -(1 as libc::c_int);
    }
    *((*self_0).alloc).offset(n as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_appendf(
    mut self_0: *mut buffer_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    let mut tmpa: ::std::ffi::VaListImpl;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: libc::c_int = 0 as libc::c_int;
    let mut required: libc::c_int = 0 as libc::c_int;
    let mut bytes: libc::c_int = 0 as libc::c_int;
    ap = args.clone();
    length = buffer_length(self_0) as libc::c_int;
    tmpa = ap.clone();
    required = vsnprintf(
        0 as *mut libc::c_char,
        0 as libc::c_int as libc::c_ulong,
        format,
        tmpa.as_va_list()
    );
    if -(1 as libc::c_int)
        == buffer_resize(self_0, (length + required) as size_t)
    {
        return -(1 as libc::c_int);
    }
    dst = ((*self_0).data).offset(length as isize);
    bytes = vsnprintf(
        dst,
        (1 as libc::c_int + required) as libc::c_ulong,
        format,
        ap.as_va_list()
    );
    return if bytes < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append(
    mut self_0: *mut buffer_t,
    mut str: *const libc::c_char
) -> libc::c_int {
    return buffer_append_n(self_0, str, strlen(str));
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append_n(
    mut self_0: *mut buffer_t,
    mut str: *const libc::c_char,
    mut len: size_t
) -> libc::c_int {
    let mut prev: size_t = strlen((*self_0).data);
    let mut needed: size_t = len.wrapping_add(prev);
    if (*self_0).len > needed {
        strncat((*self_0).data, str, len);
        return 0 as libc::c_int;
    }
    let mut ret: libc::c_int = buffer_resize(self_0, needed);
    if -(1 as libc::c_int) == ret {
        return -(1 as libc::c_int);
    }
    strncat((*self_0).data, str, len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_prepend(
    mut self_0: *mut buffer_t,
    mut str: *mut libc::c_char
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut len: size_t = strlen(str);
    let mut prev: size_t = strlen((*self_0).data);
    let mut needed: size_t = len.wrapping_add(prev);
    if !((*self_0).len > needed) {
        ret = buffer_resize(self_0, needed);
        if -(1 as libc::c_int) == ret {
            return -(1 as libc::c_int);
        }
    }
    memmove(
        ((*self_0).data).offset(len as isize) as *mut libc::c_void,
        (*self_0).data as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong)
    );
    memcpy(
        (*self_0).data as *mut libc::c_void,
        str as *const libc::c_void,
        len
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_slice(
    mut buf: *mut buffer_t,
    mut from: size_t,
    mut to: ssize_t
) -> *mut buffer_t {
    let mut len: size_t = strlen((*buf).data);
    if (to as libc::c_ulong) < from {
        return 0 as *mut buffer_t;
    }
    if to < 0 as libc::c_int as libc::c_long {
        to = len.wrapping_sub(!to as libc::c_ulong) as ssize_t;
    }
    if to as libc::c_ulong > len {
        to = len as ssize_t;
    }
    let mut n: size_t = (to as libc::c_ulong).wrapping_sub(from);
    let mut self_0: *mut buffer_t = buffer_new_with_size(n);
    memcpy(
        (*self_0).data as *mut libc::c_void,
        ((*buf).data).offset(from as isize) as *const libc::c_void,
        n
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_equals(
    mut self_0: *mut buffer_t,
    mut other: *mut buffer_t
) -> libc::c_int {
    return (0 as libc::c_int == strcmp((*self_0).data, (*other).data))
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_indexof(
    mut self_0: *mut buffer_t,
    mut str: *mut libc::c_char
) -> ssize_t {
    let mut sub: *mut libc::c_char = strstr((*self_0).data, str);
    if sub.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    return sub.offset_from((*self_0).data) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_left(mut self_0: *mut buffer_t) {
    let mut c: libc::c_int = 0;
    loop {
        c = *(*self_0).data as libc::c_int;
        if !(c != 0
            && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
        {
            break;
        }
        (*self_0).data = ((*self_0).data).offset(1);
        (*self_0).data;
    }
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_right(mut self_0: *mut buffer_t) {
    let mut c: libc::c_int = 0;
    let mut i: size_t =
        (buffer_length(self_0)).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        c = *((*self_0).data).offset(i as isize) as libc::c_int;
        if !(c != 0
            && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0)
        {
            break;
        }
        let fresh0 = i;
        i = i.wrapping_sub(1);
        *((*self_0).data).offset(fresh0 as isize) =
            0 as libc::c_int as libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim(mut self_0: *mut buffer_t) {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_fill(
    mut self_0: *mut buffer_t,
    mut c: libc::c_int
) {
    memset((*self_0).data as *mut libc::c_void, c, (*self_0).len);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_clear(mut self_0: *mut buffer_t) {
    buffer_fill(self_0, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_print(mut self_0: *mut buffer_t) {
    let mut len: size_t = (*self_0).len;
    printf(b"\n \0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < len {
        printf(
            b" %02x\0" as *const u8 as *const libc::c_char,
            *((*self_0).alloc).offset(i as isize) as libc::c_int
        );
        if (i + 1 as libc::c_int) % 8 as libc::c_int == 0 as libc::c_int {
            printf(b"\n \0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
"#;

const TEST_RS: &str = r#"use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char
    ) -> !;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn buffer_new() -> *mut buffer_t;
    fn buffer_new_with_size(n: size_t) -> *mut buffer_t;
    fn buffer_new_with_copy(str: *mut libc::c_char) -> *mut buffer_t;
    fn buffer_size(self_0: *mut buffer_t) -> size_t;
    fn buffer_length(self_0: *mut buffer_t) -> size_t;
    fn buffer_free(self_0: *mut buffer_t);
    fn buffer_prepend(
        self_0: *mut buffer_t,
        str: *mut libc::c_char
    ) -> libc::c_int;
    fn buffer_append(
        self_0: *mut buffer_t,
        str: *const libc::c_char
    ) -> libc::c_int;
    fn buffer_appendf(
        self_0: *mut buffer_t,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn buffer_append_n(
        self_0: *mut buffer_t,
        str: *const libc::c_char,
        len: size_t
    ) -> libc::c_int;
    fn buffer_equals(
        self_0: *mut buffer_t,
        other: *mut buffer_t
    ) -> libc::c_int;
    fn buffer_indexof(self_0: *mut buffer_t, str: *mut libc::c_char)
        -> ssize_t;
    fn buffer_slice(
        self_0: *mut buffer_t,
        from: size_t,
        to: ssize_t
    ) -> *mut buffer_t;
    fn buffer_compact(self_0: *mut buffer_t) -> ssize_t;
    fn buffer_fill(self_0: *mut buffer_t, c: libc::c_int);
    fn buffer_clear(self_0: *mut buffer_t);
    fn buffer_trim_left(self_0: *mut buffer_t);
    fn buffer_trim_right(self_0: *mut buffer_t);
    fn buffer_trim(self_0: *mut buffer_t);
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: *mut libc::c_char,
    pub data: *mut libc::c_char
}
#[no_mangle]
pub unsafe extern "C" fn equal(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char
) {
    if strcmp(a, b) != 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"  expected: '%s'\n\0" as *const u8 as *const libc::c_char,
            a
        );
        printf(
            b"    actual: '%s'\n\0" as *const u8 as *const libc::c_char,
            b
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new() {
    let mut buf: *mut buffer_t = buffer_new();
    if 64 as libc::c_int as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail(
            b"BUFFER_DEFAULT_SIZE == buffer_size(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"void test_buffer_new()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"void test_buffer_new()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new_with_size() {
    let mut buf: *mut buffer_t =
        buffer_new_with_size(1024 as libc::c_int as size_t);
    if 1024 as libc::c_int as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail(
            b"1024 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"void test_buffer_new_with_size()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                b"void test_buffer_new_with_size()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append() {
    let mut buf: *mut buffer_t = buffer_new();
    if 0 as libc::c_int
        == buffer_append(buf, b"Hello\0" as *const u8 as *const libc::c_char)
    {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void test_buffer_append()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" World\0" as *const u8 as *const libc::c_char)
    {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void test_buffer_append()\0"
            ))
            .as_ptr()
        );
    };
    if strlen(b"Hello World\0" as *const u8 as *const libc::c_char)
        == buffer_length(buf)
    {
    } else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void test_buffer_append()\0"
            ))
            .as_ptr()
        );
    };
    equal(
        b"Hello World\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append_n() {
    let mut buf: *mut buffer_t = buffer_new();
    if 0 as libc::c_int
        == buffer_append_n(
            buf,
            b"subway\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t
        )
    {
    } else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"subway\", 3)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void test_buffer_append_n()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int
        == buffer_append_n(
            buf,
            b"marines\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t
        )
    {
    } else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"marines\", 6)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void test_buffer_append_n()\0"
            ))
            .as_ptr()
        );
    };
    if strlen(b"submarine\0" as *const u8 as *const libc::c_char)
        == buffer_length(buf)
    {
    } else {
        __assert_fail(
            b"strlen(\"submarine\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void test_buffer_append_n()\0"
            ))
            .as_ptr()
        );
    };
    equal(
        b"submarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append__grow() {
    let mut buf: *mut buffer_t =
        buffer_new_with_size(10 as libc::c_int as size_t);
    if 0 as libc::c_int
        == buffer_append(buf, b"Hello\0" as *const u8 as *const libc::c_char)
    {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void test_buffer_append__grow()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" tobi\0" as *const u8 as *const libc::c_char)
    {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" tobi\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void test_buffer_append__grow()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" was\0" as *const u8 as *const libc::c_char)
    {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" was\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void test_buffer_append__grow()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" here\0" as *const u8 as *const libc::c_char)
    {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" here\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void test_buffer_append__grow()\0"
            ))
            .as_ptr()
        );
    };
    let mut str: *mut libc::c_char = b"Hello tobi was here\0" as *const u8
        as *const libc::c_char
        as *mut libc::c_char;
    equal(str, (*buf).data);
    if 1024 as libc::c_int as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail(
            b"1024 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void test_buffer_append__grow()\0"
            ))
            .as_ptr()
        );
    };
    if strlen(str) == buffer_length(buf) {
    } else {
        __assert_fail(
            b"strlen(str) == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void test_buffer_append__grow()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_prepend() {
    let mut buf: *mut buffer_t = buffer_new();
    if 0 as libc::c_int
        == buffer_append(buf, b" World\0" as *const u8 as *const libc::c_char)
    {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_prepend()\0"
            ))
            .as_ptr()
        );
    };
    if 0 as libc::c_int
        == buffer_prepend(
            buf,
            b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char
        )
    {
    } else {
        __assert_fail(
            b"0 == buffer_prepend(buf, \"Hello\")\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_prepend()\0"
            ))
            .as_ptr()
        );
    };
    if strlen(b"Hello World\0" as *const u8 as *const libc::c_char)
        == buffer_length(buf)
    {
    } else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_prepend()\0"
            ))
            .as_ptr()
        );
    };
    equal(
        b"Hello World\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice() {
    let mut buf: *mut buffer_t = buffer_new();
    buffer_append(buf, b"Tobi Ferret\0" as *const u8 as *const libc::c_char);
    let mut a: *mut buffer_t = buffer_slice(
        buf,
        2 as libc::c_int as size_t,
        8 as libc::c_int as ssize_t
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*buf).data
    );
    equal(
        b"bi Fer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a).data
    );
    buffer_free(buf);
    buffer_free(a);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__range_error() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    let mut a: *mut buffer_t = buffer_slice(
        buf,
        10 as libc::c_int as size_t,
        2 as libc::c_int as ssize_t
    );
    if a.is_null() {
    } else {
        __assert_fail(
            b"NULL == a\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void test_buffer_slice__range_error()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    let mut a: *mut buffer_t = buffer_slice(
        buf,
        5 as libc::c_int as size_t,
        -(1 as libc::c_int) as ssize_t
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*buf).data
    );
    equal(
        b"Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a).data
    );
    let mut b: *mut buffer_t = buffer_slice(
        buf,
        5 as libc::c_int as size_t,
        -(3 as libc::c_int) as ssize_t
    );
    equal(
        b"Ferr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*b).data
    );
    let mut c: *mut buffer_t = buffer_slice(
        buf,
        8 as libc::c_int as size_t,
        -(1 as libc::c_int) as ssize_t
    );
    equal(
        b"ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*c).data
    );
    buffer_free(buf);
    buffer_free(a);
    buffer_free(b);
    buffer_free(c);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end_overflow() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    let mut a: *mut buffer_t = buffer_slice(
        buf,
        5 as libc::c_int as size_t,
        1000 as libc::c_int as ssize_t
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*buf).data
    );
    equal(
        b"Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a).data
    );
    buffer_free(a);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_equals() {
    let mut a: *mut buffer_t = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    let mut b: *mut buffer_t = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if 1 as libc::c_int == buffer_equals(a, b) {
    } else {
        __assert_fail(
            b"1 == buffer_equals(a, b)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void test_buffer_equals()\0"
            ))
            .as_ptr()
        );
    };
    buffer_append(b, b" World\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == buffer_equals(a, b) {
    } else {
        __assert_fail(
            b"0 == buffer_equals(a, b)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void test_buffer_equals()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(a);
    buffer_free(b);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_formatting() {
    let mut buf: *mut buffer_t = buffer_new();
    let mut result: libc::c_int = buffer_appendf(
        buf,
        b"%d %s\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        b"cow\0" as *const u8 as *const libc::c_char
    );
    if 0 as libc::c_int == result {
    } else {
        __assert_fail(
            b"0 == result\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void test_buffer_formatting()\0"
            ))
            .as_ptr()
        );
    };
    equal(
        b"3 cow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data
    );
    result = buffer_appendf(
        buf,
        b" - 0x%08X\0" as *const u8 as *const libc::c_char,
        0xdeadbeef as libc::c_uint
    );
    if 0 as libc::c_int == result {
    } else {
        __assert_fail(
            b"0 == result\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"void test_buffer_formatting()\0"
            ))
            .as_ptr()
        );
    };
    equal(
        b"3 cow - 0xDEADBEEF\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_indexof() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"Tobi is a ferret\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    let mut i: ssize_t = buffer_indexof(
        buf,
        b"is\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if 5 as libc::c_int as libc::c_long == i {
    } else {
        __assert_fail(
            b"5 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_indexof()\0"
            ))
            .as_ptr()
        );
    };
    i = buffer_indexof(
        buf,
        b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if 8 as libc::c_int as libc::c_long == i {
    } else {
        __assert_fail(
            b"8 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_indexof()\0"
            ))
            .as_ptr()
        );
    };
    i = buffer_indexof(
        buf,
        b"something\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if -(1 as libc::c_int) as libc::c_long == i {
    } else {
        __assert_fail(
            b"-1 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_indexof()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_fill() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"void test_buffer_fill()\0"
            ))
            .as_ptr()
        );
    };
    buffer_fill(buf, 0 as libc::c_int);
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"void test_buffer_fill()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_clear() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char
    );
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void test_buffer_clear()\0"
            ))
            .as_ptr()
        );
    };
    buffer_clear(buf);
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"void test_buffer_clear()\0"
            ))
            .as_ptr()
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_trim() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    buffer_trim(buf);
    equal(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    buffer_trim_left(buf);
    equal(
        b"Hello\n\n \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    buffer_trim_right(buf);
    equal(
        b"  Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_compact() {
    let mut buf: *mut buffer_t = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char
    );
    buffer_trim(buf);
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_compact()\0"
            ))
            .as_ptr()
        );
    };
    if 10 as libc::c_int as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail(
            b"10 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_compact()\0"
            ))
            .as_ptr()
        );
    };
    let mut removed: ssize_t = buffer_compact(buf);
    if 5 as libc::c_int as libc::c_long == removed {
    } else {
        __assert_fail(
            b"5 == removed\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_compact()\0"
            ))
            .as_ptr()
        );
    };
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_compact()\0"
            ))
            .as_ptr()
        );
    };
    if 5 as libc::c_int as libc::c_ulong == buffer_size(buf) {
    } else {
        __assert_fail(
            b"5 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void test_buffer_compact()\0"
            ))
            .as_ptr()
        );
    };
    equal(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data
    );
    buffer_free(buf);
}
unsafe fn main_0() -> libc::c_int {
    test_buffer_new();
    test_buffer_new_with_size();
    test_buffer_append();
    test_buffer_append__grow();
    test_buffer_append_n();
    test_buffer_prepend();
    test_buffer_slice();
    test_buffer_slice__range_error();
    test_buffer_slice__end();
    test_buffer_slice__end_overflow();
    test_buffer_equals();
    test_buffer_formatting();
    test_buffer_indexof();
    test_buffer_fill();
    test_buffer_clear();
    test_buffer_trim();
    test_buffer_compact();
    printf(
        b"\n  \x1B[32m\xE2\x9C\x93 \x1B[90mok\x1B[0m\n\n\0" as *const u8
            as *const libc::c_char
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
"#;
