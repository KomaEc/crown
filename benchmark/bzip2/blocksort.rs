
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn BZ2_bz__AssertH__fail(errcode: std::os::raw::c_int);
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut std::os::raw::c_char,
    pub avail_in: std::os::raw::c_uint,
    pub total_in_lo32: std::os::raw::c_uint,
    pub total_in_hi32: std::os::raw::c_uint,
    pub next_out: *mut std::os::raw::c_char,
    pub avail_out: std::os::raw::c_uint,
    pub total_out_lo32: std::os::raw::c_uint,
    pub total_out_hi32: std::os::raw::c_uint,
    pub state: *mut std::os::raw::c_void,
    pub bzalloc: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                             _: std::os::raw::c_int, _: std::os::raw::c_int)
                            -> *mut std::os::raw::c_void>,
    pub bzfree: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *mut std::os::raw::c_void) -> ()>,
    pub opaque: *mut std::os::raw::c_void,
}
pub type Bool = std::os::raw::c_uchar;
pub type UChar = std::os::raw::c_uchar;
pub type Int32 = std::os::raw::c_int;
pub type UInt32 = std::os::raw::c_uint;
pub type UInt16 = std::os::raw::c_ushort;
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
/*-------------------------------------------------------------*/
/*--- Block sorting machinery                               ---*/
/*---                                           blocksort.c ---*/
/*-------------------------------------------------------------*/
/* ------------------------------------------------------------------
   This file is part of bzip2/libbzip2, a program and library for
   lossless, block-sorting data compression.

   bzip2/libbzip2 version 1.0.8 of 13 July 2019
   Copyright (C) 1996-2019 Julian Seward <jseward@acm.org>

   Please read the WARNING, DISCLAIMER and PATENTS sections in the 
   README file.

   This program is released under the terms of the license contained
   in the file LICENSE.
   ------------------------------------------------------------------ */
