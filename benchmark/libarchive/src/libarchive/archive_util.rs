use ::libc;
extern "C" {
    pub type archive_string_conv;
    /* Declare our basic types. */
    pub type archive_entry;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn archive_filter_bytes(_: *mut archive, _: libc::c_int) -> la_int64_t;
    #[no_mangle]
    fn archive_filter_code(_: *mut archive, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_filter_name(_: *mut archive, _: libc::c_int) -> *const libc::c_char;
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
    /* Initialize an archive_string object on the stack or elsewhere. */
    /* Append a C char to an archive_string, resizing as necessary. */
    /* Ditto for a wchar_t and an archive_wstring. */
    /* Append a raw array to an archive_string, resizing as necessary */
    /* Convert a Unicode string to current locale and append the result. */
    /* Returns -1 if conversion fails. */
    /* Create a string conversion object.
     * Return NULL and set a error message if the conversion is not supported
     * on the platform. */
    /* Create the default string conversion object for reading/writing an archive.
     * Return NULL if the conversion is unneeded.
     * Note: On non Windows platform this always returns NULL.
     */
    /* Dispose of a string conversion object. */
    /* Copy one archive_string to another in locale conversion.
     * Return -1 if conversion fails. */
    /* Copy one archive_string to another in locale conversion.
     * Return -1 if conversion fails. */
    /* Copy one archive_string to another */
    /* Concatenate one archive_string to another */
    #[no_mangle]
    fn archive_string_concat(dest: *mut archive_string, src: *mut archive_string);
    /* Ensure that the underlying buffer is at least as large as the request. */
    /* Append C string, which may lack trailing \0. */
    /* The source is declared void * here because this gets used with
     * "signed char *", "unsigned char *" and "char *" arguments.
     * Declaring it "char *" as with some of the other functions just
     * leads to a lot of extra casts. */
    /* Append a C string to an archive_string, resizing as necessary. */
    /* Copy a C string to an archive_string, resizing as necessary. */
    /* Copy a C string to an archive_string with limit, resizing as necessary. */
    /* Return length of string. */
    /* Set string length to zero. */
    /* Release any allocated storage resources. */
    /* Like 'vsprintf', but resizes the underlying string as necessary. */
    /* Note: This only implements a small subset of standard printf functionality. */
    #[no_mangle]
    fn archive_string_vsprintf(
        _: *mut archive_string,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    );
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_conversion_free(_: *mut archive);
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
/* for wchar_t */
/* For FILE * */
/* For time_t */
/*
 * Note: archive.h is for use outside of libarchive; the configuration
 * headers (config.h, archive_platform.h, etc.) are purely internal.
 * Do NOT use HAVE_XXX configuration macros to control the behavior of
 * this header!  If you must conditionalize, use predefined compiler and/or
 * platform macros.
 */
/* Get appropriate definitions of 64-bit integer */
/* Older code relied on the __LA_INT64_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
pub type la_int64_t = int64_t;
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const F_GETFD: libc::c_int = 1 as libc::c_int;
pub const F_SETFD: libc::c_int = 2 as libc::c_int;
pub const FD_CLOEXEC: libc::c_int = 1 as libc::c_int;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Generic initialization of 'struct archive' objects. */
#[no_mangle]
pub unsafe extern "C" fn __archive_clean(mut a: *mut archive) -> libc::c_int {
    archive_string_conversion_free(a);
    return 0 as libc::c_int;
}
/* Large file support for Android */
/*
 * On Windows, define LIBARCHIVE_STATIC if you're building or using a
 * .lib.  The default here assumes you're building a DLL.  Only
 * libarchive source should ever define __LIBARCHIVE_BUILD.
 */
/* Static libraries or non-Windows needs no special declaration. */
/*
 * The version number is provided as both a macro and a function.
 * The macro identifies the installed header; the function identifies
 * the library version (which may not be the same if you're using a
 * dynamically-linked version of the library).  Of course, if the
 * header and library are very different, you should expect some
 * strangeness.  Don't do that.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_version_number() -> libc::c_int {
    return 3004003 as libc::c_int;
}
/*
 * Textual name/version of the library, useful for version displays.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_version_string() -> *const libc::c_char {
    return b"libarchive 3.4.3\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_errno(mut a: *mut archive) -> libc::c_int {
    return (*a).archive_error_number;
}
#[no_mangle]
pub unsafe extern "C" fn archive_error_string(mut a: *mut archive) -> *const libc::c_char {
    if !(*a).error.is_null() && *(*a).error as libc::c_int != '\u{0}' as i32 {
        return (*a).error;
    } else {
        return 0 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_file_count(mut a: *mut archive) -> libc::c_int {
    return (*a).file_count;
}
#[no_mangle]
pub unsafe extern "C" fn archive_format(mut a: *mut archive) -> libc::c_int {
    return (*a).archive_format;
}
#[no_mangle]
pub unsafe extern "C" fn archive_format_name(mut a: *mut archive) -> *const libc::c_char {
    return (*a).archive_format_name;
}
#[no_mangle]
pub unsafe extern "C" fn archive_compression(mut a: *mut archive) -> libc::c_int {
    return archive_filter_code(a, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn archive_compression_name(mut a: *mut archive) -> *const libc::c_char {
    return archive_filter_name(a, 0 as libc::c_int);
}
/*
 * Return a count of the number of compressed bytes processed.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_position_compressed(mut a: *mut archive) -> la_int64_t {
    return archive_filter_bytes(a, -(1 as libc::c_int));
}
/*
 * Return a count of the number of uncompressed bytes processed.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_position_uncompressed(mut a: *mut archive) -> la_int64_t {
    return archive_filter_bytes(a, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn archive_clear_error(mut a: *mut archive) {
    (*a).error_string.length = 0 as libc::c_int as size_t;
    (*a).error = NULL as *const libc::c_char;
    (*a).archive_error_number = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_set_error(
    mut a: *mut archive,
    mut error_number: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    (*a).archive_error_number = error_number;
    if fmt.is_null() {
        (*a).error = NULL as *const libc::c_char;
        return;
    }
    (*a).error_string.length = 0 as libc::c_int as size_t;
    ap = args.clone();
    archive_string_vsprintf(&mut (*a).error_string, fmt, ap.as_va_list());
    (*a).error = (*a).error_string.s;
}
#[no_mangle]
pub unsafe extern "C" fn archive_copy_error(mut dest: *mut archive, mut src: *mut archive) {
    (*dest).archive_error_number = (*src).archive_error_number;
    (*dest).error_string.length = 0 as libc::c_int as size_t;
    archive_string_concat(&mut (*dest).error_string, &mut (*src).error_string);
    (*dest).error = (*dest).error_string.s;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_errx(
    mut retvalue: libc::c_int,
    mut msg: *const libc::c_char,
) -> ! {
    static mut msg1: [libc::c_char; 37] = unsafe {
        *::std::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
            b"Fatal Internal Error in libarchive: \x00",
        )
    };
    let mut s: size_t = 0;
    s = write(
        2 as libc::c_int,
        msg1.as_ptr() as *const libc::c_void,
        strlen(msg1.as_ptr()),
    ) as size_t;
    /* UNUSED */
    s = write(2 as libc::c_int, msg as *const libc::c_void, strlen(msg)) as size_t;
    /* UNUSED */
    s = write(
        2 as libc::c_int,
        b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) as size_t;
    /* UNUSED */
    exit(retvalue);
}
/*
 * Create a temporary file
 */
unsafe extern "C" fn get_tempdir(mut temppath: *mut archive_string) -> libc::c_int {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    tmp = getenv(b"TMPDIR\x00" as *const u8 as *const libc::c_char);
    if tmp.is_null() {
        tmp = b"/tmp\x00" as *const u8 as *const libc::c_char
    }
    (*temppath).length = 0 as libc::c_int as size_t;
    archive_strncat(
        temppath,
        tmp as *const libc::c_void,
        (if tmp.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(tmp)
        }),
    );
    if *(*temppath).s.offset(
        (*temppath)
            .length
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
    ) as libc::c_int
        != '/' as i32
    {
        archive_strappend_char(temppath, '/' as i32 as libc::c_char);
    }
    return 0 as libc::c_int;
}
/*
 * We can use mkstemp().
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_mktemp(mut tmpdir: *const libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut temp_name: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut fd: libc::c_int = -(1 as libc::c_int);
    temp_name.s = NULL as *mut libc::c_char;
    temp_name.length = 0 as libc::c_int as size_t;
    temp_name.buffer_length = 0 as libc::c_int as size_t;
    if tmpdir.is_null() {
        if get_tempdir(&mut temp_name) != ARCHIVE_OK {
            current_block = 1658919162659523233;
        } else {
            current_block = 7651349459974463963;
        }
    } else {
        temp_name.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut temp_name,
            tmpdir as *const libc::c_void,
            (if tmpdir.is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(tmpdir)
            }),
        );
        if *temp_name.s.offset(
            temp_name
                .length
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int
            != '/' as i32
        {
            archive_strappend_char(&mut temp_name, '/' as i32 as libc::c_char);
        }
        current_block = 7651349459974463963;
    }
    match current_block {
        7651349459974463963 => {
            archive_strcat(
                &mut temp_name,
                b"libarchive_XXXXXX\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            );
            fd = mkstemp(temp_name.s);
            if !(fd < 0 as libc::c_int) {
                __archive_ensure_cloexec_flag(fd);
                unlink(temp_name.s);
            }
        }
        _ => {}
    }
    archive_string_free(&mut temp_name);
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_mkstemp(mut template: *mut libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    fd = mkstemp(template);
    if fd >= 0 as libc::c_int {
        __archive_ensure_cloexec_flag(fd);
    }
    return fd;
}
/* Check magic value and state; return(ARCHIVE_FATAL) if it isn't valid. */
/* !HAVE_MKSTEMP */
/* !HAVE_MKSTEMP */
/* !_WIN32 || __CYGWIN__ */
/*
 * Set FD_CLOEXEC flag to a file descriptor if it is not set.
 * We have to set the flag if the platform does not provide O_CLOEXEC
 * or F_DUPFD_CLOEXEC flags.
 *
 * Note: This function is absolutely called after creating a new file
 * descriptor even if the platform seemingly provides O_CLOEXEC or
 * F_DUPFD_CLOEXEC macros because it is possible that the platform
 * merely declares those macros, especially Linux 2.6.18 - 2.6.24 do it.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_ensure_cloexec_flag(mut fd: libc::c_int) {
    let mut flags: libc::c_int = 0;
    if fd >= 0 as libc::c_int {
        flags = fcntl(fd, F_GETFD);
        if flags != -(1 as libc::c_int) && flags & FD_CLOEXEC == 0 as libc::c_int {
            fcntl(fd, F_SETFD, flags | FD_CLOEXEC);
        }
    };
}
/*
 * Utility function to sort a group of strings using quicksort.
 */
