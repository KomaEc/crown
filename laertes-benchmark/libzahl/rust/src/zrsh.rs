
extern "C" {
    #[no_mangle]
    fn libzahl_realloc(_: *mut zahl, _: size_t);
}
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_ssize_t = std::os::raw::c_long;
pub type size_t = __darwin_size_t;
pub type uint64_t = std::os::raw::c_ulonglong;
pub type ssize_t = __darwin_ssize_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: std::os::raw::c_int,
    pub padding__: std::os::raw::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
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
unsafe extern "C" fn libzahl_memmovef(mut d: *mut zahl_char_t,
                                      mut s: *const zahl_char_t,
                                      mut n: size_t) {
    if n != 0 && n < 4 as std::os::raw::c_int as std::os::raw::c_ulong {
        *d.offset(0 as std::os::raw::c_int as isize) =
            *s.offset(0 as std::os::raw::c_int as isize);
        *d.offset(1 as std::os::raw::c_int as isize) =
            *s.offset(1 as std::os::raw::c_int as isize);
        *d.offset(2 as std::os::raw::c_int as isize) =
            *s.offset(2 as std::os::raw::c_int as isize)
    } else {
        let mut i: size_t = 0;
        i = 0 as std::os::raw::c_int as size_t;
        while i < n {
            *d.offset(i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as
                          isize) =
                *s.offset(i.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as
                              isize);
            *d.offset(i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
                          isize) =
                *s.offset(i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
                              isize);
            *d.offset(i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                          isize) =
                *s.offset(i.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as
                              isize);
            *d.offset(i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as
                          isize) =
                *s.offset(i.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as
                              isize);
            i =
                (i as
                     std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t
        }
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmoveb(mut d: *mut zahl_char_t,
                                      mut s: *const zahl_char_t,
                                      mut n: size_t) {
    let mut i: ssize_t = 0;
    let mut current_block_47: u64;
    match n {
        20 => {
            *d.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((20 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 3146644675509449380;
        }
        19 => { current_block_47 = 3146644675509449380; }
        18 => { current_block_47 = 10901088805425250764; }
        17 => { current_block_47 = 5384865535390909221; }
        16 => { current_block_47 = 17566318821297549783; }
        15 => { current_block_47 = 4310035621373293786; }
        14 => { current_block_47 = 7730825046402069997; }
        13 => { current_block_47 = 7005533798885318445; }
        12 => { current_block_47 = 10587472723682454257; }
        11 => { current_block_47 = 16257047487877050539; }
        10 => { current_block_47 = 18012037057587352672; }
        9 => { current_block_47 = 12134719331697315366; }
        8 => { current_block_47 = 11900228305013741434; }
        7 => { current_block_47 = 1116179690592166573; }
        6 => { current_block_47 = 16914052422960261732; }
        5 => { current_block_47 = 10770363180794317218; }
        4 => { current_block_47 = 12684890920889971622; }
        3 => { current_block_47 = 5745611073249427146; }
        2 => { current_block_47 = 17545089610221810756; }
        1 => { current_block_47 = 11293658182915418934; }
        0 => { current_block_47 = 1423531122933789233; }
        _ => {
            i =
                n as ssize_t + 3 as std::os::raw::c_int as std::os::raw::c_long &
                    !(3 as std::os::raw::c_int) as std::os::raw::c_long;
            loop  {
                i -= 4 as std::os::raw::c_int as std::os::raw::c_long;
                if !(i >= 0 as std::os::raw::c_int as std::os::raw::c_long) { break ; }
                *d.offset((i + 3 as std::os::raw::c_int as std::os::raw::c_long) as isize) =
                    *s.offset((i + 3 as std::os::raw::c_int as std::os::raw::c_long) as
                                  isize);
                *d.offset((i + 2 as std::os::raw::c_int as std::os::raw::c_long) as isize) =
                    *s.offset((i + 2 as std::os::raw::c_int as std::os::raw::c_long) as
                                  isize);
                *d.offset((i + 1 as std::os::raw::c_int as std::os::raw::c_long) as isize) =
                    *s.offset((i + 1 as std::os::raw::c_int as std::os::raw::c_long) as
                                  isize);
                *d.offset((i + 0 as std::os::raw::c_int as std::os::raw::c_long) as isize) =
                    *s.offset((i + 0 as std::os::raw::c_int as std::os::raw::c_long) as isize)
            }
            current_block_47 = 1423531122933789233;
        }
    }
    match current_block_47 {
        3146644675509449380 => {
            *d.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((19 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 10901088805425250764;
        }
        _ => { }
    }
    match current_block_47 {
        10901088805425250764 => {
            *d.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((18 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 5384865535390909221;
        }
        _ => { }
    }
    match current_block_47 {
        5384865535390909221 => {
            *d.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((17 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 17566318821297549783;
        }
        _ => { }
    }
    match current_block_47 {
        17566318821297549783 => {
            *d.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((16 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 4310035621373293786;
        }
        _ => { }
    }
    match current_block_47 {
        4310035621373293786 => {
            *d.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((15 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 7730825046402069997;
        }
        _ => { }
    }
    match current_block_47 {
        7730825046402069997 => {
            *d.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((14 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 7005533798885318445;
        }
        _ => { }
    }
    match current_block_47 {
        7005533798885318445 => {
            *d.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((13 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 10587472723682454257;
        }
        _ => { }
    }
    match current_block_47 {
        10587472723682454257 => {
            *d.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((12 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 16257047487877050539;
        }
        _ => { }
    }
    match current_block_47 {
        16257047487877050539 => {
            *d.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((11 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 18012037057587352672;
        }
        _ => { }
    }
    match current_block_47 {
        18012037057587352672 => {
            *d.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((10 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 12134719331697315366;
        }
        _ => { }
    }
    match current_block_47 {
        12134719331697315366 => {
            *d.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((9 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 11900228305013741434;
        }
        _ => { }
    }
    match current_block_47 {
        11900228305013741434 => {
            *d.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((8 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 1116179690592166573;
        }
        _ => { }
    }
    match current_block_47 {
        1116179690592166573 => {
            *d.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((7 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 16914052422960261732;
        }
        _ => { }
    }
    match current_block_47 {
        16914052422960261732 => {
            *d.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((6 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 10770363180794317218;
        }
        _ => { }
    }
    match current_block_47 {
        10770363180794317218 => {
            *d.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((5 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 12684890920889971622;
        }
        _ => { }
    }
    match current_block_47 {
        12684890920889971622 => {
            *d.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((4 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 5745611073249427146;
        }
        _ => { }
    }
    match current_block_47 {
        5745611073249427146 => {
            *d.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((3 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 17545089610221810756;
        }
        _ => { }
    }
    match current_block_47 {
        17545089610221810756 => {
            *d.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((2 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize);
            current_block_47 = 11293658182915418934;
        }
        _ => { }
    }
    match current_block_47 {
        11293658182915418934 => {
            *d.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize) =
                *s.offset((1 as std::os::raw::c_int - 1 as std::os::raw::c_int) as isize)
        }
        _ => { }
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmove(mut d: *mut zahl_char_t,
                                     mut s: *const zahl_char_t,
                                     mut n: size_t) {
    if d < s as *mut zahl_char_t {
        libzahl_memmovef(d, s, n);
    } else { libzahl_memmoveb(d, s, n); };
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
unsafe extern "C" fn zbits(mut a: *mut zahl) -> size_t {
    let mut rc: size_t = 0;
    if (zzero(a) != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        return 1 as std::os::raw::c_int as size_t
    }
    while *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                                 isize) == 0 {
        (*a).used = (*a).used.wrapping_sub(1)
    }
    rc =
        (*a).used.wrapping_mul(8 as std::os::raw::c_int as
                                   std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<zahl_char_t>()
                                                                   as
                                                                   std::os::raw::c_ulong);
    rc =
        (rc as
             std::os::raw::c_ulong).wrapping_sub((*(*a).chars.offset((*a).used.wrapping_sub(1
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        std::os::raw::c_ulong)
                                                                 as
                                                                 isize)).leading_zeros()
                                             as i32 as size_t) as size_t as
            size_t;
    return rc;
}
/* See LICENSE file for copyright and license details. */
#[no_mangle]
pub unsafe extern "C" fn zrsh(mut a: *mut zahl, mut b: *mut zahl,
                              mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    if (bits == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        if a != b { zset(a, b); }
        return
    }
    chars = bits >> 6 as std::os::raw::c_int;
    if (zzero(b) != 0 || chars >= (*b).used || zbits(b) <= bits) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).sign = 0 as std::os::raw::c_int;
        return
    }
    bits = bits & (64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong;
    cbits = (64 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_sub(bits);
    if (chars != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 &&
           (a == b) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).used =
            ((*a).used as std::os::raw::c_ulong).wrapping_sub(chars) as size_t as
                size_t;
        libzahl_memmove((*a).chars, (*a).chars.offset(chars as isize),
                        (*a).used);
    } else if (a != b) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        (*a).used = (*b).used.wrapping_sub(chars);
        if (*a).alloced < (*a).used { libzahl_realloc(a, (*a).used); }
        libzahl_memcpy((*a).chars, (*b).chars.offset(chars as isize),
                       (*a).used);
    }
    if (bits != 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        /* This if statement is very important in C. */
        *(*a).chars.offset(0 as std::os::raw::c_int as isize) >>= bits;
        i = 1 as std::os::raw::c_int as size_t;
        while i < (*a).used {
            let ref mut fresh0 =
                *(*a).chars.offset(i.wrapping_sub(1 as std::os::raw::c_int as
                                                      std::os::raw::c_ulong) as
                                       isize);
            *fresh0 |= *(*a).chars.offset(i as isize) << cbits;
            *(*a).chars.offset(i as isize) >>= bits;
            i = i.wrapping_add(1)
        }
        while *(*a).chars.offset((*a).used.wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                                     isize) == 0 {
            (*a).used = (*a).used.wrapping_sub(1)
        }
    }
    (*a).sign = zsignum(b);
}
