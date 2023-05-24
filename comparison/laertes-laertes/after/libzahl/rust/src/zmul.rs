
extern "C" {
    
    
    #[no_mangle]
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    static mut libzahl_temp_stack: *mut *mut zahl;
    #[no_mangle]
    static mut libzahl_temp_stack_end: *mut *mut zahl;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut libzahl_jmp_buf: jmp_buf;
    #[no_mangle]
    static mut libzahl_temp_allocation: *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    static mut libzahl_error: std::os::raw::c_int;
    #[no_mangle]
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn longjmp(_: *mut std::os::raw::c_int, _: std::os::raw::c_int) -> !;
    
    
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zadd::zadd_unsigned_assign;
pub use crate::src::zfree::zfree;
pub use crate::src::zlsh::zlsh;
pub use crate::src::zrsh::zrsh;
pub use crate::src::zsub::zsub_nonnegative_assign;
pub use crate::src::ztrunc::ztrunc;
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
unsafe extern "C" fn zzero(mut a: *mut zahl) -> std::os::raw::c_int {
    return ((*a).sign == 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn zfree_temp(mut a: *mut zahl) {
    zfree(a);
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
}
#[inline]
unsafe extern "C" fn zsplit_pz(mut high: *mut zahl, mut low: *mut zahl,
                               mut a: *mut zahl, mut delim: size_t) {
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*high).sign = 0 as std::os::raw::c_int;
        (*low).sign = 0 as std::os::raw::c_int
    } else { zsplit(high, low, a, delim); };
}
#[inline]
unsafe extern "C" fn zsplit(mut high: *mut zahl, mut low: *mut zahl,
                            mut a: *mut zahl, mut delim: size_t) {
    if (high == a) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        ztrunc(low, a, delim);
        zrsh(high, a, delim);
    } else { zrsh(high, a, delim); ztrunc(low, a, delim); };
}
#[inline]
unsafe extern "C" fn zinit_temp(mut a: *mut zahl) {
    zinit(a);
    if (libzahl_temp_stack_head == libzahl_temp_stack_end) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        let mut n: size_t =
            libzahl_temp_stack_end.offset_from(libzahl_temp_stack) as
                std::os::raw::c_long as size_t;
        let mut old: *mut std::os::raw::c_void =
            libzahl_temp_stack as *mut std::os::raw::c_void;
        libzahl_temp_stack =
            realloc(old,
                    (2 as std::os::raw::c_int as
                         std::os::raw::c_ulong).wrapping_mul(n).wrapping_mul(::std::mem::size_of::<*mut zahl>()
                                                                         as
                                                                         std::os::raw::c_ulong))
                as *mut *mut zahl;
        if libzahl_temp_stack.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
            libzahl_temp_stack = old as *mut *mut zahl;
            libzahl_memfailure();
        }
        libzahl_temp_stack_head = libzahl_temp_stack.offset(n as isize);
        libzahl_temp_stack_end = libzahl_temp_stack_head.offset(n as isize)
    }
    let fresh0 = libzahl_temp_stack_head;
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(1);
    *fresh0 = a;
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
unsafe extern "C" fn zbits(mut a: *mut zahl) -> size_t {
    let mut rc: size_t = 0;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 1 as std::os::raw::c_int as size_t
    }
    while *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                                 isize) == 0 {
        (*a).used = (*a).used.wrapping_sub(1)
    }
    rc =
        (*a).used.wrapping_mul(8 as std::os::raw::c_int as
                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                   as
                                                                   std::os::raw::c_ulong);
    rc =
        (rc as
             std::os::raw::c_ulong).wrapping_sub((*(*a).chars.offset((*a).used.wrapping_sub(1
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        std::os::raw::c_ulong)
                                                                 as
                                                                 isize)).leading_zeros()
                                             as i32 as size_t) as size_t as
            size_t;
    return rc;
}
#[inline]
unsafe extern "C" fn zzero1(mut a: *mut zahl, mut b: *mut zahl)
 -> std::os::raw::c_int {
    return (zzero(a) != 0 || zzero(b) != 0) as std::os::raw::c_int;
}
/* See LICENSE file for copyright and license details. */
#[inline]
unsafe extern "C" fn zmul_ll_single_char(mut a: *mut zahl, mut b: *mut zahl,
                                         mut c: *mut zahl) {
    if (*a).alloced < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        libzahl_realloc(a, 1 as std::os::raw::c_int as size_t);
    }
    (*a).used = 1 as std::os::raw::c_int as size_t;
    *(*a).chars.offset(0 as std::os::raw::c_int as isize) =
        (*(*b).chars.offset(0 as std::os::raw::c_int as
                                isize)).wrapping_mul(*(*c).chars.offset(0 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            isize));
    (*a).sign = 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmul_ll(mut a: *mut zahl, mut b: *mut zahl,
                                 mut c: *mut zahl) {
    /*
	 * Karatsuba algorithm
	 * 
	 * Basically, this is how you were taught to multiply large numbers
	 * by hand in school: 4010*3020 = (4000 + 10)(3000 + 20) =
	 * = 40*30*10^4 + (40*20 + 30*10)*10^2 + 10*20, but the middle is
	 * optimised to only one multiplication:
	 * 40*20 + 30*10 = (40 + 10)(30 + 20) - 40*30 - 10*20.
	 * This optimisation is crucial. Without it, the algorithm with
	 * run in O(n^2).
	 */
    let mut m: size_t = 0;
    let mut m2: size_t = 0;
    let mut b_high: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    let mut b_low: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    let mut c_high: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    let mut c_low: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    if (zzero1(b, c) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int;
        return
    }
    m = zbits(b);
    m2 = if b == c { m } else { zbits(c) };
    if m.wrapping_add(m2) <= 64 as std::os::raw::c_int as std::os::raw::c_ulong {
        zmul_ll_single_char(a, b, c);
        return
    }
    m = if m > m2 { m } else { m2 };
    m2 = m >> 1 as std::os::raw::c_int;
    zinit_temp(b_high.as_mut_ptr());
    zinit_temp(b_low.as_mut_ptr());
    zinit_temp(c_high.as_mut_ptr());
    zinit_temp(c_low.as_mut_ptr());
    zsplit_pz(b_high.as_mut_ptr(), b_low.as_mut_ptr(), b, m2);
    zsplit_pz(c_high.as_mut_ptr(), c_low.as_mut_ptr(), c, m2);
    zmul_ll(a, b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zadd_unsigned_assign(b_low.as_mut_ptr(), b_high.as_mut_ptr());
    zadd_unsigned_assign(c_low.as_mut_ptr(), c_high.as_mut_ptr());
    zmul_ll(b_low.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zmul_ll(c_low.as_mut_ptr(), b_high.as_mut_ptr(), c_high.as_mut_ptr());
    zsub_nonnegative_assign(b_low.as_mut_ptr(), a);
    zsub_nonnegative_assign(b_low.as_mut_ptr(), c_low.as_mut_ptr());
    zlsh(b_low.as_mut_ptr(), b_low.as_mut_ptr(), m2);
    m2 <<= 1 as std::os::raw::c_int;
    zlsh(c_low.as_mut_ptr(), c_low.as_mut_ptr(), m2);
    zadd_unsigned_assign(a, b_low.as_mut_ptr());
    zadd_unsigned_assign(a, c_low.as_mut_ptr());
    zfree_temp(c_low.as_mut_ptr());
    zfree_temp(c_high.as_mut_ptr());
    zfree_temp(b_low.as_mut_ptr());
    zfree_temp(b_high.as_mut_ptr());
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

