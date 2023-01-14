
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    static mut __stdinp: *mut FILE;
    #[no_mangle]
    static mut __stdoutp: *mut FILE;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn ferror(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgetc(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn ungetc(_: std::os::raw::c_int, _: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn fdopen(_: std::os::raw::c_int, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn BZ2_compressBlock(_: *mut EState, _: Bool);
    #[no_mangle]
    static mut BZ2_crc32Table: [UInt32; 256];
    #[no_mangle]
    fn BZ2_decompress(_: *mut DState) -> Int32;
    #[no_mangle]
    static mut BZ2_rNums: [Int32; 512];
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
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
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: std::os::raw::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [std::os::raw::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [std::os::raw::c_char; 8],
    pub __encoding: [std::os::raw::c_char; 32],
    pub __sgetrune: Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *const std::os::raw::c_char)
                               -> __darwin_rune_t>,
    pub __sputrune: Option<unsafe extern "C" fn(_: __darwin_rune_t,
                                                _: *mut std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *mut std::os::raw::c_char)
                               -> std::os::raw::c_int>,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut std::os::raw::c_void,
    pub __variable_len: std::os::raw::c_int,
    pub __ncharclasses: std::os::raw::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
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
pub type UInt32 = std::os::raw::c_uint;
pub type Int32 = std::os::raw::c_int;
pub type UChar = std::os::raw::c_uchar;
pub type Bool = std::os::raw::c_uchar;
pub type UInt16 = std::os::raw::c_ushort;
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
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bzFile {
    pub handle: *mut FILE,
    pub buf: [Char; 5000],
    pub bufN: Int32,
    pub writing: Bool,
    pub strm: bz_stream,
    pub lastErr: Int32,
    pub initialisedOk: Bool,
}
pub type Char = std::os::raw::c_char;
#[inline]
unsafe extern "C" fn __isctype(mut _c: __darwin_ct_rune_t,
                               mut _f: std::os::raw::c_ulong) -> __darwin_ct_rune_t {
    return if _c < 0 as std::os::raw::c_int ||
                  _c >= (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int {
               0 as std::os::raw::c_int
           } else {
               (_DefaultRuneLocale.__runetype[_c as usize] as std::os::raw::c_ulong &
                    _f != 0) as std::os::raw::c_int
           };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __isctype(_c, 0x400 as std::os::raw::c_long as std::os::raw::c_ulong);
}
/*-------------------------------------------------------------*/
/*--- Library top-level functions.                          ---*/
/*---                                               bzlib.c ---*/
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
   0.9.0c   -- made zero-length BZ_FLUSH work correctly in bzCompress().
     fixed bzWrite/bzRead to ignore zero-length requests.
     fixed bzread to correctly handle read requests after EOF.
     wrong parameter order in call to bzDecompressInit in
     bzBuffToBuffDecompress.  Fixed.
*/
/*---------------------------------------------------*/
/*--- Compression stuff                           ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bz__AssertH__fail(mut errcode: std::os::raw::c_int) {
    fprintf(__stderrp,
            b"\n\nbzip2/libbzip2: internal error number %d.\nThis is a bug in bzip2/libbzip2, %s.\nPlease report it to: bzip2-devel@sourceware.org.  If this happened\nwhen you were using some program which uses libbzip2 as a\ncomponent, you should also report this bug to the author(s)\nof that program.  Please make an effort to report this bug;\ntimely and accurate bug reports eventually lead to higher\nquality software.  Thanks.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char, errcode,
            BZ2_bzlibVersion());
    if errcode == 1007 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"\n*** A special note about internal error number 1007 ***\n\nExperience suggests that a common cause of i.e. 1007\nis unreliable memory or other hardware.  The 1007 assertion\njust happens to cross-check the results of huge numbers of\nmemory reads/writes, and so acts (unintendedly) as a stress\ntest of your memory system.\n\nI suggest the following: try compressing the file again,\npossibly monitoring progress in detail with the -vv flag.\n\n* If the error cannot be reproduced, and/or happens at different\n  points in compression, you may have a flaky memory system.\n  Try a memory-test program.  I have used Memtest86\n  (www.memtest86.com).  At the time of writing it is free (GPLd).\n  Memtest86 tests memory much more thorougly than your BIOSs\n  power-on test, and may find failures that the BIOS doesn\'t.\n\n* If the error can be repeatably reproduced, this is a bug in\n  bzip2, and I would very much like to hear about it.  Please\n  let me know, and, ideally, save a copy of the file causing the\n  problem -- without which I will be unable to investigate it.\n\n\x00"
                    as *const u8 as *const std::os::raw::c_char);
    }
    exit(3 as std::os::raw::c_int);
}
/*---------------------------------------------------*/
unsafe extern "C" fn bz_config_ok() -> std::os::raw::c_int {
    if ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong !=
           4 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    if ::std::mem::size_of::<std::os::raw::c_short>() as std::os::raw::c_ulong !=
           2 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    if ::std::mem::size_of::<std::os::raw::c_char>() as std::os::raw::c_ulong !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
unsafe extern "C" fn default_bzalloc(mut opaque: *mut std::os::raw::c_void,
                                     mut items: Int32, mut size: Int32)
 -> *mut std::os::raw::c_void {
    let mut v: *mut std::os::raw::c_void = malloc((items * size) as std::os::raw::c_ulong);
    return v;
}
unsafe extern "C" fn default_bzfree(mut opaque: *mut std::os::raw::c_void,
                                    mut addr: *mut std::os::raw::c_void) {
    if !addr.is_null() { free(addr); };
}
/*---------------------------------------------------*/
unsafe extern "C" fn prepare_new_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nblock = 0 as std::os::raw::c_int;
    (*s).numZ = 0 as std::os::raw::c_int;
    (*s).state_out_pos = 0 as std::os::raw::c_int;
    (*s).blockCRC = 0xffffffff as std::os::raw::c_long as UInt32;
    i = 0 as std::os::raw::c_int;
    while i < 256 as std::os::raw::c_int {
        (*s).inUse[i as usize] = 0 as std::os::raw::c_int as Bool;
        i += 1
    }
    (*s).blockNo += 1;
}
/*---------------------------------------------------*/
unsafe extern "C" fn init_RL(mut s: *mut EState) {
    (*s).state_in_ch = 256 as std::os::raw::c_int as UInt32;
    (*s).state_in_len = 0 as std::os::raw::c_int;
}
unsafe extern "C" fn isempty_RL(mut s: *mut EState) -> Bool {
    if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*s).state_in_len > 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as Bool
    } else { return 1 as std::os::raw::c_int as Bool };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressInit(mut strm: *mut bz_stream,
                                            mut blockSize100k: std::os::raw::c_int,
                                            mut verbosity: std::os::raw::c_int,
                                            mut workFactor: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut n: Int32 = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if bz_config_ok() == 0 { return -(9 as std::os::raw::c_int) }
    if strm.is_null() || blockSize100k < 1 as std::os::raw::c_int ||
           blockSize100k > 9 as std::os::raw::c_int || workFactor < 0 as std::os::raw::c_int
           || workFactor > 250 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if workFactor == 0 as std::os::raw::c_int { workFactor = 30 as std::os::raw::c_int }
    if (*strm).bzalloc.is_none() {
        (*strm).bzalloc =
            Some(default_bzalloc as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: Int32,
                                          _: Int32) -> *mut std::os::raw::c_void)
    }
    if (*strm).bzfree.is_none() {
        (*strm).bzfree =
            Some(default_bzfree as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *mut std::os::raw::c_void) -> ())
    }
    s =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            ::std::mem::size_of::<EState>()
                                                                as
                                                                std::os::raw::c_ulong
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut EState;
    if s.is_null() { return -(3 as std::os::raw::c_int) }
    (*s).strm = strm;
    (*s).arr1 = 0 as *mut UInt32;
    (*s).arr2 = 0 as *mut UInt32;
    (*s).ftab = 0 as *mut UInt32;
    n = 100000 as std::os::raw::c_int * blockSize100k;
    (*s).arr1 =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            (n as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt32>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong)
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut UInt32;
    (*s).arr2 =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            ((n +
                                                                  (2 as
                                                                       std::os::raw::c_int
                                                                       +
                                                                       12 as
                                                                           std::os::raw::c_int
                                                                       +
                                                                       18 as
                                                                           std::os::raw::c_int
                                                                       +
                                                                       2 as
                                                                           std::os::raw::c_int))
                                                                 as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt32>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong)
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut UInt32;
    (*s).ftab =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            (65537 as
                                                                 std::os::raw::c_int
                                                                 as
                                                                 std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<UInt32>()
                                                                                                 as
                                                                                                 std::os::raw::c_ulong)
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut UInt32;
    if (*s).arr1.is_null() || (*s).arr2.is_null() || (*s).ftab.is_null() {
        if !(*s).arr1.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               (*s).arr1 as
                                                                   *mut std::os::raw::c_void);
        }
        if !(*s).arr2.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               (*s).arr2 as
                                                                   *mut std::os::raw::c_void);
        }
        if !(*s).ftab.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               (*s).ftab as
                                                                   *mut std::os::raw::c_void);
        }
        if !s.is_null() {
            (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                               s as
                                                                   *mut std::os::raw::c_void);
        }
        return -(3 as std::os::raw::c_int)
    }
    (*s).blockNo = 0 as std::os::raw::c_int;
    (*s).state = 2 as std::os::raw::c_int;
    (*s).mode = 2 as std::os::raw::c_int;
    (*s).combinedCRC = 0 as std::os::raw::c_int as UInt32;
    (*s).blockSize100k = blockSize100k;
    (*s).nblockMAX =
        100000 as std::os::raw::c_int * blockSize100k - 19 as std::os::raw::c_int;
    (*s).verbosity = verbosity;
    (*s).workFactor = workFactor;
    (*s).block = (*s).arr2 as *mut UChar;
    (*s).mtfv = (*s).arr1 as *mut UInt16;
    (*s).zbits = 0 as *mut UChar;
    (*s).ptr = (*s).arr1;
    (*strm).state = s as *mut std::os::raw::c_void;
    (*strm).total_in_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_in_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    init_RL(s);
    prepare_new_block(s);
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
unsafe extern "C" fn add_pair_to_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    let mut ch: UChar = (*s).state_in_ch as UChar;
    i = 0 as std::os::raw::c_int;
    while i < (*s).state_in_len {
        (*s).blockCRC =
            (*s).blockCRC << 8 as std::os::raw::c_int ^
                BZ2_crc32Table[((*s).blockCRC >> 24 as std::os::raw::c_int ^
                                    ch as std::os::raw::c_uint) as usize];
        i += 1
    }
    (*s).inUse[(*s).state_in_ch as usize] = 1 as std::os::raw::c_int as Bool;
    match (*s).state_in_len {
        1 => {
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1
        }
        2 => {
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1
        }
        3 => {
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1
        }
        _ => {
            (*s).inUse[((*s).state_in_len - 4 as std::os::raw::c_int) as usize] =
                1 as std::os::raw::c_int as Bool;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) = ch;
            (*s).nblock += 1;
            *(*s).block.offset((*s).nblock as isize) =
                ((*s).state_in_len - 4 as std::os::raw::c_int) as UChar;
            (*s).nblock += 1
        }
    };
}
/*---------------------------------------------------*/
unsafe extern "C" fn flush_RL(mut s: *mut EState) {
    if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint {
        add_pair_to_block(s);
    }
    init_RL(s);
}
/*---------------------------------------------------*/
/*-- fast track the common case --*/
/*-- general, uncommon cases --*/
/*---------------------------------------------------*/
unsafe extern "C" fn copy_input_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_in: Bool = 0 as std::os::raw::c_int as Bool;
    if (*s).mode == 2 as std::os::raw::c_int {
        /*-- fast track the common case --*/
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*-- block full? --*/
            if (*s).nblock >= (*s).nblockMAX { break ; }
            /*-- no input? --*/
            if (*(*s).strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                break ;
            }
            progress_in = 1 as std::os::raw::c_int as Bool;
            let mut zchh: UInt32 =
                *((*(*s).strm).next_in as *mut UChar) as UInt32;
            if zchh != (*s).state_in_ch &&
                   (*s).state_in_len == 1 as std::os::raw::c_int {
                let mut ch: UChar = (*s).state_in_ch as UChar;
                (*s).blockCRC =
                    (*s).blockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).blockCRC >> 24 as std::os::raw::c_int ^
                                            ch as std::os::raw::c_uint) as usize];
                (*s).inUse[(*s).state_in_ch as usize] =
                    1 as std::os::raw::c_int as Bool;
                *(*s).block.offset((*s).nblock as isize) = ch;
                (*s).nblock += 1;
                (*s).state_in_ch = zchh
            } else if zchh != (*s).state_in_ch ||
                          (*s).state_in_len == 255 as std::os::raw::c_int {
                if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint {
                    add_pair_to_block(s);
                }
                (*s).state_in_ch = zchh;
                (*s).state_in_len = 1 as std::os::raw::c_int
            } else { (*s).state_in_len += 1 }
            (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
            (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
            (*(*s).strm).total_in_lo32 =
                (*(*s).strm).total_in_lo32.wrapping_add(1);
            if (*(*s).strm).total_in_lo32 == 0 as std::os::raw::c_int as std::os::raw::c_uint
               {
                (*(*s).strm).total_in_hi32 =
                    (*(*s).strm).total_in_hi32.wrapping_add(1)
            }
        }
    } else {
        /*-- general, uncommon case --*/
        while 1 as std::os::raw::c_int as Bool != 0 {
            /*-- block full? --*/
            if (*s).nblock >= (*s).nblockMAX { break ; }
            /*-- no input? --*/
            if (*(*s).strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                break ;
            }
            /*-- flush/finish end? --*/
            if (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                break ;
            }
            progress_in = 1 as std::os::raw::c_int as Bool;
            let mut zchh_0: UInt32 =
                *((*(*s).strm).next_in as *mut UChar) as UInt32;
            if zchh_0 != (*s).state_in_ch &&
                   (*s).state_in_len == 1 as std::os::raw::c_int {
                let mut ch_0: UChar = (*s).state_in_ch as UChar;
                (*s).blockCRC =
                    (*s).blockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).blockCRC >> 24 as std::os::raw::c_int ^
                                            ch_0 as std::os::raw::c_uint) as usize];
                (*s).inUse[(*s).state_in_ch as usize] =
                    1 as std::os::raw::c_int as Bool;
                *(*s).block.offset((*s).nblock as isize) = ch_0;
                (*s).nblock += 1;
                (*s).state_in_ch = zchh_0
            } else if zchh_0 != (*s).state_in_ch ||
                          (*s).state_in_len == 255 as std::os::raw::c_int {
                if (*s).state_in_ch < 256 as std::os::raw::c_int as std::os::raw::c_uint {
                    add_pair_to_block(s);
                }
                (*s).state_in_ch = zchh_0;
                (*s).state_in_len = 1 as std::os::raw::c_int
            } else { (*s).state_in_len += 1 }
            (*(*s).strm).next_in = (*(*s).strm).next_in.offset(1);
            (*(*s).strm).avail_in = (*(*s).strm).avail_in.wrapping_sub(1);
            (*(*s).strm).total_in_lo32 =
                (*(*s).strm).total_in_lo32.wrapping_add(1);
            if (*(*s).strm).total_in_lo32 == 0 as std::os::raw::c_int as std::os::raw::c_uint
               {
                (*(*s).strm).total_in_hi32 =
                    (*(*s).strm).total_in_hi32.wrapping_add(1)
            }
            (*s).avail_in_expect = (*s).avail_in_expect.wrapping_sub(1)
        }
    }
    return progress_in;
}
/*---------------------------------------------------*/
unsafe extern "C" fn copy_output_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_out: Bool = 0 as std::os::raw::c_int as Bool;
    while 1 as std::os::raw::c_int as Bool != 0 {
        /*-- no output space? --*/
        if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            break ;
        }
        /*-- block done? --*/
        if (*s).state_out_pos >= (*s).numZ { break ; }
        progress_out = 1 as std::os::raw::c_int as Bool;
        *(*(*s).strm).next_out =
            *(*s).zbits.offset((*s).state_out_pos as isize) as std::os::raw::c_char;
        (*s).state_out_pos += 1;
        (*(*s).strm).avail_out = (*(*s).strm).avail_out.wrapping_sub(1);
        (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
        (*(*s).strm).total_out_lo32 =
            (*(*s).strm).total_out_lo32.wrapping_add(1);
        if (*(*s).strm).total_out_lo32 == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*(*s).strm).total_out_hi32 =
                (*(*s).strm).total_out_hi32.wrapping_add(1)
        }
    }
    return progress_out;
}
/*---------------------------------------------------*/
unsafe extern "C" fn handle_compress(mut strm: *mut bz_stream) -> Bool {
    let mut progress_in: Bool = 0 as std::os::raw::c_int as Bool;
    let mut progress_out: Bool = 0 as std::os::raw::c_int as Bool;
    let mut s: *mut EState = (*strm).state as *mut EState;
    while 1 as std::os::raw::c_int as Bool != 0 {
        if (*s).state == 1 as std::os::raw::c_int {
            progress_out =
                (progress_out as std::os::raw::c_int |
                     copy_output_until_stop(s) as std::os::raw::c_int) as Bool;
            if (*s).state_out_pos < (*s).numZ { break ; }
            if (*s).mode == 4 as std::os::raw::c_int &&
                   (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   isempty_RL(s) as std::os::raw::c_int != 0 {
                break ;
            }
            prepare_new_block(s);
            (*s).state = 2 as std::os::raw::c_int;
            if (*s).mode == 3 as std::os::raw::c_int &&
                   (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   isempty_RL(s) as std::os::raw::c_int != 0 {
                break ;
            }
        }
        if !((*s).state == 2 as std::os::raw::c_int) { continue ; }
        progress_in =
            (progress_in as std::os::raw::c_int |
                 copy_input_until_stop(s) as std::os::raw::c_int) as Bool;
        if (*s).mode != 2 as std::os::raw::c_int &&
               (*s).avail_in_expect == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            flush_RL(s);
            BZ2_compressBlock(s,
                              ((*s).mode == 4 as std::os::raw::c_int) as std::os::raw::c_int
                                  as Bool);
            (*s).state = 1 as std::os::raw::c_int
        } else if (*s).nblock >= (*s).nblockMAX {
            BZ2_compressBlock(s, 0 as std::os::raw::c_int as Bool);
            (*s).state = 1 as std::os::raw::c_int
        } else if (*(*s).strm).avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            break ;
        }
    }
    return (progress_in as std::os::raw::c_int != 0 ||
                progress_out as std::os::raw::c_int != 0) as std::os::raw::c_int as Bool;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompress(mut strm: *mut bz_stream,
                                        mut action: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut progress: Bool = 0;
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut EState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    loop  {
        match (*s).mode {
            1 => { return -(1 as std::os::raw::c_int) }
            2 => {
                if action == 0 as std::os::raw::c_int {
                    progress = handle_compress(strm);
                    return if progress as std::os::raw::c_int != 0 {
                               1 as std::os::raw::c_int
                           } else { -(2 as std::os::raw::c_int) }
                } else if action == 1 as std::os::raw::c_int {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 3 as std::os::raw::c_int
                } else if action == 2 as std::os::raw::c_int {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 4 as std::os::raw::c_int
                } else { return -(2 as std::os::raw::c_int) }
            }
            3 => {
                if action != 1 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as std::os::raw::c_int)
                }
                progress = handle_compress(strm);
                if (*s).avail_in_expect > 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ {
                    return 2 as std::os::raw::c_int
                }
                (*s).mode = 2 as std::os::raw::c_int;
                return 1 as std::os::raw::c_int
            }
            4 => {
                if action != 2 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as std::os::raw::c_int)
                }
                progress = handle_compress(strm);
                if progress == 0 { return -(1 as std::os::raw::c_int) }
                if (*s).avail_in_expect > 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ {
                    return 3 as std::os::raw::c_int
                }
                (*s).mode = 1 as std::os::raw::c_int;
                return 4 as std::os::raw::c_int
            }
            _ => { return 0 as std::os::raw::c_int }
        }
    };
    /*--not reached--*/
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressEnd(mut strm: *mut bz_stream)
 -> std::os::raw::c_int {
    let mut s: *mut EState = 0 as *mut EState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut EState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    if !(*s).arr1.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).arr1 as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).arr2.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).arr2 as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).ftab.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).ftab as
                                                               *mut std::os::raw::c_void);
    }
    (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                       (*strm).state);
    (*strm).state = 0 as *mut std::os::raw::c_void;
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
/*--- Decompression stuff                         ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressInit(mut strm: *mut bz_stream,
                                              mut verbosity: std::os::raw::c_int,
                                              mut small: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut s: *mut DState = 0 as *mut DState;
    if bz_config_ok() == 0 { return -(9 as std::os::raw::c_int) }
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    if small != 0 as std::os::raw::c_int && small != 1 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if (*strm).bzalloc.is_none() {
        (*strm).bzalloc =
            Some(default_bzalloc as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: Int32,
                                          _: Int32) -> *mut std::os::raw::c_void)
    }
    if (*strm).bzfree.is_none() {
        (*strm).bzfree =
            Some(default_bzfree as
                     unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                          _: *mut std::os::raw::c_void) -> ())
    }
    s =
        (*strm).bzalloc.expect("non-null function pointer")((*strm).opaque,
                                                            ::std::mem::size_of::<DState>()
                                                                as
                                                                std::os::raw::c_ulong
                                                                as
                                                                std::os::raw::c_int,
                                                            1 as std::os::raw::c_int)
            as *mut DState;
    if s.is_null() { return -(3 as std::os::raw::c_int) }
    (*s).strm = strm;
    (*strm).state = s as *mut std::os::raw::c_void;
    (*s).state = 10 as std::os::raw::c_int;
    (*s).bsLive = 0 as std::os::raw::c_int;
    (*s).bsBuff = 0 as std::os::raw::c_int as UInt32;
    (*s).calculatedCombinedCRC = 0 as std::os::raw::c_int as UInt32;
    (*strm).total_in_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_in_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*strm).total_out_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*s).smallDecompress = small as Bool;
    (*s).ll4 = 0 as *mut UChar;
    (*s).ll16 = 0 as *mut UInt16;
    (*s).tt = 0 as *mut UInt32;
    (*s).currBlockNo = 0 as std::os::raw::c_int;
    (*s).verbosity = verbosity;
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
/* Return  True iff data corruption is discovered.
   Returns False if there is no problem.
*/
unsafe extern "C" fn unRLE_obuf_to_output_FAST(mut s: *mut DState) -> Bool {
    let mut current_block: u64;
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as std::os::raw::c_int as Bool != 0 {
            /* try to finish existing run */
            while 1 as std::os::raw::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint
                   {
                    return 0 as std::os::raw::c_int as Bool
                }
                if (*s).state_out_len == 0 as std::os::raw::c_int { break ; }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC =
                    (*s).calculatedBlockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).calculatedBlockCRC >>
                                            24 as std::os::raw::c_int ^
                                            (*s).state_out_ch as std::os::raw::c_uint)
                                           as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
                (*(*s).strm).avail_out =
                    (*(*s).strm).avail_out.wrapping_sub(1);
                (*(*s).strm).total_out_lo32 =
                    (*(*s).strm).total_out_lo32.wrapping_add(1);
                if (*(*s).strm).total_out_lo32 ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    (*(*s).strm).total_out_hi32 =
                        (*(*s).strm).total_out_hi32.wrapping_add(1)
                }
            }
            /* can a new run be started? */
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int as Bool
            }
            /* Only caused by corrupt data stream? */
            if (*s).nblock_used > (*s).save_nblock + 1 as std::os::raw::c_int {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).state_out_len = 1 as std::os::raw::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos >=
                   (100000 as std::os::raw::c_int as
                        UInt32).wrapping_mul((*s).blockSize100k as UInt32) {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
            k1 = ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
            (*s).tPos >>= 8 as std::os::raw::c_int;
            if (*s).rNToGo == 0 as std::os::raw::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                (*s).rTPos += 1;
                if (*s).rTPos == 512 as std::os::raw::c_int {
                    (*s).rTPos = 0 as std::os::raw::c_int
                }
            }
            (*s).rNToGo -= 1;
            k1 =
                (k1 as std::os::raw::c_int ^
                     if (*s).rNToGo == 1 as std::os::raw::c_int {
                         1 as std::os::raw::c_int
                     } else { 0 as std::os::raw::c_int }) as UChar;
            (*s).nblock_used += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                continue ;
            }
            if k1 as std::os::raw::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32
            } else {
                (*s).state_out_len = 2 as std::os::raw::c_int;
                if (*s).tPos >=
                       (100000 as std::os::raw::c_int as
                            UInt32).wrapping_mul((*s).blockSize100k as UInt32)
                   {
                    return 1 as std::os::raw::c_int as Bool
                }
                (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                k1 =
                    ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                        UChar;
                (*s).tPos >>= 8 as std::os::raw::c_int;
                if (*s).rNToGo == 0 as std::os::raw::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 as std::os::raw::c_int {
                        (*s).rTPos = 0 as std::os::raw::c_int
                    }
                }
                (*s).rNToGo -= 1;
                k1 =
                    (k1 as std::os::raw::c_int ^
                         if (*s).rNToGo == 1 as std::os::raw::c_int {
                             1 as std::os::raw::c_int
                         } else { 0 as std::os::raw::c_int }) as UChar;
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                    continue ;
                }
                if k1 as std::os::raw::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32
                } else {
                    (*s).state_out_len = 3 as std::os::raw::c_int;
                    if (*s).tPos >=
                           (100000 as std::os::raw::c_int as
                                UInt32).wrapping_mul((*s).blockSize100k as
                                                         UInt32) {
                        return 1 as std::os::raw::c_int as Bool
                    }
                    (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                    k1 =
                        ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                            UChar;
                    (*s).tPos >>= 8 as std::os::raw::c_int;
                    if (*s).rNToGo == 0 as std::os::raw::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        (*s).rTPos += 1;
                        if (*s).rTPos == 512 as std::os::raw::c_int {
                            (*s).rTPos = 0 as std::os::raw::c_int
                        }
                    }
                    (*s).rNToGo -= 1;
                    k1 =
                        (k1 as std::os::raw::c_int ^
                             if (*s).rNToGo == 1 as std::os::raw::c_int {
                                 1 as std::os::raw::c_int
                             } else { 0 as std::os::raw::c_int }) as UChar;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int
                       {
                        continue ;
                    }
                    if k1 as std::os::raw::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32
                    } else {
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                        k1 =
                            ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                as UChar;
                        (*s).tPos >>= 8 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        k1 =
                            (k1 as std::os::raw::c_int ^
                                 if (*s).rNToGo == 1 as std::os::raw::c_int {
                                     1 as std::os::raw::c_int
                                 } else { 0 as std::os::raw::c_int }) as UChar;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).tPos = *(*s).tt.offset((*s).tPos as isize);
                        (*s).k0 =
                            ((*s).tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                as UChar as Int32;
                        (*s).tPos >>= 8 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        (*s).k0 ^=
                            if (*s).rNToGo == 1 as std::os::raw::c_int {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int };
                        (*s).nblock_used += 1
                    }
                }
            }
        }
    } else {
        /* restore */
        let mut c_calculatedBlockCRC: UInt32 = (*s).calculatedBlockCRC;
        let mut c_state_out_ch: UChar = (*s).state_out_ch;
        let mut c_state_out_len: Int32 = (*s).state_out_len;
        let mut c_nblock_used: Int32 = (*s).nblock_used;
        let mut c_k0: Int32 = (*s).k0;
        let mut c_tt: *mut UInt32 = (*s).tt;
        let mut c_tPos: UInt32 = (*s).tPos;
        let mut cs_next_out: *mut std::os::raw::c_char = (*(*s).strm).next_out;
        let mut cs_avail_out: std::os::raw::c_uint = (*(*s).strm).avail_out;
        let mut ro_blockSize100k: Int32 = (*s).blockSize100k;
        /* end restore */
        let mut avail_out_INIT: UInt32 = cs_avail_out;
        let mut s_save_nblockPP: Int32 = (*s).save_nblock + 1 as std::os::raw::c_int;
        let mut total_out_lo32_old: std::os::raw::c_uint = 0;
        's_569:
            while 1 as std::os::raw::c_int as Bool != 0 {
                /* try to finish existing run */
                if c_state_out_len > 0 as std::os::raw::c_int {
                    while 1 as std::os::raw::c_int as Bool != 0 {
                        if cs_avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            break 's_569 ;
                        }
                        if c_state_out_len == 1 as std::os::raw::c_int { break ; }
                        *(cs_next_out as *mut UChar) = c_state_out_ch;
                        c_calculatedBlockCRC =
                            c_calculatedBlockCRC << 8 as std::os::raw::c_int ^
                                BZ2_crc32Table[(c_calculatedBlockCRC >>
                                                    24 as std::os::raw::c_int ^
                                                    c_state_out_ch as
                                                        std::os::raw::c_uint) as
                                                   usize];
                        c_state_out_len -= 1;
                        cs_next_out = cs_next_out.offset(1);
                        cs_avail_out = cs_avail_out.wrapping_sub(1)
                    }
                    current_block = 16910810822589621899;
                } else { current_block = 3024573345131975588; }
                loop  {
                    match current_block {
                        16910810822589621899 => {
                            if cs_avail_out ==
                                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                                c_state_out_len = 1 as std::os::raw::c_int;
                                break 's_569 ;
                            } else {
                                *(cs_next_out as *mut UChar) = c_state_out_ch;
                                c_calculatedBlockCRC =
                                    c_calculatedBlockCRC << 8 as std::os::raw::c_int ^
                                        BZ2_crc32Table[(c_calculatedBlockCRC
                                                            >>
                                                            24 as std::os::raw::c_int
                                                            ^
                                                            c_state_out_ch as
                                                                std::os::raw::c_uint)
                                                           as usize];
                                cs_next_out = cs_next_out.offset(1);
                                cs_avail_out = cs_avail_out.wrapping_sub(1);
                                current_block = 3024573345131975588;
                            }
                        }
                        _ => {
                            /* Only caused by corrupt data stream? */
                            if c_nblock_used > s_save_nblockPP {
                                return 1 as std::os::raw::c_int as Bool
                            }
                            /* can a new run be started? */
                            if c_nblock_used == s_save_nblockPP {
                                c_state_out_len = 0 as std::os::raw::c_int;
                                break 's_569 ;
                            } else {
                                c_state_out_ch = c_k0 as UChar;
                                if c_tPos >=
                                       (100000 as std::os::raw::c_int as
                                            UInt32).wrapping_mul(ro_blockSize100k
                                                                     as
                                                                     UInt32) {
                                    return 1 as std::os::raw::c_int as Bool
                                }
                                c_tPos = *c_tt.offset(c_tPos as isize);
                                k1 =
                                    (c_tPos &
                                         0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                        as UChar;
                                c_tPos >>= 8 as std::os::raw::c_int;
                                c_nblock_used += 1;
                                if k1 as std::os::raw::c_int != c_k0 {
                                    c_k0 = k1 as Int32;
                                    current_block = 16910810822589621899;
                                } else {
                                    if c_nblock_used == s_save_nblockPP {
                                        current_block = 16910810822589621899;
                                        continue ;
                                    }
                                    c_state_out_len = 2 as std::os::raw::c_int;
                                    if c_tPos >=
                                           (100000 as std::os::raw::c_int as
                                                UInt32).wrapping_mul(ro_blockSize100k
                                                                         as
                                                                         UInt32)
                                       {
                                        return 1 as std::os::raw::c_int as Bool
                                    }
                                    c_tPos = *c_tt.offset(c_tPos as isize);
                                    k1 =
                                        (c_tPos &
                                             0xff as std::os::raw::c_int as
                                                 std::os::raw::c_uint) as UChar;
                                    c_tPos >>= 8 as std::os::raw::c_int;
                                    c_nblock_used += 1;
                                    if c_nblock_used == s_save_nblockPP {
                                        continue 's_569 ;
                                    }
                                    if k1 as std::os::raw::c_int != c_k0 {
                                        current_block = 18139099716546303047;
                                        break ;
                                    } else {
                                        current_block = 919396821984190499;
                                        break ;
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    18139099716546303047 => { c_k0 = k1 as Int32 }
                    _ => {
                        c_state_out_len = 3 as std::os::raw::c_int;
                        if c_tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul(ro_blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        c_tPos = *c_tt.offset(c_tPos as isize);
                        k1 =
                            (c_tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
                                UChar;
                        c_tPos >>= 8 as std::os::raw::c_int;
                        c_nblock_used += 1;
                        if c_nblock_used == s_save_nblockPP { continue ; }
                        if k1 as std::os::raw::c_int != c_k0 {
                            c_k0 = k1 as Int32
                        } else {
                            if c_tPos >=
                                   (100000 as std::os::raw::c_int as
                                        UInt32).wrapping_mul(ro_blockSize100k
                                                                 as UInt32) {
                                return 1 as std::os::raw::c_int as Bool
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            k1 =
                                (c_tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                    as UChar;
                            c_tPos >>= 8 as std::os::raw::c_int;
                            c_nblock_used += 1;
                            c_state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                            if c_tPos >=
                                   (100000 as std::os::raw::c_int as
                                        UInt32).wrapping_mul(ro_blockSize100k
                                                                 as UInt32) {
                                return 1 as std::os::raw::c_int as Bool
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            c_k0 =
                                (c_tPos & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
                                    as UChar as Int32;
                            c_tPos >>= 8 as std::os::raw::c_int;
                            c_nblock_used += 1
                        }
                    }
                }
            }
        total_out_lo32_old = (*(*s).strm).total_out_lo32;
        (*(*s).strm).total_out_lo32 =
            (*(*s).strm).total_out_lo32.wrapping_add(avail_out_INIT.wrapping_sub(cs_avail_out));
        if (*(*s).strm).total_out_lo32 < total_out_lo32_old {
            (*(*s).strm).total_out_hi32 =
                (*(*s).strm).total_out_hi32.wrapping_add(1)
        }
        /* save */
        (*s).calculatedBlockCRC = c_calculatedBlockCRC;
        (*s).state_out_ch = c_state_out_ch;
        (*s).state_out_len = c_state_out_len;
        (*s).nblock_used = c_nblock_used;
        (*s).k0 = c_k0;
        (*s).tt = c_tt;
        (*s).tPos = c_tPos;
        (*(*s).strm).next_out = cs_next_out;
        (*(*s).strm).avail_out = cs_avail_out
        /* end save */
    }
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------------*/
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn BZ2_indexIntoF(mut indx: Int32,
                                        mut cftab: *mut Int32) -> Int32 {
    let mut nb: Int32 = 0;
    let mut na: Int32 = 0;
    let mut mid: Int32 = 0;
    nb = 0 as std::os::raw::c_int;
    na = 256 as std::os::raw::c_int;
    loop  {
        mid = nb + na >> 1 as std::os::raw::c_int;
        if indx >= *cftab.offset(mid as isize) { nb = mid } else { na = mid }
        if !(na - nb != 1 as std::os::raw::c_int) { break ; }
    }
    return nb;
}
/*---------------------------------------------------*/
/* Return  True iff data corruption is discovered.
   Returns False if there is no problem.
*/
unsafe extern "C" fn unRLE_obuf_to_output_SMALL(mut s: *mut DState) -> Bool {
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as std::os::raw::c_int as Bool != 0 {
            /* try to finish existing run */
            while 1 as std::os::raw::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint
                   {
                    return 0 as std::os::raw::c_int as Bool
                }
                if (*s).state_out_len == 0 as std::os::raw::c_int { break ; }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC =
                    (*s).calculatedBlockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).calculatedBlockCRC >>
                                            24 as std::os::raw::c_int ^
                                            (*s).state_out_ch as std::os::raw::c_uint)
                                           as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
                (*(*s).strm).avail_out =
                    (*(*s).strm).avail_out.wrapping_sub(1);
                (*(*s).strm).total_out_lo32 =
                    (*(*s).strm).total_out_lo32.wrapping_add(1);
                if (*(*s).strm).total_out_lo32 ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    (*(*s).strm).total_out_hi32 =
                        (*(*s).strm).total_out_hi32.wrapping_add(1)
                }
            }
            /* can a new run be started? */
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int as Bool
            }
            /* Only caused by corrupt data stream? */
            if (*s).nblock_used > (*s).save_nblock + 1 as std::os::raw::c_int {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).state_out_len = 1 as std::os::raw::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos >=
                   (100000 as std::os::raw::c_int as
                        UInt32).wrapping_mul((*s).blockSize100k as UInt32) {
                return 1 as std::os::raw::c_int as Bool
            }
            k1 =
                BZ2_indexIntoF((*s).tPos as Int32, (*s).cftab.as_mut_ptr()) as
                    UChar;
            (*s).tPos =
                *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                    (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                          isize) as UInt32 >>
                         ((*s).tPos << 2 as std::os::raw::c_int &
                              0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                         0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                        16 as std::os::raw::c_int;
            if (*s).rNToGo == 0 as std::os::raw::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                (*s).rTPos += 1;
                if (*s).rTPos == 512 as std::os::raw::c_int {
                    (*s).rTPos = 0 as std::os::raw::c_int
                }
            }
            (*s).rNToGo -= 1;
            k1 =
                (k1 as std::os::raw::c_int ^
                     if (*s).rNToGo == 1 as std::os::raw::c_int {
                         1 as std::os::raw::c_int
                     } else { 0 as std::os::raw::c_int }) as UChar;
            (*s).nblock_used += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                continue ;
            }
            if k1 as std::os::raw::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32
            } else {
                (*s).state_out_len = 2 as std::os::raw::c_int;
                if (*s).tPos >=
                       (100000 as std::os::raw::c_int as
                            UInt32).wrapping_mul((*s).blockSize100k as UInt32)
                   {
                    return 1 as std::os::raw::c_int as Bool
                }
                k1 =
                    BZ2_indexIntoF((*s).tPos as Int32,
                                   (*s).cftab.as_mut_ptr()) as UChar;
                (*s).tPos =
                    *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                        (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                              isize) as UInt32 >>
                             ((*s).tPos << 2 as std::os::raw::c_int &
                                  0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                             0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                            16 as std::os::raw::c_int;
                if (*s).rNToGo == 0 as std::os::raw::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    (*s).rTPos += 1;
                    if (*s).rTPos == 512 as std::os::raw::c_int {
                        (*s).rTPos = 0 as std::os::raw::c_int
                    }
                }
                (*s).rNToGo -= 1;
                k1 =
                    (k1 as std::os::raw::c_int ^
                         if (*s).rNToGo == 1 as std::os::raw::c_int {
                             1 as std::os::raw::c_int
                         } else { 0 as std::os::raw::c_int }) as UChar;
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                    continue ;
                }
                if k1 as std::os::raw::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32
                } else {
                    (*s).state_out_len = 3 as std::os::raw::c_int;
                    if (*s).tPos >=
                           (100000 as std::os::raw::c_int as
                                UInt32).wrapping_mul((*s).blockSize100k as
                                                         UInt32) {
                        return 1 as std::os::raw::c_int as Bool
                    }
                    k1 =
                        BZ2_indexIntoF((*s).tPos as Int32,
                                       (*s).cftab.as_mut_ptr()) as UChar;
                    (*s).tPos =
                        *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                            (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int)
                                                  as isize) as UInt32 >>
                                 ((*s).tPos << 2 as std::os::raw::c_int &
                                      0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                                 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                16 as std::os::raw::c_int;
                    if (*s).rNToGo == 0 as std::os::raw::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        (*s).rTPos += 1;
                        if (*s).rTPos == 512 as std::os::raw::c_int {
                            (*s).rTPos = 0 as std::os::raw::c_int
                        }
                    }
                    (*s).rNToGo -= 1;
                    k1 =
                        (k1 as std::os::raw::c_int ^
                             if (*s).rNToGo == 1 as std::os::raw::c_int {
                                 1 as std::os::raw::c_int
                             } else { 0 as std::os::raw::c_int }) as UChar;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int
                       {
                        continue ;
                    }
                    if k1 as std::os::raw::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32
                    } else {
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        k1 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr()) as UChar;
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        k1 =
                            (k1 as std::os::raw::c_int ^
                                 if (*s).rNToGo == 1 as std::os::raw::c_int {
                                     1 as std::os::raw::c_int
                                 } else { 0 as std::os::raw::c_int }) as UChar;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).k0 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr());
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        if (*s).rNToGo == 0 as std::os::raw::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            (*s).rTPos += 1;
                            if (*s).rTPos == 512 as std::os::raw::c_int {
                                (*s).rTPos = 0 as std::os::raw::c_int
                            }
                        }
                        (*s).rNToGo -= 1;
                        (*s).k0 ^=
                            if (*s).rNToGo == 1 as std::os::raw::c_int {
                                1 as std::os::raw::c_int
                            } else { 0 as std::os::raw::c_int };
                        (*s).nblock_used += 1
                    }
                }
            }
        }
    } else {
        while 1 as std::os::raw::c_int as Bool != 0 {
            /* try to finish existing run */
            while 1 as std::os::raw::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint
                   {
                    return 0 as std::os::raw::c_int as Bool
                }
                if (*s).state_out_len == 0 as std::os::raw::c_int { break ; }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s).calculatedBlockCRC =
                    (*s).calculatedBlockCRC << 8 as std::os::raw::c_int ^
                        BZ2_crc32Table[((*s).calculatedBlockCRC >>
                                            24 as std::os::raw::c_int ^
                                            (*s).state_out_ch as std::os::raw::c_uint)
                                           as usize];
                (*s).state_out_len -= 1;
                (*(*s).strm).next_out = (*(*s).strm).next_out.offset(1);
                (*(*s).strm).avail_out =
                    (*(*s).strm).avail_out.wrapping_sub(1);
                (*(*s).strm).total_out_lo32 =
                    (*(*s).strm).total_out_lo32.wrapping_add(1);
                if (*(*s).strm).total_out_lo32 ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    (*(*s).strm).total_out_hi32 =
                        (*(*s).strm).total_out_hi32.wrapping_add(1)
                }
            }
            /* can a new run be started? */
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int as Bool
            }
            /* Only caused by corrupt data stream? */
            if (*s).nblock_used > (*s).save_nblock + 1 as std::os::raw::c_int {
                return 1 as std::os::raw::c_int as Bool
            }
            (*s).state_out_len = 1 as std::os::raw::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos >=
                   (100000 as std::os::raw::c_int as
                        UInt32).wrapping_mul((*s).blockSize100k as UInt32) {
                return 1 as std::os::raw::c_int as Bool
            }
            k1 =
                BZ2_indexIntoF((*s).tPos as Int32, (*s).cftab.as_mut_ptr()) as
                    UChar;
            (*s).tPos =
                *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                    (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                          isize) as UInt32 >>
                         ((*s).tPos << 2 as std::os::raw::c_int &
                              0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                         0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                        16 as std::os::raw::c_int;
            (*s).nblock_used += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                continue ;
            }
            if k1 as std::os::raw::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32
            } else {
                (*s).state_out_len = 2 as std::os::raw::c_int;
                if (*s).tPos >=
                       (100000 as std::os::raw::c_int as
                            UInt32).wrapping_mul((*s).blockSize100k as UInt32)
                   {
                    return 1 as std::os::raw::c_int as Bool
                }
                k1 =
                    BZ2_indexIntoF((*s).tPos as Int32,
                                   (*s).cftab.as_mut_ptr()) as UChar;
                (*s).tPos =
                    *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                        (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int) as
                                              isize) as UInt32 >>
                             ((*s).tPos << 2 as std::os::raw::c_int &
                                  0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                             0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                            16 as std::os::raw::c_int;
                (*s).nblock_used += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int {
                    continue ;
                }
                if k1 as std::os::raw::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32
                } else {
                    (*s).state_out_len = 3 as std::os::raw::c_int;
                    if (*s).tPos >=
                           (100000 as std::os::raw::c_int as
                                UInt32).wrapping_mul((*s).blockSize100k as
                                                         UInt32) {
                        return 1 as std::os::raw::c_int as Bool
                    }
                    k1 =
                        BZ2_indexIntoF((*s).tPos as Int32,
                                       (*s).cftab.as_mut_ptr()) as UChar;
                    (*s).tPos =
                        *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                            (*(*s).ll4.offset(((*s).tPos >> 1 as std::os::raw::c_int)
                                                  as isize) as UInt32 >>
                                 ((*s).tPos << 2 as std::os::raw::c_int &
                                      0x4 as std::os::raw::c_int as std::os::raw::c_uint) &
                                 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                16 as std::os::raw::c_int;
                    (*s).nblock_used += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int
                       {
                        continue ;
                    }
                    if k1 as std::os::raw::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32
                    } else {
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        k1 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr()) as UChar;
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        (*s).nblock_used += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as std::os::raw::c_int;
                        if (*s).tPos >=
                               (100000 as std::os::raw::c_int as
                                    UInt32).wrapping_mul((*s).blockSize100k as
                                                             UInt32) {
                            return 1 as std::os::raw::c_int as Bool
                        }
                        (*s).k0 =
                            BZ2_indexIntoF((*s).tPos as Int32,
                                           (*s).cftab.as_mut_ptr());
                        (*s).tPos =
                            *(*s).ll16.offset((*s).tPos as isize) as UInt32 |
                                (*(*s).ll4.offset(((*s).tPos >>
                                                       1 as std::os::raw::c_int) as
                                                      isize) as UInt32 >>
                                     ((*s).tPos << 2 as std::os::raw::c_int &
                                          0x4 as std::os::raw::c_int as std::os::raw::c_uint)
                                     & 0xf as std::os::raw::c_int as std::os::raw::c_uint) <<
                                    16 as std::os::raw::c_int;
                        (*s).nblock_used += 1
                    }
                }
            }
        }
    }
    panic!("Reached end of non-void function without returning");
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompress(mut strm: *mut bz_stream)
 -> std::os::raw::c_int {
    let mut corrupt: Bool = 0;
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut DState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    while 1 as std::os::raw::c_int as Bool != 0 {
        if (*s).state == 1 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        if (*s).state == 2 as std::os::raw::c_int {
            if (*s).smallDecompress != 0 {
                corrupt = unRLE_obuf_to_output_SMALL(s)
            } else { corrupt = unRLE_obuf_to_output_FAST(s) }
            if corrupt != 0 { return -(4 as std::os::raw::c_int) }
            if (*s).nblock_used == (*s).save_nblock + 1 as std::os::raw::c_int &&
                   (*s).state_out_len == 0 as std::os::raw::c_int {
                (*s).calculatedBlockCRC = !(*s).calculatedBlockCRC;
                if (*s).verbosity >= 3 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b" {0x%08x, 0x%08x}\x00" as *const u8 as
                                *const std::os::raw::c_char, (*s).storedBlockCRC,
                            (*s).calculatedBlockCRC);
                }
                if (*s).verbosity >= 2 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"]\x00" as *const u8 as *const std::os::raw::c_char);
                }
                if (*s).calculatedBlockCRC != (*s).storedBlockCRC {
                    return -(4 as std::os::raw::c_int)
                }
                (*s).calculatedCombinedCRC =
                    (*s).calculatedCombinedCRC << 1 as std::os::raw::c_int |
                        (*s).calculatedCombinedCRC >> 31 as std::os::raw::c_int;
                (*s).calculatedCombinedCRC ^= (*s).calculatedBlockCRC;
                (*s).state = 14 as std::os::raw::c_int
            } else { return 0 as std::os::raw::c_int }
        }
        if (*s).state >= 10 as std::os::raw::c_int {
            let mut r: Int32 = BZ2_decompress(s);
            if r == 4 as std::os::raw::c_int {
                if (*s).verbosity >= 3 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"\n    combined CRCs: stored = 0x%08x, computed = 0x%08x\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            (*s).storedCombinedCRC,
                            (*s).calculatedCombinedCRC);
                }
                if (*s).calculatedCombinedCRC != (*s).storedCombinedCRC {
                    return -(4 as std::os::raw::c_int)
                }
                return r
            }
            if (*s).state != 2 as std::os::raw::c_int { return r }
        }
    }
    if 0 as std::os::raw::c_int == 0 { BZ2_bz__AssertH__fail(6001 as std::os::raw::c_int); }
    return 0 as std::os::raw::c_int;
    /*NOTREACHED*/
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressEnd(mut strm: *mut bz_stream)
 -> std::os::raw::c_int {
    let mut s: *mut DState = 0 as *mut DState;
    if strm.is_null() { return -(2 as std::os::raw::c_int) }
    s = (*strm).state as *mut DState;
    if s.is_null() { return -(2 as std::os::raw::c_int) }
    if (*s).strm != strm { return -(2 as std::os::raw::c_int) }
    if !(*s).tt.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).tt as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).ll16.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).ll16 as
                                                               *mut std::os::raw::c_void);
    }
    if !(*s).ll4.is_null() {
        (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                           (*s).ll4 as
                                                               *mut std::os::raw::c_void);
    }
    (*strm).bzfree.expect("non-null function pointer")((*strm).opaque,
                                                       (*strm).state);
    (*strm).state = 0 as *mut std::os::raw::c_void;
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------*/
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c: Int32 = fgetc(f);
    if c == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int as Bool }
    ungetc(c, f);
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteOpen(mut bzerror: *mut std::os::raw::c_int,
                                         mut f: *mut FILE,
                                         mut blockSize100k: std::os::raw::c_int,
                                         mut verbosity: std::os::raw::c_int,
                                         mut workFactor: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if f.is_null() ||
           (blockSize100k < 1 as std::os::raw::c_int ||
                blockSize100k > 9 as std::os::raw::c_int) ||
           (workFactor < 0 as std::os::raw::c_int || workFactor > 250 as std::os::raw::c_int)
           || (verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int) {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if ferror(f) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    bzf =
        malloc(::std::mem::size_of::<bzFile>() as std::os::raw::c_ulong) as
            *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = -(3 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(3 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    (*bzf).initialisedOk = 0 as std::os::raw::c_int as Bool;
    (*bzf).bufN = 0 as std::os::raw::c_int;
    (*bzf).handle = f;
    (*bzf).writing = 1 as std::os::raw::c_int as Bool;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut std::os::raw::c_void;
    if workFactor == 0 as std::os::raw::c_int { workFactor = 30 as std::os::raw::c_int }
    ret =
        BZ2_bzCompressInit(&mut (*bzf).strm, blockSize100k, verbosity,
                           workFactor);
    if ret != 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = ret }
        if !bzf.is_null() { (*bzf).lastErr = ret }
        free(bzf as *mut std::os::raw::c_void);
        return 0 as *mut std::os::raw::c_void
    }
    (*bzf).strm.avail_in = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*bzf).initialisedOk = 1 as std::os::raw::c_int as Bool;
    return bzf as *mut std::os::raw::c_void;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWrite(mut bzerror: *mut std::os::raw::c_int,
                                     mut b: *mut std::os::raw::c_void,
                                     mut buf: *mut std::os::raw::c_void,
                                     mut len: std::os::raw::c_int) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if bzf.is_null() || buf.is_null() || len < 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if ferror((*bzf).handle) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return
    }
    if len == 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return
    }
    (*bzf).strm.avail_in = len as std::os::raw::c_uint;
    (*bzf).strm.next_in = buf as *mut std::os::raw::c_char;
    while 1 as std::os::raw::c_int as Bool != 0 {
        (*bzf).strm.avail_out = 5000 as std::os::raw::c_int as std::os::raw::c_uint;
        (*bzf).strm.next_out = (*bzf).buf.as_mut_ptr();
        ret = BZ2_bzCompress(&mut (*bzf).strm, 0 as std::os::raw::c_int);
        if ret != 1 as std::os::raw::c_int {
            if !bzerror.is_null() { *bzerror = ret }
            if !bzf.is_null() { (*bzf).lastErr = ret }
            return
        }
        if (*bzf).strm.avail_out < 5000 as std::os::raw::c_int as std::os::raw::c_uint {
            n =
                (5000 as std::os::raw::c_int as
                     std::os::raw::c_uint).wrapping_sub((*bzf).strm.avail_out) as
                    Int32;
            n2 =
                fwrite((*bzf).buf.as_mut_ptr() as *mut std::os::raw::c_void,
                       ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                       n as std::os::raw::c_ulong, (*bzf).handle) as Int32;
            if n != n2 || ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
                if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
                return
            }
        }
        if (*bzf).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
            if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
            return
        }
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose(mut bzerror: *mut std::os::raw::c_int,
                                          mut b: *mut std::os::raw::c_void,
                                          mut abandon: std::os::raw::c_int,
                                          mut nbytes_in: *mut std::os::raw::c_uint,
                                          mut nbytes_out: *mut std::os::raw::c_uint) {
    BZ2_bzWriteClose64(bzerror, b, abandon, nbytes_in, 0 as *mut std::os::raw::c_uint,
                       nbytes_out, 0 as *mut std::os::raw::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose64(mut bzerror: *mut std::os::raw::c_int,
                                            mut b: *mut std::os::raw::c_void,
                                            mut abandon: std::os::raw::c_int,
                                            mut nbytes_in_lo32:
                                                *mut std::os::raw::c_uint,
                                            mut nbytes_in_hi32:
                                                *mut std::os::raw::c_uint,
                                            mut nbytes_out_lo32:
                                                *mut std::os::raw::c_uint,
                                            mut nbytes_out_hi32:
                                                *mut std::os::raw::c_uint) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if ferror((*bzf).handle) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = 0 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if abandon == 0 && (*bzf).lastErr == 0 as std::os::raw::c_int {
        while 1 as std::os::raw::c_int as Bool != 0 {
            (*bzf).strm.avail_out = 5000 as std::os::raw::c_int as std::os::raw::c_uint;
            (*bzf).strm.next_out = (*bzf).buf.as_mut_ptr();
            ret = BZ2_bzCompress(&mut (*bzf).strm, 2 as std::os::raw::c_int);
            if ret != 3 as std::os::raw::c_int && ret != 4 as std::os::raw::c_int {
                if !bzerror.is_null() { *bzerror = ret }
                if !bzf.is_null() { (*bzf).lastErr = ret }
                return
            }
            if (*bzf).strm.avail_out < 5000 as std::os::raw::c_int as std::os::raw::c_uint {
                n =
                    (5000 as std::os::raw::c_int as
                         std::os::raw::c_uint).wrapping_sub((*bzf).strm.avail_out) as
                        Int32;
                n2 =
                    fwrite((*bzf).buf.as_mut_ptr() as *mut std::os::raw::c_void,
                           ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                           n as std::os::raw::c_ulong, (*bzf).handle) as Int32;
                if n != n2 || ferror((*bzf).handle) != 0 {
                    if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
                    if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
                    return
                }
            }
            if ret == 4 as std::os::raw::c_int { break ; }
        }
    }
    if abandon == 0 && ferror((*bzf).handle) == 0 {
        fflush((*bzf).handle);
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
            if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
            return
        }
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = (*bzf).strm.total_in_lo32
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = (*bzf).strm.total_in_hi32
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = (*bzf).strm.total_out_lo32
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = (*bzf).strm.total_out_hi32
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    BZ2_bzCompressEnd(&mut (*bzf).strm);
    free(bzf as *mut std::os::raw::c_void);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadOpen(mut bzerror: *mut std::os::raw::c_int,
                                        mut f: *mut FILE,
                                        mut verbosity: std::os::raw::c_int,
                                        mut small: std::os::raw::c_int,
                                        mut unused: *mut std::os::raw::c_void,
                                        mut nUnused: std::os::raw::c_int)
 -> *mut std::os::raw::c_void {
    let mut bzf: *mut bzFile = 0 as *mut bzFile;
    let mut ret: std::os::raw::c_int = 0;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if f.is_null() || small != 0 as std::os::raw::c_int && small != 1 as std::os::raw::c_int
           || (verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int)
           || unused.is_null() && nUnused != 0 as std::os::raw::c_int ||
           !unused.is_null() &&
               (nUnused < 0 as std::os::raw::c_int || nUnused > 5000 as std::os::raw::c_int) {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if ferror(f) != 0 {
        if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    bzf =
        malloc(::std::mem::size_of::<bzFile>() as std::os::raw::c_ulong) as
            *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = -(3 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(3 as std::os::raw::c_int) }
        return 0 as *mut std::os::raw::c_void
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    (*bzf).initialisedOk = 0 as std::os::raw::c_int as Bool;
    (*bzf).handle = f;
    (*bzf).bufN = 0 as std::os::raw::c_int;
    (*bzf).writing = 0 as std::os::raw::c_int as Bool;
    (*bzf).strm.bzalloc = None;
    (*bzf).strm.bzfree = None;
    (*bzf).strm.opaque = 0 as *mut std::os::raw::c_void;
    while nUnused > 0 as std::os::raw::c_int {
        (*bzf).buf[(*bzf).bufN as usize] = *(unused as *mut UChar) as Char;
        (*bzf).bufN += 1;
        unused =
            (unused as *mut UChar).offset(1 as std::os::raw::c_int as isize) as
                *mut std::os::raw::c_void;
        nUnused -= 1
    }
    ret = BZ2_bzDecompressInit(&mut (*bzf).strm, verbosity, small);
    if ret != 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = ret }
        if !bzf.is_null() { (*bzf).lastErr = ret }
        free(bzf as *mut std::os::raw::c_void);
        return 0 as *mut std::os::raw::c_void
    }
    (*bzf).strm.avail_in = (*bzf).bufN as std::os::raw::c_uint;
    (*bzf).strm.next_in = (*bzf).buf.as_mut_ptr();
    (*bzf).initialisedOk = 1 as std::os::raw::c_int as Bool;
    return bzf as *mut std::os::raw::c_void;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadClose(mut bzerror: *mut std::os::raw::c_int,
                                         mut b: *mut std::os::raw::c_void) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if (*bzf).initialisedOk != 0 { BZ2_bzDecompressEnd(&mut (*bzf).strm); }
    free(bzf as *mut std::os::raw::c_void);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzRead(mut bzerror: *mut std::os::raw::c_int,
                                    mut b: *mut std::os::raw::c_void,
                                    mut buf: *mut std::os::raw::c_void,
                                    mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut n: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    if bzf.is_null() || buf.is_null() || len < 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return 0 as std::os::raw::c_int
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return 0 as std::os::raw::c_int
    }
    if len == 0 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
        if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
        return 0 as std::os::raw::c_int
    }
    (*bzf).strm.avail_out = len as std::os::raw::c_uint;
    (*bzf).strm.next_out = buf as *mut std::os::raw::c_char;
    while 1 as std::os::raw::c_int as Bool != 0 {
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
            if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
            return 0 as std::os::raw::c_int
        }
        if (*bzf).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
               myfeof((*bzf).handle) == 0 {
            n =
                fread((*bzf).buf.as_mut_ptr() as *mut std::os::raw::c_void,
                      ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                      5000 as std::os::raw::c_int as std::os::raw::c_ulong, (*bzf).handle) as
                    Int32;
            if ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() { *bzerror = -(6 as std::os::raw::c_int) }
                if !bzf.is_null() { (*bzf).lastErr = -(6 as std::os::raw::c_int) }
                return 0 as std::os::raw::c_int
            }
            (*bzf).bufN = n;
            (*bzf).strm.avail_in = (*bzf).bufN as std::os::raw::c_uint;
            (*bzf).strm.next_in = (*bzf).buf.as_mut_ptr()
        }
        ret = BZ2_bzDecompress(&mut (*bzf).strm);
        if ret != 0 as std::os::raw::c_int && ret != 4 as std::os::raw::c_int {
            if !bzerror.is_null() { *bzerror = ret }
            if !bzf.is_null() { (*bzf).lastErr = ret }
            return 0 as std::os::raw::c_int
        }
        if ret == 0 as std::os::raw::c_int &&
               myfeof((*bzf).handle) as std::os::raw::c_int != 0 &&
               (*bzf).strm.avail_in == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
               (*bzf).strm.avail_out > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if !bzerror.is_null() { *bzerror = -(7 as std::os::raw::c_int) }
            if !bzf.is_null() { (*bzf).lastErr = -(7 as std::os::raw::c_int) }
            return 0 as std::os::raw::c_int
        }
        if ret == 4 as std::os::raw::c_int {
            if !bzerror.is_null() { *bzerror = 4 as std::os::raw::c_int }
            if !bzf.is_null() { (*bzf).lastErr = 4 as std::os::raw::c_int }
            return (len as std::os::raw::c_uint).wrapping_sub((*bzf).strm.avail_out)
                       as std::os::raw::c_int
        }
        if (*bzf).strm.avail_out == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
            if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
            return len
        }
    }
    return 0 as std::os::raw::c_int;
    /*not reached*/
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadGetUnused(mut bzerror: *mut std::os::raw::c_int,
                                             mut b: *mut std::os::raw::c_void,
                                             mut unused:
                                                 *mut *mut std::os::raw::c_void,
                                             mut nUnused: *mut std::os::raw::c_int) {
    let mut bzf: *mut bzFile = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return
    }
    if (*bzf).lastErr != 4 as std::os::raw::c_int {
        if !bzerror.is_null() { *bzerror = -(1 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(1 as std::os::raw::c_int) }
        return
    }
    if unused.is_null() || nUnused.is_null() {
        if !bzerror.is_null() { *bzerror = -(2 as std::os::raw::c_int) }
        if !bzf.is_null() { (*bzf).lastErr = -(2 as std::os::raw::c_int) }
        return
    }
    if !bzerror.is_null() { *bzerror = 0 as std::os::raw::c_int }
    if !bzf.is_null() { (*bzf).lastErr = 0 as std::os::raw::c_int }
    *nUnused = (*bzf).strm.avail_in as std::os::raw::c_int;
    *unused = (*bzf).strm.next_in as *mut std::os::raw::c_void;
}
/*---------------------------------------------------*/
/*--- Misc convenience stuff                      ---*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffCompress(mut dest: *mut std::os::raw::c_char,
                                                  mut destLen:
                                                      *mut std::os::raw::c_uint,
                                                  mut source:
                                                      *mut std::os::raw::c_char,
                                                  mut sourceLen: std::os::raw::c_uint,
                                                  mut blockSize100k:
                                                      std::os::raw::c_int,
                                                  mut verbosity: std::os::raw::c_int,
                                                  mut workFactor: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut strm: bz_stream =
        bz_stream{next_in: 0 as *mut std::os::raw::c_char,
                  avail_in: 0,
                  total_in_lo32: 0,
                  total_in_hi32: 0,
                  next_out: 0 as *mut std::os::raw::c_char,
                  avail_out: 0,
                  total_out_lo32: 0,
                  total_out_hi32: 0,
                  state: 0 as *mut std::os::raw::c_void,
                  bzalloc: None,
                  bzfree: None,
                  opaque: 0 as *mut std::os::raw::c_void,};
    let mut ret: std::os::raw::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null() ||
           blockSize100k < 1 as std::os::raw::c_int ||
           blockSize100k > 9 as std::os::raw::c_int || verbosity < 0 as std::os::raw::c_int ||
           verbosity > 4 as std::os::raw::c_int || workFactor < 0 as std::os::raw::c_int ||
           workFactor > 250 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    if workFactor == 0 as std::os::raw::c_int { workFactor = 30 as std::os::raw::c_int }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut std::os::raw::c_void;
    ret = BZ2_bzCompressInit(&mut strm, blockSize100k, verbosity, workFactor);
    if ret != 0 as std::os::raw::c_int { return ret }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzCompress(&mut strm, 2 as std::os::raw::c_int);
    if ret == 3 as std::os::raw::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return -(8 as std::os::raw::c_int)
    } else if ret != 4 as std::os::raw::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return ret
    } else {
        /* normal termination */
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzCompressEnd(&mut strm);
        return 0 as std::os::raw::c_int
    };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffDecompress(mut dest:
                                                        *mut std::os::raw::c_char,
                                                    mut destLen:
                                                        *mut std::os::raw::c_uint,
                                                    mut source:
                                                        *mut std::os::raw::c_char,
                                                    mut sourceLen:
                                                        std::os::raw::c_uint,
                                                    mut small: std::os::raw::c_int,
                                                    mut verbosity:
                                                        std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut strm: bz_stream =
        bz_stream{next_in: 0 as *mut std::os::raw::c_char,
                  avail_in: 0,
                  total_in_lo32: 0,
                  total_in_hi32: 0,
                  next_out: 0 as *mut std::os::raw::c_char,
                  avail_out: 0,
                  total_out_lo32: 0,
                  total_out_hi32: 0,
                  state: 0 as *mut std::os::raw::c_void,
                  bzalloc: None,
                  bzfree: None,
                  opaque: 0 as *mut std::os::raw::c_void,};
    let mut ret: std::os::raw::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null() ||
           small != 0 as std::os::raw::c_int && small != 1 as std::os::raw::c_int ||
           verbosity < 0 as std::os::raw::c_int || verbosity > 4 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut std::os::raw::c_void;
    ret = BZ2_bzDecompressInit(&mut strm, verbosity, small);
    if ret != 0 as std::os::raw::c_int { return ret }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzDecompress(&mut strm);
    if ret == 0 as std::os::raw::c_int {
        if strm.avail_out > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            BZ2_bzDecompressEnd(&mut strm);
            return -(7 as std::os::raw::c_int)
        } else { BZ2_bzDecompressEnd(&mut strm); return -(8 as std::os::raw::c_int) }
    } else if ret != 4 as std::os::raw::c_int {
        BZ2_bzDecompressEnd(&mut strm);
        return ret
    } else {
        /* normal termination */
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzDecompressEnd(&mut strm);
        return 0 as std::os::raw::c_int
    };
}
/*---------------------------------------------------*/
/*--
   Code contributed by Yoshioka Tsuneo (tsuneo@rr.iij4u.or.jp)
   to support better zlib compatibility.
   This code is not _officially_ part of libbzip2 (yet);
   I haven't tested it, documented it, or considered the
   threading-safeness of it.
   If this code breaks, please contact both Yoshioka and me.
--*/
/*---------------------------------------------------*/
/*---------------------------------------------------*/
/*--
   return version like "0.9.5d, 4-Sept-1999".
--*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzlibVersion() -> *const std::os::raw::c_char {
    return b"1.0.8, 13-Jul-2019\x00" as *const u8 as *const std::os::raw::c_char;
}
/*---------------------------------------------------*/
unsafe extern "C" fn bzopen_or_bzdopen(mut path: *const std::os::raw::c_char,
                                       mut fd: std::os::raw::c_int,
                                       mut mode: *const std::os::raw::c_char,
                                       mut open_mode: std::os::raw::c_int)
 -> *mut std::os::raw::c_void 
 /* bzopen: 0, bzdopen:1 */
 {
    let mut bzerr: std::os::raw::c_int = 0; /* binary mode */
    let mut unused: [std::os::raw::c_char; 5000] = [0; 5000];
    let mut blockSize100k: std::os::raw::c_int = 9 as std::os::raw::c_int;
    let mut writing: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut mode2: [std::os::raw::c_char; 10] =
        *::std::mem::transmute::<&[u8; 10],
                                 &mut [std::os::raw::c_char; 10]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut bzfp: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut verbosity: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut workFactor: std::os::raw::c_int = 30 as std::os::raw::c_int;
    let mut smallMode: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut nUnused: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if mode.is_null() { return 0 as *mut std::os::raw::c_void }
    while *mode != 0 {
        match *mode as std::os::raw::c_int {
            114 => { writing = 0 as std::os::raw::c_int }
            119 => { writing = 1 as std::os::raw::c_int }
            115 => { smallMode = 1 as std::os::raw::c_int }
            _ => {
                if isdigit(*mode as std::os::raw::c_int) != 0 {
                    blockSize100k = *mode as std::os::raw::c_int - 0x30 as std::os::raw::c_int
                }
            }
        }
        mode = mode.offset(1)
    }
    strcat(mode2.as_mut_ptr(),
           if writing != 0 {
               b"w\x00" as *const u8 as *const std::os::raw::c_char
           } else { b"r\x00" as *const u8 as *const std::os::raw::c_char });
    strcat(mode2.as_mut_ptr(), b"b\x00" as *const u8 as *const std::os::raw::c_char);
    if open_mode == 0 as std::os::raw::c_int {
        if path.is_null() ||
               strcmp(path, b"\x00" as *const u8 as *const std::os::raw::c_char) ==
                   0 as std::os::raw::c_int {
            fp = if writing != 0 { __stdoutp } else { __stdinp }
        } else { fp = fopen(path, mode2.as_mut_ptr()) }
    } else { fp = fdopen(fd, mode2.as_mut_ptr()) }
    if fp.is_null() { return 0 as *mut std::os::raw::c_void }
    if writing != 0 {
        /* Guard against total chaos and anarchy -- JRS */
        if blockSize100k < 1 as std::os::raw::c_int {
            blockSize100k = 1 as std::os::raw::c_int
        }
        if blockSize100k > 9 as std::os::raw::c_int {
            blockSize100k = 9 as std::os::raw::c_int
        }
        bzfp =
            BZ2_bzWriteOpen(&mut bzerr, fp, blockSize100k, verbosity,
                            workFactor)
    } else {
        bzfp =
            BZ2_bzReadOpen(&mut bzerr, fp, verbosity, smallMode,
                           unused.as_mut_ptr() as *mut std::os::raw::c_void, nUnused)
    }
    if bzfp.is_null() {
        if fp != __stdinp && fp != __stdoutp { fclose(fp); }
        return 0 as *mut std::os::raw::c_void
    }
    return bzfp;
}
/*---------------------------------------------------*/
/*--
   open file for read or write.
      ex) bzopen("file","w9")
      case path="" or NULL => use stdin or stdout.
--*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzopen(mut path: *const std::os::raw::c_char,
                                    mut mode: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    return bzopen_or_bzdopen(path, -(1 as std::os::raw::c_int), mode,
                             0 as std::os::raw::c_int);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzdopen(mut fd: std::os::raw::c_int,
                                     mut mode: *const std::os::raw::c_char)
 -> *mut std::os::raw::c_void {
    return bzopen_or_bzdopen(0 as *const std::os::raw::c_char, fd, mode,
                             1 as std::os::raw::c_int);
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzread(mut b: *mut std::os::raw::c_void,
                                    mut buf: *mut std::os::raw::c_void,
                                    mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut bzerr: std::os::raw::c_int = 0;
    let mut nread: std::os::raw::c_int = 0;
    if (*(b as *mut bzFile)).lastErr == 4 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    nread = BZ2_bzRead(&mut bzerr, b, buf, len);
    if bzerr == 0 as std::os::raw::c_int || bzerr == 4 as std::os::raw::c_int {
        return nread
    } else { return -(1 as std::os::raw::c_int) };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzwrite(mut b: *mut std::os::raw::c_void,
                                     mut buf: *mut std::os::raw::c_void,
                                     mut len: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut bzerr: std::os::raw::c_int = 0;
    BZ2_bzWrite(&mut bzerr, b, buf, len);
    if bzerr == 0 as std::os::raw::c_int {
        return len
    } else { return -(1 as std::os::raw::c_int) };
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzflush(mut b: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    /* do nothing now... */
    return 0 as std::os::raw::c_int;
}
/*---------------------------------------------------*/
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzclose(mut b: *mut std::os::raw::c_void) {
    let mut bzerr: std::os::raw::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if b.is_null() { return }
    fp = (*(b as *mut bzFile)).handle;
    if (*(b as *mut bzFile)).writing != 0 {
        BZ2_bzWriteClose(&mut bzerr, b, 0 as std::os::raw::c_int,
                         0 as *mut std::os::raw::c_uint, 0 as *mut std::os::raw::c_uint);
        if bzerr != 0 as std::os::raw::c_int {
            BZ2_bzWriteClose(0 as *mut std::os::raw::c_int, b, 1 as std::os::raw::c_int,
                             0 as *mut std::os::raw::c_uint, 0 as *mut std::os::raw::c_uint);
        }
    } else { BZ2_bzReadClose(&mut bzerr, b); }
    if fp != __stdinp && fp != __stdoutp { fclose(fp); };
}
/*---------------------------------------------------*/
/*--
   return last error code 
--*/
static mut bzerrorstrings: [*const std::os::raw::c_char; 16] =
    [b"OK\x00" as *const u8 as *const std::os::raw::c_char,
     b"SEQUENCE_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"PARAM_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"MEM_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"DATA_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"DATA_ERROR_MAGIC\x00" as *const u8 as *const std::os::raw::c_char,
     b"IO_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"UNEXPECTED_EOF\x00" as *const u8 as *const std::os::raw::c_char,
     b"OUTBUFF_FULL\x00" as *const u8 as *const std::os::raw::c_char,
     b"CONFIG_ERROR\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char,
     b"???\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzerror(mut b: *mut std::os::raw::c_void,
                                     mut errnum: *mut std::os::raw::c_int)
 -> *const std::os::raw::c_char {
    let mut err: std::os::raw::c_int = (*(b as *mut bzFile)).lastErr;
    if err > 0 as std::os::raw::c_int { err = 0 as std::os::raw::c_int }
    *errnum = err;
    return bzerrorstrings[(err * -(1 as std::os::raw::c_int)) as usize];
}
/*-------------------------------------------------------------*/
/*--- end                                           bzlib.c ---*/
/*-------------------------------------------------------------*/
