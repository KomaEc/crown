
extern "C" {
    #[no_mangle]
    fn libzahl_realloc(_: *mut zahl, _: size_t);
}
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
unsafe extern "C" fn libzahl_memset(mut a: *mut zahl_char_t,
                                    mut v: zahl_char_t, mut n: size_t) {
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < n {
        *a.offset(i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        *a.offset(i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        *a.offset(i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        *a.offset(i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        i =
            (i as
                 std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> std::os::raw::c_int {
    return ((*a).sign == 0) as std::os::raw::c_int;
}
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_set(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as std::os::raw::c_int as zahl_char_t;
    let mut chars: size_t = bit >> 6 as std::os::raw::c_int;
    if zzero(a) != 0 {
        (*a).used = 0 as std::os::raw::c_int as size_t;
        (*a).sign = 1 as std::os::raw::c_int
    }
    if (chars >= (*a).used) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if (*a).alloced <
               chars.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
            libzahl_realloc(a,
                            chars.wrapping_add(1 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong));
        }
        libzahl_memset((*a).chars.offset((*a).used as isize),
                       0 as std::os::raw::c_int as zahl_char_t,
                       chars.wrapping_add(1 as std::os::raw::c_int as
                                              std::os::raw::c_ulong).wrapping_sub((*a).used));
        (*a).used = chars.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    bit = bit & (64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong;
    mask <<= bit;
    let ref mut fresh0 = *(*a).chars.offset(chars as isize);
    *fresh0 |= mask;
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_clear(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as std::os::raw::c_int as zahl_char_t;
    let mut chars: size_t = bit >> 6 as std::os::raw::c_int;
    if (chars >= (*a).used) as std::os::raw::c_int as std::os::raw::c_long != 0 { return }
    bit = bit & (64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong;
    mask <<= bit;
    let ref mut fresh1 = *(*a).chars.offset(chars as isize);
    *fresh1 &= !mask;
    while (*a).used != 0 &&
              *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                     isize) == 0 {
        (*a).used = (*a).used.wrapping_sub(1)
    }
    if (*a).used == 0 { (*a).sign = 0 as std::os::raw::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_flip(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as std::os::raw::c_int as zahl_char_t;
    let mut chars: size_t = bit >> 6 as std::os::raw::c_int;
    if zzero(a) != 0 {
        (*a).used = 0 as std::os::raw::c_int as size_t;
        (*a).sign = 1 as std::os::raw::c_int
    }
    if (chars >= (*a).used) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if (*a).alloced <
               chars.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
            libzahl_realloc(a,
                            chars.wrapping_add(1 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong));
        }
        libzahl_memset((*a).chars.offset((*a).used as isize),
                       0 as std::os::raw::c_int as zahl_char_t,
                       chars.wrapping_add(1 as std::os::raw::c_int as
                                              std::os::raw::c_ulong).wrapping_sub((*a).used));
        (*a).used = chars.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    bit = bit & (64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong;
    mask <<= bit;
    let ref mut fresh2 = *(*a).chars.offset(chars as isize);
    *fresh2 ^= mask;
    while (*a).used != 0 &&
              *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                     isize) == 0 {
        (*a).used = (*a).used.wrapping_sub(1)
    }
    if (*a).used == 0 { (*a).sign = 0 as std::os::raw::c_int };
}
