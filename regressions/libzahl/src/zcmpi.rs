use ::libc;
extern "C" {
    
    
    static mut libzahl_tmp_cmp: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zcmpi(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: libc::c_longlong,
) -> libc::c_int {
    if b == 0 {
        return zsignum(a);
    }
    if zzero(a) != 0 {
        return if b > 0 as libc::c_int as libc::c_longlong {
            -(1 as libc::c_int)
        } else {
            (b < 0 as libc::c_int as libc::c_longlong) as libc::c_int
        };
    }
    crate::src::zseti::zseti(crate::src::zcmpi::libzahl_tmp_cmp.as_mut_ptr().as_mut(), b);
    return crate::src::zcmp::zcmp(a, crate::src::zcmpi::libzahl_tmp_cmp.as_mut_ptr());
}
