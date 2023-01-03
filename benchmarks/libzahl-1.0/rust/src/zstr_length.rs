use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsetu(_: *mut C2RustUnnamed, _: libc::c_ulonglong);
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    fn zdiv(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_str_div: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_tmp_str_mag: z_t;
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
pub type z_t = [C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zstr_length(
    mut a: *mut C2RustUnnamed,
    mut radix: libc::c_ulonglong,
) -> size_t {
    let mut size_total = 1 as libc::c_int as size_t;
    let mut size_temp: size_t = 0;
    zset(libzahl_tmp_str_num.as_mut_ptr(), a);
    while zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0 {
        zsetu(libzahl_tmp_str_mag.as_mut_ptr(), radix);
        zset(libzahl_tmp_str_div.as_mut_ptr(), libzahl_tmp_str_mag.as_mut_ptr());
        size_temp = 1 as libc::c_int as size_t;
        while zcmpmag(libzahl_tmp_str_mag.as_mut_ptr(), libzahl_tmp_str_num.as_mut_ptr())
            <= 0 as libc::c_int
        {
            zset(libzahl_tmp_str_div.as_mut_ptr(), libzahl_tmp_str_mag.as_mut_ptr());
            zsqr(libzahl_tmp_str_mag.as_mut_ptr(), libzahl_tmp_str_mag.as_mut_ptr());
            size_temp <<= 1 as libc::c_int;
        }
        size_temp >>= 1 as libc::c_int;
        size_total = (size_total as libc::c_ulong).wrapping_add(size_temp) as size_t
            as size_t;
        zdiv(
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_tmp_str_div.as_mut_ptr(),
        );
    }
    return size_total
        .wrapping_add((zsignum(a) < 0 as libc::c_int) as libc::c_int as libc::c_ulong);
}
