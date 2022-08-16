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


#[derive(Clone, Copy)]
struct S {
    simple: *mut i32,
    nested: *mut *mut i32,
}
#[inline(never)]
unsafe fn f(s: *mut S, r: *mut *mut i32) -> *mut i32 {
    let p = realloc(*(*s).nested as *mut libc::c_void, 24) as *mut i32;
    // let q = (*s).nested.offset(2);
    (*s).nested = r.offset(2);
    g(s)
}
#[inline(never)]
unsafe fn g(s: *mut S) -> *mut i32 {
    let mut p = i(*s);
    h(s, p)
}
#[inline(never)]
unsafe fn h(s: *mut S, p: *mut i32) -> *mut i32 {
    // (*s).simple = calloc(12, 4) as *mut i32;
    let _ = realloc((*s).simple as *mut libc::c_void, 24);
    g(s)
}
#[inline(never)]
unsafe fn i(s: S) -> *mut i32 {
    *s.nested
}
#[inline(never)]
fn cond() -> bool {
    true
}