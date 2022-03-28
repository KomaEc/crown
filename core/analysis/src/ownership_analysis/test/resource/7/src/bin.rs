use ::libc;
extern "C" {
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: u64) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    #[no_mangle]
    fn malloc(_: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = u64;
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

struct S {
    l: *mut S,
    r: *mut S,
}


/*
unsafe fn f(mut s1: *mut S, s2: *mut S) -> *mut S {
    if cond() {
        (*s1).l = f((*s1).l, s2);
    } else {
        // (*s1).r = f((*s1).r, s2);
        free(s2 as *mut libc::c_void);
    }
    s1
}
*/

/*
unsafe fn f(mut s1: *mut S, s2: *mut S) -> *mut S {
    if s1.is_null() {
        s1 = 0 as *mut S;
        // free(s1 as *mut libc::c_void);
        s2
    } else {
        if cond() {
            (*s1).l = f((*s1).l, s2);
        } else {
            // free(s2 as *mut libc::c_void);
        }
        s1
    }
}
*/

/*
unsafe fn g(mut s1: *mut S) {
    free((*s1).l as *mut libc::c_void);
    free((*s1).r as *mut libc::c_void);
}
*/

unsafe fn f(mut s1: *mut S, s2: *mut S) -> *mut S {
    if s1.is_null() {
        s1 = 0 as *mut S;
        s2
    } else {
        if cond() {
            (*s1).l = f((*s1).l, s2);
        } else {
            (*s1).r = f((*s1).r, s2);
        }
        // free(s2 as *mut libc::c_void);
        s1
    }
}

#[inline(never)]
fn cond() -> bool {
    true
}

/* 
unsafe fn f(mut p: *mut i32, q: *mut i32) -> *mut i32 {
    if p.is_null() {
        p = 0 as *mut i32;
        q
    } else {
        p
    }
}
*/