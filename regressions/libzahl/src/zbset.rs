use ::libc;
extern "C" {
    
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor4 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbset(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut bit: size_t,
    mut action: libc::c_int,
) {
    let mut mask = 1 as libc::c_int as zahl_char_t;
    let mut chars: size_t = 0;
    chars= bit >> 5 as libc::c_int;
    bit= bit & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    mask<<= bit;
    if a != b {
        crate::src::zset::zset(a, b);
    }
    if action != 0 {
        if zzero(a) != 0 {
            (*a).used= 0 as libc::c_int as size_t;
            (*a).sign= 1 as libc::c_int;
        }
        if (*a).used <= chars {
            if (*a).alloced < chars.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                crate::src::allocator::libzahl_realloc(
                    a.as_mut(),
                    chars.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            memset(
                (*a).chars.offset((*a).used as isize) as *mut libc::c_void,
                0 as libc::c_int,
                chars
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub((*a).used)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
            (*a).used= chars.wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
    }
    if action > 0 as libc::c_int {
        *(*a).chars.offset(chars as isize) = mask;
        return;
    } else {
        if action < 0 as libc::c_int {
            *(*a).chars.offset(chars as isize) ^= mask;
        } else if chars < (*a).used {
            *(*a).chars.offset(chars as isize) &= !mask;
        }
    }
    while (*a).used != 0
        && *(*a).chars
            .offset((*a).used.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0
    {
        (*a).used= (*a).used.wrapping_sub(1);
    }
    if (*a).used == 0 {
        (*a).sign= 0 as libc::c_int;
    }
}
