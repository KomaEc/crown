use ::libc;
extern "C" {

    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct OrcGeneratedXXX4;
impl Default for OrcGeneratedXXX4 {
    fn default() -> Self {
        Self {}
    }
}

#[inline]
unsafe extern "C" fn zzero(mut a: *const crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbset(
    mut a: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut b: *const crate::src::allocator::C2RustUnnamed,
    mut bit: size_t,
    mut action: libc::c_int,
) {
    let mut mask = 1 as libc::c_int as zahl_char_t;
    let mut chars: size_t = 0;
    chars = bit >> 5 as libc::c_int;
    bit = bit & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    mask <<= bit;
    if a.as_deref()
        .map(|r| r as *const _)
        .unwrap_or(std::ptr::null())
        != b
    {
        crate::src::zset::zset(a.as_deref_mut(), b);
    }
    if action != 0 {
        if zzero(
            a.as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ) != 0
        {
            (*a.as_deref_mut().unwrap()).used = 0 as libc::c_int as size_t;
            (*a.as_deref_mut().unwrap()).sign = 1 as libc::c_int;
        }
        if (*a.as_deref().unwrap()).used <= chars {
            if (*a.as_deref().unwrap()).alloced
                < chars.wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                crate::src::allocator::libzahl_realloc(
                    a.as_deref_mut(),
                    chars.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            memset(
                (*a.as_deref().unwrap())
                    .chars
                    .offset((*a.as_deref().unwrap()).used as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                chars
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub((*a.as_deref().unwrap()).used)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
            (*a.as_deref_mut().unwrap()).used =
                chars.wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
    }
    if action > 0 as libc::c_int {
        *(*a.as_deref().unwrap()).chars.offset(chars as isize) = mask;
        return;
    } else {
        if action < 0 as libc::c_int {
            *(*a.as_deref().unwrap()).chars.offset(chars as isize) ^= mask;
        } else if chars < (*a.as_deref().unwrap()).used {
            *(*a.as_deref().unwrap()).chars.offset(chars as isize) &= !mask;
        }
    }
    while (*a.as_deref().unwrap()).used != 0
        && *(*a.as_deref().unwrap()).chars.offset(
            (*a.as_deref().unwrap())
                .used
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) == 0
    {
        (*a.as_deref_mut().unwrap()).used = (*a.as_deref().unwrap()).used.wrapping_sub(1);
    }
    if (*a.as_deref().unwrap()).used == 0 {
        (*a.as_deref_mut().unwrap()).sign = 0 as libc::c_int;
    }
}
