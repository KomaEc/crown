use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: size_t);
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
pub unsafe extern "C" fn zset(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
    if zzero(b) != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        if (*a).alloced < (*b).used {
            libzahl_realloc(a, (*b).used);
        }
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            ((*b).used)
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    };
}
