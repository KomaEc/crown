
extern "C" {
    
    
    
    
    
    
    #[no_mangle]
    static mut libzahl_temp_allocation: *mut std::os::raw::c_void;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn memmove(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    static mut libzahl_tmp_str_rem: z_t;
    #[no_mangle]
    static mut libzahl_tmp_str_num: z_t;
    #[no_mangle]
    static mut libzahl_const_1e19: z_t;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    static mut libzahl_jmp_buf: jmp_buf;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    #[no_mangle]
    static mut libzahl_temp_stack: *mut *mut zahl;
    #[no_mangle]
    static mut libzahl_error: std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn longjmp(_: *mut std::os::raw::c_int, _: std::os::raw::c_int) -> !;
}
pub use crate::src::allocator::libzahl_realloc;
pub use crate::src::zdivmod::zdivmod;
pub use crate::src::zfree::zfree;
pub use crate::src::allocator::jmp_buf;
pub use crate::src::allocator::__darwin_size_t;
pub use crate::src::allocator::size_t;
pub type uint16_t = std::os::raw::c_ushort;
pub use crate::src::allocator::uint64_t;
pub use crate::src::allocator::zahl_char_t;
// #[derive(Copy, Clone)]

pub use crate::src::allocator::zahl;
pub use crate::src::zdivmod::z_t;
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
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> std::os::raw::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zabs(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b { zset(a, b); }
    (*a).sign &= 1 as std::os::raw::c_int;
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
/* All 19 you see here is derived from that 10^19 is the largest
 * power of than < 2^64, and 64 is the number of bits in
 * zahl_char_t. If zahl_char_t is chanced, the value 19, and
 * the cast to unsigned long long must be changed accordingly. */
#[inline]
unsafe extern "C" fn sprintint_fix(mut buf: *mut std::os::raw::c_char,
                                   mut v: zahl_char_t) {
    let mut partials: *const std::os::raw::c_char =
        b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\x00"
            as *const u8 as *const std::os::raw::c_char;
    let mut buffer: *mut uint16_t =
        buf.offset(1 as std::os::raw::c_int as isize) as *mut uint16_t;
    *buffer.offset(8 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(7 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(6 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(5 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(4 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(3 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(2 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(1 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buffer.offset(0 as std::os::raw::c_int as isize) =
        *(partials.offset((2 as std::os::raw::c_int as
                               std::os::raw::c_ulonglong).wrapping_mul(v.wrapping_rem(100
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  std::os::raw::c_ulonglong))
                              as isize) as *const uint16_t);
    v =
        (v as
             std::os::raw::c_ulonglong).wrapping_div(100 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) as
            zahl_char_t as zahl_char_t;
    *buf = ('0' as i32 as std::os::raw::c_ulonglong).wrapping_add(v) as std::os::raw::c_char;
    *buf.offset(19 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
}
#[inline]
unsafe extern "C" fn cmemmove(mut d: *mut std::os::raw::c_char,
                              mut s: *const std::os::raw::c_char,
                              mut n: std::os::raw::c_long) {
    loop  {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) { break ; }
        let fresh1 = s;
        s = s.offset(1);
        let fresh2 = d;
        d = d.offset(1);
        *fresh2 = *fresh1
    };
}
#[inline]
unsafe extern "C" fn sprintint_min(mut buf: *mut std::os::raw::c_char,
                                   mut v: zahl_char_t) -> size_t {
    let mut i: std::os::raw::c_long = 0 as std::os::raw::c_int as std::os::raw::c_long;
    let mut j: std::os::raw::c_long = 0;
    sprintint_fix(buf, v);
    while *buf.offset(i as isize) as std::os::raw::c_int == '0' as i32 { i += 1 }
    j = 19 as std::os::raw::c_int as std::os::raw::c_long - i;
    cmemmove(buf, buf.offset(i as isize), j);
    *buf.offset(j as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    return j as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn zstr(mut a: *mut zahl, mut b: *mut std::os::raw::c_char,
                              mut n: size_t) -> *mut std::os::raw::c_char {
    let mut buf: [std::os::raw::c_char; 20] = [0; 20];
    let mut len: size_t = 0;
    let mut neg: size_t = 0;
    let mut last: size_t = 0;
    let mut tot: size_t = 0 as std::os::raw::c_int as size_t;
    let mut overridden: std::os::raw::c_char = 0 as std::os::raw::c_int as std::os::raw::c_char;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if b.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 &&
               {
                   b =
                       malloc(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                           *mut std::os::raw::c_char;
                   (b.is_null() as std::os::raw::c_int as std::os::raw::c_long) != 0
               } {
            libzahl_memfailure();
        }
        *b.offset(0 as std::os::raw::c_int as isize) = '0' as i32 as std::os::raw::c_char;
        *b.offset(1 as std::os::raw::c_int as isize) =
            0 as std::os::raw::c_int as std::os::raw::c_char;
        return b
    }
    if n == 0 {
        /* Calculate a value that is at least the number of
		 * digits required to store the string. The overshoot
		 * is not too signicant. */
        n =
            ((20 as std::os::raw::c_int * 64 as std::os::raw::c_int / 64 as std::os::raw::c_int +
                  (64 as std::os::raw::c_int == 8 as std::os::raw::c_int) as std::os::raw::c_int) as
                 std::os::raw::c_ulong).wrapping_mul((*a).used)
        /* Note, depends on a != as ensure above. */
    }
    if b.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 &&
           {
               libzahl_temp_allocation =
                   malloc(n.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
               b = libzahl_temp_allocation as *mut std::os::raw::c_char;
               (b.is_null() as std::os::raw::c_int as std::os::raw::c_long) != 0
           } {
        libzahl_memfailure();
    }
    neg = (zsignum(a) < 0 as std::os::raw::c_int) as std::os::raw::c_int as size_t;
    zabs(libzahl_tmp_str_num.as_mut_ptr(), a);
    *b.offset(0 as std::os::raw::c_int as isize) = '-' as i32 as std::os::raw::c_char;
    b = b.offset(neg as isize);
    n = (n as std::os::raw::c_ulong).wrapping_sub(neg) as size_t as size_t;
    last = n;
    n =
        if last > 19 as std::os::raw::c_int as std::os::raw::c_ulong {
            n.wrapping_sub(19 as std::os::raw::c_int as std::os::raw::c_ulong)
        } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong };
    loop  {
        zdivmod(libzahl_tmp_str_num.as_mut_ptr(),
                libzahl_tmp_str_rem.as_mut_ptr(),
                libzahl_tmp_str_num.as_mut_ptr(),
                libzahl_const_1e19.as_mut_ptr());
        if (zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0) as std::os::raw::c_int as
               std::os::raw::c_long != 0 {
            sprintint_fix(b.offset(n as isize),
                          if zzero(libzahl_tmp_str_rem.as_mut_ptr()) != 0 {
                              0 as std::os::raw::c_int as std::os::raw::c_ulonglong
                          } else {
                              *(*libzahl_tmp_str_rem.as_mut_ptr()).chars.offset(0
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    isize)
                          });
            *b.offset(n.wrapping_add(19 as std::os::raw::c_int as std::os::raw::c_ulong) as
                          isize) = overridden;
            overridden = *b.offset(n as isize);
            last = n;
            n =
                if last > 19 as std::os::raw::c_int as std::os::raw::c_ulong {
                    n.wrapping_sub(19 as std::os::raw::c_int as std::os::raw::c_ulong)
                } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong };
            tot =
                (tot as
                     std::os::raw::c_ulong).wrapping_add(19 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t
        } else {
            len =
                sprintint_min(buf.as_mut_ptr(),
                              *(*libzahl_tmp_str_rem.as_mut_ptr()).chars.offset(0
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    isize));
            if tot != 0 {
                memcpy(b as *mut std::os::raw::c_void,
                       buf.as_mut_ptr() as *const std::os::raw::c_void, len);
                memmove(b.offset(len as isize) as *mut std::os::raw::c_void,
                        b.offset(last as isize) as *const std::os::raw::c_void,
                        tot.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
            } else {
                memcpy(b as *mut std::os::raw::c_void,
                       buf.as_mut_ptr() as *const std::os::raw::c_void,
                       len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
            }
            break ;
        }
    }
    libzahl_temp_allocation = core::ptr::null_mut();
    return b.offset(-(neg as isize));
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

