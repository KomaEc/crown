use ::libc;
extern "C" {
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn zfree(_: *mut C2RustUnnamed);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut libzahl_pool_n: [size_t; 64];
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    static mut libzahl_error: libc::c_int;
    static mut libzahl_jmp_buf: jmp_buf;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn libzahl_realloc(mut a: *mut C2RustUnnamed, mut need: size_t) {
    let mut i: size_t = 0;
    let mut x: size_t = 0;
    let mut new = 0 as *mut zahl_char_t;
    if need & (!need).wrapping_add(1 as libc::c_int as libc::c_ulong) != need {
        need |= need >> 1 as libc::c_int;
        need |= need >> 2 as libc::c_int;
        need |= need >> 4 as libc::c_int;
        i = ::std::mem::size_of::<size_t>() as libc::c_ulong;
        x = 8 as libc::c_int as size_t;
        while i != 0 {
            need |= need >> x;
            i >>= 1 as libc::c_int;
            x <<= 1 as libc::c_int;
        }
        need = (need as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    i = 0 as libc::c_int as size_t;
    x = need;
    while x != 0 {
        i = (i as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        x >>= 1 as libc::c_int;
    }
    if libzahl_pool_n[i as usize] != 0 {
        libzahl_pool_n[i as usize] = (libzahl_pool_n[i as usize]).wrapping_sub(1);
        new = *(libzahl_pool[i as usize]).offset(libzahl_pool_n[i as usize] as isize);
        memcpy(
            new as *mut libc::c_void,
            (*a).chars as *const libc::c_void,
            ((*a).alloced)
                .wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        );
        zfree(a);
        let ref mut fresh0 = (*a).chars;
        *fresh0 = new;
    } else {
        let ref mut fresh1 = (*a).chars;
        *fresh1 = realloc(
            (*a).chars as *mut libc::c_void,
            need.wrapping_mul(::std::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        ) as *mut zahl_char_t;
        if ((*a).chars).is_null() {
            if *__errno_location() == 0 {
                *__errno_location() = 12 as libc::c_int;
            }
            libzahl_error = *__errno_location();
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
        }
    }
    (*a).alloced = need;
}
