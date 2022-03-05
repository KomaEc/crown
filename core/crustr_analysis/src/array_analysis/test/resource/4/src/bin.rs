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

unsafe fn f(p: *mut i32, q: *mut i32) -> *mut i32 {
    q
}

unsafe fn g() -> *mut i32 {
    let p = malloc(4) as *mut i32;
    let q = calloc(12, 4) as *mut i32;
    f(p, q)
}

unsafe fn h() -> *mut i32 {
    realloc(g() as *mut libc::c_void, 22) as *mut i32
}