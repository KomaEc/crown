use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zadd(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed
    );
    fn zmul(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed
    );
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t
}
pub type z_t = [C2RustUnnamed; 1];
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
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zsets(
    mut a: *mut C2RustUnnamed,
    mut str: *const libc::c_char
) -> libc::c_int {
    let mut temp: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut neg: libc::c_int =
        (*str as libc::c_int == '-' as i32) as libc::c_int;
    let mut str_end: *const libc::c_char = 0 as *const libc::c_char;
    str = str.offset(
        (neg != 0 || *str as libc::c_int == '+' as i32) as libc::c_int as isize
    );
    if *str == 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    str_end = str;
    while *str_end != 0 {
        if *(*__ctype_b_loc()).offset(*str_end as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        str_end = str_end.offset(1);
        str_end;
    }
    (*a).sign = 0 as libc::c_int;
    zset(
        libzahl_tmp_str_num.as_mut_ptr(),
        libzahl_const_1e19.as_mut_ptr()
    );
    's_192: {
        let mut current_block_39: u64;
        match str_end.offset_from(str) as libc::c_long
            % 19 as libc::c_int as libc::c_long
        {
            0 => {
                current_block_39 = 5744151266997549655;
            }
            18 => {
                current_block_39 = 11176548090480166835;
            }
            17 => {
                current_block_39 = 5316576909219254384;
            }
            16 => {
                current_block_39 = 1288308673709660377;
            }
            15 => {
                current_block_39 = 10446493018551942857;
            }
            14 => {
                current_block_39 = 16157171725729039844;
            }
            13 => {
                current_block_39 = 4721959965449934127;
            }
            12 => {
                current_block_39 = 2496805846109539354;
            }
            11 => {
                current_block_39 = 2750421473661961406;
            }
            10 => {
                current_block_39 = 15967000259905517021;
            }
            9 => {
                current_block_39 = 10750807697471715846;
            }
            8 => {
                current_block_39 = 15321381213559906075;
            }
            7 => {
                current_block_39 = 9593857735485556321;
            }
            6 => {
                current_block_39 = 17405192575931126289;
            }
            5 => {
                current_block_39 = 4710239634851715620;
            }
            4 => {
                current_block_39 = 13778539474002964088;
            }
            3 => {
                current_block_39 = 12920703692983348763;
            }
            2 => {
                current_block_39 = 16633745550097629786;
            }
            1 => {
                current_block_39 = 12747732544762907134;
            }
            _ => {
                current_block_39 = 4090602189656566074;
            }
        }
        loop {
            match current_block_39 {
                4090602189656566074 => {
                    break 's_192;
                }
                5744151266997549655 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh0 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh0 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 11176548090480166835;
                }
                11176548090480166835 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh1 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh1 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 5316576909219254384;
                }
                5316576909219254384 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh2 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh2 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 1288308673709660377;
                }
                1288308673709660377 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh3 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh3 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 10446493018551942857;
                }
                10446493018551942857 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh4 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh4 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 16157171725729039844;
                }
                16157171725729039844 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh5 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh5 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 4721959965449934127;
                }
                4721959965449934127 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh6 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh6 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 2496805846109539354;
                }
                2496805846109539354 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh7 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh7 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 2750421473661961406;
                }
                2750421473661961406 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh8 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh8 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 15967000259905517021;
                }
                15967000259905517021 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh9 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh9 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 10750807697471715846;
                }
                10750807697471715846 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh10 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh10 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 15321381213559906075;
                }
                15321381213559906075 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh11 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh11 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 9593857735485556321;
                }
                9593857735485556321 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh12 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh12 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 17405192575931126289;
                }
                17405192575931126289 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh13 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh13 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 4710239634851715620;
                }
                4710239634851715620 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh14 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh14 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 13778539474002964088;
                }
                13778539474002964088 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh15 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh15 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 12920703692983348763;
                }
                12920703692983348763 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh16 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh16 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 16633745550097629786;
                }
                16633745550097629786 => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh17 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh17 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    current_block_39 = 12747732544762907134;
                }
                _ => {
                    temp = temp
                        .wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh18 = str;
                    str = str.offset(1);
                    temp = temp.wrapping_add(
                        (*fresh18 as libc::c_int & 15 as libc::c_int)
                            as libc::c_ulonglong
                    );
                    if !(temp == 0) {
                        *((*libzahl_tmp_str_num.as_mut_ptr()).chars)
                            .offset(0 as libc::c_int as isize) =
                            temp as zahl_char_t;
                        temp >>= 32 as libc::c_int;
                        *((*libzahl_tmp_str_num.as_mut_ptr()).chars)
                            .offset(1 as libc::c_int as isize) =
                            temp as zahl_char_t;
                        (*libzahl_tmp_str_num.as_mut_ptr()).used =
                            (1 as libc::c_int + (temp != 0) as libc::c_int)
                                as size_t;
                        zadd(a, a, libzahl_tmp_str_num.as_mut_ptr());
                    }
                    if !(*str != 0) {
                        current_block_39 = 4090602189656566074;
                        continue;
                    }
                    zmul(a, a, libzahl_const_1e19.as_mut_ptr());
                    temp = 0 as libc::c_int as libc::c_ulonglong;
                    current_block_39 = 5744151266997549655;
                }
            }
        }
    }
    if neg != 0 {
        (*a).sign = -zsignum(a);
    }
    return 0 as libc::c_int;
}
