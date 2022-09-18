use ::libc;
extern "C" {
    pub type static_tree_desc_s;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn _tr_flush_bits(s: *mut deflate_state);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn _tr_stored_block(s: *mut deflate_state, buf: *mut charf, stored_len: ulg, last: libc::c_int);
    #[no_mangle]
    fn _tr_align(s: *mut deflate_state);
    #[no_mangle]
    fn _tr_flush_block(s: *mut deflate_state, buf: *mut charf, stored_len: ulg, last: libc::c_int);
    /* Mapping from a distance to a distance code. dist is the distance - 1 and
     * must not have side effects. _dist_code[256] and _dist_code[257] are never
     * used.
     */
    /* Inline versions of _tr_tally for speed: */
    #[no_mangle]
    static _dist_code: [uch; 0];
    #[no_mangle]
    static _length_code: [uch; 0];
    #[no_mangle]
    static z_errmsg: [*mut libc::c_char; 10];
    #[no_mangle]
    fn _tr_init(s: *mut deflate_state);
    #[no_mangle]
    fn zcfree(opaque: voidpf, ptr: voidpf);
    #[no_mangle]
    fn zcalloc(opaque: voidpf, items: libc::c_uint, size: libc::c_uint) -> voidpf;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
}
/* zconf.h -- configuration of the zlib compression library
 * Copyright (C) 1995-2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* #undef Z_PREFIX */
/*
 * If you *really* need a unique prefix for all types and library functions,
 * compile with -DZ_PREFIX. The "standard" zlib should be compiled without it.
 * Even better than compiling with -DZ_PREFIX would be to use configure to set
 * this permanently in zconf.h using "./configure --zprefix".
 */
/* may be set to #if 1 by ./configure */
/*
 * Compile with -DMAXSEG_64K if the alloc function cannot allocate more
 * than 64k bytes at a time (needed on systems with 16-bit int).
 */
/* iSeries (formerly AS/400). */
/* Maximum value for memLevel in deflateInit2 */
/* Maximum value for windowBits in deflateInit2 and inflateInit2.
 * WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
 * created by gzip. (Files created by minigzip can still be extracted by
 * gzip.)
 */
/* 32K LZ77 window */
/* The memory requirements for deflate are (in bytes):
            (1 << (windowBits+2)) +  (1 << (memLevel+9))
 that is: 128K for windowBits=15  +  128K for memLevel = 8  (default values)
 plus a few kilobytes for small objects. For example, if you want to reduce
 the default memory requirements from 256K to 128K, compile with
     make CFLAGS="-O -DMAX_WBITS=14 -DMAX_MEM_LEVEL=7"
 Of course this will generally degrade compression (there's no free lunch).

   The memory requirements for inflate are (in bytes) 1 << windowBits
 that is, 32K for windowBits=15 (default value) plus about 7 kilobytes
 for small objects.
*/
/* Type declarations */
/* function prototypes */
/* function prototypes for stdarg */
/* The following definitions for FAR are needed only for MSDOS mixed
 * model programming (small or medium model with some far allocations).
 * This was tested only with MSC; for other MSDOS compilers you may have
 * to define NO_MEMCPY in zutil.h.  If you don't need the mixed model,
 * just define FAR to be empty.
 */
pub type Byte = libc::c_uchar;
/* 8 bits */
pub type uInt = libc::c_uint;
/* 16 bits or more */
pub type uLong = libc::c_ulong;
/* 32 bits or more */
pub type Bytef = Byte;
pub type charf = libc::c_char;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub strm: z_streamp,
    pub status: libc::c_int,
    pub pending_buf: *mut Bytef,
    pub pending_buf_size: ulg,
    pub pending_out: *mut Bytef,
    pub pending: ulg,
    pub wrap: libc::c_int,
    pub gzhead: gz_headerp,
    pub gzindex: ulg,
    pub method: Byte,
    pub last_flush: libc::c_int,
    pub w_size: uInt,
    pub w_bits: uInt,
    pub w_mask: uInt,
    pub window: *mut Bytef,
    pub window_size: ulg,
    pub prev: *mut Posf,
    pub head: *mut Posf,
    pub ins_h: uInt,
    pub hash_size: uInt,
    pub hash_bits: uInt,
    pub hash_mask: uInt,
    pub hash_shift: uInt,
    pub block_start: libc::c_long,
    pub match_length: uInt,
    pub prev_match: IPos,
    pub match_available: libc::c_int,
    pub strstart: uInt,
    pub match_start: uInt,
    pub lookahead: uInt,
    pub prev_length: uInt,
    pub max_chain_length: uInt,
    pub max_lazy_match: uInt,
    pub level: libc::c_int,
    pub strategy: libc::c_int,
    pub good_match: uInt,
    pub nice_match: libc::c_int,
    pub dyn_ltree: [ct_data_s; 573],
    pub dyn_dtree: [ct_data_s; 61],
    pub bl_tree: [ct_data_s; 39],
    pub l_desc: tree_desc_s,
    pub d_desc: tree_desc_s,
    pub bl_desc: tree_desc_s,
    pub bl_count: [ush; 16],
    pub heap: [libc::c_int; 573],
    pub heap_len: libc::c_int,
    pub heap_max: libc::c_int,
    pub depth: [uch; 573],
    pub l_buf: *mut uchf,
    pub lit_bufsize: uInt,
    pub last_lit: uInt,
    pub d_buf: *mut ushf,
    pub opt_len: ulg,
    pub static_len: ulg,
    pub matches: uInt,
    pub insert: uInt,
    pub bi_buf: ush,
    pub bi_valid: libc::c_int,
    pub high_water: ulg,
}
/* zutil.h -- internal interface and configuration of the compression library
 * Copyright (C) 1995-2016 Jean-loup Gailly, Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* WARNING: this file should *not* be used by applications. It is
  part of the implementation of the compression library and is
  subject to change. Applications should only use zlib.h.
*/
/* @(#) $Id$ */
/* since "static" is used to mean two completely different things in C, we
define "local" for the non-static meaning of "static", for readability
(compile with -Dlocal if your debugger can't find static symbols) */
pub type ulg = libc::c_ulong;
pub type ush = libc::c_ushort;
pub type ushf = ush;
pub type uchf = uch;
pub type uch = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree_desc_s {
    pub dyn_tree: *mut ct_data,
    pub max_code: libc::c_int,
    pub stat_desc: *const static_tree_desc,
}
pub type static_tree_desc = static_tree_desc_s;
pub type ct_data = ct_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ct_data_s {
    pub fc: C2RustUnnamed_0,
    pub dl: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dad: ush,
    pub len: ush,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub freq: ush,
    pub code: ush,
}
pub type IPos = libc::c_uint;
pub type Posf = Pos;
pub type Pos = ush;
pub type gz_headerp = *mut gz_header;
pub type gz_header = gz_header_s;
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
pub type z_streamp = *mut z_stream;
pub type z_stream = z_stream_s;
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
pub type deflate_state = internal_state;
pub const block_done: block_state = 1;
pub type block_state = libc::c_uint;
pub const finish_done: block_state = 3;
pub const finish_started: block_state = 2;
pub const need_more: block_state = 0;
pub type compress_func =
    Option<unsafe extern "C" fn(_: *mut deflate_state, _: libc::c_int) -> block_state>;
/* Matches of length 3 are discarded if their distance exceeds TOO_FAR */
/* Values for max_lazy_match, good_match and max_chain_length, depending on
 * the desired pack level (0..9). The values given below have been tuned to
 * exclude worst case performance for pathological files. Better values may be
 * found for specific files.
 */
pub type config = config_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_s {
    pub good_length: ush,
    pub max_lazy: ush,
    pub nice_length: ush,
    pub max_chain: ush,
    pub func: compress_func,
}
/* reduce lazy search above this match length */
/* do not perform lazy search above this match length */
/* quit search above this match length */
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
pub const Z_NEED_DICT: libc::c_int = 2 as libc::c_int;
pub const Z_STREAM_END: libc::c_int = 1 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
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
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_FULL_FLUSH: libc::c_int = 3 as libc::c_int;
pub const Z_BLOCK: libc::c_int = 5 as libc::c_int;
pub const Z_PARTIAL_FLUSH: libc::c_int = 1 as libc::c_int;
pub const FINISH_STATE: libc::c_int = 666 as libc::c_int;
pub const MIN_MATCH: libc::c_int = 3 as libc::c_int;
pub const LITERALS: libc::c_int = 256 as libc::c_int;
pub const MAX_MATCH: libc::c_int = 258 as libc::c_int;
pub const MIN_LOOKAHEAD: libc::c_int = MAX_MATCH + MIN_MATCH + 1 as libc::c_int;
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const WIN_INIT: libc::c_int = 258 as libc::c_int;
pub const HCRC_STATE: libc::c_int = 103 as libc::c_int;
pub const COMMENT_STATE: libc::c_int = 91 as libc::c_int;
pub const NAME_STATE: libc::c_int = 73 as libc::c_int;
pub const EXTRA_STATE: libc::c_int = 69 as libc::c_int;
pub const GZIP_STATE: libc::c_int = 57 as libc::c_int;
pub const PRESET_DICT: libc::c_int = 0x20 as libc::c_int;
pub const INIT_STATE: libc::c_int = 42 as libc::c_int;
pub const BUSY_STATE: libc::c_int = 113 as libc::c_int;
pub const Buf_size: libc::c_int = 16 as libc::c_int;
pub const DEF_MEM_LEVEL: libc::c_int = 8 as libc::c_int;
pub const MAX_WBITS: libc::c_int = 15 as libc::c_int;
pub const MAX_MEM_LEVEL: libc::c_int = 9 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_NULL: libc::c_int = 0 as libc::c_int;
pub const Z_FILTERED: libc::c_int = 1 as libc::c_int;
pub const Z_RLE: libc::c_int = 3 as libc::c_int;
pub const Z_DEFLATED: libc::c_int = 8 as libc::c_int;
pub const Z_STREAM_ERROR: libc::c_int = -(2 as libc::c_int);
pub const Z_DATA_ERROR: libc::c_int = -(3 as libc::c_int);
pub const Z_MEM_ERROR: libc::c_int = -(4 as libc::c_int);
pub const Z_HUFFMAN_ONLY: libc::c_int = 2 as libc::c_int;
pub const Z_UNKNOWN: libc::c_int = 2 as libc::c_int;
pub const Z_VERSION_ERROR: libc::c_int = -(6 as libc::c_int);
pub const Z_BUF_ERROR: libc::c_int = -(5 as libc::c_int);
pub const Z_DEFAULT_STRATEGY: libc::c_int = 0 as libc::c_int;
pub const Z_FIXED: libc::c_int = 4 as libc::c_int;
pub const Z_DEFAULT_COMPRESSION: libc::c_int = -(1 as libc::c_int);
/* deflate.c -- compress data using the deflation algorithm
 * Copyright (C) 1995-2017 Jean-loup Gailly and Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
 *  ALGORITHM
 *
 *      The "deflation" process depends on being able to identify portions
 *      of the input text which are identical to earlier input (within a
 *      sliding window trailing behind the input currently being processed).
 *
 *      The most straightforward technique turns out to be the fastest for
 *      most input files: try all possible matches and select the longest.
 *      The key feature of this algorithm is that insertions into the string
 *      dictionary are very simple and thus fast, and deletions are avoided
 *      completely. Insertions are performed at each input character, whereas
 *      string matches are performed only when the previous match ends. So it
 *      is preferable to spend more time in matches to allow very fast string
 *      insertions and avoid deletions. The matching algorithm for small
 *      strings is inspired from that of Rabin & Karp. A brute force approach
 *      is used to find longer strings when a small match has been found.
 *      A similar algorithm is used in comic (by Jan-Mark Wams) and freeze
 *      (by Leonid Broukhis).
 *         A previous version of this file used a more sophisticated algorithm
 *      (by Fiala and Greene) which is guaranteed to run in linear amortized
 *      time, but has a larger average cost, uses more memory and is patented.
 *      However the F&G algorithm may be faster for some highly redundant
 *      files if the parameter max_chain_length (described below) is too large.
 *
 *  ACKNOWLEDGEMENTS
 *
 *      The idea of lazy evaluation of matches is due to Jan-Mark Wams, and
 *      I found it in 'freeze' written by Leonid Broukhis.
 *      Thanks to many people for bug reports and testing.
 *
 *  REFERENCES
 *
 *      Deutsch, L.P.,"DEFLATE Compressed Data Format Specification".
 *      Available in http://tools.ietf.org/html/rfc1951
 *
 *      A description of the Rabin and Karp algorithm is given in the book
 *         "Algorithms" by R. Sedgewick, Addison-Wesley, p252.
 *
 *      Fiala,E.R., and Greene,D.H.
 *         Data Compression with Finite Windows, Comm.ACM, 32,4 (1989) 490-595
 *
 */
/* @(#) $Id$ */
#[no_mangle]
pub static mut deflate_copyright: [libc::c_char; 69] = unsafe {
    *::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
        b" deflate 1.2.11 Copyright 1995-2017 Jean-loup Gailly and Mark Adler \x00",
    )
};
/* ===========================================================================
 * Local data
 */
