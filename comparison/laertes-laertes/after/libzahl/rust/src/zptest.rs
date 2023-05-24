
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    static mut libzahl_tmp_ptest_a: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_n1: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_x: z_t;
    #[no_mangle]
    static mut libzahl_const_1: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_d: z_t;
    #[no_mangle]
    static mut libzahl_const_2: z_t;
    #[no_mangle]
    static mut libzahl_tmp_ptest_n4: z_t;
    
    
    #[no_mangle]
    static mut libzahl_const_4: z_t;
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zadd::zadd_unsigned;
pub use crate::src::zmodpow::zmodpow;
pub use crate::src::zmodsqr::zmodsqr;
pub use crate::src::zrand::zrand;
pub use crate::src::zrsh::zrsh;
pub use crate::src::zsub::zsub_unsigned;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
pub use crate::src::zdivmod::z_t;
pub type zprimality = std::os::raw::c_uint;
pub const PRIME: zprimality = 2;
pub const PROBABLY_PRIME: zprimality = 1;
pub const NONPRIME: zprimality = 0;
pub type zranddev = std::os::raw::c_uint;
pub const LIBC_RAND48_RANDOM: zranddev = 6;
pub const LIBC_RANDOM_RANDOM: zranddev = 5;
pub const LIBC_RAND_RANDOM: zranddev = 4;
pub const FASTEST_RANDOM: zranddev = 3;
pub const DEFAULT_RANDOM: zranddev = 2;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub type zranddist = std::os::raw::c_uint;
pub const MODUNIFORM: zranddist = 2;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
#[inline]
unsafe extern "C" fn libzahl_memcpy(mut d_0: *mut zahl_char_t,
                                    mut s: *const zahl_char_t,
                                    mut n: size_t) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d_0.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16314148179394187963;
        }
        19 => { current_block_42 = 16314148179394187963; }
        18 => { current_block_42 = 23778815036459645; }
        17 => { current_block_42 = 17409214555335211048; }
        16 => { current_block_42 = 15059990250786981251; }
        15 => { current_block_42 = 6620638995229345271; }
        14 => { current_block_42 = 3600730107057416047; }
        13 => { current_block_42 = 354696938566919285; }
        12 => { current_block_42 = 8395813788269618564; }
        11 => { current_block_42 = 16693777636139501188; }
        10 => { current_block_42 = 905962748010064685; }
        9 => { current_block_42 = 9361680084571800209; }
        8 => { current_block_42 = 14892601158468998162; }
        7 => { current_block_42 = 5996064384694841776; }
        6 => { current_block_42 = 2207944898923095617; }
        5 => { current_block_42 = 11359770377620528283; }
        4 => { current_block_42 = 13725184785425480707; }
        3 => { current_block_42 = 1832949796153364909; }
        2 => { current_block_42 = 15100265573273486990; }
        1 => { current_block_42 = 12150485915963817543; }
        0 => { current_block_42 = 6717214610478484138; }
        _ => {
            let mut t: zahl_char_t = 0;
            llvm_asm!("\n    shlq $$3, $3\n    addq $1, $3\n 1:\n    movq 0($2), $0\n    movq $0, 0($1)\n    movq 8($2), $0\n    movq $0, 8($1)\n    movq 16($2), $0\n    movq $0, 16($1)\n    movq 24($2), $0\n    movq $0, 24($1)\n    addq $$32, $2\n    addq $$32, $1\n    cmpq $3, $1\n    jl 1b"
                 : "=r" (t), "+r" (d_0), "+r" (s), "+r" (n) : : : "volatile");
            current_block_42 = 6717214610478484138;
        }
    }
    match current_block_42 {
        16314148179394187963 => {
            *d_0.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 23778815036459645;
        }
        _ => { }
    }
    match current_block_42 {
        23778815036459645 => {
            *d_0.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 17409214555335211048;
        }
        _ => { }
    }
    match current_block_42 {
        17409214555335211048 => {
            *d_0.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15059990250786981251;
        }
        _ => { }
    }
    match current_block_42 {
        15059990250786981251 => {
            *d_0.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 6620638995229345271;
        }
        _ => { }
    }
    match current_block_42 {
        6620638995229345271 => {
            *d_0.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 3600730107057416047;
        }
        _ => { }
    }
    match current_block_42 {
        3600730107057416047 => {
            *d_0.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 354696938566919285;
        }
        _ => { }
    }
    match current_block_42 {
        354696938566919285 => {
            *d_0.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 8395813788269618564;
        }
        _ => { }
    }
    match current_block_42 {
        8395813788269618564 => {
            *d_0.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16693777636139501188;
        }
        _ => { }
    }
    match current_block_42 {
        16693777636139501188 => {
            *d_0.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 905962748010064685;
        }
        _ => { }
    }
    match current_block_42 {
        905962748010064685 => {
            *d_0.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 9361680084571800209;
        }
        _ => { }
    }
    match current_block_42 {
        9361680084571800209 => {
            *d_0.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 14892601158468998162;
        }
        _ => { }
    }
    match current_block_42 {
        14892601158468998162 => {
            *d_0.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 5996064384694841776;
        }
        _ => { }
    }
    match current_block_42 {
        5996064384694841776 => {
            *d_0.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 2207944898923095617;
        }
        _ => { }
    }
    match current_block_42 {
        2207944898923095617 => {
            *d_0.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 11359770377620528283;
        }
        _ => { }
    }
    match current_block_42 {
        11359770377620528283 => {
            *d_0.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 13725184785425480707;
        }
        _ => { }
    }
    match current_block_42 {
        13725184785425480707 => {
            *d_0.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 1832949796153364909;
        }
        _ => { }
    }
    match current_block_42 {
        1832949796153364909 => {
            *d_0.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15100265573273486990;
        }
        _ => { }
    }
    match current_block_42 {
        15100265573273486990 => {
            *d_0.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 12150485915963817543;
        }
        _ => { }
    }
    match current_block_42 {
        12150485915963817543 => {
            *d_0.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)
        }
        _ => { }
    };
}
#[inline]
unsafe extern "C" fn zswap(mut a_: *mut zahl, mut b_: *mut zahl) {
    let mut t: std::os::raw::c_long = 0;
    let mut a_0: *mut std::os::raw::c_long = a_ as *mut std::os::raw::c_long;
    let mut b: *mut std::os::raw::c_long = b_ as *mut std::os::raw::c_long;
    t = *a_0.offset(0 as std::os::raw::c_int as isize);
    *a_0.offset(0 as std::os::raw::c_int as isize) =
        *b.offset(0 as std::os::raw::c_int as isize);
    *b.offset(0 as std::os::raw::c_int as isize) = t;
    t = *b.offset(1 as std::os::raw::c_int as isize);
    *b.offset(1 as std::os::raw::c_int as isize) =
        *a_0.offset(1 as std::os::raw::c_int as isize);
    *a_0.offset(1 as std::os::raw::c_int as isize) = t;
    t = *a_0.offset(2 as std::os::raw::c_int as isize);
    *a_0.offset(2 as std::os::raw::c_int as isize) =
        *b.offset(2 as std::os::raw::c_int as isize);
    *b.offset(2 as std::os::raw::c_int as isize) = t;
    t = *b.offset(3 as std::os::raw::c_int as isize);
    *b.offset(3 as std::os::raw::c_int as isize) =
        *a_0.offset(3 as std::os::raw::c_int as isize);
    *a_0.offset(3 as std::os::raw::c_int as isize) = t;
}
#[inline]
unsafe extern "C" fn zzero(mut a_0: *mut zahl) -> std::os::raw::c_int {
    return ((*a_0).sign == 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a_0: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a_0).sign = 0 as std::os::raw::c_int
    } else {
        (*a_0).sign = (*b).sign;
        (*a_0).used = (*b).used;
        if (*a_0).alloced < (*b).used { libzahl_realloc(a_0, (*b).used); }
        libzahl_memcpy((*a_0).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsetu(mut a_0: *mut zahl, mut b: uint64_t) {
    if (b == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a_0).sign = 0 as std::os::raw::c_int;
        return
    }
    if (*a_0).alloced < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        libzahl_realloc(a_0, 1 as std::os::raw::c_int as size_t);
    }
    (*a_0).sign = 1 as std::os::raw::c_int;
    *(*a_0).chars.offset(0 as std::os::raw::c_int as isize) = b;
    (*a_0).used = 1 as std::os::raw::c_int as size_t;
}
#[inline]
unsafe extern "C" fn zcmp(mut a_0: *mut zahl, mut b: *mut zahl)
 -> std::os::raw::c_int {
    if zsignum(a_0) != zsignum(b) {
        return if zsignum(a_0) < zsignum(b) {
                   -(1 as std::os::raw::c_int)
               } else { (zsignum(a_0) > zsignum(b)) as std::os::raw::c_int }
    }
    return zsignum(a_0) * zcmpmag(a_0, b);
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a_0: *mut zahl, mut b: *mut zahl)
 -> std::os::raw::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a_0) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return -((zzero(b) == 0) as std::os::raw::c_int)
    }
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 1 as std::os::raw::c_int
    }
    i = (*a_0).used.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    j = (*b).used.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    while i > j {
        if *(*a_0).chars.offset(i as isize) != 0 { return 1 as std::os::raw::c_int }
        (*a_0).used = (*a_0).used.wrapping_sub(1);
        i = i.wrapping_sub(1)
    }
    while j > i {
        if *(*b).chars.offset(j as isize) != 0 { return -(1 as std::os::raw::c_int) }
        (*b).used = (*b).used.wrapping_sub(1);
        j = j.wrapping_sub(1)
    }
    while i != 0 &&
              *(*a_0).chars.offset(i as isize) ==
                  *(*b).chars.offset(i as isize) {
        i = i.wrapping_sub(1)
    }
    return if *(*a_0).chars.offset(i as isize) <
                  *(*b).chars.offset(i as isize) {
               -(1 as std::os::raw::c_int)
           } else {
               (*(*a_0).chars.offset(i as isize) >
                    *(*b).chars.offset(i as isize)) as std::os::raw::c_int
           };
}
#[inline]
unsafe extern "C" fn zsignum(mut a_0: *mut zahl) -> std::os::raw::c_int {
    return (*a_0).sign;
}
#[inline]
unsafe extern "C" fn zcmpu(mut a_0: *mut zahl, mut b: uint64_t)
 -> std::os::raw::c_int {
    if (b == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 { return zsignum(a_0) }
    if (zsignum(a_0) <= 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        return -(1 as std::os::raw::c_int)
    }
    while *(*a_0).chars.offset((*a_0).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                   isize) == 0 {
        (*a_0).used = (*a_0).used.wrapping_sub(1)
    }
    if (*a_0).used > 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 1 as std::os::raw::c_int
    }
    return if *(*a_0).chars.offset(0 as std::os::raw::c_int as isize) < b {
               -(1 as std::os::raw::c_int)
           } else {
               (*(*a_0).chars.offset(0 as std::os::raw::c_int as isize) > b) as
                   std::os::raw::c_int
           };
}
#[inline]
unsafe extern "C" fn zlsb(mut a_0: *mut zahl) -> size_t {
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    if (zzero(a_0) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 18446744073709551615 as std::os::raw::c_ulong
    }
    while *(*a_0).chars.offset(i as isize) == 0 { i = i.wrapping_add(1) }
    i =
        (i as
             std::os::raw::c_ulong).wrapping_mul((8 as std::os::raw::c_int as
                                              std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                              as
                                                                              std::os::raw::c_ulong))
            as size_t as size_t;
    i =
        (i as
             std::os::raw::c_ulong).wrapping_add((*(*a_0).chars.offset(i as
                                                                   isize)).trailing_zeros()
                                             as i32 as size_t) as size_t as
            size_t;
    return i;
}
#[inline]
unsafe extern "C" fn zeven(mut a_0: *mut zahl) -> std::os::raw::c_int {
    return ((*a_0).sign == 0 ||
                !*(*a_0).chars.offset(0 as std::os::raw::c_int as isize) &
                    1 as std::os::raw::c_int as std::os::raw::c_ulonglong != 0) as
               std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zptest(mut witness: *mut zahl, mut n: *mut zahl,
                                mut t: std::os::raw::c_int) -> zprimality {
    /*
	 * Miller–Rabin primarlity test.
	 */
    let mut i: size_t = 0;
    let mut r: size_t = 0;
    if (zcmpu(n, 3 as std::os::raw::c_int as uint64_t) <= 0 as std::os::raw::c_int) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        if zcmpu(n, 1 as std::os::raw::c_int as uint64_t) <= 0 as std::os::raw::c_int {
            if !witness.is_null() { if witness != n { zset(witness, n); } }
            return NONPRIME
        } else { return PRIME }
    }
    if (zeven(n) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if !witness.is_null() {
            zsetu(witness, 2 as std::os::raw::c_int as uint64_t);
        }
        return NONPRIME
    }
    zsub_unsigned(libzahl_tmp_ptest_n1.as_mut_ptr(), n,
                  libzahl_const_1.as_mut_ptr());
    zsub_unsigned(libzahl_tmp_ptest_n4.as_mut_ptr(), n,
                  libzahl_const_4.as_mut_ptr());
    r = zlsb(libzahl_tmp_ptest_n1.as_mut_ptr());
    zrsh(libzahl_tmp_ptest_d.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr(),
         r);
    loop  {
        let fresh0 = t;
        t = t - 1;
        if !(fresh0 != 0) { break ; }
        zrand(libzahl_tmp_ptest_a.as_mut_ptr(), DEFAULT_RANDOM, UNIFORM,
              libzahl_tmp_ptest_n4.as_mut_ptr());
        zadd_unsigned(libzahl_tmp_ptest_a.as_mut_ptr(),
                      libzahl_tmp_ptest_a.as_mut_ptr(),
                      libzahl_const_2.as_mut_ptr());
        zmodpow(libzahl_tmp_ptest_x.as_mut_ptr(),
                libzahl_tmp_ptest_a.as_mut_ptr(),
                libzahl_tmp_ptest_d.as_mut_ptr(), n);
        if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(),
                libzahl_const_1.as_mut_ptr()) == 0 ||
               zcmp(libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_tmp_ptest_n1.as_mut_ptr()) == 0 {
            continue ;
        }
        i = 1 as std::os::raw::c_int as size_t;
        while i < r {
            zmodsqr(libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_tmp_ptest_x.as_mut_ptr(), n);
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_const_1.as_mut_ptr()) == 0 {
                if !witness.is_null() {
                    zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
                }
                return NONPRIME
            }
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_tmp_ptest_n1.as_mut_ptr()) == 0 {
                break ;
            }
            i = i.wrapping_add(1)
        }
        if i == r {
            if !witness.is_null() {
                zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
            }
            return NONPRIME
        }
    }
    return PROBABLY_PRIME;
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

