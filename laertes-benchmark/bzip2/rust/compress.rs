
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn BZ2_bz__AssertH__fail(errcode: std::os::raw::c_int);
    #[no_mangle]
    fn BZ2_blockSort(_: *mut EState);
    #[no_mangle]
    fn BZ2_hbAssignCodes(_: *mut Int32, _: *mut UChar, _: Int32, _: Int32,
                         _: Int32);
    #[no_mangle]
    fn BZ2_hbMakeCodeLengths(_: *mut UChar, _: *mut Int32, _: Int32,
                             _: Int32);
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
/*--- Compression machinery (not incl block sorting)        ---*/
/*---                                            compress.c ---*/
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
/* CHANGES
    0.9.0    -- original version.
    0.9.0a/b -- no changes in this file.
    0.9.0c   -- changed setting of nGroups in sendMTFValues() 
                so as to do a bit better on small files
*/
/*---------------------------------------------------*/
/*--- Bit stream I/O                              ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bsInitWrite(mut s: *mut EState) {
    (*s).bsLive = 0 as std::os::raw::c_int;
    (*s).bsBuff = 0 as std::os::raw::c_int as UInt32;
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsFinishWrite(mut s: *mut EState) {
    while (*s).bsLive > 0 as std::os::raw::c_int {
        *(*s).zbits.offset((*s).numZ as isize) =
            ((*s).bsBuff >> 24 as std::os::raw::c_int) as UChar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8 as std::os::raw::c_int;
        (*s).bsLive -= 8 as std::os::raw::c_int
    };
}
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[inline]
unsafe extern "C" fn bsW(mut s: *mut EState, mut n: Int32, mut v: UInt32) {
    while (*s).bsLive >= 8 as std::os::raw::c_int {
        *(*s).zbits.offset((*s).numZ as isize) =
            ((*s).bsBuff >> 24 as std::os::raw::c_int) as UChar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8 as std::os::raw::c_int;
        (*s).bsLive -= 8 as std::os::raw::c_int
    }
    (*s).bsBuff |= v << 32 as std::os::raw::c_int - (*s).bsLive - n;
    (*s).bsLive += n;
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsPutUInt32(mut s: *mut EState, mut u: UInt32) {
    bsW(s, 8 as std::os::raw::c_int,
        ((u >> 24 as std::os::raw::c_int) as std::os::raw::c_long & 0xff as std::os::raw::c_long) as
            UInt32);
    bsW(s, 8 as std::os::raw::c_int,
        ((u >> 16 as std::os::raw::c_int) as std::os::raw::c_long & 0xff as std::os::raw::c_long) as
            UInt32);
    bsW(s, 8 as std::os::raw::c_int,
        ((u >> 8 as std::os::raw::c_int) as std::os::raw::c_long & 0xff as std::os::raw::c_long) as
            UInt32);
    bsW(s, 8 as std::os::raw::c_int,
        (u as std::os::raw::c_long & 0xff as std::os::raw::c_long) as UInt32);
}
/*---------------------------------------------------*/
unsafe extern "C" fn bsPutUChar(mut s: *mut EState, mut c: UChar) {
    bsW(s, 8 as std::os::raw::c_int, c as UInt32);
}
/*---------------------------------------------------*/
/*--- The back end proper                         ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
unsafe extern "C" fn makeMaps_e(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).unseqToSeq[i as usize] = (*s).nInUse as UChar;
            (*s).nInUse += 1
        }
        i += 1
    };
}
/*---------------------------------------------------*/
unsafe extern "C" fn generateMTFValues(mut s: *mut EState) {
    let mut yy: [UChar; 256] = [0; 256];
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut zPend: Int32 = 0;
    let mut wr: Int32 = 0;
    let mut EOB: Int32 = 0;
    /* 
      After sorting (eg, here),
         s->arr1 [ 0 .. s->nblock-1 ] holds sorted order,
         and
         ((UChar*)s->arr2) [ 0 .. s->nblock-1 ] 
         holds the original block data.

      The first thing to do is generate the MTF values,
      and put them in
         ((UInt16*)s->arr1) [ 0 .. s->nblock-1 ].
      Because there are strictly fewer or equal MTF values
      than block values, ptr values in this area are overwritten
      with MTF values only when they are no longer needed.

      The final compressed bitstream is generated into the
      area starting at
         (UChar*) (&((UChar*)s->arr2)[s->nblock])

      These storage aliases are set up in bzCompressInit(),
      except for the last one, which is arranged in 
      compressBlock().
   */
    let mut ptr: *mut UInt32 = (*s).ptr;
    let mut block: *mut UChar = (*s).block;
    let mut mtfv: *mut UInt16 = (*s).mtfv;
    makeMaps_e(s);
    EOB = (*s).nInUse + 1 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i <= EOB { (*s).mtfFreq[i as usize] = 0 as std::os::raw::c_int; i += 1 }
    wr = 0 as std::os::raw::c_int;
    zPend = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < (*s).nInUse { yy[i as usize] = i as UChar; i += 1 }
    i = 0 as std::os::raw::c_int;
    while i < (*s).nblock {
        let mut ll_i: UChar = 0;
        j =
            (*ptr.offset(i as
                             isize)).wrapping_sub(1 as std::os::raw::c_int as
                                                      std::os::raw::c_uint) as Int32;
        if j < 0 as std::os::raw::c_int { j += (*s).nblock }
        ll_i = (*s).unseqToSeq[*block.offset(j as isize) as usize];
        if yy[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == ll_i as std::os::raw::c_int
           {
            zPend += 1
        } else {
            if zPend > 0 as std::os::raw::c_int {
                zPend -= 1;
                while 1 as std::os::raw::c_int as Bool != 0 {
                    if zPend & 1 as std::os::raw::c_int != 0 {
                        *mtfv.offset(wr as isize) =
                            1 as std::os::raw::c_int as UInt16;
                        wr += 1;
                        (*s).mtfFreq[1 as std::os::raw::c_int as usize] += 1
                    } else {
                        *mtfv.offset(wr as isize) =
                            0 as std::os::raw::c_int as UInt16;
                        wr += 1;
                        (*s).mtfFreq[0 as std::os::raw::c_int as usize] += 1
                    }
                    if zPend < 2 as std::os::raw::c_int { break ; }
                    zPend = (zPend - 2 as std::os::raw::c_int) / 2 as std::os::raw::c_int
                }
                zPend = 0 as std::os::raw::c_int
            }
            let mut rtmp: UChar = 0;
            let mut ryy_j: *mut UChar = 0 as *mut UChar;
            let mut rll_i: UChar = 0;
            rtmp = yy[1 as std::os::raw::c_int as usize];
            yy[1 as std::os::raw::c_int as usize] = yy[0 as std::os::raw::c_int as usize];
            ryy_j =
                &mut *yy.as_mut_ptr().offset(1 as std::os::raw::c_int as isize) as
                    *mut UChar;
            rll_i = ll_i;
            while rll_i as std::os::raw::c_int != rtmp as std::os::raw::c_int {
                let mut rtmp2: UChar = 0;
                ryy_j = ryy_j.offset(1);
                rtmp2 = rtmp;
                rtmp = *ryy_j;
                *ryy_j = rtmp2
            }
            yy[0 as std::os::raw::c_int as usize] = rtmp;
            j =
                ryy_j.offset_from(&mut *yy.as_mut_ptr().offset(0 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            isize)
                                               as *mut UChar) as std::os::raw::c_long
                    as Int32;
            *mtfv.offset(wr as isize) = (j + 1 as std::os::raw::c_int) as UInt16;
            wr += 1;
            (*s).mtfFreq[(j + 1 as std::os::raw::c_int) as usize] += 1
        }
        i += 1
    }
    if zPend > 0 as std::os::raw::c_int {
        zPend -= 1;
        while 1 as std::os::raw::c_int as Bool != 0 {
            if zPend & 1 as std::os::raw::c_int != 0 {
                *mtfv.offset(wr as isize) = 1 as std::os::raw::c_int as UInt16;
                wr += 1;
                (*s).mtfFreq[1 as std::os::raw::c_int as usize] += 1
            } else {
                *mtfv.offset(wr as isize) = 0 as std::os::raw::c_int as UInt16;
                wr += 1;
                (*s).mtfFreq[0 as std::os::raw::c_int as usize] += 1
            }
            if zPend < 2 as std::os::raw::c_int { break ; }
            zPend = (zPend - 2 as std::os::raw::c_int) / 2 as std::os::raw::c_int
        }
        zPend = 0 as std::os::raw::c_int
    }
    *mtfv.offset(wr as isize) = EOB as UInt16;
    wr += 1;
    (*s).mtfFreq[EOB as usize] += 1;
    (*s).nMTF = wr;
}
unsafe extern "C" fn sendMTFValues(mut s: *mut EState) {
    let mut v: Int32 = 0;
    let mut t: Int32 = 0;
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut gs: Int32 = 0;
    let mut ge: Int32 = 0;
    let mut totc: Int32 = 0;
    let mut bt: Int32 = 0;
    let mut bc: Int32 = 0;
    let mut iter: Int32 = 0;
    let mut nSelectors: Int32 = 0;
    let mut alphaSize: Int32 = 0;
    let mut minLen: Int32 = 0;
    let mut maxLen: Int32 = 0;
    let mut selCtr: Int32 = 0;
    let mut nGroups: Int32 = 0;
    let mut nBytes: Int32 = 0;
    /*--
   UChar  len [BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   is a global since the decoder also needs it.

   Int32  code[BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   Int32  rfreq[BZ_N_GROUPS][BZ_MAX_ALPHA_SIZE];
   are also globals only used in this proc.
   Made global to keep stack frame size small.
   --*/
    let mut cost: [UInt16; 6] = [0; 6];
    let mut fave: [Int32; 6] = [0; 6];
    let mut mtfv: *mut UInt16 = (*s).mtfv;
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"      %d in block, %d after MTF & 1-2 coding, %d+2 syms in use\n\x00"
                    as *const u8 as *const std::os::raw::c_char, (*s).nblock,
                (*s).nMTF, (*s).nInUse);
    }
    alphaSize = (*s).nInUse + 2 as std::os::raw::c_int;
    t = 0 as std::os::raw::c_int;
    while t < 6 as std::os::raw::c_int {
        v = 0 as std::os::raw::c_int;
        while v < alphaSize {
            (*s).len[t as usize][v as usize] = 15 as std::os::raw::c_int as UChar;
            v += 1
        }
        t += 1
    }
    /*--- Decide how many coding tables to use ---*/
    if !((*s).nMTF > 0 as std::os::raw::c_int) {
        BZ2_bz__AssertH__fail(3001 as std::os::raw::c_int);
    }
    if (*s).nMTF < 200 as std::os::raw::c_int {
        nGroups = 2 as std::os::raw::c_int
    } else if (*s).nMTF < 600 as std::os::raw::c_int {
        nGroups = 3 as std::os::raw::c_int
    } else if (*s).nMTF < 1200 as std::os::raw::c_int {
        nGroups = 4 as std::os::raw::c_int
    } else if (*s).nMTF < 2400 as std::os::raw::c_int {
        nGroups = 5 as std::os::raw::c_int
    } else { nGroups = 6 as std::os::raw::c_int }
    /*--- Generate an initial set of coding tables ---*/
    let mut nPart: Int32 = 0;
    let mut remF: Int32 = 0;
    let mut tFreq: Int32 = 0;
    let mut aFreq: Int32 = 0;
    nPart = nGroups;
    remF = (*s).nMTF;
    gs = 0 as std::os::raw::c_int;
    while nPart > 0 as std::os::raw::c_int {
        tFreq = remF / nPart;
        ge = gs - 1 as std::os::raw::c_int;
        aFreq = 0 as std::os::raw::c_int;
        while aFreq < tFreq && ge < alphaSize - 1 as std::os::raw::c_int {
            ge += 1;
            aFreq += (*s).mtfFreq[ge as usize]
        }
        if ge > gs && nPart != nGroups && nPart != 1 as std::os::raw::c_int &&
               (nGroups - nPart) % 2 as std::os::raw::c_int == 1 as std::os::raw::c_int {
            aFreq -= (*s).mtfFreq[ge as usize];
            ge -= 1
        }
        if (*s).verbosity >= 3 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"      initial group %d, [%d .. %d], has %d syms (%4.1f%%)\n\x00"
                        as *const u8 as *const std::os::raw::c_char, nPart, gs, ge,
                    aFreq,
                    100.0f64 * aFreq as std::os::raw::c_float as std::os::raw::c_double /
                        (*s).nMTF as std::os::raw::c_float as std::os::raw::c_double);
        }
        v = 0 as std::os::raw::c_int;
        while v < alphaSize {
            if v >= gs && v <= ge {
                (*s).len[(nPart - 1 as std::os::raw::c_int) as usize][v as usize] =
                    0 as std::os::raw::c_int as UChar
            } else {
                (*s).len[(nPart - 1 as std::os::raw::c_int) as usize][v as usize] =
                    15 as std::os::raw::c_int as UChar
            }
            v += 1
        }
        nPart -= 1;
        gs = ge + 1 as std::os::raw::c_int;
        remF -= aFreq
    }
    /*--- 
      Iterate up to BZ_N_ITERS times to improve the tables.
   ---*/
    iter = 0 as std::os::raw::c_int;
    while iter < 4 as std::os::raw::c_int {
        t = 0 as std::os::raw::c_int;
        while t < nGroups { fave[t as usize] = 0 as std::os::raw::c_int; t += 1 }
        t = 0 as std::os::raw::c_int;
        while t < nGroups {
            v = 0 as std::os::raw::c_int;
            while v < alphaSize {
                (*s).rfreq[t as usize][v as usize] = 0 as std::os::raw::c_int;
                v += 1
            }
            t += 1
        }
        /*---
        Set up an auxiliary length table which is used to fast-track
	the common case (nGroups == 6). 
      ---*/
        if nGroups == 6 as std::os::raw::c_int {
            v = 0 as std::os::raw::c_int;
            while v < alphaSize {
                (*s).len_pack[v as usize][0 as std::os::raw::c_int as usize] =
                    (((*s).len[1 as std::os::raw::c_int as usize][v as usize] as
                          std::os::raw::c_int) << 16 as std::os::raw::c_int |
                         (*s).len[0 as std::os::raw::c_int as usize][v as usize] as
                             std::os::raw::c_int) as UInt32;
                (*s).len_pack[v as usize][1 as std::os::raw::c_int as usize] =
                    (((*s).len[3 as std::os::raw::c_int as usize][v as usize] as
                          std::os::raw::c_int) << 16 as std::os::raw::c_int |
                         (*s).len[2 as std::os::raw::c_int as usize][v as usize] as
                             std::os::raw::c_int) as UInt32;
                (*s).len_pack[v as usize][2 as std::os::raw::c_int as usize] =
                    (((*s).len[5 as std::os::raw::c_int as usize][v as usize] as
                          std::os::raw::c_int) << 16 as std::os::raw::c_int |
                         (*s).len[4 as std::os::raw::c_int as usize][v as usize] as
                             std::os::raw::c_int) as UInt32;
                v += 1
            }
        }
        nSelectors = 0 as std::os::raw::c_int;
        totc = 0 as std::os::raw::c_int;
        gs = 0 as std::os::raw::c_int;
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*--- Set group start & end marks. --*/
            if gs >= (*s).nMTF { break ; }
            ge = gs + 50 as std::os::raw::c_int - 1 as std::os::raw::c_int;
            if ge >= (*s).nMTF { ge = (*s).nMTF - 1 as std::os::raw::c_int }
            /*-- 
            Calculate the cost of this group as coded
            by each of the coding tables.
         --*/
            t = 0 as std::os::raw::c_int;
            while t < nGroups {
                cost[t as usize] = 0 as std::os::raw::c_int as UInt16;
                t += 1
            }
            if nGroups == 6 as std::os::raw::c_int &&
                   50 as std::os::raw::c_int == ge - gs + 1 as std::os::raw::c_int {
                /*--- fast track the common case ---*/
                let mut cost01: UInt32 = 0;
                let mut cost23: UInt32 = 0;
                let mut cost45: UInt32 = 0;
                let mut icv: UInt16 = 0;
                cost45 = 0 as std::os::raw::c_int as UInt32;
                cost23 = cost45;
                cost01 = cost23;
                icv = *mtfv.offset((gs + 0 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 1 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 2 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 3 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 4 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 5 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 6 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 7 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 8 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 9 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 10 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 11 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 12 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 13 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 14 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 15 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 16 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 17 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 18 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 19 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 20 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 21 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 22 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 23 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 24 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 25 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 26 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 27 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 28 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 29 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 30 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 31 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 32 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 33 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 34 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 35 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 36 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 37 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 38 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 39 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 40 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 41 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 42 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 43 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 44 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 45 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 46 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 47 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 48 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 49 as std::os::raw::c_int) as isize);
                cost01 =
                    (cost01 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][0
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost23 =
                    (cost23 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost45 =
                    (cost45 as
                         std::os::raw::c_uint).wrapping_add((*s).len_pack[icv as
                                                                      usize][2
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 usize])
                        as UInt32 as UInt32;
                cost[0 as std::os::raw::c_int as usize] =
                    (cost01 & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UInt16;
                cost[1 as std::os::raw::c_int as usize] =
                    (cost01 >> 16 as std::os::raw::c_int) as UInt16;
                cost[2 as std::os::raw::c_int as usize] =
                    (cost23 & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UInt16;
                cost[3 as std::os::raw::c_int as usize] =
                    (cost23 >> 16 as std::os::raw::c_int) as UInt16;
                cost[4 as std::os::raw::c_int as usize] =
                    (cost45 & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UInt16;
                cost[5 as std::os::raw::c_int as usize] =
                    (cost45 >> 16 as std::os::raw::c_int) as UInt16
            } else {
                /*--- slow version which correctly handles all situations ---*/
                i = gs;
                while i <= ge {
                    let mut icv_0: UInt16 = *mtfv.offset(i as isize);
                    t = 0 as std::os::raw::c_int;
                    while t < nGroups {
                        cost[t as usize] =
                            (cost[t as usize] as std::os::raw::c_int +
                                 (*s).len[t as usize][icv_0 as usize] as
                                     std::os::raw::c_int) as UInt16;
                        t += 1
                    }
                    i += 1
                }
            }
            /*-- 
            Find the coding table which is best for this group,
            and record its identity in the selector table.
         --*/
            bc = 999999999 as std::os::raw::c_int;
            bt = -(1 as std::os::raw::c_int);
            t = 0 as std::os::raw::c_int;
            while t < nGroups {
                if (cost[t as usize] as std::os::raw::c_int) < bc {
                    bc = cost[t as usize] as Int32;
                    bt = t
                }
                t += 1
            }
            totc += bc;
            fave[bt as usize] += 1;
            (*s).selector[nSelectors as usize] = bt as UChar;
            nSelectors += 1;
            /*-- 
            Increment the symbol frequencies for the selected table.
          --*/
            if nGroups == 6 as std::os::raw::c_int &&
                   50 as std::os::raw::c_int == ge - gs + 1 as std::os::raw::c_int {
                /*--- fast track the common case ---*/
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 0 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 1 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 2 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 3 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 4 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 5 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 6 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 7 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 8 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 9 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 10 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 11 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 12 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 13 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 14 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 15 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 16 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 17 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 18 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 19 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 20 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 21 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 22 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 23 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 24 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 25 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 26 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 27 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 28 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 29 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 30 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 31 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 32 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 33 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 34 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 35 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 36 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 37 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 38 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 39 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 40 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 41 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 42 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 43 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 44 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 45 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 46 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 47 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 48 as std::os::raw::c_int) as
                                                       isize) as usize] += 1;
                (*s).rfreq[bt as
                               usize][*mtfv.offset((gs + 49 as std::os::raw::c_int) as
                                                       isize) as usize] += 1
            } else {
                /*--- slow version which correctly handles all situations ---*/
                i = gs;
                while i <= ge {
                    (*s).rfreq[bt as usize][*mtfv.offset(i as isize) as usize]
                        += 1;
                    i += 1
                }
            }
            gs = ge + 1 as std::os::raw::c_int
        }
        if (*s).verbosity >= 3 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"      pass %d: size is %d, grp uses are \x00" as
                        *const u8 as *const std::os::raw::c_char,
                    iter + 1 as std::os::raw::c_int, totc / 8 as std::os::raw::c_int);
            t = 0 as std::os::raw::c_int;
            while t < nGroups {
                fprintf(__stderrp,
                        b"%d \x00" as *const u8 as *const std::os::raw::c_char,
                        fave[t as usize]);
                t += 1
            }
            fprintf(__stderrp, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        /*--
        Recompute the tables based on the accumulated frequencies.
      --*/
      /* maxLen was changed from 20 to 17 in bzip2-1.0.3.  See 
         comment in huffman.c for details. */
        t = 0 as std::os::raw::c_int;
        while t < nGroups {
            BZ2_hbMakeCodeLengths(&mut *(*(*s).len.as_mut_ptr().offset(t as
                                                                           isize)).as_mut_ptr().offset(0
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           isize),
                                  &mut *(*(*s).rfreq.as_mut_ptr().offset(t as
                                                                             isize)).as_mut_ptr().offset(0
                                                                                                             as
                                                                                                             std::os::raw::c_int
                                                                                                             as
                                                                                                             isize),
                                  alphaSize, 17 as std::os::raw::c_int);
            t += 1
        }
        iter += 1
    }
    if !(nGroups < 8 as std::os::raw::c_int) {
        BZ2_bz__AssertH__fail(3002 as std::os::raw::c_int);
    }
    if !(nSelectors < 32768 as std::os::raw::c_int &&
             nSelectors <=
                 2 as std::os::raw::c_int + 900000 as std::os::raw::c_int / 50 as std::os::raw::c_int)
       {
        BZ2_bz__AssertH__fail(3003 as std::os::raw::c_int);
    }
    /*--- Compute MTF values for the selectors. ---*/
    let mut pos: [UChar; 6] = [0; 6];
    let mut ll_i: UChar = 0;
    let mut tmp2: UChar = 0;
    let mut tmp: UChar = 0;
    i = 0 as std::os::raw::c_int;
    while i < nGroups { pos[i as usize] = i as UChar; i += 1 }
    i = 0 as std::os::raw::c_int;
    while i < nSelectors {
        ll_i = (*s).selector[i as usize];
        j = 0 as std::os::raw::c_int;
        tmp = pos[j as usize];
        while ll_i as std::os::raw::c_int != tmp as std::os::raw::c_int {
            j += 1;
            tmp2 = tmp;
            tmp = pos[j as usize];
            pos[j as usize] = tmp2
        }
        pos[0 as std::os::raw::c_int as usize] = tmp;
        (*s).selectorMtf[i as usize] = j as UChar;
        i += 1
    }
    /*--- Assign actual codes for the tables. --*/
    t = 0 as std::os::raw::c_int;
    while t < nGroups {
        minLen = 32 as std::os::raw::c_int;
        maxLen = 0 as std::os::raw::c_int;
        i = 0 as std::os::raw::c_int;
        while i < alphaSize {
            if (*s).len[t as usize][i as usize] as std::os::raw::c_int > maxLen {
                maxLen = (*s).len[t as usize][i as usize] as Int32
            }
            if ((*s).len[t as usize][i as usize] as std::os::raw::c_int) < minLen {
                minLen = (*s).len[t as usize][i as usize] as Int32
            }
            i += 1
        }
        if maxLen > 17 as std::os::raw::c_int {
            BZ2_bz__AssertH__fail(3004 as std::os::raw::c_int);
        }
        if minLen < 1 as std::os::raw::c_int {
            BZ2_bz__AssertH__fail(3005 as std::os::raw::c_int);
        }
        BZ2_hbAssignCodes(&mut *(*(*s).code.as_mut_ptr().offset(t as
                                                                    isize)).as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    isize),
                          &mut *(*(*s).len.as_mut_ptr().offset(t as
                                                                   isize)).as_mut_ptr().offset(0
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                                   as
                                                                                                   isize),
                          minLen, maxLen, alphaSize);
        t += 1
    }
    /*--- Transmit the mapping table. ---*/
    let mut inUse16: [Bool; 16] = [0; 16];
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        inUse16[i as usize] = 0 as std::os::raw::c_int as Bool;
        j = 0 as std::os::raw::c_int;
        while j < 16 as std::os::raw::c_int {
            if (*s).inUse[(i * 16 as std::os::raw::c_int + j) as usize] != 0 {
                inUse16[i as usize] = 1 as std::os::raw::c_int as Bool
            }
            j += 1
        }
        i += 1
    }
    nBytes = (*s).numZ;
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        if inUse16[i as usize] != 0 {
            bsW(s, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int as UInt32);
        } else { bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32); }
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < 16 as std::os::raw::c_int {
        if inUse16[i as usize] != 0 {
            j = 0 as std::os::raw::c_int;
            while j < 16 as std::os::raw::c_int {
                if (*s).inUse[(i * 16 as std::os::raw::c_int + j) as usize] != 0 {
                    bsW(s, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int as UInt32);
                } else {
                    bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
                }
                j += 1
            }
        }
        i += 1
    }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"      bytes: mapping %d, \x00" as *const u8 as
                    *const std::os::raw::c_char, (*s).numZ - nBytes);
    }
    /*--- Now the selectors. ---*/
    nBytes = (*s).numZ;
    bsW(s, 3 as std::os::raw::c_int, nGroups as UInt32);
    bsW(s, 15 as std::os::raw::c_int, nSelectors as UInt32);
    i = 0 as std::os::raw::c_int;
    while i < nSelectors {
        j = 0 as std::os::raw::c_int;
        while j < (*s).selectorMtf[i as usize] as std::os::raw::c_int {
            bsW(s, 1 as std::os::raw::c_int, 1 as std::os::raw::c_int as UInt32);
            j += 1
        }
        bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
        i += 1
    }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"selectors %d, \x00" as *const u8 as *const std::os::raw::c_char,
                (*s).numZ - nBytes);
    }
    /*--- Now the coding tables. ---*/
    nBytes = (*s).numZ;
    t = 0 as std::os::raw::c_int;
    while t < nGroups {
        let mut curr: Int32 =
            (*s).len[t as usize][0 as std::os::raw::c_int as usize] as Int32;
        bsW(s, 5 as std::os::raw::c_int, curr as UInt32);
        i = 0 as std::os::raw::c_int;
        while i < alphaSize {
            while curr < (*s).len[t as usize][i as usize] as std::os::raw::c_int {
                bsW(s, 2 as std::os::raw::c_int, 2 as std::os::raw::c_int as UInt32);
                curr += 1
                /* 10 */
            }
            while curr > (*s).len[t as usize][i as usize] as std::os::raw::c_int {
                bsW(s, 2 as std::os::raw::c_int, 3 as std::os::raw::c_int as UInt32);
                curr -= 1
                /* 11 */
            }
            bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
            i += 1
        }
        t += 1
    }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"code lengths %d, \x00" as *const u8 as *const std::os::raw::c_char,
                (*s).numZ - nBytes);
    }
    /*--- And finally, the block data proper ---*/
    nBytes = (*s).numZ;
    selCtr = 0 as std::os::raw::c_int;
    gs = 0 as std::os::raw::c_int;
    while 1 as std::os::raw::c_int as Bool != 0 {
        if gs >= (*s).nMTF { break ; }
        ge = gs + 50 as std::os::raw::c_int - 1 as std::os::raw::c_int;
        if ge >= (*s).nMTF { ge = (*s).nMTF - 1 as std::os::raw::c_int }
        if !(((*s).selector[selCtr as usize] as std::os::raw::c_int) < nGroups) {
            BZ2_bz__AssertH__fail(3006 as std::os::raw::c_int);
        }
        if nGroups == 6 as std::os::raw::c_int &&
               50 as std::os::raw::c_int == ge - gs + 1 as std::os::raw::c_int {
            /*--- fast track the common case ---*/
            let mut mtfv_i: UInt16 = 0;
            let mut s_len_sel_selCtr: *mut UChar =
                &mut *(*(*s).len.as_mut_ptr().offset(*(*s).selector.as_mut_ptr().offset(selCtr
                                                                                            as
                                                                                            isize)
                                                         as
                                                         isize)).as_mut_ptr().offset(0
                                                                                         as
                                                                                         std::os::raw::c_int
                                                                                         as
                                                                                         isize)
                    as *mut UChar;
            let mut s_code_sel_selCtr: *mut Int32 =
                &mut *(*(*s).code.as_mut_ptr().offset(*(*s).selector.as_mut_ptr().offset(selCtr
                                                                                             as
                                                                                             isize)
                                                          as
                                                          isize)).as_mut_ptr().offset(0
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          isize)
                    as *mut Int32;
            mtfv_i = *mtfv.offset((gs + 0 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 1 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 2 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 3 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 4 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 5 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 6 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 7 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 8 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 9 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 10 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 11 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 12 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 13 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 14 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 15 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 16 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 17 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 18 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 19 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 20 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 21 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 22 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 23 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 24 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 25 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 26 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 27 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 28 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 29 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 30 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 31 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 32 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 33 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 34 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 35 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 36 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 37 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 38 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 39 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 40 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 41 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 42 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 43 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 44 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 45 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 46 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 47 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 48 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
            mtfv_i = *mtfv.offset((gs + 49 as std::os::raw::c_int) as isize);
            bsW(s, *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32);
        } else {
            /*--- slow version which correctly handles all situations ---*/
            i = gs;
            while i <= ge {
                bsW(s,
                    (*s).len[(*s).selector[selCtr as usize] as
                                 usize][*mtfv.offset(i as isize) as usize] as
                        Int32,
                    (*s).code[(*s).selector[selCtr as usize] as
                                  usize][*mtfv.offset(i as isize) as usize] as
                        UInt32);
                i += 1
            }
        }
        gs = ge + 1 as std::os::raw::c_int;
        selCtr += 1
    }
    if !(selCtr == nSelectors) { BZ2_bz__AssertH__fail(3007 as std::os::raw::c_int); }
    if (*s).verbosity >= 3 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"codes %d\n\x00" as *const u8 as *const std::os::raw::c_char,
                (*s).numZ - nBytes);
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_compressBlock(mut s: *mut EState,
                                           mut is_last_block: Bool) {
    if (*s).nblock > 0 as std::os::raw::c_int {
        (*s).blockCRC = !(*s).blockCRC;
        (*s).combinedCRC =
            (*s).combinedCRC << 1 as std::os::raw::c_int |
                (*s).combinedCRC >> 31 as std::os::raw::c_int;
        (*s).combinedCRC ^= (*s).blockCRC;
        if (*s).blockNo > 1 as std::os::raw::c_int { (*s).numZ = 0 as std::os::raw::c_int }
        if (*s).verbosity >= 2 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"    block %d: crc = 0x%08x, combined CRC = 0x%08x, size = %d\n\x00"
                        as *const u8 as *const std::os::raw::c_char, (*s).blockNo,
                    (*s).blockCRC, (*s).combinedCRC, (*s).nblock);
        }
        BZ2_blockSort(s);
    }
    (*s).zbits =
        &mut *((*s).arr2 as *mut UChar).offset((*s).nblock as isize) as
            *mut UChar;
    /*-- If this is the first block, create the stream header. --*/
    if (*s).blockNo == 1 as std::os::raw::c_int {
        BZ2_bsInitWrite(s);
        bsPutUChar(s, 0x42 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x5a as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x68 as std::os::raw::c_int as UChar);
        bsPutUChar(s, (0x30 as std::os::raw::c_int + (*s).blockSize100k) as UChar);
    }
    if (*s).nblock > 0 as std::os::raw::c_int {
        bsPutUChar(s, 0x31 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x41 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x59 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x26 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x53 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x59 as std::os::raw::c_int as UChar);
        /*-- Now the block's CRC, so it is in a known place. --*/
        bsPutUInt32(s, (*s).blockCRC);
        /*-- 
         Now a single bit indicating (non-)randomisation. 
         As of version 0.9.5, we use a better sorting algorithm
         which makes randomisation unnecessary.  So always set
         the randomised bit to 'no'.  Of course, the decoder
         still needs to be able to handle randomised blocks
         so as to maintain backwards compatibility with
         older versions of bzip2.
      --*/
        bsW(s, 1 as std::os::raw::c_int, 0 as std::os::raw::c_int as UInt32);
        bsW(s, 24 as std::os::raw::c_int, (*s).origPtr as UInt32);
        generateMTFValues(s);
        sendMTFValues(s);
    }
    /*-- If this is the last block, add the stream trailer. --*/
    if is_last_block != 0 {
        bsPutUChar(s, 0x17 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x72 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x45 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x38 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x50 as std::os::raw::c_int as UChar);
        bsPutUChar(s, 0x90 as std::os::raw::c_int as UChar);
        bsPutUInt32(s, (*s).combinedCRC);
        if (*s).verbosity >= 2 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"    final combined CRC = 0x%08x\n   \x00" as *const u8
                        as *const std::os::raw::c_char, (*s).combinedCRC);
        }
        bsFinishWrite(s);
    };
}
/*-------------------------------------------------------------*/
/*--- end                                        compress.c ---*/
/*-------------------------------------------------------------*/
