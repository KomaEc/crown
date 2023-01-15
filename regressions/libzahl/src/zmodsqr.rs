use ::libc;
extern "C" {

    static mut libzahl_tmp_modsqr: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct OrcGeneratedXXX27;
impl Default for OrcGeneratedXXX27 {
    fn default() -> Self {
        Self {}
    }
}

pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zmodsqr(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    if a == c {
        crate::src::zset::zset(libzahl_tmp_modsqr.as_mut_ptr().as_mut(), c);
        crate::src::zsqr::zsqr(a, b);
        crate::src::zmod::zmod(a, a, libzahl_tmp_modsqr.as_mut_ptr());
    } else {
        crate::src::zsqr::zsqr(a, b);
        crate::src::zmod::zmod(a, a, c);
    };
}
