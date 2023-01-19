use ::libc;
extern "C" {
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0;
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zabs(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>, mut b: *mut crate::src::allocator::C2RustUnnamed) {
    if a.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()) != b {
        crate::src::zset::zset(a.as_deref_mut(), b);
    }
    (*a.as_deref_mut().unwrap()).sign= (zzero(a.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) == 0) as libc::c_int;
}
