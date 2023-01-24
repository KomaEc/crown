use ::libc;
extern "C" {
    
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]

struct ErasedByPreprocessor48 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor49 { dummy: () }
pub type jmp_buf = [crate::src::allocator::__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor50 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zinit(mut a: Option<&mut crate::src::allocator::C2RustUnnamed>) {
    (*a.as_deref_mut().unwrap()).alloced= 0 as libc::c_int as size_t;
    (*a.as_deref_mut().unwrap()).chars= 0 as *mut zahl_char_t;
}
#[no_mangle]
pub static mut libzahl_tmp_modmul: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_d: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_div: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_num: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n4: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_b: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_cmp: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_mag: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_c: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_div: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_rem: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_u: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_v: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_sub: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_mod: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_d: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_a: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_x: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_d: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n1: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_modsqr: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_a: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_b: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1e9: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1e19: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_2: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_const_4: z_t = [crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_ds: [z_t; 32] = [[crate::src::allocator::C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const zahl_char_t as *mut zahl_char_t,
}; 1]; 32];
#[no_mangle]
pub static mut libzahl_jmp_buf: jmp_buf = [crate::src::allocator::__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: crate::src::allocator::__sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut libzahl_set_up: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut libzahl_error: libc::c_int = 0;
#[no_mangle]
pub static mut libzahl_pool: [*mut *mut zahl_char_t; 64] = [0 as *const *mut zahl_char_t
    as *mut *mut zahl_char_t; 64];
#[no_mangle]
pub static mut libzahl_pool_n: [size_t; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_pool_alloc: [size_t; 64] = [0; 64];
#[no_mangle]
pub unsafe extern "C" fn zsetup(mut env: *mut crate::src::allocator::__jmp_buf_tag) {
    let mut i: size_t = 0;
    *crate::src::zsetup::libzahl_jmp_buf.as_mut_ptr() = (*env);
    if crate::src::zsetup::libzahl_set_up == 0 {
        crate::src::zsetup::libzahl_set_up= 1 as libc::c_int;
        memset(
            crate::src::zsetup::libzahl_pool.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as libc::c_ulong,
        );
        memset(
            crate::src::zsetup::libzahl_pool_n.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[size_t; 64]>() as libc::c_ulong,
        );
        memset(
            crate::src::zsetup::libzahl_pool_alloc.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[size_t; 64]>() as libc::c_ulong,
        );
        zinit(crate::src::zsetup::libzahl_tmp_cmp.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_str_num.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_str_mag.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_str_div.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_str_rem.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_gcd_u.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_gcd_v.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_sub.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_modmul.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_div.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_mod.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_pow_b.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_pow_c.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_pow_d.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_modsqr.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_divmod_a.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_divmod_b.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_divmod_d.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_ptest_x.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_ptest_a.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_ptest_d.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_ptest_n1.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_tmp_ptest_n4.as_mut_ptr().as_mut());
        zinit(crate::src::zsetup::libzahl_const_1e19.as_mut_ptr().as_mut());
        crate::src::zsetu::zsetu(
            crate::src::zsetup::libzahl_const_1e19.as_mut_ptr().as_mut(),
            10000000000000000000 as libc::c_ulonglong,
        );
        zinit(crate::src::zsetup::libzahl_const_1e9.as_mut_ptr().as_mut());
        crate::src::zsetu::zsetu(crate::src::zsetup::libzahl_const_1e9.as_mut_ptr().as_mut(), 1000000000 as libc::c_ulonglong);
        zinit(crate::src::zsetup::libzahl_const_1.as_mut_ptr().as_mut());
        crate::src::zsetu::zsetu(crate::src::zsetup::libzahl_const_1.as_mut_ptr().as_mut(), 1 as libc::c_int as libc::c_ulonglong);
        zinit(crate::src::zsetup::libzahl_const_2.as_mut_ptr().as_mut());
        crate::src::zsetu::zsetu(crate::src::zsetup::libzahl_const_2.as_mut_ptr().as_mut(), 2 as libc::c_int as libc::c_ulonglong);
        zinit(crate::src::zsetup::libzahl_const_4.as_mut_ptr().as_mut());
        crate::src::zsetu::zsetu(crate::src::zsetup::libzahl_const_4.as_mut_ptr().as_mut(), 4 as libc::c_int as libc::c_ulonglong);
        i= 32 as libc::c_int as size_t;
        loop {
            let fresh1 = i;
            i= i.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            zinit(crate::src::zsetup::libzahl_tmp_divmod_ds[i as usize].as_mut_ptr().as_mut());
        }
    }
}
