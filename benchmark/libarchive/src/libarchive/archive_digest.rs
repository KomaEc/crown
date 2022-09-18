use ::libc;
extern "C" {
    pub type evp_md_st;
    pub type evp_md_ctx_st;
    #[no_mangle]
    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    #[no_mangle]
    fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
    #[no_mangle]
    fn EVP_DigestUpdate(ctx: *mut EVP_MD_CTX, d: *const libc::c_void, cnt: size_t) -> libc::c_int;
    #[no_mangle]
    fn EVP_DigestInit(ctx: *mut EVP_MD_CTX, type_0: *const EVP_MD) -> libc::c_int;
    #[no_mangle]
    fn EVP_DigestFinal(
        ctx: *mut EVP_MD_CTX,
        md: *mut libc::c_uchar,
        s: *mut libc::c_uint,
    ) -> libc::c_int;
    #[no_mangle]
    fn EVP_md5() -> *const EVP_MD;
    #[no_mangle]
    fn EVP_sha1() -> *const EVP_MD;
    #[no_mangle]
    fn EVP_sha256() -> *const EVP_MD;
    #[no_mangle]
    fn EVP_sha384() -> *const EVP_MD;
    #[no_mangle]
    fn EVP_sha512() -> *const EVP_MD;
    #[no_mangle]
    fn EVP_ripemd160() -> *const EVP_MD;
}
pub type size_t = libc::c_ulong;
pub type EVP_MD = evp_md_st;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type archive_md5_ctx = *mut EVP_MD_CTX;
pub type archive_rmd160_ctx = *mut EVP_MD_CTX;
pub type archive_sha1_ctx = *mut EVP_MD_CTX;
pub type archive_sha256_ctx = *mut EVP_MD_CTX;
pub type archive_sha384_ctx = *mut EVP_MD_CTX;
pub type archive_sha512_ctx = *mut EVP_MD_CTX;
/*-
* Copyright (c) 2003-2007 Tim Kientzle
* Copyright (c) 2011 Andres Mejia
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
 * Crypto support in various Operating Systems:
 *
 * NetBSD:
 * - MD5 and SHA1 in libc: without _ after algorithm name
 * - SHA2 in libc: with _ after algorithm name
 *
 * OpenBSD:
 * - MD5, SHA1 and SHA2 in libc: without _ after algorithm name
 * - OpenBSD 4.4 and earlier have SHA2 in libc with _ after algorithm name
 *
 * DragonFly and FreeBSD:
 * - MD5 libmd: without _ after algorithm name
 * - SHA1, SHA256 and SHA512 in libmd: with _ after algorithm name
 *
 * Mac OS X (10.4 and later):
 * - MD5, SHA1 and SHA2 in libSystem: with CC_ prefix and _ after algorithm name
 *
 * OpenSSL:
 * - MD5, SHA1 and SHA2 in libcrypto: with _ after algorithm name
 *
 * Windows:
 * - MD5, SHA1 and SHA2 in archive_crypto.c using Windows crypto API
 */
