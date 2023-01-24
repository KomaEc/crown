use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor5 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbtest(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut bit: size_t,
) -> libc::c_int {
    let mut chars: size_t = 0;
    if zzero(a) != 0 {
        return 0 as libc::c_int;
    }
    chars= bit >> 5 as libc::c_int;
    if chars >= (*a).used {
        return 0 as libc::c_int;
    }
    bit= bit & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    return (*(*a).chars.offset(chars as isize) >> bit
        & 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
