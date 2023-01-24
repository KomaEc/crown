use ::libc;
extern "C" {
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor6 { dummy: () }
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zcmp(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
) -> libc::c_int {
    if zsignum(a) != zsignum(b) {
        return if zsignum(a) < zsignum(b) {
            -(1 as libc::c_int)
        } else {
            (zsignum(a) > zsignum(b)) as libc::c_int
        };
    }
    return zsignum(a) * crate::src::zcmpmag::zcmpmag(a, b);
}