pub const NIL: libc::c_int = 0 as libc::c_int;
/* Tail of hash chains */
pub const TOO_FAR: libc::c_int = 4096 as libc::c_int;
static mut configuration_table: [config; 10] = unsafe {
    [
        {
            let mut init = config_s {
                good_length: 0 as libc::c_int as ush,
                max_lazy: 0 as libc::c_int as ush,
                nice_length: 0 as libc::c_int as ush,
                max_chain: 0 as libc::c_int as ush,
                func: Some(
                    deflate_stored
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as libc::c_int as ush,
                max_lazy: 4 as libc::c_int as ush,
                nice_length: 8 as libc::c_int as ush,
                max_chain: 4 as libc::c_int as ush,
                func: Some(
                    deflate_fast
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as libc::c_int as ush,
                max_lazy: 5 as libc::c_int as ush,
                nice_length: 16 as libc::c_int as ush,
                max_chain: 8 as libc::c_int as ush,
                func: Some(
                    deflate_fast
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as libc::c_int as ush,
                max_lazy: 6 as libc::c_int as ush,
                nice_length: 32 as libc::c_int as ush,
                max_chain: 32 as libc::c_int as ush,
                func: Some(
                    deflate_fast
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 4 as libc::c_int as ush,
                max_lazy: 4 as libc::c_int as ush,
                nice_length: 16 as libc::c_int as ush,
                max_chain: 16 as libc::c_int as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 8 as libc::c_int as ush,
                max_lazy: 16 as libc::c_int as ush,
                nice_length: 32 as libc::c_int as ush,
                max_chain: 32 as libc::c_int as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 8 as libc::c_int as ush,
                max_lazy: 16 as libc::c_int as ush,
                nice_length: 128 as libc::c_int as ush,
                max_chain: 128 as libc::c_int as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 8 as libc::c_int as ush,
                max_lazy: 32 as libc::c_int as ush,
                nice_length: 128 as libc::c_int as ush,
                max_chain: 256 as libc::c_int as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 32 as libc::c_int as ush,
                max_lazy: 128 as libc::c_int as ush,
                nice_length: 258 as libc::c_int as ush,
                max_chain: 1024 as libc::c_int as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
        {
            let mut init = config_s {
                good_length: 32 as libc::c_int as ush,
                max_lazy: 258 as libc::c_int as ush,
                nice_length: 258 as libc::c_int as ush,
                max_chain: 4096 as libc::c_int as ush,
                func: Some(
                    deflate_slow
                        as unsafe extern "C" fn(
                            _: *mut deflate_state,
                            _: libc::c_int,
                        ) -> block_state,
                ),
            };
            init
        },
    ]
};
/* ===========================================================================
 * Insert string str in the dictionary and set match_head to the previous head
 * of the hash chain (the most recent string with same hash key). Return
 * the previous length of the hash chain.
 * If this file is compiled with -DFASTEST, the compression level is forced
 * to 1, and no hash chains are maintained.
 * IN  assertion: all calls to INSERT_STRING are made with consecutive input
 *    characters and the first MIN_MATCH bytes of str are valid (except for
 *    the last MIN_MATCH-1 bytes of the input file).
 */
/* ===========================================================================
 * Initialize the hash table (avoiding 64K overflow for 16 bit systems).
 * prev[] will be initialized on the fly.
 */
/* ===========================================================================
 * Slide the hash table when sliding the window down (could be avoided with 32
 * bit values at the expense of memory usage). We slide even when level == 0 to
 * keep the hash table consistent if we switch back to level > 0 later.
 */
unsafe extern "C" fn slide_hash(mut s: *mut deflate_state) {
    let mut n: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    let mut p: *mut Posf = 0 as *mut Posf;
    let mut wsize: uInt = (*s).w_size;
    n = (*s).hash_size;
    p = &mut *(*s).head.offset(n as isize) as *mut Posf;
    loop {
        p = p.offset(-1);
        m = *p as libc::c_uint;
        *p = if m >= wsize {
            m.wrapping_sub(wsize)
        } else {
            NIL as libc::c_uint
        } as Pos;
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    n = wsize;
    p = &mut *(*s).prev.offset(n as isize) as *mut Posf;
    loop {
        p = p.offset(-1);
        m = *p as libc::c_uint;
        *p = if m >= wsize {
            m.wrapping_sub(wsize)
        } else {
            NIL as libc::c_uint
        } as Pos;
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
        /* If n is not on any hash chain, prev[n] is garbage but
         * its value will never be used.
         */
    }
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateInit_(
    mut strm: z_streamp,
    mut level: libc::c_int,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    return deflateInit2_(
        strm,
        level,
        Z_DEFLATED,
        MAX_WBITS,
        DEF_MEM_LEVEL,
        Z_DEFAULT_STRATEGY,
        version,
        stream_size,
    );
    /* To do: ignore strm->next_in if we use it as window */
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateInit2_(
    mut strm: z_streamp,
    mut level: libc::c_int,
    mut method: libc::c_int,
    mut windowBits: libc::c_int,
    mut memLevel: libc::c_int,
    mut strategy: libc::c_int,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut wrap: libc::c_int = 1 as libc::c_int;
    static mut my_version: [libc::c_char; 7] = ZLIB_VERSION;
    let mut overlay: *mut ushf = 0 as *mut ushf;
    /* We overlay pending_buf and d_buf+l_buf. This works since the average
     * output size for (length,distance) codes is <= 24 bits.
     */
    if version.is_null()
        || *version.offset(0 as libc::c_int as isize) as libc::c_int
            != my_version[0 as libc::c_int as usize] as libc::c_int
        || stream_size as libc::c_ulong != ::std::mem::size_of::<z_stream>() as libc::c_ulong
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
    if level == Z_DEFAULT_COMPRESSION {
        level = 6 as libc::c_int
    }
    if windowBits < 0 as libc::c_int {
        /* suppress zlib wrapper */
        wrap = 0 as libc::c_int; /* write gzip wrapper instead */
        windowBits = -windowBits
    } else if windowBits > 15 as libc::c_int {
        wrap = 2 as libc::c_int; /* until 256-byte window bug fixed */
        windowBits -= 16 as libc::c_int
    } /* to pass state test in deflateReset() */
    if memLevel < 1 as libc::c_int
        || memLevel > MAX_MEM_LEVEL
        || method != Z_DEFLATED
        || windowBits < 8 as libc::c_int
        || windowBits > 15 as libc::c_int
        || level < 0 as libc::c_int
        || level > 9 as libc::c_int
        || strategy < 0 as libc::c_int
        || strategy > Z_FIXED
        || windowBits == 8 as libc::c_int && wrap != 1 as libc::c_int
    {
        return Z_STREAM_ERROR;
    } /* nothing written to s->window yet */
    if windowBits == 8 as libc::c_int {
        windowBits = 9 as libc::c_int
    } /* 16K elements by default */
    s = Some((*strm).zalloc.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        1 as libc::c_int as uInt,
        ::std::mem::size_of::<deflate_state>() as libc::c_ulong as uInt,
    ) as *mut deflate_state;
    if s.is_null() {
        return Z_MEM_ERROR;
    }
    (*strm).state = s as *mut internal_state;
    (*s).strm = strm;
    (*s).status = INIT_STATE;
    (*s).wrap = wrap;
    (*s).gzhead = Z_NULL as gz_headerp;
    (*s).w_bits = windowBits as uInt;
    (*s).w_size = ((1 as libc::c_int) << (*s).w_bits) as uInt;
    (*s).w_mask = (*s).w_size.wrapping_sub(1 as libc::c_int as libc::c_uint);
    (*s).hash_bits = (memLevel as uInt).wrapping_add(7 as libc::c_int as libc::c_uint);
    (*s).hash_size = ((1 as libc::c_int) << (*s).hash_bits) as uInt;
    (*s).hash_mask = (*s)
        .hash_size
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    (*s).hash_shift = (*s)
        .hash_bits
        .wrapping_add(MIN_MATCH as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(MIN_MATCH as libc::c_uint);
    (*s).window = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).w_size,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Byte>() as libc::c_ulong) as uInt,
    ) as *mut Bytef;
    (*s).prev = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).w_size,
        ::std::mem::size_of::<Pos>() as libc::c_ulong as uInt,
    ) as *mut Posf;
    (*s).head = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).hash_size,
        ::std::mem::size_of::<Pos>() as libc::c_ulong as uInt,
    ) as *mut Posf;
    (*s).high_water = 0 as libc::c_int as ulg;
    (*s).lit_bufsize = ((1 as libc::c_int) << memLevel + 6 as libc::c_int) as uInt;
    overlay = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        (*s).lit_bufsize,
        (::std::mem::size_of::<ush>() as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as uInt,
    ) as *mut ushf;
    (*s).pending_buf = overlay as *mut uchf;
    (*s).pending_buf_size = ((*s).lit_bufsize as ulg).wrapping_mul(
        (::std::mem::size_of::<ush>() as libc::c_ulong)
            .wrapping_add(2 as libc::c_long as libc::c_ulong),
    );
    if (*s).window.is_null()
        || (*s).prev.is_null()
        || (*s).head.is_null()
        || (*s).pending_buf.is_null()
    {
        (*s).status = FINISH_STATE;
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(4 as libc::c_int)) as usize];
        deflateEnd(strm);
        return Z_MEM_ERROR;
    }
    (*s).d_buf = overlay.offset(
        ((*s).lit_bufsize as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<ush>() as libc::c_ulong) as isize,
    );
    (*s).l_buf = (*s).pending_buf.offset(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<ush>() as libc::c_ulong)
            .wrapping_mul((*s).lit_bufsize as libc::c_ulong) as isize,
    );
    (*s).level = level;
    (*s).strategy = strategy;
    (*s).method = method as Byte;
    return deflateReset(strm);
}
/* Compression function. Returns the block state after the call. */
/* =========================================================================
 * Check for a valid deflate stream state. Return 0 if ok, 1 if not.
 */
