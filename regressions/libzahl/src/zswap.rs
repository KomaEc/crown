use ::libc;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor58 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zswap(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>, mut b: Option<&mut crate::src::allocator::C2RustUnnamed>) {
    let mut t: z_t = [crate::src::allocator::C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    *t.as_mut_ptr() = (*a.as_deref().unwrap());
    *a.as_deref_mut().unwrap()= (*b.as_deref().unwrap());
    *b.as_deref_mut().unwrap()= *t.as_mut_ptr();
}
