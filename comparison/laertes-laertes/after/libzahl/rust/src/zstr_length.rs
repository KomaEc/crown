
extern "C" {
    
    
    
    
    #[no_mangle]
    static mut libzahl_tmp_div: [zahl; 1];
    
    
    
    
    #[no_mangle]
    static mut libzahl_tmp_str_div: z_t;
    #[no_mangle]
    static mut libzahl_tmp_str_num: z_t;
    #[no_mangle]
    static mut libzahl_tmp_str_mag: z_t;
    #[no_mangle]
    static mut libzahl_jmp_buf: jmp_buf;
    #[no_mangle]
    static mut libzahl_temp_allocation: *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    #[no_mangle]
    static mut libzahl_temp_stack: *mut *mut zahl;
    #[no_mangle]
    static mut libzahl_error: std::os::raw::c_int;
    #[no_mangle]
    fn longjmp(_: *mut std::os::raw::c_int, _: std::os::raw::c_int) -> !;
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zdivmod::zdivmod;
pub use crate::src::zfree::zfree;
pub use crate::src::zsqr::zsqr_ll;
pub use crate::src::allocator::jmp_buf;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
pub use crate::src::zdivmod::z_t;
pub use crate::src::zdivmod::zerror;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;
#[inline]
unsafe extern "C" fn libzahl_memcpy(mut d: *mut zahl_char_t,
                                    mut s: *const zahl_char_t,
                                    mut n: size_t) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16314148179394187963;
        }
        19 => { current_block_42 = 16314148179394187963; }
        18 => { current_block_42 = 23778815036459645; }
        17 => { current_block_42 = 17409214555335211048; }
        16 => { current_block_42 = 15059990250786981251; }
        15 => { current_block_42 = 6620638995229345271; }
        14 => { current_block_42 = 3600730107057416047; }
        13 => { current_block_42 = 354696938566919285; }
        12 => { current_block_42 = 8395813788269618564; }
        11 => { current_block_42 = 16693777636139501188; }
        10 => { current_block_42 = 905962748010064685; }
        9 => { current_block_42 = 9361680084571800209; }
        8 => { current_block_42 = 14892601158468998162; }
        7 => { current_block_42 = 5996064384694841776; }
        6 => { current_block_42 = 2207944898923095617; }
        5 => { current_block_42 = 11359770377620528283; }
        4 => { current_block_42 = 13725184785425480707; }
        3 => { current_block_42 = 1832949796153364909; }
        2 => { current_block_42 = 15100265573273486990; }
        1 => { current_block_42 = 12150485915963817543; }
        0 => { current_block_42 = 6717214610478484138; }
        _ => {
            let mut t: zahl_char_t = 0;
            llvm_asm!("\n    shlq $$3, $3\n    addq $1, $3\n 1:\n    movq 0($2), $0\n    movq $0, 0($1)\n    movq 8($2), $0\n    movq $0, 8($1)\n    movq 16($2), $0\n    movq $0, 16($1)\n    movq 24($2), $0\n    movq $0, 24($1)\n    addq $$32, $2\n    addq $$32, $1\n    cmpq $3, $1\n    jl 1b"
                 : "=r" (t), "+r" (d), "+r" (s), "+r" (n) : : : "volatile");
            current_block_42 = 6717214610478484138;
        }
    }
    match current_block_42 {
        16314148179394187963 => {
            *d.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 23778815036459645;
        }
        _ => { }
    }
    match current_block_42 {
        23778815036459645 => {
            *d.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 17409214555335211048;
        }
        _ => { }
    }
    match current_block_42 {
        17409214555335211048 => {
            *d.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15059990250786981251;
        }
        _ => { }
    }
    match current_block_42 {
        15059990250786981251 => {
            *d.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 6620638995229345271;
        }
        _ => { }
    }
    match current_block_42 {
        6620638995229345271 => {
            *d.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 3600730107057416047;
        }
        _ => { }
    }
    match current_block_42 {
        3600730107057416047 => {
            *d.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 354696938566919285;
        }
        _ => { }
    }
    match current_block_42 {
        354696938566919285 => {
            *d.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 8395813788269618564;
        }
        _ => { }
    }
    match current_block_42 {
        8395813788269618564 => {
            *d.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16693777636139501188;
        }
        _ => { }
    }
    match current_block_42 {
        16693777636139501188 => {
            *d.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 905962748010064685;
        }
        _ => { }
    }
    match current_block_42 {
        905962748010064685 => {
            *d.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 9361680084571800209;
        }
        _ => { }
    }
    match current_block_42 {
        9361680084571800209 => {
            *d.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 14892601158468998162;
        }
        _ => { }
    }
    match current_block_42 {
        14892601158468998162 => {
            *d.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 5996064384694841776;
        }
        _ => { }
    }
    match current_block_42 {
        5996064384694841776 => {
            *d.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 2207944898923095617;
        }
        _ => { }
    }
    match current_block_42 {
        2207944898923095617 => {
            *d.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 11359770377620528283;
        }
        _ => { }
    }
    match current_block_42 {
        11359770377620528283 => {
            *d.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 13725184785425480707;
        }
        _ => { }
    }
    match current_block_42 {
        13725184785425480707 => {
            *d.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 1832949796153364909;
        }
        _ => { }
    }
    match current_block_42 {
        1832949796153364909 => {
            *d.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15100265573273486990;
        }
        _ => { }
    }
    match current_block_42 {
        15100265573273486990 => {
            *d.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 12150485915963817543;
        }
        _ => { }
    }
    match current_block_42 {
        12150485915963817543 => {
            *d.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)
        }
        _ => { }
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> std::os::raw::c_int {
    return ((*a).sign == 0) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used { libzahl_realloc(a, (*b).used); }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsetu(mut a: *mut zahl, mut b: uint64_t) {
    if (b == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int;
        return
    }
    if (*a).alloced < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        libzahl_realloc(a, 1 as std::os::raw::c_int as size_t);
    }
    (*a).sign = 1 as std::os::raw::c_int;
    *(*a).chars.offset(0 as std::os::raw::c_int as isize) = b;
    (*a).used = 1 as std::os::raw::c_int as size_t;
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl)
 -> std::os::raw::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return -((zzero(b) == 0) as std::os::raw::c_int)
    }
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 1 as std::os::raw::c_int
    }
    i = (*a).used.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    j = (*b).used.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    while i > j {
        if *(*a).chars.offset(i as isize) != 0 { return 1 as std::os::raw::c_int }
        (*a).used = (*a).used.wrapping_sub(1);
        i = i.wrapping_sub(1)
    }
    while j > i {
        if *(*b).chars.offset(j as isize) != 0 { return -(1 as std::os::raw::c_int) }
        (*b).used = (*b).used.wrapping_sub(1);
        j = j.wrapping_sub(1)
    }
    while i != 0 &&
              *(*a).chars.offset(i as isize) == *(*b).chars.offset(i as isize)
          {
        i = i.wrapping_sub(1)
    }
    return if *(*a).chars.offset(i as isize) < *(*b).chars.offset(i as isize)
              {
               -(1 as std::os::raw::c_int)
           } else {
               (*(*a).chars.offset(i as isize) >
                    *(*b).chars.offset(i as isize)) as std::os::raw::c_int
           };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> std::os::raw::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zdiv(mut a: *mut zahl, mut b: *mut zahl,
                          mut c: *mut zahl) {
    zdivmod(a, libzahl_tmp_div.as_mut_ptr(), b, c);
}
#[inline]
unsafe extern "C" fn zsqr(mut a: *mut zahl, mut b: *mut zahl) {
    if (zzero(b) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int
    } else { zsqr_ll(a, b); (*a).sign = 1 as std::os::raw::c_int };
}
unsafe extern "C" fn libzahl_failure(mut error: std::os::raw::c_int) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = core::ptr::null_mut();
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zstr_length(mut a: *mut zahl,
                                     mut radix: std::os::raw::c_ulonglong) -> size_t {
    let mut size_total: size_t = 1 as std::os::raw::c_int as size_t;
    let mut size_temp: size_t = 0;
    if (radix < 2 as std::os::raw::c_int as std::os::raw::c_ulonglong) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        libzahl_failure(-(ZERROR_INVALID_RADIX as std::os::raw::c_int));
    }
    zset(libzahl_tmp_str_num.as_mut_ptr(), a);
    while zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0 {
        zsetu(libzahl_tmp_str_mag.as_mut_ptr(), radix);
        zset(libzahl_tmp_str_div.as_mut_ptr(),
             libzahl_tmp_str_mag.as_mut_ptr());
        size_temp = 1 as std::os::raw::c_int as size_t;
        while zcmpmag(libzahl_tmp_str_mag.as_mut_ptr(),
                      libzahl_tmp_str_num.as_mut_ptr()) <= 0 as std::os::raw::c_int {
            zset(libzahl_tmp_str_div.as_mut_ptr(),
                 libzahl_tmp_str_mag.as_mut_ptr());
            zsqr(libzahl_tmp_str_mag.as_mut_ptr(),
                 libzahl_tmp_str_mag.as_mut_ptr());
            size_temp <<= 1 as std::os::raw::c_int
        }
        size_temp >>= 1 as std::os::raw::c_int;
        size_total =
            (size_total as std::os::raw::c_ulong).wrapping_add(size_temp) as size_t as
                size_t;
        zdiv(libzahl_tmp_str_num.as_mut_ptr(),
             libzahl_tmp_str_num.as_mut_ptr(),
             libzahl_tmp_str_div.as_mut_ptr());
    }
    return size_total.wrapping_add((zsignum(a) < 0 as std::os::raw::c_int) as
                                       std::os::raw::c_int as std::os::raw::c_ulong);
}
pub fn borrow<'a, 'b: 'a, T>(p: &'a Option<&'b mut T>) -> Option<&'a T> {
    p.as_ref().map(|x| &**x)
}

pub fn borrow_mut<'a, 'b : 'a, T>(p: &'a mut Option<&'b mut T>) -> Option<&'a mut T> {
    p.as_mut().map(|x| &mut **x)
}

pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
    p.as_ref().map(|x| x.as_ref())
}

pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
    p.as_mut().map(|x| x.as_mut())
}

pub fn option_to_raw<T>(p: Option<&T>) -> * const T {
    p.map_or(core::ptr::null(), |p| p as * const T)
}

pub fn _ref_eq<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) == option_to_raw(q)
}

pub fn _ref_ne<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) != option_to_raw(q)
}
