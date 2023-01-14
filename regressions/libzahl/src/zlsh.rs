use ::libc;
extern "C" {
    
    
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct OrcGeneratedXXX18;
impl Default for OrcGeneratedXXX18 {fn default() -> Self {Self {
}}}

#[inline]
unsafe extern "C" fn zsignum(mut a: *const crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *const crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zlsh(
    mut a: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut b: *const crate::src::allocator::C2RustUnnamed,
    mut bits: size_t,
) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    let mut carry: [zahl_char_t; 2] = [
        0 as libc::c_int as zahl_char_t,
        0 as libc::c_int as zahl_char_t,
    ];
    if zzero(b) != 0 {
        (*a.as_deref_mut().unwrap()).sign= 0 as libc::c_int;
        return;
    }
    if bits == 0 {
        if a.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()) != b {
            crate::src::zset::zset(a.as_deref_mut(), b);
        }
        return;
    }
    chars= bits >> 5 as libc::c_int;
    bits= bits & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    cbits= (32 as libc::c_int as libc::c_ulong).wrapping_sub(bits);
    if (*a.as_deref().unwrap()).alloced < (*b).used.wrapping_add(chars) {
        crate::src::allocator::libzahl_realloc(a.as_deref_mut(), (*b).used.wrapping_add(chars));
    }
    if a.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()) == b {
        memmove(
            (*a.as_deref().unwrap()).chars.offset(chars as isize) as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            (*b).used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    } else {
        memcpy(
            (*a.as_deref().unwrap()).chars.offset(chars as isize) as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            (*b).used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    memset(
        (*a.as_deref().unwrap()).chars as *mut libc::c_void,
        0 as libc::c_int,
        chars.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
    );
    (*a.as_deref_mut().unwrap()).used= (*b).used.wrapping_add(chars);
    if bits != 0 {
        i= chars;
        while i < (*a.as_deref().unwrap()).used {
            carry[(!i & 1 as libc::c_int as libc::c_ulong)
                as usize] = *(*a.as_deref().unwrap()).chars.offset(i as isize) >> cbits;
            *(*a.as_deref().unwrap()).chars.offset(i as isize) <<= bits;
            *(*a.as_deref().unwrap()).chars.offset(i as isize) = carry[(i & 1 as libc::c_int as libc::c_ulong) as usize];
            i= i.wrapping_add(1);
        }
        if carry[(i & 1 as libc::c_int as libc::c_ulong) as usize] != 0 {
            if (*a.as_deref().unwrap()).alloced < (*a.as_deref().unwrap()).used.wrapping_add(1 as libc::c_int as libc::c_ulong)
            {
                crate::src::allocator::libzahl_realloc(
                    a.as_deref_mut(),
                    (*a.as_deref().unwrap()).used.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            *(*a.as_deref().unwrap()).chars
                .offset(
                    i as isize,
                ) = carry[(i & 1 as libc::c_int as libc::c_ulong) as usize];
            (*a.as_deref_mut().unwrap()).used= (*a.as_deref().unwrap()).used.wrapping_add(1);
        }
    }
    (*a.as_deref_mut().unwrap()).sign= zsignum(b);
}
