use ::libc;
extern "C" {
    
    
    
    static mut libzahl_tmp_modsqr: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor27 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zmodsqr(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    if a == c {
        crate::src::zset::zset(crate::src::zmodsqr::libzahl_tmp_modsqr.as_mut_ptr(), c);
        crate::src::zsqr::zsqr(a, b);
        crate::src::zmod::zmod(a, a, crate::src::zmodsqr::libzahl_tmp_modsqr.as_mut_ptr());
    } else {
        crate::src::zsqr::zsqr(a, b);
        crate::src::zmod::zmod(a, a, c);
    };
}
