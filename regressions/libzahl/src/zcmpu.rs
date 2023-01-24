use ::libc;
extern "C" {
    
    
    static mut libzahl_tmp_cmp: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor9 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zcmpu(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: libc::c_ulonglong,
) -> libc::c_int {
    if b == 0 {
        return zsignum(a);
    }
    if zsignum(a) <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    crate::src::zsetu::zsetu(crate::src::zcmpu::libzahl_tmp_cmp.as_mut_ptr().as_mut(), b);
    return crate::src::zcmp::zcmp(a, crate::src::zcmpu::libzahl_tmp_cmp.as_mut_ptr());
}
