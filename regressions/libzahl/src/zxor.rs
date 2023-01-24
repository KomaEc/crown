use ::libc;
extern "C" {
    
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor61 { dummy: () }
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zxor(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut n: size_t = 0;
    let mut m: size_t = 0;
    if zzero(b) != 0 {
        if zzero(c) != 0 {
            (*a).sign= 0 as libc::c_int;
        } else if a != c {
            crate::src::zset::zset(a, c);
        }
        return;
    } else {
        if zzero(c) != 0 {
            if a != b {
                crate::src::zset::zset(a, b);
            }
            return;
        }
    }
    m= if (*b).used > (*c).used { (*b).used } else { (*c).used };
    n= (*b).used.wrapping_add((*c).used).wrapping_sub(m);
    if (*a).alloced < m {
        crate::src::allocator::libzahl_realloc(a.as_mut(), m);
    }
    if a == b {
        if (*b).used < (*c).used {
            memcpy(
                (*a).chars.offset(n as isize) as *mut libc::c_void,
                (*c).chars.offset(n as isize) as *const libc::c_void,
                m
                    .wrapping_sub(n)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        loop {
            let fresh0 = n;
            n= n.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            *(*a).chars.offset(n as isize) ^= *(*c).chars.offset(n as isize);
        }
    } else if a == c {
        if (*c).used < (*b).used {
            memcpy(
                (*a).chars.offset(n as isize) as *mut libc::c_void,
                (*b).chars.offset(n as isize) as *const libc::c_void,
                m
                    .wrapping_sub(n)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        loop {
            let fresh2 = n;
            n= n.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            *(*a).chars.offset(n as isize) ^= *(*b).chars.offset(n as isize);
        }
    } else if m == (*b).used {
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            m.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        loop {
            let fresh4 = n;
            n= n.wrapping_sub(1);
            if !(fresh4 != 0) {
                break;
            }
            *(*a).chars.offset(n as isize) ^= *(*c).chars.offset(n as isize);
        }
    } else {
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*c).chars as *const libc::c_void,
            m.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        loop {
            let fresh6 = n;
            n= n.wrapping_sub(1);
            if !(fresh6 != 0) {
                break;
            }
            *(*a).chars.offset(n as isize) ^= *(*b).chars.offset(n as isize);
        }
    }
    (*a).used= m;
    while (*a).used != 0
        && *(*a).chars
            .offset((*a).used.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0
    {
        (*a).used= (*a).used.wrapping_sub(1);
    }
    if (*a).used != 0 {
        (*a).sign= 1 as libc::c_int
            - 2 as libc::c_int
                * (zsignum(b) ^ zsignum(c) < 0 as libc::c_int) as libc::c_int;
    } else {
        (*a).sign= 0 as libc::c_int;
    };
}
