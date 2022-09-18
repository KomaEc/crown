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
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_write_set_format_zip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_xar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_warc(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_ustar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_pax_restricted(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_pax(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_gnutar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_shar_dump(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_shar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_raw(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_mtree(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_iso9660(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_cpio_newc(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_cpio(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_7zip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
}
pub type __int64_t = libc::c_long;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type mode_t = __mode_t;
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
/*-
 * Copyright (c) 2003-2008 Tim Kientzle
 * Copyright (c) 2016 Martin Matuska
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
 * $FreeBSD: head/lib/libarchive/archive_entry.h 201096 2009-12-28 02:41:27Z kientzle $
 */
/* Note: Compiler will complain if this does not match archive.h! */
/*
 * Note: archive_entry.h is for use outside of libarchive; the
 * configuration headers (config.h, archive_platform.h, etc.) are
 * purely internal.  Do NOT use HAVE_XXX configuration macros to
 * control the behavior of this header!  If you must conditionalize,
 * use predefined compiler and/or platform macros.
 */
/* for wchar_t */
/* Get a suitable 64-bit integer type. */
/* The la_ssize_t should match the type used in 'struct stat' */
/* Get a suitable definition for mode_t */
/* Large file support for Android */
/*
 * On Windows, define LIBARCHIVE_STATIC if you're building or using a
 * .lib.  The default here assumes you're building a DLL.  Only
 * libarchive source should ever define __LIBARCHIVE_BUILD.
 */
/* Static libraries on all platforms and shared libraries on non-Windows. */
/*
 * Description of an archive entry.
 *
 * You can think of this as "struct stat" with some text fields added in.
 *
 * TODO: Add "comment", "charset", and possibly other entries that are
 * supported by "pax interchange" format.  However, GNU, ustar, cpio,
 * and other variants don't support these features, so they're not an
 * excruciatingly high priority right now.
 *
 * TODO: "pax interchange" format allows essentially arbitrary
 * key/value attributes to be attached to any entry.  Supporting
 * such extensions may make this library useful for special
 * applications (e.g., a package manager could attach special
 * package-management attributes to each entry).
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
/* A table that maps format codes to functions. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub code: libc::c_int,
    pub setter: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
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
 * $FreeBSD: head/lib/libarchive/archive_platform.h 201090 2009-12-28 02:22:04Z kientzle $
 */
/* !!ONLY FOR USE INTERNALLY TO LIBARCHIVE!! */
/*
 * This header is the first thing included in any of the libarchive
 * source files.  As far as possible, platform-specific issues should
 * be dealt with here and not within individual source files.  I'm
 * actively trying to minimize #if blocks within the main source,
 * since they obfuscate the code.
 */
/* archive.h and archive_entry.h require this. */
/* Most POSIX platforms use the 'configure' script to build config.h */
/* On macOS check for some symbols based on the deployment target version.  */
/* It should be possible to get rid of this by extending the feature-test
 * macros to cover Windows API functions, probably along with non-trivial
 * refactoring of code to find structures that sit more cleanly on top of
 * either Windows or Posix APIs. */
/*
 * The config files define a lot of feature macros.  The following
 * uses those macros to select/define replacements and include key
 * headers as required.
 */
/* Get a real definition for __FBSDID or __RCSID if we can */
/* If not, define them so as to avoid dangling semicolons. */
/* Try to get standard C99-style integer type definitions. */
/* Borland warns about its own constants!  */
/* Some platforms lack the standard *_MAX definitions. */
/*
 * If we can't restore metadata using a file descriptor, then
 * for compatibility's sake, close files before trying to restore metadata.
 */
/*
 * glibc 2.24 deprecates readdir_r
 */
/* Set up defaults for internal error codes. */
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
/*
 * Codes to identify various stream filters.
 */
/*
 * Codes returned by archive_format.
 *
 * Top 16 bits identifies the format family (e.g., "tar"); lower
 * 16 bits indicate the variant.  This is updated by read_next_header.
 * Note that the lower 16 bits will often vary from entry to entry.
 * In some cases, this variation occurs as libarchive learns more about
 * the archive (for example, later entries might utilize extensions that
 * weren't necessary earlier in the archive; in this case, libarchive
 * will change the format code to indicate the extended format that
 * was used).  In other cases, it's because different tools have
 * modified the archive and so different parts of the archive
 * actually have slightly different formats.  (Both tar and cpio store
 * format codes in each entry, so it is quite possible for each
 * entry to be in a different format.)
 */
pub const ARCHIVE_FORMAT_CPIO: libc::c_int = 0x10000 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_POSIX: libc::c_int = ARCHIVE_FORMAT_CPIO | 1 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_SVR4_NOCRC: libc::c_int = ARCHIVE_FORMAT_CPIO | 4 as libc::c_int;
pub const ARCHIVE_FORMAT_SHAR: libc::c_int = 0x20000 as libc::c_int;
pub const ARCHIVE_FORMAT_SHAR_BASE: libc::c_int = ARCHIVE_FORMAT_SHAR | 1 as libc::c_int;
pub const ARCHIVE_FORMAT_SHAR_DUMP: libc::c_int = ARCHIVE_FORMAT_SHAR | 2 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR: libc::c_int = 0x30000 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_USTAR: libc::c_int = ARCHIVE_FORMAT_TAR | 1 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE: libc::c_int = ARCHIVE_FORMAT_TAR | 2 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_PAX_RESTRICTED: libc::c_int = ARCHIVE_FORMAT_TAR | 3 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_GNUTAR: libc::c_int = ARCHIVE_FORMAT_TAR | 4 as libc::c_int;
pub const ARCHIVE_FORMAT_ISO9660: libc::c_int = 0x40000 as libc::c_int;
pub const ARCHIVE_FORMAT_ZIP: libc::c_int = 0x50000 as libc::c_int;
pub const ARCHIVE_FORMAT_MTREE: libc::c_int = 0x80000 as libc::c_int;
pub const ARCHIVE_FORMAT_RAW: libc::c_int = 0x90000 as libc::c_int;
pub const ARCHIVE_FORMAT_XAR: libc::c_int = 0xa0000 as libc::c_int;
pub const ARCHIVE_FORMAT_7ZIP: libc::c_int = 0xe0000 as libc::c_int;
pub const ARCHIVE_FORMAT_WARC: libc::c_int = 0xf0000 as libc::c_int;
/*
 * File-type constants.  These are returned from archive_entry_filetype()
 * and passed to archive_entry_set_filetype().
 *
 * These values match S_XXX defines on every platform I've checked,
 * including Windows, AIX, Linux, Solaris, and BSD.  They're
 * (re)defined here because platforms generally don't define the ones
 * they don't support.  For example, Windows doesn't define S_IFLNK or
 * S_IFBLK.  Instead of having a mass of conditional logic and system
 * checks to define any S_XXX values that aren't supported locally,
 * I've just defined a new set of such constants so that
 * libarchive-based applications can manipulate and identify archive
 * entries properly even if the hosting platform can't store them on
 * disk.
 *
 * These values are also used directly within some portable formats,
 * such as cpio.  If you find a platform that varies from these, the
 * correct solution is to leave these alone and translate from these
 * portable values to platform-native values when entries are read from
 * or written to disk.
 */
/*
 * In libarchive 4.0, we can drop the casts here.
 * They're needed to work around Borland C's broken mode_t.
 */
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFSOCK: libc::c_int = 0o140000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
static mut codes: [C2RustUnnamed; 19] = unsafe {
    [
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_7ZIP,
                setter: Some(
                    archive_write_set_format_7zip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_CPIO,
                setter: Some(
                    archive_write_set_format_cpio
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_CPIO_POSIX,
                setter: Some(
                    archive_write_set_format_cpio
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_CPIO_SVR4_NOCRC,
                setter: Some(
                    archive_write_set_format_cpio_newc
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_ISO9660,
                setter: Some(
                    archive_write_set_format_iso9660
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_MTREE,
                setter: Some(
                    archive_write_set_format_mtree
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_RAW,
                setter: Some(
                    archive_write_set_format_raw
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_SHAR,
                setter: Some(
                    archive_write_set_format_shar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_SHAR_BASE,
                setter: Some(
                    archive_write_set_format_shar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_SHAR_DUMP,
                setter: Some(
                    archive_write_set_format_shar_dump
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_TAR,
                setter: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_TAR_GNUTAR,
                setter: Some(
                    archive_write_set_format_gnutar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE,
                setter: Some(
                    archive_write_set_format_pax
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_TAR_PAX_RESTRICTED,
                setter: Some(
                    archive_write_set_format_pax_restricted
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_TAR_USTAR,
                setter: Some(
                    archive_write_set_format_ustar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_WARC,
                setter: Some(
                    archive_write_set_format_warc
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_XAR,
                setter: Some(
                    archive_write_set_format_xar
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: ARCHIVE_FORMAT_ZIP,
                setter: Some(
                    archive_write_set_format_zip
                        as unsafe extern "C" fn(_: *mut archive) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed {
                code: 0 as libc::c_int,
                setter: ::std::mem::transmute::<
                    libc::intptr_t,
                    Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
                >(NULL as libc::intptr_t),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format(
    mut a: *mut archive,
    mut code: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while codes[i as usize].code != 0 as libc::c_int {
        if code == codes[i as usize].code {
            return codes[i as usize].setter.expect("non-null function pointer")(a);
        }
        i += 1
    }
    archive_set_error(
        a,
        EINVAL,
        b"No such format\x00" as *const u8 as *const libc::c_char,
    );
    return -(30 as libc::c_int);
}
/*-
 * Copyright (c) 2020 Martin Matuska
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
 * $FreeBSD$
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_write_entry_filetype_unsupported(
    mut a: *mut archive,
    mut entry: *mut archive_entry,
    mut format: *const libc::c_char,
) {
    let mut name: *const libc::c_char = NULL as *const libc::c_char;
    match archive_entry_filetype(entry) {
        16384 => {
            /*
             * All formats should be able to archive regular files (AE_IFREG)
             */
            name = b"directories\x00" as *const u8 as *const libc::c_char
        }
        40960 => name = b"symbolic links\x00" as *const u8 as *const libc::c_char,
        8192 => name = b"character devices\x00" as *const u8 as *const libc::c_char,
        24576 => name = b"block devices\x00" as *const u8 as *const libc::c_char,
        4096 => name = b"named pipes\x00" as *const u8 as *const libc::c_char,
        49152 => name = b"sockets\x00" as *const u8 as *const libc::c_char,
        _ => {}
    }
    if !name.is_null() {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"%s: %s format cannot archive %s\x00" as *const u8 as *const libc::c_char,
            archive_entry_pathname(entry),
            format,
            name,
        );
    } else {
        archive_set_error(
            a,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"%s: %s format cannot archive files with mode 0%lo\x00" as *const u8
                as *const libc::c_char,
            archive_entry_pathname(entry),
            format,
            archive_entry_mode(entry) as libc::c_ulong,
        );
    };
}
