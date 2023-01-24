use ::libc;
extern "C" {
    
    
    
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor46 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zsets(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut temp = 0 as libc::c_int as libc::c_ulonglong;
    let mut neg = ((*str) as libc::c_int == '-' as i32) as libc::c_int;
    let mut str_end = 0 as *const libc::c_char;
    str= str
        .offset((neg != 0 || (*str) as libc::c_int == '+' as i32) as libc::c_int as isize);
    if (*str) == 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    str_end= str;
    while (*str_end) != 0 {
        if *(*__ctype_b_loc()).offset((*str_end) as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        str_end= str_end.offset(1);
    }
    (*a).sign= 0 as libc::c_int;
    crate::src::zset::zset(crate::src::zsets::libzahl_tmp_str_num.as_mut_ptr(), crate::src::zsets::libzahl_const_1e19.as_mut_ptr());
    's_234: {
        let mut current_block_39: u64;
        match  str_end.offset_from(str) as libc::c_long
            % 19 as libc::c_int as libc::c_long
        {
            0 => {
                current_block_39= 2009900886467833939;
            }
            18 => {
                current_block_39= 15121320231496893747;
            }
            17 => {
                current_block_39= 16368488404610608819;
            }
            16 => {
                current_block_39= 2817806250783016588;
            }
            15 => {
                current_block_39= 8436272415664805267;
            }
            14 => {
                current_block_39= 17260602820350443736;
            }
            13 => {
                current_block_39= 13506216435796342560;
            }
            12 => {
                current_block_39= 3798081891250826146;
            }
            11 => {
                current_block_39= 3337010891255527766;
            }
            10 => {
                current_block_39= 14701426646074699728;
            }
            9 => {
                current_block_39= 7675967895758458727;
            }
            8 => {
                current_block_39= 17228944538289729950;
            }
            7 => {
                current_block_39= 16906717074776190969;
            }
            6 => {
                current_block_39= 5148048065308018341;
            }
            5 => {
                current_block_39= 18210558662916816231;
            }
            4 => {
                current_block_39= 13503438682059240994;
            }
            3 => {
                current_block_39= 5618369753603485945;
            }
            2 => {
                current_block_39= 12749676338018479376;
            }
            1 => {
                current_block_39= 13992101357592761495;
            }
            _ => {
                current_block_39= 1434579379687443766;
            }
        }
        loop {
            match current_block_39 {
                1434579379687443766 => {
                    break 's_234;
                }
                2009900886467833939 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh0 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh0) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 15121320231496893747;
                }
                15121320231496893747 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh1 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh1) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 16368488404610608819;
                }
                16368488404610608819 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh2 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh2) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 2817806250783016588;
                }
                2817806250783016588 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh3 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh3) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 8436272415664805267;
                }
                8436272415664805267 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh4 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh4) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 17260602820350443736;
                }
                17260602820350443736 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh5 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh5) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 13506216435796342560;
                }
                13506216435796342560 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh6 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh6) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 3798081891250826146;
                }
                3798081891250826146 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh7 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh7) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 3337010891255527766;
                }
                3337010891255527766 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh8 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh8) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 14701426646074699728;
                }
                14701426646074699728 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh9 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh9) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 7675967895758458727;
                }
                7675967895758458727 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh10 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh10) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 17228944538289729950;
                }
                17228944538289729950 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh11 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh11) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 16906717074776190969;
                }
                16906717074776190969 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh12 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh12) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 5148048065308018341;
                }
                5148048065308018341 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh13 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh13) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 18210558662916816231;
                }
                18210558662916816231 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh14 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh14) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 13503438682059240994;
                }
                13503438682059240994 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh15 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh15) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 5618369753603485945;
                }
                5618369753603485945 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh16 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh16) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 12749676338018479376;
                }
                12749676338018479376 => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh17 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh17) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_39= 13992101357592761495;
                }
                _ => {
                    temp= temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh18 = str;
                    str= str.offset(1);
                    temp= temp
                        .wrapping_add(
                            ((*fresh18) as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    if !(temp == 0) {
                        *((*crate::src::zsets::libzahl_tmp_str_num.as_mut_ptr()).chars)
                            .offset(0 as libc::c_int as isize) = temp as zahl_char_t;
                        temp>>= 32 as libc::c_int;
                        *((*crate::src::zsets::libzahl_tmp_str_num.as_mut_ptr()).chars)
                            .offset(1 as libc::c_int as isize) = temp as zahl_char_t;
                        (*crate::src::zsets::libzahl_tmp_str_num.as_mut_ptr())
                            .used = (1 as libc::c_int + (temp != 0) as libc::c_int)
                            as size_t;
                        crate::src::zadd::zadd(a, a, crate::src::zsets::libzahl_tmp_str_num.as_mut_ptr());
                    }
                    if !((*str) != 0) {
                        current_block_39= 1434579379687443766;
                        continue;
                    }
                    crate::src::zmul::zmul(a, a, crate::src::zsets::libzahl_const_1e19.as_mut_ptr());
                    temp= 0 as libc::c_int as libc::c_ulonglong;
                    current_block_39= 2009900886467833939;
                }
            }
        }
    }
    if neg != 0 {
        (*a).sign= -zsignum(a);
    }
    return 0 as libc::c_int;
}
