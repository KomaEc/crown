
extern "C" {
    
    
}
pub use crate::src::allocator::libzahl_realloc;
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
unsafe extern "C" fn zmemcpy_range(mut d: *mut zahl_char_t,
                                   mut s: *const zahl_char_t, mut i: size_t,
                                   mut n: size_t) {
    d = d.offset(i as isize);
    s = s.offset(i as isize);
    n = (n as std::os::raw::c_ulong).wrapping_sub(i) as size_t as size_t;
    libzahl_memcpy(d, s, n);
}
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zxor(mut a: *mut zahl, mut b: *mut zahl,
                              mut c: *mut zahl) {
    let mut n: size_t = 0;
    let mut m: size_t = 0;
    let mut bn: size_t = 0;
    let mut cn: size_t = 0;
    let mut bc: *const zahl_char_t = 0 as *const zahl_char_t;
    let mut cc: *const zahl_char_t = 0 as *const zahl_char_t;
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if a != c { zset(a, c); }
        return
    } else {
        if (zzero(c) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
            if a != b { zset(a, b); }
            return
        }
    }
    bn = (*b).used;
    bc = (*b).chars;
    cn = (*c).used;
    cc = (*c).chars;
    n = (if bn < cn { bn } else { cn });
    m = (if bn > cn { bn } else { cn });
    if (*a).alloced < m { libzahl_realloc(a, m); }
    if a == b {
        let mut a__: *mut zahl_char_t = (*a).chars;
        let mut b__: *const zahl_char_t = (*a).chars;
        let mut c__: *const zahl_char_t = cc;
        let mut i__: size_t = 0;
        let mut n__: size_t = n;
        if n__ <= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
            if n__ >= 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a__.offset(0 as std::os::raw::c_int as isize) =
                    *b__.offset(0 as std::os::raw::c_int as isize) ^
                        *c__.offset(0 as std::os::raw::c_int as isize)
            }
            if n__ >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a__.offset(1 as std::os::raw::c_int as isize) =
                    *b__.offset(1 as std::os::raw::c_int as isize) ^
                        *c__.offset(1 as std::os::raw::c_int as isize)
            }
            if n__ >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a__.offset(2 as std::os::raw::c_int as isize) =
                    *b__.offset(2 as std::os::raw::c_int as isize) ^
                        *c__.offset(2 as std::os::raw::c_int as isize)
            }
            if n__ >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a__.offset(3 as std::os::raw::c_int as isize) =
                    *b__.offset(3 as std::os::raw::c_int as isize) ^
                        *c__.offset(3 as std::os::raw::c_int as isize)
            }
        } else {
            i__ = 0 as std::os::raw::c_int as size_t;
            loop  {
                i__ =
                    (i__ as
                         std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        size_t as size_t;
                if !(i__ < n__) { break ; }
                *a__.offset(i__.wrapping_sub(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                    *b__.offset(i__.wrapping_sub(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                        ^
                        *c__.offset(i__.wrapping_sub(1 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                        isize);
                *a__.offset(i__.wrapping_sub(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                    *b__.offset(i__.wrapping_sub(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                        ^
                        *c__.offset(i__.wrapping_sub(2 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                        isize);
                *a__.offset(i__.wrapping_sub(3 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                    *b__.offset(i__.wrapping_sub(3 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                        ^
                        *c__.offset(i__.wrapping_sub(3 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                        isize);
                *a__.offset(i__.wrapping_sub(4 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                    *b__.offset(i__.wrapping_sub(4 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                        ^
                        *c__.offset(i__.wrapping_sub(4 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                        isize)
            }
            if i__ > n__ {
                i__ =
                    (i__ as
                         std::os::raw::c_ulong).wrapping_sub(4 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        size_t as size_t;
                while i__ < n__ {
                    *a__.offset(i__ as isize) =
                        *b__.offset(i__ as isize) ^ *c__.offset(i__ as isize);
                    i__ = i__.wrapping_add(1)
                }
            }
        }
        if (*a).used < cn { zmemcpy_range((*a).chars, cc, n, m); }
    } else if (a == c) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        let mut a___0: *mut zahl_char_t = (*a).chars;
        let mut b___0: *const zahl_char_t = (*a).chars;
        let mut c___0: *const zahl_char_t = bc;
        let mut i___0: size_t = 0;
        let mut n___0: size_t = n;
        if n___0 <= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
            if n___0 >= 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a___0.offset(0 as std::os::raw::c_int as isize) =
                    *b___0.offset(0 as std::os::raw::c_int as isize) ^
                        *c___0.offset(0 as std::os::raw::c_int as isize)
            }
            if n___0 >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a___0.offset(1 as std::os::raw::c_int as isize) =
                    *b___0.offset(1 as std::os::raw::c_int as isize) ^
                        *c___0.offset(1 as std::os::raw::c_int as isize)
            }
            if n___0 >= 3 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a___0.offset(2 as std::os::raw::c_int as isize) =
                    *b___0.offset(2 as std::os::raw::c_int as isize) ^
                        *c___0.offset(2 as std::os::raw::c_int as isize)
            }
            if n___0 >= 4 as std::os::raw::c_int as std::os::raw::c_ulong {
                *a___0.offset(3 as std::os::raw::c_int as isize) =
                    *b___0.offset(3 as std::os::raw::c_int as isize) ^
                        *c___0.offset(3 as std::os::raw::c_int as isize)
            }
        } else {
            i___0 = 0 as std::os::raw::c_int as size_t;
            loop  {
                i___0 =
                    (i___0 as
                         std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        size_t as size_t;
                if !(i___0 < n___0) { break ; }
                *a___0.offset(i___0.wrapping_sub(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    =
                    *b___0.offset(i___0.wrapping_sub(1 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize) ^
                        *c___0.offset(i___0.wrapping_sub(1 as std::os::raw::c_int as
                                                             std::os::raw::c_ulong) as
                                          isize);
                *a___0.offset(i___0.wrapping_sub(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    =
                    *b___0.offset(i___0.wrapping_sub(2 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize) ^
                        *c___0.offset(i___0.wrapping_sub(2 as std::os::raw::c_int as
                                                             std::os::raw::c_ulong) as
                                          isize);
                *a___0.offset(i___0.wrapping_sub(3 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    =
                    *b___0.offset(i___0.wrapping_sub(3 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize) ^
                        *c___0.offset(i___0.wrapping_sub(3 as std::os::raw::c_int as
                                                             std::os::raw::c_ulong) as
                                          isize);
                *a___0.offset(i___0.wrapping_sub(4 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    =
                    *b___0.offset(i___0.wrapping_sub(4 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize) ^
                        *c___0.offset(i___0.wrapping_sub(4 as std::os::raw::c_int as
                                                             std::os::raw::c_ulong) as
                                          isize)
            }
            if i___0 > n___0 {
                i___0 =
                    (i___0 as
                         std::os::raw::c_ulong).wrapping_sub(4 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        size_t as size_t;
                while i___0 < n___0 {
                    *a___0.offset(i___0 as isize) =
                        *b___0.offset(i___0 as isize) ^
                            *c___0.offset(i___0 as isize);
                    i___0 = i___0.wrapping_add(1)
                }
            }
        }
        if (*a).used < bn { zmemcpy_range((*a).chars, bc, n, m); }
    } else if m == bn {
        let mut a___1: *mut zahl_char_t = (*a).chars;
        let mut b___1: *const zahl_char_t = (*c).chars;
        let mut c___1: *const zahl_char_t = (*b).chars;
        let mut i___1: size_t = 0;
        let mut n___1: size_t = n;
        i___1 = 0 as std::os::raw::c_int as size_t;
        while i___1 < n___1 {
            *a___1.offset(i___1.wrapping_add(0 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___1.offset(i___1.wrapping_add(0 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___1.offset(i___1.wrapping_add(0 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            *a___1.offset(i___1.wrapping_add(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___1.offset(i___1.wrapping_add(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___1.offset(i___1.wrapping_add(1 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            *a___1.offset(i___1.wrapping_add(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___1.offset(i___1.wrapping_add(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___1.offset(i___1.wrapping_add(2 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            *a___1.offset(i___1.wrapping_add(3 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___1.offset(i___1.wrapping_add(3 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___1.offset(i___1.wrapping_add(3 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            i___1 =
                (i___1 as
                     std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t
        }
        zmemcpy_range((*a).chars, (*b).chars, n, m);
    } else {
        let mut a___2: *mut zahl_char_t = (*a).chars;
        let mut b___2: *const zahl_char_t = (*b).chars;
        let mut c___2: *const zahl_char_t = (*c).chars;
        let mut i___2: size_t = 0;
        let mut n___2: size_t = n;
        i___2 = 0 as std::os::raw::c_int as size_t;
        while i___2 < n___2 {
            *a___2.offset(i___2.wrapping_add(0 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___2.offset(i___2.wrapping_add(0 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___2.offset(i___2.wrapping_add(0 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            *a___2.offset(i___2.wrapping_add(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___2.offset(i___2.wrapping_add(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___2.offset(i___2.wrapping_add(1 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            *a___2.offset(i___2.wrapping_add(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___2.offset(i___2.wrapping_add(2 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___2.offset(i___2.wrapping_add(2 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            *a___2.offset(i___2.wrapping_add(3 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as isize) =
                *b___2.offset(i___2.wrapping_add(3 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as isize)
                    ^
                    *c___2.offset(i___2.wrapping_add(3 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                                      isize);
            i___2 =
                (i___2 as
                     std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t
        }
        zmemcpy_range((*a).chars, (*c).chars, n, m);
    }
    (*a).used = m;
    while (*a).used != 0 &&
              *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                     isize) == 0 {
        (*a).used = (*a).used.wrapping_sub(1)
    }
    (*a).sign =
        if (*a).used != 0 {
            (1 as std::os::raw::c_int) -
                2 as std::os::raw::c_int *
                    (zsignum(b) ^ zsignum(c) < 0 as std::os::raw::c_int) as
                        std::os::raw::c_int
        } else { 0 as std::os::raw::c_int };
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

