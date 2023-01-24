use ::libc;
extern "C" {
    
    
    
    
    
    static mut libzahl_tmp_pow_d: z_t;
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_pow_c: z_t;
    
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    fn longjmp(_: *mut crate::src::allocator::__jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]

struct ErasedByPreprocessor21 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor22 { dummy: () }
pub type jmp_buf = [crate::src::allocator::__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor23 { dummy: () }
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
pub unsafe extern "C" fn zmodpow(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: *mut crate::src::allocator::C2RustUnnamed,
    mut d: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = 0;
    let mut bits: size_t = 0;
    let mut x: zahl_char_t = 0;
    if zsignum(c) <= 0 as libc::c_int {
        if zzero(c) != 0 {
            if zzero(b) != 0 {
                crate::src::zmodpow::libzahl_error= 33 as libc::c_int;
                longjmp(crate::src::zmodpow::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            } else if zzero(d) != 0 {
                crate::src::zmodpow::libzahl_error= 33 as libc::c_int;
                longjmp(crate::src::zmodpow::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            }
            crate::src::zsetu::zsetu(a.as_mut(), 1 as libc::c_int as libc::c_ulonglong);
        } else if zzero(b) != 0 || zzero(d) != 0 {
            crate::src::zmodpow::libzahl_error= 33 as libc::c_int;
            longjmp(crate::src::zmodpow::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else {
            (*a).sign= 0 as libc::c_int;
        }
        return;
    } else {
        if zzero(d) != 0 {
            crate::src::zmodpow::libzahl_error= 33 as libc::c_int;
            longjmp(crate::src::zmodpow::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else if zzero(b) != 0 {
            (*a).sign= 0 as libc::c_int;
            return;
        }
    }
    bits= crate::src::zbits::zbits(c.as_mut());
    n= bits >> 5 as libc::c_int;
    crate::src::zmod::zmod(crate::src::zmodpow::libzahl_tmp_pow_b.as_mut_ptr(), b, d);
    crate::src::zset::zset(crate::src::zmodpow::libzahl_tmp_pow_c.as_mut_ptr(), c);
    crate::src::zset::zset(crate::src::zmodpow::libzahl_tmp_pow_d.as_mut_ptr(), d);
    crate::src::zsetu::zsetu(a.as_mut(), 1 as libc::c_int as libc::c_ulonglong);
    i= 0 as libc::c_int as size_t;
    while i < n {
        x= *((*crate::src::zmodpow::libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
        j= 32 as libc::c_int as size_t;
        loop {
            let fresh0 = j;
            j= j.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            if x & 1 as libc::c_int as libc::c_uint != 0 {
                crate::src::zmodmul::zmodmul(
                    a,
                    a,
                    crate::src::zmodpow::libzahl_tmp_pow_b.as_mut_ptr(),
                    crate::src::zmodpow::libzahl_tmp_pow_d.as_mut_ptr(),
                );
            }
            crate::src::zmodsqr::zmodsqr(
                crate::src::zmodpow::libzahl_tmp_pow_b.as_mut_ptr(),
                crate::src::zmodpow::libzahl_tmp_pow_b.as_mut_ptr(),
                crate::src::zmodpow::libzahl_tmp_pow_d.as_mut_ptr(),
            );
            x>>= 1 as libc::c_int;
        }
        i= i.wrapping_add(1);
    }
    x= *((*crate::src::zmodpow::libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
    while x != 0 {
        if x & 1 as libc::c_int as libc::c_uint != 0 {
            crate::src::zmodmul::zmodmul(
                a,
                a,
                crate::src::zmodpow::libzahl_tmp_pow_b.as_mut_ptr(),
                crate::src::zmodpow::libzahl_tmp_pow_d.as_mut_ptr(),
            );
        }
        crate::src::zmodsqr::zmodsqr(
            crate::src::zmodpow::libzahl_tmp_pow_b.as_mut_ptr(),
            crate::src::zmodpow::libzahl_tmp_pow_b.as_mut_ptr(),
            crate::src::zmodpow::libzahl_tmp_pow_d.as_mut_ptr(),
        );
        x>>= 1 as libc::c_int;
    }
}
