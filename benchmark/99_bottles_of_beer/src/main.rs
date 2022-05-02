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
}
/*
 * 99 Bottles, C, KISS (i.e. keep it simple and straightforward) version
 */
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = 99 as libc::c_int;
    while n > 2 as libc::c_int {
        printf(b"%d bottles of beer on the wall, %d bottles of beer.\nTake one down and pass it around, %d bottles of beer on the wall.\n\n\x00"
                   as *const u8 as *const libc::c_char, n, n,
               n - 1 as libc::c_int);
        n -= 1
    }
    printf(b"2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake one down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n\x00"
               as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
