use ::libc;
extern "C" {
    pub type evp_cipher_st;
    pub type evp_cipher_ctx_st;
    pub type engine_st;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn EVP_EncryptInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_0: *mut ENGINE,
        key: *const libc::c_uchar,
        iv: *const libc::c_uchar,
    ) -> libc::c_int;
    #[no_mangle]
    fn EVP_EncryptUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut libc::c_uchar,
        outl: *mut libc::c_int,
        in_0: *const libc::c_uchar,
        inl: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    #[no_mangle]
    fn EVP_CIPHER_CTX_reset(c: *mut EVP_CIPHER_CTX) -> libc::c_int;
    #[no_mangle]
    fn EVP_CIPHER_CTX_free(c: *mut EVP_CIPHER_CTX);
    #[no_mangle]
    fn EVP_aes_128_ecb() -> *const EVP_CIPHER;
    #[no_mangle]
    fn EVP_aes_192_ecb() -> *const EVP_CIPHER;
    #[no_mangle]
    fn EVP_aes_256_ecb() -> *const EVP_CIPHER;
    #[no_mangle]
    fn PKCS5_PBKDF2_HMAC_SHA1(
        pass: *const libc::c_char,
        passlen: libc::c_int,
        salt: *const libc::c_uchar,
        saltlen: libc::c_int,
        iter: libc::c_int,
        keylen: libc::c_int,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
pub type EVP_CIPHER = evp_cipher_st;
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
pub type ENGINE = engine_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_crypto_ctx {
    pub ctx: *mut EVP_CIPHER_CTX,
    pub type_0: *const EVP_CIPHER,
    pub key: [uint8_t; 32],
    pub key_len: libc::c_uint,
    pub nonce: [uint8_t; 16],
    pub encr_buf: [uint8_t; 16],
    pub encr_pos: libc::c_uint,
}
/* defines */
/* Minimal interface to cryptographic functionality for internal use in
 * libarchive */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_cryptor {
    pub pbkdf2sha1: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: size_t,
            _: *const uint8_t,
            _: size_t,
            _: libc::c_uint,
            _: *mut uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_update: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
            _: *mut uint8_t,
            _: *mut size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_release:
        Option<unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int>,
    pub encrypto_aes_ctr_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub encrypto_aes_ctr_update: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
            _: *mut uint8_t,
            _: *mut size_t,
        ) -> libc::c_int,
    >,
    pub encrypto_aes_ctr_release:
        Option<unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int>,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const AES_BLOCK_SIZE: libc::c_int = 16 as libc::c_int;
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
/*
 * On systems that do not support any recognized crypto libraries,
 * the archive_cryptor.c file will normally define no usable symbols.
 *
 * But some compilers and linkers choke on empty object files, so
 * define a public symbol that will always exist.  This could
 * be removed someday if this file gains another always-present
 * symbol definition.
 */
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
/*
 * On systems that do not support any recognized crypto libraries,
 * this file will normally define no usable symbols.
 *
 * But some compilers and linkers choke on empty object files, so
 * define a public symbol that will always exist.  This could
 * be removed someday if this file gains another always-present
 * symbol definition.
 */
#[no_mangle]
pub unsafe extern "C" fn __libarchive_cryptor_build_hack() -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn pbkdf2_sha1(
    mut pw: *const libc::c_char,
    mut pw_len: size_t,
    mut salt: *const uint8_t,
    mut salt_len: size_t,
    mut rounds: libc::c_uint,
    mut derived_key: *mut uint8_t,
    mut derived_key_len: size_t,
) -> libc::c_int {
    PKCS5_PBKDF2_HMAC_SHA1(
        pw,
        pw_len as libc::c_int,
        salt,
        salt_len as libc::c_int,
        rounds as libc::c_int,
        derived_key_len as libc::c_int,
        derived_key,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn aes_ctr_init(
    mut ctx: *mut archive_crypto_ctx,
    mut key: *const uint8_t,
    mut key_len: size_t,
) -> libc::c_int {
    (*ctx).ctx = EVP_CIPHER_CTX_new();
    if (*ctx).ctx.is_null() {
        return -(1 as libc::c_int);
    }
    match key_len {
        16 => (*ctx).type_0 = EVP_aes_128_ecb(),
        24 => (*ctx).type_0 = EVP_aes_192_ecb(),
        32 => (*ctx).type_0 = EVP_aes_256_ecb(),
        _ => {
            (*ctx).type_0 = NULL as *const EVP_CIPHER;
            return -(1 as libc::c_int);
        }
    }
    (*ctx).key_len = key_len as libc::c_uint;
    memcpy(
        (*ctx).key.as_mut_ptr() as *mut libc::c_void,
        key as *const libc::c_void,
        key_len,
    );
    memset(
        (*ctx).nonce.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    (*ctx).encr_pos = AES_BLOCK_SIZE as libc::c_uint;
    if EVP_CIPHER_CTX_reset((*ctx).ctx) == 0 {
        EVP_CIPHER_CTX_free((*ctx).ctx);
        (*ctx).ctx = NULL as *mut EVP_CIPHER_CTX
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn aes_ctr_encrypt_counter(mut ctx: *mut archive_crypto_ctx) -> libc::c_int {
    let mut outl: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    r = EVP_EncryptInit_ex(
        (*ctx).ctx,
        (*ctx).type_0,
        NULL as *mut ENGINE,
        (*ctx).key.as_mut_ptr(),
        NULL as *const libc::c_uchar,
    );
    if r == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    r = EVP_EncryptUpdate(
        (*ctx).ctx,
        (*ctx).encr_buf.as_mut_ptr(),
        &mut outl,
        (*ctx).nonce.as_mut_ptr(),
        AES_BLOCK_SIZE,
    );
    if r == 0 as libc::c_int || outl != AES_BLOCK_SIZE {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn aes_ctr_release(mut ctx: *mut archive_crypto_ctx) -> libc::c_int {
    EVP_CIPHER_CTX_free((*ctx).ctx);
    memset(
        (*ctx).key.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (*ctx).key_len as libc::c_ulong,
    );
    memset(
        (*ctx).nonce.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn aes_ctr_increase_counter(mut ctx: *mut archive_crypto_ctx) {
    let nonce: *mut uint8_t = (*ctx).nonce.as_mut_ptr();
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        let ref mut fresh0 = *nonce.offset(j as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        if *fresh0 != 0 {
            break;
        }
        j += 1
    }
}
unsafe extern "C" fn aes_ctr_update(
    mut ctx: *mut archive_crypto_ctx,
    in_0: *const uint8_t,
    mut in_len: size_t,
    out: *mut uint8_t,
    mut out_len: *mut size_t,
) -> libc::c_int {
    let ebuf: *mut uint8_t = (*ctx).encr_buf.as_mut_ptr();
    let mut pos: libc::c_uint = (*ctx).encr_pos;
    let mut max: libc::c_uint = if in_len < *out_len { in_len } else { *out_len } as libc::c_uint;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < max {
        if pos == AES_BLOCK_SIZE as libc::c_uint {
            aes_ctr_increase_counter(ctx);
            if aes_ctr_encrypt_counter(ctx) != 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            while max.wrapping_sub(i) >= AES_BLOCK_SIZE as libc::c_uint {
                pos = 0 as libc::c_int as libc::c_uint;
                while pos < AES_BLOCK_SIZE as libc::c_uint {
                    *out.offset(i.wrapping_add(pos) as isize) =
                        (*in_0.offset(i.wrapping_add(pos) as isize) as libc::c_int
                            ^ *ebuf.offset(pos as isize) as libc::c_int)
                            as uint8_t;
                    pos = pos.wrapping_add(1)
                }
                i = i.wrapping_add(AES_BLOCK_SIZE as libc::c_uint);
                aes_ctr_increase_counter(ctx);
                if aes_ctr_encrypt_counter(ctx) != 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
            }
            pos = 0 as libc::c_int as libc::c_uint;
            if i >= max {
                break;
            }
        }
        let fresh1 = pos;
        pos = pos.wrapping_add(1);
        *out.offset(i as isize) = (*in_0.offset(i as isize) as libc::c_int
            ^ *ebuf.offset(fresh1 as isize) as libc::c_int)
            as uint8_t;
        i = i.wrapping_add(1)
    }
    (*ctx).encr_pos = pos;
    *out_len = i as size_t;
    return 0 as libc::c_int;
}
/* ARCHIVE_CRYPTOR_STUB */
#[no_mangle]
pub static mut __archive_cryptor: archive_cryptor = {
    let mut init = archive_cryptor {
        pbkdf2sha1: Some(
            pbkdf2_sha1
                as unsafe extern "C" fn(
                    _: *const libc::c_char,
                    _: size_t,
                    _: *const uint8_t,
                    _: size_t,
                    _: libc::c_uint,
                    _: *mut uint8_t,
                    _: size_t,
                ) -> libc::c_int,
        ),
        decrypto_aes_ctr_init: Some(
            aes_ctr_init
                as unsafe extern "C" fn(
                    _: *mut archive_crypto_ctx,
                    _: *const uint8_t,
                    _: size_t,
                ) -> libc::c_int,
        ),
        decrypto_aes_ctr_update: Some(
            aes_ctr_update
                as unsafe extern "C" fn(
                    _: *mut archive_crypto_ctx,
                    _: *const uint8_t,
                    _: size_t,
                    _: *mut uint8_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        ),
        decrypto_aes_ctr_release: Some(
            aes_ctr_release as unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int,
        ),
        encrypto_aes_ctr_init: Some(
            aes_ctr_init
                as unsafe extern "C" fn(
                    _: *mut archive_crypto_ctx,
                    _: *const uint8_t,
                    _: size_t,
                ) -> libc::c_int,
        ),
        encrypto_aes_ctr_update: Some(
            aes_ctr_update
                as unsafe extern "C" fn(
                    _: *mut archive_crypto_ctx,
                    _: *const uint8_t,
                    _: size_t,
                    _: *mut uint8_t,
                    _: *mut size_t,
                ) -> libc::c_int,
        ),
        encrypto_aes_ctr_release: Some(
            aes_ctr_release as unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int,
        ),
    };
    init
};
