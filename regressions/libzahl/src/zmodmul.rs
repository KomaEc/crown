use ::libc;
extern "C" {
    
    
    
    static mut libzahl_tmp_modmul: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor20 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zmodmul(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
    mut d: *mut crate::src::allocator::C2RustUnnamed,
) {
    if a == d {
        crate::src::zset::zset(crate::src::zmodmul::libzahl_tmp_modmul.as_mut_ptr(), d);
        crate::src::zmul::zmul(a, b, c);
        crate::src::zmod::zmod(a, a, crate::src::zmodmul::libzahl_tmp_modmul.as_mut_ptr());
    } else {
        crate::src::zmul::zmul(a, b, c);
        crate::src::zmod::zmod(a, a, d);
    };
}
