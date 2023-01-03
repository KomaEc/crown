use ::libc;
extern "C" {
    fn zfree(_: *mut C2RustUnnamed);
    fn zadd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
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
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsqr(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
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
    let mut high: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut low: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut sign: libc::c_int = 0;
    if zzero(b) != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    m2 = zbits(b);
    if m2 <= (32 as libc::c_int / 2 as libc::c_int) as libc::c_ulong {
        if (*a).alloced < 1 as libc::c_int as libc::c_ulong {
            libzahl_realloc(a, 1 as libc::c_int as size_t);
        }
        (*a).used = 1 as libc::c_int as size_t;
        *((*a).chars)
            .offset(
                0 as libc::c_int as isize,
            ) = (*((*b).chars).offset(0 as libc::c_int as isize))
            .wrapping_mul(*((*b).chars).offset(0 as libc::c_int as isize));
        (*a).sign = 1 as libc::c_int;
        return;
    }
    sign = zsignum(b);
    (*b).sign = 1 as libc::c_int;
    m2 >>= 1 as libc::c_int;
    zinit(z0.as_mut_ptr());
    zinit(z1.as_mut_ptr());
    zinit(z2.as_mut_ptr());
    zinit(high.as_mut_ptr());
    zinit(low.as_mut_ptr());
    zsplit(high.as_mut_ptr(), low.as_mut_ptr(), b, m2);
    zsqr(z0.as_mut_ptr(), low.as_mut_ptr());
    zsqr(z2.as_mut_ptr(), high.as_mut_ptr());
    zmul(z1.as_mut_ptr(), low.as_mut_ptr(), high.as_mut_ptr());
    zlsh(
        z1.as_mut_ptr(),
        z1.as_mut_ptr(),
        m2.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    m2 <<= 1 as libc::c_int;
    zlsh(z2.as_mut_ptr(), z2.as_mut_ptr(), m2);
    zadd(a, z2.as_mut_ptr(), z1.as_mut_ptr());
    zadd(a, a, z0.as_mut_ptr());
    zfree(z0.as_mut_ptr());
    zfree(z1.as_mut_ptr());
    zfree(z2.as_mut_ptr());
    zfree(high.as_mut_ptr());
    zfree(low.as_mut_ptr());
    (*b).sign = sign;
    (*a).sign = 1 as libc::c_int;
}
