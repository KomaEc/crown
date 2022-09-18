use ::libc;
pub type size_t = libc::c_ulong;
pub type z_size_t = size_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
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
/* Return codes for the compression/decompression functions. Negative values
 * are errors, positive values are used for special but normal events.
 */
/* compression levels */
/* compression strategy; see deflateInit2() below for details */
/* for compatibility with 1.2.2 and earlier */
/* Possible values of the data_type field for deflate() */
/* The deflate compression method (the only one supported in this version) */
pub const Z_NULL: libc::c_int = 0 as libc::c_int;
pub const BASE: libc::c_uint = 65521 as libc::c_uint;
/* largest prime smaller than 65536 */
pub const NMAX: libc::c_int = 5552 as libc::c_int;
/* NMAX is the largest n such that 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1 */
/* use NO_DIVIDE if your processor does not do division in hardware --
try it both ways to see which is faster */
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn adler32_z(
    mut adler: uLong,
    mut buf: *const Bytef,
    mut len: z_size_t,
) -> uLong {
    let mut sum2: libc::c_ulong = 0;
    let mut n: libc::c_uint = 0;
    /* split Adler-32 into component sums */
    sum2 = adler >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong;
    adler &= 0xffff as libc::c_int as libc::c_ulong;
    /* in case user likes doing a byte at a time, keep it fast */
    if len == 1 as libc::c_int as libc::c_ulong {
        adler = (adler as libc::c_ulong)
            .wrapping_add(*buf.offset(0 as libc::c_int as isize) as libc::c_ulong)
            as uLong as uLong;
        if adler >= BASE as libc::c_ulong {
            adler = (adler as libc::c_ulong).wrapping_sub(BASE as libc::c_ulong) as uLong as uLong
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= BASE as libc::c_ulong {
            sum2 = sum2.wrapping_sub(BASE as libc::c_ulong)
        }
        return adler | sum2 << 16 as libc::c_int;
    }
    /* initial Adler-32 value (deferred check for len == 1 speed) */
    if buf.is_null() {
        return 1 as libc::c_long as uLong;
    }
    /* in case short lengths are provided, keep it somewhat fast */
    if len < 16 as libc::c_int as libc::c_ulong {
        loop {
            let fresh0 = len; /* only added so many BASE's */
            len = len.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            let fresh1 = buf;
            buf = buf.offset(1);
            adler =
                (adler as libc::c_ulong).wrapping_add(*fresh1 as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler)
        }
        if adler >= BASE as libc::c_ulong {
            adler = (adler as libc::c_ulong).wrapping_sub(BASE as libc::c_ulong) as uLong as uLong
        }
        sum2 = sum2.wrapping_rem(BASE as libc::c_ulong);
        return adler | sum2 << 16 as libc::c_int;
    }
    /* do length NMAX blocks -- requires just one modulo operation */
    while len >= NMAX as libc::c_ulong {
        len = (len as libc::c_ulong).wrapping_sub(NMAX as libc::c_ulong) as z_size_t as z_size_t; /* NMAX is divisible by 16 */
        n = (NMAX / 16 as libc::c_int) as libc::c_uint;
        loop {
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(0 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((0 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((0 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((0 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(*buf.offset(
                (0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                    as isize,
            ) as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(8 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((8 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((8 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((8 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((8 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((8 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(*buf.offset(
                (8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                    as isize,
            ) as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            /* 16 sums unrolled */
            buf = buf.offset(16 as libc::c_int as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) {
                break;
            }
        }
        adler = (adler as libc::c_ulong).wrapping_rem(BASE as libc::c_ulong) as uLong as uLong;
        sum2 = sum2.wrapping_rem(BASE as libc::c_ulong)
    }
    /* do remaining bytes (less than NMAX, still just one modulo) */
    if len != 0 {
        /* avoid modulos if none remaining */
        while len >= 16 as libc::c_int as libc::c_ulong {
            len = (len as libc::c_ulong).wrapping_sub(16 as libc::c_int as libc::c_ulong)
                as z_size_t as z_size_t;
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(0 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((0 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((0 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((0 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((0 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((0 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(*buf.offset(
                (0 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                    as isize,
            ) as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong)
                .wrapping_add(*buf.offset(8 as libc::c_int as isize) as libc::c_ulong)
                as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((8 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((8 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((8 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as libc::c_ulong)
                    .wrapping_add(*buf.offset((8 as libc::c_int + 4 as libc::c_int) as isize)
                        as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((8 as libc::c_int + 4 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(
                *buf.offset((8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_ulong,
            ) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler = (adler as libc::c_ulong).wrapping_add(*buf.offset(
                (8 as libc::c_int + 4 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int)
                    as isize,
            ) as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler);
            buf = buf.offset(16 as libc::c_int as isize)
        }
        loop {
            let fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            let fresh3 = buf;
            buf = buf.offset(1);
            adler =
                (adler as libc::c_ulong).wrapping_add(*fresh3 as libc::c_ulong) as uLong as uLong;
            sum2 = sum2.wrapping_add(adler)
        }
        adler = (adler as libc::c_ulong).wrapping_rem(BASE as libc::c_ulong) as uLong as uLong;
        sum2 = sum2.wrapping_rem(BASE as libc::c_ulong)
    }
    /* return recombined sums */
    return adler | sum2 << 16 as libc::c_int;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn adler32(mut adler: uLong, mut buf: *const Bytef, mut len: uInt) -> uLong {
    return adler32_z(adler, buf, len as z_size_t);
}
/* adler32.c -- compute the Adler-32 checksum of a data stream
 * Copyright (C) 1995-2011, 2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* ========================================================================= */
unsafe extern "C" fn adler32_combine_(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: off64_t,
) -> uLong {
    let mut sum1: libc::c_ulong = 0;
    let mut sum2: libc::c_ulong = 0;
    let mut rem: libc::c_uint = 0;
    /* for negative len, return invalid adler32 as a clue for debugging */
    if len2 < 0 as libc::c_int as libc::c_long {
        return 0xffffffff as libc::c_ulong;
    }
    /* the derivation of this formula is left as an exercise for the reader */
    len2 %= BASE as libc::c_long; /* assumes len2 >= 0 */
    rem = len2 as libc::c_uint;
    sum1 = adler1 & 0xffff as libc::c_int as libc::c_ulong;
    sum2 = (rem as libc::c_ulong).wrapping_mul(sum1);
    sum2 = sum2.wrapping_rem(BASE as libc::c_ulong);
    sum1 = sum1.wrapping_add(
        (adler2 & 0xffff as libc::c_int as libc::c_ulong)
            .wrapping_add(BASE as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    sum2 = sum2.wrapping_add(
        (adler1 >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong)
            .wrapping_add(adler2 >> 16 as libc::c_int & 0xffff as libc::c_int as libc::c_ulong)
            .wrapping_add(BASE as libc::c_ulong)
            .wrapping_sub(rem as libc::c_ulong),
    );
    if sum1 >= BASE as libc::c_ulong {
        sum1 = sum1.wrapping_sub(BASE as libc::c_ulong)
    }
    if sum1 >= BASE as libc::c_ulong {
        sum1 = sum1.wrapping_sub(BASE as libc::c_ulong)
    }
    if sum2 >= (BASE as libc::c_ulong) << 1 as libc::c_int {
        sum2 = sum2.wrapping_sub((BASE as libc::c_ulong) << 1 as libc::c_int)
    }
    if sum2 >= BASE as libc::c_ulong {
        sum2 = sum2.wrapping_sub(BASE as libc::c_ulong)
    }
    return sum1 | sum2 << 16 as libc::c_int;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn adler32_combine(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: off_t,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
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
#[no_mangle]
pub unsafe extern "C" fn adler32_combine64(
    mut adler1: uLong,
    mut adler2: uLong,
    mut len2: off64_t,
) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
