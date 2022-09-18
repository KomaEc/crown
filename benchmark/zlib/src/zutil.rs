use ::libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
/* 8 bits */
pub type uInt = libc::c_uint;
/* 16 bits or more */
pub type uLong = libc::c_ulong;
pub type voidpf = *mut libc::c_void;
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
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
pub const Z_NEED_DICT: libc::c_int = 2 as libc::c_int;
/* zutil.c -- target dependent utility functions for the compression library
 * Copyright (C) 1995-2017 Jean-loup Gailly
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */
#[no_mangle]
pub static mut z_errmsg: [*mut libc::c_char; 10] = [
    b"need dictionary\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stream end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"file error\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stream error\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"data error\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"insufficient memory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"buffer error\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"incompatible version\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn zlibVersion() -> *const libc::c_char {
    return ZLIB_VERSION.as_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn zlibCompileFlags() -> uLong {
    let mut flags: uLong = 0;
    flags = 0 as libc::c_int as uLong;
    match ::std::mem::size_of::<uInt>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as uLong as uLong
        }
        8 => {
            flags = (flags as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as uLong as uLong
        }
        _ => {
            flags = (flags as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong)
                as uLong as uLong
        }
    }
    match ::std::mem::size_of::<uLong>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
        8 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((2 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
        _ => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((3 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
    }
    match ::std::mem::size_of::<voidpf>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
        8 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((2 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
        _ => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((3 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
    }
    match ::std::mem::size_of::<off_t>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
        8 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((2 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
        _ => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((3 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
                as uLong as uLong
        }
    }
    return flags;
}
/* Z_SOLO */
/* !Z_SOLO */
/* undocumented functions */
/* exported to allow conversion of error code to string for compress() and
 * uncompress()
 */
#[no_mangle]
pub unsafe extern "C" fn zError(mut err: libc::c_int) -> *const libc::c_char {
    return z_errmsg[(Z_NEED_DICT - err) as usize];
}
/* SYS16BIT */
/* Any system without a special alloc function */
#[no_mangle]
pub unsafe extern "C" fn zcalloc(
    mut opaque: voidpf,
    mut items: libc::c_uint,
    mut size: libc::c_uint,
) -> voidpf {
    return if ::std::mem::size_of::<uInt>() as libc::c_ulong > 2 as libc::c_int as libc::c_ulong {
        malloc(items.wrapping_mul(size) as libc::c_ulong)
    } else {
        calloc(items as libc::c_ulong, size as libc::c_ulong)
    };
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
/* Diagnostic functions */
#[no_mangle]
pub unsafe extern "C" fn zcfree(mut opaque: voidpf, mut ptr: voidpf) {
    free(ptr);
}
/* !Z_SOLO */
/* MY_ZCALLOC */