unsafe extern "C" fn deflateStateCheck(mut strm: z_streamp) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    if strm.is_null() || (*strm).zalloc.is_none() || (*strm).zfree.is_none() {
        return 1 as libc::c_int;
    }
    s = (*strm).state;
    if s.is_null()
        || (*s).strm != strm
        || (*s).status != INIT_STATE
            && (*s).status != GZIP_STATE
            && (*s).status != EXTRA_STATE
            && (*s).status != NAME_STATE
            && (*s).status != COMMENT_STATE
            && (*s).status != HCRC_STATE
            && (*s).status != BUSY_STATE
            && (*s).status != FINISH_STATE
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut str: uInt = 0;
    let mut n: uInt = 0;
    let mut wrap: libc::c_int = 0;
    let mut avail: libc::c_uint = 0;
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if deflateStateCheck(strm) != 0 || dictionary.is_null() {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state;
    wrap = (*s).wrap;
    if wrap == 2 as libc::c_int
        || wrap == 1 as libc::c_int && (*s).status != INIT_STATE
        || (*s).lookahead != 0
    {
        return Z_STREAM_ERROR;
    }
    /* when using zlib wrappers, compute Adler-32 for provided dictionary */
    if wrap == 1 as libc::c_int {
        (*strm).adler = adler32((*strm).adler, dictionary, dictLength)
    } /* avoid computing Adler-32 in read_buf */
    (*s).wrap = 0 as libc::c_int;
    /* if dictionary would fill window, just replace the history */
    if dictLength >= (*s).w_size {
        if wrap == 0 as libc::c_int {
            /* already empty otherwise */
            *(*s).head.offset(
                (*s).hash_size
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = NIL as Posf; /* use the tail */
            memset(
                (*s).head as *mut Bytef as *mut libc::c_void,
                0 as libc::c_int,
                ((*s)
                    .hash_size
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Posf>() as libc::c_ulong),
            );
            (*s).strstart = 0 as libc::c_int as uInt;
            (*s).block_start = 0 as libc::c_long;
            (*s).insert = 0 as libc::c_int as uInt
        }
        dictionary = dictionary.offset(dictLength.wrapping_sub((*s).w_size) as isize);
        dictLength = (*s).w_size
    }
    /* insert dictionary into window and hash */
    avail = (*strm).avail_in;
    next = (*strm).next_in;
    (*strm).avail_in = dictLength;
    (*strm).next_in = dictionary as *mut Bytef;
    fill_window(s);
    while (*s).lookahead >= MIN_MATCH as libc::c_uint {
        str = (*s).strstart;
        n = (*s)
            .lookahead
            .wrapping_sub((MIN_MATCH - 1 as libc::c_int) as libc::c_uint);
        loop {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset(
                    str.wrapping_add(3 as libc::c_int as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as isize,
                ) as libc::c_uint)
                & (*s).hash_mask;
            *(*s).prev.offset((str & (*s).w_mask) as isize) =
                *(*s).head.offset((*s).ins_h as isize);
            *(*s).head.offset((*s).ins_h as isize) = str as Pos;
            str = str.wrapping_add(1);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        (*s).strstart = str;
        (*s).lookahead = (MIN_MATCH - 1 as libc::c_int) as uInt;
        fill_window(s);
    }
    (*s).strstart = ((*s).strstart as libc::c_uint).wrapping_add((*s).lookahead) as uInt as uInt;
    (*s).block_start = (*s).strstart as libc::c_long;
    (*s).insert = (*s).lookahead;
    (*s).lookahead = 0 as libc::c_int as uInt;
    (*s).prev_length = (MIN_MATCH - 1 as libc::c_int) as uInt;
    (*s).match_length = (*s).prev_length;
    (*s).match_available = 0 as libc::c_int;
    (*strm).next_in = next;
    (*strm).avail_in = avail;
    (*s).wrap = wrap;
    return Z_OK;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateGetDictionary(
    mut strm: z_streamp,
    mut dictionary: *mut Bytef,
    mut dictLength: *mut uInt,
) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut len: uInt = 0;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state;
    len = (*s).strstart.wrapping_add((*s).lookahead);
    if len > (*s).w_size {
        len = (*s).w_size
    }
    if !dictionary.is_null() && len != 0 {
        memcpy(
            dictionary as *mut libc::c_void,
            (*s).window
                .offset((*s).strstart as isize)
                .offset((*s).lookahead as isize)
                .offset(-(len as isize)) as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
    if !dictLength.is_null() {
        *dictLength = len
    }
    return Z_OK;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateResetKeep(mut strm: z_streamp) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state; /* use zfree if we ever allocate msg dynamically */
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    (*strm).total_out = 0 as libc::c_int as uLong;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = Z_NULL as *mut libc::c_char;
    (*strm).data_type = Z_UNKNOWN;
    s = (*strm).state as *mut deflate_state;
    (*s).pending = 0 as libc::c_int as ulg;
    (*s).pending_out = (*s).pending_buf;
    if (*s).wrap < 0 as libc::c_int {
        (*s).wrap = -(*s).wrap
        /* was made negative by deflate(..., Z_FINISH); */
    }
    (*s).status = if (*s).wrap == 2 as libc::c_int {
        GZIP_STATE
    } else if (*s).wrap != 0 {
        INIT_STATE
    } else {
        BUSY_STATE
    };
    (*strm).adler = if (*s).wrap == 2 as libc::c_int {
        crc32(
            0 as libc::c_long as uLong,
            Z_NULL as *const Bytef,
            0 as libc::c_int as uInt,
        )
    } else {
        adler32(
            0 as libc::c_long as uLong,
            Z_NULL as *const Bytef,
            0 as libc::c_int as uInt,
        )
    };
    (*s).last_flush = Z_NO_FLUSH;
    _tr_init(s);
    return Z_OK;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateReset(mut strm: z_streamp) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = deflateResetKeep(strm);
    if ret == Z_OK {
        lm_init((*strm).state);
    }
    return ret;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateSetHeader(
    mut strm: z_streamp,
    mut head: gz_headerp,
) -> libc::c_int {
    if deflateStateCheck(strm) != 0 || (*(*strm).state).wrap != 2 as libc::c_int {
        return Z_STREAM_ERROR;
    }
    (*(*strm).state).gzhead = head;
    return Z_OK;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflatePending(
    mut strm: z_streamp,
    mut pending: *mut libc::c_uint,
    mut bits: *mut libc::c_int,
) -> libc::c_int {
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    if !pending.is_null() {
        *pending = (*(*strm).state).pending as libc::c_uint
    }
    if !bits.is_null() {
        *bits = (*(*strm).state).bi_valid
    }
    return Z_OK;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflatePrime(
    mut strm: z_streamp,
    mut bits: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut put: libc::c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state;
    if ((*s).d_buf as *mut Bytef)
        < (*s)
            .pending_out
            .offset((Buf_size + 7 as libc::c_int >> 3 as libc::c_int) as isize)
    {
        return Z_BUF_ERROR;
    }
    loop {
        put = Buf_size - (*s).bi_valid;
        if put > bits {
            put = bits
        }
        (*s).bi_buf = ((*s).bi_buf as libc::c_int
            | ((value & ((1 as libc::c_int) << put) - 1 as libc::c_int) << (*s).bi_valid) as ush
                as libc::c_int) as ush;
        (*s).bi_valid += put;
        _tr_flush_bits(s);
        value >>= put;
        bits -= put;
        if !(bits != 0) {
            break;
        }
    }
    return Z_OK;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateParams(
    mut strm: z_streamp,
    mut level: libc::c_int,
    mut strategy: libc::c_int,
) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut func: compress_func = None;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state;
    if level == Z_DEFAULT_COMPRESSION {
        level = 6 as libc::c_int
    }
    if level < 0 as libc::c_int
        || level > 9 as libc::c_int
        || strategy < 0 as libc::c_int
        || strategy > Z_FIXED
    {
        return Z_STREAM_ERROR;
    }
    func = configuration_table[(*s).level as usize].func;
    if (strategy != (*s).strategy || func != configuration_table[level as usize].func)
        && (*s).high_water != 0
    {
        /* Flush the last buffer: */
        let mut err: libc::c_int = deflate(strm, Z_BLOCK);
        if err == Z_STREAM_ERROR {
            return err;
        }
        if (*strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return Z_BUF_ERROR;
        }
    }
    if (*s).level != level {
        if (*s).level == 0 as libc::c_int && (*s).matches != 0 as libc::c_int as libc::c_uint {
            if (*s).matches == 1 as libc::c_int as libc::c_uint {
                slide_hash(s);
            } else {
                *(*s).head.offset(
                    (*s).hash_size
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        as isize,
                ) = NIL as Posf
            }
            memset(
                (*s).head as *mut Bytef as *mut libc::c_void,
                0 as libc::c_int,
                ((*s)
                    .hash_size
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Posf>() as libc::c_ulong),
            );
            (*s).matches = 0 as libc::c_int as uInt
        }
        (*s).level = level;
        (*s).max_lazy_match = configuration_table[level as usize].max_lazy as uInt;
        (*s).good_match = configuration_table[level as usize].good_length as uInt;
        (*s).nice_match = configuration_table[level as usize].nice_length as libc::c_int;
        (*s).max_chain_length = configuration_table[level as usize].max_chain as uInt
    }
    (*s).strategy = strategy;
    return Z_OK;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateTune(
    mut strm: z_streamp,
    mut good_length: libc::c_int,
    mut max_lazy: libc::c_int,
    mut nice_length: libc::c_int,
    mut max_chain: libc::c_int,
) -> libc::c_int {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state;
    (*s).good_match = good_length as uInt;
    (*s).max_lazy_match = max_lazy as uInt;
    (*s).nice_match = nice_length;
    (*s).max_chain_length = max_chain as uInt;
    return Z_OK;
}
/* =========================================================================
 * For the default windowBits of 15 and memLevel of 8, this function returns
 * a close to exact, as well as small, upper bound on the compressed size.
 * They are coded as constants here for a reason--if the #define's are
 * changed, then this function needs to be changed as well.  The return
 * value for 15 and 8 only works for those exact settings.
 *
 * For any setting other than those defaults for windowBits and memLevel,
 * the value returned is a conservative worst case for the maximum expansion
 * resulting from using fixed blocks instead of stored blocks, which deflate
 * can emit on compressed data for some combinations of the parameters.
 *
 * This function could be more sophisticated to provide closer upper bounds for
 * every combination of windowBits and memLevel.  But even the conservative
 * upper bound of about 14% expansion does not seem onerous for output buffer
 * allocation.
 */
#[no_mangle]
pub unsafe extern "C" fn deflateBound(mut strm: z_streamp, mut sourceLen: uLong) -> uLong {
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    let mut complen: uLong = 0;
    let mut wraplen: uLong = 0;
    /* conservative upper bound for compressed data */
    complen = sourceLen
        .wrapping_add(sourceLen.wrapping_add(7 as libc::c_int as libc::c_ulong) >> 3 as libc::c_int)
        .wrapping_add(
            sourceLen.wrapping_add(63 as libc::c_int as libc::c_ulong) >> 6 as libc::c_int,
        )
        .wrapping_add(5 as libc::c_int as libc::c_ulong);
    /* if can't get parameters, return conservative bound plus zlib wrapper */
    if deflateStateCheck(strm) != 0 {
        return complen.wrapping_add(6 as libc::c_int as libc::c_ulong);
    }
    /* compute wrapper length */
    s = (*strm).state;
    match (*s).wrap {
        0 => {
            /* raw deflate */
            wraplen = 0 as libc::c_int as uLong
        }
        1 => {
            /* zlib wrapper */
            wraplen = (6 as libc::c_int
                + (if (*s).strstart != 0 {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as uLong
        }
        2 => {
            /* gzip wrapper */
            wraplen = 18 as libc::c_int as uLong;
            if !(*s).gzhead.is_null() {
                /* user-supplied gzip header */
                let mut str: *mut Bytef = 0 as *mut Bytef;
                if !(*(*s).gzhead).extra.is_null() {
                    wraplen = (wraplen as libc::c_ulong).wrapping_add(
                        (2 as libc::c_int as libc::c_uint).wrapping_add((*(*s).gzhead).extra_len)
                            as libc::c_ulong,
                    ) as uLong as uLong
                }
                str = (*(*s).gzhead).name;
                if !str.is_null() {
                    loop {
                        wraplen = wraplen.wrapping_add(1);
                        let fresh0 = str;
                        str = str.offset(1);
                        if !(*fresh0 != 0) {
                            break;
                        }
                    }
                }
                str = (*(*s).gzhead).comment;
                if !str.is_null() {
                    loop {
                        wraplen = wraplen.wrapping_add(1);
                        let fresh1 = str;
                        str = str.offset(1);
                        if !(*fresh1 != 0) {
                            break;
                        }
                    }
                }
                if (*(*s).gzhead).hcrc != 0 {
                    wraplen = (wraplen as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        as uLong as uLong
                }
            }
        }
        _ => {
            /* for compiler happiness */
            wraplen = 6 as libc::c_int as uLong
        }
    }
    /* if not default parameters, return conservative bound */
    if (*s).w_bits != 15 as libc::c_int as libc::c_uint
        || (*s).hash_bits != (8 as libc::c_int + 7 as libc::c_int) as libc::c_uint
    {
        return complen.wrapping_add(wraplen);
    }
    /* default settings: return tight bound for that case */
    return sourceLen
        .wrapping_add(sourceLen >> 12 as libc::c_int)
        .wrapping_add(sourceLen >> 14 as libc::c_int)
        .wrapping_add(sourceLen >> 25 as libc::c_int)
        .wrapping_add(13 as libc::c_int as libc::c_ulong)
        .wrapping_sub(6 as libc::c_int as libc::c_ulong)
        .wrapping_add(wraplen);
}
/* =========================================================================
 * Put a short in the pending buffer. The 16-bit value is put in MSB order.
 * IN assertion: the stream state is correct and there is enough room in
 * pending_buf.
 */
unsafe extern "C" fn putShortMSB(mut s: *mut deflate_state, mut b: uInt) {
    let fresh2 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh2 as isize) = (b >> 8 as libc::c_int) as Byte;
    let fresh3 = (*s).pending;
    (*s).pending = (*s).pending.wrapping_add(1);
    *(*s).pending_buf.offset(fresh3 as isize) = (b & 0xff as libc::c_int as libc::c_uint) as Byte;
}
/* =========================================================================
 * Flush as much pending output as possible. All deflate() output, except for
 * some deflate_stored() output, goes through this function so some
 * applications may wish to modify it to avoid allocating a large
 * strm->next_out buffer and copying into it. (See also read_buf()).
 */
unsafe extern "C" fn flush_pending(mut strm: z_streamp) {
    let mut len: libc::c_uint = 0;
    let mut s: *mut deflate_state = (*strm).state;
    _tr_flush_bits(s);
    len = (*s).pending as libc::c_uint;
    if len > (*strm).avail_out {
        len = (*strm).avail_out
    }
    if len == 0 as libc::c_int as libc::c_uint {
        return;
    }
    memcpy(
        (*strm).next_out as *mut libc::c_void,
        (*s).pending_out as *const libc::c_void,
        len as libc::c_ulong,
    );
    (*strm).next_out = (*strm).next_out.offset(len as isize);
    (*s).pending_out = (*s).pending_out.offset(len as isize);
    (*strm).total_out =
        ((*strm).total_out as libc::c_ulong).wrapping_add(len as libc::c_ulong) as uLong as uLong;
    (*strm).avail_out = ((*strm).avail_out as libc::c_uint).wrapping_sub(len) as uInt as uInt;
    (*s).pending = ((*s).pending as libc::c_ulong).wrapping_sub(len as libc::c_ulong) as ulg as ulg;
    if (*s).pending == 0 as libc::c_int as libc::c_ulong {
        (*s).pending_out = (*s).pending_buf
    };
}
/* ===========================================================================
 * Update the header CRC with the bytes s->pending_buf[beg..s->pending - 1].
 */
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflate(mut strm: z_streamp, mut flush: libc::c_int) -> libc::c_int {
    let mut old_flush: libc::c_int = 0; /* value of flush param for previous deflate call */
    let mut s: *mut deflate_state = 0 as *mut deflate_state;
    if deflateStateCheck(strm) != 0 || flush > Z_BLOCK || flush < 0 as libc::c_int {
        return Z_STREAM_ERROR;
    }
    s = (*strm).state;
    if (*strm).next_out.is_null()
        || (*strm).avail_in != 0 as libc::c_int as libc::c_uint && (*strm).next_in.is_null()
        || (*s).status == FINISH_STATE && flush != Z_FINISH
    {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(2 as libc::c_int)) as usize];
        return -(2 as libc::c_int);
    }
    if (*strm).avail_out == 0 as libc::c_int as libc::c_uint {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(5 as libc::c_int)) as usize];
        return -(5 as libc::c_int);
    }
    old_flush = (*s).last_flush;
    (*s).last_flush = flush;
    /* Flush as much pending output as possible */
    if (*s).pending != 0 as libc::c_int as libc::c_ulong {
        flush_pending(strm);
        if (*strm).avail_out == 0 as libc::c_int as libc::c_uint {
            /* Make sure there is something to do and avoid duplicate consecutive
             * flushes. For repeated and useless calls with Z_FINISH, we keep
             * returning Z_STREAM_END instead of Z_BUF_ERROR.
             */
            /* Since avail_out is 0, deflate will be called again with
             * more output space, but possibly with both pending and
             * avail_in equal to zero. There won't be anything to do,
             * but this is not an error situation so make sure we
             * return OK instead of BUF_ERROR at next call of deflate:
             */
            (*s).last_flush = -(1 as libc::c_int);
            return Z_OK;
        }
    } else if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
        && flush * 2 as libc::c_int
            - (if flush > 4 as libc::c_int {
                9 as libc::c_int
            } else {
                0 as libc::c_int
            })
            <= old_flush * 2 as libc::c_int
                - (if old_flush > 4 as libc::c_int {
                    9 as libc::c_int
                } else {
                    0 as libc::c_int
                })
        && flush != Z_FINISH
    {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(5 as libc::c_int)) as usize];
        return -(5 as libc::c_int);
    }
    /* User must not provide more input after the first FINISH: */
    if (*s).status == FINISH_STATE && (*strm).avail_in != 0 as libc::c_int as libc::c_uint {
        (*strm).msg = z_errmsg[(Z_NEED_DICT - -(5 as libc::c_int)) as usize];
        return -(5 as libc::c_int);
    }
    /* Write the header */
    if (*s).status == INIT_STATE {
        /* zlib header */
        let mut header: uInt = (Z_DEFLATED as libc::c_uint).wrapping_add(
            (*s).w_bits.wrapping_sub(8 as libc::c_int as libc::c_uint) << 4 as libc::c_int,
        ) << 8 as libc::c_int;
        let mut level_flags: uInt = 0;
        if (*s).strategy >= Z_HUFFMAN_ONLY || (*s).level < 2 as libc::c_int {
            level_flags = 0 as libc::c_int as uInt
        } else if (*s).level < 6 as libc::c_int {
            level_flags = 1 as libc::c_int as uInt
        } else if (*s).level == 6 as libc::c_int {
            level_flags = 2 as libc::c_int as uInt
        } else {
            level_flags = 3 as libc::c_int as uInt
        }
        header |= level_flags << 6 as libc::c_int;
        if (*s).strstart != 0 as libc::c_int as libc::c_uint {
            header |= PRESET_DICT as libc::c_uint
        }
        header = (header as libc::c_uint).wrapping_add(
            (31 as libc::c_int as libc::c_uint)
                .wrapping_sub(header.wrapping_rem(31 as libc::c_int as libc::c_uint)),
        ) as uInt as uInt;
        putShortMSB(s, header);
        /* Save the adler32 of the preset dictionary: */
        if (*s).strstart != 0 as libc::c_int as libc::c_uint {
            putShortMSB(s, ((*strm).adler >> 16 as libc::c_int) as uInt);
            putShortMSB(
                s,
                ((*strm).adler & 0xffff as libc::c_int as libc::c_ulong) as uInt,
            );
        }
        (*strm).adler = adler32(
            0 as libc::c_long as uLong,
            Z_NULL as *const Bytef,
            0 as libc::c_int as uInt,
        );
        (*s).status = BUSY_STATE;
        /* Compression must start with an empty pending buffer */
        flush_pending(strm);
        if (*s).pending != 0 as libc::c_int as libc::c_ulong {
            (*s).last_flush = -(1 as libc::c_int);
            return Z_OK;
        }
    }
    if (*s).status == GZIP_STATE {
        /* gzip header */
        (*strm).adler = crc32(
            0 as libc::c_long as uLong,
            Z_NULL as *const Bytef,
            0 as libc::c_int as uInt,
        );
        let fresh4 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh4 as isize) = 31 as libc::c_int as Bytef;
        let fresh5 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh5 as isize) = 139 as libc::c_int as Bytef;
        let fresh6 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh6 as isize) = 8 as libc::c_int as Bytef;
        if (*s).gzhead.is_null() {
            let fresh7 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh7 as isize) = 0 as libc::c_int as Bytef;
            let fresh8 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh8 as isize) = 0 as libc::c_int as Bytef;
            let fresh9 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh9 as isize) = 0 as libc::c_int as Bytef;
            let fresh10 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh10 as isize) = 0 as libc::c_int as Bytef;
            let fresh11 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh11 as isize) = 0 as libc::c_int as Bytef;
            let fresh12 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh12 as isize) = if (*s).level == 9 as libc::c_int {
                2 as libc::c_int
            } else if (*s).strategy >= 2 as libc::c_int || (*s).level < 2 as libc::c_int {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            } as Bytef;
            let fresh13 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh13 as isize) = 3 as libc::c_int as Bytef;
            (*s).status = BUSY_STATE;
            /* Compression must start with an empty pending buffer */
            flush_pending(strm); /* start of bytes to update crc */
            if (*s).pending != 0 as libc::c_int as libc::c_ulong {
                (*s).last_flush = -(1 as libc::c_int); /* start of bytes to update crc */
                return Z_OK;
            }
        } else {
            let fresh14 = (*s).pending; /* start of bytes to update crc */
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh14 as isize) = ((if (*(*s).gzhead).text != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) + (if (*(*s).gzhead).hcrc != 0 {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            }) + (if (*(*s).gzhead).extra.is_null() {
                0 as libc::c_int
            } else {
                4 as libc::c_int
            }) + (if (*(*s).gzhead).name.is_null() {
                0 as libc::c_int
            } else {
                8 as libc::c_int
            }) + (if (*(*s).gzhead).comment.is_null()
            {
                0 as libc::c_int
            } else {
                16 as libc::c_int
            })) as Bytef;
            let fresh15 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh15 as isize) =
                ((*(*s).gzhead).time & 0xff as libc::c_int as libc::c_ulong) as Byte;
            let fresh16 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh16 as isize) = ((*(*s).gzhead).time >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong)
                as Byte;
            let fresh17 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh17 as isize) = ((*(*s).gzhead).time >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong)
                as Byte;
            let fresh18 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh18 as isize) = ((*(*s).gzhead).time >> 24 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong)
                as Byte;
            let fresh19 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh19 as isize) = if (*s).level == 9 as libc::c_int {
                2 as libc::c_int
            } else if (*s).strategy >= 2 as libc::c_int || (*s).level < 2 as libc::c_int {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            } as Bytef;
            let fresh20 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh20 as isize) =
                ((*(*s).gzhead).os & 0xff as libc::c_int) as Bytef;
            if !(*(*s).gzhead).extra.is_null() {
                let fresh21 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(fresh21 as isize) =
                    ((*(*s).gzhead).extra_len & 0xff as libc::c_int as libc::c_uint) as Bytef;
                let fresh22 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(fresh22 as isize) =
                    ((*(*s).gzhead).extra_len >> 8 as libc::c_int
                        & 0xff as libc::c_int as libc::c_uint) as Bytef
            }
            if (*(*s).gzhead).hcrc != 0 {
                (*strm).adler = crc32((*strm).adler, (*s).pending_buf, (*s).pending as uInt)
            }
            (*s).gzindex = 0 as libc::c_int as ulg;
            (*s).status = EXTRA_STATE
        }
    }
    if (*s).status == EXTRA_STATE {
        if !(*(*s).gzhead).extra.is_null() {
            let mut beg: ulg = (*s).pending;
            let mut left: uInt = (((*(*s).gzhead).extra_len & 0xffff as libc::c_int as libc::c_uint)
                as libc::c_ulong)
                .wrapping_sub((*s).gzindex) as uInt;
            while (*s).pending.wrapping_add(left as libc::c_ulong) > (*s).pending_buf_size {
                let mut copy: uInt = (*s).pending_buf_size.wrapping_sub((*s).pending) as uInt;
                memcpy(
                    (*s).pending_buf.offset((*s).pending as isize) as *mut libc::c_void,
                    (*(*s).gzhead).extra.offset((*s).gzindex as isize) as *const libc::c_void,
                    copy as libc::c_ulong,
                );
                (*s).pending = (*s).pending_buf_size;
                if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg {
                    (*strm).adler = crc32(
                        (*strm).adler,
                        (*s).pending_buf.offset(beg as isize),
                        (*s).pending.wrapping_sub(beg) as uInt,
                    )
                }
                (*s).gzindex = ((*s).gzindex as libc::c_ulong).wrapping_add(copy as libc::c_ulong)
                    as ulg as ulg;
                flush_pending(strm);
                if (*s).pending != 0 as libc::c_int as libc::c_ulong {
                    (*s).last_flush = -(1 as libc::c_int);
                    return Z_OK;
                }
                beg = 0 as libc::c_int as ulg;
                left = (left as libc::c_uint).wrapping_sub(copy) as uInt as uInt
            }
            memcpy(
                (*s).pending_buf.offset((*s).pending as isize) as *mut libc::c_void,
                (*(*s).gzhead).extra.offset((*s).gzindex as isize) as *const libc::c_void,
                left as libc::c_ulong,
            );
            (*s).pending =
                ((*s).pending as libc::c_ulong).wrapping_add(left as libc::c_ulong) as ulg as ulg;
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg {
                (*strm).adler = crc32(
                    (*strm).adler,
                    (*s).pending_buf.offset(beg as isize),
                    (*s).pending.wrapping_sub(beg) as uInt,
                )
            }
            (*s).gzindex = 0 as libc::c_int as ulg
        }
        (*s).status = NAME_STATE
    }
    if (*s).status == NAME_STATE {
        if !(*(*s).gzhead).name.is_null() {
            let mut beg_0: ulg = (*s).pending;
            let mut val: libc::c_int = 0;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_0 {
                        (*strm).adler = crc32(
                            (*strm).adler,
                            (*s).pending_buf.offset(beg_0 as isize),
                            (*s).pending.wrapping_sub(beg_0) as uInt,
                        )
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as libc::c_int as libc::c_ulong {
                        (*s).last_flush = -(1 as libc::c_int);
                        return Z_OK;
                    }
                    beg_0 = 0 as libc::c_int as ulg
                }
                let fresh23 = (*s).gzindex;
                (*s).gzindex = (*s).gzindex.wrapping_add(1);
                val = *(*(*s).gzhead).name.offset(fresh23 as isize) as libc::c_int;
                let fresh24 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(fresh24 as isize) = val as Bytef;
                if !(val != 0 as libc::c_int) {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_0 {
                (*strm).adler = crc32(
                    (*strm).adler,
                    (*s).pending_buf.offset(beg_0 as isize),
                    (*s).pending.wrapping_sub(beg_0) as uInt,
                )
            }
            (*s).gzindex = 0 as libc::c_int as ulg
        }
        (*s).status = COMMENT_STATE
    }
    if (*s).status == COMMENT_STATE {
        if !(*(*s).gzhead).comment.is_null() {
            let mut beg_1: ulg = (*s).pending;
            let mut val_0: libc::c_int = 0;
            loop {
                if (*s).pending == (*s).pending_buf_size {
                    if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_1 {
                        (*strm).adler = crc32(
                            (*strm).adler,
                            (*s).pending_buf.offset(beg_1 as isize),
                            (*s).pending.wrapping_sub(beg_1) as uInt,
                        )
                    }
                    flush_pending(strm);
                    if (*s).pending != 0 as libc::c_int as libc::c_ulong {
                        (*s).last_flush = -(1 as libc::c_int);
                        return Z_OK;
                    }
                    beg_1 = 0 as libc::c_int as ulg
                }
                let fresh25 = (*s).gzindex;
                (*s).gzindex = (*s).gzindex.wrapping_add(1);
                val_0 = *(*(*s).gzhead).comment.offset(fresh25 as isize) as libc::c_int;
                let fresh26 = (*s).pending;
                (*s).pending = (*s).pending.wrapping_add(1);
                *(*s).pending_buf.offset(fresh26 as isize) = val_0 as Bytef;
                if !(val_0 != 0 as libc::c_int) {
                    break;
                }
            }
            if (*(*s).gzhead).hcrc != 0 && (*s).pending > beg_1 {
                (*strm).adler = crc32(
                    (*strm).adler,
                    (*s).pending_buf.offset(beg_1 as isize),
                    (*s).pending.wrapping_sub(beg_1) as uInt,
                )
            }
        }
        (*s).status = HCRC_STATE
    }
    if (*s).status == HCRC_STATE {
        if (*(*s).gzhead).hcrc != 0 {
            if (*s).pending.wrapping_add(2 as libc::c_int as libc::c_ulong) > (*s).pending_buf_size
            {
                flush_pending(strm);
                if (*s).pending != 0 as libc::c_int as libc::c_ulong {
                    (*s).last_flush = -(1 as libc::c_int);
                    return Z_OK;
                }
            }
            let fresh27 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh27 as isize) =
                ((*strm).adler & 0xff as libc::c_int as libc::c_ulong) as Byte;
            let fresh28 = (*s).pending;
            (*s).pending = (*s).pending.wrapping_add(1);
            *(*s).pending_buf.offset(fresh28 as isize) =
                ((*strm).adler >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
            (*strm).adler = crc32(
                0 as libc::c_long as uLong,
                Z_NULL as *const Bytef,
                0 as libc::c_int as uInt,
            )
        }
        (*s).status = BUSY_STATE;
        /* Compression must start with an empty pending buffer */
        flush_pending(strm);
        if (*s).pending != 0 as libc::c_int as libc::c_ulong {
            (*s).last_flush = -(1 as libc::c_int);
            return Z_OK;
        }
    }
    /* Start a new block or continue the current one.
     */
    if (*strm).avail_in != 0 as libc::c_int as libc::c_uint
        || (*s).lookahead != 0 as libc::c_int as libc::c_uint
        || flush != Z_NO_FLUSH && (*s).status != FINISH_STATE
    {
        let mut bstate: block_state = need_more;
        bstate = if (*s).level == 0 as libc::c_int {
            deflate_stored(s, flush) as libc::c_uint
        } else if (*s).strategy == Z_HUFFMAN_ONLY {
            deflate_huff(s, flush) as libc::c_uint
        } else if (*s).strategy == Z_RLE {
            deflate_rle(s, flush) as libc::c_uint
        } else {
            Some(
                (*configuration_table.as_ptr().offset((*s).level as isize))
                    .func
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(s, flush) as libc::c_uint
        } as block_state;
        if bstate as libc::c_uint == finish_started as libc::c_int as libc::c_uint
            || bstate as libc::c_uint == finish_done as libc::c_int as libc::c_uint
        {
            (*s).status = FINISH_STATE
        }
        if bstate as libc::c_uint == need_more as libc::c_int as libc::c_uint
            || bstate as libc::c_uint == finish_started as libc::c_int as libc::c_uint
        {
            if (*strm).avail_out == 0 as libc::c_int as libc::c_uint {
                (*s).last_flush = -(1 as libc::c_int)
                /* avoid BUF_ERROR next call, see above */
            }
            return Z_OK;
            /* If flush != Z_NO_FLUSH && avail_out == 0, the next call
             * of deflate should use the same flush parameter to make sure
             * that the flush is complete. So we don't have to output an
             * empty block here, this will be done at next call. This also
             * ensures that for a very small output buffer, we emit at most
             * one empty block.
             */
        }
        if bstate as libc::c_uint == block_done as libc::c_int as libc::c_uint {
            if flush == Z_PARTIAL_FLUSH {
                _tr_align(s);
            } else if flush != Z_BLOCK {
                /* FULL_FLUSH or SYNC_FLUSH */
                _tr_stored_block(
                    s,
                    0 as *mut libc::c_char,
                    0 as libc::c_long as ulg,
                    0 as libc::c_int,
                );
                /* For a full flush, this empty block will be recognized
                 * as a special marker by inflate_sync().
                 */
                if flush == Z_FULL_FLUSH {
                    *(*s).head.offset(
                        (*s).hash_size
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as isize,
                    ) = NIL as Posf;
                    memset(
                        (*s).head as *mut Bytef as *mut libc::c_void,
                        0 as libc::c_int,
                        ((*s)
                            .hash_size
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<Posf>() as libc::c_ulong),
                    );
                    /* forget history */
                    if (*s).lookahead == 0 as libc::c_int as libc::c_uint {
                        (*s).strstart = 0 as libc::c_int as uInt; /* avoid BUF_ERROR at next call, see above */
                        (*s).block_start = 0 as libc::c_long;
                        (*s).insert = 0 as libc::c_int as uInt
                    }
                }
            }
            flush_pending(strm);
            if (*strm).avail_out == 0 as libc::c_int as libc::c_uint {
                (*s).last_flush = -(1 as libc::c_int);
                return Z_OK;
            }
        }
    }
    if flush != Z_FINISH {
        return Z_OK;
    }
    if (*s).wrap <= 0 as libc::c_int {
        return Z_STREAM_END;
    }
    /* Write the trailer */
    if (*s).wrap == 2 as libc::c_int {
        let fresh29 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh29 as isize) =
            ((*strm).adler & 0xff as libc::c_int as libc::c_ulong) as Byte;
        let fresh30 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh30 as isize) =
            ((*strm).adler >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
        let fresh31 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh31 as isize) =
            ((*strm).adler >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
        let fresh32 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh32 as isize) =
            ((*strm).adler >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
        let fresh33 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh33 as isize) =
            ((*strm).total_in & 0xff as libc::c_int as libc::c_ulong) as Byte;
        let fresh34 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh34 as isize) =
            ((*strm).total_in >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
        let fresh35 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh35 as isize) =
            ((*strm).total_in >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte;
        let fresh36 = (*s).pending;
        (*s).pending = (*s).pending.wrapping_add(1);
        *(*s).pending_buf.offset(fresh36 as isize) =
            ((*strm).total_in >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as Byte
    } else {
        putShortMSB(s, ((*strm).adler >> 16 as libc::c_int) as uInt);
        putShortMSB(
            s,
            ((*strm).adler & 0xffff as libc::c_int as libc::c_ulong) as uInt,
        );
    }
    flush_pending(strm);
    /* If avail_out is zero, the application will call deflate again
     * to flush the rest.
     */
    if (*s).wrap > 0 as libc::c_int {
        (*s).wrap = -(*s).wrap
    } /* write the trailer only once! */
    return if (*s).pending != 0 as libc::c_int as libc::c_ulong {
        Z_OK
    } else {
        Z_STREAM_END
    };
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn deflateEnd(mut strm: z_streamp) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if deflateStateCheck(strm) != 0 {
        return Z_STREAM_ERROR;
    }
    status = (*(*strm).state).status;
    /* Deallocate in reverse order of allocations: */
    if !(*(*strm).state).pending_buf.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).pending_buf as voidpf,
        );
    }
    if !(*(*strm).state).head.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).head as voidpf,
        );
    }
    if !(*(*strm).state).prev.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).prev as voidpf,
        );
    }
    if !(*(*strm).state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*(*strm).state).window as voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as voidpf,
    );
    (*strm).state = Z_NULL as *mut internal_state;
    return if status == BUSY_STATE {
        Z_DATA_ERROR
    } else {
        Z_OK
    };
}
/* =========================================================================
 * Copy the source state to the destination state.
 * To simplify the source, this is not supported for 16-bit MSDOS (which
 * doesn't have enough memory anyway to duplicate compression states).
 */
