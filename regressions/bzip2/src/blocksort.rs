use ::libc;
extern "C" {

    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *const libc::c_char,
    pub _IO_read_end: *const libc::c_char,
    pub _IO_read_base: *const libc::c_char,
    pub _IO_write_base: *const libc::c_char,
    pub _IO_write_ptr: *const libc::c_char,
    pub _IO_write_end: *const libc::c_char,
    pub _IO_buf_base: *const libc::c_char,
    pub _IO_buf_end: *const libc::c_char,
    pub _IO_save_base: *const libc::c_char,
    pub _IO_backup_base: *const libc::c_char,
    pub _IO_save_end: *const libc::c_char,
    pub _markers: *const crate::src::bzlib::_IO_marker,
    pub _chain: *const _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *const libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *const crate::src::bzip2recover::_IO_codecvt,
    pub _wide_data: *const crate::src::compress::_IO_wide_data,
    pub _freeres_list: *const _IO_FILE,
    pub _freeres_buf: *const libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
impl Default for _IO_FILE {
    fn default() -> Self {
        Self {
            _flags: Default::default(),
            _IO_read_ptr: std::ptr::null_mut(),
            _IO_read_end: std::ptr::null_mut(),
            _IO_read_base: std::ptr::null_mut(),
            _IO_write_base: std::ptr::null_mut(),
            _IO_write_ptr: std::ptr::null_mut(),
            _IO_write_end: std::ptr::null_mut(),
            _IO_buf_base: std::ptr::null_mut(),
            _IO_buf_end: std::ptr::null_mut(),
            _IO_save_base: std::ptr::null_mut(),
            _IO_backup_base: std::ptr::null_mut(),
            _IO_save_end: std::ptr::null_mut(),
            _markers: std::ptr::null_mut(),
            _chain: std::ptr::null_mut(),
            _fileno: Default::default(),
            _flags2: Default::default(),
            _old_offset: Default::default(),
            _cur_column: Default::default(),
            _vtable_offset: Default::default(),
            _shortbuf: Default::default(),
            _lock: std::ptr::null_mut(),
            _offset: Default::default(),
            _codecvt: std::ptr::null_mut(),
            _wide_data: std::ptr::null_mut(),
            _freeres_list: std::ptr::null_mut(),
            _freeres_buf: std::ptr::null_mut(),
            __pad5: Default::default(),
            _mode: Default::default(),
            _unused2: Default::default(),
        }
    }
}

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *const libc::c_char,
    pub avail_in: libc::c_uint,
    pub total_in_lo32: libc::c_uint,
    pub total_in_hi32: libc::c_uint,
    pub next_out: *mut libc::c_char,
    pub avail_out: libc::c_uint,
    pub total_out_lo32: libc::c_uint,
    pub total_out_hi32: libc::c_uint,
    pub state: *mut libc::c_void,
    pub bzalloc: Option<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> *mut libc::c_void,
    >,
    pub bzfree: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub opaque: *mut libc::c_void,
}
impl Default for bz_stream {
    fn default() -> Self {
        Self {
            next_in: std::ptr::null_mut(),
            avail_in: Default::default(),
            total_in_lo32: Default::default(),
            total_in_hi32: Default::default(),
            next_out: std::ptr::null_mut(),
            avail_out: Default::default(),
            total_out_lo32: Default::default(),
            total_out_hi32: Default::default(),
            state: std::ptr::null_mut(),
            bzalloc: Default::default(),
            bzfree: Default::default(),
            opaque: std::ptr::null_mut(),
        }
    }
}

pub type Bool = libc::c_uchar;
pub type UChar = libc::c_uchar;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub strm: *mut bz_stream,
    pub mode: Int32,
    pub state: Int32,
    pub avail_in_expect: UInt32,
    pub arr1: *mut UInt32,
    pub arr2: *mut UInt32,
    pub ftab: *mut UInt32,
    pub origPtr: Int32,
    pub ptr: *mut UInt32,
    pub block: *mut UChar,
    pub mtfv: *mut UInt16,
    pub zbits: *mut UChar,
    pub workFactor: Int32,
    pub state_in_ch: UInt32,
    pub state_in_len: Int32,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub nblock: Int32,
    pub nblockMAX: Int32,
    pub numZ: Int32,
    pub state_out_pos: Int32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub unseqToSeq: [UChar; 256],
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockCRC: UInt32,
    pub combinedCRC: UInt32,
    pub verbosity: Int32,
    pub blockNo: Int32,
    pub blockSize100k: Int32,
    pub nMTF: Int32,
    pub mtfFreq: [Int32; 258],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub code: [[Int32; 258]; 6],
    pub rfreq: [[Int32; 258]; 6],
    pub len_pack: [[UInt32; 4]; 258],
}
impl Default for EState {
    fn default() -> Self {
        Self {
            strm: std::ptr::null_mut(),
            mode: Default::default(),
            state: Default::default(),
            avail_in_expect: Default::default(),
            arr1: std::ptr::null_mut(),
            arr2: std::ptr::null_mut(),
            ftab: std::ptr::null_mut(),
            origPtr: Default::default(),
            ptr: std::ptr::null_mut(),
            block: std::ptr::null_mut(),
            mtfv: std::ptr::null_mut(),
            zbits: std::ptr::null_mut(),
            workFactor: Default::default(),
            state_in_ch: Default::default(),
            state_in_len: Default::default(),
            rNToGo: Default::default(),
            rTPos: Default::default(),
            nblock: Default::default(),
            nblockMAX: Default::default(),
            numZ: Default::default(),
            state_out_pos: Default::default(),
            nInUse: Default::default(),
            inUse: Default::default(),
            unseqToSeq: Default::default(),
            bsBuff: Default::default(),
            bsLive: Default::default(),
            blockCRC: Default::default(),
            combinedCRC: Default::default(),
            verbosity: Default::default(),
            blockNo: Default::default(),
            blockSize100k: Default::default(),
            nMTF: Default::default(),
            mtfFreq: Default::default(),
            selector: Default::default(),
            selectorMtf: Default::default(),
            len: Default::default(),
            code: Default::default(),
            rfreq: Default::default(),
            len_pack: Default::default(),
        }
    }
}

