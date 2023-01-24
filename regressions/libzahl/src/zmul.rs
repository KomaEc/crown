use ::libc;
extern "C" {
    
    
    
    
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor28 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zinit(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>) {
    (*a.as_deref_mut().unwrap()).alloced= 0 as libc::c_int as size_t;
    (*a.as_deref_mut().unwrap()).chars= 0 as *mut zahl_char_t;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zmul(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut m: size_t = 0;
    let mut m2: size_t = 0;
    let mut z0: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut z1: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut z2: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut b_high: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut b_low: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut c_high: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut c_low: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut b_sign: libc::c_int = 0;
    let mut c_sign: libc::c_int = 0;
    b_sign= zsignum(b);
    c_sign= zsignum(c);
    if b_sign == 0 || c_sign == 0 {
        (*a).sign= 0 as libc::c_int;
        return;
    }
    m= crate::src::zbits::zbits(b.as_mut());
    m2= if b == c { m } else { crate::src::zbits::zbits(c.as_mut()) };
    if m.wrapping_add(m2) <= 32 as libc::c_int as libc::c_ulong {
        if (*a).alloced < 1 as libc::c_int as libc::c_ulong {
            crate::src::allocator::libzahl_realloc(a.as_mut(), 1 as libc::c_int as size_t);
        }
        (*a).used= 1 as libc::c_int as size_t;
        *(*a).chars
            .offset(
                0 as libc::c_int as isize,
            ) = (*(*b).chars.offset(0 as libc::c_int as isize))
            .wrapping_mul(*(*c).chars.offset(0 as libc::c_int as isize));
        (*a).sign= b_sign * c_sign;
        return;
    }
    (*b).sign= 1 as libc::c_int;
    (*c).sign= 1 as libc::c_int;
    m= if m > m2 { m } else { m2 };
    m2= m >> 1 as libc::c_int;
    zinit(z0.as_mut_ptr().as_mut());
    zinit(z1.as_mut_ptr().as_mut());
    zinit(z2.as_mut_ptr().as_mut());
    zinit(b_high.as_mut_ptr().as_mut());
    zinit(b_low.as_mut_ptr().as_mut());
    zinit(c_high.as_mut_ptr().as_mut());
    zinit(c_low.as_mut_ptr().as_mut());
    crate::src::zsplit::zsplit(b_high.as_mut_ptr(), b_low.as_mut_ptr(), b, m2);
    crate::src::zsplit::zsplit(c_high.as_mut_ptr(), c_low.as_mut_ptr(), c, m2);
    zmul(z0.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zmul(z2.as_mut_ptr(), b_high.as_mut_ptr(), c_high.as_mut_ptr());
    crate::src::zadd::zadd(b_low.as_mut_ptr(), b_low.as_mut_ptr(), b_high.as_mut_ptr());
    crate::src::zadd::zadd(c_low.as_mut_ptr(), c_low.as_mut_ptr(), c_high.as_mut_ptr());
    zmul(z1.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
    crate::src::zsub::zsub(z1.as_mut_ptr(), z1.as_mut_ptr(), z0.as_mut_ptr());
    crate::src::zsub::zsub(z1.as_mut_ptr(), z1.as_mut_ptr(), z2.as_mut_ptr());
    crate::src::zlsh::zlsh(z1.as_mut_ptr(), z1.as_mut_ptr(), m2);
    m2<<= 1 as libc::c_int;
    crate::src::zlsh::zlsh(z2.as_mut_ptr(), z2.as_mut_ptr(), m2);
    crate::src::zadd::zadd(a, z2.as_mut_ptr(), z1.as_mut_ptr());
    crate::src::zadd::zadd(a, a, z0.as_mut_ptr());
    crate::src::zfree::zfree(z0.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(z1.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(z2.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(b_high.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(b_low.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(c_high.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(c_low.as_mut_ptr().as_mut());
    (*b).sign= b_sign;
    (*c).sign= c_sign;
    (*a).sign= b_sign * c_sign;
}
