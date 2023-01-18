use ::libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsetu(_: *mut C2RustUnnamed, _: libc::c_ulonglong);
    fn zmodmul(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zmod(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmodsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_pow_d: z_t;
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmodpowu(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: libc::c_ulonglong,
    mut d: *mut C2RustUnnamed,
) {
    if c == 0 {
        if zzero(b) != 0 {
            libzahl_error = 33 as libc::c_int;
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else if zzero(d) != 0 {
            libzahl_error = 33 as libc::c_int;
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else {
            zsetu(a, 1 as libc::c_int as libc::c_ulonglong);
        }
        return;
    } else {
        if zzero(d) != 0 {
            libzahl_error = 33 as libc::c_int;
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        } else if zzero(b) != 0 {
            (*a).sign = 0 as libc::c_int;
            return;
        }
    }
    zmod(libzahl_tmp_pow_b.as_mut_ptr(), b, d);
    zset(libzahl_tmp_pow_d.as_mut_ptr(), d);
    zsetu(a, 1 as libc::c_int as libc::c_ulonglong);
    while c != 0 {
        if c & 1 as libc::c_int as libc::c_ulonglong != 0 {
            zmodmul(
                a,
                a,
                libzahl_tmp_pow_b.as_mut_ptr(),
                libzahl_tmp_pow_d.as_mut_ptr(),
            );
        }
        zmodsqr(
            libzahl_tmp_pow_b.as_mut_ptr(),
            libzahl_tmp_pow_b.as_mut_ptr(),
            libzahl_tmp_pow_d.as_mut_ptr(),
        );
        c >>= 1 as libc::c_int;
    }
}
