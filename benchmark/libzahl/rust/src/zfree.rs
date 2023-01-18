
extern "C" {
    #[no_mangle]
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    #[no_mangle]
    static mut libzahl_pool_alloc: [size_t; 64];
    #[no_mangle]
    static mut libzahl_pool_n: [size_t; 64];
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
}
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;
pub type uint64_t = std::os::raw::c_ulonglong;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: std::os::raw::c_int,
    pub padding__: std::os::raw::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zfree(mut a: *mut zahl) {
    let mut i: size_t = 0;
    let mut x: size_t = 0;
    let mut j: size_t = 0;
    let mut new: *mut *mut zahl_char_t = 0 as *mut *mut zahl_char_t;
    if (*a).chars.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 { return }
    i =
        (8 as std::os::raw::c_int as
             std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_ulong>()
                                             as
                                             std::os::raw::c_ulong).wrapping_sub(1 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_ulong).wrapping_sub((*a).alloced.leading_zeros()
                                                                                                             as
                                                                                                             i32
                                                                                                             as
                                                                                                             size_t);
    let fresh0 = libzahl_pool_n[i as usize];
    libzahl_pool_n[i as usize] = libzahl_pool_n[i as usize].wrapping_add(1);
    j = fresh0;
    if j == libzahl_pool_alloc[i as usize] {
        x =
            if j != 0 {
                (j.wrapping_mul(3 as std::os::raw::c_int as std::os::raw::c_ulong)) >>
                    1 as std::os::raw::c_int
            } else { 128 as std::os::raw::c_int as std::os::raw::c_ulong };
        new =
            realloc(libzahl_pool[i as usize] as *mut std::os::raw::c_void,
                    x.wrapping_mul(::std::mem::size_of::<*mut zahl_char_t>()
                                       as std::os::raw::c_ulong)) as
                *mut *mut zahl_char_t;
        if new.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
            free((*a).chars as *mut std::os::raw::c_void);
            free(libzahl_pool[i as usize] as *mut std::os::raw::c_void);
            libzahl_pool_n[i as usize] = 0 as std::os::raw::c_int as size_t;
            libzahl_pool[i as usize] = 0 as *mut *mut zahl_char_t;
            libzahl_pool_alloc[i as usize] = 0 as std::os::raw::c_int as size_t;
            return
        }
        libzahl_pool[i as usize] = new;
        libzahl_pool_alloc[i as usize] = x
    }
    let ref mut fresh1 = *libzahl_pool[i as usize].offset(j as isize);
    *fresh1 = (*a).chars;
}
