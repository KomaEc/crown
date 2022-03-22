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

unsafe fn f(this: *mut S, s: *mut i32, t: *mut i32) -> *mut i32 {
    free(t as *mut libc::c_void);
    (*this).f = s;
    let r = s;
    r
}

unsafe fn g(this: *mut S) {
    let u = malloc(4) as *mut i32;
    let v = malloc(4) as *mut i32;
    let w = f(this, u, v);

    free(w as *mut libc::c_void);

    let x = malloc(4) as *mut i32;
    let y = x;
    let z = f(this, x, y);
}