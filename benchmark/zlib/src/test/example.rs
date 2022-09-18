#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(
    const_raw_ptr_to_usize_cast,
    const_transmute,
    extern_types,
    main,
    register_tool
)]
use ::rust::*;
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn deflateSetDictionary(
        strm: z_streamp,
        dictionary_0: *const Bytef,
        dictLength: uInt,
    ) -> libc::c_int;
    #[no_mangle]
    fn deflateParams(strm: z_streamp, level: libc::c_int, strategy: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateSetDictionary(
        strm: z_streamp,
        dictionary_0: *const Bytef,
        dictLength: uInt,
    ) -> libc::c_int;
    #[no_mangle]
    fn inflateSync(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn zlibCompileFlags() -> uLong;
    #[no_mangle]
    fn compress(
        dest: *mut Bytef,
        destLen: *mut uLongf,
        source: *const Bytef,
        sourceLen: uLong,
    ) -> libc::c_int;
    #[no_mangle]
    fn uncompress(
        dest: *mut Bytef,
        destLen: *mut uLongf,
        source: *const Bytef,
        sourceLen: uLong,
    ) -> libc::c_int;
    #[no_mangle]
    fn gzread(file: gzFile, buf: voidp, len: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn gzprintf(file: gzFile, format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn gzputs(file: gzFile, s: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn gzgets(file: gzFile, buf: *mut libc::c_char, len: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn gzputc(file: gzFile, c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn gzgetc(file: gzFile) -> libc::c_int;
    #[no_mangle]
    fn gzungetc(c: libc::c_int, file: gzFile) -> libc::c_int;
    #[no_mangle]
    fn gzclose(file: gzFile) -> libc::c_int;
    #[no_mangle]
    fn gzerror(file: gzFile, errnum: *mut libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn deflateInit_(
        strm: z_streamp,
        level: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn inflateInit_(
        strm: z_streamp,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn gzopen64(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
    #[no_mangle]
    fn gzseek64(_: gzFile, _: off64_t, _: libc::c_int) -> off64_t;
    #[no_mangle]
    fn gztell64(_: gzFile) -> off64_t;
    #[no_mangle]
    fn zlibVersion() -> *const libc::c_char;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type uLongf = uLong;
pub type voidpf = *mut libc::c_void;
pub type voidp = *mut libc::c_void;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off64_t;
pub type off64_t = __off64_t;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off64_t,
}
pub type gzFile = *mut gzFile_s;
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
/* zlib.h -- interface of the 'zlib' general purpose compression library
  version 1.2.11, January 15th, 2017

  Copyright (C) 1995-2017 Jean-loup Gailly and Mark Adler

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

  Jean-loup Gailly        Mark Adler
  jloup@gzip.org          madler@alumni.caltech.edu


  The data format used by the zlib library is described by RFCs (Request for
  Comments) 1950 to 1952 in the files http://tools.ietf.org/html/rfc1950
  (zlib format), rfc1951 (deflate format) and rfc1952 (gzip format).
*/
/*
    The 'zlib' compression library provides in-memory compression and
  decompression functions, including integrity checks of the uncompressed data.
  This version of the library supports only one compression method (deflation)
  but other algorithms will be added later and will have the same stream
  interface.

    Compression can be done in a single step if the buffers are large enough,
  or can be done by repeated calls of the compression function.  In the latter
  case, the application must provide more input and/or consume the output
  (providing more output space) before each call.

    The compressed data format used by default by the in-memory functions is
  the zlib format, which is a zlib wrapper documented in RFC 1950, wrapped
  around a deflate stream, which is itself documented in RFC 1951.

    The library also supports reading and writing files in gzip (.gz) format
  with an interface similar to that of stdio using the functions that start
  with "gz".  The gzip format is different from the zlib format.  gzip is a
  gzip wrapper, documented in RFC 1952, wrapped around a deflate stream.

    This library can optionally read and write gzip and raw deflate streams in
  memory as well.

    The zlib format was designed to be compact and fast for use in memory
  and on communications channels.  The gzip format was designed for single-
  file compression on file systems, has a larger header than zlib to maintain
  directory information, and uses a different, slower check method than zlib.

    The library does not install any signal handler.  The decoder checks
  the consistency of the compressed data, so the library should never crash
  even in the case of corrupted input.
*/
/* next input byte */
/* number of bytes available at next_in */
/* total number of input bytes read so far */
/* next output byte will go here */
/* remaining free space at next_out */
/* total number of bytes output so far */
/* last error message, NULL if no error */
/* not visible by applications */
/* used to allocate the internal state */
/* used to free the internal state */
/* private data object passed to zalloc and zfree */
/* best guess about the data type: binary or text
for deflate, or the decoding state for inflate */
/* Adler-32 or CRC-32 value of the uncompressed data */
/* reserved for future use */
/*
     gzip header information passed to and from zlib routines.  See RFC 1952
  for more details on the meanings of these fields.
*/
/* true if compressed data believed to be text */
/* modification time */
/* extra flags (not used when writing a gzip file) */
/* operating system */
/* pointer to extra field or Z_NULL if none */
/* extra field length (valid if extra != Z_NULL) */
/* space at extra (only when reading header) */
/* pointer to zero-terminated file name or Z_NULL */
/* space at name (only when reading header) */
/* pointer to zero-terminated comment or Z_NULL */
/* space at comment (only when reading header) */
/* true if there was or will be a header crc */
/* true when done reading gzip header (not used
when writing a gzip file) */
/*
     The application must update next_in and avail_in when avail_in has dropped
   to zero.  It must update next_out and avail_out when avail_out has dropped
   to zero.  The application must initialize zalloc, zfree and opaque before
   calling the init function.  All other fields are set by the compression
   library and must not be updated by the application.

     The opaque value provided by the application will be passed as the first
   parameter for calls of zalloc and zfree.  This can be useful for custom
   memory management.  The compression library attaches no meaning to the
   opaque value.

     zalloc must return Z_NULL if there is not enough memory for the object.
   If zlib is used in a multi-threaded application, zalloc and zfree must be
   thread safe.  In that case, zlib is thread-safe.  When zalloc and zfree are
   Z_NULL on entry to the initialization function, they are set to internal
   routines that use the standard library functions malloc() and free().

     On 16-bit systems, the functions zalloc and zfree must be able to allocate
   exactly 65536 bytes, but will not be required to allocate more than this if
   the symbol MAXSEG_64K is defined (see zconf.h).  WARNING: On MSDOS, pointers
   returned by zalloc for objects of exactly 65536 bytes *must* have their
   offset normalized to zero.  The default allocation function provided by this
   library ensures this (see zutil.c).  To reduce memory requirements and avoid
   any allocation of 64K objects, at the expense of compression ratio, compile
   the library with -DMAX_WBITS=14 (see zconf.h).

     The fields total_in and total_out can be used for statistics or progress
   reports.  After compression, total_in holds the total size of the
   uncompressed data and may be saved for use by the decompressor (particularly
   if the decompressor wants to decompress everything in a single step).
*/
/* constants */
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERNUM: libc::c_int = 0x12b0 as libc::c_int;
pub const Z_NO_COMPRESSION: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_FILTERED: libc::c_int = 1 as libc::c_int;
pub const Z_BEST_COMPRESSION: libc::c_int = 9 as libc::c_int;
pub const Z_DEFAULT_STRATEGY: libc::c_int = 0 as libc::c_int;
pub const Z_FULL_FLUSH: libc::c_int = 3 as libc::c_int;
pub const Z_DATA_ERROR: libc::c_int = -(3 as libc::c_int);
pub const Z_NULL: libc::c_int = 0 as libc::c_int;
pub const Z_NEED_DICT: libc::c_int = 2 as libc::c_int;
pub const gzopen: unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> gzFile =
    gzopen64;
pub const gzseek: unsafe extern "C" fn(_: gzFile, _: off64_t, _: libc::c_int) -> off64_t = gzseek64;
pub const gztell: unsafe extern "C" fn(_: gzFile) -> off64_t = gztell64;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
/* example.c -- usage example of the zlib compression library
 * Copyright (C) 1995-2006, 2011, 2016 Jean-loup Gailly
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
pub const TESTFILE: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"foo.gz\x00") };
static mut hello: [libc::c_char; 14] =
    unsafe { *::std::mem::transmute::<&[u8; 14], &mut [libc::c_char; 14]>(b"hello, hello!\x00") };
/* "hello world" would be more standard, but the repeated "hello"
 * stresses the compression code better, sorry...
 */
static mut dictionary: [libc::c_char; 6] =
    unsafe { *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"hello\x00") };
static mut dictId: uLong = 0;
/* !Z_SOLO */
static mut zalloc: alloc_func = None;
static mut zfree: free_func = None;
/* ===========================================================================
 * Test compress() and uncompress()
 */
#[no_mangle]
pub unsafe extern "C" fn test_compress(
    mut compr: *mut Byte,
    mut comprLen: uLong,
    mut uncompr: *mut Byte,
    mut uncomprLen: uLong,
) {
    let mut err: libc::c_int = 0;
    let mut len: uLong = strlen(hello.as_mut_ptr()).wrapping_add(1 as libc::c_int as libc::c_ulong);
    err = compress(
        compr,
        &mut comprLen,
        hello.as_mut_ptr() as *const Bytef,
        len,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"compress\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    strcpy(
        uncompr as *mut libc::c_char,
        b"garbage\x00" as *const u8 as *const libc::c_char,
    );
    err = uncompress(uncompr, &mut uncomprLen, compr, comprLen);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"uncompress\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    if strcmp(uncompr as *mut libc::c_char, hello.as_mut_ptr()) != 0 {
        fprintf(
            stderr,
            b"bad uncompress\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        printf(
            b"uncompress(): %s\n\x00" as *const u8 as *const libc::c_char,
            uncompr as *mut libc::c_char,
        );
    };
}
/* ===========================================================================
 * Test read/write of .gz files
 */
#[no_mangle]
pub unsafe extern "C" fn test_gzio(
    mut fname: *const libc::c_char,
    mut uncompr: *mut Byte,
    mut uncomprLen: uLong,
) {
    let mut err: libc::c_int = 0; /* add one zero byte */
    let mut len: libc::c_int = strlen(hello.as_mut_ptr()) as libc::c_int + 1 as libc::c_int;
    let mut file: gzFile = 0 as *mut gzFile_s;
    let mut pos: off_t = 0;
    file = gzopen64(fname, b"wb\x00" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fprintf(
            stderr,
            b"gzopen error\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    gzputc(file, 'h' as i32);
    if gzputs(file, b"ello\x00" as *const u8 as *const libc::c_char) != 4 as libc::c_int {
        fprintf(
            stderr,
            b"gzputs err: %s\n\x00" as *const u8 as *const libc::c_char,
            gzerror(file, &mut err),
        );
        exit(1 as libc::c_int);
    }
    if gzprintf(
        file,
        b", %s!\x00" as *const u8 as *const libc::c_char,
        b"hello\x00" as *const u8 as *const libc::c_char,
    ) != 8 as libc::c_int
    {
        fprintf(
            stderr,
            b"gzprintf err: %s\n\x00" as *const u8 as *const libc::c_char,
            gzerror(file, &mut err),
        );
        exit(1 as libc::c_int);
    }
    gzseek64(file, 1 as libc::c_long, SEEK_CUR);
    gzclose(file);
    file = gzopen64(fname, b"rb\x00" as *const u8 as *const libc::c_char);
    if file.is_null() {
        fprintf(
            stderr,
            b"gzopen error\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    strcpy(
        uncompr as *mut libc::c_char,
        b"garbage\x00" as *const u8 as *const libc::c_char,
    );
    if gzread(file, uncompr as voidp, uncomprLen as libc::c_uint) != len {
        fprintf(
            stderr,
            b"gzread err: %s\n\x00" as *const u8 as *const libc::c_char,
            gzerror(file, &mut err),
        );
        exit(1 as libc::c_int);
    }
    if strcmp(uncompr as *mut libc::c_char, hello.as_mut_ptr()) != 0 {
        fprintf(
            stderr,
            b"bad gzread: %s\n\x00" as *const u8 as *const libc::c_char,
            uncompr as *mut libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        printf(
            b"gzread(): %s\n\x00" as *const u8 as *const libc::c_char,
            uncompr as *mut libc::c_char,
        );
    }
    pos = gzseek64(file, -(8 as libc::c_long), SEEK_CUR);
    if pos != 6 as libc::c_int as libc::c_long || gztell64(file) != pos {
        fprintf(
            stderr,
            b"gzseek error, pos=%ld, gztell=%ld\n\x00" as *const u8 as *const libc::c_char,
            pos,
            gztell64(file),
        );
        exit(1 as libc::c_int);
    }
    if (if (*file).have != 0 {
        (*file).have = (*file).have.wrapping_sub(1);
        (*file).pos += 1;
        let fresh0 = (*file).next;
        (*file).next = (*file).next.offset(1);
        *fresh0 as libc::c_int
    } else {
        gzgetc(file)
    }) != ' ' as i32
    {
        fprintf(
            stderr,
            b"gzgetc error\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if gzungetc(' ' as i32, file) != ' ' as i32 {
        fprintf(
            stderr,
            b"gzungetc error\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    gzgets(
        file,
        uncompr as *mut libc::c_char,
        uncomprLen as libc::c_int,
    );
    if strlen(uncompr as *mut libc::c_char) != 7 as libc::c_int as libc::c_ulong {
        /* " hello!" */
        fprintf(
            stderr,
            b"gzgets err after gzseek: %s\n\x00" as *const u8 as *const libc::c_char,
            gzerror(file, &mut err),
        );
        exit(1 as libc::c_int);
    }
    if strcmp(
        uncompr as *mut libc::c_char,
        hello.as_mut_ptr().offset(6 as libc::c_int as isize),
    ) != 0
    {
        fprintf(
            stderr,
            b"bad gzgets after gzseek\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        printf(
            b"gzgets() after gzseek: %s\n\x00" as *const u8 as *const libc::c_char,
            uncompr as *mut libc::c_char,
        );
    }
    gzclose(file);
}
/* Adler32 value of the dictionary */
/* Z_SOLO */
/* ===========================================================================
 * Test deflate() with small buffers
 */
#[no_mangle]
pub unsafe extern "C" fn test_deflate(mut compr: *mut Byte, mut comprLen: uLong) {
    let mut c_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; /* compression stream */
    let mut err: libc::c_int = 0; /* force small buffers */
    let mut len: uLong = strlen(hello.as_mut_ptr()).wrapping_add(1 as libc::c_int as libc::c_ulong);
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err = deflateInit_(
        &mut c_stream,
        -(1 as libc::c_int),
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    c_stream.next_in = hello.as_mut_ptr() as *mut libc::c_uchar;
    c_stream.next_out = compr;
    while c_stream.total_in != len && c_stream.total_out < comprLen {
        c_stream.avail_out = 1 as libc::c_int as uInt;
        c_stream.avail_in = c_stream.avail_out;
        err = deflate(&mut c_stream, Z_NO_FLUSH);
        if err != Z_OK {
            fprintf(
                stderr,
                b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
                b"deflate\x00" as *const u8 as *const libc::c_char,
                err,
            );
            exit(1 as libc::c_int);
        }
    }
    loop
    /* Finish the stream, still forcing small buffers: */
    {
        c_stream.avail_out = 1 as libc::c_int as uInt;
        err = deflate(&mut c_stream, Z_FINISH);
        if err == Z_STREAM_END {
            break;
        }
        if err != Z_OK {
            fprintf(
                stderr,
                b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
                b"deflate\x00" as *const u8 as *const libc::c_char,
                err,
            );
            exit(1 as libc::c_int);
        }
    }
    err = deflateEnd(&mut c_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    };
}
/* ===========================================================================
 * Test inflate() with small buffers
 */
#[no_mangle]
pub unsafe extern "C" fn test_inflate(
    mut compr: *mut Byte,
    mut comprLen: uLong,
    mut uncompr: *mut Byte,
    mut uncomprLen: uLong,
) {
    let mut err: libc::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; /* force small buffers */
    strcpy(
        uncompr as *mut libc::c_char,
        b"garbage\x00" as *const u8 as *const libc::c_char,
    );
    d_stream.zalloc = zalloc;
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = 0 as libc::c_int as uInt;
    d_stream.next_out = uncompr;
    err = inflateInit_(
        &mut d_stream,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    while d_stream.total_out < uncomprLen && d_stream.total_in < comprLen {
        d_stream.avail_out = 1 as libc::c_int as uInt;
        d_stream.avail_in = d_stream.avail_out;
        err = inflate(&mut d_stream, Z_NO_FLUSH);
        if err == Z_STREAM_END {
            break;
        }
        if err != Z_OK {
            fprintf(
                stderr,
                b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
                b"inflate\x00" as *const u8 as *const libc::c_char,
                err,
            );
            exit(1 as libc::c_int);
        }
    }
    err = inflateEnd(&mut d_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    if strcmp(uncompr as *mut libc::c_char, hello.as_mut_ptr()) != 0 {
        fprintf(
            stderr,
            b"bad inflate\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        printf(
            b"inflate(): %s\n\x00" as *const u8 as *const libc::c_char,
            uncompr as *mut libc::c_char,
        );
    };
}
/* ===========================================================================
 * Test deflate() with large buffers and dynamic change of compression level
 */
#[no_mangle]
pub unsafe extern "C" fn test_large_deflate(
    mut compr: *mut Byte,
    mut comprLen: uLong,
    mut uncompr: *mut Byte,
    mut uncomprLen: uLong,
) {
    let mut c_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; /* compression stream */
    let mut err: libc::c_int = 0;
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err = deflateInit_(
        &mut c_stream,
        1 as libc::c_int,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    c_stream.next_out = compr;
    c_stream.avail_out = comprLen as uInt;
    /* At this point, uncompr is still mostly zeroes, so it should compress
     * very well:
     */
    c_stream.next_in = uncompr;
    c_stream.avail_in = uncomprLen as uInt;
    err = deflate(&mut c_stream, Z_NO_FLUSH);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflate\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    if c_stream.avail_in != 0 as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            b"deflate not greedy\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    /* Feed in already compressed data and switch to no compression: */
    deflateParams(&mut c_stream, Z_NO_COMPRESSION, Z_DEFAULT_STRATEGY);
    c_stream.next_in = compr;
    c_stream.avail_in = (comprLen as uInt).wrapping_div(2 as libc::c_int as libc::c_uint);
    err = deflate(&mut c_stream, Z_NO_FLUSH);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflate\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    /* Switch back to compressing mode: */
    deflateParams(&mut c_stream, Z_BEST_COMPRESSION, Z_FILTERED);
    c_stream.next_in = uncompr;
    c_stream.avail_in = uncomprLen as uInt;
    err = deflate(&mut c_stream, Z_NO_FLUSH);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflate\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    err = deflate(&mut c_stream, Z_FINISH);
    if err != Z_STREAM_END {
        fprintf(
            stderr,
            b"deflate should report Z_STREAM_END\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    err = deflateEnd(&mut c_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    };
}
/* ===========================================================================
 * Test inflate() with large buffers
 */
#[no_mangle]
pub unsafe extern "C" fn test_large_inflate(
    mut compr: *mut Byte,
    mut comprLen: uLong,
    mut uncompr: *mut Byte,
    mut uncomprLen: uLong,
) {
    let mut err: libc::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; /* discard the output */
    strcpy(
        uncompr as *mut libc::c_char,
        b"garbage\x00" as *const u8 as *const libc::c_char,
    );
    d_stream.zalloc = zalloc;
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = comprLen as uInt;
    err = inflateInit_(
        &mut d_stream,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    loop {
        d_stream.next_out = uncompr;
        d_stream.avail_out = uncomprLen as uInt;
        err = inflate(&mut d_stream, Z_NO_FLUSH);
        if err == Z_STREAM_END {
            break;
        }
        if err != Z_OK {
            fprintf(
                stderr,
                b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
                b"large inflate\x00" as *const u8 as *const libc::c_char,
                err,
            );
            exit(1 as libc::c_int);
        }
    }
    err = inflateEnd(&mut d_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    if d_stream.total_out
        != (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(uncomprLen)
            .wrapping_add(comprLen.wrapping_div(2 as libc::c_int as libc::c_ulong))
    {
        fprintf(
            stderr,
            b"bad large inflate: %ld\n\x00" as *const u8 as *const libc::c_char,
            d_stream.total_out,
        );
        exit(1 as libc::c_int);
    } else {
        printf(b"large_inflate(): OK\n\x00" as *const u8 as *const libc::c_char);
    };
}
/* ===========================================================================
 * Test deflate() with full flush
 */
#[no_mangle]
pub unsafe extern "C" fn test_flush(mut compr: *mut Byte, mut comprLen: *mut uLong) {
    let mut c_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; /* compression stream */
    let mut err: libc::c_int = 0; /* force an error in first compressed block */
    let mut len: uInt =
        (strlen(hello.as_mut_ptr()) as uInt).wrapping_add(1 as libc::c_int as libc::c_uint);
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err = deflateInit_(
        &mut c_stream,
        -(1 as libc::c_int),
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    c_stream.next_in = hello.as_mut_ptr() as *mut libc::c_uchar;
    c_stream.next_out = compr;
    c_stream.avail_in = 3 as libc::c_int as uInt;
    c_stream.avail_out = *comprLen as uInt;
    err = deflate(&mut c_stream, Z_FULL_FLUSH);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflate\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    let ref mut fresh1 = *compr.offset(3 as libc::c_int as isize);
    *fresh1 = (*fresh1).wrapping_add(1);
    c_stream.avail_in = len.wrapping_sub(3 as libc::c_int as libc::c_uint);
    err = deflate(&mut c_stream, Z_FINISH);
    if err != Z_STREAM_END {
        if err != Z_OK {
            fprintf(
                stderr,
                b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
                b"deflate\x00" as *const u8 as *const libc::c_char,
                err,
            );
            exit(1 as libc::c_int);
        }
    }
    err = deflateEnd(&mut c_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    *comprLen = c_stream.total_out;
}
/* ===========================================================================
 * Test inflateSync()
 */
#[no_mangle]
pub unsafe extern "C" fn test_sync(
    mut compr: *mut Byte,
    mut comprLen: uLong,
    mut uncompr: *mut Byte,
    mut uncomprLen: uLong,
) {
    let mut err: libc::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; /* just read the zlib header */
    strcpy(
        uncompr as *mut libc::c_char,
        b"garbage\x00" as *const u8 as *const libc::c_char,
    ); /* read all compressed data */
    d_stream.zalloc = zalloc; /* but skip the damaged part */
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = 2 as libc::c_int as uInt;
    err = inflateInit_(
        &mut d_stream,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    d_stream.next_out = uncompr;
    d_stream.avail_out = uncomprLen as uInt;
    err = inflate(&mut d_stream, Z_NO_FLUSH);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflate\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    d_stream.avail_in = (comprLen as uInt).wrapping_sub(2 as libc::c_int as libc::c_uint);
    err = inflateSync(&mut d_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateSync\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    err = inflate(&mut d_stream, Z_FINISH);
    if err != Z_DATA_ERROR {
        fprintf(
            stderr,
            b"inflate should report DATA_ERROR\n\x00" as *const u8 as *const libc::c_char,
        );
        /* Because of incorrect adler32 */
        exit(1 as libc::c_int);
    }
    err = inflateEnd(&mut d_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    printf(
        b"after inflateSync(): hel%s\n\x00" as *const u8 as *const libc::c_char,
        uncompr as *mut libc::c_char,
    );
}
/* ===========================================================================
 * Test deflate() with preset dictionary
 */
#[no_mangle]
pub unsafe extern "C" fn test_dict_deflate(mut compr: *mut Byte, mut comprLen: uLong) {
    let mut c_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    }; /* compression stream */
    let mut err: libc::c_int = 0;
    c_stream.zalloc = zalloc;
    c_stream.zfree = zfree;
    c_stream.opaque = 0 as voidpf;
    err = deflateInit_(
        &mut c_stream,
        9 as libc::c_int,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    err = deflateSetDictionary(
        &mut c_stream,
        dictionary.as_ptr() as *const Bytef,
        ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as libc::c_int as uInt,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateSetDictionary\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    dictId = c_stream.adler;
    c_stream.next_out = compr;
    c_stream.avail_out = comprLen as uInt;
    c_stream.next_in = hello.as_mut_ptr() as *mut libc::c_uchar;
    c_stream.avail_in =
        (strlen(hello.as_mut_ptr()) as uInt).wrapping_add(1 as libc::c_int as libc::c_uint);
    err = deflate(&mut c_stream, Z_FINISH);
    if err != Z_STREAM_END {
        fprintf(
            stderr,
            b"deflate should report Z_STREAM_END\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    err = deflateEnd(&mut c_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"deflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    };
}
/* ===========================================================================
 * Test inflate() with a preset dictionary
 */
#[no_mangle]
pub unsafe extern "C" fn test_dict_inflate(
    mut compr: *mut Byte,
    mut comprLen: uLong,
    mut uncompr: *mut Byte,
    mut uncomprLen: uLong,
) {
    let mut err: libc::c_int = 0; /* decompression stream */
    let mut d_stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    strcpy(
        uncompr as *mut libc::c_char,
        b"garbage\x00" as *const u8 as *const libc::c_char,
    );
    d_stream.zalloc = zalloc;
    d_stream.zfree = zfree;
    d_stream.opaque = 0 as voidpf;
    d_stream.next_in = compr;
    d_stream.avail_in = comprLen as uInt;
    err = inflateInit_(
        &mut d_stream,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateInit\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    d_stream.next_out = uncompr;
    d_stream.avail_out = uncomprLen as uInt;
    loop {
        err = inflate(&mut d_stream, Z_NO_FLUSH);
        if err == Z_STREAM_END {
            break;
        }
        if err == Z_NEED_DICT {
            if d_stream.adler != dictId {
                fprintf(
                    stderr,
                    b"unexpected dictionary\x00" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            err = inflateSetDictionary(
                &mut d_stream,
                dictionary.as_ptr() as *const Bytef,
                ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as libc::c_int as uInt,
            )
        }
        if err != Z_OK {
            fprintf(
                stderr,
                b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
                b"inflate with dict\x00" as *const u8 as *const libc::c_char,
                err,
            );
            exit(1 as libc::c_int);
        }
    }
    err = inflateEnd(&mut d_stream);
    if err != Z_OK {
        fprintf(
            stderr,
            b"%s error: %d\n\x00" as *const u8 as *const libc::c_char,
            b"inflateEnd\x00" as *const u8 as *const libc::c_char,
            err,
        );
        exit(1 as libc::c_int);
    }
    if strcmp(uncompr as *mut libc::c_char, hello.as_mut_ptr()) != 0 {
        fprintf(
            stderr,
            b"bad inflate with dict\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        printf(
            b"inflate with dictionary: %s\n\x00" as *const u8 as *const libc::c_char,
            uncompr as *mut libc::c_char,
        );
    };
}
/* ===========================================================================
 * Usage:  example [output.gz  [input.gz]]
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut compr: *mut Byte = 0 as *mut Byte; /* don't overflow on MSDOS */
    let mut uncompr: *mut Byte = 0 as *mut Byte;
    let mut comprLen: uLong = (10000 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong);
    let mut uncomprLen: uLong = comprLen;
    static mut myVersion: *const libc::c_char = ZLIB_VERSION.as_ptr();
    if *zlibVersion().offset(0 as libc::c_int as isize) as libc::c_int
        != *myVersion.offset(0 as libc::c_int as isize) as libc::c_int
    {
        fprintf(
            stderr,
            b"incompatible zlib version\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        if strcmp(zlibVersion(), ZLIB_VERSION.as_ptr()) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"warning: different zlib version\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    printf(
        b"zlib version %s = 0x%04x, compile flags = 0x%lx\n\x00" as *const u8
            as *const libc::c_char,
        ZLIB_VERSION.as_ptr(),
        ZLIB_VERNUM,
        zlibCompileFlags(),
    );
    compr = calloc(
        comprLen as uInt as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut Byte;
    uncompr = calloc(
        uncomprLen as uInt as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut Byte;
    /* compr and uncompr are cleared to avoid reading uninitialized
     * data and to ensure that uncompr compresses well.
     */
    if compr.is_null() || uncompr.is_null() {
        printf(b"out of memory\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    test_compress(compr, comprLen, uncompr, uncomprLen);
    test_gzio(
        if argc > 1 as libc::c_int {
            *argv.offset(1 as libc::c_int as isize)
        } else {
            TESTFILE.as_ptr()
        },
        uncompr,
        uncomprLen,
    );
    test_deflate(compr, comprLen);
    test_inflate(compr, comprLen, uncompr, uncomprLen);
    test_large_deflate(compr, comprLen, uncompr, uncomprLen);
    test_large_inflate(compr, comprLen, uncompr, uncomprLen);
    test_flush(compr, &mut comprLen);
    test_sync(compr, comprLen, uncompr, uncomprLen);
    comprLen = uncomprLen;
    test_dict_deflate(compr, comprLen);
    test_dict_inflate(compr, comprLen, uncompr, uncomprLen);
    free(compr as *mut libc::c_void);
    free(uncompr as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
