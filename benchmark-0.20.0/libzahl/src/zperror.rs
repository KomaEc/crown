use ::libc;
extern "C" {
    fn abort() -> !;
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_error: libc::c_int;
    fn perror(__s: *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn zperror(mut prefix: *const libc::c_char) {
    if libzahl_error >= 0 as libc::c_int {
        *__errno_location() = libzahl_error;
        perror(prefix);
    } else {
        abort();
    };
}
