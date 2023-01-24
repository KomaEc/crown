use ::libc;
extern "C" {
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor30 { dummy: () }
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn znot(mut a: *mut crate::src::allocator::C2RustUnnamed, mut b: *mut crate::src::allocator::C2RustUnnamed) {
    let mut bits: size_t = 0;
    let mut n: size_t = 0;
    if zzero(b) != 0 {
        (*a).sign= 0 as libc::c_int;
        return;
    }
    bits= crate::src::zbits::zbits(b.as_mut());
    if a != b {
        crate::src::zset::zset(a, b);
    }
    (*a).sign= -zsignum(a);
    n= (*a).used;
    loop {
        let fresh0 = n;
        n= n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        *(*a).chars.offset(n as isize) = !*(*a).chars.offset(n as isize);
    }
    bits= bits & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    if bits != 0 {
        *(*a).chars
            .offset(
                (*a).used.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) &= ((1 as libc::c_int as zahl_char_t) << bits)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
    while (*a).used != 0
        && *(*a).chars
            .offset((*a).used.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0
    {
        (*a).used= (*a).used.wrapping_sub(1);
    }
    if (*a).used == 0 {
        (*a).sign= 0 as libc::c_int;
    }
}
