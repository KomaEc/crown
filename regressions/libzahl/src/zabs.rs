use ::libc;
extern "C" {
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zabs(mut a: *mut crate::src::allocator::C2RustUnnamed, mut b: *mut crate::src::allocator::C2RustUnnamed) {
    if a != b {
        crate::src::zset::zset(a, b);
    }
    (*a).sign= (zzero(a) == 0) as libc::c_int;
}
