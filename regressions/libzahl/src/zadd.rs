use ::libc;
extern "C" {
    
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zadd_unsigned(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut i: size_t = 0;
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    let mut carry: [uint32_t; 2] = [
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
    ];
    let mut addend = 0 as *mut zahl_char_t;
    if zzero(b) != 0 {
        crate::src::zabs::zabs(a, c);
        return;
    } else {
        if zzero(c) != 0 {
            crate::src::zabs::zabs(a, b);
            return;
        }
    }
    size= if (*b).used > (*c).used { (*b).used } else { (*c).used };
    n= (*b).used.wrapping_add((*c).used).wrapping_sub(size);
    if (*a).alloced < size.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        crate::src::allocator::libzahl_realloc(a.as_mut(), size.wrapping_add(1 as libc::c_int as libc::c_ulong));
    }
    *(*a).chars.offset(size as isize) = 0 as libc::c_int as zahl_char_t;
    if a == b {
        if (*a).used < (*c).used {
            n= (*c).used;
            memset(
                (*a).chars.offset((*a).used as isize) as *mut libc::c_void,
                0 as libc::c_int,
                n
                    .wrapping_sub((*a).used)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        addend= (*c).chars;
    } else if a == c {
        if (*a).used < (*b).used {
            n= (*b).used;
            memset(
                (*a).chars.offset((*a).used as isize) as *mut libc::c_void,
                0 as libc::c_int,
                n
                    .wrapping_sub((*a).used)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        addend= (*b).chars;
    } else if (*b).used > (*c).used {
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            (*b).used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        (*a).used= (*b).used;
        addend= (*c).chars;
    } else {
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*c).chars as *const libc::c_void,
            (*c).used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        (*a).used= (*c).used;
        addend= (*b).chars;
    }
    i= 0 as libc::c_int as size_t;
    while i < n {
        if carry[(i & 1 as libc::c_int as libc::c_ulong) as usize] != 0 {
            carry[(!i & 1 as libc::c_int as libc::c_ulong)
                as usize]= ((4294967295 as libc::c_uint)
                .wrapping_sub(*(*a).chars.offset(i as isize))
                <= *addend.offset(i as isize)) as libc::c_int as uint32_t;
        } else {
            carry[(!i & 1 as libc::c_int as libc::c_ulong)
                as usize]= ((4294967295 as libc::c_uint)
                .wrapping_sub(*(*a).chars.offset(i as isize))
                < *addend.offset(i as isize)) as libc::c_int as uint32_t;
        }
        *(*a).chars.offset(i as isize) = (*(*a).chars.offset(i as isize) as libc::c_uint)
            .wrapping_add(
                (*addend.offset(i as isize))
                    .wrapping_add(
                        carry[(i & 1 as libc::c_int as libc::c_ulong) as usize],
                    ),
            ) as zahl_char_t as zahl_char_t;
        i= i.wrapping_add(1);
    }
    while carry[(i & 1 as libc::c_int as libc::c_ulong) as usize] != 0 {
        carry[(!i & 1 as libc::c_int as libc::c_ulong)
            as usize]= (*(*a).chars.offset(i as isize) == 4294967295 as libc::c_uint)
            as libc::c_int as uint32_t;
        let fresh1 = i;
        i= i.wrapping_add(1);
        *(*a).chars.offset(fresh1 as isize) = (*(*a).chars.offset(fresh1 as isize) as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as zahl_char_t
            as zahl_char_t;
    }
    if (*a).used < i {
        (*a).used= i;
    }
    (*a).sign= 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zadd(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    if zzero(b) != 0 {
        if a != c {
            crate::src::zset::zset(a, c);
        }
    } else if zzero(c) != 0 {
        if a != b {
            crate::src::zset::zset(a, b);
        }
    } else if b == c {
        crate::src::zlsh::zlsh(a, b, 1 as libc::c_int as size_t);
    } else if zsignum(b) | zsignum(c) < 0 as libc::c_int {
        if zsignum(b) < 0 as libc::c_int {
            if zsignum(c) < 0 as libc::c_int {
                zadd_unsigned(a, b, c);
                (*a).sign= -zsignum(a);
            } else {
                crate::src::zsub::zsub_unsigned(a, c, b);
            }
        } else {
            crate::src::zsub::zsub_unsigned(a, b, c);
        }
    } else {
        zadd_unsigned(a, b, c);
    };
}
