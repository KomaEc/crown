use ::libc;
extern "C" {
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[no_mangle]
pub unsafe extern "C" fn zsetu(mut a: *mut C2RustUnnamed, mut b: libc::c_ulonglong) {
    if b == 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    if (*a).alloced
        < (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
    {
        libzahl_realloc(
            a,
            (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    (*a).sign = 1 as libc::c_int;
    (*a).used = 0 as libc::c_int as size_t;
    while b != 0 {
        let ref mut fresh0 = (*a).used;
        let fresh1 = *fresh0;
        *fresh0 = (*fresh0).wrapping_add(1);
        *((*a).chars).offset(fresh1 as isize) = b as zahl_char_t;
        b >>= 32 as libc::c_int;
    }
}
