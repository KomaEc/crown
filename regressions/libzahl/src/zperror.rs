use ::libc;
extern "C" {
    fn abort() -> !;
    static mut libzahl_error: libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn perror(__s: *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn zperror(mut prefix: *const libc::c_char) {
    if crate::src::zperror::libzahl_error >= 0 as libc::c_int {
        *__errno_location() = crate::src::zperror::libzahl_error;
        perror(prefix);
    } else {
        abort();
    };
}
