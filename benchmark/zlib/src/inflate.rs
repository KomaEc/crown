use ::libc;
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn zcfree(opaque: voidpf, ptr: voidpf);
    #[no_mangle]
    fn zcalloc(opaque: voidpf, items: libc::c_uint, size: libc::c_uint) -> voidpf;
    #[no_mangle]
    fn inflate_table(
        type_0: codetype,
        lens: *mut libc::c_ushort,
        codes: libc::c_uint,
        table: *mut *mut code,
        bits: *mut libc::c_uint,
        work: *mut libc::c_ushort,
    ) -> libc::c_int;
    /* inffast.h -- header to use inffast.c
     * Copyright (C) 1995-2003, 2010 Mark Adler
     * For conditions of distribution and use, see copyright notice in zlib.h
     */
    /* WARNING: this file should *not* be used by applications. It is
      part of the implementation of the compression library and is
      subject to change. Applications should only use zlib.h.
    */
    #[no_mangle]
    fn inflate_fast(strm: z_streamp, start: libc::c_uint);
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
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
pub struct gz_header_s {
    pub text: libc::c_int,
    pub time: uLong,
    pub xflags: libc::c_int,
    pub os: libc::c_int,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: libc::c_int,
    pub done: libc::c_int,
}
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
pub type gz_header = gz_header_s;
pub type gz_headerp = *mut gz_header;
pub const COPY_: inflate_mode = 16194;
pub type inflate_mode = libc::c_uint;
pub const SYNC: inflate_mode = 16211;
pub const MEM: inflate_mode = 16210;
pub const BAD: inflate_mode = 16209;
pub const DONE: inflate_mode = 16208;
pub const LENGTH: inflate_mode = 16207;
pub const CHECK: inflate_mode = 16206;
pub const LIT: inflate_mode = 16205;
pub const MATCH: inflate_mode = 16204;
pub const DISTEXT: inflate_mode = 16203;
pub const DIST: inflate_mode = 16202;
pub const LENEXT: inflate_mode = 16201;
pub const LEN: inflate_mode = 16200;
pub const LEN_: inflate_mode = 16199;
pub const CODELENS: inflate_mode = 16198;
pub const LENLENS: inflate_mode = 16197;
pub const TABLE: inflate_mode = 16196;
pub const COPY: inflate_mode = 16195;
pub const STORED: inflate_mode = 16193;
pub const TYPEDO: inflate_mode = 16192;
pub const TYPE: inflate_mode = 16191;
pub const DICT: inflate_mode = 16190;
pub const DICTID: inflate_mode = 16189;
pub const HCRC: inflate_mode = 16188;
pub const COMMENT: inflate_mode = 16187;
pub const NAME: inflate_mode = 16186;
pub const EXTRA: inflate_mode = 16185;
pub const EXLEN: inflate_mode = 16184;
pub const OS: inflate_mode = 16183;
pub const TIME: inflate_mode = 16182;
pub const FLAGS: inflate_mode = 16181;
pub const HEAD: inflate_mode = 16180;
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
   State transitions between above modes -

   (most modes can go to BAD or MEM on error -- not shown for clarity)

   Process header:
       HEAD -> (gzip) or (zlib) or (raw)
       (gzip) -> FLAGS -> TIME -> OS -> EXLEN -> EXTRA -> NAME -> COMMENT ->
                 HCRC -> TYPE
       (zlib) -> DICTID or TYPE
       DICTID -> DICT -> TYPE
       (raw) -> TYPEDO
   Read deflate blocks:
           TYPE -> TYPEDO -> STORED or TABLE or LEN_ or CHECK
           STORED -> COPY_ -> COPY -> TYPE
           TABLE -> LENLENS -> CODELENS -> LEN_
           LEN_ -> LEN
   Read deflate codes in fixed or dynamic block:
               LEN -> LENEXT or LIT or TYPE
               LENEXT -> DIST -> DISTEXT -> MATCH -> LEN
               LIT -> LEN
   Process trailer:
       CHECK -> LENGTH -> DONE
*/
/* State maintained between inflate() calls -- approximately 7K bytes, not
including the allocated sliding window, which is up to 32K bytes. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub strm: z_streamp,
    pub mode: inflate_mode,
    pub last: libc::c_int,
    pub wrap: libc::c_int,
    pub havedict: libc::c_int,
    pub flags: libc::c_int,
    pub dmax: libc::c_uint,
    pub check: libc::c_ulong,
    pub total: libc::c_ulong,
    pub head: gz_headerp,
    pub wbits: libc::c_uint,
    pub wsize: libc::c_uint,
    pub whave: libc::c_uint,
    pub wnext: libc::c_uint,
    pub window: *mut libc::c_uchar,
    pub hold: libc::c_ulong,
    pub bits: libc::c_uint,
    pub length: libc::c_uint,
    pub offset: libc::c_uint,
    pub extra: libc::c_uint,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: libc::c_uint,
    pub distbits: libc::c_uint,
    pub ncode: libc::c_uint,
    pub nlen: libc::c_uint,
    pub ndist: libc::c_uint,
    pub have: libc::c_uint,
    pub next: *mut code,
    pub lens: [libc::c_ushort; 320],
    pub work: [libc::c_ushort; 288],
    pub codes: [code; 1444],
    pub sane: libc::c_int,
    pub back: libc::c_int,
    pub was: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: libc::c_uchar,
    pub bits: libc::c_uchar,
    pub val: libc::c_ushort,
}
pub type codetype = libc::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
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
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
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
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_BLOCK: libc::c_int = 5 as libc::c_int;
pub const Z_TREES: libc::c_int = 6 as libc::c_int;
/* Allowed flush values; see deflate() and inflate() below for details */
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_NEED_DICT: libc::c_int = 2 as libc::c_int;
pub const Z_BUF_ERROR: libc::c_int = -(5 as libc::c_int);
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
/* indexed by 2-zlib_error */
/* (size given to avoid silly warnings with Visual C++) */
/* To be used only when the state is known to be valid */
/* common constants */
/* default windowBits for decompression. MAX_WBITS is for compression only */
/* default memLevel */
/* The three kinds of block type */
/* The minimum and maximum match lengths */
/* preset dictionary flag in zlib header */
/* target dependencies */
/* provide prototypes for these when building zlib without LFS */
/* common defaults */
/* assume Unix */
/* functions */
/* MSDOS small or medium model */
pub const zmemcpy: unsafe extern "C" fn(
    _: *mut libc::c_void,
    _: *const libc::c_void,
    _: libc::c_ulong,
) -> *mut libc::c_void = memcpy;
pub const Z_NULL: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_ERROR: libc::c_int = -(2 as libc::c_int);
pub const Z_DATA_ERROR: libc::c_int = -(3 as libc::c_int);
pub const Z_DEFLATED: libc::c_int = 8 as libc::c_int;
pub const MAX_WBITS: libc::c_int = 15 as libc::c_int;
pub const DEF_WBITS: libc::c_int = MAX_WBITS;
pub const Z_VERSION_ERROR: libc::c_int = -(6 as libc::c_int);
/* op values as set by inflate_table():
   00000000 - literal
   0000tttt - table link, tttt != 0 is the number of table index bits
   0001eeee - length or distance, eeee is the number of extra bits
   01100000 - end of block
   01000000 - invalid code
*/
/* Maximum size of the dynamic table.  The maximum number of code structures is
1444, which is the sum of 852 for literal/length codes and 592 for distance
codes.  These values were found by exhaustive searches using the program
examples/enough.c found in the zlib distribtution.  The arguments to that
program are the number of symbols, the initial root table size, and the
maximum bit length of a code.  "enough 286 9 15" for literal/length codes
returns returns 852, and "enough 30 6 15" for distance codes returns 592.
The initial root table size (9 or 6) is found in the fifth argument of the
inflate_table() calls in inflate.c and infback.c.  If the root table size is
changed, then these maximum sizes would be need to be recalculated and
updated. */
pub const ENOUGH_LENS: libc::c_int = 852 as libc::c_int;
pub const ENOUGH_DISTS: libc::c_int = 592 as libc::c_int;
pub const ENOUGH: libc::c_int = ENOUGH_LENS + ENOUGH_DISTS;
/* inflate.c -- zlib decompression
 * Copyright (C) 1995-2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
 * Change history:
 *
 * 1.2.beta0    24 Nov 2002
 * - First version -- complete rewrite of inflate to simplify code, avoid
 *   creation of window when not needed, minimize use of window when it is
 *   needed, make inffast.c even faster, implement gzip decoding, and to
 *   improve code readability and style over the previous zlib inflate code
 *
 * 1.2.beta1    25 Nov 2002
 * - Use pointers for available input and output checking in inffast.c
 * - Remove input and output counters in inffast.c
 * - Change inffast.c entry and loop from avail_in >= 7 to >= 6
 * - Remove unnecessary second byte pull from length extra in inffast.c
 * - Unroll direct copy to three copies per loop in inffast.c
 *
 * 1.2.beta2    4 Dec 2002
 * - Change external routine names to reduce potential conflicts
 * - Correct filename to inffixed.h for fixed tables in inflate.c
 * - Make hbuf[] unsigned char to match parameter type in inflate.c
 * - Change strm->next_out[-state->offset] to *(strm->next_out - state->offset)
 *   to avoid negation problem on Alphas (64 bit) in inflate.c
 *
 * 1.2.beta3    22 Dec 2002
 * - Add comments on state->bits assertion in inffast.c
 * - Add comments on op field in inftrees.h
 * - Fix bug in reuse of allocated window after inflateReset()
 * - Remove bit fields--back to byte structure for speed
 * - Remove distance extra == 0 check in inflate_fast()--only helps for lengths
 * - Change post-increments to pre-increments in inflate_fast(), PPC biased?
 * - Add compile time option, POSTINC, to use post-increments instead (Intel?)
 * - Make MATCH copy in inflate() much faster for when inflate_fast() not used
 * - Use local copies of stream next and avail values, as well as local bit
 *   buffer and bit count in inflate()--for speed when inflate_fast() not used
 *
 * 1.2.beta4    1 Jan 2003
 * - Split ptr - 257 statements in inflate_table() to avoid compiler warnings
 * - Move a comment on output buffer sizes from inffast.c to inflate.c
 * - Add comments in inffast.c to introduce the inflate_fast() routine
 * - Rearrange window copies in inflate_fast() for speed and simplification
 * - Unroll last copy for window match in inflate_fast()
 * - Use local copies of window variables in inflate_fast() for speed
 * - Pull out common wnext == 0 case for speed in inflate_fast()
 * - Make op and len in inflate_fast() unsigned for consistency
 * - Add FAR to lcode and dcode declarations in inflate_fast()
 * - Simplified bad distance check in inflate_fast()
 * - Added inflateBackInit(), inflateBack(), and inflateBackEnd() in new
 *   source file infback.c to provide a call-back interface to inflate for
 *   programs like gzip and unzip -- uses window as output buffer to avoid
 *   window copying
 *
 * 1.2.beta5    1 Jan 2003
 * - Improved inflateBack() interface to allow the caller to provide initial
 *   input in strm.
 * - Fixed stored blocks bug in inflateBack()
 *
 * 1.2.beta6    4 Jan 2003
 * - Added comments in inffast.c on effectiveness of POSTINC
 * - Typecasting all around to reduce compiler warnings
 * - Changed loops from while (1) or do {} while (1) to for (;;), again to
 *   make compilers happy
 * - Changed type of window in inflateBackInit() to unsigned char *
 *
 * 1.2.beta7    27 Jan 2003
 * - Changed many types to unsigned or unsigned short to avoid warnings
 * - Added inflateCopy() function
 *
 * 1.2.0        9 Mar 2003
 * - Changed inflateBack() interface to provide separate opaque descriptors
 *   for the in() and out() functions
 * - Changed inflateBack() argument and in_func typedef to swap the length
 *   and buffer address return values for the input function
 * - Check next_in and next_out for Z_NULL on entry to inflate()
 *
 * The history for versions after 1.2.0 are in ChangeLog in zlib distribution.
 */