#[no_mangle]
pub unsafe extern "C" fn deflateCopy(mut dest: z_streamp, mut source: z_streamp) -> libc::c_int {
    let mut ds: *mut deflate_state = 0 as *mut deflate_state;
    let mut ss: *mut deflate_state = 0 as *mut deflate_state;
    let mut overlay: *mut ushf = 0 as *mut ushf;
    if deflateStateCheck(source) != 0 || dest.is_null() {
        return Z_STREAM_ERROR;
    }
    ss = (*source).state;
    memcpy(
        dest as voidpf,
        source as voidpf as *const libc::c_void,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong,
    );
    ds = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        1 as libc::c_int as uInt,
        ::std::mem::size_of::<deflate_state>() as libc::c_ulong as uInt,
    ) as *mut deflate_state;
    if ds.is_null() {
        return Z_MEM_ERROR;
    }
    (*dest).state = ds as *mut internal_state;
    memcpy(
        ds as voidpf,
        ss as voidpf as *const libc::c_void,
        ::std::mem::size_of::<deflate_state>() as libc::c_ulong,
    );
    (*ds).strm = dest;
    (*ds).window = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Byte>() as libc::c_ulong) as uInt,
    ) as *mut Bytef;
    (*ds).prev = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).w_size,
        ::std::mem::size_of::<Pos>() as libc::c_ulong as uInt,
    ) as *mut Posf;
    (*ds).head = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).hash_size,
        ::std::mem::size_of::<Pos>() as libc::c_ulong as uInt,
    ) as *mut Posf;
    overlay = Some((*dest).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*dest).opaque,
        (*ds).lit_bufsize,
        (::std::mem::size_of::<ush>() as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as uInt,
    ) as *mut ushf;
    (*ds).pending_buf = overlay as *mut uchf;
    if (*ds).window.is_null()
        || (*ds).prev.is_null()
        || (*ds).head.is_null()
        || (*ds).pending_buf.is_null()
    {
        deflateEnd(dest);
        return Z_MEM_ERROR;
    }
    /* following zmemcpy do not work for 16-bit MSDOS */
    memcpy(
        (*ds).window as *mut libc::c_void,
        (*ss).window as *const libc::c_void,
        ((*ds).w_size.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Byte>() as libc::c_ulong),
    );
    memcpy(
        (*ds).prev as voidpf,
        (*ss).prev as voidpf as *const libc::c_void,
        ((*ds).w_size as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Pos>() as libc::c_ulong),
    );
    memcpy(
        (*ds).head as voidpf,
        (*ss).head as voidpf as *const libc::c_void,
        ((*ds).hash_size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Pos>() as libc::c_ulong),
    );
    memcpy(
        (*ds).pending_buf as *mut libc::c_void,
        (*ss).pending_buf as *const libc::c_void,
        (*ds).pending_buf_size as uInt as libc::c_ulong,
    );
    (*ds).pending_out = (*ds)
        .pending_buf
        .offset((*ss).pending_out.offset_from((*ss).pending_buf) as libc::c_long as isize);
    (*ds).d_buf = overlay.offset(
        ((*ds).lit_bufsize as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<ush>() as libc::c_ulong) as isize,
    );
    (*ds).l_buf = (*ds).pending_buf.offset(
        (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<ush>() as libc::c_ulong)
            .wrapping_mul((*ds).lit_bufsize as libc::c_ulong) as isize,
    );
    (*ds).l_desc.dyn_tree = (*ds).dyn_ltree.as_mut_ptr();
    (*ds).d_desc.dyn_tree = (*ds).dyn_dtree.as_mut_ptr();
    (*ds).bl_desc.dyn_tree = (*ds).bl_tree.as_mut_ptr();
    return Z_OK;
    /* MAXSEG_64K */
}
/* ===========================================================================
 * Read a new buffer from the current input stream, update the adler32
 * and total number of bytes read.  All deflate() input goes through
 * this function so some applications may wish to modify it to avoid
 * allocating a large strm->next_in buffer and copying from it.
 * (See also flush_pending()).
 */
