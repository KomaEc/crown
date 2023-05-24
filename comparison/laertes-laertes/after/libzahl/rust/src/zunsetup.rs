
extern "C" {
    #[no_mangle]
    static mut libzahl_tmp_div: [zahl; 1];
    #[no_mangle]
    static mut libzahl_tmp_mod: [zahl; 1];
    #[no_mangle]
    static mut libzahl_temp_stack: *mut *mut zahl;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    #[no_mangle]
    static mut libzahl_pool_n: [size_t; 64];
    #[no_mangle]
    static mut libzahl_tmp_divmod_ds: [z_t; 64];
    #[no_mangle]
    static mut libzahl_tmp_ptest_n4: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_n1: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_d: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_a: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_x: z_t;
    #[no_mangle]
    static mut libzahl_tmp_divmod_d: z_t;
    #[no_mangle]
    static mut libzahl_tmp_divmod_b: z_t;
    #[no_mangle]
    static mut libzahl_tmp_divmod_a: z_t;
    #[no_mangle]
    static mut libzahl_tmp_modsqr: z_t;
    #[no_mangle]
    static mut libzahl_tmp_pow_d: z_t;
    #[no_mangle]
    static mut libzahl_tmp_pow_c: z_t;
    #[no_mangle]
    static mut libzahl_tmp_pow_b: z_t;
    #[no_mangle]
    static mut libzahl_tmp_modmul: z_t;
    #[no_mangle]
    static mut libzahl_tmp_sub: z_t;
    #[no_mangle]
    static mut libzahl_tmp_gcd_v: z_t;
    #[no_mangle]
    static mut libzahl_tmp_gcd_u: z_t;
    #[no_mangle]
    static mut libzahl_tmp_str_rem: z_t;
    #[no_mangle]
    static mut libzahl_tmp_str_div: z_t;
    #[no_mangle]
    static mut libzahl_tmp_str_mag: z_t;
    #[no_mangle]
    static mut libzahl_tmp_str_num: z_t;
    #[no_mangle]
    static mut libzahl_set_up: std::os::raw::c_int;
}
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
pub use crate::src::zdivmod::z_t;
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zunsetup() {
    let mut i: size_t = 0;
    if libzahl_set_up != 0 {
        libzahl_set_up = 0 as std::os::raw::c_int;
        free((*libzahl_tmp_div.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_mod.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_str_num.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_str_mag.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_str_div.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_str_rem.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_gcd_u.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_gcd_v.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_sub.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_modmul.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_pow_b.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_pow_c.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_pow_d.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_modsqr.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_divmod_a.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_divmod_b.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_divmod_d.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_ptest_x.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_ptest_a.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_ptest_d.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_ptest_n1.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        free((*libzahl_tmp_ptest_n4.as_mut_ptr()).chars as *mut std::os::raw::c_void);
        i = 64 as std::os::raw::c_int as size_t;
        loop  {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 != 0) { break ; }
            free((*libzahl_tmp_divmod_ds[i as usize].as_mut_ptr()).chars as
                     *mut std::os::raw::c_void);
        }
        i =
            (::std::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as
                 std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*mut *mut zahl_char_t>()
                                                 as std::os::raw::c_ulong);
        loop  {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 != 0) { break ; }
            loop  {
                let fresh2 = libzahl_pool_n[i as usize];
                libzahl_pool_n[i as usize] =
                    libzahl_pool_n[i as usize].wrapping_sub(1);
                if !(fresh2 != 0) { break ; }
                free(*libzahl_pool[i as
                                       usize].offset(libzahl_pool_n[i as
                                                                        usize]
                                                         as isize) as
                         *mut std::os::raw::c_void);
            }
            free(libzahl_pool[i as usize] as *mut std::os::raw::c_void);
        }
        free(libzahl_temp_stack as *mut std::os::raw::c_void);
    };
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

