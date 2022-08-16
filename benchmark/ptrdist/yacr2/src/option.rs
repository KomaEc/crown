use ::libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut channelFile: *mut libc::c_char;
}
/*
 *
 * option.c
 *
 */
/*
 *
 * Includes.
 *
 */
/*
 *
 * Code.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn Option(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    /*
     * Check arguments.
     */
    if argc != 2 as libc::c_int {
        printf(b"\nUsage: yacr2 <filename>\n\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /*
     * Specified options.
     */
    channelFile = *argv.offset(1 as libc::c_int as isize);
}
