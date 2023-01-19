use ::libc;
extern "C" {

    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor31;
impl Default for ErasedByPreprocessor31 {
    fn default() -> Self {
        Self {}
    }
}

#[inline]
unsafe extern "C" fn zsignum(mut a: *const crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *const crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zor(
    mut a: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut b: *const crate::src::allocator::C2RustUnnamed,
    mut c: *const crate::src::allocator::C2RustUnnamed,
) {
    let mut n: size_t = 0;
    let mut m: size_t = 0;
    if zzero(b) != 0 {
        if zzero(c) != 0 {
            (*a.as_deref_mut().unwrap()).sign = 0 as libc::c_int;
        } else if a
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null())
            != c
        {
            crate::src::zset::zset(a.as_deref_mut(), c);
        }
        return;
    } else {
        if zzero(c) != 0 {
            if a.as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null())
                != b
            {
                crate::src::zset::zset(a.as_deref_mut(), b);
            }
            return;
        }
    }
    m = if (*b).used > (*c).used {
        (*b).used
    } else {
        (*c).used
    };
    n = (*b).used.wrapping_add((*c).used).wrapping_sub(m);
    if (*a.as_deref().unwrap()).alloced < m {
        crate::src::allocator::libzahl_realloc(a.as_deref_mut(), m);
    }
    if a.as_deref()
        .map(|r| r as *const _)
        .unwrap_or(std::ptr::null())
        == b
    {
        if (*b).used < (*c).used {
            memcpy(
                (*a.as_deref().unwrap()).chars.offset(n as isize) as *mut libc::c_void,
                (*c).chars.offset(n as isize) as *const libc::c_void,
                m.wrapping_sub(n)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            *(*a.as_deref().unwrap()).chars.offset(n as isize) = *(*c).chars.offset(n as isize);
        }
    } else if a
        .as_deref()
        .map(|r| r as *const _)
        .unwrap_or(std::ptr::null())
        == c
    {
        if (*c).used < (*b).used {
            memcpy(
                (*a.as_deref().unwrap()).chars.offset(n as isize) as *mut libc::c_void,
                (*b).chars.offset(n as isize) as *const libc::c_void,
                m.wrapping_sub(n)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
        loop {
            let fresh2 = n;
            n = n.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            *(*a.as_deref().unwrap()).chars.offset(n as isize) = *(*b).chars.offset(n as isize);
        }
    } else if m == (*b).used {
        memcpy(
            (*a.as_deref().unwrap()).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            m.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        loop {
            let fresh4 = n;
            n = n.wrapping_sub(1);
            if !(fresh4 != 0) {
                break;
            }
            *(*a.as_deref().unwrap()).chars.offset(n as isize) = *(*c).chars.offset(n as isize);
        }
    } else {
        memcpy(
            (*a.as_deref().unwrap()).chars as *mut libc::c_void,
            (*c).chars as *const libc::c_void,
            m.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        loop {
            let fresh6 = n;
            n = n.wrapping_sub(1);
            if !(fresh6 != 0) {
                break;
            }
            *(*a.as_deref().unwrap()).chars.offset(n as isize) = *(*b).chars.offset(n as isize);
        }
    }
    (*a.as_deref_mut().unwrap()).used = m;
    (*a.as_deref_mut().unwrap()).sign = (zsignum(b) > 0 as libc::c_int
        && zsignum(c) > 0 as libc::c_int) as libc::c_int
        * 2 as libc::c_int
        - 1 as libc::c_int;
}
