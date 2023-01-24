use ::libc;
extern "C" {
    static mut libzahl_tmp_div: z_t;
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor10 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zdiv(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    crate::src::zdivmod::zdivmod(a, crate::src::zdiv::libzahl_tmp_div.as_mut_ptr(), b, c);
}
