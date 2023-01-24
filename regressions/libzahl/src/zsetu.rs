use ::libc;
extern "C" {
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor47 { dummy: () }
#[no_mangle]
pub unsafe extern "C" fn zsetu(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>, mut b: libc::c_ulonglong) {
    if b == 0 {
        (*a.as_deref_mut().unwrap()).sign= 0 as libc::c_int;
        return;
    }
    if (*a.as_deref().unwrap()).alloced
        < (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
    {
        crate::src::allocator::libzahl_realloc(
            a.as_deref_mut(),
            (::std::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    (*a.as_deref_mut().unwrap()).sign= 1 as libc::c_int;
    (*a.as_deref_mut().unwrap()).used= 0 as libc::c_int as size_t;
    while b != 0 {
        let fresh1 = (*a.as_deref().unwrap()).used;(*a.as_deref_mut().unwrap()).used= (*a.as_deref().unwrap()).used.wrapping_add(1);
        *(*a.as_deref().unwrap()).chars.offset(fresh1 as isize) = b as zahl_char_t;
        b>>= 32 as libc::c_int;
    }
}
