use ::libc;
extern "C" {
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zcmp(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
) -> libc::c_int {
    if zsignum(a) != zsignum(b) {
        return if zsignum(a) < zsignum(b) {
            -(1 as libc::c_int)
        } else {
            (zsignum(a) > zsignum(b)) as libc::c_int
        };
    }
    return zsignum(a) * zcmpmag(a, b);
}
