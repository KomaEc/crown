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

unsafe fn f(mut p: *mut *mut i32) {

    /*
    p = calloc(12, 8) as *mut *mut i32;
    for i in 0..12 {
        let mut q = p.offset(i);
        *q = calloc(5, 4) as *mut i32;
    }
    */

    // realloc(*p as *mut libc::c_void, 4);

    for i in 0..12 {
        let mut q = p.offset(i);
        *q = realloc(*q as *mut libc::c_void, 4) as *mut i32;
        // free(*q as *mut libc::c_void);
    }

    // realloc(p as *mut libc::c_void, 1222);
}