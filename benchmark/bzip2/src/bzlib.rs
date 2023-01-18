use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn BZ2_compressBlock(_: *mut EState, _: Bool);
    static mut BZ2_crc32Table: [UInt32; 256];
    fn BZ2_decompress(_: *mut DState) -> Int32;
    static mut BZ2_rNums: [Int32; 512];
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type UInt32 = libc::c_uint;
pub type Int32 = libc::c_int;
pub type UChar = libc::c_uchar;
pub type Bool = libc::c_uchar;
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
pub type Char = libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn BZ2_bz__AssertH__fail(mut errcode: libc::c_int) {
    fprintf(
        stderr,
        b"\n\nbzip2/libbzip2: internal error number %d.\nThis is a bug in bzip2/libbzip2, %s.\nPlease report it to: bzip2-devel@sourceware.org.  If this happened\nwhen you were using some program which uses libbzip2 as a\ncomponent, you should also report this bug to the author(s)\nof that program.  Please make an effort to report this bug;\ntimely and accurate bug reports eventually lead to higher\nquality software.  Thanks.\n\n\0"
            as *const u8 as *const libc::c_char,
        errcode,
        BZ2_bzlibVersion(),
    );
    if errcode == 1007 as libc::c_int {
        fprintf(
            stderr,
            b"\n*** A special note about internal error number 1007 ***\n\nExperience suggests that a common cause of i.e. 1007\nis unreliable memory or other hardware.  The 1007 assertion\njust happens to cross-check the results of huge numbers of\nmemory reads/writes, and so acts (unintendedly) as a stress\ntest of your memory system.\n\nI suggest the following: try compressing the file again,\npossibly monitoring progress in detail with the -vv flag.\n\n* If the error cannot be reproduced, and/or happens at different\n  points in compression, you may have a flaky memory system.\n  Try a memory-test program.  I have used Memtest86\n  (www.memtest86.com).  At the time of writing it is free (GPLd).\n  Memtest86 tests memory much more thorougly than your BIOSs\n  power-on test, and may find failures that the BIOS doesn't.\n\n* If the error can be repeatably reproduced, this is a bug in\n  bzip2, and I would very much like to hear about it.  Please\n  let me know, and, ideally, save a copy of the file causing the\n  problem -- without which I will be unable to investigate it.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    exit(3 as libc::c_int);
}
unsafe extern "C" fn bz_config_ok() -> libc::c_int {
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        != 4 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
        != 2 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
        != 1 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn default_bzalloc(
    mut opaque: *mut libc::c_void,
    mut items: Int32,
    mut size: Int32,
) -> *mut libc::c_void {
    let mut v = malloc((items * size) as libc::c_ulong);
    return v;
}
unsafe extern "C" fn default_bzfree(
    mut opaque: *mut libc::c_void,
    mut addr: *mut libc::c_void,
) {
    if !addr.is_null() {
        free(addr);
    }
}
unsafe extern "C" fn prepare_new_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    (*s).nblock = 0 as libc::c_int;
    (*s).numZ = 0 as libc::c_int;
    (*s).state_out_pos = 0 as libc::c_int;
    (*s).blockCRC = 0xffffffff as libc::c_long as UInt32;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (*s).inUse[i as usize] = 0 as libc::c_int as Bool;
        i += 1;
    }
    let ref mut fresh0 = (*s).blockNo;
    *fresh0 += 1;
}
unsafe extern "C" fn init_RL(mut s: *mut EState) {
    (*s).state_in_ch = 256 as libc::c_int as UInt32;
    (*s).state_in_len = 0 as libc::c_int;
}
unsafe extern "C" fn isempty_RL(mut s: *mut EState) -> Bool {
    if (*s).state_in_ch < 256 as libc::c_int as libc::c_uint
        && (*s).state_in_len > 0 as libc::c_int
    {
        return 0 as libc::c_int as Bool
    } else {
        return 1 as libc::c_int as Bool
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressInit(
    mut strm: *mut bz_stream,
    mut blockSize100k: libc::c_int,
    mut verbosity: libc::c_int,
    mut workFactor: libc::c_int,
) -> libc::c_int {
    let mut n: Int32 = 0;
    let mut s = 0 as *mut EState;
    if bz_config_ok() == 0 {
        return -(9 as libc::c_int);
    }
    if strm.is_null() || blockSize100k < 1 as libc::c_int
        || blockSize100k > 9 as libc::c_int || workFactor < 0 as libc::c_int
        || workFactor > 250 as libc::c_int
    {
        return -(2 as libc::c_int);
    }
    if workFactor == 0 as libc::c_int {
        workFactor = 30 as libc::c_int;
    }
    if ((*strm).bzalloc).is_none() {
        let ref mut fresh1 = (*strm).bzalloc;
        *fresh1 = Some(
            default_bzalloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    Int32,
                    Int32,
                ) -> *mut libc::c_void,
        );
    }
    if ((*strm).bzfree).is_none() {
        let ref mut fresh2 = (*strm).bzfree;
        *fresh2 = Some(
            default_bzfree
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        );
    }
    s = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        ::std::mem::size_of::<EState>() as libc::c_ulong as libc::c_int,
        1 as libc::c_int,
    ) as *mut EState;
    if s.is_null() {
        return -(3 as libc::c_int);
    }
    let ref mut fresh3 = (*s).strm;
    *fresh3 = strm;
    let ref mut fresh4 = (*s).arr1;
    *fresh4 = 0 as *mut UInt32;
    let ref mut fresh5 = (*s).arr2;
    *fresh5 = 0 as *mut UInt32;
    let ref mut fresh6 = (*s).ftab;
    *fresh6 = 0 as *mut UInt32;
    n = 100000 as libc::c_int * blockSize100k;
    let ref mut fresh7 = (*s).arr1;
    *fresh7 = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<UInt32>() as libc::c_ulong)
            as libc::c_int,
        1 as libc::c_int,
    ) as *mut UInt32;
    let ref mut fresh8 = (*s).arr2;
    *fresh8 = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        ((n
            + (2 as libc::c_int + 12 as libc::c_int + 18 as libc::c_int
                + 2 as libc::c_int)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<UInt32>() as libc::c_ulong)
            as libc::c_int,
        1 as libc::c_int,
    ) as *mut UInt32;
    let ref mut fresh9 = (*s).ftab;
    *fresh9 = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        (65537 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<UInt32>() as libc::c_ulong)
            as libc::c_int,
        1 as libc::c_int,
    ) as *mut UInt32;
    if ((*s).arr1).is_null() || ((*s).arr2).is_null() || ((*s).ftab).is_null() {
        if !((*s).arr1).is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, (*s).arr1 as *mut libc::c_void);
        }
        if !((*s).arr2).is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, (*s).arr2 as *mut libc::c_void);
        }
        if !((*s).ftab).is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, (*s).ftab as *mut libc::c_void);
        }
        if !s.is_null() {
            ((*strm).bzfree)
                .expect(
                    "non-null function pointer",
                )((*strm).opaque, s as *mut libc::c_void);
        }
        return -(3 as libc::c_int);
    }
    (*s).blockNo = 0 as libc::c_int;
    (*s).state = 2 as libc::c_int;
    (*s).mode = 2 as libc::c_int;
    (*s).combinedCRC = 0 as libc::c_int as UInt32;
    (*s).blockSize100k = blockSize100k;
    (*s).nblockMAX = 100000 as libc::c_int * blockSize100k - 19 as libc::c_int;
    (*s).verbosity = verbosity;
    (*s).workFactor = workFactor;
    let ref mut fresh10 = (*s).block;
    *fresh10 = (*s).arr2 as *mut UChar;
    let ref mut fresh11 = (*s).mtfv;
    *fresh11 = (*s).arr1 as *mut UInt16;
    let ref mut fresh12 = (*s).zbits;
    *fresh12 = 0 as *mut UChar;
    let ref mut fresh13 = (*s).ptr;
    *fresh13 = (*s).arr1;
    let ref mut fresh14 = (*strm).state;
    *fresh14 = s as *mut libc::c_void;
    (*strm).total_in_lo32 = 0 as libc::c_int as libc::c_uint;
    (*strm).total_in_hi32 = 0 as libc::c_int as libc::c_uint;
    (*strm).total_out_lo32 = 0 as libc::c_int as libc::c_uint;
    (*strm).total_out_hi32 = 0 as libc::c_int as libc::c_uint;
    init_RL(s);
    prepare_new_block(s);
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_pair_to_block(mut s: *mut EState) {
    let mut i: Int32 = 0;
    let mut ch = (*s).state_in_ch as UChar;
    i = 0 as libc::c_int;
    while i < (*s).state_in_len {
        (*s)
            .blockCRC = (*s).blockCRC << 8 as libc::c_int
            ^ BZ2_crc32Table[((*s).blockCRC >> 24 as libc::c_int ^ ch as libc::c_uint)
                as usize];
        i += 1;
    }
    (*s).inUse[(*s).state_in_ch as usize] = 1 as libc::c_int as Bool;
    match (*s).state_in_len {
        1 => {
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh15 = (*s).nblock;
            *fresh15 += 1;
        }
        2 => {
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh16 = (*s).nblock;
            *fresh16 += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh17 = (*s).nblock;
            *fresh17 += 1;
        }
        3 => {
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh18 = (*s).nblock;
            *fresh18 += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh19 = (*s).nblock;
            *fresh19 += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh20 = (*s).nblock;
            *fresh20 += 1;
        }
        _ => {
            (*s)
                .inUse[((*s).state_in_len - 4 as libc::c_int)
                as usize] = 1 as libc::c_int as Bool;
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh21 = (*s).nblock;
            *fresh21 += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh22 = (*s).nblock;
            *fresh22 += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh23 = (*s).nblock;
            *fresh23 += 1;
            *((*s).block).offset((*s).nblock as isize) = ch;
            let ref mut fresh24 = (*s).nblock;
            *fresh24 += 1;
            *((*s).block)
                .offset(
                    (*s).nblock as isize,
                ) = ((*s).state_in_len - 4 as libc::c_int) as UChar;
            let ref mut fresh25 = (*s).nblock;
            *fresh25 += 1;
        }
    };
}
unsafe extern "C" fn flush_RL(mut s: *mut EState) {
    if (*s).state_in_ch < 256 as libc::c_int as libc::c_uint {
        add_pair_to_block(s);
    }
    init_RL(s);
}
unsafe extern "C" fn copy_input_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_in = 0 as libc::c_int as Bool;
    if (*s).mode == 2 as libc::c_int {
        while 1 as libc::c_int as Bool != 0 {
            if (*s).nblock >= (*s).nblockMAX {
                break;
            }
            if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                break;
            }
            progress_in = 1 as libc::c_int as Bool;
            let mut zchh = *((*(*s).strm).next_in as *mut UChar) as UInt32;
            if zchh != (*s).state_in_ch && (*s).state_in_len == 1 as libc::c_int {
                let mut ch = (*s).state_in_ch as UChar;
                (*s)
                    .blockCRC = (*s).blockCRC << 8 as libc::c_int
                    ^ BZ2_crc32Table[((*s).blockCRC >> 24 as libc::c_int
                        ^ ch as libc::c_uint) as usize];
                (*s).inUse[(*s).state_in_ch as usize] = 1 as libc::c_int as Bool;
                *((*s).block).offset((*s).nblock as isize) = ch;
                let ref mut fresh26 = (*s).nblock;
                *fresh26 += 1;
                (*s).state_in_ch = zchh;
            } else if zchh != (*s).state_in_ch || (*s).state_in_len == 255 as libc::c_int
            {
                if (*s).state_in_ch < 256 as libc::c_int as libc::c_uint {
                    add_pair_to_block(s);
                }
                (*s).state_in_ch = zchh;
                (*s).state_in_len = 1 as libc::c_int;
            } else {
                let ref mut fresh27 = (*s).state_in_len;
                *fresh27 += 1;
            }
            let ref mut fresh28 = (*(*s).strm).next_in;
            *fresh28 = (*fresh28).offset(1);
            let ref mut fresh29 = (*(*s).strm).avail_in;
            *fresh29 = (*fresh29).wrapping_sub(1);
            let ref mut fresh30 = (*(*s).strm).total_in_lo32;
            *fresh30 = (*fresh30).wrapping_add(1);
            if (*(*s).strm).total_in_lo32 == 0 as libc::c_int as libc::c_uint {
                let ref mut fresh31 = (*(*s).strm).total_in_hi32;
                *fresh31 = (*fresh31).wrapping_add(1);
            }
        }
    } else {
        while 1 as libc::c_int as Bool != 0 {
            if (*s).nblock >= (*s).nblockMAX {
                break;
            }
            if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
                break;
            }
            if (*s).avail_in_expect == 0 as libc::c_int as libc::c_uint {
                break;
            }
            progress_in = 1 as libc::c_int as Bool;
            let mut zchh_0 = *((*(*s).strm).next_in as *mut UChar) as UInt32;
            if zchh_0 != (*s).state_in_ch && (*s).state_in_len == 1 as libc::c_int {
                let mut ch_0 = (*s).state_in_ch as UChar;
                (*s)
                    .blockCRC = (*s).blockCRC << 8 as libc::c_int
                    ^ BZ2_crc32Table[((*s).blockCRC >> 24 as libc::c_int
                        ^ ch_0 as libc::c_uint) as usize];
                (*s).inUse[(*s).state_in_ch as usize] = 1 as libc::c_int as Bool;
                *((*s).block).offset((*s).nblock as isize) = ch_0;
                let ref mut fresh32 = (*s).nblock;
                *fresh32 += 1;
                (*s).state_in_ch = zchh_0;
            } else if zchh_0 != (*s).state_in_ch
                || (*s).state_in_len == 255 as libc::c_int
            {
                if (*s).state_in_ch < 256 as libc::c_int as libc::c_uint {
                    add_pair_to_block(s);
                }
                (*s).state_in_ch = zchh_0;
                (*s).state_in_len = 1 as libc::c_int;
            } else {
                let ref mut fresh33 = (*s).state_in_len;
                *fresh33 += 1;
            }
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
            let ref mut fresh38 = (*s).avail_in_expect;
            *fresh38 = (*fresh38).wrapping_sub(1);
        }
    }
    return progress_in;
}
unsafe extern "C" fn copy_output_until_stop(mut s: *mut EState) -> Bool {
    let mut progress_out = 0 as libc::c_int as Bool;
    while 1 as libc::c_int as Bool != 0 {
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            break;
        }
        if (*s).state_out_pos >= (*s).numZ {
            break;
        }
        progress_out = 1 as libc::c_int as Bool;
        *(*(*s).strm)
            .next_out = *((*s).zbits).offset((*s).state_out_pos as isize)
            as libc::c_char;
        let ref mut fresh39 = (*s).state_out_pos;
        *fresh39 += 1;
        let ref mut fresh40 = (*(*s).strm).avail_out;
        *fresh40 = (*fresh40).wrapping_sub(1);
        let ref mut fresh41 = (*(*s).strm).next_out;
        *fresh41 = (*fresh41).offset(1);
        let ref mut fresh42 = (*(*s).strm).total_out_lo32;
        *fresh42 = (*fresh42).wrapping_add(1);
        if (*(*s).strm).total_out_lo32 == 0 as libc::c_int as libc::c_uint {
            let ref mut fresh43 = (*(*s).strm).total_out_hi32;
            *fresh43 = (*fresh43).wrapping_add(1);
        }
    }
    return progress_out;
}
unsafe extern "C" fn handle_compress(mut strm: *mut bz_stream) -> Bool {
    let mut progress_in = 0 as libc::c_int as Bool;
    let mut progress_out = 0 as libc::c_int as Bool;
    let mut s = (*strm).state as *mut EState;
    while 1 as libc::c_int as Bool != 0 {
        if (*s).state == 1 as libc::c_int {
            progress_out = (progress_out as libc::c_int
                | copy_output_until_stop(s) as libc::c_int) as Bool;
            if (*s).state_out_pos < (*s).numZ {
                break;
            }
            if (*s).mode == 4 as libc::c_int
                && (*s).avail_in_expect == 0 as libc::c_int as libc::c_uint
                && isempty_RL(s) as libc::c_int != 0
            {
                break;
            }
            prepare_new_block(s);
            (*s).state = 2 as libc::c_int;
            if (*s).mode == 3 as libc::c_int
                && (*s).avail_in_expect == 0 as libc::c_int as libc::c_uint
                && isempty_RL(s) as libc::c_int != 0
            {
                break;
            }
        }
        if !((*s).state == 2 as libc::c_int) {
            continue;
        }
        progress_in = (progress_in as libc::c_int
            | copy_input_until_stop(s) as libc::c_int) as Bool;
        if (*s).mode != 2 as libc::c_int
            && (*s).avail_in_expect == 0 as libc::c_int as libc::c_uint
        {
            flush_RL(s);
            BZ2_compressBlock(s, ((*s).mode == 4 as libc::c_int) as libc::c_int as Bool);
            (*s).state = 1 as libc::c_int;
        } else if (*s).nblock >= (*s).nblockMAX {
            BZ2_compressBlock(s, 0 as libc::c_int as Bool);
            (*s).state = 1 as libc::c_int;
        } else if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
            break;
        }
    }
    return (progress_in as libc::c_int != 0 || progress_out as libc::c_int != 0)
        as libc::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompress(
    mut strm: *mut bz_stream,
    mut action: libc::c_int,
) -> libc::c_int {
    let mut progress: Bool = 0;
    let mut s = 0 as *mut EState;
    if strm.is_null() {
        return -(2 as libc::c_int);
    }
    s = (*strm).state as *mut EState;
    if s.is_null() {
        return -(2 as libc::c_int);
    }
    if (*s).strm != strm {
        return -(2 as libc::c_int);
    }
    loop {
        match (*s).mode {
            1 => return -(1 as libc::c_int),
            2 => {
                if action == 0 as libc::c_int {
                    progress = handle_compress(strm);
                    return if progress as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        -(2 as libc::c_int)
                    };
                } else if action == 1 as libc::c_int {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 3 as libc::c_int;
                } else if action == 2 as libc::c_int {
                    (*s).avail_in_expect = (*strm).avail_in;
                    (*s).mode = 4 as libc::c_int;
                } else {
                    return -(2 as libc::c_int)
                }
            }
            3 => {
                if action != 1 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as libc::c_int);
                }
                progress = handle_compress(strm);
                if (*s).avail_in_expect > 0 as libc::c_int as libc::c_uint
                    || isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ
                {
                    return 2 as libc::c_int;
                }
                (*s).mode = 2 as libc::c_int;
                return 1 as libc::c_int;
            }
            4 => {
                if action != 2 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if (*s).avail_in_expect != (*(*s).strm).avail_in {
                    return -(1 as libc::c_int);
                }
                progress = handle_compress(strm);
                if progress == 0 {
                    return -(1 as libc::c_int);
                }
                if (*s).avail_in_expect > 0 as libc::c_int as libc::c_uint
                    || isempty_RL(s) == 0 || (*s).state_out_pos < (*s).numZ
                {
                    return 3 as libc::c_int;
                }
                (*s).mode = 1 as libc::c_int;
                return 4 as libc::c_int;
            }
            _ => return 0 as libc::c_int,
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzCompressEnd(mut strm: *mut bz_stream) -> libc::c_int {
    let mut s = 0 as *mut EState;
    if strm.is_null() {
        return -(2 as libc::c_int);
    }
    s = (*strm).state as *mut EState;
    if s.is_null() {
        return -(2 as libc::c_int);
    }
    if (*s).strm != strm {
        return -(2 as libc::c_int);
    }
    if !((*s).arr1).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).arr1 as *mut libc::c_void);
    }
    if !((*s).arr2).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).arr2 as *mut libc::c_void);
    }
    if !((*s).ftab).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).ftab as *mut libc::c_void);
    }
    ((*strm).bzfree).expect("non-null function pointer")((*strm).opaque, (*strm).state);
    let ref mut fresh44 = (*strm).state;
    *fresh44 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressInit(
    mut strm: *mut bz_stream,
    mut verbosity: libc::c_int,
    mut small: libc::c_int,
) -> libc::c_int {
    let mut s = 0 as *mut DState;
    if bz_config_ok() == 0 {
        return -(9 as libc::c_int);
    }
    if strm.is_null() {
        return -(2 as libc::c_int);
    }
    if small != 0 as libc::c_int && small != 1 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if verbosity < 0 as libc::c_int || verbosity > 4 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if ((*strm).bzalloc).is_none() {
        let ref mut fresh45 = (*strm).bzalloc;
        *fresh45 = Some(
            default_bzalloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    Int32,
                    Int32,
                ) -> *mut libc::c_void,
        );
    }
    if ((*strm).bzfree).is_none() {
        let ref mut fresh46 = (*strm).bzfree;
        *fresh46 = Some(
            default_bzfree
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        );
    }
    s = ((*strm).bzalloc)
        .expect(
            "non-null function pointer",
        )(
        (*strm).opaque,
        ::std::mem::size_of::<DState>() as libc::c_ulong as libc::c_int,
        1 as libc::c_int,
    ) as *mut DState;
    if s.is_null() {
        return -(3 as libc::c_int);
    }
    let ref mut fresh47 = (*s).strm;
    *fresh47 = strm;
    let ref mut fresh48 = (*strm).state;
    *fresh48 = s as *mut libc::c_void;
    (*s).state = 10 as libc::c_int;
    (*s).bsLive = 0 as libc::c_int;
    (*s).bsBuff = 0 as libc::c_int as UInt32;
    (*s).calculatedCombinedCRC = 0 as libc::c_int as UInt32;
    (*strm).total_in_lo32 = 0 as libc::c_int as libc::c_uint;
    (*strm).total_in_hi32 = 0 as libc::c_int as libc::c_uint;
    (*strm).total_out_lo32 = 0 as libc::c_int as libc::c_uint;
    (*strm).total_out_hi32 = 0 as libc::c_int as libc::c_uint;
    (*s).smallDecompress = small as Bool;
    let ref mut fresh49 = (*s).ll4;
    *fresh49 = 0 as *mut UChar;
    let ref mut fresh50 = (*s).ll16;
    *fresh50 = 0 as *mut UInt16;
    let ref mut fresh51 = (*s).tt;
    *fresh51 = 0 as *mut UInt32;
    (*s).currBlockNo = 0 as libc::c_int;
    (*s).verbosity = verbosity;
    return 0 as libc::c_int;
}
unsafe extern "C" fn unRLE_obuf_to_output_FAST(mut s: *mut DState) -> Bool {
    let mut current_block: u64;
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as libc::c_int as Bool != 0 {
            while 1 as libc::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int as Bool;
                }
                if (*s).state_out_len == 0 as libc::c_int {
                    break;
                }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s)
                    .calculatedBlockCRC = (*s).calculatedBlockCRC << 8 as libc::c_int
                    ^ BZ2_crc32Table[((*s).calculatedBlockCRC >> 24 as libc::c_int
                        ^ (*s).state_out_ch as libc::c_uint) as usize];
                let ref mut fresh52 = (*s).state_out_len;
                *fresh52 -= 1;
                let ref mut fresh53 = (*(*s).strm).next_out;
                *fresh53 = (*fresh53).offset(1);
                let ref mut fresh54 = (*(*s).strm).avail_out;
                *fresh54 = (*fresh54).wrapping_sub(1);
                let ref mut fresh55 = (*(*s).strm).total_out_lo32;
                *fresh55 = (*fresh55).wrapping_add(1);
                if (*(*s).strm).total_out_lo32 == 0 as libc::c_int as libc::c_uint {
                    let ref mut fresh56 = (*(*s).strm).total_out_hi32;
                    *fresh56 = (*fresh56).wrapping_add(1);
                }
            }
            if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                return 0 as libc::c_int as Bool;
            }
            if (*s).nblock_used > (*s).save_nblock + 1 as libc::c_int {
                return 1 as libc::c_int as Bool;
            }
            (*s).state_out_len = 1 as libc::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos
                >= (100000 as libc::c_int as UInt32)
                    .wrapping_mul((*s).blockSize100k as UInt32)
            {
                return 1 as libc::c_int as Bool;
            }
            (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
            k1 = ((*s).tPos & 0xff as libc::c_int as libc::c_uint) as UChar;
            (*s).tPos >>= 8 as libc::c_int;
            if (*s).rNToGo == 0 as libc::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                let ref mut fresh57 = (*s).rTPos;
                *fresh57 += 1;
                if (*s).rTPos == 512 as libc::c_int {
                    (*s).rTPos = 0 as libc::c_int;
                }
            }
            let ref mut fresh58 = (*s).rNToGo;
            *fresh58 -= 1;
            k1 = (k1 as libc::c_int
                ^ if (*s).rNToGo == 1 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as UChar;
            let ref mut fresh59 = (*s).nblock_used;
            *fresh59 += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                continue;
            }
            if k1 as libc::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32;
            } else {
                (*s).state_out_len = 2 as libc::c_int;
                if (*s).tPos
                    >= (100000 as libc::c_int as UInt32)
                        .wrapping_mul((*s).blockSize100k as UInt32)
                {
                    return 1 as libc::c_int as Bool;
                }
                (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                k1 = ((*s).tPos & 0xff as libc::c_int as libc::c_uint) as UChar;
                (*s).tPos >>= 8 as libc::c_int;
                if (*s).rNToGo == 0 as libc::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    let ref mut fresh60 = (*s).rTPos;
                    *fresh60 += 1;
                    if (*s).rTPos == 512 as libc::c_int {
                        (*s).rTPos = 0 as libc::c_int;
                    }
                }
                let ref mut fresh61 = (*s).rNToGo;
                *fresh61 -= 1;
                k1 = (k1 as libc::c_int
                    ^ if (*s).rNToGo == 1 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as UChar;
                let ref mut fresh62 = (*s).nblock_used;
                *fresh62 += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                    continue;
                }
                if k1 as libc::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32;
                } else {
                    (*s).state_out_len = 3 as libc::c_int;
                    if (*s).tPos
                        >= (100000 as libc::c_int as UInt32)
                            .wrapping_mul((*s).blockSize100k as UInt32)
                    {
                        return 1 as libc::c_int as Bool;
                    }
                    (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                    k1 = ((*s).tPos & 0xff as libc::c_int as libc::c_uint) as UChar;
                    (*s).tPos >>= 8 as libc::c_int;
                    if (*s).rNToGo == 0 as libc::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        let ref mut fresh63 = (*s).rTPos;
                        *fresh63 += 1;
                        if (*s).rTPos == 512 as libc::c_int {
                            (*s).rTPos = 0 as libc::c_int;
                        }
                    }
                    let ref mut fresh64 = (*s).rNToGo;
                    *fresh64 -= 1;
                    k1 = (k1 as libc::c_int
                        ^ if (*s).rNToGo == 1 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as UChar;
                    let ref mut fresh65 = (*s).nblock_used;
                    *fresh65 += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                        continue;
                    }
                    if k1 as libc::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32;
                    } else {
                        if (*s).tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
                        }
                        (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                        k1 = ((*s).tPos & 0xff as libc::c_int as libc::c_uint) as UChar;
                        (*s).tPos >>= 8 as libc::c_int;
                        if (*s).rNToGo == 0 as libc::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            let ref mut fresh66 = (*s).rTPos;
                            *fresh66 += 1;
                            if (*s).rTPos == 512 as libc::c_int {
                                (*s).rTPos = 0 as libc::c_int;
                            }
                        }
                        let ref mut fresh67 = (*s).rNToGo;
                        *fresh67 -= 1;
                        k1 = (k1 as libc::c_int
                            ^ if (*s).rNToGo == 1 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) as UChar;
                        let ref mut fresh68 = (*s).nblock_used;
                        *fresh68 += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as libc::c_int;
                        if (*s).tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
                        }
                        (*s).tPos = *((*s).tt).offset((*s).tPos as isize);
                        (*s)
                            .k0 = ((*s).tPos & 0xff as libc::c_int as libc::c_uint)
                            as UChar as Int32;
                        (*s).tPos >>= 8 as libc::c_int;
                        if (*s).rNToGo == 0 as libc::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            let ref mut fresh69 = (*s).rTPos;
                            *fresh69 += 1;
                            if (*s).rTPos == 512 as libc::c_int {
                                (*s).rTPos = 0 as libc::c_int;
                            }
                        }
                        let ref mut fresh70 = (*s).rNToGo;
                        *fresh70 -= 1;
                        let ref mut fresh71 = (*s).k0;
                        *fresh71
                            ^= if (*s).rNToGo == 1 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                        let ref mut fresh72 = (*s).nblock_used;
                        *fresh72 += 1;
                    }
                }
            }
        }
    } else {
        let mut c_calculatedBlockCRC = (*s).calculatedBlockCRC;
        let mut c_state_out_ch = (*s).state_out_ch;
        let mut c_state_out_len = (*s).state_out_len;
        let mut c_nblock_used = (*s).nblock_used;
        let mut c_k0 = (*s).k0;
        let mut c_tt = (*s).tt;
        let mut c_tPos = (*s).tPos;
        let mut cs_next_out = (*(*s).strm).next_out;
        let mut cs_avail_out = (*(*s).strm).avail_out;
        let mut ro_blockSize100k = (*s).blockSize100k;
        let mut avail_out_INIT = cs_avail_out;
        let mut s_save_nblockPP = (*s).save_nblock + 1 as libc::c_int;
        let mut total_out_lo32_old: libc::c_uint = 0;
        's_569: while 1 as libc::c_int as Bool != 0 {
            if c_state_out_len > 0 as libc::c_int {
                while 1 as libc::c_int as Bool != 0 {
                    if cs_avail_out == 0 as libc::c_int as libc::c_uint {
                        break 's_569;
                    }
                    if c_state_out_len == 1 as libc::c_int {
                        break;
                    }
                    *(cs_next_out as *mut UChar) = c_state_out_ch;
                    c_calculatedBlockCRC = c_calculatedBlockCRC << 8 as libc::c_int
                        ^ BZ2_crc32Table[(c_calculatedBlockCRC >> 24 as libc::c_int
                            ^ c_state_out_ch as libc::c_uint) as usize];
                    c_state_out_len -= 1;
                    cs_next_out = cs_next_out.offset(1);
                    cs_avail_out = cs_avail_out.wrapping_sub(1);
                }
                current_block = 16910810822589621899;
            } else {
                current_block = 3024573345131975588;
            }
            loop {
                match current_block {
                    16910810822589621899 => {
                        if cs_avail_out == 0 as libc::c_int as libc::c_uint {
                            c_state_out_len = 1 as libc::c_int;
                            break 's_569;
                        } else {
                            *(cs_next_out as *mut UChar) = c_state_out_ch;
                            c_calculatedBlockCRC = c_calculatedBlockCRC
                                << 8 as libc::c_int
                                ^ BZ2_crc32Table[(c_calculatedBlockCRC >> 24 as libc::c_int
                                    ^ c_state_out_ch as libc::c_uint) as usize];
                            cs_next_out = cs_next_out.offset(1);
                            cs_avail_out = cs_avail_out.wrapping_sub(1);
                            current_block = 3024573345131975588;
                        }
                    }
                    _ => {
                        if c_nblock_used > s_save_nblockPP {
                            return 1 as libc::c_int as Bool;
                        }
                        if c_nblock_used == s_save_nblockPP {
                            c_state_out_len = 0 as libc::c_int;
                            break 's_569;
                        } else {
                            c_state_out_ch = c_k0 as UChar;
                            if c_tPos
                                >= (100000 as libc::c_int as UInt32)
                                    .wrapping_mul(ro_blockSize100k as UInt32)
                            {
                                return 1 as libc::c_int as Bool;
                            }
                            c_tPos = *c_tt.offset(c_tPos as isize);
                            k1 = (c_tPos & 0xff as libc::c_int as libc::c_uint) as UChar;
                            c_tPos >>= 8 as libc::c_int;
                            c_nblock_used += 1;
                            if k1 as libc::c_int != c_k0 {
                                c_k0 = k1 as Int32;
                                current_block = 16910810822589621899;
                            } else {
                                if c_nblock_used == s_save_nblockPP {
                                    current_block = 16910810822589621899;
                                    continue;
                                }
                                c_state_out_len = 2 as libc::c_int;
                                if c_tPos
                                    >= (100000 as libc::c_int as UInt32)
                                        .wrapping_mul(ro_blockSize100k as UInt32)
                                {
                                    return 1 as libc::c_int as Bool;
                                }
                                c_tPos = *c_tt.offset(c_tPos as isize);
                                k1 = (c_tPos & 0xff as libc::c_int as libc::c_uint)
                                    as UChar;
                                c_tPos >>= 8 as libc::c_int;
                                c_nblock_used += 1;
                                if c_nblock_used == s_save_nblockPP {
                                    continue 's_569;
                                }
                                if k1 as libc::c_int != c_k0 {
                                    current_block = 18139099716546303047;
                                    break;
                                } else {
                                    current_block = 919396821984190499;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                18139099716546303047 => {
                    c_k0 = k1 as Int32;
                }
                _ => {
                    c_state_out_len = 3 as libc::c_int;
                    if c_tPos
                        >= (100000 as libc::c_int as UInt32)
                            .wrapping_mul(ro_blockSize100k as UInt32)
                    {
                        return 1 as libc::c_int as Bool;
                    }
                    c_tPos = *c_tt.offset(c_tPos as isize);
                    k1 = (c_tPos & 0xff as libc::c_int as libc::c_uint) as UChar;
                    c_tPos >>= 8 as libc::c_int;
                    c_nblock_used += 1;
                    if c_nblock_used == s_save_nblockPP {
                        continue;
                    }
                    if k1 as libc::c_int != c_k0 {
                        c_k0 = k1 as Int32;
                    } else {
                        if c_tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul(ro_blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
                        }
                        c_tPos = *c_tt.offset(c_tPos as isize);
                        k1 = (c_tPos & 0xff as libc::c_int as libc::c_uint) as UChar;
                        c_tPos >>= 8 as libc::c_int;
                        c_nblock_used += 1;
                        c_state_out_len = k1 as Int32 + 4 as libc::c_int;
                        if c_tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul(ro_blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
                        }
                        c_tPos = *c_tt.offset(c_tPos as isize);
                        c_k0 = (c_tPos & 0xff as libc::c_int as libc::c_uint) as UChar
                            as Int32;
                        c_tPos >>= 8 as libc::c_int;
                        c_nblock_used += 1;
                    }
                }
            }
        }
        total_out_lo32_old = (*(*s).strm).total_out_lo32;
        let ref mut fresh73 = (*(*s).strm).total_out_lo32;
        *fresh73 = (*fresh73).wrapping_add(avail_out_INIT.wrapping_sub(cs_avail_out));
        if (*(*s).strm).total_out_lo32 < total_out_lo32_old {
            let ref mut fresh74 = (*(*s).strm).total_out_hi32;
            *fresh74 = (*fresh74).wrapping_add(1);
        }
        (*s).calculatedBlockCRC = c_calculatedBlockCRC;
        (*s).state_out_ch = c_state_out_ch;
        (*s).state_out_len = c_state_out_len;
        (*s).nblock_used = c_nblock_used;
        (*s).k0 = c_k0;
        let ref mut fresh75 = (*s).tt;
        *fresh75 = c_tt;
        (*s).tPos = c_tPos;
        let ref mut fresh76 = (*(*s).strm).next_out;
        *fresh76 = cs_next_out;
        (*(*s).strm).avail_out = cs_avail_out;
    }
    return 0 as libc::c_int as Bool;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn BZ2_indexIntoF(
    mut indx: Int32,
    mut cftab: *mut Int32,
) -> Int32 {
    let mut nb: Int32 = 0;
    let mut na: Int32 = 0;
    let mut mid: Int32 = 0;
    nb = 0 as libc::c_int;
    na = 256 as libc::c_int;
    loop {
        mid = nb + na >> 1 as libc::c_int;
        if indx >= *cftab.offset(mid as isize) {
            nb = mid;
        } else {
            na = mid;
        }
        if !(na - nb != 1 as libc::c_int) {
            break;
        }
    }
    return nb;
}
unsafe extern "C" fn unRLE_obuf_to_output_SMALL(mut s: *mut DState) -> Bool {
    let mut k1: UChar = 0;
    if (*s).blockRandomised != 0 {
        while 1 as libc::c_int as Bool != 0 {
            while 1 as libc::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int as Bool;
                }
                if (*s).state_out_len == 0 as libc::c_int {
                    break;
                }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s)
                    .calculatedBlockCRC = (*s).calculatedBlockCRC << 8 as libc::c_int
                    ^ BZ2_crc32Table[((*s).calculatedBlockCRC >> 24 as libc::c_int
                        ^ (*s).state_out_ch as libc::c_uint) as usize];
                let ref mut fresh77 = (*s).state_out_len;
                *fresh77 -= 1;
                let ref mut fresh78 = (*(*s).strm).next_out;
                *fresh78 = (*fresh78).offset(1);
                let ref mut fresh79 = (*(*s).strm).avail_out;
                *fresh79 = (*fresh79).wrapping_sub(1);
                let ref mut fresh80 = (*(*s).strm).total_out_lo32;
                *fresh80 = (*fresh80).wrapping_add(1);
                if (*(*s).strm).total_out_lo32 == 0 as libc::c_int as libc::c_uint {
                    let ref mut fresh81 = (*(*s).strm).total_out_hi32;
                    *fresh81 = (*fresh81).wrapping_add(1);
                }
            }
            if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                return 0 as libc::c_int as Bool;
            }
            if (*s).nblock_used > (*s).save_nblock + 1 as libc::c_int {
                return 1 as libc::c_int as Bool;
            }
            (*s).state_out_len = 1 as libc::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos
                >= (100000 as libc::c_int as UInt32)
                    .wrapping_mul((*s).blockSize100k as UInt32)
            {
                return 1 as libc::c_int as Bool;
            }
            k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr()) as UChar;
            (*s)
                .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                | (*((*s).ll4).offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                    >> ((*s).tPos << 2 as libc::c_int
                        & 0x4 as libc::c_int as libc::c_uint)
                    & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
            if (*s).rNToGo == 0 as libc::c_int {
                (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                let ref mut fresh82 = (*s).rTPos;
                *fresh82 += 1;
                if (*s).rTPos == 512 as libc::c_int {
                    (*s).rTPos = 0 as libc::c_int;
                }
            }
            let ref mut fresh83 = (*s).rNToGo;
            *fresh83 -= 1;
            k1 = (k1 as libc::c_int
                ^ if (*s).rNToGo == 1 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) as UChar;
            let ref mut fresh84 = (*s).nblock_used;
            *fresh84 += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                continue;
            }
            if k1 as libc::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32;
            } else {
                (*s).state_out_len = 2 as libc::c_int;
                if (*s).tPos
                    >= (100000 as libc::c_int as UInt32)
                        .wrapping_mul((*s).blockSize100k as UInt32)
                {
                    return 1 as libc::c_int as Bool;
                }
                k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                    as UChar;
                (*s)
                    .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                    | (*((*s).ll4).offset(((*s).tPos >> 1 as libc::c_int) as isize)
                        as UInt32
                        >> ((*s).tPos << 2 as libc::c_int
                            & 0x4 as libc::c_int as libc::c_uint)
                        & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                if (*s).rNToGo == 0 as libc::c_int {
                    (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                    let ref mut fresh85 = (*s).rTPos;
                    *fresh85 += 1;
                    if (*s).rTPos == 512 as libc::c_int {
                        (*s).rTPos = 0 as libc::c_int;
                    }
                }
                let ref mut fresh86 = (*s).rNToGo;
                *fresh86 -= 1;
                k1 = (k1 as libc::c_int
                    ^ if (*s).rNToGo == 1 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as UChar;
                let ref mut fresh87 = (*s).nblock_used;
                *fresh87 += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                    continue;
                }
                if k1 as libc::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32;
                } else {
                    (*s).state_out_len = 3 as libc::c_int;
                    if (*s).tPos
                        >= (100000 as libc::c_int as UInt32)
                            .wrapping_mul((*s).blockSize100k as UInt32)
                    {
                        return 1 as libc::c_int as Bool;
                    }
                    k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                        as UChar;
                    (*s)
                        .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                        | (*((*s).ll4).offset(((*s).tPos >> 1 as libc::c_int) as isize)
                            as UInt32
                            >> ((*s).tPos << 2 as libc::c_int
                                & 0x4 as libc::c_int as libc::c_uint)
                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                    if (*s).rNToGo == 0 as libc::c_int {
                        (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                        let ref mut fresh88 = (*s).rTPos;
                        *fresh88 += 1;
                        if (*s).rTPos == 512 as libc::c_int {
                            (*s).rTPos = 0 as libc::c_int;
                        }
                    }
                    let ref mut fresh89 = (*s).rNToGo;
                    *fresh89 -= 1;
                    k1 = (k1 as libc::c_int
                        ^ if (*s).rNToGo == 1 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as UChar;
                    let ref mut fresh90 = (*s).nblock_used;
                    *fresh90 += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                        continue;
                    }
                    if k1 as libc::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32;
                    } else {
                        if (*s).tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
                        }
                        k1 = BZ2_indexIntoF(
                            (*s).tPos as Int32,
                            ((*s).cftab).as_mut_ptr(),
                        ) as UChar;
                        (*s)
                            .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                            | (*((*s).ll4)
                                .offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                                >> ((*s).tPos << 2 as libc::c_int
                                    & 0x4 as libc::c_int as libc::c_uint)
                                & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                        if (*s).rNToGo == 0 as libc::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            let ref mut fresh91 = (*s).rTPos;
                            *fresh91 += 1;
                            if (*s).rTPos == 512 as libc::c_int {
                                (*s).rTPos = 0 as libc::c_int;
                            }
                        }
                        let ref mut fresh92 = (*s).rNToGo;
                        *fresh92 -= 1;
                        k1 = (k1 as libc::c_int
                            ^ if (*s).rNToGo == 1 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) as UChar;
                        let ref mut fresh93 = (*s).nblock_used;
                        *fresh93 += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as libc::c_int;
                        if (*s).tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
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
                        if (*s).rNToGo == 0 as libc::c_int {
                            (*s).rNToGo = BZ2_rNums[(*s).rTPos as usize];
                            let ref mut fresh94 = (*s).rTPos;
                            *fresh94 += 1;
                            if (*s).rTPos == 512 as libc::c_int {
                                (*s).rTPos = 0 as libc::c_int;
                            }
                        }
                        let ref mut fresh95 = (*s).rNToGo;
                        *fresh95 -= 1;
                        let ref mut fresh96 = (*s).k0;
                        *fresh96
                            ^= if (*s).rNToGo == 1 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                        let ref mut fresh97 = (*s).nblock_used;
                        *fresh97 += 1;
                    }
                }
            }
        }
    } else {
        while 1 as libc::c_int as Bool != 0 {
            while 1 as libc::c_int as Bool != 0 {
                if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int as Bool;
                }
                if (*s).state_out_len == 0 as libc::c_int {
                    break;
                }
                *((*(*s).strm).next_out as *mut UChar) = (*s).state_out_ch;
                (*s)
                    .calculatedBlockCRC = (*s).calculatedBlockCRC << 8 as libc::c_int
                    ^ BZ2_crc32Table[((*s).calculatedBlockCRC >> 24 as libc::c_int
                        ^ (*s).state_out_ch as libc::c_uint) as usize];
                let ref mut fresh98 = (*s).state_out_len;
                *fresh98 -= 1;
                let ref mut fresh99 = (*(*s).strm).next_out;
                *fresh99 = (*fresh99).offset(1);
                let ref mut fresh100 = (*(*s).strm).avail_out;
                *fresh100 = (*fresh100).wrapping_sub(1);
                let ref mut fresh101 = (*(*s).strm).total_out_lo32;
                *fresh101 = (*fresh101).wrapping_add(1);
                if (*(*s).strm).total_out_lo32 == 0 as libc::c_int as libc::c_uint {
                    let ref mut fresh102 = (*(*s).strm).total_out_hi32;
                    *fresh102 = (*fresh102).wrapping_add(1);
                }
            }
            if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                return 0 as libc::c_int as Bool;
            }
            if (*s).nblock_used > (*s).save_nblock + 1 as libc::c_int {
                return 1 as libc::c_int as Bool;
            }
            (*s).state_out_len = 1 as libc::c_int;
            (*s).state_out_ch = (*s).k0 as UChar;
            if (*s).tPos
                >= (100000 as libc::c_int as UInt32)
                    .wrapping_mul((*s).blockSize100k as UInt32)
            {
                return 1 as libc::c_int as Bool;
            }
            k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr()) as UChar;
            (*s)
                .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                | (*((*s).ll4).offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                    >> ((*s).tPos << 2 as libc::c_int
                        & 0x4 as libc::c_int as libc::c_uint)
                    & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
            let ref mut fresh103 = (*s).nblock_used;
            *fresh103 += 1;
            if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                continue;
            }
            if k1 as libc::c_int != (*s).k0 {
                (*s).k0 = k1 as Int32;
            } else {
                (*s).state_out_len = 2 as libc::c_int;
                if (*s).tPos
                    >= (100000 as libc::c_int as UInt32)
                        .wrapping_mul((*s).blockSize100k as UInt32)
                {
                    return 1 as libc::c_int as Bool;
                }
                k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                    as UChar;
                (*s)
                    .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                    | (*((*s).ll4).offset(((*s).tPos >> 1 as libc::c_int) as isize)
                        as UInt32
                        >> ((*s).tPos << 2 as libc::c_int
                            & 0x4 as libc::c_int as libc::c_uint)
                        & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                let ref mut fresh104 = (*s).nblock_used;
                *fresh104 += 1;
                if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                    continue;
                }
                if k1 as libc::c_int != (*s).k0 {
                    (*s).k0 = k1 as Int32;
                } else {
                    (*s).state_out_len = 3 as libc::c_int;
                    if (*s).tPos
                        >= (100000 as libc::c_int as UInt32)
                            .wrapping_mul((*s).blockSize100k as UInt32)
                    {
                        return 1 as libc::c_int as Bool;
                    }
                    k1 = BZ2_indexIntoF((*s).tPos as Int32, ((*s).cftab).as_mut_ptr())
                        as UChar;
                    (*s)
                        .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                        | (*((*s).ll4).offset(((*s).tPos >> 1 as libc::c_int) as isize)
                            as UInt32
                            >> ((*s).tPos << 2 as libc::c_int
                                & 0x4 as libc::c_int as libc::c_uint)
                            & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                    let ref mut fresh105 = (*s).nblock_used;
                    *fresh105 += 1;
                    if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int {
                        continue;
                    }
                    if k1 as libc::c_int != (*s).k0 {
                        (*s).k0 = k1 as Int32;
                    } else {
                        if (*s).tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
                        }
                        k1 = BZ2_indexIntoF(
                            (*s).tPos as Int32,
                            ((*s).cftab).as_mut_ptr(),
                        ) as UChar;
                        (*s)
                            .tPos = *((*s).ll16).offset((*s).tPos as isize) as UInt32
                            | (*((*s).ll4)
                                .offset(((*s).tPos >> 1 as libc::c_int) as isize) as UInt32
                                >> ((*s).tPos << 2 as libc::c_int
                                    & 0x4 as libc::c_int as libc::c_uint)
                                & 0xf as libc::c_int as libc::c_uint) << 16 as libc::c_int;
                        let ref mut fresh106 = (*s).nblock_used;
                        *fresh106 += 1;
                        (*s).state_out_len = k1 as Int32 + 4 as libc::c_int;
                        if (*s).tPos
                            >= (100000 as libc::c_int as UInt32)
                                .wrapping_mul((*s).blockSize100k as UInt32)
                        {
                            return 1 as libc::c_int as Bool;
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
                        let ref mut fresh107 = (*s).nblock_used;
                        *fresh107 += 1;
                    }
                }
            }
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompress(mut strm: *mut bz_stream) -> libc::c_int {
    let mut corrupt: Bool = 0;
    let mut s = 0 as *mut DState;
    if strm.is_null() {
        return -(2 as libc::c_int);
    }
    s = (*strm).state as *mut DState;
    if s.is_null() {
        return -(2 as libc::c_int);
    }
    if (*s).strm != strm {
        return -(2 as libc::c_int);
    }
    while 1 as libc::c_int as Bool != 0 {
        if (*s).state == 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if (*s).state == 2 as libc::c_int {
            if (*s).smallDecompress != 0 {
                corrupt = unRLE_obuf_to_output_SMALL(s);
            } else {
                corrupt = unRLE_obuf_to_output_FAST(s);
            }
            if corrupt != 0 {
                return -(4 as libc::c_int);
            }
            if (*s).nblock_used == (*s).save_nblock + 1 as libc::c_int
                && (*s).state_out_len == 0 as libc::c_int
            {
                (*s).calculatedBlockCRC = !(*s).calculatedBlockCRC;
                if (*s).verbosity >= 3 as libc::c_int {
                    fprintf(
                        stderr,
                        b" {0x%08x, 0x%08x}\0" as *const u8 as *const libc::c_char,
                        (*s).storedBlockCRC,
                        (*s).calculatedBlockCRC,
                    );
                }
                if (*s).verbosity >= 2 as libc::c_int {
                    fprintf(stderr, b"]\0" as *const u8 as *const libc::c_char);
                }
                if (*s).calculatedBlockCRC != (*s).storedBlockCRC {
                    return -(4 as libc::c_int);
                }
                (*s)
                    .calculatedCombinedCRC = (*s).calculatedCombinedCRC
                    << 1 as libc::c_int
                    | (*s).calculatedCombinedCRC >> 31 as libc::c_int;
                let ref mut fresh108 = (*s).calculatedCombinedCRC;
                *fresh108 ^= (*s).calculatedBlockCRC;
                (*s).state = 14 as libc::c_int;
            } else {
                return 0 as libc::c_int
            }
        }
        if (*s).state >= 10 as libc::c_int {
            let mut r = BZ2_decompress(s);
            if r == 4 as libc::c_int {
                if (*s).verbosity >= 3 as libc::c_int {
                    fprintf(
                        stderr,
                        b"\n    combined CRCs: stored = 0x%08x, computed = 0x%08x\0"
                            as *const u8 as *const libc::c_char,
                        (*s).storedCombinedCRC,
                        (*s).calculatedCombinedCRC,
                    );
                }
                if (*s).calculatedCombinedCRC != (*s).storedCombinedCRC {
                    return -(4 as libc::c_int);
                }
                return r;
            }
            if (*s).state != 2 as libc::c_int {
                return r;
            }
        }
    }
    if 0 as libc::c_int == 0 {
        BZ2_bz__AssertH__fail(6001 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzDecompressEnd(mut strm: *mut bz_stream) -> libc::c_int {
    let mut s = 0 as *mut DState;
    if strm.is_null() {
        return -(2 as libc::c_int);
    }
    s = (*strm).state as *mut DState;
    if s.is_null() {
        return -(2 as libc::c_int);
    }
    if (*s).strm != strm {
        return -(2 as libc::c_int);
    }
    if !((*s).tt).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).tt as *mut libc::c_void);
    }
    if !((*s).ll16).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).ll16 as *mut libc::c_void);
    }
    if !((*s).ll4).is_null() {
        ((*strm).bzfree)
            .expect(
                "non-null function pointer",
            )((*strm).opaque, (*s).ll4 as *mut libc::c_void);
    }
    ((*strm).bzfree).expect("non-null function pointer")((*strm).opaque, (*strm).state);
    let ref mut fresh109 = (*strm).state;
    *fresh109 = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c = fgetc(f);
    if c == -(1 as libc::c_int) {
        return 1 as libc::c_int as Bool;
    }
    ungetc(c, f);
    return 0 as libc::c_int as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteOpen(
    mut bzerror: *mut libc::c_int,
    mut f: *mut FILE,
    mut blockSize100k: libc::c_int,
    mut verbosity: libc::c_int,
    mut workFactor: libc::c_int,
) -> *mut libc::c_void {
    let mut ret: Int32 = 0;
    let mut bzf = 0 as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    if f.is_null()
        || (blockSize100k < 1 as libc::c_int || blockSize100k > 9 as libc::c_int)
        || (workFactor < 0 as libc::c_int || workFactor > 250 as libc::c_int)
        || (verbosity < 0 as libc::c_int || verbosity > 4 as libc::c_int)
    {
        if !bzerror.is_null() {
            *bzerror = -(2 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(2 as libc::c_int);
        }
        return 0 as *mut libc::c_void;
    }
    if ferror(f) != 0 {
        if !bzerror.is_null() {
            *bzerror = -(6 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(6 as libc::c_int);
        }
        return 0 as *mut libc::c_void;
    }
    bzf = malloc(::std::mem::size_of::<bzFile>() as libc::c_ulong) as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = -(3 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(3 as libc::c_int);
        }
        return 0 as *mut libc::c_void;
    }
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    (*bzf).initialisedOk = 0 as libc::c_int as Bool;
    (*bzf).bufN = 0 as libc::c_int;
    let ref mut fresh110 = (*bzf).handle;
    *fresh110 = f;
    (*bzf).writing = 1 as libc::c_int as Bool;
    let ref mut fresh111 = (*bzf).strm.bzalloc;
    *fresh111 = None;
    let ref mut fresh112 = (*bzf).strm.bzfree;
    *fresh112 = None;
    let ref mut fresh113 = (*bzf).strm.opaque;
    *fresh113 = 0 as *mut libc::c_void;
    if workFactor == 0 as libc::c_int {
        workFactor = 30 as libc::c_int;
    }
    ret = BZ2_bzCompressInit(&mut (*bzf).strm, blockSize100k, verbosity, workFactor);
    if ret != 0 as libc::c_int {
        if !bzerror.is_null() {
            *bzerror = ret;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = ret;
        }
        free(bzf as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    (*bzf).strm.avail_in = 0 as libc::c_int as libc::c_uint;
    (*bzf).initialisedOk = 1 as libc::c_int as Bool;
    return bzf as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWrite(
    mut bzerror: *mut libc::c_int,
    mut b: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf = b as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    if bzf.is_null() || buf.is_null() || len < 0 as libc::c_int {
        if !bzerror.is_null() {
            *bzerror = -(2 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(2 as libc::c_int);
        }
        return;
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() {
            *bzerror = -(1 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(1 as libc::c_int);
        }
        return;
    }
    if ferror((*bzf).handle) != 0 {
        if !bzerror.is_null() {
            *bzerror = -(6 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(6 as libc::c_int);
        }
        return;
    }
    if len == 0 as libc::c_int {
        if !bzerror.is_null() {
            *bzerror = 0 as libc::c_int;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0 as libc::c_int;
        }
        return;
    }
    (*bzf).strm.avail_in = len as libc::c_uint;
    let ref mut fresh114 = (*bzf).strm.next_in;
    *fresh114 = buf as *mut libc::c_char;
    while 1 as libc::c_int as Bool != 0 {
        (*bzf).strm.avail_out = 5000 as libc::c_int as libc::c_uint;
        let ref mut fresh115 = (*bzf).strm.next_out;
        *fresh115 = ((*bzf).buf).as_mut_ptr();
        ret = BZ2_bzCompress(&mut (*bzf).strm, 0 as libc::c_int);
        if ret != 1 as libc::c_int {
            if !bzerror.is_null() {
                *bzerror = ret;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = ret;
            }
            return;
        }
        if (*bzf).strm.avail_out < 5000 as libc::c_int as libc::c_uint {
            n = (5000 as libc::c_int as libc::c_uint).wrapping_sub((*bzf).strm.avail_out)
                as Int32;
            n2 = fwrite(
                ((*bzf).buf).as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<UChar>() as libc::c_ulong,
                n as libc::c_ulong,
                (*bzf).handle,
            ) as Int32;
            if n != n2 || ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() {
                    *bzerror = -(6 as libc::c_int);
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = -(6 as libc::c_int);
                }
                return;
            }
        }
        if (*bzf).strm.avail_in == 0 as libc::c_int as libc::c_uint {
            if !bzerror.is_null() {
                *bzerror = 0 as libc::c_int;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = 0 as libc::c_int;
            }
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose(
    mut bzerror: *mut libc::c_int,
    mut b: *mut libc::c_void,
    mut abandon: libc::c_int,
    mut nbytes_in: *mut libc::c_uint,
    mut nbytes_out: *mut libc::c_uint,
) {
    BZ2_bzWriteClose64(
        bzerror,
        b,
        abandon,
        nbytes_in,
        0 as *mut libc::c_uint,
        nbytes_out,
        0 as *mut libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzWriteClose64(
    mut bzerror: *mut libc::c_int,
    mut b: *mut libc::c_void,
    mut abandon: libc::c_int,
    mut nbytes_in_lo32: *mut libc::c_uint,
    mut nbytes_in_hi32: *mut libc::c_uint,
    mut nbytes_out_lo32: *mut libc::c_uint,
    mut nbytes_out_hi32: *mut libc::c_uint,
) {
    let mut n: Int32 = 0;
    let mut n2: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = 0 as libc::c_int;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0 as libc::c_int;
        }
        return;
    }
    if (*bzf).writing == 0 {
        if !bzerror.is_null() {
            *bzerror = -(1 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(1 as libc::c_int);
        }
        return;
    }
    if ferror((*bzf).handle) != 0 {
        if !bzerror.is_null() {
            *bzerror = -(6 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(6 as libc::c_int);
        }
        return;
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = 0 as libc::c_int as libc::c_uint;
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = 0 as libc::c_int as libc::c_uint;
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = 0 as libc::c_int as libc::c_uint;
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = 0 as libc::c_int as libc::c_uint;
    }
    if abandon == 0 && (*bzf).lastErr == 0 as libc::c_int {
        while 1 as libc::c_int as Bool != 0 {
            (*bzf).strm.avail_out = 5000 as libc::c_int as libc::c_uint;
            let ref mut fresh116 = (*bzf).strm.next_out;
            *fresh116 = ((*bzf).buf).as_mut_ptr();
            ret = BZ2_bzCompress(&mut (*bzf).strm, 2 as libc::c_int);
            if ret != 3 as libc::c_int && ret != 4 as libc::c_int {
                if !bzerror.is_null() {
                    *bzerror = ret;
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = ret;
                }
                return;
            }
            if (*bzf).strm.avail_out < 5000 as libc::c_int as libc::c_uint {
                n = (5000 as libc::c_int as libc::c_uint)
                    .wrapping_sub((*bzf).strm.avail_out) as Int32;
                n2 = fwrite(
                    ((*bzf).buf).as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<UChar>() as libc::c_ulong,
                    n as libc::c_ulong,
                    (*bzf).handle,
                ) as Int32;
                if n != n2 || ferror((*bzf).handle) != 0 {
                    if !bzerror.is_null() {
                        *bzerror = -(6 as libc::c_int);
                    }
                    if !bzf.is_null() {
                        (*bzf).lastErr = -(6 as libc::c_int);
                    }
                    return;
                }
            }
            if ret == 4 as libc::c_int {
                break;
            }
        }
    }
    if abandon == 0 && ferror((*bzf).handle) == 0 {
        fflush((*bzf).handle);
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() {
                *bzerror = -(6 as libc::c_int);
            }
            if !bzf.is_null() {
                (*bzf).lastErr = -(6 as libc::c_int);
            }
            return;
        }
    }
    if !nbytes_in_lo32.is_null() {
        *nbytes_in_lo32 = (*bzf).strm.total_in_lo32;
    }
    if !nbytes_in_hi32.is_null() {
        *nbytes_in_hi32 = (*bzf).strm.total_in_hi32;
    }
    if !nbytes_out_lo32.is_null() {
        *nbytes_out_lo32 = (*bzf).strm.total_out_lo32;
    }
    if !nbytes_out_hi32.is_null() {
        *nbytes_out_hi32 = (*bzf).strm.total_out_hi32;
    }
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    BZ2_bzCompressEnd(&mut (*bzf).strm);
    free(bzf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadOpen(
    mut bzerror: *mut libc::c_int,
    mut f: *mut FILE,
    mut verbosity: libc::c_int,
    mut small: libc::c_int,
    mut unused: *mut libc::c_void,
    mut nUnused: libc::c_int,
) -> *mut libc::c_void {
    let mut bzf = 0 as *mut bzFile;
    let mut ret: libc::c_int = 0;
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    if f.is_null() || small != 0 as libc::c_int && small != 1 as libc::c_int
        || (verbosity < 0 as libc::c_int || verbosity > 4 as libc::c_int)
        || unused.is_null() && nUnused != 0 as libc::c_int
        || !unused.is_null()
            && (nUnused < 0 as libc::c_int || nUnused > 5000 as libc::c_int)
    {
        if !bzerror.is_null() {
            *bzerror = -(2 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(2 as libc::c_int);
        }
        return 0 as *mut libc::c_void;
    }
    if ferror(f) != 0 {
        if !bzerror.is_null() {
            *bzerror = -(6 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(6 as libc::c_int);
        }
        return 0 as *mut libc::c_void;
    }
    bzf = malloc(::std::mem::size_of::<bzFile>() as libc::c_ulong) as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = -(3 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(3 as libc::c_int);
        }
        return 0 as *mut libc::c_void;
    }
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    (*bzf).initialisedOk = 0 as libc::c_int as Bool;
    let ref mut fresh117 = (*bzf).handle;
    *fresh117 = f;
    (*bzf).bufN = 0 as libc::c_int;
    (*bzf).writing = 0 as libc::c_int as Bool;
    let ref mut fresh118 = (*bzf).strm.bzalloc;
    *fresh118 = None;
    let ref mut fresh119 = (*bzf).strm.bzfree;
    *fresh119 = None;
    let ref mut fresh120 = (*bzf).strm.opaque;
    *fresh120 = 0 as *mut libc::c_void;
    while nUnused > 0 as libc::c_int {
        (*bzf).buf[(*bzf).bufN as usize] = *(unused as *mut UChar) as Char;
        let ref mut fresh121 = (*bzf).bufN;
        *fresh121 += 1;
        unused = (unused as *mut UChar).offset(1 as libc::c_int as isize)
            as *mut libc::c_void;
        nUnused -= 1;
    }
    ret = BZ2_bzDecompressInit(&mut (*bzf).strm, verbosity, small);
    if ret != 0 as libc::c_int {
        if !bzerror.is_null() {
            *bzerror = ret;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = ret;
        }
        free(bzf as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    (*bzf).strm.avail_in = (*bzf).bufN as libc::c_uint;
    let ref mut fresh122 = (*bzf).strm.next_in;
    *fresh122 = ((*bzf).buf).as_mut_ptr();
    (*bzf).initialisedOk = 1 as libc::c_int as Bool;
    return bzf as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadClose(
    mut bzerror: *mut libc::c_int,
    mut b: *mut libc::c_void,
) {
    let mut bzf = b as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = 0 as libc::c_int;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0 as libc::c_int;
        }
        return;
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() {
            *bzerror = -(1 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(1 as libc::c_int);
        }
        return;
    }
    if (*bzf).initialisedOk != 0 {
        BZ2_bzDecompressEnd(&mut (*bzf).strm);
    }
    free(bzf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzRead(
    mut bzerror: *mut libc::c_int,
    mut b: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut n: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut bzf = b as *mut bzFile;
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    if bzf.is_null() || buf.is_null() || len < 0 as libc::c_int {
        if !bzerror.is_null() {
            *bzerror = -(2 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(2 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if (*bzf).writing != 0 {
        if !bzerror.is_null() {
            *bzerror = -(1 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if len == 0 as libc::c_int {
        if !bzerror.is_null() {
            *bzerror = 0 as libc::c_int;
        }
        if !bzf.is_null() {
            (*bzf).lastErr = 0 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    (*bzf).strm.avail_out = len as libc::c_uint;
    let ref mut fresh123 = (*bzf).strm.next_out;
    *fresh123 = buf as *mut libc::c_char;
    while 1 as libc::c_int as Bool != 0 {
        if ferror((*bzf).handle) != 0 {
            if !bzerror.is_null() {
                *bzerror = -(6 as libc::c_int);
            }
            if !bzf.is_null() {
                (*bzf).lastErr = -(6 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
        if (*bzf).strm.avail_in == 0 as libc::c_int as libc::c_uint
            && myfeof((*bzf).handle) == 0
        {
            n = fread(
                ((*bzf).buf).as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<UChar>() as libc::c_ulong,
                5000 as libc::c_int as libc::c_ulong,
                (*bzf).handle,
            ) as Int32;
            if ferror((*bzf).handle) != 0 {
                if !bzerror.is_null() {
                    *bzerror = -(6 as libc::c_int);
                }
                if !bzf.is_null() {
                    (*bzf).lastErr = -(6 as libc::c_int);
                }
                return 0 as libc::c_int;
            }
            (*bzf).bufN = n;
            (*bzf).strm.avail_in = (*bzf).bufN as libc::c_uint;
            let ref mut fresh124 = (*bzf).strm.next_in;
            *fresh124 = ((*bzf).buf).as_mut_ptr();
        }
        ret = BZ2_bzDecompress(&mut (*bzf).strm);
        if ret != 0 as libc::c_int && ret != 4 as libc::c_int {
            if !bzerror.is_null() {
                *bzerror = ret;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = ret;
            }
            return 0 as libc::c_int;
        }
        if ret == 0 as libc::c_int && myfeof((*bzf).handle) as libc::c_int != 0
            && (*bzf).strm.avail_in == 0 as libc::c_int as libc::c_uint
            && (*bzf).strm.avail_out > 0 as libc::c_int as libc::c_uint
        {
            if !bzerror.is_null() {
                *bzerror = -(7 as libc::c_int);
            }
            if !bzf.is_null() {
                (*bzf).lastErr = -(7 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
        if ret == 4 as libc::c_int {
            if !bzerror.is_null() {
                *bzerror = 4 as libc::c_int;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = 4 as libc::c_int;
            }
            return (len as libc::c_uint).wrapping_sub((*bzf).strm.avail_out)
                as libc::c_int;
        }
        if (*bzf).strm.avail_out == 0 as libc::c_int as libc::c_uint {
            if !bzerror.is_null() {
                *bzerror = 0 as libc::c_int;
            }
            if !bzf.is_null() {
                (*bzf).lastErr = 0 as libc::c_int;
            }
            return len;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzReadGetUnused(
    mut bzerror: *mut libc::c_int,
    mut b: *mut libc::c_void,
    mut unused: *mut *mut libc::c_void,
    mut nUnused: *mut libc::c_int,
) {
    let mut bzf = b as *mut bzFile;
    if bzf.is_null() {
        if !bzerror.is_null() {
            *bzerror = -(2 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(2 as libc::c_int);
        }
        return;
    }
    if (*bzf).lastErr != 4 as libc::c_int {
        if !bzerror.is_null() {
            *bzerror = -(1 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(1 as libc::c_int);
        }
        return;
    }
    if unused.is_null() || nUnused.is_null() {
        if !bzerror.is_null() {
            *bzerror = -(2 as libc::c_int);
        }
        if !bzf.is_null() {
            (*bzf).lastErr = -(2 as libc::c_int);
        }
        return;
    }
    if !bzerror.is_null() {
        *bzerror = 0 as libc::c_int;
    }
    if !bzf.is_null() {
        (*bzf).lastErr = 0 as libc::c_int;
    }
    *nUnused = (*bzf).strm.avail_in as libc::c_int;
    *unused = (*bzf).strm.next_in as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffCompress(
    mut dest: *mut libc::c_char,
    mut destLen: *mut libc::c_uint,
    mut source: *mut libc::c_char,
    mut sourceLen: libc::c_uint,
    mut blockSize100k: libc::c_int,
    mut verbosity: libc::c_int,
    mut workFactor: libc::c_int,
) -> libc::c_int {
    let mut strm = bz_stream {
        next_in: 0 as *mut libc::c_char,
        avail_in: 0,
        total_in_lo32: 0,
        total_in_hi32: 0,
        next_out: 0 as *mut libc::c_char,
        avail_out: 0,
        total_out_lo32: 0,
        total_out_hi32: 0,
        state: 0 as *mut libc::c_void,
        bzalloc: None,
        bzfree: None,
        opaque: 0 as *mut libc::c_void,
    };
    let mut ret: libc::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null()
        || blockSize100k < 1 as libc::c_int || blockSize100k > 9 as libc::c_int
        || verbosity < 0 as libc::c_int || verbosity > 4 as libc::c_int
        || workFactor < 0 as libc::c_int || workFactor > 250 as libc::c_int
    {
        return -(2 as libc::c_int);
    }
    if workFactor == 0 as libc::c_int {
        workFactor = 30 as libc::c_int;
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut libc::c_void;
    ret = BZ2_bzCompressInit(&mut strm, blockSize100k, verbosity, workFactor);
    if ret != 0 as libc::c_int {
        return ret;
    }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzCompress(&mut strm, 2 as libc::c_int);
    if ret == 3 as libc::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return -(8 as libc::c_int);
    } else if ret != 4 as libc::c_int {
        BZ2_bzCompressEnd(&mut strm);
        return ret;
    } else {
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzCompressEnd(&mut strm);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzBuffToBuffDecompress(
    mut dest: *mut libc::c_char,
    mut destLen: *mut libc::c_uint,
    mut source: *mut libc::c_char,
    mut sourceLen: libc::c_uint,
    mut small: libc::c_int,
    mut verbosity: libc::c_int,
) -> libc::c_int {
    let mut strm = bz_stream {
        next_in: 0 as *mut libc::c_char,
        avail_in: 0,
        total_in_lo32: 0,
        total_in_hi32: 0,
        next_out: 0 as *mut libc::c_char,
        avail_out: 0,
        total_out_lo32: 0,
        total_out_hi32: 0,
        state: 0 as *mut libc::c_void,
        bzalloc: None,
        bzfree: None,
        opaque: 0 as *mut libc::c_void,
    };
    let mut ret: libc::c_int = 0;
    if dest.is_null() || destLen.is_null() || source.is_null()
        || small != 0 as libc::c_int && small != 1 as libc::c_int
        || verbosity < 0 as libc::c_int || verbosity > 4 as libc::c_int
    {
        return -(2 as libc::c_int);
    }
    strm.bzalloc = None;
    strm.bzfree = None;
    strm.opaque = 0 as *mut libc::c_void;
    ret = BZ2_bzDecompressInit(&mut strm, verbosity, small);
    if ret != 0 as libc::c_int {
        return ret;
    }
    strm.next_in = source;
    strm.next_out = dest;
    strm.avail_in = sourceLen;
    strm.avail_out = *destLen;
    ret = BZ2_bzDecompress(&mut strm);
    if ret == 0 as libc::c_int {
        if strm.avail_out > 0 as libc::c_int as libc::c_uint {
            BZ2_bzDecompressEnd(&mut strm);
            return -(7 as libc::c_int);
        } else {
            BZ2_bzDecompressEnd(&mut strm);
            return -(8 as libc::c_int);
        }
    } else if ret != 4 as libc::c_int {
        BZ2_bzDecompressEnd(&mut strm);
        return ret;
    } else {
        *destLen = (*destLen).wrapping_sub(strm.avail_out);
        BZ2_bzDecompressEnd(&mut strm);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzlibVersion() -> *const libc::c_char {
    return b"1.0.8, 13-Jul-2019\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn bzopen_or_bzdopen(
    mut path: *const libc::c_char,
    mut fd: libc::c_int,
    mut mode: *const libc::c_char,
    mut open_mode: libc::c_int,
) -> *mut libc::c_void {
    let mut bzerr: libc::c_int = 0;
    let mut unused: [libc::c_char; 5000] = [0; 5000];
    let mut blockSize100k = 9 as libc::c_int;
    let mut writing = 0 as libc::c_int;
    let mut mode2: [libc::c_char; 10] = *::std::mem::transmute::<
        &[u8; 10],
        &mut [libc::c_char; 10],
    >(b"\0\0\0\0\0\0\0\0\0\0");
    let mut fp = 0 as *mut FILE;
    let mut bzfp = 0 as *mut libc::c_void;
    let mut verbosity = 0 as libc::c_int;
    let mut workFactor = 30 as libc::c_int;
    let mut smallMode = 0 as libc::c_int;
    let mut nUnused = 0 as libc::c_int;
    if mode.is_null() {
        return 0 as *mut libc::c_void;
    }
    while *mode != 0 {
        match *mode as libc::c_int {
            114 => {
                writing = 0 as libc::c_int;
            }
            119 => {
                writing = 1 as libc::c_int;
            }
            115 => {
                smallMode = 1 as libc::c_int;
            }
            _ => {
                if *(*__ctype_b_loc()).offset(*mode as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    blockSize100k = *mode as libc::c_int - 0x30 as libc::c_int;
                }
            }
        }
        mode = mode.offset(1);
    }
    strcat(
        mode2.as_mut_ptr(),
        if writing != 0 {
            b"w\0" as *const u8 as *const libc::c_char
        } else {
            b"r\0" as *const u8 as *const libc::c_char
        },
    );
    strcat(mode2.as_mut_ptr(), b"b\0" as *const u8 as *const libc::c_char);
    if open_mode == 0 as libc::c_int {
        if path.is_null()
            || strcmp(path, b"\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            fp = if writing != 0 { stdout } else { stdin };
        } else {
            fp = fopen(path, mode2.as_mut_ptr());
        }
    } else {
        fp = fdopen(fd, mode2.as_mut_ptr());
    }
    if fp.is_null() {
        return 0 as *mut libc::c_void;
    }
    if writing != 0 {
        if blockSize100k < 1 as libc::c_int {
            blockSize100k = 1 as libc::c_int;
        }
        if blockSize100k > 9 as libc::c_int {
            blockSize100k = 9 as libc::c_int;
        }
        bzfp = BZ2_bzWriteOpen(&mut bzerr, fp, blockSize100k, verbosity, workFactor);
    } else {
        bzfp = BZ2_bzReadOpen(
            &mut bzerr,
            fp,
            verbosity,
            smallMode,
            unused.as_mut_ptr() as *mut libc::c_void,
            nUnused,
        );
    }
    if bzfp.is_null() {
        if fp != stdin && fp != stdout {
            fclose(fp);
        }
        return 0 as *mut libc::c_void;
    }
    return bzfp;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzopen(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut libc::c_void {
    return bzopen_or_bzdopen(path, -(1 as libc::c_int), mode, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzdopen(
    mut fd: libc::c_int,
    mut mode: *const libc::c_char,
) -> *mut libc::c_void {
    return bzopen_or_bzdopen(0 as *const libc::c_char, fd, mode, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzread(
    mut b: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut bzerr: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    if (*(b as *mut bzFile)).lastErr == 4 as libc::c_int {
        return 0 as libc::c_int;
    }
    nread = BZ2_bzRead(&mut bzerr, b, buf, len);
    if bzerr == 0 as libc::c_int || bzerr == 4 as libc::c_int {
        return nread
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzwrite(
    mut b: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut bzerr: libc::c_int = 0;
    BZ2_bzWrite(&mut bzerr, b, buf, len);
    if bzerr == 0 as libc::c_int { return len } else { return -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzflush(mut b: *mut libc::c_void) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzclose(mut b: *mut libc::c_void) {
    let mut bzerr: libc::c_int = 0;
    let mut fp = 0 as *mut FILE;
    if b.is_null() {
        return;
    }
    fp = (*(b as *mut bzFile)).handle;
    if (*(b as *mut bzFile)).writing != 0 {
        BZ2_bzWriteClose(
            &mut bzerr,
            b,
            0 as libc::c_int,
            0 as *mut libc::c_uint,
            0 as *mut libc::c_uint,
        );
        if bzerr != 0 as libc::c_int {
            BZ2_bzWriteClose(
                0 as *mut libc::c_int,
                b,
                1 as libc::c_int,
                0 as *mut libc::c_uint,
                0 as *mut libc::c_uint,
            );
        }
    } else {
        BZ2_bzReadClose(&mut bzerr, b);
    }
    if fp != stdin && fp != stdout {
        fclose(fp);
    }
}
static mut bzerrorstrings: [*const libc::c_char; 16] = [
    b"OK\0" as *const u8 as *const libc::c_char,
    b"SEQUENCE_ERROR\0" as *const u8 as *const libc::c_char,
    b"PARAM_ERROR\0" as *const u8 as *const libc::c_char,
    b"MEM_ERROR\0" as *const u8 as *const libc::c_char,
    b"DATA_ERROR\0" as *const u8 as *const libc::c_char,
    b"DATA_ERROR_MAGIC\0" as *const u8 as *const libc::c_char,
    b"IO_ERROR\0" as *const u8 as *const libc::c_char,
    b"UNEXPECTED_EOF\0" as *const u8 as *const libc::c_char,
    b"OUTBUFF_FULL\0" as *const u8 as *const libc::c_char,
    b"CONFIG_ERROR\0" as *const u8 as *const libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char,
    b"???\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn BZ2_bzerror(
    mut b: *mut libc::c_void,
    mut errnum: *mut libc::c_int,
) -> *const libc::c_char {
    let mut err = (*(b as *mut bzFile)).lastErr;
    if err > 0 as libc::c_int {
        err = 0 as libc::c_int;
    }
    *errnum = err;
    return bzerrorstrings[(err * -(1 as libc::c_int)) as usize];
}
