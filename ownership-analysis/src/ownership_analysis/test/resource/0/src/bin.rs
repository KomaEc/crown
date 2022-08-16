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
    pub f: *mut i32,
}

struct T {
    pub g: *mut i32,
}


pub unsafe fn f() {
    let mut p = malloc(4) as *mut i32;
    let mut q = p;
    free(q as *mut libc::c_void);
}