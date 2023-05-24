
extern "C" {
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
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
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    static mut libzahl_pool_n: [size_t; 64];
    #[no_mangle]
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    #[no_mangle]
    fn longjmp(_: *mut std::os::raw::c_int, _: std::os::raw::c_int) -> !;
}
pub use crate::src::zfree::zfree;
pub type jmp_buf = [std::os::raw::c_int; 37];
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type size_t = std::os::raw::c_ulong;
pub type uint64_t = std::os::raw::c_ulonglong;
pub type zahl_char_t = std::os::raw::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: std::os::raw::c_int,
    pub padding__: std::os::raw::c_int,
    pub used: std::os::raw::c_ulong,
    pub alloced: std::os::raw::c_ulong,
    pub chars: * mut std::os::raw::c_ulonglong,
}
impl std::default::Default for zahl {
    fn default() -> Self {
        zahl {
        sign: std::os::raw::c_int::default(),
        padding__: std::os::raw::c_int::default(),
        used: std::os::raw::c_ulong::default(),
        alloced: std::os::raw::c_ulong::default(),
        chars: 0 as * mut std::os::raw::c_ulonglong
        }
    }
}

#[inline]
unsafe extern "C" fn libzahl_memfailure() {
    if *__error() == 0 { *__error() = 2 as std::os::raw::c_int }
    libzahl_failure(*__error());
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
#[inline]
unsafe extern "C" fn libzahl_memcpy(mut d: *mut zahl_char_t,
                                    mut s: *const zahl_char_t,
                                    mut n: size_t) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16534902330276545748;
        }
        19 => { current_block_42 = 16534902330276545748; }
        18 => { current_block_42 = 9180984893592688305; }
        17 => { current_block_42 = 6903910778835967950; }
        16 => { current_block_42 = 16586880076602670907; }
        15 => { current_block_42 = 17876073040826098961; }
        14 => { current_block_42 = 6019796990488165718; }
        13 => { current_block_42 = 4505829560289365091; }
        12 => { current_block_42 = 373777254337878974; }
        11 => { current_block_42 = 13769184057628848846; }
        10 => { current_block_42 = 17709375469860046412; }
        9 => { current_block_42 = 2453145894527220284; }
        8 => { current_block_42 = 8892552165470518290; }
        7 => { current_block_42 = 16627166552877475723; }
        6 => { current_block_42 = 15307301490765489955; }
        5 => { current_block_42 = 13434239986635690899; }
        4 => { current_block_42 = 12424771829975051849; }
        3 => { current_block_42 = 15508285826572148220; }
        2 => { current_block_42 = 8602864476982803846; }
        1 => { current_block_42 = 1546934444607533929; }
        0 => { current_block_42 = 6717214610478484138; }
        _ => {
            let mut t: zahl_char_t = 0;
            llvm_asm!("\n    shlq $$3, $3\n    addq $1, $3\n 1:\n    movq 0($2), $0\n    movq $0, 0($1)\n    movq 8($2), $0\n    movq $0, 8($1)\n    movq 16($2), $0\n    movq $0, 16($1)\n    movq 24($2), $0\n    movq $0, 24($1)\n    addq $$32, $2\n    addq $$32, $1\n    cmpq $3, $1\n    jl 1b"
                 : "=r" (t), "+r" (d), "+r" (s), "+r" (n) : : : "volatile");
            current_block_42 = 6717214610478484138;
        }
    }
    match current_block_42 {
        16534902330276545748 => {
            *d.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 9180984893592688305;
        }
        _ => { }
    }
    match current_block_42 {
        9180984893592688305 => {
            *d.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 6903910778835967950;
        }
        _ => { }
    }
    match current_block_42 {
        6903910778835967950 => {
            *d.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16586880076602670907;
        }
        _ => { }
    }
    match current_block_42 {
        16586880076602670907 => {
            *d.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 17876073040826098961;
        }
        _ => { }
    }
    match current_block_42 {
        17876073040826098961 => {
            *d.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 6019796990488165718;
        }
        _ => { }
    }
    match current_block_42 {
        6019796990488165718 => {
            *d.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 4505829560289365091;
        }
        _ => { }
    }
    match current_block_42 {
        4505829560289365091 => {
            *d.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 373777254337878974;
        }
        _ => { }
    }
    match current_block_42 {
        373777254337878974 => {
            *d.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 13769184057628848846;
        }
        _ => { }
    }
    match current_block_42 {
        13769184057628848846 => {
            *d.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 17709375469860046412;
        }
        _ => { }
    }
    match current_block_42 {
        17709375469860046412 => {
            *d.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 2453145894527220284;
        }
        _ => { }
    }
    match current_block_42 {
        2453145894527220284 => {
            *d.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 8892552165470518290;
        }
        _ => { }
    }
    match current_block_42 {
        8892552165470518290 => {
            *d.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 16627166552877475723;
        }
        _ => { }
    }
    match current_block_42 {
        16627166552877475723 => {
            *d.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15307301490765489955;
        }
        _ => { }
    }
    match current_block_42 {
        15307301490765489955 => {
            *d.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 13434239986635690899;
        }
        _ => { }
    }
    match current_block_42 {
        13434239986635690899 => {
            *d.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 12424771829975051849;
        }
        _ => { }
    }
    match current_block_42 {
        12424771829975051849 => {
            *d.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 15508285826572148220;
        }
        _ => { }
    }
    match current_block_42 {
        15508285826572148220 => {
            *d.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 8602864476982803846;
        }
        _ => { }
    }
    match current_block_42 {
        8602864476982803846 => {
            *d.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_42 = 1546934444607533929;
        }
        _ => { }
    }
    match current_block_42 {
        1546934444607533929 => {
            *d.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)
        }
        _ => { }
    };
}
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn libzahl_realloc(mut a: *mut zahl, mut need: size_t) {
    let mut i: size_t = 0;
    let mut new_size: size_t = 1 as std::os::raw::c_int as size_t;
    let mut new: *mut zahl_char_t = 0 as *mut zahl_char_t;
    i =
        (8 as std::os::raw::c_int as
             std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_ulong>()
                                             as
                                             std::os::raw::c_ulong).wrapping_sub(1 as
                                                                             std::os::raw::c_int
                                                                             as
                                                                             std::os::raw::c_ulong).wrapping_sub(need.leading_zeros()
                                                                                                             as
                                                                                                             i32
                                                                                                             as
                                                                                                             size_t);
    new_size <<= i;
    if (new_size != need) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        i =
            (i as
                 std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        new_size <<= 1 as std::os::raw::c_int
    }
    if (libzahl_pool_n[i as usize] != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        libzahl_pool_n[i as usize] =
            libzahl_pool_n[i as usize].wrapping_sub(1);
        new =
            *libzahl_pool[i as
                              usize].offset(libzahl_pool_n[i as usize] as
                                                isize);
        libzahl_memcpy(new, (*a).chars, (*a).alloced);
        zfree(a);
        (*a).chars = new
    } else {
        (*a).chars =
            realloc((*a).chars as *mut std::os::raw::c_void,
                    new_size.wrapping_add(4 as std::os::raw::c_int as
                                              std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                              as
                                                                              std::os::raw::c_ulong))
                as *mut zahl_char_t;
        if (*a).chars.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
            libzahl_memfailure();
        }
    }
    (*a).alloced = new_size;
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

