use ::libc;
extern "C" {
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut libzahl_error: libc::c_int;
    fn abort() -> !;
    fn __errno_location() -> *mut libc::c_int;
}
pub type zerror = libc::c_uint;
pub const ZERROR_ERRNO_SET: zerror = 0;
#[no_mangle]
pub unsafe extern "C" fn zerror(mut desc: Option<&mut *const libc::c_char>) -> zerror {
    if crate::src::zerror::libzahl_error >= 0 as libc::c_int {
        if !desc.as_deref().is_none() {
            *desc.as_deref_mut().unwrap()= strerror(crate::src::zerror::libzahl_error);
        }else { (); }
        *__errno_location() = crate::src::zerror::libzahl_error;
        return ZERROR_ERRNO_SET;
    } else {
        if !desc.as_deref().is_none() {
            abort();
        }else { (); }
        return -crate::src::zerror::libzahl_error as zerror;
    };
}
