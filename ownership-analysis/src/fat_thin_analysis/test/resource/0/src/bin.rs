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
    pub g: *mut *mut i32,
}


pub unsafe fn f(mut r: *mut i32, mut s: S) {
    // let mut p = 0 as *mut i32;
    // let mut q = 0 as *mut i32;
    s.f = r;
    // p = r;
    if r.is_null() {
        *s.g = calloc(12, 4) as *mut i32;
    } else {
        *s.g = realloc(r as *mut libc::c_void, 4) as *mut i32;
    }
    s.f = *s.g;
    let _ = s.f.offset(2);
}