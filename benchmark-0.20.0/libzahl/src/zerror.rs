use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn abort() -> !;
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_error: libc::c_int;
}
pub type zerror = libc::c_uint;
pub const ZERROR_ERRNO_SET: zerror = 0;
#[no_mangle]
pub unsafe extern "C" fn zerror(mut desc: *mut *const libc::c_char) -> zerror {
    if libzahl_error >= 0 as libc::c_int {
        if !desc.is_null() {
            *desc = strerror(libzahl_error);
        }
        *__errno_location() = libzahl_error;
        return ZERROR_ERRNO_SET;
    } else {
        if !desc.is_null() {
            abort();
        }
        return -libzahl_error as zerror;
    };
}
