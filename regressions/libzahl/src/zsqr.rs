use ::libc;
extern "C" {
    
    
    
    
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor52 { dummy: () }
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
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsqr(mut a: *mut crate::src::allocator::C2RustUnnamed, mut b: *mut crate::src::allocator::C2RustUnnamed) {
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
    let mut high: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut low: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut sign: libc::c_int = 0;
    if zzero(b) != 0 {
        (*a).sign= 0 as libc::c_int;
        return;
    }
    m2= crate::src::zbits::zbits(b.as_mut());
    if m2 <= (32 as libc::c_int / 2 as libc::c_int) as libc::c_ulong {
        if (*a).alloced < 1 as libc::c_int as libc::c_ulong {
            crate::src::allocator::libzahl_realloc(a.as_mut(), 1 as libc::c_int as size_t);
        }
        (*a).used= 1 as libc::c_int as size_t;
        *(*a).chars
            .offset(
                0 as libc::c_int as isize,
            ) = (*(*b).chars.offset(0 as libc::c_int as isize))
            .wrapping_mul(*(*b).chars.offset(0 as libc::c_int as isize));
        (*a).sign= 1 as libc::c_int;
        return;
    }
    sign= zsignum(b);
    (*b).sign= 1 as libc::c_int;
    m2>>= 1 as libc::c_int;
    zinit(z0.as_mut_ptr().as_mut());
    zinit(z1.as_mut_ptr().as_mut());
    zinit(z2.as_mut_ptr().as_mut());
    zinit(high.as_mut_ptr().as_mut());
    zinit(low.as_mut_ptr().as_mut());
    crate::src::zsplit::zsplit(high.as_mut_ptr(), low.as_mut_ptr(), b, m2);
    zsqr(z0.as_mut_ptr(), low.as_mut_ptr());
    zsqr(z2.as_mut_ptr(), high.as_mut_ptr());
    crate::src::zmul::zmul(z1.as_mut_ptr(), low.as_mut_ptr(), high.as_mut_ptr());
    crate::src::zlsh::zlsh(
        z1.as_mut_ptr(),
        z1.as_mut_ptr(),
        m2.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    m2<<= 1 as libc::c_int;
    crate::src::zlsh::zlsh(z2.as_mut_ptr(), z2.as_mut_ptr(), m2);
    crate::src::zadd::zadd(a, z2.as_mut_ptr(), z1.as_mut_ptr());
    crate::src::zadd::zadd(a, a, z0.as_mut_ptr());
    crate::src::zfree::zfree(z0.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(z1.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(z2.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(high.as_mut_ptr().as_mut());
    crate::src::zfree::zfree(low.as_mut_ptr().as_mut());
    (*b).sign= sign;
    (*a).sign= 1 as libc::c_int;
}
