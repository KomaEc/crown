use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    static mut libzahl_tmp_sub: z_t;
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zneg(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zadd_unsigned(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
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
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zsub_unsigned(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
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
        zabs(a, c);
        zneg(a, a);
        return;
    } else {
        if zzero(c) != 0 {
            zabs(a, b);
            return;
        }
    }
    magcmp = zcmpmag(b, c);
    if magcmp <= 0 as libc::c_int {
        if magcmp == 0 as libc::c_int {
            (*a).sign = 0 as libc::c_int;
            return;
        }
        n = if (*b).used < (*c).used { (*b).used } else { (*c).used };
        if a == b {
            zset(libzahl_tmp_sub.as_mut_ptr(), b);
            s = (*libzahl_tmp_sub.as_mut_ptr()).chars;
        } else {
            s = (*b).chars;
        }
        if a != c {
            zset(a, c);
        }
    } else {
        n = if (*b).used < (*c).used { (*b).used } else { (*c).used };
        if a == c {
            zset(libzahl_tmp_sub.as_mut_ptr(), c);
            s = (*libzahl_tmp_sub.as_mut_ptr()).chars;
        } else {
            s = (*c).chars;
        }
        if a != b {
            zset(a, b);
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        carry[(!i & 1 as libc::c_int as libc::c_ulong)
            as usize] = (if carry[(i & 1 as libc::c_int as libc::c_ulong) as usize] != 0
        {
            (*((*a).chars).offset(i as isize) <= *s.offset(i as isize)) as libc::c_int
        } else {
            (*((*a).chars).offset(i as isize) < *s.offset(i as isize)) as libc::c_int
        }) as zahl_char_t;
        let ref mut fresh0 = *((*a).chars).offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_uint).wrapping_sub(*s.offset(i as isize))
            as zahl_char_t as zahl_char_t;
        let ref mut fresh1 = *((*a).chars).offset(i as isize);
        *fresh1 = (*fresh1 as libc::c_uint)
            .wrapping_sub(carry[(i & 1 as libc::c_int as libc::c_ulong) as usize])
            as zahl_char_t as zahl_char_t;
        i = i.wrapping_add(1);
    }
    if carry[(i & 1 as libc::c_int as libc::c_ulong) as usize] != 0 {
        while *((*a).chars).offset(i as isize) == 0 {
            let fresh2 = i;
            i = i.wrapping_add(1);
            *((*a).chars).offset(fresh2 as isize) = 4294967295 as libc::c_uint;
        }
        let ref mut fresh3 = *((*a).chars).offset(i as isize);
        *fresh3 = (*fresh3 as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as zahl_char_t
            as zahl_char_t;
    }
    (*a).sign = magcmp;
}
#[no_mangle]
pub unsafe extern "C" fn zsub(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    if b == c {
        (*a).sign = 0 as libc::c_int;
    } else if zzero(b) != 0 {
        zneg(a, c);
    } else if zzero(c) != 0 {
        if a != b {
            zset(a, b);
        }
    } else if zsignum(b) | zsignum(c) < 0 as libc::c_int {
        if zsignum(b) < 0 as libc::c_int {
            if zsignum(c) < 0 as libc::c_int {
                zsub_unsigned(a, c, b);
            } else {
                zadd_unsigned(a, b, c);
                (*a).sign = -zsignum(a);
            }
        } else {
            zadd_unsigned(a, b, c);
        }
    } else {
        zsub_unsigned(a, b, c);
    };
}
