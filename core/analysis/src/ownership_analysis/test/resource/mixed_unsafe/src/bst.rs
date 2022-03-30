use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}

fn f() {
    let mut x = Box::new(0usize);
    let y = Box::new(1usize);
    x = y;
}