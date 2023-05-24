
extern "C" {
    
    
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    
    
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn longjmp(_: *mut std::os::raw::c_int, _: std::os::raw::c_int) -> !;
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zfree::zfree;
pub use crate::src::allocator::jmp_buf;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
pub use crate::src::zdivmod::z_t;
#[inline]
unsafe extern "C" fn zinit(mut a: *mut zahl) {
    (*a).alloced = 0 as std::os::raw::c_int as size_t;
    (*a).chars = 0 as *mut zahl_char_t;
}
#[inline]
unsafe extern "C" fn libzahl_memfailure() {
    if *__error() == 0 { *__error() = 2 as std::os::raw::c_int }
    libzahl_failure(*__error());
}
unsafe extern "C" fn libzahl_failure(mut error: std::os::raw::c_int) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = core::ptr::null_mut();
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as std::os::raw::c_int);
}
#[inline]
unsafe extern "C" fn zsetu(mut a: *mut zahl, mut b: uint64_t) {
    if (b == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int;
        return
    }
    if (*a).alloced < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        libzahl_realloc(a, 1 as std::os::raw::c_int as size_t);
    }
    (*a).sign = 1 as std::os::raw::c_int;
    *(*a).chars.offset(0 as std::os::raw::c_int as isize) = b;
    (*a).used = 1 as std::os::raw::c_int as size_t;
}
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub static mut libzahl_tmp_modsqr: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_modmul: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_num: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_div: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_rem: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_u: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_v: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_mag: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_sub: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_b: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_c: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_d: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_a: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_mod: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_div: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_b: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n4: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n1: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_d: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_a: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_x: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_d: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_const_1e19: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_const_1: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_const_4: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_const_2: z_t =
    [zahl{sign: 0,
          padding__: 0,
          used: 0,
          alloced: 0,
          chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_ds: [z_t; 64] =
    [[zahl{sign: 0,
           padding__: 0,
           used: 0,
           alloced: 0,
           chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1]; 64];
#[no_mangle]
pub static mut libzahl_jmp_buf: jmp_buf = [0; 37];
#[no_mangle]
pub static mut libzahl_set_up: std::os::raw::c_int = 0 as std::os::raw::c_int;
#[no_mangle]
pub static mut libzahl_error: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut libzahl_pool: [*mut *mut zahl_char_t; 64] =
    [0 as *const *mut zahl_char_t as *mut *mut zahl_char_t; 64];
#[no_mangle]
pub static mut libzahl_pool_n: [size_t; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_pool_alloc: [size_t; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_temp_stack: *mut *mut zahl =
    0 as *const *mut zahl as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_stack_head: *mut *mut zahl =
    0 as *const *mut zahl as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_stack_end: *mut *mut zahl =
    0 as *const *mut zahl as *mut *mut zahl;
#[no_mangle]
pub static mut libzahl_temp_allocation: *mut std::os::raw::c_void =
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void;
static mut constant_chars: [zahl_char_t; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn zsetup<'a1>(mut env: Option<&'a1 mut std::os::raw::c_int>) {
    let mut i: size_t = 0;
    *libzahl_jmp_buf.as_mut_ptr() = *borrow_mut(&mut env).unwrap();
    if (libzahl_set_up == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        libzahl_set_up = 1 as std::os::raw::c_int;
        memset(libzahl_pool.as_mut_ptr() as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as
                   std::os::raw::c_ulong);
        memset(libzahl_pool_n.as_mut_ptr() as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<[size_t; 64]>() as std::os::raw::c_ulong);
        memset(libzahl_pool_alloc.as_mut_ptr() as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               ::std::mem::size_of::<[size_t; 64]>() as std::os::raw::c_ulong);
        zinit(libzahl_tmp_div.as_mut_ptr());
        zinit(libzahl_tmp_mod.as_mut_ptr());
        zinit(libzahl_tmp_str_num.as_mut_ptr());
        zinit(libzahl_tmp_str_mag.as_mut_ptr());
        zinit(libzahl_tmp_str_div.as_mut_ptr());
        zinit(libzahl_tmp_str_rem.as_mut_ptr());
        zinit(libzahl_tmp_gcd_u.as_mut_ptr());
        zinit(libzahl_tmp_gcd_v.as_mut_ptr());
        zinit(libzahl_tmp_sub.as_mut_ptr());
        zinit(libzahl_tmp_modmul.as_mut_ptr());
        zinit(libzahl_tmp_pow_b.as_mut_ptr());
        zinit(libzahl_tmp_pow_c.as_mut_ptr());
        zinit(libzahl_tmp_pow_d.as_mut_ptr());
        zinit(libzahl_tmp_modsqr.as_mut_ptr());
        zinit(libzahl_tmp_divmod_a.as_mut_ptr());
        zinit(libzahl_tmp_divmod_b.as_mut_ptr());
        zinit(libzahl_tmp_divmod_d.as_mut_ptr());
        zinit(libzahl_tmp_ptest_x.as_mut_ptr());
        zinit(libzahl_tmp_ptest_a.as_mut_ptr());
        zinit(libzahl_tmp_ptest_d.as_mut_ptr());
        zinit(libzahl_tmp_ptest_n1.as_mut_ptr());
        zinit(libzahl_tmp_ptest_n4.as_mut_ptr());
        (*libzahl_const_1e19.as_mut_ptr()).alloced =
            1 as std::os::raw::c_int as size_t;
        let ref mut fresh0 = (*libzahl_const_1e19.as_mut_ptr()).chars;
        *fresh0 =
            constant_chars.as_mut_ptr().offset(0 as std::os::raw::c_int as isize);
        zsetu(libzahl_const_1e19.as_mut_ptr(),
              10000000000000000000 as std::os::raw::c_ulonglong);
        (*libzahl_const_1.as_mut_ptr()).alloced = 1 as std::os::raw::c_int as size_t;
        let ref mut fresh1 = (*libzahl_const_1.as_mut_ptr()).chars;
        *fresh1 =
            constant_chars.as_mut_ptr().offset(1 as std::os::raw::c_int as isize);
        zsetu(libzahl_const_1.as_mut_ptr(), 1 as std::os::raw::c_int as uint64_t);
        (*libzahl_const_2.as_mut_ptr()).alloced = 1 as std::os::raw::c_int as size_t;
        let ref mut fresh2 = (*libzahl_const_2.as_mut_ptr()).chars;
        *fresh2 =
            constant_chars.as_mut_ptr().offset(2 as std::os::raw::c_int as isize);
        zsetu(libzahl_const_2.as_mut_ptr(), 2 as std::os::raw::c_int as uint64_t);
        (*libzahl_const_4.as_mut_ptr()).alloced = 1 as std::os::raw::c_int as size_t;
        let ref mut fresh3 = (*libzahl_const_4.as_mut_ptr()).chars;
        *fresh3 =
            constant_chars.as_mut_ptr().offset(3 as std::os::raw::c_int as isize);
        zsetu(libzahl_const_4.as_mut_ptr(), 4 as std::os::raw::c_int as uint64_t);
        i = 64 as std::os::raw::c_int as size_t;
        loop  {
            let fresh4 = i;
            i = i.wrapping_sub(1);
            if !(fresh4 != 0) { break ; }
            zinit(libzahl_tmp_divmod_ds[i as usize].as_mut_ptr());
        }
        libzahl_temp_stack =
            malloc((256 as std::os::raw::c_int as
                        std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<*mut zahl>()
                                                        as std::os::raw::c_ulong)) as
                *mut *mut zahl;
        if libzahl_temp_stack.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
            libzahl_memfailure();
        }
        libzahl_temp_stack_head = libzahl_temp_stack;
        libzahl_temp_stack_end =
            libzahl_temp_stack.offset(256 as std::os::raw::c_int as isize)
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

