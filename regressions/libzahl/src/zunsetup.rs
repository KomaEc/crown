use ::libc;
extern "C" {
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_pool_n: [size_t; 64];
    static mut libzahl_tmp_divmod_ds: [z_t; 32];
    static mut libzahl_const_4: z_t;
    static mut libzahl_const_2: z_t;
    static mut libzahl_const_1: z_t;
    static mut libzahl_const_1e9: z_t;
    static mut libzahl_const_1e19: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_tmp_divmod_d: z_t;
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    static mut libzahl_tmp_modsqr: z_t;
    static mut libzahl_tmp_pow_d: z_t;
    static mut libzahl_tmp_pow_c: z_t;
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_mod: z_t;
    static mut libzahl_tmp_div: z_t;
    static mut libzahl_tmp_modmul: z_t;
    static mut libzahl_tmp_sub: z_t;
    static mut libzahl_tmp_gcd_v: z_t;
    static mut libzahl_tmp_gcd_u: z_t;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_div: z_t;
    static mut libzahl_tmp_str_mag: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_tmp_cmp: z_t;
    static mut libzahl_set_up: libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor60 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[no_mangle]
pub unsafe extern "C" fn zunsetup() {
    let mut i: size_t = 0;
    if crate::src::zunsetup::libzahl_set_up != 0 {
        crate::src::zunsetup::libzahl_set_up= 0 as libc::c_int;
        free((*crate::src::zunsetup::libzahl_tmp_cmp.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_str_num.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_str_mag.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_str_div.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_str_rem.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_gcd_u.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_gcd_v.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_sub.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_modmul.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_div.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_mod.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_pow_b.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_pow_c.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_pow_d.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_modsqr.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_divmod_a.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_divmod_b.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_divmod_d.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_ptest_x.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_ptest_a.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_ptest_d.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_ptest_n1.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_tmp_ptest_n4.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_const_1e19.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_const_1e9.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_const_1.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_const_2.as_mut_ptr()).chars as *mut libc::c_void);
        free((*crate::src::zunsetup::libzahl_const_4.as_mut_ptr()).chars as *mut libc::c_void);
        i= 32 as libc::c_int as size_t;
        loop {
            let fresh0 = i;
            i= i.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            free(
                (*crate::src::zunsetup::libzahl_tmp_divmod_ds[i as usize].as_mut_ptr()).chars
                    as *mut libc::c_void,
            );
        }
        i= (::std::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as libc::c_ulong)
            .wrapping_div(
                ::std::mem::size_of::<*mut *mut zahl_char_t>() as libc::c_ulong,
            );
        loop {
            let fresh1 = i;
            i= i.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            loop {
                let fresh2 = crate::src::zunsetup::libzahl_pool_n[i as usize];
                crate::src::zunsetup::libzahl_pool_n[i
                    as usize]= crate::src::zunsetup::libzahl_pool_n[i as usize].wrapping_sub(1);
                if !(fresh2 != 0) {
                    break;
                }
                free(
                    *crate::src::zunsetup::libzahl_pool[i as usize]
                        .offset(crate::src::zunsetup::libzahl_pool_n[i as usize] as isize) as *mut libc::c_void,
                );
            }
            free(crate::src::zunsetup::libzahl_pool[i as usize] as *mut libc::c_void);
        }
    }
}
