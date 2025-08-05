use ::libc;
extern "C" {
    fn zswap(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zcmp(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsub_unsigned(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed
    );
    fn zlsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zlsb(_: *mut C2RustUnnamed) -> size_t;
    static mut libzahl_tmp_gcd_v: z_t;
    static mut libzahl_tmp_gcd_u: z_t;
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
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zgcd(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed
) {
    let mut current_block: u64;
    let mut shifts: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut min: size_t = 0;
    let mut uv: zahl_char_t = 0;
    let mut bit: zahl_char_t = 0;
    let mut neg: libc::c_int = 0;
    if zcmp(b, c) == 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    if zzero(b) != 0 {
        if a != c {
            zset(a, c);
        }
        return;
    }
    if zzero(c) != 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    zabs(libzahl_tmp_gcd_u.as_mut_ptr(), b);
    zabs(libzahl_tmp_gcd_v.as_mut_ptr(), c);
    neg = (zsignum(b) < 0 as libc::c_int && zsignum(c) < 0 as libc::c_int)
        as libc::c_int;
    min = if (*libzahl_tmp_gcd_u.as_mut_ptr()).used
        < (*libzahl_tmp_gcd_v.as_mut_ptr()).used
    {
        (*libzahl_tmp_gcd_u.as_mut_ptr()).used
    } else {
        (*libzahl_tmp_gcd_v.as_mut_ptr()).used
    };
    's_104: loop {
        if !(i < min) {
            current_block = 2232869372362427478;
            break;
        }
        uv = *((*libzahl_tmp_gcd_u.as_mut_ptr()).chars).offset(i as isize)
            | *((*libzahl_tmp_gcd_v.as_mut_ptr()).chars).offset(i as isize);
        bit = 1 as libc::c_int as zahl_char_t;
        while bit != 0 {
            if uv & bit != 0 {
                current_block = 10096062616462218189;
                break 's_104;
            }
            bit <<= 1 as libc::c_int;
            shifts = shifts.wrapping_add(1);
            shifts;
        }
        i = i.wrapping_add(1);
        i;
    }
    's_132: loop {
        match current_block {
            10096062616462218189 => {
                zrsh(
                    libzahl_tmp_gcd_u.as_mut_ptr(),
                    libzahl_tmp_gcd_u.as_mut_ptr(),
                    shifts
                );
                break;
            }
            _ => {
                if i < (*libzahl_tmp_gcd_u.as_mut_ptr()).used {
                    bit = 1 as libc::c_int as zahl_char_t;
                    while bit != 0 {
                        if *((*libzahl_tmp_gcd_u.as_mut_ptr()).chars)
                            .offset(i as isize)
                            & bit
                            != 0
                        {
                            current_block = 10096062616462218189;
                            continue 's_132;
                        }
                        bit <<= 1 as libc::c_int;
                        shifts = shifts.wrapping_add(1);
                        shifts;
                    }
                    i = i.wrapping_add(1);
                    i;
                    current_block = 2232869372362427478;
                } else {
                    's_154: loop {
                        if !(i < (*libzahl_tmp_gcd_v.as_mut_ptr()).used) {
                            current_block = 10096062616462218189;
                            break;
                        }
                        bit = 1 as libc::c_int as zahl_char_t;
                        while bit != 0 {
                            if *((*libzahl_tmp_gcd_v.as_mut_ptr()).chars)
                                .offset(i as isize)
                                & bit
                                != 0
                            {
                                current_block = 10096062616462218189;
                                break 's_154;
                            }
                            bit <<= 1 as libc::c_int;
                            shifts = shifts.wrapping_add(1);
                            shifts;
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                }
            }
        }
    }
    zrsh(
        libzahl_tmp_gcd_v.as_mut_ptr(),
        libzahl_tmp_gcd_v.as_mut_ptr(),
        shifts
    );
    zrsh(
        libzahl_tmp_gcd_u.as_mut_ptr(),
        libzahl_tmp_gcd_u.as_mut_ptr(),
        zlsb(libzahl_tmp_gcd_u.as_mut_ptr())
    );
    loop {
        zrsh(
            libzahl_tmp_gcd_v.as_mut_ptr(),
            libzahl_tmp_gcd_v.as_mut_ptr(),
            zlsb(libzahl_tmp_gcd_v.as_mut_ptr())
        );
        if zcmpmag(
            libzahl_tmp_gcd_u.as_mut_ptr(),
            libzahl_tmp_gcd_v.as_mut_ptr()
        ) > 0 as libc::c_int
        {
            zswap(
                libzahl_tmp_gcd_u.as_mut_ptr(),
                libzahl_tmp_gcd_v.as_mut_ptr()
            );
        }
        zsub_unsigned(
            libzahl_tmp_gcd_v.as_mut_ptr(),
            libzahl_tmp_gcd_v.as_mut_ptr(),
            libzahl_tmp_gcd_u.as_mut_ptr()
        );
        if !(zzero(libzahl_tmp_gcd_v.as_mut_ptr()) == 0) {
            break;
        }
    }
    zlsh(a, libzahl_tmp_gcd_u.as_mut_ptr(), shifts);
    (*a).sign = if neg != 0 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