/* function prototypes */
unsafe extern "C" fn inflateStateCheck(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as libc::c_int;
    }
    state = (*strm).state as *mut inflate_state;
    if state.is_null()
        || (*state).strm != strm
        || ((*state).mode as libc::c_uint) < HEAD as libc::c_int as libc::c_uint
        || (*state).mode as libc::c_uint > SYNC as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateResetKeep(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*state).total = 0 as libc::c_int as libc::c_ulong;
    (*strm).total_out = (*state).total;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = Z_NULL as *mut libc::c_char;
    if (*state).wrap != 0 {
        /* to support ill-conceived Java test suite */
        (*strm).adler = ((*state).wrap & 1 as libc::c_int) as uLong
    }
    (*state).mode = HEAD;
    (*state).last = 0 as libc::c_int;
    (*state).havedict = 0 as libc::c_int;
    (*state).dmax = 32768 as libc::c_uint;
    (*state).head = Z_NULL as gz_headerp;
    (*state).hold = 0 as libc::c_int as libc::c_ulong;
    (*state).bits = 0 as libc::c_int as libc::c_uint;
    (*state).next = (*state).codes.as_mut_ptr();
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    (*state).sane = 1 as libc::c_int;
    (*state).back = -(1 as libc::c_int);
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateReset(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*state).wsize = 0 as libc::c_int as libc::c_uint;
    (*state).whave = 0 as libc::c_int as libc::c_uint;
    (*state).wnext = 0 as libc::c_int as libc::c_uint;
    return inflateResetKeep(strm);
}
#[no_mangle]
pub unsafe extern "C" fn inflateReset2(
    mut strm: z_streamp,
    mut windowBits: libc::c_int,
) -> libc::c_int {
    let mut wrap: libc::c_int = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    /* get the state */
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    /* extract wrap request from windowBits parameter */
    if windowBits < 0 as libc::c_int {
        wrap = 0 as libc::c_int;
        windowBits = -windowBits
    } else {
        wrap = (windowBits >> 4 as libc::c_int) + 5 as libc::c_int;
        if windowBits < 48 as libc::c_int {
            windowBits &= 15 as libc::c_int
        }
    }
    /* set number of window bits, free window if different */
    if windowBits != 0 && (windowBits < 8 as libc::c_int || windowBits > 15 as libc::c_int) {
        return Z_STREAM_ERROR;
    }
    if !(*state).window.is_null() && (*state).wbits != windowBits as libc::c_uint {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as voidpf,
        );
        (*state).window = Z_NULL as *mut libc::c_uchar
    }
    /* update state and reset the rest of it */
    (*state).wrap = wrap; /* in case we return an error */
    (*state).wbits = windowBits as libc::c_uint; /* to pass state test in inflateReset2() */
    return inflateReset(strm);
}
#[no_mangle]
pub unsafe extern "C" fn inflateInit2_(
    mut strm: z_streamp,
    mut windowBits: libc::c_int,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if version.is_null()
        || *version.offset(0 as libc::c_int as isize) as libc::c_int
            != (*::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00"))
                [0 as libc::c_int as usize] as libc::c_int
        || stream_size != ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int
    {
        return Z_VERSION_ERROR;
    }
    if strm.is_null() {
        return Z_STREAM_ERROR;
    }
    (*strm).msg = Z_NULL as *mut libc::c_char;
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(
            zcalloc as unsafe extern "C" fn(_: voidpf, _: libc::c_uint, _: libc::c_uint) -> voidpf,
        );
        (*strm).opaque = 0 as voidpf
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree = Some(zcfree as unsafe extern "C" fn(_: voidpf, _: voidpf) -> ())
    }
    state = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        1 as libc::c_int as uInt,
        ::std::mem::size_of::<inflate_state>() as libc::c_ulong as uInt,
    ) as *mut inflate_state;
    if state.is_null() {
        return Z_MEM_ERROR;
    }
    (*strm).state = state as *mut internal_state;
    (*state).strm = strm;
    (*state).window = Z_NULL as *mut libc::c_uchar;
    (*state).mode = HEAD;
    ret = inflateReset2(strm, windowBits);
    if ret != Z_OK {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            state as voidpf,
        );
        (*strm).state = Z_NULL as *mut internal_state
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn inflateInit_(
    mut strm: z_streamp,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    return inflateInit2_(strm, DEF_WBITS, version, stream_size);
}
#[no_mangle]
pub unsafe extern "C" fn inflatePrime(
    mut strm: z_streamp,
    mut bits: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if bits < 0 as libc::c_int {
        (*state).hold = 0 as libc::c_int as libc::c_ulong;
        (*state).bits = 0 as libc::c_int as libc::c_uint;
        return Z_OK;
    }
    if bits > 16 as libc::c_int
        || (*state).bits.wrapping_add(bits as uInt) > 32 as libc::c_int as libc::c_uint
    {
        return Z_STREAM_ERROR;
    }
    value = (value as libc::c_long
        & ((1 as libc::c_long) << bits) - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
    (*state).hold = (*state)
        .hold
        .wrapping_add(((value as libc::c_uint) << (*state).bits) as libc::c_ulong);
    (*state).bits = (*state).bits.wrapping_add(bits as uInt);
    return Z_OK;
}
/*
  Return state with length and distance decoding tables and index sizes set to
  fixed code decoding.  Normally this returns fixed tables from inffixed.h.
  If BUILDFIXED is defined, then instead this routine builds the tables the
  first time it's called, and returns those tables the first time and
  thereafter.  This reduces the size of the code by about 2K bytes, in
  exchange for a little execution time.  However, BUILDFIXED should not be
  used for threaded applications, since the rewriting of the tables and virgin
  may not be thread-safe.
*/
unsafe extern "C" fn fixedtables(mut state: *mut inflate_state) {
    /* inffixed.h -- table for decoding fixed codes
     * Generated automatically by makefixed().
     */
    /* WARNING: this file should *not* be used by applications.
      It is part of the implementation of this library and is
      subject to change. Applications should only use zlib.h.
    */
    static mut lenfix: [code; 512] = [
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 80 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 16 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 112 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 48 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 192 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 96 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 32 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 160 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 128 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 64 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 224 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 88 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 24 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 144 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 120 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 56 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 208 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 104 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 40 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 176 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 136 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 72 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 240 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 84 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 20 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 116 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 52 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 200 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 100 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 36 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 168 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 132 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 68 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 232 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 92 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 28 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 152 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 124 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 60 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 216 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 108 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 44 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 184 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 12 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 140 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 76 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 248 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 82 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 18 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 114 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 50 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 196 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 98 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 34 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 164 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 130 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 66 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 228 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 90 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 26 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 148 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 122 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 58 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 212 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 106 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 42 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 180 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 138 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 74 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 244 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 86 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 22 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 118 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 54 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 204 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 102 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 38 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 172 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 134 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 70 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 236 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 94 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 30 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 156 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 126 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 62 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 220 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 110 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 46 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 188 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 14 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 142 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 78 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 252 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 81 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 113 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 194 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 162 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 226 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 89 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 146 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 121 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 57 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 210 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 105 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 41 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 178 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 137 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 73 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 242 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 85 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 21 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 258 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 117 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 53 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 202 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 101 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 37 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 170 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 133 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 69 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 234 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 93 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 29 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 154 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 125 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 61 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 218 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 109 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 45 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 186 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 141 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 77 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 250 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 198 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 166 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 230 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 91 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 150 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 123 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 214 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 107 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 182 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 139 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 75 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 246 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 87 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 119 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 55 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 206 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 103 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 39 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 174 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 135 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 71 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 238 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 95 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 158 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 127 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 63 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 222 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 111 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 47 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 190 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 143 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 79 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 254 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 80 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 16 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 112 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 48 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 96 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 32 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 161 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 128 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 64 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 225 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 88 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 24 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 145 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 120 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 56 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 209 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 104 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 40 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 177 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 136 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 72 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 241 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 84 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 20 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 116 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 52 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 201 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 100 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 36 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 169 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 132 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 68 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 233 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 92 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 28 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 153 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 124 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 60 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 217 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 108 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 44 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 185 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 12 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 140 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 76 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 249 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 82 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 18 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 114 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 50 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 197 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 98 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 34 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 165 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 130 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 66 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 229 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 90 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 26 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 149 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 122 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 58 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 213 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 106 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 42 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 181 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 138 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 74 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 245 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 86 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 22 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 118 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 54 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 205 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 102 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 38 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 173 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 134 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 70 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 237 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 94 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 30 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 157 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 126 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 62 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 221 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 110 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 46 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 189 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 14 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 142 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 78 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 253 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 81 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 113 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 89 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 147 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 121 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 57 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 211 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 105 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 41 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 179 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 137 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 73 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 243 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 85 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 21 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 258 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 117 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 53 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 203 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 101 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 37 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 171 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 133 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 69 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 235 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 93 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 29 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 155 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 125 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 61 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 219 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 109 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 45 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 187 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 141 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 77 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 251 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 199 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 167 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 231 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 91 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 151 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 123 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 215 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 107 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 183 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 139 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 75 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 247 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 87 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 119 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 55 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 207 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 103 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 39 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 175 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 135 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 71 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 239 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 95 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 159 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 127 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 63 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 223 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 111 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 47 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 191 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 143 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 79 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 255 as libc::c_int as libc::c_ushort,
            };
            init
        },
    ];
    static mut distfix: [code; 32] = [
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 257 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 4097 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1025 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 16385 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 513 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 8193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 2049 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 385 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 6145 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1537 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 24577 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 769 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 12289 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 3073 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
    ];
    /* !BUILDFIXED */
    /* BUILDFIXED */
    (*state).lencode = lenfix.as_ptr();
    (*state).lenbits = 9 as libc::c_int as libc::c_uint;
    (*state).distcode = distfix.as_ptr();
    (*state).distbits = 5 as libc::c_int as libc::c_uint;
}
/* MAKEFIXED */
/*
  Update the window with the last wsize (normally 32K) bytes written before
  returning.  If window does not exist yet, create it.  This is only called
  when a window is already in use, or when output has been written during this
  inflate call, but the end of the deflate stream has not been reached yet.
  It is also called to create a window for dictionary data when a dictionary
  is loaded.

  Providing output buffers larger than 32K to inflate() should provide a speed
  advantage, since only the last 32K of output is copied to the sliding window
  upon return from inflate(), and since all distances after the first 32K of
  output will fall in the output data, making match copies simpler and faster.
  The advantage may be dependent on the size of the processor's data caches.
*/
unsafe extern "C" fn updatewindow(
    mut strm: z_streamp,
    mut end: *const Bytef,
    mut copy: libc::c_uint,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut dist: libc::c_uint = 0;
    state = (*strm).state as *mut inflate_state;
    /* if it hasn't been done already, allocate space for the window */
    if (*state).window.is_null() {
        (*state).window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            (1 as libc::c_uint) << (*state).wbits,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as uInt,
        ) as *mut libc::c_uchar;
        if (*state).window.is_null() {
            return 1 as libc::c_int;
        }
    }
    /* if window not in use yet, initialize */
    if (*state).wsize == 0 as libc::c_int as libc::c_uint {
        (*state).wsize = (1 as libc::c_uint) << (*state).wbits;
        (*state).wnext = 0 as libc::c_int as libc::c_uint;
        (*state).whave = 0 as libc::c_int as libc::c_uint
    }
    /* copy state->wsize or less output bytes into the circular window */
    if copy >= (*state).wsize {
        memcpy(
            (*state).window as *mut libc::c_void,
            end.offset(-((*state).wsize as isize)) as *const libc::c_void,
            (*state).wsize as libc::c_ulong,
        );
        (*state).wnext = 0 as libc::c_int as libc::c_uint;
        (*state).whave = (*state).wsize
    } else {
        dist = (*state).wsize.wrapping_sub((*state).wnext);
        if dist > copy {
            dist = copy
        }
        memcpy(
            (*state).window.offset((*state).wnext as isize) as *mut libc::c_void,
            end.offset(-(copy as isize)) as *const libc::c_void,
            dist as libc::c_ulong,
        );
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            memcpy(
                (*state).window as *mut libc::c_void,
                end.offset(-(copy as isize)) as *const libc::c_void,
                copy as libc::c_ulong,
            );
            (*state).wnext = copy;
            (*state).whave = (*state).wsize
        } else {
            (*state).wnext = (*state).wnext.wrapping_add(dist);
            if (*state).wnext == (*state).wsize {
                (*state).wnext = 0 as libc::c_int as libc::c_uint
            }
            if (*state).whave < (*state).wsize {
                (*state).whave = (*state).whave.wrapping_add(dist)
            }
        }
    }
    return 0 as libc::c_int;
}
/* Remove n bits from the bit accumulator */
/* Remove zero to seven bits as needed to go to a byte boundary */
/*
  inflate() uses a state machine to process as much input data and generate as
  much output data as possible before returning.  The state machine is
  structured roughly as follows:

   for (;;) switch (state) {
   ...
   case STATEn:
       if (not enough input data or output space to make progress)
           return;
       ... make progress ...
       state = STATEm;
       break;
   ...
   }

  so when inflate() is called again, the same case is attempted again, and
  if the appropriate resources are provided, the machine proceeds to the
  next state.  The NEEDBITS() macro is usually the way the state evaluates
  whether it can proceed or should return.  NEEDBITS() does the return if
  the requested bits are not available.  The typical use of the BITS macros
  is:

       NEEDBITS(n);
       ... do something with BITS(n) ...
       DROPBITS(n);

  where NEEDBITS(n) either returns from inflate() if there isn't enough
  input left to load n bits into the accumulator, or it continues.  BITS(n)
  gives the low n bits in the accumulator.  When done, DROPBITS(n) drops
  the low n bits off the accumulator.  INITBITS() clears the accumulator
  and sets the number of available bits to zero.  BYTEBITS() discards just
  enough bits to put the accumulator on a byte boundary.  After BYTEBITS()
  and a NEEDBITS(8), then BITS(8) would return the next byte in the stream.

  NEEDBITS(n) uses PULLBYTE() to get an available byte of input, or to return
  if there is no input available.  The decoding of variable length codes uses
  PULLBYTE() directly in order to pull just enough bytes to decode the next
  code, and no more.

  Some states loop until they get enough input, making sure that enough
  state information is maintained to continue the loop where it left off
  if NEEDBITS() returns in the loop.  For example, want, need, and keep
  would all have to actually be part of the saved state in case NEEDBITS()
  returns:

   case STATEw:
       while (want < need) {
           NEEDBITS(n);
           keep[want++] = BITS(n);
           DROPBITS(n);
       }
       state = STATEx;
   case STATEx:

  As shown above, if the next state is also the next case, then the break
  is omitted.

  A state may also return if there is not enough output space available to
  complete that state.  Those states are copying stored data, writing a
  literal byte, and copying a matching string.

  When returning, a "goto inf_leave" is used to update the total counters,
  update the check value, and determine whether any progress has been made
  during that inflate() call in order to return the proper return code.
  Progress is defined as a change in either strm->avail_in or strm->avail_out.
  When there is a window, goto inf_leave will update the window with the last
  output written.  If a goto inf_leave occurs in the middle of decompression
  and there is no window currently, goto inf_leave will create one and copy
  output to the window for the next call of inflate().

  In this implementation, the flush parameter of inflate() only affects the
  return code (per zlib.h).  inflate() always writes as much as possible to
  strm->next_out, given the space available and the provided input--the effect
  documented in zlib.h of Z_SYNC_FLUSH.  Furthermore, inflate() always defers
  the allocation of and copying into a sliding window until necessary, which
  provides the effect documented in zlib.h for Z_FINISH when the entire input
  stream available.  So the only thing the flush parameter actually does is:
  when flush is set to Z_FINISH, inflate() cannot return Z_OK.  Instead it
  will return Z_BUF_ERROR if it has not reached the end of the stream.
*/
#[no_mangle]
pub unsafe extern "C" fn inflate(mut strm: z_streamp, mut flush: libc::c_int) -> libc::c_int {
    let mut current_block: u64; /* next input */
    let mut state: *mut inflate_state = 0 as *mut inflate_state; /* next output */
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* available input and output */
    let mut put: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* bit buffer */
    let mut have: libc::c_uint = 0; /* bits in bit buffer */
    let mut left: libc::c_uint = 0; /* save starting available input and output */
    let mut hold: libc::c_ulong = 0; /* number of stored or match bytes to copy */
    let mut bits: libc::c_uint = 0; /* where to copy match bytes from */
    let mut in_0: libc::c_uint = 0; /* current decoding table entry */
    let mut out: libc::c_uint = 0; /* parent table entry */
    let mut copy: libc::c_uint = 0; /* length to copy for repeats, bits to drop */
    let mut from: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* return code */
    let mut here: code = code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* buffer for gzip header crc calculation */
    let mut last: code = code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* skip check */
    let mut len: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    let mut hbuf: [libc::c_uchar; 4] = [0; 4];
    static mut order: [libc::c_ushort; 19] = [
        16 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
        8 as libc::c_int as libc::c_ushort,
        7 as libc::c_int as libc::c_ushort,
        9 as libc::c_int as libc::c_ushort,
        6 as libc::c_int as libc::c_ushort,
        10 as libc::c_int as libc::c_ushort,
        5 as libc::c_int as libc::c_ushort,
        11 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        12 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        13 as libc::c_int as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        14 as libc::c_int as libc::c_ushort,
        1 as libc::c_int as libc::c_ushort,
        15 as libc::c_int as libc::c_ushort,
    ];
    if inflateStateCheck(strm) != 0
        || (*strm).next_out.is_null()
        || (*strm).next_in.is_null() && (*strm).avail_in != 0 as libc::c_int as libc::c_uint
    {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).mode as libc::c_uint == TYPE as libc::c_int as libc::c_uint {
        (*state).mode = TYPEDO
    }
    put = (*strm).next_out;
    left = (*strm).avail_out;
    next = (*strm).next_in;
    have = (*strm).avail_in;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = Z_OK;
    's_117: loop {
        match (*state).mode as libc::c_uint {
            16180 => {
                if (*state).wrap == 0 as libc::c_int {
                    (*state).mode = TYPEDO;
                    continue;
                } else {
                    while bits < 16 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if (*state).wrap & 2 as libc::c_int != 0
                        && hold == 0x8b1f as libc::c_int as libc::c_ulong
                    {
                        /* gzip header */
                        if (*state).wbits == 0 as libc::c_int as libc::c_uint {
                            (*state).wbits = 15 as libc::c_int as libc::c_uint
                        } /* expect zlib header */
                        (*state).check = crc32(
                            0 as libc::c_long as uLong,
                            Z_NULL as *const Bytef,
                            0 as libc::c_int as uInt,
                        ); /* go to byte boundary */
                        hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                        hbuf[1 as libc::c_int as usize] =
                            (hold >> 8 as libc::c_int) as libc::c_uchar;
                        (*state).check =
                            crc32((*state).check, hbuf.as_mut_ptr(), 2 as libc::c_int as uInt);
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint;
                        (*state).mode = FLAGS;
                        continue;
                    } else {
                        (*state).flags = 0 as libc::c_int;
                        if !(*state).head.is_null() {
                            (*(*state).head).done = -(1 as libc::c_int)
                        }
                        if (*state).wrap & 1 as libc::c_int == 0
                            || (((hold as libc::c_uint
                                & ((1 as libc::c_uint) << 8 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                << 8 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_add(hold >> 8 as libc::c_int)
                                .wrapping_rem(31 as libc::c_int as libc::c_ulong)
                                != 0
                        {
                            (*strm).msg = b"incorrect header check\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = BAD;
                            continue;
                        } else if hold as libc::c_uint
                            & ((1 as libc::c_uint) << 4 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            != Z_DEFLATED as libc::c_uint
                        {
                            (*strm).msg = b"unknown compression method\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = BAD;
                            continue;
                        } else {
                            hold >>= 4 as libc::c_int;
                            bits = bits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                            len = (hold as libc::c_uint
                                & ((1 as libc::c_uint) << 4 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                            .wrapping_add(8 as libc::c_int as libc::c_uint);
                            if (*state).wbits == 0 as libc::c_int as libc::c_uint {
                                (*state).wbits = len
                            }
                            if len > 15 as libc::c_int as libc::c_uint || len > (*state).wbits {
                                (*strm).msg = b"invalid window size\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                                (*state).mode = BAD;
                                continue;
                            } else {
                                (*state).dmax = (1 as libc::c_uint) << len;
                                (*state).check = adler32(
                                    0 as libc::c_long as uLong,
                                    Z_NULL as *const Bytef,
                                    0 as libc::c_int as uInt,
                                );
                                (*strm).adler = (*state).check;
                                (*state).mode = if hold & 0x200 as libc::c_int as libc::c_ulong != 0
                                {
                                    DICTID as libc::c_int
                                } else {
                                    TYPE as libc::c_int
                                } as inflate_mode;
                                hold = 0 as libc::c_int as libc::c_ulong;
                                bits = 0 as libc::c_int as libc::c_uint;
                                continue;
                            }
                        }
                    }
                }
            }
            16181 => {
                while bits < 16 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_117;
                    }
                    have = have.wrapping_sub(1);
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                (*state).flags = hold as libc::c_int;
                if (*state).flags & 0xff as libc::c_int != Z_DEFLATED {
                    (*strm).msg = b"unknown compression method\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else if (*state).flags & 0xe000 as libc::c_int != 0 {
                    (*strm).msg = b"unknown header flags set\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    if !(*state).head.is_null() {
                        (*(*state).head).text = (hold >> 8 as libc::c_int
                            & 1 as libc::c_int as libc::c_ulong)
                            as libc::c_int
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0
                        && (*state).wrap & 4 as libc::c_int != 0
                    {
                        hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                        hbuf[1 as libc::c_int as usize] =
                            (hold >> 8 as libc::c_int) as libc::c_uchar;
                        (*state).check =
                            crc32((*state).check, hbuf.as_mut_ptr(), 2 as libc::c_int as uInt)
                    }
                    hold = 0 as libc::c_int as libc::c_ulong;
                    bits = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = TIME
                }
                current_block = 7739940392431776979;
            }
            16182 => {
                current_block = 7739940392431776979;
            }
            16183 => {
                current_block = 7244994750255146185;
            }
            16184 => {
                current_block = 17421514138680506998;
            }
            16185 => {
                current_block = 3091652578027905731;
            }
            16186 => {
                current_block = 10761099658514198778;
            }
            16187 => {
                current_block = 18198803391298250726;
            }
            16188 => {
                current_block = 12478305818333681747;
            }
            16189 => {
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_117;
                    }
                    have = have.wrapping_sub(1);
                    let fresh10 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh10 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                (*state).check = (hold >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                    .wrapping_add(hold >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (hold & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (hold & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int,
                    );
                (*strm).adler = (*state).check;
                hold = 0 as libc::c_int as libc::c_ulong;
                bits = 0 as libc::c_int as libc::c_uint;
                (*state).mode = DICT;
                current_block = 18414679352751625475;
            }
            16190 => {
                current_block = 18414679352751625475;
            }
            16191 => {
                current_block = 15068134270358694900;
            }
            16192 => {
                current_block = 4793906099943269613;
            }
            16193 => {
                hold >>= bits & 7 as libc::c_int as libc::c_uint;
                bits = bits.wrapping_sub(bits & 7 as libc::c_int as libc::c_uint);
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_117;
                    }
                    have = have.wrapping_sub(1);
                    let fresh12 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh12 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                if hold & 0xffff as libc::c_int as libc::c_ulong
                    != hold >> 16 as libc::c_int ^ 0xffff as libc::c_int as libc::c_ulong
                {
                    (*strm).msg = b"invalid stored block lengths\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).length = hold as libc::c_uint & 0xffff as libc::c_int as libc::c_uint;
                    hold = 0 as libc::c_int as libc::c_ulong;
                    bits = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = COPY_;
                    if flush == Z_TREES {
                        break;
                    }
                }
                current_block = 6897396512028333200;
            }
            16194 => {
                current_block = 6897396512028333200;
            }
            16195 => {
                current_block = 13223652920459569936;
            }
            16196 => {
                while bits < 14 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_117;
                    }
                    have = have.wrapping_sub(1);
                    let fresh13 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh13 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                (*state).nlen = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 5 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                .wrapping_add(257 as libc::c_int as libc::c_uint);
                hold >>= 5 as libc::c_int;
                bits = bits.wrapping_sub(5 as libc::c_int as libc::c_uint);
                (*state).ndist = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 5 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                .wrapping_add(1 as libc::c_int as libc::c_uint);
                hold >>= 5 as libc::c_int;
                bits = bits.wrapping_sub(5 as libc::c_int as libc::c_uint);
                (*state).ncode = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 4 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                .wrapping_add(4 as libc::c_int as libc::c_uint);
                hold >>= 4 as libc::c_int;
                bits = bits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                if (*state).nlen > 286 as libc::c_int as libc::c_uint
                    || (*state).ndist > 30 as libc::c_int as libc::c_uint
                {
                    (*strm).msg = b"too many length or distance symbols\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).have = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = LENLENS
                }
                current_block = 17936412886206549082;
            }
            16197 => {
                current_block = 17936412886206549082;
            }
            16198 => {
                current_block = 12839741003815810352;
            }
            16199 => {
                current_block = 2332297531576259167;
            }
            16200 => {
                current_block = 7796943931196339650;
            }
            16201 => {
                current_block = 2829640705969624118;
            }
            16202 => {
                current_block = 13250426973517230943;
            }
            16203 => {
                current_block = 5418677336239499507;
            }
            16204 => {
                current_block = 16846429559699824015;
            }
            16205 => {
                if left == 0 as libc::c_int as libc::c_uint {
                    break;
                }
                let fresh32 = put;
                put = put.offset(1);
                *fresh32 = (*state).length as libc::c_uchar;
                left = left.wrapping_sub(1);
                (*state).mode = LEN;
                continue;
            }
            16206 => {
                if (*state).wrap != 0 {
                    while bits < 32 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh33 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh33 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    out = out.wrapping_sub(left);
                    (*strm).total_out = ((*strm).total_out as libc::c_ulong)
                        .wrapping_add(out as libc::c_ulong)
                        as uLong as uLong;
                    (*state).total = (*state).total.wrapping_add(out as libc::c_ulong);
                    if (*state).wrap & 4 as libc::c_int != 0 && out != 0 {
                        (*state).check = if (*state).flags != 0 {
                            crc32((*state).check, put.offset(-(out as isize)), out)
                        } else {
                            adler32((*state).check, put.offset(-(out as isize)), out)
                        };
                        (*strm).adler = (*state).check
                    }
                    out = left;
                    if (*state).wrap & 4 as libc::c_int != 0
                        && (if (*state).flags != 0 {
                            hold
                        } else {
                            (hold >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    hold >> 8 as libc::c_int
                                        & 0xff00 as libc::c_int as libc::c_ulong,
                                )
                                .wrapping_add(
                                    (hold & 0xff00 as libc::c_int as libc::c_ulong)
                                        << 8 as libc::c_int,
                                )
                                .wrapping_add(
                                    (hold & 0xff as libc::c_int as libc::c_ulong)
                                        << 24 as libc::c_int,
                                )
                        }) != (*state).check
                    {
                        (*strm).msg = b"incorrect data check\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint
                    }
                }
                (*state).mode = LENGTH;
                current_block = 14245139974307499898;
            }
            16207 => {
                current_block = 14245139974307499898;
            }
            16208 => {
                current_block = 4359251403161895435;
            }
            16209 => {
                ret = Z_DATA_ERROR;
                break;
            }
            16210 => return Z_MEM_ERROR,
            16211 | _ => return Z_STREAM_ERROR,
        }
        match current_block {
            14245139974307499898 => {
                if (*state).wrap != 0 && (*state).flags != 0 {
                    while bits < 32 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh34 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh34 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if hold != (*state).total & 0xffffffff as libc::c_ulong {
                        (*strm).msg = b"incorrect length check\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint
                    }
                }
                (*state).mode = DONE;
                current_block = 4359251403161895435;
            }
            17936412886206549082 => {
                while (*state).have < (*state).ncode {
                    while bits < 3 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh14 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh14 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    let fresh15 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh15 as usize] as usize] = (hold as libc::c_uint
                        & ((1 as libc::c_uint) << 3 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        as libc::c_ushort;
                    hold >>= 3 as libc::c_int;
                    bits = bits.wrapping_sub(3 as libc::c_int as libc::c_uint)
                }
                while (*state).have < 19 as libc::c_int as libc::c_uint {
                    let fresh16 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh16 as usize] as usize] =
                        0 as libc::c_int as libc::c_ushort
                }
                (*state).next = (*state).codes.as_mut_ptr();
                (*state).lencode = (*state).next as *const code;
                (*state).lenbits = 7 as libc::c_int as libc::c_uint;
                ret = inflate_table(
                    CODES,
                    (*state).lens.as_mut_ptr(),
                    19 as libc::c_int as libc::c_uint,
                    &mut (*state).next,
                    &mut (*state).lenbits,
                    (*state).work.as_mut_ptr(),
                );
                if ret != 0 {
                    (*strm).msg = b"invalid code lengths set\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).have = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = CODELENS
                }
                current_block = 12839741003815810352;
            }
            18414679352751625475 => {
                if (*state).havedict == 0 as libc::c_int {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    return Z_NEED_DICT;
                }
                (*state).check = adler32(
                    0 as libc::c_long as uLong,
                    Z_NULL as *const Bytef,
                    0 as libc::c_int as uInt,
                );
                (*strm).adler = (*state).check;
                (*state).mode = TYPE;
                current_block = 15068134270358694900;
            }
            7739940392431776979 => {
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_117;
                    }
                    have = have.wrapping_sub(1);
                    let fresh2 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh2 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                if !(*state).head.is_null() {
                    (*(*state).head).time = hold
                }
                if (*state).flags & 0x200 as libc::c_int != 0
                    && (*state).wrap & 4 as libc::c_int != 0
                {
                    hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                    hbuf[1 as libc::c_int as usize] = (hold >> 8 as libc::c_int) as libc::c_uchar;
                    hbuf[2 as libc::c_int as usize] = (hold >> 16 as libc::c_int) as libc::c_uchar;
                    hbuf[3 as libc::c_int as usize] = (hold >> 24 as libc::c_int) as libc::c_uchar;
                    (*state).check =
                        crc32((*state).check, hbuf.as_mut_ptr(), 4 as libc::c_int as uInt)
                }
                hold = 0 as libc::c_int as libc::c_ulong;
                bits = 0 as libc::c_int as libc::c_uint;
                (*state).mode = OS;
                current_block = 7244994750255146185;
            }
            6897396512028333200 => {
                (*state).mode = COPY;
                current_block = 13223652920459569936;
            }
            _ => {}
        }
        match current_block {
            12839741003815810352 => {
                while (*state).have < (*state).nlen.wrapping_add((*state).ndist) {
                    loop {
                        here = *(*state).lencode.offset(
                            (hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                as isize,
                        );
                        if here.bits as libc::c_uint <= bits {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh17 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh17 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if (here.val as libc::c_int) < 16 as libc::c_int {
                        hold >>= here.bits as libc::c_int;
                        bits = bits.wrapping_sub(here.bits as libc::c_uint);
                        let fresh18 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[fresh18 as usize] = here.val
                    } else {
                        if here.val as libc::c_int == 16 as libc::c_int {
                            while bits
                                < (here.bits as libc::c_int + 2 as libc::c_int) as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_117;
                                }
                                have = have.wrapping_sub(1);
                                let fresh19 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh19 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                            hold >>= here.bits as libc::c_int;
                            bits = bits.wrapping_sub(here.bits as libc::c_uint);
                            if (*state).have == 0 as libc::c_int as libc::c_uint {
                                (*strm).msg = b"invalid bit length repeat\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                                (*state).mode = BAD;
                                break;
                            } else {
                                len = (*state).lens[(*state)
                                    .have
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as usize] as libc::c_uint;
                                copy = (3 as libc::c_int as libc::c_uint).wrapping_add(
                                    hold as libc::c_uint
                                        & ((1 as libc::c_uint) << 2 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                );
                                hold >>= 2 as libc::c_int;
                                bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint)
                            }
                        } else if here.val as libc::c_int == 17 as libc::c_int {
                            while bits
                                < (here.bits as libc::c_int + 3 as libc::c_int) as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_117;
                                }
                                have = have.wrapping_sub(1);
                                let fresh20 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh20 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                            hold >>= here.bits as libc::c_int;
                            bits = bits.wrapping_sub(here.bits as libc::c_uint);
                            len = 0 as libc::c_int as libc::c_uint;
                            copy = (3 as libc::c_int as libc::c_uint).wrapping_add(
                                hold as libc::c_uint
                                    & ((1 as libc::c_uint) << 3 as libc::c_int)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                            );
                            hold >>= 3 as libc::c_int;
                            bits = bits.wrapping_sub(3 as libc::c_int as libc::c_uint)
                        } else {
                            while bits
                                < (here.bits as libc::c_int + 7 as libc::c_int) as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_117;
                                }
                                have = have.wrapping_sub(1);
                                let fresh21 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh21 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                            hold >>= here.bits as libc::c_int;
                            bits = bits.wrapping_sub(here.bits as libc::c_uint);
                            len = 0 as libc::c_int as libc::c_uint;
                            copy = (11 as libc::c_int as libc::c_uint).wrapping_add(
                                hold as libc::c_uint
                                    & ((1 as libc::c_uint) << 7 as libc::c_int)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                            );
                            hold >>= 7 as libc::c_int;
                            bits = bits.wrapping_sub(7 as libc::c_int as libc::c_uint)
                        }
                        if (*state).have.wrapping_add(copy)
                            > (*state).nlen.wrapping_add((*state).ndist)
                        {
                            (*strm).msg = b"invalid bit length repeat\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = BAD;
                            break;
                        } else {
                            loop {
                                let fresh22 = copy;
                                copy = copy.wrapping_sub(1);
                                if !(fresh22 != 0) {
                                    break;
                                }
                                let fresh23 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[fresh23 as usize] = len as libc::c_ushort
                            }
                        }
                    }
                }
                /* handle error breaks in while */
                if (*state).mode as libc::c_uint == BAD as libc::c_int as libc::c_uint {
                    continue;
                }
                /* check for end-of-block code (better have one) */
                if (*state).lens[256 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
                    (*strm).msg = b"invalid code -- missing end-of-block\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    /* build code tables -- note: do not change the lenbits or distbits
                    values here (9 and 6) without reading the comments in inftrees.h
                    concerning the ENOUGH constants, which depend on those values */
                    (*state).next = (*state).codes.as_mut_ptr();
                    (*state).lencode = (*state).next as *const code;
                    (*state).lenbits = 9 as libc::c_int as libc::c_uint;
                    ret = inflate_table(
                        LENS,
                        (*state).lens.as_mut_ptr(),
                        (*state).nlen,
                        &mut (*state).next,
                        &mut (*state).lenbits,
                        (*state).work.as_mut_ptr(),
                    );
                    if ret != 0 {
                        (*strm).msg = b"invalid literal/lengths set\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        (*state).distcode = (*state).next as *const code;
                        (*state).distbits = 6 as libc::c_int as libc::c_uint;
                        ret = inflate_table(
                            DISTS,
                            (*state).lens.as_mut_ptr().offset((*state).nlen as isize),
                            (*state).ndist,
                            &mut (*state).next,
                            &mut (*state).distbits,
                            (*state).work.as_mut_ptr(),
                        );
                        if ret != 0 {
                            (*strm).msg = b"invalid distances set\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = BAD;
                            continue;
                        } else {
                            (*state).mode = LEN_;
                            if flush == Z_TREES {
                                break;
                            }
                        }
                    }
                }
                current_block = 2332297531576259167;
            }
            13223652920459569936 => {
                copy = (*state).length;
                if copy != 0 {
                    if copy > have {
                        copy = have
                    }
                    if copy > left {
                        copy = left
                    }
                    if copy == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                    memcpy(
                        put as *mut libc::c_void,
                        next as *const libc::c_void,
                        copy as libc::c_ulong,
                    );
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    left = left.wrapping_sub(copy);
                    put = put.offset(copy as isize);
                    (*state).length = (*state).length.wrapping_sub(copy);
                    continue;
                } else {
                    (*state).mode = TYPE;
                    continue;
                }
            }
            7244994750255146185 => {
                while bits < 16 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_117;
                    }
                    have = have.wrapping_sub(1);
                    let fresh3 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh3 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                if !(*state).head.is_null() {
                    (*(*state).head).xflags =
                        (hold & 0xff as libc::c_int as libc::c_ulong) as libc::c_int;
                    (*(*state).head).os = (hold >> 8 as libc::c_int) as libc::c_int
                }
                if (*state).flags & 0x200 as libc::c_int != 0
                    && (*state).wrap & 4 as libc::c_int != 0
                {
                    hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                    hbuf[1 as libc::c_int as usize] = (hold >> 8 as libc::c_int) as libc::c_uchar;
                    (*state).check =
                        crc32((*state).check, hbuf.as_mut_ptr(), 2 as libc::c_int as uInt)
                }
                hold = 0 as libc::c_int as libc::c_ulong;
                bits = 0 as libc::c_int as libc::c_uint;
                (*state).mode = EXLEN;
                current_block = 17421514138680506998;
            }
            15068134270358694900 => {
                if flush == Z_BLOCK || flush == Z_TREES {
                    break;
                }
                current_block = 4793906099943269613;
            }
            4359251403161895435 => {
                ret = Z_STREAM_END;
                break;
            }
            _ => {}
        }
        match current_block {
            4793906099943269613 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as libc::c_int as libc::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as libc::c_int as libc::c_uint);
                    (*state).mode = CHECK;
                    continue;
                } else {
                    while bits < 3 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh11 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh11 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    (*state).last = (hold as libc::c_uint
                        & ((1 as libc::c_uint) << 1 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        as libc::c_int;
                    hold >>= 1 as libc::c_int;
                    bits = bits.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    match hold as libc::c_uint
                        & ((1 as libc::c_uint) << 2 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    {
                        0 => {
                            /* stored block */
                            (*state).mode = STORED
                        }
                        1 => {
                            /* fixed block */
                            fixedtables(state); /* decode codes */
                            (*state).mode = LEN_;
                            if flush == Z_TREES {
                                hold >>= 2 as libc::c_int;
                                bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint);
                                break;
                            }
                        }
                        2 => {
                            /* dynamic block */
                            (*state).mode = TABLE
                        }
                        3 => {
                            (*strm).msg = b"invalid block type\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = BAD
                        }
                        _ => {}
                    }
                    hold >>= 2 as libc::c_int;
                    bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint);
                    continue;
                }
            }
            17421514138680506998 => {
                if (*state).flags & 0x400 as libc::c_int != 0 {
                    while bits < 16 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh4 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh4 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    (*state).length = hold as libc::c_uint;
                    if !(*state).head.is_null() {
                        (*(*state).head).extra_len = hold as libc::c_uint
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0
                        && (*state).wrap & 4 as libc::c_int != 0
                    {
                        hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                        hbuf[1 as libc::c_int as usize] =
                            (hold >> 8 as libc::c_int) as libc::c_uchar;
                        (*state).check =
                            crc32((*state).check, hbuf.as_mut_ptr(), 2 as libc::c_int as uInt)
                    }
                    hold = 0 as libc::c_int as libc::c_ulong;
                    bits = 0 as libc::c_int as libc::c_uint
                } else if !(*state).head.is_null() {
                    (*(*state).head).extra = Z_NULL as *mut Bytef
                }
                (*state).mode = EXTRA;
                current_block = 3091652578027905731;
            }
            2332297531576259167 => {
                (*state).mode = LEN;
                current_block = 7796943931196339650;
            }
            _ => {}
        }
        match current_block {
            7796943931196339650 => {
                if have >= 6 as libc::c_int as libc::c_uint
                    && left >= 258 as libc::c_int as libc::c_uint
                {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    inflate_fast(strm, out);
                    put = (*strm).next_out;
                    left = (*strm).avail_out;
                    next = (*strm).next_in;
                    have = (*strm).avail_in;
                    hold = (*state).hold;
                    bits = (*state).bits;
                    if (*state).mode as libc::c_uint == TYPE as libc::c_int as libc::c_uint {
                        (*state).back = -(1 as libc::c_int)
                    }
                    continue;
                } else {
                    (*state).back = 0 as libc::c_int;
                    loop {
                        here = *(*state).lencode.offset(
                            (hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                as isize,
                        );
                        if here.bits as libc::c_uint <= bits {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh24 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh24 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if here.op as libc::c_int != 0
                        && here.op as libc::c_int & 0xf0 as libc::c_int == 0 as libc::c_int
                    {
                        last = here;
                        loop {
                            here = *(*state).lencode.offset(
                                (last.val as libc::c_uint).wrapping_add(
                                    (hold as libc::c_uint
                                        & ((1 as libc::c_uint)
                                            << last.bits as libc::c_int + last.op as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                        >> last.bits as libc::c_int,
                                ) as isize,
                            );
                            if (last.bits as libc::c_int + here.bits as libc::c_int) as libc::c_uint
                                <= bits
                            {
                                break;
                            }
                            if have == 0 as libc::c_int as libc::c_uint {
                                break 's_117;
                            }
                            have = have.wrapping_sub(1);
                            let fresh25 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh25 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                        }
                        hold >>= last.bits as libc::c_int;
                        bits = bits.wrapping_sub(last.bits as libc::c_uint);
                        (*state).back += last.bits as libc::c_int
                    }
                    hold >>= here.bits as libc::c_int;
                    bits = bits.wrapping_sub(here.bits as libc::c_uint);
                    (*state).back += here.bits as libc::c_int;
                    (*state).length = here.val as libc::c_uint;
                    if here.op as libc::c_int == 0 as libc::c_int {
                        (*state).mode = LIT;
                        continue;
                    } else if here.op as libc::c_int & 32 as libc::c_int != 0 {
                        (*state).back = -(1 as libc::c_int);
                        (*state).mode = TYPE;
                        continue;
                    } else if here.op as libc::c_int & 64 as libc::c_int != 0 {
                        (*strm).msg = b"invalid literal/length code\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        (*state).extra =
                            here.op as libc::c_uint & 15 as libc::c_int as libc::c_uint;
                        (*state).mode = LENEXT
                    }
                }
                current_block = 2829640705969624118;
            }
            3091652578027905731 => {
                if (*state).flags & 0x400 as libc::c_int != 0 {
                    copy = (*state).length;
                    if copy > have {
                        copy = have
                    }
                    if copy != 0 {
                        if !(*state).head.is_null() && !(*(*state).head).extra.is_null() {
                            len = (*(*state).head).extra_len.wrapping_sub((*state).length);
                            memcpy(
                                (*(*state).head).extra.offset(len as isize) as *mut libc::c_void,
                                next as *const libc::c_void,
                                if len.wrapping_add(copy) > (*(*state).head).extra_max {
                                    (*(*state).head).extra_max.wrapping_sub(len)
                                } else {
                                    copy
                                } as libc::c_ulong,
                            );
                        }
                        if (*state).flags & 0x200 as libc::c_int != 0
                            && (*state).wrap & 4 as libc::c_int != 0
                        {
                            (*state).check = crc32((*state).check, next, copy)
                        }
                        have = have.wrapping_sub(copy);
                        next = next.offset(copy as isize);
                        (*state).length = (*state).length.wrapping_sub(copy)
                    }
                    if (*state).length != 0 {
                        break;
                    }
                }
                (*state).length = 0 as libc::c_int as libc::c_uint;
                (*state).mode = NAME;
                current_block = 10761099658514198778;
            }
            _ => {}
        }
        match current_block {
            2829640705969624118 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh26 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh26 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    (*state).length = (*state).length.wrapping_add(
                        hold as libc::c_uint
                            & ((1 as libc::c_uint) << (*state).extra)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state).back = ((*state).back as libc::c_uint).wrapping_add((*state).extra)
                        as libc::c_int as libc::c_int
                }
                (*state).was = (*state).length;
                (*state).mode = DIST;
                current_block = 13250426973517230943;
            }
            10761099658514198778 => {
                if (*state).flags & 0x800 as libc::c_int != 0 {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                    copy = 0 as libc::c_int as libc::c_uint;
                    loop {
                        let fresh5 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(fresh5 as isize) as libc::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).name.is_null()
                            && (*state).length < (*(*state).head).name_max
                        {
                            let fresh6 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head).name.offset(fresh6 as isize) = len as Bytef
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0
                        && (*state).wrap & 4 as libc::c_int != 0
                    {
                        (*state).check = crc32((*state).check, next, copy)
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
                        break;
                    }
                } else if !(*state).head.is_null() {
                    (*(*state).head).name = Z_NULL as *mut Bytef
                }
                (*state).length = 0 as libc::c_int as libc::c_uint;
                (*state).mode = COMMENT;
                current_block = 18198803391298250726;
            }
            _ => {}
        }
        match current_block {
            13250426973517230943 => {
                loop {
                    here = *(*state).distcode.offset(
                        (hold as libc::c_uint
                            & ((1 as libc::c_uint) << (*state).distbits)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                            as isize,
                    );
                    if here.bits as libc::c_uint <= bits {
                        break;
                    }
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_117;
                    }
                    have = have.wrapping_sub(1);
                    let fresh27 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh27 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                if here.op as libc::c_int & 0xf0 as libc::c_int == 0 as libc::c_int {
                    last = here;
                    loop {
                        here = *(*state).distcode.offset(
                            (last.val as libc::c_uint).wrapping_add(
                                (hold as libc::c_uint
                                    & ((1 as libc::c_uint)
                                        << last.bits as libc::c_int + last.op as libc::c_int)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                    >> last.bits as libc::c_int,
                            ) as isize,
                        );
                        if (last.bits as libc::c_int + here.bits as libc::c_int) as libc::c_uint
                            <= bits
                        {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh28 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh28 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    hold >>= last.bits as libc::c_int;
                    bits = bits.wrapping_sub(last.bits as libc::c_uint);
                    (*state).back += last.bits as libc::c_int
                }
                hold >>= here.bits as libc::c_int;
                bits = bits.wrapping_sub(here.bits as libc::c_uint);
                (*state).back += here.bits as libc::c_int;
                if here.op as libc::c_int & 64 as libc::c_int != 0 {
                    (*strm).msg = b"invalid distance code\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).offset = here.val as libc::c_uint;
                    (*state).extra = here.op as libc::c_uint & 15 as libc::c_int as libc::c_uint;
                    (*state).mode = DISTEXT
                }
                current_block = 5418677336239499507;
            }
            18198803391298250726 => {
                if (*state).flags & 0x1000 as libc::c_int != 0 {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                    copy = 0 as libc::c_int as libc::c_uint;
                    loop {
                        let fresh7 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(fresh7 as isize) as libc::c_uint;
                        if !(*state).head.is_null()
                            && !(*(*state).head).comment.is_null()
                            && (*state).length < (*(*state).head).comm_max
                        {
                            let fresh8 = (*state).length;
                            (*state).length = (*state).length.wrapping_add(1);
                            *(*(*state).head).comment.offset(fresh8 as isize) = len as Bytef
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0
                        && (*state).wrap & 4 as libc::c_int != 0
                    {
                        (*state).check = crc32((*state).check, next, copy)
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
                        break;
                    }
                } else if !(*state).head.is_null() {
                    (*(*state).head).comment = Z_NULL as *mut Bytef
                }
                (*state).mode = HCRC;
                current_block = 12478305818333681747;
            }
            _ => {}
        }
        match current_block {
            5418677336239499507 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh29 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh29 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    (*state).offset = (*state).offset.wrapping_add(
                        hold as libc::c_uint
                            & ((1 as libc::c_uint) << (*state).extra)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state).back = ((*state).back as libc::c_uint).wrapping_add((*state).extra)
                        as libc::c_int as libc::c_int
                }
                (*state).mode = MATCH
            }
            12478305818333681747 => {
                if (*state).flags & 0x200 as libc::c_int != 0 {
                    while bits < 16 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_117;
                        }
                        have = have.wrapping_sub(1);
                        let fresh9 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh9 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if (*state).wrap & 4 as libc::c_int != 0
                        && hold != (*state).check & 0xffff as libc::c_int as libc::c_ulong
                    {
                        (*strm).msg = b"header crc mismatch\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint
                    }
                }
                if !(*state).head.is_null() {
                    (*(*state).head).hcrc = (*state).flags >> 9 as libc::c_int & 1 as libc::c_int;
                    (*(*state).head).done = 1 as libc::c_int
                }
                (*state).check = crc32(
                    0 as libc::c_long as uLong,
                    Z_NULL as *const Bytef,
                    0 as libc::c_int as uInt,
                );
                (*strm).adler = (*state).check;
                (*state).mode = TYPE;
                continue;
            }
            _ => {}
        }
        if left == 0 as libc::c_int as libc::c_uint {
            break;
        }
        copy = out.wrapping_sub(left);
        if (*state).offset > copy {
            /* copy from window */
            copy = (*state).offset.wrapping_sub(copy);
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    (*strm).msg = b"invalid distance too far back\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                }
            }
            if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                from = (*state)
                    .window
                    .offset((*state).wsize.wrapping_sub(copy) as isize)
            } else {
                from = (*state)
                    .window
                    .offset((*state).wnext.wrapping_sub(copy) as isize)
            }
            if copy > (*state).length {
                copy = (*state).length
            }
        } else {
            /* copy from output */
            from = put.offset(-((*state).offset as isize));
            copy = (*state).length
        }
        if copy > left {
            copy = left
        }
        left = left.wrapping_sub(copy);
        (*state).length = (*state).length.wrapping_sub(copy);
        loop {
            let fresh30 = from;
            from = from.offset(1);
            let fresh31 = put;
            put = put.offset(1);
            *fresh31 = *fresh30;
            copy = copy.wrapping_sub(1);
            if !(copy != 0) {
                break;
            }
        }
        if (*state).length == 0 as libc::c_int as libc::c_uint {
            (*state).mode = LEN
        }
    }
    /*
      Return from inflate(), updating the total counts and the check value.
      If there was no progress during the inflate() call, return a buffer
      error.  Call updatewindow() to create and/or update the window state.
      Note: a memory error from inflate() is non-recoverable.
    */
    (*strm).next_out = put;
    (*strm).avail_out = left;
    (*strm).next_in = next;
    (*strm).avail_in = have;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0
        || out != (*strm).avail_out
            && ((*state).mode as libc::c_uint) < BAD as libc::c_int as libc::c_uint
            && (((*state).mode as libc::c_uint) < CHECK as libc::c_int as libc::c_uint
                || flush != Z_FINISH)
    {
        if updatewindow(strm, (*strm).next_out, out.wrapping_sub((*strm).avail_out)) != 0 {
            (*state).mode = MEM;
            return Z_MEM_ERROR;
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in);
    out = out.wrapping_sub((*strm).avail_out);
    (*strm).total_in =
        ((*strm).total_in as libc::c_ulong).wrapping_add(in_0 as libc::c_ulong) as uLong as uLong;
    (*strm).total_out =
        ((*strm).total_out as libc::c_ulong).wrapping_add(out as libc::c_ulong) as uLong as uLong;
    (*state).total = (*state).total.wrapping_add(out as libc::c_ulong);
    if (*state).wrap & 4 as libc::c_int != 0 && out != 0 {
        (*state).check = if (*state).flags != 0 {
            crc32(
                (*state).check,
                (*strm).next_out.offset(-(out as isize)),
                out,
            )
        } else {
            adler32(
                (*state).check,
                (*strm).next_out.offset(-(out as isize)),
                out,
            )
        };
        (*strm).adler = (*state).check
    }
    (*strm).data_type = (*state).bits as libc::c_int
        + (if (*state).last != 0 {
            64 as libc::c_int
        } else {
            0 as libc::c_int
        })
        + (if (*state).mode as libc::c_uint == TYPE as libc::c_int as libc::c_uint {
            128 as libc::c_int
        } else {
            0 as libc::c_int
        })
        + (if (*state).mode as libc::c_uint == LEN_ as libc::c_int as libc::c_uint
            || (*state).mode as libc::c_uint == COPY_ as libc::c_int as libc::c_uint
        {
            256 as libc::c_int
        } else {
            0 as libc::c_int
        });
    if (in_0 == 0 as libc::c_int as libc::c_uint && out == 0 as libc::c_int as libc::c_uint
        || flush == Z_FINISH)
        && ret == Z_OK
    {
        ret = Z_BUF_ERROR
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn inflateEnd(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if !(*state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as voidpf,
    );
    (*strm).state = Z_NULL as *mut internal_state;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateGetDictionary(
    mut strm: z_streamp,
    mut dictionary: *mut Bytef,
    mut dictLength: *mut uInt,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    /* check state */
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    /* copy dictionary */
    if (*state).whave != 0 && !dictionary.is_null() {
        memcpy(
            dictionary as *mut libc::c_void,
            (*state).window.offset((*state).wnext as isize) as *const libc::c_void,
            (*state).whave.wrapping_sub((*state).wnext) as libc::c_ulong,
        );
        memcpy(
            dictionary
                .offset((*state).whave as isize)
                .offset(-((*state).wnext as isize)) as *mut libc::c_void,
            (*state).window as *const libc::c_void,
            (*state).wnext as libc::c_ulong,
        );
    }
    if !dictLength.is_null() {
        *dictLength = (*state).whave
    }
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut dictid: libc::c_ulong = 0;
    let mut ret: libc::c_int = 0;
    /* check state */
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap != 0 as libc::c_int
        && (*state).mode as libc::c_uint != DICT as libc::c_int as libc::c_uint
    {
        return Z_STREAM_ERROR;
    }
    /* check for correct dictionary identifier */
    if (*state).mode as libc::c_uint == DICT as libc::c_int as libc::c_uint {
        dictid = adler32(
            0 as libc::c_long as uLong,
            Z_NULL as *const Bytef,
            0 as libc::c_int as uInt,
        );
        dictid = adler32(dictid, dictionary, dictLength);
        if dictid != (*state).check {
            return Z_DATA_ERROR;
        }
    }
    /* copy dictionary to window using updatewindow(), which will amend the
    existing dictionary if appropriate */
    ret = updatewindow(strm, dictionary.offset(dictLength as isize), dictLength);
    if ret != 0 {
        (*state).mode = MEM;
        return Z_MEM_ERROR;
    }
    (*state).havedict = 1 as libc::c_int;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateGetHeader(
    mut strm: z_streamp,
    mut head: gz_headerp,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    /* check state */
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap & 2 as libc::c_int == 0 as libc::c_int {
        return Z_STREAM_ERROR;
    }
    /* save header structure */
    (*state).head = head;
    (*head).done = 0 as libc::c_int;
    return Z_OK;
}
/*
  Search buf[0..len-1] for the pattern: 0, 0, 0xff, 0xff.  Return when found
  or when out of input.  When called, *have is the number of pattern bytes
  found in order so far, in 0..3.  On return *have is updated to the new
  state.  If on return *have equals four, then the pattern was found and the
  return value is how many bytes were read including the last byte of the
  pattern.  If *have is less than four, then the pattern has not been found
  yet and the return value is len.  In the latter case, syncsearch() can be
  called again with more data and the *have state.  *have is initialized to
  zero for the first call.
*/
unsafe extern "C" fn syncsearch(
    mut have: *mut libc::c_uint,
    mut buf: *const libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_uint {
    let mut got: libc::c_uint = 0; /* number of bytes to look at or looked at */
    let mut next: libc::c_uint = 0; /* temporary to save total_in and total_out */
    got = *have; /* to restore bit buffer to byte string */
    next = 0 as libc::c_int as libc::c_uint;
    while next < len && got < 4 as libc::c_int as libc::c_uint {
        if *buf.offset(next as isize) as libc::c_int
            == (if got < 2 as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                0xff as libc::c_int
            })
        {
            got = got.wrapping_add(1)
        } else if *buf.offset(next as isize) != 0 {
            got = 0 as libc::c_int as libc::c_uint
        } else {
            got = (4 as libc::c_int as libc::c_uint).wrapping_sub(got)
        }
        next = next.wrapping_add(1)
    }
    *have = got;
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn inflateSync(mut strm: z_streamp) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    let mut in_0: libc::c_ulong = 0;
    let mut out: libc::c_ulong = 0;
    let mut buf: [libc::c_uchar; 4] = [0; 4];
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    /* check parameters */
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
        && (*state).bits < 8 as libc::c_int as libc::c_uint
    {
        return Z_BUF_ERROR;
    }
    /* if first time, start search in bit buffer */
    if (*state).mode as libc::c_uint != SYNC as libc::c_int as libc::c_uint {
        (*state).mode = SYNC;
        (*state).hold <<= (*state).bits & 7 as libc::c_int as libc::c_uint;
        (*state).bits = (*state)
            .bits
            .wrapping_sub((*state).bits & 7 as libc::c_int as libc::c_uint);
        len = 0 as libc::c_int as libc::c_uint;
        while (*state).bits >= 8 as libc::c_int as libc::c_uint {
            let fresh35 = len;
            len = len.wrapping_add(1);
            buf[fresh35 as usize] = (*state).hold as libc::c_uchar;
            (*state).hold >>= 8 as libc::c_int;
            (*state).bits = (*state).bits.wrapping_sub(8 as libc::c_int as libc::c_uint)
        }
        (*state).have = 0 as libc::c_int as libc::c_uint;
        syncsearch(&mut (*state).have, buf.as_mut_ptr(), len);
    }
    /* search available input */
    len = syncsearch(&mut (*state).have, (*strm).next_in, (*strm).avail_in);
    (*strm).avail_in = ((*strm).avail_in as libc::c_uint).wrapping_sub(len) as uInt as uInt;
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in =
        ((*strm).total_in as libc::c_ulong).wrapping_add(len as libc::c_ulong) as uLong as uLong;
    /* return no joy or set up to restart inflate() on a new block */
    if (*state).have != 4 as libc::c_int as libc::c_uint {
        return Z_DATA_ERROR;
    }
    in_0 = (*strm).total_in;
    out = (*strm).total_out;
    inflateReset(strm);
    (*strm).total_in = in_0;
    (*strm).total_out = out;
    (*state).mode = TYPE;
    return Z_OK;
}
/*
  Returns true if inflate is currently at the end of a block generated by
  Z_SYNC_FLUSH or Z_FULL_FLUSH. This function is used by one PPP
  implementation to provide an additional safety check. PPP uses
  Z_SYNC_FLUSH but removes the length bytes of the resulting empty stored
  block. When decompressing, PPP checks that at the end of input packet,
  inflate is waiting for these length bytes.
*/
#[no_mangle]
pub unsafe extern "C" fn inflateSyncPoint(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    return ((*state).mode as libc::c_uint == STORED as libc::c_int as libc::c_uint
        && (*state).bits == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn inflateCopy(mut dest: z_streamp, mut source: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut copy: *mut inflate_state = 0 as *mut inflate_state;
    let mut window: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wsize: libc::c_uint = 0;
    /* check input */
    if inflateStateCheck(source) != 0 || dest.is_null() {
        return Z_STREAM_ERROR;
    }
    state = (*source).state as *mut inflate_state;
    /* allocate space */
    copy = Some((*source).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*source).opaque,
        1 as libc::c_int as uInt,
        ::std::mem::size_of::<inflate_state>() as libc::c_ulong as uInt,
    ) as *mut inflate_state;
    if copy.is_null() {
        return Z_MEM_ERROR;
    }
    window = Z_NULL as *mut libc::c_uchar;
    if !(*state).window.is_null() {
        window = Some((*source).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*source).opaque,
            (1 as libc::c_uint) << (*state).wbits,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as uInt,
        ) as *mut libc::c_uchar;
        if window.is_null() {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")((*source).opaque, copy as voidpf);
            return Z_MEM_ERROR;
        }
    }
    /* copy state */
    memcpy(
        dest as voidpf,
        source as voidpf as *const libc::c_void,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong,
    );
    memcpy(
        copy as voidpf,
        state as voidpf as *const libc::c_void,
        ::std::mem::size_of::<inflate_state>() as libc::c_ulong,
    );
    (*copy).strm = dest;
    if (*state).lencode >= (*state).codes.as_mut_ptr()
        && (*state).lencode
            <= (*state)
                .codes
                .as_mut_ptr()
                .offset(ENOUGH as isize)
                .offset(-(1 as libc::c_int as isize))
    {
        (*copy).lencode = (*copy).codes.as_mut_ptr().offset(
            (*state)
                .lencode
                .offset_from((*state).codes.as_mut_ptr()) as libc::c_long
                as isize,
        );
        (*copy).distcode = (*copy).codes.as_mut_ptr().offset(
            (*state)
                .distcode
                .offset_from((*state).codes.as_mut_ptr()) as libc::c_long
                as isize,
        )
    }
    (*copy).next = (*copy).codes.as_mut_ptr().offset(
        (*state)
            .next
            .offset_from((*state).codes.as_mut_ptr()) as libc::c_long as isize,
    );
    if !window.is_null() {
        wsize = (1 as libc::c_uint) << (*state).wbits;
        memcpy(
            window as *mut libc::c_void,
            (*state).window as *const libc::c_void,
            wsize as libc::c_ulong,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut internal_state;
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateUndermine(
    mut strm: z_streamp,
    mut subvert: libc::c_int,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    (*state).sane = 1 as libc::c_int;
    return Z_DATA_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn inflateValidate(
    mut strm: z_streamp,
    mut check: libc::c_int,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    state = (*strm).state as *mut inflate_state;
    if check != 0 {
        (*state).wrap |= 4 as libc::c_int
    } else {
        (*state).wrap &= !(4 as libc::c_int)
    }
    return Z_OK;
}
#[no_mangle]
pub unsafe extern "C" fn inflateMark(mut strm: z_streamp) -> libc::c_long {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return -((1 as libc::c_long) << 16 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    return (((*state).back as libc::c_long as libc::c_ulong) << 16 as libc::c_int) as libc::c_long
        + (if (*state).mode as libc::c_uint == COPY as libc::c_int as libc::c_uint {
            (*state).length
        } else {
            (if (*state).mode as libc::c_uint == MATCH as libc::c_int as libc::c_uint {
                (*state).was.wrapping_sub((*state).length)
            } else {
                0 as libc::c_int as libc::c_uint
            })
        }) as libc::c_long;
}
/*
    deflatePending() returns the number of bytes and bits of output that have
  been generated, but not yet provided in the available output.  The bytes not
  provided would be due to the available output space having being consumed.
  The number of bits of output not provided are between 0 and 7, where they
  await more bits to join them in order to fill out a full byte.  If pending
  or bits are Z_NULL, then those values are not set.

    deflatePending returns Z_OK if success, or Z_STREAM_ERROR if the source
  stream state was inconsistent.
*/
/*
     deflatePrime() inserts bits in the deflate output stream.  The intent
   is that this function is used to start off the deflate output with the bits
   leftover from a previous deflate stream when appending to it.  As such, this
   function can only be used for raw deflate, and must be used before the first
   deflate() call after a deflateInit2() or deflateReset().  bits must be less
   than or equal to 16, and that many of the least significant bits of value
   will be inserted in the output.

     deflatePrime returns Z_OK if success, Z_BUF_ERROR if there was not enough
   room in the internal buffer to insert the bits, or Z_STREAM_ERROR if the
   source stream state was inconsistent.
*/
/*
     deflateSetHeader() provides gzip header information for when a gzip
   stream is requested by deflateInit2().  deflateSetHeader() may be called
   after deflateInit2() or deflateReset() and before the first call of
   deflate().  The text, time, os, extra field, name, and comment information
   in the provided gz_header structure are written to the gzip header (xflag is
   ignored -- the extra flags are set according to the compression level).  The
   caller must assure that, if not Z_NULL, name and comment are terminated with
   a zero byte, and that if extra is not Z_NULL, that extra_len bytes are
   available there.  If hcrc is true, a gzip header crc is included.  Note that
   the current versions of the command-line version of gzip (up through version
   1.3.x) do not support header crc's, and will report that it is a "multi-part
   gzip file" and give up.

     If deflateSetHeader is not used, the default gzip header has text false,
   the time set to zero, and os set to 255, with no extra, name, or comment
   fields.  The gzip header is returned to the default state by deflateReset().

     deflateSetHeader returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
ZEXTERN int ZEXPORT inflateInit2 OF((z_streamp strm,
                                     int  windowBits));

     This is another version of inflateInit with an extra parameter.  The
   fields next_in, avail_in, zalloc, zfree and opaque must be initialized
   before by the caller.

     The windowBits parameter is the base two logarithm of the maximum window
   size (the size of the history buffer).  It should be in the range 8..15 for
   this version of the library.  The default value is 15 if inflateInit is used
   instead.  windowBits must be greater than or equal to the windowBits value
   provided to deflateInit2() while compressing, or it must be equal to 15 if
   deflateInit2() was not used.  If a compressed stream with a larger window
   size is given as input, inflate() will return with the error code
   Z_DATA_ERROR instead of trying to allocate a larger window.

     windowBits can also be zero to request that inflate use the window size in
   the zlib header of the compressed stream.

     windowBits can also be -8..-15 for raw inflate.  In this case, -windowBits
   determines the window size.  inflate() will then process raw deflate data,
   not looking for a zlib or gzip header, not generating a check value, and not
   looking for any check values for comparison at the end of the stream.  This
   is for use with other formats that use the deflate compressed data format
   such as zip.  Those formats provide their own check values.  If a custom
   format is developed using the raw deflate format for compressed data, it is
   recommended that a check value such as an Adler-32 or a CRC-32 be applied to
   the uncompressed data as is done in the zlib, gzip, and zip formats.  For
   most applications, the zlib format should be used as is.  Note that comments
   above on the use in deflateInit2() applies to the magnitude of windowBits.

     windowBits can also be greater than 15 for optional gzip decoding.  Add
   32 to windowBits to enable zlib and gzip decoding with automatic header
   detection, or add 16 to decode only the gzip format (the zlib format will
   return a Z_DATA_ERROR).  If a gzip stream is being decoded, strm->adler is a
   CRC-32 instead of an Adler-32.  Unlike the gunzip utility and gzread() (see
   below), inflate() will not automatically decode concatenated gzip streams.
   inflate() will return Z_STREAM_END at the end of the gzip stream.  The state
   would need to be reset to continue decoding a subsequent gzip stream.

     inflateInit2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_VERSION_ERROR if the zlib library version is incompatible with the
   version assumed by the caller, or Z_STREAM_ERROR if the parameters are
   invalid, such as a null pointer to the structure.  msg is set to null if
   there is no error message.  inflateInit2 does not perform any decompression
   apart from possibly reading the zlib header if present: actual decompression
   will be done by inflate().  (So next_in and avail_in may be modified, but
   next_out and avail_out are unused and unchanged.) The current implementation
   of inflateInit2() does not process any header information -- that is
   deferred until inflate() is called.
*/
/*
     Initializes the decompression dictionary from the given uncompressed byte
   sequence.  This function must be called immediately after a call of inflate,
   if that call returned Z_NEED_DICT.  The dictionary chosen by the compressor
   can be determined from the Adler-32 value returned by that call of inflate.
   The compressor and decompressor must use exactly the same dictionary (see
   deflateSetDictionary).  For raw inflate, this function can be called at any
   time to set the dictionary.  If the provided dictionary is smaller than the
   window and there is already data in the window, then the provided dictionary
   will amend what's there.  The application must insure that the dictionary
   that was used for compression is provided.

     inflateSetDictionary returns Z_OK if success, Z_STREAM_ERROR if a
   parameter is invalid (e.g.  dictionary being Z_NULL) or the stream state is
   inconsistent, Z_DATA_ERROR if the given dictionary doesn't match the
   expected one (incorrect Adler-32 value).  inflateSetDictionary does not
   perform any decompression: this will be done by subsequent calls of
   inflate().
*/
/*
     Returns the sliding dictionary being maintained by inflate.  dictLength is
   set to the number of bytes in the dictionary, and that many bytes are copied
   to dictionary.  dictionary must have enough space, where 32768 bytes is
   always enough.  If inflateGetDictionary() is called with dictionary equal to
   Z_NULL, then only the dictionary length is returned, and nothing is copied.
   Similary, if dictLength is Z_NULL, then it is not set.

     inflateGetDictionary returns Z_OK on success, or Z_STREAM_ERROR if the
   stream state is inconsistent.
*/
/*
     Skips invalid compressed data until a possible full flush point (see above
   for the description of deflate with Z_FULL_FLUSH) can be found, or until all
   available input is skipped.  No output is provided.

     inflateSync searches for a 00 00 FF FF pattern in the compressed data.
   All full flush points have this pattern, but not all occurrences of this
   pattern are full flush points.

     inflateSync returns Z_OK if a possible full flush point has been found,
   Z_BUF_ERROR if no more input was provided, Z_DATA_ERROR if no flush point
   has been found, or Z_STREAM_ERROR if the stream structure was inconsistent.
   In the success case, the application may save the current current value of
   total_in which indicates where valid compressed data was found.  In the
   error case, the application may repeatedly call inflateSync, providing more
   input each time, until success or end of the input data.
*/
/*
     Sets the destination stream as a complete copy of the source stream.

     This function can be useful when randomly accessing a large stream.  The
   first pass through the stream can periodically record the inflate state,
   allowing restarting inflate at those points when randomly accessing the
   stream.

     inflateCopy returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_STREAM_ERROR if the source stream state was inconsistent
   (such as zalloc being Z_NULL).  msg is left unchanged in both source and
   destination.
*/
/*
     This function is equivalent to inflateEnd followed by inflateInit,
   but does not free and reallocate the internal decompression state.  The
   stream will keep attributes that may have been set by inflateInit2.

     inflateReset returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent (such as zalloc or state being Z_NULL).
*/
/*
     This function is the same as inflateReset, but it also permits changing
   the wrap and window size requests.  The windowBits parameter is interpreted
   the same as it is for inflateInit2.  If the window size is changed, then the
   memory allocated for the window is freed, and the window will be reallocated
   by inflate() if needed.

     inflateReset2 returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent (such as zalloc or state being Z_NULL), or if
   the windowBits parameter is invalid.
*/
/*
     This function inserts bits in the inflate input stream.  The intent is
   that this function is used to start inflating at a bit position in the
   middle of a byte.  The provided bits will be used before any bytes are used
   from next_in.  This function should only be used with raw inflate, and
   should be used before the first inflate() call after inflateInit2() or
   inflateReset().  bits must be less than or equal to 16, and that many of the
   least significant bits of value will be inserted in the input.

     If bits is negative, then the input stream bit buffer is emptied.  Then
   inflatePrime() can be called again to put bits in the buffer.  This is used
   to clear out bits leftover after feeding inflate a block description prior
   to feeding inflate codes.

     inflatePrime returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
     This function returns two values, one in the lower 16 bits of the return
   value, and the other in the remaining upper bits, obtained by shifting the
   return value down 16 bits.  If the upper value is -1 and the lower value is
   zero, then inflate() is currently decoding information outside of a block.
   If the upper value is -1 and the lower value is non-zero, then inflate is in
   the middle of a stored block, with the lower value equaling the number of
   bytes from the input remaining to copy.  If the upper value is not -1, then
   it is the number of bits back from the current bit position in the input of
   the code (literal or length/distance pair) currently being processed.  In
   that case the lower value is the number of bytes already emitted for that
   code.

     A code is being processed if inflate is waiting for more input to complete
   decoding of the code, or if it has completed decoding but is waiting for
   more output space to write the literal or match data.

     inflateMark() is used to mark locations in the input data for random
   access, which may be at bit positions, and to note those cases where the
   output of a code may span boundaries of random access blocks.  The current
   location in the input stream can be determined from avail_in and data_type
   as noted in the description for the Z_BLOCK flush parameter for inflate.

     inflateMark returns the value noted above, or -65536 if the provided
   source stream state was inconsistent.
*/
/*
     inflateGetHeader() requests that gzip header information be stored in the
   provided gz_header structure.  inflateGetHeader() may be called after
   inflateInit2() or inflateReset(), and before the first call of inflate().
   As inflate() processes the gzip stream, head->done is zero until the header
   is completed, at which time head->done is set to one.  If a zlib stream is
   being decoded, then head->done is set to -1 to indicate that there will be
   no gzip header information forthcoming.  Note that Z_BLOCK or Z_TREES can be
   used to force inflate() to return immediately after header processing is
   complete and before any actual data is decompressed.

     The text, time, xflags, and os fields are filled in with the gzip header
   contents.  hcrc is set to true if there is a header CRC.  (The header CRC
   was valid if done is set to one.) If extra is not Z_NULL, then extra_max
   contains the maximum number of bytes to write to extra.  Once done is true,
   extra_len contains the actual extra field length, and extra contains the
   extra field, or that field truncated if extra_max is less than extra_len.
   If name is not Z_NULL, then up to name_max characters are written there,
   terminated with a zero unless the length is greater than name_max.  If
   comment is not Z_NULL, then up to comm_max characters are written there,
   terminated with a zero unless the length is greater than comm_max.  When any
   of extra, name, or comment are not Z_NULL and the respective field is not
   present in the header, then that field is set to Z_NULL to signal its
   absence.  This allows the use of deflateSetHeader() with the returned
   structure to duplicate the header.  However if those fields are set to
   allocated memory, then the application will need to save those pointers
   elsewhere so that they can be eventually freed.

     If inflateGetHeader is not used, then the header information is simply
   discarded.  The header is always checked for validity, including the header
   CRC if present.  inflateReset() will reset the process to discard the header
   information.  The application would need to call inflateGetHeader() again to
   retrieve the header from the next gzip stream.

     inflateGetHeader returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
ZEXTERN int ZEXPORT inflateBackInit OF((z_streamp strm, int windowBits,
                                        unsigned char FAR *window));

     Initialize the internal stream state for decompression using inflateBack()
   calls.  The fields zalloc, zfree and opaque in strm must be initialized
   before the call.  If zalloc and zfree are Z_NULL, then the default library-
   derived memory allocation routines are used.  windowBits is the base two
   logarithm of the window size, in the range 8..15.  window is a caller
   supplied buffer of that size.  Except for special applications where it is
   assured that deflate was used with small window sizes, windowBits must be 15
   and a 32K byte window must be supplied to be able to decompress general
   deflate streams.

     See inflateBack() for the usage of these routines.

     inflateBackInit will return Z_OK on success, Z_STREAM_ERROR if any of
   the parameters are invalid, Z_MEM_ERROR if the internal state could not be
   allocated, or Z_VERSION_ERROR if the version of the library does not match
   the version of the header file.
*/
/*
     inflateBack() does a raw inflate with a single call using a call-back
   interface for input and output.  This is potentially more efficient than
   inflate() for file i/o applications, in that it avoids copying between the
   output and the sliding window by simply making the window itself the output
   buffer.  inflate() can be faster on modern CPUs when used with large
   buffers.  inflateBack() trusts the application to not change the output
   buffer passed by the output function, at least until inflateBack() returns.

     inflateBackInit() must be called first to allocate the internal state
   and to initialize the state with the user-provided window buffer.
   inflateBack() may then be used multiple times to inflate a complete, raw
   deflate stream with each call.  inflateBackEnd() is then called to free the
   allocated state.

     A raw deflate stream is one with no zlib or gzip header or trailer.
   This routine would normally be used in a utility that reads zip or gzip
   files and writes out uncompressed files.  The utility would decode the
   header and process the trailer on its own, hence this routine expects only
   the raw deflate stream to decompress.  This is different from the default
   behavior of inflate(), which expects a zlib header and trailer around the
   deflate stream.

     inflateBack() uses two subroutines supplied by the caller that are then
   called by inflateBack() for input and output.  inflateBack() calls those
   routines until it reads a complete deflate stream and writes out all of the
   uncompressed data, or until it encounters an error.  The function's
   parameters and return types are defined above in the in_func and out_func
   typedefs.  inflateBack() will call in(in_desc, &buf) which should return the
   number of bytes of provided input, and a pointer to that input in buf.  If
   there is no input available, in() must return zero -- buf is ignored in that
   case -- and inflateBack() will return a buffer error.  inflateBack() will
   call out(out_desc, buf, len) to write the uncompressed data buf[0..len-1].
   out() should return zero on success, or non-zero on failure.  If out()
   returns non-zero, inflateBack() will return with an error.  Neither in() nor
   out() are permitted to change the contents of the window provided to
   inflateBackInit(), which is also the buffer that out() uses to write from.
   The length written by out() will be at most the window size.  Any non-zero
   amount of input may be provided by in().

     For convenience, inflateBack() can be provided input on the first call by
   setting strm->next_in and strm->avail_in.  If that input is exhausted, then
   in() will be called.  Therefore strm->next_in must be initialized before
   calling inflateBack().  If strm->next_in is Z_NULL, then in() will be called
   immediately for input.  If strm->next_in is not Z_NULL, then strm->avail_in
   must also be initialized, and then if strm->avail_in is not zero, input will
   initially be taken from strm->next_in[0 ..  strm->avail_in - 1].

     The in_desc and out_desc parameters of inflateBack() is passed as the
   first parameter of in() and out() respectively when they are called.  These
   descriptors can be optionally used to pass any information that the caller-
   supplied in() and out() functions need to do their job.

     On return, inflateBack() will set strm->next_in and strm->avail_in to
   pass back any unused input that was provided by the last in() call.  The
   return values of inflateBack() can be Z_STREAM_END on success, Z_BUF_ERROR
   if in() or out() returned an error, Z_DATA_ERROR if there was a format error
   in the deflate stream (in which case strm->msg is set to indicate the nature
   of the error), or Z_STREAM_ERROR if the stream was not properly initialized.
   In the case of Z_BUF_ERROR, an input or output error can be distinguished
   using strm->next_in which will be Z_NULL only if in() returned an error.  If
   strm->next_in is not Z_NULL, then the Z_BUF_ERROR was due to out() returning
   non-zero.  (in() will always be called before out(), so strm->next_in is
   assured to be defined if out() returns non-zero.)  Note that inflateBack()
   cannot return Z_OK.
*/
/*
     All memory allocated by inflateBackInit() is freed.

     inflateBackEnd() returns Z_OK on success, or Z_STREAM_ERROR if the stream
   state was inconsistent.
*/
/* Return flags indicating compile-time options.

   Type sizes, two bits each, 00 = 16 bits, 01 = 32, 10 = 64, 11 = other:
    1.0: size of uInt
    3.2: size of uLong
    5.4: size of voidpf (pointer)
    7.6: size of z_off_t

   Compiler, assembler, and debug options:
    8: ZLIB_DEBUG
    9: ASMV or ASMINF -- use ASM code
    10: ZLIB_WINAPI -- exported functions use the WINAPI calling convention
    11: 0 (reserved)

   One-time table building (smaller code, but not thread-safe if true):
    12: BUILDFIXED -- build static block decoding tables when needed
    13: DYNAMIC_CRC_TABLE -- build CRC calculation tables when needed
    14,15: 0 (reserved)

   Library content (indicates missing functionality):
    16: NO_GZCOMPRESS -- gz* functions cannot compress (to avoid linking
                         deflate code when not needed)
    17: NO_GZIP -- deflate can't write gzip streams, and inflate can't detect
                   and decode gzip streams (to avoid linking crc code)
    18-19: 0 (reserved)

   Operation variations (changes in library functionality):
    20: PKZIP_BUG_WORKAROUND -- slightly more permissive inflate
    21: FASTEST -- deflate algorithm with only one, lowest compression level
    22,23: 0 (reserved)

   The sprintf variant used by gzprintf (zero is best):
    24: 0 = vs*, 1 = s* -- 1 means limited to 20 arguments after the format
    25: 0 = *nprintf, 1 = *printf -- 1 means gzprintf() not secure!
    26: 0 = returns value, 1 = void -- 1 means inferred string length returned

   Remainder:
    27-31: 0 (reserved)
*/
/* utility functions */
/*
     The following utility functions are implemented on top of the basic
   stream-oriented functions.  To simplify the interface, some default options
   are assumed (compression level and memory usage, standard memory allocation
   functions).  The source code of these utility functions can be modified if
   you need special options.
*/
/*
     Compresses the source buffer into the destination buffer.  sourceLen is
   the byte length of the source buffer.  Upon entry, destLen is the total size
   of the destination buffer, which must be at least the value returned by
   compressBound(sourceLen).  Upon exit, destLen is the actual size of the
   compressed data.  compress() is equivalent to compress2() with a level
   parameter of Z_DEFAULT_COMPRESSION.

     compress returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_BUF_ERROR if there was not enough room in the output
   buffer.
*/
/*
     Compresses the source buffer into the destination buffer.  The level
   parameter has the same meaning as in deflateInit.  sourceLen is the byte
   length of the source buffer.  Upon entry, destLen is the total size of the
   destination buffer, which must be at least the value returned by
   compressBound(sourceLen).  Upon exit, destLen is the actual size of the
   compressed data.

     compress2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_BUF_ERROR if there was not enough room in the output buffer,
   Z_STREAM_ERROR if the level parameter is invalid.
*/
/*
     compressBound() returns an upper bound on the compressed size after
   compress() or compress2() on sourceLen bytes.  It would be used before a
   compress() or compress2() call to allocate the destination buffer.
*/
/*
     Decompresses the source buffer into the destination buffer.  sourceLen is
   the byte length of the source buffer.  Upon entry, destLen is the total size
   of the destination buffer, which must be large enough to hold the entire
   uncompressed data.  (The size of the uncompressed data must have been saved
   previously by the compressor and transmitted to the decompressor by some
   mechanism outside the scope of this compression library.) Upon exit, destLen
   is the actual size of the uncompressed data.

     uncompress returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_BUF_ERROR if there was not enough room in the output
   buffer, or Z_DATA_ERROR if the input data was corrupted or incomplete.  In
   the case where there is not enough room, uncompress() will fill the output
   buffer with the uncompressed data up to that point.
*/
/*
     Same as uncompress, except that sourceLen is a pointer, where the
   length of the source is *sourceLen.  On return, *sourceLen is the number of
   source bytes consumed.
*/
/* gzip file access functions */
/*
     This library supports reading and writing files in gzip (.gz) format with
   an interface similar to that of stdio, using the functions that start with
   "gz".  The gzip format is different from the zlib format.  gzip is a gzip
   wrapper, documented in RFC 1952, wrapped around a deflate stream.
*/
/* semi-opaque gzip file descriptor */
/*
ZEXTERN gzFile ZEXPORT gzopen OF((const char *path, const char *mode));

     Opens a gzip (.gz) file for reading or writing.  The mode parameter is as
   in fopen ("rb" or "wb") but can also include a compression level ("wb9") or
   a strategy: 'f' for filtered data as in "wb6f", 'h' for Huffman-only
   compression as in "wb1h", 'R' for run-length encoding as in "wb1R", or 'F'
   for fixed code compression as in "wb9F".  (See the description of
   deflateInit2 for more information about the strategy parameter.)  'T' will
   request transparent writing or appending with no compression and not using
   the gzip format.

     "a" can be used instead of "w" to request that the gzip stream that will
   be written be appended to the file.  "+" will result in an error, since
   reading and writing to the same gzip file is not supported.  The addition of
   "x" when writing will create the file exclusively, which fails if the file
   already exists.  On systems that support it, the addition of "e" when
   reading or writing will set the flag to close the file on an execve() call.

     These functions, as well as gzip, will read and decode a sequence of gzip
   streams in a file.  The append function of gzopen() can be used to create
   such a file.  (Also see gzflush() for another way to do this.)  When
   appending, gzopen does not test whether the file begins with a gzip stream,
   nor does it look for the end of the gzip streams to begin appending.  gzopen
   will simply append a gzip stream to the existing file.

     gzopen can be used to read a file which is not in gzip format; in this
   case gzread will directly read from the file without decompression.  When
   reading, this will be detected automatically by looking for the magic two-
   byte gzip header.

     gzopen returns NULL if the file could not be opened, if there was
   insufficient memory to allocate the gzFile state, or if an invalid mode was
   specified (an 'r', 'w', or 'a' was not provided, or '+' was provided).
   errno can be checked to determine if the reason gzopen failed was that the
   file could not be opened.
*/
/*
     gzdopen associates a gzFile with the file descriptor fd.  File descriptors
   are obtained from calls like open, dup, creat, pipe or fileno (if the file
   has been previously opened with fopen).  The mode parameter is as in gzopen.

     The next call of gzclose on the returned gzFile will also close the file
   descriptor fd, just like fclose(fdopen(fd, mode)) closes the file descriptor
   fd.  If you want to keep fd open, use fd = dup(fd_keep); gz = gzdopen(fd,
   mode);.  The duplicated descriptor should be saved to avoid a leak, since
   gzdopen does not close fd if it fails.  If you are using fileno() to get the
   file descriptor from a FILE *, then you will have to use dup() to avoid
   double-close()ing the file descriptor.  Both gzclose() and fclose() will
   close the associated file descriptor, so they need to have different file
   descriptors.

     gzdopen returns NULL if there was insufficient memory to allocate the
   gzFile state, if an invalid mode was specified (an 'r', 'w', or 'a' was not
   provided, or '+' was provided), or if fd is -1.  The file descriptor is not
   used until the next gz* read, write, seek, or close operation, so gzdopen
   will not detect if fd is invalid (unless fd is -1).
*/
/*
     Set the internal buffer size used by this library's functions.  The
   default buffer size is 8192 bytes.  This function must be called after
   gzopen() or gzdopen(), and before any other calls that read or write the
   file.  The buffer memory allocation is always deferred to the first read or
   write.  Three times that size in buffer space is allocated.  A larger buffer
   size of, for example, 64K or 128K bytes will noticeably increase the speed
   of decompression (reading).

     The new buffer size also affects the maximum length for gzprintf().

     gzbuffer() returns 0 on success, or -1 on failure, such as being called
   too late.
*/
/*
     Dynamically update the compression level or strategy.  See the description
   of deflateInit2 for the meaning of these parameters.  Previously provided
   data is flushed before the parameter change.

     gzsetparams returns Z_OK if success, Z_STREAM_ERROR if the file was not
   opened for writing, Z_ERRNO if there is an error writing the flushed data,
   or Z_MEM_ERROR if there is a memory allocation error.
*/
/*
     Reads the given number of uncompressed bytes from the compressed file.  If
   the input file is not in gzip format, gzread copies the given number of
   bytes into the buffer directly from the file.

     After reaching the end of a gzip stream in the input, gzread will continue
   to read, looking for another gzip stream.  Any number of gzip streams may be
   concatenated in the input file, and will all be decompressed by gzread().
   If something other than a gzip stream is encountered after a gzip stream,
   that remaining trailing garbage is ignored (and no error is returned).

     gzread can be used to read a gzip file that is being concurrently written.
   Upon reaching the end of the input, gzread will return with the available
   data.  If the error code returned by gzerror is Z_OK or Z_BUF_ERROR, then
   gzclearerr can be used to clear the end of file indicator in order to permit
   gzread to be tried again.  Z_OK indicates that a gzip stream was completed
   on the last gzread.  Z_BUF_ERROR indicates that the input file ended in the
   middle of a gzip stream.  Note that gzread does not return -1 in the event
   of an incomplete gzip stream.  This error is deferred until gzclose(), which
   will return Z_BUF_ERROR if the last gzread ended in the middle of a gzip
   stream.  Alternatively, gzerror can be used before gzclose to detect this
   case.

     gzread returns the number of uncompressed bytes actually read, less than
   len for end of file, or -1 for error.  If len is too large to fit in an int,
   then nothing is read, -1 is returned, and the error state is set to
   Z_STREAM_ERROR.
*/
/*
     Read up to nitems items of size size from file to buf, otherwise operating
   as gzread() does.  This duplicates the interface of stdio's fread(), with
   size_t request and return types.  If the library defines size_t, then
   z_size_t is identical to size_t.  If not, then z_size_t is an unsigned
   integer type that can contain a pointer.

     gzfread() returns the number of full items read of size size, or zero if
   the end of the file was reached and a full item could not be read, or if
   there was an error.  gzerror() must be consulted if zero is returned in
   order to determine if there was an error.  If the multiplication of size and
   nitems overflows, i.e. the product does not fit in a z_size_t, then nothing
   is read, zero is returned, and the error state is set to Z_STREAM_ERROR.

     In the event that the end of file is reached and only a partial item is
   available at the end, i.e. the remaining uncompressed data length is not a
   multiple of size, then the final partial item is nevetheless read into buf
   and the end-of-file flag is set.  The length of the partial item read is not
   provided, but could be inferred from the result of gztell().  This behavior
   is the same as the behavior of fread() implementations in common libraries,
   but it prevents the direct use of gzfread() to read a concurrently written
   file, reseting and retrying on end-of-file, when size is not 1.
*/
/*
     Writes the given number of uncompressed bytes into the compressed file.
   gzwrite returns the number of uncompressed bytes written or 0 in case of
   error.
*/
/*
     gzfwrite() writes nitems items of size size from buf to file, duplicating
   the interface of stdio's fwrite(), with size_t request and return types.  If
   the library defines size_t, then z_size_t is identical to size_t.  If not,
   then z_size_t is an unsigned integer type that can contain a pointer.

     gzfwrite() returns the number of full items written of size size, or zero
   if there was an error.  If the multiplication of size and nitems overflows,
   i.e. the product does not fit in a z_size_t, then nothing is written, zero
   is returned, and the error state is set to Z_STREAM_ERROR.
*/
/*
     Converts, formats, and writes the arguments to the compressed file under
   control of the format string, as in fprintf.  gzprintf returns the number of
   uncompressed bytes actually written, or a negative zlib error code in case
   of error.  The number of uncompressed bytes written is limited to 8191, or
   one less than the buffer size given to gzbuffer().  The caller should assure
   that this limit is not exceeded.  If it is exceeded, then gzprintf() will
   return an error (0) with nothing written.  In this case, there may also be a
   buffer overflow with unpredictable consequences, which is possible only if
   zlib was compiled with the insecure functions sprintf() or vsprintf()
   because the secure snprintf() or vsnprintf() functions were not available.
   This can be determined using zlibCompileFlags().
*/
/*
     Writes the given null-terminated string to the compressed file, excluding
   the terminating null character.

     gzputs returns the number of characters written, or -1 in case of error.
*/
/*
     Reads bytes from the compressed file until len-1 characters are read, or a
   newline character is read and transferred to buf, or an end-of-file
   condition is encountered.  If any characters are read or if len == 1, the
   string is terminated with a null character.  If no characters are read due
   to an end-of-file or len < 1, then the buffer is left untouched.

     gzgets returns buf which is a null-terminated string, or it returns NULL
   for end-of-file or in case of error.  If there was an error, the contents at
   buf are indeterminate.
*/
/*
     Writes c, converted to an unsigned char, into the compressed file.  gzputc
   returns the value that was written, or -1 in case of error.
*/
/*
     Reads one byte from the compressed file.  gzgetc returns this byte or -1
   in case of end of file or error.  This is implemented as a macro for speed.
   As such, it does not do all of the checking the other functions do.  I.e.
   it does not check to see if file is NULL, nor whether the structure file
   points to has been clobbered or not.
*/
/*
     Push one character back onto the stream to be read as the first character
   on the next read.  At least one character of push-back is allowed.
   gzungetc() returns the character pushed, or -1 on failure.  gzungetc() will
   fail if c is -1, and may fail if a character has been pushed but not read
   yet.  If gzungetc is used immediately after gzopen or gzdopen, at least the
   output buffer size of pushed characters is allowed.  (See gzbuffer above.)
   The pushed character will be discarded if the stream is repositioned with
   gzseek() or gzrewind().
*/
/*
     Flushes all pending output into the compressed file.  The parameter flush
   is as in the deflate() function.  The return value is the zlib error number
   (see function gzerror below).  gzflush is only permitted when writing.

     If the flush parameter is Z_FINISH, the remaining data is written and the
   gzip stream is completed in the output.  If gzwrite() is called again, a new
   gzip stream will be started in the output.  gzread() is able to read such
   concatenated gzip streams.

     gzflush should be called only when strictly necessary because it will
   degrade compression if called too often.
*/
/*
ZEXTERN z_off_t ZEXPORT gzseek OF((gzFile file,
                                   z_off_t offset, int whence));

     Sets the starting position for the next gzread or gzwrite on the given
   compressed file.  The offset represents a number of bytes in the
   uncompressed data stream.  The whence parameter is defined as in lseek(2);
   the value SEEK_END is not supported.

     If the file is opened for reading, this function is emulated but can be
   extremely slow.  If the file is opened for writing, only forward seeks are
   supported; gzseek then compresses a sequence of zeroes up to the new
   starting position.

     gzseek returns the resulting offset location as measured in bytes from
   the beginning of the uncompressed stream, or -1 in case of error, in
   particular if the file is opened for writing and the new starting position
   would be before the current position.
*/
/*
     Rewinds the given file. This function is supported only for reading.

     gzrewind(file) is equivalent to (int)gzseek(file, 0L, SEEK_SET)
*/
/*
ZEXTERN z_off_t ZEXPORT    gztell OF((gzFile file));

     Returns the starting position for the next gzread or gzwrite on the given
   compressed file.  This position represents a number of bytes in the
   uncompressed data stream, and is zero when starting, even if appending or
   reading a gzip stream from the middle of a file using gzdopen().

     gztell(file) is equivalent to gzseek(file, 0L, SEEK_CUR)
*/
/*
ZEXTERN z_off_t ZEXPORT gzoffset OF((gzFile file));

     Returns the current offset in the file being read or written.  This offset
   includes the count of bytes that precede the gzip stream, for example when
   appending or when using gzdopen() for reading.  When reading, the offset
   does not include as yet unused buffered input.  This information can be used
   for a progress indicator.  On error, gzoffset() returns -1.
*/
/*
     Returns true (1) if the end-of-file indicator has been set while reading,
   false (0) otherwise.  Note that the end-of-file indicator is set only if the
   read tried to go past the end of the input, but came up short.  Therefore,
   just like feof(), gzeof() may return false even if there is no more data to
   read, in the event that the last read request was for the exact number of
   bytes remaining in the input file.  This will happen if the input file size
   is an exact multiple of the buffer size.

     If gzeof() returns true, then the read functions will return no more data,
   unless the end-of-file indicator is reset by gzclearerr() and the input file
   has grown since the previous end of file was detected.
*/
/*
     Returns true (1) if file is being copied directly while reading, or false
   (0) if file is a gzip stream being decompressed.

     If the input file is empty, gzdirect() will return true, since the input
   does not contain a gzip stream.

     If gzdirect() is used immediately after gzopen() or gzdopen() it will
   cause buffers to be allocated to allow reading the file to determine if it
   is a gzip file.  Therefore if gzbuffer() is used, it should be called before
   gzdirect().

     When writing, gzdirect() returns true (1) if transparent writing was
   requested ("wT" for the gzopen() mode), or false (0) otherwise.  (Note:
   gzdirect() is not needed when writing.  Transparent writing must be
   explicitly requested, so the application already knows the answer.  When
   linking statically, using gzdirect() will include all of the zlib code for
   gzip file reading and decompression, which may not be desired.)
*/
/*
     Flushes all pending output if necessary, closes the compressed file and
   deallocates the (de)compression state.  Note that once file is closed, you
   cannot call gzerror with file, since its structures have been deallocated.
   gzclose must not be called more than once on the same file, just as free
   must not be called more than once on the same allocation.

     gzclose will return Z_STREAM_ERROR if file is not valid, Z_ERRNO on a
   file operation error, Z_MEM_ERROR if out of memory, Z_BUF_ERROR if the
   last read ended in the middle of a gzip stream, or Z_OK on success.
*/
/*
     Same as gzclose(), but gzclose_r() is only for use when reading, and
   gzclose_w() is only for use when writing or appending.  The advantage to
   using these instead of gzclose() is that they avoid linking in zlib
   compression or decompression code that is not used when only reading or only
   writing respectively.  If gzclose() is used, then both compression and
   decompression code will be included the application when linking to a static
   zlib library.
*/
/*
     Returns the error message for the last error which occurred on the given
   compressed file.  errnum is set to zlib error number.  If an error occurred
   in the file system and not in the compression library, errnum is set to
   Z_ERRNO and the application may consult errno to get the exact error code.

     The application must not modify the returned string.  Future calls to
   this function may invalidate the previously returned string.  If file is
   closed, then the string previously returned by gzerror will no longer be
   available.

     gzerror() should be used to distinguish errors from end-of-file for those
   functions above that do not distinguish those cases in their return values.
*/
/*
     Clears the error and end-of-file flags for file.  This is analogous to the
   clearerr() function in stdio.  This is useful for continuing to read a gzip
   file that is being written concurrently.
*/
/* !Z_SOLO */
/* checksum functions */
/*
     These functions are not related to compression but are exported
   anyway because they might be useful in applications using the compression
   library.
*/
/*
     Update a running Adler-32 checksum with the bytes buf[0..len-1] and
   return the updated checksum.  If buf is Z_NULL, this function returns the
   required initial value for the checksum.

     An Adler-32 checksum is almost as reliable as a CRC-32 but can be computed
   much faster.

   Usage example:

     uLong adler = adler32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       adler = adler32(adler, buffer, length);
     }
     if (adler != original_adler) error();
*/
/*
     Same as adler32(), but with a size_t length.
*/
/*
ZEXTERN uLong ZEXPORT adler32_combine OF((uLong adler1, uLong adler2,
                                          z_off_t len2));

     Combine two Adler-32 checksums into one.  For two sequences of bytes, seq1
   and seq2 with lengths len1 and len2, Adler-32 checksums were calculated for
   each, adler1 and adler2.  adler32_combine() returns the Adler-32 checksum of
   seq1 and seq2 concatenated, requiring only adler1, adler2, and len2.  Note
   that the z_off_t type (like off_t) is a signed integer.  If len2 is
   negative, the result has no meaning or utility.
*/
/*
     Update a running CRC-32 with the bytes buf[0..len-1] and return the
   updated CRC-32.  If buf is Z_NULL, this function returns the required
   initial value for the crc.  Pre- and post-conditioning (one's complement) is
   performed within this function so it shouldn't be done by the application.

   Usage example:

     uLong crc = crc32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       crc = crc32(crc, buffer, length);
     }
     if (crc != original_crc) error();
*/
/*
     Same as crc32(), but with a size_t length.
*/
/*
ZEXTERN uLong ZEXPORT crc32_combine OF((uLong crc1, uLong crc2, z_off_t len2));

     Combine two CRC-32 check values into one.  For two sequences of bytes,
   seq1 and seq2 with lengths len1 and len2, CRC-32 check values were
   calculated for each, crc1 and crc2.  crc32_combine() returns the CRC-32
   check value of seq1 and seq2 concatenated, requiring only crc1, crc2, and
   len2.
*/
/* various hacks, don't look :) */
/* deflateInit and inflateInit are macros to allow checking the zlib version
 * and the compiler's view of z_stream:
 */
/* gzgetc() macro and its supporting function and exposed data structure.  Note
 * that the real internal state is much larger than the exposed structure.
 * This abbreviated structure exposes just enough for the gzgetc() macro.  The
 * user should not mess with these exposed elements, since their names or
 * behavior could change in the future, perhaps even capriciously.  They can
 * only be used by the gzgetc() macro.  You have been warned.
 */
/* backward compatibility */
/* provide 64-bit offset functions if _LARGEFILE64_SOURCE defined, and/or
 * change the regular functions to 64 bits if _FILE_OFFSET_BITS is 64 (if
 * both are true, the application gets the *64 functions, and the regular
 * functions are changed to 64 bits) -- in case these are set on systems
 * without large file support, _LFS64_LARGEFILE must also be true
 */
/* Z_SOLO */
/* !Z_SOLO */
/* undocumented functions */
#[no_mangle]
pub unsafe extern "C" fn inflateCodesUsed(mut strm: z_streamp) -> libc::c_ulong {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if inflateStateCheck(strm) != 0 {
        return -(1 as libc::c_int) as libc::c_ulong;
    }
    state = (*strm).state as *mut inflate_state;
    return (*state)
        .next
        .offset_from((*state).codes.as_mut_ptr()) as libc::c_long
        as libc::c_ulong;
}
