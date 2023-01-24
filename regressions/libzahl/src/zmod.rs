use ::libc;
extern "C" {
    
    static mut libzahl_tmp_mod: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor19 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zmod(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    crate::src::zdivmod::zdivmod(crate::src::zmod::libzahl_tmp_mod.as_mut_ptr(), a, b, c);
}
