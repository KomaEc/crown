use ::libc;
extern "C" {
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
pub type u_char = __u_char;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
/*	$OpenBSD: arc4random.c,v 1.24 2013/06/11 16:59:50 deraadt Exp $	*/
/*
 * Copyright (c) 1996, David Mazieres <dm@uun.org>
 * Copyright (c) 2008, Damien Miller <djm@openbsd.org>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Arc4 random number generator for OpenBSD.
 *
 * This code is derived from section 17.1 of Applied Cryptography,
 * second edition, which describes a stream cipher allegedly
 * compatible with RSA Labs "RC4" cipher (the actual description of
 * which is a trade secret).  The same algorithm is used as a stream
 * cipher called "arcfour" in Tatu Ylonen's ssh package.
 *
 * RC4 is a registered trademark of RSA Laboratories.
 */
/* !__GNUC__ */
/* !__GNUC__ */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arc4_stream {
    pub i: uint8_t,
    pub j: uint8_t,
    pub s: [uint8_t; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tv: timeval,
    pub pid: pid_t,
    pub rnd: [u_char; 128],
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const __O_CLOEXEC: libc::c_int = 0o2000000 as libc::c_int;
pub const O_CLOEXEC: libc::c_int = __O_CLOEXEC;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/*-
 * Copyright (c) 2014 Michihiro NAKAJIMA
 * All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* Random number generator. */
/* HAVE_ARC4RANDOM_BUF */
/*
 * Random number generator function.
 * This simply calls arc4random_buf function if the platform provides it.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_random(
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
) -> libc::c_int {
    arc4random_buf(buf, nbytes);
    return ARCHIVE_OK;
}
pub const RANDOMDEV: [libc::c_char; 13] =
    unsafe { *::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"/dev/urandom\x00") };
pub const KEYSIZE: libc::c_int = 128 as libc::c_int;
static mut arc4random_mtx: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut rs_initialized: libc::c_int = 0;
static mut rs: arc4_stream = arc4_stream {
    i: 0,
    j: 0,
    s: [0; 256],
};
static mut arc4_stir_pid: pid_t = 0;
static mut arc4_count: libc::c_int = 0;
#[inline]
unsafe extern "C" fn arc4_init() {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < 256 as libc::c_int {
        rs.s[n as usize] = n as uint8_t;
        n += 1
    }
    rs.i = 0 as libc::c_int as uint8_t;
    rs.j = 0 as libc::c_int as uint8_t;
}
#[inline]
unsafe extern "C" fn arc4_addrandom(mut dat: *mut u_char, mut datlen: libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut si: uint8_t = 0;
    rs.i = rs.i.wrapping_sub(1);
    n = 0 as libc::c_int;
    while n < 256 as libc::c_int {
        rs.i = (rs.i as libc::c_int + 1 as libc::c_int) as uint8_t;
        si = rs.s[rs.i as usize];
        rs.j = (rs.j as libc::c_int
            + si as libc::c_int
            + *dat.offset((n % datlen) as isize) as libc::c_int) as uint8_t;
        rs.s[rs.i as usize] = rs.s[rs.j as usize];
        rs.s[rs.j as usize] = si;
        n += 1
    }
    rs.j = rs.i;
}
unsafe extern "C" fn arc4_stir() {
    let mut done: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rdat: C2RustUnnamed = C2RustUnnamed {
        tv: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        pid: 0,
        rnd: [0; 128],
    };
    if rs_initialized == 0 {
        arc4_init();
        rs_initialized = 1 as libc::c_int
    }
    done = 0 as libc::c_int;
    fd = open(RANDOMDEV.as_ptr(), O_RDONLY | O_CLOEXEC, 0 as libc::c_int);
    if fd >= 0 as libc::c_int {
        if read(
            fd,
            &mut rdat as *mut C2RustUnnamed as *mut libc::c_void,
            KEYSIZE as size_t,
        ) == KEYSIZE as libc::c_long
        {
            done = 1 as libc::c_int
        }
        close(fd);
    }
    if done == 0 {
        gettimeofday(&mut rdat.tv, NULL as *mut timezone);
        rdat.pid = getpid()
        /* We'll just take whatever was on the stack too... */
    }
    arc4_addrandom(&mut rdat as *mut C2RustUnnamed as *mut u_char, KEYSIZE);
    /*
     * Discard early keystream, as per recommendations in:
     * "(Not So) Random Shuffles of RC4" by Ilya Mironov.
     * As per the Network Operations Division, cryptographic requirements
     * published on wikileaks on March 2017.
     */
    i = 0 as libc::c_int;
    while i < 3072 as libc::c_int {
        arc4_getbyte();
        i += 1
    }
    arc4_count = 1600000 as libc::c_int;
}
unsafe extern "C" fn arc4_stir_if_needed() {
    let mut pid: pid_t = getpid();
    if arc4_count <= 0 as libc::c_int || rs_initialized == 0 || arc4_stir_pid != pid {
        arc4_stir_pid = pid;
        arc4_stir();
    };
}
#[inline]
unsafe extern "C" fn arc4_getbyte() -> uint8_t {
    let mut si: uint8_t = 0;
    let mut sj: uint8_t = 0;
    rs.i = (rs.i as libc::c_int + 1 as libc::c_int) as uint8_t;
    si = rs.s[rs.i as usize];
    rs.j = (rs.j as libc::c_int + si as libc::c_int) as uint8_t;
    sj = rs.s[rs.j as usize];
    rs.s[rs.i as usize] = sj;
    rs.s[rs.j as usize] = si;
    return rs.s[(si as libc::c_int + sj as libc::c_int & 0xff as libc::c_int) as usize];
}
unsafe extern "C" fn arc4random_buf(mut _buf: *mut libc::c_void, mut n: size_t) {
    let mut buf: *mut u_char = _buf as *mut u_char;
    pthread_mutex_lock(&mut arc4random_mtx);
    arc4_stir_if_needed();
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        arc4_count -= 1;
        if arc4_count <= 0 as libc::c_int {
            arc4_stir();
        }
        *buf.offset(n as isize) = arc4_getbyte()
    }
    pthread_mutex_unlock(&mut arc4random_mtx);
}
/* !HAVE_ARC4RANDOM_BUF */
