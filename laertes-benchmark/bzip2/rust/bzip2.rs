
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
    fn perror(_: *const std::os::raw::c_char);
    #[no_mangle]
    fn remove(_: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn rewind(_: *mut FILE);
    #[no_mangle]
    fn ungetc(_: std::os::raw::c_int, _: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fdopen(_: std::os::raw::c_int, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fileno(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn getenv(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn signal(_: std::os::raw::c_int,
              _: Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>)
     -> Option<unsafe extern "C" fn(_: std::os::raw::c_int) -> ()>;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strerror(_: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strncpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __maskrune(_: __darwin_ct_rune_t, _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    #[no_mangle]
    fn BZ2_bzReadOpen(bzerror: *mut std::os::raw::c_int, f: *mut FILE,
                      verbosity_0: std::os::raw::c_int, small: std::os::raw::c_int,
                      unused: *mut std::os::raw::c_void, nUnused: std::os::raw::c_int)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn BZ2_bzReadClose(bzerror: *mut std::os::raw::c_int, b: *mut std::os::raw::c_void);
    #[no_mangle]
    fn BZ2_bzReadGetUnused(bzerror: *mut std::os::raw::c_int, b: *mut std::os::raw::c_void,
                           unused: *mut *mut std::os::raw::c_void,
                           nUnused: *mut std::os::raw::c_int);
    #[no_mangle]
    fn BZ2_bzRead(bzerror: *mut std::os::raw::c_int, b: *mut std::os::raw::c_void,
                  buf: *mut std::os::raw::c_void, len: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn BZ2_bzWriteOpen(bzerror: *mut std::os::raw::c_int, f: *mut FILE,
                       blockSize100k_0: std::os::raw::c_int, verbosity_0: std::os::raw::c_int,
                       workFactor_0: std::os::raw::c_int) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn BZ2_bzWrite(bzerror: *mut std::os::raw::c_int, b: *mut std::os::raw::c_void,
                   buf: *mut std::os::raw::c_void, len: std::os::raw::c_int);
    #[no_mangle]
    fn BZ2_bzWriteClose64(bzerror: *mut std::os::raw::c_int, b: *mut std::os::raw::c_void,
                          abandon: std::os::raw::c_int,
                          nbytes_in_lo32: *mut std::os::raw::c_uint,
                          nbytes_in_hi32: *mut std::os::raw::c_uint,
                          nbytes_out_lo32: *mut std::os::raw::c_uint,
                          nbytes_out_hi32: *mut std::os::raw::c_uint);
    #[no_mangle]
    fn BZ2_bzlibVersion() -> *const std::os::raw::c_char;
    #[no_mangle]
    fn open(_: *const std::os::raw::c_char, _: std::os::raw::c_int, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn utime(_: *const std::os::raw::c_char, _: *const utimbuf) -> std::os::raw::c_int;
    #[no_mangle]
    fn _exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn close(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn isatty(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn write(__fd: std::os::raw::c_int, __buf: *const std::os::raw::c_void, __nbyte: size_t)
     -> ssize_t;
    #[no_mangle]
    fn fchown(_: std::os::raw::c_int, _: uid_t, _: gid_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn fchmod(_: std::os::raw::c_int, _: mode_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn lstat(_: *const std::os::raw::c_char, _: *mut stat) -> std::os::raw::c_int;
    #[no_mangle]
    fn stat(_: *const std::os::raw::c_char, _: *mut stat) -> std::os::raw::c_int;
}
pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_longlong;
pub type __uint64_t = std::os::raw::c_ulonglong;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_ssize_t = std::os::raw::c_long;
pub type __darwin_time_t = std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
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
pub type off_t = __darwin_off_t;
pub type ssize_t = __darwin_ssize_t;
pub type uid_t = __darwin_uid_t;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
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
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: std::os::raw::c_long,
}
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type nlink_t = __uint16_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
/* */
/* BZ_UNIX */
/* BZ_LCCWIN32 */
/*---------------------------------------------*/
/*--
  Some more stuff for all platforms :-)
--*/
pub type Char = std::os::raw::c_char;
pub type Bool = std::os::raw::c_uchar;
pub type UChar = std::os::raw::c_uchar;
pub type Int32 = std::os::raw::c_int;
pub type UInt32 = std::os::raw::c_uint;
pub type Int16 = std::os::raw::c_short;
pub type UInt16 = std::os::raw::c_ushort;
/*--
  IntNative is your platform's `native' int size.
  Only here to avoid probs with 64-bit platforms.
--*/
pub type IntNative = std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UInt64 {
    pub b: [UChar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zzzz {
    pub name: *mut Char,
    pub link: *mut zzzz,
}
/*---------------------------------------------*/
/*--
  All the garbage from here to main() is purely to
  implement a linked list of command-line arguments,
  into which main() copies argv[1 .. argc-1].

  The purpose of this exercise is to facilitate 
  the expansion of wildcard characters * and ? in 
  filenames for OSs which don't know how to do it
  themselves, like MSDOS, Windows 95 and NT.

  The actual Dirty Work is done by the platform-
  specific macro APPEND_FILESPEC.
--*/
pub type Cell = zzzz;
#[inline]
unsafe extern "C" fn isascii(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (_c & !(0x7f as std::os::raw::c_int) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t,
                              mut _f: std::os::raw::c_ulong) -> std::os::raw::c_int {
    return if isascii(_c) != 0 {
               (_DefaultRuneLocale.__runetype[_c as usize] as std::os::raw::c_ulong &
                    _f != 0) as std::os::raw::c_int
           } else { (__maskrune(_c, _f) != 0) as std::os::raw::c_int };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __istype(_c, 0x4000 as std::os::raw::c_long as std::os::raw::c_ulong);
}
/*---------------------------------------------------*/
/*--- Misc (file handling) data decls             ---*/
/*---------------------------------------------------*/
#[no_mangle]
pub static mut verbosity: Int32 = 0;
#[no_mangle]
pub static mut keepInputFiles: Bool = 0;
#[no_mangle]
pub static mut smallMode: Bool = 0;
#[no_mangle]
pub static mut deleteOutputOnInterrupt: Bool = 0;
#[no_mangle]
pub static mut forceOverwrite: Bool = 0;
#[no_mangle]
pub static mut testFailsExist: Bool = 0;
#[no_mangle]
pub static mut unzFailsExist: Bool = 0;
#[no_mangle]
pub static mut noisy: Bool = 0;
#[no_mangle]
pub static mut numFileNames: Int32 = 0;
#[no_mangle]
pub static mut numFilesProcessed: Int32 = 0;
#[no_mangle]
pub static mut blockSize100k: Int32 = 0;
#[no_mangle]
pub static mut exitValue: Int32 = 0;
#[no_mangle]
pub static mut opMode: Int32 = 0;
#[no_mangle]
pub static mut srcMode: Int32 = 0;
#[no_mangle]
pub static mut longestFileName: Int32 = 0;
#[no_mangle]
pub static mut inName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut outName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut tmpName: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut progName: *mut Char = 0 as *const Char as *mut Char;
#[no_mangle]
pub static mut progNameReally: [Char; 1034] = [0; 1034];
#[no_mangle]
pub static mut outputHandleJustInCase: *mut FILE =
    0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut workFactor: Int32 = 0;
unsafe extern "C" fn uInt64_from_UInt32s(mut n: *mut UInt64, mut lo32: UInt32,
                                         mut hi32: UInt32) {
    (*n).b[7 as std::os::raw::c_int as usize] =
        (hi32 >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[6 as std::os::raw::c_int as usize] =
        (hi32 >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[5 as std::os::raw::c_int as usize] =
        (hi32 >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[4 as std::os::raw::c_int as usize] =
        (hi32 & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
    (*n).b[3 as std::os::raw::c_int as usize] =
        (lo32 >> 24 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[2 as std::os::raw::c_int as usize] =
        (lo32 >> 16 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[1 as std::os::raw::c_int as usize] =
        (lo32 >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as
            UChar;
    (*n).b[0 as std::os::raw::c_int as usize] =
        (lo32 & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
}
unsafe extern "C" fn uInt64_to_double(mut n: *mut UInt64) -> std::os::raw::c_double {
    let mut i: Int32 = 0;
    let mut base: std::os::raw::c_double = 1.0f64;
    let mut sum: std::os::raw::c_double = 0.0f64;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        sum += base * (*n).b[i as usize] as std::os::raw::c_double;
        base *= 256.0f64;
        i += 1
    }
    return sum;
}
unsafe extern "C" fn uInt64_isZero(mut n: *mut UInt64) -> Bool {
    let mut i: Int32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        if (*n).b[i as usize] as std::os::raw::c_int != 0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int as Bool
        }
        i += 1
    }
    return 1 as std::os::raw::c_int as Bool;
}
/* Divide *n by 10, and return the remainder.  */
unsafe extern "C" fn uInt64_qrm10(mut n: *mut UInt64) -> Int32 {
    let mut rem: UInt32 = 0;
    let mut tmp: UInt32 = 0;
    let mut i: Int32 = 0;
    rem = 0 as std::os::raw::c_int as UInt32;
    i = 7 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        tmp =
            rem.wrapping_mul(256 as std::os::raw::c_int as
                                 std::os::raw::c_uint).wrapping_add((*n).b[i as usize]
                                                                as
                                                                std::os::raw::c_uint);
        (*n).b[i as usize] =
            tmp.wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_uint) as UChar;
        rem = tmp.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_uint);
        i -= 1
    }
    return rem as Int32;
}
/* ... and the Whole Entire Point of all this UInt64 stuff is
   so that we can supply the following function.
*/
unsafe extern "C" fn uInt64_toAscii(mut outbuf: *mut std::os::raw::c_char,
                                    mut n: *mut UInt64) {
    let mut i: Int32 = 0;
    let mut q: Int32 = 0;
    let mut buf: [UChar; 32] = [0; 32];
    let mut nBuf: Int32 = 0 as std::os::raw::c_int;
    let mut n_copy: UInt64 = *n;
    loop  {
        q = uInt64_qrm10(&mut n_copy);
        buf[nBuf as usize] = (q + '0' as i32) as UChar;
        nBuf += 1;
        if !(uInt64_isZero(&mut n_copy) == 0) { break ; }
    }
    *outbuf.offset(nBuf as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    while i < nBuf {
        *outbuf.offset(i as isize) =
            buf[(nBuf - i - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_char;
        i += 1
    };
}
/*---------------------------------------------------*/
/*--- Processing of complete files and streams    ---*/
/*---------------------------------------------------*/
/*---------------------------------------------*/
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c: Int32 = fgetc(f);
    if c == -(1 as std::os::raw::c_int) { return 1 as std::os::raw::c_int as Bool }
    ungetc(c, f);
    return 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
unsafe extern "C" fn compressStream(mut stream: *mut FILE,
                                    mut zStream: *mut FILE) {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut ibuf: [UChar; 5000] = [0; 5000];
    let mut nIbuf: Int32 = 0;
    let mut nbytes_in_lo32: UInt32 = 0;
    let mut nbytes_in_hi32: UInt32 = 0;
    let mut nbytes_out_lo32: UInt32 = 0;
    let mut nbytes_out_hi32: UInt32 = 0;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            bzf =
                BZ2_bzWriteOpen(&mut bzerr, zStream, blockSize100k, verbosity,
                                workFactor);
            if bzerr != 0 as std::os::raw::c_int {
                current_block = 660242869387099075;
            } else {
                if verbosity >= 2 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"\n\x00" as *const u8 as *const std::os::raw::c_char);
                }
                loop  {
                    if !(1 as std::os::raw::c_int as Bool != 0) {
                        current_block = 13242334135786603907;
                        break ;
                    }
                    if myfeof(stream) != 0 {
                        current_block = 13242334135786603907;
                        break ;
                    }
                    nIbuf =
                        fread(ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                              ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong,
                              5000 as std::os::raw::c_int as std::os::raw::c_ulong, stream) as
                            Int32;
                    if ferror(stream) != 0 {
                        current_block = 18225113528933273530;
                        break ;
                    }
                    if nIbuf > 0 as std::os::raw::c_int {
                        BZ2_bzWrite(&mut bzerr, bzf,
                                    ibuf.as_mut_ptr() as *mut std::os::raw::c_void,
                                    nIbuf);
                    }
                    if bzerr != 0 as std::os::raw::c_int {
                        current_block = 660242869387099075;
                        break ;
                    }
                }
                match current_block {
                    18225113528933273530 => { }
                    660242869387099075 => { }
                    _ => {
                        BZ2_bzWriteClose64(&mut bzerr, bzf, 0 as std::os::raw::c_int,
                                           &mut nbytes_in_lo32,
                                           &mut nbytes_in_hi32,
                                           &mut nbytes_out_lo32,
                                           &mut nbytes_out_hi32);
                        if bzerr != 0 as std::os::raw::c_int {
                            current_block = 660242869387099075;
                        } else if ferror(zStream) != 0 {
                            current_block = 18225113528933273530;
                        } else {
                            ret = fflush(zStream);
                            if ret == -(1 as std::os::raw::c_int) {
                                current_block = 18225113528933273530;
                            } else {
                                if zStream != __stdoutp {
                                    let mut fd: Int32 = fileno(zStream);
                                    if fd < 0 as std::os::raw::c_int {
                                        current_block = 18225113528933273530;
                                    } else {
                                        applySavedFileAttrToOutputFile(fd);
                                        ret = fclose(zStream);
                                        outputHandleJustInCase =
                                            0 as *mut FILE;
                                        if ret == -(1 as std::os::raw::c_int) {
                                            current_block =
                                                18225113528933273530;
                                        } else {
                                            current_block =
                                                17281240262373992796;
                                        }
                                    }
                                } else {
                                    current_block = 17281240262373992796;
                                }
                                match current_block {
                                    18225113528933273530 => { }
                                    _ => {
                                        outputHandleJustInCase =
                                            0 as *mut FILE;
                                        if ferror(stream) != 0 {
                                            current_block =
                                                18225113528933273530;
                                        } else {
                                            ret = fclose(stream);
                                            if ret == -(1 as std::os::raw::c_int) {
                                                current_block =
                                                    18225113528933273530;
                                            } else {
                                                if verbosity >=
                                                       1 as std::os::raw::c_int {
                                                    if nbytes_in_lo32 ==
                                                           0 as std::os::raw::c_int as
                                                               std::os::raw::c_uint &&
                                                           nbytes_in_hi32 ==
                                                               0 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_uint
                                                       {
                                                        fprintf(__stderrp,
                                                                b" no data compressed.\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const std::os::raw::c_char);
                                                    } else {
                                                        let mut buf_nin:
                                                                [Char; 32] =
                                                            [0; 32];
                                                        let mut buf_nout:
                                                                [Char; 32] =
                                                            [0; 32];
                                                        let mut nbytes_in:
                                                                UInt64 =
                                                            UInt64{b:
                                                                       [0;
                                                                           8],};
                                                        let mut nbytes_out:
                                                                UInt64 =
                                                            UInt64{b:
                                                                       [0;
                                                                           8],};
                                                        let mut nbytes_in_d:
                                                                std::os::raw::c_double =
                                                            0.;
                                                        let mut nbytes_out_d:
                                                                std::os::raw::c_double =
                                                            0.;
                                                        uInt64_from_UInt32s(&mut nbytes_in,
                                                                            nbytes_in_lo32,
                                                                            nbytes_in_hi32);
                                                        uInt64_from_UInt32s(&mut nbytes_out,
                                                                            nbytes_out_lo32,
                                                                            nbytes_out_hi32);
                                                        nbytes_in_d =
                                                            uInt64_to_double(&mut nbytes_in);
                                                        nbytes_out_d =
                                                            uInt64_to_double(&mut nbytes_out);
                                                        uInt64_toAscii(buf_nin.as_mut_ptr(),
                                                                       &mut nbytes_in);
                                                        uInt64_toAscii(buf_nout.as_mut_ptr(),
                                                                       &mut nbytes_out);
                                                        fprintf(__stderrp,
                                                                b"%6.3f:1, %6.3f bits/byte, %5.2f%% saved, %s in, %s out.\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const std::os::raw::c_char,
                                                                nbytes_in_d /
                                                                    nbytes_out_d,
                                                                8.0f64 *
                                                                    nbytes_out_d
                                                                    /
                                                                    nbytes_in_d,
                                                                100.0f64 *
                                                                    (1.0f64 -
                                                                         nbytes_out_d
                                                                             /
                                                                             nbytes_in_d),
                                                                buf_nin.as_mut_ptr(),
                                                                buf_nout.as_mut_ptr());
                                                    }
                                                }
                                                return
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                18225113528933273530 => { }
                _ => {
                    BZ2_bzWriteClose64(&mut bzerr_dummy, bzf,
                                       1 as std::os::raw::c_int, &mut nbytes_in_lo32,
                                       &mut nbytes_in_hi32,
                                       &mut nbytes_out_lo32,
                                       &mut nbytes_out_hi32);
                    match bzerr {
                        -9 => {
                            current_block = 5122324059762049690;
                            match current_block {
                                4323399205346619401 => {
                                    panic(b"compress:unexpected error\x00" as
                                              *const u8 as
                                              *const std::os::raw::c_char);
                                }
                                2380987886157893679 => { outOfMemory(); }
                                _ => { configError(); }
                            }
                        }
                        -3 => {
                            current_block = 2380987886157893679;
                            match current_block {
                                4323399205346619401 => {
                                    panic(b"compress:unexpected error\x00" as
                                              *const u8 as
                                              *const std::os::raw::c_char);
                                }
                                2380987886157893679 => { outOfMemory(); }
                                _ => { configError(); }
                            }
                        }
                        -6 => { }
                        _ => {
                            current_block = 4323399205346619401;
                            match current_block {
                                4323399205346619401 => {
                                    panic(b"compress:unexpected error\x00" as
                                              *const u8 as
                                              *const std::os::raw::c_char);
                                }
                                2380987886157893679 => { outOfMemory(); }
                                _ => { configError(); }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------*/
unsafe extern "C" fn uncompressStream(mut zStream: *mut FILE,
                                      mut stream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut nread: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut UChar = 0 as *mut UChar;
    nUnused = 0 as std::os::raw::c_int;
    streamNo = 0 as std::os::raw::c_int;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            's_51:
                loop  {
                    if !(1 as std::os::raw::c_int as Bool != 0) {
                        current_block = 926243229934402080;
                        break ;
                    }
                    bzf =
                        BZ2_bzReadOpen(&mut bzerr, zStream, verbosity,
                                       smallMode as std::os::raw::c_int,
                                       unused.as_mut_ptr() as
                                           *mut std::os::raw::c_void, nUnused);
                    if bzf.is_null() || bzerr != 0 as std::os::raw::c_int {
                        current_block = 673979509383251364;
                        break ;
                    }
                    streamNo += 1;
                    while bzerr == 0 as std::os::raw::c_int {
                        nread =
                            BZ2_bzRead(&mut bzerr, bzf,
                                       obuf.as_mut_ptr() as *mut std::os::raw::c_void,
                                       5000 as std::os::raw::c_int);
                        if bzerr == -(5 as std::os::raw::c_int) {
                            current_block = 18063049917807660484;
                            break 's_51 ;
                        }
                        if (bzerr == 0 as std::os::raw::c_int ||
                                bzerr == 4 as std::os::raw::c_int) &&
                               nread > 0 as std::os::raw::c_int {
                            fwrite(obuf.as_mut_ptr() as *const std::os::raw::c_void,
                                   ::std::mem::size_of::<UChar>() as
                                       std::os::raw::c_ulong, nread as std::os::raw::c_ulong,
                                   stream);
                        }
                        if ferror(stream) != 0 {
                            current_block = 5049394217699438129;
                            break 's_51 ;
                        }
                    }
                    if bzerr != 4 as std::os::raw::c_int {
                        current_block = 673979509383251364;
                        break ;
                    }
                    BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV,
                                        &mut nUnused);
                    if bzerr != 0 as std::os::raw::c_int {
                        panic(b"decompress:bzReadGetUnused\x00" as *const u8
                                  as *const std::os::raw::c_char);
                    }
                    unusedTmp = unusedTmpV as *mut UChar;
                    i = 0 as std::os::raw::c_int;
                    while i < nUnused {
                        unused[i as usize] = *unusedTmp.offset(i as isize);
                        i += 1
                    }
                    BZ2_bzReadClose(&mut bzerr, bzf);
                    if bzerr != 0 as std::os::raw::c_int {
                        panic(b"decompress:bzReadGetUnused\x00" as *const u8
                                  as *const std::os::raw::c_char);
                    }
                    if nUnused == 0 as std::os::raw::c_int &&
                           myfeof(zStream) as std::os::raw::c_int != 0 {
                        current_block = 926243229934402080;
                        break ;
                    }
                }
            match current_block {
                5049394217699438129 => { }
                _ => {
                    match current_block {
                        18063049917807660484 => {
                            if forceOverwrite != 0 {
                                rewind(zStream);
                                loop  {
                                    if !(1 as std::os::raw::c_int as Bool != 0) {
                                        current_block = 926243229934402080;
                                        break ;
                                    }
                                    if myfeof(zStream) != 0 {
                                        current_block = 926243229934402080;
                                        break ;
                                    }
                                    nread =
                                        fread(obuf.as_mut_ptr() as
                                                  *mut std::os::raw::c_void,
                                              ::std::mem::size_of::<UChar>()
                                                  as std::os::raw::c_ulong,
                                              5000 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong, zStream) as
                                            Int32;
                                    if ferror(zStream) != 0 {
                                        current_block = 5049394217699438129;
                                        break ;
                                    }
                                    if nread > 0 as std::os::raw::c_int {
                                        fwrite(obuf.as_mut_ptr() as
                                                   *const std::os::raw::c_void,
                                               ::std::mem::size_of::<UChar>()
                                                   as std::os::raw::c_ulong,
                                               nread as std::os::raw::c_ulong,
                                               stream);
                                    }
                                    if ferror(stream) != 0 {
                                        current_block = 5049394217699438129;
                                        break ;
                                    }
                                }
                            } else { current_block = 673979509383251364; }
                        }
                        _ => { }
                    }
                    match current_block {
                        5049394217699438129 => { }
                        _ => {
                            match current_block {
                                673979509383251364 => {
                                    BZ2_bzReadClose(&mut bzerr_dummy, bzf);
                                    match bzerr {
                                        -9 => {
                                            current_block =
                                                3642457097893642164;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -6 => { }
                                        -4 => {
                                            current_block =
                                                10766414566319669440;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -3 => {
                                            current_block =
                                                16178635849926953562;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -7 => {
                                            current_block =
                                                5517467152645906530;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        -5 => {
                                            current_block =
                                                7372986856480808103;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            current_block =
                                                6455255476181645667;
                                            match current_block {
                                                6455255476181645667 => {
                                                    panic(b"decompress:unexpected error\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
                                                }
                                                3642457097893642164 => {
                                                    configError();
                                                }
                                                10766414566319669440 => {
                                                    crcError();
                                                }
                                                16178635849926953562 => {
                                                    outOfMemory();
                                                }
                                                5517467152645906530 => {
                                                    compressedStreamEOF();
                                                }
                                                _ => {
                                                    if zStream != __stdinp {
                                                        fclose(zStream);
                                                    }
                                                    if stream != __stdoutp {
                                                        fclose(stream);
                                                    }
                                                    if streamNo ==
                                                           1 as std::os::raw::c_int {
                                                        return 0 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    } else {
                                                        if noisy != 0 {
                                                            fprintf(__stderrp,
                                                                    b"\n%s: %s: trailing garbage after EOF ignored\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const std::os::raw::c_char,
                                                                    progName,
                                                                    inName.as_mut_ptr());
                                                        }
                                                        return 1 as
                                                                   std::os::raw::c_int
                                                                   as Bool
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    if !(ferror(zStream) != 0) {
                                        if stream != __stdoutp {
                                            let mut fd: Int32 =
                                                fileno(stream);
                                            if fd < 0 as std::os::raw::c_int {
                                                current_block =
                                                    5049394217699438129;
                                            } else {
                                                applySavedFileAttrToOutputFile(fd);
                                                current_block =
                                                    14832935472441733737;
                                            }
                                        } else {
                                            current_block =
                                                14832935472441733737;
                                        }
                                        match current_block {
                                            5049394217699438129 => { }
                                            _ => {
                                                ret = fclose(zStream);
                                                if !(ret ==
                                                         -(1 as std::os::raw::c_int))
                                                   {
                                                    if !(ferror(stream) != 0)
                                                       {
                                                        ret = fflush(stream);
                                                        if !(ret !=
                                                                 0 as
                                                                     std::os::raw::c_int)
                                                           {
                                                            if stream !=
                                                                   __stdoutp {
                                                                ret =
                                                                    fclose(stream);
                                                                outputHandleJustInCase
                                                                    =
                                                                    0 as
                                                                        *mut FILE;
                                                                if ret ==
                                                                       -(1 as
                                                                             std::os::raw::c_int)
                                                                   {
                                                                    current_block
                                                                        =
                                                                        5049394217699438129;
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        14775119014532381840;
                                                                }
                                                            } else {
                                                                current_block
                                                                    =
                                                                    14775119014532381840;
                                                            }
                                                            match current_block
                                                                {
                                                                5049394217699438129
                                                                => {
                                                                }
                                                                _ => {
                                                                    outputHandleJustInCase
                                                                        =
                                                                        0 as
                                                                            *mut FILE;
                                                                    if verbosity
                                                                           >=
                                                                           2
                                                                               as
                                                                               std::os::raw::c_int
                                                                       {
                                                                        fprintf(__stderrp,
                                                                                b"\n    \x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const std::os::raw::c_char);
                                                                    }
                                                                    return 1
                                                                               as
                                                                               std::os::raw::c_int
                                                                               as
                                                                               Bool
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------*/
unsafe extern "C" fn testStream(mut zStream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    let mut unusedTmp: *mut UChar = 0 as *mut UChar;
    nUnused = 0 as std::os::raw::c_int;
    streamNo = 0 as std::os::raw::c_int;
    if !(ferror(zStream) != 0) {
        's_41:
            loop  {
                if !(1 as std::os::raw::c_int as Bool != 0) {
                    current_block = 15125582407903384992;
                    break ;
                }
                bzf =
                    BZ2_bzReadOpen(&mut bzerr, zStream, verbosity,
                                   smallMode as std::os::raw::c_int,
                                   unused.as_mut_ptr() as *mut std::os::raw::c_void,
                                   nUnused);
                if bzf.is_null() || bzerr != 0 as std::os::raw::c_int {
                    current_block = 10905486111603547446;
                    break ;
                }
                streamNo += 1;
                while bzerr == 0 as std::os::raw::c_int {
                    BZ2_bzRead(&mut bzerr, bzf,
                               obuf.as_mut_ptr() as *mut std::os::raw::c_void,
                               5000 as std::os::raw::c_int);
                    if bzerr == -(5 as std::os::raw::c_int) {
                        current_block = 10905486111603547446;
                        break 's_41 ;
                    }
                }
                if bzerr != 4 as std::os::raw::c_int {
                    current_block = 10905486111603547446;
                    break ;
                }
                BZ2_bzReadGetUnused(&mut bzerr, bzf, &mut unusedTmpV,
                                    &mut nUnused);
                if bzerr != 0 as std::os::raw::c_int {
                    panic(b"test:bzReadGetUnused\x00" as *const u8 as
                              *const std::os::raw::c_char);
                }
                unusedTmp = unusedTmpV as *mut UChar;
                i = 0 as std::os::raw::c_int;
                while i < nUnused {
                    unused[i as usize] = *unusedTmp.offset(i as isize);
                    i += 1
                }
                BZ2_bzReadClose(&mut bzerr, bzf);
                if bzerr != 0 as std::os::raw::c_int {
                    panic(b"test:bzReadGetUnused\x00" as *const u8 as
                              *const std::os::raw::c_char);
                }
                if nUnused == 0 as std::os::raw::c_int &&
                       myfeof(zStream) as std::os::raw::c_int != 0 {
                    current_block = 15125582407903384992;
                    break ;
                }
            }
        match current_block {
            15125582407903384992 => {
                if !(ferror(zStream) != 0) {
                    ret = fclose(zStream);
                    if !(ret == -(1 as std::os::raw::c_int)) {
                        if verbosity >= 2 as std::os::raw::c_int {
                            fprintf(__stderrp,
                                    b"\n    \x00" as *const u8 as
                                        *const std::os::raw::c_char);
                        }
                        return 1 as std::os::raw::c_int as Bool
                    }
                }
            }
            _ => {
                BZ2_bzReadClose(&mut bzerr_dummy, bzf);
                if verbosity == 0 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"%s: %s: \x00" as *const u8 as
                                *const std::os::raw::c_char, progName,
                            inName.as_mut_ptr());
                }
                match bzerr {
                    -9 => {
                        current_block = 18238374633732057650;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -6 => { }
                    -4 => {
                        current_block = 11224962462315262049;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -3 => {
                        current_block = 13802719682174684861;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -7 => {
                        current_block = 10380742613918245393;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    -5 => {
                        current_block = 2539039579982765382;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                    _ => {
                        current_block = 6101827300316655396;
                        match current_block {
                            6101827300316655396 => {
                                panic(b"test:unexpected error\x00" as
                                          *const u8 as *const std::os::raw::c_char);
                            }
                            10380742613918245393 => {
                                fprintf(__stderrp,
                                        b"file ends unexpectedly\n\x00" as
                                            *const u8 as *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                            2539039579982765382 => {
                                if zStream != __stdinp { fclose(zStream); }
                                if streamNo == 1 as std::os::raw::c_int {
                                    fprintf(__stderrp,
                                            b"bad magic number (file not created by bzip2)\n\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char);
                                    return 0 as std::os::raw::c_int as Bool
                                } else {
                                    if noisy != 0 {
                                        fprintf(__stderrp,
                                                b"trailing garbage after EOF ignored\n\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char);
                                    }
                                    return 1 as std::os::raw::c_int as Bool
                                }
                            }
                            18238374633732057650 => { configError(); }
                            13802719682174684861 => { outOfMemory(); }
                            _ => {
                                fprintf(__stderrp,
                                        b"data integrity (CRC) error in data\n\x00"
                                            as *const u8 as
                                            *const std::os::raw::c_char);
                                return 0 as std::os::raw::c_int as Bool
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
    /*notreached*/
}
/*---------------------------------------------------*/
/*--- Error [non-] handling grunge                ---*/
/*---------------------------------------------------*/
/*---------------------------------------------*/
unsafe extern "C" fn setExit(mut v: Int32) {
    if v > exitValue { exitValue = v };
}
/*---------------------------------------------*/
unsafe extern "C" fn cadvise() {
    if noisy != 0 {
        fprintf(__stderrp,
                b"\nIt is possible that the compressed file(s) have become corrupted.\nYou can use the -tvv option to test integrity of such files.\n\nYou can use the `bzip2recover\' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\x00"
                    as *const u8 as *const std::os::raw::c_char);
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn showFileNames() {
    if noisy != 0 {
        fprintf(__stderrp,
                b"\tInput file = %s, output file = %s\n\x00" as *const u8 as
                    *const std::os::raw::c_char, inName.as_mut_ptr(),
                outName.as_mut_ptr());
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn cleanUpAndFail(mut ec: Int32) -> ! {
    let mut retVal: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    if srcMode == 3 as std::os::raw::c_int && opMode != 3 as std::os::raw::c_int &&
           deleteOutputOnInterrupt as std::os::raw::c_int != 0 {
        /* Check whether input file still exists.  Delete output file
         only if input exists to avoid loss of data.  Joerg Prante, 5
         January 2002.  (JRS 06-Jan-2002: other changes in 1.0.2 mean
         this is less likely to happen.  But to be ultra-paranoid, we
         do the check anyway.)  */
        retVal = stat(inName.as_mut_ptr(), &mut statBuf);
        if retVal == 0 as std::os::raw::c_int {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"%s: Deleting output file %s, if it exists.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr());
            }
            if !outputHandleJustInCase.is_null() {
                fclose(outputHandleJustInCase);
            }
            retVal = remove(outName.as_mut_ptr());
            if retVal != 0 as std::os::raw::c_int {
                fprintf(__stderrp,
                        b"%s: WARNING: deletion of output file (apparently) failed.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
            }
        } else {
            fprintf(__stderrp,
                    b"%s: WARNING: deletion of output file suppressed\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
            fprintf(__stderrp,
                    b"%s:    since input file no longer exists.  Output file\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
            fprintf(__stderrp,
                    b"%s:    `%s\' may be incomplete.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, outName.as_mut_ptr());
            fprintf(__stderrp,
                    b"%s:    I suggest doing an integrity test (bzip2 -tv) of it.\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName);
        }
    }
    if noisy as std::os::raw::c_int != 0 && numFileNames > 0 as std::os::raw::c_int &&
           numFilesProcessed < numFileNames {
        fprintf(__stderrp,
                b"%s: WARNING: some files have not been processed:\n%s:    %d specified on command line, %d not processed yet.\n\n\x00"
                    as *const u8 as *const std::os::raw::c_char, progName, progName,
                numFileNames, numFileNames - numFilesProcessed);
    }
    setExit(ec);
    exit(exitValue);
}
/*---------------------------------------------*/
unsafe extern "C" fn panic(mut s: *const Char) -> ! {
    fprintf(__stderrp,
            b"\n%s: PANIC -- internal consistency error:\n\t%s\n\tThis is a BUG.  Please report it to:\n\tbzip2-devel@sourceware.org\n\x00"
                as *const u8 as *const std::os::raw::c_char, progName, s);
    showFileNames();
    cleanUpAndFail(3 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn crcError() -> ! {
    fprintf(__stderrp,
            b"\n%s: Data integrity error when decompressing.\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName);
    showFileNames();
    cadvise();
    cleanUpAndFail(2 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn compressedStreamEOF() -> ! {
    if noisy != 0 {
        fprintf(__stderrp,
                b"\n%s: Compressed file ends unexpectedly;\n\tperhaps it is corrupted?  *Possible* reason follows.\n\x00"
                    as *const u8 as *const std::os::raw::c_char, progName);
        perror(progName);
        showFileNames();
        cadvise();
    }
    cleanUpAndFail(2 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn ioError() -> ! {
    fprintf(__stderrp,
            b"\n%s: I/O or other error, bailing out.  Possible reason follows.\n\x00"
                as *const u8 as *const std::os::raw::c_char, progName);
    perror(progName);
    showFileNames();
    cleanUpAndFail(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn mySignalCatcher(mut n: IntNative) {
    fprintf(__stderrp,
            b"\n%s: Control-C or similar caught, quitting.\n\x00" as *const u8
                as *const std::os::raw::c_char, progName);
    cleanUpAndFail(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn mySIGSEGVorSIGBUScatcher(mut n: IntNative) {
    let mut msg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if opMode == 1 as std::os::raw::c_int {
        msg =
            b": Caught a SIGSEGV or SIGBUS whilst compressing.\n\n   Possible causes are (most likely first):\n   (1) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (2) A bug in the compiler used to create this executable\n       (unlikely, if you didn\'t compile bzip2 yourself.)\n   (3) A real bug in bzip2 -- I hope this should never be the case.\n   The user\'s manual, Section 4.3, has more info on (1) and (2).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (1)\n   or (2), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user\'s manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don\'t\n   have the manual or can\'t be bothered to read it, mail me anyway.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char
    } else {
        msg =
            b": Caught a SIGSEGV or SIGBUS whilst decompressing.\n\n   Possible causes are (most likely first):\n   (1) The compressed data is corrupted, and bzip2\'s usual checks\n       failed to detect this.  Try bzip2 -tvv my_file.bz2.\n   (2) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (3) A bug in the compiler used to create this executable\n       (unlikely, if you didn\'t compile bzip2 yourself.)\n   (4) A real bug in bzip2 -- I hope this should never be the case.\n   The user\'s manual, Section 4.3, has more info on (2) and (3).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (2)\n   or (3), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user\'s manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don\'t\n   have the manual or can\'t be bothered to read it, mail me anyway.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char
    }
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    write(2 as std::os::raw::c_int, progName as *const std::os::raw::c_void,
          strlen(progName));
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    msg = b"\tInput file = \x00" as *const u8 as *const std::os::raw::c_char;
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    write(2 as std::os::raw::c_int, inName.as_mut_ptr() as *const std::os::raw::c_void,
          strlen(inName.as_mut_ptr()));
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    msg = b"\tOutput file = \x00" as *const u8 as *const std::os::raw::c_char;
    write(2 as std::os::raw::c_int, msg as *const std::os::raw::c_void, strlen(msg));
    write(2 as std::os::raw::c_int, outName.as_mut_ptr() as *const std::os::raw::c_void,
          strlen(outName.as_mut_ptr()));
    write(2 as std::os::raw::c_int,
          b"\n\x00" as *const u8 as *const std::os::raw::c_char as
              *const std::os::raw::c_void, 1 as std::os::raw::c_int as size_t);
    /* Don't call cleanupAndFail. If we ended up here something went
      terribly wrong. Trying to clean up might fail spectacularly. */
    if opMode == 1 as std::os::raw::c_int {
        setExit(3 as std::os::raw::c_int);
    } else { setExit(2 as std::os::raw::c_int); }
    _exit(exitValue);
}
/*---------------------------------------------*/
unsafe extern "C" fn outOfMemory() -> ! {
    fprintf(__stderrp,
            b"\n%s: couldn\'t allocate enough memory\n\x00" as *const u8 as
                *const std::os::raw::c_char, progName);
    showFileNames();
    cleanUpAndFail(1 as std::os::raw::c_int);
}
/*---------------------------------------------*/
unsafe extern "C" fn configError() -> ! {
    fprintf(__stderrp,
            b"bzip2: I\'m not configured correctly for this platform!\n\tI require Int32, Int16 and Char to have sizes\n\tof 4, 2 and 1 bytes to run properly, and they don\'t.\n\tProbably you can fix this by defining them correctly,\n\tand recompiling.  Bye!\n\x00"
                as *const u8 as *const std::os::raw::c_char);
    setExit(3 as std::os::raw::c_int);
    exit(exitValue);
}
/*---------------------------------------------------*/
/*--- The main driver machinery                   ---*/
/*---------------------------------------------------*/
/* All rather crufty.  The main problem is that input files
   are stat()d multiple times before use.  This should be
   cleaned up. 
*/
/*---------------------------------------------*/
unsafe extern "C" fn pad(mut s: *mut Char) {
    let mut i: Int32 = 0;
    if strlen(s) as Int32 >= longestFileName { return }
    i = 1 as std::os::raw::c_int;
    while i <= longestFileName - strlen(s) as Int32 {
        fprintf(__stderrp, b" \x00" as *const u8 as *const std::os::raw::c_char);
        i += 1
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn copyFileName(mut to: *mut Char, mut from: *mut Char) {
    if strlen(from) >
           (1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as std::os::raw::c_ulong {
        fprintf(__stderrp,
                b"bzip2: file name\n`%s\'\nis suspiciously (more than %d chars) long.\nTry using a reasonable file name instead.  Sorry! :-)\n\x00"
                    as *const u8 as *const std::os::raw::c_char, from,
                1034 as std::os::raw::c_int - 10 as std::os::raw::c_int);
        setExit(1 as std::os::raw::c_int);
        exit(exitValue);
    }
    strncpy(to, from,
            (1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as std::os::raw::c_ulong);
    *to.offset((1034 as std::os::raw::c_int - 10 as std::os::raw::c_int) as isize) =
        '\u{0}' as i32 as Char;
}
/*---------------------------------------------*/
unsafe extern "C" fn fileExists(mut name: *mut Char) -> Bool {
    let mut tmp: *mut FILE =
        fopen(name, b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    let mut exists: Bool =
        (tmp != 0 as *mut std::os::raw::c_void as *mut FILE) as std::os::raw::c_int as Bool;
    if !tmp.is_null() { fclose(tmp); }
    return exists;
}
/*---------------------------------------------*/
/* Open an output file safely with O_EXCL and good permissions.
   This avoids a race condition in versions < 1.0.2, in which
   the file was first opened and then had its interim permissions
   set safely.  We instead use open() to create the file with
   the interim permissions required. (--- --- rw-).

   For non-Unix platforms, if we are not worrying about
   security issues, simple this simply behaves like fopen.
*/
unsafe extern "C" fn fopen_output_safely(mut name: *mut Char,
                                         mut mode: *const std::os::raw::c_char)
 -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fh: IntNative = 0;
    fh =
        open(name,
             0x1 as std::os::raw::c_int | 0x200 as std::os::raw::c_int | 0x800 as std::os::raw::c_int,
             0o200 as std::os::raw::c_int | 0o400 as std::os::raw::c_int);
    if fh == -(1 as std::os::raw::c_int) { return 0 as *mut FILE }
    fp = fdopen(fh, mode);
    if fp.is_null() { close(fh); }
    return fp;
}
/*---------------------------------------------*/
/*--
  if in doubt, return True
--*/
unsafe extern "C" fn notAStandardFile(mut name: *mut Char) -> Bool {
    let mut i: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    i = lstat(name, &mut statBuf);
    if i != 0 as std::os::raw::c_int { return 1 as std::os::raw::c_int as Bool }
    if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
           0o100000 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as Bool
    }
    return 1 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
/*--
  rac 11/21/98 see if file has hard links to it
--*/
unsafe extern "C" fn countHardLinks(mut name: *mut Char) -> Int32 {
    let mut i: IntNative = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    i = lstat(name, &mut statBuf);
    if i != 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    return statBuf.st_nlink as std::os::raw::c_int - 1 as std::os::raw::c_int;
}
/*---------------------------------------------*/
/* Copy modification date, access date, permissions and owner from the
   source to destination file.  We have to copy this meta-info off
   into fileMetaInfo before starting to compress / decompress it,
   because doing it afterwards means we get the wrong access time.

   To complicate matters, in compress() and decompress() below, the
   sequence of tests preceding the call to saveInputFileMetaInfo()
   involves calling fileExists(), which in turn establishes its result
   by attempting to fopen() the file, and if successful, immediately
   fclose()ing it again.  So we have to assume that the fopen() call
   does not cause the access time field to be updated.

   Reading of the man page for stat() (man 2 stat) on RedHat 7.2 seems
   to imply that merely doing open() will not affect the access time.
   Therefore we merely need to hope that the C library only does
   open() as a result of fopen(), and not any kind of read()-ahead
   cleverness.

   It sounds pretty fragile to me.  Whether this carries across
   robustly to arbitrary Unix-like platforms (or even works robustly
   on this one, RedHat 7.2) is unknown to me.  Nevertheless ...  
*/
static mut fileMetaInfo: stat =
    stat{st_dev: 0,
         st_mode: 0,
         st_nlink: 0,
         st_ino: 0,
         st_uid: 0,
         st_gid: 0,
         st_rdev: 0,
         st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
         st_size: 0,
         st_blocks: 0,
         st_blksize: 0,
         st_flags: 0,
         st_gen: 0,
         st_lspare: 0,
         st_qspare: [0; 2],};
unsafe extern "C" fn saveInputFileMetaInfo(mut srcName: *mut Char) {
    let mut retVal: IntNative = 0;
    /* Note use of stat here, not lstat. */
    retVal = stat(srcName, &mut fileMetaInfo);
    if retVal != 0 as std::os::raw::c_int { ioError(); };
}
unsafe extern "C" fn applySavedTimeInfoToOutputFile(mut dstName: *mut Char) {
    let mut retVal: IntNative = 0;
    let mut uTimBuf: utimbuf = utimbuf{actime: 0, modtime: 0,};
    uTimBuf.actime = fileMetaInfo.st_atimespec.tv_sec;
    uTimBuf.modtime = fileMetaInfo.st_mtimespec.tv_sec;
    retVal = utime(dstName, &mut uTimBuf);
    if retVal != 0 as std::os::raw::c_int { ioError(); };
}
unsafe extern "C" fn applySavedFileAttrToOutputFile(mut fd: IntNative) {
    let mut retVal: IntNative = 0;
    retVal = fchmod(fd, fileMetaInfo.st_mode);
    if retVal != 0 as std::os::raw::c_int { ioError(); }
    fchown(fd, fileMetaInfo.st_uid, fileMetaInfo.st_gid);
    /* chown() will in many cases return with EPERM, which can
      be safely ignored.
   */
}
/*---------------------------------------------*/
unsafe extern "C" fn containsDubiousChars(mut name: *mut Char) -> Bool {
    /* On unix, files can contain any characters and the file expansion
    * is performed by the shell.
    */
    return 0 as std::os::raw::c_int as Bool;
    /* ! BZ_UNIX */
    /* BZ_UNIX */
}
#[no_mangle]
pub static mut zSuffix: [*const Char; 4] =
    [b".bz2\x00" as *const u8 as *const std::os::raw::c_char,
     b".bz\x00" as *const u8 as *const std::os::raw::c_char,
     b".tbz2\x00" as *const u8 as *const std::os::raw::c_char,
     b".tbz\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub static mut unzSuffix: [*const Char; 4] =
    [b"\x00" as *const u8 as *const std::os::raw::c_char,
     b"\x00" as *const u8 as *const std::os::raw::c_char,
     b".tar\x00" as *const u8 as *const std::os::raw::c_char,
     b".tar\x00" as *const u8 as *const std::os::raw::c_char];
unsafe extern "C" fn hasSuffix(mut s: *mut Char, mut suffix: *const Char)
 -> Bool {
    let mut ns: Int32 = strlen(s) as Int32;
    let mut nx: Int32 = strlen(suffix) as Int32;
    if ns < nx { return 0 as std::os::raw::c_int as Bool }
    if strcmp(s.offset(ns as isize).offset(-(nx as isize)), suffix) ==
           0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int as Bool
    }
    return 0 as std::os::raw::c_int as Bool;
}
unsafe extern "C" fn mapSuffix(mut name: *mut Char,
                               mut oldSuffix: *const Char,
                               mut newSuffix: *const Char) -> Bool {
    if hasSuffix(name, oldSuffix) == 0 { return 0 as std::os::raw::c_int as Bool }
    *name.offset(strlen(name).wrapping_sub(strlen(oldSuffix)) as isize) =
        0 as std::os::raw::c_int as Char;
    strcat(name, newSuffix);
    return 1 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
unsafe extern "C" fn compress(mut name: *mut Char) {
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic(b"compress: bad modes\n\x00" as *const u8 as
                  *const std::os::raw::c_char);
    }
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            strcat(outName.as_mut_ptr(),
                   b".bz2\x00" as *const u8 as *const std::os::raw::c_char);
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input file %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1 as std::os::raw::c_int);
        return
    }
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        if hasSuffix(inName.as_mut_ptr(), zSuffix[i as usize]) != 0 {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"%s: Input file %s already has %s suffix.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), zSuffix[i as usize]);
            }
            setExit(1 as std::os::raw::c_int);
            return
        }
        i += 1
    }
    if srcMode == 3 as std::os::raw::c_int || srcMode == 2 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           notAStandardFile(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Input file %s is not a normal file.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int &&
           fileExists(outName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(__stderrp,
                    b"%s: Output file %s already exists.\n\x00" as *const u8
                        as *const std::os::raw::c_char, progName,
                    outName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           { n = countHardLinks(inName.as_mut_ptr()); (n) > 0 as std::os::raw::c_int }
       {
        fprintf(__stderrp,
                b"%s: Input file %s has %d other link%s.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName, inName.as_mut_ptr(), n,
                if n > 1 as std::os::raw::c_int {
                    b"s\x00" as *const u8 as *const std::os::raw::c_char
                } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int {
        /* Save the file's meta-info before we open it.  Doing it later
         means we mess up the access times. */
        saveInputFileMetaInfo(inName.as_mut_ptr());
    }
    match srcMode {
        1 => {
            inStr = __stdinp;
            outStr = __stdoutp;
            if isatty(fileno(__stdoutp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t write compressed data to a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        2 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr = __stdoutp;
            if isatty(fileno(__stdoutp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t write compressed data to a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr =
                fopen_output_safely(outName.as_mut_ptr(),
                                    b"wb\x00" as *const u8 as
                                        *const std::os::raw::c_char);
            if outStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t create output file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !outStr.is_null() { fclose(outStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        _ => {
            panic(b"compress: bad srcMode\x00" as *const u8 as
                      *const std::os::raw::c_char);
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(inName.as_mut_ptr());
        fflush(__stderrp);
    }
    /*--- Now the input and output handles are sane.  Do the Biz. ---*/
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as std::os::raw::c_int as Bool;
    compressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    /*--- If there was an I/O error, we won't get here. ---*/
    if srcMode == 3 as std::os::raw::c_int {
        applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
        deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
        if keepInputFiles == 0 {
            let mut retVal: IntNative = remove(inName.as_mut_ptr());
            if retVal != 0 as std::os::raw::c_int { ioError(); }
        }
    }
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
}
/*---------------------------------------------*/
unsafe extern "C" fn uncompress(mut name: *mut Char) {
    let mut current_block: u64;
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut outStr: *mut FILE = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut magicNumberOK: Bool = 0;
    let mut cantGuess: Bool = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic(b"uncompress: bad modes\n\x00" as *const u8 as
                  *const std::os::raw::c_char);
    }
    cantGuess = 0 as std::os::raw::c_int as Bool;
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            i = 0 as std::os::raw::c_int;
            loop  {
                if !(i < 4 as std::os::raw::c_int) {
                    current_block = 17860125682698302841;
                    break ;
                }
                if mapSuffix(outName.as_mut_ptr(), zSuffix[i as usize],
                             unzSuffix[i as usize]) != 0 {
                    current_block = 15314513098708193206;
                    break ;
                }
                i += 1
            }
            match current_block {
                15314513098708193206 => { }
                _ => {
                    cantGuess = 1 as std::os::raw::c_int as Bool;
                    strcat(outName.as_mut_ptr(),
                           b".out\x00" as *const u8 as *const std::os::raw::c_char);
                }
            }
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(),
                         b"(stdout)\x00" as *const u8 as *const std::os::raw::c_char
                             as *mut Char);
        }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input file %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int || srcMode == 2 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           notAStandardFile(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Input file %s is not a normal file.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if cantGuess != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: Can\'t guess original name for %s -- using %s\n\x00"
                        as *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr(), outName.as_mut_ptr());
        }
        /* just a warning, no return */
    }
    if srcMode == 3 as std::os::raw::c_int &&
           fileExists(outName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(__stderrp,
                    b"%s: Output file %s already exists.\n\x00" as *const u8
                        as *const std::os::raw::c_char, progName,
                    outName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    if srcMode == 3 as std::os::raw::c_int && forceOverwrite == 0 &&
           { n = countHardLinks(inName.as_mut_ptr()); (n) > 0 as std::os::raw::c_int }
       {
        fprintf(__stderrp,
                b"%s: Input file %s has %d other link%s.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName, inName.as_mut_ptr(), n,
                if n > 1 as std::os::raw::c_int {
                    b"s\x00" as *const u8 as *const std::os::raw::c_char
                } else { b"\x00" as *const u8 as *const std::os::raw::c_char });
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode == 3 as std::os::raw::c_int {
        /* Save the file's meta-info before we open it.  Doing it later
         means we mess up the access times. */
        saveInputFileMetaInfo(inName.as_mut_ptr());
    }
    match srcMode {
        1 => {
            inStr = __stdinp;
            outStr = __stdoutp;
            if isatty(fileno(__stdinp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t read compressed data from a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        2 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr = __stdoutp;
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s:%s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            outStr =
                fopen_output_safely(outName.as_mut_ptr(),
                                    b"wb\x00" as *const u8 as
                                        *const std::os::raw::c_char);
            if outStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t create output file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        outName.as_mut_ptr(), strerror(*__error()));
                if !inStr.is_null() { fclose(inStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s: %s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                if !outStr.is_null() { fclose(outStr); }
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        _ => {
            panic(b"uncompress: bad srcMode\x00" as *const u8 as
                      *const std::os::raw::c_char);
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(inName.as_mut_ptr());
        fflush(__stderrp);
    }
    /*--- Now the input and output handles are sane.  Do the Biz. ---*/
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as std::os::raw::c_int as Bool;
    magicNumberOK = uncompressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    /*--- If there was an I/O error, we won't get here. ---*/
    if magicNumberOK != 0 {
        if srcMode == 3 as std::os::raw::c_int {
            applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
            deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
            if keepInputFiles == 0 {
                let mut retVal: IntNative = remove(inName.as_mut_ptr());
                if retVal != 0 as std::os::raw::c_int { ioError(); }
            }
        }
    } else {
        unzFailsExist = 1 as std::os::raw::c_int as Bool;
        deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
        if srcMode == 3 as std::os::raw::c_int {
            let mut retVal_0: IntNative = remove(outName.as_mut_ptr());
            if retVal_0 != 0 as std::os::raw::c_int { ioError(); }
        }
    }
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if magicNumberOK != 0 {
        if verbosity >= 1 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"done\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
    } else {
        setExit(2 as std::os::raw::c_int);
        if verbosity >= 1 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"not a bzip2 file.\n\x00" as *const u8 as
                        *const std::os::raw::c_char);
        } else {
            fprintf(__stderrp,
                    b"%s: %s is not a bzip2 file.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
        }
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn testf(mut name: *mut Char) {
    let mut inStr: *mut FILE = 0 as *mut FILE;
    let mut allOK: Bool = 0;
    let mut statBuf: stat =
        stat{st_dev: 0,
             st_mode: 0,
             st_nlink: 0,
             st_ino: 0,
             st_uid: 0,
             st_gid: 0,
             st_rdev: 0,
             st_atimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_birthtimespec: timespec{tv_sec: 0, tv_nsec: 0,},
             st_size: 0,
             st_blocks: 0,
             st_blksize: 0,
             st_flags: 0,
             st_gen: 0,
             st_lspare: 0,
             st_qspare: [0; 2],};
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    if name.is_null() && srcMode != 1 as std::os::raw::c_int {
        panic(b"testf: bad modes\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    copyFileName(outName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    match srcMode {
        1 => {
            copyFileName(inName.as_mut_ptr(),
                         b"(stdin)\x00" as *const u8 as *const std::os::raw::c_char as
                             *mut Char);
        }
        3 => { copyFileName(inName.as_mut_ptr(), name); }
        2 => { copyFileName(inName.as_mut_ptr(), name); }
        _ => { }
    }
    if srcMode != 1 as std::os::raw::c_int &&
           containsDubiousChars(inName.as_mut_ptr()) as std::os::raw::c_int != 0 {
        if noisy != 0 {
            fprintf(__stderrp,
                    b"%s: There are no files matching `%s\'.\n\x00" as
                        *const u8 as *const std::os::raw::c_char, progName,
                    inName.as_mut_ptr());
        }
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(__stderrp,
                b"%s: Can\'t open input %s: %s.\n\x00" as *const u8 as
                    *const std::os::raw::c_char, progName, inName.as_mut_ptr(),
                strerror(*__error()));
        setExit(1 as std::os::raw::c_int);
        return
    }
    if srcMode != 1 as std::os::raw::c_int {
        stat(inName.as_mut_ptr(), &mut statBuf);
        if statBuf.st_mode as std::os::raw::c_int & 0o170000 as std::os::raw::c_int ==
               0o40000 as std::os::raw::c_int {
            fprintf(__stderrp,
                    b"%s: Input file %s is a directory.\n\x00" as *const u8 as
                        *const std::os::raw::c_char, progName, inName.as_mut_ptr());
            setExit(1 as std::os::raw::c_int);
            return
        }
    }
    match srcMode {
        1 => {
            if isatty(fileno(__stdinp)) != 0 {
                fprintf(__stderrp,
                        b"%s: I won\'t read compressed data from a terminal.\n\x00"
                            as *const u8 as *const std::os::raw::c_char, progName);
                fprintf(__stderrp,
                        b"%s: For help, type: `%s --help\'.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        progName);
                setExit(1 as std::os::raw::c_int);
                return
            }
            inStr = __stdinp
        }
        2 | 3 => {
            inStr =
                fopen(inName.as_mut_ptr(),
                      b"rb\x00" as *const u8 as *const std::os::raw::c_char);
            if inStr.is_null() {
                fprintf(__stderrp,
                        b"%s: Can\'t open input file %s:%s.\n\x00" as
                            *const u8 as *const std::os::raw::c_char, progName,
                        inName.as_mut_ptr(), strerror(*__error()));
                setExit(1 as std::os::raw::c_int);
                return
            }
        }
        _ => {
            panic(b"testf: bad srcMode\x00" as *const u8 as
                      *const std::os::raw::c_char);
        }
    }
    if verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"  %s: \x00" as *const u8 as *const std::os::raw::c_char,
                inName.as_mut_ptr());
        pad(inName.as_mut_ptr());
        fflush(__stderrp);
    }
    /*--- Now the input handle is sane.  Do the Biz. ---*/
    outputHandleJustInCase = 0 as *mut FILE;
    allOK = testStream(inStr);
    if allOK as std::os::raw::c_int != 0 && verbosity >= 1 as std::os::raw::c_int {
        fprintf(__stderrp, b"ok\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    if allOK == 0 { testFailsExist = 1 as std::os::raw::c_int as Bool };
}
/*---------------------------------------------*/
unsafe extern "C" fn license() {
    fprintf(__stderrp,
            b"bzip2, a block-sorting file compressor.  Version %s.\n   \n   Copyright (C) 1996-2019 by Julian Seward.\n   \n   This program is free software; you can redistribute it and/or modify\n   it under the terms set out in the LICENSE file, which is included\n   in the bzip2 source distribution.\n   \n   This program is distributed in the hope that it will be useful,\n   but WITHOUT ANY WARRANTY; without even the implied warranty of\n   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n   LICENSE file for more details.\n   \n\x00"
                as *const u8 as *const std::os::raw::c_char, BZ2_bzlibVersion());
}
/*---------------------------------------------*/
unsafe extern "C" fn usage(mut fullProgName: *mut Char) {
    fprintf(__stderrp,
            b"bzip2, a block-sorting file compressor.  Version %s.\n\n   usage: %s [flags and input files in any order]\n\n   -h --help           print this message\n   -d --decompress     force decompression\n   -z --compress       force compression\n   -k --keep           keep (don\'t delete) input files\n   -f --force          overwrite existing output files\n   -t --test           test compressed file integrity\n   -c --stdout         output to standard out\n   -q --quiet          suppress noncritical error messages\n   -v --verbose        be verbose (a 2nd -v gives more)\n   -L --license        display software version & license\n   -V --version        display software version & license\n   -s --small          use less memory (at most 2500k)\n   -1 .. -9            set block size to 100k .. 900k\n   --fast              alias for -1\n   --best              alias for -9\n\n   If invoked as `bzip2\', default action is to compress.\n              as `bunzip2\',  default action is to decompress.\n              as `bzcat\', default action is to decompress to stdout.\n\n   If no file names are given, bzip2 compresses or decompresses\n   from standard input to standard output.  You can combine\n   short flags, so `-v -4\' means the same as -v4 or -4v, &c.\n\n\x00"
                as *const u8 as *const std::os::raw::c_char, BZ2_bzlibVersion(),
            fullProgName);
}
/*---------------------------------------------*/
unsafe extern "C" fn redundant(mut flag: *mut Char) {
    fprintf(__stderrp,
            b"%s: %s is redundant in versions 0.9.5 and above\n\x00" as
                *const u8 as *const std::os::raw::c_char, progName, flag);
}
/*---------------------------------------------*/
unsafe extern "C" fn myMalloc(mut n: Int32) -> *mut std::os::raw::c_void {
    let mut p: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    p = malloc(n as size_t);
    if p.is_null() { outOfMemory(); }
    return p;
}
/*---------------------------------------------*/
unsafe extern "C" fn mkCell() -> *mut Cell {
    let mut c: *mut Cell = 0 as *mut Cell;
    c =
        myMalloc(::std::mem::size_of::<Cell>() as std::os::raw::c_ulong as Int32) as
            *mut Cell;
    (*c).name = 0 as *mut Char;
    (*c).link = 0 as *mut zzzz;
    return c;
}
/*---------------------------------------------*/
unsafe extern "C" fn snocString(mut root: *mut Cell, mut name: *mut Char)
 -> *mut Cell {
    if root.is_null() {
        let mut tmp: *mut Cell = mkCell();
        (*tmp).name =
            myMalloc((5 as std::os::raw::c_int as
                          std::os::raw::c_ulong).wrapping_add(strlen(name)) as Int32)
                as *mut Char;
        strcpy((*tmp).name, name);
        return tmp
    } else {
        let mut tmp_0: *mut Cell = root;
        while !(*tmp_0).link.is_null() { tmp_0 = (*tmp_0).link }
        (*tmp_0).link = snocString((*tmp_0).link, name);
        return root
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn addFlagsFromEnvVar(mut argList: *mut *mut Cell,
                                        mut varName: *mut Char) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut envbase: *mut Char = 0 as *mut Char;
    let mut p: *mut Char = 0 as *mut Char;
    envbase = getenv(varName);
    if !envbase.is_null() {
        p = envbase;
        i = 0 as std::os::raw::c_int;
        while 1 as std::os::raw::c_int as Bool != 0 {
            if *p.offset(i as isize) as std::os::raw::c_int == 0 as std::os::raw::c_int {
                break ;
            }
            p = p.offset(i as isize);
            i = 0 as std::os::raw::c_int;
            while isspace(*p.offset(0 as std::os::raw::c_int as isize) as Int32) != 0
                  {
                p = p.offset(1)
            }
            while *p.offset(i as isize) as std::os::raw::c_int != 0 as std::os::raw::c_int &&
                      isspace(*p.offset(i as isize) as Int32) == 0 {
                i += 1
            }
            if i > 0 as std::os::raw::c_int {
                k = i;
                if k > 1034 as std::os::raw::c_int - 10 as std::os::raw::c_int {
                    k = 1034 as std::os::raw::c_int - 10 as std::os::raw::c_int
                }
                j = 0 as std::os::raw::c_int;
                while j < k {
                    tmpName[j as usize] = *p.offset(j as isize);
                    j += 1
                }
                tmpName[k as usize] = 0 as std::os::raw::c_int as Char;
                *argList = snocString(*argList, tmpName.as_mut_ptr())
            }
        }
    };
}
unsafe fn main_0(mut argc: IntNative, mut argv: *mut *mut Char) -> IntNative {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp: *mut Char = 0 as *mut Char;
    let mut argList: *mut Cell = 0 as *mut Cell;
    let mut aa: *mut Cell = 0 as *mut Cell;
    let mut decode: Bool = 0;
    /*-- Be really really really paranoid :-) --*/
    if ::std::mem::size_of::<Int32>() as std::os::raw::c_ulong !=
           4 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UInt32>() as std::os::raw::c_ulong !=
               4 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<Int16>() as std::os::raw::c_ulong !=
               2 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UInt16>() as std::os::raw::c_ulong !=
               2 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<Char>() as std::os::raw::c_ulong !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong ||
           ::std::mem::size_of::<UChar>() as std::os::raw::c_ulong !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong {
        configError();
    }
    /*-- Initialise --*/
    outputHandleJustInCase =
        0 as *mut FILE; /* avoid bogus warning from egcs-1.1.X */
    smallMode = 0 as std::os::raw::c_int as Bool;
    keepInputFiles = 0 as std::os::raw::c_int as Bool;
    forceOverwrite = 0 as std::os::raw::c_int as Bool;
    noisy = 1 as std::os::raw::c_int as Bool;
    verbosity = 0 as std::os::raw::c_int;
    blockSize100k = 9 as std::os::raw::c_int;
    testFailsExist = 0 as std::os::raw::c_int as Bool;
    unzFailsExist = 0 as std::os::raw::c_int as Bool;
    numFileNames = 0 as std::os::raw::c_int;
    numFilesProcessed = 0 as std::os::raw::c_int;
    workFactor = 30 as std::os::raw::c_int;
    deleteOutputOnInterrupt = 0 as std::os::raw::c_int as Bool;
    exitValue = 0 as std::os::raw::c_int;
    j = 0 as std::os::raw::c_int;
    i = j;
    /*-- Set up signal handlers for mem access errors --*/
    signal(11 as std::os::raw::c_int,
           Some(mySIGSEGVorSIGBUScatcher as
                    unsafe extern "C" fn(_: IntNative) -> ()));
    signal(10 as std::os::raw::c_int,
           Some(mySIGSEGVorSIGBUScatcher as
                    unsafe extern "C" fn(_: IntNative) -> ()));
    copyFileName(inName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    copyFileName(outName.as_mut_ptr(),
                 b"(none)\x00" as *const u8 as *const std::os::raw::c_char as
                     *mut Char);
    copyFileName(progNameReally.as_mut_ptr(),
                 *argv.offset(0 as std::os::raw::c_int as isize));
    progName =
        &mut *progNameReally.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
            *mut Char;
    tmp =
        &mut *progNameReally.as_mut_ptr().offset(0 as std::os::raw::c_int as isize) as
            *mut Char;
    while *tmp as std::os::raw::c_int != '\u{0}' as i32 {
        if *tmp as std::os::raw::c_int == '/' as i32 {
            progName = tmp.offset(1 as std::os::raw::c_int as isize)
        }
        tmp = tmp.offset(1)
    }
    /*-- Copy flags from env var BZIP2, and 
        expand filename wildcards in arg list.
   --*/
    argList = 0 as *mut Cell;
    addFlagsFromEnvVar(&mut argList,
                       b"BZIP2\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut Char);
    addFlagsFromEnvVar(&mut argList,
                       b"BZIP\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut Char);
    i = 1 as std::os::raw::c_int;
    while i <= argc - 1 as std::os::raw::c_int {
        argList = snocString(argList, *argv.offset(i as isize));
        i += 1
    }
    /*-- Find the length of the longest filename --*/
    longestFileName = 7 as std::os::raw::c_int;
    numFileNames = 0 as std::os::raw::c_int;
    decode = 1 as std::os::raw::c_int as Bool;
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            decode = 0 as std::os::raw::c_int as Bool
        } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_int == '-' as i32 &&
                        decode as std::os::raw::c_int != 0) {
            numFileNames += 1;
            if longestFileName < strlen((*aa).name) as Int32 {
                longestFileName = strlen((*aa).name) as Int32
            }
        }
        aa = (*aa).link
    }
    /*-- Determine source modes; flag handling may change this too. --*/
    if numFileNames == 0 as std::os::raw::c_int {
        srcMode = 1 as std::os::raw::c_int
    } else { srcMode = 3 as std::os::raw::c_int }
    /*-- Determine what to do (compress/uncompress/test/cat). --*/
   /*-- Note that subsequent flag handling may change this. --*/
    opMode = 1 as std::os::raw::c_int;
    if !strstr(progName,
               b"unzip\x00" as *const u8 as *const std::os::raw::c_char).is_null() ||
           !strstr(progName,
                   b"UNZIP\x00" as *const u8 as *const std::os::raw::c_char).is_null()
       {
        opMode = 2 as std::os::raw::c_int
    }
    if !strstr(progName,
               b"z2cat\x00" as *const u8 as *const std::os::raw::c_char).is_null() ||
           !strstr(progName,
                   b"Z2CAT\x00" as *const u8 as *const std::os::raw::c_char).is_null()
           ||
           !strstr(progName,
                   b"zcat\x00" as *const u8 as *const std::os::raw::c_char).is_null()
           ||
           !strstr(progName,
                   b"ZCAT\x00" as *const u8 as *const std::os::raw::c_char).is_null()
       {
        opMode = 2 as std::os::raw::c_int;
        srcMode =
            if numFileNames == 0 as std::os::raw::c_int {
                1 as std::os::raw::c_int
            } else { 2 as std::os::raw::c_int }
    }
    /*-- Look at the flags. --*/
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            break ;
        }
        if *(*aa).name.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '-' as i32 &&
               *(*aa).name.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                   '-' as i32 {
            j = 1 as std::os::raw::c_int;
            while *(*aa).name.offset(j as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
                match *(*aa).name.offset(j as isize) as std::os::raw::c_int {
                    99 => { srcMode = 2 as std::os::raw::c_int }
                    100 => { opMode = 2 as std::os::raw::c_int }
                    122 => { opMode = 1 as std::os::raw::c_int }
                    102 => { forceOverwrite = 1 as std::os::raw::c_int as Bool }
                    116 => { opMode = 3 as std::os::raw::c_int }
                    107 => { keepInputFiles = 1 as std::os::raw::c_int as Bool }
                    115 => { smallMode = 1 as std::os::raw::c_int as Bool }
                    113 => { noisy = 0 as std::os::raw::c_int as Bool }
                    49 => { blockSize100k = 1 as std::os::raw::c_int }
                    50 => { blockSize100k = 2 as std::os::raw::c_int }
                    51 => { blockSize100k = 3 as std::os::raw::c_int }
                    52 => { blockSize100k = 4 as std::os::raw::c_int }
                    53 => { blockSize100k = 5 as std::os::raw::c_int }
                    54 => { blockSize100k = 6 as std::os::raw::c_int }
                    55 => { blockSize100k = 7 as std::os::raw::c_int }
                    56 => { blockSize100k = 8 as std::os::raw::c_int }
                    57 => { blockSize100k = 9 as std::os::raw::c_int }
                    86 | 76 => { license(); }
                    118 => { verbosity += 1 }
                    104 => { usage(progName); exit(0 as std::os::raw::c_int); }
                    _ => {
                        fprintf(__stderrp,
                                b"%s: Bad flag `%s\'\n\x00" as *const u8 as
                                    *const std::os::raw::c_char, progName,
                                (*aa).name);
                        usage(progName);
                        exit(1 as std::os::raw::c_int);
                    }
                }
                j += 1
            }
        }
        aa = (*aa).link
    }
    /*-- And again ... --*/
    aa = argList;
    while !aa.is_null() {
        if strcmp((*aa).name, b"--\x00" as *const u8 as *const std::os::raw::c_char)
               == 0 as std::os::raw::c_int {
            break ;
        }
        if strcmp((*aa).name,
                  b"--stdout\x00" as *const u8 as *const std::os::raw::c_char) ==
               0 as std::os::raw::c_int {
            srcMode = 2 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--decompress\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            opMode = 2 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--compress\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            opMode = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--force\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            forceOverwrite = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--test\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            opMode = 3 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--keep\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            keepInputFiles = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--small\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            smallMode = 1 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--quiet\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            noisy = 0 as std::os::raw::c_int as Bool
        } else if strcmp((*aa).name,
                         b"--version\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            license();
        } else if strcmp((*aa).name,
                         b"--license\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            license();
        } else if strcmp((*aa).name,
                         b"--exponential\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            workFactor = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--repetitive-best\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            redundant((*aa).name);
        } else if strcmp((*aa).name,
                         b"--repetitive-fast\x00" as *const u8 as
                             *const std::os::raw::c_char) == 0 as std::os::raw::c_int {
            redundant((*aa).name);
        } else if strcmp((*aa).name,
                         b"--fast\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            blockSize100k = 1 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--best\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            blockSize100k = 9 as std::os::raw::c_int
        } else if strcmp((*aa).name,
                         b"--verbose\x00" as *const u8 as *const std::os::raw::c_char)
                      == 0 as std::os::raw::c_int {
            verbosity += 1
        } else if strcmp((*aa).name,
                         b"--help\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            usage(progName);
            exit(0 as std::os::raw::c_int);
        } else {
            if strncmp((*aa).name,
                       b"--\x00" as *const u8 as *const std::os::raw::c_char,
                       2 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int
               {
                fprintf(__stderrp,
                        b"%s: Bad flag `%s\'\n\x00" as *const u8 as
                            *const std::os::raw::c_char, progName, (*aa).name);
                usage(progName);
                exit(1 as std::os::raw::c_int);
            }
        }
        aa = (*aa).link
    }
    if verbosity > 4 as std::os::raw::c_int { verbosity = 4 as std::os::raw::c_int }
    if opMode == 1 as std::os::raw::c_int && smallMode as std::os::raw::c_int != 0 &&
           blockSize100k > 2 as std::os::raw::c_int {
        blockSize100k = 2 as std::os::raw::c_int
    }
    if opMode == 3 as std::os::raw::c_int && srcMode == 2 as std::os::raw::c_int {
        fprintf(__stderrp,
                b"%s: -c and -t cannot be used together.\n\x00" as *const u8
                    as *const std::os::raw::c_char, progName);
        exit(1 as std::os::raw::c_int);
    }
    if srcMode == 2 as std::os::raw::c_int && numFileNames == 0 as std::os::raw::c_int {
        srcMode = 1 as std::os::raw::c_int
    }
    if opMode != 1 as std::os::raw::c_int { blockSize100k = 0 as std::os::raw::c_int }
    if srcMode == 3 as std::os::raw::c_int {
        signal(2 as std::os::raw::c_int,
               Some(mySignalCatcher as
                        unsafe extern "C" fn(_: IntNative) -> ()));
        signal(15 as std::os::raw::c_int,
               Some(mySignalCatcher as
                        unsafe extern "C" fn(_: IntNative) -> ()));
        signal(1 as std::os::raw::c_int,
               Some(mySignalCatcher as
                        unsafe extern "C" fn(_: IntNative) -> ()));
    }
    if opMode == 1 as std::os::raw::c_int {
        if srcMode == 1 as std::os::raw::c_int {
            compress(0 as *mut Char);
        } else {
            decode = 1 as std::os::raw::c_int as Bool;
            aa = argList;
            while !aa.is_null() {
                if strcmp((*aa).name,
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    decode = 0 as std::os::raw::c_int as Bool
                } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int == '-' as i32 &&
                                decode as std::os::raw::c_int != 0) {
                    numFilesProcessed += 1;
                    compress((*aa).name);
                }
                aa = (*aa).link
            }
        }
    } else if opMode == 2 as std::os::raw::c_int {
        unzFailsExist = 0 as std::os::raw::c_int as Bool;
        if srcMode == 1 as std::os::raw::c_int {
            uncompress(0 as *mut Char);
        } else {
            decode = 1 as std::os::raw::c_int as Bool;
            aa = argList;
            while !aa.is_null() {
                if strcmp((*aa).name,
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    decode = 0 as std::os::raw::c_int as Bool
                } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int == '-' as i32 &&
                                decode as std::os::raw::c_int != 0) {
                    numFilesProcessed += 1;
                    uncompress((*aa).name);
                }
                aa = (*aa).link
            }
        }
        if unzFailsExist != 0 { setExit(2 as std::os::raw::c_int); exit(exitValue); }
    } else {
        testFailsExist = 0 as std::os::raw::c_int as Bool;
        if srcMode == 1 as std::os::raw::c_int {
            testf(0 as *mut Char);
        } else {
            decode = 1 as std::os::raw::c_int as Bool;
            aa = argList;
            while !aa.is_null() {
                if strcmp((*aa).name,
                          b"--\x00" as *const u8 as *const std::os::raw::c_char) ==
                       0 as std::os::raw::c_int {
                    decode = 0 as std::os::raw::c_int as Bool
                } else if !(*(*aa).name.offset(0 as std::os::raw::c_int as isize) as
                                std::os::raw::c_int == '-' as i32 &&
                                decode as std::os::raw::c_int != 0) {
                    numFilesProcessed += 1;
                    testf((*aa).name);
                }
                aa = (*aa).link
            }
        }
        if testFailsExist != 0 {
            if noisy != 0 {
                fprintf(__stderrp,
                        b"\nYou can use the `bzip2recover\' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\x00"
                            as *const u8 as *const std::os::raw::c_char);
            }
            setExit(2 as std::os::raw::c_int);
            exit(exitValue);
        }
    }
    /* Free the argument list memory to mollify leak detectors 
      (eg) Purify, Checker.  Serves no other useful purpose.
   */
    aa = argList;
    while !aa.is_null() {
        let mut aa2: *mut Cell = (*aa).link;
        if !(*aa).name.is_null() { free((*aa).name as *mut std::os::raw::c_void); }
        free(aa as *mut std::os::raw::c_void);
        aa = aa2
    }
    return exitValue;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as IntNative,
                                    args.as_mut_ptr() as *mut *mut Char) as
                                 i32)
    }
}
/*-----------------------------------------------------------*/
/*--- end                                         bzip2.c ---*/
/*-----------------------------------------------------------*/
