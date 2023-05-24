
extern "C" {
    pub type __sFILEX;
    
    
    #[no_mangle]
    static mut libzahl_error: std::os::raw::c_int;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(_: *const std::os::raw::c_char);
}
pub use crate::src::zerror::zerror;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = std::os::raw::c_longlong;

pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
pub type FILE = crate::src::zperror::__sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
impl std::default::Default for __sFILE {
    fn default() -> Self {
        __sFILE {
        _p: 0 as * mut std::os::raw::c_uchar,
        _r: std::os::raw::c_int::default(),
        _w: std::os::raw::c_int::default(),
        _flags: std::os::raw::c_short::default(),
        _file: std::os::raw::c_short::default(),
        _bf: crate::src::zperror::__sbuf::default(),
        _lbfsize: std::os::raw::c_int::default(),
        _cookie: 0 as * mut core::ffi::c_void,
        _close: None,
        _read: None,
        _seek: None,
        _write: None,
        _ub: crate::src::zperror::__sbuf::default(),
        _extra: 0 as * mut crate::src::zperror::__sFILEX,
        _ur: std::os::raw::c_int::default(),
        _ubuf: [std::os::raw::c_uchar::default(); 3],
        _nbuf: [std::os::raw::c_uchar::default(); 1],
        _lb: crate::src::zperror::__sbuf::default(),
        _blksize: std::os::raw::c_int::default(),
        _offset: std::os::raw::c_longlong::default()
        }
    }
}

pub type fpos_t = std::os::raw::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
impl std::default::Default for __sbuf {
    fn default() -> Self {
        __sbuf {
        _base: 0 as * mut std::os::raw::c_uchar,
        _size: std::os::raw::c_int::default()
        }
    }
}

/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zperror(mut prefix: *const std::os::raw::c_char) {
    if libzahl_error >= 0 as std::os::raw::c_int {
        *__error() = libzahl_error;
        perror(prefix);
    } else {
        let mut desc: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
        zerror(Some(&mut desc));
        if !prefix.is_null() && *prefix as std::os::raw::c_int != 0 {
            fprintf(__stderrp,
                    b"%s: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
                    prefix, desc);
        } else {
            fprintf(__stderrp,
                    b"%s\n\x00" as *const u8 as *const std::os::raw::c_char, desc);
        }
    };
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

