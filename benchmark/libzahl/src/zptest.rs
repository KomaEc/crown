use ::libc;
extern "C" {
    fn zswap(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zcmp(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    fn zcmpu(_: *mut C2RustUnnamed, _: libc::c_ulonglong) -> libc::c_int;
    fn zmod(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmodpow(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zadd_unsigned(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zsub_unsigned(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zlsb(_: *mut C2RustUnnamed) -> size_t;
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_const_1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_const_2: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    fn zrand(_: *mut C2RustUnnamed, _: zranddev, _: zranddist, _: *mut C2RustUnnamed);
    static mut libzahl_const_4: z_t;
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
    pub chars: *mut zahl_char_t,
}
pub type z_t = [C2RustUnnamed; 1];
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
unsafe extern "C" fn zeven(mut a_0: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a_0).sign == 0
        || *((*a_0).chars).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_uint == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zptest(
    mut witness: *mut C2RustUnnamed,
    mut n: *mut C2RustUnnamed,
    mut t: libc::c_int,
) -> zprimality {
    let mut i: size_t = 0;
    let mut r: size_t = 0;
    if zcmpu(n, 3 as libc::c_int as libc::c_ulonglong) <= 0 as libc::c_int {
        if zcmpu(n, 1 as libc::c_int as libc::c_ulonglong) <= 0 as libc::c_int {
            if !witness.is_null() {
                if witness != n {
                    zset(witness, n);
                }
            }
            return NONPRIME;
        } else {
            return PRIME
        }
    }
    if zeven(n) != 0 {
        if !witness.is_null() {
            if witness != n {
                zset(witness, n);
            }
        }
        return NONPRIME;
    }
    zsub_unsigned(libzahl_tmp_ptest_n1.as_mut_ptr(), n, libzahl_const_1.as_mut_ptr());
    zsub_unsigned(libzahl_tmp_ptest_n4.as_mut_ptr(), n, libzahl_const_4.as_mut_ptr());
    r = zlsb(libzahl_tmp_ptest_n1.as_mut_ptr());
    zrsh(libzahl_tmp_ptest_d.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr(), r);
    loop {
        let fresh0 = t;
        t = t - 1;
        if !(fresh0 != 0) {
            break;
        }
        zrand(
            libzahl_tmp_ptest_a.as_mut_ptr(),
            FAST_RANDOM,
            UNIFORM,
            libzahl_tmp_ptest_n4.as_mut_ptr(),
        );
        zadd_unsigned(
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_const_2.as_mut_ptr(),
        );
        zmodpow(
            libzahl_tmp_ptest_x.as_mut_ptr(),
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_tmp_ptest_d.as_mut_ptr(),
            n,
        );
        if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_const_1.as_mut_ptr()) == 0
            || zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
        {
            continue;
        }
        i = 1 as libc::c_int as size_t;
        while i < r {
            zsqr(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_x.as_mut_ptr());
            zmod(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_x.as_mut_ptr(), n);
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_const_1.as_mut_ptr()) == 0
            {
                if !witness.is_null() {
                    zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
                }
                return NONPRIME;
            }
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == r {
            if !witness.is_null() {
                zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
            }
            return NONPRIME;
        }
    }
    return PROBABLY_PRIME;
}
