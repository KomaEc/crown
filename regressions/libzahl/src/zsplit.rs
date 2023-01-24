use ::libc;
extern "C" {
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor51 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsplit(
    mut high: *mut crate::src::allocator::C2RustUnnamed,
    mut low: *mut crate::src::allocator::C2RustUnnamed,
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut delim: size_t,
) {
    if zzero(a) != 0 {
        (*high).sign= 0 as libc::c_int;
        (*low).sign= 0 as libc::c_int;
        return;
    }
    if high == a {
        crate::src::ztrunc::ztrunc(low, a, delim);
        crate::src::zrsh::zrsh(high, a, delim);
    } else {
        crate::src::zrsh::zrsh(high, a, delim);
        crate::src::ztrunc::ztrunc(low, a, delim);
    };
}