unsafe extern "C" fn read_buf(
    mut strm: z_streamp,
    mut buf: *mut Bytef,
    mut size: libc::c_uint,
) -> libc::c_uint {
    let mut len: libc::c_uint = (*strm).avail_in;
    if len > size {
        len = size
    }
    if len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    (*strm).avail_in = ((*strm).avail_in as libc::c_uint).wrapping_sub(len) as uInt as uInt;
    memcpy(
        buf as *mut libc::c_void,
        (*strm).next_in as *const libc::c_void,
        len as libc::c_ulong,
    );
    if (*(*strm).state).wrap == 1 as libc::c_int {
        (*strm).adler = adler32((*strm).adler, buf, len)
    } else if (*(*strm).state).wrap == 2 as libc::c_int {
        (*strm).adler = crc32((*strm).adler, buf, len)
    }
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in =
        ((*strm).total_in as libc::c_ulong).wrapping_add(len as libc::c_ulong) as uLong as uLong;
    return len;
}
/* ===========================================================================
 * Initialize the "longest match" routines for a new zlib stream
 */
unsafe extern "C" fn lm_init(mut s: *mut deflate_state) {
    (*s).window_size = (2 as libc::c_long as ulg).wrapping_mul((*s).w_size as libc::c_ulong);
    *(*s).head.offset(
        (*s).hash_size
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
    ) = NIL as Posf;
    memset(
        (*s).head as *mut Bytef as *mut libc::c_void,
        0 as libc::c_int,
        ((*s)
            .hash_size
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Posf>() as libc::c_ulong),
    );
    /* Set the default configuration parameters:
     */
    (*s).max_lazy_match = configuration_table[(*s).level as usize].max_lazy as uInt;
    (*s).good_match = configuration_table[(*s).level as usize].good_length as uInt;
    (*s).nice_match = configuration_table[(*s).level as usize].nice_length as libc::c_int;
    (*s).max_chain_length = configuration_table[(*s).level as usize].max_chain as uInt;
    (*s).strstart = 0 as libc::c_int as uInt;
    (*s).block_start = 0 as libc::c_long;
    (*s).lookahead = 0 as libc::c_int as uInt;
    (*s).insert = 0 as libc::c_int as uInt;
    (*s).prev_length = (MIN_MATCH - 1 as libc::c_int) as uInt;
    (*s).match_length = (*s).prev_length;
    (*s).match_available = 0 as libc::c_int;
    (*s).ins_h = 0 as libc::c_int as uInt;
}
/* ===========================================================================
 * Set match_start to the longest match starting at the given string and
 * return its length. Matches shorter or equal to prev_length are discarded,
 * in which case the result is equal to prev_length and match_start is
 * garbage.
 * IN assertions: cur_match is the head of the hash chain for the current
 *   string (strstart) and its distance is <= MAX_DIST, and prev_length >= 1
 * OUT assertion: the match length is not greater than s->lookahead.
 */
/* For 80x86 and 680x0, an optimized version will be provided in match.asm or
 * match.S. The code will be functionally equivalent.
 */
unsafe extern "C" fn longest_match(mut s: *mut deflate_state, mut cur_match: IPos) -> uInt
/* current match */ {
    let mut chain_length: libc::c_uint = (*s).max_chain_length; /* max hash chain length */
    let mut scan: *mut Bytef = (*s).window.offset((*s).strstart as isize); /* current string */
    let mut match_0: *mut Bytef = 0 as *mut Bytef; /* matched string */
    let mut len: libc::c_int = 0; /* length of current match */
    let mut best_len: libc::c_int = (*s).prev_length as libc::c_int; /* best match length so far */
    let mut nice_match: libc::c_int = (*s).nice_match; /* stop if match long enough */
    let mut limit: IPos = if (*s).strstart > (*s).w_size.wrapping_sub(MIN_LOOKAHEAD as libc::c_uint)
    {
        (*s).strstart
            .wrapping_sub((*s).w_size.wrapping_sub(MIN_LOOKAHEAD as libc::c_uint))
    } else {
        NIL as libc::c_uint
    };
    /* Stop when cur_match becomes <= limit. To simplify the code,
     * we prevent matches with the string of window index 0.
     */
    let mut prev: *mut Posf = (*s).prev;
    let mut wmask: uInt = (*s).w_mask;
    let mut strend: *mut Bytef = (*s)
        .window
        .offset((*s).strstart as isize)
        .offset(MAX_MATCH as isize);
    let mut scan_end1: Byte = *scan.offset((best_len - 1 as libc::c_int) as isize);
    let mut scan_end: Byte = *scan.offset(best_len as isize);
    /* The code is optimized for HASH_BITS >= 8 and MAX_MATCH-2 multiple of 16.
     * It is easy to get rid of this optimization if necessary.
     */
    /* Do not waste too much time if we already have a good match: */
    if (*s).prev_length >= (*s).good_match {
        chain_length >>= 2 as libc::c_int
    }
    /* Do not look for matches beyond the end of the input. This is necessary
     * to make deflate deterministic.
     */
    if nice_match as uInt > (*s).lookahead {
        nice_match = (*s).lookahead as libc::c_int
    }
    loop {
        match_0 = (*s).window.offset(cur_match as isize);
        /* Skip to next match if the match length cannot increase
         * or if the match length is less than 2.  Note that the checks below
         * for insufficient lookahead only occur occasionally for performance
         * reasons.  Therefore uninitialized memory will be accessed, and
         * conditional jumps will be made that depend on those values.
         * However the length of the match is limited to the lookahead, so
         * the output of deflate is not affected by the uninitialized values.
         */
        /* UNALIGNED_OK */
        if !(*match_0.offset(best_len as isize) as libc::c_int != scan_end as libc::c_int
            || *match_0.offset((best_len - 1 as libc::c_int) as isize) as libc::c_int
                != scan_end1 as libc::c_int
            || *match_0 as libc::c_int != *scan as libc::c_int
            || {
                match_0 = match_0.offset(1);
                (*match_0 as libc::c_int) != *scan.offset(1 as libc::c_int as isize) as libc::c_int
            })
        {
            /* The check at best_len-1 can be removed because it will be made
             * again later. (This heuristic is not always a win.)
             * It is not necessary to compare scan[2] and match[2] since they
             * are always equal when the other bytes match, given that
             * the hash keys are equal and that HASH_BITS >= 8.
             */
            scan = scan.offset(2 as libc::c_int as isize);
            match_0 = match_0.offset(1);
            loop
            /* We check for insufficient lookahead only every 8th comparison;
             * the 256th check will be made at strstart+258.
             */
            {
                scan = scan.offset(1);
                match_0 = match_0.offset(1);
                if !(*scan as libc::c_int == *match_0 as libc::c_int
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        (*scan as libc::c_int) == *match_0 as libc::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        (*scan as libc::c_int) == *match_0 as libc::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        (*scan as libc::c_int) == *match_0 as libc::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        (*scan as libc::c_int) == *match_0 as libc::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        (*scan as libc::c_int) == *match_0 as libc::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        (*scan as libc::c_int) == *match_0 as libc::c_int
                    }
                    && {
                        scan = scan.offset(1);
                        match_0 = match_0.offset(1);
                        (*scan as libc::c_int) == *match_0 as libc::c_int
                    }
                    && scan < strend)
                {
                    break;
                }
            }
            len = MAX_MATCH - strend.offset_from(scan) as libc::c_long as libc::c_int;
            scan = strend.offset(-(MAX_MATCH as isize));
            /* UNALIGNED_OK */
            if len > best_len {
                (*s).match_start = cur_match;
                best_len = len;
                if len >= nice_match {
                    break;
                }
                scan_end1 = *scan.offset((best_len - 1 as libc::c_int) as isize);
                scan_end = *scan.offset(best_len as isize)
            }
        }
        cur_match = *prev.offset((cur_match & wmask) as isize) as IPos;
        if !(cur_match > limit && {
            chain_length = chain_length.wrapping_sub(1);
            (chain_length) != 0 as libc::c_int as libc::c_uint
        }) {
            break;
        }
    }
    if best_len as uInt <= (*s).lookahead {
        return best_len as uInt;
    }
    return (*s).lookahead;
}
/* ASMV */
/* FASTEST */
/* FASTEST */
/* ZLIB_DEBUG */
/* ===========================================================================
 * Fill the window when the lookahead becomes insufficient.
 * Updates strstart and lookahead.
 *
 * IN assertion: lookahead < MIN_LOOKAHEAD
 * OUT assertions: strstart <= window_size-MIN_LOOKAHEAD
 *    At least one byte has been read, or avail_in == 0; reads are
 *    performed for at least two bytes (required for the zip translate_eol
 *    option -- not supported here).
 */
