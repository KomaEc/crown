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
    f: *mut i32,
}

unsafe fn g(s: *mut S) {
    free((*s).f as *mut libc::c_void);
    free(s as *mut libc::c_void);
}

unsafe fn f(p: *mut i32) -> *mut S {
    let mut this = malloc(8 as u64) as *mut S;
    if this.is_null() {
        // free(p as *mut libc::c_void);
        return this
    }

    (*this).f = p;
    return this 
}