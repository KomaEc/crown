use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn BZ2_bz__AssertH__fail(errcode: libc::c_int);
    fn BZ2_blockSort(_: *mut EState);
    fn BZ2_hbAssignCodes(_: *mut Int32, _: *mut UChar, _: Int32, _: Int32, _: Int32);
    fn BZ2_hbMakeCodeLengths(_: *mut UChar, _: *mut Int32, _: Int32, _: Int32);
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
#[no_mangle]
pub unsafe extern "C" fn BZ2_bsInitWrite(mut s: *mut EState) {
    (*s).bsLive = 0 as libc::c_int;
    (*s).bsBuff = 0 as libc::c_int as UInt32;
}
unsafe extern "C" fn bsFinishWrite(mut s: *mut EState) {
    while (*s).bsLive > 0 as libc::c_int {
        *((*s).zbits)
            .offset((*s).numZ as isize) = ((*s).bsBuff >> 24 as libc::c_int) as UChar;
        let ref mut fresh0 = (*s).numZ;
        *fresh0 += 1;
        (*s).bsBuff <<= 8 as libc::c_int;
        let ref mut fresh1 = (*s).bsLive;
        *fresh1 -= 8 as libc::c_int;
    }
}
#[inline]
unsafe extern "C" fn bsW(mut s: *mut EState, mut n: Int32, mut v: UInt32) {
    while (*s).bsLive >= 8 as libc::c_int {
        *((*s).zbits)
            .offset((*s).numZ as isize) = ((*s).bsBuff >> 24 as libc::c_int) as UChar;
        let ref mut fresh2 = (*s).numZ;
        *fresh2 += 1;
        (*s).bsBuff <<= 8 as libc::c_int;
        let ref mut fresh3 = (*s).bsLive;
        *fresh3 -= 8 as libc::c_int;
    }
    let ref mut fresh4 = (*s).bsBuff;
    *fresh4 |= v << 32 as libc::c_int - (*s).bsLive - n;
    let ref mut fresh5 = (*s).bsLive;
    *fresh5 += n;
}
unsafe extern "C" fn bsPutUInt32(mut s: *mut EState, mut u: UInt32) {
    bsW(
        s,
        8 as libc::c_int,
        ((u >> 24 as libc::c_int) as libc::c_long & 0xff as libc::c_long) as UInt32,
    );
    bsW(
        s,
        8 as libc::c_int,
        ((u >> 16 as libc::c_int) as libc::c_long & 0xff as libc::c_long) as UInt32,
    );
    bsW(
        s,
        8 as libc::c_int,
        ((u >> 8 as libc::c_int) as libc::c_long & 0xff as libc::c_long) as UInt32,
    );
    bsW(s, 8 as libc::c_int, (u as libc::c_long & 0xff as libc::c_long) as UInt32);
}
unsafe extern "C" fn bsPutUChar(mut s: *mut EState, mut c: UChar) {
    bsW(s, 8 as libc::c_int, c as UInt32);
}
unsafe extern "C" fn makeMaps_e(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nInUse = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if (*s).inUse[i as usize] != 0 {
            (*s).unseqToSeq[i as usize] = (*s).nInUse as UChar;
            let ref mut fresh6 = (*s).nInUse;
            *fresh6 += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn generateMTFValues(mut s: *mut EState) {
    let mut yy: [UChar; 256] = [0; 256];
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut zPend: Int32 = 0;
    let mut wr: Int32 = 0;
    let mut EOB: Int32 = 0;
    let mut ptr = (*s).ptr;
    let mut block = (*s).block;
    let mut mtfv = (*s).mtfv;
    makeMaps_e(s);
    EOB = (*s).nInUse + 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= EOB {
        (*s).mtfFreq[i as usize] = 0 as libc::c_int;
        i += 1;
    }
    wr = 0 as libc::c_int;
    zPend = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*s).nInUse {
        yy[i as usize] = i as UChar;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*s).nblock {
        let mut ll_i: UChar = 0;
        j = (*ptr.offset(i as isize)).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as Int32;
        if j < 0 as libc::c_int {
            j += (*s).nblock;
        }
        ll_i = (*s).unseqToSeq[*block.offset(j as isize) as usize];
        if yy[0 as libc::c_int as usize] as libc::c_int == ll_i as libc::c_int {
            zPend += 1;
        } else {
            if zPend > 0 as libc::c_int {
                zPend -= 1;
                while 1 as libc::c_int as Bool != 0 {
                    if zPend & 1 as libc::c_int != 0 {
                        *mtfv.offset(wr as isize) = 1 as libc::c_int as UInt16;
                        wr += 1;
                        let ref mut fresh7 = (*s).mtfFreq[1 as libc::c_int as usize];
                        *fresh7 += 1;
                    } else {
                        *mtfv.offset(wr as isize) = 0 as libc::c_int as UInt16;
                        wr += 1;
                        let ref mut fresh8 = (*s).mtfFreq[0 as libc::c_int as usize];
                        *fresh8 += 1;
                    }
                    if zPend < 2 as libc::c_int {
                        break;
                    }
                    zPend = (zPend - 2 as libc::c_int) / 2 as libc::c_int;
                }
                zPend = 0 as libc::c_int;
            }
            let mut rtmp: UChar = 0;
            let mut ryy_j = 0 as *mut UChar;
            let mut rll_i: UChar = 0;
            rtmp = yy[1 as libc::c_int as usize];
            yy[1 as libc::c_int as usize] = yy[0 as libc::c_int as usize];
            ryy_j = &mut *yy.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut UChar;
            rll_i = ll_i;
            while rll_i as libc::c_int != rtmp as libc::c_int {
                let mut rtmp2: UChar = 0;
                ryy_j = ryy_j.offset(1);
                rtmp2 = rtmp;
                rtmp = *ryy_j;
                *ryy_j = rtmp2;
            }
            yy[0 as libc::c_int as usize] = rtmp;
            j = ryy_j
                .offset_from(
                    &mut *yy.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut UChar,
                ) as libc::c_long as Int32;
            *mtfv.offset(wr as isize) = (j + 1 as libc::c_int) as UInt16;
            wr += 1;
            let ref mut fresh9 = (*s).mtfFreq[(j + 1 as libc::c_int) as usize];
            *fresh9 += 1;
        }
        i += 1;
    }
    if zPend > 0 as libc::c_int {
        zPend -= 1;
        while 1 as libc::c_int as Bool != 0 {
            if zPend & 1 as libc::c_int != 0 {
                *mtfv.offset(wr as isize) = 1 as libc::c_int as UInt16;
                wr += 1;
                let ref mut fresh10 = (*s).mtfFreq[1 as libc::c_int as usize];
                *fresh10 += 1;
            } else {
                *mtfv.offset(wr as isize) = 0 as libc::c_int as UInt16;
                wr += 1;
                let ref mut fresh11 = (*s).mtfFreq[0 as libc::c_int as usize];
                *fresh11 += 1;
            }
            if zPend < 2 as libc::c_int {
                break;
            }
            zPend = (zPend - 2 as libc::c_int) / 2 as libc::c_int;
        }
        zPend = 0 as libc::c_int;
    }
    *mtfv.offset(wr as isize) = EOB as UInt16;
    wr += 1;
    let ref mut fresh12 = (*s).mtfFreq[EOB as usize];
    *fresh12 += 1;
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
    let mut cost: [UInt16; 6] = [0; 6];
    let mut fave: [Int32; 6] = [0; 6];
    let mut mtfv = (*s).mtfv;
    if (*s).verbosity >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"      %d in block, %d after MTF & 1-2 coding, %d+2 syms in use\n\0"
                as *const u8 as *const libc::c_char,
            (*s).nblock,
            (*s).nMTF,
            (*s).nInUse,
        );
    }
    alphaSize = (*s).nInUse + 2 as libc::c_int;
    t = 0 as libc::c_int;
    while t < 6 as libc::c_int {
        v = 0 as libc::c_int;
        while v < alphaSize {
            (*s).len[t as usize][v as usize] = 15 as libc::c_int as UChar;
            v += 1;
        }
        t += 1;
    }
    if !((*s).nMTF > 0 as libc::c_int) {
        BZ2_bz__AssertH__fail(3001 as libc::c_int);
    }
    if (*s).nMTF < 200 as libc::c_int {
        nGroups = 2 as libc::c_int;
    } else if (*s).nMTF < 600 as libc::c_int {
        nGroups = 3 as libc::c_int;
    } else if (*s).nMTF < 1200 as libc::c_int {
        nGroups = 4 as libc::c_int;
    } else if (*s).nMTF < 2400 as libc::c_int {
        nGroups = 5 as libc::c_int;
    } else {
        nGroups = 6 as libc::c_int;
    }
    let mut nPart: Int32 = 0;
    let mut remF: Int32 = 0;
    let mut tFreq: Int32 = 0;
    let mut aFreq: Int32 = 0;
    nPart = nGroups;
    remF = (*s).nMTF;
    gs = 0 as libc::c_int;
    while nPart > 0 as libc::c_int {
        tFreq = remF / nPart;
        ge = gs - 1 as libc::c_int;
        aFreq = 0 as libc::c_int;
        while aFreq < tFreq && ge < alphaSize - 1 as libc::c_int {
            ge += 1;
            aFreq += (*s).mtfFreq[ge as usize];
        }
        if ge > gs && nPart != nGroups && nPart != 1 as libc::c_int
            && (nGroups - nPart) % 2 as libc::c_int == 1 as libc::c_int
        {
            aFreq -= (*s).mtfFreq[ge as usize];
            ge -= 1;
        }
        if (*s).verbosity >= 3 as libc::c_int {
            fprintf(
                stderr,
                b"      initial group %d, [%d .. %d], has %d syms (%4.1f%%)\n\0"
                    as *const u8 as *const libc::c_char,
                nPart,
                gs,
                ge,
                aFreq,
                100.0f64 * aFreq as libc::c_float as libc::c_double
                    / (*s).nMTF as libc::c_float as libc::c_double,
            );
        }
        v = 0 as libc::c_int;
        while v < alphaSize {
            if v >= gs && v <= ge {
                (*s)
                    .len[(nPart - 1 as libc::c_int)
                    as usize][v as usize] = 0 as libc::c_int as UChar;
            } else {
                (*s)
                    .len[(nPart - 1 as libc::c_int)
                    as usize][v as usize] = 15 as libc::c_int as UChar;
            }
            v += 1;
        }
        nPart -= 1;
        gs = ge + 1 as libc::c_int;
        remF -= aFreq;
    }
    iter = 0 as libc::c_int;
    while iter < 4 as libc::c_int {
        t = 0 as libc::c_int;
        while t < nGroups {
            fave[t as usize] = 0 as libc::c_int;
            t += 1;
        }
        t = 0 as libc::c_int;
        while t < nGroups {
            v = 0 as libc::c_int;
            while v < alphaSize {
                (*s).rfreq[t as usize][v as usize] = 0 as libc::c_int;
                v += 1;
            }
            t += 1;
        }
        if nGroups == 6 as libc::c_int {
            v = 0 as libc::c_int;
            while v < alphaSize {
                (*s)
                    .len_pack[v
                    as usize][0 as libc::c_int
                    as usize] = (((*s).len[1 as libc::c_int as usize][v as usize]
                    as libc::c_int) << 16 as libc::c_int
                    | (*s).len[0 as libc::c_int as usize][v as usize] as libc::c_int)
                    as UInt32;
                (*s)
                    .len_pack[v
                    as usize][1 as libc::c_int
                    as usize] = (((*s).len[3 as libc::c_int as usize][v as usize]
                    as libc::c_int) << 16 as libc::c_int
                    | (*s).len[2 as libc::c_int as usize][v as usize] as libc::c_int)
                    as UInt32;
                (*s)
                    .len_pack[v
                    as usize][2 as libc::c_int
                    as usize] = (((*s).len[5 as libc::c_int as usize][v as usize]
                    as libc::c_int) << 16 as libc::c_int
                    | (*s).len[4 as libc::c_int as usize][v as usize] as libc::c_int)
                    as UInt32;
                v += 1;
            }
        }
        nSelectors = 0 as libc::c_int;
        totc = 0 as libc::c_int;
        gs = 0 as libc::c_int;
        while 1 as libc::c_int as Bool != 0 {
            if gs >= (*s).nMTF {
                break;
            }
            ge = gs + 50 as libc::c_int - 1 as libc::c_int;
            if ge >= (*s).nMTF {
                ge = (*s).nMTF - 1 as libc::c_int;
            }
            t = 0 as libc::c_int;
            while t < nGroups {
                cost[t as usize] = 0 as libc::c_int as UInt16;
                t += 1;
            }
            if nGroups == 6 as libc::c_int
                && 50 as libc::c_int == ge - gs + 1 as libc::c_int
            {
                let mut cost01: UInt32 = 0;
                let mut cost23: UInt32 = 0;
                let mut cost45: UInt32 = 0;
                let mut icv: UInt16 = 0;
                cost45 = 0 as libc::c_int as UInt32;
                cost23 = cost45;
                cost01 = cost23;
                icv = *mtfv.offset((gs + 0 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 1 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 2 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 3 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 4 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 5 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 6 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 7 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 8 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 9 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 10 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 11 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 12 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 13 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 14 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 15 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 16 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 17 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 18 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 19 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 20 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 21 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 22 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 23 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 24 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 25 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 26 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 27 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 28 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 29 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 30 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 31 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 32 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 33 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 34 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 35 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 36 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 37 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 38 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 39 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 40 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 41 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 42 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 43 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 44 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 45 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 46 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 47 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 48 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                icv = *mtfv.offset((gs + 49 as libc::c_int) as isize);
                cost01 = (cost01 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][0 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost23 = (cost23 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][1 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost45 = (cost45 as libc::c_uint)
                    .wrapping_add((*s).len_pack[icv as usize][2 as libc::c_int as usize])
                    as UInt32 as UInt32;
                cost[0 as libc::c_int
                    as usize] = (cost01 & 0xffff as libc::c_int as libc::c_uint)
                    as UInt16;
                cost[1 as libc::c_int
                    as usize] = (cost01 >> 16 as libc::c_int) as UInt16;
                cost[2 as libc::c_int
                    as usize] = (cost23 & 0xffff as libc::c_int as libc::c_uint)
                    as UInt16;
                cost[3 as libc::c_int
                    as usize] = (cost23 >> 16 as libc::c_int) as UInt16;
                cost[4 as libc::c_int
                    as usize] = (cost45 & 0xffff as libc::c_int as libc::c_uint)
                    as UInt16;
                cost[5 as libc::c_int
                    as usize] = (cost45 >> 16 as libc::c_int) as UInt16;
            } else {
                i = gs;
                while i <= ge {
                    let mut icv_0 = *mtfv.offset(i as isize);
                    t = 0 as libc::c_int;
                    while t < nGroups {
                        cost[t
                            as usize] = (cost[t as usize] as libc::c_int
                            + (*s).len[t as usize][icv_0 as usize] as libc::c_int)
                            as UInt16;
                        t += 1;
                    }
                    i += 1;
                }
            }
            bc = 999999999 as libc::c_int;
            bt = -(1 as libc::c_int);
            t = 0 as libc::c_int;
            while t < nGroups {
                if (cost[t as usize] as libc::c_int) < bc {
                    bc = cost[t as usize] as Int32;
                    bt = t;
                }
                t += 1;
            }
            totc += bc;
            fave[bt as usize] += 1;
            (*s).selector[nSelectors as usize] = bt as UChar;
            nSelectors += 1;
            if nGroups == 6 as libc::c_int
                && 50 as libc::c_int == ge - gs + 1 as libc::c_int
            {
                let ref mut fresh13 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 0 as libc::c_int) as isize) as usize];
                *fresh13 += 1;
                let ref mut fresh14 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 1 as libc::c_int) as isize) as usize];
                *fresh14 += 1;
                let ref mut fresh15 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 2 as libc::c_int) as isize) as usize];
                *fresh15 += 1;
                let ref mut fresh16 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 3 as libc::c_int) as isize) as usize];
                *fresh16 += 1;
                let ref mut fresh17 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 4 as libc::c_int) as isize) as usize];
                *fresh17 += 1;
                let ref mut fresh18 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 5 as libc::c_int) as isize) as usize];
                *fresh18 += 1;
                let ref mut fresh19 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 6 as libc::c_int) as isize) as usize];
                *fresh19 += 1;
                let ref mut fresh20 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 7 as libc::c_int) as isize) as usize];
                *fresh20 += 1;
                let ref mut fresh21 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 8 as libc::c_int) as isize) as usize];
                *fresh21 += 1;
                let ref mut fresh22 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 9 as libc::c_int) as isize) as usize];
                *fresh22 += 1;
                let ref mut fresh23 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 10 as libc::c_int) as isize) as usize];
                *fresh23 += 1;
                let ref mut fresh24 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 11 as libc::c_int) as isize) as usize];
                *fresh24 += 1;
                let ref mut fresh25 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 12 as libc::c_int) as isize) as usize];
                *fresh25 += 1;
                let ref mut fresh26 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 13 as libc::c_int) as isize) as usize];
                *fresh26 += 1;
                let ref mut fresh27 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 14 as libc::c_int) as isize) as usize];
                *fresh27 += 1;
                let ref mut fresh28 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 15 as libc::c_int) as isize) as usize];
                *fresh28 += 1;
                let ref mut fresh29 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 16 as libc::c_int) as isize) as usize];
                *fresh29 += 1;
                let ref mut fresh30 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 17 as libc::c_int) as isize) as usize];
                *fresh30 += 1;
                let ref mut fresh31 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 18 as libc::c_int) as isize) as usize];
                *fresh31 += 1;
                let ref mut fresh32 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 19 as libc::c_int) as isize) as usize];
                *fresh32 += 1;
                let ref mut fresh33 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 20 as libc::c_int) as isize) as usize];
                *fresh33 += 1;
                let ref mut fresh34 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 21 as libc::c_int) as isize) as usize];
                *fresh34 += 1;
                let ref mut fresh35 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 22 as libc::c_int) as isize) as usize];
                *fresh35 += 1;
                let ref mut fresh36 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 23 as libc::c_int) as isize) as usize];
                *fresh36 += 1;
                let ref mut fresh37 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 24 as libc::c_int) as isize) as usize];
                *fresh37 += 1;
                let ref mut fresh38 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 25 as libc::c_int) as isize) as usize];
                *fresh38 += 1;
                let ref mut fresh39 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 26 as libc::c_int) as isize) as usize];
                *fresh39 += 1;
                let ref mut fresh40 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 27 as libc::c_int) as isize) as usize];
                *fresh40 += 1;
                let ref mut fresh41 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 28 as libc::c_int) as isize) as usize];
                *fresh41 += 1;
                let ref mut fresh42 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 29 as libc::c_int) as isize) as usize];
                *fresh42 += 1;
                let ref mut fresh43 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 30 as libc::c_int) as isize) as usize];
                *fresh43 += 1;
                let ref mut fresh44 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 31 as libc::c_int) as isize) as usize];
                *fresh44 += 1;
                let ref mut fresh45 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 32 as libc::c_int) as isize) as usize];
                *fresh45 += 1;
                let ref mut fresh46 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 33 as libc::c_int) as isize) as usize];
                *fresh46 += 1;
                let ref mut fresh47 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 34 as libc::c_int) as isize) as usize];
                *fresh47 += 1;
                let ref mut fresh48 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 35 as libc::c_int) as isize) as usize];
                *fresh48 += 1;
                let ref mut fresh49 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 36 as libc::c_int) as isize) as usize];
                *fresh49 += 1;
                let ref mut fresh50 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 37 as libc::c_int) as isize) as usize];
                *fresh50 += 1;
                let ref mut fresh51 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 38 as libc::c_int) as isize) as usize];
                *fresh51 += 1;
                let ref mut fresh52 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 39 as libc::c_int) as isize) as usize];
                *fresh52 += 1;
                let ref mut fresh53 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 40 as libc::c_int) as isize) as usize];
                *fresh53 += 1;
                let ref mut fresh54 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 41 as libc::c_int) as isize) as usize];
                *fresh54 += 1;
                let ref mut fresh55 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 42 as libc::c_int) as isize) as usize];
                *fresh55 += 1;
                let ref mut fresh56 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 43 as libc::c_int) as isize) as usize];
                *fresh56 += 1;
                let ref mut fresh57 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 44 as libc::c_int) as isize) as usize];
                *fresh57 += 1;
                let ref mut fresh58 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 45 as libc::c_int) as isize) as usize];
                *fresh58 += 1;
                let ref mut fresh59 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 46 as libc::c_int) as isize) as usize];
                *fresh59 += 1;
                let ref mut fresh60 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 47 as libc::c_int) as isize) as usize];
                *fresh60 += 1;
                let ref mut fresh61 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 48 as libc::c_int) as isize) as usize];
                *fresh61 += 1;
                let ref mut fresh62 = (*s)
                    .rfreq[bt
                    as usize][*mtfv.offset((gs + 49 as libc::c_int) as isize) as usize];
                *fresh62 += 1;
            } else {
                i = gs;
                while i <= ge {
                    let ref mut fresh63 = (*s)
                        .rfreq[bt as usize][*mtfv.offset(i as isize) as usize];
                    *fresh63 += 1;
                    i += 1;
                }
            }
            gs = ge + 1 as libc::c_int;
        }
        if (*s).verbosity >= 3 as libc::c_int {
            fprintf(
                stderr,
                b"      pass %d: size is %d, grp uses are \0" as *const u8
                    as *const libc::c_char,
                iter + 1 as libc::c_int,
                totc / 8 as libc::c_int,
            );
            t = 0 as libc::c_int;
            while t < nGroups {
                fprintf(
                    stderr,
                    b"%d \0" as *const u8 as *const libc::c_char,
                    fave[t as usize],
                );
                t += 1;
            }
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
        t = 0 as libc::c_int;
        while t < nGroups {
            BZ2_hbMakeCodeLengths(
                &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
                &mut *(*((*s).rfreq).as_mut_ptr().offset(t as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize),
                alphaSize,
                17 as libc::c_int,
            );
            t += 1;
        }
        iter += 1;
    }
    if !(nGroups < 8 as libc::c_int) {
        BZ2_bz__AssertH__fail(3002 as libc::c_int);
    }
    if !(nSelectors < 32768 as libc::c_int
        && nSelectors <= 2 as libc::c_int + 900000 as libc::c_int / 50 as libc::c_int)
    {
        BZ2_bz__AssertH__fail(3003 as libc::c_int);
    }
    let mut pos: [UChar; 6] = [0; 6];
    let mut ll_i: UChar = 0;
    let mut tmp2: UChar = 0;
    let mut tmp: UChar = 0;
    i = 0 as libc::c_int;
    while i < nGroups {
        pos[i as usize] = i as UChar;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < nSelectors {
        ll_i = (*s).selector[i as usize];
        j = 0 as libc::c_int;
        tmp = pos[j as usize];
        while ll_i as libc::c_int != tmp as libc::c_int {
            j += 1;
            tmp2 = tmp;
            tmp = pos[j as usize];
            pos[j as usize] = tmp2;
        }
        pos[0 as libc::c_int as usize] = tmp;
        (*s).selectorMtf[i as usize] = j as UChar;
        i += 1;
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
        if maxLen > 17 as libc::c_int {
            BZ2_bz__AssertH__fail(3004 as libc::c_int);
        }
        if minLen < 1 as libc::c_int {
            BZ2_bz__AssertH__fail(3005 as libc::c_int);
        }
        BZ2_hbAssignCodes(
            &mut *(*((*s).code).as_mut_ptr().offset(t as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            &mut *(*((*s).len).as_mut_ptr().offset(t as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            minLen,
            maxLen,
            alphaSize,
        );
        t += 1;
    }
    let mut inUse16: [Bool; 16] = [0; 16];
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        inUse16[i as usize] = 0 as libc::c_int as Bool;
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            if (*s).inUse[(i * 16 as libc::c_int + j) as usize] != 0 {
                inUse16[i as usize] = 1 as libc::c_int as Bool;
            }
            j += 1;
        }
        i += 1;
    }
    nBytes = (*s).numZ;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if inUse16[i as usize] != 0 {
            bsW(s, 1 as libc::c_int, 1 as libc::c_int as UInt32);
        } else {
            bsW(s, 1 as libc::c_int, 0 as libc::c_int as UInt32);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if inUse16[i as usize] != 0 {
            j = 0 as libc::c_int;
            while j < 16 as libc::c_int {
                if (*s).inUse[(i * 16 as libc::c_int + j) as usize] != 0 {
                    bsW(s, 1 as libc::c_int, 1 as libc::c_int as UInt32);
                } else {
                    bsW(s, 1 as libc::c_int, 0 as libc::c_int as UInt32);
                }
                j += 1;
            }
        }
        i += 1;
    }
    if (*s).verbosity >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"      bytes: mapping %d, \0" as *const u8 as *const libc::c_char,
            (*s).numZ - nBytes,
        );
    }
    nBytes = (*s).numZ;
    bsW(s, 3 as libc::c_int, nGroups as UInt32);
    bsW(s, 15 as libc::c_int, nSelectors as UInt32);
    i = 0 as libc::c_int;
    while i < nSelectors {
        j = 0 as libc::c_int;
        while j < (*s).selectorMtf[i as usize] as libc::c_int {
            bsW(s, 1 as libc::c_int, 1 as libc::c_int as UInt32);
            j += 1;
        }
        bsW(s, 1 as libc::c_int, 0 as libc::c_int as UInt32);
        i += 1;
    }
    if (*s).verbosity >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"selectors %d, \0" as *const u8 as *const libc::c_char,
            (*s).numZ - nBytes,
        );
    }
    nBytes = (*s).numZ;
    t = 0 as libc::c_int;
    while t < nGroups {
        let mut curr = (*s).len[t as usize][0 as libc::c_int as usize] as Int32;
        bsW(s, 5 as libc::c_int, curr as UInt32);
        i = 0 as libc::c_int;
        while i < alphaSize {
            while curr < (*s).len[t as usize][i as usize] as libc::c_int {
                bsW(s, 2 as libc::c_int, 2 as libc::c_int as UInt32);
                curr += 1;
            }
            while curr > (*s).len[t as usize][i as usize] as libc::c_int {
                bsW(s, 2 as libc::c_int, 3 as libc::c_int as UInt32);
                curr -= 1;
            }
            bsW(s, 1 as libc::c_int, 0 as libc::c_int as UInt32);
            i += 1;
        }
        t += 1;
    }
    if (*s).verbosity >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"code lengths %d, \0" as *const u8 as *const libc::c_char,
            (*s).numZ - nBytes,
        );
    }
    nBytes = (*s).numZ;
    selCtr = 0 as libc::c_int;
    gs = 0 as libc::c_int;
    while 1 as libc::c_int as Bool != 0 {
        if gs >= (*s).nMTF {
            break;
        }
        ge = gs + 50 as libc::c_int - 1 as libc::c_int;
        if ge >= (*s).nMTF {
            ge = (*s).nMTF - 1 as libc::c_int;
        }
        if !(((*s).selector[selCtr as usize] as libc::c_int) < nGroups) {
            BZ2_bz__AssertH__fail(3006 as libc::c_int);
        }
        if nGroups == 6 as libc::c_int && 50 as libc::c_int == ge - gs + 1 as libc::c_int
        {
            let mut mtfv_i: UInt16 = 0;
            let mut s_len_sel_selCtr: *mut UChar = &mut *(*((*s).len)
                .as_mut_ptr()
                .offset(*((*s).selector).as_mut_ptr().offset(selCtr as isize) as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut UChar;
            let mut s_code_sel_selCtr: *mut Int32 = &mut *(*((*s).code)
                .as_mut_ptr()
                .offset(*((*s).selector).as_mut_ptr().offset(selCtr as isize) as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut Int32;
            mtfv_i = *mtfv.offset((gs + 0 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 1 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 2 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 3 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 4 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 5 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 6 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 7 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 8 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 9 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 10 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 11 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 12 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 13 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 14 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 15 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 16 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 17 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 18 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 19 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 20 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 21 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 22 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 23 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 24 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 25 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 26 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 27 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 28 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 29 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 30 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 31 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 32 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 33 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 34 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 35 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 36 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 37 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 38 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 39 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 40 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 41 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 42 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 43 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 44 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 45 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 46 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 47 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 48 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
            mtfv_i = *mtfv.offset((gs + 49 as libc::c_int) as isize);
            bsW(
                s,
                *s_len_sel_selCtr.offset(mtfv_i as isize) as Int32,
                *s_code_sel_selCtr.offset(mtfv_i as isize) as UInt32,
            );
        } else {
            i = gs;
            while i <= ge {
                bsW(
                    s,
                    (*s)
                        .len[(*s).selector[selCtr as usize]
                        as usize][*mtfv.offset(i as isize) as usize] as Int32,
                    (*s)
                        .code[(*s).selector[selCtr as usize]
                        as usize][*mtfv.offset(i as isize) as usize] as UInt32,
                );
                i += 1;
            }
        }
        gs = ge + 1 as libc::c_int;
        selCtr += 1;
    }
    if !(selCtr == nSelectors) {
        BZ2_bz__AssertH__fail(3007 as libc::c_int);
    }
    if (*s).verbosity >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"codes %d\n\0" as *const u8 as *const libc::c_char,
            (*s).numZ - nBytes,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_compressBlock(mut s: *mut EState, mut is_last_block: Bool) {
    if (*s).nblock > 0 as libc::c_int {
        (*s).blockCRC = !(*s).blockCRC;
        (*s)
            .combinedCRC = (*s).combinedCRC << 1 as libc::c_int
            | (*s).combinedCRC >> 31 as libc::c_int;
        let ref mut fresh64 = (*s).combinedCRC;
        *fresh64 ^= (*s).blockCRC;
        if (*s).blockNo > 1 as libc::c_int {
            (*s).numZ = 0 as libc::c_int;
        }
        if (*s).verbosity >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"    block %d: crc = 0x%08x, combined CRC = 0x%08x, size = %d\n\0"
                    as *const u8 as *const libc::c_char,
                (*s).blockNo,
                (*s).blockCRC,
                (*s).combinedCRC,
                (*s).nblock,
            );
        }
        BZ2_blockSort(s);
    }
    let ref mut fresh65 = (*s).zbits;
    *fresh65 = &mut *((*s).arr2 as *mut UChar).offset((*s).nblock as isize)
        as *mut UChar;
    if (*s).blockNo == 1 as libc::c_int {
        BZ2_bsInitWrite(s);
        bsPutUChar(s, 0x42 as libc::c_int as UChar);
        bsPutUChar(s, 0x5a as libc::c_int as UChar);
        bsPutUChar(s, 0x68 as libc::c_int as UChar);
        bsPutUChar(s, (0x30 as libc::c_int + (*s).blockSize100k) as UChar);
    }
    if (*s).nblock > 0 as libc::c_int {
        bsPutUChar(s, 0x31 as libc::c_int as UChar);
        bsPutUChar(s, 0x41 as libc::c_int as UChar);
        bsPutUChar(s, 0x59 as libc::c_int as UChar);
        bsPutUChar(s, 0x26 as libc::c_int as UChar);
        bsPutUChar(s, 0x53 as libc::c_int as UChar);
        bsPutUChar(s, 0x59 as libc::c_int as UChar);
        bsPutUInt32(s, (*s).blockCRC);
        bsW(s, 1 as libc::c_int, 0 as libc::c_int as UInt32);
        bsW(s, 24 as libc::c_int, (*s).origPtr as UInt32);
        generateMTFValues(s);
        sendMTFValues(s);
    }
    if is_last_block != 0 {
        bsPutUChar(s, 0x17 as libc::c_int as UChar);
        bsPutUChar(s, 0x72 as libc::c_int as UChar);
        bsPutUChar(s, 0x45 as libc::c_int as UChar);
        bsPutUChar(s, 0x38 as libc::c_int as UChar);
        bsPutUChar(s, 0x50 as libc::c_int as UChar);
        bsPutUChar(s, 0x90 as libc::c_int as UChar);
        bsPutUInt32(s, (*s).combinedCRC);
        if (*s).verbosity >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"    final combined CRC = 0x%08x\n   \0" as *const u8
                    as *const libc::c_char,
                (*s).combinedCRC,
            );
        }
        bsFinishWrite(s);
    }
}
