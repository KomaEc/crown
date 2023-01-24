use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor17 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zlsb(mut a: *mut crate::src::allocator::C2RustUnnamed) -> size_t {
    let mut i = 0 as libc::c_int as size_t;
    let mut x: zahl_char_t = 0;
    if zzero(a) != 0 {
        return 18446744073709551615 as libc::c_ulong;
    }
    loop {
        x= *(*a).chars.offset(i as isize);
        if x != 0 {
            x= !x;
            i= (i as libc::c_ulong).wrapping_mul(32 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            while x & 1 as libc::c_int as libc::c_uint != 0 {
                x>>= 1 as libc::c_int;
                i= i.wrapping_add(1);
            }
            return i;
        }
        i= i.wrapping_add(1);
    };
}
