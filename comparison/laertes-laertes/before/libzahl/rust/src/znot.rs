
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;
pub type uint64_t = std::os::raw::c_ulonglong;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: std::os::raw::c_int,
    pub padding__: std::os::raw::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> std::os::raw::c_int {
    return ((*a).sign == 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> std::os::raw::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zbits(mut a: *mut zahl) -> size_t {
    let mut rc: size_t = 0;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 1 as std::os::raw::c_int as size_t
    }
    while *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                                 isize) == 0 {
        (*a).used = (*a).used.wrapping_sub(1)
    }
    rc =
        (*a).used.wrapping_mul(8 as std::os::raw::c_int as
                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                   as
                                                                   std::os::raw::c_ulong);
    rc =
        (rc as
             std::os::raw::c_ulong).wrapping_sub((*(*a).chars.offset((*a).used.wrapping_sub(1
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        std::os::raw::c_ulong)
                                                                 as
                                                                 isize)).leading_zeros()
                                             as i32 as size_t) as size_t as
            size_t;
    return rc;
}
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn znot(mut a: *mut zahl, mut b: *mut zahl) {
    let mut bits: size_t = 0;
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int;
        return
    }
    bits = zbits(b);
    (*a).used = (*b).used;
    (*a).sign = -zsignum(b);
    let mut a__: *mut zahl_char_t = (*a).chars;
    let mut b__: *const zahl_char_t = (*b).chars;
    let mut i__: size_t = 0;
    let mut n__: size_t = (*a).used;
    i__ = 0 as std::os::raw::c_int as size_t;
    while i__ < n__ {
        *a__.offset(i__.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as
                        isize) =
            !*b__.offset(i__.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                             as isize);
        *a__.offset(i__.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
                        isize) =
            !*b__.offset(i__.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                             as isize);
        *a__.offset(i__.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                        isize) =
            !*b__.offset(i__.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                             as isize);
        *a__.offset(i__.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as
                        isize) =
            !*b__.offset(i__.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                             as isize);
        i__ =
            (i__ as
                 std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t
    }
    bits = bits & (64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong;
    if bits != 0 {
        let ref mut fresh0 =
            *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) as
                                   isize);
        *fresh0 &=
            ((1 as std::os::raw::c_int as zahl_char_t) <<
                 bits).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong)
    }
    while (*a).used != 0 &&
              *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                     isize) == 0 {
        (*a).used = (*a).used.wrapping_sub(1)
    }
    if (*a).used == 0 { (*a).sign = 0 as std::os::raw::c_int };
}
