use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;

}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct OrcGeneratedXXX44;
impl Default for OrcGeneratedXXX44 {
    fn default() -> Self {
        Self {}
    }
}

#[inline]
unsafe extern "C" fn zzero(mut a: *const crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zset(
    mut a: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut b: *const crate::src::allocator::C2RustUnnamed,
) {
    if zzero(b) != 0 {
        (*a.as_deref_mut().unwrap()).sign = 0 as libc::c_int;
    } else {
        if (*a.as_deref().unwrap()).alloced < (*b).used {
            crate::src::allocator::libzahl_realloc(a.as_deref_mut(), (*b).used);
        }
        (*a.as_deref_mut().unwrap()).sign = (*b).sign;
        (*a.as_deref_mut().unwrap()).used = (*b).used;
        memcpy(
            (*a.as_deref().unwrap()).chars as *mut libc::c_void,
            (*b).chars as *const libc::c_void,
            (*b).used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    };
}
