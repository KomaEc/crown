use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: size_t);
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
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zor(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    let mut n: size_t = 0;
    let mut m: size_t = 0;
    if zzero(b) != 0 {
        if zzero(c) != 0 {
            (*a).sign = 0 as libc::c_int;
        } else if a != c {
            zset(a, c);
        }
        return;
    } else {
        if zzero(c) != 0 {
            if a != b {
                zset(a, b);
            }
            return;
        }
    }
    m = if (*b).used > (*c).used { (*b).used } else { (*c).used };
    n = ((*b).used).wrapping_add((*c).used).wrapping_sub(m);
    if (*a).alloced < m {
        libzahl_realloc(a, m);
    }
    if a == b {
        if (*b).used < (*c).used {
            memcpy(
                ((*a).chars).offset(n as isize) as *mut libc::c_void,
                ((*c).chars).offset(n as isize) as *const libc::c_void,
                m
                    .wrapping_sub(n)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            let ref mut fresh1 = *((*a).chars).offset(n as isize);
            *fresh1 |= *((*c).chars).offset(n as isize);
        }
    } else if a == c {
        if (*c).used < (*b).used {
            memcpy(
                ((*a).chars).offset(n as isize) as *mut libc::c_void,
                ((*b).chars).offset(n as isize) as *const libc::c_void,
                m
                    .wrapping_sub(n)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        loop {
            let fresh2 = n;
            n = n.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            let ref mut fresh3 = *((*a).chars).offset(n as isize);
            *fresh3 |= *((*b).chars).offset(n as isize);
        }
    } else if m == (*b).used {
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            m.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        loop {
            let fresh4 = n;
            n = n.wrapping_sub(1);
            if !(fresh4 != 0) {
                break;
            }
            let ref mut fresh5 = *((*a).chars).offset(n as isize);
            *fresh5 |= *((*c).chars).offset(n as isize);
        }
    } else {
        memcpy(
            (*a).chars as *mut libc::c_void,
            (*c).chars as *const libc::c_void,
            m.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        loop {
            let fresh6 = n;
            n = n.wrapping_sub(1);
            if !(fresh6 != 0) {
                break;
            }
            let ref mut fresh7 = *((*a).chars).offset(n as isize);
            *fresh7 |= *((*b).chars).offset(n as isize);
        }
    }
    (*a).used = m;
    (*a)
        .sign = (zsignum(b) > 0 as libc::c_int && zsignum(c) > 0 as libc::c_int)
        as libc::c_int * 2 as libc::c_int - 1 as libc::c_int;
}
