use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor8 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zcmpmag(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if zzero(a) != 0 {
        return -((zzero(b) == 0) as libc::c_int);
    }
    if zzero(b) != 0 {
        return 1 as libc::c_int;
    }
    i= (*a).used.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    j= (*b).used.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > j {
        if *(*a).chars.offset(i as isize) != 0 {
            return 1 as libc::c_int;
        }
        (*a).used= (*a).used.wrapping_sub(1);
        i= i.wrapping_sub(1);
    }
    while j > i {
        if *(*b).chars.offset(j as isize) != 0 {
            return -(1 as libc::c_int);
        }
        (*b).used= (*b).used.wrapping_sub(1);
        j= j.wrapping_sub(1);
    }
    while i != 0 {
        if *(*a).chars.offset(i as isize) != *(*b).chars.offset(i as isize) {
            return (*(*a).chars.offset(i as isize) > *(*b).chars.offset(i as isize))
                as libc::c_int * 2 as libc::c_int - 1 as libc::c_int;
        }
        i= i.wrapping_sub(1);
    }
    return if *(*a).chars.offset(0 as libc::c_int as isize)
        < *(*b).chars.offset(0 as libc::c_int as isize)
    {
        -(1 as libc::c_int)
    } else {
        (*(*a).chars.offset(0 as libc::c_int as isize)
            > *(*b).chars.offset(0 as libc::c_int as isize)) as libc::c_int
    };
}
