
pub type size_t = std::os::raw::c_ulong;
pub type z_size_t = size_t;
pub type Byte = std::os::raw::c_uchar;
pub type uInt = std::os::raw::c_uint;
pub type uLong = std::os::raw::c_ulong;
pub type Bytef = Byte;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
/* NMAX is the largest n such that 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1 */
/* use NO_DIVIDE if your processor does not do division in hardware --
   try it both ways to see which is faster */
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn adler32_z(mut adler: uLong, mut buf: *const Bytef,
                                   mut len: z_size_t) -> uLong {
    let mut sum2: std::os::raw::c_ulong = 0;
    let mut n: std::os::raw::c_uint = 0;
    /* split Adler-32 into component sums */
    sum2 =
        adler >> 16 as std::os::raw::c_int & 0xffff as std::os::raw::c_int as std::os::raw::c_ulong;
    adler &= 0xffff as std::os::raw::c_int as std::os::raw::c_ulong;
    /* in case user likes doing a byte at a time, keep it fast */
    if len == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        adler =
            (adler as
                 std::os::raw::c_ulong).wrapping_add(*buf.offset(0 as std::os::raw::c_int as
                                                             isize) as
                                                 std::os::raw::c_ulong) as uLong as
                uLong;
        if adler >= 65521 as std::os::raw::c_uint as std::os::raw::c_ulong {
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_sub(65521 as std::os::raw::c_uint as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        sum2 = sum2.wrapping_add(adler);
        if sum2 >= 65521 as std::os::raw::c_uint as std::os::raw::c_ulong {
            sum2 = sum2.wrapping_sub(65521 as std::os::raw::c_uint as std::os::raw::c_ulong)
        }
        return adler | sum2 << 16 as std::os::raw::c_int
    }
    /* initial Adler-32 value (deferred check for len == 1 speed) */
    if buf.is_null() { return 1 as std::os::raw::c_long as uLong }
    /* in case short lengths are provided, keep it somewhat fast */
    if len < 16 as std::os::raw::c_int as std::os::raw::c_ulong {
        loop  {
            let fresh0 = len; /* only added so many BASE's */
            len = len.wrapping_sub(1);
            if !(fresh0 != 0) { break ; }
            let fresh1 = buf;
            buf = buf.offset(1);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*fresh1 as std::os::raw::c_ulong) as
                    uLong as uLong;
            sum2 = sum2.wrapping_add(adler)
        }
        if adler >= 65521 as std::os::raw::c_uint as std::os::raw::c_ulong {
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_sub(65521 as std::os::raw::c_uint as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong
        }
        sum2 = sum2.wrapping_rem(65521 as std::os::raw::c_uint as std::os::raw::c_ulong);
        return adler | sum2 << 16 as std::os::raw::c_int
    }
    /* do length NMAX blocks -- requires just one modulo operation */
    while len >= 5552 as std::os::raw::c_int as std::os::raw::c_ulong {
        len =
            (len as
                 std::os::raw::c_ulong).wrapping_sub(5552 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as z_size_t as
                z_size_t; /* NMAX is divisible by 16 */
        n = (5552 as std::os::raw::c_int / 16 as std::os::raw::c_int) as std::os::raw::c_uint;
        loop  {
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset(0 as std::os::raw::c_int
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset(8 as std::os::raw::c_int
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            /* 16 sums unrolled */
            buf = buf.offset(16 as std::os::raw::c_int as isize);
            n = n.wrapping_sub(1);
            if !(n != 0) { break ; }
        }
        adler =
            (adler as
                 std::os::raw::c_ulong).wrapping_rem(65521 as std::os::raw::c_uint as
                                                 std::os::raw::c_ulong) as uLong as
                uLong;
        sum2 = sum2.wrapping_rem(65521 as std::os::raw::c_uint as std::os::raw::c_ulong)
    }
    /* do remaining bytes (less than NMAX, still just one modulo) */
    if len != 0 {
        /* avoid modulos if none remaining */
        while len >= 16 as std::os::raw::c_int as std::os::raw::c_ulong {
            len =
                (len as
                     std::os::raw::c_ulong).wrapping_sub(16 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as
                    z_size_t as z_size_t;
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset(0 as std::os::raw::c_int
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((0 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset(8 as std::os::raw::c_int
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*buf.offset((8 as std::os::raw::c_int
                                                                  +
                                                                  4 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  +
                                                                  1 as
                                                                      std::os::raw::c_int)
                                                                 as isize) as
                                                     std::os::raw::c_ulong) as uLong
                    as uLong;
            sum2 = sum2.wrapping_add(adler);
            buf = buf.offset(16 as std::os::raw::c_int as isize)
        }
        loop  {
            let fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) { break ; }
            let fresh3 = buf;
            buf = buf.offset(1);
            adler =
                (adler as
                     std::os::raw::c_ulong).wrapping_add(*fresh3 as std::os::raw::c_ulong) as
                    uLong as uLong;
            sum2 = sum2.wrapping_add(adler)
        }
        adler =
            (adler as
                 std::os::raw::c_ulong).wrapping_rem(65521 as std::os::raw::c_uint as
                                                 std::os::raw::c_ulong) as uLong as
                uLong;
        sum2 = sum2.wrapping_rem(65521 as std::os::raw::c_uint as std::os::raw::c_ulong)
    }
    /* return recombined sums */
    return adler | sum2 << 16 as std::os::raw::c_int;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn adler32(mut adler: uLong, mut buf: *const Bytef,
                                 mut len: uInt) -> uLong {
    return adler32_z(adler, buf, len as z_size_t);
}
/* adler32.c -- compute the Adler-32 checksum of a data stream
 * Copyright (C) 1995-2011, 2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
/* ========================================================================= */
unsafe extern "C" fn adler32_combine_(mut adler1: uLong, mut adler2: uLong,
                                      mut len2: off64_t) -> uLong {
    let mut sum1: std::os::raw::c_ulong = 0;
    let mut sum2: std::os::raw::c_ulong = 0;
    let mut rem: std::os::raw::c_uint = 0;
    /* for negative len, return invalid adler32 as a clue for debugging */
    if len2 < 0 as std::os::raw::c_int as std::os::raw::c_long {
        return 0xffffffff as std::os::raw::c_ulong
    }
    /* the derivation of this formula is left as an exercise for the reader */
    len2 %= 65521 as std::os::raw::c_uint as std::os::raw::c_long; /* assumes len2 >= 0 */
    rem = len2 as std::os::raw::c_uint;
    sum1 = adler1 & 0xffff as std::os::raw::c_int as std::os::raw::c_ulong;
    sum2 = (rem as std::os::raw::c_ulong).wrapping_mul(sum1);
    sum2 = sum2.wrapping_rem(65521 as std::os::raw::c_uint as std::os::raw::c_ulong);
    sum1 =
        sum1.wrapping_add((adler2 &
                               0xffff as std::os::raw::c_int as
                                   std::os::raw::c_ulong).wrapping_add(65521 as
                                                                   std::os::raw::c_uint
                                                                   as
                                                                   std::os::raw::c_ulong).wrapping_sub(1
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                                   as
                                                                                                   std::os::raw::c_ulong));
    sum2 =
        sum2.wrapping_add((adler1 >> 16 as std::os::raw::c_int &
                               0xffff as std::os::raw::c_int as
                                   std::os::raw::c_ulong).wrapping_add(adler2 >>
                                                                   16 as
                                                                       std::os::raw::c_int
                                                                   &
                                                                   0xffff as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong).wrapping_add(65521
                                                                                                       as
                                                                                                       std::os::raw::c_uint
                                                                                                       as
                                                                                                       std::os::raw::c_ulong).wrapping_sub(rem
                                                                                                                                       as
                                                                                                                                       std::os::raw::c_ulong));
    if sum1 >= 65521 as std::os::raw::c_uint as std::os::raw::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as std::os::raw::c_uint as std::os::raw::c_ulong)
    }
    if sum1 >= 65521 as std::os::raw::c_uint as std::os::raw::c_ulong {
        sum1 = sum1.wrapping_sub(65521 as std::os::raw::c_uint as std::os::raw::c_ulong)
    }
    if sum2 >= (65521 as std::os::raw::c_uint as std::os::raw::c_ulong) << 1 as std::os::raw::c_int {
        sum2 =
            sum2.wrapping_sub((65521 as std::os::raw::c_uint as std::os::raw::c_ulong) <<
                                  1 as std::os::raw::c_int)
    }
    if sum2 >= 65521 as std::os::raw::c_uint as std::os::raw::c_ulong {
        sum2 = sum2.wrapping_sub(65521 as std::os::raw::c_uint as std::os::raw::c_ulong)
    }
    return sum1 | sum2 << 16 as std::os::raw::c_int;
}
/* ========================================================================= */
#[no_mangle]
pub unsafe extern "C" fn adler32_combine(mut adler1: uLong, mut adler2: uLong,
                                         mut len2: off_t) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
#[no_mangle]
pub unsafe extern "C" fn adler32_combine64(mut adler1: uLong,
                                           mut adler2: uLong,
                                           mut len2: off64_t) -> uLong {
    return adler32_combine_(adler1, adler2, len2);
}
