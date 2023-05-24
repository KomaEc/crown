
extern "C" {
    
    
    
    
    
    
    #[no_mangle]
    static mut libzahl_tmp_str_num: z_t;
    #[no_mangle]
    static mut libzahl_const_1e19: z_t;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn isdigit(_: std::os::raw::c_int) -> std::os::raw::c_int;
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zadd::zadd;
pub use crate::src::zmul::zmul_ll;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
pub use crate::src::zdivmod::z_t;
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
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zsets(mut a: *mut zahl, mut str: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut temp: std::os::raw::c_ulonglong = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
    let mut neg: std::os::raw::c_int =
        (*str as std::os::raw::c_int == '-' as i32) as std::os::raw::c_int;
    let mut str_end: *const std::os::raw::c_char = core::ptr::null();
    str =
        str.offset((neg != 0 || *str as std::os::raw::c_int == '+' as i32) as
                       std::os::raw::c_int as isize);
    if (*str == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        *__error() = 22 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    str_end = str;
    while *str_end != 0 {
        if (isdigit(*str_end as std::os::raw::c_int) == 0) as std::os::raw::c_int as
               std::os::raw::c_long != 0 {
            *__error() = 22 as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
        str_end = str_end.offset(1)
    }
    (*a).sign = 0 as std::os::raw::c_int;
    zset(libzahl_tmp_str_num.as_mut_ptr(), libzahl_const_1e19.as_mut_ptr());
    's_222:
        {
            let mut current_block_36: u64;
            match str_end.offset_from(str) as std::os::raw::c_long %
                      19 as std::os::raw::c_int as std::os::raw::c_long {
                0 => { current_block_36 = 10213458796435641725; }
                18 => { current_block_36 = 13048035770682162122; }
                17 => { current_block_36 = 7666058323665908957; }
                16 => { current_block_36 = 17704879557454781852; }
                15 => { current_block_36 = 17775755436130248246; }
                14 => { current_block_36 = 8992350983037849588; }
                13 => { current_block_36 = 18162263098627049655; }
                12 => { current_block_36 = 2095308920063660201; }
                11 => { current_block_36 = 13589375657124263157; }
                10 => { current_block_36 = 17178013025578009494; }
                9 => { current_block_36 = 5416417228479996912; }
                8 => { current_block_36 = 4003266481717755314; }
                7 => { current_block_36 = 9133810627388831416; }
                6 => { current_block_36 = 8627767324842616562; }
                5 => { current_block_36 = 13238714754418067769; }
                4 => { current_block_36 = 15592315788776765083; }
                3 => { current_block_36 = 867551788711315662; }
                2 => { current_block_36 = 3134946140800451769; }
                1 => { current_block_36 = 16940368978273450149; }
                _ => { current_block_36 = 1847472278776910194; }
            }
            loop  {
                match current_block_36 {
                    1847472278776910194 => { break 's_222 ; }
                    10213458796435641725 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh0 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh0 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 13048035770682162122;
                    }
                    13048035770682162122 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh1 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh1 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 7666058323665908957;
                    }
                    7666058323665908957 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh2 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh2 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 17704879557454781852;
                    }
                    17704879557454781852 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh3 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh3 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 17775755436130248246;
                    }
                    17775755436130248246 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh4 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh4 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 8992350983037849588;
                    }
                    8992350983037849588 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh5 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh5 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 18162263098627049655;
                    }
                    18162263098627049655 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh6 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh6 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 2095308920063660201;
                    }
                    2095308920063660201 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh7 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh7 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 13589375657124263157;
                    }
                    13589375657124263157 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh8 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh8 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 17178013025578009494;
                    }
                    17178013025578009494 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh9 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh9 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 5416417228479996912;
                    }
                    5416417228479996912 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh10 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh10 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 4003266481717755314;
                    }
                    4003266481717755314 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh11 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh11 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 9133810627388831416;
                    }
                    9133810627388831416 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh12 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh12 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 8627767324842616562;
                    }
                    8627767324842616562 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh13 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh13 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 13238714754418067769;
                    }
                    13238714754418067769 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh14 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh14 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 15592315788776765083;
                    }
                    15592315788776765083 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh15 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh15 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 867551788711315662;
                    }
                    867551788711315662 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh16 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh16 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 3134946140800451769;
                    }
                    3134946140800451769 => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh17 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh17 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        current_block_36 = 16940368978273450149;
                    }
                    _ => {
                        temp =
                            temp.wrapping_mul(10 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong);
                        let fresh18 = str;
                        str = str.offset(1);
                        temp =
                            temp.wrapping_add((*fresh18 as std::os::raw::c_int &
                                                   15 as std::os::raw::c_int) as
                                                  std::os::raw::c_ulonglong);
                        if !(temp == 0) {
                            *(*libzahl_tmp_str_num.as_mut_ptr()).chars.offset(0
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  isize)
                                = temp;
                            zadd(a, a, libzahl_tmp_str_num.as_mut_ptr());
                        }
                        if !(*str != 0) {
                            current_block_36 = 1847472278776910194;
                            continue ;
                        }
                        zmul(a, a, libzahl_const_1e19.as_mut_ptr());
                        temp = 0 as std::os::raw::c_int as std::os::raw::c_ulonglong;
                        current_block_36 = 10213458796435641725;
                    }
                }
            }
        }
    if (neg != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = -zsignum(a)
    }
    return 0 as std::os::raw::c_int;
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

