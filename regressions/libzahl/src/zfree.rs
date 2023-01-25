use ::libc;
extern "C" {
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    static mut libzahl_pool_alloc: [size_t; 64];
    static mut libzahl_pool_n: [size_t; 64];
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor14 { dummy: () }
#[no_mangle]
pub unsafe extern "C" fn zfree(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>) {
    let mut i = 0 as libc::c_int as size_t;
    let mut x: size_t = 0;
    let mut j: size_t = 0;
    let mut new = 0 as *mut *mut zahl_char_t;
    if (*a.as_deref().unwrap()).chars.is_null() {();
        return;
    }
    x= (*a.as_deref().unwrap()).alloced;
    while x != 0 {
        i= (i as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        x>>= 1 as libc::c_int;
    }
    let fresh0 = crate::src::zfree::libzahl_pool_n[i as usize];
    crate::src::zfree::libzahl_pool_n[i as usize]= crate::src::zfree::libzahl_pool_n[i as usize].wrapping_add(1);
    j= fresh0;
    if j == crate::src::zfree::libzahl_pool_alloc[i as usize] {
        x= if j != 0 {
            j.wrapping_mul(3 as libc::c_int as libc::c_ulong) >> 1 as libc::c_int
        } else {
            128 as libc::c_int as libc::c_ulong
        };
        new= realloc(
            crate::src::zfree::libzahl_pool[i as usize] as *mut libc::c_void,
            x.wrapping_mul(::std::mem::size_of::<*mut zahl_char_t>() as libc::c_ulong),
        ) as *mut *mut zahl_char_t;
        if new.is_null() {();
            free((*a.as_deref().unwrap()).chars as *mut libc::c_void);
            free(crate::src::zfree::libzahl_pool[i as usize] as *mut libc::c_void);
            crate::src::zfree::libzahl_pool_n[i as usize]= 0 as libc::c_int as size_t;
            crate::src::zfree::libzahl_pool[i as usize]= 0 as *mut *mut zahl_char_t;
            crate::src::zfree::libzahl_pool_alloc[i as usize]= 0 as libc::c_int as size_t;
            return;
        }
        crate::src::zfree::libzahl_pool[i as usize]= new;
        crate::src::zfree::libzahl_pool_alloc[i as usize]= x;
    }
    *crate::src::zfree::libzahl_pool[i as usize].offset(j as isize) = (*a.as_deref().unwrap()).chars;
}
