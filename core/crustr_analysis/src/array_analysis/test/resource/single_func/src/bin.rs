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



pub unsafe fn f(mut r: *mut i32) {
    let mut p = 0 as *mut i32;
    let mut q = 0 as *mut i32;
    p = r;
    if r.is_null() {
        q = calloc(12, 4) as *mut i32;
    } else {
        q = realloc(r as *mut libc::c_void, 4) as *mut i32;
    }
    p = q;
    let _ = p.offset(2);
}