/*---------------------------------------------*/
/*--- Fallback O(N log(N)^2) sorting        ---*/
/*--- algorithm, for repetitive blocks      ---*/
/*---------------------------------------------*/
/*---------------------------------------------*/
#[inline]
unsafe extern "C" fn fallbackSimpleSort(mut fmap: *mut UInt32,
                                        mut eclass: *mut UInt32,
                                        mut lo: Int32, mut hi: Int32) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: Int32 = 0;
    let mut ec_tmp: UInt32 = 0;
    if lo == hi { return }
    if hi - lo > 3 as std::os::raw::c_int {
        i = hi - 4 as std::os::raw::c_int;
        while i >= lo {
            tmp = *fmap.offset(i as isize) as Int32;
            ec_tmp = *eclass.offset(tmp as isize);
            j = i + 4 as std::os::raw::c_int;
            while j <= hi &&
                      ec_tmp >
                          *eclass.offset(*fmap.offset(j as isize) as isize) {
                *fmap.offset((j - 4 as std::os::raw::c_int) as isize) =
                    *fmap.offset(j as isize);
                j += 4 as std::os::raw::c_int
            }
            *fmap.offset((j - 4 as std::os::raw::c_int) as isize) = tmp as UInt32;
            i -= 1
        }
    }
    i = hi - 1 as std::os::raw::c_int;
    while i >= lo {
        tmp = *fmap.offset(i as isize) as Int32;
        ec_tmp = *eclass.offset(tmp as isize);
        j = i + 1 as std::os::raw::c_int;
        while j <= hi &&
                  ec_tmp > *eclass.offset(*fmap.offset(j as isize) as isize) {
            *fmap.offset((j - 1 as std::os::raw::c_int) as isize) =
                *fmap.offset(j as isize);
            j += 1
        }
        *fmap.offset((j - 1 as std::os::raw::c_int) as isize) = tmp as UInt32;
        i -= 1
    };
}
unsafe extern "C" fn fallbackQSort3(mut fmap: *mut UInt32,
                                    mut eclass: *mut UInt32, mut loSt: Int32,
                                    mut hiSt: Int32) {
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
    r = 0 as std::os::raw::c_int as UInt32;
    sp = 0 as std::os::raw::c_int;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    sp += 1;
    while sp > 0 as std::os::raw::c_int {
        if !(sp < 100 as std::os::raw::c_int - 1 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(1004 as std::os::raw::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        if hi - lo < 10 as std::os::raw::c_int {
            fallbackSimpleSort(fmap, eclass, lo, hi);
        } else {
            /* Random partitioning.  Median of 3 sometimes fails to
         avoid bad cases.  Median of 9 seems to help but 
         looks rather expensive.  This too seems to work but
         is cheaper.  Guidance for the magic constants 
         7621 and 32768 is taken from Sedgewick's algorithms
         book, chapter 35.
      */
            r =
                r.wrapping_mul(7621 as std::os::raw::c_int as
                                   std::os::raw::c_uint).wrapping_add(1 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_uint).wrapping_rem(32768
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 std::os::raw::c_uint);
            r3 = r.wrapping_rem(3 as std::os::raw::c_int as std::os::raw::c_uint);
            if r3 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                med = *eclass.offset(*fmap.offset(lo as isize) as isize)
            } else if r3 == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                med =
                    *eclass.offset(*fmap.offset((lo + hi >> 1 as std::os::raw::c_int)
                                                    as isize) as isize)
            } else {
                med = *eclass.offset(*fmap.offset(hi as isize) as isize)
            }
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            loop  {
                while !(unLo > unHi) {
                    n =
                        *eclass.offset(*fmap.offset(unLo as isize) as isize)
                            as Int32 - med as Int32;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp: Int32 =
                            *fmap.offset(unLo as isize) as Int32;
                        *fmap.offset(unLo as isize) =
                            *fmap.offset(ltLo as isize);
                        *fmap.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1
                    } else { if n > 0 as std::os::raw::c_int { break ; } unLo += 1 }
                }
                while !(unLo > unHi) {
                    n =
                        *eclass.offset(*fmap.offset(unHi as isize) as isize)
                            as Int32 - med as Int32;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp_0: Int32 =
                            *fmap.offset(unHi as isize) as Int32;
                        *fmap.offset(unHi as isize) =
                            *fmap.offset(gtHi as isize);
                        *fmap.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1
                    } else { if n < 0 as std::os::raw::c_int { break ; } unHi -= 1 }
                }
                if unLo > unHi { break ; }
                let mut zztmp_1: Int32 = *fmap.offset(unLo as isize) as Int32;
                *fmap.offset(unLo as isize) = *fmap.offset(unHi as isize);
                *fmap.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1
            }
            if gtHi < ltLo { continue ; }
            n =
                if ltLo - lo < unLo - ltLo {
                    (ltLo) - lo
                } else { (unLo) - ltLo };
            let mut yyp1: Int32 = lo;
            let mut yyp2: Int32 = unLo - n;
            let mut yyn: Int32 = n;
            while yyn > 0 as std::os::raw::c_int {
                let mut zztmp_2: Int32 = *fmap.offset(yyp1 as isize) as Int32;
                *fmap.offset(yyp1 as isize) = *fmap.offset(yyp2 as isize);
                *fmap.offset(yyp2 as isize) = zztmp_2 as UInt32;
                yyp1 += 1;
                yyp2 += 1;
                yyn -= 1
            }
            m =
                if hi - gtHi < gtHi - unHi {
                    (hi) - gtHi
                } else { (gtHi) - unHi };
            let mut yyp1_0: Int32 = unLo;
            let mut yyp2_0: Int32 = hi - m + 1 as std::os::raw::c_int;
            let mut yyn_0: Int32 = m;
            while yyn_0 > 0 as std::os::raw::c_int {
                let mut zztmp_3: Int32 =
                    *fmap.offset(yyp1_0 as isize) as Int32;
                *fmap.offset(yyp1_0 as isize) = *fmap.offset(yyp2_0 as isize);
                *fmap.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                yyp1_0 += 1;
                yyp2_0 += 1;
                yyn_0 -= 1
            }
            n = lo + unLo - ltLo - 1 as std::os::raw::c_int;
            m = hi - (gtHi - unHi) + 1 as std::os::raw::c_int;
            if n - lo > hi - m {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1;
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1
            } else {
                stackLo[sp as usize] = m;
                stackHi[sp as usize] = hi;
                sp += 1;
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = n;
                sp += 1
            }
        }
    };
}
unsafe extern "C" fn fallbackSort(mut fmap: *mut UInt32,
                                  mut eclass: *mut UInt32,
                                  mut bhtab: *mut UInt32, mut nblock: Int32,
                                  mut verb: Int32) {
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
    let mut eclass8: *mut UChar = eclass as *mut UChar;
    /*--
      Initial 1-char radix sort to generate
      initial fmap and initial BH bits.
   --*/
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        bucket sorting ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    i = 0 as std::os::raw::c_int;
    while i < 257 as std::os::raw::c_int {
        ftab[i as usize] = 0 as std::os::raw::c_int;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nblock {
        ftab[*eclass8.offset(i as isize) as usize] += 1;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        ftabCopy[i as usize] = ftab[i as usize];
        i += 1
    }
    i = 1 as std::os::raw::c_int;
    while i < 257 as std::os::raw::c_int {
        ftab[i as usize] += ftab[(i - 1 as std::os::raw::c_int) as usize];
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < nblock {
        j = *eclass8.offset(i as isize) as Int32;
        k = ftab[j as usize] - 1 as std::os::raw::c_int;
        ftab[j as usize] = k;
        *fmap.offset(k as isize) = i as UInt32;
        i += 1
    }
    nBhtab = 2 as std::os::raw::c_int + nblock / 32 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < nBhtab {
        *bhtab.offset(i as isize) = 0 as std::os::raw::c_int as UInt32;
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        let ref mut fresh0 =
            *bhtab.offset((ftab[i as usize] >> 5 as std::os::raw::c_int) as isize);
        *fresh0 |=
            (1 as std::os::raw::c_int as UInt32) <<
                (ftab[i as usize] & 31 as std::os::raw::c_int);
        i += 1
    }
    /*--
      Inductively refine the buckets.  Kind-of an
      "exponential radix sort" (!), inspired by the
      Manber-Myers suffix array construction algorithm.
   --*/
    /*-- set sentinel bits for block-end detection --*/
    i = 0 as std::os::raw::c_int;
    while i < 32 as std::os::raw::c_int {
        let ref mut fresh1 =
            *bhtab.offset((nblock + 2 as std::os::raw::c_int * i >> 5 as std::os::raw::c_int)
                              as isize);
        *fresh1 |=
            (1 as std::os::raw::c_int as UInt32) <<
                (nblock + 2 as std::os::raw::c_int * i & 31 as std::os::raw::c_int);
        let ref mut fresh2 =
            *bhtab.offset((nblock + 2 as std::os::raw::c_int * i + 1 as std::os::raw::c_int >>
                               5 as std::os::raw::c_int) as isize);
        *fresh2 &=
            !((1 as std::os::raw::c_int as UInt32) <<
                  (nblock + 2 as std::os::raw::c_int * i + 1 as std::os::raw::c_int &
                       31 as std::os::raw::c_int));
        i += 1
    }
    /*-- the log(N) loop --*/
    H = 1 as std::os::raw::c_int;
    loop  {
        if verb >= 4 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"        depth %6d has \x00" as *const u8 as
                        *const std::os::raw::c_char, H);
        }
        j = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < nblock {
            if *bhtab.offset((i >> 5 as std::os::raw::c_int) as isize) &
                   (1 as std::os::raw::c_int as UInt32) << (i & 31 as std::os::raw::c_int) !=
                   0 {
                j = i
            }
            k =
                (*fmap.offset(i as isize)).wrapping_sub(H as std::os::raw::c_uint) as
                    Int32;
            if k < 0 as std::os::raw::c_int { k += nblock }
            *eclass.offset(k as isize) = j as UInt32;
            i += 1
        }
        nNotDone = 0 as std::os::raw::c_int;
        r = -(1 as std::os::raw::c_int);
        loop  {
            /*-- find the next non-singleton bucket --*/
            k = r + 1 as std::os::raw::c_int;
            while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                      (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int)
                      != 0 && k & 0x1f as std::os::raw::c_int != 0 {
                k += 1
            }
            if *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                   (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int) !=
                   0 {
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) ==
                          0xffffffff as std::os::raw::c_uint {
                    k += 32 as std::os::raw::c_int
                }
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                          (1 as std::os::raw::c_int as UInt32) <<
                              (k & 31 as std::os::raw::c_int) != 0 {
                    k += 1
                }
            }
            l = k - 1 as std::os::raw::c_int;
            if l >= nblock { break ; }
            while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                      (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int)
                      == 0 && k & 0x1f as std::os::raw::c_int != 0 {
                k += 1
            }
            if *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                   (1 as std::os::raw::c_int as UInt32) << (k & 31 as std::os::raw::c_int) ==
                   0 {
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) ==
                          0 as std::os::raw::c_int as std::os::raw::c_uint {
                    k += 32 as std::os::raw::c_int
                }
                while *bhtab.offset((k >> 5 as std::os::raw::c_int) as isize) &
                          (1 as std::os::raw::c_int as UInt32) <<
                              (k & 31 as std::os::raw::c_int) == 0 {
                    k += 1
                }
            }
            r = k - 1 as std::os::raw::c_int;
            if r >= nblock { break ; }
            /*-- now [l, r] bracket current bucket --*/
            if r > l {
                nNotDone += r - l + 1 as std::os::raw::c_int;
                fallbackQSort3(fmap, eclass, l, r);
                /*-- scan bucket and generate header bits-- */
                cc = -(1 as std::os::raw::c_int);
                i = l;
                while i <= r {
                    cc1 =
                        *eclass.offset(*fmap.offset(i as isize) as isize) as
                            Int32;
                    if cc != cc1 {
                        let ref mut fresh3 =
                            *bhtab.offset((i >> 5 as std::os::raw::c_int) as isize);
                        *fresh3 |=
                            (1 as std::os::raw::c_int as UInt32) <<
                                (i & 31 as std::os::raw::c_int);
                        cc = cc1
                    }
                    i += 1
                }
            }
        }
        if verb >= 4 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%6d unresolved strings\n\x00" as *const u8 as
                        *const std::os::raw::c_char, nNotDone);
        }
        H *= 2 as std::os::raw::c_int;
        if H > nblock || nNotDone == 0 as std::os::raw::c_int { break ; }
    }
    /*-- 
      Reconstruct the original block in
      eclass8 [0 .. nblock-1], since the
      previous phase destroyed it.
   --*/
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        reconstructing block ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    j = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < nblock {
        while ftabCopy[j as usize] == 0 as std::os::raw::c_int { j += 1 }
        ftabCopy[j as usize] -= 1;
        *eclass8.offset(*fmap.offset(i as isize) as isize) = j as UChar;
        i += 1
    }
    if !(j < 256 as std::os::raw::c_int) {
        BZ2_bz__AssertH__fail(1005 as std::os::raw::c_int);
    };
}
/*---------------------------------------------*/
/*--- The main, O(N^2 log(N)) sorting       ---*/
/*--- algorithm.  Faster for "normal"       ---*/
/*--- non-repetitive blocks.                ---*/
/*---------------------------------------------*/
/*---------------------------------------------*/
#[inline]
unsafe extern "C" fn mainGtU(mut i1: UInt32, mut i2: UInt32,
                             mut block: *mut UChar, mut quadrant: *mut UInt16,
                             mut nblock: UInt32, mut budget: *mut Int32)
 -> Bool {
    let mut k: Int32 = 0;
    let mut c1: UChar = 0;
    let mut c2: UChar = 0;
    let mut s1: UInt16 = 0;
    let mut s2: UInt16 = 0;
    /* 1 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 2 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 3 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 4 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 5 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 6 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 7 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 8 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 9 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 10 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 11 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    /* 12 */
    c1 = *block.offset(i1 as isize);
    c2 = *block.offset(i2 as isize);
    if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
        return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as Bool
    }
    i1 = i1.wrapping_add(1);
    i2 = i2.wrapping_add(1);
    k = nblock.wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_uint) as Int32;
    loop  {
        /* 1 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 2 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 3 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 4 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 5 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 6 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 7 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        /* 8 */
        c1 = *block.offset(i1 as isize);
        c2 = *block.offset(i2 as isize);
        if c1 as std::os::raw::c_int != c2 as std::os::raw::c_int {
            return (c1 as std::os::raw::c_int > c2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        s1 = *quadrant.offset(i1 as isize);
        s2 = *quadrant.offset(i2 as isize);
        if s1 as std::os::raw::c_int != s2 as std::os::raw::c_int {
            return (s1 as std::os::raw::c_int > s2 as std::os::raw::c_int) as std::os::raw::c_int as
                       Bool
        }
        i1 = i1.wrapping_add(1);
        i2 = i2.wrapping_add(1);
        if i1 >= nblock {
            i1 = (i1 as std::os::raw::c_uint).wrapping_sub(nblock) as UInt32 as UInt32
        }
        if i2 >= nblock {
            i2 = (i2 as std::os::raw::c_uint).wrapping_sub(nblock) as UInt32 as UInt32
        }
        k -= 8 as std::os::raw::c_int;
        *budget -= 1;
        if !(k >= 0 as std::os::raw::c_int) { break ; }
    }
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
/*--
   Knuth's increments seem to work better
   than Incerpi-Sedgewick here.  Possibly
   because the number of elems to sort is
   usually small, typically <= 20.
--*/
static mut incs: [Int32; 14] =
    [1 as std::os::raw::c_int, 4 as std::os::raw::c_int, 13 as std::os::raw::c_int, 40 as std::os::raw::c_int,
     121 as std::os::raw::c_int, 364 as std::os::raw::c_int, 1093 as std::os::raw::c_int,
     3280 as std::os::raw::c_int, 9841 as std::os::raw::c_int, 29524 as std::os::raw::c_int,
     88573 as std::os::raw::c_int, 265720 as std::os::raw::c_int, 797161 as std::os::raw::c_int,
     2391484 as std::os::raw::c_int];
unsafe extern "C" fn mainSimpleSort(mut ptr: *mut UInt32,
                                    mut block: *mut UChar,
                                    mut quadrant: *mut UInt16,
                                    mut nblock: Int32, mut lo: Int32,
                                    mut hi: Int32, mut d: Int32,
                                    mut budget: *mut Int32) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut h: Int32 = 0;
    let mut bigN: Int32 = 0;
    let mut hp: Int32 = 0;
    let mut v: UInt32 = 0;
    bigN = hi - lo + 1 as std::os::raw::c_int;
    if bigN < 2 as std::os::raw::c_int { return }
    hp = 0 as std::os::raw::c_int;
    while incs[hp as usize] < bigN { hp += 1 }
    hp -= 1;
    while hp >= 0 as std::os::raw::c_int {
        h = incs[hp as usize];
        i = lo + h;
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*-- copy 1 --*/
            if i > hi { break ; }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU((*ptr.offset((j - h) as
                                           isize)).wrapping_add(d as
                                                                    std::os::raw::c_uint),
                          v.wrapping_add(d as std::os::raw::c_uint), block, quadrant,
                          nblock as UInt32, budget) != 0 {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as std::os::raw::c_int { break ; }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            /*-- copy 2 --*/
            if i > hi { break ; }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU((*ptr.offset((j - h) as
                                           isize)).wrapping_add(d as
                                                                    std::os::raw::c_uint),
                          v.wrapping_add(d as std::os::raw::c_uint), block, quadrant,
                          nblock as UInt32, budget) != 0 {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as std::os::raw::c_int { break ; }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            /*-- copy 3 --*/
            if i > hi { break ; }
            v = *ptr.offset(i as isize);
            j = i;
            while mainGtU((*ptr.offset((j - h) as
                                           isize)).wrapping_add(d as
                                                                    std::os::raw::c_uint),
                          v.wrapping_add(d as std::os::raw::c_uint), block, quadrant,
                          nblock as UInt32, budget) != 0 {
                *ptr.offset(j as isize) = *ptr.offset((j - h) as isize);
                j = j - h;
                if j <= lo + h - 1 as std::os::raw::c_int { break ; }
            }
            *ptr.offset(j as isize) = v;
            i += 1;
            if *budget < 0 as std::os::raw::c_int { return }
        }
        hp -= 1
    };
}
/*---------------------------------------------*/
/*--
   The following is an implementation of
   an elegant 3-way quicksort for strings,
   described in a paper "Fast Algorithms for
   Sorting and Searching Strings", by Robert
   Sedgewick and Jon L. Bentley.
--*/
#[inline]
unsafe extern "C" fn mmed3(mut a: UChar, mut b: UChar, mut c: UChar)
 -> UChar {
    let mut t: UChar = 0;
    if a as std::os::raw::c_int > b as std::os::raw::c_int { t = a; a = b; b = t }
    if b as std::os::raw::c_int > c as std::os::raw::c_int {
        b = c;
        if a as std::os::raw::c_int > b as std::os::raw::c_int { b = a }
    }
    return b;
}
unsafe extern "C" fn mainQSort3(mut ptr: *mut UInt32, mut block: *mut UChar,
                                mut quadrant: *mut UInt16, mut nblock: Int32,
                                mut loSt: Int32, mut hiSt: Int32,
                                mut dSt: Int32, mut budget: *mut Int32) {
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
    sp = 0 as std::os::raw::c_int;
    stackLo[sp as usize] = loSt;
    stackHi[sp as usize] = hiSt;
    stackD[sp as usize] = dSt;
    sp += 1;
    while sp > 0 as std::os::raw::c_int {
        if !(sp < 100 as std::os::raw::c_int - 2 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(1001 as std::os::raw::c_int);
        }
        sp -= 1;
        lo = stackLo[sp as usize];
        hi = stackHi[sp as usize];
        d = stackD[sp as usize];
        if hi - lo < 20 as std::os::raw::c_int ||
               d > 2 as std::os::raw::c_int + 12 as std::os::raw::c_int {
            mainSimpleSort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 as std::os::raw::c_int { return }
        } else {
            med =
                mmed3(*block.offset((*ptr.offset(lo as
                                                     isize)).wrapping_add(d as
                                                                              std::os::raw::c_uint)
                                        as isize),
                      *block.offset((*ptr.offset(hi as
                                                     isize)).wrapping_add(d as
                                                                              std::os::raw::c_uint)
                                        as isize),
                      *block.offset((*ptr.offset((lo + hi >> 1 as std::os::raw::c_int)
                                                     as
                                                     isize)).wrapping_add(d as
                                                                              std::os::raw::c_uint)
                                        as isize)) as Int32;
            ltLo = lo;
            unLo = ltLo;
            gtHi = hi;
            unHi = gtHi;
            while 1 as std::os::raw::c_int as Bool != 0 {
                while 1 as std::os::raw::c_int as Bool != 0 {
                    if unLo > unHi { break ; }
                    n =
                        *block.offset((*ptr.offset(unLo as
                                                       isize)).wrapping_add(d
                                                                                as
                                                                                std::os::raw::c_uint)
                                          as isize) as Int32 - med;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp: Int32 =
                            *ptr.offset(unLo as isize) as Int32;
                        *ptr.offset(unLo as isize) =
                            *ptr.offset(ltLo as isize);
                        *ptr.offset(ltLo as isize) = zztmp as UInt32;
                        ltLo += 1;
                        unLo += 1
                    } else { if n > 0 as std::os::raw::c_int { break ; } unLo += 1 }
                }
                while 1 as std::os::raw::c_int as Bool != 0 {
                    if unLo > unHi { break ; }
                    n =
                        *block.offset((*ptr.offset(unHi as
                                                       isize)).wrapping_add(d
                                                                                as
                                                                                std::os::raw::c_uint)
                                          as isize) as Int32 - med;
                    if n == 0 as std::os::raw::c_int {
                        let mut zztmp_0: Int32 =
                            *ptr.offset(unHi as isize) as Int32;
                        *ptr.offset(unHi as isize) =
                            *ptr.offset(gtHi as isize);
                        *ptr.offset(gtHi as isize) = zztmp_0 as UInt32;
                        gtHi -= 1;
                        unHi -= 1
                    } else { if n < 0 as std::os::raw::c_int { break ; } unHi -= 1 }
                }
                if unLo > unHi { break ; }
                let mut zztmp_1: Int32 = *ptr.offset(unLo as isize) as Int32;
                *ptr.offset(unLo as isize) = *ptr.offset(unHi as isize);
                *ptr.offset(unHi as isize) = zztmp_1 as UInt32;
                unLo += 1;
                unHi -= 1
            }
            if gtHi < ltLo {
                stackLo[sp as usize] = lo;
                stackHi[sp as usize] = hi;
                stackD[sp as usize] = d + 1 as std::os::raw::c_int;
                sp += 1
            } else {
                n =
                    if ltLo - lo < unLo - ltLo {
                        (ltLo) - lo
                    } else { (unLo) - ltLo };
                let mut yyp1: Int32 = lo;
                let mut yyp2: Int32 = unLo - n;
                let mut yyn: Int32 = n;
                while yyn > 0 as std::os::raw::c_int {
                    let mut zztmp_2: Int32 =
                        *ptr.offset(yyp1 as isize) as Int32;
                    *ptr.offset(yyp1 as isize) = *ptr.offset(yyp2 as isize);
                    *ptr.offset(yyp2 as isize) = zztmp_2 as UInt32;
                    yyp1 += 1;
                    yyp2 += 1;
                    yyn -= 1
                }
                m =
                    if hi - gtHi < gtHi - unHi {
                        (hi) - gtHi
                    } else { (gtHi) - unHi };
                let mut yyp1_0: Int32 = unLo;
                let mut yyp2_0: Int32 = hi - m + 1 as std::os::raw::c_int;
                let mut yyn_0: Int32 = m;
                while yyn_0 > 0 as std::os::raw::c_int {
                    let mut zztmp_3: Int32 =
                        *ptr.offset(yyp1_0 as isize) as Int32;
                    *ptr.offset(yyp1_0 as isize) =
                        *ptr.offset(yyp2_0 as isize);
                    *ptr.offset(yyp2_0 as isize) = zztmp_3 as UInt32;
                    yyp1_0 += 1;
                    yyp2_0 += 1;
                    yyn_0 -= 1
                }
                n = lo + unLo - ltLo - 1 as std::os::raw::c_int;
                m = hi - (gtHi - unHi) + 1 as std::os::raw::c_int;
                nextLo[0 as std::os::raw::c_int as usize] = lo;
                nextHi[0 as std::os::raw::c_int as usize] = n;
                nextD[0 as std::os::raw::c_int as usize] = d;
                nextLo[1 as std::os::raw::c_int as usize] = m;
                nextHi[1 as std::os::raw::c_int as usize] = hi;
                nextD[1 as std::os::raw::c_int as usize] = d;
                nextLo[2 as std::os::raw::c_int as usize] = n + 1 as std::os::raw::c_int;
                nextHi[2 as std::os::raw::c_int as usize] = m - 1 as std::os::raw::c_int;
                nextD[2 as std::os::raw::c_int as usize] = d + 1 as std::os::raw::c_int;
                if nextHi[0 as std::os::raw::c_int as usize] -
                       nextLo[0 as std::os::raw::c_int as usize] <
                       nextHi[1 as std::os::raw::c_int as usize] -
                           nextLo[1 as std::os::raw::c_int as usize] {
                    let mut tz: Int32 = 0;
                    tz = nextLo[0 as std::os::raw::c_int as usize];
                    nextLo[0 as std::os::raw::c_int as usize] =
                        nextLo[1 as std::os::raw::c_int as usize];
                    nextLo[1 as std::os::raw::c_int as usize] = tz;
                    tz = nextHi[0 as std::os::raw::c_int as usize];
                    nextHi[0 as std::os::raw::c_int as usize] =
                        nextHi[1 as std::os::raw::c_int as usize];
                    nextHi[1 as std::os::raw::c_int as usize] = tz;
                    tz = nextD[0 as std::os::raw::c_int as usize];
                    nextD[0 as std::os::raw::c_int as usize] =
                        nextD[1 as std::os::raw::c_int as usize];
                    nextD[1 as std::os::raw::c_int as usize] = tz
                }
                if nextHi[1 as std::os::raw::c_int as usize] -
                       nextLo[1 as std::os::raw::c_int as usize] <
                       nextHi[2 as std::os::raw::c_int as usize] -
                           nextLo[2 as std::os::raw::c_int as usize] {
                    let mut tz_0: Int32 = 0;
                    tz_0 = nextLo[1 as std::os::raw::c_int as usize];
                    nextLo[1 as std::os::raw::c_int as usize] =
                        nextLo[2 as std::os::raw::c_int as usize];
                    nextLo[2 as std::os::raw::c_int as usize] = tz_0;
                    tz_0 = nextHi[1 as std::os::raw::c_int as usize];
                    nextHi[1 as std::os::raw::c_int as usize] =
                        nextHi[2 as std::os::raw::c_int as usize];
                    nextHi[2 as std::os::raw::c_int as usize] = tz_0;
                    tz_0 = nextD[1 as std::os::raw::c_int as usize];
                    nextD[1 as std::os::raw::c_int as usize] =
                        nextD[2 as std::os::raw::c_int as usize];
                    nextD[2 as std::os::raw::c_int as usize] = tz_0
                }
                if nextHi[0 as std::os::raw::c_int as usize] -
                       nextLo[0 as std::os::raw::c_int as usize] <
                       nextHi[1 as std::os::raw::c_int as usize] -
                           nextLo[1 as std::os::raw::c_int as usize] {
                    let mut tz_1: Int32 = 0;
                    tz_1 = nextLo[0 as std::os::raw::c_int as usize];
                    nextLo[0 as std::os::raw::c_int as usize] =
                        nextLo[1 as std::os::raw::c_int as usize];
                    nextLo[1 as std::os::raw::c_int as usize] = tz_1;
                    tz_1 = nextHi[0 as std::os::raw::c_int as usize];
                    nextHi[0 as std::os::raw::c_int as usize] =
                        nextHi[1 as std::os::raw::c_int as usize];
                    nextHi[1 as std::os::raw::c_int as usize] = tz_1;
                    tz_1 = nextD[0 as std::os::raw::c_int as usize];
                    nextD[0 as std::os::raw::c_int as usize] =
                        nextD[1 as std::os::raw::c_int as usize];
                    nextD[1 as std::os::raw::c_int as usize] = tz_1
                }
                stackLo[sp as usize] = nextLo[0 as std::os::raw::c_int as usize];
                stackHi[sp as usize] = nextHi[0 as std::os::raw::c_int as usize];
                stackD[sp as usize] = nextD[0 as std::os::raw::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[1 as std::os::raw::c_int as usize];
                stackHi[sp as usize] = nextHi[1 as std::os::raw::c_int as usize];
                stackD[sp as usize] = nextD[1 as std::os::raw::c_int as usize];
                sp += 1;
                stackLo[sp as usize] = nextLo[2 as std::os::raw::c_int as usize];
                stackHi[sp as usize] = nextHi[2 as std::os::raw::c_int as usize];
                stackD[sp as usize] = nextD[2 as std::os::raw::c_int as usize];
                sp += 1
            }
        }
    };
}
unsafe extern "C" fn mainSort(mut ptr: *mut UInt32, mut block: *mut UChar,
                              mut quadrant: *mut UInt16,
                              mut ftab: *mut UInt32, mut nblock: Int32,
                              mut verb: Int32, mut budget: *mut Int32) {
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
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        main sort initialise ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    /*-- set up the 2-byte frequency table --*/
    i = 65536 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        *ftab.offset(i as isize) = 0 as std::os::raw::c_int as UInt32;
        i -= 1
    }
    j =
        (*block.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) <<
            8 as std::os::raw::c_int;
    i = nblock - 1 as std::os::raw::c_int;
    while i >= 3 as std::os::raw::c_int {
        *quadrant.offset(i as isize) = 0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset(i as isize) as UInt16 as std::os::raw::c_int) <<
                    8 as std::os::raw::c_int;
        let ref mut fresh4 = *ftab.offset(j as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *quadrant.offset((i - 1 as std::os::raw::c_int) as isize) =
            0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset((i - 1 as std::os::raw::c_int) as isize) as UInt16 as
                     std::os::raw::c_int) << 8 as std::os::raw::c_int;
        let ref mut fresh5 = *ftab.offset(j as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        *quadrant.offset((i - 2 as std::os::raw::c_int) as isize) =
            0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset((i - 2 as std::os::raw::c_int) as isize) as UInt16 as
                     std::os::raw::c_int) << 8 as std::os::raw::c_int;
        let ref mut fresh6 = *ftab.offset(j as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        *quadrant.offset((i - 3 as std::os::raw::c_int) as isize) =
            0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset((i - 3 as std::os::raw::c_int) as isize) as UInt16 as
                     std::os::raw::c_int) << 8 as std::os::raw::c_int;
        let ref mut fresh7 = *ftab.offset(j as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
        i -= 4 as std::os::raw::c_int
    }
    while i >= 0 as std::os::raw::c_int {
        *quadrant.offset(i as isize) = 0 as std::os::raw::c_int as UInt16;
        j =
            j >> 8 as std::os::raw::c_int |
                (*block.offset(i as isize) as UInt16 as std::os::raw::c_int) <<
                    8 as std::os::raw::c_int;
        let ref mut fresh8 = *ftab.offset(j as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
        i -= 1
    }
    /*-- (emphasises close relationship of block & quadrant) --*/
    i = 0 as std::os::raw::c_int;
    while i <
              2 as std::os::raw::c_int + 12 as std::os::raw::c_int + 18 as std::os::raw::c_int +
                  2 as std::os::raw::c_int {
        *block.offset((nblock + i) as isize) = *block.offset(i as isize);
        *quadrant.offset((nblock + i) as isize) = 0 as std::os::raw::c_int as UInt16;
        i += 1
    }
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        bucket sorting ...\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
    }
    /*-- Complete the initial radix sort --*/
    i = 1 as std::os::raw::c_int;
    while i <= 65536 as std::os::raw::c_int {
        let ref mut fresh9 = *ftab.offset(i as isize);
        *fresh9 =
            (*fresh9 as
                 std::os::raw::c_uint).wrapping_add(*ftab.offset((i -
                                                              1 as
                                                                  std::os::raw::c_int)
                                                             as isize)) as
                UInt32 as UInt32;
        i += 1
    }
    s =
        ((*block.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) <<
             8 as std::os::raw::c_int) as UInt16;
    i = nblock - 1 as std::os::raw::c_int;
    while i >= 3 as std::os::raw::c_int {
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset(i as isize) as std::os::raw::c_int) <<
                     8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset((i - 1 as std::os::raw::c_int) as isize) as
                      std::os::raw::c_int) << 8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 1 as std::os::raw::c_int) as UInt32;
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset((i - 2 as std::os::raw::c_int) as isize) as
                      std::os::raw::c_int) << 8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 2 as std::os::raw::c_int) as UInt32;
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset((i - 3 as std::os::raw::c_int) as isize) as
                      std::os::raw::c_int) << 8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = (i - 3 as std::os::raw::c_int) as UInt32;
        i -= 4 as std::os::raw::c_int
    }
    while i >= 0 as std::os::raw::c_int {
        s =
            (s as std::os::raw::c_int >> 8 as std::os::raw::c_int |
                 (*block.offset(i as isize) as std::os::raw::c_int) <<
                     8 as std::os::raw::c_int) as UInt16;
        j =
            (*ftab.offset(s as
                              isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as Int32;
        *ftab.offset(s as isize) = j as UInt32;
        *ptr.offset(j as isize) = i as UInt32;
        i -= 1
    }
    /*--
      Now ftab contains the first loc of every small bucket.
      Calculate the running order, from smallest to largest
      big bucket.
   --*/
    i = 0 as std::os::raw::c_int;
    while i <= 255 as std::os::raw::c_int {
        bigDone[i as usize] = 0 as std::os::raw::c_int as Bool;
        runningOrder[i as usize] = i;
        i += 1
    }
    let mut vv: Int32 = 0;
    let mut h: Int32 = 1 as std::os::raw::c_int;
    loop  {
        h = 3 as std::os::raw::c_int * h + 1 as std::os::raw::c_int;
        if !(h <= 256 as std::os::raw::c_int) { break ; }
    }
    loop  {
        h = h / 3 as std::os::raw::c_int;
        i = h;
        while i <= 255 as std::os::raw::c_int {
            vv = runningOrder[i as usize];
            j = i;
            while (*ftab.offset(((runningOrder[(j - h) as usize] +
                                      1 as std::os::raw::c_int) << 8 as std::os::raw::c_int)
                                    as
                                    isize)).wrapping_sub(*ftab.offset((runningOrder[(j
                                                                                         -
                                                                                         h)
                                                                                        as
                                                                                        usize]
                                                                           <<
                                                                           8
                                                                               as
                                                                               std::os::raw::c_int)
                                                                          as
                                                                          isize))
                      >
                      (*ftab.offset(((vv + 1 as std::os::raw::c_int) <<
                                         8 as std::os::raw::c_int) as
                                        isize)).wrapping_sub(*ftab.offset((vv
                                                                               <<
                                                                               8
                                                                                   as
                                                                                   std::os::raw::c_int)
                                                                              as
                                                                              isize))
                  {
                runningOrder[j as usize] = runningOrder[(j - h) as usize];
                j = j - h;
                if j <= h - 1 as std::os::raw::c_int { break ; }
            }
            runningOrder[j as usize] = vv;
            i += 1
        }
        if !(h != 1 as std::os::raw::c_int) { break ; }
    }
    /*--
      The main sorting loop.
   --*/
    numQSorted = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i <= 255 as std::os::raw::c_int {
        /*--
         Process big buckets, starting with the least full.
         Basically this is a 3-step process in which we call
         mainQSort3 to sort the small buckets [ss, j], but
         also make a big effort to avoid the calls if we can.
      --*/
        ss = runningOrder[i as usize];
        /*--
         Step 1:
         Complete the big bucket [ss] by quicksorting
         any unsorted small buckets [ss, j], for j != ss.  
         Hopefully previous pointer-scanning phases have already
         completed many of the small buckets [ss, j], so
         we don't have to sort them at all.
      --*/
        j = 0 as std::os::raw::c_int;
        while j <= 255 as std::os::raw::c_int {
            if j != ss {
                sb = (ss << 8 as std::os::raw::c_int) + j;
                if *ftab.offset(sb as isize) &
                       ((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                           std::os::raw::c_uint == 0 {
                    let mut lo: Int32 =
                        (*ftab.offset(sb as isize) &
                             !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                                 std::os::raw::c_uint) as Int32;
                    let mut hi: Int32 =
                        (*ftab.offset((sb + 1 as std::os::raw::c_int) as isize) &
                             !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                                 std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_uint)
                            as Int32;
                    if hi > lo {
                        if verb >= 4 as std::os::raw::c_int {
                            fprintf(__stderrp,
                                    b"        qsort [0x%x, 0x%x]   done %d   this %d\n\x00"
                                        as *const u8 as *const std::os::raw::c_char,
                                    ss, j, numQSorted,
                                    hi - lo + 1 as std::os::raw::c_int);
                        }
                        mainQSort3(ptr, block, quadrant, nblock, lo, hi,
                                   2 as std::os::raw::c_int, budget);
                        numQSorted += hi - lo + 1 as std::os::raw::c_int;
                        if *budget < 0 as std::os::raw::c_int { return }
                    }
                }
                let ref mut fresh10 = *ftab.offset(sb as isize);
                *fresh10 |=
                    ((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as std::os::raw::c_uint
            }
            j += 1
        }
        if bigDone[ss as usize] != 0 {
            BZ2_bz__AssertH__fail(1006 as std::os::raw::c_int);
        }
        /*--
         Step 2:
         Now scan this big bucket [ss] so as to synthesise the
         sorted order for small buckets [t, ss] for all t,
         including, magically, the bucket [ss,ss] too.
         This will avoid doing Real Work in subsequent Step 1's.
      --*/
        j = 0 as std::os::raw::c_int;
        while j <= 255 as std::os::raw::c_int {
            copyStart[j as usize] =
                (*ftab.offset(((j << 8 as std::os::raw::c_int) + ss) as isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint) as Int32;
            copyEnd[j as usize] =
                (*ftab.offset(((j << 8 as std::os::raw::c_int) + ss +
                                   1 as std::os::raw::c_int) as isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_uint) as
                    Int32;
            j += 1
        }
        j =
            (*ftab.offset((ss << 8 as std::os::raw::c_int) as isize) &
                 !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as std::os::raw::c_uint)
                as Int32;
        while j < copyStart[ss as usize] {
            k =
                (*ptr.offset(j as
                                 isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_uint) as
                    Int32;
            if k < 0 as std::os::raw::c_int { k += nblock }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh11 = copyStart[c1 as usize];
                copyStart[c1 as usize] = copyStart[c1 as usize] + 1;
                *ptr.offset(fresh11 as isize) = k as UInt32
            }
            j += 1
        }
        j =
            (*ftab.offset(((ss + 1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) as
                              isize) &
                 !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                     std::os::raw::c_uint).wrapping_sub(1 as std::os::raw::c_int as
                                                    std::os::raw::c_uint) as Int32;
        while j > copyEnd[ss as usize] {
            k =
                (*ptr.offset(j as
                                 isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                          std::os::raw::c_uint) as
                    Int32;
            if k < 0 as std::os::raw::c_int { k += nblock }
            c1 = *block.offset(k as isize);
            if bigDone[c1 as usize] == 0 {
                let fresh12 = copyEnd[c1 as usize];
                copyEnd[c1 as usize] = copyEnd[c1 as usize] - 1;
                *ptr.offset(fresh12 as isize) = k as UInt32
            }
            j -= 1
        }
        if !(copyStart[ss as usize] - 1 as std::os::raw::c_int == copyEnd[ss as usize]
                 ||
                 copyStart[ss as usize] == 0 as std::os::raw::c_int &&
                     copyEnd[ss as usize] == nblock - 1 as std::os::raw::c_int) {
            BZ2_bz__AssertH__fail(1007 as std::os::raw::c_int);
        }
        j = 0 as std::os::raw::c_int;
        while j <= 255 as std::os::raw::c_int {
            let ref mut fresh13 =
                *ftab.offset(((j << 8 as std::os::raw::c_int) + ss) as isize);
            *fresh13 |=
                ((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as std::os::raw::c_uint;
            j += 1
        }
        /*--
         Step 3:
         The [ss] big bucket is now done.  Record this fact,
         and update the quadrant descriptors.  Remember to
         update quadrants in the overshoot area too, if
         necessary.  The "if (i < 255)" test merely skips
         this updating for the last bucket processed, since
         updating for the last bucket is pointless.

         The quadrant array provides a way to incrementally
         cache sort orderings, as they appear, so as to 
         make subsequent comparisons in fullGtU() complete
         faster.  For repetitive blocks this makes a big
         difference (but not big enough to be able to avoid
         the fallback sorting mechanism, exponential radix sort).

         The precise meaning is: at all times:

            for 0 <= i < nblock and 0 <= j <= nblock

            if block[i] != block[j], 

               then the relative values of quadrant[i] and 
                    quadrant[j] are meaningless.

               else {
                  if quadrant[i] < quadrant[j]
                     then the string starting at i lexicographically
                     precedes the string starting at j

                  else if quadrant[i] > quadrant[j]
                     then the string starting at j lexicographically
                     precedes the string starting at i

                  else
                     the relative ordering of the strings starting
                     at i and j has not yet been determined.
               }
      --*/
        bigDone[ss as usize] = 1 as std::os::raw::c_int as Bool;
        if i < 255 as std::os::raw::c_int {
            let mut bbStart: Int32 =
                (*ftab.offset((ss << 8 as std::os::raw::c_int) as isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint) as Int32;
            let mut bbSize: Int32 =
                (*ftab.offset(((ss + 1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) as
                                  isize) &
                     !((1 as std::os::raw::c_int) << 21 as std::os::raw::c_int) as
                         std::os::raw::c_uint).wrapping_sub(bbStart as std::os::raw::c_uint)
                    as Int32;
            let mut shifts: Int32 = 0 as std::os::raw::c_int;
            while bbSize >> shifts > 65534 as std::os::raw::c_int { shifts += 1 }
            j = bbSize - 1 as std::os::raw::c_int;
            while j >= 0 as std::os::raw::c_int {
                let mut a2update: Int32 =
                    *ptr.offset((bbStart + j) as isize) as Int32;
                let mut qVal: UInt16 = (j >> shifts) as UInt16;
                *quadrant.offset(a2update as isize) = qVal;
                if a2update <
                       2 as std::os::raw::c_int + 12 as std::os::raw::c_int +
                           18 as std::os::raw::c_int + 2 as std::os::raw::c_int {
                    *quadrant.offset((a2update + nblock) as isize) = qVal
                }
                j -= 1
            }
            if !(bbSize - 1 as std::os::raw::c_int >> shifts <= 65535 as std::os::raw::c_int)
               {
                BZ2_bz__AssertH__fail(1002 as std::os::raw::c_int);
            }
        }
        i += 1
    }
    if verb >= 4 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"        %d pointers, %d sorted, %d scanned\n\x00" as
                    *const u8 as *const std::os::raw::c_char, nblock, numQSorted,
                nblock - numQSorted);
    };
}
/*---------------------------------------------*/
/* Pre:
      nblock > 0
      arr2 exists for [0 .. nblock-1 +N_OVERSHOOT]
      ((UChar*)arr2)  [0 .. nblock-1] holds block
      arr1 exists for [0 .. nblock-1]

   Post:
      ((UChar*)arr2) [0 .. nblock-1] holds block
      All other areas of block destroyed
      ftab [ 0 .. 65536 ] destroyed
      arr1 [0 .. nblock-1] holds sorted order
*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_blockSort(mut s: *mut EState) {
    let mut ptr: *mut UInt32 = (*s).ptr;
    let mut block: *mut UChar = (*s).block;
    let mut ftab: *mut UInt32 = (*s).ftab;
    let mut nblock: Int32 = (*s).nblock;
    let mut verb: Int32 = (*s).verbosity;
    let mut wfact: Int32 = (*s).workFactor;
    let mut quadrant: *mut UInt16 = 0 as *mut UInt16;
    let mut budget: Int32 = 0;
    let mut budgetInit: Int32 = 0;
    let mut i: Int32 = 0;
    if nblock < 10000 as std::os::raw::c_int {
        fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
    } else {
        /* Calculate the location for quadrant, remembering to get
         the alignment right.  Assumes that &(block[0]) is at least
         2-byte aligned -- this should be ok since block is really
         the first section of arr2.
      */
        i =
            nblock +
                (2 as std::os::raw::c_int + 12 as std::os::raw::c_int + 18 as std::os::raw::c_int +
                     2 as std::os::raw::c_int);
        if i & 1 as std::os::raw::c_int != 0 { i += 1 }
        quadrant =
            &mut *block.offset(i as isize) as *mut UChar as *mut UInt16;
        /* (wfact-1) / 3 puts the default-factor-30
         transition point at very roughly the same place as 
         with v0.1 and v0.9.0.  
         Not that it particularly matters any more, since the
         resulting compressed stream is now the same regardless
         of whether or not we use the main sort or fallback sort.
      */
        if wfact < 1 as std::os::raw::c_int { wfact = 1 as std::os::raw::c_int }
        if wfact > 100 as std::os::raw::c_int { wfact = 100 as std::os::raw::c_int }
        budgetInit = nblock * ((wfact - 1 as std::os::raw::c_int) / 3 as std::os::raw::c_int);
        budget = budgetInit;
        mainSort(ptr, block, quadrant, ftab, nblock, verb, &mut budget);
        if verb >= 3 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"      %d work, %d block, ratio %5.2f\n\x00" as *const u8
                        as *const std::os::raw::c_char, budgetInit - budget, nblock,
                    ((budgetInit - budget) as std::os::raw::c_float /
                         (if nblock == 0 as std::os::raw::c_int {
                              1 as std::os::raw::c_int
                          } else { nblock }) as std::os::raw::c_float) as
                        std::os::raw::c_double);
        }
        if budget < 0 as std::os::raw::c_int {
            if verb >= 2 as std::os::raw::c_int {
                fprintf(__stderrp,
                        b"    too repetitive; using fallback sorting algorithm\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
            }
            fallbackSort((*s).arr1, (*s).arr2, ftab, nblock, verb);
        }
    }
    (*s).origPtr = -(1 as std::os::raw::c_int);
    i = 0 as std::os::raw::c_int;
    while i < (*s).nblock {
        if *ptr.offset(i as isize) == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*s).origPtr = i;
            break ;
        } else { i += 1 }
    }
    if !((*s).origPtr != -(1 as std::os::raw::c_int)) {
        BZ2_bz__AssertH__fail(1003 as std::os::raw::c_int);
    };
}
/*-------------------------------------------------------------*/
/*--- end                                       blocksort.c ---*/
/*-------------------------------------------------------------*/
