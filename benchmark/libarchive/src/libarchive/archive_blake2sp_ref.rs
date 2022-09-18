use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn blake2s_init_param(S: *mut blake2s_state, P: *const blake2s_param) -> libc::c_int;
    #[no_mangle]
    fn blake2s_update(
        S: *mut blake2s_state,
        in_0: *const libc::c_void,
        inlen: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn blake2s_final(S: *mut blake2s_state, out: *mut libc::c_void, outlen: size_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[repr(C)]
pub struct blake2sp_state__ {
    pub S: [[blake2s_state; 1]; 8],
    pub R: [blake2s_state; 1],
    pub buf: [uint8_t; 512],
    pub buflen: size_t,
    pub outlen: size_t,
}
pub type blake2sp_state = blake2sp_state__;
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
pub const PARALLELISM_DEGREE: libc::c_int = 8 as libc::c_int;
/*
  blake2sp_init_param defaults to setting the expecting output length
  from the digest_length parameter block field.

  In some cases, however, we do not want this, as the output length
  of these instances is given by inner_length instead.
*/
unsafe extern "C" fn blake2sp_init_leaf_param(
    mut S: *mut blake2s_state,
    mut P: *const blake2s_param,
) -> libc::c_int {
    let mut err: libc::c_int = blake2s_init_param(S, P);
    (*S).outlen = (*P).inner_length as size_t;
    return err;
}
unsafe extern "C" fn blake2sp_init_leaf(
    mut S: *mut blake2s_state,
    mut outlen: size_t,
    mut keylen: size_t,
    mut offset: uint32_t,
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
    (*P.as_mut_ptr()).digest_length = outlen as uint8_t;
    (*P.as_mut_ptr()).key_length = keylen as uint8_t;
    (*P.as_mut_ptr()).fanout = PARALLELISM_DEGREE as uint8_t;
    (*P.as_mut_ptr()).depth = 2 as libc::c_int as uint8_t;
    store32(
        &mut (*P.as_mut_ptr()).leaf_length as *mut uint32_t as *mut libc::c_void,
        0 as libc::c_int as uint32_t,
    );
    store32(
        &mut (*P.as_mut_ptr()).node_offset as *mut uint32_t as *mut libc::c_void,
        offset,
    );
    store16(
        &mut (*P.as_mut_ptr()).xof_length as *mut uint16_t as *mut libc::c_void,
        0 as libc::c_int as uint16_t,
    );
    (*P.as_mut_ptr()).node_depth = 0 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).inner_length = BLAKE2S_OUTBYTES as libc::c_int as uint8_t;
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
    return blake2sp_init_leaf_param(S, P.as_mut_ptr());
}
unsafe extern "C" fn blake2sp_init_root(
    mut S: *mut blake2s_state,
    mut outlen: size_t,
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
    (*P.as_mut_ptr()).digest_length = outlen as uint8_t;
    (*P.as_mut_ptr()).key_length = keylen as uint8_t;
    (*P.as_mut_ptr()).fanout = PARALLELISM_DEGREE as uint8_t;
    (*P.as_mut_ptr()).depth = 2 as libc::c_int as uint8_t;
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
    (*P.as_mut_ptr()).node_depth = 1 as libc::c_int as uint8_t;
    (*P.as_mut_ptr()).inner_length = BLAKE2S_OUTBYTES as libc::c_int as uint8_t;
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
pub unsafe extern "C" fn blake2sp_init(
    mut S: *mut blake2sp_state,
    mut outlen: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if outlen == 0 || outlen > BLAKE2S_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memset(
        (*S).buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 512]>() as libc::c_ulong,
    );
    (*S).buflen = 0 as libc::c_int as size_t;
    (*S).outlen = outlen;
    if blake2sp_init_root((*S).R.as_mut_ptr(), outlen, 0 as libc::c_int as size_t)
        < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        if blake2sp_init_leaf(
            (*S).S[i as usize].as_mut_ptr(),
            outlen,
            0 as libc::c_int as size_t,
            i as uint32_t,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    (*(*S).R.as_mut_ptr()).last_node = 1 as libc::c_int as uint8_t;
    (*(*S).S[(PARALLELISM_DEGREE - 1 as libc::c_int) as usize].as_mut_ptr()).last_node =
        1 as libc::c_int as uint8_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2sp_init_key(
    mut S: *mut blake2sp_state,
    mut outlen: size_t,
    mut key: *const libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    if outlen == 0 || outlen > BLAKE2S_OUTBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    if key.is_null() || keylen == 0 || keylen > BLAKE2S_KEYBYTES as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    memset(
        (*S).buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 512]>() as libc::c_ulong,
    );
    (*S).buflen = 0 as libc::c_int as size_t;
    (*S).outlen = outlen;
    if blake2sp_init_root((*S).R.as_mut_ptr(), outlen, keylen) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        if blake2sp_init_leaf(
            (*S).S[i as usize].as_mut_ptr(),
            outlen,
            keylen,
            i as uint32_t,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    (*(*S).R.as_mut_ptr()).last_node = 1 as libc::c_int as uint8_t;
    (*(*S).S[(PARALLELISM_DEGREE - 1 as libc::c_int) as usize].as_mut_ptr()).last_node =
        1 as libc::c_int as uint8_t;
    let mut block: [uint8_t; 64] = [0; 64];
    memset(
        block.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong,
    );
    memcpy(block.as_mut_ptr() as *mut libc::c_void, key, keylen);
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        blake2s_update(
            (*S).S[i as usize].as_mut_ptr(),
            block.as_mut_ptr() as *const libc::c_void,
            BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
        );
        i = i.wrapping_add(1)
    }
    secure_zero_memory(
        block.as_mut_ptr() as *mut libc::c_void,
        BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2sp_update(
    mut S: *mut blake2sp_state,
    mut pin: *const libc::c_void,
    mut inlen: size_t,
) -> libc::c_int {
    let mut in_0: *const libc::c_uchar = pin as *const libc::c_uchar;
    let mut left: size_t = (*S).buflen;
    let mut fill: size_t =
        (::std::mem::size_of::<[uint8_t; 512]>() as libc::c_ulong).wrapping_sub(left);
    let mut i: size_t = 0;
    if left != 0 && inlen >= fill {
        memcpy(
            (*S).buf.as_mut_ptr().offset(left as isize) as *mut libc::c_void,
            in_0 as *const libc::c_void,
            fill,
        );
        i = 0 as libc::c_int as size_t;
        while i < PARALLELISM_DEGREE as libc::c_ulong {
            blake2s_update(
                (*S).S[i as usize].as_mut_ptr(),
                (*S).buf.as_mut_ptr().offset(
                    i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong) as isize,
                ) as *const libc::c_void,
                BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
            );
            i = i.wrapping_add(1)
        }
        in_0 = in_0.offset(fill as isize);
        inlen = (inlen as libc::c_ulong).wrapping_sub(fill) as size_t as size_t;
        left = 0 as libc::c_int as size_t
    }
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        let mut inlen__: size_t = inlen;
        let mut in__: *const libc::c_uchar = in_0;
        in__ = in__
            .offset(i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong) as isize);
        while inlen__ >= (PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as libc::c_ulong {
            blake2s_update(
                (*S).S[i as usize].as_mut_ptr(),
                in__ as *const libc::c_void,
                BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
            );
            in__ = in__.offset((PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as isize);
            inlen__ = (inlen__ as libc::c_ulong).wrapping_sub(
                (PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t
        }
        i = i.wrapping_add(1)
    }
    in_0 =
        in_0.offset(inlen.wrapping_sub(inlen.wrapping_rem(
            (PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as libc::c_ulong,
        )) as isize);
    inlen = (inlen as libc::c_ulong)
        .wrapping_rem((PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as libc::c_ulong)
        as size_t as size_t;
    if inlen > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            (*S).buf.as_mut_ptr().offset(left as isize) as *mut libc::c_void,
            in_0 as *const libc::c_void,
            inlen,
        );
    }
    (*S).buflen = left.wrapping_add(inlen);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn blake2sp_final(
    mut S: *mut blake2sp_state,
    mut out: *mut libc::c_void,
    mut outlen: size_t,
) -> libc::c_int {
    let mut hash: [[uint8_t; 32]; 8] = [[0; 32]; 8];
    let mut i: size_t = 0;
    if out.is_null() || outlen < (*S).outlen {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        if (*S).buflen > i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong) {
            let mut left: size_t = (*S)
                .buflen
                .wrapping_sub(i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong));
            if left > BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong {
                left = BLAKE2S_BLOCKBYTES as libc::c_int as size_t
            }
            blake2s_update(
                (*S).S[i as usize].as_mut_ptr(),
                (*S).buf.as_mut_ptr().offset(
                    i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong) as isize,
                ) as *const libc::c_void,
                left,
            );
        }
        blake2s_final(
            (*S).S[i as usize].as_mut_ptr(),
            hash[i as usize].as_mut_ptr() as *mut libc::c_void,
            BLAKE2S_OUTBYTES as libc::c_int as size_t,
        );
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        blake2s_update(
            (*S).R.as_mut_ptr(),
            hash[i as usize].as_mut_ptr() as *const libc::c_void,
            BLAKE2S_OUTBYTES as libc::c_int as size_t,
        );
        i = i.wrapping_add(1)
    }
    return blake2s_final((*S).R.as_mut_ptr(), out, (*S).outlen);
}
#[no_mangle]
pub unsafe extern "C" fn blake2sp(
    mut out: *mut libc::c_void,
    mut outlen: size_t,
    mut in_0: *const libc::c_void,
    mut inlen: size_t,
    mut key: *const libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    let mut hash: [[uint8_t; 32]; 8] = [[0; 32]; 8];
    let mut S: [[blake2s_state; 1]; 8] = [[blake2s_state {
        h: [0; 8],
        t: [0; 2],
        f: [0; 2],
        buf: [0; 64],
        buflen: 0,
        outlen: 0,
        last_node: 0,
    }; 1]; 8];
    let mut FS: [blake2s_state; 1] = [blake2s_state {
        h: [0; 8],
        t: [0; 2],
        f: [0; 2],
        buf: [0; 64],
        buflen: 0,
        outlen: 0,
        last_node: 0,
    }; 1];
    let mut i: size_t = 0;
    /* Burn the key from stack */
    /* Verify parameters */
    if NULL as *const libc::c_void == in_0 && inlen > 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    } /* mark last node */
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
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        if blake2sp_init_leaf(S[i as usize].as_mut_ptr(), outlen, keylen, i as uint32_t)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    (*S[(PARALLELISM_DEGREE - 1 as libc::c_int) as usize].as_mut_ptr()).last_node =
        1 as libc::c_int as uint8_t;
    if keylen > 0 as libc::c_int as libc::c_ulong {
        let mut block: [uint8_t; 64] = [0; 64];
        memset(
            block.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong,
        );
        memcpy(block.as_mut_ptr() as *mut libc::c_void, key, keylen);
        i = 0 as libc::c_int as size_t;
        while i < PARALLELISM_DEGREE as libc::c_ulong {
            blake2s_update(
                S[i as usize].as_mut_ptr(),
                block.as_mut_ptr() as *const libc::c_void,
                BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
            );
            i = i.wrapping_add(1)
        }
        secure_zero_memory(
            block.as_mut_ptr() as *mut libc::c_void,
            BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
        );
        /* Burn the key from stack */
    }
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        let mut inlen__: size_t = inlen;
        let mut in__: *const libc::c_uchar = in_0 as *const libc::c_uchar;
        in__ = in__
            .offset(i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong) as isize);
        while inlen__ >= (PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as libc::c_ulong {
            blake2s_update(
                S[i as usize].as_mut_ptr(),
                in__ as *const libc::c_void,
                BLAKE2S_BLOCKBYTES as libc::c_int as size_t,
            );
            in__ = in__.offset((PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as isize);
            inlen__ = (inlen__ as libc::c_ulong).wrapping_sub(
                (PARALLELISM_DEGREE * BLAKE2S_BLOCKBYTES as libc::c_int) as libc::c_ulong,
            ) as size_t as size_t
        }
        if inlen__ > i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong) {
            let left: size_t = inlen__
                .wrapping_sub(i.wrapping_mul(BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong));
            let len: size_t = if left <= BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong {
                left
            } else {
                BLAKE2S_BLOCKBYTES as libc::c_int as libc::c_ulong
            };
            blake2s_update(S[i as usize].as_mut_ptr(), in__ as *const libc::c_void, len);
        }
        blake2s_final(
            S[i as usize].as_mut_ptr(),
            hash[i as usize].as_mut_ptr() as *mut libc::c_void,
            BLAKE2S_OUTBYTES as libc::c_int as size_t,
        );
        i = i.wrapping_add(1)
    }
    if blake2sp_init_root(FS.as_mut_ptr(), outlen, keylen) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*FS.as_mut_ptr()).last_node = 1 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as size_t;
    while i < PARALLELISM_DEGREE as libc::c_ulong {
        blake2s_update(
            FS.as_mut_ptr(),
            hash[i as usize].as_mut_ptr() as *const libc::c_void,
            BLAKE2S_OUTBYTES as libc::c_int as size_t,
        );
        i = i.wrapping_add(1)
    }
    return blake2s_final(FS.as_mut_ptr(), out, outlen);
}
