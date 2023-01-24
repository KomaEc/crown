use ::libc;
extern "C" {
    
    
    
    
    
    static mut libzahl_tmp_pow_d: z_t;
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    fn longjmp(_: *mut crate::src::allocator::__jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]

struct ErasedByPreprocessor24 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor25 { dummy: () }
pub type jmp_buf = [crate::src::allocator::__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor26 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmodpowu(
    mut a: *mut crate::src::allocator::C2RustUnnamed,
    mut b: *mut crate::src::allocator::C2RustUnnamed,
    mut c: libc::c_ulonglong,
    mut d: *mut crate::src::allocator::C2RustUnnamed,
) {
    if c == 0 {
        if zzero(b) != 0 {
            crate::src::zmodpowu::libzahl_error= 33 as libc::c_int;
            longjmp(crate::src::zmodpowu::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else if zzero(d) != 0 {
            crate::src::zmodpowu::libzahl_error= 33 as libc::c_int;
            longjmp(crate::src::zmodpowu::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else {
            crate::src::zsetu::zsetu(a.as_mut(), 1 as libc::c_int as libc::c_ulonglong);
        }
        return;
    } else {
        if zzero(d) != 0 {
            crate::src::zmodpowu::libzahl_error= 33 as libc::c_int;
            longjmp(crate::src::zmodpowu::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else if zzero(b) != 0 {
            (*a).sign= 0 as libc::c_int;
            return;
        }
    }
    crate::src::zmod::zmod(crate::src::zmodpowu::libzahl_tmp_pow_b.as_mut_ptr(), b, d);
    crate::src::zset::zset(crate::src::zmodpowu::libzahl_tmp_pow_d.as_mut_ptr(), d);
    crate::src::zsetu::zsetu(a.as_mut(), 1 as libc::c_int as libc::c_ulonglong);
    while c != 0 {
        if c & 1 as libc::c_int as libc::c_ulonglong != 0 {
            crate::src::zmodmul::zmodmul(
                a,
                a,
                crate::src::zmodpowu::libzahl_tmp_pow_b.as_mut_ptr(),
                crate::src::zmodpowu::libzahl_tmp_pow_d.as_mut_ptr(),
            );
        }
        crate::src::zmodsqr::zmodsqr(
            crate::src::zmodpowu::libzahl_tmp_pow_b.as_mut_ptr(),
            crate::src::zmodpowu::libzahl_tmp_pow_b.as_mut_ptr(),
            crate::src::zmodpowu::libzahl_tmp_pow_d.as_mut_ptr(),
        );
        c>>= 1 as libc::c_int;
    }
}
