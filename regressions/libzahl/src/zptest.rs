use ::libc;
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_const_1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_const_2: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    
    static mut libzahl_const_4: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor38 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
pub type zprimality = libc::c_uint;
pub const PRIME: zprimality = 2;
pub const PROBABLY_PRIME: zprimality = 1;
pub const NONPRIME: zprimality = 0;
pub type zranddev = libc::c_uint;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub type zranddist = libc::c_uint;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
#[inline]
unsafe extern "C" fn zeven(mut a_0: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a_0).sign == 0
        || *(*a_0).chars.offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_uint == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zptest(
    mut witness: *mut crate::src::allocator::C2RustUnnamed,
    mut n: *mut crate::src::allocator::C2RustUnnamed,
    mut t: libc::c_int,
) -> zprimality {
    let mut i: size_t = 0;
    let mut r: size_t = 0;
    if crate::src::zcmpu::zcmpu(n, 3 as libc::c_int as libc::c_ulonglong) <= 0 as libc::c_int {
        if crate::src::zcmpu::zcmpu(n, 1 as libc::c_int as libc::c_ulonglong) <= 0 as libc::c_int {
            if !witness.is_null() {
                if witness != n {
                    crate::src::zset::zset(witness, n);
                }
            }else { (); }
            return NONPRIME;
        } else {
            return PRIME
        }
    }
    if zeven(n) != 0 {
        if !witness.is_null() {
            if witness != n {
                crate::src::zset::zset(witness, n);
            }
        }else { (); }
        return NONPRIME;
    }
    crate::src::zsub::zsub_unsigned(crate::src::zptest::libzahl_tmp_ptest_n1.as_mut_ptr(), n, crate::src::zptest::libzahl_const_1.as_mut_ptr());
    crate::src::zsub::zsub_unsigned(crate::src::zptest::libzahl_tmp_ptest_n4.as_mut_ptr(), n, crate::src::zptest::libzahl_const_4.as_mut_ptr());
    r= crate::src::zlsb::zlsb(crate::src::zptest::libzahl_tmp_ptest_n1.as_mut_ptr());
    crate::src::zrsh::zrsh(crate::src::zptest::libzahl_tmp_ptest_d.as_mut_ptr(), crate::src::zptest::libzahl_tmp_ptest_n1.as_mut_ptr(), r);
    loop {
        let fresh0 = t;
        t= t - 1;
        if !(fresh0 != 0) {
            break;
        }
        crate::src::zrand::zrand(
            crate::src::zptest::libzahl_tmp_ptest_a.as_mut_ptr(),
            FAST_RANDOM,
            UNIFORM,
            crate::src::zptest::libzahl_tmp_ptest_n4.as_mut_ptr(),
        );
        crate::src::zadd::zadd_unsigned(
            crate::src::zptest::libzahl_tmp_ptest_a.as_mut_ptr(),
            crate::src::zptest::libzahl_tmp_ptest_a.as_mut_ptr(),
            crate::src::zptest::libzahl_const_2.as_mut_ptr(),
        );
        crate::src::zmodpow::zmodpow(
            crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(),
            crate::src::zptest::libzahl_tmp_ptest_a.as_mut_ptr(),
            crate::src::zptest::libzahl_tmp_ptest_d.as_mut_ptr(),
            n,
        );
        if crate::src::zcmp::zcmp(crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(), crate::src::zptest::libzahl_const_1.as_mut_ptr()) == 0
            || crate::src::zcmp::zcmp(crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(), crate::src::zptest::libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
        {
            continue;
        }
        i= 1 as libc::c_int as size_t;
        while i < r {
            crate::src::zsqr::zsqr(crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(), crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr());
            crate::src::zmod::zmod(crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(), crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(), n);
            if crate::src::zcmp::zcmp(crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(), crate::src::zptest::libzahl_const_1.as_mut_ptr()) == 0
            {
                if !witness.is_null() {
                    crate::src::zswap::zswap(witness.as_mut(), crate::src::zptest::libzahl_tmp_ptest_a.as_mut_ptr().as_mut());
                }else { (); }
                return NONPRIME;
            }
            if crate::src::zcmp::zcmp(crate::src::zptest::libzahl_tmp_ptest_x.as_mut_ptr(), crate::src::zptest::libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
            {
                break;
            }
            i= i.wrapping_add(1);
        }
        if i == r {
            if !witness.is_null() {
                crate::src::zswap::zswap(witness.as_mut(), crate::src::zptest::libzahl_tmp_ptest_a.as_mut_ptr().as_mut());
            }else { (); }
            return NONPRIME;
        }
    }
    return PROBABLY_PRIME;
}
