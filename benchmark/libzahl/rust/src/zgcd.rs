
extern "C" {
    #[no_mangle]
    static mut libzahl_tmp_gcd_v: z_t;
    #[no_mangle]
    fn zsub_positive_assign(_: *mut zahl, _: *mut zahl);
    #[no_mangle]
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    #[no_mangle]
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    #[no_mangle]
    static mut libzahl_tmp_gcd_u: z_t;
    #[no_mangle]
    fn libzahl_realloc(_: *mut zahl, _: size_t);
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
unsafe extern "C" fn zrsh_taint(mut a: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    if (bits == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 { return }
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 { return }
    chars = bits >> 6 as std::os::raw::c_int;
    if (chars >= (*a).used || zbits(a) <= bits) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        (*a).sign = 0 as std::os::raw::c_int;
        return
    }
    bits = bits & (64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong;
    cbits = (64 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_sub(bits);
    if (chars != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).used =
            ((*a).used as std::os::raw::c_ulong).wrapping_sub(chars) as size_t as
                size_t;
        (*a).chars = (*a).chars.offset(chars as isize)
    }
    if (bits != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        *(*a).chars.offset(0 as std::os::raw::c_int as isize) >>= bits;
        i = 1 as std::os::raw::c_int as size_t;
        while i < (*a).used {
            let ref mut fresh0 =
                *(*a).chars.offset(i.wrapping_sub(1 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong) as
                                       isize);
            *fresh0 |= *(*a).chars.offset(i as isize) << cbits;
            *(*a).chars.offset(i as isize) >>= bits;
            i = i.wrapping_add(1)
        }
        while *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                     isize) == 0 {
            (*a).used = (*a).used.wrapping_sub(1)
        }
    };
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
unsafe extern "C" fn zlsb(mut a: *mut zahl) -> size_t {
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 18446744073709551615 as std::os::raw::c_ulong
    }
    while *(*a).chars.offset(i as isize) == 0 { i = i.wrapping_add(1) }
    i =
        (i as
             std::os::raw::c_ulong).wrapping_mul((8 as std::os::raw::c_int as
                                              std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                              as
                                                                              std::os::raw::c_ulong))
            as size_t as size_t;
    i =
        (i as
             std::os::raw::c_ulong).wrapping_add((*(*a).chars.offset(i as
                                                                 isize)).trailing_zeros()
                                             as i32 as size_t) as size_t as
            size_t;
    return i;
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
unsafe extern "C" fn zswap_tainted_unsigned(mut a: *mut zahl,
                                            mut b: *mut zahl) {
    let mut t: z_t =
        [zahl{sign: 0,
              padding__: 0,
              used: 0,
              alloced: 0,
              chars: 0 as *const zahl_char_t as *mut zahl_char_t,}; 1];
    (*t.as_mut_ptr()).used = (*a).used;
    (*a).used = (*b).used;
    (*b).used = (*t.as_mut_ptr()).used;
    let ref mut fresh1 = (*t.as_mut_ptr()).chars;
    *fresh1 = (*b).chars;
    (*b).chars = (*a).chars;
    (*a).chars = (*t.as_mut_ptr()).chars;
}
#[no_mangle]
pub unsafe extern "C" fn zgcd(mut a: *mut zahl, mut b: *mut zahl,
                              mut c: *mut zahl) {
    /*
	 * Binary GCD algorithm.
	 */
    let mut shifts: size_t = 0;
    let mut u_orig: *mut zahl_char_t = 0 as *mut zahl_char_t;
    let mut v_orig: *mut zahl_char_t = 0 as *mut zahl_char_t;
    let mut u_lsb: size_t = 0;
    let mut v_lsb: size_t = 0;
    let mut neg: std::os::raw::c_int = 0;
    let mut cmpmag: std::os::raw::c_int = 0;
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if a != c { zset(a, c); }
        return
    }
    if (zzero(c) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if a != b { zset(a, b); }
        return
    }
    neg = (zsignum(b) & zsignum(c) < 0 as std::os::raw::c_int) as std::os::raw::c_int;
    u_lsb = zlsb(b);
    v_lsb = zlsb(c);
    shifts = if u_lsb < v_lsb { u_lsb } else { v_lsb };
    zrsh(libzahl_tmp_gcd_u.as_mut_ptr(), b, u_lsb);
    zrsh(libzahl_tmp_gcd_v.as_mut_ptr(), c, v_lsb);
    u_orig = (*libzahl_tmp_gcd_u.as_mut_ptr()).chars;
    v_orig = (*libzahl_tmp_gcd_v.as_mut_ptr()).chars;
    loop  {
        cmpmag =
            zcmpmag(libzahl_tmp_gcd_u.as_mut_ptr(),
                    libzahl_tmp_gcd_v.as_mut_ptr());
        if (cmpmag >= 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0 {
            if (cmpmag == 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long !=
                   0 {
                break ;
            }
            zswap_tainted_unsigned(libzahl_tmp_gcd_u.as_mut_ptr(),
                                   libzahl_tmp_gcd_v.as_mut_ptr());
        }
        zsub_positive_assign(libzahl_tmp_gcd_v.as_mut_ptr(),
                             libzahl_tmp_gcd_u.as_mut_ptr());
        zrsh_taint(libzahl_tmp_gcd_v.as_mut_ptr(),
                   zlsb(libzahl_tmp_gcd_v.as_mut_ptr()));
    }
    zlsh(a, libzahl_tmp_gcd_u.as_mut_ptr(), shifts);
    (*a).sign = if neg != 0 { -(1 as std::os::raw::c_int) } else { 1 as std::os::raw::c_int };
    let ref mut fresh2 = (*libzahl_tmp_gcd_u.as_mut_ptr()).chars;
    *fresh2 = u_orig;
    let ref mut fresh3 = (*libzahl_tmp_gcd_v.as_mut_ptr()).chars;
    *fresh3 = v_orig;
}
