
extern "C" {
    #[no_mangle]
    fn rand() -> std::os::raw::c_int;
    #[no_mangle]
    fn srand(_: std::os::raw::c_uint);
    #[no_mangle]
    fn close(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn read(_: std::os::raw::c_int, _: *mut std::os::raw::c_void, _: size_t) -> ssize_t;
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    static mut libzahl_jmp_buf: jmp_buf;
    #[no_mangle]
    static mut libzahl_temp_allocation: *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    #[no_mangle]
    static mut libzahl_temp_stack: *mut *mut zahl;
    #[no_mangle]
    static mut libzahl_error: std::os::raw::c_int;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut libzahl_const_1: z_t;
    #[no_mangle]
    fn lrand48() -> std::os::raw::c_long;
    #[no_mangle]
    fn srand48(_: std::os::raw::c_long);
    #[no_mangle]
    fn random() -> std::os::raw::c_long;
    #[no_mangle]
    fn srandom(_: std::os::raw::c_uint);
    #[no_mangle]
    fn longjmp(_: *mut std::os::raw::c_int, _: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn open(_: *const std::os::raw::c_char, _: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn time(_: *mut time_t) -> time_t;
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zadd::zadd;
pub use crate::src::zfree::zfree;
pub use crate::src::zmul::zmul_ll;
pub use crate::src::zrsh::zrsh;
pub use crate::src::zsub::zsub;
pub use crate::src::allocator::jmp_buf;
pub type __darwin_intptr_t = std::os::raw::c_long;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::zlsh::__darwin_ssize_t;
pub type __darwin_time_t = std::os::raw::c_long;
pub type intptr_t = std::os::raw::c_long;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::zlsh::ssize_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
pub use crate::src::zdivmod::z_t;
pub use crate::src::zptest::zranddev;
pub const LIBC_RAND48_RANDOM: zranddev = 6;
pub const LIBC_RANDOM_RANDOM: zranddev = 5;
pub const LIBC_RAND_RANDOM: zranddev = 4;
pub const FASTEST_RANDOM: zranddev = 3;
pub const DEFAULT_RANDOM: zranddev = 2;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub use crate::src::zptest::zranddist;
pub const MODUNIFORM: zranddist = 2;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
pub use crate::src::zdivmod::zerror;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
pub type time_t = std::os::raw::c_long;
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> std::os::raw::c_int {
    return ((*a).sign == 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl)
 -> std::os::raw::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return -((zzero(b) == 0) as std::os::raw::c_int)
    }
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 1 as std::os::raw::c_int
    }
    i = (*a).used.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    j = (*b).used.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    while i > j {
        if *(*a).chars.offset(i as isize) != 0 { return 1 as std::os::raw::c_int }
        (*a).used = (*a).used.wrapping_sub(1);
        i = i.wrapping_sub(1)
    }
    while j > i {
        if *(*b).chars.offset(j as isize) != 0 { return -(1 as std::os::raw::c_int) }
        (*b).used = (*b).used.wrapping_sub(1);
        j = j.wrapping_sub(1)
    }
    while i != 0 &&
              *(*a).chars.offset(i as isize) == *(*b).chars.offset(i as isize)
          {
        i = i.wrapping_sub(1)
    }
    return if *(*a).chars.offset(i as isize) < *(*b).chars.offset(i as isize)
              {
               -(1 as std::os::raw::c_int)
           } else {
               (*(*a).chars.offset(i as isize) >
                    *(*b).chars.offset(i as isize)) as std::os::raw::c_int
           };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> std::os::raw::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl,
                          mut c: *mut zahl) {
    let mut b_sign: std::os::raw::c_int = 0;
    let mut c_sign: std::os::raw::c_int = 0;
    b_sign = (*b).sign;
    (*b).sign *= b_sign;
    c_sign = (*c).sign;
    (*c).sign *= c_sign;
    zmul_ll(a, b, c);
    (*c).sign = c_sign;
    (*b).sign = b_sign;
    (*a).sign = zsignum(b) * zsignum(c);
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
unsafe extern "C" fn zrand_libc_rand(mut out: *mut std::os::raw::c_void,
                                     mut n: size_t,
                                     mut statep: *mut std::os::raw::c_void) {
    static mut inited: std::os::raw::c_char = 0 as std::os::raw::c_int as std::os::raw::c_char;
    let mut ri: std::os::raw::c_uint = 0;
    let mut rd: std::os::raw::c_double = 0.;
    let mut buf: *mut std::os::raw::c_uchar = out as *mut std::os::raw::c_uchar;
    if inited == 0 {
        inited = 1 as std::os::raw::c_int as std::os::raw::c_char;
        srand((out as intptr_t | time(0 as *mut time_t)) as std::os::raw::c_uint);
    }
    loop  {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        ri = rand() as std::os::raw::c_uint;
        rd =
            ri as std::os::raw::c_double /
                (0x7fffffff as std::os::raw::c_int as std::os::raw::c_double +
                     1 as std::os::raw::c_int as std::os::raw::c_double);
        rd *= (256 as std::os::raw::c_int * 256 as std::os::raw::c_int) as std::os::raw::c_double;
        ri = rd as std::os::raw::c_uint;
        *buf.offset(n as isize) =
            (ri >> 0 as std::os::raw::c_int & 255 as std::os::raw::c_int as std::os::raw::c_uint) as
                std::os::raw::c_uchar;
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if fresh1 == 0 { break ; }
        *buf.offset(n as isize) =
            (ri >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int as std::os::raw::c_uint) as
                std::os::raw::c_uchar
    };
}
unsafe extern "C" fn zrand_libc_rand48(mut out: *mut std::os::raw::c_void,
                                       mut n: size_t,
                                       mut statep: *mut std::os::raw::c_void) {
    static mut inited: std::os::raw::c_char = 0 as std::os::raw::c_int as std::os::raw::c_char;
    let mut r0: std::os::raw::c_long = 0;
    let mut r1: std::os::raw::c_long = 0;
    let mut buf: *mut std::os::raw::c_uchar = out as *mut std::os::raw::c_uchar;
    if inited == 0 {
        inited = 1 as std::os::raw::c_int as std::os::raw::c_char;
        srand48(out as intptr_t | time(0 as *mut time_t));
    }
    loop  {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 != 0) { break ; }
        r0 = lrand48() & 15 as std::os::raw::c_int as std::os::raw::c_long;
        r1 = lrand48() & 15 as std::os::raw::c_int as std::os::raw::c_long;
        *buf.offset(n as isize) =
            (r0 << 4 as std::os::raw::c_int | r1) as std::os::raw::c_uchar
    };
}
unsafe extern "C" fn zrand_libc_random(mut out: *mut std::os::raw::c_void,
                                       mut n: size_t,
                                       mut statep: *mut std::os::raw::c_void) {
    static mut inited: std::os::raw::c_char = 0 as std::os::raw::c_int as std::os::raw::c_char;
    let mut ri: std::os::raw::c_long = 0;
    let mut buf: *mut std::os::raw::c_uchar = out as *mut std::os::raw::c_uchar;
    if inited == 0 {
        inited = 1 as std::os::raw::c_int as std::os::raw::c_char;
        srandom((out as intptr_t | time(0 as *mut time_t)) as std::os::raw::c_uint);
    }
    loop  {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 != 0) { break ; }
        ri = random();
        *buf.offset(n as isize) =
            (ri >> 0 as std::os::raw::c_int & 255 as std::os::raw::c_int as std::os::raw::c_long) as
                std::os::raw::c_uchar;
        let fresh4 = n;
        n = n.wrapping_sub(1);
        if fresh4 == 0 { break ; }
        *buf.offset(n as isize) =
            (ri >> 8 as std::os::raw::c_int & 255 as std::os::raw::c_int as std::os::raw::c_long) as
                std::os::raw::c_uchar;
        let fresh5 = n;
        n = n.wrapping_sub(1);
        if fresh5 == 0 { break ; }
        *buf.offset(n as isize) =
            (ri >> 16 as std::os::raw::c_int & 255 as std::os::raw::c_int as std::os::raw::c_long) as
                std::os::raw::c_uchar
    };
}
unsafe extern "C" fn zrand_fd(mut out: *mut std::os::raw::c_void, mut n: size_t,
                              mut statep: *mut std::os::raw::c_void) {
    let mut fd: std::os::raw::c_int = *(statep as *mut std::os::raw::c_int);
    let mut read_just: ssize_t = 0;
    let mut read_total: size_t = 0 as std::os::raw::c_int as size_t;
    let mut buf: *mut std::os::raw::c_char = out as *mut std::os::raw::c_char;
    while n != 0 {
        read_just =
            read(fd, buf.offset(read_total as isize) as *mut std::os::raw::c_void, n);
        if (read_just < 0 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_int as
               std::os::raw::c_long != 0 {
            libzahl_failure(*__error());
        }
        read_total =
            (read_total as std::os::raw::c_ulong).wrapping_add(read_just as size_t) as
                size_t as size_t;
        n =
            (n as std::os::raw::c_ulong).wrapping_sub(read_just as size_t) as size_t
                as size_t
    };
}
unsafe extern "C" fn zrand_get_random_bits(mut r: *mut zahl, mut bits: size_t,
                                           mut fun:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut std::os::raw::c_void,
                                                                           _:
                                                                               size_t,
                                                                           _:
                                                                               *mut std::os::raw::c_void)
                                                          -> ()>,
                                           mut statep: *mut std::os::raw::c_void) {
    let mut n: size_t = 0;
    let mut chars: size_t =
        bits.wrapping_add((64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as
                              std::os::raw::c_ulong) >> 6 as std::os::raw::c_int;
    let mut mask: zahl_char_t = 1 as std::os::raw::c_int as zahl_char_t;
    if (*r).alloced < chars { libzahl_realloc(r, chars); }
    fun.expect("non-null function pointer")((*r).chars as *mut std::os::raw::c_void,
                                            chars.wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                   as
                                                                   std::os::raw::c_ulong),
                                            statep);
    bits = bits & (64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong;
    mask <<= bits;
    mask =
        (mask as
             std::os::raw::c_ulonglong).wrapping_sub(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    let ref mut fresh6 =
        *(*r).chars.offset(chars.wrapping_sub(1 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong) as isize);
    *fresh6 &= mask;
    n = chars;
    loop  {
        let fresh7 = n;
        n = n.wrapping_sub(1);
        if !(fresh7 != 0) { break ; }
        if (*(*r).chars.offset(n as isize) != 0) as std::os::raw::c_int as
               std::os::raw::c_long != 0 {
            (*r).used = n.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
            (*r).sign = 1 as std::os::raw::c_int;
            return
        }
    }
    (*r).sign = 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zrand(mut r: *mut zahl, mut dev: zranddev,
                               mut dist: zranddist, mut n: *mut zahl) {
    let mut pathname: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut bits: size_t = 0;
    let mut fd: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut statep: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut random_fun:
            Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t,
                                        _: *mut std::os::raw::c_void) -> ()> =
        Some(zrand_fd as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t,
                                      _: *mut std::os::raw::c_void) -> ());
    match dev as std::os::raw::c_uint {
        0 => {
            pathname = b"/dev/urandom\x00" as *const u8 as *const std::os::raw::c_char
        }
        1 => {
            pathname = b"/dev/random\x00" as *const u8 as *const std::os::raw::c_char
        }
        4 => {
            random_fun =
                Some(zrand_libc_rand as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t,
                                              _: *mut std::os::raw::c_void) -> ())
        }
        2 | 3 | 5 => {
            random_fun =
                Some(zrand_libc_random as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t,
                                              _: *mut std::os::raw::c_void) -> ())
        }
        6 => {
            random_fun =
                Some(zrand_libc_rand48 as
                         unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: size_t,
                                              _: *mut std::os::raw::c_void) -> ())
        }
        _ => { libzahl_failure(22 as std::os::raw::c_int); }
    }
    if (zzero(n) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*r).sign = 0 as std::os::raw::c_int;
        return
    }
    if !pathname.is_null() {
        fd = open(pathname, 0 as std::os::raw::c_int);
        if (fd < 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0 {
            libzahl_failure(*__error());
        }
        statep = &mut fd as *mut std::os::raw::c_int as *mut std::os::raw::c_void
    }
    match dist as std::os::raw::c_uint {
        0 => {
            if (zsignum(n) < 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long
                   != 0 {
                libzahl_failure(-(ZERROR_NEGATIVE as std::os::raw::c_int));
            }
            bits = zbits(n);
            loop  {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as std::os::raw::c_int != 0 &&
                         (zcmpmag(r, n) > 0 as std::os::raw::c_int) as std::os::raw::c_int as
                             std::os::raw::c_long != 0) {
                    break ;
                }
            }
            zadd(r, r, libzahl_const_1.as_mut_ptr());
            zmul(r, r, n);
            zrsh(r, r, bits);
        }
        1 => {
            if (zsignum(n) < 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long
                   != 0 {
                libzahl_failure(-(ZERROR_NEGATIVE as std::os::raw::c_int));
            }
            bits = zbits(n);
            loop  {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(1 as std::os::raw::c_int != 0 &&
                         (zcmpmag(r, n) > 0 as std::os::raw::c_int) as std::os::raw::c_int as
                             std::os::raw::c_long != 0) {
                    break ;
                }
            }
        }
        2 => {
            if (zsignum(n) < 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long
                   != 0 {
                libzahl_failure(-(ZERROR_NEGATIVE as std::os::raw::c_int));
            }
            bits = zbits(n);
            loop  {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as std::os::raw::c_int != 0 &&
                         (zcmpmag(r, n) > 0 as std::os::raw::c_int) as std::os::raw::c_int as
                             std::os::raw::c_long != 0) {
                    break ;
                }
            }
            if (zcmpmag(r, n) > 0 as std::os::raw::c_int) as std::os::raw::c_int as
                   std::os::raw::c_long != 0 {
                zsub(r, r, n);
            }
        }
        _ => { libzahl_failure(22 as std::os::raw::c_int); }
    }
    if fd >= 0 as std::os::raw::c_int { close(fd); };
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

