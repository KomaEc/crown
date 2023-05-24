
extern "C" {
    
    
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
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
pub fn borrow<'a, 'b: 'a, T>(p: &'a Option<&'b mut T>) -> Option<&'a T> {
    p.as_ref().map(|x| &**x)
}

pub fn borrow_mut<'a, 'b : 'a, T>(p: &'a mut Option<&'b mut T>) -> Option<&'a mut T> {
    p.as_mut().map(|x| &mut **x)
}

pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
    p.as_ref().map(|x| x.as_ref())
}

pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
    p.as_mut().map(|x| x.as_mut())
}

pub fn option_to_raw<T>(p: Option<&T>) -> * const T {
    p.map_or(core::ptr::null(), |p| p as * const T)
}

pub fn _ref_eq<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) == option_to_raw(q)
}

pub fn _ref_ne<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) != option_to_raw(q)
}