unsafe extern "C" fn fill_window(mut s: *mut deflate_state) {
    let mut n: libc::c_uint = 0; /* Amount of free space at the end of the window. */
    let mut more: libc::c_uint = 0;
    let mut wsize: uInt = (*s).w_size;
    loop {
        more = (*s)
            .window_size
            .wrapping_sub((*s).lookahead as ulg)
            .wrapping_sub((*s).strstart as ulg) as libc::c_uint;
        /* If the whole input has less than MIN_MATCH bytes, ins_h is garbage,
         * but this is not important since only literal bytes will be emitted.
         */
        /* Deal with !@#$% 64K limit: */
        if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            <= 2 as libc::c_int as libc::c_ulong
        {
            if more == 0 as libc::c_int as libc::c_uint
                && (*s).strstart == 0 as libc::c_int as libc::c_uint
                && (*s).lookahead == 0 as libc::c_int as libc::c_uint
            {
                more = wsize
            } else if more == -(1 as libc::c_int) as libc::c_uint {
                /* Very unlikely, but possible on 16 bit machine if
                 * strstart == 0 && lookahead == 1 (input done a byte at time)
                 */
                more = more.wrapping_sub(1)
            }
        }
        /* If the window is almost full and there is insufficient lookahead,
         * move the upper half to the lower one to make room in the upper half.
         */
        if (*s).strstart
            >= wsize.wrapping_add((*s).w_size.wrapping_sub(MIN_LOOKAHEAD as libc::c_uint))
        {
            memcpy(
                (*s).window as *mut libc::c_void,
                (*s).window.offset(wsize as isize) as *const libc::c_void,
                wsize.wrapping_sub(more) as libc::c_ulong,
            ); /* we now have strstart >= MAX_DIST */
            (*s).match_start =
                ((*s).match_start as libc::c_uint).wrapping_sub(wsize) as uInt as uInt;
            (*s).strstart = ((*s).strstart as libc::c_uint).wrapping_sub(wsize) as uInt as uInt;
            (*s).block_start -= wsize as libc::c_long;
            slide_hash(s);
            more = more.wrapping_add(wsize)
        }
        if (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint {
            break;
        }
        /* If there was no sliding:
         *    strstart <= WSIZE+MAX_DIST-1 && lookahead <= MIN_LOOKAHEAD - 1 &&
         *    more == window_size - lookahead - strstart
         * => more >= window_size - (MIN_LOOKAHEAD-1 + WSIZE + MAX_DIST-1)
         * => more >= window_size - 2*WSIZE + 2
         * In the BIG_MEM or MMAP case (not yet supported),
         *   window_size == input_size + MIN_LOOKAHEAD  &&
         *   strstart + s->lookahead <= input_size => more >= MIN_LOOKAHEAD.
         * Otherwise, window_size == 2*WSIZE so more >= 2.
         * If there was sliding, more >= WSIZE. So in all cases, more >= 2.
         */
        n = read_buf(
            (*s).strm,
            (*s).window
                .offset((*s).strstart as isize)
                .offset((*s).lookahead as isize),
            more,
        );
        (*s).lookahead = ((*s).lookahead as libc::c_uint).wrapping_add(n) as uInt as uInt;
        /* Initialize the hash value now that we have some input: */
        if (*s).lookahead.wrapping_add((*s).insert) >= MIN_MATCH as libc::c_uint {
            let mut str: uInt = (*s).strstart.wrapping_sub((*s).insert);
            (*s).ins_h = *(*s).window.offset(str as isize) as uInt;
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s)
                    .window
                    .offset(str.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                    as libc::c_uint)
                & (*s).hash_mask;
            while (*s).insert != 0 {
                (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                    ^ *(*s).window.offset(
                        str.wrapping_add(3 as libc::c_int as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as isize,
                    ) as libc::c_uint)
                    & (*s).hash_mask;
                *(*s).prev.offset((str & (*s).w_mask) as isize) =
                    *(*s).head.offset((*s).ins_h as isize);
                *(*s).head.offset((*s).ins_h as isize) = str as Pos;
                str = str.wrapping_add(1);
                (*s).insert = (*s).insert.wrapping_sub(1);
                if (*s).lookahead.wrapping_add((*s).insert) < MIN_MATCH as libc::c_uint {
                    break;
                }
            }
        }
        if !((*s).lookahead < MIN_LOOKAHEAD as libc::c_uint
            && (*(*s).strm).avail_in != 0 as libc::c_int as libc::c_uint)
        {
            break;
        }
    }
    /* If the WIN_INIT bytes after the end of the current data have never been
     * written, then zero those bytes in order to avoid memory check reports of
     * the use of uninitialized (or uninitialised as Julian writes) bytes by
     * the longest match routines.  Update the high water mark for the next
     * time through here.  WIN_INIT is set to MAX_MATCH since the longest match
     * routines allow scanning to strstart + MAX_MATCH, ignoring lookahead.
     */
    if (*s).high_water < (*s).window_size {
        let mut curr: ulg = ((*s).strstart as libc::c_ulong).wrapping_add((*s).lookahead as ulg);
        let mut init: ulg = 0;
        if (*s).high_water < curr {
            /* Previous high water mark below current data -- zero WIN_INIT
             * bytes or up to end of window, whichever is less.
             */
            init = (*s).window_size.wrapping_sub(curr);
            if init > WIN_INIT as libc::c_ulong {
                init = WIN_INIT as ulg
            }
            memset(
                (*s).window.offset(curr as isize) as *mut libc::c_void,
                0 as libc::c_int,
                init as libc::c_uint as libc::c_ulong,
            );
            (*s).high_water = curr.wrapping_add(init)
        } else if (*s).high_water < curr.wrapping_add(WIN_INIT as libc::c_ulong) {
            /* High water mark at or above current data, but below current data
             * plus WIN_INIT -- zero out to current data plus WIN_INIT, or up
             * to end of window, whichever is less.
             */
            init = curr
                .wrapping_add(WIN_INIT as libc::c_ulong)
                .wrapping_sub((*s).high_water);
            if init > (*s).window_size.wrapping_sub((*s).high_water) {
                init = (*s).window_size.wrapping_sub((*s).high_water)
            }
            memset(
                (*s).window.offset((*s).high_water as isize) as *mut libc::c_void,
                0 as libc::c_int,
                init as libc::c_uint as libc::c_ulong,
            );
            (*s).high_water = ((*s).high_water as libc::c_ulong).wrapping_add(init) as ulg as ulg
        }
    };
}
/* ===========================================================================
 * Flush the current block, with given end-of-file flag.
 * IN assertion: strstart is set to the end of the current match.
 */
/* Same but force premature exit if necessary. */
/* Maximum stored block length in deflate format (not including header). */
pub const MAX_STORED: libc::c_int = 65535 as libc::c_int;
/* ===========================================================================
 * Copy without compression as much as possible from the input stream, return
 * the current block state.
 *
 * In case deflateParams() is used to later switch to a non-zero compression
 * level, s->matches (otherwise unused when storing) keeps track of the number
 * of hash table slides to perform. If s->matches is 1, then one hash table
 * slide will be done when switching. If s->matches is 2, the maximum value
 * allowed here, then the hash table will be cleared, since two or more slides
 * is the same as a clear.
 *
 * deflate_stored() is written to minimize the number of times an input byte is
 * copied. It is most efficient with large input and output buffers, which
 * maximizes the opportunites to have a single copy from next_in to next_out.
 */
unsafe extern "C" fn deflate_stored(
    mut s: *mut deflate_state,
    mut flush: libc::c_int,
) -> block_state {
    /* Smallest worthy block size when not flushing or finishing. By default
     * this is 32K. This can be as small as 507 bytes for memLevel == 1. For
     * large input and output buffers, the stored block size will be larger.
     */
    let mut min_block: libc::c_uint = if (*s)
        .pending_buf_size
        .wrapping_sub(5 as libc::c_int as libc::c_ulong)
        > (*s).w_size as libc::c_ulong
    {
        (*s).w_size as libc::c_ulong
    } else {
        (*s).pending_buf_size
            .wrapping_sub(5 as libc::c_int as libc::c_ulong)
    } as libc::c_uint;
    /* Copy as many min_block or larger stored blocks directly to next_out as
     * possible. If flushing, copy the remaining available input to next_out as
     * stored blocks, if there is enough space.
     */
    let mut len: libc::c_uint = 0;
    let mut left: libc::c_uint = 0;
    let mut have: libc::c_uint = 0;
    let mut last: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut used: libc::c_uint = (*(*s).strm).avail_in;
    loop {
        /* Set len to the maximum size block that we can copy directly with the
         * available input data and output space. Set left to how much of that
         * would be copied from what's left in the window.
         */
        len = MAX_STORED as libc::c_uint; /* maximum deflate stored block length */
        have = ((*s).bi_valid + 42 as libc::c_int >> 3 as libc::c_int) as libc::c_uint; /* number of header bytes */
        if (*(*s).strm).avail_out < have {
            break;
        }
        /* maximum stored block length that will fit in avail_out: */
        have = (*(*s).strm).avail_out.wrapping_sub(have); /* bytes left in window */
        left = ((*s).strstart as libc::c_long - (*s).block_start) as libc::c_uint; /* limit len to the input */
        if len as libc::c_ulong > (left as ulg).wrapping_add((*(*s).strm).avail_in as libc::c_ulong)
        {
            len = left.wrapping_add((*(*s).strm).avail_in)
        } /* limit len to the output */
        if len > have {
            len = have
        }
        /* If the stored block would be less than min_block in length, or if
         * unable to copy all of the available input when flushing, then try
         * copying to the window and the pending buffer instead. Also don't
         * write an empty block when flushing -- deflate() does that.
         */
        if len < min_block
            && (len == 0 as libc::c_int as libc::c_uint && flush != Z_FINISH
                || flush == Z_NO_FLUSH
                || len != left.wrapping_add((*(*s).strm).avail_in))
        {
            break;
        }
        /* Make a dummy stored block in pending to get the header bytes,
         * including any pending bits. This also updates the debugging counts.
         */
        last = if flush == Z_FINISH && len == left.wrapping_add((*(*s).strm).avail_in) {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } as libc::c_uint;
        _tr_stored_block(
            s,
            0 as *mut libc::c_char,
            0 as libc::c_long as ulg,
            last as libc::c_int,
        );
        /* Replace the lengths in the dummy stored block with len. */
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize) =
            len as Bytef;
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize) =
            (len >> 8 as libc::c_int) as Bytef;
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize) =
            !len as Bytef;
        *(*s)
            .pending_buf
            .offset((*s).pending.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
            (!len >> 8 as libc::c_int) as Bytef;
        /* Write the stored block header bytes. */
        flush_pending((*s).strm);
        /* Copy uncompressed bytes from the window to next_out. */
        if left != 0 {
            if left > len {
                left = len
            }
            memcpy(
                (*(*s).strm).next_out as *mut libc::c_void,
                (*s).window.offset((*s).block_start as isize) as *const libc::c_void,
                left as libc::c_ulong,
            );
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(left as isize);
            (*(*s).strm).avail_out =
                ((*(*s).strm).avail_out as libc::c_uint).wrapping_sub(left) as uInt as uInt;
            (*(*s).strm).total_out = ((*(*s).strm).total_out as libc::c_ulong)
                .wrapping_add(left as libc::c_ulong) as uLong
                as uLong;
            (*s).block_start += left as libc::c_long;
            len = len.wrapping_sub(left)
        }
        /* Copy uncompressed bytes directly from next_in to next_out, updating
         * the check value.
         */
        if len != 0 {
            read_buf((*s).strm, (*(*s).strm).next_out, len);
            (*(*s).strm).next_out = (*(*s).strm).next_out.offset(len as isize);
            (*(*s).strm).avail_out =
                ((*(*s).strm).avail_out as libc::c_uint).wrapping_sub(len) as uInt as uInt;
            (*(*s).strm).total_out = ((*(*s).strm).total_out as libc::c_ulong)
                .wrapping_add(len as libc::c_ulong) as uLong
                as uLong
        }
        if !(last == 0 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    /* Update the sliding window with the last s->w_size bytes of the copied
     * data, or append all of the copied data to the existing window if less
     * than s->w_size bytes were copied. Also update the number of bytes to
     * insert in the hash tables, in the event that deflateParams() switches to
     * a non-zero compression level.
     */
    used = used.wrapping_sub((*(*s).strm).avail_in); /* number of input bytes directly copied */
    if used != 0 {
        /* If any input was used, then no unused input remains in the window,
         * therefore s->block_start == s->strstart.
         */
        if used >= (*s).w_size {
            /* supplant the previous history */
            (*s).matches = 2 as libc::c_int as uInt; /* clear hash */
            memcpy(
                (*s).window as *mut libc::c_void,
                (*(*s).strm).next_in.offset(-((*s).w_size as isize)) as *const libc::c_void,
                (*s).w_size as libc::c_ulong,
            );
            (*s).strstart = (*s).w_size
        } else {
            if (*s)
                .window_size
                .wrapping_sub((*s).strstart as libc::c_ulong)
                <= used as libc::c_ulong
            {
                /* Slide the window down. */
                (*s).strstart =
                    ((*s).strstart as libc::c_uint).wrapping_sub((*s).w_size) as uInt as uInt;
                memcpy(
                    (*s).window as *mut libc::c_void,
                    (*s).window.offset((*s).w_size as isize) as *const libc::c_void,
                    (*s).strstart as libc::c_ulong,
                );
                if (*s).matches < 2 as libc::c_int as libc::c_uint {
                    (*s).matches = (*s).matches.wrapping_add(1)
                }
                /* add a pending slide_hash() */
            }
            memcpy(
                (*s).window.offset((*s).strstart as isize) as *mut libc::c_void,
                (*(*s).strm).next_in.offset(-(used as isize)) as *const libc::c_void,
                used as libc::c_ulong,
            );
            (*s).strstart = ((*s).strstart as libc::c_uint).wrapping_add(used) as uInt as uInt
        }
        (*s).block_start = (*s).strstart as libc::c_long;
        (*s).insert = ((*s).insert as libc::c_uint).wrapping_add(
            if used > (*s).w_size.wrapping_sub((*s).insert) {
                (*s).w_size.wrapping_sub((*s).insert)
            } else {
                used
            },
        ) as uInt as uInt
    }
    if (*s).high_water < (*s).strstart as libc::c_ulong {
        (*s).high_water = (*s).strstart as ulg
    }
    /* If the last block was written to next_out, then done. */
    if last != 0 {
        return finish_done;
    }
    /* If flushing and all input has been consumed, then done. */
    if flush != Z_NO_FLUSH
        && flush != Z_FINISH
        && (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint
        && (*s).strstart as libc::c_long == (*s).block_start
    {
        return block_done;
    }
    /* Fill the window with any remaining input. */
    have = (*s)
        .window_size
        .wrapping_sub((*s).strstart as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
    if (*(*s).strm).avail_in > have && (*s).block_start >= (*s).w_size as libc::c_long {
        /* Slide the window down. */
        (*s).block_start -= (*s).w_size as libc::c_long;
        (*s).strstart = ((*s).strstart as libc::c_uint).wrapping_sub((*s).w_size) as uInt as uInt;
        memcpy(
            (*s).window as *mut libc::c_void,
            (*s).window.offset((*s).w_size as isize) as *const libc::c_void,
            (*s).strstart as libc::c_ulong,
        );
        /* more space now */
        if (*s).matches < 2 as libc::c_int as libc::c_uint {
            (*s).matches = (*s).matches.wrapping_add(1)
        } /* add a pending slide_hash() */
        have = have.wrapping_add((*s).w_size)
    }
    if have > (*(*s).strm).avail_in {
        have = (*(*s).strm).avail_in
    }
    if have != 0 {
        read_buf((*s).strm, (*s).window.offset((*s).strstart as isize), have);
        (*s).strstart = ((*s).strstart as libc::c_uint).wrapping_add(have) as uInt as uInt
    }
    if (*s).high_water < (*s).strstart as libc::c_ulong {
        (*s).high_water = (*s).strstart as ulg
    }
    /* There was not enough avail_out to write a complete worthy or flushed
     * stored block to next_out. Write a stored block to pending instead, if we
     * have enough input for a worthy block, or if flushing and there is enough
     * room for the remaining input as a stored block in the pending buffer.
     */
    have = ((*s).bi_valid + 42 as libc::c_int >> 3 as libc::c_int) as libc::c_uint; /* number of header bytes */
    /* maximum stored block length that will fit in pending: */
    have = if (*s).pending_buf_size.wrapping_sub(have as libc::c_ulong)
        > 65535 as libc::c_int as libc::c_ulong
    {
        65535 as libc::c_int as libc::c_ulong
    } else {
        (*s).pending_buf_size.wrapping_sub(have as libc::c_ulong)
    } as libc::c_uint;
    min_block = if have > (*s).w_size {
        (*s).w_size
    } else {
        have
    };
    left = ((*s).strstart as libc::c_long - (*s).block_start) as libc::c_uint;
    if left >= min_block
        || (left != 0 || flush == Z_FINISH)
            && flush != Z_NO_FLUSH
            && (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint
            && left <= have
    {
        len = if left > have { have } else { left };
        last = if flush == Z_FINISH
            && (*(*s).strm).avail_in == 0 as libc::c_int as libc::c_uint
            && len == left
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } as libc::c_uint;
        _tr_stored_block(
            s,
            ((*s).window as *mut charf).offset((*s).block_start as isize),
            len as ulg,
            last as libc::c_int,
        );
        (*s).block_start += len as libc::c_long;
        flush_pending((*s).strm);
    }
    /* We've done all we can with the available input and output. */
    return if last != 0 {
        finish_started as libc::c_int
    } else {
        need_more as libc::c_int
    } as block_state;
}
/* ===========================================================================
 * Compress as much as possible from the input stream, return the current
 * block state.
 * This function does not perform lazy evaluation of matches and inserts
 * new strings in the dictionary only for unmatched strings or for short
 * matches. It is used only for the fast compression options.
 */
unsafe extern "C" fn deflate_fast(
    mut s: *mut deflate_state,
    mut flush: libc::c_int,
) -> block_state {
    let mut hash_head: IPos = 0; /* head of the hash chain */
    let mut bflush: libc::c_int = 0; /* set if current block must be flushed */
    loop
    /* Make sure that we always have enough lookahead, except
     * at the end of the input file. We need MAX_MATCH bytes
     * for the next match, plus MIN_MATCH bytes to insert the
     * string following the next match.
     */
    {
        if (*s).lookahead < MIN_LOOKAHEAD as libc::c_uint {
            fill_window(s);
            if (*s).lookahead < MIN_LOOKAHEAD as libc::c_uint && flush == Z_NO_FLUSH {
                return need_more;
            }
            if (*s).lookahead == 0 as libc::c_int as libc::c_uint {
                break;
            }
            /* flush the current block */
        }
        /* Insert the string window[strstart .. strstart+2] in the
         * dictionary, and set hash_head to the head of the hash chain:
         */
        hash_head = NIL as IPos;
        if (*s).lookahead >= MIN_MATCH as libc::c_uint {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset(
                    (*s).strstart
                        .wrapping_add((3 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                        as isize,
                ) as libc::c_uint)
                & (*s).hash_mask;
            let ref mut fresh37 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
            *fresh37 = *(*s).head.offset((*s).ins_h as isize);
            hash_head = *fresh37 as IPos;
            *(*s).head.offset((*s).ins_h as isize) = (*s).strstart as Pos
        }
        /* Find the longest match, discarding those <= prev_length.
         * At this point we have always match_length < MIN_MATCH
         */
        if hash_head != NIL as libc::c_uint
            && (*s).strstart.wrapping_sub(hash_head)
                <= (*s).w_size.wrapping_sub(MIN_LOOKAHEAD as libc::c_uint)
        {
            /* To simplify the code, we prevent matches with the string
             * of window index 0 (in particular we have to avoid a match
             * of the string with itself at the start of the input file).
             */
            (*s).match_length = longest_match(s, hash_head)
            /* longest_match() sets match_start */
        }
        if (*s).match_length >= MIN_MATCH as libc::c_uint {
            let mut len: uch =
                (*s).match_length
                    .wrapping_sub(3 as libc::c_int as libc::c_uint) as uch;
            let mut dist: ush = (*s).strstart.wrapping_sub((*s).match_start) as ush;
            *(*s).d_buf.offset((*s).last_lit as isize) = dist;
            let fresh38 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh38 as isize) = len;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*_length_code.as_ptr().offset(len as isize) as libc::c_int
                + LITERALS
                + 1 as libc::c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(*_length_code.as_ptr().offset(len as isize) as libc::c_int
                + LITERALS
                + 1 as libc::c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[if (dist as libc::c_int) < 256 as libc::c_int {
                *_dist_code.as_ptr().offset(dist as isize) as libc::c_int
            } else {
                *_dist_code.as_ptr().offset(
                    (256 as libc::c_int + (dist as libc::c_int >> 7 as libc::c_int)) as isize,
                ) as libc::c_int
            } as usize]
                .fc
                .freq = (*s).dyn_dtree[if (dist as libc::c_int) < 256 as libc::c_int {
                *_dist_code.as_ptr().offset(dist as isize) as libc::c_int
            } else {
                *_dist_code.as_ptr().offset(
                    (256 as libc::c_int + (dist as libc::c_int >> 7 as libc::c_int)) as isize,
                ) as libc::c_int
            } as usize]
                .fc
                .freq
                .wrapping_add(1);
            bflush = ((*s).last_lit
                == (*s)
                    .lit_bufsize
                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                as libc::c_int;
            (*s).lookahead =
                ((*s).lookahead as libc::c_uint).wrapping_sub((*s).match_length) as uInt as uInt;
            /* Insert new strings in the hash table only if the match length
             * is not too large. This saves time but degrades compression.
             */
            if (*s).match_length <= (*s).max_lazy_match
                && (*s).lookahead >= MIN_MATCH as libc::c_uint
            {
                (*s).match_length = (*s).match_length.wrapping_sub(1); /* string at strstart already in table */
                loop {
                    (*s).strstart = (*s).strstart.wrapping_add(1);
                    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                        ^ *(*s).window.offset(
                            (*s).strstart
                                .wrapping_add((3 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                                as isize,
                        ) as libc::c_uint)
                        & (*s).hash_mask;
                    let ref mut fresh39 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
                    *fresh39 = *(*s).head.offset((*s).ins_h as isize);
                    hash_head = *fresh39 as IPos;
                    *(*s).head.offset((*s).ins_h as isize) = (*s).strstart as Pos;
                    (*s).match_length = (*s).match_length.wrapping_sub(1);
                    if !((*s).match_length != 0 as libc::c_int as libc::c_uint) {
                        break;
                    }
                    /* strstart never exceeds WSIZE-MAX_MATCH, so there are
                     * always MIN_MATCH bytes ahead.
                     */
                }
                (*s).strstart = (*s).strstart.wrapping_add(1)
            } else {
                (*s).strstart =
                    ((*s).strstart as libc::c_uint).wrapping_add((*s).match_length) as uInt as uInt;
                (*s).match_length = 0 as libc::c_int as uInt;
                (*s).ins_h = *(*s).window.offset((*s).strstart as isize) as uInt;
                (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                    ^ *(*s).window.offset(
                        (*s).strstart.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                    ) as libc::c_uint)
                    & (*s).hash_mask
                /* If lookahead < MIN_MATCH, ins_h is garbage, but it does not
                 * matter since it will be recomputed at next deflate call.
                 */
            }
        } else {
            /* No match, output a literal byte */
            let mut cc: uch = *(*s).window.offset((*s).strstart as isize);
            *(*s).d_buf.offset((*s).last_lit as isize) = 0 as libc::c_int as ushf;
            let fresh40 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh40 as isize) = cc;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
            bflush = ((*s).last_lit
                == (*s)
                    .lit_bufsize
                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                as libc::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1)
        }
        if bflush != 0 {
            _tr_flush_block(
                s,
                if (*s).block_start >= 0 as libc::c_long {
                    &mut *(*s)
                        .window
                        .offset((*s).block_start as libc::c_uint as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
                0 as libc::c_int,
            );
            (*s).block_start = (*s).strstart as libc::c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                return if 0 as libc::c_int != 0 {
                    finish_started as libc::c_int
                } else {
                    need_more as libc::c_int
                } as block_state;
            }
        }
    }
    (*s).insert = if (*s).strstart < (MIN_MATCH - 1 as libc::c_int) as libc::c_uint {
        (*s).strstart
    } else {
        (MIN_MATCH - 1 as libc::c_int) as libc::c_uint
    };
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            1 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 1 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            0 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 0 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
    }
    return block_done;
}
/* ===========================================================================
 * Same as above, but achieves better compression. We use a lazy
 * evaluation for matches: a match is finally adopted only if there is
 * no better match at the next window position.
 */
unsafe extern "C" fn deflate_slow(
    mut s: *mut deflate_state,
    mut flush: libc::c_int,
) -> block_state {
    let mut hash_head: IPos = 0; /* head of hash chain */
    let mut bflush: libc::c_int = 0; /* set if current block must be flushed */
    /* Process the input block. */
    loop
    /* Make sure that we always have enough lookahead, except
     * at the end of the input file. We need MAX_MATCH bytes
     * for the next match, plus MIN_MATCH bytes to insert the
     * string following the next match.
     */
    {
        if (*s).lookahead < MIN_LOOKAHEAD as libc::c_uint {
            fill_window(s);
            if (*s).lookahead < MIN_LOOKAHEAD as libc::c_uint && flush == Z_NO_FLUSH {
                return need_more;
            }
            if (*s).lookahead == 0 as libc::c_int as libc::c_uint {
                break;
            }
            /* flush the current block */
        }
        /* Insert the string window[strstart .. strstart+2] in the
         * dictionary, and set hash_head to the head of the hash chain:
         */
        hash_head = NIL as IPos;
        if (*s).lookahead >= MIN_MATCH as libc::c_uint {
            (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                ^ *(*s).window.offset(
                    (*s).strstart
                        .wrapping_add((3 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                        as isize,
                ) as libc::c_uint)
                & (*s).hash_mask;
            let ref mut fresh41 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
            *fresh41 = *(*s).head.offset((*s).ins_h as isize);
            hash_head = *fresh41 as IPos;
            *(*s).head.offset((*s).ins_h as isize) = (*s).strstart as Pos
        }
        /* Find the longest match, discarding those <= prev_length.
         */
        (*s).prev_length = (*s).match_length;
        (*s).prev_match = (*s).match_start;
        (*s).match_length = (MIN_MATCH - 1 as libc::c_int) as uInt;
        if hash_head != NIL as libc::c_uint
            && (*s).prev_length < (*s).max_lazy_match
            && (*s).strstart.wrapping_sub(hash_head)
                <= (*s).w_size.wrapping_sub(MIN_LOOKAHEAD as libc::c_uint)
        {
            /* To simplify the code, we prevent matches with the string
             * of window index 0 (in particular we have to avoid a match
             * of the string with itself at the start of the input file).
             */
            (*s).match_length = longest_match(s, hash_head);
            /* longest_match() sets match_start */
            if (*s).match_length <= 5 as libc::c_int as libc::c_uint
                && ((*s).strategy == Z_FILTERED
                    || (*s).match_length == MIN_MATCH as libc::c_uint
                        && (*s).strstart.wrapping_sub((*s).match_start) > TOO_FAR as libc::c_uint)
            {
                /* If prev_match is also MIN_MATCH, match_start is garbage
                 * but we will ignore the current match anyway.
                 */
                (*s).match_length = (MIN_MATCH - 1 as libc::c_int) as uInt
            }
        }
        /* If there was a match at the previous step and the current
         * match is not better, output the previous match:
         */
        if (*s).prev_length >= MIN_MATCH as libc::c_uint && (*s).match_length <= (*s).prev_length {
            let mut max_insert: uInt = (*s)
                .strstart
                .wrapping_add((*s).lookahead)
                .wrapping_sub(MIN_MATCH as libc::c_uint);
            /* Do not insert strings in hash table beyond this. */
            let mut len: uch =
                (*s).prev_length
                    .wrapping_sub(3 as libc::c_int as libc::c_uint) as uch;
            let mut dist: ush = (*s)
                .strstart
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_sub((*s).prev_match) as ush;
            *(*s).d_buf.offset((*s).last_lit as isize) = dist;
            let fresh42 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh42 as isize) = len;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*_length_code.as_ptr().offset(len as isize) as libc::c_int
                + LITERALS
                + 1 as libc::c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(*_length_code.as_ptr().offset(len as isize) as libc::c_int
                + LITERALS
                + 1 as libc::c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[if (dist as libc::c_int) < 256 as libc::c_int {
                *_dist_code.as_ptr().offset(dist as isize) as libc::c_int
            } else {
                *_dist_code.as_ptr().offset(
                    (256 as libc::c_int + (dist as libc::c_int >> 7 as libc::c_int)) as isize,
                ) as libc::c_int
            } as usize]
                .fc
                .freq = (*s).dyn_dtree[if (dist as libc::c_int) < 256 as libc::c_int {
                *_dist_code.as_ptr().offset(dist as isize) as libc::c_int
            } else {
                *_dist_code.as_ptr().offset(
                    (256 as libc::c_int + (dist as libc::c_int >> 7 as libc::c_int)) as isize,
                ) as libc::c_int
            } as usize]
                .fc
                .freq
                .wrapping_add(1);
            bflush = ((*s).last_lit
                == (*s)
                    .lit_bufsize
                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                as libc::c_int;
            /* Insert in hash table all strings up to the end of the match.
             * strstart-1 and strstart are already inserted. If there is not
             * enough lookahead, the last two strings are not inserted in
             * the hash table.
             */
            (*s).lookahead = ((*s).lookahead as libc::c_uint).wrapping_sub(
                (*s).prev_length
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) as uInt as uInt;
            (*s).prev_length = ((*s).prev_length as libc::c_uint)
                .wrapping_sub(2 as libc::c_int as libc::c_uint)
                as uInt as uInt;
            loop {
                (*s).strstart = (*s).strstart.wrapping_add(1);
                if (*s).strstart <= max_insert {
                    (*s).ins_h = ((*s).ins_h << (*s).hash_shift
                        ^ *(*s).window.offset(
                            (*s).strstart
                                .wrapping_add((3 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                                as isize,
                        ) as libc::c_uint)
                        & (*s).hash_mask;
                    let ref mut fresh43 = *(*s).prev.offset(((*s).strstart & (*s).w_mask) as isize);
                    *fresh43 = *(*s).head.offset((*s).ins_h as isize);
                    hash_head = *fresh43 as IPos;
                    *(*s).head.offset((*s).ins_h as isize) = (*s).strstart as Pos
                }
                (*s).prev_length = (*s).prev_length.wrapping_sub(1);
                if !((*s).prev_length != 0 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
            (*s).match_available = 0 as libc::c_int;
            (*s).match_length = (MIN_MATCH - 1 as libc::c_int) as uInt;
            (*s).strstart = (*s).strstart.wrapping_add(1);
            if bflush != 0 {
                _tr_flush_block(
                    s,
                    if (*s).block_start >= 0 as libc::c_long {
                        &mut *(*s)
                            .window
                            .offset((*s).block_start as libc::c_uint as isize)
                            as *mut Bytef as *mut charf
                    } else {
                        0 as *mut charf
                    },
                    ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
                    0 as libc::c_int,
                );
                (*s).block_start = (*s).strstart as libc::c_long;
                flush_pending((*s).strm);
                if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                    return if 0 as libc::c_int != 0 {
                        finish_started as libc::c_int
                    } else {
                        need_more as libc::c_int
                    } as block_state;
                }
            }
        } else if (*s).match_available != 0 {
            /* If there was no match at the previous position, output a
             * single literal. If there was a match but the current match
             * is longer, truncate the previous match to a single literal.
             */
            let mut cc: uch = *(*s)
                .window
                .offset((*s).strstart.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
            *(*s).d_buf.offset((*s).last_lit as isize) = 0 as libc::c_int as ushf;
            let fresh44 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh44 as isize) = cc;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
            bflush = ((*s).last_lit
                == (*s)
                    .lit_bufsize
                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                as libc::c_int;
            if bflush != 0 {
                _tr_flush_block(
                    s,
                    if (*s).block_start >= 0 as libc::c_long {
                        &mut *(*s)
                            .window
                            .offset((*s).block_start as libc::c_uint as isize)
                            as *mut Bytef as *mut charf
                    } else {
                        0 as *mut charf
                    },
                    ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
                    0 as libc::c_int,
                );
                (*s).block_start = (*s).strstart as libc::c_long;
                flush_pending((*s).strm);
            }
            (*s).strstart = (*s).strstart.wrapping_add(1);
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                return need_more;
            }
        } else {
            /* There is no previous match to compare with, wait for
             * the next step to decide.
             */
            (*s).match_available = 1 as libc::c_int;
            (*s).strstart = (*s).strstart.wrapping_add(1);
            (*s).lookahead = (*s).lookahead.wrapping_sub(1)
        }
    }
    if (*s).match_available != 0 {
        let mut cc_0: uch = *(*s)
            .window
            .offset((*s).strstart.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        *(*s).d_buf.offset((*s).last_lit as isize) = 0 as libc::c_int as ushf;
        let fresh45 = (*s).last_lit;
        (*s).last_lit = (*s).last_lit.wrapping_add(1);
        *(*s).l_buf.offset(fresh45 as isize) = cc_0;
        (*s).dyn_ltree[cc_0 as usize].fc.freq =
            (*s).dyn_ltree[cc_0 as usize].fc.freq.wrapping_add(1);
        bflush = ((*s).last_lit
            == (*s)
                .lit_bufsize
                .wrapping_sub(1 as libc::c_int as libc::c_uint)) as libc::c_int;
        (*s).match_available = 0 as libc::c_int
    }
    (*s).insert = if (*s).strstart < (MIN_MATCH - 1 as libc::c_int) as libc::c_uint {
        (*s).strstart
    } else {
        (MIN_MATCH - 1 as libc::c_int) as libc::c_uint
    };
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            1 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 1 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            0 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 0 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
    }
    return block_done;
}
/* FASTEST */
/* ===========================================================================
 * For Z_RLE, simply look for runs of bytes, generate matches only of distance
 * one.  Do not maintain a hash table.  (It will be regenerated if this run of
 * deflate switches away from Z_RLE.)
 */
unsafe extern "C" fn deflate_rle(mut s: *mut deflate_state, mut flush: libc::c_int) -> block_state {
    let mut bflush: libc::c_int = 0; /* set if current block must be flushed */
    let mut prev: uInt = 0; /* byte at distance one to match */
    let mut scan: *mut Bytef = 0 as *mut Bytef; /* scan goes up to strend for length of run */
    let mut strend: *mut Bytef = 0 as *mut Bytef;
    loop
    /* Make sure that we always have enough lookahead, except
     * at the end of the input file. We need MAX_MATCH bytes
     * for the longest run, plus one for the unrolled loop.
     */
    {
        if (*s).lookahead <= MAX_MATCH as libc::c_uint {
            fill_window(s);
            if (*s).lookahead <= MAX_MATCH as libc::c_uint && flush == Z_NO_FLUSH {
                return need_more;
            }
            if (*s).lookahead == 0 as libc::c_int as libc::c_uint {
                break;
            }
            /* flush the current block */
        }
        /* See how many times the previous byte repeats */
        (*s).match_length = 0 as libc::c_int as uInt;
        if (*s).lookahead >= MIN_MATCH as libc::c_uint
            && (*s).strstart > 0 as libc::c_int as libc::c_uint
        {
            scan = (*s)
                .window
                .offset((*s).strstart as isize)
                .offset(-(1 as libc::c_int as isize));
            prev = *scan as uInt;
            scan = scan.offset(1);
            if prev == *scan as libc::c_uint
                && {
                    scan = scan.offset(1);
                    (prev) == *scan as libc::c_uint
                }
                && {
                    scan = scan.offset(1);
                    (prev) == *scan as libc::c_uint
                }
            {
                strend = (*s)
                    .window
                    .offset((*s).strstart as isize)
                    .offset(MAX_MATCH as isize);
                loop {
                    scan = scan.offset(1);
                    if !(prev == *scan as libc::c_uint
                        && {
                            scan = scan.offset(1);
                            (prev) == *scan as libc::c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            (prev) == *scan as libc::c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            (prev) == *scan as libc::c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            (prev) == *scan as libc::c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            (prev) == *scan as libc::c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            (prev) == *scan as libc::c_uint
                        }
                        && {
                            scan = scan.offset(1);
                            (prev) == *scan as libc::c_uint
                        }
                        && scan < strend)
                    {
                        break;
                    }
                }
                (*s).match_length = (MAX_MATCH as libc::c_uint)
                    .wrapping_sub(strend.offset_from(scan) as libc::c_long as uInt);
                if (*s).match_length > (*s).lookahead {
                    (*s).match_length = (*s).lookahead
                }
            }
        }
        /* Emit match if have run of MIN_MATCH or longer, else emit literal */
        if (*s).match_length >= MIN_MATCH as libc::c_uint {
            let mut len: uch =
                (*s).match_length
                    .wrapping_sub(3 as libc::c_int as libc::c_uint) as uch;
            let mut dist: ush = 1 as libc::c_int as ush;
            *(*s).d_buf.offset((*s).last_lit as isize) = dist;
            let fresh46 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh46 as isize) = len;
            dist = dist.wrapping_sub(1);
            (*s).dyn_ltree[(*_length_code.as_ptr().offset(len as isize) as libc::c_int
                + LITERALS
                + 1 as libc::c_int) as usize]
                .fc
                .freq = (*s).dyn_ltree[(*_length_code.as_ptr().offset(len as isize) as libc::c_int
                + LITERALS
                + 1 as libc::c_int) as usize]
                .fc
                .freq
                .wrapping_add(1);
            (*s).dyn_dtree[if (dist as libc::c_int) < 256 as libc::c_int {
                *_dist_code.as_ptr().offset(dist as isize) as libc::c_int
            } else {
                *_dist_code.as_ptr().offset(
                    (256 as libc::c_int + (dist as libc::c_int >> 7 as libc::c_int)) as isize,
                ) as libc::c_int
            } as usize]
                .fc
                .freq = (*s).dyn_dtree[if (dist as libc::c_int) < 256 as libc::c_int {
                *_dist_code.as_ptr().offset(dist as isize) as libc::c_int
            } else {
                *_dist_code.as_ptr().offset(
                    (256 as libc::c_int + (dist as libc::c_int >> 7 as libc::c_int)) as isize,
                ) as libc::c_int
            } as usize]
                .fc
                .freq
                .wrapping_add(1);
            bflush = ((*s).last_lit
                == (*s)
                    .lit_bufsize
                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                as libc::c_int;
            (*s).lookahead =
                ((*s).lookahead as libc::c_uint).wrapping_sub((*s).match_length) as uInt as uInt;
            (*s).strstart =
                ((*s).strstart as libc::c_uint).wrapping_add((*s).match_length) as uInt as uInt;
            (*s).match_length = 0 as libc::c_int as uInt
        } else {
            /* No match, output a literal byte */
            let mut cc: uch = *(*s).window.offset((*s).strstart as isize);
            *(*s).d_buf.offset((*s).last_lit as isize) = 0 as libc::c_int as ushf;
            let fresh47 = (*s).last_lit;
            (*s).last_lit = (*s).last_lit.wrapping_add(1);
            *(*s).l_buf.offset(fresh47 as isize) = cc;
            (*s).dyn_ltree[cc as usize].fc.freq =
                (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
            bflush = ((*s).last_lit
                == (*s)
                    .lit_bufsize
                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                as libc::c_int;
            (*s).lookahead = (*s).lookahead.wrapping_sub(1);
            (*s).strstart = (*s).strstart.wrapping_add(1)
        }
        if bflush != 0 {
            _tr_flush_block(
                s,
                if (*s).block_start >= 0 as libc::c_long {
                    &mut *(*s)
                        .window
                        .offset((*s).block_start as libc::c_uint as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
                0 as libc::c_int,
            );
            (*s).block_start = (*s).strstart as libc::c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                return if 0 as libc::c_int != 0 {
                    finish_started as libc::c_int
                } else {
                    need_more as libc::c_int
                } as block_state;
            }
        }
    }
    (*s).insert = 0 as libc::c_int as uInt;
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            1 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 1 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            0 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 0 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
    }
    return block_done;
}
/* ===========================================================================
 * For Z_HUFFMAN_ONLY, do not look for matches.  Do not maintain a hash table.
 * (It will be regenerated if this run of deflate switches away from Huffman.)
 */
unsafe extern "C" fn deflate_huff(
    mut s: *mut deflate_state,
    mut flush: libc::c_int,
) -> block_state {
    let mut bflush: libc::c_int = 0; /* set if current block must be flushed */
    loop
    /* Make sure that we have a literal to write. */
    {
        if (*s).lookahead == 0 as libc::c_int as libc::c_uint {
            fill_window(s);
            if (*s).lookahead == 0 as libc::c_int as libc::c_uint {
                if flush == Z_NO_FLUSH {
                    return need_more;
                }
                break;
                /* flush the current block */
            }
        }
        /* Output a literal byte */
        (*s).match_length = 0 as libc::c_int as uInt;
        let mut cc: uch = *(*s).window.offset((*s).strstart as isize);
        *(*s).d_buf.offset((*s).last_lit as isize) = 0 as libc::c_int as ushf;
        let fresh48 = (*s).last_lit;
        (*s).last_lit = (*s).last_lit.wrapping_add(1);
        *(*s).l_buf.offset(fresh48 as isize) = cc;
        (*s).dyn_ltree[cc as usize].fc.freq = (*s).dyn_ltree[cc as usize].fc.freq.wrapping_add(1);
        bflush = ((*s).last_lit
            == (*s)
                .lit_bufsize
                .wrapping_sub(1 as libc::c_int as libc::c_uint)) as libc::c_int;
        (*s).lookahead = (*s).lookahead.wrapping_sub(1);
        (*s).strstart = (*s).strstart.wrapping_add(1);
        if bflush != 0 {
            _tr_flush_block(
                s,
                if (*s).block_start >= 0 as libc::c_long {
                    &mut *(*s)
                        .window
                        .offset((*s).block_start as libc::c_uint as isize)
                        as *mut Bytef as *mut charf
                } else {
                    0 as *mut charf
                },
                ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
                0 as libc::c_int,
            );
            (*s).block_start = (*s).strstart as libc::c_long;
            flush_pending((*s).strm);
            if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
                return if 0 as libc::c_int != 0 {
                    finish_started as libc::c_int
                } else {
                    need_more as libc::c_int
                } as block_state;
            }
        }
    }
    (*s).insert = 0 as libc::c_int as uInt;
    if flush == Z_FINISH {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            1 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 1 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
        return finish_done;
    }
    if (*s).last_lit != 0 {
        _tr_flush_block(
            s,
            if (*s).block_start >= 0 as libc::c_long {
                &mut *(*s)
                    .window
                    .offset((*s).block_start as libc::c_uint as isize) as *mut Bytef
                    as *mut charf
            } else {
                0 as *mut charf
            },
            ((*s).strstart as libc::c_long - (*s).block_start) as ulg,
            0 as libc::c_int,
        );
        (*s).block_start = (*s).strstart as libc::c_long;
        flush_pending((*s).strm);
        if (*(*s).strm).avail_out == 0 as libc::c_int as libc::c_uint {
            return if 0 as libc::c_int != 0 {
                finish_started as libc::c_int
            } else {
                need_more as libc::c_int
            } as block_state;
        }
    }
    return block_done;
}
