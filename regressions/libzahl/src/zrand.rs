use ::libc;
extern "C" {
    fn abort() -> !;
    
    
    
    
    
    static mut libzahl_jmp_buf: jmp_buf;
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_error: libc::c_int;
    
    static mut libzahl_const_1: z_t;
    fn longjmp(_: *mut crate::src::allocator::__jmp_buf_tag, _: libc::c_int) -> !;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]

struct ErasedByPreprocessor39 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor40 { dummy: () }
pub type jmp_buf = [crate::src::allocator::__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor41 { dummy: () }
pub type z_t = [crate::src::allocator::C2RustUnnamed; 1];
pub type zranddev = libc::c_uint;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub type zranddist = libc::c_uint;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
pub type ssize_t = __ssize_t;
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut crate::src::allocator::C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
unsafe extern "C" fn zrand_get_random_bits(
    mut r: Option<&mut crate::src::allocator::C2RustUnnamed>,
    mut bits: size_t,
    mut fd: libc::c_int,
) {
    let mut read_total = 0 as libc::c_int as size_t;
    let mut n: size_t = 0;
    let mut chars = bits
        .wrapping_add((32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        >> 5 as libc::c_int;
    let mut read_just: ssize_t = 0;
    let mut mask = 1 as libc::c_int as zahl_char_t;
    let mut buf = 0 as *mut libc::c_char;
    if (*r.as_deref().unwrap()).alloced < chars {
        crate::src::allocator::libzahl_realloc(r.as_deref_mut(), chars);
    }
    buf= (*r.as_deref().unwrap()).chars as *mut libc::c_char;
    n= chars.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong);
    while n != 0 {
        read_just= read(fd, buf.offset(read_total as isize) as *mut libc::c_void, n);
        if read_just < 0 as libc::c_int as libc::c_long {
            crate::src::zrand::libzahl_error= *__errno_location();
            longjmp(crate::src::zrand::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        }
        read_total= (read_total as libc::c_ulong).wrapping_add(read_just as size_t)
            as size_t as size_t;
        n= (n as libc::c_ulong).wrapping_sub(read_just as size_t) as size_t as size_t;
    }
    bits= bits & (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    mask<<= bits;
    mask= (mask as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as zahl_char_t as zahl_char_t;
    *(*r.as_deref().unwrap()).chars
        .offset(chars.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) &= mask;
    n= chars;
    loop {
        let fresh1 = n;
        n= n.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        if *(*r.as_deref().unwrap()).chars.offset(n as isize) != 0 {
            (*r.as_deref_mut().unwrap()).used= n.wrapping_add(1 as libc::c_int as libc::c_ulong);
            (*r.as_deref_mut().unwrap()).sign= 1 as libc::c_int;
            return;
        }
    }
    (*r.as_deref_mut().unwrap()).sign= 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zrand(
    mut r: *mut crate::src::allocator::C2RustUnnamed,
    mut dev: zranddev,
    mut dist: zranddist,
    mut n: *mut crate::src::allocator::C2RustUnnamed,
) {
    let mut pathname = 0 as *const libc::c_char;
    let mut bits: size_t = 0;
    let mut fd: libc::c_int = 0;
    match  dev as libc::c_uint {
        0 => {
            pathname= b"/dev/urandom\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            pathname= b"/dev/random\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            abort();
        }
    }
    if zzero(n) != 0 {
        (*r).sign= 0 as libc::c_int;
        return;
    }
    fd= open(pathname, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        crate::src::zrand::libzahl_error= *__errno_location();
        longjmp(crate::src::zrand::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
    }
    match  dist as libc::c_uint {
        0 => {
            if zsignum(n) < 0 as libc::c_int {
                crate::src::zrand::libzahl_error= 33 as libc::c_int;
                longjmp(crate::src::zrand::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            }
            bits= crate::src::zbits::zbits(n.as_mut());
            zrand_get_random_bits(r.as_mut(), bits, fd);
            crate::src::zadd::zadd(r, r, crate::src::zrand::libzahl_const_1.as_mut_ptr());
            crate::src::zmul::zmul(r, r, n);
            crate::src::zrsh::zrsh(r, r, bits);
        }
        1 => {
            if zsignum(n) < 0 as libc::c_int {
                crate::src::zrand::libzahl_error= 33 as libc::c_int;
                longjmp(crate::src::zrand::libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
            }
            bits= crate::src::zbits::zbits(n.as_mut());
            loop {
                zrand_get_random_bits(r.as_mut(), bits, fd);
                if !(crate::src::zcmpmag::zcmpmag(r, n) > 0 as libc::c_int) {
                    break;
                }
            }
        }
        _ => {
            abort();
        }
    }
    close(fd);
}
