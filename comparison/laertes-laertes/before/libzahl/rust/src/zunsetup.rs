
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
pub type z_t = [zahl; 1];
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
