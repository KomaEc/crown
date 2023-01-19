use ::libc;
extern "C" {
    
    
    
    static mut stderr: *mut FILE;
    
    
    static mut BZ2_rNums: [Int32; 512];
    
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor8;
pub type _IO_lock_t = ();
pub type FILE = crate::src::blocksort::_IO_FILE;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor9;
pub type Bool = libc::c_uchar;
pub type UChar = libc::c_uchar;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor10;
unsafe extern "C" fn makeMaps_d(mut s: Option<&mut crate::src::bzlib::DState>) {
    let mut i: Int32 = 0;
    (*s.as_deref_mut().unwrap()).nInUse= 0 as libc::c_int;
    i= 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if (*s.as_deref().unwrap()).inUse[i as usize] != 0 {
            (*s.as_deref_mut().unwrap()).seqToUnseq[(*s.as_deref().unwrap()).nInUse as usize] = i as UChar;
            (*s.as_deref_mut().unwrap()).nInUse+= 1;
        }
        i+= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_decompress(mut s: *mut crate::src::bzlib::DState) -> Int32 {
    let mut current_block: u64;
    let mut uc: UChar = 0;
    let mut retVal: Int32 = 0;
    let mut minLen: Int32 = 0;
    let mut maxLen: Int32 = 0;
    let mut strm = (*s).strm;
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
    let mut gLimit = 0 as *mut Int32;
    let mut gBase = 0 as *mut Int32;
    let mut gPerm = 0 as *mut Int32;
    if (*s).state == 10 as libc::c_int {
        (*s).save_i= 0 as libc::c_int;
        (*s).save_j= 0 as libc::c_int;
        (*s).save_t= 0 as libc::c_int;
        (*s).save_alphaSize= 0 as libc::c_int;
        (*s).save_nGroups= 0 as libc::c_int;
        (*s).save_nSelectors= 0 as libc::c_int;
        (*s).save_EOB= 0 as libc::c_int;
        (*s).save_groupNo= 0 as libc::c_int;
        (*s).save_groupPos= 0 as libc::c_int;
        (*s).save_nextSym= 0 as libc::c_int;
        (*s).save_nblockMAX= 0 as libc::c_int;
        (*s).save_nblock= 0 as libc::c_int;
        (*s).save_es= 0 as libc::c_int;
        (*s).save_N= 0 as libc::c_int;
        (*s).save_curr= 0 as libc::c_int;
        (*s).save_zt= 0 as libc::c_int;
        (*s).save_zn= 0 as libc::c_int;
        (*s).save_zvec= 0 as libc::c_int;
        (*s).save_zj= 0 as libc::c_int;
        (*s).save_gSel= 0 as libc::c_int;
        (*s).save_gMinlen= 0 as libc::c_int;
        (*s).save_gLimit= 0 as *mut Int32;
        (*s).save_gBase= 0 as *mut Int32;
        (*s).save_gPerm= 0 as *mut Int32;
    }
    i= (*s).save_i;
    j= (*s).save_j;
    t= (*s).save_t;
    alphaSize= (*s).save_alphaSize;
    nGroups= (*s).save_nGroups;
    nSelectors= (*s).save_nSelectors;
    EOB= (*s).save_EOB;
    groupNo= (*s).save_groupNo;
    groupPos= (*s).save_groupPos;
    nextSym= (*s).save_nextSym;
    nblockMAX= (*s).save_nblockMAX;
    nblock= (*s).save_nblock;
    es= (*s).save_es;
    N= (*s).save_N;
    curr= (*s).save_curr;
    zt= (*s).save_zt;
    zn= (*s).save_zn;
    zvec= (*s).save_zvec;
    zj= (*s).save_zj;
    gSel= (*s).save_gSel;
    gMinlen= (*s).save_gMinlen;
    gLimit= (*s).save_gLimit;
    gBase= (*s).save_gBase;
    gPerm= (*s).save_gPerm;
    retVal= 0 as libc::c_int;
    match (*s).state {
        10 => {
            (*s).state= 10 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 5658374378798827547;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v: UInt32 = 0;
                    v= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v as UChar;
                    current_block= 5658374378798827547;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x42 as libc::c_int {
                        retVal= -(5 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 3006993205021687454;
                    }
                }
            }
        }
        11 => {
            current_block= 3006993205021687454;
        }
        12 => {
            current_block= 15042001179320269575;
        }
        13 => {
            current_block= 3881449305082167623;
        }
        14 => {
            current_block= 2962939589862947044;
        }
        15 => {
            current_block= 14132709226079871668;
        }
        16 => {
            current_block= 9643419754668440244;
        }
        17 => {
            current_block= 12414557862826760723;
        }
        18 => {
            current_block= 722990626647897627;
        }
        19 => {
            current_block= 12815606485179341722;
        }
        20 => {
            current_block= 12972279979481132367;
        }
        21 => {
            current_block= 541033394006102889;
        }
        22 => {
            current_block= 3813990005137401807;
        }
        23 => {
            current_block= 1753580026933325594;
        }
        24 => {
            current_block= 278081590216066828;
        }
        25 => {
            current_block= 5048411712883903721;
        }
        26 => {
            current_block= 9518779839665286147;
        }
        27 => {
            current_block= 4398027810128408265;
        }
        28 => {
            current_block= 17438419808098997991;
        }
        29 => {
            current_block= 14196862412491516613;
        }
        30 => {
            current_block= 6492128765584295426;
        }
        31 => {
            current_block= 6018809847896417576;
        }
        32 => {
            current_block= 13333503772902270734;
        }
        33 => {
            current_block= 12765707770930006169;
        }
        34 => {
            current_block= 1232411364073810230;
        }
        35 => {
            current_block= 6592809319488464358;
        }
        36 => {
            current_block= 13427822320537439643;
        }
        37 => {
            current_block= 15560064689176435631;
        }
        38 => {
            current_block= 12417384991418392802;
        }
        39 => {
            current_block= 15126558758378545666;
        }
        40 => {
            current_block= 2516300604401959109;
        }
        41 => {
            current_block= 25455516448278576;
        }
        42 => {
            current_block= 13407868514940645036;
        }
        43 => {
            current_block= 7101023504148279726;
        }
        44 => {
            current_block= 18438429198922852023;
        }
        45 => {
            current_block= 16535968932359224061;
        }
        46 => {
            current_block= 5632782989663552478;
        }
        47 => {
            current_block= 4876153394669679111;
        }
        48 => {
            current_block= 12977702551301811648;
        }
        49 => {
            current_block= 2331137395028574638;
        }
        50 => {
            current_block= 18424679783944580699;
        }
        _ => {
            if 0 as libc::c_int as Bool == 0 {
                crate::src::bzlib::BZ2_bz__AssertH__fail(4001 as libc::c_int);
            }
            if 0 as libc::c_int as Bool == 0 {
                crate::src::bzlib::BZ2_bz__AssertH__fail(4002 as libc::c_int);
            }
            current_block= 6284263362753553522;
        }
    }
    match current_block {
        3006993205021687454 => {
            (*s).state= 11 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 1658462350791934405;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_0: UInt32 = 0;
                    v_0= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_0 as UChar;
                    current_block= 1658462350791934405;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x5a as libc::c_int {
                        retVal= -(5 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 15042001179320269575;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        15042001179320269575 => {
            (*s).state= 12 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 16314074004867283505;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_1: UInt32 = 0;
                    v_1= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_1 as UChar;
                    current_block= 16314074004867283505;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x68 as libc::c_int {
                        retVal= -(5 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 3881449305082167623;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        3881449305082167623 => {
            (*s).state= 13 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 1915186496383530739;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_2: UInt32 = 0;
                    v_2= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    (*s).blockSize100k= v_2 as Int32;
                    current_block= 1915186496383530739;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if (*s).blockSize100k < 0x30 as libc::c_int + 1 as libc::c_int
                        || (*s).blockSize100k > 0x30 as libc::c_int + 9 as libc::c_int
                    {
                        retVal= -(5 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        (*s).blockSize100k-= 0x30 as libc::c_int;
                        if (*s).smallDecompress != 0 {
                            (*s).ll16= (*strm).bzalloc
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*strm).opaque,
                                (((*s).blockSize100k * 100000 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UInt16>() as libc::c_ulong,
                                    ) as libc::c_int,
                                1 as libc::c_int,
                            ) as *mut UInt16;
                            (*s).ll4= (*strm).bzalloc
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*strm).opaque,
                                ((1 as libc::c_int
                                    + (*s).blockSize100k * 100000 as libc::c_int
                                    >> 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<UChar>() as libc::c_ulong,
                                    ) as libc::c_int,
                                1 as libc::c_int,
                            ) as *mut UChar;
                            if (*s).ll16.is_null() || (*s).ll4.is_null() {
                                retVal= -(3 as libc::c_int);
                                current_block= 6284263362753553522;
                            } else {
                                current_block= 2962939589862947044;
                            }
                        } else {
                            (*s).tt= (*strm).bzalloc
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*strm).opaque,
                                (((*s).blockSize100k * 100000 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<Int32>() as libc::c_ulong,
                                    ) as libc::c_int,
                                1 as libc::c_int,
                            ) as *mut UInt32;
                            if (*s).tt.is_null() {();
                                retVal= -(3 as libc::c_int);
                                current_block= 6284263362753553522;
                            } else {
                                current_block= 2962939589862947044;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        2962939589862947044 => {
            (*s).state= 14 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 9846950269610550213;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_3: UInt32 = 0;
                    v_3= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_3 as UChar;
                    current_block= 9846950269610550213;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int == 0x17 as libc::c_int {
                        current_block= 13407868514940645036;
                    } else if uc as libc::c_int != 0x31 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 14132709226079871668;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        13407868514940645036 => {
            (*s).state= 42 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 13262463590990658200;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_32: UInt32 = 0;
                    v_32= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_32 as UChar;
                    current_block= 13262463590990658200;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x72 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 7101023504148279726;
                    }
                }
            }
        }
        14132709226079871668 => {
            (*s).state= 15 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 3569141194949357899;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_4: UInt32 = 0;
                    v_4= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_4 as UChar;
                    current_block= 3569141194949357899;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x41 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 9643419754668440244;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        7101023504148279726 => {
            (*s).state= 43 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 10756506701594629759;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_33: UInt32 = 0;
                    v_33= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_33 as UChar;
                    current_block= 10756506701594629759;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x45 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 18438429198922852023;
                    }
                }
            }
        }
        9643419754668440244 => {
            (*s).state= 16 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 16517180880614114163;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_5: UInt32 = 0;
                    v_5= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_5 as UChar;
                    current_block= 16517180880614114163;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x59 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 12414557862826760723;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        18438429198922852023 => {
            (*s).state= 44 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 9819403752380335018;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_34: UInt32 = 0;
                    v_34= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_34 as UChar;
                    current_block= 9819403752380335018;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x38 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 16535968932359224061;
                    }
                }
            }
        }
        12414557862826760723 => {
            (*s).state= 17 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 2606663910910355487;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_6: UInt32 = 0;
                    v_6= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_6 as UChar;
                    current_block= 2606663910910355487;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x26 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 722990626647897627;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        16535968932359224061 => {
            (*s).state= 45 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 9454797012561717444;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_35: UInt32 = 0;
                    v_35= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_35 as UChar;
                    current_block= 9454797012561717444;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x50 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 5632782989663552478;
                    }
                }
            }
        }
        722990626647897627 => {
            (*s).state= 18 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 8125779086361653720;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_7: UInt32 = 0;
                    v_7= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_7 as UChar;
                    current_block= 8125779086361653720;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x53 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        current_block= 12815606485179341722;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        5632782989663552478 => {
            (*s).state= 46 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 724777313732190959;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_36: UInt32 = 0;
                    v_36= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_36 as UChar;
                    current_block= 724777313732190959;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x90 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        (*s).storedCombinedCRC= 0 as libc::c_int as UInt32;
                        current_block= 4876153394669679111;
                    }
                }
            }
        }
        12815606485179341722 => {
            (*s).state= 19 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 958128786106592581;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_8: UInt32 = 0;
                    v_8= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_8 as UChar;
                    current_block= 958128786106592581;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x59 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        (*s).currBlockNo+= 1;
                        if (*s).verbosity >= 2 as libc::c_int {
                            fprintf(
                                stderr,
                                b"\n    [%d: huff+mtf \0" as *const u8
                                    as *const libc::c_char,
                                (*s).currBlockNo,
                            );
                        }
                        (*s).storedBlockCRC= 0 as libc::c_int as UInt32;
                        current_block= 12972279979481132367;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        4876153394669679111 => {
            (*s).state= 47 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 14486187473704332379;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_37: UInt32 = 0;
                    v_37= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_37 as UChar;
                    current_block= 14486187473704332379;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedCombinedCRC= (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block= 12977702551301811648;
                }
            }
        }
        12972279979481132367 => {
            (*s).state= 20 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 3790734079518302164;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_9: UInt32 = 0;
                    v_9= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_9 as UChar;
                    current_block= 3790734079518302164;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedBlockCRC= (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block= 541033394006102889;
                }
            }
        }
        _ => {}
    }
    match current_block {
        12977702551301811648 => {
            (*s).state= 48 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 3659807904093622879;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_38: UInt32 = 0;
                    v_38= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_38 as UChar;
                    current_block= 3659807904093622879;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedCombinedCRC= (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block= 2331137395028574638;
                }
            }
        }
        541033394006102889 => {
            (*s).state= 21 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 16711521214030637000;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_10: UInt32 = 0;
                    v_10= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_10 as UChar;
                    current_block= 16711521214030637000;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedBlockCRC= (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block= 3813990005137401807;
                }
            }
        }
        _ => {}
    }
    match current_block {
        2331137395028574638 => {
            (*s).state= 49 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 2394045633138979148;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_39: UInt32 = 0;
                    v_39= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_39 as UChar;
                    current_block= 2394045633138979148;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedCombinedCRC= (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block= 18424679783944580699;
                }
            }
        }
        3813990005137401807 => {
            (*s).state= 22 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 17870985093275900527;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_11: UInt32 = 0;
                    v_11= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_11 as UChar;
                    current_block= 17870985093275900527;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedBlockCRC= (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block= 1753580026933325594;
                }
            }
        }
        _ => {}
    }
    match current_block {
        1753580026933325594 => {
            (*s).state= 23 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 13734492969709581318;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_12: UInt32 = 0;
                    v_12= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_12 as UChar;
                    current_block= 13734492969709581318;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedBlockCRC= (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block= 278081590216066828;
                }
            }
        }
        18424679783944580699 => {
            (*s).state= 50 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 1904329045571868869;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_40: UInt32 = 0;
                    v_40= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_40 as UChar;
                    current_block= 1904329045571868869;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).storedCombinedCRC= (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    (*s).state= 1 as libc::c_int;
                    retVal= 4 as libc::c_int;
                    current_block= 6284263362753553522;
                }
            }
        }
        _ => {}
    }
    match current_block {
        278081590216066828 => {
            (*s).state= 24 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 15030729790988239748;
                    break;
                }
                if (*s).bsLive >= 1 as libc::c_int {
                    let mut v_13: UInt32 = 0;
                    v_13= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                        & (((1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 1 as libc::c_int;
                    (*s).blockRandomised= v_13 as Bool;
                    current_block= 15030729790988239748;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr= 0 as libc::c_int;
                    current_block= 5048411712883903721;
                }
            }
        }
        _ => {}
    }
    match current_block {
        5048411712883903721 => {
            (*s).state= 25 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 8260322496947496197;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_14: UInt32 = 0;
                    v_14= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_14 as UChar;
                    current_block= 8260322496947496197;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr= (*s).origPtr << 8 as libc::c_int | uc as Int32;
                    current_block= 9518779839665286147;
                }
            }
        }
        _ => {}
    }
    match current_block {
        9518779839665286147 => {
            (*s).state= 26 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 5561851013817067674;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_15: UInt32 = 0;
                    v_15= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_15 as UChar;
                    current_block= 5561851013817067674;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr= (*s).origPtr << 8 as libc::c_int | uc as Int32;
                    current_block= 4398027810128408265;
                }
            }
        }
        _ => {}
    }
    match current_block {
        4398027810128408265 => {
            (*s).state= 27 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 10471999855724930313;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_16: UInt32 = 0;
                    v_16= (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    (*s).bsLive-= 8 as libc::c_int;
                    uc= v_16 as UChar;
                    current_block= 10471999855724930313;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal= 0 as libc::c_int;
                    current_block= 6284263362753553522;
                    break;
                } else {
                    (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    (*s).bsLive+= 8 as libc::c_int;
                    (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                    (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                    (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr= (*s).origPtr << 8 as libc::c_int | uc as Int32;
                    if (*s).origPtr < 0 as libc::c_int {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else if (*s).origPtr
                        > 10 as libc::c_int + 100000 as libc::c_int * (*s).blockSize100k
                    {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                    } else {
                        i= 0 as libc::c_int;
                        current_block= 17262312153619709241;
                    }
                }
            }
        }
        _ => {}
    }
    'c_10057: loop {
        match current_block {
            6284263362753553522 => {
                (*s).save_i= i;
                break;
            }
            2516300604401959109 => {
                (*s).state= 40 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_30: UInt32 = 0;
                        v_30= (*s).bsBuff >> (*s).bsLive - zn
                            & (((1 as libc::c_int) << zn) - 1 as libc::c_int)
                                as libc::c_uint;
                        (*s).bsLive-= zn;
                        zvec= v_30 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                current_block= 9078889872071895942;
            }
            15126558758378545666 => {
                (*s).state= 39 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_29: UInt32 = 0;
                        v_29= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        zj= v_29 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                zvec= zvec << 1 as libc::c_int | zj;
                current_block= 13605767259572914371;
            }
            12417384991418392802 => {
                (*s).state= 38 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_28: UInt32 = 0;
                        v_28= (*s).bsBuff >> (*s).bsLive - zn
                            & (((1 as libc::c_int) << zn) - 1 as libc::c_int)
                                as libc::c_uint;
                        (*s).bsLive-= zn;
                        zvec= v_28 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                current_block= 13605767259572914371;
            }
            15560064689176435631 => {
                (*s).state= 37 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_27: UInt32 = 0;
                        v_27= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        zj= v_27 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                zvec= zvec << 1 as libc::c_int | zj;
                current_block= 1550405138573481750;
            }
            13427822320537439643 => {
                (*s).state= 36 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_26: UInt32 = 0;
                        v_26= (*s).bsBuff >> (*s).bsLive - zn
                            & (((1 as libc::c_int) << zn) - 1 as libc::c_int)
                                as libc::c_uint;
                        (*s).bsLive-= zn;
                        zvec= v_26 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                current_block= 1550405138573481750;
            }
            6592809319488464358 => {
                (*s).state= 35 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_25: UInt32 = 0;
                        v_25= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        uc= v_25 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 0 as libc::c_int {
                    curr+= 1;
                } else {
                    curr-= 1;
                }
                current_block= 11858046780433112516;
            }
            1232411364073810230 => {
                (*s).state= 34 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_24: UInt32 = 0;
                        v_24= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        uc= v_24 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                if !(uc as libc::c_int == 0 as libc::c_int) {
                    current_block= 6592809319488464358;
                    continue;
                }
                current_block= 17503523010989424999;
            }
            12765707770930006169 => {
                (*s).state= 33 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 5 as libc::c_int {
                        let mut v_23: UInt32 = 0;
                        v_23= (*s).bsBuff >> (*s).bsLive - 5 as libc::c_int
                            & (((1 as libc::c_int) << 5 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 5 as libc::c_int;
                        curr= v_23 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                i= 0 as libc::c_int;
                current_block= 3770765986603902964;
            }
            13333503772902270734 => {
                (*s).state= 32 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_21: UInt32 = 0;
                        v_21= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        uc= v_21 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 0 as libc::c_int {
                    current_block= 5281038271658253520;
                } else {
                    j+= 1;
                    if j >= nGroups {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                        continue;
                    } else {
                        current_block= 6927328446518169316;
                    }
                }
            }
            6018809847896417576 => {
                (*s).state= 31 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 15 as libc::c_int {
                        let mut v_20: UInt32 = 0;
                        v_20= (*s).bsBuff >> (*s).bsLive - 15 as libc::c_int
                            & (((1 as libc::c_int) << 15 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 15 as libc::c_int;
                        nSelectors= v_20 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                if nSelectors < 1 as libc::c_int {
                    retVal= -(4 as libc::c_int);
                    current_block= 6284263362753553522;
                    continue;
                } else {
                    i= 0 as libc::c_int;
                }
                current_block= 6591141407893725683;
            }
            6492128765584295426 => {
                (*s).state= 30 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 3 as libc::c_int {
                        let mut v_19: UInt32 = 0;
                        v_19= (*s).bsBuff >> (*s).bsLive - 3 as libc::c_int
                            & (((1 as libc::c_int) << 3 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 3 as libc::c_int;
                        nGroups= v_19 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                if !(nGroups < 2 as libc::c_int || nGroups > 6 as libc::c_int) {
                    current_block= 6018809847896417576;
                    continue;
                }
                retVal= -(4 as libc::c_int);
                current_block= 6284263362753553522;
                continue;
            }
            14196862412491516613 => {
                (*s).state= 29 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_18: UInt32 = 0;
                        v_18= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        uc= v_18 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 1 as libc::c_int {
                    (*s).inUse[(i * 16 as libc::c_int + j)
                        as usize] = 1 as libc::c_int as Bool;
                }
                j+= 1;
                current_block= 3854024847017804838;
            }
            17262312153619709241 => {
                if i < 16 as libc::c_int {
                    current_block= 17438419808098997991;
                    continue;
                }
                i= 0 as libc::c_int;
                while i < 256 as libc::c_int {
                    (*s).inUse[i as usize] = 0 as libc::c_int as Bool;
                    i+= 1;
                }
                i= 0 as libc::c_int;
                current_block= 3472349144349095221;
            }
            17438419808098997991 => {
                (*s).state= 28 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_17: UInt32 = 0;
                        v_17= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        uc= v_17 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 1 as libc::c_int {
                    (*s).inUse16[i as usize] = 1 as libc::c_int as Bool;
                } else {
                    (*s).inUse16[i as usize] = 0 as libc::c_int as Bool;
                }
                i+= 1;
                current_block= 17262312153619709241;
                continue;
            }
            _ => {
                (*s).state= 41 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_31: UInt32 = 0;
                        v_31= (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        (*s).bsLive-= 1 as libc::c_int;
                        zj= v_31 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal= 0 as libc::c_int;
                        current_block= 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s).bsBuff= (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        (*s).bsLive+= 8 as libc::c_int;
                        (*(*s).strm).next_in= (*(*s).strm).next_in.offset(1);
                        (*(*s).strm).avail_in= (*(*s).strm).avail_in.wrapping_sub(1);
                        (*(*s).strm).total_in_lo32= (*(*s).strm).total_in_lo32.wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            (*(*s).strm).total_in_hi32= (*(*s).strm).total_in_hi32.wrapping_add(1);
                        }
                    }
                }
                zvec= zvec << 1 as libc::c_int | zj;
                current_block= 9078889872071895942;
            }
        }
        match current_block {
            9078889872071895942 => {
                if zn > 20 as libc::c_int {
                    retVal= -(4 as libc::c_int);
                    current_block= 6284263362753553522;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as libc::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as libc::c_int
                    {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                        continue;
                    } else {
                        nextSym= *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                    }
                } else {
                    zn+= 1;
                    current_block= 25455516448278576;
                    continue;
                }
                current_block= 15093386068129942558;
            }
            13605767259572914371 => {
                if zn > 20 as libc::c_int {
                    retVal= -(4 as libc::c_int);
                    current_block= 6284263362753553522;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as libc::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as libc::c_int
                    {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                        continue;
                    } else {
                        nextSym= *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                        if nextSym == 0 as libc::c_int || nextSym == 1 as libc::c_int {
                            current_block= 4550729491376650574;
                        } else {
                            es+= 1;
                            uc= (*s).seqToUnseq[(*s).mtfa[(*s).mtfbase[0 as libc::c_int as usize] as usize]
                                as usize];
                            (*s).unzftab[uc as usize] += es;
                            if (*s).smallDecompress != 0 {
                                while es > 0 as libc::c_int {
                                    if nblock >= nblockMAX {
                                        retVal= -(4 as libc::c_int);
                                        current_block= 6284263362753553522;
                                        continue 'c_10057;
                                    } else {
                                        *(*s).ll16.offset(nblock as isize) = uc as UInt16;
                                        nblock+= 1;
                                        es-= 1;
                                    }
                                }
                            } else {
                                while es > 0 as libc::c_int {
                                    if nblock >= nblockMAX {
                                        retVal= -(4 as libc::c_int);
                                        current_block= 6284263362753553522;
                                        continue 'c_10057;
                                    } else {
                                        *(*s).tt.offset(nblock as isize) = uc as UInt32;
                                        nblock+= 1;
                                        es-= 1;
                                    }
                                }
                            }
                            current_block= 15093386068129942558;
                        }
                    }
                } else {
                    zn+= 1;
                    current_block= 15126558758378545666;
                    continue;
                }
            }
            1550405138573481750 => {
                if zn > 20 as libc::c_int {
                    retVal= -(4 as libc::c_int);
                    current_block= 6284263362753553522;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as libc::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as libc::c_int
                    {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                        continue;
                    } else {
                        nextSym= *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                    }
                } else {
                    zn+= 1;
                    current_block= 15560064689176435631;
                    continue;
                }
                current_block= 15093386068129942558;
            }
            _ => {}
        }
        match current_block {
            15093386068129942558 => {
                if 1 as libc::c_int as Bool != 0 {
                    if nextSym == EOB {
                        current_block= 12118509005321596519;
                    } else {
                        if nextSym == 0 as libc::c_int || nextSym == 1 as libc::c_int {
                            es= -(1 as libc::c_int);
                            N= 1 as libc::c_int;
                        } else if nblock >= nblockMAX {
                            retVal= -(4 as libc::c_int);
                            current_block= 6284263362753553522;
                            continue;
                        } else {
                            let mut ii_0: Int32 = 0;
                            let mut jj_0: Int32 = 0;
                            let mut kk_0: Int32 = 0;
                            let mut pp: Int32 = 0;
                            let mut lno: Int32 = 0;
                            let mut off: Int32 = 0;
                            let mut nn: UInt32 = 0;
                            nn= (nextSym - 1 as libc::c_int) as UInt32;
                            if nn < 16 as libc::c_int as libc::c_uint {
                                pp= (*s).mtfbase[0 as libc::c_int as usize];
                                uc= (*s).mtfa[(pp as libc::c_uint).wrapping_add(nn) as usize];
                                while nn > 3 as libc::c_int as libc::c_uint {
                                    let mut z = (pp as libc::c_uint).wrapping_add(nn) as Int32;
                                    (*s).mtfa[z
                                        as usize] = (*s).mtfa[(z - 1 as libc::c_int) as usize];
                                    (*s).mtfa[(z - 1 as libc::c_int)
                                        as usize] = (*s).mtfa[(z - 2 as libc::c_int) as usize];
                                    (*s).mtfa[(z - 2 as libc::c_int)
                                        as usize] = (*s).mtfa[(z - 3 as libc::c_int) as usize];
                                    (*s).mtfa[(z - 3 as libc::c_int)
                                        as usize] = (*s).mtfa[(z - 4 as libc::c_int) as usize];
                                    nn= (nn as libc::c_uint)
                                        .wrapping_sub(4 as libc::c_int as libc::c_uint) as UInt32
                                        as UInt32;
                                }
                                while nn > 0 as libc::c_int as libc::c_uint {
                                    (*s).mtfa[(pp as libc::c_uint).wrapping_add(nn)
                                        as usize] = (*s).mtfa[(pp as libc::c_uint)
                                        .wrapping_add(nn)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
                                    nn= nn.wrapping_sub(1);
                                }
                                (*s).mtfa[pp as usize] = uc;
                            } else {
                                lno= nn.wrapping_div(16 as libc::c_int as libc::c_uint)
                                    as Int32;
                                off= nn.wrapping_rem(16 as libc::c_int as libc::c_uint)
                                    as Int32;
                                pp= (*s).mtfbase[lno as usize] + off;
                                uc= (*s).mtfa[pp as usize];
                                while pp > (*s).mtfbase[lno as usize] {
                                    (*s).mtfa[pp
                                        as usize] = (*s).mtfa[(pp - 1 as libc::c_int) as usize];
                                    pp-= 1;
                                }
                                (*s).mtfbase[lno as usize] += 1;
                                while lno > 0 as libc::c_int {
                                    (*s).mtfbase[lno as usize] -= 1;
                                    (*s).mtfa[(*s).mtfbase[lno as usize]
                                        as usize] = (*s).mtfa[((*s).mtfbase[(lno - 1 as libc::c_int) as usize]
                                        + 16 as libc::c_int - 1 as libc::c_int) as usize];
                                    lno-= 1;
                                }
                                (*s).mtfbase[0 as libc::c_int as usize] -= 1;
                                (*s).mtfa[(*s).mtfbase[0 as libc::c_int as usize]
                                    as usize] = uc;
                                if (*s).mtfbase[0 as libc::c_int as usize]
                                    == 0 as libc::c_int
                                {
                                    kk_0= 4096 as libc::c_int - 1 as libc::c_int;
                                    ii_0= 256 as libc::c_int / 16 as libc::c_int
                                        - 1 as libc::c_int;
                                    while ii_0 >= 0 as libc::c_int {
                                        jj_0= 16 as libc::c_int - 1 as libc::c_int;
                                        while jj_0 >= 0 as libc::c_int {
                                            (*s).mtfa[kk_0
                                                as usize] = (*s).mtfa[((*s).mtfbase[ii_0 as usize] + jj_0) as usize];
                                            kk_0-= 1;
                                            jj_0-= 1;
                                        }
                                        (*s).mtfbase[ii_0 as usize] = kk_0 + 1 as libc::c_int;
                                        ii_0-= 1;
                                    }
                                }
                            }
                            (*s).unzftab[(*s).seqToUnseq[uc as usize] as usize] += 1;
                            if (*s).smallDecompress != 0 {
                                *(*s).ll16
                                    .offset(
                                        nblock as isize,
                                    ) = (*s).seqToUnseq[uc as usize] as UInt16;
                            } else {
                                *(*s).tt
                                    .offset(
                                        nblock as isize,
                                    ) = (*s).seqToUnseq[uc as usize] as UInt32;
                            }
                            nblock+= 1;
                            if groupPos == 0 as libc::c_int {
                                groupNo+= 1;
                                if groupNo >= nSelectors {
                                    retVal= -(4 as libc::c_int);
                                    current_block= 6284263362753553522;
                                    continue;
                                } else {
                                    groupPos= 50 as libc::c_int;
                                    gSel= (*s).selector[groupNo as usize] as Int32;
                                    gMinlen= (*s).minLens[gSel as usize];
                                    gLimit= &raw mut *(*(*s).limit.as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut Int32;
                                    gPerm= &raw mut *(*(*s).perm.as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut Int32;
                                    gBase= &raw mut *(*(*s).base.as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut Int32;
                                }
                            }
                            groupPos-= 1;
                            zn= gMinlen;
                            current_block= 2516300604401959109;
                            continue;
                        }
                        current_block= 4550729491376650574;
                    }
                } else {
                    current_block = 12118509005321596519;
                }
                match current_block {
                    4550729491376650574 => {}
                    _ => {
                        if (*s).origPtr < 0 as libc::c_int || (*s).origPtr >= nblock {
                            retVal= -(4 as libc::c_int);
                            current_block= 6284263362753553522;
                            continue;
                        } else {
                            i= 0 as libc::c_int;
                            while i <= 255 as libc::c_int {
                                if (*s).unzftab[i as usize] < 0 as libc::c_int
                                    || (*s).unzftab[i as usize] > nblock
                                {
                                    retVal= -(4 as libc::c_int);
                                    current_block= 6284263362753553522;
                                    continue 'c_10057;
                                } else {
                                    i+= 1;
                                }
                            }
                            (*s).cftab[0 as libc::c_int as usize] = 0 as libc::c_int;
                            i= 1 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                (*s).cftab[i
                                    as usize] = (*s).unzftab[(i - 1 as libc::c_int) as usize];
                                i+= 1;
                            }
                            i= 1 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                (*s).cftab[i as usize] += (*s).cftab[(i - 1 as libc::c_int) as usize];
                                i+= 1;
                            }
                            i= 0 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                if (*s).cftab[i as usize] < 0 as libc::c_int
                                    || (*s).cftab[i as usize] > nblock
                                {
                                    retVal= -(4 as libc::c_int);
                                    current_block= 6284263362753553522;
                                    continue 'c_10057;
                                } else {
                                    i+= 1;
                                }
                            }
                            i= 1 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                if (*s).cftab[(i - 1 as libc::c_int) as usize]
                                    > (*s).cftab[i as usize]
                                {
                                    retVal= -(4 as libc::c_int);
                                    current_block= 6284263362753553522;
                                    continue 'c_10057;
                                } else {
                                    i+= 1;
                                }
                            }
                            (*s).state_out_len= 0 as libc::c_int;
                            (*s).state_out_ch= 0 as libc::c_int as UChar;
                            (*s).calculatedBlockCRC= 0xffffffff as libc::c_long as UInt32;
                            (*s).state= 2 as libc::c_int;
                            if (*s).verbosity >= 2 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"rt+rld\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if (*s).smallDecompress != 0 {
                                i= 0 as libc::c_int;
                                while i <= 256 as libc::c_int {
                                    (*s).cftabCopy[i as usize] = (*s).cftab[i as usize];
                                    i+= 1;
                                }
                                i= 0 as libc::c_int;
                                while i < nblock {
                                    uc= *(*s).ll16.offset(i as isize) as UChar;
                                    *(*s).ll16
                                        .offset(
                                            i as isize,
                                        ) = ((*s).cftabCopy[uc as usize] & 0xffff as libc::c_int)
                                        as UInt16;
                                    if i & 0x1 as libc::c_int == 0 as libc::c_int {
                                        *(*s).ll4
                                            .offset(
                                                (i >> 1 as libc::c_int) as isize,
                                            ) = (*(*s).ll4.offset((i >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf0 as libc::c_int
                                            | (*s).cftabCopy[uc as usize] >> 16 as libc::c_int)
                                            as UChar;
                                    } else {
                                        *(*s).ll4
                                            .offset(
                                                (i >> 1 as libc::c_int) as isize,
                                            ) = (*(*s).ll4.offset((i >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf as libc::c_int
                                            | ((*s).cftabCopy[uc as usize] >> 16 as libc::c_int)
                                                << 4 as libc::c_int) as UChar;
                                    }
                                    (*s).cftabCopy[uc as usize] += 1;
                                    i+= 1;
                                }
                                i= (*s).origPtr;
                                j= (*(*s).ll16.offset(i as isize) as UInt32
                                    | (*(*s).ll4.offset((i >> 1 as libc::c_int) as isize)
                                        as UInt32 >> (i << 2 as libc::c_int & 0x4 as libc::c_int)
                                        & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int)
                                    as Int32;
                                loop {
                                    let mut tmp_0 = (*(*s).ll16.offset(j as isize) as UInt32
                                        | (*(*s).ll4.offset((j >> 1 as libc::c_int) as isize)
                                            as UInt32 >> (j << 2 as libc::c_int & 0x4 as libc::c_int)
                                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int)
                                        as Int32;
                                    *(*s).ll16
                                        .offset(j as isize) = (i & 0xffff as libc::c_int) as UInt16;
                                    if j & 0x1 as libc::c_int == 0 as libc::c_int {
                                        *(*s).ll4
                                            .offset(
                                                (j >> 1 as libc::c_int) as isize,
                                            ) = (*(*s).ll4.offset((j >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf0 as libc::c_int
                                            | i >> 16 as libc::c_int) as UChar;
                                    } else {
                                        *(*s).ll4
                                            .offset(
                                                (j >> 1 as libc::c_int) as isize,
                                            ) = (*(*s).ll4.offset((j >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf as libc::c_int
                                            | (i >> 16 as libc::c_int) << 4 as libc::c_int) as UChar;
                                    }
                                    i= j;
                                    j= tmp_0;
                                    if !(i != (*s).origPtr) {
                                        break;
                                    }
                                }
                                (*s).tPos= (*s).origPtr as UInt32;
                                (*s).nblock_used= 0 as libc::c_int;
                                if (*s).blockRandomised != 0 {
                                    (*s).rNToGo= 0 as libc::c_int;
                                    (*s).rTPos= 0 as libc::c_int;
                                    if (*s).tPos
                                        >= (100000 as libc::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as libc::c_int as Bool as Int32;
                                    }
                                    (*s).k0= crate::src::bzlib::BZ2_indexIntoF(
                                        (*s).tPos as Int32,
                                        (*s).cftab.as_mut_ptr(),
                                    );
                                    (*s).tPos= *(*s).ll16.offset((*s).tPos as isize) as UInt32
                                        | (*(*s).ll4
                                            .offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                                            >> ((*s).tPos << 2 as libc::c_int
                                                & 0x4 as libc::c_int as libc::c_uint)
                                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                                    (*s).nblock_used+= 1;
                                    if (*s).rNToGo == 0 as libc::c_int {
                                        (*s).rNToGo= BZ2_rNums[(*s).rTPos as usize];
                                        (*s).rTPos+= 1;
                                        if (*s).rTPos == 512 as libc::c_int {
                                            (*s).rTPos= 0 as libc::c_int;
                                        }
                                    }
                                    (*s).rNToGo-= 1;
                                    (*s).k0^= if (*s).rNToGo == 1 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        };
                                } else {
                                    if (*s).tPos
                                        >= (100000 as libc::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as libc::c_int as Bool as Int32;
                                    }
                                    (*s).k0= crate::src::bzlib::BZ2_indexIntoF(
                                        (*s).tPos as Int32,
                                        (*s).cftab.as_mut_ptr(),
                                    );
                                    (*s).tPos= *(*s).ll16.offset((*s).tPos as isize) as UInt32
                                        | (*(*s).ll4
                                            .offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                                            >> ((*s).tPos << 2 as libc::c_int
                                                & 0x4 as libc::c_int as libc::c_uint)
                                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                                    (*s).nblock_used+= 1;
                                }
                            } else {
                                i= 0 as libc::c_int;
                                while i < nblock {
                                    uc= (*(*s).tt.offset(i as isize)
                                        & 0xff as libc::c_int as libc::c_uint) as UChar;
                                    *(*s).tt
                                        .offset((*s).cftab[uc as usize] as isize) = (i << 8 as libc::c_int) as libc::c_uint;
                                    (*s).cftab[uc as usize] += 1;
                                    i+= 1;
                                }
                                (*s).tPos= *(*s).tt.offset((*s).origPtr as isize)
                                    >> 8 as libc::c_int;
                                (*s).nblock_used= 0 as libc::c_int;
                                if (*s).blockRandomised != 0 {
                                    (*s).rNToGo= 0 as libc::c_int;
                                    (*s).rTPos= 0 as libc::c_int;
                                    if (*s).tPos
                                        >= (100000 as libc::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as libc::c_int as Bool as Int32;
                                    }
                                    (*s).tPos= *(*s).tt.offset((*s).tPos as isize);
                                    (*s).k0= ((*s).tPos & 0xff as libc::c_int as libc::c_uint)
                                        as UChar as Int32;
                                    (*s).tPos>>= 8 as libc::c_int;
                                    (*s).nblock_used+= 1;
                                    if (*s).rNToGo == 0 as libc::c_int {
                                        (*s).rNToGo= BZ2_rNums[(*s).rTPos as usize];
                                        (*s).rTPos+= 1;
                                        if (*s).rTPos == 512 as libc::c_int {
                                            (*s).rTPos= 0 as libc::c_int;
                                        }
                                    }
                                    (*s).rNToGo-= 1;
                                    (*s).k0^= if (*s).rNToGo == 1 as libc::c_int {
                                            1 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        };
                                } else {
                                    if (*s).tPos
                                        >= (100000 as libc::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as libc::c_int as Bool as Int32;
                                    }
                                    (*s).tPos= *(*s).tt.offset((*s).tPos as isize);
                                    (*s).k0= ((*s).tPos & 0xff as libc::c_int as libc::c_uint)
                                        as UChar as Int32;
                                    (*s).tPos>>= 8 as libc::c_int;
                                    (*s).nblock_used+= 1;
                                }
                            }
                            retVal= 0 as libc::c_int;
                            current_block= 6284263362753553522;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            4550729491376650574 => {
                if N >= 2 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int {
                    retVal= -(4 as libc::c_int);
                    current_block= 6284263362753553522;
                    continue;
                } else {
                    if nextSym == 0 as libc::c_int {
                        es= es + (0 as libc::c_int + 1 as libc::c_int) * N;
                    } else if nextSym == 1 as libc::c_int {
                        es= es + (1 as libc::c_int + 1 as libc::c_int) * N;
                    }
                    N= N * 2 as libc::c_int;
                    if groupPos == 0 as libc::c_int {
                        groupNo+= 1;
                        if groupNo >= nSelectors {
                            retVal= -(4 as libc::c_int);
                            current_block= 6284263362753553522;
                            continue;
                        } else {
                            groupPos= 50 as libc::c_int;
                            gSel= (*s).selector[groupNo as usize] as Int32;
                            gMinlen= (*s).minLens[gSel as usize];
                            gLimit= &raw mut *(*(*s).limit.as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut Int32;
                            gPerm= &raw mut *(*(*s).perm.as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut Int32;
                            gBase= &raw mut *(*(*s).base.as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut Int32;
                        }
                    }
                    groupPos-= 1;
                    zn= gMinlen;
                    current_block= 12417384991418392802;
                    continue;
                }
            }
            _ => {}
        }
        loop {
            match current_block {
                3854024847017804838 => {
                    if j < 16 as libc::c_int {
                        current_block= 14196862412491516613;
                        continue 'c_10057;
                    }
                }
                6591141407893725683 => {
                    if i < nSelectors {
                        j= 0 as libc::c_int;
                        current_block= 6927328446518169316;
                        continue;
                    } else {
                        if nSelectors
                            > 2 as libc::c_int
                                + 900000 as libc::c_int / 50 as libc::c_int
                        {
                            nSelectors= 2 as libc::c_int
                                + 900000 as libc::c_int / 50 as libc::c_int;
                        }
                        let mut pos: [UChar; 6] = [0; 6];
                        let mut tmp: UChar = 0;
                        let mut v_22: UChar = 0;
                        v_22= 0 as libc::c_int as UChar;
                        while (v_22 as libc::c_int) < nGroups {
                            pos[v_22 as usize] = v_22;
                            v_22= v_22.wrapping_add(1);
                        }
                        i= 0 as libc::c_int;
                        while i < nSelectors {
                            v_22= (*s).selectorMtf[i as usize];
                            tmp= pos[v_22 as usize];
                            while v_22 as libc::c_int > 0 as libc::c_int {
                                pos[v_22
                                    as usize] = pos[(v_22 as libc::c_int - 1 as libc::c_int)
                                    as usize];
                                v_22= v_22.wrapping_sub(1);
                            }
                            pos[0 as libc::c_int as usize] = tmp;
                            (*s).selector[i as usize] = tmp;
                            i+= 1;
                        }
                        t= 0 as libc::c_int;
                        current_block= 16916874950763617094;
                        break;
                    }
                }
                3472349144349095221 => {
                    if i < 16 as libc::c_int {
                        if (*s).inUse16[i as usize] != 0 {
                            j= 0 as libc::c_int;
                            current_block= 3854024847017804838;
                            continue;
                        }
                    } else {
                        makeMaps_d(s.as_mut());
                        if (*s).nInUse == 0 as libc::c_int {
                            current_block= 11906008669688594715;
                            break;
                        } else {
                            current_block= 7606051654693192361;
                            break;
                        }
                    }
                }
                17503523010989424999 => {
                    (*s).len[t as usize][i as usize] = curr as UChar;
                    i+= 1;
                    current_block= 3770765986603902964;
                    continue;
                }
                3770765986603902964 => {
                    if i < alphaSize {
                        current_block= 11858046780433112516;
                        continue;
                    }
                    t+= 1;
                    current_block= 16916874950763617094;
                    break;
                }
                5281038271658253520 => {
                    if i < 2 as libc::c_int + 900000 as libc::c_int / 50 as libc::c_int {
                        (*s).selectorMtf[i as usize] = j as UChar;
                    }
                    i+= 1;
                    current_block= 6591141407893725683;
                    continue;
                }
                6927328446518169316 => {
                    if 1 as libc::c_int as Bool != 0 {
                        current_block= 13333503772902270734;
                        continue 'c_10057;
                    } else {
                        current_block = 5281038271658253520;
                        continue;
                    }
                }
                _ => {
                    if !(1 as libc::c_int as Bool != 0) {
                        current_block = 17503523010989424999;
                        continue;
                    }
                    if !(curr < 1 as libc::c_int || curr > 20 as libc::c_int) {
                        current_block= 1232411364073810230;
                        continue 'c_10057;
                    }
                    retVal= -(4 as libc::c_int);
                    current_block= 6284263362753553522;
                    continue 'c_10057;
                }
            }
            i+= 1;
            current_block= 3472349144349095221;
        }
        match current_block {
            7606051654693192361 => {
                alphaSize= (*s).nInUse + 2 as libc::c_int;
                current_block= 6492128765584295426;
            }
            11906008669688594715 => {
                retVal= -(4 as libc::c_int);
                current_block= 6284263362753553522;
            }
            _ => {
                if t < nGroups {
                    current_block= 12765707770930006169;
                    continue;
                }
                t= 0 as libc::c_int;
                while t < nGroups {
                    minLen= 32 as libc::c_int;
                    maxLen= 0 as libc::c_int;
                    i= 0 as libc::c_int;
                    while i < alphaSize {
                        if (*s).len[t as usize][i as usize] as libc::c_int > maxLen {
                            maxLen= (*s).len[t as usize][i as usize] as Int32;
                        }
                        if ((*s).len[t as usize][i as usize] as libc::c_int) < minLen {
                            minLen= (*s).len[t as usize][i as usize] as Int32;
                        }
                        i+= 1;
                    }
                    crate::src::huffman::BZ2_hbCreateDecodeTables(
                        &raw mut *(*(*s).limit.as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        &raw mut *(*(*s).base.as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        &raw mut *(*(*s).perm.as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        &raw mut *(*(*s).len.as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        minLen,
                        maxLen,
                        alphaSize,
                    );
                    (*s).minLens[t as usize] = minLen;
                    t+= 1;
                }
                EOB= (*s).nInUse + 1 as libc::c_int;
                nblockMAX= 100000 as libc::c_int * (*s).blockSize100k;
                groupNo= -(1 as libc::c_int);
                groupPos= 0 as libc::c_int;
                i= 0 as libc::c_int;
                while i <= 255 as libc::c_int {
                    (*s).unzftab[i as usize] = 0 as libc::c_int;
                    i+= 1;
                }
                let mut ii: Int32 = 0;
                let mut jj: Int32 = 0;
                let mut kk: Int32 = 0;
                kk= 4096 as libc::c_int - 1 as libc::c_int;
                ii= 256 as libc::c_int / 16 as libc::c_int - 1 as libc::c_int;
                while ii >= 0 as libc::c_int {
                    jj= 16 as libc::c_int - 1 as libc::c_int;
                    while jj >= 0 as libc::c_int {
                        (*s).mtfa[kk as usize] = (ii * 16 as libc::c_int + jj) as UChar;
                        kk-= 1;
                        jj-= 1;
                    }
                    (*s).mtfbase[ii as usize] = kk + 1 as libc::c_int;
                    ii-= 1;
                }
                nblock= 0 as libc::c_int;
                if groupPos == 0 as libc::c_int {
                    groupNo+= 1;
                    if groupNo >= nSelectors {
                        retVal= -(4 as libc::c_int);
                        current_block= 6284263362753553522;
                        continue;
                    } else {
                        groupPos= 50 as libc::c_int;
                        gSel= (*s).selector[groupNo as usize] as Int32;
                        gMinlen= (*s).minLens[gSel as usize];
                        gLimit= &raw mut *(*(*s).limit.as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut Int32;
                        gPerm= &raw mut *(*(*s).perm.as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut Int32;
                        gBase= &raw mut *(*(*s).base.as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut Int32;
                    }
                }
                groupPos-= 1;
                zn= gMinlen;
                current_block= 13427822320537439643;
            }
        }
    }
    (*s).save_j= j;
    (*s).save_t= t;
    (*s).save_alphaSize= alphaSize;
    (*s).save_nGroups= nGroups;
    (*s).save_nSelectors= nSelectors;
    (*s).save_EOB= EOB;
    (*s).save_groupNo= groupNo;
    (*s).save_groupPos= groupPos;
    (*s).save_nextSym= nextSym;
    (*s).save_nblockMAX= nblockMAX;
    (*s).save_nblock= nblock;
    (*s).save_es= es;
    (*s).save_N= N;
    (*s).save_curr= curr;
    (*s).save_zt= zt;
    (*s).save_zn= zn;
    (*s).save_zvec= zvec;
    (*s).save_zj= zj;
    (*s).save_gSel= gSel;
    (*s).save_gMinlen= gMinlen;
    (*s).save_gLimit= gLimit;
    (*s).save_gBase= gBase;
    (*s).save_gPerm= gPerm;
    return retVal;
}
