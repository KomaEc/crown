use ::libc;
extern "C" {
    /*-
     * Copyright (c) 2003-2010 Tim Kientzle
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
     * $FreeBSD: head/lib/libarchive/archive_string.h 201092 2009-12-28 02:26:06Z kientzle $
     *
     */
    /* required for wchar_t on some systems */
    /*
     * Basic resizable/reusable string support similar to Java's "StringBuffer."
     *
     * Unlike sbuf(9), the buffers here are fully reusable and track the
     * length throughout.
     */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    /* Pointer to the storage */
    /* Length of 's' in characters */
    /* Length of malloc-ed storage in bytes. */
    pub type archive_string_conv;
    pub type archive_entry;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_write_add_filter_bzip2(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_gzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_none(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_xz(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_7zip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_ar_svr4(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_cpio(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_iso9660(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_pax_restricted(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_zip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
/* A table that maps names to functions. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *const libc::c_char,
    pub format: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub filter: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_STATE_FATAL: libc::c_uint = 0x8000 as libc::c_uint;
static mut names: [C2RustUnnamed; 13] = unsafe {
    [
        {
            let mut init = C2RustUnnamed {
                name: b".7z\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_7zip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".zip\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_zip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".jar\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_zip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".cpio\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_cpio
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".iso\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_iso9660
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".a\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_ar_svr4
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".ar\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_ar_svr4
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".tar\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_none
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".tgz\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_gzip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".tar.gz\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_gzip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".tar.bz2\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_bzip2
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: b".tar.xz\x00" as *const u8 as *const libc::c_char,
                format: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
                filter: Some(
                    archive_write_add_filter_xz
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                name: NULL as *const libc::c_char,
                format: ::std::mem::transmute::<
                    libc::intptr_t,
                    Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
                >(NULL as libc::intptr_t),
                filter: ::std::mem::transmute::<
                    libc::intptr_t,
                    Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
unsafe extern "C" fn cmpsuff(
    mut str: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> libc::c_int {
    let mut length_str: size_t = 0;
    let mut length_suffix: size_t = 0;
    if str.is_null() || suffix.is_null() {
        return -(1 as libc::c_int);
    }
    length_str = strlen(str);
    length_suffix = strlen(suffix);
    if length_str >= length_suffix {
        return strcmp(
            str.offset(length_str.wrapping_sub(length_suffix) as isize),
            suffix,
        );
    } else {
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn get_array_index(mut name: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !names[i as usize].name.is_null() {
        if cmpsuff(name, names[i as usize].name) == 0 as libc::c_int {
            return i;
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_filter_by_ext(
    mut a: *mut archive,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut names_index: libc::c_int = get_array_index(filename);
    if names_index >= 0 as libc::c_int {
        let mut format_state: libc::c_int = names[names_index as usize]
            .format
            .expect("non-null function pointer")(a);
        if format_state == ARCHIVE_OK {
            return names[names_index as usize]
                .filter
                .expect("non-null function pointer")(a);
        } else {
            return format_state;
        }
    }
    archive_set_error(
        a,
        EINVAL,
        b"No such format \'%s\'\x00" as *const u8 as *const libc::c_char,
        filename,
    );
    (*a).state = ARCHIVE_STATE_FATAL;
    return -(30 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_filter_by_ext_def(
    mut a: *mut archive,
    mut filename: *const libc::c_char,
    mut def_ext: *const libc::c_char,
) -> libc::c_int {
    let mut names_index: libc::c_int = get_array_index(filename);
    if names_index < 0 as libc::c_int {
        names_index = get_array_index(def_ext)
    }
    if names_index >= 0 as libc::c_int {
        let mut format_state: libc::c_int = names[names_index as usize]
            .format
            .expect("non-null function pointer")(a);
        if format_state == ARCHIVE_OK {
            return names[names_index as usize]
                .filter
                .expect("non-null function pointer")(a);
        } else {
            return format_state;
        }
    }
    archive_set_error(
        a,
        EINVAL,
        b"No such format \'%s\'\x00" as *const u8 as *const libc::c_char,
        filename,
    );
    (*a).state = ARCHIVE_STATE_FATAL;
    return -(30 as libc::c_int);
}
