use ::libc;
extern "C" {

    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn rewind(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;

    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fchown(__fd: libc::c_int, __owner: __uid_t, __group: __gid_t) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0;
impl Default for ErasedByPreprocessor0 {
    fn default() -> Self {
        Self {}
    }
}

pub type _IO_lock_t = ();
pub type FILE = crate::src::blocksort::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
impl Default for timespec {
    fn default() -> Self {
        Self {
            tv_sec: Default::default(),
            tv_nsec: Default::default(),
        }
    }
}

pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
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
pub type BZFILE = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
impl Default for stat {
    fn default() -> Self {
        Self {
            st_dev: Default::default(),
            st_ino: Default::default(),
            st_nlink: Default::default(),
            st_mode: Default::default(),
            st_uid: Default::default(),
            st_gid: Default::default(),
            __pad0: Default::default(),
            st_rdev: Default::default(),
            st_size: Default::default(),
            st_blksize: Default::default(),
            st_blocks: Default::default(),
            st_atim: Default::default(),
            st_mtim: Default::default(),
            st_ctim: Default::default(),
            __glibc_reserved: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
impl Default for utimbuf {
    fn default() -> Self {
        Self {
            actime: Default::default(),
            modtime: Default::default(),
        }
    }
}

pub type Char = libc::c_char;
pub type Bool = libc::c_uchar;
pub type UChar = libc::c_uchar;
pub type Int32 = libc::c_int;
pub type UInt32 = libc::c_uint;
pub type Int16 = libc::c_short;
pub type UInt16 = libc::c_ushort;
pub type IntNative = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UInt64 {
    pub b: [UChar; 8],
}
impl Default for UInt64 {
    fn default() -> Self {
        Self {
            b: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct zzzz {
    pub name: *mut Char,
    pub link: *mut zzzz,
}
impl Default for zzzz {
    fn default() -> Self {
        Self {
            name: std::ptr::null_mut(),
            link: std::ptr::null_mut(),
        }
    }
}

pub type Cell = zzzz;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
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
pub static mut outputHandleJustInCase: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut workFactor: Int32 = 0;
unsafe extern "C" fn uInt64_from_UInt32s(
    mut n: Option<&mut UInt64>,
    mut lo32: UInt32,
    mut hi32: UInt32,
) {
    (*n.as_deref_mut().unwrap()).b[7 as libc::c_int as usize] =
        (hi32 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as UChar;
    (*n.as_deref_mut().unwrap()).b[6 as libc::c_int as usize] =
        (hi32 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as UChar;
    (*n.as_deref_mut().unwrap()).b[5 as libc::c_int as usize] =
        (hi32 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as UChar;
    (*n.as_deref_mut().unwrap()).b[4 as libc::c_int as usize] =
        (hi32 & 0xff as libc::c_int as libc::c_uint) as UChar;
    (*n.as_deref_mut().unwrap()).b[3 as libc::c_int as usize] =
        (lo32 >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as UChar;
    (*n.as_deref_mut().unwrap()).b[2 as libc::c_int as usize] =
        (lo32 >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as UChar;
    (*n.as_deref_mut().unwrap()).b[1 as libc::c_int as usize] =
        (lo32 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as UChar;
    (*n.as_deref_mut().unwrap()).b[0 as libc::c_int as usize] =
        (lo32 & 0xff as libc::c_int as libc::c_uint) as UChar;
}
unsafe extern "C" fn uInt64_to_double(mut n: *const UInt64) -> libc::c_double {
    let mut i: Int32 = 0;
    let mut base = 1.0f64;
    let mut sum = 0.0f64;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        sum += base * (*n).b[i as usize] as libc::c_double;
        base *= 256.0f64;
        i += 1;
    }
    return sum;
}
unsafe extern "C" fn uInt64_isZero(mut n: *const UInt64) -> Bool {
    let mut i: Int32 = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if (*n).b[i as usize] as libc::c_int != 0 as libc::c_int {
            return 0 as libc::c_int as Bool;
        }
        i += 1;
    }
    return 1 as libc::c_int as Bool;
}
unsafe extern "C" fn uInt64_qrm10(mut n: Option<&mut UInt64>) -> Int32 {
    let mut rem: UInt32 = 0;
    let mut tmp: UInt32 = 0;
    let mut i: Int32 = 0;
    rem = 0 as libc::c_int as UInt32;
    i = 7 as libc::c_int;
    while i >= 0 as libc::c_int {
        tmp = rem
            .wrapping_mul(256 as libc::c_int as libc::c_uint)
            .wrapping_add((*n.as_deref().unwrap()).b[i as usize] as libc::c_uint);
        (*n.as_deref_mut().unwrap()).b[i as usize] =
            tmp.wrapping_div(10 as libc::c_int as libc::c_uint) as UChar;
        rem = tmp.wrapping_rem(10 as libc::c_int as libc::c_uint);
        i -= 1;
    }
    return rem as Int32;
}
unsafe extern "C" fn uInt64_toAscii(mut outbuf: *mut libc::c_char, mut n: *const UInt64) {
    let mut i: Int32 = 0;
    let mut q: Int32 = 0;
    let mut buf: [UChar; 32] = [0; 32];
    let mut nBuf = 0 as libc::c_int;
    let mut n_copy = (*n);
    loop {
        q = uInt64_qrm10(Some(&mut n_copy));
        buf[nBuf as usize] = (q + '0' as i32) as UChar;
        nBuf += 1;
        if !(uInt64_isZero(core::ptr::addr_of!(n_copy)) == 0) {
            break;
        }
    }
    *outbuf.offset(nBuf as isize) = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while i < nBuf {
        *outbuf.offset(i as isize) = buf[(nBuf - i - 1 as libc::c_int) as usize] as libc::c_char;
        i += 1;
    }
}
unsafe extern "C" fn myfeof(mut f: *mut FILE) -> Bool {
    let mut c = fgetc(f);
    if c == -(1 as libc::c_int) {
        return 1 as libc::c_int as Bool;
    }
    ungetc(c, f);
    return 0 as libc::c_int as Bool;
}
unsafe extern "C" fn compressStream(mut stream: *mut FILE, mut zStream: *mut FILE) {
    let mut current_block: u64;
    let mut bzf = 0 as *mut libc::c_void;
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
            bzf = crate::src::bzlib::BZ2_bzWriteOpen(
                core::ptr::addr_of_mut!(bzerr),
                zStream,
                blockSize100k,
                verbosity,
                workFactor,
            );
            if bzerr != 0 as libc::c_int {
                current_block = 12087451728421612484;
            } else {
                if verbosity >= 2 as libc::c_int {
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                }
                loop {
                    if !(1 as libc::c_int as Bool != 0) {
                        current_block = 13242334135786603907;
                        break;
                    }
                    if myfeof(stream) != 0 {
                        current_block = 13242334135786603907;
                        break;
                    }
                    nIbuf = fread(
                        ibuf.as_mut_ptr() as *mut libc::c_void,
                        ::std::mem::size_of::<UChar>() as libc::c_ulong,
                        5000 as libc::c_int as libc::c_ulong,
                        stream,
                    ) as Int32;
                    if ferror(stream) != 0 {
                        current_block = 13935372548986539192;
                        break;
                    }
                    if nIbuf > 0 as libc::c_int {
                        crate::src::bzlib::BZ2_bzWrite(
                            Some(&mut bzerr),
                            bzf,
                            ibuf.as_mut_ptr() as *mut libc::c_void,
                            nIbuf,
                        );
                    }
                    if bzerr != 0 as libc::c_int {
                        current_block = 12087451728421612484;
                        break;
                    }
                }
                match current_block {
                    13935372548986539192 => {}
                    12087451728421612484 => {}
                    _ => {
                        crate::src::bzlib::BZ2_bzWriteClose64(
                            core::ptr::addr_of_mut!(bzerr),
                            bzf,
                            0 as libc::c_int,
                            core::ptr::addr_of_mut!(nbytes_in_lo32),
                            core::ptr::addr_of_mut!(nbytes_in_hi32),
                            core::ptr::addr_of_mut!(nbytes_out_lo32),
                            core::ptr::addr_of_mut!(nbytes_out_hi32),
                        );
                        if bzerr != 0 as libc::c_int {
                            current_block = 12087451728421612484;
                        } else if ferror(zStream) != 0 {
                            current_block = 13935372548986539192;
                        } else {
                            ret = fflush(zStream);
                            if ret == -(1 as libc::c_int) {
                                current_block = 13935372548986539192;
                            } else {
                                if zStream != stdout {
                                    let mut fd = fileno(zStream);
                                    if fd < 0 as libc::c_int {
                                        current_block = 13935372548986539192;
                                    } else {
                                        applySavedFileAttrToOutputFile(fd);
                                        ret = fclose(zStream);
                                        outputHandleJustInCase = 0 as *mut FILE;
                                        if ret == -(1 as libc::c_int) {
                                            current_block = 13935372548986539192;
                                        } else {
                                            current_block = 17281240262373992796;
                                        }
                                    }
                                } else {
                                    current_block = 17281240262373992796;
                                }
                                match current_block {
                                    13935372548986539192 => {}
                                    _ => {
                                        outputHandleJustInCase = 0 as *mut FILE;
                                        if ferror(stream) != 0 {
                                            current_block = 13935372548986539192;
                                        } else {
                                            ret = fclose(stream);
                                            if ret == -(1 as libc::c_int) {
                                                current_block = 13935372548986539192;
                                            } else {
                                                if verbosity >= 1 as libc::c_int {
                                                    if nbytes_in_lo32
                                                        == 0 as libc::c_int as libc::c_uint
                                                        && nbytes_in_hi32
                                                            == 0 as libc::c_int as libc::c_uint
                                                    {
                                                        fprintf(
                                                            stderr,
                                                            b" no data compressed.\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                    } else {
                                                        let mut buf_nin: [Char; 32] = [0; 32];
                                                        let mut buf_nout: [Char; 32] = [0; 32];
                                                        let mut nbytes_in = UInt64 { b: [0; 8] };
                                                        let mut nbytes_out = UInt64 { b: [0; 8] };
                                                        let mut nbytes_in_d: libc::c_double = 0.;
                                                        let mut nbytes_out_d: libc::c_double = 0.;
                                                        uInt64_from_UInt32s(
                                                            Some(&mut nbytes_in),
                                                            nbytes_in_lo32,
                                                            nbytes_in_hi32,
                                                        );
                                                        uInt64_from_UInt32s(
                                                            Some(&mut nbytes_out),
                                                            nbytes_out_lo32,
                                                            nbytes_out_hi32,
                                                        );
                                                        nbytes_in_d = uInt64_to_double(
                                                            core::ptr::addr_of!(nbytes_in),
                                                        );
                                                        nbytes_out_d = uInt64_to_double(
                                                            core::ptr::addr_of!(nbytes_out),
                                                        );
                                                        uInt64_toAscii(
                                                            buf_nin.as_mut_ptr(),
                                                            core::ptr::addr_of!(nbytes_in),
                                                        );
                                                        uInt64_toAscii(
                                                            buf_nout.as_mut_ptr(),
                                                            core::ptr::addr_of!(nbytes_out),
                                                        );
                                                        fprintf(
                                                            stderr,
                                                            b"%6.3f:1, %6.3f bits/byte, %5.2f%% saved, %s in, %s out.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            nbytes_in_d / nbytes_out_d,
                                                            8.0f64 * nbytes_out_d / nbytes_in_d,
                                                            100.0f64 * (1.0f64 - nbytes_out_d / nbytes_in_d),
                                                            buf_nin.as_mut_ptr(),
                                                            buf_nout.as_mut_ptr(),
                                                        );
                                                    }
                                                }
                                                return;
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
                13935372548986539192 => {}
                _ => {
                    crate::src::bzlib::BZ2_bzWriteClose64(
                        core::ptr::addr_of_mut!(bzerr_dummy),
                        bzf,
                        1 as libc::c_int,
                        core::ptr::addr_of_mut!(nbytes_in_lo32),
                        core::ptr::addr_of_mut!(nbytes_in_hi32),
                        core::ptr::addr_of_mut!(nbytes_out_lo32),
                        core::ptr::addr_of_mut!(nbytes_out_hi32),
                    );
                    match bzerr {
                        -9 => {
                            current_block = 14441628745024150911;
                            match current_block {
                                16727043110015018357 => {
                                    panic(
                                        b"compress:unexpected error\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                13006829858402918495 => {
                                    outOfMemory();
                                }
                                _ => {
                                    configError();
                                }
                            }
                        }
                        -3 => {
                            current_block = 13006829858402918495;
                            match current_block {
                                16727043110015018357 => {
                                    panic(
                                        b"compress:unexpected error\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                13006829858402918495 => {
                                    outOfMemory();
                                }
                                _ => {
                                    configError();
                                }
                            }
                        }
                        -6 => {}
                        _ => {
                            current_block = 16727043110015018357;
                            match current_block {
                                16727043110015018357 => {
                                    panic(
                                        b"compress:unexpected error\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                13006829858402918495 => {
                                    outOfMemory();
                                }
                                _ => {
                                    configError();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
}
unsafe extern "C" fn uncompressStream(mut zStream: *mut FILE, mut stream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf = 0 as *mut libc::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut nread: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV = 0 as *mut libc::c_void;
    let mut unusedTmp = 0 as *mut UChar;
    nUnused = 0 as libc::c_int;
    streamNo = 0 as libc::c_int;
    if !(ferror(stream) != 0) {
        if !(ferror(zStream) != 0) {
            's_51: loop {
                if !(1 as libc::c_int as Bool != 0) {
                    current_block = 7325919439520858036;
                    break;
                }
                bzf = crate::src::bzlib::BZ2_bzReadOpen(
                    core::ptr::addr_of_mut!(bzerr),
                    zStream,
                    verbosity,
                    smallMode as libc::c_int,
                    unused.as_mut_ptr() as *mut libc::c_void,
                    nUnused,
                );
                if bzf.is_null() || bzerr != 0 as libc::c_int {
                    current_block = 15314762231929378705;
                    break;
                }
                streamNo += 1;
                while bzerr == 0 as libc::c_int {
                    nread = crate::src::bzlib::BZ2_bzRead(
                        Some(&mut bzerr),
                        bzf,
                        obuf.as_mut_ptr() as *mut libc::c_void,
                        5000 as libc::c_int,
                    );
                    if bzerr == -(5 as libc::c_int) {
                        current_block = 3516197883607697062;
                        break 's_51;
                    }
                    if (bzerr == 0 as libc::c_int || bzerr == 4 as libc::c_int)
                        && nread > 0 as libc::c_int
                    {
                        fwrite(
                            obuf.as_mut_ptr() as *const libc::c_void,
                            ::std::mem::size_of::<UChar>() as libc::c_ulong,
                            nread as libc::c_ulong,
                            stream,
                        );
                    }
                    if ferror(stream) != 0 {
                        current_block = 9292489448180700977;
                        break 's_51;
                    }
                }
                if bzerr != 4 as libc::c_int {
                    current_block = 15314762231929378705;
                    break;
                }
                crate::src::bzlib::BZ2_bzReadGetUnused(
                    Some(&mut bzerr),
                    bzf,
                    Some(&mut unusedTmpV),
                    Some(&mut nUnused),
                );
                if bzerr != 0 as libc::c_int {
                    panic(b"decompress:bzReadGetUnused\0" as *const u8 as *const libc::c_char);
                }
                unusedTmp = unusedTmpV as *mut UChar;
                i = 0 as libc::c_int;
                while i < nUnused {
                    unused[i as usize] = *unusedTmp.offset(i as isize);
                    i += 1;
                }
                crate::src::bzlib::BZ2_bzReadClose(core::ptr::addr_of_mut!(bzerr), bzf);
                if bzerr != 0 as libc::c_int {
                    panic(b"decompress:bzReadGetUnused\0" as *const u8 as *const libc::c_char);
                }
                if nUnused == 0 as libc::c_int && myfeof(zStream) as libc::c_int != 0 {
                    current_block = 7325919439520858036;
                    break;
                }
            }
            match current_block {
                9292489448180700977 => {}
                _ => {
                    match current_block {
                        3516197883607697062 => {
                            if forceOverwrite != 0 {
                                rewind(zStream);
                                loop {
                                    if !(1 as libc::c_int as Bool != 0) {
                                        current_block = 7325919439520858036;
                                        break;
                                    }
                                    if myfeof(zStream) != 0 {
                                        current_block = 7325919439520858036;
                                        break;
                                    }
                                    nread = fread(
                                        obuf.as_mut_ptr() as *mut libc::c_void,
                                        ::std::mem::size_of::<UChar>() as libc::c_ulong,
                                        5000 as libc::c_int as libc::c_ulong,
                                        zStream,
                                    ) as Int32;
                                    if ferror(zStream) != 0 {
                                        current_block = 9292489448180700977;
                                        break;
                                    }
                                    if nread > 0 as libc::c_int {
                                        fwrite(
                                            obuf.as_mut_ptr() as *const libc::c_void,
                                            ::std::mem::size_of::<UChar>() as libc::c_ulong,
                                            nread as libc::c_ulong,
                                            stream,
                                        );
                                    }
                                    if ferror(stream) != 0 {
                                        current_block = 9292489448180700977;
                                        break;
                                    }
                                }
                            } else {
                                current_block = 15314762231929378705;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        9292489448180700977 => {}
                        _ => match current_block {
                            15314762231929378705 => {
                                crate::src::bzlib::BZ2_bzReadClose(
                                    core::ptr::addr_of_mut!(bzerr_dummy),
                                    bzf,
                                );
                                match bzerr {
                                    -9 => {
                                        current_block = 16334903743006538945;
                                        match current_block {
                                            3330558285168407176 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            16334903743006538945 => {
                                                configError();
                                            }
                                            2689728545616102333 => {
                                                crcError();
                                            }
                                            12943795838694691063 => {
                                                outOfMemory();
                                            }
                                            17687586407134747593 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as libc::c_int {
                                                    return 0 as libc::c_int as Bool;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                progName,
                                                                inName.as_mut_ptr(),
                                                            );
                                                    }
                                                    return 1 as libc::c_int as Bool;
                                                }
                                            }
                                        }
                                    }
                                    -6 => {}
                                    -4 => {
                                        current_block = 2689728545616102333;
                                        match current_block {
                                            3330558285168407176 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            16334903743006538945 => {
                                                configError();
                                            }
                                            2689728545616102333 => {
                                                crcError();
                                            }
                                            12943795838694691063 => {
                                                outOfMemory();
                                            }
                                            17687586407134747593 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as libc::c_int {
                                                    return 0 as libc::c_int as Bool;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                progName,
                                                                inName.as_mut_ptr(),
                                                            );
                                                    }
                                                    return 1 as libc::c_int as Bool;
                                                }
                                            }
                                        }
                                    }
                                    -3 => {
                                        current_block = 12943795838694691063;
                                        match current_block {
                                            3330558285168407176 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            16334903743006538945 => {
                                                configError();
                                            }
                                            2689728545616102333 => {
                                                crcError();
                                            }
                                            12943795838694691063 => {
                                                outOfMemory();
                                            }
                                            17687586407134747593 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as libc::c_int {
                                                    return 0 as libc::c_int as Bool;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                progName,
                                                                inName.as_mut_ptr(),
                                                            );
                                                    }
                                                    return 1 as libc::c_int as Bool;
                                                }
                                            }
                                        }
                                    }
                                    -7 => {
                                        current_block = 17687586407134747593;
                                        match current_block {
                                            3330558285168407176 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            16334903743006538945 => {
                                                configError();
                                            }
                                            2689728545616102333 => {
                                                crcError();
                                            }
                                            12943795838694691063 => {
                                                outOfMemory();
                                            }
                                            17687586407134747593 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as libc::c_int {
                                                    return 0 as libc::c_int as Bool;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                progName,
                                                                inName.as_mut_ptr(),
                                                            );
                                                    }
                                                    return 1 as libc::c_int as Bool;
                                                }
                                            }
                                        }
                                    }
                                    -5 => {
                                        current_block = 14809079967989167248;
                                        match current_block {
                                            3330558285168407176 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            16334903743006538945 => {
                                                configError();
                                            }
                                            2689728545616102333 => {
                                                crcError();
                                            }
                                            12943795838694691063 => {
                                                outOfMemory();
                                            }
                                            17687586407134747593 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as libc::c_int {
                                                    return 0 as libc::c_int as Bool;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                progName,
                                                                inName.as_mut_ptr(),
                                                            );
                                                    }
                                                    return 1 as libc::c_int as Bool;
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        current_block = 3330558285168407176;
                                        match current_block {
                                            3330558285168407176 => {
                                                panic(
                                                    b"decompress:unexpected error\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            16334903743006538945 => {
                                                configError();
                                            }
                                            2689728545616102333 => {
                                                crcError();
                                            }
                                            12943795838694691063 => {
                                                outOfMemory();
                                            }
                                            17687586407134747593 => {
                                                compressedStreamEOF();
                                            }
                                            _ => {
                                                if zStream != stdin {
                                                    fclose(zStream);
                                                }
                                                if stream != stdout {
                                                    fclose(stream);
                                                }
                                                if streamNo == 1 as libc::c_int {
                                                    return 0 as libc::c_int as Bool;
                                                } else {
                                                    if noisy != 0 {
                                                        fprintf(
                                                                stderr,
                                                                b"\n%s: %s: trailing garbage after EOF ignored\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                progName,
                                                                inName.as_mut_ptr(),
                                                            );
                                                    }
                                                    return 1 as libc::c_int as Bool;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                if !(ferror(zStream) != 0) {
                                    if stream != stdout {
                                        let mut fd = fileno(stream);
                                        if fd < 0 as libc::c_int {
                                            current_block = 9292489448180700977;
                                        } else {
                                            applySavedFileAttrToOutputFile(fd);
                                            current_block = 14832935472441733737;
                                        }
                                    } else {
                                        current_block = 14832935472441733737;
                                    }
                                    match current_block {
                                        9292489448180700977 => {}
                                        _ => {
                                            ret = fclose(zStream);
                                            if !(ret == -(1 as libc::c_int)) {
                                                if !(ferror(stream) != 0) {
                                                    ret = fflush(stream);
                                                    if !(ret != 0 as libc::c_int) {
                                                        if stream != stdout {
                                                            ret = fclose(stream);
                                                            outputHandleJustInCase = 0 as *mut FILE;
                                                            if ret == -(1 as libc::c_int) {
                                                                current_block = 9292489448180700977;
                                                            } else {
                                                                current_block =
                                                                    14775119014532381840;
                                                            }
                                                        } else {
                                                            current_block = 14775119014532381840;
                                                        }
                                                        match current_block {
                                                            9292489448180700977 => {}
                                                            _ => {
                                                                outputHandleJustInCase =
                                                                    0 as *mut FILE;
                                                                if verbosity >= 2 as libc::c_int {
                                                                    fprintf(
                                                                        stderr,
                                                                        b"\n    \0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                return 1 as libc::c_int as Bool;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
    ioError();
}
unsafe extern "C" fn testStream(mut zStream: *mut FILE) -> Bool {
    let mut current_block: u64;
    let mut bzf = 0 as *mut libc::c_void;
    let mut bzerr: Int32 = 0;
    let mut bzerr_dummy: Int32 = 0;
    let mut ret: Int32 = 0;
    let mut streamNo: Int32 = 0;
    let mut i: Int32 = 0;
    let mut obuf: [UChar; 5000] = [0; 5000];
    let mut unused: [UChar; 5000] = [0; 5000];
    let mut nUnused: Int32 = 0;
    let mut unusedTmpV = 0 as *mut libc::c_void;
    let mut unusedTmp = 0 as *mut UChar;
    nUnused = 0 as libc::c_int;
    streamNo = 0 as libc::c_int;
    if !(ferror(zStream) != 0) {
        's_41: loop {
            if !(1 as libc::c_int as Bool != 0) {
                current_block = 15125582407903384992;
                break;
            }
            bzf = crate::src::bzlib::BZ2_bzReadOpen(
                core::ptr::addr_of_mut!(bzerr),
                zStream,
                verbosity,
                smallMode as libc::c_int,
                unused.as_mut_ptr() as *mut libc::c_void,
                nUnused,
            );
            if bzf.is_null() || bzerr != 0 as libc::c_int {
                current_block = 7431387952971291956;
                break;
            }
            streamNo += 1;
            while bzerr == 0 as libc::c_int {
                crate::src::bzlib::BZ2_bzRead(
                    Some(&mut bzerr),
                    bzf,
                    obuf.as_mut_ptr() as *mut libc::c_void,
                    5000 as libc::c_int,
                );
                if bzerr == -(5 as libc::c_int) {
                    current_block = 7431387952971291956;
                    break 's_41;
                }
            }
            if bzerr != 4 as libc::c_int {
                current_block = 7431387952971291956;
                break;
            }
            crate::src::bzlib::BZ2_bzReadGetUnused(
                Some(&mut bzerr),
                bzf,
                Some(&mut unusedTmpV),
                Some(&mut nUnused),
            );
            if bzerr != 0 as libc::c_int {
                panic(b"test:bzReadGetUnused\0" as *const u8 as *const libc::c_char);
            }
            unusedTmp = unusedTmpV as *mut UChar;
            i = 0 as libc::c_int;
            while i < nUnused {
                unused[i as usize] = *unusedTmp.offset(i as isize);
                i += 1;
            }
            crate::src::bzlib::BZ2_bzReadClose(core::ptr::addr_of_mut!(bzerr), bzf);
            if bzerr != 0 as libc::c_int {
                panic(b"test:bzReadGetUnused\0" as *const u8 as *const libc::c_char);
            }
            if nUnused == 0 as libc::c_int && myfeof(zStream) as libc::c_int != 0 {
                current_block = 15125582407903384992;
                break;
            }
        }
        match current_block {
            15125582407903384992 => {
                if !(ferror(zStream) != 0) {
                    ret = fclose(zStream);
                    if !(ret == -(1 as libc::c_int)) {
                        if verbosity >= 2 as libc::c_int {
                            fprintf(stderr, b"\n    \0" as *const u8 as *const libc::c_char);
                        }
                        return 1 as libc::c_int as Bool;
                    }
                }
            }
            _ => {
                crate::src::bzlib::BZ2_bzReadClose(core::ptr::addr_of_mut!(bzerr_dummy), bzf);
                if verbosity == 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"%s: %s: \0" as *const u8 as *const libc::c_char,
                        progName,
                        inName.as_mut_ptr(),
                    );
                }
                match bzerr {
                    -9 => {
                        current_block = 16978501743203240169;
                        match current_block {
                            11318711393363584899 => {
                                panic(
                                    b"test:unexpected error\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            2147259178340583234 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                            14509768194702470232 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int as Bool;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    return 1 as libc::c_int as Bool;
                                }
                            }
                            16978501743203240169 => {
                                configError();
                            }
                            4440632839760836095 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                        }
                    }
                    -6 => {}
                    -4 => {
                        current_block = 6035933626091472070;
                        match current_block {
                            11318711393363584899 => {
                                panic(
                                    b"test:unexpected error\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            2147259178340583234 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                            14509768194702470232 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int as Bool;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    return 1 as libc::c_int as Bool;
                                }
                            }
                            16978501743203240169 => {
                                configError();
                            }
                            4440632839760836095 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                        }
                    }
                    -3 => {
                        current_block = 4440632839760836095;
                        match current_block {
                            11318711393363584899 => {
                                panic(
                                    b"test:unexpected error\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            2147259178340583234 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                            14509768194702470232 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int as Bool;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    return 1 as libc::c_int as Bool;
                                }
                            }
                            16978501743203240169 => {
                                configError();
                            }
                            4440632839760836095 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                        }
                    }
                    -7 => {
                        current_block = 2147259178340583234;
                        match current_block {
                            11318711393363584899 => {
                                panic(
                                    b"test:unexpected error\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            2147259178340583234 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                            14509768194702470232 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int as Bool;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    return 1 as libc::c_int as Bool;
                                }
                            }
                            16978501743203240169 => {
                                configError();
                            }
                            4440632839760836095 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                        }
                    }
                    -5 => {
                        current_block = 14509768194702470232;
                        match current_block {
                            11318711393363584899 => {
                                panic(
                                    b"test:unexpected error\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            2147259178340583234 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                            14509768194702470232 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int as Bool;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    return 1 as libc::c_int as Bool;
                                }
                            }
                            16978501743203240169 => {
                                configError();
                            }
                            4440632839760836095 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                        }
                    }
                    _ => {
                        current_block = 11318711393363584899;
                        match current_block {
                            11318711393363584899 => {
                                panic(
                                    b"test:unexpected error\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            2147259178340583234 => {
                                fprintf(
                                    stderr,
                                    b"file ends unexpectedly\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                            14509768194702470232 => {
                                if zStream != stdin {
                                    fclose(zStream);
                                }
                                if streamNo == 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"bad magic number (file not created by bzip2)\n\0"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    return 0 as libc::c_int as Bool;
                                } else {
                                    if noisy != 0 {
                                        fprintf(
                                            stderr,
                                            b"trailing garbage after EOF ignored\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    return 1 as libc::c_int as Bool;
                                }
                            }
                            16978501743203240169 => {
                                configError();
                            }
                            4440632839760836095 => {
                                outOfMemory();
                            }
                            _ => {
                                fprintf(
                                    stderr,
                                    b"data integrity (CRC) error in data\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return 0 as libc::c_int as Bool;
                            }
                        }
                    }
                }
            }
        }
    }
    ioError();
}
unsafe extern "C" fn setExit(mut v: Int32) {
    if v > exitValue {
        exitValue = v;
    }
}
unsafe extern "C" fn cadvise() {
    if noisy != 0 {
        fprintf(
            stderr,
            b"\nIt is possible that the compressed file(s) have become corrupted.\nYou can use the -tvv option to test integrity of such files.\n\nYou can use the `bzip2recover' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn showFileNames() {
    if noisy != 0 {
        fprintf(
            stderr,
            b"\tInput file = %s, output file = %s\n\0" as *const u8 as *const libc::c_char,
            inName.as_mut_ptr(),
            outName.as_mut_ptr(),
        );
    }
}
unsafe extern "C" fn cleanUpAndFail(mut ec: Int32) -> ! {
    let mut retVal: IntNative = 0;
    let mut statBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if srcMode == 3 as libc::c_int
        && opMode != 3 as libc::c_int
        && deleteOutputOnInterrupt as libc::c_int != 0
    {
        retVal = stat(inName.as_mut_ptr(), core::ptr::addr_of_mut!(statBuf));
        if retVal == 0 as libc::c_int {
            if noisy != 0 {
                fprintf(
                    stderr,
                    b"%s: Deleting output file %s, if it exists.\n\0" as *const u8
                        as *const libc::c_char,
                    progName,
                    outName.as_mut_ptr(),
                );
            }
            if !outputHandleJustInCase.is_null() {
                fclose(outputHandleJustInCase);
            } else {
                ();
            }
            retVal = remove(outName.as_mut_ptr());
            if retVal != 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s: WARNING: deletion of output file (apparently) failed.\n\0" as *const u8
                        as *const libc::c_char,
                    progName,
                );
            }
        } else {
            fprintf(
                stderr,
                b"%s: WARNING: deletion of output file suppressed\n\0" as *const u8
                    as *const libc::c_char,
                progName,
            );
            fprintf(
                stderr,
                b"%s:    since input file no longer exists.  Output file\n\0" as *const u8
                    as *const libc::c_char,
                progName,
            );
            fprintf(
                stderr,
                b"%s:    `%s' may be incomplete.\n\0" as *const u8 as *const libc::c_char,
                progName,
                outName.as_mut_ptr(),
            );
            fprintf(
                stderr,
                b"%s:    I suggest doing an integrity test (bzip2 -tv) of it.\n\0" as *const u8
                    as *const libc::c_char,
                progName,
            );
        }
    }
    if noisy as libc::c_int != 0
        && numFileNames > 0 as libc::c_int
        && numFilesProcessed < numFileNames
    {
        fprintf(
            stderr,
            b"%s: WARNING: some files have not been processed:\n%s:    %d specified on command line, %d not processed yet.\n\n\0"
                as *const u8 as *const libc::c_char,
            progName,
            progName,
            numFileNames,
            numFileNames - numFilesProcessed,
        );
    }
    setExit(ec);
    exit(exitValue);
}
unsafe extern "C" fn panic(mut s: *const Char) -> ! {
    fprintf(
        stderr,
        b"\n%s: PANIC -- internal consistency error:\n\t%s\n\tThis is a BUG.  Please report it to:\n\tbzip2-devel@sourceware.org\n\0"
            as *const u8 as *const libc::c_char,
        progName,
        s,
    );
    showFileNames();
    cleanUpAndFail(3 as libc::c_int);
}
unsafe extern "C" fn crcError() -> ! {
    fprintf(
        stderr,
        b"\n%s: Data integrity error when decompressing.\n\0" as *const u8 as *const libc::c_char,
        progName,
    );
    showFileNames();
    cadvise();
    cleanUpAndFail(2 as libc::c_int);
}
unsafe extern "C" fn compressedStreamEOF() -> ! {
    if noisy != 0 {
        fprintf(
            stderr,
            b"\n%s: Compressed file ends unexpectedly;\n\tperhaps it is corrupted?  *Possible* reason follows.\n\0"
                as *const u8 as *const libc::c_char,
            progName,
        );
        perror(progName);
        showFileNames();
        cadvise();
    }
    cleanUpAndFail(2 as libc::c_int);
}
unsafe extern "C" fn ioError() -> ! {
    fprintf(
        stderr,
        b"\n%s: I/O or other error, bailing out.  Possible reason follows.\n\0" as *const u8
            as *const libc::c_char,
        progName,
    );
    perror(progName);
    showFileNames();
    cleanUpAndFail(1 as libc::c_int);
}
unsafe extern "C" fn mySignalCatcher(mut n: IntNative) {
    fprintf(
        stderr,
        b"\n%s: Control-C or similar caught, quitting.\n\0" as *const u8 as *const libc::c_char,
        progName,
    );
    cleanUpAndFail(1 as libc::c_int);
}
unsafe extern "C" fn mySIGSEGVorSIGBUScatcher(mut n: IntNative) {
    if opMode == 1 as libc::c_int {
        fprintf(
            stderr,
            b"\n%s: Caught a SIGSEGV or SIGBUS whilst compressing.\n\n   Possible causes are (most likely first):\n   (1) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (2) A bug in the compiler used to create this executable\n       (unlikely, if you didn't compile bzip2 yourself.)\n   (3) A real bug in bzip2 -- I hope this should never be the case.\n   The user's manual, Section 4.3, has more info on (1) and (2).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (1)\n   or (2), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user's manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don't\n   have the manual or can't be bothered to read it, mail me anyway.\n\n\0"
                as *const u8 as *const libc::c_char,
            progName,
        );
    } else {
        fprintf(
            stderr,
            b"\n%s: Caught a SIGSEGV or SIGBUS whilst decompressing.\n\n   Possible causes are (most likely first):\n   (1) The compressed data is corrupted, and bzip2's usual checks\n       failed to detect this.  Try bzip2 -tvv my_file.bz2.\n   (2) This computer has unreliable memory or cache hardware\n       (a surprisingly common problem; try a different machine.)\n   (3) A bug in the compiler used to create this executable\n       (unlikely, if you didn't compile bzip2 yourself.)\n   (4) A real bug in bzip2 -- I hope this should never be the case.\n   The user's manual, Section 4.3, has more info on (2) and (3).\n   \n   If you suspect this is a bug in bzip2, or are unsure about (2)\n   or (3), feel free to report it to: bzip2-devel@sourceware.org.\n   Section 4.3 of the user's manual describes the info a useful\n   bug report should have.  If the manual is available on your\n   system, please try and read it before mailing me.  If you don't\n   have the manual or can't be bothered to read it, mail me anyway.\n\n\0"
                as *const u8 as *const libc::c_char,
            progName,
        );
    }
    showFileNames();
    if opMode == 1 as libc::c_int {
        cleanUpAndFail(3 as libc::c_int);
    } else {
        cadvise();
        cleanUpAndFail(2 as libc::c_int);
    };
}
unsafe extern "C" fn outOfMemory() -> ! {
    fprintf(
        stderr,
        b"\n%s: couldn't allocate enough memory\n\0" as *const u8 as *const libc::c_char,
        progName,
    );
    showFileNames();
    cleanUpAndFail(1 as libc::c_int);
}
unsafe extern "C" fn configError() -> ! {
    fprintf(
        stderr,
        b"bzip2: I'm not configured correctly for this platform!\n\tI require Int32, Int16 and Char to have sizes\n\tof 4, 2 and 1 bytes to run properly, and they don't.\n\tProbably you can fix this by defining them correctly,\n\tand recompiling.  Bye!\n\0"
            as *const u8 as *const libc::c_char,
    );
    setExit(3 as libc::c_int);
    exit(exitValue);
}
unsafe extern "C" fn pad(mut s: *const Char) {
    let mut i: Int32 = 0;
    if strlen(s) as Int32 >= longestFileName {
        return;
    }
    i = 1 as libc::c_int;
    while i <= longestFileName - strlen(s) as Int32 {
        fprintf(stderr, b" \0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
unsafe extern "C" fn copyFileName(mut to: *mut Char, mut from: *mut Char) {
    if strlen(from) > (1034 as libc::c_int - 10 as libc::c_int) as libc::c_ulong {
        fprintf(
            stderr,
            b"bzip2: file name\n`%s'\nis suspiciously (more than %d chars) long.\nTry using a reasonable file name instead.  Sorry! :-)\n\0"
                as *const u8 as *const libc::c_char,
            from,
            1034 as libc::c_int - 10 as libc::c_int,
        );
        setExit(1 as libc::c_int);
        exit(exitValue);
    }
    strncpy(
        to,
        from,
        (1034 as libc::c_int - 10 as libc::c_int) as libc::c_ulong,
    );
    *to.offset((1034 as libc::c_int - 10 as libc::c_int) as isize) = '\0' as i32 as Char;
}
unsafe extern "C" fn fileExists(mut name: *mut Char) -> Bool {
    let mut tmp = fopen(name, b"rb\0" as *const u8 as *const libc::c_char);
    let mut exists = (tmp != 0 as *mut libc::c_void as *mut FILE) as libc::c_int as Bool;
    if !tmp.is_null() {
        fclose(tmp);
    } else {
        ();
    }
    return exists;
}
unsafe extern "C" fn fopen_output_safely(
    mut name: *mut Char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut fp = 0 as *mut FILE;
    let mut fh: IntNative = 0;
    fh = open(
        name,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o200 as libc::c_int,
        0o200 as libc::c_int | 0o400 as libc::c_int,
    );
    if fh == -(1 as libc::c_int) {
        return 0 as *mut FILE;
    }
    fp = fdopen(fh, mode);
    if fp.is_null() {
        ();
        close(fh);
    }
    return fp;
}
unsafe extern "C" fn notAStandardFile(mut name: *mut Char) -> Bool {
    let mut i: IntNative = 0;
    let mut statBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    i = lstat(name, core::ptr::addr_of_mut!(statBuf));
    if i != 0 as libc::c_int {
        return 1 as libc::c_int as Bool;
    }
    if statBuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as Bool;
    }
    return 1 as libc::c_int as Bool;
}
unsafe extern "C" fn countHardLinks(mut name: *mut Char) -> Int32 {
    let mut i: IntNative = 0;
    let mut statBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    i = lstat(name, core::ptr::addr_of_mut!(statBuf));
    if i != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return statBuf
        .st_nlink
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as Int32;
}
static mut fileMetaInfo: stat = stat {
    st_dev: 0,
    st_ino: 0,
    st_nlink: 0,
    st_mode: 0,
    st_uid: 0,
    st_gid: 0,
    __pad0: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_mtim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    st_ctim: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    __glibc_reserved: [0; 3],
};
unsafe extern "C" fn saveInputFileMetaInfo(mut srcName: *mut Char) {
    let mut retVal: IntNative = 0;
    retVal = stat(srcName, &raw mut fileMetaInfo);
    if retVal != 0 as libc::c_int {
        ioError();
    }
}
unsafe extern "C" fn applySavedTimeInfoToOutputFile(mut dstName: *mut Char) {
    let mut retVal: IntNative = 0;
    let mut uTimBuf = utimbuf {
        actime: 0,
        modtime: 0,
    };
    uTimBuf.actime = fileMetaInfo.st_atim.tv_sec;
    uTimBuf.modtime = fileMetaInfo.st_mtim.tv_sec;
    retVal = utime(dstName, core::ptr::addr_of!(uTimBuf));
    if retVal != 0 as libc::c_int {
        ioError();
    }
}
unsafe extern "C" fn applySavedFileAttrToOutputFile(mut fd: IntNative) {
    let mut retVal: IntNative = 0;
    retVal = fchmod(fd, fileMetaInfo.st_mode);
    if retVal != 0 as libc::c_int {
        ioError();
    }
    fchown(fd, fileMetaInfo.st_uid, fileMetaInfo.st_gid);
}
unsafe extern "C" fn containsDubiousChars(mut name: *const Char) -> Bool {
    return 0 as libc::c_int as Bool;
}
#[no_mangle]
pub static mut zSuffix: [*const Char; 4] = [
    b".bz2\0" as *const u8 as *const libc::c_char,
    b".bz\0" as *const u8 as *const libc::c_char,
    b".tbz2\0" as *const u8 as *const libc::c_char,
    b".tbz\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut unzSuffix: [*const Char; 4] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b".tar\0" as *const u8 as *const libc::c_char,
    b".tar\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn hasSuffix(mut s: *const Char, mut suffix: *const Char) -> Bool {
    let mut ns = strlen(s) as Int32;
    let mut nx = strlen(suffix) as Int32;
    if ns < nx {
        return 0 as libc::c_int as Bool;
    }
    if strcmp(s.offset(ns as isize).offset(-(nx as isize)), suffix) == 0 as libc::c_int {
        return 1 as libc::c_int as Bool;
    }
    return 0 as libc::c_int as Bool;
}
unsafe extern "C" fn mapSuffix(
    mut name: *mut Char,
    mut oldSuffix: *const Char,
    mut newSuffix: *const Char,
) -> Bool {
    if hasSuffix(name, oldSuffix) == 0 {
        return 0 as libc::c_int as Bool;
    }
    *name.offset((strlen(name)).wrapping_sub(strlen(oldSuffix)) as isize) =
        0 as libc::c_int as Char;
    strcat(name, newSuffix);
    return 1 as libc::c_int as Bool;
}
unsafe extern "C" fn compress(mut name: *mut Char) {
    let mut inStr = 0 as *mut FILE;
    let mut outStr = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut statBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
    if name.is_null() && srcMode != 1 as libc::c_int {
        panic(b"compress: bad modes\n\0" as *const u8 as *const libc::c_char);
    }
    match srcMode {
        1 => {
            copyFileName(
                inName.as_mut_ptr(),
                b"(stdin)\0" as *const u8 as *const libc::c_char as *mut Char,
            );
            copyFileName(
                outName.as_mut_ptr(),
                b"(stdout)\0" as *const u8 as *const libc::c_char as *mut Char,
            );
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            strcat(
                outName.as_mut_ptr(),
                b".bz2\0" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(
                outName.as_mut_ptr(),
                b"(stdout)\0" as *const u8 as *const libc::c_char as *mut Char,
            );
        }
        _ => {}
    }
    if srcMode != 1 as libc::c_int && containsDubiousChars(inName.as_mut_ptr()) as libc::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: There are no files matching `%s'.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
        }
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode != 1 as libc::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(
            stderr,
            b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const libc::c_char,
            progName,
            inName.as_mut_ptr(),
            strerror(*__errno_location()),
        );
        setExit(1 as libc::c_int);
        return;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if hasSuffix(inName.as_mut_ptr(), zSuffix[i as usize]) != 0 {
            if noisy != 0 {
                fprintf(
                    stderr,
                    b"%s: Input file %s already has %s suffix.\n\0" as *const u8
                        as *const libc::c_char,
                    progName,
                    inName.as_mut_ptr(),
                    zSuffix[i as usize],
                );
            }
            setExit(1 as libc::c_int);
            return;
        }
        i += 1;
    }
    if srcMode == 3 as libc::c_int || srcMode == 2 as libc::c_int {
        stat(inName.as_mut_ptr(), core::ptr::addr_of_mut!(statBuf));
        if statBuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s: Input file %s is a directory.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
            setExit(1 as libc::c_int);
            return;
        }
    }
    if srcMode == 3 as libc::c_int
        && forceOverwrite == 0
        && notAStandardFile(inName.as_mut_ptr()) as libc::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: Input file %s is not a normal file.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
        }
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode == 3 as libc::c_int && fileExists(outName.as_mut_ptr()) as libc::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(
                stderr,
                b"%s: Output file %s already exists.\n\0" as *const u8 as *const libc::c_char,
                progName,
                outName.as_mut_ptr(),
            );
            setExit(1 as libc::c_int);
            return;
        }
    }
    if srcMode == 3 as libc::c_int && forceOverwrite == 0 && {
        n = countHardLinks(inName.as_mut_ptr());
        n > 0 as libc::c_int
    } {
        fprintf(
            stderr,
            b"%s: Input file %s has %d other link%s.\n\0" as *const u8 as *const libc::c_char,
            progName,
            inName.as_mut_ptr(),
            n,
            if n > 1 as libc::c_int {
                b"s\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode == 3 as libc::c_int {
        saveInputFileMetaInfo(inName.as_mut_ptr());
    }
    match srcMode {
        1 => {
            inStr = stdin;
            outStr = stdout;
            if isatty(fileno(stdout)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't write compressed data to a terminal.\n\0" as *const u8
                        as *const libc::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    progName,
                );
                setExit(1 as libc::c_int);
                return;
            }
        }
        2 => {
            inStr = fopen(
                inName.as_mut_ptr(),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            outStr = stdout;
            if isatty(fileno(stdout)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't write compressed data to a terminal.\n\0" as *const u8
                        as *const libc::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    progName,
                );
                if !inStr.is_null() {
                    fclose(inStr);
                } else {
                    ();
                }
                setExit(1 as libc::c_int);
                return;
            }
            if inStr.is_null() {
                ();
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    inName.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
                setExit(1 as libc::c_int);
                return;
            }
        }
        3 => {
            inStr = fopen(
                inName.as_mut_ptr(),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            outStr = fopen_output_safely(
                outName.as_mut_ptr(),
                b"wb\0" as *const u8 as *const libc::c_char,
            );
            if outStr.is_null() {
                ();
                fprintf(
                    stderr,
                    b"%s: Can't create output file %s: %s.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    outName.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
                if !inStr.is_null() {
                    fclose(inStr);
                } else {
                    ();
                }
                setExit(1 as libc::c_int);
                return;
            }
            if inStr.is_null() {
                ();
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    inName.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
                if !outStr.is_null() {
                    fclose(outStr);
                } else {
                    ();
                }
                setExit(1 as libc::c_int);
                return;
            }
        }
        _ => {
            panic(b"compress: bad srcMode\0" as *const u8 as *const libc::c_char);
        }
    }
    if verbosity >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"  %s: \0" as *const u8 as *const libc::c_char,
            inName.as_mut_ptr(),
        );
        pad(inName.as_mut_ptr());
        fflush(stderr);
    }
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as libc::c_int as Bool;
    compressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    if srcMode == 3 as libc::c_int {
        applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
        deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
        if keepInputFiles == 0 {
            let mut retVal = remove(inName.as_mut_ptr());
            if retVal != 0 as libc::c_int {
                ioError();
            }
        }
    }
    deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
}
unsafe extern "C" fn uncompress(mut name: *mut Char) {
    let mut current_block: u64;
    let mut inStr = 0 as *mut FILE;
    let mut outStr = 0 as *mut FILE;
    let mut n: Int32 = 0;
    let mut i: Int32 = 0;
    let mut magicNumberOK: Bool = 0;
    let mut cantGuess: Bool = 0;
    let mut statBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
    if name.is_null() && srcMode != 1 as libc::c_int {
        panic(b"uncompress: bad modes\n\0" as *const u8 as *const libc::c_char);
    }
    cantGuess = 0 as libc::c_int as Bool;
    match srcMode {
        1 => {
            copyFileName(
                inName.as_mut_ptr(),
                b"(stdin)\0" as *const u8 as *const libc::c_char as *mut Char,
            );
            copyFileName(
                outName.as_mut_ptr(),
                b"(stdout)\0" as *const u8 as *const libc::c_char as *mut Char,
            );
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(outName.as_mut_ptr(), name);
            i = 0 as libc::c_int;
            loop {
                if !(i < 4 as libc::c_int) {
                    current_block = 17860125682698302841;
                    break;
                }
                if mapSuffix(
                    outName.as_mut_ptr(),
                    zSuffix[i as usize],
                    unzSuffix[i as usize],
                ) != 0
                {
                    current_block = 13197658476834711520;
                    break;
                }
                i += 1;
            }
            match current_block {
                13197658476834711520 => {}
                _ => {
                    cantGuess = 1 as libc::c_int as Bool;
                    strcat(
                        outName.as_mut_ptr(),
                        b".out\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
            copyFileName(
                outName.as_mut_ptr(),
                b"(stdout)\0" as *const u8 as *const libc::c_char as *mut Char,
            );
        }
        _ => {}
    }
    if srcMode != 1 as libc::c_int && containsDubiousChars(inName.as_mut_ptr()) as libc::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: There are no files matching `%s'.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
        }
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode != 1 as libc::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(
            stderr,
            b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const libc::c_char,
            progName,
            inName.as_mut_ptr(),
            strerror(*__errno_location()),
        );
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode == 3 as libc::c_int || srcMode == 2 as libc::c_int {
        stat(inName.as_mut_ptr(), core::ptr::addr_of_mut!(statBuf));
        if statBuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s: Input file %s is a directory.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
            setExit(1 as libc::c_int);
            return;
        }
    }
    if srcMode == 3 as libc::c_int
        && forceOverwrite == 0
        && notAStandardFile(inName.as_mut_ptr()) as libc::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: Input file %s is not a normal file.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
        }
        setExit(1 as libc::c_int);
        return;
    }
    if cantGuess != 0 {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: Can't guess original name for %s -- using %s\n\0" as *const u8
                    as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
                outName.as_mut_ptr(),
            );
        }
    }
    if srcMode == 3 as libc::c_int && fileExists(outName.as_mut_ptr()) as libc::c_int != 0 {
        if forceOverwrite != 0 {
            remove(outName.as_mut_ptr());
        } else {
            fprintf(
                stderr,
                b"%s: Output file %s already exists.\n\0" as *const u8 as *const libc::c_char,
                progName,
                outName.as_mut_ptr(),
            );
            setExit(1 as libc::c_int);
            return;
        }
    }
    if srcMode == 3 as libc::c_int && forceOverwrite == 0 && {
        n = countHardLinks(inName.as_mut_ptr());
        n > 0 as libc::c_int
    } {
        fprintf(
            stderr,
            b"%s: Input file %s has %d other link%s.\n\0" as *const u8 as *const libc::c_char,
            progName,
            inName.as_mut_ptr(),
            n,
            if n > 1 as libc::c_int {
                b"s\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode == 3 as libc::c_int {
        saveInputFileMetaInfo(inName.as_mut_ptr());
    }
    match srcMode {
        1 => {
            inStr = stdin;
            outStr = stdout;
            if isatty(fileno(stdin)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't read compressed data from a terminal.\n\0" as *const u8
                        as *const libc::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    progName,
                );
                setExit(1 as libc::c_int);
                return;
            }
        }
        2 => {
            inStr = fopen(
                inName.as_mut_ptr(),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            outStr = stdout;
            if inStr.is_null() {
                ();
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s:%s.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    inName.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
                if !inStr.is_null() {
                    fclose(inStr);
                } else {
                    ();
                }
                setExit(1 as libc::c_int);
                return;
            }
        }
        3 => {
            inStr = fopen(
                inName.as_mut_ptr(),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            outStr = fopen_output_safely(
                outName.as_mut_ptr(),
                b"wb\0" as *const u8 as *const libc::c_char,
            );
            if outStr.is_null() {
                ();
                fprintf(
                    stderr,
                    b"%s: Can't create output file %s: %s.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    outName.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
                if !inStr.is_null() {
                    fclose(inStr);
                } else {
                    ();
                }
                setExit(1 as libc::c_int);
                return;
            }
            if inStr.is_null() {
                ();
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s: %s.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    inName.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
                if !outStr.is_null() {
                    fclose(outStr);
                } else {
                    ();
                }
                setExit(1 as libc::c_int);
                return;
            }
        }
        _ => {
            panic(b"uncompress: bad srcMode\0" as *const u8 as *const libc::c_char);
        }
    }
    if verbosity >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"  %s: \0" as *const u8 as *const libc::c_char,
            inName.as_mut_ptr(),
        );
        pad(inName.as_mut_ptr());
        fflush(stderr);
    }
    outputHandleJustInCase = outStr;
    deleteOutputOnInterrupt = 1 as libc::c_int as Bool;
    magicNumberOK = uncompressStream(inStr, outStr);
    outputHandleJustInCase = 0 as *mut FILE;
    if magicNumberOK != 0 {
        if srcMode == 3 as libc::c_int {
            applySavedTimeInfoToOutputFile(outName.as_mut_ptr());
            deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
            if keepInputFiles == 0 {
                let mut retVal = remove(inName.as_mut_ptr());
                if retVal != 0 as libc::c_int {
                    ioError();
                }
            }
        }
    } else {
        unzFailsExist = 1 as libc::c_int as Bool;
        deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
        if srcMode == 3 as libc::c_int {
            let mut retVal_0 = remove(outName.as_mut_ptr());
            if retVal_0 != 0 as libc::c_int {
                ioError();
            }
        }
    }
    deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
    if magicNumberOK != 0 {
        if verbosity >= 1 as libc::c_int {
            fprintf(stderr, b"done\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        setExit(2 as libc::c_int);
        if verbosity >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"not a bzip2 file.\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(
                stderr,
                b"%s: %s is not a bzip2 file.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
        }
    };
}
unsafe extern "C" fn testf(mut name: *mut Char) {
    let mut inStr = 0 as *mut FILE;
    let mut allOK: Bool = 0;
    let mut statBuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
    if name.is_null() && srcMode != 1 as libc::c_int {
        panic(b"testf: bad modes\n\0" as *const u8 as *const libc::c_char);
    }
    copyFileName(
        outName.as_mut_ptr(),
        b"(none)\0" as *const u8 as *const libc::c_char as *mut Char,
    );
    match srcMode {
        1 => {
            copyFileName(
                inName.as_mut_ptr(),
                b"(stdin)\0" as *const u8 as *const libc::c_char as *mut Char,
            );
        }
        3 => {
            copyFileName(inName.as_mut_ptr(), name);
        }
        2 => {
            copyFileName(inName.as_mut_ptr(), name);
        }
        _ => {}
    }
    if srcMode != 1 as libc::c_int && containsDubiousChars(inName.as_mut_ptr()) as libc::c_int != 0
    {
        if noisy != 0 {
            fprintf(
                stderr,
                b"%s: There are no files matching `%s'.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
        }
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode != 1 as libc::c_int && fileExists(inName.as_mut_ptr()) == 0 {
        fprintf(
            stderr,
            b"%s: Can't open input %s: %s.\n\0" as *const u8 as *const libc::c_char,
            progName,
            inName.as_mut_ptr(),
            strerror(*__errno_location()),
        );
        setExit(1 as libc::c_int);
        return;
    }
    if srcMode != 1 as libc::c_int {
        stat(inName.as_mut_ptr(), core::ptr::addr_of_mut!(statBuf));
        if statBuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            fprintf(
                stderr,
                b"%s: Input file %s is a directory.\n\0" as *const u8 as *const libc::c_char,
                progName,
                inName.as_mut_ptr(),
            );
            setExit(1 as libc::c_int);
            return;
        }
    }
    match srcMode {
        1 => {
            if isatty(fileno(stdin)) != 0 {
                fprintf(
                    stderr,
                    b"%s: I won't read compressed data from a terminal.\n\0" as *const u8
                        as *const libc::c_char,
                    progName,
                );
                fprintf(
                    stderr,
                    b"%s: For help, type: `%s --help'.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    progName,
                );
                setExit(1 as libc::c_int);
                return;
            }
            inStr = stdin;
        }
        2 | 3 => {
            inStr = fopen(
                inName.as_mut_ptr(),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            if inStr.is_null() {
                ();
                fprintf(
                    stderr,
                    b"%s: Can't open input file %s:%s.\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    inName.as_mut_ptr(),
                    strerror(*__errno_location()),
                );
                setExit(1 as libc::c_int);
                return;
            }
        }
        _ => {
            panic(b"testf: bad srcMode\0" as *const u8 as *const libc::c_char);
        }
    }
    if verbosity >= 1 as libc::c_int {
        fprintf(
            stderr,
            b"  %s: \0" as *const u8 as *const libc::c_char,
            inName.as_mut_ptr(),
        );
        pad(inName.as_mut_ptr());
        fflush(stderr);
    }
    outputHandleJustInCase = 0 as *mut FILE;
    allOK = testStream(inStr);
    if allOK as libc::c_int != 0 && verbosity >= 1 as libc::c_int {
        fprintf(stderr, b"ok\n\0" as *const u8 as *const libc::c_char);
    }
    if allOK == 0 {
        testFailsExist = 1 as libc::c_int as Bool;
    }
}
unsafe extern "C" fn license() {
    fprintf(
        stderr,
        b"bzip2, a block-sorting file compressor.  Version %s.\n   \n   Copyright (C) 1996-2019 by Julian Seward.\n   \n   This program is free software; you can redistribute it and/or modify\n   it under the terms set out in the LICENSE file, which is included\n   in the bzip2 source distribution.\n   \n   This program is distributed in the hope that it will be useful,\n   but WITHOUT ANY WARRANTY; without even the implied warranty of\n   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n   LICENSE file for more details.\n   \n\0"
            as *const u8 as *const libc::c_char,
        crate::src::bzlib::BZ2_bzlibVersion(),
    );
}
unsafe extern "C" fn usage(mut fullProgName: *const Char) {
    fprintf(
        stderr,
        b"bzip2, a block-sorting file compressor.  Version %s.\n\n   usage: %s [flags and input files in any order]\n\n   -h --help           print this message\n   -d --decompress     force decompression\n   -z --compress       force compression\n   -k --keep           keep (don't delete) input files\n   -f --force          overwrite existing output files\n   -t --test           test compressed file integrity\n   -c --stdout         output to standard out\n   -q --quiet          suppress noncritical error messages\n   -v --verbose        be verbose (a 2nd -v gives more)\n   -L --license        display software version & license\n   -V --version        display software version & license\n   -s --small          use less memory (at most 2500k)\n   -1 .. -9            set block size to 100k .. 900k\n   --fast              alias for -1\n   --best              alias for -9\n\n   If invoked as `bzip2', default action is to compress.\n              as `bunzip2',  default action is to decompress.\n              as `bzcat', default action is to decompress to stdout.\n\n   If no file names are given, bzip2 compresses or decompresses\n   from standard input to standard output.  You can combine\n   short flags, so `-v -4' means the same as -v4 or -4v, &c.\n\n\0"
            as *const u8 as *const libc::c_char,
        crate::src::bzlib::BZ2_bzlibVersion(),
        fullProgName,
    );
}
unsafe extern "C" fn redundant(mut flag: *const Char) {
    fprintf(
        stderr,
        b"%s: %s is redundant in versions 0.9.5 and above\n\0" as *const u8 as *const libc::c_char,
        progName,
        flag,
    );
}
unsafe extern "C" fn myMalloc(mut n: Int32) -> *mut libc::c_void {
    let mut p = 0 as *mut libc::c_void;
    p = malloc(n as size_t);
    if p.is_null() {
        ();
        outOfMemory();
    }
    return p;
}
unsafe extern "C" fn mkCell() -> Option<Box<Cell>> {
    let mut c = None;
    c = myMalloc(::std::mem::size_of::<Cell>() as libc::c_ulong as Int32) as *mut Cell;
    (*c.as_deref_mut().unwrap()).name = 0 as *mut Char;
    (*c.as_deref_mut().unwrap()).link = 0 as *mut zzzz;
    return c;
}
unsafe extern "C" fn snocString(
    mut root: Option<Box<Cell>>,
    mut name: *mut Char,
) -> Option<Box<Cell>> {
    if root.as_deref().is_none() {
        ();
        let mut tmp = mkCell();
        (*tmp.as_deref_mut().unwrap()).name =
            myMalloc((5 as libc::c_int as libc::c_ulong).wrapping_add(strlen(name)) as Int32)
                as *mut Char;
        strcpy((*tmp.as_deref().unwrap()).name, name);
        return tmp;
    } else {
        let mut tmp_0 = root
            .as_deref_mut()
            .map(|r| r as *mut _)
            .unwrap_or(std::ptr::null_mut());
        while !(*tmp_0).link.is_null() {
            tmp_0 = (*tmp_0).link;
        }
        ();
        (*tmp_0).link = snocString(Some(Box::from_raw((*tmp_0).link)), name);
        return root;
    };
}
unsafe extern "C" fn addFlagsFromEnvVar(
    mut argList: Option<&mut Option<Box<Cell>>>,
    mut varName: *mut Char,
) {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut k: Int32 = 0;
    let mut envbase = 0 as *mut Char;
    let mut p = 0 as *mut Char;
    envbase = getenv(varName);
    if !envbase.is_null() {
        p = envbase;
        i = 0 as libc::c_int;
        while 1 as libc::c_int as Bool != 0 {
            if *p.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                break;
            }
            p = p.offset(i as isize);
            i = 0 as libc::c_int;
            while *(*__ctype_b_loc()).offset(*p.offset(0 as libc::c_int as isize) as Int32 as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                p = p.offset(1);
            }
            while *p.offset(i as isize) as libc::c_int != 0 as libc::c_int
                && *(*__ctype_b_loc()).offset(*p.offset(i as isize) as Int32 as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
            {
                i += 1;
            }
            if i > 0 as libc::c_int {
                k = i;
                if k > 1034 as libc::c_int - 10 as libc::c_int {
                    k = 1034 as libc::c_int - 10 as libc::c_int;
                }
                j = 0 as libc::c_int;
                while j < k {
                    tmpName[j as usize] = *p.offset(j as isize);
                    j += 1;
                }
                tmpName[k as usize] = 0 as libc::c_int as Char;
                *argList.as_deref_mut().unwrap() = snocString(
                    (*argList.as_deref_mut().unwrap()).take(),
                    tmpName.as_mut_ptr(),
                );
            }
        }
    } else {
        ();
    }
}
unsafe fn main_0(mut argc: IntNative, mut argv: *mut *mut Char) -> IntNative {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut tmp = 0 as *mut Char;
    let mut argList = None;
    let mut aa = None;
    let mut decode: Bool = 0;
    if ::std::mem::size_of::<Int32>() as libc::c_ulong != 4 as libc::c_int as libc::c_ulong
        || ::std::mem::size_of::<UInt32>() as libc::c_ulong != 4 as libc::c_int as libc::c_ulong
        || ::std::mem::size_of::<Int16>() as libc::c_ulong != 2 as libc::c_int as libc::c_ulong
        || ::std::mem::size_of::<UInt16>() as libc::c_ulong != 2 as libc::c_int as libc::c_ulong
        || ::std::mem::size_of::<Char>() as libc::c_ulong != 1 as libc::c_int as libc::c_ulong
        || ::std::mem::size_of::<UChar>() as libc::c_ulong != 1 as libc::c_int as libc::c_ulong
    {
        configError();
    }
    outputHandleJustInCase = 0 as *mut FILE;
    smallMode = 0 as libc::c_int as Bool;
    keepInputFiles = 0 as libc::c_int as Bool;
    forceOverwrite = 0 as libc::c_int as Bool;
    noisy = 1 as libc::c_int as Bool;
    verbosity = 0 as libc::c_int;
    blockSize100k = 9 as libc::c_int;
    testFailsExist = 0 as libc::c_int as Bool;
    unzFailsExist = 0 as libc::c_int as Bool;
    numFileNames = 0 as libc::c_int;
    numFilesProcessed = 0 as libc::c_int;
    workFactor = 30 as libc::c_int;
    deleteOutputOnInterrupt = 0 as libc::c_int as Bool;
    exitValue = 0 as libc::c_int;
    j = 0 as libc::c_int;
    i = j;
    signal(
        11 as libc::c_int,
        Some(mySIGSEGVorSIGBUScatcher as unsafe extern "C" fn(IntNative) -> ()),
    );
    signal(
        7 as libc::c_int,
        Some(mySIGSEGVorSIGBUScatcher as unsafe extern "C" fn(IntNative) -> ()),
    );
    copyFileName(
        inName.as_mut_ptr(),
        b"(none)\0" as *const u8 as *const libc::c_char as *mut Char,
    );
    copyFileName(
        outName.as_mut_ptr(),
        b"(none)\0" as *const u8 as *const libc::c_char as *mut Char,
    );
    copyFileName(
        progNameReally.as_mut_ptr(),
        *argv.offset(0 as libc::c_int as isize),
    );
    progName = &raw mut *progNameReally
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut Char;
    tmp = &raw mut *progNameReally
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut Char;
    while (*tmp) as libc::c_int != '\0' as i32 {
        if (*tmp) as libc::c_int == '/' as i32 {
            progName = tmp.offset(1 as libc::c_int as isize);
        }
        tmp = tmp.offset(1);
    }
    argList = None;
    addFlagsFromEnvVar(
        Some(&mut argList),
        b"BZIP2\0" as *const u8 as *const libc::c_char as *mut Char,
    );
    addFlagsFromEnvVar(
        Some(&mut argList),
        b"BZIP\0" as *const u8 as *const libc::c_char as *mut Char,
    );
    i = 1 as libc::c_int;
    while i <= argc - 1 as libc::c_int {
        argList = snocString(argList, *argv.offset(i as isize));
        i += 1;
    }
    longestFileName = 7 as libc::c_int;
    numFileNames = 0 as libc::c_int;
    decode = 1 as libc::c_int as Bool;
    aa = argList;
    while !aa.as_deref().is_none() {
        if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            decode = 0 as libc::c_int as Bool;
        } else if !(*(*aa.as_deref().unwrap())
            .name
            .offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && decode as libc::c_int != 0)
        {
            numFileNames += 1;
            if longestFileName < strlen((*aa.as_deref().unwrap()).name) as Int32 {
                longestFileName = strlen((*aa.as_deref().unwrap()).name) as Int32;
            }
        }
        aa = Some(Box::from_raw((*aa.as_deref().unwrap()).link));
    }
    ();
    if numFileNames == 0 as libc::c_int {
        srcMode = 1 as libc::c_int;
    } else {
        srcMode = 3 as libc::c_int;
    }
    opMode = 1 as libc::c_int;
    if !(strstr(progName, b"unzip\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(progName, b"UNZIP\0" as *const u8 as *const libc::c_char)).is_null()
    {
        opMode = 2 as libc::c_int;
    }
    if !(strstr(progName, b"z2cat\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(progName, b"Z2CAT\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(progName, b"zcat\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(progName, b"ZCAT\0" as *const u8 as *const libc::c_char)).is_null()
    {
        opMode = 2 as libc::c_int;
        srcMode = if numFileNames == 0 as libc::c_int {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        };
    }
    aa = argList;
    while !aa.as_deref().is_none() {
        if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            break;
        }
        if *(*aa.as_deref().unwrap())
            .name
            .offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && *(*aa.as_deref().unwrap())
                .name
                .offset(1 as libc::c_int as isize) as libc::c_int
                != '-' as i32
        {
            j = 1 as libc::c_int;
            while *(*aa.as_deref().unwrap()).name.offset(j as isize) as libc::c_int != '\0' as i32 {
                match *(*aa.as_deref().unwrap()).name.offset(j as isize) as libc::c_int {
                    99 => {
                        srcMode = 2 as libc::c_int;
                    }
                    100 => {
                        opMode = 2 as libc::c_int;
                    }
                    122 => {
                        opMode = 1 as libc::c_int;
                    }
                    102 => {
                        forceOverwrite = 1 as libc::c_int as Bool;
                    }
                    116 => {
                        opMode = 3 as libc::c_int;
                    }
                    107 => {
                        keepInputFiles = 1 as libc::c_int as Bool;
                    }
                    115 => {
                        smallMode = 1 as libc::c_int as Bool;
                    }
                    113 => {
                        noisy = 0 as libc::c_int as Bool;
                    }
                    49 => {
                        blockSize100k = 1 as libc::c_int;
                    }
                    50 => {
                        blockSize100k = 2 as libc::c_int;
                    }
                    51 => {
                        blockSize100k = 3 as libc::c_int;
                    }
                    52 => {
                        blockSize100k = 4 as libc::c_int;
                    }
                    53 => {
                        blockSize100k = 5 as libc::c_int;
                    }
                    54 => {
                        blockSize100k = 6 as libc::c_int;
                    }
                    55 => {
                        blockSize100k = 7 as libc::c_int;
                    }
                    56 => {
                        blockSize100k = 8 as libc::c_int;
                    }
                    57 => {
                        blockSize100k = 9 as libc::c_int;
                    }
                    86 | 76 => {
                        license();
                    }
                    118 => {
                        verbosity += 1;
                    }
                    104 => {
                        usage(progName);
                        exit(0 as libc::c_int);
                    }
                    _ => {
                        fprintf(
                            stderr,
                            b"%s: Bad flag `%s'\n\0" as *const u8 as *const libc::c_char,
                            progName,
                            (*aa).name,
                        );
                        usage(progName);
                        exit(1 as libc::c_int);
                    }
                }
                j += 1;
            }
        }
        aa = Some(Box::from_raw((*aa.as_deref().unwrap()).link));
    }
    ();
    aa = argList;
    while !aa.as_deref().is_none() {
        if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            break;
        }
        if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--stdout\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            srcMode = 2 as libc::c_int;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--decompress\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opMode = 2 as libc::c_int;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--compress\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opMode = 1 as libc::c_int;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--force\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            forceOverwrite = 1 as libc::c_int as Bool;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--test\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            opMode = 3 as libc::c_int;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--keep\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            keepInputFiles = 1 as libc::c_int as Bool;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--small\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            smallMode = 1 as libc::c_int as Bool;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--quiet\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            noisy = 0 as libc::c_int as Bool;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            license();
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--license\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            license();
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--exponential\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            workFactor = 1 as libc::c_int;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--repetitive-best\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            redundant((*aa.as_deref().unwrap()).name);
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--repetitive-fast\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            redundant((*aa.as_deref().unwrap()).name);
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--fast\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            blockSize100k = 1 as libc::c_int;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--best\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            blockSize100k = 9 as libc::c_int;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--verbose\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            verbosity += 1;
        } else if strcmp(
            (*aa.as_deref().unwrap()).name,
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            usage(progName);
            exit(0 as libc::c_int);
        } else {
            if strncmp(
                (*aa.as_deref().unwrap()).name,
                b"--\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"%s: Bad flag `%s'\n\0" as *const u8 as *const libc::c_char,
                    progName,
                    (*aa).name,
                );
                usage(progName);
                exit(1 as libc::c_int);
            }
        }
        aa = Some(Box::from_raw((*aa.as_deref().unwrap()).link));
    }
    ();
    if verbosity > 4 as libc::c_int {
        verbosity = 4 as libc::c_int;
    }
    if opMode == 1 as libc::c_int
        && smallMode as libc::c_int != 0
        && blockSize100k > 2 as libc::c_int
    {
        blockSize100k = 2 as libc::c_int;
    }
    if opMode == 3 as libc::c_int && srcMode == 2 as libc::c_int {
        fprintf(
            stderr,
            b"%s: -c and -t cannot be used together.\n\0" as *const u8 as *const libc::c_char,
            progName,
        );
        exit(1 as libc::c_int);
    }
    if srcMode == 2 as libc::c_int && numFileNames == 0 as libc::c_int {
        srcMode = 1 as libc::c_int;
    }
    if opMode != 1 as libc::c_int {
        blockSize100k = 0 as libc::c_int;
    }
    if srcMode == 3 as libc::c_int {
        signal(
            2 as libc::c_int,
            Some(mySignalCatcher as unsafe extern "C" fn(IntNative) -> ()),
        );
        signal(
            15 as libc::c_int,
            Some(mySignalCatcher as unsafe extern "C" fn(IntNative) -> ()),
        );
        signal(
            1 as libc::c_int,
            Some(mySignalCatcher as unsafe extern "C" fn(IntNative) -> ()),
        );
    }
    if opMode == 1 as libc::c_int {
        if srcMode == 1 as libc::c_int {
            compress(0 as *mut Char);
        } else {
            decode = 1 as libc::c_int as Bool;
            aa = argList;
            while !aa.as_deref().is_none() {
                if strcmp(
                    (*aa.as_deref().unwrap()).name,
                    b"--\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    decode = 0 as libc::c_int as Bool;
                } else if !(*(*aa.as_deref().unwrap())
                    .name
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                    && decode as libc::c_int != 0)
                {
                    numFilesProcessed += 1;
                    compress((*aa.as_deref().unwrap()).name);
                }
                aa = Some(Box::from_raw((*aa.as_deref().unwrap()).link));
            }
            ();
        }
    } else if opMode == 2 as libc::c_int {
        unzFailsExist = 0 as libc::c_int as Bool;
        if srcMode == 1 as libc::c_int {
            uncompress(0 as *mut Char);
        } else {
            decode = 1 as libc::c_int as Bool;
            aa = argList;
            while !aa.as_deref().is_none() {
                if strcmp(
                    (*aa.as_deref().unwrap()).name,
                    b"--\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    decode = 0 as libc::c_int as Bool;
                } else if !(*(*aa.as_deref().unwrap())
                    .name
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                    && decode as libc::c_int != 0)
                {
                    numFilesProcessed += 1;
                    uncompress((*aa.as_deref().unwrap()).name);
                }
                aa = Some(Box::from_raw((*aa.as_deref().unwrap()).link));
            }
            ();
        }
        if unzFailsExist != 0 {
            setExit(2 as libc::c_int);
            exit(exitValue);
        }
    } else {
        testFailsExist = 0 as libc::c_int as Bool;
        if srcMode == 1 as libc::c_int {
            testf(0 as *mut Char);
        } else {
            decode = 1 as libc::c_int as Bool;
            aa = argList;
            while !aa.as_deref().is_none() {
                if strcmp(
                    (*aa.as_deref().unwrap()).name,
                    b"--\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    decode = 0 as libc::c_int as Bool;
                } else if !(*(*aa.as_deref().unwrap())
                    .name
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                    && decode as libc::c_int != 0)
                {
                    numFilesProcessed += 1;
                    testf((*aa.as_deref().unwrap()).name);
                }
                aa = Some(Box::from_raw((*aa.as_deref().unwrap()).link));
            }
            ();
        }
        if testFailsExist != 0 {
            if noisy != 0 {
                fprintf(
                    stderr,
                    b"\nYou can use the `bzip2recover' program to attempt to recover\ndata from undamaged sections of corrupted files.\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            setExit(2 as libc::c_int);
            exit(exitValue);
        }
    }
    aa = argList;
    while !aa.as_deref().is_none() {
        let mut aa2 = Some(Box::from_raw((*aa.as_deref().unwrap()).link));
        if !(*aa.as_deref().unwrap()).name.is_null() {
            free((*aa.as_deref().unwrap()).name as *mut libc::c_void);
        } else {
            ();
        }
        ();
        aa = aa2;
    }
    ();
    return exitValue;
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
//             main_0((args.len() - 1) as IntNative, args.as_mut_ptr() as *mut *mut Char)
//                 as i32,
//         )
//     }
// }
