
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fflush(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn getc(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn perror(_: *const std::os::raw::c_char);
    #[no_mangle]
    fn putc(_: std::os::raw::c_int, _: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn sprintf(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: ...)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strncpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strrchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
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
/*-----------------------------------------------------------*/
/*--- Block recoverer program for bzip2                   ---*/
/*---                                      bzip2recover.c ---*/
/*-----------------------------------------------------------*/
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
/* This program is a complete hack and should be rewritten properly.
	 It isn't very complicated. */
/* This program records bit locations in the file to be recovered.
   That means that if 64-bit ints are not supported, we will not
   be able to recover .bz2 files over 512MB (2^32 bits) long.
   On GNU supported platforms, we take advantage of the 64-bit
   int support to circumvent this problem.  Ditto MSVC.

   This change occurred in version 1.0.2; all prior versions have
   the 512MB limitation.
*/
pub type MaybeUInt64 = std::os::raw::c_ulonglong;
pub type UInt32 = std::os::raw::c_uint;
pub type Int32 = std::os::raw::c_int;
pub type UChar = std::os::raw::c_uchar;
pub type Char = std::os::raw::c_char;
pub type Bool = std::os::raw::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub handle: *mut FILE,
    pub buffer: Int32,
    pub buffLive: Int32,
    pub mode: Char,
}
#[no_mangle]
pub static mut inFileName: [Char; 2000] = [0; 2000];
#[no_mangle]
pub static mut outFileName: [Char; 2000] = [0; 2000];
pub static mut progName: [Char; 2000] = [0; 2000];
#[no_mangle]
pub static mut bytesOut: MaybeUInt64 = 0 as std::os::raw::c_int as MaybeUInt64;
#[no_mangle]
pub static mut bytesIn: MaybeUInt64 = 0 as std::os::raw::c_int as MaybeUInt64;
/* '0' */
/*---------------------------------------------------*/
/*--- I/O errors                                  ---*/
/*---------------------------------------------------*/
/*---------------------------------------------*/
unsafe extern "C" fn readError() {
    fprintf(__stderrp,
            b"%s: I/O error reading `%s\', possible reason follows.\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr(),
            inFileName.as_mut_ptr());
    perror(progName.as_mut_ptr());
    fprintf(__stderrp,
            b"%s: warning: output file(s) may be incomplete.\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr());
    exit(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn writeError() {
    fprintf(__stderrp,
            b"%s: I/O error reading `%s\', possible reason follows.\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr(),
            inFileName.as_mut_ptr());
    perror(progName.as_mut_ptr());
    fprintf(__stderrp,
            b"%s: warning: output file(s) may be incomplete.\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr());
    exit(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn mallocFail(mut n: Int32) {
    fprintf(__stderrp,
            b"%s: malloc failed on request for %d bytes.\n\x00" as *const u8
                as *const std::os::raw::c_char, progName.as_mut_ptr(), n);
    fprintf(__stderrp,
            b"%s: warning: output file(s) may be incomplete.\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr());
    exit(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn tooManyBlocks(mut max_handled_blocks: Int32) {
    fprintf(__stderrp,
            b"%s: `%s\' appears to contain more than %d blocks\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr(),
            inFileName.as_mut_ptr(), max_handled_blocks);
    fprintf(__stderrp,
            b"%s: and cannot be handled.  To fix, increase\n\x00" as *const u8
                as *const std::os::raw::c_char, progName.as_mut_ptr());
    fprintf(__stderrp,
            b"%s: BZ_MAX_HANDLED_BLOCKS in bzip2recover.c, and recompile.\n\x00"
                as *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr());
    exit(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn bsOpenReadStream(mut stream: *mut FILE)
 -> *mut BitStream {
    let mut bs: *mut BitStream =
        malloc(::std::mem::size_of::<BitStream>() as std::os::raw::c_ulong) as
            *mut BitStream;
    if bs.is_null() {
        mallocFail(::std::mem::size_of::<BitStream>() as std::os::raw::c_ulong as
                       Int32);
    }
    (*bs).handle = stream;
    (*bs).buffer = 0 as std::os::raw::c_int;
    (*bs).buffLive = 0 as std::os::raw::c_int;
    (*bs).mode = 'r' as i32 as Char;
    return bs;
}
/*---------------------------------------------*/
unsafe extern "C" fn bsOpenWriteStream(mut stream: *mut FILE)
 -> *mut BitStream {
    let mut bs: *mut BitStream =
        malloc(::std::mem::size_of::<BitStream>() as std::os::raw::c_ulong) as
            *mut BitStream;
    if bs.is_null() {
        mallocFail(::std::mem::size_of::<BitStream>() as std::os::raw::c_ulong as
                       Int32);
    }
    (*bs).handle = stream;
    (*bs).buffer = 0 as std::os::raw::c_int;
    (*bs).buffLive = 0 as std::os::raw::c_int;
    (*bs).mode = 'w' as i32 as Char;
    return bs;
}
/*---------------------------------------------*/
unsafe extern "C" fn bsPutBit(mut bs: *mut BitStream, mut bit: Int32) {
    if (*bs).buffLive == 8 as std::os::raw::c_int {
        let mut retVal: Int32 =
            putc((*bs).buffer as UChar as std::os::raw::c_int, (*bs).handle);
        if retVal == -(1 as std::os::raw::c_int) { writeError(); }
        bytesOut = bytesOut.wrapping_add(1);
        (*bs).buffLive = 1 as std::os::raw::c_int;
        (*bs).buffer = bit & 0x1 as std::os::raw::c_int
    } else {
        (*bs).buffer =
            (*bs).buffer << 1 as std::os::raw::c_int | bit & 0x1 as std::os::raw::c_int;
        (*bs).buffLive += 1
    };
}
/*---------------------------------------------*/
/*--
   Returns 0 or 1, or 2 to indicate EOF.
--*/
unsafe extern "C" fn bsGetBit(mut bs: *mut BitStream) -> Int32 {
    if (*bs).buffLive > 0 as std::os::raw::c_int {
        (*bs).buffLive -= 1;
        return (*bs).buffer >> (*bs).buffLive & 0x1 as std::os::raw::c_int
    } else {
        let mut retVal: Int32 = getc((*bs).handle);
        if retVal == -(1 as std::os::raw::c_int) {
            if *__error() != 0 as std::os::raw::c_int { readError(); }
            return 2 as std::os::raw::c_int
        }
        (*bs).buffLive = 7 as std::os::raw::c_int;
        (*bs).buffer = retVal;
        return (*bs).buffer >> 7 as std::os::raw::c_int & 0x1 as std::os::raw::c_int
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn bsClose(mut bs: *mut BitStream) {
    let mut retVal: Int32 = 0;
    if (*bs).mode as std::os::raw::c_int == 'w' as i32 {
        while (*bs).buffLive < 8 as std::os::raw::c_int {
            (*bs).buffLive += 1;
            (*bs).buffer <<= 1 as std::os::raw::c_int
        }
        retVal = putc((*bs).buffer as UChar as std::os::raw::c_int, (*bs).handle);
        if retVal == -(1 as std::os::raw::c_int) { writeError(); }
        bytesOut = bytesOut.wrapping_add(1);
        retVal = fflush((*bs).handle);
        if retVal == -(1 as std::os::raw::c_int) { writeError(); }
    }
    retVal = fclose((*bs).handle);
    if retVal == -(1 as std::os::raw::c_int) {
        if (*bs).mode as std::os::raw::c_int == 'w' as i32 {
            writeError();
        } else { readError(); }
    }
    free(bs as *mut std::os::raw::c_void);
}
/*---------------------------------------------*/
unsafe extern "C" fn bsPutUChar(mut bs: *mut BitStream, mut c: UChar) {
    let mut i: Int32 = 0;
    i = 7 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        bsPutBit(bs,
                 (c as UInt32 >> i & 0x1 as std::os::raw::c_int as std::os::raw::c_uint) as
                     Int32);
        i -= 1
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn bsPutUInt32(mut bs: *mut BitStream, mut c: UInt32) {
    let mut i: Int32 = 0;
    i = 31 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        bsPutBit(bs, (c >> i & 0x1 as std::os::raw::c_int as std::os::raw::c_uint) as Int32);
        i -= 1
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn endsInBz2(mut name: *mut Char) -> Bool {
    let mut n: Int32 = strlen(name) as Int32;
    if n <= 4 as std::os::raw::c_int { return 0 as std::os::raw::c_int as Bool }
    return (*name.offset((n - 4 as std::os::raw::c_int) as isize) as std::os::raw::c_int ==
                '.' as i32 &&
                *name.offset((n - 3 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                    == 'b' as i32 &&
                *name.offset((n - 2 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                    == 'z' as i32 &&
                *name.offset((n - 1 as std::os::raw::c_int) as isize) as std::os::raw::c_int
                    == '2' as i32) as std::os::raw::c_int as Bool;
}
#[no_mangle]
pub static mut bStart: [MaybeUInt64; 50000] = [0; 50000];
#[no_mangle]
pub static mut bEnd: [MaybeUInt64; 50000] = [0; 50000];
#[no_mangle]
pub static mut rbStart: [MaybeUInt64; 50000] = [0; 50000];
#[no_mangle]
pub static mut rbEnd: [MaybeUInt64; 50000] = [0; 50000];
unsafe fn main_0(mut argc: Int32, mut argv: *mut *mut Char) -> Int32 {
    let mut inFile: *mut FILE = 0 as *mut FILE;
    let mut outFile: *mut FILE = 0 as *mut FILE;
    let mut bsIn: *mut BitStream = 0 as *mut BitStream;
    let mut bsWr: *mut BitStream = 0 as *mut BitStream;
    let mut b: Int32 = 0;
    let mut wrBlock: Int32 = 0;
    let mut currBlock: Int32 = 0;
    let mut rbCtr: Int32 = 0;
    let mut bitsRead: MaybeUInt64 = 0;
    let mut buffHi: UInt32 = 0;
    let mut buffLo: UInt32 = 0;
    let mut blockCRC: UInt32 = 0;
    let mut p: *mut Char = 0 as *mut Char;
    strncpy(progName.as_mut_ptr(), *argv.offset(0 as std::os::raw::c_int as isize),
            (2000 as std::os::raw::c_int - 1 as std::os::raw::c_int) as std::os::raw::c_ulong);
    progName[(2000 as std::os::raw::c_int - 1 as std::os::raw::c_int) as usize] =
        '\u{0}' as i32 as Char;
    outFileName[0 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as Char;
    inFileName[0 as std::os::raw::c_int as usize] =
        outFileName[0 as std::os::raw::c_int as usize];
    fprintf(__stderrp,
            b"bzip2recover 1.0.8: extracts blocks from damaged .bz2 files.\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    if argc != 2 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"%s: usage is `%s damaged_file_name\'.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName.as_mut_ptr(),
                progName.as_mut_ptr());
        match ::std::mem::size_of::<MaybeUInt64>() as std::os::raw::c_ulong {
            8 => {
                fprintf(__stderrp,
                        b"\trestrictions on size of recovered file: None\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
            }
            4 => {
                fprintf(__stderrp,
                        b"\trestrictions on size of recovered file: 512 MB\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
                fprintf(__stderrp,
                        b"\tto circumvent, recompile with MaybeUInt64 as an\n\tunsigned 64-bit int.\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
            }
            _ => {
                fprintf(__stderrp,
                        b"\tsizeof(MaybeUInt64) is not 4 or 8 -- configuration error.\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
            }
        }
        exit(1 as std::os::raw::c_int);
    }
    if strlen(*argv.offset(1 as std::os::raw::c_int as isize)) >=
           (2000 as std::os::raw::c_int - 20 as std::os::raw::c_int) as std::os::raw::c_ulong {
        fprintf(__stderrp,
                b"%s: supplied filename is suspiciously (>= %d chars) long.  Bye!\n\x00"
                    as *const u8 as *const std::os::raw::c_char,
                progName.as_mut_ptr(),
                strlen(*argv.offset(1 as std::os::raw::c_int as isize)) as
                    std::os::raw::c_int);
        exit(1 as std::os::raw::c_int);
    }
    strcpy(inFileName.as_mut_ptr(), *argv.offset(1 as std::os::raw::c_int as isize));
    inFile =
        fopen(inFileName.as_mut_ptr(),
              b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    if inFile.is_null() {
        fprintf(__stderrp,
                b"%s: can\'t read `%s\'\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName.as_mut_ptr(),
                inFileName.as_mut_ptr());
        exit(1 as std::os::raw::c_int);
    }
    bsIn = bsOpenReadStream(inFile);
    fprintf(__stderrp,
            b"%s: searching for block boundaries ...\n\x00" as *const u8 as
                *const std::os::raw::c_char, progName.as_mut_ptr());
    bitsRead = 0 as std::os::raw::c_int as MaybeUInt64;
    buffLo = 0 as std::os::raw::c_int as UInt32;
    buffHi = buffLo;
    currBlock = 0 as std::os::raw::c_int;
    bStart[currBlock as usize] = 0 as std::os::raw::c_int as MaybeUInt64;
    rbCtr = 0 as std::os::raw::c_int;
    while 1 as std::os::raw::c_int as Bool != 0 {
        b = bsGetBit(bsIn);
        bitsRead = bitsRead.wrapping_add(1);
        if b == 2 as std::os::raw::c_int {
            if bitsRead >= bStart[currBlock as usize] &&
                   bitsRead.wrapping_sub(bStart[currBlock as usize]) >=
                       40 as std::os::raw::c_int as std::os::raw::c_ulonglong {
                bEnd[currBlock as usize] =
                    bitsRead.wrapping_sub(1 as std::os::raw::c_int as
                                              std::os::raw::c_ulonglong);
                if currBlock > 0 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"   block %d runs from %Lu to %Lu (incomplete)\n\x00"
                                as *const u8 as *const std::os::raw::c_char,
                            currBlock, bStart[currBlock as usize],
                            bEnd[currBlock as usize]);
                }
            } else { currBlock -= 1 }
            break ;
        } else {
            buffHi = buffHi << 1 as std::os::raw::c_int | buffLo >> 31 as std::os::raw::c_int;
            buffLo =
                buffLo << 1 as std::os::raw::c_int |
                    (b & 1 as std::os::raw::c_int) as std::os::raw::c_uint;
            if (buffHi & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                   std::os::raw::c_ulong == 0x3141 as std::os::raw::c_ulong &&
                   buffLo as std::os::raw::c_ulong == 0x59265359 as std::os::raw::c_ulong ||
                   (buffHi & 0xffff as std::os::raw::c_int as std::os::raw::c_uint) as
                       std::os::raw::c_ulong == 0x1772 as std::os::raw::c_ulong &&
                       buffLo as std::os::raw::c_ulong == 0x45385090 as std::os::raw::c_ulong
               {
                if bitsRead > 49 as std::os::raw::c_int as std::os::raw::c_ulonglong {
                    bEnd[currBlock as usize] =
                        bitsRead.wrapping_sub(49 as std::os::raw::c_int as
                                                  std::os::raw::c_ulonglong)
                } else {
                    bEnd[currBlock as usize] = 0 as std::os::raw::c_int as MaybeUInt64
                }
                if currBlock > 0 as std::os::raw::c_int &&
                       bEnd[currBlock as
                                usize].wrapping_sub(bStart[currBlock as
                                                               usize]) >=
                           130 as std::os::raw::c_int as std::os::raw::c_ulonglong {
                    fprintf(__stderrp,
                            b"   block %d runs from %Lu to %Lu\n\x00" as
                                *const u8 as *const std::os::raw::c_char,
                            rbCtr + 1 as std::os::raw::c_int,
                            bStart[currBlock as usize],
                            bEnd[currBlock as usize]);
                    rbStart[rbCtr as usize] = bStart[currBlock as usize];
                    rbEnd[rbCtr as usize] = bEnd[currBlock as usize];
                    rbCtr += 1
                }
                if currBlock >= 50000 as std::os::raw::c_int {
                    tooManyBlocks(50000 as std::os::raw::c_int);
                }
                currBlock += 1;
                bStart[currBlock as usize] = bitsRead
            }
        }
    }
    bsClose(bsIn);
    /*-- identified blocks run from 1 to rbCtr inclusive. --*/
    if rbCtr < 1 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"%s: sorry, I couldn\'t find any block boundaries.\n\x00" as
                    *const u8 as *const std::os::raw::c_char, progName.as_mut_ptr());
        exit(1 as std::os::raw::c_int);
    }
    fprintf(__stderrp,
            b"%s: splitting into blocks\n\x00" as *const u8 as
                *const std::os::raw::c_char, progName.as_mut_ptr());
    inFile =
        fopen(inFileName.as_mut_ptr(),
              b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    if inFile.is_null() {
        fprintf(__stderrp,
                b"%s: can\'t open `%s\'\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName.as_mut_ptr(),
                inFileName.as_mut_ptr());
        exit(1 as std::os::raw::c_int);
    }
    bsIn = bsOpenReadStream(inFile);
    /*-- placate gcc's dataflow analyser --*/
    blockCRC = 0 as std::os::raw::c_int as UInt32;
    bsWr = 0 as *mut BitStream;
    bitsRead = 0 as std::os::raw::c_int as MaybeUInt64;
    outFile = 0 as *mut FILE;
    wrBlock = 0 as std::os::raw::c_int;
    while 1 as std::os::raw::c_int as Bool != 0 {
        b = bsGetBit(bsIn);
        if b == 2 as std::os::raw::c_int { break ; }
        buffHi = buffHi << 1 as std::os::raw::c_int | buffLo >> 31 as std::os::raw::c_int;
        buffLo =
            buffLo << 1 as std::os::raw::c_int |
                (b & 1 as std::os::raw::c_int) as std::os::raw::c_uint;
        if bitsRead ==
               (47 as std::os::raw::c_int as
                    std::os::raw::c_ulonglong).wrapping_add(rbStart[wrBlock as usize])
           {
            blockCRC =
                buffHi << 16 as std::os::raw::c_int | buffLo >> 16 as std::os::raw::c_int
        }
        if !outFile.is_null() && bitsRead >= rbStart[wrBlock as usize] &&
               bitsRead <= rbEnd[wrBlock as usize] {
            bsPutBit(bsWr, b);
        }
        bitsRead = bitsRead.wrapping_add(1);
        if bitsRead ==
               rbEnd[wrBlock as
                         usize].wrapping_add(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) {
            if !outFile.is_null() {
                bsPutUChar(bsWr, 0x17 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr, 0x72 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr, 0x45 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr, 0x38 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr, 0x50 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr, 0x90 as std::os::raw::c_int as UChar);
                bsPutUInt32(bsWr, blockCRC);
                bsClose(bsWr);
                outFile = 0 as *mut FILE
            }
            if wrBlock >= rbCtr { break ; }
            wrBlock += 1
        } else if bitsRead == rbStart[wrBlock as usize] {
            /* Create the output file name, correctly handling leading paths. 
            (31.10.2001 by Sergey E. Kusikov) */
            let mut split: *mut Char = 0 as *mut Char;
            let mut ofs: Int32 = 0;
            let mut k: Int32 = 0;
            k = 0 as std::os::raw::c_int;
            while k < 2000 as std::os::raw::c_int {
                outFileName[k as usize] = 0 as std::os::raw::c_int as Char;
                k += 1
            }
            strcpy(outFileName.as_mut_ptr(), inFileName.as_mut_ptr());
            split = strrchr(outFileName.as_mut_ptr(), '/' as i32);
            if split.is_null() {
                split = outFileName.as_mut_ptr()
            } else { split = split.offset(1) }
            /* Now split points to the start of the basename. */
            ofs =
                split.offset_from(outFileName.as_mut_ptr()) as
                    std::os::raw::c_long as Int32;
            sprintf(split, b"rec%5d\x00" as *const u8 as *const std::os::raw::c_char,
                    wrBlock + 1 as std::os::raw::c_int);
            p = split;
            while *p as std::os::raw::c_int != 0 as std::os::raw::c_int {
                if *p as std::os::raw::c_int == ' ' as i32 { *p = '0' as i32 as Char }
                p = p.offset(1)
            }
            strcat(outFileName.as_mut_ptr(),
                   inFileName.as_mut_ptr().offset(ofs as isize));
            if endsInBz2(outFileName.as_mut_ptr()) == 0 {
                strcat(outFileName.as_mut_ptr(),
                       b".bz2\x00" as *const u8 as *const std::os::raw::c_char);
            }
            fprintf(__stderrp,
                    b"   writing block %d to `%s\' ...\n\x00" as *const u8 as
                        *const std::os::raw::c_char, wrBlock + 1 as std::os::raw::c_int,
                    outFileName.as_mut_ptr());
            outFile =
                fopen(outFileName.as_mut_ptr(),
                      b"wb\x00" as *const u8 as *const std::os::raw::c_char);
            if outFile.is_null() {
                fprintf(__stderrp,
                        b"%s: can\'t write `%s\'\n\x00" as *const u8 as
                            *const std::os::raw::c_char, progName.as_mut_ptr(),
                        outFileName.as_mut_ptr());
                exit(1 as std::os::raw::c_int);
            }
            bsWr = bsOpenWriteStream(outFile);
            bsPutUChar(bsWr, 0x42 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr, 0x5a as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr, 0x68 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr,
                       (0x30 as std::os::raw::c_int + 9 as std::os::raw::c_int) as UChar);
            bsPutUChar(bsWr, 0x31 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr, 0x41 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr, 0x59 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr, 0x26 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr, 0x53 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr, 0x59 as std::os::raw::c_int as UChar);
        }
    }
    fprintf(__stderrp,
            b"%s: finished\n\x00" as *const u8 as *const std::os::raw::c_char,
            progName.as_mut_ptr());
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as Int32,
                                    args.as_mut_ptr() as *mut *mut Char) as
                                 i32)
    }
}
/*-----------------------------------------------------------*/
/*--- end                                  bzip2recover.c ---*/
/*-----------------------------------------------------------*/
