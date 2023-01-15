use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn BZ2_hbCreateDecodeTables(
        _: *mut Int32,
        _: *mut Int32,
        _: *mut Int32,
        _: *mut UChar,
        _: Int32,
        _: Int32,
        _: Int32,
    );
    fn BZ2_indexIntoF(_: Int32, _: *mut Int32) -> Int32;
    static mut BZ2_rNums: [Int32; 512];
    fn BZ2_bz__AssertH__fail(errcode: libc::c_int);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut libc::c_char,
    pub avail_in: libc::c_uint,
    pub total_in_lo32: libc::c_uint,
    pub total_in_hi32: libc::c_uint,
    pub next_out: *mut libc::c_char,
    pub avail_out: libc::c_uint,
    pub total_out_lo32: libc::c_uint,
    pub total_out_hi32: libc::c_uint,
    pub state: *mut libc::c_void,
    pub bzalloc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
        ) -> *mut libc::c_void,
    >,
    pub bzfree: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub opaque: *mut libc::c_void,
}
pub type Bool = libc::c_uchar;
pub type UChar = libc::c_uchar;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DState {
    pub strm: *mut bz_stream,
    pub state: Int32,
    pub state_out_ch: UChar,
    pub state_out_len: Int32,
    pub blockRandomised: Bool,
    pub rNToGo: Int32,
    pub rTPos: Int32,
    pub bsBuff: UInt32,
    pub bsLive: Int32,
    pub blockSize100k: Int32,
    pub smallDecompress: Bool,
    pub currBlockNo: Int32,
    pub verbosity: Int32,
    pub origPtr: Int32,
    pub tPos: UInt32,
    pub k0: Int32,
    pub unzftab: [Int32; 256],
    pub nblock_used: Int32,
    pub cftab: [Int32; 257],
    pub cftabCopy: [Int32; 257],
    pub tt: *mut UInt32,
    pub ll16: *mut UInt16,
    pub ll4: *mut UChar,
    pub storedBlockCRC: UInt32,
    pub storedCombinedCRC: UInt32,
    pub calculatedBlockCRC: UInt32,
    pub calculatedCombinedCRC: UInt32,
    pub nInUse: Int32,
    pub inUse: [Bool; 256],
    pub inUse16: [Bool; 16],
    pub seqToUnseq: [UChar; 256],
    pub mtfa: [UChar; 4096],
    pub mtfbase: [Int32; 16],
    pub selector: [UChar; 18002],
    pub selectorMtf: [UChar; 18002],
    pub len: [[UChar; 258]; 6],
    pub limit: [[Int32; 258]; 6],
    pub base: [[Int32; 258]; 6],
    pub perm: [[Int32; 258]; 6],
    pub minLens: [Int32; 6],
    pub save_i: Int32,
    pub save_j: Int32,
    pub save_t: Int32,
    pub save_alphaSize: Int32,
    pub save_nGroups: Int32,
    pub save_nSelectors: Int32,
    pub save_EOB: Int32,
    pub save_groupNo: Int32,
    pub save_groupPos: Int32,
    pub save_nextSym: Int32,
    pub save_nblockMAX: Int32,
    pub save_nblock: Int32,
    pub save_es: Int32,
    pub save_N: Int32,
    pub save_curr: Int32,
    pub save_zt: Int32,
    pub save_zn: Int32,
    pub save_zvec: Int32,
    pub save_zj: Int32,
    pub save_gSel: Int32,
    pub save_gMinlen: Int32,
    pub save_gLimit: *mut Int32,
    pub save_gBase: *mut Int32,
    pub save_gPerm: *mut Int32,
}
unsafe extern "C" fn makeMaps_d(mut s: *mut DState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).seqToUnseq[(*s).nInUse as usize] = i as UChar;
            let ref mut fresh0 = (*s).nInUse;
            *fresh0 += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_decompress(mut s: *mut DState) -> Int32 {
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
        (*s).save_i = 0 as libc::c_int;
        (*s).save_j = 0 as libc::c_int;
        (*s).save_t = 0 as libc::c_int;
        (*s).save_alphaSize = 0 as libc::c_int;
        (*s).save_nGroups = 0 as libc::c_int;
        (*s).save_nSelectors = 0 as libc::c_int;
        (*s).save_EOB = 0 as libc::c_int;
        (*s).save_groupNo = 0 as libc::c_int;
        (*s).save_groupPos = 0 as libc::c_int;
        (*s).save_nextSym = 0 as libc::c_int;
        (*s).save_nblockMAX = 0 as libc::c_int;
        (*s).save_nblock = 0 as libc::c_int;
        (*s).save_es = 0 as libc::c_int;
        (*s).save_N = 0 as libc::c_int;
        (*s).save_curr = 0 as libc::c_int;
        (*s).save_zt = 0 as libc::c_int;
        (*s).save_zn = 0 as libc::c_int;
        (*s).save_zvec = 0 as libc::c_int;
        (*s).save_zj = 0 as libc::c_int;
        (*s).save_gSel = 0 as libc::c_int;
        (*s).save_gMinlen = 0 as libc::c_int;
        let ref mut fresh1 = (*s).save_gLimit;
        *fresh1 = 0 as *mut Int32;
        let ref mut fresh2 = (*s).save_gBase;
        *fresh2 = 0 as *mut Int32;
        let ref mut fresh3 = (*s).save_gPerm;
        *fresh3 = 0 as *mut Int32;
    }
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
    retVal = 0 as libc::c_int;
    match (*s).state {
        10 => {
            (*s).state = 10 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 5658374378798827547;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v: UInt32 = 0;
                    v = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh4 = (*s).bsLive;
                    *fresh4 -= 8 as libc::c_int;
                    uc = v as UChar;
                    current_block = 5658374378798827547;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh5 = (*s).bsLive;
                    *fresh5 += 8 as libc::c_int;
                    let ref mut fresh6 = (*(*s).strm).next_in;
                    *fresh6 = (*fresh6).offset(1);
                    let ref mut fresh7 = (*(*s).strm).avail_in;
                    *fresh7 = (*fresh7).wrapping_sub(1);
                    let ref mut fresh8 = (*(*s).strm).total_in_lo32;
                    *fresh8 = (*fresh8).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh9 = (*(*s).strm).total_in_hi32;
                        *fresh9 = (*fresh9).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x42 as libc::c_int {
                        retVal = -(5 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 3006993205021687454;
                    }
                }
            }
        }
        11 => {
            current_block = 3006993205021687454;
        }
        12 => {
            current_block = 15042001179320269575;
        }
        13 => {
            current_block = 3881449305082167623;
        }
        14 => {
            current_block = 2962939589862947044;
        }
        15 => {
            current_block = 14132709226079871668;
        }
        16 => {
            current_block = 9643419754668440244;
        }
        17 => {
            current_block = 12414557862826760723;
        }
        18 => {
            current_block = 722990626647897627;
        }
        19 => {
            current_block = 12815606485179341722;
        }
        20 => {
            current_block = 12972279979481132367;
        }
        21 => {
            current_block = 541033394006102889;
        }
        22 => {
            current_block = 3813990005137401807;
        }
        23 => {
            current_block = 1753580026933325594;
        }
        24 => {
            current_block = 278081590216066828;
        }
        25 => {
            current_block = 5048411712883903721;
        }
        26 => {
            current_block = 9518779839665286147;
        }
        27 => {
            current_block = 4398027810128408265;
        }
        28 => {
            current_block = 17438419808098997991;
        }
        29 => {
            current_block = 14196862412491516613;
        }
        30 => {
            current_block = 6492128765584295426;
        }
        31 => {
            current_block = 6018809847896417576;
        }
        32 => {
            current_block = 13333503772902270734;
        }
        33 => {
            current_block = 12765707770930006169;
        }
        34 => {
            current_block = 1232411364073810230;
        }
        35 => {
            current_block = 6592809319488464358;
        }
        36 => {
            current_block = 13427822320537439643;
        }
        37 => {
            current_block = 15560064689176435631;
        }
        38 => {
            current_block = 12417384991418392802;
        }
        39 => {
            current_block = 15126558758378545666;
        }
        40 => {
            current_block = 2516300604401959109;
        }
        41 => {
            current_block = 25455516448278576;
        }
        42 => {
            current_block = 13407868514940645036;
        }
        43 => {
            current_block = 7101023504148279726;
        }
        44 => {
            current_block = 18438429198922852023;
        }
        45 => {
            current_block = 16535968932359224061;
        }
        46 => {
            current_block = 5632782989663552478;
        }
        47 => {
            current_block = 4876153394669679111;
        }
        48 => {
            current_block = 12977702551301811648;
        }
        49 => {
            current_block = 2331137395028574638;
        }
        50 => {
            current_block = 18424679783944580699;
        }
        _ => {
            if 0 as libc::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4001 as libc::c_int);
            }
            if 0 as libc::c_int as Bool == 0 {
                BZ2_bz__AssertH__fail(4002 as libc::c_int);
            }
            current_block = 6284263362753553522;
        }
    }
    match current_block {
        3006993205021687454 => {
            (*s).state = 11 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 1658462350791934405;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_0: UInt32 = 0;
                    v_0 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh10 = (*s).bsLive;
                    *fresh10 -= 8 as libc::c_int;
                    uc = v_0 as UChar;
                    current_block = 1658462350791934405;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh11 = (*s).bsLive;
                    *fresh11 += 8 as libc::c_int;
                    let ref mut fresh12 = (*(*s).strm).next_in;
                    *fresh12 = (*fresh12).offset(1);
                    let ref mut fresh13 = (*(*s).strm).avail_in;
                    *fresh13 = (*fresh13).wrapping_sub(1);
                    let ref mut fresh14 = (*(*s).strm).total_in_lo32;
                    *fresh14 = (*fresh14).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh15 = (*(*s).strm).total_in_hi32;
                        *fresh15 = (*fresh15).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x5a as libc::c_int {
                        retVal = -(5 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 15042001179320269575;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        15042001179320269575 => {
            (*s).state = 12 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 16314074004867283505;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_1: UInt32 = 0;
                    v_1 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh16 = (*s).bsLive;
                    *fresh16 -= 8 as libc::c_int;
                    uc = v_1 as UChar;
                    current_block = 16314074004867283505;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh17 = (*s).bsLive;
                    *fresh17 += 8 as libc::c_int;
                    let ref mut fresh18 = (*(*s).strm).next_in;
                    *fresh18 = (*fresh18).offset(1);
                    let ref mut fresh19 = (*(*s).strm).avail_in;
                    *fresh19 = (*fresh19).wrapping_sub(1);
                    let ref mut fresh20 = (*(*s).strm).total_in_lo32;
                    *fresh20 = (*fresh20).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh21 = (*(*s).strm).total_in_hi32;
                        *fresh21 = (*fresh21).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x68 as libc::c_int {
                        retVal = -(5 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 3881449305082167623;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        3881449305082167623 => {
            (*s).state = 13 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 1915186496383530739;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_2: UInt32 = 0;
                    v_2 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh22 = (*s).bsLive;
                    *fresh22 -= 8 as libc::c_int;
                    (*s).blockSize100k = v_2 as Int32;
                    current_block = 1915186496383530739;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh23 = (*s).bsLive;
                    *fresh23 += 8 as libc::c_int;
                    let ref mut fresh24 = (*(*s).strm).next_in;
                    *fresh24 = (*fresh24).offset(1);
                    let ref mut fresh25 = (*(*s).strm).avail_in;
                    *fresh25 = (*fresh25).wrapping_sub(1);
                    let ref mut fresh26 = (*(*s).strm).total_in_lo32;
                    *fresh26 = (*fresh26).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh27 = (*(*s).strm).total_in_hi32;
                        *fresh27 = (*fresh27).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if (*s).blockSize100k < 0x30 as libc::c_int + 1 as libc::c_int
                        || (*s).blockSize100k > 0x30 as libc::c_int + 9 as libc::c_int
                    {
                        retVal = -(5 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        let ref mut fresh28 = (*s).blockSize100k;
                        *fresh28 -= 0x30 as libc::c_int;
                        if (*s).smallDecompress != 0 {
                            let ref mut fresh29 = (*s).ll16;
                            *fresh29 = ((*strm).bzalloc)
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
                            let ref mut fresh30 = (*s).ll4;
                            *fresh30 = ((*strm).bzalloc)
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
                            if ((*s).ll16).is_null() || ((*s).ll4).is_null() {
                                retVal = -(3 as libc::c_int);
                                current_block = 6284263362753553522;
                            } else {
                                current_block = 2962939589862947044;
                            }
                        } else {
                            let ref mut fresh31 = (*s).tt;
                            *fresh31 = ((*strm).bzalloc)
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
                            if ((*s).tt).is_null() {
                                retVal = -(3 as libc::c_int);
                                current_block = 6284263362753553522;
                            } else {
                                current_block = 2962939589862947044;
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
            (*s).state = 14 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 9846950269610550213;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_3: UInt32 = 0;
                    v_3 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh32 = (*s).bsLive;
                    *fresh32 -= 8 as libc::c_int;
                    uc = v_3 as UChar;
                    current_block = 9846950269610550213;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh33 = (*s).bsLive;
                    *fresh33 += 8 as libc::c_int;
                    let ref mut fresh34 = (*(*s).strm).next_in;
                    *fresh34 = (*fresh34).offset(1);
                    let ref mut fresh35 = (*(*s).strm).avail_in;
                    *fresh35 = (*fresh35).wrapping_sub(1);
                    let ref mut fresh36 = (*(*s).strm).total_in_lo32;
                    *fresh36 = (*fresh36).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh37 = (*(*s).strm).total_in_hi32;
                        *fresh37 = (*fresh37).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int == 0x17 as libc::c_int {
                        current_block = 13407868514940645036;
                    } else if uc as libc::c_int != 0x31 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 14132709226079871668;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        13407868514940645036 => {
            (*s).state = 42 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 13262463590990658200;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_32: UInt32 = 0;
                    v_32 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh220 = (*s).bsLive;
                    *fresh220 -= 8 as libc::c_int;
                    uc = v_32 as UChar;
                    current_block = 13262463590990658200;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh221 = (*s).bsLive;
                    *fresh221 += 8 as libc::c_int;
                    let ref mut fresh222 = (*(*s).strm).next_in;
                    *fresh222 = (*fresh222).offset(1);
                    let ref mut fresh223 = (*(*s).strm).avail_in;
                    *fresh223 = (*fresh223).wrapping_sub(1);
                    let ref mut fresh224 = (*(*s).strm).total_in_lo32;
                    *fresh224 = (*fresh224).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh225 = (*(*s).strm).total_in_hi32;
                        *fresh225 = (*fresh225).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x72 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 7101023504148279726;
                    }
                }
            }
        }
        14132709226079871668 => {
            (*s).state = 15 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 3569141194949357899;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_4: UInt32 = 0;
                    v_4 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh38 = (*s).bsLive;
                    *fresh38 -= 8 as libc::c_int;
                    uc = v_4 as UChar;
                    current_block = 3569141194949357899;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh39 = (*s).bsLive;
                    *fresh39 += 8 as libc::c_int;
                    let ref mut fresh40 = (*(*s).strm).next_in;
                    *fresh40 = (*fresh40).offset(1);
                    let ref mut fresh41 = (*(*s).strm).avail_in;
                    *fresh41 = (*fresh41).wrapping_sub(1);
                    let ref mut fresh42 = (*(*s).strm).total_in_lo32;
                    *fresh42 = (*fresh42).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh43 = (*(*s).strm).total_in_hi32;
                        *fresh43 = (*fresh43).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x41 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 9643419754668440244;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        7101023504148279726 => {
            (*s).state = 43 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 10756506701594629759;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_33: UInt32 = 0;
                    v_33 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh226 = (*s).bsLive;
                    *fresh226 -= 8 as libc::c_int;
                    uc = v_33 as UChar;
                    current_block = 10756506701594629759;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh227 = (*s).bsLive;
                    *fresh227 += 8 as libc::c_int;
                    let ref mut fresh228 = (*(*s).strm).next_in;
                    *fresh228 = (*fresh228).offset(1);
                    let ref mut fresh229 = (*(*s).strm).avail_in;
                    *fresh229 = (*fresh229).wrapping_sub(1);
                    let ref mut fresh230 = (*(*s).strm).total_in_lo32;
                    *fresh230 = (*fresh230).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh231 = (*(*s).strm).total_in_hi32;
                        *fresh231 = (*fresh231).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x45 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 18438429198922852023;
                    }
                }
            }
        }
        9643419754668440244 => {
            (*s).state = 16 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 16517180880614114163;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_5: UInt32 = 0;
                    v_5 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh44 = (*s).bsLive;
                    *fresh44 -= 8 as libc::c_int;
                    uc = v_5 as UChar;
                    current_block = 16517180880614114163;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh45 = (*s).bsLive;
                    *fresh45 += 8 as libc::c_int;
                    let ref mut fresh46 = (*(*s).strm).next_in;
                    *fresh46 = (*fresh46).offset(1);
                    let ref mut fresh47 = (*(*s).strm).avail_in;
                    *fresh47 = (*fresh47).wrapping_sub(1);
                    let ref mut fresh48 = (*(*s).strm).total_in_lo32;
                    *fresh48 = (*fresh48).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh49 = (*(*s).strm).total_in_hi32;
                        *fresh49 = (*fresh49).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x59 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 12414557862826760723;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        18438429198922852023 => {
            (*s).state = 44 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 9819403752380335018;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_34: UInt32 = 0;
                    v_34 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh232 = (*s).bsLive;
                    *fresh232 -= 8 as libc::c_int;
                    uc = v_34 as UChar;
                    current_block = 9819403752380335018;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh233 = (*s).bsLive;
                    *fresh233 += 8 as libc::c_int;
                    let ref mut fresh234 = (*(*s).strm).next_in;
                    *fresh234 = (*fresh234).offset(1);
                    let ref mut fresh235 = (*(*s).strm).avail_in;
                    *fresh235 = (*fresh235).wrapping_sub(1);
                    let ref mut fresh236 = (*(*s).strm).total_in_lo32;
                    *fresh236 = (*fresh236).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh237 = (*(*s).strm).total_in_hi32;
                        *fresh237 = (*fresh237).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x38 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 16535968932359224061;
                    }
                }
            }
        }
        12414557862826760723 => {
            (*s).state = 17 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 2606663910910355487;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_6: UInt32 = 0;
                    v_6 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh50 = (*s).bsLive;
                    *fresh50 -= 8 as libc::c_int;
                    uc = v_6 as UChar;
                    current_block = 2606663910910355487;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh51 = (*s).bsLive;
                    *fresh51 += 8 as libc::c_int;
                    let ref mut fresh52 = (*(*s).strm).next_in;
                    *fresh52 = (*fresh52).offset(1);
                    let ref mut fresh53 = (*(*s).strm).avail_in;
                    *fresh53 = (*fresh53).wrapping_sub(1);
                    let ref mut fresh54 = (*(*s).strm).total_in_lo32;
                    *fresh54 = (*fresh54).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh55 = (*(*s).strm).total_in_hi32;
                        *fresh55 = (*fresh55).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x26 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 722990626647897627;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        16535968932359224061 => {
            (*s).state = 45 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 9454797012561717444;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_35: UInt32 = 0;
                    v_35 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh238 = (*s).bsLive;
                    *fresh238 -= 8 as libc::c_int;
                    uc = v_35 as UChar;
                    current_block = 9454797012561717444;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh239 = (*s).bsLive;
                    *fresh239 += 8 as libc::c_int;
                    let ref mut fresh240 = (*(*s).strm).next_in;
                    *fresh240 = (*fresh240).offset(1);
                    let ref mut fresh241 = (*(*s).strm).avail_in;
                    *fresh241 = (*fresh241).wrapping_sub(1);
                    let ref mut fresh242 = (*(*s).strm).total_in_lo32;
                    *fresh242 = (*fresh242).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh243 = (*(*s).strm).total_in_hi32;
                        *fresh243 = (*fresh243).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x50 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 5632782989663552478;
                    }
                }
            }
        }
        722990626647897627 => {
            (*s).state = 18 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 8125779086361653720;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_7: UInt32 = 0;
                    v_7 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh56 = (*s).bsLive;
                    *fresh56 -= 8 as libc::c_int;
                    uc = v_7 as UChar;
                    current_block = 8125779086361653720;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh57 = (*s).bsLive;
                    *fresh57 += 8 as libc::c_int;
                    let ref mut fresh58 = (*(*s).strm).next_in;
                    *fresh58 = (*fresh58).offset(1);
                    let ref mut fresh59 = (*(*s).strm).avail_in;
                    *fresh59 = (*fresh59).wrapping_sub(1);
                    let ref mut fresh60 = (*(*s).strm).total_in_lo32;
                    *fresh60 = (*fresh60).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh61 = (*(*s).strm).total_in_hi32;
                        *fresh61 = (*fresh61).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x53 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        current_block = 12815606485179341722;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        5632782989663552478 => {
            (*s).state = 46 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 724777313732190959;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_36: UInt32 = 0;
                    v_36 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh244 = (*s).bsLive;
                    *fresh244 -= 8 as libc::c_int;
                    uc = v_36 as UChar;
                    current_block = 724777313732190959;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh245 = (*s).bsLive;
                    *fresh245 += 8 as libc::c_int;
                    let ref mut fresh246 = (*(*s).strm).next_in;
                    *fresh246 = (*fresh246).offset(1);
                    let ref mut fresh247 = (*(*s).strm).avail_in;
                    *fresh247 = (*fresh247).wrapping_sub(1);
                    let ref mut fresh248 = (*(*s).strm).total_in_lo32;
                    *fresh248 = (*fresh248).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh249 = (*(*s).strm).total_in_hi32;
                        *fresh249 = (*fresh249).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x90 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        (*s).storedCombinedCRC = 0 as libc::c_int as UInt32;
                        current_block = 4876153394669679111;
                    }
                }
            }
        }
        12815606485179341722 => {
            (*s).state = 19 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 958128786106592581;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_8: UInt32 = 0;
                    v_8 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh62 = (*s).bsLive;
                    *fresh62 -= 8 as libc::c_int;
                    uc = v_8 as UChar;
                    current_block = 958128786106592581;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh63 = (*s).bsLive;
                    *fresh63 += 8 as libc::c_int;
                    let ref mut fresh64 = (*(*s).strm).next_in;
                    *fresh64 = (*fresh64).offset(1);
                    let ref mut fresh65 = (*(*s).strm).avail_in;
                    *fresh65 = (*fresh65).wrapping_sub(1);
                    let ref mut fresh66 = (*(*s).strm).total_in_lo32;
                    *fresh66 = (*fresh66).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh67 = (*(*s).strm).total_in_hi32;
                        *fresh67 = (*fresh67).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    if uc as libc::c_int != 0x59 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        let ref mut fresh68 = (*s).currBlockNo;
                        *fresh68 += 1;
                        if (*s).verbosity >= 2 as libc::c_int {
                            fprintf(
                                stderr,
                                b"\n    [%d: huff+mtf \0" as *const u8
                                    as *const libc::c_char,
                                (*s).currBlockNo,
                            );
                        }
                        (*s).storedBlockCRC = 0 as libc::c_int as UInt32;
                        current_block = 12972279979481132367;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        4876153394669679111 => {
            (*s).state = 47 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 14486187473704332379;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_37: UInt32 = 0;
                    v_37 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh250 = (*s).bsLive;
                    *fresh250 -= 8 as libc::c_int;
                    uc = v_37 as UChar;
                    current_block = 14486187473704332379;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh251 = (*s).bsLive;
                    *fresh251 += 8 as libc::c_int;
                    let ref mut fresh252 = (*(*s).strm).next_in;
                    *fresh252 = (*fresh252).offset(1);
                    let ref mut fresh253 = (*(*s).strm).avail_in;
                    *fresh253 = (*fresh253).wrapping_sub(1);
                    let ref mut fresh254 = (*(*s).strm).total_in_lo32;
                    *fresh254 = (*fresh254).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh255 = (*(*s).strm).total_in_hi32;
                        *fresh255 = (*fresh255).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedCombinedCRC = (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block = 12977702551301811648;
                }
            }
        }
        12972279979481132367 => {
            (*s).state = 20 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 3790734079518302164;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_9: UInt32 = 0;
                    v_9 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh69 = (*s).bsLive;
                    *fresh69 -= 8 as libc::c_int;
                    uc = v_9 as UChar;
                    current_block = 3790734079518302164;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh70 = (*s).bsLive;
                    *fresh70 += 8 as libc::c_int;
                    let ref mut fresh71 = (*(*s).strm).next_in;
                    *fresh71 = (*fresh71).offset(1);
                    let ref mut fresh72 = (*(*s).strm).avail_in;
                    *fresh72 = (*fresh72).wrapping_sub(1);
                    let ref mut fresh73 = (*(*s).strm).total_in_lo32;
                    *fresh73 = (*fresh73).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh74 = (*(*s).strm).total_in_hi32;
                        *fresh74 = (*fresh74).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedBlockCRC = (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block = 541033394006102889;
                }
            }
        }
        _ => {}
    }
    match current_block {
        12977702551301811648 => {
            (*s).state = 48 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 3659807904093622879;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_38: UInt32 = 0;
                    v_38 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh256 = (*s).bsLive;
                    *fresh256 -= 8 as libc::c_int;
                    uc = v_38 as UChar;
                    current_block = 3659807904093622879;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh257 = (*s).bsLive;
                    *fresh257 += 8 as libc::c_int;
                    let ref mut fresh258 = (*(*s).strm).next_in;
                    *fresh258 = (*fresh258).offset(1);
                    let ref mut fresh259 = (*(*s).strm).avail_in;
                    *fresh259 = (*fresh259).wrapping_sub(1);
                    let ref mut fresh260 = (*(*s).strm).total_in_lo32;
                    *fresh260 = (*fresh260).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh261 = (*(*s).strm).total_in_hi32;
                        *fresh261 = (*fresh261).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedCombinedCRC = (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block = 2331137395028574638;
                }
            }
        }
        541033394006102889 => {
            (*s).state = 21 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 16711521214030637000;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_10: UInt32 = 0;
                    v_10 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh75 = (*s).bsLive;
                    *fresh75 -= 8 as libc::c_int;
                    uc = v_10 as UChar;
                    current_block = 16711521214030637000;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh76 = (*s).bsLive;
                    *fresh76 += 8 as libc::c_int;
                    let ref mut fresh77 = (*(*s).strm).next_in;
                    *fresh77 = (*fresh77).offset(1);
                    let ref mut fresh78 = (*(*s).strm).avail_in;
                    *fresh78 = (*fresh78).wrapping_sub(1);
                    let ref mut fresh79 = (*(*s).strm).total_in_lo32;
                    *fresh79 = (*fresh79).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh80 = (*(*s).strm).total_in_hi32;
                        *fresh80 = (*fresh80).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedBlockCRC = (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block = 3813990005137401807;
                }
            }
        }
        _ => {}
    }
    match current_block {
        2331137395028574638 => {
            (*s).state = 49 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 2394045633138979148;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_39: UInt32 = 0;
                    v_39 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh262 = (*s).bsLive;
                    *fresh262 -= 8 as libc::c_int;
                    uc = v_39 as UChar;
                    current_block = 2394045633138979148;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh263 = (*s).bsLive;
                    *fresh263 += 8 as libc::c_int;
                    let ref mut fresh264 = (*(*s).strm).next_in;
                    *fresh264 = (*fresh264).offset(1);
                    let ref mut fresh265 = (*(*s).strm).avail_in;
                    *fresh265 = (*fresh265).wrapping_sub(1);
                    let ref mut fresh266 = (*(*s).strm).total_in_lo32;
                    *fresh266 = (*fresh266).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh267 = (*(*s).strm).total_in_hi32;
                        *fresh267 = (*fresh267).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedCombinedCRC = (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block = 18424679783944580699;
                }
            }
        }
        3813990005137401807 => {
            (*s).state = 22 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 17870985093275900527;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_11: UInt32 = 0;
                    v_11 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh81 = (*s).bsLive;
                    *fresh81 -= 8 as libc::c_int;
                    uc = v_11 as UChar;
                    current_block = 17870985093275900527;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh82 = (*s).bsLive;
                    *fresh82 += 8 as libc::c_int;
                    let ref mut fresh83 = (*(*s).strm).next_in;
                    *fresh83 = (*fresh83).offset(1);
                    let ref mut fresh84 = (*(*s).strm).avail_in;
                    *fresh84 = (*fresh84).wrapping_sub(1);
                    let ref mut fresh85 = (*(*s).strm).total_in_lo32;
                    *fresh85 = (*fresh85).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh86 = (*(*s).strm).total_in_hi32;
                        *fresh86 = (*fresh86).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedBlockCRC = (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block = 1753580026933325594;
                }
            }
        }
        _ => {}
    }
    match current_block {
        1753580026933325594 => {
            (*s).state = 23 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 13734492969709581318;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_12: UInt32 = 0;
                    v_12 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh87 = (*s).bsLive;
                    *fresh87 -= 8 as libc::c_int;
                    uc = v_12 as UChar;
                    current_block = 13734492969709581318;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh88 = (*s).bsLive;
                    *fresh88 += 8 as libc::c_int;
                    let ref mut fresh89 = (*(*s).strm).next_in;
                    *fresh89 = (*fresh89).offset(1);
                    let ref mut fresh90 = (*(*s).strm).avail_in;
                    *fresh90 = (*fresh90).wrapping_sub(1);
                    let ref mut fresh91 = (*(*s).strm).total_in_lo32;
                    *fresh91 = (*fresh91).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh92 = (*(*s).strm).total_in_hi32;
                        *fresh92 = (*fresh92).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedBlockCRC = (*s).storedBlockCRC << 8 as libc::c_int
                        | uc as UInt32;
                    current_block = 278081590216066828;
                }
            }
        }
        18424679783944580699 => {
            (*s).state = 50 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 1904329045571868869;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_40: UInt32 = 0;
                    v_40 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh268 = (*s).bsLive;
                    *fresh268 -= 8 as libc::c_int;
                    uc = v_40 as UChar;
                    current_block = 1904329045571868869;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh269 = (*s).bsLive;
                    *fresh269 += 8 as libc::c_int;
                    let ref mut fresh270 = (*(*s).strm).next_in;
                    *fresh270 = (*fresh270).offset(1);
                    let ref mut fresh271 = (*(*s).strm).avail_in;
                    *fresh271 = (*fresh271).wrapping_sub(1);
                    let ref mut fresh272 = (*(*s).strm).total_in_lo32;
                    *fresh272 = (*fresh272).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh273 = (*(*s).strm).total_in_hi32;
                        *fresh273 = (*fresh273).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s)
                        .storedCombinedCRC = (*s).storedCombinedCRC << 8 as libc::c_int
                        | uc as UInt32;
                    (*s).state = 1 as libc::c_int;
                    retVal = 4 as libc::c_int;
                    current_block = 6284263362753553522;
                }
            }
        }
        _ => {}
    }
    match current_block {
        278081590216066828 => {
            (*s).state = 24 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 15030729790988239748;
                    break;
                }
                if (*s).bsLive >= 1 as libc::c_int {
                    let mut v_13: UInt32 = 0;
                    v_13 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                        & (((1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh93 = (*s).bsLive;
                    *fresh93 -= 1 as libc::c_int;
                    (*s).blockRandomised = v_13 as Bool;
                    current_block = 15030729790988239748;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh94 = (*s).bsLive;
                    *fresh94 += 8 as libc::c_int;
                    let ref mut fresh95 = (*(*s).strm).next_in;
                    *fresh95 = (*fresh95).offset(1);
                    let ref mut fresh96 = (*(*s).strm).avail_in;
                    *fresh96 = (*fresh96).wrapping_sub(1);
                    let ref mut fresh97 = (*(*s).strm).total_in_lo32;
                    *fresh97 = (*fresh97).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh98 = (*(*s).strm).total_in_hi32;
                        *fresh98 = (*fresh98).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr = 0 as libc::c_int;
                    current_block = 5048411712883903721;
                }
            }
        }
        _ => {}
    }
    match current_block {
        5048411712883903721 => {
            (*s).state = 25 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 8260322496947496197;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_14: UInt32 = 0;
                    v_14 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh99 = (*s).bsLive;
                    *fresh99 -= 8 as libc::c_int;
                    uc = v_14 as UChar;
                    current_block = 8260322496947496197;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh100 = (*s).bsLive;
                    *fresh100 += 8 as libc::c_int;
                    let ref mut fresh101 = (*(*s).strm).next_in;
                    *fresh101 = (*fresh101).offset(1);
                    let ref mut fresh102 = (*(*s).strm).avail_in;
                    *fresh102 = (*fresh102).wrapping_sub(1);
                    let ref mut fresh103 = (*(*s).strm).total_in_lo32;
                    *fresh103 = (*fresh103).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh104 = (*(*s).strm).total_in_hi32;
                        *fresh104 = (*fresh104).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr = (*s).origPtr << 8 as libc::c_int | uc as Int32;
                    current_block = 9518779839665286147;
                }
            }
        }
        _ => {}
    }
    match current_block {
        9518779839665286147 => {
            (*s).state = 26 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 5561851013817067674;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_15: UInt32 = 0;
                    v_15 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh105 = (*s).bsLive;
                    *fresh105 -= 8 as libc::c_int;
                    uc = v_15 as UChar;
                    current_block = 5561851013817067674;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh106 = (*s).bsLive;
                    *fresh106 += 8 as libc::c_int;
                    let ref mut fresh107 = (*(*s).strm).next_in;
                    *fresh107 = (*fresh107).offset(1);
                    let ref mut fresh108 = (*(*s).strm).avail_in;
                    *fresh108 = (*fresh108).wrapping_sub(1);
                    let ref mut fresh109 = (*(*s).strm).total_in_lo32;
                    *fresh109 = (*fresh109).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh110 = (*(*s).strm).total_in_hi32;
                        *fresh110 = (*fresh110).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr = (*s).origPtr << 8 as libc::c_int | uc as Int32;
                    current_block = 4398027810128408265;
                }
            }
        }
        _ => {}
    }
    match current_block {
        4398027810128408265 => {
            (*s).state = 27 as libc::c_int;
            loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 10471999855724930313;
                    break;
                }
                if (*s).bsLive >= 8 as libc::c_int {
                    let mut v_16: UInt32 = 0;
                    v_16 = (*s).bsBuff >> (*s).bsLive - 8 as libc::c_int
                        & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                            as libc::c_uint;
                    let ref mut fresh111 = (*s).bsLive;
                    *fresh111 -= 8 as libc::c_int;
                    uc = v_16 as UChar;
                    current_block = 10471999855724930313;
                    break;
                } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    retVal = 0 as libc::c_int;
                    current_block = 6284263362753553522;
                    break;
                } else {
                    (*s)
                        .bsBuff = (*s).bsBuff << 8 as libc::c_int
                        | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                    let ref mut fresh112 = (*s).bsLive;
                    *fresh112 += 8 as libc::c_int;
                    let ref mut fresh113 = (*(*s).strm).next_in;
                    *fresh113 = (*fresh113).offset(1);
                    let ref mut fresh114 = (*(*s).strm).avail_in;
                    *fresh114 = (*fresh114).wrapping_sub(1);
                    let ref mut fresh115 = (*(*s).strm).total_in_lo32;
                    *fresh115 = (*fresh115).wrapping_add(1);
                    if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                        let ref mut fresh116 = (*(*s).strm).total_in_hi32;
                        *fresh116 = (*fresh116).wrapping_add(1);
                    }
                }
            }
            match current_block {
                6284263362753553522 => {}
                _ => {
                    (*s).origPtr = (*s).origPtr << 8 as libc::c_int | uc as Int32;
                    if (*s).origPtr < 0 as libc::c_int {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else if (*s).origPtr
                        > 10 as libc::c_int + 100000 as libc::c_int * (*s).blockSize100k
                    {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                    } else {
                        i = 0 as libc::c_int;
                        current_block = 17262312153619709241;
                    }
                }
            }
        }
        _ => {}
    }
    'c_10057: loop {
        match current_block {
            6284263362753553522 => {
                (*s).save_i = i;
                break;
            }
            2516300604401959109 => {
                (*s).state = 40 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_30: UInt32 = 0;
                        v_30 = (*s).bsBuff >> (*s).bsLive - zn
                            & (((1 as libc::c_int) << zn) - 1 as libc::c_int)
                                as libc::c_uint;
                        let ref mut fresh194 = (*s).bsLive;
                        *fresh194 -= zn;
                        zvec = v_30 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh195 = (*s).bsLive;
                        *fresh195 += 8 as libc::c_int;
                        let ref mut fresh196 = (*(*s).strm).next_in;
                        *fresh196 = (*fresh196).offset(1);
                        let ref mut fresh197 = (*(*s).strm).avail_in;
                        *fresh197 = (*fresh197).wrapping_sub(1);
                        let ref mut fresh198 = (*(*s).strm).total_in_lo32;
                        *fresh198 = (*fresh198).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh199 = (*(*s).strm).total_in_hi32;
                            *fresh199 = (*fresh199).wrapping_add(1);
                        }
                    }
                }
                current_block = 9078889872071895942;
            }
            15126558758378545666 => {
                (*s).state = 39 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_29: UInt32 = 0;
                        v_29 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh183 = (*s).bsLive;
                        *fresh183 -= 1 as libc::c_int;
                        zj = v_29 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh184 = (*s).bsLive;
                        *fresh184 += 8 as libc::c_int;
                        let ref mut fresh185 = (*(*s).strm).next_in;
                        *fresh185 = (*fresh185).offset(1);
                        let ref mut fresh186 = (*(*s).strm).avail_in;
                        *fresh186 = (*fresh186).wrapping_sub(1);
                        let ref mut fresh187 = (*(*s).strm).total_in_lo32;
                        *fresh187 = (*fresh187).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh188 = (*(*s).strm).total_in_hi32;
                            *fresh188 = (*fresh188).wrapping_add(1);
                        }
                    }
                }
                zvec = zvec << 1 as libc::c_int | zj;
                current_block = 13605767259572914371;
            }
            12417384991418392802 => {
                (*s).state = 38 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_28: UInt32 = 0;
                        v_28 = (*s).bsBuff >> (*s).bsLive - zn
                            & (((1 as libc::c_int) << zn) - 1 as libc::c_int)
                                as libc::c_uint;
                        let ref mut fresh177 = (*s).bsLive;
                        *fresh177 -= zn;
                        zvec = v_28 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh178 = (*s).bsLive;
                        *fresh178 += 8 as libc::c_int;
                        let ref mut fresh179 = (*(*s).strm).next_in;
                        *fresh179 = (*fresh179).offset(1);
                        let ref mut fresh180 = (*(*s).strm).avail_in;
                        *fresh180 = (*fresh180).wrapping_sub(1);
                        let ref mut fresh181 = (*(*s).strm).total_in_lo32;
                        *fresh181 = (*fresh181).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh182 = (*(*s).strm).total_in_hi32;
                            *fresh182 = (*fresh182).wrapping_add(1);
                        }
                    }
                }
                current_block = 13605767259572914371;
            }
            15560064689176435631 => {
                (*s).state = 37 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_27: UInt32 = 0;
                        v_27 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh171 = (*s).bsLive;
                        *fresh171 -= 1 as libc::c_int;
                        zj = v_27 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh172 = (*s).bsLive;
                        *fresh172 += 8 as libc::c_int;
                        let ref mut fresh173 = (*(*s).strm).next_in;
                        *fresh173 = (*fresh173).offset(1);
                        let ref mut fresh174 = (*(*s).strm).avail_in;
                        *fresh174 = (*fresh174).wrapping_sub(1);
                        let ref mut fresh175 = (*(*s).strm).total_in_lo32;
                        *fresh175 = (*fresh175).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh176 = (*(*s).strm).total_in_hi32;
                            *fresh176 = (*fresh176).wrapping_add(1);
                        }
                    }
                }
                zvec = zvec << 1 as libc::c_int | zj;
                current_block = 1550405138573481750;
            }
            13427822320537439643 => {
                (*s).state = 36 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= zn {
                        let mut v_26: UInt32 = 0;
                        v_26 = (*s).bsBuff >> (*s).bsLive - zn
                            & (((1 as libc::c_int) << zn) - 1 as libc::c_int)
                                as libc::c_uint;
                        let ref mut fresh165 = (*s).bsLive;
                        *fresh165 -= zn;
                        zvec = v_26 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh166 = (*s).bsLive;
                        *fresh166 += 8 as libc::c_int;
                        let ref mut fresh167 = (*(*s).strm).next_in;
                        *fresh167 = (*fresh167).offset(1);
                        let ref mut fresh168 = (*(*s).strm).avail_in;
                        *fresh168 = (*fresh168).wrapping_sub(1);
                        let ref mut fresh169 = (*(*s).strm).total_in_lo32;
                        *fresh169 = (*fresh169).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh170 = (*(*s).strm).total_in_hi32;
                            *fresh170 = (*fresh170).wrapping_add(1);
                        }
                    }
                }
                current_block = 1550405138573481750;
            }
            6592809319488464358 => {
                (*s).state = 35 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_25: UInt32 = 0;
                        v_25 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh159 = (*s).bsLive;
                        *fresh159 -= 1 as libc::c_int;
                        uc = v_25 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh160 = (*s).bsLive;
                        *fresh160 += 8 as libc::c_int;
                        let ref mut fresh161 = (*(*s).strm).next_in;
                        *fresh161 = (*fresh161).offset(1);
                        let ref mut fresh162 = (*(*s).strm).avail_in;
                        *fresh162 = (*fresh162).wrapping_sub(1);
                        let ref mut fresh163 = (*(*s).strm).total_in_lo32;
                        *fresh163 = (*fresh163).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh164 = (*(*s).strm).total_in_hi32;
                            *fresh164 = (*fresh164).wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 0 as libc::c_int {
                    curr += 1;
                } else {
                    curr -= 1;
                }
                current_block = 11858046780433112516;
            }
            1232411364073810230 => {
                (*s).state = 34 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_24: UInt32 = 0;
                        v_24 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh153 = (*s).bsLive;
                        *fresh153 -= 1 as libc::c_int;
                        uc = v_24 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh154 = (*s).bsLive;
                        *fresh154 += 8 as libc::c_int;
                        let ref mut fresh155 = (*(*s).strm).next_in;
                        *fresh155 = (*fresh155).offset(1);
                        let ref mut fresh156 = (*(*s).strm).avail_in;
                        *fresh156 = (*fresh156).wrapping_sub(1);
                        let ref mut fresh157 = (*(*s).strm).total_in_lo32;
                        *fresh157 = (*fresh157).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh158 = (*(*s).strm).total_in_hi32;
                            *fresh158 = (*fresh158).wrapping_add(1);
                        }
                    }
                }
                if !(uc as libc::c_int == 0 as libc::c_int) {
                    current_block = 6592809319488464358;
                    continue;
                }
                current_block = 17503523010989424999;
            }
            12765707770930006169 => {
                (*s).state = 33 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 5 as libc::c_int {
                        let mut v_23: UInt32 = 0;
                        v_23 = (*s).bsBuff >> (*s).bsLive - 5 as libc::c_int
                            & (((1 as libc::c_int) << 5 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh147 = (*s).bsLive;
                        *fresh147 -= 5 as libc::c_int;
                        curr = v_23 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh148 = (*s).bsLive;
                        *fresh148 += 8 as libc::c_int;
                        let ref mut fresh149 = (*(*s).strm).next_in;
                        *fresh149 = (*fresh149).offset(1);
                        let ref mut fresh150 = (*(*s).strm).avail_in;
                        *fresh150 = (*fresh150).wrapping_sub(1);
                        let ref mut fresh151 = (*(*s).strm).total_in_lo32;
                        *fresh151 = (*fresh151).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh152 = (*(*s).strm).total_in_hi32;
                            *fresh152 = (*fresh152).wrapping_add(1);
                        }
                    }
                }
                i = 0 as libc::c_int;
                current_block = 3770765986603902964;
            }
            13333503772902270734 => {
                (*s).state = 32 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_21: UInt32 = 0;
                        v_21 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh141 = (*s).bsLive;
                        *fresh141 -= 1 as libc::c_int;
                        uc = v_21 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh142 = (*s).bsLive;
                        *fresh142 += 8 as libc::c_int;
                        let ref mut fresh143 = (*(*s).strm).next_in;
                        *fresh143 = (*fresh143).offset(1);
                        let ref mut fresh144 = (*(*s).strm).avail_in;
                        *fresh144 = (*fresh144).wrapping_sub(1);
                        let ref mut fresh145 = (*(*s).strm).total_in_lo32;
                        *fresh145 = (*fresh145).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh146 = (*(*s).strm).total_in_hi32;
                            *fresh146 = (*fresh146).wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 0 as libc::c_int {
                    current_block = 5281038271658253520;
                } else {
                    j += 1;
                    if j >= nGroups {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                        continue;
                    } else {
                        current_block = 6927328446518169316;
                    }
                }
            }
            6018809847896417576 => {
                (*s).state = 31 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 15 as libc::c_int {
                        let mut v_20: UInt32 = 0;
                        v_20 = (*s).bsBuff >> (*s).bsLive - 15 as libc::c_int
                            & (((1 as libc::c_int) << 15 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh135 = (*s).bsLive;
                        *fresh135 -= 15 as libc::c_int;
                        nSelectors = v_20 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh136 = (*s).bsLive;
                        *fresh136 += 8 as libc::c_int;
                        let ref mut fresh137 = (*(*s).strm).next_in;
                        *fresh137 = (*fresh137).offset(1);
                        let ref mut fresh138 = (*(*s).strm).avail_in;
                        *fresh138 = (*fresh138).wrapping_sub(1);
                        let ref mut fresh139 = (*(*s).strm).total_in_lo32;
                        *fresh139 = (*fresh139).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh140 = (*(*s).strm).total_in_hi32;
                            *fresh140 = (*fresh140).wrapping_add(1);
                        }
                    }
                }
                if nSelectors < 1 as libc::c_int {
                    retVal = -(4 as libc::c_int);
                    current_block = 6284263362753553522;
                    continue;
                } else {
                    i = 0 as libc::c_int;
                }
                current_block = 6591141407893725683;
            }
            6492128765584295426 => {
                (*s).state = 30 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 3 as libc::c_int {
                        let mut v_19: UInt32 = 0;
                        v_19 = (*s).bsBuff >> (*s).bsLive - 3 as libc::c_int
                            & (((1 as libc::c_int) << 3 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh129 = (*s).bsLive;
                        *fresh129 -= 3 as libc::c_int;
                        nGroups = v_19 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh130 = (*s).bsLive;
                        *fresh130 += 8 as libc::c_int;
                        let ref mut fresh131 = (*(*s).strm).next_in;
                        *fresh131 = (*fresh131).offset(1);
                        let ref mut fresh132 = (*(*s).strm).avail_in;
                        *fresh132 = (*fresh132).wrapping_sub(1);
                        let ref mut fresh133 = (*(*s).strm).total_in_lo32;
                        *fresh133 = (*fresh133).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh134 = (*(*s).strm).total_in_hi32;
                            *fresh134 = (*fresh134).wrapping_add(1);
                        }
                    }
                }
                if !(nGroups < 2 as libc::c_int || nGroups > 6 as libc::c_int) {
                    current_block = 6018809847896417576;
                    continue;
                }
                retVal = -(4 as libc::c_int);
                current_block = 6284263362753553522;
                continue;
            }
            14196862412491516613 => {
                (*s).state = 29 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_18: UInt32 = 0;
                        v_18 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh123 = (*s).bsLive;
                        *fresh123 -= 1 as libc::c_int;
                        uc = v_18 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh124 = (*s).bsLive;
                        *fresh124 += 8 as libc::c_int;
                        let ref mut fresh125 = (*(*s).strm).next_in;
                        *fresh125 = (*fresh125).offset(1);
                        let ref mut fresh126 = (*(*s).strm).avail_in;
                        *fresh126 = (*fresh126).wrapping_sub(1);
                        let ref mut fresh127 = (*(*s).strm).total_in_lo32;
                        *fresh127 = (*fresh127).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh128 = (*(*s).strm).total_in_hi32;
                            *fresh128 = (*fresh128).wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 1 as libc::c_int {
                    (*s)
                        .inUse[(i * 16 as libc::c_int + j)
                        as usize] = 1 as libc::c_int as Bool;
                }
                j += 1;
                current_block = 3854024847017804838;
            }
            17262312153619709241 => {
                if i < 16 as libc::c_int {
                    current_block = 17438419808098997991;
                    continue;
                }
                i = 0 as libc::c_int;
                while i < 256 as libc::c_int {
                    (*s).inUse[i as usize] = 0 as libc::c_int as Bool;
                    i += 1;
                }
                i = 0 as libc::c_int;
                current_block = 3472349144349095221;
            }
            17438419808098997991 => {
                (*s).state = 28 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_17: UInt32 = 0;
                        v_17 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh117 = (*s).bsLive;
                        *fresh117 -= 1 as libc::c_int;
                        uc = v_17 as UChar;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh118 = (*s).bsLive;
                        *fresh118 += 8 as libc::c_int;
                        let ref mut fresh119 = (*(*s).strm).next_in;
                        *fresh119 = (*fresh119).offset(1);
                        let ref mut fresh120 = (*(*s).strm).avail_in;
                        *fresh120 = (*fresh120).wrapping_sub(1);
                        let ref mut fresh121 = (*(*s).strm).total_in_lo32;
                        *fresh121 = (*fresh121).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh122 = (*(*s).strm).total_in_hi32;
                            *fresh122 = (*fresh122).wrapping_add(1);
                        }
                    }
                }
                if uc as libc::c_int == 1 as libc::c_int {
                    (*s).inUse16[i as usize] = 1 as libc::c_int as Bool;
                } else {
                    (*s).inUse16[i as usize] = 0 as libc::c_int as Bool;
                }
                i += 1;
                current_block = 17262312153619709241;
                continue;
            }
            _ => {
                (*s).state = 41 as libc::c_int;
                while 1 as libc::c_int as Bool != 0 {
                    if (*s).bsLive >= 1 as libc::c_int {
                        let mut v_31: UInt32 = 0;
                        v_31 = (*s).bsBuff >> (*s).bsLive - 1 as libc::c_int
                            & (((1 as libc::c_int) << 1 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_uint;
                        let ref mut fresh200 = (*s).bsLive;
                        *fresh200 -= 1 as libc::c_int;
                        zj = v_31 as Int32;
                        break;
                    } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        retVal = 0 as libc::c_int;
                        current_block = 6284263362753553522;
                        continue 'c_10057;
                    } else {
                        (*s)
                            .bsBuff = (*s).bsBuff << 8 as libc::c_int
                            | *((*(*s).strm).next_in as *mut UChar) as UInt32;
                        let ref mut fresh201 = (*s).bsLive;
                        *fresh201 += 8 as libc::c_int;
                        let ref mut fresh202 = (*(*s).strm).next_in;
                        *fresh202 = (*fresh202).offset(1);
                        let ref mut fresh203 = (*(*s).strm).avail_in;
                        *fresh203 = (*fresh203).wrapping_sub(1);
                        let ref mut fresh204 = (*(*s).strm).total_in_lo32;
                        *fresh204 = (*fresh204).wrapping_add(1);
                        if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint
                        {
                            let ref mut fresh205 = (*(*s).strm).total_in_hi32;
                            *fresh205 = (*fresh205).wrapping_add(1);
                        }
                    }
                }
                zvec = zvec << 1 as libc::c_int | zj;
                current_block = 9078889872071895942;
            }
        }
        match current_block {
            9078889872071895942 => {
                if zn > 20 as libc::c_int {
                    retVal = -(4 as libc::c_int);
                    current_block = 6284263362753553522;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as libc::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as libc::c_int
                    {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                        continue;
                    } else {
                        nextSym = *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                    }
                } else {
                    zn += 1;
                    current_block = 25455516448278576;
                    continue;
                }
                current_block = 15093386068129942558;
            }
            13605767259572914371 => {
                if zn > 20 as libc::c_int {
                    retVal = -(4 as libc::c_int);
                    current_block = 6284263362753553522;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as libc::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as libc::c_int
                    {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                        continue;
                    } else {
                        nextSym = *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                        if nextSym == 0 as libc::c_int || nextSym == 1 as libc::c_int {
                            current_block = 4550729491376650574;
                        } else {
                            es += 1;
                            uc = (*s)
                                .seqToUnseq[(*s)
                                .mtfa[(*s).mtfbase[0 as libc::c_int as usize] as usize]
                                as usize];
                            let ref mut fresh189 = (*s).unzftab[uc as usize];
                            *fresh189 += es;
                            if (*s).smallDecompress != 0 {
                                while es > 0 as libc::c_int {
                                    if nblock >= nblockMAX {
                                        retVal = -(4 as libc::c_int);
                                        current_block = 6284263362753553522;
                                        continue 'c_10057;
                                    } else {
                                        *((*s).ll16).offset(nblock as isize) = uc as UInt16;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            } else {
                                while es > 0 as libc::c_int {
                                    if nblock >= nblockMAX {
                                        retVal = -(4 as libc::c_int);
                                        current_block = 6284263362753553522;
                                        continue 'c_10057;
                                    } else {
                                        *((*s).tt).offset(nblock as isize) = uc as UInt32;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            }
                            current_block = 15093386068129942558;
                        }
                    }
                } else {
                    zn += 1;
                    current_block = 15126558758378545666;
                    continue;
                }
            }
            1550405138573481750 => {
                if zn > 20 as libc::c_int {
                    retVal = -(4 as libc::c_int);
                    current_block = 6284263362753553522;
                    continue;
                } else if zvec <= *gLimit.offset(zn as isize) {
                    if zvec - *gBase.offset(zn as isize) < 0 as libc::c_int
                        || zvec - *gBase.offset(zn as isize) >= 258 as libc::c_int
                    {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                        continue;
                    } else {
                        nextSym = *gPerm
                            .offset((zvec - *gBase.offset(zn as isize)) as isize);
                    }
                } else {
                    zn += 1;
                    current_block = 15560064689176435631;
                    continue;
                }
                current_block = 15093386068129942558;
            }
            _ => {}
        }
        match current_block {
            15093386068129942558 => {
                if 1 as libc::c_int as Bool != 0 {
                    if nextSym == EOB {
                        current_block = 12118509005321596519;
                    } else {
                        if nextSym == 0 as libc::c_int || nextSym == 1 as libc::c_int {
                            es = -(1 as libc::c_int);
                            N = 1 as libc::c_int;
                        } else if nblock >= nblockMAX {
                            retVal = -(4 as libc::c_int);
                            current_block = 6284263362753553522;
                            continue;
                        } else {
                            let mut ii_0: Int32 = 0;
                            let mut jj_0: Int32 = 0;
                            let mut kk_0: Int32 = 0;
                            let mut pp: Int32 = 0;
                            let mut lno: Int32 = 0;
                            let mut off: Int32 = 0;
                            let mut nn: UInt32 = 0;
                            nn = (nextSym - 1 as libc::c_int) as UInt32;
                            if nn < 16 as libc::c_int as libc::c_uint {
                                pp = (*s).mtfbase[0 as libc::c_int as usize];
                                uc = (*s)
                                    .mtfa[(pp as libc::c_uint).wrapping_add(nn) as usize];
                                while nn > 3 as libc::c_int as libc::c_uint {
                                    let mut z = (pp as libc::c_uint).wrapping_add(nn) as Int32;
                                    (*s)
                                        .mtfa[z
                                        as usize] = (*s).mtfa[(z - 1 as libc::c_int) as usize];
                                    (*s)
                                        .mtfa[(z - 1 as libc::c_int)
                                        as usize] = (*s).mtfa[(z - 2 as libc::c_int) as usize];
                                    (*s)
                                        .mtfa[(z - 2 as libc::c_int)
                                        as usize] = (*s).mtfa[(z - 3 as libc::c_int) as usize];
                                    (*s)
                                        .mtfa[(z - 3 as libc::c_int)
                                        as usize] = (*s).mtfa[(z - 4 as libc::c_int) as usize];
                                    nn = (nn as libc::c_uint)
                                        .wrapping_sub(4 as libc::c_int as libc::c_uint) as UInt32
                                        as UInt32;
                                }
                                while nn > 0 as libc::c_int as libc::c_uint {
                                    (*s)
                                        .mtfa[(pp as libc::c_uint).wrapping_add(nn)
                                        as usize] = (*s)
                                        .mtfa[(pp as libc::c_uint)
                                        .wrapping_add(nn)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize];
                                    nn = nn.wrapping_sub(1);
                                }
                                (*s).mtfa[pp as usize] = uc;
                            } else {
                                lno = nn.wrapping_div(16 as libc::c_int as libc::c_uint)
                                    as Int32;
                                off = nn.wrapping_rem(16 as libc::c_int as libc::c_uint)
                                    as Int32;
                                pp = (*s).mtfbase[lno as usize] + off;
                                uc = (*s).mtfa[pp as usize];
                                while pp > (*s).mtfbase[lno as usize] {
                                    (*s)
                                        .mtfa[pp
                                        as usize] = (*s).mtfa[(pp - 1 as libc::c_int) as usize];
                                    pp -= 1;
                                }
                                let ref mut fresh190 = (*s).mtfbase[lno as usize];
                                *fresh190 += 1;
                                while lno > 0 as libc::c_int {
                                    let ref mut fresh191 = (*s).mtfbase[lno as usize];
                                    *fresh191 -= 1;
                                    (*s)
                                        .mtfa[(*s).mtfbase[lno as usize]
                                        as usize] = (*s)
                                        .mtfa[((*s).mtfbase[(lno - 1 as libc::c_int) as usize]
                                        + 16 as libc::c_int - 1 as libc::c_int) as usize];
                                    lno -= 1;
                                }
                                let ref mut fresh192 = (*s)
                                    .mtfbase[0 as libc::c_int as usize];
                                *fresh192 -= 1;
                                (*s)
                                    .mtfa[(*s).mtfbase[0 as libc::c_int as usize]
                                    as usize] = uc;
                                if (*s).mtfbase[0 as libc::c_int as usize]
                                    == 0 as libc::c_int
                                {
                                    kk_0 = 4096 as libc::c_int - 1 as libc::c_int;
                                    ii_0 = 256 as libc::c_int / 16 as libc::c_int
                                        - 1 as libc::c_int;
                                    while ii_0 >= 0 as libc::c_int {
                                        jj_0 = 16 as libc::c_int - 1 as libc::c_int;
                                        while jj_0 >= 0 as libc::c_int {
                                            (*s)
                                                .mtfa[kk_0
                                                as usize] = (*s)
                                                .mtfa[((*s).mtfbase[ii_0 as usize] + jj_0) as usize];
                                            kk_0 -= 1;
                                            jj_0 -= 1;
                                        }
                                        (*s).mtfbase[ii_0 as usize] = kk_0 + 1 as libc::c_int;
                                        ii_0 -= 1;
                                    }
                                }
                            }
                            let ref mut fresh193 = (*s)
                                .unzftab[(*s).seqToUnseq[uc as usize] as usize];
                            *fresh193 += 1;
                            if (*s).smallDecompress != 0 {
                                *((*s).ll16)
                                    .offset(
                                        nblock as isize,
                                    ) = (*s).seqToUnseq[uc as usize] as UInt16;
                            } else {
                                *((*s).tt)
                                    .offset(
                                        nblock as isize,
                                    ) = (*s).seqToUnseq[uc as usize] as UInt32;
                            }
                            nblock += 1;
                            if groupPos == 0 as libc::c_int {
                                groupNo += 1;
                                if groupNo >= nSelectors {
                                    retVal = -(4 as libc::c_int);
                                    current_block = 6284263362753553522;
                                    continue;
                                } else {
                                    groupPos = 50 as libc::c_int;
                                    gSel = (*s).selector[groupNo as usize] as Int32;
                                    gMinlen = (*s).minLens[gSel as usize];
                                    gLimit = &mut *(*((*s).limit)
                                        .as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut Int32;
                                    gPerm = &mut *(*((*s).perm)
                                        .as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut Int32;
                                    gBase = &mut *(*((*s).base)
                                        .as_mut_ptr()
                                        .offset(gSel as isize))
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as *mut Int32;
                                }
                            }
                            groupPos -= 1;
                            zn = gMinlen;
                            current_block = 2516300604401959109;
                            continue;
                        }
                        current_block = 4550729491376650574;
                    }
                } else {
                    current_block = 12118509005321596519;
                }
                match current_block {
                    4550729491376650574 => {}
                    _ => {
                        if (*s).origPtr < 0 as libc::c_int || (*s).origPtr >= nblock {
                            retVal = -(4 as libc::c_int);
                            current_block = 6284263362753553522;
                            continue;
                        } else {
                            i = 0 as libc::c_int;
                            while i <= 255 as libc::c_int {
                                if (*s).unzftab[i as usize] < 0 as libc::c_int
                                    || (*s).unzftab[i as usize] > nblock
                                {
                                    retVal = -(4 as libc::c_int);
                                    current_block = 6284263362753553522;
                                    continue 'c_10057;
                                } else {
                                    i += 1;
                                }
                            }
                            (*s).cftab[0 as libc::c_int as usize] = 0 as libc::c_int;
                            i = 1 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                (*s)
                                    .cftab[i
                                    as usize] = (*s).unzftab[(i - 1 as libc::c_int) as usize];
                                i += 1;
                            }
                            i = 1 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                let ref mut fresh206 = (*s).cftab[i as usize];
                                *fresh206 += (*s).cftab[(i - 1 as libc::c_int) as usize];
                                i += 1;
                            }
                            i = 0 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                if (*s).cftab[i as usize] < 0 as libc::c_int
                                    || (*s).cftab[i as usize] > nblock
                                {
                                    retVal = -(4 as libc::c_int);
                                    current_block = 6284263362753553522;
                                    continue 'c_10057;
                                } else {
                                    i += 1;
                                }
                            }
                            i = 1 as libc::c_int;
                            while i <= 256 as libc::c_int {
                                if (*s).cftab[(i - 1 as libc::c_int) as usize]
                                    > (*s).cftab[i as usize]
                                {
                                    retVal = -(4 as libc::c_int);
                                    current_block = 6284263362753553522;
                                    continue 'c_10057;
                                } else {
                                    i += 1;
                                }
                            }
                            (*s).state_out_len = 0 as libc::c_int;
                            (*s).state_out_ch = 0 as libc::c_int as UChar;
                            (*s)
                                .calculatedBlockCRC = 0xffffffff as libc::c_long as UInt32;
                            (*s).state = 2 as libc::c_int;
                            if (*s).verbosity >= 2 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"rt+rld\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if (*s).smallDecompress != 0 {
                                i = 0 as libc::c_int;
                                while i <= 256 as libc::c_int {
                                    (*s).cftabCopy[i as usize] = (*s).cftab[i as usize];
                                    i += 1;
                                }
                                i = 0 as libc::c_int;
                                while i < nblock {
                                    uc = *((*s).ll16).offset(i as isize) as UChar;
                                    *((*s).ll16)
                                        .offset(
                                            i as isize,
                                        ) = ((*s).cftabCopy[uc as usize] & 0xffff as libc::c_int)
                                        as UInt16;
                                    if i & 0x1 as libc::c_int == 0 as libc::c_int {
                                        *((*s).ll4)
                                            .offset(
                                                (i >> 1 as libc::c_int) as isize,
                                            ) = (*((*s).ll4).offset((i >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf0 as libc::c_int
                                            | (*s).cftabCopy[uc as usize] >> 16 as libc::c_int)
                                            as UChar;
                                    } else {
                                        *((*s).ll4)
                                            .offset(
                                                (i >> 1 as libc::c_int) as isize,
                                            ) = (*((*s).ll4).offset((i >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf as libc::c_int
                                            | ((*s).cftabCopy[uc as usize] >> 16 as libc::c_int)
                                                << 4 as libc::c_int) as UChar;
                                    }
                                    let ref mut fresh207 = (*s).cftabCopy[uc as usize];
                                    *fresh207 += 1;
                                    i += 1;
                                }
                                i = (*s).origPtr;
                                j = (*((*s).ll16).offset(i as isize) as UInt32
                                    | (*((*s).ll4).offset((i >> 1 as libc::c_int) as isize)
                                        as UInt32 >> (i << 2 as libc::c_int & 0x4 as libc::c_int)
                                        & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int)
                                    as Int32;
                                loop {
                                    let mut tmp_0 = (*((*s).ll16).offset(j as isize) as UInt32
                                        | (*((*s).ll4).offset((j >> 1 as libc::c_int) as isize)
                                            as UInt32 >> (j << 2 as libc::c_int & 0x4 as libc::c_int)
                                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int)
                                        as Int32;
                                    *((*s).ll16)
                                        .offset(j as isize) = (i & 0xffff as libc::c_int) as UInt16;
                                    if j & 0x1 as libc::c_int == 0 as libc::c_int {
                                        *((*s).ll4)
                                            .offset(
                                                (j >> 1 as libc::c_int) as isize,
                                            ) = (*((*s).ll4).offset((j >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf0 as libc::c_int
                                            | i >> 16 as libc::c_int) as UChar;
                                    } else {
                                        *((*s).ll4)
                                            .offset(
                                                (j >> 1 as libc::c_int) as isize,
                                            ) = (*((*s).ll4).offset((j >> 1 as libc::c_int) as isize)
                                            as libc::c_int & 0xf as libc::c_int
                                            | (i >> 16 as libc::c_int) << 4 as libc::c_int) as UChar;
                                    }
                                    i = j;
                                    j = tmp_0;
                                    if !(i != (*s).origPtr) {
                                        break;
                                    }
                                }
                                (*s).tPos = (*s).origPtr as UInt32;
                                (*s).nblock_used = 0 as libc::c_int;
                                if (*s).blockRandomised != 0 {
                                    (*s).rNToGo = 0 as libc::c_int;
                                    (*s).rTPos = 0 as libc::c_int;
                                    if (*s).tPos
                                        >= (100000 as libc::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as libc::c_int as Bool as Int32;
                                    }
                                    (*s)
                                        .k0 = BZ2_indexIntoF(
                                        (*s).tPos as Int32,
                                        ((*s).cftab).as_mut_ptr(),
                                    );
                                    (*s)
                                        .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                                        | (*((*s).ll4)
                                            .offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                                            >> ((*s).tPos << 2 as libc::c_int
                                                & 0x4 as libc::c_int as libc::c_uint)
                                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                                    let ref mut fresh208 = (*s).nblock_used;
                                    *fresh208 += 1;
                                    if (*s).rNToGo == 0 as libc::c_int {
                                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                                        let ref mut fresh209 = (*s).rTPos;
                                        *fresh209 += 1;
                                        if (*s).rTPos == 512 as libc::c_int {
                                            (*s).rTPos = 0 as libc::c_int;
                                        }
                                    }
                                    let ref mut fresh210 = (*s).rNToGo;
                                    *fresh210 -= 1;
                                    let ref mut fresh211 = (*s).k0;
                                    *fresh211
                                        ^= if (*s).rNToGo == 1 as libc::c_int {
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
                                    (*s)
                                        .k0 = BZ2_indexIntoF(
                                        (*s).tPos as Int32,
                                        ((*s).cftab).as_mut_ptr(),
                                    );
                                    (*s)
                                        .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                                        | (*((*s).ll4)
                                            .offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                                            >> ((*s).tPos << 2 as libc::c_int
                                                & 0x4 as libc::c_int as libc::c_uint)
                                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                                    let ref mut fresh212 = (*s).nblock_used;
                                    *fresh212 += 1;
                                }
                            } else {
                                i = 0 as libc::c_int;
                                while i < nblock {
                                    uc = (*((*s).tt).offset(i as isize)
                                        & 0xff as libc::c_int as libc::c_uint) as UChar;
                                    let ref mut fresh213 = *((*s).tt)
                                        .offset((*s).cftab[uc as usize] as isize);
                                    *fresh213 |= (i << 8 as libc::c_int) as libc::c_uint;
                                    let ref mut fresh214 = (*s).cftab[uc as usize];
                                    *fresh214 += 1;
                                    i += 1;
                                }
                                (*s)
                                    .tPos = *((*s).tt).offset((*s).origPtr as isize)
                                    >> 8 as libc::c_int;
                                (*s).nblock_used = 0 as libc::c_int;
                                if (*s).blockRandomised != 0 {
                                    (*s).rNToGo = 0 as libc::c_int;
                                    (*s).rTPos = 0 as libc::c_int;
                                    if (*s).tPos
                                        >= (100000 as libc::c_int as UInt32)
                                            .wrapping_mul((*s).blockSize100k as UInt32)
                                    {
                                        return 1 as libc::c_int as Bool as Int32;
                                    }
                                    (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                                    (*s)
                                        .k0 = ((*s).tPos & 0xff as libc::c_int as libc::c_uint)
                                        as UChar as Int32;
                                    (*s).tPos >>= 8 as libc::c_int;
                                    let ref mut fresh215 = (*s).nblock_used;
                                    *fresh215 += 1;
                                    if (*s).rNToGo == 0 as libc::c_int {
                                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                                        let ref mut fresh216 = (*s).rTPos;
                                        *fresh216 += 1;
                                        if (*s).rTPos == 512 as libc::c_int {
                                            (*s).rTPos = 0 as libc::c_int;
                                        }
                                    }
                                    let ref mut fresh217 = (*s).rNToGo;
                                    *fresh217 -= 1;
                                    let ref mut fresh218 = (*s).k0;
                                    *fresh218
                                        ^= if (*s).rNToGo == 1 as libc::c_int {
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
                                    (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                                    (*s)
                                        .k0 = ((*s).tPos & 0xff as libc::c_int as libc::c_uint)
                                        as UChar as Int32;
                                    (*s).tPos >>= 8 as libc::c_int;
                                    let ref mut fresh219 = (*s).nblock_used;
                                    *fresh219 += 1;
                                }
                            }
                            retVal = 0 as libc::c_int;
                            current_block = 6284263362753553522;
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
                    retVal = -(4 as libc::c_int);
                    current_block = 6284263362753553522;
                    continue;
                } else {
                    if nextSym == 0 as libc::c_int {
                        es = es + (0 as libc::c_int + 1 as libc::c_int) * N;
                    } else if nextSym == 1 as libc::c_int {
                        es = es + (1 as libc::c_int + 1 as libc::c_int) * N;
                    }
                    N = N * 2 as libc::c_int;
                    if groupPos == 0 as libc::c_int {
                        groupNo += 1;
                        if groupNo >= nSelectors {
                            retVal = -(4 as libc::c_int);
                            current_block = 6284263362753553522;
                            continue;
                        } else {
                            groupPos = 50 as libc::c_int;
                            gSel = (*s).selector[groupNo as usize] as Int32;
                            gMinlen = (*s).minLens[gSel as usize];
                            gLimit = &mut *(*((*s).limit)
                                .as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut Int32;
                            gPerm = &mut *(*((*s).perm)
                                .as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut Int32;
                            gBase = &mut *(*((*s).base)
                                .as_mut_ptr()
                                .offset(gSel as isize))
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut Int32;
                        }
                    }
                    groupPos -= 1;
                    zn = gMinlen;
                    current_block = 12417384991418392802;
                    continue;
                }
            }
            _ => {}
        }
        loop {
            match current_block {
                3854024847017804838 => {
                    if j < 16 as libc::c_int {
                        current_block = 14196862412491516613;
                        continue 'c_10057;
                    }
                }
                6591141407893725683 => {
                    if i < nSelectors {
                        j = 0 as libc::c_int;
                        current_block = 6927328446518169316;
                        continue;
                    } else {
                        if nSelectors
                            > 2 as libc::c_int
                                + 900000 as libc::c_int / 50 as libc::c_int
                        {
                            nSelectors = 2 as libc::c_int
                                + 900000 as libc::c_int / 50 as libc::c_int;
                        }
                        let mut pos: [UChar; 6] = [0; 6];
                        let mut tmp: UChar = 0;
                        let mut v_22: UChar = 0;
                        v_22 = 0 as libc::c_int as UChar;
                        while (v_22 as libc::c_int) < nGroups {
                            pos[v_22 as usize] = v_22;
                            v_22 = v_22.wrapping_add(1);
                        }
                        i = 0 as libc::c_int;
                        while i < nSelectors {
                            v_22 = (*s).selectorMtf[i as usize];
                            tmp = pos[v_22 as usize];
                            while v_22 as libc::c_int > 0 as libc::c_int {
                                pos[v_22
                                    as usize] = pos[(v_22 as libc::c_int - 1 as libc::c_int)
                                    as usize];
                                v_22 = v_22.wrapping_sub(1);
                            }
                            pos[0 as libc::c_int as usize] = tmp;
                            (*s).selector[i as usize] = tmp;
                            i += 1;
                        }
                        t = 0 as libc::c_int;
                        current_block = 16916874950763617094;
                        break;
                    }
                }
                3472349144349095221 => {
                    if i < 16 as libc::c_int {
                        if (*s).inUse16[i as usize] != 0 {
                            j = 0 as libc::c_int;
                            current_block = 3854024847017804838;
                            continue;
                        }
                    } else {
                        makeMaps_d(s);
                        if (*s).nInUse == 0 as libc::c_int {
                            current_block = 11906008669688594715;
                            break;
                        } else {
                            current_block = 7606051654693192361;
                            break;
                        }
                    }
                }
                17503523010989424999 => {
                    (*s).len[t as usize][i as usize] = curr as UChar;
                    i += 1;
                    current_block = 3770765986603902964;
                    continue;
                }
                3770765986603902964 => {
                    if i < alphaSize {
                        current_block = 11858046780433112516;
                        continue;
                    }
                    t += 1;
                    current_block = 16916874950763617094;
                    break;
                }
                5281038271658253520 => {
                    if i < 2 as libc::c_int + 900000 as libc::c_int / 50 as libc::c_int {
                        (*s).selectorMtf[i as usize] = j as UChar;
                    }
                    i += 1;
                    current_block = 6591141407893725683;
                    continue;
                }
                6927328446518169316 => {
                    if 1 as libc::c_int as Bool != 0 {
                        current_block = 13333503772902270734;
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
                        current_block = 1232411364073810230;
                        continue 'c_10057;
                    }
                    retVal = -(4 as libc::c_int);
                    current_block = 6284263362753553522;
                    continue 'c_10057;
                }
            }
            i += 1;
            current_block = 3472349144349095221;
        }
        match current_block {
            7606051654693192361 => {
                alphaSize = (*s).nInUse + 2 as libc::c_int;
                current_block = 6492128765584295426;
            }
            11906008669688594715 => {
                retVal = -(4 as libc::c_int);
                current_block = 6284263362753553522;
            }
            _ => {
                if t < nGroups {
                    current_block = 12765707770930006169;
                    continue;
                }
                t = 0 as libc::c_int;
                while t < nGroups {
                    minLen = 32 as libc::c_int;
                    maxLen = 0 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < alphaSize {
                        if (*s).len[t as usize][i as usize] as libc::c_int > maxLen {
                            maxLen = (*s).len[t as usize][i as usize] as Int32;
                        }
                        if ((*s).len[t as usize][i as usize] as libc::c_int) < minLen {
                            minLen = (*s).len[t as usize][i as usize] as Int32;
                        }
                        i += 1;
                    }
                    BZ2_hbCreateDecodeTables(
                        &mut *(*((*s).limit).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        &mut *(*((*s).base).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        &mut *(*((*s).perm).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        minLen,
                        maxLen,
                        alphaSize,
                    );
                    (*s).minLens[t as usize] = minLen;
                    t += 1;
                }
                EOB = (*s).nInUse + 1 as libc::c_int;
                nblockMAX = 100000 as libc::c_int * (*s).blockSize100k;
                groupNo = -(1 as libc::c_int);
                groupPos = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i <= 255 as libc::c_int {
                    (*s).unzftab[i as usize] = 0 as libc::c_int;
                    i += 1;
                }
                let mut ii: Int32 = 0;
                let mut jj: Int32 = 0;
                let mut kk: Int32 = 0;
                kk = 4096 as libc::c_int - 1 as libc::c_int;
                ii = 256 as libc::c_int / 16 as libc::c_int - 1 as libc::c_int;
                while ii >= 0 as libc::c_int {
                    jj = 16 as libc::c_int - 1 as libc::c_int;
                    while jj >= 0 as libc::c_int {
                        (*s).mtfa[kk as usize] = (ii * 16 as libc::c_int + jj) as UChar;
                        kk -= 1;
                        jj -= 1;
                    }
                    (*s).mtfbase[ii as usize] = kk + 1 as libc::c_int;
                    ii -= 1;
                }
                nblock = 0 as libc::c_int;
                if groupPos == 0 as libc::c_int {
                    groupNo += 1;
                    if groupNo >= nSelectors {
                        retVal = -(4 as libc::c_int);
                        current_block = 6284263362753553522;
                        continue;
                    } else {
                        groupPos = 50 as libc::c_int;
                        gSel = (*s).selector[groupNo as usize] as Int32;
                        gMinlen = (*s).minLens[gSel as usize];
                        gLimit = &mut *(*((*s).limit).as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut Int32;
                        gPerm = &mut *(*((*s).perm).as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut Int32;
                        gBase = &mut *(*((*s).base).as_mut_ptr().offset(gSel as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize) as *mut Int32;
                    }
                }
                groupPos -= 1;
                zn = gMinlen;
                current_block = 13427822320537439643;
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
    let ref mut fresh274 = (*s).save_gLimit;
    *fresh274 = gLimit;
    let ref mut fresh275 = (*s).save_gBase;
    *fresh275 = gBase;
    let ref mut fresh276 = (*s).save_gPerm;
    *fresh276 = gPerm;
    return retVal;
}
