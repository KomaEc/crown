use ::libc;
extern "C" {
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn ztrunc(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
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
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsplit(
    mut high: *mut C2RustUnnamed,
    mut low: *mut C2RustUnnamed,
    mut a: *mut C2RustUnnamed,
    mut delim: size_t,
) {
    if zzero(a) != 0 {
        (*high).sign = 0 as libc::c_int;
        (*low).sign = 0 as libc::c_int;
        return;
    }
    if high == a {
        ztrunc(low, a, delim);
        zrsh(high, a, delim);
    } else {
        zrsh(high, a, delim);
        ztrunc(low, a, delim);
    };
}
