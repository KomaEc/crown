
extern "C" {
    
    
    
    
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zsub::zsub_unsigned;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
#[inline]
unsafe extern "C" fn libzahl_memcpy(mut d: *mut zahl_char_t,
                                    mut s: *const zahl_char_t,
                                    mut n: size_t) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
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
                 : "=r" (t), "+r" (d), "+r" (s), "+r" (n) : : : "volatile");
            current_block_42 = 6717214610478484138;
        }
    }
    match current_block_42 {
        16314148179394187963 => {
            *d.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 23778815036459645;
        }
        _ => { }
    }
    match current_block_42 {
        23778815036459645 => {
            *d.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 17409214555335211048;
        }
        _ => { }
    }
    match current_block_42 {
        17409214555335211048 => {
            *d.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15059990250786981251;
        }
        _ => { }
    }
    match current_block_42 {
        15059990250786981251 => {
            *d.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 6620638995229345271;
        }
        _ => { }
    }
    match current_block_42 {
        6620638995229345271 => {
            *d.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 3600730107057416047;
        }
        _ => { }
    }
    match current_block_42 {
        3600730107057416047 => {
            *d.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 354696938566919285;
        }
        _ => { }
    }
    match current_block_42 {
        354696938566919285 => {
            *d.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 8395813788269618564;
        }
        _ => { }
    }
    match current_block_42 {
        8395813788269618564 => {
            *d.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16693777636139501188;
        }
        _ => { }
    }
    match current_block_42 {
        16693777636139501188 => {
            *d.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 905962748010064685;
        }
        _ => { }
    }
    match current_block_42 {
        905962748010064685 => {
            *d.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 9361680084571800209;
        }
        _ => { }
    }
    match current_block_42 {
        9361680084571800209 => {
            *d.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 14892601158468998162;
        }
        _ => { }
    }
    match current_block_42 {
        14892601158468998162 => {
            *d.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 5996064384694841776;
        }
        _ => { }
    }
    match current_block_42 {
        5996064384694841776 => {
            *d.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 2207944898923095617;
        }
        _ => { }
    }
    match current_block_42 {
        2207944898923095617 => {
            *d.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 11359770377620528283;
        }
        _ => { }
    }
    match current_block_42 {
        11359770377620528283 => {
            *d.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 13725184785425480707;
        }
        _ => { }
    }
    match current_block_42 {
        13725184785425480707 => {
            *d.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 1832949796153364909;
        }
        _ => { }
    }
    match current_block_42 {
        1832949796153364909 => {
            *d.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15100265573273486990;
        }
        _ => { }
    }
    match current_block_42 {
        15100265573273486990 => {
            *d.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 12150485915963817543;
        }
        _ => { }
    }
    match current_block_42 {
        12150485915963817543 => {
            *d.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)
        }
        _ => { }
    };
}
#[inline]
unsafe extern "C" fn libzahl_memset(mut a: *mut zahl_char_t,
                                    mut v: zahl_char_t, mut n: size_t) {
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < n {
        *a.offset(i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        *a.offset(i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        *a.offset(i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        *a.offset(i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
            = v;
        i =
            (i as
                 std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> std::os::raw::c_int {
    return ((*a).sign == 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used { libzahl_realloc(a, (*b).used); }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> std::os::raw::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zabs(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b { zset(a, b); }
    (*a).sign &= 1 as std::os::raw::c_int;
}
/* See LICENSE file for copyright and license details. */
/*
 * I have already tried setc, cmovnc, cmovc, and adc,
 * instead of the last four lines. There does not seem
 * to be any better why to store the carry flag.
 */
#[inline]
unsafe extern "C" fn zadd_impl_4(mut a: *mut zahl, mut b: *mut zahl,
                                 mut c: *mut zahl, mut n: size_t) {
    let mut ac: *mut zahl_char_t = (*a).chars;
    let mut bc: *mut zahl_char_t = (*b).chars;
    let mut cc: *mut zahl_char_t = (*c).chars;
    let mut carry: zahl_char_t = 0 as std::os::raw::c_int as zahl_char_t;
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    loop  {
        ac = ac.offset(4 as std::os::raw::c_int as isize);
        bc = bc.offset(4 as std::os::raw::c_int as isize);
        cc = cc.offset(4 as std::os::raw::c_int as isize);
        i =
            (i as
                 std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        if !(i <= n) { break ; }
        llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq -32($3), $0\n    movq $0, -32($1)\n    movq -24($2), $0\n    adcq -24($3), $0\n    movq $0, -24($1)\n    movq -16($2), $0\n    adcq -16($3), $0\n    movq $0, -16($1)\n    movq -8($2), $0\n    adcq -8($3), $0\n    movq $0, -8($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
             : "+r" (carry), "+r" (ac), "+r" (bc), "+r" (cc) : : : "volatile")
    }
    match n & 3 as std::os::raw::c_int as std::os::raw::c_ulong {
        3 => {
            llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq -32($3), $0\n    movq $0, -32($1)\n    movq -24($2), $0\n    adcq -24($3), $0\n    movq $0, -24($1)\n    movq -16($2), $0\n    adcq -16($3), $0\n    movq $0, -16($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
                 : "+r" (carry), "+r" (ac), "+r" (bc), "+r" (cc) : : :
                 "volatile")
        }
        2 => {
            llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq -32($3), $0\n    movq $0, -32($1)\n    movq -24($2), $0\n    adcq -24($3), $0\n    movq $0, -24($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
                 : "+r" (carry), "+r" (ac), "+r" (bc), "+r" (cc) : : :
                 "volatile")
        }
        1 => {
            llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq -32($3), $0\n    movq $0, -32($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
                 : "+r" (carry), "+r" (ac), "+r" (bc), "+r" (cc) : : :
                 "volatile")
        }
        _ => { }
    }
    i = n;
    while carry != 0 {
        let (fresh0, fresh1) =
            (*(*a).chars.offset(i as isize) as
                 std::os::raw::c_ulong).overflowing_add(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong);
        *((*a).chars.offset(i as isize) as *mut std::os::raw::c_ulong) = fresh0;
        carry = fresh1 as zahl_char_t;
        i = i.wrapping_add(1)
    }
    if (*a).used < i { (*a).used = i };
}
#[inline]
unsafe extern "C" fn zadd_impl_3(mut a: *mut zahl, mut b: *mut zahl,
                                 mut n: size_t) {
    let mut ac: *mut zahl_char_t = (*a).chars;
    let mut bc: *mut zahl_char_t = (*b).chars;
    let mut carry: zahl_char_t = 0 as std::os::raw::c_int as zahl_char_t;
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    loop  {
        ac = ac.offset(4 as std::os::raw::c_int as isize);
        bc = bc.offset(4 as std::os::raw::c_int as isize);
        i =
            (i as
                 std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        if !(i <= n) { break ; }
        llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq $0, -32($1)\n    movq -24($2), $0\n    adcq $0, -24($1)\n    movq -16($2), $0\n    adcq $0, -16($1)\n    movq -8($2), $0\n    adcq $0, -8($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
             : "+r" (carry), "+r" (ac), "+r" (bc) : : : "volatile")
    }
    match n & 3 as std::os::raw::c_int as std::os::raw::c_ulong {
        3 => {
            llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq $0, -32($1)\n    movq -24($2), $0\n    adcq $0, -24($1)\n    movq -16($2), $0\n    adcq $0, -16($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
                 : "+r" (carry), "+r" (ac), "+r" (bc) : : : "volatile")
        }
        2 => {
            llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq $0, -32($1)\n    movq -24($2), $0\n    adcq $0, -24($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
                 : "+r" (carry), "+r" (ac), "+r" (bc) : : : "volatile")
        }
        1 => {
            llvm_asm!("\n    addq $$-1, $0\n    movq -32($2), $0\n    adcq $0, -32($1)\n    movq $$1, $0\n    jc 1f\n    movq $$0, $0\n 1:"
                 : "+r" (carry), "+r" (ac), "+r" (bc) : : : "volatile")
        }
        _ => { }
    }
    i = n;
    while carry != 0 {
        let (fresh2, fresh3) =
            (*(*a).chars.offset(i as isize) as
                 std::os::raw::c_ulong).overflowing_add(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong);
        *((*a).chars.offset(i as isize) as *mut std::os::raw::c_ulong) = fresh2;
        carry = fresh3 as zahl_char_t;
        i = i.wrapping_add(1)
    }
    if (*a).used < i { (*a).used = i };
}
#[inline]
unsafe extern "C" fn libzahl_zadd_unsigned(mut a: *mut zahl, mut b: *mut zahl,
                                           mut c: *mut zahl) {
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        zabs(a, c);
        return
    } else {
        if (zzero(c) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
            zabs(a, b);
            return
        }
    }
    size = if (*b).used > (*c).used { (*b).used } else { (*c).used };
    n = (*b).used.wrapping_add((*c).used).wrapping_sub(size);
    if (*a).alloced < size.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
        libzahl_realloc(a,
                        size.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
    }
    *(*a).chars.offset(size as isize) = 0 as std::os::raw::c_int as zahl_char_t;
    if a == b {
        if (*a).used < (*c).used {
            n = (*c).used;
            libzahl_memset((*a).chars.offset((*a).used as isize),
                           0 as std::os::raw::c_int as zahl_char_t,
                           n.wrapping_sub((*a).used));
        }
        zadd_impl_3(a, c, n);
    } else if (a == c) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if (*a).used < (*b).used {
            n = (*b).used;
            libzahl_memset((*a).chars.offset((*a).used as isize),
                           0 as std::os::raw::c_int as zahl_char_t,
                           n.wrapping_sub((*a).used));
        }
        zadd_impl_3(a, b, n);
    } else if ((*b).used > (*c).used) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        libzahl_memcpy((*a).chars.offset(n as isize),
                       (*b).chars.offset(n as isize), size.wrapping_sub(n));
        (*a).used = size;
        zadd_impl_4(a, b, c, n);
    } else {
        libzahl_memcpy((*a).chars.offset(n as isize),
                       (*c).chars.offset(n as isize), size.wrapping_sub(n));
        (*a).used = size;
        zadd_impl_4(a, b, c, n);
    }
    (*a).sign = 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zadd_unsigned(mut a: *mut zahl, mut b: *mut zahl,
                                       mut c: *mut zahl) {
    libzahl_zadd_unsigned(a, b, c);
}
#[no_mangle]
pub unsafe extern "C" fn zadd_unsigned_assign(mut a: *mut zahl,
                                              mut b: *mut zahl) {
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        zabs(a, b);
        return
    } else {
        if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 { return }
    }
    size = if (*a).used > (*b).used { (*a).used } else { (*b).used };
    n = (*a).used.wrapping_add((*b).used).wrapping_sub(size);
    if (*a).alloced < size.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
        libzahl_realloc(a,
                        size.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
    }
    *(*a).chars.offset(size as isize) = 0 as std::os::raw::c_int as zahl_char_t;
    if (*a).used < (*b).used {
        n = (*b).used;
        libzahl_memset((*a).chars.offset((*a).used as isize),
                       0 as std::os::raw::c_int as zahl_char_t,
                       n.wrapping_sub((*a).used));
    }
    zadd_impl_3(a, b, n);
    (*a).sign = 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zadd(mut a: *mut zahl, mut b: *mut zahl,
                              mut c: *mut zahl) {
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if a != c { zset(a, c); }
    } else if (zzero(c) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if a != b { zset(a, b); }
    } else if (zsignum(b) < 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long
                  != 0 {
        if zsignum(c) < 0 as std::os::raw::c_int {
            libzahl_zadd_unsigned(a, b, c);
            (*a).sign = -zsignum(a)
        } else { zsub_unsigned(a, c, b); }
    } else if (zsignum(c) < 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long
                  != 0 {
        zsub_unsigned(a, b, c);
    } else { libzahl_zadd_unsigned(a, b, c); };
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

