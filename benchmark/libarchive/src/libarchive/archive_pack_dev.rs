use ::libc;
extern "C" {
    #[no_mangle]
    fn gnu_dev_major(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn gnu_dev_minor(__dev: __dev_t) -> libc::c_uint;
    #[no_mangle]
    fn gnu_dev_makedev(__major: libc::c_uint, __minor: libc::c_uint) -> __dev_t;
    #[no_mangle]
    fn bsearch(
        __key: *const libc::c_void,
        __base: *const libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type size_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
/*	$NetBSD: pack_dev.h,v 1.8 2013/06/14 16:28:20 tsutsui Exp $	*/
/*-
 * Copyright (c) 1998, 2001 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Charles M. Hannum.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/* Originally from NetBSD's mknod(8) source. */
pub type pack_t = unsafe extern "C" fn(
    _: libc::c_int,
    _: *mut libc::c_ulong,
    _: *mut *const libc::c_char,
) -> dev_t;
/* list of formats and pack functions */
/* this list must be sorted lexically */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format {
    pub name: *const libc::c_char,
    pub pack: Option<pack_t>,
}
static mut iMajorError: [libc::c_char; 21] = unsafe {
    *::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"invalid major number\x00")
};
static mut iMinorError: [libc::c_char; 21] = unsafe {
    *::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"invalid minor number\x00")
};
static mut tooManyFields: [libc::c_char; 27] = unsafe {
    *::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(b"too many fields for format\x00")
};
/* This is blatantly stolen from libarchive/archive_entry.c,
 * in an attempt to get this to play nice on MinGW... */
/* Play games to come up with a suitable makedev() definition. */
/* There's a "makedev" macro. */
/* exported */
#[no_mangle]
pub unsafe extern "C" fn pack_native(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = gnu_dev_makedev(
            *numbers.offset(0 as libc::c_int as isize) as libc::c_uint,
            *numbers.offset(1 as libc::c_int as isize) as libc::c_uint,
        );
        if gnu_dev_major(dev) as libc::c_ulong != *numbers.offset(0 as libc::c_int as isize) {
            *error = iMajorError.as_ptr()
        } else if gnu_dev_minor(dev) as libc::c_ulong != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
/* not lint */
unsafe extern "C" fn pack_netbsd(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 8 as libc::c_int
            & 0xfff00 as libc::c_int as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 12 as libc::c_int
                & 0xfff00000 as libc::c_uint as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 0 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong;
        if ((dev & 0xfff00 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        } else if ((dev & 0xfff00000 as libc::c_uint as libc::c_ulong) >> 12 as libc::c_int
            | (dev & 0xff as libc::c_int as libc::c_ulong) >> 0 as libc::c_int)
            as int32_t as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
unsafe extern "C" fn pack_freebsd(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 8 as libc::c_int
            & 0xff00 as libc::c_int as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 0 as libc::c_int
                & 0xffff00ff as libc::c_uint as libc::c_ulong;
        if ((dev & 0xff00 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        }
        if ((dev & 0xffff00ff as libc::c_uint as libc::c_ulong) >> 0 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
unsafe extern "C" fn pack_8_8(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 8 as libc::c_int
            & 0xff00 as libc::c_int as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 0 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong;
        if ((dev & 0xff00 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        }
        if ((dev & 0xff as libc::c_int as libc::c_ulong) >> 0 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
unsafe extern "C" fn pack_12_20(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 20 as libc::c_int
            & 0xfff00000 as libc::c_uint as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 0 as libc::c_int
                & 0xfffff as libc::c_int as libc::c_ulong;
        if ((dev & 0xfff00000 as libc::c_uint as libc::c_ulong) >> 20 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        }
        if ((dev & 0xfffff as libc::c_int as libc::c_ulong) >> 0 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
unsafe extern "C" fn pack_14_18(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 18 as libc::c_int
            & 0xfffc0000 as libc::c_uint as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 0 as libc::c_int
                & 0x3ffff as libc::c_int as libc::c_ulong;
        if ((dev & 0xfffc0000 as libc::c_uint as libc::c_ulong) >> 18 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        }
        if ((dev & 0x3ffff as libc::c_int as libc::c_ulong) >> 0 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
unsafe extern "C" fn pack_8_24(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 24 as libc::c_int
            & 0xff000000 as libc::c_uint as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 0 as libc::c_int
                & 0xffffff as libc::c_int as libc::c_ulong;
        if ((dev & 0xff000000 as libc::c_uint as libc::c_ulong) >> 24 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        }
        if ((dev & 0xffffff as libc::c_int as libc::c_ulong) >> 0 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
unsafe extern "C" fn pack_bsdos(
    mut n: libc::c_int,
    mut numbers: *mut libc::c_ulong,
    mut error: *mut *const libc::c_char,
) -> dev_t {
    let mut dev: dev_t = 0 as libc::c_int as dev_t;
    if n == 2 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 20 as libc::c_int
            & 0xfff00000 as libc::c_uint as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 0 as libc::c_int
                & 0xfffff as libc::c_int as libc::c_ulong;
        if ((dev & 0xfff00000 as libc::c_uint as libc::c_ulong) >> 20 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        }
        if ((dev & 0xfffff as libc::c_int as libc::c_ulong) >> 0 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = iMinorError.as_ptr()
        }
    } else if n == 3 as libc::c_int {
        dev = *numbers.offset(0 as libc::c_int as isize) << 20 as libc::c_int
            & 0xfff00000 as libc::c_uint as libc::c_ulong
            | *numbers.offset(1 as libc::c_int as isize) << 8 as libc::c_int
                & 0xfff00 as libc::c_int as libc::c_ulong
            | *numbers.offset(2 as libc::c_int as isize) << 0 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong;
        if ((dev & 0xfff00000 as libc::c_uint as libc::c_ulong) >> 20 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(0 as libc::c_int as isize)
        {
            *error = iMajorError.as_ptr()
        }
        if ((dev & 0xfff00 as libc::c_int as libc::c_ulong) >> 8 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(1 as libc::c_int as isize)
        {
            *error = b"invalid unit number\x00" as *const u8 as *const libc::c_char
        }
        if ((dev & 0xff as libc::c_int as libc::c_ulong) >> 0 as libc::c_int) as int32_t
            as libc::c_ulong
            != *numbers.offset(2 as libc::c_int as isize)
        {
            *error = b"invalid subunit number\x00" as *const u8 as *const libc::c_char
        }
    } else {
        *error = tooManyFields.as_ptr()
    }
    return dev;
}
static mut formats: [format; 16] = unsafe {
    [
        {
            let mut init = format {
                name: b"386bsd\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"4bsd\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"bsdos\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_bsdos as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"freebsd\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_freebsd as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"hpux\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_24 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"isc\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"linux\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"native\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_native as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"netbsd\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_netbsd as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"osf1\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_12_20 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"sco\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"solaris\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_14_18 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"sunos\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"svr3\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"svr4\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_14_18 as pack_t),
            };
            init
        },
        {
            let mut init = format {
                name: b"ultrix\x00" as *const u8 as *const libc::c_char,
                pack: Some(pack_8_8 as pack_t),
            };
            init
        },
    ]
};
unsafe extern "C" fn compare_format(
    mut key: *const libc::c_void,
    mut element: *const libc::c_void,
) -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut format: *const format = 0 as *const format;
    name = key as *const libc::c_char;
    format = element as *const format;
    return strcmp(name, (*format).name);
}
#[no_mangle]
pub unsafe extern "C" fn pack_find(mut name: *const libc::c_char) -> Option<pack_t> {
    let mut format: *mut format = 0 as *mut format;
    format = bsearch(
        name as *const libc::c_void,
        formats.as_ptr() as *const libc::c_void,
        (::std::mem::size_of::<[format; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<format>() as libc::c_ulong),
        ::std::mem::size_of::<format>() as libc::c_ulong,
        Some(
            compare_format
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut format;
    if format.is_null() {
        return None;
    }
    return (*format).pack;
}
