use ::libc;
extern "C" {
    pub type internal_state;
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: libc::c_uchar,
    pub bits: libc::c_uchar,
    pub val: libc::c_ushort,
}
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
pub const COPY_: inflate_mode = 16194;
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
/* inffast.h -- header to use inffast.c
 * Copyright (C) 1995-2003, 2010 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* WARNING: this file should *not* be used by applications. It is
  part of the implementation of the compression library and is
  subject to change. Applications should only use zlib.h.
*/
/* inffast.c -- fast decoding
 * Copyright (C) 1995-2017 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
  Decode literal, length, and distance codes and write out the resulting
  literal and match bytes until either not enough input or output is
  available, an end-of-block is encountered, or a data error is encountered.
  When large enough input and output buffers are supplied to inflate(), for
  example, a 16K input buffer and a 64K output buffer, more than 95% of the
  inflate execution time is spent in this routine.

  Entry assumptions:

       state->mode == LEN
       strm->avail_in >= 6
       strm->avail_out >= 258
       start >= strm->avail_out
       state->bits < 8

  On return, state->mode is one of:

       LEN -- ran out of enough output space or enough available input
       TYPE -- reached end of block code, inflate() to interpret next block
       BAD -- error in block data

  Notes:

   - The maximum input bits used by a length/distance pair is 15 bits for the
     length code, 5 bits for the length extra, 15 bits for the distance code,
     and 13 bits for the distance extra.  This totals 48 bits, or six bytes.
     Therefore if strm->avail_in >= 6, then there is enough input to avoid
     checking for available input while decoding.

   - The maximum bytes that a single length/distance pair can output is 258
     bytes, which is the maximum length that can be coded.  inflate_fast()
     requires strm->avail_out >= 258 for each loop to avoid checking for
     output space.
*/
#[no_mangle]
pub unsafe extern "C" fn inflate_fast(mut strm: z_streamp, mut start: libc::c_uint)
/* inflate()'s starting value for strm->avail_out */
{
    let mut state: *mut inflate_state = 0 as *mut inflate_state; /* local strm->next_in */
    let mut in_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* have enough input while in < last */
    let mut last: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* local strm->next_out */
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* inflate()'s initial strm->next_out */
    let mut beg: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* while out < end, enough space available */
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* window size or zero if not using window */
    let mut wsize: libc::c_uint = 0; /* valid bytes in the window */
    let mut whave: libc::c_uint = 0; /* window write index */
    let mut wnext: libc::c_uint = 0; /* allocated sliding window, if wsize != 0 */
    let mut window: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* local strm->hold */
    let mut hold: libc::c_ulong = 0; /* local strm->bits */
    let mut bits: libc::c_uint = 0; /* local strm->lencode */
    let mut lcode: *const code = 0 as *const code; /* local strm->distcode */
    let mut dcode: *const code = 0 as *const code; /* mask for first level of length codes */
    let mut lmask: libc::c_uint = 0; /* mask for first level of distance codes */
    let mut dmask: libc::c_uint = 0; /* retrieved table entry */
    let mut here: code = code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* code bits, operation, extra bits, or */
    /*  window position, window bytes to copy */
    let mut op: libc::c_uint = 0; /* match length, unused bytes */
    let mut len: libc::c_uint = 0; /* match distance */
    let mut dist: libc::c_uint = 0; /* where to copy match from */
    let mut from: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* copy state to local variables */
    state = (*strm).state as *mut inflate_state;
    in_0 = (*strm).next_in;
    last = in_0.offset(
        (*strm)
            .avail_in
            .wrapping_sub(5 as libc::c_int as libc::c_uint) as isize,
    );
    out = (*strm).next_out;
    beg = out.offset(-(start.wrapping_sub((*strm).avail_out) as isize));
    end = out.offset(
        (*strm)
            .avail_out
            .wrapping_sub(257 as libc::c_int as libc::c_uint) as isize,
    );
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask =
        ((1 as libc::c_uint) << (*state).lenbits).wrapping_sub(1 as libc::c_int as libc::c_uint);
    dmask =
        ((1 as libc::c_uint) << (*state).distbits).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut current_block_141: u64;
    's_132: loop
    /* decode literals and length/distances until end-of-block or not enough
    input data or output space */
    {
        if bits < 15 as libc::c_int as libc::c_uint {
            let fresh0 = in_0;
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*fresh0 as libc::c_ulong) << bits);
            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*fresh1 as libc::c_ulong) << bits);
            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
        }
        here = *lcode.offset((hold & lmask as libc::c_ulong) as isize);
        loop {
            op = here.bits as libc::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here.op as libc::c_uint;
            if op == 0 as libc::c_int as libc::c_uint {
                /* literal */
                let fresh2 = out;
                out = out.offset(1);
                *fresh2 = here.val as libc::c_uchar;
                current_block_141 = 18386322304582297246;
                break;
            } else if op & 16 as libc::c_int as libc::c_uint != 0 {
                /* length base */
                len = here.val as libc::c_uint; /* number of extra bits */
                op &= 15 as libc::c_int as libc::c_uint;
                if op != 0 {
                    if bits < op {
                        let fresh3 = in_0;
                        in_0 = in_0.offset(1);
                        hold = hold.wrapping_add((*fresh3 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    len = len.wrapping_add(
                        hold as libc::c_uint
                            & ((1 as libc::c_uint) << op)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    hold >>= op;
                    bits = bits.wrapping_sub(op)
                }
                if bits < 15 as libc::c_int as libc::c_uint {
                    let fresh4 = in_0;
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*fresh4 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    let fresh5 = in_0;
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*fresh5 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                here = *dcode.offset((hold & dmask as libc::c_ulong) as isize);
                current_block_141 = 7338997171777974246;
                break;
            } else if op & 64 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                /* 2nd level length code */
                here = *lcode.offset((here.val as libc::c_ulong).wrapping_add(
                    hold
                        & ((1 as libc::c_uint) << op).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                ) as isize)
            } else if op & 32 as libc::c_int as libc::c_uint != 0 {
                current_block_141 = 4976922244085895320;
                break;
            } else {
                current_block_141 = 5832582820025303349;
                break;
            }
        }
        match current_block_141 {
            7338997171777974246 => {
                loop {
                    op = here.bits as libc::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here.op as libc::c_uint;
                    if op & 16 as libc::c_int as libc::c_uint != 0 {
                        /* distance base */
                        dist = here.val as libc::c_uint; /* number of extra bits */
                        op &= 15 as libc::c_int as libc::c_uint; /* max distance in output */
                        if bits < op {
                            let fresh6 = in_0;
                            in_0 = in_0.offset(1);
                            hold = hold.wrapping_add((*fresh6 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                            if bits < op {
                                let fresh7 = in_0;
                                in_0 = in_0.offset(1);
                                hold = hold.wrapping_add((*fresh7 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                        }
                        dist = dist.wrapping_add(
                            hold as libc::c_uint
                                & ((1 as libc::c_uint) << op)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        op = out.offset_from(beg) as libc::c_long as libc::c_uint;
                        if dist > op {
                            current_block_141 = 5873035170358615968;
                            break;
                        } else {
                            current_block_141 = 17239133558811367971;
                            break;
                        }
                    } else if op & 64 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        /* 2nd level distance code */
                        here = *dcode.offset(
                            (here.val as libc::c_ulong).wrapping_add(
                                hold & ((1 as libc::c_uint) << op)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong,
                            ) as isize,
                        )
                    } else {
                        (*strm).msg = b"invalid distance code\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char; /* copy direct from output */
                        (*state).mode = BAD;
                        break 's_132;
                    }
                }
                match current_block_141 {
                    17239133558811367971 => {
                        from = out.offset(-(dist as isize));
                        loop {
                            /* minimum length is three */
                            let fresh26 = from;
                            from = from.offset(1);
                            let fresh27 = out;
                            out = out.offset(1);
                            *fresh27 = *fresh26;
                            let fresh28 = from;
                            from = from.offset(1);
                            let fresh29 = out;
                            out = out.offset(1);
                            *fresh29 = *fresh28;
                            let fresh30 = from;
                            from = from.offset(1);
                            let fresh31 = out;
                            out = out.offset(1);
                            *fresh31 = *fresh30;
                            len = len.wrapping_sub(3 as libc::c_int as libc::c_uint);
                            if !(len > 2 as libc::c_int as libc::c_uint) {
                                break;
                            }
                        }
                        if len != 0 {
                            let fresh32 = from;
                            from = from.offset(1);
                            let fresh33 = out;
                            out = out.offset(1);
                            *fresh33 = *fresh32;
                            if len > 1 as libc::c_int as libc::c_uint {
                                let fresh34 = from;
                                from = from.offset(1);
                                let fresh35 = out;
                                out = out.offset(1);
                                *fresh35 = *fresh34
                            }
                        }
                    }
                    _ => {
                        /* see if copy from window */
                        op = dist.wrapping_sub(op); /* distance back in window */
                        if op > whave {
                            if (*state).sane != 0 {
                                (*strm).msg = b"invalid distance too far back\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                                (*state).mode = BAD;
                                break;
                            }
                        }
                        from = window;
                        if wnext == 0 as libc::c_int as libc::c_uint {
                            /* very common case */
                            from = from.offset(wsize.wrapping_sub(op) as isize);
                            if op < len {
                                /* some from window */
                                len = len.wrapping_sub(op);
                                loop {
                                    let fresh8 = from;
                                    from = from.offset(1);
                                    let fresh9 = out;
                                    out = out.offset(1);
                                    *fresh9 = *fresh8;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize))
                                /* rest from output */
                            }
                        } else if wnext < op {
                            /* wrap around window */
                            from = from.offset(wsize.wrapping_add(wnext).wrapping_sub(op) as isize);
                            op = op.wrapping_sub(wnext);
                            if op < len {
                                /* some from end of window */
                                len = len.wrapping_sub(op);
                                loop {
                                    let fresh10 = from;
                                    from = from.offset(1);
                                    let fresh11 = out;
                                    out = out.offset(1);
                                    *fresh11 = *fresh10;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = window;
                                if wnext < len {
                                    /* some from start of window */
                                    op = wnext;
                                    len = len.wrapping_sub(op);
                                    loop {
                                        let fresh12 = from;
                                        from = from.offset(1);
                                        let fresh13 = out;
                                        out = out.offset(1);
                                        *fresh13 = *fresh12;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = out.offset(-(dist as isize))
                                    /* rest from output */
                                }
                            }
                        } else {
                            /* contiguous in window */
                            from = from.offset(wnext.wrapping_sub(op) as isize);
                            if op < len {
                                /* some from window */
                                len = len.wrapping_sub(op);
                                loop {
                                    let fresh14 = from;
                                    from = from.offset(1);
                                    let fresh15 = out;
                                    out = out.offset(1);
                                    *fresh15 = *fresh14;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize))
                                /* rest from output */
                            }
                        }
                        while len > 2 as libc::c_int as libc::c_uint {
                            let fresh16 = from;
                            from = from.offset(1);
                            let fresh17 = out;
                            out = out.offset(1);
                            *fresh17 = *fresh16;
                            let fresh18 = from;
                            from = from.offset(1);
                            let fresh19 = out;
                            out = out.offset(1);
                            *fresh19 = *fresh18;
                            let fresh20 = from;
                            from = from.offset(1);
                            let fresh21 = out;
                            out = out.offset(1);
                            *fresh21 = *fresh20;
                            len = len.wrapping_sub(3 as libc::c_int as libc::c_uint)
                        }
                        if len != 0 {
                            let fresh22 = from;
                            from = from.offset(1);
                            let fresh23 = out;
                            out = out.offset(1);
                            *fresh23 = *fresh22;
                            if len > 1 as libc::c_int as libc::c_uint {
                                let fresh24 = from;
                                from = from.offset(1);
                                let fresh25 = out;
                                out = out.offset(1);
                                *fresh25 = *fresh24
                            }
                        }
                    }
                }
            }
            5832582820025303349 => {
                (*strm).msg = b"invalid literal/length code\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                (*state).mode = BAD;
                break;
            }
            4976922244085895320 =>
            /* end-of-block */
            {
                (*state).mode = TYPE;
                break;
            }
            _ => {}
        }
        if !(in_0 < last && out < end) {
            break;
        }
    }
    /* return unused bytes (on entry, bits < 8, so in won't go too far back) */
    len = bits >> 3 as libc::c_int;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as libc::c_int);
    hold &= ((1 as libc::c_uint) << bits).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as libc::c_ulong;
    /* update state and return */
    (*strm).next_in = in_0;
    (*strm).next_out = out;
    (*strm).avail_in = if in_0 < last {
        (5 as libc::c_int as libc::c_long) + last.offset_from(in_0) as libc::c_long
    } else {
        (5 as libc::c_int as libc::c_long) - in_0.offset_from(last) as libc::c_long
    } as libc::c_uint;
    (*strm).avail_out = if out < end {
        (257 as libc::c_int as libc::c_long) + end.offset_from(out) as libc::c_long
    } else {
        (257 as libc::c_int as libc::c_long) - out.offset_from(end) as libc::c_long
    } as libc::c_uint;
    (*state).hold = hold;
    (*state).bits = bits;
}
/* !ASMINF */
/*
  inflate_fast() speedups that turned out slower (on a PowerPC G3 750CXe):
  - Using bit fields for code structure
  - Different op definition to avoid & for extra bits (do & for table bits)
  - Three separate decoding do-loops for direct, window, and wnext == 0
  - Special case for distance > 1 copies to do overlapped load and store copy
  - Explicit branch predictions (based on measured branch probabilities)
  - Deferring match copy and interspersed it with decoding subsequent codes
  - Swapping literal/length else
  - Swapping window/direct else
  - Larger unrolled copy loops (three is about right)
  - Moving len -= 3 statement into middle of loop
*/
