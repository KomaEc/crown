use ::libc;
extern "C" {
    
    
    static mut libzahl_tmp_sub: z_t;
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor57 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zsub_unsigned(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut carry: [zahl_char_t; 2] = [
        0 as libc::c_int as zahl_char_t,
        0 as libc::c_int as zahl_char_t,
    ];
    let mut s = 0 as *mut zahl_char_t;
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut magcmp: libc::c_int = 0;
    if zzero(b) != 0 {
        crate::src::zabs::zabs(a, c);
        crate::src::zneg::zneg(a, a);
        return;
    } else {
        if zzero(c) != 0 {
            crate::src::zabs::zabs(a, b);
            return;
        }
    }
    magcmp= crate::src::zcmpmag::zcmpmag(b, c);
    if magcmp <= 0 as libc::c_int {
        if magcmp == 0 as libc::c_int {
            (*a).sign= 0 as libc::c_int;
            return;
        }
        n= if (*b).used < (*c).used { (*b).used } else { (*c).used };
        if a == b {
            crate::src::zset::zset(crate::src::zsub::libzahl_tmp_sub.as_mut_ptr(), b);
            s= (*crate::src::zsub::libzahl_tmp_sub.as_mut_ptr()).chars;
        } else {
            s= (*b).chars;
        }
        if a != c {
            crate::src::zset::zset(a, c);
        }
    } else {
        n= if (*b).used < (*c).used { (*b).used } else { (*c).used };
        if a == c {
            crate::src::zset::zset(crate::src::zsub::libzahl_tmp_sub.as_mut_ptr(), c);
            s= (*crate::src::zsub::libzahl_tmp_sub.as_mut_ptr()).chars;
        } else {
            s= (*c).chars;
        }
        if a != b {
            crate::src::zset::zset(a, b);
        }
    }
    i= 0 as libc::c_int as size_t;
    while i < n {
        carry[(!i & 1 as libc::c_int as libc::c_ulong)
            as usize]= (if carry[(i & 1 as libc::c_int as libc::c_ulong) as usize] != 0
        {
            (*(*a).chars.offset(i as isize) <= *s.offset(i as isize)) as libc::c_int
        } else {
            (*(*a).chars.offset(i as isize) < *s.offset(i as isize)) as libc::c_int
        }) as zahl_char_t;
        *(*a).chars.offset(i as isize) = (*(*a).chars.offset(i as isize) as libc::c_uint).wrapping_sub(*s.offset(i as isize))
            as zahl_char_t as zahl_char_t;
        *(*a).chars.offset(i as isize) = (*(*a).chars.offset(i as isize) as libc::c_uint)
            .wrapping_sub(carry[(i & 1 as libc::c_int as libc::c_ulong) as usize])
            as zahl_char_t as zahl_char_t;
        i= i.wrapping_add(1);
    }
    if carry[(i & 1 as libc::c_int as libc::c_ulong) as usize] != 0 {
        while *(*a).chars.offset(i as isize) == 0 {
            let fresh2 = i;
            i= i.wrapping_add(1);
            *(*a).chars.offset(fresh2 as isize) = 4294967295 as libc::c_uint;
        }
        *(*a).chars.offset(i as isize) = (*(*a).chars.offset(i as isize) as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as zahl_char_t
            as zahl_char_t;
    }
    (*a).sign= magcmp;
}
#[no_mangle]
pub unsafe extern "C" fn zsub(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    if b == c {
        (*a).sign= 0 as libc::c_int;
    } else if zzero(b) != 0 {
        crate::src::zneg::zneg(a, c);
    } else if zzero(c) != 0 {
        if a != b {
            crate::src::zset::zset(a, b);
        }
    } else if zsignum(b) | zsignum(c) < 0 as libc::c_int {
        if zsignum(b) < 0 as libc::c_int {
            if zsignum(c) < 0 as libc::c_int {
                zsub_unsigned(a, c, b);
            } else {
                crate::src::zadd::zadd_unsigned(a, b, c);
                (*a).sign= -zsignum(a);
            }
        } else {
            crate::src::zadd::zadd_unsigned(a, b, c);
        }
    } else {
        zsub_unsigned(a, b, c);
    };
}
