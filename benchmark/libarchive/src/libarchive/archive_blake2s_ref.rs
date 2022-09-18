use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
/*
   BLAKE2 reference source code package - reference C implementations

   Copyright 2012, Samuel Neves <sneves@dei.uc.pt>.  You may use this under the
   terms of the CC0, the OpenSSL Licence, or the Apache Public License 2.0, at
   your option.  The terms of these licenses can be found at:

   - CC0 1.0 Universal : http://creativecommons.org/publicdomain/zero/1.0
   - OpenSSL license   : https://www.openssl.org/source/license.html
   - Apache 2.0        : http://www.apache.org/licenses/LICENSE-2.0

   More information about the BLAKE2 hash function can be found at
   https://blake2.net.
*/
pub type blake2s_constant = libc::c_uint;
pub const BLAKE2S_PERSONALBYTES: blake2s_constant = 8;
pub const BLAKE2S_SALTBYTES: blake2s_constant = 8;
pub const BLAKE2S_KEYBYTES: blake2s_constant = 32;
pub const BLAKE2S_OUTBYTES: blake2s_constant = 32;
pub const BLAKE2S_BLOCKBYTES: blake2s_constant = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blake2s_state__ {
    pub h: [uint32_t; 8],
    pub t: [uint32_t; 2],
    pub f: [uint32_t; 2],
    pub buf: [uint8_t; 64],
    pub buflen: size_t,
    pub outlen: size_t,
    pub last_node: uint8_t,
}
pub type blake2s_state = blake2s_state__;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct blake2s_param__ {
    pub digest_length: uint8_t,
    pub key_length: uint8_t,
    pub fanout: uint8_t,
    pub depth: uint8_t,
    pub leaf_length: uint32_t,
    pub node_offset: uint32_t,
    pub xof_length: uint16_t,
    pub node_depth: uint8_t,
    pub inner_length: uint8_t,
    pub salt: [uint8_t; 8],
    pub personal: [uint8_t; 8],
}
/* 32 */
pub type blake2s_param = blake2s_param__;
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
   BLAKE2 reference source code package - reference C implementations

   Copyright 2012, Samuel Neves <sneves@dei.uc.pt>.  You may use this under the
   terms of the CC0, the OpenSSL Licence, or the Apache Public License 2.0, at
   your option.  The terms of these licenses can be found at:

   - CC0 1.0 Universal : http://creativecommons.org/publicdomain/zero/1.0
   - OpenSSL license   : https://www.openssl.org/source/license.html
   - Apache 2.0        : http://www.apache.org/licenses/LICENSE-2.0

   More information about the BLAKE2 hash function can be found at
   https://blake2.net.
