use ::libc;


extern "C" {
    #[no_mangle]
    fn malloc(_: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}

struct S {
    f: *mut i32,
    g: *mut i32
}


unsafe fn f(s: *mut S) {
    (*s).f = malloc(4) as *mut i32;
    (*s).g = (*s).f
}


unsafe fn g(s: *mut S) {
    free((*s).f as *mut libc::c_void);
    free(s as *mut libc::c_void)
}