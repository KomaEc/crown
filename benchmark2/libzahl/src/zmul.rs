use ::libc;
extern "C" {
    fn zfree(_: *mut C2RustUnnamed);
    fn zadd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsub(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zlsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zsplit(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: size_t,
    );
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: size_t);
    fn zbits(_: *mut C2RustUnnamed) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zinit(mut a: *mut C2RustUnnamed) {
    (*a).alloced = 0 as libc::c_int as size_t;
    let ref mut fresh0 = (*a).chars;
    *fresh0 = 0 as *mut zahl_char_t;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zmul(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    let mut m: size_t = 0;
    let mut m2: size_t = 0;
    let mut z0: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut z1: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut z2: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut b_high: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut b_low: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut c_high: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut c_low: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut b_sign: libc::c_int = 0;
    let mut c_sign: libc::c_int = 0;
    b_sign = zsignum(b);
    c_sign = zsignum(c);
    if b_sign == 0 || c_sign == 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    m = zbits(b);
    m2 = if b == c { m } else { zbits(c) };
    if m.wrapping_add(m2) <= 32 as libc::c_int as libc::c_ulong {
        if (*a).alloced < 1 as libc::c_int as libc::c_ulong {
            libzahl_realloc(a, 1 as libc::c_int as size_t);
        }
        (*a).used = 1 as libc::c_int as size_t;
        *((*a).chars)
            .offset(
                0 as libc::c_int as isize,
            ) = (*((*b).chars).offset(0 as libc::c_int as isize))
            .wrapping_mul(*((*c).chars).offset(0 as libc::c_int as isize));
        (*a).sign = b_sign * c_sign;
        return;
    }
    (*b).sign = 1 as libc::c_int;
    (*c).sign = 1 as libc::c_int;
    m = if m > m2 { m } else { m2 };
    m2 = m >> 1 as libc::c_int;
    zinit(z0.as_mut_ptr());
    zinit(z1.as_mut_ptr());
    zinit(z2.as_mut_ptr());
    zinit(b_high.as_mut_ptr());
    zinit(b_low.as_mut_ptr());
    zinit(c_high.as_mut_ptr());
    zinit(c_low.as_mut_ptr());
    zsplit(b_high.as_mut_ptr(), b_low.as_mut_ptr(), b, m2);
    zsplit(c_high.as_mut_ptr(), c_low.as_mut_ptr(), c, m2);
    zmul(z0.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zmul(z2.as_mut_ptr(), b_high.as_mut_ptr(), c_high.as_mut_ptr());
    zadd(b_low.as_mut_ptr(), b_low.as_mut_ptr(), b_high.as_mut_ptr());
    zadd(c_low.as_mut_ptr(), c_low.as_mut_ptr(), c_high.as_mut_ptr());
    zmul(z1.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zsub(z1.as_mut_ptr(), z1.as_mut_ptr(), z0.as_mut_ptr());
    zsub(z1.as_mut_ptr(), z1.as_mut_ptr(), z2.as_mut_ptr());
    zlsh(z1.as_mut_ptr(), z1.as_mut_ptr(), m2);
    m2 <<= 1 as libc::c_int;
    zlsh(z2.as_mut_ptr(), z2.as_mut_ptr(), m2);
    zadd(a, z2.as_mut_ptr(), z1.as_mut_ptr());
    zadd(a, a, z0.as_mut_ptr());
    zfree(z0.as_mut_ptr());
    zfree(z1.as_mut_ptr());
    zfree(z2.as_mut_ptr());
    zfree(b_high.as_mut_ptr());
    zfree(b_low.as_mut_ptr());
    zfree(c_high.as_mut_ptr());
    zfree(c_low.as_mut_ptr());
    (*b).sign = b_sign;
    (*c).sign = c_sign;
    (*a).sign = b_sign * c_sign;
}
