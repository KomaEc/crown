use ::libc;
extern "C" {
    fn memcpy(
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

struct ErasedByPreprocessor16 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zload(
    mut a: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut buffer: *const libc::c_void,
) -> size_t {
    let mut buf = buffer as *const libc::c_char;
    (*a.as_deref_mut().unwrap()).sign= *(buf as *const libc::c_int);
    buf= buf.offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize);
    (*a.as_deref_mut().unwrap()).used= *(buf as *const size_t);
    buf= buf.offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize);
    if (*a.as_deref().unwrap()).sign != 0 {
        if (*a.as_deref().unwrap()).alloced < (*a.as_deref().unwrap()).used {
            crate::src::allocator::libzahl_realloc(a.as_deref_mut(), (*a.as_deref().unwrap()).used);
        }
        memcpy(
            (*a.as_deref().unwrap()).chars as *mut libc::c_void,
            buf as *const libc::c_void,
            (*a.as_deref().unwrap()).used
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    return (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(
            (if zzero(a.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (*a.as_deref().unwrap()).used
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
            }),
        );
}
