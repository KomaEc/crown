use ::libc;
extern "C" {
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn archive_write_add_filter_bzip2(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_compress(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_grzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_gzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lrzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lz4(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lzma(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_none(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_uuencode(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_xz(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_zstd(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
/*-
 * Copyright (c) 2003-2007 Tim Kientzle
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
 *
 * $FreeBSD: head/lib/libarchive/archive_private.h 201098 2009-12-28 02:58:14Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive {
    pub magic: libc::c_uint,
    pub state: libc::c_uint,
    pub vtable: *mut archive_vtable,
    pub archive_format: libc::c_int,
    pub archive_format_name: *const libc::c_char,
    pub compression_code: libc::c_int,
    pub compression_name: *const libc::c_char,
    pub file_count: libc::c_int,
    pub archive_error_number: libc::c_int,
    pub error: *const libc::c_char,
    pub error_string: archive_string,
    pub current_code: *mut libc::c_char,
    pub current_codepage: libc::c_uint,
    pub current_oemcp: libc::c_uint,
    pub sconv: *mut archive_string_conv,
    pub read_data_block: *const libc::c_char,
    pub read_data_offset: int64_t,
    pub read_data_output_offset: int64_t,
    pub read_data_remaining: size_t,
    pub read_data_is_posix_read: libc::c_char,
    pub read_data_requested: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_vtable {
    pub archive_close: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_free: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_write_finish_entry: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_data:
        Option<unsafe extern "C" fn(_: *mut archive, _: *const libc::c_void, _: size_t) -> ssize_t>,
    pub archive_write_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *const libc::c_void,
            _: size_t,
            _: int64_t,
        ) -> ssize_t,
    >,
    pub archive_read_next_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int>,
    pub archive_read_next_header2:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_read_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub archive_filter_count: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_filter_bytes:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t>,
    pub archive_filter_code:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int>,
    pub archive_filter_name:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char>,
}
/* A table that maps filter codes to functions. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub code: libc::c_int,
    pub setter: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
/*
 * Codes to identify various stream filters.
 */
pub const ARCHIVE_FILTER_NONE: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FILTER_GZIP: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_FILTER_BZIP2: libc::c_int = 2 as libc::c_int;
pub const ARCHIVE_FILTER_COMPRESS: libc::c_int = 3 as libc::c_int;
pub const ARCHIVE_FILTER_LZMA: libc::c_int = 5 as libc::c_int;
pub const ARCHIVE_FILTER_XZ: libc::c_int = 6 as libc::c_int;
pub const ARCHIVE_FILTER_UU: libc::c_int = 7 as libc::c_int;
pub const ARCHIVE_FILTER_LZIP: libc::c_int = 9 as libc::c_int;
pub const ARCHIVE_FILTER_LRZIP: libc::c_int = 10 as libc::c_int;
pub const ARCHIVE_FILTER_LZOP: libc::c_int = 11 as libc::c_int;
pub const ARCHIVE_FILTER_GRZIP: libc::c_int = 12 as libc::c_int;
pub const ARCHIVE_FILTER_LZ4: libc::c_int = 13 as libc::c_int;
pub const ARCHIVE_FILTER_ZSTD: libc::c_int = 14 as libc::c_int;
static mut codes: [C2RustUnnamed; 14] = unsafe {
    [
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_NONE,
                setter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_GZIP,
                setter: Some(
                    archive_write_add_filter_gzip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_BZIP2,
                setter: Some(
                    archive_write_add_filter_bzip2
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_COMPRESS,
                setter: Some(
                    archive_write_add_filter_compress
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_GRZIP,
                setter: Some(
                    archive_write_add_filter_grzip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_LRZIP,
                setter: Some(
                    archive_write_add_filter_lrzip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_LZ4,
                setter: Some(
                    archive_write_add_filter_lz4
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_LZIP,
                setter: Some(
                    archive_write_add_filter_lzip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_LZMA,
                setter: Some(
                    archive_write_add_filter_lzma
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_LZOP,
                setter: Some(
                    archive_write_add_filter_lzip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_UU,
                setter: Some(
                    archive_write_add_filter_uuencode
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_XZ,
                setter: Some(
                    archive_write_add_filter_xz
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FILTER_ZSTD,
                setter: Some(
                    archive_write_add_filter_zstd
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: -(1 as libc::c_int),
                setter: ::std::mem::transmute::<
                    libc::intptr_t,
                    Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
/* A convenience function to set the filter based on the code. */
#[no_mangle]
pub unsafe extern "C" fn archive_write_add_filter(
    mut a: *mut archive,
    mut code: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while codes[i as usize].code != -(1 as libc::c_int) {
        if code == codes[i as usize].code {
            return codes[i as usize].setter.expect("non-null function pointer")(a);
        }
        i += 1
    }
    archive_set_error(
        a,
        EINVAL,
        b"No such filter\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
