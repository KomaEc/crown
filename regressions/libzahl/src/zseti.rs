use ::libc;
extern "C" {
    
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor45 { dummy: () }
#[no_mangle]
pub unsafe extern "C" fn zseti(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>, mut b: libc::c_longlong) {
    if b >= 0 as libc::c_int as libc::c_longlong {
        crate::src::zsetu::zsetu(a.as_deref_mut(), b as libc::c_ulonglong);
    } else {
        crate::src::zsetu::zsetu(a.as_deref_mut(), -b as libc::c_ulonglong);
        (*a.as_deref_mut().unwrap()).sign= -(1 as libc::c_int);
    };
}
