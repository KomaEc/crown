use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zload(
    mut a: *mut C2RustUnnamed,
    mut buffer: *const libc::c_void,
) -> size_t {
    let mut buf = buffer as *const libc::c_char;
    (*a).sign = *(buf as *const libc::c_int);
    buf = buf.offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize);
    (*a).used = *(buf as *const size_t);
    buf = buf.offset(::std::mem::size_of::<size_t>() as libc::c_ulong as isize);
    if (*a).sign != 0 {
        if (*a).alloced < (*a).used {
            libzahl_realloc(a, (*a).used);
        }
        memcpy(
            (*a).chars as *mut libc::c_void,
            buf as *const libc::c_void,
            ((*a).used)
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
    }
    return (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(
            (if zzero(a) != 0 {
                0 as libc::c_int as libc::c_ulong
            } else {
                ((*a).used)
                    .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong)
            }),
        );
}
