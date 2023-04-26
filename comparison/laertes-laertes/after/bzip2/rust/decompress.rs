
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    #[no_mangle]
    static mut BZ2_rNums: [Int32; 512];
    
    
    
    
}
pub use crate::bzlib::BZ2_bz__AssertH__fail;
pub use crate::bzlib::BZ2_indexIntoF;
pub use crate::huffman::BZ2_hbCreateDecodeTables;
pub use crate::blocksort::__int64_t;
pub use crate::blocksort::__darwin_off_t;
pub use crate::blocksort::fpos_t;
// #[derive(Copy, Clone)]

pub use crate::blocksort::__sbuf;
// #[derive(Copy, Clone)]

pub use crate::blocksort::__sFILE;
pub use crate::blocksort::FILE;
// #[derive(Copy, Clone)]

pub use crate::blocksort::bz_stream;
pub use crate::blocksort::Bool;
pub use crate::blocksort::UChar;
pub use crate::blocksort::Int32;
pub use crate::blocksort::UInt32;
pub use crate::blocksort::UInt16;
// #[derive(Copy, Clone)]

pub use crate::bzlib::DState;
/*-------------------------------------------------------------*/
/*--- Decompression machinery                               ---*/
/*---                                          decompress.c ---*/
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
/*---------------------------------------------------*/
unsafe extern "C" fn makeMaps_d(mut s: *mut DState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).seqToUnseq[(*s).nInUse as usize] = i as UChar;
            (*s).nInUse += 1
        }
        i += 1
    };
}
/*---------------------------------------------------*/
/*---------------------------------------------------*/
/* the longest code */
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_decompress(mut s: *mut DState) -> Int32 {
    let mut current_block: u64;
    let mut uc: UChar = 0;
    let mut retVal: Int32 = 0;
    let mut minLen: Int32 = 0;
    let mut maxLen: Int32 = 0;
    let mut strm: *mut bz_stream = (*s).strm;
    /* stuff that needs to be saved/restored */
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut t: Int32 = 0;
    let mut alphaSize: Int32 = 0;
    let mut nGroups: Int32 = 0;
    let mut nSelectors: Int32 = 0;
    let mut EOB: Int32 = 0;
    let mut groupNo: Int32 = 0;
    let mut groupPos: Int32 = 0;
    let mut nextSym: Int32 = 0;
    let mut nblockMAX: Int32 = 0;
    let mut nblock: Int32 = 0;
    let mut es: Int32 = 0;
    let mut N: Int32 = 0;
    let mut curr: Int32 = 0;
    let mut zt: Int32 = 0;
    let mut zn: Int32 = 0;
    let mut zvec: Int32 = 0;
    let mut zj: Int32 = 0;
    let mut gSel: Int32 = 0;
    let mut gMinlen: Int32 = 0;
    let mut gLimit: *mut Int32 = core::ptr::null_mut();
    let mut gBase: *mut Int32 = core::ptr::null_mut();
    let mut gPerm: *mut Int32 = core::ptr::null_mut();
    if (*s).state == 10 as std::os::raw::c_int {
        /*initialise the save area*/
        (*s).save_i = 0 as std::os::raw::c_int;
        (*s).save_j = 0 as std::os::raw::c_int;
        (*s).save_t = 0 as std::os::raw::c_int;
        (*s).save_alphaSize = 0 as std::os::raw::c_int;
        (*s).save_nGroups = 0 as std::os::raw::c_int;
        (*s).save_nSelectors = 0 as std::os::raw::c_int;
        (*s).save_EOB = 0 as std::os::raw::c_int;
        (*s).save_groupNo = 0 as std::os::raw::c_int;
        (*s).save_groupPos = 0 as std::os::raw::c_int;
        (*s).save_nextSym = 0 as std::os::raw::c_int;
        (*s).save_nblockMAX = 0 as std::os::raw::c_int;
        (*s).save_nblock = 0 as std::os::raw::c_int;
        (*s).save_es = 0 as std::os::raw::c_int;
        (*s).save_N = 0 as std::os::raw::c_int;
        (*s).save_curr = 0 as std::os::raw::c_int;
        (*s).save_zt = 0 as std::os::raw::c_int;
        (*s).save_zn = 0 as std::os::raw::c_int;
        (*s).save_zvec = 0 as std::os::raw::c_int;
        (*s).save_zj = 0 as std::os::raw::c_int;
        (*s).save_gSel = 0 as std::os::raw::c_int;
        (*s).save_gMinlen = 0 as std::os::raw::c_int;
        (*s).save_gLimit = core::ptr::null_mut();
        (*s).save_gBase = core::ptr::null_mut();
        (*s).save_gPerm = core::ptr::null_mut()
    }
    /*restore from the save area*/
    i = (*s).save_i;
    j = (*s).save_j;
    t = (*s).save_t;
    alphaSize = (*s).save_alphaSize;
    nGroups = (*s).save_nGroups;
    nSelectors = (*s).save_nSelectors;
    EOB = (*s).save_EOB;
    groupNo = (*s).save_groupNo;
    groupPos = (*s).save_groupPos;
    nextSym = (*s).save_nextSym;
    nblockMAX = (*s).save_nblockMAX;
    nblock = (*s).save_nblock;
    es = (*s).save_es;
    N = (*s).save_N;
    curr = (*s).save_curr;
    zt = (*s).save_zt;
    zn = (*s).save_zn;
    zvec = (*s).save_zvec;
    zj = (*s).save_zj;
    gSel = (*s).save_gSel;
    gMinlen = (*s).save_gMinlen;
    gLimit = (*s).save_gLimit;
    gBase = (*s).save_gBase;
    gPerm = (*s).save_gPerm;
    retVal = 0 as std::os::raw::c_int;
    match (*s).state {
        10 => {
            (*s).state = 10 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 5658374378798827547;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v: UInt32 = 0;
                    v =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v as UChar;
                    current_block = 5658374378798827547;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x42 as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 12259750428863723923; }
                }
            }
        }
        11 => { current_block = 12259750428863723923; }
        12 => { current_block = 15146946972525368609; }
        13 => { current_block = 13504760517129887221; }
        14 => { current_block = 7948568793456312728; }
        15 => { current_block = 16703841960874917807; }
        16 => { current_block = 12231332282017165356; }
        17 => { current_block = 7286555771988341860; }
        18 => { current_block = 6656868271313165664; }
        19 => { current_block = 15902903523132075486; }
        20 => { current_block = 16204949703499709801; }
        21 => { current_block = 5505795673017046993; }
        22 => { current_block = 14563596112884461881; }
        23 => { current_block = 12051594319698232578; }
        24 => { current_block = 14315698657705028467; }
        25 => { current_block = 640681092829779800; }
        26 => { current_block = 588075840077989673; }
        27 => { current_block = 34749046854646975; }
        28 => { current_block = 16487873541482693172; }
        29 => { current_block = 1422779171932145779; }
        30 => { current_block = 3906616468301123675; }
        31 => { current_block = 5769007513321684282; }
        32 => { current_block = 4874723077730206021; }
        33 => { current_block = 10945178116989557996; }
        34 => { current_block = 1736021991379636935; }
        35 => { current_block = 5008197131544113214; }
        36 => { current_block = 16722720626876144162; }
        37 => { current_block = 14744029255125744966; }
        38 => { current_block = 5374617794059532979; }
        39 => { current_block = 13999925517074022731; }
        40 => { current_block = 2629672494974161066; }
        41 => { current_block = 1050378859040334210; }
        42 => { current_block = 10200488719709598753; }
        43 => { current_block = 9864403379770423142; }
        44 => { current_block = 8489059574810375089; }
        45 => { current_block = 12998570369541158573; }
        46 => { current_block = 10541196509243133637; }
        47 => { current_block = 8760950161942609538; }
        48 => { current_block = 3131443096645543054; }
        49 => { current_block = 1975408140333322065; }
        50 => { current_block = 15818179691129344165; }
        _ => {
            if 0 as std::os::raw::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4001 as std::os::raw::c_int);
            }
            if 0 as std::os::raw::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4002 as std::os::raw::c_int);
            }
            current_block = 15885526978618306830;
        }
    }
    match current_block {
        12259750428863723923 => {
            (*s).state = 11 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 1658462350791934405;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_0: UInt32 = 0;
                    v_0 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_0 as UChar;
                    current_block = 1658462350791934405;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x5a as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 15146946972525368609; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        15146946972525368609 => {
            (*s).state = 12 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 16314074004867283505;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_1: UInt32 = 0;
                    v_1 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_1 as UChar;
                    current_block = 16314074004867283505;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x68 as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 13504760517129887221; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        13504760517129887221 => {
            (*s).state = 13 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 1915186496383530739;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_2: UInt32 = 0;
                    v_2 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    (*s).blockSize100k = v_2 as Int32;
                    current_block = 1915186496383530739;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if (*s).blockSize100k <
                           0x30 as std::os::raw::c_int + 1 as std::os::raw::c_int ||
                           (*s).blockSize100k >
                               0x30 as std::os::raw::c_int + 9 as std::os::raw::c_int {
                        retVal = -(5 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        (*s).blockSize100k -= 0x30 as std::os::raw::c_int;
                        if (*s).smallDecompress != 0 {
                            (*s).ll16 =
                                (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                                                    (((*s).blockSize100k
                                                                                          *
                                                                                          100000
                                                                                              as
                                                                                              std::os::raw::c_int)
                                                                                         as
                                                                                         std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt16>()
                                                                                                                         as
                                                                                                                         std::os::raw::c_ulong)
                                                                                        as
                                                                                        std::os::raw::c_int,
                                                                                    1
                                                                                        as
                                                                                        std::os::raw::c_int)
                                    as *mut UInt16;
                            (*s).ll4 =
                                (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                                                    ((1
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          +
                                                                                          (*s).blockSize100k
                                                                                              *
                                                                                              100000
                                                                                                  as
                                                                                                  std::os::raw::c_int
                                                                                          >>
                                                                                          1
                                                                                              as
                                                                                              std::os::raw::c_int)
                                                                                         as
                                                                                         std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UChar>()
                                                                                                                         as
                                                                                                                         std::os::raw::c_ulong)
                                                                                        as
                                                                                        std::os::raw::c_int,
                                                                                    1
                                                                                        as
                                                                                        std::os::raw::c_int)
                                    as *mut UChar;
                            if (*s).ll16.is_null() || (*s).ll4.is_null() {
                                retVal = -(3 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                            } else { current_block = 7948568793456312728; }
                        } else {
                            (*s).tt =
                                (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                                                    (((*s).blockSize100k
                                                                                          *
                                                                                          100000
                                                                                              as
                                                                                              std::os::raw::c_int)
                                                                                         as
                                                                                         std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<Int32>()
                                                                                                                         as
                                                                                                                         std::os::raw::c_ulong)
                                                                                        as
                                                                                        std::os::raw::c_int,
                                                                                    1
                                                                                        as
                                                                                        std::os::raw::c_int)
                                    as *mut UInt32;
                            if (*s).tt.is_null() {
                                retVal = -(3 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                            } else { current_block = 7948568793456312728; }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    match current_block {
        7948568793456312728 => {
            (*s).state = 14 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 9846950269610550213;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_3: UInt32 = 0;
                    v_3 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_3 as UChar;
                    current_block = 9846950269610550213;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int == 0x17 as std::os::raw::c_int {
                        current_block = 10200488719709598753;
                    } else if uc as std::os::raw::c_int != 0x31 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 16703841960874917807; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        10200488719709598753 => {
            (*s).state = 42 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 13262463590990658200;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_32: UInt32 = 0;
                    v_32 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_32 as UChar;
                    current_block = 13262463590990658200;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x72 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 9864403379770423142; }
                }
            }
        }
        16703841960874917807 => {
            (*s).state = 15 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 3569141194949357899;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_4: UInt32 = 0;
                    v_4 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_4 as UChar;
                    current_block = 3569141194949357899;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x41 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 12231332282017165356; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        9864403379770423142 => {
            (*s).state = 43 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 10756506701594629759;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_33: UInt32 = 0;
                    v_33 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_33 as UChar;
                    current_block = 10756506701594629759;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x45 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 8489059574810375089; }
                }
            }
        }
        12231332282017165356 => {
            (*s).state = 16 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 16517180880614114163;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_5: UInt32 = 0;
                    v_5 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_5 as UChar;
                    current_block = 16517180880614114163;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x59 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 7286555771988341860; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        8489059574810375089 => {
            (*s).state = 44 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 9819403752380335018;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_34: UInt32 = 0;
                    v_34 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_34 as UChar;
                    current_block = 9819403752380335018;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x38 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 12998570369541158573; }
                }
            }
        }
        7286555771988341860 => {
            (*s).state = 17 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 2606663910910355487;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_6: UInt32 = 0;
                    v_6 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_6 as UChar;
                    current_block = 2606663910910355487;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x26 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 6656868271313165664; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        12998570369541158573 => {
            (*s).state = 45 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 9454797012561717444;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_35: UInt32 = 0;
                    v_35 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_35 as UChar;
                    current_block = 9454797012561717444;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x50 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 10541196509243133637; }
                }
            }
        }
        6656868271313165664 => {
            (*s).state = 18 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 8125779086361653720;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_7: UInt32 = 0;
                    v_7 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_7 as UChar;
                    current_block = 8125779086361653720;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x53 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else { current_block = 15902903523132075486; }
                }
            }
        }
        _ => { }
    }
    match current_block {
        10541196509243133637 => {
            (*s).state = 46 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 724777313732190959;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_36: UInt32 = 0;
                    v_36 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_36 as UChar;
                    current_block = 724777313732190959;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x90 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        (*s).storedCombinedCRC = 0 as std::os::raw::c_int as UInt32;
                        current_block = 8760950161942609538;
                    }
                }
            }
        }
        15902903523132075486 => {
            (*s).state = 19 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 958128786106592581;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_8: UInt32 = 0;
                    v_8 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_8 as UChar;
                    current_block = 958128786106592581;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    if uc as std::os::raw::c_int != 0x59 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        (*s).currBlockNo += 1;
                        if (*s).verbosity >= 2 as std::os::raw::c_int {
                            fprintf(__stderrp,
                                    b"\n    [%d: huff+mtf \x00" as *const u8
                                        as *const std::os::raw::c_char,
                                    (*s).currBlockNo);
                        }
                        (*s).storedBlockCRC = 0 as std::os::raw::c_int as UInt32;
                        current_block = 16204949703499709801;
                    }
                }
            }
        }
        _ => { }
    }
    match current_block {
        8760950161942609538 => {
            (*s).state = 47 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 14486187473704332379;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_37: UInt32 = 0;
                    v_37 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_37 as UChar;
                    current_block = 14486187473704332379;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 3131443096645543054;
                }
            }
        }
        16204949703499709801 => {
            (*s).state = 20 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 3790734079518302164;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_9: UInt32 = 0;
                    v_9 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_9 as UChar;
                    current_block = 3790734079518302164;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 5505795673017046993;
                }
            }
        }
        _ => { }
    }
    match current_block {
        3131443096645543054 => {
            (*s).state = 48 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 3659807904093622879;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_38: UInt32 = 0;
                    v_38 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_38 as UChar;
                    current_block = 3659807904093622879;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 1975408140333322065;
                }
            }
        }
        5505795673017046993 => {
            (*s).state = 21 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 16711521214030637000;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_10: UInt32 = 0;
                    v_10 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_10 as UChar;
                    current_block = 16711521214030637000;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 14563596112884461881;
                }
            }
        }
        _ => { }
    }
    match current_block {
        1975408140333322065 => {
            (*s).state = 49 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 2394045633138979148;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_39: UInt32 = 0;
                    v_39 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_39 as UChar;
                    current_block = 2394045633138979148;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 15818179691129344165;
                }
            }
        }
        14563596112884461881 => {
            (*s).state = 22 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 17870985093275900527;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_11: UInt32 = 0;
                    v_11 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_11 as UChar;
                    current_block = 17870985093275900527;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 12051594319698232578;
                }
            }
        }
        _ => { }
    }
    match current_block {
        12051594319698232578 => {
            (*s).state = 23 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 13734492969709581318;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_12: UInt32 = 0;
                    v_12 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_12 as UChar;
                    current_block = 13734492969709581318;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedBlockCRC =
                        (*s).storedBlockCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    current_block = 14315698657705028467;
                }
            }
        }
        15818179691129344165 => {
            (*s).state = 50 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 1904329045571868869;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_40: UInt32 = 0;
                    v_40 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_40 as UChar;
                    current_block = 1904329045571868869;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).storedCombinedCRC =
                        (*s).storedCombinedCRC << 8 as std::os::raw::c_int |
                            uc as UInt32;
                    (*s).state = 1 as std::os::raw::c_int;
                    retVal = 4 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                }
            }
        }
        _ => { }
    }
    match current_block {
        14315698657705028467 => {
            (*s).state = 24 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 15030729790988239748;
                    break ;
                }
                if (*s).bsLive >= 1 as std::os::raw::c_int {
                    let mut v_13: UInt32 = 0;
                    v_13 =
                        (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 1 as std::os::raw::c_int;
                    (*s).blockRandomised = v_13 as Bool;
                    current_block = 15030729790988239748;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr = 0 as std::os::raw::c_int;
                    current_block = 640681092829779800;
                }
            }
        }
        _ => { }
    }
    match current_block {
        640681092829779800 => {
            (*s).state = 25 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 8260322496947496197;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_14: UInt32 = 0;
                    v_14 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_14 as UChar;
                    current_block = 8260322496947496197;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr =
                        (*s).origPtr << 8 as std::os::raw::c_int | uc as Int32;
                    current_block = 588075840077989673;
                }
            }
        }
        _ => { }
    }
    match current_block {
        588075840077989673 => {
            (*s).state = 26 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 5561851013817067674;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_15: UInt32 = 0;
                    v_15 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_15 as UChar;
                    current_block = 5561851013817067674;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr =
                        (*s).origPtr << 8 as std::os::raw::c_int | uc as Int32;
                    current_block = 34749046854646975;
                }
            }
        }
        _ => { }
    }
    match current_block {
        34749046854646975 => {
            (*s).state = 27 as std::os::raw::c_int;
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 10471999855724930313;
                    break ;
                }
                if (*s).bsLive >= 8 as std::os::raw::c_int {
                    let mut v_16: UInt32 = 0;
                    v_16 =
                        (*s).bsBuff >> (*s).bsLive - 8 as std::os::raw::c_int &
                            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                                 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    (*s).bsLive -= 8 as std::os::raw::c_int;
                    uc = v_16 as UChar;
                    current_block = 10471999855724930313;
                    break ;
                } else if (*(*s).strm).avail_in ==
                              0 as std::os::raw::c_int as std::os::raw::c_uint {
                    retVal = 0 as std::os::raw::c_int;
                    current_block = 15885526978618306830;
                    break ;
                } else {
                    (*s).bsBuff =
                        (*s).bsBuff << 8 as std::os::raw::c_int |
                            *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive += 8 as std::os::raw::c_int;
                    (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in =
                        (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32 =
                        (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 ==
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                        (*(*s).strm).total_in_hi32 =
                            (*(*s).strm).total_in_hi32.wrapping_add(1)
                    }
                }
            }
            match current_block {
                15885526978618306830 => { }
                _ => {
                    (*s).origPtr =
                        (*s).origPtr << 8 as std::os::raw::c_int | uc as Int32;
                    if (*s).origPtr < 0 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else if (*s).origPtr >
                                  10 as std::os::raw::c_int +
                                      100000 as std::os::raw::c_int *
                                          (*s).blockSize100k {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                    } else {
                        /*--- Receive the mapping table ---*/
                        i = 0 as std::os::raw::c_int;
                        current_block = 17262312153619709241;
                    }
                }
            }
        }
        _ => { }
    }
    'c_10532:
        loop  {
            match current_block {
                15885526978618306830 => { (*s).save_i = i; break ; }
                2629672494974161066 => {
                    (*s).state = 40 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_30: UInt32 = 0;
                            v_30 =
                                (*s).bsBuff >> (*s).bsLive - zn &
                                    (((1 as std::os::raw::c_int) << zn) -
                                         1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= zn;
                            zvec = v_30 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    current_block = 9078889872071895942;
                }
                13999925517074022731 => {
                    (*s).state = 39 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_29: UInt32 = 0;
                            v_29 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            zj = v_29 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    zvec = zvec << 1 as std::os::raw::c_int | zj;
                    current_block = 13605767259572914371;
                }
                5374617794059532979 => {
                    (*s).state = 38 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_28: UInt32 = 0;
                            v_28 =
                                (*s).bsBuff >> (*s).bsLive - zn &
                                    (((1 as std::os::raw::c_int) << zn) -
                                         1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= zn;
                            zvec = v_28 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    current_block = 13605767259572914371;
                }
                14744029255125744966 => {
                    (*s).state = 37 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_27: UInt32 = 0;
                            v_27 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            zj = v_27 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    zvec = zvec << 1 as std::os::raw::c_int | zj;
                    current_block = 1550405138573481750;
                }
                16722720626876144162 => {
                    (*s).state = 36 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= zn {
                            let mut v_26: UInt32 = 0;
                            v_26 =
                                (*s).bsBuff >> (*s).bsLive - zn &
                                    (((1 as std::os::raw::c_int) << zn) -
                                         1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= zn;
                            zvec = v_26 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    current_block = 1550405138573481750;
                }
                5008197131544113214 => {
                    (*s).state = 35 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_25: UInt32 = 0;
                            v_25 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_25 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        curr += 1
                    } else { curr -= 1 }
                    current_block = 11858046780433112516;
                }
                1736021991379636935 => {
                    (*s).state = 34 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_24: UInt32 = 0;
                            v_24 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_24 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if !(uc as std::os::raw::c_int == 0 as std::os::raw::c_int) {
                        current_block = 5008197131544113214;
                        continue ;
                    }
                    current_block = 17503523010989424999;
                }
                10945178116989557996 => {
                    (*s).state = 33 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 5 as std::os::raw::c_int {
                            let mut v_23: UInt32 = 0;
                            v_23 =
                                (*s).bsBuff >> (*s).bsLive - 5 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 5 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 5 as std::os::raw::c_int;
                            curr = v_23 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    i = 0 as std::os::raw::c_int;
                    current_block = 3770765986603902964;
                }
                4874723077730206021 => {
                    (*s).state = 32 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_21: UInt32 = 0;
                            v_21 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_21 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 0 as std::os::raw::c_int {
                        current_block = 5281038271658253520;
                    } else {
                        j += 1;
                        if j >= nGroups {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else { current_block = 6927328446518169316; }
                    }
                }
                5769007513321684282 => {
                    (*s).state = 31 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 15 as std::os::raw::c_int {
                            let mut v_20: UInt32 = 0;
                            v_20 =
                                (*s).bsBuff >> (*s).bsLive - 15 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 15 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 15 as std::os::raw::c_int;
                            nSelectors = v_20 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if nSelectors < 1 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else { i = 0 as std::os::raw::c_int }
                    current_block = 6591141407893725683;
                }
                3906616468301123675 =>
                /*--- Now the selectors ---*/
                {
                    (*s).state = 30 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 3 as std::os::raw::c_int {
                            let mut v_19: UInt32 = 0;
                            v_19 =
                                (*s).bsBuff >> (*s).bsLive - 3 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 3 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 3 as std::os::raw::c_int;
                            nGroups = v_19 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if !(nGroups < 2 as std::os::raw::c_int ||
                             nGroups > 6 as std::os::raw::c_int) {
                        current_block = 5769007513321684282;
                        continue ;
                    }
                    retVal = -(4 as std::os::raw::c_int);
                    current_block = 15885526978618306830;
                    continue ;
                }
                1422779171932145779 => {
                    (*s).state = 29 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_18: UInt32 = 0;
                            v_18 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_18 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 1 as std::os::raw::c_int {
                        (*s).inUse[(i * 16 as std::os::raw::c_int + j) as usize] =
                            1 as std::os::raw::c_int as Bool
                    }
                    j += 1;
                    current_block = 3854024847017804838;
                }
                17262312153619709241 => {
                    if i < 16 as std::os::raw::c_int {
                        current_block = 16487873541482693172;
                        continue ;
                    }
                    i = 0 as std::os::raw::c_int;
                    while i < 256 as std::os::raw::c_int {
                        (*s).inUse[i as usize] = 0 as std::os::raw::c_int as Bool;
                        i += 1
                    }
                    i = 0 as std::os::raw::c_int;
                    current_block = 3472349144349095221;
                }
                16487873541482693172 => {
                    (*s).state = 28 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_17: UInt32 = 0;
                            v_17 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            uc = v_17 as UChar;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    if uc as std::os::raw::c_int == 1 as std::os::raw::c_int {
                        (*s).inUse16[i as usize] = 1 as std::os::raw::c_int as Bool
                    } else {
                        (*s).inUse16[i as usize] = 0 as std::os::raw::c_int as Bool
                    }
                    i += 1;
                    current_block = 17262312153619709241;
                    continue ;
                }
                _ => {
                    (*s).state = 41 as std::os::raw::c_int;
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if (*s).bsLive >= 1 as std::os::raw::c_int {
                            let mut v_31: UInt32 = 0;
                            v_31 =
                                (*s).bsBuff >> (*s).bsLive - 1 as std::os::raw::c_int
                                    &
                                    (((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)
                                         - 1 as std::os::raw::c_int) as std::os::raw::c_uint;
                            (*s).bsLive -= 1 as std::os::raw::c_int;
                            zj = v_31 as Int32;
                            break ;
                        } else if (*(*s).strm).avail_in ==
                                      0 as std::os::raw::c_int as std::os::raw::c_uint {
                            retVal = 0 as std::os::raw::c_int;
                            current_block = 15885526978618306830;
                            continue 'c_10532 ;
                        } else {
                            (*s).bsBuff =
                                (*s).bsBuff << 8 as std::os::raw::c_int |
                                    *((*(*s).strm).next_in as *mut UChar) as
                                        UInt32;
                            (*s).bsLive += 8 as std::os::raw::c_int;
                            (*(*s).strm).next_in =
                                (*(*s).strm).next_in.offset(1);
                            (*(*s).strm).avail_in =
                                (*(*s).strm).avail_in.wrapping_sub(1);
                            (*(*s).strm).total_in_lo32 =
                                (*(*s).strm).total_in_lo32.wrapping_add(1);
                            if (*(*s).strm).total_in_lo32 ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                (*(*s).strm).total_in_hi32 =
                                    (*(*s).strm).total_in_hi32.wrapping_add(1)
                            }
                        }
                    }
                    zvec = zvec << 1 as std::os::raw::c_int | zj;
                    current_block = 9078889872071895942;
                }
            }
            match current_block {
                9078889872071895942 => {
                    if zn > 20 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) <
                               0 as std::os::raw::c_int ||
                               zvec - *gBase.offset(zn as isize) >=
                                   258 as std::os::raw::c_int {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            nextSym =
                                *gPerm.offset((zvec -
                                                   *gBase.offset(zn as isize))
                                                  as isize)
                        }
                    } else {
                        zn += 1;
                        current_block = 1050378859040334210;
                        continue ;
                    }
                    current_block = 15093386068129942558;
                }
                13605767259572914371 => {
                    if zn > 20 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) <
                               0 as std::os::raw::c_int ||
                               zvec - *gBase.offset(zn as isize) >=
                                   258 as std::os::raw::c_int {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            nextSym =
                                *gPerm.offset((zvec -
                                                   *gBase.offset(zn as isize))
                                                  as isize);
                            if nextSym == 0 as std::os::raw::c_int ||
                                   nextSym == 1 as std::os::raw::c_int {
                                current_block = 4550729491376650574;
                            } else {
                                es += 1;
                                uc =
                                    (*s).seqToUnseq[(*s).mtfa[(*s).mtfbase[0
                                                                               as
                                                                               std::os::raw::c_int
                                                                               as
                                                                               usize]
                                                                  as usize] as
                                                        usize];
                                (*s).unzftab[uc as usize] += es;
                                if (*s).smallDecompress != 0 {
                                    while es > 0 as std::os::raw::c_int {
                                        if nblock >= nblockMAX {
                                            retVal = -(4 as std::os::raw::c_int);
                                            current_block =
                                                15885526978618306830;
                                            continue 'c_10532 ;
                                        } else {
                                            *(*s).ll16.offset(nblock as isize)
                                                = uc as UInt16;
                                            nblock += 1;
                                            es -= 1
                                        }
                                    }
                                } else {
                                    while es > 0 as std::os::raw::c_int {
                                        if nblock >= nblockMAX {
                                            retVal = -(4 as std::os::raw::c_int);
                                            current_block =
                                                15885526978618306830;
                                            continue 'c_10532 ;
                                        } else {
                                            *(*s).tt.offset(nblock as isize) =
                                                uc as UInt32;
                                            nblock += 1;
                                            es -= 1
                                        }
                                    }
                                }
                                current_block = 15093386068129942558;
                            }
                        }
                    } else {
                        zn += 1;
                        current_block = 13999925517074022731;
                        continue ;
                    }
                }
                1550405138573481750 => {
                    if zn > 20 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else if zvec <= *gLimit.offset(zn as isize) {
                        if zvec - *gBase.offset(zn as isize) <
                               0 as std::os::raw::c_int ||
                               zvec - *gBase.offset(zn as isize) >=
                                   258 as std::os::raw::c_int {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            nextSym =
                                *gPerm.offset((zvec -
                                                   *gBase.offset(zn as isize))
                                                  as isize)
                        }
                    } else {
                        zn += 1;
                        current_block = 14744029255125744966;
                        continue ;
                    }
                    current_block = 15093386068129942558;
                }
                _ => { }
            }
            match current_block {
                15093386068129942558 => {
                    if 1 as std::os::raw::c_int as Bool != 0 {
                        if nextSym == EOB {
                            current_block = 12118509005321596519;
                        } else {
                            if nextSym == 0 as std::os::raw::c_int ||
                                   nextSym == 1 as std::os::raw::c_int {
                                es = -(1 as std::os::raw::c_int);
                                N = 1 as std::os::raw::c_int
                            } else if nblock >= nblockMAX {
                                retVal = -(4 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                                continue ;
                            } else {
                                /*-- uc = MTF ( nextSym-1 ) --*/
                                let mut ii_0: Int32 = 0;
                                let mut jj_0: Int32 = 0;
                                let mut kk_0: Int32 = 0;
                                let mut pp: Int32 = 0;
                                let mut lno: Int32 = 0;
                                let mut off: Int32 = 0;
                                let mut nn: UInt32 = 0;
                                nn = (nextSym - 1 as std::os::raw::c_int) as UInt32;
                                if nn < 16 as std::os::raw::c_int as std::os::raw::c_uint {
                                    /* avoid general-case expense */
                                    pp =
                                        (*s).mtfbase[0 as std::os::raw::c_int as
                                                         usize];
                                    uc =
                                        (*s).mtfa[(pp as
                                                       std::os::raw::c_uint).wrapping_add(nn)
                                                      as usize];
                                    while nn >
                                              3 as std::os::raw::c_int as std::os::raw::c_uint
                                          {
                                        let mut z: Int32 =
                                            (pp as
                                                 std::os::raw::c_uint).wrapping_add(nn)
                                                as Int32;
                                        (*s).mtfa[z as usize] =
                                            (*s).mtfa[(z - 1 as std::os::raw::c_int)
                                                          as usize];
                                        (*s).mtfa[(z - 1 as std::os::raw::c_int) as
                                                      usize] =
                                            (*s).mtfa[(z - 2 as std::os::raw::c_int)
                                                          as usize];
                                        (*s).mtfa[(z - 2 as std::os::raw::c_int) as
                                                      usize] =
                                            (*s).mtfa[(z - 3 as std::os::raw::c_int)
                                                          as usize];
                                        (*s).mtfa[(z - 3 as std::os::raw::c_int) as
                                                      usize] =
                                            (*s).mtfa[(z - 4 as std::os::raw::c_int)
                                                          as usize];
                                        nn =
                                            (nn as
                                                 std::os::raw::c_uint).wrapping_sub(4
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_uint)
                                                as UInt32 as UInt32
                                    }
                                    while nn >
                                              0 as std::os::raw::c_int as std::os::raw::c_uint
                                          {
                                        (*s).mtfa[(pp as
                                                       std::os::raw::c_uint).wrapping_add(nn)
                                                      as usize] =
                                            (*s).mtfa[(pp as
                                                           std::os::raw::c_uint).wrapping_add(nn).wrapping_sub(1
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           std::os::raw::c_uint)
                                                          as usize];
                                        nn = nn.wrapping_sub(1)
                                    }
                                    (*s).mtfa[pp as usize] = uc
                                } else {
                                    /* general case */
                                    lno =
                                        nn.wrapping_div(16 as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                                            Int32;
                                    off =
                                        nn.wrapping_rem(16 as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                                            Int32;
                                    pp = (*s).mtfbase[lno as usize] + off;
                                    uc = (*s).mtfa[pp as usize];
                                    while pp > (*s).mtfbase[lno as usize] {
                                        (*s).mtfa[pp as usize] =
                                            (*s).mtfa[(pp - 1 as std::os::raw::c_int)
                                                          as usize];
                                        pp -= 1
                                    }
                                    (*s).mtfbase[lno as usize] += 1;
                                    while lno > 0 as std::os::raw::c_int {
                                        (*s).mtfbase[lno as usize] -= 1;
                                        (*s).mtfa[(*s).mtfbase[lno as usize]
                                                      as usize] =
                                            (*s).mtfa[((*s).mtfbase[(lno -
                                                                         1 as
                                                                             std::os::raw::c_int)
                                                                        as
                                                                        usize]
                                                           + 16 as std::os::raw::c_int
                                                           - 1 as std::os::raw::c_int)
                                                          as usize];
                                        lno -= 1
                                    }
                                    (*s).mtfbase[0 as std::os::raw::c_int as usize] -=
                                        1;
                                    (*s).mtfa[(*s).mtfbase[0 as std::os::raw::c_int as
                                                               usize] as
                                                  usize] = uc;
                                    if (*s).mtfbase[0 as std::os::raw::c_int as usize]
                                           == 0 as std::os::raw::c_int {
                                        kk_0 =
                                            4096 as std::os::raw::c_int -
                                                1 as std::os::raw::c_int;
                                        ii_0 =
                                            256 as std::os::raw::c_int /
                                                16 as std::os::raw::c_int -
                                                1 as std::os::raw::c_int;
                                        while ii_0 >= 0 as std::os::raw::c_int {
                                            jj_0 =
                                                16 as std::os::raw::c_int -
                                                    1 as std::os::raw::c_int;
                                            while jj_0 >= 0 as std::os::raw::c_int {
                                                (*s).mtfa[kk_0 as usize] =
                                                    (*s).mtfa[((*s).mtfbase[ii_0
                                                                                as
                                                                                usize]
                                                                   + jj_0) as
                                                                  usize];
                                                kk_0 -= 1;
                                                jj_0 -= 1
                                            }
                                            (*s).mtfbase[ii_0 as usize] =
                                                kk_0 + 1 as std::os::raw::c_int;
                                            ii_0 -= 1
                                        }
                                    }
                                }
                                /*-- end uc = MTF ( nextSym-1 ) --*/
                                (*s).unzftab[(*s).seqToUnseq[uc as usize] as
                                                 usize] += 1;
                                if (*s).smallDecompress != 0 {
                                    *(*s).ll16.offset(nblock as isize) =
                                        (*s).seqToUnseq[uc as usize] as UInt16
                                } else {
                                    *(*s).tt.offset(nblock as isize) =
                                        (*s).seqToUnseq[uc as usize] as UInt32
                                }
                                nblock += 1;
                                if groupPos == 0 as std::os::raw::c_int {
                                    groupNo += 1;
                                    if groupNo >= nSelectors {
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue ;
                                    } else {
                                        groupPos = 50 as std::os::raw::c_int;
                                        gSel =
                                            (*s).selector[groupNo as usize] as
                                                Int32;
                                        gMinlen = (*s).minLens[gSel as usize];
                                        gLimit =
                                            &mut *(*(*s).limit.as_mut_ptr().offset(gSel
                                                                                       as
                                                                                       isize)).as_mut_ptr().offset(0
                                                                                                                       as
                                                                                                                       std::os::raw::c_int
                                                                                                                       as
                                                                                                                       isize)
                                                as *mut Int32;
                                        gPerm =
                                            &mut *(*(*s).perm.as_mut_ptr().offset(gSel
                                                                                      as
                                                                                      isize)).as_mut_ptr().offset(0
                                                                                                                      as
                                                                                                                      std::os::raw::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                as *mut Int32;
                                        gBase =
                                            &mut *(*(*s).base.as_mut_ptr().offset(gSel
                                                                                      as
                                                                                      isize)).as_mut_ptr().offset(0
                                                                                                                      as
                                                                                                                      std::os::raw::c_int
                                                                                                                      as
                                                                                                                      isize)
                                                as *mut Int32
                                    }
                                }
                                groupPos -= 1;
                                zn = gMinlen;
                                current_block = 2629672494974161066;
                                continue ;
                            }
                            current_block = 4550729491376650574;
                        }
                    } else { current_block = 12118509005321596519; }
                    match current_block {
                        4550729491376650574 => { }
                        _ =>
                        /* Now we know what nblock is, we can do a better sanity
         check on s->origPtr.
      */
                        {
                            if (*s).origPtr < 0 as std::os::raw::c_int ||
                                   (*s).origPtr >= nblock {
                                retVal = -(4 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                                continue ;
                            } else {
                                /*-- Set up cftab to facilitate generation of T^(-1) --*/
      /* Check: unzftab entries in range. */
                                i = 0 as std::os::raw::c_int;
                                while i <= 255 as std::os::raw::c_int {
                                    if (*s).unzftab[i as usize] <
                                           0 as std::os::raw::c_int ||
                                           (*s).unzftab[i as usize] > nblock {
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue 'c_10532 ;
                                    } else { i += 1 }
                                }
                                /* Actually generate cftab. */
                                (*s).cftab[0 as std::os::raw::c_int as usize] =
                                    0 as std::os::raw::c_int;
                                i = 1 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    (*s).cftab[i as usize] =
                                        (*s).unzftab[(i - 1 as std::os::raw::c_int) as
                                                         usize];
                                    i += 1
                                }
                                i = 1 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    (*s).cftab[i as usize] +=
                                        (*s).cftab[(i - 1 as std::os::raw::c_int) as
                                                       usize];
                                    i += 1
                                }
                                /* Check: cftab entries in range. */
                                i = 0 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    if (*s).cftab[i as usize] <
                                           0 as std::os::raw::c_int ||
                                           (*s).cftab[i as usize] > nblock {
                                        /* s->cftab[i] can legitimately be == nblock */
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue 'c_10532 ;
                                    } else { i += 1 }
                                }
                                /* Check: cftab entries non-descending. */
                                i = 1 as std::os::raw::c_int;
                                while i <= 256 as std::os::raw::c_int {
                                    if (*s).cftab[(i - 1 as std::os::raw::c_int) as
                                                      usize] >
                                           (*s).cftab[i as usize] {
                                        retVal = -(4 as std::os::raw::c_int);
                                        current_block = 15885526978618306830;
                                        continue 'c_10532 ;
                                    } else { i += 1 }
                                }
                                (*s).state_out_len = 0 as std::os::raw::c_int;
                                (*s).state_out_ch = 0 as std::os::raw::c_int as UChar;
                                (*s).calculatedBlockCRC =
                                    0xffffffff as std::os::raw::c_long as UInt32;
                                (*s).state = 2 as std::os::raw::c_int;
                                if (*s).verbosity >= 2 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"rt+rld\x00" as *const u8 as
                                                *const std::os::raw::c_char);
                                }
                                if (*s).smallDecompress != 0 {
                                    /*-- Make a copy of cftab, used in generation of T --*/
                                    i = 0 as std::os::raw::c_int;
                                    while i <= 256 as std::os::raw::c_int {
                                        (*s).cftabCopy[i as usize] =
                                            (*s).cftab[i as usize];
                                        i += 1
                                    }
                                    /*-- compute the T vector --*/
                                    i = 0 as std::os::raw::c_int;
                                    while i < nblock {
                                        uc =
                                            *(*s).ll16.offset(i as isize) as
                                                UChar;
                                        *(*s).ll16.offset(i as isize) =
                                            ((*s).cftabCopy[uc as usize] &
                                                 0xffff as std::os::raw::c_int) as
                                                UInt16;
                                        if i & 0x1 as std::os::raw::c_int ==
                                               0 as std::os::raw::c_int {
                                            *(*s).ll4.offset((i >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((i >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf0 as std::os::raw::c_int |
                                                     (*s).cftabCopy[uc as
                                                                        usize]
                                                         >> 16 as std::os::raw::c_int)
                                                    as UChar
                                        } else {
                                            *(*s).ll4.offset((i >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((i >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf as std::os::raw::c_int |
                                                     ((*s).cftabCopy[uc as
                                                                         usize]
                                                          >>
                                                          16 as std::os::raw::c_int)
                                                         << 4 as std::os::raw::c_int)
                                                    as UChar
                                        }
                                        (*s).cftabCopy[uc as usize] += 1;
                                        i += 1
                                    }
                                    /*-- Compute T^(-1) by pointer reversal on T --*/
                                    i = (*s).origPtr;
                                    j =
                                        (*(*s).ll16.offset(i as isize) as
                                             UInt32 |
                                             (*(*s).ll4.offset((i >>
                                                                    1 as
                                                                        std::os::raw::c_int)
                                                                   as isize)
                                                  as UInt32 >>
                                                  (i << 2 as std::os::raw::c_int &
                                                       0x4 as std::os::raw::c_int) &
                                                  0xf as std::os::raw::c_int as
                                                      std::os::raw::c_uint) <<
                                                 16 as std::os::raw::c_int) as Int32;
                                    loop  {
                                        let mut tmp_0: Int32 =
                                            (*(*s).ll16.offset(j as isize) as
                                                 UInt32 |
                                                 (*(*s).ll4.offset((j >>
                                                                        1 as
                                                                            std::os::raw::c_int)
                                                                       as
                                                                       isize)
                                                      as UInt32 >>
                                                      (j << 2 as std::os::raw::c_int &
                                                           0x4 as std::os::raw::c_int)
                                                      &
                                                      0xf as std::os::raw::c_int as
                                                          std::os::raw::c_uint) <<
                                                     16 as std::os::raw::c_int) as
                                                Int32;
                                        *(*s).ll16.offset(j as isize) =
                                            (i & 0xffff as std::os::raw::c_int) as
                                                UInt16;
                                        if j & 0x1 as std::os::raw::c_int ==
                                               0 as std::os::raw::c_int {
                                            *(*s).ll4.offset((j >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((j >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf0 as std::os::raw::c_int |
                                                     i >> 16 as std::os::raw::c_int)
                                                    as UChar
                                        } else {
                                            *(*s).ll4.offset((j >>
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) =
                                                (*(*s).ll4.offset((j >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as std::os::raw::c_int &
                                                     0xf as std::os::raw::c_int |
                                                     (i >> 16 as std::os::raw::c_int)
                                                         << 4 as std::os::raw::c_int)
                                                    as UChar
                                        }
                                        i = j;
                                        j = tmp_0;
                                        if !(i != (*s).origPtr) { break ; }
                                    }
                                    (*s).tPos = (*s).origPtr as UInt32;
                                    (*s).nblock_used = 0 as std::os::raw::c_int;
                                    if (*s).blockRandomised != 0 {
                                        (*s).rNToGo = 0 as std::os::raw::c_int;
                                        (*s).rTPos = 0 as std::os::raw::c_int;
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).k0 =
                                            BZ2_indexIntoF((*s).tPos as Int32,
                                                           (*s).cftab.as_mut_ptr());
                                        (*s).tPos =
                                            *(*s).ll16.offset((*s).tPos as
                                                                  isize) as
                                                UInt32 |
                                                (*(*s).ll4.offset(((*s).tPos
                                                                       >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as UInt32 >>
                                                     ((*s).tPos <<
                                                          2 as std::os::raw::c_int &
                                                          0x4 as std::os::raw::c_int
                                                              as std::os::raw::c_uint)
                                                     &
                                                     0xf as std::os::raw::c_int as
                                                         std::os::raw::c_uint) <<
                                                    16 as std::os::raw::c_int;
                                        (*s).nblock_used += 1;
                                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                                            (*s).rNToGo =
                                                BZ2_rNums[(*s).rTPos as
                                                              usize];
                                            (*s).rTPos += 1;
                                            if (*s).rTPos ==
                                                   512 as std::os::raw::c_int {
                                                (*s).rTPos = 0 as std::os::raw::c_int
                                            }
                                        }
                                        (*s).rNToGo -= 1;
                                        (*s).k0 ^=
                                            if (*s).rNToGo == 1 as std::os::raw::c_int
                                               {
                                                1 as std::os::raw::c_int
                                            } else { 0 as std::os::raw::c_int }
                                    } else {
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).k0 =
                                            BZ2_indexIntoF((*s).tPos as Int32,
                                                           (*s).cftab.as_mut_ptr());
                                        (*s).tPos =
                                            *(*s).ll16.offset((*s).tPos as
                                                                  isize) as
                                                UInt32 |
                                                (*(*s).ll4.offset(((*s).tPos
                                                                       >>
                                                                       1 as
                                                                           std::os::raw::c_int)
                                                                      as
                                                                      isize)
                                                     as UInt32 >>
                                                     ((*s).tPos <<
                                                          2 as std::os::raw::c_int &
                                                          0x4 as std::os::raw::c_int
                                                              as std::os::raw::c_uint)
                                                     &
                                                     0xf as std::os::raw::c_int as
                                                         std::os::raw::c_uint) <<
                                                    16 as std::os::raw::c_int;
                                        (*s).nblock_used += 1
                                    }
                                } else {
                                    /*-- compute the T^(-1) vector --*/
                                    i = 0 as std::os::raw::c_int;
                                    while i < nblock {
                                        uc =
                                            (*(*s).tt.offset(i as isize) &
                                                 0xff as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as UChar;
                                        let ref mut fresh0 =
                                            *(*s).tt.offset((*s).cftab[uc as
                                                                           usize]
                                                                as isize);
                                        *fresh0 |=
                                            (i << 8 as std::os::raw::c_int) as
                                                std::os::raw::c_uint;
                                        (*s).cftab[uc as usize] += 1;
                                        i += 1
                                    }
                                    (*s).tPos =
                                        *(*s).tt.offset((*s).origPtr as isize)
                                            >> 8 as std::os::raw::c_int;
                                    (*s).nblock_used = 0 as std::os::raw::c_int;
                                    if (*s).blockRandomised != 0 {
                                        (*s).rNToGo = 0 as std::os::raw::c_int;
                                        (*s).rTPos = 0 as std::os::raw::c_int;
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).tPos =
                                            *(*s).tt.offset((*s).tPos as
                                                                isize);
                                        (*s).k0 =
                                            ((*s).tPos &
                                                 0xff as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as UChar as
                                                Int32;
                                        (*s).tPos >>= 8 as std::os::raw::c_int;
                                        (*s).nblock_used += 1;
                                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                                            (*s).rNToGo =
                                                BZ2_rNums[(*s).rTPos as
                                                              usize];
                                            (*s).rTPos += 1;
                                            if (*s).rTPos ==
                                                   512 as std::os::raw::c_int {
                                                (*s).rTPos = 0 as std::os::raw::c_int
                                            }
                                        }
                                        (*s).rNToGo -= 1;
                                        (*s).k0 ^=
                                            if (*s).rNToGo == 1 as std::os::raw::c_int
                                               {
                                                1 as std::os::raw::c_int
                                            } else { 0 as std::os::raw::c_int }
                                    } else {
                                        if (*s).tPos >=
                                               (100000 as std::os::raw::c_int as
                                                    UInt32).wrapping_mul((*s).blockSize100k
                                                                             as
                                                                             UInt32)
                                           {
                                            return 1 as std::os::raw::c_int as Bool as
                                                       Int32
                                        }
                                        (*s).tPos =
                                            *(*s).tt.offset((*s).tPos as
                                                                isize);
                                        (*s).k0 =
                                            ((*s).tPos &
                                                 0xff as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as UChar as
                                                Int32;
                                        (*s).tPos >>= 8 as std::os::raw::c_int;
                                        (*s).nblock_used += 1
                                    }
                                }
                                retVal = 0 as std::os::raw::c_int;
                                current_block = 15885526978618306830;
                                continue ;
                            }
                        }
                    }
                }
                _ => { }
            }
            match current_block {
                4550729491376650574 =>
                /* Check that N doesn't get too big, so that es doesn't
                  go negative.  The maximum value that can be
                  RUNA/RUNB encoded is equal to the block size (post
                  the initial RLE), viz, 900k, so bounding N at 2
                  million should guard against overflow without
                  rejecting any legitimate inputs. */
                {
                    if N >=
                           2 as std::os::raw::c_int * 1024 as std::os::raw::c_int *
                               1024 as std::os::raw::c_int {
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue ;
                    } else {
                        if nextSym == 0 as std::os::raw::c_int {
                            es =
                                es + (0 as std::os::raw::c_int + 1 as std::os::raw::c_int) * N
                        } else if nextSym == 1 as std::os::raw::c_int {
                            es =
                                es + (1 as std::os::raw::c_int + 1 as std::os::raw::c_int) * N
                        }
                        N = N * 2 as std::os::raw::c_int;
                        if groupPos == 0 as std::os::raw::c_int {
                            groupNo += 1;
                            if groupNo >= nSelectors {
                                retVal = -(4 as std::os::raw::c_int);
                                current_block = 15885526978618306830;
                                continue ;
                            } else {
                                groupPos = 50 as std::os::raw::c_int;
                                gSel =
                                    (*s).selector[groupNo as usize] as Int32;
                                gMinlen = (*s).minLens[gSel as usize];
                                gLimit =
                                    &mut *(*(*s).limit.as_mut_ptr().offset(gSel
                                                                               as
                                                                               isize)).as_mut_ptr().offset(0
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               isize)
                                        as *mut Int32;
                                gPerm =
                                    &mut *(*(*s).perm.as_mut_ptr().offset(gSel
                                                                              as
                                                                              isize)).as_mut_ptr().offset(0
                                                                                                              as
                                                                                                              std::os::raw::c_int
                                                                                                              as
                                                                                                              isize)
                                        as *mut Int32;
                                gBase =
                                    &mut *(*(*s).base.as_mut_ptr().offset(gSel
                                                                              as
                                                                              isize)).as_mut_ptr().offset(0
                                                                                                              as
                                                                                                              std::os::raw::c_int
                                                                                                              as
                                                                                                              isize)
                                        as *mut Int32
                            }
                        }
                        groupPos -= 1;
                        zn = gMinlen;
                        current_block = 5374617794059532979;
                        continue ;
                    }
                }
                _ => { }
            }
            loop  {
                match current_block {
                    3854024847017804838 => {
                        if j < 16 as std::os::raw::c_int {
                            current_block = 1422779171932145779;
                            continue 'c_10532 ;
                        }
                    }
                    6591141407893725683 => {
                        if i < nSelectors {
                            j = 0 as std::os::raw::c_int;
                            current_block = 6927328446518169316;
                            continue ;
                        } else {
                            if nSelectors >
                                   2 as std::os::raw::c_int +
                                       900000 as std::os::raw::c_int /
                                           50 as std::os::raw::c_int {
                                nSelectors =
                                    2 as std::os::raw::c_int +
                                        900000 as std::os::raw::c_int /
                                            50 as std::os::raw::c_int
                            }
                            /*--- Undo the MTF values for the selectors. ---*/
                            let mut pos: [UChar; 6] = [0; 6];
                            let mut tmp: UChar = 0;
                            let mut v_22: UChar = 0;
                            v_22 = 0 as std::os::raw::c_int as UChar;
                            while (v_22 as std::os::raw::c_int) < nGroups {
                                pos[v_22 as usize] = v_22;
                                v_22 = v_22.wrapping_add(1)
                            }
                            i = 0 as std::os::raw::c_int;
                            while i < nSelectors {
                                v_22 = (*s).selectorMtf[i as usize];
                                tmp = pos[v_22 as usize];
                                while v_22 as std::os::raw::c_int > 0 as std::os::raw::c_int {
                                    pos[v_22 as usize] =
                                        pos[(v_22 as std::os::raw::c_int -
                                                 1 as std::os::raw::c_int) as usize];
                                    v_22 = v_22.wrapping_sub(1)
                                }
                                pos[0 as std::os::raw::c_int as usize] = tmp;
                                (*s).selector[i as usize] = tmp;
                                i += 1
                            }
                            /*--- Now the coding tables ---*/
                            t = 0 as std::os::raw::c_int;
                            current_block = 16916874950763617094;
                            break ;
                        }
                    }
                    3472349144349095221 => {
                        if i < 16 as std::os::raw::c_int {
                            if (*s).inUse16[i as usize] != 0 {
                                j = 0 as std::os::raw::c_int;
                                current_block = 3854024847017804838;
                                continue ;
                            }
                        } else {
                            makeMaps_d(s);
                            if (*s).nInUse == 0 as std::os::raw::c_int {
                                current_block = 11906008669688594715;
                                break ;
                            } else {
                                current_block = 7606051654693192361;
                                break ;
                            }
                        }
                    }
                    17503523010989424999 => {
                        (*s).len[t as usize][i as usize] = curr as UChar;
                        i += 1;
                        current_block = 3770765986603902964;
                        continue ;
                    }
                    3770765986603902964 => {
                        if i < alphaSize {
                            current_block = 11858046780433112516;
                            continue ;
                        }
                        t += 1;
                        current_block = 16916874950763617094;
                        break ;
                    }
                    5281038271658253520 => {
                        /* Having more than BZ_MAX_SELECTORS doesn't make much sense
            since they will never be used, but some implementations might
            "round up" the number of selectors, so just ignore those. */
                        if i <
                               2 as std::os::raw::c_int +
                                   900000 as std::os::raw::c_int / 50 as std::os::raw::c_int {
                            (*s).selectorMtf[i as usize] = j as UChar
                        }
                        i += 1;
                        current_block = 6591141407893725683;
                        continue ;
                    }
                    6927328446518169316 => {
                        if 1 as std::os::raw::c_int as Bool != 0 {
                            current_block = 4874723077730206021;
                            continue 'c_10532 ;
                        } else {
                            current_block = 5281038271658253520;
                            continue ;
                        }
                    }
                    _ => {
                        if !(1 as std::os::raw::c_int as Bool != 0) {
                            current_block = 17503523010989424999;
                            continue ;
                        }
                        if !(curr < 1 as std::os::raw::c_int ||
                                 curr > 20 as std::os::raw::c_int) {
                            current_block = 1736021991379636935;
                            continue 'c_10532 ;
                        }
                        retVal = -(4 as std::os::raw::c_int);
                        current_block = 15885526978618306830;
                        continue 'c_10532 ;
                    }
                }
                i += 1;
                current_block = 3472349144349095221;
            }
            match current_block {
                7606051654693192361 => {
                    alphaSize = (*s).nInUse + 2 as std::os::raw::c_int;
                    current_block = 3906616468301123675;
                }
                11906008669688594715 => {
                    retVal = -(4 as std::os::raw::c_int);
                    current_block = 15885526978618306830;
                }
                _ => {
                    if t < nGroups {
                        current_block = 10945178116989557996;
                        continue ;
                    }
                    /*--- Create the Huffman decoding tables ---*/
                    t = 0 as std::os::raw::c_int;
                    while t < nGroups {
                        minLen = 32 as std::os::raw::c_int;
                        maxLen = 0 as std::os::raw::c_int;
                        i = 0 as std::os::raw::c_int;
                        while i < alphaSize {
                            if (*s).len[t as usize][i as usize] as std::os::raw::c_int
                                   > maxLen {
                                maxLen =
                                    (*s).len[t as usize][i as usize] as Int32
                            }
                            if ((*s).len[t as usize][i as usize] as
                                    std::os::raw::c_int) < minLen {
                                minLen =
                                    (*s).len[t as usize][i as usize] as Int32
                            }
                            i += 1
                        }
                        BZ2_hbCreateDecodeTables(&mut *(*(*s).limit.as_mut_ptr().offset(t
                                                                                            as
                                                                                            isize)).as_mut_ptr().offset(0
                                                                                                                            as
                                                                                                                            std::os::raw::c_int
                                                                                                                            as
                                                                                                                            isize),
                                                 &mut *(*(*s).base.as_mut_ptr().offset(t
                                                                                           as
                                                                                           isize)).as_mut_ptr().offset(0
                                                                                                                           as
                                                                                                                           std::os::raw::c_int
                                                                                                                           as
                                                                                                                           isize),
                                                 &mut *(*(*s).perm.as_mut_ptr().offset(t
                                                                                           as
                                                                                           isize)).as_mut_ptr().offset(0
                                                                                                                           as
                                                                                                                           std::os::raw::c_int
                                                                                                                           as
                                                                                                                           isize),
                                                 &mut *(*(*s).len.as_mut_ptr().offset(t
                                                                                          as
                                                                                          isize)).as_mut_ptr().offset(0
                                                                                                                          as
                                                                                                                          std::os::raw::c_int
                                                                                                                          as
                                                                                                                          isize),
                                                 minLen, maxLen, alphaSize);
                        (*s).minLens[t as usize] = minLen;
                        t += 1
                    }
                    /*--- Now the MTF values ---*/
                    EOB = (*s).nInUse + 1 as std::os::raw::c_int;
                    nblockMAX = 100000 as std::os::raw::c_int * (*s).blockSize100k;
                    groupNo = -(1 as std::os::raw::c_int);
                    groupPos = 0 as std::os::raw::c_int;
                    i = 0 as std::os::raw::c_int;
                    while i <= 255 as std::os::raw::c_int {
                        (*s).unzftab[i as usize] = 0 as std::os::raw::c_int;
                        i += 1
                    }
                    /*-- MTF init --*/
                    let mut ii: Int32 = 0;
                    let mut jj: Int32 = 0;
                    let mut kk: Int32 = 0;
                    kk = 4096 as std::os::raw::c_int - 1 as std::os::raw::c_int;
                    ii =
                        256 as std::os::raw::c_int / 16 as std::os::raw::c_int -
                            1 as std::os::raw::c_int;
                    while ii >= 0 as std::os::raw::c_int {
                        jj = 16 as std::os::raw::c_int - 1 as std::os::raw::c_int;
                        while jj >= 0 as std::os::raw::c_int {
                            (*s).mtfa[kk as usize] =
                                (ii * 16 as std::os::raw::c_int + jj) as UChar;
                            kk -= 1;
                            jj -= 1
                        }
                        (*s).mtfbase[ii as usize] = kk + 1 as std::os::raw::c_int;
                        ii -= 1
                    }
                    /*-- end MTF init --*/
                    nblock = 0 as std::os::raw::c_int;
                    if groupPos == 0 as std::os::raw::c_int {
                        groupNo += 1;
                        if groupNo >= nSelectors {
                            retVal = -(4 as std::os::raw::c_int);
                            current_block = 15885526978618306830;
                            continue ;
                        } else {
                            groupPos = 50 as std::os::raw::c_int;
                            gSel = (*s).selector[groupNo as usize] as Int32;
                            gMinlen = (*s).minLens[gSel as usize];
                            gLimit =
                                &mut *(*(*s).limit.as_mut_ptr().offset(gSel as
                                                                           isize)).as_mut_ptr().offset(0
                                                                                                           as
                                                                                                           std::os::raw::c_int
                                                                                                           as
                                                                                                           isize)
                                    as *mut Int32;
                            gPerm =
                                &mut *(*(*s).perm.as_mut_ptr().offset(gSel as
                                                                          isize)).as_mut_ptr().offset(0
                                                                                                          as
                                                                                                          std::os::raw::c_int
                                                                                                          as
                                                                                                          isize)
                                    as *mut Int32;
                            gBase =
                                &mut *(*(*s).base.as_mut_ptr().offset(gSel as
                                                                          isize)).as_mut_ptr().offset(0
                                                                                                          as
                                                                                                          std::os::raw::c_int
                                                                                                          as
                                                                                                          isize)
                                    as *mut Int32
                        }
                    }
                    groupPos -= 1;
                    zn = gMinlen;
                    current_block = 16722720626876144162;
                }
            }
        }
    (*s).save_j = j;
    (*s).save_t = t;
    (*s).save_alphaSize = alphaSize;
    (*s).save_nGroups = nGroups;
    (*s).save_nSelectors = nSelectors;
    (*s).save_EOB = EOB;
    (*s).save_groupNo = groupNo;
    (*s).save_groupPos = groupPos;
    (*s).save_nextSym = nextSym;
    (*s).save_nblockMAX = nblockMAX;
    (*s).save_nblock = nblock;
    (*s).save_es = es;
    (*s).save_N = N;
    (*s).save_curr = curr;
    (*s).save_zt = zt;
    (*s).save_zn = zn;
    (*s).save_zvec = zvec;
    (*s).save_zj = zj;
    (*s).save_gSel = gSel;
    (*s).save_gMinlen = gMinlen;
    (*s).save_gLimit = gLimit;
    (*s).save_gBase = gBase;
    (*s).save_gPerm = gPerm;
    return retVal;
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

/*-------------------------------------------------------------*/
/*--- end                                      decompress.c ---*/
/*-------------------------------------------------------------*/
