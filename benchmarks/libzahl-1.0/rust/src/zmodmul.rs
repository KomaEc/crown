use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmod(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_modmul: z_t;
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
pub type z_t = [C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zmodmul(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
    mut d: *mut C2RustUnnamed,
) {
    if a == d {
        zset(libzahl_tmp_modmul.as_mut_ptr(), d);
        zmul(a, b, c);
        zmod(a, a, libzahl_tmp_modmul.as_mut_ptr());
    } else {
        zmul(a, b, c);
        zmod(a, a, d);
    };
}
