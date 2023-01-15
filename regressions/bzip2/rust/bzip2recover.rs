
extern "C" {
    
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

struct ErasedByPreprocessor2;
impl Default for ErasedByPreprocessor2 {fn default() -> Self {Self {
}}}

#[derive(Copy, Clone)]

struct ErasedByPreprocessor3;
impl Default for ErasedByPreprocessor3 {fn default() -> Self {Self {
}}}

pub type FILE = crate::blocksort::__sFILE;
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
struct ErasedByRefactorer2;
#[repr(C)]
pub struct BitStream {
    pub handle: Option<Box<FILE>>,
    pub buffer: Int32,
    pub buffLive: Int32,
    pub mode: Char,
}
impl Default for BitStream {fn default() -> Self {Self {
handle: None,
buffer: Default::default(),
buffLive: Default::default(),
mode: Default::default(),
}}}
impl BitStream {pub fn take(&mut self) -> Self {core::mem::take(self)}}

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
unsafe extern "C" fn bsOpenReadStream(mut stream: Option<Box<FILE>>)
 -> Option<Box<BitStream>> {
    let mut bs: *mut BitStream =
        Some(Box::new(<crate::bzip2recover::BitStream as Default>::default()));
    if bs.as_deref().is_none() {();
        mallocFail(::std::mem::size_of::<BitStream>() as std::os::raw::c_ulong as
                       Int32);
    }
    (*bs.as_deref_mut().unwrap()).handle= stream;
    (*bs.as_deref_mut().unwrap()).buffer= 0 as std::os::raw::c_int;
    (*bs.as_deref_mut().unwrap()).buffLive= 0 as std::os::raw::c_int;
    (*bs.as_deref_mut().unwrap()).mode= 'r' as i32 as Char;
    return bs;
}
/*---------------------------------------------*/
unsafe extern "C" fn bsOpenWriteStream(mut stream: Option<Box<FILE>>)
 -> Option<Box<BitStream>> {
    let mut bs: *mut BitStream =
        Some(Box::new(<crate::bzip2recover::BitStream as Default>::default()));
    if bs.as_deref().is_none() {();
        mallocFail(::std::mem::size_of::<BitStream>() as std::os::raw::c_ulong as
                       Int32);
    }
    (*bs.as_deref_mut().unwrap()).handle= stream;
    (*bs.as_deref_mut().unwrap()).buffer= 0 as std::os::raw::c_int;
    (*bs.as_deref_mut().unwrap()).buffLive= 0 as std::os::raw::c_int;
    (*bs.as_deref_mut().unwrap()).mode= 'w' as i32 as Char;
    return bs;
}
/*---------------------------------------------*/
unsafe extern "C" fn bsPutBit(mut bs: Option<&mut BitStream>, mut bit: Int32) {
    if (*bs.as_deref().unwrap()).buffLive == 8 as std::os::raw::c_int {
        let mut retVal: Int32 =
            putc((*bs.as_deref().unwrap()).buffer as UChar as std::os::raw::c_int, (*bs.as_deref().unwrap()).handle.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        if retVal == -(1 as std::os::raw::c_int) { writeError(); }
        bytesOut = bytesOut.wrapping_add(1);
        (*bs.as_deref_mut().unwrap()).buffLive= 1 as std::os::raw::c_int;
        (*bs.as_deref_mut().unwrap()).buffer= bit & 0x1 as std::os::raw::c_int
    } else {
        (*bs.as_deref_mut().unwrap()).buffer=
            (*bs).buffer << 1 as std::os::raw::c_int | bit & 0x1 as std::os::raw::c_int;
        (*bs.as_deref_mut().unwrap()).buffLive+= 1
    };
}
/*---------------------------------------------*/
/*--
   Returns 0 or 1, or 2 to indicate EOF.
--*/
unsafe extern "C" fn bsGetBit(mut bs: Option<&mut BitStream>) -> Int32 {
    if (*bs.as_deref().unwrap()).buffLive > 0 as std::os::raw::c_int {
        (*bs.as_deref_mut().unwrap()).buffLive-= 1;
        return (*bs).buffer >> (*bs).buffLive & 0x1 as std::os::raw::c_int
    } else {
        let mut retVal: Int32 = getc((*bs.as_deref().unwrap()).handle.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        if retVal == -(1 as std::os::raw::c_int) {
            if *__error() != 0 as std::os::raw::c_int { readError(); }
            return 2 as std::os::raw::c_int
        }
        (*bs.as_deref_mut().unwrap()).buffLive= 7 as std::os::raw::c_int;
        (*bs.as_deref_mut().unwrap()).buffer= retVal;
        return (*bs).buffer >> 7 as std::os::raw::c_int & 0x1 as std::os::raw::c_int
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn bsClose(mut bs: Option<Box<BitStream>>) {
    let mut retVal: Int32 = 0;
    if (*bs.as_deref().unwrap()).mode as std::os::raw::c_int == 'w' as i32 {
        while (*bs.as_deref().unwrap()).buffLive < 8 as std::os::raw::c_int {
            (*bs.as_deref_mut().unwrap()).buffLive+= 1;
            (*bs.as_deref_mut().unwrap()).buffer<<= 1 as std::os::raw::c_int
        }
        retVal= putc((*bs.as_deref().unwrap()).buffer as UChar as std::os::raw::c_int, (*bs.as_deref().unwrap()).handle.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        if retVal == -(1 as std::os::raw::c_int) { writeError(); }
        bytesOut = bytesOut.wrapping_add(1);
        retVal= fflush((*bs.as_deref().unwrap()).handle.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        if retVal == -(1 as std::os::raw::c_int) { writeError(); }
    }
    retVal= fclose((*bs.as_deref().unwrap()).handle.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    if retVal == -(1 as std::os::raw::c_int) {
        if (*bs.as_deref().unwrap()).mode as std::os::raw::c_int == 'w' as i32 {
            writeError();
        } else { readError(); }
    }
    ();
}
/*---------------------------------------------*/
unsafe extern "C" fn bsPutUChar(mut bs: Option<&mut BitStream>, mut c: UChar) {
    let mut i: Int32 = 0;
    i= 7 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        bsPutBit(bs.as_deref_mut(),
                 (c as UInt32 >> i & 0x1 as std::os::raw::c_int as std::os::raw::c_uint) as
                     Int32);
        i-= 1
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn bsPutUInt32(mut bs: Option<&mut BitStream>, mut c: UInt32) {
    let mut i: Int32 = 0;
    i= 31 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        bsPutBit(bs.as_deref_mut(), (c >> i & 0x1 as std::os::raw::c_int as std::os::raw::c_uint) as Int32);
        i-= 1
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn endsInBz2(mut name: *const Char) -> Bool {
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
        match  ::std::mem::size_of::<MaybeUInt64>() as std::os::raw::c_ulong {
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
    inFile=
        fopen(inFileName.as_mut_ptr(),
              b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    if inFile.is_null() {();
        fprintf(__stderrp,
                b"%s: can\'t read `%s\'\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName.as_mut_ptr(),
                inFileName.as_mut_ptr());
        exit(1 as std::os::raw::c_int);
    }
    bsIn= bsOpenReadStream(Some(Box::from_raw(inFile)));
    fprintf(__stderrp,
            b"%s: searching for block boundaries ...\n\x00" as *const u8 as
                *const std::os::raw::c_char, progName.as_mut_ptr());
    bitsRead= 0 as std::os::raw::c_int as MaybeUInt64;
    buffLo= 0 as std::os::raw::c_int as UInt32;
    buffHi= buffLo;
    currBlock= 0 as std::os::raw::c_int;
    bStart[currBlock as usize] = 0 as std::os::raw::c_int as MaybeUInt64;
    rbCtr= 0 as std::os::raw::c_int;
    while 1 as std::os::raw::c_int as Bool != 0 {
        b= bsGetBit(bsIn.as_mut());
        bitsRead= bitsRead.wrapping_add(1);
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
            } else { currBlock-= 1 }
            break ;
        } else {
            buffHi= buffHi << 1 as std::os::raw::c_int | buffLo >> 31 as std::os::raw::c_int;
            buffLo=
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
                    rbCtr+= 1
                }
                if currBlock >= 50000 as std::os::raw::c_int {
                    tooManyBlocks(50000 as std::os::raw::c_int);
                }
                currBlock+= 1;
                bStart[currBlock as usize] = bitsRead
            }
        }
    }
    bsClose(Some(Box::from_raw(bsIn)));
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
    inFile=
        fopen(inFileName.as_mut_ptr(),
              b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    if inFile.is_null() {();
        fprintf(__stderrp,
                b"%s: can\'t open `%s\'\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName.as_mut_ptr(),
                inFileName.as_mut_ptr());
        exit(1 as std::os::raw::c_int);
    }
    bsIn= bsOpenReadStream(Some(Box::from_raw(inFile)));
    /*-- placate gcc's dataflow analyser --*/
    blockCRC= 0 as std::os::raw::c_int as UInt32;
    bsWr= 0 as *mut BitStream;
    bitsRead= 0 as std::os::raw::c_int as MaybeUInt64;
    outFile= 0 as *mut FILE;
    wrBlock= 0 as std::os::raw::c_int;
    while 1 as std::os::raw::c_int as Bool != 0 {
        b= bsGetBit(bsIn.as_mut());
        if b == 2 as std::os::raw::c_int { break ; }
        buffHi= buffHi << 1 as std::os::raw::c_int | buffLo >> 31 as std::os::raw::c_int;
        buffLo=
            buffLo << 1 as std::os::raw::c_int |
                (b & 1 as std::os::raw::c_int) as std::os::raw::c_uint;
        if bitsRead ==
               (47 as std::os::raw::c_int as
                    std::os::raw::c_ulonglong).wrapping_add(rbStart[wrBlock as usize])
           {
            blockCRC=
                buffHi << 16 as std::os::raw::c_int | buffLo >> 16 as std::os::raw::c_int
        }
        if !outFile.is_null() && bitsRead >= rbStart[wrBlock as usize] &&
               bitsRead <= rbEnd[wrBlock as usize] {
            bsPutBit(bsWr.as_mut(), b);
        }
        bitsRead= bitsRead.wrapping_add(1);
        if bitsRead ==
               rbEnd[wrBlock as
                         usize].wrapping_add(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulonglong) {
            if !outFile.is_null() {
                bsPutUChar(bsWr.as_mut(), 0x17 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x72 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x45 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x38 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x50 as std::os::raw::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x90 as std::os::raw::c_int as UChar);
                bsPutUInt32(bsWr.as_mut(), blockCRC);
                bsClose(Some(Box::from_raw(bsWr)));
                outFile= 0 as *mut FILE
            }else { (); }
            if wrBlock >= rbCtr { break ; }
            wrBlock+= 1
        } else if bitsRead == rbStart[wrBlock as usize] {
            /* Create the output file name, correctly handling leading paths. 
            (31.10.2001 by Sergey E. Kusikov) */
            let mut split: *mut Char = 0 as *mut Char;
            let mut ofs: Int32 = 0;
            let mut k: Int32 = 0;
            k= 0 as std::os::raw::c_int;
            while k < 2000 as std::os::raw::c_int {
                outFileName[k as usize] = 0 as std::os::raw::c_int as Char;
                k+= 1
            }
            strcpy(outFileName.as_mut_ptr(), inFileName.as_mut_ptr());
            split= strrchr(outFileName.as_mut_ptr(), '/' as i32);
            if split.is_null() {();
                split= outFileName.as_mut_ptr()
            } else { split= split.offset(1) }
            /* Now split points to the start of the basename. */
            ofs=
                split.offset_from(outFileName.as_mut_ptr()) as
                    std::os::raw::c_long as Int32;
            sprintf(split, b"rec%5d\x00" as *const u8 as *const std::os::raw::c_char,
                    wrBlock + 1 as std::os::raw::c_int);
            p= split;
            while (*p) as std::os::raw::c_int != 0 as std::os::raw::c_int {
                if (*p) as std::os::raw::c_int == ' ' as i32 { *p= '0' as i32 as Char }
                p= p.offset(1)
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
            outFile=
                fopen(outFileName.as_mut_ptr(),
                      b"wb\x00" as *const u8 as *const std::os::raw::c_char);
            if outFile.is_null() {();
                fprintf(__stderrp,
                        b"%s: can\'t write `%s\'\n\x00" as *const u8 as
                            *const std::os::raw::c_char, progName.as_mut_ptr(),
                        outFileName.as_mut_ptr());
                exit(1 as std::os::raw::c_int);
            }
            bsWr= bsOpenWriteStream(Some(Box::from_raw(outFile)));
            bsPutUChar(bsWr.as_mut(), 0x42 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x5a as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x68 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(),
                       (0x30 as std::os::raw::c_int + 9 as std::os::raw::c_int) as UChar);
            bsPutUChar(bsWr.as_mut(), 0x31 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x41 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x59 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x26 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x53 as std::os::raw::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x59 as std::os::raw::c_int as UChar);
        }
    }
    fprintf(__stderrp,
            b"%s: finished\n\x00" as *const u8 as *const std::os::raw::c_char,
            progName.as_mut_ptr());
    return 0 as std::os::raw::c_int;
}
// #[main]
// pub fn main() {
//     let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
//     };
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(main_0((args.len() - 1) as Int32,
//                                     args.as_mut_ptr() as *mut *mut Char) as
//                                  i32)
//     }
// }
/*-----------------------------------------------------------*/
/*--- end                                  bzip2recover.c ---*/
/*-----------------------------------------------------------*/
