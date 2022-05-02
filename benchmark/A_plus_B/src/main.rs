#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
use ::rust::*;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
// Standard input-output streams
unsafe fn main_0() -> libc::c_int {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    scanf(
        b"%d%d\x00" as *const u8 as *const libc::c_char,
        &mut a as *mut libc::c_int,
        &mut b as *mut libc::c_int,
    );
    printf(b"%d\n\x00" as *const u8 as *const libc::c_char, a + b);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
