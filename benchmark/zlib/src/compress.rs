use ::libc;
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn deflateInit_(
        strm: z_streamp,
        level: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type uLongf = uLong;
pub type voidpf = *mut libc::c_void;
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
/* Allowed flush values; see deflate() and inflate() below for details */
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_DEFAULT_COMPRESSION: libc::c_int = -(1 as libc::c_int);
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
/* compress.c -- compress a memory buffer
 * Copyright (C) 1995-2005, 2014, 2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* ===========================================================================
     Compresses the source buffer into the destination buffer. The level
   parameter has the same meaning as in deflateInit.  sourceLen is the byte
   length of the source buffer. Upon entry, destLen is the total size of the
   destination buffer, which must be at least 0.1% larger than sourceLen plus
   12 bytes. Upon exit, destLen is the actual size of the compressed buffer.

     compress2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_BUF_ERROR if there was not enough room in the output buffer,
   Z_STREAM_ERROR if the level parameter is invalid.
*/
#[no_mangle]
pub unsafe extern "C" fn compress2(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut stream: z_stream = z_stream {
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
    let mut err: libc::c_int = 0;
    let max: uInt = -(1 as libc::c_int) as uInt;
    let mut left: uLong = 0;
    left = *destLen;
    *destLen = 0 as libc::c_int as uLongf;
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = 0 as voidpf;
    err = deflateInit_(
        &mut stream,
        level,
        ZLIB_VERSION.as_ptr(),
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != Z_OK {
        return err;
    }
    stream.next_out = dest;
    stream.avail_out = 0 as libc::c_int as uInt;
    stream.next_in = source as *mut Bytef;
    stream.avail_in = 0 as libc::c_int as uInt;
    loop {
        if stream.avail_out == 0 as libc::c_int as libc::c_uint {
            stream.avail_out = if left > max as uLong {
                max
            } else {
                left as uInt
            };
            left = (left as libc::c_ulong).wrapping_sub(stream.avail_out as libc::c_ulong) as uLong
                as uLong
        }
        if stream.avail_in == 0 as libc::c_int as libc::c_uint {
            stream.avail_in = if sourceLen > max as uLong {
                max
            } else {
                sourceLen as uInt
            };
            sourceLen = (sourceLen as libc::c_ulong).wrapping_sub(stream.avail_in as libc::c_ulong)
                as uLong as uLong
        }
        err = deflate(
            &mut stream,
            if sourceLen != 0 { Z_NO_FLUSH } else { Z_FINISH },
        );
        if !(err == Z_OK) {
            break;
        }
    }
    *destLen = stream.total_out;
    deflateEnd(&mut stream);
    return if err == Z_STREAM_END { Z_OK } else { err };
}
/* ===========================================================================
 */
#[no_mangle]
pub unsafe extern "C" fn compress(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
) -> libc::c_int {
    return compress2(dest, destLen, source, sourceLen, Z_DEFAULT_COMPRESSION);
}
/* ===========================================================================
    If the default memLevel or windowBits for deflateInit() is changed, then
  this function needs to be updated.
*/
#[no_mangle]
pub unsafe extern "C" fn compressBound(mut sourceLen: uLong) -> uLong {
    return sourceLen
        .wrapping_add(sourceLen >> 12 as libc::c_int)
        .wrapping_add(sourceLen >> 14 as libc::c_int)
        .wrapping_add(sourceLen >> 25 as libc::c_int)
        .wrapping_add(13 as libc::c_int as libc::c_ulong);
}
