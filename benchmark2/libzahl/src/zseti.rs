use ::libc;
extern "C" {
    fn zsetu(_: *mut C2RustUnnamed, _: libc::c_ulonglong);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[no_mangle]
pub unsafe extern "C" fn zseti(mut a: *mut C2RustUnnamed, mut b: libc::c_longlong) {
    if b >= 0 as libc::c_int as libc::c_longlong {
        zsetu(a, b as libc::c_ulonglong);
    } else {
        zsetu(a, -b as libc::c_ulonglong);
        (*a).sign = -(1 as libc::c_int);
    };
}
