use ::libc;
extern "C" {
    
    
    
    
    
    
    
    
    
    static mut libzahl_tmp_gcd_u: z_t;
    static mut libzahl_tmp_gcd_v: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor15 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zgcd(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut current_block: u64;
    let mut shifts = 0 as libc::c_int as size_t;
    let mut i = 0 as libc::c_int as size_t;
    let mut min: size_t = 0;
    let mut uv: zahl_char_t = 0;
    let mut bit: zahl_char_t = 0;
    let mut neg: libc::c_int = 0;
    if crate::src::zcmp::zcmp(b, c) == 0 {
        if a != b {
            crate::src::zset::zset(a, b);
        }
        return;
    }
    if zzero(b) != 0 {
        if a != c {
            crate::src::zset::zset(a, c);
        }
        return;
    }
    if zzero(c) != 0 {
        if a != b {
            crate::src::zset::zset(a, b);
        }
        return;
    }
    crate::src::zabs::zabs(crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(), b);
    crate::src::zabs::zabs(crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr(), c);
    neg= (zsignum(b) < 0 as libc::c_int && zsignum(c) < 0 as libc::c_int)
        as libc::c_int;
    min= if (*crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr()).used
        < (*crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr()).used
    {
        (*crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr()).used
    } else {
        (*crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr()).used
    };
    's_124: loop {
        if !(i < min) {
            current_block= 11459959175219260272;
            break;
        }
        uv= *((*crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr()).chars).offset(i as isize)
            | *((*crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr()).chars).offset(i as isize);
        bit= 1 as libc::c_int as zahl_char_t;
        while bit != 0 {
            if uv & bit != 0 {
                current_block= 4217392055787675399;
                break 's_124;
            }
            bit<<= 1 as libc::c_int;
            shifts= shifts.wrapping_add(1);
        }
        i= i.wrapping_add(1);
    }
    's_155: loop {
        match current_block {
            4217392055787675399 => {
                crate::src::zrsh::zrsh(
                    crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(),
                    crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(),
                    shifts,
                );
                break;
            }
            _ => {
                if i < (*crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr()).used {
                    bit= 1 as libc::c_int as zahl_char_t;
                    while bit != 0 {
                        if *((*crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr()).chars).offset(i as isize)
                            & bit != 0
                        {
                            current_block= 4217392055787675399;
                            continue 's_155;
                        }
                        bit<<= 1 as libc::c_int;
                        shifts= shifts.wrapping_add(1);
                    }
                    i= i.wrapping_add(1);
                    current_block= 11459959175219260272;
                } else {
                    's_178: loop {
                        if !(i < (*crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr()).used) {
                            current_block= 4217392055787675399;
                            break;
                        }
                        bit= 1 as libc::c_int as zahl_char_t;
                        while bit != 0 {
                            if *((*crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr()).chars)
                                .offset(i as isize) & bit != 0
                            {
                                current_block= 4217392055787675399;
                                break 's_178;
                            }
                            bit<<= 1 as libc::c_int;
                            shifts= shifts.wrapping_add(1);
                        }
                        i= i.wrapping_add(1);
                    }
                }
            }
        }
    }
    crate::src::zrsh::zrsh(crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr(), crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr(), shifts);
    crate::src::zrsh::zrsh(
        crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(),
        crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(),
        crate::src::zlsb::zlsb(crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr()),
    );
    loop {
        crate::src::zrsh::zrsh(
            crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr(),
            crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr(),
            crate::src::zlsb::zlsb(crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr()),
        );
        if crate::src::zcmpmag::zcmpmag(crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(), crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr())
            > 0 as libc::c_int
        {
            crate::src::zswap::zswap(crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr().as_mut(), crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr().as_mut());
        }
        crate::src::zsub::zsub_unsigned(
            crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr(),
            crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr(),
            crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(),
        );
        if !(zzero(crate::src::zgcd::libzahl_tmp_gcd_v.as_mut_ptr()) == 0) {
            break;
        }
    }
    crate::src::zlsh::zlsh(a, crate::src::zgcd::libzahl_tmp_gcd_u.as_mut_ptr(), shifts);
    (*a).sign= if neg != 0 { -(1 as libc::c_int) } else { 1 as libc::c_int };
}