unsafe extern "C" fn archive_utility_string_sort_helper(
    mut strings: *mut *mut libc::c_char,
    mut n: libc::c_uint,
) -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut lesser_count: libc::c_uint = 0;
    let mut greater_count: libc::c_uint = 0;
    let mut lesser: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut greater: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pivot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval1: libc::c_int = 0;
    let mut retval2: libc::c_int = 0;
    /* A list of 0 or 1 elements is already sorted */
    if n <= 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    greater_count = 0 as libc::c_int as libc::c_uint;
    lesser_count = greater_count;
    greater = NULL as *mut *mut libc::c_char;
    lesser = greater;
    pivot = *strings.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int as libc::c_uint;
    while i < n {
        if strcmp(*strings.offset(i as isize), pivot) < 0 as libc::c_int {
            lesser_count = lesser_count.wrapping_add(1);
            tmp = realloc(
                lesser as *mut libc::c_void,
                (lesser_count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            if tmp.is_null() {
                free(greater as *mut libc::c_void);
                free(lesser as *mut libc::c_void);
                return -(30 as libc::c_int);
            }
            lesser = tmp;
            let ref mut fresh0 = *lesser
                .offset(lesser_count.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
            *fresh0 = *strings.offset(i as isize)
        } else {
            greater_count = greater_count.wrapping_add(1);
            tmp = realloc(
                greater as *mut libc::c_void,
                (greater_count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            if tmp.is_null() {
                free(greater as *mut libc::c_void);
                free(lesser as *mut libc::c_void);
                return -(30 as libc::c_int);
            }
            greater = tmp;
            let ref mut fresh1 = *greater
                .offset(greater_count.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
            *fresh1 = *strings.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    /* quicksort(lesser) */
    retval1 = archive_utility_string_sort_helper(lesser, lesser_count);
    i = 0 as libc::c_int as libc::c_uint;
    while i < lesser_count {
        let ref mut fresh2 = *strings.offset(i as isize);
        *fresh2 = *lesser.offset(i as isize);
        i = i.wrapping_add(1)
    }
    free(lesser as *mut libc::c_void);
    /* pivot */
    let ref mut fresh3 = *strings.offset(lesser_count as isize);
    *fresh3 = pivot;
    /* quicksort(greater) */
    retval2 = archive_utility_string_sort_helper(greater, greater_count);
    i = 0 as libc::c_int as libc::c_uint;
    while i < greater_count {
        let ref mut fresh4 = *strings.offset(
            lesser_count
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(i) as isize,
        );
        *fresh4 = *greater.offset(i as isize);
        i = i.wrapping_add(1)
    }
    free(greater as *mut libc::c_void);
    return if retval1 < retval2 { retval1 } else { retval2 };
}
/* A deprecated synonym for archive_write_open_filename() */
/* _buffSize is the size of the buffer, _used refers to a variable that
 * will be updated after each write into the buffer. */
/*
 * Note that the library will truncate writes beyond the size provided
 * to archive_write_header or pad if the provided data is short.
 */
/* This interface is currently only available for archive_write_disk handles.  */
/* Marks the archive as FATAL so that a subsequent free() operation
 * won't try to close() cleanly.  Provides a fast abort capability
 * when the client discovers that things have gone wrong. */
/* This can fail if the archive wasn't already closed, in which case
 * archive_write_free() will implicitly call archive_write_close(). */
/* Synonym for archive_write_free() for backwards compatibility. */
/*
 * Set write options.
 */
/* Apply option to the format only. */
/* Apply option to the filter only. */
/* Apply option to both the format and the filter. */
/* Apply option string to both the format and the filter. */
/*
 * Set a encryption passphrase.
 */
/*-
 * ARCHIVE_WRITE_DISK API
 *
 * To create objects on disk:
 *   1) Ask archive_write_disk_new for a new archive_write_disk object.
 *   2) Set any global properties.  In particular, you probably
 *      want to set the options.
 *   3) For each entry:
 *      - construct an appropriate struct archive_entry structure
 *      - archive_write_header to create the file/dir/etc on disk
 *      - archive_write_data to write the entry data
 *   4) archive_write_free to cleanup the writer and release resources
 *
 * In particular, you can use this in conjunction with archive_read()
 * to pull entries out of an archive and create them on disk.
 */
/* This file will not be overwritten. */
/* Set flags to control how the next item gets created.
 * This accepts a bitmask of ARCHIVE_EXTRACT_XXX flags defined above. */
/*
 * The lookup functions are given uname/uid (or gname/gid) pairs and
 * return a uid (gid) suitable for this system.  These are used for
 * restoring ownership and for setting ACLs.  The default functions
 * are naive, they just return the uid/gid.  These are small, so reasonable
 * for applications that don't need to preserve ownership; they
 * are probably also appropriate for applications that are doing
 * same-system backup and restore.
 */
/*
 * The "standard" lookup functions use common system calls to lookup
 * the uname/gname, falling back to the uid/gid if the names can't be
 * found.  They cache lookups and are reasonably fast, but can be very
 * large, so they are not used unless you ask for them.  In
 * particular, these match the specifications of POSIX "pax" and old
 * POSIX "tar".
 */
/*
 * If neither the default (naive) nor the standard (big) functions suit
 * your needs, you can write your own and register them.  Be sure to
 * include a cleanup function if you have allocated private data.
 */
/* private_data */
/* cleanup */
/* private_data */
/* cleanup */
/*
 * ARCHIVE_READ_DISK API
 *
 * This is still evolving and somewhat experimental.
 */
/* The names for symlink modes here correspond to an old BSD
 * command-line argument convention: -L, -P, -H */
/* Follow all symlinks. */
/* Follow no symlinks. */
/* Follow symlink initially, then not. */
/* TODO: Handle Linux stat32/stat64 ugliness. <sigh> */
/* fd */
/* Look up gname for gid or uname for uid. */
/* Default implementations are very, very stupid. */
/* "Standard" implementation uses getpwuid_r, getgrgid_r and caches the
 * results for performance. */
/* You can install your own lookups if you like. */
/* private_data */
/* lookup_fn */
/* cleanup_fn */
/* private_data */
/* lookup_fn */
/* cleanup_fn */
/* Start traversal. */
/*
 * Request that current entry be visited.  If you invoke it on every
 * directory, you'll get a physical traversal.  This is ignored if the
 * current entry isn't a directory or a link to a directory.  So, if
 * you invoke this on every returned path, you'll get a full logical
 * traversal.
 */
/* Request that the access time of the entry visited by traversal be restored. */
/*
 * Set behavior. The "flags" argument selects optional behavior.
 */
/* Request that the access time of the entry visited by traversal be restored.
 * This is the same as archive_read_disk_set_atime_restored. */
/* Default: Do not skip an entry which has nodump flags. */
/* Default: Skip a mac resource fork file whose prefix is "._" because of
 * using copyfile. */
/* Default: Traverse mount points. */
/* Default: Xattrs are read from disk. */
/* Default: ACLs are read from disk. */
/* Default: File flags are read from disk. */
/*
 * Set archive_match object that will be used in archive_read_disk to
 * know whether an entry should be skipped. The callback function
 * _excluded_func will be invoked when an entry is skipped by the result
 * of archive_match.
 */
/* Simplified cleanup interface;
 * This calls archive_read_free() or archive_write_free() as needed. */
/*
 * Accessor functions to read/set various information in
 * the struct archive object:
 */
/* Number of filters in the current filter pipeline. */
/* Filter #0 is the one closest to the format, -1 is a synonym for the
 * last filter, which is always the pseudo-filter that wraps the
 * client callbacks. */
/* These don't properly handle multiple filters, so are deprecated and
 * will eventually be removed. */
/* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, -1); */
/* As of libarchive 3.0, this is an alias for archive_filter_bytes(a, 0); */
/* As of libarchive 3.0, this is an alias for archive_filter_name(a, 0); */
/* As of libarchive 3.0, this is an alias for archive_filter_code(a, 0); */
/*
 * ARCHIVE_MATCH API
 */
/*
 * Test if archive_entry is excluded.
 * This is a convenience function. This is the same as calling all
 * archive_match_path_excluded, archive_match_time_excluded
 * and archive_match_owner_excluded.
 */
/*
 * Test if pathname is excluded. The conditions are set by following functions.
 */
/* Control recursive inclusion of directory content when directory is included. Default on. */
/* Add exclusion pathname pattern. */
/* Add exclusion pathname pattern from file. */
/* Add inclusion pathname pattern. */
/* Add inclusion pathname pattern from file. */
/*
 * How to get statistic information for inclusion patterns.
 */
/* Return the amount number of unmatched inclusion patterns. */
/* Return the pattern of unmatched inclusion with ARCHIVE_OK.
 * Return ARCHIVE_EOF if there is no inclusion pattern. */
/*
 * Test if a file is excluded by its time stamp.
 * The conditions are set by following functions.
 */
/*
 * Flags to tell a matching type of time stamps. These are used for
 * following functions.
 */
/* Time flag: mtime to be tested. */
/* Time flag: ctime to be tested. */
/* Comparison flag: Match the time if it is newer than. */
/* Comparison flag: Match the time if it is older than. */
/* Comparison flag: Match the time if it is equal to. */
/* Set inclusion time. */
/* Set inclusion time by a date string. */
/* Set inclusion time by a particular file. */
/* Add exclusion entry. */
/*
 * Test if a file is excluded by its uid ,gid, uname or gname.
 * The conditions are set by following functions.
 */
/* Add inclusion uid, gid, uname and gname. */
/* Utility functions */
/* Convenience function to sort a NULL terminated list of strings */
#[no_mangle]
pub unsafe extern "C" fn archive_utility_string_sort(
    mut strings: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while !(*strings.offset(size as isize)).is_null() {
        size = size.wrapping_add(1)
    }
    return archive_utility_string_sort_helper(strings, size);
}
