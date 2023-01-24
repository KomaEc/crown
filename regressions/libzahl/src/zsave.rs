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

struct ErasedByPreprocessor43 { dummy: () }
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsave(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut buffer: *mut libc::c_void,
) -> size_t {
    if !buffer.is_null() {
        let mut buf = buffer as *mut libc::c_char;
        *(buf as *mut libc::c_int) = (*a).sign;
        buf= buf.offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize);
        *(buf as *mut size_t) = (*a).used;
        buf= buf.offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize);
        if zzero(a) == 0 {
            memcpy(
                buf as *mut libc::c_void,
                (*a).chars as *const libc::c_void,
                (*a).used
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
            );
        }
    }else { (); }
    return (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(
            (if zzero(a) != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                (*a).used
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
            }),
        );
}
