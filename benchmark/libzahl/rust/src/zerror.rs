
extern "C" {
    #[no_mangle]
    static mut libzahl_error: std::os::raw::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn strerror(_: std::os::raw::c_int) -> *mut std::os::raw::c_char;
}
pub type zerror = std::os::raw::c_uint;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zerror(mut desc: *mut *const std::os::raw::c_char)
 -> zerror {
    if libzahl_error >= 0 as std::os::raw::c_int {
        if !desc.is_null() { *desc = strerror(libzahl_error) }
        *__error() = libzahl_error;
        return ZERROR_ERRNO_SET
    }
    if !desc.is_null() {
        match -libzahl_error {
            1 => {
                *desc =
                    b"indeterminate form: 0:th power of 0\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            2 => {
                *desc =
                    b"indeterminate form: 0 divided by 0\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            3 => {
                *desc =
                    b"undefined result: division by 0\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            4 => {
                *desc =
                    b"argument must be non-negative\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            _ => { abort(); }
        }
    }
    return -libzahl_error as zerror;
}
