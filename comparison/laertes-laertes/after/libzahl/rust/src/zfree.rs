
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
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
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
pub fn borrow<'a, 'b: 'a, T>(p: &'a Option<&'b mut T>) -> Option<&'a T> {
    p.as_ref().map(|x| &**x)
}

pub fn borrow_mut<'a, 'b : 'a, T>(p: &'a mut Option<&'b mut T>) -> Option<&'a mut T> {
    p.as_mut().map(|x| &mut **x)
}

pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
    p.as_ref().map(|x| x.as_ref())
}

pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
    p.as_mut().map(|x| x.as_mut())
}

pub fn option_to_raw<T>(p: Option<&T>) -> * const T {
    p.map_or(core::ptr::null(), |p| p as * const T)
}

pub fn _ref_eq<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) == option_to_raw(q)
}

pub fn _ref_ne<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) != option_to_raw(q)
}

