
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
#[inline]
unsafe extern "C" fn zzero<'a1>(mut a: Option<&'a1 mut crate::src::allocator::zahl>) -> std::os::raw::c_int {
    return ((*borrow(& a).unwrap()).sign == 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn zsignum<'a1>(mut a: Option<&'a1 mut crate::src::allocator::zahl>) -> std::os::raw::c_int {
    return (*borrow_mut(&mut a).unwrap()).sign;
}
#[inline]
unsafe extern "C" fn zbits<'a1>(mut a: Option<&'a1 mut crate::src::allocator::zahl>) -> std::os::raw::c_ulong {
    let mut rc: size_t = 0;
    if (zzero(borrow_mut(&mut a)) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 1 as std::os::raw::c_int as size_t
    }
    while *(*borrow_mut(&mut a).unwrap()).chars.offset((*borrow_mut(&mut a).unwrap()).used.wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                                 isize) == 0 {
        (*borrow_mut(&mut a).unwrap()).used = (*borrow_mut(&mut a).unwrap()).used.wrapping_sub(1)
    }
    rc =
        (*borrow_mut(&mut a).unwrap()).used.wrapping_mul(8 as std::os::raw::c_int as
                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                   as
                                                                   std::os::raw::c_ulong);
    rc =
        (rc as
             std::os::raw::c_ulong).wrapping_sub((*(*borrow_mut(&mut a).unwrap()).chars.offset((*borrow_mut(&mut a).unwrap()).used.wrapping_sub(1
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
pub unsafe extern "C" fn znot<'a1, 'a2>(mut a: Option<&'a1 mut crate::src::allocator::zahl>, mut b: Option<&'a2 mut crate::src::allocator::zahl>) {
    let mut bits: size_t = 0;
    if (zzero(borrow_mut(&mut b)) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*borrow_mut(&mut a).unwrap()).sign = 0 as std::os::raw::c_int;
        return
    }
    bits = zbits(borrow_mut(&mut b));
    (*borrow_mut(&mut a).unwrap()).used = (*borrow_mut(&mut b).unwrap()).used;
    (*borrow_mut(&mut a).unwrap()).sign = -zsignum(borrow_mut(&mut b));
    let mut a__: *mut zahl_char_t = (*borrow_mut(&mut a).unwrap()).chars;
    let mut b__: *const zahl_char_t = (*borrow_mut(&mut b).unwrap()).chars;
    let mut i__: size_t = 0;
    let mut n__: size_t = (*borrow_mut(&mut a).unwrap()).used;
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
            *(*borrow_mut(&mut a).unwrap()).chars.offset((*borrow_mut(&mut a).unwrap()).used.wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong) as
                                   isize);
        *fresh0 &=
            ((1 as std::os::raw::c_int as zahl_char_t) <<
                 bits).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong)
    }
    while (*borrow(& a).unwrap()).used != 0 &&
              *(*borrow_mut(&mut a).unwrap()).chars.offset((*borrow_mut(&mut a).unwrap()).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                     isize) == 0 {
        (*borrow_mut(&mut a).unwrap()).used = (*borrow_mut(&mut a).unwrap()).used.wrapping_sub(1)
    }
    if (*borrow(& a).unwrap()).used == 0 { (*borrow_mut(&mut a).unwrap()).sign = 0 as std::os::raw::c_int };
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

