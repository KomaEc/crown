
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
pub use crate::src::zmul::zmul_ll;
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
unsafe extern "C" fn zsplit_unsigned_fast_large_taint(mut high: *mut zahl,
                                                      mut low: *mut zahl,
                                                      mut a: *mut zahl,
                                                      mut n: size_t) {
    n >>= 6 as std::os::raw::c_int;
    (*high).sign = 1 as std::os::raw::c_int;
    (*high).used = (*a).used.wrapping_sub(n);
    (*high).chars = (*a).chars.offset(n as isize);
    (*low).sign = 1 as std::os::raw::c_int;
    (*low).used = n;
    (*low).chars = (*a).chars;
    while (*low).used != 0 &&
              *(*low).chars.offset((*low).used.wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_ulong)
                                       as isize) == 0 {
        (*low).used = (*low).used.wrapping_sub(1)
    }
    if (*low).used == 0 { (*low).sign = 0 as std::os::raw::c_int };
}
#[inline]
unsafe extern "C" fn zsplit_unsigned_fast_small_auto(mut high: *mut zahl,
                                                     mut low: *mut zahl,
                                                     mut a: *mut zahl,
                                                     mut n: size_t) {
    let mut mask: zahl_char_t = 1 as std::os::raw::c_int as zahl_char_t;
    mask = (mask << n).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulonglong);
    (*high).sign = 1 as std::os::raw::c_int;
    (*high).used = 1 as std::os::raw::c_int as size_t;
    *(*high).chars.offset(0 as std::os::raw::c_int as isize) =
        *(*a).chars.offset(0 as std::os::raw::c_int as isize) >> n;
    if (*a).used == 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        *(*high).chars.offset(1 as std::os::raw::c_int as isize) =
            *(*a).chars.offset(1 as std::os::raw::c_int as isize) >> n;
        (*high).used =
            ((*high).used as
                 std::os::raw::c_ulong).wrapping_add((*(*high).chars.offset(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        isize)
                                                  != 0) as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        n = (64 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_sub(n);
        let ref mut fresh1 = *(*high).chars.offset(0 as std::os::raw::c_int as isize);
        *fresh1 |= (*(*a).chars.offset(1 as std::os::raw::c_int as isize) & mask) << n
    }
    (*low).sign = 1 as std::os::raw::c_int;
    (*low).used = 1 as std::os::raw::c_int as size_t;
    *(*low).chars.offset(0 as std::os::raw::c_int as isize) =
        *(*a).chars.offset(0 as std::os::raw::c_int as isize) & mask;
    if (*(*low).chars.offset(0 as std::os::raw::c_int as isize) == 0) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        (*low).sign = 0 as std::os::raw::c_int
    };
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
/* See LICENSE file for copyright and license details. */
#[inline]
unsafe extern "C" fn zsqr_ll_single_char(mut a: *mut zahl, mut b: *mut zahl) {
    if (*a).alloced < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        libzahl_realloc(a, 1 as std::os::raw::c_int as size_t);
    }
    (*a).used = 1 as std::os::raw::c_int as size_t;
    *(*a).chars.offset(0 as std::os::raw::c_int as isize) =
        (*(*b).chars.offset(0 as std::os::raw::c_int as
                                isize)).wrapping_mul(*(*b).chars.offset(0 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            isize));
    (*a).sign = 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zsqr_ll(mut a: *mut zahl, mut b: *mut zahl) {
    /*
	 * Karatsuba algorithm, optimised for equal factors.
	 */
    let mut z0: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    let mut z1: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    let mut high: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    let mut low: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *mut zahl_char_t,}; 1];
    let mut auxchars: [zahl_char_t; 12] = [0; 12];
    let mut bits: size_t = 0;
    bits = zbits(b);
    if bits <= (64 as std::os::raw::c_int / 2 as std::os::raw::c_int) as std::os::raw::c_ulong {
        zsqr_ll_single_char(a, b);
        return
    }
    bits >>= 1 as std::os::raw::c_int;
    /* Try to split only at a character level rather than a bit level.
	 * Such splits are faster, even if bit-level is required, and do
	 * not require auxiliary memory except for the bit-level split
	 * which require constant auxiliary memory. */
    if bits < 64 as std::os::raw::c_int as std::os::raw::c_ulong {
        let ref mut fresh2 = (*low.as_mut_ptr()).chars;
        *fresh2 = auxchars.as_mut_ptr();
        let ref mut fresh3 = (*high.as_mut_ptr()).chars;
        *fresh3 = auxchars.as_mut_ptr().offset(4 as std::os::raw::c_int as isize);
        zsplit_unsigned_fast_small_auto(high.as_mut_ptr(), low.as_mut_ptr(),
                                        b, bits);
    } else {
        bits = bits & !((64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as size_t);
        zsplit_unsigned_fast_large_taint(high.as_mut_ptr(), low.as_mut_ptr(),
                                         b, bits);
    }
    if (zzero(low.as_mut_ptr()) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        zsqr_ll(a, high.as_mut_ptr());
        zlsh(a, a, bits << 1 as std::os::raw::c_int);
    } else {
        zinit_temp(z0.as_mut_ptr());
        zinit_temp(z1.as_mut_ptr());
        zsqr_ll(z0.as_mut_ptr(), low.as_mut_ptr());
        zmul_ll(z1.as_mut_ptr(), low.as_mut_ptr(), high.as_mut_ptr());
        zlsh(z1.as_mut_ptr(), z1.as_mut_ptr(),
             bits.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
        zsqr_ll(a, high.as_mut_ptr());
        zlsh(a, a, bits << 1 as std::os::raw::c_int);
        zadd_unsigned_assign(a, z1.as_mut_ptr());
        zadd_unsigned_assign(a, z0.as_mut_ptr());
        zfree_temp(z1.as_mut_ptr());
        zfree_temp(z0.as_mut_ptr());
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

