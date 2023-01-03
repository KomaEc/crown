use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: size_t);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zrsh(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut bits: size_t,
) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    if bits == 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    chars = bits >> 5 as libc::c_int;
    if zzero(b) != 0 || chars >= (*b).used || zbits(b) <= bits {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    bits = bits & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    cbits = (32 as libc::c_int as libc::c_ulong).wrapping_sub(bits);
    if chars != 0 && a == b {
        let ref mut fresh0 = (*a).used;
        *fresh0 = (*fresh0 as libc::c_ulong).wrapping_sub(chars) as size_t as size_t;
        memmove(
            (*a).chars as *mut libc::c_void,
            ((*a).chars).offset(chars as isize) as *const libc::c_void,
            ((*a).used)
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    } else if a != b {
        (*a).used = ((*b).used).wrapping_sub(chars);
        if (*a).alloced < (*a).used {
            libzahl_realloc(a, (*a).used);
        }
        memcpy(
            (*a).chars as *mut libc::c_void,
            ((*b).chars).offset(chars as isize) as *const libc::c_void,
            ((*a).used)
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    if bits != 0 {
        *((*a).chars).offset(0 as libc::c_int as isize) >>= bits;
        i = 1 as libc::c_int as size_t;
        while i < (*a).used {
            let ref mut fresh1 = *((*a).chars)
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            *fresh1 |= *((*a).chars).offset(i as isize) << cbits;
            *((*a).chars).offset(i as isize) >>= bits;
            i = i.wrapping_add(1);
        }
        while *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0
        {
            let ref mut fresh2 = (*a).used;
            *fresh2 = (*fresh2).wrapping_sub(1);
        }
    }
    (*a).sign = zsignum(b);
}
