use ::libc;
extern "C" {
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn zdivmod(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e9: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_error: libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn zstr_length(_: *mut C2RustUnnamed, _: libc::c_ulonglong) -> size_t;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zstr(
    mut a: *mut C2RustUnnamed,
    mut b: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut n: size_t = 0;
    let mut len: size_t = 0;
    let mut overridden = 0 as libc::c_int as libc::c_char;
    let mut neg: libc::c_int = 0;
    if zzero(a) != 0 {
        if b.is_null() {
            b = malloc(2 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
            if b.is_null() {
                libzahl_error = *__errno_location();
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            }
        }
        *b.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        *b.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        return b;
    }
    n = zstr_length(a, 10 as libc::c_int as libc::c_ulonglong);
    if b.is_null() {
        b = malloc(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if b.is_null() {
            libzahl_error = *__errno_location();
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        }
    }
    neg = (zsignum(a) < 0 as libc::c_int) as libc::c_int;
    zabs(libzahl_tmp_str_num.as_mut_ptr(), a);
    *b.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    b = b.offset(neg as isize);
    n = (n as libc::c_ulong).wrapping_sub(neg as libc::c_ulong) as size_t as size_t;
    n = if n > 9 as libc::c_int as libc::c_ulong {
        n.wrapping_sub(9 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    loop {
        zdivmod(
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_tmp_str_rem.as_mut_ptr(),
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_const_1e9.as_mut_ptr(),
        );
        if zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0 {
            sprintf(
                b.offset(n as isize),
                b"%09lu\0" as *const u8 as *const libc::c_char,
                if zzero(libzahl_tmp_str_rem.as_mut_ptr()) != 0 {
                    0 as libc::c_ulong
                } else {
                    *((*libzahl_tmp_str_rem.as_mut_ptr()).chars)
                        .offset(0 as libc::c_int as isize) as libc::c_ulong
                },
            );
            *b
                .offset(
                    n.wrapping_add(9 as libc::c_int as libc::c_ulong) as isize,
                ) = overridden;
            overridden = *b.offset(n as isize);
            n = if n > 9 as libc::c_int as libc::c_ulong {
                n.wrapping_sub(9 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
        } else {
            len = sprintf(
                buf.as_mut_ptr(),
                b"%lu\0" as *const u8 as *const libc::c_char,
                *((*libzahl_tmp_str_rem.as_mut_ptr()).chars)
                    .offset(0 as libc::c_int as isize) as libc::c_ulong,
            ) as size_t;
            if overridden != 0 {
                buf[len as usize] = *b.offset(n.wrapping_add(len) as isize);
            }
            memcpy(
                b.offset(n as isize) as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            break;
        }
    }
    return b.offset(-(neg as isize));
}
