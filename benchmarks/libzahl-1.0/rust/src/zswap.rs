use ::libc;
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
pub type z_t = [C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zswap(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
    let mut t: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    *t.as_mut_ptr() = *a;
    *a = *b;
    *b = *t.as_mut_ptr();
}
