use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor3 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zbits(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>) -> size_t {
    let mut i: size_t = 0;
    let mut x: zahl_char_t = 0;
    if zzero(a.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 {
        return 1 as libc::c_int as size_t;
    }
    i= (*a.as_deref().unwrap()).used.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        x= *(*a.as_deref().unwrap()).chars.offset(i as isize);
        if x != 0 {
            (*a.as_deref_mut().unwrap()).used= i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            i= (i as libc::c_ulong).wrapping_mul(32 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            while x != 0 {
                x>>= 1 as libc::c_int;
                i= i.wrapping_add(1);
            }
            return i;
        }
        i= i.wrapping_sub(1);
    };
}
