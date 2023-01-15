use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct OrcGeneratedXXX59;
impl Default for OrcGeneratedXXX59 {
    fn default() -> Self {
        Self {}
    }
}

#[inline]
unsafe extern "C" fn zzero(mut a: *const crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ztrunc(
    mut a: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut b: *const crate::src::allocator::C2RustUnnamed,
    mut bits: size_t,
) {
    let mut mask = 1 as libc::c_int as zahl_char_t;
    let mut chars: size_t = 0;
    let mut i: size_t = 0;
    if zzero(b) != 0 {
        (*a.as_deref_mut().unwrap()).sign = 0 as libc::c_int;
        return;
    }
    chars = bits.wrapping_add((32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        >> 5 as libc::c_int;
    (*a.as_deref_mut().unwrap()).sign = (*b).sign;
    (*a.as_deref_mut().unwrap()).used = if chars < (*b).used { chars } else { (*b).used };
    if (*a.as_deref().unwrap()).used < chars {
        bits = 0 as libc::c_int as size_t;
    }
    if a.as_deref()
        .map(|r| r as *const _)
        .unwrap_or(std::ptr::null())
        != b
    {
        if (*a.as_deref().unwrap()).alloced < (*a.as_deref().unwrap()).used {
            crate::src::allocator::libzahl_realloc(a.as_deref_mut(), (*a.as_deref().unwrap()).used);
        }
        memcpy(
            (*a.as_deref().unwrap()).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            (*a.as_deref().unwrap())
                .used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    bits = bits & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    if bits != 0 {
        mask <<= bits;
        mask = (mask as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint) as zahl_char_t
            as zahl_char_t;
        *(*a.as_deref().unwrap()).chars.offset(
            (*a.as_deref().unwrap())
                .used
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) &= mask;
    }
    i = (*a.as_deref().unwrap()).used;
    loop {
        let fresh1 = i;
        i = i.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        if *(*a.as_deref().unwrap()).chars.offset(i as isize) != 0 {
            return;
        }
    }
    (*a.as_deref_mut().unwrap()).sign = 0 as libc::c_int;
}
