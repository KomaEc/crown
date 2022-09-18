use ::libc;
extern "C" {
    pub type evp_md_st;
    pub type hmac_ctx_st;
    pub type engine_st;
    #[no_mangle]
    fn HMAC_Final(
        ctx: *mut HMAC_CTX,
        md: *mut libc::c_uchar,
        len: *mut libc::c_uint,
    ) -> libc::c_int;
    #[no_mangle]
    fn HMAC_Update(ctx: *mut HMAC_CTX, data: *const libc::c_uchar, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn HMAC_Init_ex(
        ctx: *mut HMAC_CTX,
        key: *const libc::c_void,
        len: libc::c_int,
        md: *const EVP_MD,
        impl_0: *mut ENGINE,
    ) -> libc::c_int;
    #[no_mangle]
    fn HMAC_CTX_free(ctx: *mut HMAC_CTX);
    #[no_mangle]
    fn HMAC_CTX_new() -> *mut HMAC_CTX;
    #[no_mangle]
    fn EVP_sha1() -> *const EVP_MD;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
pub type EVP_MD = evp_md_st;
pub type HMAC_CTX = hmac_ctx_st;
pub type ENGINE = engine_st;
pub type archive_hmac_sha1_ctx = *mut HMAC_CTX;
/* HMAC */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_hmac {
    pub __hmac_sha1_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_hmac_sha1_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub __hmac_sha1_update: Option<
        unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx, _: *const uint8_t, _: size_t) -> (),
    >,
    pub __hmac_sha1_final: Option<
        unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx, _: *mut uint8_t, _: *mut size_t) -> (),
    >,
    pub __hmac_sha1_cleanup: Option<unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx) -> ()>,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
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
 * the archive_hmac.c file is expected to define no usable symbols.
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
 * the archive_hmac.c file is expected to define no usable symbols.
 *
 * But some compilers and linkers choke on empty object files, so
 * define a public symbol that will always exist.  This could
 * be removed someday if this file gains another always-present
 * symbol definition.
 */
#[no_mangle]
pub unsafe extern "C" fn __libarchive_hmac_build_hack() -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn __hmac_sha1_init(
    mut ctx: *mut archive_hmac_sha1_ctx,
    mut key: *const uint8_t,
    mut key_len: size_t,
) -> libc::c_int {
    *ctx = HMAC_CTX_new();
    if (*ctx).is_null() {
        return -(1 as libc::c_int);
    }
    HMAC_Init_ex(
        *ctx,
        key as *const libc::c_void,
        key_len as libc::c_int,
        EVP_sha1(),
        NULL as *mut ENGINE,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn __hmac_sha1_update(
    mut ctx: *mut archive_hmac_sha1_ctx,
    mut data: *const uint8_t,
    mut data_len: size_t,
) {
    HMAC_Update(*ctx, data, data_len);
}
unsafe extern "C" fn __hmac_sha1_final(
    mut ctx: *mut archive_hmac_sha1_ctx,
    mut out: *mut uint8_t,
    mut out_len: *mut size_t,
) {
    let mut len: libc::c_uint = *out_len as libc::c_uint;
    HMAC_Final(*ctx, out, &mut len);
    *out_len = len as size_t;
}
unsafe extern "C" fn __hmac_sha1_cleanup(mut ctx: *mut archive_hmac_sha1_ctx) {
    HMAC_CTX_free(*ctx);
    *ctx = NULL as archive_hmac_sha1_ctx;
}
#[no_mangle]
pub static mut __archive_hmac: archive_hmac = {
    let mut init = archive_hmac {
        __hmac_sha1_init: Some(
            __hmac_sha1_init
                as unsafe extern "C" fn(
                    _: *mut archive_hmac_sha1_ctx,
                    _: *const uint8_t,
                    _: size_t,
                ) -> libc::c_int,
        ),
        __hmac_sha1_update: Some(
            __hmac_sha1_update
                as unsafe extern "C" fn(
                    _: *mut archive_hmac_sha1_ctx,
                    _: *const uint8_t,
                    _: size_t,
                ) -> (),
        ),
        __hmac_sha1_final: Some(
            __hmac_sha1_final
                as unsafe extern "C" fn(
                    _: *mut archive_hmac_sha1_ctx,
                    _: *mut uint8_t,
                    _: *mut size_t,
                ) -> (),
        ),
        __hmac_sha1_cleanup: Some(
            __hmac_sha1_cleanup as unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx) -> (),
        ),
    };
    init
};