#[inline]
unsafe extern "C" fn fallbackSimpleSort(
    mut fmap: *mut UInt32,
    mut eclass: *const UInt32,
    mut lo: Int32,
    mut hi: Int32,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: Int32 = 0;
    let mut ec_tmp: UInt32 = 0;
    if lo == hi {
        return;
    }
    if hi - lo > 3 as libc::c_int {
        i = hi - 4 as libc::c_int;
        while i >= lo {
            tmp = *fmap.offset(i as isize) as Int32;
            ec_tmp = *eclass.offset(tmp as isize);
            j = i + 4 as libc::c_int;
            while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j - 4 as libc::c_int) as isize) = *fmap.offset(j as isize);
                j += 4 as libc::c_int;
            }
            *fmap.offset((j - 4 as libc::c_int) as isize) = tmp as UInt32;
            i -= 1;
        }
    }
    i = hi - 1 as libc::c_int;
    while i >= lo {
        tmp = *fmap.offset(i as isize) as Int32;
        ec_tmp = *eclass.offset(tmp as isize);
        j = i + 1 as libc::c_int;
        while j <= hi && ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
            *fmap.offset((j - 1 as libc::c_int) as isize) = *fmap.offset(j as isize);
            j += 1;
        }
        *fmap.offset((j - 1 as libc::c_int) as isize) = tmp as UInt32;
        i -= 1;
    }
}
unsafe extern "C" fn fallbackQSort3(
    mut fmap: *mut UInt32,
    mut eclass: *const UInt32,
    mut loSt: Int32,
    mut hiSt: Int32,
) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut med: UInt32 = 0;
    let mut r: UInt32 = 0;
    let mut r3: UInt32 = 0;
    let mut stackLo: [Int32; 100] = [0; 100];
    let mut stackHi: [Int32; 100] = [0; 100];
    r = 0 as libc::c_int as UInt32;
    sp = 0 as libc::c_int;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    sp += 1;
    while sp > 0 as libc::c_int {
        if !(sp < 100 as libc::c_int - 1 as libc::c_int) {
            crate::src::bzlib::BZ2_bz__AssertH__fail(1004 as libc::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        if hi - lo < 10 as libc::c_int {
            fallbackSimpleSort(fmap, eclass, lo, hi);
        } else {
            r = r
                .wrapping_mul(7621 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_rem(32768 as libc::c_int as libc::c_uint);
            r3 = r.wrapping_rem(3 as libc::c_int as libc::c_uint);
            if r3 == 0 as libc::c_int as libc::c_uint {
                med = *eclass.offset(*fmap.offset(lo as isize) as isize);
            } else if r3 == 1 as libc::c_int as libc::c_uint {
                med = *eclass.offset(*fmap.offset((lo + hi >> 1 as libc::c_int) as isize) as isize);
            } else {
                med = *eclass.offset(*fmap.offset(hi as isize) as isize);
            }
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            loop {
                while !(unLo > unHi) {
                    n = *eclass.offset(*fmap.offset(unLo as isize) as isize) as Int32
                        - med as Int32;
                    if n == 0 as libc::c_int {
                        let mut zztmp = *fmap.offset(unLo as isize) as Int32;
                        *fmap.offset(unLo as isize) = *fmap.offset(ltLo as isize);
                        *fmap.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1;
                    } else {
                        if n > 0 as libc::c_int {
                            break;
                        }
                        unLo += 1;
                    }
                }
                while !(unLo > unHi) {
                    n = *eclass.offset(*fmap.offset(unHi as isize) as isize) as Int32
                        - med as Int32;
                    if n == 0 as libc::c_int {
                        let mut zztmp_0 = *fmap.offset(unHi as isize) as Int32;
                        *fmap.offset(unHi as isize) = *fmap.offset(gtHi as isize);
                        *fmap.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1;
                    } else {
                        if n < 0 as libc::c_int {
                            break;
                        }
                        unHi -= 1;
                    }
                }
                if unLo > unHi {
                    break;
                }
                let mut zztmp_1 = *fmap.offset(unLo as isize) as Int32;
                *fmap.offset(unLo as isize) = *fmap.offset(unHi as isize);
                *fmap.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1;
            }
            if gtHi < ltLo {
                continue;
            }
            n = if ltLo - lo < unLo - ltLo {
                ltLo - lo
            } else {
                unLo - ltLo
            };
            let mut yyp1 = lo;
            let mut yyp2 = unLo - n;
            let mut yyn = n;
            while yyn > 0 as libc::c_int {
                let mut zztmp_2 = *fmap.offset(yyp1 as isize) as Int32;
                *fmap.offset(yyp1 as isize) = *fmap.offset(yyp2 as isize);
                *fmap.offset(yyp2 as isize) = zztmp_2 as UInt32;
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1;
            }
            m = if hi - gtHi < gtHi - unHi {
                hi - gtHi
            } else {
                gtHi - unHi
            };
            let mut yyp1_0 = unLo;
            let mut yyp2_0 = hi - m + 1 as libc::c_int;
            let mut yyn_0 = m;
            while yyn_0 > 0 as libc::c_int {
                let mut zztmp_3 = *fmap.offset(yyp1_0 as isize) as Int32;
                *fmap.offset(yyp1_0 as isize) = *fmap.offset(yyp2_0 as isize);
                *fmap.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                yyp1_0 += 1;
                yyp2_0 += 1;
                yyn_0 -= 1;
            }
            n = lo + unLo - ltLo - 1 as libc::c_int;
            m = hi - (gtHi - unHi) + 1 as libc::c_int;
            if n - lo > hi - m {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1;
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1;
            } else {
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1;
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1;
            }
        }
    }
}
unsafe extern "C" fn fallbackSort(
    mut fmap: *mut UInt32,
    mut eclass: *mut UInt32,
    mut bhtab: *mut UInt32,
    mut nblock: Int32,
    mut verb: Int32,
) {
    let mut ftab: [Int32; 257] = [0; 257];
    let mut ftabCopy: [Int32; 256] = [0; 256];
    let mut H: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut l: Int32 = 0;
    let mut r: Int32 = 0;
    let mut cc: Int32 = 0;
    let mut cc1: Int32 = 0;
    let mut nNotDone: Int32 = 0;
    let mut nBhtab: Int32 = 0;
    let mut eclass8 = eclass as *mut UChar;
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        bucket sorting ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < 257 as libc::c_int {
        ftab[i as usize] = 0 as libc::c_int;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nblock {
        ftab[*eclass8.offset(i as isize) as usize] += 1;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        ftabCopy[i as usize] = ftab[i as usize];
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < 257 as libc::c_int {
        ftab[i as usize] += ftab[(i - 1 as libc::c_int) as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nblock {
        j = *eclass8.offset(i as isize) as Int32;
        k = ftab[j as usize] - 1 as libc::c_int;
        ftab[j as usize] = k;
        *fmap.offset(k as isize) = i as UInt32;
        i += 1;
    }
    nBhtab = 2 as libc::c_int + nblock / 32 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nBhtab {
        *bhtab.offset(i as isize) = 0 as libc::c_int as UInt32;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        *bhtab.offset((ftab[i as usize] >> 5 as libc::c_int) as isize) =
            (1 as libc::c_int as UInt32) << (ftab[i as usize] & 31 as libc::c_int);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *bhtab.offset((nblock + 2 as libc::c_int * i >> 5 as libc::c_int) as isize) =
            (1 as libc::c_int as UInt32) << (nblock + 2 as libc::c_int * i & 31 as libc::c_int);
        *bhtab.offset(
            (nblock + 2 as libc::c_int * i + 1 as libc::c_int >> 5 as libc::c_int) as isize,
        ) &= !((1 as libc::c_int as UInt32)
            << (nblock + 2 as libc::c_int * i + 1 as libc::c_int & 31 as libc::c_int));
        i += 1;
    }
    H = 1 as libc::c_int;
    loop {
        if verb >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"        depth %6d has \0" as *const u8 as *const libc::c_char,
                H,
            );
        }
        j = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nblock {
            if *bhtab.offset((i >> 5 as libc::c_int) as isize)
                & (1 as libc::c_int as UInt32) << (i & 31 as libc::c_int)
                != 0
            {
                j = i;
            }
            k = (*fmap.offset(i as isize)).wrapping_sub(H as libc::c_uint) as Int32;
            if k < 0 as libc::c_int {
                k += nblock;
            }
            *eclass.offset(k as isize) = j as UInt32;
            i += 1;
        }
        nNotDone = 0 as libc::c_int;
        r = -(1 as libc::c_int);
        loop {
            k = r + 1 as libc::c_int;
            while *bhtab.offset((k >> 5 as libc::c_int) as isize)
                & (1 as libc::c_int as UInt32) << (k & 31 as libc::c_int)
                != 0
                && k & 0x1f as libc::c_int != 0
            {
                k += 1;
            }
            if *bhtab.offset((k >> 5 as libc::c_int) as isize)
                & (1 as libc::c_int as UInt32) << (k & 31 as libc::c_int)
                != 0
            {
                while *bhtab.offset((k >> 5 as libc::c_int) as isize) == 0xffffffff as libc::c_uint
                {
                    k += 32 as libc::c_int;
                }
                while *bhtab.offset((k >> 5 as libc::c_int) as isize)
                    & (1 as libc::c_int as UInt32) << (k & 31 as libc::c_int)
                    != 0
                {
                    k += 1;
                }
            }
            l = k - 1 as libc::c_int;
            if l >= nblock {
                break;
            }
            while *bhtab.offset((k >> 5 as libc::c_int) as isize)
                & (1 as libc::c_int as UInt32) << (k & 31 as libc::c_int)
                == 0
                && k & 0x1f as libc::c_int != 0
            {
                k += 1;
            }
            if *bhtab.offset((k >> 5 as libc::c_int) as isize)
                & (1 as libc::c_int as UInt32) << (k & 31 as libc::c_int)
                == 0
            {
                while *bhtab.offset((k >> 5 as libc::c_int) as isize)
                    == 0 as libc::c_int as libc::c_uint
                {
                    k += 32 as libc::c_int;
                }
                while *bhtab.offset((k >> 5 as libc::c_int) as isize)
                    & (1 as libc::c_int as UInt32) << (k & 31 as libc::c_int)
                    == 0
                {
                    k += 1;
                }
            }
            r = k - 1 as libc::c_int;
            if r >= nblock {
                break;
            }
            if r > l {
                nNotDone += r - l + 1 as libc::c_int;
                fallbackQSort3(fmap, eclass, l, r);
                cc = -(1 as libc::c_int);
                i = l;
                while i <= r {
                    cc1 = *eclass.offset(*fmap.offset(i as isize) as isize) as Int32;
                    if cc != cc1 {
                        *bhtab.offset((i >> 5 as libc::c_int) as isize) =
                            (1 as libc::c_int as UInt32) << (i & 31 as libc::c_int);
                        cc = cc1;
                    }
                    i += 1;
                }
            }
        }
        if verb >= 4 as libc::c_int {
            fprintf(
                stderr,
                b"%6d unresolved strings\n\0" as *const u8 as *const libc::c_char,
                nNotDone,
            );
        }
        H *= 2 as libc::c_int;
        if H > nblock || nNotDone == 0 as libc::c_int {
            break;
        }
    }
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        reconstructing block ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nblock {
        while ftabCopy[j as usize] == 0 as libc::c_int {
            j += 1;
        }
        ftabCopy[j as usize] -= 1;
        *eclass8.offset(*fmap.offset(i as isize) as isize) = j as UChar;
        i += 1;
    }
    if !(j < 256 as libc::c_int) {
        crate::src::bzlib::BZ2_bz__AssertH__fail(1005 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn mainGtU(
    mut i1: UInt32,
    mut i2: UInt32,
    mut block: *const UChar,
    mut quadrant: *const UInt16,
    mut nblock: UInt32,
    mut budget: Option<&mut Int32>,
) -> Bool {
    let mut k: Int32 = 0;
    let mut c1: UChar = 0;
    let mut c2: UChar = 0;
    let mut s1: UInt16 = 0;
    let mut s2: UInt16 = 0;
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as libc::c_int != c2 as libc::c_int {
        return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    k = nblock.wrapping_add(8 as libc::c_int as libc::c_uint) as Int32;
    loop {
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as libc::c_int != c2 as libc::c_int {
            return (c1 as libc::c_int > c2 as libc::c_int) as libc::c_int as Bool;
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as libc::c_int != s2 as libc::c_int {
            return (s1 as libc::c_int > s2 as libc::c_int) as libc::c_int as Bool;
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        if i1 >= nblock {
            i1 = (i1 as libc::c_uint).wrapping_sub(nblock) as UInt32 as UInt32;
        }
        if i2 >= nblock {
            i2 = (i2 as libc::c_uint).wrapping_sub(nblock) as UInt32 as UInt32;
        }
        k -= 8 as libc::c_int;
        *budget.as_deref_mut().unwrap() -= 1;
        if !(k >= 0 as libc::c_int) {
            break;
        }
    }
    return 0 as libc::c_int as Bool;
}
static mut incs: [Int32; 14] = [
    1 as libc::c_int,
    4 as libc::c_int,
    13 as libc::c_int,
    40 as libc::c_int,
    121 as libc::c_int,
    364 as libc::c_int,
    1093 as libc::c_int,
    3280 as libc::c_int,
    9841 as libc::c_int,
    29524 as libc::c_int,
    88573 as libc::c_int,
    265720 as libc::c_int,
    797161 as libc::c_int,
    2391484 as libc::c_int,
];
unsafe extern "C" fn mainSimpleSort(
    mut ptr: *mut UInt32,
    mut block: *const UChar,
    mut quadrant: *const UInt16,
    mut nblock: Int32,
    mut lo: Int32,
    mut hi: Int32,
    mut d: Int32,
    mut budget: Option<&mut Int32>,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut h: Int32 = 0;
    let mut bigN: Int32 = 0;
    let mut hp: Int32 = 0;
    let mut v: UInt32 = 0;
    bigN = hi - lo + 1 as libc::c_int;
    if bigN < 2 as libc::c_int {
        return;
    }
    hp = 0 as libc::c_int;
    while incs[hp as usize] < bigN {
        hp += 1;
    }
    hp -= 1;
    while hp >= 0 as libc::c_int {
        h = incs[hp as usize];
        i = lo + h;
        while 1 as libc::c_int as Bool != 0 {
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as libc::c_uint),
                v.wrapping_add(d as libc::c_uint),
                block,
                quadrant,
                nblock as UInt32,
                budget.as_deref_mut(),
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as libc::c_int {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as libc::c_uint),
                v.wrapping_add(d as libc::c_uint),
                block,
                quadrant,
                nblock as UInt32,
                budget.as_deref_mut(),
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as libc::c_int {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if i > hi {
                break;
            }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU(
                (*ptr.offset((j - h) as isize)).wrapping_add(d as libc::c_uint),
                v.wrapping_add(d as libc::c_uint),
                block,
                quadrant,
                nblock as UInt32,
                budget.as_deref_mut(),
            ) != 0
            {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as libc::c_int {
                    break;
                }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if (*budget.as_deref().unwrap()) < 0 as libc::c_int {
                return;
            }
        }
        hp -= 1;
    }
}
#[inline]
unsafe extern "C" fn mmed3(mut a: UChar, mut b: UChar, mut c: UChar) -> UChar {
    let mut t: UChar = 0;
    if a as libc::c_int > b as libc::c_int {
        t = a;
        a = b;
        b = t;
    }
    if b as libc::c_int > c as libc::c_int {
        b = c;
        if a as libc::c_int > b as libc::c_int {
            b = a;
        }
    }
    return b;
}
unsafe extern "C" fn mainQSort3(
    mut ptr: *mut UInt32,
    mut block: *const UChar,
    mut quadrant: *const UInt16,
    mut nblock: Int32,
    mut loSt: Int32,
    mut hiSt: Int32,
    mut dSt: Int32,
    mut budget: Option<&mut Int32>,
) {
    let mut unLo: Int32 = 0;
    let mut unHi: Int32 = 0;
    let mut ltLo: Int32 = 0;
    let mut gtHi: Int32 = 0;
    let mut n: Int32 = 0;
    let mut m: Int32 = 0;
    let mut med: Int32 = 0;
    let mut sp: Int32 = 0;
    let mut lo: Int32 = 0;
    let mut hi: Int32 = 0;
    let mut d: Int32 = 0;
    let mut stackLo: [Int32; 100] = [0; 100];
    let mut stackHi: [Int32; 100] = [0; 100];
    let mut stackD: [Int32; 100] = [0; 100];
    let mut nextLo: [Int32; 3] = [0; 3];
    let mut nextHi: [Int32; 3] = [0; 3];
    let mut nextD: [Int32; 3] = [0; 3];
    sp = 0 as libc::c_int;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    stackD[sp as usize] = dSt;
    sp += 1;
    while sp > 0 as libc::c_int {
        if !(sp < 100 as libc::c_int - 2 as libc::c_int) {
            crate::src::bzlib::BZ2_bz__AssertH__fail(1001 as libc::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        d = stackD[sp as usize];
        if hi - lo < 20 as libc::c_int || d > 2 as libc::c_int + 12 as libc::c_int {
            mainSimpleSort(
                ptr,
                block,
                quadrant,
                nblock,
                lo,
                hi,
                d,
                budget.as_deref_mut(),
            );
            if (*budget.as_deref().unwrap()) < 0 as libc::c_int {
                return;
            }
        } else {
            med = mmed3(
                *block.offset((*ptr.offset(lo as isize)).wrapping_add(d as libc::c_uint) as isize),
                *block.offset((*ptr.offset(hi as isize)).wrapping_add(d as libc::c_uint) as isize),
                *block.offset(
                    (*ptr.offset((lo + hi >> 1 as libc::c_int) as isize))
                        .wrapping_add(d as libc::c_uint) as isize,
                ),
            ) as Int32;
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            while 1 as libc::c_int as Bool != 0 {
                while 1 as libc::c_int as Bool != 0 {
                    if unLo > unHi {
                        break;
                    }
                    n =
                        *block
                            .offset((*ptr.offset(unLo as isize)).wrapping_add(d as libc::c_uint)
                                as isize) as Int32
                            - med;
                    if n == 0 as libc::c_int {
                        let mut zztmp = *ptr.offset(unLo as isize) as Int32;
                        *ptr.offset(unLo as isize) = *ptr.offset(ltLo as isize);
                        *ptr.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1;
                    } else {
                        if n > 0 as libc::c_int {
                            break;
                        }
                        unLo += 1;
                    }
                }
                while 1 as libc::c_int as Bool != 0 {
                    if unLo > unHi {
                        break;
                    }
                    n =
                        *block
                            .offset((*ptr.offset(unHi as isize)).wrapping_add(d as libc::c_uint)
                                as isize) as Int32
                            - med;
                    if n == 0 as libc::c_int {
                        let mut zztmp_0 = *ptr.offset(unHi as isize) as Int32;
                        *ptr.offset(unHi as isize) = *ptr.offset(gtHi as isize);
                        *ptr.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1;
                    } else {
                        if n < 0 as libc::c_int {
                            break;
                        }
                        unHi -= 1;
                    }
                }
                if unLo > unHi {
                    break;
                }
                let mut zztmp_1 = *ptr.offset(unLo as isize) as Int32;
                *ptr.offset(unLo as isize) = *ptr.offset(unHi as isize);
                *ptr.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1;
            }
            if gtHi < ltLo {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = hi;
                stackD[sp as usize] = d + 1 as libc::c_int;
                sp += 1;
            } else {
                n = if ltLo - lo < unLo - ltLo {
                    ltLo - lo
                } else {
                    unLo - ltLo
                };
                let mut yyp1 = lo;
                let mut yyp2 = unLo - n;
                let mut yyn = n;
                while yyn > 0 as libc::c_int {
                    let mut zztmp_2 = *ptr.offset(yyp1 as isize) as Int32;
                    *ptr.offset(yyp1 as isize) = *ptr.offset(yyp2 as isize);
                    *ptr.offset(yyp2 as isize) = zztmp_2 as UInt32;
                    yyp1 += 1;
                    yyp2 += 1;
                    yyn -= 1;
                }
                m = if hi - gtHi < gtHi - unHi {
                    hi - gtHi
                } else {
                    gtHi - unHi
                };
                let mut yyp1_0 = unLo;
                let mut yyp2_0 = hi - m + 1 as libc::c_int;
                let mut yyn_0 = m;
                while yyn_0 > 0 as libc::c_int {
                    let mut zztmp_3 = *ptr.offset(yyp1_0 as isize) as Int32;
                    *ptr.offset(yyp1_0 as isize) = *ptr.offset(yyp2_0 as isize);
                    *ptr.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                    yyp1_0 += 1;
                    yyp2_0 += 1;
                    yyn_0 -= 1;
                }
                n = lo + unLo - ltLo - 1 as libc::c_int;
                m = hi - (gtHi - unHi) + 1 as libc::c_int;
                nextLo[0 as libc::c_int as usize] = lo;
                nextHi[0 as libc::c_int as usize] = n;
                nextD[0 as libc::c_int as usize] = d;
                nextLo[1 as libc::c_int as usize] = m;
                nextHi[1 as libc::c_int as usize] = hi;
                nextD[1 as libc::c_int as usize] = d;
                nextLo[2 as libc::c_int as usize] = n + 1 as libc::c_int;
                nextHi[2 as libc::c_int as usize] = m - 1 as libc::c_int;
                nextD[2 as libc::c_int as usize] = d + 1 as libc::c_int;
                if nextHi[0 as libc::c_int as usize] - nextLo[0 as libc::c_int as usize]
                    < nextHi[1 as libc::c_int as usize] - nextLo[1 as libc::c_int as usize]
                {
                    let mut tz: Int32 = 0;
                    tz = nextLo[0 as libc::c_int as usize];
                    nextLo[0 as libc::c_int as usize] = nextLo[1 as libc::c_int as usize];
                    nextLo[1 as libc::c_int as usize] = tz;
                    tz = nextHi[0 as libc::c_int as usize];
                    nextHi[0 as libc::c_int as usize] = nextHi[1 as libc::c_int as usize];
                    nextHi[1 as libc::c_int as usize] = tz;
                    tz = nextD[0 as libc::c_int as usize];
                    nextD[0 as libc::c_int as usize] = nextD[1 as libc::c_int as usize];
                    nextD[1 as libc::c_int as usize] = tz;
                }
                if nextHi[1 as libc::c_int as usize] - nextLo[1 as libc::c_int as usize]
                    < nextHi[2 as libc::c_int as usize] - nextLo[2 as libc::c_int as usize]
                {
                    let mut tz_0: Int32 = 0;
                    tz_0 = nextLo[1 as libc::c_int as usize];
                    nextLo[1 as libc::c_int as usize] = nextLo[2 as libc::c_int as usize];
                    nextLo[2 as libc::c_int as usize] = tz_0;
                    tz_0 = nextHi[1 as libc::c_int as usize];
                    nextHi[1 as libc::c_int as usize] = nextHi[2 as libc::c_int as usize];
                    nextHi[2 as libc::c_int as usize] = tz_0;
                    tz_0 = nextD[1 as libc::c_int as usize];
                    nextD[1 as libc::c_int as usize] = nextD[2 as libc::c_int as usize];
                    nextD[2 as libc::c_int as usize] = tz_0;
                }
                if nextHi[0 as libc::c_int as usize] - nextLo[0 as libc::c_int as usize]
                    < nextHi[1 as libc::c_int as usize] - nextLo[1 as libc::c_int as usize]
                {
                    let mut tz_1: Int32 = 0;
                    tz_1 = nextLo[0 as libc::c_int as usize];
                    nextLo[0 as libc::c_int as usize] = nextLo[1 as libc::c_int as usize];
                    nextLo[1 as libc::c_int as usize] = tz_1;
                    tz_1 = nextHi[0 as libc::c_int as usize];
                    nextHi[0 as libc::c_int as usize] = nextHi[1 as libc::c_int as usize];
                    nextHi[1 as libc::c_int as usize] = tz_1;
                    tz_1 = nextD[0 as libc::c_int as usize];
                    nextD[0 as libc::c_int as usize] = nextD[1 as libc::c_int as usize];
                    nextD[1 as libc::c_int as usize] = tz_1;
                }
                stackLo[sp as usize] = nextLo[0 as libc::c_int as usize];
                stackHi[sp as usize] = nextHi[0 as libc::c_int as usize];
                stackD[sp as usize] = nextD[0 as libc::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[1 as libc::c_int as usize];
                stackHi[sp as usize] = nextHi[1 as libc::c_int as usize];
                stackD[sp as usize] = nextD[1 as libc::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[2 as libc::c_int as usize];
                stackHi[sp as usize] = nextHi[2 as libc::c_int as usize];
                stackD[sp as usize] = nextD[2 as libc::c_int as usize];
                sp += 1;
            }
        }
    }
}
unsafe extern "C" fn mainSort(
    mut ptr: *mut UInt32,
    mut block: *mut UChar,
    mut quadrant: *mut UInt16,
    mut ftab: *mut UInt32,
    mut nblock: Int32,
    mut verb: Int32,
    mut budget: Option<&mut Int32>,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut ss: Int32 = 0;
    let mut sb: Int32 = 0;
    let mut runningOrder: [Int32; 256] = [0; 256];
    let mut bigDone: [Bool; 256] = [0; 256];
    let mut copyStart: [Int32; 256] = [0; 256];
    let mut copyEnd: [Int32; 256] = [0; 256];
    let mut c1: UChar = 0;
    let mut numQSorted: Int32 = 0;
    let mut s: UInt16 = 0;
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        main sort initialise ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 65536 as libc::c_int;
    while i >= 0 as libc::c_int {
        *ftab.offset(i as isize) = 0 as libc::c_int as UInt32;
        i -= 1;
    }
    j = (*block.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int;
    i = nblock - 1 as libc::c_int;
    while i >= 3 as libc::c_int {
        *quadrant.offset(i as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset(i as isize) as UInt16 as libc::c_int) << 8 as libc::c_int;
        *ftab.offset(j as isize) = (*ftab.offset(j as isize)).wrapping_add(1);
        *quadrant.offset((i - 1 as libc::c_int) as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset((i - 1 as libc::c_int) as isize) as UInt16 as libc::c_int)
                << 8 as libc::c_int;
        *ftab.offset(j as isize) = (*ftab.offset(j as isize)).wrapping_add(1);
        *quadrant.offset((i - 2 as libc::c_int) as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset((i - 2 as libc::c_int) as isize) as UInt16 as libc::c_int)
                << 8 as libc::c_int;
        *ftab.offset(j as isize) = (*ftab.offset(j as isize)).wrapping_add(1);
        *quadrant.offset((i - 3 as libc::c_int) as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset((i - 3 as libc::c_int) as isize) as UInt16 as libc::c_int)
                << 8 as libc::c_int;
        *ftab.offset(j as isize) = (*ftab.offset(j as isize)).wrapping_add(1);
        i -= 4 as libc::c_int;
    }
    while i >= 0 as libc::c_int {
        *quadrant.offset(i as isize) = 0 as libc::c_int as UInt16;
        j = j >> 8 as libc::c_int
            | (*block.offset(i as isize) as UInt16 as libc::c_int) << 8 as libc::c_int;
        *ftab.offset(j as isize) = (*ftab.offset(j as isize)).wrapping_add(1);
        i -= 1;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int + 12 as libc::c_int + 18 as libc::c_int + 2 as libc::c_int {
        *block.offset((nblock + i) as isize) = *block.offset(i as isize);
        *quadrant.offset((nblock + i) as isize) = 0 as libc::c_int as UInt16;
        i += 1;
    }
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        bucket sorting ...\n\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 1 as libc::c_int;
    while i <= 65536 as libc::c_int {
        *ftab.offset(i as isize) = (*ftab.offset(i as isize) as libc::c_uint)
            .wrapping_add(*ftab.offset((i - 1 as libc::c_int) as isize))
            as UInt32 as UInt32;
        i += 1;
    }
    s = ((*block.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int) as UInt16;
    i = nblock - 1 as libc::c_int;
    while i >= 3 as libc::c_int {
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset(i as isize) as libc::c_int) << 8 as libc::c_int) as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset((i - 1 as libc::c_int) as isize) as libc::c_int) << 8 as libc::c_int)
            as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 1 as libc::c_int) as UInt32;
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset((i - 2 as libc::c_int) as isize) as libc::c_int) << 8 as libc::c_int)
            as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 2 as libc::c_int) as UInt32;
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset((i - 3 as libc::c_int) as isize) as libc::c_int) << 8 as libc::c_int)
            as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 3 as libc::c_int) as UInt32;
        i -= 4 as libc::c_int;
    }
    while i >= 0 as libc::c_int {
        s = (s as libc::c_int >> 8 as libc::c_int
            | (*block.offset(i as isize) as libc::c_int) << 8 as libc::c_int) as UInt16;
        j = (*ftab.offset(s as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        i -= 1;
    }
    i = 0 as libc::c_int;
    while i <= 255 as libc::c_int {
        bigDone[i as usize] = 0 as libc::c_int as Bool;
        runningOrder[i as usize] = i;
        i += 1;
    }
    let mut vv: Int32 = 0;
    let mut h = 1 as libc::c_int;
    loop {
        h = 3 as libc::c_int * h + 1 as libc::c_int;
        if !(h <= 256 as libc::c_int) {
            break;
        }
    }
    loop {
        h = h / 3 as libc::c_int;
        i = h;
        while i <= 255 as libc::c_int {
            vv = runningOrder[i as usize];
            j = i;
            while (*ftab.offset(
                ((runningOrder[(j - h) as usize] + 1 as libc::c_int) << 8 as libc::c_int) as isize,
            ))
            .wrapping_sub(
                *ftab.offset((runningOrder[(j - h) as usize] << 8 as libc::c_int) as isize),
            ) > (*ftab.offset(((vv + 1 as libc::c_int) << 8 as libc::c_int) as isize))
                .wrapping_sub(*ftab.offset((vv << 8 as libc::c_int) as isize))
            {
                runningOrder[j as usize] = runningOrder[(j - h) as usize];
                j = j - h;
                if j <= h - 1 as libc::c_int {
                    break;
                }
            }
            runningOrder[j as usize] = vv;
            i += 1;
        }
        if !(h != 1 as libc::c_int) {
            break;
        }
    }
    numQSorted = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= 255 as libc::c_int {
        ss = runningOrder[i as usize];
        j = 0 as libc::c_int;
        while j <= 255 as libc::c_int {
            if j != ss {
                sb = (ss << 8 as libc::c_int) + j;
                if *ftab.offset(sb as isize)
                    & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint
                    == 0
                {
                    let mut lo = (*ftab.offset(sb as isize)
                        & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint)
                        as Int32;
                    let mut hi = (*ftab.offset((sb + 1 as libc::c_int) as isize)
                        & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as Int32;
                    if hi > lo {
                        if verb >= 4 as libc::c_int {
                            fprintf(
                                stderr,
                                b"        qsort [0x%x, 0x%x]   done %d   this %d\n\0" as *const u8
                                    as *const libc::c_char,
                                ss,
                                j,
                                numQSorted,
                                hi - lo + 1 as libc::c_int,
                            );
                        }
                        mainQSort3(
                            ptr,
                            block,
                            quadrant,
                            nblock,
                            lo,
                            hi,
                            2 as libc::c_int,
                            budget.as_deref_mut(),
                        );
                        numQSorted += hi - lo + 1 as libc::c_int;
                        if (*budget.as_deref().unwrap()) < 0 as libc::c_int {
                            return;
                        }
                    }
                }
                *ftab.offset(sb as isize) =
                    ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint;
            }
            j += 1;
        }
        if bigDone[ss as usize] != 0 {
            crate::src::bzlib::BZ2_bz__AssertH__fail(1006 as libc::c_int);
        }
        j = 0 as libc::c_int;
        while j <= 255 as libc::c_int {
            copyStart[j as usize] = (*ftab.offset(((j << 8 as libc::c_int) + ss) as isize)
                & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint)
                as Int32;
            copyEnd[j as usize] =
                (*ftab.offset(((j << 8 as libc::c_int) + ss + 1 as libc::c_int) as isize)
                    & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
            j += 1;
        }
        j = (*ftab.offset((ss << 8 as libc::c_int) as isize)
            & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint) as Int32;
        while j < copyStart[ss as usize] {
            k = (*ptr.offset(j as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
            if k < 0 as libc::c_int {
                k += nblock;
            }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh11 = copyStart[c1 as usize];
                copyStart[c1 as usize] = copyStart[c1 as usize] + 1;
                *ptr.offset(fresh11 as isize) = k as UInt32;
            }
            j += 1;
        }
        j = (*ftab.offset(((ss + 1 as libc::c_int) << 8 as libc::c_int) as isize)
            & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
        while j > copyEnd[ss as usize] {
            k = (*ptr.offset(j as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint) as Int32;
            if k < 0 as libc::c_int {
                k += nblock;
            }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh12 = copyEnd[c1 as usize];
                copyEnd[c1 as usize] = copyEnd[c1 as usize] - 1;
                *ptr.offset(fresh12 as isize) = k as UInt32;
            }
            j -= 1;
        }
        if !(copyStart[ss as usize] - 1 as libc::c_int == copyEnd[ss as usize]
            || copyStart[ss as usize] == 0 as libc::c_int
                && copyEnd[ss as usize] == nblock - 1 as libc::c_int)
        {
            crate::src::bzlib::BZ2_bz__AssertH__fail(1007 as libc::c_int);
        }
        j = 0 as libc::c_int;
        while j <= 255 as libc::c_int {
            *ftab.offset(((j << 8 as libc::c_int) + ss) as isize) =
                ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint;
            j += 1;
        }
        bigDone[ss as usize] = 1 as libc::c_int as Bool;
        if i < 255 as libc::c_int {
            let mut bbStart = (*ftab.offset((ss << 8 as libc::c_int) as isize)
                & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint)
                as Int32;
            let mut bbSize = (*ftab.offset(((ss + 1 as libc::c_int) << 8 as libc::c_int) as isize)
                & !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_uint)
                .wrapping_sub(bbStart as libc::c_uint) as Int32;
            let mut shifts = 0 as libc::c_int;
            while bbSize >> shifts > 65534 as libc::c_int {
                shifts += 1;
            }
            j = bbSize - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                let mut a2update = *ptr.offset((bbStart + j) as isize) as Int32;
                let mut qVal = (j >> shifts) as UInt16;
                *quadrant.offset(a2update as isize) = qVal;
                if a2update
                    < 2 as libc::c_int + 12 as libc::c_int + 18 as libc::c_int + 2 as libc::c_int
                {
                    *quadrant.offset((a2update + nblock) as isize) = qVal;
                }
                j -= 1;
            }
            if !(bbSize - 1 as libc::c_int >> shifts <= 65535 as libc::c_int) {
                crate::src::bzlib::BZ2_bz__AssertH__fail(1002 as libc::c_int);
            }
        }
        i += 1;
    }
    if verb >= 4 as libc::c_int {
        fprintf(
            stderr,
            b"        %d pointers, %d sorted, %d scanned\n\0" as *const u8 as *const libc::c_char,
            nblock,
            numQSorted,
            nblock - numQSorted,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_blockSort(mut s: *mut EState) {
    let mut ptr = (*s).ptr;
    let mut block = (*s).block;
    let mut ftab = (*s).ftab;
    let mut nblock = (*s).nblock;
    let mut verb = (*s).verbosity;
    let mut wfact = (*s).workFactor;
    let mut quadrant = 0 as *mut UInt16;
    let mut budget: Int32 = 0;
    let mut budgetInit: Int32 = 0;
    let mut i: Int32 = 0;
    if nblock < 10000 as libc::c_int {
        fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
    } else {
        i = nblock + (2 as libc::c_int + 12 as libc::c_int + 18 as libc::c_int + 2 as libc::c_int);
        if i & 1 as libc::c_int != 0 {
            i += 1;
        }
        quadrant = &raw mut *block.offset(i as isize) as *mut UChar as *mut UInt16;
        if wfact < 1 as libc::c_int {
            wfact = 1 as libc::c_int;
        }
        if wfact > 100 as libc::c_int {
            wfact = 100 as libc::c_int;
        }
        budgetInit = nblock * ((wfact - 1 as libc::c_int) / 3 as libc::c_int);
        budget = budgetInit;
        mainSort(ptr, block, quadrant, ftab, nblock, verb, Some(&mut budget));
        if verb >= 3 as libc::c_int {
            fprintf(
                stderr,
                b"      %d work, %d block, ratio %5.2f\n\0" as *const u8 as *const libc::c_char,
                budgetInit - budget,
                nblock,
                ((budgetInit - budget) as libc::c_float
                    / (if nblock == 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        nblock
                    }) as libc::c_float) as libc::c_double,
            );
        }
        if budget < 0 as libc::c_int {
            if verb >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"    too repetitive; using fallback sorting algorithm\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
        }
    }
    (*s).origPtr = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*s).nblock {
        if *ptr.offset(i as isize) == 0 as libc::c_int as libc::c_uint {
            (*s).origPtr = i;
            break;
        } else {
            i += 1;
        }
    }
    if !((*s).origPtr != -(1 as libc::c_int)) {
        crate::src::bzlib::BZ2_bz__AssertH__fail(1003 as libc::c_int);
    }
}