/* libc crypto headers */
/* libmd crypto headers */
/* libSystem crypto headers */
/* mbed TLS crypto headers */
/* Nettle crypto headers */
/* OpenSSL crypto headers */
/* Windows crypto headers */
/* typedefs */
/* defines */
/* Minimal interface to digest functionality for internal use in libarchive */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_digest {
    pub md5init: Option<unsafe extern "C" fn(_: *mut archive_md5_ctx) -> libc::c_int>,
    pub md5update: Option<
        unsafe extern "C" fn(
            _: *mut archive_md5_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub md5final:
        Option<unsafe extern "C" fn(_: *mut archive_md5_ctx, _: *mut libc::c_void) -> libc::c_int>,
    pub rmd160init: Option<unsafe extern "C" fn(_: *mut archive_rmd160_ctx) -> libc::c_int>,
    pub rmd160update: Option<
        unsafe extern "C" fn(
            _: *mut archive_rmd160_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub rmd160final: Option<
        unsafe extern "C" fn(_: *mut archive_rmd160_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha1init: Option<unsafe extern "C" fn(_: *mut archive_sha1_ctx) -> libc::c_int>,
    pub sha1update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha1_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha1final:
        Option<unsafe extern "C" fn(_: *mut archive_sha1_ctx, _: *mut libc::c_void) -> libc::c_int>,
    pub sha256init: Option<unsafe extern "C" fn(_: *mut archive_sha256_ctx) -> libc::c_int>,
    pub sha256update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha256_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha256final: Option<
        unsafe extern "C" fn(_: *mut archive_sha256_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha384init: Option<unsafe extern "C" fn(_: *mut archive_sha384_ctx) -> libc::c_int>,
    pub sha384update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha384_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha384final: Option<
        unsafe extern "C" fn(_: *mut archive_sha384_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
    pub sha512init: Option<unsafe extern "C" fn(_: *mut archive_sha512_ctx) -> libc::c_int>,
    pub sha512update: Option<
        unsafe extern "C" fn(
            _: *mut archive_sha512_ctx,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub sha512final: Option<
        unsafe extern "C" fn(_: *mut archive_sha512_ctx, _: *mut libc::c_void) -> libc::c_int,
    >,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
/*-
* Copyright (c) 2003-2007 Tim Kientzle
* Copyright (c) 2011 Andres Mejia
* Copyright (c) 2011 Michihiro NAKAJIMA
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
/* In particular, force the configure probe to break if it tries
 * to test a combination of OpenSSL and libmd. */
/*
 * Message digest functions for Windows platform.
 */
/* defined(ARCHIVE_CRYPTO_*_WIN) */
/* MD5 implementations */
unsafe extern "C" fn __archive_md5init(mut ctx: *mut archive_md5_ctx) -> libc::c_int {
    *ctx = EVP_MD_CTX_new();
    if (*ctx).is_null() {
        return -(25 as libc::c_int);
    }
    EVP_DigestInit(*ctx, EVP_md5());
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_md5update(
    mut ctx: *mut archive_md5_ctx,
    mut indata: *const libc::c_void,
    mut insize: size_t,
) -> libc::c_int {
    EVP_DigestUpdate(*ctx, indata, insize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_md5final(
    mut ctx: *mut archive_md5_ctx,
    mut md: *mut libc::c_void,
) -> libc::c_int {
    /* HACK: archive_write_set_format_xar.c is finalizing empty contexts, so
     * this is meant to cope with that. Real fix is probably to fix
     * archive_write_set_format_xar.c
     */
    if !(*ctx).is_null() {
        EVP_DigestFinal(*ctx, md as *mut libc::c_uchar, NULL as *mut libc::c_uint);
        EVP_MD_CTX_free(*ctx);
        *ctx = NULL as archive_md5_ctx
    }
    return 0 as libc::c_int;
}
/* RIPEMD160 implementations */
unsafe extern "C" fn __archive_ripemd160init(mut ctx: *mut archive_rmd160_ctx) -> libc::c_int {
    *ctx = EVP_MD_CTX_new();
    if (*ctx).is_null() {
        return -(25 as libc::c_int);
    }
    EVP_DigestInit(*ctx, EVP_ripemd160());
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_ripemd160update(
    mut ctx: *mut archive_rmd160_ctx,
    mut indata: *const libc::c_void,
    mut insize: size_t,
) -> libc::c_int {
    EVP_DigestUpdate(*ctx, indata, insize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_ripemd160final(
    mut ctx: *mut archive_rmd160_ctx,
    mut md: *mut libc::c_void,
) -> libc::c_int {
    if !(*ctx).is_null() {
        EVP_DigestFinal(*ctx, md as *mut libc::c_uchar, NULL as *mut libc::c_uint);
        EVP_MD_CTX_free(*ctx);
        *ctx = NULL as archive_rmd160_ctx
    }
    return 0 as libc::c_int;
}
/* SHA1 implementations */
unsafe extern "C" fn __archive_sha1init(mut ctx: *mut archive_sha1_ctx) -> libc::c_int {
    *ctx = EVP_MD_CTX_new();
    if (*ctx).is_null() {
        return -(25 as libc::c_int);
    }
    EVP_DigestInit(*ctx, EVP_sha1());
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha1update(
    mut ctx: *mut archive_sha1_ctx,
    mut indata: *const libc::c_void,
    mut insize: size_t,
) -> libc::c_int {
    EVP_DigestUpdate(*ctx, indata, insize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha1final(
    mut ctx: *mut archive_sha1_ctx,
    mut md: *mut libc::c_void,
) -> libc::c_int {
    /* HACK: archive_write_set_format_xar.c is finalizing empty contexts, so
     * this is meant to cope with that. Real fix is probably to fix
     * archive_write_set_format_xar.c
     */
    if !(*ctx).is_null() {
        EVP_DigestFinal(*ctx, md as *mut libc::c_uchar, NULL as *mut libc::c_uint);
        EVP_MD_CTX_free(*ctx);
        *ctx = NULL as archive_sha1_ctx
    }
    return 0 as libc::c_int;
}
/* SHA256 implementations */
unsafe extern "C" fn __archive_sha256init(mut ctx: *mut archive_sha256_ctx) -> libc::c_int {
    *ctx = EVP_MD_CTX_new();
    if (*ctx).is_null() {
        return -(25 as libc::c_int);
    }
    EVP_DigestInit(*ctx, EVP_sha256());
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha256update(
    mut ctx: *mut archive_sha256_ctx,
    mut indata: *const libc::c_void,
    mut insize: size_t,
) -> libc::c_int {
    EVP_DigestUpdate(*ctx, indata, insize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha256final(
    mut ctx: *mut archive_sha256_ctx,
    mut md: *mut libc::c_void,
) -> libc::c_int {
    if !(*ctx).is_null() {
        EVP_DigestFinal(*ctx, md as *mut libc::c_uchar, NULL as *mut libc::c_uint);
        EVP_MD_CTX_free(*ctx);
        *ctx = NULL as archive_sha256_ctx
    }
    return 0 as libc::c_int;
}
/* SHA384 implementations */
unsafe extern "C" fn __archive_sha384init(mut ctx: *mut archive_sha384_ctx) -> libc::c_int {
    *ctx = EVP_MD_CTX_new();
    if (*ctx).is_null() {
        return -(25 as libc::c_int);
    }
    EVP_DigestInit(*ctx, EVP_sha384());
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha384update(
    mut ctx: *mut archive_sha384_ctx,
    mut indata: *const libc::c_void,
    mut insize: size_t,
) -> libc::c_int {
    EVP_DigestUpdate(*ctx, indata, insize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha384final(
    mut ctx: *mut archive_sha384_ctx,
    mut md: *mut libc::c_void,
) -> libc::c_int {
    if !(*ctx).is_null() {
        EVP_DigestFinal(*ctx, md as *mut libc::c_uchar, NULL as *mut libc::c_uint);
        EVP_MD_CTX_free(*ctx);
        *ctx = NULL as archive_sha384_ctx
    }
    return 0 as libc::c_int;
}
/* SHA512 implementations */
unsafe extern "C" fn __archive_sha512init(mut ctx: *mut archive_sha512_ctx) -> libc::c_int {
    *ctx = EVP_MD_CTX_new();
    if (*ctx).is_null() {
        return -(25 as libc::c_int);
    }
    EVP_DigestInit(*ctx, EVP_sha512());
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha512update(
    mut ctx: *mut archive_sha512_ctx,
    mut indata: *const libc::c_void,
    mut insize: size_t,
) -> libc::c_int {
    EVP_DigestUpdate(*ctx, indata, insize);
    return 0 as libc::c_int;
}
unsafe extern "C" fn __archive_sha512final(
    mut ctx: *mut archive_sha512_ctx,
    mut md: *mut libc::c_void,
) -> libc::c_int {
    if !(*ctx).is_null() {
        EVP_DigestFinal(*ctx, md as *mut libc::c_uchar, NULL as *mut libc::c_uint);
        EVP_MD_CTX_free(*ctx);
        *ctx = NULL as archive_sha512_ctx
    }
    return 0 as libc::c_int;
}
/* NOTE: Message Digest functions are set based on availability and by the
 * following order of preference.
 * 1. libc
 * 2. libc2
 * 3. libc3
 * 4. libSystem
 * 5. Nettle
 * 6. OpenSSL
 * 7. libmd
 * 8. Windows API
 */
#[no_mangle]
pub static mut __archive_digest: archive_digest = {
    let mut init = archive_digest {
        md5init: Some(
            __archive_md5init as unsafe extern "C" fn(_: *mut archive_md5_ctx) -> libc::c_int,
        ),
        md5update: Some(
            __archive_md5update
                as unsafe extern "C" fn(
                    _: *mut archive_md5_ctx,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> libc::c_int,
        ),
        md5final: Some(
            __archive_md5final
                as unsafe extern "C" fn(
                    _: *mut archive_md5_ctx,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
        rmd160init: Some(
            __archive_ripemd160init
                as unsafe extern "C" fn(_: *mut archive_rmd160_ctx) -> libc::c_int,
        ),
        rmd160update: Some(
            __archive_ripemd160update
                as unsafe extern "C" fn(
                    _: *mut archive_rmd160_ctx,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> libc::c_int,
        ),
        rmd160final: Some(
            __archive_ripemd160final
                as unsafe extern "C" fn(
                    _: *mut archive_rmd160_ctx,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
        sha1init: Some(
            __archive_sha1init as unsafe extern "C" fn(_: *mut archive_sha1_ctx) -> libc::c_int,
        ),
        sha1update: Some(
            __archive_sha1update
                as unsafe extern "C" fn(
                    _: *mut archive_sha1_ctx,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> libc::c_int,
        ),
        sha1final: Some(
            __archive_sha1final
                as unsafe extern "C" fn(
                    _: *mut archive_sha1_ctx,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
        sha256init: Some(
            __archive_sha256init as unsafe extern "C" fn(_: *mut archive_sha256_ctx) -> libc::c_int,
        ),
        sha256update: Some(
            __archive_sha256update
                as unsafe extern "C" fn(
                    _: *mut archive_sha256_ctx,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> libc::c_int,
        ),
        sha256final: Some(
            __archive_sha256final
                as unsafe extern "C" fn(
                    _: *mut archive_sha256_ctx,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
        sha384init: Some(
            __archive_sha384init as unsafe extern "C" fn(_: *mut archive_sha384_ctx) -> libc::c_int,
        ),
        sha384update: Some(
            __archive_sha384update
                as unsafe extern "C" fn(
                    _: *mut archive_sha384_ctx,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> libc::c_int,
        ),
        sha384final: Some(
            __archive_sha384final
                as unsafe extern "C" fn(
                    _: *mut archive_sha384_ctx,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
        sha512init: Some(
            __archive_sha512init as unsafe extern "C" fn(_: *mut archive_sha512_ctx) -> libc::c_int,
        ),
        sha512update: Some(
            __archive_sha512update
                as unsafe extern "C" fn(
                    _: *mut archive_sha512_ctx,
                    _: *const libc::c_void,
                    _: size_t,
                ) -> libc::c_int,
        ),
        sha512final: Some(
            __archive_sha512final
                as unsafe extern "C" fn(
                    _: *mut archive_sha512_ctx,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
    };
    init
};
