use ::libc;
extern "C" {
    
    
    
    
    
    static mut libzahl_tmp_str_div: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_tmp_str_mag: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor56 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zstr_length(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut radix: libc::c_ulonglong,
) -> size_t {
    let mut size_total = 1 as libc::c_int as size_t;
    let mut size_temp: size_t = 0;
    crate::src::zset::zset(crate::src::zstr_length::libzahl_tmp_str_num.as_mut_ptr(), a);
    while zzero(crate::src::zstr_length::libzahl_tmp_str_num.as_mut_ptr()) == 0 {
        crate::src::zsetu::zsetu(crate::src::zstr_length::libzahl_tmp_str_mag.as_mut_ptr().as_mut(), radix);
        crate::src::zset::zset(crate::src::zstr_length::libzahl_tmp_str_div.as_mut_ptr(), crate::src::zstr_length::libzahl_tmp_str_mag.as_mut_ptr());
        size_temp= 1 as libc::c_int as size_t;
        while crate::src::zcmpmag::zcmpmag(crate::src::zstr_length::libzahl_tmp_str_mag.as_mut_ptr(), crate::src::zstr_length::libzahl_tmp_str_num.as_mut_ptr())
            <= 0 as libc::c_int
        {
            crate::src::zset::zset(crate::src::zstr_length::libzahl_tmp_str_div.as_mut_ptr(), crate::src::zstr_length::libzahl_tmp_str_mag.as_mut_ptr());
            crate::src::zsqr::zsqr(crate::src::zstr_length::libzahl_tmp_str_mag.as_mut_ptr(), crate::src::zstr_length::libzahl_tmp_str_mag.as_mut_ptr());
            size_temp<<= 1 as libc::c_int;
        }
        size_temp>>= 1 as libc::c_int;
        size_total= (size_total as libc::c_ulong).wrapping_add(size_temp) as size_t
            as size_t;
        crate::src::zdiv::zdiv(
            crate::src::zstr_length::libzahl_tmp_str_num.as_mut_ptr(),
            crate::src::zstr_length::libzahl_tmp_str_num.as_mut_ptr(),
            crate::src::zstr_length::libzahl_tmp_str_div.as_mut_ptr(),
        );
    }
    return size_total
        .wrapping_add((zsignum(a) < 0 as libc::c_int) as libc::c_int as libc::c_ulong);
}