*/
#[inline]
unsafe extern "C" fn load32(mut src: *const libc::c_void) -> uint32_t {
    let mut p: *const uint8_t = src as *const uint8_t;
    return (*p.offset(0 as libc::c_int as isize) as uint32_t) << 0 as libc::c_int
        | (*p.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | (*p.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*p.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn store16(mut dst: *mut libc::c_void, mut w: uint16_t) {
    let mut p: *mut uint8_t = dst as *mut uint8_t;
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = w as uint8_t;
    w = (w as libc::c_int >> 8 as libc::c_int) as uint16_t;
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = w as uint8_t;
}
#[inline]
unsafe extern "C" fn store32(mut dst: *mut libc::c_void, mut w: uint32_t) {
    let mut p: *mut uint8_t = dst as *mut uint8_t;
    *p.offset(0 as libc::c_int as isize) = (w >> 0 as libc::c_int) as uint8_t;
    *p.offset(1 as libc::c_int as isize) = (w >> 8 as libc::c_int) as uint8_t;
    *p.offset(2 as libc::c_int as isize) = (w >> 16 as libc::c_int) as uint8_t;
    *p.offset(3 as libc::c_int as isize) = (w >> 24 as libc::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn rotr32(w: uint32_t, c: libc::c_uint) -> uint32_t {
    return w >> c | w << (32 as libc::c_int as libc::c_uint).wrapping_sub(c);
}
/* prevents compiler optimizing out memset() */
#[inline]
unsafe extern "C" fn secure_zero_memory(mut v: *mut libc::c_void, mut n: size_t) {
    static mut memset_v: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int, _: size_t) -> *mut libc::c_void,
    > = Some(
        memset
            as unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: libc::c_int,
                _: libc::c_ulong,
            ) -> *mut libc::c_void,
    );
    memset_v.expect("non-null function pointer")(v, 0 as libc::c_int, n);
}
/*
   BLAKE2 reference source code package - reference C implementations

   Copyright 2012, Samuel Neves <sneves@dei.uc.pt>.  You may use this under the
   terms of the CC0, the OpenSSL Licence, or the Apache Public License 2.0, at
   your option.  The terms of these licenses can be found at:

   - CC0 1.0 Universal : http://creativecommons.org/publicdomain/zero/1.0
   - OpenSSL license   : https://www.openssl.org/source/license.html
   - Apache 2.0        : http://www.apache.org/licenses/LICENSE-2.0

   More information about the BLAKE2 hash function can be found at
   https://blake2.net.
*/
static mut blake2s_IV: [uint32_t; 8] = [
    0x6a09e667 as libc::c_ulong as uint32_t,
    0xbb67ae85 as libc::c_ulong as uint32_t,
    0x3c6ef372 as libc::c_ulong as uint32_t,
    0xa54ff53a as libc::c_ulong as uint32_t,
    0x510e527f as libc::c_ulong as uint32_t,
    0x9b05688c as libc::c_ulong as uint32_t,
    0x1f83d9ab as libc::c_ulong as uint32_t,
    0x5be0cd19 as libc::c_ulong as uint32_t,
];
static mut blake2s_sigma: [[uint8_t; 16]; 10] = [
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
    ],
    [
        14 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
    ],
    [
        11 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
    ],
    [
        7 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
    ],
    [
        9 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
    ],
    [
        2 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
    ],
    [
        12 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
    ],
    [
        13 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
    ],
    [
        6 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
    ],
    [
        10 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ],
];
unsafe extern "C" fn blake2s_set_lastnode(mut S: *mut blake2s_state) {
    (*S).f[1 as libc::c_int as usize] = -(1 as libc::c_int) as uint32_t;
}
/* Some helper functions, not necessarily useful */
unsafe extern "C" fn blake2s_is_lastblock(mut S: *const blake2s_state) -> libc::c_int {
    return ((*S).f[0 as libc::c_int as usize] != 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn blake2s_set_lastblock(mut S: *mut blake2s_state) {
    if (*S).last_node != 0 {
        blake2s_set_lastnode(S);
    }
    (*S).f[0 as libc::c_int as usize] = -(1 as libc::c_int) as uint32_t;
}
unsafe extern "C" fn blake2s_increment_counter(mut S: *mut blake2s_state, inc: uint32_t) {
    (*S).t[0 as libc::c_int as usize] = ((*S).t[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(inc) as uint32_t as uint32_t;
    (*S).t[1 as libc::c_int as usize] = ((*S).t[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(((*S).t[0 as libc::c_int as usize] < inc) as libc::c_int as libc::c_uint)
        as uint32_t as uint32_t;
}
unsafe extern "C" fn blake2s_init0(mut S: *mut blake2s_state) {
    let mut i: size_t = 0;
    memset(
        S as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<blake2s_state>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        (*S).h[i as usize] = blake2s_IV[i as usize];
        i = i.wrapping_add(1)
    }
}
/* init2 xors IV with input parameter block */
#[no_mangle]
pub unsafe extern "C" fn blake2s_init_param(
    mut S: *mut blake2s_state,
    mut P: *const blake2s_param,
) -> libc::c_int {
    let mut p: *const libc::c_uchar = P as *const libc::c_uchar;
    let mut i: size_t = 0;
    blake2s_init0(S);
    /* IV XOR ParamBlock */
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        (*S).h[i as usize] ^= load32(
            &*p.offset(i.wrapping_mul(4 as libc::c_int as libc::c_ulong) as isize)
                as *const libc::c_uchar as *const libc::c_void,
        );
        i = i.wrapping_add(1)
    }
    (*S).outlen = (*P).digest_length as size_t;
    return 0 as libc::c_int;
}
/* Streaming API */
/* Sequential blake2s initialization */
#[no_mangle]
pub unsafe extern "C" fn blake2s_init(
    mut S: *mut blake2s_state,
    mut outlen: size_t,
) -> libc::c_int {
    let mut P: [blake2s_param; 1] = [blake2s_param {
        digest_length: 0,
        key_length: 0,
        fanout: 0,
        depth: 0,
        leaf_length: 0,
        node_offset: 0,
        xof_length: 0,
        node_depth: 0,
        inner_length: 0,
        salt: [0; 8],
        personal: [0; 8],
    }; 1];
    /* Move interval verification here? */
    if outlen == 0 || outlen > BLAKE2S_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*P.as_mut_ptr()).digest_length = outlen as uint8_t;
    (*P.as_mut_ptr()).key_length = 0 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).fanout = 1 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).depth = 1 as libc::c_int as uint8_t;
    store32(
        &mut (*P.as_mut_ptr()).leaf_length as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store32(
        &mut (*P.as_mut_ptr()).node_offset as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store16(
        &mut (*P.as_mut_ptr()).xof_length as *mut uint16_t as *mut libc::c_void,
        0 as libc::c_int as uint16_t,
    );
    (*P.as_mut_ptr()).node_depth = 0 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).inner_length = 0 as libc::c_int as uint8_t;
    /* memset(P->reserved, 0, sizeof(P->reserved) ); */
    memset(
        (*P.as_mut_ptr()).salt.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    memset(
        (*P.as_mut_ptr()).personal.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    return blake2s_init_param(S, P.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn blake2s_init_key(
    mut S: *mut blake2s_state,
    mut outlen: size_t,
    mut key: *const libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    let mut P: [blake2s_param; 1] = [blake2s_param {
        digest_length: 0,
        key_length: 0,
        fanout: 0,
        depth: 0,
        leaf_length: 0,
        node_offset: 0,
        xof_length: 0,
        node_depth: 0,
        inner_length: 0,
        salt: [0; 8],
        personal: [0; 8],
    }; 1];
    if outlen == 0 || outlen > BLAKE2S_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if key.is_null() || keylen == 0 || keylen > BLAKE2S_KEYBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    (*P.as_mut_ptr()).digest_length = outlen as uint8_t;
    (*P.as_mut_ptr()).key_length = keylen as uint8_t;
    (*P.as_mut_ptr()).fanout = 1 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).depth = 1 as libc::c_int as uint8_t;
    store32(
        &mut (*P.as_mut_ptr()).leaf_length as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store32(
        &mut (*P.as_mut_ptr()).node_offset as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store16(
        &mut (*P.as_mut_ptr()).xof_length as *mut uint16_t as *mut libc::c_void,
        0 as libc::c_int as uint16_t,
    );
    (*P.as_mut_ptr()).node_depth = 0 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).inner_length = 0 as libc::c_int as uint8_t;
    /* memset(P->reserved, 0, sizeof(P->reserved) ); */
    memset(
        (*P.as_mut_ptr()).salt.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    memset(
        (*P.as_mut_ptr()).personal.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    if blake2s_init_param(S, P.as_mut_ptr()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut block: [uint8_t; 64] = [0; 64];
    memset(
        block.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong,
    );
    memcpy(block.as_mut_ptr() as *mut libc::c_void, key, keylen);
    blake2s_update(
        S,
        block.as_mut_ptr() as *const libc::c_void,
        BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
    );
    secure_zero_memory(
        block.as_mut_ptr() as *mut libc::c_void,
        BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn blake2s_compress(mut S: *mut blake2s_state, mut in_0: *const uint8_t) {
    let mut m: [uint32_t; 16] = [0; 16];
    let mut v: [uint32_t; 16] = [0; 16];
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 16 as libc::c_int as libc::c_ulong {
        m[i as usize] = load32(
            in_0.offset(i.wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong) as isize)
                as *const libc::c_void,
        );
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        v[i as usize] = (*S).h[i as usize];
        i = i.wrapping_add(1)
    }
    v[8 as libc::c_int as usize] = blake2s_IV[0 as libc::c_int as usize];
    v[9 as libc::c_int as usize] = blake2s_IV[1 as libc::c_int as usize];
    v[10 as libc::c_int as usize] = blake2s_IV[2 as libc::c_int as usize];
    v[11 as libc::c_int as usize] = blake2s_IV[3 as libc::c_int as usize];
    v[12 as libc::c_int as usize] =
        (*S).t[0 as libc::c_int as usize] ^ blake2s_IV[4 as libc::c_int as usize];
    v[13 as libc::c_int as usize] =
        (*S).t[1 as libc::c_int as usize] ^ blake2s_IV[5 as libc::c_int as usize];
    v[14 as libc::c_int as usize] =
        (*S).f[0 as libc::c_int as usize] ^ blake2s_IV[6 as libc::c_int as usize];
    v[15 as libc::c_int as usize] =
        (*S).f[1 as libc::c_int as usize] ^ blake2s_IV[7 as libc::c_int as usize];
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[0 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[1 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[2 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[3 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[4 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[5 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[6 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[7 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[8 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 0 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 1 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[0 as libc::c_int as usize] = v[0 as libc::c_int as usize]
        .wrapping_add(v[5 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[15 as libc::c_int as usize] = rotr32(
        v[15 as libc::c_int as usize] ^ v[0 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[10 as libc::c_int as usize] =
        v[10 as libc::c_int as usize].wrapping_add(v[15 as libc::c_int as usize]);
    v[5 as libc::c_int as usize] = rotr32(
        v[5 as libc::c_int as usize] ^ v[10 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[1 as libc::c_int as usize] = v[1 as libc::c_int as usize]
        .wrapping_add(v[6 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[12 as libc::c_int as usize] = rotr32(
        v[12 as libc::c_int as usize] ^ v[1 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[11 as libc::c_int as usize] =
        v[11 as libc::c_int as usize].wrapping_add(v[12 as libc::c_int as usize]);
    v[6 as libc::c_int as usize] = rotr32(
        v[6 as libc::c_int as usize] ^ v[11 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[2 as libc::c_int as usize] = v[2 as libc::c_int as usize]
        .wrapping_add(v[7 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 6 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[13 as libc::c_int as usize] = rotr32(
        v[13 as libc::c_int as usize] ^ v[2 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[8 as libc::c_int as usize] =
        v[8 as libc::c_int as usize].wrapping_add(v[13 as libc::c_int as usize]);
    v[7 as libc::c_int as usize] = rotr32(
        v[7 as libc::c_int as usize] ^ v[8 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 0 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        16 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        12 as libc::c_int as libc::c_uint,
    );
    v[3 as libc::c_int as usize] = v[3 as libc::c_int as usize]
        .wrapping_add(v[4 as libc::c_int as usize])
        .wrapping_add(
            m[blake2s_sigma[9 as libc::c_int as usize]
                [(2 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int) as usize]
                as usize],
        );
    v[14 as libc::c_int as usize] = rotr32(
        v[14 as libc::c_int as usize] ^ v[3 as libc::c_int as usize],
        8 as libc::c_int as libc::c_uint,
    );
    v[9 as libc::c_int as usize] =
        v[9 as libc::c_int as usize].wrapping_add(v[14 as libc::c_int as usize]);
    v[4 as libc::c_int as usize] = rotr32(
        v[4 as libc::c_int as usize] ^ v[9 as libc::c_int as usize],
        7 as libc::c_int as libc::c_uint,
    );
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        (*S).h[i as usize] = (*S).h[i as usize]
            ^ v[i as usize]
            ^ v[i.wrapping_add(8 as libc::c_int as libc::c_ulong) as usize];
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn blake2s_update(
    mut S: *mut blake2s_state,
    mut pin: *const libc::c_void,
    mut inlen: size_t,
) -> libc::c_int {
    let mut in_0: *const libc::c_uchar = pin as *const libc::c_uchar;
    if inlen > 0 as libc::c_int as libc::c_ulong {
        let mut left: size_t = (*S).buflen;
        let mut fill: size_t =
            (BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong).wrapping_sub(left);
        if inlen > fill {
            (*S).buflen = 0 as libc::c_int as size_t;
            /* Burn the key from stack */
            memcpy(
                (*S).buf.as_mut_ptr().offset(left as isize) as *mut libc::c_void,
                in_0 as *const libc::c_void,
                fill,
            ); /* Fill buffer */
            blake2s_increment_counter(S, BLAKE2S_BLOCKBYTES as libc::c_int as uint32_t); /* Compress */
            blake2s_compress(S, (*S).buf.as_mut_ptr() as *const uint8_t); /* Padding */
            in_0 = in_0.offset(fill as isize);
            inlen = (inlen as libc::c_ulong).wrapping_sub(fill) as size_t as size_t;
            while inlen > BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong {
                blake2s_increment_counter(S, BLAKE2S_BLOCKBYTES as libc::c_int as uint32_t);
                blake2s_compress(S, in_0);
                in_0 = in_0.offset(BLAKE2S_BLOCKBYTES as libc::c_int as isize);
                inlen = (inlen as libc::c_ulong)
                    .wrapping_sub(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            }
        }
        memcpy(
            (*S).buf.as_mut_ptr().offset((*S).buflen as isize) as *mut libc::c_void,
            in_0 as *const libc::c_void,
            inlen,
        );
        (*S).buflen = ((*S).buflen as libc::c_ulong).wrapping_add(inlen) as size_t as size_t
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2s_final(
    mut S: *mut blake2s_state,
    mut out: *mut libc::c_void,
    mut outlen: size_t,
) -> libc::c_int {
    let mut buffer: [uint8_t; 32] = [
        0 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: size_t = 0;
    if out.is_null() || outlen < (*S).outlen {
        return -(1 as libc::c_int);
    }
    if blake2s_is_lastblock(S) != 0 {
        return -(1 as libc::c_int);
    }
    blake2s_increment_counter(S, (*S).buflen as uint32_t);
    blake2s_set_lastblock(S);
    memset(
        (*S).buf.as_mut_ptr().offset((*S).buflen as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong).wrapping_sub((*S).buflen),
    );
    blake2s_compress(S, (*S).buf.as_mut_ptr() as *const uint8_t);
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        /* Output full hash to temp buffer */
        store32(
            buffer.as_mut_ptr().offset(
                (::std::mem::size_of::<uint32_t>() as libc::c_ulong).wrapping_mul(i) as isize,
            ) as *mut libc::c_void,
            (*S).h[i as usize],
        );
        i = i.wrapping_add(1)
    }
    memcpy(out, buffer.as_mut_ptr() as *const libc::c_void, outlen);
    secure_zero_memory(
        buffer.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
/* Simple API */
#[no_mangle]
pub unsafe extern "C" fn blake2s(
    mut out: *mut libc::c_void,
    mut outlen: size_t,
    mut in_0: *const libc::c_void,
    mut inlen: size_t,
    mut key: *const libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    let mut S: [blake2s_state; 1] = [blake2s_state {
        h: [0; 8],
        t: [0; 2],
        f: [0; 2],
        buf: [0; 64],
        buflen: 0,
        outlen: 0,
        last_node: 0,
    }; 1];
    /* Verify parameters */
    if NULL as *const libc::c_void == in_0 && inlen > 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if out.is_null() {
        return -(1 as libc::c_int);
    }
    if NULL as *const libc::c_void == key && keylen > 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if outlen == 0 || outlen > BLAKE2S_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if keylen > BLAKE2S_KEYBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if keylen > 0 as libc::c_int as libc::c_ulong {
        if blake2s_init_key(S.as_mut_ptr(), outlen, key, keylen) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else if blake2s_init(S.as_mut_ptr(), outlen) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    blake2s_update(
        S.as_mut_ptr(),
        in_0 as *const uint8_t as *const libc::c_void,
        inlen,
    );
    blake2s_final(S.as_mut_ptr(), out, outlen);
    return 0 as libc::c_int;
}
