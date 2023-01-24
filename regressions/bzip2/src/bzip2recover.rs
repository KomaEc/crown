use ::libc;
extern "C" {
    
    
    
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1 { dummy: () }
pub type _IO_lock_t = ();
pub type FILE = crate::src::blocksort::_IO_FILE;
pub type MaybeUInt64 = libc::c_ulonglong;
pub type UInt32 = libc::c_uint;
pub type Int32 = libc::c_int;
pub type UChar = libc::c_uchar;
pub type Char = libc::c_char;
pub type Bool = libc::c_uchar;
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
#[no_mangle]
pub static mut progName: [Char; 2000] = [0; 2000];
#[no_mangle]
pub static mut bytesOut: MaybeUInt64 = 0 as libc::c_int as MaybeUInt64;
#[no_mangle]
pub static mut bytesIn: MaybeUInt64 = 0 as libc::c_int as MaybeUInt64;
unsafe extern "C" fn readError() {
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: I/O error reading `%s', possible reason follows.\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
        crate::src::bzip2recover::inFileName.as_mut_ptr(),
    );
    perror(crate::src::bzip2recover::progName.as_mut_ptr());
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn writeError() {
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: I/O error reading `%s', possible reason follows.\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
        crate::src::bzip2recover::inFileName.as_mut_ptr(),
    );
    perror(crate::src::bzip2recover::progName.as_mut_ptr());
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn mallocFail(mut n: Int32) {
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: malloc failed on request for %d bytes.\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
        n,
    );
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: warning: output file(s) may be incomplete.\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn tooManyBlocks(mut max_handled_blocks: Int32) {
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: `%s' appears to contain more than %d blocks\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
        crate::src::bzip2recover::inFileName.as_mut_ptr(),
        max_handled_blocks,
    );
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: and cannot be handled.  To fix, increase\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: BZ_MAX_HANDLED_BLOCKS in bzip2recover.c, and recompile.\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn bsOpenReadStream(mut stream: *mut FILE) -> *mut /* owning */ BitStream {
    let mut bs = malloc(::std::mem::size_of::<BitStream>() as libc::c_ulong)
        as *mut BitStream;
    if bs.is_null() {();
        mallocFail(::std::mem::size_of::<BitStream>() as libc::c_ulong as Int32);
    }
    (*bs).handle= stream;
    (*bs).buffer= 0 as libc::c_int;
    (*bs).buffLive= 0 as libc::c_int;
    (*bs).mode= 'r' as i32 as Char;
    return bs;
}
unsafe extern "C" fn bsOpenWriteStream(mut stream: *mut FILE) -> *mut /* owning */ BitStream {
    let mut bs = malloc(::std::mem::size_of::<BitStream>() as libc::c_ulong)
        as *mut BitStream;
    if bs.is_null() {();
        mallocFail(::std::mem::size_of::<BitStream>() as libc::c_ulong as Int32);
    }
    (*bs).handle= stream;
    (*bs).buffer= 0 as libc::c_int;
    (*bs).buffLive= 0 as libc::c_int;
    (*bs).mode= 'w' as i32 as Char;
    return bs;
}
unsafe extern "C" fn bsPutBit(mut bs: Option<&mut BitStream>, mut bit: Int32) {
    if (*bs.as_deref().unwrap()).buffLive == 8 as libc::c_int {
        let mut retVal = putc((*bs.as_deref().unwrap()).buffer as UChar as libc::c_int, (*bs.as_deref().unwrap()).handle);
        if retVal == -(1 as libc::c_int) {
            writeError();
        }
        crate::src::bzip2recover::bytesOut= crate::src::bzip2recover::bytesOut.wrapping_add(1);
        (*bs.as_deref_mut().unwrap()).buffLive= 1 as libc::c_int;
        (*bs.as_deref_mut().unwrap()).buffer= bit & 0x1 as libc::c_int;
    } else {
        (*bs.as_deref_mut().unwrap()).buffer= (*bs.as_deref().unwrap()).buffer << 1 as libc::c_int | bit & 0x1 as libc::c_int;
        (*bs.as_deref_mut().unwrap()).buffLive+= 1;
    };
}
unsafe extern "C" fn bsGetBit(mut bs: Option<&mut BitStream>) -> Int32 {
    if (*bs.as_deref().unwrap()).buffLive > 0 as libc::c_int {
        (*bs.as_deref_mut().unwrap()).buffLive-= 1;
        return (*bs.as_deref().unwrap()).buffer >> (*bs.as_deref().unwrap()).buffLive & 0x1 as libc::c_int;
    } else {
        let mut retVal = getc((*bs.as_deref().unwrap()).handle);
        if retVal == -(1 as libc::c_int) {
            if *__errno_location() != 0 as libc::c_int {
                readError();
            }
            return 2 as libc::c_int;
        }
        (*bs.as_deref_mut().unwrap()).buffLive= 7 as libc::c_int;
        (*bs.as_deref_mut().unwrap()).buffer= retVal;
        return (*bs.as_deref().unwrap()).buffer >> 7 as libc::c_int & 0x1 as libc::c_int;
    };
}
unsafe extern "C" fn bsClose(mut bs: *mut /* owning */ BitStream) {
    let mut retVal: Int32 = 0;
    if (*bs).mode as libc::c_int == 'w' as i32 {
        while (*bs).buffLive < 8 as libc::c_int {
            (*bs).buffLive+= 1;
            (*bs).buffer<<= 1 as libc::c_int;
        }
        retVal= putc((*bs).buffer as UChar as libc::c_int, (*bs).handle);
        if retVal == -(1 as libc::c_int) {
            writeError();
        }
        crate::src::bzip2recover::bytesOut= crate::src::bzip2recover::bytesOut.wrapping_add(1);
        retVal= fflush((*bs).handle);
        if retVal == -(1 as libc::c_int) {
            writeError();
        }
    }
    retVal= fclose((*bs).handle);
    if retVal == -(1 as libc::c_int) {
        if (*bs).mode as libc::c_int == 'w' as i32 {
            writeError();
        } else {
            readError();
        }
    }
    free(bs as *mut libc::c_void);
}
unsafe extern "C" fn bsPutUChar(mut bs: Option<&mut BitStream>, mut c: UChar) {
    let mut i: Int32 = 0;
    i= 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        bsPutBit(bs.as_deref_mut(), (c as UInt32 >> i & 0x1 as libc::c_int as libc::c_uint) as Int32);
        i-= 1;
    }
}
unsafe extern "C" fn bsPutUInt32(mut bs: Option<&mut BitStream>, mut c: UInt32) {
    let mut i: Int32 = 0;
    i= 31 as libc::c_int;
    while i >= 0 as libc::c_int {
        bsPutBit(bs.as_deref_mut(), (c >> i & 0x1 as libc::c_int as libc::c_uint) as Int32);
        i-= 1;
    }
}
unsafe extern "C" fn endsInBz2(mut name: *mut Char) -> Bool {
    let mut n = strlen(name) as Int32;
    if n <= 4 as libc::c_int {
        return 0 as libc::c_int as Bool;
    }
    return (*name.offset((n - 4 as libc::c_int) as isize) as libc::c_int == '.' as i32
        && *name.offset((n - 3 as libc::c_int) as isize) as libc::c_int == 'b' as i32
        && *name.offset((n - 2 as libc::c_int) as isize) as libc::c_int == 'z' as i32
        && *name.offset((n - 1 as libc::c_int) as isize) as libc::c_int == '2' as i32)
        as libc::c_int as Bool;
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
    let mut inFile = 0 as *mut FILE;
    let mut outFile = 0 as *mut FILE;
    let mut bsIn = 0 as *mut BitStream;
    let mut bsWr = 0 as *mut BitStream;
    let mut b: Int32 = 0;
    let mut wrBlock: Int32 = 0;
    let mut currBlock: Int32 = 0;
    let mut rbCtr: Int32 = 0;
    let mut bitsRead: MaybeUInt64 = 0;
    let mut buffHi: UInt32 = 0;
    let mut buffLo: UInt32 = 0;
    let mut blockCRC: UInt32 = 0;
    let mut p = 0 as *mut Char;
    strncpy(
        crate::src::bzip2recover::progName.as_mut_ptr(),
        *argv.offset(0 as libc::c_int as isize),
        (2000 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    crate::src::bzip2recover::progName[(2000 as libc::c_int - 1 as libc::c_int) as usize]= '\0' as i32 as Char;
    crate::src::bzip2recover::outFileName[0 as libc::c_int as usize]= 0 as libc::c_int as Char;
    crate::src::bzip2recover::inFileName[0 as libc::c_int as usize]= crate::src::bzip2recover::outFileName[0 as libc::c_int as usize];
    fprintf(
        crate::src::bzip2recover::stderr,
        b"bzip2recover 1.0.8: extracts blocks from damaged .bz2 files.\n\0" as *const u8
            as *const libc::c_char,
    );
    if argc != 2 as libc::c_int {
        fprintf(
            crate::src::bzip2recover::stderr,
            b"%s: usage is `%s damaged_file_name'.\n\0" as *const u8
                as *const libc::c_char,
            crate::src::bzip2recover::progName.as_mut_ptr(),
            crate::src::bzip2recover::progName.as_mut_ptr(),
        );
        match  ::std::mem::size_of::<MaybeUInt64>() as libc::c_ulong {
            8 => {
                fprintf(
                    crate::src::bzip2recover::stderr,
                    b"\trestrictions on size of recovered file: None\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            4 => {
                fprintf(
                    crate::src::bzip2recover::stderr,
                    b"\trestrictions on size of recovered file: 512 MB\n\0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    crate::src::bzip2recover::stderr,
                    b"\tto circumvent, recompile with MaybeUInt64 as an\n\tunsigned 64-bit int.\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                fprintf(
                    crate::src::bzip2recover::stderr,
                    b"\tsizeof(MaybeUInt64) is not 4 or 8 -- configuration error.\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        exit(1 as libc::c_int);
    }
    if strlen(*argv.offset(1 as libc::c_int as isize))
        >= (2000 as libc::c_int - 20 as libc::c_int) as libc::c_ulong
    {
        fprintf(
            crate::src::bzip2recover::stderr,
            b"%s: supplied filename is suspiciously (>= %d chars) long.  Bye!\n\0"
                as *const u8 as *const libc::c_char,
            crate::src::bzip2recover::progName.as_mut_ptr(),
            strlen(*argv.offset(1 as libc::c_int as isize)) as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    strcpy(crate::src::bzip2recover::inFileName.as_mut_ptr(), *argv.offset(1 as libc::c_int as isize));
    inFile= fopen(crate::src::bzip2recover::inFileName.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
    if inFile.is_null() {();
        fprintf(
            crate::src::bzip2recover::stderr,
            b"%s: can't read `%s'\n\0" as *const u8 as *const libc::c_char,
            crate::src::bzip2recover::progName.as_mut_ptr(),
            crate::src::bzip2recover::inFileName.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    bsIn= bsOpenReadStream(inFile);
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: searching for block boundaries ...\n\0" as *const u8
            as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    bitsRead= 0 as libc::c_int as MaybeUInt64;
    buffLo= 0 as libc::c_int as UInt32;
    buffHi= buffLo;
    currBlock= 0 as libc::c_int;
    crate::src::bzip2recover::bStart[currBlock as usize]= 0 as libc::c_int as MaybeUInt64;
    rbCtr= 0 as libc::c_int;
    while 1 as libc::c_int as Bool != 0 {
        b= bsGetBit(bsIn.as_mut());
        bitsRead= bitsRead.wrapping_add(1);
        if b == 2 as libc::c_int {
            if bitsRead >= crate::src::bzip2recover::bStart[currBlock as usize]
                && bitsRead.wrapping_sub(crate::src::bzip2recover::bStart[currBlock as usize])
                    >= 40 as libc::c_int as libc::c_ulonglong
            {
                crate::src::bzip2recover::bEnd[currBlock
                    as usize]= bitsRead
                    .wrapping_sub(1 as libc::c_int as libc::c_ulonglong);
                if currBlock > 0 as libc::c_int {
                    fprintf(
                        crate::src::bzip2recover::stderr,
                        b"   block %d runs from %Lu to %Lu (incomplete)\n\0" as *const u8
                            as *const libc::c_char,
                        currBlock,
                        crate::src::bzip2recover::bStart[currBlock as usize],
                        crate::src::bzip2recover::bEnd[currBlock as usize],
                    );
                }
            } else {
                currBlock-= 1;
            }
            break;
        } else {
            buffHi= buffHi << 1 as libc::c_int | buffLo >> 31 as libc::c_int;
            buffLo= buffLo << 1 as libc::c_int | (b & 1 as libc::c_int) as libc::c_uint;
            if (buffHi & 0xffff as libc::c_int as libc::c_uint) as libc::c_ulong
                == 0x3141 as libc::c_ulong
                && buffLo as libc::c_ulong == 0x59265359 as libc::c_ulong
                || (buffHi & 0xffff as libc::c_int as libc::c_uint) as libc::c_ulong
                    == 0x1772 as libc::c_ulong
                    && buffLo as libc::c_ulong == 0x45385090 as libc::c_ulong
            {
                if bitsRead > 49 as libc::c_int as libc::c_ulonglong {
                    crate::src::bzip2recover::bEnd[currBlock
                        as usize]= bitsRead
                        .wrapping_sub(49 as libc::c_int as libc::c_ulonglong);
                } else {
                    crate::src::bzip2recover::bEnd[currBlock as usize]= 0 as libc::c_int as MaybeUInt64;
                }
                if currBlock > 0 as libc::c_int
                    && crate::src::bzip2recover::bEnd[currBlock as usize]
                        .wrapping_sub(crate::src::bzip2recover::bStart[currBlock as usize])
                        >= 130 as libc::c_int as libc::c_ulonglong
                {
                    fprintf(
                        crate::src::bzip2recover::stderr,
                        b"   block %d runs from %Lu to %Lu\n\0" as *const u8
                            as *const libc::c_char,
                        rbCtr + 1 as libc::c_int,
                        crate::src::bzip2recover::bStart[currBlock as usize],
                        crate::src::bzip2recover::bEnd[currBlock as usize],
                    );
                    crate::src::bzip2recover::rbStart[rbCtr as usize]= crate::src::bzip2recover::bStart[currBlock as usize];
                    crate::src::bzip2recover::rbEnd[rbCtr as usize]= crate::src::bzip2recover::bEnd[currBlock as usize];
                    rbCtr+= 1;
                }
                if currBlock >= 50000 as libc::c_int {
                    tooManyBlocks(50000 as libc::c_int);
                }
                currBlock+= 1;
                crate::src::bzip2recover::bStart[currBlock as usize]= bitsRead;
            }
        }
    }
    bsClose(bsIn);
    if rbCtr < 1 as libc::c_int {
        fprintf(
            crate::src::bzip2recover::stderr,
            b"%s: sorry, I couldn't find any block boundaries.\n\0" as *const u8
                as *const libc::c_char,
            crate::src::bzip2recover::progName.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: splitting into blocks\n\0" as *const u8 as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    inFile= fopen(crate::src::bzip2recover::inFileName.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
    if inFile.is_null() {();
        fprintf(
            crate::src::bzip2recover::stderr,
            b"%s: can't open `%s'\n\0" as *const u8 as *const libc::c_char,
            crate::src::bzip2recover::progName.as_mut_ptr(),
            crate::src::bzip2recover::inFileName.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    bsIn= bsOpenReadStream(inFile);
    blockCRC= 0 as libc::c_int as UInt32;
    bsWr= 0 as *mut BitStream;
    bitsRead= 0 as libc::c_int as MaybeUInt64;
    outFile= 0 as *mut FILE;
    wrBlock= 0 as libc::c_int;
    while 1 as libc::c_int as Bool != 0 {
        b= bsGetBit(bsIn.as_mut());
        if b == 2 as libc::c_int {
            break;
        }
        buffHi= buffHi << 1 as libc::c_int | buffLo >> 31 as libc::c_int;
        buffLo= buffLo << 1 as libc::c_int | (b & 1 as libc::c_int) as libc::c_uint;
        if bitsRead
            == (47 as libc::c_int as libc::c_ulonglong)
                .wrapping_add(crate::src::bzip2recover::rbStart[wrBlock as usize])
        {
            blockCRC= buffHi << 16 as libc::c_int | buffLo >> 16 as libc::c_int;
        }
        if !outFile.is_null() && bitsRead >= crate::src::bzip2recover::rbStart[wrBlock as usize]
            && bitsRead <= crate::src::bzip2recover::rbEnd[wrBlock as usize]
        {
            bsPutBit(bsWr.as_mut(), b);
        }
        bitsRead= bitsRead.wrapping_add(1);
        if bitsRead
            == crate::src::bzip2recover::rbEnd[wrBlock as usize]
                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
        {
            if !outFile.is_null() {
                bsPutUChar(bsWr.as_mut(), 0x17 as libc::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x72 as libc::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x45 as libc::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x38 as libc::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x50 as libc::c_int as UChar);
                bsPutUChar(bsWr.as_mut(), 0x90 as libc::c_int as UChar);
                bsPutUInt32(bsWr.as_mut(), blockCRC);
                bsClose(bsWr);
                outFile= 0 as *mut FILE;
            }else { (); }
            if wrBlock >= rbCtr {
                break;
            }
            wrBlock+= 1;
        } else if bitsRead == crate::src::bzip2recover::rbStart[wrBlock as usize] {
            let mut split = 0 as *mut Char;
            let mut ofs: Int32 = 0;
            let mut k: Int32 = 0;
            k= 0 as libc::c_int;
            while k < 2000 as libc::c_int {
                crate::src::bzip2recover::outFileName[k as usize]= 0 as libc::c_int as Char;
                k+= 1;
            }
            strcpy(crate::src::bzip2recover::outFileName.as_mut_ptr(), crate::src::bzip2recover::inFileName.as_mut_ptr());
            split= strrchr(crate::src::bzip2recover::outFileName.as_mut_ptr(), '/' as i32);
            if split.is_null() {();
                split= crate::src::bzip2recover::outFileName.as_mut_ptr();
            } else {
                split= split.offset(1);
            }
            ofs= split.offset_from(crate::src::bzip2recover::outFileName.as_mut_ptr()) as libc::c_long as Int32;
            sprintf(
                split,
                b"rec%5d\0" as *const u8 as *const libc::c_char,
                wrBlock + 1 as libc::c_int,
            );
            p= split;
            while (*p) as libc::c_int != 0 as libc::c_int {
                if (*p) as libc::c_int == ' ' as i32 {
                    *p= '0' as i32 as Char;
                }
                p= p.offset(1);
            }
            strcat(
                crate::src::bzip2recover::outFileName.as_mut_ptr(),
                crate::src::bzip2recover::inFileName.as_mut_ptr().offset(ofs as isize),
            );
            if endsInBz2(crate::src::bzip2recover::outFileName.as_mut_ptr()) == 0 {
                strcat(
                    crate::src::bzip2recover::outFileName.as_mut_ptr(),
                    b".bz2\0" as *const u8 as *const libc::c_char,
                );
            }
            fprintf(
                crate::src::bzip2recover::stderr,
                b"   writing block %d to `%s' ...\n\0" as *const u8
                    as *const libc::c_char,
                wrBlock + 1 as libc::c_int,
                crate::src::bzip2recover::outFileName.as_mut_ptr(),
            );
            outFile= fopen(
                crate::src::bzip2recover::outFileName.as_mut_ptr(),
                b"wb\0" as *const u8 as *const libc::c_char,
            );
            if outFile.is_null() {();
                fprintf(
                    crate::src::bzip2recover::stderr,
                    b"%s: can't write `%s'\n\0" as *const u8 as *const libc::c_char,
                    crate::src::bzip2recover::progName.as_mut_ptr(),
                    crate::src::bzip2recover::outFileName.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
            bsWr= bsOpenWriteStream(outFile);
            bsPutUChar(bsWr.as_mut(), 0x42 as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x5a as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x68 as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), (0x30 as libc::c_int + 9 as libc::c_int) as UChar);
            bsPutUChar(bsWr.as_mut(), 0x31 as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x41 as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x59 as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x26 as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x53 as libc::c_int as UChar);
            bsPutUChar(bsWr.as_mut(), 0x59 as libc::c_int as UChar);
        }
    }
    fprintf(
        crate::src::bzip2recover::stderr,
        b"%s: finished\n\0" as *const u8 as *const libc::c_char,
        crate::src::bzip2recover::progName.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0((args.len() - 1) as Int32, args.as_mut_ptr() as *mut *mut Char) as i32,
//         )
//     }
// }
