
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
pub use crate::src::zdivmod::zerror;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zerror<'a1>(mut desc: Option<&'a1 mut * const std::os::raw::c_char>)
 -> std::os::raw::c_uint {
    if libzahl_error >= 0 as std::os::raw::c_int {
        if !borrow(& desc).is_none() { *borrow_mut(&mut desc).unwrap() = strerror(libzahl_error) }
        *__error() = libzahl_error;
        return ZERROR_ERRNO_SET
    }
    if !borrow(& desc).is_none() {
        match -libzahl_error {
            1 => {
                *borrow_mut(&mut desc).unwrap() =
                    b"indeterminate form: 0:th power of 0\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            2 => {
                *borrow_mut(&mut desc).unwrap() =
                    b"indeterminate form: 0 divided by 0\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            3 => {
                *borrow_mut(&mut desc).unwrap() =
                    b"undefined result: division by 0\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            4 => {
                *borrow_mut(&mut desc).unwrap() =
                    b"argument must be non-negative\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            _ => { abort(); }
        }
    }
    return -libzahl_error as zerror;
}
pub fn borrow<'a, 'b: 'a, T>(p: &'a Option<&'b mut T>) -> Option<&'a T> {
    p.as_ref().map(|x| &**x)
}

pub fn borrow_mut<'a, 'b : 'a, T>(p: &'a mut Option<&'b mut T>) -> Option<&'a mut T> {
    p.as_mut().map(|x| &mut **x)
}

pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
    p.as_ref().map(|x| x.as_ref())
}

pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
    p.as_mut().map(|x| x.as_mut())
}

pub fn option_to_raw<T>(p: Option<&T>) -> * const T {
    p.map_or(core::ptr::null(), |p| p as * const T)
}

pub fn _ref_eq<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) == option_to_raw(q)
}

pub fn _ref_ne<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) != option_to_raw(q)
}